"""Removes illegitimate `sectionFaces` (C25) from named solids, in the asset file and in the volumes source, by text surgery (checked by JSON comparison).
Usage: python3 areareview-unsection.py   (list in TARGETS)"""
import glob, json, os, re, tempfile, time, sys
T = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(T, *[".."] * 7))
CAT = os.path.join(ROOT, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
WS = re.compile(r"\s*")
# slug -> list of (scale, list, solid id)
TARGETS = {
    "city-car": [("50", "thermal", "th-bp-lower-paint")] + [(s, "thermal", "th-bp-lower") for s in ("100", "200", "500")],
    "delivery-van": [("50", "thermal", "th-shell-paint")] + [(s, "thermal", "th-shell") for s in ("100", "200", "500")],
    "city-bus": [("50", "thermal", "th-shell-insulation")] + [(s, "thermal", "th-shell") for s in ("100", "200", "500")],
    "height-adjustable-desk": [("50", "thermal", "th-top-board-laminate")] + [(s, "thermal", "th-top-board") for s in ("100", "200", "500")],
    "dining-table": [(s, "thermal", i) for s in ("50", "100", "200") for i in ("th-table-top-top-l", "th-table-top-top-r")] + [("500", "thermal", "th-table-top-slab-l"), ("500", "thermal", "th-table-top-slab-r")],
    "person-with-umbrella": [(s, l, i) for s in ("50", "100", "200", "500") for l, i in (("body", "canopy"), ("thermal", "th-canopy"))],
}

def skip_ws(t, i):
    while t[i] in " \t\r\n": i += 1
    return i

def parse(t, i):
    """returns (end, spans) where for objects spans = {key: (key_start, val_start, val_end, value_spans)}, arrays -> list"""
    i = skip_ws(t, i)
    c = t[i]
    if c == "{":
        i += 1; m = {}
        i = skip_ws(t, i)
        if t[i] == "}": return i + 1, m
        while True:
            i = skip_ws(t, i); ks = i
            ke = t.index('"', i + 1)
            while t[ke - 1] == "\\": ke = t.index('"', ke + 1)
            key = json.loads(t[ks:ke + 1])
            i = skip_ws(t, ke + 1) + 1
            vs = skip_ws(t, i)
            ve, sub = parse(t, vs)
            m[key] = (ks, vs, ve, sub)
            i = skip_ws(t, ve)
            if t[i] == ",": i += 1; continue
            return i + 1, m
    if c == "[":
        i += 1; l = []
        i = skip_ws(t, i)
        if t[i] == "]": return i + 1, l
        while True:
            vs = skip_ws(t, i)
            ve, sub = parse(t, vs)
            l.append((vs, vs, ve, sub))
            i = skip_ws(t, ve)
            if t[i] == ",": i += 1; continue
            return i + 1, l
    if c == '"':
        j = t.index('"', i + 1)
        while t[j - 1] == "\\": j = t.index('"', j + 1)
        return j + 1, None
    m = re.compile(r"-?[0-9.eE+\-]+|true|false|null").match(t, i)
    return m.end(), None

def idof(t, spans):
    k = spans.get("id")
    return json.loads(t[k[1]:k[2]]) if k else None

def remove(text, scale, lst, sid, top):
    end, root = parse(text, 0)
    base = root["asset"][3] if "asset" in root else root
    vol = (base["volumes"] if top == "asset" else root["volumes"])[3]
    arr = vol[scale][3][lst][3]
    hits = [s for s in arr if idof(text, s[3]) == sid and "sectionFaces" in s[3]]
    if not hits: return text, False
    assert len(hits) == 1, (scale, lst, sid, len(hits))
    ks, vs, ve, _ = hits[0][3]["sectionFaces"]
    # previous member end: walk back over whitespace and the comma
    a = ks
    while text[a - 1] in " \t\r\n": a -= 1
    assert text[a - 1] == ",", "sectionFaces first member"
    return text[: a - 1] + text[ve:], True

def strip(obj, scale, lst, sid):
    for s in obj[scale][lst]:
        if s.get("id") == sid: s.pop("sectionFaces", None)

def process(path, slug, top, dry=False):
    for _ in range(8):
        text = open(path, encoding="utf8").read()
        new, n = text, 0
        for scale, lst, sid in TARGETS[slug]:
            new, ok = remove(new, scale, lst, sid, top)
            n += ok
        if n == 0: return 0
        a, b = json.loads(text), json.loads(new)
        va = a["asset"]["volumes"] if top == "asset" else a["volumes"]
        for scale, lst, sid in TARGETS[slug]: strip(va, scale, lst, sid)
        assert a == b, f"{path}: more than sectionFaces changed"
        if dry: return n
        if open(path, encoding="utf8").read() != text: time.sleep(0.2); continue
        fd, tmp = tempfile.mkstemp(dir=os.path.dirname(path))
        with os.fdopen(fd, "w", encoding="utf8") as f: f.write(new)
        os.replace(tmp, path)
        return n
    raise RuntimeError(path)

if __name__ == "__main__":
    dry = "--dry" in sys.argv
    for slug in TARGETS:
        ap = glob.glob(f"{CAT}/*/{slug}/🔣️.json")[0]
        print(slug, "asset", process(ap, slug, "asset", dry))
        for sp in glob.glob(f"{T}/volumes/*/{slug}.json"):
            print(slug, "source", process(sp, slug, "volumes", dry))
