"""🧰 Round 3 glulam beam root fixes: bearing anchors at the bearing centres, the load at the top face, and the 2D drawings
redrawn as the true views of the handcrafted body (square-sawn ends, one finger joint lay-up in elevation and plan, plates at true thickness).

Usage: python3 r3-glulam-beam-roots.py [--write]
Without --write the candidate goes to 🗑️generated/r3-glulam-beam/🔣️.json; with --write the catalog file is re-read and rewritten.

See: r3-components-glulam-beam.md, r3-glulam-beam-kit.py, round3-brief.md (ruling E1)
"""

import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/glulam-beam/🔣️.json"
OUT = os.path.join(HERE, "🗑️generated", "r3-glulam-beam", "🔣️.json")
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-glulam-beam-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)


def text(en, de):
    """🗣️ A bilingual sentence."""
    return {"en": en, "de": de}


def rect(shape, x, y, w, h):
    """▭ The shape with its geometry replaced by a rectangle, members kept in order."""
    out = {key: v for key, v in shape.items() if key not in ("kind", "points", "closed", "x", "y", "w", "h")}
    out.update({"kind": "rect", "x": x, "y": y, "w": w, "h": h})
    return out


def finger(template, id, points):
    """⚡ An open finger-joint stroke with the members of the drawing's existing finger joints."""
    out = {key: v for key, v in template.items() if key not in ("points",)}
    out["id"] = id
    out["points"] = points
    return out


def fix(doc):
    """🔧 Apply every root fix to the asset document in place."""
    a = doc["asset"]
    a["provenance"]["notes"] = text(
        "Section 160 x 600 mm, beam length 8.00 m, effective span 7.88 m between the centres of the 120 mm bearings (EN 1995-1-1); tributary width 2.0 m for the floor load.",
        "Querschnitt 160 x 600 mm, Balkenlänge 8,00 m, Stützweite 7,88 m zwischen den Mitten der 120-mm-Auflager (EN 1995-1-1); Einzugsbreite 2,0 m für die Deckenlast.",
    )
    for anchor in a["anchors"]:
        if anchor["id"] == "bearing-left":
            anchor["x"], anchor["y"] = k.BEARINGS[0], -0.32
        if anchor["id"] == "bearing-right":
            anchor["x"], anchor["y"] = k.BEARINGS[1], -0.32
        if anchor["id"] == "load-point":
            anchor["y"] = k.HALF_DEPTH
    r = a["representations"]

    g = r["50"]["geometry"]
    el = g["elevation"]
    template = next(s for s in el if s["id"].startswith("finger-joint"))
    first = next(i for i, s in enumerate(el) if s["id"].startswith("finger-joint"))
    kept = [s for s in el if not s["id"].startswith("finger-joint")]
    kept = [rect(s, 0.0, -k.HALF_DEPTH, k.LENGTH, 2 * k.HALF_DEPTH) | {"id": "beam"} if s["id"] == "beam-chamfered" else s for s in kept]
    joints = [finger(template, f"finger-joint-{n:02d}", k.zig(lam, x)) for n, (lam, x) in enumerate((lam, x) for lam in range(k.COUNT) for x in k.JOINTS[lam])]
    g["elevation"] = kept[:first] + joints + kept[first:]
    g["front"] = [rect(s, -k.HALF_WIDTH, 0.02, 2 * k.HALF_WIDTH, 2 * k.HALF_DEPTH) | {"id": "beam-end"} if s["id"] == "beam-chamfered-end" else s for s in g["front"]]
    inset = r["50"]["pens"]["detail"] * 50 / 1000 / 2
    plan = []
    for s in g["plan"]:
        if s["id"] == "beam-chamfered-top":
            plan.append(rect(s, 0.0, -k.HALF_WIDTH, k.LENGTH, 2 * k.HALF_WIDTH) | {"id": "beam-top"})
        elif s["id"].startswith("finger-joint-top-"):
            x = k.top_trace(k.JOINTS[k.COUNT - 1][int(s["id"].rsplit("-", 1)[1])])
            plan.append(s | {"points": [[x, k.r6(-k.HALF_WIDTH + inset)], [x, k.r6(k.HALF_WIDTH - inset)]]})
        else:
            plan.append(s)
    g["plan"] = plan
    d = r["50"]["decisions"]
    d["silhouette"] = text(
        "Side elevation of the 600 mm deep beam (12 mm tall and 160 mm long on paper) with its fifteen 40 mm lamellae, the 25 finger joints of the lay-up (one or two per lamella, staggered, none in the middle third of the bottom tension lamella) and the two 120 x 20 mm steel bearing plates under the square-sawn ends. Front view: the end face of 0.16 x 0.60 m (3.2 x 12 mm), fifteen lamellae separated by fourteen glue lines, over the 20 mm bearing plate. Plan view: the top lamella (160 x 3.2 mm) with its two finger joints as straight lines across the width.",
        "Seitenansicht des 600 mm hohen Balkens (12 mm hoch und 160 mm lang auf Papier) mit seinen fünfzehn Lamellen zu 40 mm, den 25 Keilzinkenstößen des Aufbaus (ein oder zwei je Lamelle, versetzt, keiner im mittleren Drittel der unteren Zuglamelle) und den beiden Stahllagerplatten 120 x 20 mm unter den rechtwinklig gesägten Enden. Vorderansicht: die Stirnfläche 0,16 x 0,60 m (3,2 x 12 mm), fünfzehn Lamellen, getrennt durch vierzehn Leimfugen, über der 20-mm-Lagerplatte. Draufsicht: die oberste Lamelle (160 x 3,2 mm) mit ihren zwei Keilzinkenstößen als gerade Linien quer über die Breite.",
    )
    d["retained"] = text(
        "All 14 glue lines at 40 mm spacing (0.8 mm) in side and end view, the two 120 x 20 mm bearing plates (2.4 x 0.4 mm), every finger joint of the lay-up at its true place; the joints of the top lamella at 2.59 m and 5.89 m appear in side view and plan at the same place.",
        "Alle 14 Leimfugen im Abstand von 40 mm (0,8 mm) in Seiten- und Stirnansicht, die beiden Lagerplatten 120 x 20 mm (2,4 x 0,4 mm), jeder Keilzinkenstoß des Aufbaus an seiner wahren Stelle; die Stöße der obersten Lamelle bei 2,59 m und 5,89 m erscheinen in Seitenansicht und Draufsicht an derselben Stelle.",
    )
    d["omitted"] = text(
        "The real finger pitch (3.8 to 6.2 mm) and the arris chamfers of a few millimetres along the long edges (0.1 mm on paper, inside the 0.35 mm outline pen, below the 7.5 mm smallest feature), so the ends are drawn square in every view. In plan the bearing plates are hidden below the beam.",
        "Die reale Zinkenteilung (3,8 bis 6,2 mm) und die Fasen von wenigen Millimetern an den Längskanten (0,1 mm auf Papier, innerhalb der 0,35-mm-Konturfeder, unter der kleinsten Einzelheit von 7,5 mm), daher sind die Enden in jeder Ansicht rechtwinklig gezeichnet. In der Draufsicht liegen die Lagerplatten verdeckt unter dem Balken.",
    )
    d["thinElements"] = text(
        "Finger joints are two-finger symbols of 20 mm finger length at 20 mm pitch (0.4 mm), because the real profile is below the printable minimum; the finger profile shows on the narrow faces of the lamellae (the beam sides), so the top face shows a straight joint line.",
        "Keilzinkenstöße sind Zeichen mit zwei Zinken von 20 mm Länge und 20 mm Teilung (0,4 mm), weil das echte Profil unter dem druckbaren Minimum liegt; das Zinkenprofil zeigt sich an den Schmalseiten der Lamellen (den Balkenseiten), daher zeigt die Oberseite eine gerade Stoßlinie.",
    )

    for h in r["100"]["hatches"]:
        if h["id"] == "grain":
            h["spacingMm"] = 0.4
    d = r["100"]["decisions"]
    d["silhouette"] = text(
        "A timber bar (80 x 6 mm) with the lamella hatch on two steel bearing plates under its ends. Front view: the end face (1.6 x 6 mm) with the lamella hatch over a bearing plate of the same width. Plan view: the top lamella (80 x 1.6 mm) with the grain hatch along the beam.",
        "Ein Holzbalken (80 x 6 mm) mit der Lamellenschraffur auf zwei Stahllagerplatten unter seinen Enden. Vorderansicht: die Stirnfläche (1,6 x 6 mm) mit der Lamellenschraffur über einer Lagerplatte gleicher Breite. Draufsicht: die oberste Lamelle (80 x 1,6 mm) mit der Faserschraffur längs des Balkens.",
    )
    d["retained"] = text(
        "Bar, lamellae and both plates (1.2 mm long). The hatch at 0.4 mm is the lamella thickness of 40 mm: glue lines in side and end view, grain lines on the top. Overall height 0.62 m (beam 0.60 m plus 0.02 m plate) in side and front view, as at every scale.",
        "Balken, Lamellen und beide Platten (1,2 mm lang). Die Schraffur mit 0,4 mm ist die Lamellendicke von 40 mm: Leimfugen in Seiten- und Stirnansicht, Faserlinien auf der Oberseite. Gesamthöhe 0,62 m (Balken 0,60 m plus 0,02 m Platte) in Seiten- und Vorderansicht, wie in jedem Maßstab.",
    )
    d["merged"] = text("The fifteen lamellae are expressed by the hatch at the lamella thickness (0.4 mm).", "Die fünfzehn Lamellen sind durch die Schraffur im Lamellenabstand (0,4 mm) ausgedrückt.")
    d["thinElements"] = text(
        "The 20 mm plates are drawn 25 mm thick (0.25 mm), the smallest filled feature that prints under the office-laser profile (0.2 mm plus margin); the extra 5 mm lies over the beam soffit, so ground line and beam height stay true; length stays 120 mm.",
        "Die 20-mm-Platten sind 25 mm dick (0,25 mm) gezeichnet, die kleinste gefüllte Einzelheit, die im Büro-Laserprofil druckt (0,2 mm plus Reserve); die zusätzlichen 5 mm liegen über der Balkenunterkante, sodass Bodenlinie und Balkenhöhe wahr bleiben; die Länge bleibt 120 mm.",
    )
    d["hatching"] = text(
        "Timber long-section hatch: parallel lines along the grain at the lamella thickness, 0.4 mm (DIN ISO 128-50, DIN 1356-1).",
        "Holz im Längsschnitt: parallele Linien in Faserrichtung im Lamellenabstand, 0,4 mm (DIN ISO 128-50, DIN 1356-1).",
    )

    g = r["200"]["geometry"]
    ends = {"bearing-left": 0.0, "bearing-right": k.LENGTH - k.PLATE[0]}
    g["elevation"] = [rect(s, ends[s["id"]], -0.32, k.PLATE[0], k.PLATE[1]) | {"filled": True} if s["id"] in ends else s for s in g["elevation"]]
    g["front"] = [rect(s, -k.HALF_WIDTH, 0.0, 2 * k.HALF_WIDTH, k.PLATE[1]) | {"filled": True} if s["id"] == "bearing-end" else s for s in g["front"]]
    d = r["200"]["decisions"]
    d["silhouette"] = text(
        "A single timber bar (40 x 3 mm) on two bearing plates under its ends. Front view: the 0.16 x 0.60 m end face (0.8 x 3 mm) over its bearing plate. Plan view: the top of the bar (40 x 0.8 mm) with a faint axis line between the bearings.",
        "Ein einzelner Holzbalken (40 x 3 mm) auf zwei Lagerplatten unter seinen Enden. Vorderansicht: die Stirnfläche 0,16 x 0,60 m (0,8 x 3 mm) über ihrer Lagerplatte. Draufsicht: die Oberseite des Balkens (40 x 0,8 mm) mit einer feinen Achslinie zwischen den Auflagern.",
    )
    d["retained"] = text(
        "Span, depth 0.60 m and the two bearings of 120 x 20 mm at their true place between ground and soffit. Overall height 0.62 m (beam 0.60 m plus 0.02 m plate) in side and front view, as at every scale.",
        "Spannweite, Höhe 0,60 m und die beiden Auflager 120 x 20 mm an ihrer wahren Stelle zwischen Boden und Balkenunterkante. Gesamthöhe 0,62 m (Balken 0,60 m plus 0,02 m Platte) in Seiten- und Vorderansicht, wie in jedem Maßstab.",
    )
    d["thinElements"] = text(
        "The 20 mm bearing plates are filled at their true 0.1 mm thickness; the 0.18 mm outline pen closes them into one dark bearing band of about 0.28 mm that sits between ground line and soffit.",
        "Die 20-mm-Lagerplatten sind in ihrer wahren Dicke von 0,1 mm gefüllt; die 0,18-mm-Konturfeder schließt sie zu einem dunklen Auflagerband von etwa 0,28 mm zwischen Bodenlinie und Balkenunterkante.",
    )
    d["overlays"] = text("Overlays colour the bar (merged faces) and the two bearing plates separately.", "Überlagerungen färben Balken (zusammengefasste Flächen) und die beiden Lagerplatten getrennt.")
    return doc


def main():
    """🚀 Build the candidate, or rewrite the catalog file with --write."""
    with open(CATALOG, encoding="utf-8") as f:
        doc = json.load(f)
    fix(doc)
    path = CATALOG if "--write" in sys.argv else OUT
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=2)
        f.write("\n")
    print(path)


if __name__ == "__main__":
    main()
