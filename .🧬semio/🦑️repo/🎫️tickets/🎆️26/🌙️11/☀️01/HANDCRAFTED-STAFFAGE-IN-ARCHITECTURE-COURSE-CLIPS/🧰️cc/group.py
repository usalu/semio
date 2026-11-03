import json, collections, sys
p = sys.argv[1]
d = json.load(open(p, encoding="utf-8"))["casts"]
g = collections.defaultdict(list)
for k, v in d.items():
    g[v["asset"]].append((k, v))
for asset in sorted(g):
    items = g[asset]
    if len(items) < 2:
        continue
    print(f"== {asset} ({len(items)})")
    for k, v in items:
        rest = {x: y for x, y in v.items() if x not in ("asset",)}
        print("  ", k, json.dumps(rest, ensure_ascii=False))
