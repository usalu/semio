"""♟️ Authoritative author script of `used-chess-table`: states every hand-chosen number of the used granite park chess table (footing, column, top with inlaid board, two stools) in world rest metres and writes the asset document `🔣️.json` next to it.

World frame of the staffage: x ahead (from White's stool to Black's), y up (y = 0 the water-bound gravel surface), z left (north = -z, the side the side elevation looks at).

@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SLUG = "used-chess-table"
G = 9.80665


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """✂️ Rounds a number for the document."""
    return round(v + 0.0, 6)


FOOT_X, FOOT_Z, FOOT_Y0, FOOT_Y1 = 1.0, 0.45, -0.16, 0.04
BED = 0.01
COL_H, COL_B, COL_CH = 0.61, 0.15, 0.01
TOP_Y0, TOP_Y1, TOP_B, CHIP = 0.66, 0.74, 0.35, 0.045
BOARD, SQ, INLAY = 0.20, 0.05, 0.02
STOOL_X, STOOL_R, STOOL_H, STOOL_CH = 0.68, 0.18, 0.40, 0.015
RHO = {"granite-grey": 2650, "granite-dark": 2950, "granite-light": 2650, "concrete-weathered": 2400, "bedding-mortar": 2000}

BONES = {
    "footing": {"o": (-FOOT_X, FOOT_Y0), "a": 0},
    "column": {"o": (0.0, FOOT_Y1 + BED), "a": 90},
    "top": {"o": (-TOP_B, TOP_Y0), "a": 0},
    "stool-white": {"o": (-STOOL_X, FOOT_Y1 + BED), "a": 90},
    "stool-black": {"o": (STOOL_X, FOOT_Y1 + BED), "a": 90},
}


def loc(bone, x, y):
    """📐️ World rest (x, y) to bone-local (u, v)."""
    ox, oy = BONES[bone]["o"]
    if BONES[bone]["a"] == 0:
        return (r6(x - ox), r6(y - oy))
    return (r6(y - oy), r6(-(x - ox)))


def loc3(bone, x, y, z):
    """📍️ World rest point to bone-local (u, v, w)."""
    u, v = loc(bone, x, y)
    return [u, v, r6(z)]


def size3(bone, sx, sy, sz):
    """📦️ World extents to bone-local extents."""
    return [r6(sx), r6(sy), r6(sz)] if BONES[bone]["a"] == 0 else [r6(sy), r6(sx), r6(sz)]


CHIP_PLAN = [(-TOP_B, -TOP_B), (TOP_B - CHIP, -TOP_B), (TOP_B, -TOP_B + CHIP), (TOP_B, TOP_B), (-TOP_B, TOP_B)]
SQUARE_PLAN = [(-TOP_B, -TOP_B), (TOP_B, -TOP_B), (TOP_B, TOP_B), (-TOP_B, TOP_B)]
BOARD_HOLE = [(-BOARD, -BOARD), (BOARD, -BOARD), (BOARD, BOARD), (-BOARD, BOARD)]


def octagon(b, c):
    """🛑️ A square of half side `b` with chamfers `c`, as (x, z) points."""
    return [(-b + c, -b), (b - c, -b), (b, -b + c), (b, b - c), (b - c, b), (-b + c, b), (-b, b - c), (-b, -b + c)]


# ---------------------------------------------------------------- solids


def base(id_, bone, mat, tone, role="body", **kw):
    """🧱️ The common fields of a solid."""
    s = {"id": id_, "bone": bone, "material": mat, "role": role, "tone": tone}
    s.update(kw)
    return s


def box(id_, bone, mat, tone, c, s, role="body", **kw):
    """⬛️ A box from its world centre and world extents."""
    return dict(base(id_, bone, mat, tone, role, **kw), kind="box", size=size3(bone, *s), transform={"translation": loc3(bone, *c)})


def pext(id_, bone, mat, tone, pts, y0, y1, holes=None, role="body", **kw):
    """🟫️ A plan-profile prism from world (x, z) points between the heights y0 and y1."""
    ox, oy = BONES[bone]["o"]
    if BONES[bone]["a"] == 0:
        conv = lambda p: [r6(p[1]), r6(p[0] - ox)]
        s = dict(base(id_, bone, mat, tone, role, **kw), kind="extrude", plane="zx", profile=[conv(p) for p in pts], **{"from": r6(y0 - oy), "to": r6(y1 - oy)})
    else:
        conv = lambda p: [r6(-(p[0] - ox)), r6(p[1])]
        s = dict(base(id_, bone, mat, tone, role, **kw), kind="extrude", plane="yz", profile=[conv(p) for p in pts], **{"from": r6(y0 - oy), "to": r6(y1 - oy)})
    if holes:
        s["holes"] = [[conv(p) for p in h] for h in holes]
    return s


def vrev(id_, bone, mat, tone, prof, base_xyz, role="body", **kw):
    """🏺️ A solid of revolution about a vertical axis through the world point `base_xyz`; `prof` is [(height above base, radius)]."""
    axis = [0, 1, 0] if BONES[bone]["a"] == 0 else [1, 0, 0]
    return dict(base(id_, bone, mat, tone, role, **kw), kind="revolve", profile=[[r6(h), r6(r)] for h, r in prof], axis=axis, origin=loc3(bone, *base_xyz))


def ell(id_, bone, mat, tone, c, radii, role="body", **kw):
    """🥚️ An ellipsoid from its world centre and world semi-axes."""
    return dict(base(id_, bone, mat, tone, role, **kw), kind="ellipsoid", radii=size3(bone, *radii), transform={"translation": loc3(bone, *c)})


def stool_profile(h, r, ch):
    """🪑️ The half profile of a granite stool of height `h`, radius `r` and top chamfer `ch`."""
    return [(0, 0), (0, r), (h - ch, r), (h, r - ch), (h, 0)] if ch > 0 else [(0, 0), (0, r), (h, r), (h, 0)]


def tiles(bone_tone_dark=0.18, bone_tone_light=0.86):
    """♟️ The 64 inlaid squares as 16 rank rows of four squares each (a1 dark, a-file on White's left at +z)."""
    out = []
    for r in range(8):
        x = -BOARD + SQ * (r + 0.5)
        for colour, f0, tone, mat in (("dark", r % 2, bone_tone_dark, "granite-dark"), ("light", 1 - r % 2, bone_tone_light, "granite-light")):
            z = BOARD - SQ * (f0 + 0.5)
            s = box(f"sq-{colour}-r{r + 1}", "top", mat, tone, (x, TOP_Y1 - INLAY / 2, z), (SQ, INLAY, SQ))
            s["repeat"] = {"count": 4, "translation": [0, 0, -2 * SQ]}
            out.append(s)
    return out


DROPPINGS = [(-0.27, 0.16, 0.013), (0.24, -0.27, 0.011), (0.29, 0.22, 0.014), (-0.05, 0.27, 0.010), (0.075, -0.125, 0.009)]
GUM = [("stool-black", 0.70, 0.06, 0.009), ("stool-black", 0.62, -0.05, 0.007), ("stool-white", -0.71, -0.07, 0.008)]


def body_50():
    """🔍️ The 1:50 body: every part above 7.5 mm, traces of use as texture solids."""
    b = [
        box("footing", "footing", "concrete-weathered", 0.72, (0, (FOOT_Y0 + FOOT_Y1) / 2, 0), (2 * FOOT_X, FOOT_Y1 - FOOT_Y0, 2 * FOOT_Z)),
        box("bed-column", "footing", "bedding-mortar", 0.55, (0, FOOT_Y1 + BED / 2, 0), (2 * COL_B - 0.02, BED, 2 * COL_B - 0.02)),
        vrev("bed-stool-white", "footing", "bedding-mortar", 0.55, [(0, 0), (0, STOOL_R - 0.01), (BED, STOOL_R - 0.01), (BED, 0)], (-STOOL_X, FOOT_Y1, 0)),
        vrev("bed-stool-black", "footing", "bedding-mortar", 0.55, [(0, 0), (0, STOOL_R - 0.01), (BED, STOOL_R - 0.01), (BED, 0)], (STOOL_X, FOOT_Y1, 0)),
        pext("column", "column", "granite-grey", 0.62, octagon(COL_B, COL_CH), FOOT_Y1 + BED, TOP_Y0),
        pext("top-slab", "top", "granite-grey", 0.66, CHIP_PLAN, TOP_Y0, TOP_Y1 - INLAY),
        pext("top-rim", "top", "granite-grey", 0.66, CHIP_PLAN, TOP_Y1 - INLAY, TOP_Y1, holes=[BOARD_HOLE]),
        vrev("stool-white", "stool-white", "granite-grey", 0.6, stool_profile(STOOL_H, STOOL_R, STOOL_CH), (-STOOL_X, FOOT_Y1 + BED, 0)),
        vrev("stool-black", "stool-black", "granite-grey", 0.6, stool_profile(STOOL_H, STOOL_R, STOOL_CH), (STOOL_X, FOOT_Y1 + BED, 0)),
    ]
    b += tiles()
    for i, (x, z, r) in enumerate(DROPPINGS):
        b.append(ell(f"dropping-{i + 1}", "top", "guano", 0.97, (x, TOP_Y1 + 0.0015, z), (r, 0.003, r * 0.8), role="texture", texture=True))
    for i, (bn, x, z, r) in enumerate(GUM):
        b.append(ell(f"gum-{i + 1}", bn, "chewing-gum", 0.25, (x, FOOT_Y1 + BED + STOOL_H + 0.0008, z), (r, 0.0015, r), role="texture", texture=True))
    return b


def body_100():
    """🔎️ The 1:100 body: beds and arris chamfers merged (10 mm under 15 mm), board kept square by square."""
    b = [
        box("footing", "footing", "concrete-weathered", 0.72, (0, (FOOT_Y0 + FOOT_Y1) / 2, 0), (2 * FOOT_X, FOOT_Y1 - FOOT_Y0, 2 * FOOT_Z)),
        box("column", "column", "granite-grey", 0.62, (0, (FOOT_Y1 + TOP_Y0) / 2, 0), (2 * COL_B, TOP_Y0 - FOOT_Y1, 2 * COL_B)),
        pext("top-slab", "top", "granite-grey", 0.66, CHIP_PLAN, TOP_Y0, TOP_Y1 - INLAY),
        pext("top-rim", "top", "granite-grey", 0.66, CHIP_PLAN, TOP_Y1 - INLAY, TOP_Y1, holes=[BOARD_HOLE]),
        vrev("stool-white", "stool-white", "granite-grey", 0.6, stool_profile(STOOL_H + BED, STOOL_R, STOOL_CH), (-STOOL_X, FOOT_Y1, 0)),
        vrev("stool-black", "stool-black", "granite-grey", 0.6, stool_profile(STOOL_H + BED, STOOL_R, STOOL_CH), (STOOL_X, FOOT_Y1, 0)),
    ]
    b += tiles(0.2, 0.84)
    return b


def body_200():
    """🗺️ The 1:200 body: the 20 mm inlay merged into one 80 mm slab, the 45 mm corner break kept."""
    return [
        box("footing", "footing", "concrete-weathered", 0.7, (0, (FOOT_Y0 + FOOT_Y1) / 2, 0), (2 * FOOT_X, FOOT_Y1 - FOOT_Y0, 2 * FOOT_Z)),
        box("column", "column", "granite-grey", 0.6, (0, (FOOT_Y1 + TOP_Y0) / 2, 0), (2 * COL_B, TOP_Y0 - FOOT_Y1, 2 * COL_B)),
        pext("top", "top", "granite-grey", 0.64, CHIP_PLAN, TOP_Y0, TOP_Y1),
        vrev("stool-white", "stool-white", "granite-grey", 0.58, stool_profile(STOOL_H + BED, STOOL_R, 0), (-STOOL_X, FOOT_Y1, 0)),
        vrev("stool-black", "stool-black", "granite-grey", 0.58, stool_profile(STOOL_H + BED, STOOL_R, 0), (STOOL_X, FOOT_Y1, 0)),
    ]


def body_500():
    """🌍️ The 1:500 body: five stone and concrete masses, the corner break (45 mm) under 75 mm left out."""
    return [
        box("footing", "footing", "concrete-weathered", 0.68, (0, (FOOT_Y0 + FOOT_Y1) / 2, 0), (2 * FOOT_X, FOOT_Y1 - FOOT_Y0, 2 * FOOT_Z)),
        box("column", "column", "granite-grey", 0.58, (0, (FOOT_Y1 + TOP_Y0) / 2, 0), (2 * COL_B, TOP_Y0 - FOOT_Y1, 2 * COL_B)),
        box("top", "top", "granite-grey", 0.62, (0, (TOP_Y0 + TOP_Y1) / 2, 0), (2 * TOP_B, TOP_Y1 - TOP_Y0, 2 * TOP_B)),
        vrev("stool-white", "stool-white", "granite-grey", 0.56, stool_profile(STOOL_H + BED, STOOL_R, 0), (-STOOL_X, FOOT_Y1, 0)),
        vrev("stool-black", "stool-black", "granite-grey", 0.56, stool_profile(STOOL_H + BED, STOOL_R, 0), (STOOL_X, FOOT_Y1, 0)),
    ]


REGION_OF = {"footing": "footing", "bed-column": "footing", "bed-stool-white": "footing", "bed-stool-black": "footing", "column": "column", "top-slab": "table-top", "top-rim": "table-top", "top": "table-top", "stool-white": "stools", "stool-black": "stools"}


def thermal_refs(body):
    """🌡️ Thermal references to every non-texture body solid."""
    out = []
    for s in body:
        if s.get("texture"):
            continue
        region = REGION_OF.get(s["id"]) or ("board-dark" if s["id"].startswith("sq-dark") else "board-light")
        out.append({"id": "th-" + s["id"], "of": s["id"], "region": region})
    return out


def thermal_board_quadrants(scale, y0, y1, sheet=False):
    """🌓️ The board generalised for the thermal model of a coarse scale: four 200 mm quadrants, dark on a1-d4 and e5-h8, light on the other two."""
    out = []
    quads = [("dark", -1, 1), ("dark", 1, -1), ("light", -1, -1), ("light", 1, 1)]
    for i, (c, sx, sz) in enumerate(quads):
        extra = {"sheet": True} if sheet else {}
        s = box(f"th-board-{c}-{i + 1}", "top", "granite-dark" if c == "dark" else "granite-light", 0.2 if c == "dark" else 0.85, (sx * BOARD / 2, (y0 + y1) / 2, sz * BOARD / 2), (BOARD, y1 - y0, BOARD), **extra)
        s["region"] = "board-" + c
        out.append(s)
    return out


def thermal_200():
    """🌡️ The 1:200 thermal model: the recess and the board in four 30 mm quadrants."""
    th = [{"id": "th-footing", "of": "footing", "region": "footing"}, {"id": "th-column", "of": "column", "region": "column"}, {"id": "th-stool-white", "of": "stool-white", "region": "stools"}, {"id": "th-stool-black", "of": "stool-black", "region": "stools"}]
    th.append(dict(pext("th-top-slab", "top", "granite-grey", 0.64, CHIP_PLAN, TOP_Y0, TOP_Y1 - 0.03), region="table-top"))
    th.append(dict(pext("th-top-rim", "top", "granite-grey", 0.64, CHIP_PLAN, TOP_Y1 - 0.03, TOP_Y1, holes=[BOARD_HOLE]), region="table-top"))
    return th + thermal_board_quadrants(200, TOP_Y1 - 0.03, TOP_Y1)


def thermal_500():
    """🌡️ The 1:500 thermal model: the slab whole, the board as four declared 5 mm sheets in its upper face."""
    th = [{"id": "th-footing", "of": "footing", "region": "footing"}, {"id": "th-column", "of": "column", "region": "column"}, {"id": "th-stool-white", "of": "stool-white", "region": "stools"}, {"id": "th-stool-black", "of": "stool-black", "region": "stools"}]
    th.append(dict(box("th-top", "top", "granite-grey", 0.62, (0, (TOP_Y0 + TOP_Y1 - 0.005) / 2, 0), (2 * TOP_B, TOP_Y1 - 0.005 - TOP_Y0, 2 * TOP_B)), region="table-top"))
    return th + thermal_board_quadrants(500, TOP_Y1 - 0.005, TOP_Y1, sheet=True)


def structure(scale, body):
    """🏗️ The structural idealisation of one scale: members are the stones and the slab themselves, dowelled bed joints, soil and lifting supports, masses at the centres."""
    f = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[scale]
    ids = {s["id"] for s in body}
    st = [{"id": "m-footing", "of": "footing", "member": "footing", "part": "member"}, {"id": "m-column", "of": "column", "member": "column", "part": "member"}]
    st += [{"id": "m-" + t, "of": t, "member": "top", "part": "member"} for t in ("top-slab", "top-rim", "top") if t in ids]
    st += [{"id": "m-stool-white", "of": "stool-white", "member": "stool-white", "part": "member"}, {"id": "m-stool-black", "of": "stool-black", "member": "stool-black", "part": "member"}]
    j = max(0.04, 2 * f)
    st.append(dict(box("joint-column-base", "column", "stainless-a4", 0.3, (0, FOOT_Y1 + BED / 2, 0), (j, max(0.03, 2 * f), j), role="detail"), member="column", part="joint"))
    st.append(dict(box("joint-column-top", "top", "stainless-a4", 0.3, (0, TOP_Y0, 0), (j, max(0.03, 2 * f), j), role="detail"), member="top", part="joint"))
    for side, sx in (("white", -1), ("black", 1)):
        st.append(dict(box(f"joint-stool-{side}", f"stool-{side}", "stainless-a4", 0.3, (sx * STOOL_X, FOOT_Y1 + BED / 2, 0), (j, max(0.03, 2 * f), j), role="detail"), member=f"stool-{side}", part="joint"))
        st.append(dict(box(f"support-soil-{side}", "footing", "concrete-weathered", 0.15, (sx * 0.5, FOOT_Y0 - f / 2, 0), (0.9, f, 2 * FOOT_Z - 0.05), role="ground"), member="footing", part="support", anchor=f"soil-{side}"))
    m = max(0.015, f / 2 + 0.0001)
    st.append(dict(ell("mass-footing", "footing", "concrete-weathered", 0.3, (0, (FOOT_Y0 + FOOT_Y1) / 2, 0), (m, m, m), role="detail"), member="footing", part="mass"))
    st.append(dict(ell("mass-column", "column", "granite-grey", 0.3, (0, FOOT_Y1 + BED + COL_H / 2, 0), (m, m, m), role="detail"), member="column", part="mass"))
    st.append(dict(ell("mass-top", "top", "granite-grey", 0.3, (0, TOP_Y0 + 0.04, 0), (m, m, m), role="detail"), member="top", part="mass"))
    for side, sx in (("white", -1), ("black", 1)):
        st.append(dict(ell(f"mass-stool-{side}", f"stool-{side}", "granite-grey", 0.3, (sx * STOOL_X, FOOT_Y1 + BED + STOOL_H / 2, 0), (m, m, m), role="detail"), member=f"stool-{side}", part="mass"))
    return st


# ---------------------------------------------------------------- 2D shapes


def shape(id_, bone, role, z, filled, tone, geom, **kw):
    """✏️ One 2D shape."""
    s = {"id": id_, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    s.update(kw)
    return s


def epoly(id_, bone, role, z, filled, tone, pts, closed=True, **kw):
    """✒️ An elevation polygon from world (x, y) points."""
    return shape(id_, bone, role, z, filled, tone, {"kind": "poly", "points": [list(loc(bone, *p)) for p in pts], "closed": closed}, **kw)


def erect(id_, bone, role, z, tone, x0, y0, x1, y1, **kw):
    """▭️ An elevation rectangle from world corners."""
    return epoly(id_, bone, role, z, True, tone, [(x0, y0), (x1, y0), (x1, y1), (x0, y1)], **kw)


def wpoly(id_, bone, role, z, filled, tone, pts, closed=True, **kw):
    """🖊️ A front or plan polygon in world rest coordinates (front: z, y; plan: x, z)."""
    return shape(id_, bone, role, z, filled, tone, {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}, **kw)


def wrect(id_, bone, role, z, tone, a0, b0, a1, b1, **kw):
    """▫️ A front or plan rectangle in world rest coordinates."""
    return wpoly(id_, bone, role, z, True, tone, [(a0, b0), (a1, b0), (a1, b1), (a0, b1)], **kw)


def wcirc(id_, bone, role, z, filled, tone, cx, cy, r, **kw):
    """⭕️ A front or plan circle in world rest coordinates."""
    return shape(id_, bone, role, z, filled, tone, {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(r), "ry": r6(r)}, **kw)


def ecirc(id_, bone, role, z, filled, tone, x, y, r, **kw):
    """🔘️ An elevation circle at a world point."""
    u, v = loc(bone, x, y)
    return shape(id_, bone, role, z, filled, tone, {"kind": "ellipse", "cx": u, "cy": v, "rx": r6(r), "ry": r6(r)}, **kw)


GR = dict(region="column", material="granite-grey")
GT = dict(region="table-top", material="granite-grey")
GTB = dict(regions=["table-top", "board-dark", "board-light"], material="granite-grey")
GS = dict(region="stools", material="granite-grey")
CF = dict(region="footing", material="concrete-weathered")
CM = dict(region="footing", material="bedding-mortar")


def stool_elev(side, sx, scale):
    """🪑️ Elevation (and front) outline of one stool at a scale, in world (horizontal, y)."""
    x0, x1 = sx * STOOL_X - STOOL_R, sx * STOOL_X + STOOL_R
    yb = FOOT_Y1 + BED if scale == 50 else FOOT_Y1
    yt = FOOT_Y1 + BED + STOOL_H
    if scale in (50, 100):
        return [(x0, yb), (x1, yb), (x1, yt - STOOL_CH), (x1 - STOOL_CH, yt), (x0 + STOOL_CH, yt), (x0, yt - STOOL_CH)]
    return [(x0, yb), (x1, yb), (x1, yt), (x0, yt)]


def elevation(scale):
    """🖼️ The side elevation seen from the right (north, -z): footing with its buried part, bed joints, column, top, stools and the north-side weathering."""
    e = []
    hatch_c = {"hatch": "concrete"} if scale in (50, 100, 200) else {}
    hatch_g = {"hatch": "granite"} if scale == 50 else {}
    e.append(erect("footing", "footing", "body", 1, 0.72, -FOOT_X, FOOT_Y0, FOOT_X, FOOT_Y1, **hatch_c, **CF))
    yb = FOOT_Y1 + BED if scale == 50 else FOOT_Y1
    if scale == 50:
        e.append(erect("bed-column", "footing", "body", 2, 0.5, -COL_B + 0.01, FOOT_Y1, COL_B - 0.01, FOOT_Y1 + BED, **CM))
        for side, sx in (("white", -1), ("black", 1)):
            e.append(erect(f"bed-stool-{side}", "footing", "body", 2, 0.5, sx * STOOL_X - STOOL_R + 0.01, FOOT_Y1, sx * STOOL_X + STOOL_R - 0.01, FOOT_Y1 + BED, **CM))
    e.append(erect("column", "column", "body", 3, 0.62, -COL_B, yb, COL_B, TOP_Y0, **hatch_g, **GR))
    if scale == 500:
        e.append(erect("top", "top", "body", 5, 0.62, -TOP_B, TOP_Y0, TOP_B, TOP_Y1, **GTB))
    else:
        e.append(erect("top", "top", "body", 5, 0.66, -TOP_B, TOP_Y0, TOP_B, TOP_Y1, **hatch_g, **GTB))
    for side, sx in (("white", -1), ("black", 1)):
        e.append(epoly(f"stool-{side}", f"stool-{side}", "body", 4, True, 0.6, stool_elev(side, sx, scale), **hatch_g, **GS))
    if scale in (50, 100):
        e.append(epoly("chip-edge", "top", "detail", 6, False, 0.3, [(TOP_B - CHIP, TOP_Y0), (TOP_B - CHIP, TOP_Y1)], closed=False, **GT))
    if scale == 200:
        e.append(epoly("chip-edge", "top", "detail", 6, False, 0.3, [(TOP_B - CHIP, TOP_Y0), (TOP_B - CHIP, TOP_Y1)], closed=False, **GT))
    if scale == 50:
        for sx in (-1, 1):
            e.append(epoly(f"column-arris-{'l' if sx < 0 else 'r'}", "column", "detail", 3.5, False, 0.35, [(sx * (COL_B - COL_CH), yb), (sx * (COL_B - COL_CH), TOP_Y0)], closed=False, **GR))
        for side, sx in (("white", -1), ("black", 1)):
            e.append(epoly(f"stool-{side}-chamfer", f"stool-{side}", "detail", 4.5, False, 0.35, [(sx * STOOL_X - STOOL_R, FOOT_Y1 + BED + STOOL_H - STOOL_CH), (sx * STOOL_X + STOOL_R, FOOT_Y1 + BED + STOOL_H - STOOL_CH)], closed=False, **GS))
        e.append(epoly("algae-column", "column", "texture", 3.6, True, 0.42, [(-0.15, 0.05), (0.15, 0.05), (0.15, 0.13), (0.09, 0.19), (0.03, 0.15), (-0.04, 0.22), (-0.11, 0.17), (-0.15, 0.2)], hatch="algae", **GR))
        e.append(epoly("graffiti-tag", "column", "texture", 3.7, False, 0.2, [(-0.11, 0.33), (-0.08, 0.41), (-0.05, 0.34), (-0.02, 0.43), (0.01, 0.35), (0.05, 0.42), (0.09, 0.36), (0.06, 0.32), (-0.1, 0.37)], closed=False, **GR))
        e.append(erect("sticker-residue", "column", "texture", 3.7, 0.82, -0.04, 0.5, 0.03, 0.545, **GR))
        e.append(epoly("algae-footing", "footing", "texture", 2.6, True, 0.45, [(-1.0, 0.0), (1.0, 0.0), (1.0, 0.03), (0.6, 0.02), (0.2, 0.035), (-0.3, 0.02), (-0.7, 0.032), (-1.0, 0.025)], hatch="algae", **CF))
        for side, sx in (("white", -1), ("black", 1)):
            c = sx * STOOL_X
            e.append(epoly(f"algae-stool-{side}", f"stool-{side}", "texture", 4.6, True, 0.42, [(c - 0.18, 0.05), (c + 0.18, 0.05), (c + 0.18, 0.11), (c + 0.08, 0.15), (c - 0.02, 0.1), (c - 0.12, 0.16), (c - 0.18, 0.12)], hatch="algae", **GS))
        e.append(epoly("dropping-run", "top", "texture", 6.2, True, 0.96, [(0.33, 0.74), (0.345, 0.74), (0.344, 0.71), (0.338, 0.695), (0.333, 0.712)], **GT))
    if scale == 100:
        e.append(epoly("algae-column", "column", "texture", 3.6, True, 0.42, [(-0.15, 0.04), (0.15, 0.04), (0.15, 0.13), (0.06, 0.2), (-0.06, 0.15), (-0.15, 0.2)], hatch="algae", **GR))
    return e


def front(scale):
    """🖼️ The front elevation seen from ahead (+x, Black's stool nearest): footing, beds, column, both stools, top."""
    fr = []
    hatch_c = {"hatch": "concrete"} if scale in (50, 100, 200) else {}
    hatch_g = {"hatch": "granite"} if scale == 50 else {}
    fr.append(wrect("footing", "footing", "body", 1, 0.72, -FOOT_Z, FOOT_Y0, FOOT_Z, FOOT_Y1, **hatch_c, **CF))
    yb = FOOT_Y1 + BED if scale == 50 else FOOT_Y1
    if scale == 50:
        fr.append(wrect("bed-column", "footing", "body", 2, 0.5, -COL_B + 0.01, FOOT_Y1, COL_B - 0.01, FOOT_Y1 + BED, **CM))
        for side in ("white", "black"):
            fr.append(wrect(f"bed-stool-{side}", "footing", "body", 2, 0.5, -STOOL_R + 0.01, FOOT_Y1, STOOL_R - 0.01, FOOT_Y1 + BED, **CM))
    fr.append(wrect("column", "column", "body", 3, 0.62, -COL_B, yb, COL_B, TOP_Y0, **hatch_g, **GR))
    for side, zz in (("white", 3.5), ("black", 4)):
        fr.append(wpoly(f"stool-{side}", f"stool-{side}", "body", zz, True, 0.6, stool_elev(side, 0, scale), **hatch_g, **GS))
    fr.append(wrect("top", "top", "body", 5, 0.64 if scale == 500 else 0.66, -TOP_B, TOP_Y0, TOP_B, TOP_Y1, **hatch_g, **GT))
    if scale in (50, 100, 200):
        fr.append(wpoly("chip-edge", "top", "detail", 6, False, 0.3, [(-TOP_B + CHIP, TOP_Y0), (-TOP_B + CHIP, TOP_Y1)], closed=False, **GT))
    if scale == 50:
        for sz in (-1, 1):
            fr.append(wpoly(f"column-arris-{'r' if sz < 0 else 'l'}", "column", "detail", 3.2, False, 0.35, [(sz * (COL_B - COL_CH), 0.45), (sz * (COL_B - COL_CH), TOP_Y0)], closed=False, **GR))
        fr.append(wpoly("stool-black-chamfer", "stool-black", "detail", 4.5, False, 0.35, [(-STOOL_R, FOOT_Y1 + BED + STOOL_H - STOOL_CH), (STOOL_R, FOOT_Y1 + BED + STOOL_H - STOOL_CH)], closed=False, **GS))
        fr.append(wpoly("dropping-run", "top", "texture", 6.2, True, 0.96, [(0.21, 0.74), (0.226, 0.74), (0.224, 0.705), (0.218, 0.69), (0.213, 0.708)], **GT))
        fr.append(wpoly("algae-footing", "footing", "texture", 2.6, True, 0.45, [(-0.45, 0.0), (-0.2, 0.0), (-0.24, 0.022), (-0.33, 0.03), (-0.45, 0.034)], hatch="algae", **CF))
        fr.append(wpoly("algae-stool-black", "stool-black", "texture", 4.6, True, 0.42, [(-0.18, 0.05), (-0.05, 0.05), (-0.08, 0.1), (-0.14, 0.14), (-0.18, 0.12)], hatch="algae", **GS))
    return fr


def plan(scale):
    """🗺️ The plan from above: footing, stools, the chipped top with its inlaid board and what pigeons, gum and weather left on it."""
    p = []
    hatch_c = {"hatch": "concrete"} if scale in (50, 100) else {}
    p.append(wrect("footing", "footing", "body", 1, 0.72, -FOOT_X, -FOOT_Z, FOOT_X, FOOT_Z, **hatch_c, **CF))
    for side, sx in (("white", -1), ("black", 1)):
        p.append(wcirc(f"stool-{side}", f"stool-{side}", "body", 4, True, 0.6, sx * STOOL_X, 0, STOOL_R, **GS))
        if scale in (50, 100):
            p.append(wcirc(f"stool-{side}-chamfer", f"stool-{side}", "detail", 4.5, False, 0.35, sx * STOOL_X, 0, STOOL_R - STOOL_CH, **GS))
    p.append(wpoly("top", "top", "body", 5, True, 0.66, SQUARE_PLAN if scale == 500 else CHIP_PLAN, **GT))
    if scale == 500:
        p.append(wrect("board", "top", "detail", 6, 0.45, -BOARD, -BOARD, BOARD, BOARD, regions=["board-dark", "board-light"]))
        return p
    p.append(wrect("board-light", "top", "detail", 6, 0.88, -BOARD, -BOARD, BOARD, BOARD, region="board-light", material="granite-light"))
    for r in range(8):
        cells = [(-BOARD + SQ * r, BOARD - SQ * (f + 1)) for f in range(8) if (r + f) % 2 == 0]
        if scale == 200:
            d = " ".join(f"M {r6(x0)} {r6(z0)} H {r6(x0 + SQ)} V {r6(z0 + SQ)} H {r6(x0)} Z" for x0, z0 in cells)
            p.append(shape(f"rank-{r + 1}-dark", "top", "detail", 6.1, True, 0.2, {"kind": "path", "d": d}, region="board-dark", material="granite-dark"))
            continue
        for x0, z0 in cells:
            f = round((BOARD - z0) / SQ) - 1
            p.append(wrect(f"sq-{'abcdefgh'[f]}{r + 1}", "top", "detail", 6.1, 0.18, x0, z0, x0 + SQ, z0 + SQ, region="board-dark", material="granite-dark"))
    if scale == 50:
        p.append(wrect("wear-centre", "top", "texture", 6.3, 0.75, -0.075, -0.075, 0.075, 0.075, hatch="wear", region="board-light", material="granite-light"))
        for i, (x, z, r) in enumerate(DROPPINGS):
            p.append(shape(f"dropping-{i + 1}", "top", "texture", 6.5, True, 0.97, {"kind": "ellipse", "cx": r6(x), "cy": r6(z), "rx": r6(r), "ry": r6(r * 0.8)}, **GT))
        for i, (bn, x, z, r) in enumerate(GUM):
            p.append(wcirc(f"gum-{i + 1}", bn, "texture", 4.8, True, 0.25, x, z, r, **GS))
        p.append(wpoly("crack", "top", "texture", 6.4, False, 0.3, [(-0.35, 0.13), (-0.31, 0.15), (-0.28, 0.14), (-0.24, 0.17), (-0.215, 0.215)], closed=False, **GT))
        p.append(wpoly("lichen-footing", "footing", "texture", 1.5, True, 0.5, [(-0.98, -0.45), (-0.62, -0.45), (-0.66, -0.43), (-0.8, -0.415), (-0.98, -0.42)], hatch="algae", **CF))
        p.append(wpoly("worn-footing-white", "footing", "texture", 1.4, True, 0.62, [(-0.49, -0.3), (-0.36, -0.32), (-0.36, 0.32), (-0.49, 0.3)], **CF))
        p.append(wpoly("worn-footing-black", "footing", "texture", 1.4, True, 0.62, [(0.36, -0.32), (0.49, -0.3), (0.49, 0.3), (0.36, 0.32)], **CF))
    if scale == 100:
        for i, (x, z, r) in enumerate(DROPPINGS[:3]):
            p.append(shape(f"dropping-{i + 1}", "top", "texture", 6.5, True, 0.97, {"kind": "ellipse", "cx": r6(x), "cy": r6(z), "rx": 0.02, "ry": 0.016}, **GT))
    return p


# ---------------------------------------------------------------- representations


def hatches(scale):
    """▤️ The hatches of one scale."""
    h = []
    if scale in (50, 100, 200):
        h.append({"id": "concrete", "title": T("Concrete, stippled", "Beton, gepunktet"), "pattern": "dots", "angleDeg": 0, "spacingMm": {50: 0.9, 100: 0.7, 200: 0.5}[scale], "penMm": 0.13})
    if scale == 50:
        h.append({"id": "granite", "title": T("Flamed granite, stipple", "Geflammter Granit, Punktierung"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 1.2, "penMm": 0.13})
        h.append({"id": "wear", "title": T("Worn, polished by use", "Abgenutzt, durch Gebrauch poliert"), "pattern": "lines", "angleDeg": 30, "spacingMm": 0.8, "penMm": 0.13})
    if scale in (50, 100):
        h.append({"id": "algae", "title": T("Algae and lichen film", "Algen- und Flechtenbelag"), "pattern": "lines", "angleDeg": 75, "spacingMm": 0.5, "penMm": 0.13})
    return h


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, 500: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

REP_DECISIONS = {
    50: {
        "silhouette": T("The whole set at 40 x 18 mm: the 2.00 x 0.90 m concrete footing with its 160 mm buried below the gravel line, the eroded mortar bed joints, the chamfered 300 mm granite column, the 700 x 80 mm top with its broken corner, the two 360 mm stools with their 15 mm top chamfer; plan with all 64 inlaid squares.", "Das ganze Ensemble mit 40 x 18 mm: das 2,00 x 0,90 m große Betonfundament mit seinen 160 mm unter der Kieslinie, die ausgewaschenen Mörtelbettfugen, die gefaste Granitsäule 300 mm, die Platte 700 x 80 mm mit ihrer abgeschlagenen Ecke, die zwei Hocker 360 mm mit 15 mm Kopffase; im Grundriss alle 64 eingelegten Felder."),
        "retained": T("Bed joints 10 mm set back 10 mm from the stone faces, the 10 mm column arrises, the stool chamfers, the 45 mm corner break of the top, the board square by square with a1 dark on White's left, pigeon droppings, gum spots, a hairline crack, algae on the north faces, a tag and sticker residue on the column.", "Bettfugen 10 mm, 10 mm hinter die Steinflächen zurückgewaschen, die 10-mm-Säulenfasen, die Hockerfasen, der 45-mm-Eckabbruch der Platte, das Brett Feld für Feld mit a1 dunkel links von Weiß, Taubenkot, Kaugummiflecken, ein Haarriss, Algen an den Nordseiten, ein Tag und Aufkleberreste an der Säule."),
        "merged": T("The 2 mm joints between the inlaid squares merge into the squares; the eased 4 mm arrises of the top merge into its outline.", "Die 2-mm-Fugen zwischen den eingelegten Feldern gehen in den Feldern auf; die 4-mm-Kantenbrechung der Platte geht in ihrem Umriss auf."),
        "omitted": T("The stainless dowels inside the bed joints (hidden), the reinforcement of the footing and the gravel bed under it.", "Die Edelstahldollen in den Bettfugen (verdeckt), die Bewehrung des Fundaments und das Kiesbett darunter."),
        "thinElements": T("Arrises, chamfer edges and the chip edge as detail strokes, the gravel line is the ground line of the sheet; tag and crack as texture strokes.", "Kanten, Fasenkanten und die Bruchkante als Detailstriche, die Kieslinie ist die Bodenlinie des Blatts; Tag und Riss als Texturstriche."),
        "hatching": T("Concrete stippled with dots, flamed granite with a stipple, algae with steep fine lines, polished wear with 30 degree lines.", "Beton mit Punkten, geflammter Granit mit Punktierung, Algen mit steilen feinen Linien, polierte Abnutzung mit 30-Grad-Linien."),
        "styles": T("ISO 128 pens 0.5 / 0.25 / 0.13 mm; granite mid grey, the dark squares nearly black, guano white.", "ISO-128-Stifte 0,5 / 0,25 / 0,13 mm; Granit mittelgrau, die dunklen Felder fast schwarz, Guano weiß."),
        "overlays": T("None: the players are loads on the seats and the table edge, not drawn figures.", "Keine: die Spieler sind Lasten auf den Sitzen und der Tischkante, keine gezeichneten Figuren."),
    },
    100: {
        "silhouette": T("The set at 20 x 9 mm: footing with the buried part, column and stools standing directly on the footing, the top with its broken corner and the full checkerboard in plan.", "Das Ensemble mit 20 x 9 mm: Fundament mit dem eingegrabenen Teil, Säule und Hocker direkt auf dem Fundament, die Platte mit abgeschlagener Ecke und das volle Schachbrett im Grundriss."),
        "retained": T("The 45 mm corner break, all 64 squares (0.5 mm on paper), three pigeon droppings, the algae at the column foot.", "Der 45-mm-Eckabbruch, alle 64 Felder (0,5 mm auf dem Papier), drei Taubenkotflecken, die Algen am Säulenfuß."),
        "merged": T("The 10 mm bed joints into column and stools and the 10 mm column arrises into a plain square; the 15 mm stool chamfers (0.15 mm) stay.", "Die 10-mm-Bettfugen in Säule und Hocker und die 10-mm-Säulenfasen in ein einfaches Quadrat; die 15-mm-Hockerfasen (0,15 mm) bleiben."),
        "omitted": T("Tag, sticker, crack, gum spots and the worn patches on the footing, below 0.15 mm of paper.", "Tag, Aufkleber, Riss, Kaugummiflecken und die abgelaufenen Stellen auf dem Fundament, unter 0,15 mm auf dem Papier."),
        "thinElements": T("The chip edge as a detail stroke.", "Die Bruchkante als Detailstrich."),
        "hatching": T("Concrete dots 0.7 mm, algae lines 0.5 mm.", "Betonpunkte 0,7 mm, Algenlinien 0,5 mm."),
        "styles": T("Pens 0.35 / 0.18 / 0.13 mm.", "Stifte 0,35 / 0,18 / 0,13 mm."),
        "overlays": T("None.", "Keine."),
    },
    200: {
        "silhouette": T("The set at 10 x 4.5 mm: five masses, footing, column, top with broken corner, two stools; the board as eight rank shapes of four dark squares each on a light field.", "Das Ensemble mit 10 x 4,5 mm: fünf Massen, Fundament, Säule, Platte mit abgeschlagener Ecke, zwei Hocker; das Brett als acht Reihenformen zu je vier dunklen Feldern auf hellem Grund."),
        "retained": T("The corner break (0.23 mm), the checkerboard (0.25 mm squares) that tells this table from a plain one.", "Der Eckabbruch (0,23 mm), das Schachbrett (Felder 0,25 mm), das diesen Tisch von einem einfachen unterscheidet."),
        "merged": T("Beds, chamfers and the inlay depth into the stones.", "Bettfugen, Fasen und die Einlagetiefe in die Steine."),
        "omitted": T("All traces of use except the broken corner: guano, gum, algae, tag are under the printable feature.", "Alle Gebrauchsspuren außer der abgeschlagenen Ecke: Guano, Kaugummi, Algen, Tag liegen unter der druckbaren Einzelheit."),
        "thinElements": T("The chip edge as a detail stroke.", "Die Bruchkante als Detailstrich."),
        "hatching": T("Concrete dots 0.5 mm only.", "Nur Betonpunkte 0,5 mm."),
        "styles": T("Pens 0.25 / 0.13 mm.", "Stifte 0,25 / 0,13 mm."),
        "overlays": T("None.", "Keine."),
    },
    500: {
        "silhouette": T("The set at 4 x 1.8 mm: footing, column, square top, two stools as five plain masses; the board as one mid-grey 400 mm square in plan.", "Das Ensemble mit 4 x 1,8 mm: Fundament, Säule, quadratische Platte, zwei Hocker als fünf einfache Massen; das Brett im Grundriss als ein mittelgraues Quadrat 400 mm."),
        "retained": T("The footprint of the footing, the column under the top, the stool circles and the board field.", "Die Grundfläche des Fundaments, die Säule unter der Platte, die Hockerkreise und das Brettfeld."),
        "merged": T("The 64 squares into one field (50 mm squares would print 0.05 mm at 1:1000).", "Die 64 Felder zu einem Feld (Felder von 50 mm würden bei 1:1000 mit 0,05 mm drucken)."),
        "omitted": T("The corner break (45 mm) and every trace of use.", "Der Eckabbruch (45 mm) und alle Gebrauchsspuren."),
        "thinElements": T("None.", "Keine."),
        "hatching": T("None.", "Keine."),
        "styles": T("Pens 0.18 / 0.13 mm.", "Stifte 0,18 / 0,13 mm."),
        "overlays": T("None.", "Keine."),
    },
}


def representation(scale):
    """🖨️ One authored representation."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": hatches(scale), "geometry": {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}, "decisions": REP_DECISIONS[scale]}


VOL_DECISIONS = {
    50: [
        T("Body for 7.5 mm: footing 2.00 x 0.20 x 0.90 m (top 40 mm above the gravel), three 10 mm mortar beds washed back 10 mm, the 300 mm column as an octagonal prism with 10 mm arrises, the top as a 60 mm slab and a 20 mm rim with the 400 mm recess, both with the 45 mm corner break, the 64 squares of 50 x 20 mm as 16 rows of four, the stools as revolved 360 mm cylinders with 15 mm chamfers.", "Körper für 7,5 mm: Fundament 2,00 x 0,20 x 0,90 m (Oberkante 40 mm über dem Kies), drei 10-mm-Mörtelbetten 10 mm zurückgewaschen, die Säule 300 mm als achteckiges Prisma mit 10-mm-Fasen, die Platte als 60-mm-Platte und 20-mm-Rand mit der 400-mm-Vertiefung, beide mit dem 45-mm-Eckabbruch, die 64 Felder 50 x 20 mm als 16 Reihen zu vier, die Hocker als gedrehte Zylinder 360 mm mit 15-mm-Fasen."),
        T("Traces of use as texture solids: five pigeon droppings 18 to 28 mm on the top, three gum spots on the stool seats; algae, tag, sticker and crack are 2D texture only.", "Gebrauchsspuren als Texturkörper: fünf Taubenkotflecken 18 bis 28 mm auf der Platte, drei Kaugummiflecken auf den Hockersitzen; Algen, Tag, Aufkleber und Riss nur als 2D-Textur."),
        T("Thermal: references to every stone, slab and bed; structure: the stones are the members, stainless dowel joints across the bed joints, soil pads under both halves of the footing, masses at the centres; the sling contacts under the overhang act only in the re-levelling pose and carry no support solid.", "Thermik: Verweise auf jeden Stein, die Platte und die Betten; Tragwerk: die Steine sind die Stäbe, Edelstahl-Dollengelenke über die Bettfugen, Bodenauflager unter beiden Fundamenthälften, Massen in den Schwerpunkten; die Schlingenkontakte unter dem Überstand wirken nur in der Pose Neu ausrichten und haben keinen Auflagerkörper."),
    ],
    100: [
        T("Body for 15 mm: the 10 mm beds and the 10 mm column arrises are below the feature, so column and stools stand directly on the footing and the column is a plain 300 mm square; the stools keep their 15 mm chamfer; the inlay keeps its 64 squares (50 mm, 20 mm deep) and the 45 mm corner break.", "Körper für 15 mm: die 10-mm-Betten und die 10-mm-Säulenfasen liegen unter der Einzelheit, daher stehen Säule und Hocker direkt auf dem Fundament und die Säule ist ein einfaches Quadrat 300 mm; die Hocker behalten ihre 15-mm-Fase; die Einlage behält ihre 64 Felder (50 mm, 20 mm tief) und den 45-mm-Eckabbruch."),
        T("No texture solids: droppings and gum are below 15 mm.", "Keine Texturkörper: Kot und Kaugummi liegen unter 15 mm."),
    ],
    200: [
        T("Body for 30 mm: the 20 mm inlay merges into one 80 mm slab with the 45 mm corner break; five masses. Thermal: the recess and the board are kept as four 200 mm quadrants of 30 mm (dark a1-d4 and e5-h8) so both board regions keep their own solids.", "Körper für 30 mm: die 20-mm-Einlage geht in einer 80-mm-Platte mit 45-mm-Eckabbruch auf; fünf Massen. Thermik: Vertiefung und Brett bleiben als vier Viertel 200 mm mit 30 mm (dunkel a1-d4 und e5-h8), so behalten beide Brettregionen eigene Körper."),
    ],
    500: [
        T("Body for 75 mm: the corner break (45 mm) is left out, the top is a plain 700 x 80 x 700 mm slab; five masses, already minimal. Thermal: the board as four declared 5 mm sheets flush in the upper face of the slab.", "Körper für 75 mm: der Eckabbruch (45 mm) entfällt, die Platte ist eine einfache Platte 700 x 80 x 700 mm; fünf Massen, bereits minimal. Thermik: das Brett als vier deklarierte 5-mm-Schichten bündig in der Oberseite der Platte."),
    ],
}


def volumes():
    """🧊️ The four handcrafted volumes."""
    out = {}
    for scale, body, th in ((50, body_50(), None), (100, body_100(), None), (200, body_200(), thermal_200()), (500, body_500(), thermal_500())):
        out[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": th if th is not None else thermal_refs(body), "structure": structure(scale, body), "decisions": VOL_DECISIONS[scale]}
    return out


# ---------------------------------------------------------------- physics


def masses():
    """⚖️ Masses from volume x density, per bone."""
    chip = 0.5 * CHIP**2
    top_slab = ((2 * TOP_B) ** 2 - chip) * (TOP_Y1 - TOP_Y0) - (2 * BOARD) ** 2 * INLAY
    tiles_v = 32 * SQ * SQ * INLAY
    col_v = ((2 * COL_B) ** 2 - 2 * COL_CH**2) * COL_H
    stool_v = math.pi * STOOL_R**2 * (STOOL_H - STOOL_CH) + math.pi / 3 * STOOL_CH * (STOOL_R**2 + STOOL_R * (STOOL_R - STOOL_CH) + (STOOL_R - STOOL_CH) ** 2)
    foot_v = 2 * FOOT_X * 2 * FOOT_Z * (FOOT_Y1 - FOOT_Y0)
    beds_v = (2 * COL_B - 0.02) ** 2 * BED + 2 * math.pi * (STOOL_R - 0.01) ** 2 * BED
    m = {
        "top-slab": top_slab * RHO["granite-grey"],
        "tiles-dark": tiles_v * RHO["granite-dark"],
        "tiles-light": tiles_v * RHO["granite-light"],
        "column": col_v * RHO["granite-grey"],
        "stool": stool_v * RHO["granite-grey"],
        "footing-concrete": foot_v * RHO["concrete-weathered"],
        "beds": beds_v * RHO["bedding-mortar"],
    }
    m["top"] = m["top-slab"] + m["tiles-dark"] + m["tiles-light"]
    m["footing"] = m["footing-concrete"] + m["beds"]
    m["total"] = m["top"] + m["column"] + 2 * m["stool"] + m["footing"]
    return m


def areas():
    """📏️ Total outer areas of the 1:50 thermal regions (C29: faces against another region count)."""
    chip = 0.5 * CHIP**2
    per = 8 * TOP_B - 2 * CHIP + CHIP * math.sqrt(2)
    top = 2 * ((2 * TOP_B) ** 2 - chip) + 4 * 2 * BOARD * INLAY + per * (TOP_Y1 - TOP_Y0)
    tile = 32 * (2 * SQ * SQ + 4 * SQ * INLAY)
    col = (8 * (COL_B - COL_CH) + 4 * COL_CH * math.sqrt(2)) * COL_H + 2 * ((2 * COL_B) ** 2 - 2 * COL_CH**2)
    r2 = STOOL_R - STOOL_CH
    stool = 2 * math.pi * STOOL_R * (STOOL_H - STOOL_CH) + math.pi * (STOOL_R + r2) * math.hypot(STOOL_CH, STOOL_CH) + math.pi * r2**2 + math.pi * STOOL_R**2
    foot = 2 * (2 * FOOT_X * 2 * FOOT_Z) + 2 * (2 * FOOT_X + 2 * FOOT_Z) * (FOOT_Y1 - FOOT_Y0) + 4 * (2 * COL_B - 0.02) * BED + 2 * 2 * math.pi * (STOOL_R - 0.01) * BED
    return {"table-top": top, "board-dark": tile, "board-light": tile, "column": col, "stools": 2 * stool, "footing": foot}


M = masses()
A = areas()


def material(id_, en, de, eps, alpha, rho, lam, cp, e=None, strength=None, src="authored-estimate", ref="iso-10456-2007"):
    """🧱️ One material."""
    m = {"id": id_, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha}
    if rho is not None:
        m["densityKgM3"] = rho
    if lam is not None:
        m["conductivityWmK"] = lam
    if cp is not None:
        m["specificHeatJKgK"] = cp
    if e is not None:
        m["elasticModulusPa"] = e
    if strength is not None:
        m["strengthPa"] = strength
    m["source"] = src
    m["reference"] = ref
    return m


MATERIALS = [
    material("granite-grey", "Light grey granite, flamed, weathered and soiled", "Hellgrauer Granit, geflammt, verwittert und verschmutzt", 0.9, 0.66, RHO["granite-grey"], 2.8, 1000, 50e9, 12e6),
    material("granite-dark", "Black gabbro-type inlay granite, polished and worn dull", "Schwarzer Gabbro-Einlagegranit, poliert und stumpf abgenutzt", 0.92, 0.88, RHO["granite-dark"], 2.8, 1000, 90e9, 18e6),
    material("granite-light", "White-grey inlay granite, polished and worn", "Weißgrauer Einlagegranit, poliert und abgenutzt", 0.9, 0.5, RHO["granite-light"], 2.8, 1000, 50e9, 12e6),
    material("concrete-weathered", "Concrete C25/30, weathered surface with algae", "Beton C25/30, verwitterte Oberfläche mit Algen", 0.92, 0.72, RHO["concrete-weathered"], 2.0, 1000, 31e9, 2.6e6),
    material("bedding-mortar", "Cement bedding mortar, partly washed out", "Zementmörtelbett, teilweise ausgewaschen", 0.92, 0.68, RHO["bedding-mortar"], 1.3, 1000, 15e9, 2.0e6),
    material("stainless-a4", "Stainless steel A4 (1.4401) dowel", "Edelstahl A4 (1.4401) Dollen", 0.3, 0.45, 8000, 15, 500, 200e9, 500e6, "literature", "en-10088-3-2014"),
    material("guano", "Dried pigeon droppings", "Getrockneter Taubenkot", 0.9, 0.3, 1200, 0.4, 1500, None, None, "illustrative", "incropera-2011"),
    material("chewing-gum", "Trodden chewing gum, blackened", "Festgetretener Kaugummi, geschwärzt", 0.9, 0.85, 1100, 0.2, 1600, None, None, "illustrative", "incropera-2011"),
]


def region(id_, en, de, mat, area, lc, geom, svf, hc, src="authored-estimate", ref="iso-10456-2007", **kw):
    """🔥️ One passive thermal region."""
    r = {"id": id_, "title": T(en, de), "kind": "surface", "model": "passive", "material": mat, "areaM2": round(area, 4), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": [], "heatCapacityJM2K": round(hc), "source": src, "reference": ref}
    r.update(kw)
    return r


REGIONS = [
    region("table-top", "Granite table top (rim, edges, underside)", "Granitplatte (Rand, Kanten, Unterseite)", "granite-grey", A["table-top"], 0.7, "horizontal-plate-up", 0.55, M["top-slab"] * 1000 / A["table-top"], ratedPowerW=0, ratedPowerBasis="passive"),
    region("board-dark", "Dark inlaid squares", "Dunkle eingelegte Felder", "granite-dark", A["board-dark"], 0.05, "horizontal-plate-up", 0.55, M["tiles-dark"] * 1000 / A["board-dark"]),
    region("board-light", "Light inlaid squares", "Helle eingelegte Felder", "granite-light", A["board-light"], 0.05, "horizontal-plate-up", 0.55, M["tiles-light"] * 1000 / A["board-light"]),
    region("column", "Granite column", "Granitsäule", "granite-grey", A["column"], 0.3, "vertical-plate", 0.4, M["column"] * 1000 / A["column"]),
    region("stools", "Two granite stools", "Zwei Granithocker", "granite-grey", A["stools"], 0.36, "cylinder", 0.45, 2 * M["stool"] * 1000 / A["stools"]),
    region("footing", "Concrete footing and mortar beds", "Betonfundament und Mörtelbetten", "concrete-weathered", A["footing"], 0.9, "horizontal-plate-up", 0.5, M["footing"] * 1000 / A["footing"]),
]

ENV = {"airTempC": 27, "meanRadiantTempC": 34, "skyTempC": 12, "groundTempC": 33, "relativeHumidity": 0.45, "windMS": 1.2, "windDirectionDeg": 90, "solarWM2": 780, "diffuseFraction": 0.2, "pressurePa": 101325}


def rb(exp, sun, contact=None):
    """🌬️ One region binding."""
    b = {"exposedFraction": exp, "sunlitFraction": sun}
    if contact:
        b["contact"] = {"fraction": contact[0], "tempC": contact[1]}
    return b


def soil_contact():
    """🪨️ Share of the footing area against the gravel and soil: underside plus the buried 160 mm of the edges."""
    under = 2 * FOOT_X * 2 * FOOT_Z
    sides = 2 * (2 * FOOT_X + 2 * FOOT_Z) * (0 - FOOT_Y0)
    return round((under + sides) / A["footing"], 3)


def exposed_footing():
    """☀️ Share of the footing area open to the air: the top beside column and stools and the 40 mm of the edges above the gravel."""
    top = 2 * FOOT_X * 2 * FOOT_Z - (2 * COL_B - 0.02) ** 2 - 2 * math.pi * (STOOL_R - 0.01) ** 2
    edges = 2 * (2 * FOOT_X + 2 * FOOT_Z) * FOOT_Y1
    return round((top + edges) / A["footing"], 3)


def exposed_top():
    """☀️ Share of the top slab area open to the air: rim face, edges and the underside beside the column."""
    chip = 0.5 * CHIP**2
    rim = (2 * TOP_B) ** 2 - chip - (2 * BOARD) ** 2
    per = 8 * TOP_B - 2 * CHIP + CHIP * math.sqrt(2)
    under = (2 * TOP_B) ** 2 - chip - (2 * COL_B) ** 2
    return round((rim + per * (TOP_Y1 - TOP_Y0) + under) / A["table-top"], 3)


TILE_EXP = round(32 * SQ * SQ / A["board-dark"], 3)
COL_EXP = round((A["column"] - 2 * ((2 * COL_B) ** 2 - 2 * COL_CH**2)) / A["column"], 3)
STOOL_EXP = round((A["stools"] - 2 * math.pi * STOOL_R**2) / A["stools"], 3)
SOIL = soil_contact()


def thermal_binding(kind):
    """🏃️ Region bindings of the three named poses (summer afternoon under plane trees, dappled shade 0.6)."""
    if kind == "in-service":
        regs = {"table-top": rb(exposed_top(), 0.2), "board-dark": rb(TILE_EXP, 0.17), "board-light": rb(TILE_EXP, 0.17), "column": rb(COL_EXP, 0.12), "stools": rb(STOOL_EXP, 0.2), "footing": rb(exposed_footing(), 0.15, (SOIL, 18))}
    elif kind == "maintenance":
        regs = {"table-top": rb(exposed_top(), 0.2), "board-dark": rb(TILE_EXP, 0.17), "board-light": rb(TILE_EXP, 0.17), "column": rb(round(COL_EXP + 0.1, 3), 0.12), "stools": rb(STOOL_EXP, 0.2), "footing": rb(round(exposed_footing() + 0.016, 3), 0.15, (SOIL, 18))}
    else:
        regs = {"table-top": rb(round(exposed_top() - 0.03, 3), 0.1, (0.03, 31)), "board-dark": rb(TILE_EXP, 0.08), "board-light": rb(TILE_EXP, 0.08), "column": rb(COL_EXP, 0.04), "stools": rb(round(STOOL_EXP - 0.155, 3), 0.06, (0.155, 31)), "footing": rb(round(exposed_footing() - 0.03, 3), 0.08, (round(SOIL + 0.03, 3), 18))}
    return {"regions": regs, "emitters": {}}


PLAYER = 80 * G


def player_loads():
    """🧍️ The two seated players: 75 % of the body weight on the seat, 15 % through the feet on the footing, 10 % through the forearms on the table edge."""
    out = []
    for side, en, de in (("white", "White", "Weiß"), ("black", "Black", "Schwarz")):
        out.append({"id": f"seat-{side}", "kind": "occupant", "title": T(f"{en} seated, 75 % of 80 kg on the stool", f"{de} sitzend, 75 % von 80 kg auf dem Hocker"), "anchor": f"seat-{side}", "fxN": 0, "fyN": r6(-0.75 * PLAYER), "source": "authored-estimate", "reference": "din-33402-2-2020"})
        out.append({"id": f"feet-{side}", "kind": "occupant", "title": T(f"{en}'s feet, 15 % of 80 kg on the footing", f"Füße von {de}, 15 % von 80 kg auf dem Fundament"), "anchor": f"feet-{side}", "fxN": 0, "fyN": r6(-0.15 * PLAYER), "source": "authored-estimate", "reference": "din-33402-2-2020"})
        out.append({"id": f"forearms-{side}", "kind": "occupant", "title": T(f"{en}'s forearms, 10 % of 80 kg on the table edge", f"Unterarme von {de}, 10 % von 80 kg auf der Tischkante"), "anchor": f"elbow-{side}", "fxN": 0, "fyN": r6(-0.10 * PLAYER), "source": "authored-estimate", "reference": "din-33402-2-2020"})
    return out


SOIL_CONTACTS = ["soil-white", "soil-black"]


def sling_loads():
    """🏗️ The crane hook as applied lifting forces: each round sling carries half the weight of column and top (the lifting points lie symmetric about their common centre of mass)."""
    half = r6((round(M["column"], 2) + round(M["top"], 2)) * G / 2)
    return [{"id": f"sling-{side}", "kind": "equipment", "title": T(f"Round sling on {en}'s side, half of the lifted table", f"Rundschlinge auf der Seite von {de}, die Hälfte des gehobenen Tisches"), "anchor": f"lift-{side}", "fxN": 0, "fyN": half, "source": "authored-estimate", "reference": "en-1492-2-2000"} for side, en, de in (("white", "White", "Weiß"), ("black", "Black", "Schwarz"))]



POSES = [
    {"id": "in-service", "title": T("In service", "Im Einsatz"), "description": T("The set as it stands in the park after years of use: empty, on its footing in water-bound gravel, under the dappled shade of plane trees on a summer afternoon.", "Das Ensemble, wie es nach Jahren im Park steht: leer, auf seinem Fundament in wassergebundener Decke, im lichten Schatten von Platanen an einem Sommernachmittag."), "bones": {}, "ground": "authored", "contacts": SOIL_CONTACTS, "thermal": thermal_binding("in-service"), "loads": []},
    {"id": "maintenance", "title": T("Re-levelling", "Neu ausrichten"), "description": T("The depot crew has lifted the table (column and top, 249 kg) 60 mm off its washed-out mortar bed on two round slings under the overhang to re-bed it level; footing and stools stay.", "Der Bauhof hat den Tisch (Säule und Platte, 249 kg) an zwei Rundschlingen unter dem Überstand 60 mm aus dem ausgewaschenen Mörtelbett gehoben, um ihn waagerecht neu zu betten; Fundament und Hocker bleiben."), "bones": {"column": {"dy": 0.06}}, "ground": "authored", "contacts": SOIL_CONTACTS, "thermal": thermal_binding("maintenance"), "loads": sling_loads()},
    {"id": "in-use", "title": T("Two players", "Zwei Spieler"), "description": T("Two adults of 80 kg play a game: each sits on a stool, feet on the footing, forearms on the table edge; their bodies shade the board and warm the seats.", "Zwei Erwachsene mit 80 kg spielen eine Partie: jeder sitzt auf einem Hocker, die Füße auf dem Fundament, die Unterarme auf der Tischkante; ihre Körper beschatten das Brett und wärmen die Sitze."), "bones": {}, "ground": "authored", "contacts": SOIL_CONTACTS, "thermal": thermal_binding("in-use"), "loads": player_loads()},
    {"id": "slings-taut", "key": True, "title": T("Slings taut", "Schlingen gespannt"), "description": T("Key of the re-levelling clip: the slings carry the table before it leaves its mortar bed; the soil carries footing and stools.", "Schlüssel des Ausricht-Clips: die Schlingen tragen den Tisch, bevor er sein Mörtelbett verlässt; der Boden trägt Fundament und Hocker."), "bones": {}, "ground": "authored", "contacts": SOIL_CONTACTS, "thermal": thermal_binding("in-service"), "loads": sling_loads()},
]

CLIPS = [
    {
        "id": "relevel",
        "title": T("Re-level the table", "Tisch neu ausrichten"),
        "description": T("The slings take the load, the table rises 60 mm in 3 s, hangs while the bed is cleaned and fresh mortar is spread, and is lowered level onto it.", "Die Schlingen nehmen die Last, der Tisch hebt sich in 3 s um 60 mm, hängt, während das Bett gereinigt und frischer Mörtel aufgezogen wird, und wird waagerecht darauf abgesenkt."),
        "durationS": 9,
        "fps": 30,
        "loop": False,
        "rootMotion": "none",
        "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 0.5, "pose": "slings-taut", "ease": "smooth"}, {"t": 3, "pose": "maintenance", "ease": "step"}, {"t": 6, "pose": "maintenance", "ease": "smooth"}, {"t": 8.5, "pose": "slings-taut", "ease": "smooth"}, {"t": 9, "pose": "in-service", "ease": "smooth"}],
        "contacts": [{"anchor": "soil-white", "fromS": 0, "toS": 9}, {"anchor": "soil-black", "fromS": 0, "toS": 9}],
        "phases": [{"id": "lift", "fromS": 0, "toS": 3, "title": T("Lift", "Heben")}, {"id": "re-bed", "fromS": 3, "toS": 6, "title": T("Re-bed", "Neu betten")}, {"id": "lower", "fromS": 6, "toS": 9, "title": T("Lower", "Absenken")}],
        "events": [{"t": 0.5, "kind": "load-on", "title": T("Slings tensioned", "Schlingen gespannt"), "target": "sling-white"}, {"t": 3, "kind": "motion", "title": T("Table 60 mm up", "Tisch 60 mm hoch")}, {"t": 6, "kind": "motion", "title": T("Lowering onto fresh mortar", "Absenken auf frischen Mörtel")}, {"t": 8.5, "kind": "load-off", "title": T("Slings slack", "Schlingen locker"), "target": "sling-white"}],
        "tracks": [{"target": "region:column:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": COL_EXP}, {"t": 3, "v": round(COL_EXP + 0.1, 3)}, {"t": 6, "v": round(COL_EXP + 0.1, 3)}, {"t": 9, "v": COL_EXP}]}],
    },
]
CLIPS.append({
    "id": "players-sit-down",
    "title": T("Players sit down", "Spieler setzen sich"),
    "description": T("White sits down first, Black two seconds later; both lean their forearms on the table edge, and their bodies shade board and seats.", "Weiß setzt sich zuerst, Schwarz zwei Sekunden später; beide stützen die Unterarme auf die Tischkante, und ihre Körper beschatten Brett und Sitze."),
    "durationS": 6,
    "fps": 30,
    "loop": False,
    "rootMotion": "none",
    "keys": [{"t": 0, "pose": "in-use", "ease": "linear"}, {"t": 6, "pose": "in-use", "ease": "linear"}],
    "contacts": [{"anchor": "soil-white", "fromS": 0, "toS": 6}, {"anchor": "soil-black", "fromS": 0, "toS": 6}],
    "phases": [{"id": "white-sits", "fromS": 0, "toS": 2, "title": T("White sits", "Weiß setzt sich")}, {"id": "black-sits", "fromS": 2, "toS": 4, "title": T("Black sits", "Schwarz setzt sich")}, {"id": "play", "fromS": 4, "toS": 6, "title": T("First moves", "Erste Züge")}],
    "events": [{"t": 1.2, "kind": "load-on", "title": T("White seated", "Weiß sitzt"), "target": "seat-white"},{"t": 3.2, "kind": "load-on", "title": T("Black seated", "Schwarz sitzt"), "target": "seat-black"}],
    "tracks": [
        {"target": "load:seat-white", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1.2, "v": 1}]},
        {"target": "load:feet-white", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1.2, "v": 1}]},
        {"target": "load:forearms-white", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1.2, "v": 0}, {"t": 4.5, "v": 1}]},
        {"target": "load:seat-black", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 2, "v": 0}, {"t": 3.2, "v": 1}]},
        {"target": "load:feet-black", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 2, "v": 0}, {"t": 3.2, "v": 1}]},
        {"target": "load:forearms-black", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 3.2, "v": 0}, {"t": 5, "v": 1}]},
        {"target": "region:stools:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.2}, {"t": 3.2, "v": 0.06}]},
        {"target": "region:board-dark:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.17}, {"t": 5, "v": 0.08}]},
        {"target": "region:board-light:sunlitFraction", "ease": "smooth", "keys": [{"t": 0, "v": 0.17}, {"t": 5, "v": 0.08}]},
    ],
})


def bone(id_, parent, en, de, length, joint, mass, comx, comy, section, ref):
    """🦴️ One bone, placed from the world rest frames of BONES."""
    ox, oy = BONES[id_]["o"]
    a = BONES[id_]["a"]
    if parent is None:
        x, y, ang = ox, oy, a
    else:
        px, py = BONES[parent]["o"]
        pa = BONES[parent]["a"]
        dx, dy = ox - px, oy - py
        c, s = math.cos(math.radians(-pa)), math.sin(math.radians(-pa))
        x, y, ang = dx * c - dy * s, dx * s + dy * c, a - pa
    b = {"id": id_, "parent": parent, "title": T(en, de), "x": r6(x), "y": r6(y), "angleDeg": ang, "lengthM": length, "joint": joint, "mass": {"kg": round(mass, 2), "comX": comx, "comY": comy, "distribution": "point", "source": "authored-estimate", "reference": ref}, "section": section}
    return b


def sec(area, i, e, mat, en, de):
    """📏️ One member section."""
    return {"areaM2": r6(area), "secondMomentM4": float(f"{i:.6g}"), "extremeFibreM": e, "material": mat, "title": T(en, de)}


RIG = {
    "kind": "structural",
    "root": "footing",
    "bones": [
        bone("footing", None, "Concrete footing slab with the mortar beds", "Betonfundamentplatte mit den Mörtelbetten", 2 * FOOT_X, {"type": "fixed"}, M["footing"], FOOT_X, 0.1, sec(0.18, 0.9 * 0.2**3 / 12, 0.1, "concrete-weathered", "Slab 900 x 200 mm", "Platte 900 x 200 mm"), "en-1991-1-1-2002"),
        bone("column", "footing", "Granite column, lifted in maintenance", "Granitsäule, bei der Wartung gehoben", COL_H, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.15}, M["column"], COL_H / 2, 0.0, sec(0.0898, 0.3**4 / 12, 0.15, "granite-grey", "Granite 300 x 300 mm, 10 mm arrises", "Granit 300 x 300 mm, 10-mm-Fasen"), "din-en-1936-2007"),
        bone("top", "column", "Granite top with the inlaid board", "Granitplatte mit eingelegtem Brett", 2 * TOP_B, {"type": "fixed"}, M["top"], TOP_B, 0.04, sec(0.7 * 0.08, 0.7 * 0.08**3 / 12, 0.04, "granite-grey", "Slab 700 x 80 mm", "Platte 700 x 80 mm"), "din-en-1936-2007"),
        bone("stool-white", "footing", "White's granite stool", "Granithocker von Weiß", STOOL_H, {"type": "fixed"}, M["stool"], STOOL_H / 2, 0.0, sec(math.pi * STOOL_R**2, math.pi * (2 * STOOL_R) ** 4 / 64, STOOL_R, "granite-grey", "Granite cylinder 360 mm", "Granitzylinder 360 mm"), "din-en-1936-2007"),
        bone("stool-black", "footing", "Black's granite stool", "Granithocker von Schwarz", STOOL_H, {"type": "fixed"}, M["stool"], STOOL_H / 2, 0.0, sec(math.pi * STOOL_R**2, math.pi * (2 * STOOL_R) ** 4 / 64, STOOL_R, "granite-grey", "Granite cylinder 360 mm", "Granitzylinder 360 mm"), "din-en-1936-2007"),
    ],
}


def anchor(id_, bone_, x, y, z, kind, en, de, **kw):
    """🪝️ One anchor from a world rest point."""
    u, v = loc(bone_, x, y)
    a = {"id": id_, "bone": bone_, "x": u, "y": v}
    if z:
        a["z"] = z
    a.update({"kind": kind, "title": T(en, de)})
    a.update(kw)
    return a


ANCHORS = [
    anchor("soil-white", "footing", -0.5, FOOT_Y0, 0, "support", "Soil bearing under White's half of the footing", "Bodenpressung unter der Fundamenthälfte von Weiß", support="contact", normalDeg=90, footprintLengthM=1.0, footprintWidthM=0.9, frictionCoefficient=0.6),
    anchor("soil-black", "footing", 0.5, FOOT_Y0, 0, "support", "Soil bearing under Black's half of the footing", "Bodenpressung unter der Fundamenthälfte von Schwarz", support="contact", normalDeg=90, footprintLengthM=1.0, footprintWidthM=0.9, frictionCoefficient=0.6),
    anchor("lift-white", "top", -0.25, TOP_Y0, 0, "load", "Round sling under the overhang on White's side", "Rundschlinge unter dem Überstand auf der Seite von Weiß"),
    anchor("lift-black", "top", 0.25, TOP_Y0, 0, "load", "Round sling under the overhang on Black's side", "Rundschlinge unter dem Überstand auf der Seite von Schwarz"),
    anchor("seat-white", "stool-white", -STOOL_X, FOOT_Y1 + BED + STOOL_H, 0, "load", "White's seat", "Sitz von Weiß"),
    anchor("seat-black", "stool-black", STOOL_X, FOOT_Y1 + BED + STOOL_H, 0, "load", "Black's seat", "Sitz von Schwarz"),
    anchor("feet-white", "footing", -0.42, FOOT_Y1, 0, "load", "White's feet on the footing", "Füße von Weiß auf dem Fundament"),
    anchor("feet-black", "footing", 0.42, FOOT_Y1, 0, "load", "Black's feet on the footing", "Füße von Schwarz auf dem Fundament"),
    anchor("elbow-white", "top", -0.32, TOP_Y1, 0, "load", "White's forearms on the table edge", "Unterarme von Weiß auf der Tischkante"),
    anchor("elbow-black", "top", 0.32, TOP_Y1, 0, "load", "Black's forearms on the table edge", "Unterarme von Schwarz auf der Tischkante"),
]

REFERENCES = [
    {"id": "en-1492-2-2000", "citation": "EN 1492-2:2000+A1:2008 Textile slings - Safety - Part 2: Roundslings made of man-made fibres for general purpose use (working load limit 1 t for the violet roundsling, mode factor 0.8 choked)."},
    {"id": "din-en-1936-2007", "citation": "DIN EN 1936:2007 Natural stone test methods - Determination of real density and apparent density, and of total and open porosity (apparent density of granites about 2600 to 2700 kg/m3, of gabbros about 2900 to 3000 kg/m3)."},
    {"id": "en-12372-2006", "citation": "EN 12372:2006 Natural stone test methods - Determination of flexural strength under concentrated load (granite typically 10 to 20 MPa)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (natural stone: granite 2.8 W/(m K), 1000 J/(kg K); concrete 2400 kg/m3: 2.0 W/(m K); cement mortar)."},
    {"id": "en-1991-1-1-2002", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: Densities, self-weight, imposed loads for buildings (Annex A: normal-weight concrete 24 kN/m3, cement mortar 19 to 23 kN/m3)."},
    {"id": "en-1992-1-1-2004", "citation": "EN 1992-1-1:2004 Eurocode 2: Design of concrete structures - Part 1-1 (C25/30: fck 25 MPa, fctm 2.6 MPa, Ecm 31 GPa)."},
    {"id": "en-206-2013", "citation": "EN 206:2013+A2:2021 Concrete - Specification, performance, production and conformity (exposure class XF1 for an exposed footing slab)."},
    {"id": "en-581-1-2017", "citation": "EN 581-1:2017 Outdoor furniture - Seating and tables for camping, domestic and contract use - Part 1: General safety requirements."},
    {"id": "en-581-3-2017", "citation": "EN 581-3:2017 Outdoor furniture - Seating and tables for camping, domestic and contract use - Part 3: Mechanical safety requirements and test methods for tables."},
    {"id": "en-527-1-2011", "citation": "EN 527-1:2011 Office furniture - Work tables and desks - Part 1: Dimensions (fixed table height 740 mm, used as the ergonomic reference for a seated board game)."},
    {"id": "din-33402-2-2020", "citation": "DIN 33402-2:2020 Ergonomics - Human body dimensions - Part 2: Values (popliteal height of adults about 0.40 to 0.46 m with shoes; body mass of adults)."},
    {"id": "fide-handbook-c02", "citation": "FIDE Handbook C.02 Standards of Chess Equipment, Tournament Venue and Chess Clocks (side of a square 5 to 6 cm; dark square at each player's left-hand corner)."},
    {"id": "en-10088-3-2014", "citation": "EN 10088-3:2014 Stainless steels - Part 3: Technical delivery conditions for semi-finished products, bars, rods (1.4401, A4)."},
    {"id": "en-1492-2-2008", "citation": "EN 1492-2:2000+A1:2008 Textile slings - Safety - Part 2: Roundslings made of man-made fibres for general purpose use."},
    {"id": "incropera-2011", "citation": "Bergman, T. L., Lavine, A. S., Incropera, F. P., DeWitt, D. P. (2011). Fundamentals of Heat and Mass Transfer, 7th ed., Wiley (emissivity of stone and concrete about 0.9)."},
    {"id": "berdahl-bretz-1997", "citation": "Berdahl, P., Bretz, S. E. (1997). Preliminary survey of the solar reflectance of cool roofing materials. Energy and Buildings 25(2), 149-158 (weathering and soiling lower the solar reflectance of light mineral surfaces, so weathered absorptance is set above new)."},
]

SUMMARY = T(
    "A used granite park chess table: a 700 x 80 mm light grey granite top with an inlaid board of 64 dark and light 50 mm squares on a 300 mm granite column, two 360 mm granite stools at 0.45 m seat height, all on a 2.00 x 0.90 x 0.20 m concrete footing that shows 40 mm above the water-bound gravel. Years in the park left a broken corner on the top, washed-out mortar beds, a dull worn centre of the board, pigeon droppings, trodden gum, algae on the north faces and a tag on the column. The depot lifts the table 60 mm on two round slings to re-bed it level; two players of 80 kg sit, feet on the footing, forearms on the edge.",
    "Ein gebrauchter Parkschachtisch aus Granit: eine hellgraue Granitplatte 700 x 80 mm mit eingelegtem Brett aus 64 dunklen und hellen Feldern zu 50 mm auf einer Granitsäule 300 mm, zwei Granithocker 360 mm mit 0,45 m Sitzhöhe, alles auf einem Betonfundament 2,00 x 0,90 x 0,20 m, das 40 mm aus der wassergebundenen Decke ragt. Die Jahre im Park hinterließen eine abgeschlagene Plattenecke, ausgewaschene Mörtelbetten, eine stumpf abgespielte Brettmitte, Taubenkot, festgetretenen Kaugummi, Algen an den Nordseiten und ein Tag an der Säule. Der Bauhof hebt den Tisch an zwei Rundschlingen 60 mm an, um ihn waagerecht neu zu betten; zwei Spieler mit 80 kg sitzen, die Füße auf dem Fundament, die Unterarme auf der Kante.",
)

THERMAL_ASSUMPTIONS = [
    T("All six regions are passive: solar gain on the sunlit share, long-wave exchange with sky and ground, forced convection; the stones are thick and slow (77 to 167 kJ/(m2 K) per outer area), so the summer afternoon state lags the sun by hours.", "Alle sechs Regionen sind passiv: Solargewinn auf dem besonnten Anteil, langwelliger Austausch mit Himmel und Boden, erzwungene Konvektion; die Steine sind dick und träge (77 bis 167 kJ/(m2 K) je Außenfläche), daher hinkt der Sommernachmittag der Sonne um Stunden hinterher."),
    T("Weathered surfaces are darker than new ones (authored estimates after Berdahl and Bretz): flamed light grey granite 0.66 instead of about 0.55 when new, soiled concrete with algae 0.72 instead of about 0.6; the polished dark squares absorb 0.88 and become the hot part of the board in sun, the light squares 0.5.", "Verwitterte Flächen sind dunkler als neue (Schätzungen nach Berdahl und Bretz): geflammter hellgrauer Granit 0,66 statt etwa 0,55 im Neuzustand, verschmutzter Beton mit Algen 0,72 statt etwa 0,6; die polierten dunklen Felder absorbieren 0,88 und werden in der Sonne der heiße Teil des Bretts, die hellen Felder 0,5."),
    T("The footing exchanges with the gravel and soil over its underside and its buried 160 mm (contact share at 18 degC); the players warm the seats and the table edge by contact at 31 degC. Areas are the total outer areas of the 1:50 solids; exposure and contact belong to the poses.", "Das Fundament tauscht über die Unterseite und die eingegrabenen 160 mm mit Kies und Boden (Kontaktanteil bei 18 degC); die Spieler wärmen Sitze und Tischkante durch Kontakt bei 31 degC. Flächen sind die gesamten Außenflächen der 1:50-Körper; Exposition und Kontakt gehören zu den Posen."),
    T("No element draws power: the table top states passive 0 W.", "Kein Element nimmt Leistung auf: die Tischplatte gibt passiv 0 W an."),
]

STRUCTURE_ASSUMPTIONS = [
    T("In service the top (104 kg) is dowelled with a stainless A4 dowel and bedded on the column (145 kg), the column and the two stools (108 kg each) are dowelled and bedded in 10 mm cement mortar on the footing (869 kg), and the footing bears on the gravel and soil; the bearing pressure is idealised as two contact supports under the two halves of the slab.", "Im Einsatz ist die Platte (104 kg) mit einem Edelstahldollen A4 auf die Säule (145 kg) gedollt und gebettet, Säule und die zwei Hocker (je 108 kg) sind in 10 mm Zementmörtel auf das Fundament (869 kg) gedollt und gebettet, und das Fundament liegt auf Kies und Boden; die Bodenpressung ist als zwei Kontaktauflager unter den zwei Plattenhälften idealisiert."),
    T("Granite works in compression in the column and stools and in bending only in the 200 mm overhang of the top (700 x 80 mm, flexural strength about 12 MPa per EN 12372): the 78 N forearm load at the edge is far from critical; the footing (C25/30) spreads 1.33 t over 1.8 m2, about 7 kPa.", "Der Granit wirkt in Säule und Hockern auf Druck und nur im 200-mm-Überstand der Platte (700 x 80 mm, Biegezugfestigkeit etwa 12 MPa nach EN 12372) auf Biegung: die 78-N-Unterarmlast an der Kante ist weit von kritisch; das Fundament (C25/30) verteilt 1,33 t auf 1,8 m2, etwa 7 kPa."),
    T("In the re-levelling pose the table hangs from two polyester round slings under the overhang and the footing and the stools stay on the soil: the crane hook is applied as its lifting force, 1.22 kN per sling and together the 2.44 kN weight of column and top, because supports of the archive are springs of equal stiffness that would share the whole 13.1 kN between slings and soil; the soil contacts then carry the remaining 10.6 kN of footing and stools; in use each player of 80 kg puts 75 % on the seat, 15 % through the feet on the footing and 10 % through the forearms on the table edge (authored estimate after DIN 33402-2 body masses, EN 581 for outdoor seating and tables).", "In der Pose Neu ausrichten hängt der Tisch an zwei Polyester-Rundschlingen unter dem Überstand und Fundament und Hocker bleiben auf dem Boden: der Kranhaken ist als seine Hubkraft angesetzt, 1,22 kN je Schlinge und zusammen die 2,44 kN Gewicht von Säule und Platte, weil Auflager des Archivs Federn gleicher Steifigkeit sind, die die ganzen 13,1 kN auf Schlingen und Boden verteilen würden; die Bodenkontakte tragen dann die übrigen 10,6 kN von Fundament und Hockern; im Gebrauch gibt jeder Spieler mit 80 kg 75 % auf den Sitz, 15 % über die Füße auf das Fundament und 10 % über die Unterarme auf die Tischkante (Schätzung nach Körpermassen DIN 33402-2, EN 581 für Sitzmöbel und Tische im Freien)."),
]


def document():
    """📄️ The whole asset document."""
    asset = {
        "id": SLUG,
        "category": "furniture",
        "title": T("Used Park Chess Table", "Gebrauchter Parkschachtisch"),
        "summary": SUMMARY,
        "tags": ["used", "park", "public-space", "seating", "dining", "game", "chess", "street-furniture", "granite", "natural-stone", "concrete", "footing", "table", "stool"],
        "provenance": {
            "author": "Semio staffage content (used park objects wave)",
            "method": "handcrafted",
            "created": "2026-11-22",
            "references": REFERENCES,
            "notes": T(
                "Catalogue-typical municipal granite chess table set (type, no manufacturer): table 0.74 m high with a 0.70 m square top, stools 0.45 m, squares 50 mm; masses from volume and density, footing included. Nothing is measured; dimensions are literature and authored estimates, the traces of use are illustrative.",
                "Katalogtypisches kommunales Granit-Schachtisch-Ensemble (Typ, kein Hersteller): Tisch 0,74 m hoch mit quadratischer Platte 0,70 m, Hocker 0,45 m, Felder 50 mm; Massen aus Volumen und Dichte, Fundament eingeschlossen. Nichts ist gemessen; Abmessungen sind Literatur und Schätzungen, die Gebrauchsspuren sind illustrativ.",
            ),
        },
        "dimensions": {"heightM": r6(TOP_Y1 - FOOT_Y0), "widthM": r6(2 * FOOT_X), "depthM": r6(2 * FOOT_Z), "massKg": round(M["total"], 1)},
        "materials": MATERIALS,
        "rig": RIG,
        "volumes": volumes(),
        "anchors": ANCHORS,
        "representations": {str(s): representation(s) for s in (500, 200, 100, 50)},
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": {"environment": ENV, "passengers": [], "operating": [], "regions": REGIONS, "emitters": [], "assumptions": THERMAL_ASSUMPTIONS},
        "structure": {"gravityMS2": G, "dynamics": "quasi-static", "supports": SOIL_CONTACTS, "assumptions": STRUCTURE_ASSUMPTIONS, "clinical": False},
        "presentation": {"defaultPose": "in-service", "defaultClip": "relevel", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(document(), f, ensure_ascii=False, indent=None, separators=(", ", ": "))
        f.write("\n")
    print("[DEBUG] masses", {k: round(v, 2) for k, v in M.items()})
    print("[DEBUG] areas", {k: round(v, 4) for k, v in A.items()})
