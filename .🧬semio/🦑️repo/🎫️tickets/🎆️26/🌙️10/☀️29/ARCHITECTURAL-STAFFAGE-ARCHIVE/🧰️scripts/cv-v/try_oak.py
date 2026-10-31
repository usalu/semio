import sys, json; sys.path.insert(0,'.')
from lib2 import *
import d_oak
O = load_orig("english-oak")["asset"]; R = Rig(O)
res = d_oak.design(R, O)
for k, shapes in res.items():
    pens = PENS[k]
    a = {"representations": {k: {"pens": pens, "geometry": {"elevation": shapes}}}, "rig": O["rig"], "symbols": []}
    json.dump({"asset": a}, open(f"{OUT}/t-oak-{k}.json", "w"))
    print(k, [round(v,2) for v in bbox_shapes(R, shapes)])
