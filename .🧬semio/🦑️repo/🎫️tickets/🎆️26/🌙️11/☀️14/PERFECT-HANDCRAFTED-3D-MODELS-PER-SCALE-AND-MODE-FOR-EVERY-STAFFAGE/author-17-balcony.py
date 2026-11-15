import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())
SL, RL = "slab", "railing"
CO, ST, WL, RN, BM = "concrete", "steel", "mineral-wool", "render", "break-module"
sc = []

def post_profile(h):
    return rect(0, 0, 0.05, 0.05), [rect(0, 0, 0.042, 0.042)] if h else None

def rebars(s, k, f):
    d = 0.012 if k == 50 else f
    n = 5 if k == 50 else 3
    step = 0.2 if k == 50 else 0.4
    z0 = -0.4 if k == 50 else -0.4
    rp = {"count": n, "translation": [0, 0, step]}
    s.st(cyl("rebar-top", SL, ST, 0.25, "x", [0.6, 0.065, z0], 1.7, d / 2, repeat=rp), "slab", "member")
    s.st(cyl("rebar-bottom", SL, ST, 0.25, "x", [0.5, -0.065, z0], 1.0, d / 2, repeat=rp), "slab", "member")
    s.st(cyl("wall-rebar-inner", SL, ST, 0.25, "y", [-0.27, 0.65, z0], 1.0, d / 2, repeat=rp), "slab", "member")
    s.st(cyl("wall-rebar-outer", SL, ST, 0.25, "y", [-0.15, 0.65, z0], 1.0, d / 2, repeat=rp), "slab", "member")
    if k == 50:
        s.st(cyl("rebar-top-hook", SL, ST, 0.25, "y", [-0.25, 0.0075, z0], 0.115, d / 2, repeat=rp), "slab", "member")

# ---------------- 1:50
s = Scale(50)
slab_prof = [[0, -0.1], [1.42, -0.1], [1.42, -0.08], [1.44, -0.08], [1.44, -0.1], [1.5, -0.1], [1.5, 0.1], [0, 0.1]]
s.b(ext("slab", SL, CO, 0.7, "xy", slab_prof, -0.5, 0.5))
s.b(box("break-module", SL, BM, 0.8, [0.12, 0.2, 1], [-0.06, 0, 0]))
s.b(box("wall-core-band", SL, CO, 0.7, [0.18, 0.2, 1], [-0.21, 0, 0]))
s.b(box("wall-core-upper", SL, CO, 0.7, [0.18, 1.1, 1], [-0.21, 0.65, 0]))
s.b(box("wall-insulation", SL, WL, 0.9, [0.1, 1.1, 1], [-0.07, 0.65, 0]))
s.b(box("wall-render", SL, RN, 0.85, [0.02, 1.1, 1], [-0.01, 0.65, 0]))
s.b(box("railing-base-plate", RL, ST, 0.3, [0.012, 0.1, 0.1], [0.006, 0, 0.44], repeat={"count": 2, "translation": [0, 0, -0.88]}))
for nm, z in (("a", 0.405), ("b", 0.475)):
    s.b(cyl(f"railing-bolts-l-{nm}", RL, ST, 0.2, "x", [0.018, 0.035, z], 0.012, 0.01, role="detail", repeat={"count": 2, "translation": [0, -0.07, 0]}))
    s.b(cyl(f"railing-bolts-r-{nm}", RL, ST, 0.2, "x", [0.018, 0.035, -z], 0.012, 0.01, role="detail", repeat={"count": 2, "translation": [0, -0.07, 0]}))
p, h = post_profile(True)
s.b(ext("railing-post", RL, ST, 0.3, "yz", p, 0.012, 1.04, holes=h, at=[0, 0, -0.44], repeat={"count": 2, "translation": [0, 0, 0.88]}))
s.b(cyl("railing-handrail", RL, ST, 0.3, "z", [1.07, 0, 0], 1.0, 0.03))
s.b(cyl("railing-mid-rail", RL, ST, 0.3, "z", [0.55, 0, 0], 0.88, 0.02))
s.th(box("th-slab-top", SL, CO, 0.7, [1.5, 0.05, 1], [0.75, 0.075, 0]), "slab-top")
s.th(box("th-slab-soffit", SL, CO, 0.7, [1.5, 0.05, 1], [0.75, -0.075, 0]), "slab-soffit")
s.th(box("th-slab-edge", SL, CO, 0.7, [0.05, 0.2, 1], [1.475, 0, 0]), "slab-edge")
for i, r_ in (("post", "railing-post"), ("handrail", "railing-handrail"), ("mid-rail", "railing-mid-rail"), ("base-plate", "railing-base-plate")):
    s.thof(f"th-railing-{i}", r_, "railing-steel")
s.thof("th-break-core", "wall-core-band", "thermal-break", "break-concrete")
s.thof("th-break-module", "break-module", "thermal-break", "break-body")
s.thof("th-wall-concrete", "wall-core-upper", "wall-face", "wall-concrete")
s.thof("th-wall-wool", "wall-insulation", "wall-face", "wall-wool")
s.thof("th-wall-render", "wall-render", "wall-face", "wall-render")
s.st(box("slab-member", SL, CO, 0.7, [1.5, 0.2, 1], [0.75, 0, 0]), "slab", "member")
rebars(s, 50, s.feature)
s.stof("railing-post-member", "railing-post", "railing", "member")
s.stof("railing-handrail-member", "railing-handrail", "railing", "member")
s.st(box("joint-load-carrier", "load-carrier", ST, 0.3, [0.05, 0.05, 0.05], [0, 0.075, 0]), "load-carrier", "joint")
s.st(box("support-wall-fix-block", SL, ST, 0.15, [0.03, 0.2, 0.9], [0, 0, 0]), "slab", "support")
s.dec("Body: the wall strip and the cantilever as built: 180 mm concrete wall core in two pieces (slab band and wall above), 100 mm mineral wool, 20 mm render, the 120 mm thermal-break module between wall and slab, the 200 mm slab as one extrusion with the 20 mm drip groove, and the railing on its 90-degree bone: two 50 x 50 x 4 mm hollow-section posts, 12 mm base plates with eight 12 mm bolts, the 60 mm handrail tube and the 40 mm mid-rail.",
      "Körper: Wandstreifen und Kragplatte wie gebaut: Betonwandkern 180 mm in zwei Teilen (Plattenband und Wand darüber), 100 mm Mineralwolle, 20 mm Putz, das Dämmelement von 120 mm zwischen Wand und Platte, die Platte von 200 mm als eine Extrusion mit der Tropfnase von 20 mm, und das Geländer auf seinem 90-Grad-Knochen: zwei Hohlprofilpfosten 50 x 50 x 4 mm, Fußplatten von 12 mm mit acht Schrauben von 12 mm, der Handlauf von 60 mm und der Mittelholm von 40 mm.")
s.dec("Thermal: slab top, soffit and edge as 50 mm concrete skins; the layered wall and the thermal break are referenced from the body solids with their layers (concrete, wool, render; concrete, break module); every railing part is referenced.",
      "Thermisch: Plattenoberseite, -unterseite und -stirn als Betonhäute von 50 mm; die geschichtete Wand und das Dämmelement werden mit ihren Schichten aus den Körpern referenziert (Beton, Wolle, Putz; Beton, Dämmelement); jedes Geländerteil wird referenziert.")
s.dec("Structure: the 1 m x 200 mm slab strip as the member, its reinforcement as 12 mm bars (top and bottom mat, wall bars with the hook), the SHS posts and the handrail as the railing members, a slider joint at the load carrier, and the fixed support as a steel block across the slab at the wall face.",
      "Tragwerk: der Plattenstreifen 1 m x 200 mm als Stab, seine Bewehrung als Stäbe von 12 mm (obere und untere Lage, Wandstäbe mit Haken), die Hohlprofilpfosten und der Handlauf als Geländerstäbe, ein Gleitgelenk am Lastträger und das feste Auflager als Stahlblock über die Platte an der Wandfläche.")
sc.append(s)

# ---------------- 1:100
s = Scale(100)
s.b(box("slab", SL, CO, 0.7, [1.5, 0.2, 1], [0.75, 0, 0]))
s.b(box("break-module", SL, BM, 0.8, [0.12, 0.2, 1], [-0.06, 0, 0]))
s.b(box("wall-core", SL, CO, 0.7, [0.18, 1.3, 1], [-0.21, 0.55, 0]))
s.b(box("wall-insulation-render", SL, WL, 0.9, [0.12, 1.1, 1], [-0.06, 0.65, 0]))
s.b(box("railing-post", RL, ST, 0.3, [1.04, 0.05, 0.05], [0.52, 0, -0.44], repeat={"count": 2, "translation": [0, 0, 0.88]}))
s.b(cyl("railing-handrail", RL, ST, 0.3, "z", [1.07, 0, 0], 1.0, 0.03))
s.b(cyl("railing-mid-rail", RL, ST, 0.3, "z", [0.55, 0, 0], 0.88, 0.02))
s.th(box("th-slab-top", SL, CO, 0.7, [1.5, 0.05, 1], [0.75, 0.075, 0]), "slab-top")
s.th(box("th-slab-soffit", SL, CO, 0.7, [1.5, 0.05, 1], [0.75, -0.075, 0]), "slab-soffit")
s.th(box("th-slab-edge", SL, CO, 0.7, [0.05, 0.1, 1], [1.475, 0, 0]), "slab-edge")
for i, r_ in (("post", "railing-post"), ("handrail", "railing-handrail"), ("mid-rail", "railing-mid-rail")):
    s.thof(f"th-railing-{i}", r_, "railing-steel")
s.th(box("th-break-core", SL, CO, 0.7, [0.18, 0.2, 1], [-0.21, 0, 0]), "thermal-break", "break-concrete")
s.thof("th-break-module", "break-module", "thermal-break", "break-body")
s.thof("th-wall-concrete", "wall-core", "wall-face", "wall-concrete")
s.thof("th-wall-wool", "wall-insulation-render", "wall-face", "wall-wool")
s.st(box("slab-member", SL, CO, 0.7, [1.5, 0.2, 1], [0.75, 0, 0]), "slab", "member")
rebars(s, 100, s.feature)
s.stof("railing-post-member", "railing-post", "railing", "member")
s.stof("railing-handrail-member", "railing-handrail", "railing", "member")
s.st(box("joint-load-carrier", "load-carrier", ST, 0.3, [0.05, 0.05, 0.05], [0, 0.075, 0]), "load-carrier", "joint")
s.st(box("support-wall-fix-block", SL, ST, 0.15, [0.045, 0.2, 0.9], [0, 0, 0]), "slab", "support")
s.dec("Body: wall core and the concrete behind the break are one 1300 mm piece, wool and render are one 120 mm slab (the drawing keeps them as one), the drip groove, base plates and bolts are gone (below the 15 mm feature), the hollow posts are solid 50 mm sections; the mid-rail (40 mm) and handrail stay.",
      "Körper: Wandkern und der Beton hinter dem Dämmelement sind ein Stück von 1300 mm, Wolle und Putz eine Platte von 120 mm (die Zeichnung behält sie als eine), Tropfnase, Fußplatten und Schrauben entfallen (unter der Einzelheit 15 mm), die Hohlpfosten sind volle Querschnitte von 50 mm; Mittelholm (40 mm) und Handlauf bleiben.")
s.dec("Thermal: skins as before but the edge skin is cut back to the 100 mm between top and soffit skins (no overlap); wall layers wool and render merged. Structure: reinforcement as three 15 mm bars per mat (the 15 mm feature), hooks merged into the wall bars.",
      "Thermisch: Häute wie zuvor, aber die Stirnhaut ist auf die 100 mm zwischen Ober- und Unterhaut zurückgeschnitten (keine Überlappung); Wandschichten Wolle und Putz zusammengefasst. Tragwerk: Bewehrung als drei Stäbe von 15 mm je Lage (Einzelheit 15 mm), die Haken in den Wandstäben aufgegangen.")
sc.append(s)

# ---------------- 1:200
s = Scale(200)
wall_prof = [[-0.3, -0.1], [-0.12, -0.1], [-0.12, 0.1], [0, 0.1], [0, 1.2], [-0.3, 1.2]]
s.b(ext("wall", SL, CO, 0.7, "xy", wall_prof, -0.5, 0.5))
s.b(box("break-module", SL, BM, 0.8, [0.12, 0.2, 1], [-0.06, 0, 0]))
s.b(box("slab", SL, CO, 0.7, [1.5, 0.2, 1], [0.75, 0, 0]))
s.b(box("railing-post", RL, ST, 0.3, [1.04, 0.05, 0.05], [0.52, 0, -0.44], repeat={"count": 2, "translation": [0, 0, 0.88]}))
s.b(cyl("railing-handrail", RL, ST, 0.3, "z", [1.07, 0, 0], 1.0, 0.03))
s.th(box("th-slab-top", SL, CO, 0.7, [1.5, 0.06, 1], [0.75, 0.07, 0]), "slab-top")
s.th(box("th-slab-soffit", SL, CO, 0.7, [1.5, 0.06, 1], [0.75, -0.07, 0]), "slab-soffit")
s.th(box("th-slab-edge", SL, CO, 0.7, [0.06, 0.08, 1], [1.47, 0, 0]), "slab-edge")
s.thof("th-railing-post", "railing-post", "railing-steel")
s.thof("th-railing-handrail", "railing-handrail", "railing-steel")
s.thof("th-break-module", "break-module", "thermal-break", "break-body")
s.thof("th-wall-mass", "wall", "wall-face", "wall-concrete")
s.st(box("slab-member", SL, CO, 0.7, [1.5, 0.2, 1], [0.75, 0, 0]), "slab", "member")
s.stof("railing-post-member", "railing-post", "railing", "member")
s.stof("railing-handrail-member", "railing-handrail", "railing", "member")
s.st(box("support-wall-fix-block", SL, ST, 0.15, [0.06, 0.2, 0.9], [0, 0, 0]), "slab", "support")
s.dec("Body: the wall is one extruded L-shaped mass (concrete core, wool and render are indistinguishable at 30 mm), the break module and the slab stay; the railing is two 50 mm posts and the handrail (mid-rail merged into the posts).",
      "Körper: die Wand ist eine extrudierte L-förmige Masse (Betonkern, Wolle und Putz sind bei 30 mm nicht unterscheidbar), Dämmelement und Platte bleiben; das Geländer sind zwei Pfosten von 50 mm und der Handlauf (Mittelholm in die Pfosten aufgenommen).")
s.dec("Thermal: 60 mm skins; the wall region references the merged wall mass. Structure: slab member, railing members, no joints and no separate reinforcement (merged into the member); the fixed block widens to 60 mm.",
      "Thermisch: Häute von 60 mm; die Wandregion verweist auf die zusammengefasste Wandmasse. Tragwerk: Plattenstab, Geländerstäbe, keine Gelenke und keine eigene Bewehrung (im Stab aufgegangen); der feste Block wächst auf 60 mm.")
sc.append(s)

# ---------------- 1:500
s = Scale(500)
bal_prof = [[-0.3, -0.1], [1.5, -0.1], [1.5, 0.1], [0, 0.1], [0, 1.2], [-0.3, 1.2]]
s.b(ext("balcony-solid", SL, CO, 0.7, "xy", bal_prof, -0.5, 0.5))
s.b(box("railing-post", RL, ST, 0.3, [1.1, 0.075, 0.075], [0.55, 0, -0.4625], repeat={"count": 2, "translation": [0, 0, 0.925]}))
s.b(cyl("railing-handrail", RL, ST, 0.3, "z", [1.0625, 0, 0], 1.0, 0.0375))
s.th(box("th-balcony-top", SL, CO, 0.7, [1.5, 0.075, 1], [0.75, 0.0625, 0]), "slab-top")
s.th(box("th-balcony-soffit", SL, CO, 0.7, [1.5, 0.075, 1], [0.75, -0.0625, 0]), "slab-soffit")
s.th(box("th-balcony-edge", SL, CO, 0.7, [0.075, 0.075, 1], [1.4625, 0, 0]), "slab-edge")
s.th(box("th-balcony-break", SL, BM, 0.8, [0.12, 0.2, 1], [-0.06, 0, 0]), "thermal-break")
s.th(box("th-balcony-wall", SL, CO, 0.7, [0.3, 1.1, 1], [-0.15, 0.65, 0]), "wall-face")
s.thof("th-railing-post", "railing-post", "railing-steel")
s.thof("th-railing-handrail", "railing-handrail", "railing-steel")
s.st(box("slab-member", SL, CO, 0.7, [1.5, 0.2, 1], [0.75, 0, 0]), "slab", "member")
s.stof("railing-post-member", "railing-post", "railing", "member")
s.st(box("support-wall-fix-block", SL, ST, 0.15, [0.075, 0.2, 0.9], [0, 0, 0]), "slab", "support")
s.dec("Body: wall and slab are one L-shaped extrusion as the 1:500 drawing; the railing is two 75 mm posts and a 75 mm handrail (the drawing draws it as one line, the load-bearing posts cannot be thinner than the feature).",
      "Körper: Wand und Platte sind eine L-förmige Extrusion wie in der Zeichnung 1:500; das Geländer sind zwei Pfosten von 75 mm und ein Handlauf von 75 mm (die Zeichnung zeichnet es als eine Linie, die tragenden Pfosten können nicht dünner als die Einzelheit sein).")
s.dec("Thermal: every region is its own 75 mm piece inside the mass (top, soffit and edge skins, break module, wall), the railing is referenced. Structure: slab member, the railing posts as the one railing member (the handrail rides on them), the support block at 75 mm; reinforcement is merged into the member.",
      "Thermisch: jede Region ist ein eigenes Stück von 75 mm in der Masse (Ober-, Unterhaut und Stirnhaut, Dämmelement, Wand), das Geländer wird referenziert. Tragwerk: Plattenstab, die Geländerpfosten als der eine Geländerstab (der Handlauf liegt auf ihnen), der Auflagerblock mit 75 mm; die Bewehrung ist im Stab aufgegangen.")
sc.append(s)
save("components", "cantilever-balcony", sc)
