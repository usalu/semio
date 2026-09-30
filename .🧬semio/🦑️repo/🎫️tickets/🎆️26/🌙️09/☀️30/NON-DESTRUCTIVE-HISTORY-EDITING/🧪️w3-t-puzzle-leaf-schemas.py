#!/usr/bin/env python3
"""🧬️ W3-T-PUZZLE authoring tool: writes the descriptor, the payload JSON Schema (with complete `x-semio-ui`)
and the TS twin of every parametric selection leaf of puzzle 3d and puzzle 5d. Re-running it reproduces the
committed files byte for byte."""
import json
import math
import os
import sys

PUZZLE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts"
SUBSET = {"3d": PUZZLE + "/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any", "5d": PUZZLE + "/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any"}
OWNER = {"3d": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any", "5d": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any"}
SCHEMA_ID = {"3d": "https://json.schemas.assets.semio-tech.com/s/puzzle/puzzle3d/mutation/%s/schema.json", "5d": "https://json.schemas.assets.semio-tech.com/s/puzzle/puzzle5d/mutation/%s/schema.json"}


def discriminator():
    return {"widget": "hidden", "role": "discriminator", "label": {"en": "Mutation", "de": "Mutation"}}


def targets(kinds, granularity, en, de, description_en, description_de):
    return {
        "type": "array",
        "items": {"type": "string"},
        "minItems": 1,
        "uniqueItems": True,
        "x-semio-ui": {
            "widget": "reference",
            "role": "target",
            "label": {"en": en, "de": de},
            "description": {"en": description_en, "de": description_de},
            "ref": {"kind": kinds, "domain": "vortex", "granularity": granularity},
            "group": "target",
            "order": 10,
        },
    }


def offset(snap, order=20):
    return {
        "type": "array",
        "items": {"type": "number"},
        "minItems": 3,
        "maxItems": 3,
        "x-semio-ui": {
            "widget": "vector",
            "role": "value",
            "label": {"en": "Offset", "de": "Versatz"},
            "description": {"en": "World offset (x, y, z) every target moves by.", "de": "Weltversatz (x, y, z), um den sich jedes Ziel verschiebt."},
            "unit": "m",
            "step": 0.1,
            "precision": 3,
            "snapSource": {"config": snap},
            "group": "offset",
            "order": order,
        },
    }


def axis():
    return {
        "type": "array",
        "items": {"type": "number"},
        "minItems": 3,
        "maxItems": 3,
        "x-semio-ui": {
            "widget": "vector",
            "role": "value",
            "label": {"en": "Axis", "de": "Achse"},
            "description": {"en": "World axis (x, y, z) every target turns about, through its own origin.", "de": "Weltachse (x, y, z), um die sich jedes Ziel durch seinen eigenen Ursprung dreht."},
            "step": 0.1,
            "precision": 3,
            "group": "rotation",
            "order": 20,
        },
    }


def angle():
    return {
        "type": "number",
        "x-semio-ui": {
            "widget": "dial",
            "role": "value",
            "label": {"en": "Angle", "de": "Winkel"},
            "description": {"en": "Right-handed rotation about the axis.", "de": "Rechtshändige Drehung um die Achse."},
            "unit": "rad",
            "displayUnit": "deg",
            "displayFactor": 180.0 / math.pi,
            "step": math.pi / 180.0,
            "softMin": -math.pi,
            "softMax": math.pi,
            "snaps": [-math.pi, -math.pi / 2, 0, math.pi / 2, math.pi],
            "group": "rotation",
            "order": 30,
        },
    }


def factors():
    return {
        "type": "array",
        "items": {"type": "number", "exclusiveMinimum": 0},
        "minItems": 3,
        "maxItems": 3,
        "x-semio-ui": {
            "widget": "vector",
            "role": "value",
            "label": {"en": "Factors", "de": "Faktoren"},
            "description": {"en": "Per-axis factors (x, y, z) every target's scale is multiplied by; 1 keeps an axis.", "de": "Faktoren je Achse (x, y, z), mit denen die Skalierung jedes Ziels multipliziert wird; 1 behält eine Achse bei."},
            "step": 0.1,
            "precision": 3,
            "group": "scale",
            "order": 20,
        },
    }


def flat(axis_name, order):
    return {
        "type": "number",
        "x-semio-ui": {
            "widget": "stepper",
            "role": "value",
            "label": {"en": "Offset %s" % axis_name, "de": "Versatz %s" % axis_name},
            "step": 1,
            "precision": 2,
            "snapSource": {"config": "gridFactor"},
            "group": "offset",
            "order": order,
        },
    }


OBJECT_TARGETS_3D = lambda verb_en, verb_de: targets(["object", "targetVolume"], "object", "Targets", "Ziele", "Objects and target volumes to %s; locked or missing ones are skipped." % verb_en, "Zu %s Objekte und Zielvolumen; gesperrte oder fehlende werden übersprungen." % verb_de)
PART_TARGETS_3D = lambda verb_en, verb_de: targets(["part", "targetVolume"], "part", "Targets", "Ziele", "Parts and target volumes to %s; locked or missing ones are skipped." % verb_en, "Zu %s Teile und Zielvolumen; gesperrte oder fehlende werden übersprungen." % verb_de)

LEAVES = {
    "3d": [
        ("✋️drag-selection", "drag-selection", "DragSelection", "Drag Selection", "✋️", 35, OBJECT_TARGETS_3D("drag", "ziehende"), [("offset", offset("gridSpacing"))], "a relative drag of objects and target volumes by one world offset", "[number, number, number]"),
        ("🔄️rotate-selection", "rotate-selection", "RotateSelection", "Rotate Selection", "🔄️", 36, OBJECT_TARGETS_3D("rotate", "drehende"), [("axis", axis()), ("angle", angle())], "a relative turn of objects and target volumes, each about its own origin", None),
        ("🔍️scale-selection", "scale-selection", "ScaleSelection", "Scale Selection", "🔍️", 37, OBJECT_TARGETS_3D("scale", "skalierende"), [("factors", factors())], "a relative per-axis scaling of objects and target volumes, each about its own origin", None),
    ],
    "5d": [
        ("✋️drag-selection2d", "drag-selection2d", "DragSelection2d", "Drag Selection 2d", "✋️", 35, targets(["part"], "part", "Parts", "Teile", "Parts whose board position moves; locked or missing ones are skipped.", "Teile, deren Brettposition sich verschiebt; gesperrte oder fehlende werden übersprungen."), [("dx", flat("X", 20)), ("dy", flat("Y", 30))], "a relative drag of parts on the board by one flat offset", None),
        ("🚚️drag-selection3d", "drag-selection3d", "DragSelection3d", "Drag Selection 3d", "🚚️", 36, PART_TARGETS_3D("drag", "ziehende"), [("offset", offset("gridSpacing"))], "a relative drag of parts and target volumes in the world by one offset", None),
        ("🔄️rotate-selection3d", "rotate-selection3d", "RotateSelection3d", "Rotate Selection 3d", "🔄️", 37, PART_TARGETS_3D("rotate", "drehende"), [("axis", axis()), ("angle", angle())], "a relative turn of parts and target volumes, each about its own origin", None),
        ("🔍️scale-selection3d", "scale-selection3d", "ScaleSelection3d", "Scale Selection 3d", "🔍️", 38, PART_TARGETS_3D("scale", "skalierende"), [("factors", factors())], "a relative per-axis scaling of parts and target volumes, each about its own origin", None),
    ],
}

TS_TYPES = {"targets": "string[]", "offset": "[number, number, number]", "axis": "[number, number, number]", "angle": "number", "factors": "[number, number, number]", "dx": "number", "dy": "number"}


def camel(kind):
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def main():
    for artifact, leaves in LEAVES.items():
        for directory, kind, variant, display, emoji, tag, target_schema, fields, summary, _ in leaves:
            root = os.path.join(SUBSET[artifact], "🧬️schema", "🧬️mutations", directory)
            descriptor = {
                "schemaVersion": 1,
                "owner": OWNER[artifact] + "/🧬️schema/🧬️mutations/" + directory,
                "semanticKind": kind,
                "displayName": display,
                "emoji": emoji,
                "aggregateVariant": variant,
                "payloadSchema": "🧬️schema/🔣️.json",
                "textOpcode": kind,
                "binaryTag": tag,
                "invertibility": "explicit-mutation",
                "diffParticipation": "detect",
                "outcomeClasses": ["applied", "no-op", "rejected"],
                "composition": "atomic",
                "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"],
            }
            write(os.path.join(root, "🔣️.json"), json.dumps(descriptor, indent=2, ensure_ascii=False) + "\n")
            properties = {"mutation": {"const": camel(kind), "x-semio-ui": discriminator()}, "targets": target_schema}
            properties.update(dict(fields))
            schema = {
                "$schema": "http://json-schema.org/draft-07/schema#",
                "$id": SCHEMA_ID[artifact] % kind,
                "title": variant,
                "description": "The `%s` mutation record — %s; internally tagged `\"mutation\": \"%s\"`, the one branch the aggregate `../🔣️.json` union carries for this kind." % (kind, summary, camel(kind)),
                "type": "object",
                "additionalProperties": False,
                "required": list(properties),
                "properties": properties,
            }
            write(os.path.join(root, "🧬️schema", "🔣️.json"), json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
            members = "\n".join("  %s: %s;" % (name, TS_TYPES[name]) for name in ["targets"] + [name for name, _ in fields])
            ts = '/** %s `%s` payload — %s; mirrors Rust `%s` (`../🦀️.rs`). */\nexport interface %s {\n%s\n}\n' % (emoji, kind, summary, variant, variant, members)
            write(os.path.join(root, "🦠️mutation", "🟦️.ts"), ts)
            print("wrote", artifact, kind)


if __name__ == "__main__":
    sys.exit(main())
