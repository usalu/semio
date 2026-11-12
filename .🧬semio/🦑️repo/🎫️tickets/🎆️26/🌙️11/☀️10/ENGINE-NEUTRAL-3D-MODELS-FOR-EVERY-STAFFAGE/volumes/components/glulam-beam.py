import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
B = "beam"
s = [
 L.box("beam", B, "glulam", "body", 0.6, [8, 0.6, 0.16], [4, 0, 0], regions=["beam-sides", "beam-top", "beam-soffit"], corner=0.006),
 L.box("bearing-plate-left", B, "steel", "body", 0.3, [0.12, 0.02, 0.16], [0.06, -0.31, 0], "bearing-steel"),
 L.box("bearing-plate-right", B, "steel", "body", 0.3, [0.12, 0.02, 0.16], [7.94, -0.31, 0], "bearing-steel"),
]
L.write("glulam-beam", s, [
 ("The 8 m beam is one box 600 x 160 mm with a 6 mm corner radius on every edge (the 2D outline is chamfered at all corners), tagged with the three face regions beam-sides, beam-top and beam-soffit so the thermal view can colour its skin.", "Der 8 m lange Träger ist ein einziger Quader 600 x 160 mm mit 6 mm Kantenradius an allen Kanten (der 2D-Umriss ist an allen Ecken gefast) und mit den drei Flächenregionen Seiten, Oberseite und Unterseite versehen, damit die thermische Ansicht seine Haut einfärben kann."),
 ("Glue lines (15 lamellas) and finger joints are 2D texture and joint marks and are not modelled; two 120 x 20 x 160 mm steel bearing plates sit under the soffit at both ends; the load carrier bone is a load point without volume.", "Klebefugen (15 Lamellen) und Keilzinken sind 2D-Textur und Stossmarkierungen und nicht modelliert; zwei Stahl-Auflagerplatten 120 x 20 x 160 mm liegen an beiden Enden unter der Unterseite; der Lastträgerknochen ist ein Lastpunkt ohne Volumen."),
])
