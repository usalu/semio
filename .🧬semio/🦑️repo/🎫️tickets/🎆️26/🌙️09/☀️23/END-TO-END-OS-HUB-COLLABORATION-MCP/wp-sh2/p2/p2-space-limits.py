#!/usr/bin/env python3
"""🧵️ SH2 P2: re-states the Space index retained-command catalog in its language-neutral fixture and the subset schema's
`SpaceIndexRetainedCommandLimits` narrowing: `touchArtifact` retired (the directory's checkpoint events move the rows), the two
host feeds publish on the Transient lane. Reads the tree, writes the P2 stage; the schema narrowing is derived from the
fixture so the two never drift."""
import json, os

REPO = "/Users/ueli/Documents/semio"
STAGE = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-p2-stage")
SUBSET = "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any"
FIXTURE = f"{SUBSET}/✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json"
SCHEMA = f"{SUBSET}/🧬️schema/🔣️.json"
FEATURES = {
    "foldDirectoryEvents": "fold one bounded directory event batch into the transient projection through the installed transient preparation factory",
    "presenceHeartbeat": "replace one artifact's presence row through the installed transient preparation factory",
}


def indent_of(text):
    return 2 if text.startswith("{\n  \"") else None


fixture_text = open(os.path.join(REPO, FIXTURE), encoding="utf-8").read()
fixture = json.loads(fixture_text)
fixture["publicationContracts"] = [
    {**row, "lanes": ["Transient"]} if row["toolId"] in FEATURES else row for row in fixture["publicationContracts"] if row["toolId"] != "touchArtifact"
]
fixture["routes"] = [{**row, "feature": FEATURES[row["id"]]} if row["id"] in FEATURES else row for row in fixture["routes"] if row["id"] != "touchArtifact"]
count = len(fixture["routes"])
assert count == len(fixture["publicationContracts"]) == 13
for key in ("routes", "bounded", "migrated"):
    fixture["oracle"]["expected"][key] = count
os.makedirs(os.path.dirname(os.path.join(STAGE, FIXTURE)), exist_ok=True)
open(os.path.join(STAGE, FIXTURE), "w", encoding="utf-8").write(json.dumps(fixture, ensure_ascii=False, indent=indent_of(fixture_text)) + "\n")

schema_text = open(os.path.join(REPO, SCHEMA), encoding="utf-8").read()
schema = json.loads(schema_text)
narrowing = schema["$defs"]["SpaceIndexRetainedCommandLimits"]["allOf"][1]["properties"]
narrowing["publicationContracts"].update({
    "items": [{"type": "object", "required": ["toolId", "lanes"], "properties": {"toolId": {"const": row["toolId"]}, "lanes": {"const": row["lanes"]}}, "additionalProperties": False} for row in fixture["publicationContracts"]],
    "minItems": count,
    "maxItems": count,
})
routes = narrowing["routes"]["allOf"][1]
routes["minItems"] = routes["maxItems"] = count


def restate(node):
    if isinstance(node, dict):
        for key, value in node.items():
            if key in ("routes", "bounded", "migrated") and isinstance(value, dict) and value.get("const") == 14:
                value["const"] = count
            restate(value)
    elif isinstance(node, list):
        for value in node:
            restate(value)


restate(narrowing["oracle"])
text = json.dumps(schema, ensure_ascii=False, indent=indent_of(schema_text)) + "\n"
assert "touchArtifact" not in json.dumps(narrowing)
os.makedirs(os.path.dirname(os.path.join(STAGE, SCHEMA)), exist_ok=True)
open(os.path.join(STAGE, SCHEMA), "w", encoding="utf-8").write(text)
print(f"SpaceIndexRetainedCommandLimits: {count} contracts; oracle expected {fixture['oracle']['expected']}")
