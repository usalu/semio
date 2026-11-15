import sys, os
__here = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(__here, "author-39-lib.py")).read())
exec(open(os.path.join(__here, "author-39-canopy-geom.py")).read())
only = [int(a) for a in sys.argv[1:]] or [50, 100, 200, 500]
ST, CO, GL, WO, RN, PD = "steel", "concrete", "glass", "mineral-wool", "render", "anchor-pad"
CB = "canopy"
HOLE = lambda v: [[v - 0.12, -0.1], [v - 0.12, 0.1], [v + 0.12, 0.1], [v + 0.12, -0.1]]
WALL_PROFILE = [[-2.33, -0.5], [-2.33, 0.5], [3.02, 0.5], [3.02, -0.5]]
ALL6 = ["-u", "+u", "-v", "+v", "-w", "+w"]
EDGES = ["-v", "+v", "-w", "+w"]
LAYER_CUT = ["-u", "-v", "+v", "-w", "+w"]
dirv = [D[0], D[1], 0]


def tube(id, p0, length, r, tone=0.45, slender=False):
    d = revolve(id, CB, ST, tone, dirv, [p0[0], p0[1], 0], [[0, r], [length, r]], role="body")
    if slender:
        d["slender"] = True
    return d


def tube_wall(id, p0, length, ro, ri, slender=False):
    d = revolve(id, CB, ST, 0.5, dirv, [p0[0], p0[1], 0], [[0, ri], [0, ro], [length, ro], [length, ri], [0, ri]], role="body")
    if slender:
        d["slender"] = True
    return d


def pin(id, at, r, half, tone=0.1, role="detail"):
    return cyl(id, CB, ST, tone, "z", [at[0], at[1], 0], 2 * half, r, role=role)


def sheet(id, u0, u1, hole=None, notch=None, v0=0.08, v1=0.1):
    if notch:
        pr = sheet_with_notch(u0, u1, -0.5, 0.5, notch[0], notch[1])
        return ext(id, CB, GL, 0.92, "zx", pr, v0, v1, role="glazing")
    if hole:
        cu, a, b = hole
        pr = [[-0.5, u0], [0.5, u0], [0.5, u1], [-0.5, u1]]
        ring = [[b * math.sin(t), cu + a * math.cos(t)] for t in [2 * math.pi * k / 20 for k in range(20)]][::-1]
        return ext(id, CB, GL, 0.92, "zx", pr, v0, v1, holes=[ring], role="glazing")
    return box(id, CB, GL, 0.92, [u1 - u0, v1 - v0, 1.0], [(u0 + u1) / 2, (v0 + v1) / 2, 0], role="glazing")


def build50():
    s = Scale(50)
    s.b(box("wall-core", CB, CO, 0.6, [0.3, 5.35, 1], [-0.35, 0.345, 0]))
    s.b(ext("wall-insulation", CB, WO, 0.9, "yz", WALL_PROFILE, -0.2, -0.02, holes=[HOLE(0), HOLE(2.88)]))
    s.b(ext("wall-render", CB, RN, 0.55, "yz", WALL_PROFILE, -0.02, 0, holes=[HOLE(0), HOLE(2.88)]))
    for nm, u in (("inner", -0.45), ("outer", -0.25)):
        s.b(cyl(f"rebar-v-{nm}", CB, ST, 0.2, "y", [u, 0.425, -0.45], 4.91, 0.006, role="detail", repeat={"count": 7, "translation": [0, 0, 0.15]}))
        s.b(cyl(f"rebar-h-{nm}", CB, ST, 0.2, "z", [u + (0.012 if nm == "inner" else -0.012), -2.03, 0], 0.9, 0.006, role="detail", repeat={"count": 33, "translation": [0, 0.15, 0]}))
    for nm, v in (("beam", 0), ("strut", 2.88)):
        s.b(box(f"pad-{nm}", CB, PD, 0.7, [0.18, 0.24, 0.2], [-0.11, v, 0]))
        s.b(box(f"plate-{nm}", CB, ST, 0.3, [0.02, 0.2, 0.16], [-0.01, v, 0]))
        for sv, tag in ((1, "top"), (-1, "bottom")):
            s.b(ext(f"nut-{nm}-{tag}", CB, ST, 0.25, "yz", hexagon(v + sv * 0.085, -0.062, 0.024), 0, 0.013, role="detail", repeat={"count": 2, "translation": [0, 0, 0.124]}))
    s.b(ext("beam", CB, ST, 0.4, "yz", ipe(), 0, 5.5))
    for k in range(4):
        u0 = 0.1 + 1.34 * k
        s.b(sheet(f"glass-{k}", u0, u0 + 1.28, notch=(3.77, 0.055) if k == 2 else None))
    for sd, z in (("l", 1), ("r", -1)):
        z0, z1 = (0.042, 0.05) if z > 0 else (-0.05, -0.042)
        s.b(ext(f"cheek-beam-{sd}", CB, ST, 0.3, "xy", CHEEK_BEAM, z0, z1))
        z0, z1 = (0.032, 0.04) if z > 0 else (-0.04, -0.032)
        s.b(ext(f"cheek-wall-{sd}", CB, ST, 0.3, "xy", CHEEK_WALL, z0, z1))
    s.b(pin("pin-beam", A, 0.012, 0.05))
    s.b(pin("pin-beam-2", P2, 0.01, 0.05))
    s.b(pin("pin-wall", P_WALL, 0.01, 0.04))
    s.b(tube("strut-tube", E_END, TUBE_LEN, R_TUBE))

    s.thof("th-wall-concrete", "wall-core", "wall-face", "wall-concrete")
    s.thof("th-wall-wool", "wall-insulation", "wall-face", "wall-wool")
    s.thof("th-wall-render", "wall-render", "wall-face", "wall-render")
    s.thermal[-1]["sectionFaces"] = LAYER_CUT
    for nm, v in (("beam", 0), ("strut", 2.88)):
        c = box(f"th-concrete-{nm}", CB, CO, 0.6, [0.3, 0.24, 0.2], [-0.35, v, 0])
        c["sectionFaces"] = ALL6
        s.th(c, "wall-plate", "plate-concrete")
        s.thof(f"th-pad-{nm}", f"pad-{nm}", "wall-plate", "plate-pad")
        s.thof(f"th-plate-{nm}", f"plate-{nm}", "wall-plate", "plate-steel")
    s.thof("th-beam", "beam", "canopy-steel")
    for k in range(4):
        s.thof(f"th-glass-{k}", f"glass-{k}", "canopy-glass")
        s.thermal[-1]["sectionFaces"] = ["-v"]
    for i in ("strut-tube", "cheek-beam-l", "cheek-beam-r", "cheek-wall-l", "cheek-wall-r", "pin-beam", "pin-beam-2", "pin-wall"):
        s.thof(f"th-{i}", i, "rod-steel")

    s.stof("m-beam", "beam", CB, "member")
    s.st(tube_wall("m-strut", E_END, TUBE_LEN, 0.0285, 0.0253), CB, "member")
    for i in ("cheek-beam-l", "cheek-beam-r", "cheek-wall-l", "cheek-wall-r", "pin-beam", "pin-beam-2", "pin-wall", "plate-beam", "plate-strut"):
        s.stof(f"j-{i}", i, CB, "joint")
    for nm, v in (("beam", 0), ("strut", 2.88)):
        for sv, tag in ((1, "top"), (-1, "bottom")):
            s.st(cyl(f"j-rod-{nm}-{tag}", CB, ST, 0.35, "x", [-0.225, v + sv * 0.085, -0.062], 0.45, 0.008, repeat={"count": 2, "translation": [0, 0, 0.124]}), CB, "joint")
    s.st(cyl("support-wall-pin", CB, ST, 0.3, "z", [-0.008, 0, 0], 0.1, 0.015), CB, "support")
    d = revolve("support-tie-rod", CB, ST, 0.3, [0, 0, 1], [4, 0, -0.045], [[0, 0.013], [0, 0.02], [0.09, 0.02], [0.09, 0.013], [0, 0.013]])
    s.st(d, CB, "support")
    s.dec("Body: the tied canopy as built. Wall stub of the three layers, concrete 300 mm with two reinforcement mats (12 mm bars, 150 mm grid, cover 50 mm, hidden), mineral wool 180 mm and render 20 mm, both with a cut-out round each anchor. Two anchor blocks (separation pad 180 mm x 240 x 200 mm) with the 20 mm steel end plate and four hex nuts of the anchor bolts on its face. The IPE 160 beam is one extrusion of the true section (82 x 160 mm, flange 7.4, web 5, root radius 9 mm, 20.2 cm2) 5.5 m long, welded to the end plate; on its top flange lie four 20 mm laminated glass sheets (1.28 x 1.0 m, 60 mm joints) of which the one the strut passes through has a slot. The strut is a 60.3 mm tube; at the beam end two 8 mm cheek plates outside the flange tips carry the 24 mm node pin through the web and a 20 mm pin through the tube end, at the wall end two 8 mm cheeks on the end plate carry one pin through the tube.",
          "Körper: das abgespannte Vordach wie gebaut. Wandausschnitt aus den drei Schichten, Beton 300 mm mit zwei Bewehrungsmatten (Stäbe 12 mm, Raster 150 mm, Überdeckung 50 mm, verdeckt), Mineralwolle 180 mm und Putz 20 mm, beide mit Aussparung um jeden Anker. Zwei Ankerblöcke (Trennlage 180 mm x 240 x 200 mm) mit der 20-mm-Stahlkopfplatte und vier Sechskantmuttern der Ankerbolzen auf ihrer Fläche. Der Träger IPE 160 ist eine Extrusion des wahren Profils (82 x 160 mm, Flansch 7,4, Steg 5, Ausrundung 9 mm, 20,2 cm2) von 5,5 m, an die Kopfplatte geschweißt; auf seinem Obergurt liegen vier Verbundglasscheiben von 20 mm (1,28 x 1,0 m, Fugen 60 mm), von denen die vom Stab durchdrungene einen Schlitz hat. Der Stab ist ein Rohr 60,3 mm; am Trägerende halten zwei Wangenbleche 8 mm außerhalb der Flanschenden den Knotenbolzen 24 mm durch den Steg und einen Bolzen 20 mm durch das Rohrende, am Wandende zwei Wangen 8 mm auf der Kopfplatte einen Bolzen durch das Rohr.")
    s.dec("Thermal: the wall region is three stacked layer slabs of their real thickness (concrete 300 / wool 180 / render 20 mm: the render slab is the exposed one and carries the cut edges as section faces); the anchor region is stacked the same way (concrete column behind the pad, pad, steel plate); beam, strut with its cheeks and pins and the four glass sheets (underside as section face, the sheet lies on the flange) reference the body.",
          "Thermik: die Wandregion besteht aus drei gestapelten Schichtplatten in ihrer wirklichen Dicke (Beton 300 / Wolle 180 / Putz 20 mm: die Putzplatte ist die freie und trägt die Schnittkanten als Schnittflächen); die Ankerregion ist gleich gestapelt (Betonsäule hinter der Trennlage, Trennlage, Stahlplatte); Träger, Stab mit Wangen und Bolzen sowie die vier Glasscheiben (Unterseite als Schnittfläche, die Scheibe liegt auf dem Flansch) verweisen auf den Körper.")
    s.dec("Structure: the IPE 160 as the member of the canopy bone, the strut as a hollow tube 60.3 x 3.2 mm (wall inside the body skin so that the exposed area stays), the cheeks, pins and end plates as joints, four 16 mm anchor bolts per plate reaching 450 mm into the concrete (hidden in the pad and the wall); supports by type: a 30 mm hinge pin through the end plate at the wall (pin), a bushing round the node pin (roller, the strut line of action).",
          "Tragwerk: der IPE 160 als Stab des Vordachknochens, der Rohrstab als Hohlrohr 60,3 x 3,2 mm (Wand innerhalb der Körperhaut, damit die freie Fläche bleibt), Wangen, Bolzen und Kopfplatten als Gelenke, vier Ankerbolzen 16 mm je Platte, 450 mm tief im Beton (verdeckt in Trennlage und Wand); Auflager nach Typ: ein Gelenkbolzen 30 mm durch die Kopfplatte an der Wand (Gelenk), eine Buchse um den Knotenbolzen (Rolle, die Wirkungslinie des Stabs).")
    return s


def build100():
    s = Scale(100)
    s.b(box("wall-core", CB, CO, 0.6, [0.3, 5.35, 1], [-0.35, 0.345, 0]))
    h = lambda v: [[v - 0.12, -0.1], [v - 0.12, 0.1], [v + 0.12, 0.1], [v + 0.12, -0.1]]
    s.b(ext("wall-insulation", CB, WO, 0.9, "yz", WALL_PROFILE, -0.2, 0, holes=[h(0), h(2.88)]))
    for nm, v in (("beam", 0), ("strut", 2.88)):
        s.b(box(f"plate-{nm}", CB, PD, 0.7, [0.2, 0.24, 0.2], [-0.1, v, 0]))
    s.b(ext("beam", CB, ST, 0.4, "yz", ipe(n=2), 0, 5.5))
    for k in range(4):
        u0 = 0.1 + 1.34 * k
        s.b(sheet(f"glass-{k}", u0, u0 + 1.28, notch=(3.77, 0.055) if k == 2 else None))
    for sd, z in (("l", 1), ("r", -1)):
        z0, z1 = (0.042, 0.057) if z > 0 else (-0.057, -0.042)
        s.b(ext(f"cheek-beam-{sd}", CB, ST, 0.3, "xy", CHEEK_BEAM, z0, z1))
        z0, z1 = (0.032, 0.047) if z > 0 else (-0.047, -0.032)
        s.b(ext(f"cheek-wall-{sd}", CB, ST, 0.3, "xy", CHEEK_WALL, z0, z1))
    s.b(pin("pin-beam", A, 0.012, 0.057))
    s.b(pin("pin-wall", P_WALL, 0.01, 0.047))
    s.b(tube("strut-tube", E_END, TUBE_LEN, R_TUBE))

    s.thof("th-wall-core", "wall-core", "wall-face")
    s.thermal[-1]["sectionFaces"] = ALL6
    s.thof("th-wall-insulation", "wall-insulation", "wall-face")
    s.thermal[-1]["sectionFaces"] = ["-u"] + EDGES
    for nm in ("beam", "strut"):
        s.thof(f"th-plate-{nm}", f"plate-{nm}", "wall-plate")
    s.thof("th-beam", "beam", "canopy-steel")
    for k in range(4):
        s.thof(f"th-glass-{k}", f"glass-{k}", "canopy-glass")
        s.thermal[-1]["sectionFaces"] = ["-v"]
    for i in ("strut-tube", "cheek-beam-l", "cheek-beam-r", "cheek-wall-l", "cheek-wall-r", "pin-beam", "pin-wall"):
        s.thof(f"th-{i}", i, "rod-steel")

    s.stof("m-beam", "beam", CB, "member")
    s.st(tube_wall("m-strut", E_END, TUBE_LEN, 0.0285, 0.0253), CB, "member")
    for i in ("cheek-beam-l", "cheek-beam-r", "cheek-wall-l", "cheek-wall-r", "pin-beam", "pin-wall", "plate-beam", "plate-strut"):
        s.stof(f"j-{i}", i, CB, "joint")
    for nm, v in (("beam", 0), ("strut", 2.88)):
        s.st(cyl(f"j-rod-{nm}", CB, ST, 0.35, "x", [-0.225, v + 0.085, -0.062], 0.45, 0.008, repeat={"count": 2, "translation": [0, 0, 0.124]}), CB, "joint")
    s.st(cyl("support-wall-pin", CB, ST, 0.3, "z", [-0.008, 0, 0], 0.1, 0.02), CB, "support")
    s.st(cyl("support-tie-rod", CB, ST, 0.3, "z", [4, 0, 0], 0.09, 0.02), CB, "support")
    s.dec("Body: the same canopy at 1:100. The wall is the concrete core and one 200 mm outer slab (wool and render merged, the 2D draws them as one) with the two anchor blocks in it (pad and end plate merged into one 200 x 240 x 200 mm block, as the 2D draws them). The IPE 160 keeps its true section with the root radius cut to two segments; the four glass sheets with the strut slot; the strut tube with the two 15 mm cheek plates at each end (they overlap the flange tips by 6 mm, the weld) and one pin per end, the 20 mm cross pin at the beam end and the anchor nuts are merged away. The reinforcement (12 mm bars) is below the 15 mm feature and left out.",
          "Körper: dasselbe Vordach bei 1:100. Die Wand ist der Betonkern und eine äußere Platte von 200 mm (Wolle und Putz zusammengefasst, die 2D zeichnet sie als eine) mit den beiden Ankerblöcken darin (Trennlage und Kopfplatte zu einem Block 200 x 240 x 200 mm zusammengefasst, wie ihn die 2D zeichnet). Der IPE 160 behält sein wahres Profil, die Ausrundung ist auf zwei Segmente gekürzt; die vier Glasscheiben mit dem Schlitz für den Stab; das Stabrohr mit je zwei Wangenblechen von 15 mm an beiden Enden (sie überdecken die Flanschenden um 6 mm, die Schweißnaht) und je einem Bolzen, der Querbolzen von 20 mm am Trägerende und die Ankermuttern sind zusammengefasst. Die Bewehrung (Stäbe 12 mm) liegt unter dem Merkmal von 15 mm und fehlt.")
    s.dec("Thermal: references to the body; the concrete core carries all faces as section faces and the outer slab its cut edges, so the wall region is the exposed outer face only; each glass sheet has its underside as section face. The layers of the wall and the anchor region are merged at this scale (B2 asks for the shells at 1:50 only).",
          "Thermik: Verweise auf den Körper; der Betonkern trägt alle Flächen als Schnittflächen und die äußere Platte ihre Schnittkanten, so dass die Wandregion nur die freie Außenfläche ist; jede Glasscheibe hat ihre Unterseite als Schnittfläche. Die Schichten von Wand und Ankerregion sind in diesem Maßstab zusammengefasst (B2 verlangt die Schalen nur bei 1:50).")
    s.dec("Structure: beam as member (the IPE body), the strut as a hollow tube, cheeks, pins and the two anchor blocks as joints, one pair of 16 mm anchor bolts per block, supports as a hinge pin at the wall and a 40 mm roller at the node.",
          "Tragwerk: Träger als Stab (der IPE-Körper), der Rohrstab als Hohlrohr, Wangen, Bolzen und die beiden Ankerblöcke als Gelenke, je ein Paar Ankerbolzen 16 mm je Block, Auflager als Gelenkbolzen an der Wand und Rolle 40 mm am Knoten.")
    return s


def build200():
    s = Scale(200)
    s.b(box("wall-core", CB, CO, 0.6, [0.3, 5.35, 1], [-0.35, 0.345, 0]))
    s.b(box("wall-outer", CB, WO, 0.8, [0.2, 5.35, 1], [-0.1, 0.345, 0]))
    s.b(ext("beam", CB, ST, 0.4, "yz", ipe(n=1), 0, 5.5))
    u_cross = A[0] + D[0] * (0.09 / D[1]) * 0 - (0.09 / 0.72)
    s.b(sheet("glass-roof", 0.1, 5.4, hole=(3.86, 0.075, 0.045)))
    s.b(tube("strut-tube", A, LEN, R_TUBE, slender=True))

    s.thof("th-wall-core", "wall-core", "wall-face")
    s.thermal[-1]["sectionFaces"] = ALL6
    s.thof("th-wall-outer", "wall-outer", "wall-face")
    s.thermal[-1]["sectionFaces"] = ["-u"] + EDGES
    for nm, v in (("beam", 0), ("strut", 2.88)):
        s.th(box(f"th-plate-{nm}", CB, PD, 0.7, [0.2, 0.24, 0.2], [-0.1, v, 0]), "wall-plate")
    s.thof("th-beam", "beam", "canopy-steel")
    s.thof("th-glass-roof", "glass-roof", "canopy-glass")
    s.thermal[-1]["sectionFaces"] = ["-v"]
    s.thof("th-strut-tube", "strut-tube", "rod-steel")

    s.stof("m-beam", "beam", CB, "member")
    s.st(tube_wall("m-strut", A, LEN, 0.0285, 0.0253, slender=True), CB, "member")
    s.st(cyl("support-wall-pin", CB, ST, 0.3, "z", [-0.008, 0, 0], 0.1, 0.03), CB, "support")
    s.st(cyl("support-tie-rod", CB, ST, 0.3, "z", [4, 0, 0], 0.09, 0.03), CB, "support")
    s.dec("Body: the wall is the concrete core and one 200 mm outer slab (anchor blocks inside, not drawn at this scale); the IPE 160 keeps its true section (82 x 160 mm, root radius a single chamfer); one 20 mm glass slab 5.3 x 1.0 m with the elliptical hole the strut passes through; the strut is the true 60.3 mm tube from node to node, `slender` (82 diameters long, the 2D draws it as a line); cheeks, pins, nuts, reinforcement and the glass joints are below the 30 mm feature and merged into the masses.",
          "Körper: die Wand ist der Betonkern und eine äußere Platte von 200 mm (Ankerblöcke darin, in diesem Maßstab nicht gezeichnet); der IPE 160 behält sein wahres Profil (82 x 160 mm, Ausrundung als eine Fase); eine Glasplatte 20 mm, 5,3 x 1,0 m mit dem elliptischen Loch, durch das der Stab geht; der Stab ist das wahre Rohr 60,3 mm von Knoten zu Knoten, `slender` (82 Durchmesser lang, die 2D zeichnet ihn als Linie); Wangen, Bolzen, Muttern, Bewehrung und die Glasfugen liegen unter dem Merkmal von 30 mm und sind in die Massen übergegangen.")
    s.dec("Thermal: references to the body with section faces on the concrete core and the cut edges of the outer slab and the glass underside; the anchor region is two real 200 x 240 x 200 mm blocks inside the outer slab (their exposed faces add 2 % to the total, which alone is compared at this scale).",
          "Thermik: Verweise auf den Körper mit Schnittflächen am Betonkern, den Schnittkanten der äußeren Platte und der Glasunterseite; die Ankerregion sind zwei wirkliche Blöcke 200 x 240 x 200 mm in der äußeren Platte (ihre freien Flächen erhöhen die Summe um 2 %, die in diesem Maßstab allein verglichen wird).")
    s.dec("Structure: beam as member, the strut as a hollow tube, the node joints are merged into the strut ends (a 40 mm ball would be a second hinge solid more than the strut ends carry), supports as a hinge pin at the wall and a roller at the node, both 60 mm thick.",
          "Tragwerk: Träger als Stab, der Rohrstab als Hohlrohr, die Knotengelenke gehen in die Stabenden über, Auflager als Gelenkbolzen an der Wand und Rolle am Knoten, beide 60 mm dick.")
    return s


def build500():
    s = Scale(500)
    s.b(box("wall", CB, CO, 0.7, [0.5, 5.35, 1], [-0.25, 0.345, 0]))
    s.b(ext("beam", CB, ST, 0.4, "yz", ipe(n=1), 0, 5.5))
    s.b(sheet("roof", 0.1, 5.4, hole=(3.86, 0.075, 0.045)))
    s.b(tube("strut-tube", A, LEN, R_TUBE, slender=True))

    s.thof("th-wall", "wall", "wall-face")
    s.thermal[-1]["sectionFaces"] = LAYER_CUT
    s.thof("th-wall-anchors", "wall", "wall-plate")
    s.thermal[-1]["sectionFaces"] = ALL6
    s.thof("th-beam", "beam", "canopy-steel")
    s.thof("th-roof", "roof", "canopy-glass")
    s.thermal[-1]["sectionFaces"] = ["-v"]
    s.thof("th-strut-tube", "strut-tube", "rod-steel")

    s.stof("m-beam", "beam", CB, "member")
    s.st(tube_wall("m-strut", A, LEN, 0.0285, 0.0253, slender=True), CB, "member")
    s.st(box("support-wall-pin", CB, ST, 0.3, [0.075, 0.075, 0.1], [-0.02, 0, 0]), CB, "support")
    s.st(box("support-tie-rod", CB, ST, 0.3, [0.075, 0.075, 0.09], [4, 0, 0]), CB, "support")
    s.dec("Body: four masses. The 500 mm wall block (concrete and insulation merged), the IPE 160 beam of its true section (82 x 160 mm, the thinnest part still above the 75 mm feature), the 20 mm glass roof as a glazing slab with the strut hole, and the strut as the true 60.3 mm tube (`slender`, the 2D draws it as a line).",
          "Körper: vier Massen. Der Wandblock 500 mm (Beton und Dämmung zusammengefasst), der Träger IPE 160 in wahrem Profil (82 x 160 mm, der dünnste Teil liegt noch über dem Merkmal von 75 mm), das Glasdach 20 mm als Glasplatte mit dem Loch für den Stab und der Stab als wahres Rohr 60,3 mm (`slender`, die 2D zeichnet ihn als Linie).")
    s.dec("Thermal: references to the body; the anchor region references the wall block with all faces as section faces (the anchors lie inside it and are not drawn at this scale; the region only stays covered).",
          "Thermik: Verweise auf den Körper; die Ankerregion verweist auf den Wandblock mit allen Flächen als Schnittflächen (die Anker liegen darin und sind in diesem Maßstab nicht gezeichnet; die Region bleibt nur belegt).")
    s.dec("Structure: beam and strut as members, supports as 75 mm blocks (hinge pin and roller); joints and anchor bolts are merged into the masses.",
          "Tragwerk: Träger und Stab als Stäbe, Auflager als Blöcke 75 mm (Gelenkbolzen und Rolle); Gelenke und Ankerbolzen sind in die Massen übergegangen.")
    return s


B = {50: build50, 100: build100, 200: build200, 500: build500}
save("components", "tied-canopy", [B[k]() for k in only])
