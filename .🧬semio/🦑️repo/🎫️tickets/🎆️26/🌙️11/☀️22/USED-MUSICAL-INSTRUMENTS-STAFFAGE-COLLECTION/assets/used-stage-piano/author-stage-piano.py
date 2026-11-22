"""🎹 Authors the staffage asset `used-stage-piano` from hand-chosen numbers and writes `🔣️.json` next to this file.

Frame: x ahead (towards the player, the key fronts face +x), y up, z left of the asset; the keyboard length runs along z with the
treble at +z. Side elevation from -z (the bass end cheek), front elevation from +x (the player's view), plan from above.

@see ../../brief.md — the binding brief of the wave
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md — the tools
"""
import json
import math
import pathlib

HERE = pathlib.Path(__file__).resolve().parent
SCALES = (50, 100, 200, 500)
MASS_R = [0.012]
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def T(en, de):
    """📝 A bilingual text, English first."""
    return {"en": en, "de": de}


def r(v, n=5):
    """🔢 Rounds a number for a stable document."""
    return round(v + 0.0, n)


def pts(seq):
    """📍 Rounds a list of points."""
    return [[r(a), r(b)] for a, b in seq]


TH_S = 2.2
TH_P = 43.0
LEG = 0.90
HALF = LEG / 2
FOOT_R = 0.015
ARM_R = 0.016
YP_S = FOOT_R + HALF * math.sin(math.radians(TH_S))
YP_P = FOOT_R + HALF * math.sin(math.radians(TH_P))
ARM_TOP_P = FOOT_R + LEG * math.sin(math.radians(TH_P)) + ARM_R
TUBE = 0.025
XA = -0.0125
XB = 0.0125

CX0 = -0.78
CD = 0.38
CH = 0.09
CL = 1.40
CW = 0.015
CB = 0.012
PX0 = CX0 + 0.025
PY0 = CB + 0.010
PD = 0.33
PH = 0.140
PL = 1.33
KEYS_L = 1.222
CHEEK = (PL - KEYS_L) / 2
WK = 0.0235
PEDAL_X0 = -1.06
PEDAL_Z = 0.15
PEDAL_L = 0.18
PEDAL_W = 0.10
PSU_X0 = -1.04
PSU_Z = -0.45
LID_IN = CW

STEEL_RHO = 7850.0
TUBE_AREA = TUBE ** 2 - (TUBE - 0.003) ** 2
CROSS_AREA = math.pi / 4 * (0.025 ** 2 - 0.022 ** 2)
LEG_TUBE_KG = TUBE_AREA * LEG * STEEL_RHO
CROSS_KG = CROSS_AREA * 0.40 * STEEL_RHO
CAPS_KG = 2 * 0.015
SLEEVE_KG = math.pi / 4 * (0.032 ** 2 - 0.025 ** 2) * 0.40 * 150
LEG_KG = round(LEG_TUBE_KG + 2 * CROSS_KG + CAPS_KG + SLEEVE_KG, 3)
HUB_KG = 0.18
CASE_KG = 2.25
LID_KG = 1.45
PIANO_KG = 17.5
PEDAL_KG = 0.37
PLATE_KG = 0.13
PSU_KG = 0.35
MASS_TOTAL = round(HUB_KG + 2 * LEG_KG + CASE_KG + LID_KG + PIANO_KG + PEDAL_KG + PLATE_KG + PSU_KG, 2)

# ---------------------------------------------------------------- kinematics


def rot_w(a):
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def rot_v(b):
    c, s = math.cos(math.radians(b)), math.sin(math.radians(b))
    return [[c, 0, -s], [0, 1, 0], [s, 0, c]]


def rot_u(t):
    c, s = math.cos(math.radians(t)), math.sin(math.radians(t))
    return [[1, 0, 0], [0, c, s], [0, -s, c]]


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def app(m, v):
    return [sum(m[i][k] * v[k] for k in range(3)) for i in range(3)]


def solve_frames(bones, offsets=None):
    """🦴 World frames (origin, rotation with columns u, v, w) of every bone, parents first, like the product's chain."""
    offsets = offsets or {}
    out = {}
    for b in bones:
        o = offsets.get(b["id"], {})
        rest = mul(mul(rot_w(b["angleDeg"]), rot_v(b.get("abductionDeg", 0))), rot_u(b.get("twistDeg", 0)))
        off = mul(mul(rot_w(o.get("angleDeg", 0)), rot_v(o.get("abductionDeg", 0))), rot_u(o.get("twistDeg", 0)))
        dx, dy = o.get("dx", 0), o.get("dy", 0)
        if b["parent"] is None:
            pr, po = [[1, 0, 0], [0, 1, 0], [0, 0, 1]], [0, 0, 0]
        else:
            po, pr = out[b["parent"]]
        loc = [b["x"] + dx, b["y"] + dy, b.get("z", 0)]
        origin = [po[i] + app(pr, loc)[i] for i in range(3)]
        out[b["id"]] = (origin, mul(mul(pr, rest), off))
    return out


def to_world(frames, bone, p):
    o, m = frames[bone]
    q = app(m, p)
    return [o[i] + q[i] for i in range(3)]


def to_local(frames, bone, p):
    o, m = frames[bone]
    d = [p[i] - o[i] for i in range(3)]
    return [sum(m[k][i] * d[k] for k in range(3)) for i in range(3)]


# ---------------------------------------------------------------- rig


def bone(id, parent, title, x, y, angle, length, joint, z=None, abduction=None, twist=None, mass=None, section=None):
    b = {"id": id, "parent": parent, "title": title, "x": r(x, 6), "y": r(y, 6)}
    if z is not None:
        b["z"] = r(z, 6)
    b["angleDeg"] = r(angle, 4)
    if abduction is not None:
        b["abductionDeg"] = abduction
    if twist is not None:
        b["twistDeg"] = twist
    b["lengthM"] = length
    b["joint"] = joint
    if mass is not None:
        b["mass"] = mass
    if section is not None:
        b["section"] = section
    return b


def m(kg, cx, cy, ref="authored-estimate", reference="catalogue-stage-pianos"):
    return {"kg": kg, "comX": r(cx, 4), "comY": r(cy, 4), "distribution": "point", "source": ref, "reference": reference}


SECTION_TUBE = {"areaM2": r(TUBE_AREA, 8), "secondMomentM4": r((TUBE ** 4 - (TUBE - 0.003) ** 4) / 12, 12), "extremeFibreM": TUBE / 2, "material": "stand-steel", "title": T("Square steel tube 25 x 25 x 1.5 mm", "Stahl-Vierkantrohr 25 x 25 x 1,5 mm")}

BONES = [
    bone("floor", None, T("Floor datum of the dealer's showroom", "Bodenbezug des Verkaufsraums"), 0, 0, 0, 0.005, {"type": "fixed"}),
    bone("stand-hub", "floor", T("Pivot hub of the X-stand with bolt and pull knob", "Gelenknabe des X-Ständers mit Bolzen und Zugknopf"), 0, YP_S, 0, 0.006, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.6}, mass=m(HUB_KG, 0.008, 0, reference="catalogue-x-stands")),
    bone("stand-axle", "stand-hub", T("Pivot axis of the X (along the depth)", "Gelenkachse des X (in der Tiefe)"), 0, 0, 0, 0.005, {"type": "fixed"}, abduction=-90),
    bone("stand-leg-a", "stand-axle", T("Leg A of the X, foot at the bass end", "Bein A des X, Fuß am Bassende"), 0, 0, 180 - TH_S, HALF, {"type": "revolute", "minDeg": -86, "maxDeg": 0}, z=XA, mass=m(LEG_KG, 0, 0, "literature", "en-10219-2006"), section=SECTION_TUBE),
    bone("stand-leg-b", "stand-axle", T("Leg B of the X, foot at the treble end", "Bein B des X, Fuß am Diskantende"), 0, 0, TH_S, HALF, {"type": "revolute", "minDeg": 0, "maxDeg": 86}, z=XB, mass=m(LEG_KG, 0, 0, "literature", "en-10219-2006"), section=SECTION_TUBE),
    bone("case-slide", "floor", T("Floor slide of the soft case", "Bodenschiene des Softcase"), CX0, 0, 0, 0.005, {"type": "prismatic", "axisDeg": 0, "minM": -0.5, "maxM": 0.5}),
    bone("case-lift", "case-slide", T("Lift of the soft case", "Hub des Softcase"), 0, 0, 0, 0.005, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.6}),
    bone("case", "case-lift", T("Soft case, lower shell with handle", "Softcase, Unterschale mit Griff"), 0, 0, 0, CD, {"type": "revolute", "minDeg": 0, "maxDeg": 90}, mass=m(CASE_KG, 0.19, 0.04, reference="catalogue-soft-cases")),
    bone("case-lid", "case", T("Soft case, lid shell on its fabric hinge", "Softcase, Deckelschale am Stoffscharnier"), 0, CH, 180, CD, {"type": "revolute", "minDeg": -180, "maxDeg": 0}, mass=m(LID_KG, 0.19, 0.06, reference="catalogue-soft-cases")),
    bone("piano-slide", "floor", T("Floor slide of the stage piano", "Bodenschiene des Stagepianos"), PX0, 0, 0, 0.005, {"type": "prismatic", "axisDeg": 0, "minM": -0.5, "maxM": 0.8}),
    bone("piano-lift", "piano-slide", T("Lift of the stage piano", "Hub des Stagepianos"), 0, 0, 0, 0.005, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.8}),
    bone("piano", "piano-lift", T("Stage piano: housing, hammer action and 88 keys", "Stagepiano: Gehäuse, Hammermechanik und 88 Tasten"), 0, PY0, 0, PD, {"type": "revolute", "minDeg": 0, "maxDeg": 90}, mass=m(PIANO_KG, 0.17, 0.065)),
    bone("pedal-slide", "floor", T("Floor slide of the sustain pedal", "Bodenschiene des Haltepedals"), PEDAL_X0, 0, 0, 0.005, {"type": "prismatic", "axisDeg": 0, "minM": -0.5, "maxM": 1.5}, z=PEDAL_Z),
    bone("pedal", "pedal-slide", T("Sustain pedal housing with rubber sole", "Haltepedalgehäuse mit Gummisohle"), 0, LID_IN, 0, PEDAL_L, {"type": "prismatic", "axisDeg": 90, "minM": -0.05, "maxM": 0.2}, mass=m(PEDAL_KG, 0.09, 0.015, reference="catalogue-pedals")),
    bone("pedal-plate", "pedal", T("Chrome pedal lever on its hinge in the housing", "Verchromter Pedalhebel am Scharnier im Gehäuse"), 0.03, 0.035, 9, 0.15, {"type": "revolute", "minDeg": -9, "maxDeg": 1}, mass=m(PLATE_KG, 0.08, -0.004, reference="catalogue-pedals")),
    bone("psu-slide", "floor", T("Floor slide of the power adapter", "Bodenschiene des Netzteils"), PSU_X0, 0, 0, 0.005, {"type": "prismatic", "axisDeg": 0, "minM": -0.5, "maxM": 1.0}, z=PSU_Z),
    bone("psu", "psu-slide", T("Power adapter brick with coiled cords", "Netzteil mit aufgerollten Kabeln"), 0, LID_IN, 0, 0.055, {"type": "prismatic", "axisDeg": 90, "minM": -0.05, "maxM": 0.2}, mass=m(PSU_KG, 0.0275, 0.0175, reference="catalogue-stage-pianos")),
]

REST = solve_frames(BONES)


def frames_of(offsets):
    return solve_frames(BONES, offsets)


def wr(bone_id, u, v, w=0.0):
    """🌍 Rest world point of a bone-local point."""
    return to_world(REST, bone_id, [u, v, w])


# ---------------------------------------------------------------- materials

MATERIALS = [
    {"id": "housing-steel", "title": T("Black textured powder-coated sheet steel of the housing, worn at the edges", "Schwarz strukturbeschichtetes Stahlblech des Gehäuses, an den Kanten abgegriffen"), "emissivity": 0.9, "solarAbsorptivity": 0.92, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "incropera-2018"},
    {"id": "abs-black", "title": T("Black ABS of key slip, panel, pedal and adapter", "Schwarzes ABS von Tastenleiste, Bedienfeld, Pedal und Netzteil"), "emissivity": 0.92, "solarAbsorptivity": 0.93, "densityKgM3": 1050, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "key-acrylic", "title": T("White acrylic key tops, yellowed and polished to a shine by fingers", "Weiße Acryl-Tastenbeläge, vergilbt und von Fingern blank poliert"), "emissivity": 0.91, "solarAbsorptivity": 0.38, "densityKgM3": 1190, "conductivityWmK": 0.19, "specificHeatJKgK": 1470, "source": "literature", "reference": "incropera-2018"},
    {"id": "key-black", "title": T("Black ABS sharps with a matt ebony finish, glossy where played", "Schwarze ABS-Obertasten mit matter Ebenholzoberfläche, glänzend wo gespielt"), "emissivity": 0.92, "solarAbsorptivity": 0.95, "densityKgM3": 1050, "conductivityWmK": 0.17, "specificHeatJKgK": 1400, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "lcd-glass", "title": T("Display window glass", "Displayfenster aus Glas"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 2500, "conductivityWmK": 1.4, "specificHeatJKgK": 840, "source": "literature", "reference": "incropera-2018"},
    {"id": "rubber", "title": T("Black rubber of feet, end caps and pedal sole", "Schwarzer Gummi von Füßen, Endkappen und Pedalsohle"), "emissivity": 0.92, "solarAbsorptivity": 0.92, "densityKgM3": 1150, "conductivityWmK": 0.16, "specificHeatJKgK": 2000, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "case-nylon", "title": T("600-denier nylon over 12 mm EPS foam of the soft case, scuffed at the corners", "600-Denier-Nylon über 12 mm EPS-Schaum des Softcase, an den Ecken abgescheuert"), "emissivity": 0.88, "solarAbsorptivity": 0.85, "densityKgM3": 95, "conductivityWmK": 0.036, "specificHeatJKgK": 1350, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "webbing", "title": T("Polypropylene webbing and padded grip of the handle", "Polypropylen-Gurtband und gepolsterter Griff"), "emissivity": 0.9, "solarAbsorptivity": 0.85, "densityKgM3": 900, "conductivityWmK": 0.12, "specificHeatJKgK": 1900, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "stand-steel", "title": T("Black powder-coated steel tube of the stand, chipped at arms and feet", "Schwarz pulverbeschichtetes Stahlrohr des Ständers, an Armen und Füßen abgeplatzt"), "emissivity": 0.9, "solarAbsorptivity": 0.92, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "en-10219-2006"},
    {"id": "foam-sleeve", "title": T("EPDM foam sleeve of the stand arms, compressed and torn", "EPDM-Schaumhülle der Ständerarme, gestaucht und eingerissen"), "emissivity": 0.9, "solarAbsorptivity": 0.92, "densityKgM3": 150, "conductivityWmK": 0.05, "specificHeatJKgK": 1800, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "zinc-steel", "title": T("Zinc-plated steel of bolt, hinge pin and nut", "Verzinkter Stahl von Bolzen, Scharnierstift und Mutter"), "emissivity": 0.3, "solarAbsorptivity": 0.5, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "elasticModulusPa": 210e9, "strengthPa": 640e6, "source": "literature", "reference": "incropera-2018"},
    {"id": "chrome-steel", "title": T("Chrome-plated pedal plate, worn dull where the foot presses", "Verchromte Pedalplatte, matt getreten wo der Fuß drückt"), "emissivity": 0.15, "solarAbsorptivity": 0.35, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "copper-cord", "title": T("PVC-sheathed copper cord", "PVC-ummanteltes Kupferkabel"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 2600, "conductivityWmK": 1.0, "specificHeatJKgK": 900, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "paper-label", "title": T("Old price sticker and tape residue", "Altes Preisetikett und Klebebandreste"), "emissivity": 0.9, "solarAbsorptivity": 0.4, "densityKgM3": 800, "conductivityWmK": 0.1, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
]

# ---------------------------------------------------------------- 2D and 3D primitives


def shape(id, bone_id, role, z, filled, tone, geom, region=None, material=None, regions=None, hatch=None, pen=None, solid=None):
    s = {"id": id, "bone": bone_id, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    if region is not None:
        s["region"] = region
    if regions is not None:
        s["regions"] = regions
    if material is not None:
        s["material"] = material
    if hatch is not None:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    if solid is not None:
        s["solid"] = solid
    return s


def g_rect(x, y, w, h, rad=None):
    g = {"kind": "rect", "x": r(x), "y": r(y), "w": r(w), "h": r(h)}
    if rad:
        g["r"] = r(rad)
    return g


def g_poly(points, closed=True):
    return {"kind": "poly", "points": pts(points), "closed": closed}


def g_ell(cx, cy, rx, ry):
    return {"kind": "ellipse", "cx": r(cx), "cy": r(cy), "rx": r(rx), "ry": r(ry)}


def g_member(f, t, sec):
    return {"kind": "member", "from": {"x": r(f[0]), "y": r(f[1]), "z": r(f[2])}, "to": {"x": r(t[0]), "y": r(t[1]), "z": r(t[2])}, "section": sec}


def rect_pts(u0, u1, v0, v1):
    return [(u0, v0), (u1, v0), (u1, v1), (u0, v1)]


def rounded_rect(a0, a1, b0, b1, rad, n=4):
    """🔲 A rounded rectangle as a polygon (counter-clockwise)."""
    out = []
    corners = [(a1 - rad, b0 + rad, -90), (a1 - rad, b1 - rad, 0), (a0 + rad, b1 - rad, 90), (a0 + rad, b0 + rad, 180)]
    for cx, cy, start in corners:
        for k in range(n + 1):
            t = math.radians(start + 90 * k / n)
            out.append((cx + rad * math.cos(t), cy + rad * math.sin(t)))
    return out


def solid_base(id, bone_id, material, role, tone, **flags):
    s = {"id": id, "bone": bone_id, "material": material, "role": role, "tone": tone}
    for k, v in flags.items():
        if v is not None and v is not False:
            s[k] = v
    return s


def ex_xy(id, bone_id, material, role, tone, profile, w0, w1, holes=None, **flags):
    s = solid_base(id, bone_id, material, role, tone, **flags)
    s.update({"kind": "extrude", "profile": pts(profile), "plane": "xy", "from": r(w0), "to": r(w1)})
    if holes:
        s["holes"] = [pts(h) for h in holes]
    return s


def ex_zx(id, bone_id, material, role, tone, profile_wu, v0, v1, holes=None, **flags):
    s = solid_base(id, bone_id, material, role, tone, **flags)
    s.update({"kind": "extrude", "profile": pts(profile_wu), "plane": "zx", "from": r(v0), "to": r(v1)})
    if holes:
        s["holes"] = [pts(h) for h in holes]
    return s


def ex_yz(id, bone_id, material, role, tone, profile_vw, u0, u1, holes=None, **flags):
    s = solid_base(id, bone_id, material, role, tone, **flags)
    s.update({"kind": "extrude", "profile": pts(profile_vw), "plane": "yz", "from": r(u0), "to": r(u1)})
    if holes:
        s["holes"] = [pts(h) for h in holes]
    return s


def box(id, bone_id, material, role, tone, c, size, **flags):
    s = solid_base(id, bone_id, material, role, tone, **flags)
    s.update({"kind": "box", "size": [r(size[0]), r(size[1]), r(size[2])], "transform": {"translation": [r(c[0]), r(c[1]), r(c[2])]}})
    return s


def cyl(id, bone_id, material, role, tone, origin, axis, a0, a1, rad, **flags):
    s = solid_base(id, bone_id, material, role, tone, **flags)
    s.update({"kind": "revolve", "profile": [[r(a0), r(rad)], [r(a1), r(rad)]], "axis": axis, "origin": [r(origin[0]), r(origin[1]), r(origin[2])]})
    return s


def revolve(id, bone_id, material, role, tone, origin, axis, profile, **flags):
    s = solid_base(id, bone_id, material, role, tone, **flags)
    s.update({"kind": "revolve", "profile": pts(profile), "axis": axis, "origin": [r(origin[0]), r(origin[1]), r(origin[2])]})
    return s


def ellipsoid(id, bone_id, material, role, tone, c, radii, **flags):
    if flags.get("part") == "mass":
        q = max(radii[0], MASS_R[0])
        radii = (q, q, q)
    s = solid_base(id, bone_id, material, role, tone, **flags)
    s.update({"kind": "ellipsoid", "radii": [r(radii[0]), r(radii[1]), r(radii[2])], "transform": {"translation": [r(c[0]), r(c[1]), r(c[2])]}})
    return s


def ref(id, of, **label):
    s = {"id": id, "of": of}
    s.update(label)
    return s


def with_repeat(s, count, step):
    s["repeat"] = {"count": count, "translation": [r(step[0]), r(step[1]), r(step[2])]}
    return s


# ---------------------------------------------------------------- piano geometry (bone `piano`, u ahead, v up, w = z)

BACK_PROFILE = [(0.0, 0.0), (0.172, 0.0), (0.172, 0.126), (0.160, 0.140), (0.004, 0.140), (0.0, 0.136)]
KEYBED_PROFILE = [(0.172, 0.0), (0.318, 0.0), (0.318, 0.062), (0.172, 0.062)]
SLIP_PROFILE = [(0.318, 0.0), (0.330, 0.0), (0.330, 0.072), (0.326, 0.075), (0.318, 0.075)]
KEY_PROFILE = [(0.172, 0.062), (0.318, 0.062), (0.318, 0.077), (0.3265, 0.077), (0.3265, 0.093), (0.325, 0.095), (0.172, 0.095)]
BLACK_PROFILE = [(0.172, 0.095), (0.268, 0.095), (0.272, 0.102), (0.266, 0.107), (0.172, 0.107)]
CHEEK_PROFILE = [(0.0, 0.0), (0.330, 0.0), (0.330, 0.100), (0.325, 0.112), (0.312, 0.119), (0.290, 0.122), (0.180, 0.140), (0.004, 0.140), (0.0, 0.136)]
FRONT_BLOCK_500 = [(0.172, -0.010), (0.330, -0.010), (0.330, 0.095), (0.172, 0.095)]
BACK_BLOCK_500 = [(0.0, -0.010), (0.172, -0.010), (0.172, 0.126), (0.160, 0.140), (0.0, 0.140)]
FEET_U = (0.035, 0.295)
FEET_W = (-0.60, 0.60)
W_KEYS0 = -KEYS_L / 2


def black_keys():
    """🎹 The 36 sharps as w centres from A#0 to A#7: C–E divided in five, F–B in seven equal parts (the classic key-cutting layout)."""
    ce, fb = 3 * WK / 5, 4 * WK / 7
    centres = [W_KEYS0 - 2 * WK + 3 * WK + fb * 5.5 - 3 * WK + 0.0]
    centres[0] = W_KEYS0 - 5 * WK + 3 * WK + fb * 5.5
    for o in range(7):
        c0 = W_KEYS0 + 2 * WK + o * 7 * WK
        centres += [c0 + ce * 1.5, c0 + ce * 3.5, c0 + 3 * WK + fb * 1.5, c0 + 3 * WK + fb * 3.5, c0 + 3 * WK + fb * 5.5]
    return centres


BLACK_W = 0.012
BLACKS = black_keys()


def octave_classes():
    """🎼 Black keys grouped by pitch class (C#, D#, F#, G#, A#) for `repeat` with the octave step, plus A#0."""
    octave0 = W_KEYS0 + 2 * WK
    ce = 3 * WK / 5
    fb = 4 * WK / 7
    firsts = [octave0 + ce * 1.5, octave0 + ce * 3.5, octave0 + 3 * WK + fb * 1.5, octave0 + 3 * WK + fb * 3.5, octave0 + 3 * WK + fb * 5.5]
    return [("cs", firsts[0]), ("ds", firsts[1]), ("fs", firsts[2]), ("gs", firsts[3]), ("as", firsts[4])], BLACKS[0]


# ---------------------------------------------------------------- the case (bone `case`, lid bone `case-lid`)

CASE_RING = rounded_rect(-CL / 2, CL / 2, 0.0, CD, 0.04, 3)
CASE_HOLE = rounded_rect(-CL / 2 + CW, CL / 2 - CW, CW, CD - CW, 0.025, 3)
GRIP_U = CD + 0.03
GRIP_V = 0.05
GRIP_W = 0.065


def wz(points_wu):
    return [(w, u) for (u, w) in points_wu]


# ---------------------------------------------------------------- representations


def pens(scale):
    return {
        50: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.18, "ground": 0.5},
        100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.13, "ground": 0.35},
        200: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.25},
        500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
    }[scale]


VALID = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def sol_w(z0, z1):
    return {"kind": "extruded", "sweep": "w", "z0": r(z0), "z1": r(z1)}


class Drawing:
    """✏️ Collects the shapes of one scale for the three viewpoints."""

    def __init__(self):
        self.e, self.f, self.p = [], [], []

    def prism(self, id, bone_id, profile, w0, w1, z, tone, region=None, material=None, role="body", regions=None, hatch=None, front=True, plan=True, ze=None, zf=None, zp=None, solid=True):
        """🧱 A prism of a profile in (u, v) swept over w0..w1: elevation poly, front and plan through the extruded solid hint (exact in every pose)."""
        oz = REST[bone_id][0][2]
        kw = dict(region=region, material=material, regions=regions, hatch=hatch)
        self.e.append(shape(id, bone_id, role, ze if ze is not None else z, True, tone, g_poly(profile), **kw))
        if front:
            self.f.append(shape(id, bone_id, role, zf if zf is not None else z, True, tone, g_poly(profile), solid=sol_w(oz + w0, oz + w1) if solid else None, **kw))
        if plan:
            self.p.append(shape(id, bone_id, role, zp if zp is not None else z, True, tone, g_poly(profile), solid=sol_w(oz + w0, oz + w1) if solid else None, **kw))

    def member(self, id, bone_id, f, t, sec, z, tone, region=None, material=None, role="body", views="efp"):
        g = g_member(f, t, sec)
        for v, lst in (("e", self.e), ("f", self.f), ("p", self.p)):
            if v in views:
                lst.append(shape(id, bone_id, role, z, True, tone, g, region=region, material=material))


def front_rect_of(bone_id, u0, u1, v0, v1, w0, w1):
    """📐 Rest world (z, y) rectangle of a bone-local box for the front view."""
    corners = [wr(bone_id, u, v, w) for u in (u0, u1) for v in (v0, v1) for w in (w0, w1)]
    zs = [c[2] for c in corners]
    ys = [c[1] for c in corners]
    return min(zs), min(ys), max(zs) - min(zs), max(ys) - min(ys)


def plan_rect_of(bone_id, u0, u1, v0, v1, w0, w1):
    """📐 Rest world (x, z) rectangle of a bone-local box for the plan."""
    corners = [wr(bone_id, u, v, w) for u in (u0, u1) for v in (v0, v1) for w in (w0, w1)]
    xs = [c[0] for c in corners]
    zs = [c[2] for c in corners]
    return min(xs), min(zs), max(xs) - min(xs), max(zs) - min(zs)


# ---------------------------------------------------------------- parts per scale


class Lists:
    """📦 Body, thermal and structure solids of one scale."""

    def __init__(self):
        self.body, self.thermal, self.structure = [], [], []

    def add(self, solid, region=None, struct=None, thermal_ref=True):
        self.body.append(solid)
        if region is not None and thermal_ref:
            self.thermal.append(ref("th-" + solid["id"], solid["id"], region=region))
        if struct is not None:
            label = dict(struct)
            self.structure.append(ref("st-" + solid["id"], solid["id"], **label))
        return solid


def piano_parts(sc, D, L):
    """🎹 The stage piano at one scale."""
    b = "piano"
    H, K = "piano-housing", "piano-keys"
    cw0, cw1 = -PL / 2, -KEYS_L / 2
    if sc == 500:
        D.prism("piano-back", b, BACK_BLOCK_500, -PL / 2, PL / 2, 2.0, 0.34, region=H, material="housing-steel")
        D.prism("piano-front", b, FRONT_BLOCK_500, -PL / 2, PL / 2, 2.1, 0.72, region=K, material="key-acrylic")
        L.add(ex_xy("piano-back", b, "housing-steel", "body", 0.34, BACK_BLOCK_500, -PL / 2, PL / 2), H)
        L.add(ex_xy("piano-front", b, "key-acrylic", "body", 0.72, FRONT_BLOCK_500, -PL / 2, PL / 2), K)
        L.structure.append(box("st-piano-plate", b, "housing-steel", "body", 0.6, (0.165, -0.0095, 0), (0.33, 0.001, 1.32), sheet=True, member=b, part="member"))
        L.structure.append(box("st-piano-backwall", b, "housing-steel", "body", 0.6, (0.0008, 0.064, 0), (0.0015, 0.146, 1.32), sheet=True, member=b, part="member"))
        L.structure.append(ellipsoid("st-piano-mass", b, "housing-steel", "detail", 0.6, (0.17, 0.065, 0), (0.02, 0.02, 0.02), member=b, part="mass"))
        return
    feet_h = {50: (-0.010, 0.0), 100: (-0.010, 0.005), 200: (-0.010, 0.020)}[sc]
    feet_r = {50: 0.0125, 100: 0.0125, 200: 0.015}[sc]
    D.prism("cheek-bass", b, CHEEK_PROFILE, cw0, cw1, 2.6, 0.28, region=H, material="housing-steel", zf=2.6, zp=2.6)
    D.prism("cheek-treble", b, CHEEK_PROFILE, -cw1, -cw0, 1.9, 0.28, region=H, material="housing-steel", zf=2.6, zp=2.6)
    D.prism("housing-back", b, BACK_PROFILE, cw1, -cw1, 2.0, 0.34, region=H, material="housing-steel", zp=2.3)
    if sc in (50, 100):
        D.prism("keybed", b, KEYBED_PROFILE, cw1, -cw1, 1.95, 0.22, region=H, material="housing-steel", zp=2.0)
        D.prism("key-slip", b, SLIP_PROFILE, cw1, -cw1, 2.05, 0.2, region=H, material="abs-black", zf=2.2, zp=2.05)
    else:
        merged = [(0.172, 0.0), (0.330, 0.0), (0.330, 0.075), (0.318, 0.075), (0.318, 0.062), (0.172, 0.062)]
        D.prism("keybed", b, merged, cw1, -cw1, 1.95, 0.22, region=H, material="housing-steel", zf=2.2, zp=2.05)
    D.prism("white-keys", b, KEY_PROFILE, cw1, -cw1, 2.1, 0.86, region=K, material="key-acrylic", zf=2.3, zp=2.4)
    for i, (u, w) in enumerate([(u, w) for u in FEET_U for w in FEET_W]):
        fid = f"foot-{i + 1}"
        D.e.append(shape(fid, b, "body", 1.7, True, 0.12, g_rect(u - feet_r, feet_h[0], 2 * feet_r, feet_h[1] - feet_h[0]), region=H, material="rubber"))
        zx, zy, zw, zh = front_rect_of(b, u - feet_r, u + feet_r, feet_h[0], feet_h[1], w - feet_r, w + feet_r)
        D.f.append(shape(fid, b, "body", 1.7, True, 0.12, g_rect(zx, zy, zw, zh), region=H, material="rubber"))
        L.add(cyl(fid, b, "rubber", "body", 0.12, (u, 0, w), [0, 1, 0], feet_h[0], feet_h[1], feet_r), H, {"member": b, "part": "member"})
    L.add(ex_xy("cheek-bass", b, "housing-steel", "body", 0.28, CHEEK_PROFILE, cw0, cw1), H, {"member": b, "part": "member"})
    L.add(ex_xy("cheek-treble", b, "housing-steel", "body", 0.28, CHEEK_PROFILE, -cw1, -cw0), H, {"member": b, "part": "member"})
    L.add(ex_xy("housing-back", b, "housing-steel", "body", 0.34, BACK_PROFILE, cw1, -cw1), H)
    if sc in (50, 100):
        L.add(ex_xy("keybed", b, "housing-steel", "body", 0.22, KEYBED_PROFILE, cw1, -cw1), H)
        L.add(ex_xy("key-slip", b, "abs-black", "body", 0.2, SLIP_PROFILE, cw1, -cw1), H)
    else:
        L.add(ex_xy("keybed", b, "housing-steel", "body", 0.22, merged, cw1, -cw1), H)
    if sc == 50:
        key = ex_xy("white-keys", b, "key-acrylic", "body", 0.86, KEY_PROFILE, cw1 + 0.0005, cw1 + WK - 0.0005)
        L.add(with_repeat(key, 52, (0, 0, WK)), K)
        classes, a0 = octave_classes()
        blk = ex_xy("black-key-a0", b, "key-black", "body", 0.08, BLACK_PROFILE, a0 - BLACK_W / 2, a0 + BLACK_W / 2)
        L.add(blk, K)
        for name, w in classes:
            s = ex_xy(f"black-keys-{name}", b, "key-black", "body", 0.08, BLACK_PROFILE, w - BLACK_W / 2, w + BLACK_W / 2)
            L.add(with_repeat(s, 7, (0, 0, 7 * WK)), K)
        for k, (w0, w1) in enumerate([(-0.24, -0.02), (0.03, 0.26)]):
            L.add(box(f"key-shine-{k + 1}", b, "key-acrylic", "texture", 0.97, (0.27, 0.0953, (w0 + w1) / 2), (0.08, 0.0006, w1 - w0), texture=True), K)
        L.add(box("display", b, "lcd-glass", "glazing", 0.1, (0.085, 0.1403, -0.02), (0.03, 0.0006, 0.11)), H)
        L.add(box("volume-slider", b, "abs-black", "body", 0.18, (0.125, 0.144, -0.55), (0.012, 0.008, 0.026)), H)
        L.add(box("button-row", b, "abs-black", "texture", 0.45, (0.11, 0.1403, 0.12), (0.012, 0.0006, 0.30), texture=True), H)
        L.add(box("price-sticker", b, "paper-label", "texture", 0.92, (0.045, 0.1403, 0.40), (0.04, 0.0006, 0.06), texture=True), H)
        L.add(box("tape-residue", b, "paper-label", "texture", 0.6, (0.148, 0.1403, -0.25), (0.016, 0.0006, 0.22), texture=True), H)
        L.add(box("jack-panel", b, "abs-black", "texture", 0.1, (-0.0003, 0.07, -0.35), (0.0006, 0.03, 0.18), texture=True), H)
        L.add(box("cheek-scuff", b, "housing-steel", "texture", 0.62, (0.31, 0.11, cw0 - 0.0003), (0.03, 0.012, 0.0006), texture=True), H)
    elif sc == 100:
        L.add(ex_xy("white-keys", b, "key-acrylic", "body", 0.86, KEY_PROFILE, cw1, -cw1), K)
        classes, a0 = octave_classes()
        L.add(ex_xy("black-key-a0", b, "key-black", "texture", 0.08, BLACK_PROFILE, a0 - BLACK_W / 2, a0 + BLACK_W / 2, texture=True), K)
        for name, w in classes:
            s = ex_xy(f"black-keys-{name}", b, "key-black", "texture", 0.08, BLACK_PROFILE, w - BLACK_W / 2, w + BLACK_W / 2, texture=True)
            L.add(with_repeat(s, 7, (0, 0, 7 * WK)), K)
        L.add(box("volume-slider", b, "abs-black", "body", 0.18, (0.125, 0.1405, -0.55), (0.016, 0.015, 0.026)), H)
    else:
        L.add(ex_xy("white-keys", b, "key-acrylic", "body", 0.86, KEY_PROFILE[:1] + [(0.318, 0.062), (0.3265, 0.062), (0.3265, 0.095), (0.172, 0.095)], cw1, -cw1), K)
    pl_t = 0.0015
    L.structure.append(box("st-piano-plate", b, "housing-steel", "body", 0.6, (0.165, pl_t / 2, 0), (0.322, pl_t, 1.22), sheet=True, member=b, part="member"))
    L.structure.append(box("st-piano-backwall", b, "housing-steel", "body", 0.6, (0.0008, 0.069, 0), (0.0015, 0.135, 1.22), sheet=True, member=b, part="member"))
    rail = {50: (0.025, 0.02), 100: (0.025, 0.02), 200: (0.03, 0.03)}[sc]
    L.structure.append(box("st-key-frame-rail", b, "housing-steel", "body", 0.6, (0.24, 0.0015 + rail[1] / 2, 0), (rail[0], rail[1], 1.222), member=b, part="member"))
    L.structure.append(ellipsoid("st-piano-mass", b, "housing-steel", "detail", 0.6, (0.17, 0.065, 0), (0.012, 0.012, 0.012), member=b, part="mass"))
    if sc == 50:
        for i in range(51):
            w = cw1 + WK * (i + 1)
            x, zz = wr(b, 0.0, 0, w)[0], wr(b, 0, 0, w)[2]
            D.p.append(shape(f"key-gap-{i + 1}", b, "texture", 2.45, False, 0.4, g_poly([(wr(b, 0.272, 0, w)[0], zz), (wr(b, 0.3265, 0, w)[0], zz)], closed=False)))
            D.f.append(shape(f"key-gap-{i + 1}", b, "texture", 2.35, False, 0.4, g_poly([(zz, wr(b, 0, 0.077, w)[1]), (zz, wr(b, 0, 0.095, w)[1])], closed=False)))
        oz = REST[b][0][2]
        groups = [[BLACKS[0]]] + [g for o in range(7) for g in (BLACKS[1 + 5 * o:3 + 5 * o], BLACKS[3 + 5 * o:6 + 5 * o])]
        for i, grp in enumerate(groups):
            z0, z1 = grp[0] - BLACK_W / 2, grp[-1] + BLACK_W / 2
            D.p.append(shape(f"sharps-{i + 1}", b, "detail", 2.5, True, 0.08, g_poly(BLACK_PROFILE), region=K, material="key-black", solid=sol_w(oz + z0, oz + z1)))
            D.f.append(shape(f"sharps-{i + 1}", b, "detail", 2.4, True, 0.08, g_poly(BLACK_PROFILE), region=K, material="key-black", solid=sol_w(oz + z0, oz + z1)))
            for k in range(len(grp) - 1):
                g0, g1 = grp[k] + BLACK_W / 2, grp[k + 1] - BLACK_W / 2
                D.p.append(shape(f"sharps-{i + 1}-gap-{k + 1}", b, "detail", r(2.51 + 0.001 * k, 3), True, 0.86, g_poly(BLACK_PROFILE), region=K, material="key-acrylic", solid=sol_w(oz + g0, oz + g1)))
                D.f.append(shape(f"sharps-{i + 1}-gap-{k + 1}", b, "detail", r(2.41 + 0.001 * k, 3), True, 0.34, g_poly(BLACK_PROFILE), region=H, material="housing-steel", solid=sol_w(oz + g0, oz + g1)))
        for k, (w0, w1) in enumerate([(-0.24, -0.02), (0.03, 0.26)]):
            px, pz, pw, ph = plan_rect_of(b, 0.27, 0.31, 0.095, 0.095, w0, w1)
            D.p.append(shape(f"key-shine-{k + 1}", b, "texture", 2.42, False, 0.97, g_rect(px, pz, pw, ph)))
        for sid, u0, u1, w0, w1, tone, role, mat, filled in [
            ("display", 0.07, 0.10, -0.075, 0.035, 0.1, "glazing", None, False),
            ("volume-slider", 0.119, 0.131, -0.563, -0.537, 0.18, "detail", None, False),
            ("button-row", 0.104, 0.116, -0.03, 0.27, 0.45, "texture", None, False),
            ("price-sticker", 0.025, 0.065, 0.37, 0.43, 0.92, "detail", None, False),
            ("tape-residue", 0.14, 0.156, -0.36, -0.14, 0.6, "texture", None, False),
        ]:
            px, pz, pw, ph = plan_rect_of(b, u0, u1, 0.14, 0.14, w0, w1)
            D.p.append(shape(sid, b, role, 2.5, filled, tone, g_rect(px, pz, pw, ph), region=H if filled else None, material=mat))
        fx, fy, fw, fh = front_rect_of(b, 0.119, 0.131, 0.14, 0.148, -0.563, -0.537)
        D.f.append(shape("volume-slider", b, "detail", 2.45, False, 0.18, g_rect(fx, fy, fw, fh)))
        D.e.append(shape("volume-slider", b, "detail", 2.65, False, 0.18, g_rect(0.119, 0.14, 0.012, 0.008)))
        D.e.append(shape("cheek-scuff", b, "texture", 2.7, False, 0.62, g_poly([(0.296, 0.104), (0.318, 0.112), (0.326, 0.116)], closed=False)))
        D.e.append(shape("cheek-seam", b, "detail", 2.7, False, 0.2, g_poly([(0.004, 0.012), (0.326, 0.012)], closed=False)))
        D.f.append(shape("slip-wear", b, "texture", 2.36, False, 0.5, g_poly([(wr(b, 0, 0, -0.28)[2], wr(b, 0, 0.069, 0)[1]), (wr(b, 0, 0, 0.30)[2], wr(b, 0, 0.069, 0)[1])], closed=False)))
    elif sc == 100:
        for i, w in enumerate(BLACKS):
            px, pz, pw, ph = plan_rect_of(b, 0.172, 0.272, 0.107, 0.107, w - BLACK_W / 2, w + BLACK_W / 2)
            D.p.append(shape(f"black-key-{i + 1}", b, "texture", 2.5, False, 0.08, g_rect(px, pz, pw, ph)))
        px, pz, pw, ph = plan_rect_of(b, 0.117, 0.133, 0.14, 0.14, -0.563, -0.537)
        D.p.append(shape("volume-slider", b, "detail", 2.5, False, 0.18, g_rect(px, pz, pw, ph)))
        D.e.append(shape("volume-slider", b, "detail", 2.65, False, 0.18, g_rect(0.117, 0.14, 0.016, 0.008)))
        D.f.append(shape("volume-slider", b, "detail", 2.45, False, 0.18, g_rect(*front_rect_of(b, 0.117, 0.133, 0.14, 0.148, -0.563, -0.537))))


def case_parts(sc, D, L):
    """🧳 The padded soft case, clamshell zipped around the middle, at one scale."""
    R = "case-shell"
    ring = [(w, u) for (w, u) in CASE_RING]
    hole = [(w, u) for (w, u) in CASE_HOLE]
    for bone_id, tag, z in (("case", "case", 1.0), ("case-lid", "lid", 0.9)):
        panel_t = CB if bone_id == "case" else CW
        v_panel = (0.0, panel_t) if bone_id == "case" else (CH - panel_t, CH)
        rid = f"{tag}-walls"
        pid = f"{tag}-panel"
        ring_s = ex_zx(rid, bone_id, "case-nylon", "body", 0.42, ring, 0.0, CH, holes=[hole])
        panel = box(pid, bone_id, "case-nylon", "body", 0.42, (CD / 2, (v_panel[0] + v_panel[1]) / 2, 0), (CD - 2 * CW + 0.0004, panel_t, CL - 2 * CW + 0.0004), sheet=True if sc >= 100 else None)
        L.add(ring_s, R, {"member": bone_id, "part": "member"})
        L.add(panel, R)
        L.structure.append(ref(f"st-{pid}", pid, member=bone_id, part="member"))
        L.structure.append(ellipsoid(f"st-{tag}-mass", bone_id, "case-nylon", "detail", 0.6, (0.19, 0.04 if bone_id == "case" else 0.06, 0), (0.012, 0.012, 0.012), member=bone_id, part="mass"))
        prof = rect_pts(0.0, CD, 0.0, CH)
        if sc <= 100:
            prof = [(0.0, 0.006), (0.006, 0.0), (CD - 0.006, 0.0), (CD, 0.006), (CD, CH - 0.004), (CD - 0.004, CH), (0.004, CH), (0.0, CH - 0.004)]
            if bone_id == "case-lid":
                prof = [(u, CH - v) for (u, v) in reversed([(0.0, 0.006), (0.006, 0.0), (CD - 0.006, 0.0), (CD, 0.006), (CD, CH - 0.004), (CD - 0.004, CH), (0.004, CH), (0.0, CH - 0.004)])]
        oz = 0.0
        D.e.append(shape(f"{tag}-end", bone_id, "body", 3.0 if bone_id == "case" else 2.95, True, 0.42, g_poly(prof), region=R, material="case-nylon", hatch="case-weave" if sc == 50 else None))
        D.f.append(shape(f"{tag}-end", bone_id, "body", 1.0 if bone_id == "case" else 0.9, True, 0.42, g_poly(prof), region=R, material="case-nylon", solid=sol_w(-CL / 2, CL / 2)))
        D.p.append(shape(f"{tag}-end", bone_id, "body", 1.0 if bone_id == "case" else 0.9, True, 0.42, g_poly(prof), region=R, material="case-nylon", solid=sol_w(-CL / 2, CL / 2)))
        lin = rect_pts(CW, CD - CW, CB, CH) if bone_id == "case" else rect_pts(CW, CD - CW, 0.0, CH - CW)
        D.p.append(shape(f"{tag}-lining", bone_id, "body", 1.05 if bone_id == "case" else 0.95, True, 0.3, g_poly(lin), region=R, material="case-nylon", solid=sol_w(-CL / 2 + CW, CL / 2 - CW)))
    if sc <= 100:
        L.add(capsule_w("grip", "case", "webbing", 0.3, GRIP_U, GRIP_V, -GRIP_W, GRIP_W, 0.014), R, {"member": "case", "part": "member"})
        for k, w in enumerate((-0.08, 0.08)):
            L.add(box(f"grip-tab-{k + 1}", "case", "webbing", "body", 0.3, (CD + 0.015, GRIP_V, w), (0.03, 0.02, 0.02)), R)
        D.e.append(shape("grip", "case", "body", 3.2, True, 0.3, g_ell(GRIP_U, GRIP_V, 0.014, 0.014), region=R, material="webbing"))
        D.e.append(shape("grip-tab", "case", "body", 3.1, True, 0.3, g_rect(CD - 0.001, GRIP_V - 0.01, 0.032, 0.02), region=R, material="webbing"))
        gx = wr("case", GRIP_U, GRIP_V)
        D.f.append(shape("grip", "case", "body", 1.3, True, 0.3, g_rect(-0.09, gx[1] - 0.014, 0.18, 0.028, 0.012), region=R, material="webbing"))
        D.p.append(shape("grip", "case", "body", 1.3, True, 0.3, g_rect(wr("case", CD, 0)[0] - 0.001, -0.09, GRIP_U + 0.014 - CD + 0.001, 0.18), region=R, material="webbing"))

    if sc == 50:
        L.add(box("zip-bead", "case", "case-nylon", "texture", 0.2, (CD + 0.0003, CH - 0.002, 0), (0.0006, 0.004, CL - 0.1), texture=True), R)
        for k, w in enumerate((-CL / 2 + 0.03, CL / 2 - 0.03)):
            L.add(box(f"scuff-{k + 1}", "case", "case-nylon", "texture", 0.7, (CD + 0.0003, 0.012, w), (0.0006, 0.02, 0.05), texture=True), R)
        L.add(box("luggage-sticker", "case-lid", "paper-label", "texture", 0.9, (0.19, CH + 0.0003, 0.35), (0.06, 0.0006, 0.09), texture=True), R)
        D.e.append(shape("zip-line", "case", "detail", 3.3, False, 0.2, g_poly([(0.004, CH - 0.002), (CD - 0.004, CH - 0.002)], closed=False)))
        D.e.append(shape("corner-scuff", "case", "texture", 3.3, False, 0.7, g_poly([(CD - 0.02, 0.004), (CD - 0.004, 0.004), (CD - 0.001, 0.018)], closed=False)))
        D.f.append(shape("zip-line", "case", "detail", 1.35, False, 0.2, g_poly([(-CL / 2 + 0.04, CH - 0.002), (CL / 2 - 0.04, CH - 0.002)], closed=False)))
        for k, w in enumerate((-CL / 2 + 0.03, CL / 2 - 0.03)):
            D.f.append(shape(f"scuff-{k + 1}", "case", "texture", 1.35, False, 0.7, g_poly([(w - 0.025, 0.004), (w + 0.025, 0.022)], closed=False)))


def capsule_w(id, bone_id, material, tone, u, v, w0, w1, rad):
    s = solid_base(id, bone_id, material, "body", tone)
    s.update({"kind": "capsule", "p0": [r(u), r(v), r(w0)], "p1": [r(u), r(v), r(w1)], "r0": r(rad), "r1": r(rad)})
    return s


def leg_world_dir(bone_id):
    return REST[bone_id][1]


def stand_parts(sc, D, L):
    """🦵 The single-braced X-stand at one scale."""
    R = "stand-frame"
    for leg, xoff in (("stand-leg-a", XA), ("stand-leg-b", XB)):
        tag = leg[-1]
        w0, w1 = -0.20 - xoff, 0.20 - xoff
        sq = {"kind": "rect", "w": TUBE, "h": TUBE}
        D.member(f"tube-{tag}", leg, (-HALF, 0, 0), (HALF, 0, 0), sq, 1.5, 0.22, region=R, material="stand-steel")
        hole = [(-0.011, -0.011), (0.011, -0.011), (0.011, 0.011), (-0.011, 0.011)]
        tube = ex_yz(f"tube-{tag}", leg, "stand-steel", "body", 0.22, [(-0.0125, -0.0125), (0.0125, -0.0125), (0.0125, 0.0125), (-0.0125, 0.0125)], -HALF, HALF, holes=[hole] if sc <= 100 else None, slender=True)
        L.add(tube, R, {"member": leg, "part": "member"})
        for end, name in ((-HALF, "foot"), (HALF, "arm")):
            if sc == 500:
                rod_r = 0.43 / 40
                D.member(f"{name}-bar-{tag}", leg, (end, 0, w0 - 0.015), (end, 0, w1 + 0.015), {"kind": "circle", "d": r(2 * rod_r)}, 1.55 if name == "foot" else 1.57, 0.2, region=R, material="stand-steel")
                L.add(cyl(f"{name}-bar-{tag}", leg, "stand-steel", "body", 0.2, (end, 0, 0), [0, 0, 1], w0 - 0.015, w1 + 0.015, rod_r, slender=True), R, {"member": leg, "part": "member"})
                continue
            bar_r = 0.015 if sc == 200 else 0.0125
            if sc == 200:
                D.member(f"{name}-bar-{tag}", leg, (end, 0, w0 - (0.01 if name == "foot" else 0)), (end, 0, w1 + (0.01 if name == "foot" else 0)), {"kind": "circle", "d": 2 * bar_r if name == "foot" else 2 * ARM_R}, 1.55 if name == "foot" else 1.57, 0.2, region=R, material="stand-steel" if name == "foot" else "foam-sleeve")
                L.add(cyl(f"{name}-bar-{tag}", leg, "stand-steel" if name == "foot" else "foam-sleeve", "body", 0.2, (end, 0, 0), [0, 0, 1], w0 - (0.01 if name == "foot" else 0), w1 + (0.01 if name == "foot" else 0), bar_r if name == "foot" else ARM_R), R, {"member": leg, "part": "member"})
            else:
                D.member(f"{name}-bar-{tag}", leg, (end, 0, w0), (end, 0, w1), {"kind": "circle", "d": 0.025}, 1.55 if name == "foot" else 1.57, 0.25, region=R, material="stand-steel")
                L.add(cyl(f"{name}-bar-{tag}", leg, "stand-steel", "body", 0.25, (end, 0, 0), [0, 0, 1], w0, w1, 0.0125), R, {"member": leg, "part": "member"})
                if name == "arm":
                    D.member(f"sleeve-{tag}", leg, (end, 0, w0 + 0.002), (end, 0, w1 - 0.002), {"kind": "circle", "d": 2 * ARM_R}, 1.6, 0.12, region=R, material="foam-sleeve")
                    L.add(cyl(f"sleeve-{tag}", leg, "foam-sleeve", "body", 0.12, (end, 0, 0), [0, 0, 1], w0 + 0.002, w1 - 0.002, ARM_R), R, {"member": leg, "part": "member"})
            if name == "foot":
                for k, (c0, c1) in enumerate(((w0 - 0.01, w0 + 0.025), (w1 - 0.025, w1 + 0.01))):
                    cid = f"cap-{tag}-{'back' if k == 0 else 'front'}"
                    if sc in (50, 100):
                        D.member(cid, leg, (end, 0, c0), (end, 0, c1), {"kind": "circle", "d": 0.03}, 1.62, 0.1, region=R, material="rubber")
                        L.add(cyl(cid, leg, "rubber", "body", 0.1, (end, 0, 0), [0, 0, 1], c0, c1, FOOT_R), R)
                        L.structure.append(ref("st-" + cid, cid, member=leg, part="support", anchor=f"foot-{tag}-{'back' if k == 0 else 'front'}"))
                        L.structure.append(cyl(f"st-{cid}-core", leg, "stand-steel", "body", 0.6, (end, 0, 0), [0, 0, 1], c0 + 0.004, c1 - 0.004, 0.011, member=leg, part="member"))
                    else:
                        L.structure.append(cyl("st-" + cid, leg, "rubber", "body", 0.6, (end, 0, 0), [0, 0, 1], c0, c1, FOOT_R, member=leg, part="support", anchor=f"foot-{tag}-{'back' if k == 0 else 'front'}"))
        if sc == 500:
            for k, x_world in enumerate((-0.17, 0.17)):
                end_name = "back" if k == 0 else "front"
                foot = wr(leg, -HALF, 0, 0)
                c = to_local(REST, leg, [x_world, 0.0045, foot[2]])
                L.structure.append(box(f"st-cap-{tag}-{end_name}", leg, "rubber", "body", 0.6, c, (0.08, 0.004, 0.08), sheet=True, member=leg, part="support", anchor=f"foot-{tag}-{end_name}"))
        L.structure.append(ellipsoid(f"st-mass-{tag}", leg, "stand-steel", "detail", 0.6, (0, 0, 0.0), (0.012, 0.012, 0.012), member=leg, part="mass"))
        if sc == 50:
            for k, (end, ww) in enumerate(((HALF, 0.12 - xoff), (-HALF, -0.17 - xoff))):
                L.add(cyl(f"chip-{tag}-{k + 1}", leg, "stand-steel", "texture", 0.65, (end, 0, 0), [0, 0, 1], ww, ww + 0.018, 0.0127 if k else ARM_R + 0.0002, texture=True), R)
    hub = "stand-hub"
    if sc == 500:
        return
    if sc == 50:
        parts = [("pivot-bolt", -0.035, 0.035, 0.005, "zinc-steel", 0.55), ("lock-nut", -0.043, -0.035, 0.0085, "zinc-steel", 0.5), ("pull-knob", 0.035, 0.059, 0.017, "abs-black", 0.1)]
    elif sc == 100:
        parts = [("pivot-bolt", -0.043, 0.035, 0.008, "zinc-steel", 0.55), ("pull-knob", 0.035, 0.059, 0.017, "abs-black", 0.1)]
    else:
        parts = [("pull-knob", -0.043, 0.059, 0.016, "abs-black", 0.1)]
    for pid, a0, a1, rad, mat, tone in parts:
        L.add(cyl(pid, hub, mat, "body", tone, (0, 0, 0), [1, 0, 0], a0, a1, rad), R)
        D.e.append(shape(pid, hub, "body", 1.7, True, tone, g_rect(a0, -rad, a1 - a0, 2 * rad), region=R, material=mat))
        D.f.append(shape(pid, hub, "body", 1.7 if pid != "pull-knob" else 1.75, True, tone, g_ell(0, YP_S, rad, rad), region=R, material=mat))
        D.p.append(shape(pid, hub, "body", 1.7, True, tone, g_rect(a0, -rad, a1 - a0, 2 * rad), region=R, material=mat))
    L.structure.append(cyl("st-pivot", "stand-leg-a", "zinc-steel", "body", 0.6, (0, 0, 0.0), [0, 0, 1], -0.02, 0.045, 0.005 if sc == 50 else (0.008 if sc == 100 else 0.016), member="stand-leg-a", part="joint"))


def pedal_parts(sc, D, L):
    """🦶 The piano-style sustain pedal at one scale."""
    R = "pedal"
    b, pb = "pedal", "pedal-plate"
    w0, w1 = -PEDAL_W / 2, PEDAL_W / 2
    if sc == 500:
        L.structure.append(box("st-pedal-sole", b, "rubber", "body", 0.6, (0.09, 0.002, 0), (0.18, 0.004, 0.10), sheet=True, member=b, part="member"))
        L.structure.append(box("st-pedal-base", b, "abs-black", "body", 0.6, (0.09, 0.02, 0), (0.18, 0.03, 0.03), slender=True, member=b, part="member"))
        L.structure.append(box("st-pedal-lever", pb, "chrome-steel", "body", 0.6, (0.075, 0.004, 0), (0.15, 0.008, 0.008), slender=True, member=pb, part="member"))
        L.thermal.append(box("th-pedal-block", b, "abs-black", "body", 0.3, (0.09, 0.0375, 0), (0.18, 0.075, 0.10), region=R))
        return
    housing = [(0.0, 0.004), (0.06, 0.004), (0.06, 0.040), (0.05, 0.048), (0.006, 0.048), (0.0, 0.040)]
    sole = [(0.0, 0.0), (0.18, 0.0), (0.18, 0.012), (0.0, 0.012)]
    lever = [(-0.012, -0.004), (0.145, -0.004), (0.152, 0.0), (0.150, 0.006), (0.140, 0.008), (-0.012, 0.008)]
    if sc == 50:
        sole = [(0.0, 0.0), (0.18, 0.0), (0.18, 0.010), (0.172, 0.012), (0.0, 0.012)]
        for pid, bone_id, prof, mat, tone in (("pedal-sole", b, sole, "rubber", 0.12), ("pedal-housing", b, housing, "abs-black", 0.15), ("pedal-lever", pb, lever, "chrome-steel", 0.75)):
            D.e.append(shape(pid, bone_id, "body", 2.0 if bone_id == b else 2.1, True, tone, g_poly(prof), region=R, material=mat))
            if bone_id == pb:
                D.f.append(shape(pid, bone_id, "body", 2.1, True, tone, g_poly(prof), region=R, material=mat, solid=sol_w(PEDAL_Z - 0.035, PEDAL_Z + 0.035)))
                D.p.append(shape(pid, bone_id, "body", 2.1, True, tone, g_poly(prof), region=R, material=mat, solid=sol_w(PEDAL_Z - 0.035, PEDAL_Z + 0.035)))
            else:
                us = [p[0] for p in prof]
                vs = [p[1] for p in prof]
                D.f.append(shape(pid, bone_id, "body", 2.0, True, tone, g_rect(*front_rect_of(bone_id, min(us), max(us), min(vs), max(vs), w0, w1)), region=R, material=mat))
                D.p.append(shape(pid, bone_id, "body", 2.0 if pid == "pedal-sole" else 2.05, True, tone, g_rect(*plan_rect_of(bone_id, min(us), max(us), min(vs), max(vs), w0, w1)), region=R, material=mat))
            ww = (-0.035, 0.035) if bone_id == pb else (w0, w1)
            L.add(ex_xy(pid, bone_id, mat, "body", tone, prof, ww[0], ww[1]), R, {"member": bone_id, "part": "member"})
        L.add(cyl("hinge-pin", pb, "zinc-steel", "body", 0.5, (0, 0, 0), [0, 0, 1], -0.045, 0.045, 0.004), R, {"member": pb, "part": "joint"})
        L.add(box("lever-wear", pb, "chrome-steel", "texture", 0.45, (0.11, 0.0083, 0), (0.05, 0.0006, 0.05), texture=True), R)
        L.add(cyl("cord-stub", b, "copper-cord", "body", 0.1, (0, 0.025, 0), [1, 0, 0], -0.012, 0.0, 0.004), R)
        D.e.append(shape("cord-stub", b, "detail", 2.05, True, 0.1, g_rect(-0.012, 0.021, 0.012, 0.008), region=R, material="copper-cord"))
        D.p.append(shape("lever-wear", pb, "texture", 2.15, False, 0.45, g_rect(*plan_rect_of(pb, 0.085, 0.135, 0.008, 0.008, -0.025, 0.025))))
    else:
        wedge = [(0.0, 0.0), (0.18, 0.0), (0.18, 0.050), (0.165, 0.060), (0.06, 0.048), (0.0, 0.048)]
        D.e.append(shape("pedal-body", b, "body", 2.0, True, 0.2, g_poly(wedge), region=R, material="abs-black"))
        D.f.append(shape("pedal-body", b, "body", 2.0, True, 0.2, g_rect(*front_rect_of(b, 0, 0.18, 0, 0.06, w0, w1)), region=R, material="abs-black"))
        D.p.append(shape("pedal-body", b, "body", 2.0, True, 0.2, g_rect(*plan_rect_of(b, 0, 0.18, 0, 0.06, w0, w1)), region=R, material="abs-black"))
        L.add(ex_xy("pedal-body", b, "abs-black", "body", 0.2, wedge, w0, w1), R, {"member": b, "part": "member"})
        L.structure.append(box("st-pedal-sole", b, "rubber", "body", 0.6, (0.09, 0.0075 if sc == 100 else 0.015, 0), (0.18, 0.015 if sc == 100 else 0.03, 0.10), member=b, part="member"))
        L.structure.append(box("st-pedal-lever", pb, "chrome-steel", "body", 0.6, (0.075, 0.004, 0), (0.15, 0.008, 0.008), slender=True, member=pb, part="member"))
    L.structure.append(ellipsoid("st-pedal-mass", b, "abs-black", "detail", 0.6, (0.09, 0.015, 0), (0.01, 0.01, 0.01), member=b, part="mass"))


def psu_parts(sc, D, L):
    """🔌 The external 12 V adapter with its coiled cords at one scale."""
    R = "power-supply"
    b = "psu"
    if sc == 500:
        L.structure.append(box("st-psu", b, "abs-black", "body", 0.6, (0.0375, 0.0375, 0), (0.075, 0.075, 0.12), member=b, part="member"))
        L.thermal.append(box("th-psu-block", b, "abs-black", "body", 0.3, (0.0375, 0.0375, 0), (0.075, 0.075, 0.12), region=R))
        return
    prof = rounded_rect(0.0, 0.055, 0.0, 0.035, 0.006, 2) if sc == 50 else rect_pts(0.0, 0.055, 0.0, 0.035)
    D.e.append(shape("psu-brick", b, "body", 2.0, True, 0.14, g_poly(prof), region=R, material="abs-black"))
    D.f.append(shape("psu-brick", b, "body", 2.0, True, 0.14, g_rect(*front_rect_of(b, 0, 0.055, 0, 0.035, -0.06, 0.06), 0.006 if sc == 50 else None), region=R, material="abs-black"))
    D.p.append(shape("psu-brick", b, "body", 2.0, True, 0.14, g_rect(*plan_rect_of(b, 0, 0.055, 0, 0.035, -0.06, 0.06), 0.006 if sc == 50 else None), region=R, material="abs-black"))
    L.add(ex_xy("psu-brick", b, "abs-black", "body", 0.14, prof, -0.06, 0.06), R, {"member": b, "part": "member"})
    if sc == 50:
        ring = [(0.0275 + 0.04, 0.003 + 0.003 * math.sin(2 * math.pi * k / 12)) for k in range(0)]
        coil = revolve("cord-coil", b, "copper-cord", "body", 0.1, (0.0275, 0, 0.105), [0, 1, 0], [(0.037 + 0.003 * math.cos(2 * math.pi * k / 10), 0.003 + 0.003 * math.sin(2 * math.pi * k / 10)) for k in range(10)] + [(0.04, 0.003)], slender=True)
        coil["profile"] = pts([(0.003 + 0.003 * math.sin(2 * math.pi * k / 10), 0.04 + 0.003 * math.cos(2 * math.pi * k / 10)) for k in range(10)] + [(0.003, 0.043)])
        L.add(coil, R)
        x0, z0 = wr(b, 0.0275, 0)[0], wr(b, 0, 0, 0.105)[2]
        D.p.append(shape("cord-coil", b, "detail", 2.05, False, 0.1, g_ell(x0, z0, 0.04, 0.04), material="copper-cord"))
        D.e.append(shape("cord-coil", b, "detail", 1.95, True, 0.1, g_rect(0.0275 - 0.043, 0.0, 0.086, 0.006), region=R, material="copper-cord"))
        D.f.append(shape("cord-coil", b, "detail", 1.95, True, 0.1, g_rect(z0 - 0.043, wr(b, 0, 0)[1], 0.086, 0.006), region=R, material="copper-cord"))
        L.add(box("psu-label", b, "paper-label", "texture", 0.9, (0.0275, 0.0353, 0), (0.035, 0.0006, 0.06), texture=True), R)
        D.p.append(shape("psu-label", b, "texture", 2.1, False, 0.9, g_rect(*plan_rect_of(b, 0.01, 0.045, 0.035, 0.035, -0.03, 0.03))))
    L.structure.append(ellipsoid("st-psu-mass", b, "abs-black", "detail", 0.6, (0.0275, 0.0175, 0), (0.008, 0.008, 0.008), member=b, part="mass"))


# ---------------------------------------------------------------- anchors


def anchor(id, bone_id, x, y, kind, title, z=None, support=None, fl=None, fw=None, mu=None):
    a = {"id": id, "bone": bone_id, "x": r(x), "y": r(y)}
    if z is not None:
        a["z"] = r(z)
    a["kind"] = kind
    a["title"] = title
    if support is not None:
        a["support"] = support
    if fl is not None:
        a["footprintLengthM"] = fl
        a["footprintWidthM"] = fw
    if mu is not None:
        a["frictionCoefficient"] = mu
    return a


FEET = ["foot-a-back", "foot-a-front", "foot-b-back", "foot-b-front"]
ANCHORS = []
for leg, xoff, tag, side_en, side_de in (("stand-leg-a", XA, "a", "bass", "Bass"), ("stand-leg-b", XB, "b", "treble", "Diskant")):
    for end, x_world, end_en, end_de in (("back", -0.1925, "rear", "hinten"), ("front", 0.1925, "front", "vorne")):
        ANCHORS.append(anchor(f"foot-{tag}-{end}", leg, -HALF, 0, "contact", T(f"Rubber end cap of the {side_en} foot, {end_en}", f"Gummi-Endkappe des Fußes auf der {side_de}seite, {end_de}"), z=x_world - xoff, support="contact", fl=0.03, fw=0.035, mu=0.7))
ANCHORS += [
    anchor("case-rest", "case", CD / 2, 0, "contact", T("Underside of the soft case on the floor", "Unterseite des Softcase auf dem Boden"), z=0, support="contact", fl=CD, fw=CL, mu=0.5),
    anchor("lid-rest", "case-lid", CD / 2, CH, "contact", T("Outer face of the opened lid on the floor", "Außenseite des aufgeklappten Deckels auf dem Boden"), z=0, support="contact", fl=CD, fw=CL, mu=0.5),
    anchor("case-grip", "case", GRIP_U, GRIP_V, "support", T("Padded carrying grip in the hand", "Gepolsterter Tragegriff in der Hand"), z=0, support="pin"),
    anchor("pedal-rest", "pedal", PEDAL_L / 2, 0, "contact", T("Rubber sole of the sustain pedal", "Gummisohle des Haltepedals"), z=0, support="contact", fl=PEDAL_L, fw=PEDAL_W, mu=0.8),
    anchor("pedal-toe", "pedal-plate", 0.13, 0.008, "load", T("Ball of the right foot on the pedal lever", "Fußballen des rechten Fußes auf dem Pedalhebel"), z=0),
    anchor("keys-left-hand", "piano", 0.30, 0.095, "load", T("Left hand on the keys around C3", "Linke Hand auf den Tasten um C3"), z=-0.28),
    anchor("keys-right-hand", "piano", 0.30, 0.095, "load", T("Right hand on the keys around C5", "Rechte Hand auf den Tasten um C5"), z=0.24),
    anchor("panel-top", "piano", 0.09, PH, "emitter", T("Warm air above the top panel", "Warme Luft über dem Bedienfeld"), z=0),
]
SUPPORTS = FEET

# ---------------------------------------------------------------- thermal

REGION_IDS = ["piano-housing", "piano-keys", "case-shell", "stand-frame", "pedal", "power-supply"]
AREA = {"piano-housing": 1.35, "piano-keys": 0.55, "case-shell": 3.3, "stand-frame": 0.36, "pedal": 0.075, "power-supply": 0.03}


def layer(id, title, kind, material, t, res, cap, src="literature", refid="incropera-2018"):
    return {"id": id, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": res, "heatCapacityJM2K": cap, "source": src, "reference": refid}


def thermal_model():
    regions = [
        {"id": "piano-housing", "title": T("Housing of the stage piano: steel bottom and back, ABS panel, end cheeks", "Gehäuse des Stagepianos: Stahlboden und -rückwand, ABS-Bedienfeld, Seitenwangen"), "kind": "surface", "model": "operating", "material": "housing-steel", "areaM2": AREA["piano-housing"], "characteristicLengthM": 0.33, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5,
         "layers": [layer("housing-sheet", T("Powder-coated sheet steel 1.2 mm", "Pulverbeschichtetes Stahlblech 1,2 mm"), "metal", "housing-steel", 0.0012, 2.4e-5, 4430)], "heatCapacityJM2K": 6200, "operating": {"key": "power", "heatW": 10.4}, "ratedPowerW": 12, "ratedPowerBasis": "electrical-input", "source": "authored-estimate", "reference": "catalogue-stage-pianos"},
        {"id": "piano-keys", "title": T("The 88 keys: yellowed acrylic naturals and ABS sharps, shiny where played", "Die 88 Tasten: vergilbte Acryl-Untertasten und ABS-Obertasten, blank wo gespielt"), "kind": "surface", "model": "passive", "material": "key-acrylic", "areaM2": AREA["piano-keys"], "characteristicLengthM": 0.15, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5,
         "layers": [layer("key-top", T("Acrylic key top 2 mm on the key stick", "Acryl-Tastenbelag 2 mm auf dem Tastenhebel"), "polymer", "key-acrylic", 0.002, 0.0105, 3500)], "heatCapacityJM2K": 5200, "source": "literature", "reference": "incropera-2018"},
        {"id": "case-shell", "title": T("Padded soft case: nylon over EPS foam, open in its storage state", "Gepolstertes Softcase: Nylon über EPS-Schaum, im Lagerzustand geöffnet"), "kind": "surface", "model": "passive", "material": "case-nylon", "areaM2": AREA["case-shell"], "characteristicLengthM": 0.38, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5,
         "layers": [layer("case-foam", T("Nylon cover on 12 to 15 mm EPS foam", "Nylonbezug auf 12 bis 15 mm EPS-Schaum"), "insulation", "case-nylon", 0.015, 0.42, 1920, "authored-estimate", "iso-10456-2007")], "heatCapacityJM2K": 1920, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "iso-10456-2007"},
        {"id": "stand-frame", "title": T("X-stand: powder-coated steel tubes with rubber caps and foam sleeves", "X-Ständer: pulverbeschichtete Stahlrohre mit Gummikappen und Schaumhüllen"), "kind": "surface", "model": "passive", "material": "stand-steel", "areaM2": AREA["stand-frame"], "characteristicLengthM": 0.025, "geometry": "cylinder", "skyViewFactor": 0.5,
         "layers": [layer("tube-wall", T("Steel tube wall 1.5 mm", "Stahlrohrwand 1,5 mm"), "metal", "stand-steel", 0.0015, 3.0e-5, 5530, "literature", "en-10219-2006")], "heatCapacityJM2K": 5530, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "literature", "reference": "en-10219-2006"},
        {"id": "pedal", "title": T("Sustain pedal: ABS housing, rubber sole and chrome lever", "Haltepedal: ABS-Gehäuse, Gummisohle und Chromhebel"), "kind": "surface", "model": "passive", "material": "abs-black", "areaM2": AREA["pedal"], "characteristicLengthM": 0.18, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5,
         "layers": [layer("pedal-shell", T("ABS shell 2.5 mm", "ABS-Schale 2,5 mm"), "polymer", "abs-black", 0.0025, 0.0147, 3700, "literature", "iso-10456-2007")], "heatCapacityJM2K": 3700, "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "catalogue-pedals"},
        {"id": "power-supply", "title": T("External 12 V adapter with coiled cords", "Externes 12-V-Netzteil mit aufgerollten Kabeln"), "kind": "surface", "model": "operating", "material": "abs-black", "areaM2": AREA["power-supply"], "characteristicLengthM": 0.12, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5,
         "layers": [layer("psu-shell", T("ABS shell 2 mm over the potted converter", "ABS-Schale 2 mm über dem vergossenen Wandler"), "polymer", "abs-black", 0.002, 0.0118, 2950, "literature", "iso-10456-2007")], "heatCapacityJM2K": 16000, "operating": {"key": "power", "heatW": 1.6}, "source": "authored-estimate", "reference": "catalogue-stage-pianos"},
    ]
    return {
        "environment": {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
        "passengers": [],
        "operating": [{"id": "power", "title": T("Electrical power of the stage piano", "Elektrische Leistung des Stagepianos"), "levels": [{"v": 0, "title": T("Off, unplugged", "Aus, ausgesteckt")}, {"v": 0.5, "title": T("Switched on, silent (6 W)", "Eingeschaltet, stumm (6 W)")}, {"v": 1, "title": T("Played at full polyphony (12 W)", "Mit voller Polyphonie gespielt (12 W)")}]}],
        "regions": regions,
        "emitters": [{"id": "panel-plume", "kind": "plume", "title": T("Free convection above the warm top panel", "Freie Konvektion über dem warmen Bedienfeld"), "anchor": "panel-top", "directionDeg": 90, "diameterM": 0.3, "region": "piano-housing", "source": "authored-estimate", "reference": "incropera-2018"}],
        "assumptions": [
            T("The stage piano draws 12 W from the mains when played (catalogue-typical consumption of 88-key stage pianos with an external adapter, 9 to 20 W); 10.4 W leave through the housing, chiefly the top panel above the sound board, and 1.6 W are the conversion loss of the 12 V adapter. Switched on and silent it draws half of that; in storage and while carried it is unplugged.", "Das Stagepiano nimmt beim Spielen 12 W aus dem Netz auf (katalogübliche Aufnahme von Stagepianos mit 88 Tasten und externem Netzteil, 9 bis 20 W); 10,4 W verlassen das Gehäuse, vor allem über das Bedienfeld oberhalb der Klangplatine, und 1,6 W sind der Wandlungsverlust des 12-V-Netzteils. Eingeschaltet und stumm nimmt es die Hälfte auf; gelagert und getragen ist es ausgesteckt."),
            T("The keys, the case, the stand and the pedal are passive: they take the room temperature, the keys warm a little under the fingers (33 degC skin on 8 % of the key surface when played). The soft case is open in storage, so its lining counts as outer surface; the EPS foam is an aged, slightly compressed 15 mm shell (resistance about 0.42 m2K/W).", "Tasten, Softcase, Ständer und Pedal sind passiv: sie nehmen die Raumtemperatur an, die Tasten erwärmen sich unter den Fingern etwas (33 degC Haut auf 8 % der Tastenfläche beim Spielen). Das Softcase ist im Lager geöffnet, sein Futter zählt daher als Außenfläche; der EPS-Schaum ist eine gealterte, leicht gestauchte 15-mm-Schale (Widerstand etwa 0,42 m2K/W)."),
            T("Aged surfaces: the polished key tops and the worn chrome of the pedal lever are smoother than new ones; their emissivity is taken unchanged (0.91 acrylic, 0.15 worn chrome against 0.06 for new polished chrome, Incropera 2018 table A.11).", "Gealterte Oberflächen: die blank gespielten Tastenbeläge und das abgetretene Chrom des Pedalhebels sind glatter als neue; ihr Emissionsgrad wird unverändert angesetzt (0,91 Acryl, 0,15 abgetretenes Chrom gegenüber 0,06 für neues poliertes Chrom, Incropera 2018 Tabelle A.11)."),
        ],
    }


def binding(power, plume, housing, keys, case, stand, pedal, psu):
    """🌡 The thermal binding of a pose: operating level, emitter, and every region as (exposed, contact fraction, contact temp)."""
    def rb(e, c=None):
        b = {"exposedFraction": e, "sunlitFraction": 0}
        if c is not None:
            b["contact"] = {"fraction": c[0], "tempC": c[1]}
        return b
    return {"operating": {"power": power}, "regions": {"piano-housing": rb(*housing), "piano-keys": rb(*keys), "case-shell": rb(*case), "stand-frame": rb(*stand), "pedal": rb(*pedal), "power-supply": rb(*psu)}, "emitters": {"panel-plume": plume}}


# ---------------------------------------------------------------- poses and clips

DY_HUB = YP_P - YP_S
DTH = TH_P - TH_S
PIANO_DY_P = round(ARM_TOP_P - PY0, 5)


def stand_open(th):
    d = th - TH_S
    return {"stand-hub": {"dy": r(FOOT_R + HALF * math.sin(math.radians(th)) - YP_S, 6)}, "stand-leg-a": {"angleDeg": r(-d, 4)}, "stand-leg-b": {"angleDeg": r(d, 4)}}


ACC_FLOOR = {"pedal-slide": {"dx": -0.30}, "pedal": {"dy": -LID_IN}, "psu-slide": {"dx": -0.25}, "psu": {"dy": -LID_IN}}
ACC_PLAYED = {"pedal-slide": {"dx": 1.30}, "pedal": {"dy": -LID_IN}, "psu-slide": {"dx": 0.64}, "psu": {"dy": -LID_IN}}
CASE_PLAYED = {"case-slide": {"dx": -0.26}, "case-lid": {"angleDeg": -180}}
CASE_CARRIED = {"case-slide": {"dx": 0.24}, "case-lift": {"dy": 0.37}, "case": {"angleDeg": 90}, "case-lid": {"angleDeg": -180}, "piano-slide": {"dx": 0.193}, "piano-lift": {"dy": 0.373}, "piano": {"angleDeg": 90}}
PIANO_PLAYED = {"piano-slide": {"dx": 0.59}, "piano-lift": {"dy": PIANO_DY_P}}
PIANO_IN_PLAYED_CASE = {"piano-slide": {"dx": -0.26}}

B_STORED = binding(0, 0, (0.55, (0.12, 20)), (1, None), (0.85, (0.15, 20)), (1, (0.004, 20)), (0.9, (0.1, 20)), (0.85, (0.15, 20)))
B_CARRIED = binding(0, 0, (0.0, None), (0.0, None), (1, (0.004, 33)), (1, (0.004, 20)), (0.9, (0.1, 20)), (0.85, (0.15, 20)))
B_CLOSED = binding(0, 0, (0.0, None), (0.0, None), (0.8, (0.2, 20)), (1, (0.004, 20)), (0.9, (0.1, 20)), (0.85, (0.15, 20)))
B_PLAYED = binding(1, 1, (0.93, (0.02, 21)), (1, (0.08, 33)), (0.8, (0.2, 20)), (1, (0.004, 20)), (0.9, (0.12, 21)), (0.85, (0.15, 20)))

LOADS_PLAYED = [
    {"id": "left-hand", "kind": "occupant", "title": T("Left hand resting and playing on the keys", "Linke Hand ruht und spielt auf den Tasten"), "anchor": "keys-left-hand", "fxN": 0, "fyN": -15, "source": "literature", "reference": "askenfelt-jansson-1991"},
    {"id": "right-hand", "kind": "occupant", "title": T("Right hand resting and playing on the keys", "Rechte Hand ruht und spielt auf den Tasten"), "anchor": "keys-right-hand", "fxN": 0, "fyN": -15, "source": "literature", "reference": "askenfelt-jansson-1991"},
    {"id": "pedal-foot", "kind": "occupant", "title": T("Right foot on the sustain pedal", "Rechter Fuß auf dem Haltepedal"), "anchor": "pedal-toe", "fxN": 0, "fyN": -25, "source": "authored-estimate", "reference": "catalogue-pedals"},
]


def merge(*ds):
    out = {}
    for d in ds:
        for k, v in d.items():
            out.setdefault(k, {}).update(v)
    return out


def pose(id, title, desc, bones, contacts, thermal, loads=None, key=False):
    p = {"id": id, "title": title, "description": desc}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "authored", "contacts": contacts, "thermal": thermal, "loads": loads or []})
    return p


POSES = [
    pose("stored", T("Stored at the dealer", "Beim Händler gelagert"), T("The cleaned stage piano lies in its opened soft case on the showroom floor, the lid folded back with the sustain pedal and the adapter in it, the X-stand folded flat in front of it: ready to be shown and tried.", "Das gereinigte Stagepiano liegt im geöffneten Softcase auf dem Boden des Verkaufsraums, der Deckel ist zurückgeklappt und enthält Haltepedal und Netzteil, davor liegt der flach zusammengeklappte X-Ständer: bereit zum Vorführen und Anspielen."), {}, FEET + ["case-rest", "lid-rest"], B_STORED),
    pose("carried", T("Carried in the soft case", "Im Softcase getragen"), T("Zipped into its case the piano hangs from the padded grip in one hand, 21 kg at knuckle height 0.78 m; the folded stand, the pedal and the adapter stay behind on the floor.", "Ins Softcase gezippt hängt das Piano am gepolsterten Griff in einer Hand, 21 kg auf Knöchelhöhe 0,78 m; der zusammengeklappte Ständer, das Pedal und das Netzteil bleiben auf dem Boden zurück."), merge(CASE_CARRIED, ACC_FLOOR), FEET + ["case-grip", "pedal-rest"], B_CARRIED),
    pose("played", T("Played on the X-stand", "Auf dem X-Ständer gespielt"), T("The X-stand is opened to the playing notch, the piano rests on its foam-sleeved arms with the white keys at 0.74 m for a player on a stool, the sustain pedal lies under the right foot and the adapter behind; the empty case lies closed on the floor behind.", "Der X-Ständer ist auf die Spielrast geöffnet, das Piano liegt auf den schaumgepolsterten Armen mit den weißen Tasten auf 0,74 m für eine Spielerin auf einem Hocker, das Haltepedal liegt unter dem rechten Fuß und das Netzteil dahinter; das leere Case liegt geschlossen am Boden dahinter."), merge(stand_open(TH_P), CASE_PLAYED, PIANO_PLAYED, ACC_PLAYED), FEET + ["case-rest", "pedal-rest"], B_PLAYED, LOADS_PLAYED),
    pose("accessories-raised", T("Pedal and adapter lifted out of the lid", "Pedal und Netzteil aus dem Deckel gehoben"), T("Key pose of the carry clip.", "Schlüsselpose des Trageclips."), {"pedal": {"dy": 0.095}, "psu": {"dy": 0.095}}, FEET + ["case-rest", "lid-rest"], B_STORED, key=True),
    pose("accessories-aside", T("Pedal and adapter held beside the lid", "Pedal und Netzteil neben dem Deckel gehalten"), T("Key pose of the carry clip.", "Schlüsselpose des Trageclips."), {"pedal-slide": {"dx": -0.30}, "pedal": {"dy": 0.095}, "psu-slide": {"dx": -0.25}, "psu": {"dy": 0.095}}, FEET + ["case-rest", "lid-rest"], B_STORED, key=True),
    pose("accessories-down", T("Pedal and adapter set down on the floor", "Pedal und Netzteil auf den Boden gelegt"), T("Key pose of the carry clip.", "Schlüsselpose des Trageclips."), ACC_FLOOR, FEET + ["case-rest", "lid-rest", "pedal-rest"], B_STORED, key=True),
    pose("case-closed", T("Case zipped shut on the floor", "Case auf dem Boden zugezippt"), T("Key pose of the carry clip: the lid is folded over the piano and zipped.", "Schlüsselpose des Trageclips: der Deckel ist über das Piano geklappt und zugezippt."), merge(ACC_FLOOR, {"case-lid": {"angleDeg": -180}}), FEET + ["case-rest", "pedal-rest"], B_CLOSED, key=True),
    pose("stand-folded", T("Stand folded, case closed beside it", "Ständer zusammengeklappt, Case daneben geschlossen"), T("Key pose of the stand clip.", "Schlüsselpose des Ständerclips."), merge(CASE_PLAYED, PIANO_IN_PLAYED_CASE, ACC_PLAYED), FEET + ["case-rest", "pedal-rest"], B_CLOSED, key=True),
    pose("stand-open", T("Stand opened to the playing notch", "Ständer auf die Spielrast geöffnet"), T("Key pose of the stand clip.", "Schlüsselpose des Ständerclips."), merge(stand_open(TH_P), CASE_PLAYED, PIANO_IN_PLAYED_CASE, ACC_PLAYED), FEET + ["case-rest", "pedal-rest"], B_CLOSED, key=True),
    pose("pedal-down", T("Sustain pedal held down", "Haltepedal gedrückt"), T("Key pose of the pedal clip.", "Schlüsselpose des Pedalclips."), merge(stand_open(TH_P), CASE_PLAYED, PIANO_PLAYED, ACC_PLAYED, {"pedal-plate": {"angleDeg": -9}}), FEET + ["case-rest", "pedal-rest"], B_PLAYED, LOADS_PLAYED, key=True),
]


def span(anchor_id, a, b):
    return {"anchor": anchor_id, "fromS": a, "toS": b}


def unfold_keys():
    keys = []
    for k in range(5):
        th = TH_S + DTH * k / 4
        if k == 0:
            keys.append({"t": 0, "pose": "stand-folded", "ease": "smooth"})
        elif k == 4:
            keys.append({"t": 3.0, "pose": "stand-open", "ease": "smooth"})
        else:
            keys.append({"t": r(0.75 * k, 3), "pose": "stand-folded", "ease": "smooth", "bones": stand_open(th)})
    return keys


CLIPS = [
    {"id": "carry", "title": T("Pack up and carry away", "Einpacken und forttragen"), "description": T("Pedal and adapter are lifted out of the lid and set down beside it, the lid is folded over the piano and zipped, and the case is lifted by its grip and turned on edge to hang from the hand.", "Pedal und Netzteil werden aus dem Deckel gehoben und daneben abgelegt, der Deckel wird über das Piano geklappt und zugezippt, dann wird das Case am Griff angehoben und hochkant in die Hand genommen."),
     "durationS": 6.0, "fps": 10, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.8, "pose": "accessories-raised", "ease": "linear"}, {"t": 1.8, "pose": "accessories-aside", "ease": "smooth"}, {"t": 2.3, "pose": "accessories-down", "ease": "smooth"}, {"t": 3.8, "pose": "case-closed", "ease": "smooth"}, {"t": 6.0, "pose": "carried", "ease": "smooth"}],
     "contacts": [span(f, 0, 6.0) for f in FEET] + [span("case-rest", 0, 3.9), span("lid-rest", 0, 2.4), span("pedal-rest", 2.3, 6.0), span("case-grip", 3.9, 6.0)],
     "phases": [{"id": "accessories", "fromS": 0, "toS": 2.3, "title": T("Accessories out", "Zubehör heraus")}, {"id": "closing", "fromS": 2.3, "toS": 3.8, "title": T("Lid closed", "Deckel geschlossen")}, {"id": "lifting", "fromS": 3.8, "toS": 6.0, "title": T("Lifted and carried", "Angehoben und getragen")}],
     "events": [{"t": 2.3, "kind": "contact-on", "title": T("The pedal is set down on the floor", "Das Pedal liegt auf dem Boden"), "target": "pedal"}, {"t": 2.4, "kind": "contact-off", "title": T("The lid leaves the floor", "Der Deckel hebt vom Boden ab"), "target": "case-lid"}, {"t": 3.9, "kind": "load-on", "title": T("The hand takes the 21 kg at the grip", "Die Hand übernimmt die 21 kg am Griff"), "target": "case"}],
     "tracks": []},
    {"id": "unfold-stand", "title": T("Unfold the X-stand", "X-Ständer aufklappen"), "description": T("The folded X-stand is opened by lifting its arms: the feet slide apart on the floor until the pull knob clicks into the playing notch at 43 degrees, arms 0.645 m above the floor.", "Der zusammengeklappte X-Ständer wird an den Armen angehoben: die Füße gleiten auf dem Boden auseinander, bis der Zugknopf bei 43 Grad in die Spielrast einrastet, die Arme 0,645 m über dem Boden."),
     "durationS": 3.0, "fps": 15, "loop": False, "rootMotion": "none", "keys": unfold_keys(),
     "contacts": [span(f, 0, 3.0) for f in FEET] + [span("case-rest", 0, 3.0), span("pedal-rest", 0, 3.0)],
     "phases": [{"id": "opening", "fromS": 0, "toS": 2.8, "title": T("Opening", "Öffnen")}, {"id": "locked", "fromS": 2.8, "toS": 3.0, "title": T("Locked", "Eingerastet")}],
     "events": [{"t": 2.9, "kind": "motion", "title": T("The pull knob clicks into the playing notch", "Der Zugknopf rastet in der Spielrast ein"), "target": "stand-hub"}],
     "tracks": []},
    {"id": "sustain-pedal", "title": T("Play with the sustain pedal", "Mit dem Haltepedal spielen"), "description": T("While both hands play, the right foot presses the sustain pedal down by 9 degrees, holds it for a bar and lets it rise again: a looping pedal cycle of two seconds at full power.", "Während beide Hände spielen, drückt der rechte Fuß das Haltepedal um 9 Grad nieder, hält es einen Takt lang und lässt es wieder steigen: ein zwei Sekunden langer Pedalzyklus in Schleife bei voller Leistung."),
     "durationS": 2.0, "fps": 25, "loop": True, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "played", "ease": "smooth"}, {"t": 0.3, "pose": "pedal-down", "ease": "linear"}, {"t": 1.2, "pose": "pedal-down", "ease": "smooth"}, {"t": 1.5, "pose": "played", "ease": "linear"}, {"t": 2.0, "pose": "played", "ease": "smooth"}],
     "contacts": [span(f, 0, 2.0) for f in FEET] + [span("case-rest", 0, 2.0), span("pedal-rest", 0, 2.0)],
     "phases": [{"id": "pressing", "fromS": 0, "toS": 0.3, "title": T("Pressing", "Niederdrücken")}, {"id": "held", "fromS": 0.3, "toS": 1.2, "title": T("Held", "Gehalten")}, {"id": "release", "fromS": 1.2, "toS": 2.0, "title": T("Released", "Losgelassen")}],
     "events": [{"t": 0.3, "kind": "motion", "title": T("Dampers lifted: the sustain switch closes", "Dämpfer gehoben: der Halteschalter schließt"), "target": "pedal-plate"}, {"t": 1.5, "kind": "motion", "title": T("The pedal is up again", "Das Pedal ist wieder oben"), "target": "pedal-plate"}],
     "tracks": [{"target": "load:pedal-foot", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": 0.3, "v": 1.6}, {"t": 1.2, "v": 1.6}, {"t": 1.5, "v": 1}, {"t": 2.0, "v": 1}]}]},
]


# ---------------------------------------------------------------- decisions

REP_DECISIONS = {
    50: {
        "silhouette": T("Side view from the bass end: the folded X-stand as a 66 mm bundle of two crossbars and the two crossing tubes, the soft case as two 90 mm shells with the piano rising 72 mm out of the lower one and its grip, the bass end cheek with its rounded front. Front view: the 1.33 m keyboard with key slip, white key lips, the 36 sharps and the end cheeks above the 1.40 m case and the flat X of the stand. Plan: the open case and lid with the pedal and adapter in the lid, the 88 keys, the top panel with display and slider, the stand bundle.", "Seitenansicht vom Bassende: der zusammengeklappte X-Ständer als 66 mm hohes Bündel aus zwei Querstreben und den zwei gekreuzten Rohren, das Softcase als zwei 90-mm-Schalen, aus deren unterer das Piano 72 mm herausragt, mit seinem Griff, die Bass-Seitenwange mit gerundeter Front. Vorderansicht: die 1,33 m lange Tastatur mit Tastenleiste, Untertastenkanten, den 36 Obertasten und den Seitenwangen über dem 1,40 m langen Case und dem flachen X des Ständers. Draufsicht: offenes Case und Deckel mit Pedal und Netzteil im Deckel, die 88 Tasten, das Bedienfeld mit Display und Schieberegler, das Ständerbündel."),
        "retained": T("Overall 1.375 x 0.162 x 1.40 m; key surface 95 mm above the plate, sharps 12 mm higher, 23.5 mm naturals and 12 mm sharps in their real places, cheeks 54 mm, 15 mm case walls, 25 mm square tubes, 25 mm crossbars with 30 mm caps and 32 mm sleeves, the pivot bolt with its pull knob, the pedal lever at 9 degrees.", "Gesamt 1,375 x 0,162 x 1,40 m; Tastenoberfläche 95 mm über der Bodenplatte, Obertasten 12 mm höher, Untertasten 23,5 mm und Obertasten 12 mm an ihren wirklichen Orten, Wangen 54 mm, Casewände 15 mm, Vierkantrohre 25 mm, Querstreben 25 mm mit 30-mm-Kappen und 32-mm-Hüllen, der Gelenkbolzen mit Zugknopf, der Pedalhebel unter 9 Grad."),
        "merged": T("The white keys are one filled band with the key joints as texture strokes; the housing is drawn as back body, keybed and key slip; the case shells are one end profile each.", "Die Untertasten sind ein gefülltes Band mit den Tastenfugen als Texturstriche; das Gehäuse ist als Rückenteil, Tastenbett und Tastenleiste gezeichnet; die Caseschalen sind je ein Stirnprofil."),
        "omitted": T("The hammer action under the keys, the screws, the cables between pedal, adapter and piano (they lie coiled or are unplugged), the zip teeth.", "Die Hammermechanik unter den Tasten, die Schrauben, die Kabel zwischen Pedal, Netzteil und Piano (sie liegen aufgerollt oder sind ausgesteckt), die Reißverschlusszähne."),
        "thinElements": T("Key joints, shine on the much-played middle keys, tape residue and scuffs are texture strokes; the stand tubes and crossbars are drawn as members with their true sections.", "Tastenfugen, Glanz auf den viel gespielten mittleren Tasten, Klebebandreste und Abschürfungen sind Texturstriche; Ständerrohre und Querstreben sind als Stäbe mit ihren wahren Querschnitten gezeichnet."),
        "hatching": T("A fine 45 degree weave hatch on the end faces of the nylon case.", "Eine feine 45-Grad-Gewebeschraffur auf den Stirnseiten des Nylon-Case."),
        "styles": T("Body pen 0.35 mm, detail 0.18, texture 0.13, glazing 0.18 for the display; housing dark (0.2 to 0.34), naturals light (0.86), sharps black (0.08), case mid grey (0.42), stand 0.22.", "Körperstift 0,35 mm, Detail 0,18, Textur 0,13, Verglasung 0,18 für das Display; Gehäuse dunkel (0,2 bis 0,34), Untertasten hell (0,86), Obertasten schwarz (0,08), Case mittelgrau (0,42), Ständer 0,22."),
        "overlays": T("Housing, keys, case, stand, pedal and adapter are separate regions, so the thermal view shows the warm panel and adapter of a played piano apart from the passive parts.", "Gehäuse, Tasten, Case, Ständer, Pedal und Netzteil sind getrennte Regionen, sodass die Thermalansicht das warme Bedienfeld und das Netzteil eines gespielten Pianos von den passiven Teilen trennt."),
    },
    100: {
        "silhouette": T("The same three views with the case grip, the pivot knob, the end cheeks, the key slip and the 36 sharps as thin strokes on the white band.", "Dieselben drei Ansichten mit Casegriff, Gelenkknopf, Seitenwangen, Tastenleiste und den 36 Obertasten als dünne Striche auf dem weißen Band."),
        "retained": T("All overall dimensions, the stand tubes and crossbars with caps and sleeves, the case walls and lining, the volume slider, pedal and adapter as compact bodies.", "Alle Gesamtmaße, Ständerrohre und Querstreben mit Kappen und Hüllen, Casewände und Futter, Schieberegler, Pedal und Netzteil als kompakte Körper."),
        "merged": T("The pedal housing, sole and lever are one wedge; bolt and nut of the pivot are one pin; the white keys are one band.", "Pedalgehäuse, Sohle und Hebel sind ein Keil; Bolzen und Mutter des Gelenks sind ein Stift; die Untertasten sind ein Band."),
        "omitted": T("Key joints, shine, stickers, tape residue, display, zip line and the cord coil.", "Tastenfugen, Glanz, Aufkleber, Klebebandreste, Display, Reißverschlusslinie und die Kabelrolle."),
        "thinElements": T("The sharps are texture outlines; tubes and crossbars stay members of their true section.", "Die Obertasten sind Texturumrisse; Rohre und Querstreben bleiben Stäbe mit wahrem Querschnitt."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.25 mm, detail 0.13, texture 0.1; tones as at 1:50.", "Körperstift 0,25 mm, Detail 0,13, Textur 0,1; Tonwerte wie bei 1:50."),
        "overlays": T("Six regions as at 1:50.", "Sechs Regionen wie bei 1:50."),
    },
    200: {
        "silhouette": T("Three views of masses: piano as back body, front rail and key band, case shells with rim and lining, the stand bundle, pedal wedge and adapter brick.", "Drei Ansichten aus Massen: Piano als Rückenteil, Frontleiste und Tastenband, Caseschalen mit Rand und Futter, das Ständerbündel, Pedalkeil und Netzteilquader."),
        "retained": T("Overall dimensions, the 30 mm crossbars (caps and sleeves merged), the 25 mm tubes, the knob of the pivot, the cheeks.", "Gesamtmaße, die 30-mm-Querstreben (Kappen und Hüllen eingeschlossen), die 25-mm-Rohre, der Gelenkknopf, die Wangen."),
        "merged": T("Keybed and key slip are one front rail; knob, bolt and nut are one hub; caps and sleeves are merged into the crossbars.", "Tastenbett und Tastenleiste sind eine Frontleiste; Knopf, Bolzen und Mutter sind eine Nabe; Kappen und Hüllen sind in die Querstreben eingeschlossen."),
        "omitted": T("The sharps, the slider, the grip of the case (28 mm, under the 30 mm feature) and every trace of use.", "Die Obertasten, der Schieberegler, der Casegriff (28 mm, unter der Mindestgröße von 30 mm) und alle Gebrauchsspuren."),
        "thinElements": T("Tubes as members of their true section.", "Rohre als Stäbe mit wahrem Querschnitt."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.18 mm, detail 0.13; three tones for piano, case and stand.", "Körperstift 0,18 mm, Detail 0,13; drei Tonwerte für Piano, Case und Ständer."),
        "overlays": T("Six regions.", "Sechs Regionen."),
    },
    500: {
        "silhouette": T("The open case with the piano as two blocks (housing and keys), the lid as a shell, the folded stand as its two tubes and four crossbar rods.", "Das offene Case mit dem Piano als zwei Blöcke (Gehäuse und Tasten), der Deckel als Schale, der zusammengeklappte Ständer als seine zwei Rohre und vier Querstabstäbe."),
        "retained": T("Overall dimensions, the open lid and the opening of the case, the X of the stand.", "Gesamtmaße, der offene Deckel und die Öffnung des Case, das X des Ständers."),
        "merged": T("Pedal and adapter are merged into the lid shell in which they lie (their regions are listed on the lid shape); the cheeks, keybed and keys are the key block; the feet close the 10 mm under the piano.", "Pedal und Netzteil sind in die Deckelschale eingeschlossen, in der sie liegen (ihre Regionen stehen an der Deckelform); Wangen, Tastenbett und Tasten sind der Tastenblock; die Füße schließen die 10 mm unter dem Piano."),
        "omitted": T("Grip, pivot hub, caps and sleeves, every detail of the panel.", "Griff, Gelenknabe, Kappen und Hüllen, alle Einzelheiten des Bedienfelds."),
        "thinElements": T("Tubes and crossbars are drawn as members of their true section.", "Rohre und Querstreben sind als Stäbe mit wahrem Querschnitt gezeichnet."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.18 mm; two tones for piano and case, dark stand.", "Körperstift 0,18 mm; zwei Tonwerte für Piano und Case, dunkler Ständer."),
        "overlays": T("Six regions, pedal and adapter painted with the lid.", "Sechs Regionen, Pedal und Netzteil mit dem Deckel gemalt."),
    },
}

VOL_DECISIONS = {
    50: [T("Body at 1:50: the real objects. The piano has its two end cheeks, back body, keybed and key slip, 52 naturals of 22.5 mm with 1 mm joints and 36 sharps in their key-cutting positions, four rubber feet, display, slider and the traces of use (shine on the middle keys, price sticker, tape residue, scuffed cheek, jack panel). The case is two EPS shells of 15 mm walls with 12 and 15 mm panels, the padded grip on two webbing tabs, zip bead and scuffed corners. The X-stand has two 25 mm square tubes on the pivot bolt with nut and pull knob, four 25 mm crossbars, rubber end caps, foam sleeves and chipped coating. Pedal housing, rubber sole, chrome lever on its pin and cord stub; adapter brick with its cord coil and label.", "Body bei 1:50: die realen Gegenstände. Das Piano hat seine zwei Seitenwangen, Rückenteil, Tastenbett und Tastenleiste, 52 Untertasten von 22,5 mm mit 1 mm Fugen und 36 Obertasten an ihren Teilungsorten, vier Gummifüße, Display, Schieberegler und die Gebrauchsspuren (Glanz auf den mittleren Tasten, Preisetikett, Klebebandreste, abgeschürfte Wange, Buchsenfeld). Das Case sind zwei EPS-Schalen mit 15-mm-Wänden und 12- und 15-mm-Böden, der gepolsterte Griff an zwei Gurtlaschen, Reißverschlusswulst und abgeschürfte Ecken. Der X-Ständer hat zwei 25-mm-Vierkantrohre am Gelenkbolzen mit Mutter und Zugknopf, vier 25-mm-Querstreben, Gummi-Endkappen, Schaumhüllen und abgeplatzte Beschichtung. Pedalgehäuse, Gummisohle, Chromhebel am Stift und Kabelstummel; Netzteilquader mit Kabelrolle und Etikett."),
       T("Thermal and structure: the thermal list references the body per region with the plume outlet on the top panel; the structure list is the load path: tubes and crossbars of the stand on its four caps, the pivot as joint, the case shells on their undersides with the grip as support for carrying, the bottom plate, back wall, cheeks, key frame rail and feet of the piano, pedal housing and lever with the hinge pin, the adapter brick, and a mass marker at every centre of mass.", "Thermik und Struktur: die Thermikliste verweist je Region auf den Body, mit dem Fahnenauslass auf dem Bedienfeld; die Strukturliste ist der Lastpfad: Rohre und Querstreben des Ständers auf seinen vier Kappen, das Gelenk als Knoten, die Caseschalen auf ihren Unterseiten mit dem Griff als Auflager beim Tragen, Bodenplatte, Rückwand, Wangen, Tastenrahmenschiene und Füße des Pianos, Pedalgehäuse und Hebel mit Scharnierstift, der Netzteilquader und ein Massenpunkt in jedem Schwerpunkt.")],
    100: [T("1:100: naturals one band, sharps texture solids, feet 15 mm high (5 mm into the plate), pivot as one 16 mm pin with the knob, pedal one wedge, adapter one brick; case panels declared sheets; traces of use, display, zip and cord coil left out.", "1:100: Untertasten ein Band, Obertasten Texturkörper, Füße 15 mm hoch (5 mm in die Bodenplatte), Gelenk als ein 16-mm-Stift mit Knopf, Pedal ein Keil, Netzteil ein Quader; Caseböden als Platten erklärt; Gebrauchsspuren, Display, Reißverschluss und Kabelrolle weggelassen.")],
    200: [T("1:200: keybed and key slip one front rail, sharps omitted, feet 30 mm, crossbars 30 mm with caps and sleeves merged, hub one 32 mm knob, case grip left out (a support block stands for it in the structure).", "1:200: Tastenbett und Tastenleiste eine Frontleiste, Obertasten weggelassen, Füße 30 mm, Querstreben 30 mm mit eingeschlossenen Kappen und Hüllen, Nabe ein 32-mm-Knopf, Casegriff weggelassen (ein Auflagerblock steht in der Struktur dafür).")],
    500: [T("1:500: piano as housing block and key block down to the floor of the case, case shells with true 15 mm walls (judged by their overall extents) and sheet panels, stand tubes slender at their true section and the crossbars as 21.5 mm slender rods, pedal and adapter merged into the lid in which they lie (their thermal regions keep feature-sized solids there), hub and grip left out.", "1:500: Piano als Gehäuseblock und Tastenblock bis auf den Caseboden, Caseschalen mit wahren 15-mm-Wänden (nach ihren Gesamtmaßen beurteilt) und Plattenböden, Ständerrohre schlank mit wahrem Querschnitt und die Querstreben als schlanke 21,5-mm-Stäbe, Pedal und Netzteil in den Deckel eingeschlossen, in dem sie liegen (ihre thermischen Regionen behalten dort Körper in Mindestgröße), Nabe und Griff weggelassen.")],
}

# ---------------------------------------------------------------- assembly


def build_scale(sc):
    MASS_R[0] = max(0.012, FEATURE[sc] / 2 + 0.0005)
    D = Drawing()
    L = Lists()
    piano_parts(sc, D, L)
    case_parts(sc, D, L)
    stand_parts(sc, D, L)
    pedal_parts(sc, D, L)
    psu_parts(sc, D, L)
    L.thermal.append(box("th-panel-plume", "piano", "housing-steel", "texture", 0.6, (0.09, PH + 0.0003, 0), ({50: 0.12, 100: 0.13, 200: 0.14, 500: 0.15}[sc], 0.0006, {50: 0.6, 100: 0.62, 200: 0.64, 500: 0.66}[sc]), texture=True, region="piano-housing", tags=["panel-plume"]))
    if sc == 500:
        for s in D.e + D.f + D.p:
            if s["id"] in ("lid-end", "lid-lining"):
                s.pop("region", None)
                s["regions"] = ["case-shell", "pedal", "power-supply"]
    hatches = [{"id": "case-weave", "title": T("Nylon weave of the case", "Nylongewebe des Case"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.6, "penMm": 0.1}] if sc == 50 else []
    rep = {"status": "authored", "denominator": sc, "validRange": {"minDenominator": VALID[sc][0], "maxDenominator": VALID[sc][1]}, "pens": pens(sc), "hatches": hatches,
           "geometry": {"elevation": D.e, "front": D.f, "plan": D.p}, "decisions": REP_DECISIONS[sc]}
    vol = {"status": "authored", "denominator": sc, "body": L.body, "thermal": L.thermal, "structure": L.structure, "decisions": VOL_DECISIONS[sc]}
    return rep, vol


REFERENCES = [
    {"id": "hornbostel-sachs-1914", "citation": "von Hornbostel, E. M., Sachs, C. (1914). Systematik der Musikinstrumente. Ein Versuch. Zeitschrift für Ethnologie 46, 553-590; electrophones added by Sachs, C. (1940). The History of Musical Instruments. New York: Norton."},
    {"id": "mimo-2011", "citation": "MIMO Consortium (2011). Revision of the Hornbostel-Sachs Classification of Musical Instruments, class 5 Electrophones: 53 radioelectric instruments (sound generated by electronic means; a sample-playback stage piano with a weighted keyboard is 53 with a keyboard as controller)."},
    {"id": "fletcher-rossing-1998", "citation": "Fletcher, N. H., Rossing, T. D. (1998). The Physics of Musical Instruments, 2nd ed. New York: Springer, chapter 12 (the piano: 88 keys A0-C8, octave span about 165 mm)."},
    {"id": "askenfelt-jansson-1991", "citation": "Askenfelt, A., Jansson, E. V. (1991). From touch to string vibrations. II: The motion of the key and hammer. Journal of the Acoustical Society of America 90(5), 2383-2393 (key forces of a few newtons to above 30 N in forte playing)."},
    {"id": "catalogue-stage-pianos", "citation": "Catalogue-typical data of 88-key stage pianos with weighted hammer action and external 12 V adapter, 2010s: 1.28-1.34 m long, 0.30-0.36 m deep, 0.13-0.19 m high, 16-23 kg, power consumption 9-20 W, key top about 95 mm above the bottom (for example Roland RD-88, Yamaha CP88, Kawai MP7SE owner's manuals); type, not a brand; authored estimate."},
    {"id": "catalogue-x-stands", "citation": "Catalogue-typical data of single-braced X keyboard stands: square steel tube 25 x 25 mm, crossbars 0.40-0.45 m with rubber caps and foam sleeves, height 0.07 m folded to about 0.9 m, playing notch about 0.65 m, 3.5-4.5 kg, load rating 40-50 kg; authored estimate."},
    {"id": "catalogue-soft-cases", "citation": "Catalogue-typical data of 88-key keyboard soft cases (EPS foam shells covered with 600-denier nylon, zipped around the middle): 1.38-1.45 m x 0.36-0.42 m x 0.16-0.20 m, 3-4.5 kg, carrying grip on the long side; authored estimate."},
    {"id": "catalogue-pedals", "citation": "Catalogue-typical data of piano-style sustain pedals for stage pianos: about 100 x 180 x 60 mm, 0.4-0.6 kg with cord, lever travel about 9 degrees, return spring force 10-20 N; authored estimate."},
    {"id": "din-33402-2-2005", "citation": "DIN 33402-2:2005-12 Ergonomie - Körpermaße des Menschen - Teil 2: Werte (knuckle height of adults about 0.75-0.80 m, seated elbow height for a keyboard at 0.72-0.76 m)."},
    {"id": "en-10219-2006", "citation": "EN 10219-2:2006 Cold formed welded structural hollow sections of non-alloy and fine grain steels - Part 2: Tolerances, dimensions and sectional properties (square 25 x 25 x 1.5: A = 1.41 cm2, I = 1.30 cm4)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities, conductivities and heat capacities; free convection above heated plates)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (EPS foam, polymers, rubber)."},
    {"id": "iec-62301-2011", "citation": "IEC 62301:2011 Household electrical appliances - Measurement of standby power (method behind the stated consumption of switched-on and idle electronic instruments)."},
]


def document():
    reps, vols = {}, {}
    for sc in SCALES:
        reps[str(sc)], vols[str(sc)] = build_scale(sc)
    asset = {
        "id": "used-stage-piano",
        "category": "appliances",
        "title": T("Used Stage Piano with Stand", "Gebrauchtes Stagepiano mit Ständer"),
        "summary": T("A second-hand 88-key stage piano of the 2010s, 1.33 m long and 17.5 kg, with its folded single-braced X-stand, piano-style sustain pedal, 12 V adapter and padded soft case, as a used-instrument dealer shows it: cleaned, keys yellowed and polished to a shine in the middle octaves, a price sticker and tape residue on the panel, scuffed cheeks and case corners, chipped coating on the stand. It is carried zipped in its case by one person and played on the opened stand with the keys at 0.74 m, drawing 12 W.", "Ein gebrauchtes Stagepiano der 2010er-Jahre mit 88 Tasten, 1,33 m lang und 17,5 kg, mit zusammengeklapptem einstrebigem X-Ständer, Haltepedal in Klavierform, 12-V-Netzteil und gepolstertem Softcase, wie es ein Gebrauchtinstrumentenhändler zeigt: gereinigt, die Tasten vergilbt und in den mittleren Oktaven blank gespielt, ein Preisetikett und Klebebandreste auf dem Bedienfeld, abgeschürfte Wangen und Case-Ecken, abgeplatzte Beschichtung am Ständer. Es wird zugezippt im Case von einer Person getragen und auf dem geöffneten Ständer mit den Tasten auf 0,74 m gespielt und nimmt dabei 12 W auf."),
        "tags": ["used", "instrument", "music", "second-hand", "keyboard", "stage", "amplification", "stage-piano", "digital-piano", "electrophone", "x-stand", "soft-case", "sustain-pedal", "steel", "nylon", "abs", "electrical", "internal-gain"],
        "provenance": {"author": "Semio staffage content (used instruments wave, row 11)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                        "notes": T("Hornbostel-Sachs 53 (radioelectric electrophone, digital sample playback, keyboard-controlled; MIMO 2011). Modelled to the type of a 2010s 88-key stage piano with weighted hammer action: 1.330 x 0.330 x 0.140 m on four 10 mm feet, 17.5 kg, 52 naturals of 23.5 mm pitch (octave 164.5 mm, keyboard 1.222 m between 54 mm end cheeks), sharps 12 mm wide, 12 mm above the naturals, key top 95 mm above the bottom plate; single-braced X-stand of 25 x 25 x 1.5 mm steel tube, 0.90 m between crossbar centres, folded at 2.2 degrees (66 mm high), opened to 43 degrees with the arms at 0.645 m so the keys are at 0.74 m; EPS soft case 1.40 x 0.38 x 0.18 m, 3.7 kg, zipped around the middle; piano-style sustain pedal 180 x 100 x 63 mm; 12 V adapter 120 x 55 x 35 mm. Everything is an authored estimate of the type from the catalogue ranges cited, no single product; nothing is measured. Total 25.71 kg.", "Hornbostel-Sachs 53 (radioelektrisches Elektrophon, digitale Sample-Wiedergabe, über eine Tastatur gesteuert; MIMO 2011). Modelliert nach dem Typ eines Stagepianos der 2010er-Jahre mit 88 gewichteten Hammermechanik-Tasten: 1,330 x 0,330 x 0,140 m auf vier 10 mm hohen Füßen, 17,5 kg, 52 Untertasten im Raster 23,5 mm (Oktave 164,5 mm, Tastatur 1,222 m zwischen 54 mm breiten Seitenwangen), Obertasten 12 mm breit und 12 mm über den Untertasten, Tastenoberfläche 95 mm über der Bodenplatte; einstrebiger X-Ständer aus Stahlrohr 25 x 25 x 1,5 mm, 0,90 m zwischen den Querstrebenmitten, zusammengeklappt bei 2,2 Grad (66 mm hoch), auf 43 Grad geöffnet mit den Armen auf 0,645 m, sodass die Tasten auf 0,74 m liegen; EPS-Softcase 1,40 x 0,38 x 0,18 m, 3,7 kg, rundum in der Mitte gezippt; Haltepedal in Klavierform 180 x 100 x 63 mm; 12-V-Netzteil 120 x 55 x 35 mm. Alles ist ein Schätzwert des Typs aus den zitierten Katalogbereichen, kein einzelnes Produkt; nichts ist gemessen. Gesamt 25,71 kg.")},
        "dimensions": {"heightM": 0.170, "widthM": 1.375, "depthM": 1.40, "massKg": MASS_TOTAL},
        "materials": MATERIALS,
        "rig": {"kind": "mechanical", "root": "floor", "bones": BONES},
        "anchors": ANCHORS,
        "representations": reps,
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": thermal_model(),
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": SUPPORTS, "assumptions": [
            T("Stored, the open case rests on its underside and its lid, the folded stand on its four rubber caps; played, the 172 N of the piano and up to 30 N of the hands pass through the foam-sleeved arms into the two 25 mm tubes (0.42 kN of compression each at most, 1 % of their buckling load) and the four caps; carried, one hand holds the 210 N of the zipped case at the grip, a pin support. Each part is a rigid body; the asset is solved as one rigid body on the supports of its pose, as the product does for every asset.", "Gelagert ruht das offene Case auf seiner Unterseite und seinem Deckel, der zusammengeklappte Ständer auf seinen vier Gummikappen; gespielt gehen die 172 N des Pianos und bis zu 30 N der Hände über die schaumgepolsterten Arme in die zwei 25-mm-Rohre (höchstens 0,42 kN Druck je Rohr, 1 % ihrer Knicklast) und die vier Kappen; getragen hält eine Hand die 210 N des zugezippten Case am Griff, einem Gelenklager. Jedes Teil ist ein starrer Körper; das Asset wird wie jedes Asset des Produkts als ein starrer Körper auf den Auflagern seiner Pose gelöst."),
            T("Masses from section times density where the part is a section (stand: 25 x 25 x 1.5 tube 1.11 kg/m, 25 x 1.5 crossbars 0.87 kg/m, so 1.74 kg per leg), catalogue-typical masses for the piano (17.5 kg), the case (3.7 kg), the pedal (0.5 kg) and the adapter (0.35 kg).", "Massen aus Querschnitt mal Dichte, wo das Teil ein Profil ist (Ständer: Rohr 25 x 25 x 1,5 mit 1,11 kg/m, Querstreben 25 x 1,5 mit 0,87 kg/m, also 1,74 kg je Bein), katalogübliche Massen für Piano (17,5 kg), Case (3,7 kg), Pedal (0,5 kg) und Netzteil (0,35 kg)."),
        ], "clinical": False},
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    print("[DEBUG] written", HERE / "🔣️.json", MASS_TOTAL)
