#!/usr/bin/env python3
"""🥁️ Authoritative author script of the staffage asset `used-timpani-pair`: two second-hand balanced-action pedal timpani (29 and 26 inch, copper bowls) with padded covers and a pair of mallets.

Every hand-chosen number of the asset is stated here in world metres (x ahead = audience side, y up, z left); the script converts them to the bone-local data of the
`semio.staffage/v2` document, draws the 2D shapes of every scale and view to the same numbers as the solids and writes `🔣️.json` next to itself.

@see ../../brief.md — the binding brief of the wave
@see ../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md — the validation and rendering tools
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")
SCALES = (50, 100, 200, 500)
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
H = 0.84
CASTOR_THETA = (0.0, 120.0, 240.0)
ROD_THETA = tuple(22.5 + 45.0 * k for k in range(8))
DRUMS = (
    {"key": "a", "inch": 29, "R": 0.3685, "D": 0.40, "z": 0.463, "L": 0.42, "dents": ((330.0, 0.80, 0.030), (300.0, 0.66, 0.024), (25.0, 0.74, 0.020))},
    {"key": "b", "inch": 26, "R": 0.330, "D": 0.36, "z": -0.5015, "L": 0.38, "dents": ((270.0, 0.79, 0.028), (240.0, 0.70, 0.022), (15.0, 0.80, 0.020))},
)
COVER_SHIFT = (1.0, -(H - 0.05))
DENSITY = {"copper": 8930, "pet-film": 1390, "chrome-steel": 7850, "cast-iron": 7200, "aluminium": 2700, "rubber": 1200, "padded-nylon": 120, "felt": 300, "hickory": 800}


def v_add(a, b):
    """➕️ Sum of two 3-vectors."""
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def v_sub(a, b):
    """➖️ Difference of two 3-vectors."""
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def v_mul(a, k):
    """✖️ A 3-vector scaled by k."""
    return (a[0] * k, a[1] * k, a[2] * k)


def v_dot(a, b):
    """🔘️ Dot product."""
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def v_len(a):
    """📏️ Length of a 3-vector."""
    return math.sqrt(v_dot(a, a))


def v_unit(a):
    """🎯️ The unit vector of a."""
    n = v_len(a)
    return (a[0] / n, a[1] / n, a[2] / n)


def v_cross(a, b):
    """❎️ Cross product."""
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def m_cols(m):
    """🧱️ The three columns of a 3x3 matrix given as columns."""
    return m


def m_apply(m, p):
    """🔁️ Matrix (as columns u, v, w) times a vector."""
    return v_add(v_add(v_mul(m[0], p[0]), v_mul(m[1], p[1])), v_mul(m[2], p[2]))


def m_apply_t(m, p):
    """🔙️ Transposed matrix (columns u, v, w) times a vector: the components of p along u, v, w."""
    return (v_dot(m[0], p), v_dot(m[1], p), v_dot(m[2], p))


def m_mul(a, b):
    """🧮️ Product of two matrices given as columns."""
    return (m_apply(a, b[0]), m_apply(a, b[1]), m_apply(a, b[2]))


def rot_w(deg):
    """🔄️ The rotation about the local w axis by deg, as columns."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return ((c, s, 0.0), (-s, c, 0.0), (0.0, 0.0, 1.0))


def quat(m):
    """🌀️ The unit quaternion [x, y, z, w] of a rotation matrix given as columns."""
    r = [[m[j][i] for j in range(3)] for i in range(3)]
    t = r[0][0] + r[1][1] + r[2][2]
    if t > 0:
        s = math.sqrt(t + 1.0) * 2
        q = ((r[2][1] - r[1][2]) / s, (r[0][2] - r[2][0]) / s, (r[1][0] - r[0][1]) / s, 0.25 * s)
    elif r[0][0] > r[1][1] and r[0][0] > r[2][2]:
        s = math.sqrt(1.0 + r[0][0] - r[1][1] - r[2][2]) * 2
        q = (0.25 * s, (r[0][1] + r[1][0]) / s, (r[0][2] + r[2][0]) / s, (r[2][1] - r[1][2]) / s)
    elif r[1][1] > r[2][2]:
        s = math.sqrt(1.0 + r[1][1] - r[0][0] - r[2][2]) * 2
        q = ((r[0][1] + r[1][0]) / s, 0.25 * s, (r[1][2] + r[2][1]) / s, (r[0][2] - r[2][0]) / s)
    else:
        s = math.sqrt(1.0 + r[2][2] - r[0][0] - r[1][1]) * 2
        q = ((r[0][2] + r[2][0]) / s, (r[1][2] + r[2][1]) / s, 0.25 * s, (r[1][0] - r[0][1]) / s)
    n = math.sqrt(sum(c * c for c in q))
    return [round(c / n, 6) for c in q]


def rd(x, n=5):
    """🔢️ A rounded float without negative zero."""
    v = round(x, n)
    return 0.0 if v == 0 else v


def t(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def polar(cx, cz, r, theta):
    """🧭️ The plan point at radius r and plan angle theta (degrees from +x towards +z) about (cx, cz): returns (x, z)."""
    a = math.radians(theta)
    return (cx + r * math.cos(a), cz + r * math.sin(a))


def hull2(points):
    """🪢️ The convex hull of 2D points, counter-clockwise (monotone chain)."""
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in points))
    if len(pts) <= 2:
        return pts

    def cross(o, a, b):
        """✳️ Turn of o-a-b."""
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


class Rig:
    """🦴️ The rig with its rest frames: every bone is planar (angleDeg only), its frame is the product of the parent frame and a rotation about w."""

    def __init__(self):
        """🏗️ An empty rig."""
        self.bones = []
        self.frames = {}

    def bone(self, bid, parent, x, y, z, angle, length, joint, title):
        """➕️ Adds a bone placed at (x, y, z) of its parent and turned by angle about the parent's w."""
        b = {"id": bid, "parent": parent, "title": title, "x": rd(x), "y": rd(y), "angleDeg": rd(angle, 4), "lengthM": rd(length), "joint": joint}
        if z:
            b["z"] = rd(z)
        self.bones.append(b)
        if parent is None:
            self.frames[bid] = (rot_w(angle), (x, y, z))
        else:
            pm, po = self.frames[parent]
            self.frames[bid] = (m_mul(pm, rot_w(angle)), v_add(po, m_apply(pm, (x, y, z))))
        return b

    def local(self, bid, p):
        """📍️ The bone-local coordinates of a world point."""
        m, o = self.frames[bid]
        return m_apply_t(m, v_sub(p, o))

    def local_dir(self, bid, d):
        """🧭️ The bone-local components of a world direction."""
        return m_apply_t(self.frames[bid][0], d)

    def rel_rot(self, bid, axes):
        """🌀️ The quaternion of a world orientation (columns) relative to the bone frame."""
        m = self.frames[bid][0]
        return quat(tuple(m_apply_t(m, a) for a in axes))


RIG = Rig()


def build_rig():
    """🦴️ The rig of the pair: one fixed root, per drum the kettle, the moving tension system, the pedal, three castors with wheels, the cover and the mallet on its carrier."""
    RIG.bone("pair", None, 0.0, 0.0, 0.0, 0.0, 0.0, {"type": "fixed"}, t("Floor origin of the pair, between the two drums", "Bodenursprung des Paares, zwischen den beiden Pauken"))
    for d in DRUMS:
        k, R, z = d["key"], d["R"], d["z"]
        rc = R + 0.075
        RIG.bone(f"drum-{k}", "pair", 0.0, 0.105, z, 90.0, H - 0.105, {"type": "fixed"}, t(f"{d['inch']} inch kettle: base, struts and copper bowl, from the base hub up to the head", f"{d['inch']}-Zoll-Kessel: Fuß, Streben und Kupferkessel, von der Fußnabe bis zum Fell"))
        RIG.bone(f"hoop-{k}", f"drum-{k}", H - 0.105 - 0.008, 0.0, 0.0, 0.0, 0.0, {"type": "prismatic", "axisDeg": 0, "minM": -0.006, "maxM": 0.004}, t(f"{d['inch']} inch tension system: counter-hoop, eight rods, spider and pull tube, drawn down by the pedal", f"{d['inch']}-Zoll-Spannsystem: Spannreifen, acht Stangen, Spinne und Zugrohr, vom Pedal nach unten gezogen"))
        RIG.bone(f"pedal-{k}", f"drum-{k}", 0.010, 0.110, 0.0, 98.0, d["L"], {"type": "revolute", "minDeg": -8, "maxDeg": 5}, t(f"{d['inch']} inch tuning pedal, hinged at the base hub", f"{d['inch']}-Zoll-Stimmpedal, an der Fußnabe gelagert"))
        for i, th in enumerate(CASTOR_THETA, 1):
            px, pz = polar(0.0, 0.0, rc, th)
            RIG.bone(f"castor-{k}-{i}", f"drum-{k}", 0.090 - 0.105, -px, pz, 180.0, 0.0675, {"type": "revolute", "continuousTwist": True}, t(f"{d['inch']} inch castor {i}: fork swivelling under its base arm", f"{d['inch']}-Zoll-Rolle {i}: Gabel, unter ihrem Fußarm schwenkbar"))
            RIG.bone(f"wheel-{k}-{i}", f"castor-{k}-{i}", 0.0525, -0.03, 0.0, 0.0, 0.0, {"type": "revolute"}, t(f"{d['inch']} inch castor wheel {i} on its axle", f"{d['inch']}-Zoll-Laufrad {i} auf seiner Achse"))
        dx, dy = COVER_SHIFT
        RIG.bone(f"cover-{k}", "pair", 0.0, H - 0.05, z, 0.0, 0.0, {"type": "prismatic", "axisDeg": rd(math.degrees(math.atan2(dy, dx)), 4), "minM": 0, "maxM": rd(math.hypot(dx, dy), 4)}, t(f"Padded cover of the {d['inch']} inch drum, lifted off and set on the floor for playing", f"Gepolsterte Haube der {d['inch']}-Zoll-Pauke, zum Spielen abgenommen und auf den Boden gestellt"))
        rest, played, tilt = mallet_points(d)
        gx, gy = played[0] - rest[0], played[1] - rest[1]
        RIG.bone(f"grip-{k}", "pair", rest[0], rest[1], rest[2], 0.0, 0.0, {"type": "prismatic", "axisDeg": rd(math.degrees(math.atan2(gy, gx)), 4), "minM": 0, "maxM": rd(math.hypot(gx, gy), 4)}, t(f"Hand of the timpanist holding the {d['inch']} inch mallet: carries it from the cover to the playing spot", f"Hand des Paukers mit dem Schlägel der {d['inch']}-Zoll-Pauke: trägt ihn von der Haube zur Anschlagstelle"))
        RIG.bone(f"mallet-{k}", f"grip-{k}", 0.0, 0.0, 0.0, tilt, 0.374, {"type": "revolute", "minDeg": -40, "maxDeg": 20}, t(f"Felt mallet of the {d['inch']} inch drum, pivoting at the grip", f"Filzschlägel der {d['inch']}-Zoll-Pauke, im Griff drehend"))


def slide(bid, frac=1.0):
    """↔️ The offset of a prismatic bone moved by frac of its full travel, exactly along its axis."""
    b = next(x for x in RIG.bones if x["id"] == bid)
    a = math.radians(b["joint"]["axisDeg"])
    s = b["joint"]["maxM"] * frac
    return {"dx": s * math.cos(a), "dy": s * math.sin(a)}


def mallet_points(d):
    """🥢️ Rest butt, played butt and rest tilt of a mallet: lying on its cover, then held 40 mm above the playing spot 100 mm in from the player-side rim."""
    R, z = d["R"], d["z"]
    zm = z + 0.10
    tilt = math.degrees(math.asin((0.019 - 0.0065) / 0.355))
    rest = (-0.20, H + 0.050 + 0.0065, zm)
    ball = (-(R - 0.10), H + 0.040 + 0.019)
    a = math.radians(-12.0)
    played = (ball[0] - 0.355 * math.cos(a), ball[1] - 0.355 * math.sin(a))
    return rest, played, tilt


def ring(c, axis, r, n=24):
    """⭕️ Sample points of a circle of radius r about a world axis through c."""
    a = v_unit(axis)
    ref = (0.0, 1.0, 0.0) if abs(a[1]) < 0.9 else (1.0, 0.0, 0.0)
    e1 = v_unit(v_cross(a, ref))
    e2 = v_cross(a, e1)
    return [v_add(c, v_add(v_mul(e1, r * math.cos(2 * math.pi * i / n)), v_mul(e2, r * math.sin(2 * math.pi * i / n)))) for i in range(n)]


def profile_polygon(prof):
    """🔺️ The closed (axial, radial) polygon a revolve profile encloses: a loop as given, an open profile closed through the axis."""
    pts = list(prof)
    if pts[0] == pts[-1]:
        return pts[:-1]
    if pts[-1][1] != 0:
        pts.append((pts[-1][0], 0.0))
    if pts[0][1] != 0:
        pts.append((pts[0][0], 0.0))
    return pts


def pappus(prof):
    """⚖️ Volume and axial centroid of a revolve profile by Pappus (2 pi times area times radial centroid)."""
    pts = profile_polygon(prof)
    a = cr = ca = 0.0
    for i in range(len(pts)):
        (x0, y0), (x1, y1) = pts[i], pts[(i + 1) % len(pts)]
        c = x0 * y1 - x1 * y0
        a += c
        ca += (x0 + x1) * c
        cr += (y0 + y1) * c
    a /= 2
    if abs(a) < 1e-12:
        return 0.0, 0.0
    ca /= 6 * a
    cr /= 6 * a
    return abs(2 * math.pi * a * cr), ca


def surface(prof):
    """🧴️ Outer surface of a revolve profile: the lateral area swept by its polygon edges that are not on the axis."""
    pts = profile_polygon(prof)
    s = 0.0
    for i in range(len(pts)):
        (x0, y0), (x1, y1) = pts[i], pts[(i + 1) % len(pts)]
        if y0 == 0 and y1 == 0:
            continue
        s += math.pi * (y0 + y1) * math.hypot(x1 - x0, y1 - y0)
    return s


class Part:
    """🧩️ One solid of one scale in world rest coordinates, with its list memberships, its 2D drawing and its mass."""

    def __init__(self, pid, bone, mat, role, tone, region, geo, **kw):
        """🏗️ A part: geo is one of vrev, cyl, box, caps, ell, hrev in world metres."""
        self.id, self.bone, self.mat, self.role, self.tone, self.region, self.geo = pid, bone, mat, role, tone, region, geo
        self.slender = kw.get("slender", False)
        self.sheet = kw.get("sheet", False)
        self.texture = kw.get("texture", False)
        self.envelope = kw.get("envelope", False)
        self.member = kw.get("member", None)
        self.thermal = kw.get("thermal", not self.texture)
        self.draw = kw.get("draw", True)
        self.mass_factor = kw.get("mass_factor", 1.0)
        self.role2d = kw.get("role2d", role)
        self.corner = kw.get("corner", None)

    def solid(self):
        """💡️ The bone-local volume solid."""
        g = self.geo
        s = {"id": self.id, "bone": self.bone, "material": self.mat, "role": self.role, "tone": self.tone}
        if g["t"] == "vrev":
            s.update({"kind": "revolve", "profile": [[rd(a), rd(r)] for a, r in g["prof"]], "axis": [rd(c, 6) for c in RIG.local_dir(self.bone, (0.0, 1.0, 0.0))], "origin": [rd(c) for c in RIG.local(self.bone, (g["c"][0], 0.0, g["c"][1]))]})
        elif g["t"] == "hrev":
            s.update({"kind": "revolve", "profile": [[rd(a), rd(r)] for a, r in g["prof"]], "axis": [rd(c, 6) for c in RIG.local_dir(self.bone, v_unit(g["a"]))], "origin": [rd(c) for c in RIG.local(self.bone, g["o"])]})
        elif g["t"] == "cyl":
            ax = v_sub(g["p1"], g["p0"])
            L = v_len(ax)
            s.update({"kind": "revolve", "profile": [[0.0, 0.0], [0.0, rd(g["r"])], [rd(L), rd(g["r"])], [rd(L), 0.0]], "axis": [rd(c, 6) for c in RIG.local_dir(self.bone, v_unit(ax))], "origin": [rd(c) for c in RIG.local(self.bone, g["p0"])]})
        elif g["t"] == "box":
            s.update({"kind": "box", "size": [rd(c) for c in g["s"]], "transform": {"rotation": RIG.rel_rot(self.bone, g["ax"]), "translation": [rd(c) for c in RIG.local(self.bone, g["c"])]}})
            if self.corner:
                s["cornerRadius"] = self.corner
        elif g["t"] == "caps":
            s.update({"kind": "capsule", "p0": [rd(c) for c in RIG.local(self.bone, g["p0"])], "p1": [rd(c) for c in RIG.local(self.bone, g["p1"])], "r0": rd(g["r"]), "r1": rd(g["r"])})
        elif g["t"] == "ell":
            s.update({"kind": "ellipsoid", "radii": [rd(c) for c in g["r"]], "transform": {"rotation": RIG.rel_rot(self.bone, g["ax"]), "translation": [rd(c) for c in RIG.local(self.bone, g["c"])]}})
        for flag in ("slender", "sheet", "texture", "envelope"):
            if getattr(self, flag):
                s[flag] = True
        return s

    def points(self):
        """☁️ World sample points of a convex part for its projections."""
        g = self.geo
        if g["t"] == "box":
            pts = []
            for sx in (-1, 1):
                for sy in (-1, 1):
                    for sz in (-1, 1):
                        off = v_add(v_add(v_mul(g["ax"][0], sx * g["s"][0] / 2), v_mul(g["ax"][1], sy * g["s"][1] / 2)), v_mul(g["ax"][2], sz * g["s"][2] / 2))
                        pts.append(v_add(g["c"], off))
            return pts
        if g["t"] == "cyl":
            ax = v_sub(g["p1"], g["p0"])
            return ring(g["p0"], ax, g["r"]) + ring(g["p1"], ax, g["r"])
        if g["t"] == "caps":
            ax = v_sub(g["p1"], g["p0"])
            pts = []
            for c in (g["p0"], g["p1"]):
                for e in ((1, 0, 0), (0, 1, 0), (0, 0, 1)):
                    pts += ring(c, e, g["r"], 16)
            return pts + ring(g["p0"], ax, g["r"]) + ring(g["p1"], ax, g["r"])
        if g["t"] == "ell":
            pts = []
            for i in range(24):
                for j in range(1, 12):
                    a, b = 2 * math.pi * i / 24, math.pi * j / 12
                    loc = (g["r"][0] * math.sin(b) * math.cos(a), g["r"][1] * math.sin(b) * math.sin(a), g["r"][2] * math.cos(b))
                    pts.append(v_add(g["c"], m_apply(g["ax"], loc)))
            return pts + [v_add(g["c"], m_apply(g["ax"], (0, 0, g["r"][2]))), v_add(g["c"], m_apply(g["ax"], (0, 0, -g["r"][2])))]
        if g["t"] == "hrev":
            pts = []
            for a, r in g["prof"]:
                c = v_add(g["o"], v_mul(v_unit(g["a"]), a))
                pts += ring(c, g["a"], max(r, 1e-6))
            return pts
        raise ValueError(g["t"])

    def views(self):
        """✏️ The filled outline of the part in elevation (bone-local), front (z, y) and plan (x, z), world rest coordinates for front and plan."""
        g = self.geo
        if g["t"] == "vrev":
            cx, cz = g["c"]
            out = g["outline"]
            el = [(cx + r, y) for y, r in out] + [(cx - r, y) for y, r in reversed(out)]
            fr = [(cz + r, y) for y, r in out] + [(cz - r, y) for y, r in reversed(out)]
            elev = [tuple(RIG.local(self.bone, (x, y, cz))[:2]) for x, y in el]
            return {"elevation": ("poly", elev), "front": ("poly", fr), "plan": ("circle", (cx, cz, max(r for _, r in out)))}
        pts = self.points()
        elev = hull2([RIG.local(self.bone, p)[:2] for p in pts])
        return {"elevation": ("poly", elev), "front": ("poly", hull2([(p[2], p[1]) for p in pts])), "plan": ("poly", hull2([(p[0], p[2]) for p in pts]))}

    def depth(self):
        """🔭️ Painting keys per view: nearest point to the viewer (elevation from -z, front from +x, plan from +y)."""
        g = self.geo
        if g["t"] == "vrev":
            cx, cz = g["c"]
            rmax = max(r for _, r in g["outline"])
            ytop = max(y for y, _ in g["outline"])
            return {"elevation": -(cz - rmax), "front": cx + rmax, "plan": ytop}
        pts = self.points()
        return {"elevation": -min(p[2] for p in pts), "front": max(p[0] for p in pts), "plan": max(p[1] for p in pts)}

    def mass(self):
        """⚖️ Mass in kg and world centre of mass, from the true section and the density of the material."""
        g = self.geo
        rho = DENSITY[self.mat] * self.mass_factor
        if g["t"] == "vrev":
            vol, cy = pappus(g["prof"])
            return vol * rho, (g["c"][0], cy, g["c"][1])
        if g["t"] == "hrev":
            vol, ca = pappus(g["prof"])
            return vol * rho, v_add(g["o"], v_mul(v_unit(g["a"]), ca))
        if g["t"] == "cyl":
            return math.pi * g["r"] ** 2 * v_len(v_sub(g["p1"], g["p0"])) * rho, v_mul(v_add(g["p0"], g["p1"]), 0.5)
        if g["t"] == "box":
            return g["s"][0] * g["s"][1] * g["s"][2] * rho, g["c"]
        if g["t"] == "caps":
            return (math.pi * g["r"] ** 2 * v_len(v_sub(g["p1"], g["p0"])) + 4 / 3 * math.pi * g["r"] ** 3) * rho, v_mul(v_add(g["p0"], g["p1"]), 0.5)
        if g["t"] == "ell":
            return 4 / 3 * math.pi * g["r"][0] * g["r"][1] * g["r"][2] * rho, g["c"]
        raise ValueError(g["t"])

    def area(self):
        """🧴️ Approximate outer surface in m², for the first estimate of the region areas."""
        g = self.geo
        if g["t"] in ("vrev", "hrev"):
            return surface(g["prof"])
        if g["t"] == "cyl":
            L = v_len(v_sub(g["p1"], g["p0"]))
            return 2 * math.pi * g["r"] * L + 2 * math.pi * g["r"] ** 2
        if g["t"] == "box":
            a, b, c = g["s"]
            return 2 * (a * b + b * c + a * c)
        if g["t"] == "caps":
            return 2 * math.pi * g["r"] * v_len(v_sub(g["p1"], g["p0"])) + 4 * math.pi * g["r"] ** 2
        if g["t"] == "ell":
            a, b, c = g["r"]
            p = 1.6075
            return 4 * math.pi * (((a * b) ** p + (a * c) ** p + (b * c) ** p) / 3) ** (1 / p)
        return 0.0


EX, EY, EZ = (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)


def radial_axes(theta):
    """🧭️ Box axes for a part along the plan direction theta: radial, up, tangential."""
    a = math.radians(theta)
    r = (math.cos(a), 0.0, math.sin(a))
    return (r, EY, v_cross(r, EY))


def tilted_axes(deg):
    """📐️ Box axes of a part tilted by deg in the x-y plane (world)."""
    a = math.radians(deg)
    u = (math.cos(a), math.sin(a), 0.0)
    return (u, (-math.sin(a), math.cos(a), 0.0), EZ)


def bowl_profile(R, D, inner=True):
    """🥣️ The copper bowl: a 70 mm cylindrical collar under the rolled bearing edge, then a quarter ellipse down to the bottom; 1.2 mm spun wall."""
    ry = D - 0.07
    outer = [(H - D, 0.0)]
    for i in range(1, 12):
        a = math.pi / 2 * i / 12
        outer.append((H - 0.07 - ry * math.cos(a), R * math.sin(a)))
    outer += [(H - 0.07, R), (H - 0.010, R), (H - 0.010, R + 0.006), (H - 0.002, R + 0.006), (H, R + 0.002)]
    if not inner:
        return outer
    w = 0.0012
    inside = [(H, R - w), (H - 0.07, R - w)]
    for i in range(11, 0, -1):
        a = math.pi / 2 * i / 12
        inside.append((H - 0.07 - (ry - w) * math.cos(a), (R - w) * math.sin(a)))
    inside.append((H - D + w, 0.0))
    return outer + inside


def bowl_surface(R, D, y, theta):
    """🫙️ Radius of the outer bowl at height y and the axes (normal, up the wall, tangential) of a patch lying on it at plan angle theta."""
    ry, yc = D - 0.07, H - 0.07
    if y >= yc:
        r, nr, ny = R, 1.0, 0.0
    else:
        q = (yc - y) / ry
        r = R * math.sqrt(max(0.0, 1 - q * q))
        nr, ny = r / R ** 2, (y - yc) / ry ** 2
    a = math.radians(theta)
    n = v_unit((math.cos(a) * nr, ny, math.sin(a) * nr))
    b = v_unit(v_sub(EY, v_mul(n, v_dot(EY, n))))
    return r, (n, b, v_cross(n, b))


def drum_parts(d, s):
    """🥁️ The parts of one drum at the scale 1:s, each scale its own abstraction of the same object."""
    k, R, D, z = d["key"], d["R"], d["D"], d["z"]
    rr, rc, Rc = R + 0.045, R + 0.075, R + 0.090
    drum, hoop, pedal, cover = f"drum-{k}", f"hoop-{k}", f"pedal-{k}", f"cover-{k}"
    P = []
    add = P.append
    bp = bowl_profile(R, D)
    add(Part(f"bowl-{k}", drum, "copper", "body", 0.42, "bowl", {"t": "vrev", "c": (0.0, z), "prof": bp, "outline": bowl_profile(R, D, False)}, member=drum))
    add(Part(f"head-{k}", drum, "pet-film", "body", 0.88, "head", {"t": "vrev", "c": (0.0, z), "prof": [(H, 0.0), (H, R + 0.0045), (H + 0.0003, R + 0.0045), (H + 0.0003, 0.0)], "outline": [(H, R + 0.0045), (H + 0.0003, R + 0.0045)]}, sheet=True))
    hoop_prof = [(H - 0.014, R + 0.016), (H - 0.014, R + 0.032), (H - 0.040, R + 0.032), (H - 0.040, R + 0.036), (H - 0.008, R + 0.036), (H - 0.008, R + 0.016), (H - 0.014, R + 0.016)]
    add(Part(f"counter-hoop-{k}", hoop, "chrome-steel", "body", 0.72, "hoop", {"t": "vrev", "c": (0.0, z), "prof": hoop_prof, "outline": [(H - 0.040, R + 0.036), (H - 0.008, R + 0.036)]}, slender=True, member=hoop))
    if s in (50, 100):
        collar = [(H + 0.0003, R + 0.0045), (H - 0.003, R + 0.0075), (H - 0.026, R + 0.0075), (H - 0.026, R + 0.019), (H - 0.014, R + 0.019), (H + 0.0003, R + 0.0065), (H + 0.0003, R + 0.0045)]
        add(Part(f"collar-{k}", hoop, "aluminium", "detail", 0.8, "head", {"t": "vrev", "c": (0.0, z), "prof": collar, "outline": [(H - 0.026, R + 0.019), (H - 0.014, R + 0.019), (H + 0.0003, R + 0.0065)]}, slender=True, member=hoop, mass_factor=0.6, draw=False))
    y_sp, sp_h = 0.36, 0.016
    rod_top = {50: H + 0.022, 100: H + 0.019, 200: H + 0.034, 500: H + 0.034}[s]
    for i, th in enumerate(ROD_THETA, 1):
        x, zz = polar(0.0, z, rr, th)
        add(Part(f"rod-{k}-{i}", hoop, "chrome-steel", "detail", 0.75, "hoop", {"t": "cyl", "p0": (x, y_sp + 0.004, zz), "p1": (x, rod_top, zz), "r": 0.005}, slender=True, member=hoop))
        if s in (50, 100):
            lx, lz = polar(0.0, z, R + 0.045, th)
            add(Part(f"lug-{k}-{i}", hoop, "chrome-steel", "detail", 0.7, "hoop", {"t": "box", "c": (lx, H - 0.017, lz), "ax": radial_axes(th), "s": (0.020, 0.018, 0.024)}, member=hoop))
            bar = 0.012 if s == 50 else 0.015
            add(Part(f"handle-{k}-{i}", hoop, "chrome-steel", "detail", 0.78, "hoop", {"t": "box", "c": (x, H + 0.034 - bar / 2, zz), "ax": radial_axes(th), "s": (bar, bar, 0.060)}, member=hoop))
    for i in range(4):
        th = ROD_THETA[i]
        x0, z0 = polar(0.0, z, rr + 0.012, th)
        x1, z1 = polar(0.0, z, rr + 0.012, th + 180)
        add(Part(f"spider-{k}-{i + 1}", hoop, "cast-iron", "detail", 0.3, "frame", {"t": "box", "c": (0.0, y_sp + sp_h / 2, z), "ax": radial_axes(th), "s": (2 * (rr + 0.012), sp_h, 0.020)}, slender=s >= 200, member=hoop, mass_factor=0.5))
    if s < 500:
        add(Part(f"spider-hub-{k}", hoop, "cast-iron", "detail", 0.28, "frame", {"t": "cyl", "p0": (0.0, y_sp - 0.010, z), "p1": (0.0, y_sp + 0.026, z), "r": 0.040}, member=hoop))
        add(Part(f"pull-tube-{k}", hoop, "chrome-steel", "detail", 0.6, "frame", {"t": "cyl", "p0": (0.0, 0.140, z), "p1": (0.0, y_sp - 0.010, z), "r": 0.020}, member=hoop, mass_factor=0.55))
    else:
        add(Part(f"column-{k}", hoop, "chrome-steel", "detail", 0.5, "frame", {"t": "cyl", "p0": (0.0, 0.140, z), "p1": (0.0, y_sp + sp_h, z), "r": 0.0375}, member=hoop))
    hub_y0 = 0.075 if s == 500 else 0.095
    add(Part(f"base-hub-{k}", drum, "cast-iron", "body", 0.22, "frame", {"t": "cyl", "p0": (0.0, hub_y0, z), "p1": (0.0, 0.150, z), "r": 0.060}, member=drum))
    arm_h = (0.075, 0.075) if s == 500 else (0.035, 0.105)
    for i, th in enumerate(CASTOR_THETA, 1):
        ax_, az_ = polar(0.0, z, (rc + 0.020) / 2 + 0.02, th)
        y0 = 0.075 if s == 500 else 0.105
        y1 = 0.150 if s == 500 else 0.140
        add(Part(f"arm-{k}-{i}", drum, "cast-iron", "body", 0.22, "frame", {"t": "box", "c": (ax_, (y0 + y1) / 2, az_), "ax": radial_axes(th), "s": (rc + 0.020 - 0.04, y1 - y0, 0.075 if s == 500 else 0.050)}, member=drum, mass_factor=0.6))
        sx, sz = polar(0.0, z, rc, th)
        add(Part(f"strut-{k}-{i}", drum, "chrome-steel", "detail", 0.7, "frame", {"t": "cyl", "p0": (sx, y1, sz), "p1": (sx, H - 0.085, sz), "r": 0.0125}, slender=True, member=drum, mass_factor=0.30))
        bx, bz = polar(0.0, z, (R - 0.004 + rc + 0.0125) / 2, th)
        bs = (rc + 0.0125 - R + 0.004, 0.040, 0.030) if s < 500 else (rc + 0.0125 - R + 0.004, 0.075, 0.075)
        add(Part(f"bracket-{k}-{i}", drum, "chrome-steel", "detail", 0.65, "frame", {"t": "box", "c": (bx, H - 0.085, bz), "ax": radial_axes(th), "s": bs}, member=drum, mass_factor=0.5))
        castor, wheel = f"castor-{k}-{i}", f"wheel-{k}-{i}"
        if s < 500:
            add(Part(f"socket-{k}-{i}", drum, "chrome-steel", "detail", 0.6, "castor", {"t": "cyl", "p0": (sx, 0.090, sz), "p1": (sx, 0.105, sz), "r": 0.020}, member=drum))
        wx = sx - 0.03
        if s == 50:
            add(Part(f"tyre-{k}-{i}", wheel, "rubber", "body", 0.15, "castor", {"t": "hrev", "o": (wx, 0.0375, sz), "a": EZ, "prof": [(-0.0125, 0.0), (-0.0125, 0.033), (-0.010, 0.0375), (0.010, 0.0375), (0.0125, 0.033), (0.0125, 0.0)]}, member=wheel))
            for side in (-1, 1):
                add(Part(f"fork-{k}-{i}-{'l' if side > 0 else 'r'}", castor, "chrome-steel", "detail", 0.62, "castor", {"t": "box", "c": (wx, 0.060, sz + side * 0.0165), "ax": (EX, EY, EZ), "s": (0.050, 0.052, 0.008)}, member=castor))
            add(Part(f"crown-{k}-{i}", castor, "chrome-steel", "detail", 0.62, "castor", {"t": "box", "c": (sx - 0.0175, 0.086, sz), "ax": (EX, EY, EZ), "s": (0.075, 0.008, 0.041)}, member=castor))
        elif s == 100:
            add(Part(f"tyre-{k}-{i}", wheel, "rubber", "body", 0.15, "castor", {"t": "hrev", "o": (wx, 0.0375, sz), "a": EZ, "prof": [(-0.0125, 0.0), (-0.0125, 0.0375), (0.0125, 0.0375), (0.0125, 0.0)]}, member=wheel))
            add(Part(f"fork-{k}-{i}", castor, "chrome-steel", "detail", 0.62, "castor", {"t": "box", "c": (sx - 0.0175, 0.066, sz), "ax": (EX, EY, EZ), "s": (0.075, 0.048, 0.041)}, member=castor))
        elif s == 200:
            add(Part(f"castor-block-{k}-{i}", castor, "rubber", "body", 0.18, "castor", {"t": "box", "c": (wx, 0.045, sz), "ax": (EX, EY, EZ), "s": (0.075, 0.090, 0.041)}, member=castor, mass_factor=0.5))
        else:
            add(Part(f"castor-block-{k}-{i}", castor, "rubber", "body", 0.18, "castor", {"t": "box", "c": (sx - 0.0275, 0.0375, sz), "ax": (EX, EY, EZ), "s": (0.085, 0.075, 0.075)}, member=castor, mass_factor=0.3))
    if s < 500:
        add(Part(f"pedal-bracket-{k}", drum, "cast-iron", "body", 0.22, "frame", {"t": "box", "c": (-0.0875, 0.115, z), "ax": (EX, EY, EZ), "s": (0.075, 0.030, 0.050)}, member=drum))
    pa = math.radians(188.0)
    pu = (math.cos(pa), math.sin(pa), 0.0)
    pv = (-math.sin(pa), math.cos(pa), 0.0)
    piv = (-0.11, 0.115, z)
    L = d["L"]
    th_p = {50: 0.025, 100: 0.025, 200: 0.030, 500: 0.075}[s]
    c_p = v_add(piv, v_mul(pu, L / 2 - 0.01))
    add(Part(f"pedal-board-{k}", pedal, "aluminium", "body", 0.35, "frame", {"t": "box", "c": c_p, "ax": (pu, pv, EZ), "s": (L + 0.02, th_p, 0.100)}, member=pedal, mass_factor=0.45))
    if s == 50:
        add(Part(f"heel-rest-{k}", pedal, "rubber", "detail", 0.15, "frame", {"t": "box", "c": v_add(piv, v_add(v_mul(pu, L - 0.03), v_mul(pv, -0.0225))), "ax": (pu, pv, EZ), "s": (0.040, 0.020, 0.100)}, member=pedal))
    top = H + 0.050
    if s < 500:
        cov = [(top, 0.0), (top, Rc - 0.006), (top - 0.006, Rc), (H - 0.050, Rc), (H - 0.050, Rc - 0.010), (top - 0.016, Rc - 0.010), (top - 0.016, 0.0)]
        add(Part(f"cover-{k}", cover, "padded-nylon", "body", 0.28, "cover", {"t": "vrev", "c": (0.0, z), "prof": cov, "outline": [(H - 0.050, Rc), (top - 0.006, Rc), (top, Rc - 0.006)]}, member=cover))
    else:
        add(Part(f"cover-{k}", cover, "padded-nylon", "body", 0.28, "cover", {"t": "vrev", "c": (0.0, z), "prof": [(top, 0.0), (top, Rc), (top - 0.016, Rc), (top - 0.016, 0.0)], "outline": [(top - 0.016, Rc), (top, Rc)]}, member=cover, sheet=True))
    rest, played, tilt = mallet_points(d)
    ma = math.radians(tilt)
    mu = (math.cos(ma), math.sin(ma), 0.0)
    b0 = rest
    mallet = f"mallet-{k}"
    if s < 500:
        add(Part(f"shaft-{k}", mallet, "hickory", "detail", 0.62, "mallet", {"t": "caps", "p0": v_add(b0, v_mul(mu, 0.0065)), "p1": v_add(b0, v_mul(mu, 0.345)), "r": 0.0065}, slender=True, member=mallet))
        add(Part(f"ball-{k}", mallet, "felt", "body", 0.86, "mallet", {"t": "ell", "c": v_add(b0, v_mul(mu, 0.355)), "r": (0.019, 0.019, 0.019), "ax": (EX, EY, EZ)}, member=mallet))
    else:
        add(Part(f"shaft-{k}", mallet, "hickory", "detail", 0.62, "mallet", {"t": "caps", "p0": v_add(b0, v_mul(mu, 0.0065)), "p1": v_add(b0, v_mul(mu, 0.3675)), "r": 0.0065}, slender=True, member=mallet))
    if s == 50:
        for j, (th, y, rad) in enumerate(d["dents"], 1):
            r_y, axes = bowl_surface(R, D, y, th)
            x, zz = polar(0.0, z, r_y - 0.001, th)
            add(Part(f"dent-{k}-{j}", drum, "copper", "texture", 0.25, "bowl", {"t": "ell", "c": (x, y, zz), "r": (0.003, rad * 0.8, rad), "ax": axes}, texture=True, thermal=False))
        sx_, sz_ = polar(0.0, z, R - 0.0012, 300.0 if k == "b" else 60.0)
        add(Part(f"sticker-{k}", drum, "pet-film", "texture", 0.95, "bowl", {"t": "box", "c": (sx_, H - 0.040, sz_), "ax": radial_axes(300.0 if k == "b" else 60.0), "s": (0.003, 0.030, 0.050)}, texture=True, thermal=False))
        wx_ = -(R - 0.10)
        add(Part(f"wear-{k}", drum, "pet-film", "texture", 0.55, "head", {"t": "ell", "c": (wx_, H + 0.0003, z + 0.10), "r": (0.045, 0.0008, 0.040), "ax": (EX, EY, EZ)}, texture=True, thermal=False))
        add(Part(f"patch-{k}", cover, "padded-nylon", "texture", 0.45, "cover", {"t": "box", "c": (0.16, top + 0.0004, z - 0.12), "ax": tilted_axes(0.0), "s": (0.12, 0.0008, 0.08)}, texture=True, thermal=False))
    return P


def parts(s):
    """🧰️ All parts of the pair at the scale 1:s."""
    return [p for d in DRUMS for p in drum_parts(d, s)]


def bounds(ps):
    """📦️ World bounding box (min, max) of the body parts."""
    pts = []
    for p in ps:
        if p.texture:
            continue
        g = p.geo
        if g["t"] == "vrev":
            cx, cz = g["c"]
            rmax = max(r for _, r in g["prof"])
            ys = [y for y, _ in g["prof"]]
            pts += [(cx - rmax, min(ys), cz - rmax), (cx + rmax, max(ys), cz + rmax)]
        else:
            pts += p.points()
    return tuple(min(q[i] for q in pts) for i in range(3)), tuple(max(q[i] for q in pts) for i in range(3))


build_rig()
FINE = parts(50)
LO, HI = bounds(FINE)


def bone_masses():
    """⚖️ Bone masses from the 1:50 parts (section times density), with their local centres of mass."""
    acc = {}
    for p in FINE:
        if p.texture:
            continue
        m, c = p.mass()
        e = acc.setdefault(p.bone, [0.0, (0.0, 0.0, 0.0)])
        e[0] += m
        e[1] = v_add(e[1], v_mul(c, m))
    out = {}
    for b, (m, mc) in acc.items():
        c = v_mul(mc, 1 / m)
        out[b] = (m, c)
    return out


MASSES = bone_masses()
for bone in RIG.bones:
    if bone["id"] in MASSES:
        m, c = MASSES[bone["id"]]
        lc = RIG.local(bone["id"], c)
        bone["mass"] = {"kg": rd(m, 3), "comX": rd(lc[0], 4), "comY": rd(lc[1], 4), "distribution": "point", "source": "authored-estimate", "reference": "timpani-catalogue"}
TOTAL_KG = sum(m for m, _ in MASSES.values())


def anchors():
    """📍️ Wheel contacts, push grips, strike spots and pedal treads."""
    out = []
    for d in DRUMS:
        k, R, z = d["key"], d["R"], d["z"]
        for i in range(1, 4):
            out.append({"id": f"wheel-{k}-{i}", "bone": f"castor-{k}-{i}", "x": 0.090, "y": -0.03, "kind": "contact", "title": t(f"{d['inch']} inch castor wheel {i} on the floor", f"{d['inch']}-Zoll-Laufrad {i} auf dem Boden"), "support": "contact", "footprintLengthM": 0.02, "footprintWidthM": 0.025, "frictionCoefficient": 0.6})
        hx = RIG.local(f"hoop-{k}", (-(R + 0.036), H - 0.024, z))
        out.append({"id": f"push-{k}", "bone": f"hoop-{k}", "x": rd(hx[0]), "y": rd(hx[1]), "kind": "load", "title": t(f"Hands on the {d['inch']} inch counter-hoop, pushing", f"Hände am {d['inch']}-Zoll-Spannreifen, schiebend")})
        sx = RIG.local(f"drum-{k}", (-(R - 0.10), H + 0.0003, z + 0.10))
        out.append({"id": f"strike-{k}", "bone": f"drum-{k}", "x": rd(sx[0]), "y": rd(sx[1]), "z": 0.10, "kind": "load", "title": t(f"Playing spot of the {d['inch']} inch head, 100 mm in from the rim", f"Anschlagstelle des {d['inch']}-Zoll-Fells, 100 mm innerhalb des Randes")})
        out.append({"id": f"tread-{k}", "bone": f"pedal-{k}", "x": rd(0.72 * d["L"]), "y": 0.0125, "kind": "load", "title": t(f"Sole of the timpanist on the {d['inch']} inch pedal", f"Sohle des Paukers auf dem {d['inch']}-Zoll-Pedal")})
    return out


def solid_extra(s):
    """🏗️ Joints, supports and masses of the structure list at 1:s, sized by the smallest feature of the scale."""
    f = FEATURE[s]
    out = []
    for d in DRUMS:
        k, R, z = d["key"], d["R"], d["z"]
        rc = R + 0.075
        jr = max(f, 0.010) / 2 + 0.001
        piv = RIG.local(f"drum-{k}", (-0.11, 0.115, z))
        out.append({"id": f"joint-pedal-{k}", "kind": "ellipsoid", "bone": f"drum-{k}", "material": "cast-iron", "role": "detail", "tone": 0.5, "radii": [rd(jr)] * 3, "transform": {"translation": [rd(c) for c in piv]}, "member": f"pedal-{k}", "part": "joint"})
        tube = RIG.local(f"drum-{k}", (0.0, 0.140, z))
        out.append({"id": f"joint-pull-{k}", "kind": "ellipsoid", "bone": f"drum-{k}", "material": "chrome-steel", "role": "detail", "tone": 0.5, "radii": [rd(max(jr, 0.021))] * 3, "transform": {"translation": [rd(c) for c in tube]}, "member": f"hoop-{k}", "part": "joint"})
        for i, th in enumerate(CASTOR_THETA, 1):
            sx, sz = polar(0.0, z, rc, th)
            kp = RIG.local(f"drum-{k}", (sx, 0.0975 if s < 500 else 0.075, sz))
            out.append({"id": f"joint-swivel-{k}-{i}", "kind": "ellipsoid", "bone": f"drum-{k}", "material": "chrome-steel", "role": "detail", "tone": 0.5, "radii": [rd(max(jr, 0.012))] * 3, "transform": {"translation": [rd(c) for c in kp]}, "member": f"castor-{k}-{i}", "part": "joint"})
            if s < 500:
                ax = RIG.local(f"castor-{k}-{i}", (sx - 0.03, 0.0375, sz))
                out.append({"id": f"joint-axle-{k}-{i}", "kind": "ellipsoid", "bone": f"castor-{k}-{i}", "material": "chrome-steel", "role": "detail", "tone": 0.5, "radii": [rd(max(jr, 0.008)), rd(max(jr, 0.008)), 0.0125 if s < 200 else rd(max(jr, 0.0205))], "transform": {"translation": [rd(c) for c in ax]}, "member": f"wheel-{k}-{i}", "part": "joint"})
            ph = max(f, 0.0075)
            pad = RIG.local(f"castor-{k}-{i}", (sx - 0.03, ph / 2, sz))
            out.append({"id": f"support-wheel-{k}-{i}", "kind": "box", "bone": f"castor-{k}-{i}", "material": "rubber", "role": "ground", "tone": 0.3, "size": [rd(max(0.02, f)), rd(ph), rd(max(0.025, f))], "transform": {"translation": [rd(c) for c in pad]}, "member": f"castor-{k}-{i}", "part": "support", "anchor": f"wheel-{k}-{i}"})
    for bid, (m, c) in MASSES.items():
        if bid.startswith("wheel") or (s == 500 and (bid.startswith("castor") or bid.startswith("mallet"))):
            continue
        r = max(FEATURE[s], 0.008) / 2 + 0.0005
        lc = RIG.local(bid, c)
        member = bid
        out.append({"id": f"mass-{bid}", "kind": "ellipsoid", "bone": bid, "material": "chrome-steel", "role": "detail", "tone": 0.5, "radii": [rd(r)] * 3, "transform": {"translation": [rd(x) for x in lc]}, "member": member, "part": "mass"})
    return out


def shape_of(p, view, z, extra=None):
    """✏️ The filled 2D shape of a part in one view."""
    kind, g = p.views()[view]
    sh = {"id": p.id, "bone": p.bone, "role": p.role2d, "z": rd(z, 4), "filled": True, "tone": p.tone, "region": p.region, "material": p.mat}
    if kind == "circle":
        sh.update({"kind": "ellipse", "cx": rd(g[0]), "cy": rd(g[1]), "rx": rd(g[2]), "ry": rd(g[2])})
    else:
        sh.update({"kind": "poly", "points": [[rd(a), rd(b)] for a, b in g], "closed": True})
    if p.texture:
        sh["role"] = "texture"
    if extra:
        sh.update(extra)
    return sh


def stroke(sid, bone, view, pts, region, z, role="detail", closed=False, tone=0.3):
    """〰️ An open or closed unfilled detail stroke; elevation points are world (x, y, z) and converted to the bone, front and plan points are paper coordinates."""
    if view == "elevation":
        pts = [RIG.local(bone, q)[:2] for q in pts]
    return {"id": sid, "bone": bone, "role": role, "z": rd(z, 4), "filled": False, "tone": tone, "region": region, "kind": "poly", "points": [[rd(a), rd(b)] for a, b in pts], "closed": closed}


def details(s):
    """🖊️ The detail strokes of the scale with their painting keys: bowl seams, cover seams, dents and the head rim, only where the scale can print them."""
    out = {"elevation": [], "front": [], "plan": []}
    if s == 500:
        return out
    for d in DRUMS:
        k, R, z = d["key"], d["R"], d["z"]
        el_key, fr_key = -(z - R) + 0.0005, R + 0.0005
        out["elevation"].append((el_key, stroke(f"collar-seam-{k}", f"drum-{k}", "elevation", [(-R, H - 0.07, z), (R, H - 0.07, z)], "bowl", 0)))
        out["front"].append((fr_key, stroke(f"collar-seam-{k}", f"drum-{k}", "front", [(z - R, H - 0.07), (z + R, H - 0.07)], "bowl", 0)))
        if s != 50:
            continue
        Rc = R + 0.090
        out["elevation"].append((-(z - Rc) + 0.0005, stroke(f"cover-seam-{k}", f"cover-{k}", "elevation", [(-Rc + 0.004, H + 0.034, z), (Rc - 0.004, H + 0.034, z)], "cover", 0, tone=0.2)))
        out["front"].append((Rc + 0.0005, stroke(f"cover-seam-{k}", f"cover-{k}", "front", [(z - Rc + 0.004, H + 0.034), (z + Rc - 0.004, H + 0.034)], "cover", 0, tone=0.2)))
        for j, (th, y, rad) in enumerate(d["dents"], 1):
            x, zz = polar(0.0, z, R, th)
            sn, cs = math.sin(math.radians(th)), math.cos(math.radians(th))
            if k == "b" and sn < -0.5:
                lc = RIG.local(f"drum-{k}", (x, y, zz))
                out["elevation"].append((el_key + 0.0002, {"id": f"dent-mark-{k}-{j}", "bone": f"drum-{k}", "role": "detail", "z": 0, "filled": False, "tone": 0.25, "region": "bowl", "kind": "ellipse", "cx": rd(lc[0]), "cy": rd(lc[1]), "rx": rd(rad * 0.8), "ry": rd(rad * abs(sn))}))
            if cs > 0.5:
                out["front"].append((fr_key + 0.0002, {"id": f"dent-mark-{k}-{j}", "bone": f"drum-{k}", "role": "detail", "z": 0, "filled": False, "tone": 0.25, "region": "bowl", "kind": "ellipse", "cx": rd(zz), "cy": rd(y), "rx": rd(rad * abs(cs)), "ry": rd(rad * 0.8)}))
        out["plan"].append((H + 0.0005, {"id": f"rim-line-{k}", "bone": f"drum-{k}", "role": "detail", "z": 0, "filled": False, "tone": 0.3, "region": "head", "kind": "ellipse", "cx": 0.0, "cy": rd(z), "rx": rd(R + 0.002), "ry": rd(R + 0.002)}))
    return out


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7}, 100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def representation(s, ps):
    """🗺️ The authored representation of 1:s: every visible part as a filled outline in its painting order, plus the detail strokes of the scale."""
    geo = {"elevation": [], "front": [], "plan": []}
    strokes = details(s)
    drawn = [p for p in ps if p.draw and not (p.texture and PENS[s]["texture"] is None)]
    if s == 500:
        for p in drawn:
            p.role2d = "body"
    for view in geo:
        keyed = sorted([(p.depth()[view], i, p) for i, p in enumerate(drawn)] + [(key, 10000 + i, sh) for i, (key, sh) in enumerate(strokes[view])], key=lambda e: (e[0], e[1]))
        for n, (_, _, item) in enumerate(keyed):
            z = rd(1.0 + n * 0.01, 4)
            sh = shape_of(item, view, z) if isinstance(item, Part) else {**item, "z": z}
            if s > 50 and sh["filled"] and (PENS[s][sh["role"]] or 0) < 0.2:
                sh["penMm"] = 0.2
            geo[view].append(sh)
    return {"status": "authored", "denominator": s, "validRange": {"minDenominator": RANGES[s][0], "maxDenominator": RANGES[s][1]}, "pens": PENS[s], "hatches": [], "geometry": geo, "decisions": REP_DECISIONS[s]}


def volume(s, ps):
    """🧊️ The authored volume of 1:s: body solids, thermal references by region and the structure of members, joints, supports and masses."""
    body = [p.solid() for p in ps]
    thermal = [{"id": f"th-{p.id}", "of": p.id, "region": p.region} for p in ps if p.thermal]
    structure = [{"id": f"st-{p.id}", "of": p.id, "member": p.member, "part": "member"} for p in ps if p.member and not p.texture]
    structure += solid_extra(s)
    return {"status": "authored", "denominator": s, "body": body, "thermal": thermal, "structure": structure, "decisions": VOL_DECISIONS[s]}


from texts import ASSUMPTIONS_STRUCT, ASSUMPTIONS_THERMAL, NOTES, REFERENCES, REP_DECISIONS, SUMMARY, TITLE, VOL_DECISIONS, pose_texts, clip_texts  # noqa: E402

MATERIALS = [
    {"id": "copper", "title": t("Spun copper, tarnished", "Gedrücktes Kupfer, angelaufen"), "emissivity": 0.5, "solarAbsorptivity": 0.65, "densityKgM3": 8930, "conductivityWmK": 390, "specificHeatJKgK": 385, "elasticModulusPa": 1.17e11, "strengthPa": 2.0e8, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "pet-film", "title": t("Polyester (PET) drumhead film", "Polyester-Fellfolie (PET)"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "densityKgM3": 1390, "conductivityWmK": 0.15, "specificHeatJKgK": 1200, "elasticModulusPa": 4.5e9, "strengthPa": 1.7e8, "source": "literature", "reference": "fletcher-rossing-1998"},
    {"id": "chrome-steel", "title": t("Chrome-plated steel, dulled", "Verchromter Stahl, matt geworden"), "emissivity": 0.2, "solarAbsorptivity": 0.4, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "elasticModulusPa": 2.1e11, "strengthPa": 2.35e8, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "cast-iron", "title": t("Cast iron, black enamel, chipped", "Gusseisen, schwarz lackiert, abgestoßen"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 7200, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 1.1e11, "strengthPa": 2.0e8, "source": "literature", "reference": "incropera-2018"},
    {"id": "aluminium", "title": t("Cast aluminium, worn paint", "Aluminiumguss, abgenutzter Lack"), "emissivity": 0.4, "solarAbsorptivity": 0.5, "densityKgM3": 2700, "conductivityWmK": 160, "specificHeatJKgK": 900, "elasticModulusPa": 7.0e10, "strengthPa": 1.5e8, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "rubber", "title": t("Grey rubber", "Grauer Gummi"), "emissivity": 0.92, "solarAbsorptivity": 0.8, "densityKgM3": 1200, "conductivityWmK": 0.2, "specificHeatJKgK": 1500, "source": "literature", "reference": "incropera-2018"},
    {"id": "padded-nylon", "title": t("Padded nylon cover over foam and fibreboard, faded", "Gepolsterte Nylonhaube über Schaum und Faserplatte, ausgeblichen"), "emissivity": 0.85, "solarAbsorptivity": 0.7, "densityKgM3": 120, "conductivityWmK": 0.04, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "felt", "title": t("Wool felt, compressed by playing", "Wollfilz, vom Spielen verdichtet"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 300, "conductivityWmK": 0.05, "specificHeatJKgK": 1300, "source": "literature", "reference": "incropera-2018"},
    {"id": "hickory", "title": t("Hickory, oiled", "Hickory, geölt"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 800, "conductivityWmK": 0.16, "specificHeatJKgK": 1600, "elasticModulusPa": 1.5e10, "strengthPa": 1.0e8, "source": "literature", "reference": "incropera-2018"},
]

REGIONS = [
    ("bowl", "Copper bowls", "Kupferkessel", "copper", 0.74, "sphere", "metal", 0.0012, "Tarnished spun copper 1.2 mm; the aged oxide film raises the emissivity from about 0.03 polished to about 0.5.", "Angelaufenes gedrücktes Kupfer 1,2 mm; die gealterte Oxidschicht hebt den Emissionsgrad von etwa 0,03 poliert auf etwa 0,5."),
    ("head", "Drumheads and collars", "Felle und Kragen", "pet-film", 0.70, "horizontal-plate-up", "polymer", 0.00025, "Polyester film 0.25 mm on aluminium flesh hoops.", "Polyesterfolie 0,25 mm auf Aluminium-Fellreifen."),
    ("hoop", "Counter-hoops, tension rods and tuning handles", "Spannreifen, Spannstangen und Stimmgriffe", "chrome-steel", 0.03, "cylinder", "metal", 0.004, "Chrome-plated steel, dulled by handling.", "Verchromter Stahl, vom Anfassen matt."),
    ("frame", "Base, struts, spider and pedals", "Fuß, Streben, Spinne und Pedale", "cast-iron", 0.05, "cylinder", "metal", 0.010, "Enamelled cast iron and chromed steel tubes.", "Lackiertes Gusseisen und verchromte Stahlrohre."),
    ("castor", "Castors", "Laufrollen", "rubber", 0.075, "cylinder", "polymer", 0.010, "Grey rubber wheels in chromed forks.", "Graue Gummiräder in verchromten Gabeln."),
    ("cover", "Padded covers", "Gepolsterte Hauben", "padded-nylon", 0.9, "horizontal-plate-up", "insulation", 0.016, "Faded nylon over 12 mm foam and a 4 mm fibreboard disc.", "Ausgeblichenes Nylon über 12 mm Schaum und einer 4-mm-Faserplatte."),
    ("mallet", "Felt mallets", "Filzschlägel", "felt", 0.038, "cylinder", "polymer", 0.010, "Wool felt heads on hickory shafts.", "Wollfilzköpfe auf Hickoryschäften."),
]

AREA_OVERRIDE = {}
if os.path.exists(os.path.join(HERE, "areas.json")):
    AREA_OVERRIDE = json.load(open(os.path.join(HERE, "areas.json")))


def thermal_model():
    """🌡️ The passive thermal model of the depot: seven regions with real material data, no emitters, no rating (0 W stated on the bowls)."""
    areas = {}
    for p in FINE:
        if p.thermal:
            areas[p.region] = areas.get(p.region, 0.0) + p.area()
    regions = []
    for rid, en, de, mat, L, geom, lkind, thick, len_, lde in REGIONS:
        m = next(x for x in MATERIALS if x["id"] == mat)
        cap = rd(m["densityKgM3"] * m["specificHeatJKgK"] * thick, 1)
        reg = {"id": rid, "title": t(en, de), "kind": "surface", "model": "passive", "material": mat, "areaM2": rd(AREA_OVERRIDE.get(rid, areas[rid]), 4), "characteristicLengthM": L, "geometry": geom, "skyViewFactor": 0.5,
               "layers": [{"id": f"{rid}-skin", "title": t(len_, lde), "kind": lkind, "material": mat, "resistanceM2KW": rd(thick / m["conductivityWmK"], 6), "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": "incropera-2018"}],
               "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": "incropera-2018"}
        if rid == "bowl":
            reg["ratedPowerW"] = 0
            reg["ratedPowerBasis"] = "passive"
        regions.append(reg)
    return {"environment": {"airTempC": 18, "meanRadiantTempC": 18, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325}, "passengers": [], "operating": [], "regions": regions, "emitters": [], "assumptions": ASSUMPTIONS_THERMAL}


def binding(state):
    """🌬️ The region binding of a pose: covered heads when stored, a hand on the hoops when rolled, uncovered heads and covers on the floor when played."""
    covered = state != "played"
    return {"regions": {
        "bowl": {"exposedFraction": 0.97, "sunlitFraction": 0},
        "head": {"exposedFraction": 0.3 if covered else 1.0, "sunlitFraction": 0},
        "hoop": {"exposedFraction": 0.5 if covered else 1.0, "sunlitFraction": 0},
        "frame": {"exposedFraction": 1.0, "sunlitFraction": 0},
        "castor": {"exposedFraction": 0.85, "sunlitFraction": 0, "contact": {"fraction": 0.05, "tempC": 18}},
        "cover": {"exposedFraction": 1.0 if covered else 0.55, "sunlitFraction": 0, **({} if covered else {"contact": {"fraction": 0.1, "tempC": 18}})},
        "mallet": {"exposedFraction": 0.75 if covered else 0.6, "sunlitFraction": 0, "contact": {"fraction": 0.05 if covered else 0.3, "tempC": 18 if covered else 33}},
    }, "emitters": {}}


WHEEL_ROLL = -math.degrees(0.6 / 0.0375)
CONTACTS = [f"wheel-{d['key']}-{i}" for d in DRUMS for i in range(1, 4)]


def poses():
    """🧍️ Stored (default), rolled and played, with the loads of each."""
    pt = pose_texts()
    played_bones = {}
    carried_bones = {}
    for d in DRUMS:
        k = d["key"]
        tilt = mallet_points(d)[2]
        played_bones[f"cover-{k}"] = slide(f"cover-{k}")
        played_bones[f"grip-{k}"] = slide(f"grip-{k}")
        played_bones[f"mallet-{k}"] = {"angleDeg": rd(-12.0 - tilt, 3)}
        for i in range(1, 4):
            carried_bones[f"wheel-{k}-{i}"] = {"angleDeg": rd(WHEEL_ROLL, 2)}
    loads_carried = [{"id": f"push-{d['key']}", "kind": "live", "title": t(f"Push on the {d['inch']} inch hoop", f"Schub am {d['inch']}-Zoll-Reifen"), "anchor": f"push-{d['key']}", "fxN": 40, "fyN": 0, "source": "authored-estimate", "reference": "timpani-catalogue"} for d in DRUMS]
    loads_played = [{"id": f"tread-{d['key']}", "kind": "live", "title": t(f"Foot resting on the {d['inch']} inch pedal", f"Fuß auf dem {d['inch']}-Zoll-Pedal"), "anchor": f"tread-{d['key']}", "fxN": 0, "fyN": -60, "source": "authored-estimate", "reference": "timpani-catalogue"} for d in DRUMS]
    loads_played += [{"id": f"strike-{d['key']}", "kind": "live", "title": t(f"Mallet stroke on the {d['inch']} inch head (peak)", f"Schlägelschlag auf das {d['inch']}-Zoll-Fell (Spitze)"), "anchor": f"strike-{d['key']}", "fxN": 0, "fyN": -120, "source": "literature", "reference": "fletcher-rossing-1998"} for d in DRUMS]
    return [
        {"id": "stored", **pt["stored"], "bones": {}, "ground": "auto", "contacts": CONTACTS, "thermal": binding("stored"), "loads": []},
        {"id": "carried", **pt["carried"], "bones": carried_bones, "root": {"x": 0.6}, "ground": "auto", "contacts": CONTACTS, "thermal": binding("carried"), "loads": loads_carried},
        {"id": "played", **pt["played"], "bones": played_bones, "ground": "auto", "contacts": CONTACTS, "thermal": binding("played"), "loads": loads_played},
    ]


def clips():
    """🎞️ Rolling the pair out of the depot, two strokes with rebound, and a pedal glissando."""
    ct = clip_texts()
    spans = lambda dur: [{"anchor": c, "fromS": 0, "toS": dur} for c in CONTACTS]
    tilt = {d["key"]: mallet_points(d)[2] for d in DRUMS}
    ready = {k: -12.0 - tilt[k] for k in tilt}
    hit = {k: ready[k] - 6.7 for k in tilt}
    lift = {k: ready[k] + 14.0 for k in tilt}

    def mal(k, ang):
        """🥢️ A mallet offset in a key."""
        return {f"mallet-{k}": {"angleDeg": rd(ang, 3)}}

    def both(ga, gb):
        """🥢️ Both mallets in one key, the grips held at the playing spot."""
        out = {}
        for d in DRUMS:
            k = d["key"]
            out[f"grip-{k}"] = slide(f"grip-{k}")
            out[f"cover-{k}"] = slide(f"cover-{k}")
        out.update(mal("a", ga))
        out.update(mal("b", gb))
        return out

    strike_keys = [
        {"t": 0.0, "pose": "played", "ease": "smooth", "bones": both(lift["a"], ready["b"])},
        {"t": 0.14, "pose": "played", "ease": "in", "bones": both(ready["a"], ready["b"])},
        {"t": 0.2, "pose": "played", "ease": "out", "bones": both(hit["a"], ready["b"])},
        {"t": 0.36, "pose": "played", "ease": "smooth", "bones": both(lift["a"], lift["b"])},
        {"t": 0.74, "pose": "played", "ease": "in", "bones": both(ready["a"], ready["b"])},
        {"t": 0.8, "pose": "played", "ease": "out", "bones": both(ready["a"], hit["b"])},
        {"t": 0.96, "pose": "played", "ease": "smooth", "bones": both(lift["a"], lift["b"])},
        {"t": 1.2, "pose": "played", "ease": "smooth", "bones": both(lift["a"], ready["b"])},
    ]
    pedal_keys = []
    for tt, ang, dx in ((0.0, -6.0, 0.004), (1.5, 5.0, -0.004), (3.0, -6.0, 0.004)):
        b = both(ready["a"], ready["b"])
        for d in DRUMS:
            b[f"pedal-{d['key']}"] = {"angleDeg": ang}
            b[f"hoop-{d['key']}"] = {"dx": dx}
        pedal_keys.append({"t": tt, "pose": "played", "ease": "smooth", "bones": b})
    roll = []
    for tt, f in ((0.0, 0.0), (4.0, 1.0)):
        b = {f"wheel-{d['key']}-{i}": {"angleDeg": rd(WHEEL_ROLL * f, 2)} for d in DRUMS for i in range(1, 4)}
        roll.append({"t": tt, "pose": "stored" if f == 0 else "carried", "ease": "smooth", "bones": b})
    return [
        {"id": "carry", **ct["carry"], "durationS": 4.0, "fps": 12, "loop": False, "rootMotion": "none", "keys": roll, "contacts": spans(4.0),
         "phases": [{"id": "release", "fromS": 0, "toS": 0.6, "title": t("Brakes released, hands on the hoops", "Bremsen gelöst, Hände an den Reifen")}, {"id": "roll", "fromS": 0.6, "toS": 4.0, "title": t("Rolling", "Rollen")}],
         "events": [{"t": 0.6, "kind": "motion", "title": t("The pair starts to roll", "Das Paar beginnt zu rollen")}],
         "tracks": [{"target": f"load:push-{d['key']}", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 0.6, "v": 1}, {"t": 3.4, "v": 1}, {"t": 4.0, "v": 0.2}]} for d in DRUMS]},
        {"id": "strike", **ct["strike"], "durationS": 1.2, "fps": 50, "loop": True, "rootMotion": "none", "keys": strike_keys, "contacts": spans(1.2),
         "phases": [{"id": "stroke-29", "fromS": 0, "toS": 0.6, "title": t("Stroke on the 29 inch drum", "Schlag auf die 29-Zoll-Pauke")}, {"id": "stroke-26", "fromS": 0.6, "toS": 1.2, "title": t("Stroke on the 26 inch drum", "Schlag auf die 26-Zoll-Pauke")}],
         "events": [{"t": 0.2, "kind": "contact-on", "title": t("Felt touches the 29 inch head", "Filz trifft das 29-Zoll-Fell"), "target": "strike-a"}, {"t": 0.8, "kind": "contact-on", "title": t("Felt touches the 26 inch head", "Filz trifft das 26-Zoll-Fell"), "target": "strike-b"}],
         "tracks": [{"target": "load:strike-a", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 0.198, "v": 0}, {"t": 0.2, "v": 1}, {"t": 0.204, "v": 0}, {"t": 1.2, "v": 0}]}, {"target": "load:strike-b", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 0.798, "v": 0}, {"t": 0.8, "v": 1}, {"t": 0.804, "v": 0}, {"t": 1.2, "v": 0}]}]},
        {"id": "pedal-tuning", **ct["pedal-tuning"], "durationS": 3.0, "fps": 25, "loop": True, "rootMotion": "none", "keys": pedal_keys, "contacts": spans(3.0),
         "phases": [{"id": "up", "fromS": 0, "toS": 1.5, "title": t("Toe down: pitch rises", "Fußspitze nach unten: Tonhöhe steigt")}, {"id": "down", "fromS": 1.5, "toS": 3.0, "title": t("Heel down: pitch falls", "Ferse nach unten: Tonhöhe sinkt")}],
         "events": [{"t": 1.5, "kind": "motion", "title": t("Highest pitch, about a fifth above the lowest", "Höchster Ton, etwa eine Quinte über dem tiefsten")}],
         "tracks": [{"target": f"load:tread-{d['key']}", "ease": "smooth", "keys": [{"t": 0, "v": 0.8}, {"t": 1.5, "v": 2.0}, {"t": 3.0, "v": 0.8}]} for d in DRUMS]},
    ]


def document():
    """📄️ The whole asset document."""
    reps, vols = {}, {}
    for s in SCALES:
        ps = parts(s)
        reps[str(s)] = representation(s, ps)
        vols[str(s)] = volume(s, ps)
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": {
        "id": "used-timpani-pair", "category": "objects", "title": TITLE, "summary": SUMMARY,
        "tags": ["used", "instrument", "music", "second-hand", "percussion", "timpani", "kettledrum", "membranophone", "pedal", "copper", "castors", "orchestra", "pair"],
        "provenance": {"author": "Semio staffage content (used instruments wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": NOTES},
        "dimensions": {"heightM": rd(HI[1] - LO[1], 3), "widthM": rd(HI[0] - LO[0], 3), "depthM": rd(HI[2] - LO[2], 3), "massKg": rd(TOTAL_KG, 2)},
        "materials": MATERIALS, "rig": {"kind": "mechanical", "root": "pair", "bones": RIG.bones}, "volumes": vols, "anchors": anchors(), "representations": reps, "symbols": [],
        "poses": poses(), "clips": clips(), "thermal": thermal_model(),
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": CONTACTS, "assumptions": ASSUMPTIONS_STRUCT, "clinical": False},
        "presentation": {"defaultPose": "stored", "defaultClip": "strike", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"], "completion": "authored"}}


if __name__ == "__main__":
    doc = document()
    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, separators=(",", ":"))
    print(f"[DEBUG] wrote {OUT}: dims {doc['asset']['dimensions']}, bounds {LO} {HI}")
    for bid, (m, c) in sorted(MASSES.items()):
        print(f"[DEBUG] mass {bid}: {m:.2f} kg")
    for s in SCALES:
        v = doc["asset"]["volumes"][str(s)]
        print(f"[DEBUG] 1:{s} body {len(v['body'])} thermal {len(v['thermal'])} structure {len(v['structure'])}")
