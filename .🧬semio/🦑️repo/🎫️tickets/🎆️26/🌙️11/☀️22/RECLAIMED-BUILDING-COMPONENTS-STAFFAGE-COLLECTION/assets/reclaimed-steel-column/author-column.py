"""♻️ Writes the asset document of the reclaimed HEB 200 column (`🔣️.json` next to this file) from the hand-chosen numbers below.

@see ../../brief.md
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))

H, B, TW, TF, R = 0.200, 0.200, 0.009, 0.015, 0.018
L = 3.600
TBP, TBH = 0.025, 0.020
BPV, BPW = 0.200, 0.150
HPV, HPW = 0.120, 0.110
SHAFT0, SHAFT1 = TBP, L - TBH
ANCHOR_V, ANCHOR_W = 0.155, 0.090
WASHER_R, WASHER_T, NUT_S, NUT_M, BOLT_R, BOLT_END = 0.022, 0.004, 0.036, 0.019, 0.012, 0.020
NUT_RC = NUT_S / math.sqrt(3)
STIFF_T = 0.012
STIFF = [(TBP, 0.115), (TBP, 0.200), (0.085, 0.200), (0.175, 0.100), (0.040, 0.100)]
GUSSET = [(2.950, 0.100), (3.250, 0.100), (3.250, 0.150), (3.205, 0.158), (3.160, 0.152), (3.110, 0.165), (3.060, 0.168), (3.000, 0.176), (2.950, 0.180)]
GUSSET_T = 0.012
WEB_HOLES = [(2.350, 0.035), (2.350, -0.035)]
WEB_HOLE_R = 0.011
HEAD_HOLES = [(0.060, 0.065), (0.060, -0.065), (-0.060, 0.065), (-0.060, -0.065)]
HEAD_HOLE_R = 0.011
BEARER = 0.160
BEARER_LEN = 1.000
BEARERS = {"bearer-a": 0.700, "bearer-b": 2.900}
AXIS_Y = BEARER + H / 2
SLING_R = 0.0125
SLING_LEN = 1.000
WRAP_T, WRAP_W = 0.025, 0.050
LIFT = 0.500
RHO_STEEL, RHO_TIMBER = 7850.0, 470.0
HEB_KG_M = 61.3
AXIAL_N, WIND_N_M = 200000.0, 320.0
G = 9.81


def t(en, de):
    return {"en": en, "de": de}


def r6(x):
    return round(x + 0.0, 6)


def pts(seq):
    return [[r6(a), r6(b)] for a, b in seq]


def circle(cx, cy, r, n=16, start=0.0):
    return [(cx + r * math.cos(start + 2 * math.pi * k / n), cy + r * math.sin(start + 2 * math.pi * k / n)) for k in range(n)]


def area(poly):
    return abs(sum(poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1] for i in range(len(poly)))) / 2


def centroid(poly):
    a = 0.0
    cx = cy = 0.0
    for i in range(len(poly)):
        x0, y0 = poly[i]
        x1, y1 = poly[(i + 1) % len(poly)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a /= 2
    return cx / (6 * a), cy / (6 * a)


def rect(u0, v0, u1, v1):
    return [(u0, v0), (u1, v0), (u1, v1), (u0, v1)]


def hexagon(cx, cy, rc):
    return [(cx + rc * math.cos(math.pi / 2 + k * math.pi / 3), cy + rc * math.sin(math.pi / 2 + k * math.pi / 3)) for k in range(6)]


def circle_d(cx, cy, r):
    return f"M {r6(cx + r)} {r6(cy)} A {r6(r)} {r6(r)} 0 1 0 {r6(cx - r)} {r6(cy)} A {r6(r)} {r6(r)} 0 1 0 {r6(cx + r)} {r6(cy)} Z"


def poly_d(poly):
    return "M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in poly) + " Z"


def flange_profile(sign, tf, tw, r, segments):
    """🪜️ The (v, w) profile of one flange with its two root fillets (arcs of `segments` chords, a chamfer for 1, none for 0)."""
    vf = H / 2 - tf
    out = [(H / 2, -B / 2), (H / 2, B / 2), (vf, B / 2)]
    if segments == 0:
        out += [(vf, tw / 2), (vf, -tw / 2)]
    else:
        cv, cw = vf - r, tw / 2 + r
        out += [(vf + 0.0, cw)]
        for k in range(1, segments + 1):
            th = -math.pi / 2 * k / segments
            out.append((cv + r * math.cos(th), cw + r * math.sin(th)))
        for k in range(segments, -1, -1):
            th = -math.pi / 2 * k / segments
            out.append((cv + r * math.cos(th), -(cw + r * math.sin(th))))
    out += [(vf, -B / 2)]
    if sign < 0:
        out = [(-v, w) for v, w in reversed(out)]
    return out


def h_profile(tf, tw, chamfer):
    """🅷️ The (v, w) outline of a whole H with flanges `tf`, web `tw` and 45-degree root chamfers `chamfer`."""
    vf = H / 2 - tf
    q = [(H / 2, -B / 2), (H / 2, B / 2), (vf, B / 2)]
    if chamfer > 0:
        q += [(vf, tw / 2 + chamfer), (vf - chamfer, tw / 2), (-vf + chamfer, tw / 2), (-vf, tw / 2 + chamfer)]
    else:
        q += [(vf, tw / 2), (-vf, tw / 2)]
    q += [(-vf, B / 2), (-H / 2, B / 2), (-H / 2, -B / 2), (-vf, -B / 2)]
    if chamfer > 0:
        q += [(-vf, -tw / 2 - chamfer), (-vf + chamfer, -tw / 2), (vf - chamfer, -tw / 2), (vf, -tw / 2 - chamfer)]
    else:
        q += [(-vf, -tw / 2), (vf, -tw / 2)]
    q += [(vf, -B / 2)]
    return q


STIFF_AREA = area(STIFF)
GUSSET_AREA = area(GUSSET)
SHAFT_KG = HEB_KG_M * (SHAFT1 - SHAFT0)
GUSSET_KG = GUSSET_AREA * GUSSET_T * RHO_STEEL
WEB_HOLES_KG = len(WEB_HOLES) * math.pi * WEB_HOLE_R ** 2 * TW * RHO_STEEL
COLUMN_KG = SHAFT_KG + GUSSET_KG - WEB_HOLES_KG
PLATE_KG = 2 * BPV * 2 * BPW * TBP * RHO_STEEL
STIFF_KG = 2 * STIFF_AREA * STIFF_T * RHO_STEEL
WASHER_KG = math.pi * (WASHER_R ** 2 - BOLT_R ** 2) * WASHER_T * RHO_STEEL
NUT_KG = (3 * math.sqrt(3) / 2 * NUT_RC ** 2 - math.pi * BOLT_R ** 2) * NUT_M * RHO_STEEL
BOLT_KG = math.pi * BOLT_R ** 2 * (WASHER_T + NUT_M + BOLT_END) * RHO_STEEL
ANCHOR_KG = WASHER_KG + NUT_KG + BOLT_KG
BASE_KG = PLATE_KG + STIFF_KG + 4 * ANCHOR_KG
stiff_c = centroid(STIFF)
base_com_u = (PLATE_KG * TBP / 2 + STIFF_KG * stiff_c[0] + 4 * (WASHER_KG * (TBP + WASHER_T / 2) + NUT_KG * (TBP + WASHER_T + NUT_M / 2) + BOLT_KG * (TBP + (WASHER_T + NUT_M + BOLT_END) / 2))) / BASE_KG
HEAD_KG = (2 * HPV * 2 * HPW - len(HEAD_HOLES) * math.pi * HEAD_HOLE_R ** 2) * TBH * RHO_STEEL
head_com_u = -TBH / 2
TOTAL_KG = COLUMN_KG + BASE_KG + HEAD_KG
COM_U = (COLUMN_KG * L / 2 + BASE_KG * base_com_u + HEAD_KG * (L + head_com_u)) / TOTAL_KG
CHOKE = round(COM_U, 4)
BEARER_KG = BEARER * BEARER * BEARER_LEN * RHO_TIMBER

REFS = [
    ("din-1025-2", "DIN 1025-2:1995-11 Hot rolled I-beams, part 2: wide flange I-beams IPB (HE-B): IPB 200 h = b = 200 mm, s = 9 mm, t = 15 mm, r = 18 mm, A = 78.1 cm2, G = 61.3 kg/m, I_y = 5696 cm4, W_y = 570 cm3, I_z = 2003 cm4, U = 1.15 m2/m."),
    ("din-17100", "DIN 17100:1980-01 Steels for general structural purposes: St 37-2, yield strength 235 N/mm2 (the grade of the original hall, today EN 10025-2 S235JR)."),
    ("en-10025-2", "EN 10025-2:2019 Hot rolled products of structural steels, part 2: S235JR, density 7850 kg/m3."),
    ("en-1993-1-1", "EN 1993-1-1:2005 Eurocode 3, design of steel structures, part 1-1: E = 210 000 N/mm2."),
    ("en-1991-1-4", "EN 1991-1-4:2005 Eurocode 1, wind actions: peak velocity pressure about 0.8 kN/m2 and force coefficient 2.0 for an I-section, 0.8 x 2.0 x 0.2 m = 320 N/m as an estimate."),
    ("din-934", "DIN 934:1987 Hexagon nuts: M24, width across flats 36 mm, height 19 mm."),
    ("din-125", "DIN 125-1:1990 Plain washers: for M24, 25 x 44 x 4 mm."),
    ("en-338", "EN 338:2016 Structural timber, strength classes: C24 softwood, mean density 420 kg/m3, 470 kg/m3 used for weathered air-dry squared timber bearers."),
    ("en-1492-2", "EN 1492-2:2000+A1:2008 Textile slings, round slings of man-made fibres: violet round sling WLL 1000 kg, about 25 mm thick."),
    ("en-iso-10456", "EN ISO 10456:2007 Building materials and products, hygrothermal properties: steel 50 W/mK, 450 J/kgK; softwood 0.13 W/mK, 1600 J/kgK."),
    ("en-iso-13786", "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); half of it is taken as the storage depth of timber."),
    ("ashrae-fundamentals", "ASHRAE Handbook Fundamentals (2021), chapter 33 table 5 and chapter 14: total hemispherical emissivity and solar absorptance of building surfaces (oxidised and painted steel 0.85 to 0.95, weathered red-oxide paint absorptance about 0.8, weathered wood 0.9)."),
    ("bauteilboerse-practice", "Bauteilnetz Schweiz, Bauteilbörse Basel / Bauteilbörse Bremen: storage of steel members on squared-timber bearers in the open yard, centre of gravity marked on the flange for a balanced lift (practice described in the operators' handling notes; used as an illustrative storage scheme)."),
    ("din-iso-128-50", "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on cuts and sections (hatching)."),
    ("din-1356-1", "DIN 1356-1:1995 Building drawings, types, contents and principles of presentation (scales, line groups)."),
]

MATERIALS = [
    {"id": "steel-painted", "title": t("Steel St 37-2, weathered red-oxide paint", "Stahl St 37-2, verwitterter Rostschutzanstrich"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": RHO_STEEL, "elasticModulusPa": 210e9, "strengthPa": 235e6, "conductivityWmK": 50, "specificHeatJKgK": 450, "source": "literature", "reference": "ashrae-fundamentals"},
    {"id": "steel-rusted", "title": t("Steel St 37-2, bare and rusted (grout side)", "Stahl St 37-2, blank und angerostet (Vergussseite)"), "emissivity": 0.85, "solarAbsorptivity": 0.85, "densityKgM3": RHO_STEEL, "elasticModulusPa": 210e9, "strengthPa": 235e6, "conductivityWmK": 50, "specificHeatJKgK": 450, "source": "literature", "reference": "ashrae-fundamentals"},
    {"id": "softwood-weathered", "title": t("Weathered softwood C24 (bearers)", "Verwittertes Nadelholz C24 (Kanthölzer)"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO_TIMBER, "elasticModulusPa": 11e9, "strengthPa": 24e6, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "source": "literature", "reference": "en-338"},
    {"id": "polyester-sling", "title": t("Polyester round sling, violet", "Polyester-Rundschlinge, violett"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": 1380, "conductivityWmK": 0.2, "specificHeatJKgK": 1200, "source": "authored-estimate", "reference": "en-1492-2"},
]


def bone(id, parent, en, de, x, y, angle, length, joint, mass=None, section=None):
    out = {"id": id, "parent": parent, "title": t(en, de), "x": r6(x), "y": r6(y), "angleDeg": angle, "lengthM": length, "joint": joint}
    if mass:
        out["mass"] = mass
    if section:
        out["section"] = section
    return out


RIG = {
    "kind": "structural",
    "root": "yard",
    "bones": [
        bone("yard", None, "Yard ground frame (massless)", "Lagerplatz-Bodenrahmen (masselos)", 0, 0, 0, 0, {"type": "fixed"}),
        bone("bearer-a", "yard", "Bearer A under the base end (stays on the ground)", "Kantholz A unter dem Fußende (bleibt liegen)", BEARERS["bearer-a"], 0, 0, 0, {"type": "fixed"}),
        bone("bearer-b", "yard", "Bearer B under the head end (stays on the ground)", "Kantholz B unter dem Kopfende (bleibt liegen)", BEARERS["bearer-b"], 0, 0, 0, {"type": "fixed"}),
        bone("hoist", "yard", "Hoist travel of the column axis (massless)", "Hubweg der Stützenachse (masselos)", 0, AXIS_Y, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": -AXIS_Y, "maxM": 0.6}),
        bone("column", "hoist", "Column shaft, HEB 200 (pivots at the base for erection)", "Stützenschaft HEB 200 (dreht beim Aufrichten um den Fuß)", 0, 0, 0, L, {"type": "revolute", "minDeg": 0, "maxDeg": 90},
             mass={"kg": round(COLUMN_KG, 2), "comX": L / 2, "comY": 0, "distribution": "uniform", "source": "literature", "reference": "din-1025-2"},
             section={"areaM2": 0.00781, "secondMomentM4": 5.696e-05, "extremeFibreM": 0.1, "material": "steel-painted", "title": t("IPB 200 (HEB 200) St 37-2", "IPB 200 (HEB 200) St 37-2")}),
        bone("base-plate", "column", "Welded base plate with stiffeners and cut anchor bolts", "Angeschweißte Fußplatte mit Rippen und abgetrennten Ankerschrauben", 0, 0, 0, 0, {"type": "fixed"},
             mass={"kg": round(BASE_KG, 2), "comX": round(base_com_u, 4), "comY": 0, "distribution": "point", "source": "literature", "reference": "din-1025-2"}),
        bone("head-plate", "column", "Head plate with four bolt holes", "Kopfplatte mit vier Schraubenlöchern", L, 0, 0, 0, {"type": "fixed"},
             mass={"kg": round(HEAD_KG, 2), "comX": head_com_u, "comY": 0, "distribution": "point", "source": "literature", "reference": "din-1025-2"}),
        bone("sling", "column", "Round sling choked at the centre of gravity (massless)", "Rundschlinge im Schwerpunkt geschnürt (masselos)", CHOKE, H / 2 + SLING_R, 0, SLING_LEN, {"type": "revolute", "minDeg": -180, "maxDeg": 90}),
    ],
}

ANCHORS = [
    {"id": "rest-a", "bone": "column", "x": BEARERS["bearer-a"], "y": -H / 2, "kind": "contact", "title": t("Bottom flange on bearer A", "Unterflansch auf Kantholz A"), "support": "contact", "normalDeg": 90, "footprintLengthM": BEARER, "footprintWidthM": B, "frictionCoefficient": 0.5},
    {"id": "rest-b", "bone": "column", "x": BEARERS["bearer-b"], "y": -H / 2, "kind": "contact", "title": t("Bottom flange on bearer B", "Unterflansch auf Kantholz B"), "support": "contact", "normalDeg": 90, "footprintLengthM": BEARER, "footprintWidthM": B, "frictionCoefficient": 0.5},
    {"id": "hook", "bone": "sling", "x": SLING_LEN, "y": 0, "kind": "support", "title": t("Crane hook in the sling eye", "Kranhaken im Schlingenauge"), "support": "pin"},
    {"id": "footing", "bone": "base-plate", "x": 0, "y": 0, "kind": "support", "title": t("New footing, fixed base", "Neues Fundament, eingespannter Fuß"), "support": "fixed"},
    {"id": "cap", "bone": "head-plate", "x": 0, "y": 0, "kind": "load", "title": t("Column head, axial load of the new beam", "Stützenkopf, Normalkraft des neuen Trägers"), "support": None},
]
for a in ANCHORS:
    if a.get("support") is None:
        a.pop("support", None)

REGIONS = ["column-flanges", "column-web", "base-plate", "head-plate", "bearers", "sling"]


def shape(id, bone, role, z, filled, tone, geom, region=None, regions=None, material=None, pen=None):
    out = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        out["region"] = region
    if regions:
        out["regions"] = regions
    if material:
        out["material"] = material
    if pen:
        out["penMm"] = pen
    out.update(geom)
    return out


def P(poly, closed=True):
    return {"kind": "poly", "points": pts(poly), "closed": closed}


def RECT(x, y, w, h):
    return {"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}


def ELL(cx, cy, rx, ry=None):
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rx), "ry": r6(ry if ry is not None else rx)}


def PATH(d):
    return {"kind": "path", "d": d}


def CAP(x0, y0, x1, y1, r):
    return {"kind": "capsule", "x0": r6(x0), "y0": r6(y0), "x1": r6(x1), "y1": r6(y1), "r0": r6(r), "r1": r6(r)}


S = "steel-painted"
RS = "steel-rusted"
TW_ = "softwood-weathered"
PS = "polyester-sling"


def stiff_drawn(scale):
    if scale == 50:
        return STIFF
    if scale == 100:
        return [(0.035, 0.100), (0.035, 0.200), (0.085, 0.200), (0.175, 0.100)]
    return [(0.05, 0.100), (0.05, 0.200), (0.095, 0.200), (0.18, 0.100)]


def gusset_drawn(scale):
    if scale == 50:
        return GUSSET
    return [(2.950, 0.100), (3.250, 0.100), (3.250, 0.150), (3.100, 0.168), (2.950, 0.180)]


def plates_drawn(scale):
    return {50: (TBP, TBH), 100: (0.035, 0.035), 200: (0.05, 0.05), 500: (0.075, 0.075)}[scale]


def wrap_drawn(scale):
    return {50: WRAP_T, 100: WRAP_T, 200: 0.035, 500: WRAP_T}[scale]


def sling_half(scale):
    return {50: SLING_R, 100: 0.015, 200: 0.025, 500: SLING_R}[scale]


def anchor_elev(v, scale, tb):
    if scale == 50:
        return [(tb, v - WASHER_R), (tb + WASHER_T, v - WASHER_R), (tb + WASHER_T, v - NUT_RC), (tb + WASHER_T + NUT_M, v - NUT_RC), (tb + WASHER_T + NUT_M, v - BOLT_R), (tb + WASHER_T + NUT_M + BOLT_END, v - BOLT_R), (tb + WASHER_T + NUT_M + BOLT_END, v + BOLT_R), (tb + WASHER_T + NUT_M, v + BOLT_R), (tb + WASHER_T + NUT_M, v + NUT_RC), (tb + WASHER_T, v + NUT_RC), (tb + WASHER_T, v + WASHER_R), (tb, v + WASHER_R)]
    hw = 0.02 if scale == 100 else 0.022
    return rect(tb, v - hw, TBP + 0.043 if scale == 100 else tb + 0.043, v + hw)


def parts(scale):
    """🧩️ The parts that move with the column, as elevation profiles with their lateral span: the front and plan drawings are these profiles swept along w, so they stay true when the column is lifted or stood up."""
    tb, th = plates_drawn(scale)
    s0, s1 = tb, L - th
    sr = sling_half(scale)
    thin = {50: 0.006, 100: 0.015, 200: 0.015, 500: 0.015}[scale]

    def part(id, bone, prof, z0, z1, tone, order, region=None, regions=None, material=S, role="body", front=True, plan=True):
        return {"id": id, "bone": bone, "prof": prof, "z0": z0, "z1": z1, "tone": tone, "order": order, "region": region, "regions": regions, "material": material, "role": role, "front": front, "plan": plan}

    out = []
    if scale == 50:
        out.append(part("web", "column", rect(s0, -(H / 2 - TF), s1, H / 2 - TF), -TW / 2, TW / 2, 0.55, 1.0, region="column-web", plan=False))
        out.append(part("flange-top", "column", rect(s0, H / 2 - TF, s1, H / 2), -B / 2, B / 2, 0.45, 1.1, region="column-flanges"))
        out.append(part("flange-bottom", "column", rect(s0, -H / 2, s1, -(H / 2 - TF)), -B / 2, B / 2, 0.45, 1.1, region="column-flanges"))
    else:
        regs = ["column-flanges", "column-web", "sling"] if scale == 500 else ["column-flanges", "column-web"]
        out.append(part("shaft", "column", rect(s0, -H / 2, s1, H / 2), -B / 2, B / 2, 0.5, 1.0, regions=regs))
    out.append(part("base-plate", "base-plate", rect(0, -BPV, tb, BPV), -BPW, BPW, 0.3, 2.0, region="base-plate", material=RS))
    out.append(part("head-plate", "head-plate", rect(-th, -HPV, 0, HPV), -HPW, HPW, 0.4, 2.0, region="head-plate"))
    if scale <= 200:
        sd = stiff_drawn(scale)
        out.append(part("stiffener-a", "base-plate", sd, -thin, thin, 0.35, 2.1, region="base-plate", material=RS))
        out.append(part("stiffener-b", "base-plate", [(u, -v) for u, v in reversed(sd)], -thin, thin, 0.35, 2.1, region="base-plate", material=RS))
        nut = NUT_S / 2 if scale == 50 else (0.02 if scale == 100 else 0.022)
        for i, (sv, sw) in enumerate([(1, 1), (1, -1), (-1, 1), (-1, -1)]):
            prof = anchor_elev(sv * ANCHOR_V, scale, tb)
            out.append(part(f"anchor-{i}", "base-plate", prof if sv > 0 else prof, sw * ANCHOR_W - nut, sw * ANCHOR_W + nut, 0.15, 2.5, region="base-plate", material=RS, role="detail"))
        out.append(part("gusset-stub", "column", gusset_drawn(scale), -thin, thin, 0.5, 1.2, region="column-flanges"))
        wt = wrap_drawn(scale)
        out.append(part("sling-wrap", "column", rect(CHOKE - WRAP_W / 2, -(H / 2 + wt), CHOKE + WRAP_W / 2, H / 2 + wt), -(B / 2 + wt), B / 2 + wt, 0.25, 1.5, region="sling", material=PS))
    if scale < 500:
        out.append(part("sling-leg", "sling", rect(0, -sr, SLING_LEN, sr), -sr, sr, 0.25, 1.6, region="sling", material=PS))
    return out


def geometry(scale):
    tb, th = plates_drawn(scale)
    s0, s1 = tb, L - th
    sr = sling_half(scale)
    el, fr, pl = [], [], []
    for b, x in BEARERS.items():
        el.append(shape(b, b, "body", 0.5, True, 0.7, RECT(-BEARER / 2, 0, BEARER, BEARER), region="bearers", material=TW_))
        fr.append(shape(b, b, "body", 0.5, True, 0.7, RECT(-BEARER_LEN / 2, 0, BEARER_LEN, BEARER), region="bearers", material=TW_))
        pl.append(shape(b, b, "body", 0.5, True, 0.7, RECT(x - BEARER / 2, -BEARER_LEN / 2, BEARER, BEARER_LEN), region="bearers", material=TW_))
        if scale <= 100:
            el.append(shape(b + "-checks", b, "texture", 0.6, False, 0.4, P([(-0.05, 0.03), (-0.01, 0.07), (0.02, 0.12)], False), region="bearers"))
            pl.append(shape(b + "-grain", b, "texture", 0.6, False, 0.4, P([(x - 0.04, -0.45), (x - 0.03, 0.0), (x - 0.045, 0.45)], False), region="bearers"))
    if scale == 50:
        d = poly_d(rect(s0, -(H / 2 - TF), s1, H / 2 - TF)) + " " + " ".join(circle_d(u, v, WEB_HOLE_R) for u, v in WEB_HOLES)
        el.append(shape("web", "column", "body", 1, True, 0.55, PATH(d), region="column-web", material=S))
        el.append(shape("flange-top", "column", "body", 1.1, True, 0.45, P(rect(s0, H / 2 - TF, s1, H / 2)), region="column-flanges", material=S))
        el.append(shape("flange-bottom", "column", "body", 1.1, True, 0.45, P(rect(s0, -H / 2, s1, -(H / 2 - TF))), region="column-flanges", material=S))
        for k, sg in (("a", 1), ("b", -1)):
            el.append(shape("fillet-" + k, "column", "detail", 1.5, False, 0.3, P([(s0, sg * (H / 2 - TF - R)), (s1, sg * (H / 2 - TF - R))], False), region="column-web", pen=0.1))
        for k, (u, v) in enumerate(WEB_HOLES):
            el.append(shape(f"web-hole-rim-{k}", "column", "detail", 1.6, False, 0.2, ELL(u, v, WEB_HOLE_R), region="column-web", pen=0.1))
        el.append(shape("rust-streak-a", "column", "texture", 1.7, False, 0.35, P([(1.12, 0.06), (1.18, 0.0), (1.30, -0.04)], False), region="column-web"))
        el.append(shape("rust-streak-b", "column", "texture", 1.7, False, 0.35, P([(2.62, 0.05), (2.70, -0.02)], False), region="column-web"))
        el.append(shape("old-paint-edge", "column", "texture", 1.7, False, 0.35, P([(0.6, -0.03), (0.9, -0.025), (1.0, -0.045)], False), region="column-web"))
    else:
        el.append(shape("shaft", "column", "body", 1, True, 0.5, P(rect(s0, -H / 2, s1, H / 2)), regions=["column-flanges", "column-web"], material=S))
        if scale in (100, 200):
            for k, sg in (("a", 1), ("b", -1)):
                el.append(shape("flange-edge-" + k, "column", "detail", 1.5, False, 0.3, P([(s0, sg * (H / 2 - TF)), (s1, sg * (H / 2 - TF))], False), region="column-flanges"))
        if scale == 100:
            for k, (u, v) in enumerate(WEB_HOLES):
                el.append(shape(f"web-hole-{k}", "column", "detail", 1.6, False, 0.2, ELL(u, v, WEB_HOLE_R), region="column-web", pen=0.1))
    el.append(shape("base-plate", "base-plate", "body", 2, True, 0.3, P(rect(0, -BPV, tb, BPV)), region="base-plate", material=RS))
    el.append(shape("head-plate", "head-plate", "body", 2, True, 0.4, P(rect(-th, -HPV, 0, HPV)), region="head-plate", material=S))
    if scale <= 200:
        sd = stiff_drawn(scale)
        el.append(shape("stiffener-a", "base-plate", "body", 2.1, True, 0.3, P(sd), region="base-plate", material=RS))
        el.append(shape("stiffener-b", "base-plate", "body", 2.1, True, 0.3, P([(u, -v) for u, v in sd]), region="base-plate", material=RS))
        for k, sg in (("a", 1), ("b", -1)):
            el.append(shape("anchor-" + k, "base-plate", "detail", 2.5, True, 0.15, P(anchor_elev(sg * ANCHOR_V, scale, tb)), region="base-plate", material=RS))
        el.append(shape("gusset-stub", "column", "body", 1.2, True, 0.45, P(gusset_drawn(scale)), region="column-flanges", material=S))
        if scale == 50:
            el.append(shape("flame-cut-edge", "column", "detail", 1.3, False, 0.2, P(GUSSET[2:], False), region="column-flanges", pen=0.1))
        el.append(shape("sling-wrap", "column", "body", 1.8, True, 0.25, P(rect(CHOKE - WRAP_W / 2, -(H / 2 + wrap_drawn(scale)), CHOKE + WRAP_W / 2, H / 2 + wrap_drawn(scale))), region="sling", material=PS))
    if scale < 500:
        el.append(shape("sling-leg", "sling", "body", 1.9, True, 0.25, CAP(0, 0, SLING_LEN, 0, sr), region="sling", material=PS))
    if scale == 50:
        el.append(shape("cog-mark", "column", "detail", 1.95, False, 0.1, P([(CHOKE - 0.06, -0.03), (CHOKE + 0.06, 0.03)], False), region="column-web", pen=0.1))
        el.append(shape("cog-mark-2", "column", "detail", 1.95, False, 0.1, P([(CHOKE - 0.06, 0.03), (CHOKE + 0.06, -0.03)], False), region="column-web", pen=0.1))

    for part in parts(scale):
        geo = {"kind": "poly", "points": pts(part["prof"]), "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": r6(part["z0"]), "z1": r6(part["z1"])}}
        kw = {"region": part.get("region"), "regions": part.get("regions"), "material": part.get("material")}
        if part["front"]:
            fr.append(shape(part["id"], part["bone"], part["role"], part["order"], True, part["tone"], dict(geo), **kw))
        if part["plan"]:
            pl.append(shape(part["id"], part["bone"], part["role"], part["order"], True, part["tone"], dict(geo), **kw))
    if scale == 500:
        el[[s["id"] for s in el].index("shaft")]["regions"] = ["column-flanges", "column-web", "sling"]
    for lst in (el, fr, pl):
        for s in lst:
            if s.get("region") is None:
                s.pop("region", None)
            if s.get("regions") is None:
                s.pop("regions", None)
    return {"elevation": el, "front": fr, "plan": pl}


PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

REP_DECISIONS = {
    50: {
        "silhouette": t("The column lies on its bottom flange on two squared-timber bearers and is seen onto its web: the long H with its flange strips, the welded base plate at the left with its two stiffeners and the near pair of cut anchor bolts (washer, nut, bolt end), the head plate at the right, the flame-cut stub of a bracing gusset standing on the top flange, the violet round sling choked at the marked centre of gravity and laid along the top flange, and the two bearers end-on. Front: the end view onto the head plate with its four bolt holes, the larger base plate behind it with stiffeners and all four anchor nuts, the sling wrap and the gusset stub above, the bearers lengthwise. Plan: the top flange between base and head plate, stiffener edge, anchor nuts, gusset stub, sling wrap and leg over the two bearers.",
                            "Die Stütze liegt auf ihrem Unterflansch auf zwei Kanthölzern und wird auf den Steg gesehen: das lange H mit seinen Flanschstreifen, links die angeschweißte Fußplatte mit zwei Rippen und dem vorderen Paar abgetrennter Ankerschrauben (Scheibe, Mutter, Bolzenende), rechts die Kopfplatte, auf dem Oberflansch der brenngeschnittene Stummel eines Verbandsblechs, die violette Rundschlinge im markierten Schwerpunkt geschnürt und auf dem Oberflansch abgelegt, und die zwei Kanthölzer im Querschnitt. Vorderansicht: Blick auf die Kopfplatte mit ihren vier Schraubenlöchern, dahinter die größere Fußplatte mit Rippen und allen vier Ankermuttern, darüber Schlingenwicklung und Blechstummel, die Kanthölzer der Länge nach. Draufsicht: der Oberflansch zwischen Fuß- und Kopfplatte, Rippenkante, Ankermuttern, Blechstummel, Schlingenwicklung und -strang über den zwei Kanthölzern."),
        "retained": t("Flanges 15 mm, root-fillet tangent lines, web with the two 22 mm holes of a removed secondary-beam cleat, base plate 25 mm, head plate 20 mm, stiffeners 0.10 x 0.15 m with snipe, M24 anchors cut flush under the plate with washer (44 mm), nut (36 mm across flats, 41.6 mm across corners) and 20 mm bolt end, the ragged flame-cut edge of the gusset stub, the 25 mm round sling and its 50 mm choke wrap, the marked centre of gravity, end grain checks of the bearers.",
                          "Flansche 15 mm, Tangentenlinien der Ausrundung, Steg mit den zwei 22-mm-Löchern einer entfernten Nebenträger-Lasche, Fußplatte 25 mm, Kopfplatte 20 mm, Rippen 0,10 x 0,15 m mit Eckausschnitt, M24-Anker unter der Platte bündig abgetrennt mit Scheibe (44 mm), Mutter (SW 36, 41,6 mm über Eck) und 20 mm Bolzenende, die unregelmäßige Brennschnittkante des Blechstummels, die 25-mm-Rundschlinge und ihre 50 mm breite Schnürung, die Schwerpunktmarke, Schwundrisse der Kanthölzer."),
        "merged": t("Washer, nut and bolt end of each anchor are one drawn outline; both chambers share the web shape.", "Scheibe, Mutter und Bolzenende jedes Ankers sind ein gezeichneter Umriss; beide Kammern teilen sich die Stegform."),
        "omitted": t("Weld seams (6 mm), the far anchor pair in the elevation (hidden behind the near one), the burnt cut faces of the bolt stubs on the plate underside, mill marks and paint lettering.", "Schweißnähte (6 mm), das hintere Ankerpaar in der Ansicht (hinter dem vorderen verdeckt), die verbrannten Schnittflächen der Bolzenstummel unter der Platte, Walzzeichen und Farbbeschriftung."),
        "thinElements": t("Stiffeners and gusset (12 mm) are drawn 15 mm wide in front and plan so that they still print at 1:75; the web thickness is not seen.", "Rippen und Blechstummel (12 mm) sind in Vorder- und Draufsicht 15 mm breit gezeichnet, damit sie bei 1:75 noch drucken; die Stegdicke ist nicht sichtbar."),
        "hatching": t("No hatch: no view is a cut; worn paint and rust streaks are fine texture strokes.", "Keine Schraffur: keine Ansicht ist ein Schnitt; abgewitterter Anstrich und Rostfahnen sind feine Texturstriche."),
        "styles": t("Outline 0.35 mm, details 0.18 mm, texture 0.1 mm; flanges darker than the web, rusted base plate darkest, timber light, sling dark violet grey.", "Kontur 0,35 mm, Details 0,18 mm, Textur 0,1 mm; Flansche dunkler als der Steg, die angerostete Fußplatte am dunkelsten, Holz hell, Schlinge dunkel violettgrau."),
        "overlays": t("Flanges, web, base plate group, head plate, bearers and sling are separate overlay cells, so the sunlit top flange, the shaded chambers, the timber and the sling read apart.", "Flansche, Steg, Fußplattengruppe, Kopfplatte, Kanthölzer und Schlinge sind getrennte Überlagerungszellen; der besonnte Oberflansch, die verschatteten Kammern, das Holz und die Schlinge bleiben unterscheidbar."),
    },
    100: {
        "silhouette": t("The same lying column as one H bar with flange edge lines, base plate with stiffeners and the near anchor studs, head plate, gusset stub, sling wrap and leg, and the two bearers; front and plan as at 1:50 with the anchors as studs and without the head-plate holes.", "Dieselbe liegende Stütze als ein H-Balken mit Flanschkantenlinien, Fußplatte mit Rippen und den vorderen Ankerbolzen, Kopfplatte, Blechstummel, Schlingenwicklung und -strang und die zwei Kanthölzer; Vorder- und Draufsicht wie bei 1:50 mit den Ankern als Bolzen und ohne die Löcher der Kopfplatte."),
        "retained": t("Column length 3.60 m, section 0.20 m, the 0.40 x 0.30 m base plate and the 0.24 x 0.22 m head plate, stiffener shape, gusset stub, sling, the two web holes as thin circles, bearers 0.16 m.", "Stützenlänge 3,60 m, Querschnitt 0,20 m, Fußplatte 0,40 x 0,30 m und Kopfplatte 0,24 x 0,22 m, Rippenform, Blechstummel, Schlinge, die zwei Steglöcher als dünne Kreise, Kanthölzer 0,16 m."),
        "merged": t("Flanges and web are one bar; washer, nut and bolt end are one 40 mm stud.", "Flansche und Steg sind ein Balken; Scheibe, Mutter und Bolzenende sind ein 40-mm-Bolzen."),
        "omitted": t("Fillet lines, head-plate holes (22 mm, 0.15 mm at 1:150), rust texture, centre-of-gravity mark, weld seams.", "Ausrundungslinien, Löcher der Kopfplatte (22 mm, 0,15 mm bei 1:150), Rosttextur, Schwerpunktmarke, Schweißnähte."),
        "thinElements": t("The 25 and 20 mm plates are drawn 35 mm thick, stiffeners and gusset 30 mm wide in front and plan and the sling 30 mm, so that every filled part prints at 1:150; the model keeps the true plates.", "Die 25- und 20-mm-Platten sind 35 mm dick gezeichnet, Rippen und Blechstummel in Vorder- und Draufsicht 30 mm breit und die Schlinge 30 mm, damit jeder gefüllte Teil bei 1:150 druckt; das Modell behält die wahren Platten."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.25 mm, details 0.13 mm.", "Kontur 0,25 mm, Details 0,13 mm."),
        "overlays": t("The bar carries flanges and web merged; plates, bearers and sling are their own cells.", "Der Balken trägt Flansche und Steg zusammengefasst; Platten, Kanthölzer und Schlinge sind eigene Zellen."),
    },
    200: {
        "silhouette": t("A lying bar 3.60 x 0.20 m with base and head plate as step-outs, the stiffener triangles, the anchor blocks, the gusset stub, the sling and the two bearer squares; front: head plate over the base plate; plan: flange strip between the plates over the bearers.", "Ein liegender Balken 3,60 x 0,20 m mit Fuß- und Kopfplatte als Absätzen, den Rippendreiecken, den Ankerklötzen, dem Blechstummel, der Schlinge und den zwei Kantholzquadraten; vorn: Kopfplatte vor der Fußplatte; Draufsicht: Flanschstreifen zwischen den Platten über den Kanthölzern."),
        "retained": t("Length, section, plate sizes, stiffener outline, anchors as 44 mm blocks, gusset stub, sling, bearers.", "Länge, Querschnitt, Plattengrößen, Rippenumriss, Anker als 44-mm-Klötze, Blechstummel, Schlinge, Kanthölzer."),
        "merged": t("Flanges and web are one bar; each anchor is one block.", "Flansche und Steg sind ein Balken; jeder Anker ist ein Klotz."),
        "omitted": t("Web holes, head-plate holes, fillet lines, texture, mark.", "Steglöcher, Kopfplattenlöcher, Ausrundungslinien, Textur, Marke."),
        "thinElements": t("Plates are drawn 50 mm thick, stiffeners, gusset 30 mm and the sling 50 mm wide, the smallest filled parts that print at this scale.", "Platten sind 50 mm dick gezeichnet, Rippen und Blechstummel 30 mm und die Schlinge 50 mm breit, die kleinsten gefüllten Teile, die bei diesem Maßstab drucken."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.18 mm, details 0.1 mm.", "Kontur 0,18 mm, Details 0,1 mm."),
        "overlays": t("Bar merged, plates, bearers and sling separate.", "Balken zusammengefasst, Platten, Kanthölzer und Schlinge getrennt."),
    },
    500: {
        "silhouette": t("The bar 3.60 x 0.20 m (7.2 x 0.4 mm) with the two plates as 75 mm step-outs on its two bearers; front: the plates over the bearer; plan: the flange strip between the plates over the bearers.", "Der Balken 3,60 x 0,20 m (7,2 x 0,4 mm) mit den zwei Platten als 75-mm-Absätzen auf seinen zwei Kanthölzern; vorn: die Platten über dem Kantholz; Draufsicht: der Flanschstreifen zwischen den Platten über den Kanthölzern."),
        "retained": t("Length, section, plate sizes and the bearers.", "Länge, Querschnitt, Plattengrößen und die Kanthölzer."),
        "merged": t("Flanges, web and the sling lying on the top flange are one bar (the sling region is painted merged into it).", "Flansche, Steg und die auf dem Oberflansch liegende Schlinge sind ein Balken (die Schlingenregion ist darin zusammengefasst gemalt)."),
        "omitted": t("Stiffeners, anchors, gusset stub, choke wrap and every hole: all below 0.15 mm on paper.", "Rippen, Anker, Blechstummel, Schnürung und alle Löcher: alle unter 0,15 mm auf dem Papier."),
        "thinElements": t("The plates are drawn 75 mm thick (0.15 mm).", "Die Platten sind 75 mm dick gezeichnet (0,15 mm)."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("One dark bar on light bearers.", "Ein dunkler Balken auf hellen Kanthölzern."),
        "overlays": t("Bar merged, plates and bearers separate.", "Balken zusammengefasst, Platten und Kanthölzer getrennt."),
    },
}


def solid(id, kind, bone, material, role, tone, **kw):
    out = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(kw)
    return out


def ext(id, bone, material, role, tone, profile, plane, a, b, holes=None, **kw):
    out = solid(id, "extrude", bone, material, role, tone, profile=pts(profile), plane=plane)
    if holes:
        out["holes"] = [pts(h) for h in holes]
    out["from"] = r6(a)
    out["to"] = r6(b)
    out.update(kw)
    return out


def body(scale):
    """🧱️ The body solids of one scale."""
    out = []
    for b in BEARERS:
        out.append(solid(b, "box", b, TW_, "body", 0.7, size=[BEARER, BEARER, BEARER_LEN], cornerRadius=0.008 if scale <= 100 else 0.0, transform={"translation": [0, BEARER / 2, 0]}))
    if scale == 50:
        out.append(ext("web", "column", S, "body", 0.55, rect(SHAFT0, -(H / 2 - TF - R), SHAFT1, H / 2 - TF - R), "xy", -TW / 2, TW / 2, holes=[circle(u, v, WEB_HOLE_R) for u, v in WEB_HOLES]))
        out.append(ext("flange-top", "column", S, "body", 0.45, flange_profile(1, TF, TW, R, 6), "yz", SHAFT0, SHAFT1))
        out.append(ext("flange-bottom", "column", S, "body", 0.45, flange_profile(-1, TF, TW, R, 6), "yz", SHAFT0, SHAFT1))
        out.append(ext("base-plate", "base-plate", RS, "body", 0.3, rect(-BPV, -BPW, BPV, BPW), "yz", 0, TBP))
        for k, sg in (("a", 1), ("b", -1)):
            out.append(ext("stiffener-" + k, "base-plate", RS, "body", 0.3, [(u, sg * v) for u, v in STIFF] if sg > 0 else [(u, -v) for u, v in reversed(STIFF)], "xy", -STIFF_T / 2, STIFF_T / 2))
        for i, (sv, sw) in enumerate([(1, 1), (1, -1), (-1, 1), (-1, -1)]):
            v, w = sv * ANCHOR_V, sw * ANCHOR_W
            out.append(solid(f"anchor-bolt-{i}", "revolve", "base-plate", RS, "detail", 0.25, profile=pts([(0, WASHER_R), (WASHER_T, WASHER_R), (WASHER_T, BOLT_R), (WASHER_T + NUT_M + BOLT_END - 0.002, BOLT_R), (WASHER_T + NUT_M + BOLT_END, BOLT_R - 0.002)]), axis=[1, 0, 0], origin=[TBP, r6(v), r6(w)]))
            out.append(ext(f"anchor-nut-{i}", "base-plate", RS, "detail", 0.2, hexagon(v, w, NUT_RC), "yz", TBP + WASHER_T, TBP + WASHER_T + NUT_M, holes=[circle(v, w, BOLT_R, 12)]))
        out.append(ext("head-plate", "head-plate", S, "body", 0.4, rect(-HPV, -HPW, HPV, HPW), "yz", -TBH, 0, holes=[circle(v, w, HEAD_HOLE_R) for v, w in HEAD_HOLES]))
        out.append(ext("gusset-stub", "column", S, "body", 0.45, GUSSET, "xy", -GUSSET_T / 2, GUSSET_T / 2))
        out.append(ext("sling-wrap", "column", PS, "body", 0.25, rect(-(H / 2 + WRAP_T), -(B / 2 + WRAP_T), H / 2 + WRAP_T, B / 2 + WRAP_T), "yz", CHOKE - WRAP_W / 2, CHOKE + WRAP_W / 2, holes=[rect(-H / 2, -B / 2, H / 2, B / 2)]))
        out.append(solid("sling-leg", "capsule", "sling", PS, "body", 0.25, p0=[0, 0, 0], p1=[SLING_LEN, 0, 0], r0=SLING_R, r1=SLING_R, slender=True))
        return out
    if scale == 100:
        out.append(ext("shaft", "column", S, "body", 0.5, h_profile(TF, TW, 0.012), "yz", SHAFT0, SHAFT1))
        out.append(ext("base-plate", "base-plate", RS, "body", 0.3, rect(-BPV, -BPW, BPV, BPW), "yz", 0, TBP))
        sp = [(TBP, 0.100), (TBP, 0.200), (0.085, 0.200), (0.175, 0.100)]
        out.append(ext("stiffener-a", "base-plate", RS, "body", 0.3, sp, "xy", -0.0075, 0.0075))
        out.append(ext("stiffener-b", "base-plate", RS, "body", 0.3, [(u, -v) for u, v in reversed(sp)], "xy", -0.0075, 0.0075))
        for i, (sv, sw) in enumerate([(1, 1), (1, -1), (-1, 1), (-1, -1)]):
            out.append(solid(f"anchor-{i}", "revolve", "base-plate", RS, "detail", 0.25, profile=pts([(0, 0.02), (0.043, 0.02)]), axis=[1, 0, 0], origin=[TBP, r6(sv * ANCHOR_V), r6(sw * ANCHOR_W)]))
        out.append(ext("head-plate", "head-plate", S, "body", 0.4, rect(-HPV, -HPW, HPV, HPW), "yz", -TBH, 0))
        out.append(ext("gusset-stub", "column", S, "body", 0.45, gusset_drawn(100), "xy", -0.0075, 0.0075))
        out.append(ext("sling-wrap", "column", PS, "body", 0.25, rect(-(H / 2 + WRAP_T), -(B / 2 + WRAP_T), H / 2 + WRAP_T, B / 2 + WRAP_T), "yz", CHOKE - WRAP_W / 2, CHOKE + WRAP_W / 2, holes=[rect(-H / 2, -B / 2, H / 2, B / 2)]))
        out.append(solid("sling-leg", "capsule", "sling", PS, "body", 0.25, p0=[0, 0, 0], p1=[SLING_LEN, 0, 0], r0=SLING_R, r1=SLING_R, slender=True))
        return out
    if scale == 200:
        out.append(ext("shaft", "column", S, "body", 0.5, h_profile(0.03, 0.03, 0.0), "yz", 0.03, L - 0.03))
        out.append(ext("base-plate", "base-plate", RS, "body", 0.3, rect(-BPV, -BPW, BPV, BPW), "yz", 0, 0.03))
        sp = [(0.03, 0.100), (0.03, 0.200), (0.09, 0.200), (0.18, 0.100)]
        out.append(ext("stiffener-a", "base-plate", RS, "body", 0.3, sp, "xy", -0.015, 0.015))
        out.append(ext("stiffener-b", "base-plate", RS, "body", 0.3, [(u, -v) for u, v in reversed(sp)], "xy", -0.015, 0.015))
        for i, (sv, sw) in enumerate([(1, 1), (1, -1), (-1, 1), (-1, -1)]):
            out.append(solid(f"anchor-{i}", "box", "base-plate", RS, "detail", 0.25, size=[0.043, 0.044, 0.044], transform={"translation": [0.03 + 0.0215, r6(sv * ANCHOR_V), r6(sw * ANCHOR_W)]}))
        out.append(ext("head-plate", "head-plate", S, "body", 0.4, rect(-HPV, -HPW, HPV, HPW), "yz", -0.03, 0))
        out.append(ext("gusset-stub", "column", S, "body", 0.45, gusset_drawn(200), "xy", -0.015, 0.015))
        out.append(ext("sling-wrap", "column", PS, "body", 0.25, rect(-(H / 2 + WRAP_T), -(B / 2 + WRAP_T), H / 2 + WRAP_T, B / 2 + WRAP_T), "yz", CHOKE - WRAP_W / 2, CHOKE + WRAP_W / 2, holes=[rect(-H / 2, -B / 2, H / 2, B / 2)]))
        out.append(solid("sling-leg", "capsule", "sling", PS, "body", 0.25, p0=[0, 0, 0], p1=[SLING_LEN, 0, 0], r0=SLING_R, r1=SLING_R, slender=True))
        return out
    out.append(ext("shaft", "column", S, "body", 0.5, h_profile(0.075, 0.075, 0.0), "yz", 0.075, L - 0.075))
    out.append(ext("base-plate", "base-plate", RS, "body", 0.3, rect(-BPV, -BPW, BPV, BPW), "yz", 0, 0.075))
    out.append(ext("head-plate", "head-plate", S, "body", 0.4, rect(-HPV, -HPW, HPV, HPW), "yz", -0.075, 0))
    out.append(solid("sling-leg", "capsule", "sling", PS, "body", 0.25, p0=[0, 0, 0], p1=[SLING_LEN, 0, 0], r0=SLING_R, r1=SLING_R, slender=True))
    return out


REGION_OF = {"bearer-a": "bearers", "bearer-b": "bearers", "web": "column-web", "flange-top": "column-flanges", "flange-bottom": "column-flanges", "gusset-stub": "column-flanges", "head-plate": "head-plate", "sling-wrap": "sling", "sling-leg": "sling"}


def thermal(scale, bodies):
    out = []
    for s in bodies:
        rid = REGION_OF.get(s["id"], "base-plate" if s["bone"] == "base-plate" else None)
        if s["id"] == "shaft":
            if scale == 100:
                tf, tw, ch, a, b = TF, TW, 0.012, SHAFT0, SHAFT1
            else:
                tf = tw = {200: 0.03, 500: 0.075}[scale]
                ch, a, b = 0.0, tf if scale == 500 else 0.03, L - (tf if scale == 500 else 0.03)
            for k, sg in (("top", 1), ("bottom", -1)):
                prof = rect(H / 2 - tf, -B / 2, H / 2, B / 2) if sg > 0 else rect(-H / 2, -B / 2, -(H / 2 - tf), B / 2)
                out.append(ext("th-flange-" + k, "column", S, "body", 0.45, prof, "yz", a, b, region="column-flanges"))
            hv = max(H / 2 - tf - ch, {100: 0.0075, 200: 0.015, 500: 0.0375}[scale])
            web = rect(-hv, -max(tw, {100: 0.015, 200: 0.03, 500: 0.075}[scale]) / 2, hv, max(tw, {100: 0.015, 200: 0.03, 500: 0.075}[scale]) / 2)
            out.append(ext("th-web", "column", S, "body", 0.55, web, "yz", a, b, region="column-web"))
            continue
        out.append({"id": "th-" + s["id"], "of": s["id"], "region": rid})
    return out


def structure(scale, bodies):
    out = []
    for s in bodies:
        i = s["id"]
        if i in ("web", "flange-top", "flange-bottom", "shaft"):
            out.append({"id": "st-" + i, "of": i, "member": "column", "part": "member"})
        elif i == "sling-leg":
            out.append({"id": "st-sling-leg", "of": i, "member": "sling", "part": "member"})
        elif i == "sling-wrap":
            out.append({"id": "st-sling-wrap", "of": i, "member": "sling", "part": "joint"})
        elif s["bone"] in BEARERS:
            out.append({"id": "st-" + i, "of": i, "member": "column", "part": "joint"})
        elif s["bone"] == "base-plate":
            out.append({"id": "st-" + i, "of": i, "member": "base-plate", "part": "joint"})
        elif s["bone"] == "head-plate":
            out.append({"id": "st-" + i, "of": i, "member": "head-plate", "part": "joint"})
        elif i == "gusset-stub":
            out.append({"id": "st-" + i, "of": i, "member": "column", "part": "joint"})
    rm = {50: 0.03, 100: 0.04, 200: 0.05, 500: 0.08}[scale]
    out.append(solid("mass-column", "ellipsoid", "column", S, "detail", 0.1, radii=[rm, rm, rm], transform={"translation": [L / 2, 0, 0]}, member="column", part="mass"))
    out.append(solid("mass-base-plate", "ellipsoid", "base-plate", RS, "detail", 0.1, radii=[rm, rm, rm], transform={"translation": [r6(max(base_com_u, rm)), 0, 0]}, member="base-plate", part="mass"))
    out.append(solid("mass-head-plate", "ellipsoid", "head-plate", S, "detail", 0.1, radii=[rm, rm, rm], transform={"translation": [r6(-max(-head_com_u, rm)), 0, 0]}, member="head-plate", part="mass"))
    return out


VOL_DECISIONS = {
    50: [
        t(f"The 1:50 body is the column as it came out of the hall. The shaft is the rolled IPB 200 in three solids: the web (9 mm) with the two 22 mm holes of a removed secondary-beam cleat at 2.35 m, and the two flanges (15 mm) with their 18 mm root fillets as six chords each, 3.555 m between the plates. The base plate is 400 x 300 x 25 mm, rusted from the grout, with two 12 mm stiffeners in the web plane (0.10 x 0.15 m with a 15 mm snipe); the four M24 anchor bolts were flame-cut flush under the plate, so each keeps its washer, its hexagon nut (36 mm across flats) and a 20 mm bolt end above the plate (washer and bolt are one turned solid, the nut has the bolt as its bore). The head plate is 240 x 220 x 20 mm with four 22 mm holes. A 12 mm bracing gusset was flame-cut 50 to 80 mm above the top flange at 2.95 to 3.25 m and its ragged edge stays. The violet round sling (25 mm) is choked at the marked centre of gravity, {CHOKE:.3f} m from the base, with a 50 mm wrap, and its 1.0 m leg lies along the top flange. The two bearers are 160 x 160 x 1000 mm squared timber at 0.70 and 2.90 m.",
          f"Der 1:50-Körper ist die Stütze, wie sie aus der Halle kam. Der Schaft ist das gewalzte IPB 200 in drei Körpern: der Steg (9 mm) mit den zwei 22-mm-Löchern einer entfernten Nebenträgerlasche bei 2,35 m und die zwei Flansche (15 mm) mit ihren 18-mm-Ausrundungen als je sechs Sehnen, 3,555 m zwischen den Platten. Die Fußplatte ist 400 x 300 x 25 mm, vom Verguss angerostet, mit zwei 12-mm-Rippen in Stegebene (0,10 x 0,15 m mit 15-mm-Eckausschnitt); die vier M24-Ankerschrauben wurden unter der Platte bündig brenngeschnitten, jede behält Scheibe, Sechskantmutter (SW 36) und 20 mm Bolzenende über der Platte (Scheibe und Bolzen sind ein Drehkörper, die Mutter hat den Bolzen als Bohrung). Die Kopfplatte ist 240 x 220 x 20 mm mit vier 22-mm-Löchern. Ein 12-mm-Verbandsblech wurde 50 bis 80 mm über dem Oberflansch bei 2,95 bis 3,25 m abgebrannt, seine unregelmäßige Kante bleibt. Die violette Rundschlinge (25 mm) ist im markierten Schwerpunkt {CHOKE:.3f} m über dem Fuß mit 50 mm Wicklung geschnürt, ihr 1,0 m langer Strang liegt auf dem Oberflansch. Die zwei Kanthölzer sind 160 x 160 x 1000 mm bei 0,70 und 2,90 m."),
        t("Thermal: column-flanges is both flanges with fillets and the gusset stub, column-web the web with its holes, base-plate the plate, stiffeners and anchors (its underside faces the open yard while stored), head-plate the plate with its holes, bearers the two timbers, sling the wrap and leg, all by reference to the body.", "Thermik: column-flanges sind beide Flansche mit Ausrundungen und der Blechstummel, column-web der Steg mit seinen Löchern, base-plate Platte, Rippen und Anker (ihre Unterseite zeigt beim Lagern in den offenen Hof), head-plate die Platte mit ihren Löchern, bearers die zwei Hölzer, sling Wicklung und Strang, alle als Verweis auf den Körper."),
        t("Structure: the three shaft solids are the member of the column bone, the base plate group and the head plate are joints on their own bones, the bearers are the joints that carry the bottom flange to the ground at the two contact anchors, the sling leg is the member of the sling bone with its wrap as joint, and three mass points mark the shaft (uniform, its centre at 1.80 m), the base plate group and the head plate.", "Statik: die drei Schaftkörper sind der Stab des Stützenknochens, Fußplattengruppe und Kopfplatte sind Knoten auf eigenen Knochen, die Kanthölzer sind die Knoten, die den Unterflansch an den zwei Kontaktankern in den Boden tragen, der Schlingenstrang ist der Stab des Schlingenknochens mit seiner Wicklung als Knoten, und drei Massepunkte markieren den Schaft (gleichmäßig, Mitte bei 1,80 m), die Fußplattengruppe und die Kopfplatte."),
    ],
    100: [
        t("At 1:100 the shaft is one H extrusion with the true 15 mm flanges and 9 mm web, each 18 mm fillet one 12 mm chamfer; the web holes and the head-plate holes (22 mm, 0.15 mm at 1:150) are left out. Plates keep their true 25 and 20 mm, stiffeners and gusset are 15 mm (the feature; 12 mm real) without snipe and with a straight cut, each anchor is one 40 mm stud of washer, nut and bolt end, and sling, wrap and bearers stay as at 1:50 without rounding below the feature.", "Bei 1:100 ist der Schaft eine H-Extrusion mit den wahren 15-mm-Flanschen und 9-mm-Steg, jede 18-mm-Ausrundung eine 12-mm-Fase; Steg- und Kopfplattenlöcher (22 mm, 0,15 mm bei 1:150) entfallen. Die Platten behalten ihre wahren 25 und 20 mm, Rippen und Blechstummel sind 15 mm (das Merkmal; 12 mm real) ohne Eckausschnitt und mit geradem Schnitt, jeder Anker ist ein 40-mm-Bolzen aus Scheibe, Mutter und Bolzenende, Schlinge, Wicklung und Kanthölzer bleiben wie bei 1:50."),
        t("Thermal: the H as two flange plates and the web between them for the two shaft regions, everything else by reference.", "Thermik: das H als zwei Flanschplatten und der Steg dazwischen für die zwei Schaftregionen, alles andere als Verweis."),
        t("Structure: the shaft as the column member, plates and bearers as joints, the sling leg as member with the wrap as joint, three mass points.", "Statik: der Schaft als Stützenstab, Platten und Kanthölzer als Knoten, der Schlingenstrang als Stab mit der Wicklung als Knoten, drei Massepunkte."),
    ],
    200: [
        t("At 1:200 the shaft is a plain H with 30 mm flanges and a 30 mm web (the feature; 15 and 9 mm real) and sharp roots between 30 mm plates (25 and 20 mm real). The stiffeners are 30 mm plates of the true outline, each anchor a 44 mm block, the gusset stub a 30 mm plate with a straight cut; wrap, sling and bearers as at 1:100.", "Bei 1:200 ist der Schaft ein einfaches H mit 30-mm-Flanschen und 30-mm-Steg (das Merkmal; 15 und 9 mm real) und scharfen Ecken zwischen 30-mm-Platten (25 und 20 mm real). Die Rippen sind 30-mm-Platten mit wahrem Umriss, jeder Anker ein 44-mm-Klotz, der Blechstummel ein 30-mm-Blech mit geradem Schnitt; Wicklung, Schlinge und Kanthölzer wie bei 1:100."),
        t("Thermal: the H as its three plates (two flanges, web), the rest by reference; only the region totals are meaningful at this scale.", "Thermik: das H als seine drei Bleche (zwei Flansche, Steg), der Rest als Verweis; nur die Regionssummen sind bei diesem Maßstab aussagekräftig."),
        t("Structure: the shaft as the member, plates and bearers as joints, the sling leg as member, three mass points.", "Statik: der Schaft als Stab, Platten und Kanthölzer als Knoten, der Schlingenstrang als Stab, drei Massepunkte."),
    ],
    500: [
        t("At 1:500 the column is 0.4 mm wide on paper. The shaft stays an H with 75 mm flanges and web (two chambers of 50 x 62.5 mm) so that it still reads as a rolled section, the plates are 75 mm blocks of the true 400 x 300 and 240 x 220 mm, the bearers keep their 160 mm. Stiffeners, anchors, gusset stub and choke wrap are below the feature and left out; the sling leg keeps its true 25 mm as a slender rod on the top flange.", "Bei 1:500 ist die Stütze 0,4 mm breit. Der Schaft bleibt ein H mit 75-mm-Flanschen und -Steg (zwei Kammern 50 x 62,5 mm), damit er sich noch als Walzprofil liest, die Platten sind 75-mm-Klötze mit den wahren 400 x 300 und 240 x 220 mm, die Kanthölzer behalten ihre 160 mm. Rippen, Anker, Blechstummel und Schnürung liegen unter dem Merkmal und entfallen; der Schlingenstrang behält seine wahren 25 mm als schlanker Stab auf dem Oberflansch."),
        t("Thermal: the H as its three plates and the plates, bearers and sling by reference.", "Thermik: das H als seine drei Bleche und Platten, Kanthölzer und Schlinge als Verweis."),
        t("Structure: the shaft as the member, plates and bearers as joints, the sling leg as member, three mass points.", "Statik: der Schaft als Stab, Platten und Kanthölzer als Knoten, der Schlingenstrang als Stab, drei Massepunkte."),
    ],
}


def regions_binding(pose):
    base = {
        "column-flanges": {"exposedFraction": 1, "sunlitFraction": 0.5},
        "column-web": {"exposedFraction": 1, "sunlitFraction": 0.3},
        "base-plate": {"exposedFraction": 1, "sunlitFraction": 0.4},
        "head-plate": {"exposedFraction": 1, "sunlitFraction": 0.4},
        "bearers": {"exposedFraction": 0.77, "sunlitFraction": 0.4, "contact": {"fraction": 0.23, "tempC": 14}},
        "sling": {"exposedFraction": 1, "sunlitFraction": 0.6},
    }
    if pose == "reinstalled":
        base["column-flanges"] = {"exposedFraction": 1, "sunlitFraction": 0.6}
        base["base-plate"] = {"exposedFraction": 0.6, "sunlitFraction": 0.1, "contact": {"fraction": 0.4, "tempC": 16}}
        base["head-plate"] = {"exposedFraction": 1, "sunlitFraction": 1}
        base["sling"] = {"exposedFraction": 1, "sunlitFraction": 0.4}
    if pose == "lifted":
        base["column-web"] = {"exposedFraction": 1, "sunlitFraction": 0.35}
        base["sling"] = {"exposedFraction": 1, "sunlitFraction": 0.8}
    return {"regions": base, "emitters": {}}


AXIAL = {"id": "axial-cap", "kind": "imposed", "title": t("Axial load of the new roof beam 200 kN", "Normalkraft des neuen Dachträgers 200 kN"), "anchor": "cap", "fxN": 0, "fyN": -AXIAL_N, "source": "authored-estimate"}
WIND = {"id": "wind-column", "kind": "wind", "title": t("Wind on the column 320 N/m", "Wind auf die Stütze 320 N/m"), "fxN": 0, "fyN": 0, "distributed": {"bone": "column", "fromM": 0, "toM": L, "wxNPerM": WIND_N_M, "wyNPerM": 0}, "source": "literature", "reference": "en-1991-1-4"}

POSES = [
    {"id": "stored", "title": t("Stored on bearers", "Auf Kanthölzern gelagert"), "description": t("The column lies on its bottom flange on two squared-timber bearers in the yard of the component exchange, the round sling choked at the marked centre of gravity and laid along the top flange; it carries only its own weight into the two bearers.", "Die Stütze liegt auf ihrem Unterflansch auf zwei Kanthölzern im Hof der Bauteilbörse, die Rundschlinge im markierten Schwerpunkt geschnürt und auf dem Oberflansch abgelegt; sie trägt nur ihr Eigengewicht in die zwei Kanthölzer ab."), "bones": {}, "ground": "authored", "contacts": ["rest-a", "rest-b"], "thermal": regions_binding("stored"), "loads": []},
    {"id": "lifted", "title": t("Lifted on the sling", "An der Schlinge angehoben"), "description": t("The crane has taken up the choked round sling at the centre of gravity and lifted the column level by 0.5 m; the bearers stay on the ground. The column hangs as a beam with two cantilevers from the one sling point, the sling leg in tension.", "Der Kran hat die geschnürte Rundschlinge im Schwerpunkt aufgenommen und die Stütze waagrecht um 0,5 m angehoben; die Kanthölzer bleiben liegen. Die Stütze hängt als Träger mit zwei Kragarmen am einen Schlingenpunkt, der Schlingenstrang auf Zug."), "bones": {"hoist": {"dy": LIFT}, "sling": {"angleDeg": 90}}, "ground": "authored", "contacts": ["hook"], "thermal": regions_binding("lifted"), "loads": []},
    {"id": "reinstalled", "title": t("Reinstalled as a canopy column", "Als Vordachstütze wieder eingebaut"), "description": t("The column stands again, its base plate fixed to a new footing by new bonded anchors in the old holes, and carries 200 kN from a new roof beam on the head plate and 320 N/m of wind along its height; the sling still hangs from the choke down the flange until the beam is bolted.", "Die Stütze steht wieder, ihre Fußplatte mit neuen Verbundankern in den alten Löchern auf einem neuen Fundament eingespannt, und trägt 200 kN eines neuen Dachträgers auf der Kopfplatte und 320 N/m Wind über die Höhe; die Schlinge hängt noch von der Schnürung am Flansch herab, bis der Träger verschraubt ist."), "bones": {"hoist": {"dy": -AXIS_Y}, "column": {"angleDeg": 90}, "sling": {"angleDeg": -180}}, "ground": "authored", "contacts": ["footing"], "thermal": regions_binding("reinstalled"), "loads": [AXIAL, WIND]},
]

CLIPS = [
    {"id": "lift", "title": t("Lift off the bearers", "Von den Kanthölzern abheben"), "description": t("The sling leg is raised from the top flange to the hook, takes up the weight and the crane lifts the column level off its bearers by 0.5 m; the bearers stay behind.", "Der Schlingenstrang wird vom Oberflansch zum Haken gehoben, nimmt das Gewicht auf, und der Kran hebt die Stütze waagrecht 0,5 m von ihren Kanthölzern ab; die Kanthölzer bleiben zurück."),
     "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2.5, "pose": "stored", "ease": "smooth", "bones": {"sling": {"angleDeg": 90}}}, {"t": 3.5, "pose": "lifted", "ease": "smooth", "bones": {"hoist": {"dy": 0}, "sling": {"angleDeg": 90}}}, {"t": 7, "pose": "lifted", "ease": "smooth"}, {"t": 8, "pose": "lifted", "ease": "smooth"}],
     "contacts": [{"anchor": "rest-a", "fromS": 0, "toS": 3.6}, {"anchor": "rest-b", "fromS": 0, "toS": 3.6}, {"anchor": "hook", "fromS": 2.5, "toS": 8}],
     "phases": [{"id": "rig", "fromS": 0, "toS": 2.5, "title": t("Rig the sling", "Schlinge anschlagen")}, {"id": "take-up", "fromS": 2.5, "toS": 3.5, "title": t("Take up the weight", "Gewicht aufnehmen")}, {"id": "hoist", "fromS": 3.5, "toS": 7, "title": t("Hoist", "Heben")}, {"id": "hold", "fromS": 7, "toS": 8, "title": t("Hold", "Halten")}],
     "events": [{"t": 2.5, "kind": "contact-on", "title": t("Hook takes the sling", "Haken nimmt die Schlinge"), "target": "hook"}, {"t": 3.6, "kind": "contact-off", "title": t("Column leaves the bearers", "Stütze verlässt die Kanthölzer"), "target": "rest-a"}],
     "tracks": []},
    {"id": "service-load", "title": t("Service load and gust", "Gebrauchslast und Böe"), "description": t("Reinstalled, the roof load rises to 200 kN and a gust raises the wind on the column to its full 320 N/m and lets it fall back to half.", "Wieder eingebaut steigt die Dachlast auf 200 kN, und eine Böe steigert den Wind auf die Stütze auf die vollen 320 N/m und lässt ihn auf die Hälfte zurückfallen."),
     "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled", "ease": "smooth"}, {"t": 8, "pose": "reinstalled", "ease": "smooth"}],
     "contacts": [{"anchor": "footing", "fromS": 0, "toS": 8}],
     "phases": [{"id": "roof-load", "fromS": 0, "toS": 4, "title": t("Roof load", "Dachlast")}, {"id": "gust", "fromS": 4, "toS": 8, "title": t("Gust", "Böe")}],
     "events": [{"t": 4, "kind": "load-on", "title": t("Roof load at full value", "Dachlast vollständig"), "target": "axial-cap"}, {"t": 6, "kind": "load-on", "title": t("Gust peak", "Böenspitze"), "target": "wind-column"}],
     "tracks": [{"target": "load:axial-cap", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 4, "v": 1}, {"t": 8, "v": 1}]}, {"target": "load:wind-column", "ease": "smooth", "keys": [{"t": 0, "v": 0.3}, {"t": 4, "v": 0.3}, {"t": 6, "v": 1}, {"t": 8, "v": 0.5}]}]},
]

SHAFT_LEN = SHAFT1 - SHAFT0
A_FLANGES = SHAFT_LEN * 2 * (B + 2 * TF + 2 * (B / 2 - TW / 2 - R) + math.pi / 2 * R * 2 / 2 * 2 / 2) + 2 * (2 * GUSSET_AREA + 0.33 * GUSSET_T)
A_WEB = SHAFT_LEN * 2 * (H - 2 * TF - 2 * R)
THERMAL_REGIONS_AREA = {}


def region(id, en, de, material, area, length, geometry, sky, hc, ref, kind="surface"):
    return {"id": id, "title": t(en, de), "kind": kind, "model": "passive", "material": material, "areaM2": round(area, 3), "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": ref}


AREAS = json.load(open(os.path.join(HERE, "areas.json"))) if os.path.exists(os.path.join(HERE, "areas.json")) else {}
THERMAL = {
    "environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
    "operating": [],
    "regions": [
        region("column-flanges", "Flanges with root fillets and gusset stub", "Flansche mit Ausrundungen und Blechstummel", S, AREAS.get("column-flanges", 3.2), 0.2, "horizontal-plate-up", 0.5, 53000, "en-iso-10456"),
        region("column-web", "Web with its two old holes", "Steg mit seinen zwei alten Löchern", S, AREAS.get("column-web", 0.95), 0.134, "vertical-plate", 0.3, 32000, "en-iso-10456"),
        region("base-plate", "Rusted base plate, stiffeners and cut anchors", "Angerostete Fußplatte, Rippen und abgetrennte Anker", RS, AREAS.get("base-plate", 0.4), 0.4, "vertical-plate", 0.5, 88000, "en-iso-10456"),
        region("head-plate", "Head plate with bolt holes", "Kopfplatte mit Schraubenlöchern", S, AREAS.get("head-plate", 0.12), 0.24, "vertical-plate", 0.5, 71000, "en-iso-10456"),
        region("bearers", "Squared-timber bearers", "Kanthölzer", TW_, AREAS.get("bearers", 1.38), 0.16, "cylinder", 0.5, 26000, "en-iso-13786"),
        region("sling", "Round sling and choke wrap", "Rundschlinge und Schnürung", PS, AREAS.get("sling", 0.2), 0.025, "cylinder", 0.8, 8300, "en-1492-2"),
    ],
    "emitters": [],
    "assumptions": [
        t("All surfaces are passive: the column is a reclaimed member without any device, so its nameplate power is none and no region is heated.", "Alle Flächen sind passiv: die Stütze ist ein gebrauchtes Bauteil ohne Gerät, sie hat keine Nennleistung und keine Region wird beheizt."),
        t("The old red-oxide paint is weathered and chalked: emissivity 0.90 and solar absorptance 0.80 instead of about 0.85 and 0.5 for a fresh light coat; the base plate was grouted and is bare rusted steel (0.85, 0.85).", "Der alte Rostschutzanstrich ist verwittert und kreidend: Emissionsgrad 0,90 und solare Absorption 0,80 statt etwa 0,85 und 0,5 bei einem frischen hellen Anstrich; die Fußplatte war vergossen und ist blanker, angerosteter Stahl (0,85, 0,85)."),
        t("Steel conducts well, so the areal heat capacity uses the full plate thickness (flange 15 mm, web 9 mm, base plate 25 mm, head plate 20 mm) with c = 450 J/(kg K); the timber bearers store over half their periodic penetration depth (35 mm), the sling over 5 mm of fibre.", "Stahl leitet gut, die flächenbezogene Wärmekapazität nutzt daher die volle Blechdicke (Flansch 15 mm, Steg 9 mm, Fußplatte 25 mm, Kopfplatte 20 mm) mit c = 450 J/(kg K); die Kanthölzer speichern über die halbe periodische Eindringtiefe (35 mm), die Schlinge über 5 mm Faser."),
        t("Stored and lifted, the column lies in the open: the top flange takes the sun, the chambers lie half in shade; the bearers lose a quarter of their surface to the ground. Reinstalled, the base plate sits on the footing with 40 % of its area.", "Gelagert und angehoben liegt die Stütze im Freien: der Oberflansch nimmt die Sonne, die Kammern liegen halb im Schatten; die Kanthölzer verlieren ein Viertel ihrer Fläche an den Boden. Wieder eingebaut sitzt die Fußplatte mit 40 % ihrer Fläche auf dem Fundament."),
    ],
    "passengers": [],
}

STRUCTURE = {
    "gravityMS2": G,
    "dynamics": "quasi-static",
    "supports": [],
    "assumptions": [
        t(f"Stored: the column (IPB 200, A = 78.1 cm2, I_y = 5696 cm4, St 37-2) rests with its bottom flange on two bearers at 0.70 and 2.90 m as unilateral contacts with friction 0.5 (steel on dry timber); the {TOTAL_KG:.0f} kg of shaft, plates and anchors give reactions of 1.40 and 1.10 kN, a hogging moment of about 0.33 kN m over bearer A (the heavy base end cantilevers 0.7 m) and only 0.1 kN m of sagging between the bearers.", f"Gelagert: die Stütze (IPB 200, A = 78,1 cm², I_y = 5696 cm⁴, St 37-2) liegt mit dem Unterflansch auf zwei Kanthölzern bei 0,70 und 2,90 m als einseitige Kontakte mit Reibung 0,5 (Stahl auf trockenem Holz); die {TOTAL_KG:.0f} kg aus Schaft, Platten und Ankern ergeben Auflagerkräfte von 1,40 und 1,10 kN, ein Stützmoment von etwa 0,33 kN m über Kantholz A (das schwere Fußende kragt 0,7 m aus) und nur 0,1 kN m Feldmoment zwischen den Kanthölzern."),
        t(f"Lifted: one round sling choked at the centre of gravity ({CHOKE:.3f} m from the base) carries the whole weight; the column is a beam with two cantilevers and the sling leg is in pure tension. The balanced single-point lift is the practice of the yard for short members with a marked centre of gravity.", f"Angehoben: eine im Schwerpunkt ({CHOKE:.3f} m vom Fuß) geschnürte Rundschlinge trägt das ganze Gewicht; die Stütze ist ein Träger mit zwei Kragarmen, der Schlingenstrang steht unter reinem Zug. Der ausgewogene Einpunkthub ist auf dem Hof die Praxis für kurze Bauteile mit markiertem Schwerpunkt."),
        t("Reinstalled: fixed base on a new footing, free head with 200 kN axial load and 320 N/m wind (0.8 kN/m2 x 2.0 x 0.2 m); second-order effects (buckling, the weak axis I_z = 2003 cm4) are not part of the statics and must be checked for the reuse.", "Wieder eingebaut: eingespannter Fuß auf neuem Fundament, freier Kopf mit 200 kN Normalkraft und 320 N/m Wind (0,8 kN/m² x 2,0 x 0,2 m); Effekte zweiter Ordnung (Knicken, schwache Achse I_z = 2003 cm⁴) sind nicht Teil der Statik und müssen für die Wiederverwendung nachgewiesen werden."),
        t(f"The two bearers ({BEARER_KG:.1f} kg each) and the sling (about 0.4 kg) are modelled massless: they stand on the ground on their own and their weight never passes through the column, while the product solves all reactions of an asset as one rigid body and would add the bearer weight to the column supports. The asset mass is therefore that of the column: shaft {COLUMN_KG:.1f} kg (61.3 kg/m), base plate group {BASE_KG:.1f} kg, head plate {HEAD_KG:.1f} kg.", f"Die zwei Kanthölzer (je {BEARER_KG:.1f} kg) und die Schlinge (etwa 0,4 kg) sind masselos modelliert: sie stehen für sich auf dem Boden und ihr Gewicht geht nie durch die Stütze, während das Produkt alle Reaktionen eines Assets als einen starren Körper löst und das Holzgewicht den Stützenauflagern zuschlagen würde. Die Assetmasse ist daher die der Stütze: Schaft {COLUMN_KG:.1f} kg (61,3 kg/m), Fußplattengruppe {BASE_KG:.1f} kg, Kopfplatte {HEAD_KG:.1f} kg."),
    ],
    "clinical": False,
}

SYMBOL = {
    "id": "reclaimed-steel-column-stock",
    "title": t("Reclaimed column in stock, conventional sign", "Gebrauchte Stütze im Lager, Zeichen"),
    "description": t("The yard sign of a stored steel column at real size: in plan the column outline between base and head plate with a diagonal stock cross over its two bearers; in the side and front view the lying bar on its bearers.", "Das Lagerzeichen einer gelagerten Stahlstütze in wahrer Größe: im Grundriss der Stützenumriss zwischen Fuß- und Kopfplatte mit einem diagonalen Lagerkreuz über den zwei Kanthölzern; in Seiten- und Vorderansicht der liegende Balken auf seinen Kanthölzern."),
    "semantics": "true-extent",
    "units": "model-m",
    "denominators": [500, 200, 100],
    "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25},
    "hatches": [],
    "geometry": {
        "elevation": [
            shape("bar", "yard", "body", 1, True, 0.4, P([(0, AXIS_Y - BPV), (0.05, AXIS_Y - BPV), (0.05, AXIS_Y - H / 2), (L - 0.05, AXIS_Y - H / 2), (L - 0.05, AXIS_Y - HPV), (L, AXIS_Y - HPV), (L, AXIS_Y + HPV), (L - 0.05, AXIS_Y + HPV), (L - 0.05, AXIS_Y + H / 2), (0.05, AXIS_Y + H / 2), (0.05, AXIS_Y + BPV), (0, AXIS_Y + BPV)]), regions=["column-flanges", "column-web", "base-plate", "head-plate"], material=S),
            shape("bearer-a", "yard", "body", 0.5, True, 0.7, RECT(BEARERS["bearer-a"] - BEARER / 2, 0, BEARER, BEARER), region="bearers", material=TW_),
            shape("bearer-b", "yard", "body", 0.5, True, 0.7, RECT(BEARERS["bearer-b"] - BEARER / 2, 0, BEARER, BEARER), region="bearers", material=TW_),
        ],
        "front": [
            shape("bearer-a", "yard", "body", 0.5, True, 0.7, RECT(-BEARER_LEN / 2, 0, BEARER_LEN, BEARER), region="bearers", material=TW_),
            shape("bar", "yard", "body", 1, True, 0.4, RECT(-BPW, AXIS_Y - BPV, 2 * BPW, 2 * BPV), regions=["column-flanges", "column-web", "base-plate", "head-plate"], material=S),
        ],
        "plan": [
            shape("bearer-a", "yard", "body", 0.5, False, 0.7, RECT(BEARERS["bearer-a"] - BEARER / 2, -BEARER_LEN / 2, BEARER, BEARER_LEN), region="bearers"),
            shape("bearer-b", "yard", "body", 0.5, False, 0.7, RECT(BEARERS["bearer-b"] - BEARER / 2, -BEARER_LEN / 2, BEARER, BEARER_LEN), region="bearers"),
            shape("bar", "yard", "body", 1, False, 0.4, RECT(0, -B / 2, L, B), regions=["column-flanges", "column-web"]),
            shape("stock-cross-a", "yard", "detail", 1.1, False, 0.2, P([(0, -B / 2), (L, B / 2)], False)),
            shape("stock-cross-b", "yard", "detail", 1.1, False, 0.2, P([(0, B / 2), (L, -B / 2)], False)),
        ],
    },
    "dimensionalNote": t("True extent in model metres: the outline is the real 3.60 x 0.20 m column and the real 1.0 m bearers; the stock cross marks a member offered for reuse.", "Wahre Größe in Modellmetern: der Umriss ist die reale Stütze 3,60 x 0,20 m und die realen 1,0-m-Kanthölzer; das Lagerkreuz kennzeichnet ein zur Wiederverwendung angebotenes Bauteil."),
}


def representation(scale):
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": geometry(scale), "decisions": REP_DECISIONS[scale]}


def volume(scale):
    b = body(scale)
    return {"status": "authored", "denominator": scale, "body": b, "thermal": thermal(scale, b), "structure": structure(scale, b), "decisions": VOL_DECISIONS[scale]}


ASSET = {
    "id": "reclaimed-steel-column",
    "category": "components",
    "title": t("Reclaimed Steel Column with Base Plate", "Gebrauchte Stahlstütze mit Fußplatte"),
    "summary": t(f"A 3.60 m IPB 200 (HEB 200) hall column in St 37-2, dismantled and stored at a component exchange: welded base plate 400 x 300 x 25 mm with two stiffeners and four M24 anchor bolts flame-cut flush under the plate (nuts and washers still on), head plate 240 x 220 x 20 mm with four bolt holes, two old cleat holes in the web, the ragged stub of a cut bracing gusset and weathered red-oxide paint. It lies on its bottom flange on two squared-timber bearers with a round sling choked at the marked centre of gravity, is lifted level on that sling and reinstalled upright as a canopy column under axial load and wind.",
                 f"Eine 3,60 m lange Hallenstütze IPB 200 (HEB 200) aus St 37-2, rückgebaut und in einer Bauteilbörse gelagert: angeschweißte Fußplatte 400 x 300 x 25 mm mit zwei Rippen und vier unter der Platte bündig brenngeschnittenen M24-Ankerschrauben (Muttern und Scheiben noch darauf), Kopfplatte 240 x 220 x 20 mm mit vier Schraubenlöchern, zwei alte Laschenlöcher im Steg, der unregelmäßige Stummel eines abgetrennten Verbandsblechs und verwitterter Rostschutzanstrich. Sie liegt auf ihrem Unterflansch auf zwei Kanthölzern mit einer im markierten Schwerpunkt geschnürten Rundschlinge, wird an dieser Schlinge waagrecht angehoben und als Vordachstütze unter Normalkraft und Wind wieder aufrecht eingebaut."),
    "tags": ["component", "reclaimed", "structural", "steel", "column", "hot-rolled", "base-plate", "stored-on-bearers"],
    "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFS], "notes": t("Reclaimed IPB 200 column of a 1970s steel hall (St 37-2), dismantled by flame-cutting the anchor bolts, stored on bearers for reuse.", "Gebrauchte IPB-200-Stütze einer Stahlhalle der 1970er-Jahre (St 37-2), durch Abbrennen der Ankerschrauben rückgebaut, zur Wiederverwendung auf Kanthölzern gelagert.")},
    "dimensions": {"heightM": round(AXIS_Y + BPV, 3), "widthM": L, "depthM": BEARER_LEN, "massKg": round(round(COLUMN_KG, 2) + round(BASE_KG, 2) + round(HEAD_KG, 2), 2)},
    "materials": MATERIALS,
    "rig": RIG,
    "volumes": {str(s): volume(s) for s in (50, 100, 200, 500)},
    "anchors": ANCHORS,
    "representations": {str(s): representation(s) for s in (50, 100, 200, 500)},
    "symbols": [SYMBOL],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": THERMAL,
    "structure": STRUCTURE,
    "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}

with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
    json.dump(DOC, f, ensure_ascii=False, indent=1)
    f.write("\n")
print(f"[DEBUG] column {COLUMN_KG:.2f} base {BASE_KG:.2f} head {HEAD_KG:.2f} total {TOTAL_KG:.2f} com {COM_U:.4f} choke {CHOKE} base_com {base_com_u:.4f} stiff_area {STIFF_AREA:.5f} gusset_area {GUSSET_AREA:.5f}")
