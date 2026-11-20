"""🌱 Round 3, machinery/diesel-generator: the root corrections outside `volumes` (rig joints, anchors, masses, references, region areas) and the 2D drawings of all four scales and three views, drawn from the same measures as the solids (ruling E1).

`apply_roots(asset, areas)` changes the asset dictionary in place.
"""
import copy
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-diesel-generator-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)
R5, T, L2, W2 = k.R5, k.T, k.L2, k.W2
C, SIL, ST, CAP, HD, FL = "container", "silencer", "stack", "cap-flap", "outlet-hood", "outlet-flap"
CS, ES, AL, GS = "container-steel", "exhaust-steel", "aluminium", "galvanised-steel"


def shape(id, bone, role, z, tone, geom, region, material, filled=True, **kw):
    """✏️ One 2D shape."""
    out = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geom)
    out.update({"region": region, "material": material})
    out.update(kw)
    return out


def poly(points, closed=True):
    """✏️ Poly geometry."""
    return {"kind": "poly", "points": [[R5(x), R5(y)] for x, y in points], "closed": closed}


def path(rings):
    """✏️ Path geometry, one closed subpath per ring (holes by even-odd)."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{R5(x)} {R5(y)}" for x, y in ring) + " Z" for ring in rings)}


def rect(x0, x1, y0, y1):
    """⬜ Rect geometry (front and plan frames)."""
    return {"kind": "rect", "x": R5(x0), "y": R5(y0), "w": R5(x1 - x0), "h": R5(y1 - y0)}


def ellipse(cx, cy, r):
    """⭕ Circle geometry."""
    return {"kind": "ellipse", "cx": R5(cx), "cy": R5(cy), "rx": R5(r), "ry": R5(r)}


def box_pts(x0, x1, y0, y1):
    """⬜ Corners of a rectangle."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def ep(bone, points):
    """🔁 World elevation points into a bone frame."""
    return [k.to_local(bone, x, y) for x, y in points]


def erect(bone, x0, x1, y0, y1):
    """⬜ A world rectangle as an elevation poly of a bone."""
    return poly(ep(bone, box_pts(x0, x1, y0, y1)))


def stadium(cx, cy, w, h, n=4):
    """🕳️ A slot outline."""
    r = h / 2
    pts = []
    for i in range(n + 1):
        a = -math.pi / 2 + math.pi * i / n
        pts.append((cx + w / 2 - r + r * math.cos(a), cy + r * math.sin(a)))
    for i in range(n + 1):
        a = math.pi / 2 + math.pi * i / n
        pts.append((cx - w / 2 + r + r * math.cos(a), cy + r * math.sin(a)))
    return pts


def circle_pts(cx, cy, r, n=10):
    """⭕ A circle polygon."""
    return [(cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)) for i in range(n)]


def cap_outline(level):
    """🎩 The rain cap seen from the side in its own frame (u from the hinge, v down): domed lid, skirt open at the hinge."""
    if level == "fine":
        return [(0.02, 0.035), (0.3, 0.035), (0.3, -0.01), (0.28, -0.045), (0.245, -0.07), (0.2, -0.088), (0.15, -0.095), (0.1, -0.088), (0.055, -0.07), (0.02, -0.045), (0.0, -0.01), (0.02, -0.01)]
    if level == "coarse":
        return [(0.0, 0.0), (0.3, 0.0), (0.28, -0.045), (0.225, -0.08), (0.15, -0.1), (0.075, -0.08), (0.02, -0.045)]
    return [(0.0, 0.0), (0.3, 0.0), (0.26, -0.07), (0.15, -0.1), (0.04, -0.07)]


def silencer_poly(steps, axis_off):
    """🛢️ The silencer outline about its axis (elevation: offset = y about SIL_Y; plan: offset = z about SIL_Z)."""
    return [(x, axis_off + r) for x, r in k.silencer_outline(steps)]


def saddle_side(xs, t, roof, base):
    """🪑 A saddle seen from the side: web up to the shell, base plate on the roof."""
    top = k.SIL_Y - k.SIL_R + 0.02
    if base:
        return [(xs - 0.075, roof), (xs + 0.075, roof), (xs + 0.075, roof + base), (xs + t / 2, roof + base), (xs + t / 2, top), (xs - t / 2, top), (xs - t / 2, roof + base), (xs - 0.075, roof + base)]
    return box_pts(xs - t / 2, xs + t / 2, roof, top)


def saddle_front(roof):
    """🪑 A saddle seen from ahead: base plate and web up to the cradle under the shell, x = lateral."""
    prof = k.saddle_profile(k.SIL_R, 0.23, roof)
    return [(z, y) for y, z in prof]


def keep_pen(old, new):
    """🖊️ Keeps a pen override of the shape with the same id."""
    for s in old:
        if s.get("id") == new["id"] and "penMm" in s and "penMm" not in new:
            new["penMm"] = s["penMm"]
    return new


def elevation(d):
    """🖼️ The side elevation of one scale (seen from -z)."""
    roof = k.ROOF_TOP
    lx, ly, lz = k.CAST
    px = k.POST[0]
    out = []
    fine, mid = d == 50, d == 100
    if fine or mid:
        out += [
            shape("skid-tank", C, "body", 1.9, 0.3, erect(C, -k.CAST_IN_X, k.CAST_IN_X, *k.RAIL_Y), "skid-tank", CS),
            shape("wall", C, "body", 2, 0.55, erect(C, -(L2 - px), L2 - px, *k.WALL_Y), "enclosure-walls", CS),
            shape("roof-band", C, "body", 2.1, 0.45, erect(C, -k.CAST_IN_X, k.CAST_IN_X, *k.TOP_RAIL_Y), "enclosure-walls", CS),
            shape("corner-post-l", C, "body", 2.05, 0.45, erect(C, -L2, -L2 + px, ly, k.H - ly), "enclosure-walls", CS),
            shape("corner-post-r", C, "body", 2.05, 0.45, erect(C, L2 - px, L2, ly, k.H - ly), "enclosure-walls", CS),
        ]
        for id, x0, y0, region in (("corner-casting-l", -L2, 0.0, "skid-tank"), ("corner-casting-r", L2 - lx, 0.0, "skid-tank"), ("top-casting-l", -L2, k.H - ly, "roof"), ("top-casting-r", L2 - lx, k.H - ly, "roof")):
            rings = [box_pts(x0, x0 + lx, y0, y0 + ly)]
            if fine:
                rings.append(stadium(x0 + lx / 2, y0 + ly / 2, 0.09, 0.05))
            out.append(shape(id, C, "detail", 2.15, 0.22, path([ep(C, r) for r in rings]), region, CS))
        f = k.LOUVRE_FRAME
        for n, (x0, x1) in zip(("1", "2"), k.LOUVRES):
            kw = {"hatch": "louvre"} if fine or mid else {}
            out.append(shape(f"inlet-louvre-{n}", C, "detail", 2.2, 0.25, erect(C, x0 - f, x1 + f, k.LOUVRE_Y[0] - f, k.LOUVRE_Y[1] + f), "inlet-louvres", AL, **kw))
        for id, (x0, x1) in zip(("door-line", "door-line-2"), k.DOORS):
            out.append(shape(id, C, "detail", 2.2, 0.5, erect(C, x0, x1, *k.DOOR_Y), "enclosure-walls", CS, filled=False))
    else:
        top = k.H - 0.15
        wx = L2 - 0.15
        out += [
            shape("skid-tank", C, "body", 1.9, 0.3 if d == 200 else 0.28, erect(C, -L2, L2, 0.0, 0.2), "skid-tank", CS),
            shape("wall", C, "body", 2, 0.55, erect(C, -wx, wx, 0.2, top), "enclosure-walls", CS),
            shape("roof-band", C, "body", 2.1, 0.45, erect(C, -L2, L2, top, k.H), "roof", CS),
            shape("corner-post-l", C, "body", 2.05, 0.42, erect(C, -L2, -wx, 0.2, top), "enclosure-walls", CS),
            shape("corner-post-r", C, "body", 2.05, 0.42, erect(C, wx, L2, 0.2, top), "enclosure-walls", CS),
        ]
        f = k.LOUVRE_FRAME
        for n, (x0, x1) in zip(("1", "2"), k.LOUVRES):
            out.append(shape(f"inlet-louvre-{n}", C, "detail" if d == 200 else "body", 2.2, 0.22, erect(C, x0 - f, x1 + f, k.LOUVRE_Y[0] - f, k.LOUVRE_Y[1] + f), "inlet-louvres", AL))
        if d == 200:
            for id, (x0, x1) in zip(("door-line", "door-line-2"), k.DOORS):
                out.append(shape(id, C, "detail", 2.2, 0.5, erect(C, x0, x1, *k.DOOR_Y), "enclosure-walls", CS, filled=False))
    if fine:
        hinges = []
        for (x0, x1) in k.DOORS:
            for i in range(3):
                y = 0.6 + 0.65 * i
                hinges.append(ep(C, box_pts(x1 - 0.012, x1 + 0.012, y, y + 0.1)))
        out += [
            shape("door-hinges", C, "detail", 2.3, 0.25, path(hinges), "enclosure-walls", CS),
            shape("door-handle", C, "detail", 2.3, 0.2, erect(C, 1.55, 1.61, 1.2, 1.5), "enclosure-walls", CS),
            shape("door-handle-lever", C, "detail", 2.35, 0.2, erect(C, 1.56, 1.7, 1.3, 1.325), "enclosure-walls", CS),
            shape("door-handle-2", C, "detail", 2.3, 0.2, erect(C, 2.4, 2.46, 1.2, 1.5), "enclosure-walls", CS),
            shape("door-handle-2-lever", C, "detail", 2.35, 0.2, erect(C, 2.41, 2.55, 1.3, 1.325), "enclosure-walls", CS),
            shape("drip-rail", C, "detail", 2.3, 0.4, erect(C, 1.45, 2.9, 2.18, 2.2), "enclosure-walls", CS),
            shape("fuel-fill", C, "detail", 2.3, 0.2, erect(C, k.FUEL_FILL[0][0], k.FUEL_FILL[0][1], *k.FUEL_FILL[1]), "skid-tank", CS),
            shape("lifting-eye", C, "detail", 2.3, 0.2, path([ep(C, [(k.EYE_X[0], roof), (k.EYE_X[1], roof), (k.EYE_X[1] - 0.06, roof + 0.08), (k.EYE_X[0] + 0.06, roof + 0.08)]), ep(C, circle_pts(2.65, roof + 0.04, 0.02, 8))]), "roof", CS),
        ]
    steps = {50: 6, 100: 4, 200: 3, 500: 2}[d]
    out.append(shape("silencer", SIL, "body", 2.5, 0.35, poly(ep(SIL, silencer_poly(steps, k.SIL_Y))), "silencer", ES))
    sroof = roof if fine or mid else k.H
    for i, xs in enumerate(k.SADDLES):
        t = {50: 0.012, 100: 0.016, 200: 0.03, 500: 0.08}[d]
        base = {50: 0.012, 100: 0.016}.get(d, 0.0)
        out.append(shape("silencer-saddle" + ("" if i == 0 else "-2"), SIL, "body" if d == 500 else "detail", 2.45, 0.3, poly(ep(SIL, saddle_side(xs, t, sroof, base))), "silencer", ES))
        if fine or mid:
            bt = 0.008 if fine else 0.015
            out.append(shape("silencer-clamp" + ("" if i == 0 else "-2"), SIL, "detail", 2.55, 0.22, erect(SIL, xs - 0.03, xs + 0.03, k.SIL_Y - k.SIL_R - bt, k.SIL_Y + k.SIL_R + bt), "silencer", ES))
    if fine:
        x = k.INLET_X
        inlet = [(x - 0.17, roof), (x + 0.17, roof), (x + 0.17, roof + 0.02), (x + 0.12, roof + 0.02), (x + 0.12, roof + 0.11), (x + 0.1, roof + 0.11), (x + 0.1, k.SIL_Y - k.SIL_R + 0.01), (x - 0.1, k.SIL_Y - k.SIL_R + 0.01), (x - 0.1, roof + 0.11), (x - 0.12, roof + 0.11), (x - 0.12, roof + 0.02), (x - 0.17, roof + 0.02)]
        out.append(shape("silencer-inlet", SIL, "detail", 2.45, 0.25, poly(ep(SIL, inlet)), "silencer", ES))
    elif mid:
        x = k.INLET_X
        inlet = [(x - 0.17, roof), (x + 0.17, roof), (x + 0.17, roof + 0.02), (x + 0.1, roof + 0.02), (x + 0.1, k.SIL_Y - k.SIL_R + 0.01), (x - 0.1, k.SIL_Y - k.SIL_R + 0.01), (x - 0.1, roof + 0.02), (x - 0.17, roof + 0.02)]
        out.append(shape("silencer-inlet", SIL, "detail", 2.45, 0.25, poly(ep(SIL, inlet)), "silencer", ES))
    out.append(shape("exhaust-stack", ST, "body", 2.6, 0.2, poly([(-0.04, 0.1), (-0.04, -0.1), (0.33, -0.1), (0.33, 0.1)]), "exhaust-stack", ES))
    if fine or mid:
        out.append(shape("stack-spigot", ST, "detail", 2.58, 0.25, poly(box_pts(0.06, 0.122, -0.14, 0.14)), "silencer", ES))
        out.append(shape("stack-hinge-bracket", ST, "detail", 2.6, 0.3, poly(box_pts(0.25, 0.315, -0.16, -0.095)), "exhaust-stack", ES))
    level = "fine" if fine or mid else ("coarse" if d == 200 else "puck")
    out.append(shape("rain-cap", CAP, "body", 2.7, 0.3, poly(cap_outline(level)), "rain-cap", GS))
    if fine or mid:
        out.append(shape("rain-cap-knuckle", CAP, "detail", 2.75, 0.3, ellipse(0, 0, 0.015 if fine else 0.02), "rain-cap", GS))
    if fine or mid:
        hood = k.HOOD
    else:
        xe = 2.96 if d == 200 else 2.93
        hood = [(1.9, k.H), (xe, k.H), (xe, 3.25), (2.2, 3.25), (1.9, 2.97)]
    out.append(shape("discharge-hood", HD, "body", 2.5, 0.4, poly(ep(HD, hood)), "discharge-hood", CS))
    t = {50: 0.06, 100: 0.06, 200: 0.07, 500: 0.1}[d]
    out.append(shape("outlet-flap", FL, "body", 2.6, 0.5, poly([(0, -t), (0.63, -t), (0.63, 0), (0, 0)]), "outlet-flap", GS))
    if fine:
        out.append(shape("outlet-flap-knuckle", FL, "detail", 2.65, 0.3, ellipse(0, 0, 0.016), "outlet-flap", GS))
    return out


def flap_solid(t):
    """🚪 The weather flap as an extruded solid across the mouth (front and plan)."""
    return shape("outlet-flap", FL, "body", 2.6, 0.5, poly([(0, -t), (0.63, -t), (0.63, 0), (0, 0)]), "outlet-flap", GS, solid={"kind": "extruded", "sweep": "w", "z0": -0.85, "z1": 0.85})


def cap_solid(level):
    """🎩 The rain cap as a revolved solid about its vertical axis (front and plan)."""
    if level in ("coarse", "puck"):
        return shape("rain-cap", CAP, "body", 2.7, 0.3, poly(cap_outline(level)), "rain-cap", GS, solid={"kind": "extruded", "sweep": "w", "z0": k.SIL_Z - k.CAP_R, "z1": k.SIL_Z + k.CAP_R})
    return shape("rain-cap", CAP, "body", 2.7, 0.3, poly(cap_outline(level)), "rain-cap", GS, solid={"kind": "revolved", "axisBone": CAP, "axis": "v", "axisX": -0.15, "axisZ": k.SIL_Z})


def front(d):
    """🖼️ The front elevation of one scale (seen from +x, x = lateral z)."""
    roof = k.ROOF_TOP
    lx, ly, lz = k.CAST
    pz = k.POST[1]
    fine, mid = d == 50, d == 100
    t = {50: 0.06, 100: 0.06, 200: 0.07, 500: 0.1}[d]
    level = "fine" if fine or mid else ("coarse" if d == 200 else "puck")
    out = [flap_solid(t)]
    if fine or mid:
        out.append(shape("outlet-mouth", HD, "detail", 2.55, 0.15, rect(-k.MOUTH_Z, k.MOUTH_Z, *k.MOUTH_Y), "discharge-hood", CS))
    elif d == 200:
        out.append(shape("outlet-mouth", HD, "detail", 2.55, 0.15, rect(-k.MOUTH_Z, k.MOUTH_Z, *k.MOUTH_Y), "discharge-hood", CS))
    out.append(shape("discharge-hood", HD, "body", 2.5, 0.4, rect(-k.HOOD_Z, k.HOOD_Z, roof if fine or mid else k.H, 3.25), "discharge-hood", CS))
    out.append(cap_solid(level))
    out.append(shape("exhaust-stack", ST, "body", 2.6, 0.2, rect(k.SIL_Z - 0.1, k.SIL_Z + 0.1, *k.STACK_Y), "exhaust-stack", ES))
    if fine or mid:
        out.append(shape("stack-spigot", ST, "detail", 2.58, 0.25, rect(k.SIL_Z - 0.14, k.SIL_Z + 0.14, 3.2, 3.262), "silencer", ES))
        out.append(shape("stack-hinge-bracket", ST, "detail", 2.62, 0.3, rect(k.SIL_Z - 0.04, k.SIL_Z + 0.04, 3.39, 3.455), "exhaust-stack", ES))
    out.append(shape("silencer", SIL, "body", 2.5, 0.35, ellipse(k.SIL_Z, k.SIL_Y, k.SIL_R), "silencer", ES))
    out.append(shape("silencer-saddle", SIL, "body" if d == 500 else "detail", 2.45, 0.3, poly(saddle_front(roof if fine or mid else k.H)), "silencer", ES))
    if fine:
        x = k.SIL_Z
        inlet = [(x - 0.17, roof), (x + 0.17, roof), (x + 0.17, roof + 0.02), (x + 0.12, roof + 0.02), (x + 0.12, roof + 0.11), (x + 0.1, roof + 0.11), (x + 0.1, k.SIL_Y - k.SIL_R + 0.01), (x - 0.1, k.SIL_Y - k.SIL_R + 0.01), (x - 0.1, roof + 0.11), (x - 0.12, roof + 0.11), (x - 0.12, roof + 0.02), (x - 0.17, roof + 0.02)]
        out.append(shape("silencer-inlet", SIL, "detail", 2.47, 0.25, poly(inlet), "silencer", ES))
    if fine or mid:
        out += [
            shape("skid-tank", C, "body", 1.9, 0.3, rect(-k.CAST_IN_Z, k.CAST_IN_Z, *k.RAIL_Y), "skid-tank", CS),
            shape("wall", C, "body", 2, 0.55, rect(-(W2 - pz), W2 - pz, k.WALL_Y[0], k.TOP_END_Y[0]), "enclosure-walls", CS),
            shape("roof-band", C, "body", 2.1, 0.45, rect(-k.CAST_IN_Z, k.CAST_IN_Z, *k.TOP_END_Y), "enclosure-walls", CS),
            shape("corner-post-left", C, "body", 2.05, 0.45, rect(W2 - pz, W2, ly, k.H - ly), "enclosure-walls", CS),
            shape("corner-post-right", C, "body", 2.05, 0.45, rect(-W2, -W2 + pz, ly, k.H - ly), "enclosure-walls", CS),
            shape("end-door", C, "detail", 2.2, 0.5, rect(k.END_DOOR_Z[0], k.END_DOOR_Z[1], *k.DOOR_Y), "enclosure-walls", CS, filled=False),
        ]
        for id, z0, y0, region in (("corner-casting-left", W2 - lz, 0.0, "skid-tank"), ("corner-casting-right", -W2, 0.0, "skid-tank"), ("top-casting-left", W2 - lz, k.H - ly, "roof"), ("top-casting-right", -W2, k.H - ly, "roof")):
            rings = [box_pts(z0, z0 + lz, y0, y0 + ly)]
            if fine:
                rings.append(stadium(z0 + lz / 2, y0 + ly / 2, 0.08, 0.05))
            out.append(shape(id, C, "detail", 2.15, 0.22, path(rings), region, CS))
        if fine:
            hinges = [box_pts(k.END_DOOR_Z[0] - 0.012, k.END_DOOR_Z[0] + 0.012, 0.6 + 0.65 * i, 0.7 + 0.65 * i) for i in range(3)]
            out += [
                shape("end-door-hinges", C, "detail", 2.3, 0.25, path(hinges), "enclosure-walls", CS),
                shape("end-door-handle", C, "detail", 2.3, 0.2, rect(0.39, 0.45, 1.2, 1.5), "enclosure-walls", CS),
                shape("end-door-handle-lever", C, "detail", 2.35, 0.2, rect(0.27, 0.41, 1.3, 1.325), "enclosure-walls", CS),
                shape("lifting-eye", C, "detail", 2.3, 0.2, path([box_pts(-k.EYE_Z - 0.015, -k.EYE_Z + 0.015, roof, roof + 0.08), box_pts(k.EYE_Z - 0.015, k.EYE_Z + 0.015, roof, roof + 0.08)]), "roof", CS),
            ]
    else:
        top = k.H - 0.15
        pw = 0.15
        out += [
            shape("skid-tank", C, "body", 1.9, 0.3 if d == 200 else 0.28, rect(-W2, W2, 0.0, 0.2), "skid-tank", CS),
            shape("wall", C, "body", 2, 0.55, rect(-(W2 - pw), W2 - pw, 0.2, top), "enclosure-walls", CS),
            shape("roof-band", C, "body", 2.1, 0.45, rect(-W2, W2, top, k.H), "roof", CS),
            shape("corner-post-left", C, "body", 2.05, 0.42, rect(W2 - pw, W2, 0.2, top), "enclosure-walls", CS),
            shape("corner-post-right", C, "body", 2.05, 0.42, rect(-W2, -W2 + pw, 0.2, top), "enclosure-walls", CS),
        ]
    return out


def plan(d):
    """🖼️ The plan of one scale (x ahead, y = lateral z)."""
    lx, ly, lz = k.CAST
    fine, mid = d == 50, d == 100
    t = {50: 0.06, 100: 0.06, 200: 0.07, 500: 0.1}[d]
    level = "fine" if fine or mid else ("coarse" if d == 200 else "puck")
    out = [flap_solid(t), cap_solid(level)]
    out.append(shape("exhaust-stack", ST, "body", 2.65, 0.2, ellipse(k.STACK_X, k.SIL_Z, k.STACK_R), "exhaust-stack", ES))
    if fine or mid:
        out.append(shape("stack-spigot", ST, "detail", 2.6, 0.25, ellipse(k.STACK_X, k.SIL_Z, 0.14), "silencer", ES))
    xe = L2 if fine or mid else (2.96 if d == 200 else 2.93)
    out.append(shape("discharge-hood", HD, "body", 2.5, 0.4, rect(1.9, xe, -k.HOOD_Z, k.HOOD_Z), "discharge-hood", CS))
    if d != 500:
        out.append(shape("hood-ridge", HD, "detail", 2.55, 0.3, poly([(2.2, -k.HOOD_Z), (2.2, k.HOOD_Z)], closed=False), "discharge-hood", CS, filled=False))
    steps = {50: 6, 100: 4, 200: 3, 500: 2}[d]
    out.append(shape("silencer", SIL, "body", 2.5, 0.35, poly(silencer_poly(steps, k.SIL_Z)), "silencer", ES))
    if fine or mid:
        bt = 0.008 if fine else 0.015
        for i, xs in enumerate(k.SADDLES):
            out.append(shape(f"silencer-clamp-{i + 1}", SIL, "detail", 2.55, 0.22, rect(xs - 0.03, xs + 0.03, k.SIL_Z - k.SIL_R - bt, k.SIL_Z + k.SIL_R + bt), "silencer", ES))
        out.append(shape("roof-edge", C, "detail", 2.1, 0.4, rect(-k.ROOF_X, k.ROOF_X, -k.ROOF_Z, k.ROOF_Z), "roof", CS, filled=False))
    out.append(shape("roof", C, "body", 2, 0.45 if fine or mid else 0.45, rect(-L2, L2, -W2, W2), "roof", CS))
    if fine or mid:
        for i, (x0, z0) in enumerate(((-L2, -W2), (L2 - lx, -W2), (-L2, W2 - lz), (L2 - lx, W2 - lz))):
            rings = [box_pts(x0, x0 + lx, z0, z0 + lz)]
            if fine:
                rings.append(stadium(x0 + lx / 2, z0 + lz / 2, 0.124, 0.063))
            out.append(shape(f"corner-casting-{i + 1}", C, "detail", 2.05, 0.22, path(rings), "roof", CS))
    if fine:
        out.append(shape("lifting-eye", C, "detail", 2.3, 0.2, path([box_pts(k.EYE_X[0], k.EYE_X[1], -k.EYE_Z - 0.015, -k.EYE_Z + 0.015), box_pts(k.EYE_X[0], k.EYE_X[1], k.EYE_Z - 0.015, k.EYE_Z + 0.015)]), "roof", CS))
    return out


DECISIONS = {
    "50": {
        "silhouette": T("The 20 ft ISO frame with its eight corner castings (oval apertures), four corner posts, top rails and bottom rails clear of the ground; all 1:100 parts plus door hinges, lever handles, drip rail, fuel fill, lifting eyes, the bellows inlet of the silencer and the flap hinge knuckles.", "Der 20-ft-ISO-Rahmen mit acht Eckbeschlägen (ovale Öffnungen), vier Eckpfosten, oberen Trägern und unteren Trägern frei über dem Boden; alle 1:100-Teile plus Türbänder, Hebelgriffe, Tropfleiste, Tankstutzen, Hebeösen, der Kompensator am Schalldämpfereintritt und die Scharnieraugen der Klappe."),
    },
    "100": {
        "silhouette": T("Frame with plain castings, posts and rails, two inlet louvres with frames and blade hatching, two side doors, the silencer with 2:1 heads on two saddles with strap bands, inlet pipe, stack on its spigot with the hinge bracket, domed rain cap, discharge hood and flap.", "Rahmen mit einfachen Beschlägen, Pfosten und Trägern, zwei Zuluftlamellen mit Rahmen und Lamellenschraffur, zwei Seitentüren, der Schalldämpfer mit Klöpperböden 2:1 auf zwei Sätteln mit Spannbändern, Eintrittsrohr, Abgasrohr auf seinem Stutzen mit Scharnierbock, gewölbte Regenkappe, Austrittshaube und Klappe."),
    },
    "200": {
        "silhouette": T("Base band, wall with four corner posts, top band, two dark inlet louvre fields, two door outlines, the silencer on two saddles, stack with the domed rain cap, hood with flap.", "Sockelband, Wand mit vier Eckpfosten, Kopfband, zwei dunkle Zuluftfelder, zwei Türumrisse, der Schalldämpfer auf zwei Sätteln, Abgasrohr mit gewölbter Regenkappe, Haube mit Klappe."),
    },
    "500": {
        "silhouette": T("A dark base band, the wall between four corner posts and a top band, two dark louvre fields, the silencer on two saddles, stack, rain cap, hood and flap: a genset container, not a cabinet.", "Ein dunkles Sockelband, die Wand zwischen vier Eckpfosten und ein Kopfband, zwei dunkle Lamellenfelder, der Schalldämpfer auf zwei Sätteln, Abgasrohr, Regenkappe, Haube und Klappe: ein Aggregatcontainer, kein Schrank."),
    },
}

REFERENCES = [
    {"id": "iso-668", "citation": "ISO 668:2020 Series 1 freight containers, classification, dimensions and ratings (20 ft 1CC: 6058 x 2438 x 2591 mm)."},
    {"id": "iso-1161", "citation": "ISO 1161:2016 Series 1 freight containers, corner and intermediate fittings, specifications (corner casting 178 x 162 x 118 mm, oval apertures)."},
    {"id": "nett-silencer", "citation": "Nett Technologies, Cylindrical residential grade silencer data sheet (cylindrical body with dished heads, saddle or bracket mounting)."},
]


def apply_roots(asset, areas):
    """🌱 Rig joints at the real parts, anchors at the castings, masses in the new frames, references, areas and the 2D of every scale."""
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    ox, oy = k.origin(C)
    bones[C]["x"], bones[C]["y"] = R5(ox), R5(oy)
    for id in (SIL, ST, CAP, HD, FL):
        x, y = k.origin(id)
        bones[id]["x"], bones[id]["y"] = R5(x - ox), R5(y - oy)
    bones[SIL]["lengthM"] = R5(k.SIL_X[1] - k.SIL_X[0])
    bones[HD]["lengthM"] = R5(L2 - 1.9)
    bones[C]["mass"]["comX"] = R5(0.0 - ox)
    bones[SIL]["mass"]["comX"], bones[SIL]["mass"]["comY"] = R5((k.SIL_X[0] + k.SIL_X[1]) / 2 - k.SIL_X[0]), 0
    bones[HD]["mass"]["comX"], bones[HD]["mass"]["comY"] = 0.6, 0.318
    lx, ly, lz = k.CAST
    world = {"contact-a-l": (-k.CX, 0.0, k.CZ), "contact-a-r": (-k.CX, 0.0, -k.CZ), "contact-b-l": (k.CX, 0.0, k.CZ), "contact-b-r": (k.CX, 0.0, -k.CZ),
             "stack-mouth": (k.STACK_X, 3.47, k.SIL_Z), "radiator-outlet": (L2, (k.MOUTH_Y[0] + k.MOUTH_Y[1]) / 2, 0.0), "fuel-cog": (0.0, 0.1, 0.0)}
    for a in asset["anchors"]:
        x, y, z = world[a["id"]]
        a["x"], a["y"] = R5(x - ox), R5(y - oy)
        if a["id"].startswith("contact"):
            a["z"] = R5(z)
            a["footprintLengthM"], a["footprintWidthM"] = lx, lz
            a["title"]["en"] = a["title"]["en"].replace("Corner casting support", "Bottom corner casting (ISO 1161) support")
            a["title"]["de"] = a["title"]["de"].replace("Eckbeschlag-Auflager", "Auflager unter dem unteren Eckbeschlag (ISO 1161)")
    refs = asset["provenance"]["references"]
    have = {r["id"] for r in refs}
    refs += [r for r in REFERENCES if r["id"] not in have]
    for region in asset["thermal"]["regions"]:
        if region["id"] in areas:
            region["areaM2"] = areas[region["id"]]
    for key, rep in asset["representations"].items():
        if rep.get("status") != "authored":
            continue
        d = int(key)
        old = rep["geometry"]
        rep["geometry"] = {
            "elevation": [keep_pen(old["elevation"], s) for s in elevation(d)],
            "front": [keep_pen(old["front"], s) for s in front(d)],
            "plan": [keep_pen(old["plan"], s) for s in plan(d)],
        }
        rep["decisions"].update(copy.deepcopy(DECISIONS[key]))
