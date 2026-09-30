#!/usr/bin/env python3
"""🏭️ W2-S-E process3d parity: the tag-only `ProcessMeasure` / `MeasureRecipe` / `CapabilityRule` stubs become the real
internally tagged unions (camelCase tags, every variant field, en/de `x-semio-ui`), `replace-stock-solid` references the io
`ArtifactRef` (its `dialect` is an object), `WorkingSolid` wires its multi-word variant fields camelCase
(`#[value(rename_all_fields = "camelCase")]`), and the `Option` fields admit `null`. Existing annotations stay; only the stub
nodes are replaced in place. Idempotent; each file is re-read right before it is written.

Usage: python3 🧪️w2-s-e-process.py [--apply]
"""
import json
import os
import sys

REPO = "/Users/ueli/Documents/semio"
ARTIFACT = "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d"
MUTATIONS = f"{ARTIFACT}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
IO_REF = "https://json.schemas.assets.semio-tech.com/framework/io/schema.json#/$defs/ArtifactRef"
APPLY = "--apply" in sys.argv


def read(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return handle.read()


def write(path, before, after):
    if before == after:
        return
    if APPLY:
        if read(path) != before:
            raise SystemExit(f"[w2-s-e] {path} changed while editing; rerun")
        with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
            handle.write(after)
    print(f"[w2-s-e] {'wrote' if APPLY else 'would write'} {path}")


def label(en, de, **extra):
    node = {"label": {"en": en, "de": de}}
    description = extra.pop("description", None)
    if description is not None:
        node["description"] = {"en": description[0], "de": description[1]}
    node.update(extra)
    return node


def metres(en, de, description=None):
    return {"type": "number", "x-semio-ui": {"widget": "stepper", "role": "value", **label(en, de, **({} if description is None else {"description": description})), "unit": "m", "displayUnit": "mm", "displayFactor": 1000, "step": 0.001, "precision": 1}}


def tag(name, value, en, de):
    return {"const": value, "x-semio-ui": {"widget": "hidden", "role": "discriminator", **label(en, de)}}


def branch(tag_name, value, en, de, fields, kind_en, kind_de):
    return {"type": "object", "additionalProperties": False, "required": [tag_name, *fields], "properties": {tag_name: tag(tag_name, value, kind_en, kind_de), **fields}, "x-semio-ui": label(en, de)}


POSE = {"title": "Pose", "type": "object", "additionalProperties": False, "required": ["position", "axis", "angle"], "properties": {
    "position": {"type": "array", "items": {"type": "number"}, "minItems": 3, "maxItems": 3, "x-semio-ui": {"widget": "vector", "role": "value", **label("Position", "Position", description=("Origin of the tool in stock coordinates, in metres.", "Ursprung des Werkzeugs in Rohteilkoordinaten, in Metern.")), "unit": "m", "step": 0.001}},
    "axis": {"type": "array", "items": {"type": "number"}, "minItems": 3, "maxItems": 3, "x-semio-ui": {"widget": "vector", "role": "value", **label("Rotation Axis", "Drehachse", description=("Direction the tool rotates about.", "Richtung, um die sich das Werkzeug dreht."))}},
    "angle": {"type": "number", "x-semio-ui": {"widget": "dial", "role": "value", **label("Angle", "Winkel", description=("Counter-clockwise rotation about the axis.", "Drehung gegen den Uhrzeigersinn um die Achse.")), "unit": "rad", "displayUnit": "deg", "displayFactor": 57.29577951308232, "step": 0.017453292519943295, "softMin": -3.141592653589793, "softMax": 3.141592653589793, "snaps": [-3.141592653589793, -1.5707963267948966, 0, 1.5707963267948966, 3.141592653589793]}},
}, "x-semio-ui": {"role": "value", **label("Pose", "Lage", description=("Position and axis-angle rotation of the tool.", "Position und Achse-Winkel-Drehung des Werkzeugs."))}}
SOLID_TAG = ("Solid type", "Körperart")
WORKING_SOLID = {"title": "WorkingSolid", "oneOf": [
    branch("kind", "box", "Box", "Quader", {"width": metres("Width", "Breite"), "depth": metres("Depth", "Tiefe"), "height": metres("Height", "Höhe")}, *SOLID_TAG),
    branch("kind", "cylinder", "Cylinder", "Zylinder", {"radius": metres("Radius", "Radius"), "height": metres("Height", "Höhe")}, *SOLID_TAG),
    branch("kind", "sphere", "Sphere", "Kugel", {"radius": metres("Radius", "Radius")}, *SOLID_TAG),
    branch("kind", "importedMesh", "Imported mesh", "Importiertes Netz", {"meshUrl": {"type": "string", "x-semio-ui": {"widget": "text", "role": "value", **label("Mesh URL", "Netz-URL", description=("Where the imported mesh is loaded from.", "Woher das importierte Netz geladen wird."))}}}, *SOLID_TAG),
    branch("kind", "importedSolid", "Imported solid", "Importierter Volumenkörper", {"solidHandle": {"type": "string", "x-semio-ui": {"widget": "text", "role": "value", **label("Solid handle", "Volumenkörper-Handle", description=("Handle of the imported B-rep solid.", "Handle des importierten B-Rep-Volumenkörpers."))}}}, *SOLID_TAG),
    branch("kind", "reference", "Reference solid", "Referenzkörper", {"referenceId": {"type": "string", "x-semio-ui": {"widget": "text", "role": "value", **label("Reference solid", "Referenzkörper", description=("Id of a solid from the built-in reference library.", "ID eines Körpers aus der eingebauten Referenzbibliothek."))}}}, *SOLID_TAG),
]}
MEASURE_TAG = ("Operation type", "Bearbeitungsart")


def measure():
    tool = dict(WORKING_SOLID, **{"x-semio-ui": {"role": "value", **label("Tool", "Werkzeug", description=("Solid removed from the stock.", "Körper, der vom Rohteil abgetragen wird."))}})
    component = dict(WORKING_SOLID, **{"x-semio-ui": {"role": "value", **label("Component", "Anbauteil", description=("Solid added to the stock.", "Körper, der am Rohteil angebaut wird."))}})
    return [
        branch("measure", "cut", "Cut", "Schnitt", {"tool": tool, "pose": POSE}, *MEASURE_TAG),
        branch("measure", "drill", "Drill", "Bohrung", {"radius": metres("Radius", "Radius", ("Bore radius.", "Bohrungsradius.")), "depth": metres("Depth", "Tiefe", ("Bore depth along the axis.", "Bohrtiefe entlang der Achse.")), "pose": POSE}, *MEASURE_TAG),
        branch("measure", "attach", "Attach", "Anbau", {"component": component, "pose": POSE}, *MEASURE_TAG),
    ]


def parameter(en, de):
    return {"type": "string", "x-semio-ui": {"widget": "text", "role": "value", **label(en, de, description=("Id of the capability parameter that supplies this size.", "ID des Fähigkeitsparameters, der dieses Maß liefert."))}}


RECIPE_TAG = ("Recipe type", "Rezeptart")
RECIPE = [
    branch("recipe", "discCut", "Disc cut", "Scheibenschnitt", {"diameter": parameter("Diameter", "Durchmesser"), "kerf": parameter("Kerf", "Schnittbreite")}, *RECIPE_TAG),
    branch("recipe", "bladeCut", "Blade cut", "Klingenschnitt", {"kerf": parameter("Kerf", "Schnittbreite"), "length": parameter("Length", "Länge"), "depth": parameter("Depth", "Tiefe")}, *RECIPE_TAG),
    branch("recipe", "pocketCut", "Pocket cut", "Taschenschnitt", {"diameter": parameter("Diameter", "Durchmesser"), "depth": parameter("Depth", "Tiefe")}, *RECIPE_TAG),
    branch("recipe", "boreDrill", "Bore drill", "Bohrung", {"radius": parameter("Radius", "Radius"), "depth": parameter("Depth", "Tiefe")}, *RECIPE_TAG),
    branch("recipe", "cylinderAttach", "Cylinder attach", "Zylinderanbau", {"radius": parameter("Radius", "Radius"), "length": parameter("Length", "Länge")}, *RECIPE_TAG),
    branch("recipe", "boxAttach", "Box attach", "Quaderanbau", {"width": parameter("Width", "Breite"), "depth": parameter("Depth", "Tiefe"), "height": parameter("Height", "Höhe")}, *RECIPE_TAG),
]
QUANTITY = {"type": "string", "enum": ["width", "depth", "height", "maxDimension", "minDimension"], "x-semio-ui": {"widget": "select", "role": "value", **label("Stock dimension", "Rohteilmaß"), "options": {"width": {"en": "Width", "de": "Breite"}, "depth": {"en": "Depth", "de": "Tiefe"}, "height": {"en": "Height", "de": "Höhe"}, "maxDimension": {"en": "Largest dimension", "de": "Größtes Maß"}, "minDimension": {"en": "Smallest dimension", "de": "Kleinstes Maß"}}}}
RULE_FIELDS = lambda: {"quantity": QUANTITY, "parameter": {"type": "string", "x-semio-ui": {"widget": "text", "role": "value", **label("Parameter", "Parameter", description=("Id of the capability parameter the stock dimension is compared with.", "ID des Fähigkeitsparameters, mit dem das Rohteilmaß verglichen wird."))}}, "margin": metres("Margin", "Zugabe", ("Added to the parameter value before the comparison.", "Wird vor dem Vergleich zum Parameterwert addiert."))}
RULE_TAG = ("Rule type", "Regelart")
RULES = [
    branch("kind", "min", "At least", "Mindestens", RULE_FIELDS(), *RULE_TAG),
    branch("kind", "max", "At most", "Höchstens", RULE_FIELDS(), *RULE_TAG),
]
CATALOG_ID = {"type": "string", "x-semio-ui": {"widget": "text", "role": "value", **label("Catalog entry", "Katalogeintrag", description=("Id of the machine catalog entry the machine was created from; empty for a custom machine.", "ID des Maschinenkatalogeintrags, aus dem die Maschine angelegt wurde; leer bei einer eigenen Maschine."))}}


def nullable(node):
    types = node.get("type")
    if isinstance(types, str):
        node["type"] = [types, "null"]
    elif isinstance(types, list) and "null" not in types:
        node["type"] = types + ["null"]


def capabilities(items):
    items["properties"]["recipe"]["oneOf"] = RECIPE
    items["properties"]["rules"]["items"] = {"oneOf": RULES}


EDITS = {
    "🌱create-step": lambda d: d["properties"]["step"]["properties"]["measure"].__setitem__("oneOf", measure()),
    "📐replace-step-measure": lambda d: d["properties"]["newMeasure"].__setitem__("oneOf", measure()),
    "🏭create-machine": lambda d: (capabilities(d["properties"]["machine"]["properties"]["capabilities"]["items"]), d["properties"]["machine"]["properties"].setdefault("catalogId", CATALOG_ID)),
    "🔁replace-machine-capabilities": lambda d: capabilities(d["properties"]["newCapabilities"]["items"]),
    "🧊replace-stock-solid": lambda d: (d["properties"]["newSolid"].__setitem__("additionalProperties", False), d["properties"]["newSolid"]["properties"].__setitem__("target", {"$ref": IO_REF, **({"x-semio-ui": d["properties"]["newSolid"]["properties"]["target"]["x-semio-ui"]} if "x-semio-ui" in d["properties"]["newSolid"]["properties"]["target"] else {})})),
    "🧷change-step-origin": lambda d: nullable(d["properties"]["newOrigin"]),
    "⏱️change-cursor": lambda d: nullable(d["properties"]["newResolvedUpTo"]),
}


def schemas():
    for leaf, edit in EDITS.items():
        path = f"{MUTATIONS}/{leaf}/🧬️schema/🔣️.json"
        before = read(path)
        document = json.loads(before)
        edit(document)
        if document == json.loads(before):
            continue
        if json.dumps(json.loads(before), indent=2, ensure_ascii=False) + ("\n" if before.endswith("\n") else "") != before:
            raise SystemExit(f"[w2-s-e] {path} is hand-formatted; edit it by hand")
        write(path, before, json.dumps(document, indent=2, ensure_ascii=False) + ("\n" if before.endswith("\n") else ""))


def rust():
    path = f"{ARTIFACT}/🦀️.rs"
    before = read(path)
    anchor = '#[value(tag = "kind", rename_all = "camelCase")]\npub enum WorkingSolid {'
    if anchor in before:
        write(path, before, before.replace(anchor, '#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]\npub enum WorkingSolid {'))
    elif 'rename_all_fields = "camelCase")]\npub enum WorkingSolid {' not in before:
        raise SystemExit(f"[w2-s-e] {path}: WorkingSolid attribute block has an unexpected shape")


if __name__ == "__main__":
    rust()
    schemas()
