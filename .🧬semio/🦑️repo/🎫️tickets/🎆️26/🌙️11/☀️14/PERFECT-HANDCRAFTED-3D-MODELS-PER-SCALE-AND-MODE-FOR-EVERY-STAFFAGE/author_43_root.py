"""Author 43: text-level root edits of the 2D shapes of a catalog asset (the asset file is not reformatted)."""
import json, re

CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/"


def path_of(slug):
    return CAT + slug + "/🔣️.json"


def load(slug):
    return open(path_of(slug)).read()


def save(slug, txt):
    json.loads(txt)
    open(path_of(slug), "w").write(txt)


def view_span(txt, scale, view):
    """(start, end) character range of the list `view` of representation `scale`."""
    i0 = re.compile(r'\n   "%s": \{' % scale).search(txt, txt.index('\n  "representations"')).start()
    g = txt.index('"geometry"', i0)
    v = txt.index('"%s": [' % view, g)
    depth = 0
    j = txt.index("[", v)
    k = j
    while True:
        c = txt[k]
        if c == "[":
            depth += 1
        elif c == "]":
            depth -= 1
            if depth == 0:
                return j, k + 1
        k += 1


def add_shapes(slug, scale, view, shapes, after_id=None):
    """Append (or insert after the shape `after_id`) shapes to one view list; a shape whose id already exists in that list is skipped."""
    txt = load(slug)
    a, b = view_span(txt, scale, view)
    block = txt[a:b]
    arr = json.loads(block)
    new = [s for s in shapes if all(x["id"] != s["id"] for x in arr)]
    if not new:
        return 0
    if after_id is None:
        pos = block.rindex("]")
        ins = "".join(",\n" + json.dumps(s, ensure_ascii=False) for s in new)
        block = block[:pos].rstrip() + ins + "\n" + block[pos:]
    else:
        m = re.search(r'\{[^{}\[\]]*"id": "%s"[^{}\[\]]*(?:\{[^{}]*\}[^{}\[\]]*)?\}' % re.escape(after_id), block)
        assert m, after_id
        ins = "".join(",\n" + json.dumps(s, ensure_ascii=False) for s in new)
        block = block[: m.end()] + ins + block[m.end():]
    json.loads(block)
    save(slug, txt[:a] + block + txt[b:])
    return len(new)


def capsule_shape(id, bone, x0, y0, x1, y1, r, z=3.05, tone=0.3, region="frame-tubes", material="paint", role="body"):
    return {"kind": "capsule", "x0": x0, "y0": y0, "x1": x1, "y1": y1, "r0": r, "r1": r, "id": id, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone, "region": region, "material": material}


def set_area(slug, region, value):
    """Sets areaM2 of one thermal region (text-level); returns the old value."""
    txt = load(slug)
    t0 = txt.index('"thermal": {')
    i = txt.index('"id": "%s"' % region, t0)
    j = txt.index('"areaM2":', i)
    m = re.match(r'"areaM2": ([-\d.eE+]+)', txt[j:])
    old = float(m.group(1))
    txt = txt[:j] + '"areaM2": %s' % repr(float("%.3g" % value)) + txt[j + m.end():]
    save(slug, txt)
    return old


def add_review(rows):
    p = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/area-review.md"
    cur = open(p).read()
    out = cur.rstrip("\n") + "\n" + "".join(r + "\n" for r in rows)
    open(p, "w").write(out)
