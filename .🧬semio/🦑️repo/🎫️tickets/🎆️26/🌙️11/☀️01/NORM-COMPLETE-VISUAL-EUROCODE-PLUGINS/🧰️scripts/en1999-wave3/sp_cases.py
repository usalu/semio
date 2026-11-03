"""Writes the three free-outline torsion cases (solid, closed box, I polygon) into third-party-extended.json with values from sectionproperties (adapter en1999.section.polygon-holes)."""
import json, importlib.util, sys
sys.path.insert(0, ".")
import run_proto as rp
spec = importlib.util.spec_from_file_location("spa", "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️harness/🔌️adapters/sectionproperties/🐍️.py")
spa = importlib.util.module_from_spec(spec); spec.loader.exec_module(spa)
TARGET = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases/third-party-extended.json"
NAMES = {"solid70x140": ("en1999.section.free-outline-solid-torsion", "solid rectangle 70 x 140 mm"), "box100x60x4": ("en1999.section.free-outline-box-torsion", "closed box 100 x 60 x 4 mm"), "I120": ("en1999.section.free-outline-i-torsion", "I section 100 x 120 mm, flanges 8 mm, web 6 mm")}
doc = json.load(open(TARGET, encoding="utf-8"))
have = {c["id"] for c in doc["cases"]}
for key, (cid, what) in NAMES.items():
    if cid in have:
        continue
    o, h = rp.cases[key]
    inputs = {"outer": [list(p) for p in o], "holes": [[list(p) for p in r] for r in h], "mesh": 4.0e-6}
    r = spa.polygon_holes(inputs)
    exp = {"It": {"value": r["It"], "unit": "m^4", "profile": "method-spread-v1"}, "Iw": {"value": r["Iw"], "unit": "m^6", "profile": "method-spread-v1"}}
    doc["cases"].append({"id": cid, "check": "en1999.section.polygon-holes", "citation": {"standard": "EN 1999-1-1", "edition": "2004", "clause": "6.2.7 and 6.3.2 torsion constant and warping constant of a free outline: " + what}, "inputs": inputs, "expected": exp, "references": [{"oracle": "norm-eurocode-sectionproperties", "adapter": "sectionproperties", "kind": "third-party-library"}]})
    print(cid, r["It"], r["Iw"], flush=True)
json.dump(doc, open(TARGET, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
open(TARGET, "a", encoding="utf-8").write("\n")
