"""🎚️ W0-I: adds the `ToolRunDefinition.settings` declaration (RFC 6901 pointers) to the tool-run schema of record and the
lifecycle-law fixture (definition sample plus the `settingsPointers` cases). Idempotent."""
import json
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏯️tool-run")
SCHEMA = ROOT / "🧬️schema/🔣️.json"
FIXTURE = ROOT / "🧫️fixtures/⚖️lifecycle-law.json"

schema = json.loads(SCHEMA.read_text())
defs = schema["$defs"]
defs["ToolRunSettingsPointer"] = {
    "title": "ToolRunSettingsPointer",
    "description": "RFC 6901 JSON Pointer into a settings document: empty, or `/`-separated reference tokens where `~` only appears as `~0` or `~1`.",
    "type": "string",
    "pattern": "^(/([^~/]|~[01])*)*$",
}
defs["ToolRunSettingsReads"] = {
    "title": "ToolRunSettingsReads",
    "description": "The settings a run's jobs read (§3.3): pointers into the app config document and, per window kind id, into that kind's window config documents. `settingsChanged` fires only when a value behind one of them changes; absent or empty reads no settings.",
    "type": "object",
    "additionalProperties": False,
    "properties": {
        "config": {"type": "array", "items": {"$ref": "#/$defs/ToolRunSettingsPointer"}},
        "windowConfig": {"type": "object", "additionalProperties": {"type": "array", "items": {"$ref": "#/$defs/ToolRunSettingsPointer"}}},
    },
}
defs["ToolRunDefinition"]["properties"]["settings"] = {"$ref": "#/$defs/ToolRunSettingsReads"}
law = defs["LifecycleLawFixture"]
law["required"] = [key for key in law["required"] if key != "settingsPointers"] + ["settingsPointers"]
law["properties"]["settingsPointers"] = {
    "description": "Settings pointer resolution (RFC 6901) over one document: every row names the value the pointer resolves to, or null when it names nothing; malformed pointers are refused by `ToolRunDefinition.validate`.",
    "type": "object",
    "additionalProperties": False,
    "required": ["document", "rows", "malformed"],
    "properties": {
        "document": {"type": "object"},
        "rows": {
            "type": "array",
            "minItems": 1,
            "items": {
                "type": "object",
                "additionalProperties": False,
                "required": ["pointer", "resolves"],
                "properties": {"pointer": {"$ref": "#/$defs/ToolRunSettingsPointer"}, "resolves": {}},
            },
        },
        "malformed": {"type": "array", "minItems": 1, "items": {"type": "string"}},
    },
}
SCHEMA.write_text(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")

fixture = json.loads(FIXTURE.read_text())
fixture["definition"]["settings"] = {"config": ["/fillCount", "/weights/crate"], "windowConfig": {"main": ["/grid/spacing"]}}
document = {
    "fillCount": 12,
    "weights": {"crate": 0.5, "a/b": 2, "m~n": 3},
    "camera": {"position": [1.5, 2.5, 3.5]},
    "": "empty key",
    "flags": [True, False],
}
rows = [
    ("", document),
    ("/fillCount", 12),
    ("/weights/crate", 0.5),
    ("/weights/a~1b", 2),
    ("/weights/m~0n", 3),
    ("/camera/position/1", 2.5),
    ("/camera/position/01", None),
    ("/camera/position/3", None),
    ("/camera/position/-", None),
    ("/", "empty key"),
    ("/missing", None),
    ("/fillCount/deeper", None),
    ("/flags/0", True),
]
fixture["settingsPointers"] = {"document": document, "rows": [{"pointer": pointer, "resolves": value} for pointer, value in rows], "malformed": ["fillCount", "/weights/~2", "/a~"]}
FIXTURE.write_text(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n")
print("[w0i] schema and fixture updated")
