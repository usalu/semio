import json, io

DEFS = {
    "StepValue": {
        "description": "step's own typed Part-21 argument value (see StepSnapshot's own docstring). Unit variants (unset/derived) are bare strings; data-carrying variants are a one-key object keyed by the camelCased variant name, matching value_derive's default externally-tagged enum representation.",
        "anyOf": [
            {"const": "unset"},
            {"const": "derived"},
            {"type": "object", "additionalProperties": False, "required": ["integer"], "properties": {"integer": {"type": "integer"}}},
            {"type": "object", "additionalProperties": False, "required": ["real"], "properties": {"real": {"type": "number"}}},
            {"type": "object", "additionalProperties": False, "required": ["string"], "properties": {"string": {"type": "string"}}},
            {"type": "object", "additionalProperties": False, "required": ["enum"], "properties": {"enum": {"type": "string"}}},
            {"type": "object", "additionalProperties": False, "required": ["reference"], "properties": {"reference": {"type": "integer", "minimum": 0}}},
            {"type": "object", "additionalProperties": False, "required": ["aggregate"], "properties": {"aggregate": {"type": "array", "items": {"$ref": "#/$defs/StepValue"}}}},
            {"type": "object", "additionalProperties": False, "required": ["typedValue"], "properties": {"typedValue": {"type": "object", "additionalProperties": False, "required": ["typeName", "value"], "properties": {"typeName": {"type": "string"}, "value": {"$ref": "#/$defs/StepValue"}}}}},
        ],
    },
    "StepComplexType": {
        "type": "object",
        "additionalProperties": False,
        "required": ["name"],
        "properties": {"name": {"type": "string"}, "args": {"type": "array", "items": {"$ref": "#/$defs/StepValue"}}},
    },
    "StepEntity": {
        "type": "object",
        "additionalProperties": False,
        "required": ["id", "name"],
        "properties": {
            "id": {"type": "integer", "minimum": 0},
            "name": {"type": "string"},
            "args": {"type": "array", "items": {"$ref": "#/$defs/StepValue"}},
            "complex": {"type": "array", "items": {"$ref": "#/$defs/StepComplexType"}},
        },
    },
    "StepFileDescription": {
        "type": "object",
        "additionalProperties": False,
        "required": [],
        "properties": {"description": {"type": "array", "items": {"type": "string"}}, "implementationLevel": {"type": "string"}},
    },
    "StepFileName": {
        "type": "object",
        "additionalProperties": False,
        "required": [],
        "properties": {
            "name": {"type": "string"},
            "timestamp": {"type": "string"},
            "author": {"type": "array", "items": {"type": "string"}},
            "organization": {"type": "array", "items": {"type": "string"}},
            "preprocessorVersion": {"type": "string"},
            "originatingSystem": {"type": "string"},
            "authorization": {"type": "string"},
        },
    },
    "StepFileSchema": {
        "type": "object",
        "additionalProperties": False,
        "required": [],
        "properties": {"schemas": {"type": "array", "items": {"type": "string"}}},
    },
    "StepHeader": {
        "type": "object",
        "additionalProperties": False,
        "required": [],
        "properties": {
            "fileDescription": {"$ref": "#/$defs/StepFileDescription"},
            "fileName": {"$ref": "#/$defs/StepFileName"},
            "fileSchema": {"$ref": "#/$defs/StepFileSchema"},
        },
    },
    "StepSnapshot": {
        "type": "object",
        "additionalProperties": False,
        "required": ["schema"],
        "properties": {
            "schema": {"type": "string"},
            "header": {"$ref": "#/$defs/StepHeader"},
            "entities": {"type": "array", "items": {"$ref": "#/$defs/StepEntity"}},
        },
    },
}


def schema(title, required, properties, needs):
    d = {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "additionalProperties": False,
        "required": required,
        "properties": properties,
    }
    if needs:
        d["$defs"] = {k: DEFS[k] for k in needs}
    d["title"] = title
    return d


MUTATIONS = {
    "📄set-snapshot": schema("SetSnapshot", ["snapshot"], {"snapshot": {"$ref": "#/$defs/StepSnapshot"}}, ["StepSnapshot", "StepHeader", "StepFileDescription", "StepFileName", "StepFileSchema", "StepEntity", "StepComplexType", "StepValue"]),
    "🗑remove-entity": schema("RemoveEntity", ["id"], {"id": {"type": "integer", "minimum": 0}}, []),
    "🧩insert-entity": schema("InsertEntity", ["index", "entity"], {"index": {"type": "integer", "minimum": 0}, "entity": {"$ref": "#/$defs/StepEntity"}}, ["StepEntity", "StepComplexType", "StepValue"]),
    "➕insert-entity-arg": schema("InsertEntityArg", ["id", "arg_index", "value"], {"id": {"type": "integer", "minimum": 0}, "arg_index": {"type": "integer", "minimum": 0}, "value": {"$ref": "#/$defs/StepValue"}}, ["StepValue"]),
    "➖remove-entity-arg": schema("RemoveEntityArg", ["id", "arg_index"], {"id": {"type": "integer", "minimum": 0}, "arg_index": {"type": "integer", "minimum": 0}}, []),
    "🔧set-entity-arg": schema("SetEntityArg", ["id", "arg_index", "value"], {"id": {"type": "integer", "minimum": 0}, "arg_index": {"type": "integer", "minimum": 0}, "value": {"$ref": "#/$defs/StepValue"}}, ["StepValue"]),
    "✏set-entity-name": schema("SetEntityName", ["id", "name"], {"id": {"type": "integer", "minimum": 0}, "name": {"type": "string"}}, []),
    "🏷set-file-schema": schema("SetFileSchema", ["file_schema"], {"file_schema": {"$ref": "#/$defs/StepFileSchema"}}, ["StepFileSchema"]),
    "📝set-file-description": schema("SetFileDescription", ["file_description"], {"file_description": {"$ref": "#/$defs/StepFileDescription"}}, ["StepFileDescription"]),
    "📛set-file-name": schema("SetFileName", ["file_name"], {"file_name": {"$ref": "#/$defs/StepFileName"}}, ["StepFileName"]),
}

ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️base/🧬️schema/🧬️mutations"
for dirname, sch in MUTATIONS.items():
    path = f"{ROOT}/{dirname}/🔣️.schema.json"
    with io.open(path, "w", encoding="utf-8") as f:
        f.write(json.dumps(sch, indent=2, ensure_ascii=False) + "\n")
    print("wrote", path)
