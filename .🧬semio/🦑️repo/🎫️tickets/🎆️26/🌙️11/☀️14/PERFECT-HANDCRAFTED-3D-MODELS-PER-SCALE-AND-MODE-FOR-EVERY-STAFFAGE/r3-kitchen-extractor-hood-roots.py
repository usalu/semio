"""🌱 Round 3, appliances/kitchen-extractor-hood: the root fixes (pyramid canopy, rig joints at the real seats, masses, anchors, materials, thermal regions, texts) and the 2D drawings of 1:50, 1:100 and 1:200 redrawn as the true views of each scale's body (ruling E1); writes the candidate `🗑️generated/r3-kitchen-extractor-hood/🔣️.json` with the volumes source merged in.

Usage: python3 r3-kitchen-extractor-hood-roots.py [--write]   (--write re-reads the catalog file and writes the root fixes into it; splice the volumes with volumes-insert.py afterwards)
"""
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/kitchen-extractor-hood/🔣️.json"))
SOURCE = os.path.join(HERE, "volumes", "appliances", "kitchen-extractor-hood.json")
OUT = os.path.join(HERE, "🗑️generated", "r3-kitchen-extractor-hood", "🔣️.json")
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-kitchen-extractor-hood-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)
text = k.text

AREAS = {"canopy": 0.97, "filter": 0.57, "lamps": 0.025}
SS, GALV, ABS = "stainless-steel", "galvanised-steel", "switch-plastic"
TOP_Y = k.TOP + k.LOWER_H
STUB_Y = TOP_Y + k.UPPER_H


def shape(sid, bone, role, z, filled, tone, kind, region=None, material=None, **geometry):
    """🧩 A 2D shape in the member order of the catalog."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    out.update(geometry)
    if region:
        out["region"] = region
    if material:
        out["material"] = material
    return out


def line(sid, bone, z, points, tone=0.35):
    """✏️ An open detail stroke."""
    return shape(sid, bone, "detail", z, False, tone, "poly", points=k.pts(points), closed=False)


def rect(sid, bone, z, tone, x, y, w, h, region=None, material=SS, role="body"):
    """⬜ A filled rectangle."""
    return shape(sid, bone, role, z, True, tone, "rect", region, material, x=k.r5(x), y=k.r5(y), w=k.r5(w), h=k.r5(h))


def elevation(scale):
    """📐 The side elevation: the wall at x = 0, every shape in its bone frame."""
    upper_d, upper_h = (k.LOWER_D, k.UPPER_H + k.STUB) if scale == "200" else (k.LOWER_D if scale == "100" else k.UPPER_D, k.UPPER_H)
    out = [shape("canopy", "canopy", "body", 3, True, 0.55, "poly", "canopy", SS, points=k.pts([(0, 0), (k.DEPTH, 0), (k.DEPTH, k.RIM), (k.TOP_D, k.TOP), (0, k.TOP)]), closed=True)]
    if scale != "200":
        out.append(line("rim-edge", "canopy", 3.3, [(0, k.RIM), (k.DEPTH, k.RIM)]))
    out.append(rect("chimney-lower", "chimney", 3.4, 0.6, 0, 0, k.LOWER_D, k.LOWER_H, "chimney"))
    out.append(rect("chimney-upper", "chimney-top", 3.5, 0.65, 0, 0, upper_d, upper_h, "chimney"))
    if scale != "200":
        out.append(rect("collar", "chimney-top", 3.6, 0.45, k.DUCT_X - k.DUCT_R, k.UPPER_H, 2 * k.DUCT_R, k.STUB, None, GALV))
    if scale == "50":
        out.append(line("seam", "chimney-top", 3.55, [(0, 0), (k.UPPER_D, 0)], 0.3))
        out += [line(f"louvre-{i + 1}", "chimney-top", 3.56, [(0.05, 0.13 + 0.02 * i), (0.14, 0.13 + 0.02 * i)], 0.3) for i in range(5)]
    return out


def front(scale):
    """🖼️ The front elevation: x lateral, y the height above the rim underside."""
    hw_u, upper_h = (k.LOWER_HW, k.UPPER_H + k.STUB) if scale == "200" else (k.LOWER_HW if scale == "100" else k.UPPER_HW, k.UPPER_H)
    out = [shape("canopy", "canopy", "body", 3, True, 0.55, "poly", "canopy", SS, points=k.pts([(-k.HALF_W, 0), (k.HALF_W, 0), (k.HALF_W, k.RIM), (k.TOP_HW, k.TOP), (-k.TOP_HW, k.TOP), (-k.HALF_W, k.RIM)]), closed=True)]
    if scale != "200":
        out.append(line("rim-edge", "canopy", 3.3, [(-k.HALF_W, k.RIM), (k.HALF_W, k.RIM)]))
    if scale == "50":
        out.append(rect("switch-panel", "canopy", 3.2, 0.2, 0.155, 0.011, 0.09, 0.023, None, ABS, "detail"))
    out.append(rect("chimney-lower", "chimney", 3.4, 0.6, -k.LOWER_HW, k.TOP, 2 * k.LOWER_HW, k.LOWER_H, "chimney"))
    out.append(rect("chimney-upper", "chimney-top", 3.5, 0.65, -hw_u, TOP_Y, 2 * hw_u, upper_h, "chimney"))
    if scale != "200":
        out.append(rect("collar", "chimney-top", 3.6, 0.45, -k.DUCT_R, STUB_Y, 2 * k.DUCT_R, k.STUB, None, GALV))
    if scale == "50":
        out.append(line("seam", "chimney-top", 3.55, [(-k.UPPER_HW, TOP_Y), (k.UPPER_HW, TOP_Y)], 0.3))
    return out


def plan(scale):
    """🗺️ The plan from above: x out from the wall, y lateral."""
    out = [rect("canopy", "canopy", 3, 0.55, 0, -k.HALF_W, k.DEPTH, 2 * k.HALF_W, "canopy")]
    out.append(line("hip-left", "canopy", 3.3, [(k.TOP_D, k.TOP_HW), (k.DEPTH, k.HALF_W)]))
    out.append(line("hip-right", "canopy", 3.3, [(k.TOP_D, -k.TOP_HW), (k.DEPTH, -k.HALF_W)]))
    if scale == "200":
        out.append(rect("chimney", "chimney", 3.4, 0.6, 0, -k.LOWER_HW, k.LOWER_D, 2 * k.LOWER_HW, "chimney"))
        return out
    out.append(rect("chimney-lower", "chimney", 3.4, 0.6, 0, -k.LOWER_HW, k.LOWER_D, 2 * k.LOWER_HW, "chimney"))
    if scale == "50":
        out.append(rect("chimney-upper", "chimney-top", 3.5, 0.65, 0, -k.UPPER_HW, k.UPPER_D, 2 * k.UPPER_HW, "chimney"))
    out.append(shape("collar", "chimney-top", "body", 3.6, True, 0.45, "ellipse", None, GALV, cx=k.DUCT_X, cy=0, rx=k.DUCT_R, ry=k.DUCT_R))
    return out


DECISIONS = {
    "50": {
        "silhouette": text("A 0.5 m by 0.915 m side profile, 10 mm by 18.3 mm on paper: the 45 mm rim band, the front slope rising from the rim to the 0.2 m deep top plate at 0.246 m, and the two-part chimney of 0.19 m depth with the duct stub on top. Front view: the 0.9 m rim under the trapezoid of the two side slopes up to the 0.26 m top plate, the switch panel on the rim and the 0.25 m chimney. Top view: the 0.5 m by 0.9 m canopy with its two hips running from the top plate to the front corners, the chimney channel and the round duct in it.",
                           "Ein Seitenprofil von 0,5 m mal 0,915 m, auf Papier 10 mm mal 18,3 mm: das Randband von 45 mm, die Frontschräge vom Rand zur 0,2 m tiefen Deckplatte auf 0,246 m und der zweiteilige Kamin von 0,19 m Tiefe mit dem Rohrstutzen obenauf. Vorderansicht: der 0,9 m breite Rand unter dem Trapez der beiden Seitenschrägen bis zur 0,26 m breiten Deckplatte, die Schalterblende am Rand und der 0,25 m breite Kamin. Draufsicht: die Haube von 0,5 m mal 0,9 m mit ihren zwei Graten von der Deckplatte zu den vorderen Ecken, der Kaminschacht und das runde Rohr darin."),
        "retained": text("Rim band of 45 mm with its upper edge, the pyramid slopes (front 34 degrees, sides 32 degrees), the top plate, the lower chimney channel of 0.25 m by 0.19 m by 0.4 m and the upper one of 0.244 m by 0.187 m with 254 mm visible, the telescope seam, five louvre slots on the side of the upper channel, the 150 mm duct stub of 15 mm, the switch panel in the front view and the two hips in the top view.",
                         "Randband von 45 mm mit seiner Oberkante, die Pyramidenschrägen (Front 34 Grad, Seiten 32 Grad), die Deckplatte, der untere Kaminschacht von 0,25 m mal 0,19 m mal 0,4 m und der obere von 0,244 m mal 0,187 m mit 254 mm sichtbarer Höhe, die Teleskopfuge, fünf Lüftungsschlitze an der Seite des oberen Schachts, der 150-mm-Rohrstutzen von 15 mm, die Schalterblende in der Vorderansicht und die zwei Grate in der Draufsicht."),
        "merged": text("The back sheet on the wall is part of the canopy shape; the edge of the top plate lies 5 to 10 mm outside the chimney and falls into its outline.",
                       "Das Rückblech an der Wand gehört zur Haubenform; die Kante der Deckplatte liegt 5 bis 10 mm außerhalb des Kamins und fällt mit dessen Umriss zusammen."),
        "omitted": text("The grease filters, the lamps and the base panel lie under the canopy, 6 mm above the rim edge, and are seen in none of the three views (hidden regions). The four buttons, the screws, the wall bracket and the fan unit inside are not drawn.",
                        "Die Fettfilter, die Lampen und das Bodenblech liegen unter der Haube, 6 mm über der Randkante, und sind in keiner der drei Ansichten zu sehen (verdeckte Regionen). Die vier Tasten, die Schrauben, der Wandwinkel und die Lüftereinheit im Inneren sind nicht gezeichnet."),
        "thinElements": text("Rim edge, seam, louvres and hips are 0.13 mm detail lines; the ten real louvre slots of 5 mm are drawn as five at a pitch of 20 mm (0.4 mm on paper); the duct stub is 15 mm (0.3 mm on paper).",
                             "Randkante, Fuge, Schlitze und Grate sind 0,13-mm-Detailstriche; die zehn echten Lüftungsschlitze von 5 mm sind als fünf im Abstand von 20 mm gezeichnet (0,4 mm auf Papier); der Rohrstutzen ist 15 mm hoch (0,3 mm auf Papier)."),
        "hatching": text("None.", "Keine."),
        "styles": text("Canopy 0.55, chimney 0.6 and 0.65, duct stub 0.45, switch panel 0.2; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.",
                       "Haube 0,55, Kamin 0,6 und 0,65, Rohrstutzen 0,45, Schalterblende 0,2; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."),
        "overlays": text("Canopy and chimney are the two visible regions: in the thermal view the canopy warms and the chimney stays cool; the filter and lamp regions are hidden under the canopy and appear in the 3D thermal model; the air leaves at the duct stub.",
                         "Haube und Kamin sind die zwei sichtbaren Regionen: in der Thermalansicht erwärmt sich die Haube, der Kamin bleibt kühl; die Regionen Filter und Lampen liegen verdeckt unter der Haube und erscheinen im thermischen 3D-Modell; die Luft tritt am Rohrstutzen aus."),
    },
    "100": {
        "silhouette": text("A 5 mm by 9.2 mm profile: a rim band under a sloping pyramid side and a two-part chimney with the duct stub. Front view: the 0.9 m rim under the trapezoid of the side slopes and the 0.25 m chimney. Top view: the canopy with its two hips and the chimney channel with the round duct.",
                           "Ein Profil von 5 mm mal 9,2 mm: ein Randband unter einer geneigten Pyramidenseite und ein zweiteiliger Kamin mit dem Rohrstutzen. Vorderansicht: der 0,9 m breite Rand unter dem Trapez der Seitenschrägen und der 0,25 m breite Kamin. Draufsicht: die Haube mit ihren zwei Graten und der Kaminschacht mit dem runden Rohr."),
        "retained": text("Rim band with its upper edge, the slopes, the top plate, both chimney channels, the duct stub and the two hips in the top view.",
                         "Randband mit seiner Oberkante, die Schrägen, die Deckplatte, beide Kaminschächte, der Rohrstutzen und die zwei Grate in der Draufsicht."),
        "merged": text("The back sheet is part of the canopy shape; the top plate edge falls into the chimney outline.",
                       "Das Rückblech gehört zur Haubenform; die Kante der Deckplatte fällt mit dem Kaminumriss zusammen."),
        "omitted": text("Telescope seam, louvres and the switch panel. Filters and lamps are hidden under the canopy in all three views.",
                        "Teleskopfuge, Lüftungsschlitze und die Schalterblende. Filter und Lampen sind in allen drei Ansichten unter der Haube verdeckt."),
        "thinElements": text("The rim band is 0.45 mm on paper and keeps its edge line; the 15 mm duct stub is 0.15 mm, the printable limit; the 3 mm telescopic step would be 0.03 mm and is merged: both chimney channels are drawn 0.25 m wide and 0.19 m deep, and the top view shows one channel.",
                             "Das Randband misst auf Papier 0,45 mm und behält seine Kantenlinie; der Rohrstutzen von 15 mm misst 0,15 mm, die Druckgrenze; der Teleskopabsatz von 3 mm wäre 0,03 mm und geht auf: beide Kaminschächte sind 0,25 m breit und 0,19 m tief gezeichnet, und die Draufsicht zeigt einen Schacht."),
        "hatching": text("None.", "Keine."),
        "styles": text("Canopy 0.55, chimney 0.6 and 0.65, duct stub 0.45; body pen 0.18 mm, detail pen 0.13 mm for the rim edge and the hips.",
                       "Haube 0,55, Kamin 0,6 und 0,65, Rohrstutzen 0,45; Körperstift 0,18 mm, Detailstift 0,13 mm für Randkante und Grate."),
        "overlays": text("Canopy and chimney are the visible regions; filter and lamps are hidden regions of the 3D thermal model.",
                         "Haube und Kamin sind die sichtbaren Regionen; Filter und Lampen sind verdeckte Regionen des thermischen 3D-Modells."),
    },
    "200": {
        "silhouette": text("A 2.5 mm by 4.6 mm profile: a rim under a sloping wedge and a plain chimney. Front view: a 0.9 m rim under a trapezoid and a 0.25 m chimney in two pieces. Top view: the canopy with its two hips and the chimney footprint.",
                           "Ein Profil von 2,5 mm mal 4,6 mm: ein Rand unter einem geneigten Keil und ein glatter Kamin. Vorderansicht: ein 0,9 m breiter Rand unter einem Trapez und ein 0,25 m breiter Kamin aus zwei Teilen. Draufsicht: die Haube mit ihren zwei Graten und die Kaminfläche."),
        "retained": text("Rim height, the slopes to the top plate, the chimney of 0.19 m by 0.25 m up to 0.915 m and the two hips in the top view.",
                         "Randhöhe, die Schrägen zur Deckplatte, der Kamin von 0,19 m mal 0,25 m bis 0,915 m und die zwei Grate in der Draufsicht."),
        "merged": text("The duct stub and the telescopic step join the upper chimney piece; the back sheet is part of the canopy.",
                       "Rohrstutzen und Teleskopabsatz gehen im oberen Kaminteil auf; das Rückblech gehört zur Haube."),
        "omitted": text("Rim edge line, seam, louvres, switch panel and the duct. Filters and lamps are hidden under the canopy.",
                        "Randkantenlinie, Fuge, Lüftungsschlitze, Schalterblende und das Rohr. Filter und Lampen sind unter der Haube verdeckt."),
        "thinElements": text("The rim band is 0.23 mm on paper; the chimney depth of 0.19 m is 0.95 mm.",
                             "Das Randband misst auf Papier 0,23 mm; die Kamintiefe von 0,19 m misst 0,95 mm."),
        "hatching": text("None.", "Keine."),
        "styles": text("Canopy 0.55, chimney 0.6 and 0.65; body pen 0.18 mm, detail pen 0.13 mm for the two hips of the top view.",
                       "Haube 0,55, Kamin 0,6 und 0,65; Körperstift 0,18 mm, Detailstift 0,13 mm für die zwei Grate der Draufsicht."),
        "overlays": text("Canopy and chimney are the visible regions; filter and lamps are hidden regions.",
                         "Haube und Kamin sind die sichtbaren Regionen; Filter und Lampen sind verdeckte Regionen."),
    },
}

REFERENCE = {"id": "bosch-dwp96bc50b", "citation": "Bosch DWP96BC50B, Series 2 wall-mounted pyramid cooker hood 90 cm, product data sheet with dimensional drawings (media3.bosch-home.com/Documents/specsheet/en-VN/DWP96BC50B.pdf, read 2026-11-20): 900 mm wide, 500 mm deep, body 246 mm high without chimney and 319 mm with the duct spigot, chimney 250 mm wide and 553 to 730 mm high, total 799 to 975 mm, hanging screws 270 mm apart at 190 mm and safety screws at 25 mm above the lower edge, outlet centre 85 mm from the wall, duct 150 or 120 mm, net weight 10.3 kg, 626 m3/h, 220 W connected load, two LED lamps, washable aluminium grease filter cassettes."}

NOTES = text(
    "Side elevation of a wall-mounted pyramid chimney hood of the common 0.9 m width class, dimensioned after the Bosch DWP96BC50B data sheet: a canopy 0.5 m deep and 0.246 m high made of a 45 mm rim band and three sloping sheets (front and two sides) under a top plate of 0.26 m by 0.2 m, three aluminium grease filter cassettes and two LED lamp modules in the base panel under it, a lower chimney channel 0.25 m wide, 0.19 m deep and 0.4 m high and a telescopic upper channel with 254 mm visible, and the 150 mm duct that leaves the chimney 85 mm from the wall and is cut 15 mm above it at 0.915 m total height (the data sheet allows 0.799 to 0.975 m). The wall lies at x = 0; the origin is the underside of the rim on the wall, and a call site places it about 0.65 m above the hob (IEC 60335-2-31 asks for at least 0.65 m over a gas hob). The width of 0.9 m is the depth in the elevation (depthM). The mass of 10.3 kg is the net weight of the data sheet: 7.6 kg canopy with the fan unit, 1.2 kg lower and 1.5 kg upper chimney channel with duct and bracket (split estimated from the sheet areas). At level 1 the extract airflow is 0.14 m3/s (500 m3/h) leaving at the duct; the fan motor and the lamps take 150 W (120 W motor, 30 W LED modules), which ends entirely as heat in the room: the asset releases it at the canopy shell and the lamp modules; the split is an authored estimate. The extracted cooking vapour itself is not modelled (temperature rise 0).",
    "Seitenansicht einer wandmontierten Pyramiden-Kaminhaube der üblichen Breitenklasse von 0,9 m, bemaßt nach dem Datenblatt der Bosch DWP96BC50B: ein Haubenkörper von 0,5 m Tiefe und 0,246 m Höhe aus einem Randband von 45 mm und drei geneigten Blechen (Front und zwei Seiten) unter einer Deckplatte von 0,26 m mal 0,2 m, darunter drei Aluminium-Fettfilterkassetten und zwei LED-Lampenmodule im Bodenblech, ein unterer Kaminschacht von 0,25 m Breite, 0,19 m Tiefe und 0,4 m Höhe und ein teleskopierbarer oberer Schacht mit 254 mm sichtbarer Höhe sowie das 150-mm-Rohr, das den Kamin 85 mm vor der Wand verlässt und 15 mm darüber bei 0,915 m Gesamthöhe geschnitten ist (das Datenblatt erlaubt 0,799 bis 0,975 m). Die Wand liegt bei x = 0; der Ursprung ist die Randunterseite an der Wand, und ein Aufrufer setzt ihn etwa 0,65 m über das Kochfeld (IEC 60335-2-31 verlangt mindestens 0,65 m über einem Gaskochfeld). Die Breite von 0,9 m ist die Tiefe in der Ansicht (depthM). Die Masse von 10,3 kg ist das Nettogewicht des Datenblatts: 7,6 kg Haubenkörper mit Lüftereinheit, 1,2 kg unterer und 1,5 kg oberer Kaminschacht mit Rohr und Winkel (Aufteilung aus den Blechflächen geschätzt). Auf Stufe 1 beträgt der Abluftstrom 0,14 m3/s (500 m3/h) und tritt am Rohr aus; Lüftermotor und Lampen nehmen 150 W auf (120 W Motor, 30 W LED-Module), die vollständig als Wärme im Raum enden: das Asset gibt sie am Haubenkörper und an den Lampenmodulen ab; die Aufteilung ist ein Schätzwert. Der abgesaugte Kochdunst selbst ist nicht modelliert (Temperaturerhöhung 0).")

MATERIALS = [
    {"id": GALV, "title": text("Galvanised steel sheet", "Verzinktes Stahlblech"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 480, "source": "literature", "reference": "incropera-2018"},
    {"id": ABS, "title": text("Black ABS switch panel", "Schwarze ABS-Schalterblende"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1050, "conductivityWmK": 0.2, "specificHeatJKgK": 1400, "source": "literature", "reference": "incropera-2018"},
]


def apply(a, volumes):
    """🛠️ Every root fix on the asset."""
    a["summary"] = text(
        "A wall-mounted pyramid chimney hood 0.9 m wide in side elevation: a stainless canopy sloping on three sides with three grease filter cassettes and two lamps under it and a telescopic chimney above; at full speed it extracts 500 m3/h and its motor and lamps release 150 W.",
        "Eine an der Wand montierte Pyramiden-Kaminhaube von 0,9 m Breite in der Seitenansicht: ein auf drei Seiten geneigter Edelstahlkörper mit drei Fettfilterkassetten und zwei Lampen darunter und einem Teleskopkamin darüber; auf voller Stufe saugt sie 500 m3/h ab, Motor und Lampen geben 150 W ab.")
    refs = [r for r in a["provenance"]["references"] if r["id"] != REFERENCE["id"]]
    a["provenance"]["references"] = refs + [REFERENCE]
    a["provenance"]["notes"] = NOTES
    a["dimensions"]["massKg"] = 10.3
    a["materials"] = [m for m in a["materials"] if m["id"] not in (GALV, ABS)] + MATERIALS
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    bones["canopy"]["mass"].update(kg=7.6, comX=0.18, comY=0.1)
    bones["chimney"].update(y=k.TOP, lengthM=k.LOWER_H)
    bones["chimney"]["mass"].update(kg=1.2, comX=0.13, comY=0.2)
    bones["chimney-top"].update(y=k.LOWER_H, lengthM=k.r5(k.UPPER_H + k.STUB))
    bones["chimney-top"]["mass"].update(kg=1.5, comX=0.12, comY=0.06)
    anchors = {x["id"]: x for x in a["anchors"]}
    anchors["outlet"].update(x=k.DUCT_X, y=k.r5(k.UPPER_H + k.STUB), title=text("Mouth of the 150 mm duct above the chimney", "Mündung des 150-mm-Rohrs über dem Kamin"))
    anchors["wall-hook"].update(y=k.HOOK_V, footprintWidthM=0.28, title=text("Two keyhole screws of the back sheet", "Zwei Schlüssellochschrauben des Rückblechs"))
    anchors["wall-rest"].update(y=k.REST_V, footprintWidthM=0.28, title=text("Two spacer pads at the lower edge of the back sheet", "Zwei Distanzpuffer an der Unterkante des Rückblechs"))
    regions = {r["id"]: r for r in a["thermal"]["regions"]}
    for rid, area in AREAS.items():
        regions[rid]["areaM2"] = area
    canopy = regions["canopy"]
    canopy["layers"][0].update(title=text("Stainless steel sheet 0.6 mm", "Edelstahlblech 0,6 mm"), thicknessM=0.0006, resistanceM2KW=4e-05, heatCapacityJM2K=2370)
    canopy["heatCapacityJM2K"] = 2370
    regions["chimney"]["characteristicLengthM"] = 0.65
    regions["filter"]["title"] = text("Grease filter cassettes", "Fettfilterkassetten")
    regions["filter"]["characteristicLengthM"] = 0.32
    hidden = {"filter": text("The three cassettes lie in the base panel under the canopy, 6 mm above the rim edge: no elevation and no plan sees them.", "Die drei Kassetten liegen im Bodenblech unter der Haube, 6 mm über der Randkante: keine Ansicht und keine Draufsicht sieht sie."),
              "lamps": text("The two lamp modules lie in the base panel under the canopy: no elevation and no plan sees them.", "Die zwei Lampenmodule liegen im Bodenblech unter der Haube: keine Ansicht und keine Draufsicht sieht sie.")}
    for rid, why in hidden.items():
        old = regions[rid]
        new = {}
        for key, value in old.items():
            if key == "hidden":
                continue
            if key == "operating" and "hidden" not in new:
                new["hidden"] = why
            new[key] = value
        if "hidden" not in new:
            rebuilt = {}
            for key, value in new.items():
                if key == "source":
                    rebuilt["hidden"] = why
                rebuilt[key] = value
            new = rebuilt
        old.clear()
        old.update(new)
    a["thermal"]["emitters"][0]["title"] = text("Extract air leaving at the duct above the chimney", "Abluft am Rohr über dem Kamin")
    a["structure"]["assumptions"] = [text(
        "The 10.3 kg hood hangs with its back sheet on two keyhole screws at 0.19 m (pin) and bears on two spacer pads at 0.03 m against the wall (roller): the screws carry the weight of 101 N, and the pair resists the overturning moment of the centre of mass 0.17 m from the wall with about 105 N of pull at the screws and of push at the pads (101 N x 0.167 m / 0.16 m). The chimney channels stand on the top plate and are guided by the wall bracket, which is not counted as a support.",
        "Die 10,3 kg schwere Haube hängt mit ihrem Rückblech an zwei Schlüssellochschrauben auf 0,19 m (Festlager) und liegt auf zwei Distanzpuffern bei 0,03 m an der Wand an (Rollenlager): die Schrauben tragen das Gewicht von 101 N, und das Paar nimmt das Kippmoment des Schwerpunkts 0,17 m vor der Wand mit etwa 105 N Zug an den Schrauben und Druck an den Puffern auf (101 N x 0,167 m / 0,16 m). Die Kaminschächte stehen auf der Deckplatte und werden vom Wandwinkel geführt, der nicht als Auflager zählt.")]
    reps = a["representations"]
    for scale in ("50", "100", "200"):
        reps[scale]["geometry"] = {"elevation": elevation(scale), "plan": plan(scale), "front": front(scale)}
        reps[scale]["decisions"] = DECISIONS[scale]
        reps[scale]["pens"]["detail"] = 0.13
    reps["500"].update(
        reason=text("At 1:500 the 0.5 m by 0.915 m hood is 1 mm by 1.8 mm and its 45 mm rim band 0.09 mm, below every print profile.", "Bei 1:500 ist die 0,5 m mal 0,915 m große Haube 1 mm mal 1,8 mm groß und ihr Randband von 45 mm 0,09 mm, unter jedem Druckprofil."),
        limitingFeature=text("Rim band of 45 mm", "Randband von 45 mm"), limitingSizeM=0.045)
    a["volumes"] = volumes


def main():
    doc = json.load(open(CATALOG, encoding="utf-8"))
    apply(doc["asset"], json.load(open(SOURCE, encoding="utf-8"))["volumes"])
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    target = CATALOG if "--write" in sys.argv[1:] else OUT
    with open(target, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, indent=2, ensure_ascii=False)
        fh.write("\n")
    print("written", target)


if __name__ == "__main__":
    main()
