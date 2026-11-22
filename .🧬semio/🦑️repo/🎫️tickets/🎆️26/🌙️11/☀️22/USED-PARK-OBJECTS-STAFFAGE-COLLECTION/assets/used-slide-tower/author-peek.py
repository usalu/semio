"""🔍 Peek into a catalog asset section by section: author-peek.py <path> <dotted.key> [maxchars]"""
import json, sys
d = json.load(open(sys.argv[1]))
o = d
for k in sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []:
    o = o[int(k)] if isinstance(o, list) else o[k]
n = int(sys.argv[3]) if len(sys.argv) > 3 else 3000
if isinstance(o, dict):
    print("KEYS:", list(o.keys()))
s = json.dumps(o, ensure_ascii=False)
print(len(s)); print(s[:n])
