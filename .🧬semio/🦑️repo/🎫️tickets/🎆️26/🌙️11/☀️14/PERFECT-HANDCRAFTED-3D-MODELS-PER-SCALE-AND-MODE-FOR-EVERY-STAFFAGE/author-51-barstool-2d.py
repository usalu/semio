import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_15_asset import edit_member
def fn(reps):
    for k, pen in (("200", 0.18), ("500", 0.15)):
        g = reps[k]["geometry"]
        for view in ("elevation", "front"):
            out = []
            for sh in g[view]:
                if sh["id"] in ("leg-l", "leg-r", "leg-m", "ring") and sh["kind"] in ("poly", "capsule"):
                    if sh["kind"] == "capsule":
                        pts = [[sh["x0"], sh["y0"]], [sh["x1"], sh["y1"]]]
                        sh = {kk: vv for kk, vv in sh.items() if kk not in ("x0", "y0", "x1", "y1", "r0", "r1", "material")}
                        sh.update({"kind": "poly", "points": pts, "closed": False, "filled": False, "penMm": pen})
                    pts = sh["points"]
                    if sh["id"] == "ring": pts = [[-0.19, 0.27], [0.19, 0.27]]
                    else:
                        sx = 1 if pts[1][0] > 0 else -1 if pts[1][0] < 0 else 0
                        pts = [[pts[0][0], 0.7], [sx * 0.1975, 0]]
                    sh["points"] = pts
                out.append(sh)
            g[view] = out
    return reps
edit_member("bar-stool", "representations", fn)
