import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

R = 0.02415
X = 0.350825
BEND = 0.18
YB = 0.59585
YT = 0.77585
A = lib.Asset("bicycle-stand", "galv-steel", 0.45, "hoop-galv")
left = [[-X, 0, 0]] + lib.arc([-(X - BEND), YB, 0], BEND, 180, 90, 8) + [[0, YT, 0]]
right = [[0, YT, 0]] + lib.arc([X - BEND, YB, 0], BEND, 90, 0, 8) + [[X, 0, 0]]
for side, path, sx in (("l", left, -1), ("r", right, 1)):
    A.loft(f"hoop-{side}", "hoop", [dict(c=c, rx=R, ry=R) for c in path], smooth=True)
for side, sx in (("l", -1), ("r", 1)):
    A.rev(f"ground-collar-{side}", "hoop", [sx * X, 0, 0], [[0, 0.0], [0, R + 0.0005], [0.008, R + 0.0005], [0.008, 0]], axis="y", role="detail", tone=0.3, maxDenominator=100)
A.cap("crossbar", "crossbar", [-X, 0.4, 0], [X, 0.4, 0], R, region="crossbar-galv")
for side, sx in (("l", -1), ("r", 1)):
    A.rev(f"weld-collar-{side}", "crossbar", [sx * 0.31265, 0.4, 0], [[-0.0045, 0], [-0.0045, R + 0.0005], [0.0045, R + 0.0005], [0.0045, 0]], axis="x", role="detail", tone=0.3, region="crossbar-galv", maxDenominator=100)
A.save([
    ("The hoop is two straight legs, two quarter-torus bends of 0.18 m centre-line radius and a straight top, all of the 48.3 mm tube, matching the authored outer and inner outline of the elevation.",
     "Der Bügel besteht aus zwei geraden Beinen, zwei Viertel-Torus-Bögen mit 0,18 m Mittellinienradius und einem geraden Oberteil, alles aus dem Rohr 48,3 mm; das entspricht Aussen- und Innenkontur der Ansicht."),
    ("Legs are lofts so the foot ends flat on the ground; the crossbar is a capsule running centre to centre of the legs, its round ends lie inside the leg tubes.",
     "Die Beine sind Lofts, damit der Fuss flach auf dem Boden endet; der Querholm ist eine Kapsel von Beinmitte zu Beinmitte, ihre runden Enden liegen im Beinrohr."),
    ("The tube bore of the front view and the dots of the plan are not modelled (the tube is closed solid); ground and weld collars are thin rings shown to 1:100.",
     "Die Rohrbohrung der Vorderansicht und die Punkte der Draufsicht werden nicht modelliert (das Rohr ist massiv geschlossen); Boden- und Schweisskragen sind dünne Ringe bis 1:100."),
])
