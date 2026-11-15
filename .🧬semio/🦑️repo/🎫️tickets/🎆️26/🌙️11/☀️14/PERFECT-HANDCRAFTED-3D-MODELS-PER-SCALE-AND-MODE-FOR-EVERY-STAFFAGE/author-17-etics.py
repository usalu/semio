import os, sys
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())
R = "root"
PL, BR, WO, RE = "lime-plaster", "solid-brick", "mineral-wool", "mineral-render"

def build(slug, ti):
    xb, xi, xr = 0.38, 0.38 + ti, 0.38 + ti + 0.02
    total = xr
    ins_en = f"{int(round(ti*1000))} mm"
    sc = []
    # ---- 1:50
    s = Scale(50)
    s.b(box("plaster", R, PL, 0.85, [0.015, 1, 1], [0.0075, 0.5, 0]))
    ys = [(0, 0.07)] + [(0.075 * j + 0.005, 0.075 * (j + 1) - 0.005) for j in range(1, 12)] + [(0.905, 1.0)]
    for j, (a, b) in enumerate(ys):
        s.b(box(f"brick-course-{j:02d}", R, BR, 0.5, [0.365, b - a, 1], [0.1975, (a + b) / 2, 0]))
    for k in range(1, 13):
        s.b(box(f"bed-joint-{k:02d}", R, BR, 0.78, [0.365, 0.01, 1], [0.1975, 0.075 * k, 0], tags=["mortar"]))
    s.b(box("insulation", R, WO, 0.9, [ti, 1, 1], [xb + ti / 2, 0.5, 0]))
    s.b(box("render", R, RE, 0.8, [0.02, 1, 1], [xi + 0.01, 0.5, 0]))
    s.thof("th-plaster", "plaster", "wall-field", "plaster")
    s.th(box("th-brick", R, BR, 0.5, [0.365, 1, 1], [0.1975, 0.5, 0]), "wall-field", "brick")
    s.thof("th-insulation", "insulation", "wall-field", "insulation")
    s.thof("th-render", "render", "wall-field", "render")
    for j in range(13):
        s.stof(f"member-brick-course-{j:02d}", f"brick-course-{j:02d}", "root", "member")
    s.st(box("support-base-fixed", R, BR, 0.2, [0.365, 0.0075, 1], [0.1975, 0.00375, 0]), "root", "support")
    s.dec(f"Body: the wall as built, layer by layer: 15 mm lime plaster, the solid brick leaf as 13 courses with 12 mortar bed joints of 10 mm (tone lighter, as the joint lines of the drawing), {ins_en} mineral wool and 20 mm mineral render, all full 1 m x 1 m.",
          f"Körper: die Wand wie gebaut, Schicht für Schicht: 15 mm Kalkputz, die Vollziegelschale als 13 Schichten mit 12 Lagerfugen zu 10 mm (heller getönt, wie die Fugenlinien der Zeichnung), {ins_en} Mineralwolle und 20 mm mineralischer Putz, alle voll 1 m x 1 m.")
    s.dec("Thermal: the layer shells of a planar wall, innermost first (plaster, brick, insulation, render), stacked as slabs of their real thicknesses; plaster, insulation and render are referenced from the body, the brick layer is one solid slab (the bed joints lie inside the layer).",
          "Thermisch: die Schichthüllen einer ebenen Wand von innen nach außen (Putz, Ziegel, Dämmung, Außenputz), als Platten ihrer echten Dicken gestapelt; Putz, Dämmung und Außenputz werden aus dem Körper referenziert, die Ziegelschicht ist eine durchgehende Platte (die Lagerfugen liegen in der Schicht).")
    s.dec("Structure: the 365 mm brick leaf (its 13 courses) is the only load-bearing member (insulation and render are cladding and absent); the fixed support is a 7.5 mm bearing block under the leaf at the base anchor.",
          "Tragwerk: die Ziegelschale von 365 mm (die 13 Schichten) ist das einzige tragende Bauteil (Dämmung und Putz sind Bekleidung und fehlen); das feste Auflager ist ein Lagerblock von 7,5 mm unter der Schale am Fußanker.")
    sc.append(s)
    # ---- 1:100
    s = Scale(100)
    s.b(box("brick", R, BR, 0.5, [0.38, 1, 1], [0.19, 0.5, 0]))
    s.b(box("insulation", R, WO, 0.9, [ti, 1, 1], [xb + ti / 2, 0.5, 0]))
    s.b(box("render", R, RE, 0.8, [0.02, 1, 1], [xi + 0.01, 0.5, 0]))
    s.th(box("th-wall-core", R, BR, 0.5, [xb, 1, 1], [xb / 2, 0.5, 0]), "wall-field", "brick")
    s.th(box("th-wall-insulation", R, WO, 0.9, [ti, 1, 1], [xb + ti / 2, 0.5, 0]), "wall-field", "insulation")
    s.th(box("th-wall-render", R, RE, 0.8, [0.02, 1, 1], [xi + 0.01, 0.5, 0]), "wall-field", "render")
    s.st(box("member-brick-leaf", R, BR, 0.5, [0.38, 1, 1], [0.19, 0.5, 0]), "root", "member")
    s.st(box("support-base-fixed", R, BR, 0.2, [0.38, 0.015, 1], [0.19, 0.0075, 0]), "root", "support")
    s.dec("Body: plaster, courses and bed joints are merged into one 380 mm masonry slab (the drawing hatches the leaf as one); insulation and the 20 mm render stay separate, the render being above the 15 mm feature.",
          "Körper: Putz, Schichten und Lagerfugen sind zu einer Mauerwerksplatte von 380 mm zusammengefasst (die Zeichnung schraffiert die Schale als eine); Dämmung und der 20-mm-Putz bleiben getrennt, der Putz liegt über der Einzelheit 15 mm.")
    s.dec("Thermal: three stacked slabs (masonry core, insulation, render) tagged with the layers the drawing keeps; the plaster is merged into the core. Structure: the leaf as one 380 mm member; the base support is a 15 mm block.",
          "Thermisch: drei gestapelte Platten (Mauerkern, Dämmung, Putz) mit den Schichten, die die Zeichnung behält; der Putz ist im Kern aufgegangen. Tragwerk: die Schale als ein Stab von 380 mm; das Fußauflager ist ein Block von 15 mm.")
    sc.append(s)
    # ---- 1:200
    s = Scale(200)
    tw = ti + 0.02
    s.b(box("brick", R, BR, 0.5, [0.38, 1, 1], [0.19, 0.5, 0]))
    s.b(box("insulation", R, WO, 0.9, [tw, 1, 1], [xb + tw / 2, 0.5, 0]))
    s.th(box("th-wall-masonry", R, BR, 0.5, [xb, 1, 1], [xb / 2, 0.5, 0]), "wall-field", "brick")
    s.th(box("th-wall-clad", R, WO, 0.9, [tw, 1, 1], [xb + tw / 2, 0.5, 0]), "wall-field", "insulation")
    s.st(box("member-brick-leaf", R, BR, 0.5, [0.38, 1, 1], [0.19, 0.5, 0]), "root", "member")
    s.st(box("support-base-fixed", R, BR, 0.2, [0.38, 0.03, 1], [0.19, 0.015, 0]), "root", "support")
    s.dec(f"Body: masonry leaf and one {int(round(tw*1000))} mm insulation slab that includes the 20 mm render (under the 30 mm feature, as drawn at 1:200).",
          f"Körper: Mauerschale und eine Dämmplatte von {int(round(tw*1000))} mm, die den 20-mm-Putz enthält (unter der Einzelheit 30 mm, wie bei 1:200 gezeichnet).")
    s.dec("Thermal: two stacked slabs, masonry and cladding (insulation with the render merged in). Structure at this scale is one member and one support. Structure: the same leaf member; the base support grows to the 30 mm feature.",
          "Thermisch: zwei gestapelte Platten, Mauerwerk und Bekleidung (Dämmung mit dem Putz darin). Das Tragwerk ist hier bereits minimal: ein Stab und ein Auflager. Tragwerk: derselbe Schalenstab; das Fußauflager wächst auf die Einzelheit 30 mm.")
    s.dec("Minimal: one member and one support are the whole load path of a plain leaf; nothing coarser exists.","Minimal: ein Stab und ein Auflager sind der ganze Lastpfad einer glatten Schale; gröber geht es nicht.")
    sc.append(s)
    # ---- 1:500
    s = Scale(500)
    s.b(box("wall", R, BR, 0.55, [total, 1, 1], [total / 2, 0.5, 0]))
    s.th(box("th-wall", R, BR, 0.55, [total, 1, 1], [total / 2, 0.5, 0]), "wall-field")
    s.st(box("member-wall", R, BR, 0.5, [0.38, 1, 1], [0.19, 0.5, 0]), "root", "member")
    s.st(box("support-base-fixed", R, BR, 0.2, [0.38, 0.075, 1], [0.19, 0.0375, 0]), "root", "support")
    s.dec(f"Body: the whole {int(round(total*1000))} mm wall is one block (the 1:500 drawing is one rectangle); layers are not distinguishable at 75 mm.",
          f"Körper: die ganze Wand von {int(round(total*1000))} mm ist ein Block (die Zeichnung 1:500 ist ein einziges Rechteck); Schichten sind bei 75 mm nicht unterscheidbar.")
    s.dec("Thermal: one wall solid carries the whole region. Structure is already minimal: one member and one support. Structure: the 380 mm masonry leaf as the member inside the block, the base support a 75 mm block.",
          "Thermisch: ein Wandkörper trägt die ganze Region. Das Tragwerk ist bereits minimal: ein Stab und ein Auflager. Tragwerk: die 380 mm starke Mauerwerksschale als Stab im Block, das Fußauflager ein Block von 75 mm.")
    sc.append(s)
    sc[-1].dec("Minimal: one block, one thermal solid, one member, one support.","Minimal: ein Block, ein Thermalkörper, ein Stab, ein Auflager.")
    for s_ in sc:
        for lst in (s_.body, s_.thermal):
            mark(lst, ["-v", "+v", "-w", "+w"])
    save("components", slug, sc)

if __name__ == "__main__":
    build("etics-brick-wall-12", 0.12)
    build("etics-brick-wall-30", 0.30)
