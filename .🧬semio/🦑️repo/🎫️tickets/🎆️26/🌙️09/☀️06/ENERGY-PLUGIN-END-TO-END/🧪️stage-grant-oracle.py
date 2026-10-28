"""🎟️ Independent JSON Schema eligibility oracle for original retirement birth denial."""
import json
from pathlib import Path
from jsonschema import Draft202012Validator
fixture=json.loads(Path("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🗄️stage/📥️lifted/🗄️durable-group/🧵️binding/🧪️tests/🔣️.json").read_text())
axes=fixture["deniedBirthAxes"]
validator=Draft202012Validator({"type":"object","required":axes,"additionalProperties":False,"properties":{axis:{"type":"integer","minimum":1} for axis in axes}})
positive={axis:1 for axis in axes}
assert validator.is_valid(positive)
for axis in axes:
    assert not validator.is_valid({**positive,axis:0})
print("[DEBUG] JSON Schema original birth eligibility: one positive and four denied axes PASS")
