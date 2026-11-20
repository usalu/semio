"""✏️ Round 3, furniture/bar-stool: the 2D drawings of every scale and view, drawn by hand from the same construction as the solids (ruling E1): frame parts as true outlines, seat, cushion and backrest as revolved or extruded solids of their profiles so they swivel and sink with their bones.
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("model", os.path.join(HERE, "r3-bar-stool-model.py"))
M = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(M)
K = M.K
R = K.rnd


def pts(seq):
    return [[R(x), R(y)] for x, y in seq]


def shape(sid, bone, role, z, filled, tone, kind, extra, region=None, material=None, pen=None, regions=None):
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    if region is not None:
        s["region"] = region
    if regions is not None:
        s["regions"] = regions
    if material is not None:
        s["material"] = material
    if pen is not None:
        s["penMm"] = pen
    s.update(extra)
    return s


def poly(seq, closed=True):
    return {"points": pts(seq), "closed": closed}


def capsule(x0, y0, x1, y1, r):
    return {"x0": R(x0), "y0": R(y0), "x1": R(x1), "y1": R(y1), "r0": R(r), "r1": R(r)}


def revolved_seat(points):
    """🔄 Half profile (world height, radius) of a part on the seat axis as a seat-bone shape: x = height above the swivel joint, y = minus the radius (ahead), mirrored."""
    return {"points": pts([(h - K.SEAT_JOINT_Y, -r) for h, r in points]), "closed": True, "solid": {"kind": "revolved", "axisBone": "seat", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}}


def revolved_pad(points):
    """🔄 Half profile (world height, radius) of a part on the pad bone: x = radius, y = height above the pad joint, mirrored."""
    return {"points": pts([(r, h - K.PAD_JOINT_Y) for h, r in points]), "closed": True, "solid": {"kind": "revolved", "axisBone": "pad", "axis": "v", "axisX": 0, "axisZ": 0, "mirror": True}}


def extruded_back(world, z0, z1):
    """🏗 A side outline in world (x, y) as a backrest-bone shape extruded laterally from z0 to z1."""
    return {"points": [K.to_local("backrest", (x, y))[:2] for x, y in world], "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": R(z0), "z1": R(z1)}}


def annulus(r_out, r_in):
    """⭕ Even-odd path of a ring seen from above: outer circle and the hole."""
    a, b = R(r_out), R(r_in)
    return f"M {a} 0 A {a} {a} 0 1 0 {-a} 0 A {a} {a} 0 1 0 {a} 0 Z M {b} 0 A {b} {b} 0 1 0 {-b} 0 A {b} {b} 0 1 0 {b} 0 Z"


def glide_side(cx):
    p = M.glide_profile()
    right = [(cx + r, h) for h, r in p[1:-1]]
    left = [(cx - r, h) for h, r in reversed(p[1:-1])]
    return right + left


def pan_side(top, bottom, r, round_r):
    return [(-r, bottom), (r, bottom)] + [(x, y) for y, x in K.arc(top - round_r, r - round_r, round_r, 0, 90, 3)] + [(-x, y) for y, x in reversed(K.arc(top - round_r, r - round_r, round_r, 0, 90, 3))]


def frame_views(scale):
    """🦿 Frame shapes (legs, glides, ring, pan) per view; the frame bone is the world frame."""
    pen = {50: 0.25, 100: 0.25}.get(scale)
    lt, lf = K.LEG_TOP, (K.LEG_FOOT if scale == 50 else (K.FOOT_R, K.TUBE_R))
    el, fr, pl = [], [], []
    if scale in (50, 100):
        legs = [("leg-l", -1), ("leg-r", 1)]
        for sid, sgn in legs:
            el.append(shape(sid, "frame", "body", 2, True, 0.7, "capsule", capsule(sgn * lt[0], lt[1], sgn * lf[0], lf[1], K.TUBE_R), "legs", "chrome-steel", pen))
            fr.append(shape(sid, "frame", "body", 2, True, 0.7, "capsule", capsule(-sgn * lt[0], lt[1], -sgn * lf[0], lf[1], K.TUBE_R), "legs", "chrome-steel", pen))
        for v in (el, fr):
            v.append(shape("leg-m", "frame", "body", 2, True, 0.7, "capsule", capsule(0, lt[1], 0, lf[1], K.TUBE_R), "legs", "chrome-steel", pen))
        for sid, sx, sy in (("leg-xp", 1, 0), ("leg-xn", -1, 0), ("leg-yp", 0, 1), ("leg-yn", 0, -1)):
            pl.append(shape(sid, "frame", "body", 2, True, 0.7, "capsule", capsule(sx * lt[0], sy * lt[0], sx * lf[0], sy * lf[0], K.TUBE_R), "legs", "chrome-steel", pen))
        if scale == 50:
            for sid, cx in (("ferrule-l", -K.FOOT_R), ("ferrule-r", K.FOOT_R), ("ferrule-m", 0.0)):
                el.append(shape(sid, "frame", "body", 2.5, True, 0.15, "poly", poly(glide_side(cx)), "legs", "rubber", pen))
                fr.append(shape(sid, "frame", "body", 2.5, True, 0.15, "poly", poly(glide_side(-cx)), "legs", "rubber", pen))
            for sid, sx, sy in (("ferrule-xp", 1, 0), ("ferrule-xn", -1, 0), ("ferrule-yp", 0, 1), ("ferrule-yn", 0, -1)):
                pl.append(shape(sid, "frame", "body", 2.5, True, 0.15, "ellipse", {"cx": R(sx * K.FOOT_R), "cy": R(sy * K.FOOT_R), "rx": K.GLIDE_R, "ry": K.GLIDE_R}, "legs", "rubber", pen))
        for v in (el, fr):
            v.append(shape("ring", "frame", "body", 3, True, 0.7, "capsule", capsule(-K.RING_R, K.RING_Y, K.RING_R, K.RING_Y, K.TUBE_R), "footrest-ring", "chrome-steel", pen))
        pl.insert(0, shape("ring", "frame", "body", 1, True, 0.7, "path", {"d": annulus(K.RING_R + K.TUBE_R, K.RING_R - K.TUBE_R)}, "footrest-ring", "chrome-steel", pen))
        if scale == 50:
            side = pan_side(K.PAN_TOP, K.PAN_TOP - K.PAN_RIM, K.PAN_R, 0.006)
        else:
            top = K.SEAT_JOINT_Y - 0.015
            side = [(-K.PAN_R, top - 0.017), (K.PAN_R, top - 0.017), (K.PAN_R, top - 0.004), (K.PAN_R - 0.004, top), (-K.PAN_R + 0.004, top), (-K.PAN_R, top - 0.004)]
        for v in (el, fr):
            v.append(shape("pan", "frame", "body", 4, True, 0.55, "poly", poly(side), "seat-mount", "chrome-steel", pen))
    else:
        for sid, sgn in (("leg-l", -1), ("leg-r", 1)):
            if scale == 200:
                el.append(shape(sid, "frame", "body", 2, True, 0.3, "capsule", capsule(sgn * 0.146, 0.676, sgn * K.FOOT_R, K.TUBE_R, K.TUBE_R), "legs", "chrome-steel"))
                fr.append(shape(sid, "frame", "body", 2, True, 0.3, "capsule", capsule(-sgn * 0.146, 0.676, -sgn * K.FOOT_R, K.TUBE_R, K.TUBE_R), "legs", "chrome-steel"))
            else:
                el.append(shape(sid, "frame", "body", 2, False, 0.3, "poly", poly([(sgn * 0.146, 0.676), (sgn * K.FOOT_R, K.TUBE_R)], False), "legs", "chrome-steel"))
                fr.append(shape(sid, "frame", "body", 2, False, 0.3, "poly", poly([(-sgn * 0.146, 0.676), (-sgn * K.FOOT_R, K.TUBE_R)], False), "legs", "chrome-steel"))
        for v in (el, fr):
            if scale == 200:
                v.append(shape("leg-m", "frame", "body", 2, True, 0.3, "capsule", capsule(0, 0.676, 0, K.TUBE_R, K.TUBE_R), "legs", "chrome-steel"))
            else:
                v.append(shape("leg-m", "frame", "body", 2, False, 0.3, "poly", poly([(0, 0.676), (0, K.TUBE_R)], False), "legs", "chrome-steel"))
            v.append(shape("ring", "frame", "body", 3, True, 0.3, "capsule", capsule(-K.RING_R, K.RING_Y, K.RING_R, K.RING_Y, K.TUBE_R), "footrest-ring", "chrome-steel"))
        pl.append(shape("ring", "frame", "body", 1, True, 0.7, "ellipse", {"cx": 0, "cy": 0, "rx": K.RING_R + K.TUBE_R, "ry": K.RING_R + K.TUBE_R}, "footrest-ring", "chrome-steel"))
        if scale == 200:
            for v in (el, fr):
                v.append(shape("mount", "frame", "body", 4, True, 0.55, "poly", poly([(-K.PAN_R, 0.666), (K.PAN_R, 0.666), (K.PAN_R, K.SEAT_JOINT_Y), (-K.PAN_R, K.SEAT_JOINT_Y)]), "seat-mount", "chrome-steel"))
    return el, fr, pl


def seat_shapes(scale):
    """🪑 Shapes of the swivelling and sinking parts (seat, pad, backrest bones); one definition serves all three views, z per view."""
    out = []
    if scale == 50:
        out.append(("swivel", 5, 5, 4, shape("swivel", "seat", "body", 5, True, 0.35, "poly", revolved_seat([(K.SEAT_JOINT_Y - 0.012, 0.0), (K.SEAT_JOINT_Y - 0.012, K.SWIVEL_R), (K.SEAT_JOINT_Y, K.SWIVEL_R), (K.SEAT_JOINT_Y, 0.0)]), "seat-mount", "chrome-steel", 0.25)))
    elif scale == 100:
        out.append(("swivel", 5, 5, 4, shape("swivel", "seat", "body", 5, True, 0.35, "poly", revolved_seat(M.swivel_profile(100)), "seat-mount", "chrome-steel", 0.25)))
    if scale in (50, 100, 200):
        prof = M.cushion_profile(scale)
        out.append(("pad", 6, 6, 6, shape("pad", "pad", "body", 6, True, 0.3, "poly", revolved_pad(prof), "seat-pad", "pu-leatherette", 0.25 if scale <= 100 else None)))
    else:
        prof = [(0.666, 0.0), (0.666, K.PAN_R), (0.690, K.SEAT_R), (0.728, K.SEAT_R), (0.746, 0.17), (K.SEAT_TOP, 0.14), (K.SEAT_TOP, 0.0)]
        out.append(("seat", 6, 6, 6, shape("seat", "seat", "body", 6, True, 0.35, "poly", revolved_seat(prof), None, "pu-leatherette", regions=["seat-pad", "seat-mount"])))
    if scale == 50:
        bar, bz, bar_top, bend, sx = K.BAR_T, K.BAR_W / 2, K.BAR_TOP_Y, 0.012, K.BAR_START_X
    elif scale == 100:
        bar, bz, bar_top, bend, sx = 0.015, K.BAR_W / 2, 0.83, 0.0125, K.BAR_START_X
    elif scale == 200:
        bar, bz, bar_top, bend, sx = 0.03, K.BAR_W / 2, 0.82, 0.016, -0.11
    if scale in (50, 100, 200):
        out.append(("back-post", 4, 4, 5, shape("back-post", "backrest", "body", 4, True, 0.7, "poly", extruded_back(K.bar_outline(bar, start_x=sx, top_y=bar_top, bend=bend), -bz, bz), "legs", "chrome-steel", 0.25 if scale <= 100 else None)))
        t = K.BACK_HALF_T if scale < 200 else 0.015
        r_in, r_out = K.BACK_BEND_R - t, K.BACK_BEND_R + t
        half = r_out * math.sin(K.BACK_HALF_RAD)
        out.append(("back-pad", 5, 5, 8, shape("back-pad", "backrest", "body", 5, True, 0.3, "poly", extruded_back(K.back_side_outline(r_in, r_out, K.BACK_Y0, K.BACK_Y1, K.BACK_HALF_RAD, 0.012, K.BACK_CENTRE_X), -half, half), "backrest", "pu-leatherette", 0.25 if scale <= 100 else None)))
    else:
        half = 0.215 * math.sin(0.78)
        out.append(("back", 7, 7, 8, shape("back", "backrest", "body", 7, True, 0.35, "poly", extruded_back(K.back_side_outline(0.14, 0.215, 0.71, 0.87, 0.78, 0.025), -half, half), "backrest", "pu-leatherette")))
    return out


def piping(scale):
    """🧵 The piping seam of the 1:50 cushion: a stroke across the side and front views, a circle in plan."""
    if scale != 50:
        return [], [], []
    y = R(0.7263 - K.PAD_JOINT_Y)
    r = 0.1793
    side = shape("piping", "pad", "detail", 6.5, False, 0.2, "poly", poly([(-r, y), (r, y)], False), "seat-pad", "pu-leatherette", 0.13)
    front = shape("piping", "pad", "detail", 6.5, False, 0.2, "poly", poly([(-r, 0.7263), (r, 0.7263)], False), "seat-pad", "pu-leatherette", 0.13)
    plan = shape("piping", "pad", "detail", 6.5, False, 0.2, "ellipse", {"cx": 0, "cy": 0, "rx": 0.1763, "ry": 0.1763}, "seat-pad", "pu-leatherette", 0.13)
    return [side], [front], [plan]


def geometry(scale):
    """🗺 The `geometry` member of one representation."""
    el, fr, pl = frame_views(scale)
    for sid, ze, zf, zp, s in seat_shapes(scale):
        el.append(dict(s, z=ze))
        fr.append(dict(s, z=zf))
        pl.append(dict(s, z=zp))
    a, b, c = piping(scale)
    return {"elevation": el + a, "front": fr + b, "plan": pl + c}
