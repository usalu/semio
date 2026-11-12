"""Writes the language-agnostic vectors of the volume (`🧫️fixtures/🧊️volume-schema/🔣️.json`): patches of the sample figure that must stay valid, patches that must break exactly one rule, and polygon profiles with their verdicts."""
import json
import sys

out = sys.argv[1]
S = "/volume/solids/id="


def patch(*ops):
    return [dict(zip(("op", "path", "value"), op)) for op in ops]


def set_(path, value):
    return ("replace", path, value)


def add(path, value):
    return ("add", path, value)


tiny = {"id": "extra", "kind": "ellipsoid", "bone": "head", "material": "skin", "role": "detail", "tone": 0.5, "radii": [0.01, 0.01, 0.01]}
shoe = [[-0.06, -0.065], [0.2, -0.065], [0.21, -0.055], [0.21, -0.045], [0.14, -0.02], [0.03, -0.01], [-0.04, -0.01], [-0.06, -0.03]]
hole_a = [[0.0, -0.05], [0.06, -0.05], [0.06, -0.035], [0.0, -0.035]]
hole_b = [[0.04, -0.045], [0.1, -0.045], [0.1, -0.03], [0.04, -0.03]]
five = [{"bone": b, "weight": 0.2} for b in ("head", "torso", "pelvis", "arm-l", "arm-r")]

valid = [
    {"name": "box without corner radius", "patch": patch(("remove", f"{S}sole-l/cornerRadius"))},
    {"name": "hair swept half a turn", "patch": patch(add(f"{S}hair/sweepRad", 3.141592653589793))},
    {"name": "extrusion along u with a taper and a hole", "patch": patch(set_(f"{S}shoe-l/plane", "yz"), set_(f"{S}shoe-l/profile", [[-0.05, -0.05], [0.05, -0.05], [0.05, 0.05], [-0.05, 0.05]]), add(f"{S}shoe-l/holes", [[[-0.02, -0.02], [0.02, -0.02], [0.0, 0.02]]]), add(f"{S}shoe-l/taperTo", [1, 0.8]))},
    {"name": "loft with a section rotation and no exponent", "patch": patch(add(f"{S}torso/sections/1/rotation", [0, 0, 0.7071067811865476, 0.7071067811865476]), ("remove", f"{S}torso/sections/0/exponent"))},
    {"name": "repeat that rotates about an axis", "patch": patch(add(f"{S}buttons/repeat/rotationAxis", [1, 0, 0]), add(f"{S}buttons/repeat/rotationRad", 0.5), add(f"{S}buttons/repeat/origin", [0.1, 0, 0]))},
    {"name": "capsule that ends in a point", "patch": patch(set_(f"{S}arm-l/r1", 0))},
    {"name": "transform with scale and a unit rotation", "patch": patch(add(f"{S}nose/transform/scale", [1.5, 1, 1]), add(f"{S}nose/transform/rotation", [0, 0, 0.7071067811865476, 0.7071067811865476]))},
    {"name": "coarsest scale and several regions", "patch": patch(set_(f"{S}nose/maxDenominator", 500), add(f"{S}nose/regions", ["head", "torso"]))},
    {"name": "single weight and tags", "patch": patch(set_(f"{S}neck/weights", [{"bone": "head", "weight": 1}]), add(f"{S}neck/tags", ["skinned", "glyph"]))},
]

invalid = [
    {"name": "second solid with an id already used", "rule": "volume-duplicate-id", "schema": "accepts", "patch": patch(add("/volume/solids/-", {**tiny, "id": "head"}))},
    {"name": "solid bound to a missing bone", "rule": "volume-bone", "schema": "accepts", "patch": patch(set_(f"{S}head/bone", "no-such-bone"))},
    {"name": "solid with a missing material", "rule": "volume-material", "schema": "accepts", "patch": patch(set_(f"{S}head/material", "no-such-material"))},
    {"name": "solid with a missing region", "rule": "volume-region", "schema": "accepts", "patch": patch(set_(f"{S}head/region", "tail"))},
    {"name": "solid with one missing region of several", "rule": "volume-region", "schema": "accepts", "patch": patch(add(f"{S}head/regions", ["head", "tail"]))},
    {"name": "weights on a missing bone", "rule": "volume-weights-bone", "schema": "accepts", "patch": patch(set_(f"{S}neck/weights/0/bone", "nowhere"))},
    {"name": "weights that name a bone twice", "rule": "volume-weights-bone", "schema": "accepts", "patch": patch(set_(f"{S}neck/weights/1/bone", "head"))},
    {"name": "weights that sum to 0.8", "rule": "volume-weights-sum", "schema": "accepts", "patch": patch(set_(f"{S}neck/weights/0/weight", 0.5))},
    {"name": "weights with a negative entry", "rule": "volume-weights-sum", "schema": "rejects", "patch": patch(set_(f"{S}neck/weights", [{"bone": "head", "weight": 1.3}, {"bone": "torso", "weight": -0.3}]))},
    {"name": "weights over five bones", "rule": "volume-weights-count", "schema": "rejects", "patch": patch(set_(f"{S}neck/weights", five))},
    {"name": "ellipsoid with a zero radius", "rule": "volume-size", "schema": "rejects", "patch": patch(set_(f"{S}head/radii/1", 0))},
    {"name": "box with a corner radius above half its smallest edge", "rule": "volume-size", "schema": "accepts", "patch": patch(set_(f"{S}sole-l/cornerRadius", 0.05))},
    {"name": "capsule with both radii zero", "rule": "volume-size", "schema": "accepts", "patch": patch(set_(f"{S}arm-l/r0", 0), set_(f"{S}arm-l/r1", 0))},
    {"name": "revolve with a zero lateral scale", "rule": "volume-size", "schema": "rejects", "patch": patch(set_(f"{S}hair/lateralScale", 0))},
    {"name": "revolve about the zero vector", "rule": "volume-size", "schema": "accepts", "patch": patch(set_(f"{S}hair/axis", [0, 0, 0]))},
    {"name": "revolve with a negative radius", "rule": "volume-size", "schema": "accepts", "patch": patch(set_(f"{S}hair/profile/1/1", -0.05))},
    {"name": "loft section with a zero radius", "rule": "volume-size", "schema": "rejects", "patch": patch(set_(f"{S}torso/sections/1/rx", 0))},
    {"name": "extrusion with a negative taper", "rule": "volume-size", "schema": "rejects", "patch": patch(add(f"{S}shoe-l/taperTo", [1, -0.1]))},
    {"name": "extrusion with from above to", "rule": "volume-extrude-range", "schema": "accepts", "patch": patch(set_(f"{S}shoe-l/from", 0.135), set_(f"{S}shoe-l/to", 0.045))},
    {"name": "extrusion with from equal to to", "rule": "volume-extrude-range", "schema": "accepts", "patch": patch(set_(f"{S}shoe-l/from", 0.1), set_(f"{S}shoe-l/to", 0.1))},
    {"name": "extrusion profile on one line", "rule": "volume-profile-degenerate", "schema": "accepts", "patch": patch(set_(f"{S}shoe-l/profile", [[0, -0.01], [0.1, -0.01], [0.2, -0.01]]))},
    {"name": "extrusion profile with a repeated point", "rule": "volume-profile-degenerate", "schema": "accepts", "patch": patch(set_(f"{S}shoe-l/profile", [[0, 0], [0.1, 0], [0.1, 0], [0.1, 0.1]]))},
    {"name": "revolve profile on the axis", "rule": "volume-profile-degenerate", "schema": "accepts", "patch": patch(set_(f"{S}hair/profile", [[0.1, 0], [0.2, 0]]))},
    {"name": "extrusion profile that crosses itself", "rule": "volume-profile-self-intersecting", "schema": "accepts", "patch": patch(set_(f"{S}shoe-l/profile", [[0, 0], [0.2, 0.1], [0.2, 0], [0, 0.1]]))},
    {"name": "extrusion profile whose vertex touches an edge", "rule": "volume-profile-self-intersecting", "schema": "accepts", "patch": patch(set_(f"{S}shoe-l/profile", [[0, 0], [0.2, 0], [0.2, 0.1], [0.1, 0], [0.1, -0.1], [0, -0.1]]))},
    {"name": "revolve profile that crosses itself", "rule": "volume-profile-self-intersecting", "schema": "accepts", "patch": patch(set_(f"{S}hair/profile", [[0.1, 0.1], [0.2, 0], [0.2, 0.1], [0.1, 0]]))},
    {"name": "hole outside the outline", "rule": "volume-hole-outside", "schema": "accepts", "patch": patch(add(f"{S}shoe-l/holes", [[[0.3, -0.05], [0.32, -0.05], [0.31, -0.03]]]))},
    {"name": "hole crossing the outline", "rule": "volume-hole-outside", "schema": "accepts", "patch": patch(add(f"{S}shoe-l/holes", [[[0.15, -0.07], [0.25, -0.07], [0.2, -0.04]]]))},
    {"name": "two holes that overlap", "rule": "volume-holes-overlap", "absent": ["volume-hole-outside"], "schema": "accepts", "patch": patch(add(f"{S}shoe-l/holes", [hole_a, hole_b]))},
    {"name": "loft with a single section", "rule": "volume-loft-sections", "schema": "rejects", "patch": patch(set_(f"{S}torso/sections", [{"at": [0, 0, 0], "rx": 0.1, "ry": 0.1}]))},
    {"name": "loft with two sections at one centre", "rule": "volume-loft-sections", "schema": "accepts", "patch": patch(set_(f"{S}torso/sections/1/at", [0, 0, 0]))},
    {"name": "repeat with count zero", "rule": "volume-repeat", "schema": "rejects", "patch": patch(set_(f"{S}buttons/repeat/count", 0))},
    {"name": "repeat above the bound", "rule": "volume-repeat", "schema": "rejects", "patch": patch(set_(f"{S}buttons/repeat/count", 257))},
    {"name": "repeat with a fractional count", "rule": "volume-repeat", "schema": "rejects", "patch": patch(set_(f"{S}buttons/repeat/count", 2.5))},
    {"name": "repeat that rotates without an axis", "rule": "volume-repeat", "schema": "accepts", "patch": patch(add(f"{S}buttons/repeat/rotationRad", 0.5))},
    {"name": "maxDenominator that is not a preset", "rule": "volume-max-denominator", "schema": "rejects", "patch": patch(set_(f"{S}nose/maxDenominator", 75))},
    {"name": "rotation that is not a unit quaternion", "rule": "volume-transform", "schema": "accepts", "patch": patch(add(f"{S}nose/transform/rotation", [0, 0, 0, 2]))},
    {"name": "scale with a zero factor", "rule": "volume-transform", "schema": "rejects", "patch": patch(add(f"{S}nose/transform/scale", [1, 0, 1]))},
    {"name": "loft section rotation that is not a unit quaternion", "rule": "volume-transform", "schema": "accepts", "patch": patch(add(f"{S}torso/sections/1/rotation", [1, 1, 0, 0]))},
    {"name": "volume without solids", "rule": "volume-empty", "schema": "rejects", "patch": patch(set_("/volume/solids", []))},
    {"name": "unknown property on a solid", "rule": None, "schema": "rejects", "patch": patch(add(f"{S}head/surprise", 1))},
    {"name": "solid without tone", "rule": None, "schema": "rejects", "patch": patch(("remove", f"{S}head/tone"))},
    {"name": "solid of an unknown kind", "rule": None, "schema": "rejects", "patch": patch(set_(f"{S}head/kind", "cone"))},
    {"name": "extrusion on an unknown plane", "rule": None, "schema": "rejects", "patch": patch(set_(f"{S}shoe-l/plane", "ab"))},
    {"name": "extrusion profile of two points", "rule": None, "schema": "rejects", "patch": patch(set_(f"{S}shoe-l/profile", [[0, 0], [1, 1]]))},
    {"name": "volume that is not authored", "rule": None, "schema": "rejects", "patch": patch(set_("/volume/status", "draft"))},
    {"name": "volume without decisions", "rule": None, "schema": "rejects", "patch": patch(("remove", "/volume/decisions"))},
    {"name": "solid with a tone above 1", "rule": None, "schema": "rejects", "patch": patch(set_(f"{S}head/tone", 1.5))},
    {"name": "solid with a bone that is not a slug", "rule": None, "schema": "rejects", "patch": patch(set_(f"{S}head/bone", "Head Bone"))},
]

square = [[0, 0], [4, 0], [4, 4], [0, 4]]
polygons = [
    {"name": "square", "points": square, "verdict": "simple", "oracle": True},
    {"name": "L shape", "points": [[0, 0], [4, 0], [4, 1], [1, 1], [1, 4], [0, 4]], "verdict": "simple", "oracle": True},
    {"name": "clockwise square", "points": [[0, 0], [0, 4], [4, 4], [4, 0]], "verdict": "simple", "oracle": True},
    {"name": "bow tie", "points": [[0, 0], [4, 4], [4, 0], [0, 4]], "verdict": "self-intersecting", "oracle": True},
    {"name": "vertex touching an edge", "points": [[0, 0], [4, 0], [4, 4], [2, 0], [2, -4], [0, -4]], "verdict": "self-intersecting", "oracle": True},
    {"name": "three points on one line", "points": [[0, 0], [1, 0], [2, 0]], "verdict": "degenerate", "oracle": True},
    {"name": "repeated point", "points": [[0, 0], [4, 0], [4, 0], [4, 4]], "verdict": "degenerate", "oracle": False},
    {"name": "square with a hole inside", "points": square, "holes": [[[1, 1], [2, 1], [2, 2], [1, 2]]], "verdict": "simple", "holesVerdict": "ok", "oracle": True},
    {"name": "square with a hole outside", "points": square, "holes": [[[5, 1], [6, 1], [6, 2]]], "verdict": "simple", "holesVerdict": "hole-outside", "oracle": True},
    {"name": "square with a hole crossing the outline", "points": square, "holes": [[[3, 1], [5, 1], [5, 2]]], "verdict": "simple", "holesVerdict": "hole-outside", "oracle": True},
    {"name": "square with a hole touching the outline", "points": square, "holes": [[[0, 1], [1, 1], [1, 2]]], "verdict": "simple", "holesVerdict": "hole-outside", "oracle": False},
    {"name": "square with two overlapping holes", "points": square, "holes": [[[1, 1], [2.5, 1], [2.5, 2.5], [1, 2.5]], [[2, 2], [3, 2], [3, 3], [2, 3]]], "verdict": "simple", "holesVerdict": "holes-overlap", "oracle": True},
    {"name": "square with one hole in another", "points": square, "holes": [[[0.5, 0.5], [3.5, 0.5], [3.5, 3.5], [0.5, 3.5]], [[1, 1], [2, 1], [2, 2], [1, 2]]], "verdict": "simple", "holesVerdict": "holes-overlap", "oracle": True},
    {"name": "square with two apart holes", "points": square, "holes": [[[0.5, 0.5], [1.5, 0.5], [1, 1.5]], [[2.5, 2.5], [3.5, 2.5], [3, 3.5]]], "verdict": "simple", "holesVerdict": "ok", "oracle": True},
]
polylines = [
    {"name": "zigzag", "points": [[0, 0], [1, 1], [2, 0], [3, 1]], "verdict": "simple", "oracle": True},
    {"name": "profile of a vase", "points": [[0, 1], [0.3, 1.2], [0.6, 0.8], [1, 0.5], [1, 0]], "verdict": "simple", "oracle": True},
    {"name": "crossing", "points": [[0, 0], [2, 2], [2, 0], [0, 2]], "verdict": "self-intersecting", "oracle": True},
    {"name": "folding back", "points": [[0, 1], [2, 1], [1, 1]], "verdict": "self-intersecting", "oracle": True},
    {"name": "everything on the axis", "points": [[0, 0], [1, 0]], "verdict": "degenerate", "oracle": False},
]

document = {
    "description": "Vectors of the declarative volume (semio.staffage/v2), language-agnostic: `valid` patches of the sample figure that both ajv and the owned rules accept; `invalid` patches that break exactly the named rule (`rule`, null when only the schema judges) and that the schema accepts or rejects as stated (`schema`); `absent` rules that must not appear; `polygons` and `polylines` with the verdict of the owned profile rule and whether a shapely `is_valid` / `is_simple` oracle agrees (`oracle`, false where the owned rule is deliberately stricter: repeated points, holes touching the outline, a polyline on the axis).",
    "defs": ["Volume", "VolumeSolid", "VolumeRepeat", "VolumeTransform", "Vector2", "Vector3", "Quaternion"],
    "rules": sorted({case["rule"] for case in invalid if case["rule"]} | {"volume-non-finite"}),
    "solidKinds": ["box", "ellipsoid", "capsule", "revolve", "extrude", "loft"],
    "bounds": {"toleranceRelative": 0.02, "toleranceM": 0.015},
    "valid": valid,
    "invalid": invalid,
    "polygons": polygons,
    "polylines": polylines,
}
open(out, "w", encoding="utf8").write(json.dumps(document, indent=1, ensure_ascii=False) + "\n")
