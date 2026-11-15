import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())
P = "panel"
GL, ST = "glass", "steel"
sc = []

# ---- 1:50
s = Scale(50)
shoe = [[0, 0.15], [0, -0.15], [0.1, -0.15], [0.1, -0.0275], [0.02, -0.0275], [0.02, 0.0275], [0.1, 0.0275], [0.1, 0.15]]
s.b(ext("shoe", P, ST, 0.35, "xy", shoe, -1, 1))
s.b(box("gasket-a", P, ST, 0.2, [0.08, 0.0165, 2], [0.06, 0.01912, 0]))
s.b(box("gasket-b", P, ST, 0.2, [0.08, 0.0165, 2], [0.06, -0.01912, 0]))
s.b(box("glass", P, GL, 0.92, [1.06, 0.0215, 1.96], [0.55, 0, 0], role="glazing"))
s.b(cyl("clamp-bolts", P, ST, 0.15, "y", [0.05, 0, -0.8], 0.3, 0.007, role="detail", repeat={"count": 5, "translation": [0, 0, 0.4]}))
s.b(cyl("handrail", P, ST, 0.35, "z", [1.079, 0, 0], 1.992, 0.0212))
s.b(ellipsoid("handrail-cap", P, ST, 0.35, [0.0212, 0.0212, 0.004], [1.079, 0, 0.996], role="detail", repeat={"count": 2, "translation": [0, 0, -1.992]}))
for i, o in (("shoe", "shoe"), ("gasket-a", "gasket-a"), ("gasket-b", "gasket-b"), ("bolts", "clamp-bolts")):
    s.thof(f"th-clamp-{i}", o, "clamp-steel")
s.thof("th-glass", "glass", "panel-glass")
s.thof("th-handrail", "handrail", "handrail-steel")
s.thof("th-handrail-cap", "handrail-cap", "handrail-steel")
s.stof("glass-member", "glass", "panel", "member")
s.stof("handrail-member", "handrail", "panel", "member")
s.stof("clamp-bolts-joint", "clamp-bolts", "panel", "joint")
s.st(box("support-base-clamp-block", P, ST, 0.15, [0.02, 0.3, 2], [0.01, 0, 0]), "panel", "support")
s.dec("Body: the clamp shoe as the U-channel of the drawing extruded 2 m (base 20 mm, cheeks to 100 mm, 55 mm slot), two 16.5 mm gaskets in the slot, the 21.5 mm laminated glass 1.96 m long from the slot to 1.08 m, five 14 mm through-bolts across the cheeks (the drawing shows them as circles in elevation and squares in front; modelled across the shoe), the 42.4 mm handrail tube with domed end caps.",
      "Körper: der Klemmschuh als U-Profil der Zeichnung 2 m lang extrudiert (Boden 20 mm, Wangen bis 100 mm, Schlitz 55 mm), zwei Dichtungen von 16,5 mm im Schlitz, das Verbundglas 21,5 mm, 1,96 m lang, vom Schlitz bis 1,08 m, fünf Durchgangsschrauben von 14 mm quer durch die Wangen (die Zeichnung zeigt sie im Aufriss als Kreise und von vorn als Quadrate; quer durch den Schuh modelliert), das Handlaufrohr 42,4 mm mit gewölbten Endkappen.")
s.dec("Thermal: every region is referenced from the body (clamp steel = shoe, gaskets and bolts; glass; handrail with its caps): the surfaces are exactly those body surfaces.",
      "Thermisch: jede Region wird aus dem Körper referenziert (Klemmstahl = Schuh, Dichtungen und Schrauben; Glas; Handlauf mit Kappen): die Flächen sind genau diese Körperflächen.")
s.dec("Structure: the laminated glass is the member (1 m strip 21.5 mm), the handrail ties the panel as a second member, the five clamp bolts are the joint between glass and shoe, the fixed support is the 20 mm base block of the shoe under the glass at the clamp anchor; no joints (the root is fixed).",
      "Tragwerk: das Verbundglas ist der Stab (Streifen 1 m, 21,5 mm), der Handlauf bindet die Scheibe als zweiter Stab, die fünf Klemmschrauben sind das Gelenk zwischen Glas und Schuh, das feste Auflager ist der Bodenblock von 20 mm des Schuhs unter dem Glas am Klemmanker; keine Gelenke (die Wurzel ist fest).")
sc.append(s)

# ---- 1:100
s = Scale(100)
s.b(box("shoe", P, ST, 0.35, [0.1, 0.3, 2], [0.05, 0, 0]))
s.b(box("glass", P, GL, 0.92, [0.98, 0.0215, 1.96], [0.59, 0, 0], role="glazing"))
s.b(cyl("handrail", P, ST, 0.35, "z", [1.079, 0, 0], 2.0, 0.0212))
s.thof("th-clamp-shoe", "shoe", "clamp-steel")
s.thof("th-glass", "glass", "panel-glass")
s.thof("th-handrail", "handrail", "handrail-steel")
s.stof("glass-member", "glass", "panel", "member")
s.stof("handrail-member", "handrail", "panel", "member")
s.st(box("support-base-clamp-block", P, ST, 0.15, [0.03, 0.3, 2], [0.015, 0, 0]), "panel", "support")
s.dec("Body: shoe, gaskets and bolts are one 100 x 300 x 2000 mm steel block (the drawing shows a plain block), the glass starts at its top, the handrail is a plain 42.4 mm tube without caps; bolts and gaskets are below the 15 mm feature.",
      "Körper: Schuh, Dichtungen und Schrauben sind ein Stahlblock 100 x 300 x 2000 mm (die Zeichnung zeigt einen glatten Block), das Glas beginnt an seiner Oberseite, der Handlauf ist ein glattes Rohr 42,4 mm ohne Kappen; Schrauben und Dichtungen liegen unter der Einzelheit 15 mm.")
s.dec("Thermal: shoe block, glass and handrail referenced from the body. Structure: glass and handrail members as before, the bolts are merged into the shoe, the base block grows to 30 mm.",
      "Thermisch: Schuhblock, Glas und Handlauf aus dem Körper referenziert. Tragwerk: Glas- und Handlaufstab wie zuvor, die Schrauben sind im Schuh aufgegangen, der Bodenblock wächst auf 30 mm.")
sc.append(s)

# ---- 1:200
s = Scale(200)
s.b(box("shoe", P, ST, 0.35, [0.125, 0.3, 2], [0.0625, 0, 0]))
s.b(box("glass", P, GL, 0.92, [0.945, 0.0215, 2.0], [0.5975, 0, 0], role="glazing"))
import math
oct = [[1.0735 + 0.0265 * math.cos(math.pi / 8 + k * math.pi / 4), 0.0265 * math.sin(math.pi / 8 + k * math.pi / 4)] for k in range(8)]
s.b(ext("handrail", P, ST, 0.35, "xy", oct, -1, 1))
s.thof("th-clamp-shoe", "shoe", "clamp-steel")
s.thof("th-glass", "glass", "panel-glass")
s.thof("th-handrail", "handrail", "handrail-steel")
s.stof("glass-member", "glass", "panel", "member")
s.st(box("handrail-member", P, ST, 0.35, [0.053, 0.053, 2], [1.0735, 0, 0]), "panel", "member")
s.st(box("support-base-clamp-block", P, ST, 0.15, [0.045, 0.3, 2], [0.0225, 0, 0]), "panel", "support")
s.dec("Body: the 125 mm shoe block, the glass as a 21.5 mm sheet over the full 2 m (the glass side is flush with the shoe length in this drawing), the handrail enlarged from 42.4 to 53 mm as an octagonal prism (the drawn disc; a circle is not resolved at 30 mm), as drawn at 1:200.",
      "Körper: der Schuhblock 125 mm, das Glas als Scheibe von 21,5 mm über die vollen 2 m (die Glaskante schließt in dieser Zeichnung mit der Schuhlänge ab), der Handlauf von 42,4 auf 53 mm vergrößert als achteckiges Prisma (die gezeichnete Scheibe; ein Kreis wird bei 30 mm nicht aufgelöst), wie bei 1:200 gezeichnet.")
s.dec("Thermal: the three regions referenced. Structure: glass member and a square 53 mm handrail member (the octagon's hull), the base block 45 mm (1.5 features).",
      "Thermisch: die drei Regionen referenziert. Tragwerk: Glasstab und ein quadratischer Handlaufstab von 53 mm (die Hülle des Achtecks), der Bodenblock 45 mm (1,5 Einzelheiten).")
sc.append(s)
save("components", "glass-balustrade", sc)
