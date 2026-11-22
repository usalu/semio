"""🎹 Authoritative generator of the used harmonium: every hand-chosen number of a German suction-wind pedal harmonium of the 1950s (five octaves, two treadles, ten stops, carved crest), its rig, poses and clips, the 2D drawings and the 3D models per scale from one part list, written to 🔣️.json next to this file."""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SLUG = "used-harmonium"


def T(en, de):
    """🗣 A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """✂ A number rounded to the micrometre."""
    return round(v + 0.0, 6)


FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}
PLANE = 0.105
SIDE_IN, SIDE_OUT = 0.578, 0.600
TREADLE_HINGE, TREADLE_REST, TREADLE_LEN, TREADLE_STROKE = (0.46, 0.06), 155.0, 0.34, 10.0
TREADLE_Z = (0.025, 0.185)
EXH_HINGE, EXH_LEN, EXH_T = (0.36, 0.33), 0.21, 0.012
EXH_Z = (0.03, 0.53)
STRAP_Z = (0.095, 0.125)
FALL_HINGE, FALL_LEN, FALL_T, FALL_OPEN = (0.362, 0.747), 0.165, 0.012, 92.0
LID_HINGE, LID_LEN, LID_T = (0.03, 1.0), 0.40, 0.02
DESK_AT, DESK_LEN, DESK_T, DESK_Z, DESK_UP = 0.355, 0.22, 0.015, 0.35, -70.0
STOPS_AT, STOP_R, STOP_L, STOP_PULL = (0.42, 0.95), 0.013, 0.032, 0.018
STOP_Z = [r6(-0.315 + 0.07 * k) for k in range(10)]
DRAWN = [-0.105, -0.035, 0.035, 0.105]
KEY_Z = 0.4235
OCTAVE = 0.1645
BLACK_OFF = [0.0145, 0.0415, 0.0840, 0.1095, 0.1350]
HANDLE_X = 0.232
GRIP_Y = 0.71


def rot(p, deg):
    """🔄 A 2D vector turned counter-clockwise by deg."""
    a = math.radians(deg)
    return (p[0] * math.cos(a) - p[1] * math.sin(a), p[0] * math.sin(a) + p[1] * math.cos(a))


def add(a, b):
    """➕ The sum of two 2D points."""
    return (a[0] + b[0], a[1] + b[1])


def toe_top(offset):
    """🦶 World point of the carpet top at the toe end of a treadle turned by offset degrees."""
    ang = TREADLE_REST + offset
    return add(TREADLE_HINGE, add(rot((TREADLE_LEN, 0), ang), rot((0, -0.028), ang)))


def exh_tip(theta):
    """🪗 World point under the free end of an exhauster board opened by theta degrees."""
    ang = 180.0 + theta
    return add(EXH_HINGE, add(rot((EXH_LEN, 0), ang), rot((0, EXH_T), ang)))


STRAP_LEN = math.dist(exh_tip(0), toe_top(0))


def solve_theta(offset):
    """🧮 The exhauster opening that keeps the strap at its length for a treadle turned by offset degrees."""
    lo, hi = -5.0, 30.0
    target = toe_top(offset)
    for _ in range(80):
        mid = (lo + hi) / 2
        if math.dist(exh_tip(mid), target) > STRAP_LEN:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def strap_rel(theta, offset):
    """🎗 Angle of the strap relative to its exhauster so that it points at the treadle toe."""
    a, b = exh_tip(theta), toe_top(offset)
    world = math.degrees(math.atan2(b[1] - a[1], b[0] - a[0]))
    rel = world - (180.0 + theta)
    while rel > 180:
        rel -= 360
    while rel <= -180:
        rel += 360
    return rel


THETA_PRESSED = solve_theta(TREADLE_STROKE)
STRAP_REST = strap_rel(0.0, 0.0)
STRAP_PRESSED = strap_rel(THETA_PRESSED, TREADLE_STROKE)

FRAMES = {"floor": (0.0, 0.0, 0.0), "case": (0.0, 0.0, 0.0), "stops": (STOPS_AT[0], STOPS_AT[1], 0.0),
          "treadle-l": (TREADLE_HINGE[0], TREADLE_HINGE[1], TREADLE_REST), "treadle-r": (TREADLE_HINGE[0], TREADLE_HINGE[1], TREADLE_REST),
          "exhauster-l": (EXH_HINGE[0], EXH_HINGE[1], 180.0), "exhauster-r": (EXH_HINGE[0], EXH_HINGE[1], 180.0),
          "fall": (FALL_HINGE[0], FALL_HINGE[1], 0.0), "lid": (LID_HINGE[0], LID_HINGE[1], 0.0),
          "desk": (LID_HINGE[0] + DESK_AT, LID_HINGE[1], 180.0)}
for side in ("l", "r"):
    tip = exh_tip(0)
    FRAMES[f"strap-{side}"] = (tip[0], tip[1], 180.0 + STRAP_REST)


def local(bone, p):
    """📐 A world rest point (x, y) in the (u, v) frame of a bone."""
    ox, oy, a = FRAMES[bone]
    return rot((p[0] - ox, p[1] - oy), -a)


def world(bone, q):
    """🌍 A bone-local (u, v) point in world rest coordinates."""
    ox, oy, a = FRAMES[bone]
    return add((ox, oy), rot(q, a))


ROTATED = {b for b, f in FRAMES.items() if abs(f[2]) > 1e-9}
TURNING = ROTATED | {"fall", "lid"}


def side_profile(scale):
    """🪵 The side panel outline in world (x, y): two feet, the straight lower front, the key cheek arm and the scroll back to the upper front."""
    if scale == 50:
        scroll = [(0.53, 0.735), (0.521, 0.75), (0.49, 0.753), (0.462, 0.762), (0.441, 0.777), (0.427, 0.798), (0.42, 0.825)]
        return [(0.0, 0.0), (0.09, 0.0), (0.09, 0.03), (0.43, 0.03), (0.43, 0.0), (0.52, 0.0), (0.52, 0.62), (0.53, 0.63)] + scroll + [(0.42, 0.98), (0.0, 0.98)]
    if scale == 100:
        return [(0.0, 0.0), (0.09, 0.0), (0.09, 0.03), (0.43, 0.03), (0.43, 0.0), (0.52, 0.0), (0.52, 0.62), (0.53, 0.63), (0.53, 0.74), (0.49, 0.753), (0.448, 0.772), (0.425, 0.80), (0.42, 0.825), (0.42, 0.98), (0.0, 0.98)]
    return [(0.0, 0.0), (0.52, 0.0), (0.52, 0.62), (0.53, 0.63), (0.53, 0.745), (0.46, 0.765), (0.42, 0.825), (0.42, 0.98), (0.0, 0.98)]


def crest_profile(scale):
    """👑 The carved crest seen from the front, (z, y): a scrolled board rising from 70 mm at the ends to 170 mm at the centre."""
    if scale == 50:
        half = [(0.0, 1.15), (0.035, 1.148), (0.07, 1.135), (0.10, 1.118), (0.13, 1.112), (0.16, 1.12), (0.19, 1.127), (0.22, 1.12), (0.26, 1.098), (0.30, 1.088),
                (0.34, 1.093), (0.38, 1.097), (0.42, 1.088), (0.46, 1.07), (0.50, 1.06), (0.53, 1.058), (0.56, 1.05), (0.56, 0.98)]
    elif scale == 100:
        half = [(0.0, 1.15), (0.07, 1.135), (0.13, 1.112), (0.19, 1.127), (0.26, 1.098), (0.34, 1.093), (0.42, 1.088), (0.50, 1.06), (0.56, 1.05), (0.56, 0.98)]
    else:
        half = [(0.0, 1.15), (0.15, 1.12), (0.32, 1.095), (0.56, 1.05), (0.56, 0.98)]
    right = [(-z, y) for z, y in reversed(half)]
    return right[:-1] + half if half[0][0] == 0 else right + half


class Part:
    """🧱 One physical part of one scale: its 3D solid in the bone frame and its 2D shapes in the three viewpoints, all from the same numbers."""

    def __init__(self, pid, bone, kind, geo, material, tone, region, role="body", views=("elevation", "front", "plan"), extra=None, pen=None):
        self.id, self.bone, self.kind, self.geo, self.material, self.tone, self.region = pid, bone, kind, geo, material, tone, region
        self.role, self.views, self.extra, self.pen = role, views, extra or {}, pen

    def zr(self):
        """↔ Lateral range of the part at rest."""
        g = self.geo
        if self.kind == "box":
            return g[0][2], g[1][2]
        if self.kind in ("pxy", "lxy"):
            return g[1], g[2]
        if self.kind == "pyz":
            return min(p[0] for p in g[0]), max(p[0] for p in g[0])
        if self.kind == "cylx":
            return g[3] - g[4], g[3] + g[4]
        if self.kind == "lzx":
            return min(p[0] for p in g[0]), max(p[0] for p in g[0])

    def xy(self):
        """🗺 The outline of the part in its bone's (u, v) frame, the elevation profile."""
        g = self.geo
        ox, oy, _ = FRAMES[self.bone]
        if self.kind == "box":
            (x0, y0, _), (x1, y1, _) = g
            return [(x0 - ox, y0 - oy), (x1 - ox, y0 - oy), (x1 - ox, y1 - oy), (x0 - ox, y1 - oy)]
        if self.kind == "pxy":
            return [local(self.bone, p) for p in g[0]]
        if self.kind == "lxy":
            return list(g[0])
        if self.kind == "pyz":
            pts, x0, x1 = g
            y0, y1 = min(p[1] for p in pts), max(p[1] for p in pts)
            return [(x0 - ox, y0 - oy), (x1 - ox, y0 - oy), (x1 - ox, y1 - oy), (x0 - ox, y1 - oy)]
        if self.kind == "cylx":
            x0, x1, cy, cz, r = g
            return [(x0 - ox, cy - r - oy), (x1 - ox, cy - r - oy), (x1 - ox, cy + r - oy), (x0 - ox, cy + r - oy)]
        if self.kind == "lzx":
            pts, v0, v1 = g
            u0, u1 = min(p[1] for p in pts), max(p[1] for p in pts)
            return [(u0, v0), (u1, v0), (u1, v1), (u0, v1)]

    def world_box(self):
        """📦 World bounds (x0, y0, z0), (x1, y1, z1) of the part at rest."""
        pts = [world(self.bone, q) for q in self.xy()]
        z0, z1 = self.zr()
        if self.kind == "pyz":
            pp = self.geo[0]
            return (self.geo[1], min(p[1] for p in pp), z0), (self.geo[2], max(p[1] for p in pp), z1)
        return (min(p[0] for p in pts), min(p[1] for p in pts), z0), (max(p[0] for p in pts), max(p[1] for p in pts), z1)

    def solid(self):
        """🧊 The volume solid of the part in the rest frame of its bone."""
        ox, oy, _ = FRAMES[self.bone]
        g = self.geo
        s = {"id": self.id, "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        if self.kind == "box" and self.bone not in ROTATED:
            (x0, y0, z0), (x1, y1, z1) = g
            s.update(kind="box", size=[r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], transform={"translation": [r6((x0 + x1) / 2 - ox), r6((y0 + y1) / 2 - oy), r6((z0 + z1) / 2)]})
            if self.extra.get("radius"):
                s["cornerRadius"] = self.extra["radius"]
        elif self.kind in ("box", "pxy", "lxy"):
            z0, z1 = self.zr()
            s.update(kind="extrude", plane="xy", profile=[[r6(a), r6(b)] for a, b in ccw(self.xy())], **{"from": r6(z0), "to": r6(z1)})
        elif self.kind == "pyz":
            pts, x0, x1 = g
            s.update(kind="extrude", plane="yz", profile=[[r6(y - oy), r6(z)] for y, z in ccw([(p[1], p[0]) for p in pts])], **{"from": r6(x0 - ox), "to": r6(x1 - ox)})
        elif self.kind == "cylx":
            x0, x1, cy, cz, r = g
            s.update(kind="revolve", profile=[[0, 0], [0, r6(r)], [r6(x1 - x0), r6(r)], [r6(x1 - x0), 0]], axis=[1, 0, 0], origin=[r6(x0 - ox), r6(cy - oy), r6(cz)])
        elif self.kind == "lzx":
            pts, v0, v1 = g
            s.update(kind="extrude", plane="zx", profile=[[r6(a), r6(b)] for a, b in ccw(pts)], **{"from": r6(v0), "to": r6(v1)})
        for key in ("slender", "sheet", "texture", "repeat", "tags", "envelope"):
            if key in self.extra:
                s[key] = self.extra[key]
        return s

    def base_shape(self, depth):
        """✏ The members every 2D shape of the part carries."""
        s = {"id": self.id, "bone": self.bone, "role": self.role, "z": round(depth, 4), "filled": True, "tone": self.tone, "material": self.material}
        if isinstance(self.region, list):
            s["regions"] = self.region
        else:
            s["region"] = self.region
        if self.pen:
            s["penMm"] = self.pen
        return s

    def elevation(self, section, scale):
        """🖼 The side elevation shape (a section at plane z when section is true), or None when the part lies in the removed half."""
        z0, z1 = self.zr()
        if section and z1 <= PLANE:
            return None
        cut = section and z0 <= PLANE <= z1
        depth = (20.0 if cut else 5.0) - (z0 if not section else max(z0, PLANE))
        s = self.base_shape(depth)
        if cut and scale == 50:
            s["hatch"] = "cut-softwood" if self.material in ("veneered-softwood", "pine", "beech") else ("cut-other" if self.material not in ("celluloid",) else None)
            if s["hatch"] is None:
                del s["hatch"]
        if not cut and section:
            s["penMm"] = {50: 0.18, 100: 0.13}[scale]
        if self.kind == "pyz" and section:
            pts, x0, x1 = self.geo
            ymax = crest_height(pts, PLANE)
            ox, oy, _ = FRAMES[self.bone]
            y0 = min(p[1] for p in pts)
            return dict(s, kind="rect", x=r6(x0 - ox), y=r6(y0 - oy), w=r6(x1 - x0), h=r6(ymax - y0))
        if self.kind == "box" and self.bone not in ROTATED:
            (x0, y0, _), (x1, y1, _) = self.geo
            ox, oy, _ = FRAMES[self.bone]
            return dict(s, kind="rect", x=r6(x0 - ox), y=r6(y0 - oy), w=r6(x1 - x0), h=r6(y1 - y0))
        return dict(s, kind="poly", closed=True, points=[[r6(a), r6(b)] for a, b in self.xy()])

    def front(self):
        """🪞 The front elevation shape, seen from ahead: lateral z to the paper's right... as the product frames it, height up."""
        (x0, y0, z0), (x1, y1, z1) = self.world_box()
        s = self.base_shape(5.0 + x1)
        if self.bone in TURNING or self.kind == "lzx":
            zz = self.zr()
            return dict(s, kind="poly", closed=True, points=[[r6(a), r6(b)] for a, b in self.xy()], solid={"kind": "extruded", "sweep": "w", "z0": r6(zz[0]), "z1": r6(zz[1])})
        if self.kind == "pyz":
            return dict(s, kind="poly", closed=True, points=[[r6(z), r6(y)] for z, y in self.geo[0]])
        if self.kind == "cylx":
            x0_, x1_, cy, cz, r = self.geo
            return dict(s, kind="ellipse", cx=r6(cz), cy=r6(cy), rx=r6(r), ry=r6(r))
        return dict(s, kind="rect", x=r6(z0), y=r6(y0), w=r6(z1 - z0), h=r6(y1 - y0))

    def plan(self):
        """🧭 The plan shape, seen from above: ahead x to the right, lateral z up the page."""
        (x0, y0, z0), (x1, y1, z1) = self.world_box()
        s = self.base_shape(5.0 + y1)
        if self.bone in TURNING or self.kind == "lzx":
            zz = self.zr()
            return dict(s, kind="poly", closed=True, points=[[r6(a), r6(b)] for a, b in self.xy()], solid={"kind": "extruded", "sweep": "w", "z0": r6(zz[0]), "z1": r6(zz[1])})
        return dict(s, kind="rect", x=r6(x0), y=r6(z0), w=r6(x1 - x0), h=r6(z1 - z0))


def crest_height(pts, plane):
    """📈 The highest point of the crest outline at or beyond the lateral position plane."""
    best = -1.0
    n = len(pts)
    for i in range(n):
        (za, ya), (zb, yb) = pts[i], pts[(i + 1) % n]
        if za >= plane:
            best = max(best, ya)
        if (za - plane) * (zb - plane) < 0:
            t = (plane - za) / (zb - za)
            best = max(best, ya + t * (yb - ya))
    return best


def ccw(pts):
    """🔁 The ring in counter-clockwise order."""
    a = sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))
    return pts if a > 0 else list(reversed(pts))


def lbox(u0, u1, v0, v1):
    """▭ A bone-local rectangle (u, v) as a ring."""
    return [(u0, v0), (u1, v0), (u1, v1), (u0, v1)]


def parts_of(scale):
    """🗂 The hand-chosen part list of one scale: every part the real harmonium has that this scale can show, generalised for its smallest feature."""
    P = []
    f = scale
    wall = {50: 0.022, 100: 0.022, 200: 0.03}[f]
    sp = side_profile(f)
    P.append(Part("side-right", "case", "pxy", (sp, -SIDE_OUT, -SIDE_OUT + wall), "veneered-softwood", 0.38, "case"))
    P.append(Part("side-left", "case", "pxy", (sp, SIDE_OUT - wall, SIDE_OUT), "veneered-softwood", 0.38 if f == 200 else 0.5, "case"))
    zi = SIDE_OUT - wall
    back_t = 0.015 if f != 200 else 0.03
    P.append(Part("back", "case", "box", ((0.0, 0.03, -zi), (back_t, 0.98, zi)), "veneered-softwood" if f == 200 else "pine", 0.45, "case", extra={"sheet": True} if f == 100 else {}))
    P.append(Part("bottom", "case", "box", ((back_t, 0.03, -zi), (0.52, 0.055 if f != 200 else 0.06, zi)), "pine", 0.55, "case"))
    if f == 200:
        P.append(Part("reservoir", "case", "box", ((back_t, 0.33, -0.54), (0.34, 0.54, 0.54)), "rubber-cloth", 0.2, "bellows"))
        P.append(Part("action", "case", "box", ((back_t, 0.54, -zi), (0.34, 0.92, zi)), "veneered-softwood", 0.42, "case"))
        P.append(Part("kneeboard", "case", "box", ((0.37, 0.36, -zi), (0.40, 0.655, zi)), "veneered-softwood", 0.4, "case"))
        P.append(Part("keys", "case", "box", ((0.34, 0.655, -KEY_Z), (0.525, 0.72, KEY_Z)), "celluloid", 0.88, "keys"))
        for side, sg in (("left", 1), ("right", -1)):
            zs = sorted((sg * KEY_Z, sg * zi))
            P.append(Part(f"cheek-block-{side}", "case", "box", ((0.34, 0.655, zs[0]), (0.525, 0.735, zs[1])), "veneered-softwood", 0.4, "case"))
        P.append(Part("stop-rail", "case", "box", ((0.34, 0.92, -zi), (0.42, 0.98, zi)), "veneered-softwood", 0.36, "case"))
        P.append(Part("crest", "case", "pyz", (crest_profile(f), 0.0, 0.03), "veneered-softwood", 0.38, "case"))
    else:
        P.append(Part("reservoir", "case", "box", ((0.03, 0.33, -0.55), (0.36, 0.54, 0.55)), "rubber-cloth", 0.2, "bellows"))
        P.append(Part("action", "case", "box", ((back_t, 0.54, -zi), (0.34, 0.80, zi)), "action", 0.5, "case"))
        P.append(Part("kneeboard", "case", "box", ((0.385, 0.36, -zi), (0.40, 0.655, zi)), "veneered-softwood", 0.4, "case"))
        P.append(Part("keybed", "case", "box", ((0.34, 0.655, -zi), (0.505, 0.69, zi)), "beech", 0.45, "case"))
        P.append(Part("key-slip", "case", "box", ((0.505, 0.655, -zi), (0.525, 0.708, zi)), "veneered-softwood", 0.36, "case"))
        P.append(Part("keys-white", "case", "box", ((0.34, 0.69, -KEY_Z), (0.505, 0.72, KEY_Z)), "celluloid", 0.88, "keys"))
        if f == 50:
            for k, off in enumerate(BLACK_OFF):
                zc = -KEY_Z + off
                P.append(Part(f"key-black-{k}", "case", "box", ((0.356, 0.715, zc - 0.005), (0.451, 0.732, zc + 0.005)), "ebonised-wood", 0.1, "keys",
                              extra={"repeat": {"count": 5, "translation": [0, 0, OCTAVE]}}))
        for side, sg in (("left", 1), ("right", -1)):
            zs = sorted((sg * KEY_Z, sg * zi))
            P.append(Part(f"cheek-block-{side}", "case", "box", ((0.34, 0.69, zs[0]), (0.505, 0.735, zs[1])), "veneered-softwood", 0.4, "case"))
        P.append(Part("nameboard", "case", "box", ((0.34, 0.7215, -zi), (0.356, 0.92, zi)), "veneered-softwood", 0.42, "case"))
        P.append(Part("stop-rail", "case", "box", ((0.34, 0.92, -zi), (0.42, 0.98, zi)), "veneered-softwood", 0.36, "case"))
        for k, z in enumerate(STOP_Z):
            if z in DRAWN:
                P.append(Part(f"stop-drawn-{k}", "stops", "cylx", (STOPS_AT[0], STOPS_AT[0] + STOP_L, STOPS_AT[1], z, STOP_R), "ebonised-wood", 0.12, "case"))
            else:
                P.append(Part(f"stop-{k}", "case", "cylx", (STOPS_AT[0], STOPS_AT[0] + STOP_L, STOPS_AT[1], z, STOP_R), "ebonised-wood", 0.12, "case"))
        P.append(Part("crest", "case", "pyz", (crest_profile(f), 0.0, 0.03), "veneered-softwood", 0.38, "case"))
        ht = 0.012 if f == 50 else 0.015
        for side, sg in (("left", 1), ("right", -1)):
            zs = sorted((sg * SIDE_OUT, sg * (SIDE_OUT + ht)))
            P.append(Part(f"handle-{side}", "case", "box", ((HANDLE_X - 0.06, GRIP_Y - 0.015, zs[0]), (HANDLE_X + 0.06, GRIP_Y + 0.015, zs[1])), "iron", 0.15, "case"))
    tt = {50: 0.022, 100: 0.028, 200: 0.03}[f]
    for side, sg in (("left", 1), ("right", -1)):
        zs = sorted((sg * TREADLE_Z[0], sg * TREADLE_Z[1]))
        b = f"treadle-{side[0]}"
        P.append(Part(f"treadle-{side}", b, "lxy", (lbox(0.0, TREADLE_LEN, -tt, 0.0), zs[0], zs[1]), "beech", 0.5, "treadles"))
        if f == 50:
            torn = 0.04
            if sg > 0:
                ring = [(zs[0] + 0.005, 0.02), (zs[1] - 0.005, 0.02), (zs[1] - 0.005, 0.282), (zs[1] - 0.03, 0.296), (zs[1] - 0.026, 0.32), (zs[0] + 0.005, 0.32)]
            else:
                ring = [(zs[0] + 0.005, 0.02), (zs[1] - 0.005, 0.02), (zs[1] - 0.005, 0.32), (zs[0] + 0.022, 0.32), (zs[0] + 0.018, 0.30 - torn / 4), (zs[0] + 0.005, 0.29)]
            P.append(Part(f"carpet-{side}", b, "lzx", (ring, -0.028, -0.022), "wool-carpet", 0.42, "treadles", extra={"sheet": True}))
        et = {50: EXH_T, 100: 0.015, 200: 0.03}[f]
        ez = sorted((sg * EXH_Z[0], sg * EXH_Z[1]))
        e = f"exhauster-{side[0]}"
        P.append(Part(f"exhauster-{side}", e, "lxy", (lbox(0.0, EXH_LEN, 0.0, et), ez[0], ez[1]), "pine", 0.3, "bellows"))
        if f != 200:
            gt = {50: 0.008, 100: 0.015}[f]
            rise = math.tan(math.radians(THETA_PRESSED + 1.0)) * EXH_LEN
            tri = [(0.0, 0.0), (EXH_LEN, 0.0), (EXH_LEN, -rise)]
            far = sorted((sg * EXH_Z[1], sg * (EXH_Z[1] - gt)))
            near = sorted((sg * EXH_Z[0], sg * (EXH_Z[0] + gt)))
            P.append(Part(f"gusset-{side}-outer", e, "lxy", (tri, far[0], far[1]), "rubber-cloth", 0.2, "bellows"))
            P.append(Part(f"gusset-{side}-inner", e, "lxy", (tri, near[0], near[1]), "rubber-cloth", 0.2, "bellows"))
        st = {50: 0.008, 100: 0.015, 200: 0.03}[f]
        sw = sorted((sg * STRAP_Z[0], sg * STRAP_Z[1]))
        P.append(Part(f"strap-{side}", f"strap-{side[0]}", "lxy", (lbox(0.0, STRAP_LEN + 0.008, -st / 2, st / 2), sw[0], sw[1]), "leather", 0.25, "bellows"))
    fv = {50: (-FALL_T, 0.0), 100: (-FALL_T, 0.003), 200: (-FALL_T, 0.018)}[f]
    P.append(Part("fall", "fall", "lxy", (lbox(0.0, FALL_LEN, fv[0], fv[1]), -(SIDE_IN - 0.001) if f != 200 else -(SIDE_OUT - wall), SIDE_IN - 0.001 if f != 200 else SIDE_OUT - wall), "veneered-softwood", 0.36, "case"))
    lv = {50: (-LID_T, 0.0), 100: (-LID_T, 0.0), 200: (-LID_T, 0.01)}[f]
    P.append(Part("lid", "lid", "lxy", (lbox(0.0, LID_LEN, lv[0], lv[1]), -SIDE_OUT, SIDE_OUT), "veneered-softwood", 0.4, "case"))
    dt = DESK_T
    dv = {50: (-DESK_T, 0.0), 100: (-DESK_T, 0.0), 200: (-0.04, -0.01)}[f]
    P.append(Part("desk", "desk", "lxy", (lbox(0.0, DESK_LEN, dv[0], dv[1]), -DESK_Z, DESK_Z), "veneered-softwood", 0.44, "case"))
    if f == 50:
        P.append(Part("desk-ledge", "desk", "lxy", (lbox(0.0, 0.02, -dt - 0.02, -dt), -DESK_Z, DESK_Z), "veneered-softwood", 0.36, "case"))
    elif f == 100:
        P.append(Part("desk-ledge", "desk", "lxy", (lbox(0.0, 0.02, -dt - 0.02, -dt), -DESK_Z, DESK_Z), "veneered-softwood", 0.36, "case"))
    return P


MATERIALS = [
    {"id": "veneered-softwood", "title": T("Spruce carcass board with walnut veneer and aged shellac, 15 to 22 mm", "Fichten-Korpusplatte mit Nussbaumfurnier und gealterter Schellackpolitur, 15 bis 22 mm"),
     "emissivity": 0.9, "solarAbsorptivity": 0.78, "densityKgM3": 470, "conductivityWmK": 0.12, "specificHeatJKgK": 1600, "elasticModulusPa": 10000000000, "strengthPa": 40000000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "pine", "title": T("Unfinished pine of the back, the bottom and the bellows boards", "Unbehandelte Kiefer von Rückwand, Boden und Balgbrettern"),
     "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 480, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
    {"id": "beech", "title": T("Beech of the keybed and the treadle boards", "Buche von Klaviaturboden und Trittbrettern"),
     "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 720, "conductivityWmK": 0.16, "specificHeatJKgK": 1600, "elasticModulusPa": 14000000000, "strengthPa": 50000000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "action", "title": T("Reed action: pine reed pan with cells, pallets, brass reeds and stop mutes, effective density", "Zungenwerk: Kiefern-Kanzellenbrett mit Ventilen, Messingzungen und Registerklappen, wirksame Dichte"),
     "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 240, "conductivityWmK": 0.1, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "ord-hume-1986"},
    {"id": "celluloid", "title": T("Celluloid key coverings, yellowed, on lime-wood keys", "Zelluloid-Tastenbeläge, vergilbt, auf Lindenholztasten"),
     "emissivity": 0.92, "solarAbsorptivity": 0.45, "densityKgM3": 560, "conductivityWmK": 0.17, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "ebonised-wood", "title": T("Ebonised pear wood of the sharps and the stop knobs", "Schwarz gebeiztes Birnbaumholz der Obertasten und Registerknöpfe"),
     "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 700, "conductivityWmK": 0.16, "specificHeatJKgK": 1600, "source": "literature", "reference": "en-iso-10456"},
    {"id": "wool-carpet", "title": T("Worn and torn wool carpet on the treadles, 6 mm", "Abgetretener und eingerissener Wollteppich auf den Tritten, 6 mm"),
     "emissivity": 0.92, "solarAbsorptivity": 0.7, "densityKgM3": 300, "conductivityWmK": 0.06, "specificHeatJKgK": 1300, "source": "literature", "reference": "en-iso-10456"},
    {"id": "rubber-cloth", "title": T("Rubberised cotton bellows cloth over pine boards", "Gummiertes Baumwoll-Balgtuch über Kiefernbrettern"),
     "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 90, "conductivityWmK": 0.08, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "gellerman-1996"},
    {"id": "leather", "title": T("Treadle strap of vegetable-tanned leather", "Trittriemen aus pflanzlich gegerbtem Leder"),
     "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 860, "conductivityWmK": 0.16, "specificHeatJKgK": 1500, "source": "literature", "reference": "incropera-2018"},
    {"id": "iron", "title": T("Blackened iron drop handles", "Brünierte eiserne Klappgriffe"),
     "emissivity": 0.7, "solarAbsorptivity": 0.8, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "literature", "reference": "incropera-2018"},
]
DENSITY = {m["id"]: m["densityKgM3"] for m in MATERIALS}


def ring_area(pts):
    """📏 Area of a ring by the shoelace formula."""
    return abs(sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))) / 2


def part_volume(p):
    """🧮 Volume of one part in cubic metres (a repeat counts every copy)."""
    z0, z1 = p.zr()
    if p.kind == "cylx":
        v = math.pi * p.geo[4] ** 2 * (p.geo[1] - p.geo[0])
    elif p.kind == "pyz":
        v = ring_area(p.geo[0]) * (p.geo[2] - p.geo[1])
    elif p.kind == "lzx":
        v = ring_area(p.geo[0]) * (p.geo[2] - p.geo[1])
    else:
        v = ring_area(p.xy()) * (z1 - z0)
    return v * p.extra.get("repeat", {}).get("count", 1)


def part_centre(p):
    """🎯 The rest centroid of a part in world (x, y), from its elevation ring."""
    pts = [world(p.bone, q) for q in p.xy()]
    if p.kind == "pyz":
        pp = p.geo[0]
        return ((p.geo[1] + p.geo[2]) / 2, sum(q[1] for q in pp) / len(pp))
    a = 0.0
    cx = cy = 0.0
    n = len(pts)
    for i in range(n):
        x0, y0 = pts[i]
        x1, y1 = pts[(i + 1) % n]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    return (cx / (3 * a), cy / (3 * a))


MASS_PARTS = parts_of(50)
MASS = {}
for _p in MASS_PARTS:
    m = part_volume(_p) * DENSITY[_p.material]
    MASS[_p.id] = (m, part_centre(_p), _p.bone)
BONE_MASS = {}
for pid, (m, c, b) in MASS.items():
    kg, sx, sy = BONE_MASS.get(b, (0.0, 0.0, 0.0))
    BONE_MASS[b] = (kg + m, sx + m * c[0], sy + m * c[1])
M_TOTAL = sum(v[0] for v in BONE_MASS.values())
COM = (sum(v[1] for v in BONE_MASS.values()) / M_TOTAL, sum(v[2] for v in BONE_MASS.values()) / M_TOTAL)

if __name__ == "__main__" and os.environ.get("HARMONIUM_MASS"):
    for pid, (m, c, b) in sorted(MASS.items(), key=lambda kv: -kv[1][0]):
        print(f"[DEBUG] {pid:24s} {b:12s} {m:7.3f} kg at {c[0]:.3f} {c[1]:.3f}")
    print(f"[DEBUG] total {M_TOTAL:.2f} kg, com {COM[0]:.4f} {COM[1]:.4f}; theta {THETA_PRESSED:.3f}; strap {STRAP_LEN:.4f} rest {STRAP_REST:.3f} pressed {STRAP_PRESSED:.3f}")
    raise SystemExit(0)

exec(open(os.path.join(HERE, "author-harmonium-model.py"), encoding="utf8").read())
