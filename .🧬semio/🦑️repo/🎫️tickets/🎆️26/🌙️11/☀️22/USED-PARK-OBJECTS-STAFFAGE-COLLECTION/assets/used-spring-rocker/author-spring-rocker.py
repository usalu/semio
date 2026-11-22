"""🐴 Authoritative generator of the used spring rocker: every hand-chosen number of the HDPE horse on its coil spring and concrete point footing, drawn in 2D and modelled in 3D per scale from one part list, written to 🔣️.json next to this file."""
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SLUG = "used-spring-rocker"
G = 9.80665
PI = math.pi


def T(en, de):
    """🗣 A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """✂ A number rounded to the micrometre."""
    return round(v + 0.0, 6)


FOOT_HALF, FOOT_DEPTH = 0.25, 0.40
AP_HALF, AP_T = 0.15, 0.010
LP_HALF, LP_T = 0.12, 0.008
UP_HALF, UP_T = 0.10, 0.008
WIRE_D, COIL_R, TURNS, EMBED = 0.020, 0.090, 6, 0.002
Y_AP = AP_T
Y_LP = Y_AP + LP_T
Y_UPB = 0.312
Y_BODY = Y_UPB + UP_T
Y_H0 = Y_LP + WIRE_D / 2 - EMBED
Y_H1 = Y_UPB - WIRE_D / 2 + EMBED
PITCH = (Y_H1 - Y_H0) / TURNS
YJ1 = Y_H0 + 4 * PITCH / 2
YJ2 = Y_H0 + 8 * PITCH / 2
PANEL_IN, PANEL_T = 0.080, 0.019
PANEL_OUT = PANEL_IN + PANEL_T
SEAT = (-0.13, 0.10, 0.490, 0.509)
BOTTOM = (-0.15, 0.15, Y_BODY, Y_BODY + 0.019)
HANDLE = (0.27, 0.66)
FOOTREST = (0.13, 0.345)
TUBE_R = 0.01345
TUBE_A = PI * (0.01345 ** 2 - 0.01085 ** 2)
GRIP_R, GRIP_END = 0.017, 0.200
SLEEVE_R, SLEEVE_END, CAP_R, CAP_T = 0.020, 0.192, 0.025, 0.008
CHIP_Z = 0.030
SPACER_BURIED = (SEAT[2] - BOTTOM[3]) * 2 * PANEL_IN + 2 * PANEL_T * 2 * PANEL_IN
EYE = (0.360, 0.710, 0.015)
NUTS = [(sx * 0.09, sz * 0.09) for sx in (-1, 1) for sz in (-1, 1)]
BOLTS = [(sx * 0.07, sz * 0.07) for sx in (-1, 1) for sz in (-1, 1)]
LIFT_DY = 0.30
ROCK_DEG = 12.0

RHO = {"concrete": 2400.0, "steel": 7850.0, "hdpe": 955.0}
E_SPRING, NU_SPRING = 206e9, 0.30

PANEL = [(-0.22, 0.32), (0.22, 0.32), (0.27, 0.34), (0.30, 0.38), (0.31, 0.44), (0.33, 0.52), (0.36, 0.58), (0.40, 0.595), (0.47, 0.60),
         (0.495, 0.615), (0.50, 0.655), (0.47, 0.685), (0.40, 0.72), (0.29, 0.775), (0.27, 0.82), (0.255, 0.78), (0.22, 0.70), (0.17, 0.60),
         (0.13, 0.54), (0.11, 0.53), (-0.13, 0.53), (-0.16, 0.545), (-0.20, 0.56), (-0.30, 0.565), (-0.35, 0.56), (-0.40, 0.585), (-0.44, 0.58),
         (-0.45, 0.56), (-0.44, 0.52), (-0.41, 0.50), (-0.37, 0.51), (-0.34, 0.49), (-0.33, 0.44), (-0.30, 0.38), (-0.26, 0.34)]
EAR_CHIP = [(0.29, 0.775), (0.284, 0.789), (0.271, 0.787), (0.276, 0.804), (0.27, 0.82)]
PANEL_CHIPPED = PANEL[:14] + EAR_CHIP[1:4] + PANEL[14:]
PANEL_200 = [(-0.22, 0.32), (0.22, 0.32), (0.30, 0.38), (0.33, 0.52), (0.36, 0.58), (0.47, 0.60), (0.50, 0.655), (0.40, 0.72), (0.29, 0.775),
             (0.27, 0.82), (0.255, 0.78), (0.17, 0.60), (0.13, 0.54), (0.11, 0.53), (-0.13, 0.53), (-0.20, 0.56), (-0.35, 0.56), (-0.40, 0.585),
             (-0.45, 0.56), (-0.41, 0.50), (-0.34, 0.49), (-0.30, 0.38)]

BONES = [
    ("footing", None, 0.0, -FOOT_DEPTH, 90.0, FOOT_DEPTH),
    ("lift", "footing", FOOT_DEPTH, 0.0, 0.0, Y_LP),
    ("spring-1", "lift", Y_LP, 0.0, 0.0, YJ1 - Y_LP),
    ("spring-2", "spring-1", YJ1 - Y_LP, 0.0, 0.0, YJ2 - YJ1),
    ("spring-3", "spring-2", YJ2 - YJ1, 0.0, 0.0, Y_BODY - YJ2),
    ("body", "spring-3", Y_BODY - YJ2, 0.0, -90.0, 0.22),
]


def frames():
    """🧭 World rest frame (origin, angle in degrees) of every bone of the planar chain."""
    out = {}
    for bid, parent, x, y, ang, _ in BONES:
        if parent is None:
            out[bid] = ((x, y), ang)
            continue
        (px, py), pa = out[parent]
        c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
        out[bid] = ((px + c * x - s * y, py + s * x + c * y), pa + ang)
    return out


FR = frames()


def loc(bone, x, y, z=0.0):
    """📍 World rest point to the bone-local (u, v, w) frame."""
    (ox, oy), a = FR[bone]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    dx, dy = x - ox, y - oy
    return (dx * c + dy * s, -dx * s + dy * c, z)


def locd(bone, d):
    """➡ World direction to the bone-local frame."""
    a = FR[bone][1]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return (d[0] * c + d[1] * s, -d[0] * s + d[1] * c, d[2])


def el(bone, pts):
    """✏ World (x, y) points as elevation shape points of the bone."""
    return [[r6(loc(bone, x, y)[0]), r6(loc(bone, x, y)[1])] for x, y in pts]


def circle(cx, cy, r, n):
    """⭕ A counter-clockwise polygon of n points."""
    return [(cx + r * math.cos(2 * PI * k / n), cy + r * math.sin(2 * PI * k / n)) for k in range(n)]


def hexagon(cx, cz, s):
    """⬡ The hexagon of a nut of width s across flats, in plan (x, z)."""
    rr = s / math.sqrt(3)
    return [(cx + rr * math.cos(PI / 6 + k * PI / 3), cz + rr * math.sin(PI / 6 + k * PI / 3)) for k in range(6)]


def area2(pts):
    """📐 Signed area of a polygon."""
    return 0.5 * sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))


def centroid2(pts):
    """🎯 Area centroid of a polygon."""
    a = area2(pts)
    cx = sum((pts[i][0] + pts[(i + 1) % len(pts)][0]) * (pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1]) for i in range(len(pts))) / (6 * a)
    cy = sum((pts[i][1] + pts[(i + 1) % len(pts)][1]) * (pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1]) for i in range(len(pts))) / (6 * a)
    return cx, cy


def perim(pts):
    """📏 Perimeter of a closed polygon."""
    return sum(math.dist(pts[i], pts[(i + 1) % len(pts)]) for i in range(len(pts)))


def quat(m):
    """🔄 Unit quaternion [x, y, z, w] of a rotation matrix given by its columns."""
    (a, d, g), (b, e, h), (c, f, i) = m
    tr = a + e + i
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        q = ((h - f) / s, (c - g) / s, (d - b) / s, 0.25 * s)
    elif a > e and a > i:
        s = math.sqrt(1.0 + a - e - i) * 2
        q = (0.25 * s, (b + d) / s, (c + g) / s, (h - f) / s)
    elif e > i:
        s = math.sqrt(1.0 + e - a - i) * 2
        q = ((b + d) / s, 0.25 * s, (f + h) / s, (c - g) / s)
    else:
        s = math.sqrt(1.0 + i - a - e) * 2
        q = ((c + g) / s, (f + h) / s, 0.25 * s, (d - b) / s)
    n = math.sqrt(sum(v * v for v in q))
    return [r6(v / n) for v in q]


def helix_point(phi):
    """🌀 World centre-line point of the spring wire at the helix angle phi."""
    return (COIL_R * math.sin(phi), Y_H0 + PITCH * phi / (2 * PI), COIL_R * math.cos(phi))


def helix_sections(bone, phi0, phi1, per_turn):
    """➰ Loft sections of the round wire along the helix between two angles, in the frame of the bone."""
    n = max(2, round((phi1 - phi0) / (2 * PI) * per_turn))
    out = []
    for k in range(n + 1):
        phi = phi0 + (phi1 - phi0) * k / n
        p = helix_point(phi)
        t = (COIL_R * math.cos(phi), PITCH / (2 * PI), -COIL_R * math.sin(phi))
        tl = math.sqrt(sum(v * v for v in t))
        t = tuple(v / tl for v in t)
        xr = (math.sin(phi), 0.0, math.cos(phi))
        yr = (t[1] * xr[2] - t[2] * xr[1], t[2] * xr[0] - t[0] * xr[2], t[0] * xr[1] - t[1] * xr[0])
        cols = (locd(bone, xr), locd(bone, yr), locd(bone, t))
        u, v, w = loc(bone, *p)
        out.append({"at": [r6(u), r6(v), r6(w)], "rx": WIRE_D / 2, "ry": WIRE_D / 2, "rotation": quat(cols)})
    return out


HELIX_LEN = TURNS * math.sqrt((2 * PI * COIL_R) ** 2 + PITCH ** 2)


class Part:
    """🧱 One physical part of one scale: its 3D solid in the bone frame, its three 2D projections, its area and its mass from the same numbers."""

    def __init__(self, pid, bone, kind, geo, material, region, tone, role="body", extra=None, mass=None, buried=0.0, hatch=None, views=("elevation", "front", "plan")):
        self.id, self.bone, self.kind, self.geo, self.material, self.region, self.tone = pid, bone, kind, geo, material, region, tone
        self.role, self.extra, self.mass_override, self.buried, self.hatch, self.views = role, extra or {}, mass, buried, hatch, views

    def solid(self):
        """🧊 The body solid of the part in the rest frame of its bone."""
        g, b = self.geo, self.bone
        base = {"id": self.id, "kind": None, "bone": b, "material": self.material, "role": self.role, "tone": self.tone}
        if self.kind == "box":
            (x0, y0, z0), (x1, y1, z1) = g
            sz = locd(b, (x1 - x0, y1 - y0, z1 - z0))
            c = loc(b, (x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2)
            s = dict(base, kind="box", size=[r6(abs(sz[0])), r6(abs(sz[1])), r6(abs(sz[2]))], transform={"translation": [r6(c[0]), r6(c[1]), r6(c[2])]})
            if self.extra.get("radius"):
                s["cornerRadius"] = self.extra["radius"]
        elif self.kind == "cyly":
            cx, cz, y0, y1, r = g
            s = dict(base, kind="revolve", profile=[[0.0, 0.0], [0.0, r6(r)], [r6(y1 - y0), r6(r)], [r6(y1 - y0), 0.0]], axis=[r6(v) for v in locd(b, (0, 1, 0))], origin=[r6(v) for v in loc(b, cx, y0, cz)])
        elif self.kind == "cylz":
            cx, cy, z0, z1, r = g
            s = dict(base, kind="revolve", profile=[[0.0, 0.0], [0.0, r6(r)], [r6(z1 - z0), r6(r)], [r6(z1 - z0), 0.0]], axis=[0.0, 0.0, 1.0], origin=[r6(v) for v in loc(b, cx, cy, z0)])
        elif self.kind == "extz":
            prof, holes, z0, z1 = g
            s = dict(base, kind="extrude", plane="xy", profile=el(b, prof), **{"from": r6(z0), "to": r6(z1)})
            if holes:
                s["holes"] = [el(b, h) for h in holes]
        elif self.kind == "exty":
            prof, y0, y1 = g
            ox, oy = FR[b][0]
            if abs(FR[b][1]) < 1e-9:
                s = dict(base, kind="extrude", plane="zx", profile=[[r6(z), r6(x - ox)] for x, z in prof], **{"from": r6(y0 - oy), "to": r6(y1 - oy)})
            else:
                s = dict(base, kind="extrude", plane="yz", profile=[[r6(-(x - ox)), r6(z)] for x, z in prof], **{"from": r6(y0 - oy), "to": r6(y1 - oy)})
        elif self.kind == "helix":
            phi0, phi1, per_turn = g
            s = dict(base, kind="loft", sections=helix_sections(b, phi0, phi1, per_turn), capStart=True, capEnd=True)
        elif self.kind == "ellipsoid":
            (cx, cy, cz), radii = g
            c = loc(b, cx, cy, cz)
            rl = locd(b, radii)
            s = dict(base, kind="ellipsoid", radii=[r6(abs(v)) for v in rl], transform={"translation": [r6(c[0]), r6(c[1]), r6(c[2])]})
        for key in ("slender", "sheet", "texture", "envelope", "tags", "sectionFaces"):
            if key in self.extra:
                s[key] = self.extra[key]
        return s

    def bounds(self):
        """📦 World bounds (x0, y0, z0), (x1, y1, z1) of the part at rest."""
        g = self.geo
        if self.kind == "box":
            return g
        if self.kind == "cyly":
            cx, cz, y0, y1, r = g
            return (cx - r, y0, cz - r), (cx + r, y1, cz + r)
        if self.kind == "cylz":
            cx, cy, z0, z1, r = g
            return (cx - r, cy - r, z0), (cx + r, cy + r, z1)
        if self.kind == "extz":
            prof, _, z0, z1 = g
            return (min(p[0] for p in prof), min(p[1] for p in prof), z0), (max(p[0] for p in prof), max(p[1] for p in prof), z1)
        if self.kind == "exty":
            prof, y0, y1 = g
            return (min(p[0] for p in prof), y0, min(p[1] for p in prof)), (max(p[0] for p in prof), y1, max(p[1] for p in prof))
        if self.kind == "helix":
            phi0, phi1, _ = g
            ya, yb = helix_point(phi0)[1], helix_point(phi1)[1]
            return (-COIL_R - WIRE_D / 2, ya - WIRE_D / 2, -COIL_R - WIRE_D / 2), (COIL_R + WIRE_D / 2, yb + WIRE_D / 2, COIL_R + WIRE_D / 2)
        (cx, cy, cz), (a, bb, c) = g
        return (cx - a, cy - bb, cz - c), (cx + a, cy + bb, cz + c)

    def volume_area_com(self):
        """⚖ Volume, outer surface area and world centre of the part."""
        g = self.geo
        if self.kind == "box":
            (x0, y0, z0), (x1, y1, z1) = g
            a, b, c = x1 - x0, y1 - y0, z1 - z0
            return a * b * c, 2 * (a * b + b * c + a * c), ((x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2)
        if self.kind == "cyly":
            cx, cz, y0, y1, r = g
            h = y1 - y0
            return PI * r * r * h, 2 * PI * r * h + 2 * PI * r * r, (cx, (y0 + y1) / 2, cz)
        if self.kind == "cylz":
            cx, cy, z0, z1, r = g
            h = z1 - z0
            return PI * r * r * h, 2 * PI * r * h + 2 * PI * r * r, (cx, cy, (z0 + z1) / 2)
        if self.kind == "extz":
            prof, holes, z0, z1 = g
            a = abs(area2(prof)) - sum(abs(area2(h)) for h in holes)
            per = perim(prof) + sum(perim(h) for h in holes)
            cx, cy = centroid2(prof)
            return a * (z1 - z0), 2 * a + per * (z1 - z0), (cx, cy, (z0 + z1) / 2)
        if self.kind == "exty":
            prof, y0, y1 = g
            a = abs(area2(prof))
            cx, cz = centroid2(prof)
            return a * (y1 - y0), 2 * a + perim(prof) * (y1 - y0), (cx, (y0 + y1) / 2, cz)
        if self.kind == "helix":
            phi0, phi1, _ = g
            frac = (phi1 - phi0) / (TURNS * 2 * PI)
            ln = HELIX_LEN * frac
            return PI * (WIRE_D / 2) ** 2 * ln, PI * WIRE_D * ln, (0.0, (helix_point(phi0)[1] + helix_point(phi1)[1]) / 2, 0.0)
        (cx, cy, cz), (a, b, c) = g
        return 4 / 3 * PI * a * b * c, 4 * PI * ((a * b) ** 1.6 / 3 + (a * c) ** 1.6 / 3 + (b * c) ** 1.6 / 3) ** (1 / 1.6), (cx, cy, cz)

    def mass(self):
        """🏋 Mass of the part from its volume and density, or the stated section mass."""
        if self.mass_override is not None:
            return self.mass_override
        v, _, _ = self.volume_area_com()
        return v * RHO[MAT_FAMILY[self.material]]

    def area(self):
        """🌡 Outer area the part gives its region (faces buried by the same region removed)."""
        return self.volume_area_com()[1] - self.buried

    def shapes(self, view, pen=None):
        """🖊 The filled closed 2D shape of the part in one view, from the same numbers as the solid."""
        if view not in self.views:
            return []
        g, b = self.geo, self.bone
        base = {"id": self.id, "bone": b, "role": self.role, "z": 0.0, "filled": True, "tone": self.tone, "region": self.region, "material": self.material}
        if self.hatch:
            base["hatch"] = self.hatch
        if pen is not None:
            base["penMm"] = pen
        (x0, y0, z0), (x1, y1, z1) = self.bounds()
        if self.kind == "helix" and view in ("elevation", "front"):
            return helix_bands(self, view, base)
        if view == "elevation":
            if self.kind in ("box", "cyly", "exty", "helix"):
                shape = dict(base, kind="poly", points=el(b, [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]), closed=True)
            elif self.kind == "cylz":
                cx, cy, _, _, r = g
                u, v, _ = loc(b, cx, cy)
                shape = dict(base, kind="ellipse", cx=r6(u), cy=r6(v), rx=r6(r), ry=r6(r))
            elif self.kind == "extz":
                prof, holes, _, _ = g
                if holes:
                    shape = dict(base, kind="path", d=pathd([el(b, prof)] + [el(b, h) for h in holes]))
                else:
                    shape = dict(base, kind="poly", points=el(b, prof), closed=True)
            else:
                (cx, cy, _), (a, bb, _) = g
                u, v, _ = loc(b, cx, cy)
                shape = dict(base, kind="ellipse", cx=r6(u), cy=r6(v), rx=r6(abs(locd(b, (a, bb, 0))[0])), ry=r6(abs(locd(b, (a, bb, 0))[1])))
            shape["z"] = r6(3.0 - (z0 + z1) / 2)
        elif view == "front":
            if self.kind == "ellipsoid":
                (cx, cy, cz), (a, bb, c) = g
                shape = dict(base, kind="ellipse", cx=r6(cz), cy=r6(cy), rx=r6(c), ry=r6(bb))
            else:
                shape = dict(base, kind="rect", x=r6(z0), y=r6(y0), w=r6(z1 - z0), h=r6(y1 - y0))
            shape["z"] = r6(3.0 + (x0 + x1) / 2)
        else:
            if self.kind == "cyly":
                cx, cz, _, _, r = g
                shape = dict(base, kind="ellipse", cx=r6(cx), cy=r6(cz), rx=r6(r), ry=r6(r))
            elif self.kind == "exty":
                prof, _, _ = g
                shape = dict(base, kind="poly", points=[[r6(x), r6(z)] for x, z in prof], closed=True)
            elif self.kind == "helix":
                ro, ri = COIL_R + WIRE_D / 2, COIL_R - WIRE_D / 2
                shape = dict(base, kind="path", d=pathd([[(r6(x), r6(z)) for x, z in circle(0, 0, ro, 48)], [(r6(x), r6(z)) for x, z in circle(0, 0, ri, 48)]]))
            elif self.kind == "ellipsoid":
                (cx, cy, cz), (a, bb, c) = g
                shape = dict(base, kind="ellipse", cx=r6(cx), cy=r6(cz), rx=r6(a), ry=r6(c))
            else:
                shape = dict(base, kind="rect", x=r6(x0), y=r6(z0), w=r6(x1 - x0), h=r6(z1 - z0))
            shape["z"] = r6(3.0 + y1)
        if self.role == "detail":
            shape["z"] = r6(shape["z"] + 0.5)
        return [shape]


def helix_bands(part, view, base):
    """➰ The true projection of the wire in side or front view: one closed band per piece of the helix that runs monotone across the view, near pieces painted over far ones."""
    phi0, phi1, _ = part.geo
    rw = WIRE_D / 2
    turn0 = PI / 2 if view == "elevation" else 0.0
    cuts = [phi0] + [turn0 + m * PI for m in range(0, 40) if phi0 + 1e-9 < turn0 + m * PI < phi1 - 1e-9] + [phi1]
    out = []
    for i in range(len(cuts) - 1):
        a, b = cuts[i], cuts[i + 1]
        samples = []
        for j in range(241):
            ph = a + (b - a) * j / 240
            x, y, z = helix_point(ph)
            samples.append(((x if view == "elevation" else z), y))
        depth = math.cos((a + b) / 2) if view == "elevation" else math.sin((a + b) / 2)
        near = depth < 0 if view == "elevation" else depth > 0
        xs = sorted(h for h, _ in samples)
        lo, hi = xs[0] - rw, xs[-1] + rw
        upper, lower = [], []
        n = 48
        for c in range(n + 1):
            xc = lo + (hi - lo) * c / n
            ys = [(y + math.sqrt(max(rw * rw - (xc - h) ** 2, 0.0)), y - math.sqrt(max(rw * rw - (xc - h) ** 2, 0.0))) for h, y in samples if abs(xc - h) <= rw + 1e-12]
            if not ys:
                continue
            upper.append((xc, max(u for u, _ in ys)))
            lower.append((xc, min(l for _, l in ys)))
        ring = upper + lower[::-1]
        clean = [ring[0]]
        for q in ring[1:]:
            if math.dist(q, clean[-1]) > 1e-5:
                clean.append(q)
        pts = el(part.bone, clean) if view == "elevation" else [[r6(h), r6(y)] for h, y in clean]
        out.append(dict(base, id=f"{part.id}-{view[0]}{i + 1}", kind="poly", points=pts, closed=True, z=r6((3.2 if near else 2.8) + i * 0.001)))
    return out


def pathd(rings):
    """〰 An SVG path of closed rings."""
    return " ".join("M " + " L ".join(f"{p[0]:.6f} {p[1]:.6f}" for p in ring) + " Z" for ring in rings)


MAT_FAMILY = {"concrete-c20-25": "concrete", "steel-galvanised-weathered": "steel", "spring-steel-painted": "steel", "hdpe-red-faded": "hdpe", "hdpe-blue-faded": "hdpe", "hdpe-green-faded": "hdpe"}
TONE = {"footing-concrete": 0.86, "plates-steel": 0.6, "spring-steel": 0.27, "panels-hdpe": 0.5, "seat-hdpe": 0.4, "grips-hdpe": 0.32, "tubes-steel": 0.66}
MAT = {"footing-concrete": "concrete-c20-25", "plates-steel": "steel-galvanised-weathered", "spring-steel": "spring-steel-painted", "panels-hdpe": "hdpe-red-faded", "seat-hdpe": "hdpe-blue-faded", "grips-hdpe": "hdpe-green-faded", "tubes-steel": "steel-galvanised-weathered"}


def P(pid, bone, kind, geo, region, **kw):
    """🧩 A part with the material and tone of its region."""
    return Part(pid, bone, kind, geo, MAT[region], region, kw.pop("tone", TONE[region]), **kw)


def sector(cx, cy, r, gap0, gap1, n=18):
    """🍕 A circular sector polygon (x, y) without the angular gap gap0..gap1 (radians), centre included."""
    span = 2 * PI - (gap1 - gap0)
    return [(cx, cy)] + [(cx + r * math.cos(gap1 + span * k / n), cy + r * math.sin(gap1 + span * k / n)) for k in range(n + 1)]


HELIX_BONES = [("spring-1", 0.0, 4 * PI), ("spring-2", 4 * PI, 8 * PI), ("spring-3", 8 * PI, 12 * PI)]


def parts(scale):
    """🗂 The part list of one scale: the same object, bones and ids, each scale modelled for its own smallest feature."""
    out = []
    hx, hy = HANDLE
    fx, fy = FOOTREST
    out.append(P("footing-block", "footing", "box", ((-FOOT_HALF, -FOOT_DEPTH, -FOOT_HALF), (FOOT_HALF, 0.0, FOOT_HALF)), "footing-concrete", hatch="concrete"))
    out.append(P("anchor-plate", "footing", "box", ((-AP_HALF, 0.0, -AP_HALF), (AP_HALF, Y_AP, AP_HALF)), "plates-steel", extra={"sheet": True} if scale >= 100 else {}, buried=(2 * LP_HALF) ** 2))
    out.append(P("lower-plate", "lift", "box", ((-LP_HALF, Y_AP, -LP_HALF), (LP_HALF, Y_LP, LP_HALF)), "plates-steel", tone=0.55, extra={"sheet": True} if scale >= 100 else {}, buried=(2 * LP_HALF) ** 2))
    per_turn = {50: 24, 100: 16}.get(scale)
    for bone, p0, p1 in HELIX_BONES:
        k = bone[-1]
        if scale <= 100:
            out.append(P(f"coil-{k}", bone, "helix", (p0, p1, per_turn), "spring-steel"))
        else:
            ya = Y_LP if k == "1" else helix_point(p0)[1]
            yb = Y_UPB if k == "3" else helix_point(p1)[1]
            out.append(P(f"coil-{k}", bone, "cyly", (0.0, 0.0, ya, yb, COIL_R + WIRE_D / 2), "spring-steel", extra={"envelope": True}))
    out.append(P("upper-plate", "spring-3", "box", ((-UP_HALF, Y_UPB, -UP_HALF), (UP_HALF, Y_BODY, UP_HALF)), "plates-steel", tone=0.55, extra={"sheet": True} if scale >= 100 else {}))
    holes = [circle(EYE[0], EYE[1], EYE[2], 20)] if scale <= 100 else []
    prof_r = PANEL_CHIPPED if scale == 50 else (PANEL if scale == 100 else PANEL_200)
    prof_l = PANEL if scale <= 100 else PANEL_200
    sheet = {"sheet": True} if scale >= 200 else {}
    out.append(P("panel-left", "body", "extz", (prof_l, holes, PANEL_IN, PANEL_OUT), "panels-hdpe", tone=0.56, extra=sheet))
    out.append(P("panel-right", "body", "extz", (prof_r, holes, -PANEL_OUT, -PANEL_IN), "panels-hdpe", extra=sheet))
    if scale <= 100:
        cav = (0.081 - (-0.111)) * 2 * PANEL_IN + 2 * PANEL_T * 2 * PANEL_IN
        out.append(P("seat-board", "body", "box", ((SEAT[0], SEAT[2], -PANEL_IN), (SEAT[1], SEAT[3], PANEL_IN)), "seat-hdpe", buried=cav))
        out.append(P("bottom-board", "body", "box", ((BOTTOM[0], BOTTOM[2], -PANEL_IN), (BOTTOM[1], BOTTOM[3], PANEL_IN)), "seat-hdpe", tone=0.45, buried=cav))
    else:
        out.append(P("seat-board", "body", "box", ((SEAT[0], SEAT[2], -PANEL_IN), (SEAT[1], 0.53, PANEL_IN)), "seat-hdpe"))
        out.append(P("bottom-board", "body", "box", ((BOTTOM[0], Y_BODY, -PANEL_IN), (BOTTOM[1], Y_BODY + 0.030, PANEL_IN)), "seat-hdpe", tone=0.45))
    if scale <= 100:
        out.append(P("spacer-front", "body", "box", ((0.081, BOTTOM[3], -PANEL_IN), (0.100, SEAT[2], PANEL_IN)), "seat-hdpe", tone=0.45, buried=SPACER_BURIED))
        out.append(P("spacer-rear", "body", "box", ((-0.130, BOTTOM[3], -PANEL_IN), (-0.111, SEAT[2], PANEL_IN)), "seat-hdpe", tone=0.45, buried=SPACER_BURIED))
    else:
        out.append(P("spacer-front", "body", "box", ((0.070, Y_BODY + 0.030, -PANEL_IN), (0.100, SEAT[2], PANEL_IN)), "seat-hdpe", tone=0.45))
    tube_r = TUBE_R if scale <= 100 else GRIP_R
    out.append(P("handle-tube", "body", "cylz", (hx, hy, -PANEL_OUT, PANEL_OUT, tube_r), "tubes-steel", mass=TUBE_A * 0.40 * RHO["steel"]))
    out.append(P("footrest-tube", "body", "cylz", (fx, fy, -PANEL_OUT, PANEL_OUT, tube_r), "tubes-steel", mass=TUBE_A * 0.40 * RHO["steel"]))
    out.append(P("grip-left", "body", "cylz", (hx, hy, PANEL_OUT, GRIP_END, GRIP_R), "grips-hdpe"))
    if scale == 50:
        out.append(P("grip-right", "body", "cylz", (hx, hy, -GRIP_END + CHIP_Z, -PANEL_OUT, GRIP_R), "grips-hdpe", buried=PI * GRIP_R ** 2 * 1.75))
        out.append(P("grip-right-chipped", "body", "extz", (sector(hx, hy, GRIP_R, 0.15 * PI, 0.65 * PI), [], -GRIP_END, -GRIP_END + CHIP_Z), "grips-hdpe", tone=0.3))
    else:
        out.append(P("grip-right", "body", "cylz", (hx, hy, -GRIP_END, -PANEL_OUT, GRIP_R), "grips-hdpe"))
    sleeve_end = SLEEVE_END if scale <= 50 else GRIP_END
    out.append(P("footrest-left", "body", "cylz", (fx, fy, PANEL_OUT, sleeve_end, SLEEVE_R), "grips-hdpe", buried=PI * SLEEVE_R ** 2 if scale == 50 else 0.0))
    out.append(P("footrest-right", "body", "cylz", (fx, fy, -sleeve_end, -PANEL_OUT, SLEEVE_R), "grips-hdpe", buried=PI * SLEEVE_R ** 2 if scale == 50 else 0.0))
    if scale == 50:
        out.append(P("cap-left", "body", "cylz", (fx, fy, SLEEVE_END, GRIP_END, CAP_R), "grips-hdpe", tone=0.28, buried=PI * SLEEVE_R ** 2))
        out.append(P("cap-right", "body", "cylz", (fx, fy, -GRIP_END, -SLEEVE_END, CAP_R), "grips-hdpe", tone=0.28, buried=PI * SLEEVE_R ** 2))
        for i, (nx, nz) in enumerate(NUTS):
            out.append(P(f"stud-{i + 1}", "footing", "cyly", (nx, nz, Y_AP, 0.042, 0.008), "plates-steel", role="detail", tone=0.5, buried=2 * PI * 0.008 * (0.0328 - Y_AP) + PI * 0.008 ** 2))
            out.append(P(f"nut-{i + 1}", "lift", "exty", (hexagon(nx, nz, 0.024), Y_LP, Y_LP + 0.0148), "plates-steel", role="detail", tone=0.45, buried=2 * (math.sqrt(3) / 2 * 0.024 ** 2) + PI * 0.008 ** 2 * 2))
        for i, (bx, bz) in enumerate(BOLTS):
            out.append(P(f"bolt-{i + 1}", "spring-3", "exty", (hexagon(bx, bz, 0.016), Y_UPB - 0.0084, Y_UPB), "plates-steel", role="detail", tone=0.45, buried=2 * (math.sqrt(3) / 2 * 0.016 ** 2)))
        for i, (side, bone, yb, yt) in enumerate([(1, "spring-1", Y_LP, Y_LP + 0.026), (-1, "spring-1", Y_LP, Y_LP + 0.026), (1, "spring-3", Y_UPB - 0.026, Y_UPB), (-1, "spring-3", Y_UPB - 0.026, Y_UPB)]):
            cx = side * (COIL_R + 0.004)
            out.append(P(f"clamp-{i + 1}", bone, "box", ((cx - 0.016, yb, -0.020), (cx + 0.016, yt, 0.020)), "plates-steel", role="detail", tone=0.5, buried=0.032 * 0.040 * 2))
    return out


def detail_strokes(scale):
    """🖋 Open detail and texture strokes of one scale: the crack and chip of the grip, the chipped ear and the traces of use."""
    out = {"elevation": [], "front": [], "plan": []}
    hx, hy = HANDLE
    if scale == 50:
        tex = {"role": "texture", "filled": False, "tone": 0.2, "penMm": 0.13, "kind": "poly", "closed": False}
        body = lambda pts: el("body", pts)
        out["elevation"] += [
            dict(tex, id="graffiti-tag", bone="body", z=4.0, region="panels-hdpe", material=MAT["panels-hdpe"], points=body([(-0.33, 0.43), (-0.31, 0.47), (-0.295, 0.43), (-0.28, 0.465), (-0.26, 0.425), (-0.24, 0.46), (-0.22, 0.43), (-0.20, 0.455)])),
            dict(tex, id="graffiti-underline", bone="body", z=4.0, region="panels-hdpe", material=MAT["panels-hdpe"], points=body([(-0.335, 0.415), (-0.25, 0.41), (-0.19, 0.418)])),
            dict(tex, id="sticker-residue", bone="body", z=4.0, region="panels-hdpe", material=MAT["panels-hdpe"], closed=True, points=body([(0.205, 0.50), (0.255, 0.505), (0.252, 0.54), (0.215, 0.545), (0.20, 0.525)])),
            dict(tex, id="rim-scuff-front", bone="body", z=4.0, region="panels-hdpe", material=MAT["panels-hdpe"], points=body([(0.04, 0.515), (0.09, 0.522), (0.12, 0.528)])),
            dict(tex, id="rim-scuff-rear", bone="body", z=4.0, region="panels-hdpe", material=MAT["panels-hdpe"], points=body([(-0.17, 0.53), (-0.11, 0.517), (-0.05, 0.515)])),
            dict(tex, id="fade-line", bone="body", z=4.0, region="panels-hdpe", material=MAT["panels-hdpe"], points=body([(-0.42, 0.545), (-0.30, 0.548), (-0.18, 0.538), (-0.10, 0.522)])),
            dict(tex, id="grip-crack", bone="body", z=4.2, region="grips-hdpe", material=MAT["grips-hdpe"], penMm=0.18, role="detail", points=body([(hx + 0.003, hy + 0.016), (hx + 0.006, hy + 0.004), (hx + 0.002, hy - 0.006), (hx + 0.005, hy - 0.015)])),
            dict(tex, id="grip-chip-edge", bone="body", z=4.2, region="grips-hdpe", material=MAT["grips-hdpe"], penMm=0.18, role="detail", points=body([(hx + GRIP_R * math.cos(0.15 * PI), hy + GRIP_R * math.sin(0.15 * PI)), (hx, hy), (hx + GRIP_R * math.cos(0.65 * PI), hy + GRIP_R * math.sin(0.65 * PI))])),
            dict(tex, id="ear-chip-edge", bone="body", z=4.0, region="panels-hdpe", material=MAT["panels-hdpe"], role="detail", penMm=0.18, points=body(EAR_CHIP[:4])),
        ]
        out["front"] += [
            dict(tex, id="grip-crack", bone="body", z=4.2, region="grips-hdpe", material=MAT["grips-hdpe"], penMm=0.18, role="detail", points=[[-0.198, hy + 0.012], [-0.17, hy + 0.009], [-0.145, hy + 0.012], [-0.118, hy + 0.008]]),
            dict(tex, id="grip-chip-edge", bone="body", z=4.2, region="grips-hdpe", material=MAT["grips-hdpe"], penMm=0.18, role="detail", points=[[-GRIP_END, hy + GRIP_R * math.sin(0.15 * PI)], [-GRIP_END + CHIP_Z, hy + GRIP_R * math.sin(0.15 * PI)], [-GRIP_END + CHIP_Z, hy + GRIP_R]]),
            dict(tex, id="seat-wear", bone="body", z=4.0, region="seat-hdpe", material=MAT["seat-hdpe"], points=[[-0.06, 0.505], [0.0, 0.507], [0.06, 0.505]]),
        ]
        out["plan"] += [
            dict(tex, id="seat-wear", bone="body", z=4.0, region="seat-hdpe", material=MAT["seat-hdpe"], closed=True, points=[[-0.09, -0.05], [0.03, -0.055], [0.06, 0.0], [0.03, 0.055], [-0.09, 0.05], [-0.11, 0.0]]),
            dict(tex, id="grip-crack", bone="body", z=4.2, region="grips-hdpe", material=MAT["grips-hdpe"], penMm=0.18, role="detail", points=[[hx - 0.004, -0.198], [hx + 0.003, -0.17], [hx - 0.002, -0.145], [hx + 0.004, -0.118]]),
            dict(tex, id="grip-chip-edge", bone="body", z=4.2, region="grips-hdpe", material=MAT["grips-hdpe"], penMm=0.18, role="detail", points=[[hx + GRIP_R * math.cos(0.15 * PI), -GRIP_END], [hx + GRIP_R * math.cos(0.15 * PI), -GRIP_END + CHIP_Z], [hx + GRIP_R * math.cos(0.65 * PI), -GRIP_END + CHIP_Z], [hx + GRIP_R * math.cos(0.65 * PI), -GRIP_END]]),
            dict(tex, id="rust-stain", bone="footing", z=4.1, region="plates-steel", material=MAT["plates-steel"], closed=True, points=[[0.104, -0.128], [0.135, -0.13], [0.142, -0.105], [0.128, -0.096], [0.106, -0.104]]),
            dict(tex, id="concrete-crack", bone="footing", z=4.0, region="footing-concrete", material=MAT["footing-concrete"], points=[[0.16, 0.24], [0.19, 0.20], [0.205, 0.165], [0.235, 0.15]]),
        ]
        for s in out["elevation"] + out["front"] + out["plan"]:
            s.setdefault("role", "texture")
    return out


def representation(scale):
    """🗺 The authored representation of one scale: pens, hatches, the three views and the decisions."""
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
            200: {"body": 0.25, "detail": 0.18, "texture": None, "glazing": None, "ground": 0.35}}[scale]
    hatches = [{"id": "concrete", "title": T("Concrete C20/25", "Beton C20/25"), "pattern": "stipple", "angleDeg": 0, "spacingMm": {50: 0.8, 100: 0.8, 200: 0.6}[scale], "penMm": 0.1}]
    ps = parts(scale)
    geo = {v: [s for p in ps for s in p.shapes(v)] for v in ("elevation", "front", "plan")}
    extra = detail_strokes(scale)
    for v in geo:
        geo[v] += extra[v]
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350)}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": hatches, "geometry": geo, "decisions": REP_DECISIONS[scale]}


REP_DECISIONS = {
    50: {
        "silhouette": T("Side elevation from the right, 19 x 24.4 mm: the right HDPE horse panel with the chipped front edge of its ear and the eye hole, the left panel behind it, the seat board between the rims, the handle with the chipped right grip and its crack, the footrest with sleeve and end cap, the 6-turn coil spring between its plates drawn as the true side projection of its 20 mm wire, one closed band per half turn with the near halves over the far ones, and the four end clamps, the anchor plate with studs and nuts on the 0.50 x 0.40 m concrete point footing below ground. Front: both panels edge-on, seat and bottom board between them, handle and footrest through the panels with grips, sleeves and caps, the spring as its wire bands, plates, studs and nuts, the footing; plan: panels, seat board, grips, sleeves, caps, the spring ring, the plates with nuts and studs, the footing top.", "Seitenansicht von rechts, 19 x 24,4 mm: die rechte HDPE-Pferdeplatte mit der abgeplatzten Vorderkante des Ohrs und dem Augenloch, die linke Platte dahinter, das Sitzbrett zwischen den Rändern, der Griff mit dem abgebrochenen rechten Griffstück und seinem Riss, die Fußraste mit Hülse und Endkappe, die Schraubenfeder mit 6 Windungen zwischen ihren Platten als wahre Seitenprojektion ihres 20-mm-Drahts, ein geschlossenes Band je halbe Windung, die vorderen Hälften über den hinteren, und die vier Endklemmen, die Ankerplatte mit Gewindebolzen und Muttern auf dem Punktfundament 0,50 x 0,40 m aus Beton unter Gelände. Vorderansicht: beide Platten von der Kante, Sitz- und Bodenbrett dazwischen, Griff und Fußraste durch die Platten mit Griffstücken, Hülsen und Kappen, die Feder als ihre Drahtbänder, Platten, Bolzen und Muttern, das Fundament; Draufsicht: Platten, Sitzbrett, Griffstücke, Hülsen, Kappen, der Federring, die Platten mit Muttern und Bolzen, die Fundamentoberseite."),
        "retained": T("Every part of the rocker: footing, anchor plate, studs, nuts, lower and upper spring plates, the helix of the spring, clamps, bolts, both panels with eye holes, seat and bottom boards, handle tube, grips, footrest tube, sleeves and caps; traces of use: chipped ear, chipped and cracked grip, graffiti tag, sticker residue, scuffed rims, faded band, a rust stain on the anchor plate, a worn seat, a crack in the footing top.", "Jedes Teil der Wippe: Fundament, Ankerplatte, Gewindebolzen, Muttern, untere und obere Federplatte, die Wendel der Feder, Klemmen, Schrauben, beide Platten mit Augenlöchern, Sitz- und Bodenbrett, Griffrohr, Griffstücke, Fußrastenrohr, Hülsen und Kappen; Gebrauchsspuren: abgeplatztes Ohr, abgebrochenes und gerissenes Griffstück, Graffiti-Kürzel, Aufkleberreste, abgewetzte Ränder, ausgeblichenes Band, ein Rostfleck auf der Ankerplatte, ein abgenutzter Sitz, ein Riss in der Fundamentoberseite."),
        "merged": T("Nothing: every part is drawn on its own; the half turns of the spring are separate bands that overlap where the wire passes behind itself.", "Nichts: jedes Teil ist einzeln gezeichnet; die halben Windungen der Feder sind getrennte Bänder, die sich überdecken, wo der Draht hinter sich selbst verläuft."),
        "omitted": T("Routed 3 mm edge radii of the HDPE panels, thread pitch, washers under the nuts, the hidden anchor bars in the concrete and the screw heads of the boards.", "Gefräste Kantenradien 3 mm der HDPE-Platten, Gewindesteigung, Unterlegscheiben unter den Muttern, die verdeckten Ankerstäbe im Beton und die Schraubenköpfe der Bretter."),
        "thinElements": T("Wire bands of 20 mm (0.4 mm) as filled shapes, crack and chip edges 0.18 mm detail lines, traces of use 0.13 mm texture lines.", "Drahtbänder von 20 mm (0,4 mm) als gefüllte Formen, Riss- und Abplatzkanten 0,18-mm-Detaillinien, Gebrauchsspuren 0,13-mm-Texturlinien."),
        "hatching": T("Stipple on the concrete footing below ground.", "Punktierung auf dem Betonfundament unter Gelände."),
        "styles": T("Tones: concrete 0.86, plates 0.6, spring 0.27, panels 0.5 / 0.56, seat 0.4, grips 0.32, tubes 0.66; body pen 0.35, detail 0.18, texture 0.13.", "Töne: Beton 0,86, Platten 0,6, Feder 0,27, Platten 0,5 / 0,56, Sitz 0,4, Griffe 0,32, Rohre 0,66; Körperstift 0,35, Detail 0,18, Textur 0,13."),
        "overlays": T("Footing = footing-concrete; plates, studs, nuts, bolts, clamps = plates-steel; helix = spring-steel; panels = panels-hdpe; seat and bottom board = seat-hdpe; grips, sleeves, caps = grips-hdpe; handle and footrest tubes = tubes-steel.", "Fundament = footing-concrete; Platten, Bolzen, Muttern, Schrauben, Klemmen = plates-steel; Wendel = spring-steel; Pferdeplatten = panels-hdpe; Sitz- und Bodenbrett = seat-hdpe; Griffstücke, Hülsen, Kappen = grips-hdpe; Griff- und Fußrastenrohr = tubes-steel."),
    },
    100: {
        "silhouette": T("Side elevation 9.5 x 12.2 mm: the two panels with eye holes (the right one without the 13 mm ear chip, below the 15 mm feature), seat board, handle and grips, footrest and sleeves, the spring as its wire bands, the three plates and the footing. Front and plan: the same parts as at 1:50 without studs, nuts, bolts, clamps and caps.", "Seitenansicht 9,5 x 12,2 mm: die zwei Platten mit Augenlöchern (die rechte ohne die 13-mm-Abplatzung am Ohr, unter dem Merkmal 15 mm), Sitzbrett, Griff und Griffstücke, Fußraste und Hülsen, die Feder als ihre Drahtbänder, die drei Platten und das Fundament. Vorder- und Draufsicht: dieselben Teile wie bei 1:50 ohne Bolzen, Muttern, Schrauben, Klemmen und Kappen."),
        "retained": T("Footing, anchor plate, both spring plates, the helix of 20 mm wire, both panels with eye holes, seat and bottom boards, handle tube and grips, footrest tube and sleeves.", "Fundament, Ankerplatte, beide Federplatten, die Wendel aus 20-mm-Draht, beide Platten mit Augenlöchern, Sitz- und Bodenbrett, Griffrohr und Griffstücke, Fußrastenrohr und Hülsen."),
        "merged": T("The end caps are merged into the footrest sleeves, which run to the full 0.20 m; the chipped grip end is drawn whole.", "Die Endkappen gehen in den Fußrastenhülsen auf, die bis zu vollen 0,20 m reichen; das abgebrochene Griffende ist ganz gezeichnet."),
        "omitted": T("Studs, nuts, bolt heads and clamps (8 to 24 mm, compact parts below or at the 15 mm feature), the ear chip, crack lines and all texture traces.", "Gewindebolzen, Muttern, Schraubenköpfe und Klemmen (8 bis 24 mm, kompakte Teile unter oder am Merkmal 15 mm), die Abplatzung am Ohr, Risslinien und alle Texturspuren."),
        "thinElements": T("Plates of 8 and 10 mm as declared sheets; the 20 mm wire as filled bands (0.2 mm).", "Platten von 8 und 10 mm als erklärte Bleche; der 20-mm-Draht als gefüllte Bänder (0,2 mm)."),
        "hatching": T("Stipple on the footing.", "Punktierung auf dem Fundament."),
        "styles": T("Tones as at 1:50; body pen 0.25, detail 0.13.", "Töne wie bei 1:50; Körperstift 0,25, Detail 0,13."),
        "overlays": T("Regions as at 1:50.", "Regionen wie bei 1:50."),
    },
    200: {
        "silhouette": T("Side elevation 4.8 x 6.1 mm: the horse outline of 22 points, seat block, handle and footrest ends, the spring as its enveloping cylinder in three stacked pieces between the plates, the footing. Front: panels, seat and bottom blocks, handle and footrest bars, spring cylinder, plates, footing; plan: panels, seat, bars, plates, footing.", "Seitenansicht 4,8 x 6,1 mm: der Pferdeumriss aus 22 Punkten, Sitzblock, Griff- und Fußrastenenden, die Feder als ihr Hüllzylinder in drei gestapelten Stücken zwischen den Platten, das Fundament. Vorderansicht: Platten, Sitz- und Bodenblock, Griff- und Fußrastenstäbe, Federzylinder, Platten, Fundament; Draufsicht: Platten, Sitz, Stäbe, Platten, Fundament."),
        "retained": T("Footing, the three plates, the spring envelope, both panels, seat, bottom block, handle and footrest.", "Fundament, die drei Platten, die Federhülle, beide Platten, Sitz, Bodenblock, Griff und Fußraste."),
        "merged": T("The 20 mm helix (below the 30 mm feature) is typified as its enveloping cylinder of 0.20 m; the seat board and the 21 mm rims above it are one 40 mm block, the bottom board a 30 mm block; tube and grips are one 34 mm handle bar, tube and sleeves one footrest bar (34 / 40 mm).", "Die 20-mm-Wendel (unter dem Merkmal 30 mm) ist als ihr Hüllzylinder von 0,20 m typisiert; das Sitzbrett und die 21 mm hohen Ränder darüber sind ein 40-mm-Block, das Bodenbrett ein 30-mm-Block; Rohr und Griffstücke sind ein Griffstab von 34 mm, Rohr und Hülsen ein Fußrastenstab (34 / 40 mm)."),
        "omitted": T("Eye holes (30 mm), the ear chip, coil lines, hardware and traces of use.", "Augenlöcher (30 mm), die Abplatzung am Ohr, Windungslinien, Kleinteile und Gebrauchsspuren."),
        "thinElements": T("Panels of 19 mm and plates of 8 and 10 mm as declared sheets.", "Platten von 19 mm und Stahlplatten von 8 und 10 mm als erklärte Bleche."),
        "hatching": T("Stipple on the footing.", "Punktierung auf dem Fundament."),
        "styles": T("Tones as at 1:50; body pen 0.25, detail 0.18.", "Töne wie bei 1:50; Körperstift 0,25, Detail 0,18."),
        "overlays": T("Regions as at 1:50; the spring cylinder carries spring-steel.", "Regionen wie bei 1:50; der Federzylinder trägt spring-steel."),
    },
}


def structure_list(scale, ps):
    """🏗 The structural idealisation of one scale: members along the bones, joints at the rig joints, the support at the footing anchor and the masses at their centres."""
    feat = {50: 0.0075, 100: 0.015, 200: 0.03}[scale]
    byid = {p.id: p for p in ps}
    out = [{"id": "st-footing", "of": "footing-block", "member": "footing", "part": "member"},
           {"id": "st-anchor-joint", "of": "anchor-plate", "member": "footing", "part": "joint"},
           {"id": "st-base-joint", "of": "lower-plate", "member": "spring-1", "part": "joint"}]
    for k in ("1", "2", "3"):
        out.append({"id": f"st-coil-{k}", "of": f"coil-{k}", "member": f"spring-{k}", "part": "member"})
    for k, yj in (("2", YJ1), ("3", YJ2)):
        c = (0.0, yj, COIL_R) if scale <= 100 else (0.0, yj, 0.0)
        pj = Part(f"st-joint-{k}", f"spring-{k}", "ellipsoid", (c, (feat * 1.6, feat * 1.6, feat * 1.6)), MAT["spring-steel"], "spring-steel", 0.2, role="detail")
        out.append(dict(pj.solid(), member=f"spring-{k}", part="joint"))
    out += [{"id": "st-top-joint", "of": "upper-plate", "member": "spring-3", "part": "joint"},
            {"id": "st-body-joint", "of": "bottom-board", "member": "body", "part": "joint"},
            {"id": "st-panel-left", "of": "panel-left", "member": "body", "part": "member"},
            {"id": "st-panel-right", "of": "panel-right", "member": "body", "part": "member"},
            {"id": "st-handle", "of": "handle-tube", "member": "body", "part": "member"},
            {"id": "st-footrest", "of": "footrest-tube", "member": "body", "part": "member"}]
    sup = Part("st-support-footing", "footing", "box", ((-2 * feat, -2 * feat, -2 * feat), (2 * feat, 0.0, 2 * feat)), MAT["footing-concrete"], "footing-concrete", 0.15)
    out.append(dict(sup.solid(), member="footing", part="support", anchor="footing-ground"))
    for bone, (kg, com) in MASSES.items():
        if kg <= 0:
            continue
        mp = Part(f"st-mass-{bone}", bone, "ellipsoid", ((com[0], com[1], 0.0), (feat * 1.4, feat * 1.4, feat * 1.4)), MAT["plates-steel"], "plates-steel", 0.2, role="detail")
        out.append(dict(mp.solid(), member=bone if bone != "lift" else "spring-1", part="mass"))
    _ = byid
    return out


def bone_masses():
    """⚖ Mass and world centre of every bone from the 1:50 parts (section x density)."""
    acc = {}
    for p in parts(50):
        m = p.mass()
        c = p.volume_area_com()[2]
        kg, sx, sy = acc.get(p.bone, (0.0, 0.0, 0.0))
        acc[p.bone] = (kg + m, sx + m * c[0], sy + m * c[1])
    return {b: (kg, (sx / kg, sy / kg)) for b, (kg, sx, sy) in acc.items()}


MASSES = bone_masses()
MASS_TOTAL = sum(kg for kg, _ in MASSES.values())


def region_areas(scale):
    """🌡 Outer area of every region at one scale from its parts."""
    acc = {}
    for p in parts(scale):
        if p.extra.get("envelope"):
            continue
        acc[p.region] = acc.get(p.region, 0.0) + p.area()
    return acc


AREAS = region_areas(50)


def volume(scale):
    """🧊 The authored volume of one scale."""
    ps = parts(scale)
    body = [p.solid() for p in ps]
    thermal = [{"id": f"th-{p.id}", "of": p.id, "region": p.region} for p in ps]
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure_list(scale, ps), "decisions": VOL_DECISIONS[scale]}


VOL_DECISIONS = {
    50: [T("Body at 1:50 (feature 7.5 mm), every part as built: the 0.50 x 0.50 x 0.40 m concrete point footing (top flush with the lawn), the 300 x 300 x 10 mm anchor plate on it with four M16 studs, the 240 x 240 x 8 mm lower spring plate with four M16 nuts (24 mm across flats), the coil spring as a true helix of 20 mm round wire, 6 turns of 90 mm mean radius and 46.3 mm pitch in three lofts that meet where the helix crosses the bending axis of a rig joint (so the wire stays closed when the spring bends), four end clamps, the 200 x 200 x 8 mm upper plate with four M10 bolt heads, two 19 mm HDPE horse panels 160 mm apart with eye holes, the right one with the chipped front edge of its ear, seat and bottom boards of 19 mm HDPE, the o26.9 handle and footrest tubes through both panels, two o34 grips (the right one chipped: a quarter of its outer 30 mm broken away), two o40 footrest sleeves with o50 end caps.", "Körper 1:50 (Merkmal 7,5 mm), jedes Teil wie gebaut: das Punktfundament 0,50 x 0,50 x 0,40 m aus Beton (Oberseite bündig mit dem Rasen), darauf die Ankerplatte 300 x 300 x 10 mm mit vier Gewindebolzen M16, die untere Federplatte 240 x 240 x 8 mm mit vier Muttern M16 (Schlüsselweite 24 mm), die Schraubenfeder als echte Wendel aus 20-mm-Runddraht, 6 Windungen mit 90 mm mittlerem Radius und 46,3 mm Steigung in drei Lofts, die sich dort treffen, wo die Wendel die Biegeachse eines Rig-Gelenks kreuzt (so bleibt der Draht beim Biegen geschlossen), vier Endklemmen, die obere Platte 200 x 200 x 8 mm mit vier Schraubenköpfen M10, zwei Pferdeplatten aus 19-mm-HDPE im Abstand 160 mm mit Augenlöchern, die rechte mit der abgeplatzten Vorderkante des Ohrs, Sitz- und Bodenbrett aus 19-mm-HDPE, Griff- und Fußrastenrohr o26,9 durch beide Platten, zwei Griffstücke o34 (das rechte abgebrochen: ein Viertel seiner äußeren 30 mm fehlt), zwei Fußrastenhülsen o40 mit Endkappen o50."),
         T("Thermal: every body solid by reference in its region; seven single-layer regions, no emitters, no hidden regions. Structure: the footing as member with its fixed support at ground level, the plates as joints, the three helix lofts as members of the three spring bones with joint markers where they meet, the panels, the handle and the footrest tube as members of the body, mass markers at the centres of mass of every bone.", "Thermik: jeder Körper als Referenz in seiner Region; sieben einschichtige Regionen, keine Emitter, keine verdeckten Regionen. Tragwerk: das Fundament als Stab mit seiner Einspannung in Geländehöhe, die Platten als Gelenke, die drei Wendel-Lofts als Stäbe der drei Federknochen mit Gelenkmarken, wo sie sich treffen, die Platten, das Griff- und das Fußrastenrohr als Stäbe des Körpers, Massenmarken in den Schwerpunkten jedes Knochens.")],
    100: [T("Body at 1:100 (feature 15 mm), authored anew: the helix keeps its true 20 mm wire with 16 sections per turn; the three steel plates are declared sheets (8 and 10 mm, 25 to 30 times as broad); the eye holes (30 mm) stay; studs, nuts, bolt heads and clamps are left out as compact parts at or below the feature; the footrest end caps merge into the sleeves, which run to 0.20 m; the chipped grip end and the ear chip (13 mm, below the 15 mm feature) are drawn whole.", "Körper 1:100 (Merkmal 15 mm), neu erstellt: die Wendel behält ihren wahren 20-mm-Draht mit 16 Schnitten je Windung; die drei Stahlplatten sind erklärte Bleche (8 und 10 mm, 25- bis 30-mal so breit); die Augenlöcher (30 mm) bleiben; Gewindebolzen, Muttern, Schraubenköpfe und Klemmen entfallen als kompakte Teile am oder unter dem Merkmal; die Endkappen der Fußrasten gehen in den Hülsen auf, die bis 0,20 m reichen; das abgebrochene Griffende und die Abplatzung am Ohr (13 mm) sind ganz gezeichnet."),
          T("Thermal by reference as at 1:50; structure as at 1:50 without the hardware.", "Thermik als Referenz wie bei 1:50; Tragwerk wie bei 1:50 ohne Kleinteile.")],
    200: [T("Body at 1:200 (feature 30 mm), authored anew: the spring is typified as its enveloping cylinder of 0.20 m diameter in three stacked envelope pieces, one per spring bone (the 20 mm wire is below the feature); the lower plate stays on the lift bone and the plates stay declared sheets; the panels keep their 19 mm as declared sheets with a horse outline of 22 points and no eye holes; the seat board and the rims above it are one 40 mm block, the bottom board a 30 mm block; tube and grips are one 34 mm bar, tube and sleeves one bar of 34 / 40 mm.", "Körper 1:200 (Merkmal 30 mm), neu erstellt: die Feder ist als ihr Hüllzylinder von 0,20 m Durchmesser in drei gestapelten Hüllstücken typisiert, eines je Federknochen (der 20-mm-Draht liegt unter dem Merkmal); die untere Platte bleibt am Hubknochen und die Platten bleiben erklärte Bleche; die Pferdeplatten behalten ihre 19 mm als erklärte Bleche mit einem Pferdeumriss aus 22 Punkten und ohne Augenlöcher; Sitzbrett und Ränder darüber sind ein 40-mm-Block, das Bodenbrett ein 30-mm-Block; Rohr und Griffstücke sind ein Stab von 34 mm, Rohr und Hülsen ein Stab von 34 / 40 mm."),
          T("Thermal by reference; the envelope cylinders give the spring region its coverage and stay out of the area sums. Structure: footing, envelope pieces as spring members with joint markers on the axis, panels, bars, plates as joints, support and masses.", "Thermik als Referenz; die Hüllzylinder geben der Federregion ihre Abdeckung und bleiben aus den Flächensummen. Tragwerk: Fundament, Hüllstücke als Federstäbe mit Gelenkmarken auf der Achse, Platten, Stäbe, Platten als Gelenke, Auflager und Massen.")],
}


def rocking():
    """🎠 Bending stiffness of the coil spring (Wahl), net rocking stiffness with the toddler and the natural period of the rocking clip."""
    d, D, n = WIRE_D, 2 * COIL_R, 5
    k_spring = E_SPRING * d ** 4 / (32 * D * n * (1 + (1 + NU_SPRING)))
    toddler, h_toddler = 14.0, 0.70
    m_up = MASSES["body"][0] + MASSES["spring-3"][0] + MASSES["spring-2"][0] / 2
    pivot = (Y_LP + Y_UPB) / 2
    com_up = (MASSES["body"][0] * MASSES["body"][1][1] + MASSES["spring-3"][0] * 0.26) / (MASSES["body"][0] + MASSES["spring-3"][0])
    k_g = G * (toddler * (h_toddler - pivot) + m_up * (com_up - pivot))
    inertia = toddler * ((h_toddler - pivot) ** 2 + 0.12 ** 2) + MASSES["body"][0] * ((com_up - pivot) ** 2 + 0.2 ** 2)
    k_net = k_spring - k_g
    period = 2 * PI * math.sqrt(inertia / k_net)
    return {"kSpring": k_spring, "kGravity": k_g, "kNet": k_net, "inertia": inertia, "period": period, "pivot": pivot}


ROCK = rocking()
PERIOD = round(ROCK["period"], 3)
TODDLER_N = round(14.0 * G, 1)


def region_bind(pose):
    """🌞 Region bindings of one pose: exposure, sun and contact."""
    seat_contact = {"fraction": 0.35, "tempC": 33.0} if pose.startswith("in-use") else None
    grip_contact = {"fraction": 0.2, "tempC": 32.0} if pose.startswith("in-use") else None
    soil = {"fraction": 0.62, "tempC": 15.0} if pose != "maintenance" else {"fraction": 0.62, "tempC": 15.0}
    out = {
        "footing-concrete": {"exposedFraction": 0.19, "sunlitFraction": 0.6, "contact": soil},
        "plates-steel": {"exposedFraction": 0.75 if pose != "maintenance" else 0.85, "sunlitFraction": 0.35},
        "spring-steel": {"exposedFraction": 1, "sunlitFraction": 0.35 if pose != "maintenance" else 0.5},
        "panels-hdpe": {"exposedFraction": 1, "sunlitFraction": 0.5 if not pose.startswith("in-use") else 0.4},
        "seat-hdpe": {"exposedFraction": 1 if not pose.startswith("in-use") else 0.65, "sunlitFraction": 0.45 if not pose.startswith("in-use") else 0.15},
        "grips-hdpe": {"exposedFraction": 1 if not pose.startswith("in-use") else 0.8, "sunlitFraction": 0.5},
        "tubes-steel": {"exposedFraction": 1, "sunlitFraction": 0.3},
    }
    if seat_contact:
        out["seat-hdpe"]["contact"] = seat_contact
        out["grips-hdpe"]["contact"] = grip_contact
    return out


SUPPORTS = ["footing-ground"]
LIFTS = ["lift-handle-left", "lift-handle-right", "lift-rear-left", "lift-rear-right"]
REAR_SLING_X = -0.1205
HANDLE_SHARE = 0.378


def sling_loads():
    """🏗️ The crane hook as applied lifting forces on the four sling legs: the handle pair at x = 0.27 m carries 37.8 % and the rear pair at x = -0.1205 m 62.2 % of the lifted rocker and spring, whose centre of mass lies at x = 0.027 m (statics of `../../lift_statics.py`)."""
    weight = (MASS_TOTAL - MASSES["footing"][0]) * G
    out = []
    for end, share, en, de in (("handle", HANDLE_SHARE, "handle end", "Griffende"), ("rear", 1 - HANDLE_SHARE, "rear spacer", "hinteren Distanzstück")):
        for side, sde in (("left", "linken"), ("right", "rechten")):
            out.append({"id": f"sling-{end}-{side}", "kind": "equipment", "title": T(f"Sling leg at the {side} {en}", f"Gehängestrang am {sde} {de}"), "anchor": f"lift-{end}-{side}", "fxN": 0, "fyN": round(weight * share / 2, 1), "source": "authored-estimate", "reference": "en-1492-2"})
    return out


def loads_in_use(push):
    """🧒 The toddler's loads: weight on the seat, hands pulling on the handle."""
    out = [{"id": "toddler-seated", "kind": "occupant", "title": T("Toddler of 14 kg on the seat", "Kleinkind mit 14 kg auf dem Sitz"), "anchor": "seat-point", "fxN": 0, "fyN": -TODDLER_N, "source": "authored-estimate", "reference": "who-growth-2006"}]
    if push:
        out += [{"id": f"hand-pull-{s}", "kind": "live", "title": T(f"Hand pull on the {s} grip (30 N back)", f"Handzug am {'linken' if s == 'left' else 'rechten'} Griff (30 N nach hinten)"), "anchor": f"grip-{s}-point", "fxN": -30, "fyN": 0, "source": "authored-estimate"} for s in ("left", "right")]
    return out


def poses():
    """🧍 The named poses: in service, lifted for the spring change, rocked forward and back by the toddler."""
    seg = ROCK_DEG / 3
    return [
        {"id": "in-service", "title": T("In service", "In Betrieb"), "description": T("Standing on its concrete point footing in the lawn, upright and unloaded in the summer sun.", "Auf seinem Punktfundament aus Beton im Rasen, aufrecht und unbelastet in der Sommersonne."),
         "bones": {}, "ground": "auto", "contacts": SUPPORTS, "thermal": {"regions": region_bind("in-service"), "emitters": {}}, "loads": []},
        {"id": "maintenance", "title": T("Lifted for the spring change", "Für den Federwechsel angehoben"), "description": T("The depot crew has taken the four M16 nuts off and lifts rocker and corroded spring 0.30 m off the anchor plate on a four-leg sling, two legs at the handle ends and two choked round the rear spacer under the seat, so that the hook stands over the centre of mass; the footing with its anchor plate and studs stays in the lawn.", "Der Bauhof hat die vier Muttern M16 abgenommen und hebt Wippe und korrodierte Feder an einem Vierstranggehänge, zwei Stränge an den Griffenden und zwei um das hintere Distanzstück unter dem Sitz geschnürt, sodass der Haken über dem Schwerpunkt steht, 0,30 m von der Ankerplatte; das Fundament mit Ankerplatte und Gewindebolzen bleibt im Rasen."),
         "bones": {"lift": {"dx": LIFT_DY}}, "ground": "auto", "contacts": SUPPORTS, "thermal": {"regions": region_bind("maintenance"), "emitters": {}}, "loads": sling_loads()},
        {"id": "slings-taut", "key": True, "title": T("Sling taut", "Gehänge gespannt"), "description": T("Key of the maintenance clip: the four sling legs carry rocker and spring before they leave the anchor plate; the soil carries the footing.", "Schlüssel des Wartungs-Clips: die vier Gehängestränge tragen Wippe und Feder, bevor sie die Ankerplatte verlassen; der Boden trägt das Fundament."),
         "bones": {}, "ground": "auto", "contacts": SUPPORTS, "thermal": {"regions": region_bind("in-service"), "emitters": {}}, "loads": sling_loads()},
        {"id": "in-use", "title": T("Toddler rocking forward", "Kleinkind wippt nach vorn"), "description": T(f"A toddler of 14 kg rocks forward to the turning point: the spring bends {ROCK_DEG:.0f} degrees over its three segments, the child pulls on the grips.", f"Ein Kleinkind mit 14 kg wippt bis zum vorderen Umkehrpunkt: die Feder biegt sich über ihre drei Abschnitte um {ROCK_DEG:.0f} Grad, das Kind zieht an den Griffen."),
         "bones": {f"spring-{k}": {"angleDeg": -seg} for k in (1, 2, 3)}, "ground": "auto", "contacts": SUPPORTS, "thermal": {"regions": region_bind("in-use"), "emitters": {}}, "loads": loads_in_use(True)},
        {"id": "in-use-back", "title": T("Toddler rocking back", "Kleinkind wippt nach hinten"), "description": T(f"The rear turning point of the same rocking: the spring bent {ROCK_DEG:.0f} degrees backwards, the child sitting upright.", f"Der hintere Umkehrpunkt desselben Wippens: die Feder um {ROCK_DEG:.0f} Grad nach hinten gebogen, das Kind sitzt aufrecht."),
         "bones": {f"spring-{k}": {"angleDeg": seg} for k in (1, 2, 3)}, "ground": "auto", "contacts": SUPPORTS, "thermal": {"regions": region_bind("in-use-back"), "emitters": {}}, "loads": loads_in_use(False)},
    ]


def clips():
    """🎞 The maintenance lift and the toddler's rocking."""
    seg = ROCK_DEG / 3
    keys = []
    for i in range(9):
        t = PERIOD * i / 8
        a = -seg * math.sin(2 * PI * i / 8)
        keys.append({"t": round(t, 4), "pose": "in-use", "ease": "smooth", "bones": {f"spring-{k}": {"angleDeg": round(a + 0.0, 4)} for k in (1, 2, 3)}})
    rock = {"id": "rocking", "title": T("Toddler rocking", "Kleinkind wippt"), "description": T(f"One period T = {PERIOD} s of rocking forward and back by {ROCK_DEG:.0f} degrees, theta(t) = -{ROCK_DEG:.0f} sin(2 pi t / T) shared equally by the three spring segments, looping; T from the bending stiffness of the spring less the toppling moment of child and rocker.", f"Eine Periode T = {PERIOD} s Wippen vor und zurück um {ROCK_DEG:.0f} Grad, theta(t) = -{ROCK_DEG:.0f} sin(2 pi t / T) zu gleichen Teilen auf die drei Federabschnitte verteilt, wiederkehrend; T aus der Biegesteifigkeit der Feder abzüglich des Kippmoments von Kind und Wippe."),
            "durationS": PERIOD, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys, "contacts": [{"anchor": "footing-ground", "fromS": 0, "toS": PERIOD}],
            "phases": [{"id": "forward-half", "fromS": 0, "toS": round(PERIOD / 2, 4), "title": T("Forward half", "Vordere Hälfte")}, {"id": "back-half", "fromS": round(PERIOD / 2, 4), "toS": PERIOD, "title": T("Back half", "Hintere Hälfte")}],
            "events": [{"t": round(PERIOD / 4, 4), "kind": "motion", "title": T("Forward turning point", "Vorderer Umkehrpunkt")}, {"t": round(3 * PERIOD / 4, 4), "kind": "motion", "title": T("Rear turning point", "Hinterer Umkehrpunkt")}], "tracks": []}
    maint = {"id": "maintenance", "title": T("Lift for the spring change", "Anheben für den Federwechsel"), "description": T("The crew takes the four nuts off, tensions the four-leg sling and lifts rocker and spring 0.30 m off the anchor plate in 4.5 s; the footing stays.", "Der Trupp nimmt die vier Muttern ab, spannt das Vierstranggehänge und hebt Wippe und Feder in 4,5 s 0,30 m von der Ankerplatte; das Fundament bleibt."),
             "durationS": 6.0, "fps": 30, "loop": False, "rootMotion": "none",
             "keys": [{"t": 0, "pose": "in-service", "ease": "linear"}, {"t": 1.0, "pose": "in-service", "ease": "smooth"}, {"t": 1.5, "pose": "slings-taut", "ease": "smooth"}, {"t": 6.0, "pose": "maintenance", "ease": "smooth"}],
             "contacts": [{"anchor": "footing-ground", "fromS": 0, "toS": 6.0}],
             "phases": [{"id": "unbolt", "fromS": 0, "toS": 1.5, "title": T("Nuts off, sling tensioned", "Muttern ab, Gehänge gespannt")}, {"id": "lift", "fromS": 1.5, "toS": 6.0, "title": T("Lift", "Heben")}],
             "events": [{"t": 1.5, "kind": "load-on", "title": T("Sling takes the load", "Gehänge übernimmt die Last"), "target": "sling-handle-left"}, {"t": 6.0, "kind": "motion", "title": T("Lifted 0.30 m", "0,30 m angehoben")}], "tracks": []}
    return [maint, rock]


def anchors():
    """📍 Support, lifting and load anchors."""
    hx, hy = HANDLE
    fx, fy = FOOTREST
    b = lambda x, y: loc("body", x, y)
    out = [{"id": "footing-ground", "bone": "footing", "x": FOOT_DEPTH, "y": 0, "kind": "support", "title": T("Concrete point footing fixed in the soil at ground level", "Punktfundament aus Beton in Geländehöhe im Boden eingespannt"), "support": "fixed", "footprintLengthM": 2 * FOOT_HALF, "footprintWidthM": 2 * FOOT_HALF}]
    for side, z in (("left", 1), ("right", -1)):
        u, v, _ = b(hx, hy)
        out.append({"id": f"lift-handle-{side}", "bone": "body", "x": r6(u), "y": r6(v), "z": r6(z * 0.15), "kind": "load", "title": T(f"Sling leg at the {side} handle end", f"Gehängestrang am {'linken' if side == 'left' else 'rechten'} Griffende")})
    for side, z in (("left", 1), ("right", -1)):
        u, v, _ = b(REAR_SLING_X, SEAT[2])
        out.append({"id": f"lift-rear-{side}", "bone": "body", "x": r6(u), "y": r6(v), "z": r6(z * PANEL_IN), "kind": "load", "title": T(f"Sling leg choked round the rear spacer under the seat, {side} side", f"Gehängestrang um das hintere Distanzstück unter dem Sitz geschnürt, {'links' if side == 'left' else 'rechts'}")})
    u, v, _ = b(-0.015, SEAT[3])
    out.append({"id": "seat-point", "bone": "body", "x": r6(u), "y": r6(v), "kind": "load", "title": T("Seat point of the toddler", "Sitzpunkt des Kleinkinds")})
    for side, z in (("left", 1), ("right", -1)):
        u, v, _ = b(hx, hy)
        out.append({"id": f"grip-{side}-point", "bone": "body", "x": r6(u), "y": r6(v), "z": r6(z * 0.15), "kind": "load", "title": T(f"Hand on the {side} grip", f"Hand am {'linken' if side == 'left' else 'rechten'} Griff")})
    return out


def rig():
    """🦴 The rig: footing, lift carrier of the unbolted rocker, three spring segments, body."""
    titles = {"footing": T("Concrete point footing with anchor plate", "Punktfundament aus Beton mit Ankerplatte"), "lift": T("Lower spring plate (lifted with the rocker)", "Untere Federplatte (mit der Wippe gehoben)"),
              "spring-1": T("Spring, lower two turns", "Feder, untere zwei Windungen"), "spring-2": T("Spring, middle two turns", "Feder, mittlere zwei Windungen"), "spring-3": T("Spring, upper two turns and upper plate", "Feder, obere zwei Windungen und obere Platte"),
              "body": T("Horse body: panels, boards, handle, footrest", "Pferdekörper: Platten, Bretter, Griff, Fußraste")}
    joints = {"footing": {"type": "fixed"}, "lift": {"type": "prismatic", "minM": 0, "maxM": 0.4, "axisDeg": 0}, "spring-1": {"type": "revolute", "minDeg": -8, "maxDeg": 8},
              "spring-2": {"type": "revolute", "minDeg": -8, "maxDeg": 8}, "spring-3": {"type": "revolute", "minDeg": -8, "maxDeg": 8}, "body": {"type": "fixed"}}
    wire = {"areaM2": r6(PI * WIRE_D ** 2 / 4), "secondMomentM4": float(f"{PI * WIRE_D ** 4 / 64:.4e}"), "extremeFibreM": WIRE_D / 2, "material": "spring-steel-painted", "title": T("Spring wire o20 (bending stress in the wire)", "Federdraht o20 (Biegespannung im Draht)")}
    sections = {"footing": {"areaM2": 0.25, "secondMomentM4": float(f"{0.5 ** 4 / 12:.4e}"), "extremeFibreM": 0.25, "material": "concrete-c20-25", "title": T("Concrete footing 0.50 x 0.50 m", "Betonfundament 0,50 x 0,50 m")},
                "spring-1": wire, "spring-2": wire, "spring-3": wire,
                "body": {"areaM2": r6(2 * PANEL_T * 0.21), "secondMomentM4": float(f"{2 * PANEL_T * 0.21 ** 3 / 12:.4e}"), "extremeFibreM": 0.105, "material": "hdpe-red-faded", "title": T("Two HDPE panels 19 x 210 mm at the seat", "Zwei HDPE-Platten 19 x 210 mm am Sitz")}}
    refs = {"footing": "en-1991-1-1", "lift": "en-10025-2", "spring-1": "en-10089", "spring-2": "en-10089", "spring-3": "en-10089", "body": "ashby-2011"}
    out = []
    for bid, parent, x, y, ang, length in BONES:
        kg, (cx, cy) = MASSES[bid]
        u, v, _ = loc(bid, cx, cy)
        bone = {"id": bid, "parent": parent, "title": titles[bid], "x": r6(x), "y": r6(y), "angleDeg": ang, "lengthM": r6(length), "joint": joints[bid],
                "mass": {"kg": round(kg, 3), "comX": r6(u), "comY": r6(v), "distribution": "point", "source": "literature", "reference": refs[bid]}}
        if bid in sections:
            bone["section"] = sections[bid]
        out.append(bone)
    return {"kind": "mechanical", "root": "footing", "bones": out}


def materials():
    """🧱 The materials, weathered where the surface has aged."""
    return [
        {"id": "concrete-c20-25", "title": T("Concrete C20/25, weathered top", "Beton C20/25, verwitterte Oberseite"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 2400, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "elasticModulusPa": 30000000000, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "steel-galvanised-weathered", "title": T("Hot-dip galvanised steel, dull and weathered", "Feuerverzinkter Stahl, matt und verwittert"), "emissivity": 0.45, "solarAbsorptivity": 0.68, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "source": "authored-estimate", "reference": "berdahl-2008"},
        {"id": "spring-steel-painted", "title": T("Spring steel 51CrV4, painted green, flaking and rusty", "Federstahl 51CrV4, grün lackiert, abblätternd und rostig"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": 7850, "conductivityWmK": 46, "specificHeatJKgK": 470, "elasticModulusPa": 206000000000, "source": "authored-estimate", "reference": "en-10089"},
        {"id": "hdpe-red-faded", "title": T("HDPE 19 mm, red, chalked and faded by UV", "HDPE 19 mm, rot, durch UV gekreidet und ausgeblichen"), "emissivity": 0.92, "solarAbsorptivity": 0.55, "densityKgM3": 955, "conductivityWmK": 0.46, "specificHeatJKgK": 1850, "elasticModulusPa": 1000000000, "source": "authored-estimate", "reference": "berdahl-2008"},
        {"id": "hdpe-blue-faded", "title": T("HDPE 19 mm, blue, faded", "HDPE 19 mm, blau, ausgeblichen"), "emissivity": 0.92, "solarAbsorptivity": 0.72, "densityKgM3": 955, "conductivityWmK": 0.46, "specificHeatJKgK": 1850, "elasticModulusPa": 1000000000, "source": "authored-estimate", "reference": "berdahl-2008"},
        {"id": "hdpe-green-faded", "title": T("HDPE grips, green, faded and embrittled", "HDPE-Griffe, grün, ausgeblichen und versprödet"), "emissivity": 0.92, "solarAbsorptivity": 0.62, "densityKgM3": 955, "conductivityWmK": 0.46, "specificHeatJKgK": 1850, "elasticModulusPa": 1000000000, "source": "authored-estimate", "reference": "berdahl-2008"},
    ]


def layer(lid, title, kind, material, t, k, rho, cp, t_eff, ref):
    """🧶 One layer with resistance and areal heat capacity."""
    return {"id": lid, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": float(f"{t / k:.4g}"), "source": "literature", "heatCapacityJM2K": round(rho * cp * t_eff), "reference": ref}


def thermal():
    """🌡 The thermal model: a summer afternoon in the park, seven passive regions."""
    spec = [
        ("footing-concrete", T("Concrete footing (top in the lawn, sides and base in the soil)", "Betonfundament (Oberseite im Rasen, Seiten und Sohle im Boden)"), "concrete-c20-25", 0.5, "horizontal-plate-up", 0.9,
         layer("concrete", T("Concrete C20/25 (55 mm active in one hour)", "Beton C20/25 (55 mm aktiv in einer Stunde)"), "masonry", "concrete-c20-25", 0.40, 2.0, 2400, 1000, math.sqrt(2.0 / (2400 * 1000) * 3600), "iso-10456-2007")),
        ("plates-steel", T("Galvanised anchor plate, spring plates, studs, nuts, bolts and clamps", "Verzinkte Ankerplatte, Federplatten, Bolzen, Muttern, Schrauben und Klemmen"), "steel-galvanised-weathered", 0.24, "horizontal-plate-up", 0.7,
         layer("plate", T("Steel plate 8 to 10 mm", "Stahlplatte 8 bis 10 mm"), "metal", "steel-galvanised-weathered", 0.009, 50, 7850, 460, 0.0045, "iso-10456-2007")),
        ("spring-steel", T("Coil spring, painted, flaking and rusty", "Schraubenfeder, lackiert, abblätternd und rostig"), "spring-steel-painted", 0.02, "cylinder", 0.5,
         layer("wire", T("Spring wire o20 (5 mm per unit area)", "Federdraht o20 (5 mm je Flächeneinheit)"), "metal", "spring-steel-painted", 0.005, 46, 7850, 470, 0.005, "en-10089")),
        ("panels-hdpe", T("Red HDPE horse panels, faded", "Rote HDPE-Pferdeplatten, ausgeblichen"), "hdpe-red-faded", 0.5, "vertical-plate", 0.5,
         layer("hdpe", T("HDPE 19 mm (9.5 mm per face)", "HDPE 19 mm (9,5 mm je Seite)"), "polymer", "hdpe-red-faded", 0.019, 0.46, 955, 1850, 0.0095, "ashby-2011")),
        ("seat-hdpe", T("Blue HDPE seat and bottom board", "Blaues HDPE-Sitz- und Bodenbrett"), "hdpe-blue-faded", 0.2, "horizontal-plate-up", 0.85,
         layer("hdpe", T("HDPE 19 mm (9.5 mm per face)", "HDPE 19 mm (9,5 mm je Seite)"), "polymer", "hdpe-blue-faded", 0.019, 0.46, 955, 1850, 0.0095, "ashby-2011")),
        ("grips-hdpe", T("Green HDPE grips and footrest sleeves", "Grüne HDPE-Griffe und Fußrastenhülsen"), "hdpe-green-faded", 0.034, "cylinder", 0.6,
         layer("hdpe", T("Solid HDPE grip o34 to o50 (8.5 mm active)", "Massiver HDPE-Griff o34 bis o50 (8,5 mm aktiv)"), "polymer", "hdpe-green-faded", 0.017, 0.46, 955, 1850, 0.0085, "ashby-2011")),
        ("tubes-steel", T("Galvanised handle and footrest tubes between the panels", "Verzinkte Griff- und Fußrastenrohre zwischen den Platten"), "steel-galvanised-weathered", 0.0269, "cylinder", 0.4,
         layer("tube", T("Tube wall 2.6 mm", "Rohrwand 2,6 mm"), "metal", "steel-galvanised-weathered", 0.0026, 50, 7850, 460, TUBE_A / (PI * 2 * TUBE_R), "en-10219-2")),
    ]
    regions = []
    for rid, title, mat, char, geom, sky, lay in spec:
        regions.append({"id": rid, "title": title, "kind": "surface", "model": "passive", "material": mat, "areaM2": round(AREAS[rid], 4), "characteristicLengthM": char, "geometry": geom, "skyViewFactor": sky,
                        "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": "berdahl-2008" if rid not in ("footing-concrete",) else "iso-10456-2007"})
    return {"environment": {"airTempC": 28, "meanRadiantTempC": 34, "skyTempC": 14, "groundTempC": 36, "relativeHumidity": 0.4, "windMS": 1.5, "windDirectionDeg": 135, "solarWM2": 850, "diffuseFraction": 0.15, "pressurePa": 101325},
            "operating": [], "emitters": [], "regions": regions,
            "assumptions": [
                T("Summer afternoon in a municipal park: 28 degC air, 850 W/m2 sun, 1.5 m/s breeze, dry lawn at 36 degC; seven passive surfaces, no emitters, no rated power (0 W).", "Sommernachmittag in einem städtischen Park: 28 degC Luft, 850 W/m2 Sonne, 1,5 m/s Brise, trockener Rasen mit 36 degC; sieben passive Flächen, keine Emitter, keine Nennleistung (0 W)."),
                T("Weathered surfaces (authored estimates after Berdahl et al. 2008): UV-chalked HDPE reflects more than new (red 0.55 instead of about 0.65, blue 0.72 instead of 0.80, green 0.62), dull galvanising with white rust absorbs 0.68 and emits 0.45 instead of 0.28 when new, the flaking green paint and rust of the spring absorb 0.8.", "Verwitterte Oberflächen (Schätzwerte nach Berdahl et al. 2008): durch UV gekreidetes HDPE reflektiert mehr als neues (rot 0,55 statt etwa 0,65, blau 0,72 statt 0,80, grün 0,62), matte Verzinkung mit Weißrost absorbiert 0,68 und emittiert 0,45 statt 0,28 im Neuzustand, die abblätternde grüne Farbe und der Rost der Feder absorbieren 0,8."),
                T("The thermal story is the seat: the blue HDPE board in full sun is the hottest surface a child touches, and the dark spring heats under the body's shade only in the low sun; the footing is mostly in contact with the soil at 15 degC (62 % of its area).", "Die thermische Geschichte ist der Sitz: das blaue HDPE-Brett in voller Sonne ist die heißeste Fläche, die ein Kind berührt, und die dunkle Feder erwärmt sich im Schatten des Körpers nur bei tiefer Sonne; das Fundament liegt größtenteils am Boden mit 15 degC an (62 % seiner Fläche)."),
                T("Areal heat capacities rho*cp*min(thickness per face, sqrt(alpha*3600 s)) with ISO 10456 and Ashby values; areas are the outer areas of the 1:50 solids of each region.", "Flächenbezogene Wärmekapazitäten rho*cp*min(Dicke je Seite, sqrt(alpha*3600 s)) mit Werten nach ISO 10456 und Ashby; die Flächen sind die Außenflächen der 1:50-Körper jeder Region."),
            ], "passengers": []}


def structure():
    """🏗 The structural model."""
    return {"gravityMS2": G, "dynamics": "inertial", "supports": SUPPORTS, "assumptions": [
        T("In service the concrete point footing is idealised as fixed in the soil at ground level; the coil spring is a cantilever of three rigid segments with revolute joints at its bending axis, the body rides on its top plate.", "In Betrieb ist das Punktfundament aus Beton in Geländehöhe im Boden eingespannt idealisiert; die Schraubenfeder ist ein Kragarm aus drei starren Abschnitten mit Drehgelenken an ihrer Biegeachse, der Körper sitzt auf ihrer oberen Platte."),
        T(f"Spring bending stiffness after Wahl: k = E d^4 / (32 D n (1 + E/2G)) = {ROCK['kSpring']:.0f} N m/rad for d = 20 mm, D = 180 mm, n = 5 active turns; the toddler of 14 kg and the rocker topple with {ROCK['kGravity']:.0f} N m/rad, leaving {ROCK['kNet']:.0f} N m/rad; with {ROCK['inertia']:.2f} kg m2 about the spring's mid-height the natural period is {PERIOD} s.", f"Biegesteifigkeit der Feder nach Wahl: k = E d^4 / (32 D n (1 + E/2G)) = {ROCK['kSpring']:.0f} N m/rad für d = 20 mm, D = 180 mm, n = 5 wirksame Windungen; das Kleinkind mit 14 kg und die Wippe kippen mit {ROCK['kGravity']:.0f} N m/rad, es bleiben {ROCK['kNet']:.0f} N m/rad; mit {ROCK['inertia']:.2f} kg m2 um die halbe Federhöhe ist die Eigenperiode {PERIOD} s."),
        T("Maintenance: the crane hook is applied as its lifting forces on the four sling legs (handle pair 37.8 %, rear pair 62.2 % of the lifted rocker and spring, " + f"{(MASS_TOTAL - MASSES['footing'][0]) * G:.0f}" + " N, by the lever arms about their centre of mass), because supports of the archive are springs of equal stiffness that would share the weight with the footing; the footing support then carries only the footing.", "Wartung: der Kranhaken ist als seine Hubkräfte an den vier Gehängesträngen angesetzt (Griffpaar 37,8 %, hinteres Paar 62,2 % der gehobenen Wippe mit Feder, " + f"{(MASS_TOTAL - MASSES['footing'][0]) * G:.0f}" + " N, nach den Hebelarmen um ihren Schwerpunkt), weil Auflager des Archivs Federn gleicher Steifigkeit sind, die das Gewicht mit dem Fundament teilen würden; das Fundamentauflager trägt dann nur das Fundament."),
        T("User loads (authored estimates): a toddler of 14 kg (WHO median at about 3 years) on the seat and 30 N per hand on the grips; EN 1176-1 and EN 1176-6 govern the design loads of the product, which are not re-derived here.", "Nutzerlasten (Schätzwerte): ein Kleinkind mit 14 kg (WHO-Median mit etwa 3 Jahren) auf dem Sitz und 30 N je Hand an den Griffen; die Bemessungslasten des Produkts regeln EN 1176-1 und EN 1176-6, sie werden hier nicht neu hergeleitet."),
    ], "clinical": False}


def provenance():
    """📚 Sources and notes."""
    refs = [
        ("en-1176-1-2017", "EN 1176-1:2017 Playground equipment and surfacing - Part 1: General safety requirements and test methods (free height of fall, impact area of at least 1.5 m for falls up to 1.5 m, turf/topsoil accepted up to 1.0 m free height of fall, entrapment of fingers)."),
        ("en-1176-6-2017", "EN 1176-6:2017 Playground equipment and surfacing - Part 6: Additional specific safety requirements and test methods for rocking equipment (spring rockers on a central spring, seats, hand grips and foot rests)."),
        ("en-1176-7-2020", "EN 1176-7:2020 Playground equipment and surfacing - Part 7: Guidance on installation, inspection, maintenance and operation (routine, operational and annual main inspection; replacement of worn springs and broken grips)."),
        ("din-18034-2020", "DIN 18034:2020 Spielplätze und Freiräume zum Spielen - Anforderungen für Planung, Bau und Betrieb (playgrounds in German municipal parks)."),
        ("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: Densities, self-weight, imposed loads (Table A.1: normal-weight concrete 24 kN/m3, steel 77-78.5 kN/m3)."),
        ("en-1992-1-1", "EN 1992-1-1:2004 Eurocode 2: Design of concrete structures (Table 3.1: C20/25, Ecm = 30 GPa)."),
        ("en-10025-2", "EN 10025-2:2019 Hot rolled products of structural steels - Part 2: Non-alloy structural steels (S235JR plates of the anchor and spring plates)."),
        ("en-10219-2", "EN 10219-2:2019 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (o26.9 x 2.6: 1.56 kg/m)."),
        ("en-10089", "EN 10089:2002 Hot rolled steels for quenched and tempered springs (51CrV4: E = 206 GPa, density 7850 kg/m3)."),
        ("iso-4032", "ISO 4032:2012 Hexagon regular nuts (style 1) (M16: 24 mm across flats, 14.8 mm high; M10: 16 mm across flats, 8.4 mm high)."),
        ("wahl-1963", "Wahl, A. M. (1963). Mechanical Springs, 2nd ed., McGraw-Hill, chapter on helical springs under bending: angular deflection 32 M D n (1 + E/2G) / (E d^4)."),
        ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (concrete 2400 kg/m3: 2.0 W/mK, 1000 J/kgK; steel 50 W/mK, 450 J/kgK)."),
        ("ashby-2011", "Ashby, M. F. (2011). Materials Selection in Mechanical Design, 4th ed., Butterworth-Heinemann (HDPE: 952-965 kg/m3, 0.46-0.50 W/mK, 1.81-1.88 kJ/kgK, E 0.95-1.1 GPa)."),
        ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities of polymers, concrete and galvanised steel)."),
        ("berdahl-2008", "Berdahl, P., Akbari, H., Levinson, R., Miller, W. A. (2008). Weathering of roofing materials - An overview. Construction and Building Materials 22(4), 423-433 (solar reflectance and emittance of aged polymer, paint and galvanised surfaces)."),
        ("who-growth-2006", "WHO Multicentre Growth Reference Study Group (2006). WHO Child Growth Standards: weight-for-age (median about 12 kg at 24 months and 14 kg at 36 months)."),
        ("en-1492-2", "EN 1492-2:2000+A1:2008 Textile slings - Safety - Part 2: Roundslings made of man-made fibres for general purpose use (working load limit 1 t, mode factor 0.8 choked)."),
        ("catalogue-spring-rocker", "Catalogue-typical data of HDPE spring rockers of German playground-equipment makers 2005-2020 compiled by the author as types, no single product or brand: length 0.85-1.0 m, seat height 0.45-0.55 m, free height of fall up to 0.6 m, HDPE 19 mm, coil spring of 20-22 mm wire with 200-220 mm outer diameter and 300-350 mm height, bolted to a cast-in anchor plate, point footing about 0.5 x 0.5 x 0.4-0.5 m of C20/25."),
    ]
    return {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in refs],
            "notes": T(f"Side elevation from the asset's right: a toddler's spring rocker of the common German municipal type, a stylised horse cut from two 19 mm HDPE panels 160 mm apart, seat board 509 mm above the lawn, handle at 660 mm, footrests at 345 mm, on a coil spring of 20 mm wire (6 turns, 200 mm outside, 294 mm between the plates) bolted with four M16 studs and nuts to an anchor plate on a 0.50 x 0.50 x 0.40 m C20/25 point footing whose top is flush with the lawn. About 15 years in service: the red panels chalked and faded, the blue seat worn smooth in the middle, the right ear chipped, the right grip embrittled with a crack and a broken-away quarter at its end, a graffiti tag and sticker residue on the right panel, the spring's green paint flaking with rust runs at the clamps, a hairline crack in the footing top; still safe and in use. Mass {MASS_TOTAL:.1f} kg, of which the footing {MASSES['footing'][0]:.1f} kg and the lifted rocker with spring {MASS_TOTAL - MASSES['footing'][0]:.1f} kg; all masses from section x density.",
                       f"Seitenansicht von rechts: eine Federwippe für Kleinkinder des verbreiteten deutschen kommunalen Typs, ein stilisiertes Pferd aus zwei 19-mm-HDPE-Platten im Abstand 160 mm, Sitzbrett 509 mm über dem Rasen, Griff auf 660 mm, Fußrasten auf 345 mm, auf einer Schraubenfeder aus 20-mm-Draht (6 Windungen, 200 mm außen, 294 mm zwischen den Platten), mit vier Gewindebolzen und Muttern M16 auf eine Ankerplatte geschraubt, die auf einem Punktfundament 0,50 x 0,50 x 0,40 m aus C20/25 sitzt, dessen Oberseite bündig mit dem Rasen ist. Etwa 15 Jahre in Betrieb: die roten Platten gekreidet und ausgeblichen, der blaue Sitz in der Mitte glatt gesessen, das rechte Ohr abgeplatzt, das rechte Griffstück versprödet mit einem Riss und einem abgebrochenen Viertel am Ende, ein Graffiti-Kürzel und Aufkleberreste auf der rechten Platte, die grüne Farbe der Feder abblätternd mit Rostläufern an den Klemmen, ein Haarriss in der Fundamentoberseite; weiterhin sicher und in Gebrauch. Masse {MASS_TOTAL:.1f} kg, davon Fundament {MASSES['footing'][0]:.1f} kg und gehobene Wippe mit Feder {MASS_TOTAL - MASSES['footing'][0]:.1f} kg; alle Massen aus Querschnitt x Dichte.")}


def asset():
    """🏺 The whole asset document."""
    reps = {str(s): representation(s) for s in (50, 100, 200)}
    reps["500"] = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": T("At 1:500 the whole rocker with its footing is 2.4 x 1.9 mm on paper, below the 3 mm of a drawable component; its 19 mm panels and 20 mm spring wire would be 0.04 mm.", "Bei 1:500 misst die ganze Wippe mit Fundament 2,4 x 1,9 mm auf dem Papier, unter den 3 mm eines zeichenbaren Bauteils; ihre 19-mm-Platten und der 20-mm-Federdraht wären 0,04 mm."),
                   "limitingFeature": T("The whole rocker, 1.22 m high including the footing", "Die ganze Wippe, 1,22 m hoch einschließlich Fundament"), "limitingSizeM": 1.22, "alternatives": [{"kind": "scale", "denominator": 200}]}
    vols = {str(s): volume(s) for s in (50, 100, 200)}
    vols["500"] = {"status": "unavailable", "denominator": 500}
    a = {
        "id": SLUG, "category": "objects", "title": T("Used Spring Rocker", "Gebrauchtes Federwipptier"),
        "summary": T(f"A toddler's spring rocking horse of two faded red 19 mm HDPE panels on a 20 mm coil spring, bolted to the anchor plate of a 0.50 m concrete point footing in the lawn; the right ear chipped, the right grip cracked and broken at its end, graffiti and sticker residue, the spring's paint flaking with rust. The depot lifts it off the anchor plate on a sling to replace the spring; a toddler rocks it with a period of {PERIOD} s.",
                     f"Ein Federwipptier für Kleinkinder aus zwei ausgeblichenen roten 19-mm-HDPE-Platten auf einer Schraubenfeder aus 20-mm-Draht, auf die Ankerplatte eines Punktfundaments von 0,50 m im Rasen geschraubt; das rechte Ohr abgeplatzt, das rechte Griffstück gerissen und am Ende abgebrochen, Graffiti und Aufkleberreste, die Farbe der Feder blättert mit Rost ab. Der Bauhof hebt es an einem Gehänge von der Ankerplatte, um die Feder zu tauschen; ein Kleinkind wippt es mit einer Periode von {PERIOD} s."),
        "tags": ["used", "park", "public-space", "play", "maintenance", "spring-rocker", "rocking", "playground", "toddler", "hdpe", "coil-spring", "spring-steel", "steel", "concrete", "footing"],
        "provenance": provenance(),
        "dimensions": {"heightM": r6(0.82 + FOOT_DEPTH), "widthM": 0.95, "depthM": 2 * FOOT_HALF, "massKg": round(MASS_TOTAL, 3)},
        "materials": materials(), "rig": rig(), "anchors": anchors(), "representations": reps, "symbols": [], "poses": poses(), "clips": clips(),
        "thermal": thermal(), "structure": structure(), "volumes": vols,
        "presentation": {"defaultPose": "in-service", "defaultClip": "rocking", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"], "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": a}


if __name__ == "__main__":
    doc = asset()
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    if "--summary" in sys.argv:
        print("[DEBUG] masses", {b: (round(kg, 3), tuple(round(c, 4) for c in com)) for b, (kg, com) in MASSES.items()}, "total", round(MASS_TOTAL, 3))
        print("[DEBUG] areas", {r: round(a, 4) for r, a in AREAS.items()})
        print("[DEBUG] rocking", {k: round(v, 4) for k, v in ROCK.items()}, "pitch", round(PITCH, 5), "yj", round(YJ1, 4), round(YJ2, 4), "helix", round(HELIX_LEN, 4))
