"""🗄️ Authoritative author script of the used solid-pine chest of drawers: every hand-chosen number of the asset lives here and the script writes `🔣️.json` next to it.

Run: `python3 author-chest.py` (prints the mass, centre-of-mass and stability table of the report).

See also ../../brief.md and ../../reports/used-chest-of-drawers.md.
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def T(en, de):
    """🔤 One bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """🎯 Rounds a number to the micrometre for the document."""
    return round(v, 6)


def pts(seq):
    """📍 Rounds a list of 2D points."""
    return [[r6(a), r6(b)] for a, b in seq]


BLANKET_T = 0.016
BLANKET_X0, BLANKET_X1, BLANKET_Z = -0.05, 0.45, 0.45
DEPTH, WIDTH_TOP, HEIGHT = 0.40, 0.80, 0.85
SIDE_X0, SIDE_X1, SIDE_ZO, SIDE_ZI = 0.003, 0.39, 0.39, 0.372
TOP_Y0, TOP_Y1 = 0.828, 0.850
BOTTOM_Y0, BOTTOM_Y1 = 0.070, 0.088
PITCH, OPENING, RAIL = 0.190, 0.170, 0.020
LEVELS = [BOTTOM_Y1 + k * PITCH for k in range(4)]
FRONT_X0, FRONT_X1, FRONT_Z = 0.372, 0.39, 0.370
BOX_X0, BOX_H, BOX_T, BOX_ZI = 0.032, 0.130, 0.012, 0.358
KNOB_Z, KNOB_V = 0.20, 0.085
ARCH_X0, ARCH_X1, ARCH_RISE = 0.07, 0.32, 0.045
APRON_H, APRON_ARCH_W, APRON_RISE = 0.070, 0.30, 0.035
PINE, HDF, PORCELAIN, BLANKET_RHO = 510.0, 900.0, 2400.0, 225.0
G = 9.80665
CLOTHES_KG = 4.0
PULL_USE = 0.25
LIFT, CARRY_BACK, TILT = 0.25, -0.45, 3.0
STACK_PULL = [0.44, 0.42, 0.40, 0.38]
STACK_BOTTOM = [0.0, 0.130, 0.260, 0.390]

KNOB = [(0.0, 0.010), (0.004, 0.0095), (0.008, 0.007), (0.012, 0.0075), (0.016, 0.012), (0.021, 0.016), (0.025, 0.0145), (0.028, 0.009), (0.029, 0.0)]
KNOB_NEW = [(0.0, 0.008), (0.006, 0.007), (0.010, 0.009), (0.015, 0.0135), (0.019, 0.0145), (0.023, 0.012), (0.025, 0.006), (0.0255, 0.0)]
KNOB_100 = [(0.0, 0.010), (0.010, 0.008), (0.020, 0.016), (0.029, 0.0)]
KNOB_NEW_100 = [(0.0, 0.008), (0.012, 0.009), (0.019, 0.0145), (0.0255, 0.0)]


def arc(x0, x1, rise, n):
    """🌈 Points of a circular segment from (x0, 0) over the apex `rise` to (x1, 0), apex included."""
    c = x1 - x0
    radius = (c * c / 4 + rise * rise) / (2 * rise)
    cx, cy = (x0 + x1) / 2, rise - radius
    a0 = math.atan2(0 - cy, x0 - cx)
    a1 = math.atan2(0 - cy, x1 - cx)
    return [(cx + radius * math.cos(a0 + (a1 - a0) * i / (n - 1)), cy + radius * math.sin(a0 + (a1 - a0) * i / (n - 1))) for i in range(n)]


def side_profile(n):
    """🦶 The side panel in the (u, v) plane: back foot, arch between the bracket feet, front foot, up to the underside of the top."""
    if n == 0:
        bottom = [(SIDE_X0, 0.0), (SIDE_X1, 0.0)]
    elif n == 2:
        bottom = [(SIDE_X0, 0.0), (ARCH_X0, 0.0), (0.13, 0.04), (0.26, 0.04), (ARCH_X1, 0.0), (SIDE_X1, 0.0)]
    else:
        bottom = [(SIDE_X0, 0.0)] + arc(ARCH_X0, ARCH_X1, ARCH_RISE, n) + [(SIDE_X1, 0.0)]
    return bottom + [(SIDE_X1, TOP_Y0), (SIDE_X0, TOP_Y0)]


def apron_arch(n):
    """🌉 The front apron in the (lateral, height) plane with its shallow arch."""
    a = arc(-APRON_ARCH_W, APRON_ARCH_W, APRON_RISE, n)
    return [(-SIDE_ZI, 0.0)] + a + [(SIDE_ZI, 0.0), (SIDE_ZI, APRON_H), (-SIDE_ZI, APRON_H)]


def poly_area(p):
    """📐 Shoelace area of a simple polygon."""
    return abs(sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))) / 2


def revolve_volume(profile):
    """🌀 Disk-method volume of a profile [axial, radial] closed through the axis."""
    return sum(math.pi * (profile[i + 1][0] - profile[i][0]) * (profile[i][1] ** 2 + profile[i][1] * profile[i + 1][1] + profile[i + 1][1] ** 2) / 3 for i in range(len(profile) - 1))


MATERIALS = [
    {"id": "pine-waxed-aged", "title": T("Solid Scots pine glued panel, beeswax worn and darkened by about 20 years of use", "Kiefer-Leimholz massiv, Bienenwachs durch etwa 20 Jahre Gebrauch abgenutzt und nachgedunkelt"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": PINE, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000.0, "strengthPa": 24000000.0, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "pine-lacquered", "title": T("Solid Scots pine, clear lacquer yellowed with age", "Kiefer massiv, Klarlack altersbedingt vergilbt"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": PINE, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000.0, "strengthPa": 24000000.0, "source": "literature", "reference": "en-338-2016"},
    {"id": "hardboard", "title": T("Hardboard 3 mm (back panel, drawer bottoms)", "Hartfaserplatte 3 mm (Rückwand, Schubkastenböden)"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": HDF, "conductivityWmK": 0.18, "specificHeatJKgK": 1700, "elasticModulusPa": 3000000000.0, "strengthPa": 30000000.0, "source": "literature", "reference": "en-622-2-2004"},
    {"id": "porcelain", "title": T("White glazed porcelain (the replacement knob)", "Weißes glasiertes Porzellan (der Ersatzknopf)"), "emissivity": 0.92, "solarAbsorptivity": 0.2, "densityKgM3": PORCELAIN, "conductivityWmK": 1.5, "specificHeatJKgK": 1085, "source": "literature", "reference": "incropera-2018"},
    {"id": "moving-blanket", "title": T("Quilted removal blanket, cotton and polyester fleece, folded in eight", "Gesteppte Möbeldecke, Baumwolle und Polyestervlies, achtfach gefaltet"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": BLANKET_RHO, "conductivityWmK": 0.045, "specificHeatJKgK": 1350, "source": "authored-estimate", "reference": "removal-blanket"},
]
DENSITY = {m["id"]: m["densityKgM3"] for m in MATERIALS}

REFERENCES = [
    {"id": "en-14749-2016", "citation": "EN 14749:2016 Furniture - Domestic and kitchen storage units and worktops - Safety requirements and test methods (strength of extension elements, stability of free-standing storage with all extension elements open)."},
    {"id": "en-16122-2012", "citation": "EN 16122:2012 Domestic and non-domestic storage furniture - Test methods for the determination of strength, durability and stability (drawer loading, overturning tests)."},
    {"id": "astm-f2057-23", "citation": "ASTM F2057-23 Standard Safety Specification for Clothing Storage Units (tip-over stability with drawers open and a 50 lb / 22.7 kg child test mass on an open drawer)."},
    {"id": "din-18011-2021", "citation": "DIN 18011:2021-05 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau (placing areas and clearances of bedroom storage furniture, 0.9 m in front of a chest of drawers)."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (softwood C24: bending strength 24 MPa, mean modulus of elasticity parallel to grain 11 GPa)."},
    {"id": "wagenfuehr-holzatlas", "citation": "Wagenführ, R., Wagenführ, A. (2022) Holzatlas, 7th ed., Hanser, München: Scots pine (Pinus sylvestris) density 0.49-0.52 g/cm3 at 12-15 % moisture content."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (softwood 500 kg/m3: 0.13 W/mK, 1600 J/kgK; hardboard; textiles)."},
    {"id": "en-622-2-2004", "citation": "EN 622-2:2004 Fibreboards - Specifications - Part 2: Requirements for hardboards (HB, density at least 900 kg/m3, bending strength 30 MPa or more)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018) Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (Table A.11 emissivity of wood about 0.9, ceramics; Table A.3 porcelain 2400 kg/m3, 1.5 W/mK, 1085 J/kgK)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - General actions, densities, self-weight, imposed loads for buildings (gravity 9.80665 m/s2 and densities of stored materials, Annex A)."},
    {"id": "pine-chest-catalogue", "citation": "Catalogue-typical solid pine chests of drawers (Kiefer massiv, gewachst oder lackiert, 4 Schubkästen) of German furniture discounters 2000-2015 and the flat-pack solid pine equivalents: 80 x 40 x 85 cm, 18-22 mm glued pine panels, inset drawers on wooden runners with 2 mm gaps, turned pine knobs about 30 mm, 28-35 kg; taken as the type, no brand."},
    {"id": "removal-blanket", "citation": "Catalogue-typical removal blanket (Möbeldecke, Umzugsdecke) 180 x 200 cm, quilted cotton/polyester non-woven, 1.4-2.0 kg, as used by removal firms and municipal reuse depots."},
    {"id": "semio-used-household", "citation": "Semio staffage content, used household objects collection (2026-11-22): traces of use, storage state and carrying of a reuse-depot object are authored estimates of the typical case; nothing is measured."},
]

carcass_parts = []
drawer_parts = []


def solid_box(sid, bone, mat, tone, x0, x1, y0, y1, z0, z1, role="body", **kw):
    """📦 A box solid from its bone-local extents."""
    s = {"id": sid, "kind": "box", "bone": bone, "material": mat, "role": role, "tone": tone, "size": [r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], "transform": {"translation": [r6((x0 + x1) / 2), r6((y0 + y1) / 2), r6((z0 + z1) / 2)]}}
    s.update(kw)
    s["_vol"] = (x1 - x0) * (y1 - y0) * (z1 - z0)
    return s


def solid_extrude(sid, bone, mat, tone, profile, plane, f, t, holes=None, role="body", **kw):
    """🧱 An extrude solid; the volume is profile area minus holes times the length."""
    s = {"id": sid, "kind": "extrude", "bone": bone, "material": mat, "role": role, "tone": tone, "profile": pts(profile), "plane": plane, "from": r6(f), "to": r6(t)}
    area = poly_area(profile)
    if holes:
        s["holes"] = [pts(h) for h in holes]
        area -= sum(poly_area(h) for h in holes)
    s.update(kw)
    s["_vol"] = area * (t - f)
    return s


def solid_knob(sid, bone, mat, tone, profile, z):
    """🍄 A turned knob revolved about the forward axis from the front face of its drawer."""
    return {"id": sid, "kind": "revolve", "bone": bone, "material": mat, "role": "detail", "tone": tone, "profile": pts(profile), "axis": [1, 0, 0], "origin": [FRONT_X1, KNOB_V, z], "_vol": revolve_volume(profile)}


def clean(lst):
    """🧽 Strips the private volume helper key."""
    return [{k: v for k, v in s.items() if not k.startswith("_")} for s in lst]


def drawer_bone(k):
    """🗃️ The bone id of drawer k (1 bottom … 4 top)."""
    return f"drawer-{k}"


def body_50():
    """🔍 Body 1:50: every board, rail, runner, corner block, drawer part and knob as built."""
    c = "carcass"
    out = [
        solid_box("blanket", "blanket", "moving-blanket", 0.45, 0.0, 0.5, 0.0, BLANKET_T, -BLANKET_Z, BLANKET_Z, cornerRadius=0.006),
        solid_extrude("side-r", c, "pine-lacquered", 0.62, side_profile(9), "xy", -SIDE_ZO, -SIDE_ZI),
        solid_extrude("side-l", c, "pine-lacquered", 0.62, side_profile(9), "xy", SIDE_ZI, SIDE_ZO),
        solid_extrude("top", c, "pine-waxed-aged", 0.5, [(-0.40, 0.0), (0.40, 0.0), (0.40, 0.388), (0.393, 0.391), (0.386, 0.395), (0.383, 0.40), (-0.40, 0.40)], "zx", TOP_Y0, TOP_Y1),
        solid_box("bottom", c, "pine-lacquered", 0.6, SIDE_X0, SIDE_X1, BOTTOM_Y0, BOTTOM_Y1, -SIDE_ZI, SIDE_ZI),
        solid_extrude("apron", c, "pine-lacquered", 0.6, [(v, w) for w, v in apron_arch(11)], "yz", FRONT_X0, SIDE_X1),
        solid_box("apron-back", c, "pine-lacquered", 0.6, SIDE_X0, 0.021, 0.0, APRON_H, -SIDE_ZI, SIDE_ZI),
        solid_box("back-panel", c, "hardboard", 0.85, 0.0, SIDE_X0, BOTTOM_Y0, TOP_Y0, -SIDE_ZO, SIDE_ZO, sheet=True),
    ]
    for k in (2, 3, 4):
        y = LEVELS[k - 1]
        out.append(solid_box(f"rail-{k}", c, "pine-lacquered", 0.6, 0.34, SIDE_X1, y - RAIL, y, -SIDE_ZI, SIDE_ZI))
        out.append(solid_box(f"runner-{k}-r", c, "pine-lacquered", 0.6, 0.021, 0.34, y - RAIL, y, -SIDE_ZI, -0.347))
        out.append(solid_box(f"runner-{k}-l", c, "pine-lacquered", 0.6, 0.021, 0.34, y - RAIL, y, 0.347, SIDE_ZI))
        out.append(solid_box(f"back-rail-{k}", c, "pine-lacquered", 0.6, SIDE_X0, 0.021, y - RAIL, y, -SIDE_ZI, SIDE_ZI))
    for name, x0 in (("back", 0.03), ("front", 0.30)):
        for side, sgn in (("r", -1), ("l", 1)):
            z0, z1 = sorted((sgn * 0.352, sgn * SIDE_ZI))
            out.append(solid_box(f"corner-block-{name}-{side}", c, "pine-lacquered", 0.6, x0, x0 + 0.06, TOP_Y0 - 0.02, TOP_Y0, z0, z1, role="detail"))
    for k in range(1, 5):
        b = drawer_bone(k)
        out += [
            solid_box(f"{b}-front", b, "pine-lacquered", 0.58, FRONT_X0, FRONT_X1, 0.002, 0.168, -FRONT_Z, FRONT_Z),
            solid_box(f"{b}-side-r", b, "pine-lacquered", 0.75, BOX_X0, FRONT_X0, 0.0, BOX_H, -FRONT_Z, -BOX_ZI),
            solid_box(f"{b}-side-l", b, "pine-lacquered", 0.75, BOX_X0, FRONT_X0, 0.0, BOX_H, BOX_ZI, FRONT_Z),
            solid_box(f"{b}-back", b, "pine-lacquered", 0.75, BOX_X0, 0.044, 0.013, BOX_H, -BOX_ZI, BOX_ZI),
            solid_box(f"{b}-bottom", b, "hardboard", 0.85, 0.044, FRONT_X0, 0.010, 0.013, -BOX_ZI, BOX_ZI, sheet=True),
            solid_knob(f"{b}-knob-l", b, "pine-lacquered", 0.42, KNOB, KNOB_Z),
            solid_knob(f"{b}-knob-r", b, "porcelain" if k == 3 else "pine-lacquered", 0.95 if k == 3 else 0.42, KNOB_NEW if k == 3 else KNOB, -KNOB_Z),
        ]
    return out


def body_100():
    """🔎 Body 1:100: true boards and the arches kept, each drawer frame one ring, each drawer box one ring, knobs simplified, no chip."""
    c = "carcass"
    out = [
        solid_box("blanket", "blanket", "moving-blanket", 0.45, 0.0, 0.5, 0.0, BLANKET_T, -BLANKET_Z, BLANKET_Z),
        solid_extrude("side-r", c, "pine-lacquered", 0.62, side_profile(5), "xy", -SIDE_ZO, -SIDE_ZI),
        solid_extrude("side-l", c, "pine-lacquered", 0.62, side_profile(5), "xy", SIDE_ZI, SIDE_ZO),
        solid_box("top", c, "pine-waxed-aged", 0.5, 0.0, DEPTH, TOP_Y0, TOP_Y1, -0.40, 0.40),
        solid_box("bottom", c, "pine-lacquered", 0.6, SIDE_X0, SIDE_X1, BOTTOM_Y0, BOTTOM_Y1, -SIDE_ZI, SIDE_ZI),
        solid_extrude("apron", c, "pine-lacquered", 0.6, [(v, w) for w, v in apron_arch(5)], "yz", FRONT_X0, SIDE_X1),
        solid_box("apron-back", c, "pine-lacquered", 0.6, SIDE_X0, 0.021, 0.0, APRON_H, -SIDE_ZI, SIDE_ZI),
        solid_box("back-panel", c, "hardboard", 0.85, 0.0, SIDE_X0, BOTTOM_Y0, TOP_Y0, -SIDE_ZO, SIDE_ZO, sheet=True),
    ]
    for k in (2, 3, 4):
        y = LEVELS[k - 1]
        out.append(solid_extrude(f"frame-{k}", c, "pine-lacquered", 0.6, [(-SIDE_ZI, SIDE_X0), (SIDE_ZI, SIDE_X0), (SIDE_ZI, SIDE_X1), (-SIDE_ZI, SIDE_X1)], "zx", y - RAIL, y, holes=[[(-0.347, 0.021), (0.347, 0.021), (0.347, 0.34), (-0.347, 0.34)]]))
    for name, x0 in (("back", 0.03), ("front", 0.30)):
        for side, sgn in (("r", -1), ("l", 1)):
            z0, z1 = sorted((sgn * 0.352, sgn * SIDE_ZI))
            out.append(solid_box(f"corner-block-{name}-{side}", c, "pine-lacquered", 0.6, x0, x0 + 0.06, TOP_Y0 - 0.02, TOP_Y0, z0, z1, role="detail"))
    for k in range(1, 5):
        b = drawer_bone(k)
        out += [
            solid_box(f"{b}-front", b, "pine-lacquered", 0.58, FRONT_X0, FRONT_X1, 0.002, 0.168, -FRONT_Z, FRONT_Z),
            solid_extrude(f"{b}-box", b, "pine-lacquered", 0.75, [(-FRONT_Z, BOX_X0), (FRONT_Z, BOX_X0), (FRONT_Z, FRONT_X0), (-FRONT_Z, FRONT_X0)], "zx", 0.0, BOX_H, holes=[[(-BOX_ZI, 0.044), (BOX_ZI, 0.044), (BOX_ZI, 0.36), (-BOX_ZI, 0.36)]]),
            solid_box(f"{b}-bottom", b, "hardboard", 0.85, 0.044, 0.36, 0.010, 0.013, -BOX_ZI, BOX_ZI, sheet=True),
            solid_knob(f"{b}-knob-l", b, "pine-lacquered", 0.42, KNOB_100, KNOB_Z),
            solid_knob(f"{b}-knob-r", b, "porcelain" if k == 3 else "pine-lacquered", 0.95 if k == 3 else 0.42, KNOB_NEW_100 if k == 3 else KNOB_100, -KNOB_Z),
        ]
    return out


def body_200():
    """🗺️ Body 1:200: sides with a three-segment arch, top, plinth block, three drawer frames, the back sheet and four open drawer trays with their bottoms; knobs left out."""
    c = "carcass"
    out = [
        solid_box("blanket", "blanket", "moving-blanket", 0.45, 0.0, 0.5, 0.0, BLANKET_T, -BLANKET_Z, BLANKET_Z, sheet=True),
        solid_extrude("side-r", c, "pine-lacquered", 0.62, side_profile(2), "xy", -SIDE_ZO, -SIDE_ZI),
        solid_extrude("side-l", c, "pine-lacquered", 0.62, side_profile(2), "xy", SIDE_ZI, SIDE_ZO),
        solid_box("top", c, "pine-waxed-aged", 0.5, 0.0, DEPTH, TOP_Y0, TOP_Y1, -0.40, 0.40),
        solid_box("plinth", c, "pine-lacquered", 0.6, SIDE_X0, SIDE_X1, 0.0, BOTTOM_Y1, -SIDE_ZI, SIDE_ZI),
        solid_box("back-panel", c, "hardboard", 0.85, 0.0, SIDE_X0, BOTTOM_Y0, TOP_Y0, -SIDE_ZO, SIDE_ZO, sheet=True),
    ]
    for k in (2, 3, 4):
        y = LEVELS[k - 1]
        out.append(solid_extrude(f"frame-{k}", c, "pine-lacquered", 0.6, [(-SIDE_ZI, SIDE_X0), (SIDE_ZI, SIDE_X0), (SIDE_ZI, SIDE_X1), (-SIDE_ZI, SIDE_X1)], "zx", y - RAIL, y, holes=[[(-0.347, 0.021), (0.347, 0.021), (0.347, 0.34), (-0.347, 0.34)]]))
    for k in range(1, 5):
        b = drawer_bone(k)
        out += [
            solid_extrude(f"{b}-tray", b, "pine-lacquered", 0.6, [(-FRONT_Z, BOX_X0), (FRONT_Z, BOX_X0), (FRONT_Z, FRONT_X1), (-FRONT_Z, FRONT_X1)], "zx", 0.0, 0.168, holes=[[(-0.34, 0.062), (0.34, 0.062), (0.34, 0.36), (-0.34, 0.36)]]),
            solid_box(f"{b}-bottom", b, "hardboard", 0.85, 0.062, 0.36, 0.010, 0.013, -0.34, 0.34, sheet=True),
        ]
    return out


def body_500():
    """🌍 Body 1:500: the carcass as one U of sides and plinth, the top slab, the back sheet, four drawer blocks that take the rails in, the blanket sheet."""
    c = "carcass"
    u = [(0.0, -SIDE_ZO), (TOP_Y0, -SIDE_ZO), (TOP_Y0, -SIDE_ZI), (BOTTOM_Y1, -SIDE_ZI), (BOTTOM_Y1, SIDE_ZI), (TOP_Y0, SIDE_ZI), (TOP_Y0, SIDE_ZO), (0.0, SIDE_ZO)]
    out = [
        solid_box("blanket", "blanket", "moving-blanket", 0.45, 0.0, 0.5, 0.0, BLANKET_T, -BLANKET_Z, BLANKET_Z, sheet=True),
        solid_extrude("carcass-shell", c, "pine-lacquered", 0.62, u, "yz", SIDE_X0, SIDE_X1),
        solid_box("top", c, "pine-waxed-aged", 0.5, 0.0, DEPTH, TOP_Y0, TOP_Y1, -0.40, 0.40),
        solid_box("back-panel", c, "hardboard", 0.85, 0.0, SIDE_X0, BOTTOM_Y0, TOP_Y0, -SIDE_ZO, SIDE_ZO, sheet=True),
    ]
    for k in range(1, 5):
        out.append(solid_box(f"{drawer_bone(k)}-block", drawer_bone(k), "pine-lacquered", 0.6, BOX_X0, FRONT_X1, 0.0, PITCH if k < 4 else OPENING, -FRONT_Z, FRONT_Z))
    return out


BODY = {"50": body_50(), "100": body_100(), "200": body_200(), "500": body_500()}


def masses():
    """⚖️ Bone masses and centres of mass from the 1:50 solids (section x density)."""
    out = {}
    for s in BODY["50"]:
        m = s["_vol"] * DENSITY[s["material"]]
        if s["kind"] == "box":
            cx, cy = s["transform"]["translation"][0], s["transform"]["translation"][1]
        elif s["kind"] == "revolve":
            cx, cy = FRONT_X1 + 0.015, KNOB_V
        elif s["id"].startswith("side"):
            cx, cy = 0.1965, 0.43
        elif s["id"] == "top":
            cx, cy = 0.2, 0.839
        else:
            cx, cy = 0.381, 0.04
        e = out.setdefault(s["bone"], [0.0, 0.0, 0.0])
        e[0] += m
        e[1] += m * cx
        e[2] += m * cy
    return {b: (v[0], v[1] / v[0], v[2] / v[0]) for b, v in out.items()}


MASS = masses()
TOTAL = sum(v[0] for v in MASS.values())

BONES = [
    {"id": "floor", "parent": None, "title": T("Floor of the depot hall", "Boden der Lagerhalle"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "blanket", "parent": "floor", "title": T("Folded removal blanket (stays on the floor)", "Gefaltete Möbeldecke (bleibt auf dem Boden)"), "x": BLANKET_X0, "y": 0, "angleDeg": 0, "lengthM": 0.5, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.6}, "mass": {"kg": r6(MASS["blanket"][0]), "comX": 0.25, "comY": 0.008, "distribution": "uniform", "source": "authored-estimate", "reference": "removal-blanket"}},
    {"id": "chest", "parent": "floor", "title": T("Standing level of the chest (on the blanket or on the floor)", "Standhöhe der Kommode (auf der Decke oder auf dem Boden)"), "x": 0, "y": BLANKET_T, "angleDeg": 0, "lengthM": 0.03, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -BLANKET_T, "maxM": 0.4}},
    {"id": "carcass-carry", "parent": "chest", "title": T("Carrying travel of the empty carcass", "Tragweg des leeren Korpus"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.03, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.6, "maxM": 0}},
    {"id": "carcass", "parent": "carcass-carry", "title": T("Carcass: sides, top, bottom, aprons, frames, back panel", "Korpus: Seiten, Deckplatte, Boden, Zargen, Rahmen, Rückwand"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": DEPTH, "joint": {"type": "revolute", "minDeg": -10, "maxDeg": 10}, "mass": {"kg": r6(MASS["carcass"][0]), "comX": r6(MASS["carcass"][1]), "comY": r6(MASS["carcass"][2]), "distribution": "point", "source": "authored-estimate", "reference": "wagenfuehr-holzatlas"}},
]
for k in range(1, 5):
    b = drawer_bone(k)
    BONES.append({"id": f"{b}-level", "parent": "chest", "title": T(f"Height of drawer {k} (lowered when it is taken out)", f"Höhe von Schubkasten {k} (abgesenkt, wenn er herausgenommen ist)"), "x": 0, "y": r6(LEVELS[k - 1]), "angleDeg": 0, "lengthM": 0.03, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.6, "maxM": 0}})
    BONES.append({"id": b, "parent": f"{b}-level", "title": T(f"Drawer {k} ({['bottom', 'second', 'third', 'top'][k - 1]}) on its runners", f"Schubkasten {k} ({['unten', 'zweiter', 'dritter', 'oben'][k - 1]}) auf seinen Laufleisten"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": FRONT_X1, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.45}, "mass": {"kg": r6(MASS[b][0]), "comX": r6(MASS[b][1]), "comY": r6(MASS[b][2]), "distribution": "point", "source": "authored-estimate", "reference": "wagenfuehr-holzatlas"}, "section": {"areaM2": r6(2 * BOX_T * BOX_H), "secondMomentM4": 2 * BOX_T * BOX_H ** 3 / 12, "extremeFibreM": r6(BOX_H / 2), "material": "pine-lacquered", "title": T("Two drawer sides 12 x 130 mm", "Zwei Schubkastenseiten 12 x 130 mm")}})

ANCHORS = [
    {"id": "blanket-floor", "bone": "blanket", "x": 0.25, "y": 0.0, "kind": "contact", "title": T("Folded blanket on the floor", "Gefaltete Decke auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.5, "footprintWidthM": 0.9, "frictionCoefficient": 0.6},
]
for side, sgn in (("r", -1), ("l", 1)):
    name = T("right", "rechts") if sgn < 0 else T("left", "links")
    ANCHORS.append({"id": f"foot-back-{side}", "bone": "carcass", "x": 0.0365, "y": 0.0, "z": r6(sgn * 0.381), "kind": "contact", "title": T(f"Back bracket foot, {name['en']}", f"Hinterer Sockelfuß, {name['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.067, "footprintWidthM": 0.018, "frictionCoefficient": 0.4})
    ANCHORS.append({"id": f"foot-front-{side}", "bone": "carcass", "x": 0.355, "y": 0.0, "z": r6(sgn * 0.381), "kind": "contact", "title": T(f"Front bracket foot, {name['en']}", f"Vorderer Sockelfuß, {name['de']}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.07, "footprintWidthM": 0.018, "frictionCoefficient": 0.4})
for side, sgn in (("r", -1), ("l", 1)):
    name = T("right", "rechts") if sgn < 0 else T("left", "links")
    ANCHORS.append({"id": f"grip-{side}-low", "bone": "carcass", "x": 0.195, "y": ARCH_RISE, "z": r6(sgn * 0.381), "kind": "support", "title": T(f"Carrier {name['en']}: hand under the side arch", f"Träger {name['de']}: Hand unter dem Seitenbogen"), "support": "pin"})
    ANCHORS.append({"id": f"grip-{side}-top", "bone": "carcass", "x": 0.195, "y": TOP_Y0, "z": r6(sgn * 0.395), "kind": "support", "title": T(f"Carrier {name['en']}: hand under the top overhang", f"Träger {name['de']}: Hand unter dem Deckplattenüberstand"), "support": "pin"})
ANCHORS.append({"id": "drawer-stack-floor", "bone": "drawer-1", "x": 0.211, "y": 0.0, "kind": "contact", "title": T("Bottom drawer of the stack on the floor", "Unterster Schubkasten des Stapels auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.34, "footprintWidthM": 0.74, "frictionCoefficient": 0.4})
FEET = ["foot-back-r", "foot-front-r", "foot-back-l", "foot-front-l"]
GRIPS = ["grip-r-low", "grip-r-top", "grip-l-low", "grip-l-top"]


def sh(sid, bone, kind, role, z, tone, filled=True, region=None, regions=None, material=None, pen=None, **geo):
    """✏️ One 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    s.update(geo)
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if pen is not None:
        s["penMm"] = pen
    return s


def rect(sid, bone, x0, y0, x1, y1, role="body", z=5, tone=0.6, **kw):
    """⬛ A rectangle shape from two corners."""
    return sh(sid, bone, "rect", role, z, tone, x=r6(min(x0, x1)), y=r6(min(y0, y1)), w=r6(abs(x1 - x0)), h=r6(abs(y1 - y0)), **kw)


def poly(sid, bone, points, role="body", z=5, tone=0.6, closed=True, filled=True, **kw):
    """🔷 A polygon or polyline shape."""
    return sh(sid, bone, "poly", role, z, tone, filled=filled, points=pts(points), closed=closed, **kw)


def ellipse(sid, bone, cx, cy, r, role="detail", z=7, tone=0.4, **kw):
    """⭕ A circle shape."""
    return sh(sid, bone, "ellipse", role, z, tone, cx=r6(cx), cy=r6(cy), rx=r6(r), ry=r6(r), **kw)


def tex(sid, bone, points, z=6.5, closed=False):
    """〰️ A texture stroke (traces of use, grain, folds)."""
    return poly(sid, bone, points, role="texture", z=z, tone=0.3, closed=closed, filled=False)


def knob_outline(profile, cx, cy):
    """🍄 The silhouette of a turned knob seen from the side or from above."""
    up = [(cx + a, cy + r) for a, r in profile]
    down = [(cx + a, cy - r) for a, r in reversed(profile) if r > 0]
    return up + down


W = BLANKET_T


def elevation(scale):
    """🖼️ Side elevation from the right: blanket, right side panel with its bracket feet, top, back edge, knobs; drawers drawn behind the side so they appear when pulled."""
    n = {"50": 9, "100": 5, "200": 2, "500": 0}[scale]
    out = [rect("blanket", "blanket", 0, 0, 0.5, BLANKET_T, z=1, tone=0.45, region="blanket", material="moving-blanket", **({"r": 0.006} if scale in ("50", "100") else {}))]
    if scale in ("50", "100"):
        out.append(rect("back-panel", "carcass", 0, BOTTOM_Y0, SIDE_X0, TOP_Y0, z=2, tone=0.85, region="back-panel", material="hardboard"))
        out.append(poly("side", "carcass", side_profile(n), z=5, tone=0.62, region="carcass", material="pine-lacquered"))
    elif scale == "200":
        out.append(rect("plinth", "carcass", SIDE_X0, 0, SIDE_X1, BOTTOM_Y1, z=2, tone=0.4, region="carcass", material="pine-lacquered"))
        out.append(poly("side", "carcass", [(0.0, BOTTOM_Y0), (SIDE_X0, BOTTOM_Y0)] + side_profile(n)[:-1] + [(0.0, TOP_Y0)], z=5, tone=0.62, regions=["carcass", "back-panel"], material="pine-lacquered"))
    else:
        out.append(rect("side", "carcass", 0, 0, SIDE_X1, TOP_Y0, z=5, tone=0.62, regions=["carcass", "back-panel"], material="pine-lacquered"))
    if scale == "50":
        out.append(poly("top", "carcass", [(0, TOP_Y0), (DEPTH, TOP_Y0), (DEPTH, 0.844), (0.398, 0.8478), (0.394, TOP_Y1), (0, TOP_Y1)], z=6, tone=0.5, region="top", material="pine-waxed-aged"))
    else:
        out.append(rect("top", "carcass", 0, TOP_Y0, DEPTH, TOP_Y1, z=6, tone=0.5, region="top", material="pine-waxed-aged"))
    if scale == "50":
        for i, x in enumerate((0.10, 0.17, 0.25, 0.32)):
            out.append(tex(f"side-lamella-{i + 1}", "carcass", [(x, 0.06), (x, 0.81)], z=5.5))
        out.append(tex("side-foot-scuff", "carcass", [(0.362, 0.012), (0.375, 0.03), (0.37, 0.052)], z=5.5))
        out.append(tex("side-water-ring", "carcass", [(0.21, 0.74), (0.24, 0.752), (0.27, 0.745), (0.285, 0.728)], z=5.5))
    if scale == "100":
        out.append(tex("side-lamella-1", "carcass", [(0.17, 0.06), (0.17, 0.81)], z=5.5))
    for k in range(1, 5):
        b = drawer_bone(k)
        if scale in ("50", "100"):
            out.append(rect(f"{b}-box", b, BOX_X0, 0, FRONT_X0, BOX_H, z=3, tone=0.75, material="pine-lacquered"))
            out.append(rect(f"{b}-front", b, FRONT_X0, 0.002, FRONT_X1, 0.168, z=3.5, tone=0.58, region="drawer-fronts", material="pine-lacquered"))
            prof = KNOB if scale == "50" else KNOB_100
            out.append(poly(f"{b}-knob-l", b, knob_outline(prof, FRONT_X1, KNOB_V), role="detail", z=7, tone=0.42, region="drawer-fronts", material="pine-lacquered"))
            if k != 3:
                out.append(poly(f"{b}-knob-r", b, knob_outline(prof, FRONT_X1, KNOB_V), role="detail", z=7.5, tone=0.42, region="drawer-fronts", material="pine-lacquered"))
            if k == 3:
                out.append(poly(f"{b}-knob-new", b, knob_outline(KNOB_NEW if scale == "50" else KNOB_NEW_100, FRONT_X1, KNOB_V), role="detail", z=7.5, tone=0.95, region="drawer-fronts", material="porcelain"))
        elif scale == "200":
            out.append(rect(f"{b}-tray", b, BOX_X0, 0, FRONT_X1, 0.168, z=3, tone=0.6, region="drawer-fronts", material="pine-lacquered"))
        else:
            out.append(rect(f"{b}-block", b, BOX_X0, 0, FRONT_X1, PITCH if k < 4 else OPENING, z=3, tone=0.6, region="drawer-fronts", material="pine-lacquered"))
    return out


def front(scale):
    """🪟 Front elevation from ahead (paper x = lateral, the asset's left on the right): fronts, rails, side edges, apron, top edge, knobs, stickers."""
    y0 = W
    out = [rect("blanket", "blanket", -BLANKET_Z, 0, BLANKET_Z, BLANKET_T, z=1, tone=0.45, region="blanket", material="moving-blanket")]
    out.append(rect("back-panel", "carcass", -SIDE_ZO, y0 + BOTTOM_Y0, SIDE_ZO, y0 + TOP_Y0, z=1.5, tone=0.25, region="back-panel", material="hardboard"))
    if scale == "500":
        u = [(-SIDE_ZO, y0), (SIDE_ZO, y0), (SIDE_ZO, y0 + TOP_Y0), (SIDE_ZI, y0 + TOP_Y0), (SIDE_ZI, y0 + BOTTOM_Y1), (-SIDE_ZI, y0 + BOTTOM_Y1), (-SIDE_ZI, y0 + TOP_Y0), (-SIDE_ZO, y0 + TOP_Y0)]
        out.append(poly("carcass-shell", "carcass", u, z=4, tone=0.62, region="carcass", material="pine-lacquered"))
    else:
        out.append(rect("side-r-edge", "carcass", -SIDE_ZO, y0, -SIDE_ZI, y0 + TOP_Y0, z=4, tone=0.62, region="carcass", material="pine-lacquered"))
        out.append(rect("side-l-edge", "carcass", SIDE_ZI, y0, SIDE_ZO, y0 + TOP_Y0, z=4.1, tone=0.62, region="carcass", material="pine-lacquered"))
        if scale == "200":
            out.append(rect("plinth", "carcass", -SIDE_ZI, y0, SIDE_ZI, y0 + BOTTOM_Y1, z=4.2, tone=0.55, region="carcass", material="pine-lacquered"))
        else:
            out.append(rect("plinth-shadow", "carcass", -SIDE_ZI, y0, SIDE_ZI, y0 + APRON_H, z=2, tone=0.2, region="carcass", material="pine-lacquered"))
            out.append(poly("apron", "carcass", [(w, y0 + v) for w, v in apron_arch(11 if scale == "50" else 5)], z=4.2, tone=0.55, region="carcass", material="pine-lacquered"))
            out.append(rect("bottom-edge", "carcass", -SIDE_ZI, y0 + BOTTOM_Y0, SIDE_ZI, y0 + BOTTOM_Y1, z=4.3, tone=0.6, region="carcass", material="pine-lacquered"))
        for k in (2, 3, 4):
            y = LEVELS[k - 1]
            out.append(rect(f"rail-{k}", "carcass", -SIDE_ZI, y0 + y - RAIL, SIDE_ZI, y0 + y, z=4.3 + k / 10, tone=0.6, region="carcass", material="pine-lacquered"))
    out.append(rect("top", "carcass", -0.40, y0 + TOP_Y0, 0.40, y0 + TOP_Y1, z=6, tone=0.5, region="top", material="pine-waxed-aged"))
    for k in range(1, 5):
        b = drawer_bone(k)
        yb = y0 + LEVELS[k - 1]
        if scale == "500":
            out.append(rect(f"{b}-block", b, -FRONT_Z, yb, FRONT_Z, yb + (PITCH if k < 4 else OPENING), z=5, tone=0.58, region="drawer-fronts", material="pine-lacquered"))
            continue
        out.append(rect(f"{b}-front", b, -FRONT_Z, yb + 0.002, FRONT_Z, yb + 0.168, z=5, tone=0.58, region="drawer-fronts", material="pine-lacquered"))
        if scale in ("50", "100"):
            out.append(ellipse(f"{b}-knob-l", b, KNOB_Z, yb + KNOB_V, 0.016, z=7, tone=0.42, material="pine-lacquered"))
            if k == 3:
                out.append(ellipse(f"{b}-knob-new", b, -KNOB_Z, yb + KNOB_V, 0.0145, z=7, tone=0.95, material="porcelain"))
            else:
                out.append(ellipse(f"{b}-knob-r", b, -KNOB_Z, yb + KNOB_V, 0.016, z=7, tone=0.42, material="pine-lacquered"))
        if scale == "50":
            for side in (-1, 1):
                kx = side * KNOB_Z
                out.append(tex(f"{b}-scratch-{'l' if side > 0 else 'r'}", b, [(kx - 0.03, yb + 0.05), (kx - 0.01, yb + 0.046), (kx + 0.02, yb + 0.049)], z=6))
            out.append(tex(f"{b}-scuff", b, [(-0.30, yb + 0.006), (-0.12, yb + 0.008), (0.05, yb + 0.006), (0.26, yb + 0.007)], z=6))
            if k == 3:
                out.append(ellipse(f"{b}-old-screw-hole", b, -KNOB_Z, yb + KNOB_V - 0.024, 0.0025, z=6.5, tone=0.1))
            if k == 1:
                out.append(ellipse(f"{b}-sticker", b, 0.06, yb + 0.12, 0.02, z=6.5, tone=0.85))
            if k == 2:
                out.append(rect(f"{b}-price-label", b, -0.33, yb + 0.12, -0.27, yb + 0.15, role="detail", z=6.5, tone=0.95))
        if scale == "100" and k == 2:
            out.append(rect(f"{b}-price-label", b, -0.33, yb + 0.12, -0.27, yb + 0.15, role="detail", z=6.5, tone=0.95))
    if scale == "50":
        out.append(tex("blanket-fold", "blanket", [(-0.44, 0.008), (0.44, 0.008)], z=1.5))
        out.append(tex("top-edge-wear", "carcass", [(-0.36, y0 + 0.846), (-0.1, y0 + 0.845), (0.2, y0 + 0.846), (0.36, y0 + 0.845)], z=6.5))
    return out


def plan(scale):
    """🗺️ Plan from above (paper x ahead, paper y = lateral): blanket margin, the top with its chip, ring marks and worn wax, the knobs in front of the top edge, the drawers below it."""
    out = [rect("blanket", "blanket", BLANKET_X0, -BLANKET_Z, BLANKET_X1, BLANKET_Z, z=1, tone=0.45, region="blanket", material="moving-blanket")]
    if scale == "50":
        out.append(poly("top", "carcass", [(0.0, -0.40), (0.0, 0.40), (0.383, 0.40), (0.386, 0.395), (0.393, 0.391), (0.40, 0.388), (0.40, -0.40)], z=6, tone=0.5, region="top", material="pine-waxed-aged"))
    else:
        out.append(rect("top", "carcass", 0, -0.40, DEPTH, 0.40, z=6, tone=0.5, region="top", material="pine-waxed-aged"))
    if scale in ("50", "100"):
        out.append(sh("ring-mark-1", "carcass", "ellipse", "texture", 6.5, 0.3, filled=False, cx=0.24, cy=0.16, rx=0.04, ry=0.04))
        out.append(sh("ring-mark-2", "carcass", "ellipse", "texture", 6.5, 0.3, filled=False, cx=0.21, cy=-0.21, rx=0.045, ry=0.045))
    if scale == "50":
        out.append(sh("ring-mark-3", "carcass", "ellipse", "texture", 6.5, 0.3, filled=False, cx=0.26, cy=-0.18, rx=0.04, ry=0.04))
        out.append(tex("worn-wax", "carcass", [(0.30, -0.12), (0.37, -0.05), (0.38, 0.08), (0.33, 0.15), (0.22, 0.12), (0.18, 0.0), (0.22, -0.10)], closed=True))
        for i, x in enumerate((0.08, 0.16, 0.24, 0.32)):
            out.append(tex(f"top-lamella-{i + 1}", "carcass", [(x, -0.39), (x, 0.39)], z=6.2))
        out.append(tex("scratch-1", "carcass", [(0.12, 0.25), (0.15, 0.31)], z=6.5))
        out.append(tex("scratch-2", "carcass", [(0.31, -0.33), (0.35, -0.28)], z=6.5))
    for k in range(1, 5):
        b = drawer_bone(k)
        if scale in ("50", "100"):
            out.append(rect(f"{b}-box", b, BOX_X0, -FRONT_Z, FRONT_X0, FRONT_Z, z=3, tone=0.75, material="pine-lacquered"))
            out.append(rect(f"{b}-inside", b, 0.044, -BOX_ZI, FRONT_X0 - 0.002, BOX_ZI, z=3.2, tone=0.88, material="hardboard"))
            out.append(rect(f"{b}-front", b, FRONT_X0, -FRONT_Z, FRONT_X1, FRONT_Z, z=3.3, tone=0.58, region="drawer-fronts", material="pine-lacquered"))
            prof = KNOB if scale == "50" else KNOB_100
            out.append(poly(f"{b}-knob-l", b, knob_outline(prof, FRONT_X1, KNOB_Z), role="detail", z=7, tone=0.42, region="drawer-fronts", material="pine-lacquered"))
            if k == 3:
                out.append(poly(f"{b}-knob-new", b, knob_outline(KNOB_NEW if scale == "50" else KNOB_NEW_100, FRONT_X1, -KNOB_Z), role="detail", z=7, tone=0.95, region="drawer-fronts", material="porcelain"))
            else:
                out.append(poly(f"{b}-knob-r", b, knob_outline(prof, FRONT_X1, -KNOB_Z), role="detail", z=7, tone=0.42, region="drawer-fronts", material="pine-lacquered"))
        elif scale == "200":
            out.append(rect(f"{b}-tray", b, BOX_X0, -FRONT_Z, FRONT_X1, FRONT_Z, z=3, tone=0.6, region="drawer-fronts", material="pine-lacquered"))
            out.append(rect(f"{b}-inside", b, 0.062, -0.34, 0.36, 0.34, z=3.2, tone=0.88, material="hardboard"))
        else:
            out.append(rect(f"{b}-block", b, BOX_X0, -FRONT_Z, FRONT_X1, FRONT_Z, z=3, tone=0.6, region="drawer-fronts", material="pine-lacquered"))
    return out


PENS = {
    "50": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5},
    "100": {"body": 0.25, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.35},
    "200": {"body": 0.25, "detail": 0.18, "texture": None, "glazing": None, "ground": 0.25},
    "500": {"body": 0.25, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
}
RANGES = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}

REP_DECISIONS = {
    "50": {
        "silhouette": T("Chest 8.0 x 17.3 mm in the side view on the 0.3 mm blanket: right side panel with its two bracket feet and the 45 mm arch between them, the 22 mm waxed top rounded by wear at its front edge, the knobs standing 29 mm proud of the fronts. Front: four inset fronts with 2 mm gaps between 18 mm sides and 20 mm rails, the apron arch. Plan: the top with its chipped front-left corner over the blanket margin.", "Kommode 8,0 x 17,3 mm in der Seitenansicht auf der 0,3 mm starken Decke: rechte Seite mit zwei Sockelfüßen und dem 45-mm-Bogen dazwischen, die 22 mm gewachste Deckplatte mit abgegriffener Vorderkante, die Knöpfe 29 mm vor den Fronten. Vorderansicht: vier eingelassene Fronten mit 2 mm Fugen zwischen 18-mm-Seiten und 20-mm-Traversen, der Zargenbogen. Grundriss: die Deckplatte mit abgeplatzter vorderer linker Ecke über dem Deckenrand."),
        "retained": T("True board thicknesses, the 190 mm drawer pitch, the inset fronts with their 2 mm gaps, the turned pine knobs and the white porcelain knob that replaced the lost right knob of drawer 3 with the old screw hole beside it, the 17 x 12 mm chip of the top, the worn wax, three cup rings, scratches under the knobs, a child's sticker and the depot price label.", "Wahre Plattenstärken, der Schubkastenabstand von 190 mm, die eingelassenen Fronten mit 2 mm Fugen, die gedrechselten Kiefernknöpfe und der weiße Porzellanknopf, der den verlorenen rechten Knopf von Schubkasten 3 ersetzt, mit dem alten Schraubloch daneben, die 17 x 12 mm große Abplatzung der Deckplatte, das abgenutzte Wachs, drei Tassenränder, Kratzer unter den Knöpfen, ein Kinderaufkleber und das Preisschild des Lagers."),
        "merged": T("Glued lamellas of each panel are one board, their joints drawn as texture; edge rounding of the side panels is merged into the board.", "Die verleimten Lamellen jeder Platte sind eine Platte, ihre Fugen als Textur gezeichnet; die Kantenrundung der Seiten geht in der Platte auf."),
        "omitted": T("Dowels, glue blocks inside the plinth, the grooves of the drawer bottoms and the nails of the back panel.", "Dübel, Leimklötze im Sockel, die Nuten der Schubkastenböden und die Nägel der Rückwand."),
        "thinElements": T("The 3 mm hardboard back (0.06 mm) and the 2 mm drawer gaps (0.04 mm) are drawn at true size and read through their outlines; the drawer bottoms are not seen from outside.", "Die 3-mm-Hartfaserrückwand (0,06 mm) und die 2-mm-Fugen der Schubkästen (0,04 mm) sind in wahrer Größe gezeichnet und wirken durch ihre Konturen; die Schubkastenböden sind von außen nicht sichtbar."),
        "hatching": T("None; pine, hardboard, porcelain and blanket are told apart by tone.", "Keine; Kiefer, Hartfaser, Porzellan und Decke werden durch Ton unterschieden."),
        "styles": T("Waxed top 0.5, sides 0.62, fronts 0.58, knobs 0.42, porcelain knob 0.95, hardboard 0.85, blanket 0.45; traces of use as 0.13 mm texture strokes.", "Gewachste Deckplatte 0,5, Seiten 0,62, Fronten 0,58, Knöpfe 0,42, Porzellanknopf 0,95, Hartfaser 0,85, Decke 0,45; Gebrauchsspuren als 0,13-mm-Texturstriche."),
        "overlays": T("Drawers are drawn behind the side panel on their own bones, so a pulled or removed drawer appears in every view; the blanket stays on the floor.", "Die Schubkästen sind auf eigenen Knochen hinter der Seite gezeichnet, so erscheint ein gezogener oder herausgenommener Schubkasten in jeder Ansicht; die Decke bleibt auf dem Boden."),
    },
    "100": {
        "silhouette": T("Chest 4.0 x 8.7 mm on the blanket: side panel with the bracket feet and a five-point arch, the top, the knobs; front with fronts, rails, side edges and a five-point apron arch; plan with the plain top.", "Kommode 4,0 x 8,7 mm auf der Decke: Seite mit Sockelfüßen und fünfpunktigem Bogen, Deckplatte, Knöpfe; Vorderansicht mit Fronten, Traversen, Seitenkanten und fünfpunktigem Zargenbogen; Grundriss mit schlichter Deckplatte."),
        "retained": T("Board thicknesses, drawer pitch, the knobs and the replacement porcelain knob, two cup rings, the price label.", "Plattenstärken, Schubkastenabstand, die Knöpfe und der Ersatzknopf aus Porzellan, zwei Tassenränder, das Preisschild."),
        "merged": T("The top edge rounding and the 17 x 12 mm chip (0.17 mm) are merged into a plain slab; each knob is a four-point profile.", "Die Kantenrundung und die 17 x 12 mm große Abplatzung (0,17 mm) gehen in einer schlichten Platte auf; jeder Knopf ist ein Vierpunktprofil."),
        "omitted": T("Scratches, scuffs, the screw hole, the child's sticker, the worn wax outline and the lamella joints except one.", "Kratzer, Scheuerstellen, das Schraubloch, der Kinderaufkleber, die Umrisslinie des abgenutzten Wachses und die Lamellenfugen bis auf eine."),
        "thinElements": T("Back panel and drawer gaps at true size, read by outline.", "Rückwand und Fugen in wahrer Größe, über die Kontur lesbar."),
        "hatching": T("None.", "Keine."),
        "styles": T("Same tones as 1:50, body pen 0.25 mm.", "Gleiche Töne wie 1:50, Körperstift 0,25 mm."),
        "overlays": T("Drawers on their own bones behind the side panel.", "Schubkästen auf eigenen Knochen hinter der Seite."),
    },
    "200": {
        "silhouette": T("Chest 2.0 x 4.3 mm on the blanket: side with a three-segment arch over the plinth, top; front with the plinth, three rails and four fronts; plan with the top.", "Kommode 2,0 x 4,3 mm auf der Decke: Seite mit dreiteiligem Bogen über dem Sockel, Deckplatte; Vorderansicht mit Sockel, drei Traversen und vier Fronten; Grundriss mit Deckplatte."),
        "retained": T("The 22 mm top, the 18 mm side edges and 20 mm rails as lines of the drawer division, the bracket feet.", "Die 22-mm-Deckplatte, die 18-mm-Seitenkanten und 20-mm-Traversen als Linien der Schubkastenteilung, die Sockelfüße."),
        "merged": T("Bottom panel, front and back apron into one plinth; the back panel edge into the side shape; drawer front and box into one tray.", "Bodenplatte, vordere und hintere Zarge in einen Sockel; die Rückwandkante in die Seitenform; Schubkastenfront und Kasten in eine Schale."),
        "omitted": T("Knobs (0.16 mm), the apron arch (0.18 mm), all traces of use: they are below a readable line at this scale.", "Knöpfe (0,16 mm), der Zargenbogen (0,18 mm), alle Gebrauchsspuren: sie liegen in diesem Maßstab unter einer lesbaren Linie."),
        "thinElements": T("Blanket 0.08 mm and top 0.11 mm are carried by the pen.", "Decke 0,08 mm und Deckplatte 0,11 mm werden vom Stift getragen."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.25 mm, tones as 1:50.", "Körperstift 0,25 mm, Töne wie 1:50."),
        "overlays": T("Drawers on their own bones.", "Schubkästen auf eigenen Knochen."),
    },
    "500": {
        "silhouette": T("Chest 0.8 x 1.7 mm on the blanket line: one block with the top line; front with the carcass U, the top and four drawer blocks; plan with the top over the blanket.", "Kommode 0,8 x 1,7 mm auf der Deckenlinie: ein Block mit der Deckplattenlinie; Vorderansicht mit dem Korpus-U, der Deckplatte und vier Schubkastenblöcken; Grundriss mit Deckplatte über der Decke."),
        "retained": T("Overall 0.80 x 0.40 x 0.85 m, the top, the four drawer bands.", "Gesamtmaß 0,80 x 0,40 x 0,85 m, die Deckplatte, die vier Schubkastenbänder."),
        "merged": T("Sides, plinth and back edge into one block; each drawer with the rail above it into one band.", "Seiten, Sockel und Rückwandkante in einen Block; jeder Schubkasten mit der Traverse darüber in ein Band."),
        "omitted": T("Bracket feet, knobs, gaps and all traces of use.", "Sockelfüße, Knöpfe, Fugen und alle Gebrauchsspuren."),
        "thinElements": T("The 16 mm blanket and the 22 mm top are drawn with the body pen.", "Die 16-mm-Decke und die 22-mm-Deckplatte werden mit dem Körperstift gezeichnet."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.25 mm, tones as 1:50.", "Körperstift 0,25 mm, Töne wie 1:50."),
        "overlays": T("Drawer bands on their own bones.", "Schubkastenbänder auf eigenen Knochen."),
    },
}


def representation(scale):
    """🖨️ One authored representation."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": int(scale), "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}, "decisions": REP_DECISIONS[scale]}


def ref(sid, of, **kw):
    """🔗 A reference solid of the thermal or structure list."""
    s = {"id": sid, "of": of}
    s.update(kw)
    return s


def mass_marker(bone, r, scale):
    """🎯 A compact mass marker at the centre of mass of a bone."""
    kg, cx, cy = MASS[bone]
    return {"id": f"mass-{bone}", "kind": "ellipsoid", "bone": bone, "material": "pine-lacquered", "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [r6(cx), r6(cy), 0.0]}, "member": bone, "part": "mass"}


def carcass_region(sid):
    """🌡️ Which thermal region a carcass body solid belongs to (None: inside, no region)."""
    if sid == "top":
        return "top"
    if sid == "back-panel":
        return "back-panel"
    if sid == "blanket":
        return "blanket"
    if sid in ("side-r", "side-l", "bottom", "apron", "apron-back", "plinth", "carcass-shell") or sid.startswith(("rail-", "frame-")):
        return "carcass"
    return None


def front_slab(bone, x0, h):
    """🚪 The thermal drawer front of a scale: a slab that fills the opening to sides and rails, so the closed carcass interior is no outer area."""
    return {"id": f"th-{bone}-front", "kind": "box", "bone": bone, "material": "pine-lacquered", "role": "body", "tone": 0.58, "size": [r6(FRONT_X1 - x0), r6(h), r6(2 * SIDE_ZI)], "transform": {"translation": [r6((FRONT_X1 + x0) / 2), r6(h / 2), 0.0]}, "region": "drawer-fronts"}


def thermal_list(scale):
    """🔥 Thermal list: references of the exposed boards; at 1:50 the drawer fronts are own solids that fill their openings, because the 2 mm gaps lie between a quarter and half a feature."""
    out = []
    for s in BODY[scale]:
        sid = s["id"]
        reg = carcass_region(sid)
        if reg:
            out.append(ref(f"th-{sid}", sid, region=reg))
        if s["bone"].startswith("drawer-"):
            k = int(s["bone"].split("-")[1])
            if scale == "50" and sid.endswith("-front"):
                out.append(front_slab(s["bone"], FRONT_X0, OPENING))
            elif scale == "200" and sid.endswith("-tray"):
                out.append(front_slab(s["bone"], FRONT_X1 - 0.03, OPENING))
            elif scale == "500" and sid.endswith("-block"):
                out.append(front_slab(s["bone"], FRONT_X1 - 0.075, PITCH if k < 4 else OPENING))
            elif sid.endswith("-front") or "-knob-" in sid:
                out.append(ref(f"th-{sid}", sid, region="drawer-fronts"))
    return out


def structure_list(scale):
    """🏗️ Structure list: the boards are the members (a panel carcass is its own structure), corner blocks the joints of the top, drawer boxes the members of the drawer bones, the blanket a mat, mass markers at the centres of mass."""
    r = {"50": 0.02, "100": 0.025, "200": 0.03, "500": 0.0375}[scale]
    out = []
    for s in BODY[scale]:
        sid = s["id"]
        if "-knob-" in sid:
            continue
        if sid.startswith("corner-block"):
            out.append(ref(f"j-{sid}", sid, member="carcass", part="joint"))
        elif s["bone"] == "carcass":
            out.append(ref(f"m-{sid}", sid, member="carcass", part="member"))
        elif s["bone"] == "blanket":
            out.append(ref("m-blanket", sid, member="blanket", part="member"))
        else:
            out.append(ref(f"m-{sid}", sid, member=s["bone"], part="member"))
    out.append(mass_marker("carcass", r, scale))
    for k in range(1, 5):
        out.append(mass_marker(drawer_bone(k), r, scale))
    return out


VOL_DECISIONS = {
    "50": [
        T("Body 1:50 is the chest as built: two 18 mm side panels standing on bracket feet with a 45 mm arch between them, a 22 mm waxed top with its 17 x 12 mm chipped front-left corner, an 18 mm bottom panel, the front apron with its 35 mm arch and the back apron, three drawer frames of a 50 x 20 mm front rail, two 25 x 20 mm runners and a back rail, four corner blocks under the top and the 3 mm hardboard back. Each drawer is its 18 mm inset front, two 12 mm sides, a 12 mm back and a 3 mm hardboard bottom, with two turned pine knobs; drawer 3 has a white porcelain knob on the right where the original was lost. The removal blanket is folded in eight to 0.50 x 0.90 x 0.016 m.", "Body 1:50 ist die Kommode wie gebaut: zwei 18-mm-Seiten auf Sockelfüßen mit 45-mm-Bogen dazwischen, eine 22-mm-Deckplatte mit abgeplatzter vorderer linker Ecke (17 x 12 mm), eine 18-mm-Bodenplatte, die vordere Zarge mit 35-mm-Bogen und die hintere Zarge, drei Schubkastenrahmen aus 50 x 20 mm Traverse, zwei 25 x 20 mm Laufleisten und Rückleiste, vier Eckklötze unter der Deckplatte und die 3-mm-Hartfaserrückwand. Jeder Schubkasten besteht aus 18-mm-Front, zwei 12-mm-Seiten, 12-mm-Rückwand und 3-mm-Hartfaserboden mit zwei gedrechselten Kiefernknöpfen; Schubkasten 3 trägt rechts einen weißen Porzellanknopf statt des verlorenen Originals. Die Möbeldecke ist achtfach auf 0,50 x 0,90 x 0,016 m gefaltet."),
        T("Thermal 1:50: top, carcass, back panel and blanket reference their boards; the drawer fronts are own solids that fill their openings to the sides and rails, so the 2 mm gaps (between a quarter and half a feature) are closed as a seam and the closed carcass interior does not count as outer area; drawer boxes, runners and corner blocks lie inside and belong to no region. Structure 1:50: every board of the carcass is a member of the carcass bone, the four corner blocks its joints, the drawer parts the members of their drawer bones, the blanket a member mat, mass markers at the computed centres of mass; the supports are the pose contacts.", "Thermik 1:50: Deckplatte, Korpus, Rückwand und Decke verweisen auf ihre Platten; die Schubkastenfronten sind eigene Körper, die ihre Öffnung bis zu Seiten und Traversen füllen, damit die 2-mm-Fugen (zwischen einem Viertel und einem halben Merkmal) als Naht geschlossen sind und das geschlossene Korpusinnere nicht als Außenfläche zählt; Schubkästen, Laufleisten und Eckklötze liegen innen und gehören zu keiner Region. Struktur 1:50: jede Korpusplatte ist ein Glied des Korpusknochens, die vier Eckklötze seine Gelenke, die Schubkastenteile die Glieder ihrer Schubkastenknochen, die Decke eine Gliedmatte, Massenmarken in den berechneten Schwerpunkten; die Auflager sind die Kontakte der Pose."),
    ],
    "100": [
        T("Body 1:100 keeps the true boards and both arches with five points each; each drawer frame (rail, runners, back rail) is one closed ring, each drawer box (sides and back) one ring behind its front, each knob a four-point profile; the chip and the edge rounding of the top are below 15 mm and merged into a plain slab.", "Body 1:100 behält die wahren Platten und beide Bögen mit je fünf Punkten; jeder Schubkastenrahmen (Traverse, Laufleisten, Rückleiste) ist ein geschlossener Ring, jeder Kasten (Seiten und Rückwand) ein Ring hinter seiner Front, jeder Knopf ein Vierpunktprofil; Abplatzung und Kantenrundung der Deckplatte liegen unter 15 mm und gehen in einer schlichten Platte auf."),
        T("Thermal 1:100 references the boards, fronts and knobs: the 2 mm gaps are below a quarter feature and close by themselves. Structure 1:100: boards, frames and drawer rings as members, corner blocks as joints, mass markers.", "Thermik 1:100 verweist auf Platten, Fronten und Knöpfe: die 2-mm-Fugen liegen unter einem Viertelmerkmal und schließen von selbst. Struktur 1:100: Platten, Rahmen und Schubkastenringe als Glieder, Eckklötze als Gelenke, Massenmarken."),
    ],
    "200": [
        T("Body 1:200: the sides keep a three-segment arch over a plinth block that merges the bottom panel and both aprons; the top, three frame rings and the back sheet remain; each drawer is one open tray of front and box with its hardboard bottom; the knobs (29 mm) and corner blocks (20 mm) are below 30 mm and left out.", "Body 1:200: die Seiten behalten einen dreiteiligen Bogen über einem Sockelblock, der Bodenplatte und beide Zargen vereint; Deckplatte, drei Rahmenringe und die Rückwandplatte bleiben; jeder Schubkasten ist eine offene Schale aus Front und Kasten mit ihrem Hartfaserboden; die Knöpfe (29 mm) und Eckklötze (20 mm) liegen unter 30 mm und entfallen."),
        T("Thermal 1:200 references plinth, sides, top, back, trays and blanket. Structure 1:200: sides, top, plinth, frames and back as carcass members, trays and bottoms as drawer members, the blanket mat, mass markers.", "Thermik 1:200 verweist auf Sockel, Seiten, Deckplatte, Rückwand, Schalen und Decke. Struktur 1:200: Seiten, Deckplatte, Sockel, Rahmen und Rückwand als Korpusglieder, Schalen und Böden als Schubkastenglieder, die Deckenmatte, Massenmarken."),
    ],
    "500": [
        T("Body 1:500: the carcass is one U of sides and plinth, the 22 mm top and the back sheet; each drawer with the rail above it is one block, stacked in the 190 mm pitch; the blanket is a sheet; feet, knobs, gaps and frames are below 75 mm.", "Body 1:500: der Korpus ist ein U aus Seiten und Sockel, dazu die 22-mm-Deckplatte und die Rückwandplatte; jeder Schubkasten mit der Traverse darüber ist ein Block im Abstand von 190 mm; die Decke ist eine Platte; Füße, Knöpfe, Fugen und Rahmen liegen unter 75 mm."),
        T("Thermal 1:500 references the U, the top, the back, the drawer blocks and the blanket. Structure 1:500: U, top and back as carcass members, the blocks as drawer members, the blanket mat, mass markers.", "Thermik 1:500 verweist auf U, Deckplatte, Rückwand, Schubkastenblöcke und Decke. Struktur 1:500: U, Deckplatte und Rückwand als Korpusglieder, die Blöcke als Schubkastenglieder, die Deckenmatte, Massenmarken."),
    ],
}


def volume(scale):
    """🧊 One authored volume."""
    return {"status": "authored", "denominator": int(scale), "body": clean(BODY[scale]), "thermal": thermal_list(scale), "structure": structure_list(scale), "decisions": VOL_DECISIONS[scale]}


AREAS = {"top": 0.389, "carcass": 1.27, "drawer-fronts": 0.53, "back-panel": 0.60, "blanket": 0.945}
AREA_FILE = os.path.join(HERE, "areas.json")
if os.path.exists(AREA_FILE):
    AREAS.update(json.load(open(AREA_FILE)))


def layer(lid, title, kind, mat, d):
    """🧶 One board layer: thickness, resistance d/k, areal heat capacity rho c d."""
    m = next(x for x in MATERIALS if x["id"] == mat)
    return {"id": lid, "title": title, "kind": kind, "material": mat, "thicknessM": d, "resistanceM2KW": r6(d / m["conductivityWmK"]), "heatCapacityJM2K": round(m["densityKgM3"] * m["specificHeatJKgK"] * d), "source": "literature", "reference": "iso-10456-2007"}


def region(rid, title, mat, ref_, length, geometry, sky, lay, **kw):
    """🌡️ One passive region."""
    r = {"id": rid, "title": title, "kind": "surface", "model": "passive", "material": mat, "areaM2": AREAS[rid], "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": ref_}
    r.update(kw)
    return r


REGIONS = [
    region("top", T("Waxed top, wax worn and darkened", "Gewachste Deckplatte, Wachs abgenutzt und nachgedunkelt"), "pine-waxed-aged", "iso-10456-2007", 0.40, "horizontal-plate-up", 0.5, layer("top-board", T("Pine glued panel 22 mm", "Kiefer-Leimholz 22 mm"), "timber", "pine-waxed-aged", 0.022)),
    region("carcass", T("Side panels, aprons, bottom panel edge and rails", "Seiten, Zargen, Bodenkante und Traversen"), "pine-lacquered", "iso-10456-2007", 0.85, "vertical-plate", 0.4, layer("carcass-board", T("Pine panel 18 mm", "Kiefernplatte 18 mm"), "timber", "pine-lacquered", 0.018), ratedPowerW=0, ratedPowerBasis="passive"),
    region("drawer-fronts", T("Four drawer fronts and their knobs", "Vier Schubkastenfronten mit ihren Knöpfen"), "pine-lacquered", "iso-10456-2007", 0.17, "vertical-plate", 0.4, layer("front-board", T("Pine front 18 mm", "Kiefernfront 18 mm"), "timber", "pine-lacquered", 0.018)),
    region("back-panel", T("Hardboard back panel", "Hartfaserrückwand"), "hardboard", "en-622-2-2004", 0.76, "vertical-plate", 0.1, layer("back-board", T("Hardboard 3 mm", "Hartfaserplatte 3 mm"), "timber", "hardboard", 0.003)),
    region("blanket", T("Folded removal blanket", "Gefaltete Möbeldecke"), "moving-blanket", "removal-blanket", 0.5, "horizontal-plate-up", 0.3, layer("blanket-fleece", T("Quilted fleece, eight plies 16 mm", "Gesteppte Vliesdecke, acht Lagen 16 mm"), "polymer", "moving-blanket", 0.016)),
]


def binding(top, carcass, fronts, back, blanket, blanket_contact):
    """🌬️ The region states of one pose: exposed and sunlit shares, blanket contact with the floor."""
    b = {"exposedFraction": blanket[0], "sunlitFraction": blanket[1]}
    if blanket_contact:
        b["contact"] = {"fraction": blanket_contact, "tempC": 13}
    return {"regions": {"top": {"exposedFraction": top[0], "sunlitFraction": top[1]}, "carcass": {"exposedFraction": carcass[0], "sunlitFraction": carcass[1]}, "drawer-fronts": {"exposedFraction": fronts[0], "sunlitFraction": fronts[1]}, "back-panel": {"exposedFraction": back[0], "sunlitFraction": back[1]}, "blanket": b}, "emitters": {}}


def clothes_loads():
    """👕 Folded clothes in every drawer, spread over the drawer bottom."""
    w = CLOTHES_KG * G / (FRONT_X0 - 0.044)
    return [{"id": f"clothes-{k}", "kind": "live", "title": T(f"Folded clothes in drawer {k} ({CLOTHES_KG:.0f} kg)", f"Gefaltete Kleidung in Schubkasten {k} ({CLOTHES_KG:.0f} kg)"), "distributed": {"bone": drawer_bone(k), "fromM": 0.044, "toM": FRONT_X0, "wxNPerM": 0, "wyNPerM": r6(-w)}, "fxN": 0, "fyN": 0, "source": "authored-estimate", "reference": "en-16122-2012"} for k in range(1, 5)]


def stack_bones():
    """📚 Bone offsets of the four drawers taken out and stacked in front of the chest, with the carcass lifted (chest dy compensated in every level)."""
    bones = {}
    for k in range(1, 5):
        b = drawer_bone(k)
        rest = BLANKET_T + LEVELS[k - 1]
        bones[f"{b}-level"] = {"dy": r6(-LIFT + STACK_BOTTOM[k - 1] - rest)}
        bones[b] = {"dx": STACK_PULL[k - 1]}
    return bones


CARRIED_BONES = {"chest": {"dy": LIFT}, "carcass-carry": {"dx": CARRY_BACK}, "carcass": {"angleDeg": TILT}}
CARRIED_BONES.update(stack_bones())
IN_USE_BONES = {"chest": {"dy": -BLANKET_T}, "blanket": {"dx": 0.5}}
OPEN_BONES = dict(IN_USE_BONES)
OPEN_BONES.update({drawer_bone(k): {"dx": PULL_USE} for k in range(1, 5)})

def stability():
    """🧮 Centre of mass of the open pose against the front feet, and the 22.7 kg child test mass of ASTM F2057 on the open top drawer."""
    cm = MASS["carcass"]
    m = cm[0]
    mx = cm[0] * cm[1]
    for k in range(1, 5):
        d = MASS[drawer_bone(k)]
        m += d[0] + CLOTHES_KG
        mx += d[0] * (d[1] + PULL_USE) + CLOTHES_KG * ((0.044 + FRONT_X0) / 2 + PULL_USE)
    com = mx / m
    edge = 0.39
    restoring = m * G * (edge - com)
    child = 22.7 * G * (FRONT_X1 + PULL_USE - edge)
    return m, com, edge, restoring, child


M_OPEN, COM_OPEN, EDGE, RESTORING, CHILD = stability()

POSES = [
    {"id": "stored", "title": T("Stored in the reuse depot", "Gelagert im Gebrauchtwarenlager"), "description": T("Emptied and cleaned, the chest stands on a removal blanket folded in eight on the depot floor, drawers closed, price label on the second drawer: the blanket carries 30 kg on 0.45 m2.", "Geleert und gereinigt steht die Kommode auf einer achtfach gefalteten Möbeldecke auf dem Lagerboden, Schubkästen geschlossen, Preisschild am zweiten Schubkasten: die Decke trägt 30 kg auf 0,45 m2."), "bones": {}, "ground": "auto", "contacts": ["blanket-floor"], "thermal": binding((1.0, 0.5), (1.0, 0.1), (1.0, 0.3), (1.0, 0.0), (1.0, 0.2), 0.5), "loads": []},
    {"id": "carried", "title": T("Carried by two people, drawers taken out", "Von zwei Personen getragen, Schubkästen herausgenommen"), "description": T(f"The four drawers are stacked in front of the chest on the floor; two carriers lift the empty {MASS['carcass'][0]:.1f} kg carcass 0.25 m, each with one hand under the side arch and one under the top overhang, tilt it 3 degrees and carry it 0.45 m backwards; the blanket stays on the floor.", f"Die vier Schubkästen stehen gestapelt vor der Kommode auf dem Boden; zwei Träger heben den leeren Korpus mit {MASS['carcass'][0]:.1f} kg um 0,25 m an, je eine Hand unter dem Seitenbogen und eine unter dem Deckplattenüberstand, kippen ihn um 3 Grad und tragen ihn 0,45 m rückwärts; die Decke bleibt auf dem Boden."), "bones": CARRIED_BONES, "ground": "authored", "contacts": GRIPS + ["drawer-stack-floor", "blanket-floor"], "thermal": binding((1.0, 0.0), (1.0, 0.0), (1.0, 0.0), (1.0, 0.0), (1.0, 0.0), 0.5), "loads": []},
    {"id": "in-use", "title": T("In use in a bedroom", "In Gebrauch im Schlafzimmer"), "description": T("Back in a bedroom the chest stands on its four bracket feet on the floor, the drawers closed and filled with 4 kg of folded clothes each; the blanket it came on lies folded in front of it.", "Zurück in einem Schlafzimmer steht die Kommode auf ihren vier Sockelfüßen auf dem Boden, die Schubkästen geschlossen und mit je 4 kg gefalteter Kleidung gefüllt; die Decke, auf der sie kam, liegt gefaltet davor."), "bones": IN_USE_BONES, "ground": "auto", "contacts": FEET + ["blanket-floor"], "thermal": binding((1.0, 0.3), (1.0, 0.1), (1.0, 0.4), (1.0, 0.0), (1.0, 0.3), 0.5), "loads": clothes_loads()},
    {"id": "drawers-open", "title": T("All drawers open", "Alle Schubkästen offen"), "description": T(f"All four filled drawers stand open 0.25 m at their stops: the centre of mass of the {M_OPEN:.1f} kg chest moves to {1000 * (EDGE - COM_OPEN):.0f} mm behind the front feet, so it still stands, with little margin (EN 14749 stability case).", f"Alle vier gefüllten Schubkästen stehen 0,25 m offen an ihren Anschlägen: der Schwerpunkt der {M_OPEN:.1f} kg schweren Kommode rückt bis {1000 * (EDGE - COM_OPEN):.0f} mm hinter die Vorderfüße, sie steht also noch, mit wenig Reserve (Standsicherheitsfall nach EN 14749)."), "bones": OPEN_BONES, "ground": "auto", "contacts": FEET + ["blanket-floor"], "thermal": binding((1.0, 0.3), (1.0, 0.1), (1.0, 0.4), (1.0, 0.0), (1.0, 0.3), 0.5), "loads": clothes_loads()},
]


def carry_clip():
    """🧺 Clip stored → carried: the drawers come out one by one and are stacked on the floor, then the carcass is lifted and carried away."""
    keys = [{"t": 0, "pose": "stored", "ease": "linear"}]
    state = {}
    t = 0.0
    events = []
    for k in range(1, 5):
        b = drawer_bone(k)
        rest = BLANKET_T + LEVELS[k - 1]
        t += 1.0
        state[b] = {"dx": STACK_PULL[k - 1]}
        keys.append({"t": r6(t), "pose": "stored", "ease": "linear", "bones": json.loads(json.dumps(state))})
        events.append({"t": r6(t), "kind": "motion", "title": T(f"Drawer {k} pulled out", f"Schubkasten {k} herausgezogen"), "target": b})
        t += 0.8
        state[f"{b}-level"] = {"dy": r6(STACK_BOTTOM[k - 1] - rest)}
        keys.append({"t": r6(t), "pose": "stored", "ease": "linear", "bones": json.loads(json.dumps(state))})
        events.append({"t": r6(t), "kind": "motion", "title": T(f"Drawer {k} set down on the stack", f"Schubkasten {k} auf dem Stapel abgesetzt"), "target": b})
    t_lift0 = t
    lifted = json.loads(json.dumps(state))
    lifted["chest"] = {"dy": LIFT}
    for k in range(1, 5):
        lifted[f"{drawer_bone(k)}-level"] = {"dy": r6(lifted[f"{drawer_bone(k)}-level"]["dy"] - LIFT)}
    t += 1.2
    keys.append({"t": r6(t), "pose": "stored", "ease": "linear", "bones": lifted})
    t_lift1 = t
    t += 1.6
    keys.append({"t": r6(t), "pose": "carried", "ease": "linear"})
    events.append({"t": r6(t_lift0), "kind": "contact-on", "title": T("Carriers take hold of the carcass", "Träger fassen den Korpus"), "target": "grip-l-low"})
    events.append({"t": r6(t_lift1), "kind": "motion", "title": T("Carcass lifted 0.25 m", "Korpus 0,25 m angehoben")})
    return {"id": "carry", "title": T("Take out the drawers and carry the carcass", "Schubkästen herausnehmen und Korpus tragen"), "description": T("The four drawers are pulled out from the bottom up and stacked on the floor in front of the chest; two carriers then lift the empty carcass off the blanket and carry it backwards; the blanket stays where it was.", "Die vier Schubkästen werden von unten nach oben herausgezogen und vor der Kommode auf dem Boden gestapelt; dann heben zwei Träger den leeren Korpus von der Decke und tragen ihn rückwärts weg; die Decke bleibt liegen."), "durationS": r6(t), "fps": 25, "loop": False, "rootMotion": "none", "keys": keys, "contacts": [{"anchor": "blanket-floor", "fromS": 0, "toS": r6(t)}, {"anchor": "drawer-stack-floor", "fromS": 1.8, "toS": r6(t)}] + [{"anchor": g, "fromS": r6(t_lift0), "toS": r6(t)} for g in GRIPS], "phases": [{"id": "unload", "fromS": 0, "toS": r6(t_lift0), "title": T("Taking out the drawers", "Schubkästen herausnehmen")}, {"id": "lift", "fromS": r6(t_lift0), "toS": r6(t_lift1), "title": T("Lifting the carcass", "Korpus anheben")}, {"id": "carry", "fromS": r6(t_lift1), "toS": r6(t), "title": T("Carrying away", "Wegtragen")}], "events": events, "tracks": []}


def pull_clip():
    """🧦 Clip in use: each drawer is pulled to its stop and pushed back, top drawer first."""
    keys = []
    events = []
    t = 0.0
    for k in (4, 3, 2, 1):
        b = drawer_bone(k)
        keys.append({"t": r6(t), "pose": "in-use", "ease": "out"})
        keys.append({"t": r6(t + 0.9), "pose": "in-use", "ease": "linear", "bones": {b: {"dx": PULL_USE}}})
        keys.append({"t": r6(t + 1.3), "pose": "in-use", "ease": "in", "bones": {b: {"dx": PULL_USE}}})
        events.append({"t": r6(t + 0.9), "kind": "motion", "title": T(f"Drawer {k} at its stop", f"Schubkasten {k} am Anschlag"), "target": b})
        t += 2.0
    keys.append({"t": r6(t), "pose": "in-use", "ease": "linear"})
    return {"id": "pull-drawers", "title": T("Open and close the drawers", "Schubkästen auf- und zuziehen"), "description": T("In the bedroom each filled drawer is pulled 0.25 m to its stop and pushed back again, from the top drawer down, as when looking for a pullover.", "Im Schlafzimmer wird jeder gefüllte Schubkasten bis zum Anschlag 0,25 m herausgezogen und wieder zugeschoben, vom obersten an abwärts, wie bei der Suche nach einem Pullover."), "durationS": r6(t), "fps": 25, "loop": True, "rootMotion": "none", "keys": keys, "contacts": [{"anchor": f, "fromS": 0, "toS": r6(t)} for f in FEET + ["blanket-floor"]], "phases": [{"id": f"drawer-{k}", "fromS": r6(i * 2.0), "toS": r6(i * 2.0 + 2.0), "title": T(f"Drawer {k}", f"Schubkasten {k}")} for i, k in enumerate((4, 3, 2, 1))], "events": events, "tracks": []}



THERMAL = {
    "environment": {"airTempC": 13, "meanRadiantTempC": 12, "relativeHumidity": 0.65, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 250, "diffuseFraction": 0.3, "pressurePa": 101325},
    "operating": [],
    "regions": REGIONS,
    "emitters": [],
    "passengers": [],
    "assumptions": [
        T("Passive solid pine (510 kg/m3, 0.13 W/mK, 1600 J/kgK after ISO 10456 and the Holzatlas), 3 mm hardboard and a quilted removal blanket; areal heat capacity rho c d of the boards (18 mm: 14.7 kJ/m2K, 22 mm: 18.0 kJ/m2K).", "Passive Massivkiefer (510 kg/m3, 0,13 W/mK, 1600 J/kgK nach ISO 10456 und Holzatlas), 3 mm Hartfaser und eine gesteppte Möbeldecke; flächenbezogene Wärmekapazität rho c d der Platten (18 mm: 14,7 kJ/m2K, 22 mm: 18,0 kJ/m2K)."),
        T("Aged surfaces differ from new ones: the worn and darkened wax of the top is taken with a solar absorptance of 0.55 against about 0.40 for freshly waxed pine, the yellowed lacquer of carcass and fronts with 0.45 (authored estimates from the colour, emissivity 0.9 of wood after Incropera Table A.11).", "Gealterte Oberflächen unterscheiden sich von neuen: das abgenutzte und nachgedunkelte Wachs der Deckplatte wird mit einem solaren Absorptionsgrad von 0,55 statt etwa 0,40 für frisch gewachste Kiefer angesetzt, der vergilbte Lack von Korpus und Fronten mit 0,45 (Schätzwerte aus der Farbe, Emissionsgrad 0,9 von Holz nach Incropera Tabelle A.11)."),
        T("Reference case: unheated reuse depot in late autumn, 13 degC air, 12 degC surfaces, a 250 W/m2 sun patch through the open roller door on half of the top and on the drawer fronts; the drawer boxes, runners and corner blocks lie in the closed carcass and are no region; the blanket lies with half of its surface on the 13 degC floor.", "Referenzfall: unbeheiztes Gebrauchtwarenlager im Spätherbst, 13 degC Luft, 12 degC Umgebungsflächen, ein Sonnenfleck von 250 W/m2 durch das offene Rolltor auf der halben Deckplatte und den Schubkastenfronten; Schubkästen, Laufleisten und Eckklötze liegen im geschlossenen Korpus und sind keine Region; die Decke liegt mit der Hälfte ihrer Fläche auf dem 13 degC kalten Boden."),
        T("No element draws power: the carcass region states 0 W passive explicitly.", "Kein Element nimmt Leistung auf: die Korpusregion gibt ausdrücklich 0 W passiv an."),
    ],
}

STRUCTURE = {
    "gravityMS2": G,
    "dynamics": "quasi-static",
    "supports": [],
    "assumptions": [
        T(f"Masses from the 1:50 solids (section x density: pine 510 kg/m3, hardboard 900 kg/m3, porcelain 2400 kg/m3, blanket 225 kg/m3): carcass {MASS['carcass'][0]:.2f} kg, drawers {MASS['drawer-1'][0]:.2f} to {MASS['drawer-3'][0]:.2f} kg, blanket {MASS['blanket'][0]:.2f} kg, total {TOTAL:.2f} kg.", f"Massen aus den 1:50-Körpern (Querschnitt x Dichte: Kiefer 510 kg/m3, Hartfaser 900 kg/m3, Porzellan 2400 kg/m3, Decke 225 kg/m3): Korpus {MASS['carcass'][0]:.2f} kg, Schubkästen {MASS['drawer-1'][0]:.2f} bis {MASS['drawer-3'][0]:.2f} kg, Decke {MASS['blanket'][0]:.2f} kg, gesamt {TOTAL:.2f} kg."),
        T(f"Load paths: stored, the bracket feet press on the blanket and the blanket on the floor (one floor contact of 0.50 x 0.90 m); in use, the four bracket feet stand on the floor; carried, two carriers hold the empty carcass at four pins (hand under each side arch and under each top overhang, about {MASS['carcass'][0] * G / 4:.0f} N per hand), while the drawer stack and the blanket rest on the floor. The product solves the whole asset as one rigid body on all contacts of a pose, so in the carried pose the shares between the hands and the floor contacts are the solver's elastic split, not separate equilibria.", f"Lastpfade: gelagert drücken die Sockelfüße auf die Decke und die Decke auf den Boden (ein Bodenkontakt 0,50 x 0,90 m); in Gebrauch stehen die vier Sockelfüße auf dem Boden; getragen halten zwei Träger den leeren Korpus an vier Gelenkpunkten (je eine Hand unter jedem Seitenbogen und unter jedem Deckplattenüberstand, etwa {MASS['carcass'][0] * G / 4:.0f} N je Hand), während Schubkastenstapel und Decke auf dem Boden liegen. Das Produkt löst das ganze Asset als einen starren Körper auf allen Kontakten einer Pose, im getragenen Zustand ist die Aufteilung zwischen Händen und Bodenkontakten daher die elastische Verteilung des Lösers, keine getrennten Gleichgewichte."),
        T(f"Drawers slide on 25 x 20 mm pine runners (the bottom drawer on the bottom panel); each drawer is a beam of two 12 x 130 mm sides (I = 4.39e-6 m4) carrying {CLOTHES_KG:.0f} kg of folded clothes (EN 16122 drawer loading as scope, the load an authored estimate).", f"Die Schubkästen gleiten auf 25 x 20 mm Kiefernlaufleisten (der unterste auf der Bodenplatte); jeder Schubkasten ist ein Balken aus zwei 12 x 130 mm Seiten (I = 4,39e-6 m4) mit {CLOTHES_KG:.0f} kg gefalteter Kleidung (Schubkastenbelastung nach EN 16122 als Rahmen, die Last geschätzt)."),
        T(f"Stability (EN 14749, ASTM F2057): with all four filled drawers open 0.25 m the {M_OPEN:.1f} kg chest has its centre of mass at x = {COM_OPEN:.3f} m, {1000 * (EDGE - COM_OPEN):.0f} mm behind the front feet (restoring moment {RESTORING:.1f} Nm); a 22.7 kg child on the open top drawer would add {CHILD:.1f} Nm and tip it, which is why such chests are anchored to the wall.", f"Standsicherheit (EN 14749, ASTM F2057): mit allen vier gefüllten, 0,25 m geöffneten Schubkästen liegt der Schwerpunkt der {M_OPEN:.1f} kg schweren Kommode bei x = {COM_OPEN:.3f} m, {1000 * (EDGE - COM_OPEN):.0f} mm hinter den Vorderfüßen (Rückstellmoment {RESTORING:.1f} Nm); ein Kind von 22,7 kg auf dem offenen obersten Schubkasten fügte {CHILD:.1f} Nm hinzu und kippte sie, deshalb werden solche Kommoden an der Wand gesichert."),
    ],
    "clinical": False,
}

ASSET = {
    "id": "used-chest-of-drawers",
    "category": "furniture",
    "title": T("Used Chest of Drawers", "Gebrauchte Kommode"),
    "summary": T("A second-hand solid pine chest of drawers, 0.80 x 0.40 x 0.85 m and 28 kg, with four inset drawers on wooden runners: the waxed top is worn and chipped at one corner, a lost knob was replaced by a white porcelain one. It is stored on a folded removal blanket, carried as an empty carcass by two people after the drawers are stacked on the floor, and back in use in a bedroom with clothes in the drawers.", "Eine gebrauchte Kommode aus Massivkiefer, 0,80 x 0,40 x 0,85 m und 28 kg, mit vier eingelassenen Schubkästen auf Holzlaufleisten: die gewachste Deckplatte ist abgenutzt und an einer Ecke abgeplatzt, ein verlorener Knopf wurde durch einen weißen Porzellanknopf ersetzt. Sie lagert auf einer gefalteten Möbeldecke, wird nach dem Stapeln der Schubkästen als leerer Korpus von zwei Personen getragen und steht wieder in Gebrauch in einem Schlafzimmer mit Kleidung in den Schubkästen."),
    "tags": ["used", "household", "second-hand", "bedroom", "chest-of-drawers", "storage", "drawers", "solid-wood", "pine", "waxed", "interior"],
    "provenance": {"author": "Semio staffage content (used household objects)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": T("Built to the catalogue-typical solid pine chest of drawers of 2000-2015 (no brand); every parameter is literature or an authored estimate, nothing is measured.", "Gebaut nach der katalogtypischen Massivkiefer-Kommode der Jahre 2000-2015 (ohne Marke); jeder Parameter ist Literatur oder Schätzung, nichts ist gemessen.")},
    "dimensions": {"heightM": r6(BLANKET_T + HEIGHT), "widthM": r6(BLANKET_X1 - BLANKET_X0), "depthM": r6(2 * BLANKET_Z), "massKg": round(TOTAL, 3)},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "floor", "bones": BONES},
    "anchors": ANCHORS,
    "representations": {s: representation(s) for s in ("500", "200", "100", "50")},
    "symbols": [],
    "poses": POSES,
    "clips": [carry_clip(), pull_clip()],
    "thermal": THERMAL,
    "structure": STRUCTURE,
    "volumes": {s: volume(s) for s in ("50", "100", "200", "500")},
    "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}

if __name__ == "__main__":
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(DOC, f, ensure_ascii=False, indent=1)
        f.write("\n")
    for b, (kg, cx, cy) in MASS.items():
        print(f"[DEBUG] mass {b}: {kg:.3f} kg at ({cx:.3f}, {cy:.3f})")
    print(f"[DEBUG] total {TOTAL:.3f} kg; open pose {M_OPEN:.2f} kg com x {COM_OPEN:.4f} edge {EDGE} restoring {RESTORING:.1f} Nm child {CHILD:.1f} Nm")
    print({s: (len(BODY[s]), len(thermal_list(s)), len(structure_list(s))) for s in BODY})
