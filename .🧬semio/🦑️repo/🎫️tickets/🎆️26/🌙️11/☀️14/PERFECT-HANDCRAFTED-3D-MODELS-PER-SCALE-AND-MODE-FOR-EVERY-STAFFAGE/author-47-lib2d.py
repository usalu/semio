"""Author 47: text-level editing of one 2D shape of a catalog asset (keeps the file's custom formatting)."""
import json, re, sys

ASSETS = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"

def path(cat, slug):
    return ASSETS + {"vegetation": "🌳️vegetation"}[cat] + "/" + slug + "/🔣️.json"

def shape_range(text, scale, view, sid):
    """(start, end) of the object text of shape `sid` in representations[scale].geometry[view]."""
    i = text.index('"representations"')
    i = text.index('"%s": {' % scale, i)
    g = text.index('"geometry"', i)
    v = text.index('"%s": [' % view, g)
    k = text.index('"id": "%s"' % sid, v)
    s = text.rfind("{", v, k)
    depth = 0
    for j in range(s, len(text)):
        c = text[j]
        if c == "{": depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0: return s, j + 1
    raise RuntimeError("unbalanced")

def get_shape(text, scale, view, sid):
    a, b = shape_range(text, scale, view, sid)
    return json.loads(text[a:b]), a, b

def put_shape(text, scale, view, sid, shape):
    a, b = shape_range(text, scale, view, sid)
    old = text[a:b]
    compact = "\n" not in old
    new = json.dumps(shape, ensure_ascii=False, separators=(", ", ": ")) if compact else json.dumps(shape, ensure_ascii=False, indent=2)
    return text[:a] + new + text[b:]
