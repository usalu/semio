"""🔍 Print one section of a catalog asset document compactly: author-inspect.py <category/slug|path> <json.path> [depth]"""
import json, sys, pathlib
S = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
FOLD = {"furniture": "🛋️furniture", "appliances": "🔌️appliances", "objects": "🪑️objects"}
def load(ref):
    p = pathlib.Path(ref)
    if not p.exists():
        c, s = ref.split("/")
        p = S / FOLD.get(c, c) / s / "🔣️.json"
    return json.loads(p.read_text())
def walk(d, path):
    for k in [k for k in path.split(".") if k]:
        d = d[int(k)] if isinstance(d, list) else d[k]
    return d
def show(d, depth, ind=0):
    pad = "  " * ind
    if depth <= 0 or not isinstance(d, (dict, list)):
        s = json.dumps(d, ensure_ascii=False)
        print(pad + (s if len(s) < 400 else s[:400] + f"... ({len(s)} chars)"))
        return
    items = d.items() if isinstance(d, dict) else enumerate(d)
    for k, v in items:
        if isinstance(v, (dict, list)) and depth > 1:
            print(f"{pad}{k}: {'{' if isinstance(v, dict) else '['}{len(v)}")
            show(v, depth - 1, ind + 1)
        else:
            s = json.dumps(v, ensure_ascii=False)
            print(f"{pad}{k}: " + (s if len(s) < 400 else s[:400] + f"... ({len(s)} chars)"))
if __name__ == "__main__":
    doc = load(sys.argv[1])
    show(walk(doc, sys.argv[2] if len(sys.argv) > 2 else ""), int(sys.argv[3]) if len(sys.argv) > 3 else 1)
