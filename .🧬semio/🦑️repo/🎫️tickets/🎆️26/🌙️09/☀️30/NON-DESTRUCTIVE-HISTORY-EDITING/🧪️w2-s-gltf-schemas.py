#!/usr/bin/env python3
"""🧊️ W2-S glTF parity: one shared snapshot and diff schema document, phase-tagged leaf payload schemas, 🦠️mutation fixtures.

Every schema written here describes the wire `value_derive::ToValue` emits for the Rust types it names:
- `📸️snapshot/🔣️.json` — `GltfSnapshot` and every record as `$defs` (hand-rolled ToValue impls included: camera, morph
  target, attribute maps, componentType codes).
- `🔺️diff/🔣️.json` — `GltfDiff` with the index-keyed collection triples, the `{state}` JSON presence and the records by
  `$ref` into the snapshot document.
- every leaf `🧬️schema/🔣️.json` — the `#[value(tag = "phase", content = "value")]` enum: `apply` carries the leaf payload
  (`$defs`), `restore` carries `$ref diff.json` (hidden input).
- every `🧬️operation/🔣️.json` fixture becomes `🦠️mutation/🔣️.json` holding the aggregate wire value.

Run from the repository root: `.venv/bin/python <ticket>/🧪️w2-s-gltf-schemas.py [--write]` (dry run without `--write`).
"""
import json
import os
import re
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
TICKET = Path(__file__).resolve().parent
ANY = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any"
SCHEMA = ANY / "🧬️schema"
LEAVES = SCHEMA / "🧬️mutations"
FIXTURES = ANY / "🧫️fixtures/🧬️mutations"
BASE = "https://json.schemas.assets.semio-tech.com/s/stdio/gltf/2.0/any/"
SNAP = BASE + "snapshot.json"
DIFF = BASE + "diff.json"
DRAFT = "http://json-schema.org/draft-07/schema#"
WRITE = "--write" in sys.argv
HARVEST = json.loads((TICKET / "🧪️w2-s-gltf-labels.json").read_text())


def text(value):
    return json.dumps(value, indent=2, ensure_ascii=False) + "\n"


def emit(path, value):
    body = text(value)
    if not path.exists() or path.read_text() != body:
        print(("write " if WRITE else "would write ") + str(path.relative_to(ROOT)))
        if WRITE:
            path.write_text(body)


JSONSCHEMA, RUST, TYPESCRIPT, GRAPHQL, PROTOBUF = "🔣️jsonschema", "🦀️rust", "🟦️typescript", "🔗️graphql", "🛰️protobuf"
FORMATS = {
    "GltfComponentType": [JSONSCHEMA, RUST, TYPESCRIPT],
    "GltfAccessorType": [JSONSCHEMA, RUST, TYPESCRIPT],
    "GltfCameraProjection": [JSONSCHEMA, RUST, TYPESCRIPT],
    "GltfAlphaMode": [JSONSCHEMA, RUST, TYPESCRIPT, GRAPHQL],
    "GltfAnimationPath": [JSONSCHEMA, RUST, TYPESCRIPT, GRAPHQL],
    "GltfInterpolation": [JSONSCHEMA, RUST, TYPESCRIPT, GRAPHQL],
    "GltfSourceForm": [JSONSCHEMA, RUST, TYPESCRIPT, GRAPHQL],
    "GltfJson": [JSONSCHEMA, RUST, TYPESCRIPT, GRAPHQL],
    "GltfJsonPresence": [JSONSCHEMA, TYPESCRIPT],
    "GltfTouchedRegion": [JSONSCHEMA, TYPESCRIPT, GRAPHQL],
    "GltfDiffDerivation": [JSONSCHEMA, TYPESCRIPT, GRAPHQL, PROTOBUF],
}
"""🏷️ Exports that exist in fewer formats than their scope provides, by design (an enum is a string field in protobuf, a JSON
helper has no Rust type): each says so with `x-semio-formats`, which `schema check` holds in both directions."""


def formats(defs):
    return {name: ({**node, "x-semio-formats": FORMATS[name]} if name in FORMATS else node) for name, node in sorted(defs.items())}


def lab(en, de, den=None, dde=None, **extra):
    ui = {"label": {"en": en, "de": de}}
    if den is not None:
        ui["description"] = {"en": den, "de": dde}
    ui.update(extra)
    return ui


def ui_of(key, fallback=None):
    found = HARVEST.get(key)
    if found is None and fallback is None:
        raise SystemExit(f"no label for {key}")
    return dict(found if found is not None else fallback)


def ref(name, doc=None):
    return {"$ref": f"{doc}#/$defs/{name}" if doc else f"#/$defs/{name}"}


def with_ui(schema, ui):
    return {**schema, "x-semio-ui": ui}


INDEX = {"type": "integer", "minimum": 0}
NUM = {"type": "number"}
STR = {"type": "string"}
BOOL = {"type": "boolean"}


def nullable(schema):
    return {"anyOf": [schema, {"type": "null"}]}


def vec(n):
    return {"type": "array", "items": NUM, "minItems": n, "maxItems": n}


def arr(items):
    return {"type": "array", "items": items}


def record(name, fields, required, doc=None):
    """🧱️ A struct wire: `fields` is [(wire name, schema, ui key or ui)], `required` the always-emitted wire names."""
    properties = {}
    for wire, schema, ui in fields:
        properties[wire] = with_ui(schema, ui if isinstance(ui, dict) else ui_of(ui))
    node = {"type": "object", "additionalProperties": False, **({"required": required} if required else {}), "properties": properties}
    if doc:
        node = {"description": doc, **node}
    return node


EXT = lambda owner: ("extensions", ref("GltfJson"), ui_of(f"{owner}.extensions", HARVEST["GltfNode.extensions"]))
XTR = lambda owner: ("extras", ref("GltfJson"), ui_of(f"{owner}.extras", HARVEST["GltfNode.extras"]))
NAME = ("name", STR, lab("Name", "Name"))
COMPONENT_CODES = [5120, 5121, 5122, 5123, 5125, 5126]
COMPONENT_UI = lab("Component type", "Komponententyp", "5120 byte, 5121 unsigned byte, 5122 short, 5123 unsigned short, 5125 unsigned int, 5126 float.", "5120 Byte, 5121 Byte ohne Vorzeichen, 5122 Short, 5123 Short ohne Vorzeichen, 5125 Integer ohne Vorzeichen, 5126 Gleitkommazahl.", snaps=COMPONENT_CODES)
ACCESSOR_TYPE_OPTIONS = {"SCALAR": lab("Scalar", "Skalar")["label"], "VEC2": lab("2D vector", "2D-Vektor")["label"], "VEC3": lab("3D vector", "3D-Vektor")["label"], "VEC4": lab("4D vector", "4D-Vektor")["label"], "MAT2": lab("2×2 matrix", "2×2-Matrix")["label"], "MAT3": lab("3×3 matrix", "3×3-Matrix")["label"], "MAT4": lab("4×4 matrix", "4×4-Matrix")["label"]}
TEXTURE_INFO = lambda name, extra: record(name, [("index", INDEX, f"{name}.index"), ("texCoord", INDEX, f"{name}.texCoord"), *extra, EXT(name), XTR(name)], ["index"])
PERSPECTIVE_FIELDS = [("aspectRatio", NUM, lab("Aspect ratio", "Seitenverhältnis", "Width over height of the viewport.", "Breite zu Höhe des Sichtfensters.")), ("yfov", NUM, lab("Vertical field of view", "Vertikaler Öffnungswinkel", "In radians.", "Im Bogenmaß.")), ("zfar", NUM, lab("Far clipping plane", "Hintere Schnittebene")), ("znear", NUM, lab("Near clipping plane", "Vordere Schnittebene"))]
ORTHOGRAPHIC_FIELDS = [("xmag", NUM, lab("Horizontal magnification", "Horizontale Vergrößerung")), ("ymag", NUM, lab("Vertical magnification", "Vertikale Vergrößerung")), ("zfar", NUM, lab("Far clipping plane", "Hintere Schnittebene")), ("znear", NUM, lab("Near clipping plane", "Vordere Schnittebene"))]


def projection_branches(extra_fields):
    """📷️ The `{"type": "perspective", "perspective": {…}}` / orthographic wire of `GltfCameraProjection`, optionally flattened with the camera's own members."""
    branches = []
    for kind, target, en, de in (("perspective", "GltfPerspective", "Perspective projection", "Perspektivische Projektion"), ("orthographic", "GltfOrthographic", "Orthographic projection", "Orthografische Projektion")):
        fields = [("type", {"const": kind}, lab("Projection type", "Projektionsart")), (kind, ref(target), lab(en, de)), *extra_fields]
        node = record(target, fields, ["type", kind])
        branches.append(with_ui(node, lab(en, de)))
    return {"oneOf": branches}


def snapshot_document():
    """📸️ `GltfSnapshot` and every record it reaches, keyed by the Rust type name."""
    defs = {
        "GltfJson": {"description": "Any JSON value: a glTF `extras`/`extensions` slot (GltfJson).", "anyOf": [{"type": "null"}, BOOL, NUM, STR, arr(ref("GltfJson")), {"type": "object", "additionalProperties": ref("GltfJson")}]},
        "GltfSourceForm": with_ui({"type": "string", "enum": ["json", "glb"]}, ui_of("GltfSourceForm")),
        "GltfComponentType": {"description": "accessor.componentType as its numeric glTF code.", "type": "integer", "enum": COMPONENT_CODES, "minimum": 5120, "maximum": 5126},
        "GltfAccessorType": with_ui({"type": "string", "enum": list(ACCESSOR_TYPE_OPTIONS)}, {"options": ACCESSOR_TYPE_OPTIONS}),
        "GltfAlphaMode": with_ui({"type": "string", "enum": ["OPAQUE", "MASK", "BLEND"]}, ui_of("GltfAlphaMode")),
        "GltfAnimationPath": with_ui({"type": "string", "enum": ["translation", "rotation", "scale", "weights"]}, ui_of("GltfAnimationPath")),
        "GltfInterpolation": with_ui({"type": "string", "enum": ["LINEAR", "STEP", "CUBICSPLINE"]}, ui_of("GltfInterpolation")),
        "GltfAsset": record("GltfAsset", [("version", STR, lab("glTF version", "glTF-Version")), ("generator", STR, "GltfAssetDiff.generator"), ("copyright", STR, "GltfAssetDiff.copyright"), ("minVersion", STR, "GltfAssetDiff.minVersion"), EXT("GltfAssetDiff"), XTR("GltfAssetDiff")], ["version"]),
        "GltfScene": record("GltfScene", [("nodes", arr(INDEX), "GltfScene.nodes"), NAME, EXT("GltfScene"), XTR("GltfScene")], []),
        "GltfNode": record("GltfNode", [("children", arr(INDEX), "GltfNode.children"), ("mesh", INDEX, "GltfNode.mesh"), ("camera", INDEX, "GltfNode.camera"), ("skin", INDEX, "GltfNode.skin"), ("matrix", vec(16), "GltfNode.matrix"), ("translation", vec(3), "GltfNode.translation"), ("rotation", vec(4), "GltfNode.rotation"), ("scale", vec(3), "GltfNode.scale"), ("weights", arr(NUM), "GltfNode.weights"), NAME, EXT("GltfNode"), XTR("GltfNode")], []),
        "GltfMorphTarget": {"description": "Attribute semantic to accessor index (GltfMorphTarget).", "type": "object", "additionalProperties": INDEX},
        "GltfPrimitive": record("GltfPrimitive", [("attributes", {"type": "object", "additionalProperties": INDEX}, "GltfPrimitive.attributes"), ("indices", INDEX, "GltfPrimitive.indices"), ("material", INDEX, "GltfPrimitive.material"), ("mode", INDEX, "GltfPrimitive.mode"), ("targets", arr(ref("GltfMorphTarget")), "GltfPrimitive.targets"), EXT("GltfPrimitive"), XTR("GltfPrimitive")], ["attributes"]),
        "GltfMesh": record("GltfMesh", [("primitives", arr(ref("GltfPrimitive")), "GltfMesh.primitives"), ("weights", arr(NUM), "GltfMesh.weights"), NAME, EXT("GltfMesh"), XTR("GltfMesh")], []),
        "GltfSparseIndices": record("GltfSparseIndices", [("bufferView", INDEX, "GltfSparseIndices.bufferView"), ("byteOffset", INDEX, "GltfSparseIndices.byteOffset"), ("componentType", ref("GltfComponentType"), COMPONENT_UI)], ["bufferView", "componentType"]),
        "GltfSparseValues": record("GltfSparseValues", [("bufferView", INDEX, "GltfSparseValues.bufferView"), ("byteOffset", INDEX, "GltfSparseValues.byteOffset")], ["bufferView"]),
        "GltfSparseAccessor": record("GltfSparseAccessor", [("count", INDEX, "GltfSparseAccessor.count"), ("indices", ref("GltfSparseIndices"), "GltfSparseAccessor.indices"), ("values", ref("GltfSparseValues"), "GltfSparseAccessor.values")], ["count", "indices", "values"]),
        "GltfAccessor": record("GltfAccessor", [("bufferView", INDEX, "GltfAccessor.bufferView"), ("byteOffset", INDEX, "GltfAccessor.byteOffset"), ("componentType", ref("GltfComponentType"), COMPONENT_UI), ("normalized", BOOL, "GltfAccessor.normalized"), ("count", INDEX, "GltfAccessor.count"), ("type", ref("GltfAccessorType"), "GltfAccessor.type"), ("max", arr(NUM), "GltfAccessor.max"), ("min", arr(NUM), "GltfAccessor.min"), ("sparse", ref("GltfSparseAccessor"), "GltfAccessor.sparse"), NAME, EXT("GltfAccessor"), XTR("GltfAccessor")], ["componentType", "count", "type"]),
        "GltfBufferView": record("GltfBufferView", [("buffer", INDEX, "GltfBufferView.buffer"), ("byteOffset", INDEX, "GltfBufferView.byteOffset"), ("byteLength", INDEX, "GltfBufferView.byteLength"), ("byteStride", INDEX, "GltfBufferView.byteStride"), ("target", INDEX, "GltfBufferView.target"), NAME, EXT("GltfBufferView"), XTR("GltfBufferView")], ["buffer", "byteLength"]),
        "GltfBuffer": record("GltfBuffer", [("byteLength", INDEX, "GltfBuffer.byteLength"), ("uri", STR, "GltfBuffer.uri"), NAME, EXT("GltfBuffer"), XTR("GltfBuffer")], ["byteLength"]),
        "GltfTextureInfo": TEXTURE_INFO("GltfTextureInfo", []),
        "GltfNormalTextureInfo": TEXTURE_INFO("GltfNormalTextureInfo", [("scale", NUM, "GltfNormalTextureInfo.scale")]),
        "GltfOcclusionTextureInfo": TEXTURE_INFO("GltfOcclusionTextureInfo", [("strength", NUM, "GltfOcclusionTextureInfo.strength")]),
        "GltfPbrMetallicRoughness": record("GltfPbrMetallicRoughness", [("baseColorFactor", vec(4), "GltfPbrMetallicRoughness.baseColorFactor"), ("baseColorTexture", ref("GltfTextureInfo"), "GltfPbrMetallicRoughness.baseColorTexture"), ("metallicFactor", NUM, "GltfPbrMetallicRoughness.metallicFactor"), ("roughnessFactor", NUM, "GltfPbrMetallicRoughness.roughnessFactor"), ("metallicRoughnessTexture", ref("GltfTextureInfo"), "GltfPbrMetallicRoughness.metallicRoughnessTexture"), EXT("GltfPbrMetallicRoughness"), XTR("GltfPbrMetallicRoughness")], []),
        "GltfMaterial": record("GltfMaterial", [NAME, ("pbrMetallicRoughness", ref("GltfPbrMetallicRoughness"), "GltfMaterial.pbrMetallicRoughness"), ("normalTexture", ref("GltfNormalTextureInfo"), "GltfMaterial.normalTexture"), ("occlusionTexture", ref("GltfOcclusionTextureInfo"), "GltfMaterial.occlusionTexture"), ("emissiveTexture", ref("GltfTextureInfo"), "GltfMaterial.emissiveTexture"), ("emissiveFactor", vec(3), "GltfMaterial.emissiveFactor"), ("alphaMode", ref("GltfAlphaMode"), "GltfMaterial.alphaMode"), ("alphaCutoff", NUM, "GltfMaterial.alphaCutoff"), ("doubleSided", BOOL, "GltfMaterial.doubleSided"), EXT("GltfMaterial"), XTR("GltfMaterial")], []),
        "GltfTexture": record("GltfTexture", [("sampler", INDEX, "GltfTexture.sampler"), ("source", INDEX, "GltfTexture.source"), NAME, EXT("GltfTexture"), XTR("GltfTexture")], []),
        "GltfImage": record("GltfImage", [("uri", STR, "GltfImage.uri"), ("mimeType", STR, "GltfImage.mimeType"), ("bufferView", INDEX, "GltfImage.bufferView"), NAME, EXT("GltfImage"), XTR("GltfImage")], []),
        "GltfSampler": record("GltfSampler", [("magFilter", INDEX, "GltfSampler.magFilter"), ("minFilter", INDEX, "GltfSampler.minFilter"), ("wrapS", INDEX, "GltfSampler.wrapS"), ("wrapT", INDEX, "GltfSampler.wrapT"), NAME, EXT("GltfSampler"), XTR("GltfSampler")], []),
        "GltfSkin": record("GltfSkin", [("inverseBindMatrices", INDEX, "GltfSkin.inverseBindMatrices"), ("skeleton", INDEX, "GltfSkin.skeleton"), ("joints", arr(INDEX), "GltfSkin.joints"), NAME, EXT("GltfSkin"), XTR("GltfSkin")], []),
        "GltfAnimationChannelTarget": record("GltfAnimationChannelTarget", [("node", INDEX, "GltfAnimationChannelTarget.node"), ("path", ref("GltfAnimationPath"), "GltfAnimationChannelTarget.path"), EXT("GltfAnimationChannelTarget"), XTR("GltfAnimationChannelTarget")], ["path"]),
        "GltfAnimationChannel": record("GltfAnimationChannel", [("sampler", INDEX, "GltfAnimationChannel.sampler"), ("target", ref("GltfAnimationChannelTarget"), "GltfAnimationChannel.target"), EXT("GltfAnimationChannel"), XTR("GltfAnimationChannel")], ["sampler", "target"]),
        "GltfAnimationSampler": record("GltfAnimationSampler", [("input", INDEX, "GltfAnimationSampler.input"), ("interpolation", ref("GltfInterpolation"), "GltfAnimationSampler.interpolation"), ("output", INDEX, "GltfAnimationSampler.output"), EXT("GltfAnimationSampler"), XTR("GltfAnimationSampler")], ["input", "output"]),
        "GltfAnimation": record("GltfAnimation", [("channels", arr(ref("GltfAnimationChannel")), "GltfAnimation.channels"), ("samplers", arr(ref("GltfAnimationSampler")), "GltfAnimation.samplers"), NAME, EXT("GltfAnimation"), XTR("GltfAnimation")], []),
        "GltfPerspective": record("GltfPerspective", [*PERSPECTIVE_FIELDS, EXT("GltfCamera"), XTR("GltfCamera")], ["yfov", "znear"]),
        "GltfOrthographic": record("GltfOrthographic", [*ORTHOGRAPHIC_FIELDS, EXT("GltfCamera"), XTR("GltfCamera")], ["xmag", "ymag", "zfar", "znear"]),
        "GltfCameraProjection": projection_branches([]),
        "GltfCamera": projection_branches([NAME, EXT("GltfCamera"), XTR("GltfCamera")]),
    }
    document_fields = [
        ("asset", ref("GltfAsset"), lab("Asset metadata", "Asset-Metadaten")),
        ("scene", INDEX, ui_of("GltfDiff.scene")),
        ("scenes", arr(ref("GltfScene")), ui_of("GltfDiff.scenes")),
        ("nodes", arr(ref("GltfNode")), ui_of("GltfDiff.nodes")),
        ("meshes", arr(ref("GltfMesh")), ui_of("GltfDiff.meshes")),
        ("accessors", arr(ref("GltfAccessor")), ui_of("GltfDiff.accessors")),
        ("bufferViews", arr(ref("GltfBufferView")), ui_of("GltfDiff.bufferViews")),
        ("buffers", arr(ref("GltfBuffer")), ui_of("GltfDiff.buffers")),
        ("materials", arr(ref("GltfMaterial")), ui_of("GltfDiff.materials")),
        ("textures", arr(ref("GltfTexture")), ui_of("GltfDiff.textures")),
        ("images", arr(ref("GltfImage")), ui_of("GltfDiff.images")),
        ("samplers", arr(ref("GltfSampler")), ui_of("GltfDiff.samplers")),
        ("skins", arr(ref("GltfSkin")), ui_of("GltfDiff.skins")),
        ("animations", arr(ref("GltfAnimation")), ui_of("GltfDiff.animations")),
        ("cameras", arr(ref("GltfCamera")), ui_of("GltfDiff.cameras")),
        ("extensionsUsed", arr(STR), ui_of("GltfDiff.extensionsUsed")),
        ("extensionsRequired", arr(STR), ui_of("GltfDiff.extensionsRequired")),
        ("extensions", ref("GltfJson"), ui_of("GltfDiff.extensions")),
        ("extras", ref("GltfJson"), ui_of("GltfDiff.extras")),
    ]
    defs["GltfDocument"] = record("GltfDocument", document_fields, ["asset"])
    state = {"x-semio-state": "artifact"}
    return {
        "$schema": DRAFT,
        "$id": SNAP,
        "title": "GltfSnapshot",
        "description": "Persisted stdio.gltf snapshot: the typed glTF 2.0 document plus the resolved bytes of every document buffer.",
        "type": "object",
        "additionalProperties": False,
        "required": ["schema", "document", "buffers", "sourceForm"],
        "properties": {
            "schema": {"type": "string", **state, "x-semio-ui": lab("Schema", "Schema", "Identifier of the snapshot schema this document follows.", "Kennung des Snapshot-Schemas, dem dieses Dokument folgt.")},
            "document": {**ref("GltfDocument"), **state, "x-semio-ui": lab("glTF document", "glTF-Dokument")},
            "buffers": {"type": "array", "items": arr({"type": "integer", "minimum": 0, "maximum": 255}), **state, "description": "Resolved raw payload bytes, index-aligned with document.buffers.", "x-semio-ui": lab("Buffer bytes", "Pufferbytes")},
            "sourceForm": {**ref("GltfSourceForm"), **state, "x-semio-ui": lab("Source form", "Quellformat")},
        },
        "$defs": formats(defs),
    }


def diff_document():
    """🔺️ `GltfDiff`: sparse per-field slots, `{state}` JSON presence, index-keyed collection triples over snapshot records."""
    rec = lambda name: ref(name, SNAP)
    presence = ref("GltfJsonPresence")
    opt = nullable

    def collection(item, change):
        return {
            "type": "object",
            "additionalProperties": False,
            "properties": {
                "removed": with_ui(arr(INDEX), ui_of("Collection.removed")),
                "modified": with_ui(arr({"type": "object", "additionalProperties": False, "required": ["index", "diff"], "properties": {"index": with_ui(INDEX, lab("Index", "Index")), "diff": with_ui(change, ui_of("Collection.diff"))}}), ui_of("Collection.modified")),
                "added": with_ui(arr({"type": "object", "additionalProperties": False, "required": ["index", "item"], "properties": {"index": with_ui(INDEX, lab("Index", "Index")), "item": with_ui(item, lab("Item", "Eintrag"))}}), ui_of("Collection.added")),
            },
        }

    defs = {
        "GltfJsonPresence": {"description": "`Option<Option<GltfJson>>` on the wire: absent clears the slot, present sets it (a present JSON null stays distinct).", "oneOf": [record("GltfJsonPresence", [("state", {"const": "absent"}, lab("Presence", "Vorhandensein"))], ["state"]), record("GltfJsonPresence", [("state", {"const": "present"}, lab("Presence", "Vorhandensein")), ("value", rec("GltfJson"), lab("Value", "Wert"))], ["state", "value"])]},
        "GltfAssetDiff": record("GltfAssetDiff", [("version", STR, lab("glTF version", "glTF-Version")), ("generator", opt(STR), "GltfAssetDiff.generator"), ("copyright", opt(STR), "GltfAssetDiff.copyright"), ("minVersion", opt(STR), "GltfAssetDiff.minVersion"), ("extensions", presence, "GltfAssetDiff.extensions"), ("extras", presence, "GltfAssetDiff.extras")], []),
        "GltfSceneDiff": record("GltfSceneDiff", [("nodes", arr(INDEX), "GltfSceneDiff.nodes"), ("name", opt(STR), lab("Name", "Name")), ("extensions", presence, "GltfSceneDiff.extensions"), ("extras", presence, "GltfSceneDiff.extras")], []),
        "GltfNodeDiff": record("GltfNodeDiff", [("children", arr(INDEX), "GltfNodeDiff.children"), ("mesh", opt(INDEX), "GltfNodeDiff.mesh"), ("camera", opt(INDEX), "GltfNodeDiff.camera"), ("skin", opt(INDEX), "GltfNodeDiff.skin"), ("matrix", opt(vec(16)), "GltfNodeDiff.matrix"), ("translation", opt(vec(3)), "GltfNodeDiff.translation"), ("rotation", opt(vec(4)), "GltfNodeDiff.rotation"), ("scale", opt(vec(3)), "GltfNodeDiff.scale"), ("weights", arr(NUM), "GltfNodeDiff.weights"), ("name", opt(STR), lab("Name", "Name")), ("extensions", presence, "GltfNodeDiff.extensions"), ("extras", presence, "GltfNodeDiff.extras")], []),
        "GltfMeshDiff": record("GltfMeshDiff", [("primitives", arr(rec("GltfPrimitive")), "GltfMeshDiff.primitives"), ("weights", arr(NUM), "GltfMeshDiff.weights"), ("name", opt(STR), lab("Name", "Name")), ("extensions", presence, "GltfMeshDiff.extensions"), ("extras", presence, "GltfMeshDiff.extras")], []),
        "GltfAccessorDiff": record("GltfAccessorDiff", [("bufferView", opt(INDEX), "GltfAccessorDiff.bufferView"), ("byteOffset", INDEX, "GltfAccessorDiff.byteOffset"), ("componentType", rec("GltfComponentType"), COMPONENT_UI), ("normalized", BOOL, "GltfAccessorDiff.normalized"), ("count", INDEX, "GltfAccessorDiff.count"), ("kind", rec("GltfAccessorType"), "GltfAccessorDiff.kind"), ("max", opt(arr(NUM)), "GltfAccessorDiff.max"), ("min", opt(arr(NUM)), "GltfAccessorDiff.min"), ("sparse", opt(rec("GltfSparseAccessor")), "GltfAccessorDiff.sparse"), ("name", opt(STR), lab("Name", "Name")), ("extensions", presence, "GltfAccessorDiff.extensions"), ("extras", presence, "GltfAccessorDiff.extras")], []),
        "GltfMaterialDiff": record("GltfMaterialDiff", [("name", opt(STR), lab("Name", "Name")), ("pbrMetallicRoughness", opt(rec("GltfPbrMetallicRoughness")), "GltfMaterialDiff.pbrMetallicRoughness"), ("normalTexture", opt(rec("GltfNormalTextureInfo")), "GltfMaterialDiff.normalTexture"), ("occlusionTexture", opt(rec("GltfOcclusionTextureInfo")), "GltfMaterialDiff.occlusionTexture"), ("emissiveTexture", opt(rec("GltfTextureInfo")), "GltfMaterialDiff.emissiveTexture"), ("emissiveFactor", vec(3), "GltfMaterialDiff.emissiveFactor"), ("alphaMode", rec("GltfAlphaMode"), "GltfMaterialDiff.alphaMode"), ("alphaCutoff", NUM, "GltfMaterialDiff.alphaCutoff"), ("doubleSided", BOOL, "GltfMaterialDiff.doubleSided"), ("extensions", presence, "GltfMaterial.extensions"), ("extras", presence, "GltfMaterial.extras")], []),
        "GltfBufferDiff": record("GltfBufferDiff", [("byteLength", INDEX, "GltfBufferDiff.byteLength"), ("uri", opt(STR), "GltfBufferDiff.uri"), ("name", opt(STR), lab("Name", "Name")), ("extensions", presence, "GltfBufferDiff.extensions"), ("extras", presence, "GltfBufferDiff.extras")], []),
        "GltfScenesDiff": collection(rec("GltfScene"), ref("GltfSceneDiff")),
        "GltfNodesDiff": collection(rec("GltfNode"), ref("GltfNodeDiff")),
        "GltfMeshesDiff": collection(rec("GltfMesh"), ref("GltfMeshDiff")),
        "GltfAccessorsDiff": collection(rec("GltfAccessor"), ref("GltfAccessorDiff")),
        "GltfMaterialsDiff": collection(rec("GltfMaterial"), ref("GltfMaterialDiff")),
        "GltfBuffersDiff": collection(rec("GltfBuffer"), ref("GltfBufferDiff")),
        "GltfBufferViewsDiff": collection(rec("GltfBufferView"), rec("GltfBufferView")),
        "GltfBufferBytesDiff": collection(arr({"type": "integer", "minimum": 0, "maximum": 255}), arr({"type": "integer", "minimum": 0, "maximum": 255})),
        "GltfTexturesDiff": collection(rec("GltfTexture"), rec("GltfTexture")),
        "GltfImagesDiff": collection(rec("GltfImage"), rec("GltfImage")),
        "GltfSamplersDiff": collection(rec("GltfSampler"), rec("GltfSampler")),
        "GltfSkinsDiff": collection(rec("GltfSkin"), rec("GltfSkin")),
        "GltfAnimationsDiff": collection(rec("GltfAnimation"), rec("GltfAnimation")),
        "GltfCamerasDiff": collection(rec("GltfCamera"), rec("GltfCamera")),
        "GltfTouchedRegion": {"type": "string", "enum": ["asset", "scene", "scenes", "nodes", "meshes", "accessors", "bufferViews", "buffers", "bufferBytes", "materials", "textures", "images", "samplers", "skins", "animations", "cameras", "extensionsUsed", "extensionsRequired", "extensions", "extras", "sourceForm"]},
        "GltfDiffDerivation": {"type": "object", "additionalProperties": False, "required": ["forward", "inverse", "touchedPaths", "touchedRegions"], "properties": {"forward": {"$ref": "#"}, "inverse": {"$ref": "#"}, "touchedPaths": {"type": "array", "items": STR, "uniqueItems": True}, "touchedRegions": {"type": "array", "items": ref("GltfTouchedRegion"), "uniqueItems": True}}},
    }
    collections = [("scenes", "GltfScenesDiff"), ("nodes", "GltfNodesDiff"), ("meshes", "GltfMeshesDiff"), ("accessors", "GltfAccessorsDiff"), ("bufferViews", "GltfBufferViewsDiff"), ("buffers", "GltfBuffersDiff"), ("bufferBytes", "GltfBufferBytesDiff"), ("materials", "GltfMaterialsDiff"), ("textures", "GltfTexturesDiff"), ("images", "GltfImagesDiff"), ("samplers", "GltfSamplersDiff"), ("skins", "GltfSkinsDiff"), ("animations", "GltfAnimationsDiff"), ("cameras", "GltfCamerasDiff")]
    fields = [("asset", ref("GltfAssetDiff"), lab("Asset metadata", "Asset-Metadaten")), ("scene", opt(INDEX), "GltfDiff.scene")]
    fields += [(wire, ref(name), f"GltfDiff.{wire}") for wire, name in collections]
    fields += [("extensionsUsed", arr(STR), "GltfDiff.extensionsUsed"), ("extensionsRequired", arr(STR), "GltfDiff.extensionsRequired"), ("extensions", presence, "GltfDiff.extensions"), ("extras", presence, "GltfDiff.extras"), ("sourceForm", rec("GltfSourceForm"), "GltfDiff.sourceForm")]
    root = record("GltfDiff", fields, [])
    return {"$schema": DRAFT, "$id": DIFF, "title": "GltfDiff", "description": "Sparse per-field glTF diff: an absent member is unchanged; index-keyed removed/modified/added triples per top-level array; no full-replace snapshot slot.", **root, "$defs": formats(defs)}


#region 🧬️Leaves
PHASE_UI = lab("Mutation phase", "Mutationsphase", role="discriminator")
APPLY_UI = lab("Apply", "Anwenden")
RESTORE_UI = lab("Restore", "Wiederherstellen")
APPLY_VALUE_UI = lab("Parameters", "Parameter", "Parameters of the forward change.", "Parameter der Vorwärtsänderung.")
RESTORE_VALUE_UI = {"widget": "hidden", **lab("Restore diff", "Wiederherstellungs-Diff", "Recorded difference that restores the state before the change; written by the inverse, not edited by hand.", "Aufgezeichnete Differenz, die den Zustand vor der Änderung wiederherstellt; stammt aus der Inversen und wird nicht von Hand bearbeitet.")}
SHARED = {"GltfJson", "GltfCameraProjection", "GltfPerspective", "GltfOrthographic", "GltfComponentType", "GltfAccessorType", "GltfAlphaMode"}
PRESENCE = {"oneOf": [record("GltfDataPresence", [("state", {"const": "absent"}, lab("Presence", "Vorhandensein"))], ["state"]), record("GltfDataPresence", [("state", {"const": "present"}, lab("Presence", "Vorhandensein")), ("value", ref("GltfJson", SNAP), lab("JSON value", "JSON-Wert"))], ["state", "value"])]}
MATERIAL = with_ui(INDEX, {"widget": "stepper", **lab("Material", "Material", "Index of the material, counted from 0.", "Index des Materials, ab 0 gezählt."), "precision": 0, "group": "target", "order": 10})


def camel(ident):
    return ident[0].lower() + ident[1:]


def rust_leaf(path):
    """🦀️ `(phase enum, apply type, restore type, {struct: [(wire field, rust type)]})` of one leaf source."""
    source = path.read_text()
    phase = re.search(r'#\[value\(tag = "phase", content = "value"[^\]]*\)\]\s*pub enum (\w+)\s*\{\s*Apply\((\w+)\),\s*Restore\(([^)]*\)?)\),', source)
    if phase is None:
        return None
    structs = {}
    for match in re.finditer(r'pub struct (\w+)\s*(?:\{\s*\}|\{(.*?)\n\})', source, re.S):
        fields = []
        for field in re.finditer(r'(?:#\[value\(([^)]*)\)\]\s*)?pub (\w+): ([^,\n]+),', match.group(2) or ""):
            rename = re.search(r'rename = "(\w+)"', field.group(1) or "")
            wire = rename.group(1) if rename else re.sub(r"_(\w)", lambda m: m.group(1).upper(), field.group(2))
            fields.append((wire, field.group(3).strip(), field.group(1) or ""))
        structs[match.group(1)] = fields
    return phase.group(1), phase.group(2), phase.group(3), structs


def local_refs(node, found):
    if isinstance(node, dict):
        target = node.get("$ref")
        if isinstance(target, str) and target.startswith("#/$defs/"):
            found.add(target[len("#/$defs/"):])
        for value in node.values():
            local_refs(value, found)
    elif isinstance(node, list):
        for value in node:
            local_refs(value, found)
    return found


def relink(node):
    """🔗️ Local refs to shared snapshot records now point into the snapshot document."""
    if isinstance(node, dict):
        target = node.get("$ref")
        if isinstance(target, str) and target.startswith("#/$defs/") and target[len("#/$defs/"):] in SHARED:
            node = {**node, "$ref": f"{SNAP}#/$defs/{target[len('#/$defs/'):]}"}
        return {key: relink(value) for key, value in node.items()}
    if isinstance(node, list):
        return [relink(value) for value in node]
    return node


def apply_payload(schema, apply_name):
    """📥️ The apply payload schema and its private `$defs`, whether the file is still flat or already a phase union."""
    defs = dict(schema.get("$defs", {}))
    if "oneOf" in schema and "properties" not in schema:
        payload = defs[apply_name]
    else:
        payload = {key: schema[key] for key in ("type", "additionalProperties", "required", "properties") if key in schema}
    needed, frontier = set(), local_refs(payload, set())
    while frontier:
        name = frontier.pop()
        if name in needed or name in SHARED:
            continue
        needed.add(name)
        local_refs(defs[name], frontier)
    return payload, {name: defs[name] for name in sorted(needed)}


def correct(leaf, payload, defs):
    """🩹️ Per-leaf corrections of payloads whose committed schema described something other than the Rust wire."""
    props = payload["properties"]
    if leaf.endswith("💎️material/🌫️change-alpha"):
        payload = {"type": "object", "properties": {"material": MATERIAL, "alphaMode": with_ui(ref("GltfAlphaMode", SNAP), {"widget": "segmented", **lab("Alpha mode", "Alphamodus", "How the alpha channel of the base color is interpreted.", "Wie der Alphakanal der Grundfarbe ausgewertet wird."), "group": "value", "order": 20})}}
    elif leaf.endswith("💎️material/🪞️change-sides"):
        payload = {"type": "object", "properties": {"material": MATERIAL, "doubleSided": with_ui(BOOL, {"widget": "toggle", **lab("Double-sided", "Doppelseitig", "Render back faces too instead of culling them.", "Rückseiten mitrendern statt sie auszublenden."), "group": "value", "order": 20})}}
    elif leaf.endswith("📐️accessor/🌱️create"):
        props["componentType"] = with_ui(ref("GltfComponentType", SNAP), {**COMPONENT_UI, "group": "value", "order": 20})
        props["kind"] = with_ui(ref("GltfAccessorType", SNAP), {"widget": "select", **lab("Element type", "Elementtyp"), "group": "value", "order": 40})
    elif leaf.endswith("🎥️camera/🌱️create"):
        props["projection"] = with_ui(ref("GltfCameraProjection", SNAP), props["projection"]["x-semio-ui"])
    elif leaf.endswith("🌳️node/📐️transform"):
        trs = defs["GltfNodeTransform"]["oneOf"][1]
        trs["required"] = ["kind", "translation", "rotation", "scale"]
        for key, labels in (("translation", ("Translation", "Verschiebung", None, None)), ("rotation", ("Rotation", "Drehung", "Unit quaternion (x, y, z, w).", "Einheitsquaternion (x, y, z, w).")), ("scale", ("Scale", "Skalierung", None, None))):
            trs["properties"][key] = with_ui(trs["properties"][key], trs["properties"][key].get("x-semio-ui") or lab(*labels))
        mat = defs["GltfNodeTransform"]["oneOf"][0]["properties"]["matrix"]
        defs["GltfNodeTransform"]["oneOf"][0]["properties"]["matrix"] = with_ui(mat, mat.get("x-semio-ui") or lab("Matrix", "Matrix", "4×4 transform, column-major.", "4×4-Transformationsmatrix, spaltenweise."))
    for key, node in list(payload["properties"].items()):
        if key == "data":
            target = node.get("$ref") or ""
            if target.endswith("GltfDataPresence"):
                defs["GltfDataPresence"] = PRESENCE
            else:
                payload["properties"][key] = with_ui(ref("GltfJson", SNAP), node["x-semio-ui"])
    defs.pop("GltfJson", None)
    payload = {"type": "object", "additionalProperties": False, **({"required": list(payload["properties"])} if payload["properties"] else {}), "properties": payload["properties"]}
    return payload, defs


RUST_TYPES = {"usize": "integer", "u32": "integer", "u64": "integer", "f64": "number", "bool": "boolean", "String": "string"}


def check_fields(leaf, name, payload, structs):
    """⚖️ The payload's members are exactly the Rust struct's wire fields."""
    fields = structs.get(name)
    if fields is None:
        raise SystemExit(f"{leaf}: no Rust struct {name}")
    wire = [field for field, _, _ in fields]
    if sorted(wire) != sorted(payload["properties"]):
        raise SystemExit(f"{leaf}: {name} Rust fields {wire} vs schema {sorted(payload['properties'])}")
    for field, rust, attrs in fields:
        if "skip_serializing_if" in attrs:
            raise SystemExit(f"{leaf}: {name}.{field} is optional on the wire; teach the generator")
        schema = payload["properties"][field]
        kind = RUST_TYPES.get(rust)
        if kind is not None and schema.get("type") != kind:
            raise SystemExit(f"{leaf}: {name}.{field} is {rust}, schema says {schema.get('type')}")
        if rust.startswith("Option<") and not (schema.get("anyOf") or "$ref" in schema):
            raise SystemExit(f"{leaf}: {name}.{field} is {rust} (emitted as null when absent), schema is not nullable")


def leaf_schema(leaf_dir, descriptor):
    """🧬️ A wrapped leaf (`#[mutation_leaf(payload = Apply)]`): the root is the editable `Apply` payload `payload_value()` yields,
    and `$defs/<phase enum>` is the leaf's whole wire, which the aggregate branch refers to (`restore` → the shared diff)."""
    parsed = rust_leaf(leaf_dir / "🦀️.rs")
    path = leaf_dir / "🧬️schema/🔣️.json"
    schema = json.loads(path.read_text())
    if parsed is None:
        return schema, None
    phase, apply_name, restore, structs = parsed
    if f"#[mutation_leaf(contract = ::protocol, payload = Apply)]\n#[value(tag = \"phase\"" not in (leaf_dir / "🦀️.rs").read_text():
        raise SystemExit(f"{leaf_dir}: {phase} is not marked payload = Apply")
    payload, defs = apply_payload(schema, apply_name)
    payload, defs = correct(str(leaf_dir), relink(payload), relink(defs))
    check_fields(leaf_dir.name, apply_name, payload, structs)
    if "GltfDiff" in restore:
        restore_value = {"$ref": DIFF}
    else:
        existing = schema.get("$defs", {}).get(restore)
        if existing is None:
            raise SystemExit(f"{leaf_dir}: restore type {restore} has no schema")
        defs[restore] = {**existing, "required": list(existing["properties"])}
        restore_value = ref(restore)
    branch = lambda phase_value, value: {"type": "object", "additionalProperties": False, "required": ["phase", "value"], "properties": {"phase": {"const": phase_value}, "value": value}}
    defs[phase] = {"description": f"The whole wire of {phase}: the editable apply payload (this document's root) or the inert restore.", "oneOf": [branch("apply", {"$ref": "#"}), branch("restore", restore_value)]}
    return {"$schema": DRAFT, "$id": schema["$id"], "title": apply_name, **payload, "$defs": dict(sorted(defs.items()))}, phase
#endregion 🧬️Leaves


#region 🧫️Fixtures
def fixtures(variants):
    """🦠️ `🧬️operation/🔣️.json` (the flat apply payload) → `🦠️mutation/🔣️.json` (the aggregate wire value)."""
    for operation in sorted(FIXTURES.rglob("🧬️operation/🔣️.json")):
        case = operation.parent.parent
        leaf = case.parent.relative_to(FIXTURES)
        variant = variants.get(str(leaf))
        if variant is None:
            raise SystemExit(f"{case}: no leaf {leaf}")
        wire = {"mutation": camel(variant), "payload": {"phase": "apply", "value": json.loads(operation.read_text())}}
        target = case / "🦠️mutation/🔣️.json"
        print(("move " if WRITE else "would move ") + str(operation.relative_to(ROOT)))
        if WRITE:
            target.parent.mkdir(exist_ok=True)
            target.write_text(text(wire))
            shutil.rmtree(operation.parent)
#endregion 🧫️Fixtures


def main():
    emit(SCHEMA / "📸️snapshot/🔣️.json", snapshot_document())
    emit(SCHEMA / "🔺️diff/🔣️.json", diff_document())
    variants, wrappers = {}, {}
    for descriptor_path in sorted(LEAVES.glob("*/*/🔣️.json")):
        descriptor = json.loads(descriptor_path.read_text())
        if "aggregateVariant" not in descriptor:
            continue
        leaf_dir = descriptor_path.parent
        variants[str(leaf_dir.relative_to(LEAVES))] = descriptor["aggregateVariant"]
        schema, phase = leaf_schema(leaf_dir, descriptor)
        emit(leaf_dir / "🧬️schema/🔣️.json", schema)
        wrappers[camel(descriptor["aggregateVariant"])] = schema["$id"] if phase is None else f"{schema['$id']}#/$defs/{phase}"
    aggregate = json.loads((LEAVES / "🔣️.json").read_text())
    if sorted(branch["properties"]["mutation"]["const"] for branch in aggregate["oneOf"]) != sorted(wrappers):
        raise SystemExit("aggregate oneOf does not name every leaf wire exactly once")
    for branch in aggregate["oneOf"]:
        branch["properties"]["payload"] = {"$ref": wrappers[branch["properties"]["mutation"]["const"]]}
    emit(LEAVES / "🔣️.json", aggregate)
    fixtures(variants)

main()
