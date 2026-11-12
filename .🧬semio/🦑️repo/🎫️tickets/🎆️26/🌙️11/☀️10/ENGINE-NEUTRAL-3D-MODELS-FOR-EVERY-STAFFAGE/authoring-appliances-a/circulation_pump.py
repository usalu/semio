from lib import *

H = Ctx("housing", "cast-iron", "casing", "body", 0.42)
M = Ctx("motor", "aluminium", "motor", "body", 0.62)
TB = Ctx("terminal", "polyamide", "terminal-box", "body", 0.78)
R = Ctx("rotor", "window-glass", None, "detail", 0.95)
PW = 0.04
S = []

S.append(rev("casing", H, [[0, 0.0515], [0.006, 0.0575], [0.044, 0.0575], [0.05, 0.0515]], "w", origin=(0, 0, 0.015)))
copper, brass = H(material="copper", tone=0.7), H(material="brass", tone=0.5)
for side, sg in (("l", -1), ("r", 1)):
    x0, x1 = (0.054, 0.0775) if sg > 0 else (-0.0775, -0.054)
    prof = [[0, 0.021], [0.013, 0.021], [0.013, 0.0195], [0.0235, 0.0195]] if sg < 0 else [[0, 0.0195], [0.0105, 0.0195], [0.0105, 0.021], [0.0235, 0.021]]
    S.append(rev("union-" + side, brass, prof, "u", origin=(x0, 0, PW), maxd=50))
    S.append(pipe("stub-" + side, copper, "u", (sg * 0.0825, 0, PW), 0.015, 0.014, 0.009, n=32))

S.append(rev("motor-head", M, [[0, 0.0415], [0.004, 0.0455], [0.071, 0.0455], [0.075, 0.0415]], "w", origin=(0, 0, -0.06)))
S.append(rev("motor-ring", M(role="detail", tone=0.5), [[0, 0.0455], [0, 0.0462], [0.003, 0.0462], [0.003, 0.0455]], "w", origin=(0, 0, -0.0465), maxd=50))
S.append(pipe("bezel", M(role="detail", tone=0.5), "w", (0, 0, -0.0625), 0.005, 0.0305, 0.0265, n=40, maxd=50))
S.append(cyl("window", M(material="window-glass", region="rotor-window", tone=0.18), "w", (0, 0, -0.06425), 0.0015, 0.0265, maxd=100))
S.append(cyl("dial", M(material="polyamide", role="detail", tone=0.25), "w", (0, 0, -0.06075), 0.0015, 0.0265, maxd=50))

S.append(box("terminal-neck", TB(tone=0.7), (-0.02, -0.0075, -0.045), (0.02, 0.0005, -0.005), cr=0.002))
S.append(box("terminal-box", TB, (-0.03, 0, -0.05), (0.03, 0.025, 0), cr=0.004))
S.append(box("terminal-lid", TB(tone=0.82), (-0.0305, 0.025, -0.0505), (0.0305, 0.0425, 0.0005), cr=0.004, maxd=100))
S.append(cyl("gland", TB(role="detail", tone=0.4), "u", (-0.03825, 0.01125, -0.025), 0.0165, 0.0068, maxd=50))
S.append(cyl("led", TB(role="detail", tone=0.95), "w", (0, 0.033, -0.0503), 0.0006, 0.0045, maxd=50))

S.append(ext("pointer", R, [[-0.007, 0], [0.007, 0], [0, 0.026]], "xy", -0.0635, -0.0615, maxd=50))
S.append(ext("tail", R(tone=0.7), [[-0.005, 0], [0.005, 0], [0, -0.012]], "xy", -0.0635, -0.0615, maxd=50))
S.append(cyl("hub", R(tone=0.35), "w", (0, 0, -0.0625), 0.002, 0.0055, maxd=50))

D = [
    text("Axes follow the plan and front views: the pump head (casing, 0.115 m across, z 0.015..0.065) and the wet-rotor motor (z -0.06..0.015) are cylinders about the lateral axis w; the unions and copper stubs lie on a line along u at w = 0.04, the centre of the casing.",
         "Die Achsen folgen Grundriss und Ansicht: Pumpenkopf (Gehäuse, 0,115 m Durchmesser, z 0,015..0,065) und Nassläufermotor (z -0,06..0,015) sind Zylinder um die Querachse w; Verschraubungen und Kupferstutzen liegen auf einer Linie entlang u bei w = 0,04, der Gehäusemitte."),
    text("The 2D drawing leaves a 4.5 mm gap between the motor top and the terminal box. A neck block is added so the box sits on the motor; the box lateral extent (w -0.05..0) follows the plan, the front view draws it 5 mm further out.",
         "Die 2D-Zeichnung lässt 4,5 mm Luft zwischen Motoroberseite und Klemmenkasten. Ein Halsblock lässt den Kasten auf dem Motor aufsitzen; die seitliche Ausdehnung (w -0,05..0) folgt dem Grundriss, die Ansicht zeichnet ihn 5 mm weiter aussen."),
    text("The copper stubs are hollow (bore 18 mm, shown end-on in the front view); the union nut and neck are one stepped revolve whose step is the drawn nut edge. Hollow bodies are rings extruded with a hole, because closed revolve profiles are rejected by the validator.",
         "Die Kupferstutzen sind hohl (Bohrung 18 mm, in der Ansicht stirnseitig gezeichnet); Überwurfmutter und Hals sind ein gestuftes Drehteil, dessen Stufe die gezeichnete Mutterkante ist. Hohlkörper sind Ringe mit Loch, da geschlossene Revolve-Profile vom Validator abgelehnt werden."),
    text("The inspection window is a glass disc, a pointer layer on the rotor bone and a dial disc behind it; pointer, tail and hub turn with the revolute rotor bone about w, everything else is rigid. Fine parts (unions, bezel, ring, gland, LED, rotor) carry maxDenominator 50.",
         "Das Schauglas besteht aus einer Glasscheibe, einer Zeigerebene am Rotorknochen und einer Zifferblattscheibe dahinter; Zeiger, Schwanz und Nabe drehen mit dem Drehknochen Rotor um w, alles übrige ist starr. Feinteile (Verschraubungen, Blende, Ring, Verschraubung, LED, Rotor) tragen maxDenominator 50."),
]
write("circulation-pump", S, D)
