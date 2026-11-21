"""🌀 Round 5, machinery/scroll-compressor: re-authors the model-only 1:500 (smooth torispherical cap instead of a two-slope cone, a base plate with chamfered ears instead of a square slab, no thermal parts standing out of the body); 1:50 to 1:200 are kept.

Writes the volumes source `volumes/machinery/scroll-compressor.json`; splice it with `volumes-insert.py`.
Usage: python3 r5-scroll-compressor-model.py
See: r3-machinery-scroll-compressor.md (Copeland ZR K5 reference table), r5-brief.md.
"""
import copy
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "machinery", "scroll-compressor.json")


def by_id(items, sid):
    """🔎 The item with this id."""
    return next(x for x in items if x["id"] == sid)


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


CAP500 = [[0.0, 0.0], [0.0, 0.1245], [0.018, 0.1245], [0.032, 0.1205], [0.044, 0.1115], [0.055, 0.096], [0.064, 0.072], [0.071, 0.04], [0.0745, 0.0]]
FEET500 = [[0.085, 0.115], [-0.085, 0.115], [-0.115, 0.085], [-0.115, -0.085], [-0.085, -0.115], [0.085, -0.115], [0.115, -0.085], [0.115, 0.085]]

DECISIONS500 = [
    text("Body, 1:500 (model only, 75 mm feature; the drawing is unavailable at this scale): the hermetic shell of 241 mm on a 30 mm base plate whose deep chamfers stand for the four ears and grommets on the 190.5 mm bolt square, the torispherical top cap as one smooth crown of 75 mm rise, the 1/2 in discharge and 7/8 in suction tubes in their directions (kept at true size, as the stubs are what makes the compressor) and the terminal cover; the window and the scroll behind it are below the feature and left out of the body.",
         "Körper, 1:500 (nur Modell, 75-mm-Merkmal; die Zeichnung ist bei diesem Maßstab nicht verfügbar): der hermetische Mantel von 241 mm auf einer 30-mm-Grundplatte, deren tiefe Fasen für die vier Ohren mit Tüllen auf dem 190,5-mm-Lochquadrat stehen, die torisphärische Kappe als eine glatte Wölbung von 75 mm Stich, Druckrohr 1/2 Zoll und Saugrohr 7/8 Zoll in ihren Richtungen (in echter Größe, weil die Stutzen den Verdichter ausmachen) und die Klemmenabdeckung; Fenster und Spirale dahinter liegen unter dem Merkmal und fehlen im Körper."),
    text("Thermal: references per region and the plume patch on the crown; the scroll region is a 90 mm disc at the height of the scroll set inside the shell, where the 1:50 window shows it.",
         "Thermik: Verweise je Region und das Fahnenfeld auf der Wölbung; die Region Spirale ist eine 90-mm-Scheibe auf der Höhe des Spiralsatzes im Mantel, wo das Fenster bei 1:50 sie zeigt."),
    text("Structure: the pressure vessel up to the lap, the foot frame, the main bearing frame, the cap shell, the terminal fence, the orbiting scroll, the motor mass and one contact pad over the grommet square.",
         "Tragwerk: der Druckbehälter bis zur Überlappung, der Fußrahmen, das Hauptlagerschild, die Kappenschale, der Klemmenzaun, die orbitierende Spirale, die Motormasse und ein Kontaktpolster über das Tüllenquadrat."),
]


def main():
    doc = json.load(open(SOURCE, encoding="utf-8"))
    v = doc["volumes"]["500"]
    body = v["body"]
    by_id(body, "top-cap")["profile"] = CAP500
    feet = by_id(body, "feet")
    for k in ("size", "transform"):
        feet.pop(k, None)
    feet.update({"kind": "extrude", "profile": FEET500, "plane": "zx", "from": 0, "to": 0.0295})
    v["thermal"] = [t for t in v["thermal"] if t["id"] != "th-window"]
    scroll = by_id(v["thermal"], "th-scroll-window")
    scroll["profile"] = [[0.06, 0.0], [0.06, 0.045], [0.115, 0.045], [0.115, 0.0]]
    v["decisions"] = DECISIONS500
    json.dump(doc, open(SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("source", SOURCE)


if __name__ == "__main__":
    main()
