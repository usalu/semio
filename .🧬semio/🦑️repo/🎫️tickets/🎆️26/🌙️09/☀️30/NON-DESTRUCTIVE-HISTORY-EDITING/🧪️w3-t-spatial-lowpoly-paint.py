#!/usr/bin/env python3
"""🧪️ W3-T-SPATIAL: authors the lowpoly `apply-paint-stroke` leaf's committed scenarios with an INDEPENDENT brush (numpy
float32, written from the leaf's schema and the brush law stated in its doc comments — no Rust read), writes each
scenario's Rust test, mounts the leaf, and registers the kind with every surface that enumerates the vocabulary: the
aggregate enum and KINDS, the JSON/TS/GraphQL/proto/grammar/protocol twins, the `💠️mutate-lowpoly-1` harness (Rust
subject, Python oracle, Gherkin rows) and the oracle catalog. Idempotent.
"""
import base64
import copy
import hashlib
import json
import math
import os
import re
from collections import OrderedDict

import numpy as np

ARTIFACT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly"
ANY = ARTIFACT + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUTATIONS = ANY + "/🧬️schema/🧬️mutations"
KIND, KIND_DIR, VARIANT, MODULE = "apply-paint-stroke", "🖌️apply-paint-stroke", "ApplyPaintStroke", "apply_paint_stroke"
LEAF = f"{MUTATIONS}/{KIND_DIR}"
FIXTURES = ANY + "/🧫️fixtures/🧬️mutations/" + KIND_DIR
HARNESS = ANY + "/🧪️tests/💠️mutate-lowpoly-1"
F32 = np.float32


# region 🖌️Brush
def round_away(value):
    """🔢️ Rounds half away from zero, the brush law's rounding."""
    value = float(value)
    return math.floor(value + 0.5) if value >= 0 else -math.floor(-value + 0.5)


def stamp(pixels, side, u, v, radius, color, hardness, opacity, eraser):
    """🖌️ One dab on a square RGBA buffer `side` pixels wide, in float32: the centre is the UV point (v up) rounded
    onto the pixel grid, every pixel within `radius` (at least half a pixel) is reached, its stamp is
    `round((hardness + (1 - hardness)·(1 - dist/radius)) · opacity · 255)`; a brush writes the colour's RGB and adds
    the stamp to alpha (saturating), an eraser subtracts it (saturating)."""
    size = F32(side)
    clamp = lambda value: min(max(F32(value), F32(0.0)), F32(1.0))
    cx = round_away(clamp(u) * (size - F32(1.0)))
    cy = round_away((F32(1.0) - clamp(v)) * (size - F32(1.0)))
    r = max(F32(radius), F32(0.5))
    reach = math.ceil(float(r))
    hard, alpha = clamp(hardness), clamp(opacity)
    for y in range(cy - reach, cy + reach + 1):
        for x in range(cx - reach, cx + reach + 1):
            if x < 0 or y < 0 or x >= side or y >= side:
                continue
            dx, dy = F32(x - cx), F32(y - cy)
            dist = np.sqrt(dx * dx + dy * dy, dtype=F32)
            if dist > r:
                continue
            t = F32(1.0) - dist / r
            falloff = hard + (F32(1.0) - hard) * t
            amount = int(min(max(round_away(falloff * alpha * F32(255.0)), 0), 255))
            at = (y * side + x) * 4
            if eraser:
                pixels[at + 3] = max(pixels[at + 3] - amount, 0)
            else:
                pixels[at:at + 3] = bytes(color[:3])
                pixels[at + 3] = min(pixels[at + 3] + amount, 255)


def runs_between(before, after):
    """🩸 The changed pixels as contiguous whole-pixel runs `(offset, bytes)`."""
    runs, pixel, count = [], 0, len(before) // 4
    while pixel < count:
        if before[pixel * 4:pixel * 4 + 4] == after[pixel * 4:pixel * 4 + 4]:
            pixel += 1
            continue
        start = pixel
        while pixel < count and before[pixel * 4:pixel * 4 + 4] != after[pixel * 4:pixel * 4 + 4]:
            pixel += 1
        runs.append((start * 4, bytes(after[start * 4:pixel * 4])))
    return runs
# endregion 🖌️Brush


# region 🧫️Scenarios
def solid(side, rgba):
    return base64.b64encode(bytes(rgba) * side * side).decode("ascii")


def document():
    """🧫️ A two-object document: `obj-hull` carries an 8×8 opaque-white base layer and an 8×8 clear (fully
    transparent) layer above it; `obj-fin` carries no paint stack."""
    transform = OrderedDict([("position", [0.0, 0.0, 0.0]), ("rotation", [0.0, 0.0, 0.0]), ("scale", [1.0, 1.0, 1.0])])
    layer = lambda name, rgba: OrderedDict([("blendMode", "normal"), ("name", name), ("opacity", 1.0), ("pixels", solid(8, rgba)), ("visible", True)])
    hull = OrderedDict([("id", "obj-hull"), ("mesh", None), ("meshContent", ""), ("name", "Hull"), ("paintLayers", [layer("Base", [255, 255, 255, 255]), layer("Clear", [0, 0, 0, 0])]), ("smoothShading", False), ("transform", copy.deepcopy(transform))])
    fin = OrderedDict([("id", "obj-fin"), ("mesh", None), ("meshContent", ""), ("name", "Fin"), ("paintLayers", []), ("smoothShading", True), ("transform", copy.deepcopy(transform))])
    return OrderedDict([("objects", [hull, fin]), ("schema", "lowpoly.document")])


def stroke(points, layer=0, eraser=False, color=(200, 30, 30, 255), radius=1.5, hardness=0.5, opacity=1.0, target="obj-hull"):
    return {VARIANT: OrderedDict([("color", list(color)), ("eraser", eraser), ("hardness", hardness), ("layerIndex", layer), ("objectId", target), ("opacity", opacity), ("points", [list(point) for point in points]), ("radius", radius)])}


SCENARIOS = [
    {"emoji": "🖌️", "slug": "dabs-red-across-the-base", "module": "dabs_red_across_the_base", "story": "Three red dabs run diagonally across the opaque-white base layer of obj-hull: each one paints a soft disc, the later dabs overpainting where they overlap.", "payload": stroke([(0.25, 0.75), (0.5, 0.5), (0.75, 0.25)])},
    {"emoji": "🧽️", "slug": "erases-a-hole-in-the-base", "module": "erases_a_hole_in_the_base", "story": "One hard eraser dab at half opacity thins the alpha of the base layer's centre, leaving its colour untouched.", "payload": stroke([(0.5, 0.5)], eraser=True, radius=2.0, hardness=1.0, opacity=0.5)},
    {"emoji": "⏸️", "slug": "erases-the-clear-layer", "module": "erases_the_clear_layer", "story": "An eraser dab over the already transparent second layer has no alpha left to remove: the stroke is a declared no-op.", "payload": stroke([(0.5, 0.5)], layer=1, eraser=True, radius=3.0)},
    {"emoji": "⛔️", "slug": "paints-a-missing-layer", "module": "paints_a_missing_layer", "story": "A stroke on paint layer 5 of obj-hull, whose stack holds two, addresses nothing and is refused.", "payload": stroke([(0.5, 0.5)], layer=5)},
    {"emoji": "🫓️", "slug": "paints-with-no-radius", "module": "paints_with_no_radius", "story": "A brush of radius 0 can reach no pixel and is refused as an invariant violation.", "payload": stroke([(0.5, 0.5)], radius=0.0), "invariant": True},
    {"emoji": "📐️", "slug": "dabs-off-the-texture", "module": "dabs_off_the_texture", "story": "A dab at u = 1.5 lies off the texture and is refused as an invariant violation.", "payload": stroke([(1.5, 0.5)]), "invariant": True},
]


def folder(scenario):
    return f'{scenario["emoji"]}{scenario["slug"]}-{hashlib.sha256(scenario["story"].encode("utf-8")).hexdigest()[:6]}'


def empty_diff():
    return OrderedDict([("artifact", None), ("objects", None), ("schema", None)])


def stroke_diff(object_id, layer_index, runs):
    return OrderedDict([("artifact", None), ("objects", OrderedDict([("added", []), ("patched", [OrderedDict([("id", object_id), ("paintLayers", OrderedDict([("added", []), ("patched", []), ("removed", []), ("strokes", [OrderedDict([("layerIndex", layer_index), ("runs", [OrderedDict([("bytes", base64.b64encode(chunk).decode("ascii")), ("offset", offset)]) for offset, chunk in runs])])])])), ("patch", OrderedDict([("mesh", None), ("meshContent", None), ("name", None), ("smoothShading", None), ("transform", None)]))])]), ("removed", []), ("reordered", None)])), ("schema", None)])


def apply_stroke(before, payload):
    """🧮️ `(after, diff, outcome)` of one stroke on `before`."""
    p = payload[VARIANT]
    if not (p["radius"] > 0) or not all(0.0 <= value <= 1.0 for value in (p["hardness"], p["opacity"])) or not p["points"] or not all(0.0 <= c <= 1.0 for point in p["points"] for c in point):
        return before, None, OrderedDict([("status", "rejected"), ("code", "mutation.invariant"), ("path", [p["objectId"]])])
    hull = next((record for record in before["objects"] if record["id"] == p["objectId"]), None)
    if hull is None or p["layerIndex"] >= len(hull["paintLayers"]):
        return before, None, OrderedDict([("status", "rejected"), ("code", "mutation.target-missing"), ("path", [p["objectId"]])])
    buffer = bytearray(base64.b64decode(hull["paintLayers"][p["layerIndex"]]["pixels"]))
    side = math.isqrt(len(buffer) // 4)
    painted = bytearray(buffer)
    for u, v in p["points"]:
        stamp(painted, side, u, v, p["radius"], p["color"], p["hardness"], p["opacity"], p["eraser"])
    runs = runs_between(buffer, painted)
    if not runs:
        return before, empty_diff(), OrderedDict([("status", "applied"), ("messages", [OrderedDict([("level", "warning"), ("code", "mutation.no-op")])])])
    after = copy.deepcopy(before)
    next(record for record in after["objects"] if record["id"] == p["objectId"])["paintLayers"][p["layerIndex"]]["pixels"] = base64.b64encode(bytes(painted)).decode("ascii")
    return after, stroke_diff(p["objectId"], p["layerIndex"], runs), OrderedDict([("status", "applied")])
# endregion 🧫️Scenarios


# region 📝️Files
def dump(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def once(text, old, new, label):
    if new in text:
        return text
    assert text.count(old) == 1, f"{label}: anchor not unique/absent: {old[:100]!r}"
    return text.replace(old, new, 1)


def test_file(scenario, name, diff, no_op):
    rel = f"../../../../../🧫️fixtures/🧬️mutations/{KIND_DIR}/{name}"
    constants = [f'const BEFORE: &str = include_str!("{rel}/📸️snapshot/⬅️before/🔣️.json");', f'const AFTER: &str = include_str!("{rel}/📸️snapshot/➡️after/🔣️.json");', f'const MUTATION: &str = include_str!("{rel}/🦠️mutation/🔣️.json");']
    if diff is not None and not no_op:
        constants.append(f'const DIFF: &str = include_str!("{rel}/🔺️diff/🔣️.json");')
    constants.append(f'const OUTCOME: &str = include_str!("{rel}/🎯️outcome/🔣️.json");')
    header = f"""//! 🧪️ `apply-paint-stroke` fixture — `{name}`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent float32 brush in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-lowpoly-paint.py`.
//!
//! {scenario["emoji"]} {scenario["story"]}

use super::laws;

{chr(10).join(constants)}
"""
    if diff is not None and not no_op:
        return header + """
/// ▶️ The stroke carries `before` to exactly the committed `after` and produces exactly the committed delta.
#[test]
fn applies_to_committed_after_with_the_committed_diff() {
    laws::forward(BEFORE, MUTATION, AFTER, DIFF);
}

/// ↩️ The computed inverse — the overwritten pixels written back as one edit — restores `before` exactly.
#[test]
fn inverse_restores_before() {
    laws::inverse_restores(BEFORE, MUTATION);
}

/// 🎯️ The declared outcome — status and ordered diagnostics — is what the leaf emits.
#[test]
fn declared_outcome_holds() {
    laws::declared_outcome(BEFORE, MUTATION, OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(BEFORE, AFTER, MUTATION, Some(DIFF));
}
"""
    return header + """
/// ⛔️ The refused or no-op stroke leaves the document byte-identical and emits the declared diagnostic.
#[test]
fn refusal_leaves_the_document_untouched() {
    laws::refusal(BEFORE, MUTATION, AFTER, OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(BEFORE, AFTER, MUTATION, None);
}
"""
# endregion 📝️Files


# region 🧬️Aggregate
def register_aggregate():
    path = MUTATIONS + "/🦀️.rs"
    text = read(path)
    text = once(text, "    EditPaintLayer(super::edit_paint_layer::EditPaintLayer),\n}", "    EditPaintLayer(super::edit_paint_layer::EditPaintLayer),\n    ApplyPaintStroke(super::apply_paint_stroke::ApplyPaintStroke),\n}", "enum")
    text = once(text, '    "edit-paint-layer",\n];', '    "edit-paint-layer",\n    "apply-paint-stroke",\n];', "KINDS")
    write(path, text)

    path = MUTATIONS + "/🔣️.json"
    data = json.loads(read(path), object_pairs_hook=OrderedDict)
    ref = "https://json.schemas.assets.semio-tech.com/s/lowpoly/lowpoly/mutation/apply-paint-stroke/schema.json"
    if not any(VARIANT in arm.get("properties", {}) for arm in data["oneOf"]):
        data["oneOf"].append(OrderedDict([("type", "object"), ("additionalProperties", False), ("required", [VARIANT]), ("properties", OrderedDict([(VARIANT, OrderedDict([("$ref", ref)]))]))]))
    data["description"] = data["description"].replace("Seventeen-variant", "Eighteen-variant")
    write(path, json.dumps(data, indent=2, ensure_ascii=False) + "\n")

    path = MUTATIONS + "/🟦️.ts"
    text = read(path)
    text = once(text, "  | { EditPaintLayer: { objectId: string; layerIndex: number; runs: PixelRun[] } };", "  | { EditPaintLayer: { objectId: string; layerIndex: number; runs: PixelRun[] } }\n  | { ApplyPaintStroke: { objectId: string; layerIndex: number; eraser: boolean; color: [number, number, number, number]; radius: number; hardness: number; opacity: number; points: [number, number][] } };", "ts union")
    text = once(text, '  "EditPaintLayer",\n] as const;', '  "EditPaintLayer",\n  "ApplyPaintStroke",\n] as const;', "ts tags")
    text = text.replace("seventeen variants: nine object-lane verbs, a\n * create/delete pair for the `mesh` CHILD slot, and six paint-layer verbs plus one pixel edit)", "eighteen variants: nine object-lane verbs, a\n * create/delete pair for the `mesh` CHILD slot, six paint-layer verbs, one pixel edit and one paint stroke)")
    write(path, text)

    path = MUTATIONS + "/🔗️.graphql"
    text = read(path)
    text = once(text, "input EditPaintLayerInput {\n  objectId: String!\n  layerIndex: Int!\n  runs: [PixelRunInput!]!\n}\n", "input EditPaintLayerInput {\n  objectId: String!\n  layerIndex: Int!\n  runs: [PixelRunInput!]!\n}\n\ninput ApplyPaintStrokeInput {\n  objectId: String!\n  layerIndex: Int!\n  eraser: Boolean!\n  color: [Int!]!\n  radius: Float!\n  hardness: Float!\n  opacity: Float!\n  points: [[Float!]!]!\n}\n", "graphql input")
    text = once(text, "  EDIT_PAINT_LAYER\n}", "  EDIT_PAINT_LAYER\n  APPLY_PAINT_STROKE\n}", "graphql enum")
    text = once(text, "  editPaintLayer: EditPaintLayerInput\n}", "  editPaintLayer: EditPaintLayerInput\n  applyPaintStroke: ApplyPaintStrokeInput\n}", "graphql envelope")
    text = once(text, "  | EditPaintLayerMutation\n", "  | EditPaintLayerMutation\n  | ApplyPaintStrokeMutation\n", "graphql union")
    text = once(text, "type EditPaintLayerPayload {\n  objectId: String!\n  layerIndex: Int!\n  runs: [PixelRun!]!\n}\n", "type EditPaintLayerPayload {\n  objectId: String!\n  layerIndex: Int!\n  runs: [PixelRun!]!\n}\n\ntype ApplyPaintStrokeMutation {\n  mutation: LowpolyMutationKind!\n  payload: ApplyPaintStrokePayload!\n}\ntype ApplyPaintStrokePayload {\n  objectId: String!\n  layerIndex: Int!\n  eraser: Boolean!\n  color: [Int!]!\n  radius: Float!\n  hardness: Float!\n  opacity: Float!\n  points: [[Float!]!]!\n}\n", "graphql type")
    write(path, text)

    path = MUTATIONS + "/🛰️.proto"
    text = read(path)
    text = once(text, "message EditPaintLayer {\n  string object_id = 1;\n  uint64 layer_index = 2;\n  repeated PixelRun runs = 3;\n}\n", "message EditPaintLayer {\n  string object_id = 1;\n  uint64 layer_index = 2;\n  repeated PixelRun runs = 3;\n}\n\nmessage StrokePoint {\n  float u = 1;\n  float v = 2;\n}\n\nmessage ApplyPaintStroke {\n  string object_id = 1;\n  uint64 layer_index = 2;\n  bool eraser = 3;\n  repeated uint32 color = 4;\n  float radius = 5;\n  float hardness = 6;\n  float opacity = 7;\n  repeated StrokePoint points = 8;\n}\n", "proto message")
    text = once(text, "    EditPaintLayer edit_paint_layer = 17;\n", "    EditPaintLayer edit_paint_layer = 17;\n    ApplyPaintStroke apply_paint_stroke = 18;\n", "proto oneof")
    write(path, text)

    path = MUTATIONS + "/📖️.grammar.semio"
    text = read(path)
    text = once(text, "         | edit-paint-layer\n", "         | edit-paint-layer | apply-paint-stroke\n", "grammar alternative")
    text = once(text, 'edit-paint-layer = "edit-paint-layer" "object-id" "=" IDENT "layer-index" "=" INT runs-table\n', 'edit-paint-layer = "edit-paint-layer" "object-id" "=" IDENT "layer-index" "=" INT runs-table\napply-paint-stroke = "apply-paint-stroke" "object-id" "=" IDENT "layer-index" "=" INT "eraser" "=" BOOL "color" "=" "[" INT INT INT INT "]" "radius" "=" FLOAT "hardness" "=" FLOAT "opacity" "=" FLOAT "points" "=" "[" stroke-point+ "]"\nstroke-point = "[" FLOAT FLOAT "]"\n', "grammar rule")
    write(path, text)

    path = MUTATIONS + "/💾️binary/📡️.protocol.semio"
    text = read(path)
    text = once(text, "record delete-mesh tag=17\nfield id utf8\n", "record delete-mesh tag=17\nfield id utf8\nrecord apply-paint-stroke tag=18\nfield object-id utf8\nfield layer-index varint\nfield eraser u8\nfield color array u8\nfield radius f32\nfield hardness f32\nfield opacity f32\nfield points array f32\n", "protocol record")
    write(path, text)


def mount():
    path = ARTIFACT + "/🦀️.rs"
    source = read(path)
    tests = []
    for scenario in SCENARIOS:
        tests += ["                            #[cfg(test)]", f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{KIND_DIR}/🧪️tests/{folder(scenario)}/🦀️.rs"]', f"                            mod tests_{scenario['module']};"]
    block = "\n".join(["                        #[path = \".\"]", f"                        pub mod {MODULE} {{",
                       f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{KIND_DIR}/🦀️.rs"]', "                            mod component;",
                       f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{KIND_DIR}/🔺️diff/🦀️.rs"]', "                            pub mod diff;",
                       f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{KIND_DIR}/↩️inverse/🦀️.rs"]', "                            pub mod inverse;",
                       "                            pub use component::*;", *tests, "                        }"]) + "\n"
    start = source.find(f"                        #[path = \".\"]\n                        pub mod {MODULE} {{")
    if start >= 0:
        end = source.index("                        }\n", source.index(f"pub mod {MODULE} {{", start)) + len("                        }\n")
        source = source[:start] + block + source[end:]
    else:
        anchor = "                            mod tests_paints_red_over_the_second_half_of_the_base_layer;\n                        }\n"
        assert source.count(anchor) == 1, "edit_paint_layer mount anchor"
        source = source.replace(anchor, anchor + block, 1)
    write(path, source)
# endregion 🧬️Aggregate


# region 🧪️Harness
PY_APPLY = '''    elif kind == "apply-paint-stroke":
        record = document["objects"][object_at(document, payload["objectId"], kind, "mutate")]
        layer = layer_at(record, payload["layerIndex"], kind, "mutate")
        layer["pixels"] = base64.b64encode(stroked(pixels(layer, "mutate-%s" % kind), payload)).decode("ascii")
'''
PY_INVERSE = '''    if kind == "apply-paint-stroke":
        record = document["objects"][object_at(document, payload["objectId"], kind, "inverse")]
        buffer = pixels(layer_at(record, payload["layerIndex"], kind, "inverse"), "inverse-%s" % kind)
        painted = stroked(buffer, payload)
        runs = []
        pixel = 0
        while pixel * 4 < len(buffer):
            if buffer[pixel * 4:pixel * 4 + 4] == painted[pixel * 4:pixel * 4 + 4]:
                pixel += 1
                continue
            start = pixel
            while pixel * 4 < len(buffer) and buffer[pixel * 4:pixel * 4 + 4] != painted[pixel * 4:pixel * 4 + 4]:
                pixel += 1
            runs.append({"offset": start * 4, "bytes": base64.b64encode(buffer[start * 4:pixel * 4]).decode("ascii")})
        return [("edit-paint-layer", {"objectId": payload["objectId"], "layerIndex": payload["layerIndex"], "runs": runs})]
'''
PY_BRUSH = '''def stroked(buffer, payload):
    """🖌️ `apply-paint-stroke`'s pixels: every dab of the stroke stamped in order onto a copy of the square RGBA
    buffer, in float32 — the centre is the UV point (v up) rounded half away from zero onto the pixel grid, every
    pixel within the radius (at least half a pixel) gains `round((hardness + (1 - hardness)·(1 - dist/radius)) ·
    opacity · 255)` of alpha under the brush colour, or loses it under the eraser, saturating either way."""
    f32 = numpy.float32
    away = lambda value: math.floor(float(value) + 0.5) if float(value) >= 0 else -math.floor(-float(value) + 0.5)
    unit = lambda value: min(max(f32(value), f32(0.0)), f32(1.0))
    held = bytearray(buffer)
    side = math.isqrt(len(held) // 4)
    if side * side * 4 != len(held) or not payload["points"] or not payload["radius"] > 0 or not 0 <= payload["hardness"] <= 1 or not 0 <= payload["opacity"] <= 1 or any(not 0 <= value <= 1 for point in payload["points"] for value in point):
        raise AssertionError("apply-paint-stroke: the layer is not a square texture, or the brush cannot dab, or a dab lies off the texture")
    size, radius = f32(side), max(f32(payload["radius"]), f32(0.5))
    hard, alpha = unit(payload["hardness"]), unit(payload["opacity"])
    for u, v in payload["points"]:
        cx, cy = away(unit(u) * (size - f32(1.0))), away((f32(1.0) - unit(v)) * (size - f32(1.0)))
        reach = math.ceil(float(radius))
        for y in range(cy - reach, cy + reach + 1):
            for x in range(cx - reach, cx + reach + 1):
                if not (0 <= x < side and 0 <= y < side):
                    continue
                dx, dy = f32(x - cx), f32(y - cy)
                dist = numpy.sqrt(dx * dx + dy * dy, dtype=f32)
                if dist > radius:
                    continue
                amount = int(min(max(away((hard + (f32(1.0) - hard) * (f32(1.0) - dist / radius)) * alpha * f32(255.0)), 0), 255))
                at = (y * side + x) * 4
                if payload["eraser"]:
                    held[at + 3] = max(held[at + 3] - amount, 0)
                else:
                    held[at:at + 3] = bytes(payload["color"][:3])
                    held[at + 3] = min(held[at + 3] + amount, 255)
    return bytes(held)


'''


def register_harness(rows):
    path = HARNESS + "/🦀️.rs"
    text = read(path)
    text = once(text, '    "edit-paint-layer",\n];', '    "edit-paint-layer",\n    "apply-paint-stroke",\n];', "rust KINDS")
    write(path, text)

    path = HARNESS + "/🐍️.py"
    text = read(path)
    text = once(text, '    "edit-paint-layer",\n)', '    "edit-paint-layer",\n    "apply-paint-stroke",\n)', "py KINDS")
    text = once(text, "import base64\nimport copy\nimport json\n", "import base64\nimport copy\nimport json\nimport math\n\nimport numpy\n", "py imports")
    text = once(text, '    else:\n        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)', PY_APPLY + '    else:\n        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)', "py apply")
    text = once(text, '    raise AssertionError("inverse-%s: this implementation declares no inverse for that kind" % kind)', PY_INVERSE + '    raise AssertionError("inverse-%s: this implementation declares no inverse for that kind" % kind)', "py inverse")
    text = once(text, "def object_at(document, identity, kind, where):", PY_BRUSH + "def object_at(document, identity, kind, where):", "py brush")
    write(path, text)

    path = HARNESS + "/🥒️.feature"
    text = read(path)
    for vector in rows:
        line = f"      | apply-paint-stroke            | {KIND_DIR}/{vector} |\n"
        anchor = "      | edit-paint-layer              | 🎨️edit-paint-layer/🖌️paints-red-over-second-half-base-layer |\n"
        if line not in text:
            assert text.count(anchor) == 2, "feature anchors"
            text = text.replace(anchor, anchor + line)
    write(path, text)

    path = ANY + "/🔮️oracles/🔣️.json"
    data = json.loads(read(path), object_pairs_hook=OrderedDict)
    catalog = data["mutationCatalogs"][0]
    if KIND not in catalog["kinds"]:
        catalog["kinds"].append(KIND)
    catalog["vectors"] = [vector for vector in catalog["vectors"] if vector["mutationId"] != KIND]
    catalog["vectors"].append(OrderedDict([("mutationId", KIND), ("sourceMutationDirectoryName", KIND_DIR), ("mutationDirectoryName", KIND_DIR), ("scenarios", [OrderedDict([("id", re.sub(r"^[^\w]+", "", folder(scenario))), ("directoryName", folder(scenario))]) for scenario in SCENARIOS])]))
    manifests = data["mutationManifests"][0]["mutations"]
    if not any(manifest["id"] == KIND for manifest in manifests):
        manifests.append(OrderedDict([("id", KIND), ("capability", "lowpoly-1-mutate"), ("payloadSchema", "🧬️.schema.json"), ("outcomes", ["applied", "no-op", "rejected"]), ("productionDispatch", OrderedDict([("operation", KIND), ("bridgeVersion", 1), ("variant", VARIANT)])), ("oracleRequirements", [OrderedDict([("capability", "lowpoly-1-mutate"), ("qualifyingKind", "verified-native-second-implementation")])])]))
    write(path, json.dumps(data, indent=2, ensure_ascii=False) + "\n")
# endregion 🧪️Harness


def main():
    applied_rows = []
    for scenario in SCENARIOS:
        name = folder(scenario)
        before = document()
        after, diff, outcome = apply_stroke(before, scenario["payload"])
        root = f"{FIXTURES}/{name}"
        dump(f"{root}/📸️snapshot/⬅️before/🔣️.json", before)
        dump(f"{root}/📸️snapshot/➡️after/🔣️.json", after)
        dump(f"{root}/🦠️mutation/🔣️.json", scenario["payload"])
        dump(f"{root}/🎯️outcome/🔣️.json", outcome)
        absent = f"{root}/🔺️diff/🚫️.absent"
        if diff is None:
            write(absent, "")
            if os.path.exists(f"{root}/🔺️diff/🔣️.json"):
                os.remove(f"{root}/🔺️diff/🔣️.json")
        else:
            dump(f"{root}/🔺️diff/🔣️.json", diff)
            if os.path.exists(absent):
                os.remove(absent)
        no_op = bool(outcome.get("messages")) and outcome["messages"][0]["code"] == "mutation.no-op"
        if diff is not None and not no_op:
            applied_rows.append(name)
        write(f"{LEAF}/🧪️tests/{name}/🦀️.rs", test_file(scenario, name, diff, bool(no_op)))
        print(name, outcome["status"], outcome.get("code", [message["code"] for message in outcome.get("messages", [])]), len(diff["objects"]["patched"][0]["paintLayers"]["strokes"][0]["runs"]) if diff and diff["objects"] else "")
    mount()
    register_aggregate()
    register_harness(applied_rows)


if __name__ == "__main__":
    main()
