"""🌱 Root fixes of the market parasol (round 3): the rig (clevis pivot above the base plate, ribs of the true length at 45-degree spacing, eight stretchers), every pose and the flutter key of the linkage, summary and structure assumptions, the redrawn 2D of every scale and the representation decisions; writes the candidate document, or the catalog file with --write.

Usage: python3 r3-market-parasol-roots.py [--write]
See ./r3-objects-market-parasol.md
"""
import copy
import importlib.util
import json
import sys
from pathlib import Path

T = Path(__file__).resolve().parent


def module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), T / f"{name}.py")
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


k = module("r3-market-parasol-kit")
draw = module("r3-market-parasol-draw")
texts = module("r3-market-parasol-texts")

TITLES = {"right": ("near right", "rechts"), "left": ("left", "links")}


def rib_title(r):
    extra = TITLES.get(r)
    az = int(k.AZ[r])
    if extra:
        return {"en": f"Rib at {az} deg ({extra[0]})", "de": f"Speiche bei {az} Grad ({extra[1]})"}
    return {"en": f"Rib at {az} deg", "de": f"Speiche bei {az} Grad"}


def stretch_title(r):
    az = int(k.AZ[r])
    return {"en": f"Stretcher of the rib at {az} deg", "de": f"Spreize der Speiche bei {az} Grad"}


def rig(old):
    bones = {b["id"]: b for b in old["bones"]}
    base, mast, crown, runner = (copy.deepcopy(bones[i]) for i in ("base", "mast", "crown", "runner"))
    mast["y"] = k.PIVOT_Y
    mast["lengthM"] = k.r6(k.MAST_L)
    mast["mass"]["comX"] = k.r6(k.MAST_L / 2)
    crown["x"] = k.r6(k.MAST_L)
    runner["x"] = k.r6(k.RUNNER_X)
    out = [base, mast, crown, runner]
    for r in k.RIBS:
        b = copy.deepcopy(bones[f"rib-{r}"])
        b["title"] = rib_title(r)
        b["angleDeg"] = k.REST_RIB[r]
        b["lengthM"] = k.RIB_L
        b["joint"] = {"type": "revolute", "minDeg": -100, "maxDeg": 100, "twistMinDeg": -180, "twistMaxDeg": 180}
        b["mass"]["comX"] = k.r6(k.RIB_L / 2)
        out.append(b)
    for r in k.RIBS:
        b = copy.deepcopy(bones.get(f"stretch-{r}", bones["stretch-right"]))
        b["id"] = f"stretch-{r}"
        b["title"] = stretch_title(r)
        b["angleDeg"] = k.REST_STRETCH[r]
        b["lengthM"] = k.STRETCH_L
        b["joint"] = {"type": "revolute", "minDeg": -100, "maxDeg": 100}
        out.append(b)
    return {**old, "bones": out}


def linkage_bones(state, runner_dx=None):
    out = {}
    if runner_dx is not None:
        out["runner"] = {"dx": runner_dx}
    for r in k.RIBS:
        out[f"rib-{r}"] = dict(state[r]["rib"])
    for r in k.RIBS:
        out[f"stretch-{r}"] = dict(state[r]["stretch"])
    return out


def poses(old):
    out = []
    for p in old:
        p = copy.deepcopy(p)
        keep = {b: v for b, v in p["bones"].items() if b == "mast"}
        if p["id"] == "furled":
            p["bones"] = {**keep, **linkage_bones(k.FURLED, k.FURL_DX)}
            p["description"] = {"en": "Ribs folded down along the mast; the runner has slid down by 0.65 m and pulled the eight stretchers in, the gores hang between the ribs.", "de": "Speichen entlang des Masts nach unten geklappt; der Läufer ist 0,65 m abgerutscht und hat die acht Spreizen eingezogen, die Stoffbahnen hängen zwischen den Speichen."}
        else:
            p["bones"] = {**keep, **linkage_bones(k.OPEN)}
        out.append(p)
    return out


def clips(old):
    out = copy.deepcopy(old)
    for c in out:
        for key in c["keys"]:
            if "bones" in key:
                key["bones"] = linkage_bones(k.FLUTTER, k.FLUTTER_DX)
        if c["id"] == "wind-flap":
            c["description"] = {"en": "A gust builds on the tilted canopy, lifts the runner by 49 mm so the ribs flutter up by 4 degrees, and the wind releases.", "de": "Eine Böe baut sich am geneigten Schirm auf, hebt den Läufer um 49 mm, sodass die Speichen um 4 Grad hochflattern, und der Wind lässt nach."}
    return out


def build(volumes=None):
    doc = json.loads(k.CATALOG.read_text(encoding="utf-8"))
    a = doc["asset"]
    a["summary"] = texts.SUMMARY
    a["rig"] = rig(a["rig"])
    a["poses"] = poses(a["poses"])
    a["clips"] = clips(a["clips"])
    a["structure"]["assumptions"] = texts.STRUCTURE_ASSUMPTIONS
    a["provenance"]["notes"] = texts.PROVENANCE_NOTES
    a["provenance"]["references"] = texts.references(a["provenance"]["references"])
    geo = draw.geometry(k)
    for s, g in geo.items():
        a["representations"][s]["geometry"] = g
        a["representations"][s]["decisions"] = texts.DECISIONS_2D[s]
    if volumes is not None:
        a["volumes"] = volumes
    return doc


def main():
    source = json.loads(k.SOURCE.read_text(encoding="utf-8"))["volumes"]
    doc = build(source)
    k.GEN.mkdir(parents=True, exist_ok=True)
    out = k.GEN / "🔣️.json"
    out.write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(out)
    if "--write" in sys.argv:
        fresh = json.loads(k.CATALOG.read_text(encoding="utf-8"))
        a, b = fresh["asset"], doc["asset"]
        for key in ("summary", "rig", "poses", "clips", "structure", "provenance", "representations"):
            a[key] = b[key]
        raw = k.CATALOG.read_text(encoding="utf-8")
        indent = 2 if raw.startswith("{\n  ") else None
        k.CATALOG.write_text(json.dumps(fresh, ensure_ascii=False, indent=indent) + "\n", encoding="utf-8")
        print("written", k.CATALOG)


if __name__ == "__main__":
    main()
