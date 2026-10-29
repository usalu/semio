import sys; sys.path.insert(0, ".")
from lib import *
doc = load("objects"); a = asset(doc, "park-bench")
SF, MS = "steel-frame", "steel-painted"
ST, MT = "seat-timber", "oak-timber"; BT = "back-timber"
g = G(); c = 0.12
for side, bone in (("l", "end-left"), ("r", "end-right")):
    def P(s): return s
    # foot flare, tapered leg, seat bearer, back post with finial, armrest, mouldings
    g.add(f"{side}-foot", bone, "body", 0, 2, path(f"M {c-0.062:.4f} 0 L {c+0.062:.4f} 0 L {c+0.062:.4f} 0.012 C {c+0.062:.4f} 0.034 {c+0.028:.4f} 0.05 {c+0.024:.4f} 0.13 L {c-0.024:.4f} 0.13 C {c-0.028:.4f} 0.05 {c-0.062:.4f} 0.034 {c-0.062:.4f} 0.012 Z"), SF, MS, tone=0.35)
    g.add(f"{side}-leg", bone, "body", 0, 3, path(f"M {c-0.024:.4f} 0.12 L {c+0.024:.4f} 0.12 C {c+0.022:.4f} 0.22 {c+0.020:.4f} 0.30 {c+0.026:.4f} 0.375 L {c-0.026:.4f} 0.375 C {c-0.020:.4f} 0.30 {c-0.022:.4f} 0.22 {c-0.024:.4f} 0.12 Z"), SF, MS, tone=0.4)
    g.add(f"{side}-bearer", bone, "body", 0, 2.5, rect(c - 0.04, 0.37, 0.08, 0.036, 0.004), SF, MS, tone=0.45)
    g.add(f"{side}-back", bone, "body", 0, 3, cap(c, 0.40, c, 0.84, 0.0155, 0.0135), SF, MS, tone=0.4)
    g.add(f"{side}-finial", bone, "body", 1, 3.1, ell(c, 0.852, 0.0185, 0.0175), SF, MS, tone=0.38)
    g.add(f"{side}-armrest", bone, "body", 2, 5, rect(c - 0.045, 0.600, 0.09, 0.034, 0.013), SF, MS, tone=0.42)
    g.add(f"{side}-arm-post", bone, "detail", 2, 4.9, rect(c - 0.016, 0.40, 0.032, 0.20), SF, MS, tone=0.45)
    g.add(f"{side}-arm-scroll", bone, "detail", 2, 5.1, path(f"M {c-0.04:.4f} 0.612 C {c-0.04:.4f} 0.585 {c-0.016:.4f} 0.58 {c-0.016:.4f} 0.55"), SF, MS, filled=False, tone=0.3)
    g.add(f"{side}-brace", bone, "detail", 1, 2.2, rect(c - 0.03, 0.19, 0.06, 0.022, 0.004), SF, MS, tone=0.5)
    g.add(f"{side}-web-rim", bone, "detail", 2, 3.2, path(f"M {c-0.012:.4f} 0.15 L {c-0.011:.4f} 0.36 M {c+0.012:.4f} 0.15 L {c+0.011:.4f} 0.36"), SF, MS, filled=False, tone=0.3)
    g.add(f"{side}-foot-bead", bone, "detail", 2, 3.2, poly([(c - 0.062, 0.016), (c + 0.062, 0.016)], False), SF, MS, filled=False, tone=0.3)
    for k, dx in enumerate((-0.05, 0.05)):
        g.add(f"{side}-foot-bolt-{k}", bone, "detail", 3, 3.3, ell(c + dx, 0.008, 0.0045, 0.0045), SF, MS, tone=0.15)
    for k, y in enumerate((0.388,)):
        g.add(f"{side}-bearer-bolt", bone, "detail", 3, 3.3, ell(c, y, 0.0045, 0.0045), SF, MS, tone=0.15)
# seat: front slat with rounded nosing, second slat edge line, end grain, screws
g.add("seat-slats", "seat", "body", 0, 4, rect(0, 0, 1.8, 0.045, 0.01), ST, MT, tone=0.6)
g.add("seat-gap", "seat", "detail", 1, 4.1, poly([(0.0, 0.0225), (1.8, 0.0225)], False), ST, MT, filled=False, tone=0.3)
g.add("seat-nosing", "seat", "detail", 2, 4.15, poly([(0.012, 0.040), (1.788, 0.040)], False), ST, MT, filled=False, tone=0.3)
g.add("seat-grain", "seat", "texture", 2, 4.2, path("M 0.20 0.011 C 0.5 0.016 0.8 0.008 1.1 0.013 C 1.35 0.016 1.55 0.010 1.62 0.012 M 0.30 0.033 C 0.6 0.030 0.9 0.036 1.2 0.032"), ST, MT, filled=False, tone=0.3)
for k, x in enumerate((0.09, 0.15, 1.65, 1.71)):
    g.add(f"seat-screw-{k}", "seat", "detail", 3, 4.3, ell(x, 0.0225, 0.004, 0.004), SF, MS, tone=0.15)
# backrest: three slats with rounded edges
for i, (y, h) in enumerate(((-0.10, 0.08), (0.01, 0.08), (0.12, 0.08)), 1):
    g.add(f"back-slat-{i}", "backrest", "body", 0, 4, rect(0, y, 1.8, h, 0.012), BT, MT, tone=0.55)
    g.add(f"back-slat-{i}-edge", "backrest", "detail", 2, 4.1, poly([(0.012, y + h - 0.012), (1.788, y + h - 0.012)], False), BT, MT, filled=False, tone=0.3)
    g.add(f"back-slat-{i}-grain", "backrest", "texture", 2, 4.2, path(f"M 0.25 {y+0.030:.3f} C 0.6 {y+0.036:.3f} 1.0 {y+0.026:.3f} 1.5 {y+0.032:.3f}"), BT, MT, filled=False, tone=0.3)
    for k, x in enumerate((0.105, 0.135, 1.665, 1.695)):
        g.add(f"back-slat-{i}-screw-{k}", "backrest", "detail", 3, 4.3, ell(x, y + 0.04, 0.0038, 0.0038), SF, MS, tone=0.15)
a["geometry"]["elevation"] = g.items
B = {b["id"]: b for b in a["rig"]["bones"]}
for k in ("end-left", "end-right"):
    B[k]["mass"]["comX"] = 0.12; B[k]["lengthM"] = 0.24
B["backrest"]["y"] = 0.62
for an in a["anchors"]:
    an["x"] = 0.12; an["footprintLengthM"] = 0.12; an["footprintWidthM"] = 0.30
a["summary"] = T("A long oak slat seat and backrest on two painted cast-style steel end frames with armrests; load states are occupied places on the seat. Front elevation: the length runs along x, the end frames are seen in front view.", "Lange Sitz- und Rückenlehne aus Eichenlatten auf zwei lackierten Stahl-Endgestellen in Gussoptik mit Armlehnen; Lastzustände sind besetzte Plätze auf der Sitzfläche. Vorderansicht: Die Länge verläuft entlang x, die Endgestelle sind in Vorderansicht zu sehen.")
save("objects", doc)
