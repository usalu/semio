from pathlib import Path
import json
import jsonschema

root = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
fixture_root = root / "🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/🧪️tests/🧫️fixtures"
fixture = json.loads((fixture_root / "🧭️source-drain.json").read_text())
schema = json.loads((fixture_root / "🧭️source-drain.schema.json").read_text())
jsonschema.Draft202012Validator.check_schema(schema)
jsonschema.Draft202012Validator(schema).validate(fixture)
assert [phase["name"] for phase in fixture["phases"]] == fixture["expectedPhases"]
assert all(policy <= total for policy, total in zip(fixture["callPolicy"], fixture["totalGrant"]))
assert fixture["phases"][-1]["cancelled"]
print("original source drain schema and independent phase oracle passed")

expiry = json.loads((fixture_root / "⌛️source-expiry.json").read_text())
expiry_schema = json.loads((fixture_root / "⌛️source-expiry.schema.json").read_text())
jsonschema.Draft202012Validator.check_schema(expiry_schema)
jsonschema.Draft202012Validator(expiry_schema).validate(expiry)
assert expiry["initialClock"] < expiry["work"]["absoluteDeadlineUs"] < expiry["expiredClock"]
assert expiry["zeroFuelBlocked"]
print("original source expiry schema passed")
