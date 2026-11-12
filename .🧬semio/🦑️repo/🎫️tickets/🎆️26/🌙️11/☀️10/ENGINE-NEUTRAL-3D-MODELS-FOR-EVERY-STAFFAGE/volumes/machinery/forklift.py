import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("forklift")
PT = dict(material="paint", role="body")
PTD = dict(material="paint", role="detail")
ST = dict(material="steel", role="body")
STD = dict(material="steel", role="detail")
IN = dict(material="interior", role="detail")
HY = dict(material="hydraulic", role="detail")
rect = lambda a0, a1, b0, b1: [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]
mirror = lambda z: dict(rep=dict(count=2, step=[0, 0, -2 * z]))

chassis = [(-1.7, 0.5), (-1.7, 0.58), (-0.02, 0.58), (0.05, 0.96), (0.35, 0.92), (0.43, 0.86), (0.45, 0.78), (0.45, 0.28), (0.4, 0.19), (0.33, 0.18), (0.33, 0.5), (-0.33, 0.5), (-0.33, 0.18), (-1.45, 0.18), (-1.45, 0.5)]
V.ext("chassis-body", "chassis", "side", chassis, -0.5, 0.5, tone=0.55, region="chassis-paint", **PT)
V.box("drive-cover", "chassis", [-0.05, 0.63, 0], [0.5, 0.1, 0.8], cr=0.01, tone=0.3, region="drive-motor", material="drive-electronics", role="detail")
V.ext("cowl-dash", "chassis", "side", [(0.05, 0.96), (-0.02, 0.58), (-0.18, 0.6), (-0.1, 1.0)], -0.3, 0.3, tone=0.25, region="controller", material="drive-electronics", role="detail")
V.cap("steering-column", "chassis", [-0.02, 0.98, 0], [-0.2, 1.3, 0], 0.02, tone=0.2, maxDenominator=50, **IN)
c = (-0.22, 1.35, 0.0)
n = (-0.94, -0.34, 0.0)
m = (-0.34, 0.94, 0.0)
t = 2 * math.pi / 16
V.cap("steering-wheel", "chassis", [c[0], c[1], c[2] + 0.17], [c[0] + 0.17 * math.sin(t) * m[0], c[1] + 0.17 * math.sin(t) * m[1], 0.17 * math.cos(t)], 0.012, tone=0.2, maxDenominator=100,
      rep=dict(count=16, angle=t, axis=list(n), origin=list(c)), **IN)
V.box("seat-cushion", "chassis", [-0.7, 1.1, 0], [0.5, 0.1, 0.5], cr=0.03, tone=0.22, maxDenominator=100, **IN)
V.box("seat-back", "chassis", [-0.96, 1.33, 0], [0.1, 0.55, 0.44], rz=8, cr=0.03, tone=0.22, maxDenominator=100, **IN)
for nm, x0, x1 in [("front", -0.045, -0.105), ("rear", -1.0, -1.0)]:
    V.cap(f"guard-post-{nm}", "chassis", [x0, 0.95 if nm == "front" else 0.58, 0.5], [x1, 2.1, 0.5], 0.03, tone=0.25, region="guard", rep=dict(count=2, step=[0, 0, -1.0]), **PTD)
V.box("guard-rail-side", "chassis", [-0.525, 2.125, 0.525], [1.35, 0.05, 0.05], tone=0.3, region="guard", rep=dict(count=2, step=[0, 0, -1.05]), **PT)
V.box("guard-rail-end", "chassis", [0.125, 2.125, 0], [0.05, 0.05, 1.0], tone=0.3, region="guard", rep=dict(count=2, step=[-1.3, 0, 0]), **PT)
V.box("guard-slat", "chassis", [-1.1, 2.125, 0], [0.03, 0.04, 1.0], tone=0.2, region="guard", maxDenominator=50, rep=dict(count=6, step=[0.2, 0, 0]), **PTD)
V.cyl("beacon", "chassis", [-0.5, 2.15, 0], [-0.5, 2.26, 0], 0.06, tone=0.85, **IN)
V.box("battery-case", "battery", [-1.2, 0.815, 0], [1.0, 0.47, 0.9], cr=0.02, tone=0.4, region="battery-pack", material="battery-case", role="body")
cw = [(-1.7, 0.5), (-1.7, 1.2), (-2.3, 1.2), (-2.4, 1.15), (-2.45, 1.0), (-2.45, 0.35), (-2.4, 0.22), (-2.3, 0.18), (-1.95, 0.18), (-1.95, 0.5)]
V.ext("counterweight-body", "counterweight", "side", cw, -0.55, 0.55, tone=0.3, region="counterweight", material="cast-iron", role="body")
V.box("counterweight-vent", "counterweight", [-2.455, 1.0, 0], [0.01, 0.2, 0.4], tone=0.2, maxDenominator=50, material="cast-iron", role="detail")
V.box("tilt-trunnion", "chassis", [-0.55, 0.66, 0.26], [0.1, 0.16, 0.07], tone=0.3, region="hydraulics", maxDenominator=200, rep=dict(count=2, step=[0, 0, -0.52]), **HY)
V.box("tilt-bracket", "mast-outer", [-0.075, 1.22, 0.26], [0.11, 0.1, 0.07], tone=0.3, region="hydraulics", maxDenominator=200, rep=dict(count=2, step=[0, 0, -0.52]), **HY)
for z in (0.26, -0.26):
    s = "left" if z > 0 else "right"
    V.cap(f"tilt-barrel-{s}", "tilt-cyl", V.W("tilt-cyl", [0, 0, z]), V.W("tilt-cyl", [0.45, 0, z]), 0.04, tone=0.35, region="hydraulics", **HY)
    V.cap(f"tilt-rod-{s}", "tilt-rod", V.W("tilt-rod", [0, 0, z]), V.W("tilt-rod", [0.45, 0, z]), 0.025, tone=0.8, region="hydraulics", **HY)
V.box("mast-outer-rail", "mast-outer", [0.05, 1.075, 0.41], [0.14, 2.05, 0.14], tone=0.35, region="mast", rep=dict(count=2, step=[0, 0, -0.82]), **ST)
V.cyl("mast-pivot", "mast-outer", [0.05, 0.3, -0.48], [0.05, 0.3, 0.48], 0.05, tone=0.15, maxDenominator=50, **STD)
V.box("mast-inner-rail", "mast-inner", [0.17, 0.9, 0.25], [0.1, 1.6, 0.14], tone=0.45, region="mast", rep=dict(count=2, step=[0, 0, -0.5]), **ST)
V.box("carriage-plate", "carriage", [0.29, 0.37, 0], [0.1, 0.6, 0.84], tone=0.5, region="fork-gear", **ST)
V.box("fork-heel", "forks", [0.375, 0.2, 0.4], [0.09, 0.26, 0.12], tone=0.3, region="fork-gear", rep=dict(count=2, step=[0, 0, -0.8]), **ST)
V.box("fork-tine", "forks", [0.91, 0.0925, 0.4], [0.98, 0.045, 0.12], tone=0.3, region="fork-gear", rep=dict(count=2, step=[0, 0, -0.8]), **ST)
for nm, z in [("outer", 0.525), ("inner", 0.225)]:
    V.box(f"fork-backrest-bar-{nm}", "forks", [0.375, 0.595, z], [0.09, 1.05, 0.05], tone=0.3, region="fork-gear", maxDenominator=50, rep=dict(count=2, step=[0, 0, -2 * z]), **ST)
V.box("fork-backrest-top", "forks", [0.375, 1.095, 0], [0.09, 0.05, 1.1], tone=0.3, region="fork-gear", **ST)
V.box("fork-backrest-rail", "forks", [0.375, 0.59, 0], [0.09, 0.04, 1.1], tone=0.3, region="fork-gear", maxDenominator=50, **STD)
V.box("pallet-runner", "pallet", [1.45, 0.065, 0], [0.1, 0.1, 0.8], tone=0.5, region="pallet", rep=dict(count=3, step=[0.5, 0, 0]), material="wood", role="body")
V.box("pallet-deck", "pallet", [2.0, 0.137, 0], [1.2, 0.044, 0.8], tone=0.62, region="pallet", material="wood", role="body")
V.box("pallet-goods", "pallet", [2.0, 0.609, 0], [1.16, 0.9, 0.76], cr=0.02, tone=0.78, region="cargo", material="goods", role="body")
ring = lambda x0, x1, y0, y1, th: (rect(x0, x1, y0, y1), rect(x0 + th, x1 - th, y0 + th, y1 - th))
V.ext("goods-strap-around", "pallet", "front", [(0.155, -0.384), (1.063, -0.384), (1.063, 0.384), (0.155, 0.384)], 1.79, 1.81, holes=[[(0.16, -0.376), (1.058, -0.376), (1.058, 0.376), (0.16, 0.376)]], tone=0.3, maxDenominator=100, rep=dict(count=2, step=[0.4, 0, 0]), material="goods", role="detail")
V.ext("goods-strap-long", "pallet", "side", rect(1.416, 2.584, 0.155, 1.063), 0.12, 0.14, holes=[rect(1.424, 2.576, 0.16, 1.058)], tone=0.3, maxDenominator=100, rep=dict(count=2, step=[0, 0, -0.26]), material="goods", role="detail")
for bone, cx, r, hw, rim, hub, ztyre, zaxle in [("wheel-f", 0.0, 0.29, 0.1, 0.17, 0.055, 0.5, 0.6), ("wheel-r", -1.7, 0.23, 0.08, 0.13, 0.04, 0.24, 0.32)]:
    tag = bone[-1]
    cy = 0.29 if tag == "f" else 0.23
    for side, sg in [("left", 1), ("right", -1)]:
        k = r / 0.29
        prof = [[sg * a * hw / 0.1, rr] for a, rr in [(-0.1, 0), (-0.1, r * 0.8), (-0.085, r * 0.93), (-0.05, r), (0.05, r), (0.085, r * 0.93), (0.1, r * 0.8), (0.1, 0)]]
        V.rev(f"tyre-{tag}-{side}", bone, [cx, cy, sg * ztyre], [0, 0, 1], prof, tone=0.1, region="tyres", material="rubber", role="body")
        zo = sg * (ztyre + hw + 0.004)
        V.cyl(f"rim-{tag}-{side}", bone, [cx, cy, sg * ztyre], [cx, cy, zo], rim, tone=0.6, region="brakes", maxDenominator=200, material="paint", role="detail")
        V.cyl(f"rim-bolt-{tag}-{side}", bone, [cx + rim * 0.6, cy, zo], [cx + rim * 0.6, cy, zo + sg * 0.012], 0.015, tone=0.35, region="brakes", maxDenominator=100, material="cast-iron", role="detail",
              rep=dict(count=6, angle=2 * math.pi / 6, axis=[0, 0, 1], origin=[cx, cy, 0]))
        V.cyl(f"hub-{tag}-{side}", bone, [cx, cy, zo], [cx, cy, zo + sg * 0.02], hub, tone=0.45, maxDenominator=200, material="paint", role="detail")
    V.cyl(f"axle-{tag}", "axle-" + tag, [cx, cy, -zaxle], [cx, cy, zaxle], 0.04 if tag == "r" else 0.05, tone=0.3, material="steel", role="detail")
V.decide("The chassis is one side-profile extrusion (z +-0.5) with wheel arches cut through it for the front and rear wheels and the battery well behind the cowl; the counterweight gets its own arch for the rear wheel, so tyres, axles and body do not interpenetrate except where the 2D views overlap (the drawn mast rails at z 0.34 to 0.48 run through the front tyres at z 0.4 to 0.6, kept as drawn).",
         "Das Fahrgestell ist eine Seitenprofil-Extrusion (z +-0,5) mit durchgeschnittenen Radkaesten fuer Vorder- und Hinterraeder und der Batteriewanne hinter der Haube; das Gegengewicht bekommt einen eigenen Radkasten fuer das Hinterrad, so durchdringen sich Reifen, Achsen und Aufbau nicht, ausser wo sich die 2D-Ansichten ueberlappen (die gezeichneten Mastschienen bei z 0,34 bis 0,48 laufen durch die Vorderreifen bei z 0,4 bis 0,6, wie gezeichnet belassen).")
V.decide("Wheels are revolved tyres with rim, hub and six rim bolts that make rotation readable, bound to the wheel bones; the mast is two outer rails (tilting bone), two inner rails (prismatic), the carriage plate and fork heels, tines and backrest (fork bone) each on the bone that drives them; tilt cylinders are barrel and rod capsules on their own bones with trunnion and mast brackets so the hydraulic ends stay attached in every pose.",
         "Raeder sind gedrehte Reifen mit Felge, Nabe und sechs Felgenbolzen, die die Drehung lesbar machen, an den Radknochen gebunden; der Mast besteht aus zwei Aussenschienen (Kippknochen), zwei Innenschienen (Schubknochen), Hubschlitten und Gabelfersen, Zinken und Rueckenlehne (Gabelknochen), jeweils auf dem Knochen, der sie antreibt; die Neigezylinder sind Rohr- und Stangenkapseln auf eigenen Knochen mit Lager- und Mastkonsolen, damit die Hydraulikenden in jeder Pose verbunden bleiben.")
V.decide("The pallet with its load sits on the pallet bone (three runners, deck, goods block, two straps around the load and two along it). dimensions.widthM (4.07 m) is the extent in the default pose lowered-forks where the pallet slides 0.98 m back onto the forks; at rest the pallet is 5.05 m out, so judge the bounding box in the default pose.",
         "Die Palette mit Ladung sitzt auf dem Knochen pallet (drei Kufen, Deck, Ladeblock, zwei Gurte um die Ladung und zwei laengs). dimensions.widthM (4,07 m) ist die Ausdehnung in der Standardpose lowered-forks, in der die Palette 0,98 m auf die Gabeln zurueckgeschoben ist; in Ruhe ragt sie bis 5,05 m, die Bounding-Box ist deshalb in der Standardpose zu beurteilen.")
V.write(os.path.join(here, "forklift.json"))
