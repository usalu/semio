import json, sys
def walk(o, p, depth, maxd):
    if depth > maxd:
        return
    if isinstance(o, dict):
        for k, v in o.items():
            t = type(v).__name__
            n = f" len={len(v)}" if isinstance(v, (list, dict)) else f" = {str(v)[:70]!r}"
            print("  " * depth + f"{k}:{t}{n if not isinstance(v,(dict,list)) else n}")
            walk(v, p + [k], depth + 1, maxd)
    elif isinstance(o, list) and o and depth < maxd:
        walk(o[0], p + ["[0]"], depth + 1, maxd)
d = json.load(open(sys.argv[1], encoding="utf-8"))
walk(d, [], 0, int(sys.argv[2]))
