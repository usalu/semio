import importlib.util, os
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("finned-heat-exchanger")
AL = dict(material="aluminium-fin", role="body")
CU = dict(material="copper", role="body")

V.box("fin", "fins", [0, 0.26, -0.2255], [0.1, 0.48, 0.009], tone=0.7, region="fins", rep=dict(count=46, step=[0, 0, 0.01]), **AL)
V.box("end-plate-left", "fins", [0, 0.26, 0.24], [0.1, 0.48, 0.02], tone=0.6, region="fins", **AL)
V.box("end-plate-right", "fins", [0, 0.26, -0.24], [0.1, 0.48, 0.02], tone=0.6, region="fins", **AL)
V.cyl("tube-row-front", "tubes", [-0.022, 0.04, -0.254], [-0.022, 0.04, 0.254], 0.009, tone=0.5, region="tubes", rep=dict(count=12, step=[0, 0.04, 0]), **CU)
V.cyl("tube-row-rear", "tubes", [0.023, 0.06, -0.254], [0.023, 0.06, 0.254], 0.009, tone=0.5, region="tubes", rep=dict(count=11, step=[0, 0.04, 0]), **CU)
V.box("header-bottom", "frame", [0, 0.01, 0], [0.15, 0.02, 0.5], cr=0.006, tone=0.4, region="headers", **CU)
V.box("header-top", "frame", [0, 0.51, 0], [0.15, 0.02, 0.5], cr=0.006, tone=0.4, region="headers", **CU)
V.cyl("inlet-stub", "frame", [0.07, 0.51, 0.2], [0.13, 0.51, 0.2], 0.006, tone=0.4, region="headers", **CU)
V.cyl("outlet-stub", "frame", [0.07, 0.01, -0.2], [0.13, 0.01, -0.2], 0.006, tone=0.4, region="headers", **CU)
V.decide("Fins are 46 aluminium plates of 9 mm at 10 mm pitch standing across the tube axis (a coarse pitch so the pack still reads as individual fins at 1:50 without more than 6 % gaps in the front and plan silhouettes); the real pitch is 2 to 3 mm.",
         "Die Lamellen sind 46 Aluminiumplatten von 9 mm im Abstand von 10 mm quer zur Rohrachse (grober Abstand, damit das Paket bei 1:50 als einzelne Lamellen lesbar bleibt und die Silhouetten in Vorderansicht und Grundriss hoechstens 6 % Luecken haben); real sind es 2 bis 3 mm.")
V.decide("The 23 tubes are two staggered rows (12 and 11, 40 mm pitch, rows 45 mm apart) written as two solids with repeat; they run through the end plates and stick out 4 mm on each side. The 2D front view draws only 11 tubes; the elevation draws all 23, which the 3D follows.",
         "Die 23 Rohre sind zwei versetzte Reihen (12 und 11, 40 mm Teilung, Reihenabstand 45 mm) als zwei Koerper mit repeat; sie laufen durch die Endplatten und stehen beidseitig 4 mm vor. Die 2D-Vorderansicht zeichnet nur 11 Rohre, die Ansicht alle 23, dem folgt das 3D.")
V.decide("Headers are rounded copper boxes (150 mm by 20 mm by 500 mm) above and below the pack as drawn in the plan; both connection stubs are cylinders on the same side (x 0.07 to 0.13), inlet on top at z +0.2, outlet at the bottom at z -0.2.",
         "Die Sammler sind abgerundete Kupferkaesten (150 mm mal 20 mm mal 500 mm) ueber und unter dem Paket wie im Grundriss; beide Anschlussstutzen sind Zylinder auf derselben Seite (x 0,07 bis 0,13), Eintritt oben bei z +0,2, Austritt unten bei z -0,2.")
V.write(os.path.join(here, "finned-heat-exchanger.json"))
