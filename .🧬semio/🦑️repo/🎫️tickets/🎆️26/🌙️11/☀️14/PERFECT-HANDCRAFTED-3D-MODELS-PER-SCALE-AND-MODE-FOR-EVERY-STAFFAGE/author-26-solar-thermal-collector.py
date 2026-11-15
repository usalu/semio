import importlib.util, os, json, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("c26", os.path.join(here, "author-26-common.py"))
c = importlib.util.module_from_spec(spec); spec.loader.exec_module(c)
ASSET = json.load(open("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/solar-thermal-collector/🔣️.json", encoding="utf8"))["asset"]
V = c.Volume("solar-thermal-collector")
TH = 35.0
C, S = math.cos(math.radians(TH)), math.sin(math.radians(TH))
P1 = (-0.79047, 0.18)
AL = dict(material="anodised-aluminium", role="body")
GL = dict(material="solar-glass")
AB = dict(material="selective-absorber", role="body")
CU = dict(material="copper")
GS = dict(material="galvanised-steel", role="body")
WOOL = dict(material="mineral-wool", role="body")


def W(s, t, z=0.0):
    return [P1[0] + s * C - t * S, P1[1] + s * S + t * C, z]


def sbox(b, id, bone, s0, s1, t0, t1, z0, z1, rep=None, **m):
    return b.box(id, bone, W((s0 + s1) / 2, (t0 + t1) / 2, (z0 + z1) / 2), [s1 - s0, t1 - t0, z1 - z0], rz=TH, rep=rep, **m)


def sprofile(pts):
    return [W(s, t)[:2] for s, t in pts]


def strut_pts(sc):
    g = ASSET["representations"][str(sc)]["geometry"]["elevation"]
    return {s["id"]: s["points"] for s in g if s["id"] in ("front-strut", "rear-strut")}


def stand(b, sc, rail_h, strut_z, strut_zw, rails=True, tone=0.5):
    if rails:
        b.box("base-rail", "stand", [0, rail_h / 2, -0.4], [1.5, rail_h, 0.05], rep={"count": 2, "step": [0, 0, 0.8]}, tone=tone, **GS)
    pts = strut_pts(sc)
    for n in ("front-strut", "rear-strut"):
        p = [[x, max(y, rail_h)] for x, y in pts[n]]
        b.ext(n, "stand", "side", p, strut_z, strut_z + strut_zw, rep={"count": 2, "step": [0, 0, 0.8]} if strut_zw < 0.2 else None, tone=tone, **GS)


FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
REGIONS = (("housing", ["tray-shell", "tray-side", "insulation", "collector-box"]), ("glazing", ["glass"]), ("absorber", ["absorber-plate", "absorber", "riser"]), ("header-lower", ["header-lower"]), ("header-upper", ["header-upper"]), ("stand", ["base-rail", "base-slab", "front-strut", "rear-strut"]))


def headers(b, half, r, t=0.048):
    for n, s in (("lower", 0.06), ("upper", 1.94)):
        p = W(s, t)
        b.cyl(f"header-{n}", "headers", [p[0], p[1], -half], [p[0], p[1], half], r, tone=0.4, role="body", **CU)


def build(sc):
    b = V.at(sc, "body")
    if sc == 50:
        b.ext("tray-shell", "housing", "side", sprofile([(0, 0.1), (0, 0), (2, 0), (2, 0.1), (1.98, 0.1), (1.98, 0.012), (0.02, 0.012), (0.02, 0.1)]), -0.5, 0.5, tone=0.35, **AL)
        sbox(b, "tray-side", "housing", 0.02, 1.98, 0, 0.1, -0.5, -0.47, rep={"count": 2, "step": [0, 0, 0.97]}, tone=0.35, **AL)
        sbox(b, "insulation", "housing", 0.02, 1.98, 0.012, 0.038, -0.47, 0.47, tone=0.8, **WOOL)
        sbox(b, "glass", "glass", 0.02, 1.98, 0.09, 0.1, -0.47, 0.47, tone=0.92, role="glazing", **GL)
        sbox(b, "absorber-plate", "absorber", 0.06, 1.94, 0.05, 0.058, -0.455, 0.455, tone=0.2, **AB)
        b.cyl("riser", "absorber", W(0.06, 0.044, -0.4), W(1.94, 0.044, -0.4), 0.006, rep={"count": 9, "step": [0, 0, 0.1]}, tone=0.55, role="detail", slender=True, **CU)
        headers(b, 0.44, 0.0165)
        stand(b, 50, 0.05, -0.425, 0.05)
    elif sc == 100:
        b.ext("tray-shell", "housing", "side", sprofile([(0, 0.1), (0, 0), (2, 0), (2, 0.1), (1.975, 0.1), (1.975, 0.034), (0.025, 0.034), (0.025, 0.1)]), -0.5, 0.5, tone=0.35, **AL)
        sbox(b, "tray-side", "housing", 0.025, 1.975, 0, 0.1, -0.5, -0.465, rep={"count": 2, "step": [0, 0, 0.965]}, tone=0.35, **AL)
        sbox(b, "glass", "glass", 0.025, 1.975, 0.088, 0.1, -0.465, 0.465, tone=0.92, role="glazing", **GL)
        sbox(b, "absorber-plate", "absorber", 0.06, 1.94, 0.045, 0.06, -0.45, 0.45, tone=0.2, **AB)
        b.cyl("riser", "absorber", W(0.06, 0.048, -0.4), W(1.94, 0.048, -0.4), 0.01, rep={"count": 9, "step": [0, 0, 0.1]}, tone=0.55, role="detail", slender=True, **CU)
        headers(b, 0.435, 0.022)
        stand(b, 100, 0.06, -0.425, 0.05)
    elif sc == 200:
        b.ext("tray-shell", "housing", "side", sprofile([(0, 0.1), (0, 0), (2, 0), (2, 0.1), (1.95, 0.1), (1.95, 0.04), (0.05, 0.04), (0.05, 0.1)]), -0.5, 0.5, tone=0.35, **AL)
        sbox(b, "tray-side", "housing", 0.05, 1.95, 0, 0.1, -0.5, -0.45, rep={"count": 2, "step": [0, 0, 0.95]}, tone=0.35, **AL)
        sbox(b, "glass", "glass", 0.05, 1.95, 0.07, 0.1, -0.45, 0.45, tone=0.92, **GL, role="body")
        sbox(b, "absorber", "absorber", 0.05, 1.95, 0.04, 0.07, -0.435, 0.435, tone=0.2, **AB)
        headers(b, 0.42, 0.0225)
        stand(b, 200, 0.08, -0.425, 0.05)
    else:
        sbox(b, "collector-box", "housing", 0, 2, 0, 0.1, -0.5, 0.5, tone=0.4, **AL)
        b.box("base-rail", "stand", [0, 0.04, -0.4], [1.5, 0.08, 0.075], rep={"count": 2, "step": [0, 0, 0.8]}, tone=0.5, **GS)
        pts = strut_pts(500)
        for n in ("front-strut", "rear-strut"):
            b.ext(n, "stand", "side", [[x, max(y, 0.08)] for x, y in pts[n]], -0.4375, -0.3625, rep={"count": 2, "step": [0, 0, 0.8]}, tone=0.5, **GS)


def thermal(sc):
    t = V.at(sc, "thermal")
    ids = {s["id"] for s in V.data[sc]["body"]}
    for r, lst in REGIONS:
        for i in lst:
            if i in ids:
                t.ref(f"th-{i}", i, region=r, **({"sectionFaces": ["-v"]} if i in ("glass", "absorber-plate", "absorber") else {}))
    f = FEATURE[sc]
    if sc == 500:
        sb = lambda id, s0, s1, t0, t1, z0, z1, region, tone: t.box(id, "glass" if region == "glazing" else "absorber", W((s0 + s1) / 2, (t0 + t1) / 2, (z0 + z1) / 2), [s1 - s0, t1 - t0, z1 - z0], rz=TH, region=region, tone=tone, sectionFaces=["-v"], **(dict(GL, role="body") if region == "glazing" else AB))
        sb("th-glass", 0.1, 1.9, 0.025, 0.1, -0.45, 0.45, "glazing", 0.92)
        sb("th-absorber", 0.1, 1.9, 0.0, 0.075, -0.4, 0.4, "absorber", 0.2)
        for n, s in (("lower", 0.0375), ("upper", 1.9625)):
            p = W(s, 0.05)
            t.cyl(f"th-header-{n}", "headers", [p[0], p[1], -0.4], [p[0], p[1], 0.4], 0.0375, region=f"header-{n}", tone=0.4, role="body", **CU)
    p = W(1.75, 0.1, 0)
    t.rev("th-absorber-plume-outlet", "glass", [p[0], p[1], 0], [-S, C, 0], [[0, 0], [0, 0.25], [f * 1.001, 0.25], [f * 1.001, 0]], region="absorber", tags=["absorber-plume"], sectionFaces=["-v", "+v"], tone=0.9, role="body", **GL)


def sbrackets(r, sc, t1, w, rep=True):
    pass


def structure(sc):
    r = V.at(sc, "structure")
    ids = {s["id"] for s in V.data[sc]["body"]}
    for i, m in (("base-rail", "stand"), ("base-slab", "stand"), ("front-strut", "stand"), ("rear-strut", "stand"), ("tray-shell", "housing"), ("tray-side", "housing"), ("collector-box", "housing"), ("glass", "glass"), ("absorber-plate", "absorber"), ("absorber", "absorber"), ("header-lower", "headers"), ("header-upper", "headers")):
        if i in ids:
            r.ref(f"st-{i}", i, member=m, part="member")
    if sc < 200:
        for n, s in (("front", 0.29), ("rear", 1.64)):
            sbox(r, f"joint-strut-{n}", "stand", s - 0.06, s + 0.06, 0, 0.012 if sc == 50 else 0.015, -0.425, -0.375, rep={"count": 2, "step": [0, 0, 0.8]}, tone=0.5, member="stand", part="joint", **GS)
    if sc == 50:
        sbox(r, "mass-housing", "housing", 0.5, 1.5, 0.0015, 0.0105, -0.3, 0.3, tone=0.35, member="housing", part="mass", **AL)
        sbox(r, "mass-glass", "glass", 0.6, 1.4, 0.0905, 0.0995, -0.25, 0.25, tone=0.92, member="glass", part="mass", material="solar-glass", role="body")
        sbox(r, "mass-absorber", "absorber", 0.6, 1.4, 0.0502, 0.0577, -0.25, 0.25, tone=0.2, member="absorber", part="mass", **AB)
        r.ell("mass-headers", "headers", W(1.0, 0.048), [0.08, 0.0075, 0.04], rz=TH, tone=0.4, member="headers", part="mass", **CU)
    elif sc == 100:
        sbox(r, "mass-housing", "housing", 0.5, 1.5, 0.0005, 0.0335, -0.3, 0.3, tone=0.35, member="housing", part="mass", **AL)
        sbox(r, "mass-glass", "glass", 0.6, 1.4, 0.085, 0.1, -0.25, 0.25, tone=0.92, member="glass", part="mass", material="solar-glass", role="body")
        sbox(r, "mass-absorber", "absorber", 0.6, 1.4, 0.045, 0.06, -0.25, 0.25, tone=0.2, member="absorber", part="mass", **AB)
    elif sc == 200:
        sbox(r, "mass-housing", "housing", 0.5, 1.5, 0.0005, 0.0395, -0.3, 0.3, tone=0.35, member="housing", part="mass", **AL)
        sbox(r, "mass-glass", "glass", 0.6, 1.4, 0.07, 0.1, -0.25, 0.25, tone=0.92, member="glass", part="mass", material="solar-glass", role="body")
        sbox(r, "mass-absorber", "absorber", 0.6, 1.4, 0.04, 0.07, -0.25, 0.25, tone=0.2, member="absorber", part="mass", **AB)
    else:
        sbox(r, "mass-collector", "housing", 0.4, 1.6, 0, 0.1, -0.3, 0.3, tone=0.4, member="housing", part="mass", **AL)
        sbox(r, "member-glass", "glass", 0.1, 1.9, 0.025, 0.1, -0.45, 0.45, tone=0.92, member="glass", part="member", material="solar-glass", role="body")
        sbox(r, "member-absorber", "absorber", 0.1, 1.9, 0.0, 0.075, -0.4, 0.4, tone=0.2, member="absorber", part="member", **AB)
        for n, s in (("lower", 0.0375), ("upper", 1.9625)):
            p = W(s, 0.05)
            r.cyl(f"member-header-{n}", "headers", [p[0], p[1], -0.4], [p[0], p[1], 0.4], 0.0375, tone=0.4, member="headers", part="member", role="body", **CU)
    if sc == 500:
        r.box("mass-stand-front", "stand", [-0.55, 0.25, -0.4], [0.075, 0.25, 0.075], rep={"count": 2, "step": [0, 0, 0.8]}, tone=0.5, member="stand", part="mass", **GS)
        r.box("mass-stand-rear", "stand", [0.55, 0.62, -0.4], [0.075, 0.6, 0.075], rep={"count": 2, "step": [0, 0, 0.8]}, tone=0.5, member="stand", part="mass", **GS)
    else:
        r.ell("mass-stand-front", "stand", [-0.55, 0.2, -0.4], [0.019, 0.12, 0.024], rep={"count": 2, "step": [0, 0, 0.8]}, tone=0.5, member="stand", part="mass", **GS)
        r.ell("mass-stand-rear", "stand", [0.55, 0.62, -0.4], [0.019, 0.3, 0.024], rep={"count": 2, "step": [0, 0, 0.8]}, tone=0.5, member="stand", part="mass", **GS)
    pt = FEATURE[sc]
    if sc == 500:
        r.box("base-contact-pad", "stand", [0, pt / 2, -0.4], [1.5, pt, 0.075], rep={"count": 2, "step": [0, 0, 0.8]}, tone=0.5, member="stand", part="support", anchor="base-contact", **GS)
    else:
        r.box("base-contact-pad", "stand", [0, pt / 2, -0.4], [1.5, pt, 0.05], rep={"count": 2, "step": [0, 0, 0.8]}, tone=0.5, member="stand", part="support", anchor="base-contact", **GS)


for sc in (50, 100, 200, 500):
    build(sc)
    thermal(sc)
    structure(sc)

V.decide(50, "Detailed model in the collector frame (tilt 35 degrees, 2.0 m along the slope, 1.0 m wide, 0.1 m deep): the tray is the drawn C-profile (12 mm floor, 20 mm end walls) extruded across the width, closed by two 30 mm side walls; 26 mm mineral-wool insulation on the floor; the glass is a 10 mm plate flush with the tray top (the real 4 mm is below the 7.5 mm smallest feature), the selective absorber an 8 mm plate (real 0.5 mm) on nine slender 12 mm copper risers, with two 33 mm headers at the ends of the slope; the stand is two 1.5 m base rails and two struts per rail at 0.55 m left and right of the centre, the struts cut on the tray underside and entering the floor by 10 mm like the drawing. The elevation drawing omitted the near side wall that front and plan show: a filled side-wall shape was added to the 1:50 and 1:100 elevations.",
         "Detailliertes Modell im Kollektorrahmen (Neigung 35 Grad, 2,0 m entlang der Schräge, 1,0 m breit, 0,1 m tief): Die Wanne ist das gezeichnete C-Profil (12 mm Boden, 20 mm Stirnwände) über die Breite extrudiert und durch zwei 30-mm-Seitenwände geschlossen; 26 mm Mineralwolle auf dem Boden; das Glas ist eine 10-mm-Platte bündig mit der Wannenoberkante (die echten 4 mm liegen unter der kleinsten Einzelheit von 7,5 mm), der selektive Absorber eine 8-mm-Platte (echt 0,5 mm) auf neun schlanken 12-mm-Kupferrohren, dazu zwei 33-mm-Sammler an den Enden der Schräge; das Gestell besteht aus zwei 1,5 m langen Fußschienen mit je zwei Streben 0,55 m links und rechts der Mitte, die Streben sind an der Wannenunterseite abgeschnitten und greifen wie gezeichnet 10 mm in den Boden. Die Seitenansicht ließ die vordere Seitenwand weg, die Vorder- und Draufsicht zeigen: in die Ansichten 1:50 und 1:100 wurde eine gefüllte Seitenwandfläche eingefügt.")
V.decide(50, "Thermal: every region is a set of references to the body solids that carry its surface (housing: tray, side walls, insulation; glazing: glass; absorber: plate and risers; two header regions; stand: rails and struts); all regions have one layer, so each is a single surface and no nested shell exists. The glass and the absorber plate are one-sided patches whose backs lie inside the sealed collector: their reference declares the lower faces as section faces, so the exposed area is the outer face. The plume emitter is a 0.5 m disc on the glass at the upper edge. Structure: the stand rails and struts, the tray and its side walls, glass, absorber plate and both headers are the members of their bones; four bracket plates at the strut tops are the joints; compact masses sit at the centres of mass of housing, glass, absorber and headers; the stand mass (centre 0.35 m, in the open space under the tray) is split into a front and a rear strut lump; two 50 mm pads under the rails carry the base-contact anchor.",
         "Thermik: Jede Region ist eine Menge von Verweisen auf die Körper, die ihre Oberfläche tragen (housing: Wanne, Seitenwände, Dämmung; glazing: Glas; absorber: Platte und Rohre; zwei Sammlerregionen; stand: Schienen und Streben); alle Regionen haben eine Schicht, also eine einfache Fläche ohne verschachtelte Schalen. Glas und Absorberplatte sind einseitige Flächen, deren Rückseite im geschlossenen Kollektor liegt: ihr Verweis erklärt die Unterseiten zu Schnittflächen, die freie Fläche ist die Außenseite. Der Fahnenemitter ist eine 0,5-m-Scheibe auf dem Glas an der Oberkante. Tragwerk: Gestellschienen und -streben, Wanne mit Seitenwänden, Glas, Absorberplatte und beide Sammler sind die Stäbe ihrer Knochen; vier Konsolplatten an den Strebenköpfen sind die Gelenke; kompakte Massen liegen in den Schwerpunkten von Wanne, Glas, Absorber und Sammlern; die Gestellmasse (Schwerpunkt 0,35 m, im freien Raum unter der Wanne) ist in einen vorderen und einen hinteren Strebenklumpen geteilt; zwei 50-mm-Platten unter den Schienen tragen den Anker base-contact.")
V.decide(100, "Redrawn against the 1:100 shapes: insulation and floor merge into one 34 mm tray floor (the 1:100 drawing keeps the insulation hatch, the solid does not), end walls 25 mm, side walls 35 mm, glass 12 mm as drawn (smallest feature 15 mm but the drawing is 12 mm thin), absorber plate 15 mm on nine 20 mm risers, headers 44 mm; the stand keeps the 1:50 rails, struts 50 mm wide with the 1:100 outline. Joints and the contact pads stay, the headers' mass merges into the absorber mass.",
         "Neu gezeichnet gegen die Formen 1:100: Dämmung und Boden gehen in einem 34 mm dicken Wannenboden auf (die Zeichnung 1:100 behält die Dämmungsschraffur, der Körper nicht), Stirnwände 25 mm, Seitenwände 35 mm, Glas 12 mm wie gezeichnet (kleinste Einzelheit 15 mm, die Zeichnung ist aber nur 12 mm dünn), Absorberplatte 15 mm auf neun 20-mm-Rohren, Sammler 44 mm; das Gestell behält die Schienen von 1:50, Streben 50 mm breit mit dem Umriss von 1:100. Gelenke und Kontaktplatten bleiben, die Sammlermasse geht in der Absorbermasse auf.")
V.decide(100, "Thermal: references per region as at 1:50 (the insulation is part of the tray solid), glass and absorber plate keep the lower faces declared as section faces, plume disc of 15 mm. Structure: the same members as references, two strut brackets (15 mm), masses of housing, glass and absorber as boxes inside their solids, two strut lumps, two 15 mm contact pads.",
         "Thermik: Verweise je Region wie bei 1:50 (die Dämmung ist Teil des Wannenkörpers), Glas und Absorberplatte behalten die als Schnittflächen erklärten Unterseiten, Fahnenscheibe von 15 mm. Tragwerk: dieselben Stäbe als Verweise, zwei Strebenkonsolen (15 mm), Massen von Wanne, Glas und Absorber als Quader in ihren Körpern, zwei Strebenklumpen, zwei 15 mm dicke Kontaktplatten.")
V.decide(200, "Redrawn against the 1:200 shapes: the collector is a tray with 40 mm floor and 50 mm end and side walls that holds a 30 mm absorber slab and a 30 mm glass slab (risers, insulation and the 8 mm plate vanish into slabs); headers are 45 mm pipes inside the slabs; rails 80 mm high, struts 80 mm wide at the 1:200 outline; no brackets or joints.",
         "Neu gezeichnet gegen die Formen 1:200: Der Kollektor ist eine Wanne mit 40 mm Boden und 50 mm Stirn- und Seitenwänden, die eine 30-mm-Absorberplatte und eine 30-mm-Glasplatte aufnimmt (Rohre, Dämmung und die 8-mm-Platte gehen in den Platten auf); die Sammler sind 45-mm-Rohre in den Platten; Schienen 80 mm hoch, Streben 80 mm breit im Umriss von 1:200; keine Konsolen und Gelenke.")
V.decide(200, "Thermal: references to the body solids, the glass and absorber references declare their lower faces as section faces, the absorber and the headers lie under the glass slab (their exposed area is small by construction), plume disc of 30 mm. Structure: members as references, masses of housing, glass and absorber as slabs, two strut lumps, two 30 mm contact pads; the 1.6 kg header mass is merged into the absorber mass.",
         "Thermik: Verweise auf die Körper, Glas- und Absorberverweis erklären ihre Unterseiten zu Schnittflächen, Absorber und Sammler liegen unter der Glasplatte (ihre freie Fläche ist konstruktionsbedingt klein), Fahnenscheibe von 30 mm. Tragwerk: Stäbe als Verweise, Massen von Wanne, Glas und Absorber als Platten, zwei Strebenklumpen, zwei 30 mm dicke Kontaktplatten; die 1,6-kg-Sammlermasse geht in der Absorbermasse auf.")
V.decide(500, "Redrawn against the 1:500 shapes: one 100 mm tilted slab for the whole collector (tray, insulation, absorber and glass), two 75 mm base rails and two 75 mm struts per rail (the drawing's foot block is generalised to the 75 mm smallest feature, not to a full-width plate).",
         "Neu gezeichnet gegen die Formen 1:500: eine einzige 100 mm dicke geneigte Platte für den ganzen Kollektor (Wanne, Dämmung, Absorber und Glas), zwei 75-mm-Fußschienen und je Schiene zwei 75-mm-Streben (der gezeichnete Fußblock ist auf die kleinste Einzelheit von 75 mm verallgemeinert, nicht zu einer Platte über die ganze Breite).")
V.decide(500, "Thermal: the housing is the slab; the glazing, absorber and the two headers have no body solid at this scale and are real hidden solids inside it (75 mm minimum: glass slab 75 mm, absorber slab 75 mm overlapping it, 75 mm header pipes), lower faces declared as section faces; plume disc of 75 mm. Structure: the slab, the two 75 mm struts per side and the rails carry the load; glass, absorber and headers are hidden member solids inside the slab (real solids, the body has none); one slab mass, two strut lumps, two contact pads.",
         "Thermik: Das Gehäuse ist die Platte; Verglasung, Absorber und die beiden Sammler haben in diesem Maßstab keinen Körper und sind echte verdeckte Körper darin (mindestens 75 mm: Glasplatte 75 mm, Absorberplatte 75 mm, überlappend, Sammlerrohre 75 mm), Unterseiten als Schnittflächen erklärt; Fahnenscheibe von 75 mm. Tragwerk: Platte, die zwei 75-mm-Streben je Seite und die Schienen tragen die Last; Glas, Absorber und Sammler sind verdeckte Stabkörper in der Platte (echte Körper, der Körper hat keine); eine Plattenmasse, zwei Strebenklumpen, zwei Kontaktplatten.")
V.write()
