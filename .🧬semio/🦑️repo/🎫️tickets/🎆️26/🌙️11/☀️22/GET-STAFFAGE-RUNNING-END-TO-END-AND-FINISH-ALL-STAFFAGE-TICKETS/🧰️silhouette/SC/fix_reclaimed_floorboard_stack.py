"""🪵️ Repair of `reclaimed-floorboard-stack`: the side elevation of 1:50 drew the upper part of each round sling from 1.092 m up, the side part up to 1.080 m, and left a 12 mm notch between them where the body of the sling is continuous down to 1.0475 m (15.1 mm from the next edge): the upper part is drawn down to where its solid ends. Idempotent: python3 fix_reclaimed_floorboard_stack.py"""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "reclaimed-floorboard-stack"
text, doc = sc.load(ID)
for n in (1, 2):
    sc.shape(doc, 50, "elevation", f"sling-{n}-top")["y0"] = -0.052
sc.save(ID, text, doc)
