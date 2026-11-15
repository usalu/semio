import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_15_asset import edit_member
def fn(reps):
    for k in ("100", "200", "500"):
        for sh in reps[k]["geometry"]["front"]:
            if sh["id"].startswith("leg-front-"):
                p = sh["points"]; sg = 1 if p[0][0] > 0 else -1
                p[0][0] = round(sg * 0.2395, 4)
    return reps
edit_member("cafe-chair", "representations", fn)
