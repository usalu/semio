"""🌱 Round 3 domino gas hob: builds the candidate asset document with every root correction (rubber feet material, contact footprint, thermal areas measured on the 1:50 model, provenance, 2D drawings of every scale and view, representation decisions) from the catalog file and the drawing module, writes it to `🗑️generated/r3-gas-hob/🔣️.json` with the source volumes merged in, and with `--apply` writes the changed members into the catalog file in its own formatting (re-read right before, member order kept, `volumes` untouched).

Usage: python3 r3-gas-hob-roots.py [--apply]
"""
import copy
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/gas-hob/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-gas-hob", "🔣️.json")
SOURCE = os.path.join(HERE, "volumes", "appliances", "gas-hob.json")
TEXTS = os.path.join(HERE, "r3-gas-hob-texts.json")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])
_spec = importlib.util.spec_from_file_location("draw", os.path.join(HERE, "r3-gas-hob-draw.py"))
D = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(D)

AREAS = json.load(open(TEXTS, encoding="utf-8"))["areas"]
RUBBER = {"id": "rubber", "title": {"en": "Rubber (EPDM)", "de": "Gummi (EPDM)"}, "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 1150, "conductivityWmK": 0.25, "specificHeatJKgK": 1900, "source": "literature", "reference": "incropera-2018"}
REFERENCES = [
    {"id": "bsh-domino-gas-2024", "citation": "BSH product data sheets of 30 cm domino gas hobs (Bosch PRB326B90X, PRA3A6D70; Siemens ER3A6AD70), read 2026-11-20: 306 mm wide, 527 mm deep, 45 mm built-in height, cast-iron pan supports, rapid burners of 2.8 to 3 kW, the control knob at the front of the top plate."},
    {"id": "gev-burners-2015", "citation": "GEV spare-parts catalogue Gas 2015, chapter 5 gas burners (Sabaf series 1 and 2), read 2026-11-20: 3 kW burner caps of 73 to 90 mm diameter on crowns with a ring of flame ports below the cap edge."},
]


def roots(asset):
    """🌱 Every root correction in one place, applied to a copy of the catalog asset."""
    a = copy.deepcopy(asset)
    if not any(m["id"] == "rubber" for m in a["materials"]):
        a["materials"].append(RUBBER)
    contact = next(x for x in a["anchors"] if x["id"] == "base-contact")
    contact["footprintLengthM"] = 0.26
    contact["footprintWidthM"] = 0.455
    texts = json.load(open(TEXTS, encoding="utf-8"))
    a["structure"]["assumptions"][0] = texts["structureAssumption"]
    for region in a["thermal"]["regions"]:
        region["areaM2"] = AREAS.get(region["id"], region["areaM2"])
    refs = a["provenance"]["references"]
    for ref in REFERENCES:
        if not any(r["id"] == ref["id"] for r in refs):
            refs.append(ref)
    a["provenance"]["notes"] = texts["provenanceNotes"]
    for den, rep in a["representations"].items():
        if rep["status"] != "authored":
            continue
        rep["geometry"] = D.geometry(den, rep["geometry"])
        rep["decisions"] = D.decisions(den, rep["decisions"])
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
