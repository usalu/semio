import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
SL, SA = "sill", "sash"
FT, FS, PG, PE = "frame-timber", "frame-sill", "pane-glass", "pane-edge"
s = [
 L.box("sill", SL, "timber", "body", 0.6, [0.3, 0.05, 0.9], [0.15, -0.025, 0], FS),
 L.box("jamb", SL, "timber", "body", 0.6, [0.3, 1.2, 0.05], [0.15, 0.6, 0.425], FT, repeat={"count": 2, "translation": [0, 0, -0.85]}),
 L.box("stile", SA, "timber", "body", 0.6, [1.2, 0.09, 0.045], [0.6, 0, 0.3775], FT, repeat={"count": 2, "translation": [0, 0, -0.755]}),
 L.ext("rail-bottom", SA, "timber", "body", 0.6, [(0, 0.045), (0, -0.045), (0.06, -0.045), (0.06, -0.013), (0.045, -0.013), (0.045, 0.013), (0.06, 0.013), (0.06, 0.045)], "xy", -0.355, 0.355, FT),
 L.ext("rail-top", SA, "timber", "body", 0.6, [(1.2, 0.045), (1.2, -0.045), (1.14, -0.045), (1.14, -0.013), (1.155, -0.013), (1.155, 0.013), (1.14, 0.013), (1.14, 0.045)], "xy", -0.355, 0.355, FT),
]
ring = [(-0.355, 0.045), (-0.355, 1.155), (0.355, 1.155), (0.355, 0.045)]
hole = [(-0.275, 0.125), (-0.275, 1.075), (0.275, 1.075), (0.275, 0.125)]
for nm, mat, tone, v0, v1, edge_mat in (("inner", "glass", 0.9, 0.008, 0.012, "glass"), ("cavity", "glass-unit-cavity", 0.9, -0.008, 0.008, "edge-zone"), ("outer", "glass", 0.9, -0.012, -0.008, "glass")):
    s.append(L.ext("ig-%s-edge" % nm, SA, edge_mat, "glazing", 0.5 if nm == "cavity" else tone, ring, "zx", v0, v1, PE, holes=[hole]))
    s.append(L.box("ig-%s-centre" % nm, SA, mat, "glazing", tone, [0.95, v1 - v0, 0.55], [0.6, (v0 + v1) / 2, 0], PG))
s.append(L.cyl("hinge-pin", SA, "timber", "detail", 0.35, [0, 0, 0.27], [0, 0, 1], 0.06, 0.02, FT, maxD=50, repeat={"count": 2, "translation": [0, 0, -0.6]}))
L.write("tilt-window", s, [
 ("The sash is bound to the tilting bone: two 45 x 90 mm stiles, a bottom and a top rail with the glass rebate (extrudes of the elevation profiles) and the insulated glass unit of 24 mm built layer by layer as the thermal view needs: inner pane 4, cavity 16, outer pane 4 mm, each as a centre box (region pane-glass) and an 80 mm edge-zone frame (extrude with a rectangular hole, region pane-edge) whose cavity layer is the edge-zone material.", "Der Flügel ist an den Kippknochen gebunden: zwei Holme 45 x 90 mm, ein unteres und ein oberes Querholz mit Glasfalz (Extrusionen der Ansichtsprofile) und die Isolierglaseinheit von 24 mm, Schicht für Schicht wie die thermische Ansicht es braucht: Innenscheibe 4, Zwischenraum 16, Aussenscheibe 4 mm, jede als Mittelquader (Region pane-glass) und als 80 mm Randzonenrahmen (Extrusion mit rechteckigem Loch, Region pane-edge), dessen Zwischenraumschicht das Randzonenmaterial ist."),
 ("The frame is drawn without a head rail in all three views, so none is modelled; the sill board and the two 50 x 1200 mm jambs sit on the sill bone, the sash rests on the sill at its hinge line and tilts about it up to 30 degrees. The 2D hinge circles disagree on the pin axis (circles in elevation, front and plan); the pins are modelled along the tilt axis (w), two of 60 mm length at z = +-0.3 (only at 1:50).", "Der Rahmen ist in allen drei Ansichten ohne Sturzriegel gezeichnet, daher ist keiner modelliert; Sohlbank und die zwei Pfosten 50 x 1200 mm sitzen am Sohlbank-Knochen, der Flügel liegt an seiner Scharnierlinie auf der Sohlbank und kippt darum bis 30 Grad. Die 2D-Scharnierkreise widersprechen sich in der Stiftachse (Kreise in Ansicht, Vorder- und Draufsicht); die Stifte sind längs der Kippachse (w) modelliert, zwei zu 60 mm Länge bei z = +-0,3 (nur bei 1:50)."),
])
