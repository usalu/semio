"""🥁️ Authoritative generator of the staffage asset `used-djembe-and-cajon`: every hand-chosen number of the used rope-tuned djembe and the used cajón lives here and the asset document `🔣️.json` next to this file is written from it.

Run: `python3 author-djembe-cajon.py` (writes `🔣️.json`).

@see ../../brief.md
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
TAU = 2 * math.pi


def T(en, de):
    """🌐️ A bilingual text."""
    return {"en": en, "de": de}


def r6(v):
    """✂️ Rounds a number to 0.1 mm precision for the document."""
    return round(v, 6)


def pts(seq):
    """📍️ Rounds a point list."""
    return [[r6(a), r6(b)] for a, b in seq]


def arc(cx, cy, r, a0, a1, n):
    """🌙️ Points on a circular arc from angle a0 to a1 (degrees), both ends included."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def circle(cx, cy, r, n):
    """⭕️ A closed polygon of n points (first point not repeated)."""
    return [(cx + r * math.cos(TAU * i / n), cy + r * math.sin(TAU * i / n)) for i in range(n)]


def interp(profile, h):
    """📈️ Radius of a (h, r) profile ordered by h at height h."""
    for (h0, a), (h1, b) in zip(profile, profile[1:]):
        if h0 <= h <= h1:
            return a + (b - a) * (h - h0) / (h1 - h0) if h1 > h0 else max(a, b)
    return profile[-1][1] if h > profile[-1][0] else profile[0][1]


# ---------------------------------------------------------------- reference numbers
CAJON_W = 0.300
CAJON_D = 0.300
CAJON_H = 0.480
PLY = 0.012
BACK = 0.009
TAPA = 0.003
PORT_R = 0.055
PORT_U = 0.320
TOP_ROUND = 0.008
X_TAPA_IN = CAJON_D / 2 - TAPA
RHO_PLY = 680.0
RHO_BEECH = 720.0
RHO_LENKE = 800.0
RHO_SKIN = 1100.0
RHO_ROPE = 870.0
RHO_RING = 2615.0
RHO_STEEL = 7850.0

SHELL_OUTER = [
    (0.000, 0.1130), (0.008, 0.1145), (0.030, 0.1125), (0.070, 0.1045), (0.120, 0.0935), (0.180, 0.0835), (0.240, 0.0755), (0.290, 0.0705),
    (0.322, 0.0700), (0.350, 0.0760), (0.375, 0.0900), (0.405, 0.1100), (0.445, 0.1310), (0.485, 0.1460), (0.525, 0.1550), (0.560, 0.1585),
    (0.582, 0.1580), (0.592, 0.1545), (0.597, 0.1505), (0.598, 0.1470),
]
SHELL_INNER = [
    (0.598, 0.1430), (0.590, 0.1425), (0.560, 0.1420), (0.525, 0.1360), (0.485, 0.1260), (0.445, 0.1090), (0.405, 0.0870), (0.375, 0.0660),
    (0.350, 0.0520), (0.322, 0.0470), (0.290, 0.0470), (0.240, 0.0520), (0.180, 0.0600), (0.120, 0.0700), (0.070, 0.0810), (0.030, 0.0900), (0.000, 0.0940),
]
H_BEAR = 0.598
SKIN = 0.0012
HEAD_R = 0.141
CROWN_H = 0.556
CROWN_A = 0.006
COLLAR_LOW = 0.540
BOTTOM_H = 0.380
BOTTOM_R = 0.1025
BOTTOM_A = 0.0075
ROPE_R = 0.0025
N_VERT = 24
RF = 0.113
GAP = 0.120


def shell_r(h):
    """🪵️ Outer radius of the carved shell at height h."""
    return interp(SHELL_OUTER, h)


SKIN_OUT = interp(SHELL_OUTER, CROWN_H) + SKIN
CROWN_R = SKIN_OUT + CROWN_A
R_MAX = CROWN_R + CROWN_A + 0.0001
VERT_TOP = (CROWN_H - CROWN_A * 0.85, CROWN_R + CROWN_A * 0.55)
VERT_BOT = (BOTTOM_H + BOTTOM_A * 0.45, BOTTOM_R + BOTTOM_A * 0.85)
XD = CAJON_D / 2 + GAP + VERT_TOP[1] + ROPE_R
XP = XD - RF
TILT = 12.0
HEAD_TOP = H_BEAR + SKIN


def rope_path(heights):
    """🪢️ Centre line (h, r) of one rope vertical: straight between the rings, lying on the bowl where the bowl bulges out."""
    (h0, a), (h1, b) = VERT_BOT, VERT_TOP
    out = []
    for h in heights:
        line = a + (b - a) * (h - h0) / (h1 - h0)
        out.append((h, max(line, shell_r(h) + ROPE_R + 0.0002)))
    return out


ROPE_50 = rope_path([VERT_TOP[0], 0.530, 0.515, 0.500, 0.485, 0.470, 0.455, 0.440, 0.425, 0.410, 0.398, VERT_BOT[0]])
ROPE_100 = rope_path([VERT_TOP[0], 0.505, 0.470, 0.430, VERT_BOT[0]])
ROPE_200 = rope_path([VERT_TOP[0], 0.480, VERT_BOT[0]])

# ---------------------------------------------------------------- materials
REFS = [
    {"id": "hornbostel-sachs-1914", "citation": "Hornbostel, E. M. von, Sachs, C. (1914). Systematik der Musikinstrumente. Ein Versuch. Zeitschrift fuer Ethnologie 46, 553-590; English: Baines, A., Wachsmann, K. P. (1961). Classification of Musical Instruments. Galpin Society Journal 14, 3-29 (djembe 211.261.1: directly struck single-skin goblet drum with open foot, cord-tensioned by rings; cajon 111.24: percussion box, directly struck idiophone)."},
    {"id": "charry-2000", "citation": "Charry, E. (2000). Mande Music: Traditional and Modern Music of the Maninka and Mandinka of Western Africa. University of Chicago Press, Chicago (djembe carved from one log of hardwood such as lenke, Afzelia africana; goatskin head; iron rings and rope lacing; played seated, tilted away from the player's lap or standing on a strap)."},
    {"id": "feldman-2006", "citation": "Feldman, H. C. (2006). Black Rhythms of Peru: Reviving African Musical Heritage in the Black Pacific. Wesleyan University Press, Middletown CT (the Afro-Peruvian cajon: a wooden box played sitting astride it, striking the front plate, the sound port in the back)."},
    {"id": "fletcher-rossing-1998", "citation": "Fletcher, N. H., Rossing, T. D. (1998). The Physics of Musical Instruments, 2nd ed. Springer, New York, chapter 18 (membranes under tension, drum heads, tension per unit length of the order of a few kN/m)."},
    {"id": "wood-handbook-2010", "citation": "Forest Products Laboratory (2010). Wood Handbook - Wood as an Engineering Material, General Technical Report FPL-GTR-190. USDA Forest Service, Madison WI (densities, elastic moduli, strengths and thermal properties of wood, birch, beech and plywood)."},
    {"id": "cirad-tropix-doussie", "citation": "CIRAD (2011). Tropix 7: Doussie / Afzelia africana and Afzelia bipindensis (lenke). Montpellier (air-dry density about 0.80, modulus of elasticity about 13 GPa, modulus of rupture about 100 MPa)."},
    {"id": "iso-10456-2007", "citation": "EN ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (thermal conductivity and specific heat of plywood, timber, rubbers and polymers)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed. Wiley (emissivities of wood, lacquer, leather and polymers; free convection on plates and cylinders)."},
    {"id": "percussion-catalogues", "citation": "Catalogue data of standard cajons and West African djembes as music shops and used-instrument dealers list them (cajon 300 x 300 x 480 mm, 4 to 5 kg, 9 to 12 mm birch plywood body, 3 to 4 mm front plate screwed on, sound port 100 to 120 mm in the back, guitar-string or snare-wire snare behind the plate; djembe 58 to 62 cm high, 30 to 33 cm head, 7 to 10 kg, carved lenke or iroko shell, goatskin head, steel crown, flesh and bottom rings, 4 to 6 mm low-stretch polyester rope with Mali weave); the pair of this asset is a typical member of these families, not one model (authored-estimate)."},
    {"id": "din-33402-2", "citation": "DIN 33402-2:2020 Ergonomics - Human body dimensions - Part 2: Values (seated knee height about 0.50 to 0.56 m and buttock-to-knee length about 0.55 to 0.62 m of adults; the seat load of a sitting adult)."},
]

MATERIALS = [
    {"id": "birch-ply", "title": T("Birch plywood 12 mm, clear lacquer worn matt", "Birkensperrholz 12 mm, Klarlack matt abgegriffen"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": RHO_PLY, "conductivityWmK": 0.17, "specificHeatJKgK": 1600, "elasticModulusPa": 8.0e9, "strengthPa": 4.0e7, "source": "literature", "reference": "wood-handbook-2010"},
    {"id": "birch-tapa", "title": T("Birch front plate 3 mm, lacquer rubbed through to bare wood", "Birken-Schlagplatte 3 mm, Lack bis aufs Holz durchgespielt"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": RHO_PLY, "conductivityWmK": 0.17, "specificHeatJKgK": 1600, "elasticModulusPa": 8.0e9, "strengthPa": 4.0e7, "source": "authored-estimate", "reference": "wood-handbook-2010"},
    {"id": "beech", "title": T("Beech battens of the snare", "Buchenleisten des Snares"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": RHO_BEECH, "conductivityWmK": 0.17, "specificHeatJKgK": 1600, "elasticModulusPa": 1.4e10, "strengthPa": 1.0e8, "source": "literature", "reference": "wood-handbook-2010"},
    {"id": "snare-steel", "title": T("Steel guitar strings of the snare", "Stahl-Gitarrensaiten des Snares"), "emissivity": 0.3, "solarAbsorptivity": 0.5, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 2.0e11, "strengthPa": 2.0e9, "source": "literature", "reference": "incropera-2018"},
    {"id": "lenke", "title": T("Lenke (Afzelia africana) carved shell, palm-oiled, darkened and scuffed", "Lenke (Afzelia africana), geschnitzter Korpus, mit Palmöl behandelt, nachgedunkelt und verschrammt"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO_LENKE, "conductivityWmK": 0.2, "specificHeatJKgK": 1600, "elasticModulusPa": 1.3e10, "strengthPa": 1.0e8, "source": "literature", "reference": "cirad-tropix-doussie"},
    {"id": "goatskin", "title": T("Shaved goatskin head 1.2 mm, hand-darkened at the playing zones", "Rasiertes Ziegenfell 1,2 mm, an den Schlagzonen von den Händen nachgedunkelt"), "emissivity": 0.95, "solarAbsorptivity": 0.6, "densityKgM3": RHO_SKIN, "conductivityWmK": 0.2, "specificHeatJKgK": 1500, "elasticModulusPa": 4.0e8, "strengthPa": 2.5e7, "source": "authored-estimate", "reference": "fletcher-rossing-1998"},
    {"id": "polyester-rope", "title": T("Low-stretch polyester rope 5 mm, faded and frayed at the knots", "Dehnungsarmes Polyesterseil 5 mm, ausgeblichen und an den Knoten ausgefranst"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": RHO_ROPE, "conductivityWmK": 0.15, "specificHeatJKgK": 1100, "elasticModulusPa": 3.0e9, "strengthPa": 3.0e8, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "ring-rope", "title": T("Steel ring 6 to 8 mm wrapped in rope loops (composite)", "Stahlring 6 bis 8 mm in Seilschlaufen gewickelt (Verbund)"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": RHO_RING, "conductivityWmK": 0.3, "specificHeatJKgK": 700, "elasticModulusPa": 2.0e11, "strengthPa": 2.35e8, "source": "authored-estimate", "reference": "percussion-catalogues"},
]

# ---------------------------------------------------------------- geometry helpers (bone-local metres)


def rev(sid, bone, material, role, tone, profile, **extra):
    """🏺️ A revolve solid about the bone axis u through the bone origin."""
    return {"id": sid, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": pts(profile), "axis": [1.0, 0.0, 0.0], "origin": [0.0, 0.0, 0.0], **extra}


def box(sid, bone, material, role, tone, lo, hi, **extra):
    """📦️ An axis-aligned box from its low and high corners (u, v, w)."""
    size = [r6(h - l) for l, h in zip(lo, hi)]
    centre = [r6((l + h) / 2) for l, h in zip(lo, hi)]
    return {"id": sid, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": size, "transform": {"translation": centre}, **extra}


def ext(sid, bone, material, role, tone, profile, plane, a, b, holes=None, **extra):
    """🧱️ An extrude solid."""
    solid = {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": pts(profile), "plane": plane, "from": r6(a), "to": r6(b), **extra}
    if holes:
        solid["holes"] = [pts(h) for h in holes]
    return solid


def torus_profile(h, r, a, n):
    """🍩️ Closed loop profile of a ring section (axial, radial); first point repeated."""
    ring = circle(h, r, a, n)
    return ring + [ring[0]]


def vertical_loft(sid, path, count, material="polyester-rope"):
    """🪢️ One rope vertical as a loft along its path, repeated about the drum axis."""
    half = TAU / count / 2
    sections = [{"at": [r6(h), r6(r * math.cos(half)), r6(r * math.sin(half))], "rx": ROPE_R, "ry": ROPE_R, "rotation": [0.5, 0.5, 0.5, 0.5]} for h, r in path]
    return {"id": sid, "kind": "loft", "bone": "djembe", "material": material, "role": "detail", "tone": 0.3, "slender": True, "sections": sections, "capStart": True, "capEnd": True, "repeat": {"count": count, "rotationAxis": [1.0, 0.0, 0.0], "rotationRad": r6(TAU / count), "origin": [0.0, 0.0, 0.0]}}


def side_profile(rounding, n):
    """📐️ Side elevation (u, v) of the carcass: sharp at the bottom and at the front, the top back edge worn round."""
    v_front = -X_TAPA_IN
    v_back = CAJON_D / 2
    corner = arc(CAJON_H - rounding, v_back - rounding, rounding, 90, 0, n) if rounding > 0 else [(CAJON_H, v_back)]
    return [(0.0, v_front), (0.0, v_back), (CAJON_H - rounding, v_back)] + [(u, v) for u, v in corner[1:]] + [(CAJON_H, v_front)]


def shell_loop(outer, inner):
    """🪵️ Closed loop of the shell section."""
    loop = list(outer) + list(inner)
    return loop + [loop[0]]


def collar_loop(heights):
    """🐐️ Closed loop of the skin collar: inside on the shell, outside one skin thickness out, over the bearing edge to the head disc."""
    inner = [(h, shell_r(h)) for h in heights]
    inner += [(H_BEAR - 0.001, 0.1505), (H_BEAR, 0.1470), (H_BEAR, HEAD_R - 0.002)]
    outer = [(H_BEAR + SKIN, HEAD_R - 0.002), (H_BEAR + SKIN, 0.1475), (H_BEAR - 0.0003, 0.1518), (0.5925, 0.1558)]
    outer += [(h, shell_r(h) + SKIN) for h in reversed(heights) if h < 0.590]
    loop = inner + outer
    return loop + [loop[0]]


def half_outline(outer, top):
    """✏️ Half profile closed to the axis for a 2D revolved shape (u, r)."""
    return [(0.0, 0.0)] + list(outer) + [(top, 0.0)]


# ---------------------------------------------------------------- 2D shape helpers


def shape(sid, bone, role, z, tone, geometry, filled=True, region=None, material=None, **extra):
    """✏️ One 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, **geometry}
    if region:
        s["region"] = region
    if material:
        s["material"] = material
    s.update(extra)
    return s


def poly(points, closed=True):
    """🔷️ Poly geometry."""
    return {"kind": "poly", "points": pts(points), "closed": closed}


def path_lines(lines):
    """〰️ A path of open polylines."""
    d = " ".join("M " + " L ".join(f"{r6(x)} {r6(y)}" for x, y in line) for line in lines)
    return {"kind": "path", "d": d}


def revolved(axis_bone="djembe"):
    """🌀️ A mirrored revolved solid placement about the u axis."""
    return {"kind": "revolved", "axisBone": axis_bone, "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}


def extruded(z0, z1):
    """🧱️ A lateral extruded solid placement."""
    return {"kind": "extruded", "sweep": "w", "z0": r6(z0), "z1": r6(z1)}


def verticals_world(path, phis):
    """🌐️ World rest points (x, y, z) of rope verticals at azimuths phi about the djembe axis."""
    return [[(XD + r * math.cos(p), h, r * math.sin(p)) for h, r in path] for p in phis]


PHIS = [TAU * (k + 0.5) / N_VERT for k in range(N_VERT)]
PHIS12 = [TAU * (2 * k + 1) / N_VERT for k in range(N_VERT // 2)]

# ---------------------------------------------------------------- volumes per scale


def cajon_solids_50():
    """📦️ 1:50 cajon body solids."""
    prof = side_profile(TOP_ROUND, 4)
    top = [(CAJON_H - PLY, -X_TAPA_IN), (CAJON_H - PLY, CAJON_D / 2), (CAJON_H - TOP_ROUND, CAJON_D / 2)] + arc(CAJON_H - TOP_ROUND, CAJON_D / 2 - TOP_ROUND, TOP_ROUND, 90, 0, 4)[1:] + [(CAJON_H, -X_TAPA_IN)]
    wi = CAJON_W / 2 - PLY
    port = circle(0.0, PORT_U, PORT_R, 24)
    back_profile = [(-wi, PLY), (wi, PLY), (wi, CAJON_H - PLY), (-wi, CAJON_H - PLY)]
    return [
        ext("side-right", "cajon", "birch-ply", "body", 0.6, prof, "xy", -CAJON_W / 2, -wi),
        ext("side-left", "cajon", "birch-ply", "body", 0.6, prof, "xy", wi, CAJON_W / 2),
        ext("top", "cajon", "birch-ply", "body", 0.64, top, "xy", -wi, wi),
        box("bottom", "cajon", "birch-ply", "body", 0.56, (0.0, -X_TAPA_IN, -wi), (PLY, CAJON_D / 2, wi)),
        ext("back", "cajon", "birch-ply", "body", 0.56, back_profile, "zx", CAJON_D / 2 - BACK, CAJON_D / 2, holes=[list(reversed(port))]),
        box("tapa", "cajon", "birch-tapa", "body", 0.74, (0.0, -CAJON_D / 2, -CAJON_W / 2), (CAJON_H, -X_TAPA_IN, CAJON_W / 2), sheet=True),
        box("snare-bar", "cajon", "beech", "detail", 0.45, (0.430, -X_TAPA_IN + 0.002, -wi), (0.448, -X_TAPA_IN + 0.020, wi)),
        box("snare-block", "cajon", "beech", "detail", 0.45, (PLY, -X_TAPA_IN + 0.002, -wi), (PLY + 0.018, -X_TAPA_IN + 0.020, wi)),
        {"id": "snare-strings", "kind": "capsule", "bone": "cajon", "material": "snare-steel", "role": "detail", "tone": 0.3, "slender": True, "p0": [r6(PLY + 0.018), r6(-X_TAPA_IN + 0.0015), -0.06], "p1": [0.430, r6(-X_TAPA_IN + 0.0015), -0.06], "r0": 0.0005, "r1": 0.0005, "repeat": {"count": 4, "translation": [0.0, 0.0, 0.04]}},
        box("tapa-wear-centre", "cajon", "birch-tapa", "texture", 0.92, (0.330, -CAJON_D / 2 - 0.0004, -0.055), (0.462, -CAJON_D / 2, 0.055), texture=True),
        box("tapa-wear-left", "cajon", "birch-tapa", "texture", 0.92, (0.405, -CAJON_D / 2 - 0.0004, 0.080), (0.472, -CAJON_D / 2, 0.144), texture=True),
        box("tapa-wear-right", "cajon", "birch-tapa", "texture", 0.92, (0.405, -CAJON_D / 2 - 0.0004, -0.144), (0.472, -CAJON_D / 2, -0.080), texture=True),
        box("seat-wear", "cajon", "birch-ply", "texture", 0.45, (CAJON_H, -0.040, -0.110), (CAJON_H + 0.0004, 0.110, 0.110), texture=True),
        box("dealer-sticker", "cajon", "birch-ply", "texture", 0.95, (0.360, 0.040, -CAJON_W / 2 - 0.0004), (0.400, 0.100, -CAJON_W / 2), texture=True),
    ]


def cajon_solids_100():
    """📦️ 1:100 cajon body: one carcass tube with the worn top back edge, the back with its port, the plate."""
    wi = CAJON_W / 2 - PLY
    tube = [(-CAJON_W / 2, 0.0), (CAJON_W / 2, 0.0), (CAJON_W / 2, CAJON_H), (-CAJON_W / 2, CAJON_H)]
    hole = [(-wi, PLY), (-wi, CAJON_H - PLY), (wi, CAJON_H - PLY), (wi, PLY)]
    port = circle(0.0, PORT_U, PORT_R, 16)
    back_profile = [(-wi, PLY), (wi, PLY), (wi, CAJON_H - PLY), (-wi, CAJON_H - PLY)]
    return [
        ext("carcass", "cajon", "birch-ply", "body", 0.6, tube, "zx", -X_TAPA_IN, CAJON_D / 2, holes=[hole]),
        ext("back", "cajon", "birch-ply", "body", 0.56, back_profile, "zx", CAJON_D / 2 - BACK, CAJON_D / 2, holes=[list(reversed(port))]),
        box("tapa", "cajon", "birch-tapa", "body", 0.74, (0.0, -CAJON_D / 2, -CAJON_W / 2), (CAJON_H, -X_TAPA_IN, CAJON_W / 2), sheet=True),
    ]


def cajon_solids_200():
    """📦️ 1:200 cajon body: the carcass and back as one box shell open only at the port, the plate."""
    wi = CAJON_W / 2 - PLY
    outer = [(0.0, -X_TAPA_IN), (0.0, CAJON_D / 2), (CAJON_H, CAJON_D / 2), (CAJON_H, -X_TAPA_IN)]
    return [
        ext("carcass", "cajon", "birch-ply", "body", 0.6, [(-CAJON_W / 2, 0.0), (CAJON_W / 2, 0.0), (CAJON_W / 2, CAJON_H), (-CAJON_W / 2, CAJON_H)], "zx", -X_TAPA_IN, CAJON_D / 2 - BACK, holes=[[(-wi, PLY), (-wi, CAJON_H - PLY), (wi, CAJON_H - PLY), (wi, PLY)]]),
        ext("back", "cajon", "birch-ply", "body", 0.56, [(-CAJON_W / 2, 0.0), (CAJON_W / 2, 0.0), (CAJON_W / 2, CAJON_H), (-CAJON_W / 2, CAJON_H)], "zx", CAJON_D / 2 - BACK, CAJON_D / 2, holes=[list(reversed(circle(0.0, PORT_U, PORT_R, 8)))]),
        box("tapa", "cajon", "birch-tapa", "body", 0.74, (0.0, -CAJON_D / 2, -CAJON_W / 2), (CAJON_H, -X_TAPA_IN, CAJON_W / 2), sheet=True),
    ], outer


SHELL_OUT_100 = [(0.000, 0.1130), (0.030, 0.1125), (0.120, 0.0935), (0.240, 0.0755), (0.322, 0.0700), (0.375, 0.0900), (0.445, 0.1310), (0.525, 0.1550), (0.570, 0.1586), (0.592, 0.1545), (0.598, 0.1470)]
SHELL_IN_100 = [(0.598, 0.1430), (0.560, 0.1420), (0.485, 0.1260), (0.405, 0.0870), (0.350, 0.0520), (0.300, 0.0470), (0.180, 0.0600), (0.070, 0.0810), (0.000, 0.0940)]
SHELL_OUT_200 = [(0.000, 0.1130), (0.060, 0.1065), (0.200, 0.0810), (0.322, 0.0700), (0.400, 0.1065), (0.480, 0.1445), (0.565, 0.1586), (0.598, 0.1470)]
SHELL_IN_200 = [(0.598, 0.1400), (0.500, 0.1300), (0.380, 0.0690), (0.300, 0.0480), (0.150, 0.0650), (0.000, 0.0940)]


def djembe_solids(scale):
    """🥁️ The djembe body solids of one scale."""
    if scale == 50:
        out, inn, n_ring, rope, count = SHELL_OUTER, SHELL_INNER, 16, ROPE_50, N_VERT
        collar = collar_loop([COLLAR_LOW, 0.560, 0.575, 0.582, 0.588, 0.592])
    elif scale == 100:
        out, inn, n_ring, rope, count = SHELL_OUT_100, SHELL_IN_100, 10, ROPE_100, N_VERT
        collar = collar_loop([COLLAR_LOW, 0.565, 0.582, 0.592])
    else:
        out, inn, n_ring, rope, count = SHELL_OUT_200, SHELL_IN_200, 8, ROPE_200, N_VERT // 2
        collar = collar_loop([COLLAR_LOW, 0.570, 0.590])
    solids = [
        rev("shell", "djembe", "lenke", "body", 0.38, shell_loop(out, inn)),
        rev("head-collar", "djembe", "goatskin", "body", 0.84, collar),
        rev("head", "djembe-head", "goatskin", "body", 0.88, [(0.0, 0.0), (0.0, HEAD_R), (SKIN, HEAD_R), (SKIN, 0.0)], sheet=True),
        rev("crown-ring", "djembe", "ring-rope", "body", 0.25, torus_profile(CROWN_H, CROWN_R, CROWN_A, n_ring), slender=True),
        rev("bottom-ring", "djembe", "ring-rope", "body", 0.25, torus_profile(BOTTOM_H, BOTTOM_R, BOTTOM_A, n_ring), slender=True),
        vertical_loft("verticals", rope, count),
    ]
    if scale == 50:
        solids += [
            box("head-grease", "djembe-head", "goatskin", "texture", 0.6, (SKIN, 0.055, -0.07), (SKIN + 0.0004, 0.125, 0.07), texture=True),
            rev("foot-groove-low", "djembe", "lenke", "texture", 0.25, torus_profile(0.030, shell_r(0.030) - 0.0005, 0.0015, 8), texture=True),
            rev("foot-groove-high", "djembe", "lenke", "texture", 0.25, torus_profile(0.050, shell_r(0.050) - 0.0005, 0.0015, 8), texture=True),
        ]
    return solids


# ---------------------------------------------------------------- areas and masses (hand calculation, Pappus)


def loop_area_volume(loop):
    """📏️ Surface area and volume of a closed-loop revolve (Pappus)."""
    area = 0.0
    a2 = 0.0
    m = 0.0
    for (h0, r0), (h1, r1) in zip(loop, loop[1:]):
        area += TAU * (r0 + r1) / 2 * math.hypot(h1 - h0, r1 - r0)
        cross = h0 * r1 - h1 * r0
        a2 += cross
        m += cross * (r0 + r1)
    sec = a2 / 2
    rc = m / (6 * sec) if sec else 0.0
    return area, abs(TAU * rc * sec)


def loop_centroid_h(loop):
    """⚖️ Axial centroid of a closed-loop revolve weighted by radius."""
    num = 0.0
    den = 0.0
    n = 200
    hs = [p[0] for p in loop]
    lo, hi = min(hs), max(hs)
    for i in range(n):
        h = lo + (hi - lo) * (i + 0.5) / n
        xs = []
        for (h0, r0), (h1, r1) in zip(loop, loop[1:]):
            if (h0 - h) * (h1 - h) <= 0 and h0 != h1:
                xs.append(r0 + (r1 - r0) * (h - h0) / (h1 - h0))
        xs.sort()
        ring = sum(xs[j + 1] ** 2 - xs[j] ** 2 for j in range(0, len(xs) - 1, 2))
        num += ring * h
        den += ring
    return num / den


SHELL_LOOP = shell_loop(SHELL_OUTER, SHELL_INNER)
A_SHELL, V_SHELL = loop_area_volume(SHELL_LOOP)
H_SHELL = loop_centroid_h(SHELL_LOOP)
COLLAR_LOOP = collar_loop([COLLAR_LOW, 0.560, 0.575, 0.582, 0.588, 0.592])
A_COLLAR, V_COLLAR = loop_area_volume(COLLAR_LOOP)
A_DISC = 2 * math.pi * HEAD_R ** 2 + TAU * HEAD_R * SKIN
V_DISC = math.pi * HEAD_R ** 2 * SKIN
A_CROWN = 4 * math.pi ** 2 * CROWN_R * CROWN_A
V_CROWN = 2 * math.pi ** 2 * CROWN_R * CROWN_A ** 2
A_BOTTOM = 4 * math.pi ** 2 * BOTTOM_R * BOTTOM_A
V_BOTTOM = 2 * math.pi ** 2 * BOTTOM_R * BOTTOM_A ** 2
ROPE_LEN = sum(math.hypot(h1 - h0, r1 - r0) for (h0, r0), (h1, r1) in zip(ROPE_50, ROPE_50[1:]))
A_VERT = N_VERT * TAU * ROPE_R * ROPE_LEN
V_VERT = N_VERT * math.pi * ROPE_R ** 2 * ROPE_LEN

WI = CAJON_W / 2 - PLY
DEPTH_CARCASS = CAJON_D - TAPA
V_SIDES = 2 * CAJON_H * DEPTH_CARCASS * PLY
V_TOPBOT = 2 * PLY * DEPTH_CARCASS * 2 * WI
V_BACK = BACK * 2 * WI * (CAJON_H - 2 * PLY) - math.pi * PORT_R ** 2 * BACK
V_TAPA = TAPA * CAJON_H * CAJON_W
V_SNARE = 2 * 0.018 * 0.018 * 2 * WI
M_CAJON = (V_SIDES + V_TOPBOT + V_BACK) * RHO_PLY + V_TAPA * RHO_PLY + V_SNARE * RHO_BEECH
OUT_CARCASS = 2 * CAJON_H * DEPTH_CARCASS + 2 * 2 * WI * DEPTH_CARCASS + 2 * PLY * DEPTH_CARCASS * 2 + 2 * WI * (CAJON_H - 2 * PLY) - math.pi * PORT_R ** 2 + 2 * PLY * CAJON_H
IN_CARCASS = 2 * (CAJON_H - 2 * PLY) * (DEPTH_CARCASS - BACK) + 2 * 2 * WI * (DEPTH_CARCASS - BACK) + 2 * WI * (CAJON_H - 2 * PLY) - math.pi * PORT_R ** 2 + TAU * PORT_R * BACK
A_CARCASS = OUT_CARCASS + IN_CARCASS
A_TAPA = 2 * CAJON_H * CAJON_W + 2 * TAPA * (CAJON_H + CAJON_W) - 2 * WI * (CAJON_H - 2 * PLY) * 0.0 - (CAJON_H * CAJON_W - 2 * WI * (CAJON_H - 2 * PLY))
M_SHELL = V_SHELL * RHO_LENKE
M_HEAD = V_DISC * RHO_SKIN
M_COLLAR = V_COLLAR * RHO_SKIN
M_RINGS = (V_CROWN + V_BOTTOM) * RHO_RING
M_VERT = V_VERT * RHO_ROPE
M_DJEMBE = M_SHELL + M_COLLAR + M_RINGS + M_VERT
H_DJEMBE = (M_SHELL * H_SHELL + M_COLLAR * 0.575 + V_CROWN * RHO_RING * CROWN_H + V_BOTTOM * RHO_RING * BOTTOM_H + M_VERT * 0.465) / M_DJEMBE
M_TOTAL = M_CAJON + M_DJEMBE + M_HEAD

# ---------------------------------------------------------------- 2D drawings


def cajon_2d(scale):
    """🖊️ Cajon shapes of one scale: elevation, front, plan."""
    rounding = {50: TOP_ROUND, 100: TOP_ROUND, 200: 0.0}[scale]
    prof = side_profile(rounding, {50: 4, 100: 2, 200: 1}[scale])
    carcass = shape("carcass", "cajon", "body", 2.0, 0.6, poly(prof), region="cajon-carcass", material="birch-ply", solid=extruded(-CAJON_W / 2, CAJON_W / 2))
    tapa = shape("tapa", "cajon", "body", 2.2, 0.74, poly([(0.0, -CAJON_D / 2), (0.0, -X_TAPA_IN), (CAJON_H, -X_TAPA_IN), (CAJON_H, -CAJON_D / 2)]), region="cajon-tapa", material="birch-tapa", solid=extruded(-CAJON_W / 2, CAJON_W / 2), **({} if scale == 50 else {"penMm": 0.25}))
    elevation = [carcass, tapa]
    front = [dict(carcass), dict(tapa)]
    plan = [dict(carcass), dict(tapa)]
    if scale == 50:
        elevation += [
            shape("dealer-sticker", "cajon", "texture", 2.4, 0.95, poly([(0.360, 0.040), (0.400, 0.040), (0.400, 0.100), (0.360, 0.100)]), filled=False, region="cajon-carcass"),
            shape("sticker-print", "cajon", "texture", 2.45, 0.5, path_lines([[(0.388, 0.050), (0.388, 0.090)], [(0.376, 0.050), (0.376, 0.078)]]), filled=False, region="cajon-carcass"),
            shape("side-scuffs", "cajon", "texture", 2.5, 0.4, path_lines([[(0.004, 0.110), (0.012, 0.135)], [(0.003, 0.125), (0.016, 0.146)], [(0.006, -0.120), (0.020, -0.141)], [(0.470, 0.060), (0.476, 0.100)]]), filled=False, region="cajon-carcass"),
            shape("ply-edge-front", "cajon", "detail", 2.3, 0.3, path_lines([[(0.0, -X_TAPA_IN), (CAJON_H, -X_TAPA_IN)]]), filled=False, region="cajon-tapa"),
        ]
        screws = []
        for u in (0.040, 0.160, 0.280, 0.400):
            for z in (-0.138, 0.138):
                screws.append((z, u))
        for z in (-0.060, 0.060):
            screws.append((z, 0.020))
        front += [
            shape("tapa-screws", "cajon", "texture", 2.6, 0.2, {"kind": "path", "d": " ".join(f"M {r6(z + 0.0035)} {r6(u)} A 0.0035 0.0035 0 1 0 {r6(z - 0.0035)} {r6(u)} A 0.0035 0.0035 0 1 0 {r6(z + 0.0035)} {r6(u)} Z" for z, u in screws)}, filled=False, region="cajon-tapa"),
            shape("tapa-wear-centre", "cajon", "texture", 2.7, 0.9, poly([(-0.055, 0.330), (0.055, 0.330), (0.050, 0.462), (-0.050, 0.462)]), filled=False, region="cajon-tapa"),
            shape("tapa-wear-left", "cajon", "texture", 2.7, 0.9, poly([(0.080, 0.405), (0.144, 0.405), (0.144, 0.472), (0.086, 0.468)]), filled=False, region="cajon-tapa"),
            shape("tapa-wear-right", "cajon", "texture", 2.7, 0.9, poly([(-0.144, 0.405), (-0.080, 0.405), (-0.086, 0.468), (-0.144, 0.472)]), filled=False, region="cajon-tapa"),
            shape("tapa-grain", "cajon", "texture", 2.65, 0.6, path_lines([[(-0.110, 0.030), (-0.112, 0.300)], [(0.020, 0.030), (0.016, 0.310)], [(0.105, 0.040), (0.108, 0.360)]]), filled=False, region="cajon-tapa"),
        ]
        plan += [
            shape("top-seams", "cajon", "detail", 2.5, 0.3, path_lines([[(-CAJON_D / 2, -WI), (X_TAPA_IN, -WI)], [(-CAJON_D / 2, WI), (X_TAPA_IN, WI)], [(-CAJON_D / 2 + BACK, -WI), (-CAJON_D / 2 + BACK, WI)], [(-CAJON_D / 2 + TOP_ROUND, -CAJON_W / 2), (-CAJON_D / 2 + TOP_ROUND, CAJON_W / 2)]]), filled=False, region="cajon-carcass"),
            shape("seat-wear", "cajon", "texture", 2.6, 0.35, poly([(-0.110, -0.110), (0.040, -0.110), (0.040, 0.110), (-0.110, 0.110)]), filled=False, region="cajon-carcass"),
        ]
    elif scale == 100:
        elevation += [shape("ply-edge-front", "cajon", "detail", 2.3, 0.3, path_lines([[(0.0, -X_TAPA_IN), (CAJON_H, -X_TAPA_IN)]]), filled=False, region="cajon-tapa")]
        front += [shape("tapa-wear", "cajon", "detail", 2.7, 0.9, poly([(-0.144, 0.405), (0.144, 0.405), (0.144, 0.472), (-0.144, 0.472)]), filled=False, region="cajon-tapa")]
        plan += [shape("top-seams", "cajon", "detail", 2.5, 0.3, path_lines([[(-CAJON_D / 2 + TOP_ROUND, -CAJON_W / 2), (-CAJON_D / 2 + TOP_ROUND, CAJON_W / 2)]]), filled=False, region="cajon-carcass")]
    return elevation, front, plan


def djembe_2d(scale):
    """🖊️ Djembe shapes of one scale: elevation, front, plan."""
    if scale == 50:
        out, collar, rope, n_ring = SHELL_OUTER, collar_loop([COLLAR_LOW, 0.560, 0.575, 0.582, 0.588, 0.592]), ROPE_50, 16
    elif scale == 100:
        out, collar, rope, n_ring = SHELL_OUT_100, collar_loop([COLLAR_LOW, 0.565, 0.582, 0.592]), ROPE_100, 10
    else:
        out, collar, rope, n_ring = SHELL_OUT_200, collar_loop([COLLAR_LOW, 0.570, 0.590]), ROPE_200, 8
    shell = shape("shell", "djembe", "body", 3.0, 0.38, poly(half_outline(out, H_BEAR)), region="djembe-shell", material="lenke", solid=revolved())
    collar_s = shape("head-collar", "djembe", "body", 3.4, 0.84, poly(collar[:-1]), region="djembe-head", material="goatskin", solid=revolved())
    head = shape("head", "djembe-head", "body", 3.5, 0.88, poly([(0.0, 0.0), (0.0, HEAD_R), (SKIN, HEAD_R), (SKIN, 0.0)]), region="djembe-head", material="goatskin", solid=revolved("djembe-head"), **({} if scale == 50 else {"penMm": 0.25}))
    crown = shape("crown-ring", "djembe", "body", 3.6, 0.25, poly(circle(CROWN_H, CROWN_R, CROWN_A, n_ring)), region="djembe-rope", material="ring-rope", solid=revolved())
    bottom = shape("bottom-ring", "djembe", "body", 3.2, 0.25, poly(circle(BOTTOM_H, BOTTOM_R, BOTTOM_A, n_ring)), region="djembe-rope", material="ring-rope", solid=revolved())
    base = [shell, collar_s, head, crown, bottom]
    elevation = [dict(s) for s in base]
    front = [dict(s) for s in base]
    plan = [dict(s) for s in (collar_s, head)] + [dict(crown, z=3.3)]
    phis = PHIS if scale != 200 else PHIS12
    world = verticals_world(rope, phis)
    side_near = [[(-(x - XD), y) for x, y, z in line][::1] for line, p in zip(world, phis) if math.sin(p) < 0 and math.cos(p) > 0]
    side_far = [[(-(x - XD), y) for x, y, z in line] for line, p in zip(world, phis) if math.sin(p) < 0 and math.cos(p) <= 0]
    front_near = [[(z, y) for x, y, z in line] for line, p in zip(world, phis) if math.cos(p) > 0 and math.sin(p) < 0]
    front_far = [[(z, y) for x, y, z in line] for line, p in zip(world, phis) if math.cos(p) > 0 and math.sin(p) >= 0]
    swap = lambda lines: [[(y, v) for v, y in line] for line in lines]
    if scale in (50, 100):
        elevation += [
            shape("verticals-front", "djembe", "detail", 3.7, 0.3, path_lines(swap(side_near)), filled=False, region="djembe-rope"),
            shape("verticals-back", "djembe", "detail", 3.7, 0.3, path_lines(swap(side_far)), filled=False, region="djembe-rope"),
        ]
        front += [
            shape("verticals-front", "djembe", "detail", 3.7, 0.3, path_lines(front_near), filled=False, region="djembe-rope"),
            shape("verticals-left", "djembe", "detail", 3.7, 0.3, path_lines(front_far), filled=False, region="djembe-rope"),
        ]
    if scale == 50:
        zig = []
        for k, p in enumerate(phis):
            if math.sin(p) >= 0:
                continue
            for h in (0.520, 0.500):
                r = interp([(a, b) for a, b in rope], h)
                zig.append([(h - 0.006, -r * math.cos(p) - 0.006), (h + 0.006, -r * math.cos(p) + 0.006)])
        elevation += [
            shape("mali-weave", "djembe", "texture", 3.8, 0.5, path_lines(zig), filled=False, region="djembe-rope"),
            shape("foot-grooves", "djembe", "texture", 3.1, 0.3, path_lines([[(0.030, -shell_r(0.030)), (0.030, shell_r(0.030))], [(0.050, -shell_r(0.050)), (0.050, shell_r(0.050))]]), filled=False, region="djembe-shell"),
            shape("shell-scuffs", "djembe", "texture", 3.15, 0.3, path_lines([[(0.004, 0.090), (0.016, 0.105)], [(0.006, -0.080), (0.020, -0.098)], [(0.150, 0.030), (0.175, 0.050)], [(0.440, -0.060), (0.460, -0.090)]]), filled=False, region="djembe-shell"),
            shape("rope-coil", "djembe", "texture", 3.25, 0.4, path_lines([[(BOTTOM_H - 0.004, -BOTTOM_R), (BOTTOM_H + 0.004, -BOTTOM_R + 0.012)], [(BOTTOM_H - 0.004, BOTTOM_R - 0.012), (BOTTOM_H + 0.004, BOTTOM_R)], [(BOTTOM_H - 0.005, -0.030), (BOTTOM_H + 0.005, 0.030)]]), filled=False, region="djembe-rope"),
        ]
        front += [
            shape("foot-grooves", "djembe", "texture", 3.1, 0.3, path_lines([[(-shell_r(0.030), 0.030), (shell_r(0.030), 0.030)], [(-shell_r(0.050), 0.050), (shell_r(0.050), 0.050)]]), filled=False, region="djembe-shell"),
        ]
        plan += [
            shape("head-grease", "djembe-head", "texture", 3.9, 0.55, poly([(XD - 0.125, -0.07), (XD - 0.055, -0.07), (XD - 0.055, 0.07), (XD - 0.125, 0.07)]), filled=False, region="djembe-head"),
            shape("head-spine", "djembe-head", "texture", 3.95, 0.4, path_lines([[(XD - 0.120, 0.010), (XD, 0.004), (XD + 0.120, -0.008)]]), filled=False, region="djembe-head"),
            shape("collar-hair", "djembe-head", "texture", 3.65, 0.5, path_lines([[(XD + R_MAX * math.cos(a) * 0.985, R_MAX * math.sin(a) * 0.985), (XD + (R_MAX - 0.010) * math.cos(a), (R_MAX - 0.010) * math.sin(a))] for a in [TAU * k / 36 for k in range(36)]]), filled=False, region="djembe-head"),
        ]
    return elevation, front, plan


PENS = {
    50: {"body": 0.25, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.35},
    100: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25},
    200: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
}

RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350)}

REP_DECISIONS = {
    50: {
        "silhouette": T("The pair as the dealer set it down: the 0.30 x 0.30 x 0.48 m cajon on its flat bottom with its 3 mm front plate ahead and the top back edge worn round, and 0.12 m ahead of it the 0.60 m djembe on its carved foot, its goblet bowl laced with 24 rope verticals between the crown ring and the bottom ring. Front: the plate with its screws and the three worn patches, the djembe in front of it; plan: the square top with the seams of the top panel, the head with the collar, crown ring and the hand-darkened zone.", "Das Paar, wie der Händler es abgestellt hat: das Cajón 0,30 x 0,30 x 0,48 m auf seinem flachen Boden mit der 3-mm-Schlagplatte vorn und abgegriffener hinterer Oberkante, und 0,12 m davor die 0,60 m hohe Djembe auf ihrem geschnitzten Fuß, die Kelchschale mit 24 Seilvertikalen zwischen Kronenring und unterem Ring geschnürt. Vorderansicht: die Platte mit ihren Schrauben und den drei abgespielten Stellen, davor die Djembe; Draufsicht: die quadratische Sitzfläche mit den Fugen des Deckels, das Fell mit Kragen, Kronenring und der von den Händen nachgedunkelten Zone."),
        "retained": T("Carcass, front plate, the worn top back edge, the dealer sticker, scuffs, plate screws and the three worn patches; shell, foot grooves, skin collar, head, crown and bottom ring, the rope verticals with their Mali weave and the excess rope coiled at the bottom ring.", "Korpus, Schlagplatte, die abgegriffene hintere Oberkante, der Händleraufkleber, Schrammen, Plattenschrauben und die drei abgespielten Stellen; Korpus der Djembe, Fußrillen, Fellkragen, Fell, Kronen- und unterer Ring, die Seilvertikalen mit ihrer Mali-Flechtung und das am unteren Ring aufgewickelte Restseil."),
        "merged": T("The flesh ring hidden under the skin is one shape with the crown ring and its rope loops; the inner snare is not drawn because no view sees it.", "Der unter dem Fell verborgene Fellring ist eine Form mit dem Kronenring und seinen Seilschlaufen; das innere Snare ist nicht gezeichnet, da keine Ansicht es sieht."),
        "omitted": T("The sound port in the back of the cajon (no view looks at the back), the knots of the weave and the hair fringe below the crown ring in the elevations.", "Die Schallöffnung in der Rückwand des Cajón (keine Ansicht blickt auf die Rückseite), die Knoten der Flechtung und der Haarsaum unter dem Kronenring in den Ansichten."),
        "thinElements": T("The front plate is drawn at its true 3 mm (0.06 mm), the skin at 1.2 mm and the ropes at 5 mm as 0.18 mm strokes; screws, worn patches, grooves and scuffs are 0.1 mm texture.", "Die Schlagplatte ist mit ihren wahren 3 mm (0,06 mm) gezeichnet, das Fell mit 1,2 mm und die Seile mit 5 mm als Striche von 0,18 mm; Schrauben, abgespielte Stellen, Rillen und Schrammen sind Textur von 0,1 mm."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.25 mm, detail 0.18 mm, texture 0.1 mm, ground 0.35 mm; the bare wood of the worn patches and the pale skin are light, rings and ropes dark.", "Körperstift 0,25 mm, Detail 0,18 mm, Textur 0,1 mm, Boden 0,35 mm; das blanke Holz der abgespielten Stellen und das helle Fell sind hell, Ringe und Seile dunkel."),
        "overlays": T("Six regions: cajon carcass, cajon front plate, djembe shell, head, rope and rings; each drawn shape names its region.", "Sechs Regionen: Cajón-Korpus, Cajón-Schlagplatte, Djembe-Korpus, Fell, Seil und Ringe; jede gezeichnete Form nennt ihre Region."),
    },
    100: {
        "silhouette": T("The cajon box with its plate and the worn top back edge, the djembe as shell, skin collar, head, the two rings and the rope verticals as strokes; front with the worn band below the top of the plate.", "Der Cajón-Kasten mit Platte und abgegriffener hinterer Oberkante, die Djembe als Korpus, Fellkragen, Fell, die zwei Ringe und die Seilvertikalen als Striche; vorn mit dem abgespielten Band unter der Oberkante der Platte."),
        "retained": T("Carcass, plate edge, the worn band, the back edge seam in plan; shell, collar, head, crown and bottom ring and the verticals.", "Korpus, Plattenkante, das abgespielte Band, die hintere Kante in der Draufsicht; Korpus, Kragen, Fell, Kronen- und unterer Ring und die Vertikalen."),
        "merged": T("The three worn patches are one band, the shell profile is generalised to 11 points.", "Die drei abgespielten Stellen sind ein Band, das Korpusprofil ist auf 11 Punkte verallgemeinert."),
        "omitted": T("Sticker, scuffs, screws, grain, grooves, the weave, the coil and the head markings.", "Aufkleber, Schrammen, Schrauben, Maserung, Rillen, Flechtung, Seilwicklung und Fellzeichnung."),
        "thinElements": T("Plate 3 mm and skin 1.2 mm at their true size through the 0.18 mm pen, ropes as 0.13 mm strokes.", "Platte 3 mm und Fell 1,2 mm in wahrer Größe über den 0,18-mm-Stift, Seile als Striche von 0,13 mm."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.18 mm, detail 0.13 mm, ground 0.25 mm.", "Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,25 mm."),
        "overlays": T("The same six regions as at 1:50.", "Dieselben sechs Regionen wie bei 1:50."),
    },
    200: {
        "silhouette": T("A square-edged box and a goblet drum: the cajon with its plate line, the djembe with its rope rings and skin collar; the head is the collar's top.", "Ein scharfkantiger Kasten und eine Kelchtrommel: das Cajón mit seiner Plattenlinie, die Djembe mit ihren Seilringen und dem Fellkragen; das Fell ist die Oberseite des Kragens."),
        "retained": T("Carcass, plate, shell, collar, head, crown and bottom ring.", "Korpus, Platte, Korpus der Djembe, Kragen, Fell, Kronen- und unterer Ring."),
        "merged": T("The worn edge is square, the shell outline has 8 points; the 24 verticals are not drawn and stay inside the bowl outline.", "Die abgegriffene Kante ist scharf, der Korpusumriss hat 8 Punkte; die 24 Vertikalen sind nicht gezeichnet und liegen im Umriss der Schale."),
        "omitted": T("All texture and detail strokes.", "Alle Textur- und Detailstriche."),
        "thinElements": T("Plate and skin at true size through the 0.18 mm pen.", "Platte und Fell in wahrer Größe über den 0,18-mm-Stift."),
        "hatching": T("None.", "Keine."),
        "styles": T("Body pen 0.18 mm, ground 0.25 mm.", "Körperstift 0,18 mm, Boden 0,25 mm."),
        "overlays": T("The same six regions as at 1:50.", "Dieselben sechs Regionen wie bei 1:50."),
    },
}


def representation(scale):
    """🗺️ One authored representation."""
    ce, cf, cp = cajon_2d(scale)
    de, df, dp = djembe_2d(scale)
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": {"elevation": ce + de, "front": cf + df, "plan": cp + dp}, "decisions": REP_DECISIONS[scale]}


# ---------------------------------------------------------------- thermal and structure lists


def thermal_list(body, scale):
    """🌡️ The thermal list: references to the body solids that carry each region."""
    region_of = {
        "side-right": "cajon-carcass", "side-left": "cajon-carcass", "top": "cajon-carcass", "bottom": "cajon-carcass", "back": "cajon-carcass", "carcass": "cajon-carcass",
        "tapa": "cajon-tapa", "shell": "djembe-shell", "head": "djembe-head", "head-collar": "djembe-head",
        "crown-ring": "djembe-rope", "bottom-ring": "djembe-rope", "verticals": "djembe-rope",
    }
    return [{"id": f"th-{s['id']}", "of": s["id"], "region": region_of[s["id"]]} for s in body if s["id"] in region_of]


def structure_list(body, scale):
    """🏗️ The structure list: carcass panels and shell as members, the rings and verticals as the tension system, the joints at the tilt pivot and under the head, mass markers at the centres of mass."""
    member_of = {
        "side-right": "cajon", "side-left": "cajon", "top": "cajon", "bottom": "cajon", "back": "cajon", "carcass": "cajon", "tapa": "cajon",
        "shell": "djembe", "head-collar": "djembe", "crown-ring": "djembe", "bottom-ring": "djembe", "verticals": "djembe", "head": "djembe-head",
    }
    out = [{"id": f"st-{s['id']}", "of": s["id"], "member": member_of[s["id"]], "part": "member"} for s in body if s["id"] in member_of]
    feature = {50: 0.0075, 100: 0.015, 200: 0.03}[scale]
    jr = max(feature, 0.008)
    out += [
        {"id": "joint-tilt-pivot", "kind": "ellipsoid", "bone": "djembe", "material": "lenke", "role": "detail", "tone": 0.25, "radii": [r6(jr), r6(jr), r6(jr)], "transform": {"translation": [r6(jr * 0.6), r6(RF - jr * 0.5), 0.0]}, "member": "djembe", "part": "joint"},
        {"id": "joint-head", "kind": "ellipsoid", "bone": "djembe-head", "material": "goatskin", "role": "detail", "tone": 0.25, "radii": [r6(jr), r6(jr), r6(jr)], "transform": {"translation": [0.0, 0.0, 0.0]}, "member": "djembe-head", "part": "joint"},
        {"id": "mass-cajon", "kind": "ellipsoid", "bone": "cajon", "material": "birch-ply", "role": "detail", "tone": 0.2, "radii": [r6(feature * 2), r6(feature * 2), r6(feature * 2)], "transform": {"translation": [r6(COM_CAJON[0]), r6(COM_CAJON[1]), 0.0]}, "member": "cajon", "part": "mass"},
        {"id": "mass-djembe", "kind": "ellipsoid", "bone": "djembe", "material": "lenke", "role": "detail", "tone": 0.2, "radii": [r6(feature * 2), r6(feature * 2), r6(feature * 2)], "transform": {"translation": [r6(H_DJEMBE), 0.0, 0.0]}, "member": "djembe", "part": "mass"},
    ]
    return out


VOLUME_DECISIONS = {
    50: [
        T("Body: the cajon built panel by panel: two 12 mm birch-ply sides with the top back edge worn to an 8 mm round, the top panel with the same round, the bottom, the 9 mm back with its 110 mm sound port (24-gon), the 3 mm front plate as a declared sheet, the snare behind it (two 18 mm beech battens and four 1 mm guitar strings, slender) and the traces of use as declared texture sheets 0.4 mm proud: the three worn patches of bare birch on the plate, the polished seat and the dealer sticker on the right side. The djembe: the carved lenke shell as one closed-loop revolve of 37 points (foot 0.229 m, waist 0.140 m at 0.32 m, bowl 0.317 m, 4 to 23 mm wall), the shaved goatskin as the 1.2 mm head disc on its own bone and the collar over the bearing edge down to 0.54 m, the crown ring and the bottom ring as rope-wrapped steel hoops (slender), 24 rope verticals of 5 mm that run straight between the rings and lie on the bowl where it bulges (slender lofts, repeated about the axis), the hand-darkened zone of the head and two carved foot grooves as texture.", "Körper: das Cajón Platte für Platte: zwei Seiten aus 12-mm-Birkensperrholz mit der zu 8 mm Radius abgegriffenen hinteren Oberkante, der Deckel mit derselben Rundung, der Boden, die 9-mm-Rückwand mit ihrer 110-mm-Schallöffnung (24-Eck), die 3-mm-Schlagplatte als deklarierte Platte, dahinter das Snare (zwei Buchenleisten 18 mm und vier Gitarrensaiten 1 mm, schlank) und die Gebrauchsspuren als deklarierte Texturfolien 0,4 mm erhaben: die drei abgespielten Stellen blanker Birke auf der Platte, die polierte Sitzfläche und der Händleraufkleber auf der rechten Seite. Die Djembe: der geschnitzte Lenke-Korpus als ein geschlossenes Rotationsprofil aus 37 Punkten (Fuß 0,229 m, Taille 0,140 m auf 0,32 m, Schale 0,317 m, Wand 4 bis 23 mm), das rasierte Ziegenfell als 1,2-mm-Fellscheibe an eigenem Knochen und der Kragen über die Gratkante bis 0,54 m hinab, Kronenring und unterer Ring als seilumwickelte Stahlreifen (schlank), 24 Seilvertikalen von 5 mm, die gerade zwischen den Ringen laufen und dort auf der Schale aufliegen, wo sie sich wölbt (schlanke Lofts, um die Achse wiederholt), die von den Händen nachgedunkelte Zone des Fells und zwei geschnitzte Fußrillen als Textur."),
        T("Thermal: the carcass panels with their inside, which the air reaches through the port, the plate, the shell with its open bore, the head disc and collar, the rings and verticals are the five regions; texture sheets and the snare are no thermal surfaces.", "Thermik: die Korpusplatten mit ihrer Innenseite, die die Luft durch die Schallöffnung erreicht, die Platte, der Korpus mit offener Bohrung, Fellscheibe und Kragen, Ringe und Vertikalen sind die fünf Regionen; Texturfolien und Snare sind keine thermischen Flächen."),
        T("Structure: the cajon box (panels and plate) and the djembe shell are the members, the rings, verticals and collar the tension system that holds the head, the head disc its own member; joints at the tilting edge of the foot and under the head centre, mass markers at the two centres of mass.", "Tragwerk: der Cajón-Kasten (Platten und Schlagplatte) und der Djembe-Korpus sind die Stäbe, Ringe, Vertikalen und Kragen das Zugsystem, das das Fell hält, die Fellscheibe ein eigener Stab; Gelenke an der Kippkante des Fußes und unter der Fellmitte, Massemarken in den zwei Schwerpunkten."),
    ],
    100: [
        T("Body for the 15 mm feature: the four 12 mm panels of sides, top and bottom are one carcass tube (an extrusion with its hollow), the back keeps its port as a 16-gon, the plate stays a 3 mm sheet; snare, texture sheets and the worn round are left out. The djembe shell is generalised to 20 points, the collar to 4 heights, the rings to 10-gon hoops, the 24 verticals to 5 sections each.", "Körper für das Merkmal von 15 mm: die vier 12-mm-Platten von Seiten, Deckel und Boden sind ein Korpusrohr (eine Extrusion mit ihrem Hohlraum), die Rückwand behält ihre Öffnung als 16-Eck, die Platte bleibt ein 3-mm-Blatt; Snare, Texturfolien und die Kantenrundung entfallen. Der Djembe-Korpus ist auf 20 Punkte verallgemeinert, der Kragen auf 4 Höhen, die Ringe auf 10-eckige Reifen, die 24 Vertikalen auf je 5 Querschnitte."),
        T("Thermal and structure follow the body: the carcass tube and the back are the carcass region and members.", "Thermik und Tragwerk folgen dem Körper: Korpusrohr und Rückwand sind Korpusregion und Stäbe."),
    ],
    200: [
        T("Body for the 30 mm feature: carcass tube and back are cut square, the port is an octagon, the plate a 3 mm sheet; the djembe shell has 14 points, the rings are octagon hoops and the rope lacing is typified to 12 of the 24 verticals with 3 sections, kept at their true 5 mm section as slender parts.", "Körper für das Merkmal von 30 mm: Korpusrohr und Rückwand sind scharfkantig, die Öffnung ein Achteck, die Platte ein 3-mm-Blatt; der Djembe-Korpus hat 14 Punkte, die Ringe sind achteckige Reifen und die Schnürung ist auf 12 der 24 Vertikalen mit 3 Querschnitten typisiert, im wahren Querschnitt von 5 mm als schlanke Teile."),
        T("Thermal and structure follow the body; the joints and mass markers are 30 mm.", "Thermik und Tragwerk folgen dem Körper; Gelenke und Massemarken messen 30 mm."),
    ],
}

COM_CAJON = (
    ((V_SIDES * 0.240 + V_TOPBOT * 0.240 + V_BACK * 0.240) * RHO_PLY + V_TAPA * RHO_PLY * 0.240 + V_SNARE * RHO_BEECH * 0.231) / M_CAJON,
    ((V_SIDES + V_TOPBOT) * RHO_PLY * 0.0015 + V_BACK * RHO_PLY * (CAJON_D / 2 - BACK / 2) + V_TAPA * RHO_PLY * (-CAJON_D / 2 + TAPA / 2) + V_SNARE * RHO_BEECH * (-X_TAPA_IN + 0.011)) / M_CAJON,
)


def volume(scale):
    """🧊️ The authored volume of one scale."""
    if scale == 50:
        body = cajon_solids_50() + djembe_solids(50)
    elif scale == 100:
        body = cajon_solids_100() + djembe_solids(100)
    else:
        body = cajon_solids_200()[0] + djembe_solids(200)
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(body, scale), "structure": structure_list(body, scale), "decisions": VOLUME_DECISIONS[scale]}


# ---------------------------------------------------------------- rig, anchors, poses, clips

RIG = {
    "kind": "mechanical",
    "root": "floor",
    "bones": [
        {"id": "floor", "parent": None, "title": T("Floor mark at the back of the cajon footprint centre", "Bodenmarke in der Mitte der Cajón-Standfläche"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "cajon", "parent": "floor", "title": T("Cajon box, from its bottom to its seat; lifted when carried", "Cajón-Kasten, vom Boden bis zur Sitzfläche; beim Tragen angehoben"), "x": 0, "y": 0, "angleDeg": 90, "lengthM": CAJON_H, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.4},
         "mass": {"kg": None, "comX": None, "comY": None, "distribution": "point", "source": "authored-estimate", "reference": "wood-handbook-2010"},
         "section": {"areaM2": None, "secondMomentM4": None, "extremeFibreM": CAJON_W / 2, "material": "birch-ply", "title": T("Box section 300 x 300 mm of 12 mm birch plywood", "Kastenquerschnitt 300 x 300 mm aus 12-mm-Birkensperrholz")}},
        {"id": "djembe-lift", "parent": "floor", "title": T("Lift of the djembe off the floor when carried", "Anheben der Djembe vom Boden beim Tragen"), "x": None, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.3}},
        {"id": "djembe-tilt", "parent": "djembe-lift", "title": T("Tilting edge of the djembe foot on the floor, on the side of the player", "Kippkante des Djembe-Fußes auf dem Boden, auf der Seite des Spielers"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 20}},
        {"id": "djembe", "parent": "djembe-tilt", "title": T("Djembe axis from the centre of the foot to the bearing edge", "Djembe-Achse von der Fußmitte zur Gratkante"), "x": RF, "y": 0, "angleDeg": 90, "lengthM": H_BEAR, "joint": {"type": "fixed"},
         "mass": {"kg": None, "comX": None, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "cirad-tropix-doussie"},
         "section": {"areaM2": None, "secondMomentM4": None, "extremeFibreM": 0.070, "material": "lenke", "title": T("Carved tube at the waist, 140 mm outside, 94 mm bore", "Geschnitztes Rohr an der Taille, außen 140 mm, Bohrung 94 mm")}},
        {"id": "djembe-head", "parent": "djembe", "title": T("Goatskin head, dips under a stroke and rebounds", "Ziegenfell, gibt beim Schlag nach und federt zurück"), "x": H_BEAR, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.003, "maxM": 0.001},
         "mass": {"kg": None, "comX": SKIN / 2, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "fletcher-rossing-1998"}},
    ],
}


def finish_rig():
    """🦴️ Fills the computed numbers into the rig."""
    b = {x["id"]: x for x in RIG["bones"]}
    b["cajon"]["mass"].update({"kg": round(M_CAJON, 3), "comX": r6(COM_CAJON[0]), "comY": r6(COM_CAJON[1])})
    a_box = CAJON_W * CAJON_D - (CAJON_W - 2 * PLY) * (CAJON_D - 2 * PLY)
    i_box = (CAJON_W ** 4 - (CAJON_W - 2 * PLY) ** 4) / 12
    b["cajon"]["section"].update({"areaM2": r6(a_box), "secondMomentM4": float(f"{i_box:.4e}")})
    b["djembe-lift"]["x"] = r6(XP)
    b["djembe"]["mass"].update({"kg": round(M_DJEMBE, 3), "comX": r6(H_DJEMBE)})
    ro, ri = 0.070, 0.047
    b["djembe"]["section"].update({"areaM2": r6(math.pi * (ro ** 2 - ri ** 2)), "secondMomentM4": float(f"{math.pi / 4 * (ro ** 4 - ri ** 4):.4e}")})
    b["djembe-head"]["mass"]["kg"] = round(M_HEAD, 3)


ANCHORS = [
    {"id": "cajon-base", "bone": "cajon", "x": 0, "y": 0, "kind": "contact", "title": T("Bottom of the cajon on the floor", "Boden des Cajón auf dem Fußboden"), "support": "contact", "footprintLengthM": CAJON_D - TAPA, "footprintWidthM": CAJON_W, "frictionCoefficient": 0.4},
    {"id": "djembe-foot", "bone": "djembe", "x": 0, "y": 0, "kind": "contact", "title": T("Foot ring of the djembe on the floor", "Fußring der Djembe auf dem Boden"), "support": "contact", "footprintLengthM": 2 * RF, "footprintWidthM": 2 * RF, "frictionCoefficient": 0.5},
    {"id": "djembe-edge", "bone": "djembe-tilt", "x": 0, "y": 0, "kind": "contact", "title": T("Edge of the foot ring on which the tilted djembe rests", "Kante des Fußrings, auf der die gekippte Djembe ruht"), "support": "contact", "footprintLengthM": 0.02, "footprintWidthM": 0.08, "frictionCoefficient": 0.5},
    {"id": "knee-left", "bone": "djembe", "x": 0.46, "y": None, "z": None, "kind": "contact", "title": T("Left inner knee of the player against the bowl", "Linkes inneres Knie des Spielers an der Schale"), "support": "contact", "normalDeg": 0, "footprintLengthM": 0.06, "footprintWidthM": 0.06, "frictionCoefficient": 0.5},
    {"id": "knee-right", "bone": "djembe", "x": 0.46, "y": None, "z": None, "kind": "contact", "title": T("Right inner knee of the player against the bowl", "Rechtes inneres Knie des Spielers an der Schale"), "support": "contact", "normalDeg": 0, "footprintLengthM": 0.06, "footprintWidthM": 0.06, "frictionCoefficient": 0.5},
    {"id": "cajon-grip", "bone": "cajon", "x": PORT_U + PORT_R, "y": CAJON_D / 2, "kind": "support", "title": T("Fingers hooked into the sound port", "In die Schallöffnung gehakte Finger"), "support": "fixed"},
    {"id": "djembe-grip", "bone": "djembe", "x": CROWN_H + CROWN_A, "y": 0, "z": None, "kind": "support", "title": T("Hand round the crown ring and its rope loops", "Hand um den Kronenring und seine Seilschlaufen"), "support": "fixed"},
    {"id": "seat", "bone": "cajon", "x": CAJON_H, "y": 0.03, "kind": "load", "title": T("Seat point of the player on the top", "Sitzpunkt des Spielers auf der Sitzfläche")},
    {"id": "strike", "bone": "djembe-head", "x": SKIN, "y": 0.10, "kind": "load", "title": T("Tone and slap zone of the head on the player's side", "Ton- und Slapzone des Fells auf der Seite des Spielers")},
]


def finish_anchors():
    """📍️ Fills the computed numbers into the anchors."""
    for a in ANCHORS:
        if a["id"] in ("knee-left", "knee-right"):
            r = shell_r(0.46) + 0.0005
            a["y"] = r6(r / math.sqrt(2))
            a["z"] = r6((1 if a["id"] == "knee-left" else -1) * r / math.sqrt(2))
        if a["id"] == "djembe-grip":
            a["z"] = r6(-CROWN_R)
        for k in ("x", "y"):
            a[k] = r6(a[k])


REGIONS = ["cajon-carcass", "cajon-tapa", "djembe-shell", "djembe-head", "djembe-rope"]


def binding(contacts):
    """🌬️ A thermal binding of every region with optional contacts {region: (fraction, tempC)}."""
    regions = {}
    for r in REGIONS:
        c = contacts.get(r)
        regions[r] = {"exposedFraction": r6(1 - c[0]) if c else 1, "sunlitFraction": 0}
        if c:
            regions[r]["contact"] = {"fraction": c[0], "tempC": c[1]}
    return {"regions": regions, "emitters": {}}


STRIKE_N = 150.0
SEAT_N = 0.75 * 75 * 9.80665

POSES = [
    {"id": "stored", "title": T("Stored at the dealer", "Beim Händler abgestellt"), "description": T("The cajon stands on its bottom, the djembe upright on its foot 0.12 m ahead of it, as the dealer set the pair down for sale.", "Das Cajón steht auf seinem Boden, die Djembe aufrecht auf ihrem Fuß 0,12 m davor, wie der Händler das Paar zum Verkauf abgestellt hat."),
     "bones": {}, "ground": "authored", "contacts": ["cajon-base", "djembe-foot"], "thermal": binding({"cajon-carcass": (0.08, 19), "djembe-shell": (0.004, 19)}), "loads": []},
    {"id": "carried", "title": T("Carried", "Getragen"), "description": T("Two helpers carry the pair to the car, each by one hand: the cajon hangs from the fingers hooked into its sound port, the djembe from the hand round its crown ring; both clear the floor.", "Zwei Helfer tragen das Paar zum Auto, jeder mit einer Hand: das Cajón hängt an den in die Schallöffnung gehakten Fingern, die Djembe an der Hand um ihren Kronenring; beide sind frei vom Boden."),
     "bones": {"cajon": {"dy": 0.345}, "djembe-lift": {"dy": 0.158}}, "ground": "authored", "contacts": ["cajon-grip", "djembe-grip"], "thermal": binding({"cajon-carcass": (0.01, 32), "djembe-rope": (0.03, 32)}), "loads": []},
    {"id": "played", "title": T("Played seated on the cajon", "Auf dem Cajón sitzend gespielt"), "description": T("The player sits on the cajon and holds the djembe between the knees, tilted 12 degrees towards the lap on the near edge of its foot so the open foot speaks; the hands strike the head on the near side.", "Der Spieler sitzt auf dem Cajón und hält die Djembe zwischen den Knien, um 12 Grad zum Schoß gekippt auf der nahen Kante ihres Fußes, damit der offene Fuß klingt; die Hände schlagen das Fell auf der nahen Seite."),
     "bones": {"djembe-tilt": {"angleDeg": TILT}}, "ground": "authored", "contacts": ["cajon-base", "djembe-edge", "knee-left", "knee-right"],
     "thermal": binding({"cajon-carcass": (0.16, 25.5), "djembe-head": (0.06, 33), "djembe-shell": (0.03, 31), "djembe-rope": (0.04, 31)}),
     "loads": [
         {"id": "sitter", "kind": "occupant", "title": T("Seated player on the top (75 % of 75 kg)", "Sitzender Spieler auf der Sitzfläche (75 % von 75 kg)"), "anchor": "seat", "fxN": 0, "fyN": round(-SEAT_N, 1), "source": "authored-estimate", "reference": "din-33402-2"},
         {"id": "strike", "kind": "live", "title": T("Hand stroke on the head along the drum axis (150 N peak)", "Handschlag auf das Fell entlang der Trommelachse (150 N Spitze)"), "anchor": "strike", "fxN": round(STRIKE_N * math.sin(math.radians(TILT)), 1), "fyN": round(-STRIKE_N * math.cos(math.radians(TILT)), 1), "source": "authored-estimate", "reference": "fletcher-rossing-1998"},
     ]},
]


def play_key(t, tilt, head):
    """🥁️ One key of the playing clip."""
    return {"t": t, "pose": "played", "ease": "smooth", "bones": {"djembe-tilt": {"angleDeg": tilt}, "djembe-head": {"dx": head}}}


CLIPS = [
    {"id": "carry", "title": T("Carry away", "Wegtragen"), "description": T("The helpers take hold of the sound port and the crown ring, lift both drums clear of the floor in 1.4 s and hold them at carrying height.", "Die Helfer greifen Schallöffnung und Kronenring, heben beide Trommeln in 1,4 s vom Boden und halten sie auf Tragehöhe."),
     "durationS": 3.0, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.8, "pose": "stored", "ease": "smooth"}, {"t": 2.2, "pose": "carried", "ease": "smooth"}, {"t": 3.0, "pose": "carried", "ease": "smooth"}],
     "contacts": [{"anchor": "cajon-base", "fromS": 0, "toS": 0.85}, {"anchor": "djembe-foot", "fromS": 0, "toS": 0.85}, {"anchor": "cajon-grip", "fromS": 0.6, "toS": 3.0}, {"anchor": "djembe-grip", "fromS": 0.6, "toS": 3.0}],
     "phases": [{"id": "grip", "fromS": 0, "toS": 0.8, "title": T("Taking hold", "Zugreifen")}, {"id": "lift", "fromS": 0.8, "toS": 2.2, "title": T("Lifting", "Anheben")}, {"id": "hold", "fromS": 2.2, "toS": 3.0, "title": T("Carrying", "Tragen")}],
     "events": [{"t": 0.6, "kind": "contact-on", "title": T("Hands take hold", "Hände greifen zu"), "target": "cajon-grip"}, {"t": 0.85, "kind": "contact-off", "title": T("Drums leave the floor", "Trommeln verlassen den Boden"), "target": "cajon-base"}],
     "tracks": []},
    {"id": "play", "title": T("Bass, tone and slap", "Bass, Ton und Slap"), "description": T("Four strokes in 2 s on the djembe held between the knees: the head dips up to 2.5 mm under each stroke and rebounds, the drum rocks between 11 and 13 degrees on the edge of its foot, the player sits on the cajon.", "Vier Schläge in 2 s auf der zwischen den Knien gehaltenen Djembe: das Fell gibt unter jedem Schlag bis zu 2,5 mm nach und federt zurück, die Trommel wiegt zwischen 11 und 13 Grad auf der Kante ihres Fußes, der Spieler sitzt auf dem Cajón."),
     "durationS": 2.0, "fps": 30, "loop": True, "rootMotion": "none",
     "keys": [play_key(0.0, 12.0, 0.0), play_key(0.12, 12.8, -0.0025), play_key(0.25, 12.4, 0.0005), play_key(0.5, 11.6, -0.0018), play_key(0.62, 11.4, 0.0004), play_key(1.0, 12.6, -0.0025), play_key(1.12, 12.9, 0.0005), play_key(1.5, 11.5, -0.0015), play_key(1.62, 11.3, 0.0003), play_key(2.0, 12.0, 0.0)],
     "contacts": [{"anchor": "cajon-base", "fromS": 0, "toS": 2.0}, {"anchor": "djembe-edge", "fromS": 0, "toS": 2.0}, {"anchor": "knee-left", "fromS": 0, "toS": 2.0}, {"anchor": "knee-right", "fromS": 0, "toS": 2.0}],
     "phases": [{"id": "bass", "fromS": 0, "toS": 0.5, "title": T("Bass stroke", "Bassschlag")}, {"id": "tone", "fromS": 0.5, "toS": 1.0, "title": T("Tone", "Ton")}, {"id": "slap", "fromS": 1.0, "toS": 1.5, "title": T("Slap", "Slap")}, {"id": "tone-2", "fromS": 1.5, "toS": 2.0, "title": T("Tone", "Ton")}],
     "events": [{"t": 0.12, "kind": "load-on", "title": T("Bass stroke", "Bassschlag"), "target": "strike"}, {"t": 0.5, "kind": "load-on", "title": T("Tone", "Ton"), "target": "strike"}, {"t": 1.0, "kind": "load-on", "title": T("Slap", "Slap"), "target": "strike"}, {"t": 1.5, "kind": "load-on", "title": T("Tone", "Ton"), "target": "strike"}],
     "tracks": [{"target": "load:strike", "ease": "linear", "keys": [{"t": 0.0, "v": 0.0}, {"t": 0.12, "v": 1.0}, {"t": 0.2, "v": 0.0}, {"t": 0.5, "v": 0.7}, {"t": 0.58, "v": 0.0}, {"t": 1.0, "v": 1.0}, {"t": 1.08, "v": 0.0}, {"t": 1.5, "v": 0.6}, {"t": 1.58, "v": 0.0}, {"t": 2.0, "v": 0.0}]}]},
]


def region(rid, title, kind, material, area, lc, geometry, layers, cap, extra=None, source="authored-estimate", reference="iso-10456-2007"):
    """🔥️ One passive thermal region."""
    r = {"id": rid, "title": title, "kind": kind, "model": "passive", "material": material, "areaM2": round(area, 4), "characteristicLengthM": lc, "geometry": geometry, "skyViewFactor": 0.5, "layers": layers, "heatCapacityJM2K": cap, "source": source, "reference": reference}
    if extra:
        r.update(extra)
    return r


def layer(lid, title, kind, material, t, k, rho, cp, reference):
    """🧶️ One layer."""
    return {"id": lid, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": round(t / k, 5), "heatCapacityJM2K": round(t * rho * cp), "source": "literature", "reference": reference}


def thermal_model():
    """🌡️ The passive thermal model of the pair."""
    c_ply = round(PLY * RHO_PLY * 1600)
    c_tapa = round(TAPA * RHO_PLY * 1600)
    c_shell = round(0.018 * RHO_LENKE * 1600)
    c_skin = round(SKIN * RHO_SKIN * 1500)
    c_rope = round(0.006 * RHO_RING * 700)
    return {
        "environment": {"airTempC": 21, "meanRadiantTempC": 21, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
        "passengers": [], "operating": [],
        "regions": [
            region("cajon-carcass", T("Cajon carcass: sides, top, bottom and back, inside and out", "Cajón-Korpus: Seiten, Deckel, Boden und Rückwand, innen und außen"), "surface", "birch-ply", AREAS["cajon-carcass"], 0.48, "vertical-plate", [layer("carcass-ply", T("Birch plywood 12 mm", "Birkensperrholz 12 mm"), "timber", "birch-ply", PLY, 0.17, RHO_PLY, 1600, "iso-10456-2007")], c_ply, {"ratedPowerW": 0, "ratedPowerBasis": "passive"}, "literature", "wood-handbook-2010"),
            region("cajon-tapa", T("Cajon front plate, played to bare wood", "Cajón-Schlagplatte, bis aufs Holz abgespielt"), "surface", "birch-tapa", AREAS["cajon-tapa"], 0.48, "vertical-plate", [layer("tapa-ply", T("Birch plywood 3 mm", "Birkensperrholz 3 mm"), "timber", "birch-tapa", TAPA, 0.17, RHO_PLY, 1600, "iso-10456-2007")], c_tapa),
            region("djembe-shell", T("Carved djembe shell with its open bore", "Geschnitzter Djembe-Korpus mit offener Bohrung"), "surface", "lenke", AREAS["djembe-shell"], 0.30, "cylinder", [layer("shell-wood", T("Lenke wall 18 mm (mean)", "Lenke-Wand 18 mm (Mittel)"), "timber", "lenke", 0.018, 0.2, RHO_LENKE, 1600, "cirad-tropix-doussie")], c_shell, {"ratedPowerW": 0, "ratedPowerBasis": "passive"}, "literature", "cirad-tropix-doussie"),
            region("djembe-head", T("Goatskin head and collar", "Ziegenfell und Kragen"), "surface", "goatskin", AREAS["djembe-head"], 0.30, "horizontal-plate-up", [layer("head-skin", T("Goatskin 1.2 mm", "Ziegenfell 1,2 mm"), "shell", "goatskin", SKIN, 0.2, RHO_SKIN, 1500, "fletcher-rossing-1998")], c_skin, None, "authored-estimate", "fletcher-rossing-1998"),
            region("djembe-rope", T("Rope lacing with crown and bottom ring", "Seilschnürung mit Kronen- und unterem Ring"), "surface", "polyester-rope", AREAS["djembe-rope"], 0.012, "cylinder", [], c_rope, None, "authored-estimate", "percussion-catalogues"),
        ],
        "emitters": [],
        "assumptions": [
            T("Both drums are acoustic and draw no power: the cajon carcass and the djembe shell each state their nameplate as 0 W passive. All five regions are inert passive surfaces in a shop at 21 degC without sun; the cajon carcass counts its inside, which the room air reaches through the 110 mm port, and the shell its open bore.", "Beide Trommeln sind akustisch und nehmen keine Leistung auf: Cajón-Korpus und Djembe-Korpus nennen ihr Typenschild als 0 W passiv. Alle fünf Regionen sind träge passive Flächen in einem Laden bei 21 °C ohne Sonne; der Cajón-Korpus zählt seine Innenseite, die die Raumluft durch die 110-mm-Öffnung erreicht, der Korpus seine offene Bohrung."),
            T("Played: the thighs and seat of the player cover the top (about 8 % of the carcass area at 32 degC) and the floor the bottom (8 % at 19 degC), stated together as 16 % at 25.5 degC; the hands cover about 6 % of the head at 33 degC and the knees 3 % of the shell and 4 % of the ropes at 31 degC. Old lacquer and palm oil are matt (emissivity 0.9), the hand-darkened goatskin absorbs more than new skin.", "Gespielt: Oberschenkel und Gesäß des Spielers bedecken die Sitzfläche (etwa 8 % der Korpusfläche bei 32 °C) und der Boden die Unterseite (8 % bei 19 °C), zusammen als 16 % bei 25,5 °C angegeben; die Hände bedecken etwa 6 % des Fells bei 33 °C, die Knie 3 % des Korpus und 4 % der Seile bei 31 °C. Alter Lack und Palmöl sind matt (Emissionsgrad 0,9), das von den Händen nachgedunkelte Ziegenfell absorbiert mehr als neues Fell."),
        ],
    }


AREAS = {}


def compute_areas():
    """📐️ Hand-calculated total outer areas of the regions at 1:50."""
    AREAS["cajon-carcass"] = A_CARCASS
    AREAS["cajon-tapa"] = 2 * CAJON_H * CAJON_W + 2 * TAPA * (CAJON_H + CAJON_W)
    AREAS["djembe-shell"] = A_SHELL
    AREAS["djembe-head"] = A_COLLAR + A_DISC
    AREAS["djembe-rope"] = A_CROWN + A_BOTTOM + A_VERT


STRUCTURE = {
    "gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [],
    "assumptions": [
        T("Two free rigid bodies. Stored, the cajon stands on its whole bottom (0.297 x 0.300 m contact) and the djembe on its foot ring (0.226 m); carried, each hangs from one hand, the cajon from the fingers in its port, the djembe from the crown ring (fixed grips that also take the moment of the offset centre of mass); played, the cajon carries the seated player (75 % of 75 kg, the feet carry the rest) on its bottom, and the djembe rests on the near edge of its foot and on the two inner knees, which hold it at its 12 degree tilt, while a 150 N stroke drives the head along the drum axis.", "Zwei freie starre Körper. Gelagert steht das Cajón auf seinem ganzen Boden (Kontakt 0,297 x 0,300 m) und die Djembe auf ihrem Fußring (0,226 m); getragen hängt jedes an einer Hand, das Cajón an den Fingern in der Öffnung, die Djembe am Kronenring (feste Griffe, die auch das Moment des versetzten Schwerpunkts aufnehmen); gespielt trägt das Cajón den sitzenden Spieler (75 % von 75 kg, die Füße tragen den Rest) auf seinem Boden, und die Djembe ruht auf der nahen Kante ihres Fußes und an den zwei inneren Knien, die sie in ihrer Neigung von 12 Grad halten, während ein Schlag von 150 N das Fell entlang der Trommelachse treibt."),
            T("The rope lacing is a self-equilibrated tension system: at a membrane tension of about 2.5 kN/m (order of Fletcher and Rossing for a tight hand drum) the 0.89 m circumference of the bearing edge carries about 2.2 kN, which the 24 verticals (about 93 N each) pull from the crown ring to the bottom ring and the shell takes in compression between bearing edge and bowl; it is internal and not a load of the global equilibrium. Masses are the modelled volumes times the densities: birch plywood 680, beech 720, lenke 800, goatskin 1100, rope 870 and the rope-wrapped steel rings 2615 kg/m3.", "Die Seilschnürung ist ein in sich ausgeglichenes Zugsystem: bei einer Fellspannung von etwa 2,5 kN/m (Größenordnung nach Fletcher und Rossing für eine straffe Handtrommel) trägt der Umfang der Gratkante von 0,89 m etwa 2,2 kN, die die 24 Vertikalen (je etwa 93 N) vom Kronenring zum unteren Ring ziehen und die der Korpus zwischen Gratkante und Schale als Druck aufnimmt; sie ist innerlich und keine Last des globalen Gleichgewichts. Massen sind die modellierten Volumina mal Dichten: Birkensperrholz 680, Buche 720, Lenke 800, Ziegenfell 1100, Seil 870 und die seilumwickelten Stahlringe 2615 kg/m3."),
    ],
    "clinical": False,
}


def build():
    """🏗️ Assembles and writes the document."""
    compute_areas()
    finish_rig()
    finish_anchors()
    reps = {str(s): representation(s) for s in (50, 100, 200)}
    reps["500"] = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature",
                   "reason": T(f"At 1:500 the pair is only {WIDTH * 2:.2f} mm long and 1.2 mm high on paper, below the 3 mm under which an asset is not drawn; its parts (the 3 mm front plate, the 5 mm ropes, the 12 mm rings) are far below every print profile.", f"Bei 1:500 ist das Paar auf dem Papier nur {WIDTH * 2:.2f} mm lang und 1,2 mm hoch, unter den 3 mm, unter denen ein Asset nicht gezeichnet wird; seine Teile (die 3-mm-Schlagplatte, die 5-mm-Seile, die 12-mm-Ringe) liegen weit unter jedem Druckprofil."),
                   "limitingFeature": T("The 0.77 m length of the whole pair", "Die Länge des ganzen Paars von 0,77 m"), "limitingSizeM": round(WIDTH, 3), "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}
    vols = {str(s): volume(s) for s in (50, 100, 200)}
    vols["500"] = {"status": "unavailable", "denominator": 500}
    asset = {
        "id": "used-djembe-and-cajon",
        "category": "objects",
        "title": T("Used Djembe and Cajón", "Gebrauchte Djembe und Cajón"),
        "summary": T(
            f"A second-hand pair of hand drums as a used-instrument dealer offers it: a rope-tuned West African djembe 0.60 m high, carved from one log of lenke with a 0.32 m bowl and a shaved goatskin head darkened by the hands at the playing zones, laced with faded 5 mm rope in 24 verticals and a Mali weave between steel crown and bottom ring, its foot ring scuffed; and a cajon box drum 0.30 x 0.30 x 0.48 m of birch plywood with a 110 mm sound port in the back, a guitar-string snare behind the 3 mm front plate, which is played through to bare wood at both upper corners and in the middle, the top back edge worn round and a dealer sticker on the side; {M_TOTAL:.1f} kg together. Poses: stored side by side on the floor, carried by two helpers by the port and the crown ring, and played seated on the cajon with the djembe tilted between the knees.",
            f"Ein gebrauchtes Paar Handtrommeln, wie es ein Gebrauchtinstrumentenhandel anbietet: eine seilgestimmte westafrikanische Djembe von 0,60 m Höhe, aus einem Stamm Lenke geschnitzt, mit einer Schale von 0,32 m und einem rasierten Ziegenfell, das an den Schlagzonen von den Händen nachgedunkelt ist, mit ausgeblichenem 5-mm-Seil in 24 Vertikalen und Mali-Flechtung zwischen stählernem Kronen- und unterem Ring geschnürt, der Fußring verschrammt; und ein Cajón 0,30 x 0,30 x 0,48 m aus Birkensperrholz mit einer 110-mm-Schallöffnung in der Rückwand, einem Gitarrensaiten-Snare hinter der 3-mm-Schlagplatte, die an beiden oberen Ecken und in der Mitte bis aufs blanke Holz abgespielt ist, der hinteren Oberkante rund abgegriffen und einem Händleraufkleber an der Seite; zusammen {M_TOTAL:.1f} kg. Posen: nebeneinander auf dem Boden gelagert, von zwei Helfern an Schallöffnung und Kronenring getragen und auf dem Cajón sitzend gespielt, die Djembe zwischen den Knien gekippt."),
        "tags": ["used", "instrument", "music", "second-hand", "percussion", "folk", "hand-drum", "djembe", "cajon", "goblet-drum", "box-drum", "membranophone", "idiophone", "goatskin", "rope-tuned", "lenke", "hardwood", "birch-plywood", "passive"],
        "provenance": {"author": "Semio staffage instrument authors (used instruments wave, author 20)", "method": "handcrafted", "created": "2026-11-22", "references": REFS,
                       "notes": T(
                           f"Djembe (Hornbostel-Sachs 211.261.1): 0.60 m to the head, foot ring 0.229 m, waist 0.140 m at 0.32 m, bowl 0.317 m, bearing edge 0.294 m; lenke wall 23 mm at the waist and 18 to 22 mm in the bowl, shell {M_SHELL:.2f} kg; shaved goatskin 1.2 mm, crown ring 0.343 m and bottom ring 0.205 m of 6 and 8 mm steel wrapped in rope, 24 verticals of 5 mm polyester rope with a Mali weave on the upper half and the excess coiled at the bottom ring; {M_DJEMBE + M_HEAD:.2f} kg. Cajon (Hornbostel-Sachs 111.24): 300 x 300 x 480 mm of 12 mm birch plywood with a 9 mm back and a 110 mm port at 0.32 m, 3 mm birch front plate with ten screws, four guitar strings on two beech battens behind it; {M_CAJON:.2f} kg. Traces of use: the plate played through to bare wood in the upper corners and in the middle, the top back edge worn to an 8 mm round, scuffs at the bottom corners, a dealer sticker, the head darkened by the hands on the player's side, faded and frayed rope, scuffed foot ring. Nothing that stops it from being played.",
                           f"Djembe (Hornbostel-Sachs 211.261.1): 0,60 m bis zum Fell, Fußring 0,229 m, Taille 0,140 m auf 0,32 m, Schale 0,317 m, Gratkante 0,294 m; Lenke-Wand 23 mm an der Taille und 18 bis 22 mm in der Schale, Korpus {M_SHELL:.2f} kg; rasiertes Ziegenfell 1,2 mm, Kronenring 0,343 m und unterer Ring 0,205 m aus 6- und 8-mm-Stahl in Seil gewickelt, 24 Vertikalen aus 5-mm-Polyesterseil mit Mali-Flechtung auf der oberen Hälfte und dem am unteren Ring aufgewickelten Restseil; {M_DJEMBE + M_HEAD:.2f} kg. Cajón (Hornbostel-Sachs 111.24): 300 x 300 x 480 mm aus 12-mm-Birkensperrholz mit 9-mm-Rückwand und 110-mm-Öffnung auf 0,32 m, 3-mm-Birken-Schlagplatte mit zehn Schrauben, dahinter vier Gitarrensaiten an zwei Buchenleisten; {M_CAJON:.2f} kg. Gebrauchsspuren: die Platte in den oberen Ecken und in der Mitte bis aufs blanke Holz abgespielt, die hintere Oberkante zu 8 mm Radius abgegriffen, Schrammen an den unteren Ecken, ein Händleraufkleber, das Fell auf der Seite des Spielers von den Händen nachgedunkelt, ausgeblichenes und ausgefranstes Seil, verschrammter Fußring. Nichts, was das Spielen verhindert.")},
        "dimensions": {"heightM": round(HEAD_TOP, 3), "widthM": round(WIDTH, 3), "depthM": round(2 * R_MAX, 3), "massKg": round(M_TOTAL, 2)},
        "materials": MATERIALS,
        "rig": RIG,
        "anchors": ANCHORS,
        "representations": reps,
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": thermal_model(),
        "structure": STRUCTURE,
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "play", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(f"[DEBUG] mass cajon {M_CAJON:.3f} djembe {M_DJEMBE:.3f} head {M_HEAD:.3f} total {M_TOTAL:.3f}; XD {XD:.4f} XP {XP:.4f} width {WIDTH:.4f} depth {2 * R_MAX:.4f} height {HEAD_TOP:.4f}")
    print(f"[DEBUG] areas {json.dumps({k: round(v, 4) for k, v in AREAS.items()})}; shell V {V_SHELL:.5f} h {H_SHELL:.3f}; rope length {ROPE_LEN:.3f}")


WIDTH = XD + R_MAX + CAJON_D / 2

if __name__ == "__main__":
    build()
