import sys; sys.path.insert(0, ".")
from lib import *
doc = load("objects")
WH = {"id": "wood-handbook-2021", "citation": "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (mechanical properties and density of timber species; the values used here are rounded air-dry estimates guided by this source, not table look-ups)."}
def walk(o):
    if isinstance(o, dict):
        if o.get("reference") == "wood-handbook":
            o["reference"] = "wood-handbook-2021"; o["source"] = "authored-estimate"
        for v in o.values(): walk(v)
    elif isinstance(o, list):
        for v in o: walk(v)
for a in doc["assets"]:
    walk(a)
    refs = a["provenance"]["references"]
    for i, r in enumerate(refs):
        if r["id"] == "wood-handbook": refs[i] = dict(WH)
save("objects", doc)
