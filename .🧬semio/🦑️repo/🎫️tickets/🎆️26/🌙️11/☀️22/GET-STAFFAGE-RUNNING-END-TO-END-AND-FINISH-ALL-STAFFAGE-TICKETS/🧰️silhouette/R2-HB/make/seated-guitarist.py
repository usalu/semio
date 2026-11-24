"""🎸️ Repair of `seated-guitarist` 1:50 and 1:100 (volume): each of the six strings was one slender loft bent at the nut (bridge, nut, tuner post). The guard takes centre line and width of a slender solid from its mesh and gave the bent strings widths of 9 to 49 mm (true 0.6 to 1.3 mm), a band of 23 mm beside the neck in the plan. Each string becomes two straight lofts: the speaking length from bridge to nut (the id it had) and the run from the nut to its tuner post (`<id>-head`), with thermal and structure entries like the string. `python3 make/seated-guitarist.py`"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
import lib

asset = lib.asset_of("seated-guitarist")
ops = []
for scale in ("50", "100"):
    volume = asset["volumes"][scale]
    for n in range(1, 7):
        sid = f"guitar-string-{n}"
        solid = next(s for s in volume["body"] if s["id"] == sid)
        if len(solid["sections"]) == 3:
            bridge, nut, post = solid["sections"]
        else:
            bridge, nut = solid["sections"]
            post = next(s for s in volume["body"] if s["id"] == sid + "-head")["sections"][1]
        ops.append({"op": "solid", "scale": scale, "id": sid, "solid": {**solid, "sections": [bridge, nut]}})
        ops.append({"op": "solid-add", "scale": scale, "after": sid, "solid": {**solid, "id": sid + "-head", "sections": [nut, post]}})
        ops.append({"op": "solid-add", "scale": scale, "list": "thermal", "after": "th-" + sid, "solid": {"id": f"th-{sid}-head", "of": sid + "-head", "region": "guitar"}})
        ops.append({"op": "solid-add", "scale": scale, "list": "structure", "after": "st-" + sid, "solid": {"id": f"st-{sid}-head", "of": sid + "-head", "member": "guitar", "part": "member"}})
lib.write_ops("seated-guitarist", ops)
