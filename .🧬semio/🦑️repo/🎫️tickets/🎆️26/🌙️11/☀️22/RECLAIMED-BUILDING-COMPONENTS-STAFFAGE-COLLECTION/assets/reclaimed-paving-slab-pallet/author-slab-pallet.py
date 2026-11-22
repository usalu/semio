"""🧱 Authoritative generator of the staffage asset `reclaimed-paving-slab-pallet`: every hand-chosen number of the reclaimed
50 x 50 x 5 cm concrete paving slabs on their EUR pallet is stated here, and the asset document `🔣️.json` next to this script is
written from them (rig, anchors, the 2D drawings of four scales, the 3D volumes of four scales, poses, clips, thermal and structure).

Run: `python3 author-slab-pallet.py` (writes `🔣️.json` in this folder).

@see ../../brief.md — the binding brief of the collection
@see ../../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md — the tools that prove it
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SLUG = "reclaimed-paving-slab-pallet"

EDGE = 0.5
THICK = 0.05
RHO_SLAB = 2300.0
RHO_TIMBER = 520.0
RHO_PET = 1380.0
DECK_TOP = 0.144
PALLET_Y = 0.133
PALLET_L = 1.2
PALLET_W = 0.8
PACK_X0 = {"rear": 0.025, "front": 0.625}
PACK_N = 11
BAND_W = 0.016
BAND_T = 0.0009
BAND_H = 0.30
COL_B_Z = 0.2525
JOINT = 0.005
FIELD_X = [1.5, 1.5 + EDGE + JOINT, 1.5 + 2 * (EDGE + JOINT)]
LIFT = 0.40
WHEEL_N = 2500.0
WHEEL_X0 = 1.5
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
SCALES = [50, 100, 200, 500]

BOARD_Z = [(-0.4, -0.255), (-0.21375, -0.11375), (-0.0725, 0.0725), (0.11375, 0.21375), (0.255, 0.4)]
ROW_X = [(0.0, 0.145), (0.5275, 0.6725), (1.055, 1.2)]
ROW_Z = [(-0.4, -0.3), (-0.0725, 0.0725), (0.3, 0.4)]
ROW_NAME = ["rear", "middle", "front"]
SIDE_NAME = ["right", "centre", "left"]

CHIPS = {
    "rear-00": [("TL", 0.035, 0.03), ("BR", 0.025, 0.02)],
    "rear-01": [("TR", 0.04, 0.035)],
    "rear-02": [("TL", 0.03, 0.045), ("TR", 0.025, 0.025)],
    "rear-03": [("TR", 0.03, 0.03)],
    "rear-04": [],
    "rear-05": [("TL", 0.045, 0.035)],
    "rear-06": [("BL", 0.03, 0.025)],
    "rear-07": [("TR", 0.035, 0.04), ("TL", 0.025, 0.02)],
    "rear-08": [],
    "rear-09": [("TL", 0.04, 0.03)],
    "rear-10": [("TR", 0.03, 0.035), ("BR", 0.03, 0.02)],
    "front-00": [("TL", 0.03, 0.035)],
    "front-01": [],
    "front-02": [("TR", 0.04, 0.03), ("BL", 0.025, 0.025)],
    "front-03": [("TL", 0.035, 0.03)],
    "front-04": [("TR", 0.025, 0.03)],
    "front-05": [],
    "front-06": [("TL", 0.04, 0.04)],
    "front-07": [("TR", 0.035, 0.03), ("TL", 0.03, 0.025)],
    "front-08": [("TR", 0.045, 0.035)],
    "front-09": [("TL", 0.03, 0.03), ("BR", 0.025, 0.025)],
    "front-10": [("TR", 0.04, 0.045), ("TL", 0.035, 0.03), ("BL", 0.03, 0.02)],
}

FIELD = {
    "slab-f1": {"pack": "front", "index": 8, "column": "a", "cell": 0},
    "slab-f2": {"pack": "front", "index": 9, "column": "a", "cell": 1},
    "slab-f3": {"pack": "front", "index": 10, "column": "a", "cell": 2},
    "slab-r3": {"pack": "rear", "index": 2, "column": "b", "cell": 0},
    "slab-r2": {"pack": "rear", "index": 1, "column": "b", "cell": 1},
    "slab-r1": {"pack": "rear", "index": 0, "column": "b", "cell": 2},
}
FIELD_ORDER = ["slab-r1", "slab-r2", "slab-r3", "slab-f1", "slab-f2", "slab-f3"]
STOCK = {"rear": list(range(3, 11)), "front": list(range(0, 8))}


def T(en, de):
    """🗣️ A bilingual text, English first, German second."""
    return {"en": en, "de": de}


def r6(value):
    """🎯 A number rounded to the micrometre so the document stays readable and deterministic."""
    return round(value + 0.0, 6)


def pt(points):
    """📍 A rounded point list."""
    return [[r6(a), r6(b)] for a, b in points]


def mm(metres, denominator):
    """📏 Paper millimetres of a real length at a scale, as English text."""
    return f"{metres * 1000 / denominator:.2f}".rstrip("0").rstrip(".")


def mmde(metres, denominator):
    """📏 Paper millimetres of a real length at a scale, as German text (decimal comma)."""
    return mm(metres, denominator).replace(".", ",")


def slab_x(pack, index):
    """↔️ The forward range of slab `index` of a pack in world metres."""
    x0 = PACK_X0[pack] + index * THICK
    return x0, x0 + THICK


def chip_area(chips):
    """✂️ The face area the chips of a slab take away (right triangles of the two legs)."""
    return sum(a * b / 2 for _, a, b in chips)


def ccw(ring):
    """🔄 The ring in counter-clockwise order (positive shoelace area)."""
    area = sum(ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1] for i in range(len(ring)))
    return ring if area > 0 else list(reversed(ring))


def face_profile(chips, w0, w1):
    """🧩 The chipped face of a slab as a ring of (height u, lateral w); corners B/T bottom/top, R/L right (low w) / left (high w), each chip a right triangle of legs a (along w) and b (along u)."""
    by = {corner: (a, b) for corner, a, b in chips}
    out = []
    a, b = by.get("BR", (0, 0))
    out += [(b, w0), (0.0, w0 + a)] if "BR" in by else [(0.0, w0)]
    a, b = by.get("BL", (0, 0))
    out += [(0.0, w1 - a), (b, w1)] if "BL" in by else [(0.0, w1)]
    a, b = by.get("TL", (0, 0))
    out += [(EDGE - b, w1), (EDGE, w1 - a)] if "TL" in by else [(EDGE, w1)]
    a, b = by.get("TR", (0, 0))
    out += [(EDGE, w0 + a), (EDGE - b, w0)] if "TR" in by else [(EDGE, w0)]
    return ccw(out)


SLAB_MASS = {}
for pack in ("rear", "front"):
    for i in range(PACK_N):
        key = f"{pack}-{i:02d}"
        SLAB_MASS[key] = (EDGE * EDGE - chip_area(CHIPS[key])) * THICK * RHO_SLAB


def field_key(bone):
    """🔑 The slab key of a field slab bone."""
    f = FIELD[bone]
    return f"{f['pack']}-{f['index']:02d}"


def field_origin_x(bone):
    """📌 The rest forward position of the pivot of a field slab: its front face at the deck."""
    f = FIELD[bone]
    return slab_x(f["pack"], f["index"])[1]


def field_travel(bone):
    """🛤️ The carrier travel (dx, dy in the pallet frame) that brings a field slab's pivot to its place in the 2 x 3 field."""
    f = FIELD[bone]
    target = FIELD_X[f["cell"]] if f["column"] == "a" else FIELD_X[f["cell"]] + EDGE
    return target - field_origin_x(bone), -DECK_TOP


def field_w(bone):
    """↕️ The lateral range of a field slab in its own bone frame (the pivot of column b lies at z 0.2525)."""
    zb = 0.0 if FIELD[bone]["column"] == "a" else COL_B_Z
    return -0.25 - zb, 0.25 - zb


PALLET_PARTS_50 = []
for k, (z0, z1) in enumerate(ROW_Z):
    PALLET_PARTS_50.append({"id": f"bottom-board-{SIDE_NAME[k]}", "kind": "bottom", "x": (0.0, PALLET_L), "y": (0.0, 0.022), "z": (z0, z1)})
for i, (x0, x1) in enumerate(ROW_X):
    for k, (z0, z1) in enumerate(ROW_Z):
        PALLET_PARTS_50.append({"id": f"block-{ROW_NAME[i]}-{SIDE_NAME[k]}", "kind": "block", "x": (x0, x1), "y": (0.022, 0.1), "z": (z0, z1)})
for i, (x0, x1) in enumerate(ROW_X):
    PALLET_PARTS_50.append({"id": f"cross-board-{ROW_NAME[i]}", "kind": "cross", "x": (x0, x1), "y": (0.1, 0.122), "z": (-0.4, 0.4)})
DECK_NAMES = ["deck-board-right", "deck-board-right-inner", "deck-board-centre", "deck-board-left-inner", "deck-board-left"]
for k, (z0, z1) in enumerate(BOARD_Z):
    PALLET_PARTS_50.append({"id": DECK_NAMES[k], "kind": "deck", "x": (0.0, PALLET_L), "y": (0.122, DECK_TOP), "z": (z0, z1)})


def vol(part):
    """📦 Volume of a world box part."""
    return (part["x"][1] - part["x"][0]) * (part["y"][1] - part["y"][0]) * (part["z"][1] - part["z"][0])


PALLET_VOLUME = sum(vol(p) for p in PALLET_PARTS_50)
PALLET_MASS = PALLET_VOLUME * RHO_TIMBER
PALLET_COM_Y = sum(vol(p) * (p["y"][0] + p["y"][1]) / 2 for p in PALLET_PARTS_50) / PALLET_VOLUME
BAND_LEN = 2 * ((PACK_N * THICK + 2 * BAND_T) + (EDGE + 2 * BAND_T))
BAND_MASS = BAND_LEN * BAND_W * BAND_T * RHO_PET
STOCK_MASS = {pack: sum(SLAB_MASS[f"{pack}-{i:02d}"] for i in STOCK[pack]) for pack in STOCK}
FIELD_MASS = {bone: SLAB_MASS[field_key(bone)] for bone in FIELD}
TOTAL_MASS = PALLET_MASS + sum(STOCK_MASS.values()) + sum(FIELD_MASS.values()) + 2 * BAND_MASS


def box_local_pallet(x, y, z):
    """🪵 A world box as a `box` solid on the pallet bone (origin at the deck mid height, axes along the world)."""
    return {"size": [r6(x[1] - x[0]), r6(y[1] - y[0]), r6(z[1] - z[0])], "transform": {"translation": [r6((x[0] + x[1]) / 2), r6((y[0] + y[1]) / 2 - PALLET_Y), r6((z[0] + z[1]) / 2)]}}


def solid(sid, bone, material, tone, geometry, **extra):
    """🧊 One volume solid."""
    out = {"id": sid, "bone": bone, "material": material, "role": "body", "tone": tone}
    out.update(geometry)
    out.update(extra)
    return out


def box(sid, bone, material, tone, size, centre, **extra):
    """📦 A box solid at a bone-local centre."""
    return solid(sid, bone, material, tone, {"kind": "box", "size": [r6(s) for s in size], "transform": {"translation": [r6(c) for c in centre]}}, **extra)


def slab_extrude(sid, bone, tone, chips, w0, w1, v0, v1, **extra):
    """🧱 A slab on a vertical bone: its face (height u, lateral w) with chipped corners extruded along v (plane zx: profile (w, u))."""
    ring = face_profile(chips, w0, w1)
    profile = ccw([(w, u) for u, w in ring])
    return solid(sid, bone, "concrete-slab", tone, {"kind": "extrude", "profile": pt(profile), "plane": "zx", "from": r6(v0), "to": r6(v1)}, **extra)


def stock_v(pack, index):
    """↕️ The v range (bone normal, pointing to the rear) of stock slab `index` on its pack bone."""
    xo = slab_x(pack, STOCK[pack][-1])[1]
    x0, x1 = slab_x(pack, index)
    return xo - x1, xo - x0


def tone_of(key):
    """🎨 A shade per slab: weathered concrete between 0.52 and 0.64, never two neighbours alike."""
    pack, i = key.split("-")
    return round(0.52 + ((int(i) * 7 + (3 if pack == "front" else 0)) % 6) * 0.024, 3)


def pallet_body(scale):
    """🪵 The pallet of one scale as body solids on the pallet bone."""
    out = []
    if scale == 50:
        for p in PALLET_PARTS_50:
            tone = {"bottom": 0.4, "block": 0.36, "cross": 0.44, "deck": 0.48}[p["kind"]]
            out.append(solid(p["id"], "pallet", "pallet-timber", tone, {"kind": "box", **box_local_pallet(p["x"], p["y"], p["z"])}))
    elif scale == 100:
        for p in PALLET_PARTS_50:
            if p["kind"] != "deck":
                tone = {"bottom": 0.4, "block": 0.36, "cross": 0.44}[p["kind"]]
                out.append(solid(p["id"], "pallet", "pallet-timber", tone, {"kind": "box", **box_local_pallet(p["x"], p["y"], p["z"])}))
        out.append(solid("deck-plate", "pallet", "pallet-timber", 0.48, {"kind": "box", **box_local_pallet((0.0, PALLET_L), (0.122, DECK_TOP), (-0.4, 0.4))}))
    elif scale == 200:
        out.append(solid("upper-plate", "pallet", "pallet-timber", 0.46, {"kind": "box", **box_local_pallet((0.0, PALLET_L), (0.1, DECK_TOP), (-0.4, 0.4))}))
        for i, (x0, x1) in enumerate(ROW_X):
            for k, (z0, z1) in enumerate(ROW_Z):
                out.append(solid(f"foot-{ROW_NAME[i]}-{SIDE_NAME[k]}", "pallet", "pallet-timber", 0.38, {"kind": "box", **box_local_pallet((x0, x1), (0.0, 0.1), (z0, z1))}))
    else:
        out.append(solid("pallet-mass", "pallet", "pallet-timber", 0.42, {"kind": "extrude", "profile": pt(pallet_front_profile_local()), "plane": "yz", "from": 0.0, "to": PALLET_L}))
    return out


def pallet_front_profile_world():
    """🔲 The front outline of the pallet with its two fork openings (lateral z, height y)."""
    return [(-0.4, 0.0), (-0.3, 0.0), (-0.3, 0.1), (-0.0725, 0.1), (-0.0725, 0.0), (0.0725, 0.0), (0.0725, 0.1), (0.3, 0.1), (0.3, 0.0), (0.4, 0.0), (0.4, DECK_TOP), (-0.4, DECK_TOP)]


def pallet_front_profile_local():
    """🔲 The front outline as a (v, w) profile of the pallet bone (plane yz)."""
    return ccw([(y - PALLET_Y, z) for z, y in pallet_front_profile_world()])


def slab_bodies(scale):
    """🧱 The slabs of one scale: stock slabs on the two pack bones and the six field slabs on their own bones."""
    out = []
    for pack in ("rear", "front"):
        bone = f"pack-{pack}"
        if scale == 50:
            for i in STOCK[pack]:
                key = f"{pack}-{i:02d}"
                v0, v1 = stock_v(pack, i)
                out.append(slab_extrude(f"slab-{key}", bone, tone_of(key), CHIPS[key], -0.25, 0.25, v0, v1))
        elif scale == 100:
            for i in STOCK[pack]:
                key = f"{pack}-{i:02d}"
                v0, v1 = stock_v(pack, i)
                out.append(box(f"slab-{key}", bone, "concrete-slab", tone_of(key), [EDGE, THICK, EDGE], [EDGE / 2, (v0 + v1) / 2, 0.0]))
        else:
            n = len(STOCK[pack])
            out.append(box(f"stock-{pack}", bone, "concrete-slab", 0.56, [EDGE, n * THICK, EDGE], [EDGE / 2, n * THICK / 2, 0.0]))
    for bone in FIELD_ORDER:
        key = field_key(bone)
        w0, w1 = field_w(bone)
        if scale == 50:
            out.append(slab_extrude(f"{bone}-body", bone, tone_of(key), CHIPS[key], w0, w1, 0.0, THICK))
        else:
            t = THICK if scale < 500 else FEATURE[500]
            out.append(box(f"{bone}-body", bone, "concrete-slab", 0.6, [EDGE, t, EDGE], [EDGE / 2, t / 2, (w0 + w1) / 2]))
    return out


def band_bars(pack, prefix):
    """🎗️ The four bars of one strapping band around a pack, slender PET strips 16 x 0.9 mm touching the slab faces."""
    bone = f"band-{pack}"
    length = PACK_N * THICK
    lx = length + 2 * BAND_T
    lz = EDGE + 2 * BAND_T
    out = []
    for side, zc in (("right", -0.25 - BAND_T / 2), ("left", 0.25 + BAND_T / 2)):
        out.append(box(f"{prefix}{pack}-{side}", bone, "pet-strap", 0.22, [lx, BAND_W, BAND_T], [length / 2, BAND_W / 2, zc], slender=True))
    for end, xc in (("rear", -BAND_T / 2), ("front", length + BAND_T / 2)):
        out.append(box(f"{prefix}{pack}-{end}", bone, "pet-strap", 0.22, [BAND_T, BAND_W, lz], [xc, BAND_W / 2, 0.0], slender=True))
    return out


def body_list(scale):
    """🧊 The whole body list of one scale."""
    out = pallet_body(scale) + slab_bodies(scale)
    if scale < 500:
        for pack in ("rear", "front"):
            out += band_bars(pack, "band-")
    return out


def region_of(s):
    """🌡️ The thermal region of a body solid."""
    if s["material"] == "pallet-timber":
        return "pallet-timber"
    if s["material"] == "pet-strap":
        return "strapping"
    return "slab-field" if s["bone"] in FIELD else "slab-stock"


def thermal_list(scale, body):
    """🌡️ The thermal list of one scale: every body solid by reference, plus the bands as their own slender bars at 1:500."""
    out = [{"id": f"th-{s['id']}", "of": s["id"], "region": region_of(s)} for s in body]
    if scale == 500:
        for pack in ("rear", "front"):
            for s in band_bars(pack, "th-band-"):
                s["region"] = "strapping"
                out.append(s)
    return out


def structure_list(scale):
    """🏗️ The structural idealisation of one scale: deck members, block joints down to the ground, the two stock packs as columns and the six field slabs as plates."""
    out = []
    if scale == 50:
        for k, (z0, z1) in enumerate(BOARD_Z):
            out.append(solid(f"st-{DECK_NAMES[k]}", "pallet", "pallet-timber", 0.5, {"kind": "box", **box_local_pallet((0.0, PALLET_L), (0.122, DECK_TOP), (z0, z1))}, member="pallet", part="member"))
    elif scale in (100, 200):
        top = 0.122 if scale == 100 else 0.1
        out.append(solid("st-deck", "pallet", "pallet-timber", 0.5, {"kind": "box", **box_local_pallet((0.0, PALLET_L), (top, DECK_TOP), (-0.4, 0.4))}, member="pallet", part="member"))
    else:
        out.append(solid("st-pallet", "pallet", "pallet-timber", 0.5, {"kind": "box", **box_local_pallet((0.0, PALLET_L), (0.0, DECK_TOP), (-0.4, 0.4))}, member="pallet", part="member"))
    if scale in (50, 100):
        for i, (x0, x1) in enumerate(ROW_X):
            out.append(solid(f"st-cross-board-{ROW_NAME[i]}", "pallet", "pallet-timber", 0.45, {"kind": "box", **box_local_pallet((x0, x1), (0.1, 0.122), (-0.4, 0.4))}, member="pallet", part="member"))
    if scale < 500:
        top = 0.1
        for i, (x0, x1) in enumerate(ROW_X):
            for k, (z0, z1) in enumerate(ROW_Z):
                out.append(solid(f"st-block-{ROW_NAME[i]}-{SIDE_NAME[k]}", "pallet", "pallet-timber", 0.4, {"kind": "box", **box_local_pallet((x0, x1), (0.0, top), (z0, z1))}, member="pallet", part="joint"))
    for pack in ("rear", "front"):
        n = len(STOCK[pack])
        out.append(box(f"st-pack-{pack}", f"pack-{pack}", "concrete-slab", 0.55, [EDGE, n * THICK, EDGE], [EDGE / 2, n * THICK / 2, 0.0], member=f"pack-{pack}", part="member"))
    for bone in FIELD_ORDER:
        w0, w1 = field_w(bone)
        t = THICK if scale < 500 else FEATURE[500]
        out.append(box(f"st-{bone}", bone, "concrete-slab", 0.6, [EDGE, t, EDGE], [EDGE / 2, t / 2, (w0 + w1) / 2], member=bone, part="member"))
    return out


def shape(sid, bone, role, z, filled, tone, geometry, **extra):
    """✏️ One 2D shape."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geometry)
    out.update(extra)
    return out


def poly(points, closed=True):
    """📐 A poly geometry."""
    return {"kind": "poly", "points": pt(points), "closed": closed}


def rect_poly(x0, x1, y0, y1):
    """▭ A rectangle as a closed poly."""
    return poly([(x0, y0), (x1, y0), (x1, y1), (x0, y1)])


def pallet_parts(scale):
    """🪵 The world boxes the pallet of a scale is drawn from (same numbers as its solids)."""
    if scale == 50:
        return PALLET_PARTS_50
    if scale == 100:
        return [p for p in PALLET_PARTS_50 if p["kind"] != "deck"] + [{"id": "deck-plate", "kind": "deck", "x": (0.0, PALLET_L), "y": (0.122, DECK_TOP), "z": (-0.4, 0.4)}]
    if scale == 200:
        parts = [{"id": "upper-plate", "kind": "deck", "x": (0.0, PALLET_L), "y": (0.1, DECK_TOP), "z": (-0.4, 0.4)}]
        for i, (x0, x1) in enumerate(ROW_X):
            for k, (z0, z1) in enumerate(ROW_Z):
                parts.append({"id": f"foot-{ROW_NAME[i]}-{SIDE_NAME[k]}", "kind": "block", "x": (x0, x1), "y": (0.0, 0.1), "z": (z0, z1)})
        return parts
    return []


PALLET_TONE = {"bottom": 0.4, "block": 0.36, "cross": 0.44, "deck": 0.48}


def pallet_shapes(scale):
    """🪵 The three drawings of the pallet of one scale: each part a closed filled shape, the side seen from the right, the front from ahead, the plan from above."""
    elev, front, plan = [], [], []
    reg = {"region": "pallet-timber", "material": "pallet-timber"}
    if scale == 500:
        elev.append(shape("pallet-side", "pallet", "body", 1, True, 0.42, rect_poly(0.0, PALLET_L, -PALLET_Y, DECK_TOP - PALLET_Y), **reg))
        front.append(shape("pallet-front", "pallet", "body", 1, True, 0.42, poly(pallet_front_profile_world()), **reg))
        plan.append(shape("pallet-top", "pallet", "body", 1, True, 0.46, rect_poly(0.0, PALLET_L, -0.4, 0.4), **reg))
        return elev, front, plan
    parts = pallet_parts(scale)
    layer = {"bottom": 0.9, "block": 1.0, "cross": 1.1, "deck": 1.2}
    for p in parts:
        tone = PALLET_TONE[p["kind"]]
        x, y, z = p["x"], p["y"], p["z"]
        if z[0] <= -0.4 + 1e-9:
            elev.append(shape(p["id"], "pallet", "body", layer[p["kind"]], True, tone, rect_poly(x[0], x[1], y[0] - PALLET_Y, y[1] - PALLET_Y), **reg))
        if x[1] >= PALLET_L - 1e-9:
            front.append(shape(p["id"], "pallet", "body", layer[p["kind"]], True, tone, rect_poly(z[0], z[1], y[0], y[1]), **reg))
        if p["kind"] in ("deck", "cross", "bottom") or (scale == 200 and p["kind"] == "block"):
            pz = {"bottom": 0.4, "cross": 0.6, "deck": 1, "block": 0.5}[p["kind"]]
            plan.append(shape(p["id"], "pallet", "body", pz, True, tone, rect_poly(x[0], x[1], z[0], z[1]), **reg))
    return elev, front, plan


def stock_shapes(scale):
    """🧱 The drawings of the two stock packs: the pack as one closed shape per view with the slab joints as detail lines, chips of the visible face at 1:50."""
    elev, front, plan = [], [], []
    for pack in ("rear", "front"):
        bone = f"pack-{pack}"
        n = len(STOCK[pack])
        reg = {"region": "slab-stock", "material": "concrete-slab"} if scale < 500 else {"regions": ["slab-stock", "strapping"], "material": "concrete-slab"}
        elev.append(shape(f"stock-{pack}", bone, "body", 2, True, 0.56, rect_poly(0.0, EDGE, 0.0, n * THICK), **reg))
        xs = [slab_x(pack, i) for i in STOCK[pack]]
        x0, x1 = xs[0][0], xs[-1][1]
        if scale <= 100:
            for j in range(1, n):
                elev.append(shape(f"stock-{pack}-joint-{j:02d}", bone, "detail", 2.1, False, 0.3, poly([(0.0, j * THICK), (EDGE, j * THICK)], closed=False), region="slab-stock"))
                plan.append(shape(f"stock-{pack}-joint-{j:02d}", bone, "detail", 2.1, False, 0.3, poly([(x0 + j * THICK, -0.25), (x0 + j * THICK, 0.25)], closed=False), region="slab-stock"))
        if scale == 50:
            for i in STOCK[pack]:
                key = f"{pack}-{i:02d}"
                v0, v1 = stock_v(pack, i)
                for corner, a, b in CHIPS[key]:
                    if corner in ("TR", "BR"):
                        u = EDGE - b if corner == "TR" else b
                        elev.append(shape(f"chip-{key}-{corner.lower()}", bone, "detail", 2.2, False, 0.3, poly([(u, v0), (u, v1)], closed=False), region="slab-stock"))
        plan.append(shape(f"stock-{pack}", bone, "body", 2, True, 0.6, rect_poly(x0, x1, -0.25, 0.25), **reg))
        if pack == "front":
            i = STOCK[pack][-1]
            key = f"{pack}-{i:02d}"
            ring = face_profile(CHIPS[key], -0.25, 0.25) if scale == 50 else face_profile([], -0.25, 0.25)
            pts = [(w, DECK_TOP + u) for u, w in ring]
            extra = {"hatch": "concrete"} if scale <= 100 else {}
            front.append(shape(f"stock-{pack}", bone, "body", 2, True, 0.56, poly(pts), **reg, **extra))
    return elev, front, plan


def field_shapes(scale):
    """🧱 The drawings of the six field slabs: an edge rectangle in the side view, and front and plan as `solid` placements of the chipped face swept through the slab thickness, so they read right standing in the pack and lying in the field."""
    elev, front, plan = [], [], []
    for bone in FIELD_ORDER:
        key = field_key(bone)
        w0, w1 = field_w(bone)
        t = THICK if scale < 500 else FEATURE[500]
        reg = {"region": "slab-field", "material": "concrete-slab"}
        elev.append(shape(f"{bone}-edge", bone, "body", 2.5, True, 0.6, rect_poly(0.0, EDGE, 0.0, t), **reg))
        chips = CHIPS[key] if scale == 50 else []
        if scale == 50:
            for corner, a, b in chips:
                if corner in ("TR", "BR"):
                    u = EDGE - b if corner == "TR" else b
                    elev.append(shape(f"chip-{key}-{corner.lower()}", bone, "detail", 2.6, False, 0.3, poly([(u, 0.0), (u, t)], closed=False), region="slab-field"))
        ring = face_profile(chips, w0, w1)
        sol = {"kind": "extruded", "sweep": "v", "z0": 0.0, "z1": r6(t)}
        extra = {"hatch": "concrete"} if scale <= 100 else {}
        front.append(shape(f"{bone}-face", bone, "body", 3, True, 0.6, poly(ring), solid=sol, **reg, **extra))
        plan.append(shape(f"{bone}-face", bone, "body", 2.5, True, 0.62, poly(ring), solid=dict(sol), **reg))
    return elev, front, plan


def band_shapes(scale):
    """🎗️ The strapping bands: a filled strip where a band is seen broadside, an outline around the pack in plan; at 1:200 a single stroke, at 1:500 not drawn."""
    elev, front, plan = [], [], []
    if scale == 500:
        return elev, front, plan
    for pack in ("rear", "front"):
        bone = f"band-{pack}"
        length = PACK_N * THICK
        x0 = PACK_X0[pack]
        reg = {"region": "strapping", "material": "pet-strap"}
        if scale <= 100:
            elev.append(shape(f"band-{pack}-side", bone, "detail", 4, True, 0.22, rect_poly(-BAND_T, length + BAND_T, 0.0, BAND_W), **reg))
            if pack == "front":
                front.append(shape(f"band-{pack}-end", bone, "detail", 4, True, 0.22, rect_poly(-0.25 - BAND_T, 0.25 + BAND_T, DECK_TOP + BAND_H, DECK_TOP + BAND_H + BAND_W), **reg))
            plan.append(shape(f"band-{pack}-loop", bone, "detail", 4, False, 0.22, poly([(x0 - BAND_T / 2, -0.25 - BAND_T / 2), (x0 + length + BAND_T / 2, -0.25 - BAND_T / 2), (x0 + length + BAND_T / 2, 0.25 + BAND_T / 2), (x0 - BAND_T / 2, 0.25 + BAND_T / 2)]), **reg))
        else:
            elev.append(shape(f"band-{pack}-side", bone, "detail", 4, False, 0.22, poly([(-BAND_T, BAND_W / 2), (length + BAND_T, BAND_W / 2)], closed=False), region="strapping"))
            if pack == "front":
                front.append(shape(f"band-{pack}-end", bone, "detail", 4, False, 0.22, poly([(-0.25 - BAND_T, DECK_TOP + BAND_H + BAND_W / 2), (0.25 + BAND_T, DECK_TOP + BAND_H + BAND_W / 2)], closed=False), region="strapping"))
            plan.append(shape(f"band-{pack}-loop", bone, "detail", 4, False, 0.22, poly([(x0 - BAND_T / 2, -0.25 - BAND_T / 2), (x0 + length + BAND_T / 2, -0.25 - BAND_T / 2), (x0 + length + BAND_T / 2, 0.25 + BAND_T / 2), (x0 - BAND_T / 2, 0.25 + BAND_T / 2)]), region="strapping"))
    return elev, front, plan


PENS = {
    50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
    100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35},
    200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25},
    500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES = {
    50: [{"id": "concrete", "title": T("Concrete, weathered face", "Beton, bewitterte Fläche"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}],
    100: [{"id": "concrete", "title": T("Concrete, weathered face", "Beton, bewitterte Fläche"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.1}],
    200: [],
    500: [],
}


def rep_decisions(scale):
    """📝 The design decisions of the representation of one scale."""
    d = scale
    if scale == 50:
        return {
            "silhouette": T(f"Side elevation: the EUR pallet (1200 x 144 mm, {mm(1.2, d)} x {mm(0.144, d)} mm on paper) with its outer bottom board, three blocks, three cross-board ends and the outer deck board, the fork-height openings between the blocks left open, and on it the two packs of eleven slabs standing on edge, each slab a 50 mm edge (1 mm) under a 16 mm strapping band. Front view: the pallet front with the two 227.5 mm fork openings and the chipped face of the front slab with the band across it. Plan: five deck boards with their 41 mm gaps, the slab tops of both packs and the band loops.", f"Seitenansicht: die EUR-Palette (1200 x 144 mm, auf Papier {mmde(1.2, d)} x {mmde(0.144, d)} mm) mit äußerem Bodenbrett, drei Klötzen, drei Querbrettenden und äußerem Deckbrett, die Öffnungen zwischen den Klötzen bleiben offen, darauf die zwei Pakete aus je elf hochkant stehenden Platten, jede Platte eine 50-mm-Kante (1 mm) unter einem 16-mm-Umreifungsband. Vorderansicht: die Palettenstirn mit den zwei 227,5-mm-Gabelöffnungen und die abgeplatzte Fläche der vordersten Platte mit dem Band. Draufsicht: fünf Deckbretter mit ihren 41-mm-Fugen, die Plattenoberkanten beider Pakete und die Bandschlaufen."),
            "retained": T("Every slab at its true 50 mm, every board, block and gap of the EPAL pallet (EN 13698-1), the chipped corners of the slabs (25 to 45 mm legs, 0.5 to 0.9 mm), the two strapping bands at 300 mm above the deck.", "Jede Platte mit ihren wahren 50 mm, jedes Brett, jeder Klotz und jede Fuge der EPAL-Palette (EN 13698-1), die abgeplatzten Plattenecken (Schenkel 25 bis 45 mm, 0,5 bis 0,9 mm), die zwei Umreifungsbänder 300 mm über dem Deck."),
            "merged": T("Each stock pack of eight slabs is one closed shape with seven joint lines; the six slabs that leave for the field are shapes of their own.", "Jedes Lagerpaket aus acht Platten ist eine geschlossene Form mit sieben Fugenlinien; die sechs Platten, die ins Feld gehen, sind eigene Formen."),
            "omitted": T("Nails of the pallet, the EPAL brand marks on the blocks, the strap seals and the 2 to 3 mm unevenness of the reclaimed slab edges.", "Nägel der Palette, die EPAL-Brandzeichen auf den Klötzen, die Bandverschlüsse und die Unebenheit von 2 bis 3 mm der gebrauchten Plattenkanten."),
            "thinElements": T("The 0.9 mm PET band is a filled 16 mm strip (0.32 mm) where it is seen broadside and an outline at its centre line in plan; chips on the near edges are detail strokes across the slab edge.", "Das 0,9-mm-PET-Band ist ein gefüllter 16-mm-Streifen (0,32 mm), wo es breit gesehen wird, und eine Kontur auf seiner Mittellinie in der Draufsicht; Abplatzungen an den nahen Kanten sind Detailstriche quer über die Plattenkante."),
            "hatching": T("Stipple on the weathered concrete faces in the front view; timber in view is not hatched.", "Punktierung auf den bewitterten Betonflächen in der Vorderansicht; Holz in Ansicht ist nicht schraffiert."),
            "styles": T("Outline 0.35 mm, joints, chips and bands 0.18 mm; slabs alternate between tone 0.52 and 0.64 (moss and soil in the pores darken the reclaimed faces), pallet timber 0.36 to 0.48.", "Kontur 0,35 mm, Fugen, Abplatzungen und Bänder 0,18 mm; die Platten wechseln zwischen Ton 0,52 und 0,64 (Moos und Erde in den Poren dunkeln die gebrauchten Flächen), Palettenholz 0,36 bis 0,48."),
            "overlays": T("Pallet, stock packs, field slabs and bands are separate cells; the field slabs carry their own region so the reinstalled field reads apart from the stock.", "Palette, Lagerpakete, Feldplatten und Bänder sind eigene Zellen; die Feldplatten tragen ihre eigene Region, damit das wieder verlegte Feld sich vom Lager abhebt."),
        }
    if scale == 100:
        return {
            "silhouette": T(f"As 1:50 at half size: pallet {mm(1.2, d)} x {mm(0.144, d)} mm with blocks, cross boards and bottom boards, the deck as one plate; two packs of eleven slab edges (0.5 mm each) under the bands; front view with fork openings and the slab face; plan with the deck plate, slab tops and band loops.", f"Wie 1:50 in halber Größe: Palette {mmde(1.2, d)} x {mmde(0.144, d)} mm mit Klötzen, Querbrettern und Bodenbrettern, das Deck als eine Platte; zwei Pakete aus elf Plattenkanten (je 0,5 mm) unter den Bändern; Vorderansicht mit Gabelöffnungen und Plattenfläche; Draufsicht mit Deckplatte, Plattenoberkanten und Bandschlaufen."),
            "retained": T("Every slab and its joint, blocks, cross boards, bottom boards and the fork openings at their true size; both bands.", "Jede Platte und ihre Fuge, Klötze, Querbretter, Bodenbretter und die Gabelöffnungen in wahrer Größe; beide Bänder."),
            "merged": T("The five deck boards are one 22 mm plate: their 41 mm gaps would close to under 0.15 mm of white between 0.25 mm outlines.", "Die fünf Deckbretter sind eine 22-mm-Platte: ihre 41-mm-Fugen würden zwischen 0,25-mm-Konturen auf unter 0,15 mm Weiß zulaufen."),
            "omitted": T("The chipped corners (0.3 mm), nails, brand marks and strap seals.", "Die Abplatzungen (0,3 mm), Nägel, Brandzeichen und Bandverschlüsse."),
            "thinElements": T("Bands as filled 16 mm strips (0.16 mm) in the 0.13 mm detail pen, outlines in plan.", "Bänder als gefüllte 16-mm-Streifen (0,16 mm) in der 0,13-mm-Detailfeder, Konturen in der Draufsicht."),
            "hatching": T("Stipple on the concrete faces in the front view.", "Punktierung auf den Betonflächen in der Vorderansicht."),
            "styles": T("Outline 0.25 mm, joints and bands 0.13 mm; slabs tone 0.56 to 0.62, pallet 0.36 to 0.48.", "Kontur 0,25 mm, Fugen und Bänder 0,13 mm; Platten Ton 0,56 bis 0,62, Palette 0,36 bis 0,48."),
            "overlays": T("Cells as at 1:50.", "Zellen wie bei 1:50."),
        }
    if scale == 200:
        return {
            "silhouette": T(f"Pallet {mm(1.2, d)} x {mm(0.144, d)} mm as an upper plate on nine feet, so the openings between the feet read in the side and the front view; each stock pack one block, the three field slabs of each pack single 50 mm edges (0.25 mm); the bands single strokes.", f"Palette {mmde(1.2, d)} x {mmde(0.144, d)} mm als obere Platte auf neun Füßen, sodass die Öffnungen zwischen den Füßen in Seiten- und Vorderansicht lesbar bleiben; jedes Lagerpaket ein Block, die drei Feldplatten je Paket einzelne 50-mm-Kanten (0,25 mm); die Bänder einzelne Striche."),
            "retained": T("Overall size, the fork openings, the gap of 50 mm between the packs, the six field slabs as single slabs.", "Gesamtgröße, die Gabelöffnungen, die Lücke von 50 mm zwischen den Paketen, die sechs Feldplatten als einzelne Platten."),
            "merged": T("Deck boards and cross boards are one 44 mm plate, block and bottom board one foot; the eight stock slabs of a pack are one block.", "Deckbretter und Querbretter sind eine 44-mm-Platte, Klotz und Bodenbrett ein Fuß; die acht Lagerplatten eines Pakets sind ein Block."),
            "omitted": T("Slab joints inside the stock blocks, the bottom boards between the feet, the chips.", "Plattenfugen in den Lagerblöcken, die Bodenbretter zwischen den Füßen, die Abplatzungen."),
            "thinElements": T("The 16 mm bands (0.08 mm) are single strokes in the 0.1 mm detail pen.", "Die 16-mm-Bänder (0,08 mm) sind einzelne Striche in der 0,1-mm-Detailfeder."),
            "hatching": T("None.", "Keine."),
            "styles": T("Outline 0.18 mm, bands 0.1 mm; slabs tone 0.56 to 0.6, pallet 0.38 to 0.46.", "Kontur 0,18 mm, Bänder 0,1 mm; Platten Ton 0,56 bis 0,6, Palette 0,38 bis 0,46."),
            "overlays": T("Pallet, stock blocks, field slabs and bands are separate cells.", "Palette, Lagerblöcke, Feldplatten und Bänder sind eigene Zellen."),
        }
    return {
        "silhouette": T(f"The loaded pallet {mm(1.2, d)} x {mm(0.644, d)} mm: the pallet a solid strip of 144 mm (0.29 mm), in the front view with its two fork openings, the stock packs as blocks and each field slab a 75 mm strip (0.15 mm), the smallest feature of the scale.", f"Die beladene Palette {mmde(1.2, d)} x {mmde(0.644, d)} mm: die Palette ein voller Streifen von 144 mm (0,29 mm), in der Vorderansicht mit ihren zwei Gabelöffnungen, die Lagerpakete als Blöcke und jede Feldplatte ein 75-mm-Streifen (0,15 mm), die kleinste Einzelheit des Maßstabs."),
        "retained": T("Pallet height, the two packs with their 50 mm gap, the fork openings in the front view, six field slabs that can leave for the field.", "Palettenhöhe, die zwei Pakete mit ihrer 50-mm-Lücke, die Gabelöffnungen in der Vorderansicht, sechs Feldplatten, die ins Feld gehen können."),
        "merged": T("The whole pallet is one body; each field slab is drawn 75 mm thick, overlapping its neighbour behind it in the pack by 25 mm.", "Die ganze Palette ist ein Körper; jede Feldplatte ist 75 mm dick gezeichnet und überdeckt ihre Nachbarin dahinter im Paket um 25 mm."),
        "omitted": T("Joints, chips, boards and blocks; the bands are merged into the outline of their pack, which carries both regions.", "Fugen, Abplatzungen, Bretter und Klötze; die Bänder gehen in der Kontur ihres Pakets auf, die beide Regionen trägt."),
        "thinElements": T("The 50 mm slabs are thickened to 75 mm (0.15 mm) so that a single field slab stays printable to 1:1000; the bands (16 mm) are not drawn.", "Die 50-mm-Platten sind auf 75 mm (0,15 mm) verdickt, damit eine einzelne Feldplatte bis 1:1000 druckbar bleibt; die Bänder (16 mm) werden nicht gezeichnet."),
        "hatching": T("None.", "Keine."),
        "styles": T("Outline 0.18 mm so that a 50 mm visible slab strip still prints as 0.2 mm at 1:1000; slabs tone 0.56 to 0.6, pallet 0.42.", "Kontur 0,18 mm, damit ein sichtbarer 50-mm-Plattenstreifen bei 1:1000 noch als 0,2 mm druckt; Platten Ton 0,56 bis 0,6, Palette 0,42."),
        "overlays": T("Pallet, stock blocks and field slabs are separate cells.", "Palette, Lagerblöcke und Feldplatten sind eigene Zellen."),
    }


def representation(scale):
    """🗺️ The authored representation of one scale."""
    e, f, p = [], [], []
    for part in (pallet_shapes(scale), stock_shapes(scale), field_shapes(scale), band_shapes(scale)):
        e += part[0]
        f += part[1]
        p += part[2]
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": HATCHES[scale], "geometry": {"elevation": e, "front": f, "plan": p}, "decisions": rep_decisions(scale)}


def volume_decisions(scale):
    """📝 What each scale's 3D model keeps, merges and why."""
    if scale == 50:
        return [
            T("Body: the EPAL pallet board by board (three bottom boards, nine blocks, three cross boards, five deck boards with their 41 mm gaps) and the twenty-two slabs one by one, each an extrusion of its own 500 x 500 mm face with its chipped corners (25 to 45 mm legs) through its 50 mm thickness, standing on edge face to face in two packs of eleven, the two PET bands as four slender 16 x 0.9 mm strips each touching the slab faces.", "Körper: die EPAL-Palette Brett für Brett (drei Bodenbretter, neun Klötze, drei Querbretter, fünf Deckbretter mit ihren 41-mm-Fugen) und die zweiundzwanzig Platten einzeln, jede eine Extrusion ihrer eigenen 500 x 500 mm großen Fläche mit ihren abgeplatzten Ecken (Schenkel 25 bis 45 mm) über ihre 50 mm Dicke, hochkant Fläche an Fläche in zwei Paketen zu elf, die zwei PET-Bänder als je vier schlanke 16 x 0,9 mm Streifen an den Plattenflächen."),
            T("Thermal: every body solid by reference, in four regions: pallet timber, stock slabs, field slabs and strapping.", "Thermisch: jeder Körper als Verweis, in vier Regionen: Palettenholz, Lagerplatten, Feldplatten und Umreifung."),
            T("Structure: the five deck boards and the three cross boards as the pallet members, the nine blocks with their bottom boards as joints down to the ground, each stock pack a column of its 400 x 500 mm section, each field slab a 500 x 50 mm plate; the supports are the contacts of each pose (pallet feet, fork tines, sand bed).", "Tragwerk: die fünf Deckbretter und die drei Querbretter als Palettenglieder, die neun Klötze mit ihren Bodenbrettern als Gelenke bis zum Boden, jedes Lagerpaket eine Stütze mit seinem Querschnitt 400 x 500 mm, jede Feldplatte eine Platte 500 x 50 mm; die Auflager sind die Kontakte jeder Pose (Palettenfüße, Gabelzinken, Sandbett)."),
        ]
    if scale == 100:
        return [
            T("Body: as 1:50 but the five deck boards are one 22 mm plate (the 41 mm gaps are 0.4 mm) and the chips are left out (0.3 mm): every slab, stock and field, is a plain 500 x 500 x 50 mm box standing face to face with its neighbours.", "Körper: wie 1:50, aber die fünf Deckbretter sind eine 22-mm-Platte (die 41-mm-Fugen sind 0,4 mm) und die Abplatzungen entfallen (0,3 mm): jede Platte, Lager- wie Feldplatte, ist ein einfacher Quader 500 x 500 x 50 mm, Fläche an Fläche mit ihren Nachbarinnen."),
            T("Thermal: every body solid by reference in the same four regions.", "Thermisch: jeder Körper als Verweis in denselben vier Regionen."),
            T("Structure: the deck as one plate member on the three cross boards, nine blocks with their bottom boards as joints, the two stock columns and six field plates.", "Tragwerk: das Deck als ein Plattenglied auf den drei Querbrettern, neun Klötze mit ihren Bodenbrettern als Gelenke, die zwei Lagerstützen und sechs Feldplatten."),
        ]
    if scale == 200:
        return [
            T("Body: the pallet as a 44 mm upper plate (deck and cross boards) on nine 100 mm feet (block and bottom board), each stock pack one 400 mm block, the six field slabs single 50 mm slabs, the bands slender strips.", "Körper: die Palette als obere 44-mm-Platte (Deck und Querbretter) auf neun 100-mm-Füßen (Klotz und Bodenbrett), jedes Lagerpaket ein 400-mm-Block, die sechs Feldplatten einzelne 50-mm-Platten, die Bänder schlanke Streifen."),
            T("Thermal: every body solid by reference in the four regions.", "Thermisch: jeder Körper als Verweis in den vier Regionen."),
            T("Structure: the upper plate as the pallet member on nine feet as joints, two stock columns, six field plates.", "Tragwerk: die obere Platte als Palettenglied auf neun Füßen als Gelenke, zwei Lagerstützen, sechs Feldplatten."),
        ]
    return [
        T("Body: the pallet one extrusion of its front outline with the two fork openings (144 mm high), each stock pack one block and each field slab 75 mm thick (the smallest feature), overlapping its neighbour behind it in the pack by 25 mm so the pack keeps its 550 mm length; the bands (16 mm) are not in the body.", "Körper: die Palette eine Extrusion ihrer Stirnkontur mit den zwei Gabelöffnungen (144 mm hoch), jedes Lagerpaket ein Block und jede Feldplatte 75 mm dick (die kleinste Einzelheit), um 25 mm in ihre Nachbarin dahinter im Paket greifend, sodass das Paket seine Länge von 550 mm behält; die Bänder (16 mm) gehören nicht zum Körper."),
        T("Thermal: the body solids by reference, the bands kept as their own slender 16 x 0.9 mm strips so that every region has a solid.", "Thermisch: die Körper als Verweise, die Bänder als eigene schlanke 16 x 0,9 mm Streifen, damit jede Region einen Körper hat."),
        T("Structure: the pallet as one 144 mm plate member on the ground, two stock columns, six field plates of 75 mm.", "Tragwerk: die Palette als ein 144-mm-Plattenglied auf dem Boden, zwei Lagerstützen, sechs Feldplatten von 75 mm."),
    ]


def volumes():
    """🧊 The four handcrafted volumes."""
    out = {}
    for scale in SCALES:
        body = body_list(scale)
        out[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(scale, body), "structure": structure_list(scale), "decisions": volume_decisions(scale)}
    return out


def section(area, inertia, fibre, material, en, de):
    """📏 A member section."""
    return {"areaM2": r6(area), "secondMomentM4": float(f"{inertia:.6e}"), "extremeFibreM": r6(fibre), "material": material, "title": T(en, de)}


def rig():
    """🦴 The rig: yard datum, the pallet on its lifting joint, two stock packs and two bands on the pallet, six field slabs each on a carrier that travels to its place in the field, and the wheel carrier of the reinstalled field."""
    deck_area = 0.635 * 0.022 + 0.345 * 0.022
    deck_i = (0.635 + 0.345) * 0.022 ** 3 / 12
    bones = [
        {"id": "yard", "parent": None, "title": T("Yard datum (ground of the component exchange)", "Hofbezug (Boden der Bauteilbörse)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "pallet", "parent": "yard", "title": T("EUR pallet (lifted by the fork tines)", "EUR-Palette (von den Gabelzinken gehoben)"), "x": 0, "y": PALLET_Y, "angleDeg": 0, "lengthM": PALLET_L, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 1.6},
         "mass": {"kg": r6(PALLET_MASS), "comX": PALLET_L / 2, "comY": r6(PALLET_COM_Y - PALLET_Y), "distribution": "uniform", "source": "authored-estimate", "reference": "en-13698-1"},
         "section": section(deck_area, deck_i, 0.011, "pallet-timber", "Deck boards and bottom boards 980 x 22 mm (not composite)", "Deck- und Bodenbretter 980 x 22 mm (nicht verbunden)")},
    ]
    for pack in ("rear", "front"):
        n = len(STOCK[pack])
        xo = slab_x(pack, STOCK[pack][-1])[1]
        bones.append({"id": f"pack-{pack}", "parent": "pallet", "title": T(f"Stock pack {'rear' if pack == 'rear' else 'front'} (eight slabs on edge)", f"Lagerpaket {'hinten' if pack == 'rear' else 'vorn'} (acht Platten hochkant)"), "x": r6(xo), "y": r6(DECK_TOP - PALLET_Y), "angleDeg": 90, "lengthM": EDGE, "joint": {"type": "fixed"},
                      "mass": {"kg": r6(STOCK_MASS[pack]), "comX": EDGE / 2, "comY": r6(n * THICK / 2), "distribution": "uniform", "source": "authored-estimate", "reference": "en-1339"},
                      "section": section(n * THICK * EDGE, EDGE * (n * THICK) ** 3 / 12, n * THICK / 2, "concrete-slab", f"{n} slabs on edge, {int(n * THICK * 1000)} x 500 mm", f"{n} Platten hochkant, {int(n * THICK * 1000)} x 500 mm")})
    for pack in ("rear", "front"):
        bones.append({"id": f"band-{pack}", "parent": "pallet", "title": T(f"Strapping band of the {pack} pack", f"Umreifungsband des {'hinteren' if pack == 'rear' else 'vorderen'} Pakets"), "x": r6(PACK_X0[pack]), "y": r6(DECK_TOP + BAND_H - PALLET_Y), "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -BAND_H, "maxM": 0},
                      "mass": {"kg": r6(BAND_MASS), "comX": r6(PACK_N * THICK / 2), "comY": r6(BAND_W / 2), "distribution": "point", "source": "authored-estimate", "reference": "en-13394"}})
    for bone in FIELD_ORDER:
        f = FIELD[bone]
        dx, dy = field_travel(bone)
        s = math.hypot(dx, dy)
        axis = math.degrees(math.atan2(dy, dx))
        col_a = f["column"] == "a"
        bones.append({"id": f"carrier-{bone[5:]}", "parent": "pallet", "title": T(f"Carrier of {bone} to its field place", f"Träger von {bone} an seinen Feldplatz"), "x": r6(field_origin_x(bone)), "y": r6(DECK_TOP - PALLET_Y), "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": axis, "minM": 0, "maxM": r6(s + 0.001)}})
        joint = {"type": "revolute", "minDeg": -90, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": 0} if col_a else {"type": "revolute", "minDeg": 0, "maxDeg": 90, "twistMinDeg": 0, "twistMaxDeg": 180}
        bones.append({"id": bone, "parent": f"carrier-{bone[5:]}", "title": T(f"Field slab {bone[5:]} (tips about its bottom edge)", f"Feldplatte {bone[5:]} (kippt um ihre Unterkante)"), "x": 0, "y": 0, **({} if col_a else {"z": COL_B_Z}), "angleDeg": 90, "lengthM": EDGE, "joint": joint,
                      "mass": {"kg": r6(FIELD_MASS[bone]), "comX": EDGE / 2, "comY": THICK / 2, "distribution": "uniform", "source": "authored-estimate", "reference": "en-1339"},
                      "section": section(EDGE * THICK, EDGE * THICK ** 3 / 12, THICK / 2, "concrete-slab", "Slab 500 x 50 mm", "Platte 500 x 50 mm")})
    bones.append({"id": "wheel-carrier", "parent": "yard", "title": T("Wheel carrier (massless, moves the wheel load across the field)", "Radträger (masselos, bewegt die Radlast über das Feld)"), "x": WHEEL_X0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.51}})
    return {"kind": "mechanical", "root": "yard", "bones": bones}


def anchors():
    """📍 Pallet feet, fork tine contacts, the sand bed under each field slab and the wheel load point."""
    out = []
    for i, x in enumerate([0.0725, 0.6, 1.1275]):
        for k, z in enumerate([-0.35, 0.0, 0.35]):
            out.append({"id": f"foot-{ROW_NAME[i]}-{SIDE_NAME[k]}", "bone": "pallet", "x": x, "y": r6(-PALLET_Y), "z": z, "kind": "contact", "title": T(f"Pallet foot {ROW_NAME[i]} {SIDE_NAME[k]} on the yard", f"Palettenfuß {['hinten', 'mitte', 'vorn'][i]} {['rechts', 'mitte', 'links'][k]} auf dem Hof"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.145, "footprintWidthM": 0.145 if k == 1 else 0.1, "frictionCoefficient": 0.5})
    for i, x in enumerate([0.0725, 0.6, 1.1275]):
        for k, z in enumerate([-0.18625, 0.18625]):
            out.append({"id": f"fork-{ROW_NAME[i]}-{['right', 'left'][k]}", "bone": "pallet", "x": x, "y": r6(0.1 - PALLET_Y), "z": z, "kind": "contact", "title": T(f"Fork tine {['right', 'left'][k]} under the {ROW_NAME[i]} cross board", f"Gabelzinke {['rechts', 'links'][k]} unter dem {['hinteren', 'mittleren', 'vorderen'][i]} Querbrett"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.145, "footprintWidthM": 0.1, "frictionCoefficient": 0.3})
    for bone in FIELD_ORDER:
        w0, w1 = field_w(bone)
        out.append({"id": f"bed-{bone[5:]}", "bone": bone, "x": EDGE / 2, "y": 0, "z": r6((w0 + w1) / 2), "kind": "contact", "title": T(f"Sand bed under field slab {bone[5:]}", f"Sandbett unter Feldplatte {bone[5:]}"), "support": "contact", "normalDeg": 90, "footprintLengthM": EDGE, "footprintWidthM": EDGE, "frictionCoefficient": 0.6})
    out.append({"id": "wheel-contact", "bone": "wheel-carrier", "x": 0, "y": THICK, "kind": "load", "title": T("Wheel contact on the field", "Radaufstand auf dem Feld")})
    return out


FEET = [f"foot-{r}-{s}" for r in ROW_NAME for s in SIDE_NAME]
FORKS = [f"fork-{r}-{s}" for r in ROW_NAME for s in ("right", "left")]
BEDS = [f"bed-{b[5:]}" for b in FIELD_ORDER]


def binding(stock, field, pallet, strap):
    """🌬️ A thermal binding of every region."""
    return {"regions": {"slab-stock": stock, "slab-field": field, "pallet-timber": pallet, "strapping": strap}, "emitters": {}}


def reinstall_bones():
    """🧩 The offsets that take the six field slabs into the 2 x 3 field and drop the opened bands to the deck."""
    out = {}
    for bone in FIELD_ORDER:
        dx, dy = field_travel(bone)
        out[f"carrier-{bone[5:]}"] = {"dx": r6(dx), "dy": r6(dy)}
        out[bone] = {"angleDeg": -90} if FIELD[bone]["column"] == "a" else {"twistDeg": 180, "angleDeg": 90}
    out["band-rear"] = {"dy": -BAND_H}
    out["band-front"] = {"dy": -BAND_H}
    return out


def poses():
    """🧍 Stored, lifted, reinstalled and reinstalled under the wheel load."""
    stored_t = binding({"exposedFraction": 1, "sunlitFraction": 0.4}, {"exposedFraction": 1, "sunlitFraction": 0.4}, {"exposedFraction": 1, "sunlitFraction": 0.2, "contact": {"fraction": 0.05, "tempC": 18}}, {"exposedFraction": 1, "sunlitFraction": 0.5})
    lifted_t = binding({"exposedFraction": 1, "sunlitFraction": 0.4}, {"exposedFraction": 1, "sunlitFraction": 0.4}, {"exposedFraction": 1, "sunlitFraction": 0.2, "contact": {"fraction": 0.03, "tempC": 25}}, {"exposedFraction": 1, "sunlitFraction": 0.5})
    laid_t = binding({"exposedFraction": 1, "sunlitFraction": 0.4}, {"exposedFraction": 1, "sunlitFraction": 0.5, "contact": {"fraction": 0.5, "tempC": 16}}, {"exposedFraction": 1, "sunlitFraction": 0.2, "contact": {"fraction": 0.05, "tempC": 18}}, {"exposedFraction": 1, "sunlitFraction": 0.3})
    wheel = {"id": "wheel-load", "kind": "live", "title": T("Wheel load 2.5 kN of a loaded hand cart", "Radlast 2,5 kN eines beladenen Handwagens"), "anchor": "wheel-contact", "fxN": 0, "fyN": -WHEEL_N, "source": "authored-estimate", "reference": "en-1991-1-1"}
    return [
        {"id": "stored", "title": T("Stored on the pallet", "Auf der Palette gelagert"), "description": T("Twenty-two reclaimed slabs stand on edge in two strapped packs of eleven on a EUR pallet in the yard of the component exchange; the pallet stands on its nine feet.", "Zweiundzwanzig gebrauchte Platten stehen hochkant in zwei umreiften Paketen zu elf auf einer EUR-Palette im Hof der Bauteilbörse; die Palette steht auf ihren neun Füßen."), "bones": {}, "ground": "authored", "contacts": FEET, "thermal": stored_t, "loads": []},
        {"id": "lifted", "title": T("Lifted on the fork tines", "Auf den Gabelzinken gehoben"), "description": T("A forklift has driven its tines under the three cross boards and lifts the loaded pallet 400 mm off the yard; the six tine contacts carry the whole load.", "Ein Gabelstapler hat seine Zinken unter die drei Querbretter gefahren und hebt die beladene Palette 400 mm vom Hof; die sechs Zinkenkontakte tragen die ganze Last."), "bones": {"pallet": {"dy": LIFT}}, "ground": "authored", "contacts": FORKS, "thermal": lifted_t, "loads": []},
        {"id": "reinstalled", "title": T("Reinstalled as a 2 x 3 field", "Als Feld 2 x 3 wieder verlegt"), "description": T("Six slabs are laid flat in a 1.0 x 1.5 m field on a sand bed in front of the pallet with 5 mm sand-filled joints; the sixteen remaining slabs stand on the pallet, their opened bands lie on the deck.", "Sechs Platten liegen flach als Feld 1,0 x 1,5 m auf einem Sandbett vor der Palette mit 5 mm breiten, gesandeten Fugen; die sechzehn übrigen Platten stehen auf der Palette, ihre geöffneten Bänder liegen auf dem Deck."), "bones": reinstall_bones(), "ground": "authored", "contacts": FEET + BEDS, "thermal": laid_t, "loads": []},
        {"id": "reinstalled-wheel", "title": T("Reinstalled under a wheel load", "Wieder verlegt unter Radlast"), "description": T("The field carries a 2.5 kN wheel load of a loaded hand cart on the middle slab of its near row.", "Das Feld trägt eine Radlast von 2,5 kN eines beladenen Handwagens auf der mittleren Platte seiner nahen Reihe."), "bones": {**reinstall_bones(), "wheel-carrier": {"dx": 0.75}}, "ground": "authored", "contacts": FEET + BEDS, "thermal": laid_t, "loads": [wheel]},
    ]


def clips():
    """🎞️ The lift of the loaded pallet and the wheel crossing the reinstalled field."""
    return [
        {"id": "lift", "title": T("Lift on the fork tines", "Anheben auf den Gabelzinken"), "description": T("The tines take the load under the cross boards and lift the pallet with its slabs 400 mm off the yard.", "Die Zinken übernehmen die Last unter den Querbrettern und heben die Palette mit ihren Platten 400 mm vom Hof."),
         "durationS": 4, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "step"}, {"t": 0.5, "pose": "lifted", "ease": "smooth", "bones": {"pallet": {"dy": 0}}}, {"t": 4, "pose": "lifted", "ease": "smooth"}],
         "contacts": [*[{"anchor": a, "fromS": 0, "toS": 0.5} for a in FEET], *[{"anchor": a, "fromS": 0.5, "toS": 4} for a in FORKS]],
         "phases": [{"id": "engage", "fromS": 0, "toS": 0.5, "title": T("Tines take the load", "Zinken übernehmen die Last")}, {"id": "raise", "fromS": 0.5, "toS": 4, "title": T("Raise", "Heben")}],
         "events": [{"t": 0.5, "kind": "contact-off", "title": T("Pallet leaves the yard", "Palette verlässt den Hof")}, {"t": 0.5, "kind": "contact-on", "title": T("Tines carry the pallet", "Zinken tragen die Palette")}], "tracks": []},
        {"id": "wheel-crossing", "title": T("Wheel crossing the field", "Rad überquert das Feld"), "description": T("A 2.5 kN wheel rolls along the near row of the reinstalled field from its first to its third slab.", "Ein Rad mit 2,5 kN rollt die nahe Reihe des wieder verlegten Felds von der ersten zur dritten Platte entlang."),
         "durationS": 5, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled-wheel", "ease": "linear", "bones": {"wheel-carrier": {"dx": 0.05}}}, {"t": 5, "pose": "reinstalled-wheel", "ease": "linear", "bones": {"wheel-carrier": {"dx": 1.45}}}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 5} for a in FEET + BEDS],
         "phases": [{"id": "roll", "fromS": 0, "toS": 5, "title": T("Roll", "Rollen")}],
         "events": [{"t": 0, "kind": "load-on", "title": T("Wheel enters the field", "Rad fährt auf das Feld")}, {"t": 5, "kind": "load-off", "title": T("Wheel leaves the field", "Rad verlässt das Feld")}], "tracks": []},
    ]


MATERIALS = [
    {"id": "concrete-slab", "title": T("Paving concrete, weathered with moss", "Gehwegbeton, bewittert mit Moos"), "emissivity": 0.94, "solarAbsorptivity": 0.75, "densityKgM3": RHO_SLAB, "conductivityWmK": 1.8, "specificHeatJKgK": 1000, "elasticModulusPa": 30000000000, "strengthPa": 4000000, "source": "authored-estimate", "reference": "en-1339"},
    {"id": "pallet-timber", "title": T("Pallet softwood, weathered grey", "Palettennadelholz, grau bewittert"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO_TIMBER, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 8000000000, "strengthPa": 16000000, "source": "authored-estimate", "reference": "en-338"},
    {"id": "pet-strap", "title": T("PET strapping band, green", "PET-Umreifungsband, grün"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": RHO_PET, "conductivityWmK": 0.24, "specificHeatJKgK": 1300, "elasticModulusPa": 10000000000, "strengthPa": 400000000, "source": "authored-estimate", "reference": "en-13394"},
]

REFERENCES = [
    {"id": "en-1339", "citation": "EN 1339:2003 Concrete paving flags, requirements and test methods: nominal 500 x 500 x 50 mm flags, bending strength classes 1 (S) 3.5 MPa, 2 (T) 4.0 MPa, 3 (U) 5.0 MPa characteristic; plain concrete of about 2300 kg/m3 (authored estimate of the period product)."},
    {"id": "din-485", "citation": "DIN 485:1987 Gehwegplatten aus Beton (the German standard under which slabs of 50 x 50 x 5 cm were made before EN 1339): sizes 30, 35, 40 and 50 cm, thickness 5 cm."},
    {"id": "en-13698-1", "citation": "EN 13698-1:2003 Pallet production specification, part 1: flat wooden pallets 800 x 1200 mm (EPAL/EUR pallet): 5 deck boards 22 mm (145, 100, 145, 100, 145 mm), 3 cross boards 145 x 22 mm, 9 blocks 78 mm high, 3 bottom boards 22 mm; height 144 mm, fork openings 227.5 mm wide."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes: C16 E0,mean 8 GPa, fm,k 16 MPa, mean density 370 kg/m3 (dry); 520 kg/m3 is taken for pallet softwood at the moisture of outdoor storage (authored estimate)."},
    {"id": "en-13394", "citation": "EN 13394:2001 Packaging, non-metallic strapping (PET strapping 16 x 0.9 mm, tensile strength of the order of 400 MPa, density 1380 kg/m3)."},
    {"id": "din-18318", "citation": "DIN 18318:2019 VOB part C, Verkehrswegebauarbeiten, Pflasterdecken und Plattenbeläge: bedding of 3 to 5 cm of sand, joints 3 to 5 mm filled with sand."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1, densities and imposed loads; the 2.5 kN wheel load of a loaded hand cart is an authored estimate within the concentrated loads of Table 6.2."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties: concrete 2200 kg/m3 1.65 W/mK, 2400 kg/m3 2.0 W/mK (1.8 W/mK interpolated for 2300 kg/m3), 1000 J/kgK; softwood 0.13 W/mK, 1600 J/kgK."},
    {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); half of it is taken as the effective storage depth of an exposed face."},
    {"id": "emissivity-literature", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 4, radiation properties of building materials: weathered concrete with biological growth emissivity about 0.94 and solar absorptance 0.7 to 0.8 (new grey concrete about 0.6); weathered softwood 0.9 and 0.6 to 0.7 (typical rounded values guided by this chapter)."},
]


def thermal():
    """🌡️ Four passive regions: the stock slabs, the field slabs, the pallet timber and the strapping."""
    pen = math.sqrt(1.8 * 86400 / (math.pi * RHO_SLAB * 1000))
    regions = [
        {"id": "slab-stock", "title": T("Stock slabs (sixteen on the pallet)", "Lagerplatten (sechzehn auf der Palette)"), "kind": "surface", "model": "passive", "material": "concrete-slab", "areaM2": 2.6, "characteristicLengthM": EDGE, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": round(pen / 2 * RHO_SLAB * 1000), "source": "authored-estimate", "reference": "en-iso-13786"},
        {"id": "slab-field", "title": T("Field slabs (six, stored and reinstalled)", "Feldplatten (sechs, gelagert und wieder verlegt)"), "kind": "surface", "model": "passive", "material": "concrete-slab", "areaM2": 1.6, "characteristicLengthM": EDGE, "geometry": "horizontal-plate-up", "skyViewFactor": 0.9, "layers": [], "heatCapacityJM2K": round(THICK * RHO_SLAB * 1000), "source": "authored-estimate", "reference": "en-iso-13786"},
        {"id": "pallet-timber", "title": T("Pallet timber", "Palettenholz"), "kind": "surface", "model": "passive", "material": "pallet-timber", "areaM2": 3.14, "characteristicLengthM": 0.145, "geometry": "horizontal-plate-up", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": round(0.011 * RHO_TIMBER * 1600), "source": "authored-estimate", "reference": "en-iso-10456"},
        {"id": "strapping", "title": T("PET strapping bands", "PET-Umreifungsbänder"), "kind": "surface", "model": "passive", "material": "pet-strap", "areaM2": r6(2 * (2 * BAND_LEN * BAND_W + 2 * BAND_LEN * BAND_T)), "characteristicLengthM": BAND_W, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": round(BAND_T / 2 * RHO_PET * 1300), "source": "authored-estimate", "reference": "en-iso-10456"},
    ]
    return {
        "environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.6, "windMS": 2, "windDirectionDeg": 0, "solarWM2": 600, "diffuseFraction": 0.25, "pressurePa": 101325},
        "operating": [], "regions": regions, "emitters": [], "passengers": [],
        "assumptions": [
            T(f"The slabs store heat in an effective depth of about {pen / 2 * 1000:.0f} mm per face (half the 24 h penetration depth of {pen * 1000:.0f} mm for 1.8 W/mK, 2300 kg/m3, 1000 J/kgK, EN ISO 13786) in the packs, and in their whole 50 mm when laid in the field.", f"Die Platten speichern Wärme in den Paketen in einer wirksamen Tiefe von etwa {pen / 2 * 1000:.0f} mm je Fläche (die Hälfte der 24-h-Eindringtiefe von {pen * 1000:.0f} mm für 1,8 W/mK, 2300 kg/m³, 1000 J/kgK, EN ISO 13786), im Feld in ihren ganzen 50 mm."),
            T("Weathered and moss-grown concrete absorbs more sun (0.75) than new grey concrete (about 0.6); weathered pallet timber is grey (0.65).", "Bewitterter, bemooster Beton nimmt mehr Sonne auf (0,75) als neuer grauer Beton (etwa 0,6); bewittertes Palettenholz ist grau (0,65)."),
            T("Areas are the outer surfaces of the stored state: slabs that stand face to face share no surface, the face against a neighbour pack or the deck counts. In the field half of the field area lies on the sand bed (contact fraction 0.5 at 16 degC).", "Flächen sind die Außenflächen des gelagerten Zustands: Platten, die Fläche an Fläche stehen, haben keine gemeinsame Oberfläche, die Fläche zum Nachbarpaket oder zum Deck zählt. Im Feld liegt die Hälfte der Feldfläche auf dem Sandbett (Kontaktanteil 0,5 bei 16 °C)."),
            T("Reference case: a mild sunny day in an open yard; all values are authored estimates.", "Referenzfall: ein milder sonniger Tag auf einem offenen Hof; alle Werte sind Schätzungen."),
        ],
    }


def structure():
    """🏗️ Quasi-static rigid body on the contacts of each pose; no permanent supports."""
    return {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
        T("The loaded pallet is one rigid body on the contacts of its pose: nine pallet feet in the yard when stored, six fork tine contacts under the cross boards when lifted, the feet and the six sand-bed contacts of the field slabs when reinstalled; contacts that would pull drop out.", "Die beladene Palette ist ein starrer Körper auf den Kontakten ihrer Pose: neun Palettenfüße auf dem Hof im Lager, sechs Zinkenkontakte unter den Querbrettern beim Heben, die Füße und die sechs Sandbettkontakte der Feldplatten nach dem Wiedereinbau; ziehende Kontakte entfallen."),
        T("Slabs carry their weight on edge through the 500 x 50 mm edge into the deck boards, which span 383 mm between the cross boards; the bands only hold the packs upright and carry no weight.", "Die Platten tragen ihr Gewicht hochkant über die Kante 500 x 50 mm in die Deckbretter, die 383 mm zwischen den Querbrettern spannen; die Bänder halten die Pakete nur aufrecht und tragen kein Gewicht."),
        T("In the field the six slabs and the pallet are one rigid group of the rig, so the wheel load is shared by every contact in proportion to the equal-stiffness solution; member forces at the field slab carriers are those of the rig tree, not of a physical connection to the pallet.", "Im Feld sind die sechs Platten und die Palette eine starre Gruppe des Rigs, daher verteilt sich die Radlast nach der Lösung gleicher Steifigkeit auf alle Kontakte; Stabkräfte an den Trägern der Feldplatten sind die des Rig-Baums, nicht die einer physischen Verbindung zur Palette."),
        T("Concrete E = 30 GPa, bending strength class 2 (T) 4.0 MPa of EN 1339 assumed for the reclaimed slabs; pallet softwood C16 (EN 338).", "Beton E = 30 GPa, Biegefestigkeitsklasse 2 (T) 4,0 MPa nach EN 1339 für die gebrauchten Platten angenommen; Palettennadelholz C16 (EN 338)."),
    ]}


def asset():
    """🏺 The whole asset."""
    return {
        "id": SLUG, "category": "components",
        "title": T("Reclaimed Paving Slab Pallet", "Palette gebrauchter Gehwegplatten"),
        "summary": T("Twenty-two reclaimed concrete paving slabs 50 x 50 x 5 cm with chipped corners and a moss-darkened face, standing on edge in two strapped packs of eleven on a EUR pallet; lifted on fork tines, and six of them reinstalled as a 2 x 3 field on a sand bed under a moving wheel load.", "Zweiundzwanzig gebrauchte Gehwegplatten aus Beton 50 x 50 x 5 cm mit abgeplatzten Ecken und vom Moos gedunkelter Fläche, hochkant in zwei umreiften Paketen zu elf auf einer EUR-Palette; auf Gabelzinken gehoben, sechs davon als Feld 2 x 3 auf einem Sandbett unter einer wandernden Radlast wieder verlegt."),
        "tags": ["component", "reclaimed", "concrete", "paving", "slab", "pallet", "timber", "hardscape"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                       "notes": T(f"Slab 500 x 500 x 50 mm, {SLAB_MASS['rear-04']:.2f} kg unchipped at 2300 kg/m3; EUR pallet 1200 x 800 x 144 mm, {PALLET_MASS:.1f} kg; total {TOTAL_MASS:.1f} kg. Field joints 5 mm on a sand bed (DIN 18318).", f"Platte 500 x 500 x 50 mm, {SLAB_MASS['rear-04']:.2f} kg ohne Abplatzung bei 2300 kg/m³; EUR-Palette 1200 x 800 x 144 mm, {PALLET_MASS:.1f} kg; gesamt {TOTAL_MASS:.1f} kg; Feldfugen 5 mm auf Sandbett (DIN 18318)".replace(".", ",") + ".")},
        "dimensions": {"heightM": r6(DECK_TOP + EDGE), "widthM": PALLET_L, "depthM": PALLET_W, "massKg": round(TOTAL_MASS, 3)},
        "materials": MATERIALS, "rig": rig(), "volumes": volumes(), "anchors": anchors(),
        "representations": {str(s): representation(s) for s in SCALES},
        "symbols": [], "poses": poses(), "clips": clips(), "thermal": thermal(), "structure": structure(),
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }


def main():
    """✍️ Writes the asset document."""
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset()}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as handle:
        json.dump(doc, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print(f"[DEBUG] pallet {PALLET_MASS:.2f} kg, stock {sum(STOCK_MASS.values()):.2f} kg, field {sum(FIELD_MASS.values()):.2f} kg, bands {2 * BAND_MASS:.3f} kg, total {TOTAL_MASS:.2f} kg")


if __name__ == "__main__":
    main()
