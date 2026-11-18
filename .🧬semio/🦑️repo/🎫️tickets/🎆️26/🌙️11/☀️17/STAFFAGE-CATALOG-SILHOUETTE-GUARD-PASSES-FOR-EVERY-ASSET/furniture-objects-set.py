"""usage: furniture-objects-set.py <category> <slug> <scale> <view> <shape|vol:solid> <key> <json-value> [more pairs]: sets keys of one drawn shape (or volume solid) in place; missing keys are added by re-emitting that single object."""
import importlib.util, os, sys, json
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
cat, slug, sc, view, sid = sys.argv[1:6]
d = l.doc(cat, slug)
n = d.solid(int(sc), sid[4:]) if sid.startswith("vol:") else d.shape(int(sc), view, sid)
kv = sys.argv[6:]; o = d.obj(n)
if all(kv[i] in o for i in range(0, len(kv), 2)):
    for i in range(0, len(kv), 2): d.set(n, kv[i], json.loads(kv[i + 1]))
else:
    for i in range(0, len(kv), 2): o[kv[i]] = json.loads(kv[i + 1])
    d.replace_obj(n, o)
d.commit()
