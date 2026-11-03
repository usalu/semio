import sys, math
sys.path.insert(0, r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🧰️M2")
from lib import *

A = "plate-heat-exchanger"
doc = load(A)
GS, AL = "galvanised-steel", "aluminium-plate"

doc["asset"]["rig"]["bones"] = [b for b in doc["asset"]["rig"]["bones"] if b["id"] != "damper-lift"]
SOL = {"kind": "extruded", "z0": -0.2, "z1": 0.2}


def flap(h):
    return R("damper", "damper", 3.6, 0.3, 0, -0.02, 0.4, h, "damper", GS, solid=SOL)


def collars_plan(h_each):
    return [
        R("collar-left", "ports", 3.2, 0.4, -0.31, -0.15, 0.06, 0.3, "ports", GS),
        R("collar-right", "ports", 3.2, 0.4, 0.25, -0.15, 0.06, 0.3, "ports", GS),
    ]


front50 = [
    R("frame", "frame", 3.3, 0.35, -0.25, 0, 0.5, 0.6, "frame", GS),
    R("plate-pack", "plates", 3.4, 0.7, -0.22, 0.03, 0.44, 0.54, "plates", AL, hatch="plates"),
    R("collar-upper", "ports", 3.5, 0.4, -0.15, 0.4, 0.3, 0.15, "ports", GS),
    R("collar-lower", "ports", 3.5, 0.4, -0.15, 0.06, 0.3, 0.15, "ports", GS),
    flap(0.02),
]
plan50 = [
    R("frame", "frame", 3.3, 0.35, -0.25, -0.25, 0.5, 0.5, "frame", GS),
    *collars_plan(0),
    R("plate-pack", "plates", 3.4, 0.7, -0.2, -0.2, 0.4, 0.4, "plates", AL, hatch="plates"),
    R("damper-hinge", "damper", 3.55, 0.3, -0.22, -0.2, 0.02, 0.4, "damper", GS, role="detail"),
    flap(0.02),
]
put(doc, 50, front50, plan50,
    "the casing shows its open front with the hatched plate edges and both duct collars standing in front of it; the damper flap is a solid extruded across the casing width, so it rises with the flap. From above the casing top carries the two collars, the plate pack under the flap opening and the flap, which shortens as it lifts.",
    "das Gehäuse zeigt seine offene Stirnseite mit den schraffierten Plattenkanten, davor stehen beide Kanalstutzen; die Klappe ist ein über die Gehäusebreite extrudierter Körper und steigt mit der Klappe. Von oben trägt das Gehäusedach die beiden Stutzen, das Plattenpaket unter der Klappenöffnung und die Klappe, die sich beim Öffnen verkürzt.")

front100 = [
    R("frame", "frame", 3.3, 0.35, -0.25, 0, 0.5, 0.6, "frame", GS),
    R("plate-pack", "plates", 3.4, 0.7, -0.21, 0.03, 0.42, 0.54, "plates", AL, hatch="plates"),
    R("collar-upper", "ports", 3.5, 0.4, -0.15, 0.4, 0.3, 0.15, "ports", GS),
    R("collar-lower", "ports", 3.5, 0.4, -0.15, 0.06, 0.3, 0.15, "ports", GS),
    flap(0.02),
]
plan100 = [
    R("frame", "frame", 3.3, 0.35, -0.25, -0.25, 0.5, 0.5, "frame", GS),
    *collars_plan(0),
    R("plate-pack", "plates", 3.4, 0.7, -0.2, -0.2, 0.4, 0.4, "plates", AL, hatch="plates"),
    flap(0.02),
]
put(doc, 100, front100, plan100,
    "front and top view keep the casing, the hatched plate edges, the collars and the damper; the flap is an extruded solid that rises in front and shortens in the top view over the plate pack.",
    "Vorder- und Draufsicht behalten Gehäuse, schraffierte Plattenkanten, Stutzen und Klappe; der Klappenstreifen steigt mit der Klappenspitze, und die Klappe ist ein extrudierter Körper, der vorn steigt und sich in der Draufsicht über dem Plattenpaket verkürzt.")

front200 = [
    R("frame", "frame", 3.3, 0.35, -0.25, 0, 0.5, 0.6, "frame", GS),
    R("plate-pack", "plates", 3.4, 0.8, -0.2, 0.04, 0.4, 0.5, "plates", AL),
    R("collar-upper", "ports", 3.5, 0.4, -0.15, 0.4, 0.3, 0.15, "ports", GS),
    R("collar-lower", "ports", 3.5, 0.4, -0.15, 0.06, 0.3, 0.15, "ports", GS),
    flap(0.04),
]
plan200 = [
    R("frame", "frame", 3.3, 0.35, -0.25, -0.25, 0.5, 0.5, "frame", GS),
    *collars_plan(0),
    R("plate-pack", "plates", 3.4, 0.8, -0.2, -0.2, 0.4, 0.4, "plates", AL),
    flap(0.04),
]
put(doc, 200, front200, plan200,
    "four masses from the front and from above: casing, plate block, collars and the damper, an extruded 40 mm plate.",
    "vier Massen von vorn und von oben: Gehäuse, Plattenblock, Stutzen und die Klappe, eine extrudierte 40-mm-Platte.")
save(A, doc)
print("ok")
