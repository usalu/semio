import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("c26", os.path.join(here, "author-26-common.py"))
c = importlib.util.module_from_spec(spec); spec.loader.exec_module(c)
V = c.Volume("sorption-wheel")
CY = 0.31
GS = dict(material="galvanised-steel", role="body")
SG = dict(material="silica-gel", role="body")
SGM = dict(material="silica-gel")
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}
HALF = math.radians(42)


def arc(cx, cy, r, a0, a1, n):
    return [(cx + r * math.cos(a0 + (a1 - a0) * i / n), cy + r * math.sin(a0 + (a1 - a0) * i / n)) for i in range(n + 1)]


def circle(cx, cy, r, n=48):
    return arc(cx, cy, r, 0, 2 * math.pi, n)[:-1]


def sector(r, n=16):
    return [(0, CY)] + [(x, y + CY) for x, y in arc(0, 0, r, -HALF, HALF, n)]


def clip(subject, clipper):
    out = subject
    m = len(clipper)
    for i in range(m):
        a, b = clipper[i], clipper[(i + 1) % m]
        inp, out = out, []
        if not inp:
            break
        side = lambda p: (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])
        for j in range(len(inp)):
            p, q = inp[j], inp[(j + 1) % len(inp)]
            sp, sq = side(p), side(q)
            if sp >= 0:
                out.append(p)
            if (sp >= 0) != (sq >= 0):
                t = sp / (sp - sq)
                out.append((p[0] + t * (q[0] - p[0]), p[1] + t * (q[1] - p[1])))
    return out


def dedupe(poly, tol=2e-4):
    out = []
    for p in poly:
        if not out or math.hypot(p[0] - out[-1][0], p[1] - out[-1][1]) > tol:
            out.append(p)
    while len(out) > 1 and math.hypot(out[0][0] - out[-1][0], out[0][1] - out[-1][1]) <= tol:
        out.pop()
    return out


def ccw(poly):
    poly = dedupe(poly)
    a = sum(poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1] for i in range(len(poly)))
    return poly if a > 0 else poly[::-1]


def casing(b, r_in_flange, wall, flange_t, band_in):
    z1, z2 = 0.09, 0.11
    prof = [[-z2, r_in_flange], [-z2, 0.31], [z2, 0.31], [z2, r_in_flange], [z1, r_in_flange], [z1, band_in], [-z1, band_in], [-z1, r_in_flange]]
    b.rev("casing", "casing", [0, CY, 0], [0, 0, 1], prof + [prof[0]], tone=0.35, **GS)


def stand(b, half_bottom, half_top, h, zh=0.09):
    b.ext("stand", "casing", "side", [[-half_bottom, 0], [half_bottom, 0], [half_top, h], [-half_top, h]], -zh, zh, tone=0.35, **GS)


def seals(b, r, z0, z1):
    pts = [list(p) for p in sector(r)]
    b.ext("regen-seal-front", "seal", "side", pts, z0, z1, tone=0.9, **GS)
    b.ext("regen-seal-rear", "seal", "side", pts, -z1, -z0, tone=0.9, **GS)


def body50():
    b = V.at(50, "body")
    casing(b, 0.27, 0.02, 0.02, 0.29)
    stand(b, 0.12, 0.10, 0.03)
    b.rev("disc", "rotor", [0, CY, 0], [0, 0, 1], [[-0.09, 0.045], [-0.09, 0.285], [0.09, 0.285], [0.09, 0.045], [-0.09, 0.045]], tone=0.6, **SG)
    b.cyl("hub", "rotor", [0, CY, -0.09], [0, CY, 0.09], 0.045, tone=0.4, **GS)
    for n, z in (("front", 0.09), ("rear", -0.095)):
        b.box(f"spoke-{n}", "rotor", [0.165, CY, z + 0.005], [0.24, 0.01, 0.01], rep={"count": 8, "angle": math.pi / 4, "axis": [0, 0, 1], "origin": [0, CY, 0]}, tone=0.2, role="detail", slender=True, **SGM)
    seals(b, 0.27, 0.095, 0.105)


def body100():
    b = V.at(100, "body")
    casing(b, 0.27, 0.02, 0.02, 0.29)
    stand(b, 0.14, 0.11, 0.035)
    b.rev("disc", "rotor", [0, CY, 0], [0, 0, 1], [[-0.09, 0.045], [-0.09, 0.275], [0.09, 0.275], [0.09, 0.045], [-0.09, 0.045]], tone=0.6, **SG)
    b.cyl("hub", "rotor", [0, CY, -0.09], [0, CY, 0.09], 0.045, tone=0.4, **GS)
    for n, z in (("front", 0.0875), ("rear", -0.0875)):
        b.box(f"spoke-{n}", "rotor", [0.165, CY, z], [0.22, 0.015, 0.015], rep={"count": 4, "angle": math.pi / 2, "axis": [0, 0, 1], "origin": [0, CY, 0]}, tone=0.2, role="detail", texture=True, **SGM)
    seals(b, 0.27, 0.095, 0.11)


def body200():
    b = V.at(200, "body")
    prof = [[-0.11, 0.245], [-0.11, 0.31], [0.11, 0.31], [0.11, 0.245], [0.08, 0.245], [0.08, 0.28], [-0.08, 0.28], [-0.08, 0.245], [-0.11, 0.245]]
    b.rev("casing", "casing", [0, CY, 0], [0, 0, 1], prof, tone=0.35, **GS)
    stand(b, 0.15, 0.12, 0.04)
    b.rev("disc", "rotor", [0, CY, 0], [0, 0, 1], [[-0.08, 0], [-0.08, 0.27], [0.08, 0.27], [0.08, 0], [-0.08, 0]], tone=0.6, **SG)
    seals(b, 0.24, 0.0825, 0.1125)


def lens(center, r, bound, rr_clip, z0, z1, region, emitter, tag, bone="casing", t_sign=1):
    pass


def thermal(sc):
    t = V.at(sc, "thermal")
    ids = {s["id"] for s in V.data[sc]["body"]}
    for reg, lst in (("casing", ["casing", "stand"]), ("disc", ["disc", "spoke-front", "spoke-rear"]), ("regen", ["regen-seal-front", "regen-seal-rear"]), ("hub", ["hub"])):
        for i in lst:
            if i in ids:
                t.ref(f"th-{i}", i, region=reg)
    f = FEATURE[sc] * 1.0001
    rc = 0.27 if sc < 200 else 0.24
    disc = ccw(circle(0, CY, rc, 48))
    zo = 0.11
    proc = ccw(clip(circle(0.2, CY - 0.19, 0.2, 48), disc))
    t.ext("th-process-air-outlet", "casing", "side", [list(p) for p in proc], -zo - f + 0.0005, -zo + 0.0005, region="disc", tags=["process-air"], sectionFaces=["-w", "+w"], tone=0.9, **GS)
    sec = ccw(sector(rc))
    reg = ccw(clip(clip(circle(0.27, CY, 0.125, 48), disc), sec))
    t.ext("th-regen-air-outlet", "casing", "side", [list(p) for p in reg], zo - 0.0005, zo + f - 0.0005, region="regen", tags=["regen-air"], sectionFaces=["-w", "+w"], tone=0.9, **GS)
    if sc == 200:
        t.cyl("th-hub", "rotor", [0, CY, -0.06], [0, CY, 0.06], 0.045, region="hub", tone=0.4, **GS)


def structure(sc):
    r = V.at(sc, "structure")
    ids = {s["id"] for s in V.data[sc]["body"]}
    for i, m in (("casing", "casing"), ("stand", "casing"), ("disc", "rotor"), ("hub", "rotor"), ("spoke-front", "rotor"), ("spoke-rear", "rotor"), ("regen-seal-front", "seal"), ("regen-seal-rear", "seal")):
        if i in ids:
            r.ref(f"st-{i}", i, member=m, part="member")
    f = FEATURE[sc]
    sw = 0.03 if sc < 200 else 0.04
    zt = 0.02 if sc < 200 else 0.03
    r.box("spider-vertical", "casing", [0, CY, -0.1], [sw, 0.58, zt], tone=0.35, member="casing", part="member", **GS)
    r.box("spider-horizontal", "casing", [0, CY, -0.1], [0.58, sw, zt], tone=0.35, member="casing", part="member", **GS)
    if sc < 200:
        r.cyl("joint-axle", "rotor", [0, CY, -0.11], [0, CY, 0.09], 0.02 if sc == 50 else 0.0225, tone=0.4, member="rotor", part="joint", **GS)
    ms = {50: ([0.1, 0.1, 0.1], [0.12, 0.12, 0.12], [0.06, 0.06, 0.06]), 100: ([0.1, 0.1, 0.12], [0.12, 0.12, 0.1], [0.06, 0.06, 0.06]), 200: ([0.1, 0.1, 0.1], [0.14, 0.14, 0.14], [0.06, 0.06, 0.06])}[sc]
    r.box("mass-casing", "casing", [0, CY, 0.0], ms[0], tone=0.35, member="casing", part="mass", **GS)
    r.box("mass-rotor", "rotor", [0, CY, 0.0], ms[1], tone=0.6, member="rotor", part="mass", **SG)
    r.box("mass-seal", "seal", [0.15, CY, 0.0], ms[2], tone=0.9, member="seal", part="mass", **GS)
    sh = {50: (0.24, 0.18), 100: (0.28, 0.18), 200: (0.3, 0.18)}[sc]
    r.box("base-contact-pad", "casing", [0, f / 2, 0], [sh[0] * 0.9, f, sh[1]], tone=0.35, member="casing", part="support", anchor="base-contact", **GS)


body50(); thermal(50); structure(50)
body100(); thermal(100); structure(100)
body200(); thermal(200); structure(200)

V.decide(50, "Detailed model with the wheel axis along the depth: the casing is a revolved C-section drum (20 mm band at radius 0.29-0.31 m, two 20 mm flange rings that retain the wheel from radius 0.27 m, 0.22 m deep) on a trapezoidal stand block of 0.18 m width; the sorption matrix is a 0.57 m by 0.18 m annulus around a 45 mm hub (flush with both faces, so its rim is buried in the matrix) with 5 mm clearance to the band; eight radial 10 mm spokes per face are slender solids half embedded in the faces (their drawing is open strokes); the hot regeneration sector is a 10 mm seal plate of 84 degrees on each face on the fixed seal bone, lifted 5 mm off the faces so that spokes pass under it. The honeycomb of 20 mm cells is drawn as open strokes and is not a solid. The wheel is held by its face contact with the two flange rings.",
         "Detailliertes Modell mit der Radachse entlang der Tiefe: Das Gehäuse ist eine gedrehte C-Trommel (20-mm-Mantel bei Radius 0,29 bis 0,31 m, zwei 20-mm-Flanschringe, die das Rad ab Radius 0,27 m halten, 0,22 m tief) auf einem trapezförmigen Fußblock von 0,18 m Breite; die Sorptionsmatrix ist ein Ring von 0,57 m Durchmesser und 0,18 m Tiefe um eine 45-mm-Nabe (bündig mit beiden Flächen, ihr Rand liegt in der Matrix vergraben) mit 5 mm Spiel zum Mantel; je Fläche acht radiale 10-mm-Speichen sind schlanke, zur Hälfte eingelassene Körper (ihre Zeichnung besteht aus offenen Strichen); der heiße Regenerationssektor ist je Fläche eine 10-mm-Dichtplatte von 84 Grad auf dem festen Knochen seal, 5 mm von den Flächen abgehoben, sodass die Speichen darunter laufen. Die Wabe aus Zellen von 20 mm ist als offene Striche gezeichnet und kein Körper. Das Rad wird durch den Flächenkontakt mit den beiden Flanschringen gehalten.")
V.decide(50, "Thermal: every region is a set of references to the body solids that carry its surface (casing: drum and stand; disc: matrix and spokes; regen: the two seal plates; hub); each region has one layer, so each is a single surface. The two vent emitters are one-feature plates just outside the faces: the process air outlet is the 0.4 m disc around its anchor cut to the matrix (rear face, region disc), the regeneration air outlet a 0.25 m disc cut to the seal sector (front face, region regen), both with their section faces declared, because an air outlet plane carries no surface. Structure: drum and stand are the members of the casing, matrix, hub and spokes members of the rotor, the seal plates members of the seal; a two-bar spider behind the wheel (inside the silhouette, not drawn in 2D) carries the 40 mm axle, the axle is the joint of the revolute rotor; three compact masses sit at the centres of casing, rotor and sector; the 0.22 m by 0.18 m contact pad carries the base-contact anchor.",
         "Thermik: Jede Region ist eine Menge von Verweisen auf die Körper, die ihre Oberfläche tragen (casing: Trommel und Fuß; disc: Matrix und Speichen; regen: die zwei Dichtplatten; hub: die Nabe); jede Region hat eine Schicht, also eine einfache Fläche. Die beiden Lüftungsemitter sind Platten von einer Einzelheit Dicke knapp außerhalb der Flächen: der Prozessluftaustritt ist die 0,4-m-Scheibe um ihren Anker, auf die Matrix zugeschnitten (Rückseite, Region disc), der Regenerationsluftaustritt eine 0,25-m-Scheibe, auf den Dichtsektor zugeschnitten (Vorderseite, Region regen); beide erklären ihre Schnittflächen, weil eine Austrittsebene keine Oberfläche trägt. Tragwerk: Trommel und Fuß sind die Stäbe des Gehäuses, Matrix, Nabe und Speichen Stäbe des Rotors, die Dichtplatten Stäbe von seal; eine Spinne aus zwei Stäben hinter dem Rad (innerhalb der Silhouette, in 2D nicht gezeichnet) trägt die 40-mm-Achse, die Achse ist das Gelenk des drehenden Rotors; drei kompakte Massen liegen in den Schwerpunkten von Gehäuse, Rotor und Sektor; die 0,22 m mal 0,18 m große Kontaktplatte trägt den Anker base-contact.")
V.decide(100, "Redrawn against the 1:100 shapes: the casing drum keeps its 20 mm band and flanges (already above 15 mm), the matrix is 0.55 m in diameter as drawn with 15 mm clearance to the band, four 15 mm spokes per face are texture solids (the 1:100 drawing has four strokes), the seal plates are 15 mm thick and flush with the flange faces, the stand is 0.28 m wide; the honeycomb of 1:50 and the four extra spokes vanish.",
         "Neu gezeichnet gegen die Formen 1:100: Die Gehäusetrommel behält 20 mm Mantel und Flansche (schon über 15 mm), die Matrix hat wie gezeichnet 0,55 m Durchmesser mit 15 mm Spiel zum Mantel, je Fläche vier 15-mm-Speichen sind Texturkörper (die Zeichnung 1:100 hat vier Striche), die Dichtplatten sind 15 mm dick und bündig mit den Flanschflächen, der Fuß ist 0,28 m breit; die Wabe von 1:50 und die vier zusätzlichen Speichen entfallen.")
V.decide(100, "Thermal and structure as at 1:50 on the 1:100 solids; the outlet plates are 15 mm thick, the axle is 45 mm, the spider bars 30 mm wide and 20 mm thick, three box masses, a 15 mm contact pad.",
         "Thermik und Tragwerk wie bei 1:50 auf den Körpern von 1:100; die Austrittsplatten sind 15 mm dick, die Achse 45 mm, die Spinnenstäbe 30 mm breit und 20 mm dick, drei Quadermassen, eine 15 mm dicke Kontaktplatte.")
V.decide(200, "Redrawn against the 1:200 shapes: the casing drum has a 30 mm band (radius 0.28-0.31 m) and 30 mm flange rings that clamp a 0.54 m by 0.16 m solid matrix (30 mm smallest feature: the 0.02 m face clearance of the real wheel becomes the flange thickness), the seal plates are 30 mm and protrude 2.5 mm beyond the casing faces, the stand is 0.3 m wide and 40 mm high; hub, spokes and honeycomb are not drawn at this scale (the 1:200 drawing merges the hub into the casing region).",
         "Neu gezeichnet gegen die Formen 1:200: Die Gehäusetrommel hat 30 mm Mantel (Radius 0,28 bis 0,31 m) und 30-mm-Flanschringe, die eine massive Matrix von 0,54 m mal 0,16 m einklemmen (kleinste Einzelheit 30 mm: das Spiel von 0,02 m an den Flächen des echten Rades wird zur Flanschdicke), die Dichtplatten sind 30 mm dick und stehen 2,5 mm über die Gehäuseflächen vor, der Fuß ist 0,3 m breit und 40 mm hoch; Nabe, Speichen und Wabe sind in diesem Maßstab nicht gezeichnet (die Zeichnung 1:200 führt die Nabe in der Gehäuseregion zusammen).")
V.decide(200, "Thermal: references to the body solids; the hub is a real 90 mm hidden cylinder inside the matrix (the body has none), the outlet plates are 30 mm. Structure: drum, stand, matrix and seal plates as members, a spider of 40 mm bars, no separate joints, three masses, a 30 mm contact pad.",
         "Thermik: Verweise auf die Körper; die Nabe ist ein echter verdeckter 90-mm-Zylinder in der Matrix (der Körper hat keinen), die Austrittsplatten sind 30 mm dick. Tragwerk: Trommel, Fuß, Matrix und Dichtplatten als Stäbe, eine Spinne aus 40-mm-Stäben, keine eigenen Gelenke, drei Massen, eine 30 mm dicke Kontaktplatte.")
V.write(unavailable=(500,))
