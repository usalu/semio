import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
from authoring_furniture_lib import *
c = Ctx("folding-deck-chair")
BE, CV, HW = "beech-timber", "cotton-canvas", "steel-hardware"
RR = 0.0045
c.lbox("rear", "rear", BE, "body", 0.55, (0.025, 1.045), (-0.0225, 0.0225), (0.2475, 0.2925), corner=RR, region="frame-timber", repeat={"count": 2, "translation": [0, 0, -0.54]})
c.lbox("front", "front", BE, "body", 0.55, (0.025, 0.726), (-0.0225, 0.0225), (0.2125, 0.2575), corner=RR, region="frame-timber", repeat={"count": 2, "translation": [0, 0, -0.47]})
c.lbox("seat", "seat", BE, "body", 0.55, (0.0, 0.495), (-0.0225, 0.0225), (0.1575, 0.2025), corner=RR, region="frame-timber", repeat={"count": 2, "translation": [0, 0, -0.36]})
c.lrev("bolt", "rear", HW, "body", 0.2, [[0, 0.0125], [0.035, 0.0125]], Z, [0.726, 0, 0.2575], region="frame-timber", maxD=100, repeat={"count": 2, "translation": [0, 0, -0.55]})
c.lcap("top-rail", "rear", BE, "body", 0.4, [1.045, 0, -0.27], [1.045, 0, 0.27], 0.0125, region="frame-timber", maxD=100)
c.lcap("front-rail", "seat", BE, "body", 0.4, [0.495, 0, -0.18], [0.495, 0, 0.18], 0.0125, region="frame-timber", maxD=100)
c.lbox("canvas-back", "canvas-a", CV, "body", 0.8, (0, 0.6), (-0.0025, 0.0025), (-0.2, 0.2), region="canvas-back")
c.lbox("canvas-seat", "canvas-b", CV, "body", 0.65, (0, 0.45), (-0.0025, 0.0025), (-0.2, 0.2), region="canvas-seat")
c.dec("The three hinged frames are one rounded 45 by 45 mm beech batten per side on the rear, front and seat bones; each side pair is one solid with repeat (laterally 0.54, 0.47 and 0.36 m apart), the rear frames outermost, so the scissor frames keep their real nesting when the chair folds.",
      "Die drei Gelenkrahmen sind je Seite eine gerundete Buchenlatte 45 mal 45 mm auf Hinter-, Vorder- und Sitzknochen; jedes Seitenpaar ist ein Koerper mit repeat (seitlich 0,54, 0,47 und 0,36 m auseinander), die hinteren Rahmen aussen, damit die Scherenrahmen beim Zusammenklappen ihre echte Verschachtelung behalten.")
c.dec("The sling is two 5 mm canvas panels, 0.40 m wide, on the two canvas bones (0.60 m back panel hung from the top rail, 0.45 m seat panel from the front rail), the 4 pose-solved bones of the asset follow every fold; top rail, front rail and the two pivot bolts are dowel capsules and a cylinder revolve.",
      "Die Bespannung besteht aus zwei 5 mm starken, 0,40 m breiten Stoffbahnen auf den beiden Stoffknochen (0,60 m Rueckenbahn an der oberen Traverse, 0,45 m Sitzbahn an der vorderen Traverse), die gemaess Pose gerechneten Knochen folgen jedem Zusammenklappen; obere und vordere Traverse sowie die beiden Drehbolzen sind Rundholz-Kapseln und ein Rotationszylinder.")
c.dec("Authored hints z0/z1 are lateral sweeps; the live catalog lacks the mandatory sweep and evaluates them as sweep v, so front and plan are only meaningful with sweep w (see authoring-requests.md).",
      "Die Hinweise z0/z1 sind seitliche Extrusionen; im Katalog fehlt das verpflichtende sweep und sie werden als sweep v ausgewertet, Vorder- und Draufsicht sind daher nur mit sweep w aussagekraeftig (siehe authoring-requests.md).")
c.write()
