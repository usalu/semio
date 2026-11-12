import sys, os, math
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
S, R = "slab", "railing"
St = "railing-steel"
s = [
 L.box("wall-core", S, "concrete", "body", 0.6, [0.18, 1.1, 1], [-0.21, 0.65, 0], "wall-face"),
 L.box("break-core", S, "concrete", "body", 0.6, [0.18, 0.2, 1], [-0.21, 0, 0], "thermal-break"),
 L.box("wall-insulation", S, "mineral-wool", "body", 0.9, [0.1, 1.1, 1], [-0.07, 0.65, 0], "wall-face"),
 L.box("wall-render", S, "render", "body", 0.55, [0.02, 1.1, 1], [-0.01, 0.65, 0], "wall-face"),
 L.box("thermal-break-block", S, "break-module", "body", 0.3, [0.12, 0.2, 1], [-0.06, 0, 0], "thermal-break"),
 L.box("slab", S, "concrete", "body", 0.72, [1.5, 0.2, 1], [0.75, 0, 0], regions=["slab-top", "slab-soffit", "slab-edge"]),
 L.box("base-plate", R, "steel", "body", 0.3, [0.012, 0.1, 0.1], [0.006, 0, 0.44], St, repeat={"count": 2, "translation": [0, 0, -0.88]}),
 L.box("post", R, "steel", "body", 0.45, [1.028, 0.05, 0.05], [0.526, 0, 0.44], St, repeat={"count": 2, "translation": [0, 0, -0.88]}),
 L.cap("handrail", R, "steel", "body", 0.35, [1.07, 0, -0.47], [1.07, 0, 0.47], 0.03, region=St),
 L.cap("mid-rail", R, "steel", "body", 0.35, [0.55, 0, -0.42], [0.55, 0, 0.42], 0.02, region=St),
]
hexa = lambda cv, cw: [(cv + 0.0115 * math.cos(math.radians(60 * k)), cw + 0.0115 * math.sin(math.radians(60 * k))) for k in range(6)]
for sv, nm in ((0.035, "a"), (-0.035, "b")):
    s.append(L.ext("bolt-%s-l" % nm, R, "steel", "detail", 0.1, hexa(sv, 0.405), "yz", 0.012, 0.024, St, maxD=50, repeat={"count": 2, "translation": [0, 0, 0.07]}))
    s.append(L.ext("bolt-%s-r" % nm, R, "steel", "detail", 0.1, hexa(sv, -0.475), "yz", 0.012, 0.024, St, maxD=50, repeat={"count": 2, "translation": [0, 0, 0.07]}))
L.write("cantilever-balcony", s, [
 ("The wall strip next to the slab is split in its thermal layers: concrete core 180 mm, mineral wool 100 mm and render 20 mm above the slab; at slab level the core is the break concrete and the 120 mm break module replaces wool and render, so each layer is its own solid with the region the thermal view colours.", "Der Wandstreifen neben der Platte ist in seine thermischen Schichten geteilt: Betonkern 180 mm, Mineralwolle 100 mm und Putz 20 mm über der Platte; auf Plattenhöhe ist der Kern der Trennbeton und das 120 mm Dämmmodul ersetzt Wolle und Putz, sodass jede Schicht ein eigener Körper mit der Region der thermischen Ansicht ist."),
 ("The slab is one 1500 x 200 mm box carrying the three face regions top, soffit and edge; the railing (bone rotated 90 degrees, so u is up and v points back) has two 50 mm square posts, a 60 mm handrail tube and a 40 mm mid rail tube (capsules whose round ends end at the slab edge lines) on 100 x 100 x 12 mm base plates with four hexagon bolt heads each.", "Die Platte ist ein Quader 1500 x 200 mm mit den drei Flächenregionen Oberseite, Unterseite und Stirn; das Geländer (Knochen um 90 Grad gedreht, also u nach oben und v nach hinten) hat zwei Pfosten 50 mm quadratisch, ein Handlaufrohr 60 mm und ein Zwischenrohr 40 mm (Kapseln, deren runde Enden an den Plattenkanten enden) auf Fussplatten 100 x 100 x 12 mm mit je vier Sechskantschrauben."),
 ("Reinforcement bars, the drip groove, the slab arris line and wall rebar markers are hidden or subtractive 2D details and are not modelled; the load carrier bone is a load point without volume.", "Bewehrungsstäbe, Tropfnase, Kantenlinie und Wandbewehrungsmarken sind verdeckte oder abtragende 2D-Details und nicht modelliert; der Lastträgerknochen ist ein Lastpunkt ohne Volumen."),
])
