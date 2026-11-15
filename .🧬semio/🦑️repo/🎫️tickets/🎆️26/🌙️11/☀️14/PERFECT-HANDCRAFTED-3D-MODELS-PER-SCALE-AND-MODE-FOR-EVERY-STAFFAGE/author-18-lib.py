import os, math
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/"


def circle_pts(cx, cy, r, n=24, a0=0.0):
    return [[cx + r * math.cos(a0 + 2 * math.pi * k / n), cy + r * math.sin(a0 + 2 * math.pi * k / n)] for k in range(n)]


def arc_pts(cx, cy, r, a0, a1, n):
    return [[cx + r * math.cos(a0 + (a1 - a0) * k / n), cy + r * math.sin(a0 + (a1 - a0) * k / n)] for k in range(n + 1)]


def save_asset(slug, scales, category="components"):
    path = os.path.join(T, "volumes", category, slug + ".json")
    cur = json.load(open(path))
    v = cur["volumes"]
    asset = json.load(open(CATALOG + slug + "/🔣️.json"))["asset"]
    sup = asset["structure"]["supports"]
    for sc_ in scales:
        for d in sc_.structure:
            if d.get("part") == "support" and "anchor" not in d:
                m = [x for x in sup if x in d["id"]]
                d["anchor"] = m[0] if m else sup[0]
    for s in scales:
        v[str(s.den)] = s.out()
    json.dump(cur, open(path, "w"), ensure_ascii=False, indent=1)
    print("saved", path, {k: (len(x.get("body", [])), len(x.get("thermal", [])), len(x.get("structure", []))) for k, x in v.items() if x["status"] == "authored"})
