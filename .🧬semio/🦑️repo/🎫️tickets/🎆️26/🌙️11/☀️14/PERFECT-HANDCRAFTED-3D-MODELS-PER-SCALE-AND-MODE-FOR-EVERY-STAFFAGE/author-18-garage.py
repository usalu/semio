import sys, os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-18-lib.py")).read())
ST, SK, PU = "steel", "sandwich-skin", "pu-foam"
HW, TR, SP = "hardware-steel", "track-steel", "spring-assembly"
HALF = 1.21
only = [int(a) for a in sys.argv[1:]] or [50, 100, 200, 500]


def cprof(sign, wall):
    z0, z1 = (1.21, 1.25) if sign > 0 else (-1.25, -1.21)
    w = wall
    if sign > 0:
        return [[1.21, -0.02], [1.25, -0.02], [1.25, 0.02], [1.21, 0.02], [1.21, 0.02 - w], [1.25 - w, 0.02 - w], [1.25 - w, -0.02 + w], [1.21, -0.02 + w]]
    return [[-1.21, 0.02], [-1.25, 0.02], [-1.25, -0.02], [-1.21, -0.02], [-1.21, -0.02 + w], [-1.25 + w, -0.02 + w], [-1.25 + w, 0.02 - w], [-1.21, 0.02 - w]]


def rail_set(s, wall, arc_closed=False):
    for sd, sg in (("l", 1), ("r", -1)):
        pr = cprof(sg, wall)
        s.b(ext(f"rail-up-{sd}", "track", ST, 0.4, "zx", pr, 0, 2.0))
        axial = [[-p[0], 0.5 + p[1]] for p in pr]
        axial.append(axial[0])
        arc = revolve(f"rail-arc-{sd}", "track", ST, 0.4, [0, 0, -1], [0, 0, 0], axial, sweep=math.pi / 2)
        arc["transform"] = {"rotation": [0, 0, 1, 0], "translation": [0.5, 2.0, 0]}
        s.b(arc)
        pb = [[2.5 + p[1], p[0]] for p in pr]
        s.b(ext(f"rail-back-{sd}", "track", ST, 0.4, "yz", pb, 0.5, 2.6))


def hanger_set(s, tube=True):
    for sd, z in (("l", 1.23), ("r", -1.23)):
        outer = rect(z, 0, 0.03, 0.04)
        d = ext(f"hanger-{sd}", "track", ST, 0.4, "zx", outer, 2.5, 2.85, holes=[rect(z, 0, 0.024, 0.034)] if tube else None, at=[0, 0, 0])
        d["transform"] = {"translation": [1.3, 0, 0]}
        d["repeat"] = {"count": 2, "translation": [1.2, 0, 0]}
        s.b(d)


def end_bar(s):
    s.b(box("track-end-bar", "track", ST, 0.4, [0.02, 0.04, 2.42], [2.59, 2.5, 0]))


def bearing_plate(s, n=24):
    cx, cy, r = 0.12, 2.75, 0.095
    a = math.asin(0.04 / r)
    arc = arc_pts(cx, cy, r, -(math.pi / 2 - a), math.pi + (math.pi / 2 - a), n)
    prof = [[0.16, 2.30]] + arc + [[0.08, 2.30]]
    s.b(ext("bearing-plate", "track", ST, 0.4, "xy", prof, 1.25, 1.26))
    mirror = ext("bearing-plate-r", "track", ST, 0.4, "xy", prof, -1.26, -1.25)
    s.b(mirror)


def panel_profile(i, chamfer):
    lo = 0.07 if i == 1 else 0.0
    c = chamfer
    p = [[lo, -0.02], [0.5, -0.02], [0.5, 0.02 - c], [0.5 - c, 0.02], [lo + c if i > 1 else lo, 0.02]]
    if i > 1:
        p.append([0.0, 0.02 - c])
    return p


def panels(s, chamfer, tones=(0.55, 0.55, 0.55, 0.55)):
    for i in range(1, 5):
        pr = panel_profile(i, chamfer)
        s.b(ext(f"panel-{i}", f"panel-{i}", SK, tones[i - 1], "xy", pr, -HALF, HALF, role="body"))


def seal(s, detail=True):
    if detail:
        pr = [[0, -0.012], [0.004, -0.02], [0.012, -0.022], [0.04, -0.022], [0.04, -0.02], [0.07, -0.02], [0.07, 0.02], [0.04, 0.02], [0.04, 0.022], [0.012, 0.022], [0.004, 0.02], [0, 0.012]]
    else:
        pr = [[0, -0.021], [0.07, -0.021], [0.07, 0.021], [0, 0.021]]
    s.b(ext("bottom-seal", "panel-1", ST, 0.15, "xy", pr, -HALF, HALF, role="detail"))


def roller_pairs(s, bone, idn, x, prof):
    for sd, z, ax in (("l", 1.205, 1), ("r", -1.205, -1)):
        s.b(revolve(f"roller-{idn}-{sd}", bone, ST, 0.2, [0, 0, ax], [x, 0, z], prof, role="detail"))


ROLLER_FINE = [[0, 0.004], [0.01, 0.004], [0.01, 0.0105], [0.012, 0.011], [0.028, 0.011], [0.03, 0.0105], [0.03, 0]]
ROLLER_COARSE = [[0, 0], [0, 0.011], [0.03, 0.011], [0.03, 0]]


def spring_parts(s, fine=True, coil_mode="wave"):
    ax = [0, 0, 1]
    if fine:
        s.b(revolve("shaft", "spring-shaft", ST, 0.2, ax, [0, 0, -1.255], [[0, 0], [0, 0.0125], [2.51, 0.0125], [2.51, 0]], role="detail"))
        for sd, z in (("l", 1.1), ("r", -1.2)):
            prof = [[0, 0], [0, 0.09], [0.012, 0.09], [0.012, 0.082], [0.088, 0.082], [0.088, 0.09], [0.1, 0.09], [0.1, 0]]
            s.b(revolve(f"drum-{sd}", "spring-shaft", ST, 0.5, ax, [0, 0, z], prof, role="body"))
    if coil_mode == "wave":
        loop = [[0.0, 0.034 + 0.004 * math.sin(2 * math.pi * k / 16)] for k in range(16)]
        loop = [[0.004 * math.cos(2 * math.pi * k / 16), 0.034 + 0.004 * math.sin(2 * math.pi * k / 16)] for k in range(16)]
        loop.append(loop[0])
        d = revolve("coil", "spring-shaft", ST, 0.4, ax, [0, 0, -0.4959], loop, role="detail")
        d["repeat"] = {"count": 122, "translation": [0, 0, 0.0082]}
        s.b(d)
    elif coil_mode == "tube":
        s.b(revolve("coil", "spring-shaft", ST, 0.4, ax, [0, 0, -0.5], [[0, 0], [0, 0.0375], [1.0, 0.0375], [1.0, 0]], role="detail"))


def build50():
    s = Scale(50)
    rail_set(s, 0.002)
    end_bar(s)
    hanger_set(s)
    bearing_plate(s)
    panels(s, 0.005)
    seal(s, True)
    for i in range(1, 5):
        s.b(box(f"panel-rib-{i}", f"panel-{i}", SK, 0.5, [0.006, 0.003, 2.22], [0.25, 0.0215, 0], role="texture", tags=["texture"]))
    s.body[-4:] = [dict(d, texture=True) for d in s.body[-4:]]
    for i in range(1, 5):
        roller_pairs(s, f"panel-{i}", i, 0.5, ROLLER_FINE)
    roller_pairs(s, "panel-1", 0, 0.0, ROLLER_FINE)
    for i in range(2, 5):
        up = box(f"hinge-leaf-up-{i}", f"panel-{i}", ST, 0.3, [0.05, 0.008, 0.12], [0.025, -0.024, -1.15], role="detail", repeat={"count": 4, "translation": [0, 0, 0.766667]})
        lo = box(f"hinge-leaf-low-{i}", f"panel-{i-1}", ST, 0.3, [0.05, 0.008, 0.12], [0.475, -0.024, -1.15], role="detail", repeat={"count": 4, "translation": [0, 0, 0.766667]})
        kn = cyl(f"hinge-knuckle-{i}", f"panel-{i}", ST, 0.25, "z", [0, -0.0335, -1.15], 0.12, 0.0115, role="detail")
        kn["transform"] = {"translation": [0, 0, 0]}
        kn["repeat"] = {"count": 4, "translation": [0, 0, 0.766667]}
        for d in (up, lo, kn):
            s.b(d)
    s.b(box("bottom-bracket", "panel-1", ST, 0.3, [0.112, 0.01, 0.12], [0.044, -0.025, 1.15], role="detail", repeat={"count": 2, "translation": [0, 0, -2.3]}))
    spring_parts(s)
    return s


def thermal_common(s, panel_lo_t=0.07):
    pass


def panel_layers(s, t_skin=0.0005, core=0.039):
    for i in range(1, 5):
        lo = 0.07 if i == 1 else 0.0
        ln = 0.5 - lo
        uc = (lo + 0.5) / 2
        o = 0.02 - t_skin / 2
        s.th(box(f"th-skin-outer-{i}", f"panel-{i}", SK, 0.55, [ln, t_skin, 2.42], [uc, o, 0]), "panel-sandwich", "skin-outer")
        s.th(box(f"th-core-{i}", f"panel-{i}", PU, 0.85, [ln, core, 2.42], [uc, 0, 0]), "panel-sandwich", "pu-core")
        s.th(box(f"th-skin-inner-{i}", f"panel-{i}", SK, 0.55, [ln, t_skin, 2.42], [uc, -o, 0]), "panel-sandwich", "skin-inner")


def th_refs(s, hardware, track, spring):
    for i in hardware:
        s.thof(f"th-{i}", i, HW, "hw-plate")
    for i in track:
        s.thof(f"th-{i}", i, TR, "track-wall")
    for i in spring:
        s.thof(f"th-{i}", i, SP, "coil-wire")


def hw_ids(s):
    return [d["id"] for d in s.body if d["id"].startswith(("roller", "hinge", "bottom"))]


def struct_common(s, hinge_ids, joints=True):
    for d in s.body:
        if d["id"].startswith(("rail-", "hanger", "track-end")):
            s.stof(f"m-{d['id']}", d["id"], "track", "member")
    for i in range(1, 5):
        s.stof(f"m-panel-{i}", f"panel-{i}", f"panel-{i}", "member")
    sh = ST
    s.st(box("support-jamb-fix", "track", sh, 0.4, [0.07, 0.2, 0.01], [0, 0.1, 1.255], repeat={"count": 2, "translation": [0, 0, -2.51]}), "track", "support")
    for n, x in ((1, 1.3), (2, 2.5)):
        s.st(cyl(f"support-hanger-{n}", "track", sh, 0.4, "z", [x, 2.835, 1.23], 0.05, 0.011, repeat={"count": 2, "translation": [0, 0, -2.46]}), "track", "support")


def struct_joints(s):
    for i in range(2, 5):
        s.stof(f"j-hinge-{i}", f"hinge-knuckle-{i}", f"panel-{i}", "joint")
    s.stof("j-roller-0-l", "roller-0-l", "panel-1", "joint")
    s.stof("j-roller-0-r", "roller-0-r", "panel-1", "joint")


def structure50(s):
    struct_common(s, None)
    struct_joints(s)
    s.stof("j-bearing-l", "bearing-plate", "spring-shaft", "joint")
    s.stof("j-bearing-r", "bearing-plate-r", "spring-shaft", "joint")
    s.st(cyl("mass-spring", "spring-shaft", ST, 0.4, "z", [0, 0, 0], 1.0, 0.029), "spring-shaft", "mass")


def finish50(s):
    s.thof  # noqa
    th_refs(s, [h for h in hw_ids(s) if h != "bottom-seal"], [d["id"] for d in s.body if d["id"].startswith(("rail-", "hanger", "track-end", "bearing"))], ["shaft", "drum-l", "drum-r", "coil"])
    panel_layers(s)
    s.thof("th-bottom-seal", "bottom-seal", "panel-sandwich", "skin-outer")
    structure50(s)
    s.dec("Body: the door as it is built. Four 40 mm sandwich panels as extrusions over the 2.42 m width with the 5 mm V-chamfer on the outer face that makes the joint groove, the bottom panel starting above its 70 mm seal profile (rubber bulb in an aluminium retainer), one embossed rib per panel (texture). Per joint five hinges (two leaves and a knuckle each, leaves on the two panels they join), a roller on each side of every joint plus the bottom roller on its bracket; both guide tracks as 40 x 40 mm C-sections with 3 mm walls (vertical, true quarter-circle R 0.5 m as a swept revolve, horizontal) with the end tie bar and hollow ceiling hangers, the spring shaft with two grooved cable drums and the wound torsion spring, and the two bearing flags (disc round the shaft, strap down to the curve) bolted to the outside of the tracks. Cable, handle and lock are not drawn at any scale and are not modelled.",
          "Körper: das Tor, wie es gebaut ist. Vier Sandwichpaneele zu 40 mm als Extrusionen über die Breite 2,42 m mit der 5-mm-V-Fase an der Außenseite, die die Fugennut bildet, das untere Paneel beginnt über seinem 70-mm-Dichtungsprofil (Gummiwulst im Aluminiumhalter), je Paneel eine geprägte Sicke (Textur). Je Fuge fünf Scharniere (zwei Laschen und ein Gelenkrohr, die Laschen auf den beiden verbundenen Paneelen), auf jeder Seite jeder Fuge eine Rolle sowie die untere Rolle auf ihrem Winkel; beide Führungsschienen als C-Profile 40 x 40 mm mit 3 mm Wand (senkrecht, echter Viertelkreis R 0,5 m als gedrehter Körper, waagerecht) mit Schienenabschluss und hohlen Deckenabhängern, die Federwelle mit zwei genuteten Seiltrommeln und der gewickelten Torsionsfeder sowie die beiden Lagerfahnen (Scheibe um die Welle, Lasche hinunter zum Bogen), außen an die Schienen geschraubt. Seil, Griff und Schloss sind in keinem Maßstab gezeichnet und nicht modelliert.")
    s.dec("Thermal: the sandwich as three stacked slabs per panel (outer skin 0.5 mm, PU core 39 mm, inner skin 0.5 mm; the core is 1 mm thinner than the 40 mm layer value so that the stack fits the drawn 40 mm panel), only the bottom panel is 70 mm shorter under its seal; hinges, rollers, bracket and seal are the hardware surface, tracks, hangers, end bar and bearing flags the track surface, shaft, drums and coil the spring surface, all referenced from the body because the thermal surface is exactly the body surface.",
          "Thermisch: das Sandwich als drei gestapelte Platten je Paneel (Außenhaut 0,5 mm, PU-Kern 39 mm, Innenhaut 0,5 mm; der Kern ist 1 mm dünner als der Schichtwert 40 mm, damit der Stapel in das gezeichnete 40-mm-Paneel passt), nur das untere Paneel ist unter der Dichtung 70 mm kürzer; Scharniere, Rollen, Winkel und Dichtung sind die Beschlagfläche, Schienen, Abhänger, Abschluss und Lagerfahnen die Schienenfläche, Welle, Trommeln und Feder die Federfläche, alle aus dem Körper referenziert, weil die thermische Fläche genau die Körperfläche ist.")
    s.dec("Structure: load path in space. The four panels are the beam members (wind carried over the 2.42 m width into the rollers), the tracks, arcs, back rails, tie bar and hangers are the members of the track frame, the hinge knuckles and the bottom roller the revolute joints, the bearing flags the bearing joints of the spring shaft, the wound spring a compact mass; supports by type: fixed jamb brackets (flat plates outside both rails at the floor), roller pins through the two ceiling hangers (two supports at 1.3 m and 2.5 m, one solid each across both sides).",
          "Tragwerk: der Lastpfad im Raum. Die vier Paneele sind die Balkenglieder (Wind über die Breite 2,42 m in die Rollen), die Schienen, Bögen, Rückschienen, Abschlussleiste und Abhänger die Glieder des Schienenrahmens, die Gelenkrohre der Scharniere und die untere Rolle die Drehgelenke, die Lagerfahnen die Lagergelenke der Federwelle, die gewickelte Feder eine kompakte Masse; Auflager nach Typ: feste Pfostenwinkel (Flachplatten außen an beiden Schienen am Boden), Rollenbolzen durch die beiden Deckenabhänger (zwei Auflager bei 1,3 m und 2,5 m, je ein Körper über beide Seiten).")
    return s


def ribbon_poly(w, n):
    outer = [[-w / 2, 0.0]] + arc_pts(0.5, 2.0, 0.5 + w / 2, math.pi, math.pi / 2, n) + [[2.6, 2.5 + w / 2]]
    inner = [[w / 2, 0.0]] + arc_pts(0.5, 2.0, 0.5 - w / 2, math.pi, math.pi / 2, n) + [[2.6, 2.5 - w / 2]]
    return outer + inner[::-1]


def ribbons(s, w, n, z0, z1, slender=False):
    pr = ribbon_poly(w, n)
    for sd, a_, b_ in (("l", z0, z1), ("r", -z1, -z0)):
        d = ext(f"rail-{sd}", "track", ST, 0.4, "xy", pr, a_, b_)
        if slender:
            d["slender"] = True
        s.b(d)


def flag_polygon(n, z0, z1, ids=("bearing-plate", "bearing-plate-r")):
    cx, cy, r = 0.12, 2.75, 0.095
    a = math.asin(0.04 / r)
    prof = [[0.16, 2.30]] + arc_pts(cx, cy, r, -(math.pi / 2 - a), math.pi + (math.pi / 2 - a), n) + [[0.08, 2.30]]
    return [ext(ids[0], "track", ST, 0.4, "xy", prof, z0, z1), ext(ids[1], "track", ST, 0.4, "xy", prof, -z1, -z0)]


def hangers_box(s, zc, tz):
    for sd, z in (("l", zc), ("r", -zc)):
        d = box(f"hanger-{sd}", "track", ST, 0.4, [0.04, 0.35, tz], [1.3, 2.675, z], repeat={"count": 2, "translation": [1.2, 0, 0]})
        s.b(d)


def panels_box(s, th, hide_seal=False):
    for i in range(1, 5):
        s.b(box(f"panel-{i}", f"panel-{i}", SK, 0.55, [0.5, th, 2.42], [0.25, 0, 0]))


def th_slabs(s, t, section=None, thin=None, seal=False):
    for i in range(1, 5):
        lo = 0.07 if (i == 1 and seal) else 0.0
        d = box(f"th-panel-{i}", f"panel-{i}", SK, 0.55, [0.5 - lo, t, 2.42], [(0.5 + lo) / 2, 0.02 - t / 2 if thin is None else thin, 0])
        if section:
            d["sectionFaces"] = section
        s.th(d, "panel-sandwich", "skin-outer")


def th_strips(s, h, t, vc, section=None):
    for i in range(1, 5):
        u = 0.035 if i == 1 else 0.0
        d = box(f"th-hw-strip-{i}", f"panel-{i}", ST, 0.3, [h, t, 2.42], [h / 2 if i == 1 else 0.0, vc, 0])
        if section:
            d["sectionFaces"] = section
        s.th(d, HW, "hw-plate")


def build100():
    s = Scale(100)
    ribbons(s, 0.04, 10, 1.21, 1.25)
    hangers_box(s, 1.23, 0.03)
    for d in flag_polygon(12, 1.25, 1.2652):
        s.b(d)
    panels_box(s, 0.04)
    s.body[-4] = box("panel-1", "panel-1", SK, 0.55, [0.43, 0.04, 2.42], [0.285, 0, 0])
    s.b(box("bottom-seal", "panel-1", ST, 0.15, [0.07, 0.044, 2.42], [0.035, 0, 0], role="detail"))
    for i in range(2, 5):
        s.b(box(f"hinge-{i}", f"panel-{i}", ST, 0.3, [0.08, 0.025, 0.12], [0, -0.0325, -1.15], role="detail", repeat={"count": 4, "translation": [0, 0, 0.766667]}))
    s.b(box("bottom-bracket", "panel-1", ST, 0.3, [0.08, 0.025, 0.12], [0.04, -0.0325, 1.15], role="detail", repeat={"count": 2, "translation": [0, 0, -2.3]}))
    for i in range(1, 5):
        roller_pairs(s, f"panel-{i}", i, 0.5, ROLLER_COARSE)
    roller_pairs(s, "panel-1", 0, 0.0, ROLLER_COARSE)
    ax = [0, 0, 1]
    s.b(revolve("shaft", "spring-shaft", ST, 0.2, ax, [0, 0, -1.255], [[0, 0], [0, 0.0125], [2.51, 0.0125], [2.51, 0]], role="detail"))
    for sd, z in (("l", 1.1), ("r", -1.2)):
        s.b(revolve(f"drum-{sd}", "spring-shaft", ST, 0.5, ax, [0, 0, z], [[0, 0], [0, 0.09], [0.1, 0.09], [0.1, 0]]))
    prof = [[0.0, 0.0]]
    for k in range(62):
        a0 = k * 0.016
        prof += [[a0, 0.0375], [a0 + 0.008, 0.0375], [a0 + 0.008, 0.0285], [a0 + 0.016, 0.0285]]
    prof[-1:] = [[1.0, 0.0375], [1.0, 0.0]]
    s.b(revolve("coil", "spring-shaft", ST, 0.4, [0, 0, 1], [0, 0, -0.5], prof, role="detail"))
    th_slabs(s, 0.0152, seal=True)
    sl = box("th-bottom-seal", "panel-1", ST, 0.15, [0.07, 0.044, 2.42], [0.035, 0, 0])
    sl["sectionFaces"] = ["-v"]
    s.th(sl, "panel-sandwich", "skin-outer")
    for d in s.body:
        i = d["id"]
        if i.startswith(("roller", "hinge", "bottom-bracket")):
            s.thof(f"th-{i}", i, HW, "hw-plate")
        elif i.startswith(("rail", "hanger", "bearing")):
            s.thof(f"th-{i}", i, TR, "track-wall")
        elif i.startswith(("shaft", "drum", "coil")):
            s.thof(f"th-{i}", i, SP, "coil-wire")
    for d in s.body:
        if d["id"].startswith(("rail", "hanger")):
            s.stof(f"m-{d['id']}", d["id"], "track", "member")
    for i in range(1, 5):
        s.stof(f"m-panel-{i}", f"panel-{i}", f"panel-{i}", "member")
    for i in range(2, 5):
        s.stof(f"j-hinge-{i}", f"hinge-{i}", f"panel-{i}", "joint")
    s.stof("j-roller-0-l", "roller-0-l", "panel-1", "joint")
    s.stof("j-roller-0-r", "roller-0-r", "panel-1", "joint")
    s.stof("j-bearing-l", "bearing-plate", "spring-shaft", "joint")
    s.stof("j-bearing-r", "bearing-plate-r", "spring-shaft", "joint")
    s.st(cyl("mass-spring", "spring-shaft", ST, 0.4, "z", [0, 0, 0], 1.0, 0.02), "spring-shaft", "mass")
    supports(s, 0.07, 0.2, 0.0152, 0.022, 0.05)
    s.dec("Body: the door at 15 mm detail. Four 40 mm panels as plain boxes (skins, core, V-chamfer and rib merged), the lowest panel above the 70 mm seal block, each hinge merged to one 15 mm block (four per joint plus the two bottom brackets; the 2D draws them as full-width bands that lie inside the panel silhouette), eleven plain rollers of 22 mm, each guide track merged from its three sections into one continuous L-shaped bar (vertical, R 0.5 m curve and horizontal in one outline, solid 40 x 40 mm), solid ceiling hangers, the shaft, two plain 90 mm drums, the spring as a corrugated 75 mm sleeve and the two bearing flags with a coarser disc, 15 mm thick as drawn.",
          "Körper: das Tor in 15-mm-Auflösung. Vier Paneele zu 40 mm als einfache Quader (Häute, Kern, V-Fase und Sicke zusammengefasst), das unterste Paneel über dem 70-mm-Dichtungsblock, jedes Scharnier zu einem 15-mm-Block zusammengefasst (vier je Fuge und die zwei unteren Winkel; die Zeichnung zeigt sie als Bänder über die volle Breite, die innerhalb der Paneelsilhouette liegen), elf einfache Rollen zu 22 mm, jede Führungsschiene aus ihren drei Abschnitten zu einem durchgehenden L-förmigen Stab zusammengefasst (senkrecht, Kurve R 0,5 m und waagerecht in einem Umriss, massiv 40 x 40 mm), massive Deckenabhänger, die Welle, zwei einfache Trommeln zu 90 mm, die Feder als geriffelte Hülse 75 mm und die beiden Lagerfahnen mit gröberer Scheibe, 15 mm dick wie gezeichnet.")
    s.dec("Thermal: the sandwich is one 15 mm outer slab per panel (skin and the outer part of the core, no layer stack any more) plus the seal's outer face; hinges, rollers and brackets are referenced from the body as the hardware surface, rails, hangers and flags as the track surface, shaft, drums and sleeve as the spring surface.",
          "Thermisch: das Sandwich ist je Paneel eine 15 mm dicke Außenplatte (Haut und äußerer Kernteil, kein Schichtstapel mehr) plus die Außenseite der Dichtung; Scharniere, Rollen und Winkel werden als Beschlagfläche aus dem Körper referenziert, Schienen, Abhänger und Fahnen als Schienenfläche, Welle, Trommeln und Hülse als Federfläche.")
    s.dec("Structure: panels, rail bars and hangers are the members, the hinge blocks and the bottom roller the joints, the flags the bearing joints, the spring a compact mass; supports as at 1:50 (jamb plates, roller pins through the hangers) with the 15 mm plate thickness of this scale.",
          "Tragwerk: Paneele, Schienenstäbe und Abhänger sind die Glieder, die Scharnierblöcke und die untere Rolle die Gelenke, die Fahnen die Lagergelenke, die Feder eine kompakte Masse; Auflager wie bei 1:50 (Pfostenplatten, Rollenbolzen durch die Abhänger) mit der Plattendicke 15 mm dieses Maßstabs.")
    return s


def supports(s, jw, jh, jt, pin_r, pin_l, zjamb=1.255, hy=2.835):
    s.st(box("support-jamb-fix", "track", ST, 0.4, [jw, jh, jt], [0, jh / 2, zjamb + (jt - 0.01) / 2], repeat={"count": 2, "translation": [0, 0, -2 * zjamb - (jt - 0.01)]}), "track", "support")
    for n, x in ((1, 1.3), (2, 2.5)):
        s.st(cyl(f"support-hanger-{n}", "track", ST, 0.4, "z", [x, hy, 1.23], pin_l, pin_r, repeat={"count": 2, "translation": [0, 0, -2.46]}), "track", "support")


def build200():
    s = Scale(200)
    ribbons(s, 0.04, 8, 1.21, 1.25)
    hangers_box(s, 1.23, 0.03)
    for d in flag_polygon(8, 1.235, 1.2652):
        s.b(d)
    panels_box(s, 0.04)
    ax = [0, 0, 1]
    sh = revolve("shaft", "spring-shaft", ST, 0.2, ax, [0, 0, -1.27], [[0, 0], [0, 0.01], [2.54, 0.01], [2.54, 0]], role="detail")
    sh["slender"] = True
    s.b(sh)
    for sd, z in (("l", 1.1), ("r", -1.2)):
        s.b(revolve(f"drum-{sd}", "spring-shaft", ST, 0.5, ax, [0, 0, z], [[0, 0], [0, 0.09], [0.1, 0.09], [0.1, 0]]))
    th_slabs(s, 0.03)
    for i in range(1, 5):
        d = box(f"th-hw-strip-{i}", f"panel-{i}", ST, 0.3, [0.07, 0.03, 2.42], [0.035 if i == 1 else 0.0, -0.005, 0])
        s.th(d, HW, "hw-plate")
    for d in s.body:
        i = d["id"]
        if i.startswith(("rail", "hanger", "bearing")):
            s.thof(f"th-{i}", i, TR, "track-wall")
        elif i.startswith(("shaft", "drum")):
            s.thof(f"th-{i}", i, SP, "coil-wire")
    for d in s.body:
        if d["id"].startswith(("rail", "hanger")):
            s.stof(f"m-{d['id']}", d["id"], "track", "member")
    for i in range(1, 5):
        s.stof(f"m-panel-{i}", f"panel-{i}", f"panel-{i}", "member")
    for i in range(2, 5):
        s.st(box(f"j-hinge-{i}", f"panel-{i}", ST, 0.3, [0.06, 0.03, 0.06], [0, 0, 0]), f"panel-{i}", "joint")
    s.stof("j-bearing-l", "bearing-plate", "spring-shaft", "joint")
    s.stof("j-bearing-r", "bearing-plate-r", "spring-shaft", "joint")
    s.st(cyl("mass-spring", "spring-shaft", ST, 0.4, "z", [0, 0, 0], 1.0, 0.03), "spring-shaft", "mass")
    supports(s, 0.07, 0.2, 0.03, 0.015, 0.06)
    s.dec("Body: the door at 30 mm detail. Four 40 mm panels as boxes, each guide track merged into one continuous L-shaped 40 mm bar (the 2D draws a 50 mm stroke), two hangers, the two bearing flags at the smallest feature 30 mm (set into the track side so the depth stays within the drawn 2.5 m), the shaft as a slender 20 mm bar and two 90 mm drums. Rollers, hinges, seal, spring coil and the end tie bar fall below 30 mm or are not drawn at this scale and are merged into the panels and tracks.",
          "Körper: das Tor in 30-mm-Auflösung. Vier Paneele zu 40 mm als Quader, jede Führungsschiene zu einem durchgehenden L-förmigen Stab von 40 mm zusammengefasst (die 2D zeichnet einen Strich von 50 mm), zwei Abhänger, die beiden Lagerfahnen mit der kleinsten Einzelheit 30 mm (in die Schienenseite eingelassen, damit die Tiefe innerhalb der gezeichneten 2,5 m bleibt), die Welle als schlanker 20-mm-Stab und zwei Trommeln zu 90 mm. Rollen, Scharniere, Dichtung, Federwicklung und der Abschlussriegel liegen unter 30 mm oder sind in diesem Maßstab nicht gezeichnet und sind in Paneele und Schienen aufgegangen.")
    s.dec("Thermal: each panel's outer skin is a 30 mm slab under the outer face; the hinges, rollers and seal are 30 mm strips flush with the inner face, inside the panel bodies (hidden detail carried only in the thermal model); track and spring regions are referenced from the body.",
          "Thermisch: die Außenhaut jedes Paneels ist eine 30 mm dicke Platte unter der Außenfläche; Scharniere, Rollen und Dichtung sind 30-mm-Streifen bündig mit der Innenfläche, im Körper der Paneele (verdecktes Detail, nur im Thermalmodell); Schienen- und Federflächen werden aus dem Körper referenziert.")
    s.dec("Structure: the panels and the track bars carry the loads; the hinge joints are 60 mm blocks inside the panels, the flags the shaft bearings, the spring a 60 mm mass; supports at 30 mm: plates outside the rails, pins through the hangers.",
          "Tragwerk: die Paneele und die Schienenstäbe tragen die Lasten; die Scharniergelenke sind 60-mm-Blöcke in den Paneelen, die Fahnen die Wellenlager, die Feder eine Masse von 60 mm; Auflager in 30 mm: Platten außen an den Schienen, Bolzen durch die Abhänger.")
    return s


def build500():
    s = Scale(500)
    for sd, z in (("l", 1.23), ("r", -1.23)):
        up = box(f"rail-up-{sd}", "track", ST, 0.4, [0.065, 2.0, 0.075], [0, 1.0, z])
        up["slender"] = True
        ring = arc_pts(0.5, 2.0, 0.5325, math.pi, math.pi / 2, 4) + arc_pts(0.5, 2.0, 0.4675, math.pi / 2, math.pi, 4)
        arc = ext(f"rail-arc-{sd}", "track", ST, 0.4, "xy", ring, z - 0.0375, z + 0.0375)
        back = box(f"rail-back-{sd}", "track", ST, 0.4, [2.1, 0.065, 0.075], [1.55, 2.5, z])
        back["slender"] = True
        for d in (up, arc, back):
            s.b(d)
    panels_box(s, 0.075)
    ax = [0, 0, 1]
    sh = revolve("shaft", "spring-shaft", ST, 0.2, ax, [0, 0, -1.2], [[0, 0], [0, 0.0325], [2.4, 0.0325], [2.4, 0]], role="detail")
    sh["slender"] = True
    s.b(sh)
    for sd, z in (("l", 1.1), ("r", -1.2)):
        s.b(revolve(f"drum-{sd}", "spring-shaft", ST, 0.5, ax, [0, 0, z], [[0, 0], [0, 0.09], [0.1, 0.09], [0.1, 0]]))
    th_slabs(s, 0.075, section=["-v"], thin=0.0)
    for i in range(1, 5):
        d = box(f"th-hw-strip-{i}", f"panel-{i}", ST, 0.3, [0.075, 0.075, 2.42], [0.0375 if i == 1 else 0.0, 0, 0])
        d["sectionFaces"] = ["-v"]
        s.th(d, HW, "hw-plate")
    for d in s.body:
        i = d["id"]
        if i.startswith("rail"):
            s.thof(f"th-{i}", i, TR, "track-wall")
        elif i.startswith(("shaft", "drum")):
            s.thof(f"th-{i}", i, SP, "coil-wire")
    for d in s.body:
        if d["id"].startswith("rail"):
            s.stof(f"m-{d['id']}", d["id"], "track", "member")
    for i in range(1, 5):
        s.stof(f"m-panel-{i}", f"panel-{i}", f"panel-{i}", "member")
    for i in range(2, 5):
        s.st(box(f"j-hinge-{i}", f"panel-{i}", ST, 0.3, [0.075, 0.075, 0.075], [0, 0, 0]), f"panel-{i}", "joint")
    s.stof("j-bearing-l", "drum-l", "spring-shaft", "joint")
    s.stof("j-bearing-r", "drum-r", "spring-shaft", "joint")
    supports(s, 0.075, 0.2, 0.075, 0.0375, 0.075, zjamb=1.2875, hy=2.8125)
    s.dec("Body: the door at 75 mm detail. Four panels as 75 mm boxes over the 2.42 m width (the plan draws their edge 65 mm thick, thickened to the smallest feature), each guide track one 75 mm bar (65 mm drawn pen width) centred on the drawn stroke so that it takes in the roller engagement, the shaft as a slender 65 mm bar and the two 90 mm drums. Hangers, bearing flags, rollers, hinges, seal, spring and tie bar are below 75 mm or not drawn and are merged into panels and tracks.",
          "Körper: das Tor in 75-mm-Auflösung. Vier Paneele als 75-mm-Quader über die Breite 2,42 m (der Grundriss zeichnet ihre Kante 65 mm dick, auf die kleinste Einzelheit verstärkt), jede Führungsschiene ein 75-mm-Stab (65 mm gezeichnete Strichbreite), auf dem gezeichneten Strich zentriert, sodass er den Eingriff der Rollen mit aufnimmt, die Welle als schlanker 65-mm-Stab und die zwei Trommeln zu 90 mm. Abhänger, Lagerfahnen, Rollen, Scharniere, Dichtung, Feder und Abschlussriegel liegen unter 75 mm oder sind nicht gezeichnet und sind in Paneele und Schienen aufgegangen.")
    s.dec("Thermal: the outer face of each panel is a full-thickness 75 mm slab (the inner face carries no region, `sectionFaces` -v); the hardware is a 75 mm strip per joint inside the panel; tracks, shaft and drums are referenced from the body.",
          "Thermisch: die Außenfläche jedes Paneels ist eine Platte in voller Dicke von 75 mm (die Innenfläche trägt keine Region, `sectionFaces` -v); der Beschlag ist je Fuge ein 75-mm-Streifen im Paneel; Schienen, Welle und Trommeln werden aus dem Körper referenziert.")
    s.dec("Structure: panels and track bars are the members, hinge joints are 75 mm blocks in the panels, the drums carry the shaft bearings; supports: jamb plates and the two hanger pins as 75 mm blocks. Minimal: the least that carries the loads.",
          "Tragwerk: Paneele und Schienenstäbe sind die Glieder, die Scharniergelenke 75-mm-Blöcke in den Paneelen, die Trommeln tragen die Wellenlager; Auflager: Pfostenplatten und die zwei Abhängerbolzen als 75-mm-Blöcke. Minimal: das Wenigste, das die Lasten trägt.")
    return s


SC = {}
for n_, f_ in ((50, lambda: finish50(build50())), (100, build100), (200, build200), (500, build500)):
    if n_ in only:
        SC[n_] = f_()
if __name__ == "__main__":
    save_asset("sectional-garage-door", [SC[k] for k in sorted(SC)])
