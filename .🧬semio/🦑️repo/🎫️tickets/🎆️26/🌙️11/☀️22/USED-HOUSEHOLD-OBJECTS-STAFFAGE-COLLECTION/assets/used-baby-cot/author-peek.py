"""🔍 Print one section of a catalog asset document, truncated, for studying siblings."""
import json, sys
p, path, n = sys.argv[1], sys.argv[2], int(sys.argv[3]) if len(sys.argv) > 3 else 3000
d = json.load(open(p))
o = d
for k in [x for x in path.split('/') if x]:
    o = o[int(k)] if isinstance(o, list) else o[k]
if isinstance(o, dict):
    print('KEYS', list(o.keys()))
s = json.dumps(o, ensure_ascii=False)
print(len(s)); print(s[:n])
