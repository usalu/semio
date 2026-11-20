"""🧱 Round 3, furniture/height-adjustable-desk: the body, thermal and structure solids of every authored scale, section by section by hand.

Usage: python3 r3-height-adjustable-desk-model.py   (prints solid counts per scale and list)
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
BOARD_LAYERS = ("board", "laminate")


def box(id, bone, material, tone, lo, hi, radius=0.0, role="body", **extra):
    """📦 Box between the bone-local corners lo and hi (u, v, w)."""
    size = [r(hi[i] - lo[i]) for i in range(3)]
    s = {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": size}
    if radius:
        s["cornerRadius"] = radius
    s["transform"] = {"translation": [r((lo[i] + hi[i]) / 2) for i in range(3)]}
    s.update(extra)
    return s


def extrude(id, bone, material, tone, plane, profile, lo, hi, role="body", holes=None, **extra):
    """🧯 Extrude of a hand-written profile on a bone-local plane."""
    s = {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": profile}
    if holes:
        s["holes"] = holes
    s.update({"plane": plane, "from": r(lo), "to": r(hi)})
    s.update(extra)
    return s


def ellipsoid(id, bone, material, tone, centre, radii, role="detail", **extra):
    """🥚 Ellipsoid at a bone-local centre."""
    s = {"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": role, "tone": tone, "radii": [r(x) for x in radii], "transform": {"translation": [r(x) for x in centre]}}
    s.update(extra)
    return s


def ref(id, of, **extra):
    """🔗 Reference to a body solid of the same scale."""
    s = {"id": id, "of": of}
    s.update(extra)
    return s


def foot_profile(under):
    """🦶 Side profile (v, w) of the pressed T-foot: flat underside, top rising from 28 mm at the ends to 45 mm over the column."""
    h, e, m = K.FOOT_L / 2, K.FOOT_END_H, K.FOOT_MID_HALF
    return [[r(under), -h], [r(under), h], [e, h], [K.FOOT_MID_H, m], [K.FOOT_MID_H, -m], [e, -h]]


def tray_profile(f=K.TRAY_FLANGE):
    """🧺 Section (v, w) of the 1.5 mm sheet cable tray: U with screw flanges of width f (0: plain U), open at the top against the board."""
    w0, w1, d, t = K.TRAY_W[0], K.TRAY_W[1], K.TRAY_DEPTH, K.TRAY_T
    if f == 0:
        return [[0.0, w0], [0.0, r(w0 + t)], [r(-d + t), r(w0 + t)], [r(-d + t), r(w1 - t)], [0.0, r(w1 - t)], [0.0, w1], [r(-d), w1], [r(-d), w0]]
    return [[0.0, r(w0 - f)], [0.0, r(w0 + t)], [r(-d + t), r(w0 + t)], [r(-d + t), r(w1 - t)], [0.0, r(w1 - t)], [0.0, r(w1 + f)], [r(-t), r(w1 + f)], [r(-t), w1], [r(-d), w1], [r(-d), w0], [r(-t), w0], [r(-t), r(w0 - f)]]


def column_side(scale, s, side):
    """🦿 One T-foot and its telescoping column at a scale."""
    foot, stage, column = f"foot-{side}", f"stage-{side}", f"column-{side}"
    U = K.FOOT_U
    out = []
    if scale == 50:
        out.append({"id": f"{s}-glide", "kind": "revolve", "bone": foot, "material": PLASTIC, "role": "body", "tone": 0.12, "profile": [[0.0, K.GLIDE_R], [0.007, K.GLIDE_R], [0.011, 0.014]], "axis": [0, 1, 0], "origin": [U, 0, -K.GLIDE_W], "repeat": {"count": 2, "translation": [0, 0, r(2 * K.GLIDE_W)]}})
        out.append(extrude(f"{s}-foot", foot, STEEL, 0.3, "yz", foot_profile(K.FOOT_UNDER), 0.0, K.FOOT_W))
        a, b, rad = K.SLEEVE
        out.append(extrude(f"{s}-sleeve", foot, STEEL, 0.36, "zx", K.rounded_rect(-b / 2, b / 2, U - a / 2, U + a / 2, rad), *K.SLEEVE_V))
        a, b, rad = K.SLEEVE_COLLAR
        out.append(extrude(f"{s}-sleeve-collar", foot, PLASTIC, 0.14, "zx", K.rounded_rect(-b / 2, b / 2, U - a / 2, U + a / 2, rad), *K.SLEEVE_COLLAR_V, role="detail"))
        a, b, rad = K.STAGE
        out.append(extrude(f"{s}-stage", stage, STEEL, 0.48, "yz", K.centred(a, b, rad), *K.STAGE_U))
        a, b, rad = K.STAGE_COLLAR
        out.append(extrude(f"{s}-stage-collar", stage, PLASTIC, 0.14, "yz", K.centred(a, b, rad), *K.STAGE_COLLAR_U, role="detail"))
        a, b, rad = K.COLUMN
        out.append(extrude(f"{s}-column", column, STEEL, 0.6, "yz", K.centred(a, b, rad), *K.COLUMN_U))
        a, b, rad = K.DRIVE
        out.append(box(f"{s}-drive", column, PLASTIC, 0.2, (K.DRIVE_U[0], -a / 2, -b / 2), (K.DRIVE_U[1], a / 2, b / 2), rad))
        return out
    top_h = {100: K.FOOT_END_H, 200: K.FOOT_MID_H, 500: K.FOOT_MID_H}[scale]
    if scale == 100:
        out.append(extrude(f"{s}-foot", foot, STEEL, 0.3, "yz", foot_profile(0.0), 0.0, K.FOOT_W))
    else:
        out.append(box(f"{s}-foot", foot, STEEL, 0.3, (0.0, 0.0, -K.FOOT_L / 2), (K.FOOT_W, top_h, K.FOOT_L / 2)))
    a, b, _ = K.SLEEVE
    out.append(box(f"{s}-sleeve", foot, STEEL, 0.36, (U - a / 2, K.SLEEVE_V[0], -b / 2), (U + a / 2, K.SLEEVE_COLLAR_V[1], b / 2)))
    a, b, _ = K.STAGE
    out.append(box(f"{s}-stage", stage, STEEL, 0.48, (0.0, -a / 2, -b / 2), (K.STAGE_COLLAR_U[1], a / 2, b / 2)))
    a, b, _ = K.COLUMN
    out.append(box(f"{s}-column", column, STEEL, 0.6, (0.0, -a / 2, -b / 2), (K.COLUMN_U[1], a / 2, b / 2)))
    a, b, rad = K.DRIVE
    head = K.DRIVE_U[1] if scale == 100 else K.COLUMN_LEN
    out.append(box(f"{s}-drive", column, PLASTIC, 0.2, (K.DRIVE_U[0], -a / 2, -b / 2), (head, a / 2, b / 2), rad if scale == 100 else 0.0))
    return out


def top_parts(scale):
    """🪵 Board, frame rails, tray, controls, monitor and laptop on the top bone at a scale."""
    L, D, H, R = K.BOARD
    out = []
    if scale == 50:
        out.append(extrude("top-board", "top", BOARD, 0.62, "zx", K.rounded_rect(-D / 2, D / 2, 0.0, L, R, 4), 0.0, H))
    else:
        out.append(box("top-board", "top", BOARD, 0.62, (0.0, 0.0, -D / 2), (L, H, D / 2)))
    bw, bh, bl = K.BRACKET
    if scale in (50, 100):
        prof = K.rounded_rect(K.COLUMN_X[0] - bw / 2, K.COLUMN_X[0] + bw / 2, -bh, 0.0, 0.003 if scale == 50 else 0.0)
        out.append(extrude("top-bracket", "top", STEEL, 0.3, "xy", prof, -bl / 2, bl / 2, repeat={"count": 2, "translation": [r(K.COLUMN_X[1] - K.COLUMN_X[0]), 0, 0]}))
    cw = K.CROSSBAR_W
    if scale == 50:
        out.append(extrude("top-crossbar", "top", STEEL, 0.34, "yz", K.rounded_rect(K.CROSSBAR_V[0], K.CROSSBAR_V[1], -cw / 2, cw / 2, 0.003), *K.CROSSBAR_U))
    else:
        out.append(box("top-crossbar", "top", STEEL, 0.34, (K.CROSSBAR_U[0], K.CROSSBAR_V[0], -cw / 2), (K.CROSSBAR_U[1], K.CROSSBAR_V[1], cw / 2)))
    if scale == 50:
        x0, x1 = K.COLUMN_X[0] + K.DRIVE[0] / 2, K.COLUMN_X[1] - K.DRIVE[0] / 2 - K.FLANGE_T
        out.append(box("top-crossbar-flange", "top", STEEL, 0.28, (x0, K.FLANGE_V[0], -K.FLANGE_W / 2), (x0 + K.FLANGE_T, K.FLANGE_V[1], K.FLANGE_W / 2), role="detail", repeat={"count": 2, "translation": [r(x1 - x0), 0, 0]}))
    if scale == 50:
        out.append(extrude("top-tray", "top", STEEL, 0.4, "yz", tray_profile(), *K.TRAY_U))
    elif scale == 100:
        out.append(extrude("top-tray", "top", STEEL, 0.4, "yz", tray_profile(0.0), *K.TRAY_U))
    elif scale == 200:
        out.append(box("top-tray", "top", STEEL, 0.4, (K.TRAY_U[0], -K.TRAY_DEPTH, K.TRAY_W[0]), (K.TRAY_U[1], 0.0, K.TRAY_W[1])))
    (u0, u1), (v0, v1), (w0, w1) = K.CONTROL
    out.append(box("top-control", "top", PLASTIC, 0.22, (u0, v0, w0), (u1, v1, w1), 0.004 if scale == 50 else 0.0))
    if scale == 50:
        (u0, u1), (v0, v1), (w0, w1) = K.HANDSET
        out.append(box("top-handset", "top", PLASTIC, 0.12, (u0, v0, w0), (u1, v1, w1), 0.006))
    out += monitor(scale) + laptop(scale)
    return out


def monitor(scale):
    """🖥️ The 27-inch monitor on its height-adjustable stand, screen towards the user (-z)."""
    def b(id, part, tone, radius=0.0, role="body"):
        (u0, u1), (v0, v1), (w0, w1) = part
        return box(id, "top", PLASTIC, tone, (u0, v0, w0), (u1, v1, w1), radius, role)
    if scale == 50:
        bulge = {"id": "monitor-bulge", "kind": "loft", "bone": "top", "material": PLASTIC, "role": "body", "tone": 0.24, "sections": [{"at": [K.MON_X, K.MON_BULGE_V, w], "rx": rx, "ry": ry, "exponent": 5 if i == 0 else 4} for i, (w, rx, ry) in enumerate(K.MON_BULGE)], "capStart": True, "capEnd": True}
        return [b("monitor-base", K.MON_BASE, 0.26, 0.004), b("monitor-neck", K.MON_NECK, 0.3, 0.008), b("monitor-hinge", K.MON_HINGE, 0.22, 0.006), b("monitor-panel", K.MON_PANEL, 0.18, 0.004), bulge, b("monitor-screen", K.MON_SCREEN, 0.08, 0.0, "glazing")]
    panel = (K.MON_PANEL[0], K.MON_PANEL[1], (K.MON_PANEL[2][0], K.MON_BULGE[-1][0] if scale == 500 else K.MON_NECK[2][0]))
    if scale == 500:
        stand = (K.MON_NECK[0], (K.MON_BASE[1][0], K.MON_PANEL[1][0] + 0.01), K.MON_NECK[2])
        return [b("monitor-panel", panel, 0.18), b("monitor-stand", ((0.57, 0.63), stand[1], (0.13, 0.20)), 0.28)]
    out = [b("monitor-base", K.MON_BASE, 0.26), b("monitor-neck", K.MON_NECK, 0.3), b("monitor-panel", panel, 0.18, 0.006 if scale == 100 else 0.0)]
    return out + ([b("monitor-screen", K.MON_SCREEN, 0.08, 0.0, "glazing")] if scale == 100 else [])


def laptop(scale):
    """💻 The closed 15.6-inch laptop: base and lid, hinge at the back."""
    if scale == 50:
        (u0, u1), (v0, v1), (w0, w1) = K.LAP_BASE
        (a0, a1), (c0, c1), (d0, d1) = K.LAP_LID
        return [box("laptop", "top", PLASTIC, 0.42, (u0, v0, w0), (u1, v1, w1), 0.004), box("laptop-lid", "top", PLASTIC, 0.5, (a0, c0, d0), (a1, c1, d1), 0.003)]
    (u0, u1), (v0, _), (w0, w1) = K.LAP_BASE
    return [box("laptop", "top", PLASTIC, 0.42, (u0, v0, w0), (u1, K.LAP_LID[1][1], w1), 0.004 if scale == 100 else 0.0)]


def body(scale):
    """🧍 Body list of a scale."""
    out = []
    for s, side in SIDES:
        out += column_side(scale, s, side)
    return out + top_parts(scale)


def thermal(scale, bodies):
    """🌡️ Thermal list: references where the region surface is the body surface, the nested top shells at 1:50."""
    ids = {b["id"] for b in bodies}
    region = {"glide": "steel-desk", "foot": "steel-desk", "sleeve": "steel-desk", "sleeve-collar": "steel-desk", "stage": "steel-desk", "stage-collar": "steel-desk", "column": "steel-desk"}
    layer = {"steel-desk": "tube", "drive-left": "housing", "drive-right": "housing", "monitor-housing": "housing", "laptop-lid": "case", "controls": "housing"}
    multi = scale == 50
    out = []
    for s, side in SIDES:
        for part, reg in region.items():
            if f"{s}-{part}" in ids:
                out.append(ref(f"th-{s}-{part}", f"{s}-{part}", region=reg, **({"layer": layer[reg]} if multi else {})))
        out.append(ref(f"th-{s}-drive", f"{s}-drive", region=f"drive-{side}", **({"layer": "housing"} if multi else {})))
    for part, reg in (("top-bracket", "steel-desk"), ("top-crossbar", "steel-desk"), ("top-crossbar-flange", "steel-desk"), ("top-tray", "steel-desk"), ("top-control", "controls"), ("top-handset", "controls"), ("monitor-base", "monitor-housing"), ("monitor-neck", "monitor-housing"), ("monitor-stand", "monitor-housing"), ("monitor-hinge", "monitor-housing"), ("monitor-panel", "monitor-housing"), ("monitor-bulge", "monitor-housing"), ("monitor-screen", "monitor-housing"), ("laptop", "laptop-lid"), ("laptop-lid", "laptop-lid")):
        if part in ids:
            out.append(ref(f"th-{part}", part, region=reg, **({"layer": layer[reg]} if multi else {})))
    if multi:
        L, D, H, R = K.BOARD
        t = K.LAMINATE_T
        prof = K.rounded_rect(-D / 2, D / 2, 0.0, L, R, 4)
        core = K.rounded_rect(-D / 2 + 0.0005, D / 2 - 0.0005, 0.0005, L - 0.0005, R - 0.0005, 4)
        out.append(dict(extrude("th-top-board-core", "top", BOARD, 0.55, "zx", core, t, H - t), region="top-board", layer=BOARD_LAYERS[0]))
        out.append(ref("th-top-board-laminate", "top-board", region="top-board", layer=BOARD_LAYERS[1]))
    else:
        out.append(ref("th-top-board", "top-board", region="top-board"))
    return out


def structure(scale, bodies):
    """🏗️ Load path: feet, the three telescoping tubes, slide bushings, head plates, side rails, crossbar and board; contacts; masses."""
    ids = {b["id"] for b in bodies}
    f = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[scale]
    m = K.masses()
    out = []
    for s, side in SIDES:
        foot, stage, column = f"foot-{side}", f"stage-{side}", f"column-{side}"
        out.append(ref(f"m-{s}-foot", f"{s}-foot", member=foot, part="member"))
        out.append(ref(f"m-{s}-sleeve", f"{s}-sleeve", member=foot, part="member"))
        out.append(ref(f"m-{s}-stage", f"{s}-stage", member=stage, part="member"))
        out.append(ref(f"m-{s}-column", f"{s}-column", member=column, part="member"))
        if scale == 500:
            pass
        elif f"{s}-sleeve-collar" in ids:
            out.append(ref(f"j-{s}-slide-lower", f"{s}-sleeve-collar", member=stage, part="joint"))
            out.append(ref(f"j-{s}-slide-upper", f"{s}-stage-collar", member=column, part="joint"))
        else:
            h = max(f, 0.015) if scale < 500 else 0.056
            a, b, _ = K.SLEEVE
            top = K.SLEEVE_COLLAR_V[1]
            out.append(dict(box(f"j-{s}-slide-lower", foot, STEEL, 0.2, (K.FOOT_U - a / 2 - 0.002, top - h, -b / 2 - 0.002), (K.FOOT_U + a / 2 + 0.002, top, b / 2 + 0.002)), member=stage, part="joint"))
            a, b, _ = K.STAGE
            top = K.STAGE_COLLAR_U[1]
            out.append(dict(box(f"j-{s}-slide-upper", stage, STEEL, 0.2, (top - h, -a / 2 - 0.002, -b / 2 - 0.002), (top, a / 2 + 0.002, b / 2 + 0.002)), member=column, part="joint"))
        hp = max(f, 0.012) if scale < 500 else 0.06
        out.append(dict(box(f"j-{s}-head", column, STEEL, 0.2, (K.DRIVE_U[1] - hp / 2, -K.DRIVE[0] / 2, -K.DRIVE[1] / 2), (K.DRIVE_U[1] + hp / 2, K.DRIVE[0] / 2, K.DRIVE[1] / 2)), member="top", part="joint"))
        ph = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.03}[scale]
        pg = {50: 0.0, 100: 0.004, 200: 0.008, 500: 0.015}[scale]
        out.append(dict(box(f"support-contact-foot-{side}", foot, STEEL, 0.2, (-pg, 0.0, -K.FOOT_L / 2 - pg), (K.FOOT_W + pg, ph, K.FOOT_L / 2 + pg), role="ground"), member=foot, part="support", anchor=f"contact-foot-{side}"))
        rf, rs, rc = {50: (0.016, 0.016, 0.016), 100: (0.016, 0.016, 0.016), 200: (0.022, 0.022, 0.022), 500: (0.0235, 0.0285, 0.0245)}[scale]
        kg, cu, cv = m["foot"]
        out.append(dict(ellipsoid(f"mass-{s}-foot", foot, STEEL, 0.5, (cu, cv, 0.0), (rf, rf, rf)), member=foot, part="mass"))
        kg, cu, cv = m["stage"]
        out.append(dict(ellipsoid(f"mass-{s}-stage", stage, STEEL, 0.5, (cu, 0.0, 0.0), (rs, rs, rs)), member=stage, part="mass"))
        kg, cu, cv = m["column"]
        out.append(dict(ellipsoid(f"mass-{s}-column", column, STEEL, 0.5, (cu, 0.0, 0.0), (rc, rc, rc)), member=column, part="mass"))
    out.append(ref("m-top-board", "top-board", member="top", part="member"))
    if "top-bracket" in ids:
        out.append(ref("m-top-bracket", "top-bracket", member="top", part="member"))
    out.append(ref("m-top-crossbar", "top-crossbar", member="top", part="member"))
    if "top-crossbar-flange" in ids:
        out.append(ref("j-top-crossbar", "top-crossbar-flange", member="top", part="joint"))
    elif scale < 500:
        x0, x1 = K.COLUMN_X[0] + K.DRIVE[0] / 2, K.COLUMN_X[1] - K.DRIVE[0] / 2 - f
        out.append(dict(box("j-top-crossbar", "top", STEEL, 0.2, (x0, K.FLANGE_V[0], -K.FLANGE_W / 2), (x0 + f, K.FLANGE_V[1], K.FLANGE_W / 2), repeat={"count": 2, "translation": [r(x1 - x0), 0, 0]}), member="top", part="joint"))
    kg, cu, cv = m["top"]
    tv = {50: 0.006, 100: 0.0075, 200: 0.015, 500: 0.0125}[scale]
    out.append(dict(ellipsoid("mass-top", "top", BOARD, 0.5, (cu, cv if scale < 200 else 0.0125, 0.0), (max(0.04, f), tv, max(0.04, f))), member="top", part="mass"))
    mv = (K.MON_PANEL[1][0] + K.MON_PANEL[1][1]) / 2 - 0.03
    out.append(dict(ellipsoid("mass-monitor", "top", PLASTIC, 0.5, (K.MON_X, mv, 0.145), (max(0.03, f), max(0.03, f), 0.02 if scale < 500 else 0.025)), member="top", part="mass"))
    out.append(dict(ellipsoid("mass-laptop", "top", PLASTIC, 0.5, (K.LAP_X, 0.035, sum(K.LAP_BASE[2]) / 2), (max(0.03, f), 0.0101, max(0.03, f))), member="top", part="mass"))
    return out


DECISIONS = {
    50: [T("1:50 is the complete desk as built: pressed 0.70 m T-feet rising from 28 mm at the ends to 45 mm over the column on two 40 mm glides each, three nested rectangular steel tubes per column (outer 64 x 94 mm in the foot, stage 56 x 86 mm, inner 48 x 78 mm) with their plastic slide bushings, the 80 x 100 x 150 mm motor housing at each column head, the two 60 x 25 mm side rails under the board, the 50 x 30 mm crossbar with its 8 mm end flanges between the column heads, the 1.5 mm sheet cable tray, the control box and the handset under the front edge, the 25 mm board with 12 mm corner radii, a 27-inch monitor (base, stand, tilt hinge, 15 mm panel with its rear bulge and the glazed screen towards the user) and a closed 15.6-inch laptop (base and lid).",
           "1:50 ist der vollständige Tisch, wie er gebaut ist: gepresste 0,70-m-T-Füße, von 28 mm an den Enden auf 45 mm über der Säule ansteigend, auf je zwei 40-mm-Gleitern, drei ineinander laufende Rechteckstahlrohre je Säule (außen 64 x 94 mm im Fuß, Stufe 56 x 86 mm, innen 48 x 78 mm) mit ihren Kunststoff-Gleitbuchsen, das Motorgehäuse 80 x 100 x 150 mm an jedem Säulenkopf, die beiden Seitenzargen 60 x 25 mm unter der Platte, der Querträger 50 x 30 mm mit seinen 8-mm-Endflanschen zwischen den Säulenköpfen, die Kabelwanne aus 1,5-mm-Blech, Steuerkasten und Bedienteil unter der Vorderkante, die 25-mm-Platte mit 12-mm-Eckradien, ein 27-Zoll-Monitor (Fuß, Ständer, Neigegelenk, 15-mm-Panel mit rückseitiger Wölbung und verglastem Schirm zum Nutzer) und ein geschlossener 15,6-Zoll-Laptop (Unterteil und Deckel)."),
         T("Thermal: the steel, drive, controls, monitor and laptop regions reference their body solids; the top board is two nested shells (the 1 mm laminate skin all round as the board solid itself, the 23 mm particleboard core inside it); the inner column tube runs on through the motor housing to the head plate. Structure: feet, outer tubes, stages and inner tubes are the members of their bones, the slide bushings are the prismatic joints, a head plate is the fixed joint to the top, the crossbar flanges are its fixed joints to the column heads, side rails, crossbar and board carry the top; contact pads under the foot footprints; masses at the centres of mass of the bones, the monitor and the laptop.",
           "Thermik: Stahl-, Antriebs-, Steuer-, Monitor- und Laptopregion verweisen auf ihre Körper; die Platte besteht aus zwei geschachtelten Schalen (die 1-mm-Beschichtung ringsum als Plattenkörper selbst, der 23-mm-Spanplattenkern darin); das Innenrohr läuft durch das Motorgehäuse bis zur Kopfplatte. Tragwerk: Füße, Außenrohre, Stufen und Innenrohre sind die Stäbe ihrer Knochen, die Gleitbuchsen die Schubgelenke, eine Kopfplatte das feste Gelenk zur Platte, die Querträgerflansche dessen feste Gelenke zu den Säulenköpfen, Seitenzargen, Querträger und Platte tragen die Platte; Kontaktflächen unter den Fußflächen; Massen in den Schwerpunkten der Knochen, des Monitors und des Laptops.")],
    100: [T("1:100 keeps every assembly at its true size but drops what lies under 15 mm: the glides (the foot stands on its pressed underside), the 15 mm slide bushings (merged into the tube ends), the handset, the tilt hinge, the rear bulge of the monitor (merged into a 60 mm panel block that reaches the stand), the crossbar flanges and the screen glazing detail; tubes, housings, rails, crossbar and laptop become plain blocks with the true sections, the tapered foot keeps its profile and the cable tray stays an open sheet U without its screw flanges.",
            "1:100 behält jede Baugruppe in wahrer Größe, lässt aber weg, was unter 15 mm liegt: die Gleiter (der Fuß steht auf seiner Unterseite), die 15-mm-Gleitbuchsen (in die Rohrenden verschmolzen), das Bedienteil, das Neigegelenk, die rückseitige Wölbung des Monitors (in einen 60-mm-Panelblock verschmolzen, der bis zum Ständer reicht), die Querträgerflansche und das Glasdetail des Schirms; Rohre, Gehäuse, Zargen, Querträger und Laptop werden einfache Blöcke mit wahren Querschnitten, der verjüngte Fuß behält sein Profil und die Kabelwanne bleibt ein offenes Blech-U ohne Schraubflansche."),
          T("Thermal references the 1:100 bodies, the board as one shell (layers merged); structure as at 1:50 with 15 mm slide collars at the tube ends and 15 mm flange joints at the crossbar ends.", "Die Thermik verweist auf die Körper von 1:100, die Platte als eine Schale (Schichten verschmolzen); Tragwerk wie bei 1:50 mit 15-mm-Gleitmanschetten an den Rohrenden und 15-mm-Flanschgelenken an den Querträgerenden.")],
    200: [T("1:200 merges further for the 30 mm feature: the feet are straight 45 mm bars, each motor housing grows up to the board and absorbs the 25 mm side rail, the monitor keeps base, stand and a 60 mm panel block, the tray is a closed block and the control box stays a block; the 25 mm board, the 20 mm laptop and the 48 to 64 mm tubes keep their true sizes because the drawing shows them so (C13).",
            "1:200 verschmilzt weiter für das 30-mm-Merkmal: die Füße sind gerade 45-mm-Balken, jedes Motorgehäuse reicht bis zur Platte und nimmt die 25-mm-Seitenzarge auf, der Monitor behält Fuß, Ständer und einen 60-mm-Panelblock, die Wanne ist ein geschlossener Block und der Steuerkasten bleibt ein Block; die 25-mm-Platte, der 20-mm-Laptop und die 48- bis 64-mm-Rohre behalten ihre wahre Größe, weil die Zeichnung sie so zeigt (C13)."),
          T("Thermal references the 1:200 bodies; structure: feet, tubes, crossbar and board as members, 30 mm slide collars, crossbar flange joints, head plates, contact pads and masses.", "Die Thermik verweist auf die Körper von 1:200; Tragwerk: Füße, Rohre, Querträger und Platte als Stäbe, 30-mm-Gleitmanschetten, Flanschgelenke des Querträgers, Kopfplatten, Kontaktflächen und Massen.")],
    500: [T("1:500 is the desk sign: two T-feet, the three stacked tubes and the head housing per column, the board with its crossbar, the monitor as panel and stand and the laptop and control box as thin blocks; the tray, the monitor base and every rounding are omitted, the parts keep their true sizes as drawn (C13).",
            "1:500 ist das Tischzeichen: zwei T-Füße, die drei gestapelten Rohre und das Kopfgehäuse je Säule, die Platte mit ihrem Querträger, der Monitor als Panel und Ständer, Laptop und Steuerkasten als dünne Blöcke; Wanne, Monitorfuß und alle Rundungen entfallen, die Teile behalten ihre gezeichnete wahre Größe (C13)."),
          T("Thermal references the 1:500 bodies; structure: the same members without slide and flange joints (below the 75 mm feature), head plates of the housing section, 30 mm contact pads standing 15 mm proud of the foot footprint and the masses.", "Die Thermik verweist auf die Körper von 1:500; Tragwerk: dieselben Stäbe ohne Gleit- und Flanschgelenke (unter dem 75-mm-Merkmal), Kopfplatten im Gehäusequerschnitt, 30-mm-Kontaktflächen, die 15 mm über die Fußfläche vorstehen, und die Massen.")],
}


def volumes():
    """📚 The four authored scales."""
    out = {}
    for scale in (50, 100, 200, 500):
        b = body(scale)
        out[str(scale)] = {"status": "authored", "denominator": scale, "body": b, "thermal": thermal(scale, b), "structure": structure(scale, b), "decisions": DECISIONS[scale]}
    return out


if __name__ == "__main__":
    for k, v in volumes().items():
        print(k, len(v["body"]), len(v["thermal"]), len(v["structure"]))
