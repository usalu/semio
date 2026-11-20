"""🧷 Round 3, appliances/room-thermostat: the root corrections outside `volumes` (rig, anchors, 2D shapes of both scales, texts, dial area) together with the new `volumes` in one write, so the catalog asset is valid before and after.

Usage: python3 r3-room-thermostat-roots.py            writes the candidate to 🗑️generated/r3-room-thermostat/🔣️.json
       python3 r3-room-thermostat-roots.py --write    re-reads the catalog asset and writes it (then run volumes-insert.py for the house style of `volumes`)
"""
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_room_thermostat_kit", os.path.join(HERE, "r3-room-thermostat-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

ASSET = os.path.normpath(os.path.join(HERE, "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🔌️appliances", "room-thermostat", "🔣️.json"))
SOURCE = os.path.join(HERE, "volumes", "appliances", "room-thermostat.json")
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-room-thermostat", "🔣️.json")

DIAL_AREA_M2 = 0.0028

REFERENCES = [
    {"id": "din-49073-1-2010", "citation": "DIN 49073-1:2010 Flush-mounted device boxes of metal or insulating material for accessories up to 16 A, 250 V (60 mm box diameter, device fixing screws 60 mm apart)."},
    {"id": "danfoss-ret1000-datasheet", "citation": "Danfoss RET1000 electronic room thermostat with dial, data sheet (84 x 84 mm face, 28 to 37 mm deep with dial), https://assets.danfoss.com/documents/79466/AI172186403796en-020201.pdf, accessed 2026-11-20."},
    {"id": "eberle-rtr-e-6121", "citation": "Eberle Controls RTR-E 6121 room temperature controller with set-point dial, product data (75 x 75 x 25.5 mm, wall or flush-box mounting, 5 to 30 degrees Celsius), https://www.eberle.de/fileadmin/Downloads/PDF/Temperaturregler_2025.pdf, accessed 2026-11-20."},
]

NOTES_SWAPS = {
    "en": [
        ("A battery-free electronic room thermostat for flush wall mounting at 1.5 m height seen face-on:", "A battery-free electronic room thermostat screwed onto a flush wall box at 1.5 m height, seen face-on (size class of the Danfoss RET1000 and the Eberle RTR-E 6121):"),
        ("a rounded square housing of 0.09 m by 0.09 m and 27 mm depth with", "a rounded square housing of 0.09 m by 0.09 m, 15 mm deep, on a base plate held by two screws 60 mm apart (DIN 49073-1), with"),
        ("and a 42 mm set-point dial turning over 270 degrees for 5 to 30 degrees Celsius.", "and a 42 mm set-point dial that stands 12 mm proud (27 mm overall depth) and turns over 270 degrees for 5 to 30 degrees Celsius."),
    ],
    "de": [
        ("Ein batterieloser elektronischer Raumthermostat zur Unterputzmontage in 1,5 m Höhe, frontal gesehen:", "Ein batterieloser elektronischer Raumthermostat, in 1,5 m Höhe auf eine Unterputzdose geschraubt, frontal gesehen (Größenklasse des Danfoss RET1000 und des Eberle RTR-E 6121):"),
        ("ein gerundetes quadratisches Gehäuse von 0,09 m mal 0,09 m und 27 mm Tiefe mit", "ein gerundetes quadratisches Gehäuse von 0,09 m mal 0,09 m und 15 mm Tiefe auf einer Grundplatte, die zwei Schrauben im Abstand von 60 mm halten (DIN 49073-1), mit"),
        ("und einem Sollwert-Drehknopf von 42 mm, der sich über 270 Grad für 5 bis 30 Grad Celsius dreht.", "und einem Sollwert-Drehknopf von 42 mm, der 12 mm vorsteht (27 mm Gesamttiefe) und sich über 270 Grad für 5 bis 30 Grad Celsius dreht."),
    ],
}

BONES = [
    {
        "id": "housing", "parent": None, "x": 0, "y": -K.CENTRE_U, "angleDeg": 90,
        "title": {"en": "Housing on its base plate, from the lower to the upper fixing screw", "de": "Gehäuse auf seiner Grundplatte, von der unteren zur oberen Befestigungsschraube"},
        "lengthM": K.SCREW_PITCH, "joint": {"type": "fixed"},
        "mass": {"kg": 0.11, "comX": K.CENTRE_U, "comY": 0, "distribution": "point", "source": "authored-estimate"},
    },
    {
        "id": "dial", "parent": "housing", "x": K.DIAL_U, "y": 0, "angleDeg": 0,
        "title": {"en": "Set-point dial, from the spindle axis along the index", "de": "Sollwert-Drehknopf, von der Achse entlang des Zeigers"},
        "lengthM": K.DIAL_R, "joint": {"type": "revolute", "minDeg": -135, "maxDeg": 135},
        "mass": {"kg": 0.025, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate"},
    },
]

ANCHOR_AT = {"wall-screw-top": K.SCREW_PITCH, "wall-screw-bottom": 0}


def housing_rect(shape, x0, y0, w, h):
    """↪ An old face-on rectangle (x to the right, y up from the housing centre) in the new housing frame (u up from the lower screw, v to the left)."""
    shape.update({"x": K.r6(y0 + K.CENTRE_U), "y": K.r6(-(x0 + w)), "w": h, "h": w})


def shape(ident, bone, role, z, tone, geometry, **binding):
    return {"id": ident, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone, **geometry, **binding}


def dial_solid():
    return {"kind": "extruded", "sweep": "w", "z0": K.W_INDEX, "z1": K.W_FACE}


def geometry_50(geometry):
    by = {s["id"]: s for s in geometry["elevation"]}
    housing_rect(by["housing"], -0.045, -0.045, 0.09, 0.09)
    housing_rect(by["display"], -0.018, 0.016, 0.036, 0.021)
    by["led"].update({"cx": K.LED_AT[0], "cy": K.LED_AT[1]})
    by["pointer"].update({"x": 0.004, "y": -0.0055, "w": 0.0145, "h": 0.011})
    for view in ("front", "plan"):
        for s in geometry[view]:
            if s["id"] == "housing":
                s["r"] = 0.002


def geometry_100():
    casing = {"regions": ["casing", "led"], "material": "abs-plastic"}
    dial = shape("dial", "dial", "body", 3.2, 0.45, {"kind": "ellipse", "cx": 0, "cy": 0, "rx": K.DIAL_R, "ry": K.DIAL_R}, region="dial", material="abs-plastic")
    return {
        "elevation": [
            shape("housing", "housing", "body", 3, 0.9, {"kind": "rect", "x": -0.015, "y": -0.045, "w": 0.09, "h": 0.09, "r": 0.004}, **casing),
            shape("display", "housing", "glazing", 3.1, 0.12, {"kind": "rect", "x": 0.046, "y": -0.018, "w": 0.021, "h": 0.036}, region="display", material="lcd-glass"),
            dict(dial),
        ],
        "plan": [
            {**dial, "solid": dial_solid()},
            shape("housing", "housing", "body", 3, 0.9, {"kind": "rect", "x": -0.045, "y": -0.0015, "w": 0.09, "h": 0.015}, **casing),
        ],
        "front": [
            shape("housing", "housing", "body", 3, 0.9, {"kind": "rect", "x": -0.0015, "y": -0.045, "w": 0.015, "h": 0.09}, **casing),
            {**dial, "solid": dial_solid()},
        ],
    }


DECISIONS_50 = {
    "silhouette": {
        "en": "A 90 mm square wall thermostat seen face-on, 1.8 mm on paper: a rounded housing with a dark display window, a small lamp and a round set-point dial with a pointer. Front view: the 0.09 m high housing, 0.015 m deep, with the dial standing 0.012 m proud of its right-hand face; the wall side is on the left. Top view: the same housing 0.09 m wide with the dial 0.042 m wide.",
        "de": "Ein quadratischer Raumthermostat von 90 mm in der Frontansicht, auf Papier 1,8 mm: ein Gehäuse mit gerundeten Ecken, ein dunkles Anzeigefenster, eine kleine Lampe und ein runder Sollwert-Drehknopf mit Zeiger. Vorderansicht: das 0,09 m hohe Gehäuse von 0,015 m Tiefe mit dem Drehknopf, der 0,012 m aus der rechten Fläche ragt; die Wandseite liegt links. Draufsicht: dasselbe Gehäuse, 0,09 m breit, mit dem 0,042 m breiten Drehknopf.",
    },
    "omitted": {
        "en": "The scale ticks, the digits on the display, the screws, the ventilation slots, the finger flutes of the knob and the 1.5 mm shadow gap above the base plate. Front and top view omit the display, the LED and the pointer, which lie on the right-hand face and are seen edge-on there; the 2 mm face-edge radius stands for the whole edge profile.",
        "de": "Die Skalenstriche, die Ziffern der Anzeige, die Schrauben, die Lüftungsschlitze, die Griffmulden des Knopfs und die 1,5 mm breite Schattenfuge über der Grundplatte. Vorder- und Draufsicht lassen Anzeige, Leuchtdiode und Zeiger weg, die auf der rechten Fläche liegen und dort nur von der Kante zu sehen wären; der Kantenradius von 2 mm an der Front steht für das ganze Kantenprofil.",
    },
}

DECISIONS_100 = {
    "silhouette": {
        "en": "A 0.9 mm square with a dark display plate and a dark round dial, all at their true size. Front view: the housing 0.015 m deep with the dial 0.012 m proud of it. Top view: the housing 0.09 m wide with the 0.042 m dial in front of it.",
        "de": "Ein Quadrat von 0,9 mm mit einer dunklen Anzeigeplatte und einem dunklen runden Drehknopf, alle in wahrer Größe. Vorderansicht: das Gehäuse von 0,015 m Tiefe mit dem Drehknopf, der 0,012 m vorsteht. Draufsicht: das 0,09 m breite Gehäuse mit dem 0,042 m breiten Drehknopf davor.",
    },
    "retained": {
        "en": "Housing, the display window of 36 by 21 mm and the round dial that carries the set point.",
        "de": "Gehäuse, das Anzeigefenster von 36 mal 21 mm und der runde Drehknopf, der den Sollwert trägt.",
    },
    "merged": {
        "en": "The lamp joins the housing (it carries its region); the pointer joins the dial; the 8 mm corner radius is reduced to 4 mm.",
        "de": "Die Lampe geht im Gehäuse auf (es trägt ihre Region); der Zeiger geht im Drehknopf auf; der Eckradius von 8 mm ist auf 4 mm verkleinert.",
    },
    "omitted": {
        "en": "Lamp and pointer: the turning of the dial is not drawn at this scale. Front and top view show housing and dial only, as plain rectangles at the true depths of 0.015 m and 0.012 m, so the overall depth is the 0.027 m of the dimensions.",
        "de": "Lampe und Zeiger: das Drehen des Knopfs wird bei diesem Maßstab nicht gezeichnet. Vorder- und Draufsicht zeigen nur Gehäuse und Drehknopf als einfache Rechtecke in den wahren Tiefen von 0,015 m und 0,012 m, die Gesamttiefe sind also die 0,027 m der Abmessungen.",
    },
    "thinElements": {
        "en": "The dial is 42 mm wide (0.42 mm on paper) and prints as a filled disc; the display window is 0.36 by 0.21 mm on paper. Housing and dial are 0.15 mm and 0.12 mm deep on paper in front and top view; they are kept at the true depth so that the proportions of the object stay right.",
        "de": "Der Drehknopf ist 42 mm breit (0,42 mm auf Papier) und druckt als gefüllte Scheibe; das Anzeigefenster misst auf Papier 0,36 mal 0,21 mm. Gehäuse und Drehknopf sind in Vorder- und Draufsicht auf Papier 0,15 mm und 0,12 mm tief; sie bleiben in wahrer Tiefe, damit die Proportionen des Objekts stimmen.",
    },
    "hatching": {"en": "None; the display is a dark tone.", "de": "Keine; die Anzeige ist ein dunkler Ton."},
    "styles": {
        "en": "Housing 0.9, dial 0.45, display 0.12; body pen 0.18 mm, glazing pen 0.13 mm.",
        "de": "Gehäuse 0,9, Drehknopf 0,45, Anzeige 0,12; Körperstift 0,18 mm, Verglasungsstift 0,13 mm.",
    },
    "overlays": {
        "en": "The housing carries casing and lamp; display and dial keep their regions.",
        "de": "Das Gehäuse trägt Gehäuse und Lampe; Anzeige und Drehknopf behalten ihre Regionen.",
    },
}

UNAVAILABLE_200 = {
    "reason": {
        "en": "At 1:200 the 90 mm thermostat is 0.45 x 0.45 mm on paper, under the 3 mm an asset needs to be drawn. The dial (42 mm = 0.21 mm) just reaches the 0.2 mm feature width of the office laser profile, but the lamp (7 mm = 0.035 mm), the pointer (11 mm = 0.055 mm) and the 9 mm between display and dial (0.045 mm) are below it, so the face would print as one dark blot; use the 1:100 or 1:50 drawing at this scale.",
        "de": "Bei 1:200 ist der 90 mm große Thermostat 0,45 x 0,45 mm auf dem Papier, unter den 3 mm, die ein Objekt zum Zeichnen braucht. Der Drehknopf (42 mm = 0,21 mm) erreicht die Merkmalsbreite von 0,2 mm des Büro-Laser-Profils gerade, aber die Lampe (7 mm = 0,035 mm), der Zeiger (11 mm = 0,055 mm) und die 9 mm zwischen Anzeige und Drehknopf (0,045 mm) liegen darunter, das Gesicht druckte also als ein dunkler Fleck; bei diesem Maßstab dient die Zeichnung 1:100 oder 1:50.",
    },
    "limitingFeature": {"en": "Heating-call lamp (7 mm = 0.035 mm)", "de": "Lampe für die Heizanforderung (7 mm = 0,035 mm)"},
    "limitingSizeM": 0.007,
}

STRUCTURE_ASSUMPTION = {
    "en": "The 0.135 kg thermostat hangs on its base plate, which two screws 60 mm apart hold on the flush wall box: the upper screw is a pin, the lower one sits in a vertical slot and acts as a roller; the pair carries the weight and the small overturning moment of the 27 mm deep device.",
    "de": "Der 0,135 kg schwere Thermostat hängt an seiner Grundplatte, die zwei Schrauben im Abstand von 60 mm an der Unterputzdose halten: die obere Schraube ist ein Festlager, die untere sitzt in einem senkrechten Langloch und wirkt als Rollenlager; das Paar trägt das Gewicht und das kleine Kippmoment des 27 mm tiefen Geräts.",
}

ANCHOR_TITLES = {
    "wall-screw-top": {"en": "Upper fixing screw in the wall box", "de": "Obere Befestigungsschraube in der Unterputzdose"},
    "wall-screw-bottom": {"en": "Lower fixing screw in its slot", "de": "Untere Befestigungsschraube im Langloch"},
}


def corrected(document, volumes):
    asset = document["asset"]
    known = {r["id"] for r in asset["provenance"]["references"]}
    asset["provenance"]["references"] += [r for r in REFERENCES if r["id"] not in known]
    for language, swaps in NOTES_SWAPS.items():
        for old, new in swaps:
            text = asset["provenance"]["notes"][language]
            if new in text:
                continue
            if old not in text:
                raise SystemExit(f"notes.{language}: sentence not found: {old[:40]}")
            asset["provenance"]["notes"][language] = text.replace(old, new)
    asset["rig"]["bones"] = BONES
    for anchor in asset["anchors"]:
        ordered = {}
        for key, value in anchor.items():
            if key == "z":
                continue
            ordered[key] = value
            if key == "y":
                ordered["z"] = K.W_WALL
        anchor.clear()
        anchor.update(ordered)
        anchor.update({"x": ANCHOR_AT[anchor["id"]], "y": 0, "title": ANCHOR_TITLES[anchor["id"]]})
    fifty = asset["representations"]["50"]
    if fifty["geometry"]["elevation"][0]["x"] == -0.045:
        geometry_50(fifty["geometry"])
    fifty["decisions"].update(DECISIONS_50)
    hundred = asset["representations"]["100"]
    hundred["pens"]["glazing"] = 0.13
    hundred["geometry"] = geometry_100()
    hundred["decisions"] = DECISIONS_100
    asset["representations"]["200"].update(UNAVAILABLE_200)
    for region in asset["thermal"]["regions"]:
        if region["id"] == "dial":
            region["areaM2"] = DIAL_AREA_M2
    asset["structure"]["assumptions"] = [STRUCTURE_ASSUMPTION]
    asset["volumes"] = volumes
    return document


def main():
    with open(SOURCE, encoding="utf-8") as handle:
        volumes = json.load(handle)["volumes"]
    with open(ASSET, encoding="utf-8") as handle:
        document = corrected(json.load(handle), volumes)
    target = ASSET if "--write" in sys.argv else CANDIDATE
    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    print("wrote", target)


if __name__ == "__main__":
    main()
