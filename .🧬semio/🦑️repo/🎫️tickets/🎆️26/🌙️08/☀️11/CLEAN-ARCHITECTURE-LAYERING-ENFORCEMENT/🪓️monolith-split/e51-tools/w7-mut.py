"""🔧 E51 wave-7 mutation leaves: widens every field that a cross-crate struct literal or pattern names, for each `*-mutations` leaf struct of the scan JSON (arg 1), to `pub` through the journalled editor.
usage: w7-mut.py <scan-json> [--apply]"""
import collections, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from widen import widen

d = json.load(open(sys.argv[1], encoding="utf-8"))
defs = collections.OrderedDict()
for h in d.get("hits") or (d["world_only"] + d["sites"]):
    D = h["defs"][0]
    if "mutations" not in D["owner"]:
        continue
    e = defs.setdefault((D["rel"], h["name"], D["line"]), set())
    e.update(D["bad"])
print(len(defs), "mutation leaf structs")
for (rel, name, line), bad in defs.items():
    if "--apply" in sys.argv:
        widen(rel, name, line, "pub", sorted(bad))
    else:
        print(rel[-60:], name, line, sorted(bad))
