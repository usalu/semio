from author_11_lib import *

P = Ctx("plate", "abs-plastic", None, "body", 0.88)
RG = Ctx("ring", "abs-plastic", None, "body", 0.7)
RT = Ctx("rotor", "abs-plastic", None, "body", 0.82)
HUB = Ctx("rotor", "motor-housing", None, "body", 0.55)
steel = Ctx("plate", "abs-plastic", None, "detail", 0.25)
BORE = 0.088
SH = {s: asset_shapes("duct-fan", "elevation", s) for s in ("50", "100", "200")}


def reg(s, r):
    s["region"] = r
    return s


def blades(scale, ctx, z0, z1):
    return [ext("blade-%d" % i, ctx, path_points(SH[scale]["blade-%d" % i]["d"])[0], "xy", z0, z1) for i in range(1, 4 if scale == "200" else 6)]


def hub(sid, ctx, z0, z1, r, dome=True):
    L = z1 - z0
    pr = [[0, 0], [0, r * 0.7], [0.003, r * 0.93], [0.006, r], [L, r], [L, 0]] if dome else [[0, 0], [0, r], [L, r], [L, 0]]
    return rev(sid, ctx, pr, "w", origin=(0, 0, z0))


def struts(sid, ctx, r, z, r0=0.03, r1=0.09):
    return cap(sid, ctx, (r0, 0, z), (r1, 0, z), r, repeat={"count": 4, "rotationAxis": [0, 0, 1], "rotationRad": math.pi / 2})


def air(r, z0, z1):
    return tag(lf("th-airstream", RG(layer="shell"), "w", [(z0, r, r, 2), (z1, r, r, 2)], cs=False, ce=False), tags=["extract"])


def support(aid, kind, y, sz, ln):
    c = steel(member="plate", part="support")
    s = cyl("support-" + aid, c, "w" if kind == "pin" else "u", (0, y, -0.039), ln, sz)
    return tag(s, tags=[aid])


# ------------------------------------------------------------------ 1:50
body50 = [
    ext("plate", P, rrect(-0.11, -0.11, 0.11, 0.11, 0.014), "xy", -0.045, -0.033),
    *[ell("screw-%s%s" % (a, b), P(role="detail", tone=0.45), (x, y, -0.033), (0.008, 0.008, 0.0045)) for a, x in (("l", -0.092), ("r", 0.092)) for b, y in (("t", 0.092), ("b", -0.092))],
    pipe("ring", RG, "w", (0, 0, 0.0035), 0.073, 0.1, BORE),
    pipe("lip", RG(tone=0.6), "w", (0, 0, 0.0412), 0.0077, 0.104, BORE),
    *blades("50", RT, -0.0285, -0.0207),
    hub("hub", HUB, -0.029, 0.022, 0.034),
    struts("strut", RG(bone="ring", tone=0.6), 0.004, 0.02),
]
th50 = [
    reg(th("th-plate", "plate", "casing", "shell"), "casing"),
    reg(th("th-ring", "ring", "casing", "shell"), "casing"),
    reg(th("th-lip", "lip", "casing", "shell"), "casing"),
    *[reg(th("th-blade-%d" % i, "blade-%d" % i, "blades", "blade"), "blades") for i in range(1, 6)],
    reg(th("th-hub", "hub", "hub", "can"), "hub"),
    reg(air(0.0875, -0.004, 0.004), "casing"),
]
st50 = [
    stm("member-plate", "plate", "plate", "member"),
    stm("member-ring", "ring", "ring", "member"),
    stm("member-strut", "strut", "ring", "member"),
    cyl("member-rotor", steel(member="rotor", part="member", bone="rotor"), "w", (0, 0, -0.0035), 0.05, 0.008),
    pipe("joint-ring", steel(member="ring", part="joint", bone="ring"), "w", (0, 0, -0.0345), 0.012, 0.099, 0.089),
    cyl("joint-rotor", steel(member="rotor", part="joint", bone="rotor"), "w", (0, 0, 0.0135), 0.016, 0.012),
    ell("mass-plate", steel(member="plate", part="mass", tone=0.15), (0, 0, -0.039), (0.07, 0.07, 0.005)),
    ell("mass-rotor", steel(member="rotor", part="mass", tone=0.15, bone="rotor"), (0, 0, -0.003), (0.025, 0.025, 0.02)),
    support("wall-screw-top", "pin", 0.092, 0.008, 0.016),
    support("wall-screw-bottom", "roller", -0.092, 0.008, 0.016),
]
D50 = [
    text("Own 1:50 model of a wall-mounted duct fan, axis w with the origin at the hub centre in the wall plane: a 0.22 m rounded plate (z -0.045..-0.033) that closes the unit at the wall, four domed screw heads, the 0.2 m housing ring (z -0.033..0.041) with the 0.208 m lip at the rear edge, the five drawn blade outlines as 7.8 mm flat vanes (z -0.0285..-0.0207), a domed 68 mm motor hub 51 mm long and four 8 mm motor struts at z 0.02 that join hub and ring. The drawn dark opening is the closed wall plate seen through the open ring.",
         "Eigenes 1:50-Modell eines wandmontierten Kanalventilators, Achse w mit dem Ursprung in der Nabenmitte in der Wandebene: eine gerundete Platte 0,22 m (z -0,045..-0,033), die die Einheit an der Wand schliesst, vier gewölbte Schraubenköpfe, der 0,2 m Gehäusering (z -0,033..0,041) mit der 0,208 m Lippe am hinteren Rand, die fünf gezeichneten Schaufelumrisse als 10-mm-Flachflügel (z -0,0285..-0,0185), eine gewölbte Motornabe von 68 mm und 51 mm Länge und vier 8-mm-Motorstreben bei z 0,02, die Nabe und Ring verbinden. Die gezeichnete dunkle Öffnung ist die durch den offenen Ring gesehene geschlossene Wandplatte."),
    text("Parts below 7.5 mm (seam bead, hub cap) are merged; the lip is 7.7 mm deep. The wall-screw anchors were given z -0.039 at the root (middle of the plate). Thermal (area = exposed surface): casing = plate, ring, lip and a 0.175 m air ring at the outlet anchor (tag extract), blades and hub by reference; areaM2 was corrected at the root (see report). Structure: plate, ring and struts are members, a 16 mm shaft is the rotor member, flange collar and bearing are the joints, mass ellipsoids sit in plate and rotor, the screws are a cross pin (top) and a roller bar (bottom).",
         "Teile unter 7,5 mm (Fugenwulst, Nabenkappe) sind zusammengefasst; die Lippe ist 7,7 mm tief. Die Wandschrauben-Anker erhielten an der Wurzel z -0,039 (Plattenmitte). Thermik (Fläche = freie Oberfläche): Gehäuse = Platte, Ring, Lippe und ein Luftring von 0,175 m am Auslassanker (Markierung extract), Schaufeln und Nabe als Verweis; areaM2 wurde an der Wurzel korrigiert (siehe Bericht). Tragwerk: Platte, Ring und Streben sind Stäbe, eine 16-mm-Welle ist der Rotorstab, Flanschkragen und Lager sind die Gelenke, Massenellipsoide sitzen in Platte und Rotor, die Schrauben sind ein Querbolzen (oben) und eine Rollenstange (unten)."),
]

# ------------------------------------------------------------------ 1:100
body100 = [
    ext("plate", P, rrect(-0.11, -0.11, 0.11, 0.11, 0.014), "xy", -0.0452, -0.03),
    pipe("ring", RG, "w", (0, 0, 0.007), 0.076, 0.1, 0.086),
    *blades("100", RT, -0.0285, -0.0132),
    hub("hub", HUB, -0.029, 0.022, 0.036, dome=False),
    struts("strut", RG(bone="ring", tone=0.6), 0.0078, 0.02, 0.036, 0.09),
]
th100 = [
    reg(th("th-plate", "plate", "casing", "shell"), "casing"),
    reg(th("th-ring", "ring", "casing", "shell"), "casing"),
    *[reg(th("th-blade-%d" % i, "blade-%d" % i, "blades", "blade"), "blades") for i in range(1, 6)],
    reg(th("th-hub", "hub", "hub", "can"), "hub"),
    reg(air(0.087, -0.008, 0.008), "casing"),
]
st100 = [
    stm("member-plate", "plate", "plate", "member"),
    stm("member-ring", "ring", "ring", "member"),
    stm("member-strut", "strut", "ring", "member"),
    cyl("member-rotor", steel(member="rotor", part="member", bone="rotor"), "w", (0, 0, -0.0035), 0.05, 0.0085),
    pipe("joint-ring", steel(member="ring", part="joint", bone="ring"), "w", (0, 0, -0.0375), 0.015, 0.099, 0.089),
    ell("mass-plate", steel(member="plate", part="mass", tone=0.15), (0, 0, -0.039), (0.07, 0.07, 0.0085)),
    support("wall-screw-top", "pin", 0.092, 0.008, 0.0165),
    support("wall-screw-bottom", "roller", -0.092, 0.008, 0.0165),
]
D100 = [
    text("Own 1:100 model against the 1:100 shapes: plate of 15 mm, ring tube without the lip and seam, five 15 mm blades of the 1:100 outlines, a plain 72 mm hub and four 15.6 mm struts; screws, lip, hub dome and cap are merged away.",
         "Eigenes 1:100-Modell gegen die 1:100-Formen: Platte von 15 mm, Ringrohr ohne Lippe und Fuge, fünf 15-mm-Schaufeln nach den 1:100-Umrissen, eine schlichte Nabe von 72 mm und vier Streben von 15,6 mm; Schrauben, Lippe, Nabenkuppe und Kappe sind zusammengefasst."),
    text("Thermal and structure are rebuilt on these solids by reference (plate, ring, blades, hub) with a shorter air ring; plate, ring, struts and a 17 mm rotor shaft are the members, one flange collar is the joint, one mass ellipsoid sits in the plate and the two wall screws are 16 mm supports.",
         "Thermik und Tragwerk sind auf diesen Körpern als Verweis neu aufgebaut (Platte, Ring, Schaufeln, Nabe) mit einem längeren Luftring; Platte, Ring, Streben und eine 17-mm-Rotorwelle sind die Stäbe, ein Flanschkragen ist das Gelenk, ein Massenellipsoid sitzt in der Platte und die zwei Wandschrauben sind 16-mm-Auflager."),
]

# ------------------------------------------------------------------ 1:200
body200 = [
    ext("plate", P, rrect(-0.11, -0.11, 0.11, 0.11, 0.016), "xy", -0.045, -0.0145),
    pipe("ring", RG, "w", (0, 0, 0.0075), 0.075, 0.1, 0.096),
    *blades("200", RT, -0.0144, 0.0162),
    hub("hub", HUB, -0.0144, 0.0356, 0.044, dome=False),
]
th200 = [
    reg(th("th-plate", "plate", "casing", "shell"), "casing"),
    reg(th("th-ring", "ring", "casing", "shell"), "casing"),
    *[reg(th("th-blade-%d" % i, "blade-%d" % i, "blades", "blade"), "blades") for i in range(1, 4)],
    reg(th("th-hub", "hub", "hub", "can"), "hub"),
    reg(air(0.087, -0.0153, 0.0153), "casing"),
]
st200 = [
    stm("member-plate", "plate", "plate", "member"),
    stm("member-ring", "ring", "ring", "member"),
    cyl("member-rotor", steel(member="rotor", part="member", bone="rotor"), "w", (0, 0, 0.0106), 0.05, 0.0155),
    ell("mass-plate", steel(member="plate", part="mass", tone=0.15), (0, 0, -0.03), (0.07, 0.07, 0.0155)),
    support("wall-screw-top", "pin", 0.092, 0.0155, 0.031),
    support("wall-screw-bottom", "roller", -0.092, 0.0155, 0.031),
]
D200 = [
    text("Own 1:200 model: a 30 mm plate, one ring tube (bore 0.192 m), three 30 mm blades of the 1:200 outlines and an 88 mm hub; the motor struts and all small parts vanish into these masses.",
         "Eigenes 1:200-Modell: eine Platte von 30 mm, ein Ringrohr (Bohrung 0,192 m), drei 30-mm-Schaufeln nach den 1:200-Umrissen und eine Nabe von 88 mm; die Motorstreben und alle Kleinteile gehen in diesen Massen auf."),
    text("Thermal and structure at 1:200 by reference to these masses with a 30 mm air ring; members are plate, ring and a 31 mm rotor shaft, one mass ellipsoid and the two wall screws at 31 mm; joints and struts are merged into the members.",
         "Thermik und Tragwerk bei 1:200 als Verweis auf diese Massen mit einem 30-mm-Luftring; Stäbe sind Platte, Ring und eine 31-mm-Rotorwelle, ein Massenellipsoid und die zwei Wandschrauben in 31 mm; Gelenke und Streben sind in den Stäben aufgegangen."),
]
write("duct-fan", {
    50: dict(body=body50, thermal=th50, structure=st50, decisions=D50),
    100: dict(body=body100, thermal=th100, structure=st100, decisions=D100),
    200: dict(body=body200, thermal=th200, structure=st200, decisions=D200),
})
