"""🪵️ Ops writer of `used-pellet-boiler`: the four lashing straps fall to the floor by 98 degrees with their ratchets and top ends when they are released, and the two legs of the chain sling swing up by 70 degrees when the load is taken up, but their front and plan shapes were flat outlines of the rest pose (straps 521 to 534 mm, sling 189 to 198 mm off the side elevation). Strap legs, top ends and ratchets become their side elevation outline swept over their width, the chain legs round bars (`member`), the stitch strokes of the front and the link strokes of the plan of 1:50 slivers on their strap and chain: `python3 make/used-pellet-boiler.py` writes its ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("used-pellet-boiler")
bones = {b["id"] for b in a["rig"]["bones"] if b["id"].startswith("strap-") or b["id"].startswith("sling-")}
chains = {"chain-front", "chain-back"}
ops = solidify(a, bones, skip=chains | {"strap-stitch-r", "strap-stitch-l", "chain-links-front", "chain-links-back"})
frames = rest(a)
for scale in scales(a):
    for view in ("front", "plan"):
        for s in views(a, scale)[view]:
            if s["id"] in chains:
                if s["kind"] == "member":
                    ops.append(op(scale, view, s["id"], s))
                    continue
                e = shape(a, scale, "elevation", s["id"])
                ops.append(op(scale, view, s["id"], member(s, (e["x0"] + math.copysign(e["r0"], e["x0"] - e["x1"]), e["y0"], 0), (e["x1"] + math.copysign(e["r0"], e["x1"] - e["x0"]), e["y1"], 0), 2 * e["r0"])))
for side, bone in (("r", "strap-r-front"), ("l", "strap-l-front")):
    s = find(a, "50", "front", f"strap-stitch-{side}")
    if s is not None:
        if "solid" in s:
            ops.append(op("50", "front", s["id"], s))
        else:
            (x0, y), (x1, _) = s["points"]
            ops.append(op("50", "front", s["id"], sliver(s, [to_local(frames[bone], (frames[bone][0] - 0.001, y))], min(x0, x1), max(x0, x1))))
for end in ("front", "back"):
    s = find(a, "50", "plan", f"chain-links-{end}")
    if s is not None:
        if "solid" in s:
            ops.append(op("50", "plan", s["id"], s))
        else:
            bone = f"sling-{end}"
            (x0, z), (x1, _) = s["points"]
            v = shape(a, "50", "elevation", f"chain-{end}")["y0"]
            ops.append(op("50", "plan", s["id"], sliver(s, sorted([to_local(frames[bone], (x0, frames[bone][1] + v)), to_local(frames[bone], (x1, frames[bone][1] + v))]), z - 0.001, z + 0.001)))
write("used-pellet-boiler", ops)
