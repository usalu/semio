import json, sys
BASE = "https://json.schemas.assets.semio-tech.com/s/oracle/"
S = "http://json-schema.org/draft-07/schema#"

def doc(name, defs):
    return {"$schema": S, "$id": f"{BASE}{name}.json", "title": name, "$defs": defs}

point = {"title": "Point", "type": "object", "additionalProperties": False, "required": ["x", "y"], "properties": {"x": {"type": "number"}, "y": {"type": "number"}}}
cases = []

cases.append({
    "name": "scalars",
    "documents": [{"name": "scalars", "seed": "", "schema": doc("scalars", {
        "Label": {"type": "string", "minLength": 1, "maxLength": 4, "pattern": "^[a-z]+$"},
        "Count": {"type": "integer", "minimum": 0, "maximum": 10},
        "Whole": {"type": "integer"},
        "Ratio": {"type": "number", "minimum": 0, "maximum": 1},
        "Flag": {"type": "boolean"},
        "Mode": {"enum": ["fast", "slow"]},
        "Fixed": {"const": "v1"},
    })}],
    "probes": [
        {"document": "scalars", "export": "Label", "valid": ["abc", "a", "abcd"], "invalid": ["", "abcde", "ABC", "a1", 1, None, True, [], {}]},
        {"document": "scalars", "export": "Count", "valid": [0, 10, 5], "invalid": [-1, 11, 1.5, "1", None, True, [], {}]},
        {"document": "scalars", "export": "Whole", "valid": [0, -5, 9007199254740991], "invalid": [1.5, "1", None, False], "stricter": [1e300]},
        {"document": "scalars", "export": "Ratio", "valid": [0, 0.5, 1], "invalid": [-0.1, 1.1, "0", None, True]},
        {"document": "scalars", "export": "Flag", "valid": [True, False], "invalid": [0, 1, "true", None, []]},
        {"document": "scalars", "export": "Mode", "valid": ["fast", "slow"], "invalid": ["other", "", 1, None, []]},
        {"document": "scalars", "export": "Fixed", "valid": ["v1"], "invalid": ["v2", "", 1, None]},
    ],
})

cases.append({
    "name": "objects",
    "documents": [{"name": "objects", "seed": "", "schema": doc("objects", {
        "Point": point,
        "Shape": {"title": "Shape", "type": "object", "additionalProperties": False, "required": ["name", "points"], "properties": {
            "name": {"type": "string"},
            "points": {"type": "array", "minItems": 1, "maxItems": 3, "items": {"$ref": "#/$defs/Point"}},
            "tags": {"type": "array", "items": {"type": "string"}},
            "origin": {"$ref": "#/$defs/Point"},
        }},
    })}],
    "probes": [
        {"document": "objects", "export": "Point", "valid": [{"x": 0, "y": 0}, {"x": -1.5, "y": 2}], "invalid": [{}, {"x": 0}, {"x": "0", "y": 0}, {"x": 0, "y": None}, [], None, 1], "lenient": [{"x": 0, "y": 0, "z": 1}]},
        {"document": "objects", "export": "Shape", "valid": [
            {"name": "a", "points": [{"x": 0, "y": 0}]},
            {"name": "b", "points": [{"x": 0, "y": 0}, {"x": 1, "y": 1}, {"x": 2, "y": 2}], "tags": ["t", "u"], "origin": {"x": 9, "y": 9}}],
         "invalid": [{}, {"name": "a"}, {"name": 1, "points": [{"x": 0, "y": 0}]}, {"name": "a", "points": []},
                     {"name": "a", "points": [{"x": 0, "y": 0}, {"x": 0, "y": 0}, {"x": 0, "y": 0}, {"x": 0, "y": 0}]},
                     {"name": "a", "points": [{"x": 0}]}, {"name": "a", "points": [{"x": 0, "y": "1"}]},
                     {"name": "a", "points": [{"x": 0, "y": 0}], "tags": [1]}, {"name": "a", "points": [{"x": 0, "y": 0}], "origin": {}}, None, 1, "x"],
         "lenient": [{"name": "a", "points": [{"x": 0, "y": 0}], "extra": 1}]},
    ],
})

cases.append({
    "name": "nullable",
    "documents": [{"name": "nullable", "seed": "", "schema": doc("nullable", {
        "MaybeText": {"type": ["string", "null"], "maxLength": 3},
        "MaybeCount": {"anyOf": [{"type": "integer", "minimum": 0}, {"type": "null"}]},
        "Holder": {"title": "Holder", "type": "object", "additionalProperties": False, "required": ["text"], "properties": {"text": {"$ref": "#/$defs/MaybeText"}, "count": {"$ref": "#/$defs/MaybeCount"}}},
    })}],
    "probes": [
        {"document": "nullable", "export": "MaybeText", "valid": ["a", None, "abc", ""], "invalid": ["abcd", 1, True, [], {}]},
        {"document": "nullable", "export": "MaybeCount", "valid": [0, None, 3], "invalid": [-1, "1", 1.5, True, []]},
        {"document": "nullable", "export": "Holder", "valid": [{"text": None}, {"text": "a", "count": 2}, {"text": "a", "count": None}], "invalid": [{}, {"text": 1}, {"text": "abcd"}, {"text": "a", "count": -1}, None]},
    ],
})

cases.append({
    "name": "unions",
    "documents": [{"name": "unions", "seed": "export type Shape2 = { readonly kind: \"circle\"; readonly radius: number } | { readonly kind: \"square\"; readonly side: number };\nexport type Edit = { readonly insert: string } | { readonly remove: number };\n", "schema": doc("unions", {
        "Shape2": {"oneOf": [
            {"type": "object", "additionalProperties": False, "required": ["kind", "radius"], "properties": {"kind": {"const": "circle"}, "radius": {"type": "number"}}},
            {"type": "object", "additionalProperties": False, "required": ["kind", "side"], "properties": {"kind": {"const": "square"}, "side": {"type": "number"}}},
        ]},
        "Edit": {"oneOf": [
            {"type": "object", "additionalProperties": False, "required": ["insert"], "properties": {"insert": {"type": "string"}}},
            {"type": "object", "additionalProperties": False, "required": ["remove"], "properties": {"remove": {"type": "integer", "minimum": 0}}},
        ]},
    })}],
    "probes": [
        {"document": "unions", "export": "Shape2", "valid": [{"kind": "circle", "radius": 1}, {"kind": "square", "side": 2}], "invalid": [{"kind": "triangle"}, {"kind": "circle"}, {"kind": "circle", "radius": "1"}, {"kind": "square", "radius": 1}, {}, None, 3, "circle"], "lenient": [{"kind": "circle", "radius": 1, "extra": True}]},
        {"document": "unions", "export": "Edit", "valid": [{"insert": "a"}, {"remove": 0}], "invalid": [{}, {"insert": 1}, {"remove": -1}, {"other": 1}, None, "x", []], "lenient": [{"insert": "a", "remove": 1}]},
    ],
})

cases.append({
    "name": "tuples",
    "documents": [{"name": "tuples", "seed": "export type Pair = [number, number];\nexport type Triple = [string, number, boolean];\n", "schema": doc("tuples", {
        "Pair": {"type": "array", "items": [{"type": "number"}, {"type": "number"}], "minItems": 2, "maxItems": 2},
        "Triple": {"type": "array", "items": [{"type": "string"}, {"type": "number"}, {"type": "boolean"}]},
    })}],
    "probes": [
        {"document": "tuples", "export": "Pair", "valid": [[1, 2], [0.5, -1]], "invalid": [[1], [1, 2, 3], [1, "2"], ["1", 2], "ab", None, {}, []]},
        {"document": "tuples", "export": "Triple", "valid": [["a", 1, True]], "invalid": [["a", "1", True], [1, 1, True], "abc", None], "stricter": [["a", 1], ["a", 1, True, False], []]},
    ],
})

cases.append({
    "name": "maps",
    "documents": [{"name": "maps", "seed": "", "schema": doc("maps", {
        "Point": point,
        "Weights": {"type": "object", "additionalProperties": {"type": "number", "minimum": 0}},
        "Index": {"type": "object", "additionalProperties": {"$ref": "#/$defs/Point"}},
    })}],
    "probes": [
        {"document": "maps", "export": "Weights", "valid": [{}, {"a": 1}, {"a": 0, "b": 2.5}], "invalid": [{"a": -1}, {"a": "x"}, {"a": None}, [], None, 1]},
        {"document": "maps", "export": "Index", "valid": [{}, {"p": {"x": 1, "y": 2}}], "invalid": [{"p": {"x": 1}}, {"p": 1}, {"p": {"x": 1, "y": "2"}}, [], None]},
    ],
    "rawProbes": [
        {"document": "maps", "export": "Weights", "text": "{\"__proto__\": 1, \"constructor\": 2}", "valid": True},
        {"document": "maps", "export": "Weights", "text": "{\"__proto__\": -1}", "valid": False},
    ],
})

cases.append({
    "name": "opaque",
    "documents": [{"name": "opaque", "seed": "export type Anything = readonly unknown[];\n", "schema": doc("opaque", {
        "Bag": {"type": "object"},
        "Anything": {"type": "array"},
    })}],
    "probes": [
        {"document": "opaque", "export": "Bag", "valid": [{}, {"a": 1}, {"a": {"b": [1, None]}}], "invalid": [[], None, 1, "x", True]},
        {"document": "opaque", "export": "Anything", "valid": [[], [1, "a", None, {"k": 1}]], "invalid": [{}, None, 1, "x"]},
    ],
})

cases.append({
    "name": "references",
    "documents": [
        {"name": "other", "seed": "", "schema": doc("other", {"Pt": {"title": "Pt", "type": "object", "additionalProperties": False, "required": ["x"], "properties": {"x": {"type": "integer"}}}})},
        {"name": "holder", "seed": "import type { Pt } from \"../../other/\U0001F9EC\ufe0fschema/\U0001F7E6\ufe0f.ts\";\n\nexport interface Holder {\n  readonly p: Pt;\n}\n\nexport type Wrapped = Pt;\n", "schema": doc("holder", {
            "Holder": {"title": "Holder", "type": "object", "additionalProperties": False, "required": ["p"], "properties": {"p": {"$ref": f"{BASE}other.json#/$defs/Pt"}}},
            "Wrapped": {"allOf": [{"$ref": f"{BASE}other.json#/$defs/Pt"}]},
        })},
    ],
    "probes": [
        {"document": "holder", "export": "Holder", "valid": [{"p": {"x": 1}}], "invalid": [{}, {"p": {}}, {"p": 1}, {"p": {"x": 1.5}}, None]},
        {"document": "holder", "export": "Wrapped", "valid": [{"x": 3}], "invalid": [{}, {"x": "3"}, None, []]},
    ],
})

cases.append({
    "name": "declared-types",
    "documents": [{"name": "declared", "seed": "export interface Payload {\n  readonly count: number;\n}\n\nexport interface Record3 {\n  readonly id: string;\n  readonly payload: Payload;\n  readonly tags: string[];\n}\n", "schema": doc("declared", {
        "Payload": {"title": "Payload", "type": "object", "additionalProperties": False, "required": ["count"], "properties": {"count": {"type": "integer", "minimum": 0}}},
        "Record3": {"title": "Record3", "type": "object", "additionalProperties": False, "required": ["id", "payload", "tags"], "properties": {"id": {"type": "string"}, "payload": {"type": "object"}, "tags": {"type": "array"}}},
    })}],
    "probes": [
        {"document": "declared", "export": "Record3", "valid": [{"id": "a", "payload": {"count": 1}, "tags": ["x"]}, {"id": "b", "payload": {"count": 0}, "tags": []}], "invalid": [{}, {"id": 1, "payload": {"count": 1}, "tags": []}, {"id": "a", "payload": [], "tags": []}, {"id": "a", "payload": {"count": 1}}],
         "stricter": [{"id": "a", "payload": {}, "tags": []}, {"id": "a", "payload": {"count": 1}, "tags": [1]}, {"id": "a", "payload": {"count": -1}, "tags": []}]},
    ],
})

json.dump({"description": "Language-agnostic corpus of the schema parser projection: each case lists schema documents (with the hand-written TypeScript a leaf already declares), and per export the instances every implementation must accept (valid), refuse (invalid), the ones the normative schema accepts but a projection may refuse because it follows a declared type or a safe-integer rule (stricter), and the ones the schema refuses but the projection tolerates (lenient: unknown properties).", "cases": cases}, sys.stdout, ensure_ascii=False, indent=2)
print()
