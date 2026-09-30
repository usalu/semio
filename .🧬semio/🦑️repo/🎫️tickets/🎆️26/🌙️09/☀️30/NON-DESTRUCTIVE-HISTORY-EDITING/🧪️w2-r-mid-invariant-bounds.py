#!/usr/bin/env python3
"""🧷️ W2-R-mid follow-up (coordinator decision "invariant refusals are negative witnesses", design §11): states in the fem
and remodel leaf payload schemas exactly the payload-intrinsic breaches their Rust diffs refuse as `mutation.invariant`.
Value ranges become hard keywords (`minimum`, `exclusiveMinimum`, `exclusiveMaximum`, `maximum`, `minItems`); cross-field
rules draft-07 cannot state are declared in the root `x-semio-invariant` (manifest `$defs/SchemaInvariants`) and named by the
refusing fixture's outcome (`"invariant": "<id>"`). `x-semio-ui` soft ranges are pulled inside the new hard bounds, and the
fem2d leaf descriptions stop claiming the schemas are structural only. Every file keeps its pretty (indent 2) layout.

Run: python3 🧪️w2-r-mid-invariant-bounds.py [--check]"""
import json
import os
import sys

REPO = "/Users/ueli/Documents/semio"
FEM2D = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets"
FEM3D = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets"
REMODEL = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
POSITIVE = {"exclusiveMinimum": 0}


def L(en, de):
    return {"en": en, "de": de}


def leaf(root, subset, name):
    return os.path.join(REPO, root, subset, "🧬️schema", "🧬️mutations", name, "🧬️schema", "🔣️.json") if subset else os.path.join(REPO, root, "🧬️schema", "🧬️mutations", name, "🧬️schema", "🔣️.json")


def outcome(root, subset, name, case):
    base = os.path.join(REPO, root, subset) if subset else os.path.join(REPO, root)
    return os.path.join(base, "🧫️fixtures", "🧬️mutations", name, case, "🎯️outcome", "🔣️.json")


REGION = [
    {"id": "region-outline-encloses-area", "description": L("The outline polygon encloses a non-zero area.", "Das Umrisspolygon umschließt eine Fläche ungleich null.")},
    {"id": "region-hole-encloses-area", "description": L("Every opening polygon encloses a non-zero area.", "Jedes Aussparungspolygon umschließt eine Fläche ungleich null.")},
    {"id": "region-holes-inside-outline", "description": L("Every opening vertex lies inside the outline or on its boundary.", "Jeder Eckpunkt einer Aussparung liegt innerhalb des Umrisses oder auf dessen Rand.")},
]
SOLID = [
    {"id": "solid-outline-encloses-area", "description": L("The footprint polygon encloses a non-zero area.", "Das Grundrisspolygon umschließt eine Fläche ungleich null.")},
    {"id": "solid-hole-encloses-area", "description": L("Every opening polygon encloses a non-zero area.", "Jedes Aussparungspolygon umschließt eine Fläche ungleich null.")},
    {"id": "solid-holes-inside-outline", "description": L("Every opening vertex lies strictly inside the footprint.", "Jeder Eckpunkt einer Aussparung liegt echt innerhalb des Grundrisses.")},
]
COMBINATION = [
    {"id": "combination-not-self-weighted", "description": L("No term weights the combination itself.", "Kein Summand gewichtet die Kombination selbst.")},
]
MATERIAL_2D = {"e": POSITIVE, "rho": POSITIVE, "nu": {"exclusiveMinimum": -1, "exclusiveMaximum": 0.5}}
MATERIAL_3D = {**MATERIAL_2D, "g": POSITIVE}
SECTION_2D = {"area": POSITIVE, "iy": POSITIVE}
SECTION_3D = {**SECTION_2D, "iz": POSITIVE, "j": POSITIVE}
REGION_BOUNDS = {"outline": {"minItems": 3}, "holes/items": {"minItems": 3}, "thickness": POSITIVE, "meshSize": POSITIVE}
SOLID_BOUNDS = {"outline": {"minItems": 3}, "holes/items": {"minItems": 3}, "height": POSITIVE, "layers": {"minimum": 1}, "meshSize": POSITIVE}
ANALYSIS = {"modalCount": {"minimum": 1}, "bucklingCount": {"minimum": 1}, "deformationScale": POSITIVE}
NU_UI = {"softMin": 0, "softMax": 0.49}

SCHEMAS = [
    (leaf(FEM2D, "🧱️material", "🌱️create-material"), "material", MATERIAL_2D, None, {"nu": NU_UI}),
    (leaf(FEM2D, "🧱️material", "🔁️replace-material"), "newMaterial", MATERIAL_2D, None, {"nu": NU_UI}),
    (leaf(FEM2D, "🕸️mesh", "📐️create-section"), "section", SECTION_2D, None, {}),
    (leaf(FEM2D, "🕸️mesh", "📏️replace-section"), "newSection", SECTION_2D, None, {}),
    (leaf(FEM2D, "🕸️mesh", "🗺️create-region"), "region", REGION_BOUNDS, REGION, {}),
    (leaf(FEM2D, "🕸️mesh", "🔄️replace-region"), "newRegion", REGION_BOUNDS, REGION, {}),
    (leaf(FEM2D, "📈️analysis", "🎛️update-analysis-settings"), "settings", ANALYSIS, None, {}),
    (leaf(FEM2D, "🏋️load", "🔗️create-combination"), None, {}, COMBINATION, {}),
    (leaf(FEM2D, "🏋️load", "🔁️replace-combination"), None, {}, COMBINATION, {}),
    (leaf(FEM3D, "🧱️material", "🌱️create-material"), "material", MATERIAL_3D, None, {"nu": NU_UI}),
    (leaf(FEM3D, "🧱️material", "🔁️replace-material"), "newMaterial", MATERIAL_3D, None, {"nu": NU_UI}),
    (leaf(FEM3D, "🕸️mesh", "📐️create-section"), "section", SECTION_3D, None, {}),
    (leaf(FEM3D, "🕸️mesh", "📏️replace-section"), "newSection", SECTION_3D, None, {}),
    (leaf(FEM3D, "🕸️mesh", "🧊️create-solid"), "solid", SOLID_BOUNDS, SOLID, {}),
    (leaf(FEM3D, "🕸️mesh", "🔄️replace-solid"), "newSolid", SOLID_BOUNDS, SOLID, {}),
    (leaf(FEM3D, "📈️analysis", "🎛️update-analysis-settings"), "settings", ANALYSIS, None, {}),
    (leaf(REMODEL, None, "🌐update-geo-params"), "params", {"gsdM": POSITIVE, "dsmCellM": POSITIVE, "dtmFilterRadiusM": POSITIVE, "orthoMaxPx": {"minimum": 1}}, None, {}),
    (leaf(REMODEL, None, "🌠update-feature-params"), "params", {"targetCount": {"minimum": 1}, "edgeThreshold": {"minimum": 0}}, None, {}),
    (leaf(REMODEL, None, "🥣update-ingest-params"), "params", {"frameSampleStride": {"minimum": 1}, "maxFrames": {"minimum": 1}, "minSharpness": {"minimum": 0}}, None, {}),
    (leaf(REMODEL, None, "🪢update-match-params"), "params", {"ratioTest": {"exclusiveMinimum": 0, "maximum": 1}}, None, {}),
]
OUTCOMES = [
    (outcome(FEM2D, "🕸️mesh", "🗺️create-region", "🕳️denies-loose-hole-d9efa1"), {"invariant": "region-holes-inside-outline"}),
    (outcome(FEM2D, "🏋️load", "🔁️replace-combination", "👻️self-term-0f54d1"), {"invariant": "combination-not-self-weighted"}),
    (outcome(FEM3D, "🕸️mesh", "🧊️create-solid", "📐️sliver-outline-316a7c"), {"invariant": "solid-outline-encloses-area"}),
    (outcome(REMODEL, None, "⛓️create-rig-extrinsic", "🚫️refuses-a-rig-cb71ba"), {"code": "mutation.target-missing"}),
    (outcome(REMODEL, None, "🌱create-stream", "👻️rejects-a-stream-aac5c2"), {"code": "mutation.target-missing", "path": ["orbit-cam-absent"]}),
    (outcome(REMODEL, None, "➕add-stream-frame", "🎬️refuses-a-frame-81beea"), {"code": "mutation.target-missing"}),
]
STRUCTURAL = "Structural only — the plausibility and geometry bounds this kind enforces live in the SNAPSHOT schema, because they are properties of a valid document rather than of a well-formed payload; each is annotated there with the `mutations::guards` function that raises them."
STATED = "Its hard bounds and `x-semio-invariant` entries state exactly the payload-intrinsic breaches `mutations::guards` refuses as `mutation.invariant`, so every such payload fails this schema; refusals that depend on the base (a missing, duplicate or still-referenced target) belong to the diff alone."


def fail(message):
    raise SystemExit("🚫️ " + message)


def with_keys(node, additions, before="x-semio-ui"):
    """🧩️ `node` with every addition set in place (an existing key keeps its position) and new keys placed before `before`."""
    result = {}
    pending = {key: value for key, value in additions.items() if key not in node}
    for key, value in node.items():
        if key == before:
            result.update(pending)
            pending = {}
        result[key] = additions.get(key, value)
    result.update(pending)
    return result


def at(node, path, label):
    for segment in path.split("/"):
        if segment == "items":
            node = node.get("items")
        else:
            node = node.get("properties", {}).get(segment)
        if not isinstance(node, dict):
            fail("%s: no node at %s" % (label, path))
    return node


def bounded(path, schema, record, bounds, invariants, ui):
    if STRUCTURAL in schema.get("description", ""):
        schema["description"] = schema["description"].replace(STRUCTURAL, STATED)
    for relative, keys in bounds.items():
        parent_path, _, key = ("%s/%s" % (record, relative)).rpartition("/")
        parent = at(schema, parent_path, path) if parent_path else schema
        container = parent if key == "items" else parent["properties"]
        target = container[key]
        if target.get("type") not in ("number", "integer", "array", ["number", "null"], ["integer", "null"]) and "items" not in relative:
            fail("%s: %s is no number/array (%s)" % (path, relative, target.get("type")))
        container[key] = with_keys(target, keys)
    for relative, keys in ui.items():
        target = at(schema, "%s/%s" % (record, relative), path)
        target["x-semio-ui"] = {**target["x-semio-ui"], **keys}
    if invariants:
        schema = with_keys(schema, {"x-semio-invariant": invariants}, before="properties")
    return schema


def rewrite(path, transform, check):
    text = open(path, encoding="utf-8").read()
    document = json.loads(text)
    if json.dumps(document, indent=2, ensure_ascii=False) + "\n" != text:
        fail("%s is not in pretty (indent 2) layout" % path)
    written = json.dumps(transform(document), indent=2, ensure_ascii=False) + "\n"
    if written != text and not check:
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(written)
    return written != text


def main():
    check = "--check" in sys.argv
    changed = 0
    for path, record, bounds, invariants, ui in SCHEMAS:
        changed += rewrite(path, lambda schema, record=record, bounds=bounds, invariants=invariants, ui=ui, path=path: bounded(path, schema, record, bounds, invariants, ui), check)
    for directory, names, files in os.walk(os.path.join(REPO, FEM2D)):
        if "🧫️fixtures" not in directory and directory.endswith("/🧬️schema") and "🔣️.json" in files and "/🧬️mutations/" in directory:
            path = os.path.join(directory, "🔣️.json")
            if STRUCTURAL in open(path, encoding="utf-8").read():
                changed += rewrite(path, lambda schema: {**schema, "description": schema["description"].replace(STRUCTURAL, STATED)}, check)
    for path, update in OUTCOMES:
        changed += rewrite(path, lambda document, update=update: with_keys(document, update, before="path") if "invariant" in update else {**document, **update}, check)
    print("%d file(s) %s" % (changed, "would change" if check else "changed"))


if __name__ == "__main__":
    sys.exit(main())
