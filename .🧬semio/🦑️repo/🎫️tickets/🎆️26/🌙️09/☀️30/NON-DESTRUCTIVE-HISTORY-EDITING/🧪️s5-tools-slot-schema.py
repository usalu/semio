"""🧬️ Wave B schema edit (design §22.10): the host-fact table, the framework-owned persisted gesture and the per-window slot
scenarios, added to `$defs` of the tool-machine schema of record. Usage: python3 schema-edit.py <🧬️schema/🔣️.json>"""

import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
schema = json.loads(text)
if json.dumps(schema, indent=2, ensure_ascii=False) + "\n" != text:
    raise SystemExit("the schema file no longer round-trips; edit it by hand")
defs = schema["$defs"]
if "GestureHostEvent" in defs:
    raise SystemExit("already applied")

nullable = lambda ref: {"oneOf": [{"type": "null"}, {"$ref": f"#/$defs/{ref}"}]}
window = {"type": "string", "minLength": 1}

for name in ("GestureDriveGesture", "GestureDriveDispatch"):
    base = defs[name]["properties"]["base"]
    if base != {"type": "string", "minLength": 1}:
        raise SystemExit(f"{name}.base moved")
    defs[name]["properties"]["base"] = {"description": "The document revision; empty = pinned to no revision (a tool whose leaves fold on any base).", "type": "string"}

defs["GestureHostEvent"] = {
    "description": "A host fact that may end a window's open gesture: the window lost focus or its pointer capture, its utility switched, it is closing, a history edit froze the document, or a remote edit moved the base.",
    "enum": ["blur", "captureLost", "utilityChanged", "retiring", "timeTravelFrozen", "baseMoved"],
}
defs["GestureHostAbort"] = {
    "description": "The reason a host fact ends an open gesture with; `null` keeps the gesture. `baseBound` says whether the gesture is pinned to a base revision: only then a moved base ends it.",
    "type": "object",
    "additionalProperties": False,
    "required": ["event", "baseBound", "reason"],
    "properties": {"event": {"$ref": "#/$defs/GestureHostEvent"}, "baseBound": {"type": "boolean"}, "reason": nullable("ToolAbortReason")},
}
defs["GestureState"] = {
    "title": "GestureState",
    "description": "One window's open gesture between dispatches — the framework-owned persisted form of a streamed tool (window slot of the plugin runtime; ephemeral, local-only, never history): the statechart configuration by stable ids, the verb, the admission's authoring seed and the document revision it opened on (empty = pinned to none), the open transaction with its keyed provisional mutations, and the tool context its entries do not already say (`null` = none).",
    "type": "object",
    "additionalProperties": False,
    "required": ["states", "verb", "authoringSeed", "baseRevision", "transaction", "entries", "context"],
    "properties": {
        "states": {"type": "array", "minItems": 1, "items": {"type": "string", "minLength": 1}},
        "verb": {"type": "string", "minLength": 1},
        "authoringSeed": {"type": "string"},
        "baseRevision": {"type": "string"},
        "transaction": {"$ref": "#/$defs/TransactionRef"},
        "entries": {"$ref": "#/$defs/TransactionEntries"},
        "context": {},
    },
}
defs["GestureSlotOutcome"] = {
    "description": "What one ledger step reports: the ticks a dispatch committed as ONE transaction, the refusal that faulted it, the gestures host facts ended (window order) and every window's persisted gesture after the step.",
    "type": "object",
    "additionalProperties": False,
    "required": ["committed", "refused", "aborted", "open"],
    "properties": {
        "committed": nullable("GestureDriveTicks"),
        "refused": nullable("ToolRefusal"),
        "aborted": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["window", "reason"], "properties": {"window": window, "reason": {"$ref": "#/$defs/ToolAbortReason"}}}},
        "open": {"type": "object", "additionalProperties": {"$ref": "#/$defs/GestureDriveGesture"}},
    },
}
step = lambda key, value: {"type": "object", "additionalProperties": False, "required": [key, "expected"], "properties": {key: value, "expected": {"$ref": "#/$defs/GestureSlotOutcome"}}}
drive = json.loads(json.dumps(defs["GestureDriveDispatch"]))
drive["description"] = "One dispatch of a streamed gesture verb in `window`."
drive["required"] = ["window", *drive["required"]]
drive["properties"] = {"window": window, **drive["properties"]}
host_all = {"type": "object", "additionalProperties": False, "required": ["event"], "properties": {"event": {"$ref": "#/$defs/GestureHostEvent"}}}
host_one = {"type": "object", "additionalProperties": False, "required": ["window", "event"], "properties": {"window": window, "event": {"$ref": "#/$defs/GestureHostEvent"}}}
defs["GestureSlotStep"] = {
    "description": "One step over the per-window gesture ledger: a dispatch of a window (`drive`), a host fact of one window (`host`) or of every window (`hostAll`), the window roster that stays (`retain`), or a tampered slot its tool can no longer restore (`corrupt`).",
    "oneOf": [step("drive", drive), step("host", host_one), step("hostAll", host_all), step("retain", {"type": "array", "items": window}), step("corrupt", window)],
}
defs["GestureSlotScenario"] = {
    "type": "object",
    "additionalProperties": False,
    "required": ["name", "steps"],
    "properties": {"name": {"type": "string", "minLength": 1}, "steps": {"type": "array", "minItems": 1, "items": {"$ref": "#/$defs/GestureSlotStep"}}},
}
law = defs.pop("GestureDriveLawFixture")
law["description"] = "Language-agnostic laws of the shared streamed-gesture runner (`drive_gesture` / `driveGesture`, `🧫️fixtures/🧫️gesture-drive-law`) and its per-window slot (`GestureLedger`), authored by an independent model of a counting tool: the phase argument table, one row per dispatch from the window's persisted gesture, the reason each host fact ends an open gesture with, and step sequences over the ledger."
law["required"] = [*law["required"], "hostEvents", "slots"]
law["properties"]["hostEvents"] = {"type": "array", "minItems": 12, "maxItems": 12, "items": {"$ref": "#/$defs/GestureHostAbort"}}
law["properties"]["slots"] = {"type": "array", "minItems": 1, "items": {"$ref": "#/$defs/GestureSlotScenario"}}
defs["GestureDriveLawFixture"] = law
path.write_text(json.dumps(schema, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(f"schema: {len(defs)} $defs")
