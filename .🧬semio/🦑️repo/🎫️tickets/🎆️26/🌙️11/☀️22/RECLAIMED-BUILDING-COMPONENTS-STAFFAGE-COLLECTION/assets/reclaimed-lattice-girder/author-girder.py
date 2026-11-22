"""🏗️ Authoritative generator of the reclaimed riveted lattice girder: states every hand-chosen number and writes 🔣️.json next to it."""
import json
import math
import os
import sys

import numpy as np
from shapely.geometry import MultiPolygon, Polygon, box as sbox
from shapely.geometry.polygon import orient
from shapely.ops import unary_union

HERE = os.path.dirname(os.path.abspath(__file__))


def sibling(name):
    """📎️ Loads a sibling author script by its file name."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(HERE, f"{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


texts = sibling("author-texts").texts
OUT = os.path.join(HERE, "🔣️.json")


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def r4(x):
    """✂️ Rounds to 0.1 mm."""
    return round(float(x) + 0.0, 4)


L = 10.24
X = [0.12 + k for k in range(11)]
MID = L / 2
Y0 = 0.12
W0 = 0.005
CH = dict(a=0.08, t=0.008, r=0.010, e=0.0226, A=12.3e-4, I=72.3e-8, kgm=9.66, gauge=0.045)
VE = dict(a=0.05, t=0.005, r=0.007, e=0.0140, A=4.80e-4, kgm=3.77, gauge=0.030)
DI = dict(a=0.06, t=0.006, r=0.008, e=0.0169, A=6.91e-4, kgm=5.42, gauge=0.035)
YB, YT = CH["e"], 1.0 - CH["e"]
WEB_LO, WEB_HI = 0.095, 0.905
CLEAR = 0.010
RIVET = dict(R=0.013, H=0.0105, pitch=0.055)
STEEL = 7850.0
TIMBER = 450.0
SLING_RHO = 600.0
BEARER_X = [X[1], X[9]]
BEARER = dict(b=0.12, h=0.12, l=1.20)
PROP_X = [MID - 2.52, MID + 2.52]
PHI = math.radians(55.0)
PROP_T = 0.08
SLING = dict(r=0.015, l=2.4, z=-0.045, u=[3.62, 6.62])
SLING_ANGLE = math.degrees(math.acos(1.5 / SLING["l"]))
CLEAT_NODES = [0, 2, 4, 8, 10]
STUMP_NODE = 6
LIFT_DY = 0.5
SEAT_DY = -0.104
BEARER_DX = 1.52


def quat_w(theta):
    """🌀️ Unit quaternion [x, y, z, w] turning by theta about the bone's w axis."""
    return [0.0, 0.0, round(math.sin(theta / 2), 6), round(math.cos(theta / 2), 6)]


def lprofile(a, t, r, heel, d1, d2, a2=None):
    """📐️ The section of an equal angle: heel corner, in-plane leg along d1, outstanding leg along d2, root fillet of radius r (three chords)."""
    h, d1, d2 = np.array(heel, float), np.array(d1, float), np.array(d2, float)
    c = h + (t + r) * d1 + (t + r) * d2
    arc = [c - r * (math.cos(q) * d2 + math.sin(q) * d1) for q in (0.0, math.pi / 8, math.pi / 4, 3 * math.pi / 8, math.pi / 2)]
    b = a if a2 is None else a2
    pts = [h, h + a * d1, h + a * d1 + t * d2, *arc, h + t * d1 + b * d2, h + b * d2]
    return [[r4(p[0]), r4(p[1])] for p in pts]


def lpieces(a, t, heel, d1, d2, a2=None):
    """🧩️ The two legs of an angle section as convex rectangles in profile coordinates."""
    h, d1, d2 = np.array(heel, float), np.array(d1, float), np.array(d2, float)
    b = a if a2 is None else a2
    return [[h, h + a * d1, h + a * d1 + t * d2, h + t * d2], [h, h + t * d1, h + t * d1 + b * d2, h + b * d2]]


def area2(poly):
    """📏️ Shoelace area of a 2D ring."""
    p = np.array(poly, float)
    return abs(float(np.dot(p[:, 0], np.roll(p[:, 1], -1)) - np.dot(p[:, 1], np.roll(p[:, 0], -1)))) / 2


class Part:
    """🧱️ One solid of one scale: its JSON, the convex pieces of its projection (bone-local points) and its steel or timber volume."""

    def __init__(self, sid, bone, material, tone, solid, pieces, volume, role="body", **flags):
        self.id, self.bone, self.material, self.tone = sid, bone, material, tone
        self.solid, self.pieces, self.volume, self.role, self.flags = solid, pieces, volume, role, flags

    def json(self):
        out = {"id": self.id, "kind": self.solid["kind"], "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        out.update({k: v for k, v in self.solid.items() if k != "kind"})
        out.update(self.flags)
        return out


ORIGIN = {}


def to_view(bone, pts, view):
    """🔭️ Bone-local rest points to the coordinates of a view: elevation bone-local (u, v), front world (z, y), plan world (x, z)."""
    o = ORIGIN[bone]
    p = np.array(pts, float)
    if view == "elevation":
        return p[:, [0, 1]]
    if view == "front":
        return np.stack([p[:, 2] + o[2], p[:, 1] + o[1]], 1)
    return np.stack([p[:, 0] + o[0], p[:, 2] + o[2]], 1)


def proj(part, view):
    """🪞️ The projection of a part in a view: the union of the convex hulls of its pieces."""
    hulls = []
    for piece in part.pieces:
        q = to_view(part.bone, piece, view)
        hulls.append(Polygon(q).convex_hull if len(q) > 2 else Polygon())
    return unary_union([h for h in hulls if h.area > 0])


def prism_yz(profile_vw, u0, u1, rot=0.0, at=(0.0, 0.0)):
    """🧊️ Points of a prism of a (v, w) profile from u0 to u1, turned by rot about w and moved to `at` (u, v)."""
    c, s = math.cos(rot), math.sin(rot)
    pts = []
    for uu in (u0, u1):
        for v, w in profile_vw:
            pts.append([at[0] + uu * c - v * s, at[1] + uu * s + v * c, w])
    return pts


def prism_zx(profile_wu, v0, v1):
    """🧊️ Points of a prism of a (w, u) profile from v0 to v1."""
    return [[u, v, w] for v in (v0, v1) for w, u in profile_wu]


def prism_xy(profile_uv, w0, w1):
    """🧊️ Points of a prism of a (u, v) profile from w0 to w1."""
    return [[u, v, w] for w in (w0, w1) for u, v in profile_uv]


def ell_pts(c, rad):
    """🔵️ Sample points of an ellipsoid."""
    out = []
    for i in range(12):
        a = 2 * math.pi * i / 12
        for b in (-1.2, -0.6, 0.0, 0.6, 1.2):
            out.append([c[0] + rad[0] * math.cos(a) * math.cos(b), c[1] + rad[1] * math.sin(a) * math.cos(b), c[2] + rad[2] * math.sin(b)])
    out += [[c[0], c[1], c[2] + rad[2]], [c[0], c[1], c[2] - rad[2]]]
    return out


def box_part(sid, bone, material, tone, size, centre, **flags):
    """📦️ A box part centred at `centre` (bone-local)."""
    sx, sy, sz = size
    cx, cy, cz = centre
    pts = [[cx + i * sx / 2, cy + j * sy / 2, cz + k * sz / 2] for i in (-1, 1) for j in (-1, 1) for k in (-1, 1)]
    solid = {"kind": "box", "size": [r4(sx), r4(sy), r4(sz)], "transform": {"translation": [r4(cx), r4(cy), r4(cz)]}}
    return Part(sid, bone, material, tone, solid, [pts], sx * sy * sz, **flags)


def rivet_runs(points):
    """🔁️ Groups rivet centres (same w) into arithmetic runs for `repeat`."""
    pts = sorted(points)
    runs, used = [], [False] * len(pts)
    for i in range(len(pts)):
        if used[i]:
            continue
        best = [i]
        for j in range(i + 1, len(pts)):
            if used[j]:
                continue
            step = np.subtract(pts[j], pts[i])
            seq = [i, j]
            nxt = np.add(pts[j], step)
            while True:
                k = next((m for m in range(len(pts)) if not used[m] and m not in seq and np.allclose(pts[m], nxt, atol=1e-6)), None)
                if k is None:
                    break
                seq.append(k)
                nxt = np.add(nxt, step)
            if len(seq) > len(best):
                best = seq
        for m in best:
            used[m] = True
        runs.append([pts[m] for m in best])
    return runs


def rivet_parts(prefix, bone, centres, w, sign):
    """🔩️ Snap rivet heads (DIN 124, d 16 mm: head 26 mm across, 10.5 mm high) as repeated ellipsoids centred on the plate face at lateral w."""
    parts = []
    for n, run in enumerate(rivet_runs([tuple(map(lambda x: round(x, 5), c)) for c in centres])):
        c0 = run[0]
        solid = {"kind": "ellipsoid", "radii": [RIVET["R"], RIVET["R"], RIVET["H"]], "transform": {"translation": [r4(c0[0]), r4(c0[1]), r4(w)]}}
        if len(run) > 1:
            step = np.subtract(run[1], run[0])
            solid["repeat"] = {"count": len(run), "translation": [r4(step[0]), r4(step[1]), 0.0]}
        pieces = [ell_pts([c[0], c[1], w], [RIVET["R"], RIVET["R"], RIVET["H"]]) for c in run]
        vol = len(run) * (2 / 3) * math.pi * RIVET["R"] ** 2 * RIVET["H"]
        parts.append(Part(f"{prefix}-{n:02d}", bone, "steel-painted", 0.32, solid, pieces, vol))
    return parts


def girder_layout():
    """🗺️ The lattice: verticals, trimmed diagonals, rivet centres per node and gusset outlines, all in girder-local (u, v)."""
    verts = []
    for k, x in enumerate(X):
        sgn = 1 if k <= 5 else -1
        ue = x - sgn * VE["e"]
        verts.append(dict(k=k, x=x, sgn=sgn, ue=ue, umin=min(ue, ue + sgn * VE["a"]), umax=max(ue, ue + sgn * VE["a"])))
    obstacles = [sbox(-1, -1, L + 1, 0.08 + CLEAR), sbox(-1, 0.92 - CLEAR, L + 1, 2)] + [sbox(vv["umin"] - CLEAR, -1, vv["umax"] + CLEAR, 2) for vv in verts]
    obstacle = unary_union(obstacles)
    diags = []
    for k in range(10):
        if k < 5:
            top, bot = (X[k], YT), (X[k + 1], YB)
        else:
            top, bot = (X[k + 1], YT), (X[k], YB)
        d = np.subtract(bot, top)
        full = float(np.hypot(*d))
        d = d / full
        n = np.array([-d[1], d[0]])
        sig = -1 if n[1] > 0 else 1
        h0 = -sig * DI["e"]
        c_lo, c_hi = min(h0, h0 + sig * DI["a"]), max(h0, h0 + sig * DI["a"])

        def clear(s):
            p = np.array(top) + s * d
            a, b = p + c_lo * n, p + c_hi * n
            seg = Polygon([a, b, b + 0.001 * d, a + 0.001 * d])
            return not seg.intersects(obstacle)

        s0 = 0.0
        while not clear(s0):
            s0 += 0.001
        s1 = full
        while not clear(s1 - 0.001):
            s1 -= 0.001
        diags.append(dict(k=k, top=top, bot=bot, d=d, n=n, sig=sig, h0=h0, c_lo=c_lo, c_hi=c_hi, s0=round(s0, 3), s1=round(s1, 3), theta=math.atan2(d[1], d[0])))
    node_rivets = {("b", k): [] for k in range(11)}
    node_rivets.update({("t", k): [] for k in range(11)})
    chord_rivets = {"b": [], "t": []}
    for k, x in enumerate(X):
        offs = [-0.12, -0.04, 0.04, 0.12]
        if k == 0:
            us = [0.06, 0.14, 0.22, 0.30]
        elif k == 10:
            us = [L - 0.06, L - 0.14, L - 0.22, L - 0.30]
        else:
            us = [x + o for o in offs]
        for side, v in (("b", CH["gauge"]), ("t", 1.0 - CH["gauge"])):
            for u in us:
                node_rivets[(side, k)].append((u, v))
                chord_rivets[side].append((u, v))
    stitch = [x + 0.5 for x in X[:-1]]
    vert_rivets = []
    for vv in verts:
        ug = vv["ue"] + vv["sgn"] * VE["gauge"]
        for side, vs in (("b", [WEB_LO + 0.03, WEB_LO + 0.03 + RIVET["pitch"]]), ("t", [WEB_HI - 0.03, WEB_HI - 0.03 - RIVET["pitch"]])):
            for v in vs:
                node_rivets[(side, vv["k"])].append((ug, v))
                vert_rivets.append((ug, v))
    diag_rivets = []
    for dd in diags:
        cg = dd["h0"] + dd["sig"] * DI["gauge"]
        ktop = dd["k"] if dd["k"] < 5 else dd["k"] + 1
        kbot = dd["k"] + 1 if dd["k"] < 5 else dd["k"]
        for s in (dd["s0"] + 0.035, dd["s0"] + 0.035 + RIVET["pitch"], dd["s0"] + 0.035 + 2 * RIVET["pitch"]):
            p = np.array(dd["top"]) + s * dd["d"] + cg * dd["n"]
            node_rivets[("t", ktop)].append((float(p[0]), float(p[1])))
            diag_rivets.append((float(p[0]), float(p[1])))
        for s in (dd["s1"] - 0.035, dd["s1"] - 0.035 - RIVET["pitch"], dd["s1"] - 0.035 - 2 * RIVET["pitch"]):
            p = np.array(dd["top"]) + s * dd["d"] + cg * dd["n"]
            node_rivets[("b", kbot)].append((float(p[0]), float(p[1])))
            diag_rivets.append((float(p[0]), float(p[1])))
    gussets = {}
    for (side, k), pts in node_rivets.items():
        hull = unary_union([Polygon([(u - 0.035, v - 0.035), (u + 0.035, v - 0.035), (u + 0.035, v + 0.035), (u - 0.035, v + 0.035)]) for u, v in pts]).convex_hull
        lo, hi = (0.005, 0.40) if side == "b" else (0.60, 0.995)
        clip = sbox(0.005, lo, L - 0.005, hi)
        g = hull.intersection(clip)
        g = g.simplify(0.002)
        ring = [(round(a, 3), round(b, 3)) for a, b in list(g.exterior.coords)[:-1]]
        gussets[(side, k)] = ring
    return dict(verts=verts, diags=diags, gussets=gussets, chord_rivets=chord_rivets, stitch=stitch, vert_rivets=vert_rivets, diag_rivets=diag_rivets)


LAY = girder_layout()


def chord_parts(scale):
    """🟫️ The two chords: at 1:50 four L 80 x 80 x 8 angles with root fillets, at 1:100 each pair merged to a T, at 1:200 a block."""
    parts = []
    for side, vtop in (("bottom", False), ("top", True)):
        if scale == 50:
            for wl, ws in (("near", -1), ("far", 1)):
                heel = (1.0 if vtop else 0.0, ws * W0)
                d1 = (-1.0, 0.0) if vtop else (1.0, 0.0)
                d2 = (0.0, float(ws))
                prof = lprofile(CH["a"], CH["t"], CH["r"], heel, d1, d2)
                pieces = [prism_yz(pc, 0.0, L) for pc in lpieces(CH["a"], CH["t"], heel, d1, d2)]
                solid = {"kind": "extrude", "profile": prof, "plane": "yz", "from": 0.0, "to": L}
                parts.append(Part(f"chord-{side}-{wl}", "girder", "steel-painted", 0.44, solid, pieces, CH["A"] * L))
        elif scale == 100:
            v0, v1, sg = (1.0, 1.0 - CH["a"], -1) if vtop else (0.0, CH["a"], 1)
            vf = v0 + sg * CH["t"]
            stem = W0 + CH["t"]
            prof = [[v0, -0.085], [v0, 0.085], [vf, 0.085], [vf, stem], [v1, stem], [v1, -stem], [vf, -stem], [vf, -0.085]]
            prof = [[r4(a), r4(b)] for a, b in prof]
            pieces = [prism_yz([[v0, -0.085], [v0, 0.085], [vf, 0.085], [vf, -0.085]], 0, L), prism_yz([[v0, -stem], [v0, stem], [v1, stem], [v1, -stem]], 0, L)]
            solid = {"kind": "extrude", "profile": prof, "plane": "yz", "from": 0.0, "to": L}
            parts.append(Part(f"chord-{side}", "girder", "steel-painted", 0.44, solid, pieces, 2 * CH["A"] * L))
        else:
            vc = 1.0 - CH["a"] / 2 if vtop else CH["a"] / 2
            parts.append(box_part(f"chord-{side}", "girder", "steel-painted", 0.44, (L, CH["a"], 0.17), (L / 2, vc, 0.0)))
    return parts


def web_parts(scale):
    """🪜️ Verticals 2 L 50 x 50 x 5 and diagonals 2 L 60 x 60 x 6 between the chords."""
    parts = []
    for vv in LAY["verts"]:
        k, sgn, ue = vv["k"], vv["sgn"], vv["ue"]
        if scale == 50:
            for wl, ws in (("near", -1), ("far", 1)):
                heel = (ws * W0, ue)
                d1, d2 = (0.0, float(sgn)), (float(ws), 0.0)
                prof = lprofile(VE["a"], VE["t"], VE["r"], heel, d1, d2)
                pieces = [prism_zx(pc, WEB_LO, WEB_HI) for pc in lpieces(VE["a"], VE["t"], heel, d1, d2)]
                solid = {"kind": "extrude", "profile": prof, "plane": "zx", "from": WEB_LO, "to": WEB_HI}
                parts.append(Part(f"vertical-{k:02d}-{wl}", "girder", "steel-painted", 0.5, solid, pieces, VE["A"] * (WEB_HI - WEB_LO)))
        elif scale == 100:
            st = W0 + VE["t"]
            ua, ub = ue, ue + sgn * VE["a"]
            uh = ue + sgn * VE["t"]
            prof = [[-st, ua], [-st, ub], [st, ub], [st, uh], [W0 + VE["a"], uh], [W0 + VE["a"], ua]]
            prof = prof + [[-(W0 + VE["a"]), ua]]
            prof = [[-(W0 + VE["a"]), ua], [-(W0 + VE["a"]), uh], [-st, uh], [-st, ub], [st, ub], [st, uh], [W0 + VE["a"], uh], [W0 + VE["a"], ua]]
            if sgn < 0:
                prof = prof[::-1]
            prof = [[r4(a), r4(b)] for a, b in prof]
            pieces = [prism_zx([[-st, ua], [-st, ub], [st, ub], [st, ua]], WEB_LO, WEB_HI), prism_zx([[-(W0 + VE["a"]), ua], [-(W0 + VE["a"]), uh], [W0 + VE["a"], uh], [W0 + VE["a"], ua]], WEB_LO, WEB_HI)]
            solid = {"kind": "extrude", "profile": prof, "plane": "zx", "from": WEB_LO, "to": WEB_HI}
            parts.append(Part(f"vertical-{k:02d}", "girder", "steel-painted", 0.5, solid, pieces, 2 * VE["A"] * (WEB_HI - WEB_LO)))
        else:
            uc = (vv["umin"] + vv["umax"]) / 2
            parts.append(box_part(f"vertical-{k:02d}", "girder", "steel-painted", 0.5, (VE["a"], WEB_HI - WEB_LO, 2 * (W0 + VE["a"])), (uc, (WEB_LO + WEB_HI) / 2, 0.0)))
    for dd in LAY["diags"]:
        k, sig, h0 = dd["k"], dd["sig"], dd["h0"]
        start = np.array(dd["top"]) + dd["s0"] * dd["d"]
        ln = dd["s1"] - dd["s0"]
        rot = dd["theta"]
        tr = {"rotation": quat_w(rot), "translation": [r4(start[0]), r4(start[1]), 0.0]}
        if scale == 50:
            for wl, ws in (("near", -1), ("far", 1)):
                heel = (h0, ws * W0)
                d1, d2 = (float(sig), 0.0), (0.0, float(ws))
                prof = lprofile(DI["a"], DI["t"], DI["r"], heel, d1, d2)
                pieces = [prism_yz(pc, 0.0, ln, rot, start) for pc in lpieces(DI["a"], DI["t"], heel, d1, d2)]
                solid = {"kind": "extrude", "profile": prof, "plane": "yz", "from": 0.0, "to": r4(ln), "transform": tr}
                parts.append(Part(f"diagonal-{k:02d}-{wl}", "girder", "steel-painted", 0.52, solid, pieces, DI["A"] * ln))
        elif scale == 100:
            st = W0 + DI["t"]
            ca, cb = h0, h0 + sig * DI["a"]
            ch = h0 + sig * DI["t"]
            prof = [[ca, -(W0 + DI["a"])], [ch, -(W0 + DI["a"])], [ch, -st], [cb, -st], [cb, st], [ch, st], [ch, W0 + DI["a"]], [ca, W0 + DI["a"]]]
            if sig < 0:
                prof = prof[::-1]
            prof = [[r4(a), r4(b)] for a, b in prof]
            pieces = [prism_yz([[ca, -st], [cb, -st], [cb, st], [ca, st]], 0, ln, rot, start), prism_yz([[ca, -(W0 + DI["a"])], [ch, -(W0 + DI["a"])], [ch, W0 + DI["a"]], [ca, W0 + DI["a"]]], 0, ln, rot, start)]
            solid = {"kind": "extrude", "profile": prof, "plane": "yz", "from": 0.0, "to": r4(ln), "transform": tr}
            parts.append(Part(f"diagonal-{k:02d}", "girder", "steel-painted", 0.52, solid, pieces, 2 * DI["A"] * ln))
        else:
            cm = (dd["c_lo"] + dd["c_hi"]) / 2
            prof = [[r4(dd["c_lo"]), -0.065], [r4(dd["c_hi"]), -0.065], [r4(dd["c_hi"]), 0.065], [r4(dd["c_lo"]), 0.065]]
            pieces = [prism_yz(prof, 0, ln, rot, start)]
            solid = {"kind": "extrude", "profile": prof, "plane": "yz", "from": 0.0, "to": r4(ln), "transform": tr}
            parts.append(Part(f"diagonal-{k:02d}", "girder", "steel-painted", 0.52, solid, pieces, 2 * DI["A"] * ln))
    return parts


def gusset_parts(scale):
    """🔶️ The 22 gusset plates, 10 mm, between the angle pairs; a declared sheet below the feature of 1:100 and 1:200."""
    parts = []
    for (side, k), ring in sorted(LAY["gussets"].items()):
        pieces = [prism_xy(ring, -W0, W0)]
        solid = {"kind": "extrude", "profile": [[r4(a), r4(b)] for a, b in ring], "plane": "xy", "from": -W0, "to": W0}
        flags = {"sheet": True} if scale >= 100 else {}
        parts.append(Part(f"gusset-{'bottom' if side == 'b' else 'top'}-{k:02d}", "girder", "steel-painted", 0.6, solid, pieces, area2(ring) * 2 * W0, **flags))
    return parts


def filler_parts():
    """🧷️ The 10 mm filler plates at the stitch rivets of the chords (1:50 only)."""
    parts = []
    for side, vc in (("bottom", 0.045), ("top", 0.955)):
        for i, u in enumerate(LAY["stitch"]):
            parts.append(box_part(f"filler-{side}-{i:02d}", "girder", "steel-painted", 0.6, (0.06, 0.06, 2 * W0), (u, vc, 0.0)))
    return parts


def rivets_50():
    """🔩️ Every snap rivet head of 1:50, both faces."""
    parts = []
    cr = LAY["chord_rivets"]["b"] + LAY["chord_rivets"]["t"] + [(u, 0.045) for u in LAY["stitch"]] + [(u, 0.955) for u in LAY["stitch"]]
    wc = W0 + CH["t"]
    parts += rivet_parts("rivets-chord-near", "girder", cr, -wc, -1) + rivet_parts("rivets-chord-far", "girder", cr, wc, 1)
    wv = W0 + VE["t"]
    parts += rivet_parts("rivets-vertical-near", "girder", LAY["vert_rivets"], -wv, -1) + rivet_parts("rivets-vertical-far", "girder", LAY["vert_rivets"], wv, 1)
    wd = W0 + DI["t"]
    parts += rivet_parts("rivets-diagonal-near", "girder", LAY["diag_rivets"], -wd, -1) + rivet_parts("rivets-diagonal-far", "girder", LAY["diag_rivets"], wd, 1)
    return parts


def circle(c, r, n=16):
    """⭕️ A polygonal circle (counter-clockwise)."""
    return [[r4(c[0] + r * math.cos(2 * math.pi * i / n)), r4(c[1] + r * math.sin(2 * math.pi * i / n))] for i in range(n)]


def slot(c, r, half, n=8):
    """🕳️ A slotted hole along the first axis."""
    pts = []
    for i in range(n + 1):
        a = -math.pi / 2 + math.pi * i / n
        pts.append([r4(c[0] + half + r * math.cos(a)), r4(c[1] + r * math.sin(a))])
    for i in range(n + 1):
        a = math.pi / 2 + math.pi * i / n
        pts.append([r4(c[0] - half + r * math.cos(a)), r4(c[1] + r * math.sin(a))])
    return pts


PLATE = dict(l=0.24, b=0.26, t=0.016, hole=0.011, hz=0.105)


def plate_parts(scale):
    """🟥️ The two riveted bearing plates 240 x 260 x 16 mm under the chord ends with their anchor holes (round at the fixed end, slotted at the sliding end); rusted bare steel."""
    parts = []
    for name, u0 in (("left", 0.0), ("right", L - PLATE["l"])):
        uc = u0 + PLATE["l"] / 2
        outer = [[-PLATE["b"] / 2, u0], [PLATE["b"] / 2, u0], [PLATE["b"] / 2, u0 + PLATE["l"]], [-PLATE["b"] / 2, u0 + PLATE["l"]]]
        outer = [[r4(a), r4(b)] for a, b in outer]
        if scale <= 100:
            if name == "left":
                holes = [circle((s * PLATE["hz"], uc), PLATE["hole"]) for s in (-1, 1)]
            else:
                holes = [[[p[1], p[0]] for p in slot((uc, s * PLATE["hz"]), PLATE["hole"], 0.025)] for s in (-1, 1)]
            solid = {"kind": "extrude", "profile": outer, "holes": holes, "plane": "zx", "from": -PLATE["t"], "to": 0.0}
            vol = PLATE["l"] * PLATE["b"] * PLATE["t"] - sum(area2(h) for h in holes) * PLATE["t"]
            parts.append(Part(f"bearing-plate-{name}", "girder", "steel-bare", 0.26, solid, [prism_zx(outer, -PLATE["t"], 0.0)], vol))
        else:
            parts.append(box_part(f"bearing-plate-{name}", "girder", "steel-bare", 0.26, (PLATE["l"], 0.030, PLATE["b"]), (uc, 0.014 - 0.015, 0.0)))
    return parts


def plate_rivets():
    """🔩️ Four countersunk-under, snap-over rivets per bearing plate through the chord flanges."""
    parts = []
    for name, uc in (("left", PLATE["l"] / 2), ("right", L - PLATE["l"] / 2)):
        cs = [(uc + du, CH["t"], s * 0.05) for du in (-0.06, 0.06) for s in (-1, 1)]
        for i, (u, v, w) in enumerate(cs):
            solid = {"kind": "ellipsoid", "radii": [RIVET["R"], RIVET["H"], RIVET["R"]], "transform": {"translation": [r4(u), r4(v), r4(w)]}}
            parts.append(Part(f"rivet-plate-{name}-{i}", "girder", "steel-painted", 0.32, solid, [ell_pts([u, v, w], [RIVET["R"], RIVET["H"], RIVET["R"]])], (2 / 3) * math.pi * RIVET["R"] ** 2 * RIVET["H"]))
    return parts


def cleat_parts(scale):
    """🪝️ Purlin cleats L 80 x 80 x 8, 85 mm wide, on the left half of the top chord at every second node; the one at node 6 was flame-cut to a stump."""
    parts = []
    for k in CLEAT_NODES:
        x = X[k]
        if scale == 50:
            prof = [[x - 0.004, 1.0], [x + 0.076, 1.0], [x + 0.076, 1.008], [x + 0.004, 1.008], [x + 0.004, 1.08], [x - 0.004, 1.08]]
            prof = [[r4(a), r4(b)] for a, b in prof]
            hz = [circle((1.044, 0.0425 + s * 0.022), 0.009, 12) for s in (-1, 1)]
            flat = Part(f"cleat-{k:02d}-seat", "girder", "steel-painted", 0.38, {"kind": "box", "size": [0.072, 0.008, 0.085], "transform": {"translation": [r4(x + 0.04), 1.004, 0.0425]}}, [prism_xy([[x + 0.004, 1.0], [x + 0.076, 1.0], [x + 0.076, 1.008], [x + 0.004, 1.008]], 0.0, 0.085)], 0.072 * 0.008 * 0.085)
            stand_prof = [[1.0, 0.0], [1.08, 0.0], [1.08, 0.085], [1.0, 0.085]]
            stand = Part(f"cleat-{k:02d}-leg", "girder", "steel-painted", 0.38, {"kind": "extrude", "profile": stand_prof, "holes": hz, "plane": "yz", "from": r4(x - 0.004), "to": r4(x + 0.004)}, [prism_yz(stand_prof, x - 0.004, x + 0.004)], 0.008 * (0.08 * 0.085 - sum(area2(h) for h in hz)))
            parts += [flat, stand]
        elif scale == 100:
            prof = [[x - 0.004, 1.0], [x + 0.076, 1.0], [x + 0.076, 1.008], [x + 0.004, 1.008], [x + 0.004, 1.08], [x - 0.004, 1.08]]
            prof = [[r4(a), r4(b)] for a, b in prof]
            parts.append(Part(f"cleat-{k:02d}", "girder", "steel-painted", 0.38, {"kind": "extrude", "profile": prof, "plane": "xy", "from": 0.0, "to": 0.085}, [prism_xy([[x - 0.004, 1.0], [x + 0.076, 1.0], [x + 0.076, 1.008], [x - 0.004, 1.008]], 0, 0.085), prism_xy([[x - 0.004, 1.0], [x + 0.004, 1.0], [x + 0.004, 1.08], [x - 0.004, 1.08]], 0, 0.085)], 0.0))
        elif scale == 200:
            parts.append(box_part(f"cleat-{k:02d}", "girder", "steel-painted", 0.38, (0.08, 0.08, 0.085), (x + 0.036, 1.04, 0.0425)))
        else:
            parts.append(box_part(f"cleat-{k:02d}", "girder", "steel-painted", 0.38, (0.08, 0.08, 0.085), (x + 0.036, 1.04, 0.0425)))
    if scale == 50:
        x = X[STUMP_NODE]
        parts.append(box_part(f"cleat-{STUMP_NODE:02d}-stump", "girder", "steel-cut", 0.3, (0.08, 0.008, 0.085), (x + 0.036, 1.004, 0.0425)))
    return parts


STUB_LEG = 0.04
BRACE = dict(u0=X[5] - 0.20, u1=X[5] + 0.20, w0=W0 + CH["t"], w1=0.223, v0=CH["t"], t=0.010)


def brace_parts(scale):
    """✂️ The lateral bracing gusset at bottom node 5 with the flame-cut stub of its L 60 x 6 bracing angle and two empty rivet holes."""
    b = BRACE
    outer = [[b["w0"], b["u0"]], [b["w1"], b["u0"] + 0.06], [b["w1"], b["u1"] - 0.06], [b["w0"], b["u1"]]]
    outer = [[r4(p), r4(q)] for p, q in outer]
    parts = []
    if scale <= 100:
        holes = [circle((0.185, X[5] - 0.07), 0.0085, 12), circle((0.185, X[5] - 0.0), 0.0085, 12)] if scale == 50 else []
        solid = {"kind": "extrude", "profile": outer, "plane": "zx", "from": b["v0"], "to": b["v0"] + b["t"]}
        if holes:
            solid["holes"] = holes
        flags = {"sheet": True} if scale >= 100 else {}
        parts.append(Part("bracing-gusset", "girder", "steel-painted", 0.5, solid, [prism_zx(outer, b["v0"], b["v0"] + b["t"])], area2(outer) * b["t"], **flags))
    else:
        parts.append(Part("bracing-gusset", "girder", "steel-painted", 0.5, {"kind": "extrude", "profile": outer, "plane": "zx", "from": b["v0"], "to": b["v0"] + b["t"]}, [prism_zx(outer, b["v0"], b["v0"] + b["t"])], area2(outer) * b["t"], sheet=True))
    if scale == 50:
        c, s = math.cos(math.radians(40)), math.sin(math.radians(40))
        base = np.array([0.06, X[5] + 0.05])
        dirn = np.array([c, s])
        nrm = np.array([-s, c])
        ln = 0.15
        v1 = b["v0"] + b["t"]
        a = -math.radians(50)
        section = lprofile(DI["a"], DI["t"], DI["r"], (v1, 0.0), (0.0, 1.0), (1.0, 0.0), STUB_LEG)
        tr = {"rotation": [0.0, round(math.sin(a / 2), 6), 0.0, round(math.cos(a / 2), 6)], "translation": [r4(base[1]), 0.0, r4(base[0])]}
        corners = []
        for pc in lpieces(DI["a"], DI["t"], (v1, 0.0), (0.0, 1.0), (1.0, 0.0), STUB_LEG):
            for along in (0.0, ln):
                for vv, ww in pc:
                    q = base + along * dirn - ww * nrm
                    corners.append([q[1], vv, q[0]])
        vol = (DI["A"]) * ln
        parts.append(Part("bracing-stub", "girder", "steel-cut", 0.36, {"kind": "extrude", "profile": section, "plane": "yz", "from": 0.0, "to": ln, "transform": tr}, [corners], vol))
        rc = base + 0.035 * dirn - 0.035 * nrm
        for i, sft in enumerate((0.0, 0.055)):
            p = rc + sft * dirn
            parts.append(Part(f"rivet-bracing-{i}", "girder", "steel-painted", 0.32, {"kind": "ellipsoid", "radii": [RIVET["R"], RIVET["H"], RIVET["R"]], "transform": {"translation": [r4(p[1]), r4(v1 + DI["t"]), r4(p[0])]}}, [ell_pts([p[1], v1 + DI["t"], p[0]], [RIVET["R"], RIVET["H"], RIVET["R"]])], (2 / 3) * math.pi * RIVET["R"] ** 2 * RIVET["H"]))
    return parts


LOOP = dict(du=0.025, w=0.10, vlo=0.90, vhi=1.03, hw=0.0855, hlo=0.92, hhi=1.0)


def loop_parts(scale):
    """🎗️ The choke loops of the two round slings around the top chord at the lifting points."""
    parts = []
    for name, u in zip(("a", "b"), SLING["u"]):
        outer = [[LOOP["vlo"], -LOOP["w"]], [LOOP["vlo"], LOOP["w"]], [LOOP["vhi"], LOOP["w"]], [LOOP["vhi"], -LOOP["w"]]]
        hole = [[LOOP["hlo"], -LOOP["hw"]], [LOOP["hlo"], LOOP["hw"]], [LOOP["hhi"], LOOP["hw"]], [LOOP["hhi"], -LOOP["hw"]]]
        solid = {"kind": "extrude", "profile": outer, "holes": [hole], "plane": "yz", "from": r4(u - LOOP["du"]), "to": r4(u + LOOP["du"])}
        vol = (area2(outer) - area2(hole)) * 2 * LOOP["du"]
        pieces = [prism_yz([[LOOP["vlo"], -LOOP["w"]], [LOOP["vlo"], LOOP["w"]], [LOOP["hlo"], LOOP["w"]], [LOOP["hlo"], -LOOP["w"]]], u - LOOP["du"], u + LOOP["du"]),
                  prism_yz([[LOOP["hhi"], -LOOP["w"]], [LOOP["hhi"], LOOP["w"]], [LOOP["vhi"], LOOP["w"]], [LOOP["vhi"], -LOOP["w"]]], u - LOOP["du"], u + LOOP["du"]),
                  prism_yz([[LOOP["vlo"], LOOP["hw"]], [LOOP["vlo"], LOOP["w"]], [LOOP["vhi"], LOOP["w"]], [LOOP["vhi"], LOOP["hw"]]], u - LOOP["du"], u + LOOP["du"]),
                  prism_yz([[LOOP["vlo"], -LOOP["w"]], [LOOP["vlo"], -LOOP["hw"]], [LOOP["vhi"], -LOOP["hw"]], [LOOP["vhi"], -LOOP["w"]]], u - LOOP["du"], u + LOOP["du"])]
        parts.append(Part(f"choke-loop-{name}", "girder", "polyester", 0.7, solid, pieces, vol))
    return parts


def envelope_500():
    """🔲️ The lattice of 1:500 as its envelope prism (typified, `envelope: true`)."""
    p = box_part("lattice-envelope", "girder", "steel-painted", 0.62, (L, 1.0, 0.17), (L / 2, 0.5, 0.0), envelope=True)
    return [p]


def bearer_parts(scale):
    """🪵️ The two squared-timber bearers 120 x 120 x 1200 mm the girder stands on."""
    return [box_part(f"bearer-{n}", f"bearer-{n}", "softwood-weathered", 0.58, (BEARER["b"], BEARER["h"], BEARER["l"]), (0.0, BEARER["h"] / 2, 0.0)) for n in ("a", "b")]


def prop_profile(side):
    """📐️ The (v, w) profile of a prop from its outer foot corner: bottom sawn flat, head cut plumb against the top chord flange tip."""
    tan, cos = math.tan(PHI), math.cos(PHI)
    ytop = Y0 + 1.0
    zf_out = 0.085 + ytop / tan
    ylow = ytop - PROP_T / cos
    zf_in = 0.085 + ylow / tan
    pts = [(0.0, 0.0), (ytop, 0.085 - zf_out), (ylow, 0.085 - zf_out), (0.0, zf_in - zf_out)]
    if side < 0:
        pts = [(v, -w) for v, w in pts][::-1]
    return [[r4(v), r4(w)] for v, w in pts], zf_out


def prop_parts(scale):
    """🪵️ The four inclined squared-timber props 80 x 80 mm that brace the girder on edge."""
    parts = []
    for name, side in PROPS:
        prof, zf = prop_profile(side)
        pieces = [prism_yz(prof, -PROP_T / 2, PROP_T / 2)]
        solid = {"kind": "extrude", "profile": prof, "plane": "yz", "from": -PROP_T / 2, "to": PROP_T / 2}
        parts.append(Part(f"{name}-bar", name, "softwood-weathered", 0.66, solid, pieces, area2(prof) * PROP_T))
    return parts


def sling_parts(scale):
    """🎗️ The two violet 1 t round slings (EN 1492-2) laid along the top chord from their choke points."""
    parts = []
    for name in ("a", "b"):
        solid = {"kind": "capsule", "p0": [0.0, 0.0, 0.0], "p1": [SLING["l"], 0.0, 0.0], "r0": SLING["r"], "r1": SLING["r"]}
        pts = ell_pts([0, 0, 0], [SLING["r"]] * 3) + ell_pts([SLING["l"], 0, 0], [SLING["r"]] * 3)
        parts.append(Part(f"sling-{name}-leg", f"sling-{name}", "polyester", 0.72, solid, [pts], math.pi * SLING["r"] ** 2 * SLING["l"] + 4 / 3 * math.pi * SLING["r"] ** 3, slender=True))
    return parts


PROPS = [("prop-1l", 1), ("prop-1r", -1), ("prop-2l", 1), ("prop-2r", -1)]


def body(scale):
    """🧍️ The body list of one scale."""
    ps = []
    if scale == 500:
        ps += envelope_500() + cleat_parts(500)
    else:
        ps += chord_parts(scale) + web_parts(scale) + gusset_parts(scale) + plate_parts(scale) + cleat_parts(scale) + brace_parts(scale) + loop_parts(scale)
        if scale == 50:
            ps += filler_parts() + rivets_50() + plate_rivets()
    ps += bearer_parts(scale) + prop_parts(scale) + sling_parts(scale)
    return ps


def build_origins():
    """📍️ Rest origins of the bones in world metres."""
    ORIGIN["yard"] = (0.0, 0.0, 0.0)
    ORIGIN["girder"] = (0.0, Y0, 0.0)
    for n, x in zip(("a", "b"), BEARER_X):
        ORIGIN[f"bearer-{n}"] = (x, 0.0, 0.0)
    for (name, side), x in zip(PROPS, [PROP_X[0], PROP_X[0], PROP_X[1], PROP_X[1]]):
        _, zf = prop_profile(side)
        ORIGIN[name] = (x, 0.0, side * zf)
    for n, u in zip(("a", "b"), SLING["u"]):
        ORIGIN[f"sling-{n}"] = (u, Y0 + 1.0 + SLING["r"], SLING["z"])


build_origins()


def holeless(geom):
    """✂️ Cuts a polygon with holes by a vertical line through every hole, so each piece is a plain ring (the drawings fill path holes)."""
    from shapely.geometry import LineString
    from shapely.ops import split
    pieces = list(getattr(geom, "geoms", [geom]))
    out = []
    while pieces:
        q = pieces.pop()
        if not q.interiors:
            out.append(q)
            continue
        hole = Polygon(q.interiors[0])
        x = hole.centroid.x
        b = q.bounds
        cut = split(q, LineString([(x, b[1] - 1), (x, b[3] + 1)]))
        pieces += [g for g in cut.geoms if g.area > 1e-9]
    return sorted(out, key=lambda g: (g.bounds[0], g.bounds[1]))


def multi_shape(sid, *args, **kw):
    """✏️ One shape per hole-free piece of a projection."""
    geom = args[5]
    parts = holeless(geom) if isinstance(geom, (Polygon, MultiPolygon)) else [geom]
    if len(parts) == 1:
        return [poly_shape(sid, *args[:5], parts[0], **kw)]
    return [poly_shape(f"{sid}-{i:02d}", *args[:5], g, **kw) for i, g in enumerate(parts)]


def poly_shape(sid, bone, role, z, filled, tone, geom, region=None, regions=None, material=None, pen=None, solid=None, hatch=None):
    """✏️ A shape from a shapely polygon or point list."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if regions:
        s["regions"] = regions
    elif region:
        s["region"] = region
    if material:
        s["material"] = material
    if pen is not None:
        s["penMm"] = pen
    if hatch:
        s["hatch"] = hatch
    if isinstance(geom, (Polygon, MultiPolygon)):
        polys = [orient(q, 1.0) for q in ([geom] if isinstance(geom, Polygon) else list(geom.geoms))]
        if len(polys) == 1 and not polys[0].interiors:
            s.update({"kind": "poly", "points": [[r4(a), r4(b)] for a, b in list(polys[0].exterior.coords)[:-1]], "closed": True})
        else:
            d = []
            for p in polys:
                for ring in [p.exterior, *p.interiors]:
                    c = list(ring.coords)[:-1]
                    d.append("M " + " L ".join(f"{r4(a)} {r4(b)}" for a, b in c) + " Z")
            s.update({"kind": "path", "d": " ".join(d)})
    else:
        s.update({"kind": "poly", "points": [[r4(a), r4(b)] for a, b in geom], "closed": filled or (len(geom) > 2 and region is None and False)})
        if not filled:
            s["closed"] = False
    if solid:
        s["solid"] = solid
    return s


def line(sid, bone, role, z, tone, pts, region, pen=None):
    """➖️ An open stroke."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": False, "tone": tone, "region": region, "kind": "poly", "points": [[r4(a), r4(b)] for a, b in pts], "closed": False}
    if pen is not None:
        s["penMm"] = pen
    return s


def region_of(part):
    """🌡️ The thermal region of a part."""
    if part.bone.startswith("bearer"):
        return "bearers"
    if part.bone.startswith("prop"):
        return "props"
    if part.material == "polyester":
        return "slings"
    if part.material == "steel-bare":
        return "bearing-steel"
    return "lattice-paint"


def clean(g, tol=0.0004):
    """🧽️ A projection without slivers."""
    return g.buffer(0).simplify(tol)


def prop_solid_shape(sid, name, z, tone, side):
    """🪵️ A prop drawn by its (v, w) profile swept along u, so every view follows its tilt."""
    prof, _ = prop_profile(side)
    return {"id": sid, "bone": name, "role": "body", "z": z, "filled": True, "tone": tone, "region": "props", "material": "softwood-weathered", "kind": "poly", "points": prof, "closed": True, "solid": {"kind": "extruded", "sweep": "u", "z0": -PROP_T / 2, "z1": PROP_T / 2}}


def sling_solid_shape(sid, name, z, tone, r):
    """🎗️ A round sling drawn by its side profile swept laterally (front and plan follow its swing)."""
    return {"id": sid, "bone": name, "role": "body", "z": z, "filled": True, "tone": tone, "region": "slings", "material": "polyester", "kind": "poly", "points": [[0.0, -r], [SLING["l"], -r], [SLING["l"], r], [0.0, r]], "closed": True,
            "solid": {"kind": "extruded", "sweep": "w", "z0": r4(SLING["z"] - SLING["r"]), "z1": r4(SLING["z"] + SLING["r"])}}


def drawings(scale, parts):
    """🖋️ The three views of one scale, drawn from the same numbers as the solids."""
    by = {p.id: p for p in parts}
    girder = [p for p in parts if p.bone == "girder"]
    E, F, P = [], [], []
    pen_web = {50: None, 100: None, 200: 0.3, 500: None}[scale]
    if scale in (50, 100, 200):
        for p in girder:
            if p.flags.get("envelope"):
                continue
            g = clean(proj(p, "elevation"))
            if g.is_empty:
                continue
            reg = region_of(p)
            if p.id.startswith("gusset") or p.id.startswith("bracing-gusset"):
                E.append(poly_shape(p.id, "girder", "body", 1, True, 0.66 if scale == 50 else 0.62, g, region=reg, material=p.material))
            elif p.id.startswith("chord"):
                if p.id.endswith("far"):
                    continue
                E.append(poly_shape(p.id.replace("-near", ""), "girder", "body", 3, True, 0.42, g, region=reg, material=p.material))
            elif p.id.startswith(("vertical", "diagonal")):
                if p.id.endswith("far"):
                    continue
                if scale == 200:
                    c = np.array(g.minimum_rotated_rectangle.exterior.coords)[:-1]
                    e01, e12 = np.linalg.norm(c[1] - c[0]), np.linalg.norm(c[2] - c[1])
                    a, b = ((c[0] + c[3]) / 2, (c[1] + c[2]) / 2) if e01 > e12 else ((c[0] + c[1]) / 2, (c[2] + c[3]) / 2)
                    E.append(line(p.id, "girder", "body", 2, 0.5, [a, b], reg, pen=pen_web))
                else:
                    E.append(poly_shape(p.id.replace("-near", ""), "girder", "body", 2, True, 0.5, g, region=reg, material=p.material))
            elif p.id.startswith("rivet") and "far" in p.id:
                continue
            elif p.id.startswith("rivet"):
                for i, piece in enumerate(p.pieces):
                    q = to_view("girder", piece, "elevation")
                    cx, cy = q[:, 0].mean(), q[:, 1].mean()
                    if p.id.startswith("rivet-plate") or p.id.startswith("rivet-bracing"):
                        continue
                    E.append({"id": f"{p.id}-{i}", "bone": "girder", "role": "detail", "z": 5, "filled": True, "tone": 0.3, "region": reg, "material": p.material, "kind": "ellipse", "cx": r4(cx), "cy": r4(cy), "rx": RIVET["R"], "ry": RIVET["R"]})
            elif p.id.startswith(("filler", "bracing-stub")):
                continue
            elif p.id.startswith("choke"):
                E.append(poly_shape(p.id, "girder", "body", 6, True, 0.72, g, region="slings", material="polyester"))
            else:
                E.append(poly_shape(p.id, "girder", "body", 4, True, 0.36 if reg == "lattice-paint" else 0.26, g, region=reg, material=p.material))
        if scale in (50, 100):
            for side, v in (("bottom", CH["t"]), ("top", 1.0 - CH["t"])):
                E.append(line(f"flange-edge-{side}", "girder", "detail", 3.5, 0.3, [(0.0, v), (L, v)], "lattice-paint"))
            E.append(line("flame-cut-end", "girder", "texture", 3.6, 0.2, [(L - 0.003, 1.0), (L + 0.002, 0.975), (L - 0.002, 0.95), (L + 0.002, 0.93)], "lattice-paint"))
    else:
        env = by["lattice-envelope"]
        g = clean(unary_union([proj(env, "elevation")] + [proj(p, "elevation") for p in girder if p.id.startswith("cleat")]))
        E.append(poly_shape("lattice-envelope", "girder", "body", 1, True, 0.7, g, regions=["lattice-paint", "bearing-steel"], material="steel-painted"))
        E.append(line("chord-line-bottom", "girder", "detail", 2, 0.3, [(0, 0.04), (L, 0.04)], "lattice-paint", pen=0.18))
        E.append(line("chord-line-top", "girder", "detail", 2, 0.3, [(0, 0.96), (L, 0.96)], "lattice-paint", pen=0.18))
        for dd in LAY["diags"]:
            E.append(line(f"diagonal-line-{dd['k']:02d}", "girder", "detail", 2, 0.3, [dd["top"], dd["bot"]], "lattice-paint", pen=0.13))
        for vv in LAY["verts"]:
            E.append(line(f"vertical-line-{vv['k']:02d}", "girder", "detail", 2, 0.3, [(vv["x"], 0.04), (vv["x"], 0.96)], "lattice-paint", pen=0.13))
    for n in ("a", "b"):
        p = by[f"bearer-{n}"]
        E.append(poly_shape(f"bearer-{n}", f"bearer-{n}", "body", 7, True, 0.56, clean(proj(p, "elevation")), region="bearers", material="softwood-weathered"))
    for name, side in PROPS:
        E.append(prop_solid_shape(f"{name}", name, 8 if side < 0 else 0.5, 0.64, side))
    for n in ("a", "b"):
        if scale == 500:
            E.append(line(f"sling-{n}", f"sling-{n}", "body", 6.5, 0.74, [(0.0, 0.0), (SLING["l"], 0.0)], "slings", pen=0.15))
        else:
            E.append({"id": f"sling-{n}", "bone": f"sling-{n}", "role": "body", "z": 6.5, "filled": True, "tone": 0.74, "region": "slings", "material": "polyester", "kind": "capsule", "x0": 0.0, "y0": 0.0, "x1": SLING["l"], "y1": 0.0, "r0": SLING["r"], "r1": SLING["r"]})

    gparts = [p for p in girder]
    gf = clean(unary_union([proj(p, "front") for p in gparts]))
    gf = unary_union([Polygon(q.exterior, [h for h in q.interiors if Polygon(h).area > 0.002]) for q in getattr(gf, "geoms", [gf])])
    F.extend(multi_shape("girder-end", "girder", "body", 2, True, 0.46, gf, regions=["lattice-paint", "bearing-steel", "slings"], material="steel-painted", pen=0.13 if scale == 50 else None))
    if scale in (50, 100):
        for side, y in (("bottom", Y0 + CH["t"]), ("top", Y0 + 1.0 - CH["t"])):
            F.append(line(f"flange-edge-{side}", "girder", "detail", 3, 0.3, [(-0.085, y), (0.085, y)], "lattice-paint"))
        F.append(line("gusset-centre", "girder", "detail", 3, 0.3, [(0.0, Y0 + 0.01), (0.0, Y0 + 0.99)], "lattice-paint"))
    for n in ("a", "b"):
        p = by[f"bearer-{n}"]
        F.append(poly_shape(f"bearer-{n}", f"bearer-{n}", "body", 1 if n == "a" else 1.2, True, 0.56, clean(proj(p, "front")), region="bearers", material="softwood-weathered"))
    for name, side in PROPS:
        F.append(prop_solid_shape(f"{name}", name, 3 if name.startswith("prop-2") else 2.5, 0.64, side))
    if scale < 500:
        for n in ("a", "b"):
            F.append(sling_solid_shape(f"sling-{n}", f"sling-{n}", 4, 0.74, SLING["r"]))

    top = clean(unary_union([proj(p, "plan") for p in gparts]))
    P.extend(multi_shape("girder-top", "girder", "body", 2, True, 0.46, top, regions=["lattice-paint", "bearing-steel", "slings"], material="steel-painted"))
    if scale in (50, 100):
        for k in CLEAT_NODES:
            P.append(line(f"cleat-leg-{k:02d}", "girder", "detail", 3, 0.3, [(X[k] + 0.004, 0.0), (X[k] + 0.004, 0.085)], "lattice-paint"))
    for n in ("a", "b"):
        p = by[f"bearer-{n}"]
        P.append(poly_shape(f"bearer-{n}", f"bearer-{n}", "body", 1, True, 0.56, clean(proj(p, "plan")), region="bearers", material="softwood-weathered"))
    for name, side in PROPS:
        P.append(prop_solid_shape(f"{name}", name, 3, 0.64, side))
    if scale < 500:
        for n in ("a", "b"):
            P.append(sling_solid_shape(f"sling-{n}", f"sling-{n}", 4, 0.74, SLING["r"]))
    return E, F, P


def structure_list(scale, parts):
    """🏗️ The structural idealisation: the lattice members, gusset joints, bearers, props, slings and their choke loops."""
    out = []
    for p in parts:
        if p.bone == "girder":
            if p.id.startswith(("chord", "vertical", "diagonal")):
                out.append({"id": f"st-{p.id}", "of": p.id, "member": "girder", "part": "member"})
            elif p.id.startswith("gusset"):
                out.append({"id": f"st-{p.id}", "of": p.id, "member": "girder", "part": "joint"})
            elif p.id.startswith("choke"):
                out.append({"id": f"st-{p.id}", "of": p.id, "member": f"sling-{p.id[-1]}", "part": "joint"})
        elif p.bone.startswith(("bearer", "prop", "sling")):
            out.append({"id": f"st-{p.id}", "of": p.id, "member": p.bone, "part": "member"})
    if scale == 500:
        for side, vc in (("bottom", 0.04), ("top", 0.96)):
            out.append({"id": f"st-chord-{side}", "kind": "box", "bone": "girder", "material": "steel-painted", "role": "body", "tone": 0.44, "size": [L, 0.08, 0.17], "transform": {"translation": [L / 2, vc, 0.0]}, "member": "girder", "part": "member", "slender": True})
        for vv in LAY["verts"]:
            uc = (vv["umin"] + vv["umax"]) / 2
            out.append({"id": f"st-vertical-{vv['k']:02d}", "kind": "box", "bone": "girder", "material": "steel-painted", "role": "body", "tone": 0.5, "size": [0.05, r4(0.92 - 0.08), 0.11], "transform": {"translation": [r4(uc), 0.5, 0.0]}, "member": "girder", "part": "member", "slender": True})
        for dd in LAY["diags"]:
            a = np.array(dd["top"]) + 0.0 * dd["d"]
            b = np.array(dd["bot"])
            c = (a + b) / 2
            ln = float(np.linalg.norm(b - a)) - 0.2
            out.append({"id": f"st-diagonal-{dd['k']:02d}", "kind": "box", "bone": "girder", "material": "steel-painted", "role": "body", "tone": 0.52, "size": [r4(ln), 0.06, 0.13], "transform": {"rotation": quat_w(dd["theta"]), "translation": [r4(c[0]), r4(c[1]), 0.0]}, "member": "girder", "part": "member", "slender": True})
    return out


def thermal_list(scale, parts):
    """🌡️ The thermal model made spatial: every body solid in its region, plus the bearing plates where the body merges them."""
    out = [{"id": f"th-{p.id}", "of": p.id, "region": region_of(p)} for p in parts if not (scale == 500 and p.id.startswith("cleat"))]
    if scale == 500:
        for name, u0 in (("left", 0.0), ("right", L - PLATE["l"])):
            out.append({"id": f"th-bearing-plate-{name}", "kind": "box", "bone": "girder", "material": "steel-bare", "role": "body", "tone": 0.26, "size": [PLATE["l"], 0.075, PLATE["b"]], "transform": {"translation": [r4(u0 + PLATE["l"] / 2), r4(-0.016 + 0.0375), 0.0]}, "region": "bearing-steel"})
    return out


def scale_volume(scale, parts):
    """🧊️ The authored volume of one scale."""
    return {"status": "authored", "denominator": scale, "body": [p.json() for p in parts], "thermal": thermal_list(scale, parts), "structure": structure_list(scale, parts), "decisions": texts["volumes"][str(scale)]}


def masses():
    """⚖️ Bone masses from the 1:50 solids: volume times density."""
    parts = body(50)
    rho = {"steel-painted": STEEL, "steel-bare": STEEL, "steel-cut": STEEL, "softwood-weathered": TIMBER, "polyester": SLING_RHO}
    m, mom = {}, {}
    for p in parts:
        bone = p.bone
        if p.id.startswith("choke-loop"):
            bone = f"sling-{p.id[-1]}"
        kg = p.volume * rho[p.material]
        m[bone] = m.get(bone, 0.0) + kg
        if bone == "girder":
            pts = np.concatenate([np.array(q) for q in p.pieces])
            mom[bone] = mom.get(bone, np.zeros(2)) + kg * pts[:, :2].mean(0)
    return m, mom


def main():
    """🚀️ Writes the asset document."""
    m, mom = masses()
    com = mom["girder"] / m["girder"]
    with open(os.path.join(HERE, "author-meta.json"), "w") as fh:
        json.dump({"masses": m, "girderCom": com.tolist(), "diagonals": [{k: (v.tolist() if isinstance(v, np.ndarray) else v) for k, v in d.items()} for d in LAY["diags"]]}, fh, indent=1)
    doc = sibling("author-document").document(dict(T=T, L=L, X=X, Y0=Y0, m=m, com=com, body=body, drawings=drawings, scale_volume=scale_volume, texts=texts, PROPS=PROPS, PROP_X=PROP_X, prop_profile=prop_profile,
                        BEARER_X=BEARER_X, SLING=SLING, SLING_ANGLE=SLING_ANGLE, CH=CH, LIFT_DY=LIFT_DY, SEAT_DY=SEAT_DY, BEARER_DX=BEARER_DX, PHI=PHI, ORIGIN=ORIGIN, CLEAT_NODES=CLEAT_NODES, PLATE=PLATE))
    with open(OUT, "w") as fh:
        json.dump(doc, fh, ensure_ascii=False, separators=(",", ":"))
    print(f"[DEBUG] wrote {OUT}: {os.path.getsize(OUT)} bytes; masses {json.dumps({k: round(v, 2) for k, v in m.items()})}")


if __name__ == "__main__":
    main()
