from author_11_lib import *

BS = Ctx("base", "pp-plastic", None, "body", 0.5)
SH = Ctx("body", "stainless", None, "body", 0.4)
WA = Ctx("water", "water", None, "body", 0.45)
HD = Ctx("handle", "pp-plastic", None, "body", 0.2)
LD = Ctx("lid", "stainless", None, "body", 0.45)
steel = Ctx("body", "stainless", None, "detail", 0.25)
N = 48


def reg(s, r):
    s["region"] = r
    return s


def shell(ro, ri, v1, scale_top):
    return ext("shell", SH, ngon(ro, N), "zx", 0, v1, holes=[ngon(ri, N)], taper=(1, scale_top))


def waterrev(r0, r1):
    return rev("water", WA, [[0, r0], [0.125, r1]], "v")


def handle(z, notch=True):
    return ext("handle", HD, [[-0.028, 0.08], [0.022, 0.075], [0.027, -0.05], [-0.025, -0.075], [-0.025, -0.052], [0.01, -0.035], [0.007, 0.058], [-0.028, 0.06]], "xy", -z, z)


def tubes(r1, r2, x0, x1, y, y2=None):
    return [
        tag(reg(lf("th-steam", WA(bone="body", layer="water-skin"), "u", [(x0, r1, r1, 2, 0, y), (x1, r1, r1, 2, 0, y)], cs=False, ce=False), "water"), tags=["steam"]),
        tag(reg(lf("th-steam-plume", SH(layer="shell-skin"), "u", [(x0, r2, r2, 2, 0, y2 or y), (x1, r2, r2, 2, 0, y2 or y)], cs=False, ce=False), "shell"), tags=["steam-plume"]),
    ]


def common_structure(joint_h, lug, ps):
    c = steel
    return [
        stm("member-base", "base", "base", "member"),
        stm("member-body", "shell", "body", "member"),
        stm("member-handle", "handle", "handle", "member"),
        stm("member-water", "water", "water", "member"),
        stm("member-lid", "lid", "lid", "member"),
    ]


def pad(r, h):
    return tag(cyl("support-base-contact", steel(member="base", part="support", bone="base"), "v", (0, h / 2, 0), h, r), tags=["base-contact"])


def mass_water(r):
    return ell("mass-water", steel(member="water", part="mass", tone=0.15, bone="water"), (0, 0.0625, 0), (r, r, r))


def mass_base(rx, ry):
    return ell("mass-base", steel(member="base", part="mass", tone=0.15, bone="base"), (0, 0.015, 0), (rx, ry, rx))


TB, TS_, TW, TL, TH_ = BS(layer="plate-skin"), SH(layer="shell-skin"), WA(layer="water-skin"), LD(layer="lid-skin"), HD(layer="handle-skin")

# ------------------------------------------------------------------ 1:50
rw = lambda v: 0.075 - 0.005 * v / 0.175
body50 = [
    rev("base", BS, [[0, 0.086], [0.004, 0.09], [0.026, 0.09], [0.03, 0.086]], "v"),
    shell(0.075, 0.073, 0.175, 70 / 75),
    cyl("floor", SH(tone=0.3), "v", (0, 0.00525, 0), 0.0105, 0.0726),
    lf("spout", SH(tone=0.4), "u", [(-0.055, 0.027, 0.0235, 2.5, 0, 0.1575), (-0.09, 0.0185, 0.0099, 2.5, 0, 0.1725), (-0.116, 0.012, 0.003, 2.5, 0, 0.19)]),
    lf("gauge", WA(bone="body", role="glazing", tone=0.7), "v", [(0.02, 0.002, 0.014, 6, -(rw(0.02) - 0.001), 0), (0.14, 0.002, 0.014, 6, -(rw(0.14) - 0.001), 0)]),
    waterrev(0.0733, 0.0695),
    handle(0.016),
    rev("lid", LD, [[0, 0.066], [0.013, 0.066], [0.016, 0.063]], "v"),
    rev("lid-button", LD(material="pp-plastic", tone=0.2), [[0, 0.012], [0.013, 0.012], [0.016, 0.009]], "v", origin=(0, 0.016, 0)),
]
th50 = [
    th("th-base", "base", "plate", "plate-skin"),
    th("th-shell", "shell", "shell", "shell-skin"),
    th("th-spout", "spout", "shell", "shell-skin"),
    th("th-water", "water", "water", "water-skin"),
    th("th-lid", "lid", "lid", "lid-skin"),
    th("th-lid-button", "lid-button", "lid", "lid-skin"),
    th("th-handle", "handle", "handle", "handle-skin"),
    *tubes(0.015, 0.03, -0.1115, -0.1195, 0.1925, 0.1845),
]
st50 = [
    *common_structure(0, 0, 0),
    pipe("joint-lid", steel(member="lid", part="joint"), "v", (0, 0.1712, 0), 0.0075, 0.069, 0.0625, n=N),
    box("joint-handle", steel(member="handle", part="joint", bone="handle"), (-0.03, 0.055, -0.016), (-0.018, 0.085, 0.016)),
    pipe("joint-base", steel(member="body", part="joint"), "v", (0, 0.00375, 0), 0.0075, 0.075, 0.07, n=N),
    mass_water(0.05),
    mass_base(0.07, 0.012),
    pad(0.085, 0.0075),
]
D50 = [
    text("Own 1:50 model of a 1.2 kg kettle from the 1:50 elevation, front and plan: a chamfered 0.18 m heating base, a hollow tapered jug shell (0.15 m to 0.14 m outer diameter, 2 mm wall) closed by a 10.5 mm floor disc, the water as its own solid that lies against the inner wall (so it is buried laterally and shows only its free surface), a three-section spout loft from the plan wedge and the elevation triangle, the C-shaped handle outline extruded 0.032 m, a 3 mm water-gauge strip on the wall (glazing) and a prismatic lid with button on the lid bone.",
         "Eigenes 1:50-Modell eines 1,2-kg-Wasserkochers nach Ansicht, Aufriss und Grundriss von 1:50: ein gefaster Heizsockel von 0,18 m, ein hohler konischer Krugmantel (Aussendurchmesser 0,15 m auf 0,14 m, 2 mm Wand) mit einer Bodenscheibe von 10,5 mm, das Wasser als eigener Körper, der an der Innenwand anliegt (seitlich verdeckt, nur die freie Fläche zeigt), ein Tüllenloft aus drei Schnitten nach Grundrisskeil und Aufrissdreieck, die C-förmige Griffkontur 0,032 m extrudiert, ein 3 mm Wasserstandsstreifen an der Wand (Glas) und ein verschiebbarer Deckel mit Knopf am Deckelknochen."),
    text("The 0.7 mm shoulder band is below the 7.5 mm feature and merged. Thermal (area = exposed surface, buried faces and the table contact do not count): plate = base, shell = jug and spout, water = its free surface, lid = lid and button, handle = handle, all by reference; the two steam emitters are 0.03 m (steam) and 0.06 m (steam-plume) rings at the spout tip (tags steam, steam-plume). areaM2 was corrected at the root where the old estimate disagreed with the model (see report).",
         "Die 0,7-mm-Schulterlinie liegt unter dem 7,5-mm-Merkmal und ist zusammengefasst. Thermik (Fläche = freie Oberfläche, verdeckte Flächen und der Tischkontakt zählen nicht): Platte = Sockel, Mantel = Krug und Tülle, Wasser = seine freie Fläche, Deckel = Deckel und Knopf, Griff = Griff, alles als Verweis; die zwei Dampfemitter sind Ringe von 0,03 m (steam) und 0,06 m (steam-plume) an der Tüllenspitze (Markierungen steam, steam-plume). areaM2 wurde an der Wurzel korrigiert, wo die alte Schätzung dem Modell widersprach (siehe Bericht)."),
    text("Structure: the five bones are members (sockel, jug shell, handle, water column, lid), the lid seat ring, two handle lugs and the base ring are the joints, a water and a base mass ellipsoid carry the 1 kg of water and the heating plate, and a 0.17 m contact disc under the base is the support.",
         "Tragwerk: die fünf Knochen sind Stäbe (Sockel, Krugmantel, Griff, Wassersäule, Deckel), Deckelsitzring, zwei Grifflaschen und Sockelring sind die Gelenke, je ein Massenellipsoid für das Wasser und die Heizplatte tragen das 1 kg Wasser und den Sockel, und eine Kontaktscheibe von 0,17 m unter dem Sockel ist das Auflager."),
]

# ------------------------------------------------------------------ 1:100
body100 = [
    rev("base", BS, [[0, 0.086], [0.004, 0.09], [0.026, 0.09], [0.03, 0.086]], "v"),
    shell(0.075, 0.069, 0.175, 70 / 75),
    cyl("floor", SH(tone=0.3), "v", (0, 0.00775, 0), 0.0155, 0.0686),
    lf("spout", SH(tone=0.4), "u", [(-0.055, 0.027, 0.0235, 2.5, 0, 0.1575), (-0.116, 0.014, 0.0075, 2.5, 0, 0.19)]),
    waterrev(0.069, 0.0655),
    handle(0.016),
    rev("lid", LD, [[0, 0.066], [0.027, 0.066], [0.03, 0.063]], "v"),
]
th100 = [
    th("th-base", "base", "plate", "plate-skin"),
    th("th-shell", "shell", "shell", "shell-skin"),
    th("th-spout", "spout", "shell", "shell-skin"),
    th("th-water", "water", "water", "water-skin"),
    th("th-lid", "lid", "lid", "lid-skin"),
    th("th-handle", "handle", "handle", "handle-skin"),
    *tubes(0.0155, 0.031, -0.1085, -0.1235, 0.1925, 0.1845),
]
st100 = [
    *common_structure(0, 0, 0),
    pipe("joint-lid", steel(member="lid", part="joint"), "v", (0, 0.1675, 0), 0.0153, 0.067, 0.0625, n=N),
    box("joint-handle", steel(member="handle", part="joint", bone="handle"), (-0.03, 0.055, -0.016), (-0.015, 0.085, 0.016)),
    mass_water(0.05),
    pad(0.085, 0.0152),
]
D100 = [
    text("Own 1:100 model against the 1:100 shapes: the jug shell has a 6 mm wall and a 15.5 mm floor disc, the spout is a two-section loft, the lid and the button are one 30 mm lid, the water-gauge strip is dropped, the handle keeps its outline; the water lies against the inner wall as at 1:50.",
         "Eigenes 1:100-Modell gegen die 1:100-Formen: der Krugmantel hat 6 mm Wand und eine Bodenscheibe von 15,5 mm, die Tülle ist ein Loft aus zwei Schnitten, Deckel und Knopf sind ein 30-mm-Deckel, der Wasserstandsstreifen entfällt, der Griff behält seine Kontur; das Wasser liegt wie bei 1:50 an der Innenwand."),
    text("Thermal and structure rebuilt on these solids: base, jug, spout, water, lid and handle by reference, two steam rings of 31 mm and 62 mm; five members, a 15 mm lid seat ring and two handle lugs as joints, one water ellipsoid and a 15 mm contact disc.",
         "Thermik und Tragwerk auf diesen Körpern neu aufgebaut: Sockel, Krug, Tülle, Wasser, Deckel und Griff als Verweis, zwei Dampfringe von 31 mm und 62 mm; fünf Stäbe, ein 15-mm-Deckelsitzring und zwei Grifflaschen als Gelenke, ein Wasserellipsoid und eine 15-mm-Kontaktscheibe."),
]

# ------------------------------------------------------------------ 1:200
body200 = [
    rev("base", BS, [[0, 0.091], [0.004, 0.095], [0.036, 0.095], [0.04, 0.091]], "v"),
    rev("shell", SH, [[0, 0], [0, 0.078], [0.175, 0.0720], [0.175, 0.059], [0.0305, 0.0648], [0.0305, 0]], "v"),
    lf("spout", SH(tone=0.4), "u", [(-0.055, 0.03, 0.025, 2.5, 0, 0.1575), (-0.118, 0.016, 0.014, 2.5, 0, 0.1865)]),
    waterrev(0.065, 0.061),
    ext("handle", HD, [[-0.028, 0.08], [0.024, 0.075], [0.03, -0.055], [-0.025, -0.08], [-0.025, -0.045], [0.002, -0.03], [0, 0.05], [-0.028, 0.055]], "xy", -0.018, 0.018),
    rev("lid", LD, [[0, 0.07], [0.037, 0.07], [0.04, 0.067]], "v"),
]
th200 = [
    th("th-base", "base", "plate", "plate-skin"),
    th("th-shell", "shell", "shell", "shell-skin"),
    th("th-spout", "spout", "shell", "shell-skin"),
    th("th-water", "water", "water", "water-skin"),
    th("th-lid", "lid", "lid", "lid-skin"),
    th("th-handle", "handle", "handle", "handle-skin"),
    *tubes(0.0155, 0.031, -0.1135, -0.1465, 0.1925),
]
st200 = [
    *common_structure(0, 0, 0),
    mass_water(0.05),
    pad(0.085, 0.0305),
]
D200 = [
    text("Own 1:200 model: a 40 mm base cylinder, the jug shell as one cup with a 13 mm wall and a 30 mm floor, a two-section spout, the 40 mm lid, the handle from the 1:200 outline 36 mm wide and the water against the inner wall; button, gauge, chamfers and the shoulder band are merged.",
         "Eigenes 1:200-Modell: ein Sockelzylinder von 40 mm, der Krugmantel als ein Becher mit 13 mm Wand und 30 mm Boden, eine Tülle aus zwei Schnitten, der 40-mm-Deckel, der Griff nach der 1:200-Kontur 36 mm breit und das Wasser an der Innenwand; Knopf, Glas, Fasen und Schulterlinie sind zusammengefasst."),
    text("Thermal and structure at 1:200 by reference, two steam rings (31 mm, 62 mm) 30 mm beyond the tip; the five members, one water ellipsoid and a 30 mm contact disc; joints and the base mass are merged into the members.",
         "Thermik und Tragwerk bei 1:200 als Verweis, zwei Dampfringe (31 mm, 62 mm) 30 mm hinter der Spitze; die fünf Stäbe, ein Wasserellipsoid und eine 30-mm-Kontaktscheibe; Gelenke und die Sockelmasse sind in den Stäben aufgegangen."),
]
write("electric-kettle", {
    50: dict(body=body50, thermal=th50, structure=st50, decisions=D50),
    100: dict(body=body100, thermal=th100, structure=st100, decisions=D100),
    200: dict(body=body200, thermal=th200, structure=st200, decisions=D200),
})
