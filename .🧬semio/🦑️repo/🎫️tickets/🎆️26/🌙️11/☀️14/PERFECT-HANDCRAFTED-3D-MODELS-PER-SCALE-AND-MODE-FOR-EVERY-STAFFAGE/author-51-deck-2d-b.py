import sys, copy
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_51_edit import edit_member
def fn(reps):
    el = reps["500"]["geometry"]["elevation"]
    src = next(s for s in reps["500"]["geometry"]["front"] if s["id"] == "canvas-seat")
    assert not any(s["id"] == "canvas-seat" for s in el)
    s = copy.deepcopy(src); s.pop("solid", None); s["z"] = max(x.get("z", 0) for x in el) + 1
    el.append(s)
    return reps
edit_member("folding-deck-chair", "representations", fn)
