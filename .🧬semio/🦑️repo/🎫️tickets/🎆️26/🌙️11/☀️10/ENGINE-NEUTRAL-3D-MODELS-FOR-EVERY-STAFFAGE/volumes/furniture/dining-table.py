import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
from authoring_furniture_lib import *
c = Ctx("dining-table")
OAK, ST = "oak-timber", "steel-slides"
def octagon(x0, x1, z0, z1, ch):
    return [(x0, z0 + ch), (x0 + ch, z0), (x1 - ch, z0), (x1, z0 + ch), (x1, z1 - ch), (x1 - ch, z1), (x0 + ch, z1), (x0, z1 - ch)]
for side, bone, x0, x1 in (("l", "top-left", 0.0, 0.8), ("r", "top-right", 0.8, 1.6)):
    c.ext(f"top-{side}", bone, OAK, "body", 0.7, octagon(x0, x1, -0.45, 0.45, 0.008), "xz", 0.72, 0.75, region="table-top")
    a0, a1 = (0.1, 0.8) if side == "l" else (0.8, 1.5)
    c.box(f"apron-{side}-side", bone, OAK, "body", 0.55, [(a0 + a1) / 2, 0.68, 0.37], [a1 - a0, 0.08, 0.02], region="apron", repeat={"count": 2, "translation": [0, 0, -0.74]})
    e0 = a0 + 0.01 if side == "l" else a0 + 0.01
    c.box(f"apron-{side}-end", bone, OAK, "body", 0.55, [a0 + 0.01, 0.68, 0], [0.02, 0.08, 0.72], region="apron", repeat={"count": 2, "translation": [a1 - a0 - 0.02, 0, 0]})
    cx = 0.135 if side == "l" else 1.465
    secs = [([cx, 0.0, 0.375], 0.025, 0.0225, 8), ([cx, 0.56, 0.375], 0.035, 0.035, 8), ([cx, 0.72, 0.375], 0.035, 0.035, 8)]
    c.loft(f"leg-{side}", bone, OAK, "body", 0.6, secs, Z, X, region="legs", repeat={"count": 2, "translation": [0, 0, -0.75]})
c.box("leaf", "leaf", OAK, "body", 0.65, [0.8, 0.705, 0], [0.6, 0.03, 0.88], region="leaf-top")
c.box("slide-rail", "frame", ST, "body", 0.35, [0.8, 0.69, -0.25], [1.0, 0.06, 0.04], region="slide-rails", repeat={"count": 2, "translation": [0, 0, 0.5]})
c.dec("Each half of the table is its own bone: a 30 mm oak top (octagonal plan with 8 mm chamfered corners, as the plan draws), a hollow four-rail apron of 20 mm boards, and two tapered legs, so the halves slide apart carrying their aprons and legs.",
      "Jede Tischhaelfte hat einen eigenen Knochen: eine 30 mm starke Eichenplatte (achteckiger Grundriss mit 8 mm gefasten Ecken wie im Grundriss), eine hohle Vier-Brett-Zarge aus 20 mm Brettern und zwei verjuengte Beine, sodass die Haelften samt Zarge und Beinen auseinandergleiten.")
c.dec("The four legs are 70 mm square lofts tapering below the shoulder at 0.56 m to 45 by 50 mm at the foot; they stand outboard of the apron (z = +-0.34..0.41) and rise to the top underside (the front view stops them at the apron, the elevation at the top).",
      "Die vier Beine sind 70-mm-Quadrat-Lofts, die sich unter der Schulter bei 0,56 m auf 45 mal 50 mm am Fuss verjuengen; sie stehen ausserhalb der Zarge (z = +-0,34..0,41) und reichen bis zur Plattenunterseite (die Vorderansicht endet am Zargenunterrand, die Ansicht an der Platte).")
c.dec("The 0.60 m leaf is a 30 mm box on its own prismatic bone hidden beneath the closed halves (x 0.5..1.1, overlapping each by 0.3 m) and lifts flush into the gap; two steel slide rails under the top hold the halves.",
      "Die 0,60 m lange Einlegeplatte ist eine 30 mm starke Box auf eigenem Schubknochen, im geschlossenen Zustand unter den Haelften verborgen (x 0,5..1,1, je 0,3 m ueberlappend), und hebt sich buendig in die Luecke; zwei Stahl-Auszugsschienen unter der Platte halten die Haelften.")
c.write()
