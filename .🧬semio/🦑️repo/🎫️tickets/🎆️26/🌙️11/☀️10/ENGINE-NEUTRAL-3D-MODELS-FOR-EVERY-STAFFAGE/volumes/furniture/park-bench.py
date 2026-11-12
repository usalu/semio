import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
from authoring_furniture_lib import *
c = Ctx("park-bench")
SP, OAK = "steel-painted", "oak-timber"
def flare(t):
    return [(1 - t) ** 3 * 0.182 + 3 * (1 - t) ** 2 * t * 0.182 + 3 * (1 - t) * t * t * 0.148 + t ** 3 * 0.144, (1 - t) ** 3 * 0.012 + 3 * (1 - t) ** 2 * t * 0.034 + 3 * (1 - t) * t * t * 0.05 + t ** 3 * 0.13]
def hz(y): return 0.028 if y <= 0.04 else 0.028 - (y - 0.04) / 0.33 * 0.004
stations = [(0.0, 0.062), (0.012, 0.062)] + [(flare(t)[1], flare(t)[0] - 0.12) for t in (0.25, 0.5, 0.75)] + [(0.13, 0.024), (0.375, 0.026)]
for side, cx, bone in (("l", 0.12, "end-left"), ("r", 1.68, "end-right")):
    c.bw(f"{side}-foot", bone, SP, "body", 0.35, (cx - 0.062, cx + 0.062), (0, 0.05), (-0.3, 0.25), corner=0.01, region="steel-frame")
    secs = [([cx, y, -0.26], hz(y), hx, 3) for y, hx in stations]
    c.loft(f"{side}-leg", bone, SP, "body", 0.4, secs, Z, X, region="steel-frame", repeat={"count": 2, "translation": [0, 0, 0.45]})
    c.bw(f"{side}-brace", bone, SP, "detail", 0.5, (cx - 0.03, cx + 0.03), (0.19, 0.212), (-0.26, 0.19), corner=0.004, region="steel-frame")
    c.bw(f"{side}-bearer", bone, SP, "body", 0.45, (cx - 0.04, cx + 0.04), (0.37, 0.406), (-0.29, 0.22), corner=0.004, region="steel-frame")
    c.cap(f"{side}-post", bone, SP, "body", 0.4, [cx, 0.4, 0.19], [cx, 0.8285, 0.2797], 0.0155, 0.0135, region="steel-frame")
    c.ell(f"{side}-finial", bone, SP, "body", 0.38, [cx, 0.8325, 0.2805], [0.018, 0.0175, 0.0185], region="steel-frame")
    c.bw(f"{side}-arm-post", bone, SP, "detail", 0.45, (cx - 0.016, cx + 0.016), (0.4, 0.6), (-0.276, -0.244), corner=0.004, region="steel-frame")
    c.bw(f"{side}-armrest", bone, SP, "body", 0.42, (cx - 0.045, cx + 0.045), (0.6, 0.634), (-0.3, 0.14), corner=0.013, region="steel-frame")
c.bw("seat-slat", "seat", OAK, "body", 0.6, (0, 1.8), (0.405, 0.45), (-0.3, -0.18125), corner=0.008, region="seat-timber", repeat={"count": 4, "translation": [0, 0, 0.12375]})
c.ext("back-slat", "backrest", OAK, "body", 0.55, [(0.52, 0.1701), (0.52, 0.2031), (0.6, 0.2199), (0.6, 0.1869)], "yz", 0.0, 1.8, region="back-timber", repeat={"count": 3, "translation": [0, 0.11, 0.0231]})
for i, (y0, z0) in enumerate(((0.56, 0.2235), (0.67, 0.2466), (0.78, 0.2697)), 1):
    for tag, xs in (("a", (0.105, 0.135)), ("b", (1.665, 1.695))):
        c.rev(f"back-slat-{i}-bolt-{tag}", "backrest", SP, "detail", 0.15, [[0, 0.004], [0.007, 0.004]], Z, [xs[0], y0, z0 - 0.0], region="steel-frame", maxD=50, repeat={"count": 2, "translation": [0.03, 0, 0]})
c.dec("Each end frame is a 0.55 m long cast foot runner carrying two legs (flared at the foot with the drawn bezier profile, loft with repeat for the pair 0.45 m apart), a cross brace, a seat bearer, a rear post inclined 12 degrees backwards with a ball finial, and a front arm post with an armrest; left and right frames are separate bones.",
      "Jeder Endrahmen ist eine 0,55 m lange gusseiserne Fusskufe mit zwei Beinen (unten mit dem gezeichneten Bezierprofil aufgeweitet, Loft mit repeat fuer das 0,45 m entfernte Paar), einer Querstrebe, einem Sitztraeger, einem 12 Grad nach hinten geneigten hinteren Pfosten mit Kugelabschluss und einem vorderen Armlehnenpfosten mit Armlehne; linker und rechter Rahmen sind eigene Knochen.")
c.dec("The seat is four oak slats of 119 mm with 5 mm gaps (the 2D draws one solid 0.49 m board), the backrest three oak slats that are exact parallelogram extrusions of the front-view outline and repeat with a 0.11 m vertical and 0.0231 m backward step; bolt heads are 8 mm discs on the slat faces.",
      "Die Sitzflaeche besteht aus vier Eichenlatten zu 119 mm mit 5 mm Fugen (das 2D zeichnet ein einziges 0,49 m breites Brett), die Lehne aus drei Eichenlatten als exakte Parallelogramm-Extrusionen des Vorderansicht-Umrisses mit 0,11 m Hoehen- und 0,0231 m Rueckversatz je Latte; Schraubenkoepfe sind 8-mm-Scheiben auf den Lattenflaechen.")
c.dec("The ground anchor bolts, web rims, scroll relief and chamfer lines of the 1:50 drawing are surface details below 10 mm and are not modelled.",
      "Die Bodenanker, Stegraender, Schneckenreliefs und Fasenlinien der 1:50-Zeichnung sind Oberflaechendetails unter 10 mm und nicht modelliert.")
c.write()
