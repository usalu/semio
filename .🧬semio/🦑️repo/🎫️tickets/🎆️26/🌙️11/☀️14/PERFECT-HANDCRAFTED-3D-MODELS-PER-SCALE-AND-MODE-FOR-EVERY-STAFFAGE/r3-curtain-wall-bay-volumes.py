"""🏢 Round 3 curtain-wall bay: the handcrafted solids of every scale (body, thermal, structure), stated part by part, written to the volumes source `volumes/components/curtain-wall-bay.json`; also the masses the roots script writes into the rig.

Usage: python3 r3-curtain-wall-bay-volumes.py
"""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-curtain-wall-bay-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
box, pane, along_x, along_y, rod, ellipsoid, repeat, cut, th, th_of, st, st_of, text, r5 = K.box, K.pane, K.along_x, K.along_y, K.rod, K.ellipsoid, K.repeat, K.cut, K.th, K.th_of, K.st, K.st_of, K.text, K.r5

FR, IG, SP, CR = "frame", "igu", "spandrel", "cradle"
AL, GL, SG, EP, PA, ES, WO, ST = "aluminium", "glass", "spandrel-glass", "epdm", "polyamide", "edge-seal", "wool", "steel"
T_CAP, T_TUBE, T_SEAL, T_ISO, T_GLASS, T_SPAN, T_WOOL, T_PAN = 0.42, 0.5, 0.12, 0.8, 0.9, 0.24, 0.88, 0.58
MX0, MX1 = K.MULLION_X
DX = MX1 - MX0
IX0, IY0 = K.IGU_O
SX0, SY0 = K.SP_O
IGU_W, IGU_H = 1.3, 2.35
SP_W, SP_H = 1.3, 0.8
CENTRE = (K.EDGE_ZONE, K.EDGE_ZONE, IGU_W - K.EDGE_ZONE, IGU_H - K.EDGE_ZONE)
VISION_OUT = K.local(K.IGU_O, K.GLASS_X[0], K.VISION_Y[0], K.GLASS_X[1], K.VISION_Y[1])
SPANDREL_OUT = K.local(K.SP_O, K.GLASS_X[0], K.SPANDREL_Y[0], K.GLASS_X[1], K.SPANDREL_Y[1])
ANCHOR_Z = 0.06
HEAD_Y = 3.275
SCREW_STEP = 0.25
CARRIER_X = (0.15, 1.15)
SAME, ALL_EDGES = ["-v", "+v"], ["-u", "+u", "-v", "+v"]


def members50():
    """🏗️ Mullions and transoms at 1:50: tube, isolator, pressure plate, cover cap and the four glazing gaskets of each member, every part with its real profile."""
    out = []
    mo, mc = K.tube(K.Z_MULLION_BACK)
    out.append(repeat(along_y("mullion-tube", FR, AL, T_TUBE, mo, MX0, 0, K.H, holes=[mc]), 2, [DX, 0, 0]))
    out.append(repeat(along_y("mullion-isolator", FR, PA, T_ISO, K.isolator(), MX0, 0, K.H), 2, [DX, 0, 0]))
    out.append(repeat(along_y("mullion-plate", FR, AL, T_CAP, K.plate(), MX0, 0, K.H), 2, [DX, 0, 0]))
    out.append(repeat(along_y("mullion-cap", FR, AL, T_CAP, K.cap(), MX0, 0, K.H), 2, [DX, 0, 0]))
    for name, sign in (("a", 1), ("b", -1)):
        out.append(repeat(along_y(f"mullion-gasket-out-{name}", FR, EP, T_SEAL, K.gasket_out(sign), MX0, 0, K.H), 2, [DX, 0, 0]))
        out.append(repeat(along_y(f"mullion-gasket-in-{name}", FR, EP, T_SEAL, K.gasket_in(sign), MX0, 0, K.H), 2, [DX, 0, 0]))
    to, tc = K.tube(K.Z_TRANSOM_BACK)
    x0, x1 = K.SIGHT_X
    for name, yc in K.TRANSOM_Y:
        out.append(along_x(f"transom-{name}-tube", FR, AL, T_TUBE, to, yc, x0, x1, holes=[tc]))
        out.append(along_x(f"transom-{name}-isolator", FR, PA, T_ISO, K.isolator(), yc, x0, x1))
        out.append(along_x(f"transom-{name}-plate", FR, AL, T_CAP, K.plate(), yc, x0, x1))
        out.append(along_x(f"transom-{name}-cap", FR, AL, T_CAP, K.cap(), yc, x0, x1))
        for side, sign in (("a", 1), ("b", -1)):
            out.append(along_x(f"transom-{name}-gasket-out-{side}", FR, EP, T_SEAL, K.gasket_out(sign), yc, x0, x1))
            out.append(along_x(f"transom-{name}-gasket-in-{side}", FR, EP, T_SEAL, K.gasket_in(sign), yc, x0, x1))
    return out


def stubs50():
    """✂️ The edges of the neighbouring infills that the outer rebates of this bay's frame clamp, cut at the boundary of the sample: vision units left and right, enamelled spandrels left and right, the vision unit of the storey below and the spandrel of the storey above."""
    z0, z1 = K.Z_GLASS_OUT, K.Z_GLASS_IN
    x0, x1 = K.GLASS_X
    return [
        cut(repeat(box("stub-vision-side", FR, GL, T_GLASS, [0, K.VISION_Y[0], z0], [K.BITE, K.VISION_Y[1], z1], role="glazing"), 2, [K.W - K.BITE, 0, 0]), "-u", "+u"),
        cut(repeat(box("stub-spandrel-side", FR, SG, T_SPAN, [0, K.SPANDREL_Y[0], z0], [K.BITE, K.SPANDREL_Y[1], z1], role="glazing"), 2, [K.W - K.BITE, 0, 0]), "-u", "+u"),
        cut(box("stub-vision-below", FR, GL, T_GLASS, [x0, 0, z0], [x1, K.BITE, z1], role="glazing"), "-v"),
        cut(box("stub-spandrel-above", FR, SG, T_SPAN, [x0, K.H - K.BITE, z0], [x1, K.H, z1], role="glazing"), "+v"),
    ]


def igu50():
    """🪟 The 6-16-6 unit, 1.33 x 2.38 m with a 15 mm bite all round: outer pane, inner low-e pane, the 12 mm warm-edge spacer with butyl and secondary seal round the edge of the cavity."""
    out = VISION_OUT
    seal = (out[0] + K.SEAL_W, out[1] + K.SEAL_W, out[2] - K.SEAL_W, out[3] - K.SEAL_W)
    return [
        pane("igu-pane-out", IG, GL, T_GLASS, out, K.Z_GLASS_OUT, K.Z_PANE_OUT),
        pane("igu-pane-in", IG, GL, T_GLASS, out, K.Z_PANE_IN, K.Z_GLASS_IN),
        pane("igu-edge-seal", IG, ES, 0.15, out, K.Z_PANE_OUT, K.Z_PANE_IN, hole=seal, role="detail"),
    ]


def spandrel50():
    """🟫 The insulated spandrel: 6 mm enamelled glass in the outer glazing plane, the 22 mm packer that fills the rebate behind its edge, the folded 1.5 mm steel back pan (flange ring and back sheet) and the 100 mm mineral wool in it, 20 mm behind the glass."""
    sight = (0.0, 0.0, SP_W, SP_H)
    inner = (K.PAN_T, K.PAN_T, SP_W - K.PAN_T, SP_H - K.PAN_T)
    return [
        pane("spandrel-enamel", SP, SG, T_SPAN, SPANDREL_OUT, K.Z_GLASS_OUT, K.Z_ENAMEL_IN),
        pane("spandrel-packer", SP, EP, T_SEAL, SPANDREL_OUT, K.Z_ENAMEL_IN, K.Z_GLASS_IN, hole=sight, role="body"),
        pane("spandrel-pan-flanges", SP, ST, T_PAN, sight, K.Z_WOOL, K.Z_PAN, hole=inner, role="body"),
        dict(pane("spandrel-pan-back", SP, ST, T_PAN, sight, K.Z_PAN, K.Z_PAN + K.PAN_T, role="body"), texture=True),
        pane("spandrel-wool", SP, WO, T_WOOL, inner, K.Z_WOOL, K.Z_PAN, role="body"),
    ]


def thermal50():
    """🌡️ 1:50 thermal: the glass zones of EN ISO 10077-1 as nested pane / cavity / pane shells (the 65 mm edge ring round the centre of the visible glass), the spandrel as enamel / air / wool / pan shells, and each frame member as the EN ISO 12631 frame zone: a break shell over its 50 mm projection (isolator, gaskets, glass edges, spacer) under the cap shell (cap and pressure plate)."""
    out = []
    ring = (0.0, 0.0, IGU_W, IGU_H)
    for region, outer, hole in (("glass-centre", CENTRE, None), ("glass-edge", ring, CENTRE)):
        mid = "cavity" if region == "glass-centre" else "spacer-cavity"
        out.append(cut(th(pane(f"th-{region}-pane-out", IG, GL, T_GLASS, outer, K.Z_GLASS_OUT, K.Z_PANE_OUT, hole=hole), region, "pane-out"), *ALL_EDGES, "+w"))
        out.append(cut(th(pane(f"th-{region}-{mid}", IG, GL, 0.97, outer, K.Z_PANE_OUT, K.Z_PANE_IN, hole=hole), region, mid), *ALL_EDGES))
        out.append(cut(th(pane(f"th-{region}-pane-in", IG, GL, T_GLASS, outer, K.Z_PANE_IN, K.Z_GLASS_IN, hole=hole), region, "pane-in"), *ALL_EDGES, "+w"))
    sight = (0.0, 0.0, SP_W, SP_H)
    out += [
        cut(th(pane("th-spandrel-enamel", SP, SG, T_SPAN, sight, K.Z_GLASS_OUT, K.Z_ENAMEL_IN), "spandrel", "enamel"), *ALL_EDGES, "+w"),
        cut(th(pane("th-spandrel-air", SP, WO, 0.97, sight, K.Z_ENAMEL_IN, K.Z_WOOL, role="body"), "spandrel", "air"), *ALL_EDGES),
        th_of("th-spandrel-wool", "spandrel-wool", "spandrel", "wool"),
        th_of("th-spandrel-pan-flanges", "spandrel-pan-flanges", "spandrel", "steel-pan"),
        th_of("th-spandrel-pan-back", "spandrel-pan-back", "spandrel", "steel-pan", ["+w"]),
    ]
    out += [
        cut(th(repeat(box("th-mullion-break", FR, PA, T_ISO, [0, 0, K.Z_PLATE], [K.FACE, K.H, K.Z_LIP]), 2, [DX, 0, 0]), "mullion", "break"), "-u", "+u", "-v", "+v", "+w"),
        cut(th(repeat(along_y("th-mullion-plate", FR, AL, T_CAP, K.plate(False), MX0, 0, K.H), 2, [DX, 0, 0]), "mullion", "cap"), *SAME, "+w"),
        th_of("th-mullion-cap", "mullion-cap", "mullion", "cap", SAME + ["+w"]),
    ]
    x0, x1 = K.SIGHT_X
    for name, yc in K.TRANSOM_Y:
        out += [
            cut(th(box(f"th-transom-{name}-break", FR, PA, T_ISO, [x0, yc - K.A_CAP, K.Z_PLATE], [x1, yc + K.A_CAP, K.Z_LIP]), "transom", "break"), "-u", "+u", "-v", "+v", "+w"),
            cut(th(along_x(f"th-transom-{name}-plate", FR, AL, T_CAP, K.plate(False), yc, x0, x1), "transom", "cap"), "+w"),
            th_of(f"th-transom-{name}-cap", f"transom-{name}-cap", "transom", "cap", ["+w"]),
        ]
    return out


def supports(scale, pin_r, spigot, width=K.FACE):
    """📌 The four supports at their anchors: the dead-load bracket bolt through the top of each mullion (pin) and the spigot inside the foot of each mullion that guides it laterally (roller across the wall)."""
    out = []
    for side, x in (("left", MX0), ("right", MX1)):
        out.append(st(rod(f"support-head-{side}", FR, ST, 0.3, [x - width / 2, HEAD_Y, ANCHOR_Z], [1, 0, 0], width, pin_r), FR, "support", f"head-anchor-{side}"))
        a, z0, z1, h = spigot
        out.append(st(box(f"support-base-{side}", FR, AL, 0.3, [x - a, 0, z0], [x + a, h, z1]), FR, "support", f"base-guide-{side}"))
    return out


def masses(r):
    """⚖️ Masses at the centres of mass of the frame, the glass unit and the spandrel."""
    return [
        st(ellipsoid("mass-frame", FR, ST, 0.2, [r, r, r], [0.7, FRAME_COM_Y, 0.02]), FR, "mass"),
        st(ellipsoid("mass-igu", IG, ST, 0.2, [r, r, 0.014 if r < 0.03 else r], [IGU_W / 2, IGU_H / 2, -0.07]), IG, "mass"),
        st(ellipsoid("mass-spandrel", SP, ST, 0.2, [r, r, r], [SP_W / 2, SP_H / 2, SPANDREL_COM_Z]), SP, "mass"),
    ]


def structure50():
    """🏗️ 1:50 structure: the load path of a stick system. Glass and spandrel are plates clamped along their edges; their weight sits on setting blocks on glass carriers of the transom below, their wind load goes through the gaskets into the pressure plates and by the plate screws (every 250 mm) into the tubes; transoms hang on the mullions by their end connectors; the mullions hang from the slab brackets and are guided by the spigot at the foot."""
    out = [
        st_of("member-mullion", "mullion-tube", FR),
        *[st_of(f"member-transom-{n}", f"transom-{n}-tube", FR) for n, _ in K.TRANSOM_Y],
        st_of("member-mullion-plate", "mullion-plate", FR),
        *[st_of(f"member-transom-{n}-plate", f"transom-{n}-plate", FR) for n, _ in K.TRANSOM_Y],
        st_of("member-igu-pane-out", "igu-pane-out", IG),
        st_of("member-igu-pane-in", "igu-pane-in", IG),
        st_of("member-spandrel-enamel", "spandrel-enamel", SP),
        st_of("member-spandrel-pan", "spandrel-pan-flanges", SP),
    ]
    n = int(round((K.H - 0.3) / SCREW_STEP)) + 1
    out.append(st(repeat(rod("joint-screws-mullion-left", FR, ST, 0.15, [MX0, 0.15, K.Z_CAP_IN], [0, 0, 1], K.Z_LIP - K.Z_CAP_IN + 0.004, 0.004), n, [0, SCREW_STEP, 0]), FR, "joint"))
    out.append(st(repeat(rod("joint-screws-mullion-right", FR, ST, 0.15, [MX1, 0.15, K.Z_CAP_IN], [0, 0, 1], K.Z_LIP - K.Z_CAP_IN + 0.004, 0.004), n, [0, SCREW_STEP, 0]), FR, "joint"))
    for name, yc in K.TRANSOM_Y:
        out.append(st(repeat(rod(f"joint-screws-transom-{name}", FR, ST, 0.15, [0.2, yc, K.Z_CAP_IN], [0, 0, 1], K.Z_LIP - K.Z_CAP_IN + 0.004, 0.004), 5, [SCREW_STEP, 0, 0]), FR, "joint"))
        out.append(st(repeat(box(f"joint-connector-{name}", FR, AL, 0.35, [K.SIGHT_X[0], yc - 0.019, K.Z_FACE + K.WALL + 0.004], [K.SIGHT_X[0] + 0.04, yc + 0.019, K.Z_TRANSOM_BACK - K.WALL - 0.004]), 2, [K.SIGHT_X[1] - K.SIGHT_X[0] - 0.04, 0, 0]), FR, "joint"))
    out.append(st(repeat(box("joint-igu-setting-blocks", IG, EP, 0.1, [CARRIER_X[0] - IX0, K.VISION_Y[0] - 0.0075 - IY0, K.Z_GLASS_OUT], [CARRIER_X[0] + 0.1 - IX0, K.VISION_Y[0] - IY0, K.Z_GLASS_IN]), 2, [CARRIER_X[1] - CARRIER_X[0], 0, 0]), IG, "joint"))
    out.append(st(repeat(box("joint-spandrel-setting-blocks", SP, EP, 0.1, [CARRIER_X[0] - SX0, K.SPANDREL_Y[0] - 0.0075 - SY0, K.Z_GLASS_OUT], [CARRIER_X[0] + 0.1 - SX0, K.SPANDREL_Y[0] - SY0, K.Z_GLASS_IN]), 2, [CARRIER_X[1] - CARRIER_X[0], 0, 0]), SP, "joint"))
    out.append(st(box("joint-cradle-hook", CR, ST, 0.3, [-0.02, -0.02, K.Z_CAP], [0.02, 0.02, K.Z_PLATE]), CR, "joint"))
    out += supports(50, 0.008, (0.019, -0.04, 0.094, 0.2))
    out += masses(0.025)
    return out


def scale50():
    body = members50() + stubs50() + igu50() + spandrel50()
    decisions = [
        text("Body: the bay as built in a stick system with 50 mm face width. Mullions are 50 x 150 mm tubes (3 mm walls, gasket grooves and screw channel on the glazing face), transoms 50 x 100 mm tubes between them; on every member a 14 x 30 mm polyamide isolator in the 20 mm glass joint, the 46 x 10 mm pressure plate and the 50 x 12 mm snap-on cover cap, with EPDM gaskets outside and inside the glass. The 6-16-6 unit (1.33 x 2.38 m) and the 6 mm enamelled spandrel glass (1.33 x 0.83 m) sit 15 mm deep in the rebates; behind the spandrel glass a 22 mm packer fills the rebate, and the folded 1.5 mm steel back pan holds 100 mm wool 20 mm behind the glass. The neighbouring infills are cut at the edge of the sample inside the outer rebates. Setting blocks (5 mm), screws and the transom connectors are below the 7.5 mm feature and only in structure.",
             "Körper: das Feld, wie es als Pfosten-Riegel-Konstruktion mit 50 mm Ansichtsbreite gebaut wird. Pfosten sind Rohre 50 x 150 mm (3-mm-Wände, Dichtungsnuten und Schraubkanal auf der Glasseite), Riegel Rohre 50 x 100 mm dazwischen; auf jedem Stab ein Polyamid-Isolator 14 x 30 mm in der 20-mm-Glasfuge, die Pressleiste 46 x 10 mm und die aufgeklipste Deckschale 50 x 12 mm, mit EPDM-Dichtungen außen und innen am Glas. Die Isolierverglasung 6-16-6 (1,33 x 2,38 m) und das 6-mm-Emailglas der Brüstung (1,33 x 0,83 m) stecken 15 mm tief im Falz; hinter dem Brüstungsglas füllt ein 22-mm-Ausgleichsprofil den Falz, und die gekantete 1,5-mm-Stahlrückwand trägt 100 mm Mineralwolle 20 mm hinter dem Glas. Die Nachbarfüllungen sind am Rand des Ausschnitts in den äußeren Fälzen geschnitten. Klotzungen (5 mm), Schrauben und Riegelverbinder liegen unter dem Merkmal von 7,5 mm und stehen nur im Tragwerk."),
        text("Thermal: the visible glass split into the EN ISO 10077-1 zones, the 1.17 x 2.22 m centre and the 65 mm edge ring, each as nested outer pane, cavity and inner pane; the spandrel as enamel, air, wool and steel-pan shells; each mullion and transom as its EN ISO 12631 frame zone, a 38 mm break shell over the 50 mm projection (isolator, gaskets, glass edges and spacer under the cap) and the cap shell (cover cap with the pressure plate). The room-side tubes are on the conditioned side of the break and are not a layer. Zone boundaries inside the continuous glass, the member ends where the wall continues and the faces towards the interior are section faces.",
             "Thermik: das sichtbare Glas in die Zonen nach EN ISO 10077-1 geteilt, die Mitte 1,17 x 2,22 m und der 65-mm-Randring, jede als verschachtelte Außenscheibe, Zwischenraum und Innenscheibe; die Brüstung als Schalen Email, Luft, Wolle und Stahlrückwand; jeder Pfosten und Riegel als seine Rahmenzone nach EN ISO 12631, eine 38-mm-Schale der thermischen Trennung über die 50-mm-Projektion (Isolator, Dichtungen, Glasränder und Abstandhalter unter der Kappe) und die Kappenschale (Deckschale mit Pressleiste). Die raumseitigen Rohre liegen auf der konditionierten Seite der Trennung und sind keine Schicht. Zonengrenzen im durchgehenden Glas, die Stabenden, wo die Wand weiterläuft, und die Flächen zum Innenraum sind Schnittflächen."),
        text("Structure: mullion and transom tubes as the members, the pressure plates that take the wind from the gaskets, the plate screws every 250 mm and the transom end connectors as joints, the glass panes and the spandrel glass and pan as the clamped plates, two setting blocks under each infill on the transom below, the hook block of the cradle on the mid transom cap, at each mullion the slab bracket bolt (pin) at the head and the spigot (lateral roller) at the foot, and the masses of frame, glass unit and spandrel.",
             "Tragwerk: Pfosten- und Riegelrohre als Stäbe, die Pressleisten, die den Wind von den Dichtungen aufnehmen, die Pressleistenschrauben alle 250 mm und die Riegelverbinder als Gelenke, die Glasscheiben sowie Brüstungsglas und -rückwand als eingespannte Platten, je zwei Klotzungen unter jeder Füllung auf dem Riegel darunter, der Hakenblock der Gondel auf der Kappe des mittleren Riegels, an jedem Pfosten der Bolzen der Deckenkonsole (Festlager) am Kopf und der Stoßdorn (seitliches Rollenlager) am Fuß sowie die Massen von Rahmen, Glaseinheit und Brüstung."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal50(), "structure": structure50(), "decisions": decisions}


def scale100():
    """🧱 1:100 (feature 15 mm): each member one extruded bar of its section outline (cap, plate, gaskets, isolator and the glass edges merged into the rebate zone), the glass unit one 28 mm slab to the sight lines, the spandrel glass zone and the back pan with its wool as two blocks."""
    mo = K.section_outline(K.Z_MULLION_BACK)
    to = K.section_outline(K.Z_TRANSOM_BACK)
    _, mc = K.tube(K.Z_MULLION_BACK)
    _, tc = K.tube(K.Z_TRANSOM_BACK)
    x0, x1 = K.SIGHT_X
    body = [repeat(along_y("mullion", FR, AL, T_CAP, mo, MX0, 0, K.H), 2, [DX, 0, 0])]
    body += [along_x(f"transom-{n}", FR, AL, T_CAP, to, yc, x0, x1) for n, yc in K.TRANSOM_Y]
    body += [
        pane("igu", IG, GL, T_GLASS, (0, 0, IGU_W, IGU_H), K.Z_GLASS_OUT, K.Z_GLASS_IN),
        pane("spandrel-glass", SP, SG, T_SPAN, (0, 0, SP_W, SP_H), K.Z_GLASS_OUT, K.Z_GLASS_IN, role="body"),
        pane("spandrel-pan", SP, ST, T_PAN, (0, 0, SP_W, SP_H), K.Z_GLASS_IN, K.Z_PAN + K.PAN_T, role="body"),
    ]
    zc = K.Z_CAP + 0.015
    thermal = [
        cut(th(pane("th-glass-centre", IG, GL, T_GLASS, CENTRE, K.Z_GLASS_OUT, K.Z_GLASS_IN), "glass-centre"), *ALL_EDGES, "+w"),
        cut(th(pane("th-glass-edge", IG, GL, T_GLASS, (0, 0, IGU_W, IGU_H), K.Z_GLASS_OUT, K.Z_GLASS_IN, hole=CENTRE), "glass-edge"), *ALL_EDGES, "+w"),
        th_of("th-spandrel", "spandrel-glass", "spandrel", faces=ALL_EDGES + ["+w"]),
        cut(th(repeat(along_y("th-mullion", FR, AL, T_CAP, K.cap_block(zc), MX0, 0, K.H), 2, [DX, 0, 0]), "mullion"), *SAME, "+w"),
        *[cut(th(along_x(f"th-transom-{n}", FR, AL, T_CAP, K.cap_block(zc), yc, x0, x1), "transom"), "+w") for n, yc in K.TRANSOM_Y],
    ]
    mt, _ = K.tube(K.Z_MULLION_BACK)
    tt, _ = K.tube(K.Z_TRANSOM_BACK)
    plain = lambda back: [[-K.A_CAP, K.Z_LIP], [K.A_CAP, K.Z_LIP], [K.A_CAP, back], [-K.A_CAP, back]]
    structure = [
        st(repeat(along_y("member-mullion", FR, AL, T_TUBE, plain(K.Z_MULLION_BACK), MX0, 0, K.H, holes=[mc]), 2, [DX, 0, 0]), FR),
        *[st(along_x(f"member-transom-{n}", FR, AL, T_TUBE, plain(K.Z_TRANSOM_BACK), yc, x0, x1, holes=[tc]), FR) for n, yc in K.TRANSOM_Y],
        st_of("member-igu", "igu", IG),
        st_of("member-spandrel", "spandrel-glass", SP),
        st(repeat(box("joint-igu-setting-blocks", IG, EP, 0.1, [CARRIER_X[0] - IX0, -0.015, K.Z_GLASS_OUT], [CARRIER_X[0] + 0.1 - IX0, 0, K.Z_GLASS_IN]), 2, [1.0, 0, 0]), IG, "joint"),
        st(repeat(box("joint-spandrel-setting-blocks", SP, EP, 0.1, [CARRIER_X[0] - SX0, -0.015, K.Z_GLASS_OUT], [CARRIER_X[0] + 0.1 - SX0, 0, K.Z_GLASS_IN]), 2, [1.0, 0, 0]), SP, "joint"),
        *[st(repeat(box(f"joint-connector-{n}", FR, AL, 0.35, [x0, yc - 0.019, -0.04], [x0 + 0.04, yc + 0.019, 0.04]), 2, [x1 - x0 - 0.04, 0, 0]), FR, "joint") for n, yc in K.TRANSOM_Y],
        st(box("joint-cradle-hook", CR, ST, 0.3, [-0.02, -0.02, K.Z_CAP], [0.02, 0.02, K.Z_CAP + 0.015]), CR, "joint"),
        *supports(100, 0.0125, (0.019, -0.04, 0.094, 0.2)),
        *masses(0.03),
    ]
    decisions = [
        text("Body: every mullion and transom is one extruded bar of its real section outline (50 x 200 mm and 50 x 150 mm with the rounded cap corners), filled as the drawing fills it: the 3 mm tube walls and the 46 x 140 mm chamber are below what 1:100 shows from outside; cap, pressure plate, gaskets, isolator and the 15 mm glass edges are merged into the rebate zone of the bar. The glass unit is one 28 mm slab between the sight lines, the spandrel one 28 mm glass zone (enamel and packer merged) and one back-pan block with its wool (the 20 mm air layer merged).",
             "Körper: jeder Pfosten und Riegel ist ein extrudierter Stab seines wahren Querschnittsumrisses (50 x 200 mm und 50 x 150 mm mit den gerundeten Kappenecken), gefüllt, wie die Zeichnung ihn füllt: die 3-mm-Rohrwände und die Kammer 46 x 140 mm liegen unter dem, was 1:100 von außen zeigt; Deckschale, Pressleiste, Dichtungen, Isolator und die 15-mm-Glasränder sind in der Falzzone des Stabs zusammengefasst. Die Isolierverglasung ist eine 28-mm-Scheibe zwischen den Sichtkanten, die Brüstung eine 28-mm-Glaszone (Email und Ausgleichsprofil zusammengefasst) und ein Rückwandblock mit seiner Wolle (die 20-mm-Luftschicht zusammengefasst)."),
        text("Thermal: one solid per region without layers: the glass centre and the 65 mm edge ring as 28 mm pieces of the unit, the spandrel glass zone, and each frame member as a 15 mm cap zone over its 50 mm face (the cap at the smallest feature); zone boundaries inside the glass, the mullion ends and the faces towards the interior are section faces. Structure: the mullion and transom tubes (3 mm walls) from the glazing face to the room side, the unit and the spandrel glass as plates, setting blocks, transom connectors, the cradle hook, bracket pins and spigots, the three masses.",
             "Thermik: ein Körper je Region ohne Schichten: Glasmitte und 65-mm-Randring als 28-mm-Stücke der Einheit, die Glaszone der Brüstung und jeder Rahmenstab als 15-mm-Kappenzone über seiner 50-mm-Ansicht (die Kappe auf das kleinste Merkmal gebracht); Zonengrenzen im Glas, die Pfostenenden und die Flächen zum Innenraum sind Schnittflächen. Tragwerk: die Pfosten- und Riegelrohre (3-mm-Wände) von der Glasseite bis zur Raumseite, Einheit und Brüstungsglas als Platten, Klotzungen, Riegelverbinder, der Gondelhaken, Konsolbolzen und Stoßdorne, die drei Massen."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


GLASS200 = (-0.086, -0.054)
SPAN200 = (-0.086, 0.045)


def scale200():
    """🧱 1:200 (feature 30 mm): the frame as one extruded grid of 150 mm depth with the room-side backs of the two mullions, the glass unit as a 32 mm slab and the spandrel as one block."""
    body = [
        K.prism("frame", FR, AL, T_CAP, K.rect(0, 0, K.W, K.H), "xy", K.Z_CAP, K.Z_TRANSOM_BACK, holes=[K.rect(0.05, 0.05, 1.35, 0.85), K.rect(0.05, 0.9, 1.35, 3.25)]),
        repeat(box("mullion-back", FR, AL, T_CAP, [0, 0, K.Z_TRANSOM_BACK], [K.FACE, K.H, K.Z_MULLION_BACK]), 2, [DX, 0, 0]),
        pane("igu", IG, GL, T_GLASS, (0, 0, IGU_W, IGU_H), *GLASS200),
        pane("spandrel", SP, SG, T_SPAN, (0, 0, SP_W, SP_H), *SPAN200, role="body"),
    ]
    zc = K.Z_CAP + 0.03
    x0, x1 = K.SIGHT_X
    thermal = [
        cut(th(pane("th-glass-centre", IG, GL, T_GLASS, CENTRE, *GLASS200), "glass-centre"), *ALL_EDGES, "+w"),
        cut(th(pane("th-glass-edge", IG, GL, T_GLASS, (0, 0, IGU_W, IGU_H), *GLASS200, hole=CENTRE), "glass-edge"), *ALL_EDGES, "+w"),
        th_of("th-spandrel", "spandrel", "spandrel", faces=ALL_EDGES + ["+w"]),
        cut(th(repeat(box("th-mullion", FR, AL, T_CAP, [0, 0, K.Z_CAP], [K.FACE, K.H, zc]), 2, [DX, 0, 0]), "mullion"), *SAME, "+w"),
        cut(th(repeat(box("th-transom", FR, AL, T_CAP, [x0, 0, K.Z_CAP], [x1, K.FACE, zc]), 2, [0, 0.85, 0]), "transom"), "+w"),
        cut(th(box("th-transom-head", FR, AL, T_CAP, [x0, 3.25, K.Z_CAP], [x1, K.H, zc]), "transom"), "+w"),
    ]
    structure = [
        st(repeat(box("member-mullion", FR, AL, T_TUBE, [0, 0, K.Z_LIP], [K.FACE, K.H, K.Z_MULLION_BACK]), 2, [DX, 0, 0]), FR),
        st(repeat(box("member-transom", FR, AL, T_TUBE, [x0, 0, K.Z_LIP], [x1, K.FACE, K.Z_TRANSOM_BACK]), 2, [0, 0.85, 0]), FR),
        st(box("member-transom-head", FR, AL, T_TUBE, [x0, 3.25, K.Z_LIP], [x1, K.H, K.Z_TRANSOM_BACK]), FR),
        st_of("member-igu", "igu", IG),
        st_of("member-spandrel", "spandrel", SP),
        *supports(200, 0.016, (0.025, -0.04, 0.1, 0.2)),
        *masses(0.04)[1:],
    ]
    decisions = [
        text("Body: mullions and transoms are one extruded 150 mm grid (cap face to transom back) with the two openings, plus the room-side backs of the mullions to their 200 mm depth; cap, isolator and gaskets are inside the grid. The glass unit is one slab at the 32 mm the drawing gives it (28 mm real), the spandrel one 131 mm block from the glass face to the back pan.",
             "Körper: Pfosten und Riegel sind ein extrudiertes 150-mm-Raster (Kappenfläche bis Riegelrücken) mit den zwei Öffnungen, dazu die raumseitigen Rücken der Pfosten bis zu ihrer Tiefe von 200 mm; Kappe, Isolator und Dichtungen liegen im Raster. Die Isolierverglasung ist eine Scheibe mit den 32 mm der Zeichnung (28 mm wirklich), die Brüstung ein 131-mm-Block von der Glasfläche bis zur Rückwand."),
        text("Thermal: glass centre, 65 mm edge ring and spandrel as single pieces with their outer face, the frame regions as 30 mm cap zones over the 50 mm faces (only the total area is compared at this scale). Structure: solid mullion and transom bars from the glazing face to the room side, the glass and spandrel slabs as plates, bracket pins and spigots at the 30 mm feature, the masses of glass unit and spandrel (the frame mass is distributed over its bars).",
             "Thermik: Glasmitte, 65-mm-Randring und Brüstung als einzelne Stücke mit ihrer Außenfläche, die Rahmenregionen als 30-mm-Kappenzonen über den 50-mm-Ansichten (bei diesem Maßstab wird nur die Gesamtfläche verglichen). Tragwerk: volle Pfosten- und Riegelstäbe von der Glasseite bis zur Raumseite, Glas- und Brüstungsscheibe als Platten, Konsolbolzen und Stoßdorne im Merkmal von 30 mm, die Massen von Glaseinheit und Brüstung (die Rahmenmasse ist auf die Stäbe verteilt)."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


Z500 = (-0.1, -0.025)
SPAN_BACK500 = 0.02


def scale500():
    """🧱 1:500 (feature 75 mm): the two mullions as full-depth bars at the feature width, between them the 75 mm vision field (glass with the mid and head transoms) and the spandrel zone as one 120 mm block."""
    m = 0.075
    body = [
        repeat(box("mullion", FR, AL, T_CAP, [0, 0, K.Z_CAP], [m, K.H, K.Z_MULLION_BACK]), 2, [K.W - m, 0, 0]),
        box("vision", IG, GL, T_GLASS, [m - IX0, 0.85 - IY0, Z500[0]], [K.W - m - IX0, K.H - IY0, Z500[1]], role="glazing"),
        box("spandrel", SP, SG, T_SPAN, [m - SX0, -SY0, Z500[0]], [K.W - m - SX0, 0.85 - SY0, SPAN_BACK500]),
    ]
    centre = (m + 0.075 - IX0, 0.925 + 0.075 - IY0, K.W - m - 0.075 - IX0, K.H - m - 0.075 - IY0)
    edge = (m - IX0, 0.925 - IY0, K.W - m - IX0, K.H - m - IY0)
    thermal = [
        cut(th(pane("th-glass-centre", IG, GL, T_GLASS, centre, *Z500), "glass-centre"), *ALL_EDGES, "+w"),
        cut(th(pane("th-glass-edge", IG, GL, T_GLASS, edge, *Z500, hole=centre), "glass-edge"), *ALL_EDGES, "+w"),
        cut(th(box("th-spandrel", SP, SG, T_SPAN, [m - SX0, m - SY0, Z500[0]], [K.W - m - SX0, 0.85 - SY0, Z500[1]]), "spandrel"), *ALL_EDGES, "+w"),
        cut(th(repeat(box("th-mullion", FR, AL, T_CAP, [0, 0, Z500[0]], [m, K.H, Z500[1]]), 2, [K.W - m, 0, 0]), "mullion"), *ALL_EDGES, "+w"),
        cut(th(repeat(box("th-transom", FR, AL, T_CAP, [m, 0, Z500[0]], [K.W - m, m, Z500[1]]), 2, [0, 0.85, 0]), "transom"), *ALL_EDGES, "+w"),
        cut(th(box("th-transom-head", FR, AL, T_CAP, [m, K.H - m, Z500[0]], [K.W - m, K.H, Z500[1]]), "transom"), *ALL_EDGES, "+w"),
    ]
    structure = [
        st_of("member-mullion", "mullion", FR),
        st(box("member-transom", FR, AL, T_TUBE, [m, 0.85, Z500[0]], [K.W - m, 0.85 + m, Z500[1]]), FR),
        st(box("member-igu", IG, GL, T_GLASS, [m - IX0, 0.925 - IY0, Z500[0]], [K.W - m - IX0, K.H - m - IY0, Z500[1]]), IG),
        st_of("member-spandrel", "spandrel", SP),
        *supports(500, 0.0375, (0.0375, -0.025, 0.1, 0.2), 0.075),
        st(ellipsoid("mass-igu", IG, ST, 0.2, [0.0375, 0.0375, 0.0375], [IGU_W / 2, IGU_H / 2, -0.0625]), IG, "mass"),
    ]
    decisions = [
        text("Body: the two mullions as full-depth bars (cap to room-side back, 200 mm) at the 75 mm feature width; between them the vision field as one 75 mm layer at the face (glass unit with the mid and head transoms merged) and the spandrel zone as one 120 mm block (enamel, wool, back pan and base transom merged), held 80 mm short of the mullion backs so that the plan section keeps a printable step.",
             "Körper: die beiden Pfosten als Stäbe über die volle Tiefe (Kappe bis raumseitiger Rücken, 200 mm) in der Merkmalsbreite von 75 mm; dazwischen das Sichtfeld als eine 75-mm-Schicht an der Außenfläche (Isolierverglasung mit mittlerem und oberem Riegel zusammengefasst) und die Brüstungszone als ein 120-mm-Block (Email, Wolle, Rückwand und Fußriegel zusammengefasst), 80 mm hinter den Pfostenrücken zurückgesetzt, damit der Grundriss-Schnitt eine druckbare Stufe behält."),
        text("Minimal: thermal and structure are authored on these blocks and are already minimal. Thermal: glass centre, edge ring, spandrel, mullion and transom zones as 75 mm pieces of the face layer, each with its outer face only. Structure: the mullion bars as the members, the mid transom bar, the vision plate, the spandrel block, bracket pins and spigots at the 75 mm feature and the mass of the glass unit.",
             "Minimal: Thermik und Tragwerk sind auf diesen Blöcken erstellt und bereits minimal. Thermik: Glasmitte, Randring, Brüstung, Pfosten- und Riegelzonen als 75-mm-Stücke der Außenschicht, jedes nur mit seiner Außenfläche. Tragwerk: die Pfostenstäbe als Stäbe, der mittlere Riegelstab, die Sichtfeldplatte, der Brüstungsblock, Konsolbolzen und Stoßdorne im Merkmal von 75 mm und die Masse der Glaseinheit."),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def frame_parts():
    """⚖️ Parts of the frame with mass (kg) and height of their centre (m)."""
    rho = K.RHO
    mo, mc = K.tube(K.Z_MULLION_BACK)
    to, tc = K.tube(K.Z_TRANSOM_BACK)
    a_m = K.section([mo, mc])[0]
    a_t = K.section([to, tc])[0]
    a_cap = K.moments(K.cap())[0]
    a_plate = K.moments(K.plate())[0]
    a_iso = K.moments(K.isolator())[0]
    a_gaskets = 2 * (K.moments(K.gasket_out(1))[0] + K.moments(K.gasket_in(1))[0])
    per_m = lambda: (a_cap + a_plate) * rho["aluminium"] + a_iso * rho["polyamide"] + a_gaskets * rho["epdm"]
    parts = [("mullion tubes", 2 * K.H * a_m * rho["aluminium"], K.H / 2), ("mullion cover parts", 2 * K.H * per_m(), K.H / 2)]
    for name, yc in K.TRANSOM_Y:
        parts.append((f"transom {name} tube", 1.3 * a_t * rho["aluminium"], yc))
        parts.append((f"transom {name} cover parts", 1.3 * per_m(), yc))
    parts.append(("connectors and screws", 6 * 0.12 + 0.6, 1.39))
    return parts


def igu_parts():
    x0, y0, x1, y1 = VISION_OUT
    area = (x1 - x0) * (y1 - y0)
    seal = area - (x1 - x0 - 2 * K.SEAL_W) * (y1 - y0 - 2 * K.SEAL_W)
    return [("panes", 2 * 0.006 * area * K.RHO["glass"]), ("edge seal", seal * 0.016 * K.RHO["edge-seal"])]


def spandrel_parts():
    """⚖️ Parts of the spandrel with mass (kg) and depth of their centre (m)."""
    x0, y0, x1, y1 = SPANDREL_OUT
    glass = (x1 - x0) * (y1 - y0)
    packer = glass - SP_W * SP_H
    inner = (SP_W - 2 * K.PAN_T) * (SP_H - 2 * K.PAN_T)
    flanges = (SP_W * SP_H - inner) * (K.Z_PAN - K.Z_WOOL)
    return [
        ("enamelled glass", 0.006 * glass * K.RHO["glass"], (K.Z_GLASS_OUT + K.Z_ENAMEL_IN) / 2),
        ("packer", packer * (K.Z_GLASS_IN - K.Z_ENAMEL_IN) * K.RHO["epdm"], (K.Z_ENAMEL_IN + K.Z_GLASS_IN) / 2),
        ("wool", inner * (K.Z_PAN - K.Z_WOOL) * K.RHO["wool"], (K.Z_WOOL + K.Z_PAN) / 2),
        ("pan flanges", flanges * K.RHO["steel"], (K.Z_WOOL + K.Z_PAN) / 2),
        ("pan back", SP_W * SP_H * K.PAN_T * K.RHO["steel"], K.Z_PAN + K.PAN_T / 2),
    ]


FRAME_KG = sum(m for _, m, _ in frame_parts())
FRAME_COM_Y = sum(m * y for _, m, y in frame_parts()) / FRAME_KG
IGU_KG = sum(m for _, m in igu_parts())
SPANDREL_KG = sum(m for _, m, _ in spandrel_parts())
SPANDREL_COM_Z = sum(m * z for _, m, z in spandrel_parts()) / SPANDREL_KG

if __name__ == "__main__":
    volumes = {"50": scale50(), "100": scale100(), "200": scale200(), "500": scale500()}
    path = os.path.join(T, "volumes", "components", "curtain-wall-bay.json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, ensure_ascii=False, indent=2)
        f.write("\n")
    print("written", path, {s: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for s, v in volumes.items()})
    print("frame kg %.2f com y %.3f | igu kg %.2f | spandrel kg %.2f com z %.4f | total %.2f" % (FRAME_KG, FRAME_COM_Y, IGU_KG, SPANDREL_KG, SPANDREL_COM_Z, FRAME_KG + IGU_KG + SPANDREL_KG))
    for name, m, y in frame_parts():
        print("  %-28s %6.2f kg at y %.3f" % (name, m, y))
