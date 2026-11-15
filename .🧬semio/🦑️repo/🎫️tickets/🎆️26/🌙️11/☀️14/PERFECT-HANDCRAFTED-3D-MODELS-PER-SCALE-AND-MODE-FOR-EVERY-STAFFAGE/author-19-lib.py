import os, math
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/"


def asset_of(slug):
    return json.load(open(CATALOG + slug + "/🔣️.json"))["asset"]


def save_asset(slug, scales, category="objects", unavailable=()):
    path = os.path.join(T, "volumes", category, slug + ".json")
    cur = json.load(open(path))
    v = cur["volumes"]
    sup = asset_of(slug)["structure"]["supports"]
    for sc_ in scales:
        for d in sc_.structure:
            if d.get("part") == "support" and "anchor" not in d:
                m = [x for x in sup if x in d["id"]]
                d["anchor"] = m[0] if m else sup[0]
    for s in scales:
        v[str(s.den)] = s.out()
    json.dump(cur, open(path, "w"), ensure_ascii=False, indent=1)
    print("saved", path, {k: (len(x.get("body", [])), len(x.get("thermal", [])), len(x.get("structure", []))) for k, x in v.items() if x["status"] == "authored"})


def circle_pts(cx, cy, r, n=24, a0=0.0):
    return [[cx + r * math.cos(a0 + 2 * math.pi * k / n), cy + r * math.sin(a0 + 2 * math.pi * k / n)] for k in range(n)]


def arc_pts(cx, cy, r, a0, a1, n):
    return [[cx + r * math.cos(a0 + (a1 - a0) * k / n), cy + r * math.sin(a0 + (a1 - a0) * k / n)] for k in range(n + 1)]


QTOP = [0.70711, 0, 0.70711, 0]


def qz(phi):
    return qmul([0, 0, math.sin(phi / 2), math.cos(phi / 2)], QTOP)


def hoop_sections(k, rx, ry, X=0.35082, yb=0.59585, R=0.18, mid=True):
    s = [sec([-X, 0, 0], rx, ry, rotation=qz(math.pi / 2))]
    for sign in (-1, 1):
        cx = sign * (X - R)
        pts = []
        for i in range(k + 1):
            a = (math.pi - (math.pi / 2) * i / k) if sign < 0 else (math.pi / 2 - (math.pi / 2) * i / k)
            pts.append(sec([cx + R * math.cos(a), yb + R * math.sin(a), 0], rx, ry, rotation=qz(a - math.pi / 2)))
        s += pts
        if sign < 0 and mid:
            s.append(sec([0, yb + R, 0], rx, ry, rotation=qz(0)))
    s.append(sec([X, 0, 0], rx, ry, rotation=qz(-math.pi / 2)))
    return s
