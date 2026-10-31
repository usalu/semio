from h3lib import *
sex, BODY, PHONE, PHONES_HP = "f", 58.96, 0.19, 0.25
MASS = round(BODY + PHONE + PHONES_HP, 3)
rig, anchors, L = build_rig(sex, BODY, stature=1.69, sL=0.95, sF=0.88, sH=0.97, sA=0.95)
bone(rig, "neck")["mass"]["kg"] = round(bone(rig, "neck")["mass"]["kg"] + PHONES_HP, 3)
add_bone(rig, "phone", "hand-r", 0.085 * 0.95, 0.022, 18, 0.148, {"type": "fixed"}, T("Smartphone in case", "Smartphone mit Hülle"),
         {"kg": PHONE, "comX": 0.0, "comY": 0, "distribution": "point", "source": "authored-estimate"})
bone(rig, "phone")["x"] = 0.0807
mass_sum_fix(rig, MASS)
anchors += [
    {"id": "phone-cog", "bone": "phone", "x": 0.0, "y": 0.0, "kind": "load", "title": T("Phone centre of gravity", "Schwerpunkt des Telefons")},
    {"id": "grip-l", "bone": "hand-l", "x": 0.09, "y": 0.0, "kind": "attachment", "title": T("Left thumb on the screen", "Linker Daumen auf dem Bildschirm")},
    {"id": "ear", "bone": "head", "x": 0.045, "y": 0.0, "kind": "attachment", "title": T("Ear (phone call)", "Ohr (Telefonat)")},
]
a = asset_stub(rig, anchors)
CONTACTS = ["heel-l", "ball-l", "heel-r", "ball-r"]
