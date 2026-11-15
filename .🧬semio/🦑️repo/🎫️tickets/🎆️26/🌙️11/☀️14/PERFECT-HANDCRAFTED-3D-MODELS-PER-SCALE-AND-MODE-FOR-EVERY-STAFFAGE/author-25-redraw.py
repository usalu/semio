"""Usage: python3 author-25-redraw.py <slug> --scale 500 --from 200 [--view elevation] [--tol 0.02] [--only id,id] [--apply]
Repairs 2D body shapes of a coarse scale whose outline does not belong to its bone (bone-local bounding box more than 4 cm away from every shape of the same bone at the finer scale, or an open stroke): replaces the outline by the finer scale's outline of that bone, Douglas-Peucker simplified to --tol metres (a generalisation, not a copy), closed and filled. Text patch of the shape objects only; prints what it changes."""
import glob, json, math, re, sys

slug = sys.argv[1]
arg = lambda k, d=None: sys.argv[sys.argv.index(k) + 1] if k in sys.argv else d
scale, src, view, tol = arg("--scale"), arg("--from"), arg("--view", "elevation"), float(arg("--tol", "0.02"))
only = set(arg("--only", "").split(",")) - {""}
path = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/%s/🔣️.json" % slug)[0]
text = open(path, encoding="utf-8").read()
asset = json.loads(text)["asset"]
dec = json.JSONDecoder()


def poly_of(s):
    if s.get("kind") == "poly" and s.get("closed", True) is not False and len(s["points"]) >= 3:
        return [tuple(p) for p in s["points"]]
    if s.get("kind") == "path":
        toks = re.findall(r"[MCLZ]|-?[0-9.]+(?:e-?[0-9]+)?", s["d"])
        pts, i, cur = [], 0, None
        while i < len(toks):
            t = toks[i]
            if t == "M" or t == "L":
                cur = (float(toks[i + 1]), float(toks[i + 2]))
                pts.append(cur)
                i += 3
            elif t == "C":
                p0 = cur
                c1, c2, p1 = [(float(toks[i + 1 + 2 * k]), float(toks[i + 2 + 2 * k])) for k in range(3)]
                for k in range(1, 7):
                    u = k / 6
                    pts.append(tuple((1 - u) ** 3 * p0[j] + 3 * (1 - u) ** 2 * u * c1[j] + 3 * (1 - u) * u * u * c2[j] + u**3 * p1[j] for j in range(2)))
                cur = p1
                i += 7
            else:
                i += 1
        return pts
    return None


def dp(pts0, eps):
    n = len(pts0)
    i0 = max(range(n), key=lambda i: pts0[i][0])
    ring = pts0[i0:] + pts0[:i0]
    ring = ring + [ring[0]]
    pts = ring

    def rec(a, b):
        best, bi = 0, -1
        (x0, y0), (x1, y1) = pts[a], pts[b]
        l = math.hypot(x1 - x0, y1 - y0) or 1e-9
        for i in range(a + 1, b):
            d = abs((x1 - x0) * (y0 - pts[i][1]) - (x0 - pts[i][0]) * (y1 - y0)) / l
            if d > best:
                best, bi = d, i
        return rec(a, bi) + [bi] + rec(bi, b) if best > eps else []

    h = len(ring) // 2
    keep = sorted({0, h, len(ring) - 1} | set(rec(0, h)) | set(rec(h, len(ring) - 1)))
    return [ring[k] for k in keep][:-1]


def bbox(pts):
    return (min(p[0] for p in pts), max(p[0] for p in pts), min(p[1] for p in pts), max(p[1] for p in pts))


area = lambda pts: abs(sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))) / 2
fine = {}
for s in asset["representations"][src]["geometry"][view]:
    if s["role"] == "body":
        p = poly_of(s)
        if p:
            fine.setdefault(s["bone"], []).append((s["id"], p))
changes = []
geo = asset["representations"][scale]["geometry"][view]
for s in geo:
    if s["role"] != "body" or (only and s["id"] not in only):
        continue
    cand = fine.get(s["bone"])
    if not cand:
        print("no finer outline for", s["id"], s["bone"])
        continue
    cur = poly_of(s) if not s.get("closed", True) is False else None
    if cur is not None and not only:
        b = bbox(cur)
        diff = min(max(abs(b[i] - bbox(p)[i]) for i in range(4)) for _, p in cand)
        if diff < 0.04:
            continue
    sid, poly = max(cand, key=lambda c: area(c[1]))
    new = dp(poly, tol)
    changes.append((s["id"], sid, len(new)))
    s["_new"] = [[round(x, 5), round(y, 5)] for x, y in new]
for c in changes:
    print("redraw", c)
if "--apply" in sys.argv and changes:
    rep0 = text.index('"representations"')
    gm = [m for m in re.finditer(r'"geometry": \{', text[rep0:])]
    out = text
    patches = []
    for m in gm:
        gstart = rep0 + m.end() - 1
        scl = re.findall(r'"denominator": (\d+)', text[rep0:gstart])[-1]
        if scl != scale:
            continue
        geo_, gend = dec.raw_decode(text, gstart)
        vm = re.compile(r'"%s": \[' % view).search(text, gstart, gend)
        ls = vm.end() - 1
        lst, le = dec.raw_decode(text, ls)
        p = ls + 1
        for sh in lst:
            while text[p] in " \n,":
                p += 1
            obj, e = dec.raw_decode(text, p)
            tgt = next((g for g in geo if g["id"] == sh["id"] and "_new" in g), None)
            if tgt is not None:
                for k in ("penMm", "d", "x0", "y0", "x1", "y1", "r0", "r1", "followTip", "points", "closed", "kind", "filled"):
                    obj.pop(k, None)
                obj["filled"] = True
                obj["kind"] = "poly"
                obj["points"] = tgt["_new"]
                obj["closed"] = True
                new_text = json.dumps(obj, ensure_ascii=False, indent=2)
                ind = " " * (len(text[text.rfind("\n", 0, p) + 1 : p]))
                new_text = new_text.replace("\n", "\n" + ind)
                patches.append((p, e, new_text))
            p = e
    fresh = open(path, encoding="utf-8").read()
    assert fresh == text, "file changed under me"
    for p, e, nt in reversed(patches):
        out = out[:p] + nt + out[e:]
    json.loads(out)
    open(path, "w", encoding="utf-8").write(out)
    print("patched", len(patches))
