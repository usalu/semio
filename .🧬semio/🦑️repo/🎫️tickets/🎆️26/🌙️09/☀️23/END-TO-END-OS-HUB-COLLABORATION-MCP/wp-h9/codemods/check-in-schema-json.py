"""📌️ One-off: replace the CheckpointPublicationCommand* $defs of the directory schema with the Check In defs."""
import json, collections
p = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"
doc = json.load(open(p, encoding="utf-8"), object_pairs_hook=collections.OrderedDict)
defs = doc["$defs"]
removed = [k for k in list(defs) if k.startswith("CheckpointPublicationCommand")]
assert len(removed) == 10, removed
MAX = 9007199254740991
new = collections.OrderedDict()
new["EditedArtifactFrontierPositiveSafe"] = {"type": "integer", "minimum": 1, "maximum": MAX}
new["EditedArtifactFrontierHash"] = {"type": "string", "pattern": "^(?!0{64}$)[0-9a-f]{64}$"}
new["EditedArtifactFrontierText"] = {"type": "string", "minLength": 1, "maxLength": 256, "pattern": "^[^\\u0000-\\u001f\\u007f-\\u009f]+$"}
new["EditedArtifactFrontierV1"] = {
    "description": "One edited, committed point of a document's ledger: the counters and content chain the hub's document authority published for it, and the edit id at its tip.",
    "type": "object", "additionalProperties": False,
    "required": ["documentId", "headEditOrdinal", "headEditId", "lastCommitSeq", "chainSha256"],
    "properties": {
        "documentId": {"$ref": "#/$defs/EditedArtifactFrontierText"},
        "headEditOrdinal": {"$ref": "#/$defs/EditedArtifactFrontierPositiveSafe"},
        "headEditId": {"$ref": "#/$defs/EditedArtifactFrontierText"},
        "lastCommitSeq": {"$ref": "#/$defs/EditedArtifactFrontierPositiveSafe"},
        "chainSha256": {"$ref": "#/$defs/EditedArtifactFrontierHash"},
    },
}
new["DocumentCheckInRequestId"] = {"type": "string", "pattern": "^(?!0{32}$)[0-9a-f]{32}$"}
new["DocumentCheckInV1"] = {
    "description": "Check In: a document author names a committed ledger head; the hub materializes the checkpoint from its own document log and makes it the active checkpoint. No pair, pair hash, parent or scope is accepted from a client: scope comes from the route and author from the session.",
    "type": "object", "additionalProperties": False,
    "required": ["schema", "requestId", "head"],
    "properties": {
        "schema": {"const": "semio.hub.document-check-in/v1"},
        "requestId": {"$ref": "#/$defs/DocumentCheckInRequestId"},
        "head": {"$ref": "#/$defs/EditedArtifactFrontierV1"},
    },
}
new["DocumentCheckInPhaseV1"] = {"enum": ["accepted", "materializing", "publishing", "ready", "failed", "cancelled"]}
new["DocumentCheckInRefusalV1"] = {
    "description": "unknown-head: not a committed point of this document's ledger; stale-head: precedes the active checkpoint's baseline; active-checkpoint-changed: another check-in or approval advanced the active checkpoint meanwhile; ledger-not-replayable: the ledger range holds an approval decision; codec-refused: the trusted catalog's codec refused the ledger or the replayed pair; authority-changed: the author lost write access or the document its descriptor; unavailable: storage or the trusted catalog was unavailable.",
    "enum": ["unknown-head", "stale-head", "active-checkpoint-changed", "ledger-not-replayable", "codec-refused", "authority-changed", "unavailable"],
}
new["DocumentCheckInProgressV1"] = {
    "type": "object", "additionalProperties": False, "required": ["completedUnits", "totalUnits"],
    "properties": {"completedUnits": {"type": "integer", "minimum": 0, "maximum": MAX}, "totalUnits": {"type": "integer", "minimum": 0, "maximum": MAX}},
}
new["DocumentCheckInReadyV1"] = {
    "type": "object", "additionalProperties": False, "required": ["checkpointId", "parentCheckpointId", "baseline"],
    "properties": {"checkpointId": {"$ref": "#/$defs/EditedArtifactFrontierHash"}, "parentCheckpointId": {"$ref": "#/$defs/EditedArtifactFrontierHash"}, "baseline": {"$ref": "#/$defs/EditedArtifactFrontierV1"}},
}
new["DocumentCheckInStatusV1"] = {
    "description": "One Check In's observable state. Only ready names the checkpoint it made active and only failed names a refusal.",
    "type": "object", "additionalProperties": False,
    "required": ["schema", "requestId", "phase", "progress"],
    "properties": {
        "schema": {"const": "semio.hub.document-check-in-status/v1"},
        "requestId": {"$ref": "#/$defs/DocumentCheckInRequestId"},
        "phase": {"$ref": "#/$defs/DocumentCheckInPhaseV1"},
        "progress": {"$ref": "#/$defs/DocumentCheckInProgressV1"},
        "ready": {"$ref": "#/$defs/DocumentCheckInReadyV1"},
        "refusal": {"$ref": "#/$defs/DocumentCheckInRefusalV1"},
    },
    "allOf": [
        {"if": {"properties": {"phase": {"const": "ready"}}, "required": ["phase"]}, "then": {"required": ["ready"]}, "else": {"not": {"required": ["ready"]}}},
        {"if": {"properties": {"phase": {"const": "failed"}}, "required": ["phase"]}, "then": {"required": ["refusal"]}, "else": {"not": {"required": ["refusal"]}}},
    ],
}
out = collections.OrderedDict()
inserted = False
for key, value in defs.items():
    if key in removed:
        if not inserted:
            out.update(new)
            inserted = True
        continue
    out[key] = value
doc["$defs"] = out
text = json.dumps(doc, ensure_ascii=False, indent=2) + "\n"
raw = open(p, encoding="utf-8").read()
assert "CheckpointPublicationCommand" not in text
open(p, "w", encoding="utf-8").write(text)
print("removed", len(removed), "added", len(new), "trailing-newline-orig", raw.endswith("\n"))
