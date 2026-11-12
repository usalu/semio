import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
from authoring_furniture_lib import *
c = Ctx("operating-table")
PC, SS, RC, FOAM = "powder-coated-steel", "stainless-steel", "rubber-castor", "pad-foam"
def wheel(r, w):
    k = w / 2
    return [[-k, r * 0.75], [-k * 0.8, r * 0.94], [-k * 0.4, r], [k * 0.4, r], [k * 0.8, r * 0.94], [k, r * 0.75]]
c.loft("tb-plate", "base", PC, "body", 0.5, [([0, 0.07, 0], 0.2, 0.4, 12), ([0, 0.12, 0], 0.2, 0.4, 12), ([0, 0.16, 0], 0.15, 0.3, 12)], Z, X, region="table-base")
for tag, x in (("a", -0.32), ("b", 0.32)):
    c.lrev(f"tb-castor-{tag}", "base", RC, "body", 0.2, wheel(0.04, 0.04), Z, [x, 0.04, 0.15], region="table-base", repeat={"count": 2, "translation": [0, 0, -0.34]})
c.loft("col-lower", "column", SS, "body", 0.6, [([0, 0.16, 0], 0.15, 0.15, 8), ([0, 0.4, 0], 0.13, 0.13, 8)], Z, X, region="table-column")
c.bw("col-upper", "column", SS, "body", 0.7, (-0.105, 0.105), (0.4, 0.74), (-0.105, 0.105), corner=0.004, region="table-column")
c.bw("col-head", "column", SS, "body", 0.5, (-0.16, 0.16), (0.74, 0.78), (-0.2, 0.2), corner=0.004, region="table-frame")
c.bw("top-carrier", "top", SS, "body", 0.55, (-0.98, 0.98), (0.78, 0.82), (-0.2, 0.2), corner=0.004, region="table-frame")
for tag, x0, x1 in (("head", -1.0, -0.622), ("back", -0.618, -0.052), ("seat", -0.048, 0.198), ("leg", 0.202, 1.0)):
    c.bw(f"top-pad-{tag}", "top", FOAM, "body", 0.3, (x0, x1), (0.82, 0.9), (-0.25, 0.25), corner=0.03, region="table-pad")
c.loft("st-base", "stand-base", PC, "body", 0.45, [([1.17, 0.06, 0], 0.25, 0.28, 12), ([1.17, 0.105, 0], 0.22, 0.24, 12)], Z, X, region="stand")
for tag, x in (("a", -0.22), ("b", 0.22)):
    c.lrev(f"st-castor-{tag}", "stand-base", RC, "body", 0.2, wheel(0.03, 0.03), Z, [x, 0.03, 0.185], region="stand", repeat={"count": 2, "translation": [0, 0, -0.37]})
c.rev("st-column", "stand-column", PC, "body", 0.5, [[0.105, 0.03], [1.88, 0.03]], Y, [1.17, 0, 0], region="stand")
c.rev("st-collar", "stand-column", PC, "body", 0.4, [[1.88, 0.045], [1.96, 0.045]], Y, [1.17, 0, 0], region="stand")
c.bw("st-arm", "stand-arm", PC, "body", 0.5, (0.67, 1.17), (1.9, 1.94), (-0.02, 0.02), corner=0.005, region="stand")
c.ell("st-knob", "stand-arm", PC, "body", 0.35, [0.67, 1.92, 0], [0.03, 0.03, 0.03], region="stand")
c.rev("st-rod", "stand-arm", PC, "body", 0.5, [[1.78, 0.012], [1.9, 0.012]], Y, [0.67, 0, 0], region="stand")
half = [[1.675, 0.25], [1.70, 0.25]] + [[1.70 + y, x] for x, y in [(p[0], p[1] - 0.04) for p in [((1 - t) ** 3 * 0.25 + 3 * (1 - t) ** 2 * t * 0.25 + 3 * (1 - t) * t * t * 0.12, (1 - t) ** 3 * 0.04 + 3 * (1 - t) ** 2 * t * 0.1 + 3 * (1 - t) * t * t * 0.12 + t ** 3 * 0.12) for t in (0.2, 0.4, 0.6, 0.8, 1.0)]]]
half[-1][1] = 0.0
c.rev("head-housing", "light-head", "light-aluminium", "body", 0.35, half, Y, [0.67, 0, 0], region="light-housing")
c.rev("head-lens", "light-head", "light-lens-glass", "body", 0.9, [[1.66, 0.24], [1.675, 0.24]], Y, [0.67, 0, 0], region="light-lens")
c.rev("head-handle", "light-head", "light-aluminium", "body", 0.4, [[1.6, 0.02], [1.66, 0.02]], Y, [0.67, 0, 0], region="light-housing")
c.dec("The table is a stainless column (tapered lower loft 0.30 m square, 0.21 m upper box) carrying a 0.40 m wide carrier and a 0.50 m wide foam top that is cut into four sections (head, back, seat, leg, 4 mm gaps) at the pad joints the 2D draws, all on the revolute top bone so the whole top tilts about the column head.",
      "Der Tisch hat eine Edelstahlsaeule (verjuengtes unteres Loft 0,30 m im Quadrat, 0,21 m oberer Kasten), die einen 0,40 m breiten Traeger und eine 0,50 m breite Schaumstoffliege traegt, die an den im 2D gezeichneten Polstergelenken in vier Abschnitte (Kopf, Ruecken, Sitz, Bein, 4 mm Fugen) geteilt ist, alles auf dem Drehknochen top, sodass die ganze Liege um den Saeulenkopf kippt.")
c.dec("Both base plates are four-sided frustum lofts (table 0.80 by 0.40 m, stand 0.56 by 0.50 m, as the elevation and front outlines give) on four castors each (two repeat solids per castor row); castor wheels are revolves with axis along z, 80 mm and 60 mm in diameter.",
      "Beide Fussplatten sind vierseitige Pyramidenstumpf-Lofts (Tisch 0,80 mal 0,40 m, Staender 0,56 mal 0,50 m, wie Ansicht und Vorderansicht es ergeben) mit je vier Rollen (zwei repeat-Koerper je Rollenreihe); die Rollen sind Rotationskoerper mit Achse in z, Durchmesser 80 mm und 60 mm.")
c.dec("The lamp stand is a round 60 mm column on a plate, a 90 mm collar, a 0.5 m horizontal arm with a knob, a vertical rod and a 0.50 m dome head (revolve of the drawn dome profile) with flat lens disc and a handle, since the plan draws the head and collar as circles; pleat lines on the table column and the head groove are not modelled.",
      "Der Lampenstaender ist eine runde 60-mm-Saeule auf einer Platte, ein 90-mm-Kragen, ein 0,5 m langer waagrechter Arm mit Knauf, eine senkrechte Stange und ein 0,50 m grosser Kuppelkopf (Rotation des gezeichneten Kuppelprofils) mit flacher Linsenscheibe und Griff, da der Grundriss Kopf und Kragen als Kreise zeigt; die Faltenlinien an der Tischsaeule und die Kopfrille sind nicht modelliert.")
c.write()
