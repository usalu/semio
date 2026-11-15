import sys, copy
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_15_asset import edit_member
def fn(reps):
    g2, g5 = reps["200"]["geometry"], reps["500"]["geometry"]
    for view in ("elevation", "plan"):
        for src in [s for s in g2[view] if s["id"].startswith("stretcher")]:
            assert not any(s["id"] == src["id"] for s in g5[view])
            s = copy.deepcopy(src); s["role"] = "body"; s["z"] = max(x.get("z", 0) for x in g5[view]) + 1
            g5[view].append(s)
    return reps
edit_member("cafe-chair", "representations", fn)
