import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("scroll-compressor")
SS = dict(material="steel-shell", role="body")
CU = dict(material="copper", role="body")
CA = dict(material="cast-aluminium")
spiral = [[0.006, 0], [0.0067, 0.00201], [0.00668, 0.00441], [0.00577, 0.00691], [0.00393, 0.0092], [0.00123, 0.01093], [-0.00214, 0.01181], [-0.0059, 0.01158], [-0.00967, 0.01012], [-0.01305, 0.0074], [-0.0156, 0.00356], [-0.01696, -0.00114], [-0.01685, -0.00632], [-0.01512, -0.01151], [-0.01176, -0.01618], [-0.00694, -0.01982], [-0.00099, -0.02198], [0.00562, -0.0223], [0.01231, -0.0206], [0.01845, -0.01687], [0.02343, -0.01128], [0.02667, -0.00422], [0.02775, 0.00376], [0.0264, 0.01199], [0.02259, 0.01974], [0.01649, 0.02625], [0.00851, 0.03085], [-0.00074, 0.03299], [-0.01051, 0.03234]]
dome = []
for i in range(0, 9):
    t = i / 8
    r = (1 - t) ** 3 * 0.1 + 3 * (1 - t) ** 2 * t * 0.1 + 3 * (1 - t) * t ** 2 * 0.05
    y = 3 * (1 - t) ** 2 * t * 0.042 + 3 * (1 - t) * t ** 2 * 0.06 + t ** 3 * 0.06
    dome.append([y, r])

V.box("feet", "shell", [0, 0.015, 0], [0.28, 0.03, 0.28], cr=0.008, tone=0.4, region="feet", maxDenominator=50, **SS)
V.cyl("shell", "shell", [0, 0.03, 0], [0, 0.34, 0], 0.1, tone=0.5, region="shell", **SS)
V.rev("dome", "dome", [0, 0.34, 0], [0, 1, 0], dome, tone=0.7, region="dome", **SS)
V.cyl("discharge-stub", "dome", [0, 0.398, 0], [0, 0.43, 0], 0.014, tone=0.7, region="discharge", **CU)
V.cyl("suction-stub", "shell", [-0.15, 0.213, 0], [-0.098, 0.213, 0], 0.013, tone=0.2, region="suction", **CU)
V.box("terminal", "terminal", [0.12, 0.28, 0], [0.04, 0.06, 0.055], cr=0.004, tone=0.3, region="terminal", material="pa-plastic", role="body")
V.cyl("sight-ring", "shell", [0, 0.18, -0.1], [0, 0.18, -0.108], 0.05, tone=0.3, region="shell", maxDenominator=50, **dict(SS, role="body"))
V.cyl("scroll-disc", "rotor", [0, 0.18, -0.106], [0, 0.18, -0.102], 0.04, tone=0.8, region="scroll", maxDenominator=100, role="glazing", **CA)
V.loft("spiral", "rotor", [([p[0], 0.18 + p[1], -0.1075], 0.0016, 0.0016) for p in spiral], caps=(True, True), tone=0.1, region="scroll", maxDenominator=100, role="detail", **CA)
V.cyl("scroll-hub", "rotor", [0, 0.18, -0.109], [0, 0.18, -0.104], 0.006, tone=0.2, region="scroll", maxDenominator=50, role="detail", **CA)
V.decide("The hermetic shell is a cylinder (r 0.1 m) with a revolved dome (the elevation Bezier) and a base plate for the feet; the side window shows the scroll set: a 4 mm cast-aluminium disc on the rotor bone with the 29-point spiral as one swept tube, so the spiral turns with the rotor in every pose and clip.",
         "Das hermetische Gehaeuse ist ein Zylinder (r 0,1 m) mit gedrehter Kuppel (Bezierkurve der Ansicht) und einer Grundplatte fuer die Fuesse; das Seitenfenster zeigt den Scrollsatz: eine 4 mm dicke Aluminiumgussscheibe auf dem Knochen rotor mit der 29-Punkte-Spirale als ein gefuehrtes Rohr, die Spirale dreht sich in jeder Pose und jedem Clip mit dem Rotor.")
V.decide("The sight glass sits on the -z side (the drawn side), 8 mm proud of the shell; the terminal box is 55 mm wide (front view 50 mm, plan 60 mm); suction pipe, discharge pipe are flat-ended copper cylinders. Total width is 0.29 m against dimensions.widthM 0.30 m because the drawn parts stop at x -0.15 and 0.14.",
         "Das Schauglas sitzt auf der -z-Seite (der gezeichneten Seite), 8 mm vor dem Gehaeuse; der Klemmenkasten ist 55 mm breit (Vorderansicht 50 mm, Grundriss 60 mm); Saug- und Druckrohr sind Kupferzylinder mit ebenen Enden. Die Gesamtbreite betraegt 0,29 m gegen dimensions.widthM 0,30 m, weil die gezeichneten Teile bei x -0,15 und 0,14 enden.")
V.write(os.path.join(here, "scroll-compressor.json"))
