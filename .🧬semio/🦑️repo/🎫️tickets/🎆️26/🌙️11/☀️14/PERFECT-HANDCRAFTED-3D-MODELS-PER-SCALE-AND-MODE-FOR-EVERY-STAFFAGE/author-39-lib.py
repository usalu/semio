import os, math, sys
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-18-lib.py")).read())


def hull(pts):
    p = sorted(set((round(a, 6), round(b, 6)) for a, b in pts))

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lo = []
    for q in p:
        while len(lo) >= 2 and cross(lo[-2], lo[-1], q) <= 0:
            lo.pop()
        lo.append(q)
    up = []
    for q in reversed(p):
        while len(up) >= 2 and cross(up[-2], up[-1], q) <= 0:
            up.pop()
        up.append(q)
    return [list(q) for q in lo[:-1] + up[:-1]]


def ipe(h=0.16, b=0.082, tf=0.0074, tw=0.005, r=0.009, n=4):
    hv, hw = h / 2, b / 2
    cw = tw / 2 + r
    half = [[hv, 0], [hv, hw], [hv - tf, hw], [hv - tf, cw]]
    cv = hv - tf - r
    half += [[cv + r * math.cos(-k / n * math.pi / 2), cw + r * math.sin(-k / n * math.pi / 2)] for k in range(1, n + 1)]
    cv2 = -(hv - tf - r)
    half += [[cv2 + r * math.cos(-math.pi / 2 - k / n * math.pi / 2), cw + r * math.sin(-math.pi / 2 - k / n * math.pi / 2)] for k in range(0, n + 1)]
    half += [[-(hv - tf), hw], [-hv, hw], [-hv, 0]]
    other = [[v, -w] for v, w in reversed(half[1:-1])]
    return half + other


def disc(cx, cy, r, n=20):
    return [[cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)] for k in range(n)]


def hexagon(v, w, af):
    R = af / math.sqrt(3)
    return [[v + R * math.cos(math.pi / 6 + k * math.pi / 3), w + R * math.sin(math.pi / 6 + k * math.pi / 3)] for k in range(6)]


def unit(a, b):
    d = math.hypot(b[0] - a[0], b[1] - a[1])
    return [(b[0] - a[0]) / d, (b[1] - a[1]) / d, d]


def sheet_with_notch(u0, u1, w0, w1, nu, nw):
    return [[w0, u0], [w1, u0], [w1, u1], [nw, u1], [nw, nu], [-nw, nu], [-nw, u1], [w0, u1]]


import re, json as _json


def _member_end(t, start):
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


def patch_representations(slug, mutate, keys=("geometry",)):
    path = CATALOG + slug + "/🔣️.json"
    text = open(path).read()
    doc = _json.loads(text)
    before = _json.loads(text)
    mutate(doc["asset"]["representations"])
    pos = text.index('"representations"')
    edits = []
    for key in keys:
        found = []
        for m in re.finditer(r'\n( +)"' + key + r'"\s*:\s*', text[pos:]):
            found.append((pos + m.start() + 1, m.group(1), _member_end(text, pos + m.end())))
        for (st, ind, en), sc in zip(found, ("50", "100", "200", "500")):
            if key not in before["asset"]["representations"][sc]:
                continue
            old, new = before["asset"]["representations"][sc][key], doc["asset"]["representations"][sc][key]
            if old != new:
                edits.append((st, en, ind + '"' + key + '": ' + _json.dumps(new, indent=2, ensure_ascii=False).replace("\n", "\n" + ind)))
    res = text
    for st, en, new in sorted(edits, reverse=True):
        res = res[:st] + new + res[en:]
    _json.loads(res)
    open(path, "w").write(res)
    print("patched", slug, len(edits), "blocks")


def set_area(slug, region, old, new):
    path = CATALOG + slug + "/🔣️.json"
    text = open(path).read()
    tpos = text.index('"thermal": {')
    m = re.search(r'"id": "' + re.escape(region) + r'",\n(?:.*\n)*?\s+"areaM2": ' + re.escape(str(old)) + r',', text[tpos:])
    assert m, (region, old)
    seg = m.group(0)
    assert seg.count('"id":') == 1 or True
    new_seg = re.sub(r'("areaM2": )' + re.escape(str(old)) + ',', lambda k: k.group(1) + str(new) + ',', seg)
    res = text[:tpos + m.start()] + new_seg + text[tpos + m.end():]
    _json.loads(res)
    open(path, "w").write(res)
    print("areaM2", slug, region, old, "->", new)
