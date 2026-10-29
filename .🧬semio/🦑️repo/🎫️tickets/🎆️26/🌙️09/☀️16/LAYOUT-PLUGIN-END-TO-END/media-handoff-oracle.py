import json
import sys
import jsonschema
import xml.etree.ElementTree as xml
from pathlib import Path

fixture = json.loads(Path("✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎥️media-export-job-and-factories/🧪️fixture.json").read_text())
schema = json.loads(Path("✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎥️media-export-job-and-factories/🧬️vector.schema.json").read_text())
jsonschema.Draft202012Validator.check_schema(schema)
for row in fixture["projectionCases"]:
    actual = json.loads(json.dumps({"schema": fixture["projectionSchema"], "source": {"documentId": row["documentId"], "frontierRevision": row["frontierRevision"]}, "svg": row["svg"]}, ensure_ascii=False))
    jsonschema.validate(actual, schema)
    assert actual == row["expected"]
    print(json.dumps({"name": row["name"], "projection": actual}, ensure_ascii=False, sort_keys=True))
if len(sys.argv) == 1:
    sys.exit(0)
records = [json.loads(line.split("[media-handoff] ", 1)[1]) for line in Path(sys.argv[1]).read_text().splitlines() if "[media-handoff] " in line]
assert len(records) == len(fixture["cases"]), records
for case, actual in zip(fixture["cases"], records):
    assert actual["name"] == case["name"] and actual["cancelled"] == case["cancel"] and actual["unchanged"]
    if not case["cancel"]:
        assert actual["schema"] == fixture["schema"] and actual["mime"] == fixture["mime"]
        projection = json.loads(actual["text"])
        jsonschema.validate(projection, schema)
        assert set(projection) == {"schema", "source", "svg"}
        assert projection["schema"] == fixture["projectionSchema"]
        assert set(projection["source"]) == {"documentId", "frontierRevision"}
        assert projection["source"]["documentId"]
        revision = projection["source"]["frontierRevision"]
        assert len(revision) == 64 and all(byte in "0123456789abcdef" for byte in revision)
        root = xml.fromstring(projection["svg"])
        assert root.tag == "{" + fixture["xmlNamespace"] + "}" + fixture["root"]
    print(json.dumps(actual, ensure_ascii=False, sort_keys=True))

children = [json.loads(line.split("[media-child] ", 1)[1]) for line in Path(sys.argv[1]).read_text().splitlines() if "[media-child] " in line]
assert len(children) == 1, children
for actual in children:
    expected = fixture["mountedChild"]
    assert actual["unchanged"] and actual["childId"] == expected["childId"]
    projection = actual["projection"]
    jsonschema.validate(projection, schema)
    assert projection["source"]["documentId"] == actual["documentId"]
    root = xml.fromstring(projection["svg"])
    assert root.tag == "{" + fixture["xmlNamespace"] + "}" + fixture["root"]
    paths = root.findall(".//{" + fixture["xmlNamespace"] + "}path")
    assert len(paths) >= expected["minimumPaths"]
    assert any(expected["expectedSvgCommand"] in path.get("d", "").split() or expected["expectedSvgCommand"] in path.get("d", "") for path in paths)
    print(json.dumps(actual, ensure_ascii=False, sort_keys=True))

cancellations = [json.loads(line.split("[media-child-cancel] ", 1)[1]) for line in Path(sys.argv[1]).read_text().splitlines() if "[media-child-cancel] " in line]
assert len(cancellations) == 1, cancellations
for actual in cancellations:
    assert actual["childId"] == fixture["mountedChild"]["childId"] and actual["cancelled"] and actual["unchanged"]
    assert actual["creditedBytes"] >= fixture["mountedChild"]["cancelAfterOutputBytes"]
    print(json.dumps(actual, ensure_ascii=False, sort_keys=True))
