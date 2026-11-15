import sys, os
__here = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(__here, "author-39-lib.py")).read())
only = [int(a) for a in sys.argv[1:]] or [50, 100, 200]
TI, GL, ED, SG = "timber", "glass", "edge-zone", "steel"
SL, SA = "sill", "sash"
ALL6 = ["-u", "+u", "-v", "+v", "-w", "+w"]
RIMS = ["-u", "+u", "-w", "+w"]


def rail_profile(top):
    if not top:
        return [[0, 0.045], [0, -0.045], [0.06, -0.045], [0.06, -0.013], [0.045, -0.013], [0.045, 0.013], [0.06, 0.013], [0.06, 0.045]]
    return [[1.2, 0.045], [1.2, -0.045], [1.14, -0.045], [1.14, -0.013], [1.155, -0.013], [1.155, 0.013], [1.14, 0.013], [1.14, 0.045]]


def frame(s, jamb_tone=0.55):
    s.b(box("sill-plate", SL, TI, 0.5, [0.3, 0.05, 0.9], [0.15, -0.025, 0]))
    for sd, z in (("left", 0.425), ("right", -0.425)):
        s.b(box(f"jamb-{sd}", SL, TI, jamb_tone, [0.3, 1.2, 0.05], [0.15, 0.6, z]))


def build50():
    s = Scale(50)
    frame(s)
    for sd, z in (("left", 0.3775), ("right", -0.3775)):
        s.b(box(f"stile-{sd}", SA, TI, 0.6, [1.2, 0.09, 0.045], [0.6, 0, z]))
    s.b(ext("rail-bottom", SA, TI, 0.6, "xy", rail_profile(False), -0.355, 0.355))
    s.b(ext("rail-top", SA, TI, 0.6, "xy", rail_profile(True), -0.355, 0.355))
    s.b(box("spacer-bottom", SA, ED, 0.4, [0.015, 0.024, 0.71], [0.0525, 0, 0]))
    s.b(box("spacer-top", SA, ED, 0.4, [0.015, 0.024, 0.71], [1.1475, 0, 0]))
    s.b(box("pane", SA, GL, 0.85, [1.08, 0.024, 0.71], [0.6, 0, 0], role="glazing"))
    s.b(cyl("hinge-barrel", SA, SG, 0.25, "z", [0, 0, 0.3], 0.08, 0.02, repeat={"count": 2, "translation": [0, 0, -0.6]}))

    for i in ("stile-left", "stile-right", "rail-bottom", "rail-top", "jamb-left", "jamb-right"):
        s.thof(f"th-{i}", i, "frame-timber")
    s.thof("th-sill-plate", "sill-plate", "frame-sill")
    cut = lambda d, z0, z1, v0, v1, u0, u1: d
    for lay, v0, v1, mat, tone in (("pane-inner", 0.008, 0.012, GL, 0.85), ("cavity", -0.008, 0.008, "glass-unit-cavity", 0.95), ("pane-outer", -0.012, -0.008, GL, 0.85)):
        c = box(f"th-centre-{lay}", SA, mat, tone, [0.95, v1 - v0, 0.58], [0.6, (v0 + v1) / 2, 0])
        c["sectionFaces"] = RIMS + (["+v"] if lay == "pane-inner" else [])
        s.th(c, "pane-glass", lay)
    ring = [[-0.355, 0.06], [0.355, 0.06], [0.355, 1.14], [-0.355, 1.14]]
    hole = [[-0.29, 0.125], [-0.29, 1.075], [0.29, 1.075], [0.29, 0.125]]
    for lay, v0, v1, mat, tone in (("pane-inner", 0.008, 0.012, GL, 0.85), ("edge-body", -0.008, 0.008, ED, 0.4), ("pane-outer", -0.012, -0.008, GL, 0.85)):
        r = ext(f"th-edge-{lay}", SA, mat, tone, "zx", ring, v0, v1, holes=[hole])
        r["sectionFaces"] = RIMS
        s.th(r, "pane-edge", lay)
    for i in ("spacer-bottom", "spacer-top"):
        s.thof(f"th-{i}", i, "pane-edge", "edge-body")
        s.thermal[-1]["sectionFaces"] = ["-w", "+w"]

    for i in ("stile-left", "stile-right", "rail-bottom", "rail-top"):
        s.stof(f"m-{i}", i, SA, "member")
    for i in ("sill-plate", "jamb-left", "jamb-right"):
        s.stof(f"m-{i}", i, SL, "member")
    s.stof("j-hinge-barrel", "hinge-barrel", SA, "joint")
    s.st(cyl("support-hinge", SA, SG, 0.2, "z", [0, 0, 0], 0.68, 0.01), SA, "support")
    s.dec("Body: the tilt-and-turn sash as built, in its default closed pose. Blend frame: a 50 mm sill plate and two 50 mm jambs, all 300 mm deep. Sash: two stiles 90 x 45 mm (the 2D draws them 45 mm across the glass), a bottom and a top rail 60 x 90 mm with the 26 mm glass rebate cut into the inner face, the warm-edge spacer bars lying in the rebates, the 24 mm insulating unit as one glazing solid (the cavity is no body, the plan and elevation draw the unit filled between the lites) and two steel hinge barrels of 40 mm at the bottom axis. The head stay is not drawn at any scale and not modelled.",
          "Körper: der Kippflügel wie gebaut, in seiner Standardpose geschlossen. Blendrahmen: ein Sockelbrett 50 mm und zwei Pfosten 50 mm, alle 300 mm tief. Flügel: zwei Stiele 90 x 45 mm (die 2D zeichnet sie 45 mm quer zum Glas), ein unterer und ein oberer Riegel 60 x 90 mm mit dem Glasfalz 26 mm an der Innenseite, die Abstandhalterstäbe der warmen Kante im Falz, die Isolierglaseinheit 24 mm als ein Glaskörper (der Zwischenraum ist kein Körper, Grundriss und Ansicht zeichnen die Einheit zwischen den Scheiben gefüllt) und zwei Stahlscharniertöpfe 40 mm an der unteren Achse. Der obere Stützarm ist in keinem Maßstab gezeichnet und nicht modelliert.")
    s.dec("Thermal: timber members and the sill reference the body (single layer regions); the glass centre is three real stacked slabs (inner 4 mm, argon cavity 16 mm, outer 4 mm; the outer slab is the exposed one), the edge-of-glass zone is the same stack as a 65 mm frame (extruded ring) with the spacer bars as its edge body; layer slabs keep their real thickness (C20), their rims are section faces.",
          "Thermik: Holzteile und Sockelbrett verweisen auf den Körper (einlagige Regionen); die Glasmitte besteht aus drei echten gestapelten Platten (innen 4 mm, Argonraum 16 mm, außen 4 mm; die äußere Platte ist die freie), die Glasrandzone ist derselbe Stapel als 65-mm-Rahmen (extrudierter Ring) mit den Abstandhalterstäben als Randkörper; die Schichtplatten behalten ihre wahre Dicke (C20), ihre Ränder sind Schnittflächen.")
    s.dec("Structure: stiles and rails as the sash members (the real profiles, not the equivalent 90 x 90 mm section), sill plate and jambs as the members of the sill bone, the hinge barrels as joints, the support of the hinge anchor as a 20 mm steel pin through the whole bottom axis (it contains the section plane z = 0).",
          "Tragwerk: Stiele und Riegel als Stäbe des Flügels (die wahren Profile, nicht der Ersatzquerschnitt 90 x 90 mm), Sockelbrett und Pfosten als Stäbe des Sockelknochens, die Scharniertöpfe als Gelenke, das Auflager des Gelenkankers als Stahlbolzen 20 mm durch die ganze untere Achse (er enthält die Schnittebene z = 0).")
    return s


def build100():
    s = Scale(100)
    frame(s)
    for sd, z in (("left", 0.3775), ("right", -0.3775)):
        s.b(box(f"stile-{sd}", SA, TI, 0.6, [1.2, 0.09, 0.045], [0.6, 0, z]))
    s.b(box("rail-bottom", SA, TI, 0.6, [0.06, 0.09, 0.71], [0.03, 0, 0]))
    s.b(box("rail-top", SA, TI, 0.6, [0.06, 0.09, 0.71], [1.17, 0, 0]))
    s.b(box("pane", SA, GL, 0.85, [1.08, 0.024, 0.71], [0.6, 0, 0], role="glazing"))
    s.b(cyl("hinge-barrel", SA, SG, 0.25, "z", [0, 0, 0.3], 0.08, 0.02, repeat={"count": 2, "translation": [0, 0, -0.6]}))
    for i in ("stile-left", "stile-right", "rail-bottom", "rail-top", "jamb-left", "jamb-right"):
        s.thof(f"th-{i}", i, "frame-timber")
    s.thof("th-sill-plate", "sill-plate", "frame-sill")
    c = box("th-pane-centre", SA, GL, 0.85, [0.95, 0.024, 0.58], [0.6, 0, 0])
    c["sectionFaces"] = RIMS + ["+v"]
    s.th(c, "pane-glass")
    ring = [[-0.355, 0.06], [0.355, 0.06], [0.355, 1.14], [-0.355, 1.14]]
    hole = [[-0.29, 0.125], [-0.29, 1.075], [0.29, 1.075], [0.29, 0.125]]
    r = ext("th-pane-edge", SA, ED, 0.4, "zx", ring, -0.012, 0.012, holes=[hole])
    r["sectionFaces"] = RIMS + ["+v"]
    s.th(r, "pane-edge")
    for i in ("stile-left", "stile-right", "rail-bottom", "rail-top"):
        s.stof(f"m-{i}", i, SA, "member")
    for i in ("sill-plate", "jamb-left", "jamb-right"):
        s.stof(f"m-{i}", i, SL, "member")
    s.stof("j-hinge-barrel", "hinge-barrel", SA, "joint")
    s.st(cyl("support-hinge", SA, SG, 0.2, "z", [0, 0, 0], 0.68, 0.0175), SA, "support")
    s.dec("Body: the same window at 1:100: the glass rebates and spacer bars are merged into the rails (plain 60 x 90 mm blocks), the unit is one 24 mm glazing slab; stiles, sill plate, jambs and the two hinge barrels as at 1:50.",
          "Körper: dasselbe Fenster bei 1:100: Glasfalze und Abstandhalter sind in die Riegel übergegangen (einfache Blöcke 60 x 90 mm), die Einheit ist eine Glasplatte 24 mm; Stiele, Sockelbrett, Pfosten und die zwei Scharniertöpfe wie bei 1:50.")
    s.dec("Thermal: the layers are merged at this scale; glass centre and edge zone are one 24 mm slab and one 24 mm ring each (their rims and the interior face are section faces, so the outer face is the exposed one).",
          "Thermik: die Schichten sind in diesem Maßstab zusammengefasst; Glasmitte und Randzone sind je eine Platte und ein Ring von 24 mm (ihre Ränder und die Innenseite sind Schnittflächen, die freie Fläche ist die Außenseite).")
    s.dec("Structure: the same members and the hinge joint as at 1:50, the support pin 35 mm.",
          "Tragwerk: dieselben Stäbe und das Scharniergelenk wie bei 1:50, der Auflagerbolzen 35 mm.")
    return s


def build200():
    s = Scale(200)
    frame(s)
    s.b(ext("sash-frame", SA, TI, 0.6, "zx", [[-0.4, 0], [0.4, 0], [0.4, 1.2], [-0.4, 1.2]], -0.045, 0.045, holes=[[[-0.34, 0.06], [-0.34, 1.14], [0.34, 1.14], [0.34, 0.06]]]))
    s.b(box("pane", SA, GL, 0.85, [1.08, 0.024, 0.68], [0.6, 0, 0], role="glazing"))
    s.thof("th-sash-frame", "sash-frame", "frame-timber")
    for i in ("jamb-left", "jamb-right"):
        s.thof(f"th-{i}", i, "frame-timber")
    s.thof("th-sill-plate", "sill-plate", "frame-sill")
    s.thof("th-pane", "pane", "pane-glass")
    s.thermal[-1]["sectionFaces"] = RIMS + ["+v"]
    s.thof("th-pane-edge", "pane", "pane-edge")
    s.thermal[-1]["sectionFaces"] = ALL6
    s.stof("m-sash-frame", "sash-frame", SA, "member")
    for i in ("sill-plate", "jamb-left", "jamb-right"):
        s.stof(f"m-{i}", i, SL, "member")
    s.st(box("support-hinge", SA, SG, 0.2, [0.04, 0.04, 0.8], [0, 0, 0]), SA, "support")
    s.dec("Body: the sash is one frame ring (stiles 60 mm, rails 60 mm, 90 mm deep) with the 24 mm glazing slab inside it (the 2D draws the pane 680 mm wide inside the 800 mm frame); sill plate and jambs as before. Hinges, spacer bars and rebates are below the 30 mm feature and merged.",
          "Körper: der Flügel ist ein Rahmenring (Stiele 60 mm, Riegel 60 mm, 90 mm tief) mit der Glasplatte 24 mm darin (die 2D zeichnet die Scheibe 680 mm breit im 800-mm-Rahmen); Sockelbrett und Pfosten wie zuvor. Scharniere, Abstandhalter und Falze liegen unter dem Merkmal von 30 mm und sind zusammengefasst.")
    s.dec("Thermal: references to the body; the glass slab carries its rims and interior face as section faces, the edge zone references the same slab with all faces as section faces (only the total area is compared at this scale).",
          "Thermik: Verweise auf den Körper; die Glasplatte trägt Ränder und Innenseite als Schnittflächen, die Randzone verweist auf dieselbe Platte mit allen Flächen als Schnittflächen (in diesem Maßstab wird nur die Gesamtfläche verglichen).")
    s.dec("Structure: the sash ring, sill plate and jambs as members, the support as a 40 mm steel block along the hinge axis.",
          "Tragwerk: der Flügelring, Sockelbrett und Pfosten als Stäbe, das Auflager als Stahlblock 40 mm entlang der Gelenkachse.")
    return s


B = {50: build50, 100: build100, 200: build200}
save("components", "tilt-window", [B[k]() for k in only])
