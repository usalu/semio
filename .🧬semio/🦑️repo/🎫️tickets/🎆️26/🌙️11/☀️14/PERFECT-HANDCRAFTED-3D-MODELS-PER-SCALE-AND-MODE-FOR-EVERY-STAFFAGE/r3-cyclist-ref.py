"""📐 Round 3 reference numbers of humans/cyclist: the rider's segment lengths as shares of her stature and the city bicycle's real geometry, in frame metres (x ahead from the rear axle's ground point, y up, z left).

Sources: brief anthropometric shares (hip 0.52 H, knee 0.285 H, ankle 0.039 H, shoulder 0.818 H, elbow 0.63 H, wrist 0.485 H), de Leva 1996 female, Drillis and Contini 1964, ETRTO 2023 (700 x 40C = 40-622), EN ISO 4210 / EN 14764 city bicycles, manufacturer geometry of low-step city bicycles (Gazelle Esprit, Kalkhoff Image, Batavus Fonk, 50 cm frame).
"""
import math

H = 1.72
THIGH = round((0.52 - 0.285) * H, 4)
SHANK = round((0.285 - 0.039) * H, 4)
HIP_Z = 0.088
SHOULDER_Z = 0.19
NECK = 0.112
HEAD = 0.158
ANKLE_SHOE = 0.092
BALL_SHOE = 0.035
ANKLE_TO_BALL_X = 0.125
FOOT = round(math.hypot(ANKLE_TO_BALL_X, ANKLE_SHOE - BALL_SHOE), 4)
FOOT_DROP = math.degrees(math.atan2(ANKLE_SHOE - BALL_SHOE, ANKLE_TO_BALL_X))
FOOT_REST = round(90 - FOOT_DROP, 3)
TOE_REST = round(FOOT_DROP, 3)
HEEL_X = -0.062
TOE_TIP = 0.083

WHEEL_R = 0.34
TYRE_W = 0.040
TYRE_H = 0.038
RIM_OUT = WHEEL_R - TYRE_H + 0.004
RIM_IN = RIM_OUT - 0.021
RIM_W = 0.024
REAR = (0.0, 0.34)
FRONT = (1.05, 0.34)
BB = (0.45, 0.30)
CRANK = 0.17
CRANK_Z = 0.078
PEDAL_Z0, PEDAL_Z1 = 0.094, 0.176
PEDAL_T = 0.025
FOOT_Z = 0.132

HEAD_ANGLE = 71.0
FORK_OFFSET = 0.045
FORK_LEN = 0.392
HT_LEN = 0.150
SEAT_ANGLE = 73.0
ST_LEN = 0.500
POST_LEN = 0.605
SADDLE_TOP = 0.92
SADDLE_X = 0.25

CLAMP = (0.868, 1.000)
GRIP = (0.800, 0.990)
GRIP_Z = 0.250
GRIP_SWEEP = 20.0
GRIP_DROP = 3.0
GRIP_LEN = 0.115
GRIP_R = 0.0165
BAR_R = 0.0111
GRIP_LOCAL = (0.084, -0.024)

FENDER_R = 0.362
CARRIER_Y = 0.80


def steer_dir():
    a = math.radians(HEAD_ANGLE)
    return (-math.cos(a), math.sin(a))


def steer_point(s):
    """🧭 The point s metres up the steering axis from its point nearest the front axle."""
    d = steer_dir()
    n = (d[1], -d[0])
    p0 = (FRONT[0] - FORK_OFFSET * n[0], FRONT[1] - FORK_OFFSET * n[1])
    return (p0[0] + s * d[0], p0[1] + s * d[1])


CROWN = steer_point(FORK_LEN)
HT_BOTTOM = steer_point(FORK_LEN + 0.012)
HT_TOP = steer_point(FORK_LEN + 0.012 + HT_LEN)
STEERER_TOP = steer_point(FORK_LEN + 0.012 + HT_LEN + 0.062)


def seat_point(L):
    a = math.radians(SEAT_ANGLE)
    return (BB[0] - L * math.cos(a), BB[1] + L * math.sin(a))


ST_TOP = seat_point(ST_LEN)
POST_TOP = seat_point(POST_LEN)


def grip_axis(side):
    """➡️ Unit direction of a grip from its inner to its outer end (swept back, slightly dropped)."""
    s = math.radians(GRIP_SWEEP)
    d = math.radians(GRIP_DROP)
    return (-math.sin(s) * math.cos(d), -math.sin(d), side * math.cos(s) * math.cos(d))


def grip_centre(side):
    return (GRIP[0], GRIP[1], side * GRIP_Z)


def pedal_target(spindle, angle_deg, side):
    """🦶 Where the ball joint sits over a pedal spindle at a pedal angle: on the pedal's normal through the spindle, half a pedal plus the shoe's ball height above it."""
    a = math.radians(angle_deg)
    n = (-math.sin(a), math.cos(a))
    h = PEDAL_T / 2 + BALL_SHOE
    return (spindle[0] + h * n[0], spindle[1] + h * n[1], side * FOOT_Z)


if __name__ == "__main__":
    for k in ("THIGH", "SHANK", "FOOT", "FOOT_REST", "TOE_REST", "CROWN", "HT_BOTTOM", "HT_TOP", "STEERER_TOP", "ST_TOP", "POST_TOP"):
        print(k, globals()[k])
