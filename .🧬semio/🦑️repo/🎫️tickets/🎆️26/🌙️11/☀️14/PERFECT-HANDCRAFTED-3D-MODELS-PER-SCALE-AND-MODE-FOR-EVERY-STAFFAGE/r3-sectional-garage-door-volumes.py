"""🚪 Round 3 sectional garage door: every solid of every scale and list, stated by hand, written to volumes/components/sectional-garage-door.json.

Usage: python3 r3-sectional-garage-door-volumes.py
"""
import importlib.util
import os

exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())
_spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-sectional-garage-door-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

ST, SK, PU, EP = "steel", "sandwich-skin", "pu-foam", "epdm"
PS, HW, TR, SP = "panel-sandwich", "hardware-steel", "track-steel", "spring-assembly"
TB, SB = "track", "spring-shaft"
SIDES = (("l", 1), ("r", -1))
QZ180 = [0, 0, 1, 0]


def side(profile, sign):
    return profile if sign > 0 else K.mirror_z(profile)


def pb(k):
    return f"panel-{k}"


def tracks(s, lips, arcs):
    """🛤️ Both roller tracks: vertical run, true quarter circle, horizontal run, one channel section throughout."""
    for sd, sg in SIDES:
        pr = side(K.track_profile(lips), sg)
        s.b(ext(f"rail-up-{sd}", TB, ST, 0.4, "zx", pr, 0, 2.0))
        loop = [[-p[0], K.ARC_R + p[1]] for p in pr]
        loop.append(loop[0])
        for k in range(arcs):
            a = revolve(f"rail-arc-{sd}" if arcs == 1 else f"rail-arc-{sd}-{k + 1}", TB, ST, 0.4, [0, 0, -1], [0, 0, 0], loop, sweep=math.pi / 2 / arcs)
            a["transform"] = {"rotation": qaxis([0, 0, 1], 180 - 90 / arcs * k), "translation": [K.ARC_C[0], K.ARC_C[1], 0]}
            s.b(a)
        s.b(ext(f"rail-back-{sd}", TB, ST, 0.4, "yz", [[K.RUN_Y + p[1], p[0]] for p in pr], K.ARC_C[0], K.TRACK_END))


def frame_hardware(s, feet=True):
    """🔧 What holds the tracks: jamb brackets, flag brackets, the stiffening angle on the horizontal run, punched-angle hangers with ceiling feet and sway braces, the rear spreader."""
    for sd, sg in SIDES:
        s.b(ext(f"jamb-bracket-{sd}", TB, ST, 0.35, "zx", side(K.angle_zx(1.205, K.WALL_X, 0.02), sg), 0.0, K.JAMB_H, repeat={"count": 3, "translation": [0, 0.67, 0]}))
        s.b(ext(f"flag-bracket-{sd}", TB, ST, 0.35, "zx", side(K.angle_zx(1.205, K.WALL_X, 0.06), sg), 1.86, 2.14))
        ang = [[2.526, 1.2175], [2.526, 1.25], [2.575, 1.25], [2.575, 1.2475], [2.5285, 1.2475], [2.5285, 1.2175]]
        s.b(ext(f"track-angle-{sd}", TB, ST, 0.4, "yz", ang if sg > 0 else K.mirror_w(ang), 0.5, K.TRACK_END))
        x0 = K.HANGER_X[0]
        hang = [[1.2175, x0 + 0.0125], [1.2175, x0 + 0.015], [1.2475, x0 + 0.015], [1.2475, x0 - 0.015], [1.245, x0 - 0.015], [1.245, x0 + 0.0125]]
        s.b(ext(f"hanger-{sd}", TB, ST, 0.4, "zx", side(hang, sg), 2.5285, K.CEILING, repeat={"count": 2, "translation": [1.2, 0, 0]}))
        foot = [[2.8475, 1.19], [2.85, 1.19], [2.85, 1.245], [2.80, 1.245], [2.80, 1.2425], [2.8475, 1.2425]]
        if feet:
            s.b(ext(f"ceiling-foot-{sd}", TB, ST, 0.35, "yz", foot if sg > 0 else K.mirror_w(foot), x0 - 0.04, x0 + 0.04, repeat={"count": 2, "translation": [1.2, 0, 0]}))
            s.b(ext(f"brace-foot-{sd}", TB, ST, 0.35, "yz", foot if sg > 0 else K.mirror_w(foot), K.BRACE_B[0] - 0.05, K.BRACE_B[0] + 0.03))
        brace_parts(s, sd, sg)
    bar = [[2.56, 2.48], [2.6, 2.48], [2.6, 2.52], [2.56, 2.52], [2.56, 2.518], [2.598, 2.518], [2.598, 2.482], [2.56, 2.482]]
    s.b(ext("track-end-bar", TB, ST, 0.4, "xy", bar, -K.TRACK_Z0, K.TRACK_Z0))


def brace_parts(s, sd, sg):
    (ax, ay), (bx, by) = K.BRACE_A, K.BRACE_B
    length = math.hypot(bx - ax, by - ay)
    turn = math.degrees(math.atan2(ax - bx, by - ay))
    pr = [[0, -0.015], [-0.0025, -0.015], [-0.0025, 0.0125], [-0.03, 0.0125], [-0.03, 0.015], [0, 0.015]]
    d = ext(f"brace-{sd}", TB, ST, 0.4, "zx", pr if sg > 0 else K.mirror_z(pr), 0, length)
    d["transform"] = {"rotation": qaxis([0, 0, 1], turn), "translation": [ax, ay, sg * 1.2425]}
    s.b(d)


def shaft_line(s, detail):
    """🌀 The balance line above the lintel: bearing plates, shaft tube, cable drums, torsion spring between its stationary and its winding cone, anchor bracket."""
    sx, sy = K.SHAFT
    for sd, sg in SIDES:
        s.b(ext(f"bearing-plate-{sd}", TB, ST, 0.4, "zx", side(K.angle_zx(1.205, K.WALL_X, 0.13), sg), 2.66, 2.84))
        if detail:
            brg = [[0, 0.0135], [0.016, 0.0135], [0.016, 0.025], [0.004, 0.025], [0.004, 0.03], [0, 0.03], [0, 0.0135]]
            s.b(revolve(f"bearing-{sd}", TB, ST, 0.25, [0, 0, sg], [sx, sy, sg * 1.2315], brg, role="detail"))
        s.b(revolve(f"drum-{sd}", SB, ST, 0.5, [0, 0, sg], [0, 0, sg * K.DRUM_Z0], K.drum_profile(detail)))
    if detail:
        s.b(revolve("shaft", SB, ST, 0.2, [0, 0, 1], [0, 0, -1.25], [[0, 0], [0, K.SHAFT_R], [2.5, K.SHAFT_R], [2.5, 0]], role="detail"))
        wire = [[(K.WIRE_R - 0.0001) * math.cos(2 * math.pi * k / 12), K.COIL_R + K.WIRE_R * math.sin(2 * math.pi * k / 12)] for k in range(12)]
        wire.append(wire[0])
        s.b(revolve("coil", SB, ST, 0.4, [0, 0, 1], [0, 0, -K.COIL_HALF + K.COIL_PITCH / 2], wire, role="detail", repeat={"count": K.COIL_TURNS, "translation": [0, 0, K.COIL_PITCH]}))
        fixed = [[0, 0.0135], [0, 0.0295], [0.018, 0.0295], [0.018, 0.045], [0.03, 0.045], [0.03, 0.0135], [0, 0.0135]]
        s.b(revolve("cone-stationary", TB, ST, 0.3, [0, 0, -1], [sx, sy, -K.COIL_HALF + 0.018], fixed, role="detail"))
        wind = [[0, K.SHAFT_R], [0, 0.0295], [0.018, 0.0295], [0.018, 0.04], [0.058, 0.04], [0.058, K.SHAFT_R], [0, K.SHAFT_R]]
        s.b(revolve("cone-winding", SB, ST, 0.3, [0, 0, 1], [0, 0, K.COIL_HALF - 0.018], wind, role="detail"))
        s.b(box("cone-winding-sockets", SB, ST, 0.3, [0.1, 0.016, 0.016], [0, 0, K.COIL_HALF + 0.022], role="detail", repeat={"count": 2, "rotationAxis": [0, 0, 1], "rotationRad": r6(math.pi / 2)}))
    else:
        s.b(revolve("shaft", SB, ST, 0.2, [0, 0, 1], [0, 0, -1.25], [[0, 0], [0, K.SHAFT_R], [2.5, K.SHAFT_R], [2.5, 0]], role="detail"))
        n, r, c, root = 2 * K.COIL_HALF, K.COIL_R + K.WIRE_R, 0.045, K.COIL_R - K.WIRE_R
        turns = [[0, 0], [0, c], [0.016, c], [0.016, root]]
        for k in range(1, 60):
            a = 0.016 * k
            turns += [[a + 0.003, r], [a + 0.011, r], [a + 0.014, root], [a + 0.016, root]]
        turns += [[n - 0.016, c], [n, c], [n, 0]]
        s.b(revolve("coil", SB, ST, 0.4, [0, 0, 1], [0, 0, -K.COIL_HALF], turns, role="detail"))
    s.b(ext("anchor-bracket", TB, ST, 0.35, "zx", anchor_plate(), 2.68, 2.82))


def anchor_plate():
    """🧱 The spring anchor bracket as (z, x) at the stationary end on the right: plate square to the wall with its wall leg."""
    z0, z1 = -K.COIL_HALF - 0.0145, -K.COIL_HALF - 0.012
    return [[z0 - 0.06, K.WALL_X], [z1, K.WALL_X], [z1, 0.125], [z0, 0.125], [z0, K.WALL_X + K.SHEET], [z0 - 0.06, K.WALL_X + K.SHEET]]


def leaf50(s):
    """🚪 The door leaf at 1:50: four sandwich panels with the finger-protection joint and the centre rib, end stiles, bottom rail with bulb seal, top cap."""
    for k in range(1, 5):
        pts, _, _ = K.panel_outline(k)
        s.b(ext(pb(k), pb(k), SK, 0.55, "xy", pts, -K.HALF, K.HALF))
        u0 = K.RAIL_TOP if k == 1 else 0.001
        u1 = K.TOP_CAP_U if k == 4 else K.PANEL
        stile = [[K.V_IN - 0.0015, 1.18], [K.V_IN - 0.0015, K.CAP], [K.V_OUT + 0.0015, K.CAP], [K.V_OUT + 0.0015, 1.18], [K.V_OUT, 1.18], [K.V_OUT, K.HALF], [K.V_IN, K.HALF], [K.V_IN, 1.18]]
        for sd, sg in SIDES:
            s.b(ext(f"stile-{k}-{sd}", pb(k), ST, 0.45, "yz", stile if sg > 0 else K.mirror_w(stile), u0, u1, role="detail"))
    lo, hi, a, b, c = K.V_IN - 0.0015, K.V_OUT + 0.0015, K.RAIL_U, K.RAIL_U + 0.006, K.RAIL_TOP
    s.b(ext("bottom-rail", pb(1), ST, 0.35, "xy", [[a, lo], [c, lo], [c, K.V_IN], [b, K.V_IN], [b, K.V_OUT], [c, K.V_OUT], [c, hi], [a, hi]], -K.HALF, K.HALF, role="detail"))
    bulb = [[a, 0.024], [a, 0.056], [0.012, 0.058], [0.004, 0.054], [0.0, 0.046], [0.0, 0.034], [0.004, 0.026], [0.012, 0.022]]
    bore = [[a - 0.003, 0.027], [a - 0.003, 0.053], [0.012, 0.055], [0.006, 0.052], [0.003, 0.045], [0.003, 0.035], [0.006, 0.028], [0.012, 0.025]]
    s.b(ext("bottom-seal", pb(1), EP, 0.15, "xy", bulb, -K.HALF, K.HALF, holes=[bore], role="detail"))
    t0, t1, e = K.TOP_CAP_U, K.TOP_FLAT, K.PANEL
    s.b(ext("top-cap", pb(4), ST, 0.35, "xy", [[e, lo], [e, hi], [t0, hi], [t0, K.V_OUT], [t1, K.V_OUT], [t1, K.V_IN], [t0, K.V_IN], [t0, lo]], -K.HALF, K.HALF, role="detail"))


def hardware50(s):
    """🔩 Hinges, rollers and roller carriers at 1:50: per joint two centre hinges and two side hinges whose pin is the roller stem, bottom and top carriers with their rollers, the cable studs."""
    c0 = K.HINGE_C[0]
    rep = {"count": 4, "translation": [0, 0, K.HINGE_STEP]}
    for j in range(1, 4):
        s.b(ext(f"hinge-leaf-up-{j}", pb(j + 1), ST, 0.3, "xy", K.leaf_upper(), c0 - 0.0175, c0 + 0.0175, role="detail", repeat=rep))
        s.b(ext(f"hinge-leaf-low-a-{j}", pb(j), ST, 0.3, "xy", K.leaf_lower(), c0 - 0.04, c0 - 0.0185, role="detail", repeat=rep))
        s.b(ext(f"hinge-leaf-low-b-{j}", pb(j), ST, 0.3, "xy", K.leaf_lower(), c0 + 0.0185, c0 + 0.04, role="detail", repeat=rep))
        pin = [[0, 0], [0, 0.006], [0.003, 0.006], [0.003, 0.004], [0.083, 0.004], [0.083, 0.006], [0.086, 0.006], [0.086, 0]]
        s.b(revolve(f"hinge-pin-{j}", pb(j + 1), ST, 0.2, [0, 0, 1], [0, 0, K.HINGE_C[1] - 0.043], pin, role="detail", repeat={"count": 2, "translation": [0, 0, K.HINGE_STEP]}))
    for name, bone, u in [("0", pb(1), K.BOTTOM_ROLLER_U), ("1", pb(1), K.PANEL), ("2", pb(2), K.PANEL), ("3", pb(3), K.PANEL), ("4", pb(4), K.TOP_ROLLER_U)]:
        for sd, sg in SIDES:
            s.b(revolve(f"roller-{name}-{sd}", bone, ST, 0.2, [0, 0, sg], [u, 0, sg * K.STEM_Z0], K.roller_profile(), role="detail"))
    rep2 = {"count": 2, "translation": [0, 0, -2.29]}
    s.b(ext("bottom-bracket", pb(1), ST, 0.3, "xy", K.roller_bracket(K.BOTTOM_ROLLER_U, K.RAIL_TOP + 0.001, 0.15), 1.11, 1.18, role="detail", repeat=rep2))
    s.b(ext("top-bracket", pb(4), ST, 0.3, "xy", K.roller_bracket(K.TOP_ROLLER_U, 0.37, K.TOP_CAP_U - 0.002), 1.11, 1.18, role="detail", repeat=rep2))
    stud = [[0, 0], [0, 0.0075], [0.066, 0.0075], [0.066, 0.005], [0.072, 0.005], [0.072, 0.0075], [0.0765, 0.0075], [0.0765, 0]]
    for sd, sg in SIDES:
        s.b(revolve(f"cable-stud-{sd}", pb(1), ST, 0.2, [0, 0, sg], [K.CABLE_EYE[0], K.CABLE_EYE[1], sg * 1.14], stud, role="detail"))


def thermal50(s):
    """🌡️ The sandwich as three nested layers per panel; edge profiles and seal on the panel region; hinges, rollers and carriers as the hardware surface; track frame and spring line from the body."""
    for k in range(1, 5):
        outer, core, inner, _ = K.skins(k)
        d = ext(f"th-skin-inner-{k}", pb(k), SK, 0.55, "xy", inner, -K.HALF, K.HALF)
        s.th(d, PS, "skin-inner")
        s.th(ext(f"th-core-{k}", pb(k), PU, 0.85, "xy", core, -K.HALF, K.HALF), PS, "pu-core")
        d = ext(f"th-skin-outer-{k}", pb(k), SK, 0.55, "xy", outer, -K.HALF, K.HALF)
        d["sectionFaces"] = ["-v"]
        s.th(d, PS, "skin-outer")
    glued = {"bottom-rail": ["-v", "+u"], "top-cap": ["-v", "-u"], "bottom-seal": ["-v", "-u"]}
    for d in s.body:
        i = d["id"]
        if i.startswith(("stile", "bottom-rail", "top-cap", "bottom-seal")):
            faces = glued.get(i) or ["-v", "-w" if i.endswith("-l") else "+w"]
            s.thermal.append({"id": f"th-{i}", "of": i, "region": PS, "layer": "skin-outer", "sectionFaces": faces})
        elif i.startswith(("hinge", "roller", "bottom-bracket", "top-bracket", "cable-stud")):
            s.thof(f"th-{i}", i, HW, "hw-plate")
        elif i.startswith(("rail", "jamb", "flag", "track", "hanger", "ceiling", "brace", "bearing", "anchor", "cone-stationary")):
            s.thof(f"th-{i}", i, TR, "track-wall")
        elif i.startswith(("shaft", "drum", "coil", "cone-winding")):
            s.thof(f"th-{i}", i, SP, "coil-wire")


def supports(s, bolt_r, bolt_l):
    """⚓ The three supports as their fixings: the foot anchors into the jamb, the ceiling anchors of the two hanger pairs."""
    stem = [[0, 0], [0, bolt_r], [bolt_l, bolt_r], [bolt_l, 0]]
    s.st(revolve("support-jamb-fix", TB, ST, 0.3, [1, 0, 0], [K.WALL_X - bolt_l + 0.0025, K.JAMB_H / 2, 1.2275], stem, repeat={"count": 2, "translation": [0, 0, -2.455]}), TB, "support")["anchor"] = "jamb-fix"
    for n, x in ((1, K.HANGER_X[0]), (2, K.HANGER_X[1])):
        s.st(revolve(f"support-hanger-{n}", TB, ST, 0.3, [0, 1, 0], [x, K.CEILING - 0.0025, 1.2175], stem, repeat={"count": 2, "translation": [0, 0, -2.435]}), TB, "support")["anchor"] = f"hanger-{n}"


def structure50(s):
    """🏗️ The load path: every panel a hollow steel-skin beam between its end stiles, hinge pins and roller stems as the joints, rollers into the tracks, track frame down to the jamb and up to the ceiling, shaft in its bearings with the spring as mass."""
    for k in range(1, 5):
        pts, _, _ = K.panel_outline(k, rib=False)
        s.st(ext(f"m-skins-{k}", pb(k), SK, 0.55, "xy", pts, -K.HALF, K.HALF, holes=[K.inset(pts, 0.0005)]), pb(k), "member")
        for sd, _ in SIDES:
            s.stof(f"m-stile-{k}-{sd}", f"stile-{k}-{sd}", pb(k), "member")
        s.st(box(f"mass-panel-{k}", pb(k), SK, 0.5, [0.06, 0.05, 0.06], [0.25, K.V_IN + K.THICK / 2, 0]), pb(k), "mass")
    for d in s.body:
        i = d["id"]
        if i.startswith(("rail", "jamb", "flag", "track", "hanger", "ceiling", "brace", "bearing-plate", "anchor")):
            s.stof(f"m-{i}", i, TB, "member")
        elif i.startswith("hinge-pin"):
            s.stof(f"j-{i}", i, d["bone"], "joint")
        elif i.startswith(("hinge-leaf", "bottom-bracket", "top-bracket")):
            s.stof(f"m-{i}", i, d["bone"], "member")
        elif i.startswith("roller"):
            s.stof(f"j-{i}", i, d["bone"], "joint")
        elif i.startswith("cable-stud"):
            s.stof(f"j-{i}", i, d["bone"], "joint")
        elif i.startswith("bearing-"):
            s.stof(f"j-{i}", i, SB, "joint")
        elif i == "shaft":
            s.stof("m-shaft", i, SB, "member")
        elif i.startswith("drum"):
            s.stof(f"m-{i}", i, SB, "member")
    s.st(revolve("mass-spring", SB, ST, 0.4, [0, 0, 1], [0, 0, -K.COIL_HALF], [[0, K.SHAFT_R], [2 * K.COIL_HALF, K.SHAFT_R], [2 * K.COIL_HALF, K.COIL_R + K.WIRE_R], [0, K.COIL_R + K.WIRE_R], [0, K.SHAFT_R]]), SB, "mass")
    supports(s, 0.005, 0.06)


def build50():
    s = Scale(50)
    tracks(s, True, 3)
    frame_hardware(s)
    shaft_line(s, True)
    leaf50(s)
    hardware50(s)
    thermal50(s)
    structure50(s)
    s.dec(
        "Body: the door as it is built. Four 40 mm sandwich panels, each one extrusion of its true section over 2.415 m: steel-skinned slab with the embossed centre rib, at the top the finger-protection nose whose flank is an arc of 52 mm about the hinge pin, at the bottom the matching socket behind the outer lip, which leaves the 8 mm shadow groove on the outside; end stiles as folded caps, the bottom panel in its bottom rail with the hollow rubber bulb, the top panel under its cap. The hinge pins lie 20 mm behind the inner face on the bone joints: per joint two centre hinges (cranked leaf with eye on the upper panel, two leaves on the lower one, headed pin) and two side hinges whose pin is the 11 mm stem of a 46 mm roller; bottom and top rollers sit in carriers 80 mm above the floor and 60 mm below the top edge, the cable stud under the bottom carrier. Track frame: 52 x 30 mm channel tracks with curled lips (vertical, true quarter circle R 0.5 m, horizontal), three jamb brackets and a flag bracket per side on the wall plane 75 mm in front of the roller line, the stiffening angle on the horizontal run, punched-angle hangers with ceiling feet and sway braces, the rear spreader. Balance line: bearing plates with flanged bearings, 25 mm shaft, two grooved cable drums, the torsion spring of 122 turns between its stationary cone on the anchor bracket and its winding cone. The running cable changes its length with the pose and cannot follow one bone: it is not a solid; its pull is the load pair of every pose.",
        "Körper: das Tor, wie es gebaut ist. Vier Sandwichpaneele zu 40 mm, jedes eine Extrusion seines wahren Querschnitts über 2,415 m: stahlbeplankte Platte mit geprägter Mittelsicke, oben die Fingerschutznase, deren Flanke ein Bogen von 52 mm um den Scharnierbolzen ist, unten die passende Pfanne hinter der Außenlippe, die außen die 8 mm breite Schattenfuge lässt; Endkappen als gekantete Stirnprofile, das unterste Paneel in seiner Bodenschiene mit hohlem Gummiwulst, das oberste unter seiner Abdeckkappe. Die Scharnierbolzen liegen 20 mm hinter der Innenseite auf den Gelenken der Knochen: je Fuge zwei Mittelscharniere (gekröpfte Lasche mit Auge am oberen Paneel, zwei Laschen am unteren, Kopfbolzen) und zwei Seitenscharniere, deren Bolzen der 11-mm-Schaft einer 46-mm-Laufrolle ist; untere und obere Rollen sitzen in Haltern 80 mm über dem Boden und 60 mm unter der Oberkante, der Seilbolzen unter dem unteren Halter. Schienenrahmen: Laufschienen als Profil 52 x 30 mm mit eingerollten Lippen (senkrecht, echter Viertelkreis R 0,5 m, waagerecht), je Seite drei Zargenwinkel und ein Bogenwinkel auf der Wandebene 75 mm vor der Rollenlinie, der Versteifungswinkel auf der Waagerechten, Lochwinkel-Abhänger mit Deckenfüßen und Schrägstreben, die hintere Querstrebe. Ausgleichslinie: Lagerplatten mit Flanschlagern, Welle 25 mm, zwei gerillte Seiltrommeln, die Torsionsfeder mit 122 Windungen zwischen ihrem festen Konus am Federbock und ihrem Spannkonus. Das laufende Seil ändert seine Länge mit der Pose und kann keinem einzelnen Knochen folgen: es ist kein Körper; sein Zug ist das Lastpaar jeder Pose.",
    )
    s.dec(
        "Thermal: the sandwich of every panel as three nested layers with their real thicknesses, inner skin 0.5 mm, PU core, outer skin 0.5 mm, each following the section round the nose and the socket; stiles, bottom rail, top cap and bulb seal close the panel region at its edges; faces towards the garage are the conditioned interior and marked as such. Hinges, rollers, carriers and cable studs are the hardware surface, tracks, brackets, angles, hangers, braces, spreader, bearing plates, anchor bracket and the stationary cone the track surface, shaft, drums, coil and winding cone the spring surface, all referenced from the body because their thermal surface is the body surface.",
        "Thermisch: das Sandwich jedes Paneels als drei geschachtelte Schichten in wahrer Dicke, Innenhaut 0,5 mm, PU-Kern, Außenhaut 0,5 mm, jede dem Querschnitt um Nase und Pfanne folgend; Endkappen, Bodenschiene, Abdeckkappe und Gummiwulst schließen die Paneelregion an ihren Rändern; garagenseitige Flächen sind der konditionierte Innenraum und so markiert. Scharniere, Rollen, Halter und Seilbolzen sind die Beschlagfläche, Schienen, Winkel, Abhänger, Streben, Querstrebe, Lagerplatten, Federbock und fester Konus die Schienenfläche, Welle, Trommeln, Feder und Spannkonus die Federfläche, alle aus dem Körper referenziert, weil ihre thermische Fläche die Körperfläche ist.",
    )
    s.dec(
        "Structure: the load path in space. Each panel is a hollow beam of its two 0.5 mm steel skins (the foam only keeps them apart) between its end stiles, with its mass at the centre; hinge leaves and roller carriers tie the pins to the panels, hinge pins and roller stems are the joints of the hinge chain, the rollers carry wind and guidance into the channel tracks; tracks, jamb and flag brackets, stiffening angles, hangers, ceiling feet, braces and spreader are the frame, fixed by the foot anchors in the jamb and hung from the ceiling anchors; the shaft turns in its two bearings, carries the drums and the spring mass, and the anchor bracket takes the spring torque into the wall.",
        "Tragwerk: der Lastpfad im Raum. Jedes Paneel ist ein Hohlträger aus seinen zwei 0,5-mm-Stahlhäuten (der Schaum hält sie nur auf Abstand) zwischen seinen Endkappen, mit seiner Masse in der Mitte; Scharnierlaschen und Rollenhalter binden die Bolzen an die Paneele, Scharnierbolzen und Rollenschäfte sind die Gelenke der Scharnierkette, die Rollen leiten Wind und Führung in die Laufschienen; Schienen, Zargen- und Bogenwinkel, Versteifungswinkel, Abhänger, Deckenfüße, Streben und Querstrebe sind der Rahmen, gehalten von den Fußankern im Pfosten und abgehängt an den Deckenankern; die Welle dreht in seinen zwei Lagern, trägt die Trommeln und die Federmasse, und der Federbock leitet das Federmoment in die Wand.",
    )
    return s


def panel_refs(s):
    for k in range(1, 5):
        s.thermal.append({"id": f"th-panel-{k}", "of": pb(k), "region": PS, "layer": "skin-outer", "sectionFaces": ["-v"]})


def body_refs(s):
    """🔗 Thermal references of every body solid that is hardware, track frame or spring line."""
    for d in s.body:
        i = d["id"]
        if i.startswith(("hinge", "roller")):
            s.thof(f"th-{i}", i, HW, "hw-plate")
        elif i.startswith(("rail", "jamb", "flag", "track", "hanger", "brace", "bearing", "anchor")):
            s.thof(f"th-{i}", i, TR, "track-wall")
        elif i.startswith(("shaft", "drum", "coil")):
            s.thof(f"th-{i}", i, SP, "coil-wire")


def frame_refs(s, plates=True):
    for d in s.body:
        i = d["id"]
        if not plates and i.startswith(("jamb", "flag", "bearing")):
            continue
        if i.startswith(("rail", "jamb", "flag", "track", "hanger", "brace", "bearing", "anchor")):
            s.stof(f"m-{i}", i, TB, "member")
        elif i == "shaft" or i.startswith("drum"):
            s.stof(f"m-{i}", i, SB, "member")


def hinge_axes(s, thermal, structure, half=K.WHEEL_Z1, body=False):
    """🪡 The hinge line of each joint where hinges and roller stems are below the scale: one rod on the pin axis across the door."""
    for j in range(1, 4):
        d = revolve(f"hinge-axis-{j}", pb(j + 1), ST, 0.25, [0, 0, 1], [0, 0, -half], [[0, 0], [0, K.STEM_R], [2 * half, K.STEM_R], [2 * half, 0]], role="detail")
        d["slender"] = True
        if body:
            s.b(d)
            s.stof(f"j-hinge-axis-{j}", d["id"], pb(j + 1), "joint")
            continue
        if thermal:
            s.th(dict(d, id=f"th-hinge-axis-{j}"), HW, "hw-plate")
        if structure:
            s.st(dict(d, id=f"j-hinge-axis-{j}"), pb(j + 1), "joint")


def build100():
    s = Scale(100)
    tracks(s, False, 2)
    frame_hardware(s, feet=False)
    shaft_line(s, False)
    for k in range(1, 5):
        s.b(ext(pb(k), pb(k), SK, 0.55, "xy", K.panel_outline_100(k), -K.CAP, K.CAP))
    s.b(box("bottom-seal", pb(1), EP, 0.15, [K.RAIL_U, 0.036, 2 * K.CAP], [K.RAIL_U / 2, K.V_IN + K.THICK / 2, 0], role="detail"))
    eye = [[K.EYE_R * math.cos(math.radians(a)), K.EYE_R * math.sin(math.radians(a))] for a in range(135, 406, 45)]
    block = [[0.07, K.V_IN], [0.002, K.V_IN]] + eye + [[0.016, K.V_IN - K.SHEET], [0.07, K.V_IN - K.SHEET]]
    for j in range(1, 4):
        s.b(ext(f"hinge-{j}", pb(j + 1), ST, 0.3, "xy", block, K.HINGE_C[0] - 0.04, K.HINGE_C[0] + 0.04, role="detail", repeat={"count": 4, "translation": [0, 0, K.HINGE_STEP]}))
        s.b(ext(f"hinge-low-{j}", pb(j), ST, 0.3, "xy", [[K.PANEL - u, v] for u, v in block][::-1], K.HINGE_C[0] - 0.04, K.HINGE_C[0] + 0.04, role="detail", repeat={"count": 4, "translation": [0, 0, K.HINGE_STEP]}))
    wheel = [[0, 0], [0, K.WHEEL_R], [K.WHEEL_Z1 - K.WHEEL_Z0, K.WHEEL_R], [K.WHEEL_Z1 - K.WHEEL_Z0, 0]]
    for name, bone, u in [("0", pb(1), K.BOTTOM_ROLLER_U), ("1", pb(1), K.PANEL), ("2", pb(2), K.PANEL), ("3", pb(3), K.PANEL), ("4", pb(4), K.TOP_ROLLER_U)]:
        s.b(revolve(f"roller-{name}", bone, ST, 0.2, [0, 0, 1], [u, 0, K.WHEEL_Z0], wheel, role="detail", repeat={"count": 2, "translation": [0, 0, -(K.WHEEL_Z0 + K.WHEEL_Z1)]}))
    panel_refs(s)
    s.thermal.append({"id": "th-bottom-seal", "of": "bottom-seal", "region": PS, "layer": "skin-outer", "sectionFaces": ["-v", "-u"]})
    body_refs(s)
    for k in range(1, 5):
        u0 = K.RAIL_U if k == 1 else 0.001
        tube = [[u0, K.V_IN], [K.PANEL, K.V_IN], [K.PANEL, K.V_OUT], [u0, K.V_OUT]]
        s.st(ext(f"m-skins-{k}", pb(k), SK, 0.55, "xy", tube, -K.CAP, K.CAP, holes=[K.inset(tube, 0.0005)]), pb(k), "member")
        s.st(box(f"mass-panel-{k}", pb(k), SK, 0.5, [0.08, 0.05, 0.08], [0.25, K.V_IN + K.THICK / 2, 0]), pb(k), "mass")
    frame_refs(s)
    for d in s.body:
        if d["id"].startswith(("hinge", "roller")) and "low" not in d["id"]:
            s.stof(f"j-{d['id']}", d["id"], d["bone"], "joint")
    ringp = [[0, K.SHAFT_R], [0.016, K.SHAFT_R], [0.016, 0.027], [0, 0.027], [0, K.SHAFT_R]]
    s.st(revolve("j-bearing", TB, ST, 0.25, [0, 0, 1], [K.SHAFT[0], K.SHAFT[1], 1.2315], ringp, repeat={"count": 2, "translation": [0, 0, -2.479]}), SB, "joint")
    s.st(revolve("mass-spring", SB, ST, 0.4, [0, 0, 1], [0, 0, -K.COIL_HALF + 0.02], [[0, 0], [0, K.COIL_R + K.WIRE_R], [2 * K.COIL_HALF - 0.04, K.COIL_R + K.WIRE_R], [2 * K.COIL_HALF - 0.04, 0]]), SB, "mass")
    supports(s, 0.0075, 0.08)
    s.dec(
        "Body: the door at 15 mm detail, drawn anew. Each panel is one extrusion over the full 2.42 m (stiles merged) of a simplified section: 40 mm slab, the nose as a chord polygon inside the 52 mm pin circle, the socket as a plain recess behind the lip; rib and shadow groove are below the scale. The bottom rail and bulb are one rubber block. A hinge is one cranked leaf with its eye on the upper panel, 80 mm wide (both leaves and the pin merged), four per joint; a roller is its 46 mm wheel alone, one pair per axis (stems of 11 mm and the carriers are below the scale). Tracks are plain channels without lips; jamb and flag brackets, stiffening angles, hangers, braces and spreader stay as angles and channel, the ceiling feet are omitted; shaft as a solid rod, drums without grooves, the spring as one body of 59 double turns between its two cone collars, bearing plates and anchor bracket.",
        "Körper: das Tor in 15-mm-Auflösung, neu gezeichnet. Jedes Paneel ist eine Extrusion über die vollen 2,42 m (Endkappen eingeschmolzen) eines vereinfachten Querschnitts: 40-mm-Platte, die Nase als Sehnenpolygon im 52-mm-Bolzenkreis, die Pfanne als einfache Aussparung hinter der Lippe; Sicke und Schattenfuge liegen unter dem Maßstab. Bodenschiene und Wulst sind ein Gummiblock. Ein Scharnier ist eine gekröpfte Lasche mit Auge am oberen Paneel, 80 mm breit (beide Laschen und der Bolzen zusammengefasst), vier je Fuge; eine Rolle ist nur ihr 46-mm-Rad, ein Paar je Achse (Schäfte von 11 mm und die Halter liegen unter dem Maßstab). Die Schienen sind einfache U-Profile ohne Lippen; Zargen- und Bogenwinkel, Versteifungswinkel, Abhänger, Streben und Querstrebe bleiben Winkel und U-Profil, die Deckenfüße entfallen; Welle als Vollstab, Trommeln ohne Rillen, die Feder als ein Körper aus 59 Doppelwindungen zwischen ihren zwei Konusbunden, Lagerplatten und Federbock.",
    )
    s.dec(
        "Thermal: the layers of the sandwich are merged; each panel body is the panel region with its garage face marked as the conditioned interior, the seal block with it. Hinge leaves and roller wheels are the hardware surface, the track frame and the spring line are referenced from the body.",
        "Thermisch: die Schichten des Sandwichs sind zusammengefasst; jeder Paneelkörper ist die Paneelregion, seine Garagenseite als konditionierter Innenraum markiert, der Dichtungsblock gehört dazu. Scharnierlaschen und Rollenräder sind die Beschlagfläche, Schienenrahmen und Federlinie werden aus dem Körper referenziert.",
    )
    s.dec(
        "Structure: each panel is the rectangular tube of its two steel skins with its mass at the centre, the hinge leaves and the roller wheels are the joints, the track frame the members down to the foot anchors and up to the ceiling anchors, the shaft a member in its two bearing rings with the drums, the spring cylinder the mass.",
        "Tragwerk: jedes Paneel ist das Rechteckrohr seiner zwei Stahlhäute mit seiner Masse in der Mitte, die Scharnierlaschen und Rollenräder sind die Gelenke, der Schienenrahmen die Glieder bis zu den Fußankern und hinauf zu den Deckenankern, die Welle ein Glied in ihren zwei Lagerringen mit den Trommeln, der Federzylinder die Masse.",
    )
    return s


def bars(s, n):
    """➖ Track bars for the scales where the channel is below the feature: the band of the side elevation as one solid of 52 x 30 mm per side, the quarter circle in n chords."""
    for sd, sg in SIDES:
        z0, z1 = (K.TRACK_Z0, K.TRACK_Z1) if sg > 0 else (-K.TRACK_Z1, -K.TRACK_Z0)
        s.b(ext(f"rail-{sd}", TB, ST, 0.4, "xy", K.ribbon(2 * K.TRACK_HALF, n), z0, z1))


def slabs(s):
    for k in range(1, 5):
        s.b(box(pb(k), pb(k), SK, 0.55, [K.PANEL, K.THICK, 2 * K.CAP], [K.PANEL / 2, K.V_IN + K.THICK / 2, 0]))


def rod(s, slender=True):
    d = revolve("shaft", SB, ST, 0.2, [0, 0, 1], [0, 0, -1.25], [[0, 0], [0, K.SHAFT_R], [2.5, K.SHAFT_R], [2.5, 0]], role="detail")
    d["slender"] = slender
    s.b(d)
    for sd, sg in SIDES:
        s.b(revolve(f"drum-{sd}", SB, ST, 0.5, [0, 0, sg], [0, 0, sg * K.DRUM_Z0], [[0, 0], [0, K.DRUM_R], [K.DRUM_W, K.DRUM_R], [K.DRUM_W, 0]]))


def build200():
    s = Scale(200)
    bars(s, 8)
    mid = (2.526 + K.CEILING) / 2
    (ax, ay), (bx, by) = K.BRACE_A, K.BRACE_B
    length, turn = math.hypot(bx - ax, by - ay), math.degrees(math.atan2(ax - bx, by - ay))
    for sd, sg in SIDES:
        s.b(box(f"hanger-{sd}", TB, ST, 0.4, [0.03, K.CEILING - 2.526, 0.03], [K.HANGER_X[0], mid, sg * 1.2325], repeat={"count": 2, "translation": [1.2, 0, 0]}))
        s.b(box(f"brace-{sd}", TB, ST, 0.4, [0.03, length, 0.03], [(ax + bx) / 2, (ay + by) / 2, sg * 1.2325], rot=qaxis([0, 0, 1], turn)))
        s.b(ext(f"bearing-plate-{sd}", TB, ST, 0.4, "zx", side(K.angle_zx(1.205, K.WALL_X, 0.13), sg), 2.66, 2.84))
        s.b(ext(f"jamb-bracket-{sd}", TB, ST, 0.35, "zx", side(K.angle_zx(1.205, K.WALL_X, 0.02), sg), 0.0, K.JAMB_H, repeat={"count": 3, "translation": [0, 0.67, 0]}))
        s.b(ext(f"flag-bracket-{sd}", TB, ST, 0.35, "zx", side(K.angle_zx(1.205, K.WALL_X, 0.06), sg), 1.86, 2.14))
    s.b(dict(box("track-end-bar", TB, ST, 0.4, [0.04, 0.04, 2 * K.TRACK_Z0], [2.58, 2.5, 0]), slender=True))
    rod(s)
    s.b(revolve("coil", SB, ST, 0.4, [0, 0, 1], [0, 0, -K.COIL_HALF], [[0, 0], [0, K.COIL_R + K.WIRE_R], [2 * K.COIL_HALF, K.COIL_R + K.WIRE_R], [2 * K.COIL_HALF, 0]], role="detail"))
    slabs(s)
    panel_refs(s)
    hinge_axes(s, True, True, body=True)
    body_refs(s)
    for k in range(1, 5):
        s.stof(f"m-panel-{k}", pb(k), pb(k), "member")
    frame_refs(s, plates=False)
    s.stof("mass-spring", "coil", SB, "mass")
    f = 0.03
    s.st(box("support-jamb-fix", TB, ST, 0.3, [-K.TRACK_HALF - K.WALL_X, K.JAMB_H, f], [(K.WALL_X - K.TRACK_HALF) / 2, K.JAMB_H / 2, 1.2325], repeat={"count": 2, "translation": [0, 0, -2.465]}), TB, "support")["anchor"] = "jamb-fix"
    for n, x in ((1, K.HANGER_X[0]), (2, K.HANGER_X[1])):
        s.st(box(f"support-hanger-{n}", TB, ST, 0.3, [0.06, f, 0.06], [x, K.CEILING - f / 2, 1.2325], repeat={"count": 2, "translation": [0, 0, -2.465]}), TB, "support")["anchor"] = f"hanger-{n}"
    s.dec(
        "Body: the door at 30 mm detail. Each panel is a plain 40 mm slab (joint profile, seal, stiles merged); each track is one solid bar of 52 x 30 mm along the vertical, the quarter circle and the horizontal run (the channel hollow and the stiffening angle are below the scale); hangers and braces are 30 mm bars, the spreader a 40 mm bar, jamb brackets, flag brackets and bearing plates stay folded plates because the scale draws them face on; shaft as a rod of true 25 mm (slender), two plain drums, the spring as a cylinder. Ceiling feet, hinge leaves, roller wheels and carriers are below 30 mm and omitted; the hinge line of each joint stays as a slender rod of 11 mm on the pin axis.",
        "Körper: das Tor in 30-mm-Auflösung. Jedes Paneel ist eine einfache 40-mm-Platte (Fugenprofil, Dichtung, Endkappen eingeschmolzen); jede Schiene ist ein Vollstab 52 x 30 mm entlang Senkrechte, Viertelkreis und Waagerechte (Profilhohlraum und Versteifungswinkel liegen unter dem Maßstab); Abhänger und Streben sind 30-mm-Stäbe, die Querstrebe ein 40-mm-Stab, Zargenwinkel, Bogenwinkel und Lagerplatten bleiben gekantete Bleche, weil der Maßstab sie in der Fläche zeichnet; Welle als Stab mit wahren 25 mm (schlank), zwei glatte Trommeln, die Feder als Zylinder. Deckenfüße, Scharnierlaschen, Rollenräder und Halter liegen unter 30 mm und entfallen; die Scharnierlinie jeder Fuge bleibt als schlanker 11-mm-Stab auf der Bolzenachse.",
    )
    s.dec(
        "Thermal: each panel slab is the panel region with the garage face as the conditioned interior; the hardware region is the hinge line of each joint, the slender rod of the body (hinges and roller stems merged); tracks, hangers, braces, spreader and bearing plates are the track surface, shaft, drums and spring cylinder the spring surface.",
        "Thermisch: jede Paneelplatte ist die Paneelregion mit der Garagenseite als konditioniertem Innenraum; die Beschlagregion ist die Scharnierlinie jeder Fuge, der schlanke Stab des Körpers (Scharniere und Rollenschäfte zusammengefasst); Schienen, Abhänger, Streben, Querstrebe und Lagerplatten sind die Schienenfläche, Welle, Trommeln und Federzylinder die Federfläche.",
    )
    s.dec(
        "Structure: the panel slabs are the beams, the hinge lines the joints of the chain, track bars, hangers, braces and spreader the frame, the shaft with its drums the turning member, the spring cylinder the mass; the folded 2.5 mm plates (brackets, bearing plates) stand edge on to two of the three views at this scale and are left to the body; the supports are 30 mm blocks: the foot fixing between the wall plane and the track in the lowest jamb bracket and the ceiling fixing on each hanger pair.",
        "Tragwerk: die Paneelplatten sind die Balken, die Scharnierlinien die Gelenke der Kette, Schienenstäbe, Abhänger, Streben und Querstrebe der Rahmen, die Welle mit ihren Trommeln das drehende Glied, der Federzylinder die Masse; die gekanteten 2,5-mm-Bleche (Winkel, Lagerplatten) stehen in diesem Maßstab in zwei der drei Ansichten hochkant und bleiben dem Körper überlassen; die Auflager sind 30-mm-Blöcke: die Fußbefestigung zwischen Wandebene und Schiene im untersten Zargenwinkel und die Deckenbefestigung auf jedem Abhängerpaar.",
    )
    return s


def build500():
    s = Scale(500)
    bars(s, 5)
    s.b(dict(box("track-end-bar", TB, ST, 0.4, [0.04, 0.04, 2 * K.TRACK_Z0], [2.58, 2.5, 0]), slender=True))
    rod(s)
    slabs(s)
    panel_refs(s)
    hinge_axes(s, True, True, K.CAP)
    body_refs(s)
    for k in range(1, 5):
        s.stof(f"m-panel-{k}", pb(k), pb(k), "member")
    frame_refs(s)
    s.st(revolve("mass-spring", SB, ST, 0.4, [0, 0, 1], [0, 0, -K.COIL_HALF], [[0, 0], [0, K.COIL_R + K.WIRE_R], [2 * K.COIL_HALF, K.COIL_R + K.WIRE_R], [2 * K.COIL_HALF, 0]]), SB, "mass")
    f = 0.075
    s.st(box("support-jamb-fix", TB, ST, 0.3, [f, f, f], [K.WALL_X + f / 2, f / 2, 1.2125], repeat={"count": 2, "translation": [0, 0, -2.425]}), TB, "support")["anchor"] = "jamb-fix"
    for n, x in ((1, K.HANGER_X[0]), (2, K.HANGER_X[1])):
        s.st(box(f"support-hanger-{n}", TB, ST, 0.3, [f, K.CEILING - 2.526, f], [x, (2.526 + K.CEILING) / 2, 1.2125], repeat={"count": 2, "translation": [0, 0, -2.425]}), TB, "support")["anchor"] = f"hanger-{n}"
    s.dec(
        "Body: the door as the scale draws it, minimal. Four 40 mm panel slabs hinged on the roller line, the two tracks as bars of their true 52 x 30 mm, as the scale draws them, along the vertical, the quarter circle and the horizontal run, the rear spreader as a slender bar, the shaft as a slender rod with its two drums. Hangers, brackets, spring, hinges and rollers are below 75 mm and not in the body.",
        "Körper: das Tor, wie der Maßstab es zeichnet, minimal. Vier 40-mm-Paneelplatten, gelenkig auf der Rollenlinie, die zwei Schienen als Stäbe mit ihren wahren 52 x 30 mm, wie der Maßstab sie zeichnet, entlang Senkrechte, Viertelkreis und Waagerechte, die hintere Querstrebe als schlanker Stab, die Welle als schlanker Stab mit ihren zwei Trommeln. Abhänger, Winkel, Feder, Scharniere und Rollen liegen unter 75 mm und sind nicht im Körper.",
    )
    s.dec(
        "Thermal: panel slabs with the garage face as the conditioned interior, the hinge lines as slender rods for the hardware region, track bars for the track region, shaft and drums for the spring region.",
        "Thermisch: Paneelplatten mit der Garagenseite als konditioniertem Innenraum, die Scharnierlinien als schlanke Stäbe für die Beschlagregion, Schienenstäbe für die Schienenregion, Welle und Trommeln für die Federregion.",
    )
    s.dec(
        "Structure: panel slabs as beams on the hinge lines, the track bars as the frame, the shaft with its drums, the spring as a cylinder mass on the shaft; the supports are feature-sized blocks: the foot fixing on the wall plane and, for each hanger pair, the hanger with its ceiling fixing merged into one post from the track to the ceiling.",
        "Tragwerk: Paneelplatten als Balken auf den Scharnierlinien, die Schienenstäbe als Rahmen, die Welle mit ihren Trommeln, die Feder als Zylindermasse auf der Welle; die Auflager sind Blöcke in Merkmalgröße: die Fußbefestigung auf der Wandebene und je Abhängerpaar der Abhänger mit seiner Deckenbefestigung, zu einem Pfosten von der Schiene bis zur Decke zusammengefasst.",
    )
    return s


if __name__ == "__main__":
    scales = [build50(), build100(), build200(), build500()]
    path = os.path.join(T, "volumes", "components", "sectional-garage-door.json")
    cur = json.load(open(path, encoding="utf-8"))
    for s in scales:
        cur["volumes"][str(s.den)] = s.out()
    with open(path, "w", encoding="utf-8") as f:
        json.dump(cur, f, ensure_ascii=False, indent=1)
    print({k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in cur["volumes"].items()})
