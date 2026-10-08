"""Writes the draft-07 JSON Schemas (schema-first contracts) of the geometry fixtures next to them: <domain>/🧬️schema/🔣️.json."""
import json
import os

ROOT = r"C:\git\semio\🧰️framework\🔨️modules\📐️geometry"
NL = chr(10)

num = {"type": "number"}
point = {"type": "array", "minItems": 2, "maxItems": 2, "items": num}
xyz = {"type": "array", "minItems": 3, "maxItems": 3, "items": num}
ring = {"type": "array", "minItems": 3, "items": point}
name = {"type": "string", "minLength": 1}
pair = {"type": "array", "minItems": 2, "maxItems": 2, "items": num}
box2 = {"type": "array", "minItems": 4, "maxItems": 4, "items": num}
box3 = {"type": "array", "minItems": 2, "maxItems": 2, "items": xyz}
vertex = {"type": "array", "minItems": 3, "maxItems": 3, "items": num}
loop = {"type": "array", "minItems": 2, "items": vertex}
seg = {"type": "object", "required": ["start", "end", "bulge"], "additionalProperties": False, "properties": {"start": point, "end": point, "bulge": num}}


def nullable(s):
    return {"oneOf": [s, {"type": "null"}]}


def obj(required, properties):
    return {"type": "object", "required": required, "additionalProperties": False, "properties": properties}


def arr(items):
    return {"type": "array", "items": items}


def schema(domain, title, body):
    doc = {"$schema": "http://json-schema.org/draft-07/schema#", "$id": "https://semio.tech/framework/geometry/" + title, **body}
    folder = os.path.join(ROOT, domain, "🧬️schema")
    os.makedirs(folder, exist_ok=True)
    with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8", newline=NL) as handle:
        json.dump(doc, handle, indent=2, ensure_ascii=False)
        handle.write(NL)


arc_expected = obj(["sweep", "radius", "center", "length", "mid", "segment_area"], {"sweep": num, "radius": nullable(num), "center": nullable(point), "length": num, "mid": point, "segment_area": num, "bounds": box2})
schema("🌙️bulge", "bulge", obj(["arcs", "offsets", "intersections", "closest", "three_points", "bands", "corners"], {
    "arcs": arr(obj(["name", "start", "end", "bulge", "expected"], {"name": name, "start": point, "end": point, "bulge": num, "expected": arc_expected})),
    "offsets": arr(obj(["name", "seg", "distance", "expected"], {"name": name, "seg": seg, "distance": num, "expected": nullable(seg)})),
    "intersections": arr(obj(["name", "a", "b", "extent", "expected"], {"name": name, "a": seg, "b": seg, "extent": {"enum": ["bounded", "unbounded"]}, "expected": arr(point)})),
    "closest": arr(obj(["name", "seg", "point", "expected"], {"name": name, "seg": seg, "point": point, "expected": obj(["t", "point", "distance"], {"t": num, "point": point, "distance": num})})),
    "three_points": arr(obj(["name", "a", "through", "b", "expected"], {"name": name, "a": point, "through": point, "b": point, "expected": nullable(obj(["center", "radius", "sweep"], {"center": point, "radius": num, "sweep": num}))})),
    "bands": arr(obj(["name", "axis", "left", "right", "expected", "area"], {"name": name, "axis": seg, "left": num, "right": num, "expected": {"type": "array", "minItems": 4, "maxItems": 4, "items": xyz}, "area": num})),
    "corners": arr(obj(["name", "prev", "prev_widths", "next", "next_widths", "expected"], {"name": name, "prev": seg, "prev_widths": pair, "next": seg, "next_widths": pair, "expected": obj(["left", "right"], {"left": nullable(point), "right": nullable(point)})})),
}))

loop_expected = obj(["signed_area", "area", "perimeter", "centroid", "bounds", "ccw"], {"signed_area": num, "area": num, "perimeter": num, "centroid": point, "bounds": box2, "ccw": {"type": "boolean"}})
schema("➰️loops", "loops", obj(["loops", "offsets"], {
    "loops": arr(obj(["name", "vertices", "expected", "contains"], {"name": name, "vertices": loop, "expected": loop_expected, "contains": arr(obj(["point", "at"], {"point": point, "at": {"enum": ["inside", "outside", "boundary"]}}))})),
    "offsets": arr(obj(["name", "vertices", "distance", "miter_limit", "expected_area", "expected_perimeter"], {"name": name, "vertices": loop, "distance": num, "miter_limit": {"type": "number", "minimum": 1}, "expected_area": nullable(num), "expected_perimeter": nullable(num)})),
}))

schema("🔺️triangulation", "triangulation", arr(obj(["name", "outer", "holes", "area", "triangles"], {"name": name, "outer": ring, "holes": arr(ring), "area": {"type": "number", "minimum": 0}, "triangles": {"type": "integer", "minimum": 0}})))

mesh_expected = obj(["volume", "area", "bounds", "centroid", "watertight"], {"volume": num, "area": num, "bounds": box3, "centroid": xyz, "watertight": {"type": "boolean"}})
sweep_case = obj(["name", "profile", "path", "base_z", "tolerance", "volume", "area", "bounds"], {"name": name, "profile": ring, "holes": arr(ring), "path": arr(seg), "base_z": num, "tolerance": {"type": "number", "exclusiveMinimum": 0}, "volume": pair, "area": nullable(pair), "bounds": box3})
face = obj(["outer", "holes"], {"outer": ring, "holes": arr(ring)})
wall_case = obj(["name", "axis", "left", "right", "axis_length", "base_z", "tolerance", "left_face", "right_face", "volume", "area", "watertight"], {"name": name, "axis": seg, "left": num, "right": num, "axis_length": {"type": "number", "exclusiveMinimum": 0}, "base_z": num, "tolerance": {"type": "number", "exclusiveMinimum": 0}, "left_face": face, "right_face": face, "volume": pair, "area": nullable(pair), "watertight": {"type": "boolean"}})
schema("🕸️mesh", "mesh", obj(["extrusions", "loop_extrusions", "prisms", "sweeps", "walls"], {
    "extrusions": arr(obj(["name", "outer", "holes", "bottom", "top", "expected"], {"name": name, "outer": ring, "holes": arr(ring), "bottom": xyz, "top": xyz, "expected": mesh_expected})),
    "loop_extrusions": arr(obj(["name", "outer", "holes", "tolerance", "bottom", "top", "volume", "area", "watertight"], {"name": name, "outer": loop, "holes": arr(loop), "tolerance": {"type": "number", "exclusiveMinimum": 0}, "bottom": xyz, "top": xyz, "volume": pair, "area": pair, "watertight": {"type": "boolean"}})),
    "prisms": arr(obj(["name", "lower", "upper", "volume", "area"], {"name": name, "lower": {"type": "array", "minItems": 3, "items": xyz}, "upper": {"type": "array", "minItems": 3, "items": xyz}, "volume": num, "area": num})),
    "sweeps": arr(sweep_case),
    "walls": arr(wall_case),
}))

schema("🔪️section", "section", arr(obj(["name", "outer", "holes", "bottom", "top", "z", "expected"], {"name": name, "outer": ring, "holes": arr(ring), "bottom": num, "top": num, "z": num, "expected": arr(obj(["closed", "area", "points"], {"closed": {"type": "boolean"}, "area": num, "points": {"type": "integer", "minimum": 2}}))})))

op = {"type": "object", "minProperties": 1, "additionalProperties": False, "properties": {"rotation_z": num, "translation": xyz, "scaling": xyz, "rotation_axis": xyz, "angle": num}}
schema("🧭️placement", "placement", obj(["placements", "z_planes"], {
    "placements": arr(obj(["name", "ops", "point", "expected"], {"name": name, "ops": arr(op), "point": xyz, "expected": xyz})),
    "z_planes": arr(obj(["name", "anchor", "z", "direction", "slope", "queries"], {"name": name, "anchor": point, "z": num, "direction": num, "slope": num, "queries": arr(obj(["point", "z"], {"point": point, "z": num}))})),
}))
print("schemas written")
