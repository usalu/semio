from h3lib import *
sex = "m"
rig, anchors, L = build_rig(sex, 72.0, stature=1.80, sL=1.0, sF=0.98, sH=1.0, sA=1.0)
HIP = L["hip_y"]
bones = rig["bones"]
rig["bones"] = [b for b in bones if b["id"] not in ("foot-r", "toe-r")]
anchors = [x for x in anchors if x["id"] not in ("heel-r", "ball-r")]
sh = bone(rig, "shank-r")
SOC = 0.18
sh.update(lengthM=SOC, title=T("Residual limb in the carbon socket and pylon", "Stumpf im Karbonschaft mit Pylon"))
sh["mass"] = {"kg": 1.9, "comX": 0.09, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "ossur-cheetah-xtreme"}
for b in rig["bones"]:
    if b["id"] == "foot-l":
        b["joint"].update(maxDeg=40)
    if b["id"].startswith("thigh"):
        b["joint"].update(minDeg=-55)
add_bone(rig, "blade-r", "shank-r", SOC, 0, -15, 0.22, {"type": "revolute", "minDeg": -22, "maxDeg": 25}, T("Upper curve of the carbon blade, attached at the socket adapter", "Oberer Bogen der Karbonfeder, am Schaftadapter befestigt"),
         {"kg": 0.55, "comX": 0.11, "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "ossur-cheetah-xtreme"})
add_bone(rig, "blade-tip-r", "blade-r", 0.22, 0, 82, 0.28, {"type": "revolute", "minDeg": -12, "maxDeg": 38}, T("Lower curl of the blade with the rubber toe, flexes under load", "Unterer Bogen der Feder mit Gummispitze, federt unter Last ein"),
         {"kg": 0.35, "comX": 0.14, "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "ossur-cheetah-xtreme"})
anchors += [
    {"id": "blade-toe", "bone": "blade-tip-r", "x": 0.28, "y": 0.0, "kind": "contact", "title": T("Rubber toe of the blade", "Gummispitze der Feder"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.075, "frictionCoefficient": 0.9},
    {"id": "adapter", "bone": "shank-r", "x": SOC, "y": 0.0, "kind": "attachment", "title": T("Socket adapter", "Schaftadapter")},
]
MASS = round(total_mass(rig), 3)
a = asset_stub(rig, anchors)
FEET = ["heel-l", "ball-l", "blade-toe"]
B1, B2 = 0.22, 0.28
