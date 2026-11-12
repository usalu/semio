import importlib.util, os
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("evaporative-humidifier")
GS = dict(material="galvanised-steel", role="body")
rect = lambda y0, y1, z0, z1: [(y0, z0), (y1, z0), (y1, z1), (y0, z1)]

V.box("sump-tray", "casing", [0, 0.01, 0], [0.48, 0.02, 0.38], tone=0.3, region="casing", maxDenominator=50, **GS)
V.box("side-panel-front", "casing", [0, 0.3, 0.195], [0.5, 0.6, 0.01], tone=0.3, region="casing", **GS)
V.box("side-panel-rear", "casing", [0, 0.3, -0.195], [0.5, 0.6, 0.01], tone=0.3, region="casing", **GS)
V.ext("roof", "casing", "plan", [(-0.25, -0.2), (0.25, -0.2), (0.25, 0.2), (-0.25, 0.2)], 0.58, 0.6,
      holes=[[(-0.2, -0.18), (0.2, -0.18), (0.2, 0.18), (-0.2, 0.18)]], tone=0.3, region="casing", **GS)
for side, x0, x1 in [("left", -0.25, -0.24), ("right", 0.24, 0.25)]:
    V.ext(f"end-wall-{side}", "casing", "front", rect(0.02, 0.58, -0.19, 0.19), x0, x1, holes=[rect(0.09, 0.47, -0.17, 0.17)], tone=0.3, region="casing", **GS)
V.box("water", "water", [0, 0.045, 0], [0.44, 0.05, 0.38], cr=0.003, tone=0.45, region="sump", material="water", role="body")
V.box("pad", "pad", [0, 0.28, 0], [0.4, 0.4, 0.36], tone=0.7, region="pad", material="cellulose", role="body")
V.box("trough-floor", "trough", [0, 0.4825, 0], [0.42, 0.005, 0.38], tone=0.4, region="trough", material="pp-plastic", role="body")
V.ext("trough-walls", "trough", "plan", [(-0.21, -0.19), (0.21, -0.19), (0.21, 0.19), (-0.21, 0.19)], 0.485, 0.53,
      holes=[[(-0.205, -0.185), (0.205, -0.185), (0.205, 0.185), (-0.205, 0.185)]], tone=0.4, region="trough", material="pp-plastic", role="body")
V.box("film", "pad", [0, 0.489, 0], [0.38, 0.008, 0.34], tone=0.85, region="pad", maxDenominator=50, material="water", role="body")
for k, (z, md) in enumerate([(-0.1, 100), (0.0, 50), (0.1, 50)]):
    V.ell(f"drop-row-{k + 1}", "drip", [-0.1, 0.5, z], [0.01, 0.013, 0.01], tone=0.85, region="pad", maxDenominator=md, material="water", role="body", rep=dict(count=3, step=[0.1, 0, 0]))
V.decide("The casing is a shell of two side panels, a roof ring and two end walls with a window onto the pad; the roof ring leaves the distribution trough and the drops visible from above and the windows show the pad from the ends, while the silhouettes still equal the drawn full rectangles.",
         "Das Gehaeuse ist eine Huelle aus zwei Seitenwaenden, einem Dachrahmen und zwei Stirnwaenden mit Fenster zum Pad; der Dachrahmen laesst Verteilrinne und Tropfen von oben sehen, die Fenster das Pad von den Enden, die Silhouetten entsprechen dennoch den gezeichneten vollen Rechtecken.")
V.decide("The trough is a tray (floor and 5 mm walls) holding a 8 mm water film; the 3 by 3 drops are ellipsoids on the prismatic drip bone written as three rows with repeat, so they fall through the pad together and stay on the pad axis.",
         "Die Rinne ist eine Wanne (Boden und 5 mm Waende) mit 8 mm Wasserfilm; die 3 mal 3 Tropfen sind Ellipsoide auf dem Schubknochen drip in drei Reihen mit repeat, sie fallen gemeinsam durch das Pad und bleiben auf der Padachse.")
V.decide("The pad is one cellulose block of 400 mm by 400 mm by 360 mm; the sump water is a rounded box in the tray. The pad stands 10 mm above the water, the drawing's own gap.",
         "Das Pad ist ein Zellulose-Block 400 mm mal 400 mm mal 360 mm; das Sumpfwasser ist ein abgerundeter Kasten in der Wanne. Das Pad steht 10 mm ueber dem Wasser, wie in der Zeichnung.")
V.write(os.path.join(here, "evaporative-humidifier.json"))
