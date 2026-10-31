"""NW-H1 scratch library: kinematics (mirrors 🦴️rig solveRig), 2-link IK, handcrafted-shape helpers (rest-world coordinates -> bone-local) and
document plumbing for new v2 human assets. Coordinates and numbers are authored by the asset scripts; this file only computes."""
import math, json, glob, os

TAU = 360.0


def tx(en, de):
    return {"en": en, "de": de}


def r4(v):
    return round(v + 0.0, 4)


def wrap180(a):
    a = (a + 180.0) % 360.0 - 180.0
    return a


def find_root():
    t = glob.glob('/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE')
    return t[0]


T = find_root()
S = '/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage'
HUM = S + '/🗂️catalog/👤️humans'


# ---------- affine ----------
class Aff:
    __slots__ = ("c", "s", "x", "y", "ang")

    def __init__(self, x=0.0, y=0.0, ang=0.0):
        self.ang = ang
        self.c = math.cos(math.radians(ang))
        self.s = math.sin(math.radians(ang))
        self.x = x
        self.y = y

    def ap(self, px, py):
        return (self.x + self.c * px - self.s * py, self.y + self.s * px + self.c * py)

    def inv(self, wx, wy):
        dx, dy = wx - self.x, wy - self.y
        return (self.c * dx + self.s * dy, -self.s * dx + self.c * dy)

    def comp(self, lx, ly, lang):
        x, y = self.ap(lx, ly)
        return Aff(x, y, self.ang + lang)


class Rig:
    """bones: list of dicts {id,parent,x,y,angleDeg,lengthM,joint,...}; order parents first."""

    def __init__(self, bones, root):
        self.bones = bones
        self.by = {b["id"]: b for b in bones}
        self.root = root

    def eff(self, b, off):
        j = b["joint"]
        off = off or {}
        if j["type"] == "revolute":
            a = off.get("angleDeg", 0.0)
            if "minDeg" in j: a = max(a, j["minDeg"])
            if "maxDeg" in j: a = min(a, j["maxDeg"])
            return a, 0.0, 0.0
        if j["type"] == "prismatic":
            ax = math.radians(j.get("axisDeg", 0.0))
            s = off.get("dx", 0.0) * math.cos(ax) + off.get("dy", 0.0) * math.sin(ax)
            if "minM" in j: s = max(s, j["minM"])
            if "maxM" in j: s = min(s, j["maxM"])
            return 0.0, s * math.cos(ax), s * math.sin(ax)
        return 0.0, 0.0, 0.0

    def fk(self, offsets, root=(0.0, 0.0, 0.0)):
        w = {}
        o = Aff(root[0], root[1], root[2])
        for b in self.bones:
            a, dx, dy = self.eff(b, offsets.get(b["id"]))
            p = o if b["parent"] is None else w[b["parent"]]
            w[b["id"]] = p.comp(b["x"] + dx, b["y"] + dy, b["angleDeg"] + a)
        return w

    def rest(self):
        return self.fk({})


def lim_check(rig, offsets, tag=""):
    for bid, off in offsets.items():
        b = rig.by[bid]
        j = b["joint"]
        if j["type"] == "revolute" and "angleDeg" in off:
            a = off["angleDeg"]
            if ("minDeg" in j and a < j["minDeg"] - 1e-6) or ("maxDeg" in j and a > j["maxDeg"] + 1e-6):
                print("LIMIT", tag, bid, round(a, 2), j.get("minDeg"), j.get("maxDeg"))


# ---------- IK ----------
def ik2(base, target, l1, l2, side):
    """world angles (a1,a2) of a two-link chain from base reaching target; side=+1: first link rotated CCW from the base-target line."""
    dx, dy = target[0] - base[0], target[1] - base[1]
    d = math.hypot(dx, dy)
    reach = l1 + l2
    ok = True
    if d > reach - 1e-6:
        d = reach - 1e-6
        ok = False
    dmin = abs(l1 - l2) + 1e-6
    if d < dmin:
        d = dmin
        ok = False
    base_ang = math.degrees(math.atan2(dy, dx))
    cosa = (l1 * l1 + d * d - l2 * l2) / (2 * l1 * d)
    al = math.degrees(math.acos(max(-1.0, min(1.0, cosa))))
    a1 = base_ang + side * al
    jx = base[0] + l1 * math.cos(math.radians(a1))
    jy = base[1] + l1 * math.sin(math.radians(a1))
    tx_ = base[0] + d * math.cos(math.radians(base_ang))
    ty_ = base[1] + d * math.sin(math.radians(base_ang))
    a2 = math.degrees(math.atan2(ty_ - jy, tx_ - jx))
    return a1, a2, ok


def offsets_from_world(rig, world_angles, prismatic=None, root_angle=0.0):
    """revolute offsets so that each listed bone reaches its world angle (parents must be listed or default to rest/offset-free chain)."""
    prismatic = prismatic or {}
    offs = {}
    wang = {}
    for b in rig.bones:
        pa = root_angle if b["parent"] is None else wang[b["parent"]]
        base = pa + b["angleDeg"]
        if b["joint"]["type"] == "revolute" and b["id"] in world_angles:
            d = wrap180(world_angles[b["id"]] - base)
            offs[b["id"]] = {"angleDeg": d}
            wang[b["id"]] = base + d
        elif b["joint"]["type"] == "prismatic" and b["id"] in prismatic:
            s = prismatic[b["id"]]
            ax = math.radians(b["joint"].get("axisDeg", 0.0))
            offs[b["id"]] = {"dx": s * math.cos(ax), "dy": s * math.sin(ax)}
            wang[b["id"]] = base
        else:
            wang[b["id"]] = base
    return offs


# ---------- shapes ----------
class Shapes:
    """Collects handcrafted shapes authored in rest-world coordinates; converts to bone-local using the rest FK of the rig."""

    def __init__(self, rig):
        self.rig = rig
        self.rest = rig.rest()
        self.items = []
        self.ids = set()

    def loc(self, bone, p):
        return self.rest[bone].inv(p[0], p[1])

    def _base(self, id_, bone, role, z, filled, tone, region, regions, hatch, pen, material):
        assert id_ not in self.ids, id_
        self.ids.add(id_)
        d = {"id": id_, "bone": bone}
        return d

    def add(self, id_, bone, geom, role="body", z=1.0, filled=True, tone=0.5, region=None, regions=None, hatch=None, pen=None, material=None):
        assert id_ not in self.ids, "dup " + id_
        assert bone in self.rig.by, bone
        self.ids.add(id_)
        d = {"id": id_, "bone": bone}
        d.update(geom)
        d["role"] = role
        d["z"] = z
        d["filled"] = filled
        d["tone"] = tone
        if region: d["region"] = region
        if regions: d["regions"] = list(regions)
        if material: d["material"] = material
        if hatch: d["hatch"] = hatch
        if pen is not None: d["penMm"] = pen
        self.items.append(d)
        return d

    # geometry in WORLD-REST coordinates -> bone-local geometry dicts
    def poly(self, bone, pts, closed=True):
        return {"kind": "poly", "points": [[r4(v) for v in self.loc(bone, p)] for p in pts], "closed": closed}

    def path(self, bone, pts, closed=True, tension=1.0):
        """pts: list of (x,y) or (x,y,'c') corner points; smooth Catmull-Rom elsewhere."""
        loc = []
        for p in pts:
            q = self.loc(bone, p[:2])
            loc.append((q[0], q[1], len(p) > 2 and p[2] == 'c'))
        return {"kind": "path", "d": smooth_d(loc, closed, tension)}

    def capsule(self, bone, p0, p1, r0, r1):
        a = self.loc(bone, p0)
        b = self.loc(bone, p1)
        return {"kind": "capsule", "x0": r4(a[0]), "y0": r4(a[1]), "x1": r4(b[0]), "y1": r4(b[1]), "r0": r4(r0), "r1": r4(r1)}

    def ellipse(self, bone, c, rx, ry, rot_world=0.0):
        a = self.loc(bone, c)
        rot = rot_world - self.rest[bone].ang
        d = {"kind": "ellipse", "cx": r4(a[0]), "cy": r4(a[1]), "rx": r4(rx), "ry": r4(ry)}
        if abs(wrap180(rot)) > 1e-6: d["rotationDeg"] = r4(wrap180(rot))
        return d

    def rect(self, bone, x, y, w, h, r=None):
        pts = [(x, y), (x + w, y), (x + w, y + h), (x, y + h)]
        return self.poly(bone, pts, True)


def smooth_d(loc, closed, tension=1.0):
    n = len(loc)
    f = lambda v: ("%.4f" % v).rstrip('0').rstrip('.') if abs(v) > 5e-5 else "0"
    d = "M %s %s" % (f(loc[0][0]), f(loc[0][1]))
    rng = range(n) if closed else range(n - 1)

    def tang(i):
        if loc[i][2]: return (0.0, 0.0)
        if not closed and (i == 0 or i == n - 1): return (0.0, 0.0)
        a = loc[(i - 1) % n]
        b = loc[(i + 1) % n]
        return ((b[0] - a[0]) * 0.5 * tension, (b[1] - a[1]) * 0.5 * tension)

    for i in rng:
        p0, p1 = loc[i], loc[(i + 1) % n]
        if p0[2] and p1[2]:
            d += " L %s %s" % (f(p1[0]), f(p1[1]))
            continue
        t0, t1 = tang(i), tang((i + 1) % n)
        c1 = (p0[0] + t0[0] / 3, p0[1] + t0[1] / 3)
        c2 = (p1[0] - t1[0] / 3, p1[1] - t1[1] / 3)
        d += " C %s %s %s %s %s %s" % (f(c1[0]), f(c1[1]), f(c2[0]), f(c2[1]), f(p1[0]), f(p1[1]))
    if closed: d += " Z"
    return d


def tube(p0, p1, stations, cap0=0.0, cap1=0.0):
    """tapered outline in world-rest coordinates around axis p0->p1. stations: list of (t, halfwidth_left, halfwidth_right) (left = CCW side of the axis direction).
    returns point list (x,y) closed; caps add a rounded point beyond the ends by cap*(halfwidth)."""
    ax, ay = p1[0] - p0[0], p1[1] - p0[1]
    L = math.hypot(ax, ay)
    ux, uy = ax / L, ay / L
    nx, ny = -uy, ux
    left, right = [], []
    for t, wl, wr in stations:
        cx, cy = p0[0] + ax * t, p0[1] + ay * t
        left.append((cx + nx * wl, cy + ny * wl))
        right.append((cx - nx * wr, cy - ny * wr))
    pts = []
    t0, wl0, wr0 = stations[0]
    t1, wl1, wr1 = stations[-1]
    if cap0 > 0:
        pts.append((p0[0] + ax * t0 - ux * cap0 * (wl0 + wr0) * 0.5, p0[1] + ay * t0 - uy * cap0 * (wl0 + wr0) * 0.5))
    pts += left
    if cap1 > 0:
        pts.append((p0[0] + ax * t1 + ux * cap1 * (wl1 + wr1) * 0.5, p0[1] + ay * t1 + uy * cap1 * (wl1 + wr1) * 0.5))
    pts += right[::-1]
    return pts


def lerp(a, b, t):
    return (a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t)


def mirror_pts(pts, x0):
    return [(2 * x0 - p[0],) + tuple(p[1:]) for p in pts]


def circle_pts(c, r, n=12, ry=None):
    ry = r if ry is None else ry
    return [(c[0] + r * math.cos(2 * math.pi * i / n), c[1] + ry * math.sin(2 * math.pi * i / n)) for i in range(n)]


def write_doc(doc, asset_id, folder="👤️humans"):
    d = S + '/🗂️catalog/' + folder + '/' + asset_id
    os.makedirs(d, exist_ok=True)
    path = d + '/🔣️.json'
    with open(path, 'w') as f:
        json.dump(doc, f, indent=2, ensure_ascii=False)
        f.write('\n')
    return path


def clip_x(pts, xcut, keep):
    """Sutherland-Hodgman clip of a closed point list against the vertical line x=xcut; keep 'lt' (x<=xcut) or 'gt'. Cut points are corners."""
    inside = (lambda p: p[0] <= xcut + 1e-12) if keep == 'lt' else (lambda p: p[0] >= xcut - 1e-12)
    out = []
    n = len(pts)
    for i in range(n):
        a, b = pts[i], pts[(i + 1) % n]
        ia, ib = inside(a), inside(b)
        if ia: out.append(a)
        if ia != ib:
            t = (xcut - a[0]) / (b[0] - a[0])
            out.append((xcut, a[1] + t * (b[1] - a[1]), 'c'))
    return out
