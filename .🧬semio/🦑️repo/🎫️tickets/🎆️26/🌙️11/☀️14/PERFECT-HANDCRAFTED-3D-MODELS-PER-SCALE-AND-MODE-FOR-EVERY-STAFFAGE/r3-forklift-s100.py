"""🔧 The 1:100 forklift (smallest feature 15 mm): the same machine authored anew for 15 mm, with merged pallet rows, solid mast bars and simplified wheels, seat and controls."""
import math

PAINT, STEEL, IRON, RUBBER, BAT, DRIVE, HYD, WOOD, GOODS, SEAT = "paint", "steel", "cast-iron", "rubber", "battery-case", "drive-electronics", "hydraulic", "wood", "goods", "interior"


def text(en, de):
    return {"en": en, "de": de}


def wheels(S, K, tag, bone, radius, width, rim_r, z):
    """🛞 Tyre with chamfered shoulders and one recessed wheel disc per side."""
    x0, y0, _ = K.REST[bone]
    h = width / 2
    for side, suffix in ((1, "l"), (-1, "r")):
        S.rev(f"tyre-{tag}-{suffix}", bone, (x0, y0, side * z), (0, 0, side), [(-h, rim_r), (-h, radius - 0.025), (-h + 0.025, radius), (h - 0.025, radius), (h, radius - 0.025), (h, rim_r), (-h, rim_r)], RUBBER, 0.1, region="tyres", views="efp" if side < 0 else "fp",
              front=K.rounded_rect(side * z - h, side * z + h, y0 - radius, y0 + radius, 0.025), plan=K.rounded_rect(x0 - radius, x0 + radius, side * z - h, side * z + h, 0.025))
        S.rev(f"rim-{tag}-{suffix}", bone, (x0, y0, side * z), (0, 0, side), [(-h + 0.01, 0), (-h + 0.01, rim_r + 0.005), (0.03, rim_r + 0.005), (0.03, rim_r * 0.5), (0.07, rim_r * 0.42), (0.07, 0)], PAINT, 0.6, views="e" if side < 0 else "")


def build(S, K):
    xr, xf = K.REAR, K.FACE
    side = [(-1.40, 0.135), (-0.30, 0.135)] + K.arc(0.0, K.RF, 0.338, 208, 152, 4) + [(-0.30, 0.66), (-0.36, 0.66), (-0.36, 0.935), (-1.40, 0.935)]
    S.ext("side", "chassis", side, (0.515, 0.575), PAINT, 0.55, region="chassis-paint", mirror=True)
    S.stroke("battery-door-seam", "chassis", "elevation", [(-1.33, 0.20), (-0.43, 0.20), (-0.43, 0.88), (-1.33, 0.88), (-1.33, 0.20)], "side", tone=0.35, material=PAINT)
    S.box("floor-pan", "chassis", (-1.40, -0.36), (0.135, 0.15), (-0.515, 0.515), PAINT, 0.5, region="chassis-paint", views="")
    S.box("step-l", "chassis", (-0.34, -0.06), (0.36, 0.38), (0.48, 0.60), STEEL, 0.4, region="chassis-paint", views="fp")
    S.box("kick-panel", "chassis", (-0.376, -0.36), (0.66, 0.935), (-0.515, 0.515), PAINT, 0.5, region="chassis-paint", views="f")
    S.box("bulkhead", "chassis", (-1.40, -1.385), (0.15, 0.935), (-0.515, 0.515), PAINT, 0.5, region="chassis-paint", views="")
    S.box("hood", "chassis", (-1.40, -0.355), (0.935, 0.99), (-0.575, 0.575), PAINT, 0.55, region="chassis-paint", radius=0.02)
    for s, sign in (("l", 1), ("r", -1)):
        fender = K.arc(0.0, K.RF, 0.346, 30, 152, 10) + list(reversed(K.arc(0.0, K.RF, 0.33, 30, 152, 10)))
        S.ext(f"fender-{s}", "chassis", fender, tuple(sorted((sign * 0.375, sign * 0.60))), PAINT, 0.5, region="chassis-paint", views="efp" if s == "r" else "fp")
    S.box("fender-deck", "chassis", (-0.36, 0.215), (0.645, 0.665), (0.375, 0.60), PAINT, 0.5, region="chassis-paint", mirror=True)
    S.box("frame-front", "chassis", (-0.376, 0.06), (0.15, 0.645), (0.345, 0.37), PAINT, 0.45, region="chassis-paint", mirror=True, views="ef")
    S.box("floor-plate", "chassis", (-0.36, 0.02), (0.645, 0.665), (-0.375, 0.375), RUBBER, 0.2, region="chassis-paint", views="fp")
    S.lofty("cowl", "chassis", [(0.665, -0.075, 0, 0.065, 0.34, 4), (0.98, -0.155, 0, 0.09, 0.36, 4), (1.095, -0.18, 0, 0.07, 0.34, 4)], DRIVE, 0.25, region="controller")
    S.cyl("column", "chassis", (-0.19, 1.07, 0), (-0.315, 1.255, 0), 0.032, SEAT, 0.2)
    centre = (-0.33, 1.278, 0)
    S.rev("steering-wheel", "chassis", centre, (-0.555, 0.832, 0), [(0.016 * math.cos(2 * math.pi * k / 6), 0.148 + 0.016 * math.sin(2 * math.pi * k / 6)) for k in range(7)], SEAT, 0.2,
          front=K.rounded_rect(-0.164, 0.164, 1.278 - 0.103, 1.278 + 0.103, 0.05), plan=[(centre[0] + 0.164 * 0.832 * math.cos(t), 0.164 * math.sin(t)) for t in [2 * math.pi * k / 16 for k in range(16)]])
    S.box("seat-base", "chassis", (-0.98, -0.62), (0.99, 1.045), (-0.20, 0.20), STEEL, 0.25)
    S.loft("seat-cushion", "chassis", [(-1.02, 1.095, 0, 0.24, 0.04, 3), (-0.80, 1.095, 0, 0.25, 0.055, 3.5), (-0.56, 1.095, 0, 0.22, 0.035, 3)], SEAT, 0.22)
    S.lofty("seat-back", "chassis", [(1.11, -1.035, 0, 0.05, 0.225, 3), (1.45, -1.085, 0, 0.065, 0.25, 3.5), (1.66, -1.115, 0, 0.055, 0.225, 3), (1.70, -1.12, 0, 0.04, 0.19, 2.5)], SEAT, 0.22)
    S.box("armrest", "chassis", (-0.93, -0.60), (1.22, 1.29), (-0.345, -0.255), SEAT, 0.2, radius=0.02)
    S.box("mini-levers", "chassis", (-0.67, -0.59), (1.29, 1.345), (-0.335, -0.265), SEAT, 0.15, views="ef")
    S.box("armrest-post", "chassis", (-0.90, -0.86), (1.045, 1.22), (-0.32, -0.28), STEEL, 0.3, views="e")
    S.ext("post-front", "chassis", [(-0.06, 0.665), (0.0, 0.665), (-0.23, 2.18), (-0.29, 2.18)], (0.475, 0.525), PAINT, 0.25, region="guard", mirror=True)
    S.box("post-rear", "chassis", (-1.49, -1.43), (1.045, 2.18), (0.475, 0.525), PAINT, 0.25, region="guard", mirror=True)
    S.box("roof-rail", "chassis", (-1.52, -0.23), (2.18, 2.24), (0.47, 0.53), PAINT, 0.3, region="guard", mirror=True)
    S.box("roof-front", "chassis", (-0.29, -0.23), (2.18, 2.24), (-0.47, 0.47), PAINT, 0.3, region="guard", views="fp")
    S.box("roof-rear", "chassis", (-1.52, -1.46), (2.18, 2.24), (-0.47, 0.47), PAINT, 0.3, region="guard", views="fp")
    for k, z in enumerate((-0.32, -0.16, 0.0, 0.16, 0.32)):
        S.box(f"roof-slat-{k + 1}", "chassis", (-1.46, -0.29), (2.185, 2.235), (z - 0.0075, z + 0.0075), PAINT, 0.3, region="guard", views="p")
    S.rev("beacon", "chassis", (-1.49, 2.24, 0), (0, 1, 0), [(0, 0), (0, 0.05), (0.02, 0.05), (0.02, 0.042), (0.05, 0.040), (0.068, 0.026), (0.085, 0.0)], SEAT, 0.85,
          front=[(-0.05, 2.24), (0.05, 2.24), (0.05, 2.26), (0.042, 2.26), (0.040, 2.29), (0.026, 2.308), (0, 2.325), (-0.026, 2.308), (-0.040, 2.29), (-0.042, 2.26), (-0.05, 2.26)], plan=[(-1.49 + 0.05 * math.cos(t), 0.05 * math.sin(t)) for t in [2 * math.pi * k / 16 for k in range(16)]])
    S.cyl("grab-handle", "chassis", (-0.073, 0.95, 0.535), (-0.164, 1.55, 0.535), 0.015, STEEL, 0.25, views="fp")
    S.box("lamp-front", "chassis", (-0.22, -0.14), (1.70, 1.77), (0.525, 0.575), STEEL, 0.6, mirror=True)
    S.lofty("tilt-lug-base", "chassis", [(0.665, -0.22, 0.42, 0.045, 0.05, 4), (0.79, -0.22, 0.42, 0.035, 0.04, 4)], STEEL, 0.35, mirror=True, views="ep")

    S.cyl("motor", "axle-f", (0.0, K.RF, -0.30), (0.0, K.RF, 0.30), 0.13, DRIVE, 0.3, region="drive-motor", target="thermal")
    S.box("terminal-box", "axle-f", (-0.06, 0.06), (0.41, 0.47), (0.10, 0.22), DRIVE, 0.3, region="drive-motor", mirror=True, target="thermal")
    S.cyl("gear-brake", "axle-f", (0.0, K.RF, 0.30), (0.0, K.RF, 0.385), 0.105, IRON, 0.35, region="brakes", mirror=True, target="thermal")
    S.box("steer-axle", "axle-r", (-1.80, -1.64), (0.17, 0.29), (-0.36, 0.36), STEEL, 0.35, views="f")
    S.cyl("steer-cylinder", "axle-r", (-1.84, 0.235, -0.33), (-1.84, 0.235, 0.33), 0.025, HYD, 0.35, region="hydraulics", views="")

    S.loft("cw-block", "counterweight", [(xr, 0.7725, 0, 0.51, 0.258, 6), (xr + 0.03, 0.7725, 0, 0.565, 0.272, 7), (xr + 0.09, 0.7725, 0, 0.575, 0.2725, 8), (-1.40, 0.7725, 0, 0.575, 0.2725, 8)], IRON, 0.3, region="counterweight")
    apron = [(xr + 0.01, 0.20), (-1.985, 0.20)] + K.arc(-K.WB, K.RR, 0.27, 168.9, 112, 4) + [(-1.82, 0.52), (xr + 0.01, 0.52)]
    S.ext("cw-apron", "counterweight", apron, (-0.49, 0.49), IRON, 0.3, region="counterweight")
    S.ext("cw-foot", "counterweight", [(-1.40, 0.30), (-1.40, 0.52), (-1.62, 0.52)] + K.arc(-K.WB, K.RR, 0.27, 68, 22, 4), (0.36, 0.575), IRON, 0.3, region="counterweight", mirror=True, views="e")
    S.ext("cw-housing", "counterweight", K.arc(-K.WB, K.RR, 0.27, 20, 160, 6), (0.33, 0.36), IRON, 0.3, region="counterweight", mirror=True, views="e")
    S.box("coupling", "counterweight", (xr - 0.015, xr + 0.03), (0.36, 0.41), (-0.06, 0.06), IRON, 0.25)
    S.box("vent-outlet", "counterweight", (xr - 0.031, xr - 0.016), (0.60, 0.72), (-0.15, 0.15), IRON, 0.2, region="counterweight", target="thermal", flags={"tags": ["motor-vent"]})
    S.box("vent-grille", "counterweight", (xr - 0.016, xr + 0.02), (0.60, 0.72), (-0.15, 0.15), IRON, 0.2, views="ef")
    S.box("lamp-rear", "counterweight", (xr - 0.015, xr + 0.02), (0.82, 0.92), (0.40, 0.52), SEAT, 0.7, mirror=True)
    S.box("battery", "battery", (-1.071, -0.36), (0.15, 0.934), (-0.514, 0.514), BAT, 0.4, region="battery-pack", target="thermal")

    wheels(S, K, "f", "wheel-f", K.RF, K.TF, 0.14, K.ZF)
    wheels(S, K, "r", "wheel-r", K.RR, K.TR, 0.115, K.ZR)

    b0 = K.REST["tilt-cyl"]
    d = (math.cos(math.radians(b0[2])), math.sin(math.radians(b0[2])))
    bx, by = K.TILT_BASE
    z = K.TILT_Z
    S.rev("tilt-barrel", "tilt-cyl", (bx, by, z), (d[0], d[1], 0), [(-0.03, 0), (-0.03, 0.045), (0.27, 0.045), (0.27, 0)], HYD, 0.35, region="hydraulics", mirror=True)
    rx0 = K.REST["tilt-rod"]
    S.cyl("tilt-rod", "tilt-rod", (rx0[0], rx0[1], z), (rx0[0] + (K.TILT_ROD + 0.02) * d[0], rx0[1] + (K.TILT_ROD + 0.02) * d[1], z), 0.0225, HYD, 0.8, region="hydraulics", mirror=True)

    mo, mi, ca, fo, pa = "mast-outer", "mast-inner", "carriage", "forks", "pallet"
    for s, sign in (("l", 1), ("r", -1)):
        c = [(0.12, 0.282), (0.14, 0.282), (0.14, 0.336), (0.225, 0.336), (0.225, 0.282), (0.245, 0.282), (0.245, 0.352), (0.12, 0.352)]
        S.upright(f"outer-rail-{s}", mo, [(x, sign * z) for x, z in (c if sign > 0 else reversed(c))], (0.12, K.MAST_TOP_OUTER), STEEL, 0.35, region="mast", views="efp" if s == "r" else "fp")
    S.box("outer-tie-top", mo, (0.10, 0.13), (2.09, K.MAST_TOP_OUTER), (-0.30, 0.30), STEEL, 0.35, region="mast", views="fp")
    S.box("tilt-lug", mo, (0.15, 0.21), (0.90, 1.00), (0.352, 0.445), STEEL, 0.35, region="mast", mirror=True)
    S.cyl("lift-cyl", mo, (0.075, 0.20, 0.317), (0.075, 1.93, 0.317), 0.035, HYD, 0.4, region="mast", mirror=True, views="ef")
    S.cyl("lift-cyl-head", mo, (0.075, 1.93, 0.317), (0.075, 1.98, 0.317), 0.041, HYD, 0.35, region="mast", mirror=True, views="e")
    for s, sign in (("l", 1), ("r", -1)):
        c = [(0.135, 0.218), (0.153, 0.218), (0.153, 0.264), (0.237, 0.264), (0.237, 0.218), (0.255, 0.218), (0.255, 0.282), (0.135, 0.282)]
        S.upright(f"inner-rail-{s}", mi, [(x, sign * z) for x, z in (c if sign > 0 else reversed(c))], (0.16, K.MAST_TOP_INNER - 0.055), STEEL, 0.45, region="mast", views="efp" if s == "r" else "fp")
    S.box("inner-head", mi, (0.05, 0.255), (K.MAST_TOP_INNER - 0.055, K.MAST_TOP_INNER), (-0.282, 0.282), STEEL, 0.45, region="mast")
    S.cyl("lift-rod", mi, (0.075, 0.40, 0.317), (0.075, K.MAST_TOP_INNER - 0.055, 0.317), 0.0225, HYD, 0.8, region="mast", mirror=True, views="e")
    S.cyl("sheave", mi, (0.23, 2.10, 0.075), (0.23, 2.10, 0.125), 0.055, STEEL, 0.4, region="mast", mirror=True, views="e")
    S.box("chain", mi, (0.272, 0.288), (0.60, 2.10), (0.08, 0.12), STEEL, 0.25, region="mast", mirror=True, views="ef", flags={"slender": True})
    S.box("carriage-top", ca, (0.325, 0.385), (0.38, 0.497), (-0.56, 0.56), STEEL, 0.5, region="fork-gear")
    S.box("carriage-low", ca, (0.335, 0.385), (0.09, 0.16), (-0.56, 0.56), STEEL, 0.5, region="fork-gear")
    S.box("carriage-plate", ca, (0.16, 0.385), (0.09, 0.52), (0.18, 0.20), STEEL, 0.45, region="fork-gear", mirror=True, views="ep")
    S.box("backrest-top", ca, (0.355, 0.385), (1.26, 1.30), (-0.56, 0.56), STEEL, 0.4, region="fork-gear")
    S.box("backrest-post", ca, (0.345, 0.385), (0.497, 1.26), (0.52, 0.56), STEEL, 0.4, region="fork-gear", mirror=True)
    for k, zc in enumerate((-0.40, -0.24, -0.08, 0.08, 0.24, 0.40)):
        S.box(f"backrest-bar-{k + 1}", ca, (0.36, 0.385), (0.497, 1.26), (zc - 0.0125, zc + 0.0125), STEEL, 0.4, region="fork-gear", views="f")
    S.box("backrest-mid", ca, (0.355, 0.385), (0.86, 0.89), (-0.56, 0.56), STEEL, 0.4, region="fork-gear", views="fp")
    t0, t1, top = 0.02, K.FORK_TOP, xf + K.FORK_L
    tine = [(0.385, 0.06)] + K.arc(0.425, 0.06, 0.04, 180, 270, 3) + [(top - 0.15, t0), (top, t0 + 0.004), (top, t0 + 0.015), (top - 0.15, t1), (xf, t1), (xf, 0.497), (0.425, 0.522), (0.318, 0.522), (0.318, 0.47), (0.385, 0.47)]
    for s, sign in (("l", 1), ("r", -1)):
        za, zb = sorted((sign * (K.FORK_Z - K.FORK_W / 2), sign * (K.FORK_Z + K.FORK_W / 2)))
        S.ext(f"tine-{s}", fo, tine, (za, zb), STEEL, 0.3, region="fork-gear", views="efp" if s == "r" else "fp", plan=[(0.318, za), (top - 0.04, za), (top, za + 0.015), (top, zb - 0.015), (top - 0.04, zb), (0.318, zb)])
    p0 = K.PALLET_X
    S.box("deck", pa, (p0, p0 + 1.2), (0.122, 0.144), (-0.4, 0.4), WOOD, 0.62, region="pallet")
    for k, (x0, x1) in enumerate([(p0, p0 + 0.145), (p0 + 0.5275, p0 + 0.6725), (p0 + 1.055, p0 + 1.2)]):
        S.box(f"stringer-{k + 1}", pa, (x0, x1), (0.100, 0.122), (-0.4, 0.4), WOOD, 0.6, region="pallet", views="ef")
        for j, (z0, z1) in enumerate([(-0.4, -0.3), (-0.0725, 0.0725), (0.3, 0.4)]):
            S.box(f"block-{k + 1}-{j + 1}", pa, (x0, x1), (0.022, 0.100), (z0, z1), WOOD, 0.55, region="pallet", views="ef" if j == 0 else ("f" if k == 0 else ""))
    for j, (z0, z1) in enumerate([(-0.4, -0.3), (-0.0725, 0.0725), (0.3, 0.4)]):
        S.box(f"bottom-{j + 1}", pa, (p0, p0 + 1.2), (0.0, 0.022), (z0, z1), WOOD, 0.6, region="pallet", views="ef" if j == 0 else "f")
    S.box("goods", pa, (p0 + 0.01, p0 + 1.19), (0.144, 1.044), (-0.39, 0.39), GOODS, 0.78, region="cargo", radius=0.02)
    for k, x in enumerate((p0 + 0.40, p0 + 0.80)):
        S.stroke(f"strap-e-{k + 1}", pa, "elevation", [(x, 0.144), (x, 1.044)], "goods", tone=0.3, material=GOODS)
        S.stroke(f"strap-p-{k + 1}", pa, "plan", [(x, -0.39), (x, 0.39)], "goods", tone=0.3, material=GOODS)

    st = dict(target="structure")
    S.box("frame-side", "chassis", (-1.40, -0.36), (0.15, 0.60), (0.515, 0.545), STEEL, 0.5, mirror=True, member="chassis", part="member", **st)
    for id in ("frame-front", "floor-pan", "post-front", "post-rear", "roof-rail"):
        S.ref(f"m-{id}", id, "chassis", "member")
    S.box("mass-chassis", "chassis", (-0.76, -0.64), (0.66, 0.78), (-0.06, 0.06), STEEL, 0.5, radius=0.059, member="chassis", part="mass", **st)
    S.ref("m-cw-block", "cw-block", "counterweight", "member")
    S.box("mass-counterweight", "counterweight", (-1.86, -1.74), (0.66, 0.78), (-0.06, 0.06), IRON, 0.3, radius=0.059, member="counterweight", part="mass", **st)
    S.box("battery-tray", "battery", (-1.071, -0.36), (0.15, 0.17), (-0.514, 0.514), BAT, 0.4, member="battery", part="member", **st)
    S.box("mass-battery", "battery", (-0.7755, -0.6555), (0.482, 0.602), (-0.06, 0.06), BAT, 0.4, radius=0.059, member="battery", part="mass", **st)
    S.cyl("drive-axle", "axle-f", (0.0, K.RF, -0.43), (0.0, K.RF, 0.43), 0.09, STEEL, 0.4, member="axle-f", part="member", **st)
    S.ref("m-steer-axle", "steer-axle", "axle-r", "member")
    S.cyl("stub-axles", "axle-r", (-K.WB, K.RR, -0.40), (-K.WB, K.RR, 0.40), 0.04, STEEL, 0.4, member="axle-r", part="joint", **st)
    for s in ("l", "r"):
        S.ref(f"m-tyre-f-{s}", f"tyre-f-{s}", "wheel-f", "member")
        S.ref(f"m-tyre-r-{s}", f"tyre-r-{s}", "wheel-r", "member")
    for anchor, bone, x, z, length, width in (("contact-f-l", "axle-f", 0.0, K.ZF, 0.15, 0.18), ("contact-f-r", "axle-f", 0.0, -K.ZF, 0.15, 0.18), ("contact-r-l", "axle-r", -K.WB, K.ZR, 0.12, 0.14), ("contact-r-r", "axle-r", -K.WB, -K.ZR, 0.12, 0.14)):
        S.box(f"pad-{anchor[8:]}", bone, (x - length / 2, x + length / 2), (0.0, 0.015), (z - width / 2, z + width / 2), RUBBER, 0.2, member=bone, part="support", anchor=anchor, **st)
    S.ref("m-tilt-barrel", "tilt-barrel", "tilt-cyl", "member")
    S.ref("m-tilt-rod", "tilt-rod", "tilt-rod", "member")
    S.cyl("pin-tilt", "tilt-rod", (K.TILT_PIN[0], K.TILT_PIN[1], 0.37), (K.TILT_PIN[0], K.TILT_PIN[1], 0.47), 0.02, STEEL, 0.4, mirror=True, member="tilt-rod", part="joint", **st)
    for id in ("outer-rail-l", "outer-rail-r", "outer-tie-top", "lift-cyl"):
        S.ref(f"m-{id}", id, "mast-outer", "member")
    S.cyl("pin-pivot", "mast-outer", (K.PIVOT[0], K.PIVOT[1], -0.375), (K.PIVOT[0], K.PIVOT[1], 0.375), 0.03, STEEL, 0.3, member="mast-outer", part="joint", **st)
    for id in ("inner-rail-l", "inner-rail-r", "inner-head", "lift-rod"):
        S.ref(f"m-{id}", id, "mast-inner", "member")
    for id in ("carriage-top", "carriage-low", "carriage-plate", "backrest-post"):
        S.ref(f"m-{id}", id, "carriage", "member")
    S.ref("m-tine-l", "tine-l", "forks", "member")
    S.ref("m-tine-r", "tine-r", "forks", "member")
    S.ref("m-bottom-1", "bottom-1", "pallet-x", "member")
    S.ref("m-bottom-3", "bottom-3", "pallet-x", "member")
    for k in (1, 2, 3):
        S.ref(f"m-block-{k}-2", f"block-{k}-2", "pallet-y", "member")
        S.ref(f"m-stringer-{k}", f"stringer-{k}", "pallet", "member")
    return S


DECISIONS = [
    text("1:100 is the same EFG 425 authored for 15 mm: frame sides with the battery-door seam, hood, fenders, cowl, column and a six-sided steering-wheel ring, seat with base, cushion and backrest lofts and the armrest, guard with five 15 mm roof slats, beacon and work lamps; counterweight loft, apron, housings, coupling, lamps and vent grille; tyres with chamfered shoulders and one recessed wheel disc each; nested C-section mast rails with 16 to 20 mm walls, lift cylinders, chains, carriage bars and side plates, backrest with six 25 mm bars, forks with the top hook and tapered tips, the Euro pallet with one deck slab over its stringers, blocks and bottom boards.",
         "1:100 ist derselbe EFG 425 für 15 mm erstellt: Rahmenseiten mit der Batterietürfuge, Haube, Kotflügel, Armaturenhaube, Lenksäule und sechseckiger Lenkradkranz, Sitz mit Unterbau, Kissen- und Lehnenloft und Armlehne, Schutzdach mit fünf 15-mm-Streben, Rundumleuchte und Arbeitsscheinwerfer; Gegengewichtsloft, Schürze, Radhäuser, Kupplung, Leuchten und Lüftungsgitter; Reifen mit gefasten Schultern und je einer vertieften Radschüssel; geschachtelte C-Profil-Mastschienen mit 16 bis 20 mm Wandstärke, Hubzylinder, Ketten, Gabelträgerleisten und Seitenbleche, Lastschutzgitter mit sechs 25-mm-Stäben, Gabeln mit oberem Haken und verjüngten Spitzen, die Europalette mit einer Deckplatte über Querbrettern, Klötzen und Bodenbrettern."),
    text("Merged or dropped against 1:50: wheel nuts, hub caps, pedals, floor ribs, spokes, the separate deck boards, rollers, pivot pins and mast ties below the head; the twin drive motors are one hidden cylinder with their terminal boxes. Structure keeps the frame plates, guard frame, counterweight, axles, tyres, tilt cylinders, both mast stages, carriage, forks and pallet with one pin per joint group.",
         "Gegenüber 1:50 zusammengefasst oder weggelassen: Radmuttern, Nabenkappen, Pedale, Bodenrippen, Speichen, die einzelnen Deckbretter, Rollen, Lagerbolzen und Mastquerträger unter dem Kopf; die beiden Fahrmotoren sind ein verdeckter Zylinder mit ihren Klemmenkästen. Das Tragwerk behält Rahmenbleche, Schutzdachrahmen, Gegengewicht, Achsen, Reifen, Neigezylinder, beide Maststufen, Gabelträger, Gabeln und Palette mit je einem Bolzen pro Gelenkgruppe."),
]
