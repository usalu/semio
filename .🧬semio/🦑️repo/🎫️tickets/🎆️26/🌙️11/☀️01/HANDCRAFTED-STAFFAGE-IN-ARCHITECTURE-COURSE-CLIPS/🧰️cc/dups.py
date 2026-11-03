import json, collections, sys
man = json.load(open(sys.argv[1], encoding="utf-8"))["casts"]
def key(v, ignore):
    return tuple(sorted((k, json.dumps(x, sort_keys=True)) for k, x in v.items() if k not in ignore))
for ignore in [("mirror", "fps", "viewpoint", "scale")]:
    g = collections.defaultdict(list)
    for k, v in man.items():
        g[key(v, ignore)].append(k)
    print("ignore", ignore)
    for kk, ids in g.items():
        if len(ids) > 1:
            print("  ", ids)
# same asset/pose/clip/view/env/span but any other differences
g = collections.defaultdict(list)
for k, v in man.items():
    g[(v["asset"], v.get("pose"), v.get("clip"), v.get("view"), v.get("environment"), v.get("thermalSpanK"))].append(k)
print("ASSET+POSE+CLIP+VIEW+ENV+SPAN duplicates")
for kk, ids in g.items():
    if len(ids) > 1: print("  ", kk, ids)
