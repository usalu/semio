#!/usr/bin/env python3
"""🧪️ S3-PROCEDURAL — writes the schema-first surfaces of the generation3d `change-widget-input` leaf (design §19 Q2).

Beside the hand-written Rust (`🦀️.rs`, `🔺️diff`, `↩️inverse`) this writes the leaf descriptor (`🔣️.json`), the payload
JSON Schema — a discriminated ROOT union over `type` (number | text | boolean | point | vector), every member with
complete `x-semio-ui` (labels en/de, widget, step, precision, `role: target` + `ref`) — the TypeScript payload twin with
`parseChangeWidgetInput()`, and the committed wire witness. A list variant carries no widget: array inputs render through
the generic array editor (item rows + Add/Remove honoring `maxItems`), inferred from the array schema.

Safe by default: a bare run is a DRY RUN that names every file it would change and writes nothing. `--write` writes only
files whose content differs, and REFUSES a file whose content is not this generator's last output (sha256 stamps in
`🧪️s3-procedural-generator-stamps.json`) — fold such a peer edit into this generator first, then pass `--force` once:
`python3 🧪️s3-procedural-change-widget-input.py [--write [--force]]`.
"""
import sys
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[7]
SUBSET = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any"
LEAF = f"{SUBSET}/🧬️schema/🧬️mutations/🎛️change-widget-input"
WITNESS = f"{SUBSET}/🧫️fixtures/🧬️mutations/🎛️change-widget-input/🧾️wire-witness/🦠️mutation/🔣️.json"
MAXIMUM_TEXT = 16_777_216
MAXIMUM_ITEMS = 1024


def label(en, de):
    return {"en": en, "de": de}


def items_list(items):
    return {
        "type": "array",
        "maxItems": MAXIMUM_ITEMS,
        "items": items,
        "x-semio-ui": {"role": "value", "label": label("Items", "Einträge"), "group": "value", "order": 40},
    }


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
    ("numberList", label("Number List", "Zahlliste"), items_list({"type": "number"})),
    ("textList", label("Text List", "Textliste"), items_list({"type": "string", "maxLength": MAXIMUM_TEXT})),
    ("booleanList", label("Switch List", "Schalterliste"), items_list({"type": "boolean"})),
    ("pointList", label("Point List", "Punktliste"), items_list({"type": "array", "items": {"type": "number"}, "minItems": 3, "maxItems": 3})),
    ("vectorList", label("Vector List", "Vektorliste"), items_list({"type": "array", "items": {"type": "number"}, "minItems": 3, "maxItems": 3})),
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
        "widget": "select",
        "role": "value",
        "label": label("Input", "Eingang"),
        "description": label("The unconnected input of the operator (a text source's `text`).", "Der unverbundene Eingang des Operators (bei einer Textquelle `text`)."),
        "optionSource": {"snapshot": "/hostSnapshot/widgets/{id}/params"},
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
type Triple = [number, number, number];

export type WidgetInputValue =
  | {{ type: "number"; value: number }}
  | {{ type: "text"; value: string }}
  | {{ type: "boolean"; value: boolean }}
  | {{ type: "point"; value: Triple }}
  | {{ type: "vector"; value: Triple }}
  | {{ type: "numberList"; value: number[] }}
  | {{ type: "textList"; value: string[] }}
  | {{ type: "booleanList"; value: boolean[] }}
  | {{ type: "pointList"; value: Triple[] }}
  | {{ type: "vectorList"; value: Triple[] }};

export type ChangeWidgetInput = {{ id: string; channel: string }} & WidgetInputValue;

const finite = (value: unknown): value is number => typeof value === "number" && Number.isFinite(value);

/** 🔣️ One scalar item of the element type `type`, or throws. */
function parseItem(type: string, value: unknown): unknown {{
  switch (type) {{
    case "number":
      if (!finite(value)) throw new TypeError("change-widget-input: a number value must be finite");
      return value;
    case "text":
      if (typeof value !== "string" || Array.from(value).length > {MAXIMUM_TEXT}) throw new TypeError("change-widget-input: a text value is a string of at most 16 MiB");
      return value;
    case "boolean":
      if (typeof value !== "boolean") throw new TypeError("change-widget-input: a boolean value is true or false");
      return value;
    case "point":
    case "vector":
      if (!Array.isArray(value) || value.length !== 3 || !value.every(finite)) throw new TypeError(`change-widget-input: a ${{type}} value is three finite numbers`);
      return [value[0], value[1], value[2]];
    default:
      throw new TypeError("change-widget-input: type must be number, text, boolean, point or vector, or a list of one of them");
  }}
}}

/** 🔣️ The one typed value the discriminator `type` names — a scalar, or a list of at most {MAXIMUM_ITEMS} scalars of one type —
 * or throws. */
function parseValue(type: unknown, value: unknown): WidgetInputValue {{
  if (typeof type !== "string") throw new TypeError("change-widget-input: type must be a string");
  if (!type.endsWith("List")) return {{ type, value: parseItem(type, value) }} as WidgetInputValue;
  if (!Array.isArray(value) || value.length > {MAXIMUM_ITEMS}) throw new TypeError("change-widget-input: a list value holds at most {MAXIMUM_ITEMS} items");
  return {{ type, value: value.map((item) => parseItem(type.slice(0, -4), item)) }} as WidgetInputValue;
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


def outputs():
    base = ROOT / LEAF
    return [
        (base / "🔣️.json", json.dumps(descriptor(), indent=2, ensure_ascii=False) + "\n"),
        (base / "🧬️schema" / "🔣️.json", json.dumps(schema(), indent=2, ensure_ascii=False) + "\n"),
        (base / "🦠️mutation" / "🟦️.ts", TYPESCRIPT),
        (ROOT / WITNESS, json.dumps(WITNESS_PAYLOAD, indent=2, ensure_ascii=False) + "\n"),
    ]


STAMPS = pathlib.Path(__file__).resolve().parent / "🧪️s3-procedural-generator-stamps.json"


def guarded_write(targets, arguments):
    """🛡️ Writes `(path, text)` targets safely (S3 incident, 10-03): a bare run is a DRY RUN; `--write` writes only files
    whose content differs, and REFUSES any file whose content is not what this generator last wrote there (recorded as a
    sha256 in `🧪️s3-procedural-generator-stamps.json`) — a peer edit is never overwritten; fold it into the generator and
    pass `--force` once."""
    import hashlib
    unknown = [argument for argument in arguments if argument not in ("--write", "--force")]
    if unknown:
        sys.exit(f"unknown argument(s) {unknown}; usage: [--write [--force]]")
    commit, force = "--write" in arguments, "--force" in arguments
    stamps = json.loads(STAMPS.read_text(encoding="utf-8")) if STAMPS.exists() else {}
    digest = lambda text: hashlib.sha256(text.encode("utf-8")).hexdigest()
    refused = False
    for path, text in targets:
        key = str(path.relative_to(ROOT))
        current = path.read_text(encoding="utf-8") if path.exists() else None
        if current == text:
            stamps[key] = digest(text)
            continue
        if current is not None and stamps.get(key) != digest(current) and not force:
            refused = True
            print(f"refused (not this generator's last output): {key}")
            continue
        print(f"{'writes' if commit else 'would write'}: {key}")
        if commit:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text, encoding="utf-8")
            stamps[key] = digest(text)
    if commit:
        STAMPS.write_text(json.dumps(dict(sorted(stamps.items())), indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    if refused:
        sys.exit(1)


def main(arguments):
    guarded_write(outputs(), arguments)


if __name__ == "__main__":
    main(sys.argv[1:])
