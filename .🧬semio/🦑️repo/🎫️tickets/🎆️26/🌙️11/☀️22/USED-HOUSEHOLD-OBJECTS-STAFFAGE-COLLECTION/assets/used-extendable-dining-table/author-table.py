"""🪵 Authoritative generator of the staffage asset `used-extendable-dining-table`: every hand-chosen number of the used beech
extending table (140-180 x 90 cm, butterfly folding leaf, wobbly short leg on a cardboard shim, heat and ring marks, chipped corner)
and the whole asset document written next to this script as `🔣️.json`.

Run: `python3 author-table.py` (no arguments). The reference table and every decision are in `../../reports/used-extendable-dining-table.md`.

@see ../../brief.md — the author brief of the wave
@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts — the typed contract
"""
import json
import math
import pathlib

HERE = pathlib.Path(__file__).resolve().parent
SLUG = "used-extendable-dining-table"
SCALES = ["50", "100", "200", "500"]
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}

L, W, H = 1.40, 0.90, 0.75
TT = 0.022
Y_TOP_U = H - TT
Y_TOP_MID = H - TT / 2
GAP = 0.048
Y_FRAME = Y_TOP_U - GAP
APRON_H = 0.080
Y_APRON_B = Y_FRAME - APRON_H
TA = 0.022
FX0, FX1 = 0.07, 1.33
FZ = 0.40
LEG_TOP, LEG_FOOT = 0.060, 0.045
LEG_X = (0.10, 1.30)
LEG_Z = 0.37
SHIM_T = 0.016
SHIM_X0, SHIM_LEN, SHIM_W = 1.225, 0.15, 0.10
EXT = 0.20
LEAF_T = 0.022
LEAF_X0, LEAF_LEN = 0.50, 0.20
LEAF_A_Y0 = Y_FRAME + 0.0005
HINGE_Y = LEAF_A_Y0 + LEAF_T
LEAF_LIFT = round(H - HINGE_Y, 4)
BAR_X0, BAR_LEN = 0.20, 1.00
BAR_T, BAR_Y0, BAR_Y1 = 0.008, 0.628, 0.677
BAR_Z = 0.372
RAIL_Y0, RAIL_Y1 = 0.625, Y_FRAME
RAIL_DEPTH, RAIL_SHEET = 0.012, 0.002
LIFT = 0.15
WOBBLE = 1.5
RHO_BEECH, RHO_STEEL, RHO_CARD = 720.0, 7850.0, 160.0

ORIGIN = {
    "ground": (0.0, 0.0, 0.0, 0.0),
    "shim": (SHIM_X0, 0.0, -LEG_Z, 0.0),
    "lift": (0.0, 0.0, 0.0, 0.0),
    "frame": (BAR_X0, Y_FRAME, 0.0, 0.0),
    "top-left": (0.0, Y_TOP_MID, 0.0, 0.0),
    "top-right": (L / 2, Y_TOP_MID, 0.0, 0.0),
    "leg-rr": (LEG_X[1], Y_FRAME, -LEG_Z, -90.0),
    "leaf-a": (LEAF_X0, LEAF_A_Y0 + LEAF_T / 2, 0.0, 0.0),
    "leaf-b": (LEAF_X0 + LEAF_LEN, HINGE_Y, 0.0, 180.0),
}


def T(en, de):
    """🌐 One bilingual text, English first, German second."""
    return {"en": en, "de": de}


def r(v, n=5):
    """🔢 Rounds a number for the document."""
    return round(v + 0.0, n)


def loc(bone, x, y):
    """📍 World rest (x, y) to the bone-local (u, v) of a sagittal bone."""
    ox, oy, _, ang = ORIGIN[bone]
    dx, dy = x - ox, y - oy
    c, s = math.cos(math.radians(ang)), math.sin(math.radians(ang))
    return (r(c * dx + s * dy), r(-s * dx + c * dy))


def locw(bone, z):
    """↔️ World rest lateral z to the bone-local w."""
    return r(z - ORIGIN[bone][2])


def shape(sid, bone, role, z, tone, geom, filled=True, region=None, material=None, hatch=None, regions=None, solid=None, pen=None):
    """✏️ One 2D shape of a representation."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
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
    if solid:
        s["solid"] = solid
    return s


def poly_world_elev(bone, pts, closed=True):
    """🔷 A polygon given in world rest (x, y), expressed in the elevation frame of `bone`."""
    return {"kind": "poly", "points": [list(loc(bone, x, y)) for x, y in pts], "closed": closed}


def box_world_elev(bone, x0, y0, x1, y1):
    """⬛ A world rest rectangle as an elevation polygon of `bone`."""
    return poly_world_elev(bone, [(x0, y0), (x1, y0), (x1, y1), (x0, y1)])


def rect(x0, y0, x1, y1):
    """▭ A rectangle in the frame of a front or plan view (world rest coordinates)."""
    return {"kind": "rect", "x": r(min(x0, x1)), "y": r(min(y0, y1)), "w": r(abs(x1 - x0)), "h": r(abs(y1 - y0))}


def poly(pts, closed=True):
    """🔶 A polygon in the frame of a front or plan view."""
    return {"kind": "poly", "points": [[r(x), r(y)] for x, y in pts], "closed": closed}


def path(d):
    """〰️ An SVG-style path."""
    return {"kind": "path", "d": d}


def ellipse(cx, cy, rx, ry):
    """⭕ An ellipse in the frame of a view."""
    return {"kind": "ellipse", "cx": r(cx), "cy": r(cy), "rx": r(rx), "ry": r(ry)}


def leg_profile_x(xc, y, foot=0.0):
    """📐 Half width of a linearly tapered leg at height y (60 mm at the frame, 45 mm at the floor)."""
    return (LEG_FOOT + (LEG_TOP - LEG_FOOT) * (y / Y_FRAME)) / 2


def leg_poly(xc, y_top, y_foot):
    """🦵 Tapered leg outline (x or z centred at xc) from y_top down to y_foot, world rest."""
    ht, hf = leg_profile_x(xc, y_top), leg_profile_x(xc, y_foot)
    return [(xc - hf, y_foot), (xc + hf, y_foot), (xc + ht, y_top), (xc - ht, y_top)]


MATERIALS = [
    {"id": "beech-aged", "title": T("Solid beech, oiled, sun-yellowed and worn", "Buche massiv, geölt, vergilbt und abgenutzt"), "emissivity": 0.9, "solarAbsorptivity": 0.58, "densityKgM3": RHO_BEECH, "conductivityWmK": 0.17, "specificHeatJKgK": 1600, "elasticModulusPa": 13000000000.0, "strengthPa": 40000000.0, "source": "authored-estimate", "reference": "wood-handbook-2021"},
    {"id": "beech-pale", "title": T("Solid beech, oiled, kept in the dark", "Buche massiv, geölt, im Dunkeln verwahrt"), "emissivity": 0.9, "solarAbsorptivity": 0.46, "densityKgM3": RHO_BEECH, "conductivityWmK": 0.17, "specificHeatJKgK": 1600, "elasticModulusPa": 13000000000.0, "strengthPa": 40000000.0, "source": "authored-estimate", "reference": "wood-handbook-2021"},
    {"id": "beech-frame", "title": T("Solid beech frame, scuffed", "Buche massiv, Gestell, verschrammt"), "emissivity": 0.9, "solarAbsorptivity": 0.52, "densityKgM3": RHO_BEECH, "conductivityWmK": 0.17, "specificHeatJKgK": 1600, "elasticModulusPa": 13000000000.0, "strengthPa": 40000000.0, "source": "authored-estimate", "reference": "en-338-2016"},
    {"id": "steel-zinc", "title": T("Zinc-plated steel slide, dulled", "Verzinkter Stahlauszug, stumpf"), "emissivity": 0.28, "solarAbsorptivity": 0.6, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000.0, "strengthPa": 235000000.0, "source": "literature", "reference": "en-1993-1-1"},
    {"id": "cardboard", "title": T("Folded corrugated cardboard", "Gefaltete Wellpappe"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": RHO_CARD, "conductivityWmK": 0.065, "specificHeatJKgK": 1340, "source": "authored-estimate", "reference": "iso-10456-2007"},
]

VOL = {}


def vol(key, v):
    """🧮 Accumulates a volume in m3 for a mass bucket."""
    VOL[key] = VOL.get(key, 0.0) + v


def frustum(h, a0, a1):
    """🗻 Volume of a square frustum of height h between side lengths a0 and a1."""
    return h / 3 * (a0 * a0 + a1 * a1 + a0 * a1)


vol("top-left", 0.70 * 0.90 * TT)
vol("top-right", 0.70 * 0.90 * TT - 0.5 * 0.028 * 0.038 * TT)
for half in ("top-left", "top-right"):
    vol(half, 2 * 0.58 * APRON_H * TA + 0.70 * APRON_H * TA)
    vol(half, 4 * 0.06 * GAP * TA + 2 * 0.06 * GAP * TA)
vol("top-left", 2 * frustum(Y_FRAME, LEG_FOOT, LEG_TOP))
vol("top-right", frustum(Y_FRAME, LEG_FOOT, LEG_TOP))
foot_rr = LEG_FOOT + (LEG_TOP - LEG_FOOT) * SHIM_T / Y_FRAME
vol("leg-rr", frustum(Y_FRAME - SHIM_T, foot_rr, LEG_TOP))
vol("leaf-a", LEAF_LEN * W * LEAF_T)
vol("leaf-b", LEAF_LEN * W * LEAF_T)
RAIL_AREA = (RAIL_Y1 - RAIL_Y0) * RAIL_SHEET + 2 * (RAIL_DEPTH - RAIL_SHEET) * RAIL_SHEET
MASS = {
    "top-left": VOL["top-left"] * RHO_BEECH + 2 * 0.55 * RAIL_AREA * RHO_STEEL,
    "top-right": VOL["top-right"] * RHO_BEECH + 2 * 0.55 * RAIL_AREA * RHO_STEEL,
    "leg-rr": VOL["leg-rr"] * RHO_BEECH,
    "leaf-a": VOL["leaf-a"] * RHO_BEECH,
    "leaf-b": VOL["leaf-b"] * RHO_BEECH,
    "frame": 2 * BAR_LEN * BAR_T * (BAR_Y1 - BAR_Y0) * RHO_STEEL,
    "shim": SHIM_LEN * SHIM_T * SHIM_W * RHO_CARD,
}
MASS = {k: round(v, 4) for k, v in MASS.items()}
MASS_KG = round(sum(MASS.values()), 4)

COM = {
    "top-left": (0.37, -0.075),
    "top-right": (0.33, -0.075),
    "leg-rr": (0.33, 0.0),
    "leaf-a": (0.10, 0.0),
    "leaf-b": (0.10, -0.011),
    "frame": (0.50, -0.0275),
    "shim": (0.075, 0.008),
}


def bone(bid, parent, title, joint, length, mass=True):
    """🦴 One bone at its rest placement from ORIGIN (positions relative to the parent origin)."""
    ox, oy, oz, ang = ORIGIN[bid]
    px, py, pz, pang = ORIGIN[parent] if parent else (0.0, 0.0, 0.0, 0.0)
    b = {"id": bid, "parent": parent, "title": title}
    if parent and pang != 0:
        c, s = math.cos(math.radians(pang)), math.sin(math.radians(pang))
        dx, dy = ox - px, oy - py
        b["x"], b["y"] = r(c * dx + s * dy), r(-s * dx + c * dy)
        b["angleDeg"] = ang - pang
    else:
        b["x"], b["y"] = r(ox - px), r(oy - py)
        b["angleDeg"] = ang - pang
    if oz - pz != 0:
        b["z"] = r(oz - pz)
    b["lengthM"] = length
    b["joint"] = joint
    if mass and bid in MASS:
        cx, cy = COM[bid]
        b["mass"] = {"kg": MASS[bid], "comX": cx, "comY": cy, "distribution": "point", "source": "authored-estimate", "reference": "wood-handbook-2021"}
    return b


RIG = {
    "kind": "mechanical",
    "root": "ground",
    "bones": [
        bone("ground", None, T("Floor reference", "Bodenbezug"), {"type": "fixed"}, 0),
        bone("shim", "ground", T("Folded cardboard shim under the short leg", "Gefalteter Pappkeil unter dem kurzen Bein"), {"type": "fixed"}, SHIM_LEN),
        bone("lift", "ground", T("Table lifted by the carriers", "Vom Tragenden angehobener Tisch"), {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.3}, 0),
        bone("frame", "lift", T("Stationary centre bars of the synchronous slide", "Feststehende Mittelstege des Synchronauszugs"), {"type": "fixed"}, BAR_LEN),
        bone("top-left", "lift", T("Left table half (top, aprons, two legs)", "Linke Tischhälfte (Platte, Zargen, zwei Beine)"), {"type": "prismatic", "axisDeg": 0, "minM": -EXT, "maxM": 0}, 0.70),
        bone("top-right", "lift", T("Right table half (top, aprons, left rear leg)", "Rechte Tischhälfte (Platte, Zargen, linkes hinteres Bein)"), {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": EXT}, 0.70),
        bone("leg-rr", "top-right", T("Wobbly right rear leg (loose corner bracket)", "Wackelndes rechtes hinteres Bein (lockerer Eckbeschlag)"), {"type": "revolute", "minDeg": -2, "maxDeg": 2}, r(Y_FRAME - SHIM_T)),
        bone("leaf-a", "lift", T("Fixed panel of the butterfly leaf", "Feste Hälfte der Klappeinlage"), {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": LEAF_LIFT}, LEAF_LEN),
        bone("leaf-b", "leaf-a", T("Folding panel of the butterfly leaf", "Klappende Hälfte der Klappeinlage"), {"type": "revolute", "minDeg": -180, "maxDeg": 0}, LEAF_LEN),
    ],
}


def anchor(aid, b, x, y, z, kind, title, **extra):
    """🪝 One anchor in world rest coordinates, stored bone-local."""
    u, v = loc(b, x, y)
    a = {"id": aid, "bone": b, "x": u, "y": v, "z": locw(b, z), "kind": kind, "title": title}
    a.update(extra)
    return a


FOOT = {"support": "contact", "normalDeg": 90, "footprintLengthM": 0.045, "footprintWidthM": 0.045, "frictionCoefficient": 0.4}
HAND = {"support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.08, "frictionCoefficient": 0.6}
ANCHORS = [
    anchor("leg-ll", "top-left", LEG_X[0], 0.0, LEG_Z, "contact", T("Left end, left leg on the floor", "Linkes Ende, linkes Bein am Boden"), **FOOT),
    anchor("leg-lr", "top-left", LEG_X[0], 0.0, -LEG_Z, "contact", T("Left end, right leg on the floor", "Linkes Ende, rechtes Bein am Boden"), **FOOT),
    anchor("leg-rl", "top-right", LEG_X[1], 0.0, LEG_Z, "contact", T("Right end, left leg on the floor", "Rechtes Ende, linkes Bein am Boden"), **FOOT),
    {"id": "leg-rr", "bone": "leg-rr", "x": r(Y_FRAME - SHIM_T), "y": 0, "z": 0, "kind": "contact", "title": T("Right end, short right leg on the cardboard shim", "Rechtes Ende, kurzes rechtes Bein auf dem Pappkeil"), **FOOT},
    anchor("grip-ll", "top-left", 0.06, Y_TOP_U, 0.28, "contact", T("Rear carrier, left hand under the top", "Hinterer Träger, linke Hand unter der Platte"), **HAND),
    anchor("grip-lr", "top-left", 0.06, Y_TOP_U, -0.28, "contact", T("Rear carrier, right hand under the top", "Hinterer Träger, rechte Hand unter der Platte"), **HAND),
    anchor("grip-rl", "top-right", 1.34, Y_TOP_U, 0.28, "contact", T("Front carrier, left hand under the top", "Vorderer Träger, linke Hand unter der Platte"), **HAND),
    anchor("grip-rr", "top-right", 1.34, Y_TOP_U, -0.28, "contact", T("Front carrier, right hand under the top", "Vorderer Träger, rechte Hand unter der Platte"), **HAND),
    {"id": "dish-spot", "bone": "leaf-b", "x": 0.10, "y": -0.0, "z": 0.08, "kind": "load", "title": T("Hot casserole on a trivet on the leaf", "Heißer Bräter auf einem Untersetzer auf der Einlage")},
]

REGIONS = ["table-top", "leaf", "apron", "legs", "slide", "shim"]

P = {
    "50": {"top_t": 0.022, "frame": Y_FRAME, "apron_t": 0.022, "leaf_t": 0.022, "shim_t": SHIM_T, "bar_z": (0.368, 0.376), "bar_y": (BAR_Y0, BAR_Y1), "legs": "taper", "blocks": "full", "rails": True, "chip": True, "texture": "full"},
    "100": {"top_t": 0.022, "frame": Y_FRAME, "apron_t": 0.022, "leaf_t": 0.022, "shim_t": SHIM_T, "bar_z": (0.363, 0.378), "bar_y": (0.625, Y_FRAME), "legs": "taper", "blocks": "full", "rails": False, "chip": True, "texture": "some"},
    "200": {"top_t": 0.030, "frame": 0.6725, "apron_t": 0.030, "leaf_t": 0.030, "shim_t": 0.030, "bar_z": (0.34, 0.37), "bar_y": (0.6175, 0.6725), "legs": "taper", "blocks": "reduced", "rails": False, "chip": False, "texture": None},
    "500": {"top_t": 0.030, "frame": 0.6725, "apron_t": 0.030, "leaf_t": 0.030, "shim_t": 0.030, "bar_z": (0.34, 0.37), "bar_y": (0.6175, 0.6725), "legs": "square", "blocks": None, "rails": False, "chip": False, "texture": None},
}
TONE = {"top": 0.62, "leaf": 0.82, "apron": 0.55, "block": 0.42, "leg": 0.58, "leg-far": 0.5, "bar": 0.35, "shim": 0.48}
CHIP = [(1.40 - 0.038, -0.45), (1.40 - 0.026, -0.441), (1.40 - 0.017, -0.437), (1.40 - 0.006, -0.429), (1.40, -0.422)]


def leg_w(y):
    """📏 Full width of a tapered leg at height y."""
    return LEG_FOOT + (LEG_TOP - LEG_FOOT) * y / Y_FRAME


def blocks_of(s, half):
    """🧱 Spacer blocks in the shadow gap of one half: (id, x0, x1, z0, z1)."""
    p = P[s]
    if p["blocks"] is None:
        return []
    zo, zi = FZ, FZ - p["apron_t"]
    if half == "l":
        xs = [(0.17, 0.23), (0.37, 0.43)] if p["blocks"] == "full" else [(0.25, 0.31)]
        ex = (FX0, FX0 + p["apron_t"])
    else:
        xs = [(0.97, 1.03), (1.17, 1.23)] if p["blocks"] == "full" else [(1.09, 1.15)]
        ex = (FX1 - p["apron_t"], FX1)
    out = []
    for k, (x0, x1) in enumerate(xs):
        out.append((f"block-{half}-{k}-l", x0, x1, zi, zo))
        out.append((f"block-{half}-{k}-r", x0, x1, -zo, -zi))
    out.append((f"block-{half}-end-l", ex[0], ex[1], 0.15, 0.21))
    out.append((f"block-{half}-end-r", ex[0], ex[1], -0.21, -0.15))
    return out


def top_y(s):
    """⬆️ Underside height of the top at scale s."""
    return H - P[s]["top_t"]


def leg_top(s):
    """🦵 Height where the legs end at scale s (the frame, or the slab at 1:500)."""
    return top_y(s) if P[s]["legs"] == "square" else P[s]["frame"]


def leaf_a_y(s):
    """📗 Height range of the fixed leaf panel at rest at scale s."""
    return (HINGE_Y - P[s]["leaf_t"], HINGE_Y)


def elevation(s):
    """📐 The side elevation of scale s, seen from the asset's right."""
    p, out = P[s], []
    fy, ty, at = p["frame"], top_y(s), p["apron_t"]
    sq = p["legs"] == "square"
    hatch_h = "grain-h" if s == "50" else None
    hatch_v = "grain-v" if s == "50" else None
    out.append(shape("bars", "frame", "body", 1, TONE["bar"], box_world_elev("frame", BAR_X0, p["bar_y"][0], BAR_X0 + BAR_LEN, p["bar_y"][1]), region="slide", material="steel-zinc"))
    for half, b, x0, x1 in (("l", "top-left", 0.0, 0.70), ("r", "top-right", 0.70, L)):
        out.append(shape(f"top-{half}", b, "body", 7, TONE["top"], box_world_elev(b, x0, ty, x1, H), region="table-top", material="beech-aged"))
        ax0, ax1 = (0.13 if not sq else 0.1375, 0.70) if half == "l" else (0.70, 1.27 if not sq else 1.2625)
        out.append(shape(f"apron-{half}", b, "body", 4, TONE["apron"], box_world_elev(b, ax0, fy - APRON_H, ax1, fy), region="apron", material="beech-frame", hatch=hatch_h))
        seen = {}
        for bid, bx0, bx1, bz0, bz1 in blocks_of(s, half):
            key = (bx0, bx1)
            if key in seen:
                continue
            seen[key] = bid
            out.append(shape(bid, b, "body", 3, TONE["block"], box_world_elev(b, bx0, fy, bx1, ty), region="apron", material="beech-frame"))
    if sq:
        h = 0.0375
        out.append(shape("leg-ll", "top-left", "body", 2, TONE["leg-far"], box_world_elev("top-left", LEG_X[0] - h, 0.0, LEG_X[0] + h, ty), region="legs", material="beech-frame"))
        out.append(shape("leg-lr", "top-left", "body", 5, TONE["leg"], box_world_elev("top-left", LEG_X[0] - h, 0.0, LEG_X[0] + h, ty), region="legs", material="beech-frame"))
        out.append(shape("leg-rl", "top-right", "body", 2, TONE["leg-far"], box_world_elev("top-right", LEG_X[1] - h, 0.0, LEG_X[1] + h, ty), region="legs", material="beech-frame"))
        out.append(shape("leg-rr", "leg-rr", "body", 5, TONE["leg"], box_world_elev("leg-rr", LEG_X[1] - h, p["shim_t"], LEG_X[1] + h, ty), region="legs", material="beech-frame"))
    else:
        def lp(xc, yt, yf):
            return [(xc - leg_w(yf) / 2, yf), (xc + leg_w(yf) / 2, yf), (xc + leg_w(yt) / 2, yt), (xc - leg_w(yt) / 2, yt)]
        out.append(shape("leg-ll", "top-left", "body", 2, TONE["leg-far"], poly_world_elev("top-left", lp(LEG_X[0], fy, 0.0)), region="legs", material="beech-frame"))
        out.append(shape("leg-lr", "top-left", "body", 5, TONE["leg"], poly_world_elev("top-left", lp(LEG_X[0], fy, 0.0)), region="legs", material="beech-frame", hatch=hatch_v))
        out.append(shape("leg-rl", "top-right", "body", 2, TONE["leg-far"], poly_world_elev("top-right", lp(LEG_X[1], fy, 0.0)), region="legs", material="beech-frame"))
        out.append(shape("leg-rr", "leg-rr", "body", 5, TONE["leg"], poly_world_elev("leg-rr", lp(LEG_X[1], fy, p["shim_t"])), region="legs", material="beech-frame", hatch=hatch_v))
    a0, a1 = leaf_a_y(s)
    out.append(shape("leaf-a", "leaf-a", "body", 6, TONE["leaf"], box_world_elev("leaf-a", LEAF_X0, a0, LEAF_X0 + LEAF_LEN, a1), region="leaf", material="beech-pale"))
    out.append(shape("leaf-b", "leaf-b", "body", 6.1, TONE["leaf"], {"kind": "rect", "x": 0.0, "y": r(-p["leaf_t"]), "w": LEAF_LEN, "h": p["leaf_t"]}, region="leaf", material="beech-pale"))
    out.append(shape("shim", "shim", "body", 5.5, TONE["shim"], {"kind": "rect", "x": 0.0, "y": 0.0, "w": SHIM_LEN, "h": p["shim_t"]}, region="shim", material="cardboard"))
    if s == "50":
        out.append(shape("shim-folds", "shim", "texture", 5.6, 0.3, path(" ".join(f"M 0.004 {r(SHIM_T * k / 4)} L 0.146 {r(SHIM_T * k / 4)}" for k in (1, 2, 3))), filled=False, region="shim", material="cardboard"))
        out.append(shape("scuffs-r", "top-right", "texture", 4.5, 0.3, path("M 0.12 -0.128 L 0.15 -0.118 M 0.31 -0.131 L 0.345 -0.122 M 0.33 -0.121 L 0.36 -0.113 M 0.47 -0.130 L 0.50 -0.119"), filled=False, region="apron", material="beech-frame"))
        out.append(shape("leaf-fold", "leaf-b", "detail", 6.2, 0.3, {"kind": "poly", "points": [[0.0, 0.0], [LEAF_LEN, 0.0]], "closed": False}, filled=False, region="leaf"))
    return out


def front(s):
    """🪞 The front elevation of scale s, seen from ahead (the right end of the table)."""
    p, out = P[s], []
    fy, ty, at = p["frame"], top_y(s), p["apron_t"]
    a0, a1 = leaf_a_y(s)
    sq = p["legs"] == "square"
    out.append(shape("leaf-a", "leaf-a", "body", 1, TONE["leaf"] - 0.1, rect(-W / 2, a0, W / 2, a1), region="leaf", material="beech-pale"))
    out.append(shape("leaf-b", "leaf-b", "body", 1.1, TONE["leaf"] - 0.1, {"kind": "poly", "points": [[0.0, r(-p["leaf_t"])], [LEAF_LEN, r(-p["leaf_t"])], [LEAF_LEN, 0.0], [0.0, 0.0]], "closed": True}, region="leaf", material="beech-pale", solid={"kind": "extruded", "sweep": "w", "z0": -W / 2, "z1": W / 2}))
    for bid, bx0, bx1, bz0, bz1 in blocks_of(s, "r"):
        if "end" in bid or bid.startswith("block-r-0"):
            out.append(shape(bid, "top-right", "body", 3, TONE["block"], rect(bz0, fy, bz1, ty), region="apron", material="beech-frame"))
    out.append(shape("end-apron-r", "top-right", "body", 4, TONE["apron"], rect(-0.34 if not sq else -0.3325, fy - APRON_H, 0.34 if not sq else 0.3325, fy), region="apron", material="beech-frame", hatch="grain-h" if s == "50" else None))
    out.append(shape("top-r", "top-right", "body", 7, TONE["top"], rect(-W / 2, ty, W / 2, H), region="table-top", material="beech-aged"))
    if sq:
        h = 0.0375
        out.append(shape("leg-rl", "top-right", "body", 5, TONE["leg"], rect(LEG_Z - h, 0.0, LEG_Z + h, ty), region="legs", material="beech-frame"))
        out.append(shape("leg-rr", "leg-rr", "body", 5, TONE["leg"], rect(-LEG_Z - h, p["shim_t"], -LEG_Z + h, ty), region="legs", material="beech-frame"))
    else:
        def lp(zc, yt, yf):
            return [(zc - leg_w(yf) / 2, yf), (zc + leg_w(yf) / 2, yf), (zc + leg_w(yt) / 2, yt), (zc - leg_w(yt) / 2, yt)]
        out.append(shape("leg-rl", "top-right", "body", 5, TONE["leg"], poly(lp(LEG_Z, fy, 0.0)), region="legs", material="beech-frame", hatch="grain-v" if s == "50" else None))
        out.append(shape("leg-rr", "leg-rr", "body", 5, TONE["leg"], poly(lp(-LEG_Z, fy, p["shim_t"])), region="legs", material="beech-frame", hatch="grain-v" if s == "50" else None))
    out.append(shape("shim", "shim", "body", 5.5, TONE["shim"], rect(-LEG_Z - SHIM_W / 2, 0.0, -LEG_Z + SHIM_W / 2, p["shim_t"]), region="shim", material="cardboard"))
    return out


def plan(s):
    """🗺️ The plan (top view) of scale s, the asset's left up the page."""
    p, out = P[s], []
    out.append(shape("bars-l", "frame", "body", 1, TONE["bar"], rect(BAR_X0, p["bar_z"][0], BAR_X0 + BAR_LEN, p["bar_z"][1]), region="slide", material="steel-zinc"))
    out.append(shape("bars-r", "frame", "body", 1, TONE["bar"], rect(BAR_X0, -p["bar_z"][1], BAR_X0 + BAR_LEN, -p["bar_z"][0]), region="slide", material="steel-zinc"))
    out.append(shape("leaf-a", "leaf-a", "body", 2, TONE["leaf"], rect(LEAF_X0, -W / 2, LEAF_X0 + LEAF_LEN, W / 2), region="leaf", material="beech-pale"))
    out.append(shape("leaf-b", "leaf-b", "body", 2.1, TONE["leaf"], {"kind": "poly", "points": [[0.0, r(-p["leaf_t"])], [LEAF_LEN, r(-p["leaf_t"])], [LEAF_LEN, 0.0], [0.0, 0.0]], "closed": True}, region="leaf", material="beech-pale", solid={"kind": "extruded", "sweep": "w", "z0": -W / 2, "z1": W / 2}))
    out.append(shape("top-l", "top-left", "body", 5, TONE["top"], rect(0.0, -W / 2, 0.70, W / 2), region="table-top", material="beech-aged"))
    if p["chip"]:
        out.append(shape("top-r", "top-right", "body", 5, TONE["top"], poly([(0.70, -W / 2)] + CHIP + [(L, W / 2), (0.70, W / 2)]), region="table-top", material="beech-aged"))
    else:
        out.append(shape("top-r", "top-right", "body", 5, TONE["top"], rect(0.70, -W / 2, L, W / 2), region="table-top", material="beech-aged"))
    if p["texture"]:
        staves = [-0.36 + 0.09 * k for k in range(9)]
        out.append(shape("staves-l", "top-left", "texture", 5.5, 0.3, path(" ".join(f"M 0.0 {r(z)} L 0.70 {r(z)}" for z in staves)), filled=False, region="table-top", material="beech-aged"))
        out.append(shape("staves-r", "top-right", "texture", 5.5, 0.3, path(" ".join(f"M 0.70 {r(z)} L {1.40 if z > -0.40 else 1.36} {r(z)}" for z in staves)), filled=False, region="table-top", material="beech-aged"))
        out.append(shape("heat-ring-0", "top-left", "texture", 5.6, 0.85, ellipse(0.38, 0.08, 0.105, 0.105), filled=False, region="table-top", material="beech-aged"))
        out.append(shape("heat-ring-1", "top-left", "texture", 5.6, 0.85, ellipse(0.20, -0.17, 0.085, 0.08), filled=False, region="table-top", material="beech-aged"))
        out.append(shape("price-label", "top-left", "texture", 5.7, 0.95, rect(0.05, -0.40, 0.11, -0.36), filled=False, region="table-top", material="beech-aged"))
        out.append(shape("leaf-staves", "leaf-a", "texture", 2.5, 0.3, path(" ".join(f"M 0.50 {r(z)} L 0.70 {r(z)}" for z in staves)), filled=False, region="leaf", material="beech-pale"))
    if p["texture"] == "full":
        out.append(shape("glass-ring-0", "top-right", "texture", 5.6, 0.8, ellipse(0.96, 0.20, 0.034, 0.034), filled=False, region="table-top", material="beech-aged"))
        out.append(shape("glass-ring-1", "top-right", "texture", 5.6, 0.8, ellipse(1.03, 0.26, 0.034, 0.034), filled=False, region="table-top", material="beech-aged"))
        out.append(shape("glass-ring-2", "top-right", "texture", 5.6, 0.8, ellipse(1.12, -0.11, 0.036, 0.036), filled=False, region="table-top", material="beech-aged"))
        out.append(shape("scratches", "top-right", "texture", 5.6, 0.3, path("M 0.82 -0.05 L 0.95 -0.11 M 0.86 0.02 L 1.01 -0.03 M 1.18 0.31 L 1.27 0.33 M 1.21 -0.30 L 1.33 -0.24"), filled=False, region="table-top", material="beech-aged"))
        out.append(shape("hinge-plates", "leaf-a", "texture", 2.6, 0.4, path("M 0.68 -0.33 L 0.70 -0.33 L 0.70 -0.27 L 0.68 -0.27 Z M 0.68 0.27 L 0.70 0.27 L 0.70 0.33 L 0.68 0.33 Z"), filled=False, region="leaf", material="steel-zinc"))
    return out


def solid(sid, b, material, tone, geom, role="body", **extra):
    """🧊 One volume solid on bone b."""
    out = {"id": sid, "kind": geom.pop("kind"), "bone": b, "material": material, "role": role, "tone": tone}
    out.update(geom)
    out.update(extra)
    return out


def box(b, x0, y0, z0, x1, y1, z1):
    """📦 A world rest box as a centred box of sagittal bone b with angle 0."""
    ox, oy, oz, _ = ORIGIN[b]
    return {"kind": "box", "size": [r(x1 - x0), r(y1 - y0), r(z1 - z0)], "transform": {"translation": [r((x0 + x1) / 2 - ox), r((y0 + y1) / 2 - oy), r((z0 + z1) / 2 - oz)]}}


def slab(b, pts_xz, y0, y1):
    """🟫 A plan outline (world x, z) extruded vertically from y0 to y1 on bone b with angle 0."""
    ox, oy, oz, _ = ORIGIN[b]
    return {"kind": "extrude", "plane": "zx", "profile": [[r(z - oz), r(x - ox)] for x, z in pts_xz], "from": r(y0 - oy), "to": r(y1 - oy)}


def leg_solid(b, xc, zc, y_top, y_foot, square):
    """🦵 A leg as a vertical extrusion (tapered unless square) on a bone with angle 0."""
    if square:
        h = 0.0375
        return slab(b, [(xc - h, zc - h), (xc + h, zc - h), (xc + h, zc + h), (xc - h, zc + h)], y_foot, y_top)
    h = LEG_TOP / 2
    g = slab(b, [(xc - h, zc - h), (xc + h, zc - h), (xc + h, zc + h), (xc - h, zc + h)], y_foot, y_top)
    g["taperTo"] = [r(leg_w(y_foot) / LEG_TOP), r(leg_w(y_top) / LEG_TOP)]
    return g


def leg_rr_solid(s):
    """🦿 The wobbly leg along its own downward bone (u down, v ahead)."""
    p = P[s]
    yt, yf = leg_top(s), p["shim_t"]
    u0, u1 = r(Y_FRAME - yt), r(Y_FRAME - yf)
    if p["legs"] == "square":
        h = 0.0375
        return {"kind": "extrude", "plane": "yz", "profile": [[-h, -h], [h, -h], [h, h], [-h, h]], "from": u0, "to": u1}
    h = LEG_TOP / 2
    return {"kind": "extrude", "plane": "yz", "profile": [[-h, -h], [h, -h], [h, h], [-h, h]], "from": u0, "to": u1, "taperTo": [r(leg_w(yt) / LEG_TOP), r(leg_w(yf) / LEG_TOP)]}


def body(s):
    """🪑 The body list of scale s."""
    p, out = P[s], []
    fy, ty, at = p["frame"], top_y(s), p["apron_t"]
    sq = p["legs"] == "square"
    out.append(solid("top-l", "top-left", "beech-aged", TONE["top"], slab("top-left", [(0.0, -W / 2), (0.70, -W / 2), (0.70, W / 2), (0.0, W / 2)], ty, H), **({"sheet": True} if False else {})))
    pts = [(0.70, -W / 2)] + (CHIP if p["chip"] else [(L, -W / 2)]) + [(L, W / 2), (0.70, W / 2)]
    out.append(solid("top-r", "top-right", "beech-aged", TONE["top"], slab("top-right", pts, ty, H)))
    for half, b, ax0, ax1, ex0 in (("l", "top-left", 0.12, 0.70, FX0), ("r", "top-right", 0.70, 1.28, FX1 - at)):
        out.append(solid(f"apron-{half}-l", b, "beech-frame", TONE["apron"], box(b, ax0, fy - APRON_H, FZ - at, ax1, fy, FZ)))
        out.append(solid(f"apron-{half}-r", b, "beech-frame", TONE["apron"], box(b, ax0, fy - APRON_H, -FZ, ax1, fy, -FZ + at)))
        out.append(solid(f"end-apron-{half}", b, "beech-frame", TONE["apron"], box(b, ex0, fy - APRON_H, -0.35, ex0 + at, fy, 0.35)))
        for bid, bx0, bx1, bz0, bz1 in blocks_of(s, half):
            out.append(solid(bid, b, "beech-frame", TONE["block"], box(b, bx0, fy, bz0, bx1, ty, bz1)))
    lt = leg_top(s)
    out.append(solid("leg-ll", "top-left", "beech-frame", TONE["leg"], leg_solid("top-left", LEG_X[0], LEG_Z, lt, 0.0, sq)))
    out.append(solid("leg-lr", "top-left", "beech-frame", TONE["leg"], leg_solid("top-left", LEG_X[0], -LEG_Z, lt, 0.0, sq)))
    out.append(solid("leg-rl", "top-right", "beech-frame", TONE["leg"], leg_solid("top-right", LEG_X[1], LEG_Z, lt, 0.0, sq)))
    out.append(solid("leg-rr", "leg-rr", "beech-frame", TONE["leg"], leg_rr_solid(s)))
    if p["rails"]:
        for half, b, u0, u1 in (("l", "top-left", 0.14, 0.69), ("r", "top-right", 0.01, 0.56)):
            for side, sg in (("l", 1), ("r", -1)):
                oy = ORIGIN[b][1]
                v0, v1, t = r(RAIL_Y0 - oy), r(RAIL_Y1 - oy), RAIL_SHEET
                wo, wi = 0.378, 0.378 - RAIL_DEPTH
                prof = [[v0, wo], [v1, wo], [v1, wi], [r(v1 - t), wi], [r(v1 - t), r(wo - t)], [r(v0 + t), r(wo - t)], [r(v0 + t), wi], [v0, wi]]
                prof = [[a, r(sg * c)] for a, c in prof]
                if sg < 0:
                    prof = prof[::-1]
                out.append(solid(f"rail-{half}-{side}", b, "steel-zinc", TONE["bar"], {"kind": "extrude", "plane": "yz", "profile": prof, "from": u0, "to": u1}))
    z0, z1 = p["bar_z"]
    out.append(solid("bar-l", "frame", "steel-zinc", TONE["bar"], box("frame", BAR_X0, p["bar_y"][0], z0, BAR_X0 + BAR_LEN, p["bar_y"][1], z1)))
    out.append(solid("bar-r", "frame", "steel-zinc", TONE["bar"], box("frame", BAR_X0, p["bar_y"][0], -z1, BAR_X0 + BAR_LEN, p["bar_y"][1], -z0)))
    a0, a1 = leaf_a_y(s)
    out.append(solid("leaf-a", "leaf-a", "beech-pale", TONE["leaf"], slab("leaf-a", [(LEAF_X0, -W / 2), (LEAF_X0 + LEAF_LEN, -W / 2), (LEAF_X0 + LEAF_LEN, W / 2), (LEAF_X0, W / 2)], a0, a1)))
    out.append(solid("leaf-b", "leaf-b", "beech-pale", TONE["leaf"], {"kind": "extrude", "plane": "zx", "profile": [[-W / 2, 0.0], [W / 2, 0.0], [W / 2, LEAF_LEN], [-W / 2, LEAF_LEN]], "from": r(-p["leaf_t"]), "to": 0.0}))
    out.append(solid("shim", "shim", "cardboard", TONE["shim"], {"kind": "box", "size": [SHIM_LEN, p["shim_t"], SHIM_W], "transform": {"translation": [SHIM_LEN / 2, r(p["shim_t"] / 2), 0.0]}}))
    return out


REGION_OF = {"top": "table-top", "apron": "apron", "end-apron": "apron", "block": "apron", "leg": "legs", "rail": "slide", "bar": "slide", "leaf": "leaf", "shim": "shim"}


def region_of(sid):
    """🌡️ The thermal region of a body solid by its id prefix."""
    for k in sorted(REGION_OF, key=len, reverse=True):
        if sid.startswith(k):
            return REGION_OF[k]
    raise KeyError(sid)


MEMBER_OF = {"top-l": "top-left", "top-r": "top-right", "apron-l": "top-left", "apron-r": "top-right", "end-apron-l": "top-left", "end-apron-r": "top-right", "leg-ll": "top-left", "leg-lr": "top-left", "leg-rl": "top-right", "leg-rr": "leg-rr", "rail-l": "top-left", "rail-r": "top-right", "bar": "frame", "leaf-a": "leaf-a", "leaf-b": "leaf-b", "shim": "shim", "block-l": "top-left", "block-r": "top-right"}


def member_of(sid):
    """🏗️ The member bone of a body solid by its id prefix."""
    for k in sorted(MEMBER_OF, key=len, reverse=True):
        if sid.startswith(k):
            return MEMBER_OF[k]
    raise KeyError(sid)


def thermal(s, bodies):
    """♨️ The thermal list of scale s: every body surface is the surface of its region."""
    return [{"id": f"th-{b['id']}", "of": b["id"], "region": region_of(b["id"])} for b in bodies]


def structure(s, bodies):
    """🏛️ The structure list of scale s: timber and steel members, gap blocks as joints, one mass marker per loaded bone."""
    out = []
    for b in bodies:
        part = "joint" if b["id"].startswith("block") else "member"
        out.append({"id": f"st-{b['id']}", "of": b["id"], "member": member_of(b["id"]), "part": part})
    rad = max(0.02, FEATURE[s] / 2)
    for bid, (cx, cy) in COM.items():
        ry = rad if bid not in ("leaf-a", "leaf-b", "shim") else max(FEATURE[s] / 2, 0.006)
        out.append({"id": f"mass-{bid}", "kind": "ellipsoid", "bone": bid, "material": "steel-zinc" if bid == "frame" else ("cardboard" if bid == "shim" else "beech-frame"), "role": "detail", "tone": 0.3, "radii": [r(rad), r(min(ry, rad)), r(rad)], "transform": {"translation": [cx, cy, 0.0]}, "member": bid, "part": "mass"})
    return out


def binding(top=(1, 0), leaf=(0.15, 0), apron=(1, 0), legs=(1, 0), slide=(0.3, 0), shim=(1, 0), top_contact=None, leaf_contact=None, shim_contact=(0.45, 18)):
    """🌬️ The thermal binding of a pose: exposed and sunlit fraction of every region, contacts where something touches."""
    def rb(v, c=None):
        out = {"exposedFraction": v[0], "sunlitFraction": v[1]}
        if c:
            out["contact"] = {"fraction": c[0], "tempC": c[1]}
        return out
    return {"regions": {"table-top": rb(top, top_contact), "leaf": rb(leaf, leaf_contact), "apron": rb(apron), "legs": rb(legs), "slide": rb(slide), "shim": rb(shim, shim_contact)}, "emitters": {}}


FLOOR = ["leg-ll", "leg-lr", "leg-rl", "leg-rr"]
GRIPS = ["grip-ll", "grip-lr", "grip-rl", "grip-rr"]
EXTENDED = {"top-left": {"dx": -EXT}, "top-right": {"dx": EXT}, "leaf-a": {"dy": LEAF_LIFT}, "leaf-b": {"angleDeg": -180}}
W_TABLEWARE = -135.0


def tableware():
    """🍽️ The tableware load of the laid table: 0.15 kN/m2 over the 0.90 m width on every board, plus the hot casserole."""
    rows = [("tableware-left", "top-left", 0.70, T("Tableware on the left half (0.15 kN/m2)", "Geschirr auf der linken Hälfte (0,15 kN/m2)")),
            ("tableware-leaf-a", "leaf-a", LEAF_LEN, T("Tableware on the fixed leaf panel (0.15 kN/m2)", "Geschirr auf der festen Einlagehälfte (0,15 kN/m2)")),
            ("tableware-leaf-b", "leaf-b", LEAF_LEN, T("Tableware on the folding leaf panel (0.15 kN/m2)", "Geschirr auf der klappenden Einlagehälfte (0,15 kN/m2)")),
            ("tableware-right", "top-right", 0.70, T("Tableware on the right half (0.15 kN/m2)", "Geschirr auf der rechten Hälfte (0,15 kN/m2)"))]
    out = [{"id": i, "kind": "live", "title": t, "distributed": {"bone": b, "fromM": 0, "toM": n, "wxNPerM": 0, "wyNPerM": W_TABLEWARE}, "fxN": 0, "fyN": 0, "source": "authored-estimate", "reference": "din-18011-2006"} for i, b, n, t in rows]
    out.append({"id": "casserole", "kind": "live", "title": T("Hot casserole with food on a cork trivet (3.0 kg)", "Heißer Bräter mit Essen auf einem Korkuntersetzer (3,0 kg)"), "anchor": "dish-spot", "fxN": 0, "fyN": -29.4, "source": "authored-estimate", "reference": "din-18011-2006"})
    return out


POSES = [
    {"id": "stored", "title": T("Stored in the reuse depot", "Im Gebrauchtwarenlager abgestellt"), "description": T("Closed at 1.40 m, the butterfly leaf folded in the shadow gap, the short right rear leg standing on the folded cardboard shim the depot slipped under it; no sun in the hall.", "Geschlossen mit 1,40 m, die Klappeinlage gefaltet in der Schattenfuge, das kurze rechte hintere Bein steht auf dem gefalteten Pappkeil, den das Lager untergeschoben hat; keine Sonne in der Halle."), "bones": {}, "ground": "auto", "contacts": FLOOR, "thermal": binding(), "loads": []},
    {"id": "carried", "title": T("Carried by two people", "Von zwei Personen getragen"), "description": T("Lifted 0.15 m by two carriers, one at each end with both hands under the top overhang; the loose right rear leg swings 1.5 degrees on its corner bracket, the shim stays on the floor.", "Von zwei Tragenden um 0,15 m angehoben, je eine Person an einem Ende mit beiden Händen unter dem Plattenüberstand; das lockere rechte hintere Bein pendelt 1,5 Grad im Eckbeschlag, der Pappkeil bleibt am Boden."), "bones": {"lift": {"dy": LIFT}, "leg-rr": {"angleDeg": WOBBLE}}, "ground": "authored", "contacts": GRIPS, "thermal": binding(top_contact=(0.02, 32)), "loads": []},
    {"id": "extended", "title": T("Extended to 1.80 m", "Auf 1,80 m ausgezogen"), "description": T("Halves pulled apart by 0.20 m each on the synchronous slide, the butterfly leaf raised and unfolded flush: 1.80 x 0.90 m, the pale leaf between the sun-yellowed halves.", "Hälften je 0,20 m auf dem Synchronauszug auseinandergezogen, die Klappeinlage angehoben und bündig aufgeklappt: 1,80 x 0,90 m, die helle Einlage zwischen den vergilbten Hälften."), "bones": EXTENDED, "ground": "auto", "contacts": FLOOR, "thermal": binding(top=(1, 0.3), leaf=(1, 0.4), slide=(0.6, 0)), "loads": []},
    {"id": "in-use", "title": T("Laid for six at home", "Zu Hause für sechs gedeckt"), "description": T("Back in a dining room, extended with tableware for six (0.15 kN/m2) and a hot casserole on a cork trivet on the leaf; window sun on part of the top.", "Wieder in einem Esszimmer, ausgezogen mit Geschirr für sechs (0,15 kN/m2) und einem heißen Bräter auf einem Korkuntersetzer auf der Einlage; Fenstersonne auf einem Teil der Platte."), "bones": EXTENDED, "ground": "auto", "contacts": FLOOR, "thermal": binding(top=(0.82, 0.25), leaf=(0.8, 0.3), slide=(0.6, 0), top_contact=(0.03, 30), leaf_contact=(0.05, 45)), "loads": tableware()},
    {"id": "pulled-apart-key", "title": T("Halves apart, leaf folded (key)", "Hälften auseinander, Einlage gefaltet (Schlüssel)"), "description": T("Halves pulled apart, the folded leaf still lying in the opening.", "Hälften auseinandergezogen, die gefaltete Einlage liegt noch in der Öffnung."), "key": True, "bones": {"top-left": {"dx": -EXT}, "top-right": {"dx": EXT}}, "ground": "auto", "contacts": FLOOR, "thermal": binding(leaf=(0.6, 0), slide=(0.6, 0)), "loads": []},
    {"id": "leaf-raised-key", "title": T("Folded leaf raised (key)", "Gefaltete Einlage angehoben (Schlüssel)"), "description": T("The folded leaf lifted so that its fixed panel is flush with the top.", "Die gefaltete Einlage so angehoben, dass ihre feste Hälfte bündig mit der Platte liegt."), "key": True, "bones": {"top-left": {"dx": -EXT}, "top-right": {"dx": EXT}, "leaf-a": {"dy": LEAF_LIFT}}, "ground": "auto", "contacts": FLOOR, "thermal": binding(leaf=(0.6, 0), slide=(0.6, 0)), "loads": []},
    {"id": "leaf-wing-key", "title": T("Leaf panel upright like a wing (key)", "Einlagehälfte senkrecht wie ein Flügel (Schlüssel)"), "description": T("The folding panel swung up to the vertical about its flush table hinges.", "Die Klapphälfte um ihre bündigen Tischbänder senkrecht hochgeschwenkt."), "key": True, "bones": {"top-left": {"dx": -EXT}, "top-right": {"dx": EXT}, "leaf-a": {"dy": LEAF_LIFT}, "leaf-b": {"angleDeg": -90}}, "ground": "auto", "contacts": FLOOR, "thermal": binding(leaf=(1, 0), slide=(0.6, 0)), "loads": []},
]


def spans(anchors, a, b):
    """🦶 Contact spans of several anchors."""
    return [{"anchor": x, "fromS": a, "toS": b} for x in anchors]


CLIPS = [
    {"id": "carry", "title": T("Carry it out", "Hinaustragen"), "description": T("Two carriers grip the top under the end overhangs, lift the table off the floor and the shim and carry it level at hip height; the loose leg swings.", "Zwei Tragende fassen die Platte unter den Stirnüberständen, heben den Tisch vom Boden und vom Pappkeil und tragen ihn waagerecht in Hüfthöhe; das lockere Bein pendelt."), "durationS": 3.2, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.6, "pose": "stored", "ease": "smooth"}, {"t": 1.8, "pose": "carried", "ease": "smooth"}, {"t": 2.5, "pose": "carried", "ease": "smooth", "bones": {"lift": {"dy": LIFT}, "leg-rr": {"angleDeg": -1.0}}}, {"t": 3.2, "pose": "carried", "ease": "smooth"}],
     "contacts": spans(FLOOR, 0, 0.6) + spans(GRIPS, 0.3, 3.2),
     "phases": [{"id": "grip", "fromS": 0, "toS": 0.6, "title": T("Grip under the top", "Unter die Platte greifen")}, {"id": "lift", "fromS": 0.6, "toS": 1.8, "title": T("Lift", "Anheben")}, {"id": "carry", "fromS": 1.8, "toS": 3.2, "title": T("Carry", "Tragen")}],
     "events": [{"t": 0.3, "kind": "contact-on", "title": T("Hands under the top", "Hände unter der Platte")}, {"t": 0.6, "kind": "contact-off", "title": T("Legs leave the floor, the shim stays", "Beine verlassen den Boden, der Pappkeil bleibt")}],
     "tracks": []},
    {"id": "extend", "title": T("Extend with the butterfly leaf", "Mit der Klappeinlage ausziehen"), "description": T("The halves are pulled apart on the synchronous slide, the folded leaf is raised into the opening and its folding panel swings over like a wing until the top is flush at 1.80 m.", "Die Hälften werden auf dem Synchronauszug auseinandergezogen, die gefaltete Einlage wird in die Öffnung gehoben und ihre Klapphälfte schwingt wie ein Flügel herüber, bis die Platte mit 1,80 m bündig ist."), "durationS": 5.2, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2.0, "pose": "pulled-apart-key", "ease": "smooth"}, {"t": 2.8, "pose": "leaf-raised-key", "ease": "smooth"}, {"t": 3.7, "pose": "leaf-wing-key", "ease": "linear"}, {"t": 4.6, "pose": "extended", "ease": "smooth"}, {"t": 5.2, "pose": "extended", "ease": "smooth"}],
     "contacts": spans(FLOOR, 0, 5.2),
     "phases": [{"id": "pull", "fromS": 0, "toS": 2.0, "title": T("Pull apart", "Auseinanderziehen")}, {"id": "raise", "fromS": 2.0, "toS": 2.8, "title": T("Raise the folded leaf", "Gefaltete Einlage anheben")}, {"id": "unfold", "fromS": 2.8, "toS": 5.2, "title": T("Unfold the butterfly leaf", "Klappeinlage aufklappen")}],
     "events": [{"t": 2.0, "kind": "motion", "title": T("Opening of 0.40 m", "Öffnung von 0,40 m")}, {"t": 4.6, "kind": "motion", "title": T("Leaf flush, table 1.80 m", "Einlage bündig, Tisch 1,80 m")}],
     "tracks": [{"target": "region:leaf:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.15}, {"t": 2.0, "v": 0.6}, {"t": 4.6, "v": 1}]}]},
    {"id": "lay-the-table", "title": T("Lay the table", "Tisch decken"), "description": T("On the extended table the tableware for six arrives board by board, then the hot casserole is set on the leaf.", "Auf dem ausgezogenen Tisch kommt das Geschirr für sechs Brett für Brett, dann wird der heiße Bräter auf die Einlage gestellt."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "extended", "ease": "linear"}, {"t": 6, "pose": "in-use", "ease": "linear"}],
     "contacts": spans(FLOOR, 0, 6),
     "phases": [{"id": "lay", "fromS": 0, "toS": 4.5, "title": T("Laying", "Decken")}, {"id": "serve", "fromS": 4.5, "toS": 6, "title": T("Serve", "Auftragen")}],
     "events": [{"t": 1, "kind": "load-on", "title": T("First places", "Erste Gedecke"), "target": "tableware-left"}, {"t": 5, "kind": "load-on", "title": T("Casserole on the trivet", "Bräter auf dem Untersetzer"), "target": "casserole"}],
     "tracks": [{"target": "load:tableware-left", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 1, "v": 0}, {"t": 2, "v": 1}]}, {"target": "load:tableware-leaf-a", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 2, "v": 0}, {"t": 3, "v": 1}]}, {"target": "load:tableware-leaf-b", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 2.5, "v": 0}, {"t": 3.5, "v": 1}]}, {"target": "load:tableware-right", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 3.5, "v": 0}, {"t": 4.5, "v": 1}]}, {"target": "load:casserole", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 4.8, "v": 0}, {"t": 5.2, "v": 1}]}]},
]


def layer(lid, title, kind, material, t, k, rho, cp):
    """🧶 A single layer with resistance t/k and the areal heat capacity rho cp t."""
    return {"id": lid, "title": title, "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": r(t / k, 6), "heatCapacityJM2K": r(rho * cp * t, 1), "source": "literature", "reference": "iso-10456-2007"}


AREA = {"table-top": 2.62, "leaf": 0.46, "apron": 0.86, "legs": 0.59, "slide": 0.31, "shim": 0.038}


def region(rid, title, material, area, length, geom, sky, lay, source="authored-estimate", **extra):
    """🔥 One passive thermal region."""
    out = {"id": rid, "title": title, "kind": "surface", "model": "passive", "material": material, "areaM2": area, "characteristicLengthM": length, "geometry": geom, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": source, "reference": "iso-10456-2007"}
    out.update(extra)
    return out


THERMAL = {
    "environment": {"airTempC": 21, "meanRadiantTempC": 21, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 350, "diffuseFraction": 0.15, "pressurePa": 101325},
    "operating": [],
    "regions": [
        region("table-top", T("Table top halves, sun-yellowed, heat and ring marks", "Plattenhälften, vergilbt, mit Hitze- und Glasrändern"), "beech-aged", AREA["table-top"], 0.9, "horizontal-plate-up", 0.85, layer("top-board", T("Beech board 22 mm", "Buchenplatte 22 mm"), "timber", "beech-aged", 0.022, 0.17, RHO_BEECH, 1600), ratedPowerW=0, ratedPowerBasis="passive"),
        region("leaf", T("Butterfly leaf, pale (kept in the dark)", "Klappeinlage, hell (im Dunkeln verwahrt)"), "beech-pale", AREA["leaf"], 0.4, "horizontal-plate-up", 0.85, layer("leaf-board", T("Beech board 22 mm", "Buchenplatte 22 mm"), "timber", "beech-pale", 0.022, 0.17, RHO_BEECH, 1600)),
        region("apron", T("Aprons and gap blocks, chair scuffs", "Zargen und Fugenklötze, Stuhlschrammen"), "beech-frame", AREA["apron"], 0.08, "vertical-plate", 0.45, layer("apron-board", T("Beech board 22 mm", "Buchenbrett 22 mm"), "timber", "beech-frame", 0.022, 0.17, RHO_BEECH, 1600)),
        region("legs", T("Tapered legs, one short and loose", "Verjüngte Beine, eines kurz und locker"), "beech-frame", AREA["legs"], 0.055, "cylinder", 0.3, layer("leg-core", T("Beech leg, half thickness 26 mm", "Buchenbein, halbe Dicke 26 mm"), "timber", "beech-frame", 0.026, 0.17, RHO_BEECH, 1600)),
        region("slide", T("Synchronous steel slide", "Stählerner Synchronauszug"), "steel-zinc", AREA["slide"], 0.05, "vertical-plate", 0.2, layer("slide-steel", T("Steel 2 to 8 mm", "Stahl 2 bis 8 mm"), "metal", "steel-zinc", 0.004, 50, RHO_STEEL, 450), source="literature"),
        region("shim", T("Folded cardboard shim", "Gefalteter Pappkeil"), "cardboard", AREA["shim"], 0.1, "horizontal-plate-up", 0.5, layer("shim-board", T("Corrugated board, five layers, 16 mm", "Wellpappe, fünf Lagen, 16 mm"), "polymer", "cardboard", SHIM_T, 0.065, RHO_CARD, 1340)),
    ],
    "emitters": [],
    "assumptions": [
        T("Solid beech after ISO 10456 and the Wood Handbook (720 kg/m3, 0.17 W/mK, 1600 J/kgK); the oiled top has aged under daylight from pale pink to honey (solar absorptance 0.58 against 0.46 of the leaf that spent its life folded in the dark, 0.52 for the frame), emissivity 0.90 for all oiled wood; aged values are authored estimates.",
          "Massive Buche nach ISO 10456 und Wood Handbook (720 kg/m3, 0,17 W/mK, 1600 J/kgK); die geölte Platte ist unter Tageslicht von blassrosa zu honigfarben gealtert (solarer Absorptionsgrad 0,58 gegenüber 0,46 der Einlage, die ihr Leben gefaltet im Dunkeln verbrachte, 0,52 für das Gestell), Emissionsgrad 0,90 für alles geölte Holz; Alterungswerte sind Schätzwerte."),
        T("The steel slide is zinc-plated and dulled (emissivity 0.28); the cardboard shim is kraft corrugated board (160 kg/m3, 0.065 W/mK).", "Der Stahlauszug ist verzinkt und stumpf (Emissionsgrad 0,28); der Pappkeil ist braune Wellpappe (160 kg/m3, 0,065 W/mK)."),
        T("The leaf area is the outer area of the folded leaf of the stored default pose (0.46 m2); unfolded both panels show 0.82 m2, so the extended poses bind it fully exposed.", "Die Fläche der Einlage ist die Außenfläche der gefalteten Einlage in der Lagerpose (0,46 m2); aufgeklappt zeigen beide Hälften 0,82 m2, die ausgezogenen Posen binden sie daher voll exponiert."),
        T("Contacts: shim on the depot floor at 18 degC (0.45 of its area), carriers' hands at 32 degC, forearms at 30 degC (ISO 7730), the cork trivet under the 90 degC casserole at about 45 degC on the leaf; reference case 21 degC with a 350 W/m2 window patch, none in the depot.", "Kontakte: Pappkeil auf dem Lagerboden mit 18 degC (0,45 seiner Fläche), Hände der Tragenden 32 degC, Unterarme 30 degC (ISO 7730), der Korkuntersetzer unter dem 90-degC-Bräter mit etwa 45 degC auf der Einlage; Referenzfall 21 degC mit 350 W/m2 Fensterlichtfleck, im Lager keiner."),
        T("A table draws no power: the top region states 0 W passive for the whole asset.", "Ein Tisch nimmt keine Leistung auf: die Plattenregion nennt 0 W passiv für das ganze Objekt."),
    ],
    "passengers": [],
}

STRUCTURE = {
    "gravityMS2": 9.80665,
    "dynamics": "quasi-static",
    "supports": [],
    "assumptions": [
        T("Each half is a rigid frame of top, U-shaped apron (22 x 80 mm, mortised into the legs) and legs, tied to the top by beech blocks in the 48 mm shadow gap; the halves slide 0.20 m each on a synchronous steel slide whose two 8 x 49 mm centre bars stay in place and run in 2 mm C-rails on the inner apron faces.", "Jede Hälfte ist ein starrer Rahmen aus Platte, U-Zarge (22 x 80 mm, in die Beine gezapft) und Beinen, über Buchenklötze in der 48-mm-Schattenfuge mit der Platte verbunden; die Hälften gleiten je 0,20 m auf einem stählernen Synchronauszug, dessen zwei Mittelstege 8 x 49 mm stehen bleiben und in 2-mm-C-Schienen an den Zargeninnenseiten laufen."),
        T("Supports are pose contacts only: the four feet in the depot and at home (the short right rear leg on its 16 mm shim, so all four bear), the four hands of the two carriers under the end overhangs when carried; no anchor is a permanent support.", "Auflager sind nur Posenkontakte: die vier Füße im Lager und zu Hause (das kurze rechte hintere Bein auf seinem 16-mm-Keil, sodass alle vier tragen), die vier Hände der beiden Tragenden unter den Stirnüberständen beim Tragen; kein Anker ist ein dauerndes Auflager."),
        T("Tableware for six: 0.15 kN/m2 (about 25 kg of plates, glasses, cutlery and dishes over 1.62 m2, authored estimate) plus a 3 kg casserole; the static strength loads of EN 12521 / EN 1730 (vertical 750 N and more) are far above it.", "Geschirr für sechs: 0,15 kN/m2 (etwa 25 kg Teller, Gläser, Besteck und Schüsseln auf 1,62 m2, Schätzwert) und ein 3-kg-Bräter; die statischen Festigkeitslasten von EN 12521 / EN 1730 (senkrecht 750 N und mehr) liegen weit darüber."),
        T("The wobbly leg: a loose wing-nut corner bracket gives 2 degrees of play in the plane of the long side; it is 16 mm short since its split foot was sawn off, so the depot shimmed it. Timber is beech, strength class D40 of EN 338 (f_m,k 40 MPa, E 13 GPa); masses are section x density (beech 720, steel 7850, board 160 kg/m3).", "Das wackelnde Bein: ein lockerer Eckbeschlag mit Flügelmutter gibt 2 Grad Spiel in der Ebene der Längsseite; es ist 16 mm kurz, seit sein gespaltener Fuß abgesägt wurde, daher hat das Lager es unterlegt. Holz ist Buche der Festigkeitsklasse D40 nach EN 338 (f_m,k 40 MPa, E 13 GPa); Massen sind Querschnitt x Dichte (Buche 720, Stahl 7850, Pappe 160 kg/m3)."),
    ],
    "clinical": False,
}


PENS = {"50": {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, "100": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, "200": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, "500": {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
RANGE = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}
HATCHES = {"50": [{"id": "grain-h", "title": T("Beech grain, horizontal", "Buchenmaserung waagerecht"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.13}, {"id": "grain-v", "title": T("Beech grain, vertical", "Buchenmaserung senkrecht"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.5, "penMm": 0.13}]}

DECISIONS = {
    "50": {
        "silhouette": T("Table 28 x 15 mm with its whole construction: the two 22 mm top halves, the 48 mm shadow gap with its beech blocks and the folded butterfly leaf (two pale 22 mm panels), the 80 mm aprons with grain, the four legs tapering from 60 to 45 mm, the short right rear leg on its 16 mm cardboard shim. Front: the right end with top, end blocks, end apron, the two near legs and the shim; the leaf end shows through the gap. Plan: the two halves with their 90 mm staves and the broken corner at the right front.", "Tisch 28 x 15 mm mit seiner ganzen Konstruktion: die zwei 22-mm-Plattenhälften, die 48-mm-Schattenfuge mit ihren Buchenklötzen und der gefalteten Klappeinlage (zwei helle 22-mm-Hälften), die 80-mm-Zargen mit Maserung, die vier von 60 auf 45 mm verjüngten Beine, das kurze rechte hintere Bein auf seinem 16-mm-Pappkeil. Vorderansicht: das rechte Ende mit Platte, Stirnklötzen, Stirnzarge, den zwei vorderen Beinen und dem Keil; durch die Fuge zeigt sich die Stirn der Einlage. Grundriss: die zwei Hälften mit ihren 90-mm-Lamellen und der ausgebrochenen Ecke rechts vorn."),
        "retained": T("Traces of use: two white heat rings from hot pots and a 60 x 40 mm depot price label on the left half, three glass rings and scratches on the right half, the 38 x 28 mm chipped corner, chair scuffs on the right apron, the folds of the shim; the tone contrast between the yellowed top and the pale leaf.", "Gebrauchsspuren: zwei weiße Hitzeringe von heißen Töpfen und ein 60 x 40 mm großes Preisschild des Lagers auf der linken Hälfte, drei Glasränder und Kratzer auf der rechten Hälfte, die 38 x 28 mm ausgebrochene Ecke, Stuhlschrammen an der rechten Zarge, die Falten des Keils; der Tonkontrast zwischen vergilbter Platte und heller Einlage."),
        "merged": T("Near and far legs, aprons and blocks coincide in the side and front views; the far legs are drawn behind so that the swinging loose leg uncovers them.", "Vordere und hintere Beine, Zargen und Klötze fallen in Seiten- und Vorderansicht zusammen; die hinteren Beine sind dahinter gezeichnet, damit das pendelnde lockere Bein sie freigibt."),
        "omitted": T("C-rails behind the aprons (modelled, never visible), the synchronising cord, screws, table locks, dowels, the flush brass table hinges in section, the felt glides of the three sound legs (4 mm).", "C-Schienen hinter den Zargen (modelliert, nie sichtbar), das Gleichlaufseil, Schrauben, Tischverbinder, Dübel, die bündigen Messing-Tischbänder im Schnitt, die Filzgleiter der drei gesunden Beine (4 mm)."),
        "thinElements": T("Top, leaf panels and aprons at their real 22 mm (0.44 mm), the shim at 16 mm (0.32 mm), the slide bars at their real 49 mm height as filled strips.", "Platte, Einlagehälften und Zargen in wahren 22 mm (0,44 mm), der Keil mit 16 mm (0,32 mm), die Auszugsstege in ihrer wahren Höhe von 49 mm als gefüllte Streifen."),
        "hatching": T("Grain lines 0.6 / 0.5 mm apart with a 0.13 mm pen on aprons and legs; staves, rings, label and scratches as 0.13 mm texture strokes.", "Maserungslinien 0,6 / 0,5 mm Abstand mit 0,13-mm-Stift an Zargen und Beinen; Lamellen, Ringe, Etikett und Kratzer als 0,13-mm-Texturstriche."),
        "styles": T("Top 0.62 (yellowed), leaf 0.82 (pale), legs 0.58, aprons 0.55, blocks 0.42 in the shadow, shim 0.48, steel 0.35.", "Platte 0,62 (vergilbt), Einlage 0,82 (hell), Beine 0,58, Zargen 0,55, Klötze 0,42 im Schatten, Keil 0,48, Stahl 0,35."),
        "overlays": T("One shape per part and region; hatched shapes keep their region; texture strokes carry the region of the board they lie on.", "Eine Form je Teil und Region; schraffierte Formen behalten ihre Region; Texturstriche tragen die Region der Platte, auf der sie liegen."),
    },
    "100": {
        "silhouette": T("Table 14 x 7.5 mm: top halves, shadow gap with four side blocks and the end blocks per half, the folded leaf in the gap, aprons, tapered legs, the shim under the short leg; plan with staves, the two heat rings, the label and the chipped corner.", "Tisch 14 x 7,5 mm: Plattenhälften, Schattenfuge mit vier Seitenklötzen und den Stirnklötzen je Hälfte, die gefaltete Einlage in der Fuge, Zargen, verjüngte Beine, der Keil unter dem kurzen Bein; Grundriss mit Lamellen, den zwei Hitzeringen, dem Etikett und der ausgebrochenen Ecke."),
        "retained": T("The yellowed top against the pale leaf, the 0.16 mm shim, the 0.38 mm chip, the heat rings (2 mm across) and the price label.", "Die vergilbte Platte gegen die helle Einlage, der 0,16-mm-Keil, die 0,38-mm-Ausbruchstelle, die Hitzeringe (2 mm Durchmesser) und das Preisschild."),
        "merged": T("The 2 mm C-rails are merged into the slide bars, drawn 15 mm thick in the rail slot.", "Die 2-mm-C-Schienen sind in die Auszugsstege aufgenommen, 15 mm dick im Schienenschlitz gezeichnet."),
        "omitted": T("Grain hatching, glass rings, scratches, chair scuffs, the folds of the shim and the hinge plates.", "Maserungsschraffur, Glasränder, Kratzer, Stuhlschrammen, die Falten des Keils und die Bandplatten."),
        "thinElements": T("Top, leaf panels and aprons at 22 mm (0.22 mm), the shim at 16 mm.", "Platte, Einlagehälften und Zargen mit 22 mm (0,22 mm), der Keil mit 16 mm."),
        "hatching": T("No hatching; staves, heat rings and label as 0.13 mm texture strokes.", "Keine Schraffur; Lamellen, Hitzeringe und Etikett als 0,13-mm-Texturstriche."),
        "styles": T("Tones as 1:50.", "Töne wie 1:50."),
        "overlays": T("One shape per part and region.", "Eine Form je Teil und Region."),
    },
    "200": {
        "silhouette": T("Table 7 x 3.75 mm: 30 mm top slabs (0.15 mm), the shadow gap with one side block per long side and half and the two end blocks, the folded leaf as two 30 mm panels, 30 mm aprons, tapered legs and the 30 mm shim block under the short leg.", "Tisch 7 x 3,75 mm: 30-mm-Plattenscheiben (0,15 mm), die Schattenfuge mit einem Seitenklotz je Längsseite und Hälfte und den zwei Stirnklötzen, die gefaltete Einlage als zwei 30-mm-Hälften, 30-mm-Zargen, verjüngte Beine und der 30-mm-Keilblock unter dem kurzen Bein."),
        "retained": T("The gap, the leaf in it, the taper of the legs and the shim; the pale leaf tone.", "Die Fuge, die Einlage darin, die Verjüngung der Beine und der Keil; der helle Ton der Einlage."),
        "merged": T("Two side blocks per long side become one; the rails are in the bars, the bars 30 mm thick; top, leaf panels, aprons and shim are drawn at the 30 mm feature.", "Zwei Seitenklötze je Längsseite werden einer; die Schienen stecken in den Stegen, die Stege 30 mm dick; Platte, Einlagehälften, Zargen und Keil sind mit dem 30-mm-Merkmal gezeichnet."),
        "omitted": T("All traces of use below 30 mm: rings, label, staves, scratches and the chipped corner.", "Alle Gebrauchsspuren unter 30 mm: Ringe, Etikett, Lamellen, Kratzer und die ausgebrochene Ecke."),
        "thinElements": T("Boards at the 30 mm feature (0.15 mm).", "Bretter mit dem 30-mm-Merkmal (0,15 mm)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Tones as 1:50.", "Töne wie 1:50."),
        "overlays": T("One shape per part and region.", "Eine Form je Teil und Region."),
    },
    "500": {
        "silhouette": T("Table 2.8 x 1.5 mm (3.6 mm extended): two 30 mm top slabs, 75 mm square legs running up to the slabs, the 80 mm aprons between them, the folded leaf in the gap and the shim as a 30 mm block under the short leg.", "Tisch 2,8 x 1,5 mm (3,6 mm ausgezogen): zwei 30-mm-Plattenscheiben, 75-mm-Vierkantbeine bis unter die Scheiben, die 80-mm-Zargen dazwischen, die gefaltete Einlage in der Fuge und der Keil als 30-mm-Block unter dem kurzen Bein."),
        "retained": T("Top, gap, aprons, four legs, the pale leaf and the shim; extended and closed lengths read.", "Platte, Fuge, Zargen, vier Beine, die helle Einlage und der Keil; ausgezogene und geschlossene Länge sind lesbar."),
        "merged": T("The gap blocks are left out: the legs, thickened to the 75 mm feature, carry the slabs; taper, rails and bar detail merged.", "Die Fugenklötze entfallen: die auf das 75-mm-Merkmal verdickten Beine tragen die Scheiben; Verjüngung, Schienen und Stegdetails zusammengefasst."),
        "omitted": T("Every trace of use, the chip, staves and grain.", "Alle Gebrauchsspuren, die Ausbruchstelle, Lamellen und Maserung."),
        "thinElements": T("Slabs, leaf panels, aprons and shim at 30 mm (0.06 mm) as filled strips joined to the legs.", "Scheiben, Einlagehälften, Zargen und Keil mit 30 mm (0,06 mm) als gefüllte Streifen an den Beinen."),
        "hatching": T("None.", "Keine."),
        "styles": T("Tones as 1:50.", "Töne wie 1:50."),
        "overlays": T("One shape per part and region.", "Eine Form je Teil und Region."),
    },
}

VOLUME_DECISIONS = {
    "50": [T("Body for 7.5 mm: two 0.70 x 0.90 m solid beech halves 22 mm thick, the right one with the broken corner (38 x 28 mm, full thickness); per half a U-apron 22 x 80 mm tenoned into the legs, four side blocks and two end blocks 48 mm tall in the shadow gap; four legs tapering on all faces from 60 mm at the frame to 45 mm at the floor, the right rear one 16 mm short on its own bone and standing on the folded 150 x 100 x 16 mm cardboard shim; per side a 2 mm zinc-plated C-rail 55 x 12 mm on the inner apron face of each half and the stationary 8 x 49 mm centre bar running in both; the butterfly leaf as two pale 0.20 x 0.90 m panels folded face to face on the left half's aprons, the folding panel on a hinge bone at the flush table hinges. Felt glides, screws, dowels, locks and the synchronising cord are below 7.5 mm or hidden and left out.", "Körper für 7,5 mm: zwei Hälften 0,70 x 0,90 m aus massiver Buche, 22 mm stark, die rechte mit der ausgebrochenen Ecke (38 x 28 mm, ganze Dicke); je Hälfte eine U-Zarge 22 x 80 mm, in die Beine gezapft, vier Seitenklötze und zwei Stirnklötze 48 mm hoch in der Schattenfuge; vier allseitig von 60 mm an der Zarge auf 45 mm am Boden verjüngte Beine, das rechte hintere 16 mm kurz auf einem eigenen Knochen und auf dem gefalteten Pappkeil 150 x 100 x 16 mm stehend; je Seite eine verzinkte 2-mm-C-Schiene 55 x 12 mm an der Zargeninnenseite jeder Hälfte und der feststehende Mittelsteg 8 x 49 mm, der in beiden läuft; die Klappeinlage als zwei helle Hälften 0,20 x 0,90 m, Gesicht an Gesicht gefaltet auf den Zargen der linken Hälfte, die Klapphälfte auf einem Gelenkknochen an den bündigen Tischbändern. Filzgleiter, Schrauben, Dübel, Verbinder und das Gleichlaufseil liegen unter 7,5 mm oder verdeckt und entfallen.")],
    "100": [T("Body for 15 mm: the parts of 1:50 that reach 15 mm keep their true sections (22 mm boards, 16 mm shim, 60/45 mm legs, the chipped corner); the 2 mm C-rails are below the feature and merged into the stationary bars, which fill the rail slot at 15 x 55 mm.", "Körper für 15 mm: die Teile von 1:50, die 15 mm erreichen, behalten ihre wahren Querschnitte (22-mm-Bretter, 16-mm-Keil, 60/45-mm-Beine, die ausgebrochene Ecke); die 2-mm-C-Schienen liegen unter dem Merkmal und sind in die feststehenden Stege aufgenommen, die den Schienenschlitz mit 15 x 55 mm füllen.")],
    "200": [T("Body for 30 mm: top halves, leaf panels, aprons and the shim are 30 mm thick (the frame drops 7.5 mm so that the folded leaf still lies in the gap and rises flush), one side block per long side and half and two end blocks per half, legs keep their 60 to 45 mm taper, the bars are 30 x 55 mm in the apron slot; the chipped corner is below the feature and left out.", "Körper für 30 mm: Plattenhälften, Einlagehälften, Zargen und Keil sind 30 mm stark (das Gestell sinkt um 7,5 mm, damit die gefaltete Einlage weiter in der Fuge liegt und bündig hochkommt), ein Seitenklotz je Längsseite und Hälfte und zwei Stirnklötze je Hälfte, die Beine behalten ihre Verjüngung 60 auf 45 mm, die Stege sind 30 x 55 mm im Zargenschlitz; die ausgebrochene Ecke liegt unter dem Merkmal und entfällt.")],
    "500": [T("Body for 75 mm: 30 mm top slabs, leaf panels, aprons and shim as drawn (the thinnest 2D strip of each bone), the legs thickened to 75 mm square and run up to the slab underside so that they carry the slabs without gap blocks, the bars 30 x 55 mm; everything else merged.", "Körper für 75 mm: 30-mm-Plattenscheiben, Einlagehälften, Zargen und Keil wie gezeichnet (der dünnste 2D-Streifen jedes Knochens), die Beine auf 75 mm im Quadrat verdickt und bis unter die Scheiben geführt, sodass sie die Scheiben ohne Fugenklötze tragen, die Stege 30 x 55 mm; alles andere zusammengefasst.")],
}


def representation(s):
    """🗺️ The authored representation of scale s."""
    return {"status": "authored", "denominator": int(s), "validRange": {"minDenominator": RANGE[s][0], "maxDenominator": RANGE[s][1]}, "pens": PENS[s], "hatches": HATCHES.get(s, []), "geometry": {"elevation": elevation(s), "front": front(s), "plan": plan(s)}, "decisions": DECISIONS[s]}


def volume(s):
    """🧊 The authored volume of scale s."""
    b = body(s)
    th = thermal(s, b)
    st = structure(s, b)
    return {"status": "authored", "denominator": int(s), "body": b, "thermal": th, "structure": st, "decisions": VOLUME_DECISIONS[s] + [T("Thermal: every body surface is the surface of its region (references); structure: the boards, legs, rails, bars, leaf panels and shim are members, the gap blocks joints, one mass marker per loaded bone at its centre.", "Thermik: jede Körperoberfläche ist die Oberfläche ihrer Region (Verweise); Tragwerk: Bretter, Beine, Schienen, Stege, Einlagehälften und Keil sind Stäbe, die Fugenklötze Knoten, je belastetem Knochen eine Massenmarke in seinem Schwerpunkt.")]}


SYMBOL = {"id": "used-extendable-dining-table-symbol", "title": T("Table symbol, true extent", "Tischsymbol, wahre Ausdehnung"), "description": T("The closed extending table in all three views: top 1.40 x 0.90 m at 0.75 m, the 0.40 m leaf opening dashed, four legs.", "Der geschlossene Ausziehtisch in allen drei Ansichten: Platte 1,40 x 0,90 m auf 0,75 m, die 0,40-m-Einlageöffnung gestrichelt, vier Beine."), "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, "hatches": [],
          "geometry": {"elevation": [{"id": "top", "bone": "ground", "role": "body", "z": 3, "filled": True, "tone": 0.85, "kind": "rect", "x": 0, "y": 0.72, "w": L, "h": 0.03}, {"id": "leg-l", "bone": "ground", "role": "body", "z": 2, "filled": True, "tone": 0.85, "kind": "rect", "x": 0.07, "y": 0, "w": 0.06, "h": 0.72}, {"id": "leg-r", "bone": "ground", "role": "body", "z": 2, "filled": True, "tone": 0.85, "kind": "rect", "x": 1.27, "y": 0, "w": 0.06, "h": 0.72}, {"id": "apron", "bone": "ground", "role": "detail", "z": 2, "filled": False, "tone": 0.5, "kind": "rect", "x": 0.13, "y": 0.60, "w": 1.14, "h": 0.08}],
                       "front": [{"id": "top", "bone": "ground", "role": "body", "z": 3, "filled": True, "tone": 0.85, "kind": "rect", "x": -0.45, "y": 0.72, "w": 0.9, "h": 0.03}, {"id": "leg-l", "bone": "ground", "role": "body", "z": 2, "filled": True, "tone": 0.85, "kind": "rect", "x": 0.34, "y": 0, "w": 0.06, "h": 0.72}, {"id": "leg-r", "bone": "ground", "role": "body", "z": 2, "filled": True, "tone": 0.85, "kind": "rect", "x": -0.40, "y": 0, "w": 0.06, "h": 0.72}],
                       "plan": [{"id": "plan-top", "bone": "ground", "role": "body", "z": 1, "filled": True, "tone": 0.85, "kind": "rect", "x": 0, "y": -0.45, "w": L, "h": W}, {"id": "plan-joint", "bone": "ground", "role": "detail", "z": 2, "filled": False, "tone": 0.5, "kind": "poly", "points": [[0.70, -0.45], [0.70, 0.45]], "closed": False}]},
          "dimensionalNote": T("Model metres at real size in the frame of the asset (x ahead, lateral positive to the left, y up), closed; the joint line marks where the halves part for the 0.40 m leaf (1.80 m extended).", "Modellmeter in wahrer Größe im Koordinatensystem des Objekts (x nach vorn, seitlich positiv nach links, y nach oben), geschlossen; die Fugenlinie zeigt, wo sich die Hälften für die 0,40-m-Einlage öffnen (1,80 m ausgezogen).")}

REFERENCES = [
    {"id": "din-18011-2006", "citation": "DIN 18011:2006-07 Stellflächen, Abstände und Bewegungsflächen im Wohnungsbau (dining place 0.60 x 0.40 m per person, table heights about 0.72 to 0.76 m)."},
    {"id": "en-12521-2015", "citation": "EN 12521:2015 Furniture - Strength, durability and safety - Requirements for domestic tables (static vertical and horizontal loads, stability)."},
    {"id": "en-1730-2012", "citation": "EN 1730:2012 Furniture - Tables - Test methods for the determination of stability, strength and durability."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (hardwood class D40: f_m,k 40 MPa, E0,mean 13 GPa, rho_k 660 kg/m3)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: densities, self-weight, imposed loads for buildings (Annex A densities)."},
    {"id": "en-1993-1-1", "citation": "EN 1993-1-1:2005 Eurocode 3: Design of steel structures - Part 1-1 (E = 210 GPa, S235, density 7850 kg/m3)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (timber 700 kg/m3 0.18 W/mK 1600 J/kgK, steel 50 W/mK, paper and board)."},
    {"id": "iso-7730-2005", "citation": "ISO 7730:2005 Ergonomics of the thermal environment (skin and clothing surface temperatures of seated persons 30 to 34 degC)."},
    {"id": "wood-handbook-2021", "citation": "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. FPL-GTR-282, USDA Forest Service (European beech density about 720 kg/m3 at 12 % moisture, darkening of finishes under light)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities of oiled wood 0.90, dull zinc-plated steel about 0.25 to 0.3, paper 0.9)."},
    {"id": "catalogue-beech-extending-tables", "citation": "Catalogue-typical data of German mid-range solid beech extending tables of the 1990s and 2000s (140-180 x 90 x 75 cm with a self-storing folding leaf on a synchronous slide, tops 20-25 mm, aprons 20-22 x 80-90 mm, legs 55-65 mm square, 40-55 kg), as listed in mail-order and furniture-store catalogues and second-hand offers; types, no brands."},
]

PROVENANCE = {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
              "notes": T(f"Second-hand solid beech extending table of a German mid-range type (late 1990s), 1.40 x 0.90 x 0.75 m closed and 1.80 m extended with a self-storing butterfly leaf of two 0.20 m panels. Each half: 22 mm top of 90 mm glued staves over a 48 mm shadow gap, a U-apron 22 x 80 mm and two legs tapering from 60 to 45 mm (knee clearance 0.60 m); the halves run 0.20 m each on a synchronous zinc-plated steel slide. Traces of its life: the top yellowed by daylight while the leaf stayed pale, two white heat rings and three glass rings, scratches, a depot price label, a 38 x 28 mm broken corner, chair scuffs on an apron, and a wobbly right rear leg (loose wing-nut bracket, split foot sawn off 16 mm short) that the reuse depot shimmed with folded cardboard. Mass {MASS_KG:.1f} kg from the modelled sections x density (beech 720, steel 7850, board 160 kg/m3).",
                         f"Gebrauchter Ausziehtisch aus massiver Buche eines deutschen Mittelklassetyps (späte 1990er), 1,40 x 0,90 x 0,75 m geschlossen und 1,80 m ausgezogen mit einer selbstverstauenden Klappeinlage aus zwei 0,20-m-Hälften. Jede Hälfte: 22-mm-Platte aus 90-mm-Lamellen über einer 48-mm-Schattenfuge, eine U-Zarge 22 x 80 mm und zwei von 60 auf 45 mm verjüngte Beine (Beinfreiheit 0,60 m); die Hälften laufen je 0,20 m auf einem verzinkten stählernen Synchronauszug. Spuren seines Lebens: die Platte vom Tageslicht vergilbt, während die Einlage hell blieb, zwei weiße Hitzeringe und drei Glasränder, Kratzer, ein Preisschild des Lagers, eine 38 x 28 mm ausgebrochene Ecke, Stuhlschrammen an einer Zarge und ein wackelndes rechtes hinteres Bein (lockerer Flügelmutterbeschlag, gespaltener Fuß 16 mm kürzer abgesägt), das das Gebrauchtwarenlager mit gefalteter Pappe unterlegt hat. Masse {MASS_KG:.1f} kg aus den modellierten Querschnitten x Dichte (Buche 720, Stahl 7850, Pappe 160 kg/m3).".replace(".", ",", 0))}


def document():
    """📄 The whole asset document."""
    asset = {
        "id": SLUG,
        "category": "furniture",
        "title": T("Used Extendable Dining Table", "Gebrauchter Ausziehtisch"),
        "summary": T("A second-hand solid beech extending table, 1.40 m closed and 1.80 m with its self-storing butterfly leaf, with a yellowed top, heat and glass rings, a chipped corner and a wobbly short leg on a cardboard shim; stored, carried by two people, extended and laid for six at home.",
                     "Ein gebrauchter Ausziehtisch aus massiver Buche, 1,40 m geschlossen und 1,80 m mit seiner selbstverstauenden Klappeinlage, mit vergilbter Platte, Hitze- und Glasrändern, ausgebrochener Ecke und einem wackelnden kurzen Bein auf einem Pappkeil; abgestellt, von zwei Personen getragen, ausgezogen und zu Hause für sechs gedeckt."),
        "tags": ["used", "household", "second-hand", "living", "kitchen", "table", "dining", "extendable", "butterfly-leaf", "timber", "beech", "furniture"],
        "provenance": PROVENANCE,
        "dimensions": {"heightM": H, "widthM": L, "depthM": W, "massKg": MASS_KG},
        "materials": MATERIALS,
        "rig": RIG,
        "volumes": {s: volume(s) for s in SCALES},
        "anchors": ANCHORS,
        "representations": {s: representation(s) for s in SCALES},
        "symbols": [SYMBOL],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": THERMAL,
        "structure": STRUCTURE,
        "presentation": {"defaultPose": "stored", "defaultClip": "extend", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    (HERE / "🔣️.json").write_text(json.dumps(document(), ensure_ascii=False, indent=1) + "\n")
    print(f"[DEBUG] wrote {HERE / '🔣️.json'} mass {MASS_KG} kg, bones {MASS}")
