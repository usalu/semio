import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("expansion-valve")
BR = dict(material="brass", role="body")
CU = dict(material="copper", role="body")

V.box("body", "body", [0, 0.025, 0], [0.064, 0.05, 0.06], cr=0.006, tone=0.5, region="body", **BR)
V.cyl("inlet", "body", [-0.078, 0.029, 0], [-0.03, 0.029, 0], 0.009, tone=0.4, region="inlet", **CU)
V.cyl("outlet", "body", [0.03, 0.019, 0], [0.078, 0.019, 0], 0.009, tone=0.4, region="outlet", **CU)
dome = []
for i in range(0, 9):
    t = i / 8
    x = (1 - t) ** 3 * 0.025 + 3 * (1 - t) ** 2 * t * 0.025 + 3 * (1 - t) * t ** 2 * 0.014
    y = (1 - t) ** 3 * 0 + 3 * (1 - t) ** 2 * t * 0.018 + 3 * (1 - t) * t ** 2 * 0.028 + t ** 3 * 0.028
    dome.append([y, x])
V.rev("head", "head", [0, 0.05, 0], [0, 1, 0], dome, tone=0.55, region="head", **BR)
V.cyl("stem", "stem", [0, 0.078, 0], [0, 0.09, 0], 0.01, tone=0.3, region="head", **BR)
V.cap("bulb", "bulb", [0.108, 0.025, 0.018], [0.108, 0.04, 0.018], 0.01, tone=0.5, region="bulb", **CU)
path = [[0.018, 0.072, 0.0], [0.05, 0.088, 0.012], [0.09, 0.082, 0.02], [0.108, 0.05, 0.018]]
for i in range(3):
    V.cap(f"capillary-{i + 1}", "head", path[i], path[i + 1], 0.0012, tone=0.1, region="capillary", maxDenominator=50, material="copper", role="detail")
V.decide("The capillary is three round bars through the 3D points read from all three drawings (elevation gives x and y, plan z, front confirms the rise to 88 mm); it sits on the head bone, which is fixed, so it stays attached at every stem position.",
         "Die Kapillare besteht aus drei Rundstaeben durch die 3D-Punkte aus allen drei Zeichnungen (Ansicht liefert x und y, Grundriss z, Vorderansicht bestaetigt den Anstieg auf 88 mm); sie sitzt auf dem festen Kopfknochen und bleibt in jeder Stellung der Spindel verbunden.")
V.decide("The diaphragm head is a revolved dome about the vertical axis (elevation outline), the spindle a cylinder on its own prismatic bone that sinks into the dome when it moves down; pipe stubs are flat-ended cylinders, the sensing bulb a vertical capsule.",
         "Der Membrankopf ist eine um die senkrechte Achse gedrehte Kuppel (Umriss der Ansicht), die Spindel ein Zylinder auf eigenem Schubknochen, der beim Absenken in die Kuppel taucht; Rohrstutzen sind Zylinder mit ebenen Enden, der Fuehler eine senkrechte Kapsel.")
V.decide("The drawn parts reach x -0.078 to 0.118 m (0.196 m) while dimensions.widthM is 0.19 m, a 3 % overshoot taken from the drawings.",
         "Die gezeichneten Teile reichen von x -0,078 bis 0,118 m (0,196 m), dimensions.widthM ist 0,19 m, 3 % Ueberstand aus den Zeichnungen.")
V.write(os.path.join(here, "expansion-valve.json"))
