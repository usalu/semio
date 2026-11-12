from lib import *

plate = Ctx("body", "powder-coated-steel", "plate", "body", 0.8)
collar = Ctx("body", "galvanised-steel", "collar", "body", 0.5)
S = [
    pipe("collar", collar, "v", (0, 0.025, 0), 0.05, 0.125, 0.112),
    rev("collar-bead", collar(role="detail", tone=0.3), [[0.0235, 0.125], [0.0235, 0.1275], [0.0285, 0.1275], [0.0285, 0.125]], "v", maxd=50),
    box("flange", plate(tone=0.88), (-0.2975, -0.012, -0.2975), (0.2975, 0, 0.2975), cr=0.003),
    frustum_y("cone-1", plate, -0.012, -0.032, 0.265, 0.245),
    frustum_y("cone-2", plate(tone=0.7), -0.032, -0.052, 0.2, 0.18),
    frustum_y("cone-3", plate(tone=0.6), -0.052, -0.07, 0.135, 0.115),
]
screw = plate(role="detail", tone=0.45)
for name, z in (("screw-right", -0.27), ("screw-left", 0.27)):
    S.append(ell(name, screw, (-0.27, 0, z), (0.006, 0.0025, 0.006), repeat={"count": 2, "translation": [0.54, 0, 0]}, maxd=50))
D = [
    text("Lateral extents follow the plan: a 0.595 m square flange, square stepped cones (lofts with a squareness exponent of 12) and a round collar, a hollow revolved tube whose bore is the 0.112 m opening drawn in plan.",
         "Die seitlichen Masse folgen dem Grundriss: ein quadratischer Flansch von 0,595 m, quadratische Kegelstufen (Lofts mit Eckigkeitsexponent 12) und ein runder Kragen als hohles Drehteil, dessen Bohrung die im Grundriss gezeichnete Öffnung von 0,112 m ist."),
    text("The three cones are stacked square frusta of solid sheet as drawn; the two slot lines are the shadow gaps between the cone steps and need no own solid. The collar bead and the four flange screws appear only at 1:50 (maxDenominator).",
         "Die drei Kegel sind wie gezeichnet gestapelte quadratische Kegelstümpfe; die beiden Schlitzlinien sind die Schattenfugen zwischen den Kegelstufen und brauchen keinen eigenen Körper. Kragenwulst und die vier Flanschschrauben erscheinen nur bei 1:50 (maxDenominator)."),
    text("The screw pairs use one solid each with repeat along x; the single rigid bone carries every solid, so all poses and clips (supply air only changes thermal state) read the same body.",
         "Die Schraubenpaare nutzen je einen Körper mit repeat entlang x; der einzige starre Knochen trägt alle Körper, alle Posen und Clips (die Zuluft ändert nur den thermischen Zustand) zeigen denselben Körper."),
]
write("ceiling-diffuser", S, D)
