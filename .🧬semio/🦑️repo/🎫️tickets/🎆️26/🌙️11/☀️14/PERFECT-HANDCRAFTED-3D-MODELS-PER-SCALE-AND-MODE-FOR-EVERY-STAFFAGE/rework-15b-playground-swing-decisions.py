"""📝 Playground swing: decisions of the spread suspension and the closed load path, per scale (en first, de second).

Usage: python3 rework-15b-playground-swing-decisions.py   (edits the source once)
"""
import json
import os

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes/objects/playground-swing.json")

BODY = {
    "en": " The hanger pins sit 0.50 m apart and the seat eyes 0.40 m, so each chain leans inward by 1.5 degrees (EN 1176-2 lateral stability: suspension spacing at least the seat fixing spacing plus 5 % of the 1.9 m chain length = 0.495 m).",
    "de": " Die Gelenkbolzen sitzen 0,50 m auseinander, die Sitzösen 0,40 m, jede Kette neigt sich daher um 1,5 Grad nach innen (Seitenstabilität nach EN 1176-2: Abstand der Aufhängepunkte mindestens Abstand der Sitzbefestigungen plus 5 % der Kettenlänge von 1,9 m = 0,495 m).",
}
STRUCTURE = {
    "50": {
        "en": " The load path is closed solid by solid: seat insert, M10 eye bolts, chain members from seat eye to hanger pin, hanger fork and clamp band, beam, apex sleeves with their welded leg sockets and gussets, legs, base plates, supports.",
        "de": " Der Lastpfad ist Körper für Körper geschlossen: Sitzeinlage, Ösenschrauben M10, Kettenstäbe von der Sitzöse zum Gelenkbolzen, Gabel und Klemmband des Gelenks, Holm, Firstmuffen mit ihren angeschweißten Beinmuffen und Knotenblechen, Beine, Fußplatten, Lager.",
    },
    "100": {
        "en": " The load path is closed: seat insert, eye bolts drawn 15 mm thick, hitch nodes, chain members up to the pivot nodes, hanger blocks, beam, apex nodes, legs, base plates as sharp-edged foot joints, supports.",
        "de": " Der Lastpfad ist geschlossen: Sitzeinlage, 15 mm stark gezeichnete Ösenschrauben, Anschlussknoten, Kettenstäbe bis zu den Gelenkknoten, Gelenkklötze, Holm, Firstknoten, Beine, Fußplatten als scharfkantige Fußknoten, Lager.",
    },
    "200": {
        "en": " The load path is closed: a hitch node of 35 mm radius joins each chain member to the seat member, the foot blocks join legs and supports.",
        "de": " Der Lastpfad ist geschlossen: ein Anschlussknoten von 35 mm Radius verbindet jeden Kettenstab mit dem Sitzstab, die Fußklötze verbinden Beine und Lager.",
    },
    "500": {
        "en": " The load path is closed: a hitch node of 40 mm radius joins each chain member to the seat member, the foot blocks join legs and supports.",
        "de": " Der Lastpfad ist geschlossen: ein Anschlussknoten von 40 mm Radius verbindet jeden Kettenstab mit dem Sitzstab, die Fußklötze verbinden Beine und Lager.",
    },
}

with open(SOURCE, encoding="utf-8") as handle:
    source = json.load(handle)
for scale, lists in source["volumes"].items():
    body, structure = lists["decisions"][0], lists["decisions"][2]
    if "EN 1176-2 lateral stability" in body["en"]:
        raise SystemExit("already applied")
    for language in ("en", "de"):
        body[language] += BODY[language]
        structure[language] += STRUCTURE[scale][language]
with open(SOURCE, "w", encoding="utf-8") as handle:
    json.dump(source, handle, ensure_ascii=False, indent=1)
