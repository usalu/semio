from author_11_lib import *

ST = Ctx("base", "stand-plastic", None, "body", 0.5)
NK = ST(bone="neck", tone=0.45)
HS = Ctx("panel", "housing-plastic", None, "body", 0.55)
BZ = Ctx("panel", "bezel-plastic", None, "body", 0.6)
FOOT = Ctx("base", "rubber-foot", None, "body", 0.15)
steel = ST(role="detail", tone=0.25)


def hump(v):
    def bz(p0, p1, p2, p3, s):
        return (1 - s) ** 3 * p0 + 3 * (1 - s) ** 2 * s * p1 + 3 * (1 - s) * s ** 2 * p2 + s ** 3 * p3
    lo = hi = None
    for k in range(2001):
        s = k / 2000
        y = bz(-0.0305, -0.015, -0.004, -0.004, s)
        if y >= v - 1e-9 and lo is None:
            lo = bz(-0.125, -0.125, -0.105, -0.085, s)
            hi = bz(0.11, 0.11, 0.09, 0.07, s)
    return lo, hi


def housing(sid, ctx, steps, e=4):
    rows = []
    for v, hw in steps:
        lo, hi = hump(v)
        rows.append((v, hw, (hi - lo) / 2, e, 0, (hi + lo) / 2))
    return lf(sid, ctx, "v", rows)


def rr(sid, ctx, w, u0, u1, v0, v1, r=0.006):
    return ext(sid, ctx, rrect(-w, u0, w, u1, r), "zx", v0, v1)


def reg(s, r):
    s["region"] = r
    return s


TG, TB, TH, TS = BZ(material="display-glass", layer="front-stack"), BZ(layer="front-stack"), HS(layer="housing-shell"), ST(layer="stand-body")

# ------------------------------------------------------------------ 1:50
body50 = [
    box("foot-front", FOOT, (0.085, 0, -0.112), (0.107, 0.0075, -0.09), repeat={"count": 2, "translation": [0, 0, 0.202]}),
    box("foot-rear", FOOT, (-0.088, 0, -0.112), (-0.066, 0.0075, -0.09), repeat={"count": 2, "translation": [0, 0, 0.202]}),
    ext("base-plate", ST, rrect(-0.125, -0.096, 0.125, 0.114, 0.012), "zx", 0.002, 0.0122),
    lf("neck", NK, "v", [(0, 0.035, 0.018, 4), (0.2, 0.03, 0.013, 4), (0.243, 0.032, 0.016, 4)]),
    cyl("hinge", NK(tone=0.35), "w", (0, 0.243, 0), 0.07, 0.012),
    housing("housing", HS, [(-0.0305, 0.265), (-0.0245, 0.261), (-0.0185, 0.253), (-0.0125, 0.242), (-0.0075, 0.23), (-0.004, 0.222)]),
    rr("panel-slab", BZ, 0.27, -0.17, 0.153, -0.04, -0.0305),
    box("screen-glass", BZ(material="display-glass", role="glazing", tone=0.1), (-0.158, -0.0414, -0.255), (0.141, -0.0386, 0.255)),
    ell("power-led", BZ(role="detail", tone=0.9), (-0.164, -0.0395, -0.24), (0.004, 0.004, 0.004)),
]
th50 = [
    th("th-screen-glass", "screen-glass", "panel-front", "front-stack"),
    th("th-panel-slab", "panel-slab", "panel-front", "front-stack"),
    th("th-housing", "housing", "housing", "housing-shell"),
    tag(reg(box("th-rear-plume", TH, (0.002, -0.016, -0.15), (0.058, -0.0085, 0.15)), "housing"), tags=["rear-plume"]),
    th("th-base-plate", "base-plate", "stand", "stand-body"),
    th("th-foot-front", "foot-front", "stand", "stand-body"),
    th("th-foot-rear", "foot-rear", "stand", "stand-body"),
    th("th-neck", "neck", "stand", "stand-body"),
    th("th-hinge", "hinge", "stand", "stand-body"),
]
st50 = [
    stm("member-base", "base-plate", "base", "member"),
    stm("member-neck", "neck", "neck", "member"),
    stm("member-panel", "panel-slab", "panel", "member"),
    stm("joint-hinge", "hinge", "neck", "joint"),
    ell("joint-base", steel(member="neck", part="joint", bone="neck"), (0, 0.012, 0), (0.025, 0.012, 0.04)),
    ell("mass-panel", steel(member="panel", part="mass", tone=0.15, bone="panel"), (-0.01, -0.025, 0), (0.1, 0.008, 0.2)),
    ell("mass-base", steel(member="base", part="mass", tone=0.15), (0.01, 0.006, 0), (0.08, 0.004, 0.1)),
    tag(box("support-base-contact", steel(member="base", part="support"), (-0.07, 0, -0.085), (0.09, 0.0075, 0.085)), tags=["base-contact"]),
]
D50 = [
    text("Own 1:50 model against the 1:50 drawings. Frames: the panel bone is the neck tip turned 95 degrees, its u axis runs up the screen and v backwards, the screen faces +x. Panel: a 0.54 x 0.323 m bezel slab with rounded corners, a 0.51 m glass sheet on its face (2.8 mm thin sheet, role glazing), an LED lens, and the rear housing hump as a six-section loft whose u extents follow the drawn elevation curve and whose width tapers from 0.53 m to 0.44 m (the plan draws it as wide as the bezel, a monitor hump is narrower). Stand: a rounded 0.25 x 0.21 m base plate on four 7.5 mm rubber feet, a tapered neck loft and the hinge barrel on the neck bone.",
         "Eigenes 1:50-Modell gegen die 1:50-Zeichnungen. Rahmen: der Knochen Panel ist die um 95 Grad gedrehte Halsspitze, seine u-Achse läuft den Bildschirm hinauf und v nach hinten, der Bildschirm blickt nach +x. Panel: eine Blendenplatte 0,54 x 0,323 m mit gerundeten Ecken, eine Glasscheibe von 0,51 m auf ihrer Fläche (1,8 mm dünnes Blech, Rolle glazing), eine LED-Linse und der hintere Gehäusebuckel als Loft aus sechs Schnitten, dessen u-Ausdehnung der gezeichneten Ansichtskurve folgt und dessen Breite von 0,53 m auf 0,44 m abnimmt (der Grundriss zeichnet ihn so breit wie die Blende, ein Monitorbuckel ist schmaler). Ständer: eine gerundete Fussplatte 0,25 x 0,21 m auf vier 7,5-mm-Gummifüssen, ein sich verjüngender Halsloft und die Scharniertrommel am Halsknochen."),
    text("Thermal (area = exposed surface): panel front = glass and bezel slab, housing = the hump, stand = base plate, feet, neck and hinge, all by reference; the rear plume is a 0.3 m wide vent patch inside the hump at the housing-top anchor (tag rear-plume). Faces buried in other solids and the floor contact do not count, so areaM2 was corrected at the root (see report). Structure: base plate, neck and bezel slab are the three members, hinge barrel and a neck-foot ellipsoid are the joints, two flat mass ellipsoids sit in panel and base, and a 0.17 x 0.2 m contact pad under the base is the support.",
         "Thermik (Fläche = freie Oberfläche): Panelfront = Glas und Blendenplatte, Gehäuse = der Buckel, Ständer = Fussplatte, Füsse, Hals und Scharnier, alles als Verweis; die Rückfahne ist ein 0,3 m breites Lüftungsfeld im Buckel am Anker housing-top (Markierung rear-plume). Verdeckte Flächen und der Bodenkontakt zählen nicht, daher wurde areaM2 an der Wurzel korrigiert (siehe Bericht). Tragwerk: Fussplatte, Hals und Blendenplatte sind die drei Stäbe, Scharniertrommel und ein Halsfuss-Ellipsoid die Gelenke, zwei flache Massenellipsoide sitzen in Panel und Fuss, und eine Kontaktfläche 0,17 x 0,2 m unter dem Fuss ist das Auflager."),
]

# ------------------------------------------------------------------ 1:100
body100 = [
    ext("base-plate", ST, rrect(-0.125, -0.096, 0.125, 0.114, 0.012), "zx", 0.0, 0.0152),
    lf("neck", NK, "v", [(0, 0.035, 0.019, 4), (0.2, 0.03, 0.014, 4), (0.243, 0.032, 0.017, 4)]),
    housing("housing", HS, [(-0.0248, 0.262), (-0.0155, 0.25), (-0.004, 0.222)], e=3),
    rr("panel-slab", BZ, 0.27, -0.17, 0.153, -0.04, -0.0248, r=0.004),
    box("screen-glass", BZ(material="display-glass", role="glazing", tone=0.1), (-0.158, -0.0414, -0.25), (0.141, -0.0386, 0.25)),
]
th100 = [
    th("th-screen-glass", "screen-glass", "panel-front", "front-stack"),
    th("th-panel-slab", "panel-slab", "panel-front", "front-stack"),
    th("th-housing", "housing", "housing", "housing-shell"),
    tag(reg(box("th-rear-plume", TH, (0.002, -0.0185, -0.15), (0.058, -0.0035, 0.15)), "housing"), tags=["rear-plume"]),
    th("th-base-plate", "base-plate", "stand", "stand-body"),
    th("th-neck", "neck", "stand", "stand-body"),
]
st100 = [
    stm("member-base", "base-plate", "base", "member"),
    stm("member-neck", "neck", "neck", "member"),
    stm("member-panel", "panel-slab", "panel", "member"),
    cyl("joint-hinge", steel(member="neck", part="joint", bone="neck"), "w", (0, 0.243, 0), 0.07, 0.0155),
    ell("mass-panel", steel(member="panel", part="mass", tone=0.15, bone="panel"), (-0.01, -0.025, 0), (0.1, 0.0155, 0.2)),
    tag(box("support-base-contact", steel(member="base", part="support"), (-0.07, 0, -0.085), (0.09, 0.0152, 0.085)), tags=["base-contact"]),
]
D100 = [
    text("Own 1:100 model against the 1:100 shapes: the base plate is 15 mm thick (the drawn 12 mm plus the merged feet), the neck keeps its taper, the housing hump is a three-section loft, the bezel slab is 15 mm and reaches into the hump, the glass stays a thin sheet. Feet, hinge barrel and the LED are merged away (below 15 mm).",
         "Eigenes 1:100-Modell gegen die 1:100-Formen: die Fussplatte ist 15 mm dick (die gezeichneten 12 mm samt zusammengefassten Füssen), der Hals behält seine Verjüngung, der Gehäusebuckel ist ein Loft aus drei Schnitten, die Blendenplatte ist 15 mm dick und reicht in den Buckel, das Glas bleibt ein dünnes Blech. Füsse, Scharniertrommel und LED sind zusammengefasst (unter 15 mm)."),
    text("Thermal and structure are rebuilt on these solids by reference (glass, bezel, hump, base plate, neck, plume patch); the hinge becomes a 31 mm joint cylinder, one flat mass ellipsoid sits in the panel and a 15 mm contact pad is the support.",
         "Thermik und Tragwerk sind auf diesen Körpern als Verweis neu aufgebaut (Glas, Blende, Buckel, Fussplatte, Hals, Fahnenfeld); das Scharnier wird ein Gelenkzylinder von 31 mm, ein flaches Massenellipsoid sitzt im Panel und eine 15-mm-Kontaktfläche ist das Auflager."),
]

# ------------------------------------------------------------------ 1:200
body200 = [
    lf("stand-base", ST, "u", [(-0.096, 0.125, 0.006, 4, 0, 0.006), (-0.06, 0.125, 0.0157, 4, 0, 0.0157), (0.0, 0.125, 0.0157, 4, 0, 0.0157), (0.114, 0.125, 0.006, 4, 0, 0.006)]),
    box("stand-neck", ST(tone=0.45), (-0.06, 0.012, -0.035), (0.0, 0.26, 0.035)),
    lf("panel", BZ, "v", [(-0.04, 0.27, 0.1615, 4, 0, -0.0085), (-0.0278, 0.27, 0.1615, 4, 0, -0.0085), (-0.0272, 0.24, 0.1175, 4, 0, -0.0075), (-0.004, 0.22, 0.1175, 4, 0, -0.0075)]),
]
th200 = [
    reg(box("th-front-slab", TB, (-0.17, -0.04, -0.27), (0.153, -0.0098, 0.27)), "panel-front"),
    reg(box("th-hump", TH, (-0.125, -0.034, -0.22), (0.11, -0.004, 0.22)), "housing"),
    th("th-stand-base", "stand-base", "stand", "stand-body"),
    th("th-stand-neck", "stand-neck", "stand", "stand-body"),
    tag(reg(box("th-rear-plume", TH, (0.002, -0.0348, -0.15), (0.058, -0.0045, 0.15)), "housing"), tags=["rear-plume"]),
]
st200 = [
    stm("member-base", "stand-base", "base", "member"),
    stm("member-neck", "stand-neck", "neck", "member"),
    stm("member-panel", "panel", "panel", "member"),
    tag(box("support-base-contact", steel(member="base", part="support"), (-0.07, 0, -0.085), (0.09, 0.0305, 0.085)), tags=["base-contact"]),
]
D200 = [
    text("Own 1:200 model: the stand is one flat base loft and one 0.06 x 0.07 m neck block, the panel one loft that starts as the 0.54 m bezel and steps in to the housing hump; bezel, glass, hump, feet, hinge and LED are merged into masses of at least 30 mm.",
         "Eigenes 1:200-Modell: der Ständer ist ein flacher Fussloft und ein Halsblock 0,06 x 0,07 m, das Panel ein Loft, der als 0,54 m breite Blende beginnt und zum Gehäusebuckel eingezogen wird; Blende, Glas, Buckel, Füsse, Scharnier und LED sind zu Massen von mindestens 30 mm zusammengefasst."),
    text("Thermal: a 30 mm bezel slab and a 30 mm hump block lying inside the panel loft carry panel front and housing (their exposed faces are the front and the back of the panel), base and neck by reference carry the stand, plus the plume patch; structure: the same three members and a contact pad, joints and masses merged into the members.",
         "Thermik: eine 30-mm-Blendenplatte und ein 30-mm-Buckelblock im Panelloft tragen Panelfront und Gehäuse (ihre freien Flächen sind Vorder- und Rückseite des Panels), Fuss und Hals als Verweis den Ständer, dazu das Fahnenfeld; Tragwerk: dieselben drei Stäbe und eine Kontaktfläche, Gelenke und Massen in den Stäben aufgegangen."),
]
write("desktop-monitor", {
    50: dict(body=body50, thermal=th50, structure=st50, decisions=D50),
    100: dict(body=body100, thermal=th100, structure=st100, decisions=D100),
    200: dict(body=body200, thermal=th200, structure=st200, decisions=D200),
})
