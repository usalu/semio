"""🧱️ Repair of `reclaimed-paving-slab-pallet` at 1:50. (1) Plan: each steel band was drawn as a closed unfilled outline around its slab pack, which the comparison reads as the edge of the pack (filled to the centre line) while the 0.9 mm band solids keep their true width; the 5 mm sliver between the pack and the outer deck board then stayed open in the body and closed in the drawing (148.1 mm in the tiles of the guard). The band is drawn as what it is, a stroke around the pack (an open line that starts and ends in the middle of its rear side), which the band solids follow at its pen width. (2) Front elevation: only the band of the front pack was drawn; the band of the rear pack lies behind it and had no drawing on its own bone, so its solids were compared at an inflated width (16.9 mm beyond the band ends): the rear band is drawn too, right under the front one, as it shows when its pack moves on its own. Idempotent: python3 fix_reclaimed_paving_slab_pallet.py"""
import copy, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "reclaimed-paving-slab-pallet"
text, doc = sc.load(ID)
for name, x0, x1 in (("band-rear-loop", 0.02455, 0.57545), ("band-front-loop", 0.62455, 1.17545)):
    loop = sc.shape(doc, 50, "plan", name)
    loop["points"] = [[x0, 0.0], [x0, -0.25045], [x1, -0.25045], [x1, 0.25045], [x0, 0.25045], [x0, 0.0]]
    loop["closed"] = False
front = copy.deepcopy(sc.shape(doc, 50, "front", "band-front-end"))
front.update({"id": "band-rear-end", "bone": "band-rear", "z": sc.r6(front["z"] - 0.1)})
sc.put(sc.shapes(doc, 50, "front"), front, before="band-front-end")
sc.save(ID, text, doc)
