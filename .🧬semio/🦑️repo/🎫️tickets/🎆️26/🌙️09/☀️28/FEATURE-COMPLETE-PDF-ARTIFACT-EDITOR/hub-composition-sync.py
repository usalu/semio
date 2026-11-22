"""🔁 Idempotently adds the eight PDF authoring actions to the hub composition JSON, mirroring `window_definition()`; rerun the descriptor generator to replace this hand sync."""
import copy
import json
from pathlib import Path

root = next(parent for parent in Path(__file__).resolve().parents if (parent / "Cargo.toml").exists())
path = root / "🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧩️component/🔣️.json"
document = json.loads(path.read_text())
label = lambda en, de: {"native": {"en": en, "de": de}, "reuse": {"en": en, "de": de}}
number = lambda id, en, de, required: {"id": id, "label": label(en, de), "schema": {"kind": "number", "integer": False}, "required": required}
string = lambda id, en, de, minimum, required=False: {"id": id, "label": label(en, de), "schema": {"kind": "string", "minLen": minimum}, "required": required}
page, x, y = number("page", "Page", "Seite", True), number("x", "X", "X", True), number("y", "Y", "Y", True)
extra = lambda en, de: string("extra", en, de, 0)
payload = lambda minimum: string("payload", "PDF Data", "PDF-Daten", minimum, minimum == 1)
specs = [
    ("set-entry", "Set Entry", "Eintrag setzen", [string("object", "Family (catalog, trailer, info, viewer)", "Familie (catalog, trailer, info, viewer)", 1, True), string("text", "Value (key=value to add)", "Wert (key=value zum Hinzufügen)", 0), string("extra", "Key", "Schlüssel", 0)]),
    ("create-annotation", "Create Annotation", "Anmerkung erstellen", [page, string("object", "Kind (note, highlight, underline, strikeOut, squiggly, ink, link)", "Art (note, highlight, underline, strikeOut, squiggly, ink, link)", 1, True), string("text", "Contents or link", "Inhalt oder Link", 0), extra("Quad points or ink strokes", "Quadpunkte oder Freihandstriche"), x, y, number("width", "Width", "Breite", False), number("height", "Height", "Höhe", False), number("red", "Red", "Rot", False), number("green", "Green", "Grün", False), number("blue", "Blue", "Blau", False)]),
    ("fill-form-field", "Fill Form Field", "Formularfeld ausfüllen", [string("object", "Field name", "Feldname", 1, True), string("text", "Value", "Wert", 0)]),
    ("replace-page", "Replace Page", "Seite ersetzen", [page, payload(0), extra("Source Page", "Quellseite"), number("width", "Width", "Breite", False), number("height", "Height", "Höhe", False)]),
    ("replace-page-pick", "Replace Page from PDF", "Seite aus PDF ersetzen", [page, extra("Source Page", "Quellseite")]),
    ("merge-document", "Merge PDF Pages", "PDF-Seiten zusammenführen", [page, payload(1), extra("Source Pages", "Quellseiten")]),
    ("merge-pick", "Insert Pages from PDF", "Seiten aus PDF einfügen", [page, extra("Source Pages", "Quellseiten")]),
    ("extract-pages", "Extract Pages to PDF", "Seiten als PDF extrahieren", [string("extra", "Pages (1-3,5; next file after ;)", "Seiten (1-3,5; weitere Datei nach ;)", 1, True)]),
]
added = 0
for app in document["manifest"]["apps"]:
    for window in app.get("windowKinds", []):
        actions = window.get("actions", [])
        ids = [action["id"] for action in actions]
        if "set-mesh-reference" not in ids:
            continue
        template = copy.deepcopy(actions[ids.index("set-page-rotation")])
        at = ids.index("set-mesh-reference") + 1
        for id, en, de, args in specs:
            if id in ids:
                continue
            entry = copy.deepcopy(template)
            entry.update(id=id, label=label(en, de), args=copy.deepcopy(args))
            actions.insert(at, entry)
            at += 1
            added += 1
path.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
print(f"added {added} action entries")
