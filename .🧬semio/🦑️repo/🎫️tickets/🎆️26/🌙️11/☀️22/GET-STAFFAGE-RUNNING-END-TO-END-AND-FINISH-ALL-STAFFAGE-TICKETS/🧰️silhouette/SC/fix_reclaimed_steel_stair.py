"""🪜️ Repair of `reclaimed-steel-stair`: the declared length of 4.974 m is no length of the flight as it lies (base plate overhang to the tip of the handrail: 4.9524 m in the body and in every drawing of 1:50 and 1:100). (1) `dimensions.widthM` becomes 4.952 m (the manifest row follows, see manifest-changes.json). (2) The body of 1:200 had no base plates under the floor cut and began 37 mm later (4.915 m): it gets the two base plates of 1:100, thickened to 31 mm into the stringer (one smallest feature of 1:200, as 1:100 thickens the 10 mm plate to its 15 mm), and its drawings show them at that thickness. 1:500 stays without (37 mm, within its limit). Idempotent: python3 fix_reclaimed_steel_stair.py"""
import copy, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "reclaimed-steel-stair"
text, doc = sc.load(ID)
doc["asset"]["dimensions"]["widthM"] = 4.952
for side in ("l", "r"):
    plate = copy.deepcopy(sc.solid(doc, 100, f"base-plate-{side}"))
    plate["to"] = 0.021
    sc.put(sc.solids(doc, 200), plate)
for view in ("elevation", "plan", "front"):
    for plate in [s for s in sc.shapes(doc, 100, view) if s["id"].startswith("base-plate-")]:
        drawn = copy.deepcopy(plate)
        drawn["points"] = [[-0.187988, 0.127885], [0.176022, -0.14391], [0.194569, -0.11907], [-0.169441, 0.152725]]
        sc.put(sc.shapes(doc, 200, view), drawn)
sc.save(ID, text, doc)
