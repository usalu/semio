import json, glob, os
GEN = "/Users/ueli/Documents/semio/.tmp-ticket/wp-r3/generated"
FX = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧫️fixtures"
def diffs(a, b, path=""):
    if type(a) != type(b) and not (isinstance(a, (int, float)) and isinstance(b, (int, float))):
        yield path, a, b; return
    if isinstance(a, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b: yield path + "." + k, a.get(k, "<absent>"), b.get(k, "<absent>")
            else: yield from diffs(a[k], b[k], path + "." + k)
    elif isinstance(a, list):
        if len(a) != len(b): yield path, f"len {len(a)}", f"len {len(b)}"; return
        for i, (x, y) in enumerate(zip(a, b)): yield from diffs(x, y, f"{path}[{i}]")
    elif a != b: yield path, a, b
for dump in sorted(glob.glob(os.path.join(GEN, "debug-*.json"))):
    text = open(dump, encoding="utf-8").read().split("\n", 1)
    before_rust = json.loads(text[0][len("[DEBUG] "):]); after_rust = json.loads(text[1])
    uri = os.path.basename(dump)[len("debug-shared:__"):-len(".json")].replace("_", "/")
    after_path = os.path.join(FX, uri)
    before_path = after_path.replace("➡️after", "⬅️before")
    rows = list(diffs(json.load(open(before_path)), before_rust)) + list(diffs(json.load(open(after_path)), after_rust))
    print(("OK  " if not rows else "DIFF"), uri, len(rows))
    for row in rows[:5]: print("     ", row[0], json.dumps(row[1])[:120], "->", json.dumps(row[2])[:120])
