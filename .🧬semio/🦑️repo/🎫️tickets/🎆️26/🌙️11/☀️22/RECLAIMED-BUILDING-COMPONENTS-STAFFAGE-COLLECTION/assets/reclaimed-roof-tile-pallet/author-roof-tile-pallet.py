"""🧱 Authoritative chisel of the staffage asset `reclaimed-roof-tile-pallet`: every hand-chosen number of the reclaimed clay plain tiles on their EUR pallet and of the double-lap patch they are reinstalled as, written out as the asset document `🔣️.json` next to this script.

Run: `python3 author-roof-tile-pallet.py` (prints the reference measurements of the model).

See ../../brief.md for the asset contract and ../../reports/reclaimed-roof-tile-pallet.md for the reference table.
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
SLUG = "reclaimed-roof-tile-pallet"
SCALES = ("50", "100", "200", "500")
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}

L = 0.375
W = 0.155
T = 0.014
HS = 0.020
NIB_L = 0.022
NIB_W = 0.020
NIB_H = 0.012
P = T + NIB_H
GAUGE = 0.150
SPLIT = L - GAUGE
RHO_CLAY = 2000.0
RHO_PALLET = 520.0
RHO_BATTEN = 450.0
RHO_PET = 1380.0

DECK = 0.144
TIER_BASE = {1: DECK, 2: DECK + W + 0.030}
TOP = TIER_BASE[2] + W
BATTEN_H = 0.030
BATTEN_W = 0.050
BATTEN_LEN = 1.2
ROW_X0 = (0.0125, 0.4125)
BATTEN_AT = (0.045, 0.1875, 0.330)
N_ROW = 42
N_BUNDLE = 14
Z0 = -0.532
BAND_W = 0.016
BAND_T = 0.0009
BAND_V = (0.022, 0.117)
TS = {"50": T, "100": 0.015, "200": 0.030, "500": 0.075}

PATCH = [((2, 0), (9, 15, 21)), ((2, 0), (12, 18, 24)), ((2, 1), (9, 15, 21)), ((2, 1), (12, 18, 24)), ((1, 0), (9, 15, 21))]
BATTEN_ROLE = {"batten-0": (0, 0), "batten-1": (0, 1), "batten-4": (0, 2), "batten-2": (1, 0), "batten-3": (1, 1), "eaves-batten": (1, 2)}
BATTENS = ["batten-0", "batten-1", "batten-2", "batten-3", "batten-4", "eaves-batten"]
RAFTER_Z = (-0.40, 0.40)
ALPHA = 45.0
DELTA = math.degrees(math.asin(T / GAUGE))
ROOF_X0 = 1.10
ROOF_Y0 = 1.70
SNOW_KNM2 = 0.4 * 0.85
QK = 1000.0
FILLET_RISE = 0.020

SIDE = ("right", "centre", "left")
X_ROWS = ((0.0, 0.1), (0.3275, 0.4725), (0.7, 0.8))
Z_ROWS = ((-0.6, -0.455), (-0.0725, 0.0725), (0.455, 0.6))
DECK_X = ((0.0, 0.145), (0.18625, 0.28625), (0.3275, 0.4725), (0.51375, 0.61375), (0.655, 0.8))
X_NAME = ("rear", "middle", "front")


def t(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r(v):
    """🎯 Rounds a coordinate to a hundredth of a millimetre."""
    return round(v + 0.0, 6)


def de_num(text):
    """🔢 German decimal commas in a number string."""
    return text.replace(".", ",")


def pts(points):
    """📐 Rounded point list."""
    return [[r(x), r(y)] for x, y in points]


def area(points):
    """📏 Shoelace area of a polygon (signed positive for counter-clockwise)."""
    return sum(points[i][0] * points[(i + 1) % len(points)][1] - points[(i + 1) % len(points)][0] * points[i][1] for i in range(len(points))) / 2


def perimeter(points):
    """📏 Perimeter of a closed polygon."""
    return sum(math.hypot(points[(i + 1) % len(points)][0] - points[i][0], points[(i + 1) % len(points)][1] - points[i][1]) for i in range(len(points)))


def ccw(points):
    """🔄 The ring counter-clockwise."""
    return points if area(points) > 0 else list(reversed(points))


def arc(n, chip_low=0.0, chip_high=0.0):
    """🌙 The segmental tail of an A tile from its lower to its upper corner (u, v), optionally chipped at a corner."""
    rad = (W * W / 4 + HS * HS) / (2 * HS)
    phi = math.asin((W / 2) / rad)
    out = []
    for k in range(n + 1):
        a = -phi + 2 * phi * k / n
        u, v = L - rad + rad * math.cos(a), W / 2 + rad * math.sin(a)
        if v < chip_low - 1e-9 or v > W - chip_high + 1e-9:
            continue
        out.append((u, v))
    return out


def arc_u(v):
    """🌙 The u of the segmental tail at height v."""
    rad = (W * W / 4 + HS * HS) / (2 * HS)
    return L - rad + math.sqrt(rad * rad - (v - W / 2) ** 2)


def face(scale, part="whole", chip=None, mirror=False):
    """🧱 The face outline (u, v) of a tile: head square at u = 0, segmental tail at u = L (A); `mirror` gives the B tile with its head at u = L; `part` cuts it at the end of the head lap."""
    seg = {"50": 12, "100": 6}.get(scale)
    lo, hi = chip if chip else (0.0, 0.0)
    if seg is None:
        tail = [(L, 0.0), (L, W)]
    else:
        tail = arc(seg, lo, hi)
        if lo > 0:
            tail = [(arc_u(0.0) - 0.014, 0.0)] + tail
        if hi > 0:
            tail = tail + [(arc_u(W) - 0.011, W)]
    if part == "whole":
        ring = [(0.0, 0.0)] + tail + [(0.0, W)]
    elif part == "head":
        ring = [(0.0, 0.0), (SPLIT, 0.0), (SPLIT, W), (0.0, W)]
    else:
        ring = [(SPLIT, 0.0)] + tail + [(SPLIT, W)]
    if mirror:
        ring = [(L - u, v) for u, v in ring]
    return pts(ccw(ring))


def nib_u(mirror):
    """🪝 The u range of the nib at the head of a tile."""
    return (L - NIB_L, L) if mirror else (0.0, NIB_L)


def z_of(k):
    """↔️ The lateral position of the back face of tile k of a row (its nib lies beyond, its exposed face 14 mm before)."""
    return Z0 + P * k


def bundle_span(b):
    """↔️ The lateral range of bundle b from the exposed face of its first tile to the nib of its last."""
    return z_of(N_BUNDLE * b) - T, z_of(N_BUNDLE * b + N_BUNDLE - 1) + NIB_H


def bundle_zc(b):
    """🎯 The lateral centre of bundle b (the origin of its bone)."""
    lo, hi = bundle_span(b)
    return (lo + hi) / 2


PATCH_TILES = []
for course, (slot, ks) in enumerate(PATCH):
    for j, k in enumerate(ks):
        PATCH_TILES.append({"id": f"tile-c{course}-{j}", "course": course, "column": j, "tier": slot[0], "row": slot[1], "k": k, "mirror": k % 2 == 1})
PATCH_KEYS = {(p["tier"], p["row"], p["k"]) for p in PATCH_TILES}


def bundle_id(tier, row, b):
    """🏷️ Bone id of a bundle."""
    return f"bundle-t{tier}-r{row}-b{b}"


BUNDLES = [(tier, row, b) for tier in (1, 2) for row in (0, 1) for b in range(3)]


def bundle_tiles(tier, row, b):
    """🧮 The tile indices of a bundle that stay in it (the patch tiles have bones of their own)."""
    return [k for k in range(N_BUNDLE * b, N_BUNDLE * b + N_BUNDLE) if (tier, row, k) not in PATCH_KEYS]


def runs(ks):
    """🔁 Runs of equal parity and step 2 among the tile indices: (first index, count)."""
    out = []
    for parity in (0, 1):
        seq = sorted(k for k in ks if k % 2 == parity)
        start, n = None, 0
        for k in seq:
            if start is not None and k == start + 2 * n:
                n += 1
            else:
                if start is not None:
                    out.append((start, n))
                start, n = k, 1
        if start is not None:
            out.append((start, n))
    return out


CHIPS = {(1, 0): (0.012, 0.0), (2, 1): (0.0, 0.010)}


def ex(id_, bone, material, tone, profile, plane, lo, hi, **kw):
    """🧊 An extrude solid."""
    s = {"id": id_, "kind": "extrude", "bone": bone, "material": material, "role": "body", "tone": tone, "profile": profile, "plane": plane, "from": r(lo), "to": r(hi)}
    s.update(kw)
    return s


def box(id_, bone, material, tone, size, at, **kw):
    """📦 A box solid centred at `at` in its bone frame."""
    s = {"id": id_, "kind": "box", "bone": bone, "material": material, "role": "body", "tone": tone, "size": [r(v) for v in size], "transform": {"translation": [r(v) for v in at]}}
    s.update(kw)
    return s


def wbox(id_, bone, material, tone, x, y, z, origin, **kw):
    """📦 A box given by its world ranges, expressed on a bone at `origin` with world-parallel axes."""
    return box(id_, bone, material, tone, [x[1] - x[0], y[1] - y[0], z[1] - z[0]], [(x[0] + x[1]) / 2 - origin[0], (y[0] + y[1]) / 2 - origin[1], (z[0] + z[1]) / 2 - origin[2]], **kw)


def rep(n, step):
    """🔁 A repeat along w."""
    return {"repeat": {"count": n, "translation": [0, 0, r(step)]}} if n > 1 else {}


PALLET_O = (0.0, DECK, 0.0)
PALLET_50 = []
for i, (x0, x1) in enumerate(X_ROWS):
    PALLET_50.append(("bottom-board-" + X_NAME[i], "bottom", (x0, x1), (0.0, 0.022), (-0.6, 0.6)))
for i, (x0, x1) in enumerate(X_ROWS):
    for k, (z0, z1) in enumerate(Z_ROWS):
        PALLET_50.append((f"block-{X_NAME[i]}-{SIDE[k]}", "block", (x0, x1), (0.022, 0.1), (z0, z1)))
for k, (z0, z1) in enumerate(Z_ROWS):
    PALLET_50.append((f"cross-board-{SIDE[k]}", "cross", (0.0, 0.8), (0.1, 0.122), (z0, z1)))
for i, (x0, x1) in enumerate(DECK_X):
    PALLET_50.append((f"deck-board-{i + 1}", "deck", (x0, x1), (0.122, DECK), (-0.6, 0.6)))
PALLET_TONE = {"bottom": 0.4, "block": 0.36, "cross": 0.44, "deck": 0.48, "runner": 0.4, "plate": 0.46, "foot": 0.38, "mass": 0.42}


def runner_profile(i):
    """🪵 The (v, w) profile of a runner of 1:100: one bottom board with its three blocks, in the pallet frame."""
    lo, hi = -DECK, 0.1 - DECK
    b = 0.022 - DECK
    ring = [(lo, -0.6), (lo, 0.6), (hi, 0.6), (hi, Z_ROWS[2][0]), (b, Z_ROWS[2][0]), (b, Z_ROWS[1][1]), (hi, Z_ROWS[1][1]), (hi, Z_ROWS[1][0]), (b, Z_ROWS[1][0]), (b, Z_ROWS[0][1]), (hi, Z_ROWS[0][1]), (hi, -0.6)]
    return pts(ccw(ring))


def pallet_parts(scale):
    """🪵 The world boxes or profiles the pallet of a scale is built and drawn from (the same numbers for solids and shapes)."""
    if scale == "50":
        return [{"id": p[0], "kind": p[1], "x": p[2], "y": p[3], "z": p[4]} for p in PALLET_50]
    if scale == "100":
        out = [{"id": f"runner-{X_NAME[i]}", "kind": "runner", "x": X_ROWS[i], "y": (0.0, 0.1), "z": (-0.6, 0.6), "i": i} for i in range(3)]
        out.append({"id": "upper-plate", "kind": "plate", "x": (0.0, 0.8), "y": (0.1, DECK), "z": (-0.6, 0.6)})
        return out
    if scale == "200":
        out = [{"id": "upper-plate", "kind": "plate", "x": (0.0, 0.8), "y": (0.1, DECK), "z": (-0.6, 0.6)}]
        for i, x in enumerate(X_ROWS):
            for k, z in enumerate(Z_ROWS):
                out.append({"id": f"foot-{X_NAME[i]}-{SIDE[k]}", "kind": "foot", "x": x, "y": (0.0, 0.1), "z": z})
        return out
    return [{"id": "pallet-mass", "kind": "mass", "x": (0.0, 0.8), "y": (0.0, DECK), "z": (-0.6, 0.6)}]


def pallet_body(scale):
    """🪵 Body solids of the pallet."""
    out = []
    for p in pallet_parts(scale):
        if p["kind"] == "runner":
            out.append(ex(p["id"], "pallet", "pallet-timber", PALLET_TONE["runner"], runner_profile(p["i"]), "yz", p["x"][0], p["x"][1]))
        else:
            out.append(wbox(p["id"], "pallet", "pallet-timber", PALLET_TONE[p["kind"]], p["x"], p["y"], p["z"], PALLET_O))
    return out


def bundle_origin(tier, row, b):
    """📍 World rest origin of a bundle bone: row start, tier base, lateral centre."""
    return (ROW_X0[row], TIER_BASE[tier], bundle_zc(b))


def tile_solids(prefix, bone, scale, tier, row, b, kind):
    """🧱 The 1:50 tiles of a bundle as runs of repeated extrudes and nibs (`kind` body, head or tail)."""
    out = []
    zc = bundle_zc(b)
    ks = bundle_tiles(tier, row, b)
    for k0, n in runs(ks):
        mirror = k0 % 2 == 1
        w1 = z_of(k0) - zc
        chip = CHIPS.get((tier, row)) if (k0 == 0 and b == 0) else None
        if chip is not None and n > 1:
            out += tile_one(prefix, bone, scale, k0, mirror, w1, chip, kind)
            out += tile_run(prefix, bone, scale, k0 + 2, n - 1, mirror, w1 + 2 * P, kind)
        else:
            out += tile_run(prefix, bone, scale, k0, n, mirror, w1, kind)
    return out


def tile_one(prefix, bone, scale, k, mirror, w1, chip, kind):
    """🧱 One chipped tile (the outermost of a row)."""
    return tile_run(prefix, bone, scale, k, 1, mirror, w1, kind, chip)


def tile_run(prefix, bone, scale, k0, n, mirror, w1, kind, chip=None):
    """🧱 A run of n tiles of equal orientation, 52 mm apart, each with its nib."""
    out = []
    tag = f"{prefix}k{k0:02d}"
    nu = nib_u(mirror)
    nib = box(f"{tag}-nib", bone, "clay-lapped", 0.5, [NIB_L, NIB_W, NIB_H], [(nu[0] + nu[1]) / 2, W / 2, w1 + NIB_H / 2], **rep(n, 2 * P))
    if kind == "body":
        out.append(ex(f"{tag}", bone, "clay-weathered", 0.5 if not mirror else 0.54, face(scale, "whole", chip, mirror), "xy", w1 - T, w1, **rep(n, 2 * P)))
        out.append(nib)
    else:
        sf_head = ["-u"] if mirror else ["+u"]
        sf_tail = ["+u"] if mirror else ["-u"]
        out.append(ex(f"{tag}-head", bone, "clay-lapped", 0.6, face(scale, "head", chip, mirror), "xy", w1 - T, w1, region=f"{kind}-lapped", sectionFaces=sf_head, **rep(n, 2 * P)))
        out.append(ex(f"{tag}-tail", bone, "clay-weathered", 0.4, face(scale, "tail", chip, mirror), "xy", w1 - T, w1, region=f"{kind}-weathered", sectionFaces=sf_tail, **rep(n, 2 * P)))
        nib["region"] = f"{kind}-lapped"
        out.append(nib)
    return out


def bundle_box(bone, b, scale, id_, nibs=True, **kw):
    """📦 The envelope of a bundle: 375 x 155 mm by the bundle length, with or without the nib of its last tile."""
    lo, hi = bundle_span(b)
    hi = hi if nibs else hi - NIB_H
    zc = bundle_zc(b)
    return box(id_, bone, "clay-weathered", 0.5, [L, W, hi - lo], [L / 2, W / 2, (lo + hi) / 2 - zc], **kw)


def straps(bone, b, prefix, scale):
    """🎗️ The two PET strap loops of a bundle at 22 and 117 mm above its base: each loop one ring of 16 x 0.9 mm band around the bundle (a plan profile with the bundle as its hole, extruded through the band width)."""
    lo, hi = bundle_span(b)
    zc = bundle_zc(b)
    w_lo, w_hi = lo - zc, z_of(N_BUNDLE * b + N_BUNDLE - 1) - zc
    outer = [(w_lo - BAND_T, -BAND_T), (w_lo - BAND_T, L + BAND_T), (w_hi + BAND_T, L + BAND_T), (w_hi + BAND_T, -BAND_T)]
    inner = [(w_lo, 0.0), (w_lo, L), (w_hi, L), (w_hi, 0.0)]
    return [ex(f"{prefix}{i}", bone, "pet-strap", 0.22, pts(ccw(outer)), "zx", v0, v0 + BAND_W, holes=[pts(ccw(inner))], **({"slender": True} if scale in ("200", "500") else {})) for i, v0 in enumerate(BAND_V)]


def batten_solid(bone, scale, prefix=""):
    """🪵 A roof batten 30 x 50 x 1200 mm, its origin at the upper edge of its leading face."""
    kw = {"slender": True} if scale == "500" else {}
    return box(f"{prefix}{bone}", bone, "batten-timber", 0.62, [BATTEN_W, BATTEN_H, BATTEN_LEN], [BATTEN_W / 2, -BATTEN_H / 2, 0.0], **kw)


def patch_solids(p, scale, kind):
    """🧱 A patch tile on its own bone: the tile and its nib (body), or its lapped head and weathered tail (thermal)."""
    th = TS[scale]
    bone = p["id"]
    out = []
    if kind == "body":
        out.append(ex(f"{bone}-tile", bone, "clay-weathered", 0.5, face(scale), "xy", -th, 0.0))
        if scale == "50":
            out.append(box(f"{bone}-nib", bone, "clay-lapped", 0.5, [NIB_L, NIB_W, NIB_H], [NIB_L / 2, W / 2, NIB_H / 2]))
    else:
        out.append(ex(f"th-{bone}-head", bone, "clay-lapped", 0.6, face(scale, "head"), "xy", -th, 0.0, region="patch-lapped", sectionFaces=["+u"]))
        out.append(ex(f"th-{bone}-tail", bone, "clay-weathered", 0.4, face(scale, "tail"), "xy", -th, 0.0, region="patch-weathered", sectionFaces=["-u"]))
        if scale == "50":
            out.append(box(f"th-{bone}-nib", bone, "clay-lapped", 0.5, [NIB_L, NIB_W, NIB_H], [NIB_L / 2, W / 2, NIB_H / 2], region="patch-lapped"))
    return out


def body(scale):
    """🧊 The body list of one scale."""
    out = pallet_body(scale)
    for tier, row, b in BUNDLES:
        bone = bundle_id(tier, row, b)
        if scale == "50":
            out += tile_solids(f"{bone}-", bone, scale, tier, row, b, "body")
        else:
            out.append(bundle_box(bone, b, scale, f"{bone}-pack", envelope=True))
        if scale in ("50", "100"):
            out += straps(bone, b, f"{bone}-strap-", scale)
    for bat in BATTENS:
        out.append(batten_solid(bat, scale))
    for p in PATCH_TILES:
        out += patch_solids(p, scale, "body")
    return out


def thermal(scale, bodies):
    """🌡️ The thermal list of one scale: pallet, battens and straps by reference, the tiles split into their lapped heads and weathered tails."""
    out = []
    for s in bodies:
        if s["material"] == "pallet-timber":
            out.append({"id": f"th-{s['id']}", "of": s["id"], "region": "pallet-timber"})
        elif s["material"] == "batten-timber":
            out.append({"id": f"th-{s['id']}", "of": s["id"], "region": "battens"})
        elif s["material"] == "pet-strap":
            out.append({"id": f"th-{s['id']}", "of": s["id"], "region": "strapping"})
    for tier, row, b in BUNDLES:
        bone = bundle_id(tier, row, b)
        if scale == "50":
            out += tile_solids(f"th-{bone}-", bone, scale, tier, row, b, "stack")
        else:
            lo, hi = bundle_span(b)
            zc = bundle_zc(b)
            slabs = (("rear", 0.0, GAUGE, "stack-weathered"), ("middle", GAUGE, SPLIT, "stack-lapped"), ("front", SPLIT, L, "stack-weathered")) if scale == "100" else (("rear", 0.0, L / 2, "stack-weathered"), ("front", L / 2, L, "stack-lapped"))
            for name, u0, u1, region in slabs:
                out.append(box(f"th-{bone}-{name}", bone, "clay-weathered" if region.endswith("weathered") else "clay-lapped", 0.5, [u1 - u0, W, hi - lo], [(u0 + u1) / 2, W / 2, (lo + hi) / 2 - zc], region=region, envelope=True))
            if scale in ("200", "500"):
                for s in straps(bone, b, f"th-{bone}-strap-", scale):
                    s["region"] = "strapping"
                    out.append(s)
    for p in PATCH_TILES:
        out += patch_solids(p, scale, "thermal")
    return out


def structure(scale):
    """🏗️ The structure list of one scale: pallet members and joints, one member per bundle, the battens and the patch tiles as members."""
    out = []
    if scale == "50":
        for p in PALLET_50:
            if p[1] in ("deck", "cross"):
                out.append(wbox(f"st-{p[0]}", "pallet", "pallet-timber", 0.5, p[2], p[3], p[4], PALLET_O, member="pallet", part="member"))
        for i, x in enumerate(X_ROWS):
            for k, z in enumerate(Z_ROWS):
                out.append(wbox(f"st-joint-{X_NAME[i]}-{SIDE[k]}", "pallet", "pallet-timber", 0.4, x, (0.0, 0.1), z, PALLET_O, member="pallet", part="joint"))
    elif scale == "100":
        out.append(wbox("st-deck", "pallet", "pallet-timber", 0.5, (0.0, 0.8), (0.122, DECK), (-0.6, 0.6), PALLET_O, member="pallet", part="member"))
        for k, z in enumerate(Z_ROWS):
            out.append(wbox(f"st-cross-{SIDE[k]}", "pallet", "pallet-timber", 0.45, (0.0, 0.8), (0.1, 0.122), z, PALLET_O, member="pallet", part="member"))
        for i, x in enumerate(X_ROWS):
            for k, z in enumerate(Z_ROWS):
                out.append(wbox(f"st-joint-{X_NAME[i]}-{SIDE[k]}", "pallet", "pallet-timber", 0.4, x, (0.0, 0.1), z, PALLET_O, member="pallet", part="joint"))
    elif scale == "200":
        out.append({"id": "st-upper-plate", "of": "upper-plate", "member": "pallet", "part": "member"})
        for i in range(3):
            for k in range(3):
                out.append({"id": f"st-foot-{X_NAME[i]}-{SIDE[k]}", "of": f"foot-{X_NAME[i]}-{SIDE[k]}", "member": "pallet", "part": "joint"})
    else:
        out.append(wbox("st-pallet-deck", "pallet", "pallet-timber", 0.5, (0.0, 0.8), (0.069, DECK), (-0.6, 0.6), PALLET_O, member="pallet", part="member"))
        for k, z in enumerate(Z_ROWS):
            out.append(wbox(f"st-pallet-leg-{SIDE[k]}", "pallet", "pallet-timber", 0.4, (0.0, 0.8), (0.0, DECK), z, PALLET_O, member="pallet", part="joint"))
    for tier, row, b in BUNDLES:
        bone = bundle_id(tier, row, b)
        if scale == "50":
            out.append(bundle_box(bone, b, scale, f"st-{bone}", nibs=False, member=bone, part="member"))
        else:
            out.append({"id": f"st-{bone}", "of": f"{bone}-pack", "member": bone, "part": "member"})
    for bat in BATTENS:
        out.append({"id": f"st-{bat}", "of": bat, "member": bat, "part": "member"})
    for p in PATCH_TILES:
        out.append({"id": f"st-{p['id']}", "of": f"{p['id']}-tile", "member": p["id"], "part": "member"})
    return out


def shape(id_, bone, role, z, filled, tone, geometry, **kw):
    """✏️ One 2D shape."""
    s = {"id": id_, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geometry)
    s.update(kw)
    return s


def poly(points, closed=True):
    """⬠ A poly geometry."""
    return {"kind": "poly", "points": pts(points), "closed": closed}


def rect(x0, x1, y0, y1):
    """▭ A rectangle as a closed poly."""
    return poly([(x0, y0), (x1, y0), (x1, y1), (x0, y1)])


def circle(cx, cy, rad, n=10):
    """⭕ A small circle as a closed poly."""
    return poly([(cx + rad * math.cos(2 * math.pi * i / n), cy + rad * math.sin(2 * math.pi * i / n)) for i in range(n)])


class Rand:
    """🎲 A tiny deterministic generator so the hand strokes are the same on every run."""

    def __init__(self, seed):
        self.s = seed

    def __call__(self):
        self.s = (self.s * 1103515245 + 12345) % 2147483648
        return self.s / 2147483648


def lichen(prefix, bone, rnd, u0, u1, n, region):
    """🍂 Lichen rosettes on the weathered tail: short curled texture strokes."""
    out = []
    for i in range(n):
        cu = u0 + 0.015 + rnd() * (u1 - u0 - 0.03)
        cv = 0.02 + rnd() * (W - 0.04)
        rad = 0.006 + 0.008 * rnd()
        a0 = rnd() * 6.28
        ring = [(cu + rad * math.cos(a0 + 0.9 * j) * (1 + 0.25 * math.sin(3 * j)), cv + rad * math.sin(a0 + 0.9 * j)) for j in range(7)]
        out.append(shape(f"{prefix}-{i:02d}", bone, "texture", 3.5, False, 0.25, poly(ring, closed=False), region=region))
    return out


SLOT_TONE = {(1, 0): 0.0, (1, 1): 0.03, (2, 0): -0.02, (2, 1): 0.02}


def elevation(scale):
    """👁️ Side elevation of one scale (bone-local metres): the pallet end, the outermost tile of each row with its lichen tail, the batten ends and the straps."""
    rnd = Rand({"50": 11, "100": 23, "200": 37, "500": 41}[scale])
    out = pallet_elevation(scale)
    for tier in (1, 2):
        for row in (0, 1):
            bone = bundle_id(tier, row, 0)
            tone = 0.56 + SLOT_TONE[(tier, row)]
            reg = {"regions": ["stack-lapped", "stack-weathered"], "material": "clay-weathered"}
            if scale == "500":
                lo = -0.030 if tier == 2 else 0.0
                regs = {"regions": ["stack-lapped", "stack-weathered", "patch-lapped", "patch-weathered", "strapping"] + (["battens"] if tier == 2 else []), "material": "clay-weathered"}
                out.append(shape(f"{bone}-row", bone, "body", 2, True, 0.5, rect(0.0, L, lo, W), **regs))
                continue
            out.append(shape(f"{bone}-row", bone, "body", 2, True, tone, rect(0.0, L, 0.0, W), **(reg if scale != "200" else {**reg, "regions": reg["regions"] + ["strapping"]})))
            chip = CHIPS.get((tier, row)) if scale == "50" else None
            out.append(shape(f"{bone}-k00-head", bone, "body", 2.2, True, tone + 0.06, poly(face(scale, "head", chip)), region="stack-lapped", material="clay-lapped"))
            out.append(shape(f"{bone}-k00-tail", bone, "body", 2.2, True, tone - 0.2, poly(face(scale, "tail", chip)), region="stack-weathered", material="clay-weathered"))
            if scale in ("50", "100"):
                out.append(shape(f"{bone}-k00-edge", bone, "detail", 2.4, False, 0.3, poly(face(scale, "whole", chip)), region="stack-weathered"))
                out += lichen(f"{bone}-lichen", bone, rnd, SPLIT, L - 0.01, 9 if scale == "50" else 4, "stack-weathered")
            if scale == "50":
                for i, (cu, cv) in enumerate(((0.035, 0.045), (0.035, 0.11))):
                    out.append(shape(f"{bone}-nail-{i}", bone, "detail", 2.5, False, 0.2, circle(cu, cv, 0.0025, 8), region="stack-lapped"))
                out.append(shape(f"{bone}-lap-line", bone, "texture", 2.5, False, 0.3, poly([(SPLIT, 0.006), (SPLIT + 0.002, W / 2), (SPLIT, W - 0.006)], closed=False), region="stack-weathered"))
                if (tier, row) == (2, 0):
                    out.append(shape(f"{bone}-mortar", bone, "texture", 2.6, False, 0.15, poly([(0.30, 0.13), (0.318, 0.142), (0.342, 0.138), (0.352, 0.124), (0.336, 0.118), (0.31, 0.121)], closed=False), region="stack-weathered"))
            out += strap_elevation(bone, scale)
    for bat in BATTENS:
        if scale != "500":
            out.append(shape(f"{bat}-end", bat, "body", 1.8, True, 0.62, rect(0.0, BATTEN_W, -BATTEN_H, 0.0), region="battens", material="batten-timber"))
            if scale == "50":
                out.append(shape(f"{bat}-ring", bat, "texture", 1.9, False, 0.3, poly([(0.008, -0.004), (0.02, -0.012), (0.034, -0.016), (0.046, -0.025)], closed=False), region="battens"))
    out += patch_shapes(scale, "elevation")
    return out


def strap_elevation(bone, scale):
    """🎗️ The near straps of a bundle seen broadside in the side elevation."""
    out = []
    for i, v0 in enumerate(BAND_V):
        if scale in ("50", "100"):
            out.append(shape(f"{bone}-strap-{i}", bone, "detail", 3, True, 0.22, rect(-BAND_T, L + BAND_T, v0, v0 + BAND_W), region="strapping", material="pet-strap"))
    return out


def pallet_elevation(scale):
    """🪵 The pallet end seen from the right (-z): every part that reaches the near end."""
    out = []
    reg = {"region": "pallet-timber", "material": "pallet-timber"}
    for p in pallet_parts(scale):
        if p["z"][0] <= -0.6 + 1e-9:
            top = 0.122 if (scale == "50" and p["kind"] == "block") else p["y"][1]
            out.append(shape(f"{p['id']}", "pallet", "body", 1, True, PALLET_TONE[p["kind"]], rect(p["x"][0], p["x"][1], p["y"][0] - DECK, top - DECK), **reg))
    if scale == "50":
        for i, (x0, x1) in enumerate(X_ROWS):
            out.append(shape(f"block-grain-{i}", "pallet", "texture", 1.5, False, 0.3, poly([(x0 + 0.02, 0.06 - DECK), ((x0 + x1) / 2, 0.064 - DECK), (x1 - 0.02, 0.058 - DECK)], closed=False), region="pallet-timber"))
    return out


def front(scale):
    """👁️ Front elevation of one scale (asset rest metres, x lateral): the tile ends of both tiers with their nibs in the gaps, the straps, the battens and the pallet's long face."""
    out = []
    reg = {"region": "pallet-timber", "material": "pallet-timber"}
    for p in pallet_parts(scale):
        if p["x"][1] >= 0.8 - 1e-9:
            if p["kind"] == "runner":
                ring = [(w, v + DECK) for v, w in runner_profile(p["i"])]
                out.append(shape(p["id"], "pallet", "body", 1, True, PALLET_TONE["runner"], poly(ring), **reg))
            else:
                top = 0.122 if (scale == "50" and p["kind"] == "block") else p["y"][1]
                out.append(shape(p["id"], "pallet", "body", 1 + (0.1 if p["kind"] == "deck" else 0), True, PALLET_TONE[p["kind"]], rect(p["z"][0], p["z"][1], p["y"][0], top), **reg))
    for tier, row, b in BUNDLES:
        bone = bundle_id(tier, row, b)
        base = TIER_BASE[tier]
        reg = {"regions": ["stack-lapped", "stack-weathered"], "material": "clay-weathered"}
        lo, hi = bundle_span(b)
        zr = 2 + 0.1 * row
        if scale == "50":
            for k in bundle_tiles(tier, row, b):
                zk = z_of(k)
                v1, v2 = base + W / 2 - NIB_W / 2, base + W / 2 + NIB_W / 2
                ring = [(zk - T, base), (zk, base), (zk, v1), (zk + NIB_H, v1), (zk + NIB_H, v2), (zk, v2), (zk, base + W), (zk - T, base + W)]
                out.append(shape(f"{bone}-k{k:02d}-end", bone, "body", zr, True, 0.5 + 0.04 * (k % 3), poly(ring), penMm=0.1, **reg))
        elif scale == "500":
            b0 = base - 0.030 if tier == 2 else base
            out.append(shape(f"{bone}-pack", bone, "body", zr, True, 0.5, rect(lo, hi, b0, base + W), **reg))
        else:
            out.append(shape(f"{bone}-pack", bone, "body", zr, True, 0.5, rect(lo, hi, base, base + W), **reg))
            if scale == "100" and row == 1:
                for k in range(N_BUNDLE * b + 1, N_BUNDLE * b + N_BUNDLE):
                    out.append(shape(f"{bone}-joint-{k:02d}", bone, "texture", zr + 0.05, False, 0.3, poly([(z_of(k) - T, base + 0.004), (z_of(k) - T, base + W - 0.004)], closed=False), region="stack-lapped"))
        if row == 1 and scale in ("50", "100"):
            w_lo, w_hi = lo - BAND_T, z_of(N_BUNDLE * b + N_BUNDLE - 1) + BAND_T
            for i, v0 in enumerate(BAND_V):
                if scale in ("50", "100"):
                    out.append(shape(f"{bone}-strap-{i}", bone, "detail", 3, True, 0.22, rect(w_lo, w_hi, base + v0, base + v0 + BAND_W), region="strapping", material="pet-strap"))
                else:
                    out.append(shape(f"{bone}-strap-{i}", bone, "detail", 3, False, 0.22, poly([(w_lo, base + v0 + BAND_W / 2), (w_hi, base + v0 + BAND_W / 2)], closed=False), region="strapping"))
    if scale != "500":
        for bat in BATTENS:
            out.append(shape(f"{bat}-side", bat, "body", 1.8, True, 0.62, poly([(0.0, -BATTEN_H), (BATTEN_W, -BATTEN_H), (BATTEN_W, 0.0), (0.0, 0.0)]), solid={"kind": "extruded", "sweep": "w", "z0": -0.6, "z1": 0.6}, region="battens", material="batten-timber"))
    out += patch_shapes(scale, "front")
    return out


def plan(scale):
    """👁️ Plan of one scale (asset rest metres, y lateral): the deck, the battens through the gaps, the top edges of both tiers with their nibs and the strap loops."""
    out = []
    reg = {"region": "pallet-timber", "material": "pallet-timber"}
    for p in pallet_parts(scale):
        if p["kind"] in ("bottom", "cross", "deck", "runner", "plate", "mass"):
            pz = {"bottom": 0.4, "runner": 0.4, "cross": 0.6, "deck": 1.0, "plate": 1.0, "mass": 1.0}[p["kind"]]
            out.append(shape(p["id"], "pallet", "body", pz, True, PALLET_TONE[p["kind"]], rect(p["x"][0], p["x"][1], p["z"][0], p["z"][1]), **reg))
    if scale != "500":
        for bat in BATTENS:
            out.append(shape(f"{bat}-top", bat, "body", 2.5, True, 0.62, poly([(0.0, -BATTEN_H), (BATTEN_W, -BATTEN_H), (BATTEN_W, 0.0), (0.0, 0.0)]), solid={"kind": "extruded", "sweep": "w", "z0": -0.6, "z1": 0.6}, region="battens", material="batten-timber"))
    for tier, row, b in BUNDLES:
        bone = bundle_id(tier, row, b)
        x0 = ROW_X0[row]
        reg = {"regions": ["stack-lapped", "stack-weathered"], "material": "clay-weathered"}
        lo, hi = bundle_span(b)
        zr = 2 if tier == 1 else 3
        if scale == "50":
            for k in bundle_tiles(tier, row, b):
                zk = z_of(k)
                nu = nib_u(k % 2 == 1)
                if k % 2 == 0:
                    ring = [(x0, zk - T), (x0 + L, zk - T), (x0 + L, zk), (x0 + nu[1], zk), (x0 + nu[1], zk + NIB_H), (x0, zk + NIB_H)]
                else:
                    ring = [(x0, zk - T), (x0 + L, zk - T), (x0 + L, zk + NIB_H), (x0 + nu[0], zk + NIB_H), (x0 + nu[0], zk), (x0, zk)]
                out.append(shape(f"{bone}-k{k:02d}-top", bone, "body", zr, True, 0.5 + 0.04 * (k % 3), poly(ring), penMm=0.1, **reg))
        else:
            out.append(shape(f"{bone}-pack", bone, "body", zr, True, 0.5, rect(x0, x0 + L, lo, hi), **reg))
            if scale == "100" and tier == 2:
                for k in range(N_BUNDLE * b + 1, N_BUNDLE * b + N_BUNDLE):
                    out.append(shape(f"{bone}-joint-{k:02d}", bone, "texture", zr + 0.05, False, 0.3, poly([(x0 + 0.004, z_of(k) - T), (x0 + L - 0.004, z_of(k) - T)], closed=False), region="stack-lapped"))
        if tier == 2 and scale in ("50", "100"):
            w_lo, w_hi = lo - BAND_T / 2, z_of(N_BUNDLE * b + N_BUNDLE - 1) + BAND_T / 2
            out.append(shape(f"{bone}-strap-loop", bone, "detail", 4, False, 0.22, poly([(x0 - BAND_T / 2, w_lo), (x0 + L + BAND_T / 2, w_lo), (x0 + L + BAND_T / 2, w_hi), (x0 - BAND_T / 2, w_hi)]), region="strapping"))
    out += patch_shapes(scale, "plan")
    return out


def patch_shapes(scale, view):
    """🧱 The patch tiles in every view as `solid` placements of their face outline swept through their thickness, so they read right standing in the stack and lying on the battens."""
    out = []
    if scale == "500":
        return out
    th = TS[scale]
    zr = {"elevation": 1.2, "front": 1.5 if scale == "500" else 2.5, "plan": 1.5 if scale == "500" else 3.5}[view]
    for p in PATCH_TILES:
        bone = p["id"]
        zk = z_of(p["k"])
        sol = {"kind": "extruded", "sweep": "w", "z0": r(zk - th), "z1": r(zk)}
        if scale in ("50", "100") and view == "elevation":
            out.append(shape(f"{bone}-head", bone, "body", zr, True, 0.62, poly(face(scale, "head")), solid=dict(sol), region="patch-lapped", material="clay-lapped"))
            out.append(shape(f"{bone}-tail", bone, "body", zr, True, 0.38, poly(face(scale, "tail")), solid=dict(sol), region="patch-weathered", material="clay-weathered"))
        else:
            out.append(shape(f"{bone}-tile", bone, "body", zr, True, 0.5, poly(face(scale)), solid=dict(sol), regions=["patch-lapped", "patch-weathered"], material="clay-weathered", **({"penMm": 0.1} if scale == "50" and view != "elevation" else {})))
        if scale == "50" and view != "elevation":
            out.append(shape(f"{bone}-nib", bone, "body", zr, True, 0.5, poly([(0.0, W / 2 - NIB_W / 2), (NIB_L, W / 2 - NIB_W / 2), (NIB_L, W / 2 + NIB_W / 2), (0.0, W / 2 + NIB_W / 2)]), solid={"kind": "extruded", "sweep": "w", "z0": r(zk), "z1": r(zk + NIB_H)}, region="patch-lapped", material="clay-lapped", penMm=0.1))
    return out


PENS = {
    "50": {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
    "100": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35},
    "200": {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25},
    "500": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18},
}
RANGES = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}


def representation(scale):
    """🖼️ The authored representation of one preset scale."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": int(scale), "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}, "decisions": REP_DECISIONS[scale]}


REP_DECISIONS = {
    "50": {
        "silhouette": t(
            "Side elevation of the loaded EUR pallet (800 mm side, 16 x 9.7 mm on paper): the pallet end with its deck boards, cross board, three blocks and bottom boards, the two rows of tiles standing on their long edge in two tiers, the outermost tile of each row with its segmental tail (20 mm rise) and its lichen-darkened lower 150 mm, the six batten ends between the tiers and the two strap bands of each row. Front view: the 1.2 m face with every tile end of both tiers (14 mm thick, 12 mm nib gap), the nibs in the gaps, the strap bands across the three bundles of a row and the battens' long face. Plan: the top edges of both tiers with their nibs, the strap loops and the deck boards and battens between them.",
            "Seitenansicht der beladenen EUR-Palette (Seite 800 mm, 16 x 9,7 mm auf Papier): das Palettenende mit Deckbrettern, Querbrett, drei Klötzen und Bodenbrettern, die zwei Reihen hochkant auf der Langkante stehender Ziegel in zwei Lagen, der äußerste Ziegel jeder Reihe mit seinem Segmentschnitt (20 mm Stich) und seinen von Flechten gedunkelten unteren 150 mm, die sechs Lattenenden zwischen den Lagen und die zwei Umreifungsbänder jeder Reihe. Vorderansicht: die 1,2 m lange Seite mit jedem Ziegelende beider Lagen (14 mm dick, 12 mm Nasenspalt), den Nasen in den Spalten, den Bändern über die drei Bündel einer Reihe und der Längsseite der Latten. Draufsicht: die Oberkanten beider Lagen mit ihren Nasen, die Bandschlaufen und dazwischen Deckbretter und Latten."),
        "retained": t(
            "Every tile of the 168 at its true place and section, the nib of each in its gap, the alternating head ends of a row, the chipped tail corners of two outer tiles, the two nail holes at the head, the lime mortar trace of the old verge on one tile, the lap line 225 mm from the head that divides the clean lapped head from the weathered exposed tail, and the fifteen patch tiles on their own bones in their places in the stack.",
            "Jeder der 168 Ziegel an seiner wahren Stelle und mit seinem Querschnitt, die Nase jedes Ziegels in ihrem Spalt, die wechselnden Kopfenden einer Reihe, die abgeplatzten Fußecken zweier äußerer Ziegel, die zwei Nagellöcher am Kopf, die Kalkmörtelspur des alten Ortgangs auf einem Ziegel, die Überdeckungslinie 225 mm vom Kopf, die den sauberen überdeckten Kopf vom bewitterten sichtbaren Fuß trennt, und die fünfzehn Musterziegel auf eigenen Knochen an ihren Plätzen im Stapel."),
        "merged": t("In the side elevation the faces of a row are one outline behind the outermost tile; only the corners of the second tile show beside the segmental tail.", "In der Seitenansicht sind die Flächen einer Reihe eine Kontur hinter dem äußersten Ziegel; nur die Ecken des zweiten Ziegels erscheinen neben dem Segmentschnitt."),
        "omitted": t("The slight camber of the tiles, the inner rows' faces in the side elevation and the hidden faces of the pallet blocks.", "Die leichte Wölbung der Ziegel, die Flächen der inneren Ziegel in der Seitenansicht und die verdeckten Flächen der Palettenklötze."),
        "thinElements": t("The tile ends and tops are filled strips of 0.28 mm with a 0.1 mm pen so the 0.24 mm nib gaps stay open; lichen, lap line and mortar are 0.1 mm texture strokes; straps 0.32 mm filled bands.", "Ziegelenden und Oberkanten sind gefüllte Streifen von 0,28 mm mit einer Feder von 0,1 mm, damit die Nasenspalte von 0,24 mm offen bleiben; Flechten, Überdeckungslinie und Mörtel sind Texturstriche von 0,1 mm; Bänder gefüllte Streifen von 0,32 mm."),
        "hatching": t("No hatch: the views are not cuts; tone and lichen strokes stand for the surface.", "Keine Schraffur: die Ansichten sind keine Schnitte; Ton und Flechtenstriche stehen für die Oberfläche."),
        "styles": t("Outline 0.35 mm, details 0.18 mm, texture 0.1 mm; the weathered tail is the darkest tile tone, the lapped head the palest.", "Kontur 0,35 mm, Details 0,18 mm, Textur 0,1 mm; der bewitterte Fuß ist der dunkelste Ziegelton, der überdeckte Kopf der hellste."),
        "overlays": t("Each tile, nib, strap, batten and pallet part is its own cell.", "Jeder Ziegel, jede Nase, jedes Band, jede Latte und jedes Palettenteil ist eine eigene Zelle."),
    },
    "100": {
        "silhouette": t(
            "The loaded pallet (8 x 4.8 mm in the side elevation, 12 x 4.8 mm in the front view): the outermost tile of each row with segmental tail and dark tail band, the batten ends, the straps; in front and plan each strapped bundle of 14 tiles is one block with its tile joints as texture lines.",
            "Die beladene Palette (8 x 4,8 mm in der Seitenansicht, 12 x 4,8 mm in der Vorderansicht): der äußerste Ziegel jeder Reihe mit Segmentschnitt und dunklem Fußband, die Lattenenden, die Bänder; in Vorderansicht und Draufsicht ist jedes umreifte Bündel aus 14 Ziegeln ein Block mit seinen Ziegelfugen als Texturlinien."),
        "retained": t("Bundles, straps, battens, pallet runners, cross and deck boards at their true places; the patch tiles keep their own outlines.", "Bündel, Bänder, Latten, Palettenkufen, Quer- und Deckbretter an ihrer wahren Stelle; die Musterziegel behalten ihre eigenen Umrisse."),
        "merged": t("The 12 mm nib gaps (0.12 mm on paper) close: a bundle is one block; bottom board and blocks of a runner are one outline.", "Die Nasenspalte von 12 mm (0,12 mm auf Papier) schließen sich: ein Bündel ist ein Block; Bodenbrett und Klötze einer Kufe sind eine Kontur."),
        "omitted": t("Nibs, nail holes, chips and the mortar trace.", "Nasen, Nagellöcher, Abplatzungen und die Mörtelspur."),
        "thinElements": t("Tile joints and lichen are 0.1 mm texture strokes; straps 0.16 mm filled bands.", "Ziegelfugen und Flechten sind Texturstriche von 0,1 mm; Bänder gefüllte Streifen von 0,16 mm."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.25 mm, details 0.13 mm, texture 0.1 mm.", "Kontur 0,25 mm, Details 0,13 mm, Textur 0,1 mm."),
        "overlays": t("Bundles, straps, battens and pallet parts are their own cells.", "Bündel, Bänder, Latten und Palettenteile sind eigene Zellen."),
    },
    "200": {
        "silhouette": t("The pallet as upper plate on nine feet, two tiers of bundles (1.9 x 0.8 mm each) with the batten layer between them; straps as lines.", "Die Palette als obere Platte auf neun Füßen, zwei Lagen Bündel (je 1,9 x 0,8 mm) mit der Lattenlage dazwischen; Bänder als Linien."),
        "retained": t("Bundle blocks, batten ends (0.25 x 0.15 mm), fork openings between the feet, the outline of the outermost tile.", "Bündelblöcke, Lattenenden (0,25 x 0,15 mm), Gabelöffnungen zwischen den Füßen, die Kontur des äußersten Ziegels."),
        "merged": t("Deck and cross boards are one plate, bottom boards and blocks nine feet; the segmental tail is square (20 mm rise below the 30 mm feature).", "Deck- und Querbretter sind eine Platte, Bodenbretter und Klötze neun Füße; der Segmentschnitt ist gerade (20 mm Stich unter der Einzelheit von 30 mm)."),
        "omitted": t("Tile joints, lichen strokes, deck gaps.", "Ziegelfugen, Flechtenstriche, Deckfugen."),
        "thinElements": t("Straps are 0.1 mm detail lines.", "Bänder sind Detaillinien von 0,1 mm."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.18 mm, details 0.1 mm.", "Kontur 0,18 mm, Details 0,1 mm."),
        "overlays": t("Bundles, battens and pallet parts are their own cells.", "Bündel, Latten und Palettenteile sind eigene Zellen."),
    },
    "500": {
        "silhouette": t("The loaded pallet as two tiers of bundle blocks on the pallet slab (1.6 x 0.97 mm in the side elevation, 2.4 x 0.97 mm in the front view).", "Die beladene Palette als zwei Lagen Bündelblöcke auf der Palettenplatte (1,6 x 0,97 mm in der Seitenansicht, 2,4 x 0,97 mm in der Vorderansicht)."),
        "retained": t("Height, length and width of the lot, the two tiers and the pallet slab.", "Höhe, Länge und Breite der Partie, die zwei Lagen und die Palettenplatte."),
        "merged": t("The batten layer (30 mm) belongs to the upper tier, the pallet is one slab, its fork openings (78 mm) are closed.", "Die Lattenlage (30 mm) gehört zur oberen Lage, die Palette ist eine Platte, ihre Gabelöffnungen (78 mm) sind geschlossen."),
        "omitted": t("Straps, joints, battens as separate cells, and the fifteen patch tiles: stored they lie inside their packs, laid as a patch of 1.1 x 2.2 mm on paper their 14 mm courses overlap below the printable feature; the 3D model keeps them.", "Bänder, Fugen, Latten als eigene Zellen und die fünfzehn Musterziegel: gelagert liegen sie in ihren Paketen, verlegt als Fläche von 1,1 x 2,2 mm auf Papier überdecken sich ihre Reihen von 14 mm unter der druckbaren Einzelheit; das 3D-Modell behält sie."),
        "thinElements": t("None.", "Keine."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": t("Bundles and pallet are their own cells.", "Bündel und Palette sind eigene Zellen."),
    },
}

VOL_DECISIONS = {
    "50": [
        t("Body: all 168 tiles as true solids: the face outline with its segmental tail (12 chords, rise 20 mm) extruded through 14 mm, the nib 22 x 20 x 12 mm at the head on the back; a row of 42 alternates its head ends and stands at a 26 mm pitch, so each nib bears on the weathered tail of its neighbour and leaves a 12 mm gap; runs of equal orientation are repeats. Two outer tiles have chipped tail corners. The fifteen patch tiles are the same solids on their own bones.",
          "Körper: alle 168 Ziegel als wahre Körper: der Umriss mit seinem Segmentschnitt (12 Sehnen, 20 mm Stich) 14 mm extrudiert, die Nase 22 x 20 x 12 mm am Kopf auf der Rückseite; eine Reihe aus 42 wechselt ihre Kopfenden und steht im Raster von 26 mm, so dass jede Nase auf dem bewitterten Fuß ihres Nachbarn aufliegt und einen Spalt von 12 mm lässt; Folgen gleicher Richtung sind Wiederholungen. Zwei äußere Ziegel haben abgeplatzte Fußecken. Die fünfzehn Musterziegel sind dieselben Körper auf eigenen Knochen."),
        t("Pallet: the EUR pallet part by part (three bottom boards, nine blocks, three cross boards, five deck boards). Battens: six 30 x 50 x 1200 mm between the tiers. Straps: two PET loops per bundle, each one closed ring of 16 x 0.9 mm band around the tiles (a plan profile with the bundle as its hole).",
          "Palette: die EUR-Palette Teil für Teil (drei Bodenbretter, neun Klötze, drei Querbretter, fünf Deckbretter). Latten: sechs 30 x 50 x 1200 mm zwischen den Lagen. Bänder: zwei PET-Schlaufen je Bündel, jede ein geschlossener Ring aus Band 16 x 0,9 mm um die Ziegel (ein Grundrissprofil mit dem Bündel als Loch)."),
        t("Thermal: every tile is split at the head lap (225 mm from the head) into its clean lapped head with the nib and its weathered exposed tail; the split faces are section faces. The 12 mm gaps are open to the air, so both faces of every tile count.",
          "Thermisch: jeder Ziegel ist an der Höhenüberdeckung (225 mm vom Kopf) in seinen sauberen überdeckten Kopf mit Nase und seinen bewitterten sichtbaren Fuß geteilt; die Teilungsflächen sind Schnittflächen. Die Spalte von 12 mm sind zur Luft offen, daher zählen beide Flächen jedes Ziegels."),
        t("Structure: deck and cross boards as members, the nine blocks as joints down to the ground, one member block per bundle, the battens and patch tiles as members; contacts change per pose (pallet feet, fork tines, rafter pins).",
          "Tragwerk: Deck- und Querbretter als Glieder, die neun Klötze als Gelenke bis zum Boden, ein Gliedblock je Bündel, Latten und Musterziegel als Glieder; die Kontakte wechseln je Pose (Palettenfüße, Gabelzinken, Sparrenauflager)."),
    ],
    "100": [
        t("Body: each bundle of 14 tiles is one envelope block (typified row, `envelope`), because the 12 mm gaps and 14 mm tiles are below the 15 mm feature; nibs are left out; the patch tiles keep their own solids at 15 mm thickness with a six-chord tail; runners of the pallet are one extrude of bottom board and blocks.",
          "Körper: jedes Bündel aus 14 Ziegeln ist ein Hüllblock (typisierte Reihe, `envelope`), weil die Spalte von 12 mm und die Ziegel von 14 mm unter der Einzelheit von 15 mm liegen; die Nasen entfallen; die Musterziegel behalten eigene Körper mit 15 mm Dicke und einem Fuß aus sechs Sehnen; die Kufen der Palette sind je eine Extrusion aus Bodenbrett und Klötzen."),
        t("Thermal: the bundle envelopes carry the stack regions as porous envelopes (lapped middle, weathered ends); the patch tiles are split into head and tail as at 1:50. Structure: one deck member, three cross boards, three runner joints, the bundle blocks, battens and patch tiles.",
          "Thermisch: die Bündelhüllen tragen die Stapelregionen als poröse Hüllen (überdeckte Mitte, bewitterte Enden); die Musterziegel sind wie bei 1:50 in Kopf und Fuß geteilt. Tragwerk: ein Deckglied, drei Querbretter, drei Kufengelenke, die Bündelblöcke, Latten und Musterziegel."),
    ],
    "200": [
        t("Body: the pallet as upper plate on nine feet (100 mm openings), bundle envelopes, battens at their true 30 mm, no straps; the patch tiles are 30 mm thick plates with a square tail (generalised for the 30 mm feature).",
          "Körper: die Palette als obere Platte auf neun Füßen (Öffnungen 100 mm), Bündelhüllen, Latten mit ihren wahren 30 mm, keine Bänder; die Musterziegel sind 30 mm dicke Platten mit geradem Fuß (verallgemeinert für die Einzelheit von 30 mm)."),
        t("Thermal: each bundle envelope is two porous halves (weathered and lapped), the straps are slender rings of the thermal list only (16 mm bands, 0.08 mm on paper); structure as at 1:100 with the plate and feet of this scale.", "Thermisch: jede Bündelhülle besteht aus zwei porösen Hälften (bewittert und überdeckt), die Bänder sind schlanke Ringe nur der thermischen Liste (Bänder von 16 mm, 0,08 mm auf Papier); Tragwerk wie bei 1:100 mit Platte und Füßen dieses Maßstabs."),
    ],
    "500": [
        t("Body: the pallet as one slab, the bundles as envelope blocks, the battens as slender bars of their true section, no straps; the patch tiles are 75 mm plates (the feature of this scale) and stay inside their bundle envelopes when stored.",
          "Körper: die Palette als eine Platte, die Bündel als Hüllblöcke, die Latten als schlanke Stäbe ihres wahren Querschnitts, keine Bänder; die Musterziegel sind Platten von 75 mm (die Einzelheit dieses Maßstabs) und liegen gelagert in ihren Bündelhüllen."),
        t("Thermal: the straps remain as slender rings of the thermal list only. Structure: a deck member on three legs of the pallet, the bundle blocks, battens and patch tiles.",
          "Thermisch: die Bänder bleiben als schlanke Ringe nur in der thermischen Liste. Tragwerk: ein Deckglied auf drei Beinen der Palette, die Bündelblöcke, Latten und Musterziegel."),
    ],
}


TILE_FACE = abs(area(face("50")))
TILE_VOL = TILE_FACE * T + NIB_L * NIB_W * NIB_H
M_TILE = TILE_VOL * RHO_CLAY
PALLET_VOL = sum((p[2][1] - p[2][0]) * (p[3][1] - p[3][0]) * (p[4][1] - p[4][0]) for p in PALLET_50)
M_PALLET = PALLET_VOL * RHO_PALLET
M_BATTEN = BATTEN_W * BATTEN_H * BATTEN_LEN * RHO_BATTEN
BAND_LEN = 2 * (L + 2 * BAND_T) + 2 * ((N_BUNDLE - 1) * P + T + 2 * BAND_T)
M_BAND = BAND_LEN * BAND_W * BAND_T * RHO_PET


def bundle_mass(tier, row, b):
    """⚖️ Tiles left in a bundle plus its two strap loops."""
    return len(bundle_tiles(tier, row, b)) * M_TILE + 2 * M_BAND


M_TOTAL = M_PALLET + sum(bundle_mass(*k) for k in BUNDLES) + 6 * M_BATTEN + len(PATCH_TILES) * M_TILE


def rot_w(a):
    """🔄 Rotation about w by a degrees (u towards v)."""
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def rot_u(a):
    """🔄 Rotation about u by a degrees in the product's sense (positive turns v to -w)."""
    c, s = math.cos(math.radians(-a)), math.sin(math.radians(-a))
    return [[1, 0, 0], [0, c, -s], [0, s, c]]


def mul(a, b):
    """✖️ 3 x 3 matrix product."""
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def apply(m, v):
    """➡️ Matrix times vector."""
    return [sum(m[i][k] * v[k] for k in range(3)) for i in range(3)]


def roof(s, n):
    """🏠 World (x, y) of a point given along the batten plane (s downslope from the top batten edge) and normal to it (n outward)."""
    a = math.radians(ALPHA)
    return (ROOF_X0 + s * math.cos(a) + n * math.sin(a), ROOF_Y0 - s * math.sin(a) + n * math.cos(a))


S_BAT = {f"batten-{i}": i * GAUGE for i in range(5)}
S_FILLET = S_BAT["batten-4"] + FILLET_RISE / math.tan(math.radians(DELTA))


def batten_rest(bat):
    """📍 World rest origin of a batten: the upper edge of its leading face, lying between the tiers."""
    row, pos = BATTEN_ROLE[bat]
    return (ROW_X0[row] + BATTEN_AT[pos] - BATTEN_W / 2, TIER_BASE[2], 0.0)


def batten_roof(bat):
    """🏠 World origin and angle of a batten on the roof (the eaves batten stands on edge as tilting fillet)."""
    if bat == "eaves-batten":
        x, y = roof(S_FILLET, -BATTEN_H)
        return (x, y, 0.0), 90 - ALPHA
    x, y = roof(S_BAT[bat], 0.0)
    return (x, y, 0.0), -ALPHA


def tile_rest(p):
    """📍 World rest origin of a patch tile (head, lower edge, back face for A; head, upper edge, back face for B, turned 180 degrees)."""
    x0 = ROW_X0[p["row"]]
    base = TIER_BASE[p["tier"]]
    z = z_of(p["k"])
    return (x0 + L, base + W, z) if p["mirror"] else (x0, base, z)


def tile_roof(p):
    """🏠 World origin of a patch tile laid in its course: its nib hooked 1.2 mm behind the batten, its back on the batten edge and the tile below."""
    d = math.radians(DELTA)
    s_e = S_BAT[f"batten-{p['course']}"]
    mu = 0.0012 / math.cos(d)
    qs, qn = s_e - mu * math.cos(d), -mu * math.sin(d)
    os_, on = qs - NIB_L * math.cos(d), qn - NIB_L * math.sin(d)
    x, y = roof(os_, on)
    return (x, y, z_of(p["k"]))


def carrier_travel(rest, target, parent_rest, parent_target, parent_angle):
    """🛤️ The (dx, dy) of a prismatic carrier in its parent frame that takes its rest origin to the target origin."""
    rel = [rest[i] - parent_rest[i] for i in range(3)]
    m = rot_w(-parent_angle)
    local = apply(m, [target[i] - parent_target[i] for i in range(3)])
    return local[0] - rel[0], local[1] - rel[1]


def reinstall_bones():
    """🧩 The offsets that take the battens to the rafters and the fifteen tiles into their courses."""
    out = {}
    for bat in BATTENS:
        o, ang = batten_roof(bat)
        dx, dy = o[0] - batten_rest(bat)[0], o[1] - batten_rest(bat)[1]
        out[f"{bat}-carrier"] = {"dx": r(dx), "dy": r(dy)}
        out[bat] = {"angleDeg": round(ang, 6)}
    for p in PATCH_TILES:
        bat = f"batten-{p['course']}"
        bo, bang = batten_roof(bat)
        dx, dy = carrier_travel(tile_rest(p), tile_roof(p), batten_rest(bat), bo, bang)
        out[f"{p['id']}-carrier"] = {"dx": r(dx), "dy": r(dy)}
        out[p["id"]] = {"angleDeg": round(DELTA - 180 if p["mirror"] else DELTA, 6), "twistDeg": -90}
    return out


REINSTALL = reinstall_bones()


def prismatic(dx, dy):
    """🛤️ A prismatic joint along the travel (dx, dy)."""
    return {"type": "prismatic", "axisDeg": math.degrees(math.atan2(dy, dx)), "minM": 0, "maxM": round(math.hypot(dx, dy) + 0.001, 6)}


def section(a, i, e, material, en, de):
    """📏 A member section."""
    return {"areaM2": r(a), "secondMomentM4": float(f"{i:.6e}"), "extremeFibreM": r(e), "material": material, "title": t(en, de)}


def rig():
    """🦴 Yard datum, the pallet on its lifting joint, twelve bundles on the pallet, six battens on carriers that take them to the rafters, fifteen patch tiles on carriers under their battens, and the carrier of the maintenance load."""
    deck_a = sum(x1 - x0 for x0, x1 in DECK_X) * 0.022
    bones = [
        {"id": "yard", "parent": None, "title": t("Yard datum (ground of the component exchange)", "Hofbezug (Boden der Bauteilbörse)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "pallet", "parent": "yard", "title": t("EUR pallet (lifted by the fork tines)", "EUR-Palette (von den Gabelzinken gehoben)"), "x": 0, "y": DECK, "angleDeg": 0, "lengthM": 0.8, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 1.6},
         "mass": {"kg": r(M_PALLET), "comX": 0.4, "comY": r(-0.07), "distribution": "uniform", "source": "authored-estimate", "reference": "en-13698-1"},
         "section": section(deck_a * 1.5, 1.2 * 0.022 ** 3 / 12 * 5, 0.011, "pallet-timber", "Deck boards 22 mm across the cross boards", "Deckbretter 22 mm über den Querbrettern")},
    ]
    for tier, row, b in BUNDLES:
        o = bundle_origin(tier, row, b)
        n = len(bundle_tiles(tier, row, b))
        lo, hi = bundle_span(b)
        bones.append({"id": bundle_id(tier, row, b), "parent": "pallet", "title": t(f"Bundle {b + 1} of row {row + 1}, tier {tier} ({n} tiles on edge, strapped)", f"Bündel {b + 1} der Reihe {row + 1}, Lage {tier} ({n} Ziegel hochkant, umreift)"), "x": r(o[0]), "y": r(o[1] - DECK), "z": r(o[2]), "angleDeg": 0, "lengthM": L, "joint": {"type": "fixed"},
                      "mass": {"kg": r(bundle_mass(tier, row, b)), "comX": L / 2, "comY": W / 2, "distribution": "uniform", "source": "authored-estimate", "reference": "en-iso-10456"},
                      "section": section(W * (hi - lo), (hi - lo) * W ** 3 / 12, W / 2, "clay-weathered", f"Bundle {int(round((hi - lo) * 1000))} x 155 mm (tiles on edge)", f"Bündel {int(round((hi - lo) * 1000))} x 155 mm (Ziegel hochkant)")})
    for bat in BATTENS:
        rest = batten_rest(bat)
        dx, dy = REINSTALL[f"{bat}-carrier"]["dx"], REINSTALL[f"{bat}-carrier"]["dy"]
        name = "eaves batten (tilting fillet on edge)" if bat == "eaves-batten" else f"batten {int(bat[-1]) + 1}"
        name_de = "Traufkantholz (Latte hochkant als Aufschiebling)" if bat == "eaves-batten" else f"Latte {int(bat[-1]) + 1}"
        bones.append({"id": f"{bat}-carrier", "parent": "pallet", "title": t(f"Carrier of {name} to the rafters", f"Träger der {name_de} zu den Sparren"), "x": r(rest[0]), "y": r(rest[1] - DECK), "z": 0, "angleDeg": 0, "lengthM": 0, "joint": prismatic(dx, dy)})
        bones.append({"id": bat, "parent": f"{bat}-carrier", "title": t(f"Roof {name}, 30 x 50 mm", f"Dach{name_de.lower() if bat != 'eaves-batten' else name_de}, 30 x 50 mm"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": BATTEN_W, "joint": {"type": "revolute", "minDeg": -90, "maxDeg": 90},
                      "mass": {"kg": r(M_BATTEN), "comX": BATTEN_W / 2, "comY": -BATTEN_H / 2, "distribution": "point", "source": "literature", "reference": "en-338"},
                      "section": section(BATTEN_W * BATTEN_H, BATTEN_W * BATTEN_H ** 3 / 12, BATTEN_H / 2, "batten-timber", "Batten 30 x 50 mm, S10 (C24), weak axis", "Latte 30 x 50 mm, S10 (C24), schwache Achse")})
    for p in PATCH_TILES:
        bat = f"batten-{p['course']}"
        rest = tile_rest(p)
        brest = batten_rest(bat)
        dx, dy = REINSTALL[f"{p['id']}-carrier"]["dx"], REINSTALL[f"{p['id']}-carrier"]["dy"]
        bones.append({"id": f"{p['id']}-carrier", "parent": bat, "title": t(f"Carrier of patch tile {p['column'] + 1} of course {p['course'] + 1}", f"Träger des Musterziegels {p['column'] + 1} der Reihe {p['course'] + 1}"), "x": r(rest[0] - brest[0]), "y": r(rest[1] - brest[1]), "z": r(rest[2]), "angleDeg": 0, "lengthM": 0, "joint": prismatic(dx, dy)})
        joint = {"type": "revolute", "minDeg": -180, "maxDeg": 0, "twistMinDeg": -90, "twistMaxDeg": 0} if p["mirror"] else {"type": "revolute", "minDeg": -10, "maxDeg": 10, "twistMinDeg": -90, "twistMaxDeg": 0}
        bones.append({"id": p["id"], "parent": f"{p['id']}-carrier", "title": t(f"Patch tile {p['column'] + 1} of course {p['course'] + 1} (tile {p['k'] + 1} of its row in the stack)", f"Musterziegel {p['column'] + 1} der Reihe {p['course'] + 1} (Ziegel {p['k'] + 1} seiner Reihe im Stapel)"), "x": 0, "y": 0, "angleDeg": 180 if p["mirror"] else 0, "lengthM": L, "joint": joint,
                      "mass": {"kg": r(M_TILE), "comX": L / 2, "comY": W / 2, "distribution": "uniform", "source": "authored-estimate", "reference": "en-iso-10456"},
                      "section": section(W * T, W * T ** 3 / 12, T / 2, "clay-weathered", "Plain tile 155 x 14 mm", "Biberschwanz 155 x 14 mm")})
    bones.append({"id": "load-carrier", "parent": "batten-0", "title": t("Load carrier (massless, moves the maintenance load down the patch)", "Lastträger (masselos, bewegt die Wartungslast die Musterfläche hinab)"), "x": 0.05, "y": 0.05, "z": -0.02, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.8}})
    return {"kind": "mechanical", "root": "yard", "bones": bones}


FEET = [f"foot-{X_NAME[i]}-{SIDE[k]}" for i in range(3) for k in range(3)]
FORKS = [f"fork-{side}-{SIDE[k]}" for side in ("rear", "front") for k in range(3)]
PINS = [f"rafter-{bat}-{side}" for bat in BATTENS for side in ("right", "left")]


def anchors():
    """🪝 Pallet feet, fork tine contacts, rafter pins of the battens, snow points and the maintenance load point."""
    out = []
    for i, (x0, x1) in enumerate(X_ROWS):
        for k, (z0, z1) in enumerate(Z_ROWS):
            out.append({"id": f"foot-{X_NAME[i]}-{SIDE[k]}", "bone": "pallet", "x": r((x0 + x1) / 2), "y": r(-DECK), "z": r((z0 + z1) / 2), "kind": "contact", "title": t(f"Pallet foot {X_NAME[i]} {SIDE[k]} on the yard", f"Palettenfuß {['hinten', 'mitte', 'vorn'][i]} {['rechts', 'mitte', 'links'][k]} auf dem Hof"), "support": "contact", "normalDeg": 90, "footprintLengthM": r(x1 - x0), "footprintWidthM": r(z1 - z0), "frictionCoefficient": 0.5})
    for name, x in (("rear", 0.21375), ("front", 0.58625)):
        for k, (z0, z1) in enumerate(Z_ROWS):
            out.append({"id": f"fork-{name}-{SIDE[k]}", "bone": "pallet", "x": x, "y": r(0.1 - DECK), "z": r((z0 + z1) / 2), "kind": "contact", "title": t(f"Fork tine {name} under the {SIDE[k]} cross board", f"Gabelzinke {'hinten' if name == 'rear' else 'vorn'} unter dem Querbrett {['rechts', 'mitte', 'links'][k]}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.1, "footprintWidthM": 0.145, "frictionCoefficient": 0.3})
    for bat in BATTENS:
        for side, z in zip(("right", "left"), RAFTER_Z):
            at = {"x": 0.0, "y": -BATTEN_H / 2} if bat == "eaves-batten" else {"x": BATTEN_W / 2, "y": -BATTEN_H}
            out.append({"id": f"rafter-{bat}-{side}", "bone": bat, **at, "z": z, "kind": "support", "title": t(f"Rafter {side} under {bat.replace('-', ' ')}", f"Sparren {'rechts' if side == 'right' else 'links'} unter {bat.replace('eaves-batten', 'Traufkantholz').replace('batten-', 'Latte ')}"), "support": "pin", "footprintLengthM": 0.08, "footprintWidthM": BATTEN_W})
    for i in range(5):
        out.append({"id": f"snow-{i}", "bone": f"batten-{i}", "x": BATTEN_W / 2, "y": 0.04, "z": -0.025, "kind": "load", "title": t(f"Snow on the patch over batten {i + 1}", f"Schnee auf der Musterfläche über Latte {i + 1}")})
    out.append({"id": "load-point", "bone": "load-carrier", "x": 0, "y": 0, "kind": "load", "title": t("Maintenance load", "Wartungslast")})
    return out


def region_binding(sw, sl, pw, pl, pallet, battens, strap):
    """🌤️ The thermal binding of the seven regions."""
    return {"regions": {"stack-weathered": sw, "stack-lapped": sl, "patch-weathered": pw, "patch-lapped": pl, "pallet-timber": pallet, "battens": battens, "strapping": strap}, "emitters": {}}


def rb(exposed, sun, contact=None):
    """🌤️ One region binding."""
    out = {"exposedFraction": exposed, "sunlitFraction": sun}
    if contact:
        out["contact"] = {"fraction": contact[0], "tempC": contact[1]}
    return out


YARD = region_binding(rb(1, 0.25), rb(1, 0.25), rb(1, 0.2), rb(1, 0.2), rb(1, 0.2, (0.08, 18)), rb(1, 0.1), rb(1, 0.5))
LIFTED = region_binding(rb(1, 0.25), rb(1, 0.25), rb(1, 0.2), rb(1, 0.2), rb(1, 0.2), rb(1, 0.1), rb(1, 0.5))
ROOF = region_binding(rb(1, 0.25), rb(1, 0.25), rb(1, 0.55), rb(0.45, 0.1, (0.35, 18)), rb(1, 0.2, (0.08, 18)), rb(0.6, 0, (0.1, 18)), rb(1, 0.5))


def snow_loads():
    """❄️ Snow on the plan projection of the patch, lumped on its five battens."""
    width = 3 * W + 2 * 0.001 + 0.078
    fy = -SNOW_KNM2 * 1000 * GAUGE * math.cos(math.radians(ALPHA)) * width
    return [{"id": f"snow-{i}", "kind": "snow", "title": t(f"Snow {abs(fy):.0f} N over batten {i + 1} (0.34 kN/m² on plan)", f"Schnee {abs(fy):.0f} N über Latte {i + 1} (0,34 kN/m² im Grundriss)"), "anchor": f"snow-{i}", "fxN": 0, "fyN": round(fy, 2), "source": "literature", "reference": "en-1991-1-3"} for i in range(5)]


QLOAD = {"id": "maintenance-load", "kind": "live", "title": t("Maintenance load 1.0 kN (roof category H)", "Wartungslast 1,0 kN (Dachkategorie H)"), "anchor": "load-point", "fxN": 0, "fyN": -QK, "source": "literature", "reference": "en-1991-1-1"}

POSES = [
    {"id": "stored", "title": t("Stored on the pallet", "Auf der Palette gelagert"), "description": t("168 reclaimed plain tiles stand on their long edge in two tiers of two rows on a EUR pallet in the yard of the component exchange, three strapped bundles of 14 to a row, the upper tier on six roof battens; the pallet stands on its nine feet.", "168 gebrauchte Biberschwanzziegel stehen auf ihrer Langkante in zwei Lagen zu zwei Reihen auf einer EUR-Palette im Hof der Bauteilbörse, drei umreifte Bündel zu 14 je Reihe, die obere Lage auf sechs Dachlatten; die Palette steht auf ihren neun Füßen."), "bones": {}, "ground": "authored", "contacts": FEET, "thermal": YARD, "loads": []},
    {"id": "lifted", "title": t("Lifted on the fork tines", "Auf den Gabelzinken gehoben"), "description": t("A forklift has run its tines from the short side under the three cross boards and lifts the loaded pallet 300 mm off the yard; the six tine contacts carry the whole lot.", "Ein Gabelstapler hat seine Zinken von der Schmalseite unter die drei Querbretter gefahren und hebt die beladene Palette 300 mm vom Hof; die sechs Zinkenkontakte tragen die ganze Partie."), "bones": {"pallet": {"dy": 0.30}}, "ground": "authored", "contacts": FORKS, "thermal": LIFTED, "loads": []},
    {"id": "reinstalled", "title": t("Reinstalled as a double-lap patch", "Als Doppeldeckung wieder eingedeckt"), "description": t("Fifteen tiles are hung again as a double-lap patch at 45 degrees: five courses of three in half bond on the five battens of 150 mm gauge, the eaves batten on edge as tilting fillet, the battens pinned to two rafters 0.80 m apart; snow of 0.34 kN/m² lies on the patch; the rest of the lot stays on the pallet.", "Fünfzehn Ziegel sind wieder als Doppeldeckung unter 45 Grad gehängt: fünf Reihen zu drei im halben Verband auf den fünf Latten mit 150 mm Lattweite, das Traufkantholz hochkant als Aufschiebling, die Latten auf zwei Sparren mit 0,80 m Abstand; Schnee von 0,34 kN/m² liegt auf der Fläche; der Rest der Partie bleibt auf der Palette."), "bones": REINSTALL, "ground": "authored", "contacts": FEET + PINS, "thermal": ROOF, "loads": snow_loads()},
    {"id": "reinstalled-maintenance", "title": t("Reinstalled, maintenance load", "Wieder eingedeckt, Wartungslast"), "description": t("The reinstalled patch under the 1.0 kN maintenance load of roof category H on its middle course.", "Die wieder eingedeckte Fläche unter der Wartungslast von 1,0 kN der Dachkategorie H auf ihrer mittleren Reihe."), "bones": {**REINSTALL, "load-carrier": {"dx": 0.35}}, "ground": "authored", "contacts": FEET + PINS, "thermal": ROOF, "loads": [QLOAD]},
]

CLIPS = [
    {"id": "lift", "title": t("Lift on the fork tines", "Anheben auf den Gabelzinken"), "description": t("The tines take the load under the cross boards and lift the pallet with its tiles 300 mm off the yard.", "Die Zinken übernehmen die Last unter den Querbrettern und heben die Palette mit ihren Ziegeln 300 mm vom Hof."),
     "durationS": 4, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "step"}, {"t": 0.5, "pose": "lifted", "ease": "smooth", "bones": {"pallet": {"dy": 0}}}, {"t": 4, "pose": "lifted", "ease": "smooth"}],
     "contacts": [*[{"anchor": a, "fromS": 0, "toS": 0.5} for a in FEET], *[{"anchor": a, "fromS": 0.5, "toS": 4} for a in FORKS]],
     "phases": [{"id": "engage", "fromS": 0, "toS": 0.5, "title": t("Tines take the load", "Zinken übernehmen die Last")}, {"id": "raise", "fromS": 0.5, "toS": 4, "title": t("Raise", "Heben")}],
     "events": [{"t": 0.5, "kind": "contact-off", "title": t("Pallet leaves the yard", "Palette verlässt den Hof")}, {"t": 0.5, "kind": "contact-on", "title": t("Tines carry the pallet", "Zinken tragen die Palette")}], "tracks": []},
    {"id": "maintenance-walk", "title": t("Maintenance load down the patch", "Wartungslast die Fläche hinab"), "description": t("A 1.0 kN maintenance load moves from the top course to the eaves of the reinstalled patch.", "Eine Wartungslast von 1,0 kN wandert von der obersten Reihe zur Traufe der wieder eingedeckten Fläche."),
     "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled-maintenance", "ease": "linear", "bones": {"load-carrier": {"dx": 0.05}}}, {"t": 6, "pose": "reinstalled-maintenance", "ease": "linear", "bones": {"load-carrier": {"dx": 0.75}}}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 6} for a in FEET + PINS],
     "phases": [{"id": "walk", "fromS": 0, "toS": 6, "title": t("Walk down", "Hinabgehen")}],
     "events": [{"t": 0, "kind": "load-on", "title": t("Load on the top course", "Last auf der obersten Reihe")}, {"t": 6, "kind": "load-off", "title": t("Load at the eaves", "Last an der Traufe")}], "tracks": []},
]

REFERENCES = [
    {"id": "en-1304", "citation": "EN 1304:2013 Clay roofing tiles and fittings, product definitions and specifications: plain tiles (Biberschwanz) with nibs, flexural strength and impermeability classes; the 155 x 375 mm format with segmental cut is the classic German Biberschwanz format."},
    {"id": "zvdh-fachregel", "citation": "ZVDH Fachregel für Dachdeckungen mit Dachziegeln und Dachsteinen (Deutsches Dachdeckerhandwerk, Regelwerk): plain tile double lap (Doppeldeckung), batten gauge = (tile length − head lap) / 2, head lap by roof pitch, half bond of the courses, raised eaves batten; and ZVDH Hinweise Holz und Holzwerkstoffe: batten 30/50 for rafter spacing up to 0.80 m."},
    {"id": "din-1055-1", "citation": "DIN 1055-1:2002-06 Einwirkungen auf Tragwerke, Teil 1: Wichten und Flächenlasten von Baustoffen: plain tiles 155 x 375 mm, double lap (Doppeldach), 0.75 kN/m² including battens."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1, Part 1-1, Table 6.10 with DIN EN 1991-1-1/NA: roofs of category H (not accessible except for maintenance), concentrated load Qk 1.0 kN."},
    {"id": "en-1991-1-3", "citation": "EN 1991-1-3:2003 Eurocode 1, Part 1-3, snow loads, Table 5.2: shape coefficient mu1 = 0.8 (60 − alpha) / 30 for 30 to 60 degrees (0.4 at 45 degrees); DIN EN 1991-1-3/NA snow zone 2 at low altitude sk 0.85 kN/m²."},
    {"id": "en-13698-1", "citation": "EN 13698-1:2003 Pallet production specification, part 1: flat wooden pallets 800 x 1200 mm (EPAL/EUR pallet): five deck boards 22 mm (145, 100, 145, 100, 145 mm), three cross boards 145 x 22 mm, nine blocks 78 mm, three bottom boards 22 mm; height 144 mm, fork openings 227.5 mm."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes: C24 E0,mean 11.0 GPa, fm,k 24 MPa, mean density 420 kg/m³ (450 at outdoor moisture); DIN 4074-1:2012 visual grade S10 corresponds to C24, the grade of roof battens."},
    {"id": "en-13394", "citation": "EN 13394:2001 Packaging, non-metallic strapping (PET strapping 16 x 0.9 mm, tensile strength of the order of 400 MPa, density 1380 kg/m³)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties, Table 3: clay roof tiles 2000 kg/m³, lambda 1.0 W/mK, cp 800 J/kgK; softwood 450 to 500 kg/m³, 0.13 W/mK, 1600 J/kgK."},
    {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); a 14 mm tile is thinner than half of it and stores heat through its whole thickness."},
    {"id": "emissivity-literature", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 4, radiation properties of building materials: red clay tile emissivity about 0.9 and solar absorptance about 0.65 (new); weathered tile with lichen and soot is darker (0.75 to 0.8, rounded authored estimate guided by this chapter); weathered softwood 0.9 and 0.6 to 0.7."},
    {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, general principles of presentation, part 50: basic conventions for areas on sections and cuts."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (scales, line groups)."},
]

MATERIALS = [
    {"id": "clay-weathered", "title": t("Old clay plain tile, weathered exposed tail with lichen", "Alter Biberschwanz aus Ton, bewitterter sichtbarer Fuß mit Flechten"), "emissivity": 0.93, "solarAbsorptivity": 0.78, "densityKgM3": RHO_CLAY, "conductivityWmK": 1.0, "specificHeatJKgK": 800, "elasticModulusPa": 20.0e9, "strengthPa": 10.0e6, "source": "authored-estimate", "reference": "emissivity-literature"},
    {"id": "clay-lapped", "title": t("Old clay plain tile, lapped head (clean red clay)", "Alter Biberschwanz aus Ton, überdeckter Kopf (sauberer roter Ton)"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO_CLAY, "conductivityWmK": 1.0, "specificHeatJKgK": 800, "elasticModulusPa": 20.0e9, "strengthPa": 10.0e6, "source": "literature", "reference": "en-iso-10456"},
    {"id": "pallet-timber", "title": t("Pallet softwood, weathered grey", "Palettennadelholz, grau bewittert"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO_PALLET, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 8.0e9, "strengthPa": 16.0e6, "source": "authored-estimate", "reference": "en-338"},
    {"id": "batten-timber", "title": t("Roof batten S10 (C24), sawn spruce", "Dachlatte S10 (C24), sägeraue Fichte"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": RHO_BATTEN, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11.0e9, "strengthPa": 24.0e6, "source": "literature", "reference": "en-338"},
    {"id": "pet-strap", "title": t("PET strapping band, green", "PET-Umreifungsband, grün"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": RHO_PET, "conductivityWmK": 0.24, "specificHeatJKgK": 1300, "elasticModulusPa": 10.0e9, "strengthPa": 400.0e6, "source": "authored-estimate", "reference": "en-13394"},
]


def rect_area(a, b):
    """▭ Area of an axis rectangle overlap of two (lo, hi) ranges pairs."""
    return max(0.0, min(a[0][1], b[0][1]) - max(a[0][0], b[0][0])) * max(0.0, min(a[1][1], b[1][1]) - max(a[1][0], b[1][0]))


def boxes_area(parts):
    """📐 Outer area of touching axis-aligned boxes of one region: every face minus the patches where another box lies against it."""
    total = 0.0
    for i, p in enumerate(parts):
        for ax in range(3):
            o = [a for a in range(3) if a != ax]
            for side in (0, 1):
                f = (p[o[0]], p[o[1]])
                a = (f[0][1] - f[0][0]) * (f[1][1] - f[1][0])
                for j, q in enumerate(parts):
                    if i != j and abs(q[ax][1 - side] - p[ax][side]) < 1e-9:
                        a -= rect_area(f, (q[o[0]], q[o[1]]))
                total += a
    return total


def tile_areas():
    """📐 Hand areas of one tile: lapped head with its nib and weathered tail, the split face left out (section face)."""
    head = face("50", "head")
    tail = face("50", "tail")
    a_head = 2 * abs(area(head)) - NIB_L * NIB_W + (perimeter(head) - W) * T + NIB_L * NIB_W + 2 * NIB_L * NIB_H + 2 * NIB_W * NIB_H
    a_tail = 2 * abs(area(tail)) + (perimeter(tail) - W) * T
    return a_head, a_tail


A_HEAD, A_TAIL = tile_areas()
N_STACK = sum(len(bundle_tiles(*k)) for k in BUNDLES)
PALLET_BOXES = [(p[2], p[3], p[4]) for p in PALLET_50]
STRAP_INNER = (L, (N_BUNDLE - 1) * P + T)
STRAP_AREA = 2 * (2 * (STRAP_INNER[0] + STRAP_INNER[1]) * BAND_W + 2 * (STRAP_INNER[0] + STRAP_INNER[1] + 4 * BAND_T) * BAND_W + 2 * ((STRAP_INNER[0] + 2 * BAND_T) * (STRAP_INNER[1] + 2 * BAND_T) - STRAP_INNER[0] * STRAP_INNER[1]))


def areas():
    """📐 Hand areas of the seven regions (m²)."""
    return {
        "stack-weathered": N_STACK * A_TAIL,
        "stack-lapped": N_STACK * A_HEAD,
        "patch-weathered": len(PATCH_TILES) * A_TAIL,
        "patch-lapped": len(PATCH_TILES) * A_HEAD,
        "pallet-timber": boxes_area(PALLET_BOXES),
        "battens": 6 * (2 * BATTEN_W * BATTEN_H + 2 * BATTEN_W * BATTEN_LEN + 2 * BATTEN_H * BATTEN_LEN),
        "strapping": len(BUNDLES) * STRAP_AREA,
    }


AREAS = areas()


def regions():
    """🌡️ The seven passive thermal regions."""
    tile_c = round(T * RHO_CLAY * 800)
    pallet_c = round(0.011 * RHO_PALLET * 1600)
    def reg(id_, en, de, material, length, geometry, sky, cap, ref):
        return {"id": id_, "title": t(en, de), "kind": "surface", "model": "passive", "material": material, "areaM2": round(AREAS[id_], 4), "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": ref}
    return [
        reg("stack-weathered", "Weathered exposed tails of the stacked tiles (lichen)", "Bewitterte sichtbare Füße der gestapelten Ziegel (Flechten)", "clay-weathered", W, "vertical-plate", 0.2, tile_c, "en-iso-13786"),
        reg("stack-lapped", "Lapped heads and nibs of the stacked tiles", "Überdeckte Köpfe und Nasen der gestapelten Ziegel", "clay-lapped", W, "vertical-plate", 0.2, tile_c, "en-iso-13786"),
        reg("patch-weathered", "Weathered exposed tails of the fifteen patch tiles", "Bewitterte sichtbare Füße der fünfzehn Musterziegel", "clay-weathered", GAUGE, "vertical-plate", 0.3, tile_c, "en-iso-13786"),
        reg("patch-lapped", "Lapped heads and nibs of the fifteen patch tiles", "Überdeckte Köpfe und Nasen der fünfzehn Musterziegel", "clay-lapped", SPLIT, "vertical-plate", 0.2, tile_c, "en-iso-13786"),
        reg("pallet-timber", "EUR pallet timber", "Holz der EUR-Palette", "pallet-timber", 0.145, "horizontal-plate-up", 0.3, pallet_c, "en-iso-10456"),
        reg("battens", "Roof battens", "Dachlatten", "batten-timber", BATTEN_W, "horizontal-plate-up", 0.2, round(0.015 * RHO_BATTEN * 1600), "en-iso-10456"),
        reg("strapping", "PET strapping bands", "PET-Umreifungsbänder", "pet-strap", BAND_W, "vertical-plate", 0.4, round(BAND_T / 2 * RHO_PET * 1300), "en-iso-10456"),
    ]


def document():
    """📄 The whole asset document."""
    asset = {
        "id": SLUG,
        "category": "components",
        "title": t("Reclaimed Roof Tile Pallet", "Palette gebrauchter Dachziegel"),
        "summary": t(
            f"168 reclaimed clay plain tiles (Biberschwanz 155 x 375 mm with segmental cut, nib and a lichen-darkened exposed tail) standing on their long edge in strapped bundles of 14, two tiers of two rows on a EUR pallet with roof battens between the tiers; lifted on fork tines, and fifteen of them hung again as a double-lap patch of five courses on 150 mm gauge battens under snow and a moving maintenance load.",
            "168 gebrauchte Biberschwanzziegel aus Ton (155 x 375 mm mit Segmentschnitt, Nase und von Flechten gedunkeltem sichtbarem Fuß), hochkant auf der Langkante in umreiften Bündeln zu 14, zwei Lagen zu zwei Reihen auf einer EUR-Palette mit Dachlatten zwischen den Lagen; auf Gabelzinken gehoben, und fünfzehn davon wieder als Doppeldeckung aus fünf Reihen auf Latten mit 150 mm Lattweite unter Schnee und einer wandernden Wartungslast gehängt."),
        "tags": ["component", "reclaimed", "clay", "ceramic", "roof-tile", "plain-tile", "biberschwanz", "roofing", "pallet", "stacked-goods"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t(
            f"Tile 155 x 375 x 14 mm, face {TILE_FACE * 1e4:.1f} cm² after the segmental cut, {M_TILE:.3f} kg at 2000 kg/m³; EUR pallet {M_PALLET:.1f} kg; six battens {M_BATTEN:.2f} kg each; total {M_TOTAL:.1f} kg. Patch: gauge 150 mm, head lap 75 mm, pitch 45 degrees, rafters 0.80 m apart. Packing, traces and the selection of the patch tiles are authored estimates.",
            de_num(f"Ziegel 155 x 375 x 14 mm, Fläche {TILE_FACE * 1e4:.1f} cm² nach dem Segmentschnitt, {M_TILE:.3f} kg bei 2000 kg/m³; EUR-Palette {M_PALLET:.1f} kg; sechs Latten je {M_BATTEN:.2f} kg; gesamt {M_TOTAL:.1f} kg") + ". Musterfläche: Lattweite 150 mm, Höhenüberdeckung 75 mm, Dachneigung 45 Grad, Sparrenabstand 0,80 m. Packung, Spuren und die Auswahl der Musterziegel sind Schätzungen.")},
        "dimensions": {"heightM": r(TOP), "widthM": 0.8, "depthM": 1.2, "massKg": round(M_TOTAL, 3)},
        "materials": MATERIALS,
        "rig": rig(),
        "anchors": anchors(),
        "representations": {s: representation(s) for s in SCALES},
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": {"environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.6, "windMS": 2, "windDirectionDeg": 0, "solarWM2": 600, "diffuseFraction": 0.25, "pressurePa": 101325}, "operating": [], "regions": regions(), "emitters": [], "passengers": [], "assumptions": [
            t("A 14 mm tile (lambda 1.0 W/mK, 2000 kg/m³, 800 J/kgK) is much thinner than its 24 h penetration depth (about 130 mm), so it stores heat through its whole thickness: about 22 kJ/m²K per face area (EN ISO 13786).", "Ein Ziegel von 14 mm (Lambda 1,0 W/mK, 2000 kg/m³, 800 J/kgK) ist viel dünner als seine 24-h-Eindringtiefe (etwa 130 mm) und speichert Wärme über die ganze Dicke: etwa 22 kJ/m²K je Fläche (EN ISO 13786)."),
            t("The exposed tail of an old tile (the last 150 mm, one gauge) is darkened by lichen, moss and soot and absorbs more sun (0.78) than its lapped head, which stayed clean red clay under the course above (0.65); emissivities 0.93 and 0.90. Authored estimates guided by ASHRAE.", "Der sichtbare Fuß eines alten Ziegels (die letzten 150 mm, eine Lattweite) ist durch Flechten, Moos und Ruß gedunkelt und nimmt mehr Sonne auf (0,78) als sein überdeckter Kopf, der unter der Reihe darüber sauberer roter Ton blieb (0,65); Emissionsgrade 0,93 und 0,90. Schätzungen nach ASHRAE."),
            t("Areas are the total outer surfaces of the stored lot: the 12 mm nib gaps between the tiles are open, so both faces of every tile count; the faces of the pallet parts where they lie on each other do not.", "Flächen sind die gesamten Außenflächen der gelagerten Partie: die Nasenspalte von 12 mm zwischen den Ziegeln sind offen, daher zählen beide Flächen jedes Ziegels; die Flächen der Palettenteile, wo sie aufeinander liegen, zählen nicht."),
            t("Stored and lifted, the lot stands in the open yard on a mild sunny day; reinstalled, the lapped heads of the patch lie under the course above and against the battens (contact 0.35) while the tails take the sun.", "Gelagert und angehoben steht die Partie an einem milden sonnigen Tag im offenen Hof; wieder eingedeckt liegen die überdeckten Köpfe der Fläche unter der Reihe darüber und an den Latten (Kontakt 0,35), während die Füße die Sonne aufnehmen."),
        ]},
        "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
            t("Stored, the lot bears through the nine pallet feet on the yard; lifted, on six fork tine contacts under the cross boards; reinstalled, the battens are pinned to two rafters 0.80 m apart (span 0.80 m, 0.20 m overhang) and the pallet keeps standing on its feet.", "Gelagert trägt die Partie über die neun Palettenfüße auf den Hof ab; angehoben über sechs Zinkenkontakte unter den Querbrettern; wieder eingedeckt sind die Latten auf zwei Sparren mit 0,80 m Abstand gelagert (Stützweite 0,80 m, 0,20 m Überstand), und die Palette steht weiter auf ihren Füßen."),
            t("The tiles stand on their long edge and carry their weight in their plane into the deck boards and battens; the straps hold the bundles together and carry nothing. Each patch tile hangs by its nib on its batten and lies on the course below; the rig carries each tile under its batten, so the battens collect the patch weight and snow.", "Die Ziegel stehen auf ihrer Langkante und tragen ihr Gewicht in ihrer Ebene in Deckbretter und Latten ab; die Bänder halten die Bündel zusammen und tragen nichts. Jeder Musterziegel hängt mit seiner Nase an seiner Latte und liegt auf der Reihe darunter; das Rig führt jeden Ziegel unter seiner Latte, so dass die Latten Gewicht und Schnee der Fläche sammeln."),
            t("The product solves the reactions of the whole asset as one rigid body: reinstalled, the pallet with the remaining 138 tiles and the patch share one equilibrium, so the reactions split between pallet feet and rafter pins by the equal-stiffness solution, not by the two separate load paths of reality.", "Das Produkt löst die Reaktionen des ganzen Assets als einen starren Körper: wieder eingedeckt teilen die Palette mit den verbliebenen 138 Ziegeln und die Musterfläche ein Gleichgewicht, daher verteilen sich die Reaktionen nach der Lösung gleicher Steifigkeit auf Palettenfüße und Sparrenauflager, nicht nach den zwei getrennten Lastpfaden der Wirklichkeit."),
            t("Old clay tile taken as E 20 GPa and bending strength 10 MPa; battens S10 (C24) with fm,k 24 MPa, I 1.1e-7 m⁴ about their weak axis; pallet softwood C16.", "Alter Ziegelton angenommen mit E 20 GPa und Biegefestigkeit 10 MPa; Latten S10 (C24) mit fm,k 24 MPa, I 1,1e-7 m⁴ um ihre schwache Achse; Palettennadelholz C16."),
        ]},
        "volumes": {},
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    for s in SCALES:
        b = body(s)
        asset["volumes"][s] = {"status": "authored", "denominator": int(s), "body": b, "thermal": thermal(s, b), "structure": structure(s), "decisions": VOL_DECISIONS[s]}
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


def world_frames(pose_bones):
    """🦴 World rotation and origin of every bone for the given offsets (the product's composition, for the checks printed below)."""
    bones = {b["id"]: b for b in rig()["bones"]}
    out = {}

    def frame(bid):
        if bid in out:
            return out[bid]
        b = bones[bid]
        off = pose_bones.get(bid, {})
        if b["parent"] is None:
            pr, po = [[1, 0, 0], [0, 1, 0], [0, 0, 1]], [0.0, 0.0, 0.0]
        else:
            pr, po = frame(b["parent"])
        j = b["joint"]
        dx, dy = (off.get("dx", 0), off.get("dy", 0)) if j["type"] == "prismatic" else (0, 0)
        o = [po[i] + v for i, v in enumerate(apply(pr, [b["x"] + dx, b["y"] + dy, b.get("z", 0)]))]
        rr = mul(pr, rot_w(b["angleDeg"]))
        if j["type"] == "revolute":
            rr = mul(rr, mul(rot_w(off.get("angleDeg", 0)), rot_u(off.get("twistDeg", 0))))
        out[bid] = (rr, o)
        return out[bid]

    for bid in bones:
        frame(bid)
    return out


def checks():
    """🔍 Prints the masses, areas and the fit of the reinstalled patch."""
    print(f"[DEBUG] tile face {TILE_FACE * 1e4:.2f} cm2, volume {TILE_VOL * 1e6:.1f} cm3, mass {M_TILE:.4f} kg; pallet {M_PALLET:.2f} kg; batten {M_BATTEN:.3f} kg; strap loop {M_BAND:.4f} kg")
    print(f"[DEBUG] tiles in stack {N_STACK}, patch {len(PATCH_TILES)}, total mass {M_TOTAL:.3f} kg; delta {DELTA:.3f} deg")
    print("[DEBUG] areas " + ", ".join(f"{k} {v:.4f}" for k, v in AREAS.items()))
    fr = world_frames(REINSTALL)
    for p in PATCH_TILES[:6]:
        rr, o = fr[p["id"]]
        tgt = tile_roof(p)
        corner = [o[i] + v for i, v in enumerate(apply(rr, [L, W, -T]))]
        print(f"[DEBUG] {p['id']} origin {[round(v, 4) for v in o]} target {[round(v, 4) for v in tgt]} far corner {[round(v, 4) for v in corner]}")
    for bat in BATTENS:
        rr, o = fr[bat]
        print(f"[DEBUG] {bat} origin {[round(v, 4) for v in o]} u {[round(v, 3) for v in (rr[0][0], rr[1][0])]}")


if __name__ == "__main__":
    doc = document()
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    checks()
