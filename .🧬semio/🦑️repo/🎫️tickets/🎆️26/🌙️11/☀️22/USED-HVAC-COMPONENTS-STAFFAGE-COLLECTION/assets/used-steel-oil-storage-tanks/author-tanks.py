"""🛢️ Authoritative generator of the staffage asset `used-steel-oil-storage-tanks`.

Three used 1000 l double-walled plastic-in-steel heating-oil battery tanks with their filler, vent and extraction lines,
cleaned and degassed, lying on edge on two squared-timber bearers; lifted on fork tines; reinstalled upright and filled.
Every hand-chosen number is stated here; running the script rewrites `🔣️.json` next to it.

@see ../../brief.md
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
"""

import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
SLUG = "used-steel-oil-storage-tanks"

L, H, B, R = 1.64, 1.10, 0.76, 0.03
TANK_Z = {"tank-1": 0.81, "tank-2": 0.0, "tank-3": -0.81}
BEAR_Y = 0.10
BEARERS = {"bearer-a": 0.27, "bearer-b": 1.27}
BEARER_SIZE, BEARER_LEN = 0.10, 2.50
BEADS = [0.50, 0.82, 1.14]
BEAD_W, BEAD_P = 0.04, 0.004
SOCKETS = [("filler", 0.30, 0.03, 0.065), ("vent", 0.55, 0.03, 0.065), ("extraction", 0.80, 0.015, 0.046)]
PIPE_U, PIPE_R, PIPE_Z = 0.0858, 0.02415, 0.93
CU_U, CU_V, CU_R = 0.051, 0.80, 0.006
LINES_LEN = 0.11
BAT_X0 = R
KIT_U0 = 0.0
KIT_V1 = 0.806
T_STEEL, T_GAP, T_PE = 0.00125, 0.015, 0.005
RHO_STEEL, RHO_PE, RHO_SPRUCE = 7850.0, 950.0, 450.0
PIPE_KG_M, CU_KG_M, RUN_M = 3.71, 0.308, 1.86
FITTINGS_KG = 1.6
OIL_KG = 845.0
G = 9.81


def t(en, de):
    """📝 One bilingual text."""
    return {"en": en, "de": de}


def dez(x):
    """🇩🇪 A number with a decimal comma."""
    return str(x).replace(".", ",")


def r6(x):
    """🎯 Round to micrometres."""
    return round(x, 6)


def box_area(l, h, b):
    """📦 Surface of a rectangular box."""
    return 2 * (l * h + l * b + h * b)


STEEL_KG = box_area(L, H, B) * T_STEEL * RHO_STEEL
PE_IN = T_STEEL + T_GAP
PE_KG = box_area(L - 2 * PE_IN, H - 2 * PE_IN, B - 2 * PE_IN) * T_PE * RHO_PE
TANK_KG = round(STEEL_KG + PE_KG + FITTINGS_KG, 2)
BEARER_KG = round(BEARER_SIZE * BEARER_SIZE * BEARER_LEN * RHO_SPRUCE, 2)
LINES_KG = round(2 * RUN_M * PIPE_KG_M + RUN_M * CU_KG_M + 4 * 0.05, 2)
MASS = round(3 * TANK_KG + 2 * BEARER_KG + LINES_KG, 2)
HEIGHT = r6(BEAR_Y + H + BEAD_P)
WIDTH = r6(1.64 + PIPE_U + PIPE_R)
DEPTH = BEARER_LEN


def rounded_rect(v0, v1, w0, w1, rad, n=4):
    """⬜ A rounded rectangle polygon in a profile plane, counter-clockwise."""
    pts = []
    corners = [(v1 - rad, w1 - rad, 0), (v0 + rad, w1 - rad, 90), (v0 + rad, w0 + rad, 180), (v1 - rad, w0 + rad, 270)]
    for cv, cw, a0 in corners:
        for k in range(n + 1):
            a = math.radians(a0 + 90 * k / n)
            pts.append([r6(cv + rad * math.cos(a)), r6(cw + rad * math.sin(a))])
    return pts


def solid_body(id_, bone, material, tone, kind, **kw):
    """🧊 A body solid."""
    s = {"id": id_, "kind": kind, "bone": bone, "material": material, "role": kw.pop("role", "body"), "tone": tone}
    s.update(kw)
    return s


def boxs(id_, bone, material, tone, size, at, radius=None, **kw):
    """📦 A box solid centred at `at`."""
    s = solid_body(id_, bone, material, tone, "box", size=[r6(x) for x in size], transform={"translation": [r6(x) for x in at]}, **kw)
    if radius:
        s["cornerRadius"] = radius
    return s


def cyl(id_, bone, material, tone, axis, origin, a0, a1, radius, **kw):
    """🥫 A cylinder as an open two-point revolve along `axis` from `a0` to `a1`."""
    return solid_body(id_, bone, material, tone, "revolve", profile=[[r6(a0), r6(radius)], [r6(a1), r6(radius)]], axis=axis, origin=[r6(x) for x in origin], **kw)


def shape(id_, bone, role, z, filled, tone, geom, region=None, regions=None, material=None, solid=None, pen=None):
    """✏️ One 2D shape."""
    s = {"id": id_, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    s.update(geom)
    if solid is not None:
        s["solid"] = {"kind": "extruded", "sweep": "w", "z0": r6(solid[0]), "z1": r6(solid[1])}
    if pen is not None:
        s["penMm"] = pen
    return s


def rect(x, y, w, h, r=None):
    """▭ A rect geometry."""
    g = {"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}
    if r:
        g["r"] = r
    return g


def circ(cx, cy, rad):
    """⚪ An ellipse geometry of equal radii."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rad), "ry": r6(rad)}


def poly(points, closed=False):
    """〰️ A polyline geometry."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in points], "closed": closed}


MATERIALS = [
    {"id": "steel-jacket", "title": t("Steel jacket DC01, powder coated, weathered", "Stahlmantel DC01, pulverbeschichtet, bewittert"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 210e6, "source": "literature", "reference": "en-10130"},
    {"id": "pe-hd", "title": t("Inner tank HDPE, blow moulded", "Innenbehälter PE-HD, blasgeformt"), "emissivity": 0.92, "solarAbsorptivity": 0.5, "densityKgM3": RHO_PE, "conductivityWmK": 0.4, "specificHeatJKgK": 1900, "elasticModulusPa": 0.9e9, "strengthPa": 22e6, "source": "literature", "reference": "en-iso-10456"},
    {"id": "interstitial-air", "title": t("Interstitial air space between inner tank and jacket", "Luftraum zwischen Innenbehälter und Mantel"), "emissivity": 0.9, "densityKgM3": 1.2, "conductivityWmK": 0.026, "specificHeatJKgK": 1005, "source": "literature", "reference": "en-iso-6946"},
    {"id": "galvanised-steel", "title": t("Galvanised steel tube EN 10255, dull weathered", "Verzinktes Stahlrohr EN 10255, matt bewittert"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 195e6, "source": "literature", "reference": "ashrae-2021"},
    {"id": "copper-oxidised", "title": t("Copper tube EN 1057, oxidised", "Kupferrohr EN 1057, oxidiert"), "emissivity": 0.6, "solarAbsorptivity": 0.65, "densityKgM3": 8900, "conductivityWmK": 380, "specificHeatJKgK": 385, "elasticModulusPa": 120e9, "source": "literature", "reference": "ashrae-2021"},
    {"id": "pe-cap", "title": t("Red PE protective cap", "Rote PE-Schutzkappe"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 950, "conductivityWmK": 0.4, "specificHeatJKgK": 1900, "source": "literature", "reference": "en-iso-10456"},
    {"id": "spruce-weathered", "title": t("Squared spruce C24, weathered grey", "Kantholz Fichte C24, vergraut"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO_SPRUCE, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11e9, "strengthPa": 24e6, "source": "literature", "reference": "en-338"},
]

REFERENCES = [
    {"id": "en-13341", "citation": "EN 13341:2005+A1:2011 Static thermoplastic tanks for above ground storage of domestic heating oils, kerosene and diesel fuels: blow moulded polyethylene tanks (inner tanks of the battery system)."},
    {"id": "trws-791", "citation": "TRwS 791-1 (DWA, 2015) Heizölverbraucheranlagen, Teil 1: Errichtung und Betrieb (battery connection of tanks, filling and vent lines, extraction line, overfill prevention; filling rates of 200 to 400 l/min of a tank truck)."},
    {"id": "awsv-2017", "citation": "AwSV, Verordnung über Anlagen zum Umgang mit wassergefährdenden Stoffen of 18 April 2017 (heating oil EL WGK 2; double-walled tanks need no separate collecting space; decommissioning: emptying, cleaning and degassing by a specialist company)."},
    {"id": "battery-tank-datasheets", "citation": "Manufacturer data sheets of double-walled plastic-in-steel heating-oil battery tanks of 1000 l (for example Roth DWT plus3, Schuetz plastic-in-steel battery tanks): typical format about 1640 x 1100 x 760 mm, steel jacket about 1.25 mm, PE inner tank, DN 40 filler and vent connections, 10 or 12 mm extraction line; values here are typical of the class, not of one product (authored-estimate)."},
    {"id": "din-51603-1", "citation": "DIN 51603-1:2020 Liquid fuels, fuel oils, part 1: fuel oil EL (density at 15 degC at most 860 kg/m3; 845 kg/m3 is taken as typical)."},
    {"id": "en-10255", "citation": "EN 10255:2004 Non-alloy steel tubes suitable for welding and threading (DN 40 medium series 48.3 x 3.2 mm, 3.56 kg/m black, about 3.71 kg/m galvanised)."},
    {"id": "en-1057", "citation": "EN 1057:2006+A1:2010 Copper and copper alloys, seamless round copper tubes for water and gas (12 x 1 mm, 0.308 kg/m)."},
    {"id": "en-10130", "citation": "EN 10130:2006 Cold rolled low carbon steel flat products for cold forming (DC01, yield strength at most 280 MPa; 210 MPa taken)."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes (C24: f_m,k 24 MPa, E0,mean 11 GPa, rho_mean 420 kg/m3; 450 kg/m3 taken for weathered timber at about 18 % moisture)."},
    {"id": "iso-2328", "citation": "ISO 2328:2011 Fork-lift trucks, hook-on type fork arms and fork arm carriages (class 2A fork arms 100 x 40 mm; tine spread of about 0.5 m used for the lifted pose)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (design conductivity and heat capacity: steel 50 W/mK, 450 J/kgK; PE-HD 0.4 W/mK, 1900 J/kgK; softwood 0.13 W/mK, 1600 J/kgK)."},
    {"id": "en-iso-6946", "citation": "EN ISO 6946:2017 Building components and building elements, thermal resistance (Table 8: thermal resistance of an unventilated air layer of 15 mm, horizontal heat flow, 0.17 m2K/W)."},
    {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics (periodic penetration depth; about half of it taken as the effective storage depth of an exposed timber face)."},
    {"id": "ashrae-2021", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 4, Table 5 emissivities and solar absorptances: weathered painted steel 0.9; oxidised galvanised steel about 0.28; oxidised copper about 0.6; weathered values differ from new metal (bright galvanised 0.05 to 0.1)."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, types, contents and basic principles of presentation (scales, line groups)."},
    {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, general principles of presentation, part 50: basic conventions for areas on cuts and sections."},
]

BONES = [
    {"id": "yard", "parent": None, "title": t("Yard ground (frame of the stack)", "Lagerplatz (Bezugsrahmen des Stapels)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "bearer-a", "parent": "yard", "title": t("Bearer under the base ends", "Kantholz unter den Bodenenden"), "x": 0.27, "y": 0, "angleDeg": 0, "lengthM": BEARER_SIZE, "joint": {"type": "fixed"}, "mass": {"kg": BEARER_KG, "comX": 0.05, "comY": 0.05, "distribution": "point", "source": "literature", "reference": "en-338"}, "section": {"areaM2": 0.01, "secondMomentM4": 8.333e-06, "extremeFibreM": 0.05, "material": "spruce-weathered", "title": t("Squared timber C24 100 x 100 mm", "Kantholz C24 100 x 100 mm")}},
    {"id": "bearer-b", "parent": "yard", "title": t("Bearer under the top ends", "Kantholz unter den Kopfenden"), "x": 1.27, "y": 0, "angleDeg": 0, "lengthM": BEARER_SIZE, "joint": {"type": "fixed"}, "mass": {"kg": BEARER_KG, "comX": 0.05, "comY": 0.05, "distribution": "point", "source": "literature", "reference": "en-338"}, "section": {"areaM2": 0.01, "secondMomentM4": 8.333e-06, "extremeFibreM": 0.05, "material": "spruce-weathered", "title": t("Squared timber C24 100 x 100 mm", "Kantholz C24 100 x 100 mm")}},
    {"id": "carrier", "parent": "yard", "title": t("Lift carrier (massless, raises the battery on the fork tines)", "Hubträger (masselos, hebt die Batterie auf den Gabelzinken)"), "x": 0, "y": BEAR_Y, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.1, "maxM": 0.6}},
    {"id": "battery", "parent": "carrier", "title": t("Tank battery (pivot at the base edge, tilts the tanks upright)", "Tankbatterie (Drehpunkt an der Bodenkante, richtet die Tanks auf)"), "x": BAT_X0, "y": 0, "angleDeg": 0, "lengthM": r6(L - 2 * BAT_X0), "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 90}},
]
TANK_SECTION = {"areaM2": r6(2 * (H + B) * T_STEEL), "secondMomentM4": r6(T_STEEL * H ** 3 / 6 + B * T_STEEL * H ** 2 / 2), "extremeFibreM": H / 2, "material": "steel-jacket", "title": t("Steel jacket as thin-walled box 1100 x 760 x 1.25 mm", "Stahlmantel als dünnwandiger Kasten 1100 x 760 x 1,25 mm")}
for i, (tid, z) in enumerate(TANK_Z.items(), 1):
    BONES.append({"id": tid, "parent": "battery", "title": t(f"Tank {i}", f"Tank {i}"), "x": -BAT_X0, "y": 0, "z": z, "angleDeg": 0, "lengthM": L, "joint": {"type": "fixed"}, "mass": {"kg": TANK_KG, "comX": L / 2, "comY": H / 2, "distribution": "uniform", "source": "authored-estimate", "reference": "battery-tank-datasheets"}, "section": TANK_SECTION})
BONES.append({"id": "lines", "parent": "battery", "title": t("Battery connection kit: filler, vent and extraction lines", "Batterieverbindung: Füll-, Lüftungs- und Entnahmeleitung"), "x": r6(L - BAT_X0), "y": 0, "angleDeg": 0, "lengthM": LINES_LEN, "joint": {"type": "fixed"}, "mass": {"kg": LINES_KG, "comX": PIPE_U, "comY": 0.45, "distribution": "point", "source": "literature", "reference": "en-10255"}, "section": {"areaM2": 0.000909, "secondMomentM4": 2.27e-07, "extremeFibreM": PIPE_R, "material": "galvanised-steel", "title": t("Two DN 40 tubes 48.3 x 3.2 mm", "Zwei Rohre DN 40 48,3 x 3,2 mm")}})


def anchor(id_, bone, x, y, kind, en, de, z=None, **kw):
    """📍 One anchor."""
    a = {"id": id_, "bone": bone, "x": r6(x), "y": r6(y)}
    if z is not None:
        a["z"] = r6(z)
    a.update({"kind": kind, "title": t(en, de)})
    a.update(kw)
    return a


SUP = {"support": "contact", "normalDeg": 90, "frictionCoefficient": 0.5}
ANCHORS = []
for bid, nm_en, nm_de in [("bearer-a", "base-end bearer", "Kantholz Bodenende"), ("bearer-b", "top-end bearer", "Kantholz Kopfende")]:
    for side, z in (("l", 1.15), ("r", -1.15)):
        ANCHORS.append(anchor(f"{bid}-ground-{side}", bid, 0.05, 0, "support", f"Ground under the {nm_en}, {'left' if side == 'l' else 'right'}", f"Boden unter dem {nm_de}, {'links' if side == 'l' else 'rechts'}", z, footprintLengthM=0.1, footprintWidthM=0.2, **SUP))
for fid, u in (("fork-a", 0.55), ("fork-b", 1.09)):
    for side, z in (("l", 0.9), ("r", -0.9)):
        ANCHORS.append(anchor(f"{fid}-{side}", "battery", u - BAT_X0, 0, "support", f"Fork tine {fid[-1].upper()} under the tanks, {'left' if side == 'l' else 'right'}", f"Gabelzinke {fid[-1].upper()} unter den Tanks, {'links' if side == 'l' else 'rechts'}", z, footprintLengthM=0.1, footprintWidthM=0.2, **SUP))
for bid, v, en, de in (("base-front", 0.0, "front", "vorn"), ("base-rear", H, "rear", "hinten")):
    for side, z in (("l", 1.15), ("r", -1.15)):
        ANCHORS.append(anchor(f"{bid}-{side}", "battery", -BAT_X0, v, "support", f"Tank base on the floor, {en} edge, {'left' if side == 'l' else 'right'}", f"Tankboden auf dem Estrich, {de}e Kante, {'links' if side == 'l' else 'rechts'}", z, footprintLengthM=0.1, footprintWidthM=0.76, **SUP))
for i, tid in enumerate(TANK_Z, 1):
    ANCHORS.append(anchor(f"oil-{tid}", tid, 0.78, H / 2, "load", f"Centre of the oil filling of tank {i}", f"Schwerpunkt der Ölfüllung von Tank {i}"))
ANCHORS.append(anchor("vent-outlet", "lines", PIPE_U, 0.55, "emitter", "Open end of the vent line (reconnected to the vent hood when reinstalled)", "Offenes Ende der Lüftungsleitung (im Wiedereinbau an die Lüftungshaube angeschlossen)", PIPE_Z + 0.04))

REGIONS_ALL = ["tank-jacket", "pipework", "bearers"]


def bodies(scale):
    """🧊 The body solids of one scale."""
    out = []
    for bid in BEARERS:
        out.append(boxs(f"{bid}-timber", bid, "spruce-weathered", 0.5, [BEARER_SIZE, BEARER_SIZE, BEARER_LEN], [0.05, 0.05, 0], 0.003 if scale == 50 else None))
    for tid in TANK_Z:
        out.append(boxs(f"{tid}-jacket", tid, "steel-jacket", 0.78, [L, H, B], [L / 2, H / 2, 0], R))
        if scale == 50:
            for k, ub in enumerate(BEADS):
                out.append(boxs(f"{tid}-bead-{k + 1}", tid, "steel-jacket", 0.72, [BEAD_W, H + 2 * BEAD_P, B + 2 * BEAD_P], [ub, H / 2, 0], BEAD_W / 2))
        for name, v, rad, ln in SOCKETS:
            if scale == 500 or (scale == 200 and name == "extraction"):
                continue
            mat = "galvanised-steel"
            out.append(cyl(f"{tid}-socket-{name}", tid, mat, 0.45, [1, 0, 0], [L, v, 0], 0, ln, rad))
    if scale == 50:
        out.append(boxs("tank-3-type-plate", "tank-3", "steel-jacket", 0.3, [0.10, 0.06, T_STEEL], [1.43, 0.15, -B / 2 - T_STEEL / 2]))
    if scale == 500:
        out.append(solid_body("line-kit", "lines", "galvanised-steel", 0.55, "extrude", profile=[[KIT_U0, 0.276], [LINES_LEN, 0.276], [LINES_LEN, KIT_V1], [KIT_U0, KIT_V1]], plane="xy", **{"from": -0.97, "to": 0.97}))
        return out
    zr = PIPE_Z + (0.04 if scale == 200 else 0)
    for name, v in (("filler", 0.30), ("vent", 0.55)):
        out.append(cyl(f"{name}-line", "lines", "galvanised-steel", 0.55, [0, 0, 1], [PIPE_U, v, 0], -zr, zr, PIPE_R))
    if scale in (50, 100):
        out.append(cyl("extraction-line", "lines", "copper-oxidised", 0.35, [0, 0, 1], [CU_U, CU_V, 0], -PIPE_Z, PIPE_Z, CU_R, **({"slender": True} if scale == 100 else {})))
        out.append(cyl("filler-cap-red", "lines", "pe-cap", 0.4, [0, 0, 1], [PIPE_U, 0.30, 0], PIPE_Z, PIPE_Z + 0.04, 0.03))
        out.append(cyl("filler-cap-steel", "lines", "galvanised-steel", 0.5, [0, 0, 1], [PIPE_U, 0.30, 0], -PIPE_Z - 0.03, -PIPE_Z, 0.028))
        out.append(cyl("vent-cap-red", "lines", "pe-cap", 0.4, [0, 0, 1], [PIPE_U, 0.55, 0], PIPE_Z, PIPE_Z + 0.04, 0.03))
        out.append(cyl("vent-cap-steel", "lines", "galvanised-steel", 0.5, [0, 0, 1], [PIPE_U, 0.55, 0], -PIPE_Z - 0.03, -PIPE_Z, 0.028))
    return out


def shell(tid, inset, thick, material, layer, tone):
    """🥡 One closed rectangular shell of a tank wall layer: a tube along u with two end plates."""
    outer = rounded_rect(inset, H - inset, -B / 2 + inset, B / 2 - inset, max(R - inset, 0.006))
    inner = rounded_rect(inset + thick, H - inset - thick, -B / 2 + inset + thick, B / 2 - inset - thick, max(R - inset - thick, 0.005))
    base = {"bone": tid, "material": material, "role": "body", "tone": tone, "kind": "extrude", "plane": "yz", "region": "tank-jacket", "layer": layer}
    return [
        dict(base, id=f"th-{tid}-{layer}-wall", profile=outer, holes=[inner], **{"from": r6(inset + thick), "to": r6(L - inset - thick)}),
        dict(base, id=f"th-{tid}-{layer}-base", profile=outer, **{"from": r6(inset), "to": r6(inset + thick)}),
        dict(base, id=f"th-{tid}-{layer}-top", profile=outer, **{"from": r6(L - inset - thick), "to": r6(L - inset)}),
    ]


def thermals(scale, body):
    """🌡️ The thermal solids of one scale."""
    out = []
    for tid in TANK_Z:
        if scale == 50:
            out += shell(tid, 0.0, T_STEEL, "steel-jacket", "steel-jacket", 0.78)
            out += shell(tid, T_STEEL, T_GAP, "interstitial-air", "interstitial", 0.9)
            out += shell(tid, T_STEEL + T_GAP, T_PE, "pe-hd", "pe-inner", 0.85)
        else:
            n = {100: 3, 200: 2}.get(scale)
            prof = rounded_rect(0, H, -B / 2, B / 2, R, n) if n else [[0, -B / 2], [H, -B / 2], [H, B / 2], [0, B / 2]]
            out.append({"id": f"th-{tid}", "kind": "extrude", "bone": tid, "material": "steel-jacket", "role": "body", "tone": 0.78, "profile": prof, "plane": "yz", "from": 0, "to": L, "region": "tank-jacket", "layer": "steel-jacket"})
    for s in body:
        if s["bone"] == "lines" or "socket" in s["id"]:
            out.append({"id": f"th-{s['id']}", "of": s["id"], "region": "pipework"})
        elif s["bone"] in BEARERS:
            out.append({"id": f"th-{s['id']}", "of": s["id"], "region": "bearers"})
    pr, pl = {50: (0.015, 0.03), 100: (0.02, 0.04), 200: (0.024, 0.05), 500: (0.04, 0.08)}[scale]
    zr = PIPE_Z + (0.04 if scale in (50, 100, 200) else 0.0) + (0.0 if scale != 500 else 0.04)
    out.append(dict(cyl("th-vent-outflow", "lines", "galvanised-steel", 0.6, [0, 0, 1], [PIPE_U if scale != 500 else LINES_LEN / 2, 0.55, 0], zr, zr + pl, pr), region="pipework", tags=["vent-displaced-air"]))
    return out


def structures(scale, body):
    """🏗️ The structural solids of one scale."""
    out = []
    for s in body:
        sid = s["id"]
        if sid == "tank-2-jacket":
            out.append({"id": "st-battery", "of": sid, "member": "battery", "part": "member"})
            out.append({"id": f"st-{sid}", "of": sid, "member": "tank-2", "part": "member"})
        elif sid.endswith("-jacket"):
            out.append({"id": f"st-{sid}", "of": sid, "member": s["bone"], "part": "member"})
        elif s["bone"] in BEARERS:
            out.append({"id": f"st-{sid}", "of": sid, "member": s["bone"], "part": "member"})
        elif s["bone"] == "lines" and "cap" not in sid:
            out.append({"id": f"st-{sid}", "of": sid, "member": "lines", "part": "member"})
        elif "socket" in sid:
            out.append({"id": f"st-{sid}", "of": sid, "member": "lines", "part": "joint"})
    side = {50: 0.02, 100: 0.03, 200: 0.04, 500: 0.08}[scale]
    for tid, z in TANK_Z.items():
        out.append(dict(boxs(f"st-pivot-{tid}", "battery", "steel-jacket", 0.3, [side, side, 0.70], [0.04 - BAT_X0 + side / 2, 0.04 + side / 2, z]), member="battery", part="joint"))
    return out


def tank_front_plan(scale, tid, z):
    """🛢️ Front and plan shapes of one tank and its sockets (extruded solids, so every pose reads right)."""
    out = [shape(f"{tid}-jacket", tid, "body", 2, True, 0.78, rect(0, 0, L, H), region="tank-jacket", material="steel-jacket", solid=(z - B / 2, z + B / 2))]
    if scale == 50:
        out.insert(0, shape(f"{tid}-bead", tid, "detail", 1.5, True, 0.72, rect(BEADS[-1] - BEAD_W / 2, -BEAD_P, BEAD_W, H + 2 * BEAD_P), region="tank-jacket", material="steel-jacket", solid=(z - B / 2 - BEAD_P, z + B / 2 + BEAD_P)))
    for name, v, rad, ln in SOCKETS:
        if scale == 500 or (scale == 200 and name == "extraction"):
            continue
        out.append(shape(f"{tid}-socket-{name}", tid, "body", 3, True, 0.45, rect(L, v - rad, ln, 2 * rad), region="pipework", material="galvanised-steel", solid=(z - rad, z + rad)))
    return out


def lines_shapes(scale, view):
    """🔧 The connection kit in one view."""
    if scale == 500:
        g = rect(KIT_U0, 0.276, LINES_LEN - KIT_U0, KIT_V1 - 0.276)
        if view == "elevation":
            return [shape("line-kit", "lines", "body", 4, False, 0.55, g, region="pipework", material="galvanised-steel")]
        if view == "plan":
            return [shape("line-kit", "lines", "body", 4, False, 0.55, rect(0, 0.276, LINES_LEN, KIT_V1 - 0.276), region="pipework", material="galvanised-steel", solid=(-0.97, 0.97))]
        return [shape("line-kit", "lines", "body", 4, False, 0.55, g, region="pipework", material="galvanised-steel", solid=(-0.97, 0.97))]
    zr = PIPE_Z + (0.04 if scale == 200 else 0)
    out = []
    for name, v in (("filler", 0.30), ("vent", 0.55)):
        if view == "elevation":
            if scale == 50:
                out.append(shape(f"{name}-cap-red", "lines", "body", 4, True, 0.4, circ(PIPE_U, v, 0.03), region="pipework", material="pe-cap"))
                out.append(shape(f"{name}-cap-steel", "lines", "body", 4.2, True, 0.5, circ(PIPE_U, v, 0.028), region="pipework", material="galvanised-steel"))
            elif scale == 100:
                out.append(shape(f"{name}-end", "lines", "body", 4, True, 0.45, circ(PIPE_U, v, 0.03), regions=["pipework"], material="galvanised-steel"))
            else:
                out.append(shape(f"{name}-line", "lines", "body", 4, True, 0.55, circ(PIPE_U, v, PIPE_R), region="pipework", material="galvanised-steel"))
        else:
            out.append(shape(f"{name}-line", "lines", "body", 4, True, 0.55, circ(PIPE_U, v, PIPE_R), region="pipework", material="galvanised-steel", solid=(-zr, zr)))
            if scale in (50, 100):
                out.append(shape(f"{name}-cap-red", "lines", "body", 4.1, True, 0.4, circ(PIPE_U, v, 0.03), region="pipework", material="pe-cap", solid=(PIPE_Z, PIPE_Z + 0.04)))
                out.append(shape(f"{name}-cap-steel", "lines", "body", 4.1, True, 0.5, circ(PIPE_U, v, 0.028), region="pipework", material="galvanised-steel", solid=(-PIPE_Z - 0.03, -PIPE_Z)))
    if scale in (50, 100):
        if view == "elevation":
            out.append(shape("extraction-line", "lines", "body", 4, True, 0.35, circ(CU_U, CU_V, CU_R), region="pipework", material="copper-oxidised"))
        else:
            out.append(shape("extraction-line", "lines", "body", 4, True, 0.35, circ(CU_U, CU_V, CU_R), region="pipework", material="copper-oxidised", solid=(-PIPE_Z, PIPE_Z)))
    return out


def elevation(scale):
    """📐 Side elevation (seen from the right, tank 3 nearest)."""
    out = []
    for bid in BEARERS:
        out.append(shape(f"{bid}-timber", bid, "body", 1, True, 0.5, rect(0, 0, BEARER_SIZE, BEARER_SIZE), region="bearers", material="spruce-weathered"))
        if scale == 50:
            out.append(shape(f"{bid}-end-grain", bid, "texture", 1.1, False, 0.3, poly([(0.012, 0.0), (0.03, 0.028), (0.032, 0.06), (0.018, 0.1)]), region="bearers"))
            out.append(shape(f"{bid}-check", bid, "detail", 1.2, False, 0.3, poly([(0.05, 0.1), (0.056, 0.074), (0.052, 0.058)]), region="bearers"))
    for k, (tid, z) in enumerate(TANK_Z.items()):
        out.append(shape(f"{tid}-jacket", tid, "body", 2 + k, True, 0.78, rect(0, 0, L, H, R if scale == 50 else None), region="tank-jacket", material="steel-jacket"))
        for name, v, rad, ln in SOCKETS:
            if scale == 500 or (scale == 200 and name == "extraction"):
                continue
            out.append(shape(f"{tid}-socket-{name}", tid, "body", 2.5 + k, True, 0.45, rect(L, v - rad, ln, 2 * rad), region="pipework", material="galvanised-steel"))
    near = "tank-3"
    if scale == 50:
        for k, ub in enumerate(BEADS):
            out.append(shape(f"{near}-bead-{k + 1}", near, "detail", 5, True, 0.72, rect(ub - BEAD_W / 2, -BEAD_P, BEAD_W, H + 2 * BEAD_P), region="tank-jacket", material="steel-jacket"))
        out.append(shape(f"{near}-type-plate", near, "detail", 5.2, True, 0.3, rect(1.38, 0.12, 0.10, 0.06), region="tank-jacket", material="steel-jacket"))
        out.append(shape(f"{near}-dent", near, "detail", 5.1, False, 0.4, poly([(0.6, 0.83), (0.64, 0.79), (0.7, 0.78), (0.75, 0.81)]), region="tank-jacket"))
        out.append(shape(f"{near}-dent-2", near, "detail", 5.1, False, 0.4, poly([(0.22, 0.24), (0.25, 0.2), (0.3, 0.2)]), region="tank-jacket"))
    if scale in (50, 100):
        out.append(shape(f"{near}-degassed-label", near, "detail", 5.3, True, 0.95, rect(0.92, 0.6, 0.148, 0.105), region="tank-jacket"))
        for k, (pts) in enumerate([[(1.64, 0.27), (1.5, 0.255), (1.36, 0.27), (1.22, 0.262)], [(1.64, 0.34), (1.56, 0.352), (1.47, 0.345)], [(1.64, 0.52), (1.55, 0.51), (1.4, 0.522), (1.3, 0.515)]]):
            out.append(shape(f"{near}-oil-streak-{k + 1}", near, "texture", 5.05, False, 0.25, poly(pts), region="tank-jacket"))
    out += lines_shapes(scale, "elevation")
    return out


def front(scale):
    """🎯 Front elevation (seen from ahead: the tank tops with the connection kit)."""
    out = []
    for k, bid in enumerate(BEARERS):
        out.append(shape(f"{bid}-timber", bid, "body", 1 + k * 0.1, True, 0.5, rect(-BEARER_LEN / 2, 0, BEARER_LEN, BEARER_SIZE), region="bearers", material="spruce-weathered"))
    if scale == 50:
        out.append(shape("bearer-b-grain", "bearer-b", "texture", 1.3, False, 0.3, poly([(-1.25, 0.036), (-0.4, 0.04), (0.5, 0.034), (1.25, 0.038)]), region="bearers"))
        out.append(shape("bearer-b-grain-2", "bearer-b", "texture", 1.3, False, 0.3, poly([(-1.25, 0.07), (-0.2, 0.066), (0.7, 0.071), (1.25, 0.068)]), region="bearers"))
    for tid, z in TANK_Z.items():
        out += tank_front_plan(scale, tid, z)
    out += lines_shapes(scale, "front")
    return out


def plan(scale):
    """🗺️ Plan (seen from above: the upper narrow faces of the lying tanks)."""
    out = []
    for bid, x in BEARERS.items():
        out.append(shape(f"{bid}-timber", bid, "body", 1, True, 0.5, rect(x, -BEARER_LEN / 2, BEARER_SIZE, BEARER_LEN), region="bearers", material="spruce-weathered"))
    for tid, z in TANK_Z.items():
        out += tank_front_plan(scale, tid, z)
    out += lines_shapes(scale, "plan")
    return out


PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

DECISIONS = {
    50: {
        "silhouette": t("Side elevation of the stack seen from the right: the nearest tank lying on edge on its 760 mm narrow face, 1640 x 1100 mm (32.8 x 22 mm on paper), with its three embossed stiffening beads, the two 100 x 100 mm bearers seen end on, the three sockets on the top end and the end views of the DN 40 filler and vent lines and the 12 mm copper extraction line. Front view: the three tank tops 760 x 1100 mm side by side at 50 mm spacing with the connection kit across them. Plan: the three upper narrow faces 1640 x 760 mm, the bearer ends standing out 60 mm on each side and the kit at the top ends.", "Seitenansicht des Stapels von rechts: der nächste Tank liegt hochkant auf seiner 760 mm schmalen Seite, 1640 x 1100 mm (32,8 x 22 mm auf Papier), mit seinen drei eingeprägten Versteifungssicken, den beiden Kanthölzern 100 x 100 mm in Stirnansicht, den drei Stutzen am Kopfende und den Stirnansichten der Füll- und Lüftungsleitung DN 40 und der Kupfer-Entnahmeleitung 12 mm. Vorderansicht: die drei Tankköpfe 760 x 1100 mm nebeneinander im Abstand von 50 mm mit der Batterieverbindung quer darüber. Draufsicht: die drei oberen Schmalseiten 1640 x 760 mm, die Kantholzenden stehen auf jeder Seite 60 mm vor, die Verbindung an den Kopfenden."),
        "retained": t("Beads proud by 4 mm (0.08 mm, drawn as filled bands so the outline steps), the type plate 100 x 60 mm, the cleaning and degassing certificate label, two dents, oil streaks below the fittings, end grain and a drying check of the bearers, the red PE caps on the cut filler and vent ends and the steel end caps on the blind ends.", "Sicken mit 4 mm Überstand (0,08 mm, als gefüllte Bänder gezeichnet, damit die Kontur springt), das Typenschild 100 x 60 mm, das Etikett der Reinigungs- und Entgasungsbescheinigung, zwei Beulen, Ölspuren unter den Anschlüssen, Hirnholz und ein Schwindriss der Kanthölzer, die roten PE-Kappen auf den abgetrennten Füll- und Lüftungsenden und die Stahlkappen auf den blinden Enden."),
        "merged": t("The two far tanks are hidden behind the nearest one in the side elevation and drawn only by their outlines and sockets; level gauge, clamping rings and T-pieces of the kit are part of the sockets and tubes.", "Die beiden hinteren Tanks liegen in der Seitenansicht hinter dem nächsten und sind nur mit Kontur und Stutzen gezeichnet; Füllstandsanzeiger, Klemmringe und T-Stücke der Verbindung sind Teil der Stutzen und Rohre."),
        "omitted": t("Thread turns of the sockets, the overfill prevention probe inside the filler socket and the screws of the type plate (all below 7.5 mm).", "Gewindegänge der Stutzen, die Grenzwertgebersonde im Füllstutzen und die Schrauben des Typenschilds (alle unter 7,5 mm)."),
        "thinElements": t("The 12 mm copper extraction line is drawn at its true diameter (0.24 mm).", "Die Kupfer-Entnahmeleitung 12 mm ist in wahrem Durchmesser gezeichnet (0,24 mm)."),
        "hatching": t("No hatch: this is a view of painted sheet steel and weathered timber; tone separates steel, galvanised tube, caps and timber.", "Keine Schraffur: Ansicht von lackiertem Stahlblech und vergrautem Holz; der Ton trennt Stahl, verzinktes Rohr, Kappen und Holz."),
        "styles": t("Outline 0.35 mm, beads, plate and dents 0.18 mm, grain and oil streaks 0.1 mm.", "Kontur 0,35 mm, Sicken, Schild und Beulen 0,18 mm, Maserung und Ölspuren 0,1 mm."),
        "overlays": t("Each tank jacket is one cell of the jacket region; sockets, tubes and caps are cells of the pipework region; the bearers are their own cells.", "Jeder Tankmantel ist eine Zelle der Mantelregion; Stutzen, Rohre und Kappen sind Zellen der Rohrregion; die Kanthölzer sind eigene Zellen."),
    },
    100: {
        "silhouette": t("The same stack at 16.4 x 11 mm per tank: tank outlines with their 30 mm rounded corners, the three sockets per tank, the filler and vent lines with their caps, the copper extraction line and the bearers seen end on; front and plan as at 1:50 without the bead rims.", "Derselbe Stapel mit 16,4 x 11 mm je Tank: Tankkonturen mit ihren 30-mm-Rundungen, die drei Stutzen je Tank, Füll- und Lüftungsleitung mit Kappen, die Kupfer-Entnahmeleitung und die Kanthölzer in Stirnansicht; Vorderansicht und Draufsicht wie 1:50 ohne die Sickenränder."),
        "retained": t("Certificate label and oil streaks on the nearest tank, both caps of each line.", "Bescheinigungsetikett und Ölspuren am nächsten Tank, beide Kappen jeder Leitung."),
        "merged": t("The 4 mm beads are merged into the jacket (0.04 mm on paper).", "Die 4-mm-Sicken gehen im Mantel auf (0,04 mm auf Papier)."),
        "omitted": t("Type plate, dents, end grain and drying check.", "Typenschild, Beulen, Hirnholz und Schwindriss."),
        "thinElements": t("The copper extraction line keeps its true 12 mm diameter (0.12 mm) as a slender part.", "Die Kupfer-Entnahmeleitung behält als schlankes Teil ihren wahren Durchmesser von 12 mm (0,12 mm)."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.25 mm, label 0.13 mm, streaks 0.1 mm.", "Kontur 0,25 mm, Etikett 0,13 mm, Ölspuren 0,1 mm."),
        "overlays": t("As at 1:50.", "Wie 1:50."),
    },
    200: {
        "silhouette": t("Three tank outlines of 8.2 x 5.5 mm, the filler and vent sockets and lines (0.24 mm tubes, caps merged into their ends) and the two bearers; front and plan show the three tanks with the 0.25 mm gaps between them.", "Drei Tankkonturen von 8,2 x 5,5 mm, Füll- und Lüftungsstutzen und -leitungen (Rohre 0,24 mm, Kappen in den Enden aufgegangen) und die beiden Kanthölzer; Vorderansicht und Draufsicht zeigen die drei Tanks mit den 0,25 mm breiten Fugen dazwischen."),
        "retained": t("The 50 mm spacing of the tanks and the 60 mm socket height that carries the lines above the tops.", "Der Abstand der Tanks von 50 mm und die Stutzenhöhe von 60 mm, die die Leitungen über den Köpfen trägt."),
        "merged": t("Caps are merged into the tube ends (tubes 40 mm longer at each end).", "Die Kappen gehen in den Rohrenden auf (Rohre an jedem Ende 40 mm länger)."),
        "omitted": t("Copper extraction line and its sockets (12 and 30 mm), label and streaks.", "Kupfer-Entnahmeleitung und ihre Stutzen (12 und 30 mm), Etikett und Ölspuren."),
        "thinElements": t("None below the 30 mm feature.", "Keine unter der kleinsten Einzelheit von 30 mm."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.18 mm.", "Kontur 0,18 mm."),
        "overlays": t("As at 1:50.", "Wie 1:50."),
    },
    500: {
        "silhouette": t("Three tank blocks of 3.3 x 2.2 mm on two bearers, the connection kit as one block 110 x 530 mm across the top ends, drawn as a closed outline in every view, because standing upright its 110 mm projection is a 0.22 mm strip at 1:500 that only the 0.13 mm pen can close; front and plan the three tanks side by side with the kit in front of them.", "Drei Tankblöcke von 3,3 x 2,2 mm auf zwei Kanthölzern, die Batterieverbindung als ein Block 110 x 530 mm quer über den Kopfenden, in jeder Ansicht als geschlossene Kontur gezeichnet, weil sein Überstand von 110 mm aufrecht stehend ein Streifen von 0,22 mm bei 1:500 ist, den nur die 0,13-mm-Feder schließen kann; Vorderansicht und Draufsicht die drei Tanks nebeneinander mit der Verbindung davor."),
        "retained": t("Overall size of the stack, the three separate tanks and the bearers that stand out at both sides.", "Gesamtgröße des Stapels, die drei getrennten Tanks und die beidseitig vorstehenden Kanthölzer."),
        "merged": t("Filler, vent and extraction lines, caps and sockets are one block of the kit (0.22 x 1.06 mm), the honest mass of the pipework at this size.", "Füll-, Lüftungs- und Entnahmeleitung, Kappen und Stutzen sind ein Block der Verbindung (0,22 x 1,06 mm), die ehrliche Masse der Leitungen in dieser Größe."),
        "omitted": t("Rounded corners, extraction line and all traces of use.", "Rundungen, Entnahmeleitung und alle Gebrauchsspuren."),
        "thinElements": t("None.", "Keine."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": t("The kit block is one cell of the pipework region; in every view it is a closed outline whose 0.13 mm pen closes the 0.22 mm strip.", "Der Verbindungsblock ist eine Zelle der Rohrregion; in jeder Ansicht ist er eine geschlossene Kontur, deren 0,13-mm-Feder den 0,22-mm-Streifen schließt."),
    },
}

VOLUME_DECISIONS = {
    50: [
        t("Body: each tank jacket is one rounded box 1640 x 1100 x 760 mm (30 mm edge radius) with three embossed stiffening beads 40 mm wide and 4 mm proud as rounded bands; three turned sockets (filler and vent 60 mm, extraction 30 mm) on the top end carry the kit; the DN 40 filler and vent tubes (48.3 mm) and the 12 mm copper extraction line run across the three tops, the cut ends closed by red PE caps and steel end caps; a 2 mm type plate on the nearest tank; two squared bearers 100 x 100 x 2500 mm.", "Körper: jeder Tankmantel ist ein gerundeter Kasten 1640 x 1100 x 760 mm (Kantenradius 30 mm) mit drei eingeprägten Versteifungssicken von 40 mm Breite und 4 mm Überstand als gerundete Bänder; drei gedrehte Stutzen (Füllen und Lüften 60 mm, Entnahme 30 mm) am Kopfende tragen die Verbindung; die Füll- und Lüftungsrohre DN 40 (48,3 mm) und die Kupfer-Entnahmeleitung 12 mm laufen über die drei Köpfe, die abgetrennten Enden mit roten PE-Kappen und Stahl-Endkappen verschlossen; ein Typenschild 2 mm am nächsten Tank; zwei Kanthölzer 100 x 100 x 2500 mm."),
        t("Thermal: the double wall of every tank as three closed nested shells, innermost first: PE-HD inner tank 5 mm, interstitial air space 15 mm, steel jacket 1.25 mm, each a tube with its base and top plate; sockets, tubes, caps and bearers reuse their body solids.", "Thermisch: die Doppelwand jedes Tanks als drei geschlossene, ineinanderliegende Schalen, innerste zuerst: PE-HD-Innenbehälter 5 mm, Luftraum 15 mm, Stahlmantel 1,25 mm, jede ein Rohr mit Boden- und Kopfplatte; Stutzen, Rohre, Kappen und Kanthölzer verwenden ihre Körper."),
        t("Structure: each tank jacket is the member of its tank bone, the bearers the members of theirs, the two tubes and the copper line the members of the kit with the sockets as its joints to the tanks; a joint block at the base edge of every tank marks the pivot of the battery.", "Tragwerk: jeder Tankmantel ist der Stab seines Tankknochens, die Kanthölzer die Stäbe der ihren, die beiden Rohre und die Kupferleitung die Stäbe der Verbindung mit den Stutzen als Gelenke zu den Tanks; ein Gelenkblock an der Bodenkante jedes Tanks markiert den Drehpunkt der Batterie."),
    ],
    100: [
        t("Body: as 1:50 without the beads (4 mm) and the type plate (2 mm), both below the 15 mm feature; the copper line keeps its true 12 mm section as a slender solid.", "Körper: wie 1:50 ohne die Sicken (4 mm) und das Typenschild (2 mm), beide unter der kleinsten Einzelheit von 15 mm; die Kupferleitung behält als schlanker Körper ihren wahren Querschnitt von 12 mm."),
        t("Thermal: each jacket is one solid of the jacket region (its outer steel layer); the nested shells of 1:50 are below the feature.", "Thermisch: jeder Mantel ist ein Körper der Mantelregion (seine äußere Stahlschicht); die Schalen von 1:50 liegen unter der kleinsten Einzelheit."),
        t("Structure: as 1:50.", "Tragwerk: wie 1:50."),
    ],
    200: [
        t("Body: tanks, bearers, filler and vent sockets and the two DN 40 tubes with the caps merged into their ends; the copper line and its 30 mm sockets are left out (12 mm, below the 30 mm feature).", "Körper: Tanks, Kanthölzer, Füll- und Lüftungsstutzen und die beiden Rohre DN 40 mit den Kappen in ihren Enden; die Kupferleitung und ihre 30-mm-Stutzen entfallen (12 mm, unter der kleinsten Einzelheit von 30 mm)."),
        t("Thermal: as 1:100.", "Thermisch: wie 1:100."),
        t("Structure: as 1:100 without the copper line.", "Tragwerk: wie 1:100 ohne die Kupferleitung."),
    ],
    500: [
        t("Body: three tank boxes, two bearers and the kit as one block 110 x 530 x 1940 mm standing on the tank tops; sockets and tubes (48 and 60 mm) are below the 75 mm feature and merge into it.", "Körper: drei Tankkästen, zwei Kanthölzer und die Verbindung als ein Block 110 x 530 x 1940 mm auf den Tankköpfen; Stutzen und Rohre (48 und 60 mm) liegen unter der kleinsten Einzelheit von 75 mm und gehen darin auf."),
        t("Thermal: jackets, bearers and the kit block reuse their body solids.", "Thermisch: Mäntel, Kanthölzer und der Verbindungsblock verwenden ihre Körper."),
        t("Structure: the kit block is the member of the kit and sits directly on the tank tops.", "Tragwerk: der Verbindungsblock ist der Stab der Verbindung und sitzt direkt auf den Tankköpfen."),
    ],
}


def representation(scale):
    """🖨️ The authored representation of one scale."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}, "decisions": DECISIONS[scale]}


def volume(scale):
    """🧊 The authored volume of one scale."""
    body = bodies(scale)
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermals(scale, body), "structure": structures(scale, body), "decisions": VOLUME_DECISIONS[scale]}


def bind(sun_tank, sun_pipe, sun_bear, exp_bear, contact=None):
    """🌬️ Region bindings of one pose."""
    regions = {"tank-jacket": {"exposedFraction": 0.97, "sunlitFraction": sun_tank}, "pipework": {"exposedFraction": 1, "sunlitFraction": sun_pipe}, "bearers": {"exposedFraction": exp_bear, "sunlitFraction": sun_bear}}
    if contact:
        regions["tank-jacket"]["contact"] = contact
    return regions


def oil_loads():
    """🛢️ The weight of the oil filling of every tank."""
    return [{"id": f"oil-{tid}", "kind": "carried", "title": t(f"Heating oil EL in tank {i}, 1000 l, {OIL_KG * G / 1000:.2f} kN", f"Heizöl EL in Tank {i}, 1000 l, {dez(f'{OIL_KG * G / 1000:.2f}')} kN"), "anchor": f"oil-{tid}", "fxN": 0, "fyN": r6(-OIL_KG * G), "source": "literature", "reference": "din-51603-1"} for i, tid in enumerate(TANK_Z, 1)]


BEARER_CONTACTS = [a["id"] for a in ANCHORS if a["id"].startswith("bearer")]
FORK_CONTACTS = [a["id"] for a in ANCHORS if a["id"].startswith("fork")]
BASE_CONTACTS = [a["id"] for a in ANCHORS if a["id"].startswith("base")]
UPRIGHT = {"carrier": {"dy": r6(BAT_X0 - BEAR_Y)}, "battery": {"angleDeg": 90}}

POSES = [
    {"id": "stored", "title": t("Stored on bearers", "Auf Kanthölzern gelagert"), "description": t("Cleaned, degassed and drained, the three tanks lie on edge on two squared bearers in the yard of the component exchange, still joined by the cut and capped battery lines.", "Gereinigt, entgast und entleert liegen die drei Tanks hochkant auf zwei Kanthölzern auf dem Hof der Bauteilbörse, noch verbunden durch die abgetrennten und verschlossenen Batterieleitungen."), "bones": {}, "ground": "authored", "contacts": BEARER_CONTACTS, "thermal": {"operating": {"filling": 0}, "regions": bind(0.4, 0.5, 0.2, 0.6), "emitters": {"vent-displaced-air": 0}}, "loads": []},
    {"id": "lifted", "title": t("Lifted on fork tines", "Auf Gabelzinken angehoben"), "description": t("A forklift has run its two tines between the bearers under the battery and raised it 500 mm; the bearers stay behind on the ground.", "Ein Gabelstapler hat seine beiden Zinken zwischen den Kanthölzern unter die Batterie geschoben und sie 500 mm angehoben; die Kanthölzer bleiben am Boden zurück."), "bones": {"carrier": {"dy": 0.5}}, "ground": "authored", "contacts": FORK_CONTACTS, "thermal": {"operating": {"filling": 0}, "regions": bind(0.4, 0.5, 0.3, 0.8), "emitters": {"vent-displaced-air": 0}}, "loads": []},
    {"id": "reinstalled-drained", "title": t("Reinstalled, still empty", "Wieder eingebaut, noch leer"), "description": t("Back in a cellar tank room, the battery stands upright on its bases, lines reconnected, before the first filling.", "Wieder in einem Kellertankraum steht die Batterie aufrecht auf ihren Böden, die Leitungen sind wieder angeschlossen, vor der ersten Befüllung."), "bones": UPRIGHT, "ground": "authored", "contacts": BASE_CONTACTS, "thermal": {"operating": {"filling": 0}, "regions": bind(0, 0, 0, 0.6, {"fraction": 0.05, "tempC": 12}), "emitters": {"vent-displaced-air": 0}}, "loads": []},
    {"id": "reinstalled", "title": t("Reinstalled and filled", "Wieder eingebaut und befüllt"), "description": t("The battery in service: each tank holds 1000 l of heating oil EL (845 kg) at cellar temperature, carried by its base on the floor.", "Die Batterie im Betrieb: jeder Tank enthält 1000 l Heizöl EL (845 kg) bei Kellertemperatur, getragen von seinem Boden auf dem Estrich."), "bones": UPRIGHT, "ground": "authored", "contacts": BASE_CONTACTS, "thermal": {"operating": {"filling": 0}, "regions": bind(0, 0, 0, 0.6, {"fraction": 0.05, "tempC": 12}), "emitters": {"vent-displaced-air": 0}}, "loads": oil_loads()},
]

CLIPS = [
    {"id": "lift", "title": t("Lift off the bearers", "Von den Kanthölzern abheben"), "description": t("The fork tines take the battery off the bearers and raise it 500 mm in four seconds; the bearers stay on the ground.", "Die Gabelzinken nehmen die Batterie von den Kanthölzern und heben sie in vier Sekunden um 500 mm an; die Kanthölzer bleiben am Boden."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1, "pose": "stored", "ease": "smooth"}, {"t": 5, "pose": "lifted", "ease": "smooth"}, {"t": 6, "pose": "lifted", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 1} for a in BEARER_CONTACTS] + [{"anchor": a, "fromS": 1, "toS": 6} for a in FORK_CONTACTS],
     "phases": [{"id": "engage", "fromS": 0, "toS": 1, "title": t("Tines engage", "Zinken greifen")}, {"id": "raise", "fromS": 1, "toS": 5, "title": t("Raise", "Anheben")}, {"id": "hold", "fromS": 5, "toS": 6, "title": t("Hold", "Halten")}],
     "events": [{"t": 1, "kind": "contact-off", "title": t("Battery leaves the bearers", "Batterie verlässt die Kanthölzer")}], "tracks": []},
    {"id": "filling", "title": t("First filling after reinstallation", "Erstbefüllung nach dem Wiedereinbau"), "description": t("A tank truck fills the three tanks through the filler line at about 400 l/min; displaced air leaves through the vent line and the oil load rises from zero to 1000 l per tank.", "Ein Tankwagen befüllt die drei Tanks über die Füllleitung mit etwa 400 l/min; die verdrängte Luft entweicht über die Lüftungsleitung, die Öllast steigt je Tank von null auf 1000 l."), "durationS": 20, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled", "ease": "linear"}, {"t": 20, "pose": "reinstalled", "ease": "linear"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 20} for a in BASE_CONTACTS],
     "phases": [{"id": "fill", "fromS": 0, "toS": 19, "title": t("Filling", "Befüllen")}, {"id": "settle", "fromS": 19, "toS": 20, "title": t("Settle", "Beruhigen")}],
     "events": [{"t": 0, "kind": "operating", "title": t("Filling starts", "Befüllung beginnt"), "target": "filling"}, {"t": 19, "kind": "operating", "title": t("Overfill prevention stops the truck", "Grenzwertgeber stoppt den Tankwagen"), "target": "filling"}],
     "tracks": [{"target": f"load:oil-{tid}", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 19, "v": 1}, {"t": 20, "v": 1}]} for tid in TANK_Z]
     + [{"target": "operating:filling", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 19, "v": 0}, {"t": 20, "v": 0}]}, {"target": "emitter:vent-displaced-air", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 19, "v": 0}, {"t": 20, "v": 0}]}]},
]

LAYERS = [
    {"id": "pe-inner", "title": t("PE-HD inner tank 5 mm", "PE-HD-Innenbehälter 5 mm"), "kind": "polymer", "material": "pe-hd", "thicknessM": T_PE, "resistanceM2KW": r6(T_PE / 0.4), "heatCapacityJM2K": round(T_PE * RHO_PE * 1900), "source": "literature", "reference": "en-iso-10456"},
    {"id": "interstitial", "title": t("Interstitial air space 15 mm", "Luftraum 15 mm"), "kind": "air-gap", "material": "interstitial-air", "thicknessM": T_GAP, "resistanceM2KW": 0.17, "heatCapacityJM2K": round(T_GAP * 1.2 * 1005), "source": "literature", "reference": "en-iso-6946"},
    {"id": "steel-jacket", "title": t("Steel jacket 1.25 mm, powder coated", "Stahlmantel 1,25 mm, pulverbeschichtet"), "kind": "metal", "material": "steel-jacket", "thicknessM": T_STEEL, "resistanceM2KW": r6(T_STEEL / 50), "heatCapacityJM2K": round(T_STEEL * RHO_STEEL * 450), "source": "literature", "reference": "en-iso-10456"},
]

AREAS = {"tank-jacket": 22.9, "pipework": 0.76, "bearers": 1.6}

THERMAL = {
    "environment": {"airTempC": 10, "meanRadiantTempC": 8, "skyTempC": -4, "groundTempC": 9, "relativeHumidity": 0.75, "windMS": 2, "windDirectionDeg": 0, "solarWM2": 350, "diffuseFraction": 0.4, "pressurePa": 101325},
    "passengers": [],
    "operating": [{"id": "filling", "title": t("Filling by tank truck", "Befüllung durch Tankwagen"), "levels": [{"v": 0, "title": t("No filling", "Keine Befüllung")}, {"v": 1, "title": t("Filling at about 400 l/min", "Befüllung mit etwa 400 l/min")}]}],
    "regions": [
        {"id": "tank-jacket", "title": t("Tank jackets (double wall)", "Tankmäntel (Doppelwand)"), "kind": "surface", "model": "passive", "material": "steel-jacket", "areaM2": AREAS["tank-jacket"], "characteristicLengthM": 0.76, "geometry": "vertical-plate", "skyViewFactor": 0.45, "layers": LAYERS, "heatCapacityJM2K": sum(x["heatCapacityJM2K"] for x in LAYERS), "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "battery-tank-datasheets"},
        {"id": "pipework", "title": t("Filler, vent and extraction lines with sockets and caps", "Füll-, Lüftungs- und Entnahmeleitung mit Stutzen und Kappen"), "kind": "surface", "model": "passive", "material": "galvanised-steel", "areaM2": AREAS["pipework"], "characteristicLengthM": 0.0483, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": round(0.0032 * RHO_STEEL * 450), "source": "authored-estimate", "reference": "en-10255"},
        {"id": "bearers", "title": t("Squared timber bearers", "Kanthölzer"), "kind": "surface", "model": "passive", "material": "spruce-weathered", "areaM2": AREAS["bearers"], "characteristicLengthM": 0.1, "geometry": "horizontal-plate-up", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": 24000, "source": "authored-estimate", "reference": "en-iso-13786"},
    ],
    "emitters": [{"id": "vent-displaced-air", "kind": "vent", "title": t("Air displaced by the incoming oil through the vent line", "Vom einströmenden Öl verdrängte Luft aus der Lüftungsleitung"), "anchor": "vent-outlet", "directionDeg": 90, "diameterM": 0.0431, "operatingKey": "filling", "volumeFlowM3S": 0.0067, "temperatureRiseK": 0, "vapourKgPerKg": 0, "source": "authored-estimate", "reference": "trws-791"}],
    "assumptions": [
        t("All three regions are passive: a tank, its lines and its bearers draw no power (0 W, stated on the jacket region).", "Alle drei Regionen sind passiv: Tank, Leitungen und Kanthölzer nehmen keine Leistung auf (0 W, an der Mantelregion angegeben)."),
        t("The double wall is three layers from inside out: PE-HD 5 mm (R 0.0125 m2K/W), unventilated interstitial air 15 mm (R 0.17 m2K/W, EN ISO 6946) and steel 1.25 mm; the contents are not part of the region, so the filled tank responds as the empty one (the 845 kg of oil per tank would slow it down by about 1.6 MJ/K).", "Die Doppelwand besteht von innen nach außen aus drei Schichten: PE-HD 5 mm (R 0,0125 m²K/W), unbelüfteter Luftraum 15 mm (R 0,17 m²K/W, EN ISO 6946) und Stahl 1,25 mm; der Inhalt ist nicht Teil der Region, der befüllte Tank reagiert daher wie der leere (die 845 kg Öl je Tank würden ihn um etwa 1,6 MJ/K verlangsamen)."),
        t("Weathered surfaces: the powder coating has lost its gloss (emissivity 0.9, solar absorptance 0.45 for light grey), the galvanised tubes are dull grey (0.28 instead of 0.05 to 0.1 when new), the spruce is weathered grey (0.65 solar absorptance).", "Bewitterte Oberflächen: die Pulverbeschichtung hat ihren Glanz verloren (Emissionsgrad 0,9, solare Absorption 0,45 für Hellgrau), die verzinkten Rohre sind mattgrau (0,28 statt 0,05 bis 0,1 im Neuzustand), die Fichte ist vergraut (solare Absorption 0,65)."),
        t("Reference case: an overcast autumn day in the yard; the reinstalled poses bind no sun and a cellar floor at 12 degC under the tank bases.", "Referenzfall: ein bedeckter Herbsttag auf dem Hof; die eingebauten Posen binden keine Sonne und einen Kellerboden von 12 °C unter den Tankböden."),
        t("Filling: a tank truck delivers about 400 l/min (TRwS 791); the same volume of air, 6.7 l/s, leaves the vent line at ambient temperature.", "Befüllung: ein Tankwagen fördert etwa 400 l/min (TRwS 791); dasselbe Luftvolumen, 6,7 l/s, verlässt die Lüftungsleitung bei Umgebungstemperatur."),
    ],
}

STRUCTURE = {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "assumptions": [
    t("Supports per pose: stored, the four ground contacts under the two bearers; lifted, four contacts on the two fork tines (ISO 2328 class 2A, spread 540 mm); reinstalled, four contacts at the front and rear base edges on the floor. The whole stack is one rigid body per pose, so in the lifted pose the bearers left on the ground are counted with the battery (22.5 kg, 6 % of the weight).", "Auflager je Pose: gelagert die vier Bodenkontakte unter den beiden Kanthölzern; angehoben vier Kontakte auf den beiden Gabelzinken (ISO 2328 Klasse 2A, Abstand 540 mm); eingebaut vier Kontakte an den vorderen und hinteren Bodenkanten auf dem Estrich. Der ganze Stapel ist je Pose ein starrer Körper; in der angehobenen Pose werden die am Boden verbliebenen Kanthölzer daher mit der Batterie gezählt (22,5 kg, 6 % des Gewichts)."),
    t(f"Masses from plate areas and densities: steel jacket {STEEL_KG:.1f} kg, PE inner tank {PE_KG:.1f} kg and {FITTINGS_KG} kg of sockets and gauge per tank ({TANK_KG} kg), bearers {BEARER_KG} kg each, connection kit {LINES_KG} kg; total {MASS} kg empty.", f"Massen aus Blechflächen und Dichten: Stahlmantel {dez(f'{STEEL_KG:.1f}')} kg, PE-Innenbehälter {dez(f'{PE_KG:.1f}')} kg und {dez(FITTINGS_KG)} kg Stutzen und Anzeiger je Tank ({dez(TANK_KG)} kg), Kanthölzer je {dez(BEARER_KG)} kg, Batterieverbindung {dez(LINES_KG)} kg; zusammen {dez(MASS)} kg leer."),
    t("Reinstalled and filled, each tank carries 1000 l of heating oil EL at 845 kg/m3 (8.29 kN) at the centre of its filling; the load rests on the base and is drawn as a point load at that centre.", "Eingebaut und befüllt trägt jeder Tank 1000 l Heizöl EL mit 845 kg/m³ (8,29 kN) im Schwerpunkt seiner Füllung; die Last ruht auf dem Boden und ist als Einzellast in diesem Schwerpunkt dargestellt."),
], "clinical": False}

ASSET = {
    "id": SLUG,
    "category": "components",
    "title": t("Used Steel Oil Storage Tank Battery", "Gebrauchte Stahl-Heizöltankbatterie"),
    "summary": t("Three used 1000 l double-walled heating-oil battery tanks, a PE-HD inner tank in a 1.25 mm steel jacket, 1640 x 1100 x 760 mm each, still joined by their cut and capped DN 40 filler and vent lines and the copper extraction line; cleaned and degassed, they lie on edge on two squared bearers, are lifted on fork tines and stand upright and filled when reinstalled.", "Drei gebrauchte doppelwandige Heizöl-Batterietanks zu 1000 l, ein PE-HD-Innenbehälter in einem Stahlmantel von 1,25 mm, je 1640 x 1100 x 760 mm, noch verbunden durch ihre abgetrennten und verschlossenen Füll- und Lüftungsleitungen DN 40 und die Kupfer-Entnahmeleitung; gereinigt und entgast liegen sie hochkant auf zwei Kanthölzern, werden auf Gabelzinken angehoben und stehen im Wiedereinbau aufrecht und befüllt."),
    "tags": ["component", "reclaimed", "used", "hvac", "heating", "plumbing", "steel", "polyethylene", "tank", "oil-tank", "storage", "double-walled"],
    "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t(
        f"Typical 1000 l plastic-in-steel battery tank: 1640 mm tall, 1100 mm long, 760 mm wide, stored on edge so the 1100 mm side stands; three tanks at 50 mm spacing on two 100 x 100 mm bearers 2.5 m long; empty mass {MASS} kg. All dimensions and masses are catalogue-typical (authored-estimate), nothing is measured.",
        f"Typischer Kunststoff-in-Stahl-Batterietank 1000 l: 1640 mm hoch, 1100 mm lang, 760 mm breit, hochkant gelagert, sodass die 1100-mm-Seite steht; drei Tanks im Abstand von 50 mm auf zwei Kanthölzern 100 x 100 mm von 2,5 m Länge; Leermasse {dez(MASS)} kg. Alle Maße und Massen sind katalogtypisch (authored-estimate), nichts ist gemessen.")},
    "dimensions": {"heightM": HEIGHT, "widthM": WIDTH, "depthM": DEPTH, "massKg": MASS},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "yard", "bones": BONES},
    "anchors": ANCHORS,
    "representations": {str(s): representation(s) for s in (50, 100, 200, 500)},
    "symbols": [],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": THERMAL,
    "structure": STRUCTURE,
    "volumes": {str(s): volume(s) for s in (50, 100, 200, 500)},
    "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}

if __name__ == "__main__":
    (HERE / "🔣️.json").write_text(json.dumps(DOC, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(f"[DEBUG] wrote {SLUG}: mass {MASS} kg, tank {TANK_KG} kg, h {HEIGHT} w {WIDTH} d {DEPTH}")
