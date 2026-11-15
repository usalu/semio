import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_15_asset import edit_member
def fn(reps):
    for k in ("100", "200", "500"):
        for sh in reps[k]["geometry"]["plan"]:
            if sh["kind"] == "capsule" and (sh["id"].startswith("leg-") or sh["id"].startswith("back-post")):
                pts = [[sh["x0"], sh["y0"]], [sh["x1"], sh["y1"]]]
                for key in ("x0", "y0", "x1", "y1", "r0", "r1"): sh.pop(key)
                sh.update({"kind": "poly", "points": pts, "closed": False, "filled": False})
    return reps
edit_member("cafe-chair", "representations", fn)
