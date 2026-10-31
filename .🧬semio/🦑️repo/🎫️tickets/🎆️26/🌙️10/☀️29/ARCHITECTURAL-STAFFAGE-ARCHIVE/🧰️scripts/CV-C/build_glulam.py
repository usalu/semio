import sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CV-C")
from lib import *

AID = "glulam-beam"
doc = load_doc(AID)
a = doc["asset"]
merge_refs(a, [EXTRA_REFS["en-iso-13786"]])
a["dimensions"].update({"heightM": 0.62, "widthM": 8.0, "depthM": 0.16, "massKg": 322.56})
a["summary"] = T(
    "An 8 m GL24h glulam beam of 160 x 600 mm simply supported on steel bearing plates (pin left, roller right), carrying a uniform floor load and a moving concentrated load; fifteen 40 mm lamellae with staggered finger joints.",
    "Ein 8 m langer Brettschichtholzbalken GL24h 160 x 600 mm auf Stahllagerplatten (Festlager links, Rollenlager rechts) mit gleichmäßiger Deckenlast und wandernder Einzellast; fünfzehn Lamellen zu 40 mm mit versetzten Keilzinkenstößen.")

set_material(a, "glulam", conductivityWmK=0.13, specificHeatJKgK=1600)
set_material(a, "steel", title=T("Bearing plate steel S235", "Lagerplatte Stahl S235"), emissivity=0.85, solarAbsorptivity=0.5, densityKgM3=7850,
             conductivityWmK=50, specificHeatJKgK=450, elasticModulusPa=210000000000, strengthPa=235000000, source="literature", reference="en-10025-2")
if not any(r["id"] == "en-10025-2" for r in a["provenance"]["references"]):
    a["provenance"]["references"].append({"id": "en-10025-2", "citation": "EN 10025-2:2019 Hot rolled products of structural steels, non-alloy structural steels (S235 yield strength 235 MPa)."})

a["thermal"] = {
    "environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0,
                    "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
    "operating": [],
    "regions": [
        region("beam-sides", "Beam side faces", "Balkenseitenflächen", "surface", "passive", "glulam", 9.6, 0.6, "vertical-plate", 0.5, [], 24000, "authored-estimate", "en-iso-13786"),
        region("beam-top", "Beam top", "Balkenoberseite", "surface", "passive", "glulam", 1.28, 0.08, "horizontal-plate-up", 0.95, [], 24000, "authored-estimate", "en-iso-13786"),
        region("beam-soffit", "Beam soffit", "Balkenunterseite", "surface", "passive", "glulam", 1.28, 0.08, "horizontal-plate-down", 0.05, [], 24000, "authored-estimate", "en-iso-13786"),
        region("bearing-steel", "Bearing plates", "Lagerplatten", "surface", "passive", "steel", 0.04, 0.12, "horizontal-plate-down", 0.05, [], 9000, "authored-estimate", "en-iso-10456"),
    ],
    "emitters": [],
    "assumptions": [
        T("Passive timber faces store heat in an effective depth of about 36 mm (half the 24 h penetration depth of 73 mm for lambda 0.13 W/mK, rho 420 kg/m3, c 1600 J/kgK, EN ISO 13786), so the beam responds slowly to sun and shade.",
          "Passive Holzflächen speichern Wärme in einer wirksamen Tiefe von etwa 36 mm (die Hälfte der 24-h-Eindringtiefe von 73 mm für Lambda 0,13 W/mK, Rho 420 kg/m³, c 1600 J/kgK, EN ISO 13786); der Balken reagiert daher träge auf Sonne und Schatten."),
        T("The beam is a free-standing element in the open (no conditioned side); the exposed side face is drawn, the opposite face is the same region.",
          "Der Balken ist ein freistehendes Bauteil im Freien (keine konditionierte Seite); die gezeichnete Seitenfläche und die gegenüberliegende bilden dieselbe Region."),
        T("Sun-exposed and shaded faces differ through the pose sunlit fractions: top fully sunlit, side face half sunlit, soffit and bearing plates shaded.",
          "Besonnte und verschattete Flächen unterscheiden sich über die Besonnungsanteile der Posen: Oberseite voll besonnt, Seitenfläche halb besonnt, Unterseite und Lagerplatten verschattet."),
        T("Reference case: a mild sunny day, all values authored estimates.", "Referenzfall: ein milder sonniger Tag, alle Werte sind Schätzungen."),
    ],
}
bind_poses(a, {"beam-sides": (1, 0.5), "beam-top": (1, 1.0), "beam-soffit": (1, 0.0), "bearing-steel": (1, 0.0)})

D = Draw(a["rig"]["bones"])
B = "beam"
ALL = ["beam-sides", "beam-top", "beam-soffit", "bearing-steel"]
TIM = ["beam-sides", "beam-top", "beam-soffit"]

r500 = rep(500, [D.rect("beam", B, "body", 1, True, 0.4, 0, 0, 8, 0.62, regions=ALL, material="glulam")], dec(
    ("One solid bar 8.00 x 0.62 m (16 x 1.24 mm); the bearing plates are carried in the outline of the bar.",
     "Ein Vollkörper 8,00 x 0,62 m (16 x 1,24 mm); die Lagerplatten sind in der Kontur des Balkens enthalten."),
    ("Span, depth and the exact overall height including the 20 mm plate level.", "Spannweite, Höhe und die genaue Gesamthöhe einschließlich der 20-mm-Lagerplattenlage."),
    ("Timber side faces, top, soffit and the two steel bearing plates are one filled bar.", "Holzseiten, Ober- und Unterseite und die beiden Stahllagerplatten sind ein gefüllter Balken."),
    ("Lamination, finger joints, plate bolts and the end lines.", "Lamellen, Keilzinkenstöße, Lagerschrauben und die Stirnlinien."),
    ("There are no thin parts; the 20 mm plates (0.04 mm) are merged into the bar bottom, a deviation of 0.04 mm.", "Es gibt keine dünnen Teile; die 20-mm-Platten (0,04 mm) gehen im Balkenfuß auf, eine Abweichung von 0,04 mm."),
    ("No hatch: a 1.24 mm bar cannot carry a printable hatch.", "Keine Schraffur: ein 1,24-mm-Balken trägt keine druckbare Schraffur."),
    ("Mid-dark filled bar reads as timber.", "Der mitteldunkle Balken liest sich als Holz."),
    ("Overlays colour the whole bar with the area-weighted value of side faces, top, soffit and bearing plates (labelled merged).",
     "Überlagerungen färben den ganzen Balken mit dem flächengewichteten Wert von Seitenflächen, Ober-, Unterseite und Lagerplatten (als zusammengefasst gekennzeichnet).")))

r200 = rep(200, [
    D.rect("beam", B, "body", 1, True, 0.6, 0, 0.02, 8, 0.62, regions=TIM, material="glulam"),
    D.line("bearing-left", B, "body", 2, 0.3, [(0, 0.01), (0.12, 0.01)], region="bearing-steel", material="steel", penMm=0.18),
    D.line("bearing-right", B, "body", 2, 0.3, [(7.88, 0.01), (8, 0.01)], region="bearing-steel", material="steel", penMm=0.18)],
    dec(("A single timber bar (40 x 3 mm) with two bearing lines under its ends.", "Ein einzelner Holzbalken (40 x 3 mm) mit zwei Auflagerlinien unter den Enden."),
        ("Span, depth 0.60 m and the two bearing lengths of 0.12 m.", "Spannweite, Höhe 0,60 m und die beiden Auflagerlängen von 0,12 m."),
        ("Side faces, top and soffit are one bar shape.", "Seitenflächen, Oberseite und Unterseite bilden eine Balkenform."),
        ("Lamination (0.2 mm spacing), finger joints and bolts.", "Lamellen (0,2 mm Abstand), Keilzinkenstöße und Schrauben."),
        ("The 20 mm bearing plates are lines of 0.18 mm pen (36 mm), 16 mm taller than real.", "Die 20-mm-Lagerplatten sind Linien mit 0,18 mm Feder (36 mm), 16 mm höher als real."),
        ("No hatch: lamella spacing is below the printable minimum.", "Keine Schraffur: der Lamellenabstand liegt unter dem druckbaren Minimum."),
        ("Mid-grey bar, dark bearing lines.", "Mittelgrauer Balken, dunkle Auflagerlinien."),
        ("Overlays colour the bar (merged faces) and the bearing lines separately.", "Überlagerungen färben Balken (zusammengefasste Flächen) und Auflagerlinien getrennt.")))

h100 = [H("grain", "Timber, longitudinal", "Holz, Längsschnitt", "lines", 0, 0.8, 0.1)]
r100 = rep(100, [
    D.rect("beam", B, "body", 1, True, 0.6, 0, 0.02, 8, 0.62, regions=TIM, material="glulam", hatch="grain"),
    D.rect("bearing-left", B, "body", 2, True, 0.3, 0, 0, 0.12, 0.02, region="bearing-steel", material="steel", penMm=0.1),
    D.rect("bearing-right", B, "body", 2, True, 0.3, 7.88, 0, 8, 0.02, region="bearing-steel", material="steel", penMm=0.1)],
    dec(("A timber bar (80 x 6 mm) with grain hatch on two steel bearing plates.", "Ein Holzbalken (80 x 6 mm) mit Faserschraffur auf zwei Stahllagerplatten."),
        ("Bar, grain direction and both plates (1.2 x 0.2 mm).", "Balken, Faserrichtung und beide Platten (1,2 x 0,2 mm)."),
        ("Lamellae (0.4 mm) are expressed by a grain hatch at every second glue line (0.8 mm).", "Lamellen (0,4 mm) sind durch eine Faserschraffur an jeder zweiten Leimfuge (0,8 mm) ausgedrückt."),
        ("Finger joints and bolts.", "Keilzinkenstöße und Schrauben."),
        ("The 20 mm plates are 0.2 mm thick filled rectangles with a 0.1 mm outline.", "Die 20-mm-Platten sind 0,2 mm dicke gefüllte Rechtecke mit 0,1 mm Kontur."),
        ("Timber long-section hatch: parallel lines along the grain at 0.8 mm (DIN ISO 128-50, DIN 1356-1).", "Holz im Längsschnitt: parallele Linien in Faserrichtung, 0,8 mm Abstand (DIN ISO 128-50, DIN 1356-1)."),
        ("Outline 0.25 mm, hatch 0.1 mm, plates darker.", "Kontur 0,25 mm, Schraffur 0,1 mm, Platten dunkler."),
        ("Bar (merged faces) and plates are separate overlay cells.", "Balken (zusammengefasste Flächen) und Platten sind getrennte Überlagerungszellen.")), hatches=h100)

s50 = [
    D.rect("beam", B, "body", 1, True, 0.6, 0, 0.02, 8, 0.62, regions=TIM, material="glulam"),
    D.rect("bearing-left", B, "body", 2, True, 0.3, 0, 0, 0.12, 0.02, region="bearing-steel", material="steel"),
    D.rect("bearing-right", B, "body", 2, True, 0.3, 7.88, 0, 8, 0.02, region="bearing-steel", material="steel")]
for i in range(1, 15):
    y = 0.02 + 0.04 * i
    s50.append(D.line(f"glue-line-{i:02d}", B, "texture", 1.5, 0.3, [(0, y), (8, y)], region="beam-sides"))
fj = [(1.90, 5), (3.70, 8), (5.30, 3), (6.60, 11), (2.80, 12), (4.40, 1)]
for j, (x, lam) in enumerate(fj):
    y0 = 0.02 + 0.04 * (lam - 1)
    s50.append(D.line(f"finger-joint-{j}", B, "detail", 1.6, 0.3, [(x, y0), (x + 0.02, y0 + 0.01), (x, y0 + 0.02), (x + 0.02, y0 + 0.03), (x, y0 + 0.04)], region="beam-sides"))
r50 = rep(50, s50, dec(
    ("Side elevation of the 600 mm deep beam (12 mm tall and 160 mm long on paper) with fifteen lamellae, six staggered finger joints and two steel bearing plates.",
     "Seitenansicht des 600 mm hohen Balkens (12 mm hoch und 160 mm lang auf Papier) mit fünfzehn Lamellen, sechs versetzten Keilzinkenstößen und zwei Stahllagerplatten."),
    ("All 14 glue lines at 40 mm spacing (0.8 mm), the two 120 x 20 mm bearing plates (2.4 x 0.4 mm), staggered finger joints.",
     "Alle 14 Leimfugen im Abstand von 40 mm (0,8 mm), die beiden Lagerplatten 120 x 20 mm (2,4 x 0,4 mm), versetzte Keilzinkenstöße."),
    ("Side faces, top and soffit remain one bar shape; the glue lines are bound to the side faces.", "Seitenflächen, Oberseite und Unterseite bleiben eine Balkenform; die Leimfugen sind an die Seitenflächen gebunden."),
    ("Plate bolts (12 mm, 0.24 mm) and the real finger pitch (12 mm).", "Lagerschrauben (12 mm, 0,24 mm) und die reale Zinkenteilung (12 mm)."),
    ("Finger joints are two-tooth symbols with a 40 mm pitch (0.8 mm), because the real pitch (0.24 mm) is below the printable minimum.",
     "Keilzinkenstöße sind Zeichen mit zwei Zinken und 40 mm Teilung (0,8 mm), weil die echte Teilung (0,24 mm) unter dem druckbaren Minimum liegt."),
    ("No hatch fill: the glue lines are the grain indication of the longitudinal view.", "Keine Schraffurfüllung: die Leimfugen sind die Faserandeutung der Längsansicht."),
    ("Outline 0.35 mm, glue lines 0.1 mm, finger joints 0.18 mm.", "Kontur 0,35 mm, Leimfugen 0,1 mm, Keilzinken 0,18 mm."),
    ("Faces are one merged bar cell, bearing plates their own cells.", "Die Flächen sind eine zusammengefasste Balkenzelle, die Lagerplatten eigene Zellen.")), hatches=[])
a["representations"] = {"500": r500, "200": r200, "100": r100, "50": r50}
a["symbols"] = []
out = finish(a)
doc["asset"] = out
print(AID)
validate(out)
save_doc(AID, doc)
