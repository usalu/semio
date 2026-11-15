import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_15_asset import edit_member
def fn(reps):
    for k in ("200", "500"):
        for view, l in reps[k]["geometry"].items():
            for sh in l:
                if sh["id"] in ("leg-l", "leg-r", "leg-m", "ring", "leg-x", "leg-y") and sh["kind"] == "poly": sh.pop("penMm", None)
    return reps
edit_member("bar-stool", "representations", fn)
