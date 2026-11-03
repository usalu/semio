import sys, copy
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️M2")
from lib import *

A = "sorption-wheel"
doc = load(A)
GS = "galvanised-steel"
reps = doc["asset"]["representations"]


def ext(e, sid, z0, z1, z=None):
    s = copy.deepcopy(e[sid])
    s["solid"] = {"kind": "extruded", "z0": z0, "z1": z1}
    if z is not None:
        s["z"] = z
    return s


def build(scale, stand_h, stand_w):
    e = {s["id"]: s for s in reps[str(scale)]["geometry"]["elevation"]}
    base = [R("stand", "casing", 3.1, 0.35, -0.11, 0, 0.22, stand_h, "casing", GS), ext(e, "casing", -0.11, 0.11), ext(e, "disc", -0.09, 0.09)]
    front = base + [ext(e, "regen-sector", -0.09, 0.09)]
    plan = [R("stand", "casing", 3.1, 0.35, -stand_w, -0.09, 2 * stand_w, 0.18, "casing", GS), ext(e, "casing", -0.11, 0.11), ext(e, "disc", -0.09, 0.09)]
    return front, plan


TXT = {
    50: ("the casing ring and the wheel rim are extruded from their elevation circles across the 0.22 m depth, the hot regeneration sector shows on the near rim, the stand carries the casing. The wheel turns about the lateral axis, so its rim is edge-on and the rotation is not visible from the front or from above; the rotor stays drawn on its bone.",
         "Gehäusering und Radkranz sind aus ihren Aufrisskreisen über die Tiefe von 0,22 m extrudiert, der heiße Regenerationssektor zeigt sich am vorderen Kranz, der Fuß trägt das Gehäuse. Das Rad dreht sich um die Querachse, sein Kranz steht daher von vorn und von oben hochkant und die Drehung ist nicht sichtbar; der Rotor bleibt an seinem Knochen gezeichnet."),
    100: ("as at 1:50: casing, rim, regeneration sector and stand.", "wie bei 1:50: Gehäuse, Kranz, Regenerationssektor und Fuß."),
    200: ("casing, rim, sector and stand as four masses.", "Gehäuse, Kranz, Sektor und Fuß als vier Massen."),
}
for sc, h, w in ((50, 0.03, 0.12), (100, 0.04, 0.14), (200, 0.04, 0.15)):
    f, p = build(sc, h, w)
    put(doc, sc, f, p, *TXT[sc])
save(A, doc)
print("ok")
