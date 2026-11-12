"""Adds the declarative 3D volume to `semio.staffage/v2` (idempotent): Volume, VolumeSolid, VolumeRepeat, VolumeTransform, Vector2/3, Quaternion, the formats glb/fbx/model, the viewpoint axonometric and Asset.volume."""
import json
import sys

path = sys.argv[1]
raw = open(path, encoding="utf8").read()
schema = json.loads(raw)
defs = schema["$defs"]

ref = lambda name: {"$ref": f"#/$defs/{name}"}
num = {"type": "number"}
pos = {"type": "number", "exclusiveMinimum": 0}
nonneg = {"type": "number", "minimum": 0}

FRAME = (
    "Coordinates are metres in the rest frame of `bone`: the origin is the bone origin (including the bone's own lateral offset z), u runs along the bone from its joint towards its tip, "
    "v is its normal in the sagittal plane (the local y of an elevation shape, u turned by +90 degrees counter-clockwise in the side elevation) and w is lateral (left of the asset is +, world z at rest); "
    "this is the frame of the elevation shapes (x = u, y = v) and of the `solid` hints. "
)

defs["Vector2"] = {
    "description": "Two numbers [a, b]: a coordinate pair in the plane of a profile, in metres.",
    "type": "array",
    "items": num,
    "minItems": 2,
    "maxItems": 2,
}
defs["Vector3"] = {
    "description": "Three numbers [u, v, w]: a point or direction in the bone-local frame, in metres for points.",
    "type": "array",
    "items": num,
    "minItems": 3,
    "maxItems": 3,
}
defs["Quaternion"] = {
    "description": "A rotation quaternion [x, y, z, w] of unit length (validator `volume-transform`); the identity is [0, 0, 0, 1].",
    "type": "array",
    "items": num,
    "minItems": 4,
    "maxItems": 4,
}
defs["VolumeTransform"] = {
    "description": "An optional placement of one volume solid inside the bone-local frame, applied to the solid's own coordinates in the order scale, rotation, translation; each part absent means identity. scale is a per-axis scale about the bone-local origin of the solid (u, v, w; every factor positive), rotation a unit quaternion about that origin, translation in metres.",
    "type": "object",
    "additionalProperties": False,
    "properties": {
        "translation": ref("Vector3"),
        "rotation": ref("Quaternion"),
        "scale": {"type": "array", "items": pos, "minItems": 3, "maxItems": 3},
    },
}
defs["VolumeRepeat"] = {
    "description": "Repeats a volume solid as `count` copies; copy k (k = 0 .. count-1) is the solid after `transform`, moved by the step applied k times: first the rotation of k * rotationRad about the axis rotationAxis through the point origin (default [0, 0, 0]), then k * translation. count is at least 1 and at most 256 (validator `volume-repeat`); absent rotationRad, rotationAxis or translation mean no rotation or no translation. Copies inherit every other field of the solid, including its weights.",
    "type": "object",
    "additionalProperties": False,
    "required": ["count"],
    "properties": {
        "count": {"type": "integer", "minimum": 1, "maximum": 256},
        "translation": ref("Vector3"),
        "rotationAxis": ref("Vector3"),
        "rotationRad": num,
        "origin": ref("Vector3"),
    },
}

common_required = ["id", "kind", "bone", "material", "role", "tone"]
common = {
    "id": {"description": "Unique among the solids of the volume (validator `volume-duplicate-id`).", "$ref": "#/$defs/Slug"},
    "bone": {"description": "The bone whose rest frame carries the coordinates of this solid and which the solid follows rigidly unless `weights` blend it (validator `volume-bone`).", "$ref": "#/$defs/Slug"},
    "material": {"description": "The id of an entry of the asset's `materials` that colours and, in the thermal view, binds the solid (validator `volume-material`).", "$ref": "#/$defs/Slug"},
    "role": {"description": "The line-hierarchy role of the solid in the line views: body for the main mass, detail for small parts, texture, glazing for transparent parts, ground for contact surfaces.", "$ref": "#/$defs/ShapeRole"},
    "tone": {"description": "The grey tone 0 (black) to 1 (white) the solid takes in the solid views of the dark and light themes, like the tone of a shape.", "type": "number", "minimum": 0, "maximum": 1},
    "region": {"description": "The thermal region this solid's surface belongs to (validator `volume-region`); a solid with neither region nor regions belongs to no thermal region.", "$ref": "#/$defs/Slug"},
    "regions": {"description": "Several thermal regions a merged solid belongs to (validator `volume-region`).", "type": "array", "items": ref("Slug"), "minItems": 1},
    "maxDenominator": {"description": "The coarsest preset drawing scale at which the solid is still modelled, as the largest denominator that shows it (for example 50 for a detail that exists only at 1:50, 500 for a solid shown at every scale); absent: shown at every scale (validator `volume-max-denominator`).", "$ref": "#/$defs/ScaleDenominator"},
    "weights": {
        "description": "Blend of the solid over bones (skinning): at most four influences, each a bone and a non-negative weight, summing to 1 (validators `volume-weights-bone` and `volume-weights-sum`); absent: the solid is rigid to `bone`. The coordinates stay those of the frame of `bone`.",
        "type": "array",
        "items": {
            "type": "object",
            "additionalProperties": False,
            "required": ["bone", "weight"],
            "properties": {"bone": ref("Slug"), "weight": nonneg},
        },
        "minItems": 1,
        "maxItems": 4,
    },
    "transform": ref("VolumeTransform"),
    "repeat": ref("VolumeRepeat"),
    "tags": {"description": "Free labels of the solid; the label `glyph` is reserved for generated analysis glyphs.", "type": "array", "items": ref("Slug"), "uniqueItems": True},
}
profile = lambda minimum: {"type": "array", "items": ref("Vector2"), "minItems": minimum}


def solid(kind, description, properties, required):
    return {
        "description": description,
        "type": "object",
        "additionalProperties": False,
        "required": common_required + required,
        "properties": {"kind": {"const": kind}, **common, **properties},
    }


defs["VolumeSolid"] = {
    "description": "One declarative solid of a volume, bound to a bone of the rig. "
    + FRAME
    + "The pose of the bone carries the solid: it follows the rig through every pose and clip. Six kinds: "
    "box (centred axis-aligned box with optionally rounded edges), ellipsoid (centred ellipsoid with the semi-axes along u, v, w), capsule (hull of two spheres, optionally elliptic), "
    "revolve (a profile of [axial, radial] revolved about an axis), extrude (a polygon with holes on a coordinate plane pushed along the third axis, optionally tapered) and "
    "loft (superelliptic sections joined in order). transform places the solid, then repeat copies it. Every size and radius is positive and every profile is a simple polygon (validators `volume-size`, `volume-profile-degenerate`, `volume-profile-self-intersecting`, `volume-hole-outside`, `volume-holes-overlap`, `volume-extrude-range`, `volume-loft-sections`).",
    "oneOf": [
        solid(
            "box",
            "A box centred on the origin of the bone-local frame with edges along u, v, w; size is the edge length along u, v, w; cornerRadius rounds every edge and is at most half the smallest edge.",
            {"size": {"type": "array", "items": pos, "minItems": 3, "maxItems": 3}, "cornerRadius": nonneg},
            ["size"],
        ),
        solid(
            "ellipsoid",
            "An ellipsoid centred on the origin of the bone-local frame with the semi-axes radii = [ru, rv, rw] along u, v, w; move it with transform.translation.",
            {"radii": {"type": "array", "items": pos, "minItems": 3, "maxItems": 3}},
            ["radii"],
        ),
        solid(
            "capsule",
            "The hull of the sphere (p0, r0) and the sphere (p1, r1): a limb or tube with round ends; one radius may be 0 for a cone ending in a point, not both (validator `volume-size`). lateralScale stretches the cross-section along w (perpendicular to the axis p0 to p1; along v when the axis is parallel to w) by this factor relative to the radius, 1 being a circle; the end caps stretch with it.",
            {"p0": ref("Vector3"), "p1": ref("Vector3"), "r0": nonneg, "r1": nonneg, "lateralScale": pos},
            ["p0", "p1", "r0", "r1"],
        ),
        solid(
            "revolve",
            "A profile of points [axial, radial] revolved about the axis through the point origin with the direction axis (not the zero vector): axial is the signed distance along the axis from origin, radial the non-negative distance from it. sweepRad is the angle swept from the plane through the axis and the sagittal normal, in (0, 2 pi], default a full turn; an open sweep is closed by flat faces. lateralScale stretches the cross-section along w (along v when the axis is parallel to w) relative to the radius, 1 being circular. The profile has at least two points and spans a non-zero area against the axis.",
            {"profile": profile(2), "axis": ref("Vector3"), "origin": ref("Vector3"), "sweepRad": {"type": "number", "exclusiveMinimum": 0, "maximum": 6.283185307179586}, "lateralScale": pos},
            ["profile", "axis", "origin"],
        ),
        solid(
            "extrude",
            "A simple polygon profile (at least three points, optional holes lying inside it) on a coordinate plane of the bone-local frame pushed along the third axis from from to to: plane xy has the profile coordinates (u, v) and extrudes along w, yz has (v, w) and extrudes along u, zx has (w, u) and extrudes along v; from < to (validator `volume-extrude-range`). taperTo = [scaleFrom, scaleTo] scales the profile about the origin of its plane at from and at to (non-negative; absent means 1 and 1).",
            {"profile": profile(3), "holes": {"type": "array", "items": profile(3)}, "plane": {"enum": ["xy", "yz", "zx"]}, "from": num, "to": num, "taperTo": {"type": "array", "items": nonneg, "minItems": 2, "maxItems": 2}},
            ["profile", "plane", "from", "to"],
        ),
        solid(
            "loft",
            "Superelliptic sections joined in order into one skin, at least two, whose centres at give the path. Each section lies in the plane perpendicular to the path tangent there (the direction to the next centre, for the last the direction from the previous one); its first axis is the bone-local w projected perpendicular to the tangent (v where the tangent is parallel to w) and carries rx, its second axis completes the frame and carries ry. exponent n shapes the section |x/rx|^n + |y/ry|^n = 1: 2 an ellipse (default), larger values a rounded rectangle, 1 a diamond; rotation is a unit quaternion turning the section about its centre in the bone-local frame. capStart and capEnd close the first and last section with a flat face.",
            {
                "sections": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "additionalProperties": False,
                        "required": ["at", "rx", "ry"],
                        "properties": {"at": ref("Vector3"), "rx": pos, "ry": pos, "exponent": pos, "rotation": ref("Quaternion")},
                    },
                    "minItems": 2,
                },
                "capStart": {"type": "boolean"},
                "capEnd": {"type": "boolean"},
            },
            ["sections", "capStart", "capEnd"],
        ),
    ],
}
defs["Volume"] = {
    "description": "The declarative 3D volume of an asset: solids bound to the bones of its rig in the bone-local rest frame, from which the engine-neutral model, the GLB and FBX files and the axonometric view are built. It is authored once for every scale: a solid's maxDenominator drops fine detail from coarse scales. The silhouette of the solids seen along each viewpoint agrees with the authored shapes of the representations, and the bounding box of the solids at rest agrees with `dimensions`.",
    "type": "object",
    "additionalProperties": False,
    "required": ["status", "solids", "decisions"],
    "properties": {
        "status": {"const": "authored"},
        "solids": {"type": "array", "items": ref("VolumeSolid"), "minItems": 1},
        "decisions": {"description": "The authoring decisions of the volume: what each group of solids stands for, which simplifications were made and why.", "type": "array", "items": ref("Text"), "minItems": 1},
    },
}

for fmt in ("glb", "fbx", "model"):
    if fmt not in defs["ExportFormat"]["enum"]:
        defs["ExportFormat"]["enum"].append(fmt)
defs["ExportFormat"]["description"] = "The output kinds an asset can be exported to; glb is a glTF 2.0 binary with every pose and clip, fbx an FBX 7.4 binary of one clip, model the engine-neutral JSON model document."
if "axonometric" not in defs["Viewpoint"]["enum"]:
    defs["Viewpoint"]["enum"].append("axonometric")
defs["Viewpoint"]["description"] = (
    "The projection an asset is drawn in: the side elevation (the rig plane, seen from the right of the asset), the front elevation (seen from ahead), the plan (top view, seen from above), "
    "or the axonometric view of the 3D volume; only the first three have authored shapes, the axonometric view is derived from `volume`."
)

props = defs["Asset"]["properties"]
if "volume" not in props:
    out = {}
    for key, value in props.items():
        out[key] = value
        if key == "rig":
            out["volume"] = {"description": "The declarative 3D volume bound to the rig (not yet required; it becomes required once every catalog asset has one).", "$ref": "#/$defs/Volume"}
    defs["Asset"]["properties"] = out

order = list(defs)
for name in ("Vector2", "Vector3", "Quaternion", "VolumeTransform", "VolumeRepeat", "VolumeSolid", "Volume"):
    order.remove(name)
at = order.index("Dimensions")
order[at:at] = ["Vector2", "Vector3", "Quaternion", "VolumeTransform", "VolumeRepeat", "VolumeSolid", "Volume"]
schema["$defs"] = {name: defs[name] for name in order}
open(path, "w", encoding="utf8").write(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
