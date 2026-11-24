"""🏛️ Repair of `reclaimed-steel-column` at 1:50: the 25 mm round sling lying on the column was declared `slender`, which compares it as a stroke with a square cap half a width beyond its rounded tip (20.6 mm in the side elevation); at 1:50 it is more than three smallest features thick, a body of its own, and is compared as the capsule it is; the front and the plan draw it to its tips. Idempotent: python3 fix_reclaimed_steel_column.py"""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "reclaimed-steel-column"
text, doc = sc.load(ID)
sc.solid(doc, 50, "sling-leg").pop("slender", None)
for view in ("front", "plan"):
    sc.shape(doc, 50, view, "sling-leg")["points"] = [[-0.0125, -0.0125], [1.0125, -0.0125], [1.0125, 0.0125], [-0.0125, 0.0125]]
sc.save(ID, text, doc)
