import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_51_edit import edit_member
LEN = {"rear": 1.045, "front": 0.726, "seat": 0.495}
def fn(reps):
    for k in ("200", "500"):
        for sh in reps[k]["geometry"]["elevation"]:
            if sh["id"] in LEN and sh["kind"] == "poly" and not sh["closed"]:
                L = LEN[sh["id"]]
                sh.update({"points": [[0.0, 0.0225], [L, 0.0225], [L, -0.0225], [0.0, -0.0225]], "closed": True, "filled": True})
    return reps
edit_member("folding-deck-chair", "representations", fn)
