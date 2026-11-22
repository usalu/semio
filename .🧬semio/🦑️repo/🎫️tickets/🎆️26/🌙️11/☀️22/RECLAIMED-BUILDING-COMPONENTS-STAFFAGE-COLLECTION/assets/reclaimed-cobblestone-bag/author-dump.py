"""🔎️ Prints one section of a catalog asset with long strings shortened: `author-dump.py <category/slug> <dotted.path> [max string length]`."""
import json, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
N = int(sys.argv[3]) if len(sys.argv) > 3 else 90
def cut(o):
    if isinstance(o, str): return o if len(o) <= N else o[:N] + "…"
    if isinstance(o, list): return [cut(x) for x in o]
    if isinstance(o, dict):
        if set(o.keys()) == {"en", "de"}: return cut(o["en"])
        return {k: cut(v) for k, v in o.items()}
    return o
o = json.load(open(sys.argv[1] if sys.argv[1].endswith(".json") else S + sys.argv[1] + "/🔣️.json"))["asset"]
for k in sys.argv[2].split("."):
    if k == "": continue
    o = o[int(k)] if isinstance(o, list) else o[k]
print(json.dumps(cut(o), ensure_ascii=False))
