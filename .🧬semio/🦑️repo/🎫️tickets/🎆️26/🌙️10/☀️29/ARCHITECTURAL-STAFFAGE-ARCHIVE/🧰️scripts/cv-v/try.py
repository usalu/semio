import sys, json, importlib; sys.path.insert(0,'.')
from lib2 import *
a, mod = sys.argv[1], sys.argv[2]
O = load_orig(a)["asset"]; R = Rig(O)
res = importlib.import_module(mod).design(R, O)
for k, shapes in res.items():
    p = {"representations": {k: {"pens": PENS[k], "geometry": {"elevation": shapes}}}, "rig": O["rig"], "symbols": []}
    json.dump({"asset": p}, open(f"{OUT}/t-{a}-{k}.json", "w"))
    print(k, len(shapes), [round(v,2) for v in bbox_shapes(R, shapes)])
