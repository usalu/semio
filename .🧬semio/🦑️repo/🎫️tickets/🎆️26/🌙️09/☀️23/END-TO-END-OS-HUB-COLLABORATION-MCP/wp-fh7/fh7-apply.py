"""✍️ FH7 applies reviewed classes (`fh7-decisions.tsv`: owner, code, class, reason) to the CEFG work list, rewriting ONLY the
`class` value of the matching line (byte-stable otherwise) and printing per-owner / per-class change counts.
Usage: python3 fh7-apply.py [--dry]"""
import collections, json, sys
from pathlib import Path

WORK = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-sets/class/family-CEFG.json")
DECISIONS = Path(__file__).with_name("fh7-decisions.tsv")
CLASSES = {"input-invalid", "precondition-failed", "conflict", "permission-denied", "unavailable", "cancelled", "internal"}
decided = {}
for row in DECISIONS.read_text().splitlines():
    if not row.strip() or row.startswith("#"):
        continue
    owner, code, cls, *_ = row.split("\t")
    assert cls in CLASSES, row
    assert (owner, code) not in decided, row
    decided[(owner, code)] = cls
text = WORK.read_text()
out, seen, per_owner, per_class = [], set(), collections.Counter(), collections.Counter()
for line in text.split("\n"):
    if line.startswith("{"):
        entry = json.loads(line.rstrip(","))
        key = (entry["owner"], entry["code"])
        if key in decided:
            seen.add(key)
            new = decided[key]
            old_fragment = f'"class": "{entry["class"]}"'
            assert line.count(old_fragment) == 1, line
            if new != entry["class"]:
                line = line.replace(old_fragment, f'"class": "{new}"')
                per_owner[entry["owner"]] += 1
                per_class[f'{entry["class"]} -> {new}'] += 1
    out.append(line)
missing = set(decided) - seen
assert not missing, missing
result = "\n".join(out)
json.loads(result)
if "--dry" not in sys.argv:
    WORK.write_text(result)
print("changed", sum(per_owner.values()), dict(per_owner))
for k, v in sorted(per_class.items()):
    print(f"  {k}: {v}")
