# [scratch] NW-A1 shared helpers: skeleton from world joints, forward kinematics, leg IK, shape builders, SVG preview, JSON helpers.
import json, math, os, subprocess

D = math.pi / 180.0


def wrap(a):
    return (a + 180.0) % 360.0 - 180.0


def R(p, deg):
    c, s = math.cos(deg * D), math.sin(deg * D)
    return (c * p[0] - s * p[1], s * p[0] + c * p[1])


def add(a, b):
    return (a[0] + b[0], a[1] + b[1])


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1])


def r4(v):
    return round(v + 0.0, 4)


def T(en, de):
    return {"en": en, "de": de}


class Skel:
    def __init__(self):
        self.bones = []
        self.by = {}
        self.w0 = {}
        self.aw = {}
        self.lim = {}
        self.anchors = []
        self.aby = {}

    def add(self, id, parent, p0, p1, lim, title, kg=0.0, com=None, dist="uniform", mref=None, ground_rel=False):
        aw = math.degrees(math.atan2(p1[1] - p0[1], p1[0] - p0[0]))
        L = math.hypot(p1[0] - p0[0], p1[1] - p0[1])
        if parent is None:
            x, y, ang = 0.0, 0.0, aw
        else:
            po, pa = self.w0[parent], self.aw[parent]
            lx, ly = R(sub(p0, po), -pa)
            x, y, ang = lx, ly, wrap(aw - pa)
        b = {"id": id, "parent": parent, "title": title, "x": r4(x), "y": r4(y), "angleDeg": r4(ang), "lengthM": r4(L), "joint": {"type": "revolute", "minDeg": lim[0], "maxDeg": lim[1]}, "_kg": kg, "_com": com, "_dist": dist, "_mref": mref}
        self.bones.append(b)
        self.by[id] = b
        self.w0[id] = p0
        self.aw[id] = aw
        self.lim[id] = lim
        return b

    def end(self, id):
        b = self.by[id]
        return add(self.w0[id], R((b["lengthM"], 0), self.aw[id]))

    def anchor(self, id, bone, pw, kind, title, **kw):
        lx, ly = R(sub(pw, self.w0[bone]), -self.aw[bone])
        a = {"id": id, "bone": bone, "x": r4(lx), "y": r4(ly), "kind": kind, "title": title}
        a.update(kw)
        self.anchors.append(a)
        self.aby[id] = a
        return a

    def local(self, bone, pw):
        return R(sub(pw, self.w0[bone]), -self.aw[bone])

    def frames(self, offs, root=None, root_angle=0.0):
        """world frames {bone: (origin, angle)} for offsets (degrees, from rest); root origin default = standing pelvis start."""
        fr = {}
        for b in self.bones:
            off = offs.get(b["id"], {}).get("angleDeg", 0.0) if isinstance(offs.get(b["id"], {}), dict) else offs.get(b["id"], 0.0)
            if b["parent"] is None:
                o = root if root is not None else self.w0[b["id"]]
                a = b["angleDeg"] + off + root_angle
            else:
                po, pa = fr[b["parent"]]
                o = add(po, R((b["x"], b["y"]), pa))
                a = pa + b["angleDeg"] + off
            fr[b["id"]] = (o, a)
        return fr

    def world(self, fr, bone, p):
        o, a = fr[bone]
        return add(o, R(p, a))

    def anchor_w(self, fr, aid):
        a = self.aby[aid]
        return self.world(fr, a["bone"], (a["x"], a["y"]))


def two_link(o, tgt, l1, l2, sign):
    """world angles (a1, a2) of two links from o reaching tgt; sign picks the side of the middle joint (+1 = left of the o->tgt line)."""
    dx, dy = tgt[0] - o[0], tgt[1] - o[1]
    d = math.hypot(dx, dy)
    d = min(d, l1 + l2 - 1e-6)
    d = max(d, abs(l1 - l2) + 1e-6)
    base = math.atan2(dy, dx)
    cosA = (l1 * l1 + d * d - l2 * l2) / (2 * l1 * d)
    A = math.acos(max(-1, min(1, cosA)))
    a1 = base + sign * A
    j = (o[0] + l1 * math.cos(a1), o[1] + l1 * math.sin(a1))
    a2 = math.atan2(tgt[1] - j[1], tgt[0] - j[0])
    return a1 / D, a2 / D


def solve_leg(sk, offs, chain, pair, fixed_w, anchor_id, target, sign, root=None):
    """chain: bone ids top to foot; pair=(bA,bB) solved by two-link IK; fixed_w: world angles of all other chain bones.
    offs (bone->deg) holds the trunk offsets; chain offsets are written into it. Returns the achieved anchor world point."""
    fr = sk.frames(offs, root)
    anc = sk.aby[anchor_id]
    last = anc["bone"]
    aw = dict(fixed_w)
    o_last = sub(target, R((anc["x"], anc["y"]), aw[last]))
    iA, iB = chain.index(pair[0]), chain.index(pair[1])
    O = {last: o_last}
    for k in range(len(chain) - 1, iB + 1, -1):
        b, prev = chain[k], chain[k - 1]
        bx = sk.by[b]
        O[prev] = sub(O[b], R((bx["x"], bx["y"]), aw[prev]))
    nxt = chain[iB + 1]
    parent = sk.by[chain[0]]["parent"]
    po, pa = fr[parent]
    o, a = po, pa
    for b in chain[:iA]:
        bb = sk.by[b]
        o = add(o, R((bb["x"], bb["y"]), a))
        a = aw[b]
    bA = sk.by[pair[0]]
    oA = add(o, R((bA["x"], bA["y"]), a))
    l1 = sk.by[pair[1]]["x"]
    l2 = sk.by[nxt]["x"]
    a1, a2 = two_link(oA, O[nxt], l1, l2, sign)
    aw[pair[0]], aw[pair[1]] = a1, a2
    prev_a = pa
    for b in chain:
        offs[b] = r4(wrap(aw[b] - prev_a - sk.by[b]["angleDeg"]))
        prev_a = aw[b]
    return sk.anchor_w(sk.frames(offs, root), anchor_id)


def poses_offsets(offs):
    return {k: {"angleDeg": v} for k, v in offs.items() if abs(v) > 1e-9}


# ---- shapes -------------------------------------------------------------------------------------------------------

def catmull_closed(pts, k=1.0):
    n = len(pts)
    segs = []
    for i in range(n):
        p0, p1, p2, p3 = pts[(i - 1) % n], pts[i], pts[(i + 1) % n], pts[(i + 2) % n]
        c1 = (p1[0] + k * (p2[0] - p0[0]) / 6.0, p1[1] + k * (p2[1] - p0[1]) / 6.0)
        c2 = (p2[0] - k * (p3[0] - p1[0]) / 6.0, p2[1] - k * (p3[1] - p1[1]) / 6.0)
        segs.append((c1, c2, p2))
    d = "M %.4f %.4f" % pts[0]
    for c1, c2, p in segs:
        d += " C %.4f %.4f %.4f %.4f %.4f %.4f" % (c1[0], c1[1], c2[0], c2[1], p[0], p[1])
    return d + " Z"


def catmull_open(pts, k=1.0):
    n = len(pts)
    d = "M %.4f %.4f" % pts[0]
    for i in range(n - 1):
        p0, p1, p2, p3 = pts[max(i - 1, 0)], pts[i], pts[i + 1], pts[min(i + 2, n - 1)]
        c1 = (p1[0] + k * (p2[0] - p0[0]) / 6.0, p1[1] + k * (p2[1] - p0[1]) / 6.0)
        c2 = (p2[0] - k * (p3[0] - p1[0]) / 6.0, p2[1] - k * (p3[1] - p1[1]) / 6.0)
        d += " C %.4f %.4f %.4f %.4f %.4f %.4f" % (c1[0], c1[1], c2[0], c2[1], p2[0], p2[1])
    return d


def shell_pts(stations):
    top = [(x, t) for x, t, b in stations]
    bot = [(x, b) for x, t, b in reversed(stations)]
    pts = top + bot
    out = []
    for p in pts:
        if not out or math.hypot(p[0] - out[-1][0], p[1] - out[-1][1]) > 1e-5:
            out.append(p)
    if math.hypot(out[0][0] - out[-1][0], out[0][1] - out[-1][1]) < 1e-5:
        out.pop()
    return out


class Shapes:
    """collects shapes of one representation"""

    def __init__(self, sk):
        self.sk = sk
        self.items = []

    def _base(self, id, bone, role, z, filled, tone, region=None, regions=None, hatch=None, pen=None, material=None):
        s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
        if region:
            s["region"] = region
        if regions:
            s["regions"] = regions
        if material:
            s["material"] = material
        if hatch:
            s["hatch"] = hatch
        if pen is not None:
            s["penMm"] = pen
        return s

    def shell(self, id, bone, stations, z, tone, role="body", k=1.0, **kw):
        pts = shell_pts(stations)
        s = self._base(id, bone, role, z, True, tone, **kw)
        s.update({"kind": "path", "d": catmull_closed(pts, k)})
        self.items.append(s)
        return s

    def blob(self, id, bone, pts, z, tone, role="body", k=1.0, world=False, **kw):
        if world:
            pts = [self.sk.local(bone, p) for p in pts]
        s = self._base(id, bone, role, z, True, tone, **kw)
        s.update({"kind": "path", "d": catmull_closed(pts, k)})
        self.items.append(s)
        return s

    def poly(self, id, bone, pts, z, tone, closed=True, role="body", filled=None, world=False, **kw):
        if world:
            pts = [self.sk.local(bone, p) for p in pts]
        s = self._base(id, bone, role, z, closed if filled is None else filled, tone, **kw)
        s.update({"kind": "poly", "points": [[r4(a), r4(b)] for a, b in pts], "closed": closed})
        self.items.append(s)
        return s

    def stroke(self, id, bone, pts, z, tone, role="body", pen=0.2, smooth=False, world=False, **kw):
        if world:
            pts = [self.sk.local(bone, p) for p in pts]
        s = self._base(id, bone, role, z, False, tone, pen=pen, **kw)
        if smooth:
            s.update({"kind": "path", "d": catmull_open(pts)})
        else:
            s.update({"kind": "poly", "points": [[r4(a), r4(b)] for a, b in pts], "closed": False})
        self.items.append(s)
        return s

    def capsule(self, id, bone, p0, p1, r0, r1, z, tone, role="body", **kw):
        s = self._base(id, bone, role, z, True, tone, **kw)
        s.update({"kind": "capsule", "x0": r4(p0[0]), "y0": r4(p0[1]), "x1": r4(p1[0]), "y1": r4(p1[1]), "r0": r4(r0), "r1": r4(r1)})
        self.items.append(s)
        return s

    def ellipse(self, id, bone, c, rx, ry, z, tone, role="detail", rot=0, **kw):
        s = self._base(id, bone, role, z, True, tone, **kw)
        s.update({"kind": "ellipse", "cx": r4(c[0]), "cy": r4(c[1]), "rx": r4(rx), "ry": r4(ry)})
        if rot:
            s["rotationDeg"] = rot
        self.items.append(s)
        return s


# ---- geometry evaluation for preview and checks -----------------------------------------------------------------

def parse_path(d):
    toks = d.replace(",", " ").split()
    i = 0
    cur = (0, 0)
    pts = []
    while i < len(toks):
        t = toks[i]
        if t == "M":
            cur = (float(toks[i + 1]), float(toks[i + 2]))
            pts.append(cur)
            i += 3
        elif t == "L":
            cur = (float(toks[i + 1]), float(toks[i + 2]))
            pts.append(cur)
            i += 3
        elif t == "C":
            c1 = (float(toks[i + 1]), float(toks[i + 2]))
            c2 = (float(toks[i + 3]), float(toks[i + 4]))
            e = (float(toks[i + 5]), float(toks[i + 6]))
            for k in range(1, 9):
                u = k / 8
                pts.append(((1 - u) ** 3 * cur[0] + 3 * (1 - u) ** 2 * u * c1[0] + 3 * (1 - u) * u * u * c2[0] + u ** 3 * e[0], (1 - u) ** 3 * cur[1] + 3 * (1 - u) ** 2 * u * c1[1] + 3 * (1 - u) * u * u * c2[1] + u ** 3 * e[1]))
            cur = e
            i += 7
        elif t == "Z":
            i += 1
        else:
            raise ValueError(t)
    return pts


def ring_of(s):
    k = s["kind"]
    if k == "path":
        return parse_path(s["d"])
    if k == "poly":
        return [tuple(p) for p in s["points"]]
    if k == "ellipse":
        return [(s["cx"] + s["rx"] * math.cos(t * D * 15) * math.cos(s.get("rotationDeg", 0) * D) - s["ry"] * math.sin(t * D * 15) * math.sin(s.get("rotationDeg", 0) * D), s["cy"] + s["rx"] * math.cos(t * D * 15) * math.sin(s.get("rotationDeg", 0) * D) + s["ry"] * math.sin(t * D * 15) * math.cos(s.get("rotationDeg", 0) * D)) for t in range(24)]
    if k == "capsule":
        p0, p1 = (s["x0"], s["y0"]), (s["x1"], s["y1"])
        ang = math.atan2(p1[1] - p0[1], p1[0] - p0[0])
        out = []
        for i in range(0, 13):
            a = ang + math.pi / 2 + math.pi * i / 12
            out.append((p0[0] + s["r0"] * math.cos(a), p0[1] + s["r0"] * math.sin(a)))
        for i in range(0, 13):
            a = ang - math.pi / 2 + math.pi * i / 12
            out.append((p1[0] + s["r1"] * math.cos(a), p1[1] + s["r1"] * math.sin(a)))
        return out
    raise ValueError(k)


def world_rings(sk, shapes, fr):
    out = []
    for s in shapes:
        o, a = fr[s["bone"]]
        ring = [add(o, R(p, a)) for p in ring_of(s)]
        out.append((s, ring))
    return out


def bounds(rings):
    xs = [p[0] for s, r in rings if s["role"] != "ground" for p in r]
    ys = [p[1] for s, r in rings if s["role"] != "ground" for p in r]
    return min(xs), max(xs), min(ys), max(ys)


def svg_pose(sk, shapes, fr, path, px_per_m=600, pad=0.08, show_anchors=False, extra=None, bg="#fff"):
    rings = world_rings(sk, shapes, fr)
    x0, x1, y0, y1 = bounds(rings)
    y0 = min(y0, 0)
    W, H = (x1 - x0 + 2 * pad) * px_per_m, (y1 - y0 + 2 * pad) * px_per_m
    def P(p):
        return ((p[0] - x0 + pad) * px_per_m, (y1 - p[1] + pad) * px_per_m)
    g = ['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" viewBox="0 0 %d %d"><rect width="100%%" height="100%%" fill="%s"/>' % (W, H, W, H, bg)]
    g.append('<line x1="0" x2="%d" y1="%.1f" y2="%.1f" stroke="#888" stroke-width="1"/>' % (W, P((0, 0))[1], P((0, 0))[1]))
    for s, ring in sorted(rings, key=lambda t: t[0]["z"]):
        pts = " ".join("%.1f,%.1f" % P(p) for p in ring)
        tone = s["tone"]
        v = int(255 * (1 - tone * 0.8)) if s["filled"] else 0
        if s["filled"]:
            g.append('<polygon points="%s" fill="rgb(%d,%d,%d)" stroke="#000" stroke-width="1"/>' % (pts, v, v, v))
        else:
            g.append('<polyline points="%s" fill="none" stroke="#000" stroke-width="%.1f"/>' % (pts, max(1, s.get("penMm", 0.2) * 3)))
    if show_anchors:
        for a in sk.anchors:
            p = P(sk.world(fr, a["bone"], (a["x"], a["y"])))
            g.append('<circle cx="%.1f" cy="%.1f" r="3" fill="red"/>' % p)
        for b in sk.bones:
            o, an = fr[b["id"]]
            p = P(o)
            g.append('<circle cx="%.1f" cy="%.1f" r="2" fill="blue"/>' % p)
    if extra:
        g.extend(extra(P))
    g.append("</svg>")
    open(path, "w").write("".join(g))
    return path


def png(svg, out, width=None):
    cmd = ["rsvg-convert", "-f", "png", "-o", out, svg]
    if width:
        cmd[1:1] = ["-w", str(width)]
    subprocess.run(cmd, check=True)


def com_of(sk, fr, bones_mass):
    """centre of mass world (x,y) and total from bone dicts with _kg and _com"""
    mx = my = m = 0.0
    for b in sk.bones:
        kg = b["_kg"]
        if kg <= 0:
            continue
        o, a = fr[b["id"]]
        cx, cy = b["_com"]
        p = add(o, R((cx, cy), a))
        mx += kg * p[0]
        my += kg * p[1]
        m += kg
    return mx / m, my / m, m
