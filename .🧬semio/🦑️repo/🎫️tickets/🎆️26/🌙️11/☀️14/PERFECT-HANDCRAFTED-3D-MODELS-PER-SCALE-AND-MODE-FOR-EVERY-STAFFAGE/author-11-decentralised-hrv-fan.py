from author_11_lib import *

P = Ctx("plate", "abs-plastic", None, "body", 0.88)
RG = Ctx("ring", "abs-plastic", None, "body", 0.7)
CO = Ctx("core", "ceramic-honeycomb", None, "body", 0.62)
RT = Ctx("rotor", "abs-plastic", None, "body", 0.82)
HUB = Ctx("rotor", "motor-housing", None, "body", 0.55)
steel = Ctx("plate", "abs-plastic", None, "detail", 0.25)
BORE = 0.088
SH = {s: {} for s in ("50", "100", "200")}
for s in SH:
    for v in ("elevation",):
        SH[s] = asset_shapes("decentralised-hrv-fan", v, s)


def reg(s, r):
    s["region"] = r
    return s


def cells(scale, R):
    out = []
    for sub in path_points(SH[scale]["core-cells"]["d"]):
        out.append((round(sum(p[0] for p in sub) / 6, 5), round(sum(p[1] for p in sub) / 6, 5)))
    return [hexagon(cx, cy, R) for cx, cy in out if math.hypot(cx, cy) + R <= 0.074]


def blades(scale, ctx, z0, z1):
    return [ext("blade-%d" % i, ctx, path_points(SH[scale]["blade-%d" % i]["d"])[0], "xy", z0, z1) for i in range(1, 4 if scale == "200" else 6)]


def hub(sid, ctx, z0, z1, r, dome=True):
    L = z1 - z0
    pr = [[0, 0], [0, r * 0.7], [0.003, r * 0.93], [0.006, r], [L, r], [L, 0]] if dome else [[0, 0], [0, r], [L, r], [L, 0]]
    return rev(sid, ctx, pr, "w", origin=(0, 0, z0))


def air(r, z0, z1, layer):
    return tag(lf("th-airstream", CO(layer=layer), "w", [(z0, r, r, 2), (z1, r, r, 2)], cs=False, ce=False), tags=["exhaust-out", "supply-in"])


def support(aid, kind, y, sz, ln):
    c = steel(member="plate", part="support")
    if kind == "pin":
        s = cyl("support-" + aid, c, "w", (0, y, -0.11), ln, sz)
    else:
        s = cyl("support-" + aid, c, "u", (0, y, -0.11), ln, sz)
    return tag(s, tags=[aid])


TC, TB, TR, TH_ = P(layer="shell"), CO(layer="wall"), RT(layer="blade"), HUB(layer="can")

# ------------------------------------------------------------------ 1:50
body50 = [
    ext("plate", P, rrect(-0.13, -0.13, 0.13, 0.13, 0.02), "xy", -0.12, -0.10),
    *[ell("screw-%s%s" % (a, b), P(role="detail", tone=0.45), (x, y, -0.1), (0.007, 0.007, 0.0045)) for a, x in (("l", -0.108), ("r", 0.108)) for b, y in (("t", 0.108), ("b", -0.108))],
    pipe("ring", RG, "w", (0, 0, 0.0025), 0.205, 0.098, BORE),
    pipe("rim", RG(tone=0.6), "w", (0, 0, 0.1125), 0.015, 0.104, BORE),
    pipe("core-frame", CO(tone=0.4), "w", (0, 0, -0.045), 0.05, 0.088, 0.08),
    ext("core", CO, ngon(0.08, 48), "xy", -0.07, -0.02, holes=cells("50", 0.0098)),
    *blades("50", RT, -0.0975, -0.0865),
    hub("hub", HUB, -0.098, -0.07, 0.028),
]
th50 = [
    reg(th("th-plate", "plate", "casing", "shell"), "casing"),
    reg(th("th-ring", "ring", "casing", "shell"), "casing"),
    reg(th("th-rim", "rim", "casing", "shell"), "casing"),
    reg(th("th-core", "core", "core", "wall"), "core"),
    *[reg(th("th-blade-%d" % i, "blade-%d" % i, "blades", "blade"), "blades") for i in range(1, 6)],
    reg(th("th-hub", "hub", "hub", "can"), "hub"),
    reg(air(0.08, -0.02, 0.02, "wall"), "core"),
]
st50 = [
    stm("member-plate", "plate", "plate", "member"),
    stm("member-ring", "ring", "ring", "member"),
    stm("member-core", "core-frame", "core", "member"),
    cyl("member-rotor", steel(member="rotor", part="member", bone="rotor"), "w", (0, 0, -0.0825), 0.025, 0.008),
    pipe("joint-ring", steel(member="ring", part="joint", bone="ring"), "w", (0, 0, -0.0925), 0.015, 0.1, 0.09),
    cyl("joint-rotor", steel(member="rotor", part="joint", bone="rotor"), "w", (0, 0, -0.0775), 0.015, 0.012),
    ell("mass-core", steel(member="core", part="mass", tone=0.15, bone="core"), (0, 0, -0.045), (0.06, 0.06, 0.02)),
    ell("mass-ring", steel(member="ring", part="mass", tone=0.15, bone="ring"), (0, 0, 0.04), (0.08, 0.08, 0.04)),
    support("wall-screw-top", "pin", 0.108, 0.007, 0.02),
    support("wall-screw-bottom", "roller", -0.108, 0.007, 0.014),
]
D50 = [
    text("Own 1:50 model of a wall-mounted heat-recovery fan, axis w: a 0.26 m rounded plate (z -0.12..-0.10) that is also the rear wall of the unit, four domed screw heads on its face, a 0.196 m ring tube of 0.205 m length with a 0.208 m front rim, a core cassette frame, the 50 mm ceramic honeycomb core with the drawn hexagonal cells as through holes (78 % of the drawn cell, 8 mm walls; the cells that the disc edge would cut are omitted), the five drawn impeller blades (11 mm flat vanes) and a domed 56 mm hub on the revolute rotor bone. The drawn dark opening is the rear wall seen through the open core.",
         "Eigenes 1:50-Modell eines wandmontierten Wärmerückgewinnungs-Lüfters, Achse w: eine gerundete Platte von 0,26 m (z -0,12..-0,10), die zugleich die Rückwand der Einheit ist, vier gewölbte Schraubenköpfe auf ihrer Fläche, ein Ringrohr von 0,196 m Durchmesser und 0,205 m Länge mit Frontkranz von 0,208 m, ein Kernrahmen, der 50 mm dicke Keramikwabenkern mit den gezeichneten sechseckigen Zellen als Durchgangslöchern (78 % der gezeichneten Zelle, 8 mm Stege; die vom Scheibenrand angeschnittenen Zellen entfallen), die fünf gezeichneten Laufradschaufeln (11 mm flache Flügel) und eine gewölbte Nabe von 56 mm am Drehknochen Rotor. Die gezeichnete dunkle Öffnung ist die durch den offenen Kern gesehene Rückwand."),
    text("The depth order is not in any 2D view and was decided from the painting order: impeller nearest the plate, core 50 mm behind it. Rim-to-wall joint line, wall gasket and hub cap are below 7.5 mm and merged. The wall-screw anchors were given z -0.11 at the root (the middle of the plate) so that the supports sit at their anchors.",
         "Die Tiefenfolge steht in keiner 2D-Ansicht und wurde aus der Malreihenfolge entschieden: Laufrad nahe der Platte, Kern 50 mm dahinter. Fugenlinie, Wanddichtung und Nabenkappe liegen unter 7,5 mm und sind zusammengefasst. Die Wandschrauben-Anker erhielten an der Wurzel z -0,11 (Plattenmitte), damit die Auflager an ihren Ankern sitzen."),
    text("Thermal: plate, ring and rim form the casing, the honeycomb with its cell walls is the core region, the five blades and the hub are their own regions, and one 0.16 m air tube through the core carries the tags exhaust-out and supply-in. Areas are exposed surfaces (buried faces and the wall contact do not count); areaM2 was corrected at the root (casing 0.09 to 0.39, core 0.9 to 0.075, blades 0.012 to 0.028, hub 0.006 to 0.0075 m2; see report). Single-layer regions are surfaces. Structure: plate, ring and core frame are the plate, ring and core members, a 16 mm shaft is the rotor member, the flange collar and the 24 mm bearing are the joints, mass ellipsoids sit in core and ring, and the two wall screws are a cross pin at the top and a roller bar at the bottom.",
         "Thermik: Platte, Ring und Kranz bilden das Gehäuse, die Wabe mit ihren Zellwänden ist die Kernregion, die fünf Schaufeln und die Nabe sind eigene Regionen, und ein Luftrohr von 0,16 m durch den Kern trägt die Markierungen exhaust-out und supply-in. Flächen sind freie Oberflächen (verdeckte Flächen und der Wandkontakt zählen nicht); areaM2 wurde an der Wurzel korrigiert (Gehäuse 0,09 auf 0,39, Kern 0,9 auf 0,075, Schaufeln 0,012 auf 0,028, Nabe 0,006 auf 0,0075 m2; siehe Bericht). Einlagige Regionen sind Flächen. Tragwerk: Platte, Ring und Kernrahmen sind Platten-, Ring- und Kernstab, eine 16-mm-Welle ist der Rotorstab, Flanschkragen und das 24-mm-Lager sind die Gelenke, Massenellipsoide sitzen in Kern und Ring, und die zwei Wandschrauben sind ein Querbolzen oben und eine Rollenstange unten."),
]

# ------------------------------------------------------------------ 1:100
body100 = [
    ext("plate", P, rrect(-0.13, -0.13, 0.13, 0.13, 0.02), "xy", -0.12, -0.10),
    pipe("ring", RG, "w", (0, 0, 0.0), 0.2, 0.098, 0.086),
    pipe("rim", RG(tone=0.6), "w", (0, 0, 0.11), 0.02, 0.104, 0.086),
    ext("core", CO, ngon(0.08, 40), "xy", -0.07, -0.02, holes=cells("100", 0.017)),
    *blades("100", RT, -0.0975, -0.0822),
    hub("hub", HUB, -0.098, -0.07, 0.03, dome=False),
]
th100 = [
    reg(th("th-plate", "plate", "casing", "shell"), "casing"),
    reg(th("th-ring", "ring", "casing", "shell"), "casing"),
    reg(th("th-rim", "rim", "casing", "shell"), "casing"),
    reg(th("th-core", "core", "core", "wall"), "core"),
    *[reg(th("th-blade-%d" % i, "blade-%d" % i, "blades", "blade"), "blades") for i in range(1, 6)],
    reg(th("th-hub", "hub", "hub", "can"), "hub"),
    reg(air(0.08, -0.03, 0.03, "wall"), "core"),
]
st100 = [
    stm("member-plate", "plate", "plate", "member"),
    stm("member-ring", "ring", "ring", "member"),
    stm("member-core", "core", "core", "member"),
    cyl("member-rotor", steel(member="rotor", part="member", bone="rotor"), "w", (0, 0, -0.0825), 0.03, 0.0095),
    pipe("joint-ring", steel(member="ring", part="joint", bone="ring"), "w", (0, 0, -0.09), 0.03, 0.1, 0.09),
    cyl("joint-rotor", steel(member="rotor", part="joint", bone="rotor"), "w", (0, 0, -0.0775), 0.015, 0.0125),
    ell("mass-core", steel(member="core", part="mass", tone=0.15, bone="core"), (0, 0, -0.045), (0.06, 0.06, 0.02)),
    support("wall-screw-top", "pin", 0.108, 0.008, 0.02),
    support("wall-screw-bottom", "roller", -0.108, 0.008, 0.02),
]
D100 = [
    text("Own 1:100 model drawn against the 1:100 shapes: the plate with its 20 mm depth, ring and rim as two tubes (rim 20 mm deep), the core as a plain 50 mm disc with the seven drawn cells (21 mm across flats) as holes, five flat 15 mm blades of the 1:100 outlines and a 60 mm hub; screws, core frame, domed hub and the fine seams are merged away.",
         "Eigenes 1:100-Modell gegen die 1:100-Formen: die Platte mit 20 mm Tiefe, Ring und Kranz als zwei Rohre (Kranz 20 mm tief), der Kern als einfache 50-mm-Scheibe mit den sieben gezeichneten Zellen (21 mm Schlüsselweite) als Löchern, fünf flache 15-mm-Schaufeln nach den 1:100-Umrissen und eine Nabe von 60 mm; Schrauben, Kernrahmen, gewölbte Nabe und die feinen Fugen sind zusammengefasst."),
    text("Thermal and structure are rebuilt: casing tubes, core, blades, hub and air tube; plate, ring and core members, a 19 mm rotor shaft, one flange collar and the bearing as joints, one mass ellipsoid, the two wall supports at 15 mm.",
         "Thermik und Tragwerk sind neu aufgebaut: Gehäuserohre, Kern, Schaufeln, Nabe und Luftrohr; Platten-, Ring- und Kernstab, eine 19-mm-Rotorwelle, ein Flanschkragen und das Lager als Gelenke, ein Massenellipsoid, die zwei Wandauflager in 15 mm."),
]

# ------------------------------------------------------------------ 1:200
body200 = [
    ext("plate", P, rrect(-0.13, -0.13, 0.13, 0.13, 0.02), "xy", -0.12, -0.0895),
    pipe("ring", RG, "w", (0, 0, 0.01), 0.22, 0.098, 0.092),
    ext("core", CO, ngon(0.082, 32), "xy", -0.0505, -0.02),
    *blades("200", RT, -0.0975, -0.067),
    hub("hub", HUB, -0.0975, -0.067, 0.04, dome=False),
]
th200 = [
    reg(th("th-plate", "plate", "casing", "shell"), "casing"),
    reg(th("th-ring", "ring", "casing", "shell"), "casing"),
    reg(th("th-core", "core", "core", "wall"), "core"),
    *[reg(th("th-blade-%d" % i, "blade-%d" % i, "blades", "blade"), "blades") for i in range(1, 4)],
    reg(th("th-hub", "hub", "hub", "can"), "hub"),
    reg(air(0.08, -0.035, 0.045, "wall"), "core"),
]
st200 = [
    stm("member-plate", "plate", "plate", "member"),
    stm("member-ring", "ring", "ring", "member"),
    stm("member-core", "core", "core", "member"),
    cyl("member-rotor", steel(member="rotor", part="member", bone="rotor"), "w", (0, 0, -0.0825), 0.031, 0.0155),
    ell("mass-core", steel(member="core", part="mass", tone=0.15, bone="core"), (0, 0, -0.045), (0.06, 0.06, 0.02)),
    support("wall-screw-top", "pin", 0.108, 0.0155, 0.031),
    support("wall-screw-bottom", "roller", -0.108, 0.0155, 0.031),
]
D200 = [
    text("Own 1:200 model: a 30 mm plate, one ring tube that now includes the rim (r 0.098 over the whole 0.22 m, bore 0.184 m), a solid 0.164 m core disc of 30 mm depth without cells, three 30 mm blades of the 1:200 outlines and a 80 mm hub. Fewer solids and no cells, because cells, screws and rim step fall below 30 mm.",
         "Eigenes 1:200-Modell: eine Platte von 30 mm, ein Ringrohr, das den Kranz einschliesst (r 0,098 über die ganzen 0,22 m, Bohrung 0,184 m), eine volle Kernscheibe von 0,164 m und 30 mm Tiefe ohne Zellen, drei 30-mm-Schaufeln nach den 1:200-Umrissen und eine Nabe von 80 mm. Weniger Körper und keine Zellen, weil Zellen, Schrauben und Kranzstufe unter 30 mm liegen."),
    text("Thermal at 1:200 keeps the casing tubes, the core disc, three blades, the hub and the air tube; structure keeps the three static members, a 30 mm rotor shaft, one mass ellipsoid and the two supports at 30 mm; the joints are merged into the members.",
         "Die Thermik bei 1:200 behält Gehäuserohre, Kernscheibe, drei Schaufeln, Nabe und Luftrohr; das Tragwerk behält die drei festen Stäbe, eine 30-mm-Rotorwelle, ein Massenellipsoid und die zwei Auflager in 30 mm; die Gelenke sind in den Stäben aufgegangen."),
]
write("decentralised-hrv-fan", {
    50: dict(body=body50, thermal=th50, structure=st50, decisions=D50),
    100: dict(body=body100, thermal=th100, structure=st100, decisions=D100),
    200: dict(body=body200, thermal=th200, structure=st200, decisions=D200),
})
