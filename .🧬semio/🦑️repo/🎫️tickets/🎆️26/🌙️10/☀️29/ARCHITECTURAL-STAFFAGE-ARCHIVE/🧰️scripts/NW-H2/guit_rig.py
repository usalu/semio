"""Seated guitarist: stool root, rig, anchors, IK poses."""
from lib import *
from human import *

H = 1.76
MASS = 74.0
SEAT = 0.56
RUNG = 0.25
GUITAR_L = 0.78


def build_rig():
    bones = []
    bones.append(bone("stool", None, "Wooden bar stool with foot rung", "Barhocker aus Holz mit Fußraste", 0, 0, 0, 0.56, {"type": "fixed"}, {"kg": 3.4, "comX": 0.0, "comY": 0.3, "distribution": "point", "source": "authored-estimate", "reference": "guitar-spec"}))
    bones.append(bone("hip-lift", "stool", "Hip height above the stool", "Hüfthöhe über dem Hocker", 0, SEAT + 0.09, 0, 0.01, {"type": "prismatic", "axisDeg": 90, "minM": -0.1, "maxM": 0.45}))
    bones.append(bone("hip-shift", "hip-lift", "Hip shift on the seat", "Hüftverschiebung auf dem Sitz", 0, 0, 0, 0.01, {"type": "prismatic", "axisDeg": 0, "minM": -0.2, "maxM": 0.4}))
    body, L = human_bones(H, True, MASS, parent_of_pelvis="hip-shift", foot_min=-65)
    bones += body
    bones.append(bone("guitar", "torso", "Acoustic guitar (dreadnought)", "Akustikgitarre (Dreadnought)", 0.12, -0.17, -58, GUITAR_L, {"type": "revolute", "minDeg": -25, "maxDeg": 25}, {"kg": 2.1, "comX": 0.20, "comY": 0.0, "distribution": "point", "source": "authored-estimate", "reference": "guitar-spec"}))
    return bones, L


def build_anchors(L):
    a = human_anchors(L, H)
    f = H / 1.82
    for x in a:
        if x["id"] == "nostrils":
            x["x"], x["y"] = round(0.145 * f, 4), round(-0.102 * f, 4)
        if x["id"] == "mouth":
            x["x"], x["y"] = round(0.117 * f, 4), round(-0.092 * f, 4)
    a.append({"id": "stool-ground", "bone": "stool", "x": 0.0, "y": 0.0, "kind": "contact", "title": tx("Stool legs on the floor", "Hockerbeine auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.32, "footprintWidthM": 0.32, "frictionCoefficient": 0.5})
    a.append({"id": "seat", "bone": "pelvis", "x": -0.09, "y": 0.0, "kind": "support", "support": "contact", "normalDeg": 90, "footprintLengthM": 0.2, "footprintWidthM": 0.3, "frictionCoefficient": 0.5, "title": tx("Seat bones on the stool", "Sitzbeinhöcker auf dem Hocker")})
    a.append({"id": "rung-support", "bone": "foot-l", "x": 0.08, "y": -0.0775, "kind": "support", "support": "contact", "normalDeg": 90, "footprintLengthM": 0.12, "footprintWidthM": 0.09, "frictionCoefficient": 0.6, "title": tx("Near foot on the rung", "Naher Fuß auf der Fußraste")})
    a.append({"id": "fret-hand", "bone": "guitar", "x": 0.50, "y": -0.06, "kind": "attachment", "title": tx("Fretting hand position", "Position der Griffhand")})
    a.append({"id": "strum-hand", "bone": "guitar", "x": 0.14, "y": 0.04, "kind": "attachment", "title": tx("Strumming hand position", "Position der Schlaghand")})
    a.append({"id": "guitar-rest", "bone": "guitar", "x": 0.0, "y": -0.10, "kind": "support", "support": "contact", "normalDeg": 90, "footprintLengthM": 0.2, "footprintWidthM": 0.1, "frictionCoefficient": 0.5, "title": tx("Guitar resting on the thigh", "Gitarre auf dem Oberschenkel")})
    return a


bones, L = build_rig()
FLOOR = 0.0775 * H / 1.76 / 0.0  if False else L["ankle"]


def seat_pose(hip_x=0.0, torso=84, head=72, guitar_rel=0, near=(0.34, None), far=(0.30, None), near_foot=0, far_foot=0, hipy=SEAT + 0.09):
    f = Fig(bones)
    f.place_pelvis(hip_x, hipy)
    f.rel("pelvis", 0)
    f.aim("torso", torso)
    f.rel("guitar", guitar_rel)
    f.aim("head", head)
    f.leg("l", (near[0], RUNG + L["ankle"]), 1)
    f.leg("r", (far[0], L["ankle"]), 1)
    f.aim("foot-l", near_foot)
    f.aim("foot-r", far_foot)
    return f


def hands(f, fret=0.50, strum=(0.14, 0.04), fret_off=-0.06, hand_l=80, hand_r=-20, bend_r=-1):
    f.arm("l", f.at("guitar", (fret, fret_off)), -1)
    f.aim("hand-l", hand_l)
    f.arm("r", f.at("guitar", strum), bend_r)
    f.aim("hand-r", f.world("forearm-r")[2] + hand_r)


PARAMS = {"fret": 0.62, "fh_torso": 80, "fh_head": 62, "rest_l": (0.22, 0.06), "rest_r": (0.10, 0.05), "rest_torso": 88, "rest_head": 85}
import json, os
if os.path.exists("/tmp/nwh2/guit_params.json"):
    PARAMS.update(json.load(open("/tmp/nwh2/guit_params.json")))


def poses():
    P = {}
    f = seat_pose()
    hands(f)
    P["strumming"] = f
    f = seat_pose(torso=PARAMS["fh_torso"], head=PARAMS["fh_head"])
    hands(f, fret=PARAMS["fret"], strum=(0.12, 0.05))
    P["fretting-high"] = f
    f = seat_pose(torso=PARAMS["rest_torso"], head=PARAMS["rest_head"], guitar_rel=10)
    f.arm("l", f.at("guitar", tuple(PARAMS["rest_l"])), -1)
    f.aim("hand-l", 10)
    f.arm("r", f.at("guitar", tuple(PARAMS["rest_r"])), -1)
    f.aim("hand-r", 10)
    P["resting"] = f
    f = seat_pose(torso=74, head=52)
    hands(f, fret=0.70, strum=(0.62, 0.0), fret_off=-0.05, hand_l=60)
    P["tuning"] = f
    return P


if __name__ == "__main__":
    P = poses()
    for n, f in P.items():
        print(n, joint_report(f))
    render(skeleton_svg(list(P.values()), list(P), 30, x_range=(-0.6, 1.2), y_range=(-0.1, 1.6)), "guit-skel", 2000)
