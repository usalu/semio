"""🚪 Round 3 sliding door: the handcrafted solids of every scale (body, thermal, structure), stated part by part, written to the volumes source `volumes/components/sliding-door.json`.

Usage: python3 r3-sliding-door-volumes.py
"""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-sliding-door-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
box, prism, rod, turned, capsule, ellipsoid, repeat, cut, flag, th, th_of, st, st_of, text, r5 = K.box, K.prism, K.rod, K.turned, K.capsule, K.ellipsoid, K.repeat, K.cut, K.flag, K.th, K.th_of, K.st, K.st_of, K.text, K.r5

TR, LF = "track", "leaf"
STL, GL, SEAL, CAV = "steel", "glass", "seal", "frame-cavity"
T_PORTAL, T_LEAF, T_DARK, T_GLASS, T_SEAL, T_AIR = 0.42, 0.48, 0.22, 0.82, 0.12, 0.88
PI = 3.14159265
JAMBS = (("west", 0.0, 1), ("east", 3.0, -1))
H, HV = K.HALF, K.HANGER_V

def stiles(prefix, u0, u1, rounded, hollow):
    """🚪 The two leaf stiles, 50 x 50 steel tubes along u, in (v, w)."""
    out = []
    r = K.R_LEAF if rounded else 0.0
    for name, sign in (("w", -1), ("e", 1)):
        v0, v1 = sorted((sign * K.V_IN, sign * K.V_OUT))
        s = prism(f"{prefix}stile-{name}", LF, STL, T_LEAF, K.rounded_rect(v0, -K.W_LEAF, v1, K.W_LEAF, r), "yz", u0, u1)
        if hollow:
            s["holes"] = [K.rounded_rect(v0 + K.T_LEAF, -K.W_LEAF + K.T_LEAF, v1 - K.T_LEAF, K.W_LEAF - K.T_LEAF, max(r - K.T_LEAF, 0.0))]
        out.append(s)
    return out


def rails(prefix, rounded, hollow):
    """🚪 Top rail 50 x 50 and bottom rail 50 x 100 with its guide groove, between the stiles, in (w, u)."""
    r = K.R_LEAF if rounded else 0.0
    top = prism(f"{prefix}rail-top", LF, STL, T_LEAF, K.rounded_rect(-K.W_LEAF, K.U_TOP, K.W_LEAF, K.U_TOP_RAIL, r), "zx", -K.V_IN, K.V_IN)
    bottom = prism(f"{prefix}rail-bottom", LF, STL, T_LEAF, K.bottom_rail_profile(), "zx", -K.V_IN, K.V_IN)
    if hollow:
        top["holes"] = [K.rounded_rect(-K.W_LEAF + K.T_LEAF, K.U_TOP + K.T_LEAF, K.W_LEAF - K.T_LEAF, K.U_TOP_RAIL - K.T_LEAF, max(r - K.T_LEAF, 0.0))]
        bottom["holes"] = [K.bottom_rail_profile(K.T_LEAF)]
    return [top, bottom]


def handle50(face):
    """🤚 One back-to-back pull handle on one glass face (face +1 exterior, -1 interior): two rosettes, two stand-off legs, the 16 mm grip with rounded ends."""
    s = "ext" if face > 0 else "int"
    w0, w1, rr = K.ROSETTE
    ax = [0, 0, face]
    rosette = repeat(turned(f"handle-rosette-{s}", LF, STL, T_DARK, [K.LEG_U[0], K.HANDLE_V, face * w0], ax, [[0.0, rr], [w1 - w0 - 0.002, rr], [w1 - w0, rr - 0.002], [w1 - w0, 0.0]]), 2, [K.LEG_U[1] - K.LEG_U[0], 0, 0])
    legs = repeat(capsule(f"handle-legs-{s}", LF, STL, T_DARK, [K.LEG_U[0], K.HANDLE_V, face * (w1 - 0.002)], [K.LEG_U[0], K.HANDLE_V, face * K.GRIP_W], 0.008), 2, [K.LEG_U[1] - K.LEG_U[0], 0, 0])
    grip = capsule(f"handle-grip-{s}", LF, STL, T_DARK, [K.HANDLE_U[0] + 0.008, K.HANDLE_V, face * K.GRIP_W], [K.HANDLE_U[1] - 0.008, K.HANDLE_V, face * K.GRIP_W], 0.008)
    return [rosette, legs, grip]


def beads50(face):
    """🪟 The 15 x 15 mm steel glazing beads round the pane on one face: two along the stiles, two between them along the rails."""
    s = "ext" if face > 0 else "int"
    wc = face * (K.GLASS_T / 2 + K.BEAD / 2)
    glass_u = K.U_GLASS_BOT - K.U_TOP_RAIL
    side = repeat(box(f"bead-stiles-{s}", LF, STL, T_LEAF, [glass_u, K.BEAD, K.BEAD], [(K.U_TOP_RAIL + K.U_GLASS_BOT) / 2, -K.V_IN + K.BEAD / 2, wc]), 2, [0, 2 * K.V_IN - K.BEAD, 0])
    across = repeat(box(f"bead-rails-{s}", LF, STL, T_LEAF, [K.BEAD, 2 * (K.V_IN - K.BEAD), K.BEAD], [K.U_TOP_RAIL + K.BEAD / 2, 0, wc]), 2, [glass_u - K.BEAD, 0, 0])
    return [side, across]


def hangers50():
    """🪝 Two pendant hangers: trolley chassis in the track chamber on four crowned wheels and stub axles, the M12 hanger rod through the brush-sealed slot, adjusting and lock nut, the clevis on the top rail and its 16 mm pin."""
    out = []
    chassis = [[-0.022, -0.006], [0.022, -0.006], [0.022, 0.04], [0.017, 0.04], [0.017, 0.01], [-0.017, 0.01], [-0.017, 0.04], [-0.022, 0.04]]
    out.append(repeat(prism("trolley-chassis", LF, STL, T_DARK, chassis, "zx", -HV - K.TROLLEY_HALF, -HV + K.TROLLEY_HALF, role="detail"), 2, [0, 2 * HV, 0]))
    w0, w1 = K.WHEEL_W
    wheel = [[0.0, K.WHEEL_R - 0.002], [0.002, K.WHEEL_R], [w1 - w0 - 0.002, K.WHEEL_R], [w1 - w0, K.WHEEL_R - 0.002], [w1 - w0, 0.0]]
    for trolley, vc in (("a", -HV), ("b", HV)):
        for side, sign in (("ext", 1), ("int", -1)):
            out.append(repeat(turned(f"trolley-{trolley}-wheels-{side}", LF, STL, T_DARK, [K.WHEEL_U, vc - K.WHEEL_DV, sign * w0], [0, 0, sign], wheel, role="detail"), 2, [0, 2 * K.WHEEL_DV, 0]))
            out.append(repeat(rod(f"trolley-{trolley}-axles-{side}", LF, STL, T_DARK, [K.WHEEL_U, vc - K.WHEEL_DV, sign * 0.017], [0, 0, sign], 0.023, 0.005, role="detail"), 2, [0, 2 * K.WHEEL_DV, 0]))
    out.append(repeat(rod("hanger-rod", LF, STL, T_DARK, [K.ROD_U[0], -HV, 0], [1, 0, 0], K.ROD_U[1] - K.ROD_U[0], 0.006, role="detail"), 2, [0, 2 * HV, 0]))
    for name, u0, u1 in (("lock", K.NUT_U[0], K.NUT_U[1]), ("adjust", K.NUT_U[1], K.NUT_U[2])):
        out.append(repeat(prism(f"hanger-nut-{name}", LF, STL, T_DARK, K.hexagon(-HV, 0.0, 0.019), "yz", u0, u1, role="detail"), 2, [0, 2 * HV, 0]))
    c0, c1 = K.CLEVIS_U
    out.append(repeat(box("hanger-clevis", LF, STL, 0.3, [c1 - c0, 0.05, 0.05], [(c0 + c1) / 2, -HV, 0], cornerRadius=0.003), 2, [0, 2 * HV, 0]))
    out.append(repeat(rod("hanger-pin", LF, STL, T_DARK, [K.PIN_U, -HV, -0.028], [0, 0, 1], 0.056, 0.008, role="detail"), 2, [0, 2 * HV, 0]))
    return out


def jamb_seals():
    return [flag(box(f"jamb-{name}-seal", TR, SEAL, T_SEAL, [K.SEAL_T, -K.FLOOR + K.JAMB_TOP, 0.05], [cx + side * (H - K.CHANNEL_D + K.SEAL_T / 2), (K.FLOOR + K.JAMB_TOP) / 2, 0], role="detail")) for name, cx, side in JAMBS]


def guide_profile():
    g0 = K.FLOOR
    return [[g0, -0.02], [g0, 0.02], [g0 + 0.004, 0.02], [g0 + 0.004, 0.007], [g0 + 0.045, 0.007], [g0 + 0.045, -0.007], [g0 + 0.004, -0.007], [g0 + 0.004, -0.02]]


def scale50():
    body = []
    for name, cx, side in JAMBS:
        body.append(prism(f"jamb-{name}", TR, STL, T_PORTAL, K.jamb_profile(cx, side), "zx", K.FLOOR, K.JAMB_TOP))
    body += jamb_seals()
    body.append(prism("track", TR, STL, T_PORTAL, K.track_profile(), "yz", K.TRACK_X0 + K.CAP_T, K.TRACK_X1 - K.CAP_T))
    body.append(repeat(prism("track-end-caps", TR, STL, T_PORTAL, K.rounded_rect(-H, -H, H, H, K.R_OUT), "yz", K.TRACK_X0, K.TRACK_X0 + K.CAP_T), 2, [K.TRACK_X1 - K.TRACK_X0 - K.CAP_T, 0, 0]))
    body.append(repeat(flag(box("track-slot-brush", TR, SEAL, T_SEAL, [K.TRACK_X1 - K.TRACK_X0 - 2 * K.CAP_T, 0.016, K.SLOT - 0.001], [1.5, -H + 0.008, -K.SLOT / 2], role="detail")), 2, [0, 0, K.SLOT]))
    body.append(prism("floor-guide", TR, STL, T_DARK, guide_profile(), "yz", K.GUIDE_X[0], K.GUIDE_X[1], role="detail"))
    body += stiles("leaf-", K.U_TOP, K.U_RAIL_BOT, True, False)
    body += rails("leaf-", True, False)
    body.append(box("leaf-glass", LF, GL, T_GLASS, [K.U_GLASS_BOT - K.U_TOP_RAIL, 2 * K.V_IN, K.GLASS_T], [(K.U_TOP_RAIL + K.U_GLASS_BOT) / 2, 0, 0], role="glazing"))
    body += beads50(1) + beads50(-1)
    b0, b1 = K.BRUSH_W
    body.append(repeat(box("bottom-seal", LF, SEAL, T_SEAL, [K.U_BRUSH - K.U_RAIL_BOT, 2 * K.V_OUT, b1 - b0], [(K.U_RAIL_BOT + K.U_BRUSH) / 2, 0, -(b0 + b1) / 2], role="detail"), 2, [0, 0, b0 + b1]))
    body += hangers50()
    body += handle50(1) + handle50(-1)

    thermal = [th_of("th-glass", "leaf-glass", "leaf-glass", "glass-pane", ["-w"])]
    for name, cx, side in JAMBS:
        thermal += [
            th_of(f"th-jamb-{name}-b", f"jamb-{name}", "jamb-steel", "skin-b", ["-w"]),
            th(prism(f"th-jamb-{name}-c", TR, CAV, T_AIR, K.jamb_profile(cx, side, K.T_TRACK), "zx", K.FLOOR + K.T_TRACK, K.JAMB_TOP - K.T_TRACK), "jamb-steel", "cavity"),
            th(box(f"th-jamb-{name}-a", TR, STL, T_PORTAL, [2 * (H - K.T_TRACK), -K.FLOOR + K.JAMB_TOP - 2 * K.T_TRACK, K.T_TRACK], [cx, (K.FLOOR + K.JAMB_TOP) / 2, -H + K.T_TRACK / 2]), "jamb-steel", "skin-a"),
        ]
    thermal += [
        th(cut(prism("th-track-b", TR, STL, T_PORTAL, K.rounded_rect(-H, -H, H, H, K.R_OUT), "yz", K.TRACK_X0, K.TRACK_X1), "-w"), "track-steel", "skin-b"),
        th(prism("th-track-c", TR, CAV, T_AIR, K.rounded_rect(-H + K.T_TRACK, -H + K.T_TRACK, H - K.T_TRACK, H - K.T_TRACK, K.R_IN), "yz", K.TRACK_X0 + K.CAP_T, K.TRACK_X1 - K.CAP_T), "track-steel", "cavity"),
        th(box("th-track-a", TR, STL, T_PORTAL, [K.TRACK_X1 - K.TRACK_X0 - 2 * K.CAP_T, 2 * (H - K.T_TRACK), K.T_TRACK], [1.5, 0, -H + K.T_TRACK / 2]), "track-steel", "skin-a"),
    ]
    members = (("stile-w", "yz"), ("stile-e", "yz"), ("rail-top", "zx"), ("rail-bottom", "zx"))
    for name, _ in members:
        thermal.append(th_of(f"th-leaf-{name}-b", f"leaf-{name}", "leaf-frame", "skin-b", ["-w"]))
    for s in stiles("th-leaf-cav-", K.U_TOP + K.T_LEAF, K.U_RAIL_BOT - K.T_LEAF, False, False):
        v0 = min(p[0] for p in s["profile"]) + K.T_LEAF
        v1 = max(p[0] for p in s["profile"]) - K.T_LEAF
        s["profile"] = K.rounded_rect(v0, -K.W_LEAF + K.T_LEAF, v1, K.W_LEAF - K.T_LEAF, 0.0)
        s["material"], s["tone"] = CAV, T_AIR
        thermal.append(th(s, "leaf-frame", "cavity"))
    thermal.append(th(prism("th-leaf-cav-rail-top", LF, CAV, T_AIR, K.rounded_rect(-K.W_LEAF + K.T_LEAF, K.U_TOP + K.T_LEAF, K.W_LEAF - K.T_LEAF, K.U_TOP_RAIL - K.T_LEAF, 0.0), "zx", -K.V_IN, K.V_IN), "leaf-frame", "cavity"))
    thermal.append(th(prism("th-leaf-cav-rail-bottom", LF, CAV, T_AIR, K.bottom_rail_profile(K.T_LEAF), "zx", -K.V_IN, K.V_IN), "leaf-frame", "cavity"))
    wa = -K.W_LEAF + K.T_LEAF / 2
    thermal.append(th(repeat(box("th-leaf-a-stiles", LF, STL, T_LEAF, [K.U_RAIL_BOT - K.U_TOP - 2 * K.T_LEAF, 0.05 - 2 * K.T_LEAF, K.T_LEAF], [(K.U_TOP + K.U_RAIL_BOT) / 2, -K.V_IN - 0.025, wa]), 2, [0, 2 * K.V_IN + 0.05, 0]), "leaf-frame", "skin-a"))
    thermal.append(th(box("th-leaf-a-rail-top", LF, STL, T_LEAF, [0.05 - 2 * K.T_LEAF, 2 * K.V_IN, K.T_LEAF], [(K.U_TOP + K.U_TOP_RAIL) / 2, 0, wa]), "leaf-frame", "skin-a"))
    thermal.append(th(box("th-leaf-a-rail-bottom", LF, STL, T_LEAF, [K.U_GROOVE - K.U_GLASS_BOT - 2 * K.T_LEAF, 2 * K.V_IN, K.T_LEAF], [(K.U_GLASS_BOT + K.U_GROOVE) / 2, 0, wa]), "leaf-frame", "skin-a"))
    for of in ("bead-stiles-ext", "bead-rails-ext", "hanger-clevis", "hanger-pin", "handle-rosette-ext", "handle-legs-ext", "handle-grip-ext"):
        thermal.append(th_of(f"th-{of}", of, "leaf-frame", "skin-b", ["-w"]))

    hollow_jambs = []
    for name, cx, side in JAMBS:
        hollow_jambs.append(st(prism(f"m-jamb-{name}", TR, STL, 0.5, K.jamb_profile(cx, side), "zx", K.FLOOR, K.JAMB_TOP, holes=[K.jamb_profile(cx, side, K.T_TRACK)]), TR))
    structure = [st_of("m-track", "track", TR), st_of("m-track-end-caps", "track-end-caps", TR)] + hollow_jambs
    structure += [st(s, LF) for s in stiles("m-leaf-", K.U_TOP, K.U_RAIL_BOT, True, True)]
    structure += [st(s, LF) for s in rails("m-leaf-", True, True)]
    structure += [
        st_of("m-hanger-rods", "hanger-rod", LF),
        st(repeat(box("j-trolleys", LF, STL, 0.35, [0.052, 2 * K.TROLLEY_HALF, 0.074], [0.02, -HV, 0], role="detail", cornerRadius=0.004), 2, [0, 2 * HV, 0]), LF, "joint"),
        st_of("j-hanger-pins", "hanger-pin", LF, "joint"),
        st_of("j-floor-guide", "floor-guide", TR, "joint"),
        st(rod("support-west-jamb", TR, STL, 0.4, [0, 0, -H], [0, 0, 1], 2 * H, 0.012), TR, "support", "west-jamb"),
        st(repeat(rod("support-east-jamb", TR, STL, 0.4, [3.0 - 0.01, 0, -H], [0, 0, 1], 2 * H, 0.008), 2, [0.02, 0, 0]), TR, "support", "east-jamb"),
        st(ellipsoid("mass-leaf", LF, STL, 0.6, [0.03, 0.03, 0.02], [LEAF_COM_U, 0, 0]), LF, "mass"),
        st(ellipsoid("mass-track", TR, STL, 0.6, [0.03, 0.03, 0.03], [1.5, 0, 0]), TR, "mass"),
    ]
    decisions = [
        text("Body: the door as built at 7.5 mm detail. Two cold-formed rebated steel jamb posts 100 x 100 mm (outer radius 8 mm) with the 60 x 20 mm receiving channel on the opening side and an 8 mm EPDM buffer in it; the 100 x 100 x 4 mm slotted box track with its running chamber, two treads and the 24 mm slot between two upturned lips, closed by two nylon brush strips, with 8 mm welded end plates. The leaf is a welded frame of 50 x 50 mm steel tubes (stiles and top rail) and a 50 x 100 mm bottom rail with the 20 x 25 mm groove that runs over the floor guide, a 10 mm toughened pane held by 15 x 15 mm steel glazing beads on both faces, two brush seals under the bottom rail (10 mm floor clearance), two pendant hangers (trolley with four crowned wheels on stub axles inside the track, M12 hanger rod, adjusting and lock nut, clevis on the top rail with a 16 mm pin) at 350 mm either side of the centre and a back-to-back pull handle through the glass (two rosettes, two 16 mm stand-offs, 300 mm grip, 25 mm finger clearance) at 1.07 m. The hollow sections are closed solids in the body; their walls are in the thermal and structure lists.",
             "Körper: die Tür, wie sie gebaut ist, in 7,5-mm-Auflösung. Zwei kaltgeformte Stahl-Falzpfosten 100 x 100 mm (Außenradius 8 mm) mit dem 60 x 20 mm Aufnahmefalz zur Öffnung und einem 8-mm-EPDM-Puffer darin; die geschlitzte Kastenlaufschiene 100 x 100 x 4 mm mit Laufkammer, zwei Laufflächen und dem 24-mm-Schlitz zwischen zwei aufgekanteten Lippen, durch zwei Nylon-Bürstenleisten geschlossen, mit 8-mm-Stirnplatten. Der Flügel ist ein geschweißter Rahmen aus Stahlrohren 50 x 50 mm (Pfosten und Kopfriegel) und einem Fußriegel 50 x 100 mm mit der 20 x 25 mm Nut, die über die Bodenführung läuft, eine 10-mm-ESG-Scheibe mit 15 x 15 mm Stahl-Glashalteleisten auf beiden Seiten, zwei Bürstendichtungen unter dem Fußriegel (10 mm Bodenluft), zwei Pendelaufhängungen (Laufwagen mit vier balligen Rollen auf Achsstummeln in der Schiene, Hängebolzen M12, Stell- und Kontermutter, Gabelkopf am Kopfriegel mit 16-mm-Bolzen) 350 mm beidseits der Mitte und ein durch das Glas verschraubter Stoßgriff beidseitig (zwei Rosetten, zwei 16-mm-Stützen, Griffstange 300 mm, 25 mm Griffraum) auf 1,07 m. Die Hohlprofile sind im Körper geschlossene Volumen; ihre Wände stehen in den Wärme- und Tragwerkslisten."),
        text("Thermal: every region with its layers as nested solids. Jambs, track and the four leaf members: the outer steel skin (skin-b) is the closed profile with its room-side face marked as the conditioned face (C25 c), the enclosed air (cavity) is the profile shrunk by the real wall (4 mm for jambs and track, 2 mm for the leaf), and the room-side wall (skin-a) is the real 4 or 2 mm plate inside it. The track skin is the closed housing because the brushes close the slot. The pane is the 10 mm glass with its room face marked. The outdoor-side glazing beads, the clevises, pins and the outdoor handle count to the leaf frame; the room-side handle and beads are inside the conditioned zone and are left out.",
             "Wärme: jede Region mit ihren Schichten als verschachtelte Volumen. Pfosten, Schiene und die vier Flügelprofile: die äußere Stahlhaut (skin-b) ist das geschlossene Profil mit seiner raumseitigen Fläche als konditionierte Fläche markiert (C25 c), die eingeschlossene Luft (cavity) ist das um die echte Wand verkleinerte Profil (4 mm bei Pfosten und Schiene, 2 mm beim Flügel), und die raumseitige Wand (skin-a) ist die echte 4- oder 2-mm-Platte darin. Die Schienenhaut ist das geschlossene Gehäuse, weil die Bürsten den Schlitz schließen. Die Scheibe ist das 10-mm-Glas mit markierter Raumseite. Die außenseitigen Glashalteleisten, Gabelköpfe, Bolzen und der Außengriff gehören zum Flügelrahmen; Innengriff und innere Leisten liegen in der konditionierten Zone und sind weggelassen."),
        text("Structure: the real load path. The leaf weight goes from the bottom rail through the stiles (50 x 50 x 2 tubes) to the top rail, into the two clevis pins, up the M12 hanger rods to the two trolleys (the prismatic joint) and into the track, a 100 x 100 x 4 slotted box section (the body solid itself, A 1523 mm2, I 222 cm4) with its end plates, which rests on the jamb heads: a pin on the west jamb, a pair of rollers on the east jamb. The rebated jamb posts are modelled with their real 4 mm wall and carry the reactions to the floor; the floor guide is the out-of-plane joint of the leaf (no load in this elevation model). Masses: the leaf at its centre of mass, the track at mid span.",
             "Tragwerk: der echte Lastpfad. Das Flügelgewicht geht vom Fußriegel über die Pfosten (Rohre 50 x 50 x 2) in den Kopfriegel, in die beiden Gabelbolzen, die Hängebolzen M12 hinauf zu den zwei Laufwagen (das Prismengelenk) und in die Schiene, ein geschlitztes Kastenprofil 100 x 100 x 4 (das Körpervolumen selbst, A 1523 mm², I 222 cm⁴) mit seinen Stirnplatten, das auf den Pfostenköpfen liegt: ein Festlager auf dem Westpfosten, ein Rollenpaar auf dem Ostpfosten. Die Falzpfosten sind mit ihrer echten 4-mm-Wand modelliert und tragen die Auflagerkräfte in den Boden; die Bodenführung ist das Gelenk des Flügels quer zur Ebene (in diesem Ansichtsmodell ohne Last). Massen: der Flügel in seinem Schwerpunkt, die Schiene in Feldmitte."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def handle_d(i):
    """🤚 The pull handle at 15 mm detail: one D profile per face (rosette and stand-off merged into a 30 mm leg, 16 mm grip), the room-side handle as the half-turn copy."""
    profile = [[0.005, 0.905], [0.005, 0.935], [0.030, 0.935], [0.030, 1.125], [0.005, 1.125], [0.005, 1.155], [0.030, 1.155], [0.030, 1.18], [0.046, 1.18], [0.046, 0.88], [0.030, 0.88], [0.030, 0.905]]
    return repeat(prism(i, LF, STL, T_DARK, profile, "zx", K.HANDLE_V - 0.01, K.HANDLE_V + 0.01), 2, axis=[0, 1, 0], angle=PI, origin=[1.03, K.HANDLE_V, 0])


def scale100():
    body = [prism(f"jamb-{name}", TR, STL, T_PORTAL, K.jamb_profile(cx, side, 0.0, 0.0), "zx", K.FLOOR, K.JAMB_TOP) for name, cx, side in JAMBS]
    body.append(box("track", TR, STL, T_PORTAL, [K.TRACK_X1 - K.TRACK_X0, 2 * H, 2 * H], [1.5, 0, 0]))
    body.append(prism("leaf-frame", LF, STL, T_LEAF, [[K.U_TOP, -K.V_OUT], [K.U_BRUSH, -K.V_OUT], [K.U_BRUSH, K.V_OUT], [K.U_TOP, K.V_OUT]], "xy", -K.W_LEAF, K.W_LEAF, holes=[[[K.U_TOP_RAIL, -K.V_IN], [K.U_GLASS_BOT, -K.V_IN], [K.U_GLASS_BOT, K.V_IN], [K.U_TOP_RAIL, K.V_IN]]]))
    body.append(box("leaf-glass", LF, GL, T_GLASS, [K.U_GLASS_BOT - K.U_TOP_RAIL, 2 * K.V_IN, K.GLASS_T], [(K.U_TOP_RAIL + K.U_GLASS_BOT) / 2, 0, 0], role="glazing"))
    body += [box(f"hanger-{n}", LF, STL, 0.3, [0.05, 0.05, 0.05], [0.075, s * HV, 0]) for n, s in (("a", -1), ("b", 1))]
    body.append(handle_d("handle"))
    thermal = [th_of("th-glass", "leaf-glass", "leaf-glass", "glass-pane", ["-w"])]
    thermal += [th_of(f"th-jamb-{name}", f"jamb-{name}", "jamb-steel", "skin-b", ["-w"]) for name, _, _ in JAMBS]
    thermal += [th_of("th-track", "track", "track-steel", "skin-b", ["-w"]), th_of("th-leaf-frame", "leaf-frame", "leaf-frame", "skin-b", ["-w"])]
    thermal += [th_of(f"th-hanger-{n}", f"hanger-{n}", "leaf-frame", "skin-b", ["-w"]) for n in "ab"]
    thermal.append(th_of("th-handle", "handle", "leaf-frame", "skin-b", ["-w"]))
    structure = [st(prism(f"m-jamb-{name}", TR, STL, 0.5, K.jamb_profile(cx, side, 0.0, 0.0), "zx", K.FLOOR, K.JAMB_TOP, holes=[K.jamb_profile(cx, side, K.T_TRACK, 0.0)]), TR) for name, cx, side in JAMBS]
    structure.append(st(prism("m-track", TR, STL, 0.5, K.rounded_rect(-H, -H, H, H, 0.0), "yz", K.TRACK_X0, K.TRACK_X1, holes=[K.rounded_rect(-H + K.T_TRACK, -H + K.T_TRACK, H - K.T_TRACK, H - K.T_TRACK, 0.0)]), TR))
    structure += [st(s, LF) for s in stiles("m-leaf-", K.U_TOP, K.U_BRUSH, False, True)]
    top = prism("m-leaf-rail-top", LF, STL, 0.5, K.rounded_rect(-K.W_LEAF, K.U_TOP, K.W_LEAF, K.U_TOP_RAIL, 0.0), "zx", -K.V_IN, K.V_IN, holes=[K.rounded_rect(-K.W_LEAF + K.T_LEAF, K.U_TOP + K.T_LEAF, K.W_LEAF - K.T_LEAF, K.U_TOP_RAIL - K.T_LEAF, 0.0)])
    bottom = prism("m-leaf-rail-bottom", LF, STL, 0.5, K.rounded_rect(-K.W_LEAF, K.U_GLASS_BOT, K.W_LEAF, K.U_BRUSH, 0.0), "zx", -K.V_IN, K.V_IN, holes=[K.rounded_rect(-K.W_LEAF + K.T_LEAF, K.U_GLASS_BOT + K.T_LEAF, K.W_LEAF - K.T_LEAF, K.U_RAIL_BOT - K.T_LEAF, 0.0)])
    structure += [st(top, LF), st(bottom, LF)]
    structure += [
        st(repeat(box("j-trolleys", LF, STL, 0.35, [0.06, 0.09, 0.07], [0.02, -HV, 0], role="detail"), 2, [0, 2 * HV, 0]), LF, "joint"),
        st_of("j-hanger-a", "hanger-a", LF, "joint"),
        st_of("j-hanger-b", "hanger-b", LF, "joint"),
        st(rod("support-west-jamb", TR, STL, 0.4, [0, 0, -H], [0, 0, 1], 2 * H, 0.016), TR, "support", "west-jamb"),
        st(repeat(rod("support-east-jamb", TR, STL, 0.4, [3.0 - 0.015, 0, -H], [0, 0, 1], 2 * H, 0.0125), 2, [0.03, 0, 0]), TR, "support", "east-jamb"),
        st(ellipsoid("mass-leaf", LF, STL, 0.6, [0.04, 0.04, 0.02], [LEAF_COM_U, 0, 0]), LF, "mass"),
        st(ellipsoid("mass-track", TR, STL, 0.6, [0.04, 0.04, 0.04], [1.5, 0, 0]), TR, "mass"),
    ]
    decisions = [
        text("Body: the door at 15 mm detail. Jamb posts as rebated 100 mm profiles with the 60 x 20 mm receiving channel (corner radii, EPDM buffer merged), the track housing as one 100 x 100 mm bar (slot, brushes, lips and end plates merged, the trolleys are hidden), the leaf frame as one extrusion round the pane (the four tubes, glazing beads, groove and brush seal merged; it reaches 10 mm above the floor), the 10 mm pane, two 50 mm hanger blocks (rod, nuts and clevis merged) and the pull handle as one D profile per face (rosette and stand-off merged into a 30 mm leg).",
             "Körper: die Tür in 15-mm-Auflösung. Pfosten als Falzprofile 100 mm mit dem 60 x 20 mm Aufnahmefalz (Eckradien, EPDM-Puffer zusammengefasst), das Schienengehäuse als ein Stab 100 x 100 mm (Schlitz, Bürsten, Lippen und Stirnplatten zusammengefasst, die Laufwagen sind verdeckt), der Flügelrahmen als eine Extrusion um die Scheibe (die vier Rohre, Glashalteleisten, Nut und Bürstendichtung zusammengefasst; er endet 10 mm über dem Boden), die 10-mm-Scheibe, zwei Aufhängeblöcke 50 mm (Bolzen, Muttern und Gabelkopf zusammengefasst) und der Stoßgriff als ein D-Profil je Seite (Rosette und Stütze zu einem 30-mm-Fuß zusammengefasst)."),
        text("Thermal: one surface solid per region part (the steel skins, the enclosed air and the room-side walls are merged into the outer skin, B2), each with its room-side face marked as the conditioned face; the pane with its room face marked.",
             "Wärme: ein Oberflächenvolumen je Regionsteil (Stahlhäute, eingeschlossene Luft und raumseitige Wände in der äußeren Haut zusammengefasst, B2), jeweils mit der raumseitigen Fläche als konditionierter Fläche; die Scheibe mit markierter Raumseite."),
        text("Structure: jambs and track as hollow sections with their real 4 mm wall (slot and rebate radii merged), the leaf frame as four 2 mm tubes with sharp corners, the two trolleys as joint blocks, the hanger blocks as the hanging joints, a pin on the west jamb and a roller pair on the east jamb at the track axis, masses of leaf and track.",
             "Tragwerk: Pfosten und Schiene als Hohlprofile mit ihrer echten 4-mm-Wand (Schlitz und Falzradien zusammengefasst), der Flügelrahmen als vier scharfkantige 2-mm-Rohre, die beiden Laufwagen als Gelenkblöcke, die Aufhängeblöcke als Hängegelenke, ein Festlager auf dem West- und ein Rollenpaar auf dem Ostpfosten in der Schienenachse, Massen von Flügel und Schiene."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale200():
    body = [box(f"jamb-{name}", TR, STL, T_PORTAL, [2 * H, -K.FLOOR + K.JAMB_TOP, 2 * H], [cx, (K.FLOOR + K.JAMB_TOP) / 2, 0]) for name, cx, _ in JAMBS]
    body.append(box("track", TR, STL, T_PORTAL, [K.TRACK_X1 - K.TRACK_X0, 2 * H, 2 * H], [1.5, 0, 0]))
    body.append(prism("leaf-frame", LF, STL, T_LEAF, [[0.05, -K.V_OUT], [K.U_BRUSH, -K.V_OUT], [K.U_BRUSH, K.V_OUT], [0.05, K.V_OUT]], "xy", -K.W_LEAF, K.W_LEAF, holes=[[[0.125, -0.425], [K.U_GLASS_BOT, -0.425], [K.U_GLASS_BOT, 0.425], [0.125, 0.425]]]))
    body.append(box("leaf-glass", LF, GL, T_GLASS, [K.U_GLASS_BOT - 0.125, 0.85, 0.02], [(0.125 + K.U_GLASS_BOT) / 2, 0, 0], role="glazing"))
    thermal = [th_of("th-glass", "leaf-glass", "leaf-glass", "glass-pane", ["-w"])]
    thermal += [th_of(f"th-jamb-{name}", f"jamb-{name}", "jamb-steel", "skin-b", ["-w"]) for name, _, _ in JAMBS]
    thermal += [th_of("th-track", "track", "track-steel", "skin-b", ["-w"]), th_of("th-leaf-frame", "leaf-frame", "leaf-frame", "skin-b", ["-w"])]
    structure = [st_of(f"m-jamb-{name}", f"jamb-{name}", TR) for name, _, _ in JAMBS]
    structure.append(st_of("m-track", "track", TR))
    structure.append(st(repeat(box("m-leaf-stiles", LF, STL, 0.5, [K.U_BRUSH - 0.05, 0.05, 0.05], [(0.05 + K.U_BRUSH) / 2, -0.475, 0]), 2, [0, 0.95, 0]), LF))
    structure.append(st(box("m-leaf-rail-top", LF, STL, 0.5, [0.075, 0.9, 0.05], [0.0875, 0, 0]), LF))
    structure.append(st(box("m-leaf-rail-bottom", LF, STL, 0.5, [K.U_BRUSH - K.U_GLASS_BOT, 0.9, 0.05], [(K.U_GLASS_BOT + K.U_BRUSH) / 2, 0, 0]), LF))
    structure += [
        st(repeat(box("j-trolleys", LF, STL, 0.35, [0.06, 0.09, 0.06], [0.02, -HV, 0], role="detail"), 2, [0, 2 * HV, 0]), LF, "joint"),
        st(rod("support-west-jamb", TR, STL, 0.4, [0, 0, -H], [0, 0, 1], 2 * H, 0.03), TR, "support", "west-jamb"),
        st(repeat(rod("support-east-jamb", TR, STL, 0.4, [3.0 - 0.02, 0, -H], [0, 0, 1], 2 * H, 0.015), 2, [0.04, 0, 0]), TR, "support", "east-jamb"),
        st(ellipsoid("mass-leaf", LF, STL, 0.6, [0.05, 0.05, 0.02], [LEAF_COM_U, 0, 0]), LF, "mass"),
    ]
    decisions = [
        text("Body: the door at 30 mm detail: jambs and track housing as three 100 mm bars (the 20 mm rebate is below the detail and merged), the leaf as one frame extrusion that reaches up to the track (hangers merged) and down to 10 mm above the floor (brush merged), its window inset 75 mm at the top and sides as the drawing shows it, and the pane drawn 20 mm thick (glazing role).",
             "Körper: die Tür in 30-mm-Auflösung: Pfosten und Schienengehäuse als drei Stäbe 100 mm (der 20-mm-Falz liegt unter der Auflösung und ist zusammengefasst), der Flügel als eine Rahmenextrusion bis an die Schiene (Aufhängung zusammengefasst) und bis 10 mm über den Boden (Bürste zusammengefasst), sein Fenster oben und seitlich 75 mm eingerückt wie in der Zeichnung, und die Scheibe 20 mm dick gezeichnet (Rolle Verglasung)."),
        text("Thermal: one surface solid per region part, referenced from the body, room-side faces marked; layers merged (B2).",
             "Wärme: ein Oberflächenvolumen je Regionsteil, aus dem Körper referenziert, raumseitige Flächen markiert; Schichten zusammengefasst (B2)."),
        text("Structure: jambs and track are the body bars (they are the members, solid at this scale, C30), the leaf frame as two stiles and two rails of its real 50 mm and 100 mm depth, the two trolleys as joint blocks in the track, a pin on the west jamb and a roller pair on the east jamb, the leaf mass at its centre (the track mass is merged into its member).",
             "Tragwerk: Pfosten und Schiene sind die Körperstäbe (sie sind die Stäbe, in diesem Maßstab voll, C30), der Flügelrahmen als zwei Pfosten und zwei Riegel in ihrer echten Tiefe von 50 und 100 mm, die beiden Laufwagen als Gelenkblöcke in der Schiene, ein Festlager auf dem West- und ein Rollenpaar auf dem Ostpfosten, die Flügelmasse in ihrem Schwerpunkt (die Schienenmasse steckt in ihrem Stab)."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale500():
    body = [
        box("jamb-west", TR, STL, T_PORTAL, [0.15, 2.15, 2 * H], [0.025, -1.025, 0]),
        box("jamb-east", TR, STL, T_PORTAL, [0.15, 2.15, 2 * H], [2.975, -1.025, 0]),
        box("track", TR, STL, T_PORTAL, [2.8, 0.15, 2 * H], [1.5, -0.025, 0]),
        box("leaf", LF, GL, 0.7, [K.U_BRUSH - K.U_TOP, 2 * K.V_OUT, 0.075], [(K.U_TOP + K.U_BRUSH) / 2, 0, 0]),
    ]
    inner = [[K.U_TOP + 0.075, -0.425], [K.U_BRUSH - 0.075, -0.425], [K.U_BRUSH - 0.075, 0.425], [K.U_TOP + 0.075, 0.425]]
    thermal = [
        th_of("th-jamb-west", "jamb-west", "jamb-steel", "skin-b", ["-w"]),
        th_of("th-jamb-east", "jamb-east", "jamb-steel", "skin-b", ["-w"]),
        th_of("th-track", "track", "track-steel", "skin-b", ["-w"]),
        th(cut(prism("th-leaf-frame", LF, STL, T_LEAF, [[K.U_TOP, -K.V_OUT], [K.U_BRUSH, -K.V_OUT], [K.U_BRUSH, K.V_OUT], [K.U_TOP, K.V_OUT]], "xy", -0.0375, 0.0375, holes=[inner]), "-w"), "leaf-frame", "skin-b"),
        th(cut(prism("th-glass", LF, GL, T_GLASS, inner, "xy", -0.0375, 0.0375, role="glazing"), "-w"), "leaf-glass", "glass-pane"),
    ]
    structure = [
        st(box("m-jamb-west", TR, STL, 0.5, [2 * H, -K.FLOOR + K.JAMB_TOP, 2 * H], [0, (K.FLOOR + K.JAMB_TOP) / 2, 0]), TR),
        st(box("m-jamb-east", TR, STL, 0.5, [2 * H, -K.FLOOR + K.JAMB_TOP, 2 * H], [3.0, (K.FLOOR + K.JAMB_TOP) / 2, 0]), TR),
        st(box("m-track", TR, STL, 0.5, [K.TRACK_X1 - K.TRACK_X0, 2 * H, 2 * H], [1.5, 0, 0]), TR),
        st(prism("m-leaf-frame", LF, STL, 0.5, [[K.U_TOP, -K.V_OUT], [K.U_BRUSH, -K.V_OUT], [K.U_BRUSH, K.V_OUT], [K.U_TOP, K.V_OUT]], "xy", -0.0375, 0.0375, holes=[inner]), LF),
        st(box("j-carriage", LF, STL, 0.35, [0.075, 0.8, 0.075], [0.0375, 0, 0], role="detail"), LF, "joint"),
        st(rod("support-west-jamb", TR, STL, 0.4, [0, 0, -H], [0, 0, 1], 2 * H, 0.0375), TR, "support", "west-jamb"),
        st(rod("support-east-jamb", TR, STL, 0.4, [3.0, -0.0375, -H], [0, 0, 1], 2 * H, 0.0375), TR, "support", "east-jamb"),
        st(ellipsoid("mass-leaf", LF, STL, 0.6, [0.075, 0.075, 0.0375], [LEAF_COM_U, 0, 0]), LF, "mass"),
    ]
    decisions = [
        text("Body: the door at 75 mm detail as the drawing shows it: the portal (two 150 mm posts and the 150 mm track housing, 100 mm deep; the real 100 mm sections are drawn 150 mm so the U keeps a printable width) and the leaf as one 75 mm deep block from the track down to 10 mm above the floor; pane, frame, beads, hangers, handle, seals and the hidden trolleys are merged into it.",
             "Körper: die Tür in 75-mm-Auflösung, wie sie die Zeichnung zeigt: das Portal (zwei Pfosten 150 mm und das Schienengehäuse 150 mm, 100 mm tief; die echten 100-mm-Profile sind 150 mm gezeichnet, damit das U druckbar breit bleibt) und der Flügel als ein 75 mm tiefer Block von der Schiene bis 10 mm über den Boden; Scheibe, Rahmen, Leisten, Aufhängung, Griff, Dichtungen und die verdeckten Laufwagen sind darin aufgegangen."),
        text("Minimal: the thermal list of 1:500 has one solid per region part and cannot drop further (two separate jambs, one track, the leaf split into its frame ring and its pane); it has as many solids and triangles as 1:200 because the 1:200 leaf frame is already a single ring.",
             "Minimal: die Wärmeliste von 1:500 hat ein Volumen je Regionsteil und kann nicht weiter sinken (zwei getrennte Pfosten, eine Schiene, der Flügel in Rahmenring und Scheibe geteilt); sie hat so viele Volumen und Dreiecke wie 1:200, weil der Flügelrahmen von 1:200 schon ein einziger Ring ist."),
        text("Thermal: the portal parts referenced from the body, the leaf block split into its two regions: a 75 mm frame ring (leaf-frame) and the pane filling the ring (leaf-glass), together exactly the body leaf; room-side faces marked, layers merged (B2).",
             "Wärme: die Portalteile aus dem Körper referenziert, der Flügelblock in seine zwei Regionen geteilt: ein 75-mm-Rahmenring (leaf-frame) und die Scheibe, die den Ring füllt (leaf-glass), zusammen genau der Körperflügel; raumseitige Flächen markiert, Schichten zusammengefasst (B2)."),
        text("Structure: the real 100 mm jambs and track as solid bars on their axes inside the drawn 150 mm portal, the leaf frame as a 75 mm ring (the four frame members merged), the carriage as one joint bar at the head of the leaf spanning both trolleys, a pin on the west jamb and a roller on the east jamb at the track axis, the leaf mass at its centre.",
             "Tragwerk: die echten 100-mm-Pfosten und die Schiene als volle Stäbe in ihren Achsen innerhalb des 150 mm gezeichneten Portals, der Flügelrahmen als 75-mm-Ring (die vier Rahmenstäbe zusammengefasst), der Laufwagen als ein Gelenkbalken am Flügelkopf über beide Laufwagen, ein Festlager auf dem West- und ein Rollenlager auf dem Ostpfosten in der Schienenachse, die Flügelmasse in ihrem Schwerpunkt."),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def leaf_mass():
    """⚖️ Mass and centre of mass (u) of the leaf with everything it carries: tubes 50 x 50 x 2 and 50 x 100 x 2, the 10 mm pane, beads (15 x 15 x 2 angles), handle pair, hangers and trolleys, brushes."""
    rho, glass = 7850, 2500
    tube50 = (4 * 0.05 - 4 * K.T_LEAF) * K.T_LEAF
    rail100 = (2 * 0.05 + 2 * 0.1 + 2 * 0.025 - 6 * K.T_LEAF) * K.T_LEAF
    angle15 = (2 * K.BEAD - K.T_LEAF) * K.T_LEAF
    parts = [
        ("stiles", 2 * tube50 * (K.U_RAIL_BOT - K.U_TOP) * rho, (K.U_TOP + K.U_RAIL_BOT) / 2),
        ("top rail", tube50 * 2 * K.V_IN * rho, (K.U_TOP + K.U_TOP_RAIL) / 2),
        ("bottom rail", rail100 * 2 * K.V_IN * rho, (K.U_GLASS_BOT + K.U_RAIL_BOT) / 2),
        ("pane", (K.U_GLASS_BOT - K.U_TOP_RAIL) * 2 * K.V_IN * K.GLASS_T * glass, (K.U_TOP_RAIL + K.U_GLASS_BOT) / 2),
        ("beads", 2 * angle15 * 2 * ((K.U_GLASS_BOT - K.U_TOP_RAIL) + 2 * K.V_IN) * rho, (K.U_TOP_RAIL + K.U_GLASS_BOT) / 2),
        ("handles", 2 * 0.65, 1.03),
        ("hangers and trolleys", 2 * 0.9, 0.05),
        ("brushes", 0.3, (K.U_RAIL_BOT + K.U_BRUSH) / 2),
    ]
    kg = sum(m for _, m, _ in parts)
    return kg, sum(m * u for _, m, u in parts) / kg, parts


LEAF_KG, LEAF_COM_U, LEAF_PARTS = leaf_mass()

if __name__ == "__main__":
    volumes = {"50": scale50(), "100": scale100(), "200": scale200(), "500": scale500()}
    path = os.path.join(T, "volumes", "components", "sliding-door.json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, ensure_ascii=False, indent=2)
        f.write("\n")
    print("written", path, {s: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for s, v in volumes.items()})
    print("leaf kg %.2f com u %.4f" % (LEAF_KG, LEAF_COM_U))
    for name, m, u in LEAF_PARTS:
        print("  %-22s %6.2f kg at u %.3f" % (name, m, u))
