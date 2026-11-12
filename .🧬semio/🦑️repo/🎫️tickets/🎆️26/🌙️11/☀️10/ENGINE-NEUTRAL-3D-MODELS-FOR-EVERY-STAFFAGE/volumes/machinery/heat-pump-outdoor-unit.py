import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("heat-pump-outdoor-unit")
PC = dict(material="powder-coat", role="body")
GS = dict(material="galvanised-steel", role="body")
GSD = dict(material="galvanised-steel", role="detail")
rect = lambda y0, y1, z0, z1: [(y0, z0), (y1, z0), (y1, z1), (y0, z1)]
FY, FZ = 0.8, 0.12
ring = lambda cx, cy, r, n=28: [(cy + r * math.sin(2 * math.pi * k / n), cx + r * math.cos(2 * math.pi * k / n)) for k in range(n)]
circ = lambda y, z, r, n=32: [(y + r * math.sin(2 * math.pi * k / n), z + r * math.cos(2 * math.pi * k / n)) for k in range(n)]

V.box("foot-left", "unit", [0, 0.045, 0.43], [0.34, 0.09, 0.18], tone=0.2, region="feet", maxDenominator=100, material="rubber", role="body")
V.box("foot-right", "unit", [0, 0.045, -0.43], [0.34, 0.09, 0.18], tone=0.2, region="feet", maxDenominator=100, material="rubber", role="body")
V.box("base-tray", "unit", [0, 0.115, 0], [0.45, 0.07, 1.1], tone=0.3, region="base-tray", **GS)
V.box("compressor-panel", "unit", [0, 0.235, 0], [0.45, 0.17, 1.1], cr=0.004, tone=0.4, region="compressor-casing", **PC)
V.box("service-panel-line", "unit", [0.2255, 0.235, 0], [0.003, 0.15, 1.04], tone=0.3, region="compressor-casing", maxDenominator=200, **PC)
V.box("rear-wall", "unit", [-0.2175, 0.865, 0], [0.015, 1.09, 1.1], tone=0.55, region="casing", **PC)
V.box("top-wall", "unit", [0, 1.4025, 0], [0.45, 0.015, 1.1], tone=0.55, region="casing", **PC)
for side, z0, z1 in [("left", 0.535, 0.55), ("right", -0.55, -0.535)]:
    V.ext(f"side-wall-{side}", "unit", "side", [(-0.225, 0.32), (0.225, 0.32), (0.225, 1.41), (-0.225, 1.41)], z0, z1,
          holes=[[(-0.13, 0.42), (0.09, 0.42), (0.09, 1.28), (-0.13, 1.28)]], tone=0.55, region="casing", **PC)
V.ext("front-panel", "unit", "front", rect(0.32, 1.41, -0.55, 0.55), 0.205, 0.225, holes=[circ(FY, FZ, 0.33)], tone=0.55, region="casing", **PC)
V.box("coil", "unit", [-0.02, 0.85, 0], [0.22, 0.86, 1.06], tone=0.25, region="coil-guard", material="aluminium", role="detail")
V.box("panel-handle", "unit", [0.12, 0.85, -0.5525], [0.04, 0.1, 0.0075], cr=0.003, tone=0.2, region="casing", maxDenominator=50, **PC)
V.cyl("fan-motor", "unit", [-0.02, FY, FZ], [0.16, FY, FZ], 0.055, tone=0.15, region="fan-guard", **GSD)
V.box("motor-stay", "unit", [-0.03, 0.865, FZ], [0.02, 1.09, 0.03], tone=0.3, region="fan-guard", maxDenominator=200, **GSD)
V.rev("fan-hub", "unit", [0.16, FY, FZ], [1, 0, 0], [[0, 0.07], [0.05, 0.07], [0.068, 0.045], [0.075, 0]], tone=0.15, region="fan-guard", maxDenominator=100, **GSD)
fan_x = 0.19
phi = math.radians(54.5)
radial = [0, math.sin(phi), math.cos(phi)]
tang = [0, -math.cos(phi), math.sin(phi)]
pitch = math.radians(25)
chord = [math.sin(pitch), math.cos(pitch) * tang[1], math.cos(pitch) * tang[2]]
secs = []
for r, rx in [(0.065, 0.02), (0.11, 0.036), (0.17, 0.043), (0.22, 0.036), (0.262, 0.018)]:
    secs.append(([fan_x, FY + r * radial[1], FZ + r * radial[2]], rx, 0.004, 2.4))
V.loft("fan-blade", "unit", secs, hint=chord, tone=0.5, region="fan-guard", maxDenominator=100, material="aluminium", role="detail",
       rep=dict(count=5, angle=-2 * math.pi / 5, axis=[1, 0, 0], origin=[fan_x, FY, FZ]))
def torus(id, r, x, wire, n=24, **kw):
    t = 2 * math.pi / n
    V.cap(id, "unit", [x, FY, FZ + r], [x, FY - r * math.sin(t), FZ + r * math.cos(t)], wire, rep=dict(count=n, angle=t, axis=[1, 0, 0], origin=[x, FY, FZ]), **kw)
torus("guard-rim", 0.345, 0.222, 0.008, n=36, tone=0.3, region="fan-guard", maxDenominator=200, **GS)
torus("fan-ring-outer", 0.27, 0.26, 0.004, tone=0.3, region="fan-guard", maxDenominator=50, **GSD)
torus("fan-ring-inner", 0.19, 0.26, 0.004, tone=0.3, region="fan-guard", maxDenominator=50, **GSD)
V.cyl("guard-cap", "unit", [0.25, FY, FZ], [0.262, FY, FZ], 0.07, tone=0.3, region="fan-guard", maxDenominator=100, **GSD)
V.cap("guard-spoke", "unit", [0.26, FY, FZ + 0.07], [0.26, FY, FZ + 0.3], 0.003, tone=0.3, region="fan-guard", maxDenominator=200,
      rep=dict(count=8, angle=2 * math.pi / 8, axis=[1, 0, 0], origin=[0.26, FY, FZ]), **GSD)
V.cap("guard-skirt", "unit", [0.26, FY, FZ + 0.3], [0.222, FY, FZ + 0.345], 0.003, tone=0.3, region="fan-guard", maxDenominator=200,
      rep=dict(count=12, angle=2 * math.pi / 12, axis=[1, 0, 0], origin=[0.26, FY, FZ]), **GSD)
V.box("lid", "lid", [0, 1.43, 0], [0.47, 0.04, 1.12], cr=0.004, tone=0.35, region="top-lid", **PC)
V.cyl("lid-hinge-front", "lid", [-0.215, 1.43, 0.25], [-0.215, 1.43, 0.5], 0.02, tone=0.3, region="top-lid", maxDenominator=100, material="powder-coat", role="detail")
V.cyl("lid-hinge-rear", "lid", [-0.215, 1.43, -0.5], [-0.215, 1.43, -0.25], 0.02, tone=0.3, region="top-lid", maxDenominator=100, material="powder-coat", role="detail")
V.box("vane", "vane", [0.36, 0.8, 0.12], [0.24, 0.05, 0.72], cr=0.02, tone=0.3, region="discharge-vane", **PC)
V.cyl("vane-hinge", "vane", [0.26, 0.8, -0.24], [0.26, 0.8, 0.48], 0.012, tone=0.3, region="discharge-vane", maxDenominator=100, material="powder-coat", role="detail")
V.decide("The casing is a thin-walled shell (rear wall, top wall, two side walls with a coil window, a front panel with a round fan opening) on a solid base tray and compressor compartment; the coil block behind the windows and the fan behind the opening are modelled, so the unit can be looked into.",
         "Das Gehaeuse ist eine duennwandige Huelle (Rueckwand, Deckwand, zwei Seitenwaende mit Registerfenster, Frontblech mit runder Ventilatoroeffnung) auf einem vollen Bodenblech und Verdichterfach; der Register-Block hinter den Fenstern und der Ventilator hinter der Oeffnung sind modelliert, man kann in das Geraet hineinsehen.")
V.decide("The fan has five identical pitched blades as one loft with a rotational repeat about the fan axis (centre z 0.12, y 0.8, as in the front view), a spinner hub and a motor on a stay that spans the casing; the guard is wire: rim ring, two rings, eight spokes and twelve skirt wires on a 4.5 cm dish, again written with rotational repeats.",
         "Der Ventilator hat fuenf gleiche angestellte Fluegel als ein Loft mit Rotationswiederholung um die Ventilatorachse (Mitte z 0,12, y 0,8 wie in der Vorderansicht), eine Nabe mit Haube und einen Motor an einer gehaeuseueberspannenden Strebe; das Schutzgitter ist Draht: Randring, zwei Ringe, acht Speichen und zwoelf Mantelstaebe auf einer 4,5 cm tiefen Schale, ebenfalls mit Rotationswiederholung.")
V.decide("The lid (on the lid bone) carries its two hinge knuckles on the same bone, the discharge vane carries its hinge rod on the vane bone, so lid and vane swing about the rig's hinge axes without leaving the casing. The 2D lid and vane drawings are 56 mm and 24 mm wider than the casing, taken as drawn.",
         "Der Deckel (Knochen lid) traegt seine zwei Scharnierknoten auf demselben Knochen, die Ausblaslamelle ihre Scharnierstange auf dem Knochen vane, so schwenken Deckel und Lamelle um die Scharnierachsen des Rigs, ohne das Gehaeuse zu verlassen. Deckel und Lamelle sind in den 2D-Zeichnungen breiter als das Gehaeuse (56 mm und 24 mm), wie gezeichnet uebernommen.")
V.write(os.path.join(here, "heat-pump-outdoor-unit.json"))
