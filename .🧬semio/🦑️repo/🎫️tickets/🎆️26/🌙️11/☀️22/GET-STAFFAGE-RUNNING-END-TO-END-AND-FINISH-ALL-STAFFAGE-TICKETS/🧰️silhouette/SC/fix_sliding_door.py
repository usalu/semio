"""🚪️ Repair of `sliding-door`. (1) Side elevation of 1:200: the leaf was drawn up to the underside of the track, the body ends 50 mm below it and hangs on its two trolleys (150 mm in the tiles of the guard): the leaf is drawn to its own top edge as at 1:100. (2) Side elevation of 1:500: the body carries the leaf on two trolley blocks that close the 50 mm slot under the track, the drawing left the slot open over the whole leaf (157.4 mm): the two trolley blocks are drawn between the leaf and the track. Idempotent: python3 fix_sliding_door.py"""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "sliding-door"
text, doc = sc.load(ID)
sc.shape(doc, 200, "elevation", "leaf-frame-200")["points"] = [[2.09, -0.5], [2.09, 0.5], [0.1, 0.5], [0.1, -0.5]]
for n, (y0, y1) in enumerate(((-0.395, -0.305), (0.305, 0.395)), start=1):
    sc.put(sc.shapes(doc, 500, "elevation"), {"id": f"trolley-{n}", "bone": "leaf", "role": "body", "z": 1.5, "filled": True, "tone": 0.3, "region": "leaf-frame", "material": "steel", "kind": "poly", "points": [[0.1, y0], [0.1, y1], [0.05, y1], [0.05, y0]], "closed": True}, after="leaf")
sc.save(ID, text, doc)
