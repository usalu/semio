import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

T = 0.0025
outer = [(0.017, 0.0)] + bezier((0.017, 0), (0.06, 0), (0.11, -0.03), (0.13, -0.133), n=12) + [(0.133, -0.133), (0.133, -0.1385)]
outer = [(y, x) for x, y in outer]
inner_tail = [(-0.1385, 0.1305)]
curve = [(y, x) for x, y in bezier((0.017, 0), (0.06, 0), (0.11, -0.03), (0.13, -0.133), n=12, first=True)]


def inward(points):
    out = []
    for i, (a, b) in enumerate(points):
        p0, p1 = points[max(0, i - 1)], points[min(len(points) - 1, i + 1)]
        ta, tb = p1[0] - p0[0], p1[1] - p0[1]
        n = math.hypot(ta, tb)
        na, nb = tb / n, -ta / n
        if nb > 0:
            na, nb = -na, -nb
        out.append((a + na * T, b + nb * T))
    return out


shade_profile = outer + [(-0.1385, 0.1305)] + list(reversed(inward(curve)))

glass = [(0.0, 0.0)] + [(y, x) for x, y in bezier((0, 0), (0.014, 0), (0.03, 0.014), (0.03, 0.032), n=8)] + [(y, x) for x, y in bezier((0.03, 0.032), (0.03, 0.046), (0.023, 0.054), (0.0185, 0.063), n=6)] + [(y, x) for x, y in bezier((0.0185, 0.063), (0.016, 0.069), (0.0143, 0.073), (0.0138, 0.078), n=4)]
glass = glass[1:] if glass[0] == glass[1] else glass

rose = [(-0.024, 0.05), (-0.016, 0.05)] + [(y, x) for x, y in bezier((0.05, -0.016), (0.05, -0.006), (0.042, 0), (0.032, 0), n=6)]

S = [
    revolve("rose", "rose", "rose-polymer", "body", 0.7, "rose", rose, axis=(0, 1, 0), maxd=200),
    revolve("cord", "cord", "textile-cord", "body", 0.15, "cord", [(-0.521, 0.0033), (0, 0.0033)], axis=(0, 1, 0), maxd=200),
    revolve("collar", "holder", "brass-lacquered", "body", 0.45, "lampholder", [(0, 0.01), (-0.006, 0.0105), (-0.0075, 0.0155), (-0.03, 0.0155)], axis=(0, 1, 0), maxd=100),
    revolve("shade", "shade", "enamel-white", "body", 0.4, "shade", shade_profile, axis=(0, 1, 0), maxd=200),
    revolve("bulb-cap", "bulb", "brass-lacquered", "body", 0.5, "lampholder", [(0.078, 0.0135), (0.105, 0.0135)], axis=(0, 1, 0), maxd=100),
    revolve("bulb-glass", "bulb", "bulb-glass", "body", 0.9, "bulb-glass", glass, axis=(0, 1, 0), maxd=200),
]
D = [dec("Every part is axisymmetric about the vertical axis (plan views are circles of 50 mm and 133 mm), so all six solids are revolves of their elevation half outline.",
         "Jedes Teil ist rotationssymmetrisch um die Hochachse (die Grundrisse sind Kreise von 50 mm und 133 mm), daher sind alle sechs Körper Rotationskörper der halben Seitenansichtskontur."),
     dec("The enamel shade is a hollow shell of 2.5 mm wall, an open revolve profile that runs down the outside and back up the inside, whose outer envelope equals the drawn dome, so the bulb hangs visibly inside; the drawn closed dome silhouette is unchanged.",
         "Der Emailleschirm ist eine hohle Schale von 2,5 mm Wandstärke, ein Rotationskörper, dessen offenes Profil außen hinab und innen hinauf läuft und dessen Außenhülle der gezeichneten Kuppel entspricht, sodass die Glühlampe sichtbar darin hängt; die Silhouette bleibt unverändert."),
     dec("Bulb cap, cap thread, filament and the hairline shade highlights have no separate material or region; only the cap cylinder is modelled, the filament and the threads are left out.",
         "Fassung, Gewinde, Glühfaden und die Haarlinien-Glanzlichter des Schirms haben kein eigenes Material oder keine eigene Region; nur der Kappenzylinder wird modelliert, Glühfaden und Gewinde entfallen.")]
write("pendant-lamp", S, D)
