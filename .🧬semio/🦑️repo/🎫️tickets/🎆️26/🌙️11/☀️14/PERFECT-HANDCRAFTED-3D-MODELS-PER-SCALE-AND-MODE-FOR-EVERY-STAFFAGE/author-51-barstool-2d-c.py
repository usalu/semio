import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_15_asset import edit_member
D = "M 0.06 0.02 L 0.135 0.02 Q 0.15 0.02 0.15 0.005 L 0.15 -0.034 Q 0.15 -0.049 0.135 -0.049 L 0.06 -0.049 Z"
def fn(reps):
    for k in ("50", "100", "200", "500"):
        for view, l in reps[k]["geometry"].items():
            for sh in l:
                if sh["bone"] != "backrest": continue
                if sh["id"] == "back-post" and k in ("50", "100"): sh["w"] = 0.1
                if sh["id"] == "back-pad":
                    if k in ("50", "100"): sh["d"] = D
                    else: sh.update({"x": 0.06, "y": -0.049, "w": 0.09, "h": 0.069})
                    sh["solid"]["z0"], sh["solid"]["z1"] = -0.105, 0.105
                if sh["id"] == "back":
                    sh.update({"x": 0.03, "y": -0.055, "w": 0.12, "h": 0.075})
                    sh["solid"]["z0"], sh["solid"]["z1"] = -0.105, 0.105
    return reps
edit_member("bar-stool", "representations", fn)
