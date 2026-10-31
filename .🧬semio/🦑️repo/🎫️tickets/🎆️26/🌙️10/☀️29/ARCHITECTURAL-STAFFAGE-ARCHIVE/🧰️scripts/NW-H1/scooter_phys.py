"""NW-H1 child-on-kick-scooter: girl 8 y, 1.28 m, 26 kg on a two-wheel kick scooter; kick-and-glide cycle."""
import math
from nwlib import *
from human import *
from gait import smooth

ID = "child-on-kick-scooter"
h = Human(1.28, 26.0, "c", head_k=1.1, limb_k=0.96, lim_over={"foot": (-65, 75), "thigh": (-60, 125), "torso": (-80, 30)})
R_W = 0.0725
X_R0 = -0.15
WB = 0.78
DECK_TOP = 0.11
PX_LOC = 0.30
N_TURNS = 5
TC = 2.2
S_ADV = N_TURNS * 2 * math.pi * R_W
V = S_ADV / TC
BAR = (0.60, 0.84)

h.extra_bones = [
    {"id": "aid-lift", "parent": "pelvis", "title": tx("Aid mount, vertical travel", "Hilfsmittelaufnahme, senkrechter Weg"), "x": 0, "y": 0, "angleDeg": -90, "lengthM": 0.0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.6, "maxM": 0.6}},
    {"id": "aid-slide", "parent": "aid-lift", "title": tx("Aid mount, horizontal travel", "Hilfsmittelaufnahme, waagerechter Weg"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -1.5, "maxM": 1.5}},
    {"id": "frame", "parent": "aid-slide", "title": tx("Scooter deck, stem and fork", "Trittbrett, Lenkstange und Gabel"), "x": X_R0, "y": -h.hh, "angleDeg": 0, "lengthM": 0.78, "joint": {"type": "fixed"},
     "mass": {"kg": 3.3, "comX": 0.40, "comY": 0.30, "distribution": "point", "source": "authored-estimate", "reference": "scooter-spec"}},
    {"id": "wheel-rear", "parent": "frame", "title": tx("Rear wheel", "Hinterrad"), "x": 0, "y": R_W, "angleDeg": 0, "lengthM": R_W, "joint": {"type": "revolute"},
     "mass": {"kg": 0.25, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "scooter-spec"}},
    {"id": "wheel-front", "parent": "frame", "title": tx("Front wheel", "Vorderrad"), "x": WB, "y": R_W, "angleDeg": 0, "lengthM": R_W, "joint": {"type": "revolute"},
     "mass": {"kg": 0.25, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "scooter-spec"}},
]
rig = h.build_rig()
REST = h.rest
ANCH = h.anchors()
ANCH += [
    {"id": "wheel-rear-contact", "bone": "frame", "x": 0, "y": 0, "kind": "support", "title": tx("Rear wheel tread patch", "Aufstandsfläche des Hinterrads"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.015, "footprintWidthM": 0.025, "frictionCoefficient": 0.02},
    {"id": "wheel-front-contact", "bone": "frame", "x": WB, "y": 0, "kind": "support", "title": tx("Front wheel tread patch", "Aufstandsfläche des Vorderrads"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.015, "footprintWidthM": 0.025, "frictionCoefficient": 0.02},
    {"id": "grip-l", "bone": "hand-l", "x": r4(0.45 * h.Lha), "y": 0, "kind": "attachment", "title": tx("Left hand on the handlebar", "Linke Hand am Lenker")},
    {"id": "grip-r", "bone": "hand-r", "x": r4(0.45 * h.Lha), "y": 0, "kind": "attachment", "title": tx("Right hand on the handlebar", "Rechte Hand am Lenker")},
    {"id": "deck", "bone": "frame", "x": 0.32, "y": DECK_TOP, "kind": "attachment", "title": tx("Deck surface", "Trittbrettfläche")},
    {"id": "fender", "bone": "frame", "x": 0.06, "y": 0.145, "kind": "attachment", "title": tx("Rear fender brake", "Hinterradbremse am Schutzblech")},
    {"id": "backpack", "bone": "torso", "x": r4(0.5 * h.Lt), "y": -0.13, "kind": "load", "title": tx("Backpack", "Rucksack")},
]
ANCH_BY = {a["id"]: a for a in ANCH}
FLAT = -h.theta


def aw(world, aid):
    a = ANCH_BY[aid]
    return world[a["bone"]].ap(a["x"], a["y"])


def state(px, py, fx, torso, neck, head, legs, wheel_deg, hand_ang=12.0, arms_ang=None):
    """frame origin world x = fx (y = 0). legs: {'l','r': dict(ankle,foot,toe)}; grips fixed on the bar."""
    sh, _ = h.shoulder((px, py), torso)
    grip = (fx + BAR[0], BAR[1])
    k = 0.45 * h.Lha
    wr = (grip[0] - k * math.cos(math.radians(hand_ang)), grip[1] - k * math.sin(math.radians(hand_ang)))
    wl = (wr[0] + 0.0, wr[1])
    arms = {"r": dict(wrist=wr, hand=hand_ang, elbow=-1), "l": dict(wrist=wl, hand=hand_ang, elbow=-1)}
    wa, flags = h.solve((px, py), torso, neck, head, legs, arms)
    lift = 0.0 - (py - h.hh)
    slide = fx - px - X_R0
    offs = offsets_from_world(rig, wa, {"aid-lift": lift, "aid-slide": slide}, 0.0)
    offs["wheel-rear"] = {"angleDeg": wheel_deg}
    offs["wheel-front"] = {"angleDeg": wheel_deg}
    return offs, wa, [k_ for k_, v in flags.items() if not v]


def on_deck(fx, local_x):
    return dict(ankle=(fx + local_x, DECK_TOP + h.ah), foot=FLAT, toe=0.0)


def glide_state(px=0.0, py=None, torso=72.0):
    py = (0.64 if py is None else py)
    fx = px - PX_LOC
    return state(px, py, fx, torso, torso - 8, torso + 10, {"l": on_deck(fx, 0.45), "r": on_deck(fx, 0.22)}, 0.0), (px, py)


def push_pose():
    px, py = 0.0, 0.585
    fx = px - PX_LOC
    ank = (px - 0.20, h.ah)
    return state(px, py, fx, 66.0, 56.0, 76.0, {"l": on_deck(fx, 0.45), "r": dict(ankle=ank, foot=FLAT - 20.0, toe=0.0)}, 0.0), (px, py)


def brake_pose():
    px, py = 0.0, 0.66
    fx = px - PX_LOC
    return state(px, py, fx, 80.0, 74.0, 90.0, {"l": on_deck(fx, 0.45), "r": dict(ankle=(fx + 0.07, 0.145 + h.ah), foot=FLAT + 10.0, toe=10.0)}, 0.0), (px, py)


def cycle(t):
    """kick-and-glide cycle, T = 2.2 s: push 0-0.38, return swing to the deck 0.38-0.85, glide 0.85-1.85, step down 1.85-2.2."""
    px = V * t
    fx = px - PX_LOC
    wheel = -math.degrees((fx - (0 - PX_LOC)) / R_W)
    PUSH, RET, GL = 0.34, 0.80, 1.85
    x_plant = 0.10
    deckR = lambda: dict(ankle=(fx + 0.22, DECK_TOP + h.ah), foot=FLAT, toe=0.0)
    if t < PUSH:
        s = t / PUSH
        py = 0.60 - 0.015 * s
        torso = 68.0 + 4.0 * s
        flat_end = 0.30
        if s < flat_end:
            ank = (x_plant, h.ah)
            rf = dict(ankle=ank, foot=FLAT, toe=0.0)
        else:
            sg = (s - flat_end) / (1 - flat_end)
            ang = FLAT - 26.0 * smooth(sg)
            ball = (x_plant + h.Lball, 0.0)
            rf = dict(ankle=h.foot_pose(ball, "ball", ang), foot=ang, toe=0.0)
    elif t < RET:
        s = (t - PUSH) / (RET - PUSH)
        sm = smooth(s)
        a0 = h.foot_pose((x_plant + h.Lball, 0.0), "ball", FLAT - 26.0)
        a1 = (fx + 0.22, DECK_TOP + h.ah)
        ank = (a0[0] + (a1[0] - a0[0]) * sm, a0[1] + (a1[1] - a0[1]) * sm + 0.07 * math.sin(math.pi * s))
        ang = (FLAT - 26.0) + (FLAT - (FLAT - 26.0)) * smooth(min(1, s * 1.3))
        rf = dict(ankle=ank, foot=ang, toe=ang + h.theta + 26.0 * (1 - smooth(min(1, s * 1.5))))
        py = 0.585 + 0.055 * sm
        torso = 72.0 - 2.0 * math.sin(math.pi * s)
    elif t < GL:
        s = (t - RET) / (GL - RET)
        rf = deckR()
        py = 0.64 + 0.004 * math.sin(2 * math.pi * s)
        torso = 72.0 + 2.0 * math.sin(math.pi * s)
    else:
        s = (t - GL) / (TC - GL)
        sm = smooth(s)
        x_land = (V * TC) + 0.10
        a0 = (fx + 0.22, DECK_TOP + h.ah)
        a1 = (x_land + 0.0, h.ah)
        ank = (a0[0] + (a1[0] - a0[0]) * sm, a0[1] + (a1[1] - a0[1]) * sm + 0.06 * math.sin(math.pi * s))
        rf = dict(ankle=ank, foot=FLAT, toe=0.0)
        py = 0.64 - 0.04 * sm
        torso = 72.0 - 4.0 * sm
    legs = {"l": on_deck(fx, 0.45), "r": rf}
    st = state(px, py, fx, torso, torso - 8, torso + 10, legs, wheel)
    return st, (px, py)
