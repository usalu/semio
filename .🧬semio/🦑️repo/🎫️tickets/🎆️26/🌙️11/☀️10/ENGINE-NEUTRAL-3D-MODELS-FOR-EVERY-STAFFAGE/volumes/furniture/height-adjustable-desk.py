import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
from authoring_furniture_lib import *
c = Ctx("height-adjustable-desk")
SD, PB, EH = "steel-desk", "particleboard-laminate", "equipment-housing"
for side, cx, dz in (("l", 0.25, -0.1), ("r", 1.35, 0.1)):
    foot, stage, col = f"foot-{'left' if side == 'l' else 'right'}", f"stage-{'left' if side == 'l' else 'right'}", f"column-{'left' if side == 'l' else 'right'}"
    dreg = "drive-left" if side == "l" else "drive-right"
    c.bw(f"{side}-plate-bar", foot, SD, "body", 0.3, (cx - 0.25, cx + 0.25), (0, 0.016), (-0.025, 0.025), corner=0.007, region="steel-desk")
    c.bw(f"{side}-plate-cross", foot, SD, "body", 0.3, (cx - 0.05, cx + 0.05), (0, 0.016), (-0.3, 0.3), corner=0.007, region="steel-desk")
    c.bw(f"{side}-sleeve", foot, SD, "body", 0.28, (cx - 0.032, cx + 0.032), (0.012, 0.4), (-0.032, 0.032), corner=0.004, region="steel-desk")
    c.bw(f"{side}-sleeve-collar", foot, SD, "detail", 0.18, (cx - 0.036, cx + 0.036), (0.385, 0.405), (-0.036, 0.036), corner=0.003, region="steel-desk", maxD=50)
    c.bw(f"{side}-drive", foot, EH, "body", 0.22, (cx - 0.05, cx + 0.05), (0.012, 0.16), (dz - 0.05, dz + 0.05), corner=0.012, region=dreg)
    c.bw(f"{side}-stage", stage, SD, "body", 0.42, (cx - 0.027, cx + 0.027), (0.05, 0.55), (-0.027, 0.027), region="steel-desk")
    c.bw(f"{side}-stage-cap", stage, SD, "detail", 0.2, (cx - 0.029, cx + 0.029), (0.538, 0.55), (-0.029, 0.029), region="steel-desk", maxD=50)
    c.bw(f"{side}-column", col, SD, "body", 0.55, (cx - 0.022, cx + 0.022), (0.25, 0.695), (-0.022, 0.022), region="steel-desk")
    c.bw(f"{side}-bracket", col, SD, "body", 0.22, (cx - 0.045, cx + 0.045), (0.66, 0.695), (-0.045, 0.045), region="steel-desk", maxD=100)
c.bw("top-board", "top", PB, "body", 0.58, (0, 1.6), (0.695, 0.72), (-0.4, 0.4), corner=0.004, region="top-board")
c.bw("top-rail", "top", SD, "body", 0.35, (0.25, 1.35), (0.62, 0.67), (0.25, 0.3), corner=0.003, region="steel-desk")
c.bw("top-rail-mount", "top", SD, "body", 0.35, (0.4, 0.45), (0.665, 0.7), (0.25, 0.3), region="steel-desk", repeat={"count": 2, "translation": [0.75, 0, 0]})
c.bw("top-cable-tray-floor", "top", SD, "detail", 0.3, (0.55, 1.05), (0.576, 0.58), (0.2, 0.3), region="steel-desk", maxD=100)
c.bw("top-cable-tray-wall", "top", SD, "detail", 0.3, (0.55, 1.05), (0.58, 0.62), (0.2, 0.204), region="steel-desk", maxD=100, repeat={"count": 2, "translation": [0, 0, 0.096]})
c.bw("top-keypad", "top", PB, "detail", 0.15, (1.42, 1.51), (0.675, 0.695), (-0.4, -0.38), corner=0.003, region="top-board", maxD=50)
c.bw("monitor-base", "top", EH, "body", 0.25, (0.51, 0.69), (0.72, 0.732), (0.05, 0.25), corner=0.005, region="monitor-housing")
c.loft("monitor-neck", "top", EH, "body", 0.3, [([0.6, 0.732, 0.15], 0.015, 0.015, 4), ([0.6, 0.845, 0.15], 0.012, 0.012, 4)], Z, X, region="monitor-housing")
c.bw("monitor-panel", "top", EH, "body", 0.2, (0.3, 0.9), (0.815, 1.18), (0.13, 0.17), corner=0.012, region="monitor-housing")
c.bw("monitor-screen", "top", EH, "glazing", 0.1, (0.315, 0.885), (0.83, 1.165), (0.17, 0.176), region="monitor-housing")
c.bw("laptop-base", "top", EH, "body", 0.4, (0.95, 1.2), (0.72, 0.729), (-0.15, 0.2), corner=0.003, region="laptop-lid")
c.bw("laptop-lid", "top", EH, "body", 0.4, (0.95, 1.2), (0.729, 0.74), (-0.15, 0.2), corner=0.003, region="laptop-lid")
c.dec("Each T-foot is a 0.50 m stem along x crossed by a 0.60 m bar (16 mm thick, rounded) instead of a solid plate: both silhouettes agree with the drawn plate in side and front view while the plan stays covered by the top, as drawn.",
      "Jeder T-Fuss ist ein 0,50 m langer Steg in x, gekreuzt von einem 0,60 m langen Balken (16 mm stark, gerundet) statt einer Vollplatte: beide Silhouetten stimmen in Seiten- und Vorderansicht mit der gezeichneten Platte ueberein, waehrend der Grundriss wie gezeichnet vom Tisch verdeckt bleibt.")
c.dec("Left and right columns are separate bones (sleeve on the foot, stage on stage-*, column on column-*) with the drives placed diagonally (left drive at z -0.1, right drive at z +0.1) as the front view draws them; the stage and column move with the prismatic joints so the top rises 0.43 m in unison.",
      "Linke und rechte Saeule sind getrennte Knochen (Huelse am Fuss, Stufe auf stage-*, Saeule auf column-*) mit diagonal angeordneten Antrieben (links z -0,1, rechts z +0,1) wie in der Vorderansicht; Stufe und Saeule folgen den Schubgelenken, sodass sich die Platte gemeinsam um 0,43 m hebt.")
c.dec("The cross rail is drawn 25 mm below the top and 0.25 m behind the columns; two small steel mounts (added, not drawn) tie it to the top underside so it does not float, and the cable tray is a three-plate steel trough under the rail.",
      "Die Querschiene ist 25 mm unter der Platte und 0,25 m hinter den Saeulen gezeichnet; zwei kleine Stahlkonsolen (ergaenzt, nicht gezeichnet) verbinden sie mit der Plattenunterseite, damit sie nicht schwebt, und die Kabelwanne ist eine dreiteilige Stahlrinne unter der Schiene.")
c.write()
