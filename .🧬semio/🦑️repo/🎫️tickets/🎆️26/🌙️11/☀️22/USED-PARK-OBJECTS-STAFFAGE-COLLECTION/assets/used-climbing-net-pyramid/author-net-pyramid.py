"""🕸️ Authoritative generator of the used rope climbing pyramid (`used-climbing-net-pyramid`).

Every number of the asset is stated here by hand: the frustum net of steel-reinforced ropes on its central mast, the head, the cross
clamps, the four ground anchors with their EPDM hoods, the traces of use, the rig, poses, clips, thermal and structural models and the
individually authored 2D drawings and 3D solids of the four preset scales. Run `python3 author-net-pyramid.py` to rewrite `🔣️.json`.

@see ../../brief.md — the brief of the wave
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — the asset contract
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")

B, H, YB, YH = 3.0, 0.105, 0.11, 3.85
RINGS = [0.55, 1.05, 1.55, 2.05, 2.55, 3.05]
SHORT_TOP = 2.05
RR, RC = 0.008, 0.0038
CLAMP_R = 0.02
MAST_R, MAST_TOP, CAP_TOP, RUST_TOP = 0.06985, 3.97, 4.0, 0.15
HEAD_Y0, HEAD_Y1, HEAD_RI, HEAD_RO = 3.79, 3.91, 0.0700, 0.16
PLATE, PLATE_T = 0.25, 0.012
HOOD_R, HOOD_H, HOOD_W = 0.10, 0.13, 0.006
EYE_R, EYE_T, EYE_Y = 0.022, 0.009, 0.113
ROD_R = 0.012
TENSION_M = 0.025
SAG_REST, SAG_USE, SAG_BOUNCE = 4.0, 15.0, 11.0
HOOD_TIP = 95.0
CORNERS = {"fl": (1, 1), "bl": (-1, 1), "br": (-1, -1), "fr": (1, -1)}
FACES = {
    "front": {"n": (1, 0, 0), "l": (0, 0, 1), "corner": "fl"},
    "left": {"n": (0, 0, 1), "l": (-1, 0, 0), "corner": "bl"},
    "back": {"n": (-1, 0, 0), "l": (0, 0, -1), "corner": "br"},
    "right": {"n": (0, 0, -1), "l": (1, 0, 0), "corner": "fr"},
}
RHO_ROPE, RHO_STEEL, RHO_AL, RHO_EPDM = 1640.0, 7850.0, 2700.0, 1150.0
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def T(en, de):
    """🌐️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """✂️ Rounds a number to micrometres for a compact document."""
    return round(float(v), 6)


def add(a, b):
    """➕️ Vector sum."""
    return tuple(x + y for x, y in zip(a, b))


def sub(a, b):
    """➖️ Vector difference."""
    return tuple(x - y for x, y in zip(a, b))


def mul(a, s):
    """✖️ Vector times scalar."""
    return tuple(x * s for x in a)


def dot(a, b):
    """⚫️ Dot product."""
    return sum(x * y for x, y in zip(a, b))


def cross(a, b):
    """❎️ Cross product."""
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def norm(a):
    """📏️ Euclidean length."""
    return math.sqrt(dot(a, a))


def unit(a):
    """🧭️ Unit vector."""
    n = norm(a)
    return tuple(x / n for x in a)


def matmul(a, b):
    """🔢️ 3x3 matrix product."""
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def mat_vec(m, v):
    """🎯️ Matrix times vector."""
    return tuple(sum(m[i][k] * v[k] for k in range(3)) for i in range(3))


def transpose(m):
    """🔄️ Matrix transpose."""
    return [[m[j][i] for j in range(3)] for i in range(3)]


def rz(deg):
    """🌀️ Rotation about w (joint axis), counter-clockwise positive."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def ry(deg):
    """🪃️ Right-handed rotation about v."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[c, 0, s], [0, 1, 0], [-s, 0, c]]


def rx(deg):
    """🧲️ Right-handed rotation about u."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [[1, 0, 0], [0, c, -s], [0, s, c]]


def rest_rotation(angle, abduction, twist):
    """🦴️ The rest rotation of a bone: angle about w, abduction (u towards w) about v, twist (v towards -w) about u."""
    return matmul(matmul(rz(angle), ry(-abduction)), rx(-twist))


def decompose(m):
    """🧮️ Angle, abduction and twist of a relative rotation built by `rest_rotation`."""
    beta = math.degrees(math.asin(max(-1.0, min(1.0, -m[2][0]))))
    angle = math.degrees(math.atan2(m[1][0], m[0][0]))
    gamma = math.degrees(math.atan2(m[2][1], m[2][2]))
    return angle, -beta, -gamma


class Rig:
    """🌳️ The bone tree with world rest frames for converting world points to bone-local coordinates."""

    def __init__(self):
        self.bones = []
        self.frames = {}

    def add(self, bone_id, parent, title, origin, u, v, length, joint, mass=None, section=None):
        """🦴️ Adds a bone by its world rest origin and axes; stores the parent-relative placement the schema wants."""
        u = unit(u)
        v = unit(sub(v, mul(u, dot(v, u))))
        w = cross(u, v)
        r_world = [[u[0], v[0], w[0]], [u[1], v[1], w[1]], [u[2], v[2], w[2]]]
        if parent is None:
            rel, offset = r_world, origin
        else:
            rp, op = self.frames[parent]
            rel = matmul(transpose(rp), r_world)
            offset = mat_vec(transpose(rp), sub(origin, op))
        angle, abduction, twist = decompose(rel)
        check = rest_rotation(angle, abduction, twist)
        assert all(abs(check[i][j] - rel[i][j]) < 1e-9 for i in range(3) for j in range(3)), bone_id
        bone = {"id": bone_id, "parent": parent, "title": title, "x": r6(offset[0]), "y": r6(offset[1])}
        if abs(offset[2]) > 1e-9:
            bone["z"] = r6(offset[2])
        bone["angleDeg"] = r6(angle)
        if abs(abduction) > 1e-9:
            bone["abductionDeg"] = r6(abduction)
        if abs(twist) > 1e-9:
            bone["twistDeg"] = r6(twist)
        bone["lengthM"] = r6(length)
        bone["joint"] = joint
        if mass is not None:
            bone["mass"] = mass
        if section is not None:
            bone["section"] = section
        self.bones.append(bone)
        self.frames[bone_id] = (r_world, origin)

    def local(self, bone_id, p):
        """📍️ A world point in the rest frame of a bone (u, v, w)."""
        r, o = self.frames[bone_id]
        return mat_vec(transpose(r), sub(p, o))

    def dir(self, bone_id, d):
        """➡️ A world direction in the rest frame of a bone."""
        r, _ = self.frames[bone_id]
        return mat_vec(transpose(r), d)

    def elev(self, bone_id, px, py):
        """✏️ A side-elevation paper point (world x, y) as an elevation shape point in the bone's own (u, v) plane."""
        r, o = self.frames[bone_id]
        ux, uy, vx, vy = r[0][0], r[1][0], r[0][1], r[1][1]
        det = ux * vy - vx * uy
        dx, dy = px - o[0], py - o[1]
        return [r6((dx * vy - vx * dy) / det), r6((ux * dy - uy * dx) / det)]


def s_at(y):
    """📐️ Half width of the net frustum at height y."""
    return B - (B - H) * (y - YB) / (YH - YB)


def face_point(face, y, t):
    """🕸️ The world point of a face at height y and lateral fraction t (-1 .. 1)."""
    f = FACES[face]
    s = s_at(y)
    return add(add(mul(f["n"], s), mul(f["l"], s * t)), (0, y, 0))


def radial_ks(y):
    """🔢️ The radial indices that pass height y (odd radials end at the short top ring)."""
    return [k for k in range(11) if k % 2 == 0 or y <= SHORT_TOP + 1e-9]


def tk(k):
    """📏️ Lateral fraction of radial k."""
    return -1 + 0.2 * k


SAG_FACE, SAG_K = "right", 5


def sag_nodes():
    """🪢️ The two clamp nodes on ring 1 of the right face between which the sagging rope segment runs."""
    return face_point(SAG_FACE, YB, tk(SAG_K)), face_point(SAG_FACE, YB, tk(SAG_K + 1))


def build_rig():
    """🦴️ The rig: ground, mast, head, four faces hanging from the head, anchors with tension eyes and hoods, the sagging rope pair."""
    rig = Rig()
    fixed = {"type": "fixed"}
    rig.add("ground", None, T("Ground frame", "Bezugsrahmen"), (0, 0, 0), (1, 0, 0), (0, 1, 0), 0.1, fixed)
    mast_area = math.pi / 4 * (0.1397 ** 2 - 0.1297 ** 2)
    mast_i = math.pi / 64 * (0.1397 ** 4 - 0.1297 ** 4)
    rig.add("mast", "ground", T("Central mast", "Mittelmast"), (0, 0, 0), (0, 1, 0), (-1, 0, 0), MAST_TOP, fixed,
            section={"areaM2": r6(mast_area), "secondMomentM4": 4.805e-06, "extremeFibreM": 0.06985, "material": "steel-galvanised-weathered",
                     "title": T("Mast tube o139.7 x 5.0 (EN 10219-2)", "Mastrohr o139,7 x 5,0 (EN 10219-2)")})
    head_area = math.pi / 4 * ((2 * HEAD_RO) ** 2 - (2 * HEAD_RI) ** 2)
    head_i = math.pi / 64 * ((2 * HEAD_RO) ** 4 - (2 * HEAD_RI) ** 4)
    rig.add("head", "mast", T("Net head on the mast", "Netzkopf am Mast"), (0, HEAD_Y0, 0), (1, 0, 0), (0, 1, 0), 0.16, fixed,
            section={"areaM2": r6(head_area), "secondMomentM4": r6(head_i), "extremeFibreM": HEAD_RO, "material": "aluminium-cast-weathered",
                     "title": T("Cast aluminium head ring o320 / o140 x 120", "Netzkopfring Aluminiumguss o320 / o140 x 120")})
    for face, f in FACES.items():
        top = add(mul(f["n"], H), (0, YH, 0))
        base = add(mul(f["n"], B), (0, YB, 0))
        u = sub(base, top)
        v = f["l"] if face in ("left", "right") else cross((0, 0, 1), unit(u))
        rig.add(f"net-{face}", "head", T(f"Net face, {face}", {"front": "Netzfläche vorn", "left": "Netzfläche links", "back": "Netzfläche hinten", "right": "Netzfläche rechts"}[face]),
                top, u, v, norm(u), fixed,
                section={"areaM2": 4.5e-04, "secondMomentM4": 1.6e-09, "extremeFibreM": RR, "material": "steel-wire-rope",
                         "title": T("Ten steel-wire cores of the radial ropes, 45 mm2 each", "Zehn Stahldrahtseelen der Radialseile, je 45 mm2")})
    for c, (sx, sz) in CORNERS.items():
        face = next(k for k, f in FACES.items() if f["corner"] == c)
        o = (sx * B, 0, sz * B)
        rig.add(f"anchor-{c}", f"net-{face}", T(f"Ground anchor {c}", f"Bodenanker {c}"), o, (1, 0, 0), (0, 1, 0), 0.1, fixed)
        rig.add(f"eye-{c}", f"anchor-{c}", T(f"Tension eye bolt {c}", f"Spannösenschraube {c}"), add(o, (0, PLATE_T, 0)), (0, 1, 0), (-1, 0, 0), 0.12,
                {"type": "prismatic", "axisDeg": 90, "minM": -0.04, "maxM": 0})
        hinge = (sx * (B + HOOD_R), PLATE_T, sz * B)
        rig.add(f"hood-{c}", f"anchor-{c}", T(f"EPDM hood {c} (tips over its outer foot)", f"EPDM-Haube {c} (kippt über ihren äußeren Fuß)"), hinge, (-sx, 0, 0), (0, 1, 0), HOOD_H,
                {"type": "revolute", "minDeg": 0, "maxDeg": 100})
    a, b = sag_nodes()
    half = norm(sub(b, a)) / 2
    over = half / math.cos(math.radians(SAG_USE))
    rig.add("sag-a", "net-right", T("Slack rope segment, first half", "Durchhängender Seilabschnitt, erste Hälfte"), a, (1, 0, 0), (0, 1, 0), over,
            {"type": "revolute", "minDeg": -25, "maxDeg": 0},
            section={"areaM2": 4.5e-05, "secondMomentM4": 1.6e-10, "extremeFibreM": RR, "material": "steel-wire-rope", "title": T("Steel-wire core of one rope, 45 mm2", "Stahldrahtseele eines Seils, 45 mm2")})
    rig.add("sag-b", "net-right", T("Slack rope segment, second half", "Durchhängender Seilabschnitt, zweite Hälfte"), b, (-1, 0, 0), (0, -1, 0), over,
            {"type": "revolute", "minDeg": 0, "maxDeg": 25},
            section={"areaM2": 4.5e-05, "secondMomentM4": 1.6e-10, "extremeFibreM": RR, "material": "steel-wire-rope", "title": T("Steel-wire core of one rope, 45 mm2", "Stahldrahtseele eines Seils, 45 mm2")})
    return rig


def ropes():
    """🪢️ Every rope of the net as (id, bone, p0, p1, material, region, kind)."""
    out = []
    a, b = sag_nodes()
    for face in FACES:
        bone = f"net-{face}"
        for k in range(1, 11):
            if k == 10:
                rid, kind = f"edge-{FACES[face]['corner']}", "edge"
            else:
                rid, kind = f"radial-{face}-{k}", "radial"
            top = YH if k % 2 == 0 else SHORT_TOP
            p0, p1 = face_point(face, YB, tk(k)), face_point(face, top, tk(k))
            new = face == "right" and k == 4
            out.append((rid, bone, p0, p1, "rope-cover-new" if new else "rope-cover-weathered", "rope-new" if new else "rope-cover", kind))
        if face == SAG_FACE:
            out.append((f"border-{face}-a", bone, face_point(face, YB, -1), a, "rope-cover-weathered", "rope-cover", "border"))
            out.append((f"border-{face}-b", bone, b, face_point(face, YB, 1), "rope-cover-weathered", "rope-cover", "border"))
        else:
            out.append((f"border-{face}", bone, face_point(face, YB, -1), face_point(face, YB, 1), "rope-cover-weathered", "rope-cover", "border"))
        for i, y in enumerate(RINGS):
            out.append((f"ring-{face}-{i + 1}", bone, face_point(face, y, -1), face_point(face, y, 1), "rope-cover-weathered", "rope-cover", "ring"))
    half = norm(sub(b, a)) / 2
    over = half / math.cos(math.radians(SAG_USE))
    out.append(("sag-rope-a", "sag-a", a, add(a, (over, 0, 0)), "rope-cover-weathered", "rope-cover", "sag"))
    out.append(("sag-rope-b", "sag-b", b, add(b, (-over, 0, 0)), "rope-cover-weathered", "rope-cover", "sag"))
    return out


def clamp_nodes():
    """🔩️ Every cross clamp: (bone, world point); each face carries its ring nodes from t > -1 to the owned edge t = +1 and its border nodes."""
    out = []
    for face in FACES:
        for y in RINGS:
            for k in radial_ks(y):
                if k == 0:
                    continue
                out.append((f"net-{face}", face_point(face, y, tk(k)), f"{face}-{y:.2f}-{k}"))
        for k in range(1, 10):
            out.append((f"net-{face}", face_point(face, YB, tk(k)), f"{face}-b-{k}"))
    return out


FRAYS = [("front", 0.1, -0.35), ("front", 0.55, 0.3), ("right", 0.11, 0.25), ("right", 0.55, -0.45), ("left", 0.11, -0.1), ("back", 0.11, 0.45), ("back", 0.55, -0.2)]


def rope_len(p0, p1):
    """📏️ Length of a straight rope."""
    return norm(sub(p1, p0))


def capsule_volume(r, length):
    """💊️ Volume of a capsule."""
    return math.pi * r * r * length + 4 / 3 * math.pi * r ** 3


def build_masses(rig):
    """⚖️ Bone masses from section times density of the 1:50 parts, with their local centres."""
    acc = {}

    def put(bone, kg, p):
        m, sx, sy, sz = acc.get(bone, (0, 0, 0, 0))
        acc[bone] = (m + kg, sx + kg * p[0], sy + kg * p[1], sz + kg * p[2])

    for rid, bone, p0, p1, *_ in ropes():
        put(bone, RHO_ROPE * math.pi * RR * RR * rope_len(p0, p1), mul(add(p0, p1), 0.5))
    for bone, p, _ in clamp_nodes():
        put(bone, RHO_AL * 4 / 3 * math.pi * CLAMP_R ** 3, p)
    mast_v = math.pi * MAST_R ** 2 * MAST_TOP
    tube_v = math.pi / 4 * (0.1397 ** 2 - 0.1297 ** 2) * MAST_TOP + math.pi * 0.0745 ** 2 * 0.006
    put("mast", RHO_STEEL * tube_v, (0, MAST_TOP / 2, 0))
    head_v = math.pi * (HEAD_RO ** 2 - HEAD_RI ** 2) * (HEAD_Y1 - HEAD_Y0)
    put("head", RHO_AL * head_v, (0, (HEAD_Y0 + HEAD_Y1) / 2, 0))
    for c, (sx, sz) in CORNERS.items():
        o = (sx * B, 0, sz * B)
        put(f"anchor-{c}", RHO_STEEL * (PLATE * PLATE * PLATE_T + math.pi * 0.021 ** 2 * 0.02), add(o, (0, 0.008, 0)))
        rod = math.pi * ROD_R ** 2 * (EYE_Y - EYE_R - 0.03)
        eye = 2 * math.pi * EYE_R * math.pi * EYE_T ** 2
        put(f"eye-{c}", RHO_STEEL * (rod + eye), add(o, (0, 0.08, 0)))
        hood = math.pi * (HOOD_R ** 2 - (HOOD_R - HOOD_W) ** 2) * HOOD_H + math.pi * HOOD_R ** 2 * 0.008
        put(f"hood-{c}", RHO_EPDM * hood, add(o, (0, PLATE_T + HOOD_H * 0.6, 0)))
    masses = {}
    for bone, (m, sx, sy, sz) in acc.items():
        com = rig.local(bone, (sx / m, sy / m, sz / m))
        masses[bone] = (m, com)
    return masses


def mass_entry(m, com, ref):
    """⚖️ A segment mass record."""
    return {"kg": r6(round(m, 3)), "comX": r6(com[0]), "comY": r6(com[1]), "distribution": "point", "source": "literature", "reference": ref}


MASS_REF = {"mast": "en-10219-2", "head": "en-1706", "sag-a": "rope-16-steel-core", "sag-b": "rope-16-steel-core"}
MASS_REF.update({f"anchor-{c}": "en-10025-2" for c in CORNERS})
MASS_REF.update({f"eye-{c}": "en-10025-2" for c in CORNERS})
MASS_REF.update({f"hood-{c}": "iso-10456-2007" for c in CORNERS})


def materials():
    """🧱️ The weathered materials of the pyramid with their thermal and mechanical data."""
    return [
        {"id": "rope-cover-weathered", "title": T("Steel-reinforced rope, weathered polyamide cover (faded red)", "Stahlverstärktes Seil, verwitterter Polyamidmantel (ausgeblichenes Rot)"), "emissivity": 0.9, "solarAbsorptivity": 0.55,
         "densityKgM3": 1640, "conductivityWmK": 0.5, "specificHeatJKgK": 900, "source": "authored-estimate", "reference": "rope-16-steel-core"},
        {"id": "rope-cover-new", "title": T("Steel-reinforced rope, fresh polyamide cover (saturated red, replaced)", "Stahlverstärktes Seil, frischer Polyamidmantel (sattes Rot, ersetzt)"), "emissivity": 0.9, "solarAbsorptivity": 0.68,
         "densityKgM3": 1640, "conductivityWmK": 0.5, "specificHeatJKgK": 900, "source": "authored-estimate", "reference": "rope-16-steel-core"},
        {"id": "steel-wire-rope", "title": T("Steel-wire rope core", "Stahldrahtseele"), "emissivity": 0.3, "densityKgM3": 7850, "conductivityWmK": 45, "specificHeatJKgK": 460,
         "elasticModulusPa": 100000000000, "strengthPa": 1770000000, "source": "literature", "reference": "en-12385-4"},
        {"id": "steel-galvanised-weathered", "title": T("Hot-dip galvanised steel, weathered grey zinc patina", "Feuerverzinkter Stahl, verwitterte graue Zinkpatina"), "emissivity": 0.28, "solarAbsorptivity": 0.72,
         "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "incropera-2018"},
        {"id": "steel-rusted", "title": T("Rusted steel at the mast foot (zinc worn through)", "Rostiger Stahl am Mastfuß (Zink durchgescheuert)"), "emissivity": 0.8, "solarAbsorptivity": 0.85,
         "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "source": "literature", "reference": "incropera-2018"},
        {"id": "aluminium-cast-weathered", "title": T("Cast aluminium, weathered dull oxide", "Aluminiumguss, verwittert matt oxidiert"), "emissivity": 0.25, "solarAbsorptivity": 0.5,
         "densityKgM3": 2700, "conductivityWmK": 150, "specificHeatJKgK": 900, "elasticModulusPa": 70000000000, "source": "literature", "reference": "en-1706"},
        {"id": "epdm-weathered", "title": T("EPDM rubber, weathered black", "EPDM-Gummi, verwittert schwarz"), "emissivity": 0.9, "solarAbsorptivity": 0.88,
         "densityKgM3": 1150, "conductivityWmK": 0.25, "specificHeatJKgK": 2000, "source": "literature", "reference": "iso-10456-2007"},
        {"id": "vinyl-film", "title": T("Sticker vinyl film and spray paint", "Aufkleberfolie und Sprühlack"), "emissivity": 0.9, "solarAbsorptivity": 0.6,
         "densityKgM3": 1400, "source": "authored-estimate", "reference": "incropera-2018"},
    ]


REGION_OF = {"rope-cover-weathered": "rope-cover", "rope-cover-new": "rope-new"}


class Scale:
    """🗂️ The solids and drawings of one preset scale as they are authored."""

    def __init__(self, rig, den):
        self.rig = rig
        self.den = den
        self.f = FEATURE[den]
        self.body, self.thermal, self.structure = [], [], []
        self.views = {"elevation": [], "front": [], "plan": []}
        self.ids = {"elevation": set(), "front": set(), "plan": set()}

    def solid(self, s, region=None, texture=False):
        """🧊️ Adds a body solid and, unless it is texture, its thermal reference."""
        s = dict(s)
        if texture:
            s["texture"] = True
        self.body.append(s)
        if region is not None:
            self.thermal.append({"id": f"th-{s['id']}", "of": s["id"], "region": region})
        return s

    def shape(self, view, s):
        """✏️ Adds a 2D shape to one view, with an id unique in that view."""
        base, n = s["id"], 1
        while s["id"] in self.ids[view]:
            n += 1
            s["id"] = f"{base}-{n}"
        self.ids[view].add(s["id"])
        self.views[view].append(s)

    def member_shape(self, sid, bone, p0, p1, d, role, tone, region, material, filled, pen, z):
        """📊️ A rope or hood drawn as a member bar in all three views."""
        a, b = self.rig.local(bone, p0), self.rig.local(bone, p1)
        for view in self.views:
            s = {"kind": "member", "from": {"x": r6(a[0]), "y": r6(a[1]), "z": r6(a[2])}, "to": {"x": r6(b[0]), "y": r6(b[1]), "z": r6(b[2])},
                 "section": {"kind": "circle", "d": d}, "id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
            if region:
                s["region"] = region
            s["material"] = material
            if pen is not None:
                s["penMm"] = pen
            self.shape(view, s)

    def poly_all(self, sid, bone, elev_pts, front_pts, plan_pts, role, tone, region, material, z, filled=True, pen=None, closed=True):
        """🔷️ One part drawn as a polygon in each view: elevation from world (x, y), front (z, y), plan (x, z)."""
        for view, pts in (("elevation", elev_pts), ("front", front_pts), ("plan", plan_pts)):
            if pts is None:
                continue
            if view == "elevation":
                pts = [self.rig.elev(bone, x, y) for x, y in pts]
            else:
                pts = [[r6(x), r6(y)] for x, y in pts]
            s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
            if region:
                s["region"] = region
            if material:
                s["material"] = material
            if pen is not None:
                s["penMm"] = pen
            s.update({"kind": "poly", "points": pts, "closed": closed})
            self.shape(view, s)


def circle_pts(cx, cy, r, n=16):
    """⭕️ A polygon approximation of a circle."""
    return [(cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)) for i in range(n)]


def rect_pts(x0, y0, x1, y1):
    """⬛️ The corners of an axis-parallel rectangle."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def profile_outline(cx, prof):
    """🏺️ The side outline of a vertical revolved profile [(y, r)] about the axis at paper x = cx."""
    right = [(cx + r, y) for y, r in prof]
    left = [(cx - r, y) for y, r in reversed(prof)]
    return right + left


def vertical_revolve(sc, sid, bone, centre, prof, material, role, tone, closed_loop=False, sweep=None):
    """🏺️ A revolve about the world vertical through `centre`, profile [(world y, r)], in the frame of `bone`."""
    origin = sc.rig.local(bone, (centre[0], 0, centre[2]))
    axis = sc.rig.dir(bone, (0, 1, 0))
    pts = [[r6(y), r6(r)] for y, r in prof]
    if closed_loop:
        pts = pts + [pts[0]]
    s = {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": pts,
         "axis": [r6(a) for a in axis], "origin": [r6(a) for a in origin]}
    if sweep is not None:
        s["sweepRad"] = sweep
    return s


def capsule(sc, sid, bone, p0, p1, r, material, role, tone):
    """💊️ A capsule between two world points in the frame of `bone`."""
    a, b = sc.rig.local(bone, p0), sc.rig.local(bone, p1)
    return {"id": sid, "kind": "capsule", "bone": bone, "material": material, "role": role, "tone": tone,
            "p0": [r6(x) for x in a], "p1": [r6(x) for x in b], "r0": r, "r1": r}


def sphere(sc, sid, bone, p, r, material, role, tone):
    """⚪️ A sphere at a world point in the frame of `bone`."""
    c = sc.rig.local(bone, p)
    return {"id": sid, "kind": "ellipsoid", "bone": bone, "material": material, "role": role, "tone": tone, "radii": [r, r, r],
            "transform": {"translation": [r6(x) for x in c]}}


def box_world(sc, sid, bone, centre, size, material, role, tone, corner=None):
    """📦️ A box on a world-aligned bone, centred at a world point, sized along x, y, z."""
    c = sc.rig.local(bone, centre)
    s = {"id": sid, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": [r6(x) for x in size],
         "transform": {"translation": [r6(x) for x in c]}}
    if corner:
        s["cornerRadius"] = corner
    return s


ROPE_STYLE = {50: (True, 0.13), 100: (True, 0.15), 200: (True, 0.18), 500: (True, 0.21)}
MAST_PROFILE = {
    50: [(RUST_TOP, MAST_R), (MAST_TOP, MAST_R), (MAST_TOP, 0.0745), (MAST_TOP + 0.006, 0.0745), (CAP_TOP - 0.012, 0.06), (CAP_TOP, 0.03), (CAP_TOP, 0.0)],
    100: [(RUST_TOP, MAST_R), (MAST_TOP, MAST_R), (MAST_TOP, 0.0745), (CAP_TOP - 0.01, 0.0745), (CAP_TOP, 0.055), (CAP_TOP, 0.0)],
    200: [(RUST_TOP, MAST_R), (MAST_TOP, MAST_R), (MAST_TOP, 0.0745), (CAP_TOP, 0.0745), (CAP_TOP, 0.0)],
    500: [(RUST_TOP, MAST_R), (CAP_TOP, MAST_R), (CAP_TOP, 0.0)],
}


def author_scale(rig, den, masses):
    """🎨️ Builds the body, thermal, structure lists and the three drawings of one scale."""
    sc = Scale(rig, den)
    p = f"s{den}"
    filled, rope_pen = ROPE_STYLE[den]
    for rid, bone, p0, p1, material, region, kind in ropes():
        if den == 500 and (rid.endswith("-1") and rid.startswith("ring-") or (kind == "radial" and int(rid.split("-")[-1]) % 2 == 1)):
            continue
        tone = 0.62 if region == "rope-new" else 0.38
        s = capsule(sc, f"{p}-{rid}", bone, p0, p1, RR, material, "body", tone)
        s["slender"] = True
        sc.solid(s, region)
        sc.member_shape(rid, bone, p0, p1, 2 * RR, "body", tone, region, material, filled, rope_pen, 2.0 if region == "rope-cover" else 2.05)
    mast_prof = MAST_PROFILE[den]
    sc.solid(vertical_revolve(sc, f"{p}-mast", "mast", (0, 0, 0), mast_prof, "steel-galvanised-weathered", "body", 0.5), "mast")
    sc.solid(vertical_revolve(sc, f"{p}-mast-rust", "mast", (0, 0, 0), [(0.0, MAST_R), (RUST_TOP, MAST_R), (RUST_TOP, 0.0)], "steel-rusted", "body", 0.22), "mast-rust")
    mast_out = profile_outline(0, [(RUST_TOP, MAST_R)] + mast_prof[1:-1] + [(CAP_TOP, max(mast_prof[-2][1], 0.0))])
    sc.poly_all("mast", "mast", mast_out, mast_out, circle_pts(0, 0, max(r for _, r in mast_prof), 24), "body", 0.5, "mast", "steel-galvanised-weathered", 1.5)
    rust = rect_pts(-MAST_R, 0, MAST_R, RUST_TOP)
    sc.poly_all("mast-rust", "mast", rust, rust, None, "body", 0.22, "mast-rust", "steel-rusted", 1.6)
    head_prof = [(HEAD_Y0, HEAD_RI), (HEAD_Y0, HEAD_RO - 0.01), (HEAD_Y0 + 0.01, HEAD_RO), (HEAD_Y1 - 0.01, HEAD_RO), (HEAD_Y1, HEAD_RO - 0.01), (HEAD_Y1, HEAD_RI)] if den <= 100 else \
        [(HEAD_Y0, HEAD_RI), (HEAD_Y0, HEAD_RO), (HEAD_Y1, HEAD_RO), (HEAD_Y1, HEAD_RI)]
    head = sc.solid(vertical_revolve(sc, f"{p}-head", "head", (0, 0, 0), head_prof, "aluminium-cast-weathered", "body", 0.6, closed_loop=True), "fittings-aluminium")
    head_out = rect_pts(-HEAD_RO, HEAD_Y0, HEAD_RO, HEAD_Y1)
    sc.poly_all("head", "head", head_out, head_out, circle_pts(0, 0, HEAD_RO, 24), "body", 0.6, "fittings-aluminium", "aluminium-cast-weathered", 2.4)
    if den <= 100:
        for bone, q, nid in clamp_nodes():
            sc.solid(sphere(sc, f"{p}-clamp-{nid.replace('.', '')}", bone, q, CLAMP_R, "aluminium-cast-weathered", "detail", 0.66), "fittings-aluminium")
        seen = {"elevation": set(), "front": set(), "plan": set()}
        for bone, q, nid in clamp_nodes():
            keys = {"elevation": (round(q[0], 3), round(q[1], 3)), "front": (round(q[2], 3), round(q[1], 3)), "plan": (round(q[0], 3), round(q[2], 3))}
            for view, key in keys.items():
                if key in seen[view]:
                    continue
                seen[view].add(key)
                pts = circle_pts(key[0], key[1], CLAMP_R, 10)
                sc.poly_all(f"clamp-{nid.replace('.', '')}", bone, pts if view == "elevation" else None, pts if view == "front" else None, pts if view == "plan" else None,
                            "detail", 0.66, "fittings-aluminium", "aluminium-cast-weathered", 2.2, pen=0.13 if den == 50 else 0.18)
    if den == 50:
        for i, (face, y, t) in enumerate(FRAYS):
            q0, q1 = face_point(face, y if y > 0.2 else YB, t - 0.03), face_point(face, y if y > 0.2 else YB, t + 0.03)
            s = capsule(sc, f"{p}-fray-{i + 1}", f"net-{face}", q0, q1, RR + 0.0045, "rope-cover-weathered", "texture", 0.3)
            sc.solid(s, None, texture=True)
        for i, (y0, y1, ang) in enumerate([(1.12, 1.19, 0.0), (1.34, 1.43, 1.9), (0.62, 0.70, 4.1)]):
            prof = [[r6(y0), MAST_R], [r6(y1), MAST_R], [r6(y1), MAST_R + 0.0007], [r6(y0), MAST_R + 0.0007], [r6(y0), MAST_R]]
            s = {"id": f"{p}-sticker-{i + 1}", "kind": "revolve", "bone": "mast", "material": "vinyl-film", "role": "texture", "tone": 0.75,
                 "profile": prof, "axis": [1, 0, 0], "origin": [0, 0, 0], "sweepRad": 0.7,
                 "transform": {"rotation": [r6(math.sin(ang / 2)), 0, 0, r6(math.cos(ang / 2))]}}
            sc.solid(s, None, texture=True)
        for view, pts in (("elevation", [(-0.05, 1.13), (-0.02, 1.18), (0.01, 1.12), (0.04, 1.18), (0.06, 1.14)]), ("front", [(-0.055, 1.36), (-0.02, 1.42), (0.015, 1.35), (0.05, 1.41)])):
            sc.poly_all("graffiti-tag", "mast", pts if view == "elevation" else None, pts if view == "front" else None, None, "texture", 0.8, None, "vinyl-film", 3.0, filled=False, closed=False)
        for i, (face, y, t) in enumerate(FRAYS):
            yy = y if y > 0.2 else YB
            q0 = face_point(face, yy, t - 0.03)
            q1 = face_point(face, yy, t + 0.03)
            for view in ("elevation", "front", "plan"):
                if view == "elevation":
                    pts = [(q0[0], q0[1] + 0.012), (q0[0] + (q1[0] - q0[0]) * 0.33, q0[1] - 0.012), (q0[0] + (q1[0] - q0[0]) * 0.66, q0[1] + 0.012), (q1[0], q1[1] - 0.012)]
                    if abs(q1[0] - q0[0]) < 0.02:
                        continue
                elif view == "front":
                    if abs(q1[2] - q0[2]) < 0.02:
                        continue
                    pts = [(q0[2], q0[1] + 0.012), (q0[2] + (q1[2] - q0[2]) * 0.33, q0[1] - 0.012), (q0[2] + (q1[2] - q0[2]) * 0.66, q0[1] + 0.012), (q1[2], q1[1] - 0.012)]
                else:
                    continue
                sc.poly_all(f"fray-{i + 1}", f"net-{face}", pts if view == "elevation" else None, pts if view == "front" else None, None, "texture", 0.3, None, "rope-cover-weathered", 3.1, filled=False, closed=False)
    for c, (sx, sz) in CORNERS.items():
        o = (sx * B, 0, sz * B)
        anchor, eye, hood = f"anchor-{c}", f"eye-{c}", f"hood-{c}"
        if den <= 200:
            plate = box_world(sc, f"{p}-plate-{c}", anchor, add(o, (0, PLATE_T / 2, 0)), (PLATE, PLATE_T, PLATE), "steel-galvanised-weathered", "body", 0.45, 0.003 if den == 50 else None)
        else:
            plate = box_world(sc, f"{p}-plate-{c}", anchor, add(o, (0, PLATE_T / 2, 0)), (PLATE, PLATE_T, PLATE), "steel-galvanised-weathered", "body", 0.45)
        if den >= 200:
            plate["sheet"] = True
        sc.solid(plate, "anchor-hardware")
        pr = rect_pts(o[0] - PLATE / 2, 0, o[0] + PLATE / 2, PLATE_T)
        pf = rect_pts(o[2] - PLATE / 2, 0, o[2] + PLATE / 2, PLATE_T)
        pp = rect_pts(o[0] - PLATE / 2, o[2] - PLATE / 2, o[0] + PLATE / 2, o[2] + PLATE / 2)
        sc.poly_all(f"plate-{c}", anchor, pr, pf, pp, "body", 0.45, "anchor-hardware", "steel-galvanised-weathered", 2.6, pen=0.25 if den == 500 else None)
        if den <= 100:
            sc.solid(vertical_revolve(sc, f"{p}-nut-{c}", anchor, o, [(PLATE_T - 0.001, 0.021), (PLATE_T + 0.02, 0.021), (PLATE_T + 0.02, 0.0)], "steel-galvanised-weathered", "detail", 0.4), "anchor-hardware")
            sc.solid(capsule(sc, f"{p}-rod-{c}", eye, add(o, (0, 0.03 + ROD_R, 0)), add(o, (0, EYE_Y - EYE_R, 0)), ROD_R, "steel-galvanised-weathered", "detail", 0.4), "anchor-hardware")
            ec = sc.rig.local(eye, add(o, (0, EYE_Y, 0)))
            ax = sc.rig.dir(eye, (0, 0, 1))
            loop = [[r6(EYE_T * math.cos(2 * math.pi * i / 12)), r6(EYE_R + EYE_T * math.sin(2 * math.pi * i / 12))] for i in range(12)]
            eye_solid = {"id": f"{p}-eye-{c}", "kind": "revolve", "bone": eye, "material": "steel-galvanised-weathered", "role": "detail", "tone": 0.4,
                         "profile": loop + [loop[0]], "axis": [r6(a) for a in ax], "origin": [r6(a) for a in ec]}
            sc.solid(eye_solid, "anchor-hardware")
            er = rect_pts(o[0] - ROD_R, 0.03, o[0] + ROD_R, EYE_Y - EYE_R)
            sc.poly_all(f"rod-{c}", eye, er, rect_pts(o[2] - ROD_R, 0.03, o[2] + ROD_R, EYE_Y - EYE_R), None, "detail", 0.4, "anchor-hardware", "steel-galvanised-weathered", 2.62)
            ring_out = circle_pts(o[0], EYE_Y, EYE_R + EYE_T, 16)
            sc.poly_all(f"eye-{c}", eye, ring_out, rect_pts(o[2] - EYE_T, EYE_Y - EYE_R - EYE_T, o[2] + EYE_T, EYE_Y + EYE_R + EYE_T),
                        rect_pts(o[0] - EYE_R - EYE_T, o[2] - EYE_T, o[0] + EYE_R + EYE_T, o[2] + EYE_T), "detail", 0.4, "anchor-hardware", "steel-galvanised-weathered", 2.63)
        if den <= 100:
            prof = [(PLATE_T, HOOD_R), (PLATE_T + HOOD_H - 0.02, HOOD_R), (PLATE_T + HOOD_H, HOOD_R - 0.02), (PLATE_T + HOOD_H, 0.004),
                    (PLATE_T + HOOD_H - 0.008, 0.004), (PLATE_T + HOOD_H - 0.008, HOOD_R - HOOD_W - 0.014), (PLATE_T + HOOD_H - 0.02, HOOD_R - HOOD_W), (PLATE_T, HOOD_R - HOOD_W)]
            sc.solid(vertical_revolve(sc, f"{p}-hood-{c}", hood, o, prof, "epdm-weathered", "body", 0.12, closed_loop=True), "caps-epdm")
        else:
            sc.solid(vertical_revolve(sc, f"{p}-hood-{c}", hood, o, [(PLATE_T, HOOD_R), (PLATE_T + HOOD_H, HOOD_R), (PLATE_T + HOOD_H, 0.0)], "epdm-weathered", "body", 0.12), "caps-epdm")
        rnd = 0.02 if den <= 100 else 0.0
        outline = [(o[0] - HOOD_R, PLATE_T), (o[0] + HOOD_R, PLATE_T), (o[0] + HOOD_R, PLATE_T + HOOD_H - rnd), (o[0] + HOOD_R - rnd, PLATE_T + HOOD_H),
                   (o[0] - HOOD_R + rnd, PLATE_T + HOOD_H), (o[0] - HOOD_R, PLATE_T + HOOD_H - rnd)]
        if rnd == 0:
            outline = [outline[0], outline[1], outline[2], outline[5]]
        local_pts = [sc.rig.elev(hood, x, y) for x, y in outline]
        for view in ("elevation", "front", "plan"):
            sh = {"id": f"hood-{c}", "bone": hood, "role": "body", "z": 2.8, "filled": True, "tone": 0.12, "region": "caps-epdm", "material": "epdm-weathered",
                  "kind": "poly", "points": local_pts, "closed": True}
            if den == 500:
                sh["penMm"] = 0.25
            if view == "plan":
                sh["solid"] = {"kind": "revolved", "axisBone": hood, "axis": "v", "axisX": r6(-sx * HOOD_R), "axisZ": 0}
            if view == "front":
                sh["solid"] = {"kind": "extruded", "sweep": "w", "z0": r6(o[2] - HOOD_R), "z1": r6(o[2] + HOOD_R)}
            sc.shape(view, sh)
    ground_pen = None
    ge = [(-B - PLATE / 2, 0), (-0.1, 0), (0.08, -0.03), (0.3, -0.04), (0.52, -0.03), (0.7, 0), (B + PLATE / 2, 0)]
    gf = [(-B - PLATE / 2, 0), (0.1, 0), (0.25, -0.03), (0.6, -0.04), (0.85, -0.03), (1.0, 0), (B + PLATE / 2, 0)]
    sc.poly_all("ground-sand", "mast", ge, gf, None, "ground", 0.0, None, None, 0.5, filled=False, closed=False)
    structure(sc, masses)
    return sc


def structure(sc, masses):
    """🏗️ The structural idealisation of one scale: steel cores of the ropes, mast tube, head ring, joints, supports, masses."""
    p, f = f"s{sc.den}", sc.f
    for rid, bone, p0, p1, *_ in ropes():
        s = capsule(sc, f"{p}-core-{rid}", bone, p0, p1, RC, "steel-wire-rope", "body", 0.3)
        s.update({"slender": True, "member": bone, "part": "member"})
        sc.structure.append(s)
    tube = [[0.0, 0.06485], [MAST_TOP, 0.06485], [MAST_TOP, MAST_R], [0.0, MAST_R], [0.0, 0.06485]]
    sc.structure.append({"id": f"{p}-core-mast", "kind": "revolve", "bone": "mast", "material": "steel-galvanised-weathered", "role": "body", "tone": 0.5,
                         "profile": tube, "axis": [1, 0, 0], "origin": [0, 0, 0], "member": "mast", "part": "member"})
    sc.structure.append({"id": f"{p}-core-head", "of": f"{p}-head", "member": "head", "part": "member"})
    j = 1.5 * f
    sc.structure.append({**sphere(sc, f"{p}-joint-mast-foot", "mast", (0, j, 0), j, "steel-galvanised-weathered", "detail", 0.2), "member": "mast", "part": "joint"})
    sc.structure.append({**sphere(sc, f"{p}-joint-head", "head", (0, HEAD_Y0 + 0.06, 0), 0.068 + f / 2, "aluminium-cast-weathered", "detail", 0.2), "member": "head", "part": "joint"})
    for face, ff in FACES.items():
        if sc.den >= 100:
            break
        top = add(mul(ff["n"], H), (0, YH, 0))
        sc.structure.append({**sphere(sc, f"{p}-joint-net-{face}", f"net-{face}", top, 0.02 + j, "steel-wire-rope", "detail", 0.2), "member": f"net-{face}", "part": "joint"})
    a, b = sag_nodes()
    for nm, q in (("sag-a", a), ("sag-b", b)) if sc.den < 500 else ():
        sc.structure.append({**sphere(sc, f"{p}-joint-{nm}", nm, q, f + 0.007, "steel-wire-rope", "detail", 0.2), "member": nm, "part": "joint"})
    pad = 2 * f + 0.02
    sc.structure.append({**box_world(sc, f"{p}-support-mast", "mast", (0, pad / 2, 0), (pad, 0.2, 0.2), "steel-galvanised-weathered", "detail", 0.2),
                         "member": "mast", "part": "support", "anchor": "mast-foot"})
    for c, (sx, sz) in CORNERS.items():
        o = (sx * B, 0, sz * B)
        face = next(k for k, ff in FACES.items() if ff["corner"] == c)
        sc.structure.append({**box_world(sc, f"{p}-support-{c}", f"anchor-{c}", add(o, (0, (YB + RC) / 2, 0)), (0.1 + f, YB + RC, 0.1 + f), "steel-galvanised-weathered", "detail", 0.2),
                             "member": f"net-{face}", "part": "support", "anchor": f"pin-{c}"})
    for bone, (m, com) in masses.items():
        if (sc.den >= 200 and bone.split("-")[0] in ("eye", "hood")) or (sc.den == 500 and bone.startswith("sag")):
            continue
        r_m = f + 0.007
        r, o = sc.rig.frames[bone]
        world = on_rope(bone, add(o, mat_vec(r, com)))
        sc.structure.append({**sphere(sc, f"{p}-mass-{bone}", bone, world, r_m, "steel-galvanised-weathered", "detail", 0.2), "member": bone, "part": "mass"})


def on_rope(bone, q):
    """🎯️ The point of the nearest rope of a net face to its centre of mass (a face is a mesh of air and rope); other bones keep q."""
    if not bone.startswith("net-"):
        return q
    best, best_d = q, 1e9
    for _, b, p0, p1, *_ in ropes():
        if b != bone:
            continue
        d = sub(p1, p0)
        t = max(0.0, min(1.0, dot(sub(q, p0), d) / dot(d, d)))
        c = add(p0, mul(d, t))
        if norm(sub(c, q)) < best_d:
            best, best_d = c, norm(sub(c, q))
    return best


def anchors_list(rig):
    """📍️ The support anchors (mast foot fixed, four pins at the ground anchors) and the load points."""
    out = [{"id": "mast-foot", "bone": "mast", "x": 0, "y": 0, "kind": "support", "title": T("Mast foot, fixed in its concrete footing", "Mastfuß, im Betonfundament eingespannt"),
            "support": "fixed", "footprintLengthM": 0.14, "footprintWidthM": 0.14}]
    for c in CORNERS:
        out.append({"id": f"pin-{c}", "bone": f"anchor-{c}", "x": 0, "y": 0, "kind": "support", "title": T(f"Ground anchor {c}, pinned point foundation", f"Bodenanker {c}, gelenkiges Punktfundament"),
                    "support": "pin", "footprintLengthM": PLATE, "footprintWidthM": PLATE, "frictionCoefficient": 0.6})
    a, b = sag_nodes()
    half = norm(sub(b, a)) / 2
    out.append({"id": "step-point", "bone": "sag-a", "x": r6(half), "y": 0, "kind": "load", "title": T("Foot of the child on the slack rope", "Fuß des Kindes auf dem durchhängenden Seil")})
    return out


def region_binding(sun):
    """🌞️ The region bindings of a pose."""
    return {k: v for k, v in sun.items()}


def thermal_model(areas):
    """🌡️ The passive thermal model: seven weathered surfaces in the summer sun."""
    env = {"airTempC": 32, "meanRadiantTempC": 36, "skyTempC": 21, "groundTempC": 50, "relativeHumidity": 0.4, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 850, "diffuseFraction": 0.12, "pressurePa": 101325}

    def region(rid, title, material, area, length, geometry, sky, layer, cap, extra=None):
        r = {"id": rid, "title": title, "kind": "surface", "model": "passive", "material": material, "areaM2": area, "characteristicLengthM": length, "geometry": geometry,
             "skyViewFactor": sky, "layers": [layer], "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": "iso-10456-2007"}
        if extra:
            r.update(extra)
        return r

    rope_layer = {"id": "rope", "title": T("Rope, 8 mm from cover to steel core", "Seil, 8 mm vom Mantel zur Stahlseele"), "kind": "polymer", "material": "rope-cover-weathered", "thicknessM": RR,
                  "resistanceM2KW": 0.016, "heatCapacityJM2K": 5900, "source": "authored-estimate", "reference": "rope-16-steel-core"}
    new_layer = dict(rope_layer, material="rope-cover-new", title=T("Replaced rope, 8 mm from cover to steel core", "Ersetztes Seil, 8 mm vom Mantel zur Stahlseele"))
    return {
        "environment": env,
        "referenceEnvironment": "summer-hot-sun",
        "passengers": [],
        "operating": [],
        "regions": [
            region("rope-cover", T("Weathered rope covers of the net", "Verwitterte Seilmäntel des Netzes"), "rope-cover-weathered", areas["rope-cover"], 0.016, "cylinder", 0.55, rope_layer, 5900),
            region("rope-new", T("Replaced radial rope, fresh cover", "Ersetztes Radialseil, frischer Mantel"), "rope-cover-new", areas["rope-new"], 0.016, "cylinder", 0.55, new_layer, 5900),
            region("mast", T("Galvanised mast and its cap", "Verzinkter Mast mit Kappe"), "steel-galvanised-weathered", areas["mast"], 0.14, "cylinder", 0.5,
                   {"id": "tube", "title": T("Galvanised tube wall 5 mm", "Verzinkte Rohrwand 5 mm"), "kind": "metal", "material": "steel-galvanised-weathered", "thicknessM": 0.005,
                    "resistanceM2KW": 1e-04, "heatCapacityJM2K": 17660, "source": "literature", "reference": "iso-10456-2007"}, 17660,
                   {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
            region("mast-rust", T("Rusted mast foot where the zinc is worn through", "Rostiger Mastfuß, wo das Zink durchgescheuert ist"), "steel-rusted", areas["mast-rust"], 0.14, "cylinder", 0.45,
                   {"id": "tube", "title": T("Rusted tube wall 5 mm", "Rostige Rohrwand 5 mm"), "kind": "metal", "material": "steel-rusted", "thicknessM": 0.005,
                    "resistanceM2KW": 1e-04, "heatCapacityJM2K": 17660, "source": "literature", "reference": "iso-10456-2007"}, 17660),
            region("fittings-aluminium", T("Cast aluminium head and cross clamps", "Netzkopf und Kreuzklemmen aus Aluminiumguss"), "aluminium-cast-weathered", areas["fittings-aluminium"], 0.04, "sphere", 0.55,
                   {"id": "casting", "title": T("Cast aluminium, solid clamps and head ring", "Aluminiumguss, volle Klemmen und Kopfring"), "kind": "metal", "material": "aluminium-cast-weathered", "thicknessM": 0.0067,
                    "resistanceM2KW": 4e-05, "heatCapacityJM2K": 25600, "source": "authored-estimate", "reference": "en-1706"}, 25600),
            region("anchor-hardware", T("Anchor plates, nuts and tension eye bolts", "Ankerplatten, Muttern und Spannösenschrauben"), "steel-galvanised-weathered", areas["anchor-hardware"], 0.25, "horizontal-plate-up", 0.3,
                   {"id": "plate", "title": T("Galvanised plate 12 mm", "Verzinkte Platte 12 mm"), "kind": "metal", "material": "steel-galvanised-weathered", "thicknessM": 0.012,
                    "resistanceM2KW": 2.4e-04, "heatCapacityJM2K": 21200, "source": "literature", "reference": "iso-10456-2007"}, 21200),
            region("caps-epdm", T("EPDM hoods over the ground anchors", "EPDM-Hauben über den Bodenankern"), "epdm-weathered", areas["caps-epdm"], 0.2, "horizontal-plate-up", 0.8,
                   {"id": "hood", "title": T("EPDM hood wall 6 mm", "EPDM-Haubenwand 6 mm"), "kind": "polymer", "material": "epdm-weathered", "thicknessM": HOOD_W,
                    "resistanceM2KW": 0.024, "heatCapacityJM2K": 13800, "source": "literature", "reference": "iso-10456-2007"}, 13800),
        ],
        "emitters": [],
        "assumptions": [
            T("Summer reference case of the preset summer-hot-sun: 850 W/m2 sun, 32 degC air, sand at 50 degC, 1.5 m/s wind; every surface is passive. The black EPDM hoods (absorptance 0.88) and the rusted mast foot (0.85) are the hottest parts; the weathered zinc of the mast (0.72 against about 0.6 when new, emissivity 0.28 against about 0.07 when new) runs warmer than the faded rope covers, whose high emissivity keeps them near hand temperature.",
              "Sommer-Referenzfall der Vorgabe summer-hot-sun: 850 W/m2 Sonne, 32 degC Luft, Sand bei 50 degC, 1,5 m/s Wind; alle Flächen sind passiv. Die schwarzen EPDM-Hauben (Absorptionsgrad 0,88) und der rostige Mastfuß (0,85) werden am heißesten; das verwitterte Zink des Masts (0,72 statt etwa 0,6 im Neuzustand, Emissionsgrad 0,28 statt etwa 0,07) läuft wärmer als die ausgeblichenen Seilmäntel, deren hoher Emissionsgrad sie nahe Handtemperatur hält."),
            T("The replaced rope has a fresh saturated cover (absorptance 0.68 against 0.55 of the faded ones) and is the warmest rope in the sun. Ropes are a polyamide cover over a steel-wire core; their heat capacity is the volume per surface of a 16 mm rod (4 mm) times the composite 1640 kg/m3 and 900 J/kgK.",
              "Das ersetzte Seil hat einen frischen, satten Mantel (Absorptionsgrad 0,68 statt 0,55 der ausgeblichenen) und ist das wärmste Seil in der Sonne. Die Seile sind ein Polyamidmantel über einer Stahldrahtseele; ihre Wärmekapazität ist das Volumen je Oberfläche eines 16-mm-Stabs (4 mm) mal 1640 kg/m3 und 900 J/kgK des Verbunds."),
            T("No element of the pyramid draws power: the mast carries the explicit passive 0 W rating; the climbing children are loads, not passengers.",
              "Kein Element der Pyramide nimmt Leistung auf: Der Mast trägt die ausdrückliche passive Bemessung 0 W; die kletternden Kinder sind Lasten, keine Mitfahrenden."),
        ],
    }


def sun(pose):
    """☀️ Exposure and sun share of every region in a pose."""
    base = {"rope-cover": {"exposedFraction": 1, "sunlitFraction": 0.5}, "rope-new": {"exposedFraction": 1, "sunlitFraction": 0.5},
            "mast": {"exposedFraction": 1, "sunlitFraction": 0.45}, "mast-rust": {"exposedFraction": 1, "sunlitFraction": 0.4},
            "fittings-aluminium": {"exposedFraction": 1, "sunlitFraction": 0.5},
            "anchor-hardware": {"exposedFraction": 0.3, "sunlitFraction": 0.1, "contact": {"fraction": 0.4, "tempC": 45}},
            "caps-epdm": {"exposedFraction": 1, "sunlitFraction": 0.6}}
    if pose == "maintenance":
        base["anchor-hardware"] = {"exposedFraction": 0.6, "sunlitFraction": 0.45, "contact": {"fraction": 0.4, "tempC": 45}}
        base["caps-epdm"] = {"exposedFraction": 0.9, "sunlitFraction": 0.5, "contact": {"fraction": 0.1, "tempC": 50}}
    if pose == "in-use":
        base["rope-cover"] = {"exposedFraction": 0.98, "sunlitFraction": 0.45, "contact": {"fraction": 0.02, "tempC": 33}}
        base["rope-new"] = {"exposedFraction": 1, "sunlitFraction": 0.45}
        base["mast"] = {"exposedFraction": 1, "sunlitFraction": 0.4}
    return {"regions": base, "emitters": {}}


SUPPORTS = ["mast-foot", "pin-fl", "pin-bl", "pin-br", "pin-fr"]


def poses(rig):
    """📥️ In service (slack segment), maintenance (hoods off, eyes pulled down, net taut) and in use (two climbing children)."""
    hoods = {f"hood-{c}": {"angleDeg": HOOD_TIP} for c in CORNERS}
    eyes = {f"eye-{c}": {"dy": -TENSION_M} for c in CORNERS}
    sag = lambda d: {"sag-a": {"angleDeg": -d}, "sag-b": {"angleDeg": d}}
    climb = rig.local("net-front", face_point("front", RINGS[2], 0.2))
    return [
        {"id": "in-service", "title": T("In service, net crept slack", "Im Betrieb, Netz nachgelängt"),
         "description": T("Standing in the sand on its mast footing and four anchors; after years of use the ropes have crept and the bottom step rope of the right face hangs 22 mm slack between its clamps.",
                          "Steht im Sand auf seinem Mastfundament und vier Ankern; nach Jahren der Nutzung haben sich die Seile gelängt, und das untere Trittseil der rechten Fläche hängt zwischen seinen Klemmen 22 mm durch."),
         "bones": sag(SAG_REST), "ground": "auto", "contacts": SUPPORTS, "thermal": sun("in-service"), "loads": []},
        {"id": "maintenance", "title": T("Re-tensioning by the depot", "Nachspannen durch den Bauhof"),
         "description": T("The depot crew has tipped the four EPDM hoods aside and drawn every tension eye 25 mm down into its anchor, so the creep is taken up and the step rope runs straight again; one fitter holds the anchor plate down while turning the nut.",
                          "Der Bauhof hat die vier EPDM-Hauben zur Seite gekippt und jede Spannöse 25 mm in ihren Anker gezogen; die Längung ist aufgenommen, das Trittseil läuft wieder gerade, ein Monteur hält beim Drehen der Mutter die Ankerplatte nieder."),
         "bones": {**hoods, **eyes, **sag(0)}, "ground": "auto", "contacts": SUPPORTS, "thermal": sun("maintenance"),
         "loads": [{"id": "fitter-hand", "kind": "equipment", "title": T("Fitter's hand on the anchor (250 N)", "Hand des Monteurs auf dem Anker (250 N)"), "bone": "anchor-fr", "x": 0, "y": PLATE_T,
                    "fxN": 0, "fyN": -250, "source": "authored-estimate"}]},
        {"id": "in-use", "title": T("Two children climbing", "Zwei kletternde Kinder"),
         "description": T("One child (30 kg) steps onto the slack step rope, which sags 80 mm under the dynamic load; a second child (35 kg) stands on the third ring of the front face.",
                          "Ein Kind (30 kg) steigt auf das durchhängende Trittseil, das unter der dynamischen Last 80 mm durchsackt; ein zweites Kind (35 kg) steht auf dem dritten Ring der vorderen Fläche."),
         "bones": sag(SAG_USE), "ground": "auto", "contacts": SUPPORTS, "thermal": sun("in-use"),
         "loads": [{"id": "child-step", "kind": "occupant", "title": T("Child stepping on the rope, 30 kg x 1.8 dynamic", "Kind tritt aufs Seil, 30 kg x 1,8 dynamisch"), "anchor": "step-point",
                    "fxN": 0, "fyN": -529.6, "source": "authored-estimate", "reference": "en-1176-1"},
                   {"id": "child-stand", "kind": "occupant", "title": T("Child standing on the third ring, 35 kg", "Kind steht auf dem dritten Ring, 35 kg"), "bone": "net-front",
                    "x": r6(climb[0]), "y": r6(climb[1]), "z": r6(climb[2]), "fxN": 0, "fyN": -343.2, "source": "authored-estimate", "reference": "en-1176-1"}]},
    ]


def clips():
    """🎞️ The re-tensioning by the depot and a child stepping onto the slack rope and bouncing."""
    hoods = lambda k: {f"hood-{c}": {"angleDeg": HOOD_TIP * k} for c in CORNERS}
    sag = lambda d: {"sag-a": {"angleDeg": -d}, "sag-b": {"angleDeg": d}}
    contacts = lambda d: [{"anchor": a, "fromS": 0, "toS": d} for a in SUPPORTS]
    return [
        {"id": "maintenance", "title": T("Re-tensioning the net", "Netz nachspannen"),
         "description": T("The hoods are tipped aside (2 s), the four eye bolts are drawn down 25 mm one after the other in the real sequence of the depot (crosswise), and the slack step rope straightens.",
                          "Die Hauben werden zur Seite gekippt (2 s), die vier Ösenschrauben werden in der Reihenfolge des Bauhofs über Kreuz nacheinander 25 mm nach unten gezogen, und das durchhängende Trittseil strafft sich."),
         "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"},
                  {"t": 2, "pose": "in-service", "ease": "linear", "bones": {**hoods(1), **sag(SAG_REST)}},
                  {"t": 3.25, "pose": "in-service", "ease": "linear", "bones": {**hoods(1), **sag(3.75), "eye-fl": {"dy": -TENSION_M}}},
                  {"t": 4.5, "pose": "in-service", "ease": "linear", "bones": {**hoods(1), **sag(2.5), "eye-fl": {"dy": -TENSION_M}, "eye-br": {"dy": -TENSION_M}}},
                  {"t": 5.75, "pose": "in-service", "ease": "linear", "bones": {**hoods(1), **sag(1.25), "eye-fl": {"dy": -TENSION_M}, "eye-br": {"dy": -TENSION_M}, "eye-bl": {"dy": -TENSION_M}}},
                  {"t": 7, "pose": "maintenance", "ease": "smooth"},
                  {"t": 8, "pose": "maintenance", "ease": "step"}],
         "contacts": contacts(8),
         "phases": [{"id": "uncover", "fromS": 0, "toS": 2, "title": T("Hoods off", "Hauben ab")},
                    {"id": "tension", "fromS": 2, "toS": 7, "title": T("Drawing the eyes down crosswise", "Ösen über Kreuz nachziehen")},
                    {"id": "check", "fromS": 7, "toS": 8, "title": T("Checking the rope tension", "Seilspannung prüfen")}],
         "events": [{"t": 2, "kind": "motion", "title": T("All hoods tipped aside", "Alle Hauben zur Seite gekippt")},
                    {"t": 7, "kind": "motion", "title": T("Net taut again", "Netz wieder straff")}],
         "tracks": []},
        {"id": "climbing-step", "title": T("Child stepping on the slack rope", "Kind steigt aufs durchhängende Seil"),
         "description": T("A child steps onto the slack step rope, which sags to 15 degrees, rebounds to 11, sags again and is released when the child climbs on (2.4 s, looping).",
                          "Ein Kind steigt auf das durchhängende Trittseil, das auf 15 Grad durchsackt, auf 11 Grad zurückfedert, erneut durchsackt und frei wird, wenn das Kind weiterklettert (2,4 s, wiederkehrend)."),
         "durationS": 2.4, "fps": 30, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"},
                  {"t": 0.6, "pose": "in-use", "ease": "smooth"},
                  {"t": 1.0, "pose": "in-use", "ease": "smooth", "bones": sag(SAG_BOUNCE)},
                  {"t": 1.5, "pose": "in-use", "ease": "smooth"},
                  {"t": 2.4, "pose": "in-service", "ease": "smooth"}],
         "contacts": contacts(2.4),
         "phases": [{"id": "step-on", "fromS": 0, "toS": 0.6, "title": T("Stepping on", "Aufsteigen")},
                    {"id": "bounce", "fromS": 0.6, "toS": 1.5, "title": T("Rope bounces", "Seil federt")},
                    {"id": "climb-on", "fromS": 1.5, "toS": 2.4, "title": T("Climbing on", "Weiterklettern")}],
         "events": [{"t": 0.6, "kind": "load-on", "title": T("Full weight on the rope", "Volles Gewicht auf dem Seil"), "target": "child-step"},
                    {"t": 2.4, "kind": "load-off", "title": T("Rope released", "Seil entlastet"), "target": "child-step"}],
         "tracks": [{"target": "load:child-step", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 0.6, "v": 1}, {"t": 1.0, "v": 0.75}, {"t": 1.5, "v": 1}, {"t": 2.4, "v": 0}]}]},
    ]


def areas_estimate():
    """📐️ Hand calculation of the outer area of every region from the 1:50 parts."""
    rope = sum(math.pi * 2 * RR * rope_len(p0, p1) for _, _, p0, p1, m, reg, _ in ropes() if reg == "rope-cover")
    new = sum(math.pi * 2 * RR * rope_len(p0, p1) for _, _, p0, p1, m, reg, _ in ropes() if reg == "rope-new")
    clamps = len(clamp_nodes()) * 4 * math.pi * CLAMP_R ** 2
    head = 2 * math.pi * HEAD_RO * (HEAD_Y1 - HEAD_Y0) + 2 * math.pi * (HEAD_RO ** 2 - HEAD_RI ** 2)
    mast = 2 * math.pi * MAST_R * (MAST_TOP - RUST_TOP) + 2 * math.pi * 0.0745 * 0.006 + math.pi * 0.0745 ** 2
    rust = 2 * math.pi * MAST_R * RUST_TOP + 2 * math.pi * MAST_R ** 2
    plate = 2 * PLATE * PLATE + 4 * PLATE * PLATE_T - math.pi * (HOOD_R - HOOD_W) ** 2
    hood = 2 * math.pi * HOOD_R * HOOD_H + math.pi * HOOD_R ** 2
    return {"rope-cover": round(rope, 3), "rope-new": round(new, 3), "mast": round(mast, 3), "mast-rust": round(rust, 3),
            "fittings-aluminium": round(clamps + head, 3), "anchor-hardware": round(4 * plate, 3), "caps-epdm": round(4 * hood, 3)}


REFERENCES = [
    {"id": "en-1176-1", "citation": "EN 1176-1:2017 Playground equipment and surfacing - Part 1: General safety requirements and test methods (user loads of Annex A, entrapment openings, free height of fall, impact area, foundations kept below or flush with the playing surface)."},
    {"id": "en-1176-11", "citation": "EN 1176-11:2014 Playground equipment and surfacing - Part 11: Additional specific safety requirements and test methods for spatial networks (free height of fall inside the net, mesh openings, rope ends and anchorages)."},
    {"id": "en-1176-7", "citation": "EN 1176-7:2020 Playground equipment and surfacing - Part 7: Guidance on installation, inspection, maintenance and operation (routine, operational and annual inspection; re-tensioning of ropes and nets, replacement of worn ropes)."},
    {"id": "en-1177", "citation": "EN 1177:2018 Impact attenuating playground surfacing - Methods of test for determination of impact attenuation (sand 0-2 mm, 300 mm loose layer, critical fall height up to 3 m)."},
    {"id": "en-10219-2", "citation": "EN 10219-2:2019 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (o139.7 x 5.0: A = 21.2 cm2, I = 481 cm4, 16.6 kg/m)."},
    {"id": "en-10025-2", "citation": "EN 10025-2:2019 Hot rolled products of structural steels - Part 2: Technical delivery conditions for non-alloy structural steels (S235JR anchor plates and eye bolts, density 7850 kg/m3)."},
    {"id": "en-12385-4", "citation": "EN 12385-4:2002+A1:2008 Steel wire ropes - Safety - Part 4: Stranded ropes for general lifting applications (wire tensile grade 1770 N/mm2, effective rope modulus about 100 GPa)."},
    {"id": "en-1706", "citation": "EN 1706:2020 Aluminium and aluminium alloys - Castings - Chemical composition and mechanical properties (EN AC-AlSi10Mg type for rope clamps and net heads, density 2700 kg/m3)."},
    {"id": "rope-16-steel-core", "citation": "Catalogue-typical data of 16 mm playground combination ropes (six strands, each a galvanised steel-wire core wrapped in polyamide or polyester yarn): about 0.33 kg/m, minimum breaking load about 40 kN, metallic core area about 45 mm2, pressed aluminium cross clamps of about 40 mm; space-net pyramids 4 m high with a central mast on a 6 x 6 m anchor square (authored estimate from makers' data sheets of the type)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values (conductivity, density and specific heat of steel, aluminium, rubber, polymers)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley, Tables A.11 and A.12 (emissivity and solar absorptance of galvanised, oxidised and rusted steel, aluminium, rubber and paints; weathering raises both of a zinc surface)."},
    {"id": "din-18034", "citation": "DIN 18034-1:2020 Spielplätze und Freiräume zum Spielen - Teil 1: Anforderungen für Planung, Bau und Betrieb (municipal playground planning, surfacing and operation in Germany)."},
]


def rep_decisions(den):
    """📝️ The design decisions of the drawing of one scale."""
    d = {
        50: dict(
            silhouette=T("Side elevation 125 x 80 mm seen from the right face: the frustum net of 16 mm ropes from the four ground anchors to the head, the near right face with its fan of radials and six rings in front of the far left face, the front and back faces edge-on along the two slanted edges, the galvanised mast with its rust-brown foot, the cast head and the dome cap, every cross clamp as a 40 mm dot, the anchor plates with their black EPDM hoods; front elevation the same seen from ahead; plan the square of 6.25 m with the converging radials, the rings as nested squares, the head over the mast cap.",
                         "Seitenansicht 125 x 80 mm von der rechten Fläche aus: das Kegelstumpfnetz aus 16-mm-Seilen von den vier Bodenankern zum Netzkopf, die nahe rechte Fläche mit ihrem Fächer aus Radialseilen und sechs Ringen vor der fernen linken Fläche, die vordere und hintere Fläche auf der Kante entlang der beiden schrägen Grate, der verzinkte Mast mit rostbraunem Fuß, der Gusskopf und die Kuppelkappe, jede Kreuzklemme als 40-mm-Punkt, die Ankerplatten mit ihren schwarzen EPDM-Hauben; die Vorderansicht ebenso von vorn; die Draufsicht das Quadrat von 6,25 m mit den zusammenlaufenden Radialseilen, den Ringen als verschachtelte Quadrate, dem Kopf über der Mastkappe."),
            retained=T("Every rope at its true 16 mm, every cross clamp, the short radials ending at the fourth ring, the head ring, the dome cap, the rust band, the anchor plates, nuts, eye bolts (hidden under the hoods at rest, visible when the hoods are tipped) and the hoods; traces of use: frayed covers at the step points, the fresh replaced radial, stickers and a tag on the mast.",
                       "Jedes Seil in seinen wahren 16 mm, jede Kreuzklemme, die kurzen Radialseile bis zum vierten Ring, der Kopfring, die Kuppelkappe, das Rostband, die Ankerplatten, Muttern, Ösenschrauben (in Ruhe unter den Hauben verborgen, sichtbar bei gekippten Hauben) und die Hauben; Gebrauchsspuren: ausgefranste Mäntel an den Trittstellen, das frische ersetzte Radialseil, Aufkleber und ein Tag am Mast."),
            merged=T("Nothing is merged; clamps that fall on one another in a view are drawn once.", "Nichts ist zusammengefasst; Klemmen, die in einer Ansicht aufeinanderfallen, sind einmal gezeichnet."),
            omitted=T("The buried concrete footings (they lie below the 300 mm sand layer, EN 1176-1), the strand lay of the ropes and the clamp screws.", "Die vergrabenen Betonfundamente (sie liegen unter der 300-mm-Sandschicht, EN 1176-1), der Litzenschlag der Seile und die Klemmschrauben."),
            thinElements=T("Ropes are member bars of 16 mm (0.32 mm) with a 0.13 mm outline, so they print at 0.21 mm and more up to 1:75.", "Seile sind Stäbe von 16 mm (0,32 mm) mit 0,13-mm-Kontur und drucken bis 1:75 mit mindestens 0,21 mm."),
            hatching=T("None; the materials read by tone.", "Keine; die Werkstoffe lesen sich über den Tonwert."),
            styles=T("Ropes tone 0.38, the replaced rope 0.62, mast 0.5, rust 0.22, head and clamps 0.6 to 0.66, hoods 0.12; sand line with two worn hollows at the entry points.", "Seile Tonwert 0,38, das ersetzte Seil 0,62, Mast 0,5, Rost 0,22, Kopf und Klemmen 0,6 bis 0,66, Hauben 0,12; Sandlinie mit zwei ausgetretenen Mulden an den Einstiegen."),
            overlays=T("Thermal, forces and moments draw over the same ropes, mast and anchors.", "Thermik, Kräfte und Momente zeichnen über dieselben Seile, Mast und Anker."),
        ),
        100: dict(
            silhouette=T("Side elevation 62 x 40 mm: all ropes as filled 16 mm member bars with a 0.15 mm outline, the clamps as 0.4 mm dots, mast, rust foot, head, cap, plates and hoods; front and plan likewise.", "Seitenansicht 62 x 40 mm: alle Seile als gefüllte 16-mm-Stäbe mit 0,15-mm-Kontur, die Klemmen als 0,4-mm-Punkte, Mast, Rostfuß, Kopf, Kappe, Platten und Hauben; Vorderansicht und Draufsicht ebenso."),
            retained=T("Every rope, every clamp, the replaced rope, the head with its chamfers, nut, eye bolt and hood.", "Jedes Seil, jede Klemme, das ersetzte Seil, der Kopf mit Fasen, Mutter, Ösenschraube und Haube."),
            merged=T("The mast cap and its rim are one dome step.", "Mastkappe und Rand sind eine Kuppelstufe."),
            omitted=T("Frayed covers, stickers and the tag (below 15 mm), the sand hollows stay as the ground line.", "Ausgefranste Mäntel, Aufkleber und Tag (unter 15 mm); die Sandmulden bleiben als Bodenlinie."),
            thinElements=T("Ropes are filled 16 mm member bars with a 0.15 mm outline, so a rope prints at 0.26 mm at 1:150; the clamps carry a 0.18 mm outline.", "Seile sind gefüllte 16-mm-Stäbe mit 0,15-mm-Kontur, so druckt ein Seil bei 1:150 mit 0,26 mm; die Klemmen tragen eine 0,18-mm-Kontur."),
            hatching=T("None.", "Keine."),
            styles=T("As 1:50 with lighter pens.", "Wie 1:50 mit leichteren Stiften."),
            overlays=T("As 1:50.", "Wie 1:50."),
        ),
        200: dict(
            silhouette=T("Side elevation 31 x 20 mm: the net as filled rope bars with a 0.18 mm outline over the mast, the head and the four anchors with their hoods; front and plan likewise.", "Seitenansicht 31 x 20 mm: das Netz als gefüllte Seilstäbe mit 0,18-mm-Kontur über Mast, Kopf und den vier Ankern mit ihren Hauben; Vorderansicht und Draufsicht ebenso."),
            retained=T("Every rope including the replaced one and the slack step rope, mast, rust foot, head, plates and hoods.", "Jedes Seil einschließlich des ersetzten und des durchhängenden Trittseils, Mast, Rostfuß, Kopf, Platten und Hauben."),
            merged=T("Cap and rim are one flat-topped step; the hood is a solid cylinder over the plate.", "Kappe und Rand sind eine flache Stufe; die Haube ist ein voller Zylinder über der Platte."),
            omitted=T("The cross clamps (40 mm, 0.2 mm on paper: drawn they would turn every node into a blot), nut and eye bolts under the hoods.", "Die Kreuzklemmen (40 mm, 0,2 mm auf dem Papier: gezeichnet würden sie jeden Knoten zum Klecks machen), Mutter und Ösenschrauben unter den Hauben."),
            thinElements=T("Rope outline 0.18 mm, so a rope prints at 0.23 mm at 1:350; the 12 mm plates take the 0.18 mm body pen and print at 0.21 mm at 1:350.", "Seilkontur 0,18 mm, so druckt ein Seil bei 1:350 mit 0,23 mm; die 12-mm-Platten tragen den 0,18-mm-Körperstift und drucken bei 1:350 mit 0,21 mm."),
            hatching=T("None.", "Keine."),
            styles=T("Tones as 1:50.", "Tonwerte wie 1:50."),
            overlays=T("As 1:50.", "Wie 1:50."),
        ),
        500: dict(
            silhouette=T("Side elevation 12.5 x 8 mm: the pyramid of rope bars with a 0.21 mm outline, the mast as a 0.14 mm stroke, the head as a short bar and the four anchor hoods as black dots; front and plan likewise.", "Seitenansicht 12,5 x 8 mm: die Pyramide aus Seilstäben mit 0,21-mm-Kontur, der Mast als 0,14-mm-Strich, der Kopf als kurzer Balken und die vier Ankerhauben als schwarze Punkte; Vorderansicht und Draufsicht ebenso."),
            retained=T("The six full radials of each face (every second one, mesh 0.6 m at the fourth ring), the border with the slack step rope, rings two to six, mast, rust foot, head, plates and hoods: the net still reads as a net at 1.2 mm meshes.", "Die sechs durchgehenden Radialseile jeder Fläche (jedes zweite, Masche 0,6 m am vierten Ring), der Randring mit dem durchhängenden Trittseil, die Ringe zwei bis sechs, Mast, Rostfuß, Kopf, Platten und Hauben: das Netz liest sich bei 1,2-mm-Maschen noch als Netz."),
            merged=T("Mast and cap are one cylinder.", "Mast und Kappe sind ein Zylinder."),
            omitted=T("The five short radials of each face and the first ring (at 1:1000 they would close to gaps of 0.07 to 0.12 mm against the next rope, below the 0.15 mm print gap), clamps, chamfers, nuts and eyes.", "Die fünf kurzen Radialseile jeder Fläche und der erste Ring (bei 1:1000 rückten sie auf 0,07 bis 0,12 mm an das nächste Seil, unter die Drucklücke von 0,15 mm), Klemmen, Fasen, Muttern und Ösen."),
            thinElements=T("Ropes print at 0.23 mm at 1:1000 through their 0.21 mm outline, the 12 mm plates through a 0.25 mm outline; the 140 mm mast takes the 0.15 mm body pen.", "Seile drucken bei 1:1000 mit 0,23 mm durch ihre 0,21-mm-Kontur, die 12-mm-Platten durch eine 0,25-mm-Kontur; der 140-mm-Mast trägt den 0,15-mm-Körperstift."),
            hatching=T("None.", "Keine."),
            styles=T("Tones as 1:50.", "Tonwerte wie 1:50."),
            overlays=T("As 1:50.", "Wie 1:50."),
        ),
    }
    return d[den]


PENS = {50: {"body": 0.35, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.5},
        100: {"body": 0.25, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.35},
        200: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.25},
        500: {"body": 0.15, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

VOL_DECISIONS = {
    50: [T("Body 1:50 (feature 7.5 mm): every rope a slender 16 mm capsule along its true line, the 236 cross clamps as 40 mm spheres, the mast as a revolve with dome cap and a separate rust-brown foot band, the head as a chamfered ring, each anchor as plate, nut, eye bolt with forged eye and a hollow EPDM hood; traces of use are geometry: seven frayed cover sleeves at the step points and three stickers on the mast (texture solids), the replaced radial in fresh rope.",
           "Körper 1:50 (Merkmal 7,5 mm): jedes Seil eine schlanke 16-mm-Kapsel auf seiner wahren Linie, die 236 Kreuzklemmen als 40-mm-Kugeln, der Mast als Drehkörper mit Kuppelkappe und eigenem rostbraunem Fußband, der Kopf als gefaster Ring, jeder Anker als Platte, Mutter, Ösenschraube mit geschmiedeter Öse und hohler EPDM-Haube; Gebrauchsspuren sind Geometrie: sieben ausgefranste Mantelstellen an den Trittpunkten und drei Aufkleber am Mast (Texturkörper), das ersetzte Radialseil in frischem Seil."),
         T("Thermal 1:50: every body solid except the texture parts by reference, seven regions. Structure 1:50: the 7.6 mm steel-wire cores of all ropes as slender members of their face, the mast tube o139.7 x 5, the head ring, joints at the mast foot, head, face roots and sag hinges, a fixed block under the mast and a pin block at each anchor, one mass marker per loaded bone (a net face is a mesh of air, so its marker sits on the rope point nearest to its centre of mass, at most 0.1 m from it).",
           "Thermik 1:50: alle Körper außer den Texturteilen als Verweis, sieben Bereiche. Tragwerk 1:50: die 7,6-mm-Stahldrahtseelen aller Seile als schlanke Stäbe ihrer Fläche, das Mastrohr o139,7 x 5, der Kopfring, Gelenke an Mastfuß, Kopf, Flächenwurzeln und Durchhanggelenken, ein Einspannblock unter dem Mast und ein Gelenkblock an jedem Anker, ein Massenmarker je belastetem Knochen (eine Netzfläche ist ein Maschenwerk aus Luft, daher sitzt ihr Marker auf dem Seilpunkt, der ihrem Schwerpunkt am nächsten liegt, höchstens 0,1 m davon entfernt).")],
    100: [T("Body 1:100 (feature 15 mm), authored anew: ropes keep their true 16 mm as slender capsules, clamps stay as 40 mm spheres, the mast cap is one dome step, the head keeps its chamfers; the fray sleeves and stickers (below 15 mm) are left out.",
            "Körper 1:100 (Merkmal 15 mm), neu erstellt: Seile behalten ihre wahren 16 mm als schlanke Kapseln, die Klemmen bleiben als 40-mm-Kugeln, die Mastkappe ist eine Kuppelstufe, der Kopf behält seine Fasen; ausgefranste Mantelstellen und Aufkleber (unter 15 mm) entfallen."),
          T("Thermal 1:100 as at 1:50. Structure 1:100: the four face-root joints are taken into the head joint, which they lie inside; joints, supports and masses sized for 15 mm.", "Thermik 1:100 wie 1:50. Tragwerk 1:100: die vier Gelenke der Flächenwurzeln gehen im Kopfgelenk auf, in dem sie liegen; Gelenke, Lager und Massen für 15 mm bemessen.")],
    200: [T("Body 1:200 (feature 30 mm), authored anew: ropes stay slender at 16 mm, the cross clamps are left out (they are what the 2D omits too), the head is a plain ring, the hood a solid cylinder over the plate, the 12 mm plate a declared sheet; nut and eye bolt are merged into the hood.",
            "Körper 1:200 (Merkmal 30 mm), neu erstellt: Seile bleiben schlank mit 16 mm, die Kreuzklemmen entfallen (wie in der 2D), der Kopf ist ein schlichter Ring, die Haube ein voller Zylinder über der Platte, die 12-mm-Platte eine erklärte Platte; Mutter und Ösenschraube gehen in der Haube auf."),
          T("Thermal 1:200: the head alone carries the aluminium region; structure as at 1:100, the eye-bolt and hood masses merged into the anchor masses, with 30 mm joints, supports and masses.", "Thermik 1:200: der Kopf allein trägt den Aluminiumbereich; Tragwerk wie 1:100, die Massen von Ösenschrauben und Hauben in den Ankermassen aufgegangen, mit 30-mm-Gelenken, -Lagern und -Massen.")],
    500: [T("Body 1:500 (feature 75 mm), authored anew: the net is generalised like its drawing to the six full radials of each face, the border and rings two to six, all slender at their true 16 mm (the short radials and the first ring would close the print gaps at 1:1000), the mast is one cylinder with its cap, the head a plain ring, the hood a cylinder, the plate a declared sheet.",
            "Körper 1:500 (Merkmal 75 mm), neu erstellt: das Netz ist wie seine Zeichnung auf die sechs durchgehenden Radialseile jeder Fläche, den Randring und die Ringe zwei bis sechs verallgemeinert, alle schlank in ihren wahren 16 mm (die kurzen Radialseile und der erste Ring schlössen bei 1:1000 die Drucklücken), der Mast ist ein Zylinder mit Kappe, der Kopf ein schlichter Ring, die Haube ein Zylinder, die Platte eine erklärte Platte."),
          T("Thermal 1:500: only the total area is compared; structure as at 1:200 with the steel cores of every rope (the load path stays complete), without the hinges and masses of the slack rope pair (0.09 kg each, below what the scale shows), with 75 mm joints, supports and mass markers.", "Thermik 1:500: verglichen wird nur die Gesamtfläche; Tragwerk wie 1:200 mit den Stahlseelen aller Seile (der Lastpfad bleibt vollständig), ohne Gelenke und Massen des durchhängenden Seilpaars (je 0,09 kg, unter dem, was der Maßstab zeigt), mit 75-mm-Gelenken, -Lagern und -Massenmarkern.")],
}


def assemble(rig, scales, total, areas):
    """📄️ The asset document from the rig, the scales and the physical models."""
    reps, vols = {}, {}
    for den, sc in scales.items():
        lo, hi = RANGES[den]
        reps[str(den)] = {"status": "authored", "denominator": den, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[den], "hatches": [],
                          "geometry": sc.views, "decisions": rep_decisions(den)}
        vols[str(den)] = {"status": "authored", "denominator": den, "body": sc.body, "thermal": sc.thermal, "structure": sc.structure, "decisions": VOL_DECISIONS[den]}
    asset = {
        "id": "used-climbing-net-pyramid",
        "category": "objects",
        "title": T("Used Rope Climbing Pyramid", "Gebrauchte Kletternetzpyramide"),
        "summary": T("A 4 m space-net pyramid of a municipal playground after years of service: 16 mm steel-reinforced ropes in a square frustum net from four ground anchors on a 6 x 6 m square to a cast head on a galvanised o139.7 mast, 236 aluminium cross clamps, black EPDM hoods over the tension eyes. The covers are faded and frayed at the step points, one radial is fresh, the mast foot is rusted and stickered, and the step rope of one face hangs slack until the depot re-tensions the net.",
                     "Eine 4 m hohe Raumnetzpyramide eines städtischen Spielplatzes nach Jahren im Einsatz: 16-mm-Seile mit Stahlseele als quadratisches Kegelstumpfnetz von vier Bodenankern auf einem Quadrat von 6 x 6 m zu einem Gusskopf auf einem verzinkten Mast o139,7, 236 Kreuzklemmen aus Aluminium, schwarze EPDM-Hauben über den Spannösen. Die Mäntel sind ausgeblichen und an den Trittstellen ausgefranst, ein Radialseil ist neu, der Mastfuß ist rostig und beklebt, und das Trittseil einer Fläche hängt durch, bis der Bauhof das Netz nachspannt."),
        "tags": ["used", "park", "public-space", "play", "climbing", "space-net", "rope", "steel", "galvanised", "aluminium", "epdm", "playground", "maintenance"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                       "notes": T("Type: the municipal space-net pyramid of German playgrounds since the 1980s, 4 m high, built from 16 mm combination ropes with pressed aluminium cross clamps on a central mast (makers as types, no brand). Dimensions, rope data and masses are catalogue-typical authored estimates; every mass is section times density of the modelled 1:50 parts (ropes 0.33 kg/m, mast 16.6 kg/m); the concrete footings below the 300 mm sand layer are not part of the asset. Traces of use: the rope covers have faded from red (solar absorptance 0.55 against 0.68 of the replaced rope), frayed at seven step points, the zinc of the mast foot is worn through to rust up to 150 mm, stickers and a tag on the mast, two worn hollows in the sand at the entry points, and the creep of the ropes leaves the step rope of the right face 22 mm slack.",
                                  "Typ: die städtische Raumnetzpyramide deutscher Spielplätze seit den 1980er Jahren, 4 m hoch, aus 16-mm-Kombinationsseilen mit gepressten Aluminium-Kreuzklemmen an einem Mittelmast (Hersteller als Typ, keine Marke). Abmessungen, Seildaten und Massen sind katalogtypische Schätzwerte; jede Masse ist Querschnitt mal Dichte der modellierten 1:50-Teile (Seile 0,33 kg/m, Mast 16,6 kg/m); die Betonfundamente unter der 300-mm-Sandschicht gehören nicht zum Asset. Gebrauchsspuren: die Seilmäntel sind von Rot ausgeblichen (Absorptionsgrad 0,55 statt 0,68 des ersetzten Seils), an sieben Trittstellen ausgefranst, das Zink des Mastfußes ist bis 150 mm zu Rost durchgescheuert, Aufkleber und ein Tag am Mast, zwei ausgetretene Mulden im Sand an den Einstiegen, und die Längung der Seile lässt das Trittseil der rechten Fläche 22 mm durchhängen.")},
        "dimensions": {"heightM": CAP_TOP, "widthM": 2 * B + PLATE, "depthM": 2 * B + PLATE, "massKg": round(total, 1)},
        "materials": materials(),
        "rig": {"kind": "mechanical", "root": "ground", "bones": rig.bones},
        "volumes": vols,
        "anchors": anchors_list(rig),
        "representations": reps,
        "symbols": [],
        "poses": poses(rig),
        "clips": clips(),
        "thermal": thermal_model(areas),
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": SUPPORTS,
                      "assumptions": [
                          T("The mast stands fixed in its buried concrete footing; the four ground anchors are pins on their point footings. The net faces hang from the head and are held down at their corner anchors, so the evaluation reads each face as one member from the head to its anchor with the steel cores of its ten radials as section.",
                            "Der Mast steht eingespannt in seinem vergrabenen Betonfundament; die vier Bodenanker sind Gelenke auf ihren Punktfundamenten. Die Netzflächen hängen am Kopf und werden an ihren Eckankern niedergehalten; die Auswertung liest jede Fläche als einen Stab vom Kopf zum Anker mit den Stahlseelen ihrer zehn Radialseile als Querschnitt."),
                          T("The pretension of the net (a few kN per rope, set by the tension eyes) is internal and self-equilibrated and is not part of this rigid-body model; the children are characteristic user loads after EN 1176-1 Annex A with a dynamic factor of 1.8 on the stepping child (authored estimate); the free height of fall inside the net stays below 1 m because every mesh is closed by a ring 0.5 m above the next (EN 1176-11).",
                            "Die Vorspannung des Netzes (einige kN je Seil, über die Spannösen eingestellt) ist innerlich im Gleichgewicht und nicht Teil dieses Starrkörpermodells; die Kinder sind charakteristische Nutzerlasten nach EN 1176-1 Anhang A mit einem dynamischen Faktor von 1,8 für das aufsteigende Kind (Schätzwert); die freie Fallhöhe im Netz bleibt unter 1 m, weil jede Masche 0,5 m über der nächsten von einem Ring geschlossen ist (EN 1176-11)."),
                      ], "clinical": False},
        "presentation": {"defaultPose": "in-service", "defaultClip": "climbing-step", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


def main():
    """🚀️ Writes the asset document."""
    rig = build_rig()
    masses = build_masses(rig)
    for bone in rig.bones:
        if bone["id"] in masses:
            m, com = masses[bone["id"]]
            ref = MASS_REF.get(bone["id"], "rope-16-steel-core" if bone["id"].startswith("net-") else "en-10219-2")
            bone["mass"] = mass_entry(m, com, ref)
    total = sum(b["mass"]["kg"] for b in rig.bones if "mass" in b)
    print(f"[DEBUG] total mass {total:.2f} kg; bones {[(b['id'], b['mass']['kg']) for b in rig.bones if 'mass' in b]}")
    areas = areas_estimate()
    print(f"[DEBUG] areas {areas}")
    scales = {den: author_scale(rig, den, masses) for den in (50, 100, 200, 500)}
    doc = assemble(rig, scales, total, areas)
    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, separators=(",", ":"))
    for den, sc in scales.items():
        print(f"[DEBUG] 1:{den} body {len(sc.body)} thermal {len(sc.thermal)} structure {len(sc.structure)} shapes {[len(v) for v in sc.views.values()]}")


if __name__ == "__main__":
    main()
