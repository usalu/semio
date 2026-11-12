import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
B, H = "body", "handrail"
RG = ["ramp-surface", "ramp-side"]
two = {"count": 2, "translation": [0, 0, -1.3]}
s = []
parts = [("run-1", [(1.5, 0), (5.1, 0.3), (5.1, 0)]), ("landing-1", [(5.1, 0), (5.1, 0.3), (6.6, 0.3), (6.6, 0)]), ("run-2", [(6.6, 0), (6.6, 0.3), (10.2, 0.6), (10.2, 0)]), ("landing-2", [(10.2, 0), (10.2, 0.6), (11.7, 0.6), (11.7, 0)])]
for id, pts in parts:
    s.append(L.ext(id, B, "concrete", "body", 0.6, pts, "xy", -0.7, 0.7, regions=RG))
kerb = [(1.5, 0), (5.1, 0.3), (6.6, 0.3), (10.2, 0.6), (10.2, 0.7), (6.6, 0.4), (5.1, 0.4), (1.5, 0.1)]
s.append(L.ext("kerb", B, "concrete", "body", 0.45, kerb, "xy", 0.6, 0.7, "kerb", repeat={"count": 2, "translation": [0, 0, -1.3]}))
path = [(0, 0.85), (0.3, 0.85), (3.9, 1.15), (5.4, 1.15), (9, 1.45), (9.3, 1.45)]
for k in range(5):
    s.append(L.cap("rail-%d" % k, H, "steel", "detail", 0.35, [path[k][0], path[k][1], 0.65], [path[k + 1][0], path[k + 1][1], 0.65], 0.0212, region="handrail-steel", repeat=two))
s.append(L.cap("rail-return-start", H, "steel", "detail", 0.35, [0, 0.85, 0.65], [0, 0.75, 0.65], 0.0212, region="handrail-steel", repeat=two))
s.append(L.cap("rail-return-end", H, "steel", "detail", 0.35, [9.3, 1.45, 0.65], [9.3, 1.35, 0.65], 0.0212, region="handrail-steel", repeat=two))
for grp, (u0, v0) in (("a", (0.3, 0.1)), ("b", (5.4, 0.4))):
    rep = {"count": 3, "translation": [1.8, 0.15, 0]}
    s.append(L.box("plate-" + grp, H, "steel", "detail", 0.3, [0.12, 0.02, 0.1], [u0, v0 + 0.01, 0.65], "handrail-steel", repeat=rep))
    s.append(L.box("post-" + grp, H, "steel", "detail", 0.35, [0.05, 0.709, 0.05], [u0, v0 + 0.02 + 0.3545, 0.65], "handrail-steel", repeat=rep))
    s.append(L.box("plate-%s-r" % grp, H, "steel", "detail", 0.3, [0.12, 0.02, 0.1], [u0, v0 + 0.01, -0.65], "handrail-steel", repeat=rep))
    s.append(L.box("post-%s-r" % grp, H, "steel", "detail", 0.35, [0.05, 0.709, 0.05], [u0, v0 + 0.02 + 0.3545, -0.65], "handrail-steel", repeat=rep))
L.write("wheelchair-ramp", s, [
 ("The ramp body is four solid concrete extrudes of the elevation outline, split at the expansion joints into run 1, landing 1, run 2 and landing 2, all carrying the regions ramp-surface and ramp-side; the 2D draws it filled and its mass of 12.8 t matches a solid body.", "Der Rampenkörper besteht aus vier massiven Beton-Extrusionen des Ansichtsumrisses, an den Dehnfugen geteilt in Lauf 1, Podest 1, Lauf 2 und Podest 2, alle mit den Regionen Rampenoberfläche und Rampenseite; die 2D-Ansicht zeichnet ihn gefüllt und seine Masse von 12,8 t entspricht einem Vollkörper."),
 ("The two 100 mm kerbs follow the slope as one extrude each (one solid with repeat across the 1.3 m), the two handrails are chains of 42.4 mm tube capsules whose round joints form the elbows, with returns at both ends; posts and 120 x 20 x 100 mm base plates stand on the kerb in two regular groups of three per side (one solid each with a repeat that follows the slope).", "Die beiden 100 mm Bordsteine folgen der Neigung als je eine Extrusion (ein Körper mit Wiederholung über die 1,3 m), die beiden Handläufe sind Ketten aus Rohrkapseln 42,4 mm, deren runde Gelenke die Knicke bilden, mit Rückführungen an beiden Enden; Pfosten und Fussplatten 120 x 20 x 100 mm stehen auf dem Bord in zwei regelmässigen Dreiergruppen je Seite (je ein Körper mit einer der Neigung folgenden Wiederholung)."),
 ("The handrail bone starts at x = 1.2 m, so the overall length 10.5 m is measured from the handrail start to the ramp end at 11.7 m; the 2D elevation draws the ramp from x = 1.5 m; the chair carrier bone is a load point without volume.", "Der Handlauf-Knochen beginnt bei x = 1,2 m, die Gesamtlänge 10,5 m wird daher vom Handlaufbeginn bis zum Rampenende bei 11,7 m gemessen; die 2D-Ansicht zeichnet die Rampe ab x = 1,5 m; der Rollstuhl-Lastträgerknochen ist ein Lastpunkt ohne Volumen."),
])
