"""🧭️ One-off SH2 codemod: re-states `SpacePlayRetainedCommandLimits` (space plugin `🧬️schema/🔣️.json`) from the studio's
language-neutral retained-command fixture, which the Rust law already pins to the code (40 publication contracts, every route
bounded + migrated). The schema narrowing still stated the 09-24 catalog (16 contracts, 15 bounded / 25 batch). `--root` picks
the tree to rewrite (stage or overlay)."""
import argparse, json, os

parser = argparse.ArgumentParser()
parser.add_argument("--root", required=True)
root = parser.parse_args().root
schema_path = os.path.join(root, "✏️s/🔌️plugins/🪐️space/🧬️schema/🔣️.json")
fixture_path = os.path.join(root, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🧫️fixtures/🧫️retained-command-limits/🔣️.json")
schema = json.load(open(schema_path, encoding="utf-8"))
fixture = json.load(open(fixture_path, encoding="utf-8"))
owner = schema["$defs"]["SpacePlayRetainedCommandLimits"]["allOf"][1]["properties"]
contracts = fixture["publicationContracts"]
owner["publicationContracts"] = {
    "type": "array",
    "additionalItems": False,
    "minItems": len(contracts),
    "maxItems": len(contracts),
    "items": [{"type": "object", "required": ["toolId", "lanes"], "properties": {"toolId": {"const": row["toolId"]}, "lanes": {"const": row["lanes"]}}, "additionalProperties": False} for row in contracts],
}
expected = owner["oracle"]["properties"]["expected"]["properties"]
for key in ("routes", "bounded", "batch", "migrated", "unique"):
    expected[key] = {"const": fixture["oracle"]["expected"][key]}
with open(schema_path, "w", encoding="utf-8") as handle:
    handle.write(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
print(f"SpacePlayRetainedCommandLimits: {len(contracts)} contracts, expected {fixture['oracle']['expected']}")
