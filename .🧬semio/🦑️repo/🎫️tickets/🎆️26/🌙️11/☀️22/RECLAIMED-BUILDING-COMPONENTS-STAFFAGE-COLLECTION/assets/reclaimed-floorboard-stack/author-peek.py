"""🔍 Print a dotted path of a JSON asset document, truncated: python3 author-peek.py <file> <dotted.path> [maxchars]."""
import json, sys
d = json.load(open(sys.argv[1]))
for k in [x for x in (sys.argv[2] if len(sys.argv) > 2 else "").split(".") if x]:
    d = d[int(k)] if isinstance(d, list) else d[k]
n = int(sys.argv[3]) if len(sys.argv) > 3 else 3000
if isinstance(d, dict):
    for k, v in d.items():
        s = json.dumps(v, ensure_ascii=False)
        print(f"{k}: {len(s)}  {s[:n]}")
else:
    print(json.dumps(d, ensure_ascii=False)[:n])
