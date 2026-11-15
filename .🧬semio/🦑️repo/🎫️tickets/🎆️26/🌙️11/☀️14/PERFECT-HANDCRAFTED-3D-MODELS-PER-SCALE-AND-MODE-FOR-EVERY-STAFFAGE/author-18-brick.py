import sys, os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-18-lib.py")).read())
R = "root"
PL, BR, RE = "lime-plaster", "solid-brick", "mineral-render"
WF = "wall-field"
LAT = ["-v", "+v", "-w", "+w"]


def lat(s_, extra=None):
    for d in s_.thermal:
        if "of" not in d:
            d["sectionFaces"] = LAT + (extra or [])
    return s_


def build():
    sc = []
    s = Scale(50)
    s.b(box("plaster", R, PL, 0.85, [0.015, 1, 1], [0.0075, 0.5, 0]))
    ys = [(0, 0.07)] + [(0.075 * j + 0.005, 0.075 * (j + 1) - 0.005) for j in range(1, 12)] + [(0.905, 1.0)]
    for j, (a, b) in enumerate(ys):
        s.b(box(f"brick-course-{j:02d}", R, BR, 0.5, [0.365, b - a, 1], [0.1975, (a + b) / 2, 0]))
    for k in range(1, 13):
        s.b(box(f"bed-joint-{k:02d}", R, BR, 0.78, [0.365, 0.01, 1], [0.1975, 0.075 * k, 0], tags=["mortar"]))
    s.b(box("render", R, RE, 0.8, [0.02, 1, 1], [0.39, 0.5, 0]))
    s.thof("th-plaster", "plaster", WF, "plaster")
    s.th(box("th-brick", R, BR, 0.5, [0.365, 1, 1], [0.1975, 0.5, 0]), WF, "brick")
    s.thof("th-render", "render", WF, "render")
    for j in range(13):
        s.stof(f"member-brick-course-{j:02d}", f"brick-course-{j:02d}", "root", "member")
    s.st(box("support-base-fixed", R, BR, 0.2, [0.365, 0.0076, 1], [0.1975, 0.0038, 0]), "root", "support")
    s.dec("Body: the wall as built, layer by layer: 15 mm lime plaster on the room side, the 365 mm solid brick leaf as 13 courses of 65 mm (the lowest and the highest cut to the base and the top of the 1 m sample) with 12 mortar bed joints of 10 mm (lighter tone, the joint lines of the drawing), and 20 mm mineral render outside; the head joints are inside the cut and not drawn, so the courses run through. All full 1 m x 1 m, the cut edges are section faces.",
          "Körper: die Wand wie gebaut, Schicht für Schicht: 15 mm Kalkputz raumseitig, die 365 mm starke Vollziegelschale als 13 Schichten zu 65 mm (die unterste und die oberste an Fuß und Kopf der 1-m-Probe abgeschnitten) mit 12 Lagerfugen zu 10 mm (heller getönt, die Fugenlinien der Zeichnung) und 20 mm mineralischer Putz außen; die Stoßfugen liegen im Schnitt und sind nicht gezeichnet, die Schichten laufen also durch. Alle voll 1 m x 1 m, die Schnittkanten sind Schnittflächen.")
    s.dec("Thermal: the three layer shells of a planar wall, innermost first (plaster, brick, render), stacked as slabs of their real thicknesses; plaster and render are referenced from the body, the brick layer is one solid slab (the bed joints lie inside the layer). The area of the region is the outer face of the render (the cut edges carry no area).",
          "Thermisch: die drei Schichthüllen einer ebenen Wand von innen nach außen (Putz, Ziegel, Außenputz), als Platten ihrer echten Dicken gestapelt; Putz und Außenputz werden aus dem Körper referenziert, die Ziegelschicht ist eine durchgehende Platte (die Lagerfugen liegen in der Schicht). Die Fläche der Region ist die Außenseite des Außenputzes (die Schnittkanten tragen keine Fläche).")
    s.dec("Structure: the 365 mm brick leaf (its 13 courses, each a compression member of 365 x 65 mm over the metre) is the load path; plaster and render are finishes and absent; the fixed support is a bearing block under the leaf at the base anchor.",
          "Tragwerk: die Ziegelschale von 365 mm (ihre 13 Schichten, je ein Druckglied 365 x 65 mm über den Meter) ist der Lastpfad; Putz und Außenputz sind Oberflächen und fehlen; das feste Auflager ist ein Lagerblock unter der Schale am Fußanker.")
    sc.append(s)
    s = Scale(100)
    s.b(box("brick", R, BR, 0.5, [0.38, 1, 1], [0.19, 0.5, 0]))
    s.b(box("render", R, RE, 0.8, [0.02, 1, 1], [0.39, 0.5, 0]))
    s.th(box("th-wall-core", R, BR, 0.5, [0.38, 1, 1], [0.19, 0.5, 0]), WF, "brick")
    s.th(box("th-wall-render", R, RE, 0.8, [0.02, 1, 1], [0.39, 0.5, 0]), WF, "render")
    s.st(box("member-brick-leaf", R, BR, 0.5, [0.38, 1, 1], [0.19, 0.5, 0]), "root", "member")
    s.st(box("support-base-fixed", R, BR, 0.2, [0.38, 0.0152, 1], [0.19, 0.0076, 0]), "root", "support")
    s.dec("Body: plaster, courses and bed joints are merged into one 380 mm masonry slab (the drawing hatches the leaf as one); the 20 mm render stays separate, above the 15 mm feature.",
          "Körper: Putz, Schichten und Lagerfugen sind zu einer Mauerwerksplatte von 380 mm zusammengefasst (die Zeichnung schraffiert die Schale als eine); der 20-mm-Putz bleibt getrennt, über der Einzelheit 15 mm.")
    s.dec("Thermal: two stacked slabs (masonry core with the plaster merged in, render) carrying the layers the drawing keeps. Structure: the leaf as one 380 mm member; the base support is a 15 mm block.",
          "Thermisch: zwei gestapelte Platten (Mauerkern mit dem Putz darin, Außenputz) mit den Schichten, die die Zeichnung behält. Tragwerk: die Schale als ein Stab von 380 mm; das Fußauflager ist ein Block von 15 mm.")
    sc.append(s)
    s = Scale(200)
    s.b(box("brick", R, BR, 0.5, [0.37, 1, 1], [0.185, 0.5, 0]))
    s.b(box("render", R, RE, 0.8, [0.03, 1, 1], [0.385, 0.5, 0]))
    s.th(box("th-wall-masonry", R, BR, 0.5, [0.37, 1, 1], [0.185, 0.5, 0]), WF, "brick")
    s.th(box("th-wall-render", R, RE, 0.8, [0.03, 1, 1], [0.385, 0.5, 0]), WF, "render")
    s.st(box("member-brick-leaf", R, BR, 0.5, [0.37, 1, 1], [0.185, 0.5, 0]), "root", "member")
    s.st(box("support-base-fixed", R, BR, 0.2, [0.37, 0.0302, 1], [0.185, 0.0151, 0]), "root", "support")
    s.dec("Body: masonry leaf of 370 mm and a render skin of 30 mm (the smallest feature; it takes the outer 10 mm of the masonry and the 20 mm render, the drawing is one 400 mm rectangle).",
          "Körper: Mauerschale von 370 mm und eine Putzhaut von 30 mm (die kleinste Einzelheit; sie nimmt die äußeren 10 mm des Mauerwerks und den 20-mm-Putz auf, die Zeichnung ist ein Rechteck von 400 mm).")
    s.dec("Thermal: two stacked slabs, masonry and render skin. Structure: the same 370 mm leaf member; the base support grows to the 30 mm feature.",
          "Thermisch: zwei gestapelte Platten, Mauerwerk und Putzhaut. Tragwerk: derselbe Schalenstab von 370 mm; das Fußauflager wächst auf die Einzelheit 30 mm.")
    s.dec("Minimal: a masonry block, a render skin, one member and one support are the whole of a plain leaf; nothing coarser keeps the layers.", "Minimal: ein Mauerblock, eine Putzhaut, ein Stab und ein Auflager sind die ganze glatte Schale; gröber bleiben die Schichten nicht erhalten.")
    sc.append(s)
    s = Scale(500)
    s.b(box("wall", R, BR, 0.55, [0.4, 1, 1], [0.2, 0.5, 0]))
    d = box("th-wall", R, BR, 0.55, [0.4, 1, 1], [0.2, 0.5, 0])
    s.th(d, WF)
    s.st(box("member-wall", R, BR, 0.5, [0.38, 1, 1], [0.19, 0.5, 0]), "root", "member")
    s.st(box("support-base-fixed", R, BR, 0.2, [0.38, 0.0752, 1], [0.19, 0.0376, 0]), "root", "support")
    s.dec("Body: the whole 400 mm wall is one block (the 1:500 drawing is one rectangle); layers are not distinguishable at 75 mm.",
          "Körper: die ganze Wand von 400 mm ist ein Block (die Zeichnung 1:500 ist ein einziges Rechteck); Schichten sind bei 75 mm nicht unterscheidbar.")
    s.dec("Thermal: one wall solid carries the whole region (its room face carries no area). Structure: the 380 mm masonry leaf as the member inside the block, the base support a 75 mm block.",
          "Thermisch: ein Wandkörper trägt die ganze Region (seine Raumseite trägt keine Fläche). Tragwerk: die 380 mm starke Mauerwerksschale als Stab im Block, das Fußauflager ein Block von 75 mm.")
    s.dec("Minimal: one block, one thermal solid, one member, one support.", "Minimal: ein Block, ein Thermalkörper, ein Stab, ein Auflager.")
    sc.append(s)
    for s_ in sc:
        lat(s_)
        for d in s_.body:
            d["sectionFaces"] = LAT
    sc[3].thermal[0]["sectionFaces"] = LAT + ["-u"]
    for k in (1, 2):
        for d in sc[k].thermal:
            if d["id"] in ("th-wall-core", "th-wall-masonry"):
                d["sectionFaces"] = LAT + ["-u"]
    return sc


SC = build()
if __name__ == "__main__":
    save_asset("solid-brick-wall-1960", SC)
