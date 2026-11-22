"""🔎 Summarise a catalog asset document section by section without dumping it."""
import json, sys
p = sys.argv[1]
d = json.load(open(p))
a = d["asset"]
path = sys.argv[2:] if len(sys.argv) > 2 else []
x = a
for k in path:
    x = x[int(k)] if isinstance(x, list) else x[k]
def short(v, n=600):
    s = json.dumps(v, ensure_ascii=False)
    return s if len(s) <= n else s[:n] + f"... ({len(s)} chars)"
if isinstance(x, dict):
    for k, v in x.items():
        print(f"{k}: {short(v)}")
elif isinstance(x, list):
    print(f"list of {len(x)}")
    for i, v in enumerate(x[:60]):
        print(i, short(v, 400))
else:
    print(short(x, 5000))
