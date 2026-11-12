from lib import *

CS = Ctx("body", "abs-plastic", "casing", "body", 0.88)
WN = Ctx("body", "polycarbonate", "window", "body", 0.1)
PL = Ctx("body", "abs-plastic", "plate", "body", 0.78)
DK = Ctx("disc", "aluminium-disc", "disc", "body", 0.86)
ZF, ZB = -0.058, 0.035
S = []

S.append(box("back-plate", CS(tone=0.78), (-0.085, -0.1425, ZB), (0.085, 0.1425, 0.065), cr=0.004))
S.append(box("body", CS, (-0.085, -0.1, ZF), (0.085, 0.1425, ZB), cr=0.012))
S.append(box("terminals", CS(tone=0.55), (-0.072, -0.1425, ZF), (0.072, -0.1, ZB), cr=0.004))
S.append(cyl("screws", CS(role="detail", tone=0.2), "w", (-0.058, -0.121, ZF - 0.0005), 0.001, 0.005, repeat={"count": 2, "translation": [0.116, 0, 0]}, maxd=50))
S.append(cyl("eye", CS(role="detail", tone=0.15), "w", (0, 0.123, -0.06), 0.004, 0.008, maxd=50))

S.append(pipe("rim", WN(tone=0.25), "w", (0, 0.01, -0.0615), 0.007, 0.05, 0.0465, n=48))
S.append(cyl("window-glass", WN(role="glazing"), "w", (0, 0.01, -0.06425), 0.0015, 0.0465))
S.append(box("counter", WN, (-0.045, 0.078, -0.065), (0.045, 0.104, ZF), cr=0.003))
dg = WN(role="detail", tone=0.92)
S.append(box("digits", dg, (-0.0425, 0.082, -0.0655), (-0.0315, 0.1, -0.065), repeat={"count": 5, "translation": [0.0145, 0, 0]}, maxd=50))
S.append(box("digit-6", dg(tone=0.55), (0.03, 0.082, -0.0655), (0.041, 0.1, -0.065), maxd=50))

S.append(box("plate", PL, (-0.065, -0.085, -0.065), (0.065, -0.045, ZF), cr=0.003))
S.append(box("plate-lines", PL(role="detail", tone=0.25), (-0.052, -0.0555, -0.0655), (0.052, -0.0545, -0.065), repeat={"count": 3, "translation": [0, -0.01, 0]}, maxd=50))

S.append(cyl("disc", DK, "w", (0, 0, -0.061), 0.001, 0.042))
S.append(cyl("hub", DK(role="detail", tone=0.3), "w", (0, 0, -0.0615), 0.002, 0.006, maxd=50))
S.append(box("mark", DK(role="detail", tone=0.1), (0.02, -0.005, -0.0618), (0.04, 0.005, -0.0609)))

D = [
    text("The plan and front views give the depth stack: 30 mm back plate (z 0.035..0.065), a 93 mm housing with 12 mm rounded edges, and a 7 mm raised front with the disc window (ring 0.1 m across), the counter window and the name plate (z -0.065..-0.058); the terminal block hangs below the housing at the same depth.",
         "Grundriss und Ansicht geben den Tiefenaufbau: 30 mm Rückplatte (z 0,035..0,065), ein 93 mm tiefes Gehäuse mit 12 mm gerundeten Kanten und eine 7 mm erhabene Front mit dem Scheibenfenster (Ring 0,1 m Durchmesser), dem Zählwerkfenster und dem Typenschild (z -0,065..-0,058); der Klemmenblock hängt in gleicher Tiefe unter dem Gehäuse."),
    text("The Ferraris disc (aluminium, 1 mm, 0.084 m across) lies behind a polycarbonate glazing pane inside the window ring and turns with the revolute disc bone about w; hub and mark are on that bone, so every pose and the run clips show the mark circling. The digits and name-plate lines are 0.5 mm printed relief on the window and plate and only appear at 1:50.",
         "Die Ferraris-Scheibe (Aluminium, 1 mm, 0,084 m Durchmesser) liegt hinter einer Polycarbonat-Scheibe im Fensterring und dreht mit dem Drehknochen Scheibe um w; Nabe und Marke liegen an diesem Knochen, sodass jede Pose und die Lauf-Clips die umlaufende Marke zeigen. Ziffern und Typenschildlinien sind 0,5 mm Reliefdruck auf Fenster und Schild und erscheinen nur bei 1:50."),
    text("The plan seam line between housing and back plate is the joint of the two boxes and the plan screw lines are not modelled (terminal screws are modelled as two heads on the terminal block front, from the elevation).",
         "Die Fugenlinie im Grundriss zwischen Gehäuse und Rückplatte ist die Stossfläche der beiden Quader, die Schraubenlinien im Grundriss sind nicht modelliert (Klemmenschrauben sind nach dem Aufriss als zwei Köpfe auf der Klemmenblockfront modelliert)."),
]
write("electricity-meter", S, D)
