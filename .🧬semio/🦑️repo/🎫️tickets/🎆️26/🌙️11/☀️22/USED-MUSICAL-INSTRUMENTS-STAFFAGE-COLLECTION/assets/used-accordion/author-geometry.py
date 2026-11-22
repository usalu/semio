"""🪗️ Hand-chosen geometry of `used-accordion`: one parts list per authored scale in world metres of the rest pose (= the default pose `stored`).

Every part is an axis-aligned box, a cylinder, a capsule or an extrusion on one bone; `author-accordion.py` turns each part into its body solid (bone-local),
its thermal reference and its three 2D drawings, so the 2D and the 3D of a scale are the same numbers.

World frame: x ahead along the bellows axis (treble end at -x, bass end at +x), y up, z to the left of the asset (the grille and the audience at -z,
the player's chest, the straps and the hinge-side back of the case at +z). The case stands open on the floor, its front lid lying flat at -z.

@see ../../brief.md
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03}

T_WALL = 0.010
T_LINING = 0.008
CASE_X = 0.25
CASE_Z0 = -0.110
CASE_Z1 = 0.135
CASE_TOP = 0.510
LID_T = 0.025
LID_Z_FAR = -0.62

X_T = -0.074
Y_FOOT = 0.018
Y_MID = 0.2555
Y_BOX0 = 0.028
Y_BOX1 = 0.483
Z_BOX = 0.095
PITCH_CLOSED = 0.030
SEG_LEN = 0.110
SEGMENTS = 4
WHITE_PITCH = 0.0177
WHITE_V0 = 0.0431
BLACK_AFTER = [i for i in range(23) if i % 7 in (0, 1, 2, 4, 5)]
BUTTON_PITCH = 0.0185

ORIGINS = {
    "case": (-CASE_X, 0.0, 0.0),
    "lid": (-CASE_X, 0.0125, -0.1225),
    "handle": (-0.065, CASE_TOP, 0.0125),
    "treble": (X_T, Y_MID, 0.0),
    **{f"bellows-{k}": (X_T + PITCH_CLOSED * (k - 1), Y_MID, 0.0) for k in range(1, SEGMENTS + 1)},
    "bass": (X_T + PITCH_CLOSED * SEGMENTS, Y_MID, 0.0),
}


def box(pid, bone, x0, x1, y0, y1, z0, z1, material, region, tone, role="body", **extra):
    """📦️ One axis-aligned box part in world rest metres."""
    return {"id": pid, "bone": bone, "kind": "box", "x": (x0, x1), "y": (y0, y1), "z": (z0, z1), "material": material, "region": region, "tone": tone, "role": role, **extra}


def cylinder(pid, bone, axis, base, length, radius, material, region, tone, role="body", **extra):
    """🛢️ One cylinder part: `axis` "x" or "y", `base` the world centre of its first face."""
    bx, by, bz = base
    if axis == "y":
        x, y, z = (bx - radius, bx + radius), (by, by + length), (bz - radius, bz + radius)
    else:
        x, y, z = (bx, bx + length), (by - radius, by + radius), (bz - radius, bz + radius)
    return {"id": pid, "bone": bone, "kind": "cylinder", "axis": axis, "base": base, "length": length, "radius": radius, "x": x, "y": y, "z": z, "material": material, "region": region, "tone": tone, "role": role, **extra}


def case_tub(scale):
    """🧳️ The tub of the hard case: boards of 9 mm poplar plywood with tolex (10 mm), the plush lining, corner caps, latches.

    At 1:100 and 1:200 the end walls, floor and top are one hollow frame (an extrusion with its opening as a hole, so the true wall needs no flag), drawn as one outline per view."""
    if scale in (100, 200):
        t = T_WALL if scale == 100 else T_WALL + T_LINING
        back0 = CASE_Z1 - T_WALL if scale == 100 else CASE_Z1 - T_WALL - T_LINING
        parts = [
            {"id": "case-frame", "bone": "case", "kind": "frame", "wall": t, "x": (-CASE_X, CASE_X), "y": (0.0, CASE_TOP), "z": (CASE_Z0, CASE_Z1), "material": "case-tolex", "region": "case-shell", "tone": 0.22, "role": "body", "draw": False},
            box("case-back", "case", -CASE_X + t, CASE_X - t, t, CASE_TOP - t, back0, CASE_Z1, "case-tolex", "case-shell" if scale == 100 else "case-lining", 0.22, sheet=True, draw=False),
            {"id": "case-drawing", "bone": "case", "kind": "drawing", "x": (-CASE_X, CASE_X), "y": (0.0, CASE_TOP), "z": (CASE_Z0, CASE_Z1), "material": "case-tolex", "region": "case-shell", "tone": 0.22, "role": "body", "inner": t},
        ]
        if scale == 100:
            parts += [
                box("lining-floor", "case", -0.24, 0.24, T_WALL, T_WALL + T_LINING, CASE_Z0, 0.125, "case-plush", "case-lining", 0.42, sheet=True, draw=False),
                box("lining-back", "case", -0.24, 0.24, T_WALL + T_LINING, CASE_TOP - T_WALL, 0.125 - T_LINING, 0.125, "case-plush", "case-lining", 0.42, sheet=True, draw=False),
                box("lining-end-treble", "case", -0.24, -0.24 + T_LINING, T_WALL + T_LINING, CASE_TOP - T_WALL, CASE_Z0, 0.117, "case-plush", "case-lining", 0.42, sheet=True, draw=False),
                box("lining-end-bass", "case", 0.24 - T_LINING, 0.24, T_WALL + T_LINING, CASE_TOP - T_WALL, CASE_Z0, 0.117, "case-plush", "case-lining", 0.42, sheet=True, draw=False),
            ]
        return parts
    parts = []
    sheet = False
    parts += [
        box("case-bottom", "case", -CASE_X, CASE_X, 0.0, T_WALL, CASE_Z0, CASE_Z1, "case-tolex", "case-shell", 0.22, sheet=sheet),
        box("case-top", "case", -CASE_X, CASE_X, CASE_TOP - T_WALL, CASE_TOP, CASE_Z0, CASE_Z1, "case-tolex", "case-shell", 0.22, sheet=sheet),
        box("case-end-treble", "case", -CASE_X, -CASE_X + T_WALL, T_WALL, CASE_TOP - T_WALL, CASE_Z0, CASE_Z1, "case-tolex", "case-shell", 0.22, sheet=sheet),
        box("case-end-bass", "case", CASE_X - T_WALL, CASE_X, T_WALL, CASE_TOP - T_WALL, CASE_Z0, CASE_Z1, "case-tolex", "case-shell", 0.22, sheet=sheet),
        box("case-back", "case", -CASE_X + T_WALL, CASE_X - T_WALL, T_WALL, CASE_TOP - T_WALL, CASE_Z1 - T_WALL, CASE_Z1, "case-tolex", "case-shell", 0.22, sheet=sheet),
        box("lining-floor", "case", -0.24, 0.24, T_WALL, T_WALL + T_LINING, CASE_Z0, 0.125, "case-plush", "case-lining", 0.42, sheet=sheet),
        box("lining-back", "case", -0.24, 0.24, T_WALL + T_LINING, CASE_TOP - T_WALL, 0.125 - T_LINING, 0.125, "case-plush", "case-lining", 0.42, sheet=sheet),
        box("lining-end-treble", "case", -0.24, -0.24 + T_LINING, T_WALL + T_LINING, CASE_TOP - T_WALL, CASE_Z0, 0.117, "case-plush", "case-lining", 0.42, sheet=sheet),
        box("lining-end-bass", "case", 0.24 - T_LINING, 0.24, T_WALL + T_LINING, CASE_TOP - T_WALL, CASE_Z0, 0.117, "case-plush", "case-lining", 0.42, sheet=sheet),
    ]
    if scale == 50:
        c, p = 0.03, 0.0015
        for name, (xa, xb) in (("treble", (-CASE_X - p, -CASE_X + c)), ("bass", (CASE_X - c, CASE_X + p))):
            for level, (ya, yb) in (("low", (0.0, c)), ("high", (CASE_TOP - c, CASE_TOP + p))):
                parts.append(box(f"corner-{name}-{level}-back", "case", xa, xb, ya, yb, CASE_Z1 - c, CASE_Z1 + p, "corner-steel", "case-shell", 0.62))
                parts.append(box(f"corner-{name}-{level}-front", "case", xa, xb, ya, yb, CASE_Z0, CASE_Z0 + c, "corner-steel", "case-shell", 0.62))
        for name, cx in (("treble", -0.16), ("bass", 0.16)):
            parts.append(box(f"latch-{name}", "case", cx - 0.0175, cx + 0.0175, CASE_TOP, CASE_TOP + 0.012, CASE_Z0 + 0.005, CASE_Z0 + 0.030, "corner-steel", "case-shell", 0.7))
    return parts


def case_lid(scale):
    """🚪️ The front lid, lying open on the floor at -z, its plush face up; corner caps at the far edge at 1:50."""
    parts = [box("lid-panel", "lid", -CASE_X, CASE_X, 0.0, LID_T if scale < 200 else 0.030, LID_Z_FAR, CASE_Z0, "case-tolex", "case-lid", 0.24)]
    if scale == 50:
        c, p = 0.03, 0.0015
        for name, (xa, xb) in (("treble", (-CASE_X - p, -CASE_X + c)), ("bass", (CASE_X - c, CASE_X + p))):
            parts.append(box(f"lid-corner-{name}", "lid", xa, xb, 0.0, LID_T + p, LID_Z_FAR - p, LID_Z_FAR + c, "corner-steel", "case-lid", 0.62))
    return parts


def handle(scale):
    """👜️ The leather carrying handle on two riveted lugs, centred on the case top."""
    zc = 0.0125
    if scale == 200:
        return [box("handle-block", "handle", -0.065, 0.065, CASE_TOP, 0.545, zc - 0.015, zc + 0.015, "leather", "case-shell", 0.3)]
    return [
        box("handle-lug-treble", "handle", -0.065, -0.045, CASE_TOP, 0.528, zc - 0.0125, zc + 0.0125, "corner-steel", "case-shell", 0.6),
        box("handle-lug-bass", "handle", 0.045, 0.065, CASE_TOP, 0.528, zc - 0.0125, zc + 0.0125, "corner-steel", "case-shell", 0.6),
        {"id": "handle-grip", "bone": "handle", "kind": "grip", "x": (-0.054, 0.054), "y": (0.527, 0.545), "z": (zc - 0.0126, zc + 0.0126), "material": "leather", "region": "case-shell", "tone": 0.3, "role": "body"},
    ]


def treble(scale):
    """🎹️ The treble box: casing, grille, keyboard of 41 keys (24 white, 17 black), shoulder straps, strap hooks, feet and the bellows straps."""
    parts = []
    xb0, xb1 = X_T - 0.105, X_T
    if scale == 200:
        parts.append(box("treble-box", "treble", xb0, xb1, Y_FOOT, Y_BOX1, -Z_BOX, Z_BOX, "celluloid", "treble-casing", 0.38))
        parts.append(box("grille", "treble", X_T - 0.095, X_T - 0.022, 0.073, 0.438, -0.098, -Z_BOX, "plated-steel", "grille", 0.78, sheet=True))
        parts.append(box("keyboard", "treble", X_T - 0.140, xb0, 0.041, 0.470, -0.090, 0.035, "key-plastic", "keyboard", 0.8))
        return parts
    if scale == 50:
        parts.append(box("treble-box", "treble", xb0, xb1, Y_BOX0, Y_BOX1, -Z_BOX, Z_BOX, "celluloid", "treble-casing", 0.38, r=0.008))
        for side, zc in (("front", -0.06), ("back", 0.06)):
            parts.append(cylinder(f"treble-foot-{side}", "treble", "y", (X_T - 0.055, Y_FOOT, zc), 0.010, 0.009, "rubber", "treble-casing", 0.15))
    else:
        parts.append(box("treble-box", "treble", xb0, xb1, Y_FOOT, Y_BOX1, -Z_BOX, Z_BOX, "celluloid", "treble-casing", 0.38, r=0.008))
    parts.append(box("grille", "treble", X_T - 0.095, X_T - 0.022, 0.073, 0.438, -0.098, -Z_BOX, "plated-steel", "grille", 0.78, sheet=True))
    if scale == 50:
        parts.append(box("key-bed", "treble", X_T - 0.113, xb0, 0.041, 0.470, -0.090, 0.035, "key-bed-wood", "keyboard", 0.18))
        parts.append(box("white-keys", "treble", X_T - 0.125, X_T - 0.113, WHITE_V0, WHITE_V0 + 0.0172, -0.088, 0.004, "key-plastic", "keyboard", 0.86, repeat=24))
        groups, start = [], None
        for i in BLACK_AFTER:
            if start is None or i != prev + 1:
                groups.append([i])
            else:
                groups[-1].append(i)
            start, prev = i, i
        for g in groups:
            yc = WHITE_V0 + (g[0] + 1) * WHITE_PITCH - 0.00025
            parts.append(box(f"black-keys-{g[0]}", "treble", X_T - 0.140, X_T - 0.125, yc - 0.0045, yc + 0.0045, -0.038, 0.004, "key-plastic", "keyboard", 0.08, repeat=len(g), black=g))
        for name, xc in (("right", X_T - 0.0755), ("left", X_T - 0.0255)):
            parts.append(box(f"shoulder-strap-{name}", "treble", xc - 0.0225, xc + 0.0225, 0.058, 0.458, Z_BOX, Z_BOX + 0.008, "leather", "straps", 0.3))
            parts.append(box(f"strap-hook-{name}", "treble", xc - 0.010, xc + 0.010, Y_BOX1, Y_BOX1 + 0.010, 0.075, Z_BOX, "plated-steel", "straps", 0.7))
        for name, (ya, yb) in (("top", (0.443, 0.468)), ("bottom", (0.043, 0.068))):
            parts.append(box(f"bellows-strap-{name}", "treble", X_T - 0.030, X_T + 0.140, ya, yb, -0.098, -0.095, "leather", "straps", 0.28, texture=True))
    else:
        for name, xc in (("right", X_T - 0.0755), ("left", X_T - 0.0255)):
            parts.append(box(f"shoulder-strap-{name}", "treble", xc - 0.0225, xc + 0.0225, 0.058, 0.458, Z_BOX, Z_BOX + 0.015, "leather", "straps", 0.3, draw=False))
        for name, (ya, yb) in (("top", (0.443, 0.468)), ("bottom", (0.043, 0.068))):
            parts.append(box(f"bellows-strap-{name}", "treble", X_T - 0.030, X_T + 0.140, ya, yb, -0.098, -0.095, "leather", "straps", 0.28, texture=True))
        parts.append({"id": "keyboard", "bone": "treble", "kind": "keyboard", "x": (X_T - 0.140, xb0), "y": (0.041, 0.470), "z": (-0.090, 0.035), "material": "key-plastic", "region": "keyboard", "tone": 0.82, "role": "body"})
    return parts


def bellows(scale):
    """🪗️ The bellows in four telescoping segments of 110 mm, each the run of five folds; the segment towards the bass box is the outermost."""
    parts = []
    for k in range(1, SEGMENTS + 1):
        inset = 0.001 * (SEGMENTS - k)
        x0 = X_T + PITCH_CLOSED * (k - 1)
        parts.append(box(f"bellows-segment-{k}", f"bellows-{k}", x0, x0 + SEG_LEN, 0.034 + inset, 0.477 - inset, -0.089 + inset, 0.089 - inset, "bellows-card", "bellows", 0.13))
    return parts


def bass(scale):
    """🎛️ The bass box with 120 buttons in six rows, the air button, the bass strap and two feet."""
    parts = []
    x0 = X_T + PITCH_CLOSED * SEGMENTS
    xe = x0 + 0.140
    if scale == 50:
        parts.append(box("bass-box", "bass", x0, xe, Y_BOX0, Y_BOX1, -Z_BOX, Z_BOX, "celluloid", "bass-casing", 0.38, r=0.008))
        for side, zc in (("front", -0.06), ("back", 0.06)):
            parts.append(cylinder(f"bass-foot-{side}", "bass", "y", (x0 + 0.08, Y_FOOT, zc), 0.010, 0.009, "rubber", "bass-casing", 0.15))
        for j in range(6):
            zc = -0.060 + 0.0155 * j
            yc = 0.083 + 0.004 * j
            parts.append(cylinder(f"bass-buttons-row-{j + 1}", "bass", "x", (xe, yc, zc), 0.010, 0.0045, "key-plastic", "bass-buttons", 0.8 if j > 1 else 0.6, repeat=20))
        parts.append(cylinder("air-button", "bass", "x", (xe, 0.465, 0.055), 0.012, 0.009, "key-plastic", "bass-buttons", 0.75))
    else:
        parts.append(box("bass-box", "bass", x0, xe, Y_FOOT, Y_BOX1, -Z_BOX, Z_BOX, "celluloid", "bass-casing", 0.38, r=0.008 if scale == 100 else 0.0))
        xo = xe + (0.025 if scale == 100 else 0.028)
        if scale == 100:
            parts.append(box("bass-end", "bass", xe, xo, 0.0605, 0.4445, -0.0645, 0.080, "key-plastic", "bass-buttons", 0.6, thermal=False))
            for n, zc in enumerate((-0.045, 0.0)):
                parts.append(box(f"bass-button-rows-{n + 1}", "bass", xe, xe + 0.015, 0.0605, 0.4445, zc - 0.0075, zc + 0.0075, "key-plastic", "bass-buttons", 0.72, body=False))
            parts.append(box("bass-strap", "bass", xe, xo, 0.0605, 0.4445, 0.0285, 0.080, "leather", "straps", 0.3, body=False))
            return parts
        parts.append(box("bass-button-field", "bass", xe, xo, 0.0605, 0.4445, -0.0645, 0.0285, "key-plastic", "bass-buttons", 0.72))
        parts.append(box("bass-strap", "bass", xe, xo, 0.0605, 0.4445, 0.0285, 0.080, "leather", "straps", 0.3))
        return parts
    parts.append({"id": "bass-strap", "bone": "bass", "kind": "strap", "x": (xe, xe + 0.025), "y": (0.068, 0.443), "z": (0.035, 0.080), "material": "leather", "region": "straps", "tone": 0.3, "role": "body"})
    return parts


def parts_of(scale):
    """🧩️ Every part of one authored scale."""
    return case_tub(scale) + case_lid(scale) + handle(scale) + treble(scale) + bellows(scale) + bass(scale)
