import sys
from build import *

REL = "🚜️vehicles/city-bus"
SHELL = "M 0.15,0.3 L 2.83,0.3 C 2.83,0.8 3.1,1.12 3.45,1.12 C 3.8,1.12 4.07,0.8 4.07,0.3 L 8.73,0.3 C 8.73,0.8 9.0,1.12 9.35,1.12 C 9.7,1.12 9.97,0.8 9.97,0.3 L 11.85,0.3 C 11.97,0.32 12.0,0.4 12.0,0.5 L 12.0,0.95 C 12.0,1.0 11.99,1.05 11.98,1.1 L 11.78,2.7 C 11.76,2.8 11.72,2.88 11.62,2.93 C 11.5,2.97 11.3,2.98 11.0,2.98 L 1.0,2.98 C 0.5,2.98 0.15,2.95 0.06,2.85 L 0.0,2.6 L 0.0,0.5 C 0.0,0.38 0.05,0.3 0.15,0.3 Z"
SHELL500 = "M 0.15,0.3 L 11.85,0.3 C 11.97,0.32 12.0,0.4 12.0,0.5 L 12.0,0.95 C 12.0,1.0 11.99,1.05 11.98,1.1 L 11.78,2.7 C 11.76,2.8 11.72,2.88 11.62,2.93 C 11.5,2.97 11.3,2.98 11.0,2.98 L 1.0,2.98 C 0.5,2.98 0.15,2.95 0.06,2.85 L 0.0,2.6 L 0.0,0.5 C 0.0,0.38 0.05,0.3 0.15,0.3 Z"

KEYS = [
    opkey("traction", "Traction drive", "Fahrantrieb", [(0, "Off", "Aus"), (0.05, "Ready, standing", "Bereit, stehend"), (0.5, "Cruising", "Reisefahrt"), (1, "Full power", "Volle Leistung")]),
    opkey("rolling", "Rolling speed", "Rollgeschwindigkeit", [(0, "Standstill", "Stillstand"), (0.5, "About 25 km/h", "Etwa 25 km/h"), (1, "About 50 km/h", "Etwa 50 km/h")]),
    opkey("braking", "Friction braking", "Reibbremsen", [(0, "Not braking", "Nicht gebremst"), (0.3, "Service braking", "Betriebsbremsung"), (1, "Full stop from 50 km/h", "Vollbremsung aus 50 km/h")]),
    opkey("hvac", "Heat pump and ventilation", "Wärmepumpe und Lüftung", [(0, "Off", "Aus"), (0.3, "Ventilation", "Lüftung"), (0.7, "Climate control", "Klimatisierung"), (1, "Maximum", "Maximum")]),
]
skin = lambda: layer("skin", "Painted aluminium or steel skin 1.5 mm", "Lackierte Aluminium- oder Stahlhaut 1,5 mm", "metal", "paint", 0.0015, 0.0015 / 50, 4000, source="literature", ref="incropera-2011")
insul = lambda: layer("insulation", "Mineral wool lining 40 mm", "Mineralwolle-Verkleidung 40 mm", "insulation", "interior-trim", 0.04, 0.04 / 0.04, 800, source="authored-estimate")
COND = None
REGIONS = [
    region("body-panels", "Painted side and end panels", "Lackierte Seiten- und Endbleche", "wall", "conditioned", "paint", 52, 2.4, "vertical-plate", 0.5, 8000, [skin(), insul()], ref="ashrae-2021"),
    region("roof", "Roof panel", "Dachblech", "wall", "conditioned", "paint", 30, 2.5, "horizontal-plate-up", 0.95, 8000, [skin(), insul()], ref="ashrae-2021"),
    region("glazing", "Window and door glass", "Fenster- und Türverglasung", "glazing", "conditioned", "glass", 24, 1.2, "vertical-plate", 0.6, 15000,
           [layer("pane-out", "Toughened glass 4 mm", "Einscheibensicherheitsglas 4 mm", "glass", "glass", 0.004, 0.004, 7500, source="literature", ref="incropera-2011"),
            layer("gap", "Air gap 12 mm", "Luftspalt 12 mm", "air-gap", "glass", 0.012, 0.15, None, source="literature", ref="incropera-2011"),
            layer("pane-in", "Toughened glass 4 mm", "Einscheibensicherheitsglas 4 mm", "glass", "glass", 0.004, 0.004, 7500, source="literature", ref="incropera-2011")], ref="ashrae-2021"),
    region("doors", "Door leaf frames", "Türflügelrahmen", "wall", "conditioned", "door-leaf", 8, 2.1, "vertical-plate", 0.5, 6000, [skin(), insul()], ref="ashrae-2021"),
    region("battery-pack", "Roof battery housing", "Dach-Batteriegehäuse", "surface", "operating", "battery-housing", 30, 6.3, "horizontal-plate-up", 0.95, 12000,
           [layer("housing", "Aluminium housing 3 mm", "Aluminiumgehäuse 3 mm", "metal", "battery-housing", 0.003, 0.003 / 160, 8000, source="literature", ref="incropera-2011")], ("traction", 4200)),
    region("hvac-unit", "Heat pump housing", "Wärmepumpengehäuse", "surface", "operating", "hvac-housing", 10, 2.5, "horizontal-plate-up", 0.9, 6000, [], ("hvac", 1800), ref="ashrae-2021"),
    region("e-axle", "Electric drive axle casting", "Gehäuse der elektrischen Antriebsachse", "engine", "operating", "e-axle", 3, 0.9, "cylinder", 0.2, 20000,
           [layer("casting", "Cast aluminium housing 8 mm", "Aluminiumgussgehäuse 8 mm", "metal", "e-axle", 0.008, 0.008 / 150, 20000, source="authored-estimate")], ("traction", 1650)),
    region("tyres", "Tyre treads and sidewalls", "Reifenlaufflächen und -flanken", "tyre", "operating", "rubber", 6, 0.96, "cylinder", 0.4, 70000,
           [layer("tread", "Rubber tread and carcass", "Gummilauffläche und Karkasse", "polymer", "rubber", 0.015, 0.015 / 0.2, 70000, source="authored-estimate", ref="wong-2008")], ("rolling", 1800), ref="wong-2008"),
    region("brakes", "Brake discs seen through the wheel holes", "Bremsscheiben, durch die Radöffnungen gesehen", "surface", "operating", "cast-iron", 1.2, 0.4, "vertical-plate", 0.2, 53000, [], ("braking", 200000), ref="limpert-1999"),
]
EMITTERS = [emitter("hvac-vent", "vent", "Roof heat-pump exhaust air", "Abluft der Dach-Wärmepumpe", "hvac-vent", 90, 0.7, "hvac", 1.5, 22, 0.0)]
DEFAULTS = {"body-panels": (1, 0.5), "roof": (1, 1), "glazing": (1, 0.5), "doors": (1, 0.5), "battery-pack": (1, 1), "hvac-unit": (1, 1), "e-axle": (0.3, 0), "tyres": (1, 0.1), "brakes": (0.4, 0)}
CONTACTS = {"tyres": (0.04, 30)}


def P(tr, ro, br, hv):
    return {"operating": {"traction": tr, "rolling": ro, "braking": br, "hvac": hv}, "emitters": {"hvac-vent": 1 if hv > 0 else 0}}


POSES = {"level-cruise": P(0.5, 0.8, 0, 0.7), "at-stop-level": P(0.05, 0, 0.3, 0.5), "kneeling-at-stop": P(0, 0, 0.3, 0.9), "braking": P(0.7, 0.9, 1, 0.7),
         "arriving": P(0.4, 0.5, 0.8, 0.7), "pull-away-8": P(0.8, 0.6, 0, 0.7), "pull-away-18": P(0.5, 0.8, 0, 0.7)}


def fix_clips(clips):
    for c in clips:
        c["tracks"] = [t for t in c["tracks"] if t["target"] != "emitter:hvac-vent"]
        for t in c["tracks"]:
            if t["target"] == "operating:hvac":
                pass
        c["tracks"].append({"target": "operating:hvac", "ease": "smooth", "keys": [{"t": 0, "v": 0.7}, {"t": c["durationS"] * 0.6, "v": 0.9}, {"t": c["durationS"], "v": 0.9}]} if c["id"] == "arrive-kneel-doors" else {"target": "operating:hvac", "ease": "smooth", "keys": [{"t": 0, "v": 0.9}, {"t": 4, "v": 0.7}, {"t": 11, "v": 0.7}]})
        if c["id"] == "depart":
            c["events"].insert(0, {"t": 0.5, "kind": "ignition", "title": tx("Traction system ready", "Antrieb betriebsbereit"), "target": "hvac-vent"})
    c0 = [c for c in clips if c["id"] == "depart"][0]
    clips.append({"id": "layover-shutdown", "title": tx("Layover shutdown", "Abstellen in der Wendezeit"), "description": tx("The kneeling bus at a terminus switches off traction and heat pump; the surfaces cool.", "Der abgesenkte Bus an der Endstelle schaltet Antrieb und Wärmepumpe ab; die Oberflächen kühlen aus."),
                  "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "kneeling-at-stop", "ease": "linear"}, {"t": 6, "pose": "kneeling-at-stop", "ease": "linear"}],
                  "contacts": [{"anchor": "contact-f", "fromS": 0, "toS": 6}, {"anchor": "contact-r", "fromS": 0, "toS": 6}],
                  "phases": [{"id": "run-down", "fromS": 0, "toS": 6, "title": tx("Systems run down", "Systeme fahren herunter")}],
                  "events": [{"t": 2, "kind": "shutdown", "title": tx("Shutdown", "Abschaltung"), "target": "hvac-vent"}],
                  "tracks": [{"target": "operating:traction", "ease": "smooth", "keys": [{"t": 0, "v": 0.05}, {"t": 2, "v": 0}]},
                             {"target": "operating:hvac", "ease": "smooth", "keys": [{"t": 0, "v": 0.9}, {"t": 2, "v": 0.9}, {"t": 3, "v": 0}]},
                             {"target": "operating:braking", "ease": "smooth", "keys": [{"t": 0, "v": 0.3}, {"t": 2, "v": 0}]},
                             {"target": "emitter:hvac-vent", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 3, "v": 0}]}]})


def reps(a, old):
    R = {}
    G = lambda: None
    door_open = lambda: [rect("door-opening-m", "body", "detail", 1, 6.35, 0.34, 1.1, 2.12, tone=0.05, material="interior-trim"), rect("door-opening-f", "body", "detail", 1, 10.35, 0.34, 1.1, 2.12, tone=0.05, material="interior-trim")]
    leaves = lambda glass_only=False: [
        rect("leaf-%s-%s" % (g, s), "door-%s-%s" % (g, s), "body", 3, 0, 0, 0.55, 2.12, tone=0.8, regions=["doors", "glazing"], material="door-leaf") for g in "mf" for s in ("aft", "fwd")]
    battery = lambda t: path("battery-pack", "battery", "body", 3, "M 0,0 L %s,%s L %s,%s L 6.3,0 Z" % (0.12 if t < 0.15 else 0.1, t, 6.1 if t < 0.15 else 6.2, t), 0.45, True, region="battery-pack", material="battery-housing")
    hvac = lambda h=0.16: path("hvac-unit", "hvac", "body", 3, "M 0,0 L 0.12,%s L 2.28,%s L 2.5,0 Z" % (h, h), 0.5, True, region="hvac-unit", material="hvac-housing")
    wh = lambda sfx, bone, kind: [ell("tyre-" + sfx, bone, "body", 4, 0, 0, 0.48, tone=0.1, region="tyres", material="rubber")] + kind(sfx, bone)
    w500 = lambda sfx, bone: []
    axle = [rect("e-axle-casting", "axle-r", "detail", 3.5, -0.25, -0.15, 0.5, 0.3, tone=0.3, region="e-axle", material="e-axle")]

    # 500
    e500 = [path("body-shell", "body", "body", 2, SHELL500, 0.62, True, regions=["body-panels", "roof"], material="paint")] + [dict(d, role="body", region="doors") for d in door_open()] + [
        path("window-band-a", "body", "glazing", 6, "M 0.3,1.2 L 6.35,1.2 L 6.35,2.78 L 0.3,2.78 Z", 0.85, True, region="glazing", material="glass"),
        path("window-band-b", "body", "glazing", 6, "M 7.45,1.2 L 10.35,1.2 L 10.35,2.78 L 7.45,2.78 Z", 0.85, True, region="glazing", material="glass"),
        path("window-band-c", "body", "glazing", 6, "M 11.5,1.2 L 11.88,1.2 L 11.78,2.72 L 11.5,2.72 Z", 0.85, True, region="glazing", material="glass"),
        battery(0.16), hvac(0.19)]
    e500 += [ell("tyre-f", "wheel-f", "body", 4, 0, 0, 0.48, tone=0.1, regions=["tyres", "brakes"], material="rubber"), ell("tyre-r", "wheel-r", "body", 4, 0, 0, 0.48, tone=0.1, regions=["tyres", "brakes"], material="rubber")]
    e500[0]["regions"] = ["body-panels", "roof", "e-axle"]
    R["500"] = rep(500, *RANGES[500], PRINT_PENS[500], [], e500, decisions(
        tx("One long low-floor box with rounded corners, the roof pods on top and two black wheel discs; the wheel arches are closed.", "Ein langer Niederflurkasten mit gerundeten Ecken, den Dachaufbauten und zwei schwarzen Radscheiben; die Radkästen sind geschlossen."),
        tx("Body, three glazing bands, two dark door openings (region doors; the leaves are not drawn), roof battery, heat pump, both wheels.", "Aufbau, drei Verglasungsbänder, zwei dunkle Türöffnungen (Region Türen; die Flügel sind nicht gezeichnet), Dachbatterie, Wärmepumpe, beide Räder."),
        tx("All window panes of a bay (pillars 0.1 to 0.15 m = 0.2 to 0.3 mm) form one band; each door opening is one dark shape (leaf glass and frame merge into it; the 0.55 m leaf stroke of 0.45 m is 0.2 mm on paper); rim, hub, brake and tyre form one disc; the e-axle merges into the body fill.", "Alle Scheiben einer Bucht (Pfosten 0,1 bis 0,15 m = 0,2 bis 0,3 mm) bilden ein Band; jede Türöffnung ist eine dunkle Form (Türglas und Rahmen gehen darin auf; der Flügelweg von 0,45 m ist 0,2 mm auf Papier); Felge, Nabe, Bremse und Reifen eine Scheibe; die E-Achse geht in der Aufbaufläche auf."),
        tx("Clerestory window row as a separate pane, pillar lines, belt line, skirt, lamps, mirror, seats, rails, wheel details.", "Obere Fensterreihe als eigene Scheibe, Pfostenlinien, Gürtellinie, Schweller, Leuchten, Spiegel, Sitze, Griffstangen, Raddetails."),
        tx("The roof pods are drawn 0.16 m high (battery real 0.14 m) to reach 0.32 mm; everything else is at real size.", "Die Dachaufbauten sind 0,16 m hoch gezeichnet (Batterie real 0,14 m), um 0,32 mm zu erreichen; alles andere in Realgröße."),
        tx("None.", "Keine."),
        tx("Body mid-tone, glass light, tyres black; one 0.15 mm outline pen and 0.13 mm for the glass edge.", "Aufbau mittlerer Ton, Glas hell, Reifen schwarz; ein Konturstift 0,15 mm und 0,13 mm für die Glaskante."),
        tx("Thermal overlays colour the body by the merged panel, roof and e-axle regions; battery, heat pump, glass, doors and tyres (with brakes) are separate.", "Thermische Überlagerungen färben den Aufbau mit den zusammengefassten Regionen Bleche, Dach und E-Achse; Batterie, Wärmepumpe, Glas, Türen und Reifen (mit Bremsen) sind getrennt.")))

    # 200
    panes = [(0.3, 1.45), (1.55, 2.7), (2.8, 4.1), (4.2, 5.35), (5.45, 6.25), (7.55, 8.65), (8.75, 9.85), (9.95, 10.25)]
    e200 = [path("body-shell", "body", "body", 2, SHELL, 0.62, True, region="body-panels", material="paint"),
            path("roof-band", "body", "detail", 2.1, "M 11.62,2.93 C 11.5,2.97 11.3,2.98 11.0,2.98 L 1.0,2.98 C 0.5,2.98 0.15,2.95 0.06,2.85 L 0.1,2.8 C 0.2,2.88 0.5,2.9 1.0,2.9 L 11.0,2.9 C 11.3,2.9 11.5,2.88 11.7,2.82 Z", 0.5, True, region="roof", material="paint")] + door_open()
    for i, (x0, x1) in enumerate(panes):
        e200.append(rect("pane-%d" % (i + 1), "body", "glazing", 6, x0, 1.2, x1 - x0, 1.2 + 0.0 if False else 1.2, tone=0.85, region="glazing", material="glass"))
    e200.append(path("pane-cab", "body", "glazing", 6, "M 11.56,1.2 L 11.88,1.2 L 11.83,2.4 L 11.56,2.4 Z", 0.85, True, region="glazing", material="glass"))
    e200.append(path("upper-band", "body", "glazing", 6, "M 0.3,2.48 L 10.25,2.48 L 10.25,2.78 L 0.3,2.78 Z M 10.35,2.48 L 11.76,2.48 L 11.75,2.72 L 10.35,2.72 Z", 0.85, True, region="glazing", material="glass"))
    e200 += [battery(0.14), hvac()] + [rect("leaf-%s-%s" % (g, s), "door-%s-%s" % (g, s), "body", 3, 0, -0.04, 0.55, 2.2, tone=0.75, region="doors", material="door-leaf") for g in "mf" for s in ("aft", "fwd")]
    e200 += [rect("leaf-%s-%s-glass" % (g, s), "door-%s-%s" % (g, s), "glazing", 3.1, 0.07, 0.5, 0.41, 1.5, tone=0.85, region="glazing", material="glass") for g in "mf" for s in ("aft", "fwd")]
    e200 += [path("skirt-line", "body", "detail", 7, "M 0.05,0.62 L 2.8,0.62 M 4.1,0.62 L 8.7,0.62 M 10.0,0.62 L 11.98,0.62", filled=False)]
    e200 += axle
    for sfx, bone in (("f", "wheel-f"), ("r", "wheel-r")):
        e200 += [ell("tyre-" + sfx, bone, "body", 4, 0, 0, 0.48, tone=0.1, region="tyres", material="rubber"), ell("rim-" + sfx, bone, "detail", 4.2, 0, 0, 0.286, tone=0.5, region="brakes", material="steel-wheel"), ell("hub-" + sfx, bone, "detail", 5.2, 0, 0, 0.11, tone=0.7, material="steel-wheel")]
    e200 += [rect("air-spring-f", "axle-f", "detail", 0.5, -0.14, 0.3, 0.28, 0.12, 0.04, tone=0.4, material="steel-wheel"), rect("air-spring-r", "axle-r", "detail", 0.5, -0.14, 0.3, 0.28, 0.12, 0.04, tone=0.4, material="steel-wheel")]
    # fix pane heights (rect h)
    for s in e200:
        if s["id"].startswith("pane-") and s["id"] != "pane-cab":
            s["h"] = 1.2
    R["200"] = rep(200, *RANGES[200], PRINT_PENS[200], [], e200, decisions(
        tx("Low-floor bus profile with arches (0.14 m = 0.7 mm clearance to the tyre), separate roof band, eight lower window panes, upper clerestory band, two door openings with sliding leaves, roof pods.", "Niederflurbusprofil mit Radläufen (0,14 m = 0,7 mm Abstand zum Reifen), eigenem Dachband, acht unteren Fensterscheiben, oberem Fensterband, zwei Türöffnungen mit Schiebeflügeln, Dachaufbauten."),
        tx("Pillars between panes (0.1 m = 0.5 mm), door leaves with glass, skirt line, e-axle casting, air springs, wheels with rim and hub.", "Pfosten zwischen den Scheiben (0,1 m = 0,5 mm), Türflügel mit Glas, Schwellerlinie, E-Achsgehäuse, Luftfedern, Räder mit Felge und Nabe."),
        tx("Upper clerestory panes form one band; rim and brake disc are one lighter disc (brakes region).", "Obere Scheiben bilden ein Band; Felge und Bremsscheibe sind eine hellere Scheibe (Region Bremsen)."),
        tx("Seat backs, handrails, lamps, mirror, wheel studs and hand holes, battery seams, louvres.", "Sitzlehnen, Griffstangen, Leuchten, Spiegel, Radbolzen und Handlöcher, Batterienähte, Lamellen."),
        tx("None thickened; pillars 0.1 m stay above the 0.3 mm gap limit.", "Nichts verstärkt; die Pfosten mit 0,1 m liegen über der Abstandsgrenze von 0,3 mm."),
        tx("None.", "Keine."),
        tx("Body mid-tone, roof lighter, glass lightest, tyres black, door openings dark; outlines 0.18 mm.", "Aufbau mittlerer Ton, Dach heller, Glas am hellsten, Reifen schwarz, Türöffnungen dunkel; Konturen 0,18 mm."),
        tx("Every region has its own shape; the battery and heat-pump pods carry the traction and climate heat, the doors show their own temperature when open.", "Jede Region hat eine eigene Form; Batterie- und Wärmepumpenaufbau tragen die Antriebs- und Klimawärme, die Türen zeigen offen ihre eigene Temperatur.")))

    # 100
    e100 = [dict(s) for s in e200 if s["id"] not in ("rim-f", "rim-r", "hub-f", "hub-r")]
    for s in e100:
        if s["id"] == "body-shell":
            s["d"] = SHELL.replace("1.12", "1.15")
        if s["id"].startswith("pane-") and s["id"] != "pane-cab":
            s["h"] = 1.22
        if s["id"] == "upper-band":
            s["d"] = "M 0.3,2.48 L 10.25,2.48 L 10.25,2.78 L 0.3,2.78 Z M 10.35,2.48 L 11.68,2.48 L 11.66,2.66 L 10.35,2.66 Z"
    e100 += [path("windscreen-edge", "body", "glazing", 6.1, "M 11.98,1.2 L 11.8,2.68 L 11.73,2.66 L 11.9,1.2 Z", 0.85, True, region="glazing", material="glass"),
             rect("headlamp", "body", "detail", 7, 11.9, 0.62, 0.1, 0.22, 0.03, tone=0.85, material="glass"), rect("tail-lamp", "body", "detail", 7, 0, 0.62, 0.06, 0.5, 0.02, tone=0.85, material="glass"),
             cap("mirror", "body", "detail", 7, 11.85, 2.45, 11.95, 2.15, 0.03, 0.045, tone=0.3, material="paint"),
             path("belt-line", "body", "detail", 7, "M 0.0,1.2 L 6.35,1.2 M 7.45,1.2 L 10.35,1.2 M 11.45,1.2 L 11.98,1.2", filled=False),
             path("pillar-lines", "body", "detail", 7, "M 1.5,1.2 L 1.5,2.9 M 2.75,1.2 L 2.75,2.9 M 4.15,1.2 L 4.15,2.9 M 5.4,1.2 L 5.4,2.9 M 7.5,1.2 L 7.5,2.9 M 8.7,1.2 L 8.7,2.9 M 9.9,1.2 L 9.9,2.9", filled=False)]
    e100 = [s for s in e100 if s["id"] != "skirt-line"] + [path("skirt-line", "body", "detail", 7, "M 0.05,0.62 L 2.8,0.62 M 4.1,0.62 L 8.7,0.62 M 10.0,0.62 L 11.85,0.62", filled=False)]
    hh = [s for s in old if s["id"] in ("hand-holes-f", "hand-holes-r")]
    hubs = [s for s in old if s["id"] in ("hub-f", "hub-r")]
    rims = [s for s in old if s["id"] in ("rim-f", "rim-r")]
    e100 += convert_v1_shapes(rims) + convert_v1_shapes(hh) + convert_v1_shapes(hubs)
    for s in e100:
        if s["id"].startswith("rim-"):
            s["region"] = None
            del s["region"]
    e100 += [cap("leaf-handrail-%s-%s" % (g, s), "door-%s-%s" % (g, s), "detail", 3.2, 0.02, 0.34, 0.53, 0.34, 0.02, tone=0.3, material="interior-trim") for g in "mf" for s in ("aft", "fwd")]
    R["100"] = rep(100, *RANGES[100], pens(0.18, 0.13, None, 0.1, 0.35), [], e100, decisions(
        tx("The 1:200 profile plus pillar lines, belt line, windscreen, lamps and mirror; wheels with rim, hand holes and hub.", "Das 1:200-Profil mit Pfostenlinien, Gürtellinie, Frontscheibe, Leuchten und Spiegel; Räder mit Felge, Handlöchern und Nabe."),
        tx("Door handrails on every leaf, hand holes showing the brake discs, roof pods, air springs, e-axle.", "Griffstangen an jedem Flügel, Handlöcher mit Bremsscheiben, Dachaufbauten, Luftfedern, E-Achse."),
        tx("Studs merge into the hub; battery seams and louvres are omitted.", "Bolzen gehen in der Nabe auf; Batterienähte und Lamellen entfallen."),
        tx("Seat backs, grab rails, driver cab, sidewall and wheel studs (below 0.04 m).", "Sitzlehnen, Haltestangen, Fahrerplatz, Flanke und Radbolzen (unter 0,04 m)."),
        tx("Hand holes are 0.08 m (0.8 mm) at real size; all other elements unthickened.", "Handlöcher sind mit 0,08 m (0,8 mm) in Realgröße; alle anderen Elemente nicht verstärkt."),
        tx("None.", "Keine."),
        tx("Body mid-tone, steel wheels lighter, tyres black; 0.25 / 0.18 mm pens.", "Aufbau mittlerer Ton, Stahlräder heller, Reifen schwarz; Stifte 0,25 / 0,18 mm."),
        tx("As 1:200; the hand holes carry the brakes region and show braking heat.", "Wie 1:200; die Handlöcher tragen die Region Bremsen und zeigen die Bremswärme.")))

    e50 = convert_v1_shapes(old)
    import re as _re
    scale_nums = lambda d, k: _re.sub(r"-?\d+\.\d+", lambda m: "%.3f" % (float(m.group()) * k), d)
    for s in e50:
        if s["role"] == "glazing" and s["id"] in ("side-windows", "upper-windows", "windscreen-edge"):
            s["hatch"] = "glass-sheen"
        if s["id"] in ("studs-f", "studs-r"):
            s["d"] = scale_nums(s["d"], 0.7)
        if s["id"] in ("tread-f", "tread-r"):
            s["d"] = scale_nums(s["d"], 0.96)
        if s["id"] == "mirror":
            s["x0"], s["x1"] = 11.8, 11.88
        if s["id"] == "side-windows":
            s["d"] = s["d"].replace("L 11.88,1.2 L 11.83,2.4", "L 11.8,1.2 L 11.76,2.4")
        if s["id"] == "belt-line":
            s["d"] = s["d"].replace("1.12", "1.2")
    R["50"] = rep(50, *RANGES[50], pens(0.25, 0.18, 0.1, 0.13, 0.5), [hatch("glass-sheen", "Glass sheen", "Glasglanz", "lines", 55, 1.8, 0.13)], e50, decisions(
        tx("Full side elevation of the first version with all panes, door leaves, pillars, wheels and roof pods.", "Vollständige Seitenansicht der ersten Fassung mit allen Scheiben, Türflügeln, Pfosten, Rädern und Dachaufbauten."),
        tx("Everything: seats, grab rail, driver cab, studs, louvres, battery seams, sidewall.", "Alles: Sitze, Haltestange, Fahrerplatz, Bolzen, Lamellen, Batterienähte, Flanke."),
        tx("Nothing merged.", "Nichts zusammengefasst."), tx("Interior beyond the seat backs.", "Innenraum jenseits der Sitzlehnen."),
        tx("Studs 0.016 m = 0.3 mm at the texture pen.", "Bolzen 0,016 m = 0,3 mm mit Texturstift."),
        tx("Windows carry a sheen hatch (55 degrees, 1.8 mm).", "Fenster tragen eine Glanzschraffur (55 Grad, 1,8 mm)."),
        tx("Three line weights 0.35 / 0.25 / 0.13 mm; tyres black.", "Drei Linienstärken 0,35 / 0,25 / 0,13 mm; Reifen schwarz."),
        tx("All regions at real extent.", "Alle Regionen in realer Ausdehnung.")))
    return R


NOTES = tx(
    "Battery-electric: no exhaust; waste heat leaves through the traction battery housing, the e-axle casting and the roof heat pump (plus brake discs and tyres). The cabin is modelled as conditioned to 21 C (body, roof, doors and glass are conditioned elements). Passenger loads as in the first version (Directive 2001/85/EC). Dimensions, masses and thermal parameters are authored typical values (authored-estimate).",
    "Batterieelektrisch: kein Auspuff; Abwärme verlässt das Fahrzeug über Batteriegehäuse, E-Achsgehäuse und die Dach-Wärmepumpe (dazu Bremsscheiben und Reifen). Der Innenraum ist auf 21 C konditioniert (Aufbau, Dach, Türen und Glas sind konditionierte Elemente). Fahrgastlasten wie in der ersten Fassung (Richtlinie 2001/85/EG). Abmessungen, Massen und thermische Parameter sind selbst gesetzte typische Werte (authored-estimate).")
ASSUMPTIONS = [
    tx("Heat release at level 1: traction battery housing 4.2 kW (about +14 K), e-axle 1.65 kW (+55 K), heat-pump housing 1.8 kW (+18 K), tyres 1.8 kW (about 50 km/h), friction discs 200 kW for a full stop from 50 km/h (1.2 MJ in 6 s, regeneration is not credited at level 1).",
       "Wärmefreisetzung bei Stufe 1: Batteriegehäuse 4,2 kW (etwa +14 K), E-Achse 1,65 kW (+55 K), Wärmepumpengehäuse 1,8 kW (+18 K), Reifen 1,8 kW (etwa 50 km/h), Reibscheiben 200 kW bei einer Vollbremsung aus 50 km/h (1,2 MJ in 6 s, Rekuperation wird bei Stufe 1 nicht angerechnet)."),
    tx("Roof heat-pump exhaust at level 1: 1.5 m3/s through a 0.7 m fan, 22 K warmer than ambient, dry (sensible heat rejection); flow and temperature rise scale with the hvac level.",
       "Abluft der Dach-Wärmepumpe bei Stufe 1: 1,5 m3/s durch einen 0,7-m-Ventilator, 22 K wärmer als die Umgebung, trocken (fühlbare Wärmeabgabe); Volumenstrom und Temperaturerhöhung skalieren mit der Klimastufe."),
    tx("Interior at 21 C; glazing is double (2 x 4 mm, 12 mm gap, U about 2.9 W/(m2 K)); body panels carry 40 mm lining (U about 0.9 W/(m2 K)). Sun on the visible flank half, roof pods fully.",
       "Innenraum 21 C; Verglasung doppelt (2 x 4 mm, 12 mm Spalt, U etwa 2,9 W/(m2 K)); Bleche tragen 40 mm Verkleidung (U etwa 0,9 W/(m2 K)). Sonne auf der sichtbaren Flanke zur Hälfte, Dachaufbauten voll."),
]
for r in REGIONS:
    if r["model"] == "conditioned":
        r["conditioned"] = {"interiorTempC": 21, "interiorFilmM2KW": 0.13}

SPEC = dict(sky=8, ground=28, operating=KEYS, regions=REGIONS, emitters=EMITTERS, assumptions=ASSUMPTIONS, defaults=DEFAULTS, contacts=CONTACTS, poses=POSES, fix_clips=fix_clips, reps=reps, notes=NOTES)
if __name__ == "__main__":
    a = assemble("city-bus", REL, SPEC)
    if len(sys.argv) > 1:
        previews(a, "bus", poses=sys.argv[2:] or None, dens=[int(x) for x in sys.argv[1].split(",")], zoom=(3,))
