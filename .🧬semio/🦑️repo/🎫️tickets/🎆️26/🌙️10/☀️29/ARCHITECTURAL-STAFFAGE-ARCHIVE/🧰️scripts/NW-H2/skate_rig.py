"""Skateboarder: rig, anchors and IK poses (board frame; the board is the root, the rider hangs on prismatic hips)."""
import math
from lib import *
from human import *

H = 1.78
MASS = 68.0
BOARD_Y = 0.085
DECK_TOP = 0.012
WB = 0.1875
WHEEL_R = 0.027
WHEEL_Y = -(BOARD_Y - WHEEL_R)


def build_rig():
    bones = []
    bones.append(bone("board", None, "Skateboard deck, trucks and bearings", "Skateboard-Deck mit Achsen und Kugellagern", 0, BOARD_Y, 0, 0.81, {"type": "fixed"}, {"kg": 1.9, "comX": 0.0, "comY": -0.03, "distribution": "point", "source": "authored-estimate", "reference": "board-spec"}))
    for s, x, en, de in (("f", WB, "Front wheels", "Vorderräder"), ("r", -WB, "Rear wheels", "Hinterräder")):
        bones.append(bone(f"wheel-{s}", "board", en, de, x, WHEEL_Y, 0, WHEEL_R, {"type": "revolute"}, {"kg": 0.175, "comX": 0.0, "comY": 0.0, "distribution": "point", "source": "authored-estimate", "reference": "board-spec"}))
    bones.append(bone("hip-lift", "board", "Hip height above the board", "Hüfthöhe über dem Brett", 0, 0.6, 0, 0.01, {"type": "prismatic", "axisDeg": 90, "minM": -0.1, "maxM": 0.7}))
    bones.append(bone("hip-shift", "hip-lift", "Hip shift along the board", "Hüftverschiebung entlang des Bretts", 0, 0, 0, 0.01, {"type": "prismatic", "axisDeg": 0, "minM": -0.9, "maxM": 0.9}))
    body, L = human_bones(H, True, MASS, parent_of_pelvis="hip-shift", foot_min=-65)
    bones += body
    return bones, L


def build_anchors(L):
    a = human_anchors(L, H)
    a.insert(0, {"id": "wheel-f-contact", "bone": "wheel-f", "x": 0.0, "y": -WHEEL_R, "kind": "contact", "title": tx("Front wheel contact", "Kontakt Vorderrad"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.04, "frictionCoefficient": 0.02})
    a.insert(1, {"id": "wheel-r-contact", "bone": "wheel-r", "x": 0.0, "y": -WHEEL_R, "kind": "contact", "title": tx("Rear wheel contact", "Kontakt Hinterrad"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.04, "frictionCoefficient": 0.02})
    a.insert(2, {"id": "tail", "bone": "board", "x": -0.398, "y": 0.066, "kind": "contact", "title": tx("Tail tip", "Heckspitze"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.03, "footprintWidthM": 0.1, "frictionCoefficient": 0.4})
    a.append({"id": "board-load", "bone": "board", "x": 0.0, "y": DECK_TOP, "kind": "load", "title": tx("Deck centre", "Deckmitte")})
    return a


def new_fig(bones):
    return Fig(bones)


def to_board(fig, p, theta):
    """Point relative to the unrotated board origin -> board-frame world point (for IK) given the board tilt theta."""
    c, s = rot(-theta)
    return (c * p[0] - s * p[1], s * p[0] + c * p[1] + BOARD_Y)


def ankle_for_ball(ball_pt, phi, L):
    """Ankle position so that the ball-of-foot anchor sits at ball_pt with the foot bone at world angle phi."""
    c, s = rot(phi)
    lx, ly = L["ball"], -L["ankle"]
    return (ball_pt[0] - (c * lx - s * ly), ball_pt[1] - (s * lx + c * ly))


def make_pose(bones, L, pelvis, torso_deg, head_deg, front, rear, arm_near, arm_far, theta=0.0, front_foot=0.0, rear_foot=0.0, toe_f=0.0, toe_r=0.0, hand=(0, 0), pelvis_tilt=0.0, elbow=(-1, -1)):
    """All inputs are world values relative to the unrotated board origin (positions) and world angles; theta is the board tilt.
    front/rear: ankle targets of the near (l, front) and far (r, rear) leg; arm targets are wrist points."""
    f = Fig(bones)
    tb = lambda p: to_board(f, p, theta)
    f.place_pelvis(*tb(pelvis))
    f.rel("pelvis", pelvis_tilt)
    f.aim("torso", torso_deg - theta)
    f.aim("head", head_deg - theta)
    f.leg("l", tb(front), +1)
    f.leg("r", tb(rear), +1)
    f.aim("foot-l", front_foot - theta)
    f.aim("foot-r", rear_foot - theta)
    f.rel("toe-l", toe_f)
    f.rel("toe-r", toe_r)
    f.arm("l", tb(arm_near), elbow[0])
    f.arm("r", tb(arm_far), elbow[1])
    f.aim("hand-l", norm180(f.world("forearm-l")[2] + hand[0]))
    f.aim("hand-r", norm180(f.world("forearm-r")[2] + hand[1]))
    f.root["angleDeg"] = theta
    return f
