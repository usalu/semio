"""🪵 Authoritative generator of the staffage asset `reclaimed-parquet-crate`: every hand-chosen number of the crate of reclaimed oak herringbone blocks is stated here and the asset document `🔣️.json` next to this script is written from them.

Run: python3 author-parquet-crate.py  (needs shapely for the outline of the relaid field)
See ../../brief.md and ../../reports/reclaimed-parquet-crate.md
"""
import json
import math
import os

from shapely.geometry import Polygon
from shapely.ops import unary_union

HERE = os.path.dirname(os.path.abspath(__file__))


def t(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """🎯 Round to a micrometre so the document stays readable."""
    return round(v + 0.0, 6)


BW, BL, BT, BB = 0.070, 0.280, 0.022, 0.0015
PITCH = BT + BB
ROWS_X, ROWS_Z, LAYERS = 4, 10, 18
GAP_X, GAP_Z = 0.003, 0.002
OAK_RHO, BIT_RHO, SOFT_RHO, STEEL_RHO = 700.0, 1050.0, 450.0, 7850.0

X0 = 0.022
PAL_L, PAL_W, PAL_H = 1.2, 0.8, 0.144
BEARER = 0.10
CRATE_H, SLAT_H, SLAT_T, BAT_W, BAT_T = 0.53, 0.10, 0.019, 0.070, 0.022
SLAT_GAP = (CRATE_H - 4 * SLAT_H) / 3
STACK_L = ROWS_X * BL + (ROWS_X - 1) * GAP_X
STACK_W = ROWS_Z * BW + (ROWS_Z - 1) * GAP_Z
STACK_H = LAYERS * PITCH
STACK_X = (PAL_L - STACK_L) / 2
TINE_L, TINE_W, TINE_T, SHANK_H, TINE_Z = 1.15, 0.100, 0.040, 0.462, 0.200
FX = X0 + PAL_L + 0.15
FORK_REACH, FORK_TOUCH, LIFT = -1.20, 0.16, 0.30

BLOCK_U = [(0.0, 0.145), (0.5275, 0.6725), (1.055, 1.2)]
FOOT_Z = [(0.30, 0.40), (-0.0725, 0.0725), (-0.40, -0.30)]
BOTTOM_Z = FOOT_Z
DECK_Z = [(0.255, 0.40), (0.11375, 0.21375), (-0.0725, 0.0725), (-0.21375, -0.11375), (-0.40, -0.255)]
BEARER_Z = [(0.30, 0.40), (-0.40, -0.30)]

ORIGIN = {
    "yard": (0.0, 0.0),
    "bearers": (X0, 0.0),
    "pallet": (X0, BEARER),
    "crate": (X0, BEARER + PAL_H),
    "stack": (X0 + STACK_X, BEARER + PAL_H),
    "fork-reach": (FX, 0.0),
    "forks": (FX, 0.0),
}


def herringbone():
    """🐟 The 27 blocks of the relaid field: a classic herringbone of 70 x 280 mm blocks at plus and minus 45 degrees with the spine along x, the largest set of whole blocks that fits on the top layer of the stack (stack-local x, z)."""
    w, L = BW, BL
    c = math.sqrt(0.5)
    out = []
    for i in range(-12, 13):
        for k in range(-12, 13):
            ox, oy = i * (L + w) + k * w, i * (w - L) + k * w
            for kind, q in (("a", [(ox, oy), (ox + L, oy), (ox + L, oy + w), (ox, oy + w)]), ("b", [(ox + L, oy + w - L), (ox + L + w, oy + w - L), (ox + L + w, oy + w), (ox + L, oy + w)])):
                out.append((kind, [((p[0] + p[1]) * c, (p[1] - p[0]) * c) for p in q]))
    W, H = STACK_L - 0.011, STACK_W
    best = None
    for sx in range(0, 100):
        for sy in range(0, 400, 2):
            fx, fz = sx / 1000.0, sy / 1000.0
            sel = [(kd, q) for kd, q in out if all(fx <= p[0] + 0.6 <= fx + W and fz <= p[1] + 0.6 <= fz + H for p in q)]
            if best is None or len(sel) > len(best[2]):
                best = (fx, fz, sel)
    fx, fz, sel = best
    xs = [p[0] for _, q in sel for p in q]
    zs = [p[1] for _, q in sel for p in q]
    cx, cz = (min(xs) + max(xs)) / 2, (min(zs) + max(zs)) / 2
    blocks = [(kd, [(p[0] - cx + STACK_L / 2, p[1] - cz) for p in q]) for kd, q in sel]
    blocks.sort(key=lambda b: (sum(p[0] for p in b[1]), sum(p[1] for p in b[1])))
    return blocks


FIELD_BLOCKS = herringbone()
FIELD_XMIN = min(p[0] for _, q in FIELD_BLOCKS for p in q)
FIELD_XMAX = max(p[0] for _, q in FIELD_BLOCKS for p in q)
FIELD_ZMIN = min(p[1] for _, q in FIELD_BLOCKS for p in q)
FIELD_ZMAX = max(p[1] for _, q in FIELD_BLOCKS for p in q)
FIELD_L = FIELD_XMAX - FIELD_XMIN
ORIGIN["field"] = (ORIGIN["stack"][0] + FIELD_XMIN, ORIGIN["stack"][1] + STACK_H)
FIELD_TARGET_X = -0.15 - FIELD_L
FIELD_DX, FIELD_DY = FIELD_TARGET_X - ORIGIN["field"][0], -ORIGIN["field"][1]
FIELD_TRAVEL = round(math.hypot(FIELD_DX, FIELD_DY), 6)
FIELD_AXIS = round(math.degrees(math.atan2(FIELD_DY, FIELD_DX)), 4)
FIELD_DX = FIELD_TRAVEL * math.cos(math.radians(FIELD_AXIS))
FIELD_DY = FIELD_TRAVEL * math.sin(math.radians(FIELD_AXIS))
ORIGIN["load-carrier"] = (ORIGIN["field"][0] + 0.15, ORIGIN["field"][1] + PITCH)
CHIPPED = {4: (1, 0.018, 0.012), 13: (3, 0.012, 0.018), 21: (2, 0.018, 0.012)}


def field_poly(i):
    """🧩 The plan outline of field block i in field-local (x, z); three blocks keep an old chipped corner cut back by 18 x 12 mm."""
    kind, q = FIELD_BLOCKS[i]
    pts = [(p[0] - FIELD_XMIN, p[1]) for p in q]
    if i in CHIPPED:
        k, da, db = CHIPPED[i]
        p, prev, nxt = pts[k], pts[k - 1], pts[(k + 1) % 4]
        def toward(o, d):
            n = math.hypot(o[0] - p[0], o[1] - p[1])
            return (p[0] + (o[0] - p[0]) * d / n, p[1] + (o[1] - p[1]) * d / n)
        pts = pts[:k] + [toward(prev, da), toward(nxt, db)] + pts[k + 1:]
    return [(r6(x), r6(z)) for x, z in pts]


FIELD_OUTLINE = unary_union([Polygon(field_poly(i)) for i in range(len(FIELD_BLOCKS))]).buffer(1e-6, join_style=2).buffer(-1e-6, join_style=2)
FIELD_RING = [(r6(x), r6(z)) for x, z in list(FIELD_OUTLINE.exterior.coords)[:-1]]
FIELD_AREA = sum(Polygon(field_poly(i)).area for i in range(len(FIELD_BLOCKS)))


def simplify_ring(ring, tol):
    """✂️ The field outline simplified for a coarse scale (Douglas-Peucker at `tol` metres)."""
    s = Polygon(ring).simplify(tol, preserve_topology=True)
    return [(r6(x), r6(z)) for x, z in list(s.exterior.coords)[:-1]]


MASS = {
    "bearers": 2 * BEARER * BEARER * PAL_L * SOFT_RHO,
    "pallet": 25.0,
    "crate": 0.0,
    "stack": ROWS_X * ROWS_Z * LAYERS * (BL * BW * BT * OAK_RHO + BL * BW * BB * BIT_RHO),
    "field": FIELD_AREA * (BT * OAK_RHO + BB * BIT_RHO),
}
CRATE_VOL = 2 * 4 * PAL_L * SLAT_H * SLAT_T + 2 * 4 * (PAL_W - 2 * SLAT_T) * SLAT_H * SLAT_T + 4 * BAT_W * BAT_T * CRATE_H + 4 * (BAT_T + BAT_W) * BAT_T * CRATE_H + 2 * BAT_W * BAT_T * CRATE_H
MASS["crate"] = CRATE_VOL * SOFT_RHO
PALLET_VOL = PAL_L * BT * 0.345 + 3 * 0.345 * 0.145 * 0.078 + 3 * 0.145 * PAL_W * BT + PAL_L * BT * 0.635
PALLET_RHO = MASS["pallet"] / PALLET_VOL
TOTAL_MASS = sum(MASS.values())


def box(id_, bone, mat, tone, x0, x1, y0, y1, z0, z1, **extra):
    """📦 A box solid from world rest coordinates of its bone (all bones rest unrotated, so local = world - origin)."""
    ox, oy = ORIGIN[bone]
    s = {"id": id_, "kind": "box", "bone": bone, "material": mat, "role": extra.pop("role", "body"), "tone": tone, "size": [r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], "transform": {"translation": [r6((x0 + x1) / 2 - ox), r6((y0 + y1) / 2 - oy), r6((z0 + z1) / 2)]}}
    s.update(extra)
    return s


def ext_xy(id_, bone, mat, tone, profile, z0, z1, **extra):
    """📐 An extrude of a bone-local (u, v) profile along w from z0 to z1."""
    s = {"id": id_, "kind": "extrude", "bone": bone, "material": mat, "role": extra.pop("role", "body"), "tone": tone, "profile": [[r6(u), r6(v)] for u, v in profile], "plane": "xy", "from": r6(z0), "to": r6(z1)}
    s.update(extra)
    return s


def ext_plan(id_, bone, mat, tone, ring, v0, v1, **extra):
    """🗺️ An extrude of a plan ring given as bone-local (u, w) points, pushed along v from v0 to v1 (plane zx takes the profile as (w, u))."""
    pts = [[r6(w), r6(u)] for u, w in ring]
    area = sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))
    if area < 0:
        pts = pts[::-1]
    s = {"id": id_, "kind": "extrude", "bone": bone, "material": mat, "role": extra.pop("role", "body"), "tone": tone, "profile": pts, "plane": "zx", "from": r6(v0), "to": r6(v1)}
    s.update(extra)
    return s


def ref(id_, of, **label):
    """🔗 A reference to a body solid of the same scale."""
    d = {"id": id_, "of": of}
    d.update(label)
    return d


def wy(bone, v):
    """🌐 World height of a bone-local v at rest."""
    return ORIGIN[bone][1] + v


def wx(bone, u):
    """🌐 World x of a bone-local u at rest."""
    return ORIGIN[bone][0] + u


class Parts:
    """🧱 The parts of one scale with their world rest boxes, from which the 3D solids and the three 2D views are drawn to the same numbers."""

    def __init__(self):
        self.items = []

    def add(self, id_, bone, mat, region, tone, x0, x1, y0, y1, z0, z1, views=("elevation", "front", "plan"), **extra):
        self.items.append(dict(id=id_, bone=bone, mat=mat, region=region, tone=tone, box=(x0, x1, y0, y1, z0, z1), views=views, extra=extra))

    def solids(self):
        return [box(p["id"], p["bone"], p["mat"], p["tone"], *p["box"], **{k: v for k, v in p["extra"].items() if k != "group"}) for p in self.items]


def rect_shape(id_, bone, role, z, tone, x, y, w, h, region=None, regions=None, material=None, filled=True, pen=None):
    """▭ A rectangle shape."""
    s = {"id": id_, "bone": bone, "role": role, "z": round(z, 4), "filled": filled, "tone": tone, "kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if pen:
        s["penMm"] = pen
    return s


def poly_shape(id_, bone, role, z, tone, pts, closed=True, filled=True, region=None, regions=None, material=None, pen=None):
    """⬠ A polygon or polyline shape."""
    s = {"id": id_, "bone": bone, "role": role, "z": round(z, 4), "filled": filled, "tone": tone, "kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if pen:
        s["penMm"] = pen
    return s


def union_shapes(id_, bone, z, tone, rects, region=None, regions=None, material=None, fill_holes=False):
    """🧷 One filled shape per connected piece of the union of rectangles of one group: a poly, or an even-odd path when the piece has holes (the gaps between slats, the fork openings of the pallet)."""
    geom = unary_union([Polygon([(x, y), (x + w, y), (x + w, y + h), (x, y + h)]) for x, y, w, h in rects])
    pieces = list(geom.geoms) if hasattr(geom, "geoms") else [geom]
    pieces.sort(key=lambda g: (g.bounds[0], g.bounds[1]))
    out = []
    for n, g in enumerate(pieces):
        g = g.simplify(1e-7)
        sid = id_ if len(pieces) == 1 else f"{id_}-{n}"
        ext = [(r6(x), r6(y)) for x, y in list(g.exterior.coords)[:-1]]
        holes = [] if fill_holes else [[(r6(x), r6(y)) for x, y in list(h.coords)[:-1]] for h in g.interiors]
        if not holes:
            out.append(poly_shape(sid, bone, "body", z, tone, ext, region=region, regions=regions, material=material))
        else:
            d = " ".join("M " + " L ".join(f"{x:g} {y:g}" for x, y in ring) + " Z" for ring in [ext] + holes)
            sh = {"id": sid, "bone": bone, "role": "body", "z": round(z, 4), "filled": True, "tone": tone, "kind": "path", "d": d}
            if region:
                sh["region"] = region
            if regions:
                sh["regions"] = regions
            if material:
                sh["material"] = material
            out.append(sh)
    return out


def view_shapes(parts, fill_holes=()):
    """🖼️ The parts of each group joined into filled shapes in the three views: elevation bone-local, front (world z, world y) and plan (world x, world z) in rest coordinates; nearer groups paint later."""
    out = {"elevation": [], "front": [], "plan": []}
    groups = {}
    for p in parts.items:
        x0, x1, y0, y1, z0, z1 = p["box"]
        ox, oy = ORIGIN[p["bone"]]
        g = p["extra"].get("group", p["bone"])
        for view in p["views"]:
            key = (view, g)
            e = groups.setdefault(key, dict(bone=p["bone"], region=p["region"], mat=p["mat"], tone=p["tone"], rects=[], z=-9))
            if view == "elevation":
                e["rects"].append((x0 - ox, y0 - oy, x1 - x0, y1 - y0)); e["z"] = max(e["z"], {"bearers": 2.6, "pallet": 2.55}.get(g, 2 - z0))
            elif view == "front":
                e["rects"].append((z0, y0, z1 - z0, y1 - y0)); e["z"] = max(e["z"], {"bearers": 4.05, "pallet": 4.0}.get(g, 2 + x1))
            else:
                e["rects"].append((x0, z0, x1 - x0, z1 - z0)); e["z"] = max(e["z"], 2 + y1)
    for (view, g), e in groups.items():
        out[view] += union_shapes(g, e["bone"], e["z"], e["tone"], e["rects"], region=e["region"], material=e["mat"], fill_holes=(g in fill_holes and view != "plan"))
    return out


def bearers_parts(P, scale):
    """🪵 The two reused squared-timber bearers 100 x 100 x 1200 mm under the outer bottom boards of the pallet."""
    for k, (z0, z1) in enumerate(BEARER_Z):
        P.add(f"bearer-{'left' if k == 0 else 'right'}", "bearers", "softwood-weathered", "bearer-timber", 0.42 + 0.04 * k, X0, X0 + PAL_L, 0, BEARER, z0, z1, views=("elevation", "front") if k == 0 else ("front",))


def pallet_parts(P, scale):
    """🟫 The EPAL Euro pallet 1200 x 800 x 144 mm (board by board at 1:50 and 1:100, generalised below)."""
    y = BEARER
    if scale in (50, 100):
        for k, (z0, z1) in enumerate(BOTTOM_Z):
            P.add(f"pallet-bottom-{k}", "pallet", "pallet-timber", "pallet-timber", 0.5, X0, X0 + PAL_L, y, y + BT, z0, z1, views=("elevation", "front") if k == 2 else ("front",))
        for i, (u0, u1) in enumerate(BLOCK_U):
            for k, (z0, z1) in enumerate(FOOT_Z):
                P.add(f"pallet-block-{i}{k}", "pallet", "pallet-timber", "pallet-timber", 0.38, X0 + u0, X0 + u1, y + BT, y + 0.1, z0, z1, views=("elevation", "front") if k == 2 else (("front",) if i == 2 else ()))
        for i, (u0, u1) in enumerate(BLOCK_U):
            P.add(f"pallet-stringer-{i}", "pallet", "pallet-timber", "pallet-timber", 0.46, X0 + u0, X0 + u1, y + 0.1, y + 0.122, -PAL_W / 2, PAL_W / 2, views=("elevation", "front", "plan") if i == 2 else ("elevation", "plan"))
        for k, (z0, z1) in enumerate(DECK_Z):
            P.add(f"pallet-deck-{k}", "pallet", "pallet-timber", "pallet-timber", 0.54, X0, X0 + PAL_L, y + 0.122, y + PAL_H, z0, z1, views=("elevation", "front", "plan") if k == 4 else ("front", "plan"))
    elif scale == 200:
        for i, (u0, u1) in enumerate(BLOCK_U):
            for k, (z0, z1) in enumerate(FOOT_Z):
                P.add(f"pallet-foot-{i}{k}", "pallet", "pallet-timber", "pallet-timber", 0.4, X0 + u0, X0 + u1, y, y + 0.1, z0, z1, views=("elevation", "front") if k == 2 else (("front",) if i == 2 else ()))
        P.add("pallet-deck", "pallet", "pallet-timber", "pallet-timber", 0.52, X0, X0 + PAL_L, y + 0.1, y + PAL_H, -PAL_W / 2, PAL_W / 2, views=("elevation", "front", "plan"))


def crate_parts(P, scale):
    """🧺 The slatted spruce crate nailed onto the pallet deck: four rows of 100 x 19 mm slats with 33 mm gaps, vertical 70 x 22 mm battens outside at the corners and the middle of the long walls."""
    y0 = BEARER + PAL_H
    if scale == 500:
        P.add("pallet-block", "pallet", "pallet-timber", "pallet-timber", 0.45, X0, X0 + PAL_L, BEARER, y0, -PAL_W / 2, PAL_W / 2)
        P.add("crate-mass", "crate", "softwood-crate", "crate-timber", 0.56, X0 - BAT_T, X0 + PAL_L + BAT_T, y0, y0 + CRATE_H, -PAL_W / 2 - BAT_T, PAL_W / 2 + BAT_T)
        return
    wt = SLAT_T if scale in (50, 100) else 0.030
    bt = BAT_T if scale in (50, 100) else 0.030
    if scale in (50, 100):
        for k in range(4):
            v0 = y0 + k * (SLAT_H + SLAT_GAP)
            tone = 0.62 + 0.03 * ((k * 7) % 3)
            P.add(f"slat-right-{k}", "crate", "softwood-crate", "crate-timber", tone, X0, X0 + PAL_L, v0, v0 + SLAT_H, -PAL_W / 2, -PAL_W / 2 + wt, views=("elevation", "plan") if k == 3 else ("elevation",))
            P.add(f"slat-left-{k}", "crate", "softwood-crate", "crate-timber", tone, X0, X0 + PAL_L, v0, v0 + SLAT_H, PAL_W / 2 - wt, PAL_W / 2, views=("plan",) if k == 3 else ())
            P.add(f"slat-front-{k}", "crate", "softwood-crate", "crate-timber", tone - 0.03, X0 + PAL_L - wt, X0 + PAL_L, v0, v0 + SLAT_H, -PAL_W / 2 + wt, PAL_W / 2 - wt, views=("front", "plan") if k == 3 else ("front",))
            P.add(f"slat-back-{k}", "crate", "softwood-crate", "crate-timber", tone - 0.03, X0, X0 + wt, v0, v0 + SLAT_H, -PAL_W / 2 + wt, PAL_W / 2 - wt, views=("plan",) if k == 3 else ())
    else:
        P.add("wall-right", "crate", "softwood-crate", "crate-timber", 0.62, X0, X0 + PAL_L, y0, y0 + CRATE_H, -PAL_W / 2, -PAL_W / 2 + wt, views=("elevation", "plan"))
        P.add("wall-left", "crate", "softwood-crate", "crate-timber", 0.62, X0, X0 + PAL_L, y0, y0 + CRATE_H, PAL_W / 2 - wt, PAL_W / 2, views=("plan",))
        P.add("wall-front", "crate", "softwood-crate", "crate-timber", 0.6, X0 + PAL_L - wt, X0 + PAL_L, y0, y0 + CRATE_H, -PAL_W / 2 + wt, PAL_W / 2 - wt, views=("front", "plan"))
        P.add("wall-back", "crate", "softwood-crate", "crate-timber", 0.6, X0, X0 + wt, y0, y0 + CRATE_H, -PAL_W / 2 + wt, PAL_W / 2 - wt, views=("plan",))
    for side, (za, zb) in (("right", (-PAL_W / 2 - bt, -PAL_W / 2)), ("left", (PAL_W / 2, PAL_W / 2 + bt))):
        ev = ("elevation", "plan") if side == "right" else ("plan",)
        P.add(f"batten-{side}-back", "crate", "softwood-crate", "crate-timber", 0.5, X0, X0 + BAT_W, y0, y0 + CRATE_H, za, zb, views=ev)
        P.add(f"batten-{side}-mid", "crate", "softwood-crate", "crate-timber", 0.5, X0 + PAL_L / 2 - BAT_W / 2, X0 + PAL_L / 2 + BAT_W / 2, y0, y0 + CRATE_H, za, zb, views=ev)
        P.add(f"batten-{side}-front", "crate", "softwood-crate", "crate-timber", 0.5, X0 + PAL_L - BAT_W, X0 + PAL_L, y0, y0 + CRATE_H, za, zb, views=ev)
        zc = (PAL_W / 2 - BAT_W, PAL_W / 2 + bt) if side == "left" else (-PAL_W / 2 - bt, -PAL_W / 2 + BAT_W)
        P.add(f"batten-{side}-front-end", "crate", "softwood-crate", "crate-timber", 0.46, X0 + PAL_L, X0 + PAL_L + bt, y0, y0 + CRATE_H, zc[0], zc[1], views=("elevation", "front", "plan") if side == "right" else ("front", "plan"))
        P.add(f"batten-{side}-back-end", "crate", "softwood-crate", "crate-timber", 0.46, X0 - bt, X0, y0, y0 + CRATE_H, zc[0], zc[1], views=("elevation", "plan") if side == "right" else ("plan",))


def stack_parts(P, scale):
    """🧱 The stacked blocks: 18 layers of 4 x 10 blocks lying flat with the grain along x, the bitumen remnants of each underside on the block below."""
    if scale == 500:
        return
    y0 = BEARER + PAL_H
    xs = X0 + STACK_X
    if scale == 50:
        return
    P.add("stack", "stack", "oak-reclaimed", "stack-blocks", 0.5, xs, xs + STACK_L, y0, y0 + STACK_H, -STACK_W / 2, STACK_W / 2)


def stack_solids_50():
    """🧱 The 1:50 stack: every block a box of oak 280 x 70 x 22 mm on a 1.5 mm bitumen remnant sheet, rows repeated over the 18 layers."""
    out = []
    xs = STACK_X
    for i in range(ROWS_X):
        for j in range(ROWS_Z):
            u0 = xs - STACK_X + i * (BL + GAP_X)
            z0 = -STACK_W / 2 + j * (BW + GAP_Z)
            tone = [0.5, 0.56, 0.47, 0.53, 0.59][(i * 3 + j * 2) % 5]
            rep = {"count": LAYERS, "translation": [0, PITCH, 0]}
            out.append({"id": f"block-{i}-{j}", "kind": "box", "bone": "stack", "material": "oak-reclaimed", "role": "body", "tone": tone, "size": [BL, BT, BW], "transform": {"translation": [r6(u0 + BL / 2), r6(BB + BT / 2), r6(z0 + BW / 2)]}, "repeat": rep})
            out.append({"id": f"bitumen-{i}-{j}", "kind": "box", "bone": "stack", "material": "bitumen", "role": "body", "tone": 0.12, "size": [BL, BB, BW], "sheet": True, "transform": {"translation": [r6(u0 + BL / 2), r6(BB / 2), r6(z0 + BW / 2)]}, "repeat": rep})
    return out


def field_thickness(scale):
    """📏 The modelled thickness of the field (oak + bitumen) per scale: true 23.5 mm at 1:50 and 1:100, the smallest feature at 1:200 and 1:500."""
    return {50: PITCH, 100: PITCH, 200: 0.030, 500: 0.100}[scale]


def field_solids(scale):
    """🐟 The relaid field: 27 blocks one by one at 1:50, their joined outline at 1:100, the outline at the smallest feature thickness further out."""
    out = []
    tones = [0.52, 0.58, 0.46, 0.55, 0.61, 0.49]
    if scale == 50:
        for i in range(len(FIELD_BLOCKS)):
            ring = field_poly(i)
            out.append(ext_plan(f"field-block-{i:02d}", "field", "oak-reclaimed", tones[i % 6], ring, BB, PITCH))
            out.append(ext_plan(f"field-bitumen-{i:02d}", "field", "bitumen", 0.12, ring, 0.0, BB, sheet=True))
        return out
    if scale == 100:
        out.append(ext_plan("field-oak", "field", "oak-reclaimed", 0.54, FIELD_RING, BB, PITCH))
        out.append(ext_plan("field-bitumen", "field", "bitumen", 0.12, FIELD_RING, 0.0, BB, sheet=True))
        return out
    ring = simplify_ring(FIELD_RING, 0.004 if scale == 200 else 0.01)
    out.append(ext_plan("field-oak", "field", "oak-reclaimed", 0.54, ring, 0.0, field_thickness(scale)))
    return out


def fork_profile(scale):
    """🍴 The side profile of one fork arm (ISO 2328 class 2, 1150 x 100 x 40 mm blade, 462 mm shank with the upper carriage hook), in forks-local (u from the tip, v up)."""
    if scale == 50:
        return [(0, 0), (1.15, 0), (1.15, 0.462), (1.09, 0.462), (1.09, 0.42), (1.11, 0.42), (1.11, 0.07), (1.104, 0.055), (1.09, 0.044), (1.07, 0.04), (0.16, 0.04), (0, 0.012)]
    if scale == 100:
        return [(0, 0), (1.15, 0), (1.15, 0.462), (1.09, 0.462), (1.09, 0.42), (1.11, 0.42), (1.11, 0.06), (1.08, 0.04), (0.16, 0.04), (0, 0.02)]
    if scale == 200:
        return [(0, 0), (1.15, 0), (1.15, 0.462), (1.11, 0.462), (1.11, 0.04), (0, 0.04)]
    return [(0, 0), (1.15, 0), (1.15, 0.462), (1.05, 0.462), (1.05, 0.1), (0, 0.1)]


def fork_solids(scale):
    """🍴 The two fork arms of the forklift carriage, lowered on the ground in front of the pallet."""
    return [ext_xy(f"fork-{side}", "forks", "steel-fork", 0.26, fork_profile(scale), sgn * TINE_Z - TINE_W / 2, sgn * TINE_Z + TINE_W / 2) for side, sgn in (("left", 1), ("right", -1))]


def build_scale(scale):
    """🏗️ Parts, solids and 2D shapes of one preset scale."""
    P = Parts()
    bearers_parts(P, scale)
    pallet_parts(P, scale)
    crate_parts(P, scale)
    stack_parts(P, scale)
    body = P.solids()
    if scale == 50:
        body += stack_solids_50()
    body += field_solids(scale)
    body += fork_solids(scale)
    shapes = view_shapes(P, fill_holes=("crate",) if scale in (100, 200) else ())
    return P, body, shapes


def field_2d(scale, shapes):
    """🐟 The field in the three views: in stored pose it lies on the stack inside the crate (elevation and front behind the slats, plan seen from above)."""
    th = field_thickness(scale)
    oy = ORIGIN["field"][1]
    fz0, fz1 = FIELD_ZMIN, FIELD_ZMAX
    if scale in (50, 100):
        shapes["elevation"].append(rect_shape("field-oak", "field", "body", 2 - fz0 - 0.05, 0.54, 0, BB, FIELD_L, PITCH - BB, region="field-face", material="oak-reclaimed"))
        shapes["elevation"].append(rect_shape("field-bitumen", "field", "body", 2 - fz0 - 0.049, 0.12, 0, 0, FIELD_L, BB, region="field-bitumen", material="bitumen"))
        shapes["front"].append(rect_shape("field-oak", "field", "body", 2 + ORIGIN["field"][0] + FIELD_L - 0.05, 0.54, fz0, oy + BB, fz1 - fz0, PITCH - BB, region="field-face", material="oak-reclaimed"))
        shapes["front"].append(rect_shape("field-bitumen", "field", "body", 2 + ORIGIN["field"][0] + FIELD_L - 0.049, 0.12, fz0, oy, fz1 - fz0, BB, region="field-bitumen", material="bitumen"))
    else:
        shapes["elevation"].append(rect_shape("field-oak", "field", "body", 2 - fz0 - 0.05, 0.54, 0, 0, FIELD_L, th, regions=["field-face", "field-bitumen"], material="oak-reclaimed"))
        shapes["front"].append(rect_shape("field-oak", "field", "body", 2 + ORIGIN["field"][0] + FIELD_L - 0.05, 0.54, fz0, oy, fz1 - fz0, th, regions=["field-face", "field-bitumen"], material="oak-reclaimed"))
    fx = ORIGIN["field"][0]
    ztop = 2 + oy + th
    if scale == 50:
        tones = [0.52, 0.58, 0.46, 0.55, 0.61, 0.49]
        for i in range(len(FIELD_BLOCKS)):
            ring = field_poly(i)
            shapes["plan"].append(poly_shape(f"field-block-{i:02d}", "field", "body", ztop, tones[i % 6], [(fx + u, w) for u, w in ring], region="field-face", material="oak-reclaimed"))
    else:
        ring = FIELD_RING if scale == 100 else simplify_ring(FIELD_RING, 0.004 if scale == 200 else 0.01)
        shapes["plan"].append(poly_shape("field-oak", "field", "body", ztop, 0.54, [(fx + u, w) for u, w in ring], region="field-face", material="oak-reclaimed"))
        if scale in (100, 200):
            for i in range(len(FIELD_BLOCKS)):
                ring_i = field_poly(i)
                shapes["plan"].append(poly_shape(f"field-joint-{i:02d}", "field", "detail" if scale == 200 else "texture", ztop + 0.001, 0.3, [(fx + u, w) for u, w in ring_i], closed=True, filled=False, region="field-face"))


def stack_2d(scale, shapes):
    """🧱 The stack in the three views: one filled face per view with the joints of the blocks as texture lines at 1:50 and 1:100."""
    if scale == 500:
        return
    y0 = BEARER + PAL_H
    xs = X0 + STACK_X
    if scale == 50:
        shapes["elevation"].append(rect_shape("stack", "stack", "body", 2 + STACK_W / 2, 0.5, 0, 0, STACK_L, STACK_H, region="stack-blocks", material="oak-reclaimed"))
        shapes["front"].append(rect_shape("stack", "stack", "body", 2 + xs + STACK_L, 0.5, -STACK_W / 2, y0, STACK_W, STACK_H, region="stack-blocks", material="oak-reclaimed"))
        shapes["plan"].append(rect_shape("stack", "stack", "body", 2 + y0 + STACK_H, 0.5, xs, -STACK_W / 2, STACK_L, STACK_W, region="stack-blocks", material="oak-reclaimed"))
    texture = "texture" if scale == 50 else "detail"
    zt = 2 + STACK_W / 2 + 0.001
    for k in range(1, LAYERS) if scale == 50 else range(3, LAYERS, 3):
        shapes["elevation"].append(poly_shape(f"stack-layer-{k:02d}", "stack", texture, zt, 0.3, [(0, k * PITCH), (STACK_L, k * PITCH)], closed=False, filled=False, region="stack-blocks"))
        shapes["front"].append(poly_shape(f"stack-layer-{k:02d}", "stack", texture, 2 + xs + STACK_L + 0.001, 0.3, [(-STACK_W / 2, y0 + k * PITCH), (STACK_W / 2, y0 + k * PITCH)], closed=False, filled=False, region="stack-blocks"))
    for i in range(1, ROWS_X):
        u = i * (BL + GAP_X) - GAP_X / 2
        shapes["elevation"].append(poly_shape(f"stack-end-joint-{i}", "stack", texture, zt, 0.3, [(u, 0), (u, STACK_H)], closed=False, filled=False, region="stack-blocks"))
        if scale == 50:
            shapes["plan"].append(poly_shape(f"stack-end-joint-{i}", "stack", texture, 2 + y0 + STACK_H + 0.001, 0.3, [(xs + u, -STACK_W / 2), (xs + u, STACK_W / 2)], closed=False, filled=False, region="stack-blocks"))
    for j in range(1, ROWS_Z) if scale == 50 else range(2, ROWS_Z, 2):
        z = -STACK_W / 2 + j * (BW + GAP_Z) - GAP_Z / 2
        shapes["front"].append(poly_shape(f"stack-row-joint-{j}", "stack", texture, 2 + xs + STACK_L + 0.001, 0.3, [(z, y0), (z, y0 + STACK_H)], closed=False, filled=False, region="stack-blocks"))
        if scale == 50:
            shapes["plan"].append(poly_shape(f"stack-row-joint-{j}", "stack", texture, 2 + y0 + STACK_H + 0.001, 0.3, [(xs, z), (xs + STACK_L, z)], closed=False, filled=False, region="stack-blocks"))


def fork_2d(scale, shapes):
    """🍴 The fork arms: their side profile, their shanks from ahead and their blades from above."""
    prof = fork_profile(scale)
    shapes["elevation"].append(poly_shape("fork-right", "forks", "body", 2 + TINE_Z + TINE_W / 2, 0.26, prof, region="fork-steel", material="steel-fork"))
    for side, sgn in (("left", 1), ("right", -1)):
        z0 = sgn * TINE_Z - TINE_W / 2
        shapes["front"].append(rect_shape(f"fork-{side}", "forks", "body", 2 + FX + TINE_L, 0.26, z0, 0, TINE_W, SHANK_H, region="fork-steel", material="steel-fork"))
        shapes["plan"].append(rect_shape(f"fork-{side}", "forks", "body", 2 + SHANK_H, 0.26, FX, z0, TINE_L, TINE_W, region="fork-steel", material="steel-fork"))


def detail_2d(scale, shapes):
    """✏️ The traces and marks drawn as strokes: old nail holes of the bearers, the EPAL marks on the pallet blocks, the nails of the crate slats and the grain of the oak."""
    if scale == 50:
        for k, u in enumerate([0.18, 0.47, 0.83, 1.05]):
            shapes["elevation"].append({"id": f"bearer-nail-hole-{k}", "bone": "bearers", "role": "texture", "z": 3.0, "filled": False, "tone": 0.2, "kind": "ellipse", "cx": u, "cy": 0.055 + 0.012 * (k % 2), "rx": 0.004, "ry": 0.004, "region": "bearer-timber"})
        for i, (u0, u1) in enumerate(BLOCK_U):
            shapes["elevation"].append(poly_shape(f"epal-mark-{i}", "pallet", "texture", 3.0, 0.2, [(u0 + 0.03, 0.045), (u1 - 0.03, 0.045)], closed=False, filled=False, region="pallet-timber"))
        y0 = 0.0
        for k in range(4):
            v = y0 + k * (SLAT_H + SLAT_GAP) + SLAT_H / 2
            for u in (0.035, PAL_L / 2, PAL_L - 0.035):
                shapes["elevation"].append({"id": f"slat-nail-{k}-{int(u * 1000)}", "bone": "crate", "role": "texture", "z": 3.0, "filled": False, "tone": 0.2, "kind": "ellipse", "cx": r6(u), "cy": r6(v), "rx": 0.003, "ry": 0.003, "region": "crate-timber"})
    if scale in (100, 200):
        for k in range(1, 4):
            v = k * (SLAT_H + SLAT_GAP) - SLAT_GAP / 2
            shapes["elevation"].append(poly_shape(f"slat-joint-{k}", "crate", "detail", 2.5, 0.3, [(0, v), (PAL_L, v)], closed=False, filled=False, region="crate-timber"))
            shapes["front"].append(poly_shape(f"slat-joint-{k}", "crate", "detail", 2 + X0 + PAL_L + 0.01, 0.3, [(-PAL_W / 2, BEARER + PAL_H + v), (PAL_W / 2, BEARER + PAL_H + v)], closed=False, filled=False, region="crate-timber"))


def crate500_2d(shapes):
    """🧺 The 1:500 crate mass carries the stack in all three views; its outline names every region it merges."""
    for view in ("elevation", "front", "plan"):
        for s in shapes[view]:
            if s["id"] == "crate":
                s.pop("region", None)
                s["regions"] = ["crate-timber", "stack-blocks"]


DECISIONS = {
    50: {
        "silhouette": t("Side elevation: the two 100 mm bearers, the 144 mm Euro pallet with its blocks, stringer ends and fork openings, the crate wall of four 100 mm slats with 33 mm gaps between three vertical battens, and behind the gaps the stack of oak blocks with all 17 layer joints; in front of it the lowered fork arm. Front elevation: the 800 mm face of pallet and crate with the block ends of the stack in a 72 x 23.5 mm grid behind the slat gaps and the two fork shanks. Plan: the crate rim, the battens, the top layer of the stack and the relaid herringbone sample of 27 blocks on it, the fork blades in front.", "Seitenansicht: die beiden 100-mm-Kanthölzer, die 144 mm hohe Europalette mit Klötzen, Querbrettenden und Einfahröffnungen, die Kistenwand aus vier 100-mm-Latten mit 33 mm Abstand zwischen drei senkrechten Leisten und hinter den Lücken der Stapel der Eichenstäbe mit allen 17 Lagenfugen; davor der abgesenkte Gabelzinken. Vorderansicht: die 800-mm-Stirnseite von Palette und Kiste mit den Stabenden des Stapels im Raster 72 x 23,5 mm hinter den Lattenlücken und den beiden Gabelrücken. Draufsicht: der Kistenrand, die Leisten, die oberste Stapellage und darauf die neu gelegte Fischgrätprobe aus 27 Stäben, davor die Gabelzinken."),
        "retained": t("Every slat, batten, pallet board and block at its true place; every block joint of the stack (layers 23.5 mm = 0.47 mm, rows 72 mm, ends 283 mm); every one of the 27 field blocks with its own outline, three with a chipped corner of 18 x 12 mm; the 1.5 mm bitumen remnant under the field as a band.", "Jede Latte, Leiste, jedes Palettenbrett und jeder Klotz an seiner wahren Stelle; jede Stabfuge des Stapels (Lagen 23,5 mm = 0,47 mm, Reihen 72 mm, Enden 283 mm); jeder der 27 Stäbe der Probefläche mit eigener Kontur, drei mit einer abgeplatzten Ecke von 18 x 12 mm; der 1,5-mm-Bitumenrest unter der Fläche als Band."),
        "merged": t("The two bearers and the three pallet rows coincide in side view and are drawn once; the stack is one filled face per view with its joints as texture, the bitumen of the stacked blocks lies inside the layer joints.", "Die beiden Kanthölzer und die drei Palettenreihen decken sich in der Seitenansicht und werden einmal gezeichnet; der Stapel ist je Ansicht eine gefüllte Fläche mit den Fugen als Textur, das Bitumen der gestapelten Stäbe liegt in den Lagenfugen."),
        "omitted": t("Tongue and groove of the blocks (6 mm, 0.12 mm on paper), nail heads of the pallet, the 1 to 3 mm gaps between the rows of the stack.", "Nut und Feder der Stäbe (6 mm, 0,12 mm auf Papier), die Nagelköpfe der Palette, die 1 bis 3 mm breiten Spalten zwischen den Stapelreihen."),
        "thinElements": t("Layer, row and end joints of the stack, the field joints and the nails as 0.1 mm texture; the bitumen band at its true 1.5 mm as a filled strip in elevation and front.", "Lagen-, Reihen- und Stoßfugen des Stapels, die Fugen der Probefläche und die Nägel als 0,1-mm-Textur; das Bitumenband in seiner wahren Dicke von 1,5 mm als gefüllter Streifen in Seiten- und Vorderansicht."),
        "hatching": t("No hatch: oak, spruce and steel read by tone; the field blocks alternate in six oak tones as reclaimed blocks do.", "Keine Schraffur: Eiche, Fichte und Stahl lesen sich über den Ton; die Stäbe der Probefläche wechseln in sechs Eichentönen wie gebrauchte Stäbe."),
        "styles": t("Body pen 0.35 mm for every part, texture 0.1 mm; weathered grey spruce lighter than the dark oak, bitumen black, the fork steel dark.", "Körperfeder 0,35 mm für jedes Teil, Textur 0,1 mm; vergraute Fichte heller als die dunkle Eiche, Bitumen schwarz, der Gabelstahl dunkel."),
        "overlays": t("Ground line under bearers and forks; no dimensions.", "Bodenlinie unter Kanthölzern und Gabel; keine Maße."),
    },
    100: {
        "silhouette": t("The same parts as at 1:50 with the slats, battens and pallet boards at their true size (19 to 145 mm = 0.19 to 1.45 mm); the stack shows every third layer joint and its three end joints, the field its 27 joints as texture inside one filled outline.", "Dieselben Teile wie bei 1:50 mit Latten, Leisten und Palettenbrettern in wahrer Größe (19 bis 145 mm = 0,19 bis 1,45 mm); der Stapel zeigt jede dritte Lagenfuge und seine drei Stoßfugen, die Probefläche ihre 27 Fugen als Textur in einer gefüllten Kontur."),
        "retained": t("Four slat rows with their 33 mm gaps (0.33 mm), all battens, the board-by-board pallet with its fork openings, the zigzag outline of the herringbone field, the fork profile with its 20 mm hook.", "Vier Lattenreihen mit ihren 33-mm-Lücken (0,33 mm), alle Leisten, die Palette Brett für Brett mit ihren Einfahröffnungen, die Zickzackkontur der Fischgrätfläche, das Gabelprofil mit seinem 20-mm-Haken."),
        "merged": t("The 27 field blocks into one outline; the stack into one face with every third layer joint (70.5 mm).", "Die 27 Stäbe der Probefläche zu einer Kontur; der Stapel zu einer Fläche mit jeder dritten Lagenfuge (70,5 mm)."),
        "omitted": t("Nails, nail holes, the EPAL marks, the chipped corners (18 mm = 0.18 mm, inside the outline pen), the heel radius of the fork.", "Nägel, Nagellöcher, die EPAL-Zeichen, die abgeplatzten Ecken (18 mm = 0,18 mm, innerhalb der Konturfeder), der Rückenradius der Gabel."),
        "thinElements": t("The 1.5 mm bitumen band remains as a filled strip under the field; joints as 0.1 mm texture and 0.13 mm detail.", "Das 1,5-mm-Bitumenband bleibt als gefüllter Streifen unter der Fläche; Fugen als 0,1-mm-Textur und 0,13-mm-Detail."),
        "hatching": t("No hatch, tones as at 1:50.", "Keine Schraffur, Töne wie bei 1:50."),
        "styles": t("Body pen 0.25 mm, detail 0.13 mm, texture 0.1 mm.", "Körperfeder 0,25 mm, Detail 0,13 mm, Textur 0,1 mm."),
        "overlays": t("Ground line only.", "Nur die Bodenlinie."),
    },
    200: {
        "silhouette": t("Bearers, a pallet of nine feet under a 44 mm deck, a crate of four 30 mm walls with the three slat joints as detail lines and battens of 30 mm, the stack and the field inside, the fork with its 40 mm blade.", "Kanthölzer, eine Palette aus neun Füßen unter einem 44-mm-Deck, eine Kiste aus vier 30-mm-Wänden mit den drei Lattenfugen als Detaillinien und 30-mm-Leisten, Stapel und Probefläche darin, die Gabel mit ihrem 40-mm-Zinken."),
        "retained": t("The fork openings of 382 x 100 mm (1.9 x 0.5 mm) in side and front view, the 100 mm bearers, the field outline at 30 mm (0.15 mm) thickness.", "Die Einfahröffnungen von 382 x 100 mm (1,9 x 0,5 mm) in Seiten- und Vorderansicht, die 100-mm-Kanthölzer, die Kontur der Probefläche in 30 mm (0,15 mm) Dicke."),
        "merged": t("Bottom boards into the feet, stringer and top deck boards into one deck, the slats of a wall into one 30 mm wall (the 19 mm slat is drawn at the 30 mm print minimum), the layers of the stack into one face, the field joints as detail.", "Unterbretter in die Füße, Querbretter und Deckbretter zu einem Deck, die Latten einer Wand zu einer 30-mm-Wand (die 19-mm-Latte wird mit dem Druckminimum von 30 mm gezeichnet), die Lagen des Stapels zu einer Fläche, die Fugen der Probefläche als Detail."),
        "omitted": t("Slat gaps (33 mm, closed into the wall), the stack joints, the fork hook and tip taper, the bitumen band (merged into the field).", "Lattenlücken (33 mm, in die Wand geschlossen), die Stapelfugen, Gabelhaken und Spitzenverjüngung, das Bitumenband (in die Probefläche aufgegangen)."),
        "thinElements": t("Field joints and slat joints as 0.1 mm detail lines.", "Fugen der Probefläche und Lattenfugen als 0,1-mm-Detaillinien."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Body pen 0.18 mm, detail 0.1 mm.", "Körperfeder 0,18 mm, Detail 0,1 mm."),
        "overlays": t("Ground line only.", "Nur die Bodenlinie."),
    },
    500: {
        "silhouette": t("Two bearers, nine pallet feet with the fork openings between them, one mass of crate and deck from 200 to 744 mm, the field as a 75 mm slab inside it, the L of the fork arms at 75 mm.", "Zwei Kanthölzer, neun Palettenfüße mit den Einfahröffnungen dazwischen, eine Masse aus Kiste und Deck von 200 bis 744 mm, die Probefläche als 75-mm-Platte darin, das L der Gabelzinken in 75 mm."),
        "retained": t("Overall size 2.52 x 0.84 x 0.74 m, the 100 mm bearers and the 100 mm fork openings (0.2 mm), the fork arms at the 75 mm print minimum.", "Gesamtgröße 2,52 x 0,84 x 0,74 m, die 100-mm-Kanthölzer und die 100-mm-Einfahröffnungen (0,2 mm), die Gabelzinken im Druckminimum von 75 mm."),
        "merged": t("Crate, stack and the pallet deck into one mass; the field into a 75 mm slab that leaves the mass in the reinstalled pose.", "Kiste, Stapel und Palettendeck zu einer Masse; die Probefläche zu einer 75-mm-Platte, die in der wiederverbauten Pose die Masse verlässt."),
        "omitted": t("Slats, battens, joints and every mark.", "Latten, Leisten, Fugen und alle Zeichen."),
        "thinElements": t("None: every part is at least 75 mm (0.15 mm).", "Keine: jedes Teil ist mindestens 75 mm (0,15 mm)."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Body pen 0.13 mm.", "Körperfeder 0,13 mm."),
        "overlays": t("Ground line only.", "Nur die Bodenlinie."),
    },
}

PENS = {
    50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
    100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35},
    200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25},
    500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18},
}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def thermal_list(scale, body):
    """🌡️ The thermal model made spatial: references to the body where the region surface is that body surface, the stack as its own envelope box."""
    out = []
    ids = [s["id"] for s in body]
    for s in body:
        sid = s["id"]
        if sid.startswith("bearer-"):
            out.append(ref(f"th-{sid}", sid, region="bearer-timber"))
        elif sid.startswith("pallet-"):
            out.append(ref(f"th-{sid}", sid, region="pallet-timber"))
        elif sid.startswith(("slat-", "batten-", "wall-", "crate-mass")):
            out.append(ref(f"th-{sid}", sid, region="crate-timber"))
        elif sid.startswith("fork-"):
            out.append(ref(f"th-{sid}", sid, region="fork-steel"))
        elif sid.startswith("field-block-") or sid == "field-oak":
            if scale in (200, 500):
                ring = simplify_ring(FIELD_RING, 0.004 if scale == 200 else 0.01)
                out.append(dict(ext_plan("th-field-face", "field", "oak-reclaimed", 0.54, ring, BB, BB + field_thickness(scale)), region="field-face"))
                out.append(dict(ext_plan("th-field-bitumen", "field", "bitumen", 0.12, ring, 0.0, BB, sheet=True), region="field-bitumen"))
            else:
                out.append(ref(f"th-{sid}", sid, region="field-face"))
        elif sid.startswith("field-bitumen"):
            out.append(dict({k: v for k, v in s.items() if k != "id"}, id=f"th-{sid}", region="field-bitumen"))
    if scale == 500:
        y0 = BEARER + PAL_H
        out.append(dict(box("th-stack", "stack", "oak-reclaimed", 0.5, X0 + STACK_X, X0 + STACK_X + STACK_L, y0, y0 + STACK_H - 0.08, -STACK_W / 2, STACK_W / 2), region="stack-blocks"))
    elif scale == 50:
        y0 = BEARER + PAL_H
        for i in range(ROWS_X):
            a0 = X0 + STACK_X + i * STACK_L / ROWS_X
            out.append(dict(box(f"th-stack-column-{i}", "stack", "oak-reclaimed", 0.5, a0, a0 + STACK_L / ROWS_X, y0, y0 + STACK_H, -STACK_W / 2, STACK_W / 2), region="stack-blocks"))
    else:
        y0 = BEARER + PAL_H
        out.append(dict(box("th-stack", "stack", "oak-reclaimed", 0.5, X0 + STACK_X, X0 + STACK_X + STACK_L, y0, y0 + STACK_H, -STACK_W / 2, STACK_W / 2), region="stack-blocks"))
    return out


def structure_list(scale, body):
    """🏗️ The structural idealisation: bearers and fork arms are their own members, the pallet a beam of deck and feet, the crate four wall plates, stack and field members along their bones."""
    out = []
    for s in body:
        if s["id"].startswith("bearer-"):
            out.append(ref(f"st-{s['id']}", s["id"], member="bearers", part="member"))
        if s["id"].startswith("fork-"):
            out.append(ref(f"st-{s['id']}", s["id"], member="forks", part="member"))
    y = BEARER
    if scale != 500:
        out.append(box("st-pallet-deck", "pallet", "pallet-timber", 0.5, X0, X0 + PAL_L, y + 0.1, y + PAL_H, -PAL_W / 2, PAL_W / 2, member="pallet", part="member"))
    for i, (u0, u1) in enumerate(BLOCK_U if scale != 500 else []):
        for k, (z0, z1) in enumerate(FOOT_Z):
            out.append(box(f"st-pallet-foot-{i}{k}", "pallet", "pallet-timber", 0.45, X0 + u0, X0 + u1, y, y + 0.1, z0, z1, member="pallet", part="member"))
    y0 = BEARER + PAL_H
    wt = {50: 0.019, 100: 0.019, 200: 0.030, 500: 0.075}[scale]
    if scale == 500:
        out.append(box("st-pallet-block", "pallet", "pallet-timber", 0.5, X0, X0 + PAL_L, BEARER, BEARER + PAL_H, -PAL_W / 2, PAL_W / 2, member="pallet", part="member"))
        out.append(box("st-crate-shell", "crate", "softwood-crate", 0.55, X0, X0 + PAL_L, BEARER + PAL_H, y0 + CRATE_H, -PAL_W / 2, PAL_W / 2, member="crate", part="member"))
    else:
        out.append(box("st-crate-wall-right", "crate", "softwood-crate", 0.55, X0, X0 + PAL_L, y0, y0 + CRATE_H, -PAL_W / 2, -PAL_W / 2 + wt, member="crate", part="member"))
        out.append(box("st-crate-wall-left", "crate", "softwood-crate", 0.55, X0, X0 + PAL_L, y0, y0 + CRATE_H, PAL_W / 2 - wt, PAL_W / 2, member="crate", part="member"))
        out.append(box("st-crate-wall-front", "crate", "softwood-crate", 0.55, X0 + PAL_L - wt, X0 + PAL_L, y0, y0 + CRATE_H, -PAL_W / 2 + wt, PAL_W / 2 - wt, member="crate", part="member"))
        out.append(box("st-crate-wall-back", "crate", "softwood-crate", 0.55, X0, X0 + wt, y0, y0 + CRATE_H, -PAL_W / 2 + wt, PAL_W / 2 - wt, member="crate", part="member"))
    if scale == 500:
        out.append(box("st-stack", "stack", "oak-reclaimed", 0.5, X0 + STACK_X, X0 + STACK_X + STACK_L, y0, y0 + STACK_H - 0.08, -STACK_W / 2, STACK_W / 2, member="stack", part="member"))
    else:
        out.append(box("st-stack", "stack", "oak-reclaimed", 0.5, X0 + STACK_X, X0 + STACK_X + STACK_L, y0, y0 + STACK_H, -STACK_W / 2, STACK_W / 2, member="stack", part="member"))
    ring = FIELD_RING if scale in (50, 100) else simplify_ring(FIELD_RING, 0.004 if scale == 200 else 0.01)
    out.append(ext_plan("st-field", "field", "oak-reclaimed", 0.54, ring, 0.0, field_thickness(scale), member="field", part="member"))
    mk = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[scale]
    sx, sy = STACK_L / 2, STACK_H / 2
    out.append({"id": "st-stack-mass", "kind": "ellipsoid", "bone": "stack", "material": "oak-reclaimed", "role": "body", "tone": 0.3, "radii": [mk * 2, mk * 2, mk * 2], "transform": {"translation": [r6(sx), r6(sy), 0]}, "member": "stack", "part": "mass"})
    return out


def volume_decisions(scale):
    """📝 What each 3D scale models, merges and leaves out."""
    if scale == 50:
        return [
            t("Body: every part as it is built. The stack is 720 oak blocks of 280 x 70 x 22 mm (DIN 280-1 Stabparkett), 4 along x with 3 mm end gaps and 10 across with 2 mm row gaps, in 18 layers; each block lies on its own 1.5 mm bitumen remnant sheet (declared sheet), as the old hot-bitumen bed came off with it; each of the 40 positions is one box repeated over the layers. The 27 relaid blocks of the herringbone sample are one extrusion each at plus or minus 45 degrees, three with an 18 x 12 mm chipped corner, each on its own bitumen sheet.", "Körper: jedes Teil, wie es gebaut ist. Der Stapel sind 720 Eichenstäbe 280 x 70 x 22 mm (DIN 280-1 Stabparkett), 4 längs mit 3 mm Stoßfuge und 10 quer mit 2 mm Reihenfuge, in 18 Lagen; jeder Stab liegt auf seinem eigenen Bitumenrest von 1,5 mm (erklärte Schicht), wie das alte Heißbitumenbett mit abging; jede der 40 Positionen ist ein Quader, über die Lagen wiederholt. Die 27 neu gelegten Stäbe der Fischgrätprobe sind je eine Extrusion unter plus oder minus 45 Grad, drei mit einer abgeplatzten Ecke von 18 x 12 mm, jeder auf seiner eigenen Bitumenschicht."),
            t("Body: the EPAL pallet board by board (three bottom boards, nine blocks, three stringer boards, five top deck boards with 41 mm gaps), the crate of 16 slats 100 x 19 mm and ten battens 70 x 22 mm outside, the two bearers 100 x 100 mm and the two fork arms (ISO 2328 class 2 profile with heel radius and upper hook) lowered on the ground in front.", "Körper: die EPAL-Palette Brett für Brett (drei Unterbretter, neun Klötze, drei Querbretter, fünf Deckbretter mit 41-mm-Lücken), die Kiste aus 16 Latten 100 x 19 mm und zehn außen liegenden Leisten 70 x 22 mm, die beiden Kanthölzer 100 x 100 mm und die beiden Gabelzinken (Profil ISO 2328 Klasse 2 mit Rückenradius und oberem Haken), vor der Palette auf den Boden abgesenkt."),
            t("Thermal: references to the body for bearers, pallet, crate, forks and field; the stack is its own envelope box, because the joints of the stacked blocks are closed seams and only its outer faces exchange heat; the bitumen sheets count only their faces towards the stack or the screed (the face glued to the oak is a section face).", "Thermisch: Verweise auf den Körper für Kanthölzer, Palette, Kiste, Gabel und Probefläche; der Stapel ist ein eigener Hüllquader, weil die Fugen der gestapelten Stäbe geschlossene Nähte sind und nur seine Außenflächen Wärme tauschen; die Bitumenschichten zählen nur ihre Fläche zum Stapel oder zum Estrich (die an die Eiche geklebte Fläche ist eine Schnittfläche)."),
            t("Structure: the bearers and fork arms are their own members, the pallet a beam of deck and nine feet, the crate four wall plates, the stack one member with a mass marker at its centre, the field one plate.", "Tragwerk: Kanthölzer und Gabelzinken sind ihre eigenen Glieder, die Palette ein Balken aus Deck und neun Füßen, die Kiste vier Wandscheiben, der Stapel ein Glied mit einer Massenmarke im Schwerpunkt, die Probefläche eine Platte."),
        ]
    if scale == 100:
        return [
            t("Body: the stack becomes one oak box of 1129 x 718 x 423 mm, because its 2 and 3 mm joints and the 1.5 mm bitumen are below the 15 mm feature; the field becomes one extrusion of its joined zigzag outline (23.5 mm) on one bitumen sheet; pallet, crate, bearers keep every board, which are 19 mm and more; the fork profile loses its heel radius.", "Körper: der Stapel wird ein Eichenquader 1129 x 718 x 423 mm, weil seine Fugen von 2 und 3 mm und das Bitumen von 1,5 mm unter der kleinsten Einzelheit von 15 mm liegen; die Probefläche wird eine Extrusion ihrer vereinigten Zickzackkontur (23,5 mm) auf einer Bitumenschicht; Palette, Kiste und Kanthölzer behalten jedes Brett, das 19 mm und mehr misst; das Gabelprofil verliert seinen Rückenradius."),
            t("Thermal and structure as at 1:50 on the solids of this scale.", "Thermik und Tragwerk wie bei 1:50 auf den Körpern dieses Maßstabs."),
        ]
    if scale == 200:
        return [
            t("Body: the pallet becomes nine feet (block and bottom board, 100 mm) under one 44 mm deck, because the 22 mm boards are below the 30 mm feature; each crate wall becomes one 30 mm plate (the 19 mm slats at the print minimum) with 30 mm battens; the field one 30 mm slab of its outline; the fork arm a plain L of 40 mm.", "Körper: die Palette wird zu neun Füßen (Klotz und Unterbrett, 100 mm) unter einem 44-mm-Deck, weil die 22-mm-Bretter unter der kleinsten Einzelheit von 30 mm liegen; jede Kistenwand wird eine 30-mm-Platte (die 19-mm-Latten im Druckminimum) mit 30-mm-Leisten; die Probefläche eine 30-mm-Platte ihrer Kontur; der Gabelzinken ein einfaches L von 40 mm."),
            t("Thermal: the field face and its 1.5 mm bitumen (declared sheet) stay two regions; structure as at 1:100 with 30 mm walls.", "Thermisch: Oberfläche der Probefläche und ihr Bitumen von 1,5 mm (erklärte Schicht) bleiben zwei Regionen; Tragwerk wie bei 1:100 mit 30-mm-Wänden."),
        ]
    return [
        t("Body: two bearers, nine pallet feet and one mass of crate, stack and pallet deck from 200 to 744 mm; the field a 75 mm slab inside it at the top of the stack, which leaves the mass in the reinstalled pose; the fork arms an L of 75 mm, the print minimum.", "Körper: zwei Kanthölzer, neun Palettenfüße und eine Masse aus Kiste, Stapel und Palettendeck von 200 bis 744 mm; die Probefläche eine 75-mm-Platte darin oben auf dem Stapel, die in der wiederverbauten Pose die Masse verlässt; die Gabelzinken ein L von 75 mm, das Druckminimum."),
        t("Thermal: the stack keeps its own box inside the mass so every region has a solid; structure: the crate shell carries the stack member and its mass marker.", "Thermisch: der Stapel behält seinen eigenen Quader in der Masse, damit jede Region einen Körper hat; Tragwerk: die Kistenhülle trägt das Stapelglied und seine Massenmarke."),
    ]


def scale_docs():
    """🗂️ Representations and volumes of the four preset scales."""
    reps, vols = {}, {}
    for scale in (50, 100, 200, 500):
        P, body, shapes = build_scale(scale)
        stack_2d(scale, shapes)
        field_2d(scale, shapes)
        fork_2d(scale, shapes)
        detail_2d(scale, shapes)
        if scale == 500:
            crate500_2d(shapes)
        lo, hi = RANGES[scale]
        reps[str(scale)] = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": shapes, "decisions": DECISIONS[scale]}
        vols[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(scale, body), "structure": structure_list(scale, body), "decisions": volume_decisions(scale)}
    return reps, vols


def region(id_, title, material, area, length, geometry, sky, cap, ref_, kind="surface", **extra):
    """🔥 A passive surface region."""
    d = {"id": id_, "title": title, "kind": kind, "model": "passive", "material": material, "areaM2": area, "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": ref_}
    d.update(extra)
    return d


AREAS = json.load(open(os.path.join(HERE, "author-areas.json"))) if os.path.exists(os.path.join(HERE, "author-areas.json")) else {}


def area(key, estimate):
    """📐 The region area: the value measured on the honest 1:50 model when recorded in author-areas.json, else the hand estimate."""
    return round(AREAS.get(key, estimate), 3)


REGIONS = [
    region("bearer-timber", t("Squared-timber bearers", "Kanthölzer"), "softwood-weathered", area("bearer-timber", 2 * (4 * 0.1 * 1.2 + 2 * 0.01)), 0.1, "horizontal-plate-up", 0.5, 18000, "en-iso-13786"),
    region("pallet-timber", t("Euro pallet", "Europalette"), "pallet-timber", area("pallet-timber", 3.9), 0.145, "horizontal-plate-up", 0.3, 9000, "en-iso-13786"),
    region("crate-timber", t("Crate slats and battens", "Kistenlatten und -leisten"), "softwood-crate", area("crate-timber", 6.0), 0.1, "vertical-plate", 0.5, 8000, "en-iso-13786"),
    region("stack-blocks", t("Stacked parquet blocks", "Gestapelte Parkettstäbe"), "oak-reclaimed", area("stack-blocks", 3.24), 0.423, "vertical-plate", 0.4, 30000, "en-iso-13786"),
    region("field-face", t("Wear face of the relaid herringbone sample", "Nutzschicht der neu gelegten Fischgrätprobe"), "oak-reclaimed", area("field-face", FIELD_AREA * 1.08), 0.07, "horizontal-plate-up", 0.9, 30000, "en-iso-13786"),
    region("field-bitumen", t("Bitumen remnants under the field", "Bitumenreste unter der Probefläche"), "bitumen", area("field-bitumen", 2 * FIELD_AREA), 0.07, "horizontal-plate-down", 0.0, 1600, "en-iso-10456", hidden=t("The bitumen remnant faces down: it lies between the field and the top layer of the stack when stored and between the field and the screed when relaid, so no view sees it; drawn as a band in elevation and front.", "Der Bitumenrest zeigt nach unten: gelagert liegt er zwischen Probefläche und oberster Stapellage, neu verlegt zwischen Probefläche und Estrich, keine Ansicht sieht ihn; in Seiten- und Vorderansicht als Band gezeichnet.")),
    region("fork-steel", t("Fork arms", "Gabelzinken"), "steel-fork", area("fork-steel", 2 * (2 * 0.04 * 1.15 + 2 * 0.1 * 1.15 + 2 * 0.1 * 0.462 + 2 * 0.04 * 0.46)), 0.1, "horizontal-plate-up", 0.7, 25000, "en-iso-10456", ratedPowerW=0, ratedPowerBasis="passive"),
]

ALL_REGIONS = [r["id"] for r in REGIONS]


def binding(sun, extra=None, contact=None):
    """🌬️ A thermal binding of every region with its exposed and sunlit share."""
    regions = {}
    for rid in ALL_REGIONS:
        e, s = sun.get(rid, (1.0, 0.3))
        regions[rid] = {"exposedFraction": e, "sunlitFraction": s}
        if contact and rid in contact:
            regions[rid]["contact"] = contact[rid]
    return {"regions": regions, "emitters": {}}


STORED_SUN = {"bearer-timber": (0.8, 0.2), "pallet-timber": (0.85, 0.15), "crate-timber": (1.0, 0.45), "stack-blocks": (0.55, 0.15), "field-face": (1.0, 0.9), "field-bitumen": (0.05, 0.0), "fork-steel": (0.9, 0.5)}
LIFTED_SUN = dict(STORED_SUN, **{"bearer-timber": (0.8, 0.5)})
INSTALLED_SUN = dict(STORED_SUN, **{"field-face": (1.0, 0.6), "field-bitumen": (0.02, 0.0)})
STORED_CONTACT = {"bearer-timber": {"fraction": 0.2, "tempC": 12}, "field-bitumen": {"fraction": 0.95, "tempC": 15}}
INSTALLED_CONTACT = {"bearer-timber": {"fraction": 0.2, "tempC": 12}, "field-bitumen": {"fraction": 0.98, "tempC": 18}}

BEARER_CONTACTS = ["bearer-left-back", "bearer-left-front", "bearer-right-back", "bearer-right-front"]
FORK_HOOKS = ["fork-hook-left", "fork-hook-right"]
FIELD_CONTACTS = ["field-back-left", "field-back-right", "field-front-left", "field-front-right"]
FIELD_LOAD_W = 2000.0 * (FIELD_ZMAX - FIELD_ZMIN)


def anchors():
    """📍 Contacts of the bearers, the carriage hooks of the forks, the bedding points of the relaid field and the point of the walking load."""
    out = []
    for name, z in (("left", 0.35), ("right", -0.35)):
        for end, u in (("back", 0.05), ("front", 1.15)):
            out.append({"id": f"bearer-{name}-{end}", "bone": "bearers", "x": u, "y": 0, "z": z, "kind": "contact", "title": t(f"Bearer {name}, {end} end on the ground", f"Kantholz {'links' if name == 'left' else 'rechts'}, {'hinteres' if end == 'back' else 'vorderes'} Ende auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.1, "footprintWidthM": 0.1, "frictionCoefficient": 0.5})
    for name, z in (("left", TINE_Z), ("right", -TINE_Z)):
        out.append({"id": f"fork-hook-{name}", "bone": "forks", "x": 1.13, "y": 0.44, "z": z, "kind": "support", "title": t(f"Carriage hook of the {name} fork arm", f"Trägerhaken des {'linken' if name == 'left' else 'rechten'} Gabelzinkens"), "support": "fixed"})
    zl, zr = FIELD_ZMAX - 0.12, FIELD_ZMIN + 0.12
    for end, u in (("back", 0.25), ("front", FIELD_L - 0.25)):
        for name, z in (("left", zl), ("right", zr)):
            out.append({"id": f"field-{end}-{name}", "bone": "field", "x": r6(u), "y": 0, "z": r6(z * 0.5), "kind": "contact", "title": t(f"Field bedded on the screed, {end} {name}", f"Probefläche auf dem Estrich, {'hinten' if end == 'back' else 'vorn'} {'links' if name == 'left' else 'rechts'}"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.3, "footprintWidthM": 0.3, "frictionCoefficient": 0.6})
    out.append({"id": "load-point", "bone": "load-carrier", "x": 0, "y": 0, "kind": "load", "title": t("Concentrated floor load", "Einzellast auf dem Boden")})
    return out


def pose(id_, title, desc, bones, contacts, sun, contact, loads, key=False):
    """🧍 A pose with every region bound."""
    d = {"id": id_, "title": title, "description": desc, "bones": bones, "ground": "authored", "contacts": contacts, "thermal": binding(sun, contact=contact), "loads": loads}
    if key:
        d["key"] = True
    return d


IMPOSED = {"id": "imposed-floor", "kind": "imposed", "title": t("Imposed floor load 2 kN/m²", "Nutzlast 2 kN/m²"), "fxN": 0, "fyN": 0, "distributed": {"bone": "field", "fromM": 0, "toM": r6(FIELD_L), "wxNPerM": 0, "wyNPerM": -round(FIELD_LOAD_W, 1)}, "source": "literature", "reference": "en-1991-1-1"}
POINT = {"id": "point-floor", "kind": "live", "title": t("Concentrated load 2 kN", "Einzellast 2 kN"), "anchor": "load-point", "fxN": 0, "fyN": -2000, "source": "literature", "reference": "en-1991-1-1"}
FIELD_OFFSET = {"field": {"dx": FIELD_DX, "dy": FIELD_DY}}


def poses():
    """🎭 Stored, lifted, reinstalled under the floor load and under a point load, and the key poses of the lift."""
    lifted_bones = {"fork-reach": {"dx": FORK_REACH}, "forks": {"dy": FORK_TOUCH + LIFT}, "pallet": {"dy": LIFT}}
    return [
        pose("stored", t("Stored in the yard", "Im Lager gelagert"), t("The crate on its Euro pallet stands on two squared-timber bearers; the relaid herringbone sample lies on the top layer, the fork arms of the forklift are lowered on the ground in front of the pallet.", "Die Kiste auf ihrer Europalette steht auf zwei Kanthölzern; die neu gelegte Fischgrätprobe liegt auf der obersten Lage, die Gabelzinken des Staplers sind vor der Palette auf den Boden abgesenkt."), {}, BEARER_CONTACTS, STORED_SUN, STORED_CONTACT, []),
        pose("forks-in", t("Forks entered", "Gabel eingefahren"), t("The fork arms have entered the pallet openings on the ground.", "Die Gabelzinken sind am Boden in die Palettenöffnungen eingefahren."), {"fork-reach": {"dx": FORK_REACH}}, BEARER_CONTACTS, STORED_SUN, STORED_CONTACT, [], key=True),
        pose("forks-contact", t("Forks under the stringers", "Gabel unter den Querbrettern"), t("The fork arms touch the undersides of the three stringer boards.", "Die Gabelzinken berühren die Unterseiten der drei Querbretter."), {"fork-reach": {"dx": FORK_REACH}, "forks": {"dy": FORK_TOUCH}}, BEARER_CONTACTS, STORED_SUN, STORED_CONTACT, [], key=True),
        pose("lifted", t("Lifted on the forks", "Auf der Gabel angehoben"), t("Pallet, crate, stack and sample lifted 300 mm on the fork arms, held at the two carriage hooks; the bearers stay on the ground.", "Palette, Kiste, Stapel und Probe 300 mm auf den Gabelzinken angehoben, an den beiden Trägerhaken gehalten; die Kanthölzer bleiben am Boden."), lifted_bones, FORK_HOOKS, LIFTED_SUN, {"field-bitumen": STORED_CONTACT["field-bitumen"]}, []),
        pose("reinstalled", t("Relaid on screed", "Auf Estrich neu verlegt"), t("The 27-block herringbone sample relaid in hot bitumen on a cement screed beside the crate (the ground line is the screed surface) under the imposed floor load of 2 kN/m².", "Die Fischgrätprobe aus 27 Stäben neben der Kiste in Heißbitumen auf Zementestrich neu verlegt (die Bodenlinie ist die Estrichoberfläche), unter der Nutzlast von 2 kN/m²."), FIELD_OFFSET, BEARER_CONTACTS + FIELD_CONTACTS, INSTALLED_SUN, INSTALLED_CONTACT, [IMPOSED]),
        pose("reinstalled-point-load", t("Relaid, point load", "Neu verlegt, Einzellast"), t("The relaid sample under a concentrated load of 2 kN in the middle of the field.", "Die neu verlegte Probe unter einer Einzellast von 2 kN in Feldmitte."), dict(FIELD_OFFSET, **{"load-carrier": {"dx": r6(FIELD_L / 2 - 0.15)}}), BEARER_CONTACTS + FIELD_CONTACTS, INSTALLED_SUN, INSTALLED_CONTACT, [POINT]),
    ]


def clips():
    """🎞️ The lift on the forks and a walking load across the relaid field."""
    lift = {"id": "lift", "title": t("Lift on the forks", "Anheben mit der Gabel"), "description": t("The fork arms slide along the ground into the pallet openings, rise to the stringer boards and lift pallet, crate and sample 300 mm; the bearers stay behind.", "Die Gabelzinken fahren am Boden in die Palettenöffnungen, heben sich bis an die Querbretter und heben Palette, Kiste und Probe um 300 mm an; die Kanthölzer bleiben zurück."), "durationS": 8, "fps": 25, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 3, "pose": "forks-in", "ease": "smooth"}, {"t": 4, "pose": "forks-contact", "ease": "smooth"}, {"t": 7, "pose": "lifted", "ease": "smooth"}, {"t": 8, "pose": "lifted", "ease": "smooth"}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 4} for a in BEARER_CONTACTS] + [{"anchor": a, "fromS": 4, "toS": 8} for a in FORK_HOOKS],
            "phases": [{"id": "enter", "fromS": 0, "toS": 3, "title": t("Enter", "Einfahren")}, {"id": "engage", "fromS": 3, "toS": 4, "title": t("Engage", "Unterfassen")}, {"id": "lift", "fromS": 4, "toS": 7, "title": t("Lift", "Heben")}, {"id": "hold", "fromS": 7, "toS": 8, "title": t("Hold", "Halten")}],
            "events": [{"t": 3, "kind": "motion", "title": t("Forks fully entered", "Gabel ganz eingefahren")}, {"t": 4, "kind": "contact-on", "title": t("Forks take the load", "Gabel übernimmt die Last")}, {"t": 7, "kind": "motion", "title": t("Lifted 300 mm", "Um 300 mm angehoben")}], "tracks": []}
    walk = {"id": "walking-load", "title": t("Walking load on the relaid field", "Wandernde Last auf der neu verlegten Fläche"), "description": t("A concentrated load of 2 kN crosses the relaid sample along its spine.", "Eine Einzellast von 2 kN überquert die neu verlegte Probe entlang ihrer Mittelachse."), "durationS": 5, "fps": 25, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "reinstalled-point-load", "ease": "linear", "bones": {"load-carrier": {"dx": 0}}}, {"t": 5, "pose": "reinstalled-point-load", "ease": "linear", "bones": {"load-carrier": {"dx": r6(FIELD_L - 0.3)}}}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 5} for a in BEARER_CONTACTS + FIELD_CONTACTS],
            "phases": [{"id": "cross", "fromS": 0, "toS": 5, "title": t("Crossing", "Überqueren")}],
            "events": [{"t": 0, "kind": "motion", "title": t("Load steps on at the back end", "Last tritt am hinteren Ende auf")}], "tracks": []}
    return [lift, walk]


def rig():
    """🦴 The rig: a massless yard datum carries the bearers, the pallet on its lift slide with crate, stack and field, and the two slides of the fork."""
    def bone(id_, parent, title, length, joint, mass=None, section=None):
        ox, oy = ORIGIN[id_]
        px, py = ORIGIN[parent] if parent else (0.0, 0.0)
        b = {"id": id_, "parent": parent, "title": title, "x": r6(ox - px), "y": r6(oy - py), "angleDeg": 0, "lengthM": r6(length), "joint": joint}
        if mass:
            b["mass"] = mass
        if section:
            b["section"] = section
        return b

    def m(kg, cx, cy, ref_, src="literature"):
        return {"kg": round(kg, 3), "comX": r6(cx), "comY": r6(cy), "distribution": "point", "source": src, "reference": ref_}

    return {"kind": "mechanical", "root": "yard", "bones": [
        bone("yard", None, t("Yard datum (massless, on the ground)", "Lagerplatz-Bezug (masselos, am Boden)"), 0, {"type": "fixed"}),
        bone("bearers", "yard", t("Two squared-timber bearers", "Zwei Kanthölzer"), PAL_L, {"type": "fixed"}, m(MASS["bearers"], PAL_L / 2, BEARER / 2, "en-338"), {"areaM2": 0.02, "secondMomentM4": r6(2 * 0.1 ** 4 / 12), "extremeFibreM": 0.05, "material": "softwood-weathered", "title": t("2 x 100 x 100 mm", "2 x 100 x 100 mm")}),
        bone("pallet", "yard", t("Euro pallet (lifted by the forks)", "Europalette (von der Gabel gehoben)"), PAL_L, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.6}, m(MASS["pallet"], PAL_L / 2, 0.085, "en-13698-1"), {"areaM2": 0.02156, "secondMomentM4": 7.32e-05, "extremeFibreM": 0.09, "material": "pallet-timber", "title": t("Top deck and bottom boards", "Deck- und Unterbretter")}),
        bone("crate", "pallet", t("Slatted crate", "Lattenkiste"), PAL_L, {"type": "fixed"}, m(MASS["crate"], PAL_L / 2, CRATE_H / 2, "en-338", "authored-estimate")),
        bone("stack", "crate", t("Stack of parquet blocks", "Stapel der Parkettstäbe"), STACK_L, {"type": "fixed"}, m(MASS["stack"], STACK_L / 2, STACK_H / 2, "din-280-1")),
        bone("field", "stack", t("Herringbone sample (taken out and relaid)", "Fischgrätprobe (herausgenommen und neu verlegt)"), FIELD_L, {"type": "prismatic", "axisDeg": round(FIELD_AXIS, 4), "minM": 0, "maxM": r6(FIELD_TRAVEL + 1e-3)}, m(MASS["field"], FIELD_L / 2, PITCH / 2, "din-280-1")),
        bone("load-carrier", "field", t("Load carrier (massless, moves the point load)", "Lastträger (masselos, bewegt die Einzellast)"), 0, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": r6(FIELD_L - 0.3)}),
        bone("fork-reach", "yard", t("Fork reach (massless slide along the ground)", "Gabelvorschub (masseloser Schlitten am Boden)"), 0, {"type": "prismatic", "axisDeg": 0, "minM": -1.3, "maxM": 0}),
        bone("forks", "fork-reach", t("Fork arms (carried by the forklift, massless here)", "Gabelzinken (vom Stapler getragen, hier masselos)"), TINE_L, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.7}, None, {"areaM2": 0.008, "secondMomentM4": 1.0667e-06, "extremeFibreM": 0.02, "material": "steel-fork", "title": t("2 blades 100 x 40 mm", "2 Zinken 100 x 40 mm")}),
    ]}


MATERIALS = [
    {"id": "oak-reclaimed", "title": t("Reclaimed oak parquet (aged wear face)", "Gebrauchtes Eichenparkett (gealterte Nutzschicht)"), "emissivity": 0.9, "solarAbsorptivity": 0.72, "densityKgM3": OAK_RHO, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 30000000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "bitumen", "title": t("Hot-bitumen parquet adhesive (remnant)", "Heißbitumen-Parkettklebstoff (Rest)"), "emissivity": 0.93, "solarAbsorptivity": 0.92, "densityKgM3": BIT_RHO, "conductivityWmK": 0.17, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "softwood-weathered", "title": t("Weathered spruce, reused bearers", "Vergraute Fichte, wiederverwendete Kanthölzer"), "emissivity": 0.9, "solarAbsorptivity": 0.68, "densityKgM3": SOFT_RHO, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
    {"id": "softwood-crate", "title": t("Rough-sawn spruce, crate", "Sägeraue Fichte, Kiste"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": SOFT_RHO, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
    {"id": "pallet-timber", "title": t("Pallet timber, boards and chip blocks", "Palettenholz, Bretter und Spanklötze"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": round(PALLET_RHO, 1), "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 9000000000, "source": "authored-estimate", "reference": "en-13698-1"},
    {"id": "steel-fork", "title": t("Fork steel, black paint worn bare on the blades", "Gabelstahl, schwarzer Lack, auf den Zinken blank gefahren"), "emissivity": 0.85, "solarAbsorptivity": 0.8, "densityKgM3": STEEL_RHO, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000, "strengthPa": 355000000, "source": "literature", "reference": "iso-2328"},
]

REFERENCES = [
    {"id": "din-280-1", "citation": "DIN 280-1:1990 Parkett; Parkettstäbe und Tafeln für Tafelparkett (Stabparkett: thickness 22 mm, widths 45 to 80 mm, lengths 250 to 600 mm; groove on all sides or tongue and groove); superseded by DIN EN 13226:2009 (solid wood parquet elements with grooves and/or tongues, thickness 14 to 22 mm)."},
    {"id": "en-13226", "citation": "EN 13226:2009 Wood flooring, solid parquet elements with grooves and/or tongues (dimensional tolerances; the 22 x 70 x 280 mm herringbone block is a stock size)."},
    {"id": "en-13698-1", "citation": "EN 13698-1:2003 Pallet production specification, part 1: 800 mm x 1200 mm flat wooden pallet (EPAL 1: 1200 x 800 x 144 mm, 5 top deck boards 22 mm of 145/100 mm, 3 stringer boards 145 x 22 mm, 9 blocks 78 mm, 3 bottom boards; nominal mass about 25 kg; fork entry height 100 mm)."},
    {"id": "iso-2328", "citation": "ISO 2328:2011 Fork-lift trucks, hook-on type fork arms and fork arm carriages, mounting dimensions (class 2 carriage; fork arm section 100 x 40 mm, 1150 mm blade)."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes (C24: E0,mean 11 GPa, mean density 420 kg/m³; 450 kg/m³ taken for air-dry reused spruce)."},
    {"id": "en-350", "citation": "EN 350:2016 Durability of wood and wood-based products (European oak Quercus robur/petraea, density at 12 % moisture 670 to 760 kg/m³; 700 kg/m³ used)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1, Part 1-1, Tables 6.1 and 6.2 (category A floors: qk 2.0 kN/m², Qk 2.0 kN)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (hardwood 0.18 W/mK, 1600 J/kgK; softwood 0.13 W/mK; bitumen 0.17 W/mK, 1000 J/kgK; steel 50 W/mK, 450 J/kgK)."},
    {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics (effective storage depth of a timber face about half the 24 h penetration depth)."},
    {"id": "ashrae-fundamentals", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 33, radiation properties: weathered wood 0.90 emissivity, asphalt 0.93, painted steel 0.85 to 0.9; solar absorptance of grey weathered wood 0.6 to 0.75 (rounded guidance, not table look-ups)."},
    {"id": "tkb-pak", "citation": "Merkblatt TKB-1 (Industrieverband Klebstoffe) and the German PAK guideline for parquet adhesives: hot-bitumen and tar-based parquet adhesives were used up to the 1970s; tar-based remnants (PAH) must be tested before reuse, bitumen remnants are the inert case modelled here."},
]


def main():
    """🖨️ Write the asset document."""
    reps, vols = scale_docs()
    asset = {
        "id": "reclaimed-parquet-crate",
        "category": "components",
        "title": t("Reclaimed Parquet Crate", "Gebrauchtes Parkett in der Kiste"),
        "summary": t(f"About 15 m² of reclaimed oak herringbone blocks 22 x 70 x 280 mm with the black remnants of their hot-bitumen bed on the underside, stacked in 18 layers of 4 x 10 blocks in a slatted spruce crate nailed onto a Euro pallet on two squared-timber bearers, with a 27-block herringbone sample laid out on top; lifted on forklift forks, the sample relaid on screed under the imposed floor load and a walking point load.", f"Rund 15 m² gebrauchte Eichen-Fischgrätstäbe 22 x 70 x 280 mm mit den schwarzen Resten ihres Heißbitumenbetts auf der Unterseite, in 18 Lagen zu 4 x 10 Stäben in einer Lattenkiste aus Fichte gestapelt, die auf eine Europalette auf zwei Kanthölzern genagelt ist, obenauf eine Fischgrätprobe aus 27 Stäben; mit der Staplergabel angehoben, die Probe auf Estrich unter Nutzlast und wandernder Einzellast neu verlegt."),
        "tags": ["component", "reclaimed", "flooring", "parquet", "herringbone", "oak", "timber", "bitumen", "pallet", "crate", "stacked-goods"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t(f"Block 22 x 70 x 280 mm, 0.302 kg of oak plus 0.031 kg of bitumen remnant; stack 720 blocks ({MASS['stack']:.1f} kg), sample 27 blocks ({MASS['field']:.2f} kg); pallet 25.0 kg, crate {MASS['crate']:.2f} kg, bearers {MASS['bearers']:.1f} kg; total {TOTAL_MASS:.1f} kg without the fork arms, which belong to the forklift.", f"Stab 22 x 70 x 280 mm, 0,302 kg Eiche und 0,031 kg Bitumenrest; Stapel 720 Stäbe ({format(MASS['stack'], '.1f').replace('.', ',')} kg), Probe 27 Stäbe ({format(MASS['field'], '.2f').replace('.', ',')} kg); Palette 25,0 kg, Kiste {format(MASS['crate'], '.2f').replace('.', ',')} kg, Kanthölzer {format(MASS['bearers'], '.1f').replace('.', ',')} kg; gesamt {format(TOTAL_MASS, '.1f').replace('.', ',')} kg ohne die Gabelzinken, die zum Stapler gehören.")},
        "dimensions": {"heightM": r6(BEARER + PAL_H + CRATE_H), "widthM": r6(FX + TINE_L), "depthM": r6(PAL_W + 2 * BAT_T), "massKg": round(TOTAL_MASS, 2)},
        "materials": MATERIALS,
        "rig": rig(),
        "volumes": vols,
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": {
            "environment": {"airTempC": 15, "meanRadiantTempC": 14, "skyTempC": 4, "groundTempC": 12, "relativeHumidity": 0.7, "windMS": 1.0, "windDirectionDeg": 0, "solarWM2": 350, "diffuseFraction": 0.5, "pressurePa": 101325},
            "operating": [],
            "regions": REGIONS,
            "emitters": [],
            "passengers": [],
            "assumptions": [
                t("Passive timber faces store heat in an effective depth of about a third of the 24 h penetration depth (EN ISO 13786): oak 30 kJ/m²K, spruce 8 to 18 kJ/m²K; the bitumen remnant stores almost nothing.", "Passive Holzflächen speichern Wärme in einer wirksamen Tiefe von etwa einem Drittel der 24-h-Eindringtiefe (EN ISO 13786): Eiche 30 kJ/m²K, Fichte 8 bis 18 kJ/m²K; der Bitumenrest speichert fast nichts."),
                t("Weathered spruce of crate, pallet and bearers is grey and absorbs 0.6 to 0.68 of the sun; the aged oak wear face with old varnish is darker (0.72); the bitumen remnant is black (0.92); the fork steel is black paint worn bare on the blades (0.8). Values are rounded guidance (ASHRAE Fundamentals), not measurements.", "Vergraute Fichte von Kiste, Palette und Kanthölzern ist grau und absorbiert 0,6 bis 0,68 der Sonne; die gealterte Eichen-Nutzschicht mit altem Lack ist dunkler (0,72); der Bitumenrest ist schwarz (0,92); der Gabelstahl ist schwarzer Lack, auf den Zinken blank gefahren (0,8). Die Werte sind gerundete Richtwerte (ASHRAE Fundamentals), keine Messungen."),
                t("Reference case: an overcast autumn day in an open yard (15 °C air, 350 W/m² sun with half of it diffuse); in the reinstalled poses the bitumen under the field lies on a screed at 18 °C.", "Referenzfall: ein bedeckter Herbsttag auf einem offenen Lagerplatz (Luft 15 °C, Sonne 350 W/m², zur Hälfte diffus); in den wiederverbauten Posen liegt das Bitumen unter der Probefläche auf einem Estrich von 18 °C."),
                t("Power: every element is passive (0 W); the fork arms are the only steel and draw nothing.", "Leistung: jedes Element ist passiv (0 W); die Gabelzinken sind der einzige Stahl und nehmen nichts auf."),
            ],
        },
        "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [
            t("Each pose names its own supports: the four bearer ends on the ground when stored, the two carriage hooks of the fork arms (fixed) when lifted, the bearers and four bedding points of the field on the screed when reinstalled.", "Jede Pose nennt ihre eigenen Auflager: die vier Kantholzenden am Boden im Lager, die beiden Trägerhaken der Gabelzinken (eingespannt) beim Heben, die Kanthölzer und vier Bettungspunkte der Probefläche auf dem Estrich nach dem Neuverlegen."),
            t("The solver treats the asset as one rigid body: when lifted the forks also carry the 10.8 kg of the bearers that really stay on the ground (3.6 % over the true fork load), and when reinstalled the reactions are shared between bearers and field by equal support stiffness although the two rest separately; the member forces of the field under its own load are the meaningful result there.", "Der Löser behandelt das Asset als einen starren Körper: beim Heben trägt die Gabel auch die 10,8 kg der Kanthölzer, die in Wirklichkeit am Boden bleiben (3,6 % über der wahren Gabellast), und nach dem Neuverlegen werden die Auflagerkräfte nach gleicher Auflagersteifigkeit auf Kanthölzer und Probefläche verteilt, obwohl beide getrennt aufliegen; aussagekräftig sind dort die Schnittgrößen der Probefläche unter ihrer eigenen Last."),
            t("Loose blocks carry no bending: the stack and the field transfer their weight by contact; the field on its bitumen bed is idealised as a plate on four bedding points (EN 1991-1-1 category A: 2 kN/m² and 2 kN).", "Lose Stäbe tragen keine Biegung: Stapel und Probefläche geben ihr Gewicht über Kontakt ab; die Probefläche auf ihrem Bitumenbett ist als Platte auf vier Bettungspunkten idealisiert (EN 1991-1-1 Kategorie A: 2 kN/m² und 2 kN)."),
            t("Masses from volume and density: oak 700 kg/m³ (EN 350), bitumen 1050 kg/m³, reused spruce 450 kg/m³ (EN 338), the pallet at its EPAL nominal 25 kg; the fork arms belong to the forklift and are massless here.", "Massen aus Volumen und Dichte: Eiche 700 kg/m³ (EN 350), Bitumen 1050 kg/m³, wiederverwendete Fichte 450 kg/m³ (EN 338), die Palette mit ihren EPAL-Nennwert 25 kg; die Gabelzinken gehören zum Stapler und sind hier masselos."),
        ], "clinical": False},
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    print(f"[DEBUG] blocks {len(FIELD_BLOCKS)} field L {FIELD_L:.3f} z {FIELD_ZMIN:.3f}..{FIELD_ZMAX:.3f} area {FIELD_AREA:.3f} mass {TOTAL_MASS:.2f} {MASS}")


if __name__ == "__main__":
    main()
