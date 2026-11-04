import collections
import hashlib
import json
import pathlib
import re
import sys
import toml

ROOT = pathlib.Path(__file__).parent
LOCK = pathlib.Path("🌎️hub/Cargo.lock")

def compact(text):
    matches = list(re.finditer(r"\[\[package\]\][\s\S]*?(?=\[\[package\]\]|\Z)", text))
    kept = {}
    output = text[:matches[0].start()] if matches else text
    removed = 0
    for match in matches:
        block = match.group()
        fields = dict(re.findall(r'^(name|version|source) = "([^"]*)"$', block, re.M))
        identity = (fields["name"], fields["version"], fields.get("source", ""))
        body = block.rstrip()
        if identity in kept:
            if kept[identity] != body:
                raise ValueError("Conflicting package body: " + repr(identity))
            removed += 1
        else:
            kept[identity] = body
            output += block
    return output, removed

def oracle(text):
    data = toml.loads(text)
    groups = collections.defaultdict(list)
    for package in data["package"]:
        groups[(package["name"], package["version"], package.get("source", ""))].append(package)
    assert all(all(row == rows[0] for row in rows) for rows in groups.values())
    return data, {identity: rows[0] for identity, rows in groups.items()}, sum(len(rows)-1 for rows in groups.values())

fixture = json.loads((ROOT / "1103-lock-cases.json").read_text())
for case in fixture:
    try:
        result, removed = compact(case["input"])
        assert not case.get("refuse", False)
        before, expected, duplicates = oracle(case["input"])
        after, actual, remaining = oracle(result)
        assert actual == expected and remaining == 0
        assert removed == duplicates == case["removed"]
    except ValueError:
        assert case.get("refuse", False)

source = LOCK.read_text()
before_hash = hashlib.sha256(source.encode()).hexdigest()
before, expected, duplicates = oracle(source)
result, removed = compact(source)
after, actual, remaining = oracle(result)
assert actual == expected and remaining == 0 and removed == duplicates
assert {k: v for k, v in before.items() if k != "package"} == {k: v for k, v in after.items() if k != "package"}
report = ["# Hub Lock Repair", "", "Third-party Python toml independently validates the raw-block repair against all original package records and references.", "", "- Fixture cases passed: " + str(len(fixture)), "- Before duplicate rows: " + str(duplicates), "- Removed identical package bodies: " + str(removed), "- After duplicate identities: " + str(remaining), "- Original SHA256: " + before_hash, "- Candidate SHA256: " + hashlib.sha256(result.encode()).hexdigest(), "- All retained package bodies, dependency references, and non-package metadata preserved.", "- Final block separator whitespace is excluded from package body equality; no package text is normalized.", ""]
if "--apply" in sys.argv:
    assert LOCK.read_text() == source
    LOCK.write_text(result)
    report.append("Applied only to 🌎️hub/Cargo.lock.")
else:
    report.append("Audit only; no lock writes.")
(ROOT / "1103-hub-lock-repair.md").write_text("\n".join(report) + "\n")
print(json.dumps({"fixtures": len(fixture), "duplicates": duplicates, "removed": removed, "remaining": remaining, "applied": "--apply" in sys.argv}))
