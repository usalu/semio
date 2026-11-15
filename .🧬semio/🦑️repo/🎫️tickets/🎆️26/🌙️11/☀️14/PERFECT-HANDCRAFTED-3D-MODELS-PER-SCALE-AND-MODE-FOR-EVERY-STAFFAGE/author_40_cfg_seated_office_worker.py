from author_24_lib import *

FIT50 = False
floor_skip = ("shoe", "boot", "chair-arm")
sym_skip = ()
YS = [f"chair-{p}-{q}" for p in ("yoke", "stem") for q in ("front", "side-a", "side-b", "back-a", "back-b")]
DROP = {
    100: ["eye-l", "eye-r", "chair-seat-lever", "laptop-hinge", "chair-column-collar"],
    200: ["ear-l", "ear-r", "nose", "thumb-l", "thumb-r", "chair-column-collar", "chair-seat-lever", "chair-seat-mechanism", "laptop-hinge", "desk-foot-plate"] + YS,
    500: ["shoulder-l", "shoulder-r", "knee-l", "knee-r", "collar", "shoulders", "chair-castor-side-b", "chair-castor-back-b", "chair-column-sleeve", "chair-column-collar", "chair-seat-lever", "chair-seat-mechanism", "chair-armrest-post", "chair-armrest-pad", "laptop-hinge", "desk-foot-plate", "desk-apron", "chair-column-cone"] + YS,
}


def repair50(vols):
    pass


def post50(vols):
    for s in vols["50"]["body"]:
        if s["id"] == "pelvis-trousers":
            for q in s["sections"]:
                if q["at"][0] > 0.0:
                    q["rx"] *= 0.86
                    q["ry"] *= 0.88
        if s["id"] == "shirt":
            for q in s["sections"][:5]:
                q["rx"] *= 1.07
                q["ry"] *= 1.07


def P(b, i):
    return next((x for x in b if x["id"] == i), None)


def special(b, s):
    if s >= 200:
        P(b, "desk-top")["size"] = [0.78, 0.025, 0.6]
        P(b, "desk-post")["size"] = [0.695, 0.04, 0.04]
        P(b, "laptop-base")["size"] = [0.32, 0.021, 0.34]
        P(b, "laptop-lid")["size"] = [0.2 if s == 500 else 0.215, 0.03 if s == 200 else 0.075, 0.34]
        if s == 500:
            P(b, "laptop-base")["size"] = [0.32, 0.075, 0.34]
            P(b, "laptop-base")["transform"]["translation"][1] = -0.0175
    if s == 200:
        P(b, "desk-apron")["size"] = [0.03, 0.1, 0.51]
    if s == 500:
        P(b, "chair-column")["r0"] = P(b, "chair-column")["r1"] = 0.0575
        P(b, "chair-column")["p0"] = [0.005, 0, 0]
        P(b, "chair-column")["p1"] = [0.43, 0, 0]
        for q in P(b, "chair-backrest-shell")["sections"]:
            q["at"][1] = 0.0
        P(b, "chair-castor-front")["radii"] = [0.028] * 3
        P(b, "chair-castor-side-a")["radii"] = [0.028] * 3
        P(b, "chair-castor-back-a")["radii"] = [0.028] * 3


skip_members = ("floor",)


def struct_extra(a, body, scale):
    f = FEATURE[scale]
    have = {s["bone"] for s in body}
    out = []
    need = [b for b in member_bones(a) if b not in skip_members]
    for bone in need:
        p = primary(body, bone)
        if p is None or p["kind"] in ("loft", "ellipsoid"):
            continue
        c = scaled(p, 0.7, f"member-{bone}")
        if c["kind"] == "box":
            c["size"] = [min(o, max(n, f)) for o, n in zip(p["size"], c["size"])]
        c.pop("region", None)
        c["member"], c["part"] = bone, "member"
        c["role"] = "body"
        out.append(c)
    hh = max(0.0075, 0.55 * f)
    rr = max(0.014, 1.1 * hh)
    spots = [(0.34, 0), (0.133, 0.273), (0.133, -0.273), (-0.203, 0.176), (-0.203, -0.176), (1.02, 0.275), (1.02, -0.275)]
    for i, (x, z) in enumerate(spots):
        out.append(ellipsoid(f"member-floor-pad-{i + 1}", "floor", "rubber" if i < 5 else "desk-board", 0.3, [rr, hh, rr], [x, hh, z], member="floor", part="member"))
    return out
DECISIONS = {
    50: [
        D("Body: the person is built from lofts of superelliptic sections (trunk in the shirt, trousers over pelvis and thighs, sleeves, forearms, mitten hands with thumbs, shoes in a rear part and a toe part, neck, head with hair, ears, eyes and nose) typing on a laptop. The furniture is modelled as real objects: a five-star chair base with hub, castors, yokes and stems, a gas-lift column in its sleeve and collar, a seat cushion on a steel mechanism with lever, a backrest shell on its strut, two armrests with posts and pads, and a desk of a 25 mm board, an apron and two square legs on foot plates, the laptop as base, hinge and lid. The trousers waist is taken in 12-14 % above the hip and the shirt hem is 7 % wider so that the shirt hangs over the waistband as it does on a sitting person; this keeps the exposed shirt surface within 30 % of its physiological area.", "Körper: die Person besteht aus Lofts mit superelliptischen Querschnitten (Rumpf im Hemd, Hose über Becken und Oberschenkeln, Ärmel, Unterarme, Fäustlingshände mit Daumen, Schuhe aus Hinter- und Zehenteil, Hals, Kopf mit Haar, Ohren, Augen und Nase) und tippt auf einem Laptop. Die Möbel sind als echte Gegenstände modelliert: ein fünfarmiger Stuhlfuss mit Nabe, Rollen, Gabeln und Schäften, eine Gasfedersäule in Hülse und Manschette, ein Sitzpolster auf einer Stahlmechanik mit Hebel, eine Rückenschale auf ihrer Strebe, zwei Armlehnen mit Stützen und Polstern sowie ein Tisch aus 25-mm-Platte, Zarge und zwei Vierkantbeinen auf Fussplatten, der Laptop aus Basis, Scharnier und Deckel. Die Hosentaille ist über der Hüfte um 12-14 % eingezogen und der Hemdsaum 7 % weiter, damit das Hemd wie bei einer sitzenden Person über den Bund fällt; so bleibt die frei liegende Hemdfläche innerhalb von 30 % ihrer physiologischen Fläche."),
        D("Thermal: every region consists of references to the body solids that carry it; shirt, trousers, shoes and the seat fabric are nested shells per layer, innermost first; nose and mouth outlets and the thermal plume are tagged capsules and a disc at their anchors, the laptop fan exhaust a capsule at the vent.", "Thermik: jede Region besteht aus Verweisen auf die tragenden Körper; Hemd, Hose, Schuhe und der Sitzstoff sind verschachtelte Schalen je Schicht, innerste zuerst; Nasen- und Mundauslass sowie die thermische Fahne sind markierte Kapseln und eine Scheibe an ihren Ankern, die Laptop-Abluft eine Kapsel am Auslass."),
        D("Structure: member lofts with a core section along every load-bearing bone of the body (60-80 % of the clothed section), cores of the furniture parts (star arms, column, seat, backrest, desk top and legs, laptop), ball joints at the rig joints and mass ellipsoids at the heavy segments; the load path runs from the floor through castors, star base, column, seat and thighs into the pelvis and trunk, through the neck into the head and through the shoulders and arms to the laptop and the desk, whose legs stand on floor pads.", "Tragwerk: Stab-Lofts mit Kernquerschnitt entlang jedes lastabtragenden Knochens des Körpers (60-80 % des bekleideten Querschnitts), Kerne der Möbelteile (Sternarme, Säule, Sitz, Rücken, Tischplatte und -beine, Laptop), Kugelgelenke an den Rig-Gelenken und Massenellipsoide an den schweren Segmenten; der Lastweg verläuft vom Boden über Rollen, Sternfuss, Säule, Sitz und Oberschenkel in Becken und Rumpf, durch den Hals in den Kopf und über Schultern und Arme zu Laptop und Tisch, dessen Beine auf Bodenplatten stehen."),
    ],
    100: [
        D("1:100 is derived from the 1:50 model and refitted to the 1:100 drawings: loft sections reduced to those that carry the outline, eyes, the seat lever, the laptop hinge and the column collar below 15 mm are absorbed, the shoes and the waist are taken in slightly so that the shirt stays 30 % within its area. Thermal: references and layer shells follow the new solids; structure: member lofts with three sections and joints at all rig joints.", "1:100 ist aus dem 1:50-Modell abgeleitet und an die 1:100-Zeichnungen neu angepasst: Loft-Querschnitte auf die konturtragenden reduziert, Augen, Sitzhebel, Laptop-Scharnier und Säulenmanschette unter 15 mm gehen auf, Schuhe und Taille sind leicht eingezogen, damit das Hemd innerhalb von 30 % seiner Fläche bleibt. Thermik: Verweise und Schichtschalen folgen den neuen Körpern; Tragwerk: Stab-Lofts mit drei Querschnitten und Gelenke an allen Rig-Gelenken."),
    ],
    200: [
        D("1:200 merges: ears, nose, eyes, thumbs, the chair yokes and stems, the seat mechanism and lever, the column collar, the laptop hinge and the desk foot plates (below 30 mm) are absorbed by their neighbours; the desk board, the legs and the laptop keep their drawn thicknesses (25, 40 and 21 mm: the drawings are thinner than the feature, C13), the lid is 30 mm. Thermal: one surface per region; structure: member lofts with two sections, joints only at the main body joints, floor pads under castors and desk legs.", "1:200 fasst zusammen: Ohren, Nase, Augen, Daumen, Stuhlgabeln und -schäfte, Sitzmechanik und -hebel, Säulenmanschette, Laptop-Scharnier und Tischfussplatten (unter 30 mm) gehen in den Nachbarn auf; Tischplatte, Beine und Laptop behalten ihre gezeichneten Dicken (25, 40 und 21 mm: die Zeichnungen sind dünner als das Merkmal, C13), der Deckel ist 30 mm. Thermik: eine Oberfläche je Region; Tragwerk: Stab-Lofts mit zwei Querschnitten, Gelenke nur an den Hauptgelenken des Körpers, Bodenplatten unter Rollen und Tischbeinen."),
    ],
    500: [
        D("1:500 is a generalised silhouette model: the column is a 115 mm rod as drawn, the backrest stands vertical as drawn, the star arms are 31 mm bars on the base bone (the drawn base bar is 30 mm), the desk board is 25 mm and the legs 40 mm, the laptop is a 75 mm base and lid, everything else follows the feature of 75 mm; seat mechanism, armrests, apron and hub details are not drawn and left out. Thermal: one solid per region; structure: member lofts with two sections, the star arms as members of their bones on the base, floor pads under castors and desk legs.", "1:500 ist ein verallgemeinertes Silhouettenmodell: die Säule ist wie gezeichnet ein 115-mm-Stab, die Rückenlehne steht wie gezeichnet senkrecht, die Sternarme sind 31-mm-Stäbe auf dem Fussknochen (der gezeichnete Fussbalken ist 30 mm), die Tischplatte 25 mm und die Beine 40 mm, der Laptop eine 75-mm-Basis mit Deckel, alles andere folgt dem Merkmal von 75 mm; Sitzmechanik, Armlehnen, Zarge und Nabendetails sind nicht gezeichnet und entfallen. Thermik: ein Körper je Region; Tragwerk: Stab-Lofts mit zwei Querschnitten, die Sternarme als Stäbe ihrer Knochen auf dem Fuss, Bodenplatten unter Rollen und Tischbeinen."),
    ],
}


def post_finish(vols):
    st = vols["500"]["structure"]
    arms = [s for s in vols["500"]["body"] if s["id"].startswith("chair-arm-")]
    st[:] = [m for m in st if not (m.get("member", "").startswith("chair-arm-"))]
    for a in arms:
        m = copy.deepcopy(a)
        m["id"] = "member-" + a["id"]
        m["member"], m["part"] = a["id"][0:], "member"
        st.append(m)
    for m in vols["200"]["structure"]:
        if m["id"] == "member-chair-column":
            m["r0"] = max(m["r0"], 0.0128)
            m["r1"] = max(m["r1"], 0.0128)
    for sc in ("50", "100", "200", "500"):
        body = {s["id"]: s for s in vols[sc]["body"]}
        for j in vols[sc]["structure"]:
            if j.get("part") == "joint" and j["bone"].startswith("chair"):
                p = primary(vols[sc]["body"], j["bone"])
                if p is not None and p["kind"] == "loft":
                    q = min(p["sections"], key=lambda q: q["at"][0])
                    r = max(0.75 * min(q["rx"], q["ry"]), 0.5008 * FEATURE[int(sc)])
                    j["radii"] = [min(x, r) for x in j["radii"]]


def post_fit(b, s):
    if s == 500:
        for x in b:
            if x["id"].startswith("chair-arm-") and x["kind"] == "loft":
                flip = "back" in x["id"]
                p = [q["at"] for q in x["sections"]]
                f = (lambda t: [-t[0], -t[1], t[2]]) if flip else (lambda t: list(t))
                new = capsule(x["id"], "chair-base", x["material"], x["tone"], f(p[0]), f(p[-1]), 0.0156)
                x.clear()
                x.update(new)
            if x["id"] == "chair-column":
                x["r0"] = x["r1"] = 0.0575
            if x["id"].startswith("shoe-toe") and x["kind"] == "loft":
                for q in x["sections"]:
                    q["rx"] = max(q["rx"], 0.0392)
                    q["ry"] = max(q["ry"], 0.0392)
                x["sections"][-1]["at"][0] = max(x["sections"][-1]["at"][0], 0.0765)
    if s == 100:
        for x in b:
            if x["id"].startswith("shoe-rear"):
                for q in x["sections"]:
                    q["rx"] *= 0.95
            if x["id"] == "pelvis-trousers":
                for q in x["sections"]:
                    if q["at"][0] > 0.0:
                        q["rx"] *= 0.91
                        q["ry"] *= 0.91
