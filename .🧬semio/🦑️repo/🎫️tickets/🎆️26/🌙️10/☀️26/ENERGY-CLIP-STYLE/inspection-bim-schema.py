"""📜 Independently validate current BIM story assets with the existing test-only JSON Schema library."""
import copy
import json
from pathlib import Path
import sys

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(TICKET / "🗑️generated/iv/deps"))
from jsonschema import Draft202012Validator, ValidationError

STORY = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/🎞️presentation"
CASES = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🎭️presentation/🔣️.cases.json"


def main():
    """🧪 Validate declared bilingual stories and reject portable malformed-asset controls."""
    document = json.loads((STORY / "🔣️.json").read_text(encoding="utf-8"))
    schema = json.loads((STORY / "🔣️.schema.json").read_text(encoding="utf-8"))
    cases = json.loads(CASES.read_text(encoding="utf-8"))
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema)
    validator.validate(document)
    expected = {case["topic"]: case["keys"] for case in cases["chapters"] if case["topic"] in cases["story_topics"]}
    assert set(document["topics"]) == set(expected)
    for topic, keys in expected.items():
        assert [state["key"] for state in document["topics"][topic]] == keys
    for case in cases["invalid_story"]:
        modified = copy.deepcopy(document)
        node = modified
        for key in case["path"][:-1]:
            node = node[key]
        if case.get("remove"):
            del node[case["path"][-1]]
        else:
            node[case["path"][-1]] = case["value"]
        try:
            validator.validate(modified)
        except ValidationError:
            pass
        else:
            raise AssertionError("Schema accepted " + case["id"])
    (TICKET / "inspection-bim-schema.md").write_text(
        "# Inspection BIM Story Schema\n\nPassed independent Draft202012 JSON Schema validation of the current story document and schema. "
        + str(len(expected)) + " used topics contain " + str(sum(map(len, expected.values())))
        + " clauses in the exact authored narration order. Both languages are required; portable missing-German, invalid-diagram and invalid-wall-dimension controls are rejected. "
        + "The validator uses the already provisioned disposable test-only jsonschema library under generated/iv/deps; no runtime dependency or new installation is introduced.\n",
        encoding="utf-8")
    print("[DEBUG] BIM schema current topics/clauses", len(expected), sum(map(len, expected.values())), flush=True)


if __name__ == "__main__":
    main()
