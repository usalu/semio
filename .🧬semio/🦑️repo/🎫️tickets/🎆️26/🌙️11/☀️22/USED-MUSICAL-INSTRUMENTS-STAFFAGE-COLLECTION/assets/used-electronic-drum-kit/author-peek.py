"""🔍 Print a section of a catalog asset document compactly."""
import json, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
d = json.load(open(S + sys.argv[1] + "/🔣️.json"))
o = d
for k in sys.argv[2].split("."):
    o = o[int(k)] if isinstance(o, list) else o[k]
lim = int(sys.argv[3]) if len(sys.argv) > 3 else 6000
s = json.dumps(o, ensure_ascii=False, indent=None if len(sys.argv) > 4 else 1)
print(s[:lim])
