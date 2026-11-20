"""🏭 Round 3, machinery/diesel-generator: the handcrafted `volumes` of all four scales (body, thermal, structure, decisions), every section stated by hand from the measures of `r3-diesel-generator-kit.py`.

`volumes()` returns the `Volumes` object.
"""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-diesel-generator-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)

CS, ES, AL, GS = "container-steel", "exhaust-steel", "aluminium", "galvanised-steel"
C, SIL, ST, CAP, HD, FL = "container", "silencer", "stack", "cap-flap", "outlet-hood", "outlet-flap"
box, lbox, extrude, revolve, ellipsoid, ref, T = k.box, k.lbox, k.extrude, k.revolve, k.ellipsoid, k.ref, k.T
L2, W2, R5 = k.L2, k.W2, k.R5
TAU = 2 * math.pi


def stadium(cx, cy, w, h, n=6):
    """🕳️ A stadium (rounded slot) polygon of overall width w and height h."""
    r = h / 2
    pts = []
    for i in range(n + 1):
        a = -math.pi / 2 + math.pi * i / n
        pts.append((cx + w / 2 - r + r * math.cos(a), cy + r * math.sin(a)))
    for i in range(n + 1):
        a = math.pi / 2 + math.pi * i / n
        pts.append((cx - w / 2 + r + r * math.cos(a), cy + r * math.sin(a)))
    return pts


def circle(cx, cy, r, n=12):
    """⭕ A circle polygon."""
    return [(cx + r * math.cos(TAU * i / n), cy + r * math.sin(TAU * i / n)) for i in range(n)]


def castings(prefix, apertures):
    """🧊 The eight ISO 1161 corner castings as four solids (bottom and top at each end, repeated across the width)."""
    lx, ly, lz = k.CAST
    out = []
    for end, sx in (("a", -1), ("b", 1)):
        x0, x1 = (-L2, -L2 + lx) if sx < 0 else (L2 - lx, L2)
        for level, (y0, y1) in (("bottom", (0.0, ly)), ("top", (k.H - ly, k.H))):
            id = f"{prefix}-{level}-{end}"
            rep = {"count": 2, "translation": [0.0, 0.0, R5(2 * W2 - lz)]}
            if apertures:
                hole = stadium((x0 + x1) / 2, (y0 + y1) / 2, 0.09, 0.05)
                out.append(extrude(id, C, CS, "detail", 0.22, "xy", [(x0, y0), (x1, y0), (x1, y1), (x0, y1)], -W2, -W2 + lz, holes=[hole], repeat=rep))
            else:
                out.append(box(id, C, CS, "detail", 0.22, x0, x1, y0, y1, -W2, -W2 + lz, repeat=rep))
    return out


def posts(y0, y1, px, pz, tone):
    """🏛️ The four corner posts between the castings, outer faces flush with the frame."""
    out = []
    for end, sx in (("a", -1), ("b", 1)):
        for side, sz in (("l", 1), ("r", -1)):
            xa, xb = sorted((sx * L2, sx * (L2 - px)))
            za, zb = sorted((sz * W2, sz * (W2 - pz)))
            out.append(box(f"post-{end}-{side}", C, CS, "body", tone, xa, xb, y0, y1, za, zb))
    return out


def post_faces(end, side):
    """✂️ The faces of a corner post that look into the closed container (C31 stand-in)."""
    return ["+u" if end == "a" else "-u", "-w" if side == "l" else "+w"]


def silencer_parts(steps, band, saddle_t, base_t, roof, detail):
    """🛢️ The roof silencer: shell with 2:1 dished heads, strap bands, saddles with base plates, and at 1:50 the bellows inlet."""
    cx = (k.SIL_X[0] + k.SIL_X[1]) / 2
    xs0 = k.SADDLES[0]
    gap = k.SADDLES[1] - k.SADDLES[0]
    out = [revolve("silencer", SIL, ES, "body", 0.35, k.silencer_profile(steps), [1, 0, 0], (cx, k.SIL_Y, k.SIL_Z))]
    if band:
        w, t = band
        out.append(revolve("silencer-band", SIL, ES, "detail", 0.22, [(-w / 2, k.SIL_R - 0.001), (w / 2, k.SIL_R - 0.001), (w / 2, k.SIL_R + t), (-w / 2, k.SIL_R + t), (-w / 2, k.SIL_R - 0.001)], [1, 0, 0], (xs0, k.SIL_Y, k.SIL_Z), repeat={"count": 2, "translation": [R5(gap), 0.0, 0.0]}))
    base_y = roof + base_t
    out.append(extrude("silencer-saddle", SIL, ES, "detail", 0.3, "yz", k.saddle_profile(k.SIL_R - 0.003, 0.23, base_y - 0.002), xs0 - saddle_t / 2, xs0 + saddle_t / 2, repeat={"count": 2, "translation": [R5(gap), 0.0, 0.0]}))
    if base_t:
        out.append(box("silencer-saddle-base", SIL, ES, "detail", 0.3, xs0 - 0.075, xs0 + 0.075, roof, base_y, k.SIL_Z - 0.25, k.SIL_Z + 0.25, repeat={"count": 2, "translation": [R5(gap), 0.0, 0.0]}))
    if detail == "bellows":
        out.append(revolve("silencer-inlet-collar", SIL, ES, "detail", 0.25, [(0, 0.17), (0.02, 0.17)], [0, 1, 0], (k.INLET_X, roof, k.SIL_Z)))
        wave = [(0, 0.105)]
        for i in range(1, 8):
            wave.append((0.0125 * i, 0.12 if i % 2 else 0.105))
        wave += [(0.11, 0.1), (0.14, 0.1)]
        out.append(revolve("silencer-inlet", SIL, ES, "detail", 0.25, wave, [0, 1, 0], (k.INLET_X, roof + 0.02, k.SIL_Z)))
    elif detail == "pipe":
        out.append(revolve("silencer-inlet", SIL, ES, "detail", 0.25, [(0, 0.17), (0.02, 0.17), (0.02, 0.1), (0.14, 0.1)], [0, 1, 0], (k.INLET_X, roof, k.SIL_Z)))
    return out


def stack_parts(hollow, spigot, bracket):
    """🏭 The exhaust stack (stack frame: u up from y 3.14, v = -(x + 1.2), w lateral), its outlet spigot on the silencer and the hinge bracket of the rain cap."""
    out = []
    if spigot:
        out.append(revolve("stack-spigot", ST, ES, "detail", 0.25, [(0, 0.14), (0.062, 0.14)], [1, 0, 0], (0.06, 0.0, k.SIL_Z), world=False))
    if hollow:
        out.append(revolve("exhaust-stack", ST, ES, "body", 0.2, [(-0.04, 0.094), (0.33, 0.094), (0.33, 0.1), (-0.04, 0.1), (-0.04, 0.094)], [1, 0, 0], (0.0, 0.0, k.SIL_Z), world=False))
    else:
        out.append(revolve("exhaust-stack", ST, ES, "body", 0.2, [(-0.04, 0.1), (0.33, 0.1)], [1, 0, 0], (0.0, 0.0, k.SIL_Z), world=False))
    if bracket:
        out.append(lbox("stack-hinge-bracket", ST, ES, "detail", 0.3, 0.25, 0.315, -0.16, -0.095, k.SIL_Z - bracket, k.SIL_Z + bracket))
    return out


def cap_parts(level):
    """🎩 The flapper rain cap (cap frame: u = -(x + 1.05), v = 3.47 - y, axis of the lid along v through u 0.15): a domed lid with a skirt that is open at the hinge, and the hinge knuckle."""
    at = (0.15, 0.0, k.SIL_Z)
    if level == "fine":
        return [
            revolve("rain-cap", CAP, GS, "body", 0.3, [(-0.01, k.CAP_R), (-0.045, 0.13), (-0.07, 0.095), (-0.088, 0.05), (-0.095, 0.0)], [0, 1, 0], at, world=False),
            revolve("rain-cap-skirt", CAP, GS, "detail", 0.3, [(-0.012, k.CAP_R - 0.003), (0.035, k.CAP_R - 0.003), (0.035, k.CAP_R), (-0.012, k.CAP_R), (-0.012, k.CAP_R - 0.003)], [0, 1, 0], at, world=False, sweep=R5(TAU * 300 / 360), rotation=k.quat("y", 210)),
            revolve("rain-cap-knuckle", CAP, GS, "detail", 0.3, [(0, 0.015), (0.12, 0.015)], [0, 0, 1], (0.0, 0.0, k.SIL_Z - 0.06), world=False),
        ]
    if level == "mid":
        return [
            revolve("rain-cap", CAP, GS, "body", 0.3, [(-0.01, k.CAP_R), (-0.05, 0.12), (-0.08, 0.07), (-0.095, 0.0)], [0, 1, 0], at, world=False),
            revolve("rain-cap-skirt", CAP, GS, "detail", 0.3, [(-0.012, k.CAP_R - 0.003), (0.035, k.CAP_R - 0.003), (0.035, k.CAP_R), (-0.012, k.CAP_R), (-0.012, k.CAP_R - 0.003)], [0, 1, 0], at, world=False, sweep=R5(TAU * 300 / 360), rotation=k.quat("y", 210)),
            revolve("rain-cap-knuckle", CAP, GS, "detail", 0.3, [(0, 0.02), (0.12, 0.02)], [0, 0, 1], (0.0, 0.0, k.SIL_Z - 0.06), world=False),
        ]
    if level == "coarse":
        return [revolve("rain-cap", CAP, GS, "body", 0.3, [(0.0, k.CAP_R), (-0.045, 0.13), (-0.08, 0.075), (-0.1, 0.0)], [0, 1, 0], at, world=False)]
    return [revolve("rain-cap", CAP, GS, "body", 0.3, [(0.0, k.CAP_R), (-0.07, 0.11), (-0.1, 0.0)], [0, 1, 0], at, world=False)]


def hood_plates(t, frame, splitters, roof):
    """🌬️ The radiator discharge hood of sheet plates `t` thick: two pentagon sides, sloped back, top, back, the flap seat frame and at 1:50 the attenuator splitters."""
    p = [(x, max(y, roof)) for x, y in k.HOOD]
    hz = k.HOOD_Z
    dx, dy = 2.2 - 1.9, 3.25 - 2.97
    n = math.hypot(dx, dy)
    ox, oy = dy / n * t, -dx / n * t
    out = [
        extrude("hood-side-l", HD, CS, "body", 0.4, "xy", p, hz - t, hz),
        extrude("hood-side-r", HD, CS, "body", 0.4, "xy", p, -hz, -hz + t),
        box("hood-top", HD, CS, "body", 0.4, 2.2, L2, 3.25 - t, 3.25, -hz + t, hz - t),
        extrude("hood-slope", HD, CS, "body", 0.4, "xy", [(1.9, 2.97), (2.2, 3.25), (2.2 + ox, 3.25 + oy), (1.9 + ox, 2.97 + oy)], -hz + t, hz - t),
        box("hood-back", HD, CS, "body", 0.4, 1.9, 1.9 + t, roof, 2.97, -hz + t, hz - t),
    ]
    if frame:
        out.append(extrude("hood-frame", HD, CS, "detail", 0.15, "yz", [(roof, -hz + t), (roof, hz - t), (3.25 - t, hz - t), (3.25 - t, -hz + t)], 2.97 - frame, 2.97, holes=[[(k.MOUTH_Y[0], -k.MOUTH_Z), (k.MOUTH_Y[0], k.MOUTH_Z), (k.MOUTH_Y[1], k.MOUTH_Z), (k.MOUTH_Y[1], -k.MOUTH_Z)]]))
    if splitters:
        out.append(box("hood-splitters", HD, CS, "detail", 0.28, 2.45, 2.95, roof, 3.2, -0.65, -0.55, repeat={"count": 4, "translation": [0.0, 0.0, 0.4]}))
    return out


def hood_block(xe, roof):
    """🌬️ The discharge hood as one closed pentagon prism up to the flap (coarse scales)."""
    p = [(1.9, roof), (xe, roof), (xe, 3.25), (2.2, 3.25), (1.9, 2.97)]
    return extrude("discharge-hood", HD, CS, "body", 0.4, "xy", p, -k.HOOD_Z, k.HOOD_Z)


def flap_parts(t, knuckle, n):
    """🚪 The gravity weather flap (flap frame: u = 3.25 - y down the flap, v = x - 3.03, w lateral) and its hinge knuckles."""
    out = [lbox("outlet-flap", FL, GS, "body", 0.5, 0.0, k.FLAP[0], -t, 0.0, -k.FLAP[2] / 2, k.FLAP[2] / 2, radius=0.005 if t < 0.07 else None)]
    if knuckle:
        step = 1.4 / (n - 1)
        out.append(revolve("outlet-flap-knuckle", FL, GS, "detail", 0.3, [(0, knuckle), (0.12, knuckle)], [0, 0, 1], (0.0, 0.0, -0.76), world=False, repeat={"count": n, "translation": [0.0, 0.0, R5(step)]}))
    return out


def louvre_frames(z_in, z_out):
    """🪟 The aluminium frames of the four inlet louvres (both long sides), 50 mm wide."""
    (a0, a1), (b0, b1) = k.LOUVRES
    y0, y1 = k.LOUVRE_Y
    f = k.LOUVRE_FRAME
    outer = [(a0 - f, y0 - f), (a1 + f, y0 - f), (a1 + f, y1 + f), (a0 - f, y1 + f)]
    hole = [[(a0, y0), (a1, y0), (a1, y1), (a0, y1)]]
    rep = {"count": 2, "translation": [R5(b0 - a0), 0.0, 0.0]}
    return [
        extrude("louvre-frame-r", C, AL, "detail", 0.45, "xy", outer, -z_out, -z_in, holes=hole, repeat=rep),
        extrude("louvre-frame-l", C, AL, "detail", 0.45, "xy", outer, z_in, z_out, holes=hole, repeat=rep),
    ]


def louvre_slats(count, width, thick, zc):
    """🪟 The louvre blades, sloping down outwards (rain), as texture sheets: one solid per panel and side."""
    (a0, a1), (b0, b1) = k.LOUVRES
    y0, y1 = k.LOUVRE_Y
    pitch = (y1 - y0) / count
    out = []
    for name, (x0, x1) in (("1", (a0, a1)), ("2", (b0, b1))):
        for side, sz in (("r", -1), ("l", 1)):
            out.append({"id": f"inlet-louvre-{name}-{side}-slat", "kind": "box", "bone": C, "material": AL, "role": "detail", "tone": 0.25, "texture": True,
                        "size": [R5(x1 - x0), thick, width],
                        "transform": {"translation": [R5((x0 + x1) / 2 + L2), R5(y0 + pitch / 2), R5(sz * zc)], "rotation": k.quat("x", 30 * sz)},
                        "repeat": {"count": count, "translation": [0.0, R5(pitch), 0.0]}})
    return out


def attenuators(z_wall, t):
    """🔇 The dark perforated faces of the inlet attenuators behind the louvre blades (texture sheets)."""
    (a0, a1), (b0, b1) = k.LOUVRES
    y0, y1 = k.LOUVRE_Y
    rep = {"count": 2, "translation": [R5(b0 - a0), 0.0, 0.0]}
    return [
        box("attenuator-r", C, CS, "detail", 0.1, a0, a1, y0, y1, -z_wall - t, -z_wall, texture=True, repeat=rep),
        box("attenuator-l", C, CS, "detail", 0.1, a0, a1, y0, y1, z_wall, z_wall + t, texture=True, repeat=rep),
    ]


def doors(t, fine):
    """🚪 The two side doors on the drawn side and the end door at the radiator end, with hinges, lever handles and the drip rail at 1:50."""
    zw = k.WALL_Z
    y0, y1 = k.DOOR_Y
    (d10, d11), (d20, d21) = k.DOORS
    out = [
        box("door-1", C, CS, "detail", 0.52, d10, d11, y0, y1, -zw - t, -zw),
        box("door-2", C, CS, "detail", 0.52, d20, d21, y0, y1, -zw - t, -zw),
        box("end-door", C, CS, "detail", 0.52, 3.0, 3.0 + t, y0, y1, k.END_DOOR_Z[0], k.END_DOOR_Z[1]),
    ]
    if fine:
        hinge = [(0, 0.012), (0.1, 0.012)]
        rep = {"count": 3, "translation": [0.0, 0.65, 0.0]}
        out += [
            revolve("door-1-hinge", C, CS, "detail", 0.25, hinge, [0, 1, 0], (d11, 0.6, -zw - 0.015), repeat=rep),
            revolve("door-2-hinge", C, CS, "detail", 0.25, hinge, [0, 1, 0], (d21, 0.6, -zw - 0.015), repeat=rep),
            revolve("end-door-hinge", C, CS, "detail", 0.25, hinge, [0, 1, 0], (3.0 + 0.015, 0.6, k.END_DOOR_Z[0]), repeat=rep),
            box("door-handle", C, CS, "detail", 0.2, 1.55, 1.61, 1.2, 1.5, -zw - t - 0.012, -zw - t),
            box("door-handle-lever", C, CS, "detail", 0.2, 1.56, 1.7, 1.3, 1.325, -zw - t - 0.03, -zw - t - 0.012),
            box("door-handle-2", C, CS, "detail", 0.2, 2.4, 2.46, 1.2, 1.5, -zw - t - 0.012, -zw - t),
            box("door-handle-2-lever", C, CS, "detail", 0.2, 2.41, 2.55, 1.3, 1.325, -zw - t - 0.03, -zw - t - 0.012),
            box("end-door-handle", C, CS, "detail", 0.2, 3.0 + t, 3.0 + t + 0.012, 1.2, 1.5, 0.39, 0.45),
            box("end-door-handle-lever", C, CS, "detail", 0.2, 3.0 + t + 0.012, 3.0 + t + 0.03, 1.3, 1.325, 0.27, 0.41),
            box("drip-rail", C, CS, "detail", 0.4, 1.45, 2.9, 2.18, 2.2, -W2, -zw),
        ]
    return out


def emitters(t):
    """💨 The two outlets as patches `t` thick flush with the surface they leave (D2): the bore of the stack mouth and the hood mouth in front of the flap."""
    roof = k.origin(HD)[1]
    my = (k.MOUTH_Y[0] + k.MOUTH_Y[1]) / 2
    r = (k.MOUTH_Y[1] - k.MOUTH_Y[0]) / 2
    return [
        {"id": "th-exhaust-stack-jet", "kind": "revolve", "bone": ST, "material": ES, "role": "detail", "tone": 0.2, "region": "exhaust-stack", "tags": ["exhaust-stack-jet"],
         "profile": [[0.0, 0.094], [R5(t), 0.094]], "axis": [1, 0, 0], "origin": [R5(0.33 - t), 0.0, k.SIL_Z]},
        {"id": "th-radiator-air", "kind": "revolve", "bone": HD, "material": CS, "role": "detail", "tone": 0.4, "region": "discharge-hood", "tags": ["radiator-air"],
         "profile": [[0.0, R5(r)], [R5(t), R5(r)]], "axis": [1, 0, 0], "origin": [R5(L2 - 1.9), R5(my - roof), 0.0], "lateralScale": R5(0.8 / r)},
    ]


def wall_plates(t, layer, inward, tag):
    """🧱 Wall shell plates of thickness `t` on the outer faces of the wall core (outward = inward False) or behind them, between the posts; their interior faces are C31 stand-in section faces."""
    zw, xw = k.WALL_Z, k.WALL_X
    px, pz = k.POST
    y0, y1 = k.WALL_Y
    xs = L2 - px
    zs = W2 - pz
    d = 0.002 if inward else 0.0
    out = []
    for side, sz in (("r", -1), ("l", 1)):
        za, zb = sorted((sz * (zw - d), sz * (zw - d - t)))
        out.append(box(f"th-wall-{tag}-{side}", C, CS, "body", 0.6 if tag == "skin" else 0.8, -xs, xs, y0, y1, za, zb, sectionFaces=[("+w" if sz < 0 else "-w")]))
        out[-1]["region"] = "enclosure-walls"
        out[-1]["layer"] = layer
    for end, sx in (("a", -1), ("b", 1)):
        xa, xb = sorted((sx * (xw - d), sx * (xw - d - t)))
        out.append(box(f"th-wall-{tag}-{end}", C, CS, "body", 0.6 if tag == "skin" else 0.8, xa, xb, y0, k.TOP_END_Y[0], -zs, zs, sectionFaces=[("+u" if sx < 0 else "-u")]))
        out[-1]["region"] = "enclosure-walls"
        out[-1]["layer"] = layer
    return out


def reorder(s):
    """🧾 Moves `region`/`layer`/`member`/`part`/`anchor` right after `tone` in a solid of its own (member order of the catalog)."""
    if "of" in s:
        return s
    head = ["id", "kind", "bone", "material", "role", "tone", "region", "layer", "member", "part", "anchor"]
    out = {key: s[key] for key in head if key in s}
    out.update({key: v for key, v in s.items() if key not in out})
    return out


def own(solid, **kw):
    """🏷️ A solid of its own in the thermal or structure list."""
    solid = dict(solid)
    solid.update(kw)
    return reorder(solid)


def s50():
    """🔎 1:50 (smallest feature 7.5 mm)."""
    roof = k.ROOF_TOP
    px, pz = k.POST
    body = castings("casting", True)
    body += posts(k.CAST[1], k.H - k.CAST[1], px, pz, 0.45)
    body += [
        box("bottom-rail", C, CS, "body", 0.42, -k.CAST_IN_X, k.CAST_IN_X, *k.RAIL_Y, -W2, -W2 + k.RAIL_D, repeat={"count": 2, "translation": [0.0, 0.0, R5(2 * W2 - k.RAIL_D)]}),
        box("bottom-end-rail", C, CS, "body", 0.42, -L2, -k.CAST_IN_X, *k.RAIL_Y, -k.CAST_IN_Z, k.CAST_IN_Z, repeat={"count": 2, "translation": [R5(2 * L2 - k.CAST[0]), 0.0, 0.0]}),
        box("skid-tank", C, CS, "body", 0.3, -k.CAST_IN_X, k.CAST_IN_X, k.RAIL_Y[0] + 0.005, k.RAIL_Y[1] - 0.005, -k.TANK_Z, k.TANK_Z),
        box("top-rail-r", C, CS, "body", 0.45, -k.CAST_IN_X, k.CAST_IN_X, *k.TOP_RAIL_Y, -W2, -W2 + k.TOP_RAIL_D),
        box("top-rail-l", C, CS, "body", 0.45, -k.CAST_IN_X, k.CAST_IN_X, *k.TOP_RAIL_Y, W2 - k.TOP_RAIL_D, W2),
        box("top-end-rail-a", C, CS, "body", 0.45, -L2, -L2 + k.TOP_RAIL_D, *k.TOP_END_Y, -k.CAST_IN_Z, k.CAST_IN_Z),
        box("top-end-rail-b", C, CS, "body", 0.45, L2 - k.TOP_RAIL_D, L2, *k.TOP_END_Y, -k.CAST_IN_Z, k.CAST_IN_Z),
        box("wall", C, CS, "body", 0.55, -k.WALL_X, k.WALL_X, *k.WALL_Y, -k.WALL_Z, k.WALL_Z),
        box("roof", C, CS, "body", 0.5, -k.ROOF_X, k.ROOF_X, k.WALL_Y[1], roof, -k.ROOF_Z, k.ROOF_Z),
    ]
    body += louvre_frames(k.WALL_Z, W2)
    body += louvre_slats(15, 0.035, 0.002, 1.2045)
    body += attenuators(k.WALL_Z, 0.006)
    body += doors(0.012, True)
    (f0, f1), (fy0, fy1) = k.FUEL_FILL
    body.append(box("fuel-fill", C, CS, "detail", 0.2, f0, f1, fy0, fy1, -W2 - 0.008, -W2))
    eye = [(k.EYE_X[0], roof), (k.EYE_X[1], roof), (k.EYE_X[1] - 0.06, roof + 0.08), (k.EYE_X[0] + 0.06, roof + 0.08)]
    body.append(extrude("lifting-eye", C, CS, "detail", 0.2, "xy", eye, -k.EYE_Z - 0.015, -k.EYE_Z + 0.015, holes=[circle(2.65, roof + 0.04, 0.02)], repeat={"count": 2, "translation": [0.0, 0.0, R5(2 * k.EYE_Z)]}))
    body += silencer_parts(6, (0.06, 0.008), 0.012, 0.012, roof, "bellows")
    body += stack_parts(True, True, 0.04)
    body += cap_parts("fine")
    body += hood_plates(0.008, 0.008, True, roof)
    body += flap_parts(k.FLAP[1], 0.016, 3)

    skin = {"region": "enclosure-walls", "layer": "skin"}
    thermal = [own(s, **{"id": s["id"].replace("-skin-", "-wool-")}) for s in wall_plates(0.05, "wool", True, "wool")]
    thermal += wall_plates(0.002, "skin", False, "skin")
    thermal = [reorder(s) for s in thermal]
    for end in ("a", "b"):
        for side in ("l", "r"):
            thermal.append(ref(f"th-post-{end}-{side}", f"post-{end}-{side}", sectionFaces=post_faces(end, side), **skin))
    thermal += [
        ref("th-top-rail-r", "top-rail-r", sectionFaces=["+w"], **skin),
        ref("th-top-rail-l", "top-rail-l", sectionFaces=["-w"], **skin),
        ref("th-top-end-rail-a", "top-end-rail-a", sectionFaces=["+u"], **skin),
        ref("th-top-end-rail-b", "top-end-rail-b", sectionFaces=["-u"], **skin),
        ref("th-casting-top-a", "casting-top-a", region="roof", layer="sheet"),
        ref("th-casting-top-b", "casting-top-b", region="roof", layer="sheet"),
        ref("th-louvre-frame-r", "louvre-frame-r", **skin),
        ref("th-louvre-frame-l", "louvre-frame-l", **skin),
    ]
    for id in ("door-1", "door-2", "end-door", "door-1-hinge", "door-2-hinge", "end-door-hinge", "door-handle", "door-handle-lever", "door-handle-2", "door-handle-2-lever", "end-door-handle", "end-door-handle-lever", "drip-rail"):
        thermal.append(ref(f"th-{id}", id, **skin))
    thermal.append(own(box("th-roof-sheet", C, CS, "body", 0.5, -k.ROOF_X, k.ROOF_X, roof - 0.002, roof, -k.ROOF_Z, k.ROOF_Z), region="roof", layer="sheet", sectionFaces=["-v"]))
    thermal.append(ref("th-lifting-eye", "lifting-eye", region="roof", layer="sheet"))
    for n in ("1", "2"):
        for side in ("r", "l"):
            thermal.append(ref(f"th-inlet-louvre-{n}-{side}-slat", f"inlet-louvre-{n}-{side}-slat", region="inlet-louvres"))
    for id in ("skid-tank", "bottom-rail", "bottom-end-rail", "casting-bottom-a", "casting-bottom-b", "fuel-fill"):
        thermal.append(ref(f"th-{id}", id, region="skid-tank"))
    for id in ("silencer", "silencer-band", "silencer-saddle", "silencer-saddle-base", "silencer-inlet-collar", "silencer-inlet", "stack-spigot"):
        thermal.append(ref(f"th-{id}", id, region="silencer"))
    thermal.append(own(revolve("th-exhaust-stack", ST, ES, "body", 0.2, [(-0.04, 0.1), (0.33, 0.1)], [1, 0, 0], (0.0, 0.0, k.SIL_Z), world=False), region="exhaust-stack"))
    thermal.append(ref("th-stack-hinge-bracket", "stack-hinge-bracket", region="exhaust-stack"))
    for id in ("rain-cap", "rain-cap-skirt", "rain-cap-knuckle"):
        thermal.append(ref(f"th-{id}", id, region="rain-cap"))
    for id, faces in (("hood-side-l", ["-w"]), ("hood-side-r", ["+w"]), ("hood-top", ["-v"]), ("hood-slope", ["-v"]), ("hood-back", ["+u"])):
        thermal.append(ref(f"th-{id}", id, region="discharge-hood", sectionFaces=faces))
    thermal += [ref("th-outlet-flap", "outlet-flap", region="outlet-flap"), ref("th-outlet-flap-knuckle", "outlet-flap-knuckle", region="outlet-flap")]
    thermal += emitters(0.0077)

    structure = []
    for id in ("bottom-rail", "bottom-end-rail", "post-a-l", "post-a-r", "post-b-l", "post-b-r", "top-rail-r", "top-rail-l", "top-end-rail-a", "top-end-rail-b"):
        structure.append(ref(f"member-{id}", id, member=C, part="member"))
    structure += [
        own(box("member-cross-member", C, CS, "body", 0.35, -2.65, -2.55, 0.08, k.RAIL_Y[1], -k.TANK_Z, k.TANK_Z, repeat={"count": 9, "translation": [0.65, 0.0, 0.0]}), member=C, part="member"),
        own(box("member-skid-rail", C, CS, "body", 0.35, -2.45, 1.85, k.RAIL_Y[1], 0.36, 0.4, 0.5, repeat={"count": 2, "translation": [0.0, 0.0, -0.9]}), member=C, part="member"),
        own(revolve("joint-mount-l", C, CS, "detail", 0.2, [(0, 0.06), (0.07, 0.06)], [0, 1, 0], (-2.0, 0.36, 0.45), repeat={"count": 3, "translation": [1.7, 0.0, 0.0]}), member=C, part="joint"),
        own(revolve("joint-mount-r", C, CS, "detail", 0.2, [(0, 0.06), (0.07, 0.06)], [0, 1, 0], (-2.0, 0.36, -0.45), repeat={"count": 3, "translation": [1.7, 0.0, 0.0]}), member=C, part="joint"),
    ]
    for level in ("bottom", "top"):
        for end in ("a", "b"):
            structure.append(ref(f"joint-casting-{level}-{end}", f"casting-{level}-{end}", member=C, part="joint"))
    structure += supports(0.0077)
    structure += masses(0.3, 0.06)
    structure += [
        ref("member-silencer", "silencer", member=SIL, part="member"),
        ref("member-silencer-saddle", "silencer-saddle", member=SIL, part="member"),
        ref("joint-silencer-band", "silencer-band", member=SIL, part="joint"),
        ref("joint-silencer-saddle-base", "silencer-saddle-base", member=SIL, part="joint"),
        ref("member-exhaust-stack", "exhaust-stack", member=ST, part="member"),
        ref("joint-stack-spigot", "stack-spigot", member=ST, part="joint"),
        ref("joint-stack-hinge-bracket", "stack-hinge-bracket", member=ST, part="joint"),
        ref("member-rain-cap", "rain-cap", member=CAP, part="member"),
        ref("joint-rain-cap-knuckle", "rain-cap-knuckle", member=CAP, part="joint"),
    ]
    for id in ("hood-side-l", "hood-side-r", "hood-top", "hood-slope", "hood-back"):
        structure.append(ref(f"member-{id}", id, member=HD, part="member"))
    structure.append(hood_base_ring(0.03, roof))
    structure += [ref("member-outlet-flap", "outlet-flap", member=FL, part="member"), hinge_rod(0.006)]
    decisions = [
        T("The container is modelled as the real 20 ft ISO frame (ISO 668, ISO 1161): eight 178 x 162 x 118 mm corner castings with their oval side apertures, four 160 x 150 mm corner posts between them, bottom side rails 12.5 mm clear of the ground (only the castings stand on it), bottom end sills, the integral base fuel tank between the rails, top side rails and deeper top end rails, the flat insulated wall panels set 30 mm behind the frame and the roof sheet 14 mm below the tops of the castings.",
          "Der Container ist als echter 20-ft-ISO-Rahmen modelliert (ISO 668, ISO 1161): acht Eckbeschläge 178 x 162 x 118 mm mit ihren ovalen Seitenöffnungen, vier Eckpfosten 160 x 150 mm dazwischen, untere Längsträger 12,5 mm über dem Boden (nur die Beschläge stehen auf), untere Stirnschwellen, der integrierte Grundtank zwischen den Trägern, obere Längsträger und tiefere obere Stirnträger, die ebenen gedämmten Wandpaneele 30 mm hinter dem Rahmen und das Dachblech 14 mm unter der Oberkante der Beschläge."),
        T("Openings: four framed inlet louvres (both long sides) with fifteen 2 mm blades each that slope down outwards against rain and the dark faces of the acoustic attenuators behind them, two side doors and the end door with three hinges, lockable lever handles and a drip rail, the lockable fuel fill on the bottom rail and two lifting eyes on the roof.",
          "Öffnungen: vier gerahmte Zuluftlamellen (beide Längsseiten) mit je fünfzehn 2-mm-Lamellen, die gegen Regen nach außen fallen, dahinter die dunklen Flächen der Schalldämpfkulissen, zwei Seitentüren und die Stirntür mit drei Bändern, abschließbaren Hebelgriffen und Tropfleiste, der abschließbare Tankstutzen am unteren Träger und zwei Hebeösen auf dem Dach."),
        T("The silencer is a 0.57 m shell with 2:1 dished heads on two saddles with strap bands, raised 0.1 m above the roof, fed by a bellows inlet through a roof collar; the stack is a 6 mm wall pipe on a spigot with the hinge bracket; the rain cap is a domed lid with a skirt that is open at the hinge; the discharge hood is 8 mm sheet (sides, sloped back, top, back, the seat frame of the flap) with four attenuator splitters inside, and the flap hangs on three knuckles.",
          "Der Schalldämpfer ist ein Mantel von 0,57 m mit Klöpperböden 2:1 auf zwei Sätteln mit Spannbändern, 0,1 m über dem Dach, gespeist über einen Kompensator durch einen Dachkragen; das Abgasrohr ist ein Rohr mit 6 mm Wand auf einem Stutzen mit dem Scharnierbock; die Regenkappe ist ein gewölbter Deckel mit Schürze, die am Scharnier offen ist; die Austrittshaube ist aus 8-mm-Blech (Seiten, Schräge, Decke, Rückwand, Anschlagrahmen der Klappe) mit vier Kulissen innen, die Klappe hängt an drei Scharnieraugen."),
        T("Thermal: the walls are two nested shells per wall panel (2 mm skin on 50 mm wool) plus the frame, doors and louvre frames as skin; the roof is its 2 mm sheet; faces that look into the closed container or hood carry sectionFaces as the stand-in for the C31 flood the product does not have yet; every other region references its body solids, the exhaust stack is its closed outer cylinder; the exhaust outlet is a patch in the stack bore and the radiator outlet a 1.6 x 0.55 m patch in the hood mouth.",
          "Thermik: die Wände sind je Paneel zwei geschachtelte Schalen (2 mm Blech auf 50 mm Wolle) plus Rahmen, Türen und Lamellenrahmen als Blech; das Dach ist sein 2-mm-Blech; Flächen, die in den geschlossenen Container oder die Haube schauen, tragen sectionFaces als Ersatz für die C31-Flutung, die das Produkt noch nicht hat; alle anderen Regionen verweisen auf ihre Body-Körper, das Abgasrohr ist sein geschlossener Außenzylinder; der Abgasaustritt ist eine Fläche in der Rohrbohrung, der Kühlerluftaustritt eine Fläche von 1,6 x 0,55 m in der Haubenmündung."),
        T("Structure: the container frame carries everything to the four bottom castings: bottom side rails, end sills and nine floor cross members, corner posts, top rails; the genset skid rails on six anti-vibration mounts carry the engine and alternator mass; silencer shell, saddles, stack, cap lid, hood panels and flap are members with their straps, saddle plates, spigot, brackets, hinge knuckles, hood base angle and flap hinge as joints; masses sit at the centres of mass of container, fuel, silencer and hood.",
          "Tragwerk: der Containerrahmen trägt alles zu den vier unteren Eckbeschlägen: untere Längsträger, Stirnschwellen und neun Bodenquerträger, Eckpfosten, obere Träger; die Grundrahmenträger des Aggregats auf sechs Schwingungsdämpfern tragen die Masse von Motor und Generator; Schalldämpfermantel, Sättel, Abgasrohr, Kappendeckel, Haubenwände und Klappe sind Stäbe, ihre Spannbänder, Sattelplatten, Stutzen, Böcke, Scharnieraugen, der Haubenfußwinkel und das Klappenscharnier Gelenke; Massen sitzen in den Schwerpunkten von Container, Kraftstoff, Schalldämpfer und Haube."),
    ]
    return authored(50, body, thermal, structure, decisions)


def supports(t):
    """🟩 One contact pad under each bottom corner casting at its anchor."""
    lx, _, lz = k.CAST
    out = []
    for anchor, sx, sz in (("contact-a-l", -1, 1), ("contact-a-r", -1, -1), ("contact-b-l", 1, 1), ("contact-b-r", 1, -1)):
        s = box(f"support-{anchor}", C, CS, "detail", 0.2, sx * k.CX - lx / 2, sx * k.CX + lx / 2, 0.0, t, sz * k.CZ - lz / 2, sz * k.CZ + lz / 2)
        out.append(own(s, member=C, part="support", anchor=anchor))
    return out


def masses(rc, rf, small=True):
    """⚫ Compact solids at the centres of mass: container with the set (rig mass 6554 kg at x 0, y 1.0), the fuel in the base tank (load anchor fuel-cog), silencer and hood."""
    out = [
        own(ellipsoid("mass-container", C, CS, "body", 0.2, (0.0, 1.0, 0.0), (rc, rc, rc)), member=C, part="mass"),
        own(ellipsoid("mass-fuel", C, CS, "body", 0.2, (0.0, 0.1, 0.0), (rf * 6, rf, rf * 5)), member=C, part="mass"),
    ]
    if small:
        out += [
            own(ellipsoid("mass-silencer", SIL, ES, "body", 0.2, ((k.SIL_X[0] + k.SIL_X[1]) / 2, k.SIL_Y, k.SIL_Z), (0.12, 0.12, 0.12)), member=SIL, part="mass"),
            own(ellipsoid("mass-hood", HD, CS, "body", 0.2, (2.5, k.ROOF_TOP + 0.318, 0.0), (0.1, 0.1, 0.1)), member=HD, part="mass"),
        ]
    return out


def hood_base_ring(t, roof):
    """🔩 The bolted base angle of the hood on the roof (joint)."""
    hz = k.HOOD_Z
    s = extrude("joint-hood-base", HD, CS, "detail", 0.25, "zx", [(-hz, 1.9), (hz, 1.9), (hz, L2), (-hz, L2)], roof, roof + t, holes=[[(-hz + 0.04, 1.94), (hz - 0.04, 1.94), (hz - 0.04, L2 - 0.04), (-hz + 0.04, L2 - 0.04)]])
    return own(s, member=HD, part="joint")


def hinge_rod(r):
    """🔩 The hinge axis of the weather flap (joint)."""
    s = revolve("joint-flap-hinge", FL, GS, "detail", 0.3, [(0, r), (k.FLAP[2], r)], [0, 0, 1], (0.0, 0.0, -k.FLAP[2] / 2), world=False)
    return own(s, member=FL, part="joint")


def authored(d, body, thermal, structure, decisions):
    """📦 An authored scale volume."""
    return {"status": "authored", "denominator": d, "body": [reorder(s) for s in body], "thermal": thermal, "structure": structure, "decisions": decisions}


def s100():
    """🔎 1:100 (smallest feature 15 mm)."""
    roof = k.ROOF_TOP
    px, pz = k.POST
    body = castings("casting", False)
    body += posts(k.CAST[1], k.H - k.CAST[1], px, pz, 0.45)
    body += [
        box("base", C, CS, "body", 0.32, -k.CAST_IN_X, k.CAST_IN_X, *k.RAIL_Y, -W2, W2),
        box("bottom-end-rail", C, CS, "body", 0.42, -L2, -k.CAST_IN_X, *k.RAIL_Y, -k.CAST_IN_Z, k.CAST_IN_Z, repeat={"count": 2, "translation": [R5(2 * L2 - k.CAST[0]), 0.0, 0.0]}),
        box("top-rail-r", C, CS, "body", 0.45, -k.CAST_IN_X, k.CAST_IN_X, *k.TOP_RAIL_Y, -W2, -W2 + k.TOP_RAIL_D),
        box("top-rail-l", C, CS, "body", 0.45, -k.CAST_IN_X, k.CAST_IN_X, *k.TOP_RAIL_Y, W2 - k.TOP_RAIL_D, W2),
        box("top-end-rail-a", C, CS, "body", 0.45, -L2, -L2 + k.TOP_RAIL_D, *k.TOP_END_Y, -k.CAST_IN_Z, k.CAST_IN_Z),
        box("top-end-rail-b", C, CS, "body", 0.45, L2 - k.TOP_RAIL_D, L2, *k.TOP_END_Y, -k.CAST_IN_Z, k.CAST_IN_Z),
        box("wall", C, CS, "body", 0.55, -k.WALL_X, k.WALL_X, *k.WALL_Y, -k.WALL_Z, k.WALL_Z),
        box("roof", C, CS, "body", 0.5, -k.ROOF_X, k.ROOF_X, k.WALL_Y[1], roof, -k.ROOF_Z, k.ROOF_Z),
    ]
    body += louvre_frames(k.WALL_Z, W2)
    body += louvre_slats(8, 0.05, 0.004, 1.2045)
    body += attenuators(k.WALL_Z, 0.006)
    body += doors(0.015, False)
    body += silencer_parts(4, (0.06, 0.015), 0.016, 0.016, roof, "pipe")
    body += stack_parts(False, True, 0.04)
    body += cap_parts("mid")
    body += hood_plates(0.016, 0.016, False, roof)
    body += flap_parts(k.FLAP[1], 0.016, 3)

    walls = {"region": "enclosure-walls"}
    thermal = [ref("th-wall", "wall", sectionFaces=["-v", "+v"], **walls)]
    for end in ("a", "b"):
        for side in ("l", "r"):
            thermal.append(ref(f"th-post-{end}-{side}", f"post-{end}-{side}", sectionFaces=post_faces(end, side), **walls))
    thermal += [
        ref("th-top-rail-r", "top-rail-r", sectionFaces=["+w"], **walls),
        ref("th-top-rail-l", "top-rail-l", sectionFaces=["-w"], **walls),
        ref("th-top-end-rail-a", "top-end-rail-a", sectionFaces=["+u"], **walls),
        ref("th-top-end-rail-b", "top-end-rail-b", sectionFaces=["-u"], **walls),
        ref("th-casting-top-a", "casting-top-a", region="roof"),
        ref("th-casting-top-b", "casting-top-b", region="roof"),
        ref("th-louvre-frame-r", "louvre-frame-r", **walls),
        ref("th-louvre-frame-l", "louvre-frame-l", **walls),
        ref("th-door-1", "door-1", **walls),
        ref("th-door-2", "door-2", **walls),
        ref("th-end-door", "end-door", **walls),
        ref("th-roof", "roof", region="roof", sectionFaces=["-v"]),
    ]
    for n in ("1", "2"):
        for side in ("r", "l"):
            thermal.append(ref(f"th-inlet-louvre-{n}-{side}-slat", f"inlet-louvre-{n}-{side}-slat", region="inlet-louvres"))
    for id in ("base", "bottom-end-rail", "casting-bottom-a", "casting-bottom-b"):
        thermal.append(ref(f"th-{id}", id, region="skid-tank"))
    for id in ("silencer", "silencer-band", "silencer-saddle", "silencer-saddle-base", "silencer-inlet", "stack-spigot"):
        thermal.append(ref(f"th-{id}", id, region="silencer"))
    thermal += [ref("th-exhaust-stack", "exhaust-stack", region="exhaust-stack"), ref("th-stack-hinge-bracket", "stack-hinge-bracket", region="exhaust-stack")]
    thermal += [ref("th-rain-cap", "rain-cap", region="rain-cap"), ref("th-rain-cap-skirt", "rain-cap-skirt", region="rain-cap"), ref("th-rain-cap-knuckle", "rain-cap-knuckle", region="rain-cap")]
    for id, faces in (("hood-side-l", ["-w"]), ("hood-side-r", ["+w"]), ("hood-top", ["-v"]), ("hood-slope", ["-v"]), ("hood-back", ["+u"])):
        thermal.append(ref(f"th-{id}", id, region="discharge-hood", sectionFaces=faces))
    thermal += [ref("th-outlet-flap", "outlet-flap", region="outlet-flap"), ref("th-outlet-flap-knuckle", "outlet-flap-knuckle", region="outlet-flap")]
    thermal += emitters(0.0155)

    structure = [
        own(box("member-bottom-rail", C, CS, "body", 0.35, -k.CAST_IN_X, k.CAST_IN_X, *k.RAIL_Y, -W2, -W2 + k.RAIL_D, repeat={"count": 2, "translation": [0.0, 0.0, R5(2 * W2 - k.RAIL_D)]}), member=C, part="member"),
        ref("member-bottom-end-rail", "bottom-end-rail", member=C, part="member"),
    ]
    for id in ("post-a-l", "post-a-r", "post-b-l", "post-b-r", "top-rail-r", "top-rail-l", "top-end-rail-a", "top-end-rail-b"):
        structure.append(ref(f"member-{id}", id, member=C, part="member"))
    structure += [
        own(box("member-cross-member", C, CS, "body", 0.35, -2.65, -2.55, 0.08, k.RAIL_Y[1], -k.TANK_Z, k.TANK_Z, repeat={"count": 5, "translation": [1.3, 0.0, 0.0]}), member=C, part="member"),
        own(box("member-skid-rail", C, CS, "body", 0.35, -2.45, 1.85, k.RAIL_Y[1], 0.36, 0.4, 0.5, repeat={"count": 2, "translation": [0.0, 0.0, -0.9]}), member=C, part="member"),
        own(revolve("joint-mount", C, CS, "detail", 0.2, [(0, 0.06), (0.07, 0.06)], [0, 1, 0], (-2.0, 0.36, 0.45), repeat={"count": 2, "translation": [0.0, 0.0, -0.9]}), member=C, part="joint"),
        own(revolve("joint-mount-front", C, CS, "detail", 0.2, [(0, 0.06), (0.07, 0.06)], [0, 1, 0], (1.4, 0.36, 0.45), repeat={"count": 2, "translation": [0.0, 0.0, -0.9]}), member=C, part="joint"),
    ]
    for level in ("bottom", "top"):
        for end in ("a", "b"):
            structure.append(ref(f"joint-casting-{level}-{end}", f"casting-{level}-{end}", member=C, part="joint"))
    structure += supports(0.0155)
    structure += masses(0.28, 0.06, small=False)
    structure += [
        ref("member-silencer", "silencer", member=SIL, part="member"),
        ref("member-silencer-saddle", "silencer-saddle", member=SIL, part="member"),
        ref("joint-silencer-band", "silencer-band", member=SIL, part="joint"),
        ref("member-exhaust-stack", "exhaust-stack", member=ST, part="member"),
        ref("joint-stack-hinge-bracket", "stack-hinge-bracket", member=ST, part="joint"),
        ref("member-rain-cap", "rain-cap", member=CAP, part="member"),
        ref("joint-rain-cap-knuckle", "rain-cap-knuckle", member=CAP, part="joint"),
    ]
    for id in ("hood-side-l", "hood-side-r", "hood-top", "hood-slope", "hood-back"):
        structure.append(ref(f"member-{id}", id, member=HD, part="member"))
    structure.append(hood_base_ring(0.03, roof))
    structure += [ref("member-outlet-flap", "outlet-flap", member=FL, part="member"), hinge_rod(0.012)]
    decisions = [
        T("At 1:100 the frame keeps castings (plain blocks, the 90 mm apertures are below the scale), posts, top rails and end sills; the bottom side rails and the tank merge into one base block between the end sills; the louvres keep their frames with eight 50 mm blades each; doors are 15 mm leaves without hinges, handles, drip rail, fuel fill and lifting eyes.",
          "Bei 1:100 behält der Rahmen Beschläge (einfache Blöcke, die 90-mm-Öffnungen liegen unter dem Maßstab), Pfosten, obere Träger und Stirnschwellen; untere Längsträger und Tank verschmelzen zu einem Sockelblock zwischen den Stirnschwellen; die Lamellen behalten ihre Rahmen mit je acht 50-mm-Lamellen; Türen sind 15-mm-Blätter ohne Bänder, Griffe, Tropfleiste, Tankstutzen und Hebeösen."),
        T("Silencer with 2:1 heads, 15 mm strap bands and saddles, a plain inlet pipe with its roof collar; the stack is a solid pipe on its spigot; the rain cap is one domed lid with its skirt; the hood is 16 mm sheet without splitters; the flap keeps three knuckles.",
          "Schalldämpfer mit Klöpperböden 2:1, 15-mm-Spannbändern und Sätteln, ein glattes Eintrittsrohr mit Dachkragen; das Abgasrohr ist ein volles Rohr auf seinem Stutzen; die Regenkappe ist ein gewölbter Deckel mit Schürze; die Haube ist aus 16-mm-Blech ohne Kulissen; die Klappe behält drei Scharnieraugen."),
        T("Thermal: the walls are the wall core with its top and bottom marked as the closed interior (no layer shells at this scale), with frame, doors and louvre frames; the roof is the roof slab with its underside marked; the other regions reference their body solids; emitters are 15.5 mm patches.",
          "Thermik: die Wände sind der Wandkern, Ober- und Unterseite als geschlossenes Inneres markiert (keine Schichtschalen in diesem Maßstab), mit Rahmen, Türen und Lamellenrahmen; das Dach ist die Dachplatte mit markierter Unterseite; die übrigen Regionen verweisen auf ihre Body-Körper; Emitter sind Flächen von 15,5 mm."),
        T("Structure: bottom side rails inside the base block, end sills, five floor cross members, posts and top rails to the castings and supports; skid rails on four mounts; one mass for the container with the set and one for the fuel; silencer, saddles, stack, cap, hood panels and flap as at 1:50.",
          "Tragwerk: untere Längsträger im Sockelblock, Stirnschwellen, fünf Bodenquerträger, Pfosten und obere Träger zu den Beschlägen und Auflagern; Grundrahmenträger auf vier Lagern; eine Masse für Container mit Aggregat und eine für den Kraftstoff; Schalldämpfer, Sättel, Rohr, Kappe, Haubenwände und Klappe wie bei 1:50."),
    ]
    return authored(100, body, thermal, structure, decisions)


def coarse_container(d, roof_band, wall_z, post_w, louvre_t, tone_base):
    """📦 The container of 1:200 and 1:500: base band and top band over the full footprint (castings merged), four corner posts between them, the wall block set back and the louvre fields flush with the frame plane."""
    yb, yt = 0.2, k.H - roof_band
    body = [
        box("base", C, CS, "body", tone_base, -L2, L2, 0.0, yb, -W2, W2),
        box("roof", C, CS, "body", 0.45, -L2, L2, yt, k.H, -W2, W2),
    ]
    for end, sx in (("a", -1), ("b", 1)):
        xa, xb = sorted((sx * L2, sx * (L2 - post_w)))
        body.append(box(f"post-{end}", C, CS, "body", 0.42, xa, xb, yb, yt, -W2, -W2 + post_w, repeat={"count": 2, "translation": [0.0, 0.0, R5(2 * W2 - post_w)]}))
    body.append(box("wall", C, CS, "body", 0.55, -(L2 - (W2 - wall_z)), L2 - (W2 - wall_z), yb, yt, -wall_z, wall_z))
    (a0, a1), (b0, b1) = k.LOUVRES
    f = k.LOUVRE_FRAME
    y0, y1 = k.LOUVRE_Y
    rep = {"count": 2, "translation": [R5(b0 - a0), 0.0, 0.0]}
    body.append(box("inlet-louvre-r", C, AL, "detail", 0.22, a0 - f, a1 + f, y0 - f, y1 + f, -W2, -W2 + louvre_t, repeat=rep))
    body.append(box("inlet-louvre-l", C, AL, "detail", 0.22, a0 - f, a1 + f, y0 - f, y1 + f, W2 - louvre_t, W2, repeat=rep))
    return body


def coarse_lists(d, body, fl, rail_w, cross, rc, rf):
    """🧩 Thermal and structure of 1:200 and 1:500 from their own body."""
    feat = 0.15e-3 * d * 1.03
    thermal = [ref("th-wall", "wall", region="enclosure-walls", sectionFaces=["-v", "+v"])]
    thermal += [ref("th-post-a", "post-a", region="enclosure-walls"), ref("th-post-b", "post-b", region="enclosure-walls")]
    thermal.append(ref("th-roof", "roof", region="roof", sectionFaces=["-v"]))
    thermal += [ref("th-inlet-louvre-r", "inlet-louvre-r", region="inlet-louvres"), ref("th-inlet-louvre-l", "inlet-louvre-l", region="inlet-louvres")]
    thermal.append(ref("th-base", "base", region="skid-tank"))
    thermal += [ref("th-silencer", "silencer", region="silencer"), ref("th-silencer-saddle", "silencer-saddle", region="silencer")]
    thermal.append(ref("th-exhaust-stack", "exhaust-stack", region="exhaust-stack"))
    thermal.append(ref("th-rain-cap", "rain-cap", region="rain-cap"))
    thermal.append(ref("th-discharge-hood", "discharge-hood", region="discharge-hood", sectionFaces=["-v"]))
    thermal.append(ref("th-outlet-flap", "outlet-flap", region="outlet-flap"))
    thermal += emitters(R5(feat))
    ib = 0.0
    structure = [
        own(box("member-bottom-rail", C, CS, "body", 0.35, -k.CAST_IN_X, k.CAST_IN_X, k.RAIL_Y[0], k.RAIL_Y[1], -W2 + ib, -W2 + ib + rail_w, repeat={"count": 2, "translation": [0.0, 0.0, R5(2 * W2 - 2 * ib - rail_w)]}), member=C, part="member"),
        own(box("member-bottom-end-rail", C, CS, "body", 0.35, -L2, -L2 + rail_w, k.RAIL_Y[0], k.RAIL_Y[1], -k.CAST_IN_Z, k.CAST_IN_Z, repeat={"count": 2, "translation": [R5(2 * L2 - rail_w), 0.0, 0.0]}), member=C, part="member"),
        ref("member-post-a", "post-a", member=C, part="member"),
        ref("member-post-b", "post-b", member=C, part="member"),
        own(box("member-top-rail", C, CS, "body", 0.35, -k.CAST_IN_X, k.CAST_IN_X, k.H - 0.12, k.H - 0.008, -W2, -W2 + rail_w, repeat={"count": 2, "translation": [0.0, 0.0, R5(2 * W2 - rail_w)]}), member=C, part="member"),
        own(box("member-top-end-rail", C, CS, "body", 0.35, -L2, -L2 + rail_w, k.H - 0.15, k.H - 0.008, -k.CAST_IN_Z, k.CAST_IN_Z, repeat={"count": 2, "translation": [R5(2 * L2 - rail_w), 0.0, 0.0]}), member=C, part="member"),
        own(box("member-cross-member", C, CS, "body", 0.35, -cross[0] - 0.05, -cross[0] + 0.05, 0.08, k.RAIL_Y[1], -k.TANK_Z, k.TANK_Z, repeat={"count": cross[1], "translation": [R5(2 * cross[0] / (cross[1] - 1)), 0.0, 0.0]}), member=C, part="member"),
        own(box("member-skid-rail", C, CS, "body", 0.35, -2.45, 1.85, 0.2, 0.36, 0.4, 0.5, repeat={"count": 2, "translation": [0.0, 0.0, -0.9]}), member=C, part="member"),
    ]
    lx, ly, lz = k.CAST
    for level, (y0, y1) in (("bottom", (0.0, ly)), ("top", (k.H - ly, k.H))):
        for end, sx in (("a", -1), ("b", 1)):
            x0, x1 = sorted((sx * L2, sx * (L2 - lx)))
            structure.append(own(box(f"joint-casting-{level}-{end}", C, CS, "detail", 0.2, x0, x1, y0, y1, -W2, -W2 + lz, repeat={"count": 2, "translation": [0.0, 0.0, R5(2 * W2 - lz)]}), member=C, part="joint"))
    structure += supports(R5(feat))
    structure += masses(rc, rf, small=False)
    structure += [
        ref("member-silencer", "silencer", member=SIL, part="member"),
        ref("member-silencer-saddle", "silencer-saddle", member=SIL, part="member"),
        ref("member-exhaust-stack", "exhaust-stack", member=ST, part="member"),
        ref("member-rain-cap", "rain-cap", member=CAP, part="member"),
    ]
    p = [(1.9, k.H), (fl, k.H), (fl, 3.25), (2.2, 3.25), (1.9, 2.97)]
    t = max(0.03, feat)
    structure += [
        own(extrude("member-hood-side-l", HD, CS, "body", 0.4, "xy", p, k.HOOD_Z - t, k.HOOD_Z), member=HD, part="member"),
        own(extrude("member-hood-side-r", HD, CS, "body", 0.4, "xy", p, -k.HOOD_Z, -k.HOOD_Z + t), member=HD, part="member"),
        own(box("member-hood-top", HD, CS, "body", 0.4, 2.2, fl, 3.25 - t, 3.25, -k.HOOD_Z + t, k.HOOD_Z - t), member=HD, part="member"),
        ref("member-outlet-flap", "outlet-flap", member=FL, part="member"),
        hinge_rod(max(0.016, feat / 2 + 0.001)),
    ]
    return thermal, structure


def s200():
    """🔎 1:200 (smallest feature 30 mm)."""
    body = coarse_container(200, 0.15, 1.19, 0.15, 0.03, 0.3)
    body += silencer_parts(3, None, 0.03, 0.0, k.H, None)
    body += stack_parts(False, False, None)
    body += cap_parts("coarse")
    body.append(hood_block(2.96, k.H))
    body += flap_parts(0.07, None, 0)
    thermal, structure = coarse_lists(200, body, 2.96, 0.06, (2.0, 3), 0.25, 0.06)
    decisions = [
        T("At 1:200 the castings merge into a 0.2 m base band and a 0.15 m top band over the full footprint (the 8 mm roof recess and the 12.5 mm rail clearance are below the scale), the four corner posts stay as 150 mm columns standing one feature proud of the wall block, the louvres are four dark 30 mm fields flush with the frame plane, the doors are drawn outlines only (a 15 mm leaf is half a feature).",
          "Bei 1:200 verschmelzen die Beschläge zu einem Sockelband von 0,2 m und einem Kopfband von 0,15 m über die ganze Grundfläche (die 8 mm Dachvertiefung und der 12,5 mm Bodenabstand liegen unter dem Maßstab), die vier Eckpfosten bleiben als 150-mm-Stützen, die ein Merkmal vor dem Wandblock stehen, die Lamellen sind vier dunkle 30-mm-Felder bündig mit der Rahmenebene, die Türen nur gezeichnete Umrisse (ein 15-mm-Blatt ist ein halbes Merkmal)."),
        T("Silencer with 2:1 heads on two 30 mm saddles (bands merged), solid stack without spigot and bracket, a domed rain cap without skirt, the hood as one closed prism up to the 70 mm flap.",
          "Schalldämpfer mit Klöpperböden 2:1 auf zwei 30-mm-Sätteln (Bänder verschmolzen), volles Abgasrohr ohne Stutzen und Bock, eine gewölbte Regenkappe ohne Schürze, die Haube als ein geschlossenes Prisma bis zur 70-mm-Klappe."),
        T("Thermal: wall block with top and bottom marked as the closed interior and the posts, top band with its underside marked, base band, louvre fields and the other parts by reference; emitters are 31 mm patches.",
          "Thermik: Wandblock mit Ober- und Unterseite als geschlossenes Inneres markiert und die Pfosten, Kopfband mit markierter Unterseite, Sockelband, Lamellenfelder und die übrigen Teile als Verweise; Emitter sind 31-mm-Flächen."),
        T("Structure: 60 mm bottom side rails and end sills, three cross members, posts, top rails and end rails, the eight castings as joints, skid rails, supports, two masses; silencer, saddles, stack, cap, two hood side plates with the top plate, flap and hinge.",
          "Tragwerk: 60-mm-Längsträger und Stirnschwellen unten, drei Querträger, Pfosten, obere Längs- und Stirnträger, die acht Beschläge als Gelenke, Grundrahmenträger, Auflager, zwei Massen; Schalldämpfer, Sättel, Rohr, Kappe, zwei Haubenseitenwände mit Deckblech, Klappe und Scharnier."),
    ]
    return authored(200, body, thermal, structure, decisions)


def s500():
    """🔎 1:500 (smallest feature 75 mm)."""
    body = coarse_container(500, 0.15, 1.145, 0.15, 0.075, 0.28)
    body += silencer_parts(2, None, 0.08, 0.0, k.H, None)
    body += stack_parts(False, False, None)
    body += cap_parts("puck")
    body.append(hood_block(2.93, k.H))
    body += flap_parts(0.1, None, 0)
    thermal, structure = coarse_lists(500, body, 2.93, 0.08, (2.0, 3), 0.3, 0.06)
    decisions = [
        T("At 1:500 the container still reads as a genset container, not a cabinet: a dark 0.2 m base band and a 0.15 m top band (castings, rails and roof merged), four 150 mm corner posts standing 75 mm proud of the wall block, and the four inlet louvre fields as dark 75 mm panels flush with the frame plane.",
          "Bei 1:500 liest sich der Container noch als Aggregatcontainer, nicht als Schrank: ein dunkles Sockelband von 0,2 m und ein Kopfband von 0,15 m (Beschläge, Träger und Dach verschmolzen), vier 150-mm-Eckpfosten, die 75 mm vor dem Wandblock stehen, und die vier Zuluftfelder als dunkle 75-mm-Paneele bündig mit der Rahmenebene."),
        T("The silencer keeps its dished heads with two segments each on two 80 mm saddles, the stack is a 0.2 m cylinder, the rain cap a domed puck, the hood one prism up to the 0.1 m flap.",
          "Der Schalldämpfer behält seine Klöpperböden mit je zwei Segmenten auf zwei 80-mm-Sätteln, das Abgasrohr ist ein Zylinder von 0,2 m, die Regenkappe eine gewölbte Scheibe, die Haube ein Prisma bis zur 0,1-m-Klappe."),
        T("Thermal: as at 1:200 on this scale's own solids; emitters are 77 mm patches.",
          "Thermik: wie bei 1:200 auf den eigenen Körpern dieses Maßstabs; Emitter sind 77-mm-Flächen."),
        T("Structure: 80 mm rails and sills, three cross members, posts, top rails, eight castings as joints, skid rails, supports, two masses; silencer, saddles, stack, cap, hood side and top plates of 77 mm, flap and hinge.",
          "Tragwerk: 80-mm-Träger und Schwellen, drei Querträger, Pfosten, obere Träger, acht Beschläge als Gelenke, Grundrahmenträger, Auflager, zwei Massen; Schalldämpfer, Sättel, Rohr, Kappe, Haubenseiten- und Deckblech von 77 mm, Klappe und Scharnier."),
    ]
    return authored(500, body, thermal, structure, decisions)


def volumes():
    """📦 All four scales."""
    return {"50": s50(), "100": s100(), "200": s200(), "500": s500()}
