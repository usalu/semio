import json, sys
d = json.load(open(sys.argv[1], encoding="utf-8"))
def walk(o, depth, maxd, path=""):
    if isinstance(o, dict):
        for k, v in o.items():
            if isinstance(v, (dict, list)) and depth < maxd:
                print("  " * depth + f"{k}: {type(v).__name__}[{len(v)}]")
                walk(v, depth + 1, maxd)
            else:
                s = json.dumps(v, ensure_ascii=False)
                print("  " * depth + f"{k}: {s[:100]}")
    elif isinstance(o, list):
        for i, v in enumerate(o[:6]):
            if isinstance(v, (dict, list)) and depth < maxd:
                print("  " * depth + f"[{i}]")
                walk(v, depth + 1, maxd)
walk(d, 0, int(sys.argv[2]))
