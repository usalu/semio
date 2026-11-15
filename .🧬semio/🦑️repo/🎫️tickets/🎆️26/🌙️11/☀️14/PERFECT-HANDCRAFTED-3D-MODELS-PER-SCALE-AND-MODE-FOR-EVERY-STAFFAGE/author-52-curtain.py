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
RIN = rect(0.65, 1.175, 1.17, 2.22)
RO = rect(0.65, 1.175, 1.3, 2.35)
for pn, z0, z1 in (("out", -0.039, -0.033), ("in", -0.017, -0.011)):
    s.b(ext(f"pane-{pn}-centre", I, GL, 0.92, "xy", RIN, z0, z1, role="glazing"))
    s.b(ext(f"pane-{pn}-edge", I, GL, 0.9, "xy", RO, z0, z1, holes=[RIN], role="glazing"))
for nm, sz, at in (("base", [1.3, 0.012, 0.016], [0.65, 0.006, -0.025]), ("head", [1.3, 0.012, 0.016], [0.65, 2.344, -0.025]), ("left", [0.012, 2.326, 0.016], [0.006, 1.175, -0.025]), ("right", [0.012, 2.326, 0.016], [1.294, 1.175, -0.025])):
    s.b(box(f"spacer-{nm}", I, GL, 0.5, sz, at, role="detail"))
# thermal
s.thof("th-centre-pane-out", "pane-out-centre", "glass-centre", "pane-out")
s.thermal[-1]["sectionFaces"] = ["+w"]
s.th(ext("th-centre-cavity", I, GL, 0.95, "xy", RIN, -0.033, -0.017, role="glazing"), "glass-centre", "cavity")
s.thof("th-centre-pane-in", "pane-in-centre", "glass-centre", "pane-in")
s.thof("th-edge-pane-out", "pane-out-edge", "glass-edge", "pane-out")
s.thermal[-1]["sectionFaces"] = ["+w"]
s.th(ext("th-edge-spacer-cavity", I, GL, 0.5, "xy", RO, -0.033, -0.017, holes=[RIN], role="detail"), "glass-edge", "spacer-cavity")
s.thof("th-edge-pane-in", "pane-in-edge", "glass-edge", "pane-in")
s.th(box("th-spandrel-air", SP, WO, 0.95, [1.3, 0.8, 0.02], [0.65, 0.4, -0.051], role="glazing"), "spandrel", "air")
s.thof("th-spandrel-enamel", "spandrel-enamel", "spandrel", "enamel")
s.thermal[-1]["sectionFaces"] = ["+w"]
s.thof("th-spandrel-wool", "spandrel-wool", "spandrel", "wool")
s.thof("th-spandrel-pan", "spandrel-pan", "spandrel", "steel-pan")
s.thof("th-mullion-break", "mullion-break", "mullion", "break")
s.thof("th-mullion-cap", "mullion-outer", "mullion", "cap")
s.thermal[-1]["sectionFaces"] = ["-u", "+u", "-v", "+v", "+w"]
for xy, nmt in TY:
    s.thof(f"th-transom-{nmt}-break", f"transom-{nmt}-break", "transom", "break")
    s.thof(f"th-transom-{nmt}-cap", f"transom-{nmt}-outer", "transom", "cap")
    s.thermal[-1]["sectionFaces"] = ["-u", "+u", "-v", "+v", "+w"]
# structure
s.st(box("member-mullion", F, AL, 0.4, [0.05, 3.3, 0.15], [0.025, 1.65, 0], repeat={"count": 2, "translation": [1.35, 0, 0]}), "frame", "member")
for xy, nmt in TY:
    s.st(box(f"member-transom-{nmt}", F, AL, 0.4, [1.3, 0.05, 0.15], [0.7, xy, 0]), "frame", "member")
s.st(box("member-igu", I, GL, 0.9, [1.3, 2.35, 0.027], [0.65, 1.175, -0.025]), "igu", "member")
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
s.b(ext("glass-centre", I, GL, 0.92, "xy", RIN, -0.039, -0.011, role="glazing"))
s.b(ext("glass-edge", I, GL, 0.9, "xy", RO, -0.039, -0.011, holes=[RIN], role="glazing"))
s.thof("th-glass-centre", "glass-centre", "glass-centre")
s.thermal[-1]["sectionFaces"] = ["-w"]
s.thof("th-glass-edge", "glass-edge", "glass-edge")
s.thermal[-1]["sectionFaces"] = ["-w"]
s.thof("th-spandrel", "spandrel", "spandrel")
s.thermal[-1]["sectionFaces"] = ["-w"]
s.thof("th-mullion", "mullion", "mullion")
s.thermal[-1]["sectionFaces"] = ["-u", "+u", "-v", "+v", "-w"]
for xy, nmt in TY:
    s.thof(f"th-transom-{nmt}", f"transom-{nmt}", "transom")
    s.thermal[-1]["sectionFaces"] = ["-u", "+u", "-v", "+v", "-w"]
s.stof("member-mullion", "mullion", "frame", "member")
for xy, nmt in TY:
    s.stof(f"member-transom-{nmt}", f"transom-{nmt}", "frame", "member")
s.st(box("member-igu", I, GL, 0.9, [1.3, 2.35, 0.026], [0.65, 1.175, -0.025]), "igu", "member")
s.stof("member-spandrel", "spandrel", "spandrel", "member")
s.st(box("joint-cradle", CR, ST, 0.3, [0.06, 0.06, 0.03], [0.65, 0, -0.025]), "cradle", "joint")
lay_support(s, 100)
s.dec("Body: the aluminium sections are solid 50 x 150 mm (the three layers of the section and the 2 mm caps are merged), the spandrel is one 127 mm panel, the glass unit is one 28 mm slab in two pieces, the 1.17 x 2.22 m vision zone and the 65 mm edge ring (panes, spacers and cavity merged, the zones stay apart because they are two thermal regions).",
      "Körper: die Aluminiumprofile sind voll 50 x 150 mm (die drei Schichten des Profils und die 2-mm-Kappen sind zusammengefasst), die Brüstung ist ein Paneel von 127 mm, die Glaseinheit eine Platte von 28 mm in zwei Teilen, der Sichtzone 1,17 x 2,22 m und dem Randring von 65 mm (Scheiben, Abstandhalter und Zwischenraum zusammengefasst, die Zonen bleiben getrennt, weil sie zwei thermische Regionen sind).")
s.dec("Thermal: every region is referenced from its own merged solid and counts only the face that the nested layers show to the outside (the other faces are contacts and cuts of the continuing construction, `sectionFaces`): the room-side glass faces, the room-side spandrel face and the visible 50 mm faces of mullions and transoms. Structure: the same members on the merged solids, the IGU as a 26 mm member inside the glass slab, the cradle slider at 60 mm, supports at the 15 mm and 20 mm sizes.",
      "Thermisch: jede Region wird aus ihrem eigenen zusammengefassten Körper referenziert und zählt nur die Fläche, die die verschachtelten Schichten nach außen zeigen (die übrigen Flächen sind Kontakte und Schnitte der weiterlaufenden Konstruktion, `sectionFaces`): die raumseitigen Glasflächen, die raumseitige Brüstungsfläche und die sichtbaren 50-mm-Flächen von Pfosten und Riegeln. Tragwerk: dieselben Stäbe auf den zusammengefassten Körpern, die Isolierverglasung als Stab von 26 mm innerhalb der Glasplatte, der Wagengleiter mit 60 mm, Auflager mit 15 und 20 mm.")
sc.append(s)

# ---- 1:200
s = Scale(200)
s.b(ext("frame", F, AL, 0.4, "xy", OUTERH, -0.075, 0.075, holes=[HOLE_SP, HOLE_IG]))
s.b(box("spandrel", SP, SG, 0.3, [1.3, 0.8, 0.127], [0.65, 0.4, -0.0065]))
s.b(box("glass", I, GL, 0.92, [1.3, 2.35, 0.04], [0.65, 1.175, -0.025], role="glazing"))
FACE = ["-u", "+u", "-v", "+v", "-w"]
s.th(ext("th-glass-centre", I, GL, 0.92, "xy", RIN, -0.045, -0.005, role="glazing"), "glass-centre").update(sectionFaces=FACE)
s.th(ext("th-glass-edge", I, GL, 0.9, "xy", RO, -0.045, -0.005, holes=[RIN], role="glazing"), "glass-edge").update(sectionFaces=FACE)
s.thof("th-spandrel", "spandrel", "spandrel")
s.thermal[-1]["sectionFaces"] = ["-w"]
s.th(box("th-mullion", F, AL, 0.4, [0.05, 3.3, 0.15], [0.025, 1.65, 0], repeat={"count": 2, "translation": [1.35, 0, 0]}), "mullion").update(sectionFaces=FACE)
s.th(box("th-transom", F, AL, 0.4, [1.3, 0.05, 0.15], [0.7, 0.025, 0], repeat={"count": 2, "translation": [0, 3.25, 0]}), "transom").update(sectionFaces=FACE)
s.stof("member-frame", "frame", "frame", "member")
s.st(box("member-igu", I, GL, 0.9, [1.3, 2.35, 0.038], [0.65, 1.175, -0.025]), "igu", "member")
s.stof("member-spandrel", "spandrel", "spandrel", "member")
lay_support(s, 200)
s.dec("Body: mullions and transoms are one extruded 150 mm frame with two openings (the drawing draws them as detail, not as separate sections), the spandrel as before, the glass unit one slab thickened to the 40 mm feature (drawn 40 mm; vision zone and edge ring are no longer separate bodies).",
      "Körper: Pfosten und Riegel sind ein extrudierter Rahmen von 150 mm mit zwei Öffnungen (die Zeichnung zeichnet sie als Detail, nicht als einzelne Profile), die Brüstung wie zuvor, die Glaseinheit eine Platte, auf die Einzelheit 40 mm verstärkt (gezeichnet 40 mm; Sichtzone und Randring sind keine getrennten Körper mehr).")
s.dec("Thermal: the zones are thermal pieces flush with the merged bodies (vision zone, 65 mm edge ring, two mullion strips, base and head transom strips: only the room-side faces count, `sectionFaces`), the spandrel references its room-side face; the mid transom is not drawn separately in the frame body and is covered by the transom strips. Only the total exposed area is compared here. Structure: frame, glass and spandrel members, no cradle joint (merged), supports at 30 mm.",
      "Thermisch: die Zonen sind thermische Stücke bündig mit den zusammengefassten Körpern (Sichtzone, Randring von 65 mm, zwei Pfostenstreifen, Fuß- und Kopfriegelstreifen: nur die Raumseiten zählen, `sectionFaces`), die Brüstung verweist auf ihre Raumseite; der mittlere Riegel ist im Rahmenkörper nicht einzeln gezeichnet und wird von den Riegelstreifen abgedeckt. Hier wird nur die Summe der freien Flächen verglichen. Tragwerk: Rahmen-, Glas- und Brüstungsstab, kein Wagengelenk (zusammengefasst), Auflager mit 30 mm.")
sc.append(s)

# ---- 500
s = Scale(500)
s.b(box("frame", F, AL, 0.4, [1.4, 0.075, 0.15], [0.7, 3.2625, 0]))
s.b(box("spandrel", SP, SG, 0.3, [1.4, 0.85, 0.15], [0.7, 0.425, 0]))
s.b(box("glass", I, GL, 0.92, [1.4, 2.4, 0.15], [0.7, 2.05, 0], role="glazing"))
R5I = rect(0.7, 2.05, 1.27, 2.27)
R5O = rect(0.7, 2.05, 1.4, 2.4)
s.th(ext("th-glass-centre", I, GL, 0.92, "xy", R5I, -0.075, 0.075, role="glazing"), "glass-centre").update(sectionFaces=FACE)
s.th(ext("th-glass-edge", I, GL, 0.9, "xy", R5O, -0.075, 0.075, holes=[R5I], role="glazing"), "glass-edge").update(sectionFaces=FACE)
s.thof("th-spandrel", "spandrel", "spandrel")
s.thermal[-1]["sectionFaces"] = FACE
s.thof("th-transom", "frame", "transom")
s.thermal[-1]["sectionFaces"] = FACE
s.th(box("th-mullion", F, AL, 0.4, [0.075, 2.4, 0.15], [0.0375, 1.7, 0], repeat={"count": 2, "translation": [1.325, 0, 0]}), "mullion").update(sectionFaces=FACE)
s.stof("member-frame", "frame", "frame", "member")
s.stof("member-igu", "glass", "igu", "member")
s.stof("member-spandrel", "spandrel", "spandrel", "member")
lay_support(s, 500)
s.dec("Body: three 150 mm blocks, the spandrel zone, the vision zone (glass with the frame merged) and the head transom, as the 1:500 drawing (one section rectangle in front and plan).",
      "Körper: drei Blöcke von 150 mm, Brüstungszone, Sichtzone (Glas mit dem Rahmen zusammengefasst) und der Kopfriegel, wie die Zeichnung 1:500 (ein Schnittrechteck in Ansicht und Grundriss).")
s.dec("Minimal: thermal and structure are authored on their own merged blocks and are already minimal: three members and the two supports at 75 mm. Every region counts one face only (the room side) (`sectionFaces`): the vision zone and the 65 mm edge ring are thermal pieces flush with the glass block, the mullions 75 mm strips on it.",
      "Minimal: Thermik und Tragwerk sind eigenständig auf den zusammengefassten Blöcken erstellt und bereits minimal: drei Stäbe und die zwei Auflager mit 75 mm. Jede Region zählt nur eine Fläche (die Raumseite) (`sectionFaces`): Sichtzone und Randring von 65 mm sind thermische Stücke bündig mit dem Glasblock, die Pfosten Streifen von 75 mm darauf.")
sc.append(s)
rebase(sc[3], {"igu": (0.05, 0.9), "spandrel": (0.05, 0.05)})
save("components", "curtain-wall-bay", sc)
