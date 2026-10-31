"""NW-V1 scratch toolkit: coordinates only. Every tree, mass, limb, pose and drawing decision is authored in species files; this module
only converts world (rest pose) coordinates to bone-local ones, smooths control points into cubic paths and serialises JSON."""
import math, json, re

D2R = math.pi / 180.0


def T(en, de):
    return {"en": en, "de": de}


def num(v):
    s = ("%.3f" % v).rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s


class Bone:
    def __init__(self, id, parent, base, ang, length, title, d0, d1, kind, limit, density=None, kg=None, material=None, section=True):
        self.id, self.parent, self.bx, self.by, self.ang, self.L = id, parent, base[0], base[1], ang, length
        self.title, self.d0, self.d1, self.kind, self.limit = title, d0, d1, kind, limit
        self.density, self.kg, self.material, self.section = density, kg, material, section
        self.com = None
        self.region = None

    def to_local(self, p):
        dx, dy = p[0] - self.bx, p[1] - self.by
        c, s = math.cos(self.ang * D2R), math.sin(self.ang * D2R)
        return (dx * c + dy * s, -dx * s + dy * c)

    def to_world(self, p):
        c, s = math.cos(self.ang * D2R), math.sin(self.ang * D2R)
        return (self.bx + p[0] * c - p[1] * s, self.by + p[0] * s + p[1] * c)

    def at(self, s, off=0.0):
        return self.to_world((s, off))

    def width(self, s):
        return self.d0 + (self.d1 - self.d0) * (s / self.L if self.L else 0)


class Rig:
    def __init__(self):
        self.bones = {}
        self.order = []

    def add(self, id, parent, base, ang, length, title, d0, d1=None, kind="wood", limit=5.0, density=None, kg=None, material=None, section=True):
        b = Bone(id, parent, base, ang, length, title, d0, d0 if d1 is None else d1, kind, limit, density, kg, material, section)
        self.bones[id] = b
        self.order.append(id)
        return b

    def tip(self, id):
        b = self.bones[id]
        return b.at(b.L)

    def json(self):
        out = []
        for id in self.order:
            b = self.bones[id]
            if b.parent is None:
                x, y, a = b.bx, b.by, b.ang
                joint = {"type": "fixed"}
            else:
                p = self.bones[b.parent]
                x, y = p.to_local((b.bx, b.by))
                a = b.ang - p.ang
                joint = {"type": "revolute", "minDeg": -b.limit, "maxDeg": b.limit}
            d = {"id": id, "parent": b.parent, "title": b.title, "x": round(x, 4), "y": round(y, 4), "angleDeg": round(a, 3), "lengthM": round(b.L, 3), "joint": joint}
            if b.kg is not None:
                cx, cy = b.com if b.com else (b.L / 2, 0.0)
                d["mass"] = {"kg": round(b.kg, 2), "comX": round(cx, 3), "comY": round(cy, 3), "distribution": b.dist if hasattr(b, "dist") else "uniform", "source": "authored-estimate", "reference": b.ref if hasattr(b, "ref") else "wood-handbook-2021"}
            if b.section and b.material:
                dm = b.d0
                d["section"] = {"areaM2": round(math.pi * dm * dm / 4, 5), "secondMomentM4": float("%.4g" % (math.pi * dm ** 4 / 64)), "extremeFibreM": round(dm / 2, 4), "material": b.material, "title": T("%s, d = %s m" % (b.title["en"], ("%.2f" % dm)), "%s, d = %s m" % (b.title["de"], ("%.2f" % dm).replace(".", ",")))}
            out.append(d)
        return out


def wood_volume(b):
    return math.pi / 12 * b.L * (b.d0 ** 2 + b.d0 * b.d1 + b.d1 ** 2)


def wood_area(b):
    return math.pi * (b.d0 + b.d1) / 2 * b.L


def ellipse_area(rx, ry):
    return math.pi * rx * ry


#region paths
def cr_closed(pts):
    n = len(pts)
    cmds = [("M", pts[0])]
    for i in range(n):
        p0, p1, p2, p3 = pts[i - 1], pts[i], pts[(i + 1) % n], pts[(i + 2) % n]
        c1 = (p1[0] + (p2[0] - p0[0]) / 6, p1[1] + (p2[1] - p0[1]) / 6)
        c2 = (p2[0] - (p3[0] - p1[0]) / 6, p2[1] - (p3[1] - p1[1]) / 6)
        cmds.append(("C", c1, c2, p2))
    cmds.append(("Z",))
    return cmds


def cr_open(pts):
    n = len(pts)
    cmds = [("M", pts[0])]
    for i in range(n - 1):
        p0 = pts[i - 1] if i > 0 else pts[0]
        p1, p2 = pts[i], pts[i + 1]
        p3 = pts[i + 2] if i + 2 < n else pts[-1]
        c1 = (p1[0] + (p2[0] - p0[0]) / 6, p1[1] + (p2[1] - p0[1]) / 6)
        c2 = (p2[0] - (p3[0] - p1[0]) / 6, p2[1] - (p3[1] - p1[1]) / 6)
        cmds.append(("C", c1, c2, p2))
    return cmds


def poly(pts, closed=True):
    cmds = [("M", pts[0])] + [("L", p) for p in pts[1:]]
    if closed:
        cmds.append(("Z",))
    return cmds


def serialise(cmds, bone):
    out = []
    for c in cmds:
        if c[0] == "Z":
            out.append("Z")
            continue
        pts = [bone.to_local(p) for p in c[1:]]
        out.append(c[0] + " ".join("%s %s" % (num(p[0]), num(p[1])) for p in pts))
    return "".join(out)


def flat(cmds, steps=6):
    """Flatten to polylines in world coordinates (for own previews and gap probes)."""
    rings, cur = [], []
    last = None
    for c in cmds:
        if c[0] == "M":
            if cur:
                rings.append(cur)
            cur = [c[1]]
            last = c[1]
        elif c[0] == "L":
            cur.append(c[1])
            last = c[1]
        elif c[0] == "C":
            p0, p1, p2, p3 = last, c[1], c[2], c[3]
            for i in range(1, steps + 1):
                t = i / steps
                u = 1 - t
                cur.append((u ** 3 * p0[0] + 3 * u * u * t * p1[0] + 3 * u * t * t * p2[0] + t ** 3 * p3[0], u ** 3 * p0[1] + 3 * u * u * t * p1[1] + 3 * u * t * t * p2[1] + t ** 3 * p3[1]))
            last = p3
        elif c[0] == "Z":
            if cur:
                cur.append(cur[0])
    if cur:
        rings.append(cur)
    return rings
#endregion paths


#region forms
def rot_pts(pts, cx, cy, deg):
    c, s = math.cos(deg * D2R), math.sin(deg * D2R)
    return [(cx + (x - cx) * c - (y - cy) * s, cy + (x - cx) * s + (y - cy) * c) for x, y in pts]


def blob(cx, cy, rx, ry, rot=0.0, k=9, amp=0.12, phase=0.0, vary=None, n=6, under=0.0, pointed=0.6, tilt=0.0):
    """A scalloped (cloud-like) closed outline: k scallops around an ellipse; `vary` is a hand-set list of per-scallop radius factors;
    `under` flattens the lower half (0..1); `tilt` shears the top sideways (m per ry)."""
    vary = vary or [1.0]
    pts = []
    for i in range(k * n):
        phi = 2 * math.pi * i / (k * n) + phase
        j = (i // n) % len(vary)
        u = (i % n) / n
        bump = math.sin(math.pi * u) ** pointed
        f = 1 - amp * vary[j] + amp * vary[j] * bump
        x = rx * f * math.cos(phi)
        y = ry * f * math.sin(phi)
        if y < 0:
            y *= (1 - under)
        x += tilt * max(y, 0) / ry if ry else 0
        pts.append((x, y))
    return rot_pts([(cx + x, cy + y) for x, y in pts], cx, cy, rot)


def envelope(cx, cy, rx, ry, radii, k=0, amp=0.0, phase=0.0, n=3, rot=0.0):
    """A hand-listed polar envelope: `radii` are radius factors at equally spaced angles starting at 0 (CCW), interpolated cosine-smooth,
    with optional small scallops (k per turn)."""
    m = len(radii)
    pts = []
    total = max(k, m) * n if k else m * 6
    for i in range(total):
        phi = 2 * math.pi * i / total
        t = phi / (2 * math.pi) * m
        a = int(t) % m
        b = (a + 1) % m
        w = t - int(t)
        w = (1 - math.cos(math.pi * w)) / 2
        f = radii[a] * (1 - w) + radii[b] * w
        if k:
            u = (phi / (2 * math.pi) * k + phase) % 1.0
            f *= 1 - amp + amp * math.sin(math.pi * u) ** 0.6
        pts.append((cx + rx * f * math.cos(phi), cy + ry * f * math.sin(phi)))
    return rot_pts(pts, cx, cy, rot)


def limb_outline(b, d0=None, d1=None, stations=7, wob=None, s0=0.0, s1=None, round_end=True, bend=0.0, swell0=0.0):
    """A tapered outline along bone `b` in world coordinates; `wob` hand-set half-width factors per station, `bend` bows the axis (m at mid)."""
    d0 = b.d0 if d0 is None else d0
    d1 = b.d1 if d1 is None else d1
    s1 = b.L if s1 is None else s1
    wob = wob or [1.0]
    left, right = [], []
    for i in range(stations):
        t = i / (stations - 1)
        s = s0 + (s1 - s0) * t
        w = (d0 + (d1 - d0) * t) / 2 * wob[i % len(wob)]
        if i == 0 and swell0:
            w *= 1 + swell0
        yb = bend * 4 * t * (1 - t)
        left.append(b.to_world((s, yb + w)))
        right.append(b.to_world((s, yb - w)))
    pts = right + left[::-1]
    return pts


def ring_pts(cx, cy, r, n=10, ry=None, phase=0.0, wob=None):
    wob = wob or [1.0]
    ry = r if ry is None else ry
    return [(cx + r * wob[i % len(wob)] * math.cos(2 * math.pi * i / n + phase), cy + ry * wob[i % len(wob)] * math.sin(2 * math.pi * i / n + phase)) for i in range(n)]


def arc(cx, cy, rx, ry, a0, a1, n=6, rot=0.0):
    pts = [(cx + rx * math.cos((a0 + (a1 - a0) * i / (n - 1)) * D2R), cy + ry * math.sin((a0 + (a1 - a0) * i / (n - 1)) * D2R)) for i in range(n)]
    return rot_pts(pts, cx, cy, rot)


def bbox(pts):
    xs = [p[0] for p in pts]
    ys = [p[1] for p in pts]
    return min(xs), min(ys), max(xs), max(ys)
#endregion forms


class Shape:
    def __init__(self, id, bone, role, cmds, filled, tone, z, region=None, regions=None, material=None, hatch=None, pen=None):
        self.id, self.bone, self.role, self.cmds, self.filled, self.tone, self.z = id, bone, role, cmds, filled, tone, z
        self.region, self.regions, self.material, self.hatch, self.pen = region, regions, material, hatch, pen
        self.frame = None

    def json(self):
        d = {"id": self.id, "bone": self.bone.id, "role": self.role, "z": self.z, "filled": self.filled, "tone": self.tone, "kind": "path", "d": serialise(self.cmds, self.frame or self.bone)}
        if self.region:
            d["region"] = self.region
        if self.regions:
            d["regions"] = self.regions
        if self.material:
            d["material"] = self.material
        if self.hatch:
            d["hatch"] = self.hatch
        if self.pen:
            d["penMm"] = self.pen
        return d


def wind_force(area, zM, v10, cd=0.35, v_ref=8.0, alpha=0.22, expo=-0.6):
    """Drag F = 0.5 rho cd A v(z)^2 with the reconfiguration reduction (v/v_ref)^expo above v_ref (Vogel 1989 type)."""
    v = v10 * (max(zM, 1.0) / 10.0) ** alpha
    red = (v / v_ref) ** expo if v > v_ref else 1.0
    return 0.5 * 1.25 * cd * red * area * v * v


def dump(path, obj):
    with open(path, "w", encoding="utf-8") as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)
        f.write("\n")


IDENT = Bone("model", None, (0, 0), 0, 0, None, 0, 0, "wood", 0, section=False)
