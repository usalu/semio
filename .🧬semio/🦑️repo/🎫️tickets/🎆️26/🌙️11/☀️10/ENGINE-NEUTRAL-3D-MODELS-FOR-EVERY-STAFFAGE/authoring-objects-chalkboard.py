import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

def rect(x0, y0, x1, y1):
    return [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]

AL = "aluminium-frame"
A = lib.Asset("chalkboard", AL, 0.75, "frame")
ZF0, ZF1 = -0.005, 0.04
A.ext("frame-ring", "frame", rect(-2, 0.06, 2, 1.26), ZF0, ZF1, holes=[rect(-1.965, 0.095, -0.015, 1.225), rect(0.015, 0.095, 1.965, 1.225)])
for side, x0, x1 in (("left", -1.967, -0.013), ("right", 0.013, 1.967)):
    A.box(f"face-{side}", "frame", [(x0 + x1) / 2, 0.66, 0.015], [x1 - x0, 1.134, 0.03], material="enamel-board", tone=0.3, region="board-face")
A.box("top-rail", "frame", [0, 1.224, -0.0025], [3.976, 0.048, 0.005], tone=0.5, maxDenominator=100)

A.ext("leaf-frame", "leaf", rect(-2, 0.06, 0, 1.26), -0.04, -0.005, holes=[rect(-1.965, 0.095, -0.035, 1.225)], tone=0.85, region="leaf")
A.box("leaf-face", "leaf", [-1, 0.66, -0.022], [1.934, 1.134, 0.02], material="enamel-board", tone=0.24, region="leaf")
A.box("leaf-top-rail", "leaf", [-1, 1.2265, -0.0225], [1.976, 0.043, 0.027], tone=0.55, region="leaf", maxDenominator=100)
for k, x in (("a", -1.6), ("b", -0.4)):
    A.box(f"leaf-grip-{k}", "leaf", [x + 0.06, 0.118, -0.04125], [0.12, 0.02, 0.0025], corner=0.001, tone=0.4, role="detail", region="leaf", maxDenominator=50)
for k, x in (("a", -1.7), ("b", -0.3)):
    A.rev(f"hanger-{k}", "leaf", [x, 1.226, -0.0415], [[0, 0], [0, 0.022], [0.0035, 0.022], [0.0035, 0]], axis="z", tone=0.2, region="leaf", maxDenominator=50)

A.ext_yz("tray", "tray", [[0, -0.04], [0, 0.04], [0.06, 0.04], [0.06, 0.032], [0.022, 0.032], [0.022, -0.032], [0.06, -0.032], [0.06, -0.04]], -2, 2, tone=0.65, region="tray")
for k, x0, x1, z, t in (("a", -1.3, -1.21, -0.012, 0.9), ("b", -1.15, -1.08, 0.006, 0.85), ("c", -1.05, -0.97, -0.003, 0.8)):
    A.cap(f"chalk-{k}", "tray", [x0 + 0.007, 0.029, z], [x1 - 0.007, 0.029, z], 0.007, material="chalk", tone=t, role="detail", region="tray", maxDenominator=50)
A.box("sponge", "tray", [0.97, 0.04, 0], [0.14, 0.036, 0.05], corner=0.004, material="sponge", tone=0.6, role="detail", region="tray", maxDenominator=50)
A.box("ruler", "tray", [-0.25, 0.028, 0.004], [0.9, 0.012, 0.04], material="chalk", tone=0.55, role="detail", region="tray", maxDenominator=50)
A.save([
    ("The frame is one aluminium ring with two window openings, 45 mm deep and set to the back (z -0.005 to 0.04); the sliding leaf hangs in front of it (z -0.04 to -0.005), so the leaf passes the centre post in every pose. The authored front and plan views draw the frame the full 80 mm deep, which would make the leaf collide with the frame.",
     "Der Rahmen ist ein einziger Aluminiumring mit zwei Fensteröffnungen, 45 mm tief und nach hinten gesetzt (z -0,005 bis 0,04); der Schiebeflügel hängt davor (z -0,04 bis -0,005) und passiert so in jeder Pose den Mittelpfosten. Die gezeichnete Vorder- und Draufsicht zeigt den Rahmen in voller Tiefe von 80 mm, wodurch der Flügel mit dem Rahmen kollidieren würde."),
    ("The writing surfaces are 30 mm thick enamel panels recessed into the frame; the MDF core of the layer build-up is not modelled separately.",
     "Die Schreibflächen sind 30 mm dicke Emailtafeln, in den Rahmen eingelassen; der MDF-Kern des Schichtaufbaus wird nicht gesondert modelliert."),
    ("The tray is a trough (extrusion of a U profile along the board) with chalk sticks as capsules, a sponge and a ruler lying in it; they are shown to 1:50 only. The hanger wheels and the grips stand 1.5 to 3 mm proud of the leaf front.",
     "Die Ablage ist eine Rinne (Extrusion eines U-Profils entlang der Tafel) mit Kreidestücken als Kapseln, einem Schwamm und einem Lineal; sie erscheinen nur bis 1:50. Die Laufrollen und Griffe stehen 1,5 bis 3 mm vor der Flügelfront."),
])
