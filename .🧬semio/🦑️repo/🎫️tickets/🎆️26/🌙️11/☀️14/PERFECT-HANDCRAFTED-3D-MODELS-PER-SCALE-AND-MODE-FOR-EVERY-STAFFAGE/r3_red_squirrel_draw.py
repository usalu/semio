"""✏️ Round 3 red squirrel: the 2D drawings of both authored scales redrawn from the handcrafted body (ruling E1).

Every shape is the outline of one body solid in the frame of its bone, thinned to the points the scale can show, and carries a `solid` hint, so side elevation, front elevation and plan are projections of one body in every pose. A part that is deeper than wide is the prism of its side outline over its lateral span; a part that is clearly rounded or flat gets a second (and third) shape of the same tone: the body of revolution of its lateral half width (`-core`) and the prism of its plan outline over its depth (`-plan`). In each view the shape whose outline is true there lies on top. The table of parts per scale is stated by hand.
"""
import math
import r3_red_squirrel_rig as rig
from r3_red_squirrel_fk import chain
from r3_red_squirrel_geo import refine, outline, simplify, pchip

M = 0.001
REST_Z = {}
for _b in rig.BONES:
    REST_Z[_b["id"]] = _b["z"] * 1000 if "z" in _b else (REST_Z[_b["parent"]] if _b["parent"] else 0.0)


def rnd(points, n=5):
    out = []
    for x, y in points:
        p = [round(x * M, n), round(y * M, n)]
        if not out or out[-1] != p:
            out.append(p)
    if len(out) > 1 and out[0] == out[-1]:
        out.pop()
    return out


def stations(solid, sub=3):
    return refine(solid["_st"]["stations"], max(solid["_st"]["sub"], sub))


def field(solid, f, u):
    s = solid["_st"]["stations"]
    us = [x["u"] for x in s]
    return pchip(us, [x[f] for x in s], min(max(u, us[0]), us[-1]))


def side_ring(solid):
    s = stations(solid)
    return [(x["u"], x["vc"] + x["rv"]) for x in s] + [(x["u"], x["vc"] - x["rv"]) for x in s][::-1]


def plan_ring(solid):
    s = stations(solid)
    return [(x["u"], x["wc"] + x["rw"]) for x in s] + [(x["u"], x["wc"] - x["rw"]) for x in s][::-1]


def core_ring(solid, axis_w=0.0):
    s = stations(solid)
    r = [(x["u"], max(min(x["rw"] - abs(x["wc"] - axis_w), x["rv"] - abs(x["vc"])), 0.0)) for x in s]
    return [(s[0]["u"], 0.0)] + r + [(s[-1]["u"], 0.0)]


def ellipse_ring(cx, cy, rx, ry, n=12):
    return [(cx + rx * math.cos(2 * math.pi * k / n), cy + ry * math.sin(2 * math.pi * k / n)) for k in range(n)]


def squeeze(ring, k):
    """The ring drawn a touch inside the true edge across its thin direction (v), so that the drawn part is never thicker than the solid."""
    lo, hi = {}, {}
    for x, y in ring:
        key = round(x, 3)
        lo[key], hi[key] = min(lo.get(key, y), y), max(hi.get(key, y), y)
    return [(x, (lo[round(x, 3)] + hi[round(x, 3)]) / 2 + (y - (lo[round(x, 3)] + hi[round(x, 3)]) / 2) * k) for x, y in ring]


def hull(points):
    pts = sorted(set(points))
    def half(seq):
        out = []
        for p in seq:
            while len(out) >= 2 and (out[-1][0] - out[-2][0]) * (p[1] - out[-2][1]) - (out[-1][1] - out[-2][1]) * (p[0] - out[-2][0]) <= 0:
                out.pop()
            out.append(p)
        return out
    lower, upper = half(pts), half(pts[::-1])
    return lower[:-1] + upper[:-1]


def thin(points):
    """The thinnest extent of a drawn ring as the validator measures it (metres in, millimetres out)."""
    pts = [(x / M, y / M) for x, y in points]
    xs, ys = [p[0] for p in pts], [p[1] for p in pts]
    best = min(max(xs) - min(xs), max(ys) - min(ys))
    h = hull(pts)
    for i in range(len(h)):
        a, b = h[i], h[(i + 1) % len(h)]
        l = math.hypot(b[0] - a[0], b[1] - a[1])
        if l > 0:
            best = min(best, max(abs((b[0] - a[0]) * (q[1] - a[1]) - (b[1] - a[1]) * (q[0] - a[0])) / l for q in h))
    return best


def span(solid, c, r, u, k, end):
    """The interval a prism spans at the joint (u = 0) or at the tip of the bone: k times the half extent about the centre, or (`fit`) the interval that lies inside every section beyond that end, so that a free pointed tip is not drawn square."""
    if end == "fit":
        beyond = [x for x in stations(solid) if (x["u"] >= u if u > 0 else x["u"] <= u)]
        if beyond:
            lo, hi = max(x[c] - 0.9 * x[r] for x in beyond), min(x[c] + 0.9 * x[r] for x in beyond)
            if hi > lo:
                return lo, hi
    return field(solid, c, u) - k * field(solid, r, u), field(solid, c, u) + k * field(solid, r, u)


def prism_w(solid, k, bone_length, ends=("k", "k")):
    """The lateral span of a part as the hint of its side outline, in world z of the rest pose."""
    base = REST_Z[solid["bone"]]
    a, b = span(solid, "wc", "rw", 0.0, k, ends[0]), span(solid, "wc", "rw", bone_length, k, ends[1])
    return {"kind": "extruded", "sweep": "w", "z0": round((base + a[0]) * M, 5), "z1": round((base + a[1]) * M, 5), "zTip": [round((base + b[0]) * M, 5), round((base + b[1]) * M, 5)]}


def prism_v(solid, k, bone_length, ends=("k", "k")):
    """The depth span of a part as the hint of its plan outline, along the bone normal."""
    a, b = span(solid, "vc", "rv", 0.0, k, ends[0]), span(solid, "vc", "rv", bone_length, k, ends[1])
    return {"kind": "extruded", "sweep": "v", "z0": round(a[0] * M, 5), "z1": round(a[1] * M, 5), "zTip": [round(b[0] * M, 5), round(b[1] * M, 5)]}


def revolved(bone, **extra):
    out = {"kind": "revolved", "axisBone": bone, "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}
    out.update(extra)
    return out


def bib_edge(bib, trunk):
    """Where the white shell leaves the red coat, per station: (u, lowest v of the shell, v of the edge, w of the edge)."""
    t = trunk["_st"]["stations"]
    tu = [s["u"] for s in t]
    out = []
    for s in stations(bib):
        if s["u"] < tu[0] or s["u"] > tu[-1]:
            continue
        T = {f: pchip(tu, [x[f] for x in t], s["u"]) for f in ("rw", "rv", "vc")}
        edge_w, edge_v = 0.0, s["vc"] - s["rv"]
        for i in range(1, 200):
            w = s["rw"] * i / 200
            if w >= T["rw"]:
                break
            vb = s["vc"] - s["rv"] * math.sqrt(1 - (w / s["rw"]) ** 2)
            vt = T["vc"] - T["rv"] * math.sqrt(1 - (w / T["rw"]) ** 2)
            if vb > vt:
                break
            edge_w, edge_v = w, vb
        if edge_w > 0.5:
            out.append((s["u"], s["vc"] - s["rv"], edge_v, edge_w))
    return out


def default_frames():
    off, root, ground, contacts = rig.poses(["sitting-eating"])["sitting-eating"]
    return chain(rig.BONES, off, (0.0, rig.root_y("sitting-eating", off, root, ground, contacts), 0.0))


def centre(solid, frames):
    """The middle of a solid in the world of the default pose (millimetres)."""
    f = frames[solid["bone"]]
    if solid["kind"] == "ellipsoid":
        p = [c / M for c in solid["transform"]["translation"]]
    else:
        o, A, X, Y = solid["_st"]["frame"]
        s = solid["_st"]["stations"]
        m = s[len(s) // 2]
        p = [o[i] + A[i] * m["u"] + Y[i] * m["vc"] + X[i] * m["wc"] for i in range(3)]
    return [f["o"][i] * 1000 + f["u"][i] * p[0] + f["v"][i] * p[1] + f["w"][i] * p[2] for i in range(3)]


REGION = {"rump": "fur-back", "chest": "fur-back", "belly-patch": "fur-belly", "belly-patch-rump": "fur-belly", "throat-patch": "fur-belly", "neck": "head-fur", "head": "head-fur", "nose": "nose", "near-eye": "head-fur", "far-eye": "head-fur", "near-ear": "ears", "far-ear": "ears", "tail-1": "tail-fur", "tail-2": "tail-fur"}
for _n in ("near", "far"):
    REGION.update({f"{_n}-arm": "arms-fur", f"{_n}-forearm": "arms-fur", f"{_n}-hand": "arms-fur", f"{_n}-fingers": "hands-skin", f"{_n}-thigh": "legs-fur", f"{_n}-shank": "legs-fur", f"{_n}-foot": "legs-fur", f"{_n}-sole": "feet-skin"})
SHAPE_ID = {"near-ear": "ear", "far-ear": "ear-far"}
TRUNK = {"belly-patch": "chest", "belly-patch-rump": "rump", "throat-patch": "neck"}
FREE_TIP = {"tail-2", "head", "near-foot", "far-foot"}
PRIORITY = ["tail-1", "tail-2", "rump", "chest", "belly-patch-rump", "belly-patch", "neck", "throat-patch", "head", "nose"]

MODES_50 = {"rump": "PC", "chest": "PC", "neck": "PC", "head": "PVC", "tail-1": "PVC", "tail-2": "PVC", "belly-patch": "BIB", "belly-patch-rump": "BIB", "throat-patch": "BIB", "near-ear": "EAR", "far-ear": "EAR", "near-eye": "DOT", "far-eye": "DOT", "nose": "DOT"}
MODES_100 = {"rump": "PC", "chest": "PC", "neck": "PC", "head": "PVC", "tail-1": "PVC", "tail-2": "PVC", "belly-patch": "BIB", "belly-patch-rump": "BIB", "near-ear": "EAR", "far-ear": "EAR"}
for _n in ("near", "far"):
    MODES_50.update({f"{_n}-arm": "LC", f"{_n}-forearm": "LC", f"{_n}-hand": "P", f"{_n}-fingers": "Pt", f"{_n}-thigh": "LC", f"{_n}-shank": "LC", f"{_n}-foot": "PV", f"{_n}-sole": "Pt"})
    MODES_100.update({f"{_n}-arm": "LC", f"{_n}-forearm": "LC", f"{_n}-hand": "Pt", f"{_n}-thigh": "LC", f"{_n}-shank": "LC", f"{_n}-foot": "PV", f"{_n}-sole": "Pt"})


def parts(solid, mode, tol, S):
    """The drawn shapes of one solid: (suffix, geometry, hint), the first is the side outline."""
    L = rig.LEN[solid["bone"] if solid["bone"] in rig.LEN else solid["bone"][:-2]] * 1000
    ring = lambda pts, t=tol: {"kind": "poly", "points": rnd(simplify(pts, t)), "closed": True}
    if mode == "DOT" and solid["id"] == "nose":
        c, r = [x / M for x in solid["transform"]["translation"]], [x / M for x in solid["radii"]]
        half = [(c[0] - r[0], 0.0)] + [(c[0] - r[0] * math.cos(math.pi * k / 8), 0.5 * (r[1] + r[2]) * math.sin(math.pi * k / 8)) for k in range(1, 8)] + [(c[0] + r[0], 0.0)]
        return [("", {"kind": "poly", "points": rnd(half), "closed": True}, revolved(solid["bone"]))]
    if mode == "DOT":
        c, r = [x / M for x in solid["transform"]["translation"]], [x / M for x in solid["radii"]]
        inner = c[2] - r[2] if c[2] > 0 else c[2] + r[2]
        return [("", {"kind": "poly", "points": rnd(ellipse_ring(c[0], c[1], r[0], r[1])), "closed": True}, {"kind": "extruded", "sweep": "w", "z0": round(min(inner, inner * 1.36) * M, 5), "z1": round(max(inner, inner * 1.36) * M, 5)})]
    if mode == "EAR":
        o, A, X, Y = solid["_st"]["frame"]
        st_ = refine(solid["_st"]["stations"], 2)
        mid_w = o[2] + A[2] * 16
        axis_u = o[0] + A[0] * 14
        prof = [(axis_u, o[1] - 1.0)] + [(axis_u + math.hypot(s["rw"] * X[2], s["rv"] * Y[2]), o[1] + A[1] * s["u"]) for s in st_] + [(axis_u, o[1] + A[1] * st_[-1]["u"] + 0.4)]
        hint = {"kind": "revolved", "axisBone": "head", "axis": "v", "axisX": round(axis_u * M / math.cos(math.radians(10)), 5), "axisZ": round(mid_w * M, 5), "mirror": True}
        return [("", ring(outline(solid, "uv"), min(tol, 0.5)), {"kind": "extruded", "sweep": "w", "z0": round((mid_w - 1.5) * M, 5), "z1": round((mid_w + 1.5) * M, 5)}), ("-core", ring(prof, min(tol, 0.4)), hint)]
    if mode == "BIB":
        e = bib_edge(solid, S[TRUNK[solid["id"]]])
        strip = [(u, top) for u, low, top, w in e] + [(u, low) for u, low, top, w in e][::-1]
        near = lambda u: min(e, key=lambda x: abs(x[0] - u))[3]
        return [("", ring(strip, min(tol, 0.4)), {"kind": "extruded", "sweep": "w", "z0": round(-near(0.0) * M, 5), "z1": round(near(0.0) * M, 5), "zTip": [round(-near(L) * M, 5), round(near(L) * M, 5)]})]
    if mode == "Pt":
        return [("", ring(squeeze(side_ring(solid), 0.95), min(tol, 0.25)), prism_w(solid, 1.0, L))]
    if mode == "P":
        return [("", ring(side_ring(solid)), prism_w(solid, 1.0, L))]
    axis_w = 4.5 * (1 if solid["id"].startswith("far") else -1) if "thigh" in solid["id"] else 0.0
    ends = ("k", "fit") if solid["id"] in FREE_TIP else (("k", "k") if "L" in mode or "tail" in solid["id"] else ("fit", "fit"))
    out = [("", ring(side_ring(solid)), prism_w(solid, 0.7 if "L" in mode else 0.5, L, ends))]
    if "V" in mode:
        out.append(("-plan", ring(plan_ring(solid)), prism_v(solid, 0.5, L, ends)))
    if "C" in mode:
        out.append(("-core", ring(core_ring(solid, axis_w)), revolved(solid["bone"], axisZ=round((REST_Z[solid["bone"]] * 0 + axis_w) * M, 5))))
    return out


def geometry(solids, modes, tol, regions_of=None):
    S = {s["id"]: s for s in solids}
    frames = default_frames()
    made = []
    for i, s in enumerate(solids):
        c = centre(s, frames)
        for k, (suffix, geo, hint) in enumerate(parts(s, modes[s["id"]], tol, S)):
            sid = SHAPE_ID.get(s["id"], s["id"]) + suffix
            region = REGION[s["id"]]
            shape = {"id": sid, "bone": s["bone"], "role": s["role"], "z": 0, "filled": True, "tone": s["tone"]}
            extra = (regions_of or {}).get(s["id"])
            shape.update({"regions": extra} if isinstance(extra, list) else {"region": extra or region})
            shape.update(geo)
            shape["solid"] = hint
            made.append((s["id"], i, k, suffix, c, shape))
    out = {}
    for view, key, top in (("elevation", lambda c: -c[2], ""), ("front", lambda c: c[0], "-core"), ("plan", lambda c: c[1], "-plan")):
        def rank(m):
            sid, i, k, suffix, c, shape = m
            prio = PRIORITY.index(sid) if sid in PRIORITY else len(PRIORITY)
            lead = 2 if suffix == top else (1 if suffix != "" and view != "elevation" else (2 if view == "elevation" and suffix == "" else 0))
            if view != "elevation" and sid.endswith("-eye"):
                return (-1e9, prio, i, lead)
            return (round(key(c), 1) if abs(key(c)) > 0.05 else 0.0, prio, i, lead)
        ordered = sorted(made, key=rank)
        shapes = []
        for n, m in enumerate(ordered):
            shape = dict(m[5])
            shape["z"] = round(1 + n * 0.05, 2)
            shapes.append(shape)
        out[view] = shapes
    return out


def limits(geo):
    """The thinnest drawn extent per bone in millimetres (what `volumes-feature` takes as the limit of the bone)."""
    out = {}
    for shape in geo["elevation"]:
        out[shape["bone"]] = min(out.get(shape["bone"], 1e9), thin(shape["points"]))
    return out


def representations(old, solids):
    import r3_red_squirrel_text as text
    out = dict(old)
    for k, modes, tol in (("50", MODES_50, 0.4), ("100", MODES_100, 0.8)):
        new = dict(old[k])
        new["geometry"] = geometry(solids[k], modes, tol, {"head": ["head-fur", "nose"], "near-hand": "hands-skin", "far-hand": "hands-skin"} if k == "100" else None)
        new["decisions"] = text.REP_DECISIONS[k]
        if k in text.PENS:
            new["pens"] = text.PENS[k]
        out[k] = new
    for k in ("200", "500"):
        new = dict(old[k])
        for field_ in ("reason", "limitingFeature"):
            texts = dict(new[field_])
            for lang in texts:
                for a, b in text.UNAVAILABLE_PATCH:
                    texts[lang] = texts[lang].replace(a, b)
            new[field_] = texts
        out[k] = new
    return out
