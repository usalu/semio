"""🧱 Rework 19, cavity wall with lintel: the 27 stainless wall ties (4 mm wire, every third course, doubled beside the opening) were joints of the structure list only; at 1:50 they are bodies too, bedded in both leaves and crossing the full-fill cavity, so the cut model shows what holds the facing leaf."""
import copy
import sys

sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE")
import rework_19_lib as L

SLUG = "masonry-wall-with-lintel"
NOTE = {"en": "The wall ties are bodies at this scale: 4 mm stainless wire in every third bed joint, embedded in both leaves across the insulated cavity and doubled along the reveals.", "de": "Die Luftschichtanker sind in diesem Maßstab Körper: 4-mm-Edelstahldraht in jeder dritten Lagerfuge, in beide Schalen eingebettet, quer durch die gedämmte Luftschicht und entlang der Laibungen verdoppelt."}
doc = L.load_source(SLUG)
v = doc["volumes"]["50"]
if not any(x["id"].startswith("wall-tie-") for x in v["body"]):
    for s in v["structure"]:
        if s["id"].startswith("tie-"):
            b = {k: copy.deepcopy(val) for k, val in s.items() if k not in ("member", "part")}
            b["id"], b["role"] = "wall-" + s["id"], "detail"
            v["body"].append(b)
    v["decisions"].append(NOTE)
L.save_source(SLUG, doc)
