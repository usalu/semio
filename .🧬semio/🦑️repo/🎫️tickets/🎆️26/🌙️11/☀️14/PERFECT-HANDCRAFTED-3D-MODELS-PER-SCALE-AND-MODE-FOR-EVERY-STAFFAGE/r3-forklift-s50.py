"""🔩 The 1:50 forklift, part by part (smallest feature 7.5 mm): every assembly of the EFG 425 with its real sections, stated by hand in world rest coordinates."""
import math

PAINT, STEEL, IRON, RUBBER, BAT, DRIVE, HYD, WOOD, GOODS, SEAT = "paint", "steel", "cast-iron", "rubber", "battery-case", "drive-electronics", "hydraulic", "wood", "goods", "interior"


def text(en, de):
    return {"en": en, "de": de}


def tyre_profile(K, radius, width, rim, shoulder):
    h = width / 2
    return [(-h, rim), (-h, radius - shoulder), (-h + shoulder * 0.35, radius - shoulder * 0.35), (-h + shoulder, radius), (h - shoulder, radius), (h - shoulder * 0.35, radius - shoulder * 0.35), (h, radius - shoulder), (h, rim)]


def wheel_set(S, K, tag, bone, radius, width, rim_r, z, nuts, hub_r, views_far):
    """🛞 Tyre, dished steel rim, hub cap and wheel nuts on both sides of one axle (left far side drawn in front and plan only)."""
    for side, suffix in ((1, "l"), (-1, "r")):
        views = views_far if side == 1 else "efp"
        origin = (REST_X[bone], REST_Y[bone], side * z)
        h = width / 2
        S.rev(f"tyre-{tag}-{suffix}", bone, origin, (0, 0, side), tyre_profile(K, radius, width, rim_r + 0.012, 0.03) + [(-h, rim_r + 0.012)], RUBBER, 0.1, region="tyres", views=views,
              front=K.rounded_rect(side * z - h, side * z + h, REST_Y[bone] - radius, REST_Y[bone] + radius, 0.03) if side == 1 else K.rounded_rect(-z - h, -z + h, REST_Y[bone] - radius, REST_Y[bone] + radius, 0.03),
              plan=K.rounded_rect(REST_X[bone] - radius, REST_X[bone] + radius, side * z - h, side * z + h, 0.03) if side == 1 else K.rounded_rect(REST_X[bone] - radius, REST_X[bone] + radius, -z - h, -z + h, 0.03))
        S.rev(f"rim-{tag}-{suffix}", bone, origin, (0, 0, side), [(-h + 0.008, rim_r - 0.012), (-h + 0.008, rim_r + 0.016), (h - 0.008, rim_r + 0.016), (h - 0.008, rim_r - 0.012), (0.035, rim_r - 0.012), (0.035, hub_r + 0.01), (0.045, hub_r), (0.045, 0.03), (0.025, 0.03), (0.025, rim_r - 0.03), (-h + 0.02, rim_r - 0.012), (-h + 0.008, rim_r - 0.012)], PAINT, 0.6, views="e" if side == -1 else "")
        S.rev(f"hub-{tag}-{suffix}", bone, origin, (0, 0, side), [(0.03, 0), (0.03, hub_r * 0.75), (0.07, hub_r * 0.62), (0.08, hub_r * 0.35), (0.08, 0)], STEEL, 0.45, views="e" if side == -1 else "")
        for k in range(nuts):
            a = 2 * math.pi * (k + 0.5) / nuts
            cx, cy = REST_X[bone] + 0.62 * hub_r * 1.45 * math.cos(a), REST_Y[bone] + 0.62 * hub_r * 1.45 * math.sin(a)
            S.cyl(f"nut-{tag}-{suffix}-{k + 1}", bone, (cx, cy, side * (z + 0.04)), (cx, cy, side * (z + 0.058)), 0.011, STEEL, 0.35, views="e" if side == -1 else "")


REST_X, REST_Y = {}, {}


def build(S, K):
    for bone, (x, y, _) in K.REST.items():
        REST_X[bone], REST_Y[bone] = x, y
    R = K.rounded_rect
    xr, xf = K.REAR, K.FACE

    S.ext("floor-pan", "chassis", [(-1.40, 0.135), (-0.36, 0.135), (-0.36, 0.15), (-1.40, 0.15)], (-0.515, 0.515), PAINT, 0.5, region="chassis-paint", views="")
    side = [(-1.40, 0.135), (-0.30, 0.135)] + K.arc(0.0, K.RF, 0.338, 208, 152, 6) + [(-0.30, 0.66), (-0.36, 0.66), (-0.36, 0.935), (-1.40, 0.935)]
    S.ext("side", "chassis", side, (0.515, 0.575), PAINT, 0.55, region="chassis-paint", mirror=True)
    S.stroke("battery-door-seam", "chassis", "elevation", [(-1.33, 0.20), (-0.43, 0.20), (-0.43, 0.88), (-1.33, 0.88), (-1.33, 0.20)], "side", tone=0.35, material=PAINT)
    S.stroke("battery-door-handle", "chassis", "elevation", [(-0.62, 0.70), (-0.50, 0.70), (-0.50, 0.73), (-0.62, 0.73)], "side", closed=True, filled=True, tone=0.25, material=PAINT)
    S.box("battery-door", "chassis", (-1.33, -0.43), (0.20, 0.88), (-0.583, -0.575), PAINT, 0.55, region="chassis-paint", views="fp", radius=0.004)
    S.cyl("grab-handle", "chassis", (-0.073, 0.95, 0.535), (-0.164, 1.55, 0.535), 0.014, STEEL, 0.25, views="fp")
    S.box("door-handle-r", "chassis", (-0.62, -0.50), (0.70, 0.73), (-0.598, -0.583), PAINT, 0.25, region="chassis-paint", views="efp")
    S.box("step-l", "chassis", (-0.34, -0.06), (0.36, 0.38), (0.48, 0.60), STEEL, 0.4, region="chassis-paint", views="efp")
    S.box("step-bracket-l", "chassis", (-0.30, -0.10), (0.38, 0.66), (0.555, 0.575), PAINT, 0.5, region="chassis-paint", views="e")
    S.ext("kick-panel", "chassis", [(-0.376, 0.66), (-0.36, 0.66), (-0.36, 0.935), (-0.376, 0.935)], (-0.515, 0.515), PAINT, 0.5, region="chassis-paint", views="f")
    S.box("bulkhead", "chassis", (-1.40, -1.385), (0.15, 0.935), (-0.515, 0.515), PAINT, 0.5, region="chassis-paint", views="")
    S.box("hood", "chassis", (-1.40, -0.355), (0.935, 0.99), (-0.575, 0.575), PAINT, 0.55, region="chassis-paint", radius=0.022)
    S.stroke("hood-seam", "chassis", "plan", [(-1.36, -0.53), (-0.40, -0.53), (-0.40, 0.53), (-1.36, 0.53), (-1.36, -0.53)], "hood", tone=0.35, material=PAINT)
    S.stroke("hood-grip", "chassis", "front", [(-0.15, 0.95), (0.15, 0.95)], "hood", tone=0.25, material=PAINT)
    for s, sign in (("l", 1), ("r", -1)):
        fender = K.arc(0.0, K.RF, 0.338, 30, 152, 14) + list(reversed(K.arc(0.0, K.RF, 0.33, 30, 152, 14)))
        S.ext(f"fender-{s}", "chassis", fender, tuple(sorted((sign * 0.375, sign * 0.60))), PAINT, 0.5, region="chassis-paint", views="efp" if s == "r" else "fp")
    S.box("fender-deck", "chassis", (-0.36, 0.215), (0.645, 0.665), (0.375, 0.60), PAINT, 0.5, region="chassis-paint", mirror=True, radius=0.008)
    S.box("frame-front", "chassis", (-0.376, 0.06), (0.15, 0.645), (0.345, 0.37), PAINT, 0.45, region="chassis-paint", mirror=True, views="ef")
    S.box("floor-plate", "chassis", (-0.36, 0.02), (0.645, 0.665), (-0.375, 0.375), RUBBER, 0.2, region="chassis-paint", views="fp")
    S.stroke("floor-ribs", "chassis", "plan", [(-0.33, -0.30), (-0.03, -0.30), (-0.03, -0.20), (-0.33, -0.20), (-0.33, -0.10), (-0.03, -0.10), (-0.03, 0.0), (-0.33, 0.0), (-0.33, 0.10), (-0.03, 0.10), (-0.03, 0.20), (-0.33, 0.20), (-0.33, 0.30), (-0.03, 0.30)], "floor-plate", role="texture", tone=0.3, material=RUBBER)
    S.box("pedal-drive", "chassis", (-0.22, -0.12), (0.665, 0.70), (-0.20, -0.12), RUBBER, 0.15, views="p")
    S.box("pedal-brake", "chassis", (-0.22, -0.12), (0.665, 0.70), (-0.06, 0.04), RUBBER, 0.15, views="p")
    S.lofty("cowl", "chassis", [(0.665, -0.075, 0, 0.065, 0.34, 5), (0.80, -0.105, 0, 0.085, 0.35, 5), (0.98, -0.155, 0, 0.09, 0.36, 5), (1.07, -0.175, 0, 0.08, 0.35, 5), (1.095, -0.18, 0, 0.065, 0.33, 4)], DRIVE, 0.25, region="controller")
    S.stroke("display", "chassis", "plan", [(-0.20, -0.10), (-0.13, -0.10), (-0.13, 0.10), (-0.20, 0.10), (-0.20, -0.10)], "cowl", tone=0.15, material=DRIVE)
    S.cyl("column", "chassis", (-0.19, 1.07, 0), (-0.315, 1.255, 0), 0.032, SEAT, 0.2)
    axis = (-0.555, 0.832, 0)
    centre = (-0.33, 1.278, 0)
    S.rev("steering-wheel", "chassis", centre, axis, [(0.014 * math.cos(2 * math.pi * k / 10), 0.150 + 0.014 * math.sin(2 * math.pi * k / 10)) for k in range(11)], SEAT, 0.2,
          front=K.rounded_rect(-0.164, 0.164, 1.278 - 0.164 * 0.555 - 0.012, 1.278 + 0.164 * 0.555 + 0.012, 0.05), plan=[(centre[0] + 0.164 * 0.832 * math.cos(t) - 0.012 * 0.555, 0.164 * math.sin(t)) for t in [2 * math.pi * k / 20 for k in range(20)]])
    S.cyl("wheel-hub", "chassis", (-0.315, 1.255, 0), (-0.337, 1.288, 0), 0.035, SEAT, 0.2, views="e")
    for k, a in enumerate((90, 210, 330)):
        e1 = (0.832, 0.555)
        ux, uy, uz = math.cos(math.radians(a)) * e1[0], math.cos(math.radians(a)) * e1[1], math.sin(math.radians(a))
        S.cyl(f"spoke-{k + 1}", "chassis", (centre[0] + 0.03 * ux, centre[1] + 0.03 * uy, 0.03 * uz), (centre[0] + 0.142 * ux, centre[1] + 0.142 * uy, 0.142 * uz), 0.009, SEAT, 0.2, views="")

    S.box("seat-base", "chassis", (-0.98, -0.62), (0.99, 1.045), (-0.20, 0.20), STEEL, 0.25, radius=0.01)
    S.loft("seat-cushion", "chassis", [(-1.02, 1.095, 0, 0.235, 0.035, 3), (-0.96, 1.095, 0, 0.25, 0.05, 3.5), (-0.70, 1.095, 0, 0.25, 0.055, 3.5), (-0.585, 1.095, 0, 0.24, 0.045, 3), (-0.56, 1.095, 0, 0.215, 0.03, 2.5)], SEAT, 0.22)
    S.lofty("seat-back", "chassis", [(1.11, -1.035, 0, 0.045, 0.22, 3), (1.20, -1.05, 0, 0.065, 0.245, 3.5), (1.45, -1.085, 0, 0.065, 0.25, 3.5), (1.66, -1.115, 0, 0.055, 0.225, 3), (1.70, -1.12, 0, 0.04, 0.19, 2.5)], SEAT, 0.22)
    S.box("armrest", "chassis", (-0.93, -0.60), (1.22, 1.29), (-0.345, -0.255), SEAT, 0.2, radius=0.02)
    S.box("armrest-post", "chassis", (-0.90, -0.86), (1.045, 1.22), (-0.32, -0.28), STEEL, 0.3, views="e")
    S.box("mini-levers", "chassis", (-0.67, -0.59), (1.29, 1.345), (-0.335, -0.265), SEAT, 0.15, views="ef", radius=0.006)

    S.ext("post-front", "chassis", [(-0.06, 0.665), (0.0, 0.665), (-0.23, 2.18), (-0.29, 2.18)], (0.475, 0.525), PAINT, 0.25, region="guard", mirror=True)
    S.box("post-rear", "chassis", (-1.49, -1.43), (1.045, 2.18), (0.475, 0.525), PAINT, 0.25, region="guard", mirror=True, radius=0.008)
    S.box("roof-rail", "chassis", (-1.52, -0.23), (2.18, 2.24), (0.47, 0.53), PAINT, 0.3, region="guard", mirror=True, radius=0.008)
    S.box("roof-front", "chassis", (-0.29, -0.23), (2.18, 2.24), (-0.47, 0.47), PAINT, 0.3, region="guard", views="fp", radius=0.008)
    S.box("roof-rear", "chassis", (-1.52, -1.46), (2.18, 2.24), (-0.47, 0.47), PAINT, 0.3, region="guard", views="fp", radius=0.008)
    for k, z in enumerate((-0.36, -0.24, -0.12, 0.0, 0.12, 0.24, 0.36)):
        S.box(f"roof-slat-{k + 1}", "chassis", (-1.46, -0.29), (2.185, 2.235), (z - 0.005, z + 0.005), PAINT, 0.3, region="guard", views="p")
    S.box("lamp-front", "chassis", (-0.22, -0.14), (1.70, 1.77), (0.525, 0.575), STEEL, 0.6, mirror=True, radius=0.01)
    S.rev("beacon-base", "chassis", (-1.49, 2.24, 0), (0, 1, 0), [(0, 0), (0, 0.05), (0.02, 0.05), (0.02, 0)], STEEL, 0.3,
          front=[(-0.05, 2.24), (0.05, 2.24), (0.05, 2.26), (-0.05, 2.26)], plan=[(-1.49 + 0.05 * math.cos(t), 0.05 * math.sin(t)) for t in [2 * math.pi * k / 20 for k in range(20)]])
    S.rev("beacon", "chassis", (-1.49, 2.24, 0), (0, 1, 0), [(0.02, 0), (0.02, 0.042), (0.05, 0.040), (0.068, 0.026), (0.085, 0.0)], SEAT, 0.85,
          front=[(-0.042, 2.26), (0.042, 2.26), (0.040, 2.29), (0.026, 2.308), (0, 2.325), (-0.026, 2.308), (-0.040, 2.29)], plan=[(-1.49 + 0.042 * math.cos(t), 0.042 * math.sin(t)) for t in [2 * math.pi * k / 20 for k in range(20)]])

    S.box("motor-mount", "axle-f", (-0.10, 0.10), (0.20, 0.40), (-0.04, 0.04), DRIVE, 0.35, region="drive-motor", radius=0.01, target="thermal")
    S.box("terminal-box", "axle-f", (-0.06, 0.06), (0.41, 0.47), (0.10, 0.22), DRIVE, 0.3, region="drive-motor", mirror=True, radius=0.008, target="thermal")
    S.cyl("motor", "axle-f", (0.0, K.RF, 0.04), (0.0, K.RF, 0.30), 0.13, DRIVE, 0.3, region="drive-motor", mirror=True, target="thermal")
    S.cyl("gear-brake", "axle-f", (0.0, K.RF, 0.30), (0.0, K.RF, 0.385), 0.105, IRON, 0.35, region="brakes", mirror=True, target="thermal")
    S.box("pivot-bracket", "chassis", (0.05, 0.20), (0.25, 0.35), (0.355, 0.372), STEEL, 0.35, mirror=True, views="")
    S.lofty("tilt-lug-base", "chassis", [(0.665, -0.22, 0.42, 0.045, 0.05, 4), (0.79, -0.22, 0.42, 0.035, 0.04, 4)], STEEL, 0.35, mirror=True, views="ep")

    S.box("steer-axle", "axle-r", (-1.80, -1.64), (0.17, 0.29), (-0.36, 0.36), STEEL, 0.35, radius=0.02, views="f")
    S.cyl("steer-cylinder", "axle-r", (-1.84, 0.235, -0.25), (-1.84, 0.235, 0.25), 0.025, HYD, 0.35, region="hydraulics", views="")
    S.cyl("steer-rod", "axle-r", (-1.84, 0.235, -0.33), (-1.84, 0.235, 0.33), 0.012, HYD, 0.7, region="hydraulics", views="")
    S.cyl("kingpin", "axle-r", (-1.72, 0.14, 0.37), (-1.72, 0.32, 0.37), 0.035, STEEL, 0.35, mirror=True, views="")
    S.cyl("stub-axle", "axle-r", (-1.72, K.RR, 0.33), (-1.72, K.RR, 0.40), 0.045, STEEL, 0.35, mirror=True, views="")

    S.loft("cw-block", "counterweight", [(xr, 0.7725, 0, 0.50, 0.255, 6), (xr + 0.012, 0.7725, 0, 0.545, 0.268, 6), (xr + 0.045, 0.7725, 0, 0.572, 0.2725, 7), (xr + 0.09, 0.7725, 0, 0.575, 0.2725, 8), (-1.40, 0.7725, 0, 0.575, 0.2725, 8)], IRON, 0.3, region="counterweight")
    apron = [(xr + 0.01, 0.20), (-1.985, 0.20)] + K.arc(-K.WB, K.RR, 0.27, 168.9, 112, 6) + [(-1.82, 0.52), (xr + 0.01, 0.52)]
    S.ext("cw-apron", "counterweight", apron, (-0.49, 0.49), IRON, 0.3, region="counterweight")
    front = [(-1.40, 0.30), (-1.40, 0.52), (-1.62, 0.52)] + K.arc(-K.WB, K.RR, 0.27, 68, 22, 5)
    S.ext("cw-foot", "counterweight", front, (0.36, 0.575), IRON, 0.3, region="counterweight", mirror=True, views="e")
    housing = K.arc(-K.WB, K.RR, 0.27, 20, 160, 10)
    S.ext("cw-housing", "counterweight", housing, (0.33, 0.36), IRON, 0.3, region="counterweight", mirror=True, views="e")
    S.box("coupling", "counterweight", (xr - 0.012, xr + 0.03), (0.36, 0.41), (-0.06, 0.06), IRON, 0.25, views="efp", radius=0.008)
    S.cyl("coupling-pin", "counterweight", (xr + 0.0, 0.41, 0), (xr + 0.0, 0.53, 0), 0.016, STEEL, 0.4)
    S.box("lamp-rear", "counterweight", (xr - 0.008, xr + 0.02), (0.82, 0.92), (0.40, 0.52), SEAT, 0.7, mirror=True, radius=0.008)
    S.box("vent-grille", "counterweight", (xr - 0.006, xr + 0.02), (0.60, 0.72), (-0.15, 0.15), IRON, 0.2, views="ef", radius=0.006)
    for k, y in enumerate((0.62, 0.645, 0.67, 0.695)):
        S.stroke(f"vent-slot-{k + 1}", "counterweight", "front", [(-0.14, y), (0.14, y)], "vent-grille", role="texture", tone=0.1, material=IRON)

    S.box("battery", "battery", (-1.071, -0.36), (0.15, 0.934), (-0.514, 0.514), BAT, 0.4, region="battery-pack", radius=0.01, target="thermal")
    S.box("vent-outlet", "counterweight", (xr - 0.014, xr - 0.006), (0.60, 0.72), (-0.15, 0.15), IRON, 0.2, region="counterweight", target="thermal", flags={"tags": ["motor-vent"]})
    S.box("battery-tray", "battery", (-1.071, -0.36), (0.15, 0.17), (-0.514, 0.514), BAT, 0.4, target="structure", member="battery", part="member")

    wheel_set(S, K, "f", "wheel-f", K.RF, K.TF, 0.127, K.ZF, 6, 0.06, "fp")
    wheel_set(S, K, "r", "wheel-r", K.RR, K.TR, 0.1016, K.ZR, 5, 0.05, "fp")
    for k in range(18):
        a = 2 * math.pi * k / 18
        S.stroke(f"tread-f-{k + 1}", "wheel-f", "elevation", [(0.262 * math.cos(a), K.RF + 0.262 * math.sin(a)), (K.RF * math.cos(a), K.RF + K.RF * math.sin(a))], "tyre-f-r", role="texture", tone=0.35, material=RUBBER)
    for k in range(16):
        a = 2 * math.pi * k / 16
        S.stroke(f"tread-r-{k + 1}", "wheel-r", "elevation", [(-K.WB + 0.200 * math.cos(a), K.RR + 0.200 * math.sin(a)), (-K.WB + K.RR * math.cos(a), K.RR + K.RR * math.sin(a))], "tyre-r-r", role="texture", tone=0.35, material=RUBBER)

    b0 = K.REST["tilt-cyl"]
    d = (math.cos(math.radians(b0[2])), math.sin(math.radians(b0[2])))
    bx, by = K.TILT_BASE
    z = K.TILT_Z
    S.cyl("tilt-eye", "tilt-cyl", (bx, by, z - 0.03), (bx, by, z + 0.03), 0.032, HYD, 0.35, region="hydraulics", mirror=True, views="e")
    S.rev("tilt-barrel", "tilt-cyl", (bx, by, z), (d[0], d[1], 0), [(0.025, 0), (0.025, 0.038), (0.035, 0.045), (0.25, 0.045), (0.255, 0.036), (0.27, 0.036), (0.27, 0)], HYD, 0.35, region="hydraulics", mirror=True)
    S.cyl("tilt-port", "tilt-cyl", (bx + 0.06 * d[0], by + 0.06 * d[1], z), (bx + 0.06 * d[0] - 0.06 * d[1], by + 0.06 * d[1] + 0.06 * d[0], z), 0.012, HYD, 0.35, region="hydraulics", mirror=True, views="e")
    rx0 = K.REST["tilt-rod"]
    S.cyl("tilt-rod", "tilt-rod", (rx0[0], rx0[1], z), (rx0[0] + (K.TILT_ROD - 0.025) * d[0], rx0[1] + (K.TILT_ROD - 0.025) * d[1], z), 0.0225, HYD, 0.8, region="hydraulics", mirror=True)
    S.cyl("tilt-rod-eye", "tilt-rod", (K.TILT_PIN[0], K.TILT_PIN[1], z - 0.025), (K.TILT_PIN[0], K.TILT_PIN[1], z + 0.025), 0.03, HYD, 0.6, region="hydraulics", mirror=True, views="e")

    mo = "mast-outer"
    for s, sign in (("l", 1), ("r", -1)):
        c = [(0.12, 0.282), (0.14, 0.282), (0.14, 0.336), (0.225, 0.336), (0.225, 0.282), (0.245, 0.282), (0.245, 0.352), (0.12, 0.352)]
        S.upright(f"outer-rail-{s}", mo, [(x, sign * z) for x, z in (c if sign > 0 else reversed(c))], (0.12, K.MAST_TOP_OUTER), STEEL, 0.35, region="mast", views="efp" if s == "r" else "fp")
    S.box("outer-tie-top", mo, (0.10, 0.13), (2.09, K.MAST_TOP_OUTER), (-0.30, 0.30), STEEL, 0.35, region="mast", views="fp")
    S.box("outer-tie-low", mo, (0.115, 0.13), (0.14, 0.26), (-0.30, 0.30), STEEL, 0.35, region="mast", views="")
    S.cyl("pivot-pin", mo, (K.PIVOT[0], K.PIVOT[1], 0.30), (K.PIVOT[0], K.PIVOT[1], 0.375), 0.03, STEEL, 0.3, region="mast", mirror=True, views="")
    S.box("tilt-lug", mo, (0.15, 0.21), (0.90, 1.00), (0.352, 0.445), STEEL, 0.35, region="mast", mirror=True, radius=0.01, views="efp")
    S.cyl("lift-cyl", mo, (0.075, 0.20, 0.317), (0.075, 1.95, 0.317), 0.035, HYD, 0.4, region="mast", mirror=True, views="ef")
    S.cyl("lift-cyl-head", mo, (0.075, 1.93, 0.317), (0.075, 1.98, 0.317), 0.041, HYD, 0.35, region="mast", mirror=True, views="")
    mi = "mast-inner"
    for s, sign in (("l", 1), ("r", -1)):
        c = [(0.135, 0.218), (0.153, 0.218), (0.153, 0.264), (0.237, 0.264), (0.237, 0.218), (0.255, 0.218), (0.255, 0.282), (0.135, 0.282)]
        S.upright(f"inner-rail-{s}", mi, [(x, sign * z) for x, z in (c if sign > 0 else reversed(c))], (0.16, K.MAST_TOP_INNER - 0.055), STEEL, 0.45, region="mast", views="efp" if s == "r" else "fp")
    S.box("inner-head", mi, (0.05, 0.255), (K.MAST_TOP_INNER - 0.055, K.MAST_TOP_INNER), (-0.282, 0.282), STEEL, 0.45, region="mast", radius=0.01)
    S.box("inner-tie-low", mi, (0.135, 0.16), (0.16, 0.24), (-0.218, 0.218), STEEL, 0.45, region="mast", views="")
    S.cyl("lift-rod", mi, (0.075, 0.40, 0.317), (0.075, K.MAST_TOP_INNER - 0.055, 0.317), 0.0225, HYD, 0.8, region="mast", mirror=True, views="e")
    S.cyl("sheave", mi, (0.23, 2.10, 0.075), (0.23, 2.10, 0.125), 0.055, STEEL, 0.4, region="mast", mirror=True, views="e")
    S.box("chain", mi, (0.272, 0.288), (0.60, 2.10), (0.08, 0.12), STEEL, 0.25, region="mast", mirror=True, views="ef", flags={"slender": True})
    S.cyl("inner-roller", mi, (0.1825, 0.30, 0.282), (0.1825, 0.30, 0.31), 0.04, STEEL, 0.4, region="mast", mirror=True, views="")

    ca = "carriage"
    S.box("carriage-top", ca, (0.325, 0.385), (0.38, 0.497), (-0.56, 0.56), STEEL, 0.5, region="fork-gear", radius=0.006)
    S.box("carriage-low", ca, (0.335, 0.385), (0.09, 0.16), (-0.56, 0.56), STEEL, 0.5, region="fork-gear", radius=0.006)
    S.box("carriage-plate", ca, (0.16, 0.385), (0.09, 0.52), (0.18, 0.20), STEEL, 0.45, region="fork-gear", mirror=True, views="ep")
    for k, y in enumerate((0.15, 0.45)):
        S.cyl(f"carriage-roller-{k + 1}", ca, (0.195, y, 0.20), (0.195, y, 0.26), 0.04, STEEL, 0.4, region="fork-gear", mirror=True, views="")
    S.box("chain-anchor", ca, (0.27, 0.325), (0.50, 0.60), (0.07, 0.13), STEEL, 0.4, region="fork-gear", mirror=True, views="e")
    S.box("backrest-top", ca, (0.355, 0.385), (1.26, 1.30), (-0.56, 0.56), STEEL, 0.4, region="fork-gear", radius=0.006)
    S.box("backrest-mid", ca, (0.355, 0.385), (0.86, 0.89), (-0.56, 0.56), STEEL, 0.4, region="fork-gear", views="fp")
    S.box("backrest-post", ca, (0.345, 0.385), (0.497, 1.26), (0.52, 0.56), STEEL, 0.4, region="fork-gear", mirror=True, radius=0.006)
    for k, zc in enumerate((-0.40, -0.24, -0.08, 0.08, 0.24, 0.40)):
        S.box(f"backrest-bar-{k + 1}", ca, (0.36, 0.38), (0.497, 1.26), (zc - 0.01, zc + 0.01), STEEL, 0.4, region="fork-gear", views="f")

    fo = "forks"
    t0, t1, top = 0.02, K.FORK_TOP, xf + K.FORK_L
    tine = [(0.385 - 0.0, t0 + 0.04)] + K.arc(0.385 + 0.04, t0 + 0.04, 0.04, 180, 270, 4) + [(top - 0.16, t0), (top, t0 + 0.002), (top, t0 + 0.012), (top - 0.16, t1), (xf + 0.015, t1)] + K.arc(xf + 0.015, t1 + 0.015, 0.015, 270, 180, 3) + [(xf, 0.497), (0.425, 0.522), (0.318, 0.522), (0.318, 0.47), (0.33, 0.47), (0.33, 0.505), (0.385, 0.505), (0.385, 0.14), (0.33, 0.14), (0.33, 0.08), (0.385, 0.08)]
    for s, sign in (("l", 1), ("r", -1)):
        za, zb = sorted((sign * (K.FORK_Z - K.FORK_W / 2), sign * (K.FORK_Z + K.FORK_W / 2)))
        plan = [(0.318, za), (top - 0.05, za), (top, za + 0.012), (top, zb - 0.012), (top - 0.05, zb), (0.318, zb)]
        S.ext(f"tine-{s}", fo, tine, (za, zb), STEEL, 0.3, region="fork-gear", views="efp" if s == "r" else "fp", plan=plan)
    S.box("fork-lock", fo, (0.385, 0.40), (0.522, 0.55), (K.FORK_Z - 0.02, K.FORK_Z + 0.02), STEEL, 0.25, mirror=True, views="")

    pa = "pallet"
    p0 = K.PALLET_X
    rows_x = [(p0, p0 + 0.145), (p0 + 0.5275, p0 + 0.6725), (p0 + 1.055, p0 + 1.2)]
    deck_z = [(-0.4, -0.255), (-0.21375, -0.11375), (-0.0725, 0.0725), (0.11375, 0.21375), (0.255, 0.4)]
    for k, (z0, z1) in enumerate(deck_z):
        S.box(f"deck-{k + 1}", pa, (p0, p0 + 1.2), (0.122, 0.144), (z0, z1), WOOD, 0.62, region="pallet", views="efp" if k == 0 else "fp")
    for k, (x0, x1) in enumerate(rows_x):
        S.box(f"stringer-{k + 1}", pa, (x0, x1), (0.100, 0.122), (-0.4, 0.4), WOOD, 0.6, region="pallet")
        for j, (z0, z1) in enumerate([(-0.4, -0.3), (-0.0725, 0.0725), (0.3, 0.4)]):
            S.box(f"block-{k + 1}-{j + 1}", pa, (x0, x1), (0.022, 0.100), (z0, z1), WOOD, 0.55, region="pallet", views="ef" if j == 0 else "f")
    for j, (z0, z1) in enumerate([(-0.4, -0.3), (-0.0725, 0.0725), (0.3, 0.4)]):
        S.box(f"bottom-{j + 1}", pa, (p0, p0 + 1.2), (0.0, 0.022), (z0, z1), WOOD, 0.6, region="pallet", views="ef" if j == 0 else "f")
    S.box("goods", pa, (p0 + 0.01, p0 + 1.19), (0.144, 1.044), (-0.39, 0.39), GOODS, 0.78, region="cargo", radius=0.02)
    for k, y in enumerate((0.32, 0.50, 0.68, 0.86)):
        S.stroke(f"course-e-{k + 1}", pa, "elevation", [(p0 + 0.02, y), (p0 + 1.18, y)], "goods", role="texture", tone=0.55, material=GOODS)
        S.stroke(f"course-f-{k + 1}", pa, "front", [(-0.38, y), (0.38, y)], "goods", role="texture", tone=0.55, material=GOODS)
    for k, x in enumerate((p0 + 0.40, p0 + 0.80)):
        S.box(f"strap-{k + 1}", pa, (x - 0.008, x + 0.008), (1.044, 1.047), (-0.393, 0.393), GOODS, 0.3, region="cargo", views="", flags={"texture": True})
        S.stroke(f"strap-e-{k + 1}", pa, "elevation", [(x, 0.144), (x, 1.044)], "goods", tone=0.3, material=GOODS)
        S.stroke(f"strap-p-{k + 1}", pa, "plan", [(x, -0.39), (x, 0.39)], "goods", tone=0.3, material=GOODS)
    structure(S, K)
    return S


def structure(S, K):
    """🦴 The load path: frame side plates, front frame members, floor pan and rear cross member, the guard frame, the counterweight casting, the drive axle tube through the motors, the steer axle, the tyres, tilt cylinders, both mast stages with their ties, carriage bars and side plates, fork arms and the pallet; pins and rollers at the joints, four contact pads, the battery, counterweight and frame masses."""
    st = dict(target="structure")
    S.box("frame-side", "chassis", (-1.40, -0.36), (0.15, 0.60), (0.515, 0.545), STEEL, 0.5, mirror=True, member="chassis", part="member", **st)
    S.box("frame-cross-rear", "chassis", (-1.40, -1.30), (0.15, 0.50), (-0.515, 0.515), STEEL, 0.5, member="chassis", part="member", **st)
    for id in ("frame-front", "floor-pan", "post-front", "post-rear", "roof-rail", "roof-front", "roof-rear"):
        S.ref(f"m-{id}", id, "chassis", "member")
    S.box("mass-chassis", "chassis", (-0.76, -0.64), (0.66, 0.78), (-0.06, 0.06), STEEL, 0.5, radius=0.059, member="chassis", part="mass", **st)
    S.ref("m-cw-block", "cw-block", "counterweight", "member")
    S.ref("m-cw-apron", "cw-apron", "counterweight", "member")
    S.box("mass-counterweight", "counterweight", (-1.86, -1.74), (0.66, 0.78), (-0.06, 0.06), IRON, 0.3, radius=0.059, member="counterweight", part="mass", **st)
    S.box("mass-battery", "battery", (-0.7755, -0.6555), (0.482, 0.602), (-0.06, 0.06), BAT, 0.4, radius=0.059, member="battery", part="mass", **st)
    S.cyl("drive-axle", "axle-f", (0.0, K.RF, -0.385), (0.0, K.RF, 0.385), 0.09, STEEL, 0.4, member="axle-f", part="member", **st)
    S.cyl("bearing-f", "axle-f", (0.0, K.RF, 0.385), (0.0, K.RF, 0.43), 0.06, STEEL, 0.4, mirror=True, member="wheel-f", part="joint", **st)
    S.ref("m-steer-axle", "steer-axle", "axle-r", "member")
    S.ref("j-kingpin", "kingpin", "axle-r", "joint")
    S.ref("j-stub-axle", "stub-axle", "wheel-r", "joint")
    for side in ("l", "r"):
        S.ref(f"m-tyre-f-{side}", f"tyre-f-{side}", "wheel-f", "member")
        S.ref(f"m-tyre-r-{side}", f"tyre-r-{side}", "wheel-r", "member")
    for anchor, bone, x, z, length, width in (("contact-f-l", "axle-f", 0.0, K.ZF, 0.15, 0.18), ("contact-f-r", "axle-f", 0.0, -K.ZF, 0.15, 0.18), ("contact-r-l", "axle-r", -K.WB, K.ZR, 0.12, 0.14), ("contact-r-r", "axle-r", -K.WB, -K.ZR, 0.12, 0.14)):
        S.box(f"pad-{anchor[8:]}", bone, (x - length / 2, x + length / 2), (0.0, 0.0075), (z - width / 2, z + width / 2), RUBBER, 0.2, member=bone, part="support", anchor=anchor, **st)
    S.ref("m-tilt-barrel", "tilt-barrel", "tilt-cyl", "member")
    S.ref("j-tilt-eye", "tilt-eye", "tilt-cyl", "joint")
    S.ref("m-tilt-rod", "tilt-rod", "tilt-rod", "member")
    S.ref("j-tilt-rod-eye", "tilt-rod-eye", "tilt-rod", "joint")
    for id in ("outer-rail-l", "outer-rail-r", "outer-tie-top", "outer-tie-low", "lift-cyl"):
        S.ref(f"m-{id}", id, "mast-outer", "member")
    S.ref("j-pivot-pin", "pivot-pin", "mast-outer", "joint")
    for id in ("inner-rail-l", "inner-rail-r", "inner-head", "lift-rod"):
        S.ref(f"m-{id}", id, "mast-inner", "member")
    S.ref("j-inner-roller", "inner-roller", "mast-inner", "joint")
    for id in ("carriage-top", "carriage-low", "carriage-plate", "backrest-post", "backrest-top"):
        S.ref(f"m-{id}", id, "carriage", "member")
    S.ref("j-carriage-roller-1", "carriage-roller-1", "carriage", "joint")
    S.ref("j-carriage-roller-2", "carriage-roller-2", "carriage", "joint")
    S.ref("m-tine-l", "tine-l", "forks", "member")
    S.ref("m-tine-r", "tine-r", "forks", "member")
    S.ref("j-fork-lock", "fork-lock", "forks", "joint")
    for k in (1, 3):
        S.ref(f"m-bottom-{k}", f"bottom-{k}", "pallet-x", "member")
    for k in (1, 2, 3):
        S.ref(f"m-block-{k}-2", f"block-{k}-2", "pallet-y", "member")
        S.ref(f"m-stringer-{k}", f"stringer-{k}", "pallet", "member")
    S.ref("m-bottom-2", "bottom-2", "pallet", "member")


DECISIONS = [
    text("1:50 shows every assembly of a Jungheinrich EFG 425 with a Duplex ZT 3100 mast at 7.5 mm: frame side plates with the front wheel arch, battery door, entry step, floor plate with pedals, battery hood, front fenders as curved sheets with their decks, cowl as a superelliptic loft with the display, steering column, inclined steering wheel ring with three spokes and spinner, seat with base, cushion and backrest lofts, armrest with mini levers; overhead guard with front posts leaning back over the mast, rear posts on the counterweight, roof frame with seven slats, work lamps and beacon; cast counterweight as a loft rounded in plan with the rear apron, wheel-housing feet, coupling and pin, rear lamps and vent grille; twin drive motors with gear and brake housings, steer axle with king pins and steering cylinder; tyres with shoulders, dished rims, hub caps and nuts; nested C-section mast rails, ties, pivot pins, tilt lugs, two lift cylinders with rods, inner head with sheaves, chains and rollers; ISO class 2 carriage with its bars, side plates, rollers and load backrest; forks 40 x 120 x 1150 mm with hooks and tapered tips; a Euro pallet board by board with goods and straps.",
         "1:50 zeigt jede Baugruppe eines Jungheinrich EFG 425 mit Duplex-ZT-3100-Mast bei 7,5 mm: Rahmenseitenbleche mit vorderem Radlauf, Batterietür, Einstiegsstufe, Bodenplatte mit Pedalen, Batteriehaube, vordere Kotflügel als gewölbte Bleche mit ihren Abdeckungen, Armaturenhaube als superelliptischer Loft mit Anzeige, Lenksäule, geneigter Lenkradkranz mit drei Speichen und Knauf, Sitz mit Unterbau, Sitzkissen- und Lehnenloft, Armlehne mit Minihebeln; Fahrerschutzdach mit nach hinten geneigten Frontpfosten über dem Mast, Heckpfosten auf dem Gegengewicht, Dachrahmen mit sieben Streben, Arbeitsscheinwerfern und Rundumleuchte; gegossenes Gegengewicht als im Grundriss gerundeter Loft mit Heckschürze, Radhausfüßen, Kupplung mit Bolzen, Rückleuchten und Lüftungsgitter; zwei Fahrmotoren mit Getriebe- und Bremsgehäusen, Lenkachse mit Achsschenkelbolzen und Lenkzylinder; Reifen mit Schultern, gekröpfte Felgen, Nabenkappen und Muttern; ineinander geschachtelte C-Profil-Mastschienen, Traversen, Lagerbolzen, Neigelaschen, zwei Hubzylinder mit Stangen, Innenmastkopf mit Kettenrollen, Ketten und Laufrollen; Gabelträger nach ISO Klasse 2 mit Leisten, Seitenblechen, Rollen und Lastschutzgitter; Gabeln 40 x 120 x 1150 mm mit Haken und verjüngten Spitzen; eine Europalette Brett für Brett mit Ladegut und Umreifung."),
]
