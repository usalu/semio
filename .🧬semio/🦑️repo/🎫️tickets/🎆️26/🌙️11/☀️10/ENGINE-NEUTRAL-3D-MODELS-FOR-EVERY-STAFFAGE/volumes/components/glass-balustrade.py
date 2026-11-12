import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
P = "panel"; K = "clamp-steel"
shoe = [(0, 0.15), (0, -0.15), (0.1, -0.15), (0.1, -0.0275), (0.02, -0.0275), (0.02, 0.0275), (0.1, 0.0275), (0.1, 0.15)]
s = [
 L.ext("shoe", P, "steel", "body", 0.35, shoe, "xy", -1, 1, K),
 L.box("gasket-a", P, "steel", "body", 0.2, [0.08, 0.01675, 2], [0.06, 0.019125, 0], K),
 L.box("gasket-b", P, "steel", "body", 0.2, [0.08, 0.01675, 2], [0.06, -0.019125, 0], K),
 L.box("glass", P, "glass", "glazing", 0.92, [1.06, 0.0215, 1.96], [0.55, 0, 0], "panel-glass"),
 L.cap("handrail", P, "steel", "body", 0.35, [1.079, 0, -0.9788], [1.079, 0, 0.9788], 0.0212, region="handrail-steel"),
 L.cyl("bolt-a", P, "steel", "detail", 0.15, [0.05, 0.15, -0.8], [0, 1, 0], 0.003, 0.007, K, maxD=50, repeat={"count": 5, "translation": [0, 0, 0.4]}),
 L.cyl("bolt-b", P, "steel", "detail", 0.15, [0.05, -0.15, -0.8], [0, -1, 0], 0.003, 0.007, K, maxD=50, repeat={"count": 5, "translation": [0, 0, 0.4]}),
]
L.write("glass-balustrade", s, [
 ("The base shoe is a U-channel (extrude of the elevation outline along the 2 m length) whose slot is filled by two EPDM gaskets and the 21.5 mm laminated glass pane, 1.06 m high and 1.96 m long; the 42.4 mm handrail tube is a capsule whose round ends reach the 2 m line.", "Der Fussschuh ist ein U-Profil (Extrusion des Ansichtsumrisses über die 2 m Länge), dessen Schlitz von zwei EPDM-Dichtungen und der 21,5 mm dicken Verbundglasscheibe, 1,06 m hoch und 1,96 m lang, gefüllt ist; das Handlaufrohr 42,4 mm ist eine Kapsel, deren runde Enden die 2-m-Linie erreichen."),
 ("The 2D views disagree about the clamp bolts (elevation circles with axis along the length, front squares); modelled as the physical solution: five clamp bolts every 0.4 m per side with axis across the shoe and heads 3 mm proud of the shoe flanks (only at 1:50). Glass reflex lines are 2D texture.", "Die 2D-Ansichten widersprechen sich bei den Klemmschrauben (Kreise in der Ansicht mit Achse in Längsrichtung, Quadrate in der Vorderansicht); modelliert ist die physikalische Lösung: je Seite fünf Klemmschrauben im Abstand 0,4 m mit Achse quer zum Schuh und 3 mm vorstehenden Köpfen (nur bei 1:50). Glasreflexlinien sind 2D-Textur."),
])
