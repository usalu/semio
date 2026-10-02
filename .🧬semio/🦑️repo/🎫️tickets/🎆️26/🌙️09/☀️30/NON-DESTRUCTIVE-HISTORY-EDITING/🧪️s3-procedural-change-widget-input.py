#!/usr/bin/env python3
"""🧪️ S3-PROCEDURAL — writes the schema-first surfaces of the generation3d `change-widget-input` leaf (design §19 Q2).

Beside the hand-written Rust (`🦀️.rs`, `🔺️diff`, `↩️inverse`) this writes the leaf descriptor (`🔣️.json`), the payload
JSON Schema — a discriminated ROOT union over `type` (number | text | boolean | point | vector), every member with
complete `x-semio-ui` (labels en/de, widget, step, precision, `role: target` + `ref`) — the TypeScript payload twin with
`parseChangeWidgetInput()`, and the committed wire witness. Re-running reproduces the committed files byte for byte:
`python3 🧪️s3-procedural-change-widget-input.py`.
"""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[7]
SUBSET = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any"
LEAF = f"{SUBSET}/🧬️schema/🧬️mutations/🎛️change-widget-input"
WITNESS = f"{SUBSET}/🧫️fixtures/🧬️mutations/🎛️change-widget-input/🧾️wire-witness/🦠️mutation/🔣️.json"
MAXIMUM_TEXT = 1_048_576


def label(en, de):
    return {"en": en, "de": de}


def coordinates(en, de):
    return {
        "type": "array",
        "items": {"type": "number"},
        "minItems": 3,
        "maxItems": 3,
        "x-semio-ui": {"widget": "vector", "role": "value", "label": label(en, de), "step": 0.1, "precision": 3, "group": "value", "order": 40},
    }


VARIANTS = [
    ("number", label("Number", "Zahl"), {"type": "number", "x-semio-ui": {"widget": "stepper", "role": "value", "label": label("Value", "Wert"), "step": 0.1, "precision": 3, "group": "value", "order": 40}}),
    ("text", label("Text", "Text"), {"type": "string", "maxLength": MAXIMUM_TEXT, "x-semio-ui": {"widget": "multiline", "role": "value", "label": label("Text", "Text"), "group": "value", "order": 40}}),
    ("boolean", label("Switch", "Schalter"), {"type": "boolean", "x-semio-ui": {"widget": "toggle", "role": "value", "label": label("On", "Ein"), "group": "value", "order": 40}}),
    ("point", label("Point", "Punkt"), coordinates("Point", "Punkt")),
    ("vector", label("Vector", "Vektor"), coordinates("Vector", "Vektor")),
]

TARGET = {
    "type": "string",
    "minLength": 1,
    "x-semio-ui": {
        "widget": "reference",
        "role": "target",
        "label": label("Operator", "Operator"),
        "description": label("The operator (or text source) whose input this value is set on.", "Der Operator (oder die Textquelle), an dessen Eingang dieser Wert gesetzt wird."),
        "ref": {"kind": "widget", "domain": "graph", "granularity": "node"},
        "group": "target",
        "order": 10,
    },
}

CHANNEL = {
    "type": "string",
    "minLength": 1,
    "maxLength": 256,
    "x-semio-ui": {
        "widget": "text",
        "role": "value",
        "label": label("Input", "Eingang"),
        "description": label("The unconnected input of the operator (a text source's `text`).", "Der unverbundene Eingang des Operators (bei einer Textquelle `text`)."),
        "group": "target",
        "order": 20,
    },
}


def member(kind, variant_label, value):
    return {
        "type": "object",
        "additionalProperties": False,
        "required": ["mutation", "id", "channel", "type", "value"],
        "properties": {
            "mutation": {"const": "changeWidgetInput"},
            "id": TARGET,
            "channel": CHANNEL,
            "type": {"const": kind, "x-semio-ui": {"role": "discriminator", "label": label("Type", "Typ"), "description": label("The literal type the input declares.", "Der Literaltyp, den der Eingang deklariert.")}},
            "value": value,
        },
        "x-semio-ui": {"label": variant_label},
    }


def schema():
    return {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": "https://json.schemas.assets.semio-tech.com/s/procedural/generation3d/mutation/change-widget-input/schema.json",
        "title": "ChangeWidgetInput",
        "description": "The `change-widget-input` mutation record — the ABSOLUTE typed literal one unconnected operator input of the generator graph (or a text source's text) is set to: an inspector field committed on blur, or a parameter a mesh edit inserts its operator with. Numbers are finite; the diff refuses a wired input or a literal of another declared type.",
        "oneOf": [member(kind, variant_label, value) for kind, variant_label, value in VARIANTS],
    }


def descriptor():
    return {
        "schemaVersion": 1,
        "owner": LEAF,
        "semanticKind": "change-widget-input",
        "displayName": "Change Widget Input",
        "emoji": "🎛️",
        "aggregateVariant": "ChangeWidgetInput",
        "payloadSchema": "🧬️schema/🔣️.json",
        "textOpcode": None,
        "binaryTag": 19,
        "invertibility": "explicit-mutation",
        "diffParticipation": "detect",
        "outcomeClasses": ["applied", "no-op", "rejected"],
        "composition": "atomic",
        "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"],
    }


TYPESCRIPT = f'''/** 🎛️ generation3d direct `change-widget-input` payload mirror of `ChangeWidgetInput`, with its closed-schema parser. */
export type WidgetInputValue =
  | {{ type: "number"; value: number }}
  | {{ type: "text"; value: string }}
  | {{ type: "boolean"; value: boolean }}
  | {{ type: "point"; value: [number, number, number] }}
  | {{ type: "vector"; value: [number, number, number] }};

export type ChangeWidgetInput = {{ id: string; channel: string }} & WidgetInputValue;

const finite = (value: unknown): value is number => typeof value === "number" && Number.isFinite(value);

/** 🔣️ The one typed value the discriminator `type` names, or throws. */
function parseValue(type: unknown, value: unknown): WidgetInputValue {{
  switch (type) {{
    case "number":
      if (!finite(value)) throw new TypeError("change-widget-input: a number value must be finite");
      return {{ type, value }};
    case "text":
      if (typeof value !== "string" || Array.from(value).length > {MAXIMUM_TEXT}) throw new TypeError("change-widget-input: a text value is a string of at most 1 MiB");
      return {{ type, value }};
    case "boolean":
      if (typeof value !== "boolean") throw new TypeError("change-widget-input: a boolean value is true or false");
      return {{ type, value }};
    case "point":
    case "vector":
      if (!Array.isArray(value) || value.length !== 3 || !value.every(finite)) throw new TypeError(`change-widget-input: a ${{type}} value is three finite numbers`);
      return {{ type, value: [value[0], value[1], value[2]] }};
    default:
      throw new TypeError("change-widget-input: type must be number, text, boolean, point or vector");
  }}
}}

/** 🚪️ Parses one `change-widget-input` payload the way its JSON Schema admits it, or throws. */
export function parseChangeWidgetInput(value: unknown): ChangeWidgetInput {{
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("change-widget-input: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["id", "channel", "type", "value", "mutation"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`change-widget-input: unknown field ${{unknownKey}}`);
  if (row.mutation !== undefined && row.mutation !== "changeWidgetInput") throw new TypeError("change-widget-input: wrong mutation tag");
  if (typeof row.id !== "string" || row.id.length === 0) throw new TypeError("change-widget-input: id must be a nonempty string");
  if (typeof row.channel !== "string" || row.channel.length === 0 || Array.from(row.channel).length > 256) throw new TypeError("change-widget-input: channel must be a nonempty string of at most 256 characters");
  return {{ id: row.id, channel: row.channel, ...parseValue(row.type, row.value) }};
}}
'''

WITNESS_PAYLOAD = {"mutation": "changeWidgetInput", "id": "extrude__extrude", "channel": "distance", "type": "number", "value": 0.25}


def write(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def main():
    base = ROOT / LEAF
    write(base / "🔣️.json", json.dumps(descriptor(), indent=2, ensure_ascii=False) + "\n")
    write(base / "🧬️schema" / "🔣️.json", json.dumps(schema(), indent=2, ensure_ascii=False) + "\n")
    write(base / "🦠️mutation" / "🟦️.ts", TYPESCRIPT)
    write(ROOT / WITNESS, json.dumps(WITNESS_PAYLOAD, indent=2, ensure_ascii=False) + "\n")
    print(LEAF)


if __name__ == "__main__":
    main()
