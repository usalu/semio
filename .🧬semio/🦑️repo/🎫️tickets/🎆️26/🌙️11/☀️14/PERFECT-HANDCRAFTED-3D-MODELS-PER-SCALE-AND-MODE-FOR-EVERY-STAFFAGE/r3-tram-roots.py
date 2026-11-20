"""🌱️ Round 3 tram: root fixes of the catalog asset outside `volumes` (`python3 r3-tram-roots.py [--write]`).

Without `--write` the candidate document, with the `volumes` of the source, goes to `🗑️generated/r3-tram/🔣️.json` (check it with `r3-validate.ts --file` and `r3-2d.ts --file`); with `--write` the catalog file is re-read and only the changed members are rewritten in the file's own one-space style.
Steps: glazing layers innermost first; the true lengths of the two car body bones and the centre of mass of the middle module; the 2D drawings of every scale and view replaced by the reviewed shapes in `r3-tram-2d/s<N>.<view>.json`; `areaM2` corrections measured on the 1:50 model.
"""
import glob, json, os, re, sys

T = os.path.dirname(os.path.abspath(__file__))
CAT = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/tram/🔣️.json")[0]
OUT = os.path.join(T, "🗑️generated", "r3-tram", "🔣️.json")
SHAPES = os.path.join(T, "r3-tram-2d")
AREAS = {"body-panels": 105.7}
RIG = {"body-mid": {"lengthM": 6.0, "comX": 3.2}, "body-end": {"lengthM": 9.42}}


def glazing_layers(a):
    for r in a["thermal"]["regions"]:
        if r["id"] == "glazing":
            order = {"pane-in": 0, "gap": 1, "pane-out": 2}
            r["layers"].sort(key=lambda l: order[l["id"]])


def drawings(a):
    for path in sorted(glob.glob(os.path.join(SHAPES, "s*.*.json"))):
        scale, view = os.path.basename(path)[1:-5].split(".")
        a["representations"][scale]["geometry"][view] = json.load(open(path))


def rig(a):
    for b in a["rig"]["bones"]:
        for k, v in RIG.get(b["id"], {}).items():
            if k == "comX":
                b["mass"]["comX"] = v
            else:
                b[k] = v


def areas(a):
    for r in a["thermal"]["regions"]:
        if r["id"] in AREAS:
            r["areaM2"] = AREAS[r["id"]]


def block(key, value):
    t = json.dumps({key: value}, ensure_ascii=False, indent=1)
    return "\n".join(" " + line for line in t[2:-2].split("\n"))


def member_range(text, key):
    m = re.search(r'\n  "' + key + r'"\s*:\s*', text)
    i, depth, quoted = m.end(), 0, False
    while True:
        c = text[i]
        if quoted:
            if c == "\\":
                i += 1
            elif c == '"':
                quoted = False
        elif c == '"':
            quoted = True
        elif c in "{[":
            depth += 1
        elif c in "}]":
            depth -= 1
            if depth == 0:
                return m.start() + 1, i + 1
        i += 1


if __name__ == "__main__":
    text = open(CAT, encoding="utf-8").read()
    doc = json.loads(text)
    before = json.loads(text)["asset"]
    a = doc["asset"]
    glazing_layers(a)
    drawings(a)
    rig(a)
    areas(a)
    changed = [k for k in a if a[k] != before[k]]
    print("changed members:", changed)
    if "--write" not in sys.argv:
        os.makedirs(os.path.dirname(OUT), exist_ok=True)
        a["volumes"] = json.load(open(os.path.join(T, "volumes", "vehicles", "tram.json")))["volumes"]
        json.dump(doc, open(OUT, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print(OUT)
        sys.exit(0)
    fresh = open(CAT, encoding="utf-8").read()
    if fresh != text:
        sys.exit("catalog changed while editing; retry")
    for k in changed:
        s, e = member_range(fresh, k)
        fresh = fresh[:s] + block(k, a[k]) + fresh[e:]
    check = json.loads(fresh)
    assert check == doc and list(check["asset"]) == list(before), "round trip failed"
    open(CAT, "w", encoding="utf-8").write(fresh)
    print("written", CAT)
