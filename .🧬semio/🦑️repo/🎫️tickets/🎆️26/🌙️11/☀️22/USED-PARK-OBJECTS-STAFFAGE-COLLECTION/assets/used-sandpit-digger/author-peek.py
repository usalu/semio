"""🔍 Peek into a catalog asset document section by section."""
import json, sys
p = sys.argv[1]
path = sys.argv[2:] 
d = json.load(open(p))
o = d
for k in path:
    o = o[int(k)] if isinstance(o, list) else o[k]
if isinstance(o, dict):
    for k, v in o.items():
        s = json.dumps(v, ensure_ascii=False)
        print(f"{k}: ({len(s)}) {s[:300]}")
elif isinstance(o, list):
    print(f"list len {len(o)}")
    for i, v in enumerate(o):
        s = json.dumps(v, ensure_ascii=False)
        print(f"[{i}] ({len(s)}) {s[:400]}")
else:
    print(o)
