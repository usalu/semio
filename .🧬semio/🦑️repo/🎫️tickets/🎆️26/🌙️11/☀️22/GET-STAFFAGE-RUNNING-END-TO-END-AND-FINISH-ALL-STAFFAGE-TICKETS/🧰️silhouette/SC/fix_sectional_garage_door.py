"""🚗️ Repair of `sectional-garage-door` (partial, see the report for what stays). (1) The two diagonal braces of the horizontal tracks were cut square 16.7 mm too long, so their upper corner stood 11 mm above the ceiling they are fixed to (body 2.8611 m against the declared 2.85 m): they end where that corner meets the ceiling, in every body that has them and in every side elevation that draws them. (2) Side elevation of 1:100: the ceiling foot of the brace was not drawn (55.7 mm): it is drawn as at 1:50. Idempotent: python3 fix_sectional_garage_door.py"""
import copy, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "sectional-garage-door"
text, doc = sc.load(ID)
for scale in (50, 100, 200, 500):
    if doc["asset"]["volumes"][str(scale)]["status"] != "authored":
        continue
    for solid in sc.solids(doc, scale):
        if solid["id"] in ("brace-l", "brace-r") and solid["kind"] == "extrude" and solid.get("to") in (0.403609, 0.38694):
            solid["to"] = 0.38694
    for shape in sc.shapes(doc, scale, "elevation"):
        if shape["id"] == "brace" and shape.get("kind") == "poly" and len(shape["points"]) == 4 and shape["points"][0] == [2.47497, 2.56885]:
            shape["points"] = [[2.47497, 2.56885], [2.18736, 2.8277], [2.20742, 2.85], [2.49503, 2.59115]]
sc.put(sc.shapes(doc, 100, "elevation"), copy.deepcopy(sc.shape(doc, 50, "elevation", "brace-foot")), after="brace")
sc.save(ID, text, doc)
