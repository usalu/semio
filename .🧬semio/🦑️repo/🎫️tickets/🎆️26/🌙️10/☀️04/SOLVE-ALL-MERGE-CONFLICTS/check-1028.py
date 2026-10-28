from pathlib import Path
import json
import re

root = Path(__file__).resolve().parents[7]
contract = json.loads(Path(__file__).with_name("merge-1028.json").read_text(encoding="utf-8"))
for name in contract["files"]:
    text = (root / name).read_text(encoding="utf-8")
    assert not re.search(r"^(?:<{7}|={7}|>{7}|\|{7})(?: |$)", text, re.M), name
project = json.loads((root / contract["files"][1]).read_text(encoding="utf-8"))
assert project["targets"]["build"]["metadata"]["semio"]["closure"]["modules"] == contract["buildModules"]
assert "tailwindScan" in project["targets"]["build"]["inputs"]
assert project["namedInputs"]["tailwindScan"]
workers = next(row for row in project["targets"]["test-e2e"]["metadata"]["semio"]["dashboard"]["parameters"] if row["id"] == "workers")
assert "admitted memory" in workers["description"]["en"]
assert set(workers["description"]) == {"en", "de"}
config = (root / contract["files"][2]).read_text(encoding="utf-8")
declared = re.findall(r'"(🧪️tests/[^"]+\.ts)"', config)
assert set(declared) == set(contract["testIncludes"])
assert len(declared) == len(set(declared))
for name in declared:
    assert (root / "🎓️teaching/🏛️architecture/📚️course" / name).is_file(), name
report = (root / contract["files"][0]).read_text(encoding="utf-8")
assert contract["routingImport"] in report
assert "basename" not in report
assert "reportCommandSourceIdentity" in report
print("PASS: conflict markers, merged build closure, scan inputs, worker descriptions, test union, and routing imports")
