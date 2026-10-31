"""NW-H1 person-on-crutches: young man, left foot off-loaded in a walking boot, two forearm crutches, swing-to three-point gait."""
import math
from nwlib import *
from human import *
from gait import smooth

ID = "person-on-crutches"
h = Human(1.82, 78.0, "m")
L_C = 0.90
T_GAIT = 1.8
D_STEP = 0.40


def circ_int(c0, r0, c1, r1):
    """intersections of two circles; returns the one with larger y"""
    dx, dy = c1[0] - c0[0], c1[1] - c0[1]
    d = math.hypot(dx, dy)
    d = min(max(d, abs(r0 - r1) + 1e-6), r0 + r1 - 1e-6)
    a = (r0 * r0 - r1 * r1 + d * d) / (2 * d)
    hh = math.sqrt(max(r0 * r0 - a * a, 0))
    ux, uy = dx / math.hypot(dx, dy), dy / math.hypot(dx, dy)
    mx, my = c0[0] + a * ux, c0[1] + a * uy
    p1 = (mx - hh * uy, my + hh * ux)
    p2 = (mx + hh * uy, my - hh * ux)
    return p1 if p1[1] > p2[1] else p2


h.extra_bones = []
for s, (en, de) in (("l", ("Left", "Linke")), ("r", ("Right", "Rechte"))):
    h.extra_bones.append({"id": "crutch-" + s, "parent": "hand-" + s, "title": tx(en + " forearm crutch", de + " Unterarmgehstütze"), "x": r4(0.45 * h.Lha), "y": 0, "angleDeg": 0, "lengthM": L_C,
                          "joint": {"type": "revolute", "minDeg": -130, "maxDeg": 130},
                          "mass": {"kg": 0.55, "comX": 0.40, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "iso-11334-1-2007"}})
rig = h.build_rig()
REST = h.rest
ANCH = h.anchors()
for s, (en, de) in (("l", ("Left", "Linke")), ("r", ("Right", "Rechte"))):
    ANCH.append({"id": "tip-" + s, "bone": "crutch-" + s, "x": L_C, "y": 0, "kind": "contact", "title": tx(en + " rubber crutch tip", de + " Gummi-Krückenspitze"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.04, "frictionCoefficient": 0.7})
    ANCH.append({"id": "grip-" + s, "bone": "hand-" + s, "x": r4(0.45 * h.Lha), "y": 0, "kind": "attachment", "title": tx(en + " hand on the crutch grip", de + " Hand am Krückengriff")})
ANCH_BY = {a["id"]: a for a in ANCH}


def aw(world, aid):
    a = ANCH_BY[aid]
    return world[a["bone"]].ap(a["x"], a["y"])


def crutch_arm(sh_pt, tip, rarm):
    g = circ_int(sh_pt, rarm, tip, L_C)
    ang = math.degrees(math.atan2(tip[1] - g[1], tip[0] - g[0]))
    fa = math.degrees(math.atan2(g[1] - sh_pt[1], g[0] - sh_pt[0]))
    k = 0.45 * h.Lha
    wr = (g[0] - k * math.cos(math.radians(fa)), g[1] - k * math.sin(math.radians(fa)))
    return g, ang, fa, wr


def state(px, py, torso, neck, head, right_foot, left_leg, tips, rarm=0.64, pel_ang=90.0):
    """right_foot = dict(ankle,foot,toe); left_leg = dict(thigh,shank,foot,toe) world angles; tips = {'l':(x,y),'r':(x,y)}"""
    sh, _ = h.shoulder((px, py), torso, pel_ang)
    arms = {}
    extra = {}
    for s in "lr":
        g, ang, fa, wr = crutch_arm(sh, tips[s], rarm)
        arms[s] = dict(wrist=wr, hand=fa, elbow=-1)
        extra["crutch-" + s] = ang
    legs = {"r": right_foot}
    a1, a2, ok = ik2((px, py), right_foot["ankle"], h.Lth, h.Lsh, +1)
    wa, flags = h.solve((px, py), torso, neck, head, {"r": right_foot, "l": dict(ankle=right_foot["ankle"], foot=0, toe=0)}, arms, pelvis_ang=pel_ang, extra=extra)
    wa["thigh-l"], wa["shank-l"], wa["foot-l"], wa["toe-l"] = left_leg
    flags.pop("leg-l", None)
    offs = offsets_from_world(rig, wa, {}, 0.0)
    return offs, wa, [k for k, v in flags.items() if not v]


FLAT = -h.theta


def flat_foot(x):
    return dict(ankle=(x, h.ah), foot=FLAT, toe=0.0)


LEFT_HOLD = (-76.0, -118.0, -52.0, None)


def left_leg(t_sway=0.0):
    return (-66.0 + t_sway, -122.0 + 0.5 * t_sway, -40.0, -40.0 + h.theta)


def stand_tripod(x0=0.0, tip_x=0.15):
    py = h.hh - 0.02
    rf = flat_foot(x0)
    return state(x0, py, 84.0, 78.0, 88.0, rf, left_leg(), {"l": (tip_x + 0.05, 0.0), "r": (tip_x, 0.0)})


def swing_crutch_pose():
    py = h.hh - 0.02
    rf = flat_foot(0.0)
    return state(0.0, py, 82.0, 74.0, 86.0, rf, left_leg(2), {"l": (0.55, 0.06), "r": (0.50, 0.08)}, rarm=0.66)


def swing_body_pose():
    py = h.hh + 0.045
    xp = 0.20
    ank = (0.34, h.ah + 0.10)
    rf = dict(ankle=ank, foot=-h.theta + 18.0, toe=18.0)
    return state(xp, py, 84.0, 78.0, 88.0, rf, left_leg(-4), {"l": (0.57, 0.0), "r": (0.52, 0.0)}, rarm=0.66)


def rarm_at(u):
    if u < 0.7: return 0.64 + 0.035 * (u / 0.7)
    if u < 1.35: return 0.675 - 0.035 * smooth((u - 0.7) / 0.65)
    return 0.64


def gait_state(t):
    """one swing-to cycle of T=1.8 s: A crutches advance (0-0.7) on the right foot, B body swings to the crutch line (0.7-1.35), C landing and loading (1.35-1.8)."""
    D = D_STEP
    u = t
    tip0r, tip0l = 0.12, 0.17
    A_END, B_END = 0.7, 1.35
    rarm = rarm_at(u)
    if u < A_END:
        s = u / A_END
        sm = smooth(s)
        lift = 0.10 * math.sin(math.pi * s) ** 1.5
        tr = (tip0r + D * sm, lift)
        tl = (tip0l + D * smooth(max(0, min(1, s * 1.1 - 0.05))), 0.9 * lift)
        px = 0.0
        py = h.hh - 0.02 + 0.006 * math.sin(math.pi * s)
        rf = flat_foot(0.0)
        torso = 84.0 - 4.0 * math.sin(math.pi * s)
        ll = left_leg(3 * math.sin(math.pi * s))
    elif u < B_END:
        s = (u - A_END) / (B_END - A_END)
        sm = smooth(s)
        tr = (tip0r + D, 0.0)
        tl = (tip0l + D, 0.0)
        px = D * sm
        py = h.hh - 0.02 + 0.06 * math.sin(math.pi * s) ** 0.8
        if s < 0.2:
            sg = s / 0.2
            ang = FLAT - 28.0 * smooth(sg)
            rf = dict(ankle=h.foot_pose((h.Lball, 0.0), "ball", ang), foot=ang, toe=0.0)
        else:
            sg = (s - 0.2) / 0.8
            a0 = h.foot_pose((h.Lball, 0.0), "ball", FLAT - 28.0)
            a1 = h.foot_pose((D - h.heel_back, 0.0), "heel", FLAT + 14.0)
            sm2 = smooth(sg)
            ank = (a0[0] + (a1[0] - a0[0]) * sm2, a0[1] + (a1[1] - a0[1]) * sm2 + 0.10 * math.sin(math.pi * sg) ** 1.2)
            ang = (FLAT - 28.0) + ((FLAT + 14.0) - (FLAT - 28.0)) * smooth(min(1, sg * 1.2))
            rf = dict(ankle=ank, foot=ang, toe=ang + h.theta + 28.0 * (1 - smooth(min(1, sg * 1.5))))
        torso = 80.0 + 4.0 * math.sin(math.pi * s)
        ll = left_leg(-6 * math.sin(math.pi * s))
    else:
        s = (u - B_END) / (T_GAIT - B_END)
        tr = (tip0r + D, 0.0)
        tl = (tip0l + D, 0.0)
        px = D
        py = h.hh - 0.02 + 0.02 * (1 - smooth(min(1, s * 2.0)))
        xh = D - h.heel_back
        if s < 0.3:
            ang = FLAT + 14.0 * (1 - smooth(s / 0.3))
            rf = dict(ankle=h.foot_pose((xh, 0.0), "heel", ang), foot=ang, toe=ang + h.theta)
        else:
            rf = dict(ankle=h.foot_pose((xh, 0.0), "heel", FLAT), foot=FLAT, toe=0.0)
        torso = 84.0
        ll = left_leg(0.0)
    st = state(px, py, torso, torso - 7, torso + 5, rf, ll, {"l": tl, "r": tr}, rarm=rarm)
    return st, (px, py)
