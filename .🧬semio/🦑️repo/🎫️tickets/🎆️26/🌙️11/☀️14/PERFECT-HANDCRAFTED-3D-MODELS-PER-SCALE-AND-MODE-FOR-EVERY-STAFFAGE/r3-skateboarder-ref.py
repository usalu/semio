"""📏 Round 3 reference numbers of humans/skateboarder: the rider's joints as shares of his stature and the street skateboard's real geometry, in board metres (x ahead along the deck from its centre, y up from the ground under the wheels, z left).

Sources: brief anthropometric shares (hip 0.52 H, knee 0.285 H, ankle 0.039 H, shoulder 0.818 H, elbow 0.63 H, wrist 0.485 H), Drillis and Contini 1964, de Leva 1996 (male segment masses and centres, head-and-neck 0.1395 H from C7), ANSUR II male (Gordon et al. 2014: biacromial 0.405 m, hip joint spacing 0.17 to 0.18 m, foot 0.152 H), Bell et al. 1990 (hip joint centres), Vans and Nike SB vulcanised skate shoes (heel sole 14 mm, forefoot 11 mm), skateboard catalogue data (8.0 x 31.9 in seven-ply maple deck, wheelbase 14.5 in between the inner truck holes, 2 1/8 in hole pattern, 8.0 in axle trucks of 53 mm height, 54 mm 99A wheels 32 mm wide with 608 bearings).
"""
import math

H = 1.78
SOLE = 0.014
SOLE_BALL = 0.011
HIP = round(0.52 * H + SOLE, 4)
KNEE = round(0.285 * H + SOLE, 4)
ANKLE = round(0.039 * H + SOLE, 4)
THIGH = round(HIP - KNEE, 4)
SHANK = round(KNEE - ANKLE, 4)
HIP_Z = 0.087
PELVIS = 0.094
SACRUM_BACK = 0.05
C7 = round(0.861 * H + SOLE, 4)
TORSO = round(C7 - HIP - PELVIS, 4)
SHOULDER = round(0.818 * H + SOLE, 4)
SHOULDER_X = round(SHOULDER - HIP - PELVIS, 4)
SHOULDER_FWD = 0.05
GIRDLE = "shoulder-girdle"
GIRDLE_X = 0.42
GIRDLE_LEN = 0.12
GAZE = "gaze"
GAZE_X = 0.13
GAZE_LEN = 0.01
SHOULDER_Z = 0.178
UPPER_ARM = round((0.818 - 0.63) * H, 4)
FOREARM = round((0.63 - 0.485) * H, 4)
HAND = 0.19
HEAD_TILT = -6.0
HEAD = round((H + SOLE - C7) / math.cos(math.radians(HEAD_TILT)), 4)

FOOT_LEN = round(0.152 * H, 4)
HEEL_BACK = 0.052
BALL_FWD = round(0.73 * FOOT_LEN - HEEL_BACK, 4)
BALL_UP = 0.032
FOOT = round(math.hypot(BALL_FWD, ANKLE - BALL_UP), 4)
FOOT_DROP = math.degrees(math.atan2(ANKLE - BALL_UP, BALL_FWD))
FOOT_REST = round(90 - FOOT_DROP, 3)
TOE = 0.075
SHOE_BACK = 0.066
SHOE_TOE = 0.083
SHOE_W = 0.108

DECK_L = 0.81
DECK_W = 0.2032
DECK_T = 0.0115
DECK_Y = 0.085
DECK_TOP = round(DECK_Y + DECK_T / 2, 4)
CONCAVE = 0.008
HOLE_IN = 0.184
HOLE_OUT = 0.238
KICK_START = 0.262
NOSE_TIP = 0.407
TAIL_TIP = -0.403
NOSE_RISE = 0.054
TAIL_RISE = 0.050
KICK_DEG = 21.0
AXLE_X = 0.205
WHEEL_R = 0.027
WHEEL_W = 0.032
AXLE_HALF = 0.1015
WHEEL_Z = round(AXLE_HALF - WHEEL_W / 2, 4)
TRUCK_H = round(DECK_Y - DECK_T / 2 - WHEEL_R, 4)
BEARING_R = 0.011


def deck_y(x):
    """🛹 Height of the deck's middle surface above its flat middle (board frame) at x: flat between the kick starts, an arc of 0.12 m radius into the straight kick of KICK_DEG."""
    s = 1 if x >= 0 else -1
    ax = abs(x)
    if ax <= KICK_START:
        return 0.0
    r = 0.12
    a = math.radians(KICK_DEG)
    blend = r * math.sin(a)
    if ax <= KICK_START + blend:
        d = ax - KICK_START
        return r - math.sqrt(r * r - d * d)
    y0 = r - r * math.cos(a)
    return y0 + (ax - KICK_START - blend) * math.tan(a) * (1.0 if s > 0 else TAIL_RISE / NOSE_RISE)


def deck_half_width(x):
    """🛹 Half width of the deck outline at x: full width over the middle, a round nose and tail."""
    tip = NOSE_TIP if x >= 0 else -TAIL_TIP
    ax = abs(x)
    r = 0.098
    c = tip - r
    if ax <= c:
        return DECK_W / 2 - max(0.0, ax - 0.21) ** 2 * 0.012 / 0.1 ** 2 * 0.25
    hw = DECK_W / 2 - max(0.0, c - 0.21) ** 2 * 0.012 / 0.1 ** 2 * 0.25
    return min(hw, math.sqrt(max(r * r - (ax - c) ** 2, 0.0)) * hw / r)


if __name__ == "__main__":
    for k in ("HIP", "KNEE", "ANKLE", "THIGH", "SHANK", "C7", "TORSO", "SHOULDER", "SHOULDER_X", "UPPER_ARM", "FOREARM", "HEAD", "FOOT_LEN", "BALL_FWD", "FOOT", "FOOT_DROP", "FOOT_REST", "DECK_TOP", "TRUCK_H", "WHEEL_Z"):
        print(k, globals()[k])
    print("deck", [(x, round(deck_y(x), 4), round(deck_half_width(x), 4)) for x in (0, 0.2, 0.26, 0.3, 0.35, 0.38, 0.4, -0.4)])
