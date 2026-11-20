"""✏️ Round 3, furniture/height-adjustable-desk: the 2D drawings of every scale and view (ruling E1), drawn by hand from the construction in the kit.

Elevation shapes are bone-local (u, v), seen from the user's side (-z); front shapes x = world z, y = rest height, seen from +x (the right
column is in front); plan shapes x = world x, y = world z, seen from above.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-height-adjustable-desk-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
r, T = K.r, K.text

STEEL, PLASTIC, BOARD = "steel-desk", "equipment-housing", "particleboard-laminate"
SIDES = (("l", "left"), ("r", "right"))


def rect(id, bone, z, tone, region, material, x0, x1, y0, y1, radius=0.0, role="body", pen=None, hatch=None):
    """▭ Filled rectangle from its corners."""
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone, "kind": "rect", "x": r(x0), "y": r(y0), "w": r(x1 - x0), "h": r(y1 - y0)}
    if radius:
        s["r"] = radius
    s.update({"region": region, "material": material})
    if hatch:
        s["hatch"] = hatch
    if pen:
        s["penMm"] = pen
    return s


def poly(id, bone, z, tone, region, material, points, closed=True, role="body", pen=None, filled=None):
    """⬠ Polygon or open stroke."""
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": closed if filled is None else filled, "tone": tone, "kind": "poly", "points": [[r(x), r(y)] for x, y in points], "closed": closed, "region": region, "material": material}
    if pen:
        s["penMm"] = pen
    return s


def path(id, bone, z, tone, region, material, rings, role="body", pen=None):
    """⬚ Several disjoint filled rings in one shape."""
    d = " ".join("M " + " L ".join(f"{r(x)} {r(y)}" for x, y in ring) + " Z" for ring in rings)
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone, "kind": "path", "d": d, "region": region, "material": material}
    if pen:
        s["penMm"] = pen
    return s


def elevation(scale):
    """👁️ Side elevation from the user's side: per bone, true outlines of that scale's model."""
    fine, s50 = scale <= 100, scale == 50
    tube_pen = {50: 0.35, 100: 0.25, 200: None, 500: None}[scale]
    small_pen = {50: 0.25, 100: 0.18, 200: None, 500: None}[scale]
    out = []
    U = K.FOOT_U
    for s, side in SIDES:
        foot, stage, column = f"foot-{side}", f"stage-{side}", f"column-{side}"
        if s50:
            out.append(rect(f"{s}-glide", foot, 2.9, 0.12, "steel-desk", PLASTIC, U - K.GLIDE_R, U + K.GLIDE_R, 0.0, 0.011, pen=small_pen))
        out.append(rect(f"{s}-foot", foot, 3.0, 0.3, "steel-desk", STEEL, 0.0, K.FOOT_W, K.FOOT_UNDER if s50 else 0.0, K.FOOT_MID_H, pen=tube_pen))
        if s50:
            out.append(poly(f"{s}-foot-edge", foot, 3.1, 0.3, "steel-desk", STEEL, [(0.003, K.FOOT_END_H), (K.FOOT_W - 0.003, K.FOOT_END_H)], closed=False, role="detail"))
        a = K.SLEEVE[0]
        out.append(rect(f"{s}-sleeve", foot, 2.0, 0.36, "steel-desk", STEEL, U - a / 2, U + a / 2, K.SLEEVE_V[0], K.SLEEVE_V[1] if s50 else K.SLEEVE_COLLAR_V[1], K.SLEEVE[2] if s50 else 0.0, pen=tube_pen))
        if s50:
            a = K.SLEEVE_COLLAR[0]
            out.append(rect(f"{s}-sleeve-collar", foot, 2.1, 0.14, "steel-desk", PLASTIC, U - a / 2, U + a / 2, *K.SLEEVE_COLLAR_V, role="detail", pen=small_pen))
        a = K.STAGE[0]
        out.append(rect(f"{s}-stage", stage, 1.8, 0.48, "steel-desk", STEEL, K.STAGE_U[0], K.STAGE_U[1] if s50 else K.STAGE_COLLAR_U[1], -a / 2, a / 2, pen=tube_pen))
        if s50:
            a = K.STAGE_COLLAR[0]
            out.append(rect(f"{s}-stage-collar", stage, 1.9, 0.14, "steel-desk", PLASTIC, *K.STAGE_COLLAR_U, -a / 2, a / 2, role="detail", pen=small_pen))
        a = K.COLUMN[0]
        out.append(rect(f"{s}-column", column, 1.6, 0.6, "steel-desk", STEEL, *K.COLUMN_U, -a / 2, a / 2, pen=tube_pen))
        a = K.DRIVE[0]
        head = K.DRIVE_U[1] if fine else K.COLUMN_LEN
        out.append(rect(f"{s}-drive", column, 2.2, 0.2, f"drive-{side}", PLASTIC, K.DRIVE_U[0], head, -a / 2, a / 2, K.DRIVE[2] if fine else 0.0, pen=tube_pen))
    L, D, H, R = K.BOARD
    out.append(rect("top-board", "top", 4.0, 0.62, "top-board", BOARD, 0.0, L, 0.0, H, hatch="board-grain" if s50 else None, pen={50: 0.5, 100: 0.35, 200: None, 500: None}[scale]))
    if fine:
        bw, bh, _ = K.BRACKET
        for i, x in enumerate(K.COLUMN_X):
            out.append(rect(f"top-bracket-{('left', 'right')[i]}", "top", 2.5, 0.3, "steel-desk", STEEL, x - bw / 2, x + bw / 2, -bh, 0.0, pen=small_pen))
    out.append(rect("top-crossbar", "top", 2.15, 0.34, "steel-desk", STEEL, K.CROSSBAR_U[0], K.CROSSBAR_U[1], *K.CROSSBAR_V, pen=tube_pen))
    if s50:
        for i, x in enumerate((K.COLUMN_X[0] + K.DRIVE[0] / 2, K.COLUMN_X[1] - K.DRIVE[0] / 2 - K.FLANGE_T)):
            out.append(rect(f"top-crossbar-flange-{('left', 'right')[i]}", "top", 2.17, 0.28, "steel-desk", STEEL, x, x + K.FLANGE_T, *K.FLANGE_V, role="detail", pen=small_pen))
    if scale < 500:
        out.append(rect("top-tray", "top", 1.0, 0.4, "steel-desk", STEEL, K.TRAY_U[0], K.TRAY_U[1], -K.TRAY_DEPTH, 0.0, pen=small_pen))
    (u0, u1), (v0, v1), _ = K.CONTROL
    out.append(rect("top-control", "top", 1.5, 0.22, "controls", PLASTIC, u0, u1, v0, v1, pen=small_pen))
    if s50:
        (u0, u1), (v0, v1), _ = K.HANDSET
        out.append(rect("top-handset", "top", 5.0, 0.12, "controls", PLASTIC, u0, u1, v0, v1, 0.006, pen=small_pen))
    out += monitor_elevation(scale, small_pen)
    (u0, u1), (v0, _), _ = K.LAP_BASE
    out.append(rect("laptop", "top", 4.6, 0.42, "laptop-lid", PLASTIC, u0, u1, v0, K.LAP_LID[1][1], 0.003 if s50 else 0.0, pen=small_pen))
    if s50:
        out.append(poly("laptop-seam", "top", 4.7, 0.3, "laptop-lid", PLASTIC, [(K.LAP_BASE[0][0] + 0.004, K.LAP_BASE[1][1]), (K.LAP_BASE[0][1] - 0.004, K.LAP_BASE[1][1])], closed=False, role="detail"))
    return out


def monitor_elevation(scale, pen):
    """🖥️ Monitor seen from the user: base, the neck below the panel, panel and screen."""
    (pu0, pu1), (pv0, pv1), _ = K.MON_PANEL
    if scale == 500:
        return [rect("monitor-stand", "top", 4.05, 0.28, "monitor-housing", PLASTIC, 0.57, 0.63, K.MON_BASE[1][0], pv0 + 0.01),
                rect("monitor-panel", "top", 4.3, 0.18, "monitor-housing", PLASTIC, pu0, pu1, pv0, pv1)]
    out = [rect("monitor-base", "top", 4.1, 0.26, "monitor-housing", PLASTIC, *K.MON_BASE[0], *K.MON_BASE[1], 0.004 if scale == 50 else 0.0, pen=pen),
           rect("monitor-neck", "top", 4.05, 0.3, "monitor-housing", PLASTIC, *K.MON_NECK[0], *K.MON_NECK[1], pen=pen),
           rect("monitor-panel", "top", 4.3, 0.18, "monitor-housing", PLASTIC, pu0, pu1, pv0, pv1, 0.004 if scale <= 100 else 0.0, pen=pen)]
    if scale <= 100:
        out.append(rect("monitor-screen", "top", 4.4, 0.08, "monitor-housing", PLASTIC, *K.MON_SCREEN[0], *K.MON_SCREEN[1], role="glazing"))
    return out


def front(scale):
    """👁️ Front view from +x: the right column in front of the left one, the board end, rails, tray and controls behind, monitor and laptop edge-on."""
    fine, s50 = scale <= 100, scale == 50
    tube_pen = {50: 0.35, 100: 0.25, 200: None, 500: None}[scale]
    small_pen = {50: 0.25, 100: 0.18, 200: None, 500: None}[scale]
    s, side = "r", "right"
    foot, stage, column = f"foot-{side}", f"stage-{side}", f"column-{side}"
    h, e, m = K.FOOT_L / 2, K.FOOT_END_H, K.FOOT_MID_HALF
    out = []
    if s50:
        g, gw = K.GLIDE_R, K.GLIDE_W
        out.append(path(f"{s}-glide", foot, 2.0, 0.12, "steel-desk", PLASTIC, [[(-gw - g, 0.0), (-gw + g, 0.0), (-gw + g, 0.011), (-gw - g, 0.011)], [(gw - g, 0.0), (gw + g, 0.0), (gw + g, 0.011), (gw - g, 0.011)]], pen=small_pen))
    if scale <= 100:
        y0 = K.FOOT_UNDER if s50 else 0.0
        out.append(poly(f"{s}-foot", foot, 2.1, 0.3, "steel-desk", STEEL, [(-h, y0), (h, y0), (h, e), (m, K.FOOT_MID_H), (-m, K.FOOT_MID_H), (-h, e)], pen=tube_pen))
    else:
        out.append(rect(f"{s}-foot", foot, 2.1, 0.3, "steel-desk", STEEL, -h, h, 0.0, K.FOOT_MID_H))
    b = K.SLEEVE[1]
    out.append(rect(f"{s}-sleeve", foot, 2.2, 0.36, "steel-desk", STEEL, -b / 2, b / 2, K.SLEEVE_V[0], K.SLEEVE_V[1] if s50 else K.SLEEVE_COLLAR_V[1], K.SLEEVE[2] if s50 else 0.0, pen=tube_pen))
    if s50:
        b = K.SLEEVE_COLLAR[1]
        out.append(rect(f"{s}-sleeve-collar", foot, 2.3, 0.14, "steel-desk", PLASTIC, -b / 2, b / 2, *K.SLEEVE_COLLAR_V, role="detail", pen=small_pen))
    y = K.STAGE_Y
    b = K.STAGE[1]
    out.append(rect(f"{s}-stage", stage, 2.15, 0.48, "steel-desk", STEEL, -b / 2, b / 2, y + K.STAGE_U[0], y + (K.STAGE_U[1] if s50 else K.STAGE_COLLAR_U[1]), pen=tube_pen))
    if s50:
        b = K.STAGE_COLLAR[1]
        out.append(rect(f"{s}-stage-collar", stage, 2.25, 0.14, "steel-desk", PLASTIC, -b / 2, b / 2, y + K.STAGE_COLLAR_U[0], y + K.STAGE_COLLAR_U[1], role="detail", pen=small_pen))
    y = K.COLUMN_Y
    b = K.COLUMN[1]
    out.append(rect(f"{s}-column", column, 2.12, 0.6, "steel-desk", STEEL, -b / 2, b / 2, y + K.COLUMN_U[0], y + K.COLUMN_U[1], pen=tube_pen))
    b = K.DRIVE[1]
    head = K.DRIVE_U[1] if fine else K.COLUMN_LEN
    out.append(rect(f"{s}-drive", column, 2.5, 0.2, f"drive-{side}", PLASTIC, -b / 2, b / 2, y + K.DRIVE_U[0], y + head, K.DRIVE[2] if fine else 0.0, pen=tube_pen))
    ty = K.TOP_Y
    L, D, H, R = K.BOARD
    out.append(rect("top-board", "top", 4.0, 0.62, "top-board", BOARD, -D / 2, D / 2, ty, ty + H, hatch="board-grain" if s50 else None, pen={50: 0.5, 100: 0.35, 200: None, 500: None}[scale]))
    if fine:
        bw, bh, bl = K.BRACKET
        out.append(rect("top-bracket-right", "top", 2.6, 0.3, "steel-desk", STEEL, -bl / 2, bl / 2, ty - bh, ty, pen=small_pen))
    if scale < 500:
        w0, w1, f, t = K.TRAY_W[0], K.TRAY_W[1], K.TRAY_FLANGE, 0.004 if s50 else 0.008
        if s50:
            out.append(poly("top-tray", "top", 1.5, 0.4, "steel-desk", STEEL, [(w0 - f, ty), (w1 + f, ty), (w1 + f, ty - t), (w1, ty - t), (w1, ty - K.TRAY_DEPTH), (w0, ty - K.TRAY_DEPTH), (w0, ty - t), (w0 - f, ty - t), (w0 - f, ty), (w0 + t, ty), (w0 + t, ty - K.TRAY_DEPTH + t), (w1 - t, ty - K.TRAY_DEPTH + t), (w1 - t, ty), (w0 - f, ty)], pen=small_pen))
        elif scale == 100:
            out.append(poly("top-tray", "top", 1.5, 0.4, "steel-desk", STEEL, [(w0, ty), (w0 + t, ty), (w0 + t, ty - K.TRAY_DEPTH + t), (w1 - t, ty - K.TRAY_DEPTH + t), (w1 - t, ty), (w1, ty), (w1, ty - K.TRAY_DEPTH), (w0, ty - K.TRAY_DEPTH)], pen=small_pen))
        else:
            out.append(rect("top-tray", "top", 1.5, 0.4, "steel-desk", STEEL, w0, w1, ty - K.TRAY_DEPTH, ty))
    _, (v0, v1), (w0, w1) = K.CONTROL
    out.append(rect("top-control", "top", 1.6, 0.22, "controls", PLASTIC, w0, w1, ty + v0, ty + v1, pen=small_pen))
    if s50:
        _, (v0, v1), (w0, w1) = K.HANDSET
        out.append(rect("top-handset", "top", 2.8, 0.12, "controls", PLASTIC, w0, w1, ty + v0, ty + v1, 0.006, pen=small_pen))
    out += monitor_front(scale, ty, small_pen)
    _, (v0, _), (w0, w1) = K.LAP_BASE
    out.append(rect("laptop", "top", 4.6, 0.42, "laptop-lid", PLASTIC, w0, w1, ty + v0, ty + K.LAP_LID[1][1], 0.003 if s50 else 0.0, pen=small_pen))
    return out


def monitor_front(scale, ty, pen):
    """🖥️ Monitor from the side: panel edge with its rear bulge, hinge, stand and base."""
    _, (pv0, pv1), (pw0, pw1) = K.MON_PANEL
    if scale == 500:
        return [rect("monitor-stand", "top", 4.05, 0.28, "monitor-housing", PLASTIC, 0.13, 0.2, ty + K.MON_BASE[1][0], ty + pv0 + 0.01),
                rect("monitor-panel", "top", 4.2, 0.18, "monitor-housing", PLASTIC, pw0, K.MON_BULGE[-1][0], ty + pv0, ty + pv1)]
    out = [rect("monitor-neck", "top", 4.05, 0.3, "monitor-housing", PLASTIC, K.MON_NECK[2][0], K.MON_NECK[2][1], ty + K.MON_NECK[1][0], ty + K.MON_NECK[1][1], pen=pen),
           rect("monitor-base", "top", 4.1, 0.26, "monitor-housing", PLASTIC, *K.MON_BASE[2], ty + K.MON_BASE[1][0], ty + K.MON_BASE[1][1], 0.004 if scale == 50 else 0.0, pen=pen)]
    if scale == 50:
        cv = ty + K.MON_BULGE_V
        out.append(rect("monitor-hinge", "top", 4.07, 0.22, "monitor-housing", PLASTIC, *K.MON_HINGE[2], ty + K.MON_HINGE[1][0], ty + K.MON_HINGE[1][1], pen=pen))
        out.append(poly("monitor-bulge", "top", 4.15, 0.24, "monitor-housing", PLASTIC, [(w, cv - ry) for w, _, ry in K.MON_BULGE] + [(w, cv + ry) for w, _, ry in reversed(K.MON_BULGE)], pen=pen))
        out.append(rect("monitor-panel", "top", 4.2, 0.18, "monitor-housing", PLASTIC, pw0, pw1, ty + pv0, ty + pv1, 0.004, pen=pen))
    else:
        out.append(rect("monitor-panel", "top", 4.2, 0.18, "monitor-housing", PLASTIC, pw0, K.MON_NECK[2][0], ty + pv0, ty + pv1, 0.006 if scale == 100 else 0.0, pen=pen))
    return out


def plan(scale):
    """🗺️ Plan from above: the board with the handset just proud of the front edge, the monitor and the laptop on it."""
    s50 = scale == 50
    pen = {50: 0.25, 100: 0.18, 200: None, 500: None}[scale]
    L, D, H, R = K.BOARD
    out = []
    if s50:
        (u0, u1), _, (w0, w1) = K.HANDSET
        out.append(rect("top-handset", "top", 0.5, 0.12, "controls", PLASTIC, u0, u1, w0, w1, 0.006, pen=pen))
    out.append(rect("top-board", "top", 1.0, 0.62, "top-board", BOARD, 0.0, L, -D / 2, D / 2, R if scale <= 100 else 0.0, hatch="board-grain" if s50 else None, pen={50: 0.5, 100: 0.35, 200: None, 500: None}[scale]))
    (pu0, pu1), _, (pw0, pw1) = K.MON_PANEL
    if scale == 500:
        out.append(rect("monitor-stand", "top", 2.5, 0.28, "monitor-housing", PLASTIC, 0.57, 0.63, 0.13, 0.2))
        out.append(rect("monitor-panel", "top", 3.0, 0.18, "monitor-housing", PLASTIC, pu0, pu1, pw0, K.MON_BULGE[-1][0]))
    else:
        out.append(rect("monitor-base", "top", 2.0, 0.26, "monitor-housing", PLASTIC, *K.MON_BASE[0], *K.MON_BASE[2], 0.008 if scale <= 100 else 0.0, pen=pen))
        out.append(rect("monitor-neck", "top", 2.5, 0.3, "monitor-housing", PLASTIC, *K.MON_NECK[0], *K.MON_NECK[2], pen=pen))
        if s50:
            out.append(rect("monitor-hinge", "top", 2.6, 0.22, "monitor-housing", PLASTIC, *K.MON_HINGE[0], *K.MON_HINGE[2], pen=pen))
            out.append(poly("monitor-bulge", "top", 2.8, 0.24, "monitor-housing", PLASTIC, [(K.MON_X - rx, w) for w, rx, _ in K.MON_BULGE] + [(K.MON_X + rx, w) for w, rx, _ in reversed(K.MON_BULGE)], pen=pen))
            out.append(rect("monitor-panel", "top", 3.0, 0.18, "monitor-housing", PLASTIC, pu0, pu1, pw0, pw1, 0.004, pen=pen))
        else:
            out.append(rect("monitor-panel", "top", 3.0, 0.18, "monitor-housing", PLASTIC, pu0, pu1, pw0, K.MON_NECK[2][0], 0.006 if scale == 100 else 0.0, pen=pen))
    (u0, u1), _, (w0, w1) = K.LAP_BASE
    out.append(rect("laptop", "top", 2.0, 0.42, "laptop-lid", PLASTIC, u0, u1, w0, w1, 0.004 if scale <= 100 else 0.0, pen=pen))
    if s50:
        (a0, a1), _, (d0, d1) = K.LAP_LID
        out.append(poly("laptop-hinge", "top", 2.1, 0.3, "laptop-lid", PLASTIC, [(a0 + 0.02, d1 - 0.012), (a1 - 0.02, d1 - 0.012)], closed=False, role="detail"))
    return out


def geometry(scale):
    """📐 The three views of one scale."""
    return {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}


def decisions(silhouette, retained, merged, omitted, thin, styles, overlays):
    """📝 The eight decision texts of one representation."""
    return {"silhouette": T(*silhouette), "retained": T(*retained), "merged": T(*merged), "omitted": T(*omitted), "thinElements": T(*thin), "hatching": T(*HATCH), "styles": T(*styles), "overlays": T(*overlays)}


HATCH = ("Laminate grain lines on the cut board edge and the board top at 1:50; none at the coarser scales.", "Dekorlinien auf der Plattenkante und der Plattenoberseite bei 1:50; keine bei den gröberen Maßstäben.")

DECISIONS = {
    50: decisions(
        ("Front elevation 32 x 23.6 mm from the user's side, the true view of the 1:50 model: two pressed T-feet seen end-on on their glides, the outer tube with its bushing, the middle stage with its bushing and the inner tube, the motor housing at each column head under the 60 x 25 mm side rail, the crossbar with its end flanges between the housings, the cable tray and the control box behind, the 25 mm board, the handset under the front edge, the 27-inch monitor with base, neck, panel and screen facing the viewer, and the closed laptop. Front view from the right end: the tapered 0.70 m foot with its two glides, the three tubes with bushings and the housing, the side rail under the board, the U of the cable tray, the control box, the handset just proud of the front edge, the monitor edge-on with its rear bulge, hinge, neck and base, the laptop edge-on. Plan: the board with 12 mm corner radii, the handset lip at the front edge, the monitor base, neck, hinge, bulge and panel, the laptop with its hinge line.",
         "Vorderansicht 32 x 23,6 mm von der Nutzerseite, die wahre Ansicht des 1:50-Modells: zwei gepresste T-Füße stirnseitig auf ihren Gleitern, das Außenrohr mit Buchse, die Mittelstufe mit Buchse und das Innenrohr, das Motorgehäuse an jedem Säulenkopf unter der Seitenzarge 60 x 25 mm, der Querträger mit seinen Endflanschen zwischen den Gehäusen, dahinter Kabelwanne und Steuerkasten, die 25-mm-Platte, das Bedienteil unter der Vorderkante, der 27-Zoll-Monitor mit Fuß, Hals, Panel und dem Betrachter zugewandtem Schirm und der geschlossene Laptop. Seitenansicht vom rechten Ende: der verjüngte 0,70-m-Fuß mit seinen zwei Gleitern, die drei Rohre mit Buchsen und das Gehäuse, die Seitenzarge unter der Platte, das U der Kabelwanne, der Steuerkasten, das knapp über die Vorderkante stehende Bedienteil, der Monitor hochkant mit rückseitiger Wölbung, Gelenk, Hals und Fuß, der Laptop hochkant. Grundriss: die Platte mit 12-mm-Eckradien, die Lippe des Bedienteils an der Vorderkante, Monitorfuß, Hals, Gelenk, Wölbung und Panel, der Laptop mit seiner Scharnierlinie."),
        ("Glides 40 x 11 mm, foot 80 x 45 mm (end edge at 28 mm), tubes 64, 56 and 48 mm wide (94, 86 and 78 mm deep) with 15 mm bushings, housings 80 x 100 x 150 mm, side rails 60 x 25 mm, crossbar 50 x 30 mm, tray 110 mm deep, control box, handset, board 25 mm, monitor 614 x 368 mm with screen, laptop 20 mm.", "Gleiter 40 x 11 mm, Fuß 80 x 45 mm (Stirnkante bei 28 mm), Rohre 64, 56 und 48 mm breit (94, 86 und 78 mm tief) mit 15-mm-Buchsen, Gehäuse 80 x 100 x 150 mm, Seitenzargen 60 x 25 mm, Querträger 50 x 30 mm, Wanne 110 mm tief, Steuerkasten, Bedienteil, Platte 25 mm, Monitor 614 x 368 mm mit Schirm, Laptop 20 mm."),
        ("Left and right column coincide in the front view (the right one is drawn); the two glides of a foot coincide in the elevation.", "Linke und rechte Säule fallen in der Seitenansicht zusammen (die rechte ist gezeichnet); die zwei Gleiter eines Fußes fallen in der Vorderansicht zusammen."),
        ("Screws, cables, the spindles and motors inside the tubes and housings, the handset buttons.", "Schrauben, Kabel, die Spindeln und Motoren in Rohren und Gehäusen, die Tasten des Bedienteils."),
        ("Tubes with 0.35 mm outlines so the 4 mm steps of the telescope read as joints; the 1.5 mm tray sheet, the foot end edge, the laptop seam and hinge are 0.25 mm strokes or thin filled bands.", "Rohre mit 0,35-mm-Umriss, sodass die 4-mm-Stufen des Teleskops als Fugen lesbar sind; das 1,5-mm-Blech der Wanne, die Fußstirnkante, Laptopfuge und -scharnier sind 0,25-mm-Striche oder dünne gefüllte Bänder."),
        ("Board 0.62, steel tubes 0.36 to 0.6 (inner tubes lighter), bushings and glides 0.12 to 0.14, housings and controls 0.12 to 0.22, monitor 0.18 to 0.3 with the screen at 0.08, laptop 0.42.", "Platte 0,62, Stahlrohre 0,36 bis 0,6 (Innenrohre heller), Buchsen und Gleiter 0,12 bis 0,14, Gehäuse und Steuerung 0,12 bis 0,22, Monitor 0,18 bis 0,3 mit Schirm 0,08, Laptop 0,42."),
        ("One shape per part and bone; the bushings and glides belong to the steel region, control box and handset to the controls region.", "Eine Form je Teil und Knochen; Buchsen und Gleiter gehören zur Stahlregion, Steuerkasten und Bedienteil zur Steuerregion.")),
    100: decisions(
        ("Front elevation 16 x 11.8 mm, the true view of the 1:100 model: feet on the floor, the three tubes as plain bands, motor housings, side rails, crossbar, tray, control box, the board, the monitor with base, neck, panel and screen, the laptop. Front view: tapered foot, tubes, housing, side rail, tray, control box, monitor edge-on as one block, laptop. Plan: board, monitor and laptop.", "Vorderansicht 16 x 11,8 mm, die wahre Ansicht des 1:100-Modells: Füße auf dem Boden, die drei Rohre als einfache Bänder, Motorgehäuse, Seitenzargen, Querträger, Wanne, Steuerkasten, die Platte, der Monitor mit Fuß, Hals, Panel und Schirm, der Laptop. Seitenansicht: verjüngter Fuß, Rohre, Gehäuse, Seitenzarge, Wanne, Steuerkasten, Monitor hochkant als ein Block, Laptop. Grundriss: Platte, Monitor und Laptop."),
        ("Feet, three tubes, housings, rails, crossbar, tray, control box, board, monitor with screen, laptop.", "Füße, drei Rohre, Gehäuse, Zargen, Querträger, Wanne, Steuerkasten, Platte, Monitor mit Schirm, Laptop."),
        ("The bushings into the tube ends, the glides into the feet, the monitor bulge and hinge into a 60 mm panel block.", "Die Buchsen in die Rohrenden, die Gleiter in die Füße, Wölbung und Gelenk des Monitors in einen 60-mm-Panelblock."),
        ("Handset, roundings under 15 mm, laptop seam and hinge, grain.", "Bedienteil, Rundungen unter 15 mm, Laptopfuge und -scharnier, Maserung."),
        ("Tubes with 0.25 mm outlines; the 25 mm board and rails and the 20 mm laptop as thin filled bands.", "Rohre mit 0,25-mm-Umriss; die 25-mm-Platte und -Zargen und der 20-mm-Laptop als dünne gefüllte Bänder."),
        ("As at 1:50.", "Wie bei 1:50."),
        ("One shape per part.", "Eine Form je Teil.")),
    200: decisions(
        ("Front elevation 8 x 5.9 mm: feet as bars, three stacked tubes per column under a head housing that reaches the board, crossbar, tray and control box, the board, monitor with base, neck and panel, laptop. Front view: straight foot bar, tubes, head housing, board, tray, control box, monitor block and laptop. Plan: board, monitor and laptop.", "Vorderansicht 8 x 5,9 mm: Füße als Balken, drei gestapelte Rohre je Säule unter einem bis zur Platte reichenden Kopfgehäuse, Querträger, Wanne und Steuerkasten, die Platte, Monitor mit Fuß, Hals und Panel, Laptop. Seitenansicht: gerader Fußbalken, Rohre, Kopfgehäuse, Platte, Wanne, Steuerkasten, Monitorblock und Laptop. Grundriss: Platte, Monitor und Laptop."),
        ("Feet, tubes, head housings, crossbar, tray, control box, board, monitor, laptop.", "Füße, Rohre, Kopfgehäuse, Querträger, Wanne, Steuerkasten, Platte, Monitor, Laptop."),
        ("Side rails into the head housings, foot taper into a straight 45 mm bar.", "Seitenzargen in die Kopfgehäuse, Fußverjüngung in einen geraden 45-mm-Balken."),
        ("Screen, roundings, bushings, glides.", "Schirm, Rundungen, Buchsen, Gleiter."),
        ("The 25 mm board, 20 mm laptop and 18 mm monitor base are drawn at their true thickness (thin filled bands under the outline pen).", "Die 25-mm-Platte, der 20-mm-Laptop und der 18-mm-Monitorfuß sind in wahrer Dicke gezeichnet (dünne gefüllte Bänder unter dem Umrissstift)."),
        ("Board 0.62, tubes 0.36 to 0.6, housings 0.2, monitor 0.18 to 0.3, laptop 0.42.", "Platte 0,62, Rohre 0,36 bis 0,6, Gehäuse 0,2, Monitor 0,18 bis 0,3, Laptop 0,42."),
        ("One shape per part.", "Eine Form je Teil.")),
    500: decisions(
        ("Desk sign 3.2 x 2.4 mm: two feet, the stacked tubes and head housing of each column, the board with its crossbar, the control box, the monitor as panel on a stand, the laptop. Front view: foot bar, tubes, housing, board, control box, monitor block and stand, laptop. Plan: board, monitor and laptop.", "Tischzeichen 3,2 x 2,4 mm: zwei Füße, die gestapelten Rohre und das Kopfgehäuse jeder Säule, die Platte mit Querträger, der Steuerkasten, der Monitor als Panel auf einem Ständer, der Laptop. Seitenansicht: Fußbalken, Rohre, Gehäuse, Platte, Steuerkasten, Monitorblock und Ständer, Laptop. Grundriss: Platte, Monitor und Laptop."),
        ("Feet, tubes, housings, crossbar, control box, board, monitor, laptop.", "Füße, Rohre, Gehäuse, Querträger, Steuerkasten, Platte, Monitor, Laptop."),
        ("Monitor base and neck into one stand; side rails into the housings.", "Monitorfuß und Hals zu einem Ständer; Seitenzargen in die Gehäuse."),
        ("Tray, screen, every rounding and detail stroke.", "Wanne, Schirm, jede Rundung und jeder Detailstrich."),
        ("All parts at their true size as 0.18 mm outlined bands; the 25 mm board and the 20 mm laptop read as lines.", "Alle Teile in wahrer Größe als Bänder mit 0,18-mm-Umriss; die 25-mm-Platte und der 20-mm-Laptop lesen sich als Linien."),
        ("Board 0.62, tubes 0.36 to 0.6, housings 0.2, monitor 0.18 to 0.28, laptop 0.42.", "Platte 0,62, Rohre 0,36 bis 0,6, Gehäuse 0,2, Monitor 0,18 bis 0,28, Laptop 0,42."),
        ("One shape per part.", "Eine Form je Teil.")),
}
