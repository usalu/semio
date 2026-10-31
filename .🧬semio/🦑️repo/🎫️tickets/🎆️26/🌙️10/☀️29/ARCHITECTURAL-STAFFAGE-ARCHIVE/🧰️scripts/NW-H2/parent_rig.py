"""Parent carrying a toddler on the hip: rig (pelvis root, toddler bones), anchors and gait/IK poses."""
import math
from lib import *
from human import *

H = 1.70
MASS = 66.0
TMASS = 12.2
TH = 0.865
bones0, L = human_bones(H, False, MASS, parent_of_pelvis=None, foot_min=-65)

TL = dict(torso=0.27, head=0.19, arm=0.20, thigh=0.13, shank=0.14)
TFRAC = dict(torso=0.45, head=0.15, arm=0.05, thigh=0.08, shank=0.07)


def tbone(i, parent, en, de, x, y, ang, length, lo, hi, frac):
    m = {"kg": round(TMASS * frac, 3), "comX": round(length * 0.5, 4), "comY": 0.0, "distribution": "point", "source": "literature", "reference": "who-2006"}
    return bone(i, parent, en, de, x, y, ang, length, {"type": "revolute", "minDeg": lo, "maxDeg": hi}, m)


def build_rig():
    bones = list(bones0)
    bones.append(tbone("toddler-torso", "pelvis", "Toddler trunk", "Rumpf des Kleinkinds", 0.105, -0.15, -14, TL["torso"], -30, 30, TFRAC["torso"]))
    bones.append(tbone("toddler-head", "toddler-torso", "Toddler head and neck", "Kopf und Hals des Kleinkinds", TL["torso"], 0, 0, TL["head"], -40, 40, TFRAC["head"]))
    bones.append(tbone("toddler-arm-n", "toddler-torso", "Toddler near arm", "Naher Arm des Kleinkinds", TL["torso"] - 0.03, 0, 84, TL["arm"], -60, 120, TFRAC["arm"]))
    bones.append(tbone("toddler-arm-f", "toddler-torso", "Toddler far arm", "Ferner Arm des Kleinkinds", TL["torso"] - 0.03, 0, -150, TL["arm"], -90, 120, TFRAC["arm"]))
    for s, en, de, ang in (("n", "near", "nahes", -78), ("f", "far", "fernes", -84)):
        bones.append(tbone(f"toddler-thigh-{s}", "toddler-torso", f"Toddler {en} thigh", f"{de.capitalize()} Bein, Oberschenkel des Kleinkinds", 0.0, 0.0, ang, TL["thigh"], -60, 60, TFRAC["thigh"]))
        bones.append(tbone(f"toddler-shank-{s}", f"toddler-thigh-{s}", f"Toddler {en} shank and foot", f"{de.capitalize()} Bein, Unterschenkel und Fuß des Kleinkinds", TL["thigh"], 0, -62, TL["shank"], -100, 10, TFRAC["shank"]))
    return bones


def build_anchors():
    a = human_anchors(L, H)
    f = H / 1.82
    for x in a:
        if x["id"] == "nostrils":
            x["x"], x["y"] = round(0.145 * f, 4), round(-0.102 * f, 4)
        if x["id"] == "mouth":
            x["x"], x["y"] = round(0.117 * f, 4), round(-0.092 * f, 4)
    k = TH / 1.82
    a.append({"id": "toddler-mouth", "bone": "toddler-head", "x": 0.075, "y": -0.062, "kind": "emitter", "title": tx("Toddler mouth", "Mund des Kleinkinds")})
    a.append({"id": "toddler-nostrils", "bone": "toddler-head", "x": 0.092, "y": -0.070, "kind": "emitter", "title": tx("Toddler nostrils", "Nasenlöcher des Kleinkinds")})
    a.append({"id": "toddler-crown", "bone": "toddler-head", "x": 0.185, "y": 0.0, "kind": "emitter", "title": tx("Toddler crown (plume source)", "Scheitel des Kleinkinds (Fahnenquelle)")})
    a.append({"id": "toddler-seat", "bone": "toddler-torso", "x": 0.0, "y": 0.0, "kind": "load", "title": tx("Toddler seat on the hip", "Sitzfläche des Kleinkinds auf der Hüfte")})
    return a


def walk_phase(phi, step=0.54, stance=0.62, clear=0.115, angle_hs=14):
    """Left-foot ankle target relative to the pelvis origin for gait phase phi in [0, 1): returns (x, y_rel_to_ground_ankle, foot_world_deg).
    y is the ankle height above the ground; stance foot moves back at constant speed, swing foot follows a sine arc."""
    a = 0.0772 * (H / 1.70)
    half = step / 2
    if phi < stance:
        u = phi / stance
        x = half - step * u
        ang = angle_hs * (1 - u * 4) if u < 0.25 else (0 if u < 0.75 else -32 * (u - 0.75) / 0.25)
        y = a if u < 0.75 else a + 0.045 * (u - 0.75) / 0.25
    else:
        u = (phi - stance) / (1 - stance)
        x = -half + step * (0.5 - 0.5 * math.cos(math.pi * u))
        y = a + 0.045 * (1 - u) + clear * math.sin(math.pi * u) ** 1.2
        ang = -32 * (1 - u) + angle_hs * u * 0.5
    return x, y, ang


def make_walk(phi, torso=91, hs=None):
    """Walking pose with the left foot at phase phi (right foot at phi + 0.5); pelvis at the origin, ground below."""
    f = Fig(bones)
    ll = (L["thigh"] + L["shank"])
    xl, yl, al = walk_phase(phi)
    xr, yr, ar = walk_phase((phi + 0.5) % 1.0)
    xs = [abs(x) for x, y, ang in ((xl, yl, al), (xr, yr, ar)) if y < 0.10]
    xm = max(xs) if xs else 0.0
    hp = math.sqrt((0.99 * ll) ** 2 - xm ** 2)
    f.root["x"] = 0
    f.rel("pelvis", 0)
    f.aim("torso", torso)
    f.aim("head", torso + 0)
    f.leg("l", (xl, yl - hp), 1)
    f.leg("r", (xr, yr - hp), 1)
    f.aim("foot-l", al)
    f.aim("foot-r", ar)
    return f, hp


bones = build_rig()


def arms_carry(f, swing=0.0, toddler_seat_dy=0.0, hip_y=0.0):
    """Near arm cradles the toddler's seat, far arm swings."""
    hip = f.world("pelvis")[:2]
    f.arm("l", (hip[0] + 0.15, hip[1] - 0.035 + toddler_seat_dy), -1)
    f.aim("hand-l", f.world("forearm-l")[2] + 10)
    sh = f.world("upper-arm-r")[:2]
    ang = -90 + swing
    f.aim("upper-arm-r", ang)
    f.aim("forearm-r", ang + 18 + abs(swing) * 0.4)
    f.aim("hand-r", ang + 18 + abs(swing) * 0.4)


def toddler_pose(f, bob=0.0, look=0.0):
    f.rel("toddler-torso", bob * 0.5)
    f.rel("toddler-head", look)
    f.rel("toddler-arm-n", 0)
    f.rel("toddler-arm-f", 0)
    f.rel("toddler-thigh-n", 0)
    f.rel("toddler-shank-n", 0)
    f.rel("toddler-thigh-f", 0)
    f.rel("toddler-shank-f", 0)


def poses():
    P = {}
    f = Fig(bones)
    f.aim("torso", 92)
    f.aim("head", 92)
    f.leg("l", (0.03, -L["thigh"] - L["shank"] * 0.985), 1)
    f.leg("r", (0.015, -L["thigh"] - L["shank"] * 0.985), 1)
    f.aim("foot-l", 0)
    f.aim("foot-r", 0)
    arms_carry(f)
    toddler_pose(f)
    P["standing-carry"] = f
    return P


if __name__ == "__main__":
    P = poses()
    figs = list(P.values())
    names = list(P)
    for k in range(8):
        f, hp = make_walk(k / 8)
        arms_carry(f, swing=20 * math.sin(2 * math.pi * k / 8))
        toddler_pose(f)
        figs.append(f)
        names.append(f"walk{k}")
        print(k, joint_report(f), round(hp, 3))
    for n, f in P.items():
        print(n, joint_report(f))
    for f in figs:
        f.root["y"] = 0.9
    render(skeleton_svg(figs, names, 30, x_range=(-0.6, 0.8), y_range=(-0.1, 1.95), ground_y=0.0), "parent-skel", 2400)
