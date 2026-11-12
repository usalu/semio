import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
C = "canopy"
rect = lambda v0, v1, w: [(v0, -w), (v0, w), (v1, w), (v1, -w)]
holes = [rect(-0.12, 0.12, 0.1), rect(2.76, 3.0, 0.1)]
outer = rect(-2.33, 3.02, 0.5)
s = [
 L.box("wall-core", C, "concrete", "body", 0.6, [0.3, 5.35, 1], [-0.35, 0.345, 0], "wall-face"),
 L.ext("wall-insulation", C, "mineral-wool", "body", 0.9, outer, "yz", -0.2, -0.02, "wall-face", holes=holes),
 L.ext("wall-render", C, "render", "body", 0.55, outer, "yz", -0.02, 0, "wall-face", holes=holes),
 L.box("pad-beam", C, "anchor-pad", "body", 0.7, [0.18, 0.24, 0.2], [-0.11, 0, 0], "wall-plate"),
 L.box("pad-strut", C, "anchor-pad", "body", 0.7, [0.18, 0.24, 0.2], [-0.11, 2.88, 0], "wall-plate"),
 L.box("plate-beam", C, "steel", "body", 0.3, [0.02, 0.2, 0.16], [-0.01, 0, 0], "wall-plate"),
 L.box("plate-strut", C, "steel", "body", 0.3, [0.02, 0.2, 0.16], [-0.01, 2.88, 0], "wall-plate"),
 L.box("beam-flange-top", C, "steel", "body", 0.4, [5.5, 0.0074, 0.082], [2.75, 0.0763, 0], "canopy-steel"),
 L.box("beam-flange-bottom", C, "steel", "body", 0.4, [5.5, 0.0074, 0.082], [2.75, -0.0763, 0], "canopy-steel"),
 L.box("beam-web", C, "steel", "body", 0.4, [5.5, 0.1452, 0.005], [2.75, 0, 0], "canopy-steel"),
 L.box("glass", C, "glass", "glazing", 0.92, [1.28, 0.02, 1.0], [0.74, 0.09, 0], "canopy-glass", repeat={"count": 4, "translation": [1.34, 0, 0]}),
 L.cap("strut-tube", C, "steel", "body", 0.45, [4, 0, 0], [0, 2.88, 0], 0.03, region="rod-steel"),
 L.box("clevis-beam", C, "steel", "body", 0.3, [0.12, 0.24, 0.1], [4, 0, 0], "rod-steel", corner=0.01),
 L.box("clevis-wall", C, "steel", "body", 0.3, [0.08, 0.24, 0.08], [0, 2.88, 0], "rod-steel", corner=0.01),
 L.cyl("pin-beam", C, "steel", "detail", 0.1, [4, 0, -0.055], [0, 0, 1], 0.11, 0.012, "rod-steel", maxD=50),
 L.cyl("pin-wall", C, "steel", "detail", 0.1, [0, 2.88, -0.045], [0, 0, 1], 0.09, 0.012, "rod-steel", maxD=50),
]
L.write("tied-canopy", s, [
 ("The wall strip is built in its thermal layers: concrete core 300, mineral wool 180 and render 20 mm; the wool and render layers are extrudes with rectangular holes where the two anchor plates sit, and the 180 mm anchor pads (thermal break) fill the wool holes so each layer is a solid for the thermal view.", "Der Wandstreifen ist in seinen thermischen Schichten aufgebaut: Betonkern 300, Mineralwolle 180 und Putz 20 mm; Dämmung und Putz sind Extrusionen mit rechteckigen Löchern an den beiden Ankerplatten, und die 180 mm dicken Ankerpads (thermische Trennung) füllen die Dämmungslöcher, sodass jede Schicht ein Körper für die thermische Ansicht ist."),
 ("Anchor plates are 160 x 200 x 20 mm (front view; the elevation draws 240 mm height, 2D inconsistency) sitting on pads of 200 x 240 mm recessed 20 mm behind the render; the beam is an IPE160-like I-section (flanges 82 x 7.4, web 5 mm) with four 1280 mm glass panes 20 mm thick on its top flange and a 60 mm tie rod from the beam clevis to the wall clevis.", "Die Ankerplatten sind 160 x 200 x 20 mm (Vorderansicht; die Ansicht zeichnet 240 mm Höhe, 2D-Widerspruch) auf Pads von 200 x 240 mm, 20 mm hinter dem Putz zurückversetzt; der Träger ist ein IPE160-artiges I-Profil (Flansche 82 x 7,4, Steg 5 mm) mit vier 1280 mm langen, 20 mm dicken Glasscheiben auf dem Obergurt und einem 60 mm Zugstab vom Trägerbock zum Wandbock."),
 ("Pins are cylinders along w protruding 5 mm from the clevis blocks; rebar lines and the insulation zigzag are 2D texture and not modelled.", "Bolzen sind Zylinder längs w, die 5 mm aus den Böcken ragen; Bewehrungslinien und die Dämmungs-Zickzacklinie sind 2D-Textur und nicht modelliert."),
])
