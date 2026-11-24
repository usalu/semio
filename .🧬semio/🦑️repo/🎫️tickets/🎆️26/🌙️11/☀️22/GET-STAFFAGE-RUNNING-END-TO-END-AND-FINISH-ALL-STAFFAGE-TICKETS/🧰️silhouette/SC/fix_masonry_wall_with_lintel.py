"""🧱️ Repair of `masonry-wall-with-lintel`: the front section of 1:100 left the sliver between the sloped sill and the reveal undrawn (the facing beyond the cut is visible there, 127.5 mm from the next drawn edge); it gets the `sill-end-beyond` wedge that 1:50 draws. Idempotent: python3 fix_masonry_wall_with_lintel.py"""
import copy, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "masonry-wall-with-lintel"
text, doc = sc.load(ID)
wedge = copy.deepcopy(sc.shape(doc, 50, "front", "sill-end-beyond"))
sc.put(sc.shapes(doc, 100, "front"), wedge, after="reveal")
sc.save(ID, text, doc)
