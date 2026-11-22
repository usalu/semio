"""🔍 Print one JSON path of a staffage document, truncated."""
import json, sys
d = json.load(open(sys.argv[1]))
for k in sys.argv[2].split('.') if sys.argv[2] else []:
    d = d[int(k)] if isinstance(d, list) else d[k]
n = int(sys.argv[3]) if len(sys.argv) > 3 else 4000
if isinstance(d, dict) and len(sys.argv) > 4:
    print({k: (len(json.dumps(v))) for k, v in d.items()})
else:
    print(json.dumps(d, ensure_ascii=False)[:n])
