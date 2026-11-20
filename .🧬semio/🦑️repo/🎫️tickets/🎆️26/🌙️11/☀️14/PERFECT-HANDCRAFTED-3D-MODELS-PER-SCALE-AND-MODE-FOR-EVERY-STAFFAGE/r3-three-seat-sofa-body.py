"""🛋 Round 3, furniture/three-seat-sofa: the body list of every scale, solid by solid, from the station tables of the kit.

1:50 (7.5 mm): turned feet with a bead; the plinth as a closed upholstered box with a round front edge; box arms with 60 mm top rolls; the leaning back frame with its rolled top; three crowned seat cushions with top and bottom piping; three pillow back cushions. 1:100 (15 mm): the same masses with fewer stations, a solid plinth, plain tapered feet and no piping. 1:200 (30 mm): chamfered rolls, a square plinth, straight feet. 1:500 (75 mm): one seat block, one back block, two arm blocks and four 75 mm feet.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_three_seat_sofa_kit", os.path.join(HERE, "r3-three-seat-sofa-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

T_ARM, T_SEAT, T_BACK, T_FRAME, T_BASE, T_FOOT = 0.5, 0.72, 0.62, 0.55, 0.45, 0.4
ARMS = (("arm-l", "arm-left", 0.1), ("arm-r", "arm-right", 1.9))


def feet(scale):
    """🪵 The four feet: one revolve per end, repeated across the depth."""
    out = []
    for name, x in (("foot-l", K.FOOT_X[0]), ("foot-r", K.FOOT_X[1])):
        if scale == "500":
            f = K.box(name, "frame", K.BEECH, T_FOOT, (x - 0.0375, x + 0.0375), (0.0, K.FOOT_H), (K.FOOT_Z - 0.0375, K.FOOT_Z + 0.0375))
        else:
            f = K.turned(name, "frame", K.BEECH, T_FOOT, K.FOOT[scale], (x, 0.0, K.FOOT_Z))
        out.append(K.repeat(f, 2, dw=-2 * K.FOOT_Z))
    return out


def arms(scale):
    """🟪 The two box arms."""
    out = []
    for sid, bone, cx in ARMS:
        if scale == "500":
            out.append(K.box(sid, bone, K.FABRIC, T_ARM, (cx - 0.1, cx + 0.1), K.ARM_Y, K.ARM_Z))
        else:
            out.append(K.yloft(sid, bone, K.FABRIC, T_ARM, cx, K.arm_stations(scale), 10))
    return out


def cushions(scale):
    """🟫 The three seat cushions and the three back cushions on their own bones."""
    out = []
    for k in range(3):
        cx = (K.SEAT_X[k] + K.SEAT_X[k + 1]) / 2
        out.append(K.yloft(f"seat-{k + 1}", f"seat-{k + 1}", K.FABRIC, T_SEAT, cx, K.seat_stations(scale), 10))
        out.append(K.yloft(f"back-{k + 1}", f"back-{k + 1}", K.FABRIC, T_BACK, cx, K.back_stations(scale), 5))
    return out


def piping():
    """🧵 The 8 mm piping beads round the top and bottom boxing seams of every seat cushion (1:50 only), proud of the boxing by 4 mm."""
    out = []
    hx = (K.SEAT_X[1] - K.SEAT_X[0]) / 2
    zc, hz = (K.SEAT_Z[0] + K.SEAT_Z[1]) / 2, (K.SEAT_Z[1] - K.SEAT_Z[0]) / 2
    for k in range(3):
        cx = (K.SEAT_X[k] + K.SEAT_X[k + 1]) / 2
        ring = K.ring_plan(f"piping-{k + 1}", f"seat-{k + 1}", K.FABRIC, 0.35, cx, zc, (hx - 0.002, hz + 0.004), (hx - 0.012, hz - 0.006), (0.396, 0.404), 10, texture=True)
        out.append(K.repeat(ring, 2, dv=-0.15))
    return out


def body(scale):
    """🧩 The body list of a scale."""
    out = feet(scale)
    if scale == "500":
        out.append(K.box("seat-block", "frame", K.FABRIC, T_SEAT, K.BASE_X, (K.BASE_Y[0], K.SEAT_TOP), (K.BASE_Z[0], K.SEAT_Z[1])))
        prof = [(K.DECK_Y, K.SEAT_Z[1]), (K.DECK_Y, 0.45), (K.FRAME_TOP, 0.45), (K.FRAME_TOP, 0.33), (0.78, 0.22), (K.SEAT_TOP, 0.15), (K.SEAT_TOP, K.SEAT_Z[1])]
        out.append(K.along_x("back-block", "frame", K.FABRIC, T_BACK, prof, K.BASE_X))
        out += arms(scale)
        return out
    out.append(K.along_x("base", "frame", K.FABRIC, T_BASE, K.base_profile(scale), K.BASE_X))
    out.append(K.yloft("back-frame", "frame", K.FABRIC, T_FRAME, 1.0, K.backframe_stations(scale), 12))
    out += arms(scale)
    out += cushions(scale)
    if scale == "50":
        out += piping()
    return out


def body50():
    """🔍 1:50."""
    return body("50")


def body100():
    """🔎 1:100."""
    return body("100")


def body200():
    """🔭 1:200."""
    return body("200")


def body500():
    """🛰 1:500."""
    return body("500")
