import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
from author_51_edit import edit_member
def fn(reps):
    g = reps["500"]["geometry"]
    for view in ("elevation", "front"):
        for sh in g[view]:
            if sh["id"].startswith("leg-") and sh["kind"] == "poly" and not sh["closed"]:
                cx = sh["points"][0][0]
                sh.update({"points": [[round(cx - 0.03, 4), 0.0], [round(cx + 0.03, 4), 0.0], [round(cx + 0.03, 4), 0.64], [round(cx - 0.03, 4), 0.64]], "closed": True, "filled": True})
                sh.pop("penMm", None)
    return reps
edit_member("dining-table", "representations", fn)
