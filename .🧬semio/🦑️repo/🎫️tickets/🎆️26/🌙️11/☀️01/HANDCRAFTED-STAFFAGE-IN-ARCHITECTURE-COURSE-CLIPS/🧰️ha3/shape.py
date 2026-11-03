import json, sys

def shape(v, depth=0, maxd=3):
    pad = "  " * depth
    if isinstance(v, dict):
        for k, x in v.items():
            if isinstance(x, (dict, list)) and depth < maxd:
                n = len(x)
                print(f"{pad}{k}: {type(x).__name__}[{n}]")
                shape(x, depth + 1, maxd)
            else:
                s = json.dumps(x, ensure_ascii=False)
                print(f"{pad}{k}: {s[:100]}")
    elif isinstance(v, list):
        if v and depth < maxd:
            shape(v[0], depth + 1, maxd)
            if len(v) > 1:
                print(f"{pad}... ({len(v)} items; first shown)")

with open(sys.argv[1], encoding="utf-8") as f:
    d = json.load(f)
maxd = int(sys.argv[2]) if len(sys.argv) > 2 else 3
shape(d, 0, maxd)
