"""🏛️ Authoritative chisel of the staffage asset `reclaimed-cast-iron-column`: states every hand-chosen number and writes `🔣️.json` next to this file.

Run: `python3 author-column.py` (prints the derived masses, centres and areas used in the report).

@see ../../brief.md — the asset contract of this collection
@see ../../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md — the proof tools
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SLUG = "reclaimed-cast-iron-column"

RHO_IRON = 7100.0
RHO_WOOD = 420.0
RHO_STEEL = 7850.0
RHO_MORTAR = 1800.0
SLING_KG = 0.35

LB = 3.35
U_TOP = 3.40
MORTAR_T = 0.008
T_SL = 0.022
THETA = 55.0
D_SL = 0.45
W_EYE = 0.025
CLEAR = 0.001
LIFT = 0.50
CR_HALF_X = 0.08
CR_HALF_Z = 0.35
CR_H = 0.16
CR_OFF = 1.15
NOTCH_BETA = 60.0
NB_A = 0.125
BED = 0.008


def r_shaft(u):
    """📏️ Outer radius of the tapered shaft between the apophyge and the astragal."""
    return 0.080 - (u - 0.20) * (0.010 / 2.80)


def r_bore(u):
    """🕳️ Radius of the cored bore, a slightly tapered cylinder from the base plate to the head plate."""
    return 0.060 - 0.010 * (u - 0.030) / 3.290


def rnd(x, n=5):
    """🔢️ Rounded coordinate for the document."""
    v = round(x, n)
    return 0.0 if v == 0 else v


def pts(seq):
    """📐️ A rounded point list."""
    return [[rnd(a), rnd(b)] for a, b in seq]


BASE_OUT_50 = [(0.030, 0.118), (0.034, 0.124), (0.042, 0.128), (0.052, 0.129), (0.062, 0.127), (0.070, 0.121), (0.074, 0.113), (0.076, 0.106), (0.080, 0.104), (0.088, 0.098), (0.098, 0.096), (0.108, 0.099), (0.112, 0.103), (0.116, 0.104), (0.120, 0.100), (0.128, 0.097), (0.136, 0.094), (0.142, 0.089), (0.150, 0.086)]
SHAFT_OUT_50 = [(0.150, 0.086), (0.170, 0.082), (0.200, 0.080)] + [(u, r_shaft(u)) for u in (0.80, 1.40, 2.00, 2.60, 3.00)] + [(3.004, 0.078), (3.009, 0.081), (3.015, 0.082), (3.021, 0.081), (3.026, 0.078), (3.030, 0.071), (3.060, 0.071)]
CAP_OUT_50 = [(3.060, 0.071), (3.080, 0.074), (3.120, 0.083), (3.170, 0.097), (3.220, 0.113), (3.260, 0.124), (3.290, 0.130), (3.305, 0.132), (3.305, 0.140), (3.320, 0.140)]

BASE_OUT_100 = [(0.030, 0.118), (0.040, 0.128), (0.055, 0.129), (0.072, 0.116), (0.090, 0.098), (0.110, 0.100), (0.118, 0.103), (0.135, 0.095), (0.150, 0.086)]
SHAFT_OUT_100 = [(0.150, 0.086), (0.200, 0.080), (1.60, r_shaft(1.60)), (3.000, 0.0700), (3.006, 0.080), (3.024, 0.080), (3.030, 0.071), (3.060, 0.071)]
CAP_OUT_100 = [(3.060, 0.071), (3.120, 0.083), (3.200, 0.107), (3.270, 0.126), (3.305, 0.132), (3.305, 0.140), (3.320, 0.140)]

BASE_OUT_200 = [(0.030, 0.124), (0.060, 0.129), (0.085, 0.100), (0.115, 0.104), (0.150, 0.086)]
SHAFT_OUT_200 = [(0.150, 0.086), (0.200, 0.080), (3.000, 0.070), (3.000, 0.081), (3.030, 0.081), (3.030, 0.071), (3.060, 0.071)]
CAP_OUT_200 = [(3.060, 0.071), (3.180, 0.100), (3.290, 0.130), (3.320, 0.140)]

SHAFT_OUT_500 = [(0.080, 0.125), (0.110, 0.110), (0.200, 0.080), (3.060, 0.071)]
CAP_OUT_500 = [(3.060, 0.071), (3.180, 0.100), (3.300, 0.135)]


WALL = 0.022


def loop(outer, hollow, shell=False):
    """♻️ Closed revolve loop: the outer contour and, for a hollow part, the bore back to the start (the shaft) or a 22 mm cast wall following the outline (moulded base and capital); a solid part is closed through the axis."""
    if hollow:
        u0, u1 = outer[0][0], outer[-1][0]
        inner = [(u, max(r_bore(u), r - WALL)) for u, r in reversed(outer)] if shell else [(u1, r_bore(u1)), (u0, r_bore(u0))]
        ring = list(outer) + inner
        return ring + [ring[0]]
    return list(outer)


def half(outer):
    """🌗️ The half profile closed through the axis, the 2D profile of a revolved front or plan solid."""
    return [(outer[0][0], 0.0)] + list(outer) + [(outer[-1][0], 0.0)]


def mirrored(outer):
    """🪞️ The elevation outline of a turned part: the outer contour and its mirror image."""
    return list(outer) + [(u, -r) for u, r in reversed(outer)]


def poly_moments(poly):
    """🧮️ Area and first moments of a closed polygon given as (x, y) points: A, ∫x dA, ∫y dA, ∫xy dA, ∫y·(y) dA terms for Pappus."""
    a = sx = sy = sxy = syy = 0.0
    n = len(poly)
    for i in range(n):
        x0, y0 = poly[i]
        x1, y1 = poly[(i + 1) % n]
        c = x0 * y1 - x1 * y0
        a += c / 2
        sx += (x0 + x1) * c / 6
        sy += (y0 + y1) * c / 6
        sxy += (2 * x0 * y0 + x0 * y1 + x1 * y0 + 2 * x1 * y1) * c / 24
        syy += (y0 * y0 + y0 * y1 + y1 * y1) * c / 12
    s = 1 if a >= 0 else -1
    return a * s, sx * s, sy * s, sxy * s, syy * s


def revolve_mass(ring, rho):
    """⚖️ Mass and axial centre of a full revolve of a closed (u, r) ring (Pappus, exact for polygons)."""
    pts_ = ring[:-1] if ring[0] == ring[-1] else ring
    a, sx, sy, sxy, syy = poly_moments(pts_)
    vol = 2 * math.pi * sy
    return vol * rho, sxy / sy


def area_poly(poly):
    """📐️ Unsigned polygon area."""
    return poly_moments(poly)[0]


def centroid(poly):
    """🎯️ Centroid of a polygon."""
    a, sx, sy, _, _ = poly_moments(poly)
    return sx / a, sy / a


def circle(cx, cy, r, n=16, start=0.0):
    """⭕️ A polygonal circle."""
    return [(cx + r * math.cos(start + 2 * math.pi * k / n), cy + r * math.sin(start + 2 * math.pi * k / n)) for k in range(n)]


def surface_of_revolution(outer):
    """🌀️ Lateral area of the surface swept by an outer (u, r) contour."""
    s = 0.0
    for (u0, r0), (u1, r1) in zip(outer, outer[1:]):
        s += math.pi * (r0 + r1) * math.hypot(u1 - u0, r1 - r0)
    return s


BASE_PLATE = [(0.18, -0.15), (0.15, -0.18), (-0.18, -0.18), (-0.18, 0.18), (0.18, 0.18)]
BASE_PLATE_SQ = [(-0.18, -0.18), (0.18, -0.18), (0.18, 0.18), (-0.18, 0.18)]
BASE_HOLES = [(sv * 0.135, sw * 0.135) for sv in (-1, 1) for sw in (-1, 1)]
HEAD_PLATE = [(-0.17, -0.17), (0.17, -0.17), (0.17, 0.17), (-0.17, 0.17)]
HEAD_HOLES = [(sv * 0.14, sw * 0.08) for sv in (-1, 1) for sw in (-1, 1)]
STUBS = [(0.14, 0.08), (-0.14, -0.08)]
SHIM = [(-0.165, -0.105), (0.165, -0.105), (0.165, 0.105), (-0.165, 0.105)]
MORTAR = [
    [(-0.150, -0.120), (-0.112, -0.150), (-0.040, -0.142), (-0.020, -0.090), (-0.060, -0.040), (-0.128, -0.052)],
    [(0.030, 0.040), (0.085, 0.020), (0.128, 0.055), (0.112, 0.105), (0.060, 0.118), (0.025, 0.090)],
]
LUG_W = (0.115, 0.135)
LUG_W_200 = (0.105, 0.135)

m_bplate = (area_poly(BASE_PLATE) - 4 * math.pi * 0.013 ** 2) * 0.030 * RHO_IRON
m_mortar = sum(area_poly(p) for p in MORTAR) * MORTAR_T * RHO_MORTAR
m_bmould, u_bmould = revolve_mass(loop(BASE_OUT_50, True, True), RHO_IRON)
m_shaft, u_shaft = revolve_mass(loop(SHAFT_OUT_50, True), RHO_IRON)
m_cap, u_cap = revolve_mass(loop(CAP_OUT_50, True, True), RHO_IRON)
m_hplate = (area_poly(HEAD_PLATE) - 4 * math.pi * 0.011 ** 2) * 0.030 * RHO_IRON
m_shim = (area_poly(SHIM) - 4 * math.pi * 0.011 ** 2) * 0.006 * RHO_STEEL
m_lugs = 2 * 0.34 * 0.020 * 0.050 * RHO_IRON
m_stubs = 2 * math.pi * 0.010 ** 2 * 0.052 * RHO_STEEL
m_base = m_bplate + m_mortar + m_bmould
u_base = (m_bplate * 0.015 + m_mortar * -0.004 + m_bmould * u_bmould) / m_base
m_capt = m_cap + m_hplate + m_shim + m_lugs + m_stubs
u_capt = (m_cap * u_cap + m_hplate * 3.335 + m_shim * 3.353 + m_lugs * 3.375 + m_stubs * 3.336) / m_capt
M_COL = m_base + m_shaft + m_capt
U_COM = (m_base * u_base + m_shaft * u_shaft + m_capt * u_capt) / M_COL
M_UNI = m_shaft
mb_model = (M_COL * U_COM - M_UNI * LB / 2 - (M_COL - M_UNI) * u_capt) / (u_base - u_capt)
mc_model = M_COL - M_UNI - mb_model

S_CM = U_COM
S_A = S_CM - CR_OFF
S_B = S_CM + CR_OFF
R_A = r_shaft(S_A)
R_B = r_shaft(S_B)
SIN_B = math.sin(math.radians(NOTCH_BETA))
TAN_B = math.tan(math.radians(NOTCH_BETA))
Y_A = NB_A + R_A / SIN_B
NB_B = Y_A - R_B / SIN_B
DEPTH = {"a": CR_H - NB_A, "b": CR_H - NB_B}
HALFW = {"a": DEPTH["a"] * TAN_B, "b": DEPTH["b"] * TAN_B}
S_CR = {"a": S_A, "b": S_B}

L_SL = D_SL / math.cos(math.radians(THETA))
SIN_T = math.sin(math.radians(THETA))


def cradle_profile(c, chamfer):
    """🪵️ The (w, u) section of a cradle: a 160 x 160 mm squared bearer 700 mm long with its V seat cut to its station of the tapered shaft (u down from the top face)."""
    d, hw = DEPTH[c], HALFW[c]
    if chamfer:
        return [(-0.35, 0.008), (-0.342, 0.0), (-hw, 0.0), (0.0, d), (hw, 0.0), (0.342, 0.0), (0.35, 0.008), (0.35, 0.152), (0.342, 0.16), (-0.342, 0.16), (-0.35, 0.152)]
    return [(-0.35, 0.0), (-hw, 0.0), (0.0, d), (hw, 0.0), (0.35, 0.0), (0.35, 0.16), (-0.35, 0.16)]


m_cr = {c: area_poly(cradle_profile(c, True)) * 0.16 * RHO_WOOD for c in "ab"}
u_cr = {c: centroid(cradle_profile(c, True))[1] for c in "ab"}


def sling_station(c):
    """🪢️ Shaft station of the basket of a sling (lifted pose)."""
    return X_HOOK - D_SL if c == "a" else X_HOOK + D_SL


X_HOOK = (M_COL * S_CM + m_cr["a"] * S_A + m_cr["b"] * S_B) / (M_COL + m_cr["a"] + m_cr["b"])


def sling_r(c):
    """⭕️ Inner radius of the basket of a sling: the shaft at its station plus a millimetre."""
    return r_shaft(sling_station(c)) + CLEAR


def ellipse_arc(rw, ru, n, outward=True):
    """🥚️ Half ellipse under the shaft, from (0, +rw) round u < 0 to (0, -rw), as (u, w)."""
    return [(-ru * math.sin(math.pi * k / n), rw * math.cos(math.pi * k / n)) for k in range(n + 1)]


def eye_arc(r, n):
    """🪝️ Half circle of the eye over the hook, from (L, -r) round u > L to (L, +r), as (u, w)."""
    return [(L_SL + r * math.sin(math.pi * k / n), -r * math.cos(math.pi * k / n)) for k in range(n + 1)]


def sling_rings(c, t, n_bend, n_eye):
    """🔁️ Outer ring and hole of a round sling of section t in its (u, w) plane: legs from the hook to the shaft, basket half ellipse under the shaft and the eye over the hook."""
    ri = sling_r(c)
    ro = ri + t
    outer = eye_arc(W_EYE + t / 2, n_eye) + [(0.0, ro)] + ellipse_arc(ro, ro / SIN_T, n_bend)[1:-1] + [(0.0, -ro)]
    inner_eye = [(L_SL + (W_EYE - t / 2) * math.sin(math.pi * k / n_eye), -(W_EYE - t / 2) * math.cos(math.pi * k / n_eye)) for k in range(n_eye + 1)]
    hole = inner_eye + [(0.0, ri)] + ellipse_arc(ri, ri / SIN_T, n_bend)[1:-1] + [(0.0, -ri)]
    return outer, hole


def sling_band(c, t, n_bend):
    """〰️ The basket and legs of a sling as one simple U-shaped band in (u, w), open at the eye, for front and plan solids."""
    ri = sling_r(c)
    ro = ri + t
    out_leg = [(L_SL, W_EYE + t / 2), (0.0, ro)] + ellipse_arc(ro, ro / SIN_T, n_bend)[1:-1] + [(0.0, -ro), (L_SL, -(W_EYE + t / 2))]
    in_leg = [(L_SL, -(W_EYE - t / 2)), (0.0, -ri)] + list(reversed(ellipse_arc(ri, ri / SIN_T, n_bend)[1:-1])) + [(0.0, ri), (L_SL, W_EYE - t / 2)]
    return out_leg + in_leg


def sling_eye(t, n):
    """🪝️ The eye of a sling as a half annulus in (u, w)."""
    ro, ri = W_EYE + t / 2, W_EYE - t / 2
    return eye_arc(ro, n) + [(L_SL + ri * math.sin(math.pi * k / n), ri * math.cos(math.pi * k / n)) for k in range(n + 1)]


def sling_u_range(c, t):
    """↔️ Extent of a sling along its bone: from under the shaft to the top of the eye."""
    return -(sling_r(c) + t) / SIN_T, L_SL + W_EYE + t / 2


a_out = max(-sling_u_range(c, T_SL)[0] for c in "ab")
u_eye_top = sling_u_range("a", T_SL)[1]
X_OA = S_CM - 0.003 - u_eye_top
X_OB = S_CM + 0.003 + u_eye_top
X_R = X_HOOK
X_RA = S_A - CR_HALF_X - 0.02 - u_eye_top
X_RB = 2 * X_R - X_RA
X_O = {"a": X_OA, "b": X_OB}
TILT = math.degrees(math.atan(-0.010 / 2.80))
REST_ANG = {"a": TILT, "b": 180 + TILT}
Y_REST = {c: Y_A + r_shaft(X_O[c]) + T_SL / 2 for c in "ab"}
X_RS = {"a": X_RA, "b": X_RB}
sl_outer, sl_hole = sling_rings("a", T_SL, 16, 8)
U_SL_COM = centroid(sl_outer)[0]


def tx(t):
    """🌍️ Bilingual text."""
    return {"en": t[0], "de": t[1]}


REFS = [
    ("en-1561", "EN 1561:2011 Founding, grey cast irons: EN-GJL-150 tensile strength 150 to 250 MPa, compressive strength about 600 MPa, modulus of elasticity 78 to 103 GPa, density 7.10 g/cm3, thermal conductivity about 52 W/(m K) at 100 degC, specific heat about 460 J/(kg K) (informative property annex)."),
    ("ahnert-krause", "Ahnert, R.; Krause, K. H.: Typische Baukonstruktionen von 1860 bis 1960 zur Beurteilung der vorhandenen Bausubstanz, Band III (Stützen, Unterzüge, Treppen), Berlin: Verlag Bauwesen / Huss-Medien. Hollow cast-iron columns of the period: outer diameters about 120 to 300 mm, walls 15 to 30 mm, moulded base and capital, square base and head plates with bolt holes and beam lugs."),
    ("kurrer-2008", "Kurrer, K.-E.: The History of the Theory of Structures, Berlin: Ernst & Sohn, 2008: cast iron as a compression material of nineteenth-century iron construction; brittle in tension, therefore columns pin-ended and axially loaded."),
    ("en-338", "EN 338:2016 Structural timber, strength classes, Table 1: C24 mean modulus of elasticity parallel to grain E0,mean = 11.0 GPa, mean density 420 kg/m3."),
    ("en-1492-2", "EN 1492-2:2000+A1:2008 Textile slings, safety, part 2: roundslings made of man-made fibres for general purpose use; colour code violet for a working load limit of 1 t in a straight pull, mode factor of a basket hitch at 45 to 60 degrees 1.4."),
    ("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1, actions on structures, part 1-1: imposed floor loads, Table 6.2 (category B offices 2.0 to 3.0 kN/m2) and self-weights of construction materials, Annex A."),
    ("en-iso-10456", "EN ISO 10456:2007 Building materials and products, hygrothermal properties: design thermal conductivity and specific heat (cast iron 50 W/(m K), 450 J/(kg K); softwood 0.13 W/(m K), 1600 J/(kg K); cement-lime mortar 1800 kg/m3, 0.8 W/(m K), 1000 J/(kg K); polyester 0.17 to 0.4 W/(m K))."),
    ("ashrae-radiation", "ASHRAE Handbook, Fundamentals (2021), chapter on heat transfer, thermal radiation properties of surfaces: typical emissivities of oil paint 0.90 to 0.95, oxidised and rusted iron 0.6 to 0.85, weathered wood 0.90, textiles 0.9 (rounded typical values guided by this chapter, not table look-ups)."),
    ("en-iso-13786", "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); half of it is taken as the effective storage depth of a thick exposed face."),
    ("din-1356-1", "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (scales, line groups)."),
    ("en-10025-2", "EN 10025-2:2019 Hot rolled products of structural steels, non-alloy structural steels: S235, density 7850 kg/m3 (the steel packing plate and the cut bolt stubs)."),
]


def materials():
    """🧱️ The asset materials."""
    return [
        {"id": "cast-iron-painted", "title": tx(("Grey cast iron, old oil paint (grey-green over red-lead primer, weathered)", "Grauguss, alter Ölfarbanstrich (graugrün über Bleimennige, verwittert)")), "emissivity": 0.9, "solarAbsorptivity": 0.72, "densityKgM3": RHO_IRON, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 100e9, "strengthPa": 600e6, "source": "literature", "reference": "en-1561"},
        {"id": "cast-iron-rusted", "title": tx(("Grey cast iron, rust patina with paint remnants", "Grauguss, Rostpatina mit Farbresten")), "emissivity": 0.8, "solarAbsorptivity": 0.78, "densityKgM3": RHO_IRON, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 100e9, "strengthPa": 600e6, "source": "authored-estimate", "reference": "ashrae-radiation"},
        {"id": "steel-rusted", "title": tx(("Mild steel S235, rusted (packing plate, cut bolts)", "Baustahl S235, angerostet (Futterblech, abgetrennte Schrauben)")), "emissivity": 0.8, "solarAbsorptivity": 0.78, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "en-10025-2"},
        {"id": "mortar", "title": tx(("Old cement-lime bedding mortar", "Alter Kalkzement-Mörtel der Lagerfuge")), "emissivity": 0.93, "solarAbsorptivity": 0.6, "densityKgM3": RHO_MORTAR, "conductivityWmK": 0.8, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10456"},
        {"id": "softwood", "title": tx(("Weathered softwood C24 (squared bearers)", "Verwittertes Nadelholz C24 (Kanthölzer)")), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": RHO_WOOD, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11e9, "strengthPa": 21e6, "source": "literature", "reference": "en-338"},
        {"id": "polyester-sling", "title": tx(("Polyester roundsling, violet sleeve (WLL 1 t)", "Polyester-Rundschlinge, violette Hülle (WLL 1 t)")), "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": 900, "conductivityWmK": 0.2, "specificHeatJKgK": 1200, "source": "authored-estimate", "reference": "en-1492-2"},
    ]


SECTION_D, SECTION_DI = 0.150, 0.110
SEC_A = math.pi / 4 * (SECTION_D ** 2 - SECTION_DI ** 2)
SEC_I = math.pi / 64 * (SECTION_D ** 4 - SECTION_DI ** 4)


def lim(values, pad):
    """📏️ Joint limits that hold every authored offset."""
    lo, hi = min(values + [0.0]), max(values + [0.0])
    return lo - pad, hi + pad


def pose_offsets():
    """🎚️ The bone offsets of the named and key poses."""
    sl = {}
    for c in "ab":
        sl[f"sling-{c}-shift"] = {"dx": rnd(sling_station(c) - X_O[c])}
        sl[f"sling-{c}-drop"] = {"dy": rnd(Y_A - Y_REST[c])}
        sl[f"sling-{c}"] = {"angleDeg": rnd((THETA if c == "a" else 180 - THETA) - REST_ANG[c], 4)}
    lifted = dict(sl)
    for c in "ab":
        lifted[f"cradle-{c}-drop"] = {"dy": -LIFT}
    rein = {}
    for c in "ab":
        rein[f"cradle-{c}-shift"] = {"dx": rnd(CR_H - BED - S_CR[c])}
        rein[f"cradle-{c}-drop"] = {"dy": rnd(X_R - S_CR[c] - CR_H + Y_A)}
        rein[f"cradle-{c}"] = {"angleDeg": -90}
        rein[f"sling-{c}-shift"] = {"dx": rnd(T_SL / 2 - BED - X_O[c])}
        rein[f"sling-{c}-drop"] = {"dy": rnd(X_R - X_RS[c] - (Y_REST[c] - Y_A))}
        rein[f"sling-{c}"] = {"angleDeg": rnd(-90 - TILT, 4)}
    ecc = dict(rein)
    ecc["load-carrier"] = {"dy": 0.05}
    return sl, lifted, rein, ecc


SLUNG, LIFTED, REIN, ECC = pose_offsets()


def joint_range(bone, key):
    """🦾️ The travel or turn of a bone over all poses and clip keys."""
    vals = [p[bone][key] for p in (SLUNG, LIFTED, REIN, ECC) if bone in p and key in p[bone]]
    if bone == "load-carrier":
        vals += [-0.05, 0.05]
    return vals


def rig():
    """🦴️ The rig: the column as root (placed by the pose root), its end masses, the load carrier, and per cradle and sling a two-axis slide and a revolute body so that the storage aids stay where they lie whatever the column does."""
    bones = [
        {"id": "column", "parent": None, "title": tx(("Column (root, placed by the pose)", "Stütze (Wurzel, durch die Pose platziert)")), "x": 0, "y": rnd(Y_A), "angleDeg": 0, "lengthM": LB, "joint": {"type": "fixed"},
         "mass": {"kg": rnd(M_UNI, 3), "comX": rnd(LB / 2), "comY": 0, "distribution": "uniform", "source": "literature", "reference": "en-1561"},
         "section": {"areaM2": rnd(SEC_A, 7), "secondMomentM4": float(f"{SEC_I:.5g}"), "extremeFibreM": SECTION_D / 2, "material": "cast-iron-painted", "title": tx(("Hollow cast-iron shaft at mid-height, 150 / 110 mm", "Hohler Gussschaft in halber Höhe, 150 / 110 mm"))}},
        {"id": "column-base", "parent": "column", "title": tx(("Base plate and moulded base (mass)", "Fußplatte und profilierte Basis (Masse)")), "x": rnd(u_base), "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"},
         "mass": {"kg": rnd(mb_model, 3), "comX": 0, "comY": 0, "distribution": "point", "source": "literature", "reference": "en-1561"}},
        {"id": "column-capital", "parent": "column", "title": tx(("Capital, head plate, lugs and remnants (mass)", "Kapitell, Kopfplatte, Knaggen und Reste (Masse)")), "x": rnd(u_capt), "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"},
         "mass": {"kg": rnd(mc_model, 3), "comX": 0, "comY": 0, "distribution": "point", "source": "literature", "reference": "en-1561"}},
        {"id": "load-carrier", "parent": "column", "title": tx(("Load carrier on the head plate (massless, moves the beam reaction)", "Lastträger auf der Kopfplatte (masselos, verschiebt die Trägerauflagerkraft)")), "x": LB, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.06, "maxM": 0.06}},
    ]
    for c in "ab":
        lo, hi = lim(joint_range(f"cradle-{c}-shift", "dx"), 0.05)
        lo2, hi2 = lim(joint_range(f"cradle-{c}-drop", "dy"), 0.05)
        bones += [
            {"id": f"cradle-{c}-shift", "parent": "column", "title": tx((f"Cradle {c.upper()} slide along the column (massless)", f"Lagerbock {c.upper()} Schieber längs der Stütze (masselos)")), "x": rnd(S_CR[c]), "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": rnd(lo, 3), "maxM": rnd(hi, 3)}},
            {"id": f"cradle-{c}-drop", "parent": f"cradle-{c}-shift", "title": tx((f"Cradle {c.upper()} slide across the column (massless)", f"Lagerbock {c.upper()} Schieber quer zur Stütze (masselos)")), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": rnd(lo2, 3), "maxM": rnd(hi2, 3)}},
            {"id": f"cradle-{c}", "parent": f"cradle-{c}-drop", "title": tx((f"Timber cradle {c.upper()} (from its seat down to the ground)", f"Holzlagerbock {c.upper()} (vom Auflager bis zum Boden)")), "x": 0, "y": rnd(CR_H - Y_A), "angleDeg": -90, "lengthM": CR_H, "joint": {"type": "revolute", "minDeg": -95, "maxDeg": 5},
             "mass": {"kg": rnd(m_cr[c], 3), "comX": rnd(u_cr[c]), "comY": 0, "distribution": "point", "source": "literature", "reference": "en-338"},
             "section": {"areaM2": rnd(0.16 * 0.70, 5), "secondMomentM4": float(f"{0.70 * 0.16 ** 3 / 12:.5g}"), "extremeFibreM": 0.08, "material": "softwood", "title": tx(("Squared bearer 160 x 700 mm", "Kantholz 160 x 700 mm"))}},
        ]
    for c in "ab":
        lo, hi = lim(joint_range(f"sling-{c}-shift", "dx"), 0.05)
        lo2, hi2 = lim(joint_range(f"sling-{c}-drop", "dy"), 0.05)
        bones += [
            {"id": f"sling-{c}-shift", "parent": "column", "title": tx((f"Sling {c.upper()} slide along the column (massless)", f"Schlinge {c.upper()} Schieber längs der Stütze (masselos)")), "x": rnd(X_O[c]), "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": rnd(lo, 3), "maxM": rnd(hi, 3)}},
            {"id": f"sling-{c}-drop", "parent": f"sling-{c}-shift", "title": tx((f"Sling {c.upper()} slide across the column (massless)", f"Schlinge {c.upper()} Schieber quer zur Stütze (masselos)")), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": rnd(lo2, 3), "maxM": rnd(hi2, 3)}},
            {"id": f"sling-{c}", "parent": f"sling-{c}-drop", "title": tx((f"Roundsling {c.upper()} (from the basket under the shaft to the hook)", f"Rundschlinge {c.upper()} (vom Korb unter dem Schaft bis zum Haken)")), "x": 0, "y": rnd(Y_REST[c] - Y_A), "angleDeg": rnd(REST_ANG[c], 4), "lengthM": rnd(L_SL), "joint": {"type": "revolute", "minDeg": -95, "maxDeg": 60},
             "mass": {"kg": SLING_KG, "comX": rnd(U_SL_COM), "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "en-1492-2"}},
        ]
    return {"kind": "structural", "root": "column", "bones": bones}


def anchors():
    """📍️ Contacts of the cradles, the hook rollers of the slings, the footing and head restraints and the load point."""
    out = []
    for c in "ab":
        for side, z in (("l", 0.25), ("r", -0.25)):
            out.append({"id": f"cradle-{c}-foot-{side}", "bone": f"cradle-{c}", "x": CR_H, "y": 0, "z": z, "kind": "contact", "title": tx((f"Cradle {c.upper()} on the yard, {'left' if side == 'l' else 'right'} half", f"Lagerbock {c.upper()} auf dem Hof, {'linke' if side == 'l' else 'rechte'} Hälfte")), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.16, "footprintWidthM": 0.2, "frictionCoefficient": 0.5})
    out.append({"id": "hook-a", "bone": "sling-a", "x": rnd(L_SL), "y": 0, "kind": "support", "title": tx(("Crane hook, sling A (pull along the sling)", "Kranhaken, Schlinge A (Zug längs der Schlinge)")), "support": "roller", "normalDeg": THETA})
    out.append({"id": "hook-b", "bone": "sling-b", "x": rnd(L_SL), "y": 0, "kind": "support", "title": tx(("Crane hook, sling B (pull along the sling)", "Kranhaken, Schlinge B (Zug längs der Schlinge)")), "support": "roller", "normalDeg": 180 - THETA})
    out.append({"id": "footing", "bone": "column", "x": 0, "y": 0, "kind": "support", "title": tx(("Footing with dowel, pinned base", "Fundament mit Dorn, gelenkiger Fuß")), "support": "pin"})
    out.append({"id": "head-x", "bone": "column", "x": LB, "y": 0, "kind": "support", "title": tx(("Head held by the floor beam, forward", "Kopf vom Deckenträger gehalten, vorwärts")), "support": "roller", "normalDeg": 0})
    out.append({"id": "head-z", "bone": "column", "x": LB, "y": 0, "kind": "support", "title": tx(("Head held by the floor beam, lateral", "Kopf vom Deckenträger gehalten, seitlich")), "support": "roller", "normalDeg": 0, "tiltDeg": 90})
    out.append({"id": "load-point", "bone": "load-carrier", "x": 0, "y": 0, "kind": "load", "title": tx(("Beam reaction on the head plate", "Trägerauflagerkraft auf der Kopfplatte")) })
    return out


REGIONS = ["shaft", "base", "capital", "cradles", "slings"]


def binding(state):
    """🌬️ Region bindings of a pose: stored and lifted outdoors in the yard, reinstalled indoors."""
    if state == "rein":
        r = {"shaft": {"exposedFraction": 1, "sunlitFraction": 0.0}, "base": {"exposedFraction": 0.8, "sunlitFraction": 0.0, "contact": {"fraction": 0.2, "tempC": 16}}, "capital": {"exposedFraction": 0.85, "sunlitFraction": 0.0, "contact": {"fraction": 0.15, "tempC": 18}},
             "cradles": {"exposedFraction": 0.85, "sunlitFraction": 0.4, "contact": {"fraction": 0.15, "tempC": 14}}, "slings": {"exposedFraction": 0.6, "sunlitFraction": 0.3, "contact": {"fraction": 0.4, "tempC": 14}}}
    elif state == "lifted":
        r = {"shaft": {"exposedFraction": 1, "sunlitFraction": 0.5}, "base": {"exposedFraction": 1, "sunlitFraction": 0.3}, "capital": {"exposedFraction": 1, "sunlitFraction": 0.3},
             "cradles": {"exposedFraction": 0.85, "sunlitFraction": 0.4, "contact": {"fraction": 0.15, "tempC": 14}}, "slings": {"exposedFraction": 0.95, "sunlitFraction": 0.4}}
    else:
        r = {"shaft": {"exposedFraction": 0.98, "sunlitFraction": 0.5, "contact": {"fraction": 0.02, "tempC": 18}}, "base": {"exposedFraction": 1, "sunlitFraction": 0.3}, "capital": {"exposedFraction": 1, "sunlitFraction": 0.3},
             "cradles": {"exposedFraction": 0.83, "sunlitFraction": 0.4, "contact": {"fraction": 0.17, "tempC": 14}}, "slings": {"exposedFraction": 0.9, "sunlitFraction": 0.5, "contact": {"fraction": 0.1, "tempC": 18}}}
    return {"regions": r, "emitters": {}}


FEET = [f"cradle-{c}-foot-{s}" for c in "ab" for s in "lr"]
P_AXIAL = 250000.0


def axial_load():
    """🧲️ The service load of the reinstalled column: the reactions of the floor beams of two storeys above."""
    return {"id": "floor-reactions", "kind": "carried", "title": tx(("Floor beam reactions 250 kN", "Auflagerkräfte der Deckenträger 250 kN")), "anchor": "load-point", "fxN": 0, "fyN": -P_AXIAL, "source": "authored-estimate", "reference": "en-1991-1-1"}


def poses():
    """🧍️ Stored, slung (clip key), lifted, reinstalled and reinstalled with an eccentric beam reaction."""
    rein_root = {"x": rnd(X_R + Y_A), "y": BED, "angleDeg": 90}
    return [
        {"id": "stored", "title": tx(("Stored in two cradles", "In zwei Lagerböcken gelagert")), "description": tx(("The column lies in the V seats of two notched squared bearers 2.3 m apart in the yard of the component exchange; the two roundslings for the next lift lie flat on top of the shaft.", "Die Stütze liegt in den V-Kerben zweier gekerbter Kanthölzer im Abstand von 2,3 m auf dem Hof der Bauteilbörse; die zwei Rundschlingen für das nächste Anheben liegen flach oben auf dem Schaft.")), "bones": {}, "ground": "authored", "contacts": FEET, "thermal": binding("stored"), "loads": []},
        {"id": "slung", "key": True, "title": tx(("Slung, still in the cradles", "Angeschlagen, noch in den Lagerböcken")), "description": tx(("Both roundslings in a basket hitch under the shaft, 0.9 m apart about the centre of mass, their eyes on the crane hook; the column still rests in the cradles.", "Beide Rundschlingen im Hängegang unter dem Schaft, 0,9 m auseinander um den Schwerpunkt, ihre Augen im Kranhaken; die Stütze ruht noch in den Lagerböcken.")), "bones": SLUNG, "ground": "authored", "contacts": FEET, "thermal": binding("stored"), "loads": []},
        {"id": "lifted", "title": tx(("Lifted on two roundslings", "An zwei Rundschlingen angehoben")), "description": tx((f"The column hangs level {LIFT:.1f} m above its cradles from two roundslings at {THETA:.0f} degrees, each pulling along its leg; the cradles and their timber stay on the ground.", f"Die Stütze hängt waagerecht {LIFT:.1f} m über ihren Lagerböcken an zwei Rundschlingen unter {THETA:.0f} Grad, jede zieht längs ihres Strangs; die Lagerböcke bleiben am Boden.")), "bones": LIFTED, "root": {"x": 0, "y": LIFT}, "ground": "authored", "contacts": ["hook-a", "hook-b"], "thermal": binding("lifted"), "loads": []},
        {"id": "reinstalled", "title": tx(("Reinstalled under axial load", "Wieder eingebaut unter Normalkraft")), "description": tx(("Upright again on a new 8 mm mortar bed and a doweled footing, the head held by the floor beam; the beam reactions of two storeys, 250 kN, act centrally on the head plate. The cradles and slings lie in the yard beside it.", "Wieder aufrecht auf einem neuen 8-mm-Mörtelbett und einem Fundament mit Dorn, der Kopf vom Deckenträger gehalten; die Trägerauflagerkräfte zweier Geschosse, 250 kN, wirken mittig auf die Kopfplatte. Lagerböcke und Schlingen liegen daneben auf dem Hof.")), "bones": REIN, "root": rein_root, "ground": "authored", "contacts": ["footing", "head-x", "head-z"], "thermal": binding("rein"), "loads": [axial_load()]},
        {"id": "reinstalled-eccentric", "title": tx(("Reinstalled, eccentric beam reaction", "Wieder eingebaut, außermittige Trägerauflagerkraft")), "description": tx(("The same 250 kN reach the head plate 50 mm off the axis because the floor bays on the two sides are loaded unequally; the pinned column bends linearly from the head down.", "Dieselben 250 kN erreichen die Kopfplatte 50 mm außerhalb der Achse, weil die Deckenfelder beidseits ungleich belastet sind; die gelenkige Stütze biegt sich linear vom Kopf nach unten.")), "bones": ECC, "root": rein_root, "ground": "authored", "contacts": ["footing", "head-x", "head-z"], "thermal": binding("rein"), "loads": [axial_load()]},
    ]


def clips():
    """🎞️ The lift (stored, slung by a cut, hoisted) and two clips of the reinstalled life."""
    return [
        {"id": "lift", "title": tx(("Lift out of the cradles", "Aus den Lagerböcken heben")), "description": tx(("The slings are put on (a cut: a rigid sling cannot be threaded under the shaft), take up the load and hoist the column 0.5 m clear of the cradles, which stay on the ground.", "Die Schlingen werden angelegt (ein Schnitt: eine starre Schlinge lässt sich nicht unter dem Schaft durchziehen), nehmen die Last auf und heben die Stütze 0,5 m über die Lagerböcke, die am Boden bleiben.")),
         "durationS": 4, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "step"}, {"t": 1.0, "pose": "slung", "ease": "linear"}, {"t": 1.4, "pose": "slung", "ease": "smooth"}, {"t": 4.0, "pose": "lifted", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 1.45} for a in FEET] + [{"anchor": "hook-a", "fromS": 1.0, "toS": 4.0}, {"anchor": "hook-b", "fromS": 1.0, "toS": 4.0}],
         "phases": [{"id": "slinging", "fromS": 0, "toS": 1.0, "title": tx(("Putting on the slings", "Anschlagen"))}, {"id": "take-up", "fromS": 1.0, "toS": 1.4, "title": tx(("Taking up the load", "Last aufnehmen"))}, {"id": "hoist", "fromS": 1.4, "toS": 4.0, "title": tx(("Hoisting", "Heben"))}],
         "events": [{"t": 1.0, "kind": "motion", "title": tx(("Slings on the hook (cut)", "Schlingen im Haken (Schnitt)"))}, {"t": 1.45, "kind": "contact-off", "title": tx(("Lift-off from the cradles", "Abheben von den Lagerböcken"))}], "tracks": []},
        {"id": "loading", "title": tx(("Floors loaded", "Decken belastet")), "description": tx(("The beam reactions rise from zero to 250 kN as the floors above are built up and occupied.", "Die Trägerauflagerkräfte steigen von null auf 250 kN, während die Decken darüber aufgebaut und genutzt werden.")),
         "durationS": 5, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled", "ease": "smooth"}, {"t": 5, "pose": "reinstalled", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 5} for a in ("footing", "head-x", "head-z")],
         "phases": [{"id": "ramp", "fromS": 0, "toS": 4, "title": tx(("Ramp", "Aufbau"))}, {"id": "hold", "fromS": 4, "toS": 5, "title": tx(("Hold", "Halten"))}],
         "events": [{"t": 4, "kind": "load-on", "title": tx(("Full service load", "Volle Gebrauchslast"))}], "tracks": [{"target": "load:floor-reactions", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 4, "v": 1}, {"t": 5, "v": 1}]}]},
        {"id": "eccentricity-sweep", "title": tx(("Eccentricity sweep", "Exzentrizität wandert")), "description": tx(("Under the full 250 kN the beam reaction moves across the head plate from 50 mm on one side to 50 mm on the other, as the bays on either side are loaded in turn.", "Unter vollen 250 kN wandert die Trägerauflagerkraft über die Kopfplatte von 50 mm auf der einen zu 50 mm auf der anderen Seite, während die Felder beidseits nacheinander belastet werden.")),
         "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled-eccentric", "ease": "smooth", "bones": {"load-carrier": {"dy": -0.05}}}, {"t": 6, "pose": "reinstalled-eccentric", "ease": "smooth", "bones": {"load-carrier": {"dy": 0.05}}}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 6} for a in ("footing", "head-x", "head-z")],
         "phases": [{"id": "sweep", "fromS": 0, "toS": 6, "title": tx(("Sweep", "Wanderung"))}],
         "events": [{"t": 3, "kind": "motion", "title": tx(("Reaction on the axis", "Auflagerkraft in der Achse"))}], "tracks": []},
    ]


def shape(sid, bone, role, z, filled, tone, geom, region=None, regions=None, material=None, pen=None, solid=None):
    """✏️ One authored 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if pen is not None:
        s["penMm"] = pen
    s.update(geom)
    if solid:
        s["solid"] = solid
    return s


def polyg(points, closed=True):
    """🔷️ Polygon geometry."""
    return {"kind": "poly", "points": pts(points), "closed": closed}


def rectg(x, y, w, h):
    """⬛️ Rectangle geometry."""
    return {"kind": "rect", "x": rnd(x), "y": rnd(y), "w": rnd(w), "h": rnd(h)}


REV = {"kind": "revolved", "axisBone": "column", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}


def ext_u(u0, u1):
    """🌀️ A plate swept along the column axis."""
    return {"kind": "extruded", "sweep": "u", "z0": rnd(u0), "z1": rnd(u1)}


def ext_v(v0, v1):
    """🌀️ A sling band swept across its own plane."""
    return {"kind": "extruded", "sweep": "v", "z0": rnd(v0), "z1": rnd(v1)}


SCALE = {
    50: {"feature": 0.0075, "base": BASE_OUT_50, "shaft": SHAFT_OUT_50, "cap": CAP_OUT_50, "hollow": True, "t_sl": T_SL, "bend": 16, "eye": 8},
    100: {"feature": 0.015, "base": BASE_OUT_100, "shaft": SHAFT_OUT_100, "cap": CAP_OUT_100, "hollow": True, "t_sl": T_SL, "bend": 10, "eye": 6},
    200: {"feature": 0.03, "base": BASE_OUT_200, "shaft": SHAFT_OUT_200, "cap": CAP_OUT_200, "hollow": False, "t_sl": 0.030, "bend": 6, "eye": 4},
    500: {"feature": 0.075, "base": None, "shaft": SHAFT_OUT_500, "cap": CAP_OUT_500, "hollow": False, "t_sl": T_SL, "bend": 0, "eye": 0},
}


def column_body(sc):
    """🏛️ The body solids of the column at one scale, with their regions and materials."""
    p = SCALE[sc]
    out = []

    def rev(sid, outer, region, material, tone, hollow):
        out.append(({"id": sid, "kind": "revolve", "bone": "column", "material": material, "role": "body", "tone": tone, "profile": pts(loop(outer, hollow, sid != "shaft")), "axis": [1, 0, 0], "origin": [0, 0, 0]}, region))

    def plate(sid, prof, holes, u0, u1, region, material, tone, role="body", extra=None):
        s = {"id": sid, "kind": "extrude", "bone": "column", "material": material, "role": role, "tone": tone, "profile": pts(prof), "plane": "yz", "from": rnd(u0), "to": rnd(u1)}
        if holes:
            s["holes"] = [pts(h) for h in holes]
        if extra:
            s.update(extra)
        out.append((s, region))

    if sc == 50:
        plate("base-plate", BASE_PLATE, [circle(v, w, 0.013, 12) for v, w in BASE_HOLES], 0, 0.03, "base", "cast-iron-rusted", 0.32)
        for i, m in enumerate(MORTAR):
            plate(f"mortar-{i + 1}", m, None, -MORTAR_T, 0, "base", "mortar", 0.78, role="detail")
        rev("base-moulding", p["base"], "base", "cast-iron-rusted", 0.36, True)
        rev("shaft", p["shaft"], "shaft", "cast-iron-painted", 0.45, True)
        rev("capital", p["cap"], "capital", "cast-iron-painted", 0.42, True)
        plate("head-plate", HEAD_PLATE, [circle(v, w, 0.011, 12) for v, w in HEAD_HOLES], 3.32, 3.35, "capital", "cast-iron-rusted", 0.32)
        plate("packing-plate", SHIM, [circle(v, w, 0.011, 12) for v, w in HEAD_HOLES], 3.35, 3.356, "capital", "steel-rusted", 0.25, role="detail", extra={"sheet": True})
        for side, sgn in (("l", 1), ("r", -1)):
            w0, w1 = sorted((sgn * LUG_W[0], sgn * LUG_W[1]))
            plate(f"lug-{side}", [(-0.17, w0), (0.17, w0), (0.17, w1), (-0.17, w1)], None, 3.35, U_TOP, "capital", "cast-iron-rusted", 0.34)
        for i, (v, w) in enumerate(STUBS):
            out.append(({"id": f"bolt-stub-{i + 1}", "kind": "revolve", "bone": "column", "material": "steel-rusted", "role": "detail", "tone": 0.2, "profile": [[3.31, 0], [3.31, 0.01], [3.362, 0.01], [3.362, 0]], "axis": [1, 0, 0], "origin": [0, v, w]}, "capital"))
    elif sc == 100:
        plate("base-plate", BASE_PLATE, None, 0, 0.03, "base", "cast-iron-rusted", 0.32)
        rev("base-moulding", p["base"], "base", "cast-iron-rusted", 0.36, True)
        rev("shaft", p["shaft"], "shaft", "cast-iron-painted", 0.45, True)
        rev("capital", p["cap"], "capital", "cast-iron-painted", 0.42, True)
        plate("head-plate", HEAD_PLATE, None, 3.32, 3.356, "capital", "cast-iron-rusted", 0.32)
        for side, sgn in (("l", 1), ("r", -1)):
            w0, w1 = sorted((sgn * LUG_W[0], sgn * LUG_W[1]))
            plate(f"lug-{side}", [(-0.17, w0), (0.17, w0), (0.17, w1), (-0.17, w1)], None, 3.356, U_TOP, "capital", "cast-iron-rusted", 0.34)
    elif sc == 200:
        plate("base-plate", BASE_PLATE_SQ, None, 0, 0.032, "base", "cast-iron-rusted", 0.32)
        rev("base-moulding", [(0.032, 0.125)] + p["base"][1:], "base", "cast-iron-rusted", 0.36, False)
        rev("shaft", p["shaft"], "shaft", "cast-iron-painted", 0.45, False)
        rev("capital", p["cap"], "capital", "cast-iron-painted", 0.42, False)
        w0, w1 = LUG_W_200
        out.append(({"id": "head-plate", "kind": "extrude", "bone": "column", "material": "cast-iron-rusted", "role": "body", "tone": 0.32, "profile": pts([(-0.17, 3.32), (0.17, 3.32), (0.17, 3.35), (w1, 3.35), (w1, U_TOP), (w0, U_TOP), (w0, 3.35), (-w0, 3.35), (-w0, U_TOP), (-w1, U_TOP), (-w1, 3.35), (-0.17, 3.35)]), "plane": "zx", "from": -0.17, "to": 0.17}, "capital"))
    else:
        plate("base-plate", BASE_PLATE_SQ, None, 0, 0.08, "base", "cast-iron-rusted", 0.32)
        rev("shaft", p["shaft"], "shaft", "cast-iron-painted", 0.45, False)
        rev("capital", p["cap"], "capital", "cast-iron-painted", 0.42, False)
        plate("head-plate", HEAD_PLATE, None, 3.30, U_TOP, "capital", "cast-iron-rusted", 0.32)
    return out


def cradle_body(sc, c):
    """🪵️ The body solid of a cradle."""
    return {"id": f"cradle-{c}", "kind": "extrude", "bone": f"cradle-{c}", "material": "softwood", "role": "body", "tone": 0.72, "profile": pts(cradle_profile(c, sc == 50)), "plane": "zx", "from": -CR_HALF_X, "to": CR_HALF_X}


def sling_v(sc, c):
    """↕️ The span of a sling band across its plane: centred on the bone, at 1:200 the generalised 30 mm band keeps its lower face where the true band has it, on the side of the bone that faces down at rest."""
    t = SCALE[sc]["t_sl"]
    if sc != 200:
        return -t / 2, t / 2
    return (-T_SL / 2, t - T_SL / 2) if c == "a" else (-(t - T_SL / 2), T_SL / 2)


def sling_body(sc, c):
    """🪢️ The body solids of a sling: one closed loop band at 1:50 to 1:200, two slender legs at 1:500."""
    p = SCALE[sc]
    t = p["t_sl"]
    if sc == 500:
        length = L_SL + W_EYE + t / 2
        return [{"id": f"sling-{c}", "kind": "box", "bone": f"sling-{c}", "material": "polyester-sling", "role": "body", "tone": 0.68, "slender": True, "size": [rnd(length), t, t], "transform": {"translation": [rnd(length / 2), 0, 0]}}]
    outer, hole = sling_rings(c, t, p["bend"], p["eye"])
    v0, v1 = sling_v(sc, c)
    return [{"id": f"sling-{c}", "kind": "extrude", "bone": f"sling-{c}", "material": "polyester-sling", "role": "body", "tone": 0.68, "profile": pts([(w, u) for u, w in outer]), "holes": [pts([(w, u) for u, w in hole])], "plane": "zx", "from": rnd(v0), "to": rnd(v1)}]


def elevation(sc):
    """📐️ The side elevation of one scale, drawn in the bone frames to the numbers of the solids."""
    p = SCALE[sc]
    out = []
    pen_fine = None
    if sc == 50:
        out.append(shape("base-plate", "column", "body", 2.0, True, 0.32, polyg([(0, -0.18), (0.03, -0.18), (0.03, 0.18), (0, 0.18)]), region="base", material="cast-iron-rusted"))
        for i, m in enumerate(MORTAR):
            vs = [v for v, w in m]
            out.append(shape(f"mortar-{i + 1}", "column", "detail", 2.1 + i * 0.01, True, 0.78, polyg([(-MORTAR_T, min(vs)), (0, min(vs)), (0, max(vs)), (-MORTAR_T, max(vs))]), region="base", material="mortar"))
    else:
        u1 = {100: 0.03, 200: 0.032, 500: 0.08}[sc]
        out.append(shape("base-plate", "column", "body", 2.0, True, 0.32, polyg([(0, -0.18), (u1, -0.18), (u1, 0.18), (0, 0.18)]), region="base", material="cast-iron-rusted", pen=pen_fine))
    if p["base"]:
        b = p["base"] if sc != 200 else [(0.032, 0.125)] + p["base"][1:]
        out.append(shape("base-moulding", "column", "body", 1.6, True, 0.36, polyg(mirrored(b)), region="base", material="cast-iron-rusted"))
    out.append(shape("shaft", "column", "body", 1.0, True, 0.45, polyg(mirrored(p["shaft"])), region="shaft", material="cast-iron-painted"))
    out.append(shape("capital", "column", "body", 1.5, True, 0.42, polyg(mirrored(p["cap"])), region="capital", material="cast-iron-painted"))
    hp = {50: (3.32, 3.35), 100: (3.32, 3.356), 200: (3.32, 3.35), 500: (3.30, U_TOP)}[sc]
    out.append(shape("head-plate", "column", "body", 2.0, True, 0.32, polyg([(hp[0], -0.17), (hp[1], -0.17), (hp[1], 0.17), (hp[0], 0.17)]), region="capital", material="cast-iron-rusted", pen=pen_fine))
    if sc in (50, 100, 200):
        lu0 = {50: 3.35, 100: 3.356, 200: 3.35}[sc]
        out.append(shape("lugs", "column", "body", 2.2, True, 0.34, polyg([(lu0, -0.17), (U_TOP, -0.17), (U_TOP, 0.17), (lu0, 0.17)]), region="capital", material="cast-iron-rusted", pen=pen_fine))
    if sc == 50:
        for i, u in enumerate((0.074, 0.116)):
            r = dict(BASE_OUT_50)[u]
            out.append(shape(f"moulding-line-{i + 1}", "column", "detail", 1.65, False, 0.25, polyg([(u, -r), (u, r)], False), region="base"))
        for i, u in enumerate((3.004, 3.026)):
            r = dict(SHAFT_OUT_50)[u]
            out.append(shape(f"astragal-line-{i + 1}", "column", "detail", 1.1, False, 0.3, polyg([(u, -r), (u, r)], False), region="shaft"))
        out.append(shape("abacus-line", "column", "detail", 1.55, False, 0.3, polyg([(3.305, -0.132), (3.305, 0.132)], False), region="capital"))
        for i, (u0, v0, du, dv) in enumerate(((0.62, 0.03, 0.16, 0.035), (1.92, -0.055, 0.22, 0.03), (2.71, 0.012, 0.12, 0.04))):
            out.append(shape(f"paint-loss-{i + 1}", "column", "texture", 1.2, False, 0.3, polyg([(u0, v0), (u0 + du * 0.4, v0 + dv * 0.9), (u0 + du, v0 + dv * 0.7), (u0 + du * 0.85, v0 - dv * 0.1), (u0 + du * 0.3, v0 - dv * 0.25)]), region="shaft"))
    elif sc == 100:
        out.append(shape("astragal-line", "column", "detail", 1.1, False, 0.3, polyg([(3.015, -0.08), (3.015, 0.08)], False), region="shaft"))
    for c in "ab":
        out.append(shape(f"cradle-{c}", f"cradle-{c}", "body", 1.8, True, 0.72, rectg(0, -CR_HALF_X, CR_H, 2 * CR_HALF_X), region="cradles", material="softwood", pen=pen_fine))
        if sc == 50:
            for i, rr in enumerate((0.035, 0.06)):
                out.append(shape(f"cradle-{c}-ring-{i + 1}", f"cradle-{c}", "texture", 1.85, False, 0.4, polyg([(CR_H * 0.5 + rr * math.sin(a), 0.02 + rr * math.cos(a)) for a in [math.radians(x) for x in range(-70, 75, 20)]], False), region="cradles"))
            out.append(shape(f"cradle-{c}-check", f"cradle-{c}", "texture", 1.86, False, 0.4, polyg([(0.0, -0.01), (0.03, -0.012), (0.06, -0.006)], False), region="cradles"))
    for c in "ab":
        t = SCALE[sc]["t_sl"]
        u0, u1 = sling_u_range(c, t)
        if sc == 500:
            out.append(shape(f"sling-{c}", f"sling-{c}", "body", 0.6, False, 0.68, polyg([(0.0, 0), (L_SL + W_EYE + t / 2, 0)], False), region="slings", pen=0.1))
        else:
            v0, v1 = sling_v(sc, c)
            out.append(shape(f"sling-{c}", f"sling-{c}", "body", 0.6, True, 0.68, polyg([(u0, v0), (u1, v0), (u1, v1), (u0, v1)]), region="slings", material="polyester-sling", pen=pen_fine))
    return out


def front_plan(sc, view):
    """🔭️ Front or plan of one scale: the column and the slings as solids of their rest profiles (exact in every pose), the cradles flat (they never turn)."""
    p = SCALE[sc]
    out = []
    pen_fine = None
    zb = {"front": {"base-plate": 1.0, "base-moulding": 1.1, "shaft": 1.2, "capital": 1.3, "head-plate": 1.4, "packing-plate": 1.45, "lugs": 1.5, "stub": 1.6},
          "plan": {"base-plate": 2.0, "base-moulding": 2.1, "shaft": 2.2, "capital": 2.3, "head-plate": 2.4, "packing-plate": 2.45, "lugs": 2.5, "stub": 2.6}}[view]
    bp = {50: BASE_PLATE, 100: BASE_PLATE, 200: BASE_PLATE_SQ, 500: BASE_PLATE_SQ}[sc]
    bu = {50: 0.03, 100: 0.03, 200: 0.032, 500: 0.08}[sc]
    out.append(shape("base-plate", "column", "body", zb["base-plate"], True, 0.32, polyg(bp), region="base", material="cast-iron-rusted", pen=pen_fine, solid=ext_u(0, bu)))
    if p["base"]:
        b = p["base"] if sc != 200 else [(0.032, 0.125)] + p["base"][1:]
        out.append(shape("base-moulding", "column", "body", zb["base-moulding"], True, 0.36, polyg(half(b)), region="base", material="cast-iron-rusted", solid=REV))
    out.append(shape("shaft", "column", "body", zb["shaft"], True, 0.45, polyg(half(p["shaft"])), region="shaft", material="cast-iron-painted", solid=REV))
    out.append(shape("capital", "column", "body", zb["capital"], True, 0.42, polyg(half(p["cap"])), region="capital", material="cast-iron-painted", solid=REV))
    hp = {50: (3.32, 3.35), 100: (3.32, 3.356), 200: (3.32, 3.35), 500: (3.30, U_TOP)}[sc]
    out.append(shape("head-plate", "column", "body", zb["head-plate"], True, 0.32, polyg(HEAD_PLATE), region="capital", material="cast-iron-rusted", pen=pen_fine, solid=ext_u(*hp)))
    if sc == 50:
        out.append(shape("packing-plate", "column", "detail", zb["packing-plate"], True, 0.25, polyg(SHIM), region="capital", material="steel-rusted", solid=ext_u(3.35, 3.356)))
    if sc in (50, 100, 200):
        lw = LUG_W if sc != 200 else LUG_W_200
        lu0 = {50: 3.35, 100: 3.356, 200: 3.35}[sc]
        for side, sgn in (("l", 1), ("r", -1)):
            w0, w1 = sorted((sgn * lw[0], sgn * lw[1]))
            out.append(shape(f"lug-{side}", "column", "body", zb["lugs"] + (0.01 if side == "r" else 0), True, 0.34, polyg([(-0.17, w0), (0.17, w0), (0.17, w1), (-0.17, w1)]), region="capital", material="cast-iron-rusted", pen=pen_fine, solid=ext_u(lu0, U_TOP)))
    if sc == 50:
        for i, (v, w) in enumerate(STUBS):
            out.append(shape(f"bolt-stub-{i + 1}", "column", "detail", zb["stub"] + i * 0.01, True, 0.2, polyg(circle(v, w, 0.01, 12)), region="capital", material="steel-rusted", solid=ext_u(3.31, 3.362)))
    for c in "ab":
        s = S_CR[c]
        if view == "front":
            prof = cradle_profile(c, sc == 50)
            out.append(shape(f"cradle-{c}", f"cradle-{c}", "body", 0.5 + (0.05 if c == "b" else 0), True, 0.72, polyg([(w, CR_H - u) for w, u in prof]), region="cradles", material="softwood", pen=pen_fine))
        else:
            out.append(shape(f"cradle-{c}", f"cradle-{c}", "body", 0.5 + (0.05 if c == "b" else 0), True, 0.72, rectg(s - CR_HALF_X, -CR_HALF_Z, 2 * CR_HALF_X, 2 * CR_HALF_Z), region="cradles", material="softwood", pen=pen_fine))
            if sc == 50:
                for i, zz in enumerate((-0.28, -0.17, 0.19, 0.27)):
                    out.append(shape(f"cradle-{c}-grain-{i + 1}", f"cradle-{c}", "texture", 0.58, False, 0.4, polyg([(s - 0.06, zz), (s - 0.01, zz + 0.006), (s + 0.06, zz - 0.004)], False), region="cradles"))
    if sc != 500:
        t = p["t_sl"]
        for c in "ab":
            v0, v1 = sling_v(sc, c)
            zz = 0.3 + (0.02 if c == "b" else 0)
            out.append(shape(f"sling-{c}", f"sling-{c}", "body", zz, True, 0.68, polyg(sling_band(c, t, p["bend"])), region="slings", material="polyester-sling", pen=pen_fine, solid=ext_v(v0, v1)))
            out.append(shape(f"sling-{c}-eye", f"sling-{c}", "body", zz + 0.01, True, 0.68, polyg(sling_eye(t, p["eye"])), region="slings", material="polyester-sling", pen=pen_fine, solid=ext_v(v0, v1)))
    return out


PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35},
        200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
RANGE = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

DECISIONS = {
    50: {
        "silhouette": ("The column lying in its cradles as a turned casting: the square base plate, the moulded base (torus, scotia, upper torus, apophyge), the shaft tapering from 160 to 140 mm, astragal and neck, the bell capital with its round abacus, the square head plate and the two beam lugs; under it the two notched squared bearers and, flat on top of the shaft, the two roundslings. Front and plan are the solids of the same profiles, so they stay true when the column is lifted or stood upright.", "Die in ihren Lagerböcken liegende Stütze als gedrehtes Gussstück: quadratische Fußplatte, profilierte Basis (Wulst, Kehle, oberer Wulst, Anlauf), der von 160 auf 140 mm verjüngte Schaft, Halsring und Hals, das Glockenkapitell mit rundem Abakus, die quadratische Kopfplatte und die beiden Trägerknaggen; darunter die beiden gekerbten Kanthölzer und flach am Boden die beiden Rundschlingen. Vorder- und Draufsicht sind Körper derselben Profile und bleiben richtig, wenn die Stütze gehoben oder aufgestellt wird."),
        "retained": ("Base plate 360 x 360 x 30 mm with its chipped corner, the mortar crust of the old bed (8 mm) on its underside, the profile of the base at its true 7 to 10 mm steps, the 20 mm wall of the cored shaft (hidden), astragal 82 mm radius, the 30 mm head plate with the rusted 6 mm steel packing plate and the two cut M20 bolt stubs between the 20 mm lugs, the 120 degree V seats of the cradles (35 and 25 mm deep) and the 22 mm roundslings with their baskets and eyes.", "Fußplatte 360 x 360 x 30 mm mit abgeplatzter Ecke, die Mörtelkruste des alten Betts (8 mm) auf ihrer Unterseite, das Profil der Basis mit seinen wahren 7- bis 10-mm-Stufen, die 20-mm-Wand des hohl gegossenen Schafts (verdeckt), Halsring mit 82 mm Radius, die 30-mm-Kopfplatte mit dem angerosteten 6-mm-Futterblech und den zwei abgetrennten M20-Schraubenstümpfen zwischen den 20-mm-Knaggen, die 120-Grad-V-Kerben der Lagerböcke (35 und 25 mm tief) und die 22-mm-Rundschlingen mit Korb und Auge."),
        "merged": ("Base plate, mortar and moulded base share the region base; astragal and neck belong to the shaft; abacus, head plate, packing plate, lugs and stubs to the capital.", "Fußplatte, Mörtel und profilierte Basis teilen sich die Region Basis; Halsring und Hals gehören zum Schaft; Abakus, Kopfplatte, Futterblech, Knaggen und Stümpfe zum Kapitell."),
        "omitted": ("The four bolt holes of each plate are not drawn in the views (they face the column axis); the bolt stubs are hidden behind the near lug in the side elevation.", "Die vier Schraubenlöcher jeder Platte werden in den Ansichten nicht gezeichnet (sie liegen in Achsrichtung); die Schraubenstümpfe sind in der Seitenansicht hinter der vorderen Knagge verdeckt."),
        "thinElements": ("Mortar crust 8 mm (0.16 mm) and packing plate 6 mm are filled details; moulding steps, astragal and abacus edges are 0.18 mm detail lines; paint losses and growth rings 0.1 mm texture.", "Mörtelkruste 8 mm (0,16 mm) und Futterblech 6 mm sind gefüllte Details; Profilstufen, Halsring und Abakuskante sind 0,18-mm-Detaillinien; Farbabplatzungen und Jahrringe 0,1-mm-Textur."),
        "hatching": ("No hatch: no view is a section.", "Keine Schraffur: keine Ansicht ist ein Schnitt."),
        "styles": ("Outline 0.35 mm, details 0.18 mm, texture 0.1 mm; painted shaft mid grey, rusted plates darker, timber light, slings mid tone.", "Kontur 0,35 mm, Details 0,18 mm, Textur 0,1 mm; gestrichener Schaft mittelgrau, angerostete Platten dunkler, Holz hell, Schlingen Mittelton."),
        "overlays": ("Shaft, base and capital are separate regions, so the painted shaft and the rusted ends read differently in the thermal view; cradles and slings are regions of their own.", "Schaft, Basis und Kapitell sind getrennte Regionen, so lesen sich gestrichener Schaft und angerostete Enden in der thermischen Ansicht verschieden; Lagerböcke und Schlingen sind eigene Regionen."),
    },
    100: {
        "silhouette": ("The same column with a simplified profile: plates, a base with two steps, the tapered shaft with a plain astragal band, the bell capital, the head plate (packing plate merged into it) and the lugs; cradles with their V seats and the slings.", "Dieselbe Stütze mit vereinfachtem Profil: Platten, Basis mit zwei Stufen, der verjüngte Schaft mit einfachem Halsringband, das Glockenkapitell, die Kopfplatte (Futterblech eingeschmolzen) und die Knaggen; Lagerböcke mit V-Kerben und die Schlingen."),
        "retained": ("Plate thicknesses 30 mm, lugs 20 mm, chipped base corner, the cored shaft wall 20 mm, V seats, slings at their 22 mm section.", "Plattendicken 30 mm, Knaggen 20 mm, abgeplatzte Basisecke, Schaftwand 20 mm, V-Kerben, Schlingen in ihrem 22-mm-Querschnitt."),
        "merged": ("The 6 mm packing plate is merged into the head plate (36 mm); the moulding has five steps instead of eighteen.", "Das 6-mm-Futterblech ist in die Kopfplatte eingeschmolzen (36 mm); das Profil hat fünf statt achtzehn Stufen."),
        "omitted": ("Mortar crust (8 mm) and the cut bolt stubs are below this scale's 15 mm feature; texture of paint losses and timber.", "Mörtelkruste (8 mm) und die Schraubenstümpfe liegen unter dem 15-mm-Merkmal dieses Maßstabs; Textur der Farbabplatzungen und des Holzes."),
        "thinElements": ("Lugs and plates at true thickness (0.2 to 0.36 mm); astragal as one 0.13 mm detail line.", "Knaggen und Platten in wahrer Dicke (0,2 bis 0,36 mm); Halsring als eine 0,13-mm-Detaillinie."),
        "hatching": ("No hatch.", "Keine Schraffur."),
        "styles": ("Outline 0.25 mm, detail 0.13 mm.", "Kontur 0,25 mm, Detail 0,13 mm."),
        "overlays": ("Regions as at 1:50.", "Regionen wie bei 1:50."),
    },
    200: {
        "silhouette": ("A turned column on two bearers: base plate, a base of three steps, the tapered shaft with the astragal as a block, the bell capital, the head plate with the two lugs as one U-section, the cradles and the slings as 30 mm bands.", "Eine gedrehte Stütze auf zwei Kanthölzern: Fußplatte, Basis mit drei Stufen, der verjüngte Schaft mit dem Halsring als Block, das Glockenkapitell, die Kopfplatte mit den beiden Knaggen als ein U-Querschnitt, die Lagerböcke und die Schlingen als 30-mm-Bänder."),
        "retained": ("Taper of the shaft, the outline of base and capital, the V seats, the lugs at the 30 mm feature.", "Verjüngung des Schafts, Umriss von Basis und Kapitell, die V-Kerben, die Knaggen mit dem 30-mm-Merkmal."),
        "merged": ("Head plate and lugs are one section; the chipped corner is filled; the bore is not modelled (closed and invisible).", "Kopfplatte und Knaggen sind ein Querschnitt; die abgeplatzte Ecke ist gefüllt; die Bohrung wird nicht modelliert (geschlossen und unsichtbar)."),
        "omitted": ("All details and texture; the astragal lines.", "Alle Details und Texturen; die Halsringlinien."),
        "thinElements": ("Lugs drawn at 30 mm (true 20 mm) and the slings at 30 mm (true 22 mm), both 0.15 mm on paper.", "Knaggen mit 30 mm (wahr 20 mm) und Schlingen mit 30 mm (wahr 22 mm) gezeichnet, beide 0,15 mm auf dem Papier."),
        "hatching": ("No hatch.", "Keine Schraffur."),
        "styles": ("Outline 0.18 mm for every part, so the 30 mm lugs and slings still print at 1:350.", "Kontur 0,18 mm für jedes Teil, damit die 30-mm-Knaggen und -Schlingen auch bei 1:350 drucken."),
        "overlays": ("Regions as at 1:50.", "Regionen wie bei 1:50."),
    },
    500: {
        "silhouette": ("The column as a tapered bar with a square block at each end, lying on two bearers; the slings as thin lines on top of the shaft.", "Die Stütze als verjüngter Stab mit einem quadratischen Block an jedem Ende, auf zwei Kanthölzern liegend; die Schlingen als dünne Linien oben auf dem Schaft."),
        "retained": ("Length 3.4 m, plate squares 360 and 340 mm, shaft 160 to 140 mm, the bell of the capital, cradle size and seat.", "Länge 3,4 m, Plattenquadrate 360 und 340 mm, Schaft 160 bis 140 mm, die Glocke des Kapitells, Größe und Kerbe der Lagerböcke."),
        "merged": ("Base plate and moulded base are one 80 mm block; abacus, head plate and lugs one 100 mm block; the base moulding flares out of the shaft.", "Fußplatte und profilierte Basis sind ein 80-mm-Block; Abakus, Kopfplatte und Knaggen ein 100-mm-Block; die Basis läuft als Anlauf aus dem Schaft."),
        "omitted": ("Front and plan show no slings (22 mm, 0.04 mm on paper); in the side elevation each is a 0.1 mm line.", "Vorder- und Draufsicht zeigen keine Schlingen (22 mm, 0,04 mm auf dem Papier); in der Seitenansicht ist jede eine 0,1-mm-Linie."),
        "thinElements": ("Shaft 0.3 mm wide on paper; slings as strokes.", "Schaft 0,3 mm breit auf dem Papier; Schlingen als Striche."),
        "hatching": ("No hatch.", "Keine Schraffur."),
        "styles": ("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": ("Regions as at 1:50.", "Regionen wie bei 1:50."),
    },
}


def representation(sc):
    """🗺️ One authored representation."""
    lo, hi = RANGE[sc]
    return {"status": "authored", "denominator": sc, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[sc], "hatches": [],
            "geometry": {"elevation": elevation(sc), "front": front_plan(sc, "front"), "plan": front_plan(sc, "plan")},
            "decisions": {k: tx(v) for k, v in DECISIONS[sc].items()}}


def volume(sc):
    """🧊️ One authored volume: body, thermal (references with regions), structure (members, joints, mass)."""
    col = column_body(sc)
    body = [s for s, _ in col]
    region_of = {s["id"]: r for s, r in col}
    for c in "ab":
        body.append(cradle_body(sc, c))
        region_of[f"cradle-{c}"] = "cradles"
    sl = []
    for c in "ab":
        for s in sling_body(sc, c):
            sl.append(s)
            region_of[s["id"]] = "slings"
    body += sl
    thermal = []
    for s in body:
        ref = {"id": f"th-{s['id']}", "of": s["id"], "region": region_of[s["id"]]}
        thermal.append(ref)
    structure = []
    structure.append({"id": "st-shaft", "of": "shaft", "member": "column", "part": "member"})
    for jid in [s["id"] for s in body if s["bone"] == "column" and s["id"] not in ("shaft",) and not s["id"].startswith("mortar")]:
        structure.append({"id": f"st-{jid}", "of": jid, "member": "column", "part": "joint"})
    for c in "ab":
        structure.append({"id": f"st-cradle-{c}", "of": f"cradle-{c}", "member": f"cradle-{c}", "part": "member"})
    for s in sl:
        structure.append({"id": f"st-{s['id']}", "of": s["id"], "member": s["bone"], "part": "member"})
    rm = {50: 0.035, 100: 0.04, 200: 0.05, 500: 0.06}[sc]
    structure.append({"id": "mass-column", "kind": "ellipsoid", "bone": "column", "material": "cast-iron-painted", "role": "detail", "tone": 0.1, "radii": [rm, rm, rm], "transform": {"translation": [rnd(U_COM), 0, 0]}, "member": "column", "part": "mass"})
    return {"status": "authored", "denominator": sc, "body": body, "thermal": thermal, "structure": structure, "decisions": [tx(d) for d in VOLUME_DECISIONS[sc]]}


VOLUME_DECISIONS = {
    50: [("The 1:50 body is the casting as found: base plate with four 26 mm holes and a chipped corner, two patches of the old 8 mm mortar bed on its underside, the moulded base, the shaft and the bell capital as three turned shells around the cored bore (20 mm wall, closed by the plates), the head plate with four 22 mm holes, the rusted 6 mm packing plate (declared sheet) and two cut M20 stubs, and the two 20 mm lugs. The cradles are 160 x 160 x 700 mm squared bearers with worn arrises and 120 degree V seats cut 35 and 25 mm deep so that both carry the tapered shaft at one axis height; the slings are 22 mm round bands with a basket that follows the elliptical section of the shaft at 55 degrees and an eye for the hook.", "Der 1:50-Körper ist das Gussstück im vorgefundenen Zustand: Fußplatte mit vier 26-mm-Löchern und abgeplatzter Ecke, zwei Flecken des alten 8-mm-Mörtelbetts auf ihrer Unterseite, die profilierte Basis, der Schaft und das Glockenkapitell als drei gedrehte Schalen um die gegossene Bohrung (20 mm Wand, durch die Platten verschlossen), die Kopfplatte mit vier 22-mm-Löchern, das angerostete 6-mm-Futterblech (als Blech deklariert) und zwei abgetrennte M20-Stümpfe sowie die beiden 20-mm-Knaggen. Die Lagerböcke sind Kanthölzer 160 x 160 x 700 mm mit abgenutzten Kanten und 120-Grad-V-Kerben, 35 und 25 mm tief geschnitten, damit beide den verjüngten Schaft in einer Achshöhe tragen; die Schlingen sind 22-mm-Rundbänder mit einem Korb, der dem elliptischen Schnitt des Schafts unter 55 Grad folgt, und einem Auge für den Haken."),
         ("Thermal: every body solid is its own thermal surface ({of} references) in five regions; the closed bore is a cavity the air cannot reach. Structure: the hollow shaft is the member of the column bone, base, capital, plates and lugs are joint solids, each cradle and each sling is the member of its bone, and a marker sits at the centre of mass of the column.", "Thermik: jeder Körper ist seine eigene thermische Oberfläche ({of}-Verweise) in fünf Regionen; die geschlossene Bohrung ist ein Hohlraum, den die Luft nicht erreicht. Tragwerk: der hohle Schaft ist das Stabglied des Stützenknochens, Basis, Kapitell, Platten und Knaggen sind Knotenkörper, jeder Lagerbock und jede Schlinge ist das Stabglied ihres Knochens, und eine Marke sitzt im Schwerpunkt der Stütze.")],
    100: [("At 1:100 (15 mm feature) the mortar crust and the bolt stubs are left out, the packing plate is merged into a 36 mm head plate, the plate holes are closed, the base profile has five steps and the capital four; the shaft keeps its cored 20 mm wall, cradles their V seats, slings their 22 mm section with coarser bends.", "Bei 1:100 (15-mm-Merkmal) entfallen Mörtelkruste und Schraubenstümpfe, das Futterblech ist in eine 36-mm-Kopfplatte eingeschmolzen, die Plattenlöcher sind geschlossen, das Basisprofil hat fünf Stufen und das Kapitell vier; der Schaft behält seine 20-mm-Wand, die Lagerböcke ihre V-Kerben, die Schlingen ihren 22-mm-Querschnitt mit gröberen Bögen."),
          ("Thermal and structure lists follow the same solids as at 1:50 (references), without the parts that this scale omits.", "Thermik- und Tragwerksliste folgen denselben Körpern wie bei 1:50 (Verweise), ohne die bei diesem Maßstab entfallenen Teile.")],
    200: [("At 1:200 (30 mm feature) the column is modelled solid (the closed bore is invisible), the base plate is 32 mm and square, the base has three steps, the astragal is a block, and head plate and lugs are one U-section extrusion with the lugs at 30 mm; the slings are generalised to 30 mm bands (true 22 mm) lying on top of the shaft.", "Bei 1:200 (30-mm-Merkmal) wird die Stütze voll modelliert (die geschlossene Bohrung ist unsichtbar), die Fußplatte ist 32 mm dick und quadratisch, die Basis hat drei Stufen, der Halsring ist ein Block, und Kopfplatte und Knaggen sind eine Extrusion mit U-Querschnitt mit 30-mm-Knaggen; die Schlingen sind zu 30-mm-Bändern verallgemeinert (wahr 22 mm), die oben auf dem Schaft liegen."),
          ("Thermal and structure lists reference these solids.", "Thermik- und Tragwerksliste verweisen auf diese Körper.")],
    500: [("At 1:500 (75 mm feature) the column is an 80 mm base block, one turned bar from the flare of the base to the neck, the bell capital and a 100 mm head block; the cradles keep their seats; each sling is two slender 22 mm legs at their true section from the shaft to the hook, the basket and the eye are below the feature and left out.", "Bei 1:500 (75-mm-Merkmal) ist die Stütze ein 80-mm-Fußblock, ein gedrehter Stab vom Anlauf der Basis bis zum Hals, das Glockenkapitell und ein 100-mm-Kopfblock; die Lagerböcke behalten ihre Kerben; jede Schlinge besteht aus zwei schlanken 22-mm-Strängen in wahrem Querschnitt vom Schaft bis zum Haken, Korb und Auge liegen unter dem Merkmal und entfallen."),
          ("Thermal and structure lists reference these solids; the slender legs are the sling members.", "Thermik- und Tragwerksliste verweisen auf diese Körper; die schlanken Stränge sind die Schlingenglieder.")],
}


def thermal_model(areas):
    """🌡️ Five passive regions with weathered and painted surface data."""
    def region(rid, title, kind, material, area, char, geom, sky, cap, src, ref):
        return {"id": rid, "title": tx(title), "kind": kind, "model": "passive", "material": material, "areaM2": area, "characteristicLengthM": char, "geometry": geom, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": cap, "source": src, "reference": ref}
    return {
        "environment": {"airTempC": 18, "meanRadiantTempC": 18, "skyTempC": 6, "groundTempC": 16, "relativeHumidity": 0.6, "windMS": 1.5, "windDirectionDeg": 90, "solarWM2": 550, "diffuseFraction": 0.25, "pressurePa": 101325},
        "operating": [],
        "regions": [
            region("shaft", ("Painted shaft (old oil paint over red lead, weathered)", "Gestrichener Schaft (alte Ölfarbe über Bleimennige, verwittert)"), "surface", "cast-iron-painted", areas["shaft"], 0.15, "cylinder", 0.5, 65000, "authored-estimate", "en-iso-13786"),
            region("base", ("Base plate and moulded base, rusted, with mortar crust", "Fußplatte und profilierte Basis, angerostet, mit Mörtelkruste"), "surface", "cast-iron-rusted", areas["base"], 0.36, "vertical-plate", 0.5, 110000, "authored-estimate", "en-iso-13786"),
            region("capital", ("Capital, head plate, lugs, packing plate and stubs, rusted", "Kapitell, Kopfplatte, Knaggen, Futterblech und Stümpfe, angerostet"), "surface", "cast-iron-rusted", areas["capital"], 0.34, "vertical-plate", 0.5, 100000, "authored-estimate", "en-iso-13786"),
            region("cradles", ("Weathered softwood cradles", "Verwitterte Lagerböcke aus Nadelholz"), "surface", "softwood", areas["cradles"], 0.16, "horizontal-plate-up", 0.7, 24000, "authored-estimate", "en-iso-13786"),
            region("slings", ("Polyester roundslings", "Polyester-Rundschlingen"), "surface", "polyester-sling", areas["slings"], 0.022, "cylinder", 0.5, 12000, "authored-estimate", "en-iso-10456"),
        ],
        "emitters": [],
        "assumptions": [
            tx(("The column is grey cast iron with a 20 mm wall; the painted shaft stores heat in its whole wall (7100 kg/m3 x 460 J/(kg K) x 0.02 m = 65 kJ/(m2 K)), the thicker base and capital about 110 and 100 kJ/(m2 K).", "Die Stütze ist Grauguss mit 20 mm Wand; der gestrichene Schaft speichert Wärme in der ganzen Wand (7100 kg/m3 x 460 J/(kg K) x 0,02 m = 65 kJ/(m2 K)), die dickere Basis und das Kapitell etwa 110 und 100 kJ/(m2 K).")),
            tx(("Weathered surfaces differ from new ones: the old oil paint of the shaft has emissivity 0.90 and solar absorptance 0.72 (grey-green, chalked), the rusted ends 0.80 and 0.78; new paint would be about 0.92 and 0.55 to 0.65 (authored estimates after ASHRAE).", "Verwitterte Oberflächen unterscheiden sich von neuen: die alte Ölfarbe des Schafts hat Emissionsgrad 0,90 und solare Absorption 0,72 (graugrün, gekreidet), die angerosteten Enden 0,80 und 0,78; neue Farbe läge bei etwa 0,92 und 0,55 bis 0,65 (Schätzungen nach ASHRAE).")),
            tx(("Timber stores heat in about 36 mm (half the 24 h penetration depth of 73 mm for 0.13 W/(m K), 420 kg/m3, 1600 J/(kg K)); the slings in their 11 mm half thickness.", "Holz speichert Wärme in etwa 36 mm (die Hälfte der 24-h-Eindringtiefe von 73 mm für 0,13 W/(m K), 420 kg/m3, 1600 J/(kg K)); die Schlingen in ihrer halben Dicke von 11 mm.")),
            tx(("Reference case: a mild sunny day in the yard; the reinstalled poses stand indoors out of the sun at the same air temperature. The column is no device and has no nameplate (0 W).", "Referenzfall: ein milder sonniger Tag auf dem Hof; die wieder eingebauten Posen stehen drinnen ohne Sonne bei derselben Lufttemperatur. Die Stütze ist kein Gerät und hat kein Typenschild (0 W).")),
        ],
        "passengers": [],
    }


def estimate_areas():
    """📐️ The outer areas of the five regions from the 1:50 solids (the measured values of the validator are written back after the first run)."""
    shaft = surface_of_revolution(SHAFT_OUT_50) + math.pi * (0.086 ** 2 - r_bore(0.15) ** 2) + math.pi * (0.071 ** 2 - r_bore(3.06) ** 2)
    base = (2 * area_poly(BASE_PLATE) - math.pi * (0.118 ** 2) + 4 * 0.36 * 0.03 + 4 * math.pi * 0.026 * 0.03) + surface_of_revolution(BASE_OUT_50) + math.pi * (0.086 ** 2 - r_bore(0.15) ** 2)
    cap = surface_of_revolution(CAP_OUT_50) + math.pi * (0.14 ** 2 - 0.0) + 2 * area_poly(HEAD_PLATE) - math.pi * 0.14 ** 2 + 4 * 0.34 * 0.03 + 2 * (2 * 0.34 * 0.05 + 2 * 0.02 * 0.05 + 0.34 * 0.02)
    cr = sum(2 * area_poly(cradle_profile(c, True)) + 0.16 * (2 * 0.7 + 2 * 0.16) for c in "ab")
    sl = 2 * (2 * (area_poly(sl_outer) - area_poly(sl_hole)) + T_SL * 2 * 2.0)
    return {"shaft": round(shaft, 3), "base": round(base, 3), "capital": round(cap, 3), "cradles": round(cr, 3), "slings": round(sl, 3)}


AREAS_FILE = os.path.join(HERE, "areas.json")


def main():
    """🏁️ Write the document."""
    areas = estimate_areas()
    if os.path.exists(AREAS_FILE):
        areas.update(json.load(open(AREAS_FILE)))
    mass_total = M_COL + m_cr["a"] + m_cr["b"] + 2 * SLING_KG
    heights = Y_A + 0.18
    asset = {
        "id": SLUG, "category": "components",
        "title": tx(("Reclaimed Cast-Iron Column", "Gebrauchte Gusseisenstütze")),
        "summary": tx((f"A Gründerzeit hollow cast-iron column, 3.40 m long, its shaft tapering from 160 to 140 mm over a 20 mm wall, with a moulded base on a 360 mm base plate and a bell capital under a 340 mm head plate with two beam lugs. It was taken out of a building of about 1890: the old bedding mortar still clings to the base plate, one corner is chipped, a rusted packing plate and two cut bolt stubs remain on the head plate, the grey-green oil paint is chalked and flaking. In the yard it lies in two notched squared bearers, is lifted on two roundslings and is reinstalled as a pin-ended column under 250 kN.", f"Eine hohl gegossene Gründerzeit-Gusseisenstütze, 3,40 m lang, ihr Schaft verjüngt sich über einer 20-mm-Wand von 160 auf 140 mm, mit profilierter Basis auf einer 360-mm-Fußplatte und einem Glockenkapitell unter einer 340-mm-Kopfplatte mit zwei Trägerknaggen. Sie stammt aus einem Gebäude um 1890: der alte Mörtel haftet noch an der Fußplatte, eine Ecke ist abgeplatzt, ein angerostetes Futterblech und zwei abgetrennte Schraubenstümpfe sitzen noch auf der Kopfplatte, die graugrüne Ölfarbe ist gekreidet und blättert. Auf dem Hof liegt sie in zwei gekerbten Kanthölzern, wird an zwei Rundschlingen gehoben und als gelenkige Stütze unter 250 kN wieder eingebaut.")),
        "tags": ["component", "reclaimed", "cast-iron", "column", "structural", "historic", "gruenderzeit", "compression"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFS],
                       "notes": tx((f"Dimensions authored after typical hollow cast-iron columns of 1870 to 1900 (Ahnert and Krause, Band III); no single object was measured. Mass {M_COL:.1f} kg of cast iron at 7100 kg/m3, centre of mass {U_COM:.3f} m from the base underside; cradles at {S_A:.3f} and {S_B:.3f} m, sling baskets at {sling_station('a'):.3f} and {sling_station('b'):.3f} m.", f"Maße nach typischen hohl gegossenen Gusseisenstützen 1870 bis 1900 (Ahnert und Krause, Band III); kein einzelnes Objekt wurde gemessen. Masse {M_COL:.1f} kg Grauguss bei 7100 kg/m3, Schwerpunkt {U_COM:.3f} m über der Unterseite der Fußplatte; Lagerböcke bei {S_A:.3f} und {S_B:.3f} m, Schlingenkörbe bei {sling_station('a'):.3f} und {sling_station('b'):.3f} m."))},
        "dimensions": {"heightM": round(heights, 4), "widthM": round(U_TOP + MORTAR_T, 4), "depthM": 2 * CR_HALF_Z, "massKg": round(mass_total, 3)},
        "materials": materials(),
        "rig": rig(),
        "anchors": anchors(),
        "representations": {str(sc): representation(sc) for sc in (50, 100, 200, 500)},
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal_model(areas),
        "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [],
                      "assumptions": [
                          tx(("Each pose has its own supports (pose contacts): stored, the four ground contacts of the two cradles; lifted, the crane hook as two rollers pulling along the slings at 55 degrees; reinstalled, a pinned footing and the head held forward and laterally by the floor beam, so the column is a pendulum column (Pendelstütze) as cast-iron columns were designed.", "Jede Pose hat ihre eigenen Auflager (Kontakte der Pose): gelagert die vier Bodenkontakte der beiden Lagerböcke; gehoben der Kranhaken als zwei Rollenlager, die längs der Schlingen unter 55 Grad ziehen; wieder eingebaut ein gelenkiger Fuß und der vom Deckenträger vorwärts und seitlich gehaltene Kopf, die Stütze ist also eine Pendelstütze, wie Gusseisenstützen bemessen wurden.")),
                          tx((f"The shaft is a hollow circle 150 / 110 mm at mid-height (A = {SEC_A * 1e4:.1f} cm2, I = {SEC_I * 1e8:.0f} cm4), cast iron E = 100 GPa (EN 1561, middle of 78 to 103 GPa for historic grey iron). Euler load of the pinned 3.35 m column with the top section 140 / 100 mm: about 1.2 MN, so the 250 kN service load is governed by the compressive stress (about 31 MPa), not by buckling; second-order effects are not part of the statics.", f"Der Schaft ist ein Kreisring 150 / 110 mm in halber Höhe (A = {SEC_A * 1e4:.1f} cm2, I = {SEC_I * 1e8:.0f} cm4), Gusseisen E = 100 GPa (EN 1561, Mitte von 78 bis 103 GPa für historischen Grauguss). Eulerlast der gelenkigen 3,35-m-Stütze mit dem Kopfquerschnitt 140 / 100 mm: etwa 1,2 MN, die Gebrauchslast 250 kN wird also von der Druckspannung (etwa 31 MPa) bestimmt, nicht vom Knicken; Effekte zweiter Ordnung sind nicht Teil der Statik.")),
                          tx(("The mass of the column is a uniform shaft along the bone plus point masses of the base and the capital, chosen so that total mass and centre of mass equal those of the 1:50 casting; cradles and slings carry their own point masses.", "Die Masse der Stütze ist ein gleichmäßiger Schaft längs des Knochens zuzüglich Punktmassen von Basis und Kapitell, so gewählt, dass Gesamtmasse und Schwerpunkt denen des 1:50-Gussstücks gleichen; Lagerböcke und Schlingen tragen eigene Punktmassen.")),
                          tx(("The rig is one tree with the column at its root: lying, the cradle reactions reach the column at their stations (a two-point support with overhangs), hanging, the sling forces at the baskets. The cost of one rigid body: lifted, the weight of the two cradles (about 2 x 7 kg) hangs on the column at the cradle stations although they stay on the ground (about 5 % of the lifted weight); stored, the slings rest on the shaft at their stations, which is where the tree carries them.", "Das Rig ist ein Baum mit der Stütze als Wurzel: liegend erreichen die Auflagerkräfte der Lagerböcke die Stütze an ihren Stationen (Zweipunktlagerung mit Kragarmen), hängend die Schlingenkräfte an den Körben. Der Preis eines starren Körpers: gehoben hängt das Gewicht der beiden Lagerböcke (etwa 2 x 7 kg) an den Lagerbockstationen an der Stütze, obwohl sie am Boden bleiben (etwa 5 % des gehobenen Gewichts); gelagert liegen die Schlingen an ihren Stationen auf dem Schaft, genau dort, wo der Baum sie trägt.")),
                      ], "clinical": False},
        "volumes": {str(sc): volume(sc) for sc in (50, 100, 200, 500)},
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    print(json.dumps({"M_COL": M_COL, "m_base": m_base, "u_base": u_base, "m_shaft": m_shaft, "u_shaft": u_shaft, "m_capt": m_capt, "u_capt": u_capt, "U_COM": U_COM, "mb_model": mb_model, "mc_model": mc_model,
                      "m_cr": m_cr, "Y_A": Y_A, "NB_B": NB_B, "S_A": S_A, "S_B": S_B, "X_HOOK": X_HOOK, "L_SL": L_SL, "X_O": X_O, "X_RS": X_RS, "areas": areas, "mass_total": mass_total, "heightM": heights,
                      "parts": {"bplate": m_bplate, "mortar": m_mortar, "bmould": m_bmould, "cap": m_cap, "hplate": m_hplate, "shim": m_shim, "lugs": m_lugs, "stubs": m_stubs}, "A": SEC_A, "I": SEC_I}, indent=1))


if __name__ == "__main__":
    main()
