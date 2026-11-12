import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("diesel-generator")
CS = dict(material="container-steel", role="body")
CSD = dict(material="container-steel", role="detail")
ES = dict(material="exhaust-steel", role="body")
ESD = dict(material="exhaust-steel", role="detail")
GS = dict(material="galvanised-steel", role="body")
AL = dict(material="aluminium", role="detail")
rect = lambda a0, a1, b0, b1: [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]

V.box("skid-tank", "container", [0, 0.1, 0], [6.06, 0.2, 1.94], tone=0.3, region="skid-tank", **CS)
V.box("corner-casting-a", "container", [-2.905, 0.1, -1.095], [0.25, 0.2, 0.25], tone=0.2, region="skid-tank", maxDenominator=100, rep=dict(count=2, step=[0, 0, 2.19]), **CSD)
V.box("corner-casting-b", "container", [2.905, 0.1, -1.095], [0.25, 0.2, 0.25], tone=0.2, region="skid-tank", maxDenominator=100, rep=dict(count=2, step=[0, 0, 2.19]), **CSD)
V.box("fuel-fill", "container", [-2.7, 0.1, -1.0], [0.4, 0.1, 0.04], tone=0.2, region="skid-tank", maxDenominator=50, **CSD)
V.box("wall", "container", [0, 1.395, 0], [6.06, 2.29, 2.44], tone=0.55, region="enclosure-walls", **CS)
V.box("roof-band", "container", [0, 2.54, 0], [6.06, 0.1, 2.44], tone=0.4, region="roof", **CS)
V.box("roof-edge", "container", [0, 2.5975, 0], [5.86, 0.015, 2.24], tone=0.4, region="roof", maxDenominator=100, **CSD)
V.ext("lifting-eye", "container", "side", rect(2.5, 2.8, 2.59, 2.66), 0.985, 1.015, holes=[[(2.65 + 0.018 * math.cos(2 * math.pi * k / 12), 2.625 + 0.018 * math.sin(2 * math.pi * k / 12)) for k in range(12)]],
      tone=0.2, region="roof", maxDenominator=50, rep=dict(count=2, step=[0, 0, -2.0]), **CSD)
tilt = math.radians(30)
ax = ((1, 0, 0), (0, math.cos(tilt), math.sin(tilt)), (0, -math.sin(tilt), math.cos(tilt)))
for k, (x, name) in enumerate([(-2.15, "inlet-louvre-1"), (0.45, "inlet-louvre-2")]):
    for side, z in [("front", -1.2), ("rear", 1.2)]:
        a = ax if z < 0 else ((1, 0, 0), (0, math.cos(tilt), -math.sin(tilt)), (0, math.sin(tilt), math.cos(tilt)))
        V.box(f"{name}-{side}-slat", "container", [x, 0.62, z], [1.3, 0.025, 0.035], axes=a, tone=0.25, region="inlet-louvres", maxDenominator=200,
              rep=dict(count=15, step=[0, 0.1, 0]), **AL)
V.box("door-line", "container", [1.875, 1.3, -1.2275], [0.75, 1.7, 0.015], tone=0.5, region="enclosure-walls", maxDenominator=200, **CSD)
V.box("door-handle", "container", [1.58, 1.35, -1.23], [0.06, 0.3, 0.02], tone=0.2, region="enclosure-walls", maxDenominator=50, **CSD)
V.box("end-door", "container", [3.0375, 1.3, 0], [0.015, 1.7, 1.0], tone=0.5, region="enclosure-walls", maxDenominator=200, **CSD)
V.cap("silencer", "silencer", [-1.725, 2.865, -0.9], [-0.075, 2.865, -0.9], 0.275, tone=0.35, region="silencer", **ES)
V.cyl("silencer-flange", "silencer", [0.0, 2.865, -0.9], [0.08, 2.865, -0.9], 0.285, tone=0.2, region="silencer", maxDenominator=50, **ESD)
V.cyl("silencer-clamp-1", "silencer", [-1.6, 2.865, -0.9], [-1.48, 2.865, -0.9], 0.285, tone=0.2, region="silencer", maxDenominator=100, **ESD)
V.cyl("silencer-clamp-2", "silencer", [-0.5, 2.865, -0.9], [-0.38, 2.865, -0.9], 0.285, tone=0.2, region="silencer", maxDenominator=100, **ESD)
V.cyl("exhaust-stack", "stack", [-1.2, 3.1, -0.9], [-1.2, 3.47, -0.9], 0.1, tone=0.2, **dict(ES, region="exhaust-stack"))
V.box("stack-hinge-bracket", "stack", [-1.07, 3.43, -0.9], [0.07, 0.08, 0.1], tone=0.3, region="exhaust-stack", maxDenominator=100, **ESD)
V.box("rain-cap", "cap-flap", [-1.2, 3.52, -0.9], [0.3, 0.1, 0.3], cr=0.02, tone=0.3, region="rain-cap", **GS)
V.cyl("rain-cap-knuckle", "cap-flap", [-1.05, 3.47, -0.96], [-1.05, 3.47, -0.84], 0.025, tone=0.3, region="rain-cap", maxDenominator=100, material="galvanised-steel", role="detail")
hood = [(1.9, 2.59), (3.03, 2.59), (3.03, 3.25), (2.2, 3.25), (1.9, 2.95)]
V.ext("hood-side-left", "outlet-hood", "side", hood, 0.85, 0.9, tone=0.4, region="discharge-hood", **CS)
V.ext("hood-side-right", "outlet-hood", "side", hood, -0.9, -0.85, tone=0.4, region="discharge-hood", **CS)
V.box("hood-slope", "outlet-hood", [2.05, 3.1, 0], [0.4243, 0.04, 1.7], rz=45, tone=0.4, region="discharge-hood", **CS)
V.box("hood-top", "outlet-hood", [2.615, 3.23, 0], [0.83, 0.04, 1.7], tone=0.4, region="discharge-hood", **CS)
V.box("hood-back", "outlet-hood", [1.925, 2.77, 0], [0.05, 0.36, 1.7], tone=0.4, region="discharge-hood", **CS)
V.ext("hood-outlet-frame", "outlet-hood", "front", rect(2.59, 3.25, -0.85, 0.85), 2.98, 3.03, holes=[rect(2.65, 3.2, -0.8, 0.8)], tone=0.15, region="discharge-hood", maxDenominator=200, **dict(CS, role="detail"))
V.box("outlet-flap", "outlet-flap", [3.0, 2.935, 0], [0.06, 0.63, 1.7], tone=0.3, region="outlet-flap", **GS)
V.decide("The container is a plain box on a skid tank with four corner castings; louvres are 15 tilted slats per panel on both long sides (the elevation draws one side), door and handle are raised panels on the drawn side and the end door on the +x end, so the bounding box exceeds the 2.44 m depth by about 0.02 m.",
         "Der Container ist ein einfacher Kasten auf einem Sockeltank mit vier Eckbeschlaegen; die Lamellen sind 15 geneigte Lamellen je Feld auf beiden Laengsseiten (die Ansicht zeichnet eine Seite), Tuer und Griff sind aufgesetzte Felder auf der gezeichneten Seite, die Stirntuer am +x-Ende, daher ueberschreitet die Bounding-Box die Tiefe 2,44 m um etwa 0,02 m.")
V.decide("The silencer is a capsule along x at z -0.9 (as in the front ellipse, 0.55 m diameter) with flange and two clamp rings as cylinders; the stack stands vertical at x -1.2 and carries the hinge bracket; the rain cap is one rounded box on the cap-flap bone with its knuckle, so it opens about the rig hinge at x -1.05.",
         "Der Schalldaempfer ist eine Kapsel entlang x bei z -0,9 (wie die Ellipse der Vorderansicht, 0,55 m Durchmesser) mit Flansch und zwei Klemmringen als Zylindern; der Kamin steht senkrecht bei x -1,2 und traegt den Scharnierwinkel; die Regenhaube ist eine abgerundete Box auf dem Knochen cap-flap mit Scharnierknoten und oeffnet um das Rig-Scharnier bei x -1,05.")
V.decide("The discharge hood is a shell (two side plates of the elevation pentagon, sloped front panel, top panel, back panel and an outlet frame with the 1.6 m by 0.55 m mouth), the outlet flap one plate on its own bone covering the mouth, so the open flap shows the hollow hood.",
         "Die Ausblashaube ist eine Huelle (zwei Seitenbleche des Fuenfecks der Ansicht, schraege Frontplatte, Deckplatte, Rueckplatte und ein Auslassrahmen mit der Muendung 1,6 m mal 0,55 m), die Auslassklappe eine Platte auf eigenem Knochen, die die Muendung deckt, so zeigt die offene Klappe die hohle Haube.")
V.write(os.path.join(here, "diesel-generator.json"))
