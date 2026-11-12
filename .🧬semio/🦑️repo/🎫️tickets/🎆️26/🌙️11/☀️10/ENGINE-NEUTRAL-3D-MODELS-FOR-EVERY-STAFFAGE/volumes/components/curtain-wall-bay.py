import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
F, G, S = "frame", "igu", "spandrel"
AL = "aluminium"
zs = [("cap", -0.075, -0.073, 0.08), ("outer", -0.073, -0.04, 0.4), ("break", -0.04, -0.01, 0.85), ("inner", -0.01, 0.075, 0.4)]
s = []
for nm, a, b, tone in zs:
    s.append(L.box("mullion-%s" % nm, F, AL, "body", tone, [0.05, 3.3, b - a], [0.025, 1.65, (a + b) / 2], "mullion", repeat={"count": 2, "translation": [1.35, 0, 0]}))
for nm, a, b, tone in zs:
    s.append(L.box("transom-%s" % nm, F, AL, "body", tone, [1.3, 0.05, b - a], [0.7, 0.025, (a + b) / 2], "transom", repeat={"count": 2, "translation": [0, 0.85, 0]}))
for nm, a, b, tone in zs:
    s.append(L.box("transom-head-%s" % nm, F, AL, "body", tone, [1.3, 0.05, b - a], [0.7, 3.275, (a + b) / 2], "transom"))
ring = [(0, 0), (1.3, 0), (1.3, 2.35), (0, 2.35)]
hole = [(0.065, 0.065), (0.065, 2.285), (1.235, 2.285), (1.235, 0.065)]
for nm, tone, w0, w1 in (("pane-out", 0.15, -0.036, -0.030), ("cavity", 0.08, -0.030, -0.014), ("pane-in", 0.15, -0.014, -0.008)):
    s.append(L.ext("%s-edge" % nm, G, "glass", "glazing", 0.4 if nm == "cavity" else tone, ring, "xy", w0, w1, "glass-edge", holes=[hole]))
    s.append(L.box("%s-centre" % nm, G, "glass", "glazing", tone, [1.17, 2.22, w1 - w0], [0.65, 1.175, (w0 + w1) / 2], "glass-centre"))
for nm, mat, tone, a, b in (("enamel", "spandrel-glass", 0.65, -0.070, -0.064), ("air", "wool", 0.5, -0.064, -0.044), ("wool", "wool", 0.9, -0.044, 0.056), ("pan", "steel", 0.3, 0.056, 0.0575)):
    s.append(L.box("spandrel-" + nm, S, mat, "body", tone, [1.3, 0.8, b - a], [0.65, 0.4, (a + b) / 2], "spandrel"))
L.write("curtain-wall-bay", s, [
 ("Mullions and transoms are split through their 150 mm depth into the layers the thermal view needs: a 2 mm outer cap, the 33 mm outer part, the 30 mm thermal break and the 85 mm inner part (exterior is -z as in the front view); each is one solid per member kind with a repeat for the two mullions and for the base and mid transoms.", "Pfosten und Riegel sind über ihre 150 mm Tiefe in die Schichten geteilt, die die thermische Ansicht braucht: eine 2 mm Aussenkappe, der 33 mm Aussenteil, die 30 mm thermische Trennung und der 85 mm Innenteil (aussen ist -z wie in der Vorderansicht); je Profilart ein Körper mit Wiederholung für die zwei Pfosten und für den unteren und mittleren Riegel."),
 ("The insulated glass unit is built layer by layer (outer pane 6, cavity 16, inner pane 6 mm; the catalog gives the cavity the glass material), each as a centre box (region glass-centre) and a 65 mm edge-zone frame (extrude with hole, region glass-edge); the spandrel panel stacks enamel 6, an air layer 20 mm (material wool as the catalog layers say), wool 100 and the steel pan 1.5 mm.", "Die Isolierglaseinheit ist Schicht für Schicht aufgebaut (Aussenscheibe 6, Zwischenraum 16, Innenscheibe 6 mm; der Katalog gibt dem Zwischenraum das Glasmaterial), jede als Mittelquader (Region glass-centre) und als 65 mm Randzonenrahmen (Extrusion mit Loch, Region glass-edge); das Brüstungspaneel stapelt Emaille 6, eine 20 mm Luftschicht (Material Wolle, wie die Katalogschichten sagen), Wolle 100 und die Stahlwanne 1,5 mm."),
 ("The cradle bone is a load point without volume; reflexes and the spandrel box outline are 2D texture and not modelled.", "Der Knochen des Gondelwagens ist ein Lastpunkt ohne Volumen; Reflexe und die Kastenkontur der Brüstung sind 2D-Textur und nicht modelliert."),
])
