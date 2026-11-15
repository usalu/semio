from author_37_lib import *
import math

PT = Ctx("pot", "stainless", None, "body", 0.4)
WT = Ctx("water", "water", None, "body", 0.45)
HD = Ctx("handles", "stainless", None, "body", 0.2)
SF = Ctx("surface", "water", None, "detail", 0.9)
ST = Ctx("pot", "stainless", None, "detail", 0.15)
RO, RI, H, TB = 0.086, 0.0852, 0.1, 0.008


def circ(cy, cr, y0, n=16):
    return [[y0 + cy + cr * math.sin(2 * math.pi * k / n), cr * math.cos(2 * math.pi * k / n)] for k in range(n)]


def bead():
    pts = [[0.0965 + 0.0035 * math.sin(2 * math.pi * k / 16), 0.0825 + 0.0035 * math.cos(2 * math.pi * k / 16)] for k in range(16)]
    return pts


def handle(sign, rows, name):
    rr = [(sign * t, ra, rb, e, 0, cb) for (t, ra, rb, e, cb) in rows]
    return lf(name, HD, "u", rr)


def hrows(wide, high, e, cb=0.001):
    return [(0.083, wide, high, e, cb), (0.098, wide, high, e, cb), (0.114, wide * 0.92, high * 0.92, e, cb), (0.1225, wide * 0.6, high * 0.62, e, cb), (0.125, wide * 0.25, high * 0.3, e, cb)]


def skin(sid, region, layer, r, y0, y1, bone="pot", tone=0.4, mat="stainless"):
    c = Ctx(bone, mat, region, "body", tone)(layer=layer)
    return lf(sid, c, "v", [(y0, r, r, 2), (y1, r, r, 2)], cs=False, ce=False)


def emit(sid, y0, y1, tag):
    y0, y1 = y0 - 0.055, y1 - 0.055
    c = Ctx("surface", "water", "water", "detail", 0.9)(layer="water-skin")
    s = ext(sid, c, ngon(0.08, 24), "zx", y0, y1)
    s["sectionFaces"] = ["+u", "-u", "+v", "-v", "+w", "-w"]
    s["tags"] = [tag]
    return s


def pad(r, h):
    return tag(cyl("support-base-contact", ST(member="pot", part="support", tone=0.1), "v", (0, h / 2, 0), h, r), tags=["base-contact"])


def mass(sid, member, bone, c, rr):
    return ell(sid, ST(member=member, part="mass", bone=bone), c, rr)


# ------------------------------------------------------------ 1:50
bub = [("a", (-0.03, 0.02), 0.006), ("b", (0.025, 0.03), 0.005), ("c", (0.0, -0.02), 0.007), ("d", (0.04, -0.015), 0.005), ("e", (-0.04, -0.03), 0.005)]
body50 = [
    rev("base", PT, [[0, 0], [0, 0.0825], [0.0025, 0.086], [TB, 0.086], [TB, 0]], "v"),
    ring("wall", PT, [[0.007, RO], [0.0935, RO], [0.0935, RI], [0.007, RI]], "v"),
    ring("rim", PT, bead(), "v"),
    rev("water", WT, [[0.002, 0], [0.002, 0.0853], [0.05, 0.0853], [0.05, 0]], "v"),
    ell("surface-lens", SF(tone=0.8), (0, 0.0505, 0), (0.07, 0.0045, 0.07)),
    handle(-1, hrows(0.010, 0.007, 3), "handle-left"),
    handle(1, hrows(0.010, 0.007, 3), "handle-right"),
    ell("rivet-left", HD(tone=0.5, role="detail"), (-0.108, 0.008, 0), (0.003, 0.003, 0.003)),
    ell("rivet-right", HD(tone=0.5, role="detail"), (0.108, 0.008, 0), (0.003, 0.003, 0.003)),
]
for k, (x, z), r in bub:
    body50.append(ell("bubble-" + k, SF(tone=0.9), (x, 0.055, z), (r, 0.004, r)))
for b_ in body50:
    if b_["id"] == "surface-lens" or b_["id"].startswith("bubble"):
        b_["bone"] = "surface"
        t = b_["transform"]["translation"]
        t[1] -= 0.055
th50 = [
    th("th-base", "base", "base", "base-skin"),
    skin("th-wall", "wall", "wall-skin", 0.0861, TB, H),
    th("th-water", "water", "water", "water-skin"),
    th("th-handle-left", "handle-left", "handle", "handle-skin"),
    th("th-handle-right", "handle-right", "handle", "handle-skin"),
    th("th-rivet-left", "rivet-left", "handle", "handle-skin"),
    th("th-rivet-right", "rivet-right", "handle", "handle-skin"),
    emit("th-steam", 0.0662, 0.0738, "steam"),
    emit("th-steam-plume", 0.0738, 0.0814, "steam-plume"),
]
st50 = [
    stm("member-base", "base", "pot", "member"),
    stm("member-wall", "wall", "pot", "member"),
    stm("member-rim", "rim", "pot", "member"),
    stm("member-water", "water", "water", "member"),
    stm("member-handle-left", "handle-left", "handles", "member"),
    stm("member-handle-right", "handle-right", "handles", "member"),
    stm("member-surface", "surface-lens", "surface", "member"),
    stm("joint-rivet-left", "rivet-left", "handles", "joint"),
    stm("joint-rivet-right", "rivet-right", "handles", "joint"),
    mass("mass-pot", "pot", "pot", (0, 0.025, 0), (0.05, 0.012, 0.05)),
    mass("mass-water", "water", "water", (0, 0.025, 0), (0.06, 0.012, 0.06)),
    pad(0.075, 0.0075),
]
D50 = [
    text("Own 1:50 model of the 0.172 m stock pot against the 1:50 drawings, which cut it through its axis: a 6 mm stainless sandwich base disc with a chamfered edge (the drawn 8 mm), the 0.8 mm cylindrical wall as a hollow revolve (the stated wall thickness; the 2D draws it thickened to 6 mm), the rolled 7 mm rim bead on top that gives the drawn rim ring between 0.079 and 0.086 m, 1 L of water as a revolve that lies 0.1 mm into the wall so only its free surface shows, a flat rolling-boil lens just under the surface on the surface bone with five bubble domes of 10 to 14 mm, and two lofted riveted ear handles (20 x 14 mm, tapering to a rounded tip) with a domed rivet each.",
         "Eigenes 1:50-Modell des Kochtopfs von 0,172 m nach den 1:50-Zeichnungen, die ihn durch die Achse schneiden: eine Edelstahl-Sandwichbodenscheibe mit gefaster Kante (gezeichnet 8 mm), die 0,8-mm-Zylinderwand als hohle Drehfläche (die genannte Wandstärke; das 2D zeichnet sie auf 6 mm verdickt), der gerollte 7-mm-Randwulst oben, der den gezeichneten Randring zwischen 0,079 und 0,086 m ergibt, 1 l Wasser als Drehkörper, der 0,1 mm in die Wand reicht, sodass nur seine freie Fläche zeigt, eine flache Siedelinse knapp unter der Oberfläche am Oberflächenknochen mit fünf Blasenkuppen von 10 bis 14 mm und zwei gelofteten genieteten Ohrgriffen (20 x 14 mm, zur runden Spitze verjüngt) mit je einer Nietkuppe."),
    text("Thermal: wall = an open skin cylinder 0.1 mm outside the wall (the region's 0.054 m2 is one lateral surface used for outside and wetted inside, coupled with the water), base, water, handles and rivets by reference, steam and steam plume = two 7.5 mm discs of the emitter diameter 0.16 m at the anchor 15 mm above the surface (tags steam, steam-plume; all faces declared section faces because they are virtual outlets of a continuing vapour column). The base's contact face lies on the hob and counts no area (C1).",
         "Thermik: Wand = offene Hautzylinder 0,1 mm ausserhalb der Wand (die 0,054 m2 der Region sind eine Mantelfläche für aussen und benetzt innen, gekoppelt mit dem Wasser), Boden, Wasser, Griffe und Nieten als Verweis, Dampf und Dampffahne = zwei 7,5-mm-Scheiben mit dem Emitterdurchmesser 0,16 m am Anker 15 mm über der Oberfläche (Markierungen steam, steam-plume; alle Flächen als Schnittflächen erklärt, weil sie virtuelle Auslässe einer weiterlaufenden Dampfsäule sind). Die Kontaktfläche des Bodens liegt auf dem Kochfeld und zählt keine Fläche (C1)."),
    text("Structure: base, wall and rim bead are the members of the pot bone, the water body the water member, each ear handle a member of the handles bone with its rivet as the joint, the boil lens the (prismatic) surface member; a pot mass ellipsoid (0.54 kg) and a water mass ellipsoid (1 kg) at their centres of mass, and the 0.15 m contact disc under the base is the support of the anchor base-contact.",
         "Tragwerk: Boden, Wand und Randwulst sind die Stäbe des Topfknochens, der Wasserkörper der Wasserstab, jeder Ohrgriff ein Stab des Griffknochens mit seiner Niete als Gelenk, die Siedelinse der (prismatische) Oberflächenstab; ein Topf-Massenellipsoid (0,54 kg) und ein Wasser-Massenellipsoid (1 kg) in ihren Schwerpunkten, und die 0,15-m-Kontaktscheibe unter dem Boden ist das Auflager des Ankers base-contact."),
]

# ------------------------------------------------------------ 1:100
body100 = [
    rev("wall", PT, [[0, 0], [0, 0.086], [H, 0.086], [H, 0.080], [TB, 0.080], [TB, 0]], "v"),
    rev("water", WT, [[0.003, 0], [0.003, 0.0805], [0.05, 0.0805], [0.05, 0]], "v"),
    handle(-1, [(0.083, 0.010, 0.0078, 3, 0.001), (0.108, 0.0102, 0.0078, 3, 0.001), (0.125, 0.0045, 0.0045, 2, 0.001)], "handle-left"),
    handle(1, [(0.083, 0.010, 0.0078, 3, 0.001), (0.108, 0.0102, 0.0078, 3, 0.001), (0.125, 0.0045, 0.0045, 2, 0.001)], "handle-right"),
]
th100 = [
    skin("th-base", "base", "base-skin", 0.0861, 0, 0.015),
    skin("th-wall", "wall", "wall-skin", 0.0861, 0.015, H),
    th("th-water", "water", "water", "water-skin"),
    th("th-handle-left", "handle-left", "handle", "handle-skin"),
    th("th-handle-right", "handle-right", "handle", "handle-skin"),
    emit("th-steam", 0.0625, 0.0775, "steam"),
    emit("th-steam-plume", 0.0775, 0.0925, "steam-plume"),
]
st100 = [
    stm("member-wall", "wall", "pot", "member"),
    stm("member-water", "water", "water", "member"),
    stm("member-handle-left", "handle-left", "handles", "member"),
    stm("member-handle-right", "handle-right", "handles", "member"),
    tag(ell("member-surface", ST(member="surface", part="member", bone="surface"), (0, 0.0425 - 0.055, 0), (0.05, 0.0075, 0.05)), member="surface", part="member"),
    mass("mass-pot", "pot", "pot", (0, 0.025, 0), (0.05, 0.0125, 0.05)),
    mass("mass-water", "water", "water", (0, 0.025, 0), (0.06, 0.0125, 0.06)),
    pad(0.075, 0.015),
]
D100 = [
    text("Own 1:100 model: the base disc and the wall are one hollow cup revolve whose 6 mm wall and 8 mm floor are the drawn U (the print feature of 15 mm applies to the solid as a whole, and a thicker wall would have a cut face that the drawing does not show), the water against the inner wall, the handles as two 3-section lofts of 20 x 15.6 mm; rim bead, chamfer, boil lens, bubbles and rivets are merged.",
         "Eigenes 1:100-Modell: Bodenscheibe und Wand sind ein hohler Becher als Drehfläche, dessen 6-mm-Wand und 8-mm-Boden das gezeichnete U sind (das Druckmerkmal von 15 mm gilt für den ganzen Körper, und eine dickere Wand hätte eine Schnittfläche, die die Zeichnung nicht zeigt), das Wasser an der Innenwand, die Griffe als zwei Lofts aus drei Schnitten von 20 x 15,6 mm; Randwulst, Fase, Siedelinse, Blasen und Nieten sind zusammengefasst."),
    text("Thermal: base and wall as two skin cylinders, water and handles by reference, two steam discs of 15 mm; structure: cup, water and handles as members, a 15 mm lens as the surface member, two mass ellipsoids and a 15 mm contact disc.",
         "Thermik: Boden und Wand als zwei Hautzylinder, Wasser und Griffe als Verweis, zwei Dampfscheiben von 15 mm; Tragwerk: Becher, Wasser und Griffe als Stäbe, eine 15-mm-Linse als Oberflächenstab, zwei Massenellipsoide und eine 15-mm-Kontaktscheibe."),
]

# ------------------------------------------------------------ 1:200
body200 = [
    rev("wall", PT, [[0, 0], [0, 0.086], [H, 0.086], [H, 0.078], [0.012, 0.078], [0.012, 0]], "v"),
    rev("water", WT, [[0.007, 0], [0.007, 0.0785], [0.05, 0.0785], [0.05, 0]], "v"),
    box("handle-left", HD, (-0.125, -0.006, -0.012), (-0.082, 0.034, 0.012)),
    box("handle-right", HD, (0.082, -0.006, -0.012), (0.125, 0.034, 0.012)),
]
th200 = [
    skin("th-base", "base", "base-skin", 0.0861, 0, 0.03),
    skin("th-wall", "wall", "wall-skin", 0.0861, 0.03, H),
    th("th-water", "water", "water", "water-skin"),
    th("th-handle-left", "handle-left", "handle", "handle-skin"),
    th("th-handle-right", "handle-right", "handle", "handle-skin"),
    emit("th-steam", 0.055, 0.085, "steam"),
    emit("th-steam-plume", 0.085, 0.115, "steam-plume"),
]
st200 = [
    stm("member-wall", "wall", "pot", "member"),
    stm("member-water", "water", "water", "member"),
    stm("member-handle-left", "handle-left", "handles", "member"),
    stm("member-handle-right", "handle-right", "handles", "member"),
    tag(ell("member-surface", ST(member="surface", part="member", bone="surface"), (0, 0.0435 - 0.055, 0), (0.05, 0.015, 0.05)), member="surface", part="member"),
    mass("mass-water", "water", "water", (0, 0.025, 0), (0.06, 0.015, 0.06)),
    pad(0.075, 0.03),
]
D200 = [
    text("Own 1:200 model: one cup revolve with the drawn 8 mm wall and 12 mm floor, the water against the inner wall, and two 43 x 24 x 40 mm handle blocks from the drawn elevation bar and plan; the single drawn bar across the rim is the two handles joined in the drawing, not a part of the pot.",
         "Eigenes 1:200-Modell: ein Becher als Drehfläche mit der gezeichneten 8-mm-Wand und dem 12-mm-Boden, das Wasser an der Innenwand und zwei Griffblöcke von 43 x 24 x 40 mm nach gezeichnetem Aufrissbalken und Grundriss; der gezeichnete Balken über dem Rand sind die beiden Griffe zusammengefasst in der Zeichnung, kein Teil des Topfes."),
    text("Thermal: skins for base and wall, water and handles by reference, two steam discs of 30 and 10 mm; structure: cup, water, the two handles and a 30 mm lens as members, one water mass and a 30 mm contact disc; the pot mass is merged into the cup member.",
         "Thermik: Häute für Boden und Wand, Wasser und Griffe als Verweis, zwei Dampfscheiben von 30 und 10 mm; Tragwerk: Becher, Wasser, die beiden Griffe und eine 30-mm-Linse als Stäbe, eine Wassermasse und eine 30-mm-Kontaktscheibe; die Topfmasse ist im Becherstab aufgegangen."),
]
write("cooking-pot", {
    50: dict(body=body50, thermal=th50, structure=st50, decisions=D50),
    100: dict(body=body100, thermal=th100, structure=st100, decisions=D100),
    200: dict(body=body200, thermal=th200, structure=st200, decisions=D200),
}, category="objects")
