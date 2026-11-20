"""💾 Writes the root fixes of the round-3 wheel loader into the catalog: re-reads the catalog asset right before writing, replaces only the members that differ from the candidate document (never `volumes`, which volumes-insert.py splices), keeps member order and the file's one-space indent, and updates heightM, widthM and distinction of the asset's own manifest entry."""
import json, os, re, sys

T = os.path.dirname(os.path.abspath(__file__))
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
CATALOG = S + "/🏗️machinery/wheel-loader/🔣️.json"
MANIFEST = S + "/🔣️.json"
cand = json.load(open(os.path.join(T, "🗑️generated", "r3-wheel-loader", "🔣️.json")))["asset"]


def span(text, key, depth):
    m = re.search(r'\n' + " " * depth + json.dumps(key) + ": ", text)
    i = m.end()
    d, j, q = 0, i, False
    while True:
        c = text[j]
        if q:
            if c == "\\":
                j += 1
            elif c == '"':
                q = False
                if d == 0:
                    return i, j + 1
        elif c == '"':
            q = True
        elif c in "{[":
            d += 1
        elif c in "}]":
            d -= 1
            if d == 0:
                return i, j + 1
        elif d == 0 and c in ",\n":
            return i, j
        j += 1


def dump(value, depth):
    return json.dumps(value, ensure_ascii=False, indent=1).replace("\n", "\n" + " " * depth)


text = open(CATALOG).read()
live = json.loads(text)["asset"]
assert list(live.keys()) == list(cand.keys()), "member order changed"
changed = []
for key in live:
    if key == "volumes" or live[key] == cand[key]:
        continue
    i, j = span(text, key, 2)
    assert json.loads(text[i:j]) == live[key], key
    text = text[:i] + dump(cand[key], 2) + text[j:]
    changed.append(key)
check = json.loads(text)["asset"]
assert all(check[k] == cand[k] for k in check if k != "volumes")
if "--dry" not in sys.argv:
    open(CATALOG, "w").write(text)
print("asset members written:", changed)
mt = open(MANIFEST).read()
man = json.loads(mt)
entry = next(e for e in man["entries"] if e["id"] == "wheel-loader")
i = mt.index('"id": "wheel-loader"')
j = mt.index('"id":', i + 10)
block = mt[i:j]
new = block
for key, val in (("heightM", cand["dimensions"]["heightM"]), ("widthM", cand["dimensions"]["widthM"]), ("distinction", cand["summary"])):
    if entry.get(key) != val:
        a, b = span(new, key, len(re.search(r'\n( *)"heightM"', new).group(1)))
        depth = len(re.search(r'\n( *)"heightM"', new).group(1))
        style = json.dumps(val, ensure_ascii=False, indent=2).replace("\n", "\n" + " " * depth)
        new = new[:a] + style + new[b:]
if new != block and "--dry" not in sys.argv:
    out = mt[:i] + new + mt[j:]
    json.loads(out)
    open(MANIFEST, "w").write(out)
print("manifest entry", "updated" if new != block else "unchanged")
