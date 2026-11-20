"""🖍️ Round 3 chalkboard: the root fixes outside `volumes` (rig origins, anchors, masses, materials, references, texts and the 2D drawings of every scale as the true views of that scale's body), applied to the catalog asset.

Usage: python3 r3-chalkboard-roots.py            writes the candidate document (with the volumes source) to 🗑️generated/r3-chalkboard/🔣️.json
       python3 r3-chalkboard-roots.py --apply    writes the changed members into the catalog file (re-read right before, only these members change)
       --area=<region>:<m2>                      sets a thermal region area (C16, from the 1:50 model)
"""
import copy
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))


def load(name, file):
    spec = importlib.util.spec_from_file_location(name, os.path.join(T, file))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


K = load("kit", "r3-chalkboard-kit.py")
V = load("vol", "r3-chalkboard-volumes.py")
shape, rect, poly, disc, text = K.shape, K.rect, K.poly, K.disc, K.text
FR, TY, LF = K.FR, K.TY, K.LF
ALU, ENAMEL, PA, CHALK, SPONGE, WOOD = K.ALU, K.ENAMEL, K.PA, K.CHALK, K.SPONGE, K.WOOD
W, BAR = K.HALF_W, K.BAR
MEMBERS = ["summary", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "thermal", "structure"]
AREAS = {}
MASS = V.masses()
KG = {b: round(m[0], 1) for b, m in MASS.items()}
TOTAL_KG = round(sum(KG.values()), 1)
FIX_Y = (K.Y_TRAY, K.Y_BOARD_TOP)
LEAF_Y = (K.Y_LEAF0, K.Y_LEAF1)
T_FRAME, T_LEAF, T_HEAD, T_TRAY, T_FACE, T_LEAF_FACE, T_PA, T_GRIP = 0.75, 0.85, 0.5, 0.65, 0.3, 0.24, 0.3, 0.4


def local(bone):
    ox, oy = K.ORIGIN[bone]
    return lambda x0, y0, x1, y1: rect(x0 - ox, y0 - oy, x1 - ox, y1 - oy)


def local_points(bone, points):
    ox, oy = K.ORIGIN[bone]
    return [[x - ox, y - oy] for x, y in points]


def corners(prefix, bone, z, x, y, region):
    """🧷 The four corner pieces of a frame as dark squares."""
    r = local(bone)
    out = []
    for name, cx, cy in (("bl", x[0], y[0]), ("br", x[1] - BAR, y[0]), ("tr", x[1] - BAR, y[1] - BAR), ("tl", x[0], y[1] - BAR)):
        out.append(shape(f"{prefix}-corner-{name}", bone, "detail", z, True, T_PA, r(cx, cy, cx + BAR, cy + BAR), region=region, material=PA))
    return out


def stroke(i, bone, z, points, region, pen=None):
    return shape(i, bone, "detail", z, False, 0.3, poly(local_points(bone, points), False), region=region, pen=pen)


def elevation50():
    f, t, l = local(FR), local(TY), local(LF)
    cap = W - K.CAP
    hx0, hx1 = K.HANDLE_X
    r = K.HANDLE_R
    out = [
        shape("frame", FR, "body", 3, True, T_FRAME, poly(local_points(FR, K.rounded_rect(-W, FIX_Y[0], W, FIX_Y[1], K.R_CORNER, 2))), region="frame", material=ALU),
        shape("face-left", FR, "body", 3.1, True, T_FACE, f(-W + BAR, FIX_Y[0] + BAR, -K.MULLION, FIX_Y[1] - BAR), region="board-face", material=ENAMEL),
        shape("face-right", FR, "body", 3.1, True, T_FACE, f(K.MULLION, FIX_Y[0] + BAR, W - BAR, FIX_Y[1] - BAR), region="board-face", material=ENAMEL),
    ]
    out += corners("frame", FR, 3.05, (-W, W), FIX_Y, "frame")
    out += [
        shape("top-rail", FR, "body", 3.7, True, T_HEAD, f(-W, K.Y_BOARD_TOP, W, K.Y_TOP), region="frame", material=ALU),
        stroke("rail-track", FR, 3.8, [[-cap, K.Y_TOP - 0.013], [cap, K.Y_TOP - 0.013]], "frame"),
        stroke("rail-cap-left", FR, 3.8, [[-cap, K.Y_BOARD_TOP], [-cap, K.Y_TOP]], "frame"),
        stroke("rail-cap-right", FR, 3.8, [[cap, K.Y_BOARD_TOP], [cap, K.Y_TOP]], "frame"),
        shape("tray", TY, "body", 3.2, True, T_TRAY, t(-W, 0, W, K.Y_TRAY), region="tray", material=ALU),
        stroke("tray-lip", TY, 3.25, [[-cap, K.Y_LIP], [cap, K.Y_LIP]], "tray"),
        stroke("tray-cap-left", TY, 3.25, [[-cap, 0], [-cap, K.Y_TRAY]], "tray"),
        stroke("tray-cap-right", TY, 3.25, [[cap, 0], [cap, K.Y_TRAY]], "tray"),
        shape("sponge", TY, "detail", 3.3, True, 0.6, t(0.90, K.Y_LIP, 1.04, K.T_TRAY + 0.036), region="tray", material=SPONGE),
        shape("ruler", TY, "detail", 3.3, True, 0.55, t(-0.255, K.Y_LIP, -0.145, K.T_TRAY + 0.028), region="tray", material=WOOD),
        shape("leaf-frame", LF, "body", 4, True, T_LEAF, poly(local_points(LF, K.rounded_rect(-W, LEAF_Y[0], 0, LEAF_Y[1], K.R_CORNER, 2))), region="leaf", material=ALU),
        shape("leaf-face", LF, "body", 4.1, True, T_LEAF_FACE, l(-W + BAR, LEAF_Y[0] + BAR, -BAR, LEAF_Y[1] - BAR), region="leaf", material=ENAMEL),
    ]
    out += corners("leaf", LF, 4.05, (-W, 0), LEAF_Y, "leaf")
    ox, oy = K.ORIGIN[LF]
    for name, hx in (("a", hx0), ("b", hx1)):
        out.append(shape(f"leaf-grip-{name}", LF, "detail", 4.2, True, T_GRIP, {"kind": "capsule", "x0": K.r5(hx - ox), "y0": K.r5(K.HANDLE_Y[0] - oy), "x1": K.r5(hx - ox), "y1": K.r5(K.HANDLE_Y[1] - oy), "r0": r, "r1": r}, region="leaf", material=ALU))
    for name, hx in (("a", K.HANGERS[0]), ("b", K.HANGERS[1])):
        out.append(shape(f"hanger-{name}", LF, "detail", 3.9, True, 0.2, l(hx - 0.0075, LEAF_Y[1], hx + 0.0075, K.Y_BOARD_TOP), region="leaf", material=ALU))
    return out


def front50():
    env = K.tray_envelope()
    hy0, hy1 = K.HANDLE_Y
    r = K.HANDLE_R
    zg = K.Z_GRIP
    bow = [[K.ZL0, hy0 - r], [zg, hy0 - r], [zg - r, hy0], [zg - r, hy1], [zg, hy1 + r], [K.ZL0, hy1 + r], [K.ZL0, hy1 - r], [zg + r, hy1 - r], [zg + r, hy0 + r], [K.ZL0, hy0 + r]]
    return [
        shape("end-tray", TY, "body", 3.2, True, T_TRAY, poly([[z, y] for y, z in env]), region="tray", material=ALU),
        shape("end-sponge", TY, "detail", 3.1, True, 0.6, rect(-0.037, K.Y_LIP, 0.008, K.T_TRAY + 0.036), region="tray", material=SPONGE),
        shape("end-frame", FR, "body", 3, True, T_FRAME, rect(K.ZF0, FIX_Y[0], K.ZF1, FIX_Y[1]), region="frame", material=ALU),
        shape("end-rail", FR, "body", 3.7, True, T_PA, poly([[z, y] for y, z in K.header_envelope()]), region="frame", material=PA),
        shape("end-cleat", FR, "detail", 2.9, True, T_HEAD, rect(K.ZF1, 1.150, K.Z_WALL, 1.245), region="frame", material=ALU),
        shape("end-buffer", FR, "detail", 2.9, True, T_PA, rect(K.ZF1, K.Y_BUFFER - K.BUFFER_R, K.Z_WALL, K.Y_BUFFER + K.BUFFER_R), region="frame", material=PA),
        shape("end-leaf-frame", LF, "body", 2.8, True, T_LEAF, rect(K.ZL0, LEAF_Y[0], K.ZL1, LEAF_Y[1]), region="leaf", material=ALU),
        shape("end-leaf-grip", LF, "detail", 2.7, True, T_GRIP, poly(bow), region="leaf", material=ALU),
        shape("end-hanger", LF, "detail", 2.7, True, 0.2, rect(K.Z_HANG - 0.0075, LEAF_Y[1], K.Z_HANG + 0.0075, K.Y_BOARD_TOP), region="leaf", material=ALU),
    ]


def plan50():
    cap = W - K.CAP
    out = [
        shape("top-tray", TY, "body", 2.5, True, T_TRAY, rect(-W, K.ZT0, W, K.ZH0 + 0.004), region="tray", material=ALU),
        shape("top-tray-lip", TY, "detail", 2.55, False, 0.3, poly([[-cap, K.ZT0 + K.T_TRAY], [cap, K.ZT0 + K.T_TRAY]], False), region="tray"),
        shape("top-frame", FR, "body", 3, True, T_HEAD, rect(-W, K.ZH0, W, K.ZF1), region="frame", material=ALU),
        shape("top-cleat", FR, "body", 2.9, True, T_HEAD, rect(-K.CLEAT_X, K.ZF1, K.CLEAT_X, K.Z_WALL), region="frame", material=ALU),
        shape("chalk-a", TY, "detail", 2.6, True, 0.9, rect(-1.30, -0.036, -1.22, -0.026), region="tray", material=CHALK),
        shape("chalk-b", TY, "detail", 2.6, True, 0.85, rect(-1.19, -0.0345, -1.11, -0.0245), region="tray", material=CHALK),
        shape("chalk-c", TY, "detail", 2.6, True, 0.8, rect(-1.06, -0.037, -1.008, -0.027), region="tray", material=CHALK),
        shape("top-sponge", TY, "detail", 2.6, True, 0.6, rect(0.90, -0.037, 1.04, K.ZH0), region="tray", material=SPONGE),
        shape("top-ruler", TY, "detail", 2.6, True, 0.55, rect(-0.7, -0.035, 0.3, K.ZH0), region="tray", material=WOOD),
    ]
    for name, hx in (("a", K.HANDLE_X[0]), ("b", K.HANDLE_X[1])):
        out.append(shape(f"top-leaf-grip-{name}", LF, "detail", 2.8, True, T_GRIP, rect(hx - K.HANDLE_R, K.ZT0, hx + K.HANDLE_R, K.ZH0), region="leaf", material=ALU))
    return out


def geometry100():
    f, t, l = local(FR), local(TY), local(LF)
    ly = FIX_Y
    elevation = [
        shape("frame", FR, "body", 3, True, T_FRAME, f(-W, FIX_Y[0], W, FIX_Y[1]), region="frame", material=ALU),
        shape("face-left", FR, "body", 3.1, True, T_FACE, f(-W + BAR, FIX_Y[0] + BAR, -K.MULLION, FIX_Y[1] - BAR), region="board-face", material=ENAMEL),
        shape("face-right", FR, "body", 3.1, True, T_FACE, f(K.MULLION, FIX_Y[0] + BAR, W - BAR, FIX_Y[1] - BAR), region="board-face", material=ENAMEL),
        shape("top-rail", FR, "body", 3.7, True, T_HEAD, f(-W, K.Y_BOARD_TOP, W, K.Y_TOP), region="frame", material=ALU),
        shape("tray", TY, "body", 3.2, True, T_TRAY, t(-W, 0, W, K.Y_TRAY), region="tray", material=ALU),
        shape("sponge", TY, "body", 3.3, True, 0.6, t(0.90, K.Y_LIP, 1.04, K.T_TRAY + 0.036), region="tray", material=SPONGE),
        shape("leaf-frame", LF, "body", 4, True, T_LEAF, l(-W, ly[0], 0, ly[1]), region="leaf", material=ALU),
        shape("leaf-face", LF, "body", 4.1, True, T_LEAF_FACE, l(-W + BAR, ly[0] + BAR, -BAR, ly[1] - BAR), region="leaf", material=ENAMEL),
    ]
    front = [
        shape("end-tray", TY, "body", 3.2, True, T_TRAY, poly([[K.ZT0, 0], [K.ZT1, 0], [K.ZT1, K.Y_TRAY], [K.Z_BOX, K.Y_TRAY], [K.Z_BOX, K.Y_LIP], [K.ZT0, K.Y_LIP]]), region="tray", material=ALU),
        shape("end-sponge", TY, "body", 3.1, True, 0.6, rect(-0.037, K.Y_LIP, 0.008, K.T_TRAY + 0.036), region="tray", material=SPONGE),
        shape("end-frame", FR, "body", 3, True, T_FRAME, rect(K.ZF0, FIX_Y[0], K.ZF1, FIX_Y[1]), region="frame", material=ALU),
        shape("end-rail", FR, "body", 3.7, True, T_HEAD, rect(K.ZH0, K.Y_BOARD_TOP, K.Z_WALL, K.Y_TOP), region="frame", material=ALU),
        shape("end-leaf-frame", LF, "body", 2.8, True, T_LEAF, rect(K.ZL0, ly[0], K.ZL1, ly[1]), region="leaf", material=ALU),
    ]
    plan = [
        shape("top-tray", TY, "body", 2.5, True, T_TRAY, rect(-W, K.ZT0, W, K.ZH0 + 0.004), region="tray", material=ALU),
        shape("top-sponge", TY, "body", 2.6, True, 0.6, rect(0.90, -0.037, 1.04, K.ZH0), region="tray", material=SPONGE),
        shape("top-frame", FR, "body", 3, True, T_HEAD, rect(-W, K.ZH0, W, K.Z_WALL), region="frame", material=ALU),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def geometry200():
    f, t, l = local(FR), local(TY), local(LF)
    both = ["frame", "board-face"]
    elevation = [
        shape("frame", FR, "body", 3, True, 0.5, f(-W, FIX_Y[0], W, FIX_Y[1]), regions=both, material=ENAMEL),
        shape("top-rail", FR, "body", 3.7, True, 0.6, f(-W, K.Y_BOARD_TOP, W, K.Y_TOP), region="frame", material=ALU),
        shape("tray", TY, "body", 3.2, True, T_TRAY, t(-W, 0, W, K.Y_TRAY), region="tray", material=ALU),
        shape("leaf", LF, "body", 4, True, 0.35, l(-W, FIX_Y[0], 0, FIX_Y[1]), region="leaf", material=ENAMEL),
    ]
    front = [
        shape("end-tray", TY, "body", 3.2, True, T_TRAY, poly([[K.ZT0, 0], [K.Z_WALL, 0], [K.Z_WALL, K.Y_TRAY], [K.Z_BOX, K.Y_TRAY], [K.Z_BOX, K.Y_LIP], [K.ZT0, K.Y_LIP]]), region="tray", material=ALU),
        shape("end-frame", FR, "body", 3, True, 0.5, rect(K.ZL1, FIX_Y[0], K.Z_WALL, FIX_Y[1]), regions=both, material=ENAMEL),
        shape("end-rail", FR, "body", 3.7, True, 0.6, rect(K.ZH0, K.Y_BOARD_TOP, K.Z_WALL, K.Y_TOP), region="frame", material=ALU),
        shape("end-leaf", LF, "body", 2.8, True, 0.35, rect(K.ZL1 - 0.024, FIX_Y[0], K.ZL1, FIX_Y[1]), region="leaf", material=ENAMEL),
    ]
    plan = [
        shape("top-tray", TY, "body", 2.5, True, T_TRAY, rect(-W, K.ZT0, W, K.ZH0 + 0.004), region="tray", material=ALU),
        shape("top-frame", FR, "body", 3, True, 0.6, rect(-W, K.ZH0, W, K.Z_WALL), region="frame", material=ALU),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def geometry500():
    f, l = local(FR), local(LF)
    both = ["frame", "board-face"]
    ox, oy = K.ORIGIN[TY]
    elevation = [
        shape("frame", FR, "body", 3, True, 0.5, f(-W, K.Y_TRAY, W, K.Y_TOP), regions=both, material=ENAMEL),
        shape("tray", TY, "body", 3.2, True, T_TRAY, local(TY)(-W, 0, W, K.Y_TRAY), region="tray", material=ALU),
        shape("leaf", LF, "body", 4, True, 0.35, l(-W, K.Y_TRAY, 0, K.Y_BOARD_TOP), region="leaf", material=ENAMEL),
    ]
    front = [
        shape("end-tray", TY, "body", 2.6, True, T_TRAY, rect(K.ZT0, 0, K.Z_WALL, K.Y_TRAY), region="tray", material=ALU),
        shape("end-frame", FR, "body", 3, True, 0.5, rect(0, K.Y_TRAY, K.Z_WALL, K.Y_TOP), regions=both, material=ENAMEL),
        shape("end-leaf", LF, "body", 2.8, True, 0.35, rect(K.ZT0, K.Y_TRAY, 0, K.Y_BOARD_TOP), region="leaf", material=ENAMEL),
    ]
    plan = [
        shape("top-tray", TY, "body", 2.5, True, T_TRAY, rect(-W, K.ZT0, W, K.Z_WALL), region="tray", material=ALU),
        shape("top-frame", FR, "body", 3, True, 0.5, rect(-W, 0, W, K.Z_WALL), regions=both, material=ENAMEL),
        shape("top-leaf", LF, "body", 4, True, 0.35, rect(-W, K.ZT0, 0, 0), region="leaf", material=ENAMEL),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


DECISIONS = {
    "50": {
        "silhouette": text("A 4.0 m wide, 1.26 m high sign, 80 mm by 25 mm on paper: two dark panels in a light aluminium frame under the header track, the chalk tray below and a second framed panel hanging in front of the left half. Front: the end of the board from the tray end cap with the sponge behind it over frame stile, leaf stile with its bow handle and hanger to the header end cap, the hanging cleat and a buffer in the 8 mm gap to the wall; plan: the header track over the tray strip with chalk, ruler and sponge, the hanging cleat behind it and the two handles of the leaf, which itself runs hidden under the track.",
                           "Ein 4,0 m breites und 1,26 m hohes Zeichen, auf Papier 80 mm mal 25 mm: zwei dunkle Platten in einem hellen Aluminiumrahmen unter der Laufschiene, darunter die Kreideablage und davor eine zweite gerahmte Platte, die vor der linken Hälfte hängt. Vorderansicht: die Stirnseite der Tafel von der Ablagen-Endkappe mit dem Schwamm dahinter über Rahmenpfosten, Flügelpfosten mit Bügelgriff und Hänger bis zur Endkappe der Laufschiene, die Hängeleiste und ein Puffer im 8-mm-Spalt zur Wand; Draufsicht: die Laufschiene über dem Ablagestreifen mit Kreide, Lineal und Schwamm, dahinter die Hängeleiste und die zwei Griffe des Flügels, der selbst verdeckt unter der Schiene läuft."),
        "retained": text("Overall 4.0 m by 1.26 m: the 0.06 m tray with its 18 mm lip line and end caps, the fixed board of 1.162 m with its 35 mm frame, 30 mm mullion and four dark corner pieces, the 38 mm header track with its soft-edge line and end caps, the leaf of 2.0 m by 1.146 m with its own frame and corner pieces running 8 mm clear of tray and track, its two hanger nuts in the upper gap and the two bow handles on its stiles, the sponge and the ruler handle standing above the tray lip.",
                         "Gesamt 4,0 m mal 1,26 m: die 0,06 m hohe Ablage mit der Linie ihrer 18-mm-Lippe und Endkappen, die feste Tafel von 1,162 m mit 35-mm-Rahmen, 30-mm-Mittelsteg und vier dunklen Eckstücken, die 38 mm hohe Laufschiene mit der Linie ihrer weichen Kante und Endkappen, der Flügel von 2,0 m mal 1,146 m mit eigenem Rahmen und Eckstücken, der 8 mm frei über Ablage und unter der Schiene läuft, seine zwei Hängermuttern im oberen Spalt und die zwei Bügelgriffe auf seinen Pfosten, der Schwamm und der Linealgriff, die über die Ablagelippe ragen."),
        "merged": text("The two back panels share one frame shape with the mullion; the profile chambers and the panel pockets are not resolved.",
                       "Die beiden Rückplatten teilen sich eine Rahmenform mit dem Mittelsteg; Profilkammern und Plattentaschen sind nicht aufgelöst."),
        "omitted": text("The trolleys inside the track, the glides in the guide slot, the wall screws, every writing on the surface. Chalk and the ruler blade lie behind the tray lip and appear only in plan; the leaf lies under the track in plan.",
                        "Die Laufwagen in der Schiene, die Gleiter im Führungsschlitz, die Wandschrauben, jede Beschriftung auf der Fläche. Kreide und Linealblatt liegen hinter der Ablagelippe und erscheinen nur in der Draufsicht; der Flügel liegt in der Draufsicht unter der Schiene."),
        "thinElements": text("The bow handles are 8 mm rods (0.16 mm on paper), the chalk sticks 10 mm (0.2 mm), the hanging cleat and the buffers fill the 8 mm wall gap (0.16 mm); the 8 mm running gaps of the leaf stay open.",
                             "Die Bügelgriffe sind 8-mm-Stäbe (0,16 mm auf Papier), die Kreidestücke 10 mm (0,2 mm), Hängeleiste und Puffer füllen den 8-mm-Wandspalt (0,16 mm); die 8-mm-Laufspalte des Flügels bleiben offen."),
        "hatching": text("None; the dark panel is a tone fill.", "Keine; die dunkle Tafel ist eine Tonfüllung."),
        "styles": text("Panels 0.24 to 0.3, frames 0.75 to 0.85, header track 0.5, tray 0.65, corner pieces and caps 0.3, chalk 0.8 to 0.9, sponge 0.6, ruler 0.55; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.",
                       "Tafeln 0,24 bis 0,3, Rahmen 0,75 bis 0,85, Laufschiene 0,5, Ablage 0,65, Eckstücke und Kappen 0,3, Kreide 0,8 bis 0,9, Schwamm 0,6, Lineal 0,55; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."),
        "overlays": text("Back panels, frame with header track, tray and sliding leaf are four regions: the thermal view shows how the leaf shades half of the back panels.",
                         "Rückplatten, Rahmen mit Laufschiene, Ablage und Schiebeflügel sind vier Regionen: die Thermalansicht zeigt, wie der Flügel die Hälfte der Rückplatten abdeckt."),
    },
    "100": {
        "silhouette": text("A 40 mm by 12.6 mm sign: a framed dark panel under the header track with a tray strip and a framed leaf over its left half. Front: tray end, frame end, the leaf in front of it and the header reaching back to the wall; plan: the header track over the front strip of the tray.",
                           "Ein Zeichen von 40 mm mal 12,6 mm: eine gerahmte dunkle Tafel unter der Laufschiene mit Ablagestreifen und ein gerahmter Flügel über der linken Hälfte. Vorderansicht: Ablagenende, Rahmenende, der Flügel davor und die Laufschiene, die bis zur Wand zurückreicht; Draufsicht: die Laufschiene über dem vorderen Streifen der Ablage."),
        "retained": text("Board 4.0 m by 1.162 m with its 35 mm frame and mullion, the 38 mm header track, the tray of 0.06 m with the sponge, the leaf with its frame. Leaf and frame stay separate shapes bound to their own bones, so the elevation follows every leaf pose and clip.",
                         "Tafel 4,0 m mal 1,162 m mit 35-mm-Rahmen und Mittelsteg, die 38 mm hohe Laufschiene, die Ablage von 0,06 m mit dem Schwamm, der Flügel mit seinem Rahmen. Flügel und Rahmen bleiben getrennte Formen an ihren eigenen Knochen, die Ansicht folgt daher jeder Flügelpose und jedem Clip."),
        "merged": text("The 8 mm running gaps of the leaf are closed (0.08 mm on paper): the leaf reaches from the tray to the header. The hanging cleats join the back of the header; corner pieces join the frame.",
                       "Die 8-mm-Laufspalte des Flügels sind geschlossen (0,08 mm auf Papier): der Flügel reicht von der Ablage bis zur Laufschiene. Die Hängeleisten gehen im Rücken der Laufschiene auf, die Eckstücke im Rahmen."),
        "omitted": text("Chalk, ruler, bow handles, hangers, buffers, the tray lip line. In plan the leaf runs hidden under the header.",
                        "Kreide, Lineal, Bügelgriffe, Hänger, Puffer, die Linie der Ablagelippe. In der Draufsicht läuft der Flügel verdeckt unter der Laufschiene."),
        "thinElements": text("The 35 mm frame is 0.35 mm on paper; frame end and leaf end are 20 mm strips (0.2 mm), the tray strip in plan 27.5 mm (0.28 mm).",
                             "Der 35-mm-Rahmen ist auf Papier 0,35 mm breit; Rahmenende und Flügelende sind Streifen von 20 mm (0,2 mm), der Ablagestreifen in der Draufsicht 27,5 mm (0,28 mm)."),
        "hatching": text("None.", "Keine."),
        "styles": text("Panels 0.24 to 0.3, frames 0.75 to 0.85, header 0.5, tray 0.65, sponge 0.6; one body pen of 0.18 mm.", "Tafeln 0,24 bis 0,3, Rahmen 0,75 bis 0,85, Laufschiene 0,5, Ablage 0,65, Schwamm 0,6; ein Körperstift von 0,18 mm."),
        "overlays": text("Every part keeps its region, so the leaf and the back panels can differ in the thermal view.", "Jedes Teil behält seine Region, sodass sich Flügel und Rückplatten in der Thermalansicht unterscheiden können."),
    },
    "200": {
        "silhouette": text("A 20 mm by 6.3 mm sign: a dark rectangle under the header bar on a thin tray strip with the sliding leaf as a second rectangle in front of the left half. Front: the end of the board (stepped tray, board slab to the wall, leaf in front, header over both); plan: the header bar over the front strip of the tray.",
                           "Ein Zeichen von 20 mm mal 6,3 mm: ein dunkles Rechteck unter dem Laufschienenbalken auf einem dünnen Ablagestreifen, davor der Schiebeflügel als zweites Rechteck vor der linken Hälfte. Vorderansicht: die Stirnseite der Tafel (gestufte Ablage, Tafelscheibe bis zur Wand, Flügel davor, Laufschiene über beiden); Draufsicht: der Laufschienenbalken über dem vorderen Streifen der Ablage."),
        "retained": text("Board 4.0 m by 1.162 m, the 38 mm header bar, the tray strip of 0.06 m and the sliding leaf of 2.0 m.", "Tafel 4,0 m mal 1,162 m, der 38 mm hohe Laufschienenbalken, der Ablagestreifen von 0,06 m und der Schiebeflügel von 2,0 m."),
        "merged": text("Frame and back panels are one rectangle (regions frame and board-face merged) that reaches the wall plane; the leaf frame and leaf face are one rectangle from tray to header.",
                       "Rahmen und Rückplatten sind ein Rechteck (Regionen Rahmen und Tafelfläche zusammengeführt) bis zur Wandebene; Flügelrahmen und Flügelfläche sind ein Rechteck von der Ablage bis zur Laufschiene."),
        "omitted": text("The mullion, handles, hangers, chalk, sponge and tray lip. In plan the leaf runs hidden under the header.", "Der Mittelsteg, Griffe, Hänger, Kreide, Schwamm und die Ablagelippe. In der Draufsicht läuft der Flügel verdeckt unter der Laufschiene."),
        "thinElements": text("The header bar is 0.19 mm and the tray 0.3 mm high on paper; the leaf end is thickened from 20 to 24 mm (0.12 mm plus its outline) so that it prints.", "Der Laufschienenbalken ist auf Papier 0,19 mm und die Ablage 0,3 mm hoch; das Flügelende ist von 20 auf 24 mm verdickt (0,12 mm plus Umriss), damit es druckbar ist."),
        "hatching": text("None.", "Keine."),
        "styles": text("Back board 0.5, leaf 0.35, header 0.6, tray 0.65; one body pen of 0.18 mm.", "Rückplatte 0,5, Flügel 0,35, Laufschiene 0,6, Ablage 0,65; ein Körperstift von 0,18 mm."),
        "overlays": text("The rectangle lists frame and board face as merged regions; the header, the leaf and the tray stay separate.", "Das Rechteck führt Rahmen und Tafelfläche als zusammengeführte Regionen; Laufschiene, Flügel und Ablage bleiben getrennt."),
    },
    "500": {
        "silhouette": text("Sliding blackboard sign 8.0 x 2.5 mm: the 4 m wall board as one long rectangle with the 2 m leaf as a second rectangle on top that slides left and right with the leaf bone; the side view is the board face, the front view the 0.08 m end of tray, board and leaf, the plan the 0.16 mm thin top edge with the leaf strip.",
                           "Zeichen der Schiebetafel 8,0 x 2,5 mm: die 4 m lange Wandtafel als ein langes Rechteck mit dem 2-m-Flügel als zweitem Rechteck darüber, das mit dem Flügelknochen nach links und rechts gleitet; die Seitenansicht ist die Tafelfläche, die Vorderansicht das 0,08 m dicke Ende aus Ablage, Tafel und Flügel, die Draufsicht die 0,16 mm dünne Oberkante mit dem Flügelstreifen."),
        "retained": text("Overall width 4 m, height 1.26 m, the tray as its own band at the foot, the sliding leaf with its real travel of 2 m under the header.", "Gesamtbreite 4 m, Höhe 1,26 m, die Ablage als eigener Streifen am Fuß, der Schiebeflügel mit seinem echten Weg von 2 m unter der Laufschiene."),
        "merged": text("Frame, board face, header track and hanging gap are one rectangle on the wall half of the depth; the leaf is thickened to the room half.", "Rahmen, Tafelfläche, Laufschiene und Hängespalt sind ein Rechteck auf der Wandhälfte der Tiefe; der Flügel ist auf die Raumhälfte verdickt."),
        "omitted": text("Hanging rail, buffers, chalk, frame profile, handles and hangers (all below 0.1 m).", "Hängeschiene, Puffer, Kreide, Rahmenprofil, Griffe und Hänger (alle unter 0,1 m)."),
        "thinElements": text("The 0.08 m depth is 0.16 mm on paper, drawn as two 0.08 mm strips in front and plan; the 0.06 m tray is a 0.12 mm band in the side view.", "Die Tiefe von 0,08 m ist 0,16 mm auf Papier, in Vorder- und Draufsicht als zwei Streifen von 0,08 mm gezeichnet; die 0,06 m hohe Ablage ist in der Seitenansicht ein Streifen von 0,12 mm."),
        "hatching": text("None.", "Keine."),
        "styles": text("Board 0.5, leaf 0.35, tray 0.65; one 0.18 mm outline.", "Tafel 0,5, Flügel 0,35, Ablage 0,65; eine 0,18-mm-Kontur."),
        "overlays": text("Thermal regions as at 1:200: frame and board face share one shape, the leaf and the tray stay separate so the leaf moves with its bone.", "Thermische Regionen wie bei 1:200: Rahmen und Tafelfläche teilen eine Form, Flügel und Ablage bleiben getrennt, damit sich der Flügel mit seinem Knochen bewegt."),
    },
}


def apply(asset):
    asset["summary"] = text(
        "A wall-mounted classroom chalkboard 4.0 m wide and 1.26 m high: two fixed enamelled panels in an aluminium frame under a header track, a 2.0 m leaf that hangs on two trolleys and slides in front of them, and a chalk tray below, shown in front elevation; a passive surface that warms in the sun.",
        "Eine an der Wand montierte Klassenraumtafel von 4,0 m Breite und 1,26 m Höhe: zwei feste emaillierte Platten in einem Aluminiumrahmen unter einer Laufschiene, ein 2,0 m breiter Flügel, der an zwei Laufwagen hängt und davor gleitet, und darunter eine Kreideablage, in Vorderansicht; eine passive Fläche, die sich in der Sonne erwärmt.")
    refs = asset["provenance"]["references"]
    if not any(r["id"] == "sliding-board-products" for r in refs):
        refs.insert(1, {"id": "sliding-board-products", "citation": "Manufacturers' product data for horizontal sliding writing-board units with a fixed back panel (GMi Ghent Duo-Track, MooreCo Best-Rite horizontal sliding board, Claridge horizontal sliding system, AARCO horizontal sliding unit; consulted 2026-11): porcelain-enamel steel laminated on 12 to 16 mm board, anodised aluminium tube frames, panels on nylon rollers in an aluminium track, full-length tray with enclosed end caps, about 90 mm deep with two planes."})
    asset["provenance"]["notes"] = text(
        "Front elevation of a classroom board of the common 4.0 m class as it is built. Fixed board 4.0 m by 1.162 m: two panels of 0.6 mm ceramic steel on a 16 mm MDF core (16.7 kg per square metre) seated 12 mm deep in a 35 mm by 20 mm anodised aluminium edge profile with moulded corner pieces of 10 mm radius (safety radius of EN 14434) and a 30 mm H mullion. On its top bar a 48.5 mm by 38 mm header track; the leaf of 2.0 m by 1.146 m, built like the fixed board, hangs in it on two four-wheel trolleys 0.3 m from its ends, runs 8 mm clear of track and tray and is guided at the bottom by two glides in a slot of the bottom bar; two bow handles sit on its stiles. Below, a 0.06 m high and 0.08 m deep tray extrusion with an 18 mm lip, a 50 mm trough and the box that carries the board, with end caps. The board hangs on a pair of 2.5 mm Z cleats 8 mm off the wall and leans on five buffers. The origin is the middle of the lower edge of the tray; a call site places it about 0.85 m above the floor. Masses follow from the modelled parts (" + f"{TOTAL_KG:g}" + " kg: fixed board " + f"{KG[FR]:g}" + ", leaf " + f"{KG[LF]:g}" + ", tray " + f"{KG[TY]:g}" + "); the depth of 0.08 m is that of the two-plane product class. The board has no heat source: it exchanges heat with the room and takes the sun that falls on it.",
        "Vorderansicht einer Klassenraumtafel der üblichen 4,0-m-Klasse, wie sie gebaut ist. Feste Tafel 4,0 m mal 1,162 m: zwei Platten aus 0,6 mm Keramikstahl auf einem 16 mm starken MDF-Kern (16,7 kg je Quadratmeter), 12 mm tief in einem eloxierten Aluminium-Kantenprofil 35 mm mal 20 mm mit gespritzten Eckstücken von 10 mm Radius (Sicherheitsradius nach EN 14434) und einem 30 mm breiten H-Mittelsteg. Auf dem oberen Stab eine Laufschiene 48,5 mm mal 38 mm; der Flügel von 2,0 m mal 1,146 m, gebaut wie die feste Tafel, hängt darin an zwei vierrolligen Laufwagen 0,3 m von seinen Enden, läuft 8 mm frei unter der Schiene und über der Ablage und wird unten von zwei Gleitern in einem Schlitz des unteren Stabs geführt; auf seinen Pfosten sitzen zwei Bügelgriffe. Darunter ein 0,06 m hohes und 0,08 m tiefes Ablageprofil mit 18 mm Lippe, 50 mm Mulde und dem Kasten, der die Tafel trägt, mit Endkappen. Die Tafel hängt 8 mm vor der Wand an einem Paar 2,5 mm starker Z-Leisten und lehnt an fünf Puffern. Der Ursprung ist die Mitte der Unterkante der Ablage; ein Aufrufer setzt ihn etwa 0,85 m über den Boden. Die Massen folgen aus den modellierten Teilen (" + f"{TOTAL_KG:g}".replace(".", ",") + " kg: feste Tafel " + f"{KG[FR]:g}".replace(".", ",") + ", Flügel " + f"{KG[LF]:g}".replace(".", ",") + ", Ablage " + f"{KG[TY]:g}".replace(".", ",") + "); die Tiefe von 0,08 m ist die der zweiebenigen Produktklasse. Die Tafel hat keine Wärmequelle: sie tauscht Wärme mit dem Raum aus und nimmt die Sonne auf, die auf sie fällt.")
    asset["dimensions"]["massKg"] = TOTAL_KG
    have = {m["id"] for m in asset["materials"]}
    if PA not in have:
        asset["materials"].append({"id": PA, "title": text("Polyamide (corner pieces, caps, rollers, buffers)", "Polyamid (Eckstücke, Kappen, Rollen, Puffer)"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 1150, "conductivityWmK": 0.25, "specificHeatJKgK": 1600, "source": "literature", "reference": "iso-10456-2007"})
    if WOOD not in have:
        asset["materials"].append({"id": WOOD, "title": text("Beech wood (board ruler)", "Buchenholz (Tafellineal)"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 720, "conductivityWmK": 0.18, "specificHeatJKgK": 1600, "source": "literature", "reference": "iso-10456-2007"})
    for bone in asset["rig"]["bones"]:
        ox, oy = K.ORIGIN[bone["id"]]
        kg, cx, cy = MASS[bone["id"]]
        bone["mass"]["kg"] = KG[bone["id"]]
        bone["mass"]["comX"] = round(cx - ox, 3) + 0.0
        bone["mass"]["comY"] = round(cy - oy, 3) + 0.0
        if bone["id"] == FR:
            bone["x"] = -2
            bone["title"] = text("Board frame with two back panels and the header track", "Tafelrahmen mit zwei Rückplatten und der Laufschiene")
        if bone["id"] == LF:
            bone["y"] = K.Y_LEAF0
    for anchor in asset["anchors"]:
        anchor["x"] = 2
        if anchor["id"] == "wall-hook":
            anchor["y"], anchor["z"] = 1.2, 0.036
            anchor["title"] = text("Hanging cleat on the wall behind the top bar", "Hängeleiste an der Wand hinter dem oberen Stab")
        else:
            anchor["y"], anchor["z"] = K.Y_BUFFER, 0.036
            anchor["title"] = text("Distance buffers on the back of the bottom bar", "Abstandspuffer an der Rückseite des unteren Stabs")
        for key in ("title", "support", "normalDeg", "footprintLengthM", "footprintWidthM", "frictionCoefficient"):
            if key in anchor:
                anchor[key] = anchor.pop(key)
    reps = asset["representations"]
    reps["50"]["geometry"] = {"elevation": elevation50(), "front": front50(), "plan": plan50()}
    reps["100"]["geometry"] = geometry100()
    reps["200"]["geometry"] = geometry200()
    reps["500"]["geometry"] = geometry500()
    for scale, decisions in DECISIONS.items():
        reps[scale]["decisions"] = decisions
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
        if region["id"] == "frame":
            region["title"] = text("Aluminium frame, mullion and header track", "Aluminiumrahmen, Mittelsteg und Laufschiene")
    asset["structure"]["assumptions"] = [text(
        f"The {TOTAL_KG:g} kg board hangs on the cleat behind its top bar (pin) and leans on buffers at the bottom bar (roller): the cleat carries the weight, the pair resists the overturning moment of the {KG[LF]:g} kg leaf, which hangs on two trolleys in the header track.",
        f"Die {TOTAL_KG:g} kg schwere Tafel hängt an der Leiste hinter ihrem oberen Stab (Festlager) und lehnt unten an Puffern (Rollenlager): die Leiste trägt das Gewicht, das Paar nimmt das Kippmoment des {KG[LF]:g} kg schweren Flügels auf, der an zwei Laufwagen in der Laufschiene hängt.")]


if __name__ == "__main__":
    for arg in sys.argv[1:]:
        if arg.startswith("--area="):
            rid, value = arg[7:].split(":")
            AREAS[rid] = float(value)
    source = open(K.CATALOG, encoding="utf-8").read()
    document = json.loads(source)
    before = copy.deepcopy(document)
    apply(document["asset"])
    print("masses", KG, "total", TOTAL_KG, {b: (round(m[1] - K.ORIGIN[b][0], 3), round(m[2] - K.ORIGIN[b][1], 3)) for b, m in MASS.items()})
    if "--apply" in sys.argv:
        for key in MEMBERS:
            start, end = K.member(source, key)
            source = source[:start] + K.dumped(document["asset"][key]) + source[end:]
        check = json.loads(source)
        for key in check["asset"]:
            if key == "volumes":
                continue
            if key not in MEMBERS and check["asset"][key] != before["asset"][key]:
                raise SystemExit("member %s changed unexpectedly" % key)
            if key in MEMBERS and check["asset"][key] != document["asset"][key]:
                raise SystemExit("member %s not written as intended" % key)
        with open(K.CATALOG, "w", encoding="utf-8") as f:
            f.write(source)
        print("catalog members written:", ", ".join(MEMBERS))
    else:
        document["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes", "objects", "chalkboard.json"), encoding="utf-8"))["volumes"]
        folder = os.path.join(T, "🗑️generated", "r3-chalkboard")
        os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8") as f:
            json.dump(document, f, ensure_ascii=False, indent=2)
        print("candidate written")
