"""Root fixes of 2D shapes in an asset file: replaces the text span of one shape (found by scale, view and id) with the mutated, re-serialised object; every other byte stays."""
import json
import re

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles"


def path_of(slug):
    return f"{CATALOG}/{slug}/🔣️.json"


def span_at(text, anchor):
    depth, i = 0, anchor
    while i >= 0:
        c = text[i]
        if c == "}":
            depth += 1
        elif c == "{":
            if depth == 0:
                break
            depth -= 1
        i -= 1
    start = i
    depth, quoted, j = 0, False, start
    while j < len(text):
        c = text[j]
        if quoted:
            if c == "\\":
                j += 1
            elif c == '"':
                quoted = False
        elif c == '"':
            quoted = True
        elif c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return start, j + 1
        j += 1
    raise SystemExit("unbalanced")


def patch(slug, edits):
    """edits: list of (scale:str, view:str, id:str, fn(shape)->None). Re-reads the file, applies all, writes once."""
    path = path_of(slug)
    text = open(path, encoding="utf-8").read()
    data = json.loads(text)
    reps = data["asset"]["representations"]
    wanted = {}
    for scale, view, sid, fn in edits:
        wanted.setdefault(sid, []).append((scale, view, fn))
    spans = []
    for sid, items in wanted.items():
        order = []
        for sc in ("50", "100", "200", "500"):
            rep = reps.get(sc)
            if not rep or rep.get("status") != "authored":
                continue
            for v in ("elevation", "front", "plan"):
                for sh in rep["geometry"].get(v, []):
                    if sh.get("id") == sid:
                        order.append((sc, v))
        r0 = text.index('\n  "representations"')
        r1 = text.index('\n  "symbols"')
        hits = [m.start() for m in re.finditer(r'"id":\s*' + re.escape(json.dumps(sid, ensure_ascii=False)), text) if r0 < m.start() < r1]
        assert len(hits) == len(order), (sid, len(hits), len(order))
        for (sc, v), pos in zip(order, hits):
            for scale, view, fn in items:
                if scale == sc and view == v:
                    a, b = span_at(text, pos)
                    obj = json.loads(text[a:b])
                    fn(obj)
                    line_start = text.rfind("\n", 0, a) + 1
                    indent = text[line_start:a]
                    pad = " " * len(indent)
                    dumped = json.dumps(obj, indent=1, ensure_ascii=False).replace("\n", "\n" + pad)
                    spans.append((a, b, dumped))
    for a, b, d in sorted(spans, reverse=True):
        text = text[:a] + d + text[b:]
    open(path, "w", encoding="utf-8").write(text)
    json.loads(text)
    return len(spans)


def add_shape(slug, scale, view, after_id, shape):
    """Inserts `shape` (a dict) after the shape `after_id` of the view of the scale, in the file's own list formatting."""
    path = path_of(slug)
    text = open(path, encoding="utf-8").read()
    data = json.loads(text)
    reps = data["asset"]["representations"]
    order = []
    for sc in ("50", "100", "200", "500"):
        rep = reps.get(sc)
        if not rep or rep.get("status") != "authored":
            continue
        for v in ("elevation", "front", "plan"):
            for sh in rep["geometry"].get(v, []):
                if sh.get("id") == after_id:
                    order.append((sc, v))
    r0 = text.index('\n  "representations"')
    r1 = text.index('\n  "symbols"')
    hits = [m.start() for m in re.finditer(r'"id":\s*' + re.escape(json.dumps(after_id, ensure_ascii=False)), text) if r0 < m.start() < r1]
    assert len(hits) == len(order), (after_id, len(hits), len(order))
    pos = hits[order.index((scale, view))]
    a, b = span_at(text, pos)
    line_start = text.rfind("\n", 0, a) + 1
    pad = " " * len(text[line_start:a])
    dumped = json.dumps(shape, indent=1, ensure_ascii=False).replace("\n", "\n" + pad)
    text = text[:b] + ",\n" + pad + dumped + text[b:]
    open(path, "w", encoding="utf-8").write(text)
    json.loads(text)
