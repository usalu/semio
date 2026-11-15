import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())
F, BX, BR = "facade", "box", "bottom-rail"
AL, GL, MA, EP = "aluminium", "glass", "masonry", "eps"
sc = []
N = 17
LAYW = [("plaster", MA, 0.85, -0.27, 0.015), ("block", MA, 0.55, -0.255, 0.175), ("eps", EP, 0.9, -0.08, 0.07), ("render", MA, 0.8, -0.01, 0.01)]
PARTS = [("sill", 0.0, 1.0), ("head", 2.25, 0.5)]

def slat(s, i, kind):
    if kind == "lens":
        return s.b(loft(f"slat-{i}", f"slat-{i}", AL, 0.55, [sec([0, 0, -0.585], 0.045, 0.006), sec([0, 0, 0.585], 0.045, 0.006)], role="detail"))
    if kind == "sheet":
        return s.b(loft(f"slat-{i}", f"slat-{i}", AL, 0.55, [sec([0, 0, -0.585], 0.045, 0.0006), sec([0, 0, 0.585], 0.045, 0.0006)], role="texture"))
    if kind == "bar":
        return s.b(box(f"slat-{i}", f"slat-{i}", AL, 0.55, [0.09, 0.075, 1.17], [0, 0, 0], role="detail"))
    return s.b(box(f"slat-{i}", f"slat-{i}", AL, 0.55, [0.09, 0.0012, 1.17], [0, 0, 0], role="texture"))

def guides(s, kind):
    if kind == "c":
        for nm, sg in (("left", 1), ("right", -1)):
            pr = [[sg * 0.6, 0.095], [sg * 0.57, 0.095], [sg * 0.57, 0.1025], [sg * 0.5925, 0.1025], [sg * 0.5925, 0.1075], [sg * 0.57, 0.1075], [sg * 0.57, 0.115], [sg * 0.6, 0.115]]
            s.b(ext(f"guide-{nm}", BX, AL, 0.45, "zx", pr, -1.29, 0, role="detail"))
    else:
        w = {"b": 0.03, "r": 0.075}[kind]
        xw = 0.02 if kind == "b" and False else (0.03 if kind == "b" else 0.075)
        for nm, sg in (("left", 1), ("right", -1)):
            s.b(box(f"guide-{nm}", BX, AL, 0.45, [xw, 1.29, w], [0.105, -0.645, sg * (0.6 - w / 2)], role="detail"))

def wall(s, layered, w=0.27, glass_x=None):
    for nm, y0, h in PARTS:
        if layered:
            for ln, m, t, x0, th in LAYW:
                s.b(box(f"wall-{ln}-{nm}", F, m, t, [th, h, 1.2], [x0 + th / 2, y0 + h / 2, 0]))
        else:
            s.b(box(f"wall-{nm}", F, MA, 0.6, [w, h, 1.2], [-w / 2, y0 + h / 2, 0]))

# ---------- 1:50
s = Scale(50)
wall(s, True)
for ln, x0, th, nm in (("pane-out", -0.2, 0.004, "pane-out"), ("pane-in", -0.18, 0.004, "pane-in")):
    s.b(box(nm, F, GL, 0.92, [th, 1.25, 1.1], [x0 + th / 2, 1.625, 0], role="glazing"))
box_out = [[0, 0], [0.03, 0], [0.03, 0.0075], [0.12, 0.0075], [0.12, 0], [0.15, 0], [0.15, 0.21], [0, 0.21]]
s.b(ext("box", BX, AL, 0.5, "xy", box_out, -0.585, 0.585, holes=[rect(0.075, 0.1088, 0.135, 0.1875)]))
s.b(box("box-end-cap", BX, AL, 0.5, [0.15, 0.21, 0.015], [0.075, 0.105, 0.5925], repeat={"count": 2, "translation": [0, 0, -1.185]}))
guides(s, "c")
for i in range(N):
    slat(s, i, "lens")
s.b(box("bottom-rail", BR, AL, 0.45, [0.09, 0.04, 1.17], [0, 0, 0]))
for ln, m, t, x0, th in LAYW:
    for nm, y0, h in PARTS:
        pass
s.th(box("th-cavity", F, GL, 0.95, [0.016, 1.25, 1.1], [-0.188, 1.625, 0], role="glazing"), "glazing-behind", "cavity")
s.thof("th-pane-out", "pane-out", "glazing-behind", "pane-out")
s.thof("th-pane-in", "pane-in", "glazing-behind", "pane-in")
for ln, m, t, x0, th in LAYW:
    for nm, y0, h in PARTS:
        s.thof(f"th-wall-{ln}-{nm}", f"wall-{ln}-{nm}", "facade-wall", ln)
for i in range(N):
    s.thof(f"th-lamella-{i}", f"slat-{i}", "lamellae", "lam-sheet")
for o in ("box", "box-end-cap", "guide-left", "guide-right", "bottom-rail"):
    s.thof(f"th-{o}", o, "box-and-rails", "box-wall")
s.stof("member-wall-block-sill", "wall-block-sill", "facade", "member")
s.stof("member-wall-block-head", "wall-block-head", "facade", "member")
s.stof("member-box", "box", "box", "member")
s.stof("member-guide-left", "guide-left", "box", "member")
s.stof("member-guide-right", "guide-right", "box", "member")
for i in range(N):
    s.st(cyl(f"joint-slat-{i}", f"slat-{i}", AL, 0.3, "z", [0, 0, 0.5], 0.03, 0.005, repeat={"count": 2, "translation": [0, 0, -1.0]}), f"slat-{i}", "joint")
s.st(box("support-wall-foot-block", F, MA, 0.2, [0.27, 0.0075, 1.2], [-0.135, 0.00375, 0]), "facade", "support")
s.dec("Body: the facade strip as built (sill 1 m and head 0.5 m, each in its four layers: 15 mm plaster, 175 mm block, 70 mm EPS, 10 mm render), the IGU behind the blind as two 4 mm panes, the roller-blind box as a hollow aluminium extrusion with a bottom slot (wall thickened from 1.5 to 7.5 mm, the smallest feature) and end caps, two C-shaped guide rails 30 x 20 mm, 17 lamellae of 90 mm chord as lens-shaped 12 mm solids on their own bones (as the drawing: the real lamella is an arched 0.45 mm sheet) and the 90 x 40 mm bottom rail.",
      "Körper: der Fassadenstreifen wie gebaut (Brüstung 1 m und Sturz 0,5 m, je in vier Schichten: 15 mm Putz, 175 mm Block, 70 mm EPS, 10 mm Putz), die Verglasung hinter dem Behang als zwei Scheiben zu 4 mm, der Behangkasten als hohle Aluminiumprofil-Extrusion mit Schlitz unten (Wand von 1,5 auf 7,5 mm verstärkt, die kleinste Einzelheit) und Endkappen, zwei C-förmige Führungsschienen 30 x 20 mm, 17 Lamellen mit 90 mm Breite als linsenförmige Körper von 12 mm auf eigenen Knochen (wie die Zeichnung: die echte Lamelle ist ein gewölbtes Blech von 0,45 mm) und die Endschiene 90 x 40 mm.")
s.dec("Thermal: lamella region as the 17 lens surfaces, box and rails from the body, glazing as shells pane-out, 16 mm cavity, pane-in, the wall as its four layers on sill and head.",
      "Thermisch: die Lamellenregion als die 17 Linsenflächen, Kasten und Schienen aus dem Körper, die Verglasung als Hüllen Außenscheibe, Zwischenraum 16 mm, Innenscheibe, die Wand in ihren vier Schichten an Brüstung und Sturz.")
s.dec("Structure: the 175 mm block leaf at sill and head as the facade member, the box and both guides as the box members, a 10 mm pivot pin at each end of every slat as the revolute joint (the prismatic slide bones coincide with the slat bones and are merged), the fixed foot block at the wall foot.",
      "Tragwerk: die Blockschale 175 mm an Brüstung und Sturz als Fassadenstab, Kasten und beide Führungen als Kastenstäbe, ein Drehstift von 10 mm an jedem Lamellenende als Drehgelenk (die prismatischen Gleitknochen fallen mit den Lamellenknochen zusammen und sind zusammengefasst), der feste Fußblock am Wandfuß.")
sc.append(s)

def coarse(den, wallw, gl, kind_slat, gk, boxsz, rail, sup, minimal, texts):
    s = Scale(den)
    wall(s, False, wallw)
    s.b(box("glass", F, GL, 0.92, [gl, 1.25, 1.1], [-0.188, 1.625, 0], role="glazing"))
    s.b(box("box", BX, AL, 0.5, [0.15, 0.21, boxsz], [0.075, 0.105, 0]))
    guides(s, gk)
    for i in range(N):
        slat(s, i, kind_slat)
    s.b(box("bottom-rail", BR, AL, 0.45, [0.09, rail, 1.17], [0, 0, 0]))
    s.thof("th-glass-centre", "glass", "glazing-behind", "cavity")
    for nm, y0, h in PARTS:
        s.thof(f"th-wall-{nm}", f"wall-{nm}", "facade-wall", "block")
    for i in range(N):
        s.thof(f"th-lamella-{i}", f"slat-{i}", "lamellae", "lam-sheet")
    for o in ("box", "guide-left", "guide-right", "bottom-rail"):
        s.thof(f"th-{o}", o, "box-and-rails", "box-wall")
    for nm, y0, h in PARTS[:1]:
        pass
    s.st(box("member-wall-block-sill", F, MA, 0.55, [0.19, 1.0, 1.2], [-0.175, 0.5, 0]), "facade", "member")
    s.st(box("member-wall-block-head", F, MA, 0.55, [0.19, 0.5, 1.2], [-0.175, 2.5, 0]), "facade", "member")
    s.stof("member-box", "box", "box", "member")
    s.stof("member-guide-left", "guide-left", "box", "member")
    s.stof("member-guide-right", "guide-right", "box", "member")
    if den == 100:
        for i in range(N):
            s.st(box(f"joint-slat-{i}", f"slat-{i}", AL, 0.3, [0.015, 0.015, 0.03], [0, 0, 0.5], repeat={"count": 2, "translation": [0, 0, -1.0]}), f"slat-{i}", "joint")
    s.st(box("support-wall-foot-block", F, MA, 0.2, [0.27, sup, 1.2], [-0.135, sup / 2, 0]), "facade", "support")
    for en, de in texts:
        s.dec(en, de)
    sc.append(s)

coarse(100, 0.27, 0.024, "sheet", "b", 0.15 * 0 + 1.2, 0.04, 0.015, False, [(
    "Body: sill and head are single 270 mm masonry blocks (layers merged), the IGU one 24 mm slab, the box a solid 150 x 210 mm block, the guides 20 x 30 mm bars; lamellae are 1.2 mm thin sheets with the 90 mm chord (the drawing draws them as lines) on their 17 bones, the bottom rail stays 90 x 40 mm.",
    "Körper: Brüstung und Sturz sind einzelne Mauerwerksblöcke von 270 mm (Schichten zusammengefasst), die Verglasung eine Platte von 24 mm, der Kasten ein voller Block 150 x 210 mm, die Führungen Stäbe 20 x 30 mm; die Lamellen sind Bleche von 1,2 mm mit 90 mm Breite (die Zeichnung zeichnet sie als Linien) auf ihren 17 Knochen, die Endschiene bleibt 90 x 40 mm."),
    ("Thermal: regions referenced from the merged body (wall on the block layer, glazing on the cavity layer). Structure: block members of the 190 mm leaf, box and guides, the pivots as 15 mm blocks at the slat ends, the foot block at 15 mm.",
     "Thermisch: Regionen aus dem zusammengefassten Körper referenziert (Wand auf der Blockschicht, Verglasung auf der Zwischenraumschicht). Tragwerk: Blockstäbe der Schale 190 mm, Kasten und Führungen, die Drehpunkte als Blöcke von 15 mm an den Lamellenenden, der Fußblock mit 15 mm.")])
coarse(200, 0.27, 0.024, "flat", "b", 1.2, 0.04, 0.03, False, [(
    "Body: as 1:100 but the lamella sheets are flat 1.2 mm plates (the arch is not resolvable) and the box is the same block; walls, glass, guides and rail follow the 1:200 drawing, rail 40 mm above the 30 mm feature.",
    "Körper: wie 1:100, aber die Lamellenbleche sind ebene Platten von 1,2 mm (die Wölbung ist nicht auflösbar), der Kasten ist derselbe Block; Wände, Glas, Führungen und Schiene folgen der Zeichnung 1:200, Schiene 40 mm über der Einzelheit 30 mm."),
    ("Thermal: referenced as at 1:100. Structure: leaf, box and guide members; the slat pivots are merged into the lamellae; foot block at 30 mm.",
     "Thermisch: referenziert wie bei 1:100. Tragwerk: Schalen-, Kasten- und Führungsstäbe; die Lamellendrehpunkte sind in den Lamellen aufgegangen; Fußblock mit 30 mm.")])
coarse(500, 0.27, 0.075, "bar", "r", 1.2, 0.075, 0.075, True, [(
    "Minimal: the same masses at the 75 mm feature (guides 75 mm, bottom rail 75 mm high); every lamella keeps its bone as a 75 mm bar and the glass is a 75 mm slab because the 1:500 drawing still draws each as a pen line of 90 mm: this model is already minimal in count.",
    "Minimal: dieselben Massen mit der Einzelheit 75 mm (Führungen 75 mm, Endschiene 75 mm hoch); jede Lamelle behält ihren Knochen als Balken von 75 mm und das Glas ist eine Platte von 75 mm, da die Zeichnung 1:500 jedes noch als Stiftlinie von 90 mm zeichnet: dieses Modell ist in der Anzahl bereits minimal."),
    ("Thermal and structure are authored on this minimal body: regions referenced, members for leaf, box and guides, foot block 75 mm.",
     "Thermik und Tragwerk sind auf diesem minimalen Körper erstellt: Regionen referenziert, Stäbe für Schale, Kasten und Führungen, Fußblock 75 mm.")])
save("components", "external-venetian-blind", sc)
