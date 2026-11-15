from author_11_lib import *

plate = Ctx("body", "powder-coated-steel", None, "body", 0.8)
collar = Ctx("body", "galvanised-steel", None, "body", 0.5)
T_PLATE = plate(role="body", layer="sheet", tone=0.8)
T_COLLAR = collar(role="body", layer="tube")
steel = Ctx("body", "powder-coated-steel", None, "detail", 0.25)


def square(h):
    return [[-h, -h], [h, -h], [h, h], [-h, h]]


def cone(sid, ctx, y0, y1, h0, h1, cs=True, ce=True, **kw):
    return frustum_y(sid, ctx, y0, y1, h0, h1, cs=cs, ce=ce, **kw)


def hanger(side, x, sz, y0, kind):
    c = steel(member="body", part="support")
    a = "ceiling-hanger-" + side
    if kind == "pin":
        return tag(cyl("support-" + a, c, "v", (x, y0 + sz / 2, 0), sz, sz / 2), tags=[a])
    return tag(cyl("support-" + a, c, "w", (x, y0 + sz / 8, 0), sz, sz / 2), tags=[a])


def jets(r, x0, x1, y):
    c = plate(role="body", layer="sheet", region="plate")
    return [tag(cyl("jet-left", c, "u", ((-x0 - x1) / 2, y, 0), x1 - x0, r), tags=["jet-left"]), tag(cyl("jet-right", c, "u", ((x0 + x1) / 2, y, 0), x1 - x0, r), tags=["jet-right"])]


def tube(sid, ctx, y0, y1, r):
    return lf(sid, ctx, "v", [(y0, r, r, 2), (y1, r, r, 2)], cs=False, ce=False)


def tr(sol, region):
    sol["region"] = region
    return sol


# ---------------- 1:50
c50 = pipe("collar", collar, "v", (0, 0.025, 0), 0.05, 0.1225, 0.112)
body50 = [
    c50,
    rev("collar-bead", collar(role="detail", tone=0.3), [[0.0235, 0.1175], [0.0235, 0.125], [0.031, 0.125], [0.031, 0.1175]], "v"),
    box("flange", plate(tone=0.88), (-0.2975, -0.012, -0.2975), (0.2975, 0, 0.2975), cr=0.003),
    cone("cone-1", plate, -0.012, -0.032, 0.265, 0.245),
    cone("cone-2", plate(tone=0.7), -0.032, -0.052, 0.2, 0.18),
    cone("cone-3", plate(tone=0.6), -0.052, -0.07, 0.135, 0.115),
]
for name, z in (("screw-right", -0.27), ("screw-left", 0.27)):
    body50.append(ell(name, plate(role="detail", tone=0.45), (-0.27, 0, z), (0.006, 0.004, 0.006), repeat={"count": 2, "translation": [0.54, 0, 0]}))


def thermal(scale):
    fl = {50: (0.012, 0.265, 0.0), 100: (0.02, 0.255, 0.0), 200: (0.04, 0.24, 0.0)}[scale]
    return fl


th50 = [
    th("th-flange", "flange", "plate", "sheet"),
    th("th-cone-1", "cone-1", "plate", "sheet"),
    th("th-cone-2", "cone-2", "plate", "sheet"),
    th("th-cone-3", "cone-3", "plate", "sheet"),
    th("th-screw-right", "screw-right", "plate", "sheet"),
    th("th-screw-left", "screw-left", "plate", "sheet"),
    tr(tube("th-collar", T_COLLAR, 0, 0.05, 0.1258), "collar"),
    *[tr(j, "plate") for j in jets(0.02, 0.262, 0.302, -0.022)],
]
st50 = [
    stm("member-body", "flange", "body", "member"),
    tag(ell("mass-body", steel(member="body", part="mass", tone=0.15), (0, -0.01, 0), (0.09, 0.02, 0.09)), ),
    hanger("left", -0.28, 0.016, -0.012, "pin"),
    hanger("right", 0.28, 0.016, 0.0, "roller"),
]

# ---------------- 1:100
body100 = [
    pipe("collar", collar, "v", (0, 0.025, 0), 0.05, 0.125, 0.1),
    box("flange", plate(tone=0.88), (-0.2975, -0.02, -0.2975), (0.2975, 0, 0.2975)),
    cone("cone-1", plate, -0.02, -0.045, 0.255, 0.225),
    cone("cone-2", plate(tone=0.7), -0.045, -0.07, 0.18, 0.12),
]
th100 = [
    th("th-flange", "flange", "plate", "sheet"),
    th("th-cone-1", "cone-1", "plate", "sheet"),
    th("th-cone-2", "cone-2", "plate", "sheet"),
    tr(tube("th-collar", T_COLLAR, 0, 0.05, 0.1258), "collar"),
    *[tr(j, "plate") for j in jets(0.02, 0.262, 0.302, -0.03)],
]
st100 = [
    stm("member-body", "flange", "body", "member"),
    ell("mass-body", steel(member="body", part="mass", tone=0.15), (0, -0.015, 0), (0.1, 0.03, 0.1)),
    hanger("left", -0.28, 0.016, -0.02, "pin"),
    hanger("right", 0.28, 0.016, 0.0, "roller"),
]

# ---------------- 1:200
body200 = [
    pipe("collar", collar, "v", (0, 0.025, 0), 0.05, 0.127, 0.097),
    box("flange", plate(tone=0.88), (-0.2975, -0.03, -0.2975), (0.2975, 0, 0.2975)),
    cone("cone", plate(tone=0.7), -0.03, -0.07, 0.245, 0.12),
]
th200 = [
    th("th-flange", "flange", "plate", "sheet"),
    tr(cone("th-cone", T_PLATE, -0.03, -0.07, 0.245, 0.12, cs=False, ce=False), "plate"),
    tr(tube("th-collar", T_COLLAR, 0, 0.05, 0.127), "collar"),
    *[tr(j, "plate") for j in jets(0.02, 0.262, 0.3, -0.03)],
]
st200 = [
    stm("member-body", "flange", "body", "member"),
    box("mass-body", steel(member="body", part="mass", tone=0.15), (-0.1, -0.06, -0.1), (0.1, 0.02, 0.1)),
    hanger("left", -0.28, 0.03, -0.03, "pin"),
    hanger("right", 0.28, 0.03, 0.0, "roller"),
]

D50 = [
    text("Own model for 1:50, built from the plan (0.595 m square flange, 0.25 m round collar with a bore of 0.112 m) and the elevation: a hollow collar tube with a rolled bead, a 12 mm flange plate, three stacked square sheet cones (lofts with squareness 12) and four domed screw heads. The bore stays closed in the body because the drawing paints the opening as a filled disc.",
         "Eigenes Modell für 1:50 nach Grundriss (quadratischer Flansch 0,595 m, runder Kragen 0,25 m mit 0,112 m Bohrung) und Ansicht: ein hohler Kragen mit Wulst, eine 12 mm Flanschplatte, drei gestapelte quadratische Blechkegel (Lofts mit Eckigkeit 12) und vier gewölbte Schraubenköpfe. Die Bohrung bleibt im Körper geschlossen, weil die Zeichnung die Öffnung als gefüllte Scheibe malt."),
    text("Thermal: the plate region is the exposed surface of the flange, the three cones and the screw heads (references to the body solids, buried faces do not count), the collar region the outer face of the collar as an open tube skin; the two supply jets are 0.04 m nozzles at the outlet slots. Single-layer sheets of 0.8 mm and 0.7 mm are surfaces, so no nested shells are needed.",
         "Thermik: die Region Platte ist die freie Fläche von Flansch, den drei Kegeln und den Schraubenköpfen (Verweise auf die Körper, verdeckte Flächen zählen nicht), die Region Kragen die Aussenfläche des Kragens als offene Rohrhaut; die zwei Zuluftstrahlen sind Düsen von 0,04 m an den Auslassschlitzen. Einlagige Bleche von 0,8 mm und 0,7 mm sind Flächen, verschachtelte Schalen entfallen."),
    text("Structure: the flange plate carries the 3.2 kg through the two hangers (pin left as a vertical stud through the plate, roller right as a transverse roller at the plate top); a compact mass ellipsoid sits at the centre of mass.",
         "Tragwerk: die Flanschplatte trägt die 3,2 kg über die zwei Halter (links ein Festlager als senkrechter Bolzen durch die Platte, rechts ein Rollenlager als Querrolle auf der Plattenoberseite); ein kompakter Massenellipsoid liegt im Schwerpunkt."),
]
D100 = [
    text("Own model for 1:100 against the 1:100 drawings: the flange is 20 mm thick, only two cones remain (0.255 to 0.225 and 0.18 to 0.12 half widths) and the collar bore is 0.1 m. Bead, screw heads and the third cone step are merged away because they fall below 15 mm.",
         "Eigenes Modell für 1:100 nach den 1:100-Zeichnungen: der Flansch ist 20 mm dick, es bleiben zwei Kegel (Halbbreiten 0,255 auf 0,225 und 0,18 auf 0,12) und die Kragenbohrung misst 0,1 m. Wulst, Schraubenköpfe und die dritte Kegelstufe sind weggefallen, weil sie unter 15 mm liegen."),
    text("Thermal and structure are rebuilt on these solids: references to flange, both cones and the collar, two jets; flange plate as member, one mass ellipsoid, pin and roller supports sized at the 15 mm feature.",
         "Thermik und Tragwerk sind auf diesen Körpern neu aufgebaut: Verweise auf Flansch, beide Kegel und Kragen, zwei Strahlen; Flanschplatte als Stab, ein Massenellipsoid, Bolzen- und Rollenlager in der Grösse des 15-mm-Merkmals."),
]
D200 = [
    text("Own model for 1:200: one 30 mm flange slab (flange plus the first cone step merged) and one merged cone from half width 0.245 to 0.12 over 40 mm and a 0.254 m collar tube with a 30 mm wall. Recognisably the same diffuser with the same 0.595 x 0.07 x 0.595 m envelope; a minimal model of three solids.",
         "Eigenes Modell für 1:200: eine Flanschplatte von 30 mm (Flansch plus erste Kegelstufe), ein zusammengefasster Kegel von der Halbbreite 0,245 auf 0,12 über 40 mm und ein Kragenrohr von 0,254 m mit 30 mm Wandstärke. Erkennbar derselbe Auslass in derselben Hülle 0,595 x 0,07 x 0,595 m; ein minimales Modell aus drei Körpern."),
    text("Thermal: the flange by reference, the sloped sides of the merged cone as an open skin and the collar with two jets; structure: flange plate, mass ellipsoid and the two hanger supports enlarged to the 30 mm feature.",
         "Thermik: der Flansch als Verweis, die Schrägflächen des zusammengefassten Kegels als offene Haut und der Kragen mit zwei Strahlen; Tragwerk: Flanschplatte, Massenellipsoid und die beiden Halterlager auf das 30-mm-Merkmal vergrössert."),
]
write("ceiling-diffuser", {
    50: dict(body=body50, thermal=th50, structure=st50, decisions=D50),
    100: dict(body=body100, thermal=th100, structure=st100, decisions=D100),
    200: dict(body=body200, thermal=th200, structure=st200, decisions=D200),
})
