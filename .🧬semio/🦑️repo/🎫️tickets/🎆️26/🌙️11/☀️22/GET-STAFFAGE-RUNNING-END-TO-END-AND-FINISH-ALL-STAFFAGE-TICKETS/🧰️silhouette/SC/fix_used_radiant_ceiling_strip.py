"""🌡️ Repair of `used-radiant-ceiling-strip`: the front elevation of 1:50 and 1:100 drew the flow stub with its cap on one side of the panel and not the return stub of the far end, which stands just as high on the other side and is hidden by nothing (45 mm in the tiles of the guard): the return stub is drawn as the mirror image of the flow stub. Idempotent: python3 fix_used_radiant_ceiling_strip.py"""
import copy, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "used-radiant-ceiling-strip"
text, doc = sc.load(ID)
for scale in (50, 100):
    front = sc.shapes(doc, scale, "front")
    for flow in [s for s in front if s["id"].startswith("flow-")]:
        assert flow["kind"] == "rect", flow["kind"]
        mirror = copy.deepcopy(flow)
        mirror.update({"id": "return-" + flow["id"][5:], "x": sc.r6(-flow["x"] - flow["w"]), "z": sc.r6(flow["z"] - 0.5)})
        sc.put(front, mirror, after=flow["id"])
sc.save(ID, text, doc)
