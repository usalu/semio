import os
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())
B = "beam"
G, ST = "glulam", "steel"
TD = 0.036


def pad_pin_roller(sc, scale):
    d = {50: 0.016, 100: 0.02, 200: 0.03, 500: 0.075}[scale]
    ph = {50: 0.008, 100: 0.01, 200: 0.015, 500: 0.0375}[scale]
    kn = {50: 0.012, 100: 0.015, 200: 0.0225, 500: 0.0375}[scale]
    g = -0.32
    x = 0.06
    sc.st(ext("support-bearing-left-pin", B, ST, 0.4, "xy", [[x - 0.06, g], [x + 0.06, g], [x + 0.06, g + ph], [x + 0.03, g + ph], [x, g + ph + kn], [x - 0.03, g + ph], [x - 0.06, g + ph]], -0.08, 0.08), "beam", "support")
    x = 7.94
    sc.st(cyl("support-bearing-right-roller", B, ST, 0.4, "z", [x, g + d / 2, 0], 0.16, d / 2), "beam", "support")


def member(sc):
    sc.st(box("beam-member", B, G, 0.6, [8, 0.6, 0.16], [4, 0, 0], r=0.006 if sc.den == 50 else None), "beam", "member")


def skins(sc, t):
    c = 0.08 - t / 2
    for sd, z, inner in (("l", -c, "+w"), ("r", c, "-w")):
        sc.th(box(f"th-side-{sd}", B, G, 0.6, [8, 0.6, t], [4, 0, z]), "beam-sides")
        sc.thermal[-1]["sectionFaces"] = [inner, "-v", "+v"]
    sc.th(box("th-top", B, G, 0.6, [8, t, 0.16], [4, 0.3 - t / 2, 0]), "beam-top")
    sc.thermal[-1]["sectionFaces"] = ["-v", "-w", "+w"]
    sc.th(box("th-soffit", B, G, 0.6, [8, t, 0.16], [4, -0.3 + t / 2, 0]), "beam-soffit")
    sc.thermal[-1]["sectionFaces"] = ["+v", "-w", "+w"]


scales = []
s = Scale(50)
for k in range(15):
    y0, y1 = -0.3 + 0.04 * k, -0.26 + 0.04 * k
    c = 0.006
    if k == 0:
        prof = [[y0, -0.08 + c], [y0 + c, -0.08], [y1, -0.08], [y1, 0.08], [y0 + c, 0.08], [y0, 0.08 - c]]
    elif k == 14:
        prof = [[y0, -0.08], [y1 - c, -0.08], [y1, -0.08 + c], [y1, 0.08 - c], [y1 - c, 0.08], [y0, 0.08]]
    else:
        prof = [[y0, -0.08], [y1, -0.08], [y1, 0.08], [y0, 0.08]]
    s.b(ext(f"lamella-{k:02d}", B, G, 0.56 if k % 2 == 0 else 0.62, "yz", prof, 0, 8))
for sd, x in (("left", 0.06), ("right", 7.94)):
    s.b(box(f"bearing-plate-{sd}", B, ST, 0.3, [0.12, 0.02, 0.16], [x, -0.31, 0]))
skins(s, TD)
for sd in ("left", "right"):
    s.thof(f"th-bearing-plate-{sd}", f"bearing-plate-{sd}", "bearing-steel")
member(s)
pad_pin_roller(s, 50)
s.st(box("joint-load-carrier", "load-carrier", ST, 0.3, [0.05, 0.05, 0.05], [0, 0.275, 0]), "load-carrier", "joint")
s.dec("Body: the beam as its fifteen 40 mm lamellae (alternating tone so the glue lines read), the outermost two with the 6 mm arris chamfer of the drawing, plus the two 120 x 20 x 160 mm steel bearing plates. Finger joints and glue lines are texture in the drawing and stay off the solid.",
      "Körper: der Balken als seine fünfzehn Lamellen zu 40 mm (abwechselnder Ton, damit die Leimfugen lesbar sind), die äußeren beiden mit der 6-mm-Kantenfase der Zeichnung, dazu die zwei Stahllagerplatten 120 x 20 x 160 mm. Keilzinkenstöße und Leimfugen sind in der Zeichnung Textur und bleiben ohne Volumen.")
s.dec("Thermal: faces are skins of the effective storage depth 36 mm of the passive timber (heat capacity 24 kJ/m2K = 420 kg/m3 x 1600 J/kgK x 0.036 m): two side skins, top and soffit; the bearing plates are referenced from the body.",
      "Thermisch: Flächen sind Häute der wirksamen Speichertiefe 36 mm des passiven Holzes (Wärmekapazität 24 kJ/m2K = 420 kg/m3 x 1600 J/kgK x 0,036 m): zwei Seitenhäute, Oberseite und Unterseite; die Lagerplatten werden aus dem Körper referenziert.")
s.dec("Structure: one member solid of the real 160 x 600 mm section (A 0.096 m2, I 0.00288 m4), a slider joint at the prismatic load carrier, and the supports by type: pin = steel pad with a fixed wedge knuckle, roller = steel pad with a free roller across the beam width.",
      "Tragwerk: ein Stabkörper mit dem echten Querschnitt 160 x 600 mm (A 0,096 m2, I 0,00288 m4), ein Gleitgelenk am verschieblichen Lastträger und die Auflager nach Typ: Festlager = Stahlplatte mit festem Keilgelenk, Rollenlager = Stahlplatte mit freier Rolle über die Balkenbreite.")
scales.append(s)
s = Scale(100)
s.b(box("beam", B, G, 0.6, [8, 0.6, 0.16], [4, 0, 0]))
for sd, x in (("left", 0.06), ("right", 7.94)):
    s.b(box(f"bearing-plate-{sd}", B, ST, 0.3, [0.12, 0.025, 0.16], [x, -0.3075, 0]))
skins(s, 0.04)
for sd in ("left", "right"):
    s.thof(f"th-bearing-plate-{sd}", f"bearing-plate-{sd}", "bearing-steel")
member(s)
pad_pin_roller(s, 100)
s.st(box("joint-load-carrier", "load-carrier", ST, 0.3, [0.05, 0.05, 0.05], [0, 0.275, 0]), "load-carrier", "joint")
s.dec("Body: lamellae, chamfers and finger joints are merged into one 160 x 600 x 8000 mm beam (the drawing shows grain hatch only); the bearing plates keep their 25 mm drawn thickness.",
      "Körper: Lamellen, Fasen und Keilzinkenstöße sind zu einem Balken 160 x 600 x 8000 mm zusammengefasst (die Zeichnung zeigt nur Faserschraffur); die Lagerplatten behalten ihre gezeichnete Dicke 25 mm.")
s.dec("Thermal: the same four faces as skins, now one solid per face and the 36 mm storage depth rounded to 40 mm (the nearest 10 mm); plates referenced from the body. Structure: member as before, pad and knuckle of each support are 15 mm parts, the load-carrier joint stays a 50 mm slider.",
      "Thermisch: dieselben vier Flächen als Häute, jetzt ein Körper je Fläche, die Speichertiefe 36 mm auf 40 mm gerundet (nächste 10 mm); Platten aus dem Körper referenziert. Tragwerk: Stab wie zuvor, Platte und Gelenk jedes Auflagers sind Teile von 15 mm, das Lastträgergelenk bleibt ein Gleiter von 50 mm.")
s.dec("Minimal: the thermal list is one solid per timber face region and one per bearing plate (six); a coarser list would lose a region, so its count equals the 1:50 list while body (17 to 3 solids) and structure triangles drop.",
      "Minimal: die Thermalliste hat einen Körper je Holzflächenregion und einen je Lagerplatte (sechs); eine gröbere Liste verlöre eine Region, ihre Anzahl gleicht daher der 1:50-Liste, während Körper (17 auf 3 Körper) und Tragwerksdreiecke sinken.")
scales.append(s)
s = Scale(200)
s.b(box("beam", B, G, 0.6, [8, 0.6, 0.16], [4, 0, 0]))
for sd, x in (("left", 0.06), ("right", 7.94)):
    s.b(box(f"bearing-plate-{sd}", B, ST, 0.3, [0.12, 0.03, 0.16], [x, -0.305, 0]))
skins(s, 0.045)
for sd in ("left", "right"):
    s.thof(f"th-bearing-plate-{sd}", f"bearing-plate-{sd}", "bearing-steel")
member(s)
pad_pin_roller(s, 200)
s.dec("Body: beam and the two bearing plates, the plates thickened from 20 to 30 mm (the smallest feature at 1:200; the drawing shows them as lines at the soffit); same proportions and bounding box.",
      "Körper: Balken und die zwei Lagerplatten, die Platten von 20 auf 30 mm verstärkt (kleinste Einzelheit bei 1:200; die Zeichnung zeigt sie als Linien an der Unterseite); gleiche Proportionen und Hüllbox.")
s.dec("Thermal: faces as skins of 45 mm (1.5 features; the 36 mm storage depth would sit just above the 30 mm feature and make this list a copy of the 1:50 one). Structure: the joint of the load carrier is merged into the member; each support is a single 30 mm part (wedge for the pin, roller for the roller bearing).",
      "Thermisch: Flächen als Häute von 45 mm (1,5 Einzelheiten; die Speichertiefe 36 mm läge knapp über der Einzelheit 30 mm und machte diese Liste zur Kopie der 1:50-Liste). Tragwerk: das Lastträgergelenk ist in den Stab aufgenommen; jedes Auflager ist ein einzelnes Teil von 30 mm (Keil für das Festlager, Rolle für das Rollenlager).")
s.dec("Minimal: beam and two plates are the least that carries bearings; thermal skins and structure are as at 1:100.", "Minimal: Balken und zwei Platten sind das Wenigste, das Auflager trägt; Thermalhäute und Tragwerk wie bei 1:100.")
scales.append(s)
s = Scale(500)
s.b(box("beam", B, G, 0.6, [8, 0.62, 0.16], [4, -0.01, 0]))
skins(s, 0.075)
for sd, x in (("left", 0.06), ("right", 7.94)):
    s.th(box(f"th-bearing-plate-{sd}", B, ST, 0.3, [0.12, 0.075, 0.16], [x, -0.3225, 0]), "bearing-steel")
member(s)
pad_pin_roller(s, 500)
s.dec("Body: one 8000 x 620 x 160 mm block from soffit of the plates to the top of the beam, as the 1:500 drawing (a single rectangle); lamellae, chamfers and plates are merged into it.",
      "Körper: ein Block 8000 x 620 x 160 mm von der Plattenunterseite bis zur Balkenoberkante, wie die Zeichnung 1:500 (ein einziges Rechteck); Lamellen, Fasen und Platten sind darin aufgegangen.")
s.dec("Thermal and structure are authored on their own: the bearing steel is a 75 mm block inside the block, the skins are 75 mm (the smallest feature), the member is the true section, supports are 75 mm wedge and roller blocks under the member ends; no joint (merged).",
      "Thermik und Tragwerk sind eigenständig: der Lagerstahl ist ein Block von 75 mm im Block, die Häute sind 75 mm (kleinste Einzelheit), der Stab hat den echten Querschnitt, die Auflager sind Keil- und Rollenblöcke von 75 mm unter den Stabenden; kein Gelenk (zusammengefasst).")
s.dec("Minimal: one block, six thermal pieces, three structure solids.", "Minimal: ein Block, sechs Thermalstücke, drei Tragwerkskörper.")
scales.append(s)
EN = " The four skins overlap in the arris prisms: the top and soffit skins span the full width, the side skins the full height, and the faces that lie against the core or against the neighbouring skin (inner faces, arris strips) are section faces, so every skin exposes its own outer face once."
DE = " Die vier Häute überlappen in den Kantenprismen: Ober- und Unterhaut laufen über die ganze Breite, die Seitenhäute über die ganze Höhe, und die Flächen am Kern oder an der Nachbarhaut (Innenflächen, Kantenstreifen) sind Schnittflächen, damit jede Haut ihre Außenfläche genau einmal zeigt."
for sc_ in scales:
    for d in sc_.decisions:
        if d["en"].startswith("Thermal"):
            d["en"] += EN
            d["de"] += DE
            break
save("components", "glulam-beam", scales)
