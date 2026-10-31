"""NW-H1 person-with-white-cane: woman, 1.68 m, long white cane with roller tip in constant-contact technique."""
import math
from nwlib import *
from human import *
from gait import Gait, smooth

ID = "person-with-white-cane"
h = Human(1.68, 62.0, "f")
L_CANE = 1.30
T_WALK = 1.1
S_WALK = 1.10

h.extra_bones = [{"id": "cane", "parent": "hand-r", "title": tx("Long white cane", "Langer Blindenstock"), "x": r4(0.45 * h.Lha), "y": 0, "angleDeg": 0, "lengthM": L_CANE,
                  "joint": {"type": "revolute", "minDeg": -100, "maxDeg": 100},
                  "mass": {"kg": 0.25, "comX": 0.55, "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "jacobson-2013"}}]
rig = h.build_rig()
REST = h.rest
ANCH = h.anchors()
ANCH += [{"id": "cane-tip", "bone": "cane", "x": L_CANE, "y": 0, "kind": "support", "title": tx("Roller tip of the cane", "Rollspitze des Stocks"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.02, "footprintWidthM": 0.02, "frictionCoefficient": 0.02},
         {"id": "grip-r", "bone": "hand-r", "x": r4(0.45 * h.Lha), "y": 0, "kind": "attachment", "title": tx("Right hand on the cane grip", "Rechte Hand am Stockgriff")},
         {"id": "bag-strap", "bone": "torso", "x": r4(0.8 * h.Lt), "y": 0.0, "kind": "load", "title": tx("Shoulder bag strap", "Schultergurt der Tasche")}]
ANCH_BY = {a["id"]: a for a in ANCH}


def aw(world, aid):
    a = ANCH_BY[aid]
    return world[a["bone"]].ap(a["x"], a["y"])


def cane_state(px, py, torso, neck, head, legs, grip_rel, left_arm, pel_ang=90.0):
    """legs: {'l':{ankle,foot,toe},'r':{...}}; grip_rel: grip position relative to the shoulder; left_arm: world angles (a1,a2,a3)"""
    sh, _ = h.shoulder((px, py), torso, pel_ang)
    g = (sh[0] + grip_rel[0], sh[1] + grip_rel[1])
    tipx = g[0] + math.sqrt(max(L_CANE ** 2 - g[1] ** 2, 1e-6))
    ang = math.degrees(math.atan2(0 - g[1], tipx - g[0]))
    k = 0.45 * h.Lha
    ha = ang + 24.0
    wr = (g[0] - k * math.cos(math.radians(ha)), g[1] - k * math.sin(math.radians(ha)))
    arms = {"r": dict(wrist=wr, hand=ha, elbow=-1), "l": dict(ang=left_arm)}
    wa, flags = h.solve((px, py), torso, neck, head, legs, arms, pelvis_ang=pel_ang, extra={"cane": ang})
    offs = offsets_from_world(rig, wa, {}, 0.0)
    return offs, wa, [k_ for k_, v in flags.items() if not v], tipx


FLAT = -h.theta


def flat(x):
    return dict(ankle=(x, h.ah), foot=FLAT, toe=0.0)


def stand_state(torso=89.0, grip=(0.14, -0.28), y=None, lx=0.06, rx=-0.06, head=92.0):
    py = (h.hh - 0.012) if y is None else y
    return cane_state(0.0, py, torso, torso - 3, head, {"l": flat(lx), "r": flat(rx)}, grip, (-92.0, -84.0, -84.0))


def diag_state():
    return cane_state(0.0, h.hh - 0.012, 88.0, 86.0, 92.0, {"l": flat(0.08), "r": flat(-0.07)}, (0.02, -0.34), (-96.0, -80.0, -80.0))


def vertical_state():
    return cane_state(0.0, h.hh - 0.012, 90.0, 88.0, 93.0, {"l": flat(0.05), "r": flat(-0.05)}, (0.08, -0.26), (-90.0, -86.0, -86.0))


GAIT = Gait(h, T_WALK, S_WALK, stance=0.6, u1=0.10, u2=0.40, hs_deg=16.0, to_deg=34.0, clear=0.05, flat_k=0.22, bob=0.011, y_sag=0.04)


def walk_state(t):
    G = GAIT
    px, py = G.px(t), G.py(t)
    fl, fr = G.foot(t, 0.0), G.foot(t, 0.5)
    ph = t / G.T
    gx = 0.14 + 0.04 * math.sin(2 * math.pi * ph + 0.5)
    la = -90.0 + 20.0 * math.sin(2 * math.pi * ph + math.pi)
    left = (la, la + 14.0, la + 14.0)
    st = cane_state(px, py, 89.0, 86.0, 92.0, {"l": dict(ankle=fl["ankle"], foot=fl["foot"], toe=fl["toe"]), "r": dict(ankle=fr["ankle"], foot=fr["foot"], toe=fr["toe"])}, (gx, -0.28), left)
    return st[0], (px, py), st[2]
