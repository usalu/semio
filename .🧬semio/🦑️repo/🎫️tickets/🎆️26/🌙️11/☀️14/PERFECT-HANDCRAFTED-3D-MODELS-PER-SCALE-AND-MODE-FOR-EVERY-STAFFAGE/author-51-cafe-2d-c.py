import sys, copy
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_15_asset import edit_member
def fn(reps):
    g2, g5 = reps["200"]["geometry"], reps["500"]["geometry"]
    for view in ("elevation", "front"):
        assert not any(s["id"] == "back-panel" for s in g5[view])
        src = next(s for s in g2[view] if s["id"] == "back-panel")
        s = copy.deepcopy(src)
        if view == "elevation":
            for kk in ("x0", "y0", "x1", "y1", "r0", "r1"): s.pop(kk, None)
            s.update({"kind": "rect", "x": 0.195, "y": -0.0435, "w": 0.19, "h": 0.075})
        else:
            s.update({"x": -0.17, "y": 0.654, "w": 0.34, "h": 0.19}); s.pop("r", None)
        s["z"] = max(x.get("z", 0) for x in g5[view]) + 1
        g5[view].append(s)
    return reps
edit_member("cafe-chair", "representations", fn)
