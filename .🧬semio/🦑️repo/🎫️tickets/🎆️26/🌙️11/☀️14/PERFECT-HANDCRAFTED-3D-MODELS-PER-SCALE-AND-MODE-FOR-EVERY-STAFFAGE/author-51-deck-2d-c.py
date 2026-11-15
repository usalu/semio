import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_51_edit import edit_member
def fn(reps):
    for s in reps["500"]["geometry"]["elevation"]:
        if s["id"] == "canvas-seat": s["filled"] = False
    return reps
edit_member("folding-deck-chair", "representations", fn)
