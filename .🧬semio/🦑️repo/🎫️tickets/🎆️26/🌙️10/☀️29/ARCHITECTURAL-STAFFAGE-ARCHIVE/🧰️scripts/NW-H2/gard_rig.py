"""Kneeling gardener: rig, anchors, IK poses. World frame: soil at y = 0, the kneeling pad is the root bone."""
from lib import *
from human import *

H = 1.66
MASS = 64.0
PAD_T = 0.04
SHIN_Y = 0.105          # shank axis height while the shin lies on the pad
TROWEL_L = 0.30
TRUG_X = 1.02


def build_rig():
    bones = []
    bones.append(bone("pad", None, "Kneeling pad on the soil", "Kniekissen auf dem Boden", 0, 0, 0, 0.6, {"type": "fixed"}, {"kg": 0.6, "comX": 0.0, "comY": 0.02, "distribution": "point", "source": "authored-estimate", "reference": "garden-spec"}))
    bones.append(bone("trug", "pad", "Flexible garden trug with weeds", "Flexibler Gartentrog mit Unkraut", TRUG_X, 0, 0, 0.5, {"type": "fixed"}, {"kg": 1.6, "comX": 0.0, "comY": 0.1, "distribution": "point", "source": "authored-estimate", "reference": "garden-spec"}))
    bones.append(bone("hip-lift", "pad", "Hip height above the pad", "Hüfthöhe über dem Kissen", 0, 0.5, 0, 0.01, {"type": "prismatic", "axisDeg": 90, "minM": -0.2, "maxM": 0.5}))
    bones.append(bone("hip-shift", "hip-lift", "Hip shift along the pad", "Hüftverschiebung entlang des Kissens", 0, 0, 0, 0.01, {"type": "prismatic", "axisDeg": 0, "minM": -0.6, "maxM": 0.9}))
    body, L = human_bones(H, False, MASS, parent_of_pelvis="hip-shift", foot_min=-65)
    bones += body
    bones.append(bone("trowel", "hand-r", "Hand trowel", "Handschaufel", 0.07, 0.0, 0, TROWEL_L, {"type": "revolute", "minDeg": -45, "maxDeg": 45}, {"kg": 0.25, "comX": 0.15, "comY": 0.0, "distribution": "point", "source": "authored-estimate", "reference": "garden-spec"}))
    return bones, L


def build_anchors(L):
    a = human_anchors(L, H)
    f = H / 1.82
    for s in ("l", "r"):
        e, g = ("Left", "Linke") if s == "l" else ("Right", "Rechte")
        a.append({"id": f"shin-{s}", "bone": f"shank-{s}", "x": 0.20, "y": 0.065, "kind": "contact", "title": tx(f"{e} shin on the pad", f"{g} Schienbein auf dem Kissen"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.28, "footprintWidthM": 0.10, "frictionCoefficient": 0.6})
        a.append({"id": f"instep-{s}", "bone": f"foot-{s}", "x": 0.06, "y": 0.04, "kind": "contact", "title": tx(f"{e} instep on the pad", f"{g} Fußrücken auf dem Kissen"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.12, "footprintWidthM": 0.08, "frictionCoefficient": 0.6})
    a.append({"id": "pad-ground", "bone": "pad", "x": 0.0, "y": 0.0, "kind": "contact", "title": tx("Pad on the soil", "Kissen auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.5, "footprintWidthM": 0.3, "frictionCoefficient": 0.6})
    a.append({"id": "palm-l", "bone": "hand-l", "x": 0.09, "y": -0.03, "kind": "contact", "title": tx("Left palm", "Linke Handfläche"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.08, "frictionCoefficient": 0.6})
    a.append({"id": "trowel-tip", "bone": "trowel", "x": TROWEL_L, "y": 0.0, "kind": "attachment", "title": tx("Trowel tip", "Schaufelspitze")})
    a.append({"id": "trug-grip", "bone": "trug", "x": 0.0, "y": 0.2, "kind": "load", "title": tx("Trug handle", "Trog-Griff")})
    for x in a:
        if x["id"] == "nostrils":
            x["x"], x["y"] = round(0.145 * f, 4), round(-0.102 * f, 4)
        if x["id"] == "mouth":
            x["x"], x["y"] = round(0.117 * f, 4), round(-0.092 * f, 4)
    return a


bones, L = build_rig()


def kneel(hip, thigh_deg=-80.0, torso_deg=35, head_deg=50, shank_deg=180.0, foot_deg=210.0, far_dx=0.0):
    f = Fig(bones)
    f.place_pelvis(*hip)
    f.rel("pelvis", 0)
    f.aim("torso", torso_deg)
    f.aim("head", head_deg)
    for s, dx in (("l", 0.0), ("r", far_dx)):
        f.aim(f"thigh-{s}", thigh_deg + dx * 20)
        f.aim(f"shank-{s}", shank_deg + dx * 10)
        f.aim(f"foot-{s}", foot_deg)
        f.rel(f"toe-{s}", 0)
    return f


def wrist_for(point, hand_dir, local):
    c, s = rot(hand_dir)
    return (point[0] - (c * local[0] - s * local[1]), point[1] - (s * local[0] + c * local[1]))


def arm_to(f, side, wrist, hand_dir, bend=-1):
    f.arm(side, wrist, bend)
    f.aim(f"hand-{side}", hand_dir)


def poses():
    P = {}
    # weeding: right hand digs, left hand presses on the left thigh
    f = kneel((0.05, 0.50), torso_deg=32, head_deg=48, far_dx=0.05)
    tip = (0.70, 0.0)
    hd = -50
    c, s = rot(hd)
    wr = (tip[0] - (0.07 + TROWEL_L) * c, tip[1] - (0.07 + TROWEL_L) * s)
    arm_to(f, "r", wr, hd, -1)
    f.rel("trowel", 0)
    arm_to(f, "l", (0.06, 0.42), -80, -1)
    P["kneeling-weeding"] = f
    f = kneel((-0.02, 0.50), torso_deg=82, head_deg=92, far_dx=0.05)
    arm_to(f, "r", (0.30, 0.66), -20, -1)
    f.rel("trowel", -15)
    arm_to(f, "l", (0.20, 0.44), -70, -1)
    P["kneeling-upright"] = f
    f = kneel((0.12, 0.50), thigh_deg=-76, torso_deg=14, head_deg=26, far_dx=0.05)
    arm_to(f, "r", (0.78, 0.52), -25, -1)
    f.rel("trowel", 20)
    arm_to(f, "l", (0.22, 0.30), -80, -1)
    P["reaching-trug"] = f
    return P


if __name__ == "__main__":
    P = poses()
    for n, f in P.items():
        print(n, joint_report(f), "trowel tip", [round(v, 3) for v in f.at("trowel", (TROWEL_L, 0))], "palm-l", [round(v, 3) for v in f.at("hand-l", (0.09, -0.03))])
    render(skeleton_svg(list(P.values()), list(P), 25, x_range=(-0.8, 1.3), y_range=(-0.1, 1.2)), "gard-skel", 2000)
