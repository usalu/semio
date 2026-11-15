import importlib.util, os, math, json, glob, re, collections
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-04-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
from math import sqrt
V = lib.Volume("tower-crane")
asset = json.load(open(glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/tower-crane/🔣️.json")[0]))["asset"]
F = V.frames
ST = dict(material="steel-s355", role="body")
CO = dict(material="concrete", role="body")
MA = dict(material="machinery", role="body")
WR = dict(material="wire-rope", role="detail")
GL = dict(material="glass", role="glazing")
SK = dict(material="skip-steel", role="body")


def rad(length, r, tie=False):
    return length / 41.0 if tie and length / (2 * r) < 20.5 and length / 41.0 >= 0.8 * r else r


def SL(length, width):
    return {"slender": True} if length / width >= 20.5 else {}


def W(bone, p):
    f = F[bone]
    return [f["o"][k] + p[0] * f["u"][k] + p[1] * f["v"][k] + p[2] * f["w"][k] for k in range(3)]


def Wd(bone, d):
    f = F[bone]
    return [p[0] * f["u"][k] + p[1] * f["v"][k] + p[2] * f["w"][k] for k in range(3) for p in [d]]


def members(scale):
    g = asset["representations"][str(scale)]["geometry"]
    out = collections.OrderedDict()
    for view in g:
        for s in g[view]:
            if s["kind"] == "member" and s["id"] not in out:
                out[s["id"]] = s
    fam = collections.OrderedDict()
    for i, s in out.items():
        fam.setdefault((s["bone"], re.sub(r"-\d+$", "", i)), []).append(s)
    return fam


def ap(items):
    if len(items) < 3:
        return len(items) == 2 and [items[1]["from"][k] - items[0]["from"][k] for k in "xyz"] == [items[1]["to"][k] - items[0]["to"][k] for k in "xyz"]
    d = [items[1]["from"][k] - items[0]["from"][k] for k in "xyz"]
    for a, b in zip(items, items[1:]):
        if any(abs((b["from"][k] - a["from"][k]) - d[i]) > 1e-6 or abs((b["to"][k] - a["to"][k]) - d[i]) > 1e-6 for i, k in enumerate("xyz")):
            return False
    return True


def lattice_bars(V, fam, rmin, rscale=1.0, only=None, skip=()):
    ids = []
    for (bone, stem), items in fam.items():
        if only and not any(stem.startswith(o) for o in only):
            continue
        if any(stem.startswith(o) for o in skip):
            continue
        sec = items[0]["section"]
        groups = [items] if ap(items) else [[i] for i in items]
        for gi, grp in enumerate(groups):
            a = W(bone, [grp[0]["from"][k] for k in "xyz"])
            b = W(bone, [grp[0]["to"][k] for k in "xyz"])
            rep = None
            if len(grp) > 1:
                step = Wd(bone, [grp[1]["from"][k] - grp[0]["from"][k] for k in "xyz"])
                rep = {"count": len(grp), "step": step}
            sid = stem if len(groups) == 1 else f"{stem}-{gi + 1}"
            if sec["kind"] == "circle":
                r = rad(lib.length(lib.sub(b, a)), max(sec["d"] / 2 * rscale, rmin), "tie" in stem)
                if ("diag" in stem or "brace" in stem) and bone in ("counter-jib", "jib"):
                    u = lib.unit(lib.sub(b, a))
                    a, b = lib.add(a, lib.mul(u, 0.8 * r)), lib.sub(b, lib.mul(u, 0.8 * r))
                V.cyl(sid, bone, a, b, r, rep=rep, tone=0.5, **SL(lib.length(lib.sub(b, a)), 2 * r), **ST)
            else:
                w = max(sec["w"] * rscale, 2 * rmin)
                V.box(sid, bone, lib.lerp(a, b, 0.5), [lib.length(lib.sub(b, a)), w, w], axes=lib.frame_along(a, b, (1, 0, 0) if abs(lib.sub(b, a)[0]) < 1e-6 else (0, 1, 0)), rep=rep, tone=0.5, **SL(lib.length(lib.sub(b, a)), w), **ST)
            ids.append((sid, bone))
    return ids


def unrolled_members(V):
    for (bone, stem), items in members(50).items():
        if bone not in ("mast", "jib", "counter-jib"):
            continue
        for it in items:
            a = W(bone, [it["from"][k] for k in "xyz"])
            b = W(bone, [it["to"][k] for k in "xyz"])
            sec = it["section"]
            if sec["kind"] == "circle":
                u = lib.unit(lib.sub(b, a))
                r = rad(lib.length(lib.sub(b, a)), max(sec["d"] / 2, 0.01), "tie" in stem)
                if ("diag" in stem or "brace" in stem) and bone in ("counter-jib", "jib"):
                    a, b = lib.add(a, lib.mul(u, 0.8 * r)), lib.sub(b, lib.mul(u, 0.8 * r))
                V.cyl("member-" + it["id"], bone, a, b, r, tone=0.5, member=bone, part="member", **SL(lib.length(lib.sub(b, a)), 2 * r), **ST)
            else:
                w = max(sec["w"], 0.02)
                V.box("member-" + it["id"], bone, lib.lerp(a, b, 0.5), [lib.length(lib.sub(b, a)), w, w], axes=lib.frame_along(a, b, (1, 0, 0) if abs(lib.sub(b, a)[0]) < 1e-6 else (0, 1, 0)), tone=0.5, member=bone, part="member", **SL(lib.length(lib.sub(b, a)), w), **ST)


def hook_path():
    pts = [(0, -0.5), (0, -0.8)]
    def bz(p0, p1, p2, p3, n=7):
        return [tuple((1 - t) ** 3 * p0[i] + 3 * (1 - t) ** 2 * t * p1[i] + 3 * (1 - t) * t ** 2 * p2[i] + t ** 3 * p3[i] for i in (0, 1)) for t in [k / n for k in range(1, n + 1)]]
    pts += bz((0, -0.8), (0.3, -0.8), (0.35, -1.15), (0.05, -1.2)) + bz((0.05, -1.2), (-0.1, -1.22), (-0.16, -1.1), (-0.1, -1.02), 5)
    return pts


def fixed_parts(S, scale):
    f = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[scale]
    V.at(scale, "body")
    if scale == 50:
        for k in range(4):
            V.box(f"ballast-block-{k + 1}", "base", [-2.625 + 1.75 * k, 0.5, 0], [1.75, 1.0, 3.5], cr=0.02, tone=0.35, **CO)
    else:
        V.ext("ballast-slab", "base", "plan", [(-3.5, -1.75), (3.5, -1.75), (3.5, 1.75), (-3.5, 1.75)], 0.0, 1.0, holes=[[(-1.5, -1.5), (1.5, -1.5), (1.5, 1.5), (-1.5, 1.5)]], tone=0.35, **CO)
        V.box("mast-foot-core", "base", [0, 0.5, 0], [3.0, 1.0, 3.0], tone=0.5, **ST)
    V.ext("mast-foot", "base", "plan", [(-1.5, -1.5), (1.5, -1.5), (1.5, 1.5), (-1.5, 1.5)], 1.0, 1.5, taper=[1.0, 0.6667], tone=0.5, **ST)
    seg = 24 if scale == 50 else 16
    V.ext("slew-ring", "slewing", "plan", [(1.4 * math.cos(2 * math.pi * k / 32), 1.0 * math.sin(2 * math.pi * k / 32)) for k in range(32)], 40.0, 40.5, tone=0.3, material="machinery", role="detail")
    V.box("slew-platform", "slewing", [0.5, 40.85, 0], [5.8, 0.7, 2.4], cr=0.03 if scale == 50 else None, tone=0.5, **MA)
    V.box("cab", "slewing", [2.35, 42.3, -0.5], [1.9, 2.2, 1.0], cr=0.1 if scale <= 100 else None, tone=0.6, **MA)
    V.box("cab-glass-front", "slewing", [3.29, 42.45, -0.5], [0.02, 1.5, 0.7], tone=0.85, **GL)
    V.box("cab-glass-side", "slewing", [2.6, 42.45, -0.98], [1.2, 1.5, 0.04], tone=0.85, **GL)
    if scale == 50:
        for k in range(3):
            V.box(f"counterweight-slab-{k + 1}", "counterweight", [-14.9, 41.5 + 0.3667 + 0.7333 * k, 0], [2.4, 0.7333, 2.4], tone=0.3, **CO)
    else:
        V.box("counterweight-slab", "counterweight", [-14.9, 41.5 + 1.1, 0], [2.4, 2.2, 2.4], tone=0.3, **CO)
    t = f
    V.box("winch-house-wall-l", "winch", [-8, 46.05, 0.7 - t / 2], [4, 1.8, t], tone=0.55, **MA)
    V.box("winch-house-wall-r", "winch", [-8, 46.05, -0.7 + t / 2], [4, 1.8, t], tone=0.55, **MA)
    V.box("winch-house-end-f", "winch", [-6 - t / 2, 46.05, 0], [t, 1.8, 1.4 - 2 * t], tone=0.55, **MA)
    V.box("winch-house-end-b", "winch", [-10 + t / 2, 46.05, 0], [t, 1.8, 1.4 - 2 * t], tone=0.55, **MA)
    V.box("winch-house-roof", "winch", [-8, 46.95 - t / 2, 0], [4 - 2 * t, t, 1.4 - 2 * t], tone=0.55, **MA)
    V.box("winch-house-floor", "winch", [-8, 45.15 + t / 2, 0], [4 - 2 * t, t, 1.4 - 2 * t], tone=0.55, **MA)
    if scale < 500:
        V.box("winch-motor-grille", "winch", [-9, 46.95 - t / 2, 0], [1.2, t, 0.8], tone=0.15, material="machinery", role="detail")
    V.cyl("winch-vent-stack", "winch", [-8, 46.95, 0], [-8, 47.15, 0], 0.2, tone=0.3, material="machinery", role="detail")
    V.box("trolley-frame", "trolley", [3, 43.325, 0], [1.4, 0.55, 2.0], tone=0.4, **MA)
    V.box("trolley-wheel", "trolley", [2.6, 43.54, 0], [0.2, 0.12, 1.9], rep={"count": 2, "step": [0.8, 0, 0]}, tone=0.2, material="machinery", role="detail")
    V.cyl("trolley-sheave", "trolley", [3, 43.28, -0.1], [3, 43.28, 0.1], 0.18, tone=0.3, material="steel-s355", role="detail")
    V.box("block-body", "hoist-block", [3, 37.05, 0], [0.56, 0.5, 0.56], cr=0.03 if scale == 50 else None, tone=0.3, **SK)
    V.cyl("block-sheave", "hoist-block", [3, 37.05, -0.1], [3, 37.05, 0.1], 0.18, tone=0.5, material="steel-s355", role="detail")
    hp = [([3 + x, 37.3 + y, 0], 0.04, 0.04) for x, y in hook_path()]
    V.loft("hook", "hoist-block", hp[::1 if scale == 50 else 2], hint=(0, 0, 1), caps=(True, True), tone=0.2, material="skip-steel", role="detail")
    V.ext("skip-body", "skip", "plan", [(2.55, -0.45), (3.45, -0.45), (3.45, 0.45), (2.55, 0.45)], 34.8, 35.8, taper=[1.0, 1.5556], tone=0.45, **SK)
    V.box("skip-gate", "skip", [3, 34.725, 0], [0.4, 0.15, 0.4], tone=0.25, **SK)
    for i in range(8):
        y0 = 43.05 - 5 * i
        V.cyl(f"rope-{i + 1}", f"rope-{i + 1}", [3, y0, 0], [3, y0 - 5.75, 0], {50: 0.0095, 100: 0.0065, 200: 0.013, 500: 0.0325}[scale], tone=0.2, slender=True, **WR)
    V.cyl("head-sheave", "slewing", [0, 48.6, -0.1], [0, 48.6, 0.1], 0.25, tone=0.3, material="steel-s355", role="detail")
    V.cyl("jib-tip-sheave", "jib", [55, 44.4, -0.1], [55, 44.4, 0.1], 0.3, tone=0.3, material="steel-s355", role="detail")
    rr = {50: 0.006, 100: 0.0065, 200: 0.013, 500: 0.0325}[scale]
    V.cap("pendant-fore-l", "slewing", [0, 48.6, 0.12], [22, 45.2, 0.135], rr, tone=0.3, slender=True, **WR)
    V.cap("pendant-fore-r", "slewing", [0, 48.6, -0.12], [50, 45.2, -0.135], 0.07 if scale == 500 else rr, tone=0.3, slender=True, **WR)
    V.cap("pendant-back", "slewing", [0, 48.6, 0], [-15.6, 45.1, 0], rr, tone=0.3, slender=True, **WR)




def offset_polygon(P, h):
    n = len(P)
    def nrm(a, b):
        dx, dy = b[0] - a[0], b[1] - a[1]
        l = math.hypot(dx, dy)
        return (-dy / l, dx / l)
    left, right = [], []
    for i, p in enumerate(P):
        if i == 0:
            m = nrm(P[0], P[1]); k = 1.0
        elif i == n - 1:
            m = nrm(P[-2], P[-1]); k = 1.0
        else:
            n1, n2 = nrm(P[i - 1], p), nrm(p, P[i + 1])
            sx, sy = n1[0] + n2[0], n1[1] + n2[1]
            den = 1 + n1[0] * n2[0] + n1[1] * n2[1]
            m = (sx / den, sy / den); k = 1.0
        left.append((p[0] + h * m[0], p[1] + h * m[1]))
        right.append((p[0] - h * m[0], p[1] - h * m[1]))
    return left + right[::-1]


def local_ext(V, id, bone, plane, profile, a0, a1, holes=None, taper=None, rep=None, **m):
    d = V._meta(id, bone, "extrude", m)
    d["profile"] = [lib.rv(p) for p in profile]
    if holes:
        d["holes"] = [[lib.rv(p) for p in h] for h in holes]
    d["plane"] = plane
    d["from"], d["to"] = lib.r5(min(a0, a1)), lib.r5(max(a0, a1))
    if taper:
        d["taperTo"] = lib.rv(taper)
    if rep:
        d["repeat"] = rep
    return V.put(d)


def strip(V, id, bone, items_a, items_b, half, thick, **m):
    pts = []
    for ia, ib in zip(items_a, items_b):
        pts += [[ia["from"][k] for k in "xyz"], [ia["to"][k] for k in "xyz"]]
        last = [ib["to"][k] for k in "xyz"]
    pts.append(last)
    return strip_pts(V, id, bone, pts, half, thick, **m)


def strip_pts(V, id, bone, pts, half, thick, **m):
    clean = [pts[0]]
    for p in pts[1:]:
        if max(abs(p[k] - clean[-1][k]) for k in range(3)) > 1e-6:
            clean.append(p)
    pts = clean
    for const, plane, idx in ((2, "xy", (0, 1)), (1, "zx", (2, 0)), (0, "yz", (1, 2))):
        if all(abs(p[const] - pts[0][const]) < 1e-6 for p in pts):
            break
    prof = offset_polygon([(p[idx[0]], p[idx[1]]) for p in pts], half)
    c = pts[0][const]
    return local_ext(V, id, bone, plane, prof, c - thick / 2, c + thick / 2, **m)


def fam_of(fam, bone, stem):
    return fam[(bone, stem)]


def tower_head_apex_pyramid(V, scale):
    local_ext(V, "tower-head", "slewing", "zx", [(-0.9, -1.2), (0.9, -1.2), (0.9, 1.2), (-0.9, 1.2)], 1.2, 8.6, taper=[1.0, 0.05], tone=0.8, **ST)


def tower_head_ties(V, scale):
    r = {50: 0.03, 100: 0.03, 200: 0.02, 500: 0.0375}[scale]
    for i, y in enumerate((2.6, 4.0, 5.4, 6.8)):
        t = (y - 1.2) / 7.4
        xh, zh = 1.2 * (1 - t), 0.9 - 0.78 * t
        wy = 40 + y
        V.cyl(f"tower-head-tie-x-{i + 1}", "slewing", [-xh, wy, zh], [xh, wy, zh], r, rep={"count": 2, "step": [0, 0, -2 * zh]}, tone=0.5, **SL(2 * xh, 2 * r), **ST)
        V.cyl(f"tower-head-tie-z-{i + 1}", "slewing", [xh, wy, -zh], [xh, wy, zh], r, rep={"count": 2, "step": [-2 * xh, 0, 0]}, tone=0.5, **SL(2 * zh, 2 * r), **ST)


def lattice_drawn(scale):
    V.at(scale, "body")
    s100 = scale == 100
    W_ = {100: dict(mc=0.08, md=0.05, mt=0.03, jc=0.08, jr=0.08, jd=0.06, je=0.04, jb=0.06, cc=0.09, cd=0.063, cb=0.063), 200: dict(mc=0.1, md=0.06, mt=0.06, jc=0.1, jr=0.1, jd=0.06, je=0.06, jb=0.086, cc=0.107, cd=0.063, cb=0.063)}[scale]
    cw = W_["mc"]
    tr = W_["mt"] / 2

    def legs(id, bone, a, b, d, count, step):
        V.cyl(id, bone, a, b, d / 2, rep={"count": count, "step": step}, tone=0.5, **SL(lib.length(lib.sub(b, a)), d), **ST)

    for nm, x in (("px", 1.0), ("nx", -1.0)):
        V.box(f"mast-chord-{nm}", "mast", [x, 20.5, 1.0], [cw, 39, cw], rep={"count": 2, "step": [0, 0, -2]}, tone=0.5, **SL(39, cw), **ST)
    for nm, z in (("pz", 1.0), ("nz", -1.0)):
        legs(f"mast-diag-a-{nm}", "mast", [1, 1, z], [-1, 3, z], W_["md"], 10, [0, 4, 0])
        legs(f"mast-diag-b-{nm}", "mast", [-1, 3, z], [1, 5, z], W_["md"], 9, [0, 4, 0])
    for nm, x in (("px", 1.0), ("nx", -1.0)):
        legs(f"mast-diag-a-{nm}", "mast", [x, 1, -1], [x, 3, 1], W_["md"], 10, [0, 4, 0])
        legs(f"mast-diag-b-{nm}", "mast", [x, 3, 1], [x, 5, -1], W_["md"], 9, [0, 4, 0])
    if s100:
        for nm, z in (("pz", 1.0), ("nz", -1.0)):
            V.cyl(f"mast-hz-{nm}", "mast", [-1, 1, z], [1, 1, z], tr, rep={"count": 20, "step": [0, 2, 0]}, tone=0.5, **SL(2, 2 * tr), **ST)
        for nm, x in (("px", 1.0), ("nx", -1.0)):
            V.cyl(f"mast-tie-{nm}", "mast", [x, 3, -1], [x, 3, 1], tr, rep={"count": 18, "step": [0, 2, 0]}, tone=0.5, **SL(2, 2 * tr), **ST)
        V.cyl("mast-end-tie-z", "mast", [1, 40, -1], [1, 40, 1], tr, rep={"count": 2, "step": [-2, 0, 0]}, tone=0.5, **SL(2, 2 * tr), **ST)
    else:
        for nm, z in (("pz", 1.0), ("nz", -1.0)):
            V.cyl(f"mast-end-tie-x-{nm}", "mast", [-1, 1, z], [1, 1, z], tr, rep={"count": 2, "step": [0, 39, 0]}, tone=0.5, **SL(2, 2 * tr), **ST)
        for nm, x in (("px", 1.0), ("nx", -1.0)):
            V.cyl(f"mast-end-tie-z-{nm}", "mast", [x, 1, -1], [x, 1, 1], tr, rep={"count": 2, "step": [0, 39, 0]}, tone=0.5, **SL(2, 2 * tr), **ST)
    V.box("jib-chord-top", "jib", [27.5, 45.2, 0.8], [55, W_["jc"], W_["jc"]], rep={"count": 2, "step": [0, 0, -1.6]}, tone=0.5, **SL(55, W_["jc"]), **ST)
    bz = 0.45
    V.box("jib-chord-bottom", "jib", [27.5, 43.6, bz], [55, W_["jr"], W_["jr"]], rep={"count": 2, "step": [0, 0, -2 * bz]}, tone=0.5, **SL(55, W_["jr"]), **ST)
    for side, z in (("l", 1.0), ("r", -1.0)):
        zb, zt = bz * z, 0.8 * z
        legs(f"jib-diag-up-{side}", "jib", [0, 43.6, zb], [1.8, 45.2, zt], W_["jd"], 15, [3.6, 0, 0])
        legs(f"jib-diag-down-{side}", "jib", [1.8, 45.2, zt], [3.6, 43.6, zb], W_["jd"], 15, [3.6, 0, 0])
    for nm, x in (("root", 0.0), ("tip", 55.0)):
        legs(f"jib-end-{nm}-v", "jib", [x, 43.6, 0.8], [x, 45.2, 0.8], W_["je"], 2, [0, 0, -1.6])
        legs(f"jib-end-{nm}-h", "jib", [x, 43.6, -0.8], [x, 43.6, 0.8], W_["je"], 2, [0, 1.6, 0])
    legs("jib-brace-a", "jib", [0, 45.2, -0.8], [1.8, 45.2, 0.8], W_["jb"], 15, [3.6, 0, 0])
    legs("jib-brace-b", "jib", [1.8, 45.2, 0.8], [3.6, 45.2, -0.8], W_["jb"], 15, [3.6, 0, 0])
    V.box("counter-chord-top", "counter-jib", [-8.25, 45.1, 0.7], [16.5, W_["cc"], W_["cc"]], rep={"count": 2, "step": [0, 0, -1.4]}, tone=0.5, **SL(16.5, W_["cc"]), **ST)
    V.box("counter-chord-bottom", "counter-jib", [-8.25, 43.7, 0.7], [16.5, W_["cc"], W_["cc"]], rep={"count": 2, "step": [0, 0, -1.4]}, tone=0.5, **SL(16.5, W_["cc"]), **ST)
    for side, z in (("l", 0.7), ("r", -0.7)):
        legs(f"counter-diag-down-{side}", "counter-jib", [0, 45.1, z], [-1.65, 43.7, z], W_["cd"], 5, [-3.3, 0, 0])
        legs(f"counter-diag-up-{side}", "counter-jib", [-1.65, 43.7, z], [-3.3, 45.1, z], W_["cd"], 5, [-3.3, 0, 0])
    legs("counter-brace-a", "counter-jib", [0, 45.1, -0.7], [-1.65, 45.1, 0.7], W_["cb"], 5, [-3.3, 0, 0])
    legs("counter-brace-b", "counter-jib", [-1.65, 45.1, 0.7], [-3.3, 45.1, -0.7], W_["cb"], 5, [-3.3, 0, 0])
    fam = members(scale)
    rm = {100: 0.0075, 200: 0.015}[scale]
    lattice_bars(V, fam, rm)
    if s100:
        tower_head_ties(V, scale)


def lattice(scale):
    V.at(scale, "body")
    fam = members(scale)
    f50 = members(50)
    if scale == 50:
        V.cyl("mast-ladder-rail", "mast", [0.55, 1.2, -0.2], [0.55, 39.8, -0.2], 0.02, rep={"count": 2, "step": [0, 0, 0.4]}, tone=0.3, texture=True, **ST)
        V.cyl("mast-ladder-rung", "mast", [0.55, 1.5, -0.2], [0.55, 1.5, 0.2], 0.0125, rep={"count": 125, "step": [0, 0.3, 0]}, tone=0.3, texture=True, **ST)
        ids = lattice_bars(V, fam, 0.01)
        tower_head_ties(V, scale)
        return ids + [(f"tower-head-tie-{k}-{i + 1}", "slewing") for i in range(4) for k in "xz"]
    ids = []
    rm = {100: 0.0075, 200: 0.015, 500: 0.0375}[scale]
    if scale == 500:
        V.box("mast-silhouette", "mast", [0, 20.5, 0], [2, 39, 2], tone=0.8, envelope=True, **ST)
        V.box("jib-silhouette", "jib", [27.5, 44.4, 0], [55, 1.6, 1.6], tone=0.8, envelope=True, **ST)
        V.box("counter-silhouette", "counter-jib", [-8.25, 44.4, 0], [16.5, 1.4, 1.4], tone=0.8, envelope=True, **ST)
        ids += lattice_bars(V, fam, rm)
        ids += lattice_bars(V, f50, rm, only=("tower-head-leg", "tower-head-frame"))
        tower_head_ties(V, scale)
        ids += [(f"tower-head-tie-{k}-{i + 1}", "slewing") for i in range(4) for k in "xz"]
        return [("mast-silhouette", "mast"), ("jib-silhouette", "jib"), ("counter-silhouette", "counter-jib")] + ids
    if scale in (100, 200):
        lattice_drawn(scale)
        return []
    cr = 0.08 if scale == 100 else 0.1
    sw = 0.025 if scale == 100 else 0.02
    th = 0.05 if scale == 100 else 0.04
    for s in ("pxpz", "pxnz", "nxpz", "nxnz"):
        a = f50[("mast", "mast-chord-" + s)][0]
        pa, pb = W("mast", [a["from"][k] for k in "xyz"]), W("mast", [a["to"][k] for k in "xyz"])
        V.box("mast-chord-" + s, "mast", lib.lerp(pa, pb, 0.5), [cr, lib.length(lib.sub(pb, pa)), cr], tone=0.5, **ST)
        ids.append(("mast-chord-" + s, "mast"))
    for face in ("pz", "nz", "px", "nx"):
        strip(V, "mast-diag-" + face, "mast", f50[("mast", "mast-diag-a-" + face)], f50[("mast", "mast-diag-b-" + face)], sw, th, tone=0.5, **ST)
        ids.append(("mast-diag-" + face, "mast"))
    if scale == 100:
        for face in ("px", "nx", "pz", "nz"):
            it = f50[("mast", "mast-tie-" + face)]
            a, b = W("mast", [it[0]["from"][k] for k in "xyz"]), W("mast", [it[0]["to"][k] for k in "xyz"])
            V.cyl("mast-tie-" + face, "mast", a, b, 0.0375, rep={"count": len(it), "step": Wd("mast", [it[1]["from"][k] - it[0]["from"][k] for k in "xyz"])}, tone=0.5, **ST)
            ids.append(("mast-tie-" + face, "mast"))
    else:
        for y in (1.0, 40.0):
            pass
        local_ext(V, "mast-end-frame", "mast", "zx", [(-0.96, 0), (0.96, 0), (0.96, 0), (-0.96, 0)], 0, 0.1) if False else None
        V.box("mast-end-tie-x", "mast", [0, 1.0, 0], [1.84, 0.06, 0.06], rep={"count": 2, "step": [0, 39.0, 0]}, tone=0.5, **ST)
        V.box("mast-end-tie-z", "mast", [0, 1.0, 0], [0.06, 0.06, 1.84], rep={"count": 2, "step": [0, 39.0, 0]}, tone=0.5, **ST)
        ids += [("mast-end-tie-x", "mast"), ("mast-end-tie-z", "mast")]
    jc = f50[("jib", "jib-chord")]
    ra = max(jc[0]["section"]["d"] / 2, rm)
    pa, pb = W("jib", [jc[0]["from"][k] for k in "xyz"]), W("jib", [jc[0]["to"][k] for k in "xyz"])
    V.cyl("jib-chord", "jib", pa, pb, ra, rep={"count": 2, "step": Wd("jib", [0, 0, -1.48])}, tone=0.5, **ST)
    jr = f50[("jib", "jib-rail")]
    pa, pb = W("jib", [jr[0]["from"][k] for k in "xyz"]), W("jib", [jr[0]["to"][k] for k in "xyz"])
    V.cyl("jib-rail", "jib", pa, pb, max(0.06, rm), rep={"count": 2, "step": Wd("jib", [0, 0, -0.9])}, tone=0.5, **ST)
    ids += [("jib-chord", "jib"), ("jib-rail", "jib")]
    for side in ("l", "r"):
        for kind in ("down", "up"):
            it = f50[("jib", f"jib-diag-{kind}-{side}")]
            a, b = W("jib", [it[0]["from"][k] for k in "xyz"]), W("jib", [it[0]["to"][k] for k in "xyz"])
            V.cyl(f"jib-diag-{kind}-{side}", "jib", a, b, max(0.04, rm), rep={"count": len(it), "step": Wd("jib", [it[1]["from"][k] - it[0]["from"][k] for k in "xyz"])}, tone=0.5, **ST)
            ids.append((f"jib-diag-{kind}-{side}", "jib"))
    strip(V, "jib-brace", "jib", f50[("jib", "jib-brace-a")], f50[("jib", "jib-brace-b")], 0.03 if scale == 100 else 0.02, 0.06 if scale == 100 else 0.04, tone=0.5, **ST)
    ids.append(("jib-brace", "jib"))
    for kind in ("top", "bottom"):
        it = f50[("counter-jib", f"counter-chord-{kind}")]
        pa, pb = W("counter-jib", [it[0]["from"][k] for k in "xyz"]), W("counter-jib", [it[0]["to"][k] for k in "xyz"])
        V.cyl(f"counter-chord-{kind}", "counter-jib", pa, pb, max(0.06, rm), rep={"count": 2, "step": Wd("counter-jib", [0, 0, -1.28])}, tone=0.5, **ST)
        ids.append((f"counter-chord-{kind}", "counter-jib"))
    for side in ("l", "r"):
        strip(V, f"counter-diag-{side}", "counter-jib", [f50[("counter-jib", f"counter-diag-down-{side}")][j] for j in range(5)], [f50[("counter-jib", f"counter-diag-up-{side}")][j] for j in range(5)], 0.03, 0.06, tone=0.5, **ST) if False else None
    for side in ("l", "r"):
        a_, b_ = f50[("counter-jib", f"counter-diag-down-{side}")], f50[("counter-jib", f"counter-diag-up-{side}")]
        strip(V, f"counter-diag-{side}", "counter-jib", a_, b_, 0.03 if scale == 100 else 0.02, 0.06 if scale == 100 else 0.04, tone=0.5, **ST)
        ids.append((f"counter-diag-{side}", "counter-jib"))
    strip(V, "counter-brace", "counter-jib", f50[("counter-jib", "counter-brace-a")], f50[("counter-jib", "counter-brace-b")], 0.03 if scale == 100 else 0.02, 0.06 if scale == 100 else 0.04, tone=0.5, **ST)
    ids.append(("counter-brace", "counter-jib"))
    ids += lattice_bars(V, fam, rm)
    tower_head_ties(V, scale)
    ids += [(f"tower-head-tie-{k}-{i + 1}", "slewing") for i in range(4) for k in "xz"]
    return ids


for sc in (50, 100, 200, 500):
    fixed_parts(V, sc)
    if sc == 500:
        pass
    lattice(sc)


def region_of(i):
    for pre, reg in (("ballast", "ballast"), ("counterweight", "counterweight"), ("slew-platform", "house"), ("slew-ring", "slew-drive"), ("cab-glass", "cab-glazing"), ("cab", "cab"), ("winch-house", "winch-house"), ("winch-vent", "winch-house"), ("trolley", "trolley"), ("block-body", "skip"), ("hook", "skip"), ("skip", "skip"), ("sling", "skip"), ("rope", "rope"), ("pendant", "pendants"), ("mast", "lattice"), ("jib-rail", "lattice"), ("jib-chord", "lattice"), ("jib-diag", "lattice"), ("jib-brace", "lattice"), ("jib-end", "lattice"), ("jib-silhouette", "lattice"), ("counter-", "lattice"), ("tower-head", "lattice")):
        if i.startswith(pre):
            return reg
    return None


def derive(scale):
    body = V.data[scale]["body"]
    V.at(scale, "thermal")
    for b in body:
        r = region_of(b["id"])
        if r is None or (r == "cab-glazing" and scale == 50) or (r == "cab" and scale == 50):
            continue
        V.ref("th-" + b["id"], b["id"], region=r, **({"sectionFaces": ["-v", "+v"]} if b["id"] == "slew-ring" else {}))
    if scale == 50:
        for w, (c, sz) in {"front": ([3.31, 42.45, -0.5], [0.02, 1.5, 0.7]), "side": ([2.6, 42.45, -0.98], [1.2, 1.5, 0.04])}.items():
            ax = 0 if w == "front" else 2
            off = [0.0, 0.0, 0.0]
            ths = [("pane-out", 0.004), ("gap", 0.012), ("pane-in", 0.004)]
            if w == "side":
                ths = ths[::-1]
            cur = -sz[ax] / 2
            for lid, t in ths:
                cc = list(c); cc[ax] = c[ax] + cur + t / 2
                ss = list(sz); ss[ax] = t
                V.box(f"th-glass-{w}-{lid}", "slewing", cc, ss, tone=0.85, material="glass", role="glazing", region="cab-glazing", layer=lid)
                cur += t
        V.box("th-cab-lining", "slewing", [2.35, 42.3, -0.5], [1.9, 2.2, 1.0], cr=0.1, tone=0.6, material="machinery", role="body", region="cab", layer="lining")
        V.box("th-cab-steel", "slewing", [2.35, 42.3, -0.5], [1.84, 2.14, 0.94], cr=0.07, tone=0.5, material="machinery", role="body", region="cab", layer="steel")
    V.box("th-hoist-motor", "winch", [-9, 45.95, 0], {50: [1.2, 0.8, 0.8], 100: [1.2, 0.8, 0.76], 200: [1.2, 0.8, 0.72], 500: [1.2, 0.8, 0.68]}[scale], tone=0.15, material="machinery", role="body", region="hoist-motor")
    V.rev("th-winch-vent-outlet", "winch", [-8, 47.15, 0], [0, 1, 0], [[0, 0.16], [0, 0.2], [{50: 0.02, 100: 0.04, 200: 0.06, 500: 0.1}[scale], 0.2], [{50: 0.02, 100: 0.04, 200: 0.06, 500: 0.1}[scale], 0.16], [0, 0.16]], tone=0.9, material="machinery", role="body", region="winch-house", tags=["winch-vent"])
    V.at(scale, "structure")
    bones = set()
    for b in body:
        if (b["bone"] in ("mast", "jib", "counter-jib") and scale != 50) or b["id"].startswith(("tower-head", "sling", "rope-", "mast-silhouette", "tower-head")) or b["id"] in ("slew-platform", "ballast-slab", "mast-foot") or b["id"].startswith("ballast-block"):
            if b["id"].startswith("pendant") or (scale == 500 and b["id"] in ("mast-silhouette", "jib-silhouette", "counter-silhouette")):
                continue
            V.ref("member-" + b["id"], b["id"], member=b["bone"], part="member", **({"slender": True} if b.get("slender") else {}))
    if scale == 50:
        unrolled_members(V)
    if scale == 500:
        for nm, x in (("px", 0.92), ("nx", -0.92)):
            V.box(f"member-mast-chord-{nm}", "mast", [x, 20.5, 0.92], [0.15, 39, 0.15], rep={"count": 2, "step": [0, 0, -1.84]}, tone=0.5, member="mast", part="member", slender=True, **ST)
        V.box("member-jib-chord", "jib", [27.5, 45.14, 0.74], [55, 0.14, 0.14], rep={"count": 2, "step": [0, 0, -1.48]}, tone=0.5, member="jib", part="member", slender=True, **ST)
        V.box("member-jib-rail", "jib", [27.5, 43.66, 0.45], [55, 0.12, 0.12], rep={"count": 2, "step": [0, 0, -0.9]}, tone=0.5, member="jib", part="member", slender=True, **ST)
        for nm, y in (("top", 45.04), ("bottom", 43.76)):
            V.box(f"member-counter-chord-{nm}", "counter-jib", [-8.25, y, 0.64], [16.5, 0.12, 0.12], rep={"count": 2, "step": [0, 0, -1.28]}, tone=0.5, member="counter-jib", part="member", slender=True, **ST)
    V.box("member-winch-frame", "winch", [-8, 45.3, 0], [4, 0.3, 1.2], tone=0.5, member="winch", part="member", **ST)
    V.cyl("joint-slew-bearing", "turntable", [0, 40.0, 0], [0, 40.5, 0], {50: 0.9, 100: 0.92, 200: 0.95, 500: 1.0}[scale], tone=0.4, member="turntable", part="joint", **ST)
    jl = {50: 0.3, 100: 0.4, 200: 0.5, 500: 0.9}[scale]
    if scale == 50:
        V.box("joint-jib-root", "jib", [jl / 2, 45.14, 0.74], [jl, 0.1, 0.1], rep={"count": 2, "step": [0, 0, -1.48]}, tone=0.4, member="jib", part="joint", **ST)
        V.box("joint-counter-root", "counter-jib", W("counter-jib", [jl / 2, 0.64, 0.64]), [jl, 0.08, 0.08], rep={"count": 2, "step": [0, 0, -1.28]}, tone=0.4, member="counter-jib", part="joint", **ST)
    elif scale < 500:
        jw, cw_ = {100: (0.08, 0.09), 200: (0.1, 0.107)}[scale]
        V.box("joint-jib-root", "jib", [jl / 2, 45.2, 0.8], [jl, jw, jw], rep={"count": 2, "step": [0, 0, -1.6]}, tone=0.4, member="jib", part="joint", **ST)
        V.box("joint-counter-root", "counter-jib", [-jl / 2, 45.1, 0.7], [jl, cw_, cw_], rep={"count": 2, "step": [0, 0, -1.4]}, tone=0.4, member="counter-jib", part="joint", **ST)
    else:
        V.box("joint-jib-root", "jib", [jl / 2, 44.4, 0], [jl, 1.2, 1.2], tone=0.4, member="jib", part="joint", **ST)
        V.box("joint-counter-root", "counter-jib", W("counter-jib", [jl / 2, 0, 0]), [jl, 1.0, 1.0], tone=0.4, member="counter-jib", part="joint", **ST)
    V.ref("joint-trolley", "trolley-frame", member="trolley", part="joint")
    V.ref("joint-hoist-block", "block-body", member="hoist-block", part="joint")
    V.ref("mass-counterweight", "counterweight-slab-2" if scale == 50 else "counterweight-slab", member="counterweight", part="mass")
    V.ref("mass-trolley", "trolley-frame", member="trolley", part="mass")
    V.ell("mass-skip", "skip", [3, 35.2, 0], [0.4, 0.4, 0.4], tone=0.4, member="skip", part="mass", **SK)
    V.ell("mass-winch", "winch", [-9, 45.95, 0], [0.45, 0.3, 0.3], tone=0.4, member="winch", part="mass", **MA)
    V.box("mass-base", "base", [0, 0.5, 0], {50: [6.6, 0.8, 3.2], 100: [6.4, 0.8, 3.0], 200: [6.2, 0.8, 2.8], 500: [6.0, 0.8, 2.6]}[scale], tone=0.4, member="base", part="mass", **CO)
    V.box("base-fixed-block", "base", [0, 0.125, 0], [7, 0.25, 3.5], tone=0.4, member="base", part="support", anchor="base-fixed", **CO)


for sc in (50, 100, 200, 500):
    derive(sc)
    if sc == 50:
        pass
EN = {
 50: ("Detailed model: every 2D member of the 1:50 drawing is a real bar (322 members as repeat families): four 160 mm mast chords, 90 mm ties and 80 mm diagonals on four faces, trapezoidal jib (140 mm top chords at +-0.74 m, 120 mm bottom rails at +-0.45 m, 80 mm side diagonals, 60 mm top bracing), counter-jib, A-frame tower head with 16 diagonals and four tie levels, slings, a rung ladder inside the mast (texture, 125 rungs); ballast as four separate blocks, frustum mast foot, slewing ring ellipse 2.8 m along the jib by 2.0 m, platform, rounded cab with two glazing panes flush with its walls, three stacked counterweight slabs, winch house as six hollow walls with louvre and vent stack, trolley frame with axle wheels, sheaves, block, swept hook, tapered skip, eight 19 mm rope pieces and three 12 mm pendants as slender solids. Bars at least 20 times longer than wide are flagged slender (C2); the ties are 90 mm instead of the drawn 100 mm so that they qualify.",
      "Thermal: a reference to every body solid of a region; the cab is two nested shells (30 mm lining outside, 3 mm steel inside) and each window three stacked sheets (4 / 12 / 4 mm); the hoist motor is a box inside the winch house; the winch vent emitter is a 400 mm outlet ring on its stack. Structure: every lattice bar is a member of its own (292 bars) with the platform, the ballast blocks, the tower head and the ropes, the slew bearing, the jib and counter-jib roots, trolley and block are joints, counterweight, trolley, skip content, winch motor and ballast are masses, a 7 m x 3.5 m fixed block is the foundation support."),
 100: ("Redrawn against the 1:100 drawing: every stroke of the lattice is a bar centred on it and as wide as the pen draws it (mast chords 80 mm, diagonals 50 mm, horizontals 30 mm; jib chords 80 mm at +-0.8 m on top and rails 80 mm at +-0.45 m below, diagonals 60 mm, end frames 40 mm, top bracing as a zigzag; counter-jib chords 90 mm, diagonals and bracing 63 mm); the mast diagonals pitch 2 m on all four faces; ballast is one slab with the mast-foot core in its hole, the counterweight one block, the cab a single rounded box, ropes and pendants 13 mm.",
      "Thermal and structure are derived from the body like at 1:50 but with one skin per region (layers are merged at this scale); structure members are the lattice bars themselves, joints sit inside the jib and counter-jib chords; same support, masses and joints as 1:50 in simpler sizes."),
 200: ("Redrawn against the 1:200 drawing: the jib has 100 mm top chords at +-0.8 m and 100 mm bottom rails at +-0.45 m, 86 mm diagonals on both sides, 86 mm top bracing and 60 mm end frames, the counter-jib has 107 mm chords and 63 mm diagonals and bracing, the mast four 100 mm chords with 60 mm diagonals on all faces and end ties at both ends (the horizontals of 1:100 are dropped), the tower head legs, frame and diagonals only; the winch house walls are 30 mm, the hook is omitted, ropes and pendants are 26 mm.",
      "Thermal and structure are derived from the body with one skin per region; the lattice bars are members, the joints are 500 mm blocks in the chords; the region areas are sums over the exposed bar surfaces."),
 500: ("Merged masses: mast, jib and counter-jib are prisms of their envelope (marked envelope: true, the drawing fills them), the tower head is four 100 mm legs with a base frame and four tie levels, the winch house four hollow 75 mm walls, ballast and counterweight single blocks, ropes 65 mm, the pendants 65 mm and 140 mm as the drawing strokes are wide; the cab and platform are boxes.",
      "Thermal: references to the body solids; the lattice region is covered by the envelope prisms and the tower head bars, the hull surface of a prism stands for a porous lattice and is left out of the area sums (C22). Structure: the real chords inside the prisms (four 150 mm mast chords, two 140 mm jib chords, two 120 mm rails and four 120 mm counter-jib chords), tower head, ropes and platform are members, the jib and counter-jib roots 900 mm joint blocks, the same support and masses."),
}
DE = {
 50: ("Detailmodell: Jeder 2D-Stab der Zeichnung 1:50 ist ein echter Stab (322 Stäbe als Wiederholungsfamilien): vier Mastgurte 160 mm, Riegel 90 mm und Diagonalen 80 mm auf vier Seiten, trapezförmiger Ausleger (Obergurte 140 mm bei +-0,74 m, Untergurtschienen 120 mm bei +-0,45 m, Seitendiagonalen 80 mm, Kopfverband 60 mm), Gegenausleger, A-Bock mit 16 Diagonalen und vier Riegelebenen, Anschlagseile, eine Leiter im Mast (Textur, 125 Sprossen); Ballast aus vier getrennten Blöcken, Mastfuß als Pyramidenstumpf, Drehkranz-Ellipse 2,8 m entlang des Auslegers mal 2,0 m, Plattform, gerundete Kabine mit zwei bündigen Scheiben, drei gestapelte Gegengewichtsplatten, Windenhaus aus sechs hohlen Wänden mit Lamelle und Lüfterstutzen, Katzrahmen mit Radachsen, Seilscheiben, Flasche, geführter Haken, konischer Kübel, acht Seilstücke mit 19 mm und drei Abspannungen mit 12 mm als schlanke Körper. Stäbe, die mindestens 20-mal länger als breit sind, tragen das Zeichen schlank (C2); die Riegel sind 90 mm statt der gezeichneten 100 mm, damit sie dazugehören.",
      "Thermik: ein Verweis auf jeden Körper einer Region; die Kabine besteht aus zwei verschachtelten Schalen (30 mm Auskleidung außen, 3 mm Stahl innen), jedes Fenster aus drei gestapelten Scheiben (4 / 12 / 4 mm); der Hubmotor ist ein Kasten im Windenhaus; der Lüfter-Emitter ist ein 400-mm-Austrittsring auf dem Stutzen. Tragwerk: jeder Gitterstab ist ein eigener Stab (292 Stäbe), dazu Plattform, Ballastblöcke, Turmspitze und Seile; Drehlager, Wurzeln von Ausleger und Gegenausleger, Katze und Flasche sind Gelenke, Gegengewicht, Katze, Kübelinhalt, Windenmotor und Ballast sind Massen, ein Block 7 m x 3,5 m ist das Fundamentauflager."),
 100: ("Neu gezeichnet gegen die Zeichnung 1:100: Jeder Strich des Gitters ist ein Stab, auf ihm zentriert und so breit wie der Stift zeichnet (Mastgurte 80 mm, Diagonalen 50 mm, Riegel 30 mm; Auslegergurte 80 mm bei +-0,8 m oben und Schienen 80 mm bei +-0,45 m unten, Diagonalen 60 mm, Endrahmen 40 mm, Kopfverband als Zickzack; Gegenauslegergurte 90 mm, Diagonalen und Verband 63 mm); die Mastdiagonalen steigen auf allen vier Seiten je 2 m; der Ballast ist eine Platte mit dem Mastfußkern in ihrem Loch, das Gegengewicht ein Block, die Kabine ein gerundeter Kasten, Seile und Abspannungen 13 mm.",
      "Thermik und Tragwerk wie bei 1:50 aus dem Körper abgeleitet, aber mit einer Haut je Region (Schichten sind in diesem Maßstab zusammengefasst); die Tragwerksstäbe sind die Gitterstäbe selbst, die Gelenke sitzen in den Gurten von Ausleger und Gegenausleger; gleiches Auflager und gleiche Massen wie bei 1:50 in einfacheren Größen."),
 200: ("Neu gezeichnet gegen die Zeichnung 1:200: Der Ausleger hat Obergurte von 100 mm bei +-0,8 m und Untergurtschienen von 100 mm bei +-0,45 m, Diagonalen 60 mm auf beiden Seiten, Kopfverband 86 mm und Endrahmen 60 mm, der Gegenausleger hat Gurte 107 mm sowie Diagonalen und Verband 63 mm, der Mast vier Gurte 100 mm mit Diagonalen 60 mm auf allen Seiten und Endriegeln an beiden Enden (die Querriegel von 1:100 entfallen), die Turmspitze nur Beine, Rahmen und Diagonalen; die Wände des Windenhauses sind 30 mm, der Haken entfällt, Seile und Abspannungen sind 26 mm.",
      "Thermik und Tragwerk aus dem Körper abgeleitet mit einer Haut je Region; die Gitterstäbe sind Stäbe des Tragwerks, die Gelenke 500-mm-Blöcke in den Gurten; die Regionsflächen sind Summen über die freien Stabflächen."),
 500: ("Zusammengefasste Massen: Mast, Ausleger und Gegenausleger sind Prismen ihrer Hüllen (mit envelope: true markiert, die Zeichnung füllt sie), die Turmspitze besteht aus vier Beinen von 100 mm mit Grundrahmen und vier Riegelebenen, das Windenhaus aus vier hohlen 75-mm-Wänden, Ballast und Gegengewicht sind einzelne Blöcke, Seile 65 mm, die Abspannungen 65 mm und 140 mm so breit wie die Striche der Zeichnung; Kabine und Plattform sind Kästen.",
      "Thermik: Verweise auf die Körper; die Gitterregion ist durch die Hüllprismen und die Stäbe der Turmspitze abgedeckt, die Hüllfläche eines Prismas steht für ein poröses Gitter und bleibt aus den Flächensummen heraus (C22). Tragwerk: die echten Gurte in den Prismen (vier Mastgurte 150 mm, zwei Auslegergurte 140 mm, zwei Schienen 120 mm und vier Gegenauslegergurte 120 mm), Turmspitze, Seile und Plattform sind Stäbe, die Wurzeln von Ausleger und Gegenausleger 900-mm-Gelenkblöcke, gleiches Auflager und gleiche Massen."),
}
for sc in (50, 100, 200, 500):
    for k in (0, 1):
        V.decide(sc, EN[sc][k], DE[sc][k])
V.write()
