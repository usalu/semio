"""🎼 Authoritative generator of the used 3.5-octave rosewood xylophone (`used-xylophone`).

Every number is stated here by hand; the script writes `🔣️.json` next to it (rig, anchors, 2D at four scales,
volumes at four scales, poses, clips, thermal, structure) from one parts description per scale, so the
drawings and the solids are the same geometry. Run: `python3 author-xylophone.py`.

See also:
- `../../brief.md` (binding brief of the wave)
- `🎓️teaching/🏛️architecture/🧍️staffage/README.md` (contract)
- https://doi.org/10.1007/978-0-387-21603-4 (Fletcher & Rossing, The Physics of Musical Instruments)
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")

PITCH = 0.052
BAR_W = 0.040
BAR_T = 0.022
ROW_GAP = 0.030
L0, SLOPE = 0.285, 0.19 / 1.30
NAT_Y0, NAT_Y1 = 0.848, 0.870
ACC_Y0, ACC_Y1 = 0.868, 0.890
CHEEK_X0, CHEEK_X1 = 0.680, 0.705
CHEEK_Y0, CHEEK_Y1 = 0.770, 0.860
LEG_X = 0.6925
FOOT_Y0, FOOT_Y1 = 0.085, 0.115
FOOT_Z = 0.33
LEG_Z = {"bass": 0.26, "treble": 0.13}
LEG_R = 0.014
CROSS_Y, CROSS_R = 0.48, 0.0125
CASTOR_Z = 0.31
WHEEL_R, WHEEL_HW, TRAIL = 0.0375, 0.012, 0.03
TUBE_R, TUBE_WALL, TUBE_CAP = 0.020, 0.001, 0.003
NAT_TUBE_TOP, ACC_TUBE_TOP = 0.828, 0.848
NAT_BANK_Y, ACC_BANK_Y = (0.790, 0.820), (0.810, 0.840)
MALLET_SHAFT_R, MALLET_HEAD_R, MALLET_HEAD_U, MALLET_LEN, MALLET_SHAFT_END = 0.0045, 0.015, 0.365, 0.380, 0.3505
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.030, 500: 0.075}
NAMES = ["F", "G", "A", "B", "C", "D", "E"]
SEMIS = [0, 2, 4, 6, 7, 9, 11]
CHIPPED_NAT = {1: 0.010, 3: 0.012, 9: 0.009, 14: 0.011, 17: 0.010}
CHIPPED_ACC = {1: 0.010, 11: 0.009}
REPLACED_NAT = 12
MALLET_BARS = {"mallet-bass": 4, "mallet-treble": 6}
MALLET_HEAD_Z = -0.19
DENSITY = {"rosewood": 1030, "padauk": 750, "beech": 720, "alu": 2700, "steel": 7800, "zinc": 7800, "rubber": 1200, "rattan": 600, "phenolic": 1350, "tape": 1100}


def L(x):
    """📏 Bar length (along z) at keyboard position x, linear between the bass F3 (0.380 m) and the top C7 (0.190 m)."""
    return L0 - SLOPE * x


def r6(v):
    """🔢 Rounds a number to micrometres for the document."""
    return round(float(v), 6)


def t(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def note_of(i):
    """🎹 Name and semitone index from F3 of natural number i (0 = F3)."""
    octave = 3 + (i + 3) // 7
    return f"{NAMES[i % 7]}{octave}", SEMIS[i % 7] + 12 * (i // 7)


NATURALS = []
for i in range(26):
    name, semi = note_of(i)
    NATURALS.append({"i": i, "name": name, "semi": semi, "x": (i - 12.5) * PITCH})
ACCIDENTALS = []
for i in range(24):
    if i % 7 in (0, 1, 2, 4, 5):
        name, semi = note_of(i)
        ACCIDENTALS.append({"i": i, "name": name + "s", "semi": semi + 1, "x": (i - 12.5) * PITCH + PITCH / 2})
assert len(NATURALS) == 26 and len(ACCIDENTALS) == 18


def resonator_length(semi):
    """🎺 Quarter-wave resonator length of a note sounding one octave above written pitch (F4 = 349.23 Hz), end correction 0.61 r, at least 0.10 m (stopped tubes above)."""
    f = 349.228 * 2 ** (semi / 12)
    return max(0.10, 343.0 / (4 * f) - 0.61 * TUBE_R)


def zc_nat(x):
    """↔️ Lateral centre of a natural bar (player side is -z)."""
    return -ROW_GAP / 2 - L(x) / 2


def zc_acc(x):
    """↔️ Lateral centre of an accidental bar."""
    return ROW_GAP / 2 + L(x) / 2


def mat_mul(a, b):
    """✖️ 3x3 matrix product."""
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def mat_vec(a, v):
    """➡️ 3x3 matrix times vector."""
    return [sum(a[i][k] * v[k] for k in range(3)) for i in range(3)]


def transpose(a):
    """🔄 Transposed 3x3 matrix."""
    return [[a[j][i] for j in range(3)] for i in range(3)]


def rot(angle, abd, twist):
    """🧭 Rest rotation Rw(angle) Rv(abduction) Ru(twist) of the README chain."""
    a, b, c = (math.radians(v) for v in (angle, abd, twist))
    rw = [[math.cos(a), -math.sin(a), 0], [math.sin(a), math.cos(a), 0], [0, 0, 1]]
    rv = [[math.cos(b), 0, -math.sin(b)], [0, 1, 0], [math.sin(b), 0, math.cos(b)]]
    ru = [[1, 0, 0], [0, math.cos(c), math.sin(c)], [0, -math.sin(c), math.cos(c)]]
    return mat_mul(mat_mul(rw, rv), ru)


class Frame:
    """🦴 The rest frame of a bone in world metres: origin and the columns u, v, w."""

    def __init__(self, o, r):
        self.o, self.r = o, r

    def local(self, p):
        return mat_vec(transpose(self.r), [p[0] - self.o[0], p[1] - self.o[1], p[2] - self.o[2]])

    def local_dir(self, d):
        return mat_vec(transpose(self.r), d)

    def axis_aligned(self):
        return all(abs(abs(v) - round(abs(v))) < 1e-9 for row in self.r for v in row)


BONES = []
FRAMES = {}


def bone(id, parent, x, y, angle, length, joint, title, z=0.0, abd=0.0, twist=0.0):
    """➕ Adds a bone and solves its rest frame."""
    b = {"id": id, "parent": parent, "title": title, "x": r6(x), "y": r6(y), "angleDeg": r6(angle), "lengthM": r6(length), "joint": joint}
    if z:
        b["z"] = r6(z)
    if abd:
        b["abductionDeg"] = r6(abd)
    if twist:
        b["twistDeg"] = r6(twist)
    BONES.append(b)
    rr = rot(angle, abd, twist)
    if parent is None:
        FRAMES[id] = Frame([x, y, z], rr)
    else:
        p = FRAMES[parent]
        off = mat_vec(p.r, [x, y, z])
        FRAMES[id] = Frame([p.o[0] + off[0], p.o[1] + off[1], p.o[2] + off[2]], mat_mul(p.r, rr))
    return b


FIXED = {"type": "fixed"}
bone("stand", None, -LEG_X, 0.10, 0, 2 * LEG_X, FIXED, t("Stand: central square stretcher tube between the two end frames", "Gestell: mittleres Vierkant-Längsrohr zwischen den beiden Endrahmen"))
for end, x in (("bass", 0.0), ("treble", 2 * LEG_X)):
    bone(f"leg-{end}", "stand", x, 0.0, 90, 0.67, FIXED, t(f"{'Bass' if end == 'bass' else 'Treble'} end frame: foot bar, two legs and the cross tube", f"{'Bass' if end == 'bass' else 'Diskant'}-Endrahmen: Fußrohr, zwei Beine und Querrohr"))
bone("keyboard", "stand", 0.0, CHEEK_Y0 - 0.10, 0, 2 * LEG_X, FIXED, t("Keyboard frame: two end cheeks and four felted rails", "Klaviaturrahmen: zwei Stirnwangen und vier befilzte Auflageleisten"))
bone("naturals", "keyboard", -0.65 + LEG_X, NAT_Y0 - CHEEK_Y0, 0, 1.30, FIXED, t("Row of the 26 natural bars F3 to C7", "Reihe der 26 Stammtonplatten F3 bis C7"))
bone("accidentals", "keyboard", ACCIDENTALS[0]["x"] + LEG_X, ACC_Y0 - CHEEK_Y0, 0, ACCIDENTALS[-1]["x"] - ACCIDENTALS[0]["x"], FIXED, t("Raised row of the 18 accidental bars F#3 to A#6", "Erhöhte Reihe der 18 Halbtonplatten Fis3 bis Ais6"))
bone("resonators-naturals", "keyboard", -CHEEK_X0 + LEG_X, NAT_BANK_Y[0] - CHEEK_Y0, 0, 2 * CHEEK_X0, FIXED, t("Resonator bank under the naturals: 26 tubes on two bank bars", "Resonatorbank unter den Stammtönen: 26 Rohre an zwei Bankleisten"))
bone("resonators-accidentals", "keyboard", -CHEEK_X0 + LEG_X, ACC_BANK_Y[0] - CHEEK_Y0, 0, 2 * CHEEK_X0, FIXED, t("Resonator bank under the accidentals: 18 tubes on two bank bars", "Resonatorbank unter den Halbtönen: 18 Rohre an zwei Bankleisten"))
CASTORS = []
for end in ("bass", "treble"):
    for side, sz in (("r", -1), ("l", 1)):
        cid = f"castor-{end}-{side}"
        CASTORS.append((cid, end, side, sz))
        bone(cid, f"leg-{end}", FOOT_Y0 - 0.10, 0.0, 180, 0.0475, {"type": "revolute", "continuousTwist": True}, t(f"{'Bass' if end == 'bass' else 'Treble'} {'right' if side == 'r' else 'left'} swivel castor fork about its kingpin", f"{'Bass' if end == 'bass' else 'Diskant'}-Lenkrolle {'rechts' if side == 'r' else 'links'}: Gabel um den Königszapfen"), z=sz * CASTOR_Z)
        bone(f"wheel-{end}-{side}", cid, 0.0475, -TRAIL, 90, WHEEL_R, {"type": "revolute"}, t(f"{'Bass' if end == 'bass' else 'Treble'} {'right' if side == 'r' else 'left'} rubber wheel on its axle", f"{'Bass' if end == 'bass' else 'Diskant'}-Gummirad {'rechts' if side == 'r' else 'links'} auf seiner Achse"))
        if end == "bass":
            bone(f"brake-{end}-{side}", cid, 0.015, 0.012, 70, 0.044, {"type": "revolute", "minDeg": 0, "maxDeg": 40}, t(f"Brake pedal of the bass {'right' if side == 'r' else 'left'} castor (rest: pressed, locked)", f"Bremspedal der Bass-Lenkrolle {'rechts' if side == 'r' else 'links'} (Ruhe: getreten, gesperrt)"))

MALLETS = {}
for mid, bi in MALLET_BARS.items():
    nb = NATURALS[bi]
    x = nb["x"]
    z_edge = -ROW_GAP / 2 - L(x)
    y_head = NAT_Y1 + MALLET_HEAD_R
    y_edge = NAT_Y1 + MALLET_SHAFT_R
    beta = math.degrees(math.atan2(y_head - y_edge, MALLET_HEAD_Z - z_edge))
    gz = MALLET_HEAD_Z - MALLET_HEAD_U * math.cos(math.radians(beta))
    gy = y_head - MALLET_HEAD_U * math.sin(math.radians(beta))
    MALLETS[mid] = {"bar": bi, "x": x, "beta": beta, "gy": gy, "gz": gz}
    hand = "left" if mid == "mallet-bass" else "right"
    hand_de = "linke" if mid == "mallet-bass" else "rechte"
    bone(f"grip-{mid[7:]}", "stand", x + LEG_X, gy - 0.10, 0, 0.03, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.2}, t(f"Grip of the {hand}-hand mallet, lifted by the player's hand (prismatic, vertical)", f"Griffpunkt des Schlägels der {hand_de}n Hand, von der Hand des Spielers gehoben (Schubgelenk, senkrecht)"), z=gz)
    bone(mid, f"grip-{mid[7:]}", 0.0, 0.0, 90, MALLET_LEN, {"type": "revolute", "minDeg": -25, "maxDeg": 30}, t(f"{'Left' if hand == 'left' else 'Right'}-hand xylophone mallet: rattan shaft and phenolic head, pitching about the wrist", f"Xylophonschlägel der {hand_de}n Hand: Rattanstiel und Phenolkopf, um das Handgelenk kippend"), abd=90 - beta, twist=90)
    fr = FRAMES[mid]
    head = [fr.o[i] + MALLET_HEAD_U * fr.r[i][0] for i in range(3)]
    assert abs(head[1] - y_head) < 1e-9 and abs(head[2] - MALLET_HEAD_Z) < 1e-9 and abs(head[0] - x) < 1e-9, head


def tone_wood(i):
    """🎨 Tone of a bar: rosewood dark with a little variation of the grain."""
    return round(0.30 + 0.04 * ((i * 7) % 5) / 4, 3)


class Part:
    """🧩 One physical part in world rest metres on a bone; it writes its solid and its three 2D shapes."""

    def __init__(self, id, bone, kind, geo, material, region, tone, role="body", views=("elevation", "front", "plan"), flags=None, pen=None, filled=True, solid=True, envelope=False, drawn=True, tags=None, bias=0.0):
        self.bias = bias
        self.id, self.bone, self.kind, self.geo = id, bone, kind, geo
        self.material, self.region, self.tone, self.role = material, region, tone, role
        self.views, self.flags, self.pen, self.filled = views, flags or {}, pen, filled
        self.solid, self.envelope, self.drawn, self.tags = solid, envelope, drawn, tags

    def frame(self):
        return FRAMES[self.bone]

    def bounds(self):
        g, k = self.geo, self.kind
        if k == "box":
            return g["x"], g["y"], g["z"]
        if k == "planprism":
            xs, zs = [p[0] for p in g["poly"]], [p[1] for p in g["poly"]]
            return (min(xs), max(xs)), g["y"], (min(zs), max(zs))
        if k == "frontprism":
            zs, ys = [p[0] for p in g["poly"]], [p[1] for p in g["poly"]]
            return g["x"], (min(ys), max(ys)), (min(zs), max(zs))
        if k == "elevprism":
            xs, ys = [p[0] for p in g["poly"]], [p[1] for p in g["poly"]]
            return (min(xs), max(xs)), (min(ys), max(ys)), g["z"]
        if k == "vcyl":
            return (g["cx"] - g["r"], g["cx"] + g["r"]), g["y"], (g["cz"] - g["r"], g["cz"] + g["r"])
        if k == "zcyl":
            return (g["cx"] - g["r"], g["cx"] + g["r"]), (g["cy"] - g["r"], g["cy"] + g["r"]), g["z"]
        raise ValueError(k)

    def volume(self):
        g, k = self.geo, self.kind
        if k == "box":
            return (g["x"][1] - g["x"][0]) * (g["y"][1] - g["y"][0]) * (g["z"][1] - g["z"][0])
        if k == "planprism":
            return area(g["poly"]) * (g["y"][1] - g["y"][0])
        if k == "frontprism":
            return area(g["poly"]) * (g["x"][1] - g["x"][0])
        if k == "elevprism":
            return area(g["poly"]) * (g["z"][1] - g["z"][0])
        if k == "vcyl":
            h = g["y"][1] - g["y"][0]
            if g.get("wall"):
                return math.pi * (g["r"] ** 2 - (g["r"] - g["wall"]) ** 2) * h + math.pi * (g["r"] - g["wall"]) ** 2 * g["cap"]
            return math.pi * g["r"] ** 2 * h
        if k == "zcyl":
            return math.pi * g["r"] ** 2 * (g["z"][1] - g["z"][0])
        raise ValueError(k)

    def centroid(self):
        (x0, x1), (y0, y1), (z0, z1) = self.bounds()
        return [(x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2]

    def solid_json(self):
        f = self.frame()
        g, k = self.geo, self.kind
        s = {"id": self.id, "bone": self.bone, "material": self.material, "role": self.role, "tone": self.tone}
        if k == "box":
            assert f.axis_aligned(), self.id
            c = f.local([(g["x"][0] + g["x"][1]) / 2, (g["y"][0] + g["y"][1]) / 2, (g["z"][0] + g["z"][1]) / 2])
            size = [abs(v) for v in f.local_dir([g["x"][1] - g["x"][0], g["y"][1] - g["y"][0], g["z"][1] - g["z"][0]])]
            s.update({"kind": "box", "size": [r6(v) for v in size], "transform": {"translation": [r6(v) for v in c]}})
        elif k == "planprism":
            assert f.r == [[1, 0, 0], [0, 1, 0], [0, 0, 1]] or all(abs(f.r[i][j] - (i == j)) < 1e-12 for i in range(3) for j in range(3)), self.id
            poly = ccw([(p[1] - f.o[2], p[0] - f.o[0]) for p in g["poly"]])
            s.update({"kind": "extrude", "plane": "zx", "profile": [[r6(a), r6(b)] for a, b in poly], "from": r6(g["y"][0] - f.o[1]), "to": r6(g["y"][1] - f.o[1])})
        elif k == "frontprism":
            assert all(abs(f.r[i][j] - (i == j)) < 1e-12 for i in range(3) for j in range(3)), self.id
            poly = ccw([(p[1] - f.o[1], p[0] - f.o[2]) for p in g["poly"]])
            s.update({"kind": "extrude", "plane": "yz", "profile": [[r6(a), r6(b)] for a, b in poly], "from": r6(g["x"][0] - f.o[0]), "to": r6(g["x"][1] - f.o[0])})
        elif k == "elevprism":
            assert abs(f.r[2][2] - 1) < 1e-9, self.id
            poly = ccw([tuple(f.local([p[0], p[1], 0])[:2]) for p in g["poly"]])
            s.update({"kind": "extrude", "plane": "xy", "profile": [[r6(a), r6(b)] for a, b in poly], "from": r6(g["z"][0] - f.o[2]), "to": r6(g["z"][1] - f.o[2])})
        elif k == "vcyl":
            axis = f.local_dir([0, 1, 0])
            origin = f.local([g["cx"], g["y"][0], g["cz"]])
            h = g["y"][1] - g["y"][0]
            r = g["r"]
            if g.get("wall"):
                ri = r - g["wall"]
                prof = [[0, 0], [0, r], [h, r], [h, ri], [g["cap"], ri], [g["cap"], 0]]
            else:
                prof = [[0, 0], [0, r], [h, r], [h, 0]]
            s.update({"kind": "revolve", "profile": [[r6(a), r6(b)] for a, b in prof], "axis": [round(v) for v in axis], "origin": [r6(v) for v in origin]})
        elif k == "zcyl":
            assert abs(f.r[2][2] - 1) < 1e-9, self.id
            n = 32
            poly = [tuple(f.local([g["cx"] + g["r"] * math.cos(2 * math.pi * j / n), g["cy"] + g["r"] * math.sin(2 * math.pi * j / n), 0])[:2]) for j in range(n)]
            s.update({"kind": "extrude", "plane": "xy", "profile": [[r6(a), r6(b)] for a, b in ccw(poly)], "from": r6(g["z"][0] - f.o[2]), "to": r6(g["z"][1] - f.o[2])})
        if self.envelope:
            s["envelope"] = True
        s.update(self.flags)
        if self.tags:
            s["tags"] = self.tags
        return s

    def outline(self, view):
        """✏️ The 2D outline of the part in a view: elevation bone-local (u, v), front world (z, y), plan world (x, z)."""
        f = self.frame()
        g, k = self.geo, self.kind
        (x0, x1), (y0, y1), (z0, z1) = self.bounds()
        if view == "elevation":
            if k == "elevprism":
                return "poly", [tuple(f.local([p[0], p[1], 0])[:2]) for p in g["poly"]]
            if k == "zcyl":
                c = f.local([g["cx"], g["cy"], 0])
                return "ellipse", (c[0], c[1], g["r"])
            return "poly", [tuple(f.local([x, y, 0])[:2]) for x, y in ((x0, y0), (x1, y0), (x1, y1), (x0, y1))]
        if view == "front":
            if k == "frontprism":
                return "poly", list(g["poly"])
            return "poly", [(z0, y0), (z1, y0), (z1, y1), (z0, y1)]
        if k == "planprism":
            return "poly", list(g["poly"])
        if k == "vcyl":
            return "ellipse", (g["cx"], g["cz"], g["r"])
        return "poly", [(x0, z0), (x1, z0), (x1, z1), (x0, z1)]

    def depth(self, view):
        (x0, x1), (y0, y1), (z0, z1) = self.bounds()
        if view == "elevation":
            return round(5 - (z0 + z1) / 2 + self.bias, 4)
        if view == "front":
            return round(5 + (x0 + x1) / 2 + self.bias, 4)
        return round(5 + y1 + self.bias, 4)

    def shapes(self, scale):
        out = []
        if not self.drawn:
            return out
        for view in self.views:
            kind, geo = self.outline(view)
            s = {"id": self.id, "bone": self.bone, "role": self.role, "z": self.depth(view), "filled": self.filled, "tone": self.tone, "region": self.region, "material": self.material}
            if self.pen is not None:
                s["penMm"] = self.pen
            if kind == "poly":
                s.update({"kind": "poly", "points": [[r6(a), r6(b)] for a, b in geo], "closed": True})
            else:
                s.update({"kind": "ellipse", "cx": r6(geo[0]), "cy": r6(geo[1]), "rx": r6(geo[2]), "ry": r6(geo[2])})
            out.append(s)
        return out


def area(poly):
    """📐 Signed area of a polygon (shoelace), positive counter-clockwise."""
    return abs(signed(poly))


def signed(poly):
    """➰ Signed shoelace area."""
    return sum(poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1] for i in range(len(poly))) / 2


def ccw(poly):
    """🔁 The polygon in counter-clockwise order."""
    return poly if signed(poly) > 0 else list(reversed(poly))


def strip_plan(x0, x1, zf, half):
    """🛤️ A plan parallelogram of half width `half` about the line z = zf(x) from x0 to x1."""
    return [(x0, zf(x0) - half), (x1, zf(x1) - half), (x1, zf(x1) + half), (x0, zf(x0) + half)]


def bar_profile(row, x, chip, detail):
    """🪵 Front profile (z, y) of one bar: the arch undercut between the nodes and an optional chip at the struck end."""
    l = L(x)
    if row == "nat":
        zi, zo, y0, y1, sign = -ROW_GAP / 2, -ROW_GAP / 2 - l, NAT_Y0, NAT_Y1, -1
    else:
        zi, zo, y0, y1, sign = ROW_GAP / 2, ROW_GAP / 2 + l, ACC_Y0, ACC_Y1, 1
    if not detail:
        return [(zi, y0), (zo, y0), (zo, y1), (zi, y1)]
    depth = 0.008 * l / 0.38
    arch = []
    for k in range(7):
        s = 0.28 + 0.44 * k / 6
        arch.append((zi + sign * s * l, y0 + depth * math.sin(math.pi * k / 6) ** 0.7))
    top = []
    if chip and row == "nat":
        top = [(zo, y1 - 0.006), (zo - sign * chip, y1)]
        pts = [(zi, y0)] + arch + [(zo, y0)] + top + [(zi, y1)]
    elif chip:
        pts = [(zi, y0)] + arch + [(zo, y0), (zo, y1), (zi + sign * chip, y1), (zi, y1 - 0.006)]
    else:
        pts = [(zi, y0)] + arch + [(zo, y0), (zo, y1), (zi, y1)]
    return pts


def parts_keyboard(scale):
    """🎹 Bars, rails, cheeks of one scale."""
    P = []
    pen_b = {50: 0.25, 100: 0.18, 200: 0.2}[scale]
    if scale == 50:
        for nb in NATURALS:
            x = nb["x"]
            chip = CHIPPED_NAT.get(nb["i"])
            mat = "padauk" if nb["i"] == REPLACED_NAT else "rosewood"
            tone = 0.48 if nb["i"] == REPLACED_NAT else tone_wood(nb["i"])
            P.append(Part(f"bar-{nb['name'].lower()}", "naturals", "frontprism", {"poly": bar_profile("nat", x, chip, True), "x": (x - BAR_W / 2, x + BAR_W / 2)}, mat, "bars", tone, pen=pen_b))
        for ab in ACCIDENTALS:
            x = ab["x"]
            chip = CHIPPED_ACC.get(ab["i"])
            P.append(Part(f"bar-{ab['name'].lower()}", "accidentals", "frontprism", {"poly": bar_profile("acc", x, chip, True), "x": (x - BAR_W / 2, x + BAR_W / 2)}, "rosewood", "bars", round(tone_wood(ab["i"] + 2) - 0.04, 3), pen=pen_b))
    else:
        xa, xb = NATURALS[0]["x"] - BAR_W / 2, NATURALS[-1]["x"] + BAR_W / 2
        P.append(Part("naturals-row", "naturals", "planprism", {"poly": [(xa, -ROW_GAP / 2 - L(xa)), (xb, -ROW_GAP / 2 - L(xb)), (xb, -ROW_GAP / 2), (xa, -ROW_GAP / 2)], "y": (NAT_Y0, NAT_Y1)}, "rosewood", "bars", 0.32, pen=pen_b))
        groups, cur = [], [ACCIDENTALS[0]]
        for ab in ACCIDENTALS[1:]:
            if abs(ab["x"] - cur[-1]["x"] - PITCH) < 1e-9:
                cur.append(ab)
            else:
                groups.append(cur)
                cur = [ab]
        groups.append(cur)
        if scale == 200:
            groups = [ACCIDENTALS]
        for g in groups:
            xa, xb = g[0]["x"] - BAR_W / 2, g[-1]["x"] + BAR_W / 2
            gid = "accidentals-row" if len(g) == len(ACCIDENTALS) else f"accidentals-{g[0]['name'].lower()}-{g[-1]['name'].lower()}"
            P.append(Part(gid, "accidentals", "planprism", {"poly": [(xa, ROW_GAP / 2), (xb, ROW_GAP / 2), (xb, ROW_GAP / 2 + L(xb)), (xa, ROW_GAP / 2 + L(xa))], "y": (ACC_Y0, ACC_Y1)}, "rosewood", "bars", 0.28, pen=pen_b))
    if True:
        rails = [("rail-naturals-outer", -1, 0.776, (0.804, NAT_Y0)), ("rail-naturals-inner", -1, 0.224, (0.804, NAT_Y0)), ("rail-accidentals-inner", 1, 0.224, (0.824, ACC_Y0)), ("rail-accidentals-outer", 1, 0.776, (0.824, ACC_Y0))]
        for n, (rid, sg, frac, ys) in enumerate(rails):
            zf = (lambda s, fr: (lambda x: s * (ROW_GAP / 2 + fr * L(x))))(sg, frac)
            P.append(Part(rid, "keyboard", "planprism", {"poly": strip_plan(-CHEEK_X0, CHEEK_X0, zf, 0.015), "y": ys}, "beech", "keyboard-frame", 0.62, pen={50: 0.25, 100: 0.18, 200: 0.18}[scale], bias=0.001 * (n + 1)))
        for end, sx in (("bass", -1), ("treble", 1)):
            half = {"bass": 0.345, "treble": 0.19}[end]
            if scale == 50:
                poly = [(-half, CHEEK_Y0), (half, CHEEK_Y0), (half, CHEEK_Y1 - 0.012), (half - 0.012, CHEEK_Y1), (-half + 0.012, CHEEK_Y1), (-half, CHEEK_Y1 - 0.012)]
            else:
                poly = [(-half, CHEEK_Y0), (half, CHEEK_Y0), (half, CHEEK_Y1), (-half, CHEEK_Y1)]
            xr = (sx * CHEEK_X0, sx * CHEEK_X1) if sx > 0 else (-CHEEK_X1, -CHEEK_X0)
            P.append(Part(f"cheek-{end}", "keyboard", "frontprism", {"poly": poly, "x": xr}, "beech", "keyboard-frame", 0.56, pen={50: 0.35, 100: 0.25, 200: 0.18}[scale]))
    return P


def parts_deck500():
    """🧱 At 1:500 the four rails and the two cheeks are one frame deck under the bars, the naturals one 22 mm slab and the accidentals one slab down to the rail felts (the E-F and B-C gaps of 64 mm are below the print gap there)."""
    P = []
    xa, xb = -CHEEK_X1, CHEEK_X1
    half = lambda x: 0.345 + (0.19 - 0.345) * (x - xa) / (xb - xa)
    P.append(Part("frame-deck", "keyboard", "planprism", {"poly": [(xa, -half(xa)), (xb, -half(xb)), (xb, half(xb)), (xa, half(xa))], "y": (CHEEK_Y0, NAT_Y0)}, "beech", "keyboard-frame", 0.56, pen=0.2))
    xa, xb = NATURALS[0]["x"] - BAR_W / 2, NATURALS[-1]["x"] + BAR_W / 2
    P.append(Part("naturals-row", "naturals", "planprism", {"poly": [(xa, -ROW_GAP / 2 - L(xa)), (xb, -ROW_GAP / 2 - L(xb)), (xb, -ROW_GAP / 2), (xa, -ROW_GAP / 2)], "y": (NAT_Y0, NAT_Y1)}, "rosewood", "bars", 0.32, pen=0.2))
    xa, xb = ACCIDENTALS[0]["x"] - BAR_W / 2, ACCIDENTALS[-1]["x"] + BAR_W / 2
    P.append(Part("accidentals-row", "accidentals", "planprism", {"poly": [(xa, ROW_GAP / 2), (xb, ROW_GAP / 2), (xb, ROW_GAP / 2 + L(xb)), (xa, ROW_GAP / 2 + L(xa))], "y": (NAT_Y0, ACC_Y1)}, "rosewood", "bars", 0.28, pen=0.2))
    return P


def parts_resonators(scale):
    """🎺 Tubes and bank bars (1:50), stepped banks (1:100), sloped banks (1:200, 1:500)."""
    P = []
    banks = (("naturals", NATURALS, zc_nat, NAT_TUBE_TOP, NAT_BANK_Y), ("accidentals", ACCIDENTALS, zc_acc, ACC_TUBE_TOP, ACC_BANK_Y))
    for row, notes, zc, top, bank_y in banks:
        bid = f"resonators-{row}"
        if scale == 50:
            for nb in notes:
                lr = resonator_length(nb["semi"])
                P.append(Part(f"tube-{nb['name'].lower()}", bid, "vcyl", {"cx": nb["x"], "cz": zc(nb["x"]), "y": (top - lr, top), "r": TUBE_R, "wall": TUBE_WALL, "cap": TUBE_CAP}, "alu-lacquer", "resonators", 0.74 if row == "naturals" else 0.68, pen=0.18))
            for side, sg in (("outer", -1 if row == "naturals" else 1), ("inner", 1 if row == "naturals" else -1)):
                zf = (lambda zz, s: (lambda x: zz(x) + s * (TUBE_R + 0.004)))(zc, sg)
                P.append(Part(f"bank-bar-{row}-{side}", bid, "planprism", {"poly": strip_plan(-CHEEK_X0, CHEEK_X0, zf, 0.004), "y": bank_y}, "alu-lacquer", "resonators", 0.8, pen=0.18))
        else:
            half = {100: TUBE_R + 0.008, 200: TUBE_R + 0.008, 500: TUBE_R + 0.008}[scale]
            xs = [nb["x"] for nb in notes]
            lengths = [resonator_length(nb["semi"]) for nb in notes]
            flat = [k for k, l in enumerate(lengths) if l <= 0.1 + 1e-9][0]
            if scale == 100:
                for k in range(flat):
                    x0 = -CHEEK_X0 if k == 0 else (xs[k - 1] + xs[k]) / 2
                    x1 = (xs[k] + xs[k + 1]) / 2
                    P.append(Part(f"bank-{row}-step-{notes[k]['name'].lower()}", bid, "planprism", {"poly": strip_plan(x0, x1, zc, half), "y": (top - lengths[k], top)}, "alu-lacquer", "resonators", 0.72, pen=0.18, envelope=True))
                x0 = (xs[flat - 1] + xs[flat]) / 2
                P.append(Part(f"bank-{row}-upper", bid, "planprism", {"poly": strip_plan(x0, CHEEK_X0, zc, half), "y": (top - 0.10, top)}, "alu-lacquer", "resonators", 0.72, pen=0.18, envelope=True))
            else:
                xm = (xs[flat - 1] + xs[flat]) / 2
                xa = -CHEEK_X0
                steps = 2 if scale == 200 else 1
                xk = [xa + (xm - xa) * j / steps for j in range(steps + 1)]
                for j in range(steps):
                    xc = (xk[j] + xk[j + 1]) / 2
                    lm = max(lengths[k] for k in range(len(xs)) if xk[j] - 1e-9 <= xs[k] <= xk[j + 1] + PITCH / 2) if scale == 500 else sum(lengths[k] for k in range(len(xs)) if xk[j] <= xs[k] < xk[j + 1]) / max(1, len([1 for k in range(len(xs)) if xk[j] <= xs[k] < xk[j + 1]]))
                    P.append(Part(f"bank-{row}-low-{j + 1}", bid, "planprism", {"poly": strip_plan(xk[j], xk[j + 1], zc, half), "y": (top - lm, top)}, "alu-lacquer", "resonators", 0.72, pen=0.18, envelope=True))
                P.append(Part(f"bank-{row}-upper", bid, "planprism", {"poly": strip_plan(xm, CHEEK_X0, zc, half), "y": (top - 0.10, top)}, "alu-lacquer", "resonators", 0.72, pen=0.18, envelope=True))
    return P


def parts_stand(scale):
    """🦿 Stretcher, end frames, cross tubes."""
    P = []
    slender = {"slender": True} if scale in (200, 500) else {}
    pen = {50: 0.35, 100: 0.25, 200: 0.18, 500: 0.18}[scale]
    P.append(Part("stretcher", "stand", "box", {"x": (-LEG_X + 0.015, LEG_X - 0.015), "y": (FOOT_Y0, FOOT_Y1), "z": (-0.015, 0.015)}, "steel-powder", "stand", 0.2, pen=pen, flags=slender))
    for end, sx in (("bass", -1), ("treble", 1)):
        lb = f"leg-{end}"
        xl = sx * LEG_X
        P.append(Part(f"foot-bar-{end}", lb, "box", {"x": (xl - 0.015, xl + 0.015), "y": (FOOT_Y0, FOOT_Y1), "z": (-FOOT_Z, FOOT_Z)}, "steel-powder", "stand", 0.2, pen=pen, flags=slender))
        zl = LEG_Z[end]
        for side, sz in (("r", -1), ("l", 1)):
            P.append(Part(f"leg-tube-{end}-{side}", lb, "vcyl", {"cx": xl, "cz": sz * zl, "y": (FOOT_Y1, CHEEK_Y0), "r": LEG_R}, "steel-powder", "stand", 0.2, pen=pen, flags={"slender": True} if scale in (100, 200, 500) else {}))
        if scale in (50, 100) or (scale == 200 and end == "bass"):
            P.append(Part(f"cross-tube-{end}", lb, "zcyl", {"cx": xl, "cy": CROSS_Y, "r": CROSS_R, "z": (-zl, zl)}, "steel-powder", "stand", 0.24, pen=pen, flags={"slender": True} if scale == 200 or (scale == 100 and end == "bass") else {}))
    return P


def parts_castors(scale):
    """🛞 The four swivel castors."""
    P = []
    for cid, end, side, sz in CASTORS:
        xl = (-1 if end == "bass" else 1) * LEG_X
        zc = sz * CASTOR_Z
        wid = f"wheel-{end}-{side}"
        if scale == 50:
            P.append(Part(f"housing-{end}-{side}", cid, "vcyl", {"cx": xl, "cz": zc, "y": (0.075, FOOT_Y0), "r": 0.025}, "steel-zinc", "castors", 0.7, pen=0.18))
            P.append(Part(f"crown-{end}-{side}", cid, "box", {"x": (xl - 0.050, xl + 0.012), "y": (0.065, 0.075), "z": (zc - 0.021, zc + 0.021)}, "steel-zinc", "castors", 0.66, pen=0.18))
            for s2, zz in (("o", (zc + 0.013, zc + 0.021)), ("i", (zc - 0.021, zc - 0.013))):
                P.append(Part(f"fork-{end}-{side}-{s2}", cid, "box", {"x": (xl - 0.045, xl - 0.015), "y": (0.028, 0.065), "z": zz}, "steel-zinc", "castors", 0.62, pen=0.18))
            P.append(Part(f"tyre-{end}-{side}", wid, "zcyl", {"cx": xl - TRAIL, "cy": WHEEL_R, "r": WHEEL_R, "z": (zc - WHEEL_HW, zc + WHEEL_HW)}, "rubber", "castors", 0.12, pen=0.25))
            P.append(Part(f"hub-{end}-{side}", wid, "zcyl", {"cx": xl - TRAIL, "cy": WHEEL_R, "r": 0.016, "z": (zc - 0.013, zc + 0.013)}, "steel-zinc", "castors", 0.78, role="detail", pen=0.13))
        elif scale == 100:
            P.append(Part(f"fork-{end}-{side}", cid, "box", {"x": (xl - 0.050, xl + 0.012), "y": (0.028, FOOT_Y0), "z": (zc - 0.021, zc + 0.021)}, "steel-zinc", "castors", 0.64, pen=0.18))
            P.append(Part(f"tyre-{end}-{side}", wid, "zcyl", {"cx": xl - TRAIL, "cy": WHEEL_R, "r": WHEEL_R, "z": (zc - WHEEL_HW, zc + WHEEL_HW)}, "rubber", "castors", 0.12, pen=0.18))
        else:
            P.append(Part(f"fork-{end}-{side}", cid, "box", {"x": (xl - 0.050, xl + 0.012), "y": (WHEEL_R, FOOT_Y0), "z": (zc - 0.021, zc + 0.021)}, "steel-zinc", "castors", 0.6, pen=0.18))
            P.append(Part(f"wheel-{end}-{side}", wid, "zcyl", {"cx": xl - TRAIL, "cy": WHEEL_R, "r": WHEEL_R, "z": (zc - 0.017, zc + 0.017)}, "rubber", "castors", 0.14, pen=0.18))
    return P


def pedal_parts(scale):
    """🦶 The two brake pedals of the bass castors (1:50 and 1:100)."""
    P = []
    if scale not in (50, 100):
        return P
    for cid, end, side, sz in CASTORS:
        if end != "bass":
            continue
        pid = f"brake-{end}-{side}"
        f = FRAMES[pid]
        P.append(("pedal", pid, f, sz * CASTOR_Z))
    return P


SHAPES_EXTRA = {50: [], 100: [], 200: [], 500: []}


def build_scale(scale):
    """🏗️ Parts, solids and shapes of one scale."""
    parts = parts_deck500() if scale == 500 else parts_keyboard(scale)
    parts += parts_resonators(scale)
    parts += parts_stand(scale)
    parts += parts_castors(scale)
    return parts


def mallet_profile(scale):
    """🥢 Half profile (u, v ≥ 0) of a mallet: shaft, worn tape wrap at the grip, head."""
    pts = [(0.0, 0.0)]
    if scale == 50:
        pts += [(0.0, 0.0055), (0.10, 0.0055), (0.10, MALLET_SHAFT_R)]
    else:
        pts += [(0.0, MALLET_SHAFT_R)]
    if scale == 500:
        pts += [(MALLET_LEN, MALLET_SHAFT_R), (MALLET_LEN, 0.0)]
        return pts
    a0 = math.degrees(math.asin(MALLET_SHAFT_R / MALLET_HEAD_R))
    pts.append((MALLET_HEAD_U - MALLET_HEAD_R * math.cos(math.radians(a0)), MALLET_SHAFT_R))
    for k in range(1, 9):
        a = math.radians(180 - a0 - (180 - a0) * k / 8)
        pts.append((MALLET_HEAD_U + MALLET_HEAD_R * math.cos(a), MALLET_HEAD_R * math.sin(a)))
    pts[-1] = (MALLET_LEN, 0.0)
    return pts


def mallet_solids(scale, mid):
    """🥢 Solids of one mallet in its own frame."""
    out = []
    shaft_end = MALLET_HEAD_U if scale == 500 else MALLET_SHAFT_END + 0.004
    sh = {"id": f"{mid}-shaft", "kind": "capsule", "bone": mid, "material": "rattan", "role": "body", "tone": 0.66, "p0": [0.0, 0.0, 0.0], "p1": [r6(shaft_end if scale != 500 else MALLET_LEN - MALLET_SHAFT_R), 0.0, 0.0], "r0": MALLET_SHAFT_R, "r1": MALLET_SHAFT_R}
    if scale != 50:
        sh["slender"] = True
    sh["p0"] = [MALLET_SHAFT_R, 0.0, 0.0]
    out.append(sh)
    if scale == 50:
        out.append({"id": f"{mid}-wrap", "kind": "capsule", "bone": mid, "material": "tape", "role": "body", "tone": 0.45, "p0": [0.0055, 0.0, 0.0], "p1": [0.0945, 0.0, 0.0], "r0": 0.0055, "r1": 0.0055})
    if scale != 500:
        out.append({"id": f"{mid}-head", "kind": "ellipsoid", "bone": mid, "material": "phenolic", "role": "body", "tone": 0.22, "radii": [MALLET_HEAD_R] * 3, "transform": {"translation": [MALLET_HEAD_U, 0.0, 0.0]}})
    return out


def mallet_shapes(scale, mid):
    """🥢 The mallet drawn as a revolved solid in all three views."""
    prof = mallet_profile(scale)
    out = []
    for view in ("elevation", "front", "plan"):
        out.append({"id": f"{mid}", "bone": mid, "role": "body", "z": 9.0, "filled": scale != 500, "tone": 0.3, "region": "mallets", "material": "phenolic" if scale != 500 else "rattan", "penMm": {50: 0.18, 100: 0.13, 200: 0.13, 500: 0.13}[scale], "kind": "poly", "points": [[r6(a), r6(b)] for a, b in prof], "closed": True, "solid": {"kind": "revolved", "axisBone": mid, "axis": "u", "axisX": 0, "mirror": True}})
    return out


def pedal_geometry(pid):
    """🦶 Pedal plate in its own frame: 44 mm long, 8 mm thick, 24 mm wide."""
    return (0.0, 0.044), (-0.004, 0.004), (-0.012, 0.012)


def pedal_solid(pid, zc):
    u, v, w = pedal_geometry(pid)
    return {"id": f"pedal-{pid[6:]}", "kind": "box", "bone": pid, "material": "steel-zinc", "role": "body", "tone": 0.5, "size": [r6(u[1] - u[0]), r6(v[1] - v[0]), r6(w[1] - w[0])], "transform": {"translation": [r6((u[0] + u[1]) / 2), 0.0, 0.0]}}


def pedal_shapes(pid, zc, pen):
    u, v, w = pedal_geometry(pid)
    prof = [[u[0], v[0]], [u[1], v[0]], [u[1], v[1]], [u[0], v[1]]]
    out = [{"id": f"pedal-{pid[6:]}", "bone": pid, "role": "body", "z": round(5 - zc, 4), "filled": True, "tone": 0.5, "region": "castors", "material": "steel-zinc", "penMm": pen, "kind": "poly", "points": prof, "closed": True}]
    for view in ("front", "plan"):
        out.append({"id": f"pedal-{pid[6:]}", "bone": pid, "role": "body", "z": 5.2 if view == "front" else 5.08, "filled": True, "tone": 0.5, "region": "castors", "material": "steel-zinc", "penMm": pen, "kind": "poly", "points": prof, "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": r6(zc - 0.012), "z1": r6(zc + 0.012)}})
    return out


def wheel_shapes_solidify(shapes, parts):
    """🛞 Front and plan shapes of wheel parts become lateral extrusions so they read right while the wheel spins."""
    wheel_parts = {p.id: p for p in parts if p.bone.startswith("wheel-")}
    out = []
    for s in shapes:
        p = wheel_parts.get(s["id"])
        if p is not None and s.get("kind") == "poly" and s.get("_view") in ("front", "plan"):
            f = p.frame()
            n = 32
            prof = [[r6(f.local([p.geo["cx"] + p.geo["r"] * math.cos(2 * math.pi * j / n), p.geo["cy"] + p.geo["r"] * math.sin(2 * math.pi * j / n), 0])[0]), r6(f.local([p.geo["cx"] + p.geo["r"] * math.cos(2 * math.pi * j / n), p.geo["cy"] + p.geo["r"] * math.sin(2 * math.pi * j / n), 0])[1])] for j in range(n)]
            s = dict(s)
            s["points"] = prof
            s["solid"] = {"kind": "extruded", "sweep": "w", "z0": r6(p.geo["z"][0]), "z1": r6(p.geo["z"][1])}
        s.pop("_view", None)
        out.append(s)
    return out


def details_2d(scale):
    """🖊️ The traces of use and fine details drawn as strokes: felt, worn playing spots, a dented tube, the inventory label, scratches."""
    el, fr, pl = [], [], []
    kb = FRAMES["keyboard"]
    if scale == 50:
        for rid, y in (("felt-naturals", NAT_Y0 - 0.002), ("felt-accidentals", ACC_Y0 - 0.002)):
            el.append({"id": rid, "bone": "keyboard", "role": "detail", "z": 5.6, "filled": False, "tone": 0.4, "region": "keyboard-frame", "kind": "poly", "points": [[r6(-CHEEK_X0 - kb.o[0]), r6(y - kb.o[1])], [r6(CHEEK_X0 - kb.o[0]), r6(y - kb.o[1])]], "closed": False, "penMm": 0.13})
        nat = FRAMES["naturals"]
        for nb in NATURALS:
            if nb["i"] % 7 in (0, 2, 4) or nb["i"] in (3, 9, 11):
                rx = 0.012
                ry = 0.03 + 0.01 * ((nb["i"] * 3) % 4) / 3
                pl.append({"id": f"wear-{nb['name'].lower()}", "bone": "naturals", "role": "texture", "z": 6.0, "filled": False, "tone": 0.55, "region": "bars", "kind": "ellipse", "cx": r6(nb["x"]), "cy": r6(zc_nat(nb["x"]) + 0.02), "rx": rx, "ry": r6(ry), "penMm": 0.1})
        for ab in ACCIDENTALS:
            if ab["i"] % 7 in (0, 1, 4):
                pl.append({"id": f"wear-{ab['name'].lower()}", "bone": "accidentals", "role": "texture", "z": 6.1, "filled": False, "tone": 0.55, "region": "bars", "kind": "ellipse", "cx": r6(ab["x"]), "cy": r6(ROW_GAP / 2 + 0.05), "rx": 0.012, "ry": 0.025, "penMm": 0.1})
        rn = FRAMES["resonators-naturals"]
        nb = NATURALS[2]
        lr = resonator_length(nb["semi"])
        cy = NAT_TUBE_TOP - 0.55 * lr
        el.append({"id": "dent-tube-a3", "bone": "resonators-naturals", "role": "detail", "z": 8.0, "filled": False, "tone": 0.4, "region": "resonators", "kind": "poly", "points": [[r6(nb["x"] - 0.012 - rn.o[0]), r6(cy + 0.012 - rn.o[1])], [r6(nb["x"] - 0.004 - rn.o[0]), r6(cy - rn.o[1])], [r6(nb["x"] - 0.010 - rn.o[0]), r6(cy - 0.014 - rn.o[1])]], "closed": False, "penMm": 0.13})
        fr.append({"id": "inventory-label", "bone": "keyboard", "role": "detail", "z": 6.5, "filled": True, "tone": 0.92, "region": "keyboard-frame", "kind": "poly", "points": [[-0.06, 0.792], [0.0, 0.792], [0.0, 0.832], [-0.06, 0.832]], "closed": True, "penMm": 0.13})
        fr.append({"id": "inventory-label-text", "bone": "keyboard", "role": "texture", "z": 6.6, "filled": False, "tone": 0.3, "region": "keyboard-frame", "kind": "poly", "points": [[-0.052, 0.812], [-0.008, 0.812]], "closed": False, "penMm": 0.1})
        for end, sx in (("bass", -1), ("treble", 1)):
            lb = FRAMES[f"leg-{end}"]
            for k, (y0, y1) in enumerate(((0.30, 0.36), (0.60, 0.64))):
                fr.append({"id": f"scuff-{end}-{k + 1}", "bone": f"leg-{end}", "role": "texture", "z": 5.0 + sx * LEG_X + 0.02, "filled": False, "tone": 0.6, "region": "stand", "kind": "poly", "points": [[r6(-LEG_Z[end] - 0.006 + 0.004 * k), y0], [r6(-LEG_Z[end] + 0.006 - 0.004 * k), y1]], "closed": False, "penMm": 0.1})
    if scale == 100:
        nat = FRAMES["naturals"]
        for nb in NATURALS[:-1]:
            xg = nb["x"] + PITCH / 2
            pl.append({"id": f"gap-{nb['name'].lower()}", "bone": "naturals", "role": "texture", "z": 6.0, "filled": False, "tone": 0.4, "region": "bars", "kind": "poly", "points": [[r6(xg), r6(-ROW_GAP / 2 - L(xg))], [r6(xg), r6(-ROW_GAP / 2)]], "closed": False, "penMm": 0.1})
            el.append({"id": f"gap-{nb['name'].lower()}", "bone": "naturals", "role": "texture", "z": 6.0, "filled": False, "tone": 0.4, "region": "bars", "kind": "poly", "points": [[r6(xg - nat.o[0]), r6(NAT_Y0 - nat.o[1])], [r6(xg - nat.o[0]), r6(NAT_Y1 - nat.o[1])]], "closed": False, "penMm": 0.1})
    return el, fr, pl


def representation(scale):
    """🖨️ The authored representation of one scale."""
    parts = build_scale(scale)
    geo = {"elevation": [], "front": [], "plan": []}
    for p in parts:
        for v, s in zip(p.views, p.shapes(scale)):
            s["_view"] = v
            geo[v].append(s)
    for v in geo:
        geo[v] = wheel_shapes_solidify(geo[v], parts)
    for mid in MALLETS:
        for v, s in zip(("elevation", "front", "plan"), mallet_shapes(scale, mid)):
            geo[v].append(s)
    for _, pid, f, zc in pedal_parts(scale):
        for v, s in zip(("elevation", "front", "plan"), pedal_shapes(pid, zc, 0.18)):
            geo[v].append(s)
    el, fr, pl = details_2d(scale)
    geo["elevation"] += el
    geo["front"] += fr
    geo["plan"] += pl
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}[scale]
    if scale == 500:
        for v in geo:
            for s in geo[v]:
                if s["role"] == "detail":
                    s["role"] = "body"
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}[scale]
    rep = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": [], "geometry": geo, "decisions": DECISIONS_2D[scale]}
    return rep, parts


def solids_of(scale, parts):
    """🧊 Body, thermal and structure lists of one scale."""
    body = [p.solid_json() for p in parts if p.solid]
    for mid in MALLETS:
        body += mallet_solids(scale, mid)
    for _, pid, f, zc in pedal_parts(scale):
        body.append(pedal_solid(pid, zc))
    region_of = {p.id: p.region for p in parts}
    for s in body:
        if s["id"].startswith("mallet-"):
            region_of[s["id"]] = "mallets"
        if s["id"].startswith("pedal-"):
            region_of[s["id"]] = "castors"
    thermal = [{"id": f"th-{s['id']}", "of": s["id"], "region": region_of[s["id"]]} for s in body]
    structure = []
    member_of = {}
    for s in body:
        b = s["bone"]
        if s["id"].startswith("tube-") or (s["id"].startswith("bank-") and "bar" not in s["id"]):
            continue
        if s["id"].endswith(("-head", "-wrap")):
            continue
        member_of[s["id"]] = b
    for sid, b in member_of.items():
        structure.append({"id": f"st-{sid}", "of": sid, "member": b, "part": "member"})
    feat = FEATURE[scale]
    for row in ("resonators-naturals", "resonators-accidentals"):
        if not any(m == row for m in member_of.values()):
            bf = FRAMES[row]
            y0 = (NAT_BANK_Y if "naturals" in row else ACC_BANK_Y)
            zf = zc_nat if "naturals" in row else zc_acc
            structure.append({"id": f"st-{row}-hanger", "kind": "extrude", "bone": row, "material": "alu-lacquer", "role": "detail", "tone": 0.5, "plane": "zx", "profile": [[r6(a), r6(b)] for a, b in ccw([(p[1] - bf.o[2], p[0] - bf.o[0]) for p in strip_plan(-CHEEK_X0, CHEEK_X0, zf, 0.004)])], "from": 0.0, "to": r6(0.03), "slender": True, "member": row, "part": "member"})
    for cid, end, side, sz in CASTORS:
        jr = max(0.006, feat / 2)
        if scale in (50, 100):
            structure.append({"id": f"joint-kingpin-{end}-{side}", "kind": "ellipsoid", "bone": cid, "material": "steel-zinc", "role": "detail", "tone": 0.5, "radii": [jr, jr, jr], "transform": {"translation": [r6(jr), 0.0, 0.0]}, "member": cid, "part": "joint"})
            wf = FRAMES[f"wheel-{end}-{side}"]
            structure.append({"id": f"joint-axle-{end}-{side}", "kind": "capsule", "bone": f"wheel-{end}-{side}", "material": "steel-zinc", "role": "detail", "tone": 0.5, "p0": [0.0, 0.0, -0.021], "p1": [0.0, 0.0, 0.021], "r0": 0.004 if scale == 50 else 0.0075, "r1": 0.004 if scale == 50 else 0.0075, "member": cid, "part": "joint"})
        ss = max(0.0075, feat)
        structure.append({"id": f"support-{end}-{side}", "kind": "box", "bone": cid, "material": "rubber", "role": "ground", "tone": 0.3, "size": [r6(ss), r6(0.02), r6(0.024)], "transform": {"translation": [r6(0.085 - ss / 2), r6(-TRAIL), 0.0]}, "member": cid, "part": "support", "anchor": f"floor-{end}-{side}"})
    for mid in MALLETS:
        jr = max(0.006, feat / 2) if scale in (50, 100) else None
        if jr:
            structure.append({"id": f"joint-grip-{mid[7:]}", "kind": "ellipsoid", "bone": mid, "material": "rattan", "role": "detail", "tone": 0.5, "radii": [jr, jr, jr], "transform": {"translation": [r6(jr), 0.0, 0.0]}, "member": mid, "part": "joint"})
    for b in BONES:
        if "mass" in b:
            m = b["mass"]
            mr = max(feat / 2, 0.006) if scale in (50, 100) else feat / 2
            structure.append({"id": f"mass-{b['id']}", "kind": "ellipsoid", "bone": b["id"], "material": MASS_MATERIAL[b["id"]], "role": "detail", "tone": 0.5, "radii": [r6(mr)] * 3, "transform": {"translation": [r6(m["comX"]), r6(m["comY"]), r6(MASS_W.get(b["id"], 0.0))]}, "member": b["id"], "part": "mass"})
    return body, thermal, structure


MASS_MATERIAL = {}
TRUE_COM = {}
MASS_W = {}
DECISIONS_2D = {}
DECISIONS_3D = {}


def compute_masses():
    """⚖️ Bone masses from the 1:50 parts: volume of every part (hollow sections for tubes) times its density, centres in bone coordinates."""
    parts = build_scale(50)
    acc = {}
    hollow_square = {"stretcher": (0.03, 0.002), "foot-bar-bass": (0.03, 0.002), "foot-bar-treble": (0.03, 0.002)}
    for p in parts:
        v = p.volume()
        if p.id in hollow_square:
            a, w = hollow_square[p.id]
            v = v * (a * a - (a - 2 * w) ** 2) / (a * a)
        if p.id.startswith("leg-tube"):
            v = v * (LEG_R ** 2 - (LEG_R - 0.0015) ** 2) / LEG_R ** 2
        if p.id.startswith("cross-tube"):
            v = v * (CROSS_R ** 2 - (CROSS_R - 0.0015) ** 2) / CROSS_R ** 2
        dens = DENSITY[{"rosewood": "rosewood", "padauk": "padauk", "beech": "beech", "alu-lacquer": "alu", "steel-powder": "steel", "steel-zinc": "zinc", "rubber": "rubber"}[p.material]]
        m = v * dens
        if p.id.startswith("tyre"):
            m = m * 0.75
        c = p.centroid()
        acc.setdefault(p.bone, []).append((m, c, p.material))
    for mid in MALLETS:
        f = FRAMES[mid]
        shaft = math.pi * MALLET_SHAFT_R ** 2 * MALLET_SHAFT_END * DENSITY["rattan"]
        head = 4 / 3 * math.pi * MALLET_HEAD_R ** 3 * DENSITY["phenolic"]
        wrap = math.pi * (0.0055 ** 2 - MALLET_SHAFT_R ** 2) * 0.09 * DENSITY["tape"]
        acc.setdefault(mid, []).append((shaft, [f.o[i] + 0.175 * f.r[i][0] for i in range(3)], "rattan"))
        acc[mid].append((head, [f.o[i] + MALLET_HEAD_U * f.r[i][0] for i in range(3)], "phenolic"))
        acc[mid].append((wrap, [f.o[i] + 0.05 * f.r[i][0] for i in range(3)], "tape"))
    for cid, end, side, sz in CASTORS:
        if end == "bass":
            f = FRAMES[f"brake-{end}-{side}"]
            acc.setdefault(f"brake-{end}-{side}", []).append((0.044 * 0.008 * 0.024 * DENSITY["zinc"], [f.o[i] + 0.022 * f.r[i][0] for i in range(3)], "steel-zinc"))
    total = 0.0
    for b in BONES:
        items = acc.get(b["id"])
        if not items:
            continue
        m = sum(i[0] for i in items)
        c = [sum(i[0] * i[1][k] for i in items) / m for k in range(3)]
        TRUE_COM[b["id"]] = list(c)
        if b["id"].startswith("leg-"):
            c[1] = (FOOT_Y0 + FOOT_Y1) / 2
        if b["id"] == "accidentals":
            c[0] = min((a["x"] for a in ACCIDENTALS), key=lambda x: abs(x - c[0]))
        loc = FRAMES[b["id"]].local(c)
        mats = {}
        for i in items:
            mats[i[2]] = mats.get(i[2], 0) + i[0]
        MASS_MATERIAL[b["id"]] = max(mats, key=mats.get)
        MASS_W[b["id"]] = loc[2]
        b["mass"] = {"kg": round(m, 3), "comX": r6(loc[0]), "comY": r6(loc[1]), "distribution": "point", "source": "authored-estimate", "reference": "wagenfuehr-2007"}
        total += round(m, 3)
    return round(total, 3), acc


def sections():
    """📏 Member sections of the load-bearing bones."""
    sec = {
        "stand": (0.03 ** 2 - 0.026 ** 2, (0.03 ** 4 - 0.026 ** 4) / 12, 0.015, "steel-powder", t("Square steel tube 30 x 30 x 2 mm", "Stahl-Vierkantrohr 30 x 30 x 2 mm")),
        "leg-bass": (2 * math.pi / 4 * (0.028 ** 2 - 0.025 ** 2), 2 * math.pi / 64 * (0.028 ** 4 - 0.025 ** 4), 0.014, "steel-powder", t("Two steel tubes 28 x 1.5 mm", "Zwei Stahlrohre 28 x 1,5 mm")),
        "leg-treble": (2 * math.pi / 4 * (0.028 ** 2 - 0.025 ** 2), 2 * math.pi / 64 * (0.028 ** 4 - 0.025 ** 4), 0.014, "steel-powder", t("Two steel tubes 28 x 1.5 mm", "Zwei Stahlrohre 28 x 1,5 mm")),
        "keyboard": (4 * 0.03 * 0.044, 4 * 0.03 * 0.044 ** 3 / 12, 0.022, "beech", t("Four beech rails 30 x 44 mm (felt included)", "Vier Buchenleisten 30 x 44 mm (mit Filz)")),
        "mallet-bass": (math.pi / 4 * 0.009 ** 2, math.pi / 64 * 0.009 ** 4, 0.0045, "rattan", t("Rattan shaft 9 mm", "Rattanstiel 9 mm")),
        "mallet-treble": (math.pi / 4 * 0.009 ** 2, math.pi / 64 * 0.009 ** 4, 0.0045, "rattan", t("Rattan shaft 9 mm", "Rattanstiel 9 mm")),
    }
    for b in BONES:
        if b["id"] in sec:
            a, i, c, m, title = sec[b["id"]]
            b["section"] = {"areaM2": float(f"{a:.6g}"), "secondMomentM4": float(f"{i:.6g}"), "extremeFibreM": c, "material": m, "title": title}


def material(id, en, de, eps, alpha, rho, k, cp, e, src, ref):
    """🧱 One material entry."""
    m = {"id": id, "title": t(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": cp}
    if e:
        m["elasticModulusPa"] = e
    m.update({"source": src, "reference": ref})
    return m


MATERIALS = [
    material("rosewood", "Honduras rosewood bar, aged nitro lacquer worn at the playing spots", "Honduras-Palisander-Klangplatte, gealterter Nitrolack, an den Anschlagstellen abgespielt", 0.9, 0.78, 1030, 0.2, 1600, 2.2e10, "literature", "wagenfuehr-2007"),
    material("padauk", "Padauk replacement bar, younger lacquer (lighter red)", "Padouk-Ersatzplatte, jüngerer Lack (helleres Rot)", 0.9, 0.62, 750, 0.17, 1600, 1.5e10, "literature", "wagenfuehr-2007"),
    material("beech", "Stained beech, satin lacquer scuffed at the cheek edges", "Gebeizte Buche, Seidenmattlack, an den Wangenkanten abgestoßen", 0.9, 0.6, 720, 0.16, 1600, 1.4e10, "literature", "iso-10456-2007"),
    material("alu-lacquer", "Lacquered aluminium tube, lacquer dulled and tarnished", "Lackiertes Aluminiumrohr, Lack matt und angelaufen", 0.7, 0.35, 2700, 160, 900, 7.0e10, "authored-estimate", "incropera-2018"),
    material("steel-powder", "Black powder-coated steel tube, coat chipped at the feet", "Schwarz pulverbeschichtetes Stahlrohr, Beschichtung an den Füßen abgeplatzt", 0.9, 0.88, 7800, 50, 450, 2.1e11, "literature", "iso-10456-2007"),
    material("steel-zinc", "Zinc-plated pressed steel, tarnished", "Verzinkter Stahlpressteil, angelaufen", 0.3, 0.55, 7800, 50, 450, 2.1e11, "authored-estimate", "incropera-2018"),
    material("rubber", "Grey castor rubber, worn flat at the tread", "Graues Rollengummi, an der Lauffläche abgeflacht", 0.92, 0.85, 1200, 0.25, 1900, None, "literature", "iso-10456-2007"),
    material("rattan", "Rattan mallet shaft", "Rattan-Schlägelstiel", 0.9, 0.55, 600, 0.12, 1700, 3.0e9, "authored-estimate", "wagenfuehr-2007"),
    material("phenolic", "Phenolic mallet head, scuffed", "Phenol-Schlägelkopf, verkratzt", 0.9, 0.7, 1350, 0.25, 1400, 7.0e9, "authored-estimate", "incropera-2018"),
    material("tape", "Cloth grip tape, worn and faded", "Gewebe-Griffband, abgegriffen und verblasst", 0.92, 0.6, 1100, 0.2, 1500, None, "authored-estimate", "iso-10456-2007"),
]

REFERENCES = [
    {"id": "hornbostel-sachs-1961", "citation": "von Hornbostel, E. M. & Sachs, C. (1961). Classification of Musical Instruments, transl. A. Baines & K. P. Wachsmann. Galpin Society Journal 14, 3-29 (class 111.212: sets of percussion sticks, the xylophone)."},
    {"id": "fletcher-rossing-1998", "citation": "Fletcher, N. H. & Rossing, T. D. (1998). The Physics of Musical Instruments, 2nd ed., Springer, ch. 19 Mallet percussion instruments (nodes of a free bar at 0.224 L, xylophone tuning of the third partial to three times the fundamental, quarter-wave tube resonators).", "url": "https://doi.org/10.1007/978-0-387-21603-4"},
    {"id": "bork-1995", "citation": "Bork, I. (1995). Practical tuning of xylophone bars and resonators. Applied Acoustics 46, 103-127 (arch undercut, resonator length and end correction).", "url": "https://doi.org/10.1016/0003-682X(95)93954-3"},
    {"id": "blades-1992", "citation": "Blades, J. (1992). Percussion Instruments and Their History, rev. ed., Bold Strummer (the orchestral xylophone on its frame, written range and mallets)."},
    {"id": "iso-16-1975", "citation": "ISO 16:1975 Acoustics - Standard tuning frequency (A4 = 440 Hz; F4 = 349.23 Hz in equal temperament)."},
    {"id": "wagenfuehr-2007", "citation": "Wagenführ, R. (2007). Holzatlas, 6th ed., Fachbuchverlag Leipzig im Carl Hanser Verlag (Honduras rosewood 0.95-1.10 g/cm3, padauk 0.70-0.80 g/cm3, beech 0.72 g/cm3, rattan)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (steel, aluminium, timber, rubber)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L. & Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities of lacquered, anodised and oxidised metals, convection)."},
    {"id": "en-12530-1999", "citation": "EN 12530:1999 Castors and wheels - Castors and wheels for manually propelled institutional applications (swivel castors with brake, wheel diameters 50-125 mm, rolling resistance)."},
    {"id": "xylophone-class", "citation": "Catalogue-typical data of 3.5-octave rosewood concert xylophones on frames with resonators and castors, compiled from several makers' data sheets as a type, no single make: written range F3-C7 (44 bars), bars 38-41 mm wide and 22-25 mm thick, overall 1.45-1.50 m by 0.75-0.81 m, playing height 0.86-0.90 m, 35-50 kg with frame (authored estimate)."},
]


def anchors():
    """📍 Floor contacts of the castors, the push point, the strike points and the grips."""
    out = []
    for cid, end, side, sz in CASTORS:
        out.append({"id": f"floor-{end}-{side}", "bone": cid, "x": 0.085, "y": -TRAIL, "z": 0, "kind": "contact", "title": t(f"Wheel contact of the {end} {'right' if side == 'r' else 'left'} castor", f"Radaufstand der {'Bass' if end == 'bass' else 'Diskant'}-Lenkrolle {'rechts' if side == 'r' else 'links'}"), "support": "contact", "footprintLengthM": 0.02, "footprintWidthM": 0.024, "frictionCoefficient": 0.6})
    kb = FRAMES["keyboard"]
    out.append({"id": "push-bass", "bone": "keyboard", "x": r6(-CHEEK_X1 - kb.o[0]), "y": r6(0.84 - kb.o[1]), "z": 0, "kind": "load", "title": t("Push point on the bass cheek", "Schiebepunkt an der Basswange")})
    nat = FRAMES["naturals"]
    for mid, m in MALLETS.items():
        nb = NATURALS[m["bar"]]
        out.append({"id": f"strike-{nb['name'].lower()}", "bone": "naturals", "x": r6(nb["x"] - nat.o[0]), "y": r6(NAT_Y1 - nat.o[1]), "z": MALLET_HEAD_Z, "kind": "load", "title": t(f"Strike point on the {nb['name']} bar", f"Anschlagpunkt auf der Platte {nb['name']}")})
        out.append({"id": f"grip-{mid[7:]}", "bone": mid, "x": 0.04, "y": 0.0, "z": 0, "kind": "load", "title": t(f"Hand grip of the {'left' if mid == 'mallet-bass' else 'right'}-hand mallet", f"Handgriff des Schlägels der {'linken' if mid == 'mallet-bass' else 'rechten'} Hand")})
    return out


REGIONS = ["bars", "resonators", "keyboard-frame", "stand", "castors", "mallets"]


def binding(played=False):
    """🌡️ Thermal binding of a pose: indoor depot, no sun; the grips warm in the hands when played."""
    regs = {}
    for r in REGIONS:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if r == "castors":
            b["contact"] = {"fraction": 0.03, "tempC": 17}
        if r == "mallets" and played:
            b["contact"] = {"fraction": 0.12, "tempC": 33}
        regs[r] = b
    return {"regions": regs, "emitters": {}}


G = 9.80665


def mallet_mass(mid):
    return BONE_BY_ID[mid]["mass"]["kg"]


BONE_BY_ID = {}


def poses():
    """🧍 Stored (default), rolled, played and strike, plus the key pose of the roll."""
    contacts = [f"floor-{e}-{s}" for _, e, s, _ in CASTORS]
    brake_off = {f"brake-bass-{s}": {"angleDeg": 20} for s in ("r", "l")}
    lift = 0.11
    raised, strike = {}, {}
    for mid, m in MALLETS.items():
        g = mid[7:]
        gy = m["gy"] + lift
        th = math.degrees(math.asin((NAT_Y1 + MALLET_HEAD_R - gy) / MALLET_HEAD_U))
        m["strike_offset"] = th - m["beta"]
        m["raised_offset"] = 15 - m["beta"]
        raised[f"grip-{g}"] = {"dy": lift}
        raised[mid] = {"angleDeg": r6(m["raised_offset"])}
        strike[f"grip-{g}"] = {"dy": lift}
        strike[mid] = {"angleDeg": r6(m["strike_offset"])}
    hands = [{"id": f"hand-{mid[7:]}", "kind": "live", "title": t(f"{'Left' if mid == 'mallet-bass' else 'Right'} hand holding the mallet", f"{'Linke' if mid == 'mallet-bass' else 'Rechte'} Hand hält den Schlägel"), "fxN": 0, "fyN": r6(mallet_mass(mid) * G), "anchor": f"grip-{mid[7:]}", "source": "authored-estimate", "reference": "blades-1992"} for mid in MALLETS]
    strikes = [{"id": f"strike-{mid[7:]}", "kind": "live", "title": t(f"Mallet impact on {NATURALS[m['bar']]['name']} (peak, quasi-static)", f"Schlägelanschlag auf {NATURALS[m['bar']]['name']} (Spitze, quasistatisch)"), "fxN": 0, "fyN": -80, "anchor": f"strike-{NATURALS[m['bar']]['name'].lower()}", "source": "authored-estimate", "reference": "fletcher-rossing-1998"} for mid, m in MALLETS.items()]
    push = {"id": "push", "kind": "live", "title": t("Push at the bass cheek to roll the frame", "Schub an der Basswange zum Rollen des Gestells"), "fxN": 40, "fyN": 0, "anchor": "push-bass", "source": "authored-estimate", "reference": "en-12530-1999"}
    roll = 1.2
    spin = -math.degrees(roll / WHEEL_R)
    wheels = {f"wheel-{e}-{s}": {"angleDeg": r6(spin)} for _, e, s, _ in CASTORS}
    P = [
        {"id": "stored", "title": t("Stored in the depot", "Im Lager abgestellt"), "description": t("Standing on its four castors with the two bass castors braked, the two mallets lying across C4 and E4 with their handles over the player-side edge, as the dealer keeps it between viewings.", "Auf den vier Lenkrollen stehend, die beiden Bassrollen gebremst, die zwei Schlägel quer über C4 und E4 gelegt, die Griffe über die Spielerkante ragend, wie der Händler es zwischen zwei Besichtigungen abstellt."), "bones": {}, "ground": "auto", "contacts": contacts, "thermal": binding(), "loads": []},
        {"id": "rolled", "title": t("Brakes released, pushed at the bass end", "Bremsen gelöst, am Bassende geschoben"), "description": t("Both brake pedals flipped up, a 40 N push at the bass cheek starts the frame rolling on its four trailing castors (rubber wheels on a hard floor).", "Beide Bremspedale hochgeklappt, ein Schub von 40 N an der Basswange setzt das Gestell auf seinen vier nachlaufenden Lenkrollen in Bewegung (Gummiräder auf hartem Boden)."), "bones": brake_off, "ground": "auto", "contacts": contacts, "thermal": binding(), "loads": [push]},
        {"id": "played", "title": t("Played standing, mallets raised", "Im Stehen gespielt, Schlägel erhoben"), "description": t("Brakes locked; the player's hands hold the two mallets 0.97 m above the floor in front of the naturals, the heads raised 15 degrees above the horizontal before the stroke; the hands carry the mallet weight.", "Bremsen gesperrt; die Hände des Spielers halten die zwei Schlägel 0,97 m über dem Boden vor den Stammtönen, die Köpfe vor dem Schlag 15 Grad über die Waagrechte gehoben; die Hände tragen das Schlägelgewicht."), "bones": raised, "ground": "auto", "contacts": contacts, "thermal": binding(True), "loads": hands},
        {"id": "strike", "title": t("Strike on C4 and E4", "Anschlag auf C4 und E4"), "description": t("Both phenolic heads hit the middle of the C4 and E4 bars together (a third); the peak impact of about 80 N per mallet is taken as a quasi-static load at the strike points, carried by the felted rails into the frame.", "Beide Phenolköpfe treffen gleichzeitig die Mitte der Platten C4 und E4 (eine Terz); die Spitzenkraft von etwa 80 N je Schlägel wird als quasistatische Last an den Anschlagpunkten angesetzt und über die befilzten Leisten in den Rahmen geleitet."), "bones": strike, "ground": "auto", "contacts": contacts, "thermal": binding(True), "loads": hands + strikes},
        {"id": "rolling-far", "title": t("Rolled 1.2 m", "1,2 m gerollt"), "description": t("Key pose of the roll: the frame 1.2 m further on, every wheel turned by the distance over its 37.5 mm radius.", "Schlüsselpose des Rollens: das Gestell 1,2 m weiter, jedes Rad um den Weg durch seinen Radius von 37,5 mm gedreht."), "key": True, "bones": {**brake_off, **wheels}, "root": {"x": roll}, "ground": "auto", "contacts": contacts, "thermal": binding(), "loads": [push]},
    ]
    return P, raised


def clips(raised):
    """🎞️ The roll to the next room and the mallet strokes."""
    contacts = lambda d: [{"anchor": f"floor-{e}-{s}", "fromS": 0, "toS": d} for _, e, s, _ in CASTORS]
    carry = {"id": "carry", "title": t("Roll to the next room", "In den Nebenraum rollen"), "description": t("The brakes are released, a push at the bass cheek rolls the xylophone 1.2 m on its castors in 5 s (0.24 m/s), the wheels turning without slip; the mallets ride on the bars.", "Die Bremsen werden gelöst, ein Schub an der Basswange rollt das Xylophon in 5 s 1,2 m auf seinen Rollen (0,24 m/s), die Räder drehen schlupffrei; die Schlägel fahren auf den Platten mit."), "durationS": 6.5, "fps": 10, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.8, "pose": "rolled", "ease": "linear"}, {"t": 5.8, "pose": "rolling-far", "ease": "linear"}, {"t": 6.5, "pose": "rolling-far", "ease": "linear"}], "contacts": contacts(6.5), "phases": [{"id": "release", "fromS": 0, "toS": 0.8, "title": t("Releasing the brakes", "Bremsen lösen")}, {"id": "roll", "fromS": 0.8, "toS": 5.8, "title": t("Rolling", "Rollen")}, {"id": "arrive", "fromS": 5.8, "toS": 6.5, "title": t("Standing at the new place", "Am neuen Platz")}], "events": [{"t": 0.6, "kind": "motion", "title": t("Brake pedals flipped up", "Bremspedale hochgeklappt")}, {"t": 0.8, "kind": "load-on", "title": t("Push begins", "Schub beginnt"), "target": "push"}, {"t": 5.8, "kind": "load-off", "title": t("Push ends", "Schub endet"), "target": "push"}], "tracks": [{"target": "load:push", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 0.8, "v": 0}, {"t": 1.2, "v": 1}, {"t": 5.4, "v": 1}, {"t": 5.8, "v": 0}, {"t": 6.5, "v": 0}]}]}
    strokes = {"id": "mallet-strokes", "title": t("Mallet strokes on C4 and E4", "Schlägelanschläge auf C4 und E4"), "description": t("A loop of one stroke per second: both mallets fall from the raised position, hit C4 and E4 at 0.18 s, rebound off the bars and are raised again; the impact load lasts a few hundredths of a second.", "Eine Schleife mit einem Schlag pro Sekunde: beide Schlägel fallen aus der erhobenen Stellung, treffen C4 und E4 bei 0,18 s, federn von den Platten zurück und werden wieder gehoben; die Anschlaglast dauert wenige Hundertstelsekunden."), "durationS": 1.0, "fps": 30, "loop": True, "rootMotion": "none", "keys": [{"t": 0, "pose": "strike", "ease": "in", "bones": raised}, {"t": 0.18, "pose": "strike", "ease": "out"}, {"t": 0.4, "pose": "strike", "ease": "smooth", "bones": raised}, {"t": 1.0, "pose": "strike", "ease": "smooth", "bones": raised}], "contacts": contacts(1.0), "phases": [{"id": "fall", "fromS": 0, "toS": 0.18, "title": t("Mallets falling", "Schlägel fallen")}, {"id": "rebound", "fromS": 0.18, "toS": 0.4, "title": t("Rebound", "Rückprall")}, {"id": "raise", "fromS": 0.4, "toS": 1.0, "title": t("Raised for the next stroke", "Für den nächsten Schlag gehoben")}], "events": [{"t": 0.18, "kind": "load-on", "title": t("Heads hit the bars", "Köpfe treffen die Platten"), "target": "strike-bass"}, {"t": 0.2, "kind": "load-off", "title": t("Heads leave the bars", "Köpfe verlassen die Platten"), "target": "strike-bass"}], "tracks": [{"target": f"load:strike-{m}", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 0.16, "v": 0}, {"t": 0.18, "v": 1}, {"t": 0.2, "v": 0}, {"t": 1.0, "v": 0}]} for m in ("bass", "treble")]}
    return [carry, strokes]


def thermal(areas):
    """🌡️ Indoor depot, six passive regions; the stand states the passive 0 W nameplate."""
    def layer(id, en, de, kind, mat, th, k, rho, cp):
        return {"id": id, "title": t(en, de), "kind": kind, "material": mat, "thicknessM": th, "resistanceM2KW": float(f"{th / k:.4g}"), "heatCapacityJM2K": round(rho * cp * th), "source": "literature", "reference": "iso-10456-2007"}
    R = [
        {"id": "bars", "title": t("Rosewood bars (one padauk replacement)", "Palisanderplatten (eine Ersatzplatte aus Padouk)"), "kind": "surface", "model": "passive", "material": "rosewood", "areaM2": areas["bars"], "characteristicLengthM": 0.04, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5, "layers": [layer("bar", "Rosewood bar 22 mm", "Palisanderplatte 22 mm", "timber", "rosewood", 0.022, 0.2, 1030, 1600)], "heatCapacityJM2K": round(1030 * 1600 * 0.022), "source": "authored-estimate", "reference": "wagenfuehr-2007"},
        {"id": "resonators", "title": t("Aluminium resonator tubes and bank bars, inside and outside", "Aluminium-Resonatorrohre und Bankleisten, innen und außen"), "kind": "surface", "model": "passive", "material": "alu-lacquer", "areaM2": areas["resonators"], "characteristicLengthM": 0.04, "geometry": "cylinder", "skyViewFactor": 0.2, "layers": [layer("tube-wall", "Aluminium tube wall 1 mm", "Aluminium-Rohrwand 1 mm", "metal", "alu-lacquer", 0.001, 160, 2700, 900)], "heatCapacityJM2K": round(2700 * 900 * 0.001), "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "keyboard-frame", "title": t("Beech rails with felt and end cheeks", "Buchenleisten mit Filz und Stirnwangen"), "kind": "surface", "model": "passive", "material": "beech", "areaM2": areas["keyboard-frame"], "characteristicLengthM": 0.044, "geometry": "vertical-plate", "skyViewFactor": 0.3, "layers": [layer("timber", "Beech 30 mm", "Buche 30 mm", "timber", "beech", 0.03, 0.16, 720, 1600)], "heatCapacityJM2K": round(720 * 1600 * 0.03), "source": "authored-estimate", "reference": "iso-10456-2007"},
        {"id": "stand", "title": t("Powder-coated steel stand", "Pulverbeschichtetes Stahlgestell"), "kind": "surface", "model": "passive", "material": "steel-powder", "areaM2": areas["stand"], "characteristicLengthM": 0.028, "geometry": "cylinder", "skyViewFactor": 0.4, "layers": [layer("tube", "Steel tube wall 1.5 mm", "Stahlrohrwand 1,5 mm", "metal", "steel-powder", 0.0015, 50, 7800, 450)], "heatCapacityJM2K": round(7800 * 450 * 0.0015), "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "iso-10456-2007"},
        {"id": "castors", "title": t("Swivel castors: rubber wheels, zinc-plated forks and brake pedals", "Lenkrollen: Gummiräder, verzinkte Gabeln und Bremspedale"), "kind": "tyre", "model": "passive", "material": "rubber", "areaM2": areas["castors"], "characteristicLengthM": 0.075, "geometry": "cylinder", "skyViewFactor": 0.1, "layers": [layer("tread", "Rubber tread 12 mm", "Gummilauffläche 12 mm", "polymer", "rubber", 0.012, 0.25, 1200, 1900)], "heatCapacityJM2K": round(1200 * 1900 * 0.012), "source": "authored-estimate", "reference": "en-12530-1999"},
        {"id": "mallets", "title": t("Two mallets: rattan shafts, taped grips, phenolic heads", "Zwei Schlägel: Rattanstiele, bandumwickelte Griffe, Phenolköpfe"), "kind": "surface", "model": "passive", "material": "rattan", "areaM2": areas["mallets"], "characteristicLengthM": 0.009, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [layer("shaft", "Rattan 9 mm", "Rattan 9 mm", "timber", "rattan", 0.0045, 0.12, 600, 1700)], "heatCapacityJM2K": round(600 * 1700 * 0.0045), "source": "authored-estimate", "reference": "wagenfuehr-2007"},
    ]
    env = {"airTempC": 18, "meanRadiantTempC": 18, "skyTempC": 18, "groundTempC": 17, "relativeHumidity": 0.45, "windMS": 0.05, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
    return {"environment": env, "operating": [], "passengers": [], "regions": R, "emitters": [], "assumptions": [
        t("An acoustic instrument draws no power: every region is a passive surface in the unheated depot at 18 °C without sun (the stand states the passive 0 W nameplate); the mallet grips take 33 °C from the hands while played.", "Ein akustisches Instrument nimmt keine Leistung auf: jede Region ist eine passive Fläche im ungeheizten Lager bei 18 °C ohne Sonne (das Gestell nennt die passive Nennleistung 0 W); die Schlägelgriffe nehmen beim Spielen 33 °C von den Händen an."),
        t("Aged surfaces: the worn nitro lacquer of the bars and the dulled, tarnished lacquer of the tubes are authored estimates of emissivity and absorptivity; the felt strips under the bars are counted with the rail region because the bars cover them except in the 12 mm gaps.", "Gealterte Oberflächen: der abgespielte Nitrolack der Platten und der matte, angelaufene Lack der Rohre sind geschätzte Emissions- und Absorptionsgrade; die Filzstreifen unter den Platten zählen zur Leistenregion, weil die Platten sie bis auf die 12-mm-Spalte bedecken."),
        t("The resonator area counts the inside of every tube: the tubes are open at the top 20 mm below the bars and closed at the bottom.", "Die Resonatorfläche zählt das Innere jedes Rohrs: die Rohre sind oben 20 mm unter den Platten offen und unten geschlossen."),
    ]}


STRUCTURE = {"gravityMS2": G, "dynamics": "quasi-static", "supports": [f"floor-{e}-{s}" for _, e, s, _ in CASTORS], "assumptions": [
    t("A rigid frame on four castor contacts (bass pair at x = -0.72 m, treble pair at x = +0.66 m, both at z = +-0.31 m; the wheels trail 30 mm behind their kingpins); the bars rest on felted beech rails at their nodes (0.224 L from the ends), the rails sit on the two end cheeks and the cheeks on the leg frames, so the tree of the rig carries the keyboard from the bass end.", "Ein starrer Rahmen auf vier Rollenkontakten (Basspaar bei x = -0,72 m, Diskantpaar bei x = +0,66 m, beide bei z = +-0,31 m; die Räder laufen 30 mm hinter ihren Königszapfen nach); die Platten liegen an ihren Knotenpunkten (0,224 L von den Enden) auf befilzten Buchenleisten, die Leisten auf den beiden Stirnwangen und die Wangen auf den Beinrahmen, so dass der Baum des Rigs die Klaviatur vom Bassende her trägt."),
    t("Masses are the sum of the parts of the 1:50 model, section times density (hollow 30 x 30 x 2 mm and 28 x 1.5 mm steel tubes, 1 mm aluminium tubes); the 80 N mallet impacts and the 40 N push are authored estimates, the impact a quasi-static peak of a contact lasting about a millisecond.", "Die Massen sind die Summe der Teile des 1:50-Modells, Querschnitt mal Dichte (hohle Stahlrohre 30 x 30 x 2 mm und 28 x 1,5 mm, Aluminiumrohre 1 mm); die Schlägelanschläge von 80 N und der Schub von 40 N sind Schätzwerte, der Anschlag eine quasistatische Spitze eines etwa eine Millisekunde dauernden Kontakts."),
    t("The string-free bars carry no static tension; the strike load path is bar, felt, rail, cheek, legs, foot bar, castor, floor.", "Die saitenlosen Platten tragen keine statische Spannung; der Lastpfad des Anschlags ist Platte, Filz, Leiste, Wange, Beine, Fußrohr, Rolle, Boden."),
], "clinical": False}


def dec(**kw):
    """📝 The eight decision texts of a representation."""
    return {k: t(*kw[k]) for k in ("silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays")}


DECISIONS_2D[50] = dec(
    silhouette=("Side elevation from the player's side, 29 mm long and 18 mm high: the 26 natural bars seen end-on at their 52 mm pitch with the 18 raised accidentals behind them, the felted rails and the end cheeks, the 44 resonator tubes hanging from their bank bars with the bass octave stepping down from 233 mm to 100 mm, the two leg frames with cross tubes, the stretcher, four swivel castors with forks, hubs and the two brake pedals, and the mallets lying across C4 and E4. Front view from the treble end: every bar profile with its arch undercut and the chips, the two levels of the rows, rails, cheek with the inventory label, the tube rows and the end frame. Plan: the trapezoid keyboard, every bar with its worn playing spot, the cheeks, the mallets and the trailing castors.", "Seitenansicht von der Spielerseite, 29 mm lang und 18 mm hoch: die 26 Stammtonplatten stirnseitig im Abstand von 52 mm, dahinter die 18 erhöhten Halbtöne, die befilzten Leisten und die Stirnwangen, die 44 Resonatorrohre an ihren Bankleisten, die Bassoktave von 233 mm auf 100 mm gestuft, die zwei Beinrahmen mit Querrohren, das Längsrohr, vier Lenkrollen mit Gabeln, Naben und den zwei Bremspedalen sowie die über C4 und E4 gelegten Schlägel. Vorderansicht vom Diskantende: jedes Plattenprofil mit seinem Bogenunterschnitt und den Ausbrüchen, die zwei Ebenen der Reihen, Leisten, Wange mit dem Inventaretikett, die Rohrreihen und der Endrahmen. Draufsicht: die trapezförmige Klaviatur, jede Platte mit ihrer abgespielten Anschlagstelle, die Wangen, die Schlägel und die nachlaufenden Rollen."),
    retained=("Every bar at its true 40 x 22 mm section and its length (380 mm at F3 to 190 mm at C7), the chips of seven bars at their struck ends, the padauk replacement bar D5 in a lighter tone, every tube at 40 mm with its true length, rails 30 x 44 mm, cheeks 25 mm, tubes 28 mm, castors with 75 mm wheels.", "Jede Platte mit ihrem wahren Querschnitt 40 x 22 mm und ihrer Länge (380 mm bei F3 bis 190 mm bei C7), die Ausbrüche von sieben Platten an ihren Anschlagenden, die Ersatzplatte D5 aus Padouk in hellerem Ton, jedes Rohr mit 40 mm und wahrer Länge, Leisten 30 x 44 mm, Wangen 25 mm, Rohre 28 mm, Rollen mit 75-mm-Rädern."),
    merged=("The far leg, castor and tube rows coincide with the near ones in the side elevation; the felt strips are one line on the rails.", "Ferne Beine, Rollen und Rohrreihen fallen in der Seitenansicht mit den nahen zusammen; die Filzstreifen sind je eine Linie auf den Leisten."),
    omitted=("The bar pins and suspension cord, screws of the cheeks, rivets of the bank bars, the tube stoppers above G4 and the castor bearings.", "Plattenstifte und Aufhängeschnur, Schrauben der Wangen, Nieten der Bankleisten, die Rohrstopfen oberhalb G4 und die Rollenlager."),
    thinElements=("Felt strips (4 mm) and the dent of the A3 tube are detail strokes; worn playing spots, the label text and the scuffs of the legs are texture strokes of 0.1 mm.", "Filzstreifen (4 mm) und die Delle des Rohrs A3 sind Detailstriche; abgespielte Anschlagstellen, die Etikettschrift und die Schrammen der Beine sind Texturstriche von 0,1 mm."),
    hatching=("No hatches: the woods, metals and rubber read by tone (rosewood dark, padauk lighter, beech mid, aluminium light, stand black).", "Keine Schraffur: Hölzer, Metalle und Gummi lesen sich über den Ton (Palisander dunkel, Padouk heller, Buche mittel, Aluminium hell, Gestell schwarz)."),
    styles=("Body pen 0.35 mm for the frame and cheeks, 0.25 mm for bars and rails, 0.18 mm for tubes and castor parts, detail 0.13-0.18 mm, texture 0.1 mm.", "Körperstift 0,35 mm für Gestell und Wangen, 0,25 mm für Platten und Leisten, 0,18 mm für Rohre und Rollenteile, Detail 0,13-0,18 mm, Textur 0,1 mm."),
    overlays=("Thermal and structural views colour the same shapes by region and member; the mallets are revolved solids so they read right when they are raised.", "Thermische und statische Ansichten färben dieselben Formen nach Region und Stab; die Schlägel sind Drehkörper und lesen sich auch erhoben richtig."),
)
DECISIONS_2D[100] = dec(
    silhouette=("Side elevation 15 mm long: the naturals as one band of bars with their 26 divisions as texture lines, the accidentals in their seven groups of two and three, rails, cheeks, the two resonator banks with the stepped bass octave, leg frames, stretcher, castors with fork block and wheel, brake pedals and mallets. Front view: the two levels of the keyboard, rails, cheek, both banks, the end frame and the castors. Plan: the naturals band and the seven accidental groups over the trapezoid, the cheeks and the mallets.", "Seitenansicht 15 mm lang: die Stammtöne als ein Plattenband mit ihren 26 Teilungen als Texturlinien, die Halbtöne in ihren sieben Gruppen zu zwei und drei, Leisten, Wangen, die zwei Resonatorbänke mit gestufter Bassoktave, Beinrahmen, Längsrohr, Rollen mit Gabelblock und Rad, Bremspedale und Schlägel. Vorderansicht: die zwei Ebenen der Klaviatur, Leisten, Wange, beide Bänke, der Endrahmen und die Rollen. Draufsicht: das Stammtonband und die sieben Halbtongruppen über dem Trapez, die Wangen und die Schlägel."),
    retained=("Bar thickness, row levels, lengths and the grouping of the accidentals; tube lengths of the bass octave as steps; rails, cheeks, legs, castors and mallet heads at true size.", "Plattendicke, Reihenhöhen, Längen und die Gruppierung der Halbtöne; Rohrlängen der Bassoktave als Stufen; Leisten, Wangen, Beine, Rollen und Schlägelköpfe in wahrer Größe."),
    merged=("The 12 mm gaps between the bars (0.12 mm on paper, below the 0.15 mm print gap) close: each row or group is one band; the tubes of a bank with their 12 mm gaps and bank bars become one stepped band; the fork cheeks, crown and housing of a castor one block.", "Die 12-mm-Spalte zwischen den Platten (0,12 mm auf dem Papier, unter der Druckgrenze 0,15 mm) schließen sich: jede Reihe oder Gruppe ist ein Band; die Rohre einer Bank mit ihren 12-mm-Spalten und Bankleisten werden ein gestuftes Band; Gabelwangen, Krone und Gehäuse einer Rolle ein Block."),
    omitted=("Chips, undercuts, worn spots, the dent, the label, felt lines, hubs and the tape wrap of the grips.", "Ausbrüche, Unterschnitte, abgespielte Stellen, Delle, Etikett, Filzlinien, Naben und das Griffband."),
    thinElements=("Bar divisions as 0.1 mm texture lines in plan and elevation; mallet shafts at their true 9 mm.", "Plattenteilungen als Texturlinien 0,1 mm in Draufsicht und Ansicht; Schlägelstiele in wahren 9 mm."),
    hatching=("No hatches; tones as at 1:50.", "Keine Schraffur; Töne wie bei 1:50."),
    styles=("Body pen 0.25 mm for frame and cheeks, 0.18 mm for bars, rails, banks and castors, detail 0.13 mm, texture 0.1 mm.", "Körperstift 0,25 mm für Gestell und Wangen, 0,18 mm für Platten, Leisten, Bänke und Rollen, Detail 0,13 mm, Textur 0,1 mm."),
    overlays=("Analytic views on the same bands; the banks are typified rows (envelope) of their tubes.", "Analytische Ansichten auf denselben Bändern; die Bänke sind typisierte Reihen (Hülle) ihrer Rohre."),
)
DECISIONS_2D[200] = dec(
    silhouette=("Side elevation 7.3 mm long and 4.5 mm high: the naturals band, the raised accidentals band, rails and cheeks, the two banks with the bass octave as two slopes, the leg frames, stretcher, castor blocks and the mallets. Front view: two keyboard levels, rails, cheek, banks, end frame, castors. Plan: the naturals band, the accidentals band, cheeks and mallets.", "Seitenansicht 7,3 mm lang und 4,5 mm hoch: das Stammtonband, das erhöhte Halbtonband, Leisten und Wangen, die zwei Bänke mit der Bassoktave als zwei Schrägen, Beinrahmen, Längsrohr, Rollenblöcke und Schlägel. Vorderansicht: zwei Klaviaturebenen, Leisten, Wange, Bänke, Endrahmen, Rollen. Draufsicht: das Stammtonband, das Halbtonband, Wangen und Schlägel."),
    retained=("The 22 mm bar bands and 25 mm cheeks at their true thickness (the thinnest drawing of their bones), legs and tubes at true width.", "Die 22-mm-Plattenbänder und 25-mm-Wangen in wahrer Dicke (die dünnste Zeichnung ihrer Knochen), Beine und Rohre in wahrer Breite."),
    merged=("The seven accidental groups are one band (their 64 mm gaps less the pens print below 0.15 mm at 1:350); the bass octave steps of the banks become two slopes; fork and wheel of a castor one block 34 mm wide; the brake pedals go into the fork block.", "Die sieben Halbtongruppen sind ein Band (ihre 64-mm-Lücken abzüglich der Stifte drucken bei 1:350 unter 0,15 mm); die Stufen der Bassoktave werden zwei Schrägen; Gabel und Rad einer Rolle ein Block von 34 mm Breite; die Bremspedale gehen in den Gabelblock ein."),
    omitted=("Bar divisions, the treble cross tube (212 mm long, 25 mm thick: below the 30 mm feature and too short to be slender) and every 1:100 omission.", "Plattenteilungen, das Diskant-Querrohr (212 mm lang, 25 mm dick: unter dem Merkmal von 30 mm und zu kurz, um schlank zu sein) und alles, was bei 1:100 entfällt."),
    thinElements=("Legs, foot bars, stretcher and the bass cross tube are slender at their true section; the mallet shafts too.", "Beine, Fußrohre, Längsrohr und das Bass-Querrohr sind schlank in ihrem wahren Querschnitt; ebenso die Schlägelstiele."),
    hatching=("No hatches.", "Keine Schraffur."),
    styles=("One body pen of 0.18 mm, detail 0.13 mm.", "Ein Körperstift von 0,18 mm, Detail 0,13 mm."),
    overlays=("Analytic views on the same shapes.", "Analytische Ansichten auf denselben Formen."),
)
DECISIONS_2D[500] = dec(
    silhouette=("Side elevation 2.9 mm long and 1.8 mm high: the frame deck with the naturals band on it and the accidentals behind, the two resonator banks as one slope each, the leg frames and stretcher, the castor blocks and the mallet shafts. Front view: deck, the two rows, both banks, legs, foot bar and castors. Plan: the trapezoid of the two rows over the frame deck and the mallet shafts.", "Seitenansicht 2,9 mm lang und 1,8 mm hoch: das Rahmendeck mit dem Stammtonband darauf und den Halbtönen dahinter, die zwei Resonatorbänke mit je einer Schräge, Beinrahmen und Längsrohr, die Rollenblöcke und die Schlägelstiele. Vorderansicht: Deck, die zwei Reihen, beide Bänke, Beine, Fußrohr und Rollen. Draufsicht: das Trapez der zwei Reihen über dem Rahmendeck und die Schlägelstiele."),
    retained=("Overall size, the two bar levels, the tapering trapezoid, the resonator depth, the leg frames and the castors.", "Gesamtgröße, die zwei Plattenebenen, das sich verjüngende Trapez, die Resonatortiefe, die Beinrahmen und die Rollen."),
    merged=("Rails and cheeks are one frame deck; the accidental groups are one band reaching down to the felts (the 64 mm gaps are 0.13 mm on paper); each bank is one band with one slope.", "Leisten und Wangen sind ein Rahmendeck; die Halbtongruppen ein Band bis hinunter zu den Filzen (die 64-mm-Lücken sind 0,13 mm auf dem Papier); jede Bank ist ein Band mit einer Schräge."),
    omitted=("Cross tubes, brake pedals, mallet heads and every finer omission.", "Querrohre, Bremspedale, Schlägelköpfe und alles, was feiner entfällt."),
    thinElements=("The bars, banks, legs, foot bars and stretcher keep their true sections, drawn with a 0.18-0.2 mm pen so they print to 1:1000; mallet shafts as closed outlines.", "Platten, Bänke, Beine, Fußrohre und Längsrohr behalten ihre wahren Querschnitte, mit einem Stift von 0,18-0,2 mm gezeichnet, damit sie bis 1:1000 drucken; Schlägelstiele als geschlossene Umrisse."),
    hatching=("No hatches.", "Keine Schraffur."),
    styles=("Body pen 0.13 mm, small parts 0.18-0.2 mm.", "Körperstift 0,13 mm, kleine Teile 0,18-0,2 mm."),
    overlays=("Analytic views on the same shapes.", "Analytische Ansichten auf denselben Formen."),
)
DECISIONS_3D = {
    50: [t("1:50 (feature 7.5 mm) is the instrument as built: 44 bars as extrusions of their front profile with the arch undercut between the nodes and seven chips at the struck ends, the padauk bar D5, 44 hollow aluminium tubes (1 mm wall, 3 mm bottom, open top) of their quarter-wave lengths on four 8 x 30 mm bank bars, four felted beech rails along the node lines, two cheeks, the steel stand (stretcher, foot bars, four legs, two cross tubes), four castors (housing, crown, two fork cheeks thickened from 3 to 8 mm, rubber wheel, hub), two brake pedals and two mallets (shaft, tape wrap, head).", "1:50 (kleinstes Merkmal 7,5 mm) ist das Instrument wie gebaut: 44 Platten als Extrusionen ihres Stirnprofils mit dem Bogenunterschnitt zwischen den Knoten und sieben Ausbrüchen an den Anschlagenden, die Padouk-Platte D5, 44 hohle Aluminiumrohre (Wand 1 mm, Boden 3 mm, oben offen) in ihren Viertelwellenlängen an vier Bankleisten 8 x 30 mm, vier befilzte Buchenleisten auf den Knotenlinien, zwei Wangen, das Stahlgestell (Längsrohr, Fußrohre, vier Beine, zwei Querrohre), vier Lenkrollen (Gehäuse, Krone, zwei von 3 auf 8 mm verstärkte Gabelwangen, Gummirad, Nabe), zwei Bremspedale und zwei Schlägel (Stiel, Griffband, Kopf)."), t("The felt (4 mm) is part of the rail solids, which reach up to the bar bottoms so the bars rest on them; the structure list carries the bars, rails, cheeks, tubes of the stand, forks, pedals and shafts as members, joints at kingpins, axles and grips, a support under every wheel and a mass marker per bone.", "Der Filz (4 mm) gehört zu den Leistenkörpern, die bis an die Plattenunterseiten reichen, damit die Platten auf ihnen liegen; die Tragwerksliste führt Platten, Leisten, Wangen, Gestellrohre, Gabeln, Pedale und Stiele als Stäbe, Gelenke an Königszapfen, Achsen und Griffen, ein Auflager unter jedem Rad und eine Massenmarke je Knochen.")],
    100: [t("1:100 (feature 15 mm): each bar row or accidental group is one 22 mm slab over its trapezoid (the 12 mm gaps close), each resonator bank one typified row (envelope) of its tubes with the bass octave as steps and the upper tubes as one 100 mm band, the castor fork one block; undercuts, chips, hubs and the tape wrap go.", "1:100 (kleinstes Merkmal 15 mm): jede Plattenreihe oder Halbtongruppe ist eine 22-mm-Platte über ihrem Trapez (die 12-mm-Spalte schließen sich), jede Resonatorbank eine typisierte Reihe (Hülle) ihrer Rohre mit der Bassoktave als Stufen und den oberen Rohren als ein 100-mm-Band, die Rollengabel ein Block; Unterschnitte, Ausbrüche, Naben und Griffband entfallen."), t("Legs, cross tubes and mallet shafts keep their true sections as slender parts; the bank members are slender hanger strips inside the bands.", "Beine, Querrohre und Schlägelstiele behalten als schlanke Teile ihre wahren Querschnitte; die Bankstäbe sind schlanke Hängeleisten in den Bändern.")],
    200: [t("1:200 (feature 30 mm): the naturals slab, one accidentals slab (the seven groups merged), rails and cheeks as at 1:100 at their drawn thickness (22 and 25 mm, the thinnest drawing of their bones), the bass octave of each bank as two slopes, fork and wheel of a castor merged into a block and a 34 mm wheel, the brake pedals and the treble cross tube left out; stand tubes slender.", "1:200 (kleinstes Merkmal 30 mm): die Stammtonplatte, eine Halbtonplatte (die sieben Gruppen zusammengefasst), Leisten und Wangen wie bei 1:100 in ihrer gezeichneten Dicke (22 und 25 mm, die dünnste Zeichnung ihrer Knochen), die Bassoktave jeder Bank als zwei Schrägen, Gabel und Rad einer Rolle zu einem Block und einem 34-mm-Rad zusammengefasst, Bremspedale und Diskant-Querrohr entfallen; Gestellrohre schlank.")],
    500: [t("1:500 (feature 75 mm): the rails and cheeks are one beech frame deck, the naturals one 22 mm slab on it, the accidentals one slab down to the felts, each bank one band with a single slope, the stand tubes slender, castors as fork block and wheel, the mallets as slender shafts without heads; no cross tubes and no pedals.", "1:500 (kleinstes Merkmal 75 mm): Leisten und Wangen sind ein Rahmendeck aus Buche, die Stammtöne eine 22-mm-Platte darauf, die Halbtöne eine Platte bis zu den Filzen, jede Bank ein Band mit einer Schräge, die Gestellrohre schlank, Rollen als Gabelblock und Rad, die Schlägel als schlanke Stiele ohne Köpfe; keine Querrohre und keine Pedale.")],
}


def bbox_of_rest(parts):
    """📦 Rest bounding box of a part list plus the mallets."""
    lo, hi = [1e9] * 3, [-1e9] * 3
    for p in parts:
        b = p.bounds()
        for k in range(3):
            lo[k] = min(lo[k], b[k][0])
            hi[k] = max(hi[k], b[k][1])
    for mid in MALLETS:
        f = FRAMES[mid]
        for u, r in ((0.0055, 0.0055), (MALLET_HEAD_U, MALLET_HEAD_R), (MALLET_LEN, 0.0)):
            c = [f.o[i] + u * f.r[i][0] for i in range(3)]
            for k in range(3):
                lo[k] = min(lo[k], c[k] - r)
                hi[k] = max(hi[k], c[k] + r)
    return lo, hi


AREAS = {"bars": 1.75, "resonators": 2.0, "keyboard-frame": 1.2, "stand": 0.75, "castors": 0.11, "mallets": 0.03}
AREAS_FILE = os.path.join(HERE, "author-areas.json")


def shift_lateral(asset, dz):
    """↔️ Moves the whole asset laterally by `dz` so its lateral extent (instrument plus the mallet handles on the player side) is centred on the symmetry plane: the root bone, every front x, every plan y and every lateral extrusion span; bone-local data stays."""
    SHIFT[0] = dz
    asset["rig"]["bones"][0]["z"] = r6(dz)
    for rep in asset["representations"].values():
        for view in ("front", "plan"):
            for s in rep["geometry"][view]:
                if s.get("solid", {}).get("kind") == "extruded" and s["solid"]["sweep"] == "w":
                    s["solid"]["z0"] = r6(s["solid"]["z0"] + dz)
                    s["solid"]["z1"] = r6(s["solid"]["z1"] + dz)
                    continue
                if s.get("solid"):
                    continue
                if s["kind"] == "poly":
                    s["points"] = [[a + (dz if view == "front" else 0), b + (dz if view == "plan" else 0)] for a, b in s["points"]]
                    s["points"] = [[r6(a), r6(b)] for a, b in s["points"]]
                elif s["kind"] == "ellipse":
                    if view == "front":
                        s["cx"] = r6(s["cx"] + dz)
                    else:
                        s["cy"] = r6(s["cy"] + dz)
        for view in ("elevation",):
            for s in rep["geometry"][view]:
                if s.get("solid", {}).get("kind") == "extruded" and s["solid"]["sweep"] == "w":
                    s["solid"]["z0"] = r6(s["solid"]["z0"] + dz)
                    s["solid"]["z1"] = r6(s["solid"]["z1"] + dz)


SHIFT = [0.0]


def main():
    """🚀 Builds and writes the asset document."""
    if os.path.exists(AREAS_FILE):
        AREAS.update(json.load(open(AREAS_FILE)))
    total, _ = compute_masses()
    sections()
    for b in BONES:
        BONE_BY_ID[b["id"]] = b
    reps, vols = {}, {}
    lo = hi = None
    for scale in (500, 200, 100, 50):
        rep, parts = representation(scale)
        reps[str(scale)] = rep
        body, therm, struct = solids_of(scale, parts)
        vols[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": therm, "structure": struct, "decisions": DECISIONS_3D[scale]}
        if scale == 50:
            lo, hi = bbox_of_rest(parts)
    dims = {"heightM": r6(round(hi[1], 4)), "widthM": r6(round(hi[0] - lo[0], 4)), "depthM": r6(round(hi[2] - lo[2], 4)), "massKg": round(total, 2)}
    P, raised = poses()
    asset = {
        "id": "used-xylophone", "category": "objects", "title": t("Used Xylophone", "Gebrauchtes Xylophon"),
        "summary": t(f"A used 3.5-octave concert xylophone (written F3 to C7, 44 Honduras rosewood bars 40 x 22 mm on felted beech rails over tuned aluminium resonator tubes) on a black steel frame with four 75 mm swivel castors, {dims['widthM']:.2f} m long, {dims['heightM']:.2f} m high and 0.79 m deep at the bass end, {dims['massKg']:.0f} kg: stored braked in the dealer's depot with the mallets lying across the bars, rolled on its castors and played standing, with chipped bar ends, worn playing spots, a padauk replacement bar, a dented tube and a scuffed stand.", f"Ein gebrauchtes Konzertxylophon über 3,5 Oktaven (notiert F3 bis C7, 44 Platten aus Honduras-Palisander 40 x 22 mm auf befilzten Buchenleisten über gestimmten Aluminium-Resonatorrohren) auf einem schwarzen Stahlgestell mit vier 75-mm-Lenkrollen, {dims['widthM']:.2f} m lang, {dims['heightM']:.2f} m hoch und am Bassende 0,79 m tief, {dims['massKg']:.0f} kg: gebremst im Lager des Händlers abgestellt, die Schlägel quer über den Platten, auf seinen Rollen gerollt und im Stehen gespielt, mit ausgebrochenen Plattenenden, abgespielten Anschlagstellen, einer Ersatzplatte aus Padouk, einem eingedellten Rohr und einem verschrammten Gestell.".replace(".", ",", 0)),
        "tags": ["used", "instrument", "music", "second-hand", "percussion", "xylophone", "idiophone", "mallet-percussion", "rosewood", "resonators", "castors", "concert"],
        "provenance": {"author": "Semio staffage content (used-instruments wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t(
            "Hornbostel-Sachs 111.212 (sets of percussion sticks). Asset frame: x along the keyboard from the bass (-x) to the treble (+x), the player stands on the asset's right (-z) in front of the naturals, so the side elevation is the player's view. Bars: written range F3 to C7 (sounding an octave higher), 26 naturals at a pitch of 52 mm and 18 accidentals between them, 40 mm wide, 22 mm thick, lengths falling linearly from 380 mm (F3) to 190 mm (C7) so the rails along the nodes (0.224 L from both ends) are straight; naturals top at 0.870 m, accidentals raised to 0.890 m. Resonators: closed tubes of 40 x 1 mm aluminium, quarter-wave length 343/(4 f) - 0.61 r of the sounding pitch (233 mm at F3), at least 100 mm with stoppers above G4. Frame: black steel stand of a 30 x 30 x 2 mm stretcher, 30 x 30 x 2 mm foot bars and four 28 x 1.5 mm legs, beech cheeks and rails, four 75 mm rubber castors trailing 30 mm, brakes on the bass pair. Mallets 0.38 m (9 mm rattan, 30 mm phenolic head). Mass from the parts. Traces of use: chips at the struck ends of seven bars, worn lacquer at the playing spots, the D5 bar replaced in padauk, the A3 tube dented, the inventory label on the treble cheek, scuffed powder coat; it stays fully playable.",
            "Hornbostel-Sachs 111.212 (Aufschlagstabspiele). Asset-Rahmen: x entlang der Klaviatur vom Bass (-x) zum Diskant (+x), der Spieler steht rechts des Assets (-z) vor den Stammtönen, die Seitenansicht ist also die Sicht des Spielers. Platten: notierter Umfang F3 bis C7 (eine Oktave höher klingend), 26 Stammtöne im Abstand von 52 mm und 18 Halbtöne dazwischen, 40 mm breit, 22 mm dick, Längen linear fallend von 380 mm (F3) auf 190 mm (C7), so dass die Leisten auf den Knotenlinien (0,224 L von beiden Enden) gerade sind; Stammtöne oben auf 0,870 m, Halbtöne erhöht auf 0,890 m. Resonatoren: geschlossene Rohre aus Aluminium 40 x 1 mm, Viertelwellenlänge 343/(4 f) - 0,61 r der klingenden Tonhöhe (233 mm bei F3), mindestens 100 mm mit Stopfen oberhalb G4. Gestell: schwarzes Stahlgestell aus Längsrohr 30 x 30 x 2 mm, Fußrohren 30 x 30 x 2 mm und vier Beinen 28 x 1,5 mm, Wangen und Leisten aus Buche, vier 75-mm-Gummirollen mit 30 mm Nachlauf, Bremsen am Basspaar. Schlägel 0,38 m (Rattan 9 mm, Phenolkopf 30 mm). Masse aus den Teilen. Gebrauchsspuren: Ausbrüche an den Anschlagenden von sieben Platten, abgespielter Lack an den Anschlagstellen, die Platte D5 in Padouk ersetzt, das Rohr A3 eingedellt, das Inventaretikett an der Diskantwange, verschrammte Pulverbeschichtung; es bleibt voll spielbar.")},
        "dimensions": dims, "materials": MATERIALS, "rig": {"kind": "mechanical", "root": "stand", "bones": BONES}, "volumes": vols,
        "anchors": anchors(), "representations": reps, "symbols": [], "poses": P, "clips": clips(raised),
        "thermal": thermal(AREAS), "structure": STRUCTURE,
        "presentation": {"defaultPose": "stored", "defaultClip": "mallet-strokes", "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    shift_lateral(asset, -(lo[2] + hi[2]) / 2)
    asset["representations"] = {k: asset["representations"][k] for k in ("50", "100", "200", "500")}
    asset["volumes"] = {k: asset["volumes"][k] for k in ("50", "100", "200", "500")}
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(OUT, "w") as fh:
        json.dump(doc, fh, ensure_ascii=False, separators=(",", ":"))
    print(f"[DEBUG] wrote {OUT}: dims {dims}, bones {len(BONES)}, bbox {[round(v, 4) for v in lo]} {[round(v, 4) for v in hi]}, lateral shift {SHIFT[0]:.4f}")
    for k, v in vols.items():
        print(f"[DEBUG] 1:{k} body {len(v['body'])} thermal {len(v['thermal'])} structure {len(v['structure'])} shapes {sum(len(x) for x in reps[k]['geometry'].values())}")
    for mid, m in MALLETS.items():
        print(f"[DEBUG] {mid}: beta {m['beta']:.3f} grip ({m['gy']:.4f}, {m['gz']:.4f}) strike {m['strike_offset']:.3f} raised {m['raised_offset']:.3f}")


if __name__ == "__main__":
    main()
