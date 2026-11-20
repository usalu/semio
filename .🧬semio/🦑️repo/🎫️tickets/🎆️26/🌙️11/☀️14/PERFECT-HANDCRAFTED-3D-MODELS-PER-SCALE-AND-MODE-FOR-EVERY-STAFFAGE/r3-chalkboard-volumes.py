"""🖍️ Round 3 chalkboard: the handcrafted solids of every scale (body, thermal, structure), stated part by part in world coordinates, written to the volumes source `volumes/objects/chalkboard.json`.

Usage: python3 r3-chalkboard-volumes.py
"""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-chalkboard-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
box, prism, rod, turned, capsule, repeat, flag, th, th_of, st, st_of, text = K.box, K.prism, K.rod, K.turned, K.capsule, K.repeat, K.flag, K.th, K.th_of, K.st, K.st_of, K.text
FR, TY, LF = K.FR, K.TY, K.LF
ALU, ENAMEL, MDF, PA, CHALK, SPONGE, WOOD = K.ALU, K.ENAMEL, K.MDF, K.PA, K.CHALK, K.SPONGE, K.WOOD

T_FRAME, T_LEAF, T_HEAD, T_TRAY, T_FACE, T_LEAF_FACE, T_PA, T_GRIP, T_CORE = 0.75, 0.85, 0.6, 0.65, 0.3, 0.24, 0.28, 0.4, 0.7
W, BAR = K.HALF_W, K.BAR
FIX_Y = (K.Y_TRAY, K.Y_BOARD_TOP)
LEAF_Y = (K.Y_LEAF0, K.Y_LEAF1)
LEAF_X = (-W, 0.0)
PANEL_Z = (K.ZF0 + K.WALL, K.ZF1 - K.WALL)
LEAF_PANEL_Z = (K.ZL0 + K.WALL, K.ZL1 - K.WALL)
FIX_PANELS = (("left", -W + K.POCKET, -K.MULLION + K.BITE), ("right", K.MULLION - K.BITE, W - K.POCKET))
FIX_OPENINGS = (("left", -W + BAR, -K.MULLION), ("right", K.MULLION, W - BAR))
HANG_STEP = K.HANGERS[1] - K.HANGERS[0]
HANDLE_STEP = K.HANDLE_X[1] - K.HANDLE_X[0]


def bar_ring(prefix, bone, tone, x, y, z0, bottom_slot=False):
    """🖼️ Four bars of the 35 x 20 edge profile between the corner pieces of one framed panel: bottom and top bar along X, two stiles along Y; the profile's soft edge faces the room, its pocket the panel."""
    out = []
    for name, y_out, sign in (("bottom", y[0], 1), ("top", y[1], -1)):
        outline, hole = K.edge_profile(bottom_slot and name == "bottom")
        to = lambda ring: [[y_out + sign * s, z0 + d] for s, d in ring]
        out.append(prism(f"{prefix}bar-{name}", bone, ALU, tone, to(outline), "yz", x[0] + BAR, x[1] - BAR, holes=[to(hole)] if hole else None))
    outline, hole = K.edge_profile()
    for name, x_out, sign in (("left", x[0], 1), ("right", x[1], -1)):
        to = lambda ring: [[z0 + d, x_out + sign * s] for s, d in ring]
        out.append(prism(f"{prefix}stile-{name}", bone, ALU, tone, to(outline), "zx", y[0] + BAR, y[1] - BAR, holes=[to(hole)]))
    return out


def corner_pieces(prefix, bone, x, y, z0):
    """🧷 Four moulded polyamide corner pieces 35 x 35 x 20 with the 10 mm safety radius the bars plug into."""
    out, r = [], K.R_CORNER
    for name, cx, cy, sx, sy, a0 in (("bottom-left", x[0], y[0], 1, 1, 180), ("bottom-right", x[1], y[0], -1, 1, 270), ("top-right", x[1], y[1], -1, -1, 0), ("top-left", x[0], y[1], 1, -1, 90)):
        ring = K.arc((cx + sx * r, cy + sy * r), r, a0, a0 + 90)
        inner = [[cx + sx * BAR, cy], [cx + sx * BAR, cy + sy * BAR], [cx, cy + sy * BAR]]
        if sx * sy < 0:
            inner.reverse()
        out.append(prism(f"{prefix}corner-{name}", bone, PA, T_PA, K.clean(ring + inner), "xy", z0, z0 + K.BAR_D, role="detail"))
    return out


def fixed50():
    out = bar_ring("frame-", FR, T_FRAME, (-W, W), FIX_Y, K.ZF0, bottom_slot=True)
    out += corner_pieces("frame-", FR, (-W, W), FIX_Y, K.ZF0)
    out.append(prism("frame-mullion", FR, ALU, T_FRAME, K.mullion_profile(), "zx", FIX_Y[0] + BAR, FIX_Y[1] - BAR))
    for name, x0, x1 in FIX_PANELS:
        out.append(box(f"panel-{name}", FR, ENAMEL, T_FACE, (x0, x1), (FIX_Y[0] + K.POCKET, FIX_Y[1] - K.POCKET), PANEL_Z))
    outline, hole = K.header_profile()
    out.append(prism("header-track", FR, ALU, T_HEAD, outline, "yz", -W + K.CAP, W - K.CAP, holes=[hole]))
    out.append(repeat(prism("header-end-caps", FR, PA, T_PA, K.header_envelope(), "yz", -W, -W + K.CAP, role="detail"), 2, [2 * W - K.CAP, 0, 0]))
    out.append(repeat(box("header-stops", FR, PA, T_PA, (-1.745, -1.725), (1.226, 1.256), (-0.012, 0.008), role="detail"), 2, [3.47, 0, 0]))
    out.append(flag(prism("wall-cleat", FR, ALU, T_HEAD, K.wall_cleat_profile(), "yz", -K.CLEAT_X, K.CLEAT_X), slender=True))
    out.append(flag(prism("board-cleat", FR, ALU, T_HEAD, K.board_cleat_profile(), "yz", -K.CLEAT_X, K.CLEAT_X), slender=True))
    out.append(repeat(turned("wall-buffers", FR, PA, T_PA, (-1.8, K.Y_BUFFER, K.ZF1), (0, 0, 1), [[0, K.BUFFER_R], [0.007, K.BUFFER_R], [0.008, K.BUFFER_R - 0.001], [0.008, 0]], role="detail"), 5, [0.9, 0, 0]))
    return out


def leaf50():
    out = bar_ring("leaf-", LF, T_LEAF, LEAF_X, LEAF_Y, K.ZL0)
    out += corner_pieces("leaf-", LF, LEAF_X, LEAF_Y, K.ZL0)
    out.append(box("leaf-panel", LF, ENAMEL, T_LEAF_FACE, (LEAF_X[0] + K.POCKET, LEAF_X[1] - K.POCKET), (LEAF_Y[0] + K.POCKET, LEAF_Y[1] - K.POCKET), LEAF_PANEL_Z))
    x0, zh = K.HANGERS[0], K.Z_HANG
    step = [HANG_STEP, 0, 0]
    out.append(repeat(box("trolley-chassis", LF, PA, T_PA, (x0 - 0.025, x0 + 0.025), (1.226, 1.244), (zh - 0.004, zh + 0.004), role="detail"), 2, step))
    wheel = [[0, 0.0035], [0.0005, 0.0035], [0.0005, 0.007], [0.0015, 0.008], [0.0065, 0.008], [0.0075, 0.007], [0.0075, 0]]
    for side, z, axis in (("front", zh - 0.0035, (0, 0, -1)), ("back", zh + 0.0035, (0, 0, 1))):
        for end, dx in (("a", -0.014), ("b", 0.014)):
            out.append(repeat(turned(f"trolley-wheels-{side}-{end}", LF, PA, 0.2, (x0 + dx, 1.232, z), axis, wheel, role="detail"), 2, step))
    out.append(repeat(rod("hanger-bolts", LF, ALU, T_GRIP, (x0, 1.206, zh), (0, 1, 0), 0.03, 0.004, role="detail"), 2, step))
    out.append(repeat(prism("hanger-lock-nuts", LF, ALU, T_GRIP, K.hexagon(zh, x0, 0.013), "zx", 1.2142, 1.2217, role="detail"), 2, step))
    out.append(repeat(prism("leaf-glides", LF, PA, T_PA, K.glide_profile(), "yz", x0 - 0.02, x0 + 0.02, role="detail"), 2, step))
    hx, (hy0, hy1), r = K.HANDLE_X[0], K.HANDLE_Y, K.HANDLE_R
    hstep = [HANDLE_STEP, 0, 0]
    for name, hy in (("lower", hy0), ("upper", hy1)):
        out.append(repeat(capsule(f"handle-legs-{name}", LF, ALU, T_GRIP, (hx, hy, K.ZL0 - 0.001), (hx, hy, K.Z_GRIP), r, role="detail"), 2, hstep))
    out.append(repeat(capsule("handle-grips", LF, ALU, T_GRIP, (hx, hy0, K.Z_GRIP), (hx, hy1, K.Z_GRIP), r, role="detail"), 2, hstep))
    return out


def chalk(i, x, z, length):
    """🖍️ One stick of round blackboard chalk, 10 mm tapering to 9 mm, lying on the tray floor."""
    return turned(i, TY, CHALK, 0.9, (x, K.T_TRAY + 0.005, z), (1, 0, 0), [[0, 0], [0, 0.0046], [0.0008, 0.005], [length - 0.0008, 0.0045], [length, 0.0041], [length, 0]], role="detail")


def tray50():
    outline, hole = K.tray_profile()
    out = [prism("tray", TY, ALU, T_TRAY, outline, "yz", -W + K.CAP, W - K.CAP, holes=[hole])]
    out.append(repeat(prism("tray-end-caps", TY, PA, T_PA, K.tray_envelope(), "yz", -W, -W + K.CAP, role="detail"), 2, [2 * W - K.CAP, 0, 0]))
    out += [chalk("chalk-a", -1.30, -0.031, 0.08), chalk("chalk-b", -1.19, -0.0295, 0.08), chalk("chalk-c", -1.06, -0.032, 0.052)]
    out.append(box("sponge", TY, SPONGE, 0.6, (0.90, 1.04), (K.T_TRAY, K.T_TRAY + 0.036), (-0.037, 0.008), role="detail", radius=0.006))
    y0 = K.T_TRAY
    out.append(flag(prism("ruler", TY, WOOD, 0.55, [[y0, -0.035], [y0, 0.005], [y0 + 0.006, 0.005], [y0 + 0.006, -0.027], [y0 + 0.002, -0.035]], "yz", -0.7, 0.3, role="detail"), slender=True))
    out.append(box("ruler-handle", TY, WOOD, 0.5, (-0.255, -0.145), (y0 + 0.006, y0 + 0.028), (-0.024, -0.006), role="detail", radius=0.004))
    return out


def panel_layers(prefix, bone, region, x, y, z):
    """🌡️ One enamelled panel as its two declared layers: the 16 mm MDF core and the 0.6 mm ceramic-steel sheet bonded to its room face (the bonded back of the sheet is no surface)."""
    return [
        th(box(f"{prefix}-core", bone, MDF, T_CORE, x, y, (z[0] + K.T_ENAMEL, z[1])), region, "core"),
        th(box(f"{prefix}-enamel", bone, ENAMEL, T_FACE, x, y, (z[0], z[0] + K.T_ENAMEL)), region, "enamel", ["+w"]),
    ]


def enamel_sheet(i, bone, region, x, y, z_face):
    """🌡️ The enamelled writing face alone, 0.6 mm on the room side of its plate: the surface of a panel at the coarser scales, where core and plate merge."""
    return th(box(i, bone, ENAMEL, T_FACE, x, y, (z_face, z_face + K.T_ENAMEL)), region, "enamel", ["+w"])


def mass_disc(i, bone, x, y, z, radius, member):
    """⚖️ A panel's weight as a flat disc at its centre of mass, inside the panel thickness."""
    return st(turned(i, bone, MDF, 0.35, (x, y, z[0]), (0, 0, 1), [[0, radius], [z[1] - z[0], radius]]), member, "mass")


def scale50():
    fixed, leaf, tray = fixed50(), leaf50(), tray50()
    body = fixed + leaf + tray
    thermal = []
    for name, x0, x1 in FIX_PANELS:
        thermal += panel_layers(f"th-panel-{name}", FR, "board-face", (x0, x1), (FIX_Y[0] + K.POCKET, FIX_Y[1] - K.POCKET), PANEL_Z)
    thermal += panel_layers("th-leaf-panel", LF, "leaf", (LEAF_X[0] + K.POCKET, LEAF_X[1] - K.POCKET), (LEAF_Y[0] + K.POCKET, LEAF_Y[1] - K.POCKET), LEAF_PANEL_Z)
    env = K.edge_envelope()
    for name, y_out, sign in (("bottom", FIX_Y[0], 1), ("top", FIX_Y[1], -1)):
        thermal.append(th(prism(f"th-frame-bar-{name}", FR, ALU, T_FRAME, [[y_out + sign * a, K.ZF0 + d] for a, d in env], "yz", -W + BAR, W - BAR), "frame"))
    for name, x_out, sign in (("left", -W, 1), ("right", W, -1)):
        thermal.append(th(prism(f"th-frame-stile-{name}", FR, ALU, T_FRAME, [[K.ZF0 + d, x_out + sign * a] for a, d in env], "zx", FIX_Y[0] + BAR, FIX_Y[1] - BAR), "frame"))
    thermal.append(th(box("th-frame-mullion", FR, ALU, T_FRAME, (-K.MULLION, K.MULLION), (FIX_Y[0] + BAR, FIX_Y[1] - BAR), (K.ZF0, K.ZF1)), "frame"))
    thermal.append(th(prism("th-header-track", FR, ALU, T_HEAD, K.header_envelope(), "yz", -W, W), "frame"))
    thermal.append(th(prism("th-tray", TY, ALU, T_TRAY, K.tray_profile()[0], "yz", -W, W), "tray"))
    own = ("panel-left", "panel-right", "leaf-panel", "frame-bar-bottom", "frame-bar-top", "frame-stile-left", "frame-stile-right", "frame-mullion", "header-track", "header-end-caps", "header-stops", "wall-cleat", "board-cleat", "tray", "tray-end-caps")
    region = {FR: "frame", LF: "leaf", TY: "tray"}
    for s in body:
        if s["id"] not in own:
            thermal.append(th_of("th-" + s["id"], s["id"], region[s["bone"]]))
    structure = []
    for s in fixed:
        if s["id"].startswith(("frame-bar", "frame-stile", "frame-mullion")) or s["id"] in ("header-track", "board-cleat"):
            structure.append(st_of("st-" + s["id"], s["id"], FR))
        elif s["id"].startswith("frame-corner"):
            structure.append(st_of("st-" + s["id"], s["id"], FR, "joint"))
    structure.append(repeat(st(box("st-mullion-joints", FR, ALU, 0.4, (-0.015, 0.015), (FIX_Y[0] + BAR - 0.006, FIX_Y[0] + BAR + 0.006), (K.ZF0, K.ZF1)), FR, "joint"), 2, [0, FIX_Y[1] - FIX_Y[0] - 2 * BAR, 0]))
    structure.append(st_of("support-wall-hook", "wall-cleat", FR, "support", "wall-hook"))
    structure.append(st_of("support-wall-rest", "wall-buffers", FR, "support", "wall-rest"))
    for name, x0, x1 in FIX_PANELS:
        structure.append(mass_disc(f"mass-panel-{name}", FR, (x0 + x1) / 2, (FIX_Y[0] + FIX_Y[1]) / 2, PANEL_Z, 0.12, FR))
    structure.append(st_of("st-tray", "tray", TY))
    structure.append(repeat(st(rod("st-tray-screws", TY, ALU, 0.4, (-1.8, K.Y_TRAY - 0.004, 0.022), (0, 1, 0), 0.008, 0.004), TY, "joint"), 5, [0.9, 0, 0]))
    for s in leaf:
        if s["id"].startswith(("leaf-bar", "leaf-stile")):
            structure.append(st_of("st-" + s["id"], s["id"], LF))
        elif s["id"].startswith("leaf-corner"):
            structure.append(st_of("st-" + s["id"], s["id"], LF, "joint"))
    structure.append(st_of("st-hanger-bolts", "hanger-bolts", LF, "joint"))
    structure.append(st_of("st-trolley-chassis", "trolley-chassis", LF, "joint"))
    structure.append(st_of("st-leaf-glides", "leaf-glides", LF, "joint"))
    structure.append(mass_disc("mass-leaf-panel", LF, -1.0, (LEAF_Y[0] + LEAF_Y[1]) / 2, LEAF_PANEL_Z, 0.12, LF))
    decisions = [
        text("Body: the board as it is built. Fixed board: four bars of a 35 x 20 mm aluminium edge profile (1.7 mm walls, closed chamber, 12 mm pocket for the panel edge, soft front edge) plugged into four moulded corner pieces with the 10 mm safety radius, an H mullion between the two back panels, each panel 16.6 mm (0.6 mm ceramic steel on 16 mm MDF) seated 12 mm in the pockets; the bottom bar has the 6 mm guide slot of the leaf. On the top bar the 48.5 x 38 mm header track with its running chamber, treads, slot, closed back box, 7.5 mm end caps and two end stops. Behind: the pair of 2.5 mm Z cleats the board hangs on (their screw heads of 3 mm are below the 7.5 mm feature and left out), five 30 mm buffers at the bottom bar. Leaf: the same framed panel 2.0 x 1.146 m, hung on two trolleys (chassis, four 16 mm wheels with their hubs each) by M8 hanger bolts with flange lock nuts in the 8 mm gap, guided at the bottom by two glides in the slot, two bow handles of 8 mm rod on the stiles. Tray: one extrusion with 2 mm floor, 18 mm front lip, 50 mm trough and the closed box that carries the bottom bar, two end caps; three sticks of chalk, the sponge, the 1 m ruler with its handle.",
             "Körper: die Tafel, wie sie gebaut ist. Feste Tafel: vier Stäbe eines Aluminium-Kantenprofils 35 x 20 mm (Wand 1,7 mm, geschlossene Kammer, 12 mm Tasche für die Plattenkante, weiche Vorderkante), in vier gespritzte Eckstücke mit dem Sicherheitsradius 10 mm gesteckt, ein H-Mittelsteg zwischen den beiden Rückplatten, jede Platte 16,6 mm (0,6 mm Keramikstahl auf 16 mm MDF), 12 mm tief in den Taschen; der untere Stab hat den 6-mm-Führungsschlitz des Flügels. Auf dem oberen Stab die Laufschiene 48,5 x 38 mm mit Laufkammer, Laufflächen, Schlitz, geschlossenem Kasten dahinter, Endkappen von 7,5 mm und zwei Endanschlägen. Dahinter: das Paar 2,5 mm starker Z-Leisten, an dem die Tafel hängt (ihre Schraubenköpfe von 3 mm liegen unter dem Kleinstmaß von 7,5 mm und entfallen), fünf Puffer von 30 mm am unteren Stab. Flügel: dieselbe gerahmte Platte 2,0 x 1,146 m, an zwei Laufwagen (Körper, je vier Rollen von 16 mm mit ihren Naben) mit M8-Hängebolzen und Flansch-Kontermuttern im 8-mm-Spalt aufgehängt, unten von zwei Gleitern im Schlitz geführt, zwei Bügelgriffe aus 8-mm-Rundstab auf den Pfosten. Ablage: ein Strangpressprofil mit 2 mm Boden, 18 mm Vorderlippe, 50 mm Mulde und dem geschlossenen Kasten, der den unteren Stab trägt, zwei Endkappen; drei Kreidestücke, der Schwamm, das 1-m-Lineal mit Griff."),
        text("Thermal: every panel is its two declared layers, the 16 mm MDF core and the 0.6 mm ceramic-steel sheet on its room face (the bonded back of the sheet counts for no area); the frame bars, the mullion, the header track and the tray are their closed outer envelopes, the surface the room air touches (the chambers of the 1.7 to 2 mm profiles are no surface); corner pieces, buffers, leaf frame with its hardware and the tray contents are the body parts themselves. The two hanging cleats lie in the 8 mm gap against the wall and are no part of the frame surface.",
             "Thermik: jede Platte besteht aus ihren zwei deklarierten Schichten, dem MDF-Kern von 16 mm und dem Keramikstahlblech von 0,6 mm auf der Raumseite (die verklebte Rückseite des Blechs zählt nicht zur Fläche); Rahmenstäbe, Mittelsteg, Laufschiene und Ablage sind ihre geschlossenen Außenhüllen, die Fläche, die die Raumluft berührt (die Kammern der 1,7 bis 2 mm starken Profile sind keine Oberfläche); Eckstücke, Puffer, Flügelrahmen mit Beschlägen und der Inhalt der Ablage sind die Körperteile selbst. Die beiden Hängeleisten liegen im 8-mm-Spalt an der Wand und gehören nicht zur Rahmenoberfläche."),
        text("Structure: the load path. The frame bars, the mullion, the header track and the board cleat are the members, the corner pieces and the mullion ends the joints; the panels stand in the pockets and act as two masses at their centres. The wall cleat is the pin support at the hanging rail, the five buffers are the roller support. The leaf is its four bars with corner joints, hung at the two trolleys and hanger bolts and guided by the two glides, its panel a mass; the tray extrusion is a member screwed under the bottom bar at five points.",
             "Tragwerk: der Lastweg. Rahmenstäbe, Mittelsteg, Laufschiene und Tafelleiste sind die Stäbe, Eckstücke und Mittelstegenden die Knoten; die Platten stehen in den Taschen und wirken als zwei Massen in ihren Schwerpunkten. Die Wandleiste ist das Festlager an der Hängeschiene, die fünf Puffer sind das Rollenlager. Der Flügel besteht aus seinen vier Stäben mit Eckknoten, hängt an den zwei Laufwagen und Hängebolzen und wird von den zwei Gleitern geführt, seine Platte ist eine Masse; das Ablageprofil ist ein Stab, an fünf Punkten unter den unteren Stab geschraubt."),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}

def ring(i, bone, material, tone, x, y, z, openings, radius=0.0, **extra):
    """🖼️ A frame as one plate with its panel openings."""
    return prism(i, bone, material, tone, K.rounded_rect(x[0], y[0], x[1], y[1], radius), "xy", z[0], z[1], holes=[K.rounded_rect(a0, b0, a1, b1, 0) for a0, b0, a1, b1 in openings], **extra)


def bars(prefix, bone, tone, x, y, z, width, mullion=None):
    """🏗️ The bars of a frame as plain members: bottom and top bar over the full width, the stiles (and the mullion) between them."""
    out = [
        box(f"{prefix}bar-bottom", bone, ALU, tone, x, (y[0], y[0] + width), z),
        box(f"{prefix}bar-top", bone, ALU, tone, x, (y[1] - width, y[1]), z),
        box(f"{prefix}stile-left", bone, ALU, tone, (x[0], x[0] + width), (y[0] + width, y[1] - width), z),
        box(f"{prefix}stile-right", bone, ALU, tone, (x[1] - width, x[1]), (y[0] + width, y[1] - width), z),
    ]
    if mullion:
        out.append(box(f"{prefix}mullion", bone, ALU, tone, (-mullion, mullion), (y[0] + width, y[1] - width), z))
    return [st(s, bone) for s in out]


def scale100():
    ly = (K.Y_TRAY, K.Y_BOARD_TOP)
    fo = [(x0, FIX_Y[0] + BAR, x1, FIX_Y[1] - BAR) for _, x0, x1 in FIX_OPENINGS]
    lo = (LEAF_X[0] + BAR, ly[0] + BAR, LEAF_X[1] - BAR, ly[1] - BAR)
    fz, lz = (K.ZF0, K.ZF1), (K.ZL0, K.ZL1)
    y0, y1, t = K.Y_BOARD_TOP, K.Y_TOP, K.T_HEAD
    hood = [[y0, K.ZH0], [y1, K.ZH0], [y1, K.Z_WALL], [y0, K.Z_WALL], [y0, K.CHAMBER_Z[1]], [y1 - t, K.CHAMBER_Z[1]], [y1 - t, K.CHAMBER_Z[0]], [y0, K.CHAMBER_Z[0]]]
    tt = K.T_TRAY
    trough = [[0, K.ZT0], [K.Y_LIP, K.ZT0], [K.Y_LIP, K.ZT0 + tt], [tt, K.ZT0 + tt], [tt, K.Z_BOX], [K.Y_TRAY, K.Z_BOX], [K.Y_TRAY, K.ZT1], [0, K.ZT1]]
    body = [
        ring("frame", FR, ALU, T_FRAME, (-W, W), FIX_Y, fz, fo, K.R_CORNER),
        box("panel-left", FR, ENAMEL, T_FACE, (fo[0][0], fo[0][2]), (fo[0][1], fo[0][3]), PANEL_Z),
        box("panel-right", FR, ENAMEL, T_FACE, (fo[1][0], fo[1][2]), (fo[1][1], fo[1][3]), PANEL_Z),
        prism("header-track", FR, ALU, T_HEAD, hood, "yz", -W, W),
        ring("leaf-frame", LF, ALU, T_LEAF, LEAF_X, ly, lz, [lo], K.R_CORNER),
        box("leaf-panel", LF, ENAMEL, T_LEAF_FACE, (lo[0], lo[2]), (lo[1], lo[3]), LEAF_PANEL_Z),
        prism("tray", TY, ALU, T_TRAY, trough, "yz", -W, W),
        box("sponge", TY, SPONGE, 0.6, (0.90, 1.04), (tt, tt + 0.036), (-0.037, 0.008), role="detail"),
    ]
    thermal = [
        enamel_sheet("th-panel-left", FR, "board-face", (fo[0][0], fo[0][2]), (fo[0][1], fo[0][3]), PANEL_Z[0]),
        enamel_sheet("th-panel-right", FR, "board-face", (fo[1][0], fo[1][2]), (fo[1][1], fo[1][3]), PANEL_Z[0]),
        th_of("th-frame", "frame", "frame"),
        th(box("th-header-track", FR, ALU, T_HEAD, (-W, W), (y0, y1), (K.ZH0, K.ZF1)), "frame"),
        enamel_sheet("th-leaf-panel", LF, "leaf", (lo[0], lo[2]), (lo[1], lo[3]), LEAF_PANEL_Z[0]),
        th_of("th-leaf-frame", "leaf-frame", "leaf"),
        th_of("th-tray", "tray", "tray"),
        th_of("th-sponge", "sponge", "tray"),
    ]
    structure = bars("st-frame-", FR, T_FRAME, (-W, W), FIX_Y, fz, BAR, K.MULLION)
    structure += [
        st_of("st-header-track", "header-track", FR),
        st(box("support-wall-hook", FR, ALU, 0.15, (-0.15, 0.15), (1.1925, 1.2075), (K.Z_WALL - 0.015, K.Z_WALL)), FR, "support", "wall-hook"),
        st(rod("support-wall-rest", FR, PA, 0.15, (-0.15, K.Y_BUFFER, K.Z_WALL - 0.0075), (1, 0, 0), 0.3, 0.0075), FR, "support", "wall-rest"),
        mass_disc("mass-panel-left", FR, (fo[0][0] + fo[0][2]) / 2, (FIX_Y[0] + FIX_Y[1]) / 2, PANEL_Z, 0.12, FR),
        mass_disc("mass-panel-right", FR, (fo[1][0] + fo[1][2]) / 2, (FIX_Y[0] + FIX_Y[1]) / 2, PANEL_Z, 0.12, FR),
        st_of("st-tray", "tray", TY),
        repeat(st(box("st-tray-fixings", TY, ALU, 0.4, (-1.815, -1.785), (K.Y_TRAY - 0.008, K.Y_TRAY + 0.008), (0.014, 0.030)), TY, "joint"), 3, [1.8, 0, 0]),
    ]
    structure += bars("st-leaf-", LF, T_LEAF, LEAF_X, ly, lz, BAR)
    structure += [
        repeat(st(box("st-hangers", LF, ALU, 0.4, (K.HANGERS[0] - 0.025, K.HANGERS[0] + 0.025), (ly[1] - 0.008, y1 - t), (K.Z_HANG - 0.0075, K.Z_HANG + 0.0075)), LF, "joint"), 2, [HANG_STEP, 0, 0]),
        mass_disc("mass-leaf-panel", LF, -1.0, (ly[0] + ly[1]) / 2, LEAF_PANEL_Z, 0.12, LF),
    ]
    decisions = [
        text("Body: each framed panel is one frame plate of the 35 mm bars with the 10 mm corner radius and its panel openings (profile chambers, pockets and corner pieces merged), the panels fill the openings 16.6 mm thick. The header track is a hood over the leaf whose solid back reaches the wall plane (treads, slot, end caps and the two hanging cleats merged). The leaf reaches from the tray to the header: its running gaps of 8 mm are below the 15 mm feature and closed, the trolleys hidden in the hood are left out. The tray keeps its 2 mm floor, lip, trough and the box as a solid; only the sponge lies in it.",
             "Körper: jede gerahmte Platte ist eine Rahmenscheibe aus den 35-mm-Stäben mit dem Eckradius 10 mm und ihren Plattenöffnungen (Profilkammern, Taschen und Eckstücke zusammengefasst), die Platten füllen die Öffnungen 16,6 mm dick. Die Laufschiene ist eine Haube über dem Flügel, deren voller Rücken bis zur Wandebene reicht (Laufflächen, Schlitz, Endkappen und die beiden Hängeleisten zusammengefasst). Der Flügel reicht von der Ablage bis zur Laufschiene: seine Laufspalte von 8 mm liegen unter dem Kleinstmaß von 15 mm und sind geschlossen, die in der Haube verdeckten Laufwagen entfallen. Die Ablage behält ihren 2-mm-Boden, Lippe, Mulde und den Kasten als Vollkörper; nur der Schwamm liegt darin."),
        text("Omitted below 15 mm: chalk, ruler, bow handles, glides, buffers, lock nuts, wheels. Thermal: each panel is its 0.6 mm enamel sheet, the surface that exchanges (the core is merged into the plate, as the check counts every solid of a region at this scale); frame plate, header as its closed envelope, leaf frame, tray and sponge. Structure: the bars of both frames as plain 35 x 20 members with the mullion, the header track, the hanging rail as pin block and the buffer line as roller at the back, panels as masses, two hanger joints in the hood, the tray with three fixings.",
             "Unter 15 mm entfallen: Kreide, Lineal, Bügelgriffe, Gleiter, Puffer, Kontermuttern, Rollen. Thermik: jede Platte ist ihr Emailblech von 0,6 mm, die Fläche, die Wärme tauscht (der Kern geht in der Scheibe auf, da die Prüfung in diesem Maßstab jeden Körper einer Region zählt); Rahmenscheibe, Laufschiene als geschlossene Hülle, Flügelrahmen, Ablage und Schwamm. Tragwerk: die Stäbe beider Rahmen als glatte Stäbe 35 x 20 mit dem Mittelsteg, die Laufschiene, die Hängeschiene als Festlagerblock und die Pufferlinie als Rolle an der Rückseite, Platten als Massen, zwei Hängerknoten in der Haube, die Ablage mit drei Befestigungen."),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale200():
    slab_z, lz = (K.ZL1, K.Z_WALL), (K.ZL1 - 0.024, K.ZL1)
    ly = (K.Y_TRAY, K.Y_BOARD_TOP)
    step = [[0, K.ZT0], [K.Y_LIP, K.ZT0], [K.Y_LIP, K.Z_BOX], [K.Y_TRAY, K.Z_BOX], [K.Y_TRAY, K.Z_WALL], [0, K.Z_WALL]]
    body = [
        box("board", FR, ENAMEL, T_FACE, (-W, W), FIX_Y, slab_z),
        box("header", FR, ALU, T_HEAD, (-W, W), (K.Y_BOARD_TOP, K.Y_TOP), (K.ZH0, K.Z_WALL)),
        box("leaf", LF, ENAMEL, T_LEAF_FACE, LEAF_X, ly, lz),
        prism("tray", TY, ALU, T_TRAY, step, "yz", -W, W),
    ]
    fo = [(x0, FIX_Y[0] + BAR, x1, FIX_Y[1] - BAR) for _, x0, x1 in FIX_OPENINGS]
    thermal = [
        enamel_sheet("th-face-left", FR, "board-face", (fo[0][0], fo[0][2]), (fo[0][1], fo[0][3]), slab_z[0]),
        enamel_sheet("th-face-right", FR, "board-face", (fo[1][0], fo[1][2]), (fo[1][1], fo[1][3]), slab_z[0]),
        th(ring("th-frame", FR, ALU, T_FRAME, (-W, W), FIX_Y, slab_z, fo, sectionFaces=["+w"]), "frame"),
        th_of("th-header", "header", "frame"),
        enamel_sheet("th-leaf", LF, "leaf", LEAF_X, ly, lz[0]),
        th_of("th-tray", "tray", "tray"),
    ]
    fz = slab_z
    structure = bars("st-frame-", FR, T_FRAME, (-W, W), (FIX_Y[0], K.Y_TOP), fz, 0.04)
    structure += [
        st(box("support-wall-hook", FR, ALU, 0.15, (-0.15, 0.15), (1.185, 1.215), (0.014, 0.044)), FR, "support", "wall-hook"),
        st(rod("support-wall-rest", FR, PA, 0.15, (-0.15, K.Y_BUFFER, 0.029), (1, 0, 0), 0.3, 0.015), FR, "support", "wall-rest"),
        mass_disc("mass-panels", FR, 0.0, (FIX_Y[0] + FIX_Y[1]) / 2, slab_z, 0.17, FR),
        st_of("st-tray", "tray", TY),
    ]
    structure += bars("st-leaf-", LF, T_LEAF, LEAF_X, ly, lz, 0.04)
    structure.append(mass_disc("mass-leaf-panel", LF, -1.0, (ly[0] + ly[1]) / 2, lz, 0.12, LF))
    decisions = [
        text("Body: four masses. The fixed board with its frame, both panels and the 8 mm hanging gap is one slab down to the wall plane; the header track with the hangers is one bar over it and over the leaf; the leaf is one slab of 24 mm (20 mm thickened to the printable width of its end view) that reaches from the tray to the header (the 8 mm running gaps are closed); the tray is a stepped bar, its trough filled to the height of the lip.",
             "Körper: vier Massen. Die feste Tafel mit Rahmen, beiden Platten und dem 8-mm-Hängespalt ist eine Scheibe bis zur Wandebene; die Laufschiene mit den Hängern ist ein Balken darüber und über dem Flügel; der Flügel ist eine Scheibe von 24 mm (20 mm, auf die druckbare Breite seiner Stirnansicht verdickt) von der Ablage bis zur Laufschiene (die Laufspalte von 8 mm sind geschlossen); die Ablage ist ein gestufter Balken, die Mulde bis zur Lippenhöhe gefüllt."),
        text("Thermal: the two writing faces and the leaf face as 0.6 mm enamel sheets on their slabs, the frame plate round the faces, the header and the tray; the cores are merged. Structure: both frames as four 40 mm bars each (the header and the top bar are one chord), the panels as masses, the hanging rail as pin block and the buffers as roller at the back, the tray bar.",
             "Thermik: die zwei Schreibflächen und die Flügelfläche als Emailbleche von 0,6 mm auf ihren Scheiben, die Rahmenscheibe um die Flächen, die Laufschiene und die Ablage; die Kerne sind zusammengefasst. Tragwerk: beide Rahmen als je vier Stäbe von 40 mm (Laufschiene und oberer Stab sind ein Gurt), die Platten als Massen, die Hängeschiene als Festlagerblock und die Puffer als Rolle an der Rückseite, der Ablagebalken."),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale500():
    bz, lz = (0.0, K.Z_WALL), (K.ZT0, 0.0)
    by = (K.Y_TRAY, K.Y_TOP)
    ly = (K.Y_TRAY, K.Y_BOARD_TOP)
    body = [
        box("board", FR, ENAMEL, T_FACE, (-W, W), by, bz),
        box("leaf", LF, ENAMEL, T_LEAF_FACE, LEAF_X, ly, lz),
        box("tray", TY, ALU, T_TRAY, (-W, W), (0.0, K.Y_TRAY), (K.ZT0, K.Z_WALL)),
    ]
    thermal = [
        enamel_sheet("th-face", FR, "board-face", (-W + 0.075, W - 0.075), (by[0] + 0.075, by[1] - 0.075), bz[0]),
        th(ring("th-frame", FR, ALU, T_FRAME, (-W, W), by, bz, [(-W + 0.075, by[0] + 0.075, W - 0.075, by[1] - 0.075)]), "frame"),
        enamel_sheet("th-leaf", LF, "leaf", LEAF_X, ly, lz[0]),
        th_of("th-tray", "tray", "tray"),
    ]
    structure = [
        st(ring("st-frame", FR, ALU, T_FRAME, (-W, W), by, bz, [(-W + 0.075, by[0] + 0.075, W - 0.075, by[1] - 0.075)]), FR),
        st(box("support-wall-hook", FR, ALU, 0.15, (-0.15, 0.15), (1.1625, 1.2375), (0.004, 0.079)), FR, "support", "wall-hook"),
        st(box("support-wall-rest", FR, PA, 0.15, (-0.15, 0.15), (0.06, 0.135), (0.004, 0.079)), FR, "support", "wall-rest"),
        st(ring("st-leaf", LF, ALU, T_LEAF, LEAF_X, ly, lz, [(LEAF_X[0] + 0.075, ly[0] + 0.075, LEAF_X[1] - 0.075, ly[1] - 0.075)]), LF),
        st_of("st-tray", "tray", TY),
    ]
    decisions = [
        text("Body: three masses, already minimal. The fixed board with header track, frame and hanging gap is one slab on the wall side, the leaf one slab on the room side that still slides with its bone, the tray the 60 x 80 mm bar at the foot, true to size.",
             "Körper: drei Massen, bereits minimal. Die feste Tafel mit Laufschiene, Rahmen und Hängespalt ist eine Scheibe auf der Wandseite, der Flügel eine Scheibe auf der Raumseite, die weiter mit ihrem Knochen gleitet, die Ablage der Balken 60 x 80 mm am Fuß, maßgetreu."),
        text("Thermal: the writing face as one enamel sheet inside a 75 mm frame band, the leaf face as a second sheet, the tray. Structure: each frame as one ring of 75 mm bars, the pin and the roller support as blocks behind the board at the hanging rail and at the foot, the tray bar.",
             "Thermik: die Schreibfläche als ein Emailblech in einem Rahmenband von 75 mm, die Flügelfläche als zweites Blech, die Ablage. Tragwerk: jeder Rahmen als ein Ring aus 75-mm-Stäben, Fest- und Rollenlager als Blöcke hinter der Tafel an der Hängeschiene und am Fuß, der Ablagebalken."),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def masses():
    """⚖️ The masses of the three bones from the modelled parts: (kg, centre X, centre Y) per bone, world coordinates in pose leaf-left."""
    rho = K.RHO
    panel = K.T_ENAMEL * rho[ENAMEL] + K.T_CORE * rho[MDF]
    bar = K.section_area(*K.edge_profile()) * rho[ALU]
    corner = BAR * BAR * K.BAR_D * rho[PA] * 0.6
    parts = {FR: [], LF: [], TY: []}
    fh = FIX_Y[1] - FIX_Y[0]
    for name, x0, x1 in FIX_PANELS:
        parts[FR].append(((x1 - x0) * (fh - 2 * K.POCKET) * panel, (x0 + x1) / 2, (FIX_Y[0] + FIX_Y[1]) / 2))
    parts[FR].append((bar * (2 * W - 2 * BAR), 0, FIX_Y[0] + BAR / 2))
    parts[FR].append((bar * (2 * W - 2 * BAR), 0, FIX_Y[1] - BAR / 2))
    parts[FR].append((2 * bar * (fh - 2 * BAR), 0, (FIX_Y[0] + FIX_Y[1]) / 2))
    parts[FR].append((K.area(K.mullion_profile()) * (fh - 2 * BAR) * rho[ALU], 0, (FIX_Y[0] + FIX_Y[1]) / 2))
    parts[FR].append((4 * corner, 0, (FIX_Y[0] + FIX_Y[1]) / 2))
    parts[FR].append((K.section_area(*K.header_profile()) * (2 * W - 2 * K.CAP) * rho[ALU], 0, (K.Y_BOARD_TOP + K.Y_TOP) / 2))
    parts[FR].append(((K.area(K.wall_cleat_profile()) + K.area(K.board_cleat_profile())) * 2 * K.CLEAT_X * rho[ALU], 0, 1.2))
    parts[FR].append((0.15, 0, 0.6))
    lh = LEAF_Y[1] - LEAF_Y[0]
    lx = (LEAF_X[0] + LEAF_X[1]) / 2
    ly = (LEAF_Y[0] + LEAF_Y[1]) / 2
    parts[LF].append(((LEAF_X[1] - LEAF_X[0] - 2 * K.POCKET) * (lh - 2 * K.POCKET) * panel, lx, ly))
    parts[LF].append((2 * bar * (LEAF_X[1] - LEAF_X[0] - 2 * BAR) + 2 * bar * (lh - 2 * BAR) + 4 * corner, lx, ly))
    parts[LF].append((0.30, lx, 1.23))
    parts[LF].append((0.16, lx, 0.416))
    parts[TY].append((K.section_area(*K.tray_profile()) * (2 * W - 2 * K.CAP) * rho[ALU], 0, 0.02))
    parts[TY].append((1.0 * 0.04 * 0.006 * rho[WOOD] + 0.03 + 3 * 0.012 + 0.14 * 0.036 * 0.045 * rho[SPONGE], -0.1, 0.01))
    out = {}
    for bone, rows in parts.items():
        kg = sum(r[0] for r in rows)
        out[bone] = (kg, sum(r[0] * r[1] for r in rows) / kg, sum(r[0] * r[2] for r in rows) / kg)
    return out


def volumes():
    return {"50": scale50(), "100": scale100(), "200": scale200(), "500": scale500()}


if __name__ == "__main__":
    import sys
    path = os.path.join(T, "volumes", "objects", "chalkboard.json")
    if "--only50" in sys.argv:
        out = json.load(open(path, encoding="utf-8"))
        out["volumes"]["50"] = scale50()
    else:
        out = {"volumes": volumes()}
    with open(path, "w", encoding="utf-8") as f:
        json.dump(out, f, ensure_ascii=False, indent=1)
        f.write("\n")
    for k, v in out["volumes"].items():
        print(k, {l: len(v[l]) for l in ("body", "thermal", "structure")})
    print({b: tuple(round(c, 3) for c in m) for b, m in masses().items()})
