"""🏗️ Round 5 cantilever balcony (from round 3, coarse thermal wall band and 1:500 rail member added): the handcrafted solids of every scale (body, thermal, structure), stated part by part, written to the volumes source."""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-cantilever-balcony-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
box, prism, rod, tube, bar, repeat, cut, th, th_of, st, st_of, text, rail, r5, top = K.box, K.prism, K.rod, K.tube, K.bar, K.repeat, K.cut, K.th, K.th_of, K.st, K.st_of, K.text, K.rail, K.r5, K.top

SL, RA = "slab", "railing"
CO, STL, WO, RE, BM = "concrete", "steel", "mineral-wool", "render", "break-module"
T_CO, T_WALL, T_WO, T_RE, T_BM, T_ST, T_PAD = 0.72, 0.62, 0.92, 0.85, 0.42, 0.3, 0.6
SIDES = (("l", 1), ("r", -1))


def wall_layers():
    return [
        cut(box("wall-core", SL, CO, T_WALL, [0.18, 1.3, 1.0], [-0.21, 0.55, 0]), "-v", "+v", "-w", "+w"),
        cut(box("wall-insulation", SL, WO, T_WO, [0.1, 1.1, 1.0], [-0.07, 0.65, 0]), "+v", "-w", "+w"),
        cut(box("wall-render", SL, RE, T_RE, [0.02, 1.1, 1.0], [-0.01, 0.65, 0]), "+v", "-w", "+w"),
    ]


def break_module():
    return cut(box("break-module", SL, BM, T_BM, [0.12, 0.2, 1.0], [-0.06, 0, 0]), "-w", "+w")


def scale50():
    body = [cut(prism("slab", SL, CO, T_CO, K.SLAB_PROFILE, "xy", -0.5, 0.5), "-w", "+w"), break_module()] + wall_layers()
    body.append(prism("railing-grout-pad", SL, CO, T_PAD, K.PAD_PROFILE, "xy", -0.085, 0.085, role="detail"))
    body.append(box("railing-base-plate", RA, STL, T_ST, [K.PLATE_T, 0.15, 0.15], [(K.PLATE_U0 + K.PLATE_U1) / 2, 0, 0]))
    for name, sign in (("outer", -1), ("inner", 1)):
        nut = prism(f"railing-anchor-nut-{name}", RA, STL, 0.2, K.hexagon(0.017), "yz", K.PLATE_U1, K.PLATE_U1 + 0.01, role="detail")
        nut["transform"] = {"translation": [0, r5(sign * K.ANCHOR), K.ANCHOR]}
        body.append(repeat(nut, 2, [0, 0, -2 * K.ANCHOR]))
        body.append(repeat(rod(f"railing-anchor-stud-{name}", RA, STL, 0.2, [K.PLATE_U1 + 0.01, sign * K.ANCHOR, K.ANCHOR], [1, 0, 0], 0.008, 0.005, role="detail"), 2, [0, 0, -2 * K.ANCHOR]))
    post = prism("railing-post", RA, STL, T_ST, K.rounded(0.025, 0.008), "yz", K.PLATE_U1, K.POST_TOP)
    post["holes"] = [K.rounded(0.021, 0.004)]
    body.append(post)
    body.append(box("railing-post-cap", RA, STL, T_ST, [K.CAP_T, 0.05, 0.05], [K.POST_TOP + K.CAP_T / 2, 0, 0]))
    body.append(rod("railing-handrail-stub", RA, STL, T_ST, [K.POST_TOP + K.CAP_T, 0, 0], [1, 0, 0], K.HAND_U - K.HAND_R + 0.002 - K.POST_TOP - K.CAP_T, 0.01))
    body.append(cut(tube("railing-handrail", RA, STL, T_ST, [K.HAND_U, 0, -0.5], [0, 0, 1], 1.0, K.HAND_R, K.HAND_T), "-w", "+w"))
    for chord, u in (("top", K.CHORD_TOP), ("bottom", K.CHORD_LOW)):
        for side, sign in SIDES:
            body.append(cut(box(f"railing-{chord}-chord-{side}", RA, STL, T_ST, [0.01, 0.04, 0.475], [u, 0, sign * 0.2625]), "+w" if sign > 0 else "-w"))
    for side, sign in SIDES:
        body.append(repeat(rod(f"railing-balusters-{side}", RA, STL, T_ST, [K.CHORD_LOW + 0.004, 0, sign * K.BALUSTER_W[0]], [1, 0, 0], K.CHORD_TOP - K.CHORD_LOW - 0.008, K.BALUSTER_R, role="detail"), 4, [0, 0, sign * 0.10689]))

    thermal = K.skins(SL, True)
    thermal += [
        th(cut(box("th-wall-concrete", SL, CO, T_WALL, [0.18, 1.1, 1.0], [-0.21, 0.65, 0]), "-u", "+v", "-v", "-w", "+w"), "wall-face", "wall-concrete"),
        th(cut(box("th-wall-wool", SL, WO, T_WO, [0.1, 1.1, 1.0], [-0.07, 0.65, 0]), "-u", "+v", "-v", "-w", "+w"), "wall-face", "wall-wool"),
        th(cut(box("th-wall-render", SL, RE, T_RE, [0.02, 1.1, 1.0], [-0.01, 0.65, 0]), "-u", "+v", "-v", "-w", "+w"), "wall-face", "wall-render"),
        th(cut(box("th-break-core", SL, CO, T_WALL, [0.18, 0.2, 1.0], [-0.21, 0, 0]), "-u", "+v", "-v", "-w", "+w"), "thermal-break", "break-concrete"),
        th_of("th-break-module", "break-module", "thermal-break", "break-body", ["-u", "-w", "+w"]),
        th(prism("th-railing-post", RA, STL, T_ST, K.rounded(0.025, 0.008), "yz", K.PLATE_U1, K.POST_TOP), "railing-steel"),
        th(cut(rod("th-railing-handrail", RA, STL, T_ST, [K.HAND_U, 0, -0.5], [0, 0, 1], 1.0, K.HAND_R), "-w", "+w"), "railing-steel"),
    ]
    for of in ("railing-base-plate", "railing-anchor-nut-outer", "railing-anchor-nut-inner", "railing-anchor-stud-outer", "railing-anchor-stud-inner", "railing-post-cap", "railing-handrail-stub", "railing-balusters-l", "railing-balusters-r"):
        thermal.append(th_of("th-" + of, of, "railing-steel"))
    for chord in ("top", "bottom"):
        for side, sign in SIDES:
            thermal.append(th_of(f"th-railing-{chord}-chord-{side}", f"railing-{chord}-chord-{side}", "railing-steel", faces=["+w" if sign > 0 else "-w"]))

    structure = [
        st_of("slab-member", "slab", SL),
        st_of("support-wall-core", "wall-core", SL, "support", "wall-fix"),
        st(repeat(bar("joint-tension-bars", SL, STL, 0.2, K.TENSION_PATH, K.BAR_W[0], 0.006), 5, [0, 0, 0.2]), SL, "joint"),
        st(repeat(box("joint-pressure-bearings", SL, CO, 0.5, [0.14, 0.03, 0.04], [-0.06, -0.065, K.BEARING_W[0]]), 4, [0, 0, 0.2]), SL, "joint"),
        st(repeat(bar("joint-shear-bars", SL, STL, 0.2, K.SHEAR_PATH, K.SHEAR_W[0], 0.004), 4, [0, 0, 0.2]), SL, "joint"),
        st(repeat(bar("slab-top-bars", SL, STL, 0.25, K.TOP_BAR, K.BAR_W[0] - 0.03, 0.006), 5, [0, 0, 0.2]), SL),
        st(repeat(bar("slab-bottom-bars", SL, STL, 0.25, K.BOTTOM_BAR, K.BAR_W[0] - 0.03, 0.004), 5, [0, 0, 0.2]), SL),
        st(repeat(bar("slab-edge-hairpins", SL, STL, 0.25, K.EDGE_PATH, K.BAR_W[0] + 0.03, 0.004), 5, [0, 0, 0.2]), SL),
        st(repeat(rod("slab-top-cross-bars", SL, STL, 0.25, [K.CROSS_TOP_X[0], top(K.CROSS_TOP_X[0]) - K.CROSS_TOP_DROP, -0.47], [0, 0, 1], 0.94, 0.004), 6, [0.25, -0.25 * K.FALL, 0]), SL),
        st(repeat(rod("slab-bottom-cross-bars", SL, STL, 0.25, [K.CROSS_BOTTOM_X[0], K.CROSS_BOTTOM_V, -0.47], [0, 0, 1], 0.94, 0.004), 6, [0.25, 0, 0]), SL),
        st({"id": "mass-slab", "kind": "ellipsoid", "bone": SL, "material": CO, "role": "detail", "tone": 0.4, "radii": [0.05, 0.05, 0.05], "transform": {"translation": [r5(SLAB_COM[0]), r5(SLAB_COM[1]), 0]}}, SL, "mass"),
        st_of("railing-post-member", "railing-post", RA),
        st_of("railing-post-cap-member", "railing-post-cap", RA),
        st_of("railing-handrail-stub-member", "railing-handrail-stub", RA),
        st_of("railing-handrail-member", "railing-handrail", RA),
        st_of("joint-railing-base-plate", "railing-base-plate", RA, "joint"),
    ]
    for name, sign in (("outer", -1), ("inner", 1)):
        structure.append(st(repeat(rod(f"joint-railing-anchors-{name}", RA, STL, 0.2, [K.PLATE_U1 + 0.018, sign * K.ANCHOR, K.ANCHOR], [-1, 0, 0], 0.11, 0.005), 2, [0, 0, -2 * K.ANCHOR]), RA, "joint"))
    decisions = [
        text(
            "Body: a one-metre strip of a real cantilever balcony. The 200 mm slab is one extrusion with its 1.5 % drainage fall (200 mm at the wall, 177.5 mm at the edge), 10 mm arris chamfers and a 25/15 x 15 mm drip groove 70 mm behind the edge; behind it the 120 mm load-bearing thermal-break module in the insulation plane and the wall as built: 180 mm concrete in one piece, 100 mm mineral wool, 20 mm render. The railing follows the rig, one post per metre at the strip centre, 120 mm inside the edge so that the anchors keep 65 mm edge distance: a grout bed levels the fall, a 150 x 150 x 12 mm base plate with four M10 stud anchors and hexagon nuts, a cold-formed hollow post 50 x 50 x 4 with rounded corners, an 8 mm cap, a 20 mm round bracket and the handrail tube 60.3 x 2.9 with its real wall, cut at the strip ends. The infill between the posts is welded: two flat bars 40 x 10 as chords and eight 12 mm round balusters per metre with 95 mm clear spacing, the lower chord 85 mm above the slab.",
            "Körper: ein Meterstreifen eines echten Kragbalkons. Die Platte von 200 mm ist eine Extrusion mit 1,5 % Gefälle (200 mm an der Wand, 177,5 mm an der Kante), Fasen von 10 mm und einer Tropfnut 25/15 x 15 mm 70 mm hinter der Kante; dahinter das tragende Dämmelement von 120 mm in der Dämmebene und die Wand wie gebaut: 180 mm Beton in einem Stück, 100 mm Mineralwolle, 20 mm Putz. Das Geländer folgt dem Skelett, ein Pfosten je Meter in Streifenmitte, 120 mm hinter der Kante, damit die Anker 65 mm Randabstand halten: ein Mörtelbett gleicht das Gefälle aus, darauf die Fußplatte 150 x 150 x 12 mm mit vier Bolzenankern M10 und Sechskantmuttern, der kaltgeformte Hohlprofilpfosten 50 x 50 x 4 mit runden Ecken, ein Deckel von 8 mm, ein runder Träger von 20 mm und das Handlaufrohr 60,3 x 2,9 mit echter Wandung, an den Streifenenden geschnitten. Die Füllung zwischen den Pfosten ist geschweißt: zwei Flachstähle 40 x 10 als Gurte und acht Rundstäbe von 12 mm je Meter mit 95 mm lichtem Abstand, der Untergurt 85 mm über der Platte.",
        ),
        text(
            "Thermal: slab top, soffit and edge are the 80 mm storage skins of the thermal model, wedges that meet on mitres inside the concrete, so each exposes exactly its own outer face (the chamfers go with top and soffit, the drip groove with the soffit); the faces inside the concrete and the strip ends are section faces. The wall is three nested plates in their real thicknesses (concrete, wool, render) and counts its rendered outer face; the thermal bridge is the concrete band behind the slab plus the break module, which counts its top, underside and outer face. The railing steel is every steel part; post and handrail are closed bodies here because their bores are sealed air.",
            "Thermisch: Plattenoberseite, -unterseite und -stirn sind die Speicherhäute von 80 mm des thermischen Modells, Keile, die sich auf Gehrungen im Beton treffen, sodass jede genau ihre eigene Außenfläche zeigt (die Fasen gehören zu Ober- und Unterseite, die Tropfnut zur Unterseite); die Flächen im Beton und die Streifenenden sind Schnittflächen. Die Wand besteht aus drei geschachtelten Platten in echter Dicke (Beton, Wolle, Putz) und zählt ihre verputzte Außenfläche; die Wärmebrücke ist das Betonband hinter der Platte samt Dämmelement, das Oberseite, Unterseite und Außenfläche zählt. Der Geländerstahl ist jedes Stahlteil; Pfosten und Handlauf sind hier geschlossene Körper, weil ihre Hohlräume eingeschlossene Luft sind.",
        ),
        text(
            "Structure: the load path of the reinforced slab. The slab itself is the member; the wall core is the fixed support. Between them only the connection crosses the insulation: five 12 mm tension bars in the top layer, bent down into the wall with a 30 mm bend radius and lapped 700 mm into the slab, four concrete pressure bearings at the bottom and four 8 mm shear bars running diagonally down towards the balcony. Inside the slab lie the five 12 mm top bars along the fall, 8 mm cross bars top and bottom, 8 mm bottom bars and hairpins around the free edge. The railing member is post, cap, bracket and handrail; its joint is the base plate with four anchors reaching 90 mm into the slab. A ball marks the centre of mass of the slab.",
            "Tragwerk: der Lastpfad der bewehrten Platte. Die Platte selbst ist der Stab, der Wandkern das feste Auflager. Dazwischen quert nur der Anschluss die Dämmung: fünf Zugstäbe von 12 mm in der oberen Lage, mit 30 mm Biegeradius in die Wand abgebogen und 700 mm in die Platte übergreifend, vier Betondrucklager unten und vier Querkraftstäbe von 8 mm, die schräg zum Balkon hin abfallen. In der Platte liegen die fünf oberen Stäbe von 12 mm im Gefälle, Querstäbe von 8 mm oben und unten, untere Stäbe von 8 mm und Steckbügel um die freie Kante. Der Geländerstab ist Pfosten, Deckel, Träger und Handlauf; sein Gelenk ist die Fußplatte mit vier Ankern, die 90 mm in die Platte reichen. Eine Kugel markiert den Massenschwerpunkt der Platte.",
        ),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale100():
    head_u0 = 1.02
    body = [
        cut(prism("slab", SL, CO, T_CO, K.SLAB_WEDGE, "xy", -0.5, 0.5), "-w", "+w"),
        break_module(),
        cut(box("wall-core", SL, CO, T_WALL, [0.18, 1.3, 1.0], [-0.21, 0.55, 0]), "-v", "+v", "-w", "+w"),
        cut(box("wall-insulation-render", SL, WO, T_WO, [0.12, 1.1, 1.0], [-0.06, 0.65, 0]), "+v", "-w", "+w"),
        box("railing-foot", RA, STL, T_ST, [K.PLATE_U1 + 0.002, 0.15, 0.15], [(K.PLATE_U1 - 0.002) / 2, 0, 0]),
    ]
    post = prism("railing-post", RA, STL, T_ST, [[-0.025, -0.025], [0.025, -0.025], [0.025, 0.025], [-0.025, 0.025]], "yz", K.PLATE_U1, head_u0)
    post["holes"] = [[[-0.021, -0.021], [0.021, -0.021], [0.021, 0.021], [-0.021, 0.021]]]
    body.append(post)
    body.append(box("railing-post-head", RA, STL, T_ST, [K.HAND_U - K.HAND_R + 0.002 - head_u0, 0.05, 0.05], [(K.HAND_U - K.HAND_R + 0.002 + head_u0) / 2, 0, 0]))
    body.append(cut(tube("railing-handrail", RA, STL, T_ST, [K.HAND_U, 0, -0.5], [0, 0, 1], 1.0, K.HAND_R, K.HAND_T), "-w", "+w"))
    for chord, u in (("top", K.CHORD_TOP), ("bottom", K.CHORD_LOW)):
        c = cut(box(f"railing-{chord}-chord", RA, STL, T_ST, [0.01, 0.04, 1.0], [u, 0, 0], role="detail"), "-w", "+w")
        c["slender"] = True
        body.append(c)
    for side, sign in SIDES:
        b = repeat(rod(f"railing-balusters-{side}", RA, STL, T_ST, [K.CHORD_LOW, 0, sign * K.BALUSTER_W[0]], [1, 0, 0], K.CHORD_TOP - K.CHORD_LOW, K.BALUSTER_R, role="detail"), 4, [0, 0, sign * 0.10689])
        b["slender"] = True
        body.append(b)

    thermal = K.skins(SL, False)
    thermal += [
        th(cut(box("th-wall-concrete", SL, CO, T_WALL, [0.18, 1.1, 1.0], [-0.21, 0.65, 0]), "-u", "+v", "-v", "-w", "+w"), "wall-face", "wall-concrete"),
        th(cut(box("th-wall-wool-render", SL, WO, T_WO, [0.12, 1.1, 1.0], [-0.06, 0.65, 0]), "-u", "+v", "-v", "-w", "+w"), "wall-face", "wall-render"),
        th(cut(box("th-break-core", SL, CO, T_WALL, [0.18, 0.2, 1.0], [-0.21, 0, 0]), "-u", "+v", "-v", "-w", "+w"), "thermal-break", "break-concrete"),
        th_of("th-break-module", "break-module", "thermal-break", "break-body", ["-u", "-w", "+w"]),
        th(box("th-railing-post", RA, STL, T_ST, [K.HAND_U - K.HAND_R + 0.002 - K.PLATE_U1, 0.05, 0.05], [(K.HAND_U - K.HAND_R + 0.002 + K.PLATE_U1) / 2, 0, 0]), "railing-steel"),
        th(cut(rod("th-railing-handrail", RA, STL, T_ST, [K.HAND_U, 0, -0.5], [0, 0, 1], 1.0, K.HAND_R), "-w", "+w"), "railing-steel"),
        th_of("th-railing-foot", "railing-foot", "railing-steel"),
        th_of("th-railing-top-chord", "railing-top-chord", "railing-steel", faces=["-w", "+w"]),
        th_of("th-railing-bottom-chord", "railing-bottom-chord", "railing-steel", faces=["-w", "+w"]),
        th_of("th-railing-balusters-l", "railing-balusters-l", "railing-steel"),
        th_of("th-railing-balusters-r", "railing-balusters-r", "railing-steel"),
    ]

    tension = repeat(rod("joint-tension-bars", SL, STL, 0.2, [-0.25, K.TENSION_V, K.BAR_W[0]], [1, 0, 0], 0.95, 0.006), 5, [0, 0, 0.2])
    tension["slender"] = True
    tops = repeat(rod("slab-top-bars", SL, STL, 0.25, [0.03, top(0.03) - K.TOP_BAR_DROP, K.BAR_W[0] - 0.03], [1, -K.FALL, 0], 1.41, 0.006), 5, [0, 0, 0.2])
    tops["slender"] = True
    structure = [
        st_of("slab-member", "slab", SL),
        st_of("support-wall-core", "wall-core", SL, "support", "wall-fix"),
        st(tension, SL, "joint"),
        st(box("joint-pressure-strip", SL, CO, 0.5, [0.14, 0.03, 0.8], [-0.06, -0.065, 0]), SL, "joint"),
        st(tops, SL),
        st_of("railing-post-member", "railing-post", RA),
        st_of("railing-post-head-member", "railing-post-head", RA),
        st_of("railing-handrail-member", "railing-handrail", RA),
        st_of("joint-railing-foot", "railing-foot", RA, "joint"),
    ]
    decisions = [
        text(
            "Body: redrawn for 15 mm. The slab is the plain wedge with its fall (22 mm over the depth); chamfers and drip groove are gone. Wool and render are one 120 mm plate as in the drawing. The railing foot is one 27 mm block for grout bed and base plate, the anchors are gone; the post is a sharp-cornered hollow section with one solid head for cap and bracket, the handrail keeps its tube wall at the cut. Chords and balusters stay at their true 10 and 12 mm as slender parts, the chords running through the post, because the drawing shows them as pen lines.",
            "Körper: neu gezeichnet für 15 mm. Die Platte ist der einfache Keil mit Gefälle (22 mm über die Tiefe); Fasen und Tropfnut entfallen. Wolle und Putz sind wie in der Zeichnung eine Platte von 120 mm. Der Geländerfuß ist ein Block von 27 mm für Mörtelbett und Fußplatte, die Anker entfallen; der Pfosten ist ein scharfkantiges Hohlprofil mit einem vollen Kopf für Deckel und Träger, der Handlauf behält am Schnitt seine Rohrwand. Gurte und Füllstäbe bleiben als schlanke Teile bei ihren wahren 10 und 12 mm, die Gurte laufen durch den Pfosten, weil die Zeichnung sie als Federlinien zeigt.",
        ),
        text(
            "Thermal: the three 80 mm skins without chamfers and groove; the wall as concrete plus one merged wool and render shell that carries the outer face; the thermal bridge as concrete band plus break module; the railing with closed post and handrail bodies and the referenced foot, chords and balusters.",
            "Thermisch: die drei Häute von 80 mm ohne Fasen und Nut; die Wand als Beton und eine zusammengefasste Schale aus Wolle und Putz, die die Außenfläche trägt; die Wärmebrücke als Betonband und Dämmelement; das Geländer mit geschlossenen Körpern für Pfosten und Handlauf sowie Fuß, Gurten und Füllstäben als Verweise.",
        ),
        text(
            "Structure: slab member and wall core support as before; the connection is reduced to its force couple, five straight 12 mm tension bars through the insulation (hooks and shear bars merged away) and one pressure strip for the four bearings; of the slab reinforcement only the five top bars along the fall remain. Railing: post, head and handrail on the foot block as the joint.",
            "Tragwerk: Plattenstab und Wandkern als Auflager wie zuvor; der Anschluss ist auf sein Kräftepaar reduziert, fünf gerade Zugstäbe von 12 mm durch die Dämmung (Haken und Querkraftstäbe aufgegangen) und ein Druckstreifen für die vier Lager; von der Plattenbewehrung bleiben nur die fünf oberen Stäbe im Gefälle. Geländer: Pfosten, Kopf und Handlauf auf dem Fußblock als Gelenk.",
        ),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale200():
    wall = [[-0.3, -0.1], [-0.12, -0.1], [-0.12, 0.1], [0.0, 0.1], [0.0, 1.2], [-0.3, 1.2]]
    infill = box("railing-infill", RA, STL, 0.6, [K.CHORD_TOP - K.CHORD_LOW + 0.01, 0.012, 1.0], [(K.CHORD_TOP + K.CHORD_LOW) / 2, 0, 0], role="detail")
    infill["texture"] = True
    body = [
        cut(prism("wall", SL, CO, T_WALL, wall, "xy", -0.5, 0.5), "-v", "+v", "-w", "+w"),
        break_module(),
        cut(prism("slab", SL, CO, T_CO, K.SLAB_WEDGE, "xy", -0.5, 0.5), "-w", "+w"),
        box("railing-post", RA, STL, T_ST, [K.HAND_U + 0.002, 0.05, 0.05], [(K.HAND_U - 0.002) / 2, 0, 0]),
        cut(rod("railing-handrail", RA, STL, T_ST, [K.HAND_U, 0, -0.5], [0, 0, 1], 1.0, K.HAND_R), "-w", "+w"),
        infill,
    ]
    tip = top(1.5)
    thermal = [
        th(cut(prism("th-slab-top", SL, CO, 0.75, [[0, 0.1], [0.16, 0.02], [1.34, 0.0], [1.5, r5(tip)]], "xy", -0.5, 0.5), "-v", "-w", "+w"), "slab-top"),
        th(cut(prism("th-slab-soffit", SL, CO, 0.6, [[0, -0.1], [1.5, -0.1], [1.34, -0.02], [0.16, -0.02]], "xy", -0.5, 0.5), "+v", "-w", "+w"), "slab-soffit"),
        th(cut(prism("th-slab-edge", SL, CO, 0.68, [[1.5, r5(tip)], [1.34, 0.0], [1.34, -0.02], [1.5, -0.1]], "xy", -0.5, 0.5), "-u", "+v", "-v", "-w", "+w"), "slab-edge"),
        th(cut(box("th-wall", SL, CO, T_WALL, [0.3, 1.1, 1.0], [-0.15, 0.65, 0]), "-u", "+v", "-v", "-w", "+w"), "wall-face"),
        th(cut(box("th-break-core", SL, CO, T_WALL, [0.18, 0.2, 1.0], [-0.21, 0, 0]), "-u", "+v", "-v", "-w", "+w"), "thermal-break"),
        th_of("th-break-module", "break-module", "thermal-break", faces=["-u", "-w", "+w"]),
        th_of("th-railing-post", "railing-post", "railing-steel"),
        th_of("th-railing-handrail", "railing-handrail", "railing-steel", faces=["-w", "+w"]),
    ]
    structure = [
        st_of("slab-member", "slab", SL),
        st(box("support-wall-core", SL, CO, T_WALL, [0.18, 1.3, 1.0], [-0.21, 0.55, 0]), SL, "support", "wall-fix"),
        st(box("joint-tension-layer", SL, STL, 0.2, [0.55, 0.03, 0.9], [0.025, K.TENSION_V, 0]), SL, "joint"),
        st(box("joint-pressure-layer", SL, CO, 0.5, [0.16, 0.03, 0.9], [-0.06, -0.065, 0]), SL, "joint"),
        st_of("railing-post-member", "railing-post", RA),
        st_of("railing-handrail-member", "railing-handrail", RA),
    ]
    decisions = [
        text(
            "Body: redrawn for 30 mm. The wall is one L-shaped mass (core, wool and render cannot be told apart), the break module stays as the dark joint, the slab keeps its wedge. The railing is one solid 50 mm post up to the solid 60 mm handrail; the baluster panel between the chords is one thin sheet marked as texture, as the drawing shows it as a tone.",
            "Körper: neu gezeichnet für 30 mm. Die Wand ist eine L-förmige Masse (Kern, Wolle und Putz sind nicht zu unterscheiden), das Dämmelement bleibt als dunkle Fuge, die Platte behält ihren Keil. Das Geländer ist ein voller Pfosten von 50 mm bis zum vollen Handlauf von 60 mm; das Stabfeld zwischen den Gurten ist eine dünne, als Textur markierte Scheibe, wie die Zeichnung es als Tonfläche zeigt.",
        ),
        text(
            "Thermal: three plain skins, one wall block above the slab for the layered wall, the thermal bridge as the wall concrete band in slab height plus the break module, post and handrail; the baluster panel carries no surface here.",
            "Thermisch: drei einfache Häute, ein Wandblock über der Platte für die geschichtete Wand, die Wärmebrücke als Wandbetonband in Plattenhöhe und das Dämmelement, Pfosten und Handlauf; das Stabfeld trägt hier keine Fläche.",
        ),
        text(
            "Structure: slab member, wall core as the fixed support, and the connection as its two layers across the insulation, a steel tension layer on top and a concrete pressure layer at the bottom, so the path slab, joint, wall reads; post and handrail are the railing member.",
            "Tragwerk: Plattenstab, Wandkern als festes Auflager und der Anschluss als seine zwei Lagen durch die Dämmung, oben eine stählerne Zuglage und unten eine Betondrucklage, sodass der Weg Platte, Anschluss, Wand lesbar ist; Pfosten und Handlauf sind der Geländerstab.",
        ),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale500():
    mass = [[-0.3, -0.1], [1.5, -0.1], [1.5, 0.1], [0.0, 0.1], [0.0, 1.2], [-0.3, 1.2]]
    body = [
        cut(prism("balcony-solid", SL, CO, T_CO, mass, "xy", -0.5, 0.5), "-w", "+w"),
        box("railing-post", RA, STL, T_ST, [K.RAIL_H - 0.075, 0.075, 0.075], [(K.RAIL_H - 0.075) / 2, 0, 0]),
        cut(box("railing-handrail", RA, STL, T_ST, [0.075, 0.075, 1.0], [K.RAIL_H - 0.0375, 0, 0]), "-w", "+w"),
    ]
    thermal = [
        th(cut(prism("th-slab-top", SL, CO, 0.75, [[0, 0.1], [0.16, 0.02], [1.34, 0.02], [1.5, 0.1]], "xy", -0.5, 0.5), "-v", "-w", "+w"), "slab-top"),
        th(cut(prism("th-slab-soffit", SL, CO, 0.6, [[0, -0.1], [1.5, -0.1], [1.34, -0.02], [0.16, -0.02]], "xy", -0.5, 0.5), "+v", "-w", "+w"), "slab-soffit"),
        th(cut(prism("th-slab-edge", SL, CO, 0.68, [[1.5, 0.1], [1.34, 0.02], [1.34, -0.02], [1.5, -0.1]], "xy", -0.5, 0.5), "-u", "+v", "-v", "-w", "+w"), "slab-edge"),
        th(cut(box("th-break-core", SL, CO, T_WALL, [0.18, 0.2, 1.0], [-0.21, 0, 0]), "-u", "+v", "-v", "-w", "+w"), "thermal-break"),
        th(cut(box("th-break", SL, BM, T_BM, [0.12, 0.2, 1.0], [-0.06, 0, 0]), "-u", "-w", "+w"), "thermal-break"),
        th(cut(box("th-wall", SL, CO, T_WALL, [0.3, 1.1, 1.0], [-0.15, 0.65, 0]), "-u", "+v", "-v", "-w", "+w"), "wall-face"),
        th_of("th-railing-post", "railing-post", "railing-steel"),
        th_of("th-railing-handrail", "railing-handrail", "railing-steel", faces=["-w", "+w"]),
    ]
    structure = [
        st(box("slab-member", SL, CO, T_CO, [1.5, 0.2, 1.0], [0.75, 0, 0]), SL),
        st(box("joint-wall-connection", SL, STL, 0.2, [0.12, 0.2, 1.0], [-0.06, 0, 0]), SL, "joint"),
        st(box("support-wall-core", SL, CO, T_WALL, [0.18, 1.3, 1.0], [-0.21, 0.55, 0]), SL, "support", "wall-fix"),
        st_of("railing-post-member", "railing-post", RA),
        st_of("railing-handrail-member", "railing-handrail", RA),
    ]
    decisions = [
        text(
            "Body: redrawn for 75 mm. Wall, joint and slab are one L-shaped poché mass with a level top; the railing is one 75 mm post under one 75 mm rail across the strip, the least a post and a rail can be at this scale.",
            "Körper: neu gezeichnet für 75 mm. Wand, Anschluss und Platte sind eine L-förmige Poché-Masse mit ebener Oberseite; das Geländer ist ein Pfosten von 75 mm unter einem Holm von 75 mm über den Streifen, das Geringste, was Pfosten und Holm in diesem Maßstab sein können.",
        ),
        text(
            "Thermal: every region keeps its own piece inside the mass, top, soffit and edge skins of 80 mm, the thermal bridge as wall concrete band in slab height and break module, the wall block above the slab, and the railing as post and rail.",
            "Thermisch: jede Region behält ihr eigenes Stück in der Masse, Häute von 80 mm für Oberseite, Unterseite und Stirn, die Wärmebrücke als Wandbetonband in Plattenhöhe und Dämmelement, den Wandblock über der Platte und das Geländer als Pfosten und Holm.",
        ),
        text(
            "Structure: three blocks in a row carry the load, the slab member, the connection in the insulation plane and the wall core as the fixed support; post and rail are the railing member that takes the barrier load. This is the minimal model.",
            "Tragwerk: drei Blöcke in Reihe tragen die Last, der Plattenstab, der Anschluss in der Dämmebene und der Wandkern als festes Auflager; Pfosten und Holm sind der Geländerstab, der die Holmlast trägt. Dies ist das minimale Modell.",
        ),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


SLAB_AREA, SLAB_CX, SLAB_CY = K.polygon(K.SLAB_PROFILE)
SLAB_COM = (SLAB_CX, SLAB_CY)

if __name__ == "__main__":
    volumes = {"50": scale50(), "100": scale100(), "200": scale200(), "500": scale500()}
    path = os.path.join(T, "volumes", "components", "cantilever-balcony.json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print("slab section area", round(SLAB_AREA, 6), "com", round(SLAB_CX, 4), round(SLAB_CY, 4), {k: [len(v[l]) for l in ("body", "thermal", "structure")] for k, v in volumes.items()})
