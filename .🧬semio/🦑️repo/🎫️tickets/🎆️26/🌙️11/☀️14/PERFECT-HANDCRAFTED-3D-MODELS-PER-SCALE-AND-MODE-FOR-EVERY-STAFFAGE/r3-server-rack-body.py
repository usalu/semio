"""🗄 Round 3, appliances/server-rack: every body solid of every authored scale, stated by hand in world measures (see r3-server-rack-kit.py for the frame)."""
import importlib.util
import os

_spec = importlib.util.spec_from_file_location("r3_server_rack_kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-server-rack-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

STEEL, MESH, CHASSIS, BLANK, RUBBER = "rack-steel", "mesh-steel", "chassis", "blank-steel", "caster-rubber"
SHEET = 0.0012
FRONT_FIELD = (-0.235, 0.221, 0.135, 1.932)
HOLE50, PITCH50 = 0.020, 0.028
HOLE100, PITCH100 = 0.030, 0.045
HOLE200, PITCH200 = 0.030, 0.060


def front_holes(scale):
    """🔳 The perforation of the front door at a scale: the real 70 percent open sheet generalised to holes and webs the scale can show."""
    hole, pitch = {50: (HOLE50, PITCH50), 100: (HOLE100, PITCH100), 200: (HOLE200, PITCH200)}[scale]
    x0, x1, y0, y1 = FRONT_FIELD
    return K.grid(x0, x1, y0, y1, hole, pitch)


def rear_holes(x0, x1):
    """🔳 The perforation of one rear leaf at 1:50: the front pattern shifted by half a pitch, so that a sight line through both doors meets a web."""
    out = []
    for ring in front_holes(50):
        (hx, hy) = ring[0]
        hx, hy = hx + PITCH50 / 2, hy + PITCH50 / 2
        if hx >= x0 and hx + HOLE50 <= x1 and hy + HOLE50 <= FRONT_FIELD[3]:
            out.append([(K.r6(hx), K.r6(hy)), (K.r6(hx + HOLE50), K.r6(hy)), (K.r6(hx + HOLE50), K.r6(hy + HOLE50)), (K.r6(hx), K.r6(hy + HOLE50))])
    return out


def ring_xz(x0, x1, z0, z1, wall):
    """⬛ Outline and hole of a closed rectangular ring in plan."""
    return K.rect(x0, x1, z0, z1), [list(reversed(K.rect(x0 + wall, x1 - wall, z0 + wall, z1 - wall)))]


def ring_xy(x0, x1, y0, y1, wall):
    """⬛ Outline and hole of a closed rectangular ring in elevation."""
    return K.rect(x0, x1, y0, y1), [list(reversed(K.rect(x0 + wall, x1 - wall, y0 + wall, y1 - wall)))]


def side_tray(sid, side, z0, z1, wall, ret, y=(0.070, 1.965), **more):
    """🧱 One side panel as a folded tray standing up: the face sheet at the outside with its front and rear returns, `ret` deep."""
    s = 1 if side > 0 else -1
    xo, xi = s * K.HALF_W, s * (K.HALF_W - ret)
    xs = s * (K.HALF_W - wall)
    if s > 0:
        profile = [(xi, z0), (xo, z0), (xo, z1), (xi, z1), (xi, z1 - wall), (xs, z1 - wall), (xs, z0 + wall), (xi, z0 + wall)]
    else:
        profile = [(xo, z0), (xi, z0), (xi, z0 + wall), (xs, z0 + wall), (xs, z1 - wall), (xi, z1 - wall), (xi, z1), (xo, z1)]
    return K.up(sid, "frame", STEEL, profile, y, **more)


def caster_parts(scale, rear):
    """🛞 One pair of swivel casters (left and right, the second by repeat): top plate, swivel race, fork and a 50 mm wheel trailing outwards."""
    s = 1 if rear else -1
    cx, cz, cw = -K.CASTER_X, s * K.CASTER_SWIVEL_Z, s * K.CASTER_WHEEL_Z
    tag = "r" if rear else "f"
    out = []
    if scale == 50:
        out.append(K.box(f"caster-plate-{tag}", "feet", STEEL, (cx - 0.031, cx + 0.031), (0.062, 0.070), (cz - 0.0475, cz + 0.0475), radius=0.003, tone=0.35))
        out.append(K.turned(f"caster-race-{tag}", "feet", STEEL, [(0, 0.022), (0.008, 0.022)], "y", (cx, 0.054, cz), tone=0.3))
        out.append(K.box(f"caster-crown-{tag}", "feet", STEEL, (cx - 0.018, cx + 0.018), (0.046, 0.054), (cz - 0.016, cz + 0.016), tone=0.3))
        cheek = [(cz - s * 0.016, 0.054), (cz + s * 0.016, 0.054), (cw + s * 0.012, 0.030), (cw + s * 0.010, 0.019), (cw, 0.0135), (cw - s * 0.010, 0.017), (cw - s * 0.012, 0.027), (cz - s * 0.016, 0.046)]
        if not rear:
            cheek = list(reversed(cheek))
        for side, (a, b) in (("a", (cx - 0.018, cx - 0.0105)), ("b", (cx + 0.0105, cx + 0.018))):
            out.append(K.across(f"caster-cheek-{side}-{tag}", "feet", STEEL, [(y, z) for z, y in cheek], (a, b), tone=0.3))
        wheel = [(-0.010, 0.006), (-0.010, 0.0205), (-0.0085, 0.0240), (-0.0055, 0.025), (0.0055, 0.025), (0.0085, 0.0240), (0.010, 0.0205), (0.010, 0.006)]
        out.append(K.turned(f"caster-wheel-{tag}", "feet", RUBBER, wheel, "x", (cx, K.WHEEL_R, cw)))
    elif scale == 100:
        out.append(K.box(f"caster-plate-{tag}", "feet", STEEL, (cx - 0.031, cx + 0.031), (0.054, 0.070), (cz - 0.0475, cz + 0.0475), tone=0.35))
        out.append(K.box(f"caster-fork-{tag}", "feet", STEEL, (cx - 0.018, cx + 0.018), (0.026, 0.054), (min(cz, cw) - 0.016, max(cz, cw) + 0.012), tone=0.3))
        out.append(K.turned(f"caster-wheel-{tag}", "feet", RUBBER, [(-0.010, 0.006), (-0.010, 0.022), (-0.007, 0.025), (0.007, 0.025), (0.010, 0.022), (0.010, 0.006)], "x", (cx, K.WHEEL_R, cw)))
    else:
        out.append(K.box(f"caster-{tag}", "feet", RUBBER, (cx - 0.03, cx + 0.03), (0.0, 0.070), (min(cz, cw) - 0.035, max(cz, cw) + 0.03)))
    for solid in out:
        K.repeat(solid, 2, dx=2 * K.CASTER_X)
    return out


def units(scale):
    """🖥 The equipment stack: the 4U storage array, six 2U servers, patch panel, cable brush and the 1U switch, each a chassis behind a full-width front with its ears."""
    out = []
    fz0, fz1 = (-0.408, -0.400) if scale == 50 else (-0.415, -0.400)
    for uid, first, height, region in K.UNITS:
        y0, y1 = K.unit_y(first, height)
        if uid.startswith("server-") and uid != "server-5":
            continue
        count = 6 if uid == "server-5" else 1
        name = "server" if uid == "server-5" else uid
        depth = {"patch-40": -0.30, "brush-41": -0.30, "switch-42": 0.0}.get(uid, 0.30)
        material = BLANK if region == "blanks" else CHASSIS
        tone = {"storage": 0.4, "server": 0.45, "patch-40": 0.5, "brush-41": 0.38, "switch-42": 0.3}[name]
        if scale == 100 and uid in ("patch-40", "brush-41", "switch-42"):
            if uid != "patch-40":
                continue
            y1, depth, material = K.unit_y(42, 1)[1], 0.0, CHASSIS
            name, tone = "top-group", 0.4
        out.append(K.repeat(K.box(f"{name}-front", "equipment", material, (-K.EAR, K.EAR), (y0, y1), (fz0, fz1), tone=tone), count, dy=2 * K.U))
        out.append(K.repeat(K.box(f"{name}-chassis", "equipment", material, (-K.BODY_X, K.BODY_X), (y0, y1), (fz1, depth), tone=tone - 0.05), count, dy=2 * K.U))
    if scale != 50:
        return out
    s0 = K.unit_y(1, 4)[0]
    for row, dy in (("0", 0.032), ("1", 0.0875)):
        out.append(K.repeat(K.box(f"storage-bay-{row}", "equipment", CHASSIS, (-0.19, -0.138), (s0 + dy, s0 + dy + 0.045), (-0.413, -0.405), tone=0.3), 6, dx=0.0645))
    out.append(K.blob("storage-led", "equipment", CHASSIS, (0.200, s0 + 0.0885, -0.408), (0.0065, 0.0065, 0.0035), tone=0.1, role="glazing"))
    v0 = K.unit_y(5, 2)[0]
    for k, x in enumerate((0.02, 0.0755, 0.131, 0.1865)):
        out.append(K.repeat(K.box(f"server-bay-{k}", "equipment", CHASSIS, (x, x + 0.043), (v0 + 0.0155, v0 + 0.0755), (-0.413, -0.405), tone=0.3), 6, dy=2 * K.U))
    out.append(K.repeat(K.box("server-vent", "equipment", CHASSIS, (-0.205, -0.043), (v0 + 0.0155, v0 + 0.0778), (-0.410, -0.406), tone=0.25, texture=True), 6, dy=2 * K.U))
    out.append(K.repeat(K.blob("server-led", "equipment", CHASSIS, (-0.216, v0 + 0.044, -0.408), (0.0065, 0.0065, 0.0035), tone=0.1, role="glazing"), 6, dy=2 * K.U))
    for side, x in (("a", -0.2365), ("b", 0.2265)):
        out.append(K.repeat(K.box(f"server-handle-{side}", "equipment", CHASSIS, (x, x + 0.010), (v0 + 0.014, v0 + 0.074), (-0.422, -0.406), radius=0.003, tone=0.3), 6, dy=2 * K.U))
        out.append(K.box(f"storage-handle-{side}", "equipment", CHASSIS, (x, x + 0.010), (s0 + 0.03, s0 + 0.146), (-0.422, -0.406), radius=0.003, tone=0.3))
    p0 = K.unit_y(40, 1)[0]
    out.append(K.box("patch-40-ports", "equipment", BLANK, (-0.20, 0.20), (p0 + 0.016, p0 + 0.028), (-0.410, -0.406), tone=0.15, texture=True))
    b0 = K.unit_y(41, 1)[0]
    out.append(K.box("brush-41-slot", "equipment", BLANK, (-0.19, 0.19), (b0 + 0.017, b0 + 0.027), (-0.410, -0.406), tone=0.08, texture=True))
    w0 = K.unit_y(42, 1)[0]
    for row, dy in (("0", 0.0095), ("1", 0.0235)):
        out.append(K.repeat(K.box(f"switch-port-{row}", "equipment", CHASSIS, (-0.19, -0.1745), (w0 + dy, w0 + dy + 0.0105), (-0.410, -0.406), tone=0.12, texture=True), 12, dx=0.0245))
    out.append(K.blob("switch-led", "equipment", CHASSIS, (0.200, w0 + 0.0217, -0.408), (0.0065, 0.0065, 0.0035), tone=0.1, role="glazing"))
    return out


def rails(scale):
    """🪜 The four 19-inch mounting angles (front and rear pair) from the lowest to the highest height unit."""
    out = []
    for tag, zf, s in (("front", K.FRONT_FLANGE_Z, 1), ("rear", K.REAR_FLANGE_Z, -1)):
        for side in (-1, 1):
            name = f"rail-{tag}-{'a' if side < 0 else 'b'}"
            if scale == 50:
                xa, xb, xl = side * 0.225, side * 0.2445, side * 0.2425
                zb, zl = zf + s * 0.002, zf + s * 0.042
                pts = [(xa, zf), (xb, zf), (xb, zl), (xl, zl), (xl, zb), (xa, zb)]
                if (side < 0) != (s < 0):
                    pts = list(reversed(pts))
                out.append(K.up(name, "equipment", STEEL, pts, (K.RAIL_Y0, K.RAIL_Y1), tone=0.55))
            else:
                w = 0.022 if scale == 100 else 0.030
                xa = side * 0.2245
                out.append(K.box(name, "equipment", STEEL, (min(xa, xa + side * w), max(xa, xa + side * w)), (K.RAIL_Y0, K.RAIL_Y1), (min(zf, zf + s * w), max(zf, zf + s * w)), tone=0.55))
    return out


def door(scale):
    """🚪 The front door: a leaf of folded sheet (returns 25 mm deep) with a perforated face, an unperforated stile for the swing handle, three hinges on the left."""
    x0, x1, y0, y1 = -K.DOOR_X, K.DOOR_X, K.DOOR_Y0, K.DOOR_Y1
    zo, zi = -K.HALF_D, -K.HALF_D + K.DOOR_DEPTH
    out = []
    if scale in (50, 100):
        face = 0.0075 if scale == 50 else 0.015
        wall = 0.0015 if scale == 50 else 0.003
        outline, hole = ring_xy(x0, x1, y0, y1, wall)
        out.append(K.deep("door-return", "door", MESH, outline, (zo, zi), holes=hole, tone=0.2))
        out.append(K.deep("door-field", "door", MESH, K.rect(x0 + wall, x1 - wall, y0 + wall, y1 - wall), (zo, zo + face), holes=front_holes(scale)))
        if scale == 50:
            out.append(K.box("door-handle-housing", "door", MESH, (0.226, 0.252), (0.94, 1.10), (-0.5075, -0.500), radius=0.003, tone=0.25))
            out.append(K.box("door-handle", "door", MESH, (0.231, 0.247), (0.95, 1.07), (-0.515, -0.5075), radius=0.003, tone=0.12))
            out.append(K.turned("door-lock", "door", MESH, [(0, 0.007), (0.0075, 0.007), (0.0075, 0.006)], "z", (0.239, 1.085, -0.5075), tone=0.6))
            out[-1]["axis"] = [0, 0, -1]
            out.append(K.repeat(K.turned("door-hinge", "door", MESH, [(0, 0.006), (0.06, 0.006)], "y", (-0.2735, 0.27, -0.4935), tone=0.3), 3, dy=0.72))
        else:
            out.append(K.box("door-handle", "door", MESH, (0.226, 0.252), (0.93, 1.09), (-0.515, -0.500), radius=0.005, tone=0.15))
    else:
        outline, hole = ring_xy(x0, x1, y0, y1, 0.03)
        out.append(K.deep("door-return", "door", MESH, outline, (zo, zi + 0.005), holes=hole, tone=0.2))
        field = K.grid(-0.24, 0.24, 0.13, 1.937, HOLE200, PITCH200)
        out.append(K.deep("door-field", "door", MESH, K.rect(-0.24, 0.24, 0.13, 1.937), (zo + 0.0, zo + 0.03), holes=field))
    return out


def rear_doors(scale):
    """🚪 The split rear door of two perforated leaves with a small swing handle each at the meeting stiles."""
    out = []
    zi, zo = K.HALF_D - K.DOOR_DEPTH, K.HALF_D
    for tag, (x0, x1) in (("a", (-K.DOOR_X, -0.001)), ("b", (0.001, K.DOOR_X))):
        if scale == 50:
            outline, hole = ring_xy(x0, x1, K.DOOR_Y0, K.DOOR_Y1, 0.0015)
            out.append(K.deep(f"rear-door-{tag}-return", "frame", STEEL, outline, (zi, zo), holes=hole, tone=0.42))
            fx0, fx1 = (x0 + 0.035, x1 - 0.045) if tag == "a" else (x0 + 0.045, x1 - 0.035)
            out.append(K.deep(f"rear-door-{tag}", "frame", MESH, K.rect(x0 + 0.0015, x1 - 0.0015, K.DOOR_Y0 + 0.0015, K.DOOR_Y1 - 0.0015), (zo - 0.0075, zo), holes=rear_holes(fx0, fx1), tone=0.25))
            hx = (x1 - 0.026, x1 - 0.010) if tag == "a" else (x0 + 0.010, x0 + 0.026)
            out.append(K.box(f"rear-handle-{tag}", "frame", STEEL, hx, (0.98, 1.10), (zo, zo + 0.003), tone=0.15, texture=True))
        elif scale == 100:
            out.append(K.box(f"rear-door-{tag}", "frame", MESH, (x0, x1), (K.DOOR_Y0, K.DOOR_Y1), (zi, zo), tone=0.25))
        else:
            continue
    if scale == 200:
        out.append(K.box("rear-door", "frame", MESH, (-K.DOOR_X, K.DOOR_X), (K.DOOR_Y0, K.DOOR_Y1), (zo - 0.03, zo), tone=0.25))
    return out


def frame(scale):
    """🏗 The cabinet: welded base and top frames, four corner posts, depth rails, side panels, roof with fans and cable entry, PDUs inside."""
    out = []
    zf = K.FRAME_Z
    if scale in (50, 100):
        outline, hole = ring_xz(-K.POST_X1, K.POST_X1, -zf, zf, 0.04)
        out.append(K.up("base-frame", "frame", STEEL, outline, (K.BASE_Y0, K.BASE_Y1), holes=hole, tone=0.4))
        out.append(K.up("top-frame", "frame", STEEL, outline, (K.TOP_Y0, K.TOP_Y1), holes=hole, tone=0.45))
        if scale == 100:
            out.append(K.up("gland-plate", "frame", STEEL, K.rect(-0.24, 0.24, -0.435, 0.435), (K.BASE_Y0, K.BASE_Y0 + 0.015), tone=0.35))
        if scale == 50:
            out.append(K.up("gland-plate", "frame", STEEL, K.rect(-0.24, 0.24, -0.435, 0.435), (K.BASE_Y0, K.BASE_Y0 + 0.0075), holes=[list(reversed(K.rounded((-0.235, -0.165), (0.15, 0.43), 0.02)))], tone=0.35))
            for side in ("a", "b"):
                xa, xb = (-0.28, -0.245) if side == "a" else (0.245, 0.28)
                post = K.up(f"post-{side}", "frame", STEEL, K.rect(xa, xb, -0.475, -0.435), (K.BASE_Y1, K.TOP_Y0), holes=[list(reversed(K.rect(xa + 0.002, xb - 0.002, -0.473, -0.437)))], tone=0.45)
                out.append(K.repeat(post, 2, dz=0.91))
                xr = (-0.2645, -0.2445) if side == "a" else (0.2445, 0.2645)
                out.append(K.repeat(K.box(f"depth-rail-{side}", "frame", STEEL, xr, (0.10, 0.13), (-0.435, 0.435), tone=0.5), 2, dy=1.807))
            for side in (-1, 1):
                for half, (z0, z1) in (("f", (-zf, -0.0025)), ("r", (0.0025, zf))):
                    out.append(side_tray(f"side-{'a' if side < 0 else 'b'}-{half}", side, z0, z1, SHEET, 0.02))
            out.append(K.repeat(K.box("side-mullion", "frame", STEEL, (-0.28, -0.26), (K.BASE_Y1, K.TOP_Y0), (-0.02, 0.02), tone=0.45), 2, dx=0.54))
            out.append(K.repeat(K.box("side-louvre", "frame", STEEL, (0.30, 0.304), (1.62, 1.632), (0.22, 0.42), tone=0.3, texture=True), 7, dy=0.0517))
            out.append(K.repeat(K.box("pdu", "frame", CHASSIS, (-0.245, -0.19), (0.20, 1.85), (0.37, 0.42), radius=0.004, tone=0.2), 2, dx=0.435))
        else:
            for side in ("a", "b"):
                xa, xb = (-0.28, -0.245) if side == "a" else (0.245, 0.28)
                out.append(K.repeat(K.box(f"post-{side}", "frame", STEEL, (xa, xb), (K.BASE_Y1, K.TOP_Y0), (-0.475, -0.435), tone=0.45), 2, dz=0.91))
            for side in (-1, 1):
                out.append(side_tray(f"side-{'a' if side < 0 else 'b'}", side, -zf, zf, 0.0015, 0.02))
        skirt, inner = ring_xz(-K.HALF_W, K.HALF_W, -zf, zf, 0.0015 if scale == 50 else 0.003)
        out.append(K.up("roof-skirt", "frame", STEEL, skirt, (K.TOP_Y1, K.ROOF_Y1), holes=inner, tone=0.45))
        sheet = 0.0075 if scale == 50 else 0.015
        cut = 0.0015 if scale == 50 else 0.003
        holes = [list(reversed(K.circle(0, z, K.FAN_R, 32))) for z in K.FAN_Z] + [list(reversed(K.rounded((K.SLOT[0], K.SLOT[1]), (K.SLOT[2], K.SLOT[3]), 0.008)))]
        out.append(K.up("roof", "frame", STEEL, K.rect(-K.HALF_W + cut, K.HALF_W - cut, -zf + cut, zf - cut), (K.ROOF_Y1 - sheet, K.ROOF_Y1), holes=holes, tone=0.5))
        fan = K.up("fan-housing", "frame", CHASSIS, K.rounded((-0.086, 0.086), (K.FAN_Z[0] - 0.086, K.FAN_Z[0] + 0.086), 0.008), (K.ROOF_Y1 - sheet - 0.038, K.ROOF_Y1 - sheet), holes=[list(reversed(K.circle(0, K.FAN_Z[0], 0.078, 32)))], tone=0.2)
        out.append(K.repeat(fan, 2, dz=K.FAN_Z[1] - K.FAN_Z[0]))
        if scale == 50:
            hub = K.turned("fan-hub", "frame", CHASSIS, [(0, 0.03), (0.03, 0.03), (0.034, 0.024)], "y", (0, K.ROOF_Y1 - sheet - 0.036, K.FAN_Z[0]), tone=0.15)
            out.append(K.repeat(hub, 2, dz=K.FAN_Z[1] - K.FAN_Z[0]))
            for k, r in enumerate((0.035, 0.055, 0.075)):
                ring = K.turned(f"fan-guard-{k}", "frame", STEEL, [(0, r - 0.0015), (0, r + 0.0015), (0.003, r + 0.0015), (0.003, r - 0.0015), (0, r - 0.0015)], "y", (0, K.ROOF_Y1 - 0.0045, K.FAN_Z[0]), tone=0.6, texture=True)
                out.append(K.repeat(ring, 2, dz=K.FAN_Z[1] - K.FAN_Z[0]))
            for k, (dx, dz) in enumerate(((0.079, 0.0015), (0.0015, 0.079))):
                spoke = K.box(f"fan-spoke-{k}", "frame", STEEL, (-dx, dx), (K.ROOF_Y1 - 0.0045, K.ROOF_Y1 - 0.0015), (K.FAN_Z[0] - dz, K.FAN_Z[0] + dz), tone=0.6, texture=True)
                out.append(K.repeat(spoke, 2, dz=K.FAN_Z[1] - K.FAN_Z[0]))
            out.append(K.box("cable-brush", "frame", STEEL, (K.SLOT[0] + 0.01, K.SLOT[1] - 0.01), (K.ROOF_Y1 - 0.0075, K.ROOF_Y1 - 0.0015), (K.SLOT[2] + 0.004, K.SLOT[3] - 0.004), tone=0.08, texture=True))
    elif scale == 200:
        outline, hole = ring_xz(-0.27, 0.27, -zf, zf, 0.04)
        out.append(K.up("base-frame", "frame", STEEL, outline, (K.BASE_Y0, K.BASE_Y1 + 0.02), holes=hole, tone=0.4))
        for side in (-1, 1):
            out.append(K.box(f"side-{'a' if side < 0 else 'b'}", "frame", STEEL, tuple(sorted((side * 0.27, side * 0.30))), (K.BASE_Y0, 1.94), (-zf, zf), tone=0.5))
        out.append(K.box("roof", "frame", STEEL, (-K.HALF_W, K.HALF_W), (1.94, K.ROOF_Y1), (-zf, zf), tone=0.45))
    return out


def body50():
    """🔍 1:50 (7.5 mm): every assembly of the cabinet down to bays, handles, hinges, fan guards and caster forks."""
    return frame(50) + rear_doors(50) + door(50) + rails(50) + units(50) + caster_parts(50, False) + caster_parts(50, True)


def body100():
    """🔎 1:100 (15 mm): trays, rings, posts and units as plain parts, the door perforation coarsened, casters as plate, fork and wheel."""
    return frame(100) + rear_doors(100) + door(100) + rails(100) + units(100) + caster_parts(100, False) + caster_parts(100, True)


def body200():
    """🔭 1:200 (30 mm): a carcass of side walls, roof, base frame and rear door, the framed door with a coarse perforation, the server block and top group, four caster blocks."""
    out = frame(200) + rear_doors(200) + door(200)
    s0, s1 = K.unit_y(1, 4)[0], K.unit_y(15, 2)[1]
    t0, t1 = K.unit_y(40, 1)[0], K.unit_y(42, 1)[1]
    out.append(K.box("server-block", "equipment", CHASSIS, (-K.EAR, K.EAR), (s0, s1), (-0.43, 0.30), tone=0.42))
    out.append(K.box("top-group", "equipment", CHASSIS, (-K.EAR, K.EAR), (t0, t1), (-0.43, 0.0), tone=0.4))
    return out + caster_parts(200, False) + caster_parts(200, True)


def body500():
    """🛰 1:500 (75 mm): the cabinet block with the two equipment groups seen in the front through the perforated door, standing on a band of casters."""
    s0, s1 = K.RAIL_Y0, K.unit_y(15, 2)[1]
    t0, t1 = K.unit_y(40, 1)[0], K.RAIL_Y1
    windows = [list(reversed(K.rect(-0.2, 0.2, s0, s1))), list(reversed(K.rect(-0.2, 0.2, t0, t1)))]
    out = [K.box("cabinet", "frame", STEEL, (-K.HALF_W, K.HALF_W), (0.075, K.HEIGHT), (-0.415, K.HALF_D), tone=0.5)]
    out.append(K.deep("door-face", "frame", STEEL, K.rect(-K.HALF_W, K.HALF_W, 0.075, K.HEIGHT), (-K.HALF_D, -0.415), holes=windows, tone=0.45))
    out.append(K.box("server-block", "equipment", CHASSIS, (-0.2, 0.2), (s0, s1), (-0.49, -0.415), tone=0.42))
    out.append(K.box("top-group", "equipment", CHASSIS, (-0.2, 0.2), (t0, t1), (-0.49, -0.415), tone=0.4))
    out.append(K.box("casters", "feet", RUBBER, (-0.28, 0.28), (0.0, 0.075), (-0.475, 0.475), tone=0.2))
    return out
