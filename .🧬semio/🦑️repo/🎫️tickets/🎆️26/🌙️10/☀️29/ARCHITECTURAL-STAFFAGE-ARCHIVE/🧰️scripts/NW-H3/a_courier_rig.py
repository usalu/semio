from h3lib import *
sex = "m"
BODY, TRUCK, PARCELS = 84.5, 11.0, 38.0
MASS = round(BODY + TRUCK + PARCELS, 3)
rig, anchors, L = build_rig(sex, BODY, stature=1.82, sL=1.02, sF=1.07, sH=1.0, sA=1.02)
HIP = L["hip_y"]
WHEEL_R = 0.13
TILT = 22.0
D_AX = 0.66
add_bone(rig, "truck-lift", "pelvis", WHEEL_R - HIP, -D_AX, -90, 0.2, {"type": "prismatic", "axisDeg": 0, "minM": -0.15, "maxM": 0.15}, T("Hand truck vertical float on the ground", "Senkrechte Führung der Sackkarre am Boden"))
add_bone(rig, "hand-truck", "truck-lift", 0, 0, 90 + TILT, 1.12, {"type": "revolute", "minDeg": -27, "maxDeg": 70}, T("Hand truck frame, tilted back on its axle", "Sackkarrenrahmen, auf der Achse nach hinten geneigt"),
         {"kg": TRUCK, "comX": 0.42, "comY": -0.01, "distribution": "point", "source": "authored-estimate"})
add_bone(rig, "parcels", "hand-truck", 0, 0, 0, 0.9, {"type": "fixed"}, T("Stack of three parcels", "Stapel aus drei Paketen"),
         {"kg": PARCELS, "comX": 0.36, "comY": -0.19, "distribution": "point", "source": "authored-estimate"})
mass_sum_fix(rig, MASS)
anchors += [
    {"id": "wheel-contact", "bone": "truck-lift", "x": 0.0, "y": -WHEEL_R, "kind": "contact", "title": T("Wheel on the ground", "Rad am Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.03, "footprintWidthM": 0.05, "frictionCoefficient": 0.6},
    {"id": "handle-grip", "bone": "hand-truck", "x": 0.96, "y": 0.02, "kind": "attachment", "title": T("Handle bar", "Griffholm")},
    {"id": "parcels-cog", "bone": "parcels", "x": 0.36, "y": -0.19, "kind": "load", "title": T("Centre of gravity of the parcels", "Schwerpunkt der Pakete")},
    {"id": "grip-r", "bone": "hand-r", "x": 0.09 * 1.02, "y": 0.0, "kind": "attachment", "title": T("Right glove on the handle", "Rechter Handschuh am Griff")},
    {"id": "grip-l", "bone": "hand-l", "x": 0.09 * 1.02, "y": 0.0, "kind": "attachment", "title": T("Left glove on the handle", "Linker Handschuh am Griff")},
]
a = asset_stub(rig, anchors)
FEET = ["heel-l", "ball-l", "heel-r", "ball-r"]
CONTACTS = FEET + ["wheel-contact"]
