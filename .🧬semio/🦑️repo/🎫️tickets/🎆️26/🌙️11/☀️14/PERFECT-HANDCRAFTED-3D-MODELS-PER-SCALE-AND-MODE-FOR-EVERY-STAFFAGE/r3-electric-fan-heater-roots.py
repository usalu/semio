"""🌱 Round 3 electric fan heater: builds the candidate asset document with every root correction (materials, rig, anchors, thermal areas, provenance, the 2D drawings of every scale and view) from the catalog file and the drawing module, writes it to `🗑️generated/r3-electric-fan-heater/🔣️.json` with the source volumes merged in, and with `--apply` writes only the changed members into the catalog file in the catalog's own style.

Usage: python3 r3-electric-fan-heater-roots.py [--apply] [--check-style]
"""
import copy
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/electric-fan-heater/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-electric-fan-heater", "🔣️.json")
SOURCE = os.path.join(HERE, "volumes", "appliances", "electric-fan-heater.json")
TEXTS = os.path.join(HERE, "r3-electric-fan-heater-texts.json")
DRAW = os.path.join(HERE, "r3-electric-fan-heater-draw.py")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])

MATERIALS = [
    {"id": "steel-zinc", "title": {"en": "Galvanised steel sheet", "de": "Verzinktes Stahlblech"}, "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "nichrome-coil", "title": {"en": "Nickel-chromium heating wire, oxidised", "de": "Nickel-Chrom-Heizdraht, oxidiert"}, "emissivity": 0.87, "solarAbsorptivity": 0.8, "densityKgM3": 8400, "conductivityWmK": 12, "specificHeatJKgK": 420, "source": "authored-estimate", "reference": "incropera-2018"},
]
AREAS = {"casing": 0.22, "intake": 0.037, "control": 0.0058}
ANCHORS = {"foot-back": {"footprintLengthM": 0.025, "footprintWidthM": 0.025}, "foot-front": {"footprintLengthM": 0.025, "footprintWidthM": 0.025}}
BODY_LENGTH = 0.098
REFERENCES = [{"id": "retail-box-fan-heaters-2026", "citation": "Retailer and manufacturer data of 2000 W portable fan heaters with two heat settings, thermostat and carry handle, read 2026-11-20: Einhell EH 2000 22.1 x 19.8 x 31.7 cm; Black+Decker BXFSH2000E 20.2 x 26.8 x 31.6 cm, 1.5 kg; Steba VTH 2 27.5 x 26 x 20 cm, 1.8 kg; Beldray EH0567WK 26.5 x 22.5 x 12.5 cm, 1.15 kg; Logik L20PTC23 25.7 x 17 x 13 cm, 1.49 kg."}]


def bone(asset, bid):
    return next(b for b in asset["rig"]["bones"] if b["id"] == bid)


def roots(asset):
    """🌱 Every root correction in one place, applied to a copy of the catalog asset."""
    a = copy.deepcopy(asset)
    for m in MATERIALS:
        if not any(x["id"] == m["id"] for x in a["materials"]):
            a["materials"].append(copy.deepcopy(m))
    if BODY_LENGTH is not None:
        bone(a, "body")["lengthM"] = BODY_LENGTH
    for anchor in a["anchors"]:
        anchor.update(ANCHORS.get(anchor["id"], {}))
    for region in a["thermal"]["regions"]:
        region["areaM2"] = AREAS.get(region["id"], region["areaM2"])
    refs = a["provenance"]["references"]
    for r in REFERENCES:
        if not any(x["id"] == r["id"] for x in refs):
            refs.append(copy.deepcopy(r))
    texts = json.load(open(TEXTS, encoding="utf-8")) if os.path.exists(TEXTS) else {}
    if "provenanceNotes" in texts:
        a["provenance"]["notes"] = texts["provenanceNotes"]
    for old, new in texts.get("thermalAssumptions", []):
        for x in a["thermal"]["assumptions"]:
            for lang in ("en", "de"):
                x[lang] = x[lang].replace(old[lang], new[lang])
    if os.path.exists(DRAW):
        spec = importlib.util.spec_from_file_location("draw", DRAW)
        draw = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(draw)
        for den, rep in a["representations"].items():
            if rep["status"] != "authored":
                continue
            rep["geometry"] = draw.geometry(den, rep["geometry"])
            rep["decisions"] = draw.decisions(den, rep["decisions"])
    return a


def number(x):
    """🔢 A number as JSON.stringify writes it: integral values without a fraction, small values without an exponent down to 1e-6."""
    if isinstance(x, bool):
        return "true" if x else "false"
    if isinstance(x, int) or float(x).is_integer():
        return str(int(x))
    text = repr(float(x))
    if "e" in text and 1e-6 <= abs(x) < 1:
        text = format(x, ".20f").rstrip("0")
        while float(text[:-1]) == x:
            text = text[:-1]
    return text


def inline(value, indent=0):
    """🖨️ JSON in the style of this catalog file: JSON.stringify with two spaces, every array expanded."""
    pad = "  " * indent
    if isinstance(value, list):
        if not value:
            return "[]"
        return "[\n" + ",\n".join(pad + "  " + inline(x, indent + 1) for x in value) + "\n" + pad + "]"
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(pad + "  " + json.dumps(k, ensure_ascii=False) + ": " + inline(v, indent + 1) for k, v in value.items()) + "\n" + pad + "}"
    if isinstance(value, (int, float)) and not isinstance(value, bool):
        return number(value)
    return json.dumps(value, ensure_ascii=False)


def check_style():
    text = open(CATALOG, encoding="utf-8").read()
    have = json.loads(text)["asset"]
    for key in have:
        if not isinstance(have[key], (dict, list)):
            continue
        start, end = member_range(text, "    ", key)
        same = text[start:end] == "    " + json.dumps(key) + ": " + inline(have[key], 2)
        print(key, "same style" if same else "DIFFERENT STYLE")


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
    if "--check-style" in sys.argv:
        check_style()
        return
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
