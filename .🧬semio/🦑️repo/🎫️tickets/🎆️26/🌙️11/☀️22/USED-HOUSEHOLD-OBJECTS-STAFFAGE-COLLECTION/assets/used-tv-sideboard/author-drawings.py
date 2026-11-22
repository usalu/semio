"""✏️ The hand-drawn 2D representations of the used TV sideboard at 1:50, 1:100, 1:200 and 1:500 (elevation, front, plan), drawn to the same numbers as the solids.

Executed inside `author-model.py`; `representations()` returns the four authored representations.
"""

S, R, P, E = g.shape, g.rect, g.poly, g.ellipse
PENS = {"50": {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, "100": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, "200": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}, "500": {"body": 0.2, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}
CHIP = "chipboard-white-foil"


def dr(X, Y):
    """🗄️ A rect in the drawer's elevation frame from world ranges."""
    ox, oy, _ = g.FRAMES["drawer"]["o"]
    return R(X[0] - ox, X[1] - ox, Y[0] - oy, Y[1] - oy)


def fl(bone, X, Y):
    """🚪 A rect in a flap's elevation frame from world ranges."""
    return g.flap_profile_rect(bone, X, Y)


def fl_pts(bone, pts):
    """🚪 World (x, y) points in a flap's elevation frame."""
    return P([list(g.local_2d(bone, x, y)) for x, y in pts])


def bar_circle_local(bone, y):
    """⭕ The round bar of a handle seen end on, as an octagon in the bone's elevation frame."""
    cu, cv = g.local_2d(bone, g.BAR_X, y)
    return P([[cu + g.BAR_R * math.cos(k * math.pi / 8), cv + g.BAR_R * math.sin(k * math.pi / 8)] for k in range(16)])


def niche_back(pen, tone, regions=None, hole=True):
    """🕳️ The back panel behind the fronts, seen through the open niche, with the cable hole as a hole."""
    outer = f"M {-g.ZS:.4f} {g.YF:.4f} L {g.ZS:.4f} {g.YF:.4f} L {g.ZS:.4f} {g.YT0:.4f} L {-g.ZS:.4f} {g.YT0:.4f} Z"
    ring = g.circle(0.0, g.HOLE_Y, g.HOLE_R)[::-1]
    inner = " M " + " L ".join(f"{p[0]:.4f} {p[1]:.4f}" for p in ring) + " Z"
    d = outer + (inner if hole else "")
    return S("niche-back", "carcass", "body", 1, tone, {"kind": "path", "d": d}, region="back-panel" if regions is None else None, regions=regions, material="hdf-back", penMm=pen)


def foot_front(z, simple):
    """🦶 A glide foot seen from ahead."""
    pts = g.foot_poly_elev(z, simple)["points"]
    return {"kind": "poly", "points": pts, "closed": True}


def knob_plan(simple):
    """🍄 The drawer knob seen from above: its turned outline about the front normal."""
    return P(g.knob_poly_side(simple, g.XD, 0.0))


def knob_side(simple):
    """🍄 The drawer knob seen from the right (or cut through its axis), in the drawer's elevation frame."""
    ox, oy, _ = g.FRAMES["drawer"]["o"]
    return P([[x - ox, y - oy] for x, y in g.knob_poly_side(simple, g.XD, g.KNOB_Y)])


def handle_r_front():
    """🔩 The bar handle of the right flap seen from ahead."""
    return {"kind": "capsule", "x0": -g.DOOR_ZC - g.BAR_HALF, "y0": g.HANDLE_Y_DOOR, "x1": -g.DOOR_ZC + g.BAR_HALF, "y1": g.HANDLE_Y_DOOR, "r0": g.BAR_R, "r1": g.BAR_R}


def flap_l_front_shapes(scale):
    """🚪 The left flap in front and plan: every shape a lateral sweep of its side profile, so it lies down with the flap."""
    out = [S("door-l", "flap-l", "body", 4, 0.93, fl("flap-l", (g.XF, g.XD), (g.DOOR_Y0, g.DOOR_Y1)), region="fronts" if scale == "50" else None, regions=None if scale == "50" else ["fronts", "handles"], material=CHIP, penMm={"50": 0.35, "100": 0.25, "200": None, "500": None}[scale], solid=g.sweep_w((g.DOOR_Z0, g.DOOR_Z1)))]
    if scale == "50":
        out.append(S("handle-l-bar", "flap-l", "detail", 6, 0.5, bar_circle_local("flap-l", g.HANDLE_Y_DOOR), region="handles", material="aluminium-handle", solid=g.sweep_w((g.DOOR_ZC - g.BAR_HALF - g.BAR_R, g.DOOR_ZC + g.BAR_HALF + g.BAR_R))))
        for k, zc in enumerate((g.DOOR_ZC - g.LEG_C, g.DOOR_ZC + g.LEG_C)):
            out.append(S(f"handle-l-leg-{k + 1}", "flap-l", "detail", 5.5, 0.5, fl("flap-l", (g.XD, g.BAR_X), (g.HANDLE_Y_DOOR - g.LEG_HALF, g.HANDLE_Y_DOOR + g.LEG_HALF)), region="handles", material="aluminium-handle", solid=g.sweep_w((zc - g.LEG_HALF, zc + g.LEG_HALF))))
    return out


def elevation50():
    """✂️ Section A-A at the centre plane (z = 0), looking at the left half: cut top, back with the cable hole, bottom, niche shelf, drawer and drawer handle, middle feet; beyond them the left divider, the drawer side, the edge of the left flap and its handle."""
    out = [
        S("divider-beyond", "carcass", "body", 1, 0.86, R(g.XB, g.XF, g.YB1, g.YT0), region="carcass-foil", material=CHIP, penMm=0.25),
        S("back-beyond", "carcass", "body", 1.5, 0.72, R(g.X0, g.XB, g.YF, g.YT0), region="back-panel", material="hdf-back", penMm=0.25),
        S("top-cut", "carcass", "body", 6, 0.3, R(g.X0, g.XD, g.YT0, g.YT1), region="top-foil", material="chipboard-yellowed-foil"),
        S("back-cut-low", "carcass", "body", 6, 0.3, R(g.X0, g.XB, g.YF, g.HOLE_Y - g.HOLE_R), region="back-panel", material="hdf-back", penMm=0.25),
        S("back-cut-high", "carcass", "body", 6, 0.3, R(g.X0, g.XB, g.HOLE_Y + g.HOLE_R, g.YT0), region="back-panel", material="hdf-back", penMm=0.25),
        S("bottom-cut", "carcass", "body", 6, 0.3, R(g.XB, g.XF, g.YF, g.YB1), region="carcass-foil", material=CHIP),
        S("mid-shelf-cut", "carcass", "body", 6, 0.3, R(g.XB, g.XF, g.YM0, g.YM1), region="carcass-foil", material=CHIP),
        S("foot-front-cut", "carcass", "body", 5.8, 0.3, g.foot_poly_elev(g.FOOT_XF, False), region="feet", material="pp-feet", penMm=0.25),
        S("foot-back-cut", "carcass", "body", 5.8, 0.3, g.foot_poly_elev(g.FOOT_XB, False), region="feet", material="pp-feet", penMm=0.25),
        S("drawer-side-beyond", "drawer", "body", 3, 0.8, dr((g.DRW_X0, g.XF), (g.BOX_Y0, g.BOX_Y1)), material=CHIP, penMm=0.25),
        S("drawer-front-cut", "drawer", "body", 7, 0.3, dr((g.XF, g.XD), (g.DRW_Y0, g.DRW_Y1)), region="fronts", material=CHIP),
        S("drawer-back-cut", "drawer", "body", 7, 0.3, dr((g.DRW_X0, g.DRW_X0 + g.BOX_T), (g.BOX_Y0, g.BOX_Y1)), material=CHIP, penMm=0.25),
        S("drawer-bottom-cut", "drawer", "body", 7, 0.3, dr((g.DRW_X0 + g.BOX_T, g.XF), (g.BOX_Y0, g.BOX_Y0 + g.TB)), material="hdf-back", penMm=0.25),
        S("knob-d-cut", "drawer", "detail", 7, 0.3, knob_side(False), region="knob", material="beech-knob"),
        S("door-l", "flap-l", "body", 4, 0.85, fl("flap-l", (g.XF, g.XD), (g.DOOR_Y0, g.DOOR_Y1)), region="fronts", material=CHIP, penMm=0.35),
        S("handle-l-leg-2", "flap-l", "detail", 5, 0.5, fl("flap-l", (g.XD, g.BAR_X), (g.HANDLE_Y_DOOR - g.LEG_HALF, g.HANDLE_Y_DOOR + g.LEG_HALF)), region="handles", material="aluminium-handle"),
        S("handle-l-bar", "flap-l", "detail", 5.5, 0.5, bar_circle_local("flap-l", g.HANDLE_Y_DOOR), region="handles", material="aluminium-handle"),
    ]
    dx, dy = g.FRAMES["drawer"]["o"][:2]
    out.append(S("drawer-runner-hidden", "drawer", "texture", 3.5, 0.5, P([[g.RUN_X0 - dx, g.RUN_Y1 - dy], [g.XF - dx, g.RUN_Y1 - dy]], closed=False), filled=False))
    return out


def front50():
    """🖼️ Front elevation 1:50 (seen from ahead): top edge, carcass edges, the niche with shelf edge and cable hole, three fronts with handles, the replacement knob, the screw holes of the lost handle, the chip, tape strips, scuffs and feet."""
    out = [
        niche_back(0.25, 0.6),
        S("side-l", "carcass", "body", 2, 0.88, R(g.ZSI, g.ZS, g.YF, g.YT0), region="carcass-foil", material=CHIP, penMm=0.25),
        S("side-r", "carcass", "body", 2, 0.88, R(-g.ZS, -g.ZSI, g.YF, g.YT0), region="carcass-foil", material=CHIP, penMm=0.25),
        S("bottom-edge", "carcass", "body", 2, 0.84, R(-g.ZSI, g.ZSI, g.YF, g.YB1), region="carcass-foil", material=CHIP, penMm=0.25),
        S("divider-l", "carcass", "body", 2, 0.84, R(g.ZDI, g.ZDO, g.YB1, g.YT0), region="carcass-foil", material=CHIP, penMm=0.25),
        S("divider-r", "carcass", "body", 2, 0.84, R(-g.ZDO, -g.ZDI, g.YB1, g.YT0), region="carcass-foil", material=CHIP, penMm=0.25),
        S("mid-shelf-edge", "carcass", "body", 2, 0.84, R(-g.ZDI, g.ZDI, g.YM0, g.YM1), region="carcass-foil", material=CHIP, penMm=0.25),
        S("top-edge", "carcass", "body", 5, 0.86, R(-g.ZS, g.ZS, g.YT0, g.YT1), region="top-foil", material="chipboard-yellowed-foil", penMm=0.35),
        S("tape-top-l", "carcass", "texture", 6, 0.72, R(g.TAPE_L[0], g.TAPE_L[1], g.DOOR_Y1 + 0.001, g.YT1), material="packing-tape", filled=False),
        S("tape-top-r", "carcass", "texture", 6, 0.72, R(g.TAPE_R[0], g.TAPE_R[1], g.DOOR_Y1 + 0.001, g.YT1), material="packing-tape", filled=False),
        S("tape-shelf", "carcass", "texture", 3, 0.72, R(g.TAPE_D[0], g.TAPE_D[1], g.YM0, g.YM1), material="packing-tape", filled=False),
        S("scuff-top-1", "carcass", "texture", 6.5, 0.5, P([[-0.64, 0.4415], [-0.57, 0.4425]], closed=False), filled=False),
        S("scuff-top-2", "carcass", "texture", 6.5, 0.5, P([[0.04, 0.4405], [0.13, 0.4418]], closed=False), filled=False),
        S("scuff-top-3", "carcass", "texture", 6.5, 0.5, P([[0.31, 0.443], [0.355, 0.4425]], closed=False), filled=False),
    ]
    for i, z in enumerate((g.FOOT_Z, 0.0, -g.FOOT_Z)):
        out.append(S(f"foot-{('left', 'middle', 'right')[i]}", "carcass", "body", 1, 0.35, foot_front(z, False), region="feet", material="pp-feet", penMm=0.25))
    out += flap_l_front_shapes("50")
    out.append(S("tape-door-l", "flap-l", "texture", 5, 0.72, fl("flap-l", (g.XD, g.XD + g.TAPE), (0.33, g.DOOR_Y1)), material="packing-tape", solid=g.sweep_w(g.TAPE_L), filled=False))
    out += [
        S("door-r", "flap-r", "body", 4, 0.93, R(-g.DOOR_Z1, -g.DOOR_Z0, g.DOOR_Y0, g.DOOR_Y1), region="fronts", material=CHIP, penMm=0.35),
        S("door-r-chip", "flap-r", "detail", 4.5, 0.45, P([[-g.DOOR_Z1, g.DOOR_Y1], [-g.DOOR_Z1 + 0.03, g.DOOR_Y1], [-g.DOOR_Z1, g.DOOR_Y1 - 0.012]]), material=CHIP),
        S("handle-r-bar", "flap-r", "detail", 5.5, 0.5, handle_r_front(), region="handles", material="aluminium-handle"),
        S("tape-door-r", "flap-r", "texture", 5, 0.72, R(g.TAPE_R[0], g.TAPE_R[1], 0.33, g.DOOR_Y1), material="packing-tape", filled=False),
        S("scuff-door-r-1", "flap-r", "texture", 5, 0.5, P([[-0.885, 0.05], [-0.84, 0.052]], closed=False), filled=False),
        S("scuff-door-r-2", "flap-r", "texture", 5, 0.5, P([[-0.80, 0.047], [-0.76, 0.048]], closed=False), filled=False),
        S("drawer-front", "drawer", "body", 4, 0.93, R(-g.DRW_Z, g.DRW_Z, g.DRW_Y0, g.DRW_Y1), region="fronts", material=CHIP, penMm=0.35),
        S("knob-d", "drawer", "detail", 5.5, 0.55, E(0.0, g.KNOB_Y, 0.015), region="knob", material="beech-knob"),
        S("screw-hole-1", "drawer", "texture", 5, 0.2, E(-g.LEG_C, g.HANDLE_Y_DRAWER, 0.002), filled=False),
        S("screw-hole-2", "drawer", "texture", 5, 0.2, E(g.LEG_C, g.HANDLE_Y_DRAWER, 0.002), filled=False),
        S("tape-drawer", "drawer", "texture", 5, 0.72, R(g.TAPE_D[0], g.TAPE_D[1], 0.13, g.DRW_Y1), material="packing-tape", filled=False),
        S("scuff-drawer-1", "drawer", "texture", 5, 0.5, P([[-0.22, 0.047], [-0.15, 0.0485]], closed=False), filled=False),
        S("scuff-drawer-2", "drawer", "texture", 5, 0.5, P([[0.05, 0.046], [0.11, 0.047]], closed=False), filled=False),
    ]
    return out


def plan50():
    """🗺️ Plan 1:50: the top with its traces (ring marks, the paler patch where the TV foot stood, scratches, the depot price label, the tape ends), the handles and the knob beyond the front edge; the fronts and the drawer box under the top for the open poses."""
    out = [
        S("top", "carcass", "body", 5, 0.88, R(g.X0, g.XD, -g.ZS, g.ZS), region="top-foil", material="chipboard-yellowed-foil", penMm=0.35),
        S("tv-patch", "carcass", "texture", 6, 0.5, R(0.10, 0.30, -0.30, 0.30), filled=False),
        S("ring-mug", "carcass", "texture", 6, 0.5, E(0.15, 0.62, 0.04), filled=False),
        S("ring-pot", "carcass", "texture", 6, 0.5, E(0.21, -0.55, 0.095), filled=False),
        S("ring-pot-2", "carcass", "texture", 6, 0.5, E(0.225, -0.535, 0.095), filled=False),
        S("scratch-1", "carcass", "texture", 6, 0.5, P([[0.33, -0.12], [0.36, 0.05]], closed=False), filled=False),
        S("scratch-2", "carcass", "texture", 6, 0.5, P([[0.35, 0.18], [0.37, 0.26]], closed=False), filled=False),
        S("price-label", "carcass", "texture", 6.5, 0.97, R(g.LABEL[0][0], g.LABEL[0][1], g.LABEL[1][0], g.LABEL[1][1]), material="paper-label"),
        S("tape-top-l", "carcass", "texture", 6.5, 0.72, R(0.30, g.XD + g.TAPE, g.TAPE_L[0], g.TAPE_L[1]), material="packing-tape", filled=False),
        S("tape-top-r", "carcass", "texture", 6.5, 0.72, R(0.30, g.XD + g.TAPE, g.TAPE_R[0], g.TAPE_R[1]), material="packing-tape", filled=False),
    ]
    out += flap_l_front_shapes("50")
    out += [
        S("door-r", "flap-r", "body", 1, 0.93, R(g.XF, g.XD, -g.DOOR_Z1, -g.DOOR_Z0), region="fronts", material=CHIP, penMm=0.35),
        S("handle-r-legs", "flap-r", "detail", 5.4, 0.5, R(g.XD, g.BAR_X, -g.DOOR_ZC - g.LEG_C - g.LEG_HALF, -g.DOOR_ZC - g.LEG_C + g.LEG_HALF), region="handles", material="aluminium-handle"),
        S("handle-r-legs-2", "flap-r", "detail", 5.4, 0.5, R(g.XD, g.BAR_X, -g.DOOR_ZC + g.LEG_C - g.LEG_HALF, -g.DOOR_ZC + g.LEG_C + g.LEG_HALF), region="handles", material="aluminium-handle"),
        S("handle-r-bar", "flap-r", "detail", 5.5, 0.5, {"kind": "capsule", "x0": g.BAR_X, "y0": -g.DOOR_ZC - g.BAR_HALF, "x1": g.BAR_X, "y1": -g.DOOR_ZC + g.BAR_HALF, "r0": g.BAR_R, "r1": g.BAR_R}, region="handles", material="aluminium-handle"),
        S("drawer-side-l", "drawer", "body", 0.5, 0.84, R(g.DRW_X0, g.XF, g.BOX_ZI, g.BOX_ZO), material=CHIP, penMm=0.25),
        S("drawer-side-r", "drawer", "body", 0.5, 0.84, R(g.DRW_X0, g.XF, -g.BOX_ZO, -g.BOX_ZI), material=CHIP, penMm=0.25),
        S("drawer-back", "drawer", "body", 0.5, 0.84, R(g.DRW_X0, g.DRW_X0 + g.BOX_T, -g.BOX_ZI, g.BOX_ZI), material=CHIP, penMm=0.25),
        S("drawer-bottom", "drawer", "body", 0.4, 0.78, R(g.DRW_X0 + g.BOX_T, g.XF, -g.BOX_ZI, g.BOX_ZI), material="hdf-back", penMm=0.25),
        S("drawer-front", "drawer", "body", 1, 0.93, R(g.XF, g.XD, -g.DRW_Z, g.DRW_Z), region="fronts", material=CHIP, penMm=0.35),
        S("knob-d", "drawer", "detail", 5.5, 0.55, knob_plan(False), region="knob", material="beech-knob"),
    ]
    return out


def side_plain(scale):
    """👁️ The plain side elevation seen from the right (1:100, 1:200, 1:500): right side panel, back edge, top edge, the edge of the right flap with its knob, feet or plinth."""
    pen = {"100": 0.25, "200": None, "500": None}[scale]
    if scale == "500":
        mass = P([[0, 0], [g.XF, 0], [g.XF, g.YT0], [g.XD, g.YT0], [g.XD, g.YT1], [0, g.YT1]])
        return [
            S("carcass-mass", "carcass", "body", 3, 0.88, mass, regions=["carcass-foil", "back-panel", "feet"], material=CHIP),
            S("top-edge", "carcass", "body", 4, 0.86, R(g.X0, g.XD, g.YT0, g.YT1), region="top-foil", material="chipboard-yellowed-foil"),
            S("door-r", "flap-r", "body", 4, 0.85, fl("flap-r", (g.XF, g.XD), (g.DOOR_Y0, g.DOOR_Y1)), regions=["fronts", "handles"], material=CHIP),
            S("knob-d", "drawer", "body", 5, 0.55, knob_side(True), region="knob", material="beech-knob"),
        ]
    y0 = g.YF if scale == "100" else 0.0
    out = [
        S("side-r", "carcass", "body", 3, 0.88, R(g.XB, g.XF, y0, g.YT0), region="carcass-foil", material=CHIP, penMm=pen),
        S("back-edge", "carcass", "body", 3, 0.72, R(g.X0, g.XB, g.YF, g.YT0), region="back-panel", material="hdf-back", penMm=pen),
        S("top-edge", "carcass", "body", 4, 0.86, R(g.X0, g.XD, g.YT0, g.YT1), region="top-foil", material="chipboard-yellowed-foil", penMm=pen),
        S("door-r", "flap-r", "body", 4, 0.85, fl("flap-r", (g.XF, g.XD), (g.DOOR_Y0, g.DOOR_Y1)), regions=["fronts", "handles"], material=CHIP, penMm=pen),
        S("knob-d", "drawer", "detail", 5, 0.55, knob_side(True), region="knob", material="beech-knob"),
    ]
    if scale == "100":
        out.append(S("foot-front", "carcass", "body", 2, 0.35, g.foot_poly_elev(g.FOOT_XF, True), region="feet", material="pp-feet", penMm=pen))
        out.append(S("foot-back", "carcass", "body", 2, 0.35, g.foot_poly_elev(g.FOOT_XB, True), region="feet", material="pp-feet", penMm=pen))
    else:
        out.append(S("plinth", "carcass", "body", 3.5, 0.45, R(g.XB, g.XF, 0.0, g.YF), region="feet", material="pp-feet", penMm=pen))
    return out


def front_coarse(scale):
    """🖼️ The front elevation at 1:100 and 1:200: carcass edges, niche with cable hole, three fronts, knob; 1:100 keeps feet and tape, 1:200 merges the feet into the plinth."""
    pen = {"100": 0.25, "200": None}[scale]
    y0 = g.YF if scale == "100" else 0.0
    out = [
        niche_back(pen, 0.6),
        S("side-l", "carcass", "body", 2, 0.88, R(g.ZSI, g.ZS, y0, g.YT0), region="carcass-foil", material=CHIP, penMm=pen),
        S("side-r", "carcass", "body", 2, 0.88, R(-g.ZS, -g.ZSI, y0, g.YT0), region="carcass-foil", material=CHIP, penMm=pen),
        S("divider-l", "carcass", "body", 2, 0.84, R(g.ZDI, g.ZDO, g.YB1, g.YT0), region="carcass-foil", material=CHIP, penMm=pen),
        S("divider-r", "carcass", "body", 2, 0.84, R(-g.ZDO, -g.ZDI, g.YB1, g.YT0), region="carcass-foil", material=CHIP, penMm=pen),
        S("mid-shelf-edge", "carcass", "body", 2, 0.84, R(-g.ZDI, g.ZDI, g.YM0, g.YM1), region="carcass-foil", material=CHIP, penMm=pen),
        S("top-edge", "carcass", "body", 5, 0.86, R(-g.ZS, g.ZS, g.YT0, g.YT1), region="top-foil", material="chipboard-yellowed-foil", penMm=pen),
    ]
    if scale == "100":
        out.append(S("bottom-edge", "carcass", "body", 2, 0.84, R(-g.ZSI, g.ZSI, g.YF, g.YB1), region="carcass-foil", material=CHIP, penMm=pen))
        for i, z in enumerate((g.FOOT_Z, 0.0, -g.FOOT_Z)):
            out.append(S(f"foot-{('left', 'middle', 'right')[i]}", "carcass", "body", 1, 0.35, foot_front(z, True), region="feet", material="pp-feet", penMm=pen))
        out += [
            S("tape-top-l", "carcass", "texture", 6, 0.72, R(g.TAPE_L[0], g.TAPE_L[1], g.DOOR_Y1 + 0.001, g.YT1), material="packing-tape", filled=False),
            S("tape-top-r", "carcass", "texture", 6, 0.72, R(g.TAPE_R[0], g.TAPE_R[1], g.DOOR_Y1 + 0.001, g.YT1), material="packing-tape", filled=False),
            S("tape-shelf", "carcass", "texture", 3, 0.72, R(g.TAPE_D[0], g.TAPE_D[1], g.YM0, g.YM1), material="packing-tape", filled=False),
        ]
    else:
        out.append(S("plinth", "carcass", "body", 2, 0.45, R(-g.ZSI, g.ZSI, 0.0, g.YB1), regions=["feet", "carcass-foil"], material="pp-feet", penMm=pen))
    out += flap_l_front_shapes(scale)
    out += [
        S("door-r", "flap-r", "body", 4, 0.93, R(-g.DOOR_Z1, -g.DOOR_Z0, g.DOOR_Y0, g.DOOR_Y1), regions=["fronts", "handles"], material=CHIP, penMm=pen),
        S("drawer-front", "drawer", "body", 4, 0.93, R(-g.DRW_Z, g.DRW_Z, g.DRW_Y0, g.DRW_Y1), region="fronts", material=CHIP, penMm=pen),
        S("knob-d", "drawer", "detail", 5.5, 0.55, E(0.0, g.KNOB_Y, 0.015), region="knob", material="beech-knob"),
    ]
    if scale == "100":
        out += [
            S("tape-door-l", "flap-l", "texture", 5, 0.72, fl("flap-l", (g.XD, g.XD + g.TAPE), (0.33, g.DOOR_Y1)), material="packing-tape", solid=g.sweep_w(g.TAPE_L), filled=False),
            S("tape-door-r", "flap-r", "texture", 5, 0.72, R(g.TAPE_R[0], g.TAPE_R[1], 0.33, g.DOOR_Y1), material="packing-tape", filled=False),
            S("tape-drawer", "drawer", "texture", 5, 0.72, R(g.TAPE_D[0], g.TAPE_D[1], 0.13, g.DRW_Y1), material="packing-tape", filled=False),
        ]
    return out


def plan_coarse(scale):
    """🗺️ The plan at 1:100, 1:200 and 1:500: the top, the knob beyond the front edge, the fronts and the drawer under the top for the open poses; 1:100 keeps label, tape ends and the large ring mark."""
    pen = {"100": 0.25, "200": None, "500": None}[scale]
    out = [S("top", "carcass", "body", 5, 0.88, R(g.X0, g.XD, -g.ZS, g.ZS), region="top-foil", material="chipboard-yellowed-foil", penMm=pen)]
    if scale == "100":
        out += [
            S("price-label", "carcass", "texture", 6.5, 0.97, R(g.LABEL[0][0], g.LABEL[0][1], g.LABEL[1][0], g.LABEL[1][1]), material="paper-label"),
            S("tape-top-l", "carcass", "texture", 6.5, 0.72, R(0.30, g.XD + g.TAPE, g.TAPE_L[0], g.TAPE_L[1]), material="packing-tape", filled=False),
            S("tape-top-r", "carcass", "texture", 6.5, 0.72, R(0.30, g.XD + g.TAPE, g.TAPE_R[0], g.TAPE_R[1]), material="packing-tape", filled=False),
            S("ring-pot", "carcass", "texture", 6, 0.5, E(0.21, -0.55, 0.095), filled=False),
        ]
    out += flap_l_front_shapes(scale)
    out += [
        S("door-r", "flap-r", "body", 1, 0.93, R(g.XF, g.XD, -g.DOOR_Z1, -g.DOOR_Z0), regions=["fronts", "handles"], material=CHIP, penMm=pen),
        S("drawer-box", "drawer", "body", 0.5, 0.84, R(g.DRW_X0, g.XF, -g.BOX_ZO, g.BOX_ZO), material=CHIP, penMm=pen),
        S("drawer-front", "drawer", "body", 1, 0.93, R(g.XF, g.XD, -g.DRW_Z, g.DRW_Z), region="fronts", material=CHIP, penMm=pen),
        S("knob-d", "drawer", "body" if scale == "500" else "detail", 5.5, 0.55, knob_plan(True), region="knob", material="beech-knob"),
    ]
    return out


def front500():
    """🖼️ The front elevation 1:500: the carcass mass with its top, the dark open niche, the three fronts and the knob."""
    return [
        S("carcass-mass", "carcass", "body", 2, 0.88, R(-g.ZS, g.ZS, 0.0, g.YT0), regions=["carcass-foil", "feet"], material=CHIP),
        S("niche", "carcass", "body", 3, 0.55, R(-g.ZDI, g.ZDI, g.YM1, g.YT0), region="back-panel", material="hdf-back"),
        S("top-edge", "carcass", "body", 5, 0.86, R(-g.ZS, g.ZS, g.YT0, g.YT1), region="top-foil", material="chipboard-yellowed-foil"),
    ] + flap_l_front_shapes("500") + [
        S("door-r", "flap-r", "body", 4, 0.93, R(-g.DOOR_Z1, -g.DOOR_Z0, g.DOOR_Y0, g.DOOR_Y1), regions=["fronts", "handles"], material=CHIP),
        S("drawer-front", "drawer", "body", 4, 0.93, R(-g.DRW_Z, g.DRW_Z, g.DRW_Y0, g.DRW_Y1), region="fronts", material=CHIP),
        S("knob-d", "drawer", "body", 5.5, 0.55, E(0.0, g.KNOB_Y, 0.015), region="knob", material="beech-knob"),
    ]


def decisions(scale):
    """📝 The eight authored decisions of one representation."""
    return {k: t(f"rep{scale}-{k}") for k in ("silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays")}


def representations():
    """🖨️ The four authored representations."""
    geo = {
        "50": {"elevation": elevation50(), "front": front50(), "plan": plan50()},
        "100": {"elevation": side_plain("100"), "front": front_coarse("100"), "plan": plan_coarse("100")},
        "200": {"elevation": side_plain("200"), "front": front_coarse("200"), "plan": plan_coarse("200")},
        "500": {"elevation": side_plain("500"), "front": front500(), "plan": plan_coarse("500")},
    }
    out = {}
    for sc, gm in geo.items():
        rep = {"status": "authored", "denominator": int(sc), "validRange": {"minDenominator": RANGES[sc][0], "maxDenominator": RANGES[sc][1]}, "pens": PENS[sc], "hatches": [], "geometry": gm}
        if sc == "50":
            rep["sections"] = {"elevation": {"plane": 0.0, "label": {"en": "A-A", "de": "A-A"}}}
        rep["decisions"] = decisions(sc)
        out[sc] = rep
    return out
