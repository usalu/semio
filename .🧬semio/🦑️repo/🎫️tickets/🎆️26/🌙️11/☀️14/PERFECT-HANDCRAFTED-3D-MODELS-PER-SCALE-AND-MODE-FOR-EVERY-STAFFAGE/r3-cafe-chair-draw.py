"""✏ Round 3, furniture/cafe-chair: the 2D drawings of every scale as the true orthographic views of that scale's body (ruling E1), drawn by hand from the same measures as the solids: side elevation (seen from the right, bone-local; near and far tubes coincide), front elevation (seen from ahead, x = lateral z, y = height) and plan (seen from above, x ahead, y = lateral z). Tubes are filled outlines at 1:50 and pen strokes along their centre lines at the coarser scales.
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_cafe_chair_kit", os.path.join(HERE, "r3-cafe-chair-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

SEAT = {"region": "seat-beech", "material": K.BEECH}
BACK = {"region": "back-beech", "material": K.BEECH}
TUBE = {"region": "steel-tube", "material": K.STEEL}
NYL = {"material": K.NYLON}
TUBE50 = dict(TUBE, penMm=0.25)
SEAT50 = dict(SEAT, penMm=0.35)
BACK50 = dict(BACK, penMm=0.35)
NYL50 = dict(NYL, penMm=0.18)
T_SEAT, T_RAIL, T_BACK, T_TUBE, T_NYL = 0.62, 0.45, 0.55, 0.3, 0.15


def n(value):
    """🔢 A coordinate as written in the file."""
    return round(value + 0.0, 4)


def shape(sid, bone, z, tone, geometry, role="body", filled=True, **more):
    """🧾 One authored shape."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geometry)
    out.update(more)
    return out


def poly(points, closed=True):
    """🔷 A polygon or an open polyline."""
    return {"kind": "poly", "points": [[n(x), n(y)] for x, y in points], "closed": closed}


def rect(x0, y0, x1, y1):
    """▭ A rectangle between two corners."""
    return {"kind": "rect", "x": n(min(x0, x1)), "y": n(min(y0, y1)), "w": n(abs(x1 - x0)), "h": n(abs(y1 - y0))}


def circle(cx, cy, r):
    """⚪ A circle."""
    return {"kind": "ellipse", "cx": n(cx), "cy": n(cy), "rx": n(r), "ry": n(r)}


def strokes(paths):
    """〰 Several open polylines as one path."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{n(x)} {n(y)}" for x, y in pts) for pts in paths)}


def el(bone, pts):
    """📐 World (x, y) points in the elevation frame of a bone."""
    return [K.local(bone, (x, y, 0.0))[:2] for x, y in pts]


def strip(centre, half):
    """🧪 The outline of a tube of half width `half` about a polyline: left offsets out, right offsets back."""
    left, right = [], []
    for i, p in enumerate(centre):
        a = centre[max(i - 1, 0)]
        b = centre[min(i + 1, len(centre) - 1)]
        t = K.unit((b[0] - a[0], b[1] - a[1]))
        nx, ny = -t[1], t[0]
        left.append((p[0] + half * nx, p[1] + half * ny))
        right.append((p[0] - half * nx, p[1] - half * ny))
    return left + right[::-1]


def rear_centres():
    """🦿 World centre lines (x, y) of the rear tube: leg part (glide to bend middle) and post part (bend middle to top)."""
    leg, post = K.rear_stations(True)
    return [p for p, _ in leg], [p for p, _ in post]


def front_centre(y0=0.005):
    """🦵 World centre line of the front leg."""
    return [K.add(K.FRONT_FOOT, K.FRONT_DIR, y0 / K.FRONT_DIR[1]), K.FRONT_TOP]


def glide_world(xf):
    """🔘 The side outline of a glide (world x, y) about a foot at xf."""
    g = K.GLIDE_R
    return [(xf - 0.0105, 0.0), (xf + 0.0105, 0.0), (xf + g, 0.0015), (xf + g, 0.008), (xf + 0.011, K.GLIDE_H), (xf - 0.011, K.GLIDE_H), (xf - g, 0.008), (xf - g, 0.0015)]


def cap_local():
    """🎩 The side outline of a post cap in the back frame."""
    L = K.POST_L
    return [(L - 0.001, K.R), (L + 0.002, K.R), (L + 0.005, 0.0095), (L + 0.007, 0.006), (L + K.CAP_H, 0.0), (L + 0.007, -0.006), (L + 0.005, -0.0095), (L + 0.002, -K.R), (L - 0.001, -K.R)]


def back_board_v(u0, u1):
    """🌙 The elevation v range of a curved back board between u0 and u1: from the front face of its ends to the back face of its middle."""
    return K.back_vc(K.BACK_W) - K.BACK_T / 2, K.back_vc(0.0) + K.BACK_T / 2


def seat_band_front(v0):
    """🪑 The front view of the seat board (or slab down to v0): the front face over the lower rear corners of the raked board."""
    top_f = K.seat_xy(K.SEAT_L, 0.0)[1]
    bot_f = K.seat_xy(K.SEAT_L, v0)[1]
    bot_r = K.seat_xy(0.0, v0)[1]
    w, wr = K.SEAT_FRONT_W, K.SEAT_REAR_W
    return [(-w, bot_f), (-w, top_f), (w, top_f), (w, bot_f), (wr, bot_r), (-wr, bot_r)]


def apron_front(v_top):
    """🟫 The front view of the apron frame below the board."""
    lo = K.seat_xy(K.U_POST, K.APRON_V0)[1]
    hi = K.seat_xy(K.U_FRONT, v_top)[1]
    return rect(-K.APRON_W1, lo, K.APRON_W1, hi)


def front_leg_front(side):
    """🦵 Front view outline of a front leg (z, y)."""
    return strip([(side * K.FRONT_FOOT_Z, 0.005), (side * K.FRONT_TOP_Z, K.FRONT_TOP[1])], K.R)


def rear_leg_front(side):
    """🦿 Front view outline of a rear tube from the glide to the top (z, y)."""
    leg, post = K.rear_stations(True)
    pts = [(side * K.rear_z(p), p[1]) for p, _ in leg + post[1:]]
    return strip(pts, K.R)


def back_front(u0, u1):
    """▭ Front view of a back board: lateral span by height span."""
    ys = [K.world("back", (u, v, 0))[1] for u in (u0, u1) for v in back_board_v(u0, u1)]
    return rect(-K.BACK_W, min(ys), K.BACK_W, max(ys))


def plan_seat(steps, notch=True):
    """⬜ The plan of the seat board (x, z) from its top face."""
    return [K.seat_xy(u, 0.0)[0:1] + (w,) for w, u in K.seat_outline(steps, notch=notch)]


def plan_band(u0, u1, steps, flat=None):
    """🌙 The plan of a back board between heights u0 and u1: front edge from its lower front face, rear edge from its upper back face, sampled along w."""
    ws = [-K.BACK_W + 2 * K.BACK_W * k / steps for k in range(steps + 1)]
    vc = (lambda w: flat) if flat is not None else K.back_vc
    half = K.BACK_T / 2 if flat is None else 0.015
    front = [(K.world("back", (u0, vc(w) - half, 0))[0], w) for w in ws]
    rear = [(K.world("back", (u1, vc(w) + half, 0))[0], w) for w in reversed(ws)]
    return front + rear


def post_plan(side):
    """🦿 Plan outline of a post from the bend middle to the top (x, z)."""
    _, post = rear_centres()
    return strip([(p[0], side * K.POST_Z) for p in post], K.R)


def rear_leg_plan(side):
    """🦿 Plan outline of a rear leg from the glide to the bend middle (x, z)."""
    leg, _ = K.rear_stations(True)
    return strip([(p[0], side * K.rear_z(p)) for p, _ in leg], K.R)


def front_leg_plan(side):
    """🦵 Plan outline of a front leg (x, z)."""
    return strip([(K.FRONT_FOOT[0], side * K.FRONT_FOOT_Z), (K.FRONT_TOP[0], side * K.FRONT_TOP_Z)], K.R)


def stretcher_plan(side, half):
    """➖ Plan outline of a stretcher (x, z)."""
    a, b = K.STRETCH_REAR, K.STRETCH_FRONT
    return strip([(a[0], side * a[2]), (b[0], side * b[2])], half)


def elevation50():
    """🔍 1:50 side elevation."""
    leg, post = rear_centres()
    u0, u1 = K.U_POST + 0.004, K.U_FRONT - 0.004
    bv0, bv1 = back_board_v(K.BOARD_U0, K.BOARD_U1)
    return [
        shape("seat-apron", "seat", 2, T_RAIL, rect(u0, K.APRON_V0, u1, K.APRON_V1), **SEAT50),
        shape("stretcher-tube", "stretcher", 2.5, T_TUBE, poly(strip([(0.0, 0.0), (K.STRETCH_L, 0.0)], K.RS)), **TUBE50),
        shape("back-panel", "back", 3, T_BACK, rect(K.BOARD_U0, bv0, K.BOARD_U1, bv1), **BACK50),
        shape("back-lower-rail", "back", 3, T_BACK, rect(K.RAIL_U0, bv0, K.RAIL_U1, bv1), **BACK50),
        shape("back-panel-end", "back", 3.1, 0.3, poly([(K.BOARD_U0, bv0 + K.BACK_T), (K.BOARD_U1, bv0 + K.BACK_T)], False), role="detail", filled=False, region="back-beech"),
        shape("leg-front-tube", "leg-front", 3.5, T_TUBE, poly(el("leg-front", strip(front_centre(), K.R))), **TUBE50),
        shape("seat-pad", "seat", 4, T_SEAT, rect(0.0, -K.BOARD_T, K.SEAT_L, 0.0), **SEAT50),
        shape("seat-grain", "seat", 4.2, 0.3, strokes([[(0.09, -0.007), (0.16, -0.006), (0.25, -0.008), (0.33, -0.007)], [(0.14, -0.014), (0.22, -0.013), (0.31, -0.015), (0.38, -0.014)]]), role="texture", filled=False, region="seat-beech"),
        shape("leg-rear-tube", "leg-rear", 5, T_TUBE, poly(el("leg-rear", strip(leg, K.R))), **TUBE50),
        shape("back-post", "back", 5, T_TUBE, poly(el("back", strip(post, K.R))), **TUBE50),
        shape("leg-foot-rear", "leg-rear", 5.5, T_NYL, poly(el("leg-rear", glide_world(K.REAR_FOOT[0]))), role="detail", **NYL50),
        shape("leg-foot-front", "leg-front", 5.5, T_NYL, poly(el("leg-front", glide_world(K.FRONT_FOOT[0]))), role="detail", **NYL50),
        shape("back-cap", "back", 5.5, T_NYL, poly(cap_local()), role="detail", **NYL50),
    ]


def front50():
    """🔍 1:50 front elevation."""
    out = []
    for k, side in enumerate((1, -1)):
        out.append(shape(f"leg-rear-tube-{k}", "leg-rear", 1, T_TUBE, poly(rear_leg_front(side)), **TUBE50))
        g = K.GLIDE_R
        out.append(shape(f"leg-foot-rear-{k}", "leg-rear", 1.2, T_NYL, poly([(side * K.REAR_FOOT_Z - 0.0105, 0.0), (side * K.REAR_FOOT_Z + 0.0105, 0.0), (side * K.REAR_FOOT_Z + g, 0.0015), (side * K.REAR_FOOT_Z + g, 0.008), (side * K.REAR_FOOT_Z + 0.011, K.GLIDE_H), (side * K.REAR_FOOT_Z - 0.011, K.GLIDE_H), (side * K.REAR_FOOT_Z - g, 0.008), (side * K.REAR_FOOT_Z - g, 0.0015)]), role="detail", **NYL50))
        top = K.world("back", (K.POST_L, 0, 0))[1]
        out.append(shape(f"back-cap-{k}", "back", 1.5, T_NYL, poly([(side * K.POST_Z - K.R, top - 0.001), (side * K.POST_Z + K.R, top - 0.001), (side * K.POST_Z + K.R, top + 0.002), (side * K.POST_Z + 0.0095, top + 0.005), (side * K.POST_Z + 0.006, top + 0.007), (side * K.POST_Z, top + K.CAP_H), (side * K.POST_Z - 0.006, top + 0.007), (side * K.POST_Z - 0.0095, top + 0.005), (side * K.POST_Z - K.R, top + 0.002)]), role="detail", **NYL50))
    out.append(shape("back-lower-rail", "back", 2, T_BACK, back_front(K.RAIL_U0, K.RAIL_U1), **BACK50))
    out.append(shape("back-panel", "back", 2.5, T_BACK, back_front(K.BOARD_U0, K.BOARD_U1), **BACK50))
    ys = back_front(K.BOARD_U0, K.BOARD_U1)
    yb, hb = ys["y"], ys["h"]
    out.append(shape("back-grain", "back", 2.6, 0.3, strokes([[(-0.15, yb + 0.6 * hb), (-0.05, yb + 0.62 * hb), (0.06, yb + 0.59 * hb), (0.15, yb + 0.61 * hb)], [(-0.11, yb + 0.33 * hb), (0.0, yb + 0.35 * hb), (0.12, yb + 0.32 * hb)]]), role="texture", filled=False, region="back-beech"))
    out.append(shape("seat-apron", "seat", 3, T_RAIL, apron_front(-K.BOARD_T), **SEAT50))
    for k, side in enumerate((1, -1)):
        out.append(shape(f"leg-front-tube-{k}", "leg-front", 4, T_TUBE, poly(front_leg_front(side)), **TUBE50))
        x = side * K.FRONT_FOOT_Z
        g = K.GLIDE_R
        out.append(shape(f"leg-foot-front-{k}", "leg-front", 4.5, T_NYL, poly([(x - 0.0105, 0.0), (x + 0.0105, 0.0), (x + g, 0.0015), (x + g, 0.008), (x + 0.011, K.GLIDE_H), (x - 0.011, K.GLIDE_H), (x - g, 0.008), (x - g, 0.0015)]), role="detail", **NYL50))
    out.append(shape("seat-pad", "seat", 5, T_SEAT, poly(seat_band_front(-K.BOARD_T)), **SEAT50))
    return out


def plan50():
    """🔍 1:50 plan."""
    out = []
    for k, side in enumerate((1, -1)):
        out.append(shape(f"leg-rear-tube-{k}", "leg-rear", 1, T_TUBE, poly(rear_leg_plan(side)), **TUBE50))
        out.append(shape(f"leg-foot-rear-{k}", "leg-rear", 1.1, T_NYL, circle(K.REAR_FOOT[0], side * K.REAR_FOOT_Z, K.GLIDE_R), role="detail", **NYL50))
        out.append(shape(f"leg-front-tube-{k}", "leg-front", 1, T_TUBE, poly(front_leg_plan(side)), **TUBE50))
        out.append(shape(f"leg-foot-front-{k}", "leg-front", 1.1, T_NYL, circle(K.FRONT_FOOT[0], side * K.FRONT_FOOT_Z, K.GLIDE_R), role="detail", **NYL50))
        out.append(shape(f"stretcher-tube-{k}", "stretcher", 1.5, T_TUBE, poly(stretcher_plan(side, K.RS)), **TUBE50))
    out.append(shape("seat-pad", "seat", 3, T_SEAT, poly(plan_seat(4)), **SEAT50))
    x0, x1 = K.seat_xy(0.09, 0)[0], K.seat_xy(0.39, 0)[0]
    out.append(shape("seat-grain", "seat", 3.2, 0.3, strokes([[(x0, -0.12), (x0 + 0.1, -0.11), (x0 + 0.2, -0.125), (x1, -0.115)], [(x0 + 0.05, 0.06), (x0 + 0.15, 0.07), (x0 + 0.24, 0.055), (x1, 0.065)]]), role="texture", filled=False, region="seat-beech"))
    for k, side in enumerate((1, -1)):
        out.append(shape(f"back-post-{k}", "back", 4, T_TUBE, poly(post_plan(side)), **TUBE50))
    out.append(shape("back-lower-rail", "back", 5, T_BACK, poly(plan_band(K.RAIL_U0, K.RAIL_U1, 8)), **BACK50))
    out.append(shape("back-panel", "back", 6, T_BACK, poly(plan_band(K.BOARD_U0, K.BOARD_U1, 8)), **BACK50))
    for k, side in enumerate((1, -1)):
        out.append(shape(f"back-cap-{k}", "back", 7, T_NYL, circle(K.POST_TOP[0], side * K.POST_Z, K.R), role="detail", **NYL50))
    return out


def stroke_el(bone, pts):
    """〰 An open centre-line stroke in the elevation frame of a bone."""
    return poly(el(bone, pts), False)


def elevation100():
    """🔍 1:100 side elevation."""
    leg, post = rear_centres()
    leg = [(K.REAR_FOOT[0], 0.0)] + leg[1:]
    bv0, bv1 = back_board_v(K.BOARD_U0, K.BOARD_U1)
    return [
        shape("seat-apron", "seat", 2, T_RAIL, rect(K.U_POST + 0.004, K.APRON_V0, K.U_FRONT - 0.004, K.APRON_V1), **SEAT),
        shape("stretcher", "stretcher", 2.5, T_TUBE, poly([(0.0, 0.0), (K.STRETCH_L, 0.0)], False), role="detail", filled=False, **TUBE),
        shape("back-panel", "back", 3, T_BACK, rect(K.BOARD_U0, bv0, K.BOARD_U1, bv1), **BACK),
        shape("back-rail", "back", 3, T_BACK, rect(K.RAIL_U0, bv0, K.RAIL_U1, bv1), **BACK),
        shape("leg-front", "leg-front", 3.5, T_TUBE, stroke_el("leg-front", [(K.FRONT_FOOT[0], 0.0), K.FRONT_TOP]), filled=False, **TUBE),
        shape("seat-band", "seat", 4, T_SEAT, rect(0.0, -K.BOARD_T, K.SEAT_L, 0.0), **SEAT),
        shape("leg-rear", "leg-rear", 5, T_TUBE, stroke_el("leg-rear", leg), filled=False, **TUBE),
        shape("back-post", "back", 5, T_TUBE, stroke_el("back", post), filled=False, **TUBE),
    ]


def front_strokes(k, side):
    """〰 Front view strokes of the tubes of one side: rear leg, post, front leg."""
    leg, post = K.rear_stations(True)
    rear = [(side * K.REAR_FOOT_Z, 0.0)] + [(side * K.rear_z(p), p[1]) for p, _ in leg[1:]]
    up = [(side * K.POST_Z, p[1]) for p, _ in post]
    return [
        shape(f"leg-rear-{k}", "leg-rear", 1, T_TUBE, poly(rear, False), filled=False, **TUBE),
        shape(f"back-post-{k}", "back", 1.5, T_TUBE, poly(up, False), filled=False, **TUBE),
        shape(f"leg-front-{k}", "leg-front", 4, T_TUBE, poly([(side * K.FRONT_FOOT_Z, 0.0), (side * K.FRONT_TOP_Z, K.FRONT_TOP[1])], False), filled=False, **TUBE),
    ]


def front100():
    """🔍 1:100 front elevation."""
    out = []
    for k, side in enumerate((1, -1)):
        out += front_strokes(k, side)
    out.append(shape("back-rail", "back", 2, T_BACK, back_front(K.RAIL_U0, K.RAIL_U1), **BACK))
    out.append(shape("back-panel", "back", 2.5, T_BACK, back_front(K.BOARD_U0, K.BOARD_U1), **BACK))
    out.append(shape("seat-apron", "seat", 3, T_RAIL, apron_front(-K.BOARD_T), **SEAT))
    out.append(shape("seat-band", "seat", 5, T_SEAT, poly(seat_band_front(-K.BOARD_T)), **SEAT))
    return out


def plan_strokes(k, side):
    """〰 Plan strokes of the tubes of one side: rear leg, post, front leg, stretcher."""
    leg, post = K.rear_stations(True)
    rear = [(K.REAR_FOOT[0], side * K.REAR_FOOT_Z)] + [(p[0], side * K.rear_z(p)) for p, _ in leg[1:]]
    up = [(p[0], side * K.POST_Z) for p, _ in post]
    a, b = K.STRETCH_REAR, K.STRETCH_FRONT
    return [
        shape(f"leg-rear-{k}", "leg-rear", 1, T_TUBE, poly(rear, False), filled=False, **TUBE),
        shape(f"back-post-{k}", "back", 4, T_TUBE, poly(up, False), filled=False, **TUBE),
        shape(f"leg-front-{k}", "leg-front", 1, T_TUBE, poly([(K.FRONT_FOOT[0], side * K.FRONT_FOOT_Z), (K.FRONT_TOP[0], side * K.FRONT_TOP_Z)], False), filled=False, **TUBE),
        shape(f"stretcher-{k}", "stretcher", 1.5, T_TUBE, poly([(a[0], side * a[2]), (b[0], side * b[2])], False), role="detail", filled=False, **TUBE),
    ]


def plan100():
    """🔍 1:100 plan."""
    out = []
    for k, side in enumerate((1, -1)):
        out += plan_strokes(k, side)
    out.append(shape("seat-band", "seat", 3, T_SEAT, poly(plan_seat(2)), **SEAT))
    out.append(shape("back-rail", "back", 5, T_BACK, poly(plan_band(K.RAIL_U0, K.RAIL_U1, 4)), **BACK))
    out.append(shape("back-panel", "back", 6, T_BACK, poly(plan_band(K.BOARD_U0, K.BOARD_U1, 4)), **BACK))
    return out


def rear_coarse():
    """🦿 World centre lines of the rear tube at 1:100 and 1:200: leg part from the floor to the bend middle, post part from there to the top."""
    leg, post = K.rear_stations(False)
    return [(K.REAR_FOOT[0], 0.0)] + [p for p, _ in leg[1:]], [p for p, _ in post]


def elevation200():
    """🔍 1:200 side elevation."""
    v_front = K.back_vc(K.POST_Z) - K.BACK_T / 2
    return [
        shape("seat-apron", "seat", 2, T_RAIL, rect(K.U_POST + 0.004, K.APRON_V0, K.U_FRONT - 0.004, -0.030), **SEAT),
        shape("stretcher", "stretcher", 2.5, T_TUBE, poly([(0.0, 0.0), (K.STRETCH_L, 0.0)], False), role="detail", filled=False, **TUBE),
        shape("back-panel", "back", 3, T_BACK, rect(K.BOARD_U0, v_front, K.BOARD_U1, v_front + 0.030), **BACK),
        shape("back-rail", "back", 3, T_BACK, rect(K.RAIL_U0, v_front, K.RAIL_U1, v_front + 0.030), **BACK),
        shape("leg-front", "leg-front", 3.5, T_TUBE, stroke_el("leg-front", [(K.FRONT_FOOT[0], 0.0), K.FRONT_TOP]), filled=False, **TUBE),
        shape("seat-band", "seat", 4, T_SEAT, rect(0.0, -0.030, K.SEAT_L, 0.0), **SEAT),
        shape("leg-rear", "leg-rear", 5, T_TUBE, stroke_el("leg-rear", rear_coarse()[0]), filled=False, **TUBE),
        shape("back-post", "back", 5, T_TUBE, stroke_el("back", rear_coarse()[1]), filled=False, **TUBE),
    ]


def back_front_flat(u0, u1, v_front, thick):
    """▭ Front view of a flat back board."""
    ys = [K.world("back", (u, v, 0))[1] for u in (u0, u1) for v in (v_front, v_front + thick)]
    return rect(-K.BACK_W, min(ys), K.BACK_W, max(ys))


def front200():
    """🔍 1:200 front elevation."""
    v_front = K.back_vc(K.POST_Z) - K.BACK_T / 2
    out = []
    for k, side in enumerate((1, -1)):
        out += front_strokes(k, side)
    out.append(shape("back-rail", "back", 2, T_BACK, back_front_flat(K.RAIL_U0, K.RAIL_U1, v_front, 0.030), **BACK))
    out.append(shape("back-panel", "back", 2.5, T_BACK, back_front_flat(K.BOARD_U0, K.BOARD_U1, v_front, 0.030), **BACK))
    out.append(shape("seat-apron", "seat", 3, T_RAIL, apron_front(-0.030), **SEAT))
    out.append(shape("seat-band", "seat", 5, T_SEAT, poly(seat_band_front(-0.030)), **SEAT))
    return out


def plan_flat(u0, u1, v_front, thick):
    """▭ Plan of a flat back board."""
    xs = [K.world("back", (u, v, 0))[0] for u in (u0, u1) for v in (v_front, v_front + thick)]
    return rect(min(xs), -K.BACK_W, max(xs), K.BACK_W)


def plan200():
    """🔍 1:200 plan."""
    v_front = K.back_vc(K.POST_Z) - K.BACK_T / 2
    out = []
    for k, side in enumerate((1, -1)):
        out += plan_strokes(k, side)
    out.append(shape("seat-band", "seat", 3, T_SEAT, poly(plan_seat(0, notch=False)), **SEAT))
    out.append(shape("back-panel", "back", 6, T_BACK, plan_flat(K.BOARD_U0, K.BOARD_U1, v_front, 0.030), **BACK))
    return out


def elevation500():
    """🔍 1:500 side elevation."""
    return [
        shape("back-panel", "back", 3, T_BACK, rect(K.BOARD_U0, K.SLAB500_V0, K.BOARD_U1, K.SLAB500_V1), **BACK),
        shape("leg-front", "leg-front", 3.5, T_TUBE, stroke_el("leg-front", [(K.FRONT_FOOT[0], 0.0), K.FRONT_TOP]), filled=False, **TUBE),
        shape("seat-band", "seat", 4, T_SEAT, rect(0.0, -0.075, K.SEAT_L, 0.0), **SEAT),
        shape("stretcher", "stretcher", 2.5, T_TUBE, poly([(0.0, 0.0), (K.STRETCH_L, 0.0)], False), filled=False, **TUBE),
        shape("leg-rear", "leg-rear", 5, T_TUBE, stroke_el("leg-rear", [(K.REAR_FOOT[0], 0.0), K.CORNER]), filled=False, **TUBE),
        shape("back-post", "back", 5, T_TUBE, stroke_el("back", [K.CORNER, K.POST_TOP]), filled=False, **TUBE),
    ]


def front500():
    """🔍 1:500 front elevation."""
    out = []
    for k, side in enumerate((1, -1)):
        out.append(shape(f"leg-rear-{k}", "leg-rear", 1, T_TUBE, poly([(side * K.REAR_FOOT_Z, 0.0), (side * K.POST_Z, K.CORNER[1])], False), filled=False, **TUBE))
        out.append(shape(f"back-post-{k}", "back", 1.5, T_TUBE, poly([(side * K.POST_Z, K.CORNER[1]), (side * K.POST_Z, K.POST_TOP[1])], False), filled=False, **TUBE))
        out.append(shape(f"leg-front-{k}", "leg-front", 4, T_TUBE, poly([(side * K.FRONT_FOOT_Z, 0.0), (side * K.FRONT_TOP_Z, K.FRONT_TOP[1])], False), filled=False, **TUBE))
    out.append(shape("back-panel", "back", 2.5, T_BACK, back_front_flat(K.BOARD_U0, K.BOARD_U1, K.SLAB500_V0, K.SLAB500_V1 - K.SLAB500_V0), **BACK))
    out.append(shape("seat-band", "seat", 5, T_SEAT, poly(seat_band_front(-0.075)), **SEAT))
    return out


def plan500():
    """🔍 1:500 plan."""
    out = []
    for k, side in enumerate((1, -1)):
        out.append(shape(f"leg-rear-{k}", "leg-rear", 1, T_TUBE, poly([(K.REAR_FOOT[0], side * K.REAR_FOOT_Z), (K.CORNER[0], side * K.POST_Z)], False), filled=False, **TUBE))
        out.append(shape(f"back-post-{k}", "back", 4, T_TUBE, poly([(K.CORNER[0], side * K.POST_Z), (K.POST_TOP[0], side * K.POST_Z)], False), filled=False, **TUBE))
        a, b = K.STRETCH_REAR, K.STRETCH_FRONT
        out.append(shape(f"stretcher-{k}", "stretcher", 1.5, T_TUBE, poly([(a[0], side * a[2]), (b[0], side * b[2])], False), filled=False, **TUBE))
    out.append(shape("seat-band", "seat", 3, T_SEAT, poly(plan_seat(0, notch=False)), **SEAT))
    out.append(shape("back-panel", "back", 6, T_BACK, plan_flat(K.BOARD_U0, K.BOARD_U1, K.SLAB500_V0, K.SLAB500_V1 - K.SLAB500_V0), **BACK))
    return out


def T(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


DECISIONS = {
    "50": {
        "silhouette": T("Side elevation 17.6 x 9.7 mm: the raked 20 mm beech seat board over its apron rail, the rear tube rising from its glide, bending at the seat and running on as the reclined back post to its cap, the straight front leg on its glide, the stretcher at 0.15 m and the curved backrest board and lower rail seen edge-on in front of the post (near and far tubes coincide). Front view: the seat board and apron over the slightly splayed front legs, the rear tubes just inside them rising to the posts, the backrest board and rail across the posts. Plan: the trapezoidal seat board with rounded front corners and the notches round the posts, the curved backrest board and the lower rail behind it, the post tops with their caps, and the glides, legs and stretchers where they show past the seat.", "Seitenansicht 17,6 x 9,7 mm: das geneigte 20-mm-Buchensitzbrett über seiner Zargenleiste, das Hinterrohr, das von seinem Gleiter aufsteigt, am Sitz umbiegt und als geneigter Lehnenpfosten bis zur Kappe weiterläuft, das gerade Vorderbein auf seinem Gleiter, die Strebe auf 0,15 m und das gebogene Rückenbrett und die untere Leiste von der Kante gesehen vor dem Pfosten (nahe und ferne Rohre fallen zusammen). Vorderansicht: Sitzbrett und Zarge über den leicht gespreizten Vorderbeinen, die Hinterrohre knapp innen bis zu den Pfosten, Rückenbrett und Leiste über die Pfosten. Grundriss: das trapezförmige Sitzbrett mit gerundeten Vorderecken und den Ausklinkungen um die Pfosten, das gebogene Rückenbrett und dahinter die untere Leiste, die Pfostenenden mit Kappen sowie Gleiter, Beine und Streben, wo sie neben dem Sitz sichtbar sind."),
        "retained": T("Seat board, apron rail, rear tube with its bend, post with cap, front leg, glides, stretcher, curved backrest board and lower rail, grain.", "Sitzbrett, Zargenleiste, Hinterrohr mit Bogen, Pfosten mit Kappe, Vorderbein, Gleiter, Strebe, gebogenes Rückenbrett und untere Leiste, Maserung."),
        "merged": T("Nothing merged; near and far tubes are one projected tube each.", "Nichts verschmolzen; nahe und ferne Rohre sind je ein projiziertes Rohr."),
        "omitted": T("Seat and back screws and the welded tabs (5-6 mm, 0.1 mm on paper, and hidden under the board); the hidden apron rails in front view and plan; the stretchers in front view (behind the front legs).", "Sitz- und Rückenschrauben und die angeschweißten Laschen (5-6 mm, 0,1 mm auf Papier, und unter dem Brett verborgen); die verdeckten Zargenleisten in Vorderansicht und Grundriss; die Streben in der Vorderansicht (hinter den Vorderbeinen)."),
        "thinElements": T("The Ø22 tubes are 0.44 mm filled outlines with the 0.5 mm body pen, the Ø16 stretcher a 0.32 mm outline, the glides and caps detail shapes.", "Die Rohre Ø22 sind 0,44 mm breite gefüllte Umrisse mit dem 0,5-mm-Körperstift, die Strebe Ø16 ein 0,32-mm-Umriss, Gleiter und Kappen Detailformen."),
        "hatching": T("Grain only as two short 0.13 mm texture strokes on the seat (elevation and plan) and on the backrest board (front view).", "Maserung nur als zwei kurze 0,13-mm-Texturstriche auf dem Sitz (Aufriss und Grundriss) und auf dem Rückenbrett (Vorderansicht)."),
        "styles": T("Beech light (0.45-0.62), steel dark (0.3), nylon black (0.15); body 0.5, detail 0.25, texture 0.13.", "Buche hell (0,45-0,62), Stahl dunkel (0,3), Nylon schwarz (0,15); Körper 0,5, Detail 0,25, Textur 0,13."),
        "overlays": T("Seat board and apron are seat-beech, backrest board and rail back-beech, every tube steel-tube; glides and caps belong to no region.", "Sitzbrett und Zarge sind seat-beech, Rückenbrett und Leiste back-beech, jedes Rohr steel-tube; Gleiter und Kappen gehören zu keiner Region."),
    },
    "100": {
        "silhouette": T("Side elevation 8.8 x 4.9 mm: the seat board band over the apron, the rear tube and post as one bent stroke, the front leg stroke, the stretcher hairline, backrest board and rail edge-on. Front view: seat band and apron, front legs, the rear tubes rising to the posts behind, backrest board and rail. Plan: the notched seat board with rounded front corners, the curved backrest board and rail, posts and leg strokes where they show.", "Seitenansicht 8,8 x 4,9 mm: das Sitzbrettband über der Zarge, Hinterrohr und Pfosten als ein gebogener Strich, der Vorderbeinstrich, die Strebe als Haarlinie, Rückenbrett und Leiste von der Kante. Vorderansicht: Sitzband und Zarge, Vorderbeine, dahinter die Hinterrohre bis zu den Pfosten, Rückenbrett und Leiste. Grundriss: das ausgeklinkte Sitzbrett mit gerundeten Vorderecken, das gebogene Rückenbrett und die Leiste, Pfosten- und Beinstriche, wo sie sichtbar sind."),
        "retained": T("Seat board, apron, bent rear tube, front leg, stretcher, backrest board and rail.", "Sitzbrett, Zarge, gebogenes Hinterrohr, Vorderbein, Strebe, Rückenbrett und Leiste."),
        "merged": T("The four apron rails are one band; near and far tubes stay one projected line.", "Die vier Zargenleisten sind ein Band; nahe und ferne Rohre bleiben eine projizierte Linie."),
        "omitted": T("Glides and caps (8-10 mm), screws, grain.", "Gleiter und Kappen (8-10 mm), Schrauben, Maserung."),
        "thinElements": T("Tubes (22 mm) are single 0.35 mm body strokes (35 mm of real line weight against a 22 mm tube, within one 15 mm feature); the 16 mm stretcher is a 0.18 mm detail stroke; seat board (20 mm) and boards (18 mm) are filled.", "Rohre (22 mm) sind einzelne 0,35-mm-Körperstriche (35 mm reale Strichbreite gegenüber dem 22-mm-Rohr, innerhalb eines 15-mm-Mindestmaßes); die 16-mm-Strebe ist ein 0,18-mm-Detailstrich; Sitzbrett (20 mm) und Bretter (18 mm) sind gefüllt."),
        "hatching": T("None.", "Keine."),
        "styles": T("Beech tone 0.45-0.62; steel strokes dark; body pen 0.35, detail 0.18.", "Buche Ton 0,45-0,62; Stahlstriche dunkel; Körperstift 0,35, Detail 0,18."),
        "overlays": T("Strokes carry steel-tube as line colour; seat band and apron are seat-beech, backrest board and rail back-beech.", "Striche tragen steel-tube als Linienfarbe; Sitzband und Zarge sind seat-beech, Rückenbrett und Leiste back-beech."),
    },
    "200": {
        "silhouette": T("Side elevation 4.4 x 2.4 mm: seat band of board and apron, one bent stroke for rear leg and post, the front leg stroke, the stretcher hairline, backrest board and rail as two bars in front of the post. Front view: seat band and apron, leg and post strokes, backrest board and rail. Plan: the trapezoidal seat, the backrest board as a bar behind it, post and stretcher strokes.", "Seitenansicht 4,4 x 2,4 mm: Sitzband aus Brett und Zarge, ein gebogener Strich für Hinterbein und Pfosten, der Vorderbeinstrich, die Strebe als Haarlinie, Rückenbrett und Leiste als zwei Balken vor dem Pfosten. Vorderansicht: Sitzband und Zarge, Bein- und Pfostenstriche, Rückenbrett und Leiste. Grundriss: der trapezförmige Sitz, das Rückenbrett als Balken dahinter, Pfosten- und Strebenstriche."),
        "retained": T("Seat, apron, the bent rear tube, front leg, stretcher, backrest board and rail.", "Sitz, Zarge, das gebogene Hinterrohr, Vorderbein, Strebe, Rückenbrett und Leiste."),
        "merged": T("Rear leg and post are one stroke; board and apron are 30 mm bands; the back boards are flat.", "Hinterbein und Pfosten sind ein Strich; Brett und Zarge sind 30-mm-Bänder; die Rückenbretter sind eben."),
        "omitted": T("Glides, caps, notches, corner radii, the board curve (26 mm), screws, grain; the back rail in plan (under the board edge).", "Gleiter, Kappen, Ausklinkungen, Eckradien, die Brettkrümmung (26 mm), Schrauben, Maserung; die Rückenleiste im Grundriss (unter der Brettkante)."),
        "thinElements": T("Tubes are single 0.25 mm strokes (50 mm of real line weight; the 3D tubes stay at their true 22 mm as slender solids, C24); the stretcher a 0.13 mm detail stroke; seat board and back boards are thickened from 18-20 to 30 mm (0.15 mm minimum feature).", "Rohre sind einzelne 0,25-mm-Striche (50 mm reale Strichbreite; die 3D-Rohre bleiben als schlanke Körper in echter Größe 22 mm, C24); die Strebe ein 0,13-mm-Detailstrich; Sitzbrett und Rückenbretter werden von 18-20 auf 30 mm verstärkt (0,15 mm Mindestmaß)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Beech mid tone, strokes dark; body pen 0.25, detail 0.13.", "Buche mittlerer Ton, Striche dunkel; Körperstift 0,25, Detail 0,13."),
        "overlays": T("Seat band and apron are seat-beech, back boards back-beech, strokes steel-tube.", "Sitzband und Zarge sind seat-beech, Rückenbretter back-beech, Striche steel-tube."),
    },
    "500": {
        "silhouette": T("Chair sign 1.8 x 1.0 mm: a seat slab, a back slab on the leaning rear stroke, a front leg stroke. Front view: seat slab, back slab, four leg strokes. Plan: seat slab and back slab with the post strokes behind.", "Stuhlzeichen 1,8 x 1,0 mm: eine Sitzplatte, eine Rückenplatte am geneigten Hinterstrich, ein Vorderbeinstrich. Vorderansicht: Sitzplatte, Rückenplatte, vier Beinstriche. Grundriss: Sitzplatte und Rückenplatte mit den Pfostenstrichen dahinter."),
        "retained": T("Overall height and depth, seat height, the raked rear legs and the reclined back.", "Gesamthöhe und -tiefe, Sitzhöhe, die ausgestellten Hinterbeine und die geneigte Lehne."),
        "merged": T("Seat board and apron become one 75 mm slab; backrest board and posts one 75 mm back slab; the bend is a corner.", "Sitzbrett und Zarge werden eine 75-mm-Platte; Rückenbrett und Pfosten eine 75-mm-Rückenplatte; der Bogen ist eine Ecke."),
        "omitted": T("Stretchers (16 mm, 0.03 mm on paper), back rail, glides, caps, all details.", "Streben (16 mm, 0,03 mm auf Papier), Rückenleiste, Gleiter, Kappen, alle Details."),
        "thinElements": T("Seat and back are thickened to 75 mm (0.15 mm minimum feature); tubes are single 0.18 mm strokes (90 mm of real line weight; the 3D tubes stay at true size as slender solids).", "Sitz und Lehne werden auf 75 mm verstärkt (0,15 mm Mindestmaß); Rohre sind einzelne 0,18-mm-Striche (90 mm reale Strichbreite; die 3D-Rohre bleiben als schlanke Körper in echter Größe)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Beech mid tone, strokes dark; one 0.18 mm weight.", "Buche mittlerer Ton, Striche dunkel; eine Strichstärke 0,18 mm."),
        "overlays": T("Seat slab seat-beech, back slab back-beech, strokes steel-tube.", "Sitzplatte seat-beech, Rückenplatte back-beech, Striche steel-tube."),
    },
}


def apply(asset):
    """✏ Writes the shapes and representation decisions of every scale into the asset in memory."""
    reps = asset["representations"]
    for scale in ("50", "100", "200", "500"):
        g = globals()
        reps[scale]["geometry"] = {"elevation": g["elevation" + scale](), "front": g["front" + scale](), "plan": g["plan" + scale]()}
        reps[scale]["decisions"] = DECISIONS[scale]
    return asset
