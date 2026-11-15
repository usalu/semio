import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_51_edit import edit_member
def fn(reps):
    for scale in ("200", "500"):
        for view, lst in reps[scale]["geometry"].items():
            for sh in lst: sh.pop("r", None)
    return reps
edit_member("height-adjustable-desk", "representations", fn)
