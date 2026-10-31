from h3lib import *
sex = "f"
BELLY, CLOTHES = 6.0, 1.4
BODYSEG = 71.0 - BELLY + CLOTHES
MASS = round(71.0 + CLOTHES, 3)
rig, anchors, L = build_rig(sex, BODYSEG, stature=1.66, sL=0.935, sF=0.86, sH=0.97, sA=0.94)
HIP = L["hip_y"]
for s in "lr":
    bone(rig, f"foot-{s}")["joint"].update(maxDeg=40)
add_bone(rig, "belly", "torso", 0.03, 0.0, -90, 0.24, {"type": "revolute", "minDeg": -3, "maxDeg": 3}, T("Gravid abdomen with the fetal unit", "Schwangerer Bauch mit der Frucht"),
         {"kg": BELLY, "comX": 0.11, "comY": 0.0, "distribution": "point", "source": "authored-estimate", "reference": "hytten-chamberlain-1980"})
mass_sum_fix(rig, MASS)
anchors += [
    {"id": "belly-front", "bone": "belly", "x": 0.225, "y": 0.0, "kind": "attachment", "title": T("Front of the abdomen", "Vorderseite des Bauchs")},
    {"id": "belly-cog", "bone": "belly", "x": 0.11, "y": 0.0, "kind": "load", "title": T("Centre of the fetal unit", "Schwerpunkt der Frucht")},
    {"id": "palm-r", "bone": "hand-r", "x": 0.08, "y": 0.0, "kind": "attachment", "title": T("Right palm", "Rechte Handfläche")},
    {"id": "palm-l", "bone": "hand-l", "x": 0.08, "y": 0.0, "kind": "attachment", "title": T("Left palm", "Linke Handfläche")},
]
a = asset_stub(rig, anchors)
CONTACTS = ["heel-l", "ball-l", "heel-r", "ball-r"]
SA = 0.94
