"""🪢 Market parasol: the 2D knuckle is drawn just above the mast head (same layer band), so the tilted canopy covers it in the front elevation as it covers the mast head."""
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/market-parasol/🔣️.json"
document = json.load(open(CATALOG, encoding="utf-8"))
for scale, rep in document["asset"]["representations"].items():
    for view in ("elevation", "front"):
        shapes = rep["geometry"][view]
        head = next(s for s in shapes if s["id"] == "mast-upper")
        for s in shapes:
            if s["id"] == "mast-knuckle":
                s["z"] = round(head["z"] + 0.05, 3)
                print(scale, view, "mast-upper z", head["z"], "mast z", next(m["z"] for m in shapes if m["id"] == "mast"), "knuckle z", s["z"])
scope = {}
exec(open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read().split("\ndef member_end")[0], scope)
open(CATALOG, "w", encoding="utf-8").write(scope["dump"](document, 0) + "\n")
