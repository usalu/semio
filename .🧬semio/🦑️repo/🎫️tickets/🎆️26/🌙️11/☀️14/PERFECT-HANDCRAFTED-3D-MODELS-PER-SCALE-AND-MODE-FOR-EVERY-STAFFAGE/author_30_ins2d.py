import json, re, glob


def file_of(cat, slug):
    return glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*{cat}/{slug}/🔣️.json")[0]


def insert_after(cat, slug, scale, view, after_id, shape):
    F = file_of(cat, slug)
    text = open(F, encoding="utf-8").read()
    rep0 = text.index('"representations"')
    sc_pos = text.index(f'"denominator": {scale}', rep0)
    v = text.index(f'"{view}"', sc_pos)
    nxt = min(x for x in (text.find('"front"', v + 1) if view == "elevation" else -1, text.find('"plan"', v + 1) if view in ("elevation", "front") else -1, len(text)) if x > 0)
    nd = text.find('"denominator"', v + 1)
    seg_end = nxt if view != "plan" else (nd if nd > 0 else len(text))
    block = text[v:seg_end]
    if f'"id": "{shape["id"]}"' in block:
        return False
    i = text.index(f'"id": "{after_id}"', v)
    assert i < seg_end, "after-id not in view"
    start = text.rfind("{", 0, i)
    # the object start may be before "id" with kind first; find enclosing object start at depth 0
    depth, q = 0, i
    while q > v:
        if text[q] == "}":
            depth += 1
        elif text[q] == "{":
            if depth == 0:
                break
            depth -= 1
        q -= 1
    start = q
    start_line = text.rfind("\n", 0, start) + 1
    obj_indent = re.match(r" *", text[start_line:]).group(0)
    depth, q = 0, start
    while True:
        ch = text[q]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                break
        q += 1
    block = json.dumps(shape, indent=2, ensure_ascii=False)
    block = "\n".join(obj_indent + line if k else line for k, line in enumerate(block.split("\n")))
    text = text[: q + 1] + ",\n" + obj_indent + block + text[q + 1 :]
    json.loads(text)
    open(F, "w", encoding="utf-8").write(text)
    return True


def shape_of(cat, slug, scale, view, sid):
    a = json.load(open(file_of(cat, slug), encoding="utf-8"))["asset"]
    return next(s for s in a["representations"][str(scale)]["geometry"][view] if s["id"] == sid)


def replace_points(cat, slug, scale, view, sid, points):
    F = file_of(cat, slug)
    text = open(F, encoding="utf-8").read()
    rep0 = text.index('"representations"')
    sc_pos = text.index(f'"denominator": {scale}', rep0)
    v = text.index(f'"{view}"', sc_pos)
    i = text.index(f'"id": "{sid}"', v)
    depth, q = 0, i
    while q > v:
        if text[q] == "}":
            depth += 1
        elif text[q] == "{":
            if depth == 0:
                break
            depth -= 1
        q -= 1
    start = q
    depth, e = 0, start
    while True:
        ch = text[e]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                break
        e += 1
    obj = text[start : e + 1]
    j = obj.index('"points":')
    k = j + len('"points":')
    while obj[k] == " ":
        k += 1
    depth, m = 0, k
    while True:
        if obj[m] == "[":
            depth += 1
        elif obj[m] == "]":
            depth -= 1
            if depth == 0:
                break
        m += 1
    ls = text.rfind("\n", 0, start + j) + 1
    ind = re.match(r" *", text[ls:]).group(0)
    new = "[\n" + ",\n".join(ind + "  " + json.dumps(p) for p in points) + "\n" + ind + "]"
    obj2 = obj[:k] + new + obj[m + 1 :]
    text = text[:start] + obj2 + text[e + 1 :]
    json.loads(text)
    open(F, "w", encoding="utf-8").write(text)


def _obj_range(text, v, i):
    depth, q = 0, i
    while q > v:
        if text[q] == "}":
            depth += 1
        elif text[q] == "{":
            if depth == 0:
                break
            depth -= 1
        q -= 1
    start = q
    depth, e = 0, start
    while True:
        ch = text[e]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                break
        e += 1
    return start, e


def rewrite_shape(cat, slug, scale, view, sid, fn, remove=False):
    F = file_of(cat, slug)
    text = open(F, encoding="utf-8").read()
    rep0 = text.index('"representations"')
    sc_pos = text.index(f'"denominator": {scale}', rep0)
    v = text.index(f'"{view}"', sc_pos)
    i = text.index(f'"id": "{sid}"', v)
    start, e = _obj_range(text, v, i)
    ls = text.rfind("\n", 0, start) + 1
    ind = re.match(r" *", text[ls:]).group(0)
    obj = json.loads(text[start : e + 1])
    if remove:
        # remove together with a neighbouring comma
        before = text[:start].rstrip()
        after = text[e + 1 :]
        if after.lstrip().startswith(","):
            cut = e + 1 + (len(after) - len(after.lstrip())) + 1
            text = text[:ls] + text[cut:].lstrip("\n").lstrip(" ") if False else text[:start] + text[cut:].lstrip()
        else:
            text = before.rstrip(",") + text[e + 1 :]
    else:
        obj = fn(obj) or obj
        block = json.dumps(obj, indent=2, ensure_ascii=False)
        block = "\n".join(ind + line if k else line for k, line in enumerate(block.split("\n")))
        text = text[:start] + block + text[e + 1 :]
    json.loads(text)
    open(F, "w", encoding="utf-8").write(text)
