"""🪨️ Authoritative generator of the staffage asset `reclaimed-granite-kerbs`: eight reclaimed granite kerbs DIN 482 form A (15 x 30 cm) on squared-timber bearers, one lifted in a scissor kerb clamp, a row of three reset in a concrete bed.

Every hand-chosen number of the asset is stated in this file; running it rewrites `🔣️.json` next to it.

@see ../../brief.md
@see ../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
G = 9.81

RHO_GRANITE = 2650.0
RHO_MORTAR = 1800.0
RHO_TIMBER = 420.0
RHO_CONCRETE = 2400.0
RHO_STEEL = 7850.0
RHO_RUBBER = 1200.0

KERB_B = 0.15
KERB_H = 0.30
ARRIS_R = 0.02
BACK_CHAMFER = 0.01
GAP = 0.05
Z0 = -0.4625
COLUMNS = [Z0 - 0.30, Z0 - 0.10, Z0 + 0.10, Z0 + 0.30]
BEARER_X = [0.20, 0.60]
BEARER = (0.10, 0.10, 0.95)
BATTEN = (0.08, 0.05, 0.85)
Y_BOTTOM = 0.10
Y_BATTEN = 0.40
Y_TOP = 0.45

KERBS = [
    ("kerb-1", 1.20, "bottom", COLUMNS[0], 0.66),
    ("kerb-2", 0.95, "bottom", COLUMNS[1], 0.70),
    ("kerb-3", 1.10, "bottom", COLUMNS[2], 0.63),
    ("kerb-4", 0.85, "bottom", COLUMNS[3], 0.68),
    ("kerb-5", 1.00, "top", COLUMNS[0], 0.64),
    ("kerb-6", 1.15, "top", COLUMNS[1], 0.69),
    ("kerb-7", 0.80, "top", COLUMNS[2], 0.62),
    ("kerb-8", 1.10, "top", COLUMNS[3], 0.67),
]
LENGTH = {k[0]: k[1] for k in KERBS}

BED_X0 = -0.95
BED_LEN = 3.25
BED_Z = Z0 + 1.175
BED_W = 0.45
BED_T = 0.20
HAUNCH_H = 0.15
HAUNCH_TOP = 0.08
ROW = {"kerb-6": -0.86, "kerb-7": 0.30, "kerb-8": 1.11}
ROW_DY = BED_T - Y_TOP

CLAMP_U = 0.50
PIVOT_V = 0.18
EYE_V = 0.52
CHAIN_LEN = 0.60
LINK_PITCH = 0.024
LINK_COUNT = 25
LIFT_DY = 0.60

WHEEL_LOAD_N = 45000.0
CARRIER_U = 0.40

LUMPS = [
    ("lump-1-bed", "kerb-1", "bottom", (0.30, 0.48), (-0.05, 0.06)),
    ("lump-1-back", "kerb-1", "back", (0.70, 0.92), (0.01, 0.11)),
    ("lump-3-bed", "kerb-3", "bottom", (0.75, 0.95), (-0.06, 0.05)),
    ("lump-5-bed", "kerb-5", "bottom", (0.28, 0.50), (-0.07, 0.04)),
    ("lump-5-back", "kerb-5", "back", (0.62, 0.85), (0.0, 0.10)),
    ("lump-6-bed", "kerb-6", "bottom", (0.72, 0.92), (-0.05, 0.05)),
    ("lump-6-back", "kerb-6", "back", (0.10, 0.35), (0.0, 0.12)),
    ("lump-8-bed", "kerb-8", "bottom", (0.30, 0.50), (-0.06, 0.06)),
]
LUMP_T = 0.02

SCALES = [50, 100, 200, 500]
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def T(en, de):
    """🔤️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r(value, digits=5):
    """🎯️ A number rounded for the document."""
    out = round(value, digits)
    return 0.0 if out == 0 else out


def rp(points):
    """🧮️ A rounded point list."""
    return [[r(a), r(b)] for a, b in points]


def arc(cv, cw, radius, a0, a1, steps):
    """🌙️ Points of an arc in a (v, w) profile, angles in degrees, both ends included."""
    return [(cv + radius * math.cos(math.radians(a0 + (a1 - a0) * k / steps)), cw + radius * math.sin(math.radians(a0 + (a1 - a0) * k / steps))) for k in range(steps + 1)]


def circle(cv, cw, radius, n=16):
    """⭕️ A polygon circle in a profile plane."""
    return [(cv + radius * math.cos(2 * math.pi * k / n), cw + radius * math.sin(2 * math.pi * k / n)) for k in range(n)]


def stadium(cu, cv, half_len, half_wid, n=6):
    """🏟️ A stadium (rounded slot) with its long axis along the first coordinate."""
    pts = []
    c1 = cu + half_len - half_wid
    c0 = cu - half_len + half_wid
    for k in range(n + 1):
        a = -math.pi / 2 + math.pi * k / n
        pts.append((c1 + half_wid * math.cos(a), cv + half_wid * math.sin(a)))
    for k in range(n + 1):
        a = math.pi / 2 + math.pi * k / n
        pts.append((c0 + half_wid * math.cos(a), cv + half_wid * math.sin(a)))
    return pts


def strip(p0, p1, width):
    """📏️ A straight bar of `width` between two profile points, as a four-point polygon."""
    dv, dw = p1[0] - p0[0], p1[1] - p0[1]
    n = math.hypot(dv, dw)
    ov, ow = -dw / n * width / 2, dv / n * width / 2
    return [(p0[0] + ov, p0[1] + ow), (p1[0] + ov, p1[1] + ow), (p1[0] - ov, p1[1] - ow), (p0[0] - ov, p0[1] - ow)]


def poly_area(points):
    """📐️ The area of a simple polygon."""
    s = 0.0
    for i in range(len(points)):
        a, b = points[i], points[(i + 1) % len(points)]
        s += a[0] * b[1] - b[0] * a[1]
    return abs(s) / 2


def poly_centroid(points):
    """🎯️ The centroid of a simple polygon."""
    a = 0.0
    cx = cy = 0.0
    for i in range(len(points)):
        p, q = points[i], points[(i + 1) % len(points)]
        c = p[0] * q[1] - q[0] * p[1]
        a += c
        cx += (p[0] + q[0]) * c
        cy += (p[1] + q[1]) * c
    a /= 2
    return cx / (6 * a), cy / (6 * a)


def kerb_profile(scale):
    """🪨️ The section of a kerb in (v, w): visible face at -w with its worn, rounded top arris, a small chamfer at the back top edge; square at 1:200 and 1:500."""
    b, h = KERB_B / 2, KERB_H
    if scale == 50:
        pts = [(0, -b), (0, b), (h - BACK_CHAMFER, b), (h, b - BACK_CHAMFER)]
        pts += arc(h - ARRIS_R, -b + ARRIS_R, ARRIS_R, 0, -90, 4)
        return pts
    if scale == 100:
        return [(0, -b), (0, b), (h, b), (h, -b + ARRIS_R), (h - ARRIS_R, -b)]
    return [(0, -b), (0, b), (h, b), (h, -b)]


BONES = []
ORIGIN = {}
THETA = {}


def bone(id, parent, x, y, z, title, length, joint, angle=0.0, abduction=None, mass=None, section=None, world=None, theta=0.0):
    """🦴️ Adds a bone of the rig and records its rest world origin and its sagittal rotation."""
    entry = {"id": id, "parent": parent, "title": title, "x": r(x), "y": r(y)}
    if z:
        entry["z"] = r(z)
    entry["angleDeg"] = angle
    if abduction is not None:
        entry["abductionDeg"] = abduction
    entry["lengthM"] = r(length)
    entry["joint"] = joint
    if mass is not None:
        entry["mass"] = mass
    if section is not None:
        entry["section"] = section
    BONES.append(entry)
    ORIGIN[id] = world
    THETA[id] = theta


class Solid:
    """🧊️ A solid of one scale in the bone-local frame: a box or a prism along a bone axis, with its region and mass density."""

    def __init__(self, id, bone, kind, material, tone, region, rho, **kw):
        self.id, self.bone, self.kind, self.material, self.tone, self.region, self.rho = id, bone, kind, material, tone, region, rho
        self.kw = kw

    def doc(self):
        """📄️ The JSON of the solid."""
        d = {"id": self.id, "kind": "extrude" if self.kind == "prism" else self.kind, "bone": self.bone, "material": self.material, "role": self.kw.get("role", "body"), "tone": self.tone}
        for flag in ("slender", "sheet", "envelope", "texture"):
            if self.kw.get(flag):
                d[flag] = True
        if self.kind == "box":
            d["size"] = [r(v) for v in self.kw["size"]]
            d["transform"] = {"translation": [r(v) for v in self.kw["at"]]}
        elif self.kind == "prism":
            d["profile"] = rp(self.kw["profile"])
            if self.kw.get("holes"):
                d["holes"] = [rp(h) for h in self.kw["holes"]]
            d["plane"] = {"u": "yz", "w": "xy", "v": "zx"}[self.kw["axis"]]
            d["from"] = r(self.kw["range"][0])
            d["to"] = r(self.kw["range"][1])
        elif self.kind == "capsule":
            d["p0"] = [r(v) for v in self.kw["p0"]]
            d["p1"] = [r(v) for v in self.kw["p1"]]
            d["r0"] = r(self.kw["r"])
            d["r1"] = r(self.kw["r"])
        elif self.kind == "ellipsoid":
            d["radii"] = [r(v) for v in self.kw["radii"]]
            d["transform"] = {"translation": [r(v) for v in self.kw["at"]]}
        if "repeat" in self.kw:
            d["repeat"] = self.kw["repeat"]
        return d

    def copies(self):
        """📚️ The prisms of the solid in the local frame as (axis, profile, holes, range) tuples; a repeated chain link alternates its plane by quarter turns about u."""
        if self.kind == "box":
            (su, sv, sw), (cu, cv, cw) = self.kw["size"], self.kw["at"]
            return [("u", [(cv - sv / 2, cw - sw / 2), (cv + sv / 2, cw - sw / 2), (cv + sv / 2, cw + sw / 2), (cv - sv / 2, cw + sw / 2)], [], (cu - su / 2, cu + su / 2))]
        if self.kind == "capsule":
            (u0, v0, w0), (u1, v1, w1), rad = self.kw["p0"], self.kw["p1"], self.kw["r"]
            return [("u", circle(v0, w0, rad, 12), [], (u0 - rad, u1 + rad))]
        if self.kind == "ellipsoid":
            (ru, rv, rw), (cu, cv, cw) = self.kw["radii"], self.kw["at"]
            return [("u", circle(cv, cw, rv, 12), [], (cu - ru, cu + ru))]
        out = [(self.kw["axis"], self.kw["profile"], self.kw.get("holes", []), self.kw["range"])]
        if "repeat" in self.kw:
            rep = self.kw["repeat"]
            axis, prof, holes, rng = out[0]
            out = []
            for k in range(rep["count"]):
                du = rep["translation"][0] * k
                if k % 2 == 0:
                    out.append(("w", [(a + du, b) for a, b in prof], [[(a + du, b) for a, b in h] for h in holes], rng))
                else:
                    out.append(("v", [(b, a + du) for a, b in prof], [[(b, a + du) for a, b in h] for h in holes], rng))
        return out

    def volume(self):
        """⚖️ The volume of the solid in cubic metres (prisms exact, holes subtracted)."""
        if self.kind == "box":
            su, sv, sw = self.kw["size"]
            return su * sv * sw
        if self.kind == "capsule":
            (u0, _, _), (u1, _, _), rad = self.kw["p0"], self.kw["p1"], self.kw["r"]
            return math.pi * rad * rad * (u1 - u0) + 4 / 3 * math.pi * rad ** 3
        if self.kind == "ellipsoid":
            return 4 / 3 * math.pi * self.kw["radii"][0] * self.kw["radii"][1] * self.kw["radii"][2]
        area = poly_area(self.kw["profile"]) - sum(poly_area(h) for h in self.kw.get("holes", []))
        vol = area * (self.kw["range"][1] - self.kw["range"][0])
        return vol * self.kw.get("repeat", {}).get("count", 1)

    def centroid(self):
        """🎯️ The local centre of the solid (u, v)."""
        if self.kind in ("box", "ellipsoid"):
            return self.kw["at"][0], self.kw["at"][1]
        if self.kind == "capsule":
            return (self.kw["p0"][0] + self.kw["p1"][0]) / 2, self.kw["p0"][1]
        axis, prof, _, rng = self.copies()[0]
        cu = (rng[0] + rng[1]) / 2
        if self.kind == "prism" and "repeat" in self.kw:
            n = self.kw["repeat"]["count"]
            cu0, cv0 = poly_centroid(prof)
            return cu0 + self.kw["repeat"]["translation"][0] * (n - 1) / 2, cv0
        if axis == "u":
            cv, _ = poly_centroid(prof)
            return cu, cv
        if axis == "w":
            return poly_centroid(prof)
        cw, cu2 = poly_centroid(prof)
        return cu2, (rng[0] + rng[1]) / 2


def to_world(bone_id, u, v, w):
    """🌍️ A bone-local rest point in world metres (x ahead, y up, z left)."""
    ox, oy, oz = ORIGIN[bone_id]
    t = math.radians(THETA[bone_id])
    return ox + u * math.cos(t) - v * math.sin(t), oy + u * math.sin(t) + v * math.cos(t), oz + w


def local_point(axis, a, b, c):
    """🔀️ A point of a prism in (u, v, w) from its profile coordinates (a, b) and its extrusion coordinate c."""
    if axis == "u":
        return c, a, b
    if axis == "w":
        return a, b, c
    return b, c, a


def project(bone_id, axis, prof, rng, view):
    """🖼️ The outline of one prism of a bone in a view: the profile itself when the prism runs along the line of sight, else the rectangle of its extents; elevation in bone-local (u, v), front in world (z, y), plan in world (x, z)."""
    pts3 = [to_world(bone_id, *local_point(axis, a, b, c)) for a, b in prof for c in rng]
    def paper(p):
        x, y, z = p
        if view == "elevation":
            ox, oy, _ = ORIGIN[bone_id]
            t = math.radians(THETA[bone_id])
            dx, dy = x - ox, y - oy
            return (dx * math.cos(t) + dy * math.sin(t), -dx * math.sin(t) + dy * math.cos(t))
        if view == "front":
            return (z, y)
        return (x, z)
    depth = {"elevation": 2, "front": 0, "plan": 1}[view]
    lp0 = to_world(bone_id, *local_point(axis, prof[0][0], prof[0][1], rng[0]))
    lp1 = to_world(bone_id, *local_point(axis, prof[0][0], prof[0][1], rng[1]))
    along = [abs(lp1[k] - lp0[k]) for k in range(3)]
    if along[depth] > 1e-9 and sum(along) - along[depth] < 1e-9:
        return [paper(to_world(bone_id, *local_point(axis, a, b, rng[0]))) for a, b in prof], True
    xs = [paper(p)[0] for p in pts3]
    ys = [paper(p)[1] for p in pts3]
    return [(min(xs), min(ys)), (max(xs), min(ys)), (max(xs), max(ys)), (min(xs), max(ys))], False


MATERIALS = [
    {"id": "granite", "title": T("Weathered tooled granite", "Verwitterter bearbeiteter Granit"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": RHO_GRANITE, "conductivityWmK": 2.8, "specificHeatJKgK": 1000, "elasticModulusPa": 50000000000, "strengthPa": 12000000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "old-mortar", "title": T("Old cement mortar and bed-concrete remnants", "Alte Zementmörtel- und Bettungsbetonreste"), "emissivity": 0.93, "solarAbsorptivity": 0.6, "densityKgM3": RHO_MORTAR, "conductivityWmK": 1.0, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "softwood", "title": T("Weathered softwood C24", "Verwittertes Nadelholz C24"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": RHO_TIMBER, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
    {"id": "bed-concrete", "title": T("Earth-moist bedding concrete C20/25", "Erdfeuchter Bettungsbeton C20/25"), "emissivity": 0.93, "solarAbsorptivity": 0.6, "densityKgM3": RHO_CONCRETE, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "elasticModulusPa": 30000000000, "strengthPa": 20000000, "source": "literature", "reference": "en-1992-1-1"},
    {"id": "clamp-steel", "title": T("Painted steel of the kerb clamp and alloy lifting chain", "Lackierter Stahl der Bordsteinzange und legierte Hebekette"), "emissivity": 0.85, "solarAbsorptivity": 0.55, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 355000000, "source": "literature", "reference": "en-10025-2"},
    {"id": "grip-rubber", "title": T("Rubber grip pads of the clamp jaws", "Gummigreifbeläge der Zangenbacken"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": RHO_RUBBER, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "en-iso-10456"},
]
RHO = {m["id"]: m["densityKgM3"] for m in MATERIALS}

REGION_OF_MATERIAL = {"granite": "kerb-granite", "old-mortar": "mortar-remnants", "softwood": "timber-bearers", "bed-concrete": "concrete-bed", "clamp-steel": "lifting-gear", "grip-rubber": "lifting-gear"}


def kerb_world(id):
    """📍️ The rest world origin of a kerb: start end, bottom, centre of its width."""
    k = next(k for k in KERBS if k[0] == id)
    return (0.0, Y_BOTTOM if k[2] == "bottom" else Y_TOP, k[3])


def build_rig():
    """🦴️ The rig: bearers as root, battens, eight kerbs (row kerbs on lateral and forward carriages), the clamp with its two scissor arms, the sling, the concrete bed and the wheel-load carrier."""
    bone("bearers", None, 0, 0, Z0, T("Squared-timber bearers 10/10 (stay on the ground)", "Kantholz-Unterleger 10/10 (bleiben am Boden)"), 0.80, {"type": "fixed"}, world=(0, 0, Z0))
    bone("battens", "bearers", 0, Y_BATTEN, 0, T("Intermediate battens 5/8 between the layers", "Zwischenlatten 5/8 zwischen den Lagen"), 0.80, {"type": "fixed"}, world=(0, Y_BATTEN, Z0))
    for id, length, layer, z, _ in KERBS:
        if layer == "bottom":
            bone(id, "bearers", 0, Y_BOTTOM, z - Z0, T(f"Kerb {id[-1]}, bottom layer, {length:.2f} m", f"Bordstein {id[-1]}, untere Lage, {length:.2f} m".replace(".", ",")), length, {"type": "fixed"}, world=(0, Y_BOTTOM, z))
    bone("kerb-5", "battens", 0, Y_TOP - Y_BATTEN, COLUMNS[0] - Z0, T("Kerb 5, top layer, 1.00 m (lifted in the clamp)", "Bordstein 5, obere Lage, 1,00 m (in der Zange gehoben)"), LENGTH["kerb-5"], {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.8}, world=(0, Y_TOP, COLUMNS[0]))
    bone("lateral", "bearers", 0, 0, 0, T("Lateral set-out axis of the reinstated row (massless)", "Quer-Absteckachse der wieder versetzten Reihe (masselos)"), 0, {"type": "fixed"}, abduction=90, world=(0, 0, Z0))
    for id in ("kerb-6", "kerb-7", "kerb-8"):
        z = next(k[3] for k in KERBS if k[0] == id)
        n = id[-1]
        bone(f"{id}-shift-z", "lateral", 0, 0, 0, T(f"Lateral carriage of kerb {n} (massless)", f"Querschlitten von Bordstein {n} (masselos)"), 0, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.3}, abduction=-90, world=(0, 0, Z0))
        bone(f"{id}-shift-x", f"{id}-shift-z", 0, Y_TOP, z - Z0, T(f"Forward carriage of kerb {n} (massless)", f"Längsschlitten von Bordstein {n} (masselos)"), 0, {"type": "prismatic", "axisDeg": 0, "minM": -1.0, "maxM": 1.2}, world=(0, Y_TOP, z))
        length = LENGTH[id]
        bone(id, f"{id}-shift-x", 0, 0, 0, T(f"Kerb {n}, top layer, {length:.2f} m (reset in the row)", f"Bordstein {n}, obere Lage, {length:.2f} m (in der Reihe versetzt)".replace(".", ",")), length, {"type": "prismatic", "axisDeg": 90, "minM": -0.3, "maxM": 0}, world=(0, Y_TOP, z))
    cw = (CLAMP_U, Y_TOP + KERB_H, COLUMNS[0])
    bone("clamp", "kerb-5", CLAMP_U, KERB_H, 0, T("Scissor kerb clamp: frame, stop plate, pivot and eye", "Scheren-Bordsteinzange: Rahmen, Anschlagplatte, Drehpunkt und Öse"), 0.15, {"type": "fixed"}, world=cw)
    for side in ("a", "b"):
        bone(f"clamp-arm-{side}", "clamp", 0, PIVOT_V, 0, T(f"Scissor arm {side.upper()} with jaw (turns about the pivot pin)", f"Scherenarm {side.upper()} mit Backe (dreht um den Drehbolzen)"), 0.075, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -12, "twistMaxDeg": 12}, world=(cw[0], cw[1] + PIVOT_V, cw[2]))
    bone("sling", "clamp", 0, EYE_V, 0, T("Lifting chain and master link (hook side)", "Hebekette und Aufhängering (Hakenseite)"), CHAIN_LEN, {"type": "fixed"}, angle=90, world=(cw[0], cw[1] + EYE_V, cw[2]), theta=90.0)
    bone("bed", "bearers", BED_X0, 0, BED_Z - Z0, T("Concrete bed with back haunch of the new kerb line", "Betonbett mit Rückenstütze der neuen Bordsteinlinie"), BED_LEN, {"type": "fixed"}, world=(BED_X0, 0, BED_Z))
    bone("wheel-carrier", "kerb-7", CARRIER_U, KERB_H, 0, T("Wheel-load carrier (massless, moves the wheel along the row)", "Radlastträger (masselos, bewegt das Rad entlang der Reihe)"), 0, {"type": "prismatic", "axisDeg": 0, "minM": -1.4, "maxM": 1.4}, world=(CARRIER_U, Y_TOP + KERB_H, COLUMNS[2]))


def clamp_solids(scale):
    """🗜️ The solids of the kerb clamp at one scale: frame parts on `clamp`, arm, jaw and pad on each arm bone (arm-local v is measured from the pivot)."""
    out = []
    upper = 0.34
    lower = 0.02
    if scale in (50, 100):
        pad = scale == 50
        out.append(Solid("stop-plate", "clamp", "box", "clamp-steel", 0.32, "lifting-gear", RHO_STEEL, size=(0.12, 0.015, 0.11), at=(0, 0.0075, 0)))
        out.append(Solid("stem", "clamp", "box", "clamp-steel", 0.3, "lifting-gear", RHO_STEEL, size=(0.03, 0.17, 0.03), at=(0, 0.10, 0)))
        out.append(Solid("pivot-pin", "clamp", "prism", "clamp-steel", 0.22, "lifting-gear", RHO_STEEL, axis="u", profile=circle(PIVOT_V, 0, 0.0125), range=(-0.04, 0.04)))
        link_t = 0.015
        out.append(Solid("link-a", "clamp", "prism", "clamp-steel", 0.3, "lifting-gear", RHO_STEEL, axis="u", profile=strip((upper, 0.0925), (0.48, 0), 0.04), range=(-0.015 - link_t, -0.015)))
        out.append(Solid("link-b", "clamp", "prism", "clamp-steel", 0.3, "lifting-gear", RHO_STEEL, axis="u", profile=strip((upper, -0.0925), (0.48, 0), 0.04), range=(0.015, 0.015 + link_t)))
        out.append(Solid("eye-pin", "clamp", "prism", "clamp-steel", 0.22, "lifting-gear", RHO_STEEL, axis="u", profile=circle(0.48, 0, 0.01 if pad else 0.0125), range=(-0.035, 0.035)))
        eye = [(0.45, -0.03), (0.45, 0.03)] + [(0.53 + 0.03 * math.sin(math.pi * k / 8), 0.03 * math.cos(math.pi * k / 8)) for k in range(9)]
        out.append(Solid("eye-plate", "clamp", "prism", "clamp-steel", 0.3, "lifting-gear", RHO_STEEL, axis="u", profile=eye, range=(-0.015, 0.015)))
        for side, sign, urange in (("a", -1, (-0.030, -0.015)), ("b", 1, (0.015, 0.030))):
            b = f"clamp-arm-{side}"
            out.append(Solid(f"arm-{side}", b, "prism", "clamp-steel", 0.28, "lifting-gear", RHO_STEEL, axis="u", profile=strip((lower - PIVOT_V, sign * 0.0925), (upper - PIVOT_V, -sign * 0.0925), 0.05), range=urange))
            if pad:
                jw = (0.085, 0.100)
                out.append(Solid(f"jaw-{side}", b, "box", "clamp-steel", 0.26, "lifting-gear", RHO_STEEL, size=(0.15, 0.16, 0.015), at=(0, -0.05 - PIVOT_V, sign * 0.0925)))
                out.append(Solid(f"pad-{side}", b, "box", "grip-rubber", 0.12, "lifting-gear", RHO_RUBBER, size=(0.14, 0.10, 0.01), at=(0, -0.07 - PIVOT_V, sign * 0.08)))
            else:
                out.append(Solid(f"jaw-{side}", b, "box", "clamp-steel", 0.24, "lifting-gear", RHO_STEEL, size=(0.15, 0.16, 0.025), at=(0, -0.05 - PIVOT_V, sign * 0.0875)))
        return out
    if scale == 200:
        out.append(Solid("frame", "clamp", "prism", "clamp-steel", 0.3, "lifting-gear", RHO_STEEL, axis="u", profile=[(0, -0.055), (0, 0.055), (0.03, 0.055), (0.03, 0.015), (0.19, 0.015), (0.19, -0.015), (0.03, -0.015), (0.03, -0.055)], range=(-0.06, 0.06)))
        out.append(Solid("head", "clamp", "prism", "clamp-steel", 0.3, "lifting-gear", RHO_STEEL, axis="u", profile=[(upper - 0.03, -0.11), (upper - 0.03, 0.11), (upper + 0.02, 0.11), (0.56, 0.03), (0.56, -0.03), (upper + 0.02, -0.11)], range=(-0.03, 0.03)))
        for side, sign, urange in (("a", -1, (-0.03, 0.0)), ("b", 1, (0.0, 0.03))):
            b = f"clamp-arm-{side}"
            out.append(Solid(f"arm-{side}", b, "prism", "clamp-steel", 0.28, "lifting-gear", RHO_STEEL, axis="u", profile=strip((lower - PIVOT_V, sign * 0.09), (upper - 0.02 - PIVOT_V, -sign * 0.09), 0.05), range=urange))
            out.append(Solid(f"jaw-{side}", b, "box", "clamp-steel", 0.24, "lifting-gear", RHO_STEEL, size=(0.15, 0.16, 0.03), at=(0, -0.05 - PIVOT_V, sign * 0.09)))
        return out
    out.append(Solid("clamp-block", "clamp", "prism", "clamp-steel", 0.28, "lifting-gear", RHO_STEEL, axis="u", profile=[(0.0, -0.10), (0.0, 0.10), (0.36, 0.10), (0.56, 0.04), (0.56, -0.04), (0.36, -0.10)], range=(-0.075, 0.075)))
    return out


def sling_solids(scale):
    """⛓️ The lifting chain (25 alternating links of 8 mm grade-8 chain) and the master link at 1:50, a true-section rod above."""
    if scale == 50:
        link = Solid("chain", "sling", "prism", "clamp-steel", 0.24, "lifting-gear", RHO_STEEL, axis="w", profile=stadium(0.016, 0, 0.020, 0.0135), holes=[stadium(0.016, 0, 0.012, 0.0055)], range=(-0.004, 0.004), repeat={"count": LINK_COUNT, "translation": [LINK_PITCH, 0, 0], "rotationAxis": [1, 0, 0], "rotationRad": r(math.pi / 2, 6)})
        master = Solid("master-link", "sling", "prism", "clamp-steel", 0.24, "lifting-gear", RHO_STEEL, axis="w", profile=stadium(0.668, 0, 0.068, 0.043, 8), holes=[stadium(0.668, 0, 0.055, 0.030, 8)], range=(-0.0065, 0.0065))
        return [link, master]
    if scale == 100:
        rod = Solid("chain", "sling", "capsule", "clamp-steel", 0.24, "lifting-gear", RHO_STEEL, p0=(0.0135, 0, 0), p1=(0.5865, 0, 0), r=0.0135, slender=True)
        master = Solid("master-link", "sling", "prism", "clamp-steel", 0.24, "lifting-gear", RHO_STEEL, axis="w", profile=stadium(0.668, 0, 0.068, 0.043, 6), holes=[stadium(0.668, 0, 0.053, 0.028, 6)], range=(-0.0075, 0.0075))
        return [rod, master]
    return [Solid("chain", "sling", "capsule", "clamp-steel", 0.24, "lifting-gear", RHO_STEEL, p0=(0.0135, 0, 0), p1=(0.7225, 0, 0), r=0.0135, slender=True)]


def kerb_solids(scale):
    """🪨️ The eight kerbs and their mortar remnants at one scale; at 1:500 the bottom kerbs take the 50 mm batten layer into their mass."""
    out = []
    for id, length, layer, z, tone in KERBS:
        if scale == 500 and layer == "bottom":
            prof = [(0, -KERB_B / 2), (0, KERB_B / 2), (KERB_H + 0.05, KERB_B / 2), (KERB_H + 0.05, -KERB_B / 2)]
        else:
            prof = kerb_profile(scale)
        out.append(Solid(id, id, "prism", "granite", tone, "kerb-granite", RHO_GRANITE, axis="u", profile=prof, range=(0, length)))
    for lid, kid, where, (u0, u1), (a0, a1) in LUMPS:
        texture = scale in (200, 500)
        if where == "bottom":
            size, at = (u1 - u0, LUMP_T, a1 - a0), ((u0 + u1) / 2, -LUMP_T / 2, (a0 + a1) / 2)
        else:
            size, at = (u1 - u0, a1 - a0, LUMP_T), ((u0 + u1) / 2, (a0 + a1) / 2, KERB_B / 2 + LUMP_T / 2)
        out.append(Solid(lid, kid, "box", "old-mortar", 0.84, "mortar-remnants", RHO_MORTAR, size=size, at=at, texture=texture))
    return out


def timber_solids(scale):
    """🪵️ Bearers and battens; at 1:500 the battens are taken into the bottom kerbs."""
    out = []
    for i, x in enumerate(BEARER_X):
        out.append(Solid(f"bearer-{'ab'[i]}", "bearers", "box", "softwood", 0.42 + 0.04 * i, "timber-bearers", RHO_TIMBER, size=BEARER, at=(x, BEARER[1] / 2, 0)))
    if scale != 500:
        for i, x in enumerate(BEARER_X):
            out.append(Solid(f"batten-{'ab'[i]}", "battens", "box", "softwood", 0.5 + 0.04 * i, "timber-bearers", RHO_TIMBER, size=BATTEN, at=(x, BATTEN[1] / 2, 0)))
    return out


def bed_solids(scale):
    """🧱️ The concrete bed (200 mm C20/25, 450 mm wide) and the back haunch; one L-shaped section at 1:500."""
    if scale == 500:
        prof = [(0, -BED_W / 2), (0, BED_W / 2), (BED_T + HAUNCH_H, KERB_B / 2 + HAUNCH_TOP), (BED_T + HAUNCH_H, KERB_B / 2), (BED_T, KERB_B / 2), (BED_T, -BED_W / 2)]
        return [Solid("bed", "bed", "prism", "bed-concrete", 0.78, "concrete-bed", RHO_CONCRETE, axis="u", profile=prof, range=(0, BED_LEN))]
    base = Solid("bed", "bed", "box", "bed-concrete", 0.78, "concrete-bed", RHO_CONCRETE, size=(BED_LEN, BED_T, BED_W), at=(BED_LEN / 2, BED_T / 2, 0))
    haunch = Solid("haunch", "bed", "prism", "bed-concrete", 0.72, "concrete-bed", RHO_CONCRETE, axis="u", profile=[(BED_T, KERB_B / 2), (BED_T, BED_W / 2), (BED_T + HAUNCH_H, KERB_B / 2 + HAUNCH_TOP), (BED_T + HAUNCH_H, KERB_B / 2)], range=(0, BED_LEN))
    return [base, haunch]


def body(scale):
    """🧍️ The body list of a scale."""
    return kerb_solids(scale) + timber_solids(scale) + bed_solids(scale) + clamp_solids(scale) + sling_solids(scale)


def nearness(solid, view):
    """🔭️ How near the solid is to the viewer of a view at rest (elevation: the smaller z, front: the larger x, plan: the larger y), so the authored paint order is the physical one."""
    pts = [to_world(solid.bone, *local_point(axis, a, b, c)) for axis, prof, _, rng in solid.copies() for a, b in prof for c in rng]
    if view == "elevation":
        return -min(p[2] for p in pts)
    if view == "front":
        return max(p[0] for p in pts)
    return max(p[1] for p in pts)


def z_of(solid, view, copy=0):
    """🪜️ The authored paint order of a shape: its nearness to the viewer, a copy of a repeat a hair above the previous one."""
    return r(10 + nearness(solid, view) + 0.0001 * copy, 4)


def shapes_of(solid, view, scale):
    """✏️ The filled closed 2D shapes of one body solid in one view (the chain at 1:500 is an open stroke along its axis, its plan is left inside the clamp)."""
    out = []
    copies = solid.copies()
    if solid.kw.get("texture"):
        return out
    if solid.id == "chain" and scale == 500:
        if view == "plan":
            return out
        if view == "elevation":
            pts = [(0.0, 0.0), (0.736, 0.0)]
        else:
            x0, y0, z0 = to_world("sling", 0, 0, 0)
            pts = [(z0, y0), (z0, y0 + 0.736)]
        return [{"id": "chain", "bone": solid.bone, "role": "body", "z": z_of(solid, view), "filled": False, "tone": solid.tone, "region": solid.region, "material": solid.material, "kind": "poly", "points": rp(pts), "closed": False}]
    if solid.id == "chain" and scale == 200 and view == "plan":
        return out
    for k, (axis, prof, holes, rng) in enumerate(copies):
        outline, along = project(solid.bone, axis, prof, rng, view)
        sid = solid.id if len(copies) == 1 else f"{solid.id}-{k:02d}"
        base = {"id": sid, "bone": solid.bone, "role": "body", "z": z_of(solid, view, k), "filled": True, "tone": solid.tone, "region": solid.region, "material": solid.material}
        if scale >= 200 and solid.region == "kerb-granite" and any(l[1] == solid.bone for l in LUMPS):
            del base["region"]
            base["regions"] = ["kerb-granite", "mortar-remnants"]
        if along and holes:
            rings = [outline] + [project(solid.bone, axis, h, rng, view)[0] for h in holes]
            d = " ".join("M " + " L ".join(f"{r(a)} {r(b)}" for a, b in ring) + " Z" for ring in rings)
            out.append({**base, "kind": "path", "d": d})
        else:
            out.append({**base, "kind": "poly", "points": rp(outline), "closed": True})
    return out


def kerb_z(id, scale, view, lift):
    """🪜️ The paint order of a detail stroke of a kerb: just above the kerb it lies on."""
    solid = next(s for s in kerb_solids(scale) if s.id == id)
    return r(z_of(solid, view) + lift, 4)


def detail_lines(scale, view):
    """〰️ Detail and texture strokes: the worn arris and the edge of the bush-hammered band of each kerb, the back chamfer, the end checks of the bearers."""
    out = []
    if scale >= 200:
        return out
    for id, length, layer, z, tone in KERBS:
        if view == "elevation":
            out.append({"id": f"{id}-arris", "bone": id, "role": "detail", "z": kerb_z(id, scale, view, 0.0003), "filled": False, "tone": 0.3, "region": "kerb-granite", "kind": "poly", "points": rp([(0, KERB_H - ARRIS_R), (length, KERB_H - ARRIS_R)]), "closed": False})
            out.append({"id": f"{id}-tooling", "bone": id, "role": "texture", "z": kerb_z(id, scale, view, 0.0002), "filled": False, "tone": 0.35, "region": "kerb-granite", "kind": "poly", "points": rp([(0, 0.10), (length, 0.10)]), "closed": False})
        if view == "plan":
            ox, oy, oz = ORIGIN[id]
            out.append({"id": f"{id}-arris", "bone": id, "role": "detail", "z": kerb_z(id, scale, view, 0.0003), "filled": False, "tone": 0.3, "region": "kerb-granite", "kind": "poly", "points": rp([(ox, oz - KERB_B / 2 + ARRIS_R), (ox + length, oz - KERB_B / 2 + ARRIS_R)]), "closed": False})
            if scale == 50:
                out.append({"id": f"{id}-chamfer", "bone": id, "role": "detail", "z": kerb_z(id, scale, view, 0.0003), "filled": False, "tone": 0.3, "region": "kerb-granite", "kind": "poly", "points": rp([(ox, oz + KERB_B / 2 - BACK_CHAMFER), (ox + length, oz + KERB_B / 2 - BACK_CHAMFER)]), "closed": False})
    if scale == 50 and view == "elevation":
        for i, x in enumerate(BEARER_X):
            out.append({"id": f"bearer-{'ab'[i]}-check", "bone": "bearers", "role": "texture", "z": r(10 + 0.9375 + 0.0003, 4), "filled": False, "tone": 0.3, "region": "timber-bearers", "kind": "poly", "points": rp([(x - 0.03, 0.065), (x + 0.005, 0.045), (x + 0.02, 0.02)]), "closed": False})
    return out


HATCHES = {
    50: [{"id": "tooled-granite", "title": T("Bush-hammered granite face", "Gestockte Granitfläche"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.1}],
    100: [{"id": "tooled-granite", "title": T("Bush-hammered granite face", "Gestockte Granitfläche"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}],
    200: [],
    500: [],
}
PENS = {
    50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5},
    100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35},
    200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25},
    500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18},
}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def geometry(scale):
    """🗺️ The three views of a scale, drawn from the same numbers as its solids."""
    out = {}
    solids = body(scale)
    for view in ("elevation", "front", "plan"):
        shapes = []
        for solid in solids:
            for shape in shapes_of(solid, view, scale):
                if solid.region == "kerb-granite" and view in ("elevation", "plan") and HATCHES[scale]:
                    shape["hatch"] = "tooled-granite"
                shapes.append(shape)
        shapes += detail_lines(scale, view)
        out[view] = shapes
    return out


def mass_by_bone():
    """⚖️ Mass and local centre of every bone from the 1:50 solids (section x density)."""
    acc = {}
    for s in body(50):
        m = s.volume() * RHO[s.material]
        cu, cv = s.centroid()
        a = acc.setdefault(s.bone, [0.0, 0.0, 0.0])
        a[0] += m
        a[1] += m * cu
        a[2] += m * cv
    return {b: (m, mu / m, mv / m) for b, (m, mu, mv) in acc.items()}


SOURCES = {"bearers": ("literature", "en-338"), "battens": ("literature", "en-338"), "bed": ("literature", "en-1991-1-1"), "clamp": ("authored-estimate", "en-13155"), "clamp-arm-a": ("authored-estimate", "en-13155"), "clamp-arm-b": ("authored-estimate", "en-13155"), "sling": ("literature", "en-818-2")}
SECTIONS = {
    "bearers": {"areaM2": 0.01, "secondMomentM4": 8.333e-06, "extremeFibreM": 0.05, "material": "softwood", "title": T("Squared timber 100 x 100 mm, C24", "Kantholz 100 x 100 mm, C24")},
    "battens": {"areaM2": 0.004, "secondMomentM4": 8.333e-07, "extremeFibreM": 0.025, "material": "softwood", "title": T("Batten 50 x 80 mm (flat), C24", "Latte 50 x 80 mm (flach), C24")},
    "bed": {"areaM2": 0.09, "secondMomentM4": 0.0003, "extremeFibreM": 0.1, "material": "bed-concrete", "title": T("Bed 450 x 200 mm, C20/25 (haunch not counted)", "Bett 450 x 200 mm, C20/25 (Rückenstütze nicht angesetzt)")},
    "clamp": {"areaM2": 0.0009, "secondMomentM4": 6.75e-08, "extremeFibreM": 0.015, "material": "clamp-steel", "title": T("Clamp stem 30 x 30 mm", "Zangenschaft 30 x 30 mm")},
    "clamp-arm-a": {"areaM2": 0.00075, "secondMomentM4": 1.5625e-07, "extremeFibreM": 0.025, "material": "clamp-steel", "title": T("Scissor arm, flat bar 50 x 15 mm", "Scherenarm, Flachstahl 50 x 15 mm")},
    "clamp-arm-b": {"areaM2": 0.00075, "secondMomentM4": 1.5625e-07, "extremeFibreM": 0.025, "material": "clamp-steel", "title": T("Scissor arm, flat bar 50 x 15 mm", "Scherenarm, Flachstahl 50 x 15 mm")},
    "sling": {"areaM2": 0.0001005, "secondMomentM4": 4.02e-10, "extremeFibreM": 0.004, "material": "clamp-steel", "title": T("Round-steel chain 8 mm, grade 8 (two link legs)", "Rundstahlkette 8 mm, Güteklasse 8 (zwei Gliedschenkel)")},
}
KERB_SECTION = {"areaM2": 0.045, "secondMomentM4": 0.0003375, "extremeFibreM": 0.15, "material": "granite", "title": T("Granite kerb 150 x 300 mm (DIN 482 form A)", "Granitbordstein 150 x 300 mm (DIN 482 Form A)")}


def finish_rig():
    """🧷️ Writes masses and sections onto the bones."""
    masses = mass_by_bone()
    for b in BONES:
        if b["id"] in masses:
            m, cu, cv = masses[b["id"]]
            src = SOURCES.get(b["id"], ("literature", "en-1343"))
            b["mass"] = {"kg": r(m, 3), "comX": r(cu, 4), "comY": r(cv, 4), "distribution": "point", "source": src[0], "reference": src[1]}
            b["section"] = KERB_SECTION if b["id"].startswith("kerb-") else SECTIONS[b["id"]]
    return masses


def anchors():
    """📍️ Ground contacts of bearers and bed, the hook in the master link and the wheel contact."""
    return [
        {"id": "bearer-a-ground", "bone": "bearers", "x": BEARER_X[0], "y": 0, "kind": "contact", "title": T("Front bearer on the yard gravel", "Vorderer Unterleger auf dem Hofschotter"), "support": "contact", "normalDeg": 90, "footprintLengthM": BEARER[0], "footprintWidthM": BEARER[2], "frictionCoefficient": 0.5},
        {"id": "bearer-b-ground", "bone": "bearers", "x": BEARER_X[1], "y": 0, "kind": "contact", "title": T("Rear bearer on the yard gravel", "Hinterer Unterleger auf dem Hofschotter"), "support": "contact", "normalDeg": 90, "footprintLengthM": BEARER[0], "footprintWidthM": BEARER[2], "frictionCoefficient": 0.5},
        {"id": "bed-ground-a", "bone": "bed", "x": r(BED_LEN / 4), "y": 0, "kind": "contact", "title": T("Concrete bed on the formation, first half", "Betonbett auf dem Planum, erste Hälfte"), "support": "contact", "normalDeg": 90, "footprintLengthM": r(BED_LEN / 2), "footprintWidthM": BED_W, "frictionCoefficient": 0.6},
        {"id": "bed-ground-b", "bone": "bed", "x": r(3 * BED_LEN / 4), "y": 0, "kind": "contact", "title": T("Concrete bed on the formation, second half", "Betonbett auf dem Planum, zweite Hälfte"), "support": "contact", "normalDeg": 90, "footprintLengthM": r(BED_LEN / 2), "footprintWidthM": BED_W, "frictionCoefficient": 0.6},
        {"id": "sling-point", "bone": "sling", "x": 0.736, "y": 0, "kind": "load", "title": T("Crane or excavator hook in the master link", "Kran- oder Baggerhaken im Aufhängering")},
        {"id": "wheel-contact", "bone": "wheel-carrier", "x": 0, "y": 0, "kind": "load", "title": T("Wheel contact on the kerb top", "Radaufstand auf der Bordsteinoberseite")},
    ]


CONTACTS = ["bearer-a-ground", "bearer-b-ground", "bed-ground-a", "bed-ground-b"]
REINSTALLED = {}
for _id in ("kerb-6", "kerb-7", "kerb-8"):
    _z = next(k[3] for k in KERBS if k[0] == _id)
    REINSTALLED[f"{_id}-shift-z"] = {"dx": r(BED_Z - _z)}
    REINSTALLED[f"{_id}-shift-x"] = {"dx": ROW[_id]}
    REINSTALLED[_id] = {"dy": r(ROW_DY)}

BINDINGS = {
    "stored": {"kerb-granite": (0.86, 0.35), "mortar-remnants": (0.9, 0.1), "timber-bearers": (0.75, 0.15), "concrete-bed": (0.95, 0.6), "lifting-gear": (1, 0.5)},
    "lifted": {"kerb-granite": (0.88, 0.38), "mortar-remnants": (0.95, 0.2), "timber-bearers": (0.75, 0.15), "concrete-bed": (0.95, 0.6), "lifting-gear": (1, 0.5)},
    "reinstalled": {"kerb-granite": (0.78, 0.4), "mortar-remnants": (0.55, 0.05), "timber-bearers": (0.75, 0.2), "concrete-bed": (0.8, 0.5), "lifting-gear": (1, 0.5)},
}


def binding(kind):
    """🌬️ The thermal binding of a pose: every region with its exposed and sunlit share, nothing operating."""
    return {"regions": {k: {"exposedFraction": e, "sunlitFraction": s} for k, (e, s) in BINDINGS[kind].items()}, "emitters": {}}


def poses(hung_n):
    """📥️ Stored on bearers, lifted in the clamp, reset in the bed, and reset under a wheel load."""
    kn = f"{hung_n / 1000:.2f}"
    return [
        {"id": "stored", "title": T("Stored on bearers", "Auf Unterlegern gelagert"), "description": T("Eight kerbs upright in two layers of four on two squared-timber bearers with battens between the layers; the clamp is set on kerb 5 and the chain hangs taut from the hook, ready to lift.", "Acht Bordsteine stehend in zwei Lagen zu vier auf zwei Kanthölzern mit Zwischenlatten; die Zange sitzt auf Bordstein 5, die Kette hängt straff am Haken, hebebereit."), "bones": {}, "ground": "authored", "contacts": CONTACTS, "thermal": binding("stored"), "loads": []},
        {"id": "lifted", "title": T("Kerb 5 lifted in the clamp", "Bordstein 5 in der Zange gehoben"), "description": T(f"Kerb 5 hangs 0.60 m above its place in the scissor clamp; the hook carries kerb, remnants, clamp and chain ({kn} kN); bearers, battens and the other seven kerbs stay on the ground.", f"Bordstein 5 hängt 0,60 m über seinem Platz in der Scherenzange; der Haken trägt Stein, Reste, Zange und Kette ({kn.replace('.', ',')} kN); Unterleger, Latten und die übrigen sieben Steine bleiben am Boden."), "bones": {"kerb-5": {"dy": LIFT_DY}}, "ground": "authored", "contacts": CONTACTS, "thermal": binding("lifted"), "loads": [{"id": "hook-lift", "kind": "carried", "title": T("Hook force of the crane or excavator", "Hakenkraft des Krans oder Baggers"), "anchor": "sling-point", "fxN": 0, "fyN": r(hung_n, 1), "source": "authored-estimate", "reference": "en-13155"}]},
        {"id": "reinstalled", "title": T("Row reset in the concrete bed", "Reihe im Betonbett versetzt"), "description": T("Kerbs 6, 7 and 8 stand end to end in the earth-moist concrete bed against the back haunch, with 10 mm joints; kerb 5 and the clamp are still on the stack.", "Die Bordsteine 6, 7 und 8 stehen Stoß an Stoß im erdfeuchten Betonbett an der Rückenstütze, Fugen 10 mm; Bordstein 5 und die Zange sind noch auf dem Stapel."), "bones": dict(REINSTALLED), "ground": "authored", "contacts": CONTACTS, "thermal": binding("reinstalled"), "loads": []},
        {"id": "reinstalled-wheel-load", "title": T("Reset row under a wheel load", "Versetzte Reihe unter Radlast"), "description": T("A 45 kN wheel of a 90 kN axle stands on the middle kerb of the reset row (EN 1991-1-1 category G).", "Ein Rad von 45 kN einer 90-kN-Achse steht auf dem mittleren Stein der versetzten Reihe (EN 1991-1-1 Kategorie G)."), "bones": {**REINSTALLED, "wheel-carrier": {"dx": 0}}, "ground": "authored", "contacts": CONTACTS, "thermal": binding("reinstalled"), "loads": [{"id": "wheel", "kind": "live", "title": T("Wheel load 45 kN", "Radlast 45 kN"), "anchor": "wheel-contact", "fxN": 0, "fyN": -WHEEL_LOAD_N, "source": "literature", "reference": "en-1991-1-1"}]},
    ]


def clips():
    """🎞️ `lift` (the hook takes the weight, then hoists 0.60 m) and `wheel-overrun` (a wheel rolls along the reset row)."""
    return [
        {"id": "lift", "title": T("Lift in the kerb clamp", "Heben in der Bordsteinzange"), "description": T("The hook takes up the weight of kerb 5 within one second, then hoists it 0.60 m off the stack; the stack stays on its bearers.", "Der Haken übernimmt in einer Sekunde das Gewicht von Bordstein 5 und hebt ihn dann 0,60 m vom Stapel; der Stapel bleibt auf seinen Unterlegern."), "durationS": 5, "fps": 30, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "lifted", "ease": "smooth", "bones": {"kerb-5": {"dy": 0}}}, {"t": 1, "pose": "lifted", "ease": "smooth", "bones": {"kerb-5": {"dy": 0}}}, {"t": 5, "pose": "lifted", "ease": "smooth"}], "contacts": [], "phases": [{"id": "take-up", "fromS": 0, "toS": 1, "title": T("Take up the weight", "Gewicht übernehmen")}, {"id": "hoist", "fromS": 1, "toS": 5, "title": T("Hoist", "Anheben")}], "events": [{"t": 1, "kind": "load-on", "title": T("The hook carries the whole kerb", "Der Haken trägt den ganzen Stein"), "target": "hook-lift"}, {"t": 1, "kind": "motion", "title": T("Kerb 5 leaves the battens", "Bordstein 5 verlässt die Latten")}], "tracks": [{"target": "load:hook-lift", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1, "v": 1}, {"t": 5, "v": 1}]}]},
        {"id": "wheel-overrun", "title": T("Wheel rolling along the reset row", "Rad rollt über die versetzte Reihe"), "description": T("A 45 kN wheel rolls from 0.2 m inside kerb 6 to 0.2 m before the end of kerb 8, across both joints.", "Ein Rad von 45 kN rollt von 0,2 m in Bordstein 6 bis 0,2 m vor das Ende von Bordstein 8, über beide Fugen."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "reinstalled-wheel-load", "ease": "linear", "bones": {"wheel-carrier": {"dx": -1.36}}}, {"t": 6, "pose": "reinstalled-wheel-load", "ease": "linear", "bones": {"wheel-carrier": {"dx": 1.31}}}], "contacts": [], "phases": [{"id": "roll", "fromS": 0, "toS": 6, "title": T("Roll", "Rollen")}], "events": [{"t": 0, "kind": "load-on", "title": T("Wheel on kerb 6", "Rad auf Bordstein 6"), "target": "wheel"}, {"t": 6, "kind": "motion", "title": T("Wheel near the end of kerb 8", "Rad nahe dem Ende von Bordstein 8")}], "tracks": []},
    ]


AREAS = {"kerb-granite": 7.93, "mortar-remnants": 0.47, "timber-bearers": 1.26, "concrete-bed": 5.05, "lifting-gear": 0.4}


def thermal():
    """🌡️ The thermal model: five passive surface regions of real materials on a mild sunny day."""
    reg = lambda id, title, material, char, geom, sky, cap, ref: {"id": id, "title": title, "kind": "surface", "model": "passive", "material": material, "areaM2": AREAS[id], "characteristicLengthM": char, "geometry": geom, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": ref}
    return {
        "environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
        "operating": [],
        "regions": [
            reg("kerb-granite", T("Granite kerb faces (weathered, bush-hammered top and visible face, split back and bottom)", "Granitbordsteinflächen (verwittert, Oberseite und Ansichtsfläche gestockt, Rücken und Lager gespalten)"), "granite", 0.3, "vertical-plate", 0.5, 198750, "en-iso-13786"),
            reg("mortar-remnants", T("Old mortar and bed-concrete remnants", "Alte Mörtel- und Bettungsbetonreste"), "old-mortar", 0.2, "vertical-plate", 0.2, 36000, "en-iso-10456"),
            reg("timber-bearers", T("Weathered bearers and battens", "Verwitterte Unterleger und Latten"), "softwood", 0.1, "vertical-plate", 0.4, 24200, "en-iso-13786"),
            reg("concrete-bed", T("Concrete bed and back haunch", "Betonbett und Rückenstütze"), "bed-concrete", 0.45, "horizontal-plate-up", 0.9, 181700, "en-iso-13786"),
            reg("lifting-gear", T("Kerb clamp and lifting chain", "Bordsteinzange und Hebekette"), "clamp-steel", 0.05, "vertical-plate", 0.6, 26500, "en-iso-10456"),
        ],
        "emitters": [],
        "assumptions": [
            T("Passive granite stores heat in about 75 mm, half the kerb width: the 24 h penetration depth is 170 mm for lambda 2.8 W/mK, rho 2650 kg/m3 and c 1000 J/kgK (EN ISO 13786), so half of it exceeds the half-width of the kerb and the half-width is taken.", "Passiver Granit speichert Wärme in etwa 75 mm, der halben Steinbreite: die 24-h-Eindringtiefe beträgt 170 mm für Lambda 2,8 W/mK, Rho 2650 kg/m³ und c 1000 J/kgK (EN ISO 13786); ihre Hälfte übersteigt die halbe Steinbreite, daher gilt diese."),
            T("Weathered surfaces are darker than new ones: weathered grey granite absorbs about 0.55 of the sun against about 0.45 for a freshly tooled face, weathered grey softwood about 0.7 against 0.6 for new timber; these are authored estimates guided by the ASHRAE Handbook, not measurements.", "Verwitterte Flächen sind dunkler als neue: verwitterter grauer Granit absorbiert etwa 0,55 der Sonne gegen etwa 0,45 bei frisch bearbeiteter Fläche, vergrautes Nadelholz etwa 0,7 gegen 0,6 bei neuem Holz; das sind Schätzungen nach dem ASHRAE Handbook, keine Messungen."),
            T("The mortar remnants are thin crusts of 20 mm: their heat capacity is their whole thickness (36 kJ/m2K).", "Die Mörtelreste sind dünne Krusten von 20 mm: ihre Wärmekapazität ist ihre ganze Dicke (36 kJ/m²K)."),
            T("Exposed shares follow the pose: the stack shades its inner faces, the reset kerbs lose their bedded bottom and back, the mortar remnants of the reset kerbs are mostly embedded in the fresh concrete.", "Die freien Anteile folgen der Stellung: der Stapel verschattet seine inneren Flächen, die versetzten Steine verlieren ihr gebettetes Lager und ihren Rücken, die Mörtelreste der versetzten Steine stecken größtenteils im frischen Beton."),
            T("Nothing is heated or operated: every element is passive with no nameplate power.", "Nichts wird beheizt oder betrieben: jedes Element ist passiv ohne Nennleistung."),
            T("Reference case: a mild sunny day, all values authored estimates.", "Referenzfall: ein milder sonniger Tag, alle Werte sind Schätzungen."),
        ],
        "passengers": [],
    }


def structure():
    """🏗️ Rigid-body statics on the ground contacts of bearers and bed, the hook as an applied force."""
    return {"gravityMS2": G, "dynamics": "quasi-static", "supports": CONTACTS, "assumptions": [
        T("Stored: the self-weight of the eight kerbs goes through the battens and the two 100 x 100 mm bearers to the yard; each bearer bears on a 0.10 x 0.95 m strip.", "Gelagert: das Eigengewicht der acht Steine geht über die Latten und die beiden Kanthölzer 100 x 100 mm in den Hof; jeder Unterleger liegt auf einem Streifen 0,10 x 0,95 m auf."),
        T("Lifted: the hook force equals the weight of kerb 5 with its remnants, the clamp and the chain (section times density); the bearers carry the rest. The clamp is a non-fixed load lifting attachment to EN 13155, its grip is friction of the rubber pads on the tooled faces.", "Gehoben: die Hakenkraft ist das Gewicht von Bordstein 5 mit seinen Resten, Zange und Kette (Querschnitt mal Dichte); die Unterleger tragen den Rest. Die Zange ist ein loses Lastaufnahmemittel nach EN 13155, sie hält durch Reibung der Gummibeläge auf den bearbeiteten Flächen."),
        T("Reset: the three kerbs stand on a 200 mm bed of C20/25 with a 150 mm back haunch (DIN 18318); the wheel load of 45 kN (90 kN axle, EN 1991-1-1 category G) goes through the kerb into the bed and the formation.", "Versetzt: die drei Steine stehen auf einem Bett von 200 mm C20/25 mit 150 mm Rückenstütze (DIN 18318); die Radlast von 45 kN (Achse 90 kN, EN 1991-1-1 Kategorie G) geht über den Stein in das Bett und das Planum."),
        T("The solver treats the whole asset as one rigid body on its ground contacts: the stack and the bed share the self-weight by their lever arms, which for two groups each standing over its own contacts gives each group its own weight.", "Der Löser behandelt das ganze Asset als einen starren Körper auf seinen Bodenkontakten: Stapel und Bett teilen das Eigengewicht nach ihren Hebelarmen, was für zwei Gruppen über jeweils eigenen Kontakten jeder Gruppe ihr eigenes Gewicht zuweist."),
        T("Granite: apparent density 2650 kg/m3 (EN 1936 test, typical German granite), E about 50 GPa and a flexural strength of about 12 MPa (EN 12372 typical, authored estimate).", "Granit: Rohdichte 2650 kg/m³ (Prüfung EN 1936, typischer deutscher Granit), E etwa 50 GPa und Biegefestigkeit etwa 12 MPa (typisch nach EN 12372, Schätzung)."),
    ], "clinical": False}


def ref(id, solid, **label):
    """🔗️ A reference solid of the thermal or structure list."""
    return {"id": id, "of": solid, **label}


def thermal_list(scale):
    """🌡️ Every body solid is its own thermal surface, bound to the region of its material."""
    return [ref(f"th-{s.id}", s.id, region=s.region) for s in body(scale)]


def structure_list(scale):
    """🏗️ Members on the kerbs, timber, bed, clamp and chain; supports at the four ground contacts; the wheel-carrier joint; masses at the centres of kerbs, bed and clamp."""
    f = FEATURE[scale]
    out = []
    masses = mass_by_bone()
    for s in body(scale):
        if s.material == "old-mortar":
            continue
        part = "joint" if s.id in ("pivot-pin", "eye-pin") else "member"
        label = {"member": s.bone, "part": part}
        if s.id == "clamp-block":
            label["members"] = ["clamp-arm-a", "clamp-arm-b"]
        out.append(ref(f"st-{s.id}", s.id, **label))
    if scale == 500:
        for i, x in enumerate(BEARER_X):
            out.append({"id": f"st-batten-{'ab'[i]}", "kind": "box", "bone": "battens", "material": "softwood", "role": "body", "tone": 0.5, "slender": True, "size": list(BATTEN), "transform": {"translation": [x, BATTEN[1] / 2, 0]}, "member": "battens", "part": "member"})
    for i, x in enumerate(BEARER_X):
        out.append({"id": f"support-bearer-{'ab'[i]}", "kind": "box", "bone": "bearers", "material": "softwood", "role": "body", "tone": 0.2, "size": [BEARER[0], f, BEARER[2]], "transform": {"translation": [x, r(f / 2), 0]}, "member": "bearers", "part": "support", "anchor": f"bearer-{'ab'[i]}-ground"})
    for i, k in enumerate((0.25, 0.75)):
        out.append({"id": f"support-bed-{'ab'[i]}", "kind": "box", "bone": "bed", "material": "bed-concrete", "role": "body", "tone": 0.2, "size": [r(BED_LEN / 2), f, BED_W], "transform": {"translation": [r(BED_LEN * k), r(f / 2), 0]}, "member": "bed", "part": "support", "anchor": f"bed-ground-{'ab'[i]}"})
    out.append({"id": "joint-wheel-carrier", "kind": "box", "bone": "wheel-carrier", "material": "granite", "role": "body", "tone": 0.3, "size": [0.10, f, 0.10], "transform": {"translation": [0, r(-f / 2), 0]}, "member": "wheel-carrier", "part": "joint"})
    rad = max(f / 2, 0.02)
    for b in [k[0] for k in KERBS] + ["bed"]:
        m, cu, cv = masses[b]
        out.append({"id": f"mass-{b}", "kind": "ellipsoid", "bone": b, "material": "granite" if b.startswith("kerb") else "bed-concrete", "role": "body", "tone": 0.5, "radii": [rad, rad, rad], "transform": {"translation": [r(cu, 4), r(cv, 4), 0]}, "member": b, "part": "mass"})
    return out


REP_DECISIONS = {
    50: {
        "silhouette": T("Side elevation of the stack seen from the visible faces: two layers of upright kerbs 300 mm high (6 mm on paper) on the end faces of the 100 x 100 mm bearers, the 50 mm battens between the layers, the scissor clamp on kerb 5 with its 25 links of 8 mm chain and the master link, the concrete bed and haunch behind. Front: the kerb sections with the worn arris and the back chamfer, the crossed scissor arms, jaws and rubber pads. Plan: the tooled tops of the top layer, the longer bottom kerbs, bearers and battens in the gaps, the bed strip with its haunch.", "Seitenansicht des Stapels von den Ansichtsflächen: zwei Lagen stehender Bordsteine 300 mm hoch (6 mm auf Papier) auf den Stirnseiten der Kanthölzer 100 x 100 mm, die 50-mm-Latten zwischen den Lagen, die Scherenzange auf Bordstein 5 mit 25 Gliedern 8-mm-Kette und Aufhängering, dahinter Betonbett und Rückenstütze. Vorderansicht: die Steinquerschnitte mit abgenutzter Kante und hinterer Fase, die gekreuzten Scherenarme, Backen und Gummibeläge. Draufsicht: die gestockten Oberseiten der oberen Lage, die längeren unteren Steine, Unterleger und Latten in den Fugen, der Bettstreifen mit Rückenstütze."),
        "retained": T("The rounded worn arris (r 20 mm) as a line 20 mm below the top, the edge of the bush-hammered band 200 mm below the top, the 10 mm back chamfer in plan, every mortar remnant at its place, both pins, the eye plate and each chain link.", "Die gerundete abgenutzte Kante (r 20 mm) als Linie 20 mm unter der Oberkante, die Grenze des gestockten Bands 200 mm unter der Oberkante, die hintere Fase 10 mm in der Draufsicht, jeder Mörtelrest an seinem Ort, beide Bolzen, die Ösenplatte und jedes Kettenglied."),
        "merged": T("Each kerb is one closed shape with its arris in the outline; each remnant, bearer, batten and clamp part is its own closed shape.", "Jeder Stein ist eine geschlossene Form mit der Kante im Umriss; jeder Rest, jedes Kantholz, jede Latte und jedes Zangenteil ist eine eigene geschlossene Form."),
        "omitted": T("The split relief of the backs and bottoms (a few millimetres), the individual hammer marks and the chain shackle threads.", "Das Spaltrelief von Rücken und Lager (wenige Millimeter), die einzelnen Stockhiebe und die Gewinde der Kettenverbindungsglieder."),
        "thinElements": T("The 10 mm rubber pads (0.2 mm) and the 8 mm chain wire (0.16 mm) are drawn at their true size inside the 0.35 mm outline pen.", "Die 10-mm-Gummibeläge (0,2 mm) und der 8-mm-Kettendraht (0,16 mm) sind in wahrer Größe innerhalb der 0,35-mm-Konturfeder gezeichnet."),
        "hatching": T("A stipple of 0.8 mm on the kerb faces in elevation and plan stands for the bush-hammered granite; timber, concrete and steel carry tones only, nothing is cut.", "Eine Punktierung von 0,8 mm auf den Steinflächen in Ansicht und Draufsicht steht für den gestockten Granit; Holz, Beton und Stahl tragen nur Töne, nichts ist geschnitten."),
        "styles": T("Outline 0.35 mm, arris and chamfer lines 0.18 mm, tooling edge and timber checks 0.1 mm.", "Kontur 0,35 mm, Kanten- und Faslinien 0,18 mm, Bearbeitungsgrenze und Holzrisse 0,1 mm."),
        "overlays": T("Every kerb, remnant, bearer, batten, bed part and clamp part is its own cell.", "Jeder Stein, Rest, Unterleger, jede Latte, jedes Bett- und Zangenteil ist eine eigene Zelle."),
    },
    100: {
        "silhouette": T("The same stack at 3 mm per kerb height: kerbs with the worn arris as a 20 mm chamfer, the remnants, bearers and battens, the clamp with arms, links, eye and jaws, the chain as one 27 mm rod under the master link, bed and haunch.", "Derselbe Stapel mit 3 mm Steinhöhe: Steine mit der abgenutzten Kante als 20-mm-Fase, Reste, Unterleger und Latten, die Zange mit Armen, Laschen, Öse und Backen, die Kette als ein 27-mm-Stab unter dem Aufhängering, Bett und Rückenstütze."),
        "retained": T("Arris line and tooling edge in elevation, the arris line in plan, every remnant of 20 mm.", "Kantenlinie und Bearbeitungsgrenze in der Ansicht, die Kantenlinie in der Draufsicht, jeder 20-mm-Rest."),
        "merged": T("The rubber pads are merged into 25 mm jaws; the 25 links into one chain rod of their envelope.", "Die Gummibeläge gehen in 25-mm-Backen auf, die 25 Glieder in einen Kettenstab ihrer Hüllform."),
        "omitted": T("The 10 mm back chamfer (0.1 mm) and the end checks of the bearers.", "Die hintere Fase 10 mm (0,1 mm) und die Stirnrisse der Unterleger."),
        "thinElements": T("Pins of 25 mm and the 15 mm arms and links stay at their true size; the chain rod is 0.27 mm.", "Bolzen 25 mm und die 15-mm-Arme und Laschen bleiben in wahrer Größe; der Kettenstab misst 0,27 mm."),
        "hatching": T("A stipple of 0.6 mm on the kerb faces in elevation and plan.", "Eine Punktierung von 0,6 mm auf den Steinflächen in Ansicht und Draufsicht."),
        "styles": T("Outline 0.25 mm, arris line 0.13 mm, tooling edge 0.1 mm.", "Kontur 0,25 mm, Kantenlinie 0,13 mm, Bearbeitungsgrenze 0,1 mm."),
        "overlays": T("Kerbs, remnants, timber, bed parts and the clamp parts are separate cells.", "Steine, Reste, Holz, Bettteile und Zangenteile sind getrennte Zellen."),
    },
    200: {
        "silhouette": T("Square kerb sections 150 x 300 mm (0.75 x 1.5 mm) in two layers on bearers and battens; the clamp as frame, two arms, two jaws and a head; the chain as a rod to the top of the master link; bed and haunch.", "Quadratische Steinquerschnitte 150 x 300 mm (0,75 x 1,5 mm) in zwei Lagen auf Unterlegern und Latten; die Zange als Rahmen, zwei Arme, zwei Backen und Kopf; die Kette als Stab bis zum oberen Ende des Aufhängerings; Bett und Rückenstütze."),
        "retained": T("The 50 mm joints between the kerbs and the battens between the layers, the true chain length, the gaps of the reset row.", "Die 50-mm-Fugen zwischen den Steinen und die Latten zwischen den Lagen, die wahre Kettenlänge, die Fugen der versetzten Reihe."),
        "merged": T("Stop plate, stem and pivot into the frame; links, eye and pin into the head; master link into the chain rod.", "Anschlagplatte, Schaft und Drehpunkt in den Rahmen; Laschen, Öse und Bolzen in den Kopf; Aufhängering in den Kettenstab."),
        "omitted": T("Arris, chamfer and tooling lines (0.1 mm and less), the mortar remnants (20 mm, 0.1 mm), the hatch.", "Kanten-, Fas- und Bearbeitungslinien (0,1 mm und weniger), die Mörtelreste (20 mm, 0,1 mm), die Schraffur."),
        "thinElements": T("The chain rod of 27 mm is 0.135 mm wide, closed by the 0.18 mm outline pen; in plan it lies inside the clamp head.", "Der Kettenstab von 27 mm ist 0,135 mm breit und wird von der 0,18-mm-Konturfeder geschlossen; in der Draufsicht liegt er im Zangenkopf."),
        "hatching": T("No hatch: a 0.75 mm wide kerb carries no printable stipple.", "Keine Schraffur: ein 0,75 mm breiter Stein trägt keine druckbare Punktierung."),
        "styles": T("Outline 0.18 mm.", "Kontur 0,18 mm."),
        "overlays": T("Each kerb, bearer, batten and bed part is one cell; the clamp has five cells.", "Jeder Stein, Unterleger, jede Latte und jedes Bettteil ist eine Zelle; die Zange hat fünf Zellen."),
    },
    500: {
        "silhouette": T("Eight kerb blocks (0.3 x 0.6 mm), the bottom ones up to the top of the batten layer, on the two bearer ends; the clamp as one block of 150 x 560 mm on kerb 5, the chain as a single stroke; the bed as one L-section with its haunch.", "Acht Steinblöcke (0,3 x 0,6 mm), die unteren bis zur Oberkante der Lattenlage, auf den beiden Unterlegerenden; die Zange als ein Block 150 x 560 mm auf Bordstein 5, die Kette als ein einzelner Strich; das Bett als ein L-Querschnitt mit Rückenstütze."),
        "retained": T("The eight kerbs as separate blocks with their true lengths, the two layers, the bearers, the clamp and chain up to the true overall height of 2.01 m, the three reset kerbs as separate blocks in the bed.", "Die acht Steine als getrennte Blöcke mit ihren wahren Längen, die zwei Lagen, die Unterleger, Zange und Kette bis zur wahren Gesamthöhe von 2,01 m, die drei versetzten Steine als getrennte Blöcke im Bett."),
        "merged": T("The 50 mm batten layer into the bottom kerbs, the clamp parts into one block, bed and haunch into one section.", "Die 50-mm-Lattenlage in die unteren Steine, die Zangenteile in einen Block, Bett und Rückenstütze in einen Querschnitt."),
        "omitted": T("Remnants, arris, the clamp jaws beside kerb 5 (25 mm), the chain in plan.", "Reste, Kanten, die Zangenbacken neben Bordstein 5 (25 mm), die Kette in der Draufsicht."),
        "thinElements": T("The chain of 27 mm is one 0.13 mm stroke; the joints of 50 mm are closed by the outline pens.", "Die Kette von 27 mm ist ein Strich von 0,13 mm; die 50-mm-Fugen schließen sich in den Konturfedern."),
        "hatching": T("No hatch at this size.", "Keine Schraffur in dieser Größe."),
        "styles": T("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": T("One cell per kerb, bearer, bed and the clamp.", "Eine Zelle je Stein, Unterleger, Bett und Zange."),
    },
}

VOL_DECISIONS = {
    50: [
        T("Body: each kerb is one extrude of its true section, 150 x 300 mm with the worn visible-face arris rounded to r 20 mm and a 10 mm chamfer at the back top edge, over its true length of 0.80 to 1.20 m; the eight mortar and bed-concrete remnants are 20 mm crusts on the bottoms and backs where the old bed and haunch held them. Bearers 100 x 100 x 950 mm and battens 50 x 80 x 850 mm are boxes; bed and haunch are a 200 mm box and a trapezoid extrude.", "Körper: jeder Stein ist ein Extrusionskörper seines wahren Querschnitts, 150 x 300 mm mit der auf r 20 mm abgenutzten Kante der Ansichtsfläche und einer Fase 10 mm an der hinteren Oberkante, über seine wahre Länge von 0,80 bis 1,20 m; die acht Mörtel- und Bettungsbetonreste sind 20-mm-Krusten an Lager und Rücken, wo das alte Bett und die Rückenstütze sie hielten. Kanthölzer 100 x 100 x 950 mm und Latten 50 x 80 x 850 mm sind Quader; Bett und Rückenstütze sind ein 200-mm-Quader und ein Trapez-Extrusionskörper."),
        T("Clamp: stop plate, stem, pivot pin, two links, eye pin and eye plate on the clamp bone; each scissor arm (flat bar 50 x 15 mm) carries its 15 mm jaw and 10 mm rubber pad on its own bone at the pivot. Chain: one 8 mm link extruded with its hole and repeated 25 times at 24 mm pitch, each copy turned a quarter turn about the chain axis, and a 13 mm master link.", "Zange: Anschlagplatte, Schaft, Drehbolzen, zwei Laschen, Ösenbolzen und Ösenplatte am Zangenknochen; jeder Scherenarm (Flachstahl 50 x 15 mm) trägt seine 15-mm-Backe und den 10-mm-Gummibelag am eigenen Knochen im Drehpunkt. Kette: ein 8-mm-Glied mit Loch extrudiert und 25-mal in 24 mm Teilung wiederholt, jede Kopie eine Vierteldrehung um die Kettenachse gedreht, und ein 13-mm-Aufhängering."),
        T("Thermal: every body solid is its own surface, bound to the region of its material. Structure: members are the body solids, the pins are joints, the ground contacts have support pads of one smallest feature, masses sit at the centres of the kerbs, the bed and the clamp.", "Thermik: jeder Körper ist seine eigene Fläche, gebunden an die Region seines Materials. Tragwerk: Stäbe sind die Körper, die Bolzen sind Gelenke, die Bodenkontakte haben Auflagerplatten von einem kleinsten Merkmal, Massen sitzen in den Mitten von Steinen, Bett und Zange."),
    ],
    100: [
        T("Body: the arris becomes a 20 mm chamfer and the 10 mm back chamfer is left out (below 15 mm); remnants, timber and bed as at 1:50; the jaws take the rubber pads in (25 mm); the 25 chain links are one 27 mm rod of their envelope (declared slender, 22:1) and the master link keeps its 15 mm ring.", "Körper: die Kante wird eine 20-mm-Fase, die hintere Fase 10 mm entfällt (unter 15 mm); Reste, Holz und Bett wie bei 1:50; die Backen nehmen die Gummibeläge auf (25 mm); die 25 Kettenglieder sind ein 27-mm-Stab ihrer Hüllform (schlank erklärt, 22:1), der Aufhängering behält seinen 15-mm-Ring."),
        T("Thermal and structure as at 1:50 on the solids of this scale.", "Thermik und Tragwerk wie bei 1:50 auf den Körpern dieses Maßstabs."),
    ],
    200: [
        T("Body: square kerb sections (the 20 mm arris is below 30 mm); remnants stay as texture crusts so the mortar region keeps its surface; bearers, battens, bed and haunch as boxes and extrude; the clamp is a T-shaped frame (stop plate, stem, pivot), a pentagonal head (links, eye, pin), two 30 mm arms and two 30 mm jaws; the chain rod runs to the top of the master link.", "Körper: quadratische Steinquerschnitte (die 20-mm-Kante liegt unter 30 mm); die Reste bleiben als Texturkrusten, damit die Mörtelregion ihre Fläche behält; Unterleger, Latten, Bett und Rückenstütze als Quader und Extrusion; die Zange ist ein T-förmiger Rahmen (Anschlagplatte, Schaft, Drehpunkt), ein fünfeckiger Kopf (Laschen, Öse, Bolzen), zwei 30-mm-Arme und zwei 30-mm-Backen; der Kettenstab reicht bis zum oberen Ende des Aufhängerings."),
        T("Thermal and structure on the solids of this scale; frame and head are members of the clamp, arms and jaws of their arm bones.", "Thermik und Tragwerk auf den Körpern dieses Maßstabs; Rahmen und Kopf sind Stäbe der Zange, Arme und Backen ihrer Armknochen."),
    ],
    500: [
        T("Body: the 50 mm batten layer is below 75 mm and is taken into the bottom kerbs, which run up to its top (350 mm); the top kerbs stay separate blocks on it; the clamp is one block of 150 x 560 x 200 mm on kerb 5; bed and haunch are one L-section; the chain is the 27 mm rod at its true section (slender, 27:1); remnants are texture crusts.", "Körper: die 50-mm-Lattenlage liegt unter 75 mm und geht in die unteren Steine auf, die bis zu ihrer Oberkante reichen (350 mm); die oberen Steine bleiben getrennte Blöcke darauf; die Zange ist ein Block 150 x 560 x 200 mm auf Bordstein 5; Bett und Rückenstütze sind ein L-Querschnitt; die Kette ist der 27-mm-Stab im wahren Querschnitt (schlank, 27:1); die Reste sind Texturkrusten."),
        T("Structure: the battens keep two slender members of their true 50 x 80 mm section; the clamp block is the member of the clamp and of both arm bones.", "Tragwerk: die Latten behalten zwei schlanke Stäbe ihres wahren Querschnitts 50 x 80 mm; der Zangenblock ist der Stab der Zange und beider Armknochen."),
    ],
}

PROVENANCE = {
    "author": "Semio staffage component authors",
    "method": "handcrafted",
    "created": "2026-11-22",
    "references": [
        {"id": "din-482", "citation": "DIN 482:2002-06 Straßenbordsteine aus Naturstein (withdrawn, superseded by DIN EN 1343): form A, raised kerb (Hochbord) of 150 mm width and 300 mm height with a tooled top and visible face; the historic profile of the reclaimed kerbs."},
        {"id": "en-1343", "citation": "EN 1343:2012 Kerbs of natural stone for external paving: requirements and declared values (dimensions, flexural strength to EN 12372, apparent density to EN 1936)."},
        {"id": "din-18318", "citation": "DIN 18318:2019-09 VOB Part C, Traffic routes construction, paving and slab covering, kerbs and edgings: kerbs set in a concrete bed with back haunch, concrete at least C20/25, bed and haunch thickness to the design (200 mm bed and 150 mm haunch taken here)."},
        {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties, Table 3: granite 2500 to 2700 kg/m3, lambda 2.8 W/mK, c 1000 J/kgK; cement mortar 1800 kg/m3, lambda 1.0 W/mK; high-density concrete 2400 kg/m3, lambda 2.0 W/mK; rubber 1200 kg/m3, lambda 0.17 W/mK; steel lambda 50 W/mK, c 450 J/kgK."},
        {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); about half of it is taken as the effective storage depth of an exposed face, at most the half thickness of the part."},
        {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes, Table 1: C24 mean density 420 kg/m3, E0,mean 11 GPa, f_m,k 24 MPa."},
        {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1, Part 1-1, Table A.1 (normal-weight concrete 24 kN/m3) and Table 6.8 (traffic category G, axle load 90 kN, wheel 45 kN)."},
        {"id": "en-1992-1-1", "citation": "EN 1992-1-1:2004 Eurocode 2, Table 3.1: C20/25 with f_ck 20 MPa and E_cm 30 GPa."},
        {"id": "en-10025-2", "citation": "EN 10025-2:2019 Hot rolled products of structural steels, non-alloy structural steels (S355, yield strength 355 MPa)."},
        {"id": "en-13155", "citation": "EN 13155:2020 Cranes, safety, non-fixed load lifting attachments: clamps for stone kerbs, working load limit and grip by friction; clamp geometry and mass of 16 kg are authored estimates of a manual scissor kerb clamp."},
        {"id": "en-818-2", "citation": "EN 818-2:2008 Short link chain for lifting purposes, grade 8: 8 mm chain, pitch 24 mm, outer width about 27 mm, about 1.4 kg/m."},
        {"id": "ashrae-fundamentals", "citation": "ASHRAE Handbook Fundamentals (2021), thermal radiation properties of building materials (typical emissivity and solar absorptance of stone, concrete, timber and painted steel; rounded values guided by this chapter, not table look-ups)."},
        {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on sections and cuts (hatching)."},
        {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (scales, line groups)."},
    ],
    "notes": T("Kerbs 150 x 300 mm of 0.80 to 1.20 m, 2650 kg/m3, 118.9 kg per metre with the worn arris; stack of 2 x 4 on two bearers 100 x 100 mm and battens 50 x 80 mm, 50 mm joints; scissor clamp at the middle of kerb 5 with 0.60 m of 8 mm chain; reset row of 1.15 + 0.80 + 1.10 m with 10 mm joints in a 3.25 m bed of C20/25. Lengths, remnant positions and clamp geometry are authored estimates.", "Bordsteine 150 x 300 mm von 0,80 bis 1,20 m, 2650 kg/m³, 118,9 kg je Meter mit abgenutzter Kante; Stapel 2 x 4 auf zwei Kanthölzern 100 x 100 mm und Latten 50 x 80 mm, Fugen 50 mm; Scherenzange in der Mitte von Bordstein 5 mit 0,60 m 8-mm-Kette; versetzte Reihe 1,15 + 0,80 + 1,10 m mit 10-mm-Fugen in einem 3,25-m-Bett aus C20/25. Längen, Lage der Reste und Zangengeometrie sind Schätzungen."),
}


def extents(scale):
    """📦️ The rest bounding box of the body of a scale in world metres."""
    lo, hi = [1e9] * 3, [-1e9] * 3
    for s in body(scale):
        for axis, prof, holes, rng in s.copies():
            for a, b in prof:
                for c in rng:
                    p = to_world(s.bone, *local_point(axis, a, b, c))
                    for k in range(3):
                        lo[k], hi[k] = min(lo[k], p[k]), max(hi[k], p[k])
    return lo, hi


def document():
    """📄️ The whole asset document."""
    build_rig()
    masses = finish_rig()
    total = sum(m for m, _, _ in masses.values())
    hung = sum(masses[b][0] for b in ("kerb-5", "clamp", "clamp-arm-a", "clamp-arm-b", "sling")) * G
    lo, hi = extents(50)
    reps, vols = {}, {}
    for scale in SCALES:
        reps[str(scale)] = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": RANGES[scale][0], "maxDenominator": RANGES[scale][1]}, "pens": PENS[scale], "hatches": HATCHES[scale], "geometry": geometry(scale), "decisions": REP_DECISIONS[scale]}
        vols[str(scale)] = {"status": "authored", "denominator": scale, "body": [s.doc() for s in body(scale)], "thermal": thermal_list(scale), "structure": structure_list(scale), "decisions": VOL_DECISIONS[scale]}
    asset = {
        "id": "reclaimed-granite-kerbs",
        "category": "components",
        "title": T("Reclaimed Granite Kerbs", "Gebrauchte Granitbordsteine"),
        "summary": T("Eight reclaimed granite kerbs of the historic DIN 482 form A, 150 x 300 mm and 0.80 to 1.20 m long, with bush-hammered tops and visible faces, worn arrises and crusts of the old bed concrete, stored upright in two layers on squared-timber bearers; one is lifted in a scissor kerb clamp, three are reset as a row in a new concrete bed with back haunch under a 45 kN wheel.", "Acht gebrauchte Granitbordsteine der historischen DIN 482 Form A, 150 x 300 mm und 0,80 bis 1,20 m lang, mit gestockter Oberseite und Ansichtsfläche, abgenutzten Kanten und Krusten des alten Bettungsbetons, stehend in zwei Lagen auf Kanthölzern gelagert; einer wird in einer Scheren-Bordsteinzange gehoben, drei sind als Reihe in einem neuen Betonbett mit Rückenstütze unter einem Rad von 45 kN versetzt."),
        "tags": ["component", "reclaimed", "granite", "natural-stone", "kerb", "paving", "landscape", "lifting"],
        "provenance": PROVENANCE,
        "dimensions": {"heightM": r(hi[1] - lo[1], 3), "widthM": r(hi[0] - lo[0], 3), "depthM": r(hi[2] - lo[2], 3), "massKg": r(total, 2)},
        "materials": MATERIALS,
        "rig": {"kind": "mechanical", "root": "bearers", "bones": BONES},
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(hung),
        "clips": clips(),
        "thermal": thermal(),
        "structure": structure(),
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}, masses, hung, (lo, hi)


if __name__ == "__main__":
    doc, masses, hung, (lo, hi) = document()
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    a = doc["asset"]
    print("[DEBUG] dimensions", a["dimensions"], "extent", [round(v, 3) for v in lo], [round(v, 3) for v in hi])
    print("[DEBUG] hook N", round(hung, 1))
    for b, (m, cu, cv) in masses.items():
        print("[DEBUG] mass", b, round(m, 3), round(cu, 3), round(cv, 3))
