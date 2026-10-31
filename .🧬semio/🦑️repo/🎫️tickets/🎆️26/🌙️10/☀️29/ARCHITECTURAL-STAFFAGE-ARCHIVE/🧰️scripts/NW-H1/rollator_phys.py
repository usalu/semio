"""NW-H1 person-with-rollator: rig, anchors, poses, clips, thermal scenario (physical part, scale independent)."""
import math
from nwlib import *
from human import *
from gait import Gait, smooth

ID = "person-with-rollator"
h = Human(1.58, 62.0, "f")
R_W = 0.10
WB = 0.52
X_R0 = 0.38
GRIP_X = -0.07
GRIP_Y = 0.85
SEAT_Y = 0.52

T_WALK = 1.3
S_WALK = 2 * math.pi * R_W


def add_aid_bones():
    h.extra_bones = [
        {"id": "aid-lift", "parent": "pelvis", "title": tx("Aid mount, vertical travel", "Hilfsmittelaufnahme, senkrechter Weg"), "x": 0, "y": 0, "angleDeg": -90, "lengthM": 0.0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.6, "maxM": 0.6}},
        {"id": "aid-slide", "parent": "aid-lift", "title": tx("Aid mount, horizontal travel", "Hilfsmittelaufnahme, waagerechter Weg"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -1.5, "maxM": 1.5}},
        {"id": "frame", "parent": "aid-slide", "title": tx("Rollator frame with seat and basket", "Rollatorrahmen mit Sitz und Korb"), "x": X_R0, "y": -h.hh, "angleDeg": 0, "lengthM": 0.52, "joint": {"type": "fixed"},
         "mass": {"kg": 5.4, "comX": 0.26, "comY": 0.40, "distribution": "point", "source": "authored-estimate", "reference": "iso-11199-2-2021"}},
        {"id": "wheel-rear", "parent": "frame", "title": tx("Rear wheel pair", "Hinterradpaar"), "x": 0, "y": R_W, "angleDeg": 0, "lengthM": R_W, "joint": {"type": "revolute"},
         "mass": {"kg": 1.0, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "iso-11199-2-2021"}},
        {"id": "wheel-front", "parent": "frame", "title": tx("Front caster pair", "Vorderes Lenkrollenpaar"), "x": WB, "y": R_W, "angleDeg": 0, "lengthM": R_W, "joint": {"type": "revolute"},
         "mass": {"kg": 0.8, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "iso-11199-2-2021"}},
    ]


add_aid_bones()
rig = h.build_rig()
REST = h.rest
ANCH = h.anchors()
ANCH += [
    {"id": "wheel-rear-contact", "bone": "frame", "x": 0, "y": 0, "kind": "support", "title": tx("Rear wheel tread patch", "Aufstandsfläche der Hinterräder"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.08, "frictionCoefficient": 0.03},
    {"id": "wheel-front-contact", "bone": "frame", "x": WB, "y": 0, "kind": "support", "title": tx("Front caster tread patch", "Aufstandsfläche der Lenkrollen"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.08, "frictionCoefficient": 0.03},
    {"id": "grip-l", "bone": "hand-l", "x": r4(0.45 * h.Lha), "y": 0, "kind": "attachment", "title": tx("Left hand on the grip", "Linke Hand am Griff")},
    {"id": "grip-r", "bone": "hand-r", "x": r4(0.45 * h.Lha), "y": 0, "kind": "attachment", "title": tx("Right hand on the grip", "Rechte Hand am Griff")},
    {"id": "basket", "bone": "frame", "x": 0.26, "y": 0.40, "kind": "load", "title": tx("Centre of the basket", "Korbmitte")},
    {"id": "seat", "bone": "frame", "x": 0.255, "y": SEAT_Y, "kind": "attachment", "title": tx("Seat surface", "Sitzfläche")},
]
ANCH_BY = {a["id"]: a for a in ANCH}


def aw(world, aid):
    a = ANCH_BY[aid]
    return world[a["bone"]].ap(a["x"], a["y"])


def solve_state(pelvis, torso, neck, head, ankles, arms_wrist, frame_xy, hand_ang=-8.0, elbow=-1, pel_ang=90.0, wheel_deg=(0.0, 0.0), foot=None, toe=None, arm_ang=None):
    """full offsets for one instant. ankles: {'l':(x,y),'r':(x,y)}; foot/toe world angles per side default flat."""
    foot = foot or {"l": -h.theta, "r": -h.theta}
    toe = toe or {"l": 0.0, "r": 0.0}
    legs = {s: dict(ankle=ankles[s], foot=foot[s], toe=toe[s]) for s in "lr"}
    arms = {}
    for s in "lr":
        if arm_ang and s in arm_ang:
            arms[s] = dict(ang=arm_ang[s])
        else:
            wx, wy = arms_wrist[s]
            arms[s] = dict(wrist=(wx, wy), hand=hand_ang, elbow=elbow)
    wa, flags = h.solve(pelvis, torso, neck, head, legs, arms, pelvis_ang=pel_ang)
    bad = [k for k, v in flags.items() if not v]
    # aid mount: frame origin world (frame_xy) = pelvis + (slide + X_R0, lift - hh)
    lift = frame_xy[1] - (pelvis[1] - h.hh)
    slide = frame_xy[0] - pelvis[0] - X_R0
    offs = offsets_from_world(rig, wa, {"aid-lift": lift, "aid-slide": slide}, 0.0)
    offs["wheel-rear"] = {"angleDeg": wheel_deg[0]}
    offs["wheel-front"] = {"angleDeg": wheel_deg[1]}
    return offs, wa, bad


def wrist_for_grip(grip, ang_deg):
    a = math.radians(ang_deg)
    k = 0.45 * h.Lha
    return (grip[0] - k * math.cos(a), grip[1] - k * math.sin(a))


FLAT = -h.theta


def ankle_flat(x):
    return (x, h.ah)


def stand_state(sway=0.0, lean=76.0, neck=66.0, head=78.0, y=0.835, xf=0.40, lfoot=0.05, rfoot=-0.05):
    px = sway
    grip = (xf + GRIP_X, GRIP_Y)
    gl = (grip[0] + 0.015, grip[1])
    wr = {"r": wrist_for_grip(grip, -8.0), "l": wrist_for_grip(gl, -8.0)}
    return solve_state((px, y), lean, neck, head, {"l": ankle_flat(lfoot), "r": ankle_flat(rfoot)}, wr, (xf, 0.0))


def seated_state():
    px, py = 0.0, SEAT_Y + 0.095
    xf = -0.19
    sh, _ = h.shoulder((px, py), 86.0)
    thigh_dir = None
    wr = {}
    for s, off in (("r", 0.0), ("l", 0.03)):
        wr[s] = (0.20 + off, py + 0.01)
    return solve_state((px, py), 86.0, 82.0, 88.0, {"l": ankle_flat(0.50), "r": ankle_flat(0.47)}, wr, (xf, 0.0), hand_ang=-5.0)


def stoop_state():
    px, py = 0.0, 0.73
    xf = 0.09
    wr = {"r": (xf + 0.30, 0.56), "l": (xf + 0.32, 0.56)}
    return solve_state((px, py), 36.0, 24.0, 52.0, {"l": ankle_flat(0.03), "r": ankle_flat(-0.04)}, wr, (xf, 0.0), hand_ang=-35.0)


def walk_state(t, G, ofs=0.40, osc=0.012):
    px, py = G.px(t), G.py(t)
    fl = G.foot(t, 0.0)
    fr = G.foot(t, 0.5)
    ph = t / G.T
    xf = px + ofs + osc * math.sin(4 * math.pi * ph + 0.6)
    lean = 74.0 + 1.2 * math.cos(4 * math.pi * (ph - 0.2))
    grip = (xf + GRIP_X, GRIP_Y)
    gl = (grip[0] + 0.015, grip[1])
    wr = {"r": wrist_for_grip(grip, -8.0), "l": wrist_for_grip(gl, -8.0)}
    wheel = -math.degrees((xf - (G.px(0) + ofs + osc * math.sin(0.6))) / R_W)
    offs, wa, bad = solve_state((px, py), lean, lean - 10, lean + 2, {"l": fl["ankle"], "r": fr["ankle"]}, wr, (xf, 0.0), wheel_deg=(wheel, wheel),
                                foot={"l": fl["foot"], "r": fr["foot"]}, toe={"l": fl["toe"], "r": fr["toe"]})
    return offs, (px, py), bad


GAIT = Gait(h, T_WALK, S_WALK, stance=0.64, bob=0.010, y_sag=0.04, clear=0.04, hs_deg=12.0, to_deg=26.0)
