"""🔎️ Prints one section of a catalog asset compactly, one list item per line."""
import json, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
path = sys.argv[1] if sys.argv[1].endswith(".json") else S + sys.argv[1] + "/🔣️.json"
o = json.load(open(path))["asset"]
for k in (sys.argv[2].split(".") if len(sys.argv) > 2 and sys.argv[2] else []):
    o = o[int(k)] if isinstance(o, list) else o[k]
lim = int(sys.argv[3]) if len(sys.argv) > 3 else 6000
per = int(sys.argv[4]) if len(sys.argv) > 4 else 400
out = []
if isinstance(o, list):
    out = [json.dumps(i, ensure_ascii=False)[:per] for i in o]
elif isinstance(o, dict):
    out = [k + ": " + json.dumps(v, ensure_ascii=False)[:per] for k, v in o.items()]
else:
    out = [json.dumps(o, ensure_ascii=False)]
print("\n".join(out)[:lim])
