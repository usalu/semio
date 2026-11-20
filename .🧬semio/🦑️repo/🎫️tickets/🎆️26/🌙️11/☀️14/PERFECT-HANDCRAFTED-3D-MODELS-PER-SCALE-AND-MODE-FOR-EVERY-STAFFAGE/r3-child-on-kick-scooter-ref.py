"""📏 Round 3 reference of humans/child-on-kick-scooter: the real dimensions of an 8-year-old girl and of a 145 mm children's kick scooter, as the numbers every other r3-child-on-kick-scooter script reads.

Sources: Fryar 2021 (stature and mass class of the asset), Snyder 1977 (UM-HSRI-77-17, landmark heights and breadths of 8-year-olds), Drillis and Contini 1964 (arm link ratios), Micro Speed / Micro Sprite Suspension data sheets (145 mm wheels, deck 0.34 x 0.12 m, bar 0.68-0.96 m, folded length 0.67 m, 3.75 kg, bar width 0.33-0.34 m).
"""
import math

SOLE = 0.015
STATURE = 1.274
VERTEX = STATURE + SOLE
HEAD_H = 0.195
ANKLE_Y = 0.048 + SOLE
KNEE_Y = round(0.268 * STATURE + SOLE, 4)
HIP_Y = round(0.505 * STATURE + SOLE, 4)
PELVIS_L = 0.064
C7_Y = round(0.83 * STATURE + SOLE, 4)
SHOULDER_Y = round(0.795 * STATURE + SOLE, 4)
AO_Y = round(VERTEX - 0.124, 4)

THIGH_L = round(HIP_Y - KNEE_Y, 4)
SHANK_L = round(KNEE_Y - ANKLE_Y, 4)
TORSO_L = round(C7_Y - HIP_Y - PELVIS_L, 4)
NECK_L = round(AO_Y - C7_Y, 4)
HEAD_L = round(VERTEX - AO_Y, 4)
SHOULDER_U = round(SHOULDER_Y - HIP_Y - PELVIS_L, 4)
SHOULDER_Z = 0.115
HIP_Z = 0.065
ARM_L = 0.2286
FOREARM_L = 0.1794
HAND_L = 0.1327

BALL_X = 0.092
FOOT_L = round(math.hypot(BALL_X, ANKLE_Y), 4)
FOOT_DIR = math.degrees(math.atan2(-ANKLE_Y, BALL_X))
FOOT_REST = round(FOOT_DIR + 90, 3)
TOE_REST = round(-FOOT_DIR, 3)
TOE_L = 0.062
HEEL_BACK = -0.062
HEEL_PAD_X = -0.040

WB = 0.56
WHEEL_R = 0.0725
WHEEL_W = 0.030
DECK_TOP = 0.085
DECK_BOTTOM = 0.057
DECK_X0 = 0.085
DECK_X1 = 0.415
DECK_HALF = 0.060
RAKE = math.radians(10)
HT_BOTTOM = (0.5449, 0.16)
HT_T0 = 0.02
HT_T1 = 0.12
CROWN_T = 0.008
AXIS = (-math.sin(RAKE), math.cos(RAKE))
BAR_Y = 0.78
BAR_T = (BAR_Y - HT_BOTTOM[1]) / AXIS[1]
BAR_X = HT_BOTTOM[0] + AXIS[0] * BAR_T
BAR_HALF = 0.17
GRIP_Z = 0.115
GRIP_R = 0.015
FRAME_X = -0.14
FENDER_R0 = WHEEL_R + 0.008
FENDER_R1 = WHEEL_R + 0.014
FENDER_TOP = (0.0, round(WHEEL_R + FENDER_R1, 4))

HAND_DIR = 5.0
GRIP_LOCAL = (0.060, -0.026)


def stem_point(t):
    return (HT_BOTTOM[0] + AXIS[0] * t, HT_BOTTOM[1] + AXIS[1] * t)


def foot_local(x, y):
    a = math.radians(FOOT_DIR)
    return (x * math.cos(a) + y * math.sin(a), -x * math.sin(a) + y * math.cos(a))


if __name__ == "__main__":
    for k, v in sorted(globals().items()):
        if k.isupper():
            print(k, v)
