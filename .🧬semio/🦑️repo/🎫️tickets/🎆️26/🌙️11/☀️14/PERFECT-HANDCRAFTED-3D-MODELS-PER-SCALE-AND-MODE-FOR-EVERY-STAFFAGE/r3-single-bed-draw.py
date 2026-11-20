"""✏ Round 3, furniture/single-bed: the 2D drawings of every scale as the true orthographic views of that scale's body (ruling E1), drawn by hand from the same measures as the solids: side elevation (seen from the right), front elevation (seen from the foot end) and plan. Shapes are bone-local; the slat sections and mattress parts are extruded elevation profiles in front and plan, so they tilt and sink with their bones.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_single_bed_kit", os.path.join(HERE, "r3-single-bed-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

FRAME = {"region": "bed-frame", "material": K.PINE}
SLATS = {"region": "slats", "material": K.BIRCH}
POST_Z0, POST_Z1 = K.POST_Z - 0.03, K.POST_Z + 0.03


def n(value):
    """🔢 A coordinate as written in the file."""
    return round(value + 0.0, 4)


def shape(sid, bone, z, tone, geometry, role="body", filled=True, **more):
    """🧾 One authored shape."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geometry)
    out.update(more)
    return out


def rect(x, y, w, h, r=0.0):
    out = {"kind": "rect", "x": n(x), "y": n(y), "w": n(w), "h": n(h)}
    if r:
        out["r"] = r
    return out


def poly(points, closed=True):
    return {"kind": "poly", "points": [[n(x), n(y)] for x, y in points], "closed": closed}


def path(d):
    return {"kind": "path", "d": d}


def post_profile(a0, top, chamfer):
    """🪵 A post seen from the side or the front: 60 mm wide from a0, chamfered at the top."""
    a1 = a0 + K.POST
    if not chamfer:
        return rect(a0, 0, K.POST, top)
    return poly([(a0, 0), (a1, 0), (a1, top - chamfer), (a1 - chamfer, top), (a0 + chamfer, top), (a0, top - chamfer)])


def strokes(xs, y0, y1):
    """〰 Vertical joint lines of a boarded panel as one open path."""
    return path(" ".join(f"M {n(x)} {n(y0)} L {n(x)} {n(y1)}" for x in xs))


def comb(bone, x0, x1, y0, y1, teeth, half, top):
    """🪮 A rail with the slat caps standing on it, as one outline in the frame of its bone."""
    ox, oy = K.ORIGIN[bone]
    points = [(x0 - ox, y0 - oy), (x1 - ox, y0 - oy), (x1 - ox, y1 - oy)]
    for t in sorted(teeth, reverse=True):
        points += [(t + half - ox, y1 - oy), (t + half - ox, top - oy), (t - half - ox, top - oy), (t - half - ox, y1 - oy)]
    points.append((x0 - ox, y1 - oy))
    return poly(points)


def stepped(bone, rail, deck):
    """🪜 A rail with its slat deck on top as one outline (rail and deck are (x0, x1, y0, y1) in world metres)."""
    ox, oy = K.ORIGIN[bone]
    rx0, rx1, ry0, ry1 = rail
    dx0, dx1, _, dy1 = deck
    return poly([(rx0 - ox, ry0 - oy), (rx1 - ox, ry0 - oy), (rx1 - ox, ry1 - oy), (dx1 - ox, ry1 - oy), (dx1 - ox, dy1 - oy), (dx0 - ox, dy1 - oy), (dx0 - ox, ry1 - oy), (rx0 - ox, ry1 - oy)])


def mattress_profile(bone, x0, x1, end, cut, rise):
    """🛏 The side profile of a mattress part in the frame of its bone: square at the joints, rounded over `cut` (along) and `rise` (up) at a free end."""
    ox = K.ORIGIN[bone][0]
    a0, a1, t = n(x0 - ox), n(x1 - ox), K.MAT_T
    if not cut:
        return rect(a0, 0, a1 - a0, t)
    if end == -1:
        return path(f"M {a1} 0 L {a1} {t} L {n(a0 + cut)} {t} Q {a0} {t} {a0} {n(t - rise)} L {a0} {n(rise)} Q {a0} 0 {n(a0 + cut)} 0 Z")
    if end == 1:
        return path(f"M {a0} 0 L {n(a1 - cut)} 0 Q {a1} 0 {a1} {n(rise)} L {a1} {n(t - rise)} Q {a1} {t} {n(a1 - cut)} {t} L {a0} {t} Z")
    return rect(a0, 0, a1 - a0, t)


PARTS = (("mat-head", K.MAT_X0, K.HINGE_X, -1, 3), ("mat-mid", K.HINGE_X, K.FOOT_X, 0, 3.1), ("mat-foot", K.FOOT_X, K.MAT_X1, 1, 3))
EXTRUDED = lambda half: {"solid": {"kind": "extruded", "sweep": "w", "z0": -half, "z1": half}}


def mattress(view, cut, rise, regions, seams):
    """🧵 The three mattress parts of a view, with the piped seams as detail strokes in the side elevation."""
    out = []
    for bone, x0, x1, end, z in PARTS:
        extra = {} if view == "elevation" else EXTRUDED(K.MAT_Z)
        out.append(shape(bone, bone, z, 0.85, mattress_profile(bone, x0, x1, end, cut, rise), regions=regions, material=K.QUILT, **extra))
    if view == "elevation":
        for level, y in seams:
            for bone, x0, x1, end, _ in PARTS:
                ox = K.ORIGIN[bone][0]
                a0 = x0 + (0.004 if end == -1 else 0) - ox
                a1 = x1 - (0.004 if end == 1 else 0) - ox
                out.append(shape(f"piping-{level}-{bone}", bone, 3.05 if end == -1 else 3.5, 0.3, poly([(a0, y), (a1, y)], False), role="detail", filled=False, region="mattress"))
    return out


def slat_sections(view, scale):
    """🪵 The three sections of the slatted base as one outline each (rails with caps or deck, or the merged slab)."""
    extra = lambda half: {} if view == "elevation" else EXTRUDED(half)
    if scale == "50":
        cap_top = K.BASE_Y1 + K.CAP[1]
        head = comb("slat-head", 0.10, 0.80, K.LIFT_Y0, K.LIFT_Y1, K.HEAD_SLATS, K.CAP[0] / 2, cap_top)
        mid = comb("slat-mid", 0.065, K.FOOT_X, K.BASE_Y0, K.BASE_Y1, K.MID_SLATS, K.CAP[0] / 2, cap_top)
        foot = comb("slat-foot", K.FOOT_X, 2.055, K.BASE_Y0, K.BASE_Y1, K.FOOT_SLATS, K.CAP[0] / 2, cap_top)
        halves = (0.415, K.BASE_Z1, K.BASE_Z1)
    elif scale == "100":
        head = stepped("slat-head", (0.10, 0.79, K.LIFT_Y0, K.LIFT_Y1), (0.131, 0.754, K.LIFT_Y1, K.MAT_Y0))
        mid = stepped("slat-mid", (0.065, K.FOOT_X, K.BASE_Y0, K.BASE_Y1), (0.796, 1.378, K.BASE_Y1, K.MAT_Y0))
        foot = stepped("slat-foot", (K.FOOT_X, 2.055, K.BASE_Y0, K.BASE_Y1), (1.408, 1.99, K.BASE_Y1, K.MAT_Y0))
        halves = (0.415, K.BASE_Z1, K.BASE_Z1)
    else:
        slab = lambda bone, x0, x1: rect(x0 - K.ORIGIN[bone][0], K.BASE_Y0 - K.HINGE_Y, x1 - x0, K.MAT_Y0 - K.BASE_Y0)
        head, mid, foot = slab("slat-head", 0.065, K.HINGE_X), slab("slat-mid", K.HINGE_X, K.FOOT_X), slab("slat-foot", K.FOOT_X, 2.055)
        halves = (K.BASE_Z1, K.BASE_Z1, K.BASE_Z1)
    return [shape("slats-" + name, "slat-" + name, 2, 0.6, geometry, **SLATS, **extra(half)) for name, geometry, half in (("head", head, halves[0]), ("mid", mid, halves[1]), ("foot", foot, halves[2]))]


def hatch(scale, name):
    return {"hatch": name} if scale == "50" else {}


def detailed(scale):
    """📐 1:50 and 1:100: posts, frame-and-panel ends, rails, leg, slat sections and mattress."""
    fine = scale == "50"
    chamfer = 0.012 if fine else 0
    cut, rise = (0.045, 0.035) if fine else (0.045, 0.03)
    seams = (("top", K.MAT_T - 0.022), ("bottom", 0.022)) if fine else ()
    joints = [0.089 * k for k in range(-4, 5)]
    gv, gh = hatch(scale, "grain-v"), hatch(scale, "grain-h")
    elevation = [
        shape("head", "frame", 6, 0.5, post_profile(0, K.HEAD_H, chamfer), **FRAME, **gv),
        shape("foot", "frame", 6, 0.5, post_profile(K.LENGTH - K.POST, K.FOOT_H, chamfer), **FRAME, **gv),
        shape("rail", "frame", 5, 0.55, rect(K.POST, K.RAIL_Y0, K.LENGTH - 2 * K.POST, K.RAIL_Y1 - K.RAIL_Y0), **FRAME, **gh),
        shape("leg-mid", "frame", 4.4, 0.5, rect(1.03, 0, 0.06, K.RAIL_Y0), **FRAME, **gv),
    ] + slat_sections("elevation", scale) + mattress("elevation", cut, rise, ["mattress"], seams)
    width = 2 * K.BOARD_Z
    front = mattress("front", cut, rise, ["mattress"], ()) + slat_sections("front", scale) + [
        shape("head-post-0", "frame", 1, 0.5, post_profile(POST_Z0, K.HEAD_H, chamfer), **FRAME, **gv),
        shape("head-post-1", "frame", 1, 0.5, post_profile(-POST_Z1, K.HEAD_H, chamfer), **FRAME, **gv),
        shape("head", "frame", 0.9, 0.55, rect(-K.BOARD_Z, 0.30, width, 0.55), **FRAME, **gv),
        shape("head-rail", "frame", 1, 0.5, rect(-K.BOARD_Z, 0.85, width, 0.08), **FRAME, **gh),
        shape("foot-post-0", "frame", 6, 0.5, post_profile(POST_Z0, K.FOOT_H, chamfer), **FRAME, **gv),
        shape("foot-post-1", "frame", 6, 0.5, post_profile(-POST_Z1, K.FOOT_H, chamfer), **FRAME, **gv),
        shape("foot-panel", "frame", 5.9, 0.55, rect(-K.BOARD_Z, 0.30, width, 0.15), **FRAME, **gv),
        shape("foot-rail", "frame", 6, 0.5, rect(-K.BOARD_Z, 0.45, width, 0.08), **FRAME, **gh),
        shape("rail-end", "frame", 6, 0.55, rect(-K.BOARD_Z, K.RAIL_Y0, width, K.RAIL_Y1 - K.RAIL_Y0), **FRAME, **gh),
        shape("leg-mid", "frame", 4.4, 0.5, rect(-0.03, 0, 0.06, K.RAIL_Y0), **FRAME, **gv),
    ]
    if fine:
        front.append(shape("head-joints", "frame", 0.95, 0.3, strokes(joints, 0.30, 0.85), role="texture", filled=False, region="bed-frame"))
        front.append(shape("foot-joints", "frame", 5.95, 0.3, strokes(joints, 0.30, 0.45), role="texture", filled=False, region="bed-frame"))
    r = 0.005 if fine else 0.008
    plan = mattress("plan", cut, rise, ["mattress"], ()) + slat_sections("plan", scale) + [
        shape("head", "frame", 6, 0.5, rect(0.015, -K.BOARD_Z, K.RAIL_T, width), **FRAME, **gv),
        shape("head-post-0", "frame", 6, 0.5, rect(0, POST_Z0, K.POST, K.POST, r), **FRAME),
        shape("head-post-1", "frame", 6, 0.5, rect(0, -POST_Z1, K.POST, K.POST, r), **FRAME),
        shape("foot", "frame", 6, 0.5, rect(2.075, -K.BOARD_Z, K.RAIL_T, width), **FRAME, **gv),
        shape("foot-post-0", "frame", 6, 0.5, rect(K.LENGTH - K.POST, POST_Z0, K.POST, K.POST, r), **FRAME),
        shape("foot-post-1", "frame", 6, 0.5, rect(K.LENGTH - K.POST, -POST_Z1, K.POST, K.POST, r), **FRAME),
        shape("rail-0", "frame", 4.5, 0.55, rect(K.POST, K.CLEAR_Z, K.LENGTH - 2 * K.POST, K.RAIL_T), **FRAME, **gh),
        shape("rail-1", "frame", 4.5, 0.55, rect(K.POST, -K.CLEAR_Z - K.RAIL_T, K.LENGTH - 2 * K.POST, K.RAIL_T), **FRAME, **gh),
    ]
    for name, z in (("ledger-0", K.CLEAR_Z - K.LEDGER), ("ledger-1", -K.CLEAR_Z)):
        plan.append(shape(name, "frame", 1.5, 0.55, rect(K.POST, z, K.LENGTH - 2 * K.POST, K.LEDGER), **FRAME))
    return {"elevation": elevation, "front": front, "plan": plan}


def coarse200():
    """📏 1:200: posts, one board per end, rails, leg, slat slabs and mattress."""
    width = 2 * K.BOARD_Z
    elevation = [
        shape("head", "frame", 6, 0.5, rect(0, 0, K.POST, K.HEAD_H), **FRAME),
        shape("foot", "frame", 6, 0.5, rect(K.LENGTH - K.POST, 0, K.POST, K.FOOT_H), **FRAME),
        shape("rail", "frame", 5, 0.55, rect(K.POST, K.RAIL_Y0, K.LENGTH - 2 * K.POST, K.RAIL_Y1 - K.RAIL_Y0), **FRAME),
        shape("leg-mid", "frame", 4.4, 0.5, rect(1.03, 0, 0.06, K.RAIL_Y0), **FRAME),
    ] + slat_sections("elevation", "200") + mattress("elevation", 0.04, 0.03, ["mattress"], ())
    front = mattress("front", 0.04, 0.03, ["mattress"], ()) + slat_sections("front", "200") + [
        shape("head-post-0", "frame", 1, 0.5, rect(POST_Z0, 0, K.POST, K.HEAD_H), **FRAME),
        shape("head-post-1", "frame", 1, 0.5, rect(-POST_Z1, 0, K.POST, K.HEAD_H), **FRAME),
        shape("head", "frame", 0.9, 0.55, rect(-K.BOARD_Z, K.RAIL_Y0, width, 0.93 - K.RAIL_Y0), **FRAME),
        shape("foot-post-0", "frame", 6, 0.5, rect(POST_Z0, 0, K.POST, K.FOOT_H), **FRAME),
        shape("foot-post-1", "frame", 6, 0.5, rect(-POST_Z1, 0, K.POST, K.FOOT_H), **FRAME),
        shape("foot-rail", "frame", 5.9, 0.55, rect(-K.BOARD_Z, K.RAIL_Y0, width, 0.53 - K.RAIL_Y0), **FRAME),
        shape("leg-mid", "frame", 4.4, 0.5, rect(-0.03, 0, 0.06, K.RAIL_Y0), **FRAME),
    ]
    plan = mattress("plan", 0.04, 0.03, ["mattress"], ()) + slat_sections("plan", "200") + [
        shape("head", "frame", 6, 0.5, rect(0.015, -K.BOARD_Z, K.RAIL_T, width), **FRAME),
        shape("head-post-0", "frame", 6, 0.5, rect(0, POST_Z0, K.POST, K.POST), **FRAME),
        shape("head-post-1", "frame", 6, 0.5, rect(0, -POST_Z1, K.POST, K.POST), **FRAME),
        shape("foot", "frame", 6, 0.5, rect(2.075, -K.BOARD_Z, K.RAIL_T, width), **FRAME),
        shape("foot-post-0", "frame", 6, 0.5, rect(K.LENGTH - K.POST, POST_Z0, K.POST, K.POST), **FRAME),
        shape("foot-post-1", "frame", 6, 0.5, rect(K.LENGTH - K.POST, -POST_Z1, K.POST, K.POST), **FRAME),
        shape("rail-0", "frame", 4.5, 0.55, rect(K.POST, K.CLEAR_Z, K.LENGTH - 2 * K.POST, K.RAIL_T), **FRAME),
        shape("rail-1", "frame", 4.5, 0.55, rect(K.POST, -K.CLEAR_Z - K.RAIL_T, K.LENGTH - 2 * K.POST, K.RAIL_T), **FRAME),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def coarse500():
    """🗺 1:500: two end panels, the tray, the leg and the mattress."""
    wide = 2 * POST_Z1
    tray = 2 * (K.CLEAR_Z + K.RAIL_T)
    regions = ["mattress", "slats"]
    elevation = [
        shape("head", "frame", 6, 0.5, rect(0, 0, K.POST, K.HEAD_H), **FRAME),
        shape("foot", "frame", 6, 0.5, rect(K.LENGTH - K.POST, 0, K.POST, K.FOOT_H), **FRAME),
        shape("rail", "frame", 5, 0.55, rect(K.POST, K.RAIL_Y0, K.LENGTH - 2 * K.POST, K.RAIL_Y1 - K.RAIL_Y0), **FRAME),
        shape("leg-mid", "frame", 4.4, 0.5, rect(1.03, 0, 0.06, K.RAIL_Y0), **FRAME),
    ] + mattress("elevation", 0, 0, regions, ())
    front = mattress("front", 0, 0, regions, ()) + [
        shape("head", "frame", 1, 0.5, rect(-POST_Z1, 0, wide, K.HEAD_H), **FRAME),
        shape("foot", "frame", 6, 0.5, rect(-POST_Z1, 0, wide, K.FOOT_H), **FRAME),
    ]
    plan = mattress("plan", 0, 0, regions, ()) + [
        shape("head", "frame", 6, 0.5, rect(0, -POST_Z1, K.POST, wide), **FRAME),
        shape("foot", "frame", 6, 0.5, rect(K.LENGTH - K.POST, -POST_Z1, K.POST, wide), **FRAME),
        shape("rail", "frame", 2, 0.55, rect(K.POST, -tray / 2, K.LENGTH - 2 * K.POST, tray), **FRAME),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def apply(asset):
    """🖊 Replaces the geometry of every representation."""
    reps = asset["representations"]
    reps["50"]["geometry"] = detailed("50")
    reps["100"]["geometry"] = detailed("100")
    reps["200"]["geometry"] = coarse200()
    reps["500"]["geometry"] = coarse500()
