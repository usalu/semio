"""🌀 Round 3 heat pump outdoor unit (12 kW single-fan air-to-water monobloc): reference numbers, every part of every scale stated by hand with its solid and its 2D shapes, the root members, the candidate build and `--apply`."""
import json
import math
import pathlib
import sys

T = pathlib.Path(__file__).resolve().parent
R = T.parents[6]
CATALOG = R / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
ASSET_FILE = CATALOG / "🏗️machinery/heat-pump-outdoor-unit/🔣️.json"
MANIFEST_FILE = CATALOG / "🔣️.json"
SOURCE_FILE = T / "volumes/machinery/heat-pump-outdoor-unit.json"
OUT = T / "🗑️generated/r3-heat-pump-outdoor-unit"

XB, XF = -0.215, 0.215
ZR, ZL = -0.55, 0.55
ZP0, ZP1 = 0.2425, 0.25
ZC = 0.2525
Y_TRAY, Y_BASE, Y_TOP = 0.09, 0.15, 1.425
LID_X, LID_Y, LID_L, LID_W = -0.235, 1.41, 0.47, 0.5575
LID_TOP = 0.04
FY, FZ = 0.80, -0.15
HOLE_R, RIM_R = 0.30, 0.312
COIL_Y = (0.165, 1.40)
FEET_Z = 0.43


def r5(value):
    rounded = round(value + 0.0, 5)
    return int(rounded) if rounded == int(rounded) else rounded


def pts(points):
    return [[r5(a), r5(b)] for a, b in points]


def circle(cx, cy, r, n=48, start=0.0):
    return [(cx + r * math.cos(start + 2 * math.pi * i / n), cy + r * math.sin(start + 2 * math.pi * i / n)) for i in range(n)]


def arc(cx, cy, r, a0, a1, n):
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def quat(m):
    """🧮 Unit quaternion [x, y, z, w] of a rotation matrix given by its three columns."""
    (a, d, g), (b, e, h), (c, f, i) = m
    tr = a + e + i
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        w, x, y, z = 0.25 * s, (h - f) / s, (c - g) / s, (d - b) / s
    elif a > e and a > i:
        s = math.sqrt(1.0 + a - e - i) * 2
        w, x, y, z = (h - f) / s, 0.25 * s, (b + d) / s, (c + g) / s
    elif e > i:
        s = math.sqrt(1.0 + e - a - i) * 2
        w, x, y, z = (c - g) / s, (b + d) / s, 0.25 * s, (f + h) / s
    else:
        s = math.sqrt(1.0 + i - a - e) * 2
        w, x, y, z = (d - b) / s, (c + g) / s, (f + h) / s, 0.25 * s
    n = math.sqrt(w * w + x * x + y * y + z * z)
    return [r5(x / n), r5(y / n), r5(z / n), r5(w / n)]


def qx(deg):
    h = math.radians(deg) / 2
    return [r5(math.sin(h)), 0, 0, r5(math.cos(h))]


class Scale:
    """🧱 Part collector of one scale: body solids with their thermal regions, own thermal and structure solids, and the shapes of the three viewpoints."""

    def __init__(self, denominator):
        self.d = denominator
        self.body, self.regions, self.thermal_own, self.structure = [], {}, [], []
        self.views = {"elevation": [], "front": [], "plan": []}

    def add(self, solid, region=None, **flags):
        solid.update({k: v for k, v in flags.items() if v is not None})
        self.body.append(solid)
        if region:
            self.regions[solid["id"]] = region
        return solid

    def own(self, solid, region, tags=None):
        solid = {**solid, "region": region}
        if tags:
            solid["tags"] = tags
        self.thermal_own.append(solid)

    def member(self, solid, member, part, anchor=None):
        solid = {**solid, "member": member, "part": part}
        if anchor:
            solid["anchor"] = anchor
        self.structure.append(solid)

    def ref(self, id, of, member, part):
        self.structure.append({"id": id, "of": of, "member": member, "part": part})

    def shape(self, view, shape):
        self.views[view].append(shape)

    def thermal(self):
        return [{"id": "th-" + s["id"], "of": s["id"], "region": self.regions[s["id"]]} for s in self.body if s["id"] in self.regions] + self.thermal_own

    def volume(self, decisions):
        return {"status": "authored", "denominator": self.d, "body": self.body, "thermal": self.thermal(), "structure": self.structure, "decisions": decisions}


def base(id, material, tone, role="body", bone="unit"):
    return {"id": id, "kind": None, "bone": bone, "material": material, "role": role, "tone": tone}


def box(id, x, y, z, material, tone, role="body", bone="unit", radius=0, rotation=None, repeat=None):
    s = {**base(id, material, tone, role, bone), "kind": "box", "size": [r5(x[1] - x[0]), r5(y[1] - y[0]), r5(z[1] - z[0])]}
    if radius:
        s["cornerRadius"] = radius
    s["transform"] = {"translation": [r5((x[0] + x[1]) / 2), r5((y[0] + y[1]) / 2), r5((z[0] + z[1]) / 2)]}
    if rotation:
        s["transform"]["rotation"] = rotation
    if repeat:
        s["repeat"] = repeat
    return s


def ext(id, plane, profile, lo, hi, material, tone, role="body", bone="unit", holes=None, repeat=None):
    s = {**base(id, material, tone, role, bone), "kind": "extrude", "profile": pts(profile)}
    if holes:
        s["holes"] = [pts(h) for h in holes]
    s.update({"plane": plane, "from": r5(lo), "to": r5(hi)})
    if repeat:
        s["repeat"] = repeat
    return s


def rev(id, profile, axis, origin, material, tone, role="body", bone="unit", repeat=None):
    s = {**base(id, material, tone, role, bone), "kind": "revolve", "profile": pts(profile), "axis": axis, "origin": [r5(v) for v in origin]}
    if repeat:
        s["repeat"] = repeat
    return s


def cap(id, p0, p1, r, material, tone, role="detail", bone="unit", repeat=None, slender=True):
    s = {**base(id, material, tone, role, bone), "kind": "capsule", "p0": [r5(v) for v in p0], "p1": [r5(v) for v in p1], "r0": r, "r1": r}
    if slender:
        s["slender"] = True
    if repeat:
        s["repeat"] = repeat
    return s


def torus(id, axial, radius, wire, origin, material, tone, role="detail"):
    loop = [(axial + wire * math.cos(2 * math.pi * i / 10), radius + wire * math.sin(2 * math.pi * i / 10)) for i in range(10)]
    loop.append(loop[0])
    s = rev(id, loop, [1, 0, 0], origin, material, tone, role)
    s["slender"] = True
    return s


def ring_repeat(count):
    return {"count": count, "rotationRad": r5(2 * math.pi / count), "rotationAxis": [1, 0, 0], "origin": [0, FY, FZ]}


def shape(id, kind, role, z, tone, region=None, regions=None, material=None, filled=True, bone="unit", **geo):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    s["kind"] = kind
    for k, v in geo.items():
        s[k] = pts(v) if k == "points" else (r5(v) if isinstance(v, float) else v)
    if kind == "poly":
        s["closed"] = True
    return s


def rect(id, x0, y0, x1, y1, role, z, tone, **kw):
    return shape(id, "rect", role, z, tone, x=r5(x0), y=r5(y0), w=r5(x1 - x0), h=r5(y1 - y0), **kw)


def poly(id, points, role, z, tone, **kw):
    return shape(id, "poly", role, z, tone, points=points, **kw)


def ellipse(id, cx, cy, r, role, z, tone, **kw):
    return shape(id, "ellipse", role, z, tone, cx=r5(cx), cy=r5(cy), rx=r5(r), ry=r5(r), **kw)


def lid_shape(profile, z, tone=0.35):
    return poly("lid", profile, "body", z, tone, region="top-lid", material="powder-coat", bone="lid") | {"solid": {"kind": "extruded", "sweep": "w", "z0": -LID_W, "z1": LID_W}}


def coil_profile(t):
    """🧊 L-shaped fin pack in the (z, x) plane: back leg across the fan chamber, 70 mm outer bend radius, side leg on the right side."""
    zc, xc = -0.45, -0.205 + 0.07
    outer = [(ZP0, -0.205), (zc, -0.205)] + arc(zc, xc, 0.07, -90, -180, 8)[1:] + [(-0.52, 0.12)]
    inner = [(-0.52 + t, 0.12), (-0.52 + t, xc)] + arc(zc, xc, 0.07 - t, 180, 270, 8)[1:] + [(ZP0, -0.205 + t)]
    return outer + inner


def blade_sections(thick):
    """🪭 Sections of one sickle blade: root at the hub, swept forward, pitch falling from 36 to 21 degrees, chord growing to the tip and rounding off."""
    out = []
    table = [(0.07, 0.050, 36), (0.10, 0.058, 34), (0.14, 0.068, 30), (0.18, 0.078, 27), (0.22, 0.084, 24), (0.25, 0.080, 22), (0.268, 0.062, 21), (0.276, 0.030, 21)]
    for r, half, pitch in table:
        phi = math.radians(8 + 95 * ((r - 0.07) / 0.206) ** 1.6)
        er = (0, math.cos(phi), math.sin(phi))
        et = (0, -math.sin(phi), math.cos(phi))
        b = math.radians(pitch)
        c = (math.sin(b), math.cos(b) * et[1], math.cos(b) * et[2])
        n = (math.cos(b), -math.sin(b) * et[1], -math.sin(b) * et[2])
        at = (0.145, FY + r * er[1], FZ + r * er[2])
        out.append({"at": [r5(v) for v in at], "rx": r5(half), "ry": r5(thick / 2), "exponent": 2.2, "rotation": quat((c, n, er))})
    return out


def blade_outline(sections, k):
    """🪶 Front-view outline (lateral, height) of blade copy k: leading edge out, trailing edge back."""
    lead, trail = [], []
    for s in sections:
        q = s["rotation"]
        x, y, z, w = q
        cy = 2 * (x * y + w * z)
        cz = 2 * (x * z - w * y)
        for sign, side in ((1, lead), (-1, trail)):
            py = s["at"][1] + sign * s["rx"] * cy
            pz = s["at"][2] + sign * s["rx"] * cz
            a = 2 * math.pi * k / 3
            dy, dz = py - FY, pz - FZ
            side.append((FZ + dy * math.sin(a) + dz * math.cos(a), FY + dy * math.cos(a) - dz * math.sin(a)))
    return lead + trail[::-1]


def dome_x(r, peak):
    return 0.2175 + peak * (1 - (r / RIM_R) ** 2)


def dome_outline(peak, wire):
    """⛑️ Side-view outline of the domed fan grille in front of the panel."""
    rs = [RIM_R * (1 - i / 8) for i in range(9)]
    upper = [(dome_x(r, peak) + wire, FY + r) for r in rs]
    lower = [(dome_x(r, peak) + wire, FY - r) for r in reversed(rs[:-1])]
    return [(XF, FY + RIM_R + wire)] + upper + lower + [(XF, FY - RIM_R - wire)]


def feet_profile(ch):
    return [(0, FEET_Z - 0.09), (0, FEET_Z + 0.09), (0.09 - ch, FEET_Z + 0.09), (0.09, FEET_Z + 0.09 - ch), (0.09, FEET_Z - 0.09 + ch), (0.09 - ch, FEET_Z - 0.09)]


def front_feet(sc, ch, z):
    for id, sign in (("foot-left", 1), ("foot-right", -1)):
        p = [(sign * zz, y) for y, zz in feet_profile(ch)]
        sc.shape("front", poly(id, p, "body", z, 0.2, region="feet", material="rubber"))


def emitters(sc, t):
    sc.own(rev("th-discharge-heating", [(0, 0), (0, 0.28), (t, 0.28), (t, 0)], [1, 0, 0], [0.235 - t, FY, FZ], "galvanised-steel", 0.5, "detail"), "fan-guard", ["discharge-heating"])
    sc.own(rev("th-discharge-cooling", [(0, 0), (0, 0.28), (t, 0.28), (t, 0)], [1, 0, 0], [0.235 - 2 * t, FY, FZ], "galvanised-steel", 0.5, "detail"), "fan-guard", ["discharge-cooling"])
    sc.own(box("th-defrost-vapour", (-0.2125, -0.2125 + max(t, 0.0075)), (1.30, 1.30 + max(0.08, t)), (-0.40, 0.15), "aluminium", 0.5, "detail"), "coil-guard", ["defrost-vapour"])


def supports(sc, t):
    for anchor, sign in (("contact-l", 1), ("contact-r", -1)):
        sc.member(box(f"support-{anchor}", (-0.17, 0.17), (0, t), (sign * FEET_Z - 0.09, sign * FEET_Z + 0.09), "rubber", 0.2), "unit", "support", anchor)


def masses(sc, r):
    sc.member({**base("mass-unit", "powder-coat", 0.2), "kind": "ellipsoid", "radii": [r, r, r], "transform": {"translation": [0, 0.55, 0]}}, "unit", "mass")
    sc.member({**base("mass-lid", "powder-coat", 0.2, bone="lid"), "kind": "ellipsoid", "radii": [r, min(r, 0.0225), r], "transform": {"translation": [0.235, 0.0175, 0]}}, "lid", "mass")


def lid_elevation_profile(t):
    return [(0, LID_TOP), (LID_L, LID_TOP), (LID_L, -0.005), (0.02, -0.005), (0.02, LID_TOP - t), (0, LID_TOP - t)]


def s50():
    """🔩 1:50, smallest feature 7.5 mm: every sheet part, the L coil with its louvres and wire guard, the whole fan set."""
    t = 0.0075
    sc = Scale(50)
    m = sc.add
    m(ext("feet", "yz", feet_profile(0.015), -0.17, 0.17, "rubber", 0.2, repeat={"count": 2, "translation": [0, 0, -2 * FEET_Z]}), "feet")
    m(box("base-tray", (XB, XF), (Y_TRAY, Y_BASE), (ZR, ZL), "galvanised-steel", 0.3, radius=0.005), "base-tray")
    hole = circle(FY, FZ, HOLE_R, 48)
    m(ext("front-panel", "yz", [(Y_BASE, ZR), (Y_TOP, ZR), (Y_TOP, ZP0 + 0.005), (Y_BASE, ZP0 + 0.005)], XF - t, XF, "powder-coat", 0.55, holes=[hole]), "casing")
    m(ext("column-panel", "yz", [(Y_BASE, ZP0 + 0.005), (Y_TOP, ZP0 + 0.005), (Y_TOP, ZL), (Y_BASE, ZL)], XF - t, XF, "powder-coat", 0.5), "compressor-casing")
    m(box("side-strip", (0.12, XF - t), (Y_BASE, Y_TOP), (ZR, ZR + t), "powder-coat", 0.55), "casing")
    m(ext("rear-post", "zx", [(ZR, XB), (ZR + 0.06, XB), (ZR + 0.06, XB + t), (ZR + t, XB + t), (ZR + t, XB + 0.06), (ZR, XB + 0.06)], Y_BASE, Y_TOP, "powder-coat", 0.5), "casing")
    m(box("top-panel", (XB, XF), (Y_TOP - t, Y_TOP), (ZR, ZL), "powder-coat", 0.55), "casing")
    m(box("partition", (XB + t, XF - t), (Y_BASE, Y_TOP - t), (ZP0, ZP1), "galvanised-steel", 0.45), "casing")
    m(box("column-back", (XB, XB + t), (Y_BASE, Y_TOP), (ZP0, ZL), "powder-coat", 0.5), "compressor-casing")
    m(box("service-panel", (XB, XF), (Y_BASE, Y_TOP), (ZL - t, ZL), "powder-coat", 0.5), "compressor-casing")
    m(box("service-grip", (0.10, 0.16), (0.98, 1.10), (ZL, ZL + t), "powder-coat", 0.25, "detail", radius=0.003), "compressor-casing")
    m(rev("water-connections", [(0, 0), (0, 0.021), (0.02, 0.021), (0.02, 0)], [1, 0, 0], [LID_X, 0.30, 0.34], "galvanised-steel", 0.35, "detail", repeat={"count": 2, "translation": [0, 0, 0.12]}), "casing")
    m(box("hinge-brackets", (LID_X, XB), (1.40, 1.42), (0.28, 0.34), "powder-coat", 0.35, "detail", repeat={"count": 2, "translation": [0, 0, -0.62]}), "casing")
    m(rev("hinge-knuckles", [(0, 0.009), (0.06, 0.009)], [0, 0, 1], [LID_X, LID_Y, 0.28], "powder-coat", 0.3, "detail", repeat={"count": 2, "translation": [0, 0, -0.62]}), "casing")
    m(ext("coil", "zx", coil_profile(0.04), COIL_Y[0], COIL_Y[1], "aluminium", 0.22, "detail"), "coil-guard")
    m(box("side-louvres", (-0.155, 0.12), (0.18, 0.21), (-0.551, -0.521), "powder-coat", 0.4, "detail", rotation=qx(-40), repeat={"count": 25, "translation": [0, 0.05, 0]}) | {"size": [0.275, t, 0.03], "transform": {"translation": [-0.0175, 0.195, -0.536], "rotation": qx(-40)}}, "coil-guard")
    m(cap("guard-wires-vertical", (-0.2105, 0.17, -0.48), (-0.2105, 1.395, -0.48), 0.002, "galvanised-steel", 0.3, slender=False, repeat={"count": 15, "translation": [0, 0, 0.05]}) | {"texture": True}, "coil-guard")
    m(cap("guard-wires-horizontal", (-0.2145, 0.185, -0.488), (-0.2145, 0.185, 0.2405), 0.002, "galvanised-steel", 0.3, slender=False, repeat={"count": 13, "translation": [0, 0.1, 0]}) | {"texture": True}, "coil-guard")
    bell = [(0.075, 0.345), (0.09, 0.312), (0.11, 0.296), (0.135, 0.29), (0.18, 0.29), (XF - t, 0.298), (XF - t, 0.3055), (0.18, 0.2975), (0.135, 0.2975), (0.112, 0.3035), (0.095, 0.3185), (0.0825, 0.345), (0.075, 0.345)]
    m(rev("bell-mouth", bell, [1, 0, 0], [0, FY, FZ], "galvanised-steel", 0.3, "detail"), "fan-guard")
    m(box("motor-bracket", (0.0, 0.02), (COIL_Y[0], Y_TOP - t), (FZ - 0.125, FZ - 0.095), "galvanised-steel", 0.3, "detail", repeat={"count": 2, "translation": [0, 0, 0.22]}), "casing")
    m(box("motor-plate", (0.0, 0.02), (0.73, 0.87), (FZ - 0.095, FZ + 0.095), "galvanised-steel", 0.3, "detail"), "casing")
    m(rev("fan-motor", [(0.02, 0), (0.02, 0.062), (0.03, 0.07), (0.105, 0.07), (0.105, 0)], [1, 0, 0], [0, FY, FZ], "galvanised-steel", 0.2, "detail"), "fan-guard")
    m(rev("fan-hub", [(0.105, 0), (0.105, 0.075), (0.16, 0.075), (0.178, 0.064), (0.189, 0.042), (0.194, 0)], [1, 0, 0], [0, FY, FZ], "galvanised-steel", 0.2, "detail"), "fan-guard")
    blades = blade_sections(0.008)
    m({**base("fan-blades", "aluminium", 0.45, "detail"), "kind": "loft", "sections": blades, "capStart": True, "capEnd": True, "repeat": ring_repeat(3)}, "fan-guard")
    m(torus("guard-rim", 0.2175, RIM_R, 0.005, [0, FY, FZ], "galvanised-steel", 0.25, "body"), "fan-guard")
    for i, rr in enumerate([0.27, 0.23, 0.19, 0.15, 0.11]):
        m(torus(f"guard-ring-{i + 1}", dome_x(rr, 0.013), rr, 0.0025, [0, FY, FZ], "galvanised-steel", 0.3), "fan-guard")
    m(cap("guard-spokes", (dome_x(0.06, 0.013), FY + 0.06, FZ), (0.2175, FY + RIM_R - 0.004, FZ), 0.0025, "galvanised-steel", 0.3, repeat=ring_repeat(8)), "fan-guard")
    m(rev("guard-cap", [(0.227, 0), (0.227, 0.06), (0.233, 0.058), (0.235, 0.05), (0.235, 0)], [1, 0, 0], [0, FY, FZ], "galvanised-steel", 0.35, "detail"), "fan-guard")
    lid_u = [(-LID_W, 0.02), (-LID_W, LID_L), (LID_W, LID_L), (LID_W, 0.02), (ZL, 0.02), (ZL, LID_L - t), (-ZL, LID_L - t), (-ZL, 0.02)]
    m(box("lid-plate", (0, LID_L), (LID_TOP - t, LID_TOP), (-LID_W, LID_W), "powder-coat", 0.35, bone="lid"), "top-lid")
    m(ext("lid-skirt", "zx", lid_u, -0.005, LID_TOP - t, "powder-coat", 0.35, bone="lid"), "top-lid")
    m(rev("lid-knuckles", [(0, 0.009), (0.06, 0.009)], [0, 0, 1], [0, 0, 0.34], "powder-coat", 0.3, "detail", bone="lid", repeat={"count": 2, "translation": [0, 0, -0.74]}), "top-lid")
    m(box("lid-hinge-leaves", (-0.004, 0.004), (0, LID_TOP - t), (0.34, 0.40), "powder-coat", 0.3, "detail", bone="lid", repeat={"count": 2, "translation": [0, 0, -0.74]}), "top-lid")
    emitters(sc, t)
    for id, x, z in (("post-front-right", (XF - 0.04, XF), (ZR, ZR + 0.04)), ("post-front-left", (XF - 0.04, XF), (ZL - 0.04, ZL)), ("post-rear-left", (XB, XB + 0.04), (ZL - 0.04, ZL))):
        sx, sz = (1 if x[0] < 0 else -1), (1 if z[0] < 0 else -1)
        cz0 = z[0] if sz > 0 else z[1]
        cx0 = x[0] if sx > 0 else x[1]
        lp = [(cz0, cx0), (cz0 + sz * 0.04, cx0), (cz0 + sz * 0.04, cx0 + sx * t), (cz0 + sz * t, cx0 + sx * t), (cz0 + sz * t, cx0 + sx * 0.04), (cz0, cx0 + sx * 0.04)]
        sc.member(ext(id, "zx", lp, Y_BASE, Y_TOP - t, "galvanised-steel", 0.45), "unit", "member")
    sc.ref("post-rear-right", "rear-post", "unit", "member")
    for id, x, z in (("top-rail-front", (XF - 0.04, XF), (ZR, ZL)), ("top-rail-rear", (XB, XB + 0.04), (ZR, ZL)), ("top-rail-right", (XB, XF), (ZR, ZR + 0.04)), ("top-rail-left", (XB, XF), (ZL - 0.04, ZL))):
        sc.member(box(id, x, (Y_TOP - t - 0.04, Y_TOP - t), z, "galvanised-steel", 0.45), "unit", "member")
    sc.ref("base-pan", "base-tray", "unit", "member")
    sc.ref("feet-blocks", "feet", "unit", "member")
    sc.ref("partition-plate", "partition", "unit", "member")
    sc.ref("motor-bracket-bars", "motor-bracket", "unit", "member")
    sc.ref("motor-bracket-plate", "motor-plate", "unit", "member")
    sc.ref("lid-plate-member", "lid-plate", "lid", "member")
    sc.ref("lid-skirt-member", "lid-skirt", "lid", "member")
    sc.ref("hinge-lid", "lid-knuckles", "lid", "joint")
    sc.ref("hinge-casing", "hinge-knuckles", "unit", "joint")
    sc.member(rev("compressor", [(0.17, 0), (0.17, 0.085), (0.5, 0.085), (0.54, 0.05), (0.55, 0)], [0, 1, 0], [-0.02, 0, 0.40], "galvanised-steel", 0.2), "unit", "mass")
    supports(sc, t)
    masses(sc, 0.03)
    e = lambda s: sc.shape("elevation", s)
    e(rect("water-connections", LID_X, 0.279, XB, 0.321, "detail", 1.8, 0.35, region="casing", material="galvanised-steel"))
    e(rect("hinge-bracket", LID_X - 0.009, 1.40, XB, 1.42, "detail", 1.85, 0.35, region="casing", material="powder-coat"))
    e(rect("feet", -0.17, 0, 0.17, 0.09, "body", 1.5, 0.2, region="feet", material="rubber"))
    e(rect("casing", XB, Y_BASE, XF, Y_TOP, "body", 2, 0.55, regions=["casing", "compressor-casing"], material="powder-coat"))
    e(rect("base-tray", XB, Y_TRAY, XF, Y_BASE, "body", 2.1, 0.3, region="base-tray", material="galvanised-steel"))
    e(rect("rear-post", XB, Y_BASE, XB + 0.06, Y_TOP, "detail", 2.35, 0.5, material="powder-coat"))
    e(rect("coil-guard", XB, COIL_Y[0], 0.12, COIL_Y[1], "detail", 2.3, 0.25, region="coil-guard", material="aluminium", hatch="fins"))
    e(poly("fan-guard", dome_outline(0.013, 0.005), "body", 2.2, 0.3, region="fan-guard", material="galvanised-steel"))
    e(lid_shape(lid_elevation_profile(t), 2.5))
    f = lambda s: sc.shape("front", s)
    front_feet(sc, 0.015, 1.5)
    f(rect("base-tray", ZR, Y_TRAY, ZL, Y_BASE, "body", 2.1, 0.3, region="base-tray", material="galvanised-steel"))
    f(rect("casing", ZR, Y_BASE, ZL, Y_TOP, "body", 2, 0.55, region="casing", material="powder-coat"))
    f(rect("compressor-panel", ZC, Y_BASE, ZL, Y_TOP, "body", 2.05, 0.5, region="compressor-casing", material="powder-coat"))
    f(ellipse("fan-guard", FZ, FY, RIM_R + 0.005, "body", 2.2, 0.25, region="fan-guard", material="galvanised-steel"))
    for k in range(3):
        f(poly(f"fan-blade-{k + 1}", blade_outline(blades, k), "detail", 2.3, 0.45, material="aluminium"))
    f(ellipse("fan-hub", FZ, FY, 0.075, "detail", 2.35, 0.3, material="galvanised-steel"))
    for i, rr in enumerate([0.27, 0.23, 0.19, 0.15, 0.11]):
        f(ellipse(f"guard-ring-{i + 1}", FZ, FY, rr, "detail", 2.4, 0.3, region="fan-guard", material="galvanised-steel", filled=False, penMm=0.13))
    spokes = " ".join(f"M {r5(FZ + 0.06 * math.sin(a))} {r5(FY + 0.06 * math.cos(a))} L {r5(FZ + (RIM_R - 0.004) * math.sin(a))} {r5(FY + (RIM_R - 0.004) * math.cos(a))}" for a in [2 * math.pi * k / 8 for k in range(8)])
    f(shape("guard-spokes", "path", "detail", 2.42, 0.3, region="fan-guard", material="galvanised-steel", filled=False, d=spokes, penMm=0.13))
    f(ellipse("guard-cap", FZ, FY, 0.06, "detail", 2.45, 0.35, material="galvanised-steel"))
    f(lid_shape(lid_elevation_profile(t), 2.5))
    sc.shape("plan", rect("casing", XB, ZR, XF, ZL, "body", 2, 0.55, region="casing", material="powder-coat"))
    sc.shape("plan", lid_shape(lid_elevation_profile(t), 2.5))
    return sc


def s100():
    """🔧 1:100, smallest feature 15 mm: sheets 15 mm, twelve louvres, grille of rim, two rings and four spokes, no wire guard, grip or partition seam detail."""
    t = 0.015
    sc = Scale(100)
    m = sc.add
    m(ext("feet", "yz", feet_profile(0.015), -0.17, 0.17, "rubber", 0.2, repeat={"count": 2, "translation": [0, 0, -2 * FEET_Z]}), "feet")
    m(box("base-tray", (XB, XF), (Y_TRAY, Y_BASE), (ZR, ZL), "galvanised-steel", 0.3), "base-tray")
    m(ext("front-panel", "yz", [(Y_BASE, ZR), (Y_TOP, ZR), (Y_TOP, ZP0 + 0.005), (Y_BASE, ZP0 + 0.005)], XF - t, XF, "powder-coat", 0.55, holes=[circle(FY, FZ, HOLE_R, 32)]), "casing")
    m(ext("column-panel", "yz", [(Y_BASE, ZP0 + 0.005), (Y_TOP, ZP0 + 0.005), (Y_TOP, ZL), (Y_BASE, ZL)], XF - t, XF, "powder-coat", 0.5), "compressor-casing")
    m(box("side-strip", (0.12, XF - t), (Y_BASE, Y_TOP), (ZR, ZR + t), "powder-coat", 0.55), "casing")
    m(ext("rear-post", "zx", [(ZR, XB), (ZR + 0.06, XB), (ZR + 0.06, XB + t), (ZR + t, XB + t), (ZR + t, XB + 0.06), (ZR, XB + 0.06)], Y_BASE, Y_TOP, "powder-coat", 0.5), "casing")
    m(box("top-panel", (XB, XF), (Y_TOP - t, Y_TOP), (ZR, ZL), "powder-coat", 0.55), "casing")
    m(box("column-back", (XB, XB + t), (Y_BASE, Y_TOP), (ZP0, ZL), "powder-coat", 0.5), "compressor-casing")
    m(box("partition", (XB + t, XF - t), (Y_BASE, Y_TOP - t), (ZP0, ZP0 + t), "galvanised-steel", 0.45), "casing")
    m(box("service-panel", (XB, XF), (Y_BASE, Y_TOP), (ZL - t, ZL), "powder-coat", 0.5), "compressor-casing")
    m(rev("water-connections", [(0, 0), (0, 0.021), (0.02, 0.021), (0.02, 0)], [1, 0, 0], [LID_X, 0.30, 0.34], "galvanised-steel", 0.35, "detail", repeat={"count": 2, "translation": [0, 0, 0.12]}), "casing")
    m(box("hinge-brackets", (LID_X, XB), (1.40, 1.42), (0.28, 0.34), "powder-coat", 0.35, "detail", repeat={"count": 2, "translation": [0, 0, -0.62]}), "casing")
    m(ext("coil", "zx", coil_profile(0.04), COIL_Y[0], COIL_Y[1], "aluminium", 0.22, "detail"), "coil-guard")
    m(box("side-louvres", (0, 1), (0, 1), (0, 1), "powder-coat", 0.4, "detail") | {"size": [0.275, t, 0.03], "transform": {"translation": [-0.0175, 0.215, -0.536], "rotation": qx(-40)}, "repeat": {"count": 12, "translation": [0, 0.1, 0]}}, "coil-guard")
    bell = [(0.08, 0.34), (0.11, 0.298), (0.14, 0.29), (XF - t, 0.293), (XF - t, 0.308), (0.14, 0.305), (0.118, 0.311), (0.095, 0.34), (0.08, 0.34)]
    m(rev("bell-mouth", bell, [1, 0, 0], [0, FY, FZ], "galvanised-steel", 0.3, "detail"), "fan-guard")
    m(box("motor-bracket", (0.0, 0.02), (COIL_Y[0], Y_TOP - t), (FZ - 0.125, FZ - 0.095), "galvanised-steel", 0.3, "detail", repeat={"count": 2, "translation": [0, 0, 0.22]}), "casing")
    m(rev("fan-drive", [(0.02, 0), (0.02, 0.07), (0.16, 0.075), (0.18, 0.06), (0.194, 0)], [1, 0, 0], [0, FY, FZ], "galvanised-steel", 0.2, "detail"), "fan-guard")
    blades = blade_sections(0.015)
    m({**base("fan-blades", "aluminium", 0.45, "detail"), "kind": "loft", "sections": blades[::2] + [blades[-1]], "capStart": True, "capEnd": True, "repeat": ring_repeat(3)}, "fan-guard")
    m(torus("guard-rim", 0.2175, RIM_R, 0.0075, [0, FY, FZ], "galvanised-steel", 0.25, "body"), "fan-guard")
    for i, rr in enumerate([0.24, 0.15]):
        m(torus(f"guard-ring-{i + 1}", dome_x(rr, 0.01), rr, 0.0075, [0, FY, FZ], "galvanised-steel", 0.3), "fan-guard")
    m(cap("guard-spokes", (dome_x(0.07, 0.01), FY + 0.07, FZ), (0.2175, FY + RIM_R - 0.006, FZ), 0.0075, "galvanised-steel", 0.3, repeat=ring_repeat(4), slender=False), "fan-guard")
    m(rev("guard-cap", [(0.22, 0), (0.22, 0.07), (0.235, 0.06), (0.235, 0)], [1, 0, 0], [0, FY, FZ], "galvanised-steel", 0.35, "detail"), "fan-guard")
    lid_u = [(-LID_W, 0.02), (-LID_W, LID_L), (LID_W, LID_L), (LID_W, 0.02), (LID_W - t, 0.02), (LID_W - t, LID_L - t), (-LID_W + t, LID_L - t), (-LID_W + t, 0.02)]
    m(box("lid-plate", (0, LID_L), (LID_TOP - t, LID_TOP), (-LID_W, LID_W), "powder-coat", 0.35, bone="lid"), "top-lid")
    m(ext("lid-skirt", "zx", lid_u, -0.005, LID_TOP - t, "powder-coat", 0.35, bone="lid"), "top-lid")
    m(rev("lid-knuckles", [(0, 0.009), (0.06, 0.009)], [0, 0, 1], [0, 0, 0.34], "powder-coat", 0.3, "detail", bone="lid", repeat={"count": 2, "translation": [0, 0, -0.74]}), "top-lid")
    m(box("lid-hinge-leaves", (-0.0075, 0.0075), (0, LID_TOP - t), (0.34, 0.40), "powder-coat", 0.3, "detail", bone="lid", repeat={"count": 2, "translation": [0, 0, -0.74]}), "top-lid")
    emitters(sc, t)
    for id, x, z in (("post-front-right", (XF - 0.03, XF), (ZR, ZR + 0.03)), ("post-front-left", (XF - 0.03, XF), (ZL - 0.03, ZL)), ("post-rear-left", (XB, XB + 0.03), (ZL - 0.03, ZL))):
        sc.member(box(id, x, (Y_BASE, Y_TOP - t), z, "galvanised-steel", 0.45), "unit", "member")
    sc.ref("post-rear-right", "rear-post", "unit", "member")
    sc.member(ext("top-frame", "zx", [(ZR, XB), (ZL, XB), (ZL, XF), (ZR, XF)], Y_TOP - t - 0.03, Y_TOP - t, "galvanised-steel", 0.45, holes=[[(ZR + 0.03, XB + 0.03), (ZL - 0.03, XB + 0.03), (ZL - 0.03, XF - 0.03), (ZR + 0.03, XF - 0.03)]]), "unit", "member")
    sc.ref("base-pan", "base-tray", "unit", "member")
    sc.ref("feet-blocks", "feet", "unit", "member")
    sc.ref("partition-plate", "partition", "unit", "member")
    sc.ref("motor-bracket-bars", "motor-bracket", "unit", "member")
    sc.ref("lid-plate-member", "lid-plate", "lid", "member")
    sc.ref("lid-skirt-member", "lid-skirt", "lid", "member")
    sc.ref("hinge-lid", "lid-knuckles", "lid", "joint")
    sc.member(rev("compressor", [(0.17, 0), (0.17, 0.085), (0.55, 0.085), (0.55, 0)], [0, 1, 0], [-0.02, 0, 0.40], "galvanised-steel", 0.2), "unit", "mass")
    supports(sc, t)
    masses(sc, 0.035)
    e = lambda s: sc.shape("elevation", s)
    e(rect("water-connections", LID_X, 0.279, XB, 0.321, "detail", 1.8, 0.35, region="casing", material="galvanised-steel"))
    e(rect("feet", -0.17, 0, 0.17, 0.09, "body", 1.5, 0.2, region="feet", material="rubber"))
    e(rect("casing", XB, Y_BASE, XF, Y_TOP, "body", 2, 0.55, regions=["casing", "compressor-casing"], material="powder-coat"))
    e(rect("base-tray", XB, Y_TRAY, XF, Y_BASE, "body", 2.1, 0.3, region="base-tray", material="galvanised-steel"))
    e(rect("rear-post", XB, Y_BASE, XB + 0.06, Y_TOP, "detail", 2.35, 0.5, material="powder-coat"))
    e(rect("coil-guard", XB, COIL_Y[0], 0.12, COIL_Y[1], "detail", 2.3, 0.25, region="coil-guard", material="aluminium", hatch="fins"))
    e(poly("fan-guard", dome_outline(0.01, 0.0075), "body", 2.2, 0.3, region="fan-guard", material="galvanised-steel"))
    e(lid_shape(lid_elevation_profile(t), 2.5))
    f = lambda s: sc.shape("front", s)
    front_feet(sc, 0.015, 1.5)
    f(rect("base-tray", ZR, Y_TRAY, ZL, Y_BASE, "body", 2.1, 0.3, region="base-tray", material="galvanised-steel"))
    f(rect("casing", ZR, Y_BASE, ZL, Y_TOP, "body", 2, 0.55, region="casing", material="powder-coat"))
    f(rect("compressor-panel", ZC, Y_BASE, ZL, Y_TOP, "body", 2.05, 0.5, region="compressor-casing", material="powder-coat"))
    f(ellipse("fan-guard", FZ, FY, RIM_R + 0.0075, "body", 2.2, 0.25, region="fan-guard", material="galvanised-steel"))
    lofted = blades[::2] + [blades[-1]]
    for k in range(3):
        f(poly(f"fan-blade-{k + 1}", blade_outline(lofted, k), "detail", 2.3, 0.45, material="aluminium"))
    for i, rr in enumerate([0.24, 0.15]):
        f(ellipse(f"guard-ring-{i + 1}", FZ, FY, rr, "detail", 2.4, 0.3, region="fan-guard", material="galvanised-steel", filled=False))
    spokes = " ".join(f"M {r5(FZ + 0.07 * math.sin(a))} {r5(FY + 0.07 * math.cos(a))} L {r5(FZ + (RIM_R - 0.006) * math.sin(a))} {r5(FY + (RIM_R - 0.006) * math.cos(a))}" for a in [2 * math.pi * k / 4 for k in range(4)])
    f(shape("guard-spokes", "path", "detail", 2.42, 0.3, region="fan-guard", material="galvanised-steel", filled=False, d=spokes))
    f(ellipse("fan-hub", FZ, FY, 0.075, "detail", 2.45, 0.3, material="galvanised-steel"))
    f(lid_shape(lid_elevation_profile(t), 2.5))
    sc.shape("plan", rect("casing", XB, ZR, XF, ZL, "body", 2, 0.55, region="casing", material="powder-coat"))
    sc.shape("plan", lid_shape(lid_elevation_profile(t), 2.5))
    return sc


def s200():
    """🧰 1:200, smallest feature 30 mm: the unit as fan chamber and compressor column blocks with coil faces as dark slabs, the grille a dark disc with its hub, the cover one slab."""
    t = 0.03
    sc = Scale(200)
    m = sc.add
    m(box("feet", (-0.17, 0.17), (0, Y_TRAY), (FEET_Z - 0.09, FEET_Z + 0.09), "rubber", 0.2, repeat={"count": 2, "translation": [0, 0, -2 * FEET_Z]}), "feet")
    m(box("base-tray", (XB, XF), (Y_TRAY, Y_BASE), (ZR, ZL), "galvanised-steel", 0.3), "base-tray")
    m(ext("front-panel", "yz", [(Y_BASE, ZR), (Y_TOP, ZR), (Y_TOP, ZP0 + 0.005), (Y_BASE, ZP0 + 0.005)], XF - t, XF, "powder-coat", 0.55, holes=[circle(FY, FZ, HOLE_R, 24)]), "casing")
    m(ext("column-shell", "zx", [(ZP0 + 0.005, XF), (ZL, XF), (ZL, XB), (ZP0, XB), (ZP0, XB + t), (ZL - t, XB + t), (ZL - t, XF - t), (ZP0 + 0.005, XF - t)], Y_BASE, Y_TOP, "powder-coat", 0.5), "compressor-casing")
    m(box("side-strip", (0.12, XF - t), (Y_BASE, Y_TOP), (ZR, ZR + t), "powder-coat", 0.55), "casing")
    m(box("top-panel", (XB, XF), (Y_TOP - t, Y_TOP), (ZR, ZL - t), "powder-coat", 0.55), "casing")
    m(box("partition", (XB + t, XF - t), (Y_BASE, Y_TOP - t), (ZP0, ZP0 + t), "galvanised-steel", 0.45), "casing")
    m(ext("coil", "zx", [(ZP0, XB), (ZR, XB), (ZR, 0.12), (ZR + 0.04, 0.12), (ZR + 0.04, XB + 0.04), (ZP0, XB + 0.04)], Y_BASE, Y_TOP - t, "aluminium", 0.22, "detail"), "coil-guard")
    m(rev("fan-guard", [(0.185, 0), (0.185, HOLE_R), (XF, HOLE_R), (XF, RIM_R + 0.005), (0.225, RIM_R + 0.005), (0.225, 0)], [1, 0, 0], [0, FY, FZ], "galvanised-steel", 0.25), "fan-guard")
    m(rev("fan-hub", [(0.20, 0), (0.20, 0.075), (0.235, 0.06), (0.235, 0)], [1, 0, 0], [0, FY, FZ], "galvanised-steel", 0.35, "detail"), "fan-guard")
    m(box("lid", (0, LID_L), (-0.005, LID_TOP), (-LID_W, LID_W), "powder-coat", 0.35, bone="lid"), "top-lid")
    emitters(sc, t)
    for id, x, z in (("post-front-right", (XF - t, XF), (ZR, ZR + t)), ("post-front-left", (XF - t, XF), (ZL - t, ZL)), ("post-rear-right", (XB, XB + t), (ZR, ZR + t)), ("post-rear-left", (XB, XB + t), (ZL - t, ZL))):
        sc.member(box(id, x, (Y_BASE, Y_TOP), z, "galvanised-steel", 0.45), "unit", "member")
    sc.member(ext("top-frame", "zx", [(ZR, XB), (ZL, XB), (ZL, XF), (ZR, XF)], Y_TOP - t, Y_TOP, "galvanised-steel", 0.45, holes=[[(ZR + t, XB + t), (ZL - t, XB + t), (ZL - t, XF - t), (ZR + t, XF - t)]]), "unit", "member")
    sc.ref("base-pan", "base-tray", "unit", "member")
    sc.ref("feet-blocks", "feet", "unit", "member")
    sc.ref("partition-plate", "partition", "unit", "member")
    sc.ref("lid-member", "lid", "lid", "member")
    sc.member(rev("hinge-lid", [(0, 0.015), (0.8, 0.015)], [0, 0, 1], [0.015, 0.0025, -0.4], "powder-coat", 0.3, bone="lid"), "lid", "joint")
    sc.member(rev("compressor", [(0.17, 0), (0.17, 0.085), (0.55, 0.085), (0.55, 0)], [0, 1, 0], [-0.02, 0, 0.40], "galvanised-steel", 0.2), "unit", "mass")
    supports(sc, t)
    masses(sc, 0.04)
    e = lambda s: sc.shape("elevation", s)
    e(rect("feet", -0.17, 0, 0.17, 0.09, "body", 1.5, 0.2, region="feet", material="rubber"))
    e(rect("casing", XB, Y_BASE, XF, Y_TOP, "body", 2, 0.55, regions=["casing", "compressor-casing", "fan-guard"], material="powder-coat"))
    e(rect("base-tray", XB, Y_TRAY, XF, Y_BASE, "body", 2.1, 0.3, region="base-tray", material="galvanised-steel"))
    e(rect("coil-guard", XB, Y_BASE, 0.12, Y_TOP, "detail", 2.3, 0.25, region="coil-guard", material="aluminium"))
    e(lid_shape([(0, LID_TOP), (LID_L, LID_TOP), (LID_L, -0.005), (0, -0.005)], 2.5))
    f = lambda s: sc.shape("front", s)
    for id, sign in (("foot-left", 1), ("foot-right", -1)):
        f(rect(id, sign * FEET_Z - 0.09, 0, sign * FEET_Z + 0.09, Y_TRAY, "body", 1.5, 0.2, region="feet", material="rubber"))
    f(rect("base-tray", ZR, Y_TRAY, ZL, Y_BASE, "body", 2.1, 0.3, region="base-tray", material="galvanised-steel"))
    f(rect("casing", ZR, Y_BASE, ZL, Y_TOP, "body", 2, 0.55, region="casing", material="powder-coat"))
    f(rect("compressor-panel", ZP1, Y_BASE, ZL, Y_TOP, "body", 2.05, 0.5, region="compressor-casing", material="powder-coat"))
    f(ellipse("fan-guard", FZ, FY, RIM_R + 0.005, "body", 2.2, 0.25, region="fan-guard", material="galvanised-steel"))
    f(ellipse("fan-hub", FZ, FY, 0.075, "detail", 2.3, 0.35, material="galvanised-steel"))
    f(lid_shape([(0, LID_TOP), (LID_L, LID_TOP), (LID_L, -0.005), (0, -0.005)], 2.5))
    sc.shape("plan", rect("casing", XB, ZR, XF, ZL, "body", 2, 0.55, region="casing", material="powder-coat"))
    sc.shape("plan", lid_shape([(0, LID_TOP), (LID_L, LID_TOP), (LID_L, -0.005), (0, -0.005)], 2.5))
    return sc


def s500():
    """🧱 1:500, smallest feature 75 mm: casing and compressor column as blocks over the merged base pan, the L coil as one dark slab, a dark fan disc, two feet and the cover slab."""
    t = 0.075
    sc = Scale(500)
    m = sc.add
    m(box("feet", (-0.17, 0.17), (0, Y_TRAY), (FEET_Z - 0.09, FEET_Z + 0.09), "rubber", 0.2, repeat={"count": 2, "translation": [0, 0, -2 * FEET_Z]}), "feet")
    m(box("base-tray", (XB, XF), (Y_TRAY, Y_TRAY + t), (ZR, ZL), "galvanised-steel", 0.3), "base-tray")
    m(ext("front-panel", "yz", [(Y_TRAY + t, ZR), (Y_TOP, ZR), (Y_TOP, ZL), (Y_TRAY + t, ZL)], XF - t, XF, "powder-coat", 0.55, holes=[circle(FY, FZ, HOLE_R, 16)]), "casing")
    m(ext("column-shell", "zx", [(ZP1, XB), (ZL, XB), (ZL, XF - t), (ZL - t, XF - t), (ZL - t, XB + t), (ZP1, XB + t)], Y_TRAY + t, Y_TOP, "powder-coat", 0.5), "compressor-casing")
    m(box("top-panel", (XB + t, XF - t), (Y_TOP - t, Y_TOP), (ZR + t, ZL - t), "powder-coat", 0.55), "casing")
    m(ext("coil", "zx", [(ZP1, XB), (ZR, XB), (ZR, XF - t), (ZR + t, XF - t), (ZR + t, XB + t), (ZP1, XB + t)], Y_TRAY + t, Y_TOP, "aluminium", 0.22, "detail"), "coil-guard")
    m(rev("fan-disc", [(XF - t, 0), (XF - t, HOLE_R), (0.235, HOLE_R), (0.235, 0)], [1, 0, 0], [0, FY, FZ], "galvanised-steel", 0.25), "fan-guard")
    m(box("lid", (0, LID_L), (-0.005, LID_TOP), (-LID_W, LID_W), "powder-coat", 0.35, bone="lid"), "top-lid")
    emitters(sc, t)
    for id, x, z in (("post-front-right", (XF - t, XF), (ZR, ZR + t)), ("post-front-left", (XF - t, XF), (ZL - t, ZL)), ("post-rear-right", (XB, XB + t), (ZR, ZR + t)), ("post-rear-left", (XB, XB + t), (ZL - t, ZL))):
        sc.member(box(id, x, (Y_TRAY + t, Y_TOP - t), z, "galvanised-steel", 0.45), "unit", "member")
    for id, y in (("base-frame", (Y_TRAY, Y_TRAY + t)), ("top-frame", (Y_TOP - t, Y_TOP))):
        sc.member(ext(id, "zx", [(ZR, XB), (ZL, XB), (ZL, XF), (ZR, XF)], y[0], y[1], "galvanised-steel", 0.45, holes=[[(ZR + t, XB + t), (ZL - t, XB + t), (ZL - t, XF - t), (ZR + t, XF - t)]]), "unit", "member")
    sc.ref("feet-blocks", "feet", "unit", "member")
    sc.ref("lid-member", "lid", "lid", "member")
    sc.member(rev("hinge-lid", [(0, 0.0375), (0.8, 0.0375)], [0, 0, 1], [0.0375, 0.0175, -0.4], "powder-coat", 0.3, bone="lid"), "lid", "joint")
    supports(sc, t)
    masses(sc, 0.075)
    e = lambda s: sc.shape("elevation", s)
    e(rect("feet", -0.17, 0, 0.17, 0.09, "body", 1.5, 0.2, region="feet", material="rubber"))
    e(rect("casing", XB, Y_TRAY, XF, Y_TOP, "body", 2, 0.55, regions=["casing", "compressor-casing", "fan-guard", "base-tray", "coil-guard"], material="powder-coat"))
    e(lid_shape([(0, LID_TOP), (LID_L, LID_TOP), (LID_L, -0.005), (0, -0.005)], 2.5))
    f = lambda s: sc.shape("front", s)
    for id, sign in (("foot-left", 1), ("foot-right", -1)):
        f(rect(id, sign * FEET_Z - 0.09, 0, sign * FEET_Z + 0.09, Y_TRAY, "body", 1.5, 0.2, region="feet", material="rubber"))
    f(rect("casing", ZR, Y_TRAY, ZL, Y_TOP, "body", 2, 0.55, regions=["casing", "compressor-casing", "base-tray"], material="powder-coat"))
    f(ellipse("fan-disc", FZ, FY, RIM_R, "body", 2.2, 0.25, region="fan-guard", material="galvanised-steel"))
    f(lid_shape([(0, LID_TOP), (LID_L, LID_TOP), (LID_L, -0.005), (0, -0.005)], 2.5))
    sc.shape("plan", rect("casing", XB, ZR, XF, ZL, "body", 2, 0.55, region="casing", material="powder-coat"))
    sc.shape("plan", lid_shape([(0, LID_TOP), (LID_L, LID_TOP), (LID_L, -0.005), (0, -0.005)], 2.5))
    return sc


def txt(en, de):
    return {"en": en, "de": de}


AREAS = {"casing": 4.6, "fan-guard": 1.15}
TEXTS = json.loads((T / "r3-heat-pump-outdoor-unit-texts.json").read_text())


def build():
    """🏗️ Candidate asset document: catalog asset with every root member and the volumes replaced."""
    doc = json.loads(ASSET_FILE.read_text())
    a = doc["asset"]
    scales = {"50": s50(), "100": s100(), "200": s200(), "500": s500()}
    a["summary"] = TEXTS["summary"]
    prov = a["provenance"]
    prov["references"] = [ref for ref in prov["references"] if ref["id"] != "vaillant-arotherm-plus"] + [{"id": "vaillant-arotherm-plus", "citation": TEXTS["vaillant"]}]
    prov["notes"] = TEXTS["provenanceNotes"]
    a["dimensions"] = {"heightM": 1.45, "widthM": 0.47, "depthM": 1.1, "massKg": 150}
    a["rig"]["bones"] = [b for b in a["rig"]["bones"] if b["id"] != "vane"]
    for b in a["rig"]["bones"]:
        if b["id"] == "unit":
            b["title"] = TEXTS["unitTitle"]
            b["mass"].update({"kg": 143, "comX": 0, "comY": 0.55})
        if b["id"] == "lid":
            b["title"] = TEXTS["lidTitle"]
            b.update({"x": LID_X, "y": LID_Y, "lengthM": LID_L})
            b["mass"].update({"kg": 7, "comX": 0.235, "comY": 0.0175})
    for anchor in a["anchors"]:
        if anchor["id"] == "fan-outlet":
            anchor.update({"x": 0.235, "y": FY, "z": FZ})
        if anchor["id"] == "coil-top":
            anchor.update({"x": -0.2125, "y": 1.34, "z": -0.125})
    for pose in a["poses"]:
        pose["bones"].pop("vane", None)
        pose["thermal"]["regions"].pop("discharge-vane", None)
        if pose["id"] in TEXTS["poses"]:
            pose["description"] = TEXTS["poses"][pose["id"]]
    clip = a["clips"][0]
    clip.update(TEXTS["clip"])
    clip["keys"] = [{"t": 0, "pose": "heating", "ease": "smooth"}, {"t": 10, "pose": "heating", "ease": "smooth"}, {"t": 20, "pose": "heating", "ease": "smooth"}]
    clip["tracks"] = [
        {"target": "operating:compressor", "ease": "smooth", "keys": [{"t": 0, "v": 0.6}, {"t": 10, "v": 0.8}, {"t": 20, "v": 0.6}]},
        {"target": "operating:refrigerant-duty", "ease": "smooth", "keys": [{"t": 0, "v": 0.6}, {"t": 10, "v": 0.8}, {"t": 20, "v": 0.6}]},
        {"target": "operating:fan-speed", "ease": "smooth", "keys": [{"t": 0, "v": 0.6}, {"t": 10, "v": 0.8}, {"t": 20, "v": 0.6}]},
        {"target": "emitter:discharge-heating", "ease": "step", "keys": [{"t": 0, "v": 1}]},
    ]
    clip = {k: clip[k] for k in ["id", "title", "description", "durationS", "fps", "loop", "rootMotion", "keys", "contacts", "phases", "events", "tracks"] if k in clip}
    a["clips"][0] = clip
    a["presentation"]["defaultClip"] = clip["id"]
    th = a["thermal"]
    th["regions"] = [r for r in th["regions"] if r["id"] != "discharge-vane"]
    for region in th["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
    for em in th["emitters"]:
        if em["id"].startswith("discharge-"):
            em["diameterM"] = 0.56
    sym = a["symbols"][0]
    sym["description"] = TEXTS["symbolDescription"]
    sym["dimensionalNote"] = TEXTS["symbolNote"]
    sym["geometry"] = {
        "elevation": [rect("elevation-casing", XB, 0, XF, Y_TOP, "body", 1, 0.6), rect("elevation-lid", LID_X, 1.405, -LID_X, 1.45, "body", 2, 0.5), rect("elevation-coil-guard", -0.155, COIL_Y[0], 0.12, COIL_Y[1], "detail", 3, 0.4)],
        "front": [rect("front-casing", ZR, 0, ZL, Y_TOP, "body", 1, 0.6), rect("front-lid", -LID_W, 1.405, LID_W, 1.45, "body", 2, 0.5), ellipse("front-fan-opening", FZ, FY, RIM_R, "detail", 3, 0.5, filled=False), ellipse("front-fan-hub", FZ, FY, 0.075, "detail", 4, 0.4), rect("front-column", ZC, Y_BASE, ZL, Y_TOP, "detail", 3.5, 0.55, filled=False)],
        "plan": [rect("plan-casing", XB, ZR, XF, ZL, "body", 1, 0.6), rect("plan-lid", LID_X, -LID_W, -LID_X, LID_W, "body", 2, 0.5)],
    }
    for shapes in sym["geometry"].values():
        for s in shapes:
            s.pop("region", None)
    reps = a["representations"]
    for key, sc in scales.items():
        rep = reps[key]
        rep["geometry"] = sc.views
        rep["decisions"] = TEXTS["representation"][key]
        if key in ("50", "100"):
            rep["hatches"] = [{"id": "fins", "title": txt("Coil guard louvres", "Lamellen des Tauscherschutzes"), "pattern": "lines", "angleDeg": 0, "spacingMm": 1.0, "penMm": 0.1}]
    volumes = {key: sc.volume(TEXTS["volumes"][key]) for key, sc in scales.items()}
    a["volumes"] = volumes
    return doc, volumes


def apply(doc, volumes):
    """✍️ Writes the root members into the catalog file (re-read right before), the manifest entry and the volumes source."""
    current = json.loads(ASSET_FILE.read_text())
    new = doc["asset"]
    for key in list(current["asset"].keys()):
        if key != "volumes":
            current["asset"][key] = new[key]
    ASSET_FILE.write_text(json.dumps(current, ensure_ascii=False, indent=1) + "\n")
    SOURCE_FILE.write_text(json.dumps({"volumes": volumes}, ensure_ascii=False, indent=1) + "\n")
    text = MANIFEST_FILE.read_text()
    start = text.index('"id": "heat-pump-outdoor-unit"')
    end = text.index('"completion"', start)
    entry = json.loads(MANIFEST_FILE.read_text())
    old = next(e for e in entry["entries"] if e["id"] == "heat-pump-outdoor-unit")
    part = text[start:end]
    part = part.replace(f'"widthM": {old["widthM"]},', f'"widthM": {new["dimensions"]["widthM"]},')
    for before, after in zip(old["clips"], [c["id"] for c in new["clips"]]):
        part = part.replace(json.dumps(before), json.dumps(after))
    for lang in ("en", "de"):
        part = part.replace(json.dumps(old["distinction"][lang], ensure_ascii=False), json.dumps(new["summary"][lang], ensure_ascii=False))
    MANIFEST_FILE.write_text(text[:start] + part + text[end:])
    check = next(e for e in json.loads(MANIFEST_FILE.read_text())["entries"] if e["id"] == "heat-pump-outdoor-unit")
    assert check["widthM"] == new["dimensions"]["widthM"] and check["clips"] == [c["id"] for c in new["clips"]] and check["distinction"] == new["summary"]


if __name__ == "__main__":
    doc, volumes = build()
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
    print("candidate", OUT / "🔣️.json")
    if "--apply" in sys.argv:
        apply(doc, volumes)
        print("applied")
