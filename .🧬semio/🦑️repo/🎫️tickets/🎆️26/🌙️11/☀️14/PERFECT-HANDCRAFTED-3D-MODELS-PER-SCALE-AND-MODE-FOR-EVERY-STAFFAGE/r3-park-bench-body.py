"""🪑 Round 3, furniture/park-bench: every body solid of every scale, stated by hand from the measures of the kit.

1:50 shows the whole construction: two one-piece cast end frames of I-section (40 mm flange rims around the outline and both openings, 12 mm web), each with front leg and arm post in one, raked seat rail, back post raked 8 degrees, rear leg kicked back, lower brace, arm band under a 50 x 35 mm rounded armrest with a knuckle, and a foot pad under each leg; four 100 x 45 mm oak seat slats and three 80 x 35 mm oak back slats with eased edges; the domed heads of the carriage bolts that hold every slat on each frame.
"""
import importlib.util
import os

from shapely.geometry import Polygon
from shapely.ops import unary_union

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_park_bench_kit", os.path.join(HERE, "r3-park-bench-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

ENDS = (("l", "end-left"), ("r", "end-right"))
TONE = {"rim": 0.42, "web": 0.32, "arm": 0.45, "pad": 0.3, "seat": 0.62, "back": 0.58, "bolt": 0.15}


def r6(v):
    """🔢 Short for the kit's rounding."""
    return K.r6(v)


def armrest(sid, bone, half_h, half_w, knuckle=True, exponent=2.6):
    """🦾 The armrest: a rounded bar along the frame from its knuckle at the front to 12 mm inside the back post, centred on the arm band."""
    yc = (K.ARM_Y0 + K.ARM_Y1) / 2
    stations = [(K.ARM_FRONT, 0.35, 0.4), (K.ARM_FRONT + 0.003, 0.75, 0.78), (K.ARM_FRONT + 0.009, 0.94, 0.94), (K.ARM_FRONT + 0.018, 1.0, 1.0)] if knuckle else [(K.ARM_FRONT, 1.0, 1.0)]
    stations.append((K.arm_reach(), 1.0, 1.0))
    out = K.base(sid, "loft", bone, K.STEEL, TONE["arm"])
    out["sections"] = [{"at": [r6(yc), 0, r6(z)], "rx": r6(half_h * a), "ry": r6(half_w * b), "exponent": exponent} for z, a, b in stations]
    out["capStart"] = True
    out["capEnd"] = True
    return out


def pads(sid, bone, height, length, width, radius=0.004, tone=TONE["pad"], role="body"):
    """🦶 The foot pads under the front and the rear leg."""
    out = K.base(sid, "box", bone, K.STEEL, tone, role)
    out["size"] = [r6(height), r6(width), r6(length)]
    if radius:
        out["cornerRadius"] = radius
    out["transform"] = {"translation": [r6(height / 2), 0, r6(K.FRONT_Z)]}
    out["repeat"] = {"count": 2, "translation": [0, 0, r6(K.REAR_FOOT_Z - K.FRONT_Z)]}
    return out


def seat_slats(radius, oy):
    """💺 The four seat slats as one repeated solid along the rake."""
    s = K.along("seat-slat", "seat", oy, K.seat_slats(radius)[0], TONE["seat"])
    step = K.SEAT_W + K.SEAT_GAP
    s["repeat"] = {"count": K.SEAT_N, "translation": [0, r6(K.D[1] * step), r6(K.D[0] * step)]}
    return s


def back_slats(radius, oy):
    """🪑 The three back slats."""
    return [K.along(f"back-slat-{k + 1}", "backrest", oy, sec, TONE["back"]) for k, sec in enumerate(K.back_slats(radius))]


def dome(sid, bone, origin, axis, tone=TONE["bolt"]):
    """🔘 A carriage bolt head: a 19 mm dome 5.5 mm high on the slat face, sunk 0.5 mm."""
    out = K.base(sid, "revolve", bone, K.STEEL, tone, "detail")
    out.update({"profile": [[-0.0005, 0.0095], [0.0015, 0.009], [0.0035, 0.0068], [0.0048, 0.0038], [0.0053, 0.0]], "axis": [0, r6(axis[1]), r6(axis[0])], "origin": [r6(origin[0]), r6(origin[1]), r6(origin[2])], "texture": True})
    return out


def seat_bolts(oy):
    """🔘 The bolt heads on the seat slats over each frame."""
    out = []
    step = K.SEAT_W + K.SEAT_GAP
    s0, s1 = K.SEAT_S[0]
    z, y = K.seat_point((s0 + s1) / 2, K.SEAT_T)
    for p, x in zip(("l", "r"), K.FRAME_X):
        b = dome(f"seat-bolt-{p}", "seat", (x, y - oy, z), K.N)
        b["repeat"] = {"count": K.SEAT_N, "translation": [0, r6(K.D[1] * step), r6(K.D[0] * step)]}
        out.append(b)
    return out


def back_bolts(oy):
    """🔘 The bolt heads on the back slats over each frame."""
    out = []
    fwd = (K.M[0], K.M[1])
    for k, (t0, t1) in enumerate(K.BACK_S):
        z, y = K.add(K.post_front((t0 + t1) / 2), K.M, K.BACK_T)
        b = dome(f"back-bolt-{k + 1}", "backrest", (K.FRAME_X[0], y - oy, z), fwd)
        b["repeat"] = {"count": 2, "translation": [r6(K.FRAME_X[1] - K.FRAME_X[0]), 0, 0]}
        out.append(b)
    return out


def body50():
    """🔍 1:50, smallest feature 7.5 mm."""
    sy, by = K.seat_y(), K.back_y()
    profile = K.frame_profile(True)
    core, rims = K.i_section(profile)
    out = []
    for p, bone in ENDS:
        out.append(K.end_extrude(f"{p}-web", bone, core, K.WEB / 2, TONE["web"]))
        for k, piece in enumerate(sorted(rims, key=lambda g: -g.area)):
            out.append(K.end_extrude(f"{p}-rim-{k + 1}", bone, piece, K.FLANGE / 2, TONE["rim"]))
        out.append(armrest(f"{p}-armrest", bone, 0.0175, 0.025))
        out.append(pads(f"{p}-pads", bone, K.PAD_H, K.PAD_L, K.PAD_X))
    out.append(seat_slats(K.SLAT_R, sy))
    out += back_slats(K.SLAT_R, by)
    out += seat_bolts(sy) + back_bolts(by)
    return out


def body100():
    """🔎 1:100, smallest feature 15 mm: the casting as one 40 mm plate (the 15 mm recess of the web is below the feature), square slats."""
    sy, by = K.seat_y(), K.back_y()
    profile = K.frame_profile(True, 0.008)
    out = []
    for p, bone in ENDS:
        out.append(K.end_extrude(f"{p}-frame", bone, profile, K.FLANGE / 2, TONE["rim"]))
        out.append(armrest(f"{p}-armrest", bone, 0.0175, 0.025, knuckle=False, exponent=4))
        out.append(pads(f"{p}-pads", bone, K.PAD_H, K.PAD_L, K.PAD_X, radius=0))
    out.append(seat_slats(0, sy))
    out += back_slats(0, by)
    return out


def coarse_profile(floor=True):
    """🔩 The 1:200 casting: members of at least 30 mm, legs to the floor with their feet, arm band as the armrest (the 5 mm the armrest stands proud is below the feature)."""
    parts = K.frame_parts(False)
    fz = K.FRONT_Z
    feet = [K.quad([(fz - 0.035, 0.0), (fz + 0.035, 0.0), (fz + 0.025, 0.03), (fz - 0.025, 0.03)]), K.quad([(K.REAR_FOOT_Z - 0.035, 0.0), (K.REAR_FOOT_Z + 0.035, 0.0), (K.REAR_FOOT_Z + 0.025, 0.03), (K.REAR_FOOT_Z - 0.025, 0.03)])]
    arm = K.quad([(K.ARM_FRONT, K.ARM_Y0), (K.arm_reach(), K.ARM_Y0), (K.arm_reach(), K.ARM_Y1), (K.ARM_FRONT, K.ARM_Y1)])
    return unary_union(parts + feet + [arm]).buffer(0.01, join_style=1).buffer(-0.02, join_style=1).buffer(0.01, join_style=1).simplify(0.001)


def body200():
    """🗺 1:200, smallest feature 30 mm: casting as one 40 mm plate with legs to the floor; seat as one raked board (the 10 mm gaps are below the feature); back as the lumbar slat and one board of the two upper slats (30 mm gap merged), the armrest sits in the 90 mm gap between them."""
    sy, by = K.seat_y(), K.back_y()
    out = []
    for p, bone in ENDS:
        out.append(K.end_extrude(f"{p}-frame", bone, coarse_profile(), K.FLANGE / 2, TONE["rim"]))
    s0, s1 = K.SEAT_S[0][0], K.SEAT_S[-1][1]
    out.append(K.along("seat-board", "seat", sy, K.slat_section(K.seat_point(s0), K.D, K.N, s1 - s0, K.SEAT_T, 0), TONE["seat"]))
    for k, (t0, t1) in enumerate([K.BACK_S[0], (K.BACK_S[1][0], K.BACK_S[2][1])]):
        sec = K.slat_section(K.add(K.post_front(t0), K.M, K.BACK_T), K.E, (-K.M[0], -K.M[1]), t1 - t0, K.BACK_T, 0)
        out.append(K.along(f"back-board-{k + 1}", "backrest", by, sec, TONE["back"]))
    return out


SEAT500 = (0.375, 0.45)
FRAME500 = 0.075


def back500():
    """🪑 The 1:500 back: posts and slats in one 75 mm slab, from the posts' back face forward, from the seat top to the top of the back."""
    t0 = (SEAT500[1] - K.POST_FOOT[1]) / K.E[1]
    a, b = K.post_back(t0), K.post_back(K.POST_TOP)
    sec = [a, b, K.add(b, K.M, FRAME500), K.add(a, K.M, FRAME500)]
    sec[3] = (sec[3][0], SEAT500[1])
    return sec


def profile500():
    """🔩 The 1:500 casting: 75 mm members: front column up to the armrest, seat rail under the seat slab, rear leg up into the back slab, arm band from the column to the back slab; the posts are part of the back slab; the openings under the seat and between seat and armrest stay."""
    fz0, fz1 = K.ARM_FRONT, K.ARM_FRONT + FRAME500
    foot = K.REAR_FOOT_Z + 0.04
    back = back500()
    t_arm = (K.ARM_Y1 - K.POST_FOOT[1]) / K.E[1]
    arm_end = K.add(K.post_back(t_arm), K.M, FRAME500)[0] + 0.01
    parts = [
        K.quad([(fz0, 0.0), (fz1, 0.0), (fz1, K.ARM_Y1), (fz0, K.ARM_Y1)]),
        K.quad([(fz0, SEAT500[0] - FRAME500), (0.2, SEAT500[0] - FRAME500), (0.2, SEAT500[0]), (fz0, SEAT500[0])]),
        K.quad([(foot - FRAME500, 0.0), (foot, 0.0), K.add(back[0], K.E, 0.02), K.add(back[3], K.E, 0.02)]),
        K.quad([(fz0, K.ARM_Y1 - FRAME500), (arm_end, K.ARM_Y1 - FRAME500), (arm_end, K.ARM_Y1), (fz0, K.ARM_Y1)]),
    ]
    return unary_union(parts).buffer(0.015, join_style=1).buffer(-0.03, join_style=1).buffer(0.015, join_style=1).simplify(0.002)


def body500():
    """🌍 1:500, smallest feature 75 mm: each end frame a 75 mm cast plate of 75 mm members; the seat a flat 75 mm slab (the 38 mm rake is below the feature); the back one raked 75 mm slab from the seat to the top."""
    sy, by = K.seat_y(), K.back_y()
    out = []
    for p, bone in ENDS:
        out.append(K.end_extrude(f"{p}-frame", bone, profile500(), FRAME500 / 2, TONE["rim"]))
    s0, s1 = K.NOSE[0], K.seat_point(K.SEAT_S[-1][1])[0]
    out.append(K.along("seat-slab", "seat", sy, [(s0, SEAT500[0]), (s1, SEAT500[0]), (s1, SEAT500[1]), (s0, SEAT500[1])], TONE["seat"]))
    out.append(K.along("back-slab", "backrest", by, back500(), TONE["back"]))
    return out


if __name__ == "__main__":
    for name in ("body50", "body100", "body200", "body500"):
        solids = globals()[name]()
        print(name, len(solids), sum(s.get("repeat", {}).get("count", 1) for s in solids))
    print(profile500().bounds, coarse_profile().bounds)
