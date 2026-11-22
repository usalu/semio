"""🏮️ Authors the staffage asset `used-park-lantern` (a used 4 m cast-aluminium post-top park lantern on a hinged foot) from hand-chosen numbers.

Writes `🔣️.json` next to this file. Every number below is a decision of the author; the reference table is in `../../reports/used-park-lantern.md`.
World frame: x ahead, y up, z left. Bones: foundation (world axes), hinge (pivot of the tilting foot), column (mast axis, u up, v back),
door (base door, hinged at its lower edge), lantern (lantern axis on the spigot, u up, v back).

@see ../../brief.md
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json, math, os

HERE = os.path.dirname(os.path.abspath(__file__))
T = lambda en, de: {"en": en, "de": de}


def r6(x):
    v = round(x, 6)
    return 0.0 if v == 0 else v


def pts(points):
    return [[r6(a), r6(b)] for a, b in points]


PIVOT = (0.15, 0.095)
COLUMN_Y = 0.095
LANTERN_U = 3.56
EAVE_R = PIVOT[0] + PIVOT[1]
FIN_TOP = 0.985
HEIGHT = COLUMN_Y + LANTERN_U + FIN_TOP
DOOR_U, DOOR_H, DOOR_HALF = 0.22, 0.30, math.radians(27)
STRUT_ANGLES = [0, 60, 120, 180, 240, 300]

COLUMN = {
    "foot-ring": [(-0.02, 0.145), (0.0, 0.145)],
    "plinth": [(0.0, 0.125), (0.05, 0.125), (0.055, 0.1275), (0.0625, 0.1295), (0.0725, 0.130), (0.0825, 0.1295), (0.09, 0.1275), (0.095, 0.123), (0.10, 0.118), (0.11, 0.1135), (0.125, 0.111), (0.14, 0.110)],
    "capital": [(0.70, 0.110), (0.70, 0.116), (0.705, 0.1205), (0.715, 0.122), (0.725, 0.1205), (0.73, 0.116), (0.73, 0.105), (0.76, 0.105), (0.96, 0.068), (0.96, 0.075), (0.99, 0.075)],
    "shaft": [(0.99, 0.057), (3.50, 0.044)],
    "top": [(3.50, 0.050), (3.505, 0.054), (3.52, 0.056), (3.535, 0.054), (3.54, 0.050), (3.54, 0.046), (3.56, 0.046), (3.56, 0.038), (3.66, 0.038)],
}
LANTERN = {
    "neck": [(0.0, 0.058), (0.015, 0.058), (0.02, 0.052), (0.11, 0.052)],
    "bowl": [(0.11, 0.052), (0.12, 0.07), (0.13, 0.09), (0.14, 0.11), (0.15, 0.125), (0.15, 0.132), (0.17, 0.132)],
    "top-ring": [(0.59, 0.200), (0.615, 0.200)],
    "roof": [(0.615, EAVE_R), (0.64, EAVE_R), (0.64, 0.238), (0.655, 0.22), (0.67, 0.20), (0.69, 0.175), (0.71, 0.15), (0.73, 0.125), (0.75, 0.102), (0.77, 0.085)],
    "crown": [(0.77, 0.075), (0.84, 0.070)],
    "cap": [(0.84, 0.088), (0.856, 0.088)],
    "finial": [(0.856, 0.018), (0.87, 0.018), (0.875, 0.028), (0.885, 0.034), (0.90, 0.036), (0.915, 0.034), (0.925, 0.028), (0.93, 0.018), (0.94, 0.014), (FIN_TOP, 0.004)],
    "led-bracket": [(0.17, 0.012), (0.22, 0.012)],
    "led-module": [(0.22, 0.045), (0.42, 0.045), (0.43, 0.040), (0.44, 0.025)],
}
GLASS = [(0.17, 0.114), (0.59, 0.186), (0.59, 0.190), (0.17, 0.118)]


def filled(profile):
    a0, a1 = profile[0][0], profile[-1][0]
    return pts([(a0, 0.0)] + profile + [(a1, 0.0)])


def loop(profile):
    return pts(profile + [profile[0]])


def outline(profile):
    upper = [(a, r) for a, r in profile]
    lower = [(a, -r) for a, r in reversed(profile) if r > 1e-9]
    if profile[0][1] > 1e-9:
        pass
    return pts(upper + lower)


def ring_outline(r_out, a0, a1):
    return pts([(a0, r_out), (a1, r_out), (a1, -r_out), (a0, -r_out)])


def arc(r, a0, a1, n, cv=0.0):
    return [(cv + r * math.cos(a0 + (a1 - a0) * i / n), r * math.sin(a0 + (a1 - a0) * i / n)) for i in range(n + 1)]


def door_band(t):
    outer = arc(0.110, -DOOR_HALF, DOOR_HALF, 10, -0.110)
    inner = arc(0.110 - t, DOOR_HALF, -DOOR_HALF, 10, -0.110)
    return pts(outer + inner)


def shell_c(t):
    outer = arc(0.110, DOOR_HALF, 2 * math.pi - DOOR_HALF, 48)
    inner = arc(0.110 - t, 2 * math.pi - DOOR_HALF, DOOR_HALF, 48)
    return pts(outer + inner)


def hexagon(cx, cy, af):
    r = af / math.sqrt(3)
    return pts([(cx + r * math.cos(math.pi / 3 * i), cy + r * math.sin(math.pi / 3 * i)) for i in range(6)])


def circle(cx, cy, r, n=16):
    return pts([(cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)) for i in range(n)])


def rad_at(profile, u):
    for (a0, r0), (a1, r1) in zip(profile, profile[1:]):
        if a0 <= u <= a1 and a1 > a0:
            return r0 + (r1 - r0) * (u - a0) / (a1 - a0)
    return profile[-1][1]


def glass_r(u):
    return 0.118 + (0.190 - 0.118) * (u - 0.17) / 0.42


MATERIALS = [
    {"id": "concrete-weathered", "title": T("Weathered precast concrete", "Verwitterter Betonfertigteil"), "emissivity": 0.93, "solarAbsorptivity": 0.72, "densityKgM3": 2400, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "elasticModulusPa": 33000000000, "source": "authored-estimate", "reference": "berdahl-2008"},
    {"id": "galv-steel-weathered", "title": T("Weathered hot-dip galvanised steel", "Verwitterter feuerverzinkter Stahl"), "emissivity": 0.45, "solarAbsorptivity": 0.75, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "source": "authored-estimate", "reference": "berdahl-2008"},
    {"id": "cast-aluminium-painted", "title": T("Cast aluminium AC-44100, chalked fir-green powder coat", "Aluminiumguss AC-44100, verkreidete tannengrüne Pulverbeschichtung"), "emissivity": 0.9, "solarAbsorptivity": 0.86, "densityKgM3": 2650, "conductivityWmK": 130, "specificHeatJKgK": 900, "elasticModulusPa": 75000000000, "strengthPa": 240000000, "source": "authored-estimate", "reference": "en-1706"},
    {"id": "aluminium-6060-painted", "title": T("Extruded aluminium EN AW-6060 T66, chalked fir-green powder coat", "Strangpress-Aluminium EN AW-6060 T66, verkreidete tannengrüne Pulverbeschichtung"), "emissivity": 0.9, "solarAbsorptivity": 0.86, "densityKgM3": 2700, "conductivityWmK": 200, "specificHeatJKgK": 900, "elasticModulusPa": 70000000000, "strengthPa": 160000000, "source": "authored-estimate", "reference": "en-755-2"},
    {"id": "aluminium-bare-oxidised", "title": T("Bare oxidised aluminium at paint chips", "Blankes, oxidiertes Aluminium an Lackabplatzern"), "emissivity": 0.2, "solarAbsorptivity": 0.35, "densityKgM3": 2650, "conductivityWmK": 130, "specificHeatJKgK": 900, "source": "literature", "reference": "incropera-2018"},
    {"id": "polycarbonate-yellowed", "title": T("UV-yellowed polycarbonate glazing", "UV-vergilbte Polycarbonat-Verglasung"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "densityKgM3": 1200, "conductivityWmK": 0.2, "specificHeatJKgK": 1200, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "polycarbonate-new", "title": T("New clear polycarbonate (replaced pane)", "Neues klares Polycarbonat (ersetzte Scheibe)"), "emissivity": 0.9, "solarAbsorptivity": 0.12, "densityKgM3": 1200, "conductivityWmK": 0.2, "specificHeatJKgK": 1200, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "led-heatsink", "title": T("Black-coated die-cast aluminium LED heat sink", "Schwarz beschichteter Aluminiumdruckguss-Kühlkörper der LED"), "emissivity": 0.85, "solarAbsorptivity": 0.8, "densityKgM3": 2700, "conductivityWmK": 150, "specificHeatJKgK": 900, "source": "authored-estimate", "reference": "schubert-2006"},
    {"id": "polyamide-grey", "title": T("Grey glass-fibre polyamide (connection box)", "Grauer glasfaserverstärkter Polyamid (Anschlusskasten)"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 1350, "conductivityWmK": 0.3, "specificHeatJKgK": 1500, "source": "literature", "reference": "ashby-2011"},
    {"id": "spray-paint", "title": T("Graffiti spray paint", "Graffiti-Sprühlack"), "emissivity": 0.92, "solarAbsorptivity": 0.7, "source": "illustrative"},
    {"id": "algae-film", "title": T("Green algae film", "Grüner Algenbelag"), "emissivity": 0.95, "solarAbsorptivity": 0.85, "source": "illustrative"},
    {"id": "sticker-paper", "title": T("Sticker and poster residue", "Aufkleber- und Plakatreste"), "emissivity": 0.92, "solarAbsorptivity": 0.4, "source": "illustrative"},
]

REFERENCES = [
    {"id": "en-40-2", "citation": "EN 40-2:2004 Lighting columns, Part 2: General requirements and dimensions (post-top spigots 60 and 76 mm x 100 mm, door openings, tolerances)."},
    {"id": "en-40-3-1", "citation": "EN 40-3-1:2013 Lighting columns, Part 3-1: Design and verification, specification for characteristic loads (wind on column and luminaire)."},
    {"id": "en-40-6", "citation": "EN 40-6:2002 Lighting columns, Part 6: Requirements for aluminium lighting columns (cast and extruded alloys, wall thicknesses, door reinforcement)."},
    {"id": "en-1706", "citation": "EN 1706:2020 Aluminium and aluminium alloys, Castings, Chemical composition and mechanical properties (EN AC-44100 AlSi12(b): density 2650 kg/m3, Rm about 150 to 240 MPa, E about 75 GPa)."},
    {"id": "en-755-2", "citation": "EN 755-2:2016 Aluminium and aluminium alloys, Extruded rod/bar, tube and profiles, Part 2: Mechanical properties (EN AW-6060 T66: Rp0.2 160 MPa)."},
    {"id": "en-1999-1-1", "citation": "EN 1999-1-1:2007 Eurocode 9: Design of aluminium structures, Part 1-1 (E = 70 GPa, density 2700 kg/m3)."},
    {"id": "en-1991-1-4", "citation": "EN 1991-1-4:2005 with DIN EN 1991-1-4/NA:2010-12 (wind zone 2 inland, vb,0 = 25 m/s, simplified peak velocity pressure 0.65 kN/m2 up to 10 m height)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures, Part 1-1: densities, self-weight, imposed loads (horizontal loads on partitions and barriers, section 6.4)."},
    {"id": "en-1992-1-1", "citation": "EN 1992-1-1:2004 Eurocode 2: Design of concrete structures (normal-weight concrete 2400 kg/m3, C30/37 E about 33 GPa)."},
    {"id": "en-13201-2", "citation": "EN 13201-2:2015 Road lighting, Part 2: Performance requirements (classes P4 to P6 for park paths, mounting heights 3.5 to 5 m)."},
    {"id": "licht-wissen-03", "citation": "licht.de (2014). licht.wissen 03: Strassen, Wege und Plaetze. Frankfurt am Main (park and path lighting with post-top luminaires on 3.5 to 4.5 m columns)."},
    {"id": "catalogue-heritage-columns", "citation": "Catalogue-typical data of heritage-style cast-aluminium post-top columns of German municipal street-furniture suppliers, taken as a type without brand: overall column 3.5 to 4 m, cast base 250 to 330 mm, shaft 114 to 76 mm, spigot 76 x 100 mm, 30 to 50 kg, flange on four M16 to M20 anchor bolts or hinged foot (Klappmast) for parks without aerial-platform access; round heritage lanterns 0.4 to 0.5 m across, 8 to 15 kg."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products, Hygrothermal properties, Tabulated design values (conductivity, specific heat and density of concrete, steel, aluminium, polycarbonate)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities of oxidised aluminium, galvanised steel, paints)."},
    {"id": "berdahl-2008", "citation": "Berdahl, P., Akbari, H., Levinson, R., Miller, W. A. (2008). Weathering of roofing materials, An overview. Construction and Building Materials 22(4), 423-433 (soiling, chalking and biological growth change the solar reflectance of weathered surfaces)."},
    {"id": "ashby-2011", "citation": "Ashby, M. F. (2011). Materials Selection in Mechanical Design, 4th ed., Butterworth-Heinemann (density and conductivity of polyamides)."},
    {"id": "schubert-2006", "citation": "Schubert, E. F. (2006). Light-Emitting Diodes, 2nd ed., Cambridge University Press (more than half of the electrical input of a white LED is dissipated as heat in package and heat sink)."},
]


def mass_bones():
    rho_c, rho_s, rho_ac, rho_aw, rho_pc = 2400, 7850, 2650, 2700, 1200
    head = (0.40 * 0.40 - 0.04 * 0.04 / 2) * 0.06 * rho_c
    plate = 0.34 * 0.34 * 0.015 * rho_s
    nuts = 4 * (0.25 * 3 ** 0.5 * 0.024 ** 2 * 0.013 + math.pi * 0.008 ** 2 * 0.012) * rho_s
    lugs = 2 * (0.03 * 0.02 + math.pi * 0.015 ** 2 / 2) * 0.012 * rho_s
    pin = math.pi * 0.008 ** 2 * 0.16 * rho_s
    knuckle = math.pi * 0.016 ** 2 * 0.104 * rho_ac
    foot = math.pi * (0.145 ** 2 - 0.09 ** 2) * 0.02 * rho_ac
    base_area = 2 * math.pi * 0.125 * 0.14 + 2 * math.pi * 0.106 * 0.56 + 2 * math.pi * 0.09 * 0.29
    door = 2 * DOOR_HALF * 0.105 * DOOR_H * 0.010 * rho_ac
    base = base_area * 0.010 * rho_ac - door
    shaft = 2 * math.pi * (0.0505 - 0.003) * 0.006 * 2.51 * rho_aw
    top = (math.pi * (0.056 ** 2 - 0.044 ** 2) * 0.06 + math.pi * (0.038 ** 2 - 0.032 ** 2) * 0.10) * rho_aw
    box = 0.6
    bolts = 2 * 0.011
    column = foot + base + shaft + top + box + bolts
    neck = 2 * math.pi * 0.055 * 0.11 * 0.007 * rho_ac
    bowl = 0.07 * 0.006 * rho_ac
    struts = 6 * math.pi * 0.007 ** 2 * 0.43 * rho_ac
    rings = (2 * math.pi * 0.22 * 0.05 + math.pi * (EAVE_R ** 2 - 0.19 ** 2)) * 0.005 * rho_ac
    roof = math.pi * (EAVE_R + 0.085) * math.hypot(0.155, EAVE_R - 0.085) * 0.004 * rho_ac
    crown_cap_finial = 0.9
    glass = math.pi * (0.116 + 0.188) * math.hypot(0.42, 0.072) * 0.004 * rho_pc
    led = 1.3 + 0.3
    lantern = neck + bowl + struts + rings + roof + crown_cap_finial + glass + led
    return {"foundation": head + plate + nuts + lugs + pin, "hinge": knuckle, "column": column, "door": door, "lantern": lantern,
            "parts": {"head": head, "plate": plate, "nuts": nuts, "lugs": lugs, "pin": pin, "knuckle": knuckle, "foot": foot, "base": base, "door": door, "shaft": shaft, "top": top,
                      "neck": neck, "bowl": bowl, "struts": struts, "rings": rings, "roof": roof, "glass": glass, "led": led}}


MASS = mass_bones()


def rig():
    m = lambda kg, cx, cy, dist="point", src="authored-estimate", ref="catalogue-heritage-columns": {"kg": round(kg, 2), "comX": cx, "comY": cy, "distribution": dist, "source": src, "reference": ref}
    a_shaft = math.pi * (0.057 ** 2 - 0.051 ** 2)
    i_shaft = math.pi / 4 * (0.057 ** 4 - 0.051 ** 4)
    return {"kind": "mechanical", "root": "foundation", "bones": [
        {"id": "foundation", "parent": None, "title": T("Foundation head, anchor plate and hinge lugs", "Fundamentkopf, Ankerplatte und Gelenklaschen"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.2, "joint": {"type": "fixed"},
         "mass": m(MASS["foundation"], 0, 0.04, ref="en-1992-1-1")},
        {"id": "hinge", "parent": "foundation", "title": T("Hinged foot knuckle (tilting axis)", "Gelenkauge des Klappfusses (Kippachse)"), "x": PIVOT[0], "y": PIVOT[1], "angleDeg": 180, "lengthM": PIVOT[0], "joint": {"type": "revolute", "minDeg": -90, "maxDeg": 0},
         "mass": m(MASS["hinge"], 0, 0)},
        {"id": "column", "parent": "hinge", "title": T("Cast base, shaft and spigot", "Gusssockel, Schaft und Zapfen"), "x": PIVOT[0], "y": 0, "angleDeg": -90, "lengthM": 3.66, "joint": {"type": "fixed"},
         "mass": m(MASS["column"], 1.15, 0, "point"),
         "section": {"areaM2": r6(a_shaft), "secondMomentM4": float(f"{i_shaft:.4e}"), "extremeFibreM": 0.057, "material": "aluminium-6060-painted", "title": T("Aluminium tube 114 x 6 mm at the foot of the shaft (critical section)", "Aluminiumrohr 114 x 6 mm am Schaftfuss (massgebender Schnitt)")}},
        {"id": "door", "parent": "column", "title": T("Hook-in base door", "Eingehängte Masttür"), "x": DOOR_U, "y": 0.110, "angleDeg": 0, "lengthM": DOOR_H, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 100},
         "mass": m(MASS["door"], 0.15, -0.005, ref="en-40-6")},
        {"id": "lantern", "parent": "column", "title": T("Round heritage lantern with LED retrofit", "Runde Nostalgieleuchte mit LED-Nachrüstung"), "x": LANTERN_U, "y": 0, "angleDeg": 0, "lengthM": FIN_TOP, "joint": {"type": "fixed"},
         "mass": m(MASS["lantern"], 0.47, 0)},
    ]}




ANCHORS = [
    {"id": "foundation-support", "bone": "foundation", "x": 0, "y": 0, "kind": "support", "title": T("Precast foundation in the ground", "Betonfertigteilfundament im Boden"), "support": "fixed", "footprintLengthM": 0.4, "footprintWidthM": 0.4},
    {"id": "lantern-rest", "bone": "lantern", "x": 0.6275, "y": r6(-EAVE_R), "kind": "load", "title": T("Roof eave resting on the lawn mat", "Dachtraufe auf der Rasenmatte abgelegt")},
    {"id": "lantern-plume", "bone": "lantern", "x": 0.80, "y": 0, "kind": "emitter", "title": T("Warm air leaving the crown vents", "Warme Luft aus den Kronenschlitzen")},
]

STRUT_R = 0.008


def base(sid, kind, bone, material, role, tone, **extra):
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **extra}


def rev(sid, bone, profile, material, role, tone, axis=(1, 0, 0), origin=(0, 0, 0), **extra):
    return base(sid, "revolve", bone, material, role, tone, profile=profile, axis=list(axis), origin=list(origin), **extra)


def qx(deg):
    a = math.radians(deg) / 2
    return [r6(math.sin(a)), 0, 0, r6(math.cos(a))]


def partial(sid, bone, a0, a1, r, t, psi0, sweep, material, tone):
    return rev(sid, bone, loop([(a0, r), (a1, r), (a1, r + t), (a0, r + t)]), material, "texture", tone, sweepRad=r6(math.radians(sweep)), transform={"rotation": qx(psi0 - 90)}, texture=True)


def struts(sid, r, n_scale_tone=0.3):
    u0, u1 = 0.172, 0.588
    return base(sid, "capsule", "lantern", "cast-aluminium-painted", "body", n_scale_tone, p0=[u0, r6(glass_r(u0) + r), 0.0], p1=[u1, r6(glass_r(u1) + r), 0.0], r0=r, r1=r,
                repeat={"count": 6, "rotationAxis": [1, 0, 0], "rotationRad": r6(math.pi / 3), "origin": [0, 0, 0]}, slender=True)


def shape(sid, bone, role, z, tone, geom, region=None, regions=None, material=None, filled=True, hatch=None, pen=None, solid=None):
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, **geom}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen:
        s["penMm"] = pen
    if solid:
        s["solid"] = solid
    return s


def poly(points, closed=True):
    return {"kind": "poly", "points": points, "closed": closed}


def rect(x, y, w, h, r=None):
    g = {"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}
    if r:
        g["r"] = r
    return g


def ell(cx, cy, rx, ry):
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rx), "ry": r6(ry)}


def cap(x0, y0, x1, y1, r):
    return {"kind": "capsule", "x0": r6(x0), "y0": r6(y0), "x1": r6(x1), "y1": r6(y1), "r0": r6(r), "r1": r6(r)}


class Views:
    """🗂️ Collects the shapes of the three views of one scale in drawing order."""

    def __init__(self):
        self.v = {"elevation": [], "front": [], "plan": []}
        self.z = 0.0

    def add(self, view, s):
        self.z += 0.01
        s["z"] = round(self.z, 3)
        self.v[view].append(s)

    def revolved(self, sid, bone, prof, role, tone, region=None, regions=None, material=None, views=("elevation", "front", "plan"), pen=None):
        out = outline(prof)
        for view in views:
            sol = None if view == "elevation" else {"kind": "revolved", "axisBone": bone, "axis": "u", "axisX": 0, "mirror": True}
            self.add(view, shape(sid, bone, role, 0, tone, poly(out), region, regions, material, pen=pen, solid=sol))


def column_profile(parts):
    out = []
    for p in parts:
        for q in p:
            if not out or (abs(out[-1][0] - q[0]) > 1e-9 or abs(out[-1][1] - q[1]) > 1e-9):
                out.append(q)
    return out


FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
WALL = {50: 0.010, 100: 0.015, 200: 0.030}
LY = COLUMN_Y + LANTERN_U
HEAD_WU = [(-0.2, -0.2), (0.2, -0.2), (0.2, 0.2), (-0.16, 0.2), (-0.2, 0.16)]

P = {
    "plinth100": [(0.0, 0.125), (0.05, 0.125), (0.0725, 0.130), (0.095, 0.123), (0.11, 0.1135), (0.14, 0.110)],
    "lower200": [(-0.02, 0.145), (0.0, 0.145), (0.0, 0.125), (0.05, 0.125), (0.0725, 0.130), (0.095, 0.123), (0.14, 0.110)],
    "capital100": [(0.70, 0.110), (0.70, 0.122), (0.73, 0.122), (0.73, 0.105), (0.76, 0.105), (0.96, 0.068), (0.96, 0.075), (0.99, 0.075)],
    "capital200": [(0.70, 0.110), (0.70, 0.122), (0.73, 0.122), (0.76, 0.105), (0.96, 0.068), (0.99, 0.066)],
    "top100": [(3.50, 0.050), (3.52, 0.056), (3.54, 0.050), (3.54, 0.046), (3.56, 0.046), (3.56, 0.038), (3.66, 0.038)],
    "shafttop200": [(0.99, 0.057), (3.50, 0.044), (3.50, 0.056), (3.54, 0.056), (3.54, 0.046), (3.56, 0.046), (3.56, 0.038), (3.66, 0.038)],
    "column500": [(-0.02, 0.145), (0.0, 0.145), (0.06, 0.13), (0.14, 0.112), (0.70, 0.112), (0.73, 0.12), (0.76, 0.105), (0.96, 0.068), (0.99, 0.06), (3.50, 0.046), (3.54, 0.054), (3.56, 0.05), (3.56, 0.04), (3.66, 0.04)],
    "finial100": [(0.856, 0.018), (0.87, 0.018), (0.875, 0.028), (0.885, 0.034), (0.90, 0.036), (0.915, 0.034), (0.925, 0.028), (0.93, 0.018), (0.94, 0.014), (FIN_TOP, 0.008)],
    "lower200l": [(0.0, 0.058), (0.02, 0.052), (0.11, 0.052), (0.15, 0.125), (0.15, 0.132), (0.17, 0.132)],
    "lower500": [(0.0, 0.058), (0.11, 0.055), (0.15, 0.125), (0.17, 0.132)],
    "roof200": [(0.59, 0.200), (0.615, 0.200), (0.615, EAVE_R), (0.64, EAVE_R), (0.67, 0.20), (0.71, 0.15), (0.75, 0.102), (0.77, 0.085), (0.77, 0.075), (0.84, 0.070), (0.84, 0.088), (0.856, 0.088)],
    "finial200": [(0.856, 0.03), (0.87, 0.03), (0.885, 0.036), (0.905, 0.036), (0.925, 0.03), (0.94, 0.022), (FIN_TOP, 0.016)],
    "roof500": [(0.59, 0.200), (0.615, EAVE_R), (0.64, EAVE_R), (0.70, 0.17), (0.77, 0.085), (0.84, 0.085), (0.86, 0.06), (0.90, 0.045), (FIN_TOP, 0.04)],
    "led200": [(0.17, 0.016), (0.22, 0.016), (0.22, 0.045), (0.42, 0.045), (0.44, 0.025)],
    "led500": [(0.17, 0.04), (0.22, 0.045), (0.42, 0.045), (0.44, 0.03)],
}


def body_list(den):
    """🧱️ The body solids of one scale, each its own model for the smallest feature of that scale."""
    B = []
    w = WALL.get(den)
    if den < 500:
        B.append(base("foundation-head", "extrude", "foundation", "concrete-weathered", "body", 0.62, profile=pts(HEAD_WU), plane="zx", **{"from": 0.0, "to": 0.06}))
        B.append(base("anchor-plate", "box", "foundation", "galv-steel-weathered", "body", 0.45, size=[0.34, 0.015, 0.34], transform={"translation": [0, 0.0675, 0]}, sheet=True))
    else:
        B.append(base("foundation-head", "box", "foundation", "concrete-weathered", "body", 0.6, size=[0.4, 0.075, 0.4], transform={"translation": [0, 0.0375, 0]}))
    if den == 50:
        B.append(base("anchor-nut", "extrude", "foundation", "galv-steel-weathered", "detail", 0.35, profile=hexagon(0, 0, 0.024), plane="zx", **{"from": 0.075, "to": 0.088}, transform={"translation": [0.125, 0, 0.125]}, repeat={"count": 4, "rotationAxis": [0, 1, 0], "rotationRad": r6(math.pi / 2)}))
        B.append(rev("anchor-bolt", "foundation", pts([(0.088, 0), (0.088, 0.008), (0.10, 0.008), (0.10, 0)]), "galv-steel-weathered", "detail", 0.4, axis=(0, 1, 0), origin=(0.125, 0, 0.125), repeat={"count": 4, "rotationAxis": [0, 1, 0], "rotationRad": r6(math.pi / 2)}))
    if den <= 100:
        t = 0.012 if den == 50 else 0.015
        lug = [(0.135, 0.075), (0.165, 0.075), (0.165, 0.095)] + [(0.15 + 0.015 * math.cos(math.radians(a)), 0.095 + 0.015 * math.sin(math.radians(a))) for a in range(15, 180, 15)] + [(0.135, 0.095)]
        B.append(base("hinge-lug", "extrude", "foundation", "galv-steel-weathered", "body", 0.45, profile=pts(lug), plane="xy", **{"from": 0.055, "to": r6(0.055 + t)}, repeat={"count": 2, "translation": [0, 0, r6(-0.11 - t)]}))
        B.append(base("hinge-pin", "capsule", "foundation", "galv-steel-weathered", "detail", 0.4, p0=[PIVOT[0], PIVOT[1], -0.072], p1=[PIVOT[0], PIVOT[1], 0.072], r0=0.008, r1=0.008))
        B.append(base("hinge-knuckle", "extrude", "hinge", "cast-aluminium-painted", "body", 0.28, profile=circle(0, 0, 0.016), plane="xy", **{"from": -0.052, "to": 0.052}))
    elif den == 200:
        B.append(base("hinge-knuckle", "extrude", "hinge", "cast-aluminium-painted", "body", 0.28, profile=circle(0, 0, 0.016), plane="xy", **{"from": -0.072, "to": 0.072}))
    if den <= 100:
        B.append(rev("foot-ring", "column", filled(COLUMN["foot-ring"]), "cast-aluminium-painted", "body", 0.3))
        B.append(rev("plinth", "column", filled(COLUMN["plinth"] if den == 50 else P["plinth100"]), "cast-aluminium-painted", "body", 0.28))
        B.append(rev("base-shell-low", "column", loop([(0.14, 0.110 - w), (0.22, 0.110 - w), (0.22, 0.110), (0.14, 0.110)]), "cast-aluminium-painted", "body", 0.28))
        B.append(base("base-shell-door", "extrude", "column", "cast-aluminium-painted", "body", 0.28, profile=shell_c(w), plane="yz", **{"from": DOOR_U, "to": DOOR_U + DOOR_H}))
        B.append(rev("base-shell-up", "column", loop([(0.52, 0.110 - w), (0.70, 0.110 - w), (0.70, 0.110), (0.52, 0.110)]), "cast-aluminium-painted", "body", 0.28))
        B.append(rev("capital", "column", filled(COLUMN["capital"] if den == 50 else P["capital100"]), "cast-aluminium-painted", "body", 0.27))
        B.append(rev("shaft", "column", filled(COLUMN["shaft"]), "aluminium-6060-painted", "body", 0.3))
        B.append(rev("column-top", "column", filled(COLUMN["top"] if den == 50 else P["top100"]), "aluminium-6060-painted", "body", 0.27))
        B.append(base("connection-box", "box", "column", "polyamide-grey", "detail", 0.6, size=[0.18, 0.06, 0.06], transform={"translation": [0.37, -0.065, 0]}))
        B.append(base("door", "extrude", "door", "cast-aluminium-painted", "body", 0.3, profile=door_band(w), plane="yz", **{"from": 0.0, "to": DOOR_H}))
    elif den == 200:
        B.append(rev("plinth", "column", filled(P["lower200"]), "cast-aluminium-painted", "body", 0.28))
        B.append(rev("base-shell", "column", loop([(0.14, 0.110 - w), (0.70, 0.110 - w), (0.70, 0.110), (0.14, 0.110)]), "cast-aluminium-painted", "body", 0.28))
        B.append(rev("capital", "column", filled(P["capital200"]), "cast-aluminium-painted", "body", 0.27))
        B.append(rev("shaft", "column", filled(P["shafttop200"]), "aluminium-6060-painted", "body", 0.3))
    else:
        B.append(rev("column", "column", filled(P["column500"]), "aluminium-6060-painted", "body", 0.3))
    if den == 50:
        B.append(base("lock-bolt", "extrude", "column", "galv-steel-weathered", "detail", 0.4, profile=hexagon(0.128, 0.04, 0.017), plane="yz", **{"from": 0.0, "to": 0.008}, repeat={"count": 2, "translation": [0, 0, -0.08]}))
        B.append(base("door-lock", "extrude", "door", "galv-steel-weathered", "detail", 0.4, profile=circle(0, 0.27, 0.008, 12), plane="zx", **{"from": 0.0, "to": 0.008}))
    if den <= 100:
        B.append(rev("lantern-neck", "lantern", filled(LANTERN["neck"]), "cast-aluminium-painted", "body", 0.28))
        B.append(rev("lantern-bowl", "lantern", filled(LANTERN["bowl"]), "cast-aluminium-painted", "body", 0.27))
        B.append(rev("glass-aged", "lantern", loop(GLASS), "polycarbonate-yellowed", "glazing", 0.8, sweepRad=r6(math.radians(300)), transform={"rotation": qx(300 - 90)}))
        B.append(rev("glass-new", "lantern", loop(GLASS), "polycarbonate-new", "glazing", 0.93, sweepRad=r6(math.radians(60)), transform={"rotation": qx(240 - 90)}))
        B.append(struts("struts", STRUT_R, 0.28))
        B.append(rev("top-ring", "lantern", filled(LANTERN["top-ring"]), "cast-aluminium-painted", "body", 0.27))
        B.append(rev("roof", "lantern", filled(LANTERN["roof"]), "cast-aluminium-painted", "body", 0.22))
        B.append(rev("crown", "lantern", filled(LANTERN["crown"]), "cast-aluminium-painted", "body", 0.24))
        B.append(rev("cap", "lantern", filled(LANTERN["cap"]), "cast-aluminium-painted", "body", 0.22))
        B.append(rev("finial", "lantern", filled(LANTERN["finial"] if den == 50 else P["finial100"]), "cast-aluminium-painted", "body", 0.2))
        B.append(rev("led-bracket", "lantern", filled(LANTERN["led-bracket"]), "led-heatsink", "detail", 0.35))
        B.append(rev("led-module", "lantern", filled(LANTERN["led-module"]), "led-heatsink", "detail", 0.3))
    elif den == 200:
        B.append(rev("lantern-neck", "lantern", filled(P["lower200l"]), "cast-aluminium-painted", "body", 0.28))
        B.append(rev("glass-aged", "lantern", loop(GLASS), "polycarbonate-yellowed", "glazing", 0.82))
        B.append(rev("roof", "lantern", filled(P["roof200"]), "cast-aluminium-painted", "body", 0.22))
        B.append(rev("finial", "lantern", filled(P["finial200"]), "cast-aluminium-painted", "body", 0.2))
        B.append(rev("led-module", "lantern", filled(P["led200"]), "led-heatsink", "detail", 0.3))
    else:
        B.append(rev("lantern-neck", "lantern", filled(P["lower500"]), "cast-aluminium-painted", "body", 0.28))
        B.append(rev("glass-aged", "lantern", loop(GLASS), "polycarbonate-yellowed", "glazing", 0.82))
        B.append(rev("roof", "lantern", filled(P["roof500"]), "cast-aluminium-painted", "body", 0.22))
        B.append(rev("led-module", "lantern", filled(P["led500"]), "led-heatsink", "detail", 0.3))
    if den <= 100:
        B.append(partial("graffiti", "column", 0.30, 0.47, 0.110, 0.0015, 200, 50, "spray-paint", 0.12))
        B.append(partial("algae", "column", 0.0, 0.26, 0.1305, 0.0015, -50, 100, "algae-film", 0.45))
        B.append(partial("sticker", "column", 1.30, 1.40, 0.0535, 0.0015, 205, 40, "sticker-paper", 0.8))
    return B


def views(den):
    """✏️ The three hand-drawn views of one scale, drawn to the same numbers as its solids."""
    V = Views()
    E, F, PL = "elevation", "front", "plan"
    w = WALL.get(den)
    hatch = "concrete-stipple" if den <= 100 else None
    if den <= 100:
        for view, g in ((E, rect(-0.2, 0, 0.4, 0.06)), (F, rect(-0.2, 0, 0.4, 0.06)), (PL, poly(pts([(u, ww) for ww, u in HEAD_WU])))):
            V.add(view, shape("foundation-head", "foundation", "body", 0, 0.62, g, "foundation", material="concrete-weathered", hatch=hatch))
        V.add(E, shape("foundation-chip", "foundation", "detail", 0, 0.5, rect(0.16, 0, 0.04, 0.06), "foundation", material="concrete-weathered"))
        V.add(F, shape("foundation-chip", "foundation", "detail", 0, 0.5, rect(-0.2, 0, 0.04, 0.06), "foundation", material="concrete-weathered"))
        for view, g in ((E, rect(-0.17, 0.06, 0.34, 0.015)), (F, rect(-0.17, 0.06, 0.34, 0.015)), (PL, rect(-0.17, -0.17, 0.34, 0.34))):
            V.add(view, shape("anchor-plate", "foundation", "body", 0, 0.45, g, "anchorage", material="galv-steel-weathered"))
    else:
        hh = 0.075
        for view, g in ((E, rect(-0.2, 0, 0.4, hh)), (F, rect(-0.2, 0, 0.4, hh)), (PL, poly(pts([(u, ww) for ww, u in HEAD_WU]) if den == 200 else pts([(-0.2, -0.2), (0.2, -0.2), (0.2, 0.2), (-0.2, 0.2)])))):
            V.add(view, shape("foundation-head", "foundation", "body", 0, 0.6, g, regions=["foundation", "anchorage"], material="concrete-weathered"))
        if den == 200:
            V.add(E, shape("foundation-chip", "foundation", "detail", 0, 0.5, rect(0.16, 0, 0.04, 0.06), "foundation", material="concrete-weathered"))
            V.add(F, shape("foundation-chip", "foundation", "detail", 0, 0.5, rect(-0.2, 0, 0.04, 0.06), "foundation", material="concrete-weathered"))
    if den == 50:
        for x in (-0.125, 0.125):
            V.add(E, shape(f"anchor-nut-{'b' if x < 0 else 'f'}", "foundation", "detail", 0, 0.35, rect(x - 0.01386, 0.075, 0.02771, 0.013), "anchorage", material="galv-steel-weathered"))
            V.add(E, shape(f"anchor-bolt-{'b' if x < 0 else 'f'}", "foundation", "detail", 0, 0.4, rect(x - 0.008, 0.088, 0.016, 0.012), "anchorage", material="galv-steel-weathered"))
            V.add(F, shape(f"anchor-nut-{'r' if x < 0 else 'l'}", "foundation", "detail", 0, 0.35, rect(x - 0.012, 0.075, 0.024, 0.013), "anchorage", material="galv-steel-weathered"))
            V.add(F, shape(f"anchor-bolt-{'r' if x < 0 else 'l'}", "foundation", "detail", 0, 0.4, rect(x - 0.008, 0.088, 0.016, 0.012), "anchorage", material="galv-steel-weathered"))
        for i, (x, z) in enumerate([(0.125, 0.125), (-0.125, 0.125), (-0.125, -0.125), (0.125, -0.125)]):
            V.add(PL, shape(f"anchor-nut-{i}", "foundation", "detail", 0, 0.35, poly(hexagon(x, z, 0.024)), "anchorage", material="galv-steel-weathered"))
        V.add(E, shape("rust-streak", "foundation", "texture", 0, 0.35, poly(pts([(0.121, 0.059), (0.119, 0.042), (0.121, 0.028)]), False), "foundation", material="galv-steel-weathered", filled=False))
        V.add(F, shape("rust-streak", "foundation", "texture", 0, 0.35, poly(pts([(-0.121, 0.059), (-0.119, 0.04), (-0.121, 0.03)]), False), "foundation", material="galv-steel-weathered", filled=False))
    if den <= 100:
        t = 0.012 if den == 50 else 0.015
        lug = pts([(0.135, 0.075), (0.165, 0.075), (0.165, 0.095)] + [(0.15 + 0.015 * math.cos(math.radians(a)), 0.095 + 0.015 * math.sin(math.radians(a))) for a in range(15, 180, 15)] + [(0.135, 0.095)])
        V.add(E, shape("hinge-lug", "foundation", "body", 0, 0.45, poly(lug), "anchorage", material="galv-steel-weathered"))
        V.add(F, shape("hinge-lug-l", "foundation", "body", 0, 0.45, rect(0.055, 0.075, t, 0.035), "anchorage", material="galv-steel-weathered"))
        V.add(F, shape("hinge-lug-r", "foundation", "body", 0, 0.45, rect(-0.055 - t, 0.075, t, 0.035), "anchorage", material="galv-steel-weathered"))
        V.add(PL, shape("hinge-lug-l", "foundation", "body", 0, 0.45, rect(0.135, 0.055, 0.03, t), "anchorage", material="galv-steel-weathered"))
        V.add(PL, shape("hinge-lug-r", "foundation", "body", 0, 0.45, rect(0.135, -0.055 - t, 0.03, t), "anchorage", material="galv-steel-weathered"))
    if den <= 200:
        kz = 0.052 if den <= 100 else 0.072
        V.add(E, shape("hinge-knuckle", "hinge", "body", 0, 0.28, ell(0, 0, 0.016, 0.016), "column-base", material="cast-aluminium-painted"))
        V.add(F, shape("hinge-knuckle", "hinge", "body", 0, 0.28, rect(-kz, 0.079, 2 * kz, 0.032), "column-base", material="cast-aluminium-painted"))
        V.add(PL, shape("hinge-knuckle", "hinge", "body", 0, 0.28, rect(0.134, -kz, 0.032, 2 * kz), "column-base", material="cast-aluminium-painted"))
    if den <= 100:
        V.add(E, shape("hinge-pin", "foundation", "detail", 0, 0.4, ell(PIVOT[0], PIVOT[1], 0.008, 0.008), "anchorage", material="galv-steel-weathered"))
        V.add(F, shape("hinge-pin", "foundation", "detail", 0, 0.4, cap(-0.072, PIVOT[1], 0.072, PIVOT[1], 0.008), "anchorage", material="galv-steel-weathered"))
        V.add(PL, shape("hinge-pin", "foundation", "detail", 0, 0.4, cap(PIVOT[0], -0.072, PIVOT[0], 0.072, 0.008), "anchorage", material="galv-steel-weathered"))
    cb, cs = "column-base", "column-shaft"
    if den <= 100:
        V.revolved("foot-ring", "column", COLUMN["foot-ring"], "body", 0.3, cb, material="cast-aluminium-painted")
        V.revolved("plinth", "column", COLUMN["plinth"] if den == 50 else P["plinth100"], "body", 0.28, cb, material="cast-aluminium-painted")
        V.revolved("base-body", "column", [(0.14, 0.110), (0.70, 0.110)], "body", 0.28, cb, material="cast-aluminium-painted")
        V.revolved("capital", "column", COLUMN["capital"] if den == 50 else P["capital100"], "body", 0.27, cb, material="cast-aluminium-painted")
        V.revolved("shaft", "column", COLUMN["shaft"], "body", 0.3, cs, material="aluminium-6060-painted")
        V.revolved("column-top", "column", COLUMN["top"] if den == 50 else P["top100"], "body", 0.27, cs, material="aluminium-6060-painted")
        door = {"kind": "extruded", "sweep": "u", "z0": 0.0, "z1": DOOR_H}
        for view in (E, F, PL):
            V.add(view, shape("door", "door", "detail", 0, 0.28, poly(door_band(w)), cb, material="cast-aluminium-painted", solid=door))
    elif den == 200:
        V.revolved("plinth", "column", P["lower200"], "body", 0.28, cb, material="cast-aluminium-painted")
        V.revolved("base-body", "column", [(0.14, 0.110), (0.70, 0.110)], "body", 0.28, cb, material="cast-aluminium-painted")
        V.revolved("capital", "column", P["capital200"], "body", 0.27, cb, material="cast-aluminium-painted")
        V.revolved("shaft", "column", P["shafttop200"], "body", 0.3, cs, material="aluminium-6060-painted")
        V.add(E, shape("door-joint", "column", "detail", 0, 0.3, poly(pts([(DOOR_U, 0.11), (DOOR_U + DOOR_H, 0.11)]), False), cb, material="cast-aluminium-painted", filled=False))
    else:
        V.revolved("column", "column", P["column500"], "body", 0.3, regions=[cb, cs], material="aluminium-6060-painted")
    if den == 50:
        V.add(E, shape("lock-bolt", "column", "detail", 0, 0.4, rect(0, 0.1182, 0.008, 0.0196), cb, material="galv-steel-weathered"))
        V.add(E, shape("door-lock", "door", "detail", 0, 0.4, rect(0.262, 0.0, 0.016, 0.008), cb, material="galv-steel-weathered"))
    lf, lr, lg, lm = "lantern-frame", "lantern-roof", "lantern-glazing", "led-module"
    if den <= 100:
        V.revolved("lantern-neck", "lantern", LANTERN["neck"], "body", 0.28, lf, material="cast-aluminium-painted")
        V.revolved("lantern-bowl", "lantern", LANTERN["bowl"], "body", 0.27, lf, material="cast-aluminium-painted")
        V.add(E, shape("glass-aged", "lantern", "glazing", 0, 0.8, poly(outline([(0.17, 0.118), (0.59, 0.190)])), lg, material="polycarbonate-yellowed"))
        for view in (F, PL):
            V.add(view, shape("glass-aged", "lantern", "glazing", 0, 0.8, poly(outline([(0.17, 0.118), (0.59, 0.190)])), lg, material="polycarbonate-yellowed", solid={"kind": "revolved", "axisBone": "lantern", "axis": "u", "axisX": 0, "mirror": True}))
        V.revolved("led-bracket", "lantern", LANTERN["led-bracket"], "detail", 0.35, lm, material="led-heatsink", views=(E, F))
        V.revolved("led-module", "lantern", LANTERN["led-module"], "detail", 0.3, lm, material="led-heatsink", views=(E, F))
        V.add(E, shape("glass-new", "lantern", "glazing", 0, 0.93, poly(pts([(0.17, -0.059), (0.59, -0.095), (0.59, 0.095), (0.17, 0.059)])), lg, material="polycarbonate-new"))
        V.add(F, shape("glass-new", "lantern", "glazing", 0, 0.93, poly(pts([(-0.118, LY + 0.17), (-0.1022, LY + 0.17), (-0.1645, LY + 0.59), (-0.190, LY + 0.59)])), lg, material="polycarbonate-new"))
        for k, psi in enumerate(STRUT_ANGLES):
            c, s_ = math.cos(math.radians(psi)), math.sin(math.radians(psi))
            r0, r1 = glass_r(0.172) + STRUT_R, glass_r(0.588) + STRUT_R
            V.add(E, shape(f"strut-{k}", "lantern", "detail", 0, 0.28, cap(0.172, r0 * c, 0.588, r1 * c, STRUT_R), lf, material="cast-aluminium-painted"))
            V.add(F, shape(f"strut-{k}", "lantern", "detail", 0, 0.28, cap(r0 * s_, LY + 0.172, r1 * s_, LY + 0.588, STRUT_R), lf, material="cast-aluminium-painted"))
        V.revolved("top-ring", "lantern", LANTERN["top-ring"], "body", 0.27, lf, material="cast-aluminium-painted")
        V.revolved("roof", "lantern", LANTERN["roof"], "body", 0.22, lr, material="cast-aluminium-painted")
        V.revolved("crown", "lantern", LANTERN["crown"], "body", 0.24, lr, material="cast-aluminium-painted")
        V.revolved("cap", "lantern", LANTERN["cap"], "body", 0.22, lr, material="cast-aluminium-painted")
        V.revolved("finial", "lantern", LANTERN["finial"] if den == 50 else P["finial100"], "body", 0.2, lr, material="cast-aluminium-painted")
    elif den == 200:
        V.revolved("lantern-neck", "lantern", P["lower200l"], "body", 0.28, lf, material="cast-aluminium-painted")
        V.add(E, shape("glass-aged", "lantern", "glazing", 0, 0.82, poly(outline([(0.17, 0.118), (0.59, 0.190)])), lg, material="polycarbonate-yellowed"))
        for view in (F, PL):
            V.add(view, shape("glass-aged", "lantern", "glazing", 0, 0.82, poly(outline([(0.17, 0.118), (0.59, 0.190)])), lg, material="polycarbonate-yellowed", solid={"kind": "revolved", "axisBone": "lantern", "axis": "u", "axisX": 0, "mirror": True}))
        V.revolved("led-module", "lantern", P["led200"], "detail", 0.3, lm, material="led-heatsink", views=(E, F))
        V.revolved("roof", "lantern", P["roof200"], "body", 0.22, regions=[lr, lf], material="cast-aluminium-painted")
        V.revolved("finial", "lantern", P["finial200"], "body", 0.2, lr, material="cast-aluminium-painted")
    else:
        V.revolved("lantern-neck", "lantern", P["lower500"], "body", 0.28, lf, material="cast-aluminium-painted")
        V.revolved("glass-aged", "lantern", [(0.17, 0.118), (0.59, 0.190)], "body", 0.75, regions=[lg, lm], material="polycarbonate-yellowed")
        V.revolved("roof", "lantern", P["roof500"], "body", 0.22, regions=[lr, lf], material="cast-aluminium-painted")
    if den <= 100:
        V.add(E, shape("graffiti", "column", "texture", 0, 0.12, poly(pts([(0.31, -0.096), (0.40, -0.082), (0.33, -0.07), (0.44, -0.058), (0.35, -0.047), (0.46, -0.04)]), False), cb, material="spray-paint", filled=False))
        V.add(F, shape("graffiti", "column", "texture", 0, 0.12, poly(pts([(-0.096, COLUMN_Y + 0.31), (-0.082, COLUMN_Y + 0.40), (-0.07, COLUMN_Y + 0.33), (-0.058, COLUMN_Y + 0.44), (-0.047, COLUMN_Y + 0.35), (-0.04, COLUMN_Y + 0.46)]), False), cb, material="spray-paint", filled=False))
        V.add(E, shape("algae", "column", "texture", 0, 0.45, poly(pts([(0.0, 0.07), (0.0, 0.124), (0.05, 0.124), (0.0725, 0.129), (0.095, 0.122), (0.14, 0.109), (0.26, 0.109), (0.22, 0.092), (0.12, 0.078), (0.05, 0.072)])), cb, material="algae-film", filled=False, hatch="algae-stipple"))
        V.add(E, shape("sticker", "column", "texture", 0, 0.8, poly(pts([(1.30, -0.036), (1.40, -0.034), (1.40, 0.004), (1.36, 0.006), (1.33, -0.004), (1.30, 0.002)])), cs, material="sticker-paper", filled=False))
        V.add(F, shape("sticker", "column", "texture", 0, 0.8, poly(pts([(-0.046, COLUMN_Y + 1.30), (-0.027, COLUMN_Y + 1.30), (-0.027, COLUMN_Y + 1.36), (-0.034, COLUMN_Y + 1.40), (-0.046, COLUMN_Y + 1.40)])), cs, material="sticker-paper", filled=False))
        for k, (a, b) in enumerate([((0.86, -0.03), (0.95, -0.018)), ((0.98, -0.034), (1.08, -0.02)), ((1.10, -0.012), (1.17, 0.002))]):
            V.add(E, shape(f"lock-scratch-{k}", "column", "texture", 0, 0.6, poly(pts([a, b]), False), cs, material="aluminium-bare-oxidised", filled=False))
        V.add(E, shape("white-bloom", "column", "texture", 0, 0.85, poly(pts([(-0.017, -0.138), (-0.006, -0.128), (-0.015, -0.115), (-0.004, -0.104)]), False), cb, material="aluminium-bare-oxidised", filled=False))
    if den == 50:
        for k, chip in enumerate([[(0.06, -0.1285), (0.075, -0.1293), (0.08, -0.117), (0.064, -0.1155)], [(-0.019, -0.1445), (-0.004, -0.1445), (-0.004, -0.133), (-0.017, -0.1315)], [(0.30, -0.1098), (0.315, -0.1098), (0.318, -0.098), (0.302, -0.0965)]]):
            V.add(E, shape(f"paint-chip-{k}", "column", "detail", 0, 0.82, poly(pts(chip)), cb, material="aluminium-bare-oxidised"))
        V.add(E, shape("eave-chip", "lantern", "detail", 0, 0.82, poly(pts([(0.616, -0.244), (0.639, -0.244), (0.639, -0.228), (0.618, -0.226)])), lr, material="aluminium-bare-oxidised"))
        V.add(PL, shape("eave-chip", "lantern", "detail", 0, 0.82, poly(pts([(0.226, -0.012), (0.244, -0.012), (0.244, 0.006), (0.228, 0.006)])), lr, material="aluminium-bare-oxidised"))
        for k, blob in enumerate([[(0.66, 0.085), (0.674, 0.09), (0.672, 0.106), (0.658, 0.1)], [(0.695, 0.02), (0.71, 0.026), (0.706, 0.04), (0.693, 0.034)]]):
            V.add(E, shape(f"dropping-{k}", "lantern", "detail", 0, 0.95, poly(pts(blob)), lr, material="cast-aluminium-painted"))
        V.add(PL, shape("dropping-0", "lantern", "detail", 0, 0.95, poly(pts([(-0.106, 0.03), (-0.088, 0.034), (-0.09, 0.05), (-0.104, 0.048)])), lr, material="cast-aluminium-painted"))
        for k, u in enumerate([0.79, 0.805, 0.82]):
            V.add(E, shape(f"crown-slot-{k}", "lantern", "detail", 0, 0.1, rect(u, -0.05, 0.008, 0.1), lr, material="cast-aluminium-painted"))
        for k, u in enumerate([0.26, 0.30, 0.34, 0.38]):
            V.add(E, shape(f"led-fin-{k}", "lantern", "texture", 0, 0.2, poly(pts([(u, -0.045), (u, 0.045)]), False), lm, material="led-heatsink", filled=False))
        V.add(E, shape("grub-screw", "lantern", "detail", 0, 0.4, ell(0.06, -0.052, 0.005, 0.005), lf, material="galv-steel-weathered"))
    return V.v


def ref(sid, of, **label):
    return {"id": sid, "of": of, **label}


def strut_rods(sid, label):
    s = struts(sid, STRUT_R, 0.28)
    s.update(label)
    return s


def thermal_list(den):
    """🌡️ The thermal model of one scale as solids, one region per surface, hollow parts kept hollow."""
    th = []
    fd, an, cb, cs, lf, lr, lg, lm = "foundation", "anchorage", "column-base", "column-shaft", "lantern-frame", "lantern-roof", "lantern-glazing", "led-module"
    if den < 500:
        th += [ref("th-foundation", "foundation-head", region=fd, sectionFaces=["-v"]), ref("th-anchor-plate", "anchor-plate", region=an)]
    else:
        th.append(ref("th-foundation", "foundation-head", region=fd, sectionFaces=["-v"]))
        th.append(base("th-anchor-plate", "box", "foundation", "galv-steel-weathered", "body", 0.45, size=[0.34, 0.015, 0.34], transform={"translation": [0, 0.0825, 0]}, sheet=True, region=an))
    if den == 50:
        th += [ref("th-anchor-nut", "anchor-nut", region=an), ref("th-anchor-bolt", "anchor-bolt", region=an)]
    if den <= 100:
        th += [ref("th-hinge-lug", "hinge-lug", region=an), ref("th-hinge-pin", "hinge-pin", region=an), ref("th-hinge-knuckle", "hinge-knuckle", region=cb),
               ref("th-foot-ring", "foot-ring", region=cb), ref("th-shaft", "shaft", region=cs), ref("th-column-top", "column-top", region=cs)]
        if den == 50:
            th += [ref("th-plinth", "plinth", region=cb), rev("th-base", "column", filled([(0.14, 0.110), (0.70, 0.110)]), "cast-aluminium-painted", "body", 0.28, region=cb), ref("th-capital", "capital", region=cb)]
        else:
            th.append(rev("th-base", "column", filled(P["plinth100"] + [(0.70, 0.110)] + P["capital100"][1:]), "cast-aluminium-painted", "body", 0.28, region=cb))
        if den == 50:
            th.append(ref("th-lock-bolt", "lock-bolt", region=cb))
        th += [ref("th-lantern-neck", "lantern-neck", region=lf), ref("th-lantern-bowl", "lantern-bowl", region=lf), ref("th-struts", "struts", region=lf), ref("th-top-ring", "top-ring", region=lf),
               ref("th-glass-aged", "glass-aged", region=lg), ref("th-glass-new", "glass-new", region=lg),
               ref("th-roof", "roof", region=lr), ref("th-crown", "crown", region=lr), ref("th-cap", "cap", region=lr), ref("th-finial", "finial", region=lr),
               ref("th-led-bracket", "led-bracket", region=lm), ref("th-led-module", "led-module", region=lm)]
    elif den == 200:
        th += [ref("th-hinge-knuckle", "hinge-knuckle", region=cb), ref("th-plinth", "plinth", region=cb),
               rev("th-base", "column", filled([(0.14, 0.110), (0.70, 0.110)]), "cast-aluminium-painted", "body", 0.28, region=cb),
               ref("th-capital", "capital", region=cb), ref("th-shaft", "shaft", region=cs),
               ref("th-lantern-neck", "lantern-neck", region=lf), strut_rods("th-struts", {"region": lf}), ref("th-glass-aged", "glass-aged", region=lg),
               ref("th-roof", "roof", region=lr), ref("th-finial", "finial", region=lr), ref("th-led-module", "led-module", region=lm)]
    else:
        prof = P["column500"]
        th += [rev("th-base", "column", filled(prof[:9]), "cast-aluminium-painted", "body", 0.28, region=cb),
               rev("th-shaft", "column", filled(prof[8:]), "aluminium-6060-painted", "body", 0.3, region=cs),
               ref("th-lantern-neck", "lantern-neck", region=lf), strut_rods("th-struts", {"region": lf}), ref("th-glass-aged", "glass-aged", region=lg),
               ref("th-roof", "roof", region=lr), ref("th-led-module", "led-module", region=lm)]
    rc = {50: 0.0724, 100: 0.0724, 200: 0.0724, 500: 0.085}[den]
    th.append(rev("th-lantern-plume", "lantern", loop([(0.785, rc), (0.83, rc), (0.83, rc + 0.003), (0.785, rc + 0.003)]), "cast-aluminium-painted", "detail", 0.9, region=lr, tags=["lantern-plume"], texture=True))
    return th


COM = {"foundation": (0, 0.04), "hinge": (0, 0), "column": (1.17, 0), "door": (0.15, -0.008), "lantern": (0.47, 0)}


def structure_list(den):
    """🏗️ The structural idealisation of one scale: members along the bones, joints, the support and the masses."""
    st = []
    mem = lambda m, p="member", **k: {"member": m, "part": p, **k}
    if den < 500:
        st += [ref("member-foundation", "foundation-head", **mem("foundation")), ref("member-anchor-plate", "anchor-plate", **mem("foundation"))]
    else:
        st.append(ref("member-foundation", "foundation-head", **mem("foundation")))
    if den <= 100:
        st += [ref("member-hinge-lug", "hinge-lug", **mem("foundation")), ref("joint-hinge-pin", "hinge-pin", **mem("hinge", "joint")), ref("member-hinge-knuckle", "hinge-knuckle", **mem("hinge")),
               ref("member-foot-ring", "foot-ring", **mem("column")),
               rev("member-base", "column", filled((COLUMN["plinth"] + [(0.70, 0.110)] + COLUMN["capital"][1:]) if den == 50 else (P["plinth100"] + [(0.70, 0.110)] + P["capital100"][1:])), "cast-aluminium-painted", "body", 0.28, **mem("column")),
               ref("member-shaft", "shaft", **mem("column")), ref("joint-spigot", "column-top", **mem("lantern", "joint")), ref("member-door", "door", **mem("door")),
               ref("member-lantern-neck", "lantern-neck", **mem("lantern")), ref("member-lantern-bowl", "lantern-bowl", **mem("lantern")), ref("member-struts", "struts", **mem("lantern")),
               ref("member-top-ring", "top-ring", **mem("lantern")), ref("member-roof", "roof", **mem("lantern"))]
    elif den == 200:
        st += [ref("member-hinge-knuckle", "hinge-knuckle", **mem("hinge")),
               rev("member-base", "column", filled(P["lower200"] + [(0.70, 0.110)] + P["capital200"][1:]), "cast-aluminium-painted", "body", 0.28, **mem("column", members=["door"])),
               ref("member-shaft", "shaft", **mem("column")), ref("joint-spigot", "lantern-neck", **mem("lantern", "joint")),
               strut_rods("member-struts", mem("lantern")), ref("member-roof", "roof", **mem("lantern"))]
    else:
        st += [ref("member-column", "column", **mem("column", members=["hinge", "door"])),
               base("joint-hinge", "ellipsoid", "hinge", "cast-aluminium-painted", "body", 0.3, radii=[0.0375, 0.0375, 0.0375], **mem("hinge", "joint")),
               rev("member-lantern", "lantern", filled(P["lower500"] + [(0.59, 0.190)] + P["roof500"]), "cast-aluminium-painted", "body", 0.28, **mem("lantern"))]
    t = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[den]
    st.append(base("support-foundation", "box", "foundation", "concrete-weathered", "body", 0.1, size=[0.4, t, 0.4], transform={"translation": [0, t / 2, 0]}, **mem("foundation", "support", anchor="foundation-support")))
    rad = {50: 0.015, 100: 0.02, 200: 0.03, 500: 0.0375}[den]
    for bone in (["foundation", "hinge", "column", "door", "lantern"] if den <= 100 else ["foundation", "hinge", "column", "lantern"]):
        cx, cy = COM[bone]
        if den >= 200 and bone == "hinge":
            continue
        st.append(base(f"mass-{bone}", "ellipsoid", bone, "cast-aluminium-painted" if bone != "foundation" else "concrete-weathered", "body", 0.5, radii=[0.008 if bone == "door" else rad] * 3, transform={"translation": [cx, cy, 0]}, **mem(bone, "mass")))
    return st


AREA = {"foundation": 0.248, "anchorage": 0.264, "column-base": 0.72, "column-shaft": 0.83, "lantern-frame": 0.416, "lantern-roof": 0.40, "lantern-glazing": 0.38, "led-module": 0.075}


def layer(lid, en, de, kind, material, t, k, rho, cp, src="literature", refid="iso-10456-2007", active=None):
    d = active if active else t
    return {"id": lid, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": float(f"{t / k:.3g}"), "source": src, "heatCapacityJM2K": round(rho * cp * d), "reference": refid}


def region(rid, en, de, kind, material, geometry, lc, sky, lay, **extra):
    return {"id": rid, "title": T(en, de), "kind": kind, "model": extra.pop("model", "passive"), "material": material, "areaM2": AREA[rid], "characteristicLengthM": lc, "geometry": geometry, "skyViewFactor": sky,
            "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": extra.pop("source", "authored-estimate"), "reference": extra.pop("reference", "iso-10456-2007"), **extra}


def regions():
    return [
        region("foundation", "Precast concrete foundation head, weathered", "Kopf des Betonfertigteilfundaments, verwittert", "surface", "concrete-weathered", "vertical-plate", 0.4, 0.6,
               layer("concrete", "Concrete 60 mm out of the ground (thermally active 55 mm)", "Beton 60 mm über Gelände (thermisch aktiv 55 mm)", "masonry", "concrete-weathered", 0.06, 2.0, 2400, 1000, active=0.0547)),
        region("anchorage", "Galvanised anchor plate, nuts and hinge lugs", "Verzinkte Ankerplatte, Muttern und Gelenklaschen", "surface", "galv-steel-weathered", "horizontal-plate-up", 0.34, 0.8,
               layer("steel", "Galvanised steel plate 15 mm", "Verzinktes Stahlblech 15 mm", "metal", "galv-steel-weathered", 0.015, 50, 7850, 450)),
        region("column-base", "Cast base with door and hinged foot, painted", "Gusssockel mit Tür und Klappfuss, lackiert", "surface", "cast-aluminium-painted", "cylinder", 0.22, 0.5,
               layer("casting", "Cast aluminium wall 10 mm, chalked powder coat", "Aluminiumgusswand 10 mm, verkreidete Pulverbeschichtung", "metal", "cast-aluminium-painted", 0.01, 130, 2650, 900, refid="en-1706")),
        region("column-shaft", "Extruded aluminium shaft, painted", "Strangpress-Aluminiumschaft, lackiert", "surface", "aluminium-6060-painted", "cylinder", 0.1, 0.5,
               layer("tube", "Aluminium tube wall 6 mm, chalked powder coat", "Aluminiumrohrwand 6 mm, verkreidete Pulverbeschichtung", "metal", "aluminium-6060-painted", 0.006, 200, 2700, 900, refid="en-755-2")),
        region("lantern-frame", "Lantern neck, bowl, struts and rings", "Leuchtenhals, Boden, Streben und Ringe", "surface", "cast-aluminium-painted", "cylinder", 0.26, 0.5,
               layer("casting", "Cast aluminium 6 mm, chalked powder coat", "Aluminiumguss 6 mm, verkreidete Pulverbeschichtung", "metal", "cast-aluminium-painted", 0.006, 130, 2650, 900, refid="en-1706")),
        region("lantern-roof", "Lantern roof, crown and finial", "Leuchtendach, Krone und Spitze", "surface", "cast-aluminium-painted", "horizontal-plate-up", 0.49, 0.9,
               layer("casting", "Cast aluminium roof 4 mm, chalked powder coat with bird droppings", "Aluminiumgussdach 4 mm, verkreidete Pulverbeschichtung mit Vogelkot", "metal", "cast-aluminium-painted", 0.004, 130, 2650, 900, refid="en-1706")),
        region("lantern-glazing", "UV-yellowed polycarbonate glazing, one pane replaced", "UV-vergilbte Polycarbonat-Verglasung, eine Scheibe ersetzt", "glazing", "polycarbonate-yellowed", "vertical-plate", 0.42, 0.5,
               layer("pane", "Polycarbonate 4 mm", "Polycarbonat 4 mm", "polymer", "polycarbonate-yellowed", 0.004, 0.2, 1200, 1200)),
        region("led-module", "LED retrofit module in the lantern", "LED-Nachrüstmodul in der Leuchte", "lamp", "led-heatsink", "cylinder", 0.09, 0.3,
               layer("heatsink", "Die-cast aluminium heat sink 3 mm", "Kühlkörper aus Aluminiumdruckguss 3 mm", "metal", "led-heatsink", 0.003, 150, 2700, 900, src="authored-estimate", refid="schubert-2006"),
               model="operating", operating={"key": "led", "heatW": 20}, ratedPowerW=30, ratedPowerBasis="electrical-input", reference="schubert-2006"),
    ]


RIDS = list(AREA)
SUN_DAY = {"foundation": 0.6, "anchorage": 0.6, "column-base": 0.45, "column-shaft": 0.45, "lantern-frame": 0.45, "lantern-roof": 0.7, "lantern-glazing": 0.45, "led-module": 0.25}
SUN_TIPPED = {"foundation": 0.6, "anchorage": 0.6, "column-base": 0.5, "column-shaft": 0.5, "lantern-frame": 0.5, "lantern-roof": 0.45, "lantern-glazing": 0.5, "led-module": 0.25}


def binding(sun, led):
    return {"regions": {r: {"exposedFraction": 1.0, "sunlitFraction": sun.get(r, 0.0)} for r in RIDS}, "emitters": {"lantern-plume": led}, "operating": {"led": led}}


WIND = [
    {"id": "wind-base", "kind": "wind", "title": T("Gust on the cast base", "Böe auf den Gusssockel"), "distributed": {"bone": "column", "fromM": 0, "toM": 0.99, "wxNPerM": 114, "wyNPerM": 0}, "fxN": 0, "fyN": 0, "source": "authored-estimate", "reference": "en-1991-1-4"},
    {"id": "wind-shaft", "kind": "wind", "title": T("Gust on the shaft", "Böe auf den Schaft"), "distributed": {"bone": "column", "fromM": 0.99, "toM": 3.66, "wxNPerM": 45.5, "wyNPerM": 0}, "fxN": 0, "fyN": 0, "source": "authored-estimate", "reference": "en-1991-1-4"},
    {"id": "wind-lantern", "kind": "wind", "title": T("Gust on the lantern", "Böe auf die Leuchte"), "bone": "lantern", "x": 0.4, "y": 0, "fxN": 130, "fyN": 0, "source": "authored-estimate", "reference": "en-40-3-1"},
]


EAVE_REST_N = 197.4


def poses():
    p = lambda pid, en, de, den, dde, bones, contacts, th, loads=(), key=False: {"id": pid, "title": T(en, de), "description": T(den, dde), **({"key": True} if key else {}), "bones": bones, "ground": "auto", "contacts": contacts, "thermal": th, "loads": list(loads)}
    fs = ["foundation-support"]
    mat = [{"id": "mat-reaction", "kind": "equipment", "title": T("Lawn mat carrying the eave of the lowered column (0.20 kN)", "Rasenmatte trägt die Traufe der abgesenkten Säule (0,20 kN)"), "anchor": "lantern-rest", "fxN": 0, "fyN": EAVE_REST_N, "source": "authored-estimate"}]
    return [
        p("in-service", "In service, summer afternoon", "Im Betrieb, Sommernachmittag",
          "Locked upright on its hinged foot in the afternoon sun; the LED is off, the dark green paint, the roof and the galvanised plate heat up.",
          "Aufrecht auf dem Klappfuss verriegelt in der Nachmittagssonne; die LED ist aus, der dunkelgrüne Lack, das Dach und die verzinkte Platte erwärmen sich.", {}, fs, binding(SUN_DAY, 0)),
        p("lit-evening", "Lit after dusk", "Nach der Dämmerung beleuchtet",
          "The photocell has switched the 30 W LED retrofit on; still air, no sun, a warm plume leaves the crown vents.",
          "Der Dämmerungsschalter hat das 30-W-LED-Nachrüstmodul eingeschaltet; Windstille, keine Sonne, eine warme Fahne steigt aus den Kronenschlitzen.", {}, fs, binding({}, 1)),
        p("in-use", "In use, a visitor leaning", "In Gebrauch, ein Besucher lehnt",
          "At night under the lit lantern a visitor leans his back against the shaft (0.3 kN at 1.3 m) and a bicycle leans on the base (0.07 kN).",
          "Nachts unter der leuchtenden Laterne lehnt ein Besucher mit dem Rücken am Schaft (0,3 kN in 1,3 m) und ein Fahrrad lehnt am Sockel (0,07 kN).", {}, fs, binding({}, 1),
          [{"id": "visitor-lean", "kind": "occupant", "title": T("Visitor leaning against the shaft", "Besucher lehnt am Schaft"), "bone": "column", "x": 1.205, "y": -0.054, "fxN": -300, "fyN": 0, "source": "authored-estimate", "reference": "en-1991-1-1"},
           {"id": "bicycle-lean", "kind": "live", "title": T("Bicycle leaning on the base", "Fahrrad lehnt am Sockel"), "bone": "column", "x": 0.62, "y": -0.11, "fxN": -60, "fyN": -30, "source": "authored-estimate", "reference": "en-1991-1-1"}]),
        p("storm-night", "Lit in a design gust", "Beleuchtet in der Bemessungsböe",
          "Lit at night while the design gust of wind zone 2 (0.65 kN/m2) presses on base, shaft and lantern; the locking bolts behind the hinge take the overturning moment.",
          "Nachts beleuchtet, während die Bemessungsböe der Windzone 2 (0,65 kN/m2) auf Sockel, Schaft und Leuchte drückt; die Verriegelungsschrauben hinter dem Gelenk nehmen das Kippmoment auf.", {}, fs, binding({}, 1), WIND),
        p("door-open", "Base door hooked open", "Masttür ausgehängt",
          "The depot has unlocked the base door and swung it out about its lower hooks; the LED driver in the connection box is reachable.",
          "Der Bauhof hat die Masttür entriegelt und um ihre unteren Haken ausgeschwenkt; der LED-Treiber im Anschlusskasten ist zugänglich.", {"door": {"angleDeg": 95}}, fs, binding(SUN_DAY, 0), key=True),
        p("maintenance", "Maintenance, head lowered", "Wartung, Kopf abgesenkt",
          "The locking bolts are out, the column is tilted forward about its hinged foot and the lantern rests with its eave on a rubber mat on the lawn; the foundation and the anchor plate stay behind.",
          "Die Verriegelungsschrauben sind gelöst, die Säule ist um den Klappfuss nach vorn gekippt und die Leuchte liegt mit der Traufe auf einer Gummimatte im Rasen; Fundament und Ankerplatte bleiben stehen.",
          {"hinge": {"angleDeg": -90}, "door": {"angleDeg": 95}}, fs, binding(SUN_TIPPED, 0), mat),
    ]


def clips():
    return [
        {"id": "dusk-switching", "title": T("Switching on at dusk", "Einschalten in der Dämmerung"),
         "description": T("The sun sets behind the park trees, the photocell switches the LED on at about 10 lux and the lantern warms up while a plume rises from the crown.",
                          "Die Sonne geht hinter den Parkbäumen unter, der Dämmerungsschalter schaltet die LED bei etwa 10 Lux ein und die Leuchte erwärmt sich, während aus der Krone eine Fahne aufsteigt."),
         "durationS": 900, "fps": 2, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "linear"}, {"t": 300, "pose": "lit-evening", "ease": "step"}, {"t": 900, "pose": "lit-evening", "ease": "step"}],
         "contacts": [{"anchor": "foundation-support", "fromS": 0, "toS": 900}],
         "phases": [{"id": "sunset", "fromS": 0, "toS": 300, "title": T("Sun setting", "Sonnenuntergang")}, {"id": "lit", "fromS": 300, "toS": 900, "title": T("Lit, lantern warming up", "Beleuchtet, Leuchte erwärmt sich")}],
         "events": [{"t": 300, "kind": "ignition", "title": T("Photocell switches the LED on", "Dämmerungsschalter schaltet die LED ein"), "target": "led"},
                    {"t": 330, "kind": "thermal", "title": T("Lantern housing starts to warm up", "Leuchtengehäuse beginnt sich zu erwärmen"), "target": "led"}],
         "tracks": [{"target": "operating:led", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 300, "v": 1}]}, {"target": "emitter:lantern-plume", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 300, "v": 1}]}]},
        {"id": "maintenance", "title": T("Lowering the head for maintenance", "Kopf zur Wartung absenken"),
         "description": T("Two depot workers open the base door, take out the locking bolts and lower the column on a rope about its hinged foot until the lantern eave rests on the lawn mat.",
                          "Zwei Bauhof-Mitarbeiter öffnen die Masttür, nehmen die Verriegelungsschrauben heraus und senken die Säule am Seil um den Klappfuss ab, bis die Leuchtentraufe auf der Rasenmatte liegt."),
         "durationS": 60, "fps": 10, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "out"}, {"t": 8, "pose": "door-open", "ease": "step"}, {"t": 15, "pose": "door-open", "ease": "out"}, {"t": 55, "pose": "maintenance", "ease": "step"}, {"t": 60, "pose": "maintenance", "ease": "step"}],
         "contacts": [{"anchor": "foundation-support", "fromS": 0, "toS": 60}],
         "phases": [{"id": "door", "fromS": 0, "toS": 15, "title": T("Door open, bolts out", "Tür auf, Schrauben heraus")}, {"id": "lowering", "fromS": 15, "toS": 55, "title": T("Lowering on the rope", "Absenken am Seil")}, {"id": "rest", "fromS": 55, "toS": 60, "title": T("Lantern on the mat", "Leuchte auf der Matte")}],
         "events": [{"t": 2, "kind": "motion", "title": T("Door lock released", "Türschloss gelöst"), "target": "door"}, {"t": 12, "kind": "motion", "title": T("Locking bolts removed", "Verriegelungsschrauben entfernt"), "target": "hinge"},
                    {"t": 15, "kind": "motion", "title": T("Column starts to tilt", "Säule beginnt zu kippen"), "target": "hinge"}, {"t": 55, "kind": "motion", "title": T("Eave touches the mat", "Traufe berührt die Matte"), "target": "lantern"}],
         "tracks": []},
    ]


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5},
        200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
RANGE = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES = {50: [{"id": "concrete-stipple", "title": T("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.13},
                {"id": "algae-stipple", "title": T("Algae film stipple", "Algenbelag-Punktierung"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}],
           100: [{"id": "concrete-stipple", "title": T("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.7, "penMm": 0.1},
                 {"id": "algae-stipple", "title": T("Algae film stipple", "Algenbelag-Punktierung"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.5, "penMm": 0.1}],
           200: [], 500: []}

DECISIONS = {
    50: {
        "silhouette": T("Side elevation 9.8 x 92.8 mm: the 0.40 m precast foundation head 60 mm proud of the gravel with its broken front arris, the 340 mm galvanised anchor plate with two of four M16 nuts and bolt ends, the two hinge lugs with the pin, the hinged cast foot ring with its knuckle and the two locking bolts at the back, the cast plinth with its half-round bead and cove, the 220 mm base body with the hook-in door at the back, the capital ring and cone into the 114 to 88 mm shaft, the top bead and the spigot neck, and the round heritage lantern: neck sleeve with grub screw, bowl, bottom ring, the conical polycarbonate glazing between six struts with the replaced clear pane facing the viewer, the LED retrofit module behind it, top ring, 0.49 m eave, bell roof, crown with vent slots, cap and finial. Front view: the same column and lantern from ahead with the chipped arris, both hinge lugs, the pin and the knuckle across, the struts at their lateral positions and the replaced pane as a band at the right edge. Plan: the chipped foundation head, the plate, four hexagon nuts, lugs and pin under the eave circle, the roof, crown, cap and finial rings, a paint chip on the eave and droppings on the roof.",
                        "Seitenansicht 9,8 x 92,8 mm: der 0,40 m grosse Kopf des Betonfertigteilfundaments 60 mm über dem Kies mit abgeschlagener vorderer Kante, die verzinkte Ankerplatte 340 mm mit zwei von vier Muttern M16 und Schraubenenden, die beiden Gelenklaschen mit Bolzen, der gegossene Klappfussring mit Gelenkauge und den beiden Verriegelungsschrauben hinten, der Gusssockel mit Halbrundwulst und Hohlkehle, der Sockelkörper 220 mm mit der eingehängten Tür hinten, Kapitellring und Konus in den Schaft 114 auf 88 mm, der obere Wulst und der Zapfenhals sowie die runde Nostalgieleuchte: Halshülse mit Madenschraube, Boden, Bodenring, die konische Polycarbonat-Verglasung zwischen sechs Streben mit der ersetzten klaren Scheibe zum Betrachter, dahinter das LED-Nachrüstmodul, Kopfring, Traufe 0,49 m, Glockendach, Krone mit Lüftungsschlitzen, Kappe und Spitze. Vorderansicht: dieselbe Säule und Leuchte von vorn mit abgeschlagener Kante, beiden Gelenklaschen, Bolzen und Gelenkauge quer, den Streben an ihren seitlichen Lagen und der ersetzten Scheibe als Streifen am rechten Rand. Draufsicht: der abgeschlagene Fundamentkopf, die Platte, vier Sechskantmuttern, Laschen und Bolzen unter dem Traufkreis, die Ringe von Dach, Krone, Kappe und Spitze, ein Lackabplatzer an der Traufe und Vogelkot auf dem Dach."),
        "retained": T("Every part of the real post and lantern and every trace of use above 7.5 mm: the broken arris, rust streaks below the nuts, the white corrosion bloom on the foot ring, three paint chips from strimmer and ladder knocks, the graffiti tag on the base, the algae film on the shaded back of the plinth, lock scratches and the sticker residue on the shaft, the replaced pane, droppings on the roof.",
                      "Jedes Teil des echten Masts und der Leuchte und jede Gebrauchsspur über 7,5 mm: die abgeschlagene Kante, Rostfahnen unter den Muttern, die weisse Korrosionsblüte am Fussring, drei Lackabplatzer von Freischneider und Leiter, das Graffiti-Tag am Sockel, der Algenbelag auf der beschatteten Sockelrückseite, Schlosskratzer und Aufkleberreste am Schaft, die ersetzte Scheibe, Vogelkot auf dem Dach."),
        "merged": T("Nothing is merged; the base shell, the door and the connection box are separate parts, the six struts are one rotated row.", "Nichts ist zusammengefasst; Sockelschale, Tür und Anschlusskasten sind eigene Teile, die sechs Streben sind eine gedrehte Reihe."),
        "omitted": T("The in-ground part of the precast foundation (cut at the ground), the cable, the fuse inside the connection box and the LED driver electronics.", "Der eingebundene Teil des Fertigteilfundaments (im Gelände geschnitten), das Kabel, die Sicherung im Anschlusskasten und die Elektronik des LED-Treibers."),
        "thinElements": T("Struts 16 mm (0.32 mm), pin 16 mm, lock bolts 8 mm high (0.16 mm) and the 8 mm crown slots stay at their real size; scratches, bloom, rust and LED fins are 0.13 mm texture strokes.", "Streben 16 mm (0,32 mm), Bolzen 16 mm, Verriegelungsschrauben 8 mm hoch (0,16 mm) und die 8-mm-Kronenschlitze bleiben in echter Grösse; Kratzer, Blüte, Rost und LED-Rippen sind Texturstriche 0,13 mm."),
        "hatching": T("Concrete stipple in the foundation head, a fine stipple marks the algae film; the glazing is a light tone without hatch.", "Betonpunktierung im Fundamentkopf, eine feine Punktierung markiert den Algenbelag; die Verglasung ist ein heller Ton ohne Schraffur."),
        "styles": T("Pens 0.5 body, 0.25 detail, 0.13 texture, 0.18 glazing; dark fir-green tones for the painted aluminium, mid grey for galvanised steel, light grey for concrete, yellowed and clear tones for the old and the new pane.", "Stifte 0,5 Körper, 0,25 Detail, 0,13 Textur, 0,18 Verglasung; dunkle tannengrüne Töne für das lackierte Aluminium, mittleres Grau für verzinkten Stahl, helles Grau für Beton, vergilbte und klare Töne für alte und neue Scheibe."),
        "overlays": T("Thermal, force and moment overlays use the same outlines; the hinge pin and the support carry the force glyphs.", "Wärme-, Kraft- und Momentüberlagerungen nutzen dieselben Umrisse; Gelenkbolzen und Auflager tragen die Kraftsymbole."),
    },
    100: {
        "silhouette": T("Side elevation 4.9 x 46.4 mm: foundation head with broken arris, anchor plate, the two lugs at 15 mm and the pin, knuckle, foot ring, a simpler plinth bead, base body with the door, capital, shaft, top bead and spigot neck; the lantern with neck, bowl, glazing between six struts, the replaced pane, the LED module, top ring, eave, roof, crown, cap and finial. Front and plan views draw the same parts.",
                        "Seitenansicht 4,9 x 46,4 mm: Fundamentkopf mit abgeschlagener Kante, Ankerplatte, die beiden Laschen mit 15 mm und der Bolzen, Gelenkauge, Fussring, ein einfacherer Sockelwulst, Sockelkörper mit Tür, Kapitell, Schaft, oberer Wulst und Zapfenhals; die Leuchte mit Hals, Boden, Verglasung zwischen sechs Streben, ersetzter Scheibe, LED-Modul, Kopfring, Traufe, Dach, Krone, Kappe und Spitze. Vorder- und Draufsicht zeigen dieselben Teile."),
        "retained": T("The broken arris, the graffiti tag, the algae film, the sticker residue, lock scratches, the white bloom and the replaced pane.", "Die abgeschlagene Kante, das Graffiti-Tag, der Algenbelag, die Aufkleberreste, Schlosskratzer, die weisse Blüte und die ersetzte Scheibe."),
        "merged": T("Bead and cove of the plinth and the rings of the capital and of the top are drawn with fewer facets; the lugs are thickened to 15 mm.", "Wulst und Kehle des Sockels sowie die Ringe von Kapitell und Kopf sind mit weniger Facetten gezeichnet; die Laschen sind auf 15 mm verstärkt."),
        "omitted": T("Anchor nuts and bolt ends (13 mm), the locking bolts (8 mm), the door lock, the paint chips, droppings, crown slots and LED fins, all below 15 mm.", "Ankermuttern und Schraubenenden (13 mm), die Verriegelungsschrauben (8 mm), das Türschloss, Lackabplatzer, Vogelkot, Kronenschlitze und LED-Rippen, alle unter 15 mm."),
        "thinElements": T("The 16 mm struts and the 16 mm pin keep their real width (0.16 mm).", "Die 16-mm-Streben und der 16-mm-Bolzen behalten ihre echte Breite (0,16 mm)."),
        "hatching": T("Concrete stipple at 0.7 mm, algae stipple at 0.5 mm.", "Betonpunktierung 0,7 mm, Algenpunktierung 0,5 mm."),
        "styles": T("Pens 0.35 body, 0.18 detail, 0.13 texture and glazing; the same tones as at 1:50.", "Stifte 0,35 Körper, 0,18 Detail, 0,13 Textur und Verglasung; dieselben Töne wie in 1:50."),
        "overlays": T("Thermal, force and moment overlays on the same outlines.", "Wärme-, Kraft- und Momentüberlagerungen auf denselben Umrissen."),
    },
    200: {
        "silhouette": T("Side elevation 2.5 x 23.2 mm: one foundation block of head and plate with its broken arris, the hinge barrel, the plinth with the foot step, base body with the door joint, capital, the shaft with its top, the lantern neck, the glazing with the LED module behind, the roof with eave and crown in one outline and the finial.",
                        "Seitenansicht 2,5 x 23,2 mm: ein Fundamentblock aus Kopf und Platte mit abgeschlagener Kante, die Gelenkwalze, der Sockel mit Fussstufe, Sockelkörper mit Türfuge, Kapitell, der Schaft mit Kopf, der Leuchtenhals, die Verglasung mit dem LED-Modul dahinter, das Dach mit Traufe und Krone in einem Umriss und die Spitze."),
        "retained": T("The broken foundation arris (40 mm) and the door joint; the LED module behind the glass.", "Die abgeschlagene Fundamentkante (40 mm) und die Türfuge; das LED-Modul hinter dem Glas."),
        "merged": T("Head and plate are one shape (the 15 mm plate is 0.075 mm here); lugs, pin and knuckle are one barrel; foot ring and plinth, shaft and top, top ring, eave, roof, crown and cap are merged outlines.", "Kopf und Platte sind eine Form (die 15-mm-Platte ist hier 0,075 mm); Laschen, Bolzen und Gelenkauge sind eine Walze; Fussring und Sockel, Schaft und Kopf, Kopfring, Traufe, Dach, Krone und Kappe sind zusammengefasste Umrisse."),
        "omitted": T("The six struts (16 mm, 0.08 mm on paper), the replaced pane, all textures of use and weather (the texture pen is not drawn at this scale).", "Die sechs Streben (16 mm, 0,08 mm auf Papier), die ersetzte Scheibe, alle Gebrauchs- und Witterungstexturen (der Texturstift wird in diesem Massstab nicht gezeichnet)."),
        "thinElements": T("The 88 mm shaft top is 0.44 mm, the finial tip is widened to 32 mm (0.16 mm).", "Das 88-mm-Schaftende misst 0,44 mm, die Spitze ist auf 32 mm (0,16 mm) verbreitert."),
        "hatching": T("No hatch; tones alone.", "Keine Schraffur; nur Töne."),
        "styles": T("Pens 0.25 body, 0.13 detail and glazing.", "Stifte 0,25 Körper, 0,13 Detail und Verglasung."),
        "overlays": T("Thermal, force and moment overlays on the merged outlines; merged shapes are painted with the area-weighted value of their regions.", "Wärme-, Kraft- und Momentüberlagerungen auf den zusammengefassten Umrissen; zusammengefasste Formen erhalten den flächengewichteten Wert ihrer Bereiche."),
    },
    500: {
        "silhouette": T("Side elevation 1.0 x 9.3 mm: the foundation block, the column in one outline from the foot step over the base and capital to the spigot, the lantern neck, the glazing with the LED module as one tone and the roof running into the finial.",
                        "Seitenansicht 1,0 x 9,3 mm: der Fundamentblock, die Säule in einem Umriss von der Fussstufe über Sockel und Kapitell bis zum Zapfen, der Leuchtenhals, Verglasung und LED-Modul als ein Ton und das in die Spitze auslaufende Dach."),
        "retained": T("The proportions of base, shaft and lantern and the 0.49 m eave.", "Die Proportionen von Sockel, Schaft und Leuchte und die Traufe 0,49 m."),
        "merged": T("Foundation and plate; foot ring, base, capital, shaft and top; glazing and LED module; top ring, eave, roof, crown, cap and finial.", "Fundament und Platte; Fussring, Sockel, Kapitell, Schaft und Kopf; Verglasung und LED-Modul; Kopfring, Traufe, Dach, Krone, Kappe und Spitze."),
        "omitted": T("The hinge, the door, the struts, the broken arris and every trace of use (all below 75 mm).", "Das Gelenk, die Tür, die Streben, die abgeschlagene Kante und alle Gebrauchsspuren (alle unter 75 mm)."),
        "thinElements": T("The 88 mm shaft top is 0.18 mm; the finial ends at 80 mm (0.16 mm).", "Das 88-mm-Schaftende misst 0,18 mm; die Spitze endet bei 80 mm (0,16 mm)."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Body pen 0.18 only.", "Nur der Körperstift 0,18."),
        "overlays": T("Thermal, force and moment overlays on the five outlines.", "Wärme-, Kraft- und Momentüberlagerungen auf den fünf Umrissen."),
    },
}

VDEC = {
    50: [T("Body: the post as built and used. Foundation head as a plan extrusion with the broken arris (40 mm); anchor plate as a declared 15 mm sheet; four hexagon nuts AF 24 and bolt ends as rotated rows; two 12 mm hinge lugs and the 16 mm pin; the knuckle of the cast foot. Column revolves about its own axis: foot ring, plinth with bead and cove, the 10 mm base shell in three parts, the middle one a C-shaped extrusion that leaves the 100 mm door opening at the back, the capital, the shaft 114 to 88 mm, top bead and 76 mm spigot; the polyamide connection box with the LED driver sits on the inner wall behind the door; the door is a 10 mm curved panel with its lock boss. Lantern: neck, bowl, the 4 mm glazing as two shells (300 degrees UV-yellowed, the replaced 60 degree pane clear), six 16 mm struts as one rotated row, top ring, roof with 0.49 m eave, crown, cap, finial, the LED retrofit on its bracket. Graffiti, algae and sticker residue are 1.5 mm texture films.",
           "Körper: der Mast wie gebaut und benutzt. Fundamentkopf als Grundriss-Extrusion mit abgeschlagener Kante (40 mm); Ankerplatte als erklärtes 15-mm-Blech; vier Sechskantmuttern SW 24 und Schraubenenden als gedrehte Reihen; zwei 12-mm-Gelenklaschen und der 16-mm-Bolzen; das Gelenkauge des Gussfusses. Die Säule dreht um ihre eigene Achse: Fussring, Sockel mit Wulst und Kehle, die 10-mm-Sockelschale in drei Teilen, das mittlere eine C-förmige Extrusion, die hinten die 100-mm-Türöffnung frei lässt, Kapitell, Schaft 114 auf 88 mm, oberer Wulst und 76-mm-Zapfen; der Polyamid-Anschlusskasten mit dem LED-Treiber sitzt hinter der Tür an der Innenwand; die Tür ist ein gebogenes 10-mm-Blatt mit Schlossbuckel. Leuchte: Hals, Boden, die 4-mm-Verglasung als zwei Schalen (300 Grad UV-vergilbt, die ersetzte 60-Grad-Scheibe klar), sechs 16-mm-Streben als gedrehte Reihe, Kopfring, Dach mit Traufe 0,49 m, Krone, Kappe, Spitze, das LED-Nachrüstmodul auf seinem Halter. Graffiti, Algen und Aufkleberreste sind 1,5-mm-Texturfilme."),
         T("Thermal: eight regions as solids; the base is one filled body over its outer skin, so the inside of the hollow base and of the lantern does not count as surface, and the LED module inside the closed glazing counts its whole surface. Structure: members along foundation, hinge, column, door and lantern bones, the pin and the spigot as joints, the fixed support at the ground cut, masses at the centres of the bones.",
           "Wärme: acht Bereiche als Körper; der Sockel ist ein gefüllter Körper über seiner Aussenhaut, so dass das Innere des hohlen Sockels und der Leuchte nicht als Fläche zählt, und das LED-Modul in der geschlossenen Verglasung zählt seine ganze Oberfläche. Tragwerk: Stäbe entlang Fundament, Gelenk, Säule, Tür und Leuchte, Bolzen und Zapfen als Gelenke, das feste Auflager im Geländeschnitt, Massen in den Schwerpunkten der Knochen.")],
    100: [T("Body for 15 mm: nuts, bolt ends, locking bolts and the door lock are left out; the lugs are 15 mm thick; the plinth, capital and top have fewer facets; the base shell and the door are 15 mm thick; struts keep their 16 mm section as slender rods; textures as at 1:50.",
            "Körper für 15 mm: Muttern, Schraubenenden, Verriegelungsschrauben und Türschloss entfallen; die Laschen sind 15 mm dick; Sockel, Kapitell und Kopf haben weniger Facetten; Sockelschale und Tür sind 15 mm dick; die Streben behalten ihren 16-mm-Querschnitt als schlanke Stäbe; Texturen wie in 1:50."),
          T("Thermal and structure follow the body of this scale; the struts stay in both lists.", "Wärme und Tragwerk folgen dem Körper dieses Massstabs; die Streben bleiben in beiden Listen.")],
    200: [T("Body for 30 mm: head and plate stay (plate as declared sheet), lugs, pin and knuckle become one 32 mm hinge barrel, foot ring and plinth are one revolve, the base is one closed 30 mm shell without the door, capital and shaft with top are revolves, the lantern is neck, glazing shell, LED module, roof with top ring, eave, crown and cap, and finial; struts, connection box, door and textures are left out.",
            "Körper für 30 mm: Kopf und Platte bleiben (Platte als erklärtes Blech), Laschen, Bolzen und Gelenkauge werden eine 32-mm-Gelenkwalze, Fussring und Sockel sind ein Drehkörper, der Sockel ist eine geschlossene 30-mm-Schale ohne Tür, Kapitell und Schaft mit Kopf sind Drehkörper, die Leuchte besteht aus Hals, Glasschale, LED-Modul, Dach mit Kopfring, Traufe, Krone und Kappe sowie Spitze; Streben, Anschlusskasten, Tür und Texturen entfallen."),
          T("Thermal keeps the six struts as slender rods so the lantern frame keeps its surface; the structure keeps them as slender members, the column member also stands for the door.", "Die Wärme behält die sechs Streben als schlanke Stäbe, damit der Leuchtenrahmen seine Fläche behält; das Tragwerk behält sie als schlanke Stäbe, der Säulenstab steht auch für die Tür.")],
    500: [T("Body for 75 mm: the foundation and plate are one 75 mm block, the column one revolve from the foot step to the spigot, the lantern neck, the glazing shell, the 90 mm LED module, and the roof running into the finial.",
            "Körper für 75 mm: Fundament und Platte sind ein 75-mm-Block, die Säule ein Drehkörper von der Fussstufe bis zum Zapfen, der Leuchtenhals, die Glasschale, das 90-mm-LED-Modul und das in die Spitze auslaufende Dach."),
          T("Thermal splits the column into base and shaft and keeps the anchor plate as a 15 mm sheet on the block and the struts as slender rods; the structure has one column member that also stands for hinge and door, a hinge joint at the pivot, the lantern members, the support and three masses.",
            "Die Wärme teilt die Säule in Sockel und Schaft und behält die Ankerplatte als 15-mm-Blech auf dem Block und die Streben als schlanke Stäbe; das Tragwerk hat einen Säulenstab, der auch für Gelenk und Tür steht, ein Gelenk am Drehpunkt, die Leuchtenstäbe, das Auflager und drei Massen.")],
}


def asset():
    reps, vols = {}, {}
    for den in (50, 100, 200, 500):
        reps[str(den)] = {"status": "authored", "denominator": den, "validRange": {"minDenominator": RANGE[den][0], "maxDenominator": RANGE[den][1]}, "pens": PENS[den], "hatches": HATCHES[den], "geometry": views(den), "decisions": DECISIONS[den]}
        vols[str(den)] = {"status": "authored", "denominator": den, "body": body_list(den), "thermal": thermal_list(den), "structure": structure_list(den), "decisions": VDEC[den]}
    mass = round(sum(MASS[b] for b in ("foundation", "hinge", "column", "door", "lantern")), 1)
    return {
        "id": "used-park-lantern", "category": "objects", "title": T("Used Park Post-Top Lantern", "Gebrauchte Parkleuchte"),
        "summary": T("A 4 m heritage-style cast-aluminium post-top park lantern on a hinged foot, retrofitted with a 30 W LED module, standing on its precast foundation after decades in a municipal park: chalked fir-green paint with chips, graffiti, sticker residue and algae, a yellowed glazing with one replaced pane; the depot lowers its head about the hinge, at dusk it switches on.",
                     "Eine 4 m hohe Nostalgie-Aufsatzleuchte aus Aluminiumguss auf einem Klappfuss, mit einem 30-W-LED-Modul nachgerüstet, auf ihrem Fertigteilfundament nach Jahrzehnten in einem städtischen Park: verkreideter tannengrüner Lack mit Abplatzern, Graffiti, Aufkleberresten und Algen, eine vergilbte Verglasung mit einer ersetzten Scheibe; der Bauhof senkt ihren Kopf um das Gelenk ab, in der Dämmerung schaltet sie ein."),
        "tags": ["used", "park", "public-space", "lighting", "lantern", "street-lamp", "cast-aluminium", "led", "heritage", "hinged-column", "street-furniture"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                       "notes": T("Side elevation from the right. Light point height 4.0 m (LED centre), overall 4.64 m to the finial tip; the 3.66 m column stands on a hinged cast foot (Klappmast) over a galvanised anchor plate on a precast foundation whose head stands 60 mm out of the worn gravel. Masses are section times density of the real hollow parts (wall thicknesses in the report), not of the filled modelling solids. Nothing is measured: dimensions are catalogue-typical (authored-estimate), the traces of use are illustrative.",
                                  "Seitenansicht von rechts. Lichtpunkthöhe 4,0 m (LED-Mitte), gesamt 4,64 m bis zur Spitze; die 3,66 m hohe Säule steht auf einem gegossenen Klappfuss über einer verzinkten Ankerplatte auf einem Fertigteilfundament, dessen Kopf 60 mm aus dem ausgetretenen Kies ragt. Massen sind Querschnitt mal Dichte der echten hohlen Teile (Wanddicken im Bericht), nicht der gefüllten Modellkörper. Nichts ist gemessen: Masse sind katalogtypisch (authored-estimate), die Gebrauchsspuren illustrativ.")},
        "dimensions": {"heightM": r6(HEIGHT), "widthM": r6(2 * EAVE_R), "depthM": r6(2 * EAVE_R), "massKg": mass},
        "materials": MATERIALS, "rig": rig(), "volumes": vols, "anchors": ANCHORS, "representations": reps, "symbols": [], "poses": poses(), "clips": clips(),
        "thermal": {"environment": {"airTempC": 30, "meanRadiantTempC": 33, "skyTempC": 18, "groundTempC": 40, "relativeHumidity": 0.45, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 800, "diffuseFraction": 0.15, "pressurePa": 101325},
                    "referenceEnvironment": "summer-hot-sun", "passengers": [], "operating": [{"id": "led", "title": T("LED retrofit module", "LED-Nachrüstmodul"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 1, "title": T("On (30 W, 20 W of heat)", "Ein (30 W, 20 W Wärme)")}]}],
                    "regions": regions(),
                    "emitters": [{"id": "lantern-plume", "kind": "plume", "title": T("Warm plume above the lantern", "Warme Fahne über der Leuchte"), "anchor": "lantern-plume", "directionDeg": 90, "diameterM": 0.14, "region": "led-module", "source": "authored-estimate"}],
                    "assumptions": [
                        T("Reference case: a summer afternoon in the park, 30 degC air, 800 W/m2 sun with 15 percent diffuse; the in-service pose binds the sunlit share of every region (vertical cylinders about half, the roof 0.7); the night poses bind no sun. No surface temperature is authored.", "Referenzfall: ein Sommernachmittag im Park, Luft 30 degC, Sonne 800 W/m2 mit 15 Prozent Diffusanteil; die Betriebspose bindet den besonnten Anteil jedes Bereichs (senkrechte Zylinder etwa die Hälfte, das Dach 0,7); die Nachtposen binden keine Sonne. Es wird keine Oberflächentemperatur festgelegt."),
                        T("Weathered surfaces differ from new ones: the chalked dark-green powder coat absorbs 0.86 of the sun (fresh about 0.9, chalking and soiling change it, Berdahl et al. 2008), aged galvanising 0.75 with emissivity 0.45 instead of about 0.25 new, weathered concrete 0.72 instead of about 0.6, the UV-yellowed polycarbonate 0.30 against 0.12 of the replaced clear pane; these optical values are authored estimates.", "Verwitterte Oberflächen unterscheiden sich von neuen: die verkreidete dunkelgrüne Pulverbeschichtung absorbiert 0,86 der Sonne (neu etwa 0,9, Kreidung und Verschmutzung verändern sie, Berdahl et al. 2008), gealterte Verzinkung 0,75 bei Emissionsgrad 0,45 statt etwa 0,25 neu, verwitterter Beton 0,72 statt etwa 0,6, das UV-vergilbte Polycarbonat 0,30 gegenüber 0,12 der ersetzten klaren Scheibe; diese optischen Werte sind Schätzungen."),
                        T("The LED retrofit draws 30 W electrical input; about two thirds (20 W) are released as heat from its heat sink inside the closed glazing (Schubert 2006); the plume above the crown follows the solved sensible heat of the module. The 30 W and the heat share are authored estimates.", "Das LED-Nachrüstmodul nimmt 30 W elektrisch auf; etwa zwei Drittel (20 W) werden als Wärme von seinem Kühlkörper in der geschlossenen Verglasung abgegeben (Schubert 2006); die Fahne über der Krone folgt der berechneten fühlbaren Wärme des Moduls. Die 30 W und der Wärmeanteil sind Schätzungen."),
                        T("Heat capacities: rho*cp*min(thickness, sqrt(alpha*3600 s)) with ISO 10456 values (the concrete head: 55 mm thermally active).", "Wärmekapazitäten: rho*cp*min(Dicke, sqrt(alpha*3600 s)) mit ISO-10456-Werten (der Betonkopf: 55 mm thermisch aktiv)."),
                    ]},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["foundation-support"], "clinical": False, "assumptions": [
            T("The precast foundation (0.40 x 0.40 x 0.80 m) is a fixed support cut at the ground; only its 60 mm head is modelled and counted in the mass.", "Das Fertigteilfundament (0,40 x 0,40 x 0,80 m) ist ein festes Auflager, im Gelände geschnitten; nur sein 60-mm-Kopf ist modelliert und in der Masse enthalten."),
            T("In service the column is locked on its hinged foot by two M10 bolts behind the hinge; the hinge pin and the bolts form a fixed base, so the column is a cantilever. The design gust of wind zone 2 (0.65 kN/m2, cf 0.8 on the base, 0.7 on the shaft, 1.0 on the lantern) gives about 0.6 kNm at the foot and 10 MPa in the 114 x 6 mm shaft (EN AW-6060 T66: 160 MPa).", "Im Betrieb ist die Säule auf ihrem Klappfuss durch zwei Schrauben M10 hinter dem Gelenk verriegelt; Gelenkbolzen und Schrauben bilden eine Einspannung, die Säule ist ein Kragarm. Die Bemessungsböe der Windzone 2 (0,65 kN/m2, cf 0,8 am Sockel, 0,7 am Schaft, 1,0 an der Leuchte) ergibt etwa 0,6 kNm am Fuss und 10 MPa im Schaft 114 x 6 mm (EN AW-6060 T66: 160 MPa)."),
            T("In maintenance the bolts are out and the column turns about the hinge pin; while it is lowered the moment about the pin (about 0.9 kNm at the start of the tilt) is held by two workers on a rope tied to the lantern neck, which the quasi-static joint moment of the hinge stands for; at rest the lowered part (44.9 kg, centre of mass 1.88 m from the hinge pin) lies on the pin and, 4.19 m from it, with the lantern eave on the lawn mat: the mat carries 0.20 kN and the pin 0.24 kN; the mat is applied as this carrying force because a second support of the archive would share the weight with the foundation as a spring of equal stiffness, and while the column is lowered the same force, fading in, stands for the rope.", "In der Wartung sind die Schrauben heraus und die Säule dreht um den Gelenkbolzen; während des Absenkens halten zwei Mitarbeiter das Moment um den Bolzen (etwa 0,9 kNm zu Beginn des Kippens) an einem Seil am Leuchtenhals, wofür das quasistatische Gelenkmoment steht; in Ruhe liegt der abgesenkte Teil (44,9 kg, Schwerpunkt 1,88 m vom Gelenkbolzen) auf dem Bolzen und 4,19 m davon mit der Leuchtentraufe auf der Rasenmatte: die Matte trägt 0,20 kN und der Bolzen 0,24 kN; die Matte ist als diese Tragkraft angesetzt, weil ein zweites Auflager des Archivs das Gewicht als Feder gleicher Steifigkeit mit dem Fundament teilen würde, und während des Absenkens steht dieselbe, einblendende Kraft für das Seil."),
            T("A visitor leaning on the shaft is 0.3 kN horizontal at 1.3 m and a leaning bicycle 0.07 kN at the base (authored estimates on the basis of EN 1991-1-1, 6.4).", "Ein am Schaft lehnender Besucher ist 0,3 kN waagerecht in 1,3 m und ein angelehntes Fahrrad 0,07 kN am Sockel (Schätzungen auf Grundlage von EN 1991-1-1, 6.4)."),
        ]},
        "presentation": {"defaultPose": "in-service", "defaultClip": "dusk-switching", "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }


if __name__ == "__main__":
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset()}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    print("[DEBUG] wrote", os.path.join(HERE, "🔣️.json"), doc["asset"]["dimensions"])
