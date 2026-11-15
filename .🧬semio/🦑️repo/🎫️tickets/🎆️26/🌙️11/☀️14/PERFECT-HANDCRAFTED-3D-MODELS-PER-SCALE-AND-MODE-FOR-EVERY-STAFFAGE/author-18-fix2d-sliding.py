import json, re
PATH = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/sliding-door/🔣️.json"
text = open(PATH).read()
doc = json.loads(text)
a = doc["asset"]


def member_end(t, start):
    depth, i, q = 0, start, False
    while i < len(t):
        c = t[i]
        if q:
            if c == "\\":
                i += 1
            elif c == '"':
                q = False
        elif c == '"':
            q = True
        elif c in "{[":
            depth += 1
        elif c in "}]":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    raise SystemExit("unbalanced")


def ranges(key, n):
    pos = text.index('"representations"')
    out = []
    for m in re.finditer(r'\n( +)"' + key + r'"\s*:\s*', text[pos:]):
        out.append((pos + m.start() + 1, m.group(1), member_end(text, pos + m.end())))
        if len(out) == n:
            break
    return out


enc = lambda ind, key, v: ind + '"' + key + '": ' + json.dumps(v, indent=2, ensure_ascii=False).replace("\n", "\n" + ind)
orig = json.loads(text)
geo, sec = ranges("geometry", 4), ranges("sections", 4)
for (st, ind, en), sc in zip(geo, ("50", "100", "200", "500")):
    assert enc(ind, "geometry", orig["asset"]["representations"][sc]["geometry"]) == text[st:en], "geometry roundtrip " + sc
for (st, ind, en), sc in zip(sec, ("50", "100", "200", "500")):
    assert enc(ind, "sections", orig["asset"]["representations"][sc]["sections"]) == text[st:en], "sections roundtrip " + sc
for sc in ("50", "100", "200"):
    for s in a["representations"][sc]["geometry"]["plan"]:
        if s["id"] == "track-above" or (sc == "50" and s["id"].startswith("hanger-above")):
            s["role"] = "texture"
            if sc == "200":
                s["penMm"] = 0.1
a["representations"]["500"]["sections"] = {}
res = text
edits = [(st, en, enc(ind, "geometry", a["representations"][sc]["geometry"])) for (st, ind, en), sc in zip(geo, ("50", "100", "200", "500"))]
edits += [(st, en, enc(ind, "sections", a["representations"][sc]["sections"])) for (st, ind, en), sc in zip(sec, ("50", "100", "200", "500"))]
for st, en, new in sorted(edits, reverse=True):
    res = res[:st] + new + res[en:]
json.loads(res)
open(PATH, "w").write(res)
print("patched")
