"""🪜 Hand-authored coarse scales of components/cantilever-balcony (pass 1): the railing, joints, masses and thermal shells of 1:100, 1:200 and 1:500 modelled for 15, 30 and 75 mm features; states the chosen numbers and rewrites the volumes source."""
import json
import sys

path = sys.argv[1]
doc = json.load(open(path))
vol = doc["volumes"]
STEEL = {"bone": "railing", "material": "steel", "role": "body", "tone": 0.3}
COM_SLAB = [0.73411, -0.00542, 0]
COM_RAILING = [0.566, 0.0, 0.0]


def pick(scale, name, solid_id):
    return next(s for s in vol[scale][name] if s["id"] == solid_id)


def box(solid_id, size, at, **more):
    return {"id": solid_id, "kind": "box", **STEEL, "size": size, "transform": {"translation": at}, **more}


def bar(solid_id, radius, start, length, origin_vw, **more):
    return {"id": solid_id, "kind": "revolve", **STEEL, "profile": [[0.0, radius], [length, radius]], "axis": [1.0, 0.0, 0.0], "origin": [start, origin_vw[0], origin_vw[1]], **more}


def handrail(radius, centre):
    return {"id": "railing-handrail", "kind": "revolve", **STEEL, "profile": [[0.0, radius], [1.0, radius]], "axis": [0.0, 0.0, 1.0], "origin": [centre, 0.0, -0.5], "sectionFaces": ["-w", "+w"]}


def ball(solid_id, bone, material, radius, at, member):
    return {"id": solid_id, "kind": "ellipsoid", "bone": bone, "material": material, "role": "detail", "tone": 0.4, "radii": [radius] * 3, "transform": {"translation": at}, "member": member, "part": "mass"}


def chords(thickness, depth, inner, positions):
    out = []
    for name, u in positions:
        half = (0.5 - inner) / 2
        out.append(box(f"railing-{name}-chord-l", [thickness, depth, 0.5 - inner], [u, 0.0, inner + half], sectionFaces=["+w"]))
        out.append(box(f"railing-{name}-chord-r", [thickness, depth, 0.5 - inner], [u, 0.0, -inner - half], sectionFaces=["-w"]))
    return out


def thermal_railing(body):
    return [{"id": "th-" + s["id"], "of": s["id"], "region": "railing-steel", **({"sectionFaces": s["sectionFaces"]} if "sectionFaces" in s else {})} for s in body if s["bone"] == "railing"]


def members(body, joints):
    return [{"id": ("joint-" if s["id"] in joints else "member-") + s["id"], "of": s["id"], "member": "railing", "part": "joint" if s["id"] in joints else "member"} for s in body if s["bone"] == "railing"]


def wall_shells(scale):
    slab_skins = [pick(scale, "thermal", i) for i in ("th-slab-top", "th-slab-soffit", "th-slab-edge")]
    faces = ["-u", "+v", "-v", "-w", "+w"]
    return slab_skins + [
        {"id": "th-wall-concrete", "kind": "box", "bone": "slab", "material": "concrete", "role": "body", "tone": 0.62, "size": [0.18, 1.1, 1.0], "transform": {"translation": [-0.21, 0.65, 0.0]}, "sectionFaces": faces, "region": "wall-face", "layer": "wall-concrete"},
        {"id": "th-wall-wool-render", "kind": "box", "bone": "slab", "material": "mineral-wool", "role": "body", "tone": 0.92, "size": [0.12, 1.1, 1.0], "transform": {"translation": [-0.06, 0.65, 0.0]}, "sectionFaces": faces, "region": "wall-face", "layer": "wall-render"},
        {"id": "th-break-core", "kind": "box", "bone": "slab", "material": "concrete", "role": "body", "tone": 0.62, "size": [0.18, 0.2, 1.0], "transform": {"translation": [-0.21, 0.0, 0.0]}, "sectionFaces": faces, "region": "thermal-break", "layer": "break-concrete"},
    ]


s50 = vol["50"]
s50["structure"] = [s for s in s50["structure"] if s["id"] != "mass-railing"]
s50["structure"].insert(next(i for i, s in enumerate(s50["structure"]) if s["id"] == "mass-slab") + 1, ball("mass-railing", "railing", "steel", 0.03, COM_RAILING, "railing"))

b100 = [pick("100", "body", i) for i in ("slab", "break-module", "wall-core", "wall-insulation-render")]
r100 = [
    box("railing-foot", [0.0272, 0.15, 0.15], [0.0116, 0.0, 0.0]),
    box("railing-post", [1.0165, 0.05, 0.05], [0.53345, 0.0, 0.0]),
    handrail(0.03015, 1.06985),
    *chords(0.015, 0.04, 0.025, (("top", 0.955), ("bottom", 0.085))),
    bar("railing-balusters-l", 0.0075, 0.0925, 0.855, (0.0, 0.12589), role="detail", repeat={"count": 4, "translation": [0.0, 0.0, 0.10689]}),
    bar("railing-balusters-r", 0.0075, 0.0925, 0.855, (0.0, -0.12589), role="detail", repeat={"count": 4, "translation": [0.0, 0.0, -0.10689]}),
]
vol["100"]["body"] = b100 + r100
vol["100"]["thermal"] = [pick("100", "thermal", i) for i in ("th-slab-top", "th-slab-soffit", "th-slab-edge", "th-wall-concrete")] + [pick("50", "thermal", i) for i in ("th-wall-wool", "th-wall-render")] + [pick("100", "thermal", i) for i in ("th-break-core", "th-break-module")] + thermal_railing(r100)
vol["100"]["structure"] = [pick("100", "structure", i) for i in ("slab-member", "support-wall-core", "joint-tension-bars", "joint-pressure-strip", "slab-top-bars")] + [
    ball("mass-slab", "slab", "concrete", 0.05, COM_SLAB, "slab"),
    *members(r100, {"railing-foot"}),
    {"id": "joint-railing-anchors-outer", "kind": "revolve", "bone": "railing", "material": "steel", "role": "body", "tone": 0.2, "profile": [[0.0, 0.0075], [0.09, 0.0075]], "axis": [-1.0, 0.0, 0.0], "origin": [0.0, -0.055, 0.055], "repeat": {"count": 2, "translation": [0.0, 0.0, -0.11]}, "member": "railing", "part": "joint"},
    {"id": "joint-railing-anchors-inner", "kind": "revolve", "bone": "railing", "material": "steel", "role": "body", "tone": 0.2, "profile": [[0.0, 0.0075], [0.09, 0.0075]], "axis": [-1.0, 0.0, 0.0], "origin": [0.0, 0.055, 0.055], "repeat": {"count": 2, "translation": [0.0, 0.0, -0.11]}, "member": "railing", "part": "joint"},
    ball("mass-railing", "railing", "steel", 0.035, COM_RAILING, "railing"),
]

b200 = [pick("200", "body", i) for i in ("wall", "break-module", "slab")]
r200 = [
    box("railing-post", [1.0437, 0.05, 0.05], [0.51985, 0.0, 0.0]),
    handrail(0.03015, 1.06985),
    *chords(0.03, 0.04, 0.025, (("top", 0.955), ("bottom", 0.085))),
    bar("railing-balusters-l", 0.015, 0.1, 0.84, (0.0, 0.12589), role="detail", repeat={"count": 4, "translation": [0.0, 0.0, 0.10689]}),
    bar("railing-balusters-r", 0.015, 0.1, 0.84, (0.0, -0.12589), role="detail", repeat={"count": 4, "translation": [0.0, 0.0, -0.10689]}),
]
fixing200 = {"id": "joint-railing-fixing", "kind": "box", "bone": "railing", "material": "steel", "role": "body", "tone": 0.2, "size": [0.12, 0.15, 0.15], "transform": {"translation": [-0.03, 0.0, 0.0]}, "member": "railing", "part": "joint"}
vol["200"]["body"] = b200 + r200
vol["200"]["thermal"] = wall_shells("200") + [{"id": "th-break-module", "of": "break-module", "region": "thermal-break", "layer": "break-body", "sectionFaces": ["-u", "-w", "+w"]}] + thermal_railing(r200)
vol["200"]["structure"] = [pick("200", "structure", i) for i in ("slab-member", "support-wall-core", "joint-tension-layer", "joint-pressure-layer")] + [
    ball("mass-slab", "slab", "concrete", 0.06, COM_SLAB, "slab"),
    *members(r200, set()),
    fixing200,
    ball("mass-railing", "railing", "steel", 0.04, COM_RAILING, "railing"),
]

b500 = [pick("500", "body", "balcony-solid")]
r500 = [
    box("railing-post", [1.025, 0.075, 0.075], [0.5125, 0.0, 0.0]),
    handrail(0.0375, 1.0625),
    *chords(0.075, 0.075, 0.0375, (("bottom", 0.085),)),
    bar("railing-balusters-l", 0.0375, 0.1225, 0.9025, (0.0, 0.33967), role="detail"),
    bar("railing-balusters-r", 0.0375, 0.1225, 0.9025, (0.0, -0.33967), role="detail"),
]
vol["500"]["body"] = b500 + r500
vol["500"]["thermal"] = wall_shells("500") + [
    {"id": "th-break", "kind": "box", "bone": "slab", "material": "break-module", "role": "body", "tone": 0.42, "size": [0.12, 0.2, 1.0], "transform": {"translation": [-0.06, 0.0, 0.0]}, "sectionFaces": ["-u", "-w", "+w"], "region": "thermal-break", "layer": "break-body"},
] + thermal_railing(r500)
vol["500"]["structure"] = [pick("500", "structure", i) for i in ("slab-member", "joint-wall-connection", "support-wall-core")] + [
    ball("mass-slab", "slab", "concrete", 0.075, COM_SLAB, "slab"),
    *members(r500, set()),
    {**fixing200, "size": [0.15, 0.15, 0.15], "transform": {"translation": [0.0, 0.0, 0.0]}},
    ball("mass-railing", "railing", "steel", 0.06, COM_RAILING, "railing"),
]

MID = {"50": -0.01, "100": -0.01, "200": -0.01, "500": 0.0}
for scale, mid in MID.items():
    top, soffit, edge = (pick(scale, "thermal", i) for i in ("th-slab-top", "th-slab-soffit", "th-slab-edge"))
    tip_top = [1.5, 0.0675] if scale == "50" else [1.5, 0.1 if scale == "500" else 0.0775]
    tip_bottom = [1.5, -0.09] if scale == "50" else [1.5, -0.1]
    arris = [[1.49, 0.07765]] if scale == "50" else []
    groove = [[1.405, -0.1], [1.41, -0.085], [1.425, -0.085], [1.43, -0.1], [1.49, -0.1]] if scale == "50" else []
    top["profile"] = [[0.0, 0.1], [0.0, mid], [1.34, mid], tip_top, *arris]
    top["sectionFaces"] = ["-u", "-v", "-w", "+w"]
    soffit["profile"] = [[0.0, -0.1], *groove, tip_bottom, [1.34, mid], [0.0, mid]]
    soffit["sectionFaces"] = ["-u", "+v", "-w", "+w"]
    edge["profile"] = [tip_top, [1.34, mid], tip_bottom]
    edge["sectionFaces"] = ["-u", "+v", "-v", "-w", "+w"]

D = {}
D["50"] = list(vol["50"]["decisions"][:3])
D["50"][1] = {
    "en": "Thermal: slab top, soffit and edge fill the whole concrete section and meet inside it, top and soffit on the mid-plane 110 mm under the root surface and the edge on two mitres 160 mm behind the face, so each region exposes exactly its own outer face (the chamfers go with top and soffit, the drip groove with the soffit) and no part of the slab is without a region; the faces inside the concrete, the root face at the break and the strip ends are section faces. The wall is three nested plates in their real thicknesses (concrete, wool, render) and counts its rendered outer face; the thermal bridge is the concrete band behind the slab plus the break module, which counts its top, underside and outer face. The railing steel is every steel part; post and handrail are closed bodies here because their bores are sealed air.",
    "de": "Thermisch: Plattenoberseite, -unterseite und -stirn füllen den ganzen Betonquerschnitt und treffen sich in seinem Inneren, Ober- und Unterseite auf der Mittelebene 110 mm unter der Oberfläche an der Wand und die Stirn auf zwei Gehrungen 160 mm hinter der Kante, sodass jede Region genau ihre eigene Außenfläche zeigt (die Fasen gehören zu Ober- und Unterseite, die Tropfnut zur Unterseite) und kein Teil der Platte ohne Region bleibt; die Flächen im Beton, die Wurzelfläche am Dämmelement und die Streifenenden sind Schnittflächen. Die Wand besteht aus drei geschachtelten Platten in echter Dicke (Beton, Wolle, Putz) und zählt ihre verputzte Außenfläche; die Wärmebrücke ist das Betonband hinter der Platte samt Dämmelement, das Oberseite, Unterseite und Außenfläche zählt. Der Geländerstahl ist jedes Stahlteil; Pfosten und Handlauf sind hier geschlossene Körper, weil ihre Hohlräume eingeschlossene Luft sind.",
}
D["50"][2] = {
    "en": D["50"][2]["en"].replace("A ball marks the centre of mass of the slab.", "Two balls mark the masses where they act: 707 kg of slab 734 mm from the wall and 24 kg of railing 566 mm up the post."),
    "de": D["50"][2]["de"].replace("Eine Kugel markiert den Massenschwerpunkt der Platte.", "Zwei Kugeln markieren die Massen an ihrem Ort: 707 kg Platte 734 mm vor der Wand und 24 kg Geländer 566 mm über dem Pfostenfuß."),
}
D["100"] = [
    {"en": "Body for a 15 mm feature: the slab is the plain wedge with its 1.5 % fall (200 mm at the wall, 177.5 mm at the edge) without chamfers and drip groove; the wall is its 180 mm concrete core and one 120 mm plate of insulation with render, the 120 mm break module stays its own body. The railing keeps every member but none below 15 mm: grout bed and base plate are one 150 x 150 x 27 mm foot, the post is a closed 50 mm square that runs into the handrail, the handrail a closed 60.3 mm round, the two flats 40 x 15 mm and the eight balusters 15 mm at the true pitch of 106.9 mm; nuts, studs, cap, bracket and tube walls are left out.", "de": "Körper für 15 mm Mindestmaß: Die Platte ist der einfache Keil mit 1,5 % Gefälle (200 mm an der Wand, 177,5 mm an der Kante) ohne Fasen und Tropfnut; die Wand besteht aus ihrem Betonkern von 180 mm und einer Platte von 120 mm aus Dämmung mit Putz, das Dämmelement von 120 mm bleibt ein eigener Körper. Das Geländer behält jedes Bauteil, aber keines unter 15 mm: Mörtelbett und Fußplatte sind ein Fuß von 150 x 150 x 27 mm, der Pfosten ist ein geschlossenes Quadrat von 50 mm bis in den Handlauf, der Handlauf ein geschlossener Rundstab von 60,3 mm, die zwei Flachstähle messen 40 x 15 mm und die acht Füllstäbe 15 mm im wahren Raster von 106,9 mm; Muttern, Bolzen, Deckel, Träger und Rohrwandungen entfallen."},
    {"en": "Thermal: top, soffit and edge of the slab fill the wedge and meet on its mid-plane and on two mitres; the wall is three nested plates because the 20 mm render still exceeds the feature size (concrete core, wool, render), the thermal bridge the concrete band behind the slab plus the break module. The railing steel is the body railing, part for part.", "de": "Thermisch: Oberseite, Unterseite und Stirn der Platte füllen den Keil und treffen sich auf seiner Mittelebene und auf zwei Gehrungen; die Wand besteht aus drei geschachtelten Platten, weil der Putz von 20 mm noch über dem Mindestmaß liegt (Betonkern, Wolle, Putz), die Wärmebrücke aus dem Betonband hinter der Platte samt Dämmelement. Der Geländerstahl ist das Geländer des Körpers, Teil für Teil."},
    {"en": "Structure: the slab is the member and the wall core its fixed support. Across the insulation run five straight 12 mm tension bars in the top layer, lapped 700 mm into the slab, and one pressure strip at the bottom that stands for the four bearings; the five top bars of the slab follow the fall. The railing member is post, handrail, flats and balusters; its joint is the foot with four 15 mm anchors 90 mm deep. Balls mark the masses of slab and railing.", "de": "Tragwerk: Die Platte ist der Stab, der Wandkern ihr festes Auflager. Durch die Dämmung laufen fünf gerade Zugstäbe von 12 mm in der oberen Lage, 700 mm in die Platte übergreifend, und unten ein Druckstreifen, der für die vier Drucklager steht; die fünf oberen Stäbe der Platte folgen dem Gefälle. Der Geländerstab ist Pfosten, Handlauf, Flachstähle und Füllstäbe; sein Gelenk ist der Fuß mit vier Ankern von 15 mm, 90 mm tief. Kugeln markieren die Massen von Platte und Geländer."},
]
D["200"] = [
    {"en": "Body for a 30 mm feature: wall stub (300 mm with its notch for the break), the 120 mm break module and the slab wedge are three masses. The railing is reduced to what 30 mm can carry: a closed 50 mm post from the slab into the handrail without a foot, the closed 60.3 mm handrail with its top at 1.10 m, two rails 40 x 30 mm and the eight balusters thickened to 30 mm at their true pitch, so the bar field still reads as bars and matches the pen lines of the drawing.", "de": "Körper für 30 mm Mindestmaß: Wandscheibe (300 mm mit Aussparung für das Dämmelement), das Dämmelement von 120 mm und der Plattenkeil sind drei Massen. Das Geländer ist auf das reduziert, was 30 mm tragen: ein geschlossener Pfosten von 50 mm von der Platte bis in den Handlauf ohne Fuß, der geschlossene Handlauf von 60,3 mm mit Oberkante auf 1,10 m, zwei Holme 40 x 30 mm und die acht Füllstäbe, auf 30 mm verstärkt, im wahren Raster, damit das Stabfeld als Stäbe lesbar bleibt und zu den Federlinien der Zeichnung passt."},
    {"en": "Thermal: the slab is divided into top, soffit and edge on its mid-plane and two mitres; the wall into concrete core and insulated outer skin, the thermal bridge into the concrete band and the 120 mm break body. The railing steel is the body railing.", "de": "Thermisch: Die Platte ist auf ihrer Mittelebene und zwei Gehrungen in Oberseite, Unterseite und Stirn geteilt; die Wand in Betonkern und gedämmte Außenhaut, die Wärmebrücke in das Betonband und den Dämmkörper von 120 mm. Der Geländerstahl ist das Geländer des Körpers."},
    {"en": "Structure: slab member on the fixed wall core; the connection is a 30 mm tension layer at the top that reaches 250 mm to the wall side and 300 mm into the slab and a 30 mm pressure layer at the bottom, the couple that carries the root moment. The railing member stands on a 150 mm fixing block that reaches 90 mm into the slab. Balls mark the masses of slab and railing.", "de": "Tragwerk: Plattenstab am festen Wandkern; der Anschluss ist eine Zuglage von 30 mm oben, die 250 mm zur Wandseite und 300 mm in die Platte reicht, und eine Drucklage von 30 mm unten, das Kräftepaar für das Einspannmoment. Der Geländerstab steht auf einem Befestigungsblock von 150 mm, der 90 mm in die Platte reicht. Kugeln markieren die Massen von Platte und Geländer."},
]
D["500"] = [
    {"en": "Body for a 75 mm feature: wall stub, break and a flat 200 mm slab are one L-shaped mass, as in the drawing. The railing is a sign of a railing at true centre lines and true height: a 75 mm post, a 75 mm handrail with its top at 1.10 m, one 75 mm bottom rail 85 mm above the post foot and two 75 mm balusters a third of a metre either side of the post; the upper flat is merged into the handrail.", "de": "Körper für 75 mm Mindestmaß: Wandscheibe, Dämmelement und eine ebene Platte von 200 mm sind eine L-förmige Masse wie in der Zeichnung. Das Geländer ist ein Zeichen eines Geländers mit wahren Achsen und wahrer Höhe: ein Pfosten von 75 mm, ein Handlauf von 75 mm mit Oberkante auf 1,10 m, ein unterer Holm von 75 mm 85 mm über dem Pfostenfuß und zwei Füllstäbe von 75 mm je ein Drittel Meter neben dem Pfosten; der obere Flachstahl geht im Handlauf auf."},
    {"en": "Thermal: the single mass is still divided into its regions, because the thermal bridge is the point of the asset: slab top, soffit and edge on the mid-plane and two mitres, the wall as concrete core and 120 mm insulated skin, the bridge as concrete band and 120 mm break body. The railing steel is the body railing.", "de": "Thermisch: Die eine Masse bleibt in ihre Regionen geteilt, weil die Wärmebrücke der Sinn des Bauteils ist: Plattenoberseite, -unterseite und -stirn auf der Mittelebene und zwei Gehrungen, die Wand als Betonkern und gedämmte Haut von 120 mm, die Brücke als Betonband und Dämmkörper von 120 mm. Der Geländerstahl ist das Geländer des Körpers."},
    {"en": "Structure: a 200 mm slab member, the 120 mm wall connection as its joint and the 180 mm wall core as the fixed support; the railing member stands on a 150 mm fixing block. Balls of 150 and 120 mm mark the masses of slab and railing.", "de": "Tragwerk: ein Plattenstab von 200 mm, der Wandanschluss von 120 mm als sein Gelenk und der Wandkern von 180 mm als festes Auflager; der Geländerstab steht auf einem Befestigungsblock von 150 mm. Kugeln von 150 und 120 mm markieren die Massen von Platte und Geländer."},
]
for scale, texts in D.items():
    vol[scale]["decisions"] = texts

json.dump(doc, open(path, "w"), ensure_ascii=False, indent=1)
print("[DEBUG] written", path, {k: {n: len(v[n]) for n in ("body", "thermal", "structure")} for k, v in vol.items()})
