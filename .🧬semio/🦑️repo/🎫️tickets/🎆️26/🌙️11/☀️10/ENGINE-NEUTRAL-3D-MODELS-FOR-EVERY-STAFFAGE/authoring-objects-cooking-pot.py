import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

A = lib.Asset("cooking-pot", "stainless", 0.4, "wall")
A.rev("base", "pot", [0, 0, 0], [[0, 0], [0, 0.086], [0.008, 0.086], [0.008, 0]], region="base")
A.ring("wall", "pot", [0, 0], 0.008, 0.1, 0.086, 0.086, 0.006)
for sx, nm in ((-1, "left"), (1, "right")):
    A.box(f"handle-{nm}", "handles", [sx * 0.1035, 0.0705, 0], [0.043, 0.015, 0.02], corner=0.003, tone=0.2, region="handle")
    A.rev(f"rivet-{nm}", "handles", [sx * 0.108, 0.078, 0], [[0, 0], [0, 0.003], [0.0012, 0.003], [0.0012, 0]], tone=0.5, role="detail", region="handle", maxDenominator=50)
A.rev("water", "water", [0, 0.008, 0], [[0, 0], [0, 0.08], [0.046, 0.08], [0.046, 0]], material="water", tone=0.45, region="water")
A.rev("water-surface", "surface", [0, 0.054, 0], [[0, 0], [0, 0.0795], [0.002, 0.0795], [0.002, 0]], material="water", tone=0.9, role="detail", region="water")
for k, (x, z, r) in enumerate(((-0.03, 0.02, 0.006), (0.025, 0.03, 0.005), (0.0, -0.02, 0.007), (0.04, -0.015, 0.005), (-0.04, -0.03, 0.005))):
    A.ell(f"bubble-{'abcde'[k]}", "surface", [x, 0.0568, z], [r, 0.003, r], material="water", tone=0.9, role="detail", region="water", maxDenominator=50)
A.save([
    ("The pot is a hollow cylinder: a 6 mm wall ring on an 8 mm solid base disc, open at the top; the water fills the inner radius 0.08 m as drawn in the elevation (the plan draws the water top smaller, 0.072 m, as a visual margin) and its surface disc and bubbles sit on the surface bone, so they rise with the boiling pose.",
     "Der Topf ist ein hohler Zylinder: ein 6 mm starker Wandring auf einer massiven 8 mm Bodenscheibe, oben offen; das Wasser füllt den Innenradius 0,08 m wie in der Ansicht gezeichnet (die Draufsicht zeichnet die Wasseroberfläche mit 0,072 m kleiner, als optischen Rand); Oberflächenscheibe und Blasen sitzen am Oberflächenknochen und steigen in der Kochpose mit."),
    ("The two flat grip straps with their rivets are boxes anchored 2 mm inside the wall; the rim bead of the 1:50 line drawing is not modelled.",
     "Die zwei flachen Griffbänder mit ihren Nieten sind Quader, 2 mm in der Wand verankert; der Randwulst der Strichzeichnung 1:50 wird nicht modelliert."),
])
