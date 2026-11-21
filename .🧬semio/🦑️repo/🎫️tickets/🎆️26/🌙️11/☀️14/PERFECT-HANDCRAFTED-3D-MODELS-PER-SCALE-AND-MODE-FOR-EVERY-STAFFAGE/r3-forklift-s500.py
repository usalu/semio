"""🗜️ The 1:500 forklift (smallest feature 75 mm): 75 mm frame plates, fenders and counterweight as side-profile masses, the guard as 75 mm posts under a roof slab, two mast stages as 75 mm rails, carriage plate and backrest frame, L forks, pallet block with goods and disc wheels; the proportions of the EFG 425 kept."""
import math

PAINT, STEEL, IRON, RUBBER, BAT, DRIVE, HYD, WOOD, GOODS, SEAT = "paint", "steel", "cast-iron", "rubber", "battery-case", "drive-electronics", "hydraulic", "wood", "goods", "interior"


def text(en, de):
    return {"en": en, "de": de}


def build(S, K):
    xr, xf = K.REAR, K.FACE
    body = [(-1.40, 0.135), (-0.30, 0.135)] + K.arc(0.0, K.RF, 0.34, 208, 152, 2) + [(-0.30, 0.66), (-0.355, 0.66), (-0.355, 0.99), (-1.40, 0.99)]
    S.ext("body", "chassis", body, (0.50, 0.575), PAINT, 0.55, region="chassis-paint", mirror=True)
    S.box("hood", "chassis", (-1.40, -0.355), (0.915, 0.99), (-0.50, 0.50), PAINT, 0.55, region="chassis-paint", views="fp")
    S.box("floor-pan", "chassis", (-1.40, -0.355), (0.135, 0.21), (-0.50, 0.50), PAINT, 0.5, region="chassis-paint", views="")
    S.box("kick-panel", "chassis", (-0.43, -0.355), (0.21, 0.915), (-0.50, 0.50), PAINT, 0.5, region="chassis-paint", views="f")
    S.ext("fender", "chassis", K.arc(0.0, K.RF, 0.345, 30, 152, 4) + [(-0.36, 0.46), (-0.36, 0.665), (0.215, 0.665)], (0.375, 0.60), PAINT, 0.5, region="chassis-paint", mirror=True, views="efp")
    S.box("frame-front", "chassis", (-0.36, 0.06), (0.15, 0.665), (-0.375, 0.375), PAINT, 0.45, region="chassis-paint", views="fp")
    S.box("cowl", "chassis", (-0.25, -0.03), (0.665, 1.095), (-0.36, 0.36), DRIVE, 0.25, region="controller")
    S.box("seat", "chassis", (-1.02, -0.56), (0.99, 1.15), (-0.25, 0.25), SEAT, 0.22, views="ep")
    S.ext("seat-back", "chassis", [(-1.00, 1.11), (-1.075, 1.11), (-1.16, 1.70), (-1.085, 1.70)], (-0.25, 0.25), SEAT, 0.22)
    S.ext("post-front", "chassis", [(-0.075, 0.665), (0.0, 0.665), (-0.235, 2.165), (-0.31, 2.165)], (0.4625, 0.5375), PAINT, 0.25, region="guard", mirror=True)
    S.box("post-rear", "chassis", (-1.5, -1.425), (0.99, 2.165), (0.4625, 0.5375), PAINT, 0.25, region="guard", mirror=True)
    S.box("roof", "chassis", (-1.52, -0.235), (2.165, 2.24), (-0.5375, 0.5375), PAINT, 0.3, region="guard")
    S.rev("beacon", "chassis", (-1.49, 2.24, 0), (0, 1, 0), [(0, 0), (0, 0.05), (0.085, 0.0)], SEAT, 0.85,
          front=[(-0.05, 2.24), (0.05, 2.24), (0, 2.325)], plan=[(-1.49 + 0.05 * math.cos(t), 0.05 * math.sin(t)) for t in [2 * math.pi * k / 8 for k in range(8)]])

    S.cyl("motor", "axle-f", (0.0, K.RF, -0.385), (0.0, K.RF, 0.385), 0.13, DRIVE, 0.3, region="drive-motor", target="thermal")
    S.cyl("gear-brake", "axle-f", (0.0, K.RF, 0.30), (0.0, K.RF, 0.385), 0.11, IRON, 0.35, region="brakes", mirror=True, target="thermal")
    S.cyl("steer-axle", "axle-r", (-K.WB, K.RR, -0.38), (-K.WB, K.RR, 0.38), 0.075, STEEL, 0.35, views="")

    S.loft("cw-block", "counterweight", [(xr, 0.7725, 0, 0.52, 0.262, 6), (xr + 0.075, 0.7725, 0, 0.575, 0.2725, 8), (-1.40, 0.7725, 0, 0.575, 0.2725, 8)], IRON, 0.3, region="counterweight")
    lower = [(xr + 0.01, 0.20), (-1.985, 0.20)] + K.arc(-K.WB, K.RR, 0.27, 168.9, 22, 6) + [(-1.40, 0.30), (-1.40, 0.52), (xr + 0.01, 0.52)]
    S.ext("cw-lower", "counterweight", lower, (-0.53, 0.53), IRON, 0.3, region="counterweight")
    S.ext("cw-housing", "counterweight", K.arc(-K.WB, K.RR, 0.27, 20, 160, 5), (-0.36, 0.36), IRON, 0.3, region="counterweight", views="e")
    S.box("vent-outlet", "counterweight", (xr - 0.075, xr), (0.60, 0.72), (-0.15, 0.15), IRON, 0.2, region="counterweight", target="thermal", flags={"tags": ["motor-vent"]})
    S.box("battery", "battery", (-1.071, -0.36), (0.15, 0.934), (-0.514, 0.514), BAT, 0.4, region="battery-pack", target="thermal")

    for tag, bone, radius, width, z in (("f", "wheel-f", K.RF, K.TF, K.ZF), ("r", "wheel-r", K.RR, K.TR, K.ZR)):
        x0, y0, _ = K.REST[bone]
        h = width / 2
        for side, suffix in ((1, "l"), (-1, "r")):
            S.rev(f"tyre-{tag}-{suffix}", bone, (x0, y0, side * z), (0, 0, side), [(-h, 0), (-h, radius), (h, radius), (h, 0)], RUBBER, 0.1, region="tyres", views="efp" if side < 0 else "fp")

    b0 = K.REST["tilt-cyl"]
    d = (math.cos(math.radians(b0[2])), math.sin(math.radians(b0[2])))
    bx, by = K.TILT_BASE
    S.rev("tilt-barrel", "tilt-cyl", (bx, by, K.TILT_Z), (d[0], d[1], 0), [(-0.04, 0), (-0.04, 0.045), (0.27, 0.045), (0.27, 0)], HYD, 0.35, region="hydraulics", mirror=True)
    rx0 = K.REST["tilt-rod"]
    S.cyl("tilt-rod", "tilt-rod", (rx0[0], rx0[1], K.TILT_Z), (rx0[0] + (K.TILT_ROD + 0.03) * d[0], rx0[1] + (K.TILT_ROD + 0.03) * d[1], K.TILT_Z), 0.0375, HYD, 0.8, region="hydraulics", mirror=True)

    mo, mi, ca, fo, pa = "mast-outer", "mast-inner", "carriage", "forks", "pallet"
    S.box("outer-rail", mo, (0.12, 0.245), (0.12, K.MAST_TOP_OUTER), (0.282, 0.357), STEEL, 0.35, region="mast", mirror=True)
    S.box("inner-rail", mi, (0.135, 0.255), (0.16, K.MAST_TOP_INNER - 0.075), (0.207, 0.282), STEEL, 0.45, region="mast", mirror=True)
    S.box("inner-head", mi, (0.05, 0.255), (K.MAST_TOP_INNER - 0.075, K.MAST_TOP_INNER), (-0.282, 0.282), STEEL, 0.45, region="mast")
    S.box("carriage", ca, (0.31, 0.385), (0.09, 0.52), (-0.56, 0.56), STEEL, 0.5, region="fork-gear", views="ef")
    S.box("backrest-top", ca, (0.31, 0.385), (1.225, 1.30), (-0.56, 0.56), STEEL, 0.4, region="fork-gear")
    S.box("backrest-post", ca, (0.31, 0.385), (0.52, 1.225), (0.485, 0.56), STEEL, 0.4, region="fork-gear", mirror=True)
    S.box("backrest-bar", ca, (0.31, 0.385), (0.52, 1.225), (-0.0375, 0.0375), STEEL, 0.4, region="fork-gear", views="f")
    top = xf + K.FORK_L
    tine = [(0.35, 0.0), (top, 0.0), (top, 0.075), (xf, 0.075), (xf, 0.522), (0.35, 0.522)]
    for s, sign in (("l", 1), ("r", -1)):
        za, zb = sorted((sign * (K.FORK_Z - K.FORK_W / 2), sign * (K.FORK_Z + K.FORK_W / 2)))
        S.ext(f"tine-{s}", fo, tine, (za, zb), STEEL, 0.3, region="fork-gear", views="efp" if s == "r" else "fp")
    p0 = K.PALLET_X
    S.box("pallet", pa, (p0, p0 + 1.2), (0.0, 0.144), (-0.4, 0.4), WOOD, 0.6, region="pallet")
    S.box("goods", pa, (p0 + 0.01, p0 + 1.19), (0.144, 1.044), (-0.39, 0.39), GOODS, 0.78, region="cargo")

    st = dict(target="structure")
    S.ref("m-body", "body", "chassis", "member")
    S.ref("m-floor-pan", "floor-pan", "chassis", "member")
    S.ref("m-frame-front", "frame-front", "chassis", "member")
    S.ref("m-post-front", "post-front", "chassis", "member")
    S.ref("m-post-rear", "post-rear", "chassis", "member")
    S.box("roof-rail", "chassis", (-1.50, -0.235), (2.165, 2.24), (0.4625, 0.5375), STEEL, 0.3, member="chassis", part="member", mirror=True, **st)
    S.box("mass-chassis", "chassis", (-0.76, -0.64), (0.66, 0.78), (-0.06, 0.06), STEEL, 0.5, member="chassis", part="mass", **st)
    S.box("cw-beam", "counterweight", (-1.86, -1.40), (0.45, 0.525), (-0.50, 0.50), IRON, 0.35, member="counterweight", part="member", **st)
    S.box("mass-counterweight", "counterweight", (-1.86, -1.74), (0.66, 0.78), (-0.06, 0.06), IRON, 0.3, member="counterweight", part="mass", **st)
    S.box("battery-tray", "battery", (-1.071, -0.36), (0.15, 0.225), (-0.514, 0.514), BAT, 0.4, member="battery", part="member", **st)
    S.box("mass-battery", "battery", (-0.7755, -0.6555), (0.482, 0.602), (-0.06, 0.06), BAT, 0.4, member="battery", part="mass", **st)
    S.cyl("drive-axle", "axle-f", (0.0, K.RF, -0.43), (0.0, K.RF, 0.43), 0.09, STEEL, 0.4, member="axle-f", part="member", **st)
    S.ref("m-steer-axle", "steer-axle", "axle-r", "member")
    for s, side in (("l", 1), ("r", -1)):
        S.cyl(f"hub-f-{s}", "wheel-f", (0.0, K.RF, side * (K.ZF - 0.0375)), (0.0, K.RF, side * (K.ZF + 0.0375)), 0.15, STEEL, 0.4, member="wheel-f", part="member", **st)
        S.cyl(f"hub-r-{s}", "wheel-r", (-K.WB, K.RR, side * (K.ZR - 0.0375)), (-K.WB, K.RR, side * (K.ZR + 0.0375)), 0.12, STEEL, 0.4, member="wheel-r", part="member", **st)
    for anchor, bone, x, z in (("contact-f-l", "axle-f", 0.0, K.ZF), ("contact-f-r", "axle-f", 0.0, -K.ZF), ("contact-r-l", "axle-r", -K.WB, K.ZR), ("contact-r-r", "axle-r", -K.WB, -K.ZR)):
        S.box(f"pad-{anchor[8:]}", bone, (x - 0.0375, x + 0.0375), (0.0, 0.075), (z - 0.075, z + 0.075), RUBBER, 0.2, member=bone, part="support", anchor=anchor, **st)
    S.ref("m-tilt-barrel", "tilt-barrel", "tilt-cyl", "member")
    S.ref("m-tilt-rod", "tilt-rod", "tilt-rod", "member")
    S.ref("m-outer-rail", "outer-rail", "mast-outer", "member")
    S.ref("m-inner-rail", "inner-rail", "mast-inner", "member")
    S.ref("m-inner-head", "inner-head", "mast-inner", "member")
    S.ref("m-carriage", "carriage", "carriage", "member")
    S.ref("m-tine-l", "tine-l", "forks", "member")
    S.ref("m-tine-r", "tine-r", "forks", "member")
    S.box("pallet-base", "pallet-x", (K.PALLET_X, K.PALLET_X + 1.2), (0.0, 0.075), (-0.4, -0.3), WOOD, 0.55, member="pallet-x", part="member", **st)
    S.box("pallet-column", "pallet-y", (K.PALLET_X + 0.5275, K.PALLET_X + 0.6725), (0.0, 0.144), (-0.0725, 0.0725), WOOD, 0.55, member="pallet-y", part="member", **st)
    S.box("pallet-stringer", "pallet", (K.PALLET_X, K.PALLET_X + 1.2), (0.069, 0.144), (-0.0375, 0.0375), WOOD, 0.55, member="pallet", part="member", **st)
    return S


DECISIONS = [
    text("1:500 draws and models the EFG 425 as a few masses at 75 mm while keeping every real proportion: the frame sides are 75 mm side-profile plates with the front wheel arch, closed by the hood, the floor pan and the kick panel around the battery (a hidden thermal solid), fenders and the under-floor frame are blocks, cowl, seat and backrest are blocks; the guard is four 75 mm posts under a 75 mm roof slab in place of the roof grid; the counterweight is its loft over one lower casting with the rear wheel arch and the axle housing; tyres are discs; the mast is two 75 mm rails per stage and the inner head, the carriage is one plate and the barred load backrest a 75 mm frame with its centre bar, the forks 75 mm L profiles, the pallet one block under the goods.",
         "1:500 zeichnet und modelliert den EFG 425 als wenige Massen bei 75 mm und behält alle realen Proportionen: die Rahmenseiten sind 75 mm dicke Seitenprofilbleche mit dem vorderen Radlauf, geschlossen von Haube, Bodenwanne und Fußblech um die Batterie (ein verdeckter Wärmekörper), Kotflügel und Unterbodenrahmen sind Blöcke, Armaturenhaube, Sitz und Lehne sind Blöcke; das Schutzdach sind vier 75-mm-Pfosten unter einer 75-mm-Dachplatte anstelle des Dachgitters; das Gegengewicht ist sein Loft über einem unteren Gussteil mit dem hinteren Radlauf und dem Achsgehäuse; Reifen sind Scheiben; der Mast sind zwei 75-mm-Schienen je Stufe und der Innenmastkopf, der Gabelträger ist eine Platte und das vergitterte Lastschutzgitter ein 75-mm-Rahmen mit Mittelstab, die Gabeln 75-mm-L-Profile, die Palette ein Block unter dem Ladegut."),
    text("Thickened to the 75 mm feature: guard posts (60 mm), fork blades (40 mm, so the blade top lies at 75 mm instead of 60 mm), tilt rods (45 mm), rails (64 to 70 mm wide). Dropped: steering wheel and column, armrest, lamps, coupling, chains, lift cylinders, pallet openings, straps. Structure: frame side plates (they are the frame members at this scale), floor pan and under-floor frame, guard posts with the two roof rails, the counterweight beam that carries the steer axle, axles, wheel hubs, tilt cylinders, rails and inner head, carriage, forks, the pallet base, block and centre stringer (the load path, not the masses of the body); four 75 mm contact pads and three masses.",
         "Auf das Kleinstmaß 75 mm verdickt: Schutzdachpfosten (60 mm), Gabelblätter (40 mm, die Blattoberkante liegt daher bei 75 statt 60 mm), Kolbenstangen der Neigezylinder (45 mm), Schienen (64 bis 70 mm breit). Weggelassen: Lenkrad und Säule, Armlehne, Leuchten, Kupplung, Ketten, Hubzylinder, Palettenöffnungen, Umreifung. Tragwerk: Rahmenseitenbleche (bei diesem Maßstab sind sie die Rahmenträger), Bodenwanne und Unterbodenrahmen, Schutzdachpfosten mit den zwei Dachlängsträgern, der Gegengewichtsträger, der die Lenkachse trägt, Achsen, Radnaben, Neigezylinder, Schienen und Innenmastkopf, Gabelträger, Gabeln, Palettenfuß, -klotz und Mittellängsbrett (der Kraftfluss, nicht die Massen des Körpers); vier 75-mm-Aufstandsplatten und drei Massen."),
]
