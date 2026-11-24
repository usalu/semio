"""🪜️ Repair of `reclaimed-precast-stair`: the four cut bar stubs (10 mm, 49 mm beyond the lower end) make the declared 3.2958 m. (1) Side elevation of 1:50: the stub was drawn from the end face outwards only, the bars of the body run on through the notch of the bearing nib into the concrete (15.5 mm): the stub is drawn over the whole length of its bar. (2) The bodies of 1:100 and 1:200 had no stubs and were 3.247 m and 3.254 m long: they get the four bars of 1:50, and their side elevation and plan draw them as 1:50 does. Idempotent: python3 fix_reclaimed_precast_stair.py"""
import copy, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "reclaimed-precast-stair"
text, doc = sc.load(ID)
stub = sc.shape(doc, 50, "elevation", "bar-stub")
stub["points"] = [[-0.101295, 0.152405], [-0.18151, 0.15774], [-0.17997, 0.16762], [-0.099765, 0.162295]]
for scale in (100, 200):
    for n in (1, 2, 3, 4):
        sc.put(sc.solids(doc, scale), copy.deepcopy(sc.solid(doc, 50, f"bar-stub-{n}")))
        sc.put(sc.shapes(doc, scale, "plan"), copy.deepcopy(sc.shape(doc, 50, "plan", f"bar-stub-{n}")))
    sc.put(sc.shapes(doc, scale, "elevation"), copy.deepcopy(stub))
sc.save(ID, text, doc)
