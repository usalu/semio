#!/usr/bin/env python3
"""🧪️ W3-T2-PROCEDURAL — writes the schema-first surfaces of the procedural relative/absolute gesture leaves.

For every new generation2d / generation3d mutation leaf this writes, beside the hand-written Rust:
the leaf descriptor (`🔣️.json`), the payload JSON Schema with complete `x-semio-ui` (labels en/de, widget, step,
precision, role, ref) and declared `x-semio-invariant`s, and (generation3d) the TypeScript payload twin with its
`parse<Type>()`. Re-running reproduces the committed files byte for byte. Run from anywhere:
`python3 🧪️w3-t2-procedural-leaves.py`.
"""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[7]
PLUGIN = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts"
SUBSET = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
ARTIFACTS = {"generation3d": "🧊️generation3d", "generation2d": "🌀️generation2d"}


def label(en, de):
    return {"en": en, "de": de}


def number(en, de, order, group, step, precision, extra=None, bounds=None):
    ui = {"widget": "stepper", "role": "value", "label": label(en, de), "step": step, "precision": precision, "group": group, "order": order}
    ui.update(extra or {})
    prop = {"type": "number"}
    prop.update(bounds or {})
    prop["x-semio-ui"] = ui
    return prop


def widget_ref(en, de, description_en, description_de, many):
    ui = {
        "widget": "reference",
        "role": "target",
        "label": label(en, de),
        "description": label(description_en, description_de),
        "ref": {"kind": "widget", "domain": "graph", "granularity": "node"},
        "group": "target",
        "order": 10,
    }
    if many:
        return {"type": "array", "items": {"type": "string", "minLength": 1}, "minItems": 1, "uniqueItems": True, "x-semio-ui": ui}
    return {"type": "string", "minLength": 1, "x-semio-ui": ui}


TRANSFORM_TARGETS = widget_ref(
    "Transforms",
    "Transformationen",
    "The gumball transform operators of the generator graph this gesture composes into; missing or other operators are skipped.",
    "Die Gumball-Transformationsoperatoren des Generatorgraphen, in die diese Geste einrechnet; fehlende oder andere Operatoren werden übersprungen.",
    True,
)

LEAVES = [
    {
        "artifact": "generation3d",
        "dir": "🎚️change-slider-value",
        "kind": "change-slider-value",
        "display": "Change Slider Value",
        "emoji": "🎚️",
        "variant": "ChangeSliderValue",
        "tag": 14,
        "outcomes": ["applied", "no-op", "rejected"],
        "description": "The `change-slider-value` mutation record — the ABSOLUTE value one input slider of the generator graph is set to (a dragged slider, a held spinner, a typed number). A value outside the slider's range widens the range like the canvas knob.",
        "properties": {
            "id": widget_ref("Slider", "Schieberegler", "The input slider this value is set on.", "Der Eingabe-Schieberegler, auf dem dieser Wert gesetzt wird.", False),
            "value": number("Value", "Wert", 20, "value", 0.1, 3),
        },
        "required": ["id", "value"],
        "ts": [("id", "string"), ("value", "number")],
    },
    {
        "artifact": "generation3d",
        "dir": "✋️drag-transforms",
        "kind": "drag-transforms",
        "display": "Drag Transforms",
        "emoji": "✋️",
        "variant": "DragTransforms",
        "tag": 15,
        "outcomes": ["applied", "no-op", "rejected"],
        "description": "The `drag-transforms` mutation record — one gumball drag as intent: the offset every addressed translate operator adds to its own offset, read off the BASE graph, so an edited drag replays on any base.",
        "properties": {
            "targets": TRANSFORM_TARGETS,
            "dx": number("Offset X", "Versatz X", 20, "offset", 0.1, 3),
            "dy": number("Offset Y", "Versatz Y", 30, "offset", 0.1, 3),
            "dz": number("Offset Z", "Versatz Z", 40, "offset", 0.1, 3),
        },
        "required": ["targets", "dx", "dy", "dz"],
        "ts": [("targets", "string[]"), ("dx", "number"), ("dy", "number"), ("dz", "number")],
    },
    {
        "artifact": "generation3d",
        "dir": "🔃️rotate-transforms",
        "kind": "rotate-transforms",
        "display": "Rotate Transforms",
        "emoji": "🔃️",
        "variant": "RotateTransforms",
        "tag": 16,
        "outcomes": ["applied", "no-op", "rejected"],
        "description": "The `rotate-transforms` mutation record — one gumball rotation as intent: the world-axis rotation every addressed rotate operator composes after its own rotation, read off the BASE graph.",
        "properties": {
            "targets": TRANSFORM_TARGETS,
            "ax": number("Axis X", "Achse X", 20, "axis", 0.1, 3),
            "ay": number("Axis Y", "Achse Y", 30, "axis", 0.1, 3),
            "az": number("Axis Z", "Achse Z", 40, "axis", 0.1, 3),
            "angle": number("Angle", "Winkel", 50, "angle", 0.017453292519943295, 4, {"widget": "dial", "unit": "rad", "displayUnit": "°", "displayFactor": 57.29577951308232}),
        },
        "required": ["targets", "ax", "ay", "az", "angle"],
        "invariants": [
            {
                "id": "axis-nonzero",
                "description": label("The rotation axis is not the zero vector.", "Die Drehachse ist nicht der Nullvektor."),
            }
        ],
        "ts": [("targets", "string[]"), ("ax", "number"), ("ay", "number"), ("az", "number"), ("angle", "number")],
    },
    {
        "artifact": "generation3d",
        "dir": "📏️scale-transforms",
        "kind": "scale-transforms",
        "display": "Scale Transforms",
        "emoji": "📏️",
        "variant": "ScaleTransforms",
        "tag": 17,
        "outcomes": ["applied", "no-op", "rejected"],
        "description": "The `scale-transforms` mutation record — one gumball scaling as intent: the per-axis factors every addressed scale operator multiplies into its own factors, read off the BASE graph.",
        "properties": {
            "targets": TRANSFORM_TARGETS,
            "sx": number("Factor X", "Faktor X", 20, "factor", 0.1, 3, None, {"exclusiveMinimum": 0}),
            "sy": number("Factor Y", "Faktor Y", 30, "factor", 0.1, 3, None, {"exclusiveMinimum": 0}),
            "sz": number("Factor Z", "Faktor Z", 40, "factor", 0.1, 3, None, {"exclusiveMinimum": 0}),
        },
        "required": ["targets", "sx", "sy", "sz"],
        "ts": [("targets", "string[]"), ("sx", "number"), ("sy", "number"), ("sz", "number")],
    },
    {
        "artifact": "generation3d",
        "dir": "🚚️move-nodes",
        "kind": "move-nodes",
        "display": "Move Nodes",
        "emoji": "🚚️",
        "variant": "MoveNodes",
        "tag": 18,
        "outcomes": ["applied", "no-op", "rejected"],
        "description": "The `move-nodes` mutation record — one node-graph drag as intent: the canvas offset every addressed widget moves by from its BASE position, so an edited drag replays on any base.",
        "properties": {
            "ids": widget_ref("Nodes", "Knoten", "The graph widgets this drag moves; missing or unplaced ones are skipped.", "Die Graph-Bausteine, die dieser Zug verschiebt; fehlende oder unplatzierte werden übersprungen.", True),
            "dx": number("Offset X", "Versatz X", 20, "offset", 1, 1),
            "dy": number("Offset Y", "Versatz Y", 30, "offset", 1, 1),
        },
        "required": ["ids", "dx", "dy"],
        "ts": [("ids", "string[]"), ("dx", "number"), ("dy", "number")],
    },
    {
        "artifact": "generation2d",
        "dir": "🎚️change-slider-value",
        "kind": "change-slider-value",
        "display": "Change Slider Value",
        "emoji": "🎚️",
        "variant": "ChangeSliderValue",
        "tag": 14,
        "outcomes": ["applied", "no-op", "rejected"],
        "description": "The `change-slider-value` mutation record — the ABSOLUTE value one input slider of the generator graph is set to (a dragged slider, a held spinner, a typed number). A value outside the slider's range widens the range like the canvas knob.",
        "properties": {
            "id": widget_ref("Slider", "Schieberegler", "The input slider this value is set on.", "Der Eingabe-Schieberegler, auf dem dieser Wert gesetzt wird.", False),
            "value": number("Value", "Wert", 20, "value", 0.1, 3),
        },
        "required": ["id", "value"],
        "ts": [("id", "string"), ("value", "number")],
    },
    {
        "artifact": "generation2d",
        "dir": "🚚️move-nodes",
        "kind": "move-nodes",
        "display": "Move Nodes",
        "emoji": "🚚️",
        "variant": "MoveNodes",
        "tag": 15,
        "outcomes": ["applied", "no-op", "rejected"],
        "description": "The `move-nodes` mutation record — one node-graph drag as intent: the canvas offset every addressed widget moves by from its BASE position, so an edited drag replays on any base.",
        "properties": {
            "ids": widget_ref("Nodes", "Knoten", "The graph widgets this drag moves; missing or unplaced ones are skipped.", "Die Graph-Bausteine, die dieser Zug verschiebt; fehlende oder unplatzierte werden übersprungen.", True),
            "dx": number("Offset X", "Versatz X", 20, "offset", 1, 1),
            "dy": number("Offset Y", "Versatz Y", 30, "offset", 1, 1),
        },
        "required": ["ids", "dx", "dy"],
        "ts": [("ids", "string[]"), ("dx", "number"), ("dy", "number")],
    },
]


def camel(kind):
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


def write(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def descriptor(leaf, owner):
    return {
        "schemaVersion": 1,
        "owner": owner,
        "semanticKind": leaf["kind"],
        "displayName": leaf["display"],
        "emoji": leaf["emoji"],
        "aggregateVariant": leaf["variant"],
        "payloadSchema": "🧬️schema/🔣️.json",
        "textOpcode": None,
        "binaryTag": leaf["tag"],
        "invertibility": "explicit-mutation",
        "diffParticipation": "detect",
        "outcomeClasses": leaf["outcomes"],
        "composition": "atomic",
        "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"],
    }


def schema(leaf):
    artifact = leaf["artifact"]
    properties = {}
    required = list(leaf["required"])
    if artifact == "generation3d":
        properties["mutation"] = {"const": camel(leaf["kind"]), "x-semio-ui": {"widget": "hidden", "role": "discriminator", "label": label("Mutation", "Mutation")}}
        required = ["mutation"] + required
    properties.update(leaf["properties"])
    document = {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": f"https://json.schemas.assets.semio-tech.com/s/procedural/{artifact}/mutation/{leaf['kind']}/schema.json",
        "title": leaf["variant"],
        "description": leaf["description"],
        "type": "object",
        "additionalProperties": False,
        "required": required,
        "properties": properties,
    }
    if leaf.get("invariants"):
        document["x-semio-invariant"] = leaf["invariants"]
    return document


def typescript(leaf):
    fields = leaf["ts"]
    keys = ", ".join(f'"{name}"' for name, _ in fields)
    tagged = leaf["artifact"] == "generation3d"
    lines = [f"/** {leaf['emoji']} {leaf['artifact']} direct `{leaf['kind']}` payload mirror of `{leaf['variant']}`, with its closed-schema parser. */"]
    lines.append(f"export interface {leaf['variant']} {{")
    lines += [f"  {name}: {kind};" for name, kind in fields]
    lines.append("}")
    lines.append("")
    lines.append(f"/** 🚪️ Parses one `{leaf['kind']}` payload the way its JSON Schema admits it, or throws. */")
    lines.append(f"export function parse{leaf['variant']}(value: unknown): {leaf['variant']} {{")
    lines.append(f'  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("{leaf["kind"]}: payload is not an object");')
    lines.append("  const row = value as Record<string, unknown>;")
    allowed = keys + (', "mutation"' if tagged else "")
    lines.append(f'  const unknownKey = Object.keys(row).find((key) => ![{allowed}].includes(key));')
    lines.append(f'  if (unknownKey !== undefined) throw new TypeError(`{leaf["kind"]}: unknown field ${{unknownKey}}`);')
    if tagged:
        lines.append(f'  if (row.mutation !== undefined && row.mutation !== "{camel(leaf["kind"])}") throw new TypeError("{leaf["kind"]}: wrong mutation tag");')
    for name, kind in fields:
        if kind == "string":
            lines.append(f'  if (typeof row.{name} !== "string" || row.{name}.length === 0) throw new TypeError("{leaf["kind"]}: {name} must be a nonempty string");')
        elif kind == "string[]":
            lines.append(f'  if (!Array.isArray(row.{name}) || row.{name}.length === 0 || row.{name}.some((entry) => typeof entry !== "string" || entry.length === 0) || new Set(row.{name}).size !== row.{name}.length) throw new TypeError("{leaf["kind"]}: {name} must be a nonempty list of unique ids");')
        else:
            positive = name in ("sx", "sy", "sz")
            check = f'typeof row.{name} !== "number" || !Number.isFinite(row.{name})' + (f" || row.{name} <= 0" if positive else "")
            lines.append(f'  if ({check}) throw new TypeError("{leaf["kind"]}: {name} must be a finite{" positive" if positive else ""} number");')
    if leaf["kind"] == "rotate-transforms":
        lines.append('  if (row.ax === 0 && row.ay === 0 && row.az === 0) throw new TypeError("rotate-transforms: axis-nonzero");')
    body = ", ".join(f"{name}: row.{name} as {kind}" for name, kind in fields)
    lines.append(f"  return {{ {body} }};")
    lines.append("}")
    return "\n".join(lines) + "\n"


def main():
    for leaf in LEAVES:
        base = ROOT / PLUGIN / ARTIFACTS[leaf["artifact"]] / SUBSET / leaf["dir"]
        owner = f"{PLUGIN}/{ARTIFACTS[leaf['artifact']]}/{SUBSET}/{leaf['dir']}"
        write(base / "🔣️.json", json.dumps(descriptor(leaf, owner), indent=2, ensure_ascii=False) + "\n")
        write(base / "🧬️schema" / "🔣️.json", json.dumps(schema(leaf), indent=2, ensure_ascii=False) + "\n")
        write(base / "🦠️mutation" / "🟦️.ts", typescript(leaf))
        print(owner)


if __name__ == "__main__":
    main()
