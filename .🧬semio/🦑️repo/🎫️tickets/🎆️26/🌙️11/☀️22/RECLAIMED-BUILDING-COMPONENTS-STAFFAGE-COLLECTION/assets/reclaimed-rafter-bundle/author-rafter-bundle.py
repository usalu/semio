"""🪵️ Authoritative author script of the staffage asset `reclaimed-rafter-bundle`: every hand-chosen number of the twelve reclaimed 80 x 160 mm softwood rafters, their stickered and strapped bundle on two bearers, the two round slings of the lift and the one rafter reinstalled on wall plate and ridge purlin, written as `🔣️.json` next to this file.

Run: `python3 author-rafter-bundle.py` (prints the measurement table of the report).

@see ../../brief.md — the asset contract of the ticket
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — Pipeline, Conventions, Front and plan views, Volume
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE / "🔣️.json"

PITCH = 40.0
SN, CS = math.sin(math.radians(PITCH)), math.cos(math.radians(PITCH))
TN = SN / CS
L = 4.50
B = 0.08
H = 0.16
D = H * TN
NOTCH = 0.04
SW = 0.82
PURLIN = 0.16
SR = (L - D + NOTCH * TN) - (PURLIN / 2) / CS
GAP = 0.025
PITCH_Z = B + GAP
COLS = [round((k - 2.5) * PITCH_Z, 4) for k in range(6)]
HALF = 3 * B + 2.5 * GAP
STK_H = 0.03
STK_W = 0.05
STK_L = 0.80
V_LOW = 0.0
V_UP = H + STK_H
TOP = V_UP + H
BEARER = 0.10
BEARER_L = 1.00
X_BEAR = (0.95, 3.55)
X_STRAP = (0.45, 4.05)
STRAP_W = 0.032
STRAP_T = 0.0015
X_SLING = (1.15, 3.35)
LOOP_W = 0.04
SLING_D = 0.03
LEG_A = 2.10
LEG_Z = 0.0
LEG_V = TOP + math.sqrt((SLING_D / 2) ** 2 - (GAP / 2) ** 2)
REACH = (X_SLING[1] - X_SLING[0]) / 2
THETA_A = math.degrees(math.acos(REACH / LEG_A))
RISE = LEG_A * math.sin(math.radians(THETA_A))
THETA_B = math.degrees(math.atan2(RISE - SLING_D, REACH))
LEG_B = math.hypot(REACH, RISE - SLING_D)
LEGS = (LEG_A, LEG_B)
LIFT = 0.45
RZ = COLS[5]
RAFTER_V = V_UP + H / 2
INSTALL = 1.75
CHIP = 0.03
NAIL_R = 0.0028

RHO_RAFTER = 420.0
RHO_YARD = 450.0
RHO_STRAP = 520.0
RHO_SLING = 220.0

ROOF_DEAD = 0.55
SPACING = 0.80
SNOW_SK = 0.85
MU1 = 0.8 * (60 - PITCH) / 30
ROOFER = 1000.0


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(x):
    """🔢️ A number rounded to the micrometre for the document."""
    return round(x + 0.0, 6)


def pts(seq):
    """📍️ Rounded point lists."""
    return [[r6(a), r6(b)] for a, b in seq]


def notch(s):
    """🐦️ The three points of one birdsmouth (plumb cut, apex, seat cut) at the apex station `s` on the bottom edge of a rafter centred on v = 0."""
    return [(s - NOTCH * TN, -H / 2), (s, -H / 2 + NOTCH), (s + NOTCH / TN, -H / 2)]


def rafter_profile(notches=True, chip_foot=False, chip_top=False, dv=0.0):
    """✂️ The side profile of one rafter: plumb cut at foot and ridge, the two birdsmouths, optionally a splintered acute tip, shifted by `dv`."""
    p = []
    if chip_foot:
        p.append((CHIP, -H / 2))
    else:
        p.append((0.0, -H / 2))
    if notches:
        p += notch(SW) + notch(SR)
    p.append((L - D, -H / 2))
    if chip_top:
        p += [(L - CHIP * SN, H / 2 - CHIP * CS), (L - CHIP, H / 2)]
    else:
        p.append((L, H / 2))
    p.append((D, H / 2))
    if chip_foot:
        p.append((CHIP * SN, -H / 2 + CHIP * CS))
    return [(x, y + dv) for x, y in p]


def block_profile(notches):
    """🧱️ The merged side profile of the two layers and the sticker slot between them (1:200, 1:500)."""
    low = [(0.0, 0.0)]
    if notches:
        low += [(x, y + H / 2) for x, y in notch(SW) + notch(SR)]
    return low + [(L - D, 0.0), (L, H), (L - D, H), (L - D, V_UP), (L, TOP), (D, TOP), (0.0, V_UP), (D, V_UP), (D, H)]


def lower_slot_profile(notches):
    """🧱️ The lower layer with the sticker slot merged onto it (1:200, 1:500 bodies)."""
    low = [(0.0, 0.0)]
    if notches:
        low += [(x, y + H / 2) for x, y in notch(SW) + notch(SR)]
    return low + [(L - D, 0.0), (L, H), (L - D, H), (L - D, V_UP), (D, V_UP), (D, H)]


def upper_profile():
    """🧱️ The upper layer as one parallelogram (1:200, 1:500 bodies)."""
    return [(0.0, V_UP), (L - D, V_UP), (L, TOP), (D, TOP)]


def poly_area(p):
    """📐️ Shoelace area of a simple polygon."""
    return abs(sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))) / 2


def poly_perimeter(p):
    """📏️ Perimeter of a closed polygon."""
    return sum(math.dist(p[i], p[(i + 1) % len(p)]) for i in range(len(p)))


def extrude_area(p, t):
    """🧮️ Surface of a prism of profile `p` and length `t`."""
    return 2 * poly_area(p) + poly_perimeter(p) * t


def rounded_box_area(a, b, c, r):
    """🧮️ Surface of a box with rounded edges and corners of radius `r`."""
    flat = 2 * ((a - 2 * r) * (b - 2 * r) + (b - 2 * r) * (c - 2 * r) + (c - 2 * r) * (a - 2 * r))
    edges = math.pi / 2 * r * 4 * ((a - 2 * r) + (b - 2 * r) + (c - 2 * r))
    return flat + edges + 4 * math.pi * r * r


def rounded_box_volume(a, b, c, r):
    """🧮️ Volume of a box with rounded edges and corners of radius `r`."""
    return a * b * c - (4 - math.pi) * r * r * ((a - 2 * r) + (b - 2 * r) + (c - 2 * r)) - (8 - 4 / 3 * math.pi) * r ** 3


def ring_profiles(v0, v1, w, t):
    """⭕️ Outer ring and hole of a band of thickness `t` hugging the bundle envelope v0..v1, w = ±w."""
    outer = [(v0 - t, -w - t), (v1 + t, -w - t), (v1 + t, w + t), (v0 - t, w + t)]
    hole = [(v0, -w), (v1, -w), (v1, w), (v0, w)]
    return outer, hole


def circle_path(cx, cy, r):
    """🕳️ One SVG circle subpath."""
    return f"M {r6(cx + r)} {r6(cy)} A {r6(r)} {r6(r)} 0 1 0 {r6(cx - r)} {r6(cy)} A {r6(r)} {r6(r)} 0 1 0 {r6(cx + r)} {r6(cy)} Z"


def rect_path(x0, y0, x1, y1):
    """▭️ One SVG rectangle subpath."""
    return f"M {r6(x0)} {r6(y0)} L {r6(x1)} {r6(y0)} L {r6(x1)} {r6(y1)} L {r6(x0)} {r6(y1)} Z"


def poly_path(p):
    """🔷️ One SVG polygon subpath."""
    return "M " + " L ".join(f"{r6(x)} {r6(y)}" for x, y in p) + " Z"


def shape(sid, bone, role, z, filled, tone, geo, region=None, regions=None, material=None, solid=None, pen=None, hatch=None):
    """✏️ One authored 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    s.update(geo)
    if solid:
        s["solid"] = solid
    return s


def rect(x, y, w, h, r=None):
    """⬛️ Rect geometry."""
    g = {"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}
    if r:
        g["r"] = r
    return g


def poly(p, closed=True):
    """🔷️ Poly geometry."""
    return {"kind": "poly", "points": pts(p), "closed": closed}


def path(d):
    """〰️ Path geometry."""
    return {"kind": "path", "d": d}


def wsolid(z0, z1):
    """🌀️ A lateral extrusion placement for front and plan shapes of a bone that turns."""
    return {"kind": "extruded", "sweep": "w", "z0": r6(z0), "z1": r6(z1)}


def solid(sid, bone, material, tone, geo, **extra):
    """🧊️ One volume solid of the body list."""
    s = {"id": sid, "bone": bone, "material": material, "role": "body", "tone": tone}
    s.update(geo)
    s.update(extra)
    return s


def xy_extrude(p, w0, w1):
    """🧊️ An extrude of a side profile across w0..w1."""
    return {"kind": "extrude", "profile": pts(p), "plane": "xy", "from": r6(w0), "to": r6(w1)}


def yz_ring(outer, hole, u0, u1):
    """🧊️ A band extruded along u from a (v, w) ring with its hole."""
    return {"kind": "extrude", "profile": pts(outer), "holes": [pts(hole)], "plane": "yz", "from": r6(u0), "to": r6(u1)}


def box(size, at, r=None):
    """🧊️ A box at a bone-local centre."""
    g = {"kind": "box", "size": [r6(v) for v in size], "transform": {"translation": [r6(v) for v in at]}}
    if r:
        g["cornerRadius"] = r
    return g


def rod(length, radius):
    """🧊️ A cylinder along u from the bone origin."""
    return {"kind": "revolve", "profile": pts([(0, 0), (0, radius), (length, radius), (length, 0)]), "axis": [1, 0, 0], "origin": [0, 0, 0]}


def repeat(count, dw):
    """🔁️ Copies stepped across the bundle."""
    return {"count": count, "translation": [0, 0, r6(dw)]}


# ----------------------------------------------------------------------------- geometry numbers

PROF = rafter_profile()
PROF_CHIP_FOOT = rafter_profile(chip_foot=True)
PROF_CHIP_TOP = rafter_profile(chip_top=True)
PROF_PLAIN = rafter_profile(notches=False)
NOTCH_AREA = 2 * 0.5 * (NOTCH * TN + NOTCH / TN) * NOTCH
RAFTER_GROSS_V = (L - D) * H * B
RAFTER_VOL = poly_area(PROF) * B
RAFTER_V_CHIP = poly_area(PROF_CHIP_FOOT) * B
SEAT_HALF = NOTCH / SN / 2
WALL_U, WALL_V = SW + SEAT_HALF * CS, -H / 2 + NOTCH - SEAT_HALF * SN
RIDGE_U, RIDGE_V = SR + SEAT_HALF * CS, WALL_V
APEX_W = (SW * CS - (-H / 2 + NOTCH) * SN, SW * SN + (-H / 2 + NOTCH) * CS + BEARER + RAFTER_V + INSTALL)
APEX_R = (SR * CS - (-H / 2 + NOTCH) * SN, SR * SN + (-H / 2 + NOTCH) * CS + BEARER + RAFTER_V + INSTALL)
SPAN_H = APEX_R[0] - APEX_W[0]

STRAP_OUT, STRAP_HOLE = ring_profiles(0.0, TOP, HALF, STRAP_T)
LOOP_OUT, LOOP_HOLE = ring_profiles(0.0, TOP, HALF, SLING_D)
STRAP_V = (poly_area(STRAP_OUT) - poly_area(STRAP_HOLE)) * STRAP_W
LOOP_V = (poly_area(LOOP_OUT) - poly_area(LOOP_HOLE)) * LOOP_W
LEG_VOL = math.pi * (SLING_D / 2) ** 2 * (LEG_A + LEG_B) / 2
BUCKLE = (0.036, 0.008, 0.045)
BUCKLE_W = 0.10
BEAR_R = 0.008

M_RAFTER = RAFTER_VOL * RHO_RAFTER
M_RAFTER_REIN = RAFTER_V_CHIP * RHO_RAFTER
M_STICKER = STK_W * STK_H * STK_L * RHO_YARD
M_BEARER = rounded_box_volume(BEARER, BEARER, BEARER_L, BEAR_R) * RHO_YARD
M_STRAP = STRAP_V * RHO_STRAP + BUCKLE[0] * BUCKLE[1] * BUCKLE[2] * 7850
M_LOOP = LOOP_V * RHO_SLING
M_LEG = LEG_VOL * RHO_SLING
N_CHIPPED_BUNDLE = 2
M_BUNDLE = 9 * M_RAFTER + N_CHIPPED_BUNDLE * (poly_area(PROF_CHIP_TOP) * B * RHO_RAFTER) + 2 * M_STICKER + 2 * M_STRAP + 2 * M_LOOP
M_TOTAL = M_BUNDLE + M_RAFTER_REIN + 2 * M_BEARER + 2 * M_LEG
COM_V_BUNDLE = (
    (6 * M_RAFTER) * (H / 2) + (5 * M_RAFTER) * (V_UP + H / 2) + 2 * M_STICKER * (H + STK_H / 2) + 2 * M_STRAP * (TOP / 2) + 2 * M_LOOP * (TOP / 2)
) / (11 * M_RAFTER + 2 * M_STICKER + 2 * M_STRAP + 2 * M_LOOP)

A_RAF = B * H
I_RAF = B * H ** 3 / 12
A_SLING = math.pi * SLING_D ** 2 / 4
I_SLING = math.pi * SLING_D ** 4 / 64

G_DEAD = ROOF_DEAD * SPACING * 1000
S_SLOPE = MU1 * SNOW_SK * SPACING * CS * 1000

AREA_RAFTER = extrude_area(PROF, B)
AREA_RAFTER_CHIP_FOOT = extrude_area(PROF_CHIP_FOOT, B)
AREA_RAFTER_CHIP_TOP = extrude_area(PROF_CHIP_TOP, B)
AREA_BUNDLE = 9 * AREA_RAFTER + 2 * AREA_RAFTER_CHIP_TOP
AREA_BEARER = rounded_box_area(BEARER, BEARER, BEARER_L, BEAR_R)
AREA_STICKER = 2 * (STK_W * STK_H + STK_H * STK_L + STK_L * STK_W)
AREA_STORAGE = 2 * AREA_BEARER + 2 * AREA_STICKER
AREA_STRAP = 2 * (poly_area(STRAP_OUT) - poly_area(STRAP_HOLE)) + (poly_perimeter(STRAP_OUT) + poly_perimeter(STRAP_HOLE)) * STRAP_W
AREA_BUCKLE = 2 * (BUCKLE[0] * BUCKLE[1] + BUCKLE[1] * BUCKLE[2] + BUCKLE[2] * BUCKLE[0]) - 2 * BUCKLE[0] * BUCKLE[2]
AREA_STRAPS = 2 * (AREA_STRAP + AREA_BUCKLE)
AREA_LOOP = 2 * (poly_area(LOOP_OUT) - poly_area(LOOP_HOLE)) + (poly_perimeter(LOOP_OUT) + poly_perimeter(LOOP_HOLE)) * LOOP_W
AREA_LEG = 2 * math.pi * (SLING_D / 2) * (LEG_A + LEG_B) / 2 + 2 * math.pi * (SLING_D / 2) ** 2 - (2 * math.pi * (SLING_D / 2) * (LOOP_W / 2) + math.pi * (SLING_D / 2) ** 2)
AREA_SLINGS = 2 * (AREA_LOOP + AREA_LEG)

HEIGHT = LEG_V + SLING_D + SLING_D / 2 + BEARER
DEPTH = BEARER_L

# ----------------------------------------------------------------------------- materials, rig, anchors

MATERIALS = [
    {"id": "reclaimed-softwood", "title": T("Reclaimed softwood rafter timber (spruce/fir, S10 = C24), aged", "Gebrauchtes Nadelholz der Sparren (Fichte/Tanne, S10 = C24), gealtert"), "emissivity": 0.9, "solarAbsorptivity": 0.72, "densityKgM3": RHO_RAFTER, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
    {"id": "yard-softwood", "title": T("Yard softwood of bearers and stickers, weathered grey", "Lagerholz der Unterleger und Stapellatten, vergraut"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO_YARD, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "source": "authored-estimate", "reference": "en-iso-10456"},
    {"id": "polyester-strap", "title": T("Woven polyester cord strap, white", "Gewebtes Polyester-Kraftband, weiß"), "emissivity": 0.88, "solarAbsorptivity": 0.3, "densityKgM3": RHO_STRAP, "conductivityWmK": 0.15, "specificHeatJKgK": 1250, "source": "authored-estimate", "reference": "ashrae-fundamentals"},
    {"id": "polyester-sling", "title": T("Polyester round sling, violet (WLL 1000 kg)", "Polyester-Rundschlinge, violett (WLL 1000 kg)"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": RHO_SLING, "conductivityWmK": 0.15, "specificHeatJKgK": 1250, "elasticModulusPa": 2000000000, "source": "authored-estimate", "reference": "en-1492-2"},
    {"id": "galvanised-steel", "title": T("Galvanised wire buckle", "Verzinkte Drahtschnalle"), "emissivity": 0.3, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "source": "literature", "reference": "en-iso-10456"},
]

BONES = [
    {"id": "yard", "parent": None, "title": T("Yard ground (massless reference)", "Lagerplatzboden (masseloser Bezug)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "bundle", "parent": "yard", "title": T("Bundle of eleven rafters with stickers, straps and sling chokes (lifts vertically)", "Bündel aus elf Sparren mit Stapellatten, Bändern und Schlingenschnürungen (hebt sich senkrecht)"), "x": 0, "y": BEARER, "angleDeg": 0, "lengthM": L, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.8}, "mass": {"kg": r6(M_BUNDLE), "comX": L / 2, "comY": r6(COM_V_BUNDLE), "distribution": "uniform", "source": "literature", "reference": "en-338"}, "section": {"areaM2": r6(11 * A_RAF), "secondMomentM4": float(f"{11 * I_RAF:.6e}"), "extremeFibreM": H / 2, "material": "reclaimed-softwood", "title": T("Eleven rafters 80 x 160 mm side by side, not composite", "Elf Sparren 80 x 160 mm nebeneinander, nicht verbunden")}},
    {"id": "bearer-a", "parent": "bundle", "title": T("Squared-timber bearer at the foot end (stays on the ground)", "Kantholz-Unterleger am Fußende (bleibt am Boden)"), "x": X_BEAR[0], "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.8, "maxM": 0}, "mass": {"kg": r6(M_BEARER), "comX": 0, "comY": -BEARER / 2, "distribution": "point", "source": "authored-estimate", "reference": "en-iso-10456"}},
    {"id": "bearer-b", "parent": "bundle", "title": T("Squared-timber bearer at the ridge end (stays on the ground)", "Kantholz-Unterleger am Firstende (bleibt am Boden)"), "x": X_BEAR[1], "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.8, "maxM": 0}, "mass": {"kg": r6(M_BEARER), "comX": 0, "comY": -BEARER / 2, "distribution": "point", "source": "authored-estimate", "reference": "en-iso-10456"}},
    {"id": "rafter-carrier", "parent": "bundle", "title": T("Reinstallation carrier (massless, lifts the outer upper rafter to the roof)", "Einbauträger (masselos, hebt den äußeren oberen Sparren ins Dach)"), "x": 0, "y": RAFTER_V, "z": RZ, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 2.0}},
    {"id": "rafter", "parent": "rafter-carrier", "title": T("Reinstalled rafter (centroid axis from the foot plumb cut, pitches about its foot)", "Wiedereingebauter Sparren (Schwerachse ab dem Fußlotschnitt, neigt sich um seinen Fuß)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": L, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 45}, "mass": {"kg": r6(M_RAFTER_REIN), "comX": L / 2, "comY": 0, "distribution": "uniform", "source": "literature", "reference": "en-338"}, "section": {"areaM2": r6(A_RAF), "secondMomentM4": float(f"{I_RAF:.6e}"), "extremeFibreM": H / 2, "material": "reclaimed-softwood", "title": T("Rafter 80 x 160 mm, C24", "Sparren 80 x 160 mm, C24")}},
    {"id": "load-carrier", "parent": "rafter", "title": T("Roofer load carrier on the rafter top edge (massless)", "Lastträger Dachdecker auf der Sparrenoberkante (masselos)"), "x": r6(WALL_U), "y": H / 2, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": r6(RIDGE_U - WALL_U)}},
    {"id": "sling-a", "parent": "bundle", "title": T("Round sling A, leg from the choke at 1.15 m to the crane hook", "Rundschlinge A, Strang von der Schnürung bei 1,15 m zum Kranhaken"), "x": X_SLING[0], "y": r6(LEG_V), "angleDeg": 0, "lengthM": LEG_A, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 70}, "mass": {"kg": r6(M_LEG * LEG_A * 2 / (LEG_A + LEG_B)), "comX": LEG_A / 2, "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "en-1492-2"}, "section": {"areaM2": float(f"{A_SLING:.6e}"), "secondMomentM4": float(f"{I_SLING:.6e}"), "extremeFibreM": SLING_D / 2, "material": "polyester-sling", "title": T("Round sling, 30 mm equivalent rod", "Rundschlinge, 30-mm-Ersatzstab")}},
    {"id": "sling-b", "parent": "bundle", "title": T("Round sling B, leg from the choke at 3.35 m to the crane hook", "Rundschlinge B, Strang von der Schnürung bei 3,35 m zum Kranhaken"), "x": X_SLING[1], "y": r6(LEG_V + SLING_D), "angleDeg": 180, "lengthM": r6(LEG_B), "joint": {"type": "revolute", "minDeg": -70, "maxDeg": 0}, "mass": {"kg": r6(M_LEG * LEG_B * 2 / (LEG_A + LEG_B)), "comX": r6(LEG_B / 2), "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "en-1492-2"}, "section": {"areaM2": float(f"{A_SLING:.6e}"), "secondMomentM4": float(f"{I_SLING:.6e}"), "extremeFibreM": SLING_D / 2, "material": "polyester-sling", "title": T("Round sling, 30 mm equivalent rod", "Rundschlinge, 30-mm-Ersatzstab")}},
]

ANCHORS = [
    {"id": "bearer-a-ground", "bone": "bearer-a", "x": 0, "y": -BEARER, "kind": "contact", "title": T("Foot-end bearer on the yard ground", "Unterleger am Fußende auf dem Lagerplatz"), "support": "contact", "footprintLengthM": BEARER, "footprintWidthM": BEARER_L},
    {"id": "bearer-b-ground", "bone": "bearer-b", "x": 0, "y": -BEARER, "kind": "contact", "title": T("Ridge-end bearer on the yard ground", "Unterleger am Firstende auf dem Lagerplatz"), "support": "contact", "footprintLengthM": BEARER, "footprintWidthM": BEARER_L},
    {"id": "rafter-on-sticker-a", "bone": "rafter", "x": X_BEAR[0], "y": -H / 2, "kind": "contact", "title": T("Outer upper rafter on the foot-end sticker", "Äußerer oberer Sparren auf der fußseitigen Stapellatte"), "support": "contact", "footprintLengthM": STK_W, "footprintWidthM": B, "frictionCoefficient": 0},
    {"id": "rafter-on-sticker-b", "bone": "rafter", "x": X_BEAR[1], "y": -H / 2, "kind": "contact", "title": T("Outer upper rafter on the ridge-end sticker", "Äußerer oberer Sparren auf der firstseitigen Stapellatte"), "support": "contact", "footprintLengthM": STK_W, "footprintWidthM": B, "frictionCoefficient": 0},
    {"id": "hook-a", "bone": "sling-a", "x": LEG_A, "y": 0, "kind": "support", "title": T("Eye of sling A in the crane hook, pulls along the leg", "Auge der Schlinge A im Kranhaken, zieht entlang des Strangs"), "support": "roller", "normalDeg": r6(THETA_A)},
    {"id": "hook-b", "bone": "sling-b", "x": r6(LEG_B), "y": 0, "kind": "support", "title": T("Eye of sling B in the crane hook, pulls along the leg", "Auge der Schlinge B im Kranhaken, zieht entlang des Strangs"), "support": "roller", "normalDeg": r6(180 - THETA_B)},
    {"id": "wall-plate", "bone": "rafter", "x": r6(WALL_U), "y": r6(WALL_V), "kind": "support", "title": T("Birdsmouth seat on the wall plate, pin", "Kervensitz auf der Fußpfette, Festlager"), "support": "pin"},
    {"id": "ridge-purlin", "bone": "rafter", "x": r6(RIDGE_U), "y": r6(RIDGE_V), "kind": "support", "title": T("Birdsmouth seat on the ridge purlin, roller", "Kervensitz auf der Firstpfette, Rollenlager"), "support": "roller", "normalDeg": 90},
    {"id": "roofer-load", "bone": "load-carrier", "x": 0, "y": 0, "kind": "load", "title": T("Roofer standing on the rafter", "Dachdecker auf dem Sparren"), "footprintLengthM": 0.1, "footprintWidthM": B},
]

# ----------------------------------------------------------------------------- thermal

REG = ["rafter-reinstalled", "bundle-rafters", "storage-timbers", "straps", "slings"]


def binding(rein_sun, bundle_sun, storage_sun, strap_sun=0.3, sling_sun=0.5, ground=True):
    """🌤️ The region binding of a pose."""
    st = {"exposedFraction": 1, "sunlitFraction": storage_sun}
    if ground:
        st["contact"] = {"fraction": r6(2 * BEARER * BEARER_L / AREA_STORAGE), "tempC": 18}
    return {
        "regions": {
            "rafter-reinstalled": {"exposedFraction": 1, "sunlitFraction": rein_sun},
            "bundle-rafters": {"exposedFraction": 1, "sunlitFraction": bundle_sun},
            "storage-timbers": st,
            "straps": {"exposedFraction": 1, "sunlitFraction": strap_sun},
            "slings": {"exposedFraction": 1, "sunlitFraction": sling_sun},
        },
        "emitters": {},
    }


THERMAL = {
    "minimumAirPassageM": GAP,
    "environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
    "operating": [],
    "regions": [
        {"id": "rafter-reinstalled", "title": T("Reinstalled rafter, all faces", "Wiedereingebauter Sparren, alle Flächen"), "kind": "surface", "model": "passive", "material": "reclaimed-softwood", "areaM2": round(AREA_RAFTER_CHIP_FOOT, 4), "characteristicLengthM": H, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 24000, "source": "authored-estimate", "reference": "en-iso-13786"},
        {"id": "bundle-rafters", "title": T("Eleven bundled rafters, all faces in the stickered stack", "Elf gebündelte Sparren, alle Flächen im gelatteten Stapel"), "kind": "surface", "model": "passive", "material": "reclaimed-softwood", "areaM2": round(AREA_BUNDLE, 4), "characteristicLengthM": H, "geometry": "vertical-plate", "skyViewFactor": 0.3, "layers": [], "heatCapacityJM2K": 24000, "source": "authored-estimate", "reference": "en-iso-13786"},
        {"id": "storage-timbers", "title": T("Bearers and stickers", "Unterleger und Stapellatten"), "kind": "surface", "model": "passive", "material": "yard-softwood", "areaM2": round(AREA_STORAGE, 4), "characteristicLengthM": BEARER, "geometry": "vertical-plate", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": 18000, "source": "authored-estimate", "reference": "en-iso-13786"},
        {"id": "straps", "title": T("Polyester cord straps with wire buckles", "Polyester-Kraftbänder mit Drahtschnallen"), "kind": "surface", "model": "passive", "material": "polyester-strap", "areaM2": round(AREA_STRAPS, 4), "characteristicLengthM": STRAP_W, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 490, "source": "authored-estimate", "reference": "ashrae-fundamentals"},
        {"id": "slings", "title": T("Two violet round slings (chokes and legs)", "Zwei violette Rundschlingen (Schnürungen und Stränge)"), "kind": "surface", "model": "passive", "material": "polyester-sling", "areaM2": round(AREA_SLINGS, 4), "characteristicLengthM": SLING_D, "geometry": "cylinder", "skyViewFactor": 0.6, "layers": [], "heatCapacityJM2K": 2000, "source": "authored-estimate", "reference": "en-1492-2"},
    ],
    "emitters": [],
    "assumptions": [
        T("Timber faces store heat in about 36 mm (half the 24 h penetration depth of 73 mm for lambda 0.13 W/mK, rho 420 kg/m3, c 1600 J/kgK, EN ISO 13786); the 80 mm rafter therefore responds as a whole within a day. Bearers and stickers take 25 mm of 450 kg/m3 yard timber.", "Holzflächen speichern Wärme in etwa 36 mm (die Hälfte der 24-h-Eindringtiefe von 73 mm für Lambda 0,13 W/mK, Rho 420 kg/m³, c 1600 J/kgK, EN ISO 13786); der 80-mm-Sparren reagiert daher innerhalb eines Tages als Ganzes. Unterleger und Stapellatten nehmen 25 mm Lagerholz zu 450 kg/m³ an."),
        T("Aged rafter timber from a roof space is darker than fresh spruce: solar absorptance 0.72 against about 0.5 when new; the bearers and stickers stand outdoors and are weathered grey (0.65). Both are authored estimates guided by the ASHRAE radiation tables; emissivity 0.9 for all timber.", "Gealtertes Sparrenholz aus einem Dachraum ist dunkler als frische Fichte: solarer Absorptionsgrad 0,72 statt etwa 0,5 im Neuzustand; Unterleger und Stapellatten liegen im Freien und sind vergraut (0,65). Beides sind Schätzungen nach den ASHRAE-Strahlungstabellen; Emissionsgrad 0,9 für alles Holz."),
        T("The 25 mm gaps between the rafters and the 30 mm sticker slot are open to the air (minimumAirPassageM), so every rafter face exchanges heat; only the top faces of the upper layer and the sunward side see the sun (sunlit fractions of the poses).", "Die 25-mm-Fugen zwischen den Sparren und der 30-mm-Lattenspalt sind zur Luft offen (minimumAirPassageM), daher tauscht jede Sparrenfläche Wärme aus; nur die Oberseiten der oberen Lage und die sonnenseitige Flanke sehen die Sonne (Besonnungsanteile der Posen)."),
        T("Reinstalled, the rafter lies under the roof covering: shaded, in the air of the roof space taken equal to the outdoor reference case. The bundle is no device and draws no power.", "Wiedereingebaut liegt der Sparren unter der Dachdeckung: verschattet, in der Luft des Dachraums, die dem Außenreferenzfall gleichgesetzt ist. Das Bündel ist kein Gerät und nimmt keine Leistung auf."),
    ],
    "passengers": [],
}

# ----------------------------------------------------------------------------- poses and clips

STORED_CONTACTS = ["bearer-a-ground", "bearer-b-ground", "rafter-on-sticker-a", "rafter-on-sticker-b"]
LIFTED_CONTACTS = ["hook-a", "hook-b", "rafter-on-sticker-a", "rafter-on-sticker-b"]
ROOF_CONTACTS = ["wall-plate", "ridge-purlin"]
INSTALLED = {"rafter-carrier": {"dy": INSTALL}, "rafter": {"angleDeg": PITCH}}
LIFTED_BONES = {"bundle": {"dy": LIFT}, "bearer-a": {"dy": -LIFT}, "bearer-b": {"dy": -LIFT}, "sling-a": {"angleDeg": r6(THETA_A)}, "sling-b": {"angleDeg": r6(-THETA_B)}}
TAUT_BONES = {"sling-a": {"angleDeg": r6(THETA_A)}, "sling-b": {"angleDeg": r6(-THETA_B)}}
DEAD = {"id": "roof-dead-load", "kind": "carried", "title": T(f"Roof covering {ROOF_DEAD:.2f} kN/m² x {SPACING:.2f} m = {G_DEAD / 1000:.2f} kN/m", f"Dachdeckung {ROOF_DEAD:.2f} kN/m² x {SPACING:.2f} m = {G_DEAD / 1000:.2f} kN/m".replace(".", ",")), "fxN": 0, "fyN": 0, "distributed": {"bone": "rafter", "fromM": 0, "toM": L, "wxNPerM": 0, "wyNPerM": -round(G_DEAD, 1)}, "source": "authored-estimate", "reference": "din-en-1991-1-1-na"}
SNOW = {"id": "snow", "kind": "snow", "title": T(f"Snow mu1 s_k = {MU1:.3f} x {SNOW_SK:.2f} kN/m² on plan, {S_SLOPE / 1000:.3f} kN/m along the rafter", f"Schnee mu1 s_k = {MU1:.3f} x {SNOW_SK:.2f} kN/m² Grundfläche, {S_SLOPE / 1000:.3f} kN/m entlang des Sparrens".replace(".", ",")), "fxN": 0, "fyN": 0, "distributed": {"bone": "rafter", "fromM": 0, "toM": L, "wxNPerM": 0, "wyNPerM": -round(S_SLOPE, 1)}, "source": "literature", "reference": "en-1991-1-3"}
G = 9.81
STACK_W = G * (M_BUNDLE + 2 * M_BEARER + 2 * M_LEG)
STACK_MX = G * (M_BUNDLE * L / 2 + M_BEARER * (X_BEAR[0] + X_BEAR[1]) + M_LEG * LEG_A * 2 / (LEG_A + LEG_B) * (X_SLING[0] + LEG_A / 2) + M_LEG * LEG_B * 2 / (LEG_A + LEG_B) * (X_SLING[1] - LEG_B / 2))
STACK_B = (STACK_MX - STACK_W * X_BEAR[0]) / (X_BEAR[1] - X_BEAR[0])
STACK_A = STACK_W - STACK_B


def ground_load(k, newtons, what_en, what_de):
    """🟫️ The known yard-ground reaction under one bearer, applied as a load where the stack and the moved part are two separately supported bodies."""
    return {"id": f"yard-under-bearer-{'ab'[k]}", "kind": "carried", "title": T(f"Yard ground under bearer {'AB'[k]}: {newtons / 1000:.3f} kN ({what_en})", f"Lagerplatz unter Unterleger {'AB'[k]}: {newtons / 1000:.3f} kN ({what_de})".replace(".", ",")), "anchor": f"bearer-{'ab'[k]}-ground", "fxN": 0, "fyN": round(newtons, 1), "source": "authored-estimate", "reference": "en-338"}


ROOF_GROUND = [ground_load(0, STACK_A, "the stack left in the yard", "der auf dem Hof verbliebene Stapel"), ground_load(1, STACK_B, "the stack left in the yard", "der auf dem Hof verbliebene Stapel")]
LIFT_GROUND = [ground_load(k, G * M_BEARER, "the bearer left on the ground", "der am Boden verbliebene Unterleger") for k in range(2)]
ROOFER_LOAD = {"id": "roofer", "kind": "occupant", "title": T("Roofer 1.0 kN (category H)", "Dachdecker 1,0 kN (Kategorie H)"), "anchor": "roofer-load", "fxN": 0, "fyN": -ROOFER, "source": "literature", "reference": "en-1991-1-1"}

POSES = [
    {"id": "stored", "title": T("Stored in the yard", "Gelagert auf dem Hof"), "description": T("Twelve rafters on edge in two stickered layers on two squared-timber bearers, strapped twice, the two round slings choked around the bundle with their legs laid on top.", "Zwölf Sparren hochkant in zwei gelatteten Lagen auf zwei Kantholz-Unterlegern, zweimal umreift, die beiden Rundschlingen um das Bündel geschnürt, ihre Stränge oben aufgelegt."), "bones": {}, "ground": "authored", "contacts": STORED_CONTACTS, "thermal": binding(0.49, 0.1, 0.25), "loads": []},
    {"id": "slings-taut", "key": True, "title": T("Slings in the hook", "Schlingen im Haken"), "description": T("The crane hook has taken both sling eyes; the legs stand at 58 degrees, the bundle still rests on its bearers.", "Der Kranhaken hat beide Schlingenaugen aufgenommen; die Stränge stehen unter 58 Grad, das Bündel liegt noch auf den Unterlegern."), "bones": TAUT_BONES, "ground": "authored", "contacts": STORED_CONTACTS, "thermal": binding(0.49, 0.1, 0.25), "loads": []},
    {"id": "lifted", "title": T("Lifted on two round slings", "An zwei Rundschlingen angehoben"), "description": T(f"The bundle hangs {LIFT:.2f} m above its bearers from the two violet round slings choked at 1.15 m and 3.35 m, legs at 58 degrees to the horizontal, both eyes in the crane hook; the bearers stay on the ground.", f"Das Bündel hängt {LIFT:.2f} m über seinen Unterlegern an den zwei violetten Rundschlingen, geschnürt bei 1,15 m und 3,35 m, Stränge unter 58 Grad zur Waagrechten, beide Augen im Kranhaken; die Unterleger bleiben am Boden.".replace("0.45", "0,45")), "bones": LIFTED_BONES, "ground": "authored", "contacts": LIFTED_CONTACTS, "thermal": binding(0.49, 0.1, 0.35), "loads": LIFT_GROUND},
    {"id": "reinstalled", "title": T("Reinstalled under the roof covering", "Wiedereingebaut unter der Dachdeckung"), "description": T(f"The outer upper rafter is back in a {PITCH:.0f} degree purlin roof: birdsmouth on the wall plate (pin) and on the ridge purlin (roller), horizontal span {SPAN_H:.2f} m between the plate and purlin faces, carrying its roof covering; the rest of the bundle stays in the yard.", f"Der äußere obere Sparren liegt wieder in einem {PITCH:.0f}-Grad-Pfettendach: Kerve auf der Fußpfette (Festlager) und auf der Firstpfette (Rollenlager), waagrechte Stützweite {SPAN_H:.2f} m zwischen den Pfettenaußenkanten, mit seiner Dachdeckung belastet; der Rest des Bündels bleibt auf dem Hof.".replace(f"{SPAN_H:.2f}", f"{SPAN_H:.2f}".replace(".", ","))), "bones": INSTALLED, "ground": "authored", "contacts": ROOF_CONTACTS, "thermal": binding(0.0, 0.15, 0.25), "loads": [DEAD] + ROOF_GROUND},
    {"id": "reinstalled-snow", "title": T("Reinstalled under roof covering and snow", "Wiedereingebaut unter Dachdeckung und Schnee"), "description": T("As reinstalled, with the snow load of zone 2 at low altitude on the 40 degree roof.", "Wie wiedereingebaut, dazu die Schneelast der Zone 2 in geringer Höhe auf dem 40-Grad-Dach."), "bones": INSTALLED, "ground": "authored", "contacts": ROOF_CONTACTS, "thermal": binding(0.0, 0.15, 0.25), "loads": [DEAD, SNOW] + ROOF_GROUND},
    {"id": "reinstalled-roofer", "title": T("Reinstalled with a roofer at mid-span", "Wiedereingebaut mit Dachdecker in Feldmitte"), "description": T("As reinstalled, with the 1.0 kN concentrated load of a roofer standing on the rafter at mid-span.", "Wie wiedereingebaut, dazu die Einzellast 1,0 kN eines Dachdeckers, der in Feldmitte auf dem Sparren steht."), "bones": {**INSTALLED, "load-carrier": {"dx": r6((RIDGE_U - WALL_U) / 2)}}, "ground": "authored", "contacts": ROOF_CONTACTS, "thermal": binding(0.0, 0.15, 0.25), "loads": [DEAD, ROOFER_LOAD] + ROOF_GROUND},
]

CLIPS = [
    {"id": "lift", "title": T("Lift off the bearers", "Abheben von den Unterlegern"), "description": T("The crane takes the two sling eyes, the legs rise to 58 degrees, then the bundle lifts 0.45 m off its bearers, which stay on the ground.", "Der Kran nimmt die beiden Schlingenaugen auf, die Stränge richten sich auf 58 Grad auf, dann hebt das Bündel 0,45 m von seinen Unterlegern ab, die am Boden bleiben."), "durationS": 4, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "out"}, {"t": 1.5, "pose": "slings-taut", "ease": "linear"}, {"t": 3.5, "pose": "lifted", "ease": "step"}, {"t": 4, "pose": "lifted", "ease": "step"}],
     "contacts": [{"anchor": "bearer-a-ground", "fromS": 0, "toS": 1.5}, {"anchor": "bearer-b-ground", "fromS": 0, "toS": 1.5}, {"anchor": "hook-a", "fromS": 1.5, "toS": 4}, {"anchor": "hook-b", "fromS": 1.5, "toS": 4}, {"anchor": "rafter-on-sticker-a", "fromS": 0, "toS": 4}, {"anchor": "rafter-on-sticker-b", "fromS": 0, "toS": 4}],
     "phases": [{"id": "slinging", "fromS": 0, "toS": 1.5, "title": T("Hooking the slings", "Schlingen einhängen")}, {"id": "lifting", "fromS": 1.5, "toS": 3.5, "title": T("Lifting", "Anheben")}, {"id": "hanging", "fromS": 3.5, "toS": 4, "title": T("Hanging", "Hängen")}],
     "events": [{"t": 1.5, "kind": "contact-off", "title": T("Bundle leaves the bearers", "Bündel verlässt die Unterleger")}], "tracks": []},
    {"id": "roofer-walk", "title": T("Roofer walking up the rafter", "Dachdecker geht den Sparren hinauf"), "description": T("A 1.0 kN roofer load travels on the top edge from the wall plate seat to the ridge purlin seat over the roof covering load.", "Eine Dachdeckerlast von 1,0 kN wandert auf der Oberkante vom Kervensitz der Fußpfette zum Kervensitz der Firstpfette, zusätzlich zur Last der Dachdeckung."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled-roofer", "ease": "linear", "bones": {"load-carrier": {"dx": 0}}}, {"t": 6, "pose": "reinstalled-roofer", "ease": "linear", "bones": {"load-carrier": {"dx": r6(RIDGE_U - WALL_U)}}}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 6} for a in ROOF_CONTACTS],
     "phases": [{"id": "walk", "fromS": 0, "toS": 6, "title": T("Walking up", "Hinaufgehen")}],
     "events": [{"t": 0, "kind": "load-on", "title": T("Roofer steps on above the wall plate", "Dachdecker tritt über der Fußpfette auf")}], "tracks": []},
]

STRUCTURE = {
    "gravityMS2": 9.81,
    "dynamics": "quasi-static",
    "supports": [],
    "assumptions": [
        T("Softwood rafters of sorting class S10 (DIN 4074-1), assigned to C24 (EN 338: E0,mean 11.0 GPa, f_m,k 24 MPa, mean density 420 kg/m3); section 80 x 160 mm on edge, A = 0.0128 m2, I = 27.3e-6 m4. Masses are section x density of the modelled solids.", "Nadelholzsparren der Sortierklasse S10 (DIN 4074-1), zugeordnet C24 (EN 338: E0,mean 11,0 GPa, f_m,k 24 MPa, mittlere Rohdichte 420 kg/m³); Querschnitt 80 x 160 mm hochkant, A = 0,0128 m², I = 27,3e-6 m⁴. Massen sind Querschnitt x Rohdichte der modellierten Körper."),
        T("Stored: each rafter bears on the two stickers or bearers 2.60 m apart with 0.95 m overhangs; the eleven bundle rafters act as one member of eleven independent sections, the outer upper rafter as its own member on its two frictionless sticker contacts, so its reactions are shown where they act on it and the ground reactions of the bearers show the rest.", "Gelagert: jeder Sparren liegt auf den zwei Stapellatten oder Unterlegern im Abstand 2,60 m mit 0,95 m Kragarmen; die elf Bündelsparren wirken als ein Stab aus elf unabhängigen Querschnitten, der äußere obere Sparren als eigener Stab auf seinen zwei reibungsfreien Lattenkontakten, deren Reaktionen dort gezeigt werden, wo sie auf ihn wirken; die Bodenreaktionen der Unterleger zeigen den Rest."),
        T("Lifted: the two sling legs are pinned in the crane hook, 58.4 degrees to the horizontal (included angle 63 degrees, within the 0 to 90 degree range of EN 1492-2 for a two-leg hitch); the bearers stay on the ground and carry their own 2 x 4.5 kg there.", "Angehoben: die zwei Schlingenstränge sind im Kranhaken gelenkig gelagert, 58,4 Grad zur Waagrechten (Spreizwinkel 63 Grad, im Bereich 0 bis 90 Grad der EN 1492-2 für zwei Stränge); die Unterleger bleiben am Boden und tragen dort ihre eigenen 2 x 4,5 kg."),
        T(f"Reinstalled: a purlin roof of {PITCH:.0f} degrees, rafter spacing {SPACING:.2f} m; the birdsmouth (40 mm deep, h/4) seats on the wall plate as a pin and on the ridge purlin as a roller, horizontal span {SPAN_H:.3f} m, eaves overhang 0.60 m; roof covering {ROOF_DEAD:.2f} kN/m2 of roof area, snow s_k {SNOW_SK:.2f} kN/m2 with mu1 {MU1:.3f} (EN 1991-1-3 Table 5.2), roofer 1.0 kN (EN 1991-1-1 category H).", f"Wiedereingebaut: ein Pfettendach mit {PITCH:.0f} Grad, Sparrenabstand {SPACING:.2f} m; die Kerve (40 mm tief, h/4) sitzt auf der Fußpfette als Festlager und auf der Firstpfette als Rollenlager, waagrechte Stützweite {SPAN_H:.3f} m, Traufüberstand 0,60 m; Dachdeckung {ROOF_DEAD:.2f} kN/m² Dachfläche, Schnee s_k {SNOW_SK:.2f} kN/m² mit mu1 {MU1:.3f} (EN 1991-1-3 Tabelle 5.2), Dachdecker 1,0 kN (EN 1991-1-1 Kategorie H).".replace(".", ",").replace("EN 1991-1-3", "EN 1991-1-3").replace("1,0 kN (EN 1991-1-1", "1,0 kN (EN 1991-1-1")),
        T("The structural solver treats each pose as one rigid body on supports of equal stiffness. Where the asset is two separately supported bodies (the rafter in the roof and the stack in the yard; the hanging bundle and the bearers left on the ground), the statically determinate ground reactions under the bearers are applied as known loads (yard-under-bearer), so that the roof supports and the sling legs carry exactly their own body; in the lift clip that load fades in while the bundle rises.", "Der Tragwerkslöser behandelt jede Pose als einen starren Körper auf Auflagern gleicher Steifigkeit. Wo das Asset zwei getrennt gelagerte Körper ist (der Sparren im Dach und der Stapel auf dem Hof; das hängende Bündel und die am Boden verbliebenen Unterleger), werden die statisch bestimmten Bodenreaktionen unter den Unterlegern als bekannte Lasten angesetzt (yard-under-bearer), damit Dachauflager und Schlingenstränge genau ihren eigenen Körper tragen; im Hebe-Clip wird diese Last eingeblendet, während das Bündel steigt."),
    ],
    "clinical": False,
}

# ----------------------------------------------------------------------------- 2D

ROLE_TONE = {"rafter": 0.6, "behind": 0.46, "bearer": 0.34, "sticker": 0.72, "strap": 0.92, "loop": 0.3, "leg": 0.3, "buckle": 0.5}
PENS = {
    50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
    100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35},
    200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25},
    500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18},
}
LEG_PEN = {500: 0.2}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def nails_side(dv):
    """🔩️ The purlin-anchor nail patterns (2 x 3 holes beside each birdsmouth) and the old rafter-nail hole through each seat, on a side face."""
    d = []
    for s in (SW, SR):
        for k in range(3):
            for row in (-0.045, -0.015):
                d.append(circle_path(s + 0.075 + 0.025 * k, row + dv, NAIL_R))
        d.append(circle_path(s + 0.012, -H / 2 + NOTCH + 0.012 + dv, 0.0032))
    return " ".join(d)


def nails_top(zc):
    """🔩️ The batten-nail holes along the top edge at the 330 mm batten gauge, in plan."""
    return " ".join(circle_path(0.40 + 0.33 * k, zc + (0.012 if k % 2 else -0.012), NAIL_R) for k in range(12))


def ground_shapes(view):
    """➖️ The yard ground line, carried by the bearer that never leaves it."""
    if view == "elevation":
        return [shape("ground", "bearer-a", "ground", 0, False, 0.5, poly([(-X_BEAR[0] - 0.25, -BEARER), (L - X_BEAR[0] + 0.25, -BEARER)], closed=False))]
    if view == "front":
        return [shape("ground", "bearer-a", "ground", 0, False, 0.5, poly([(-0.75, 0), (0.75, 0)], closed=False))]
    return []


def rep_elevation(scale):
    """🪵️ The side elevation of one scale, bone-local."""
    sh = []
    rough = scale >= 200
    if scale == 50:
        sh.append(shape("layer-lower", "bundle", "body", 1, True, ROLE_TONE["rafter"], poly(rafter_profile(dv=H / 2)), region="bundle-rafters", material="reclaimed-softwood", pen=0.22))
        sh.append(shape("layer-upper", "bundle", "body", 1, True, ROLE_TONE["behind"], poly(rafter_profile(dv=V_UP + H / 2)), region="bundle-rafters", material="reclaimed-softwood", pen=0.22))
        sh.append(shape("rafter-upper-near", "bundle", "body", 1.2, True, ROLE_TONE["rafter"] - 0.03, poly(rafter_profile(chip_top=True, dv=V_UP + H / 2)), region="bundle-rafters", material="reclaimed-softwood", pen=0.22))
        sh.append(shape("nails-near", "bundle", "texture", 1.5, False, 0.2, path(nails_side(H / 2) + " " + nails_side(V_UP + H / 2)), region="bundle-rafters"))
    elif scale == 100:
        sh.append(shape("layer-lower", "bundle", "body", 1, True, ROLE_TONE["rafter"], poly(rafter_profile(dv=H / 2)), region="bundle-rafters", material="reclaimed-softwood"))
        sh.append(shape("layer-upper", "bundle", "body", 1, True, ROLE_TONE["rafter"] - 0.04, poly(rafter_profile(dv=V_UP + H / 2)), region="bundle-rafters", material="reclaimed-softwood"))
    else:
        regs = ["bundle-rafters", "straps", "slings"] if scale == 500 else None
        sh.append(shape("block", "bundle", "body", 1, True, ROLE_TONE["rafter"] - (0.1 if scale == 500 else 0.04), poly(block_profile(scale == 200)), region=None if regs else "bundle-rafters", regions=regs, material="reclaimed-softwood"))
    if scale <= 200:
        for k, x in enumerate(X_BEAR):
            sh.append(shape(f"sticker-{'ab'[k]}", "bundle", "body", 3, True, ROLE_TONE["sticker"], rect(x - STK_W / 2, H, STK_W, STK_H), region="storage-timbers", material="yard-softwood"))
        for k, x in enumerate(X_STRAP):
            sh.append(shape(f"strap-{'ab'[k]}", "bundle", "body", 4, True, ROLE_TONE["strap"], rect(x - STRAP_W / 2, -STRAP_T, STRAP_W, TOP + 2 * STRAP_T), region="straps", material="polyester-strap"))
            if scale == 50:
                sh.append(shape(f"buckle-{'ab'[k]}", "bundle", "detail", 3.9, True, ROLE_TONE["buckle"], rect(x - BUCKLE[0] / 2, TOP + STRAP_T, BUCKLE[0], BUCKLE[1]), region="straps", material="galvanised-steel"))
        for k, x in enumerate(X_SLING):
            sh.append(shape(f"loop-{'ab'[k]}", "bundle", "body", 4, True, ROLE_TONE["loop"], rect(x - LOOP_W / 2, -SLING_D, LOOP_W, TOP + 2 * SLING_D), region="slings", material="polyester-sling"))
    prof = rafter_profile(notches=scale <= 200, chip_foot=scale == 50)
    sh.append(shape("rafter", "rafter", "body", 0.5, True, ROLE_TONE["rafter"] - 0.06, poly(prof), region="rafter-reinstalled", material="reclaimed-softwood"))
    if scale == 50:
        sh.append(shape("rafter-nails", "rafter", "texture", 0.6, False, 0.2, path(nails_side(0.0)), region="rafter-reinstalled"))
    for k, x in enumerate(X_BEAR):
        sh.append(shape(f"bearer-{'ab'[k]}", f"bearer-{'ab'[k]}", "body", 2, True, ROLE_TONE["bearer"], rect(-BEARER / 2, -BEARER, BEARER, BEARER, BEAR_R if scale == 50 else None), region="storage-timbers", material="yard-softwood"))
    for k in range(2):
        sh.append(shape(f"leg-{'ab'[k]}", f"sling-{'ab'[k]}", "body", 5, True, ROLE_TONE["leg"], rect(0, -SLING_D / 2, LEGS[k], SLING_D), region="slings", material="polyester-sling", pen=LEG_PEN.get(scale)))
    return sh + ground_shapes("elevation")


def rep_front(scale):
    """🪵️ The front elevation (from ahead) of one scale, world rest coordinates: x = lateral z, y = height."""
    sh = []
    y0 = BEARER
    if scale <= 100:
        sh.append(shape("ends-lower", "bundle", "body", 2, True, ROLE_TONE["rafter"], path(" ".join(rect_path(c - B / 2, y0, c + B / 2, y0 + H) for c in COLS)), region="bundle-rafters", material="reclaimed-softwood"))
        sh.append(shape("ends-upper", "bundle", "body", 2, True, ROLE_TONE["rafter"] - 0.04, path(" ".join(rect_path(c - B / 2, y0 + V_UP, c + B / 2, y0 + TOP) for c in COLS[:5])), region="bundle-rafters", material="reclaimed-softwood"))
    else:
        regs = ["bundle-rafters", "straps", "slings"] if scale == 500 else None
        lshape = [(-HALF, y0), (HALF, y0), (HALF, y0 + V_UP), (COLS[4] + B / 2, y0 + V_UP), (COLS[4] + B / 2, y0 + TOP), (-HALF, y0 + TOP)]
        sh.append(shape("block", "bundle", "body", 2, True, ROLE_TONE["rafter"] - (0.1 if scale == 500 else 0.04), poly(lshape), region=None if regs else "bundle-rafters", regions=regs, material="reclaimed-softwood"))
    if scale <= 200:
        for k in range(2):
            sh.append(shape(f"sticker-{'ab'[k]}", "bundle", "body", 1, True, ROLE_TONE["sticker"], rect(-STK_L / 2, y0 + H, STK_L, STK_H), region="storage-timbers", material="yard-softwood"))
        for k in range(2):
            ring = [(-HALF - STRAP_T / 2, y0 - STRAP_T / 2), (HALF + STRAP_T / 2, y0 - STRAP_T / 2), (HALF + STRAP_T / 2, y0 + TOP + STRAP_T / 2), (-HALF - STRAP_T / 2, y0 + TOP + STRAP_T / 2)]
            sh.append(shape(f"strap-{'ab'[k]}", "bundle", "detail", 3, False, ROLE_TONE["strap"], poly(ring + ring[:1], closed=False), region="straps"))
            if scale == 50:
                sh.append(shape(f"buckle-{'ab'[k]}", "bundle", "detail", 3.5, True, ROLE_TONE["buckle"], rect(BUCKLE_W - BUCKLE[2] / 2, y0 + TOP + STRAP_T, BUCKLE[2], BUCKLE[1]), region="straps", material="galvanised-steel"))
        for k in range(2):
            d = rect_path(-HALF - SLING_D, y0 - SLING_D, HALF + SLING_D, y0 + TOP + SLING_D) + " " + rect_path(-HALF, y0, HALF, y0 + TOP)
            sh.append(shape(f"loop-{'ab'[k]}", "bundle", "body", 2.5, True, ROLE_TONE["loop"], path(d), region="slings", material="polyester-sling"))
    prof = rafter_profile(notches=scale <= 200, chip_foot=scale == 50)
    sh.append(shape("rafter", "rafter", "body", 2, True, ROLE_TONE["rafter"] - 0.06, poly(prof), region="rafter-reinstalled", material="reclaimed-softwood", solid=wsolid(RZ - B / 2, RZ + B / 2)))
    for k in range(2):
        sh.append(shape(f"bearer-{'ab'[k]}", f"bearer-{'ab'[k]}", "body", 1, True, ROLE_TONE["bearer"], rect(-BEARER_L / 2, 0, BEARER_L, BEARER, BEAR_R if scale == 50 else None), region="storage-timbers", material="yard-softwood"))
    for k, zc in enumerate((LEG_Z, LEG_Z)):
        sh.append(shape(f"leg-{'ab'[k]}", f"sling-{'ab'[k]}", "body", 4, True, ROLE_TONE["leg"], rect(0, -SLING_D / 2, LEGS[k], SLING_D), region="slings", material="polyester-sling", pen=LEG_PEN.get(scale), solid=wsolid(zc - SLING_D / 2, zc + SLING_D / 2)))
    return sh + ground_shapes("front")


def rep_plan(scale):
    """🪵️ The plan (from above) of one scale, world rest coordinates: x = ahead, y = lateral z."""
    sh = []
    for k, x in enumerate(X_BEAR):
        sh.append(shape(f"bearer-{'ab'[k]}", f"bearer-{'ab'[k]}", "body", 0.2, True, ROLE_TONE["bearer"], rect(x - BEARER / 2, -BEARER_L / 2, BEARER, BEARER_L, BEAR_R if scale == 50 else None), region="storage-timbers", material="yard-softwood"))
    if scale <= 200:
        for k, x in enumerate(X_BEAR):
            sh.append(shape(f"sticker-{'ab'[k]}", "bundle", "body", 0.5, True, ROLE_TONE["sticker"], rect(x - STK_W / 2, -STK_L / 2, STK_W, STK_L), region="storage-timbers", material="yard-softwood"))
    if scale <= 100:
        sh.append(shape("top-lower-outer", "bundle", "body", 0.8, True, ROLE_TONE["behind"], rect(0, COLS[5] - B / 2, L, B), region="bundle-rafters", material="reclaimed-softwood"))
        sh.append(shape("tops-upper", "bundle", "body", 1, True, ROLE_TONE["rafter"], path(" ".join(rect_path(0, c - B / 2, L, c + B / 2) for c in COLS[:5])), region="bundle-rafters", material="reclaimed-softwood"))
        sh.append(shape("plumb-edges", "bundle", "detail", 1.1, False, 0.3, path(" ".join(f"M {r6(D)} {r6(c - B / 2)} L {r6(D)} {r6(c + B / 2)}" for c in COLS[:5])), region="bundle-rafters"))
        if scale == 50:
            sh.append(shape("nails-top", "bundle", "texture", 1.2, False, 0.2, path(" ".join(nails_top(c) for c in COLS[:5])), region="bundle-rafters"))
    else:
        regs = ["bundle-rafters", "straps", "slings"] if scale == 500 else None
        sh.append(shape("block", "bundle", "body", 1, True, ROLE_TONE["rafter"] - (0.1 if scale == 500 else 0.04), rect(0, -HALF, L, 2 * HALF), region=None if regs else "bundle-rafters", regions=regs, material="reclaimed-softwood"))
        if scale == 200:
            sh.append(shape("plumb-edges", "bundle", "detail", 1.1, False, 0.3, path(f"M {r6(D)} {r6(-HALF)} L {r6(D)} {r6(COLS[4] + B / 2)}"), region="bundle-rafters"))
    if scale <= 200:
        for k, x in enumerate(X_STRAP):
            sh.append(shape(f"strap-{'ab'[k]}", "bundle", "body", 3, True, ROLE_TONE["strap"], rect(x - STRAP_W / 2, -HALF - STRAP_T, STRAP_W, 2 * (HALF + STRAP_T)), region="straps", material="polyester-strap"))
            if scale == 50:
                sh.append(shape(f"buckle-{'ab'[k]}", "bundle", "detail", 3.5, True, ROLE_TONE["buckle"], rect(x - BUCKLE[0] / 2, BUCKLE_W - BUCKLE[2] / 2, BUCKLE[0], BUCKLE[2]), region="straps", material="galvanised-steel"))
        for k, x in enumerate(X_SLING):
            sh.append(shape(f"loop-{'ab'[k]}", "bundle", "body", 3, True, ROLE_TONE["loop"], rect(x - LOOP_W / 2, -HALF - SLING_D, LOOP_W, 2 * (HALF + SLING_D)), region="slings", material="polyester-sling"))
    prof = rafter_profile(notches=scale <= 200, chip_foot=scale == 50)
    sh.append(shape("rafter", "rafter", "body", 1, True, ROLE_TONE["rafter"] - 0.06, poly(prof), region="rafter-reinstalled", material="reclaimed-softwood", solid=wsolid(RZ - B / 2, RZ + B / 2)))
    if scale <= 200 and scale != 200:
        sh.append(shape("rafter-plumb-edge", "rafter", "detail", 1.1, False, 0.3, poly([(D, RZ - B / 2), (D, RZ + B / 2)], closed=False), region="rafter-reinstalled"))
    if scale == 50:
        sh.append(shape("rafter-nails-top", "rafter", "texture", 1.2, False, 0.2, path(nails_top(RZ)), region="rafter-reinstalled"))
    for k, zc in enumerate((LEG_Z, LEG_Z)):
        sh.append(shape(f"leg-{'ab'[k]}", f"sling-{'ab'[k]}", "body", 4, True, ROLE_TONE["leg"], rect(0, -SLING_D / 2, LEGS[k], SLING_D), region="slings", material="polyester-sling", pen=LEG_PEN.get(scale), solid=wsolid(zc - SLING_D / 2, zc + SLING_D / 2)))
    return sh


def rep_decisions(scale):
    """📝️ The authored decisions of one representation."""
    if scale == 50:
        return {
            "silhouette": T("Side elevation of the 4.50 m bundle (90 mm on paper): the two layers of rafters on edge with their plumb-cut ends and the two birdsmouths of every rafter (open through the whole bundle), the 30 mm sticker slot between the layers, the two 100 mm bearers, the white cord straps and the violet sling chokes as bands across the side, the sling legs on top. Front: twelve 80 x 160 mm end faces with 25 mm gaps, the stickers in the slot, the sling chokes as rings. Plan: the upper rafters with their gaps over the stickers and bearers.", "Seitenansicht des 4,50 m langen Bündels (90 mm auf Papier): die zwei Lagen hochkant stehender Sparren mit Lotschnitten an den Enden und den zwei Kerven jedes Sparrens (durch das ganze Bündel offen), der 30-mm-Lattenspalt zwischen den Lagen, die zwei 100-mm-Unterleger, die weißen Kraftbänder und die violetten Schlingenschnürungen als Bänder über die Flanke, die Schlingenstränge obenauf. Vorderansicht: zwölf Stirnflächen 80 x 160 mm mit 25-mm-Fugen, die Latten im Spalt, die Schnürungen als Ringe. Draufsicht: die oberen Sparren mit ihren Fugen über Latten und Unterlegern."),
            "retained": T("Every rafter at its true place, both 40 mm deep birdsmouths (0.8 mm), the plumb cuts (2.7 mm offset), the splintered acute tips of three rafters, the 25 mm gaps (0.5 mm), the 32 mm straps with their wire buckles and the 40 mm chokes.", "Jeder Sparren an seiner wahren Stelle, beide 40 mm tiefen Kerven (0,8 mm), die Lotschnitte (2,7 mm Versatz), die abgesplitterten spitzen Enden dreier Sparren, die 25-mm-Fugen (0,5 mm), die 32-mm-Bänder mit ihren Drahtschnallen und die 40-mm-Schnürungen."),
            "merged": T("The rafters behind the near column of each layer share one outline in the side elevation; the hidden lower layer is drawn in plan only under the outer upper rafter, where it shows once that rafter is reinstalled.", "Die Sparren hinter der vorderen Reihe jeder Lage teilen sich in der Seitenansicht einen Umriss; die verdeckte untere Lage ist in der Draufsicht nur unter dem äußeren oberen Sparren gezeichnet, wo sie nach dessen Wiedereinbau sichtbar wird."),
            "omitted": T("The 4 to 6 mm nail holes are texture, not holes in the outline; the arris wane of a few millimetres is below the 7.5 mm smallest feature.", "Die Nagellöcher von 4 bis 6 mm sind Textur, keine Löcher im Umriss; die Baumkante von wenigen Millimetern liegt unter der kleinsten Einzelheit von 7,5 mm."),
            "thinElements": T("The 1.5 mm cord strap is a filled 32 mm band in side view and plan and a 0.18 mm outline around the bundle in the front view, where only its edge shows.", "Das 1,5-mm-Kraftband ist in Seitenansicht und Draufsicht ein gefülltes 32-mm-Band und in der Vorderansicht ein 0,18-mm-Umriss um das Bündel, wo nur seine Kante zu sehen ist."),
            "hatching": T("No hatch: the faces are views, not sections; tones tell the aged rafters, the grey yard timber, the white straps and the violet slings apart.", "Keine Schraffur: die Flächen sind Ansichten, keine Schnitte; Grautöne unterscheiden gealterte Sparren, graues Lagerholz, weiße Bänder und violette Schlingen."),
            "styles": T("Outline 0.35 mm; the rafter layers of the side elevation 0.22 mm, so that the 30 mm sticker slot stays a printable white line up to 1:75; buckles and plumb edges 0.18 mm, nail holes 0.1 mm texture.", "Kontur 0,35 mm; die Sparrenlagen der Seitenansicht 0,22 mm, damit der 30-mm-Lattenspalt bis 1:75 eine druckbare weiße Linie bleibt; Schnallen und Lotkanten 0,18 mm, Nagellöcher 0,1 mm Textur."),
            "overlays": T("Each rafter group, the storage timbers, the straps and the slings are their own region cells.", "Jede Sparrengruppe, das Lagerholz, die Bänder und die Schlingen sind eigene Regionenzellen."),
        }
    if scale == 100:
        return {
            "silhouette": T("The bundle 45 x 4.8 mm on paper: two layers of plumb-cut rafters with the birdsmouths, sticker slot, bearers, straps, chokes and the sling legs; front with the twelve end faces and 25 mm gaps (0.25 mm); plan with the five upper rafters and the outer lower one.", "Das Bündel 45 x 4,8 mm auf Papier: zwei Lagen lotgeschnittener Sparren mit den Kerven, Lattenspalt, Unterleger, Bänder, Schnürungen und die Schlingenstränge; Vorderansicht mit den zwölf Stirnflächen und 25-mm-Fugen (0,25 mm); Draufsicht mit den fünf oberen Sparren und dem äußeren unteren."),
            "retained": T("Birdsmouths (0.4 mm deep), plumb cuts, gaps and slot, straps and chokes as bands, the reinstalled rafter as its own part.", "Kerven (0,4 mm tief), Lotschnitte, Fugen und Spalt, Bänder und Schnürungen als Bänder, der wiedereingebaute Sparren als eigenes Teil."),
            "merged": T("Each layer is one outline in side view; the splintered tips (30 mm) are merged into the straight cut.", "Jede Lage ist in der Seitenansicht ein Umriss; die abgesplitterten Spitzen (30 mm) gehen im geraden Schnitt auf."),
            "omitted": T("Nail holes, wire buckles and the rounded arrises of the bearers.", "Nagellöcher, Drahtschnallen und die gerundeten Kanten der Unterleger."),
            "thinElements": T("Straps as 32 mm bands (0.32 mm) in side view and plan, as an outline in the front view; sling legs as 30 mm bars.", "Bänder als 32-mm-Bänder (0,32 mm) in Seitenansicht und Draufsicht, als Umriss in der Vorderansicht; Schlingenstränge als 30-mm-Stäbe."),
            "hatching": T("No hatch.", "Keine Schraffur."),
            "styles": T("Outline 0.25 mm, plumb edges 0.13 mm.", "Kontur 0,25 mm, Lotkanten 0,13 mm."),
            "overlays": T("Region cells as at 1:50.", "Regionenzellen wie bei 1:50."),
        }
    if scale == 200:
        return {
            "silhouette": T("The bundle as one stack 22.5 x 1.75 mm on paper with the sawtooth of the plumb-cut ends and the lower birdsmouths, the sticker ends, bearers, straps, chokes and legs; the reinstalled rafter stays separate in every view.", "Das Bündel als ein Stapel 22,5 x 1,75 mm auf Papier mit dem Sägezahn der lotgeschnittenen Enden und den unteren Kerven, Lattenenden, Unterleger, Bänder, Schnürungen und Stränge; der wiedereingebaute Sparren bleibt in jeder Ansicht eigenständig."),
            "retained": T("The lower birdsmouths (40 mm, 0.2 mm), the 0.67 mm plumb offset, the protruding sticker ends, straps and chokes.", "Die unteren Kerven (40 mm, 0,2 mm), der 0,67-mm-Lotversatz, die überstehenden Lattenenden, Bänder und Schnürungen."),
            "merged": T("The 25 mm gaps (0.13 mm) and the 30 mm slot (0.15 mm) close; the eleven bundle rafters become one block with the outer upper corner left open for the reinstalled rafter. The upper birdsmouths open into the closed slot and merge.", "Die 25-mm-Fugen (0,13 mm) und der 30-mm-Spalt (0,15 mm) schließen sich; die elf Bündelsparren werden ein Block, dessen äußere obere Ecke für den wiedereingebauten Sparren frei bleibt. Die oberen Kerven öffnen sich in den geschlossenen Spalt und gehen darin auf."),
            "omitted": T("Nail holes, buckles, splintered tips, the individual end faces.", "Nagellöcher, Schnallen, abgesplitterte Spitzen, die einzelnen Stirnflächen."),
            "thinElements": T("Straps 32 mm (0.16 mm) as filled bands that the 0.18 mm pen makes legible; sling legs as 30 mm bars.", "Bänder 32 mm (0,16 mm) als gefüllte Bänder, die die 0,18-mm-Feder lesbar macht; Schlingenstränge als 30-mm-Stäbe."),
            "hatching": T("No hatch.", "Keine Schraffur."),
            "styles": T("Outline 0.18 mm, the foot plumb edge 0.1 mm in plan.", "Kontur 0,18 mm, die Fußlotkante 0,1 mm in der Draufsicht."),
            "overlays": T("The block is the bundle-rafter cell; bearers with stickers, straps, slings and the rafter keep their cells.", "Der Block ist die Zelle der Bündelsparren; Unterleger mit Latten, Bänder, Schlingen und der Sparren behalten ihre Zellen."),
        }
    return {
        "silhouette": T("The stack as one block 9 x 0.7 mm on paper with its plumb-cut ends, the reinstalled rafter beside it, the two bearers as 0.2 mm squares and the sling legs as 0.2 mm bars; front view the L-shaped stack beside the rafter end, plan the 4.50 x 0.605 m stack over the 1.0 m bearers.", "Der Stapel als ein Block 9 x 0,7 mm auf Papier mit seinen lotgeschnittenen Enden, daneben der wiedereingebaute Sparren, die zwei Unterleger als 0,2-mm-Quadrate und die Schlingenstränge als 0,2-mm-Stäbe; Vorderansicht der L-förmige Stapel neben dem Sparrenende, Draufsicht der 4,50 x 0,605 m große Stapel über den 1,0 m langen Unterlegern."),
        "retained": T("Overall length, height and width, the plumb offsets (0.27 mm), the bearers and the rafter that is reinstalled.", "Gesamtlänge, -höhe und -breite, die Lotversätze (0,27 mm), die Unterleger und der wiederverwendete Sparren."),
        "merged": T("Rafters, stickers, straps and chokes are one block (regions merged); the birdsmouths (40 mm, 0.08 mm) close.", "Sparren, Latten, Bänder und Schnürungen sind ein Block (Regionen zusammengefasst); die Kerven (40 mm, 0,08 mm) schließen sich."),
        "omitted": T("Gaps, slot, birdsmouths, buckles and the protruding sticker ends (30 mm).", "Fugen, Spalt, Kerven, Schnallen und die überstehenden Lattenenden (30 mm)."),
        "thinElements": T("The 30 mm sling legs (0.06 mm) are filled bars drawn with a 0.2 mm pen, so they print and read as lines in every view and in the lifted pose.", "Die 30-mm-Schlingenstränge (0,06 mm) sind gefüllte Stäbe mit einer 0,2-mm-Feder, damit sie drucken und sich in jeder Ansicht und in der angehobenen Pose als Linien lesen."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Outline 0.13 mm; the block one tone darker than at 1:200 so that it holds at 1:1000.", "Kontur 0,13 mm; der Block einen Ton dunkler als bei 1:200, damit er bei 1:1000 trägt."),
        "overlays": T("The block carries the area-weighted value of the bundle rafters, straps and slings (labelled merged).", "Der Block trägt den flächengewichteten Wert von Bündelsparren, Bändern und Schlingen (als zusammengefasst gekennzeichnet)."),
    }


def representation(scale):
    """🗺️ One authored representation."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": {"elevation": rep_elevation(scale), "front": rep_front(scale), "plan": rep_plan(scale)}, "decisions": rep_decisions(scale)}


# ----------------------------------------------------------------------------- 3D

def bundle_rafters_50():
    """🪵️ The eleven bundle rafters of 1:50: intact runs by repeat, three with a splintered acute tip."""
    lo = [c for c in COLS]
    out = [
        solid("rafters-lower-a", "bundle", "reclaimed-softwood", 0.6, xy_extrude(rafter_profile(dv=H / 2), lo[0] - B / 2, lo[0] + B / 2), repeat=repeat(3, PITCH_Z)),
        solid("rafter-lower-chipped", "bundle", "reclaimed-softwood", 0.55, xy_extrude(rafter_profile(chip_foot=True, dv=H / 2), lo[3] - B / 2, lo[3] + B / 2)),
        solid("rafters-lower-b", "bundle", "reclaimed-softwood", 0.63, xy_extrude(rafter_profile(dv=H / 2), lo[4] - B / 2, lo[4] + B / 2), repeat=repeat(2, PITCH_Z)),
        solid("rafter-upper-chipped", "bundle", "reclaimed-softwood", 0.57, xy_extrude(rafter_profile(chip_top=True, dv=V_UP + H / 2), lo[0] - B / 2, lo[0] + B / 2)),
        solid("rafters-upper", "bundle", "reclaimed-softwood", 0.61, xy_extrude(rafter_profile(dv=V_UP + H / 2), lo[1] - B / 2, lo[1] + B / 2), repeat=repeat(4, PITCH_Z)),
    ]
    return out


def bundle_rafters_100():
    """🪵️ The eleven bundle rafters of 1:100: two repeated runs, tips straight."""
    return [
        solid("rafters-lower", "bundle", "reclaimed-softwood", 0.6, xy_extrude(rafter_profile(dv=H / 2), COLS[0] - B / 2, COLS[0] + B / 2), repeat=repeat(6, PITCH_Z)),
        solid("rafters-upper", "bundle", "reclaimed-softwood", 0.57, xy_extrude(rafter_profile(dv=V_UP + H / 2), COLS[0] - B / 2, COLS[0] + B / 2), repeat=repeat(5, PITCH_Z)),
    ]


def storage(scale):
    """🪵️ Bearers (and stickers where the scale keeps them)."""
    out = [solid(f"bearer-{'ab'[k]}", f"bearer-{'ab'[k]}", "yard-softwood", 0.34, box((BEARER, BEARER, BEARER_L), (0, -BEARER / 2, 0), BEAR_R if scale == 50 else None)) for k in range(2)]
    if scale <= 200:
        out += [solid(f"sticker-{'ab'[k]}", "bundle", "yard-softwood", 0.72, box((STK_W, STK_H, STK_L), (x, H + STK_H / 2, 0))) for k, x in enumerate(X_BEAR)]
    return out


def textiles(scale):
    """🎗️ Straps, buckles, chokes and sling legs of a scale."""
    out = []
    if scale <= 200:
        out += [solid(f"strap-{'ab'[k]}", "bundle", "polyester-strap", 0.92, yz_ring(STRAP_OUT, STRAP_HOLE, x - STRAP_W / 2, x + STRAP_W / 2)) for k, x in enumerate(X_STRAP)]
        if scale == 50:
            out += [solid(f"buckle-{'ab'[k]}", "bundle", "galvanised-steel", 0.5, box(BUCKLE, (x, TOP + STRAP_T + BUCKLE[1] / 2, BUCKLE_W))) for k, x in enumerate(X_STRAP)]
        out += [solid(f"loop-{'ab'[k]}", "bundle", "polyester-sling", 0.3, yz_ring(LOOP_OUT, LOOP_HOLE, x - LOOP_W / 2, x + LOOP_W / 2)) for k, x in enumerate(X_SLING)]
    out += [solid(f"leg-{'ab'[k]}", f"sling-{'ab'[k]}", "polyester-sling", 0.3, rod(LEGS[k], SLING_D / 2), slender=True) for k in range(2)]
    return out


def rafter_body(scale):
    """🪵️ The reinstalled rafter of a scale."""
    return [solid("rafter", "rafter", "reclaimed-softwood", 0.56, xy_extrude(rafter_profile(notches=scale <= 200, chip_foot=scale == 50), -B / 2, B / 2))]


def body(scale):
    """🧊️ The body list of a scale."""
    if scale == 50:
        bundle = bundle_rafters_50()
    elif scale == 100:
        bundle = bundle_rafters_100()
    else:
        bundle = [
            solid("block-lower", "bundle", "reclaimed-softwood", 0.56 if scale == 200 else 0.5, xy_extrude(lower_slot_profile(scale == 200), -HALF, HALF)),
            solid("block-upper", "bundle", "reclaimed-softwood", 0.56 if scale == 200 else 0.5, xy_extrude(upper_profile(), -HALF, COLS[4] + B / 2)),
        ]
    return bundle + rafter_body(scale) + storage(scale) + textiles(scale)


def ref(sid, of, **label):
    """🔗️ A reference to a body solid."""
    out = {"id": sid, "of": of}
    out.update(label)
    return out


def thermal_list(scale, bodies):
    """🌡️ The thermal list of a scale."""
    region_of = {"rafter": "rafter-reinstalled", "bearer-a": "storage-timbers", "bearer-b": "storage-timbers", "sticker-a": "storage-timbers", "sticker-b": "storage-timbers", "strap-a": "straps", "strap-b": "straps", "buckle-a": "straps", "buckle-b": "straps", "loop-a": "slings", "loop-b": "slings", "leg-a": "slings", "leg-b": "slings"}
    if scale <= 100:
        return [ref(f"th-{s['id']}", s["id"], region=region_of.get(s["id"], "bundle-rafters")) for s in bodies]
    out = []
    for layer, dv in (("lower", H / 2), ("upper", V_UP + H / 2)):
        n = 6 if layer == "lower" else 5
        out.append({**solid(f"th-rafters-{layer}", "bundle", "reclaimed-softwood", 0.6, xy_extrude(rafter_profile(notches=scale == 200, dv=dv), COLS[0] - B / 2, COLS[0] + B / 2), repeat=repeat(n, PITCH_Z)), "region": "bundle-rafters"})
    out.append(ref("th-rafter", "rafter", region="rafter-reinstalled"))
    out += [ref(f"th-bearer-{c}", f"bearer-{c}", region="storage-timbers") for c in "ab"]
    if scale == 200:
        out += [ref(f"th-sticker-{c}", f"sticker-{c}", region="storage-timbers") for c in "ab"]
        out += [ref(f"th-strap-{c}", f"strap-{c}", region="straps") for c in "ab"]
    else:
        band_out, band_hole = ring_profiles(0.0, TOP, HALF, STRAP_T)
        out += [{**solid(f"th-strap-{'ab'[k]}", "bundle", "polyester-strap", 0.92, yz_ring(band_out, band_hole, x - 0.0375, x + 0.0375)), "region": "straps"} for k, x in enumerate(X_STRAP)]
    out += [ref(f"th-leg-{c}", f"leg-{c}", region="slings") for c in "ab"]
    return out


def structure_list(scale, bodies):
    """🏗️ The structure list of a scale."""
    ids = [s["id"] for s in bodies]
    out = []
    if scale <= 100:
        for sid in ids:
            if sid.startswith("rafters-") or sid.startswith("rafter-lower") or sid.startswith("rafter-upper"):
                out.append(ref(f"st-{sid}", sid, member="bundle", part="member"))
        out += [ref(f"st-sticker-{c}", f"sticker-{c}", member="bundle", part="member") for c in "ab"]
    elif scale == 200:
        for layer, dv in (("lower", H / 2), ("upper", V_UP + H / 2)):
            n = 6 if layer == "lower" else 5
            out.append({**solid(f"st-rafters-{layer}", "bundle", "reclaimed-softwood", 0.6, xy_extrude(rafter_profile(notches=False, dv=dv), COLS[0] - B / 2, COLS[0] + B / 2), repeat=repeat(n, PITCH_Z)), "member": "bundle", "part": "member"})
        out += [ref(f"st-sticker-{c}", f"sticker-{c}", member="bundle", part="member") for c in "ab"]
    else:
        out += [ref("st-block-lower", "block-lower", member="bundle", part="member"), ref("st-block-upper", "block-upper", member="bundle", part="member")]
    out.append(ref("st-rafter", "rafter", member="rafter", part="member"))
    out += [ref(f"st-leg-{c}", f"leg-{c}", member=f"sling-{c}", part="member") for c in "ab"]
    return out


VOL_DECISIONS = {
    50: [
        T("Body: every rafter is its true side profile extruded 80 mm across: plumb cuts at foot and ridge, the wall-plate birdsmouth at 0.82 m and the ridge-purlin birdsmouth at 4.29 m, both 40 mm deep with horizontal seat and plumb face; three rafters have their 50-degree acute tip splintered off over 30 mm. Runs of identical rafters are repeats at the 105 mm pitch of the stack (80 mm rafter, 25 mm gap).", "Körper: jeder Sparren ist sein wahres Seitenprofil, 80 mm quer extrudiert: Lotschnitte an Fuß und First, die Fußpfettenkerve bei 0,82 m und die Firstpfettenkerve bei 4,29 m, beide 40 mm tief mit waagrechtem Sitz und lotrechter Stirn; bei drei Sparren ist die spitze 50-Grad-Ecke auf 30 mm abgesplittert. Läufe gleicher Sparren sind Wiederholungen im Raster 105 mm des Stapels (80 mm Sparren, 25 mm Fuge)."),
        T("Storage aids: 100 x 100 x 1000 mm bearers with 8 mm rounded arrises on their own bones, 50 x 30 x 800 mm stickers on the bundle, two 32 x 1.5 mm cord straps hugging the stack with galvanised wire buckles on top, two violet round slings choked as 40 x 30 mm bands, their 30 mm legs lying on the upper layer side by side.", "Lagerhilfen: Unterleger 100 x 100 x 1000 mm mit 8-mm-gerundeten Kanten auf eigenen Knochen, Stapellatten 50 x 30 x 800 mm am Bündel, zwei Kraftbänder 32 x 1,5 mm eng um den Stapel mit verzinkten Drahtschnallen oben, zwei violette Rundschlingen als 40 x 30 mm Schnürbänder, ihre 30-mm-Stränge nebeneinander auf der oberen Lage."),
        T("Thermal: the regions are the body surfaces themselves (references): the reinstalled rafter, the eleven bundle rafters, bearers with stickers, straps with buckles, slings; the open gaps let the air reach every face.", "Thermik: die Regionen sind die Körperoberflächen selbst (Verweise): der wiedereingebaute Sparren, die elf Bündelsparren, Unterleger mit Latten, Bänder mit Schnallen, Schlingen; die offenen Fugen lassen die Luft an jede Fläche."),
        T("Structure: the bundle member is the eleven rafters with the two stickers that carry the upper layer onto the lower one over the bearers; the reinstalled rafter and the two sling legs are their own members. The bearers are compression blocks on the ground (no member), the supports are the pose contacts.", "Tragwerk: der Bündelstab sind die elf Sparren mit den zwei Latten, die die obere Lage über den Unterlegern auf die untere übertragen; der wiedereingebaute Sparren und die zwei Schlingenstränge sind eigene Stäbe. Die Unterleger sind Druckklötze auf dem Boden (kein Stab), die Auflager sind die Kontakte der Posen."),
    ],
    100: [
        T("Body: the same twelve rafters with plumb cuts and both 40 mm birdsmouths, now two repeated runs of straight-cut rafters: the 30 mm splinters are below what this scale keeps apart from the cut. Bearers are sharp-edged boxes, the wire buckles (8 mm) are left out; straps, stickers, chokes and legs keep their true sizes, all at least 15 mm.", "Körper: dieselben zwölf Sparren mit Lotschnitten und beiden 40-mm-Kerven, jetzt zwei wiederholte Läufe gerade geschnittener Sparren: die 30-mm-Absplitterungen trennt dieser Maßstab nicht mehr vom Schnitt. Unterleger sind scharfkantige Quader, die Drahtschnallen (8 mm) entfallen; Bänder, Latten, Schnürungen und Stränge behalten ihre wahren Maße, alle mindestens 15 mm."),
        T("Thermal: references to the body surfaces by region, as at 1:50.", "Thermik: Verweise auf die Körperoberflächen je Region, wie bei 1:50."),
        T("Structure: the eleven rafters and two stickers as the bundle member, the reinstalled rafter and the two legs.", "Tragwerk: die elf Sparren und zwei Latten als Bündelstab, der wiedereingebaute Sparren und die zwei Stränge."),
    ],
    200: [
        T("Body: the 25 mm gaps and the 30 mm slot are at or below the 30 mm feature, so the eleven bundle rafters are one stack: the lower layer with the slot as one prism keeping the 40 mm birdsmouths and the sawtooth plumb ends, the upper five as one parallelogram prism that leaves the outer upper place to the reinstalled rafter (its own prism with both birdsmouths). The stickers stay because they stand 97.5 mm out of both flanks; straps, chokes and legs are at least 30 mm.", "Körper: die 25-mm-Fugen und der 30-mm-Spalt liegen bei oder unter der Einzelheit von 30 mm, daher sind die elf Bündelsparren ein Stapel: die untere Lage mit dem Spalt als ein Prisma mit den 40-mm-Kerven und den gesägezahnten Lotenden, die oberen fünf als ein Parallelogrammprisma, das den äußeren oberen Platz dem wiedereingebauten Sparren lässt (eigenes Prisma mit beiden Kerven). Die Latten bleiben, weil sie 97,5 mm aus beiden Flanken ragen; Bänder, Schnürungen und Stränge sind mindestens 30 mm."),
        T("Thermal: the exchange surface of a stickered stack is the surface of its rafters, so the thermal model keeps the twelve rafters apart with their open gaps (minimumAirPassageM 25 mm), with the birdsmouths; bearers, stickers, straps and legs are references; the chokes (2 % of the area) merge into the legs.", "Thermik: die Austauschfläche eines gelatteten Stapels ist die Oberfläche seiner Sparren, daher hält das Thermikmodell die zwölf Sparren mit ihren offenen Fugen getrennt (minimumAirPassageM 25 mm), mit den Kerven; Unterleger, Latten, Bänder und Stränge sind Verweise; die Schnürungen (2 % der Fläche) gehen in den Strängen auf."),
        T("Structure: the bundle member keeps the eleven true 80 x 160 mm sections as prisms with plumb ends (the birdsmouths are local reductions below this idealisation) and the two stickers that join the layers; rafter and legs are references.", "Tragwerk: der Bündelstab behält die elf wahren Querschnitte 80 x 160 mm als Prismen mit Lotenden (die Kerven sind örtliche Schwächungen unter dieser Idealisierung) und die zwei Latten, die die Lagen verbinden; Sparren und Stränge sind Verweise."),
    ],
    500: [
        T("Body: one stack of the two layers with the slot closed, plumb-cut sawtooth ends kept (134 mm offset), the 40 mm birdsmouths closed; the reinstalled rafter as its own 80 x 160 mm prism, the 100 mm bearers, the 30 mm legs as slender rods, drawn as 0.2 mm bars. Stickers (30 mm), straps (32 mm), buckles and chokes (40 mm) are below the 75 mm feature and are part of the stack.", "Körper: ein Stapel aus beiden Lagen mit geschlossenem Spalt, die Sägezahnenden der Lotschnitte bleiben (134 mm Versatz), die 40-mm-Kerven schließen sich; der wiedereingebaute Sparren als eigenes Prisma 80 x 160 mm, die 100-mm-Unterleger, die 30-mm-Stränge als schlanke Stäbe, gezeichnet als 0,2-mm-Stäbe. Latten (30 mm), Bänder (32 mm), Schnallen und Schnürungen (40 mm) liegen unter der Einzelheit von 75 mm und gehören zum Stapel."),
        T("Thermal: the twelve rafters stay apart as 80 x 160 mm prisms without birdsmouths so that the stack keeps its exchange surface through the 25 mm gaps (minimumAirPassageM); the straps are 75 mm bands at the size floor of this scale, bearers and legs are references, stickers and chokes are left out (2 % of the area).", "Thermik: die zwölf Sparren bleiben als Prismen 80 x 160 mm ohne Kerven getrennt, damit der Stapel durch die 25-mm-Fugen seine Austauschfläche behält (minimumAirPassageM); die Bänder sind 75-mm-Bänder an der Größenuntergrenze dieses Maßstabs, Unterleger und Stränge sind Verweise, Latten und Schnürungen entfallen (2 % der Fläche)."),
        T("Structure: the 25 mm gaps and the 30 mm slot are below the 75 mm feature, so the eleven rafters are one member band per layer, the body prisms themselves (the lower layer with the slot, the upper five beside the reinstalled rafter); the member keeps the true section of eleven rafters in its section data. Rafter and legs are references.", "Tragwerk: die 25-mm-Fugen und der 30-mm-Spalt liegen unter der Einzelheit von 75 mm, daher sind die elf Sparren je Lage ein Stabband, die Körperprismen selbst (die untere Lage mit dem Spalt, die oberen fünf neben dem wiedereingebauten Sparren); der Stab behält in seinen Querschnittswerten den wahren Querschnitt von elf Sparren. Sparren und Stränge sind Verweise."),
    ],
}


def volume(scale):
    """🔧️ One authored volume."""
    b = body(scale)
    return {"status": "authored", "denominator": scale, "body": b, "thermal": thermal_list(scale, b), "structure": structure_list(scale, b), "decisions": VOL_DECISIONS[scale]}


# ----------------------------------------------------------------------------- document

REFERENCES = [
    {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes, Table 1: C24 f_m,k 24 N/mm2, E0,mean 11.0 kN/mm2, rho_k 350 kg/m3, rho_mean 420 kg/m3."},
    {"id": "din-4074-1", "citation": "DIN 4074-1:2012-06 Sortierung von Holz nach der Tragfähigkeit, Teil 1: Nadelschnittholz (visual grade S10, assigned to C24 by EN 1912)."},
    {"id": "en-1995-1-1", "citation": "EN 1995-1-1:2004+A2:2014 Eurocode 5, design of timber structures, 6.5 (notched members at supports), used for the birdsmouth depth against the h/4 rule of carpentry practice."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1, Part 1-1, Table 6.9 and 6.10: roofs of category H (not accessible except for normal maintenance), concentrated load Q_k 1.0 kN as set by the German National Annex."},
    {"id": "din-en-1991-1-1-na", "citation": "DIN EN 1991-1-1/NA:2010-12, Annex NA.A (weights of building materials and area loads of roof coverings): interlocking clay tiles with battens about 0.55 kN/m2 of roof area, used as the authored dead load."},
    {"id": "en-1991-1-3", "citation": "EN 1991-1-3:2003 Eurocode 1, Part 1-3, snow loads, Table 5.2: mu1 = 0.8 (60 - alpha)/30 for 30 <= alpha <= 60 degrees; DIN EN 1991-1-3/NA:2010-12, zone 2 minimum s_k = 0.85 kN/m2."},
    {"id": "en-1492-2", "citation": "EN 1492-2:2000+A1:2008 Textile slings, safety, part 2: round slings made of man-made fibres; colour code violet = working load limit 1000 kg in straight pull, mode factors for a two-leg choke."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties, Table 3 (softwood 500 kg/m3: lambda 0.13 W/mK, c 1600 J/kgK; steel 50 W/mK, 450 J/kgK)."},
    {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); about half of it is taken as the effective storage depth of an exposed face."},
    {"id": "ashrae-fundamentals", "citation": "ASHRAE Handbook Fundamentals (2021), thermal radiation properties of building materials (typical emissivity and solar absorptance of weathered wood and of white and coloured polymer textiles; rounded values guided by the chapter, not table look-ups)."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (scales, line groups)."},
]

PROVENANCE = {
    "author": "Semio staffage component authors",
    "method": "handcrafted",
    "created": "2026-11-22",
    "references": REFERENCES,
    "notes": T(
        f"Twelve rafters 80 x 160 mm, 4.50 m over the plumb-cut ends (plumb offset {D * 1000:.0f} mm at {PITCH:.0f} degrees), birdsmouths 40 mm deep at 0.82 m and {SR:.2f} m; stack of two layers of six on edge, 25 mm gaps, 50 x 30 mm stickers over the 100 x 100 mm bearers 2.60 m apart; reinstalled horizontal span {SPAN_H:.3f} m. Geometry is authored-estimate after typical German purlin roofs; nothing is measured.",
        f"Zwölf Sparren 80 x 160 mm, 4,50 m über die Lotschnittenden (Lotversatz {D * 1000:.0f} mm bei {PITCH:.0f} Grad), Kerven 40 mm tief bei 0,82 m und {SR:.2f} m".replace(".", ",") + "; Stapel aus zwei Lagen zu sechs hochkant, 25-mm-Fugen, Stapellatten 50 x 30 mm über den Unterlegern 100 x 100 mm im Abstand 2,60 m; " + f"waagrechte Stützweite im Wiedereinbau {SPAN_H:.3f} m".replace(".", ",") + ". Die Geometrie ist eine Schätzung nach typischen deutschen Pfettendächern; nichts ist gemessen.",
    ),
}

ASSET = {
    "id": "reclaimed-rafter-bundle",
    "category": "components",
    "title": T("Reclaimed Rafter Bundle", "Gebrauchtes Sparrenbündel"),
    "summary": T(
        "Twelve reclaimed softwood rafters 80 x 160 mm, 4.50 m long with plumb-cut ends, two birdsmouths each and the nail holes of battens and purlin anchors, stacked on edge in two stickered layers on two squared-timber bearers and strapped twice; lifted on two choked round slings, and one rafter reinstalled in a 40 degree purlin roof on wall plate and ridge purlin under roof covering, snow and a roofer.",
        "Zwölf gebrauchte Nadelholzsparren 80 x 160 mm, 4,50 m lang mit Lotschnittenden, je zwei Kerven und den Nagellöchern von Lattung und Sparrenpfettenankern, hochkant in zwei gelatteten Lagen auf zwei Kantholz-Unterlegern gestapelt und zweimal umreift; an zwei geschnürten Rundschlingen angehoben, und ein Sparren in einem 40-Grad-Pfettendach auf Fuß- und Firstpfette wiedereingebaut unter Dachdeckung, Schnee und einem Dachdecker.",
    ),
    "tags": ["component", "reclaimed", "structural", "timber", "softwood", "rafter", "roof", "bundle", "stored"],
    "provenance": PROVENANCE,
    "dimensions": {"heightM": round(HEIGHT, 3), "widthM": L, "depthM": DEPTH, "massKg": round(M_TOTAL, 2)},
    "materials": MATERIALS,
    "rig": {"kind": "structural", "root": "yard", "bones": BONES},
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

ORDER = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "volumes", "presentation", "exports", "completion"]


def main():
    """🖨️ Writes the document and prints the measurement table."""
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": {k: ASSET[k] for k in ORDER}}
    OUT.write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    rows = [
        ("plumb offset D (m)", D), ("ridge birdsmouth apex SR (m)", SR), ("notch area per rafter (m2)", NOTCH_AREA),
        ("rafter gross volume (m3)", RAFTER_GROSS_V), ("rafter net volume (m3)", RAFTER_VOL), ("rafter mass (kg)", M_RAFTER),
        ("reinstalled rafter mass (kg)", M_RAFTER_REIN), ("bundle bone mass (kg)", M_BUNDLE), ("bearer mass (kg)", M_BEARER),
        ("sticker mass (kg)", M_STICKER), ("strap+buckle mass (kg)", M_STRAP), ("choke mass (kg)", M_LOOP), ("leg mass (kg)", M_LEG),
        ("total mass (kg)", M_TOTAL), ("bundle COM v (m)", COM_V_BUNDLE), ("sling angles A/B (deg)", (THETA_A, THETA_B)), ("leg B length", LEG_B),
        ("wall-plate apex world", APEX_W), ("ridge apex world", APEX_R), ("horizontal span (m)", SPAN_H),
        ("dead load (N/m)", G_DEAD), ("snow along slope (N/m)", S_SLOPE), ("mu1", MU1),
        ("area rafter-reinstalled", AREA_RAFTER_CHIP_FOOT), ("area bundle-rafters", AREA_BUNDLE), ("area storage-timbers", AREA_STORAGE),
        ("area straps", AREA_STRAPS), ("area slings", AREA_SLINGS), ("height", HEIGHT),
    ]
    for name, value in rows:
        print(f"{name:34s} {value}")
    print(f"written {OUT}")


if __name__ == "__main__":
    main()
