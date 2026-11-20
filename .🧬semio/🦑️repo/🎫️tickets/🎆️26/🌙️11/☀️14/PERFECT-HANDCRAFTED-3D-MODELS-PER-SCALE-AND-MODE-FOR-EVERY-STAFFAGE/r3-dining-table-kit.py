"""🪵 Round 3, furniture/dining-table: the measures of the real table (solid oak, 1.60 x 0.90 x 0.75 m, two halves on synchronised steel slides, a 0.60 m leaf stored under the top in a 30 mm shadow gap) and the helpers that turn world coordinates into bone-local solids.

World frame: x from the left end to the right end, y up, z lateral (left of the asset is +). Every measure is metres. The right half is the mirror of the left half about x = 0.80.
"""
import math

LENGTH, HALF, DEPTH, HEIGHT = 1.6, 0.8, 0.9, 0.75
TOP_Y0, TOP_Y1 = 0.72, 0.75
CREASE_Y = 0.735
BEVEL = 0.015
GAP_Y0 = 0.69
APRON_Y0, APRON_Y1 = 0.61, 0.69
APRON_T = 0.025
APRON_Z0, APRON_Z1 = 0.375, 0.40
END_X0, END_X1 = 0.11, 0.135
LEG_X, LEG_Z = 0.135, 0.375
LEG_TOP, LEG_FOOT = 0.035, 0.0225
TAPER_Y = 0.58
TENON = 0.02
BLOCK_X = (0.25, 0.40)
BLOCK_L = 0.06
BLOCK_Z0, BLOCK_Z1 = 0.365, 0.39
END_BLOCK_Z = 0.15
RUN_Y0, RUN_Y1 = 0.665, 0.69
RUN_Z0, RUN_Z1 = 0.363, 0.375
RUN_X0, RUN_X1 = 0.17, 0.79
WALL = 0.002
CENTRE_X0, CENTRE_X1 = 0.3, 1.3
CENTRE_Y0, CENTRE_Y1 = 0.667, 0.688
CENTRE_Z0, CENTRE_Z1 = 0.365, 0.373
LEAF_X0, LEAF_X1 = 0.5, 1.1
LEAF_T = 0.03
STAVES = (-0.27, -0.09, 0.09, 0.27)
SLAB_Y0, FRAME500_Y0 = 0.675, 0.60
LEAF500_Z = 0.39

FRAME_Y = 0.6775
ORIGIN = {"ground": (0.0, 0.0), "frame": (CENTRE_X0, FRAME_Y), "top-left": (0.0, CREASE_Y), "top-right": (HALF, CREASE_Y), "leaf": (LEAF_X0, GAP_Y0)}

OAK, STEEL = "oak-timber", "steel-slides"
TONE = {OAK: 0.7, STEEL: 0.35}
SIDES = (("l", 1), ("r", -1))
HALVES = (("left", "top-left", 1), ("right", "top-right", -1))


def r6(value):
    """🔢 Rounds to a hundredth of a millimetre so the written numbers stay readable."""
    return round(value + 0.0, 5)


def wx(x, hand):
    """🪞 A world x of the left half carried to the half `hand` (+1 left, -1 right, mirrored about the centre joint)."""
    return x if hand == 1 else LENGTH - x


def span(a, b, hand):
    """↔ The world x range of a left-half range [a, b] on the half `hand`, ordered."""
    p, q = wx(a, hand), wx(b, hand)
    return (min(p, q), max(p, q))


def base(sid, kind, bone, material, tone=None, role="body"):
    """🏷 The common members of a solid."""
    return {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": TONE[material] if tone is None else tone}


def box(sid, bone, material, x, y, z, radius=0.0, tone=None, role="body", **more):
    """📦 A box between world extents x, y, z (each a pair)."""
    ox, oy = ORIGIN[bone]
    out = base(sid, "box", bone, material, tone, role)
    out["size"] = [r6(x[1] - x[0]), r6(y[1] - y[0]), r6(z[1] - z[0])]
    if radius:
        out["cornerRadius"] = radius
    out["transform"] = {"translation": [r6((x[0] + x[1]) / 2 - ox), r6((y[0] + y[1]) / 2 - oy), r6((z[0] + z[1]) / 2)]}
    out.update(more)
    return out


def along_x(sid, bone, material, profile_yz, x, tone=None, role="body", **more):
    """➡ A prism along the table: a world profile of (y, z) points pushed from x[0] to x[1]."""
    ox, oy = ORIGIN[bone]
    out = base(sid, "extrude", bone, material, tone, role)
    out.update({"profile": [[r6(y - oy), r6(z)] for y, z in profile_yz], "plane": "yz", "from": r6(x[0] - ox), "to": r6(x[1] - ox)})
    out.update(more)
    return out


def plate(sid, bone, material, pivot, profile_zx, y, taper=None, tone=None, role="body", **more):
    """⬆ A plate standing on the plan: a profile of (z, x) points relative to the world `pivot` (x, z), pushed up from y[0] to y[1]; `taper` scales the profile about the pivot at y[1] (a bevel)."""
    ox, oy = ORIGIN[bone]
    out = base(sid, "extrude", bone, material, tone, role)
    out.update({"profile": [[r6(z), r6(x)] for z, x in profile_zx], "plane": "zx", "from": r6(y[0] - oy), "to": r6(y[1] - oy)})
    if taper:
        out["taperTo"] = [r6(taper[0]), r6(taper[1])]
    out["transform"] = {"translation": [r6(pivot[0] - ox), 0, r6(pivot[1])]}
    out.update(more)
    return out


def leg(sid, bone, x, z, sections, exponent=10, tone=None, **more):
    """🦵 A square leg as a superelliptic loft through world (y, half width) stations at the plan centre (x, z)."""
    ox, oy = ORIGIN[bone]
    out = base(sid, "loft", bone, OAK, tone)
    out["sections"] = [{"at": [r6(x - ox), r6(y - oy), r6(z)], "rx": r6(h), "ry": r6(h), "exponent": exponent, "rotation": [0.707107, 0.0, 0.0, 0.707107]} for y, h in sections]
    out["capStart"] = True
    out["capEnd"] = True
    out.update(more)
    return out


def turned(sid, bone, material, profile, axis, origin, tone=None, role="body", **more):
    """⭕ A revolve of an (axial, radial) profile about a bone axis through a world origin."""
    ox, oy = ORIGIN[bone]
    out = base(sid, "revolve", bone, material, tone, role)
    out.update({"profile": [[r6(a), r6(r)] for a, r in profile], "axis": {"u": [1, 0, 0], "v": [0, 1, 0], "w": [0, 0, 1]}[axis], "origin": [r6(origin[0] - ox), r6(origin[1] - oy), r6(origin[2])]})
    out.update(more)
    return out


def ellipsoid(sid, bone, material, centre, radii, tone=None, role="body", **more):
    """🥚 An ellipsoid at a world centre."""
    ox, oy = ORIGIN[bone]
    out = base(sid, "ellipsoid", bone, material, tone, role)
    out["radii"] = [r6(r) for r in radii]
    out["transform"] = {"translation": [r6(centre[0] - ox), r6(centre[1] - oy), r6(centre[2])]}
    out.update(more)
    return out


def repeat(solid, count, du=0.0, dv=0.0, dw=0.0):
    """🔁 Copies a solid `count` times along a step."""
    solid["repeat"] = {"count": count, "translation": [r6(du), r6(dv), r6(dw)]}
    return solid


def tube(side):
    """🛤 The cross-section (y, z) of the steel runner on the long apron of side `side` (+1 left, -1 right): a 25 x 12 mm rectangular tube of 2 mm sheet against the apron, as outline and bore."""
    outer = [(RUN_Y0, RUN_Z0), (RUN_Y0, RUN_Z1), (RUN_Y1, RUN_Z1), (RUN_Y1, RUN_Z0)]
    bore = [(RUN_Y0 + WALL, RUN_Z0 + WALL), (RUN_Y0 + WALL, RUN_Z1 - WALL), (RUN_Y1 - WALL, RUN_Z1 - WALL), (RUN_Y1 - WALL, RUN_Z0 + WALL)]
    flip = lambda pts: [(y, side * z) for y, z in pts] if side == 1 else list(reversed([(y, side * z) for y, z in pts]))
    return flip(outer), flip(bore)


def top_outline(hand, inset):
    """⬜ The plan of a top half relative to its joint centre (z, x): square at the joint, set in by `inset` on the three free edges."""
    a = HALF - inset
    zz = DEPTH / 2 - inset
    s = -hand
    pts = [(-zz, 0.0), (zz, 0.0), (zz, s * a), (-zz, s * a)]
    return pts if hand == -1 else list(reversed(pts))


def mass_of(solids, density):
    """⚖ Mass and centre (x, y) of boxes and extrudes by their nominal volume; lofts by station frusta (used for the rig masses)."""
    total, mx, my = 0.0, 0.0, 0.0
    for kg, cx, cy in solids:
        total += kg
        mx += kg * cx
        my += kg * cy
    return total, mx / total, my / total


def frustum(h, a, b):
    """📐 Volume of a square frustum of height h between side a and side b."""
    return h * (a * a + a * b + b * b) / 3


def rig(old):
    """🦴 The corrected rig: the halves slide in the plane of the top (mid plate, y 0.735), the slide frame lies on the slide runners (x 0.30, y 0.6775), the floor reference carries no length; masses and centres from the modelled oak and steel."""
    m = masses()
    bones = []
    for bone in old["bones"]:
        bone = dict(bone)
        if bone["id"] == "ground":
            bone["lengthM"] = 0
        if bone["id"] == "frame":
            bone.update(x=CENTRE_X0, y=FRAME_Y)
            bone["mass"] = dict(bone["mass"], kg=m["frame"][0], comX=r6(m["frame"][1] - CENTRE_X0), comY=r6(m["frame"][2] - FRAME_Y))
        if bone["id"] in ("top-left", "top-right"):
            ox, oy = ORIGIN[bone["id"]]
            kg, cx, cy = m["half"]
            if bone["id"] == "top-right":
                cx = LENGTH - cx
            bone["y"] = CREASE_Y
            bone["mass"] = dict(bone["mass"], kg=kg, comX=r6(round(cx - ox, 3)), comY=r6(round(cy - oy, 3)))
        if bone["id"] == "leaf":
            bone["mass"] = dict(bone["mass"], kg=m["leaf"][0])
        bones.append(bone)
    return dict(old, bones=bones)


def masses():
    """⚖ The masses of the modelled parts (oak 700 kg/m3, steel 7800 kg/m3), rounded to 0.1 kg, with their centres in world x, y."""
    oak, steel = 700.0, 7800.0
    top = HALF * DEPTH * 0.015 + (HALF - BEVEL / 2) * (DEPTH - BEVEL) * 0.015
    legs = 2 * (frustum(TAPER_Y, 2 * LEG_FOOT, 2 * LEG_TOP) + (TOP_Y0 - TAPER_Y) * (2 * LEG_TOP) ** 2)
    leg_y = (frustum(TAPER_Y, 2 * LEG_FOOT, 2 * LEG_TOP) * 0.31 + (TOP_Y0 - TAPER_Y) * (2 * LEG_TOP) ** 2 * 0.65) / (legs / 2)
    long_aprons = 2 * (HALF - (LEG_X + LEG_TOP)) * (APRON_Y1 - APRON_Y0) * APRON_T
    end_apron = (2 * (LEG_Z - LEG_TOP)) * (APRON_Y1 - APRON_Y0) * APRON_T
    blocks = (4 * BLOCK_L * (BLOCK_Z1 - BLOCK_Z0) + 2 * BLOCK_L * (END_X1 - 0.12)) * (TOP_Y0 - GAP_Y0)
    runner = 2 * (RUN_X1 - RUN_X0) * (2 * (RUN_Y1 - RUN_Y0) + 2 * (RUN_Z1 - RUN_Z0)) * WALL
    parts = [
        (top * oak, HALF / 2, CREASE_Y),
        (legs * oak, LEG_X, leg_y),
        (long_aprons * oak, (LEG_X + LEG_TOP + HALF) / 2, (APRON_Y0 + APRON_Y1) / 2),
        (end_apron * oak, (END_X0 + END_X1) / 2, (APRON_Y0 + APRON_Y1) / 2),
        (blocks * oak, 0.3, (GAP_Y0 + TOP_Y0) / 2),
        (runner * steel, (RUN_X0 + RUN_X1) / 2, (RUN_Y0 + RUN_Y1) / 2),
    ]
    half = mass_of(parts, 1)
    leaf = (LEAF_X1 - LEAF_X0) * (DEPTH * 0.015 + (DEPTH - BEVEL) * 0.015) * oak
    frame = 2 * (CENTRE_X1 - CENTRE_X0) * (CENTRE_Y1 - CENTRE_Y0) * (CENTRE_Z1 - CENTRE_Z0) * steel
    return {"half": (round(half[0], 1), half[1], half[2]), "leaf": (round(leaf, 1), 0.3, 0.015), "frame": (round(frame, 1), (CENTRE_X0 + CENTRE_X1) / 2, (CENTRE_Y0 + CENTRE_Y1) / 2)}


if __name__ == "__main__":
    print(masses())
