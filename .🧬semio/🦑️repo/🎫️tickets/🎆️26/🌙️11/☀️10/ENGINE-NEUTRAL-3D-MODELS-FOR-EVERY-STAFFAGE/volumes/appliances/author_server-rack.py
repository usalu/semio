import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

circle = lambda cw, cu, rad, n=32: [(cw + rad * math.cos(2 * math.pi * k / n), cu + rad * math.sin(2 * math.pi * k / n)) for k in range(n)]
F = -0.46
FD = F - 0.002
W = 0.4826
PITCH = 0.0889
Y0 = 0.2783
ry = {"count": 6, "translation": [0, PITCH, 0]}

S = [
    box("side-wall", "frame", "rack-steel", "body", 0.5, "cabinet", (0.03, 1.93, 1.0), at=(-0.285, 1.035, 0), repeat={"count": 2, "translation": [0.57, 0, 0]}),
    extrude("roof", "frame", "rack-steel", "body", 0.45, "cabinet", rrect(0.0125, 0, 0.975, 0.6, rad=0.01), "zx", 1.967, 2.0,
            holes=[circle(0.1, 0, 0.08), circle(0.32, 0, 0.08), rrect(0.45, 0, 0.03, 0.3, rad=0.01)]),
    box("side-vent", "frame", "rack-steel", "detail", 0.3, "cabinet", (0.002, 0.012, 0.2), at=(0.3005, 1.626, 0.32), repeat={"count": 7, "translation": [0, 0.05, 0]}, maxd=50),
    box("base-rail", "frame", "rack-steel", "body", 0.45, "cabinet", (0.54, 0.03, 1.0), at=(0, 0.085, 0)),
    box("rear-panel", "frame", "rack-steel", "body", 0.5, "cabinet", (0.54, 1.867, 0.012), at=(0, 1.0335, 0.494)),
    box("door-field", "door", "mesh-steel", "body", 0.18, "door", (0.54, 1.867, 0.025), at=(0, 1.0335, -0.4875)),
    box("door-handle", "door", "mesh-steel", "body", 0.2, "door", (0.022, 0.13, 0.012), at=(0.243, 1.015, -0.506), corner=0.004, maxd=100),
    revolve("caster-l", "feet", "caster-rubber", "body", 0.2, "feet", [(0, 0.011), (0.004, 0.015), (0.07, 0.015)], axis=(0, 1, 0), translate=[-0.25, 0, 0.4], repeat={"count": 2, "translation": [0.5, 0, 0]}),
    revolve("caster-r", "feet", "caster-rubber", "body", 0.2, "feet", [(0, 0.011), (0.004, 0.015), (0.07, 0.015)], axis=(0, 1, 0), translate=[-0.25, 0, -0.4], repeat={"count": 2, "translation": [0.5, 0, 0]}),

    box("storage", "equipment", "chassis", "body", 0.4, "servers", (W, 0.1768, 0.72), at=(0, 0.1889, -0.1), maxd=100),
    box("storage-bay-0", "equipment", "chassis", "detail", 0.3, "servers", (0.052, 0.045, 0.004), at=(-0.164, 0.155, FD), repeat={"count": 6, "translation": [0.0645, 0, 0]}, maxd=50),
    box("storage-bay-1", "equipment", "chassis", "detail", 0.3, "servers", (0.052, 0.045, 0.004), at=(-0.164, 0.2105, FD), repeat={"count": 6, "translation": [0.0645, 0, 0]}, maxd=50),
    ellipsoid("storage-led", "equipment", "chassis", "detail", 0.1, "servers", (0.0065, 0.0065, 0.002), at=(0.215, 0.189, F), maxd=50),

    box("server", "equipment", "chassis", "body", 0.45, "servers", (W, 0.0879, 0.72), at=(0, Y0 + 0.04395, -0.1), repeat=ry, maxd=100),
    box("server-bay-0", "equipment", "chassis", "detail", 0.3, "servers", (0.043, 0.06, 0.004), at=(0.0415, 0.3238, FD), repeat=ry, maxd=50),
    box("server-bay-1", "equipment", "chassis", "detail", 0.3, "servers", (0.043, 0.06, 0.004), at=(0.097, 0.3238, FD), repeat=ry, maxd=50),
    box("server-bay-2", "equipment", "chassis", "detail", 0.3, "servers", (0.043, 0.06, 0.004), at=(0.1525, 0.3238, FD), repeat=ry, maxd=50),
    box("server-bay-3", "equipment", "chassis", "detail", 0.3, "servers", (0.043, 0.06, 0.004), at=(0.208, 0.3238, FD), repeat=ry, maxd=50),
    box("server-vent", "equipment", "chassis", "detail", 0.3, "servers", (0.162, 0.0585, 0.003), at=(-0.124, 0.32305, F - 0.0015), repeat=ry, maxd=50),
    ellipsoid("server-led", "equipment", "chassis", "detail", 0.1, "servers", (0.0065, 0.0065, 0.002), at=(-0.216, 0.32225, F), repeat=ry, maxd=50),

    box("patch-40", "equipment", "blank-steel", "body", 0.5, "blanks", (W, 0.04345, 0.1), at=(0, 1.8558, -0.41), maxd=100),
    box("brush-41", "equipment", "blank-steel", "body", 0.38, "blanks", (W, 0.04345, 0.1), at=(0, 1.9002, -0.41), maxd=100),
    box("switch-42", "equipment", "chassis", "body", 0.3, "switch", (W, 0.04345, 0.4), at=(0, 1.9447, -0.26), maxd=100),
    box("switch-port-0", "equipment", "chassis", "detail", 0.3, "switch", (0.0155, 0.0105, 0.004), at=(-0.18225, 1.93775, FD), repeat={"count": 12, "translation": [0.0245, 0, 0]}, maxd=50),
    box("switch-port-1", "equipment", "chassis", "detail", 0.3, "switch", (0.0155, 0.0105, 0.004), at=(-0.18225, 1.95175, FD), repeat={"count": 12, "translation": [0.0245, 0, 0]}, maxd=50),
    ellipsoid("switch-led", "equipment", "chassis", "detail", 0.1, "switch", (0.0065, 0.0065, 0.002), at=(0.215, 1.94468, F), maxd=50),
]
D = [dec("The cabinet has the door at the -z face (the plan draws its 25 mm leaf at z = -0.5 to -0.475, the elevation its mesh) and the rear at +z; the elevation posts and the front side panel are read as two 30 mm side walls, the 33 mm roof as a rounded plate with the two 160 mm fan openings and the cable slot of the plan cut through it.",
         "Der Schrank hat die Tür an der -z-Fläche (der Grundriss zeichnet ihr 25-mm-Blatt bei z = -0,5 bis -0,475, die Seitenansicht ihr Gitter) und die Rückseite bei +z; die Pfosten der Seitenansicht und das seitliche Wandfeld der Vorderansicht sind als zwei 30-mm-Seitenwände gelesen, das 33-mm-Dach als gerundete Platte, in die die beiden 160-mm-Lüfteröffnungen und der Kabelschlitz des Grundrisses geschnitten sind."),
     dec("The equipment stack is read from the elevation: a 4U storage chassis, six 2U servers as one box repeated six times at the 88.9 mm pitch, two 1U blanking plates and a 1U switch; bays, LEDs, vents and ports are thin plates 2 to 4 mm proud of their faces, each as one repeated solid. Depths (0.72 m servers, 0.4 m switch, 0.1 m blanks) and the 15 mm gap behind the door are not drawn and are estimates.",
         "Der Geräteturm ist aus der Seitenansicht gelesen: ein 4HE-Speicherchassis, sechs 2HE-Server als ein sechsmal im Raster 88,9 mm wiederholter Quader, zwei 1HE-Blenden und ein 1HE-Switch; Einschübe, LEDs, Lüftungsfelder und Ports sind dünne Platten 2 bis 4 mm vor ihren Flächen, je als ein wiederholter Körper. Tiefen (0,72 m Server, 0,4 m Switch, 0,1 m Blenden) und der Spalt von 15 mm hinter der Tür sind nicht gezeichnet und geschätzt."),
     dec("The perforated door is a solid 25 mm plate; its 588 drawn slots, the panel seams and the cabinet's rear panel (not drawn) are not modelled in detail; the handle stands 12 mm proud, so the depth is 1.012 m against 1.0 m. The four casters are two solids repeated along x; the seven slots of the +x side panel are one box repeated seven times.",
         "Die perforierte Tür ist eine massive 25-mm-Platte; ihre 588 gezeichneten Schlitze, die Blechfugen und die nicht gezeichnete Rückwand des Schranks werden nicht im Detail modelliert; der Griff steht 12 mm vor, die Tiefe beträgt daher 1,012 m gegenüber 1,0 m. Die vier Rollen sind zwei entlang x wiederholte Körper; die sieben Schlitze der +x-Seitenwand sind ein siebenmal wiederholter Quader.")]
write("server-rack", S, D)
