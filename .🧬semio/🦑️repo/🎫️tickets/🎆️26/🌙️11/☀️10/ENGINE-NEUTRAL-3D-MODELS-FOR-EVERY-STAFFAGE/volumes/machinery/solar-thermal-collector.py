import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("solar-thermal-collector")
TH = 35.0
c, s_ = math.cos(math.radians(TH)), math.sin(math.radians(TH))
P1 = (-0.79047, 0.18)
W = lambda s, t, z=0.0: [P1[0] + s * c - t * s_, P1[1] + s * s_ + t * c, z]
def slab(id, bone, s0, s1, t0, t1, z0, z1, **m):
    return V.box(id, bone, W((s0 + s1) / 2, (t0 + t1) / 2, (z0 + z1) / 2), [s1 - s0, t1 - t0, z1 - z0], rz=TH, **m)
AA = dict(material="anodised-aluminium", role="body", region="housing")
GS = dict(material="galvanised-steel", role="body", region="stand")

V.box("base-rail", "stand", [0, 0.025, 0.4], [1.5, 0.05, 0.05], tone=0.5, rep=dict(count=2, step=[0, 0, -0.8]), **GS)
V.ext("front-strut", "stand", "side", [(-0.57, 0.04), (-0.53, 0.04), (-0.53, 0.37439), (-0.57, 0.34638)], 0.375, 0.425, tone=0.5, rep=dict(count=2, step=[0, 0, -0.8]), **GS)
V.ext("rear-strut", "stand", "side", [(0.53, 0.04), (0.57, 0.04), (0.57, 1.14461), (0.53, 1.11661)], 0.375, 0.425, tone=0.5, rep=dict(count=2, step=[0, 0, -0.8]), **GS)
slab("tray-floor", "housing", 0, 2.0, 0, 0.012, -0.5, 0.5, tone=0.35, **AA)
slab("tray-end-low", "housing", 0, 0.02, 0.012, 0.1, -0.5, 0.5, tone=0.35, **AA)
slab("tray-end-high", "housing", 1.98, 2.0, 0.012, 0.1, -0.5, 0.5, tone=0.35, **AA)
slab("tray-side-left", "housing", 0.02, 1.98, 0.012, 0.1, 0.47, 0.5, tone=0.35, **AA)
slab("tray-side-right", "housing", 0.02, 1.98, 0.012, 0.1, -0.5, -0.47, tone=0.35, **AA)
slab("insulation", "housing", 0.02, 1.98, 0.012, 0.038, -0.47, 0.47, tone=0.8, maxDenominator=100, material="mineral-wool", role="body", region="housing")
slab("absorber-plate", "absorber", 0.06, 1.94, 0.05, 0.058, -0.455, 0.455, tone=0.2, material="selective-absorber", role="body", region="absorber")
a, b = W(0.06, 0.044, -0.4), W(1.94, 0.044, -0.4)
V.cyl("riser", "absorber", a, b, 0.006, tone=0.55, maxDenominator=100, material="copper", role="detail", region="absorber", rep=dict(count=9, step=[0, 0, 0.1]))
V.cyl("header-lower", "headers", W(0.06, 0.048, -0.44), W(0.06, 0.048, 0.44), 0.0165, tone=0.4, maxDenominator=100, material="copper", role="body", region="header-lower")
V.cyl("header-upper", "headers", W(1.94, 0.048, -0.44), W(1.94, 0.048, 0.44), 0.0165, tone=0.4, maxDenominator=100, material="copper", role="body", region="header-upper")
slab("glass", "glass", 0.019, 1.981, 0.09, 0.1, -0.47, 0.47, tone=0.92, material="solar-glass", role="glazing", region="glazing")
V.decide("The collector is built in its own slope frame (35 degrees, origin at the lower tray corner, s up the slope, t out of the panel): tray floor, two 20 mm end walls and two 30 mm side walls, 26 mm mineral wool, 8 mm absorber at t 0.05, nine 12 mm copper risers at 100 mm pitch under it, two 33 mm headers across the ends and a 10 mm glass pane flush with the rim. All values are read from the elevation polygons and checked against front and plan.",
         "Der Kollektor ist in seinem eigenen Neigungssystem aufgebaut (35 Grad, Ursprung an der unteren Wannenecke, s die Neigung hinauf, t aus der Flaeche heraus): Wannenboden, zwei 20-mm-Stirnwaende und zwei 30-mm-Seitenwaende, 26 mm Mineralwolle, 8 mm Absorber bei t 0,05, neun 12-mm-Kupfersteiger im Abstand 100 mm darunter, zwei 33-mm-Sammler an den Enden und eine 10-mm-Glasscheibe buendig mit dem Rand. Alle Werte stammen aus den Polygonen der Ansicht und wurden mit Vorderansicht und Grundriss geprueft.")
V.decide("Four galvanised struts (two front, two rear, 50 mm wide at z +-0.4) end exactly on the tray floor (they reach 12 mm into it) and stand on two base rails, so the frame carries the tray without a gap; the stand and housing bones are fixed, nothing moves in any pose.",
         "Vier verzinkte Stuetzen (zwei vorn, zwei hinten, 50 mm breit bei z +-0,4) enden genau am Wannenboden (12 mm ueberlappend) und stehen auf zwei Grundschienen, das Gestell traegt die Wanne luckenlos; Gestell und Gehaeuse sind feste Knochen, in keiner Pose bewegt sich etwas.")
V.write(os.path.join(here, "solar-thermal-collector.json"))
