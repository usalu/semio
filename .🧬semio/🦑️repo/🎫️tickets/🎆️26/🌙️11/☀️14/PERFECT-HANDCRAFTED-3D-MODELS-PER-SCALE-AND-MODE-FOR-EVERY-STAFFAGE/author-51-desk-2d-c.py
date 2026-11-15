import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_51_edit import edit_member
def fn(reps):
    for scale, y in (("50", 0.033), ("100", 0.0405), ("200", 0.026), ("500", 0.026)):
        for view in ("elevation", "front"):
            for sh in reps[scale]["geometry"][view]:
                if sh["id"] == "laptop" and sh["kind"] == "rect": sh["y" if view == "elevation" else "y"] = y if view == "elevation" else round(0.695 + y, 4)
    return reps
edit_member("height-adjustable-desk", "representations", fn)
