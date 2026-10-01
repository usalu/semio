#!/usr/bin/env python3
"""🎛️ Writes the payload schema of the forms `change-block-field` leaf (design §17.1 of ticket
26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): a root union discriminated by `field`, one member per settable question
field with its typed `value`, hard bounds in standard keywords and full `x-semio-ui` (labels en/de, widget, group,
order). Run from the repository root; rewrites the schema file in place."""

import json
import pathlib

LEAF = pathlib.Path("✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-block-field/🧬️schema/🔣️.json")
DEFINITION = "https://json.schemas.assets.semio-tech.com/s/forms/forms/definition.json"


def text(nullable):
    return {"type": ["string", "null"]} if nullable else {"type": "string"}


def number(**bounds):
    return {"type": ["number", "null"], **bounds}


OPTION = {
    "type": "object",
    "additionalProperties": False,
    "required": ["value", "label"],
    "properties": {
        "value": {"type": "string", "minLength": 1, "x-semio-ui": {"widget": "text", "role": "value", "label": {"en": "Value", "de": "Wert"}, "description": {"en": "Stored answer of this option, unique in the question.", "de": "Gespeicherte Antwort dieser Option, eindeutig in der Frage."}, "group": "choices", "order": 10}},
        "label": {"type": "string", "x-semio-ui": {"widget": "text", "role": "value", "label": {"en": "Label", "de": "Beschriftung"}, "group": "choices", "order": 20}},
    },
}

VECTOR_FIELD = {
    "type": "object",
    "additionalProperties": False,
    "required": ["key"],
    "properties": {
        "key": {"type": "string", "minLength": 1, "x-semio-ui": {"widget": "text", "role": "value", "label": {"en": "Key", "de": "Schlüssel"}, "description": {"en": "Component key, unique in the vector.", "de": "Komponentenschlüssel, eindeutig im Vektor."}, "group": "choices", "order": 10}},
        "label": {"type": "string", "x-semio-ui": {"widget": "text", "role": "value", "label": {"en": "Label", "de": "Beschriftung"}, "group": "choices", "order": 20}},
        "value": {"type": "number", "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": "Value", "de": "Wert"}, "step": 1, "precision": 2, "group": "choices", "order": 30}},
    },
}

FIELDS = [
    ("label", {"en": "Label", "de": "Beschriftung"}, {"en": "Question text shown to the respondent.", "de": "Fragetext, den die befragte Person sieht."}, {**text(False), "x-ui": {"widget": "text", "group": "content"}}),
    ("description", {"en": "Description", "de": "Beschreibung"}, {"en": "Help text below the question; empty removes it.", "de": "Hilfetext unter der Frage; leer entfernt ihn."}, {**text(True), "x-ui": {"widget": "multiline", "group": "content"}}),
    ("placeholder", {"en": "Placeholder", "de": "Platzhalter"}, {"en": "Hint shown in an empty answer field.", "de": "Hinweis in einem leeren Antwortfeld."}, {**text(True), "x-ui": {"widget": "text", "group": "content"}}),
    ("text", {"en": "Text", "de": "Text"}, {"en": "Static text of an information block.", "de": "Fester Text eines Informationsblocks."}, {**text(True), "x-ui": {"widget": "multiline", "group": "content"}}),
    ("unit", {"en": "Unit", "de": "Einheit"}, {"en": "Unit shown next to a number answer.", "de": "Einheit neben einer Zahlenantwort."}, {**text(True), "x-ui": {"widget": "text", "group": "content"}}),
    ("schema", {"en": "Schema", "de": "Schema"}, {"en": "Schema id the answer follows.", "de": "Schema-ID, der die Antwort folgt."}, {**text(True), "x-ui": {"widget": "text", "group": "source"}}),
    ("src", {"en": "Source", "de": "Quelle"}, {"en": "Address of the file, e.g. an https or data URL.", "de": "Adresse der Datei, z. B. eine https- oder data-URL."}, {**text(True), "x-ui": {"widget": "text", "group": "source"}}),
    ("accept", {"en": "Accepted files", "de": "Akzeptierte Dateien"}, {"en": "File types an upload accepts, e.g. image/*.", "de": "Dateitypen, die ein Upload annimmt, z. B. image/*."}, {**text(True), "x-ui": {"widget": "text", "group": "source"}}),
    ("fixtureSlug", {"en": "Fixture slug", "de": "Fixture-Kürzel"}, {"en": "Slug of the bundled fixture the question draws its data from.", "de": "Kürzel der mitgelieferten Fixture, aus der die Frage ihre Daten bezieht."}, {**text(True), "x-ui": {"widget": "text", "group": "source"}}),
    ("required", {"en": "Required", "de": "Pflichtfeld"}, {"en": "The form cannot be submitted without an answer.", "de": "Das Formular kann ohne Antwort nicht übermittelt werden."}, {"type": ["boolean", "null"], "x-ui": {"widget": "toggle", "group": "validation"}}),
    ("min", {"en": "Minimum", "de": "Minimum"}, {"en": "Smallest accepted number; empty removes the bound.", "de": "Kleinste zulässige Zahl; leer entfernt die Grenze."}, {**number(), "x-ui": {"widget": "stepper", "step": 1, "precision": 2, "group": "validation"}}),
    ("max", {"en": "Maximum", "de": "Maximum"}, {"en": "Largest accepted number; empty removes the bound.", "de": "Größte zulässige Zahl; leer entfernt die Grenze."}, {**number(), "x-ui": {"widget": "stepper", "step": 1, "precision": 2, "group": "validation"}}),
    ("step", {"en": "Step", "de": "Schrittweite"}, {"en": "Increment of a number answer, greater than zero.", "de": "Schrittweite einer Zahlenantwort, größer als null."}, {**number(exclusiveMinimum=0), "x-ui": {"widget": "stepper", "step": 0.1, "precision": 2, "group": "validation"}}),
    ("default", {"en": "Default", "de": "Standardwert"}, {"en": "Answer used until the respondent answers, typed by the question kind.", "de": "Antwort, die gilt, bis die befragte Person antwortet, typisiert nach Fragetyp."}, {"x-ui": {"group": "value"}}),
    ("params", {"en": "Parameters", "de": "Parameter"}, {"en": "Extra settings of the question type as one JSON object.", "de": "Zusätzliche Einstellungen des Fragetyps als ein JSON-Objekt."}, {"type": ["object", "null"], "x-ui": {"group": "kind"}}),
    ("condition", {"en": "Condition", "de": "Bedingung"}, {"en": "The question is shown only while this expression holds; empty always shows it.", "de": "Die Frage wird nur angezeigt, solange dieser Ausdruck gilt; leer zeigt sie immer."}, {"oneOf": [{"$ref": f"{DEFINITION}#/$defs/Expression"}, {"type": "null"}], "x-ui": {"group": "condition"}}),
    ("options", {"en": "Options", "de": "Optionen"}, {"en": "Choices of a select question, values unique.", "de": "Auswahlmöglichkeiten einer Auswahlfrage, Werte eindeutig."}, {"type": ["array", "null"], "items": OPTION, "x-ui": {"group": "choices"}}),
    ("fields", {"en": "Vector fields", "de": "Vektorfelder"}, {"en": "Components of a vector question, keys unique.", "de": "Komponenten einer Vektorfrage, Schlüssel eindeutig."}, {"type": ["array", "null"], "items": VECTOR_FIELD, "x-ui": {"group": "choices"}}),
]


def member(name, label, description, value):
    ui = value.pop("x-ui")
    value["x-semio-ui"] = {**({"widget": ui["widget"]} if "widget" in ui else {}), "role": "value", "label": label, "description": description, **{key: ui[key] for key in ("step", "precision") if key in ui}, "group": ui["group"], "order": 30}
    return {
        "type": "object",
        "additionalProperties": False,
        "required": ["mutation", "blockId", "field", "value"],
        "properties": {
            "mutation": {"const": "changeBlockField"},
            "blockId": {
                "type": "string",
                "minLength": 1,
                "x-semio-ui": {
                    "widget": "reference",
                    "role": "target",
                    "label": {"en": "Question", "de": "Frage"},
                    "description": {"en": "The question whose field this mutation sets.", "de": "Die Frage, deren Feld diese Mutation setzt."},
                    "ref": {"kind": "question", "domain": "fields", "granularity": "field"},
                    "group": "target",
                    "order": 10,
                },
            },
            "field": {
                "const": name,
                "x-semio-ui": {
                    "role": "discriminator",
                    "label": {"en": "Field", "de": "Feld"},
                    "description": {"en": "The question field this mutation sets.", "de": "Das Fragenfeld, das diese Mutation setzt."},
                },
            },
            "value": value,
        },
        "x-semio-ui": {"label": label},
    }


def main():
    schema = {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": "https://json.schemas.assets.semio-tech.com/s/forms/forms/mutation/change-block-field/schema.json",
        "title": "ChangeBlockField",
        "description": "Sets ONE field of one question to an absolute value (null clears an optional field).",
        "oneOf": [member(name, label, description, dict(value)) for name, label, description, value in FIELDS],
    }
    LEAF.parent.mkdir(parents=True, exist_ok=True)
    LEAF.write_text(json.dumps(schema, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
