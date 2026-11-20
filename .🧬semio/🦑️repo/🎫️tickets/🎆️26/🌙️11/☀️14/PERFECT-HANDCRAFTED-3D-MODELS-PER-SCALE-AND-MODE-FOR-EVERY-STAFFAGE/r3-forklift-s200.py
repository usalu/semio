"""🪛 The 1:200 forklift (smallest feature 30 mm): 30 mm frame plates around the hidden battery, merged fenders, solid mast bars, one carriage plate, a three-runner pallet and plain wheels, still the EFG 425 in every proportion."""
import math

PAINT, STEEL, IRON, RUBBER, BAT, DRIVE, HYD, WOOD, GOODS, SEAT = "paint", "steel", "cast-iron", "rubber", "battery-case", "drive-electronics", "hydraulic", "wood", "goods", "interior"


def text(en, de):
    return {"en": en, "de": de}


def build(S, K):
    xr, xf = K.REAR, K.FACE
    body = [(-1.40, 0.135), (-0.30, 0.135)] + K.arc(0.0, K.RF, 0.34, 208, 152, 3) + [(-0.30, 0.66), (-0.355, 0.66), (-0.355, 0.99), (-1.40, 0.99)]
    S.ext("body", "chassis", body, (0.545, 0.575), PAINT, 0.55, region="chassis-paint", mirror=True)
    S.box("hood", "chassis", (-1.40, -0.355), (0.96, 0.99), (-0.545, 0.545), PAINT, 0.55, region="chassis-paint", views="fp")
    S.box("floor-pan", "chassis", (-1.40, -0.355), (0.135, 0.165), (-0.545, 0.545), PAINT, 0.5, region="chassis-paint", views="")
    S.box("kick-panel", "chassis", (-0.385, -0.355), (0.165, 0.96), (-0.545, 0.545), PAINT, 0.5, region="chassis-paint", views="f")
    S.stroke("battery-door-seam", "chassis", "elevation", [(-1.33, 0.20), (-0.43, 0.20), (-0.43, 0.88), (-1.33, 0.88), (-1.33, 0.20)], "body-r", tone=0.35, material=PAINT)
    S.ext("fender", "chassis", K.arc(0.0, K.RF, 0.345, 30, 152, 6) + [(-0.36, 0.46), (-0.36, 0.665), (0.215, 0.665)], (0.375, 0.60), PAINT, 0.5, region="chassis-paint", mirror=True, views="ef")
    S.box("frame-front", "chassis", (-0.385, 0.06), (0.15, 0.635), (0.345, 0.375), PAINT, 0.45, region="chassis-paint", mirror=True, views="ef")
    S.box("floor-plate", "chassis", (-0.355, 0.02), (0.635, 0.665), (-0.375, 0.375), RUBBER, 0.2, region="chassis-paint", views="fp")
    S.box("cowl", "chassis", (-0.25, -0.03), (0.665, 1.095), (-0.36, 0.36), DRIVE, 0.25, region="controller", radius=0.03)
    S.cyl("column", "chassis", (-0.19, 1.07, 0), (-0.315, 1.255, 0), 0.035, SEAT, 0.2)
    S.cyl("steering-wheel", "chassis", (-0.32, 1.26, 0), (-0.337, 1.285, 0), 0.165, SEAT, 0.2, front=K.rounded_rect(-0.165, 0.165, 1.17, 1.38, 0.04))
    S.box("seat", "chassis", (-1.02, -0.56), (0.99, 1.15), (-0.25, 0.25), SEAT, 0.22, radius=0.03)
    S.lofty("seat-back", "chassis", [(1.11, -1.035, 0, 0.05, 0.23, 3), (1.45, -1.085, 0, 0.065, 0.25, 3), (1.70, -1.12, 0, 0.045, 0.20, 3)], SEAT, 0.22)
    S.box("armrest", "chassis", (-0.93, -0.59), (1.15, 1.30), (-0.345, -0.255), SEAT, 0.2)
    S.ext("post-front", "chassis", [(-0.06, 0.665), (0.0, 0.665), (-0.23, 2.18), (-0.29, 2.18)], (0.47, 0.53), PAINT, 0.25, region="guard", mirror=True)
    S.box("post-rear", "chassis", (-1.49, -1.43), (0.99, 2.18), (0.47, 0.53), PAINT, 0.25, region="guard", mirror=True)
    S.box("roof-rail", "chassis", (-1.52, -0.23), (2.18, 2.24), (0.47, 0.53), PAINT, 0.3, region="guard", mirror=True)
    S.box("roof-front", "chassis", (-0.29, -0.23), (2.18, 2.24), (-0.47, 0.47), PAINT, 0.3, region="guard", views="fp")
    S.box("roof-rear", "chassis", (-1.52, -1.46), (2.18, 2.24), (-0.47, 0.47), PAINT, 0.3, region="guard", views="fp")
    for k, z in enumerate((-0.235, 0.0, 0.235)):
        S.box(f"roof-slat-{k + 1}", "chassis", (-1.46, -0.29), (2.19, 2.235), (z - 0.015, z + 0.015), PAINT, 0.3, region="guard", views="p")
    S.rev("beacon", "chassis", (-1.49, 2.24, 0), (0, 1, 0), [(0, 0), (0, 0.05), (0.03, 0.045), (0.085, 0.0)], SEAT, 0.85,
          front=[(-0.05, 2.24), (0.05, 2.24), (0.045, 2.27), (0, 2.325), (-0.045, 2.27)], plan=[(-1.49 + 0.05 * math.cos(t), 0.05 * math.sin(t)) for t in [2 * math.pi * k / 12 for k in range(12)]])

    S.cyl("motor", "axle-f", (0.0, K.RF, -0.385), (0.0, K.RF, 0.385), 0.13, DRIVE, 0.3, region="drive-motor", target="thermal")
    S.cyl("gear-brake", "axle-f", (0.0, K.RF, 0.30), (0.0, K.RF, 0.385), 0.11, IRON, 0.35, region="brakes", mirror=True, target="thermal")
    S.box("steer-axle", "axle-r", (-1.80, -1.64), (0.17, 0.29), (-0.38, 0.38), STEEL, 0.35, views="")
    S.cyl("steer-cylinder", "axle-r", (-1.84, 0.235, -0.33), (-1.84, 0.235, 0.33), 0.03, HYD, 0.35, region="hydraulics", views="")

    S.loft("cw-block", "counterweight", [(xr, 0.7725, 0, 0.515, 0.26, 6), (xr + 0.05, 0.7725, 0, 0.575, 0.2725, 8), (-1.40, 0.7725, 0, 0.575, 0.2725, 8)], IRON, 0.3, region="counterweight")
    apron = [(xr + 0.01, 0.20), (-1.985, 0.20)] + K.arc(-K.WB, K.RR, 0.27, 168.9, 112, 3) + [(-1.82, 0.52), (xr + 0.01, 0.52)]
    S.ext("cw-apron", "counterweight", apron, (-0.49, 0.49), IRON, 0.3, region="counterweight")
    S.ext("cw-foot", "counterweight", [(-1.40, 0.30), (-1.40, 0.52), (-1.62, 0.52)] + K.arc(-K.WB, K.RR, 0.27, 68, 22, 3), (-0.575, 0.575), IRON, 0.3, region="counterweight", views="e")
    S.ext("cw-housing", "counterweight", K.arc(-K.WB, K.RR, 0.27, 20, 160, 4), (-0.36, 0.36), IRON, 0.3, region="counterweight", views="e")
    S.box("vent-outlet", "counterweight", (xr - 0.03, xr), (0.60, 0.72), (-0.15, 0.15), IRON, 0.2, region="counterweight", target="thermal", flags={"tags": ["motor-vent"]})
    S.box("battery", "battery", (-1.071, -0.36), (0.15, 0.934), (-0.514, 0.514), BAT, 0.4, region="battery-pack", target="thermal")

    for tag, bone, radius, width, z in (("f", "wheel-f", K.RF, K.TF, K.ZF), ("r", "wheel-r", K.RR, K.TR, K.ZR)):
        x0, y0, _ = K.REST[bone]
        h = width / 2
        for side, suffix in ((1, "l"), (-1, "r")):
            S.rev(f"tyre-{tag}-{suffix}", bone, (x0, y0, side * z), (0, 0, side), [(-h, 0), (-h, radius - 0.03), (-h + 0.03, radius), (h - 0.03, radius), (h, radius - 0.03), (h, 0)], RUBBER, 0.1, region="tyres", views="efp" if side < 0 else "fp",
                  front=K.rounded_rect(side * z - h, side * z + h, y0 - radius, y0 + radius, 0.03), plan=K.rounded_rect(x0 - radius, x0 + radius, side * z - h, side * z + h, 0.03))
        S.stroke(f"rim-{tag}", bone, "elevation", [(x0 + 0.62 * radius * math.cos(2 * math.pi * k / 12), y0 + 0.62 * radius * math.sin(2 * math.pi * k / 12)) for k in range(13)], f"tyre-{tag}-r", tone=0.5, material=PAINT)

    b0 = K.REST["tilt-cyl"]
    d = (math.cos(math.radians(b0[2])), math.sin(math.radians(b0[2])))
    bx, by = K.TILT_BASE
    S.rev("tilt-barrel", "tilt-cyl", (bx, by, K.TILT_Z), (d[0], d[1], 0), [(-0.03, 0), (-0.03, 0.045), (0.27, 0.045), (0.27, 0)], HYD, 0.35, region="hydraulics", mirror=True)
    rx0 = K.REST["tilt-rod"]
    S.cyl("tilt-rod", "tilt-rod", (rx0[0], rx0[1], K.TILT_Z), (rx0[0] + (K.TILT_ROD + 0.02) * d[0], rx0[1] + (K.TILT_ROD + 0.02) * d[1], K.TILT_Z), 0.03, HYD, 0.8, region="hydraulics", mirror=True)

    mo, mi, ca, fo, pa = "mast-outer", "mast-inner", "carriage", "forks", "pallet"
    S.box("outer-rail", mo, (0.12, 0.245), (0.12, K.MAST_TOP_OUTER), (0.282, 0.352), STEEL, 0.35, region="mast", mirror=True)
    S.box("outer-tie-top", mo, (0.10, 0.13), (2.09, K.MAST_TOP_OUTER), (-0.30, 0.30), STEEL, 0.35, region="mast", views="fp")
    S.box("tilt-lug", mo, (0.15, 0.21), (0.90, 1.00), (0.352, 0.445), STEEL, 0.35, region="mast", mirror=True)
    S.cyl("lift-cyl", mo, (0.075, 0.20, 0.317), (0.075, 1.98, 0.317), 0.04, HYD, 0.4, region="mast", mirror=True, views="e")
    S.box("inner-rail", mi, (0.135, 0.255), (0.16, K.MAST_TOP_INNER - 0.055), (0.218, 0.282), STEEL, 0.45, region="mast", mirror=True)
    S.box("inner-head", mi, (0.05, 0.255), (K.MAST_TOP_INNER - 0.06, K.MAST_TOP_INNER), (-0.282, 0.282), STEEL, 0.45, region="mast")
    S.cyl("lift-rod", mi, (0.075, 0.40, 0.317), (0.075, K.MAST_TOP_INNER - 0.06, 0.317), 0.03, HYD, 0.8, region="mast", mirror=True, views="e")
    S.box("carriage", ca, (0.325, 0.385), (0.09, 0.497), (-0.56, 0.56), STEEL, 0.5, region="fork-gear")
    S.box("backrest-top", ca, (0.355, 0.385), (1.26, 1.30), (-0.56, 0.56), STEEL, 0.4, region="fork-gear")
    S.box("backrest-post", ca, (0.345, 0.385), (0.497, 1.26), (0.52, 0.56), STEEL, 0.4, region="fork-gear", mirror=True)
    for k, zc in enumerate((-0.24, 0.0, 0.24)):
        S.box(f"backrest-bar-{k + 1}", ca, (0.355, 0.385), (0.497, 1.26), (zc - 0.015, zc + 0.015), STEEL, 0.4, region="fork-gear", views="f")
    t0, t1, top = 0.02, K.FORK_TOP, xf + K.FORK_L
    tine = [(0.385, t0), (top - 0.12, t0), (top, t0 + 0.01), (top, t0 + 0.03), (top - 0.12, t1), (xf, t1), (xf, 0.522), (0.385, 0.522)]
    for s, sign in (("l", 1), ("r", -1)):
        za, zb = sorted((sign * (K.FORK_Z - K.FORK_W / 2), sign * (K.FORK_Z + K.FORK_W / 2)))
        S.ext(f"tine-{s}", fo, tine, (za, zb), STEEL, 0.3, region="fork-gear", views="efp" if s == "r" else "fp")
    p0 = K.PALLET_X
    S.box("deck", pa, (p0, p0 + 1.2), (0.10, 0.144), (-0.4, 0.4), WOOD, 0.62, region="pallet")
    for j, (z0, z1) in enumerate([(-0.4, -0.3), (-0.0725, 0.0725), (0.3, 0.4)]):
        S.box(f"runner-{j + 1}", pa, (p0, p0 + 1.2), (0.0, 0.10), (z0, z1), WOOD, 0.55, region="pallet", views="ef" if j == 0 else "f")
    S.stroke("runner-blocks", pa, "elevation", [(p0 + 0.145, 0.03), (p0 + 0.5275, 0.03), (p0 + 0.5275, 0.07), (p0 + 0.145, 0.07), (p0 + 0.145, 0.03)], "runner-1", tone=0.4, material=WOOD)
    S.box("goods", pa, (p0 + 0.01, p0 + 1.19), (0.144, 1.044), (-0.39, 0.39), GOODS, 0.78, region="cargo")

    st = dict(target="structure")
    S.box("frame-side", "chassis", (-1.40, -0.36), (0.15, 0.60), (0.515, 0.545), STEEL, 0.5, mirror=True, member="chassis", part="member", **st)
    S.ref("m-floor-pan", "floor-pan", "chassis", "member")
    S.ref("m-frame-front", "frame-front", "chassis", "member")
    for id in ("post-front", "post-rear", "roof-rail"):
        S.ref(f"m-{id}", id, "chassis", "member")
    S.box("mass-chassis", "chassis", (-0.76, -0.64), (0.66, 0.78), (-0.06, 0.06), STEEL, 0.5, member="chassis", part="mass", **st)
    S.ref("m-cw-block", "cw-block", "counterweight", "member")
    S.box("mass-counterweight", "counterweight", (-1.86, -1.74), (0.66, 0.78), (-0.06, 0.06), IRON, 0.3, member="counterweight", part="mass", **st)
    S.box("battery-tray", "battery", (-1.071, -0.36), (0.15, 0.18), (-0.514, 0.514), BAT, 0.4, member="battery", part="member", **st)
    S.box("mass-battery", "battery", (-0.7755, -0.6555), (0.482, 0.602), (-0.06, 0.06), BAT, 0.4, member="battery", part="mass", **st)
    S.cyl("drive-axle", "axle-f", (0.0, K.RF, -0.43), (0.0, K.RF, 0.43), 0.09, STEEL, 0.4, member="axle-f", part="member", **st)
    S.ref("m-steer-axle", "steer-axle", "axle-r", "member")
    for s in ("l", "r"):
        S.ref(f"m-tyre-f-{s}", f"tyre-f-{s}", "wheel-f", "member")
        S.ref(f"m-tyre-r-{s}", f"tyre-r-{s}", "wheel-r", "member")
    for anchor, bone, x, z, length, width in (("contact-f-l", "axle-f", 0.0, K.ZF, 0.15, 0.18), ("contact-f-r", "axle-f", 0.0, -K.ZF, 0.15, 0.18), ("contact-r-l", "axle-r", -K.WB, K.ZR, 0.12, 0.14), ("contact-r-r", "axle-r", -K.WB, -K.ZR, 0.12, 0.14)):
        S.box(f"pad-{anchor[8:]}", bone, (x - length / 2, x + length / 2), (0.0, 0.03), (z - width / 2, z + width / 2), RUBBER, 0.2, member=bone, part="support", anchor=anchor, **st)
    S.ref("m-tilt-barrel", "tilt-barrel", "tilt-cyl", "member")
    S.ref("m-tilt-rod", "tilt-rod", "tilt-rod", "member")
    for id in ("outer-rail", "outer-tie-top"):
        S.ref(f"m-{id}", id, "mast-outer", "member")
    S.cyl("pin-pivot", "mast-outer", (K.PIVOT[0], K.PIVOT[1], -0.375), (K.PIVOT[0], K.PIVOT[1], 0.375), 0.03, STEEL, 0.3, member="mast-outer", part="joint", **st)
    for id in ("inner-rail", "inner-head"):
        S.ref(f"m-{id}", id, "mast-inner", "member")
    S.ref("m-carriage", "carriage", "carriage", "member")
    S.ref("m-backrest-post", "backrest-post", "carriage", "member")
    S.ref("m-tine-l", "tine-l", "forks", "member")
    S.ref("m-tine-r", "tine-r", "forks", "member")
    S.ref("m-runner-1", "runner-1", "pallet-x", "member")
    S.ref("m-runner-3", "runner-3", "pallet-x", "member")
    S.ref("m-runner-2", "runner-2", "pallet-y", "member")
    S.ref("m-deck", "deck", "pallet", "member")
    return S


DECISIONS = [
    text("1:200 keeps the EFG 425 in every proportion at 30 mm: the frame sides are 30 mm side-profile plates with the front wheel arch around the hidden battery, under the hood and over the floor pan, the fenders are one curved mass with their deck, the cowl a rounded block with column and steering-wheel disc, the seat one cushion block with its backrest loft and armrest; overhead guard posts, roof frame and three 30 mm slats, beacon; counterweight loft with apron, wheel-housing feet and the axle housing; plain tyres with chamfered shoulders; solid mast bars, lift cylinders, inner head, one carriage plate over both bars, backrest frame with three bars, forks as L profiles; the pallet as deck and three runners.",
         "1:200 behält den EFG 425 bei 30 mm in allen Proportionen: Die Rahmenseiten sind 30 mm dicke Seitenprofilbleche mit dem vorderen Radlauf um die verdeckte Batterie, unter der Haube und über der Bodenwanne, die Kotflügel eine gewölbte Masse mit ihrer Abdeckung, die Armaturenhaube ein gerundeter Block mit Lenksäule und Lenkradscheibe, der Sitz ein Kissenblock mit Lehnenloft und Armlehne; Schutzdachpfosten, Dachrahmen und drei 30-mm-Streben, Rundumleuchte; Gegengewichtsloft mit Schürze, Radhausfüßen und Achsgehäuse; glatte Reifen mit gefasten Schultern; volle Maststäbe, Hubzylinder, Innenmastkopf, eine Gabelträgerplatte über beiden Leisten, Lastschutzrahmen mit drei Stäben, Gabeln als L-Profile; die Palette als Deck und drei Kufen."),
    text("Dropped against 1:100: doors, step, lamps, coupling, grille, pedals, chains, sheaves, wheel discs (drawn as a rim line), separate pallet boards and blocks (one runner per block row; the openings stay in the front view). Thermal regions keep their solids; the twin motors are one hidden cylinder. Structure: frame plates and floor, guard frame, counterweight, axles, tyres, tilt cylinders, both mast stages, carriage, forks and pallet, one pivot pin.",
         "Gegenüber 1:100 weggelassen: Türen, Stufe, Leuchten, Kupplung, Gitter, Pedale, Ketten, Kettenrollen, Radschüsseln (als Felgenlinie gezeichnet), einzelne Palettenbretter und -klötze (eine Kufe je Klotzreihe; die Öffnungen bleiben in der Vorderansicht). Die Wärmeregionen behalten ihre Körper; die beiden Motoren sind ein verdeckter Zylinder. Tragwerk: Rahmenbleche und Boden, Schutzdachrahmen, Gegengewicht, Achsen, Reifen, Neigezylinder, beide Maststufen, Gabelträger, Gabeln und Palette, ein Mastlagerbolzen."),
]
