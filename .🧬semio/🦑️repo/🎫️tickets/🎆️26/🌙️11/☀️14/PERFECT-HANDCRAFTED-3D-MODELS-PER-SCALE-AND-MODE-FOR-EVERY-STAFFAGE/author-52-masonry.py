import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())
W, L, S = "wall", "lintel", "sill"
BR, BL, CO, WO, PL = "brick", "block", "concrete", "wool", "plaster"
OUT = [[0, 0], [3, 0], [3, 2.75], [0, 2.75]]
OPEN = [[0.625, 2.5], [2.375, 2.5], [2.375, 2.25], [2.125, 2.25], [2.125, 0.91667], [2.2, 0.91667], [2.2, 0.83333], [0.8, 0.83333], [0.8, 0.91667], [0.875, 0.91667], [0.875, 2.25], [0.625, 2.25]]
WIN = [[0.875, 0.91667], [2.125, 0.91667], [2.125, 2.25], [0.875, 2.25]]
ZL = {"facing": (-0.2225, -0.1075), "fill": (-0.1075, 0.0325), "inner-leaf": (0.0325, 0.2075), "plaster": (0.2075, 0.2225)}
MAT = {"facing": (BR, 0.55), "fill": (WO, 0.9), "inner-leaf": (BL, 0.7), "plaster": (PL, 0.85)}
sc = []

def layers(s, hole):
    for k, (a, b) in ZL.items():
        m, t = MAT[k]
        s.b(ext(k, W, m, t, "xy", OUT, a, b, holes=[hole]))

def sill_lintel_fifty(s):
    LB = [("lintel-shell", CO, 0.3, 0.04, -0.2225), ("lintel-ins", WO, 0.9, 0.08, -0.1825), ("lintel-inner", CO, 0.3, 0.175, -0.1025), ("lintel-plaster", PL, 0.85, 0.015, 0.0725)]
    for id_, m, t, th, z0 in LB:
        s.b(box(id_, L, m, t, [1.75, 0.25, th], [0.875, 0.125, z0 + th / 2]))
    s.b(box("lintel-back", W, BL, 0.7, [1.75, 0.25, 0.12], [1.5, 2.375, 0.1475]))
    s.b(box("lintel-back-plaster", W, PL, 0.85, [1.75, 0.25, 0.015], [1.5, 2.375, 0.215]))
    SB = [("sill-shell", CO, 0.3, 0.07, -0.2225), ("sill-ins", WO, 0.9, 0.08, -0.1525), ("sill-block", BL, 0.7, 0.175, -0.0725), ("sill-plaster", PL, 0.85, 0.015, 0.1025)]
    for id_, m, t, th, z0 in SB:
        s.b(box(id_, S, m, t, [1.4, 0.08334, th], [0.7, 0.04167, z0 + th / 2]))
    s.b(box("sill-back", W, BL, 0.7, [1.4, 0.08334, 0.09], [1.5, 0.87500, 0.1625]))
    s.b(box("sill-back-plaster", W, PL, 0.85, [1.4, 0.08334, 0.015], [1.5, 0.875, 0.215]))

FACE = ["-u", "+u", "-v", "+v", "+w"]

def face(s, *ids):
    for d in s.thermal:
        if d["id"] in ids:
            d["sectionFaces"] = FACE

def supports(s, w, h, d, tag=""):
    for i, x in enumerate((0.4, 1.5, 2.6)):
        s.st(box(f"support-found-{i}-pad", W, CO, 0.15, [w, h, d], [x, h / 2, 0]), "wall", "support")

# ---------------- 1:50
s = Scale(50)
layers(s, OPEN)
sill_lintel_fifty(s)
for k in ("plaster", "inner-leaf", "fill", "facing"):
    s.thof(f"th-field-{k}", k, "masonry-field", k)
s.thof("th-field-inner-leaf-lintel-back", "lintel-back", "masonry-field", "inner-leaf")
s.thof("th-field-plaster-lintel-back", "lintel-back-plaster", "masonry-field", "plaster")
s.thof("th-field-inner-leaf-sill-back", "sill-back", "masonry-field", "inner-leaf")
s.thof("th-field-plaster-sill-back", "sill-back-plaster", "masonry-field", "plaster")
for r, pairs in (("lintel", (("plaster", "lintel-plaster"), ("lintel-inner", "lintel-inner"), ("lintel-ins", "lintel-ins"), ("lintel-shell", "lintel-shell"))), ("sill", (("plaster", "sill-plaster"), ("sill-block", "sill-block"), ("sill-ins", "sill-ins"), ("sill-concrete", "sill-shell")))):
    for lay, o in pairs:
        s.thof(f"th-{r}-{lay}", o, r, lay)
face(s, "th-field-facing", "th-lintel-lintel-shell", "th-sill-sill-concrete")
s.stof("member-wall-leaf", "inner-leaf", "wall", "member")
s.stof("member-lintel-inner", "lintel-inner", "lintel", "member")
s.stof("member-lintel-shell", "lintel-shell", "lintel", "member")
s.stof("member-sill-block", "sill-block", "sill", "member")
s.st(box("joint-load-carrier", "load-carrier", CO, 0.3, [0.05, 0.05, 0.175], [0.025, -0.025, 0.12]), "load-carrier", "joint")
supports(s, 0.3, 0.0075, 0.445)
s.dec("Body: the cavity wall layer by layer as four extrusions with the window opening and the lintel and sill recesses as one hole (115 mm facing brick, 140 mm cavity wool, 175 mm block leaf, 15 mm plaster), the precast L-lintel with its 40 mm outer shell, 80 mm wool, 175 mm concrete beam and plaster, the sill with its 70 mm shell, wool, block and plaster, and the block and plaster that close the back of lintel and sill zones so the wall stays 445 mm thick everywhere.",
      "Körper: die zweischalige Wand Schicht für Schicht als vier Extrusionen mit Fensteröffnung sowie Sturz- und Sohlbankaussparungen als ein Loch (115 mm Vormauerziegel, 140 mm Kerndämmung, 175 mm Blockschale, 15 mm Putz), der Fertigteilsturz mit 40 mm Außenschale, 80 mm Wolle, 175 mm Betonbalken und Putz, die Sohlbank mit 70 mm Schale, Wolle, Block und Putz, sowie Block und Putz, die die Rückseite von Sturz- und Sohlbankzone schließen, sodass die Wand überall 445 mm dick bleibt.")
s.dec("Thermal: all three regions with every layer, innermost first, referenced from the body layers (masonry field: plaster, inner leaf, wool, facing, including the back pieces of the lintel and sill zones; lintel and sill with their own four layers). The outermost layer of every region (facing, lintel shell, sill concrete) counts its outer face only, the 1:50 area of the region is that one face; its edges and reveals are cuts and contacts of the continuing wall (`sectionFaces`).",
      "Thermisch: alle drei Regionen mit jeder Schicht, von innen nach außen, aus den Körperschichten referenziert (Mauerfeld: Putz, Innenschale, Wolle, Vormauerung, einschließlich der Rückstücke der Sturz- und Sohlbankzonen; Sturz und Sohlbank mit ihren eigenen vier Schichten). Die äußerste Schicht jeder Region (Vormauerung, Sturzschale, Sohlbankbeton) zählt nur ihre Außenfläche, die Fläche der Region bei 1:50 ist genau diese eine Fläche; ihre Ränder und Laibungen sind Schnitte und Kontakte der weiterlaufenden Wand (`sectionFaces`).")
s.dec("Structure: the 175 mm block leaf with its opening as the wall member, the concrete beam and the 40 mm shell as the lintel members, the sill block as the sill member, a joint block at the load carrier on the wall head, and three contact pads at the foundation anchors.",
      "Tragwerk: die Blockschale von 175 mm mit ihrer Öffnung als Wandstab, der Betonbalken und die 40-mm-Schale als Sturzstäbe, der Sohlbankblock als Sohlbankstab, ein Gelenkblock am Lastträger auf dem Wandkopf und drei Kontaktplatten an den Fundamentankern.")
sc.append(s)

# ---------------- 1:100
s = Scale(100)
layers(s, OPEN)
s.b(box("lintel", L, CO, 0.3, [1.75, 0.25, 0.31], [0.875, 0.125, -0.0675]))
s.b(box("lintel-back", W, BL, 0.7, [1.75, 0.25, 0.135], [1.5, 2.375, 0.155]))
s.b(box("sill", S, CO, 0.3, [1.4, 0.08334, 0.34], [0.7, 0.04167, -0.0525]))
s.b(box("sill-back", W, BL, 0.7, [1.4, 0.08334, 0.105], [1.5, 0.875, 0.17]))
s.thof("th-field", "facing", "masonry-field", "facing")
s.thof("th-field-leaf", "inner-leaf", "masonry-field", "inner-leaf")
s.thermal[-1]["sectionFaces"] = ["-u", "+u", "-v", "+v"]
s.thof("th-lintel", "lintel", "lintel", "lintel-shell")
s.thof("th-sill", "sill", "sill", "sill-concrete")
face(s, "th-field", "th-lintel", "th-sill")
s.stof("member-wall-leaf", "inner-leaf", "wall", "member")
s.stof("member-lintel", "lintel", "lintel", "member")
s.stof("member-sill", "sill", "sill", "member")
s.st(box("joint-load-carrier", "load-carrier", CO, 0.3, [0.05, 0.05, 0.175], [0.025, -0.025, 0.12]), "load-carrier", "joint")
supports(s, 0.3, 0.015, 0.445)
s.dec("Body: the four wall layers stay (the drawing draws them at 1:100) but lintel and sill are single concrete pieces of 310 and 340 mm depth (shell, wool, beam and plaster merged) with one closing piece of block behind each, plaster included.",
      "Körper: die vier Wandschichten bleiben (die Zeichnung zeichnet sie bei 1:100), aber Sturz und Sohlbank sind einzelne Betonstücke von 310 und 340 mm Tiefe (Schale, Wolle, Balken und Putz zusammengefasst) mit je einem schließenden Blockstück dahinter, Putz eingeschlossen.")
s.dec("Thermal: layers are merged at this scale, each region counts its outer face only (`sectionFaces`): the facing brick for the field (the 1:100 drawing shows the facing) beside the inner leaf, which lies fully between plaster and wool and only gives its cut edges away, the merged lintel and sill pieces for their regions. Structure: one member per bone, the contact pads at 15 mm.",
      "Thermisch: die Schichten sind bei diesem Maßstab zusammengefasst, jede Region zählt nur ihre Außenfläche (`sectionFaces`): die Vormauerung für das Feld (die Zeichnung 1:100 zeigt die Vormauerung) neben der Innenschale, die ganz zwischen Putz und Wolle liegt und nur ihre Schnittkanten abgibt, die zusammengefassten Sturz- und Sohlbankstücke für ihre Regionen. Tragwerk: ein Stab je Knochen, die Kontaktplatten mit 15 mm.")
sc.append(s)

# ---------------- 1:200
s = Scale(200)
s.b(ext("field", W, BR, 0.55, "xy", OUT, -0.2225, 0.2225, holes=[OPEN]))
s.b(box("lintel", L, CO, 0.3, [1.75, 0.25, 0.445], [0.875, 0.125, 0]))
s.b(box("sill", S, CO, 0.3, [1.4, 0.08334, 0.445], [0.7, 0.04167, 0]))
s.thof("th-field", "field", "masonry-field")
s.thof("th-lintel", "lintel", "lintel")
s.thof("th-sill", "sill", "sill")
face(s, "th-field", "th-lintel", "th-sill")
s.stof("member-wall", "field", "wall", "member")
s.stof("member-lintel", "lintel", "lintel", "member")
s.stof("member-sill", "sill", "sill", "member")
supports(s, 0.3, 0.03, 0.445)
s.dec("Body: the wall is one brick-coloured extrusion of 445 mm with the opening and the two recesses, the lintel and the sill fill their recesses over the whole depth (the 1:200 drawing no longer shows layers).",
      "Körper: die Wand ist eine ziegelfarbene Extrusion von 445 mm mit Öffnung und den zwei Aussparungen, Sturz und Sohlbank füllen ihre Aussparungen über die ganze Tiefe (die Zeichnung 1:200 zeigt keine Schichten mehr).")
s.dec("Thermal: one solid per region (layers are merged and not distinguished), each with its outer face only (`sectionFaces`). Structure: the whole wall mass is the member (the 175 mm leaf is not resolvable), lintel and sill as members; the load-carrier joint is merged into the wall head; pads at 30 mm.",
      "Thermisch: ein Körper je Region (Schichten sind zusammengefasst und nicht unterschieden), jeder mit nur seiner Außenfläche (`sectionFaces`). Tragwerk: die ganze Wandmasse ist der Stab (die Schale von 175 mm ist nicht auflösbar), Sturz und Sohlbank als Stäbe; das Lastträgergelenk ist im Wandkopf aufgegangen; Platten mit 30 mm.")
sc.append(s)

# ---------------- 1:500
s = Scale(500)
s.b(ext("field", W, BR, 0.55, "xy", OUT, -0.2225, 0.2225, holes=[WIN]))
s.b(box("lintel", L, CO, 0.3, [1.75, 0.25, 0.445], [0.875, 0.125, 0]))
s.b(box("sill", S, CO, 0.3, [1.4, 0.08334, 0.34], [0.7, 0.04167, 0]))
s.thof("th-field", "field", "masonry-field")
s.thof("th-lintel", "lintel", "lintel")
s.thof("th-sill", "sill", "sill")
face(s, "th-field", "th-lintel", "th-sill")
s.stof("member-wall", "field", "wall", "member")
s.stof("member-lintel", "lintel", "lintel", "member")
s.stof("member-sill", "sill", "sill", "member")
supports(s, 0.3, 0.075, 0.445)
s.dec("Body: the wall is one extrusion with the plain window opening of the 1:500 drawing; lintel and sill are 75 mm minimum pieces lying inside the wall mass (they keep their own bones; the recesses are no longer cut).",
      "Körper: die Wand ist eine Extrusion mit der glatten Fensteröffnung der Zeichnung 1:500; Sturz und Sohlbank sind Stücke ab 75 mm, die in der Wandmasse liegen (sie behalten ihre eigenen Knochen; die Aussparungen werden nicht mehr geschnitten).")
s.dec("Thermal: one solid per region on the merged masses, each with its outer face only (`sectionFaces`). Structure: wall, lintel and sill members as before on the simpler opening, pads at the 75 mm feature.",
      "Thermisch: ein Körper je Region auf den zusammengefassten Massen, jeder mit nur seiner Außenfläche (`sectionFaces`). Tragwerk: Wand-, Sturz- und Sohlbankstab wie zuvor auf der einfacheren Öffnung, Platten mit der Einzelheit 75 mm.")
sc.append(s)
save("components", "masonry-wall-with-lintel", sc)
