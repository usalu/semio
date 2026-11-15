import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())
F, I, SP, CR = "frame", "igu", "spandrel", "cradle"
AL, GL, SG, WO, ST = "aluminium", "glass", "spandrel-glass", "wool", "steel"
sc = []
TY = [(0.025, "base"), (0.875, "mid"), (3.275, "head")]
OUTERH = rect(0.7, 1.65, 1.4, 3.3)
HOLE_SP = rect(0.7, 0.45, 1.3, 0.8)
HOLE_IG = rect(0.7, 2.075, 1.3, 2.35)
LAY = [("outer", -0.075, 0.035, 0.4), ("break", -0.04, 0.03, 0.85), ("inner", -0.01, 0.085, 0.4)]

def lay_support(s, scale):
    d = {50: 0.016, 100: 0.02, 200: 0.03, 500: 0.075}[scale]
    s.st(cyl("support-base-guide-roller", F, ST, 0.4, "z", [0.7, d / 2, 0], 0.15, d / 2), "frame", "support")
    ph = {50: 0.008, 100: 0.015, 200: 0.03, 500: 0.075}[scale]
    top = 3.3
    s.st(ext("support-head-anchor-pin", F, ST, 0.4, "xy", [[0.64, top], [0.76, top], [0.76, top - ph], [0.72, top - ph], [0.7, top - ph * 2], [0.68, top - ph], [0.64, top - ph]], -0.05, 0.05), "frame", "support")

# ---- 1:50
s = Scale(50)
for nm, z0, th, tn in LAY:
    zc = z0 + th / 2
    s.b(box(f"mullion-{nm}", F, AL, tn, [0.05, 3.3, th], [0.025, 1.65, zc], repeat={"count": 2, "translation": [1.35, 0, 0]}))
    for xy, nmt in TY:
        s.b(box(f"transom-{nmt}-{nm}", F, AL, tn, [1.3, 0.05, th], [0.7, xy, zc]))
s.b(box("spandrel-enamel", SP, SG, 0.2, [1.3, 0.8, 0.006], [0.65, 0.4, -0.064], role="glazing"))
s.b(box("spandrel-wool", SP, WO, 0.9, [1.3, 0.8, 0.1], [0.65, 0.4, 0.006]))
s.b(box("spandrel-pan", SP, ST, 0.3, [1.3, 0.8, 0.0015], [0.65, 0.4, 0.05675], role="texture"))
s.b(box("pane-out", I, GL, 0.92, [1.3, 2.35, 0.006], [0.65, 1.175, -0.036], role="glazing"))
s.b(box("pane-in", I, GL, 0.92, [1.3, 2.35, 0.006], [0.65, 1.175, -0.014], role="glazing"))
for nm, sz, at in (("base", [1.3, 0.012, 0.016], [0.65, 0.006, -0.025]), ("head", [1.3, 0.012, 0.016], [0.65, 2.344, -0.025]), ("left", [0.012, 2.326, 0.016], [0.006, 1.175, -0.025]), ("right", [0.012, 2.326, 0.016], [1.294, 1.175, -0.025])):
    s.b(box(f"spacer-{nm}", I, GL, 0.5, sz, at, role="detail"))
# thermal
RIN = rect(0.65, 1.175, 1.17, 2.22)
RO = rect(0.65, 1.175, 1.3, 2.35)
s.th(ext("th-centre-pane-out", I, GL, 0.92, "xy", RIN, -0.039, -0.033, role="glazing"), "glass-centre", "pane-out")
s.th(ext("th-centre-cavity", I, GL, 0.95, "xy", RIN, -0.033, -0.017, role="glazing"), "glass-centre", "cavity")
s.th(ext("th-centre-pane-in", I, GL, 0.92, "xy", RIN, -0.017, -0.011, role="glazing"), "glass-centre", "pane-in")
s.th(ext("th-edge-pane-out", I, GL, 0.92, "xy", RO, -0.039, -0.033, holes=[RIN], role="glazing"), "glass-edge", "pane-out")
s.th(ext("th-edge-spacer-cavity", I, GL, 0.5, "xy", RO, -0.033, -0.017, holes=[RIN], role="detail"), "glass-edge", "spacer-cavity")
s.th(ext("th-edge-pane-in", I, GL, 0.92, "xy", RO, -0.017, -0.011, holes=[RIN], role="glazing"), "glass-edge", "pane-in")
s.th(box("th-spandrel-air", SP, WO, 0.95, [1.3, 0.8, 0.02], [0.65, 0.4, -0.051], role="glazing"), "spandrel", "air")
s.thof("th-spandrel-enamel", "spandrel-enamel", "spandrel", "enamel")
s.thof("th-spandrel-wool", "spandrel-wool", "spandrel", "wool")
s.thof("th-spandrel-pan", "spandrel-pan", "spandrel", "steel-pan")
s.thof("th-mullion-break", "mullion-break", "mullion", "break")
s.th(box("th-mullion-cap", F, AL, 0.08, [0.05, 3.3, 0.002], [0.025, 1.65, -0.074], role="texture", repeat={"count": 2, "translation": [1.35, 0, 0]}), "mullion", "cap")
for xy, nmt in TY:
    s.thof(f"th-transom-{nmt}-break", f"transom-{nmt}-break", "transom", "break")
    s.th(box(f"th-transom-{nmt}-cap", F, AL, 0.08, [1.3, 0.05, 0.002], [0.7, xy, -0.074], role="texture"), "transom", "cap")
# structure
s.st(box("member-mullion", F, AL, 0.4, [0.05, 3.3, 0.15], [0.025, 1.65, 0], repeat={"count": 2, "translation": [1.35, 0, 0]}), "frame", "member")
for xy, nmt in TY:
    s.st(box(f"member-transom-{nmt}", F, AL, 0.4, [1.3, 0.05, 0.15], [0.7, xy, 0]), "frame", "member")
s.st(box("member-igu", I, GL, 0.9, [1.3, 2.35, 0.034], [0.65, 1.175, -0.025]), "igu", "member")
s.stof("member-spandrel", "spandrel-wool", "spandrel", "member")
s.st(box("joint-cradle", CR, ST, 0.3, [0.06, 0.06, 0.03], [0.65, 0, -0.025]), "cradle", "joint")
for nm in ("base", "head", "left", "right"):
    s.stof(f"joint-igu-spacer-{nm}", f"spacer-{nm}", "igu", "joint")
lay_support(s, 50)
s.dec("Body: the bay as built: two mullions and three transoms in the drawn three layers (35 mm outer, 30 mm thermal break, 85 mm inner section), the spandrel as 6 mm enamel glass, 100 mm wool and the 1.5 mm steel pan, the IGU as two 6 mm panes with four 12 x 16 mm spacer bars; the air layer is the empty gap.",
      "Körper: das Feld wie gebaut: zwei Pfosten und drei Riegel in den gezeichneten drei Schichten (35 mm außen, 30 mm Dämmsteg, 85 mm innen), die Brüstung als 6 mm Emailglas, 100 mm Wolle und das 1,5-mm-Stahlblech, die Isolierverglasung als zwei Scheiben zu 6 mm mit vier Abstandhaltern 12 x 16 mm; die Luftschicht ist der leere Spalt.")
s.dec("Thermal: IGU centre and edge zone as stacked shells pane-out, cavity, pane-in (centre 1.17 x 2.22 m, edge as the 65 mm frame ring around it, the spacer ring in the cavity layer), spandrel with its air shell, mullions and transoms with the break from the body and a 2 mm cap skin.",
      "Thermisch: Glasmitte und Randzone als gestapelte Hüllen Außenscheibe, Zwischenraum, Innenscheibe (Mitte 1,17 x 2,22 m, Rand als 65-mm-Rahmenring darum, der Abstandhalterring in der Zwischenraumschicht), Brüstung mit ihrer Luftschicht, Pfosten und Riegel mit dem Steg aus dem Körper und einer Deckhaut von 2 mm.")
s.dec("Structure: mullions and transoms as full 50 x 150 mm aluminium sections, the IGU unit and the spandrel as the load-carrying panels, a slider at the cradle hook, the pin at the head anchor and the roller at the base guide.",
      "Tragwerk: Pfosten und Riegel als volle Aluminiumprofile 50 x 150 mm, die Isolierverglasung und die Brüstung als lasttragende Paneele, ein Gleiter am Wagenhaken, das Festlager am Kopfanker und das Rollenlager an der Fußführung.")
sc.append(s)

# ---- 1:100
s = Scale(100)
s.b(box("mullion", F, AL, 0.4, [0.05, 3.3, 0.15], [0.025, 1.65, 0], repeat={"count": 2, "translation": [1.35, 0, 0]}))
for xy, nmt in TY:
    s.b(box(f"transom-{nmt}", F, AL, 0.4, [1.3, 0.05, 0.15], [0.7, xy, 0]))
s.b(box("spandrel", SP, SG, 0.3, [1.3, 0.8, 0.127], [0.65, 0.4, -0.0065]))
s.b(box("glass", I, GL, 0.92, [1.3, 2.35, 0.028], [0.65, 1.175, -0.025], role="glazing"))
s.thof("th-glass-centre", "glass", "glass-centre")
s.thof("th-glass-edge", "glass", "glass-edge")
s.thof("th-spandrel", "spandrel", "spandrel")
s.thof("th-mullion", "mullion", "mullion")
for xy, nmt in TY:
    s.thof(f"th-transom-{nmt}", f"transom-{nmt}", "transom")
s.stof("member-mullion", "mullion", "frame", "member")
for xy, nmt in TY:
    s.stof(f"member-transom-{nmt}", f"transom-{nmt}", "frame", "member")
s.stof("member-igu", "glass", "igu", "member")
s.stof("member-spandrel", "spandrel", "spandrel", "member")
s.st(box("joint-cradle", CR, ST, 0.3, [0.06, 0.06, 0.03], [0.65, 0, -0.025]), "cradle", "joint")
lay_support(s, 100)
s.dec("Body: aluminium sections are solid 50 x 150 mm (the three layers of the section and the 2 mm caps are merged), spandrel is one 127 mm panel, the IGU one 28 mm glass unit (panes, spacers and cavity merged).",
      "Körper: die Aluminiumprofile sind voll 50 x 150 mm (die drei Schichten des Profils und die 2-mm-Kappen sind zusammengefasst), die Brüstung ist ein Paneel von 127 mm, die Isolierverglasung eine Glaseinheit von 28 mm (Scheiben, Abstandhalter und Zwischenraum zusammengefasst).")
s.dec("Thermal: all five regions are referenced from the merged body. Structure: the same members on the merged solids, the cradle slider at 60 mm, supports at the 15 mm and 20 mm sizes.",
      "Thermisch: alle fünf Regionen werden aus dem zusammengefassten Körper referenziert. Tragwerk: dieselben Stäbe auf den zusammengefassten Körpern, der Wagengleiter mit 60 mm, Auflager mit 15 und 20 mm.")
sc.append(s)

# ---- 1:200
s = Scale(200)
s.b(ext("frame", F, AL, 0.4, "xy", OUTERH, -0.075, 0.075, holes=[HOLE_SP, HOLE_IG]))
s.b(box("spandrel", SP, SG, 0.3, [1.3, 0.8, 0.127], [0.65, 0.4, -0.0065]))
s.b(box("glass", I, GL, 0.92, [1.3, 2.35, 0.04], [0.65, 1.175, -0.025], role="glazing"))
s.thof("th-glass-centre", "glass", "glass-centre")
s.thof("th-glass-edge", "glass", "glass-edge")
s.thof("th-spandrel", "spandrel", "spandrel")
s.th(box("th-mullion", F, AL, 0.4, [0.05, 3.3, 0.15], [0.025, 1.65, 0], repeat={"count": 2, "translation": [1.35, 0, 0]}), "mullion")
for xy, nmt in TY:
    s.th(box(f"th-transom-{nmt}", F, AL, 0.4, [1.3, 0.05, 0.15], [0.7, xy, 0]), "transom")
s.stof("member-frame", "frame", "frame", "member")
s.stof("member-igu", "glass", "igu", "member")
s.stof("member-spandrel", "spandrel", "spandrel", "member")
lay_support(s, 200)
s.dec("Body: mullions and transoms are one extruded 150 mm frame with two openings (the drawing draws them as detail, not as separate sections), spandrel as before, the glass unit thickened to the 40 mm feature (drawn 40 mm).",
      "Körper: Pfosten und Riegel sind ein extrudierter Rahmen von 150 mm mit zwei Öffnungen (die Zeichnung zeichnet sie als Detail, nicht als einzelne Profile), Brüstung wie zuvor, die Glaseinheit auf die Einzelheit 40 mm verstärkt (gezeichnet 40 mm).")
s.dec("Thermal: regions referenced from the frame, spandrel and glass. Structure: frame, glass and spandrel members, no cradle joint (merged), supports at 30 mm.",
      "Thermisch: Regionen aus Rahmen, Brüstung und Glas referenziert. Tragwerk: Rahmen-, Glas- und Brüstungsstab, kein Wagengelenk (zusammengefasst), Auflager mit 30 mm.")
sc.append(s)

# ---- 500
s = Scale(500)
s.b(box("frame", F, AL, 0.4, [1.4, 0.075, 0.15], [0.7, 3.2625, 0]))
s.b(box("spandrel", SP, SG, 0.3, [1.4, 0.85, 0.15], [0.7, 0.425, 0]))
s.b(box("glass", I, GL, 0.92, [1.4, 2.4, 0.15], [0.7, 2.05, 0], role="glazing"))
s.thof("th-glass-centre", "glass", "glass-centre")
s.th(box("th-glass-edge", I, GL, 0.9, [1.4, 2.4, 0.075], [0.7, 2.05, -0.0375], role="glazing"), "glass-edge")
s.thof("th-spandrel", "spandrel", "spandrel")
s.thof("th-mullion", "frame", "mullion")
s.th(box("th-transom", F, AL, 0.4, [1.4, 0.075, 0.075], [0.7, 0.8375, 0.0375]), "transom")
s.stof("member-frame", "frame", "frame", "member")
s.stof("member-igu", "glass", "igu", "member")
s.stof("member-spandrel", "spandrel", "spandrel", "member")
lay_support(s, 500)
s.dec("Body: three 150 mm blocks, the spandrel zone, the vision zone (glass with the frame merged) and the head transom, as the 1:500 drawing (one section rectangle in front and plan).",
      "Körper: drei Blöcke von 150 mm, Brüstungszone, Sichtzone (Glas mit dem Rahmen zusammengefasst) und der Kopfriegel, wie die Zeichnung 1:500 (ein Schnittrechteck in Ansicht und Grundriss).")
s.dec("Minimal: thermal and structure are authored on their own merged blocks and are already minimal: three members and the two supports at 75 mm; edge zone and transom are 75 mm pieces.",
      "Thermik und Tragwerk sind eigenständig auf den zusammengefassten Blöcken erstellt und bereits minimal: drei Stäbe und die zwei Auflager mit 75 mm; Randzone und Riegel sind Stücke von 75 mm.")
sc.append(s)
rebase(sc[3], {"igu": (0.05, 0.9), "spandrel": (0.05, 0.05)})
save("components", "curtain-wall-bay", sc)
