"""🗄 Round 3, appliances/server-rack: the 2D shapes of every scale and view, drawn from the same measures as the solids (ruling E1). Elevation shapes are bone-local (x along the upright bone, y = -world x), front shapes are (world z, world y), plan shapes (world x, world z)."""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def _load(name):
    spec = importlib.util.spec_from_file_location("r3_server_rack_" + name, os.path.join(HERE, f"r3-server-rack-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = _load("kit")
B = _load("body")
r6 = K.r6


def shape(sid, bone, role, z, filled, tone, geometry, **more):
    """🖊 A shape with its common members first."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geometry)
    out.update(more)
    return out


def erect(bone, x0, x1, y0, y1, r=0.0):
    """⬜ A world rectangle of the side elevation in the frame of an upright bone."""
    oy = K.ORIGIN[bone]
    out = {"kind": "rect", "x": r6(y0 - oy), "y": r6(-x1), "w": r6(y1 - y0), "h": r6(x1 - x0)}
    if r:
        out["r"] = r
    return out


def epoints(bone, points):
    """📍 World (x, y) points of the side elevation in the frame of an upright bone."""
    oy = K.ORIGIN[bone]
    return [[r6(y - oy), r6(-x)] for x, y in points]


def epoly(bone, points):
    """🔷 A closed world polygon of the side elevation."""
    return {"kind": "poly", "points": epoints(bone, points), "closed": True}


def epath(bone, rings, closed=True):
    """🛤 Rings (or open runs) of world (x, y) points as one path of the side elevation."""
    parts = []
    for ring in rings:
        pts = epoints(bone, ring)
        parts.append("M " + " L ".join(f"{p[0]:g} {p[1]:g}" for p in pts) + (" Z" if closed else ""))
    return {"kind": "path", "d": " ".join(parts)}


def eellipse(bone, x, y, r):
    """⚪ A world circle of the side elevation."""
    return {"kind": "ellipse", "cx": r6(y - K.ORIGIN[bone]), "cy": r6(-x), "rx": r, "ry": r}


def rect(x0, x1, y0, y1, r=0.0):
    """⬜ A rectangle of the front or the plan."""
    out = {"kind": "rect", "x": r6(x0), "y": r6(y0), "w": r6(x1 - x0), "h": r6(y1 - y0)}
    if r:
        out["r"] = r
    return out


def path(rings, closed=True):
    """🛤 Rings of the front or the plan as one path."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{r6(x):g} {r6(y):g}" for x, y in ring) + (" Z" if closed else "") for ring in rings)}


def poly(points, closed=True):
    """🔷 A polygon of the front or the plan."""
    return {"kind": "poly", "points": [[r6(x), r6(y)] for x, y in points], "closed": closed}


def caster_elevation(cx, scale):
    """🛞 The outline of one caster seen from the front of the cabinet: wheel under the fork, race and top plate."""
    if scale == 50:
        half = [(0.010, 0.0), (0.010, 0.014), (0.018, 0.016), (0.018, 0.054), (0.022, 0.054), (0.022, 0.062), (0.031, 0.062), (0.031, 0.070)]
    elif scale == 100:
        half = [(0.010, 0.0), (0.010, 0.026), (0.018, 0.026), (0.018, 0.054), (0.031, 0.054), (0.031, 0.070)]
    else:
        half = [(0.03, 0.0), (0.03, 0.07)]
    right = [(cx + dx, y) for dx, y in half]
    left = [(cx - dx, y) for dx, y in reversed(half)]
    return right + left


def caster_side(rear, scale):
    """🛞 One caster seen from the side: the fork (plate, race and cheek) and the wheel."""
    s = 1 if rear else -1
    cz, cw = s * K.CASTER_SWIVEL_Z, s * K.CASTER_WHEEL_Z
    if scale == 50:
        rel = [(-0.0475, 0.070), (0.0475, 0.070), (0.0475, 0.062), (0.022, 0.062), (0.022, 0.054), (0.016, 0.054), (0.037, 0.030), (0.035, 0.019), (0.025, 0.0135), (0.015, 0.017), (0.013, 0.027), (-0.016, 0.046), (-0.016, 0.054), (-0.022, 0.054), (-0.022, 0.062), (-0.0475, 0.062)]
        fork = [(cz + s * dz, y) for dz, y in rel]
        if s < 0:
            fork = list(reversed(fork))
        return fork, (cw, K.WHEEL_R, K.WHEEL_R)
    if scale == 100:
        lo, hi = min(cz, cw) - 0.016, max(cz, cw) + 0.012
        fork = [(cz - 0.0475, 0.070), (cz + 0.0475, 0.070), (cz + 0.0475, 0.054), (hi, 0.054), (hi, 0.026), (lo, 0.026), (lo, 0.054), (cz - 0.0475, 0.054)]
        return fork, (cw, K.WHEEL_R, K.WHEEL_R)
    lo, hi = min(cz, cw) - 0.035, max(cz, cw) + 0.03
    return [(lo, 0.0), (hi, 0.0), (hi, 0.07), (lo, 0.07)], None


def hole_squares(holes, bone):
    """🔳 The perforation holes as small closed squares of the side elevation."""
    return epath(bone, [[(a[0], a[1]) for a in ring] for ring in holes])


def hole_dashes(holes, bone):
    """➖ The perforation holes as one short stroke per hole, at mid height of the hole."""
    runs = []
    for ring in holes:
        (x0, y0), (x1, _), _, (_, y1) = ring
        runs.append([(x0, (y0 + y1) / 2), (x1, (y0 + y1) / 2)])
    return epath(bone, runs, closed=False)


def unit_shapes(scale):
    """🖥 The equipment of the side elevation at 1:50 and 1:100."""
    out = []
    for uid, first, height, region in K.UNITS:
        y0, y1 = K.unit_y(first, height)
        material = "blank-steel" if region == "blanks" else "chassis"
        tone = {"storage": 0.4, "patch-40": 0.5, "brush-41": 0.38, "switch-42": 0.3}.get(uid, 0.45)
        sid = "storage-1" if (uid == "storage" and scale == 50) else uid
        if scale == 100 and uid in ("patch-40", "brush-41", "switch-42"):
            if uid != "patch-40":
                continue
            out.append(shape("top-group", "equipment", "body", 3.2, True, 0.4, erect("equipment", -K.EAR, K.EAR, y0, K.unit_y(42, 1)[1]), regions=["switch", "blanks"], material="chassis"))
            continue
        out.append(shape(sid, "equipment", "body", 3.2, True, tone, erect("equipment", -K.EAR, K.EAR, y0, y1), region=region, material=material))
        if scale != 50:
            continue
        if uid == "storage":
            for row, dy in (("0", 0.032), ("1", 0.0875)):
                for col in range(6):
                    x = -0.19 + 0.0645 * col
                    out.append(shape(f"storage-1-bay-{row}-{col}", "equipment", "detail", 3.3, False, 0.3, erect("equipment", x, x + 0.052, y0 + dy, y0 + dy + 0.045)))
            out.append(shape("storage-1-led", "equipment", "detail", 3.3, True, 0.1, eellipse("equipment", 0.200, y0 + 0.0885, 0.0065), region="servers"))
        elif uid.startswith("server-"):
            for k, x in enumerate((0.02, 0.0755, 0.131, 0.1865)):
                out.append(shape(f"{uid}-bay-{k}", "equipment", "detail", 3.3, False, 0.3, erect("equipment", x, x + 0.043, y0 + 0.0155, y0 + 0.0755)))
            out.append(shape(f"{uid}-led", "equipment", "detail", 3.3, True, 0.1, eellipse("equipment", -0.216, y0 + 0.044, 0.0065), region="servers"))
            cells = []
            for col in range(5):
                for row in range(2):
                    x, y = -0.205 + 0.035 * col, y0 + 0.0155 + 0.033 * row
                    cells.append([(x, y), (x + 0.022, y), (x + 0.022, y + 0.0293), (x, y + 0.0293)])
            out.append(shape(f"{uid}-vent", "equipment", "detail", 3.3, False, 0.3, epath("equipment", cells)))
        elif uid == "patch-40":
            out.append(shape("patch-40-slot", "equipment", "detail", 3.3, False, 0.2, epath("equipment", [[(-0.20, y0 + 0.016), (0.20, y0 + 0.016), (0.20, y0 + 0.028), (-0.20, y0 + 0.028)]])))
        elif uid == "brush-41":
            out.append(shape("brush-41-slot", "equipment", "detail", 3.3, False, 0.2, epath("equipment", [[(-0.19, y0 + 0.022), (0.19, y0 + 0.022)]], closed=False)))
        elif uid == "switch-42":
            ports = []
            for col in range(12):
                for dy in (0.0095, 0.0235):
                    x = -0.19 + 0.0245 * col
                    ports.append([(x, y0 + dy), (x + 0.0155, y0 + dy), (x + 0.0155, y0 + dy + 0.0105), (x, y0 + dy + 0.0105)])
            out.append(shape("switch-42-ports", "equipment", "detail", 3.3, False, 0.3, epath("equipment", ports)))
            out.append(shape("switch-42-led", "equipment", "detail", 3.3, True, 0.1, eellipse("equipment", 0.200, y0 + 0.0217, 0.0065), region="switch"))
    return out


def door_shapes(scale):
    """🚪 The door of the side elevation: the perforated face behind the equipment it shows, the unperforated frame and stile in front, the perforation, handle and hinges."""
    x0, x1, y0, y1 = -K.DOOR_X, K.DOOR_X, K.DOOR_Y0, K.DOOR_Y1
    if scale in (50, 100):
        fx0, fx1, fy0, fy1 = B.FRONT_FIELD
    else:
        fx0, fx1, fy0, fy1 = -0.24, 0.24, 0.13, 1.937
    ring = [[(x0, y0), (x1, y0), (x1, y1), (x0, y1)], [(fx0, fy0), (fx1, fy0), (fx1, fy1), (fx0, fy1)]]
    out = [shape("door-field", "door", "body", 2.9, True, 0.18, erect("door", x0, x1, y0, y1), region="door", material="mesh-steel")]
    out.append(shape("door-frame", "door", "body", 4.0, True, 0.24, epath("door", ring), region="door", material="mesh-steel"))
    if scale == 50:
        out.append(shape("door-perforation", "door", "detail", 3.0, False, 0.3, hole_squares(B.front_holes(50), "door")))
        out.append(shape("door-handle", "door", "body", 4.1, True, 0.2, erect("door", 0.226, 0.252, 0.94, 1.10, 0.003), region="door", material="mesh-steel"))
        out.append(shape("door-handle-lever", "door", "detail", 4.2, False, 0.12, erect("door", 0.231, 0.247, 0.95, 1.07, 0.003)))
        out.append(shape("door-lock", "door", "detail", 4.2, True, 0.6, eellipse("door", 0.239, 1.085, 0.007)))
        for k in range(3):
            out.append(shape(f"door-hinge-{k + 1}", "door", "detail", 4.1, True, 0.3, erect("door", -0.2795, -0.27, 0.27 + 0.72 * k, 0.33 + 0.72 * k)))
    elif scale == 100:
        out.append(shape("door-perforation", "door", "detail", 3.0, False, 0.3, hole_dashes(B.front_holes(100), "door")))
        out.append(shape("door-handle", "door", "body", 4.1, True, 0.2, erect("door", 0.226, 0.252, 0.93, 1.09, 0.005), region="door", material="mesh-steel"))
    return out


def elevation(scale):
    """📐 The side elevation (the face of the cabinet with its door)."""
    out = []
    if scale in (50, 100, 200):
        side = 0.03
        out.append(shape("post-left", "frame", "body", 3, True, 0.5, erect("frame", -K.HALF_W, -K.HALF_W + side, K.BASE_Y0, 1.94 if scale == 200 else K.TOP_Y1), region="cabinet", material="rack-steel"))
        out.append(shape("post-right", "frame", "body", 3, True, 0.5, erect("frame", K.HALF_W - side, K.HALF_W, K.BASE_Y0, 1.94 if scale == 200 else K.TOP_Y1), region="cabinet", material="rack-steel"))
        out.append(shape("roof", "frame", "body", 3, True, 0.45, erect("frame", -K.HALF_W, K.HALF_W, 1.94 if scale == 200 else K.TOP_Y1, K.ROOF_Y1), region="cabinet", material="rack-steel"))
        out.append(shape("base-rail", "frame", "body", 3, True, 0.45, erect("frame", -0.27, 0.27, K.BASE_Y0, K.BASE_Y1 + (0.02 if scale == 200 else 0)), region="cabinet", material="rack-steel"))
        for sid, cx in (("caster-left", -K.CASTER_X), ("caster-right", K.CASTER_X)):
            out.append(shape(sid, "feet", "body", 3, True, 0.2, epoly("feet", caster_elevation(cx, scale)), region="feet", material="caster-rubber"))
        if scale in (50, 100):
            if scale == 50:
                out.append(shape("rail-a", "equipment", "body", 3.1, True, 0.55, erect("equipment", -0.2445, -0.225, K.RAIL_Y0, K.RAIL_Y1), region="cabinet", material="rack-steel"))
                out.append(shape("rail-b", "equipment", "body", 3.1, True, 0.55, erect("equipment", 0.225, 0.2445, K.RAIL_Y0, K.RAIL_Y1), region="cabinet", material="rack-steel"))
            out += unit_shapes(scale)
        else:
            s0, s1 = K.unit_y(1, 4)[0], K.unit_y(15, 2)[1]
            t0, t1 = K.unit_y(40, 1)[0], K.unit_y(42, 1)[1]
            out.append(shape("server-block", "equipment", "body", 3.2, True, 0.42, erect("equipment", -K.EAR, K.EAR, s0, s1), region="servers", material="chassis"))
            out.append(shape("top-group", "equipment", "body", 3.2, True, 0.4, erect("equipment", -K.EAR, K.EAR, t0, t1), regions=["switch", "blanks"], material="chassis"))
        out += door_shapes(scale)
    else:
        out.append(shape("cabinet", "frame", "body", 3, True, 0.5, erect("frame", -K.HALF_W, K.HALF_W, 0.075, K.HEIGHT), regions=["cabinet", "door"], material="rack-steel"))
        out.append(shape("server-block", "equipment", "body", 3.2, True, 0.42, erect("equipment", -0.2, 0.2, K.RAIL_Y0, K.unit_y(15, 2)[1]), region="servers", material="chassis"))
        out.append(shape("top-group", "equipment", "body", 3.2, True, 0.4, erect("equipment", -0.2, 0.2, K.unit_y(40, 1)[0], K.RAIL_Y1), regions=["switch", "blanks"], material="chassis"))
        out.append(shape("casters", "feet", "body", 3, True, 0.2, erect("feet", -0.28, 0.28, 0.0, 0.075), region="feet", material="caster-rubber"))
    return out


def front(scale):
    """📐 The front view (from the right-hand side of the cabinet): side panel, roof, door and rear door edges, two casters."""
    zf = K.FRAME_Z
    out = []
    if scale == 500:
        out.append(shape("cabinet", "frame", "body", 3, True, 0.5, rect(-K.HALF_D, K.HALF_D, 0.075, K.HEIGHT), regions=["cabinet", "door"], material="rack-steel"))
        out.append(shape("casters", "feet", "body", 3, True, 0.2, rect(-0.475, 0.475, 0.0, 0.075), region="feet", material="caster-rubber"))
        return out
    top = 1.94 if scale == 200 else 1.965
    roof0 = 1.94 if scale == 200 else K.TOP_Y1
    out.append(shape("side-panel", "frame", "body", 3, True, 0.5, rect(-zf, zf, K.BASE_Y0, top), region="cabinet", material="rack-steel"))
    out.append(shape("roof", "frame", "body", 3.1, True, 0.45, rect(-zf, zf, roof0, K.ROOF_Y1), region="cabinet", material="rack-steel"))
    depth = 0.03 if scale == 200 else K.DOOR_DEPTH
    out.append(shape("door-edge", "door", "body", 3.4, True, 0.18, rect(-K.HALF_D, -K.HALF_D + depth, K.DOOR_Y0, K.DOOR_Y1), region="door", material="mesh-steel"))
    out.append(shape("rear-door-edge", "frame", "body", 3.4, True, 0.25, rect(K.HALF_D - depth, K.HALF_D, K.DOOR_Y0, K.DOOR_Y1), region="cabinet", material="mesh-steel"))
    if scale == 50:
        out.append(shape("panel-seam", "frame", "detail", 3.2, False, 0.35, poly([(0.0, K.BASE_Y0), (0.0, 1.965)], closed=False)))
        out.append(shape("vents", "frame", "detail", 3.3, True, 0.3, path([[(0.22, y), (0.42, y), (0.42, y + 0.012), (0.22, y + 0.012)] for y in [1.62 + 0.0517 * k for k in range(7)]])))
        out.append(shape("door-handle", "door", "body", 3.5, True, 0.2, poly([(-0.5, 0.94), (-0.5, 1.10), (-0.5075, 1.10), (-0.5075, 1.07), (-0.515, 1.07), (-0.515, 0.95), (-0.5075, 0.95), (-0.5075, 0.94)]), region="door", material="mesh-steel"))
    elif scale == 100:
        out.append(shape("door-handle", "door", "body", 3.5, True, 0.2, rect(-0.515, -0.5, 0.93, 1.09), region="door", material="mesh-steel"))
    for sid, rear in (("caster-front", False), ("caster-rear", True)):
        fork, wheel = caster_side(rear, scale)
        if wheel is not None:
            out.append(shape(sid, "feet", "body", 3, True, 0.2, {"kind": "ellipse", "cx": r6(wheel[0]), "cy": r6(wheel[1]), "rx": wheel[2], "ry": wheel[2]}, region="feet", material="caster-rubber"))
            out.append(shape(f"{sid}-fork", "feet", "body", 3.05, True, 0.3, poly(fork), region="feet", material="rack-steel"))
        else:
            out.append(shape(sid, "feet", "body", 3, True, 0.2, poly(fork), region="feet", material="caster-rubber"))
    return out


def plan(scale):
    """📐 The plan: roof with fan openings and cable slot, the door and rear door edges."""
    zf = K.FRAME_Z
    if scale == 500:
        return [shape("roof", "frame", "body", 3, True, 0.45, rect(-K.HALF_W, K.HALF_W, -K.HALF_D, K.HALF_D), regions=["cabinet", "door"], material="rack-steel")]
    out = [shape("roof", "frame", "body", 3, True, 0.45, rect(-K.HALF_W, K.HALF_W, -zf, zf), region="cabinet", material="rack-steel")]
    if scale in (50, 100):
        for k, z in enumerate(K.FAN_Z):
            out.append(shape(f"fan-opening-{k + 1}", "frame", "detail", 3.2, True, 0.15, poly(K.circle(0, z, K.FAN_R, 32)), material="rack-steel"))
            if scale == 50:
                rings = [K.circle(0, z, r, 24) for r in (0.035, 0.055, 0.075)]
                out.append(shape(f"fan-guard-{k + 1}", "frame", "detail", 3.3, False, 0.6, path(rings)))
                out.append(shape(f"fan-spokes-{k + 1}", "frame", "detail", 3.3, False, 0.6, path([[(-0.079, z), (0.079, z)], [(0.0, z - 0.079), (0.0, z + 0.079)]], closed=False)))
        out.append(shape("cable-slot", "frame", "detail", 3.2, True, 0.2, rect(K.SLOT[0], K.SLOT[1], K.SLOT[2], K.SLOT[3], 0.008), material="rack-steel"))
    depth = 0.03 if scale == 200 else K.DOOR_DEPTH
    out.append(shape("door-field", "door", "body", 3.4, True, 0.18, rect(-K.DOOR_X, K.DOOR_X, -K.HALF_D, -K.HALF_D + depth), region="door", material="mesh-steel"))
    out.append(shape("rear-door-edge", "frame", "body", 3.4, True, 0.25, rect(-K.DOOR_X, K.DOOR_X, K.HALF_D - depth, K.HALF_D), region="cabinet", material="mesh-steel"))
    if scale in (50, 100):
        out.append(shape("door-handle", "door", "body", 3.5, True, 0.2, rect(0.226, 0.252, -0.515, -0.5), region="door", material="mesh-steel"))
    if scale == 50:
        out.append(shape("door-hinge", "door", "detail", 3.5, True, 0.3, {"kind": "ellipse", "cx": -0.2735, "cy": -0.4935, "rx": 0.006, "ry": 0.006}))
    return out


def apply(asset):
    """✍ Writes the redrawn geometry of every scale into the asset."""
    for scale in (50, 100, 200, 500):
        rep = asset["representations"][str(scale)]
        rep["geometry"] = {"elevation": elevation(scale), "plan": plan(scale), "front": front(scale)}
