import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

# World = rig rest pose, root on the ground, forks pointing to +x. Front x = lateral (left +x); plan x = forward, plan y = lateral (left +y).
# Everything that tilts, lifts or slides on the mast (mast stages, tilt cylinders, carriage, forks, pallet and load) is an extruded
# solid of its bone-local side-view profile, so every pose and clip instant reads in the front (height range) and in the plan
# (forward range) with the full bone transform. Chassis, guard and tyres are fixed and drawn explicitly.


def pair(make, lo, hi, name):
    return [make(name + "-left", lo, hi), make(name + "-right", -hi, -lo)]


def mech(d):
    s = []
    GT = 0.78
    # pallet and load
    s += [X(rect("pallet-goods", "pallet", "body", 6.0, GT if d <= 100 else 0.75, 0.02, 0.144 if d <= 100 else 0.16, 1.16, 0.9 if d <= 100 else 0.884, rr=0.02 if d < 500 else None, region="cargo", material="goods"), -0.38, 0.38)]
    s += [X(rect("pallet-deck", "pallet", "body", 5.9, 0.62 if d <= 100 else 0.6, 0, 0.1, 1.2, 0.044 if d <= 100 else 0.06, region="pallet", material="wood"), -0.4, 0.4)]
    if d <= 200:
        s += [X(rect("pallet-feet", "pallet", "body", 2.8 if d <= 100 else 3.0, 0.5 if d <= 100 else 0.6, 0, 0, 1.2, 0.1, region="pallet", material="wood"), -0.4, 0.4)] if False else []
    # forks
    s += pair(lambda i, lo, hi: X(rect(i, "forks", "body", 3.3, 0.3 if d <= 100 else 0.5, 0, -1.07, 0.045, 0.98, region="fork-gear", material="steel"), lo, hi), 0.34, 0.46, "fork-tine")
    if d == 50:
        s += pair(lambda i, lo, hi: X(rect(i, "forks", "body", 3.3, 0.3, 0, -0.09, 1.05, 0.09, region="fork-gear", material="steel"), lo, hi), 0.5, 0.55, "fork-backrest-bar-outer")
        s += pair(lambda i, lo, hi: X(rect(i, "forks", "body", 3.3, 0.3, 0, -0.09, 1.05, 0.09, region="fork-gear", material="steel"), lo, hi), 0.2, 0.25, "fork-backrest-bar-inner")
        s += [X(rect("fork-backrest-top", "forks", "body", 3.3, 0.3, 1.0, -0.09, 0.05, 0.09, region="fork-gear", material="steel"), -0.55, 0.55),
              X(rect("fork-backrest-rail", "forks", "detail", 3.3, 0.3, 0.5, -0.09, 0.04, 0.09, region="fork-gear", material="steel"), -0.55, 0.55)]
    elif d == 100:
        s += pair(lambda i, lo, hi: X(rect(i, "forks", "body", 3.3, 0.3, 0, -0.09, 1.05, 0.09, region="fork-gear", material="steel"), lo, hi), 0.5, 0.55, "fork-backrest-bar")
        s += [X(rect("fork-backrest-top", "forks", "body", 3.3, 0.3, 1.0, -0.09, 0.05, 0.09, region="fork-gear", material="steel"), -0.55, 0.55)]
    elif d == 200:
        s += pair(lambda i, lo, hi: X(rect(i, "forks", "body", 3.3, 0.5, 0, -0.09, 1.05, 0.09, region="fork-gear", material="steel"), lo, hi), 0.49, 0.55, "fork-backrest-bar")
        s += [X(rect("fork-backrest-top", "forks", "body", 3.3, 0.5, 0.99, -0.09, 0.06, 0.09, region="fork-gear", material="steel"), -0.55, 0.55)]
    else:
        s += [X(rect("fork-backrest-top", "forks", "body", 3.3, 0.5, 0.98, -0.09, 0.07, 0.09, region="fork-gear", material="steel"), -0.55, 0.55)]
    s += [X(rect("carriage-plate", "carriage", "body", 3.2, 0.5, 0, -0.05, 0.6, 0.1, region="fork-gear", material="steel"), -0.42, 0.42)]
    # mast stages and tilt cylinders
    s += pair(lambda i, lo, hi: X(rect(i, "mast-inner", "body", 3.1, 0.45, 0, -0.05, 1.6, 0.1, region="mast", material="steel"), lo, hi), 0.18, 0.32, "mast-inner")
    s += pair(lambda i, lo, hi: X(rect(i, "mast-outer", "body", 3.0, 0.35 if d <= 100 else 0.5, -0.25, -0.07, 2.05, 0.14, region="mast", material="steel"), lo, hi), 0.34, 0.48, "mast-outer")
    if d <= 200:
        s += pair(lambda i, lo, hi: X(capsule(i, "tilt-cyl", "detail", 3.4, 0.35 if d <= 100 else 0.4, 0, 0, 0.45, 0, 0.04, 0.04, region="hydraulics", material="hydraulic"), lo, hi), 0.22, 0.30, "tilt-barrel")
    if d <= 100:
        s += pair(lambda i, lo, hi: X(capsule(i, "tilt-rod", "detail", 3.45, 0.8, 0, 0, 0.45, 0, 0.025, 0.025, region="hydraulics", material="hydraulic"), lo, hi), 0.235, 0.285, "tilt-rod")
    return s


def front(d):
    s = mech(d)
    if d == 50:
        s += [poly("goods-strap-%d" % i, "pallet", "detail", 6.1, 0.3, [(x, 0.159), (x, 1.059)], closed=False, material="goods") for i, x in enumerate((-0.13, 0.13), 1)]
    s += [ellipse("beacon", "chassis", "detail" if d < 500 else "body", 2.7 if d <= 100 else 3.0, 0.85, 0, 2.2, 0.06, 0.06, material="interior")]
    if d == 500:
        s += [poly("guard-frame", "chassis", "body", 3.0, 0.3, [(-0.5, 0.96), (-0.5, 2.125), (0.5, 2.125), (0.5, 0.96)], closed=False, region="guard", material="paint", penMm=0.15)]
    else:
        s += [rect("guard-roof", "chassis", "body", 2.6 if d <= 100 else 3.0, 0.3 if d <= 100 else 0.4, -0.55, 2.1, 1.1, 0.05 if d <= 100 else 0.06, region="guard", material="paint")]
        s += pair(lambda i, lo, hi: rect(i, "chassis", "detail", 2.6 if d <= 100 else 3.0, 0.25 if d <= 100 else 0.4, lo, 0.96, hi - lo, 1.14, region="guard", material="paint"), 0.47, 0.53, "guard-post")
    if d == 50:
        s += [rect("seat-back", "chassis", "detail", 2.5, 0.22, -0.22, 1.1, 0.44, 0.5, rr=0.06, material="interior"),
              rect("seat-cushion", "chassis", "detail", 2.45, 0.22, -0.25, 1.05, 0.5, 0.1, rr=0.03, material="interior"),
              ellipse("steering-wheel", "chassis", "detail", 2.4, 0.2, 0, 1.35, 0.17, 0.17, material="interior", filled=False),
              rect("steering-column", "chassis", "detail", 2.4, 0.2, -0.02, 0.98, 0.04, 0.2, material="interior")]
    elif d == 100:
        s += [rect("seat-back", "chassis", "detail", 2.5, 0.22, -0.22, 1.1, 0.44, 0.5, rr=0.06, material="interior"),
              ellipse("steering-wheel", "chassis", "detail", 2.4, 0.2, 0, 1.35, 0.17, 0.17, material="interior", filled=False)]
    if d <= 200:
        s += [rect("cowl-dash", "chassis", "detail", 2.3 if d <= 100 else 2.1, 0.25 if d <= 100 else 0.3, -0.3, 0.6, 0.6, 0.4, region="controller", material="drive-electronics"),
              rect("drive-cover", "chassis", "detail", 2.2 if d <= 100 else 2.1, 0.3, -0.4, 0.58, 0.8, 0.1, region="drive-motor", material="drive-electronics"),
              rect("battery-case", "battery", "body", 1.95, 0.4, -0.45, 0.58, 0.9, 0.47, region="battery-pack", material="battery-case")]
    if d < 500:
        s += [rect("chassis-body", "chassis", "body", 2.0, 0.55 if d <= 50 else 0.6, -0.5, 0.18, 1.0, 0.78, region="chassis-paint", material="paint")]
    else:
        s += [rect("battery-case", "battery", "body", 1.95, 0.4, -0.45, 0.58, 0.9, 0.47, region="battery-pack", material="battery-case"),
              rect("chassis-body", "chassis", "body", 2.0, 0.6, -0.5, 0.18, 1.0, 0.78, regions=["chassis-paint", "drive-motor", "controller"], material="paint")]
    s += [rect("counterweight-body", "counterweight", "body", 1.9, 0.3 if d <= 50 else 0.4, -0.55, 0.18, 1.1, 1.02, rr=0.05, region="counterweight", material="cast-iron")]
    reg = ["tyres", "brakes"] if d == 500 else ["tyres"]
    s += [rect("tyre-f-left", "wheel-f", "body", 4.0, 0.1, 0.4, 0, 0.2, 0.58, regions=reg, material="rubber"),
          rect("tyre-f-right", "wheel-f", "body", 4.0, 0.1, -0.6, 0, 0.2, 0.58, regions=reg, material="rubber")]
    if d <= 100:
        s += [rect("rim-f-left", "wheel-f", "detail", 4.3, 0.6, 0.44, 0.12, 0.08, 0.34, region="brakes", material="paint"),
              rect("rim-f-right", "wheel-f", "detail", 4.3, 0.6, -0.52, 0.12, 0.08, 0.34, region="brakes", material="paint"),
              rect("tyre-r-left", "wheel-r", "body", 1.8, 0.1, 0.16, 0, 0.16, 0.46, region="tyres", material="rubber"),
              rect("tyre-r-right", "wheel-r", "body", 1.8, 0.1, -0.32, 0, 0.16, 0.46, region="tyres", material="rubber")]
    return s


def plan(d):
    s = mech(d)
    if d == 50:
        s += [poly("goods-strap-%d" % i, "pallet", "detail", 6.1, 0.3, [(x, -0.38), (x, 0.38)], closed=False, material="goods") for i, x in enumerate((1.8, 2.2), 1)]
    s += [ellipse("beacon", "chassis", "detail" if d < 500 else "body", 3.5, 0.85 if d <= 100 else 0.8, -0.5, 0, 0.06, 0.06, material="interior")]
    if d == 50:
        s += [poly("guard-slat-%d" % i, "chassis", "texture", 3.1, 0.2, [(x, -0.5), (x, 0.5)], closed=False, material="paint") for i, x in enumerate((-1.1, -0.9, -0.7, -0.5, -0.3, -0.1), 1)]
    if d == 500:
        s += [poly("guard-frame", "chassis", "body", 3.0, 0.3, [(-1.2, -0.52), (0.15, -0.52), (0.15, 0.52), (-1.2, 0.52)], closed=True, region="guard", material="paint", penMm=0.15, filled=False)]
    else:
        s += [rect("guard-roof", "chassis", "body", 3.0, 0.3 if d <= 100 else 0.4, -1.2, -0.55, 1.35, 1.1, region="guard", material="paint")]
    if d <= 100:
        s += [rect("cowl-dash", "chassis", "detail", 2.3, 0.25, -0.18, -0.3, 0.23, 0.6, region="controller", material="drive-electronics"),
              rect("drive-cover", "chassis", "detail", 2.2, 0.3, -0.3, -0.4, 0.5, 0.8, region="drive-motor", material="drive-electronics")]
    if d < 500:
        s += [rect("chassis-body", "chassis", "body", 2.0, 0.55 if d == 50 else 0.6, -1.7, -0.5, 2.15, 1.0, rr=0.1 if d <= 100 else None, region="chassis-paint", material="paint")]
    else:
        s += [rect("chassis-body", "chassis", "body", 2.0, 0.6, -1.7, -0.5, 2.15, 1.0, regions=["chassis-paint", "drive-motor", "controller", "battery-pack"], material="paint")]
    s += [rect("counterweight-body", "counterweight", "body", 1.9, 0.3 if d <= 50 else 0.4, -2.45, -0.55, 0.75, 1.1, rr=0.08, region="counterweight", material="cast-iron")]
    reg = ["tyres", "brakes"] if d >= 500 else ["tyres"]
    s += [rect("tyre-f-left", "wheel-f", "body", 1.5, 0.1, -0.29, 0.4, 0.58, 0.2, regions=reg, material="rubber"),
          rect("tyre-f-right", "wheel-f", "body", 1.5, 0.1, -0.29, -0.6, 0.58, 0.2, regions=reg, material="rubber")]
    return s


def clean(sh):
    for x in sh:
        for k in ("r", "regions", "region"):
            if k in x and x[k] is None:
                del x[k]
    return sh


D = {
    "50": ("Front view: full detail from the fork side: the load block with straps on the pallet, tines, the barred backrest, carriage plate, both mast stages and the tilt barrels and rods as extruded solids of their side-view outlines (mast tilt, lift and fork travel change their height range exactly), guard posts and roof, seat, steering wheel and column, beacon, cowl, battery, counterweight and the front tyres with rims. Plan: the same solids seen from above (the tilting mast stretches backwards), pallet with goods and straps, roof with slats, beacon, cowl, chassis, counterweight and front tyres.",
           "Vorderansicht: voller Detailgrad von der Gabelseite: der Lastblock mit Gurten auf der Palette, Zinken, das Gitter des Lastschutzes, Gabelträger, beide Mastteile sowie Neigezylinder und Kolbenstangen als extrudierte Körper ihrer Seitenansichtsumrisse (Mastneigung, Hub und Gabelweg ändern ihren Höhenbereich genau), Dachstützen und Dach, Sitz, Lenkrad und Säule, Rundumleuchte, Haube, Batterie, Gegengewicht und die Vorderreifen mit Felgen. Grundriss: dieselben Körper von oben (der kippende Mast streckt sich nach hinten), Palette mit Ladung und Gurten, Dach mit Lamellen, Rundumleuchte, Haube, Fahrgestell, Gegengewicht und Vorderreifen."),
    "100": ("Front view: load block, pallet, tines, backrest frame, carriage, both mast stages, tilt barrels and rods as extruded side-view solids, guard posts and roof, seat back, steering wheel, beacon, cowl, battery, counterweight and tyres with rims. Plan: the same solids from above, roof, beacon, cowl, chassis, counterweight and front tyres.",
            "Vorderansicht: Lastblock, Palette, Zinken, Lastschutzrahmen, Gabelträger, beide Mastteile, Neigezylinder und Kolbenstangen als extrudierte Seitenansichtskörper, Dachstützen und Dach, Sitzlehne, Lenkrad, Rundumleuchte, Haube, Batterie, Gegengewicht und Reifen mit Felgen. Grundriss: dieselben Körper von oben, Dach, Rundumleuchte, Haube, Fahrgestell, Gegengewicht und Vorderreifen."),
    "200": ("Front view: load, pallet, two tines, backrest bars, carriage, mast stages and tilt barrels as extruded side-view solids, guard, beacon, body, battery, counterweight and two tyres. Plan: the same solids from above, roof, body, counterweight and the two front tyres.",
            "Vorderansicht: Ladung, Palette, zwei Zinken, Lastschutzstäbe, Gabelträger, Mastteile und Neigezylinder als extrudierte Seitenansichtskörper, Schutzdach, Rundumleuchte, Aufbau, Batterie, Gegengewicht und zwei Reifen. Grundriss: dieselben Körper von oben, Dach, Aufbau, Gegengewicht und die zwei Vorderreifen."),
    "500": ("Front view: one body mass with battery and counterweight blocks, the guard as a line, the two mast stages as bars, the fork as tines under a backrest bar, the pallet block with goods and two black tyre bars; mast and fork are extruded solids and still tilt and lift. Plan: one chassis block, the counterweight block, the guard outline, the mast bars, fork, pallet with goods and the two front tyres.",
            "Vorderansicht: eine Aufbaumasse mit Batterie- und Gegengewichtblock, das Schutzdach als Linie, die zwei Mastteile als Balken, die Gabel als Zinken unter einem Lastschutzbalken, der Palettenblock mit Ladung und zwei schwarze Reifenbalken; Mast und Gabel sind extrudierte Körper und kippen und heben weiter. Grundriss: ein Fahrgestellblock, der Gegengewichtblock, der Dachumriss, die Mastbalken, Gabel, Palette mit Ladung und die zwei Vorderreifen."),
}
OM = {
    "50": ("The rear tyres and the seat are hidden below the body and roof in the plan; the rear tyre pair is hidden behind the front tyres in the front view.",
           "Die Hinterreifen und der Sitz sind im Grundriss unter Aufbau und Dach verdeckt; das hintere Reifenpaar ist in der Vorderansicht hinter den Vorderreifen verdeckt."),
    "100": ("Rear tyres below the body; the backrest has no middle bars.", "Hinterreifen unter dem Aufbau; der Lastschutz hat keine Mittelstäbe."),
    "200": ("Seat, steering wheel, tilt rods, tyre rims, straps and the rear tyres.", "Sitz, Lenkrad, Kolbenstangen, Reifenfelgen, Gurte und die Hinterreifen."),
    "500": ("In both views seat, steering wheel, tilt cylinders, rims and rear tyres.", "In beiden Ansichten Sitz, Lenkrad, Neigezylinder, Felgen und Hinterreifen."),
}
plan_ = {str(d): {"front": clean(front(d)), "plan": clean(plan(d)), "dec": {"retained": D[str(d)], "omitted": OM[str(d)]}} for d in (50, 100, 200, 500)}
apply("forklift", plan_)
