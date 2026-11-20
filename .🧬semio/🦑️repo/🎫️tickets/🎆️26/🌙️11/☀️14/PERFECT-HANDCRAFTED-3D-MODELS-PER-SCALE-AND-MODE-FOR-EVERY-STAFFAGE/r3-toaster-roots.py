"""🌱 Round 3 two-slice toaster: builds the candidate asset document with every root correction (rig, poses, anchors, thermal areas, provenance, the 2D drawings of every scale and view, representation decisions) from the catalog file and the drawing module, writes it to `🗑️generated/r3-toaster/🔣️.json` with the source volumes merged in, and with `--apply` writes the changed members into the catalog file in its own formatting (re-read right before, member order kept, `volumes` untouched).

Usage: python3 r3-toaster-roots.py [--apply]
"""
import copy
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/toaster/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-toaster", "🔣️.json")
SOURCE = os.path.join(HERE, "volumes", "appliances", "toaster.json")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])
_spec = importlib.util.spec_from_file_location("draw", os.path.join(HERE, "r3-toaster-draw.py"))
D = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(D)
K = D.K

AREAS = {"casing": 0.181, "top-plate": 0.048, "bread": 0.054, "control": 0.0057, "feet": 0.0079}
REFERENCE = {"id": "retail-two-slice-toasters-2026", "citation": "De'Longhi and retailer product data of two-slice pop-up toasters, read 2026-11-20 (De'Longhi Icona CTO2003: 185 mm x 320 mm x 205 mm, 900 W, 2 kg, two slots): the class is 0.26 to 0.32 m long, 0.16 to 0.19 m deep and 0.18 to 0.21 m high at 800 to 1000 W."}
CASING_EN = ("95 W on 0.24 m2, about 400 W/m2", "95 W on {a} m2, about {q} W/m2")
CASING_DE = ("95 W auf 0,24 m2, etwa 400 W/m2", "95 W auf {a} m2, etwa {q} W/m2")


def bone(asset, bid):
    return next(b for b in asset["rig"]["bones"] if b["id"] == bid)


def roots(asset):
    """🌱 Every root correction in one place, applied to a copy of the catalog asset."""
    a = copy.deepcopy(asset)
    s = bone(a, "slices")
    s["y"] = K.SEAT
    s["mass"]["comY"] = 0.055
    s["joint"]["minM"], s["joint"]["maxM"] = -K.TRAVEL, K.TRAVEL
    bone(a, "lever")["joint"]["minM"] = -K.TRAVEL
    next(p for p in a["poses"] if p["id"] == "toasting")["bones"]["lever"]["dy"] = -K.TRAVEL
    for anchor in a["anchors"]:
        if anchor["id"] in ("foot-left", "foot-right"):
            anchor["footprintLengthM"] = 2 * K.FOOT_R1
            anchor["footprintWidthM"] = 2 * K.FOOT_R1
    for region in a["thermal"]["regions"]:
        region["areaM2"] = AREAS[region["id"]]
    area = AREAS["casing"]
    flux = str(int(round(95 / area / 10.0) * 10))
    for x in a["thermal"]["assumptions"]:
        x["en"] = x["en"].replace(CASING_EN[0], CASING_EN[1].format(a=f"{area:g}", q=flux))
        x["de"] = x["de"].replace(CASING_DE[0], CASING_DE[1].format(a=f"{area:g}".replace(".", ","), q=flux))
    for x in a["thermal"]["assumptions"]:
        x["en"] = x["en"].replace("cool-touch at about 60 degC surface", "touchable at about 70 degC surface")
        x["de"] = x["de"].replace("berührungskühl bei etwa 60 °C Oberfläche", "berührbar bei etwa 70 °C Oberfläche")
    refs = a["provenance"]["references"]
    if not any(r["id"] == REFERENCE["id"] for r in refs):
        refs.append(REFERENCE)
    a["provenance"]["notes"] = json.load(open(D.TEXTS, encoding="utf-8"))["provenanceNotes"]
    for den, rep in a["representations"].items():
        if rep["status"] != "authored":
            continue
        rep["geometry"] = D.geometry(den, rep["geometry"])
        rep["decisions"] = D.decisions(den)
    return a


def inline(value, indent=0):
    """🖨️ JSON in the catalog's style: two-space indent, arrays of numbers and of number pairs on one line."""
    pad = "  " * indent
    if isinstance(value, list) and value and all(isinstance(x, (int, float)) and not isinstance(x, bool) for x in value):
        return json.dumps(value)
    if isinstance(value, list) and value and all(isinstance(x, list) and all(isinstance(y, (int, float)) for y in x) for x in value):
        return "[" + ", ".join(json.dumps(x) for x in value) + "]"
    if isinstance(value, list):
        if not value:
            return "[]"
        return "[\n" + ",\n".join(pad + "  " + inline(x, indent + 1) for x in value) + "\n" + pad + "]"
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(pad + "  " + json.dumps(k, ensure_ascii=False) + ": " + inline(v, indent + 1) for k, v in value.items()) + "\n" + pad + "}"
    return json.dumps(value, ensure_ascii=False)


def apply(want):
    text = open(CATALOG, encoding="utf-8").read()
    have = json.loads(text)["asset"]
    changed = [key for key in have if key != "volumes" and have[key] != want[key]]
    for key in changed:
        start, end = member_range(text, "    ", key)
        text = text[:start] + "    " + json.dumps(key) + ": " + inline(want[key], 2) + text[end:]
    got = json.loads(text)["asset"]
    if list(got) != list(have) or any(got[key] != want[key] for key in want if key != "volumes") or got["volumes"] != have["volumes"]:
        raise SystemExit("the edited text does not match the candidate")
    open(CATALOG, "w", encoding="utf-8").write(text)
    return changed


def main():
    doc = json.load(open(CATALOG, encoding="utf-8"))
    want = roots(doc["asset"])
    cand = copy.deepcopy(doc)
    cand["asset"] = copy.deepcopy(want)
    cand["asset"]["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    with open(CANDIDATE, "w", encoding="utf-8") as f:
        json.dump(cand, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print("candidate", CANDIDATE)
    if "--apply" in sys.argv:
        print("written", CATALOG, "members:", ", ".join(apply(want)) or "none")


if __name__ == "__main__":
    main()
