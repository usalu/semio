"""✍️ FH5 (session 15): writes the reviewed fault classes of `fh5-decisions.tsv` (owner, code, from, to, reason) into the
class work lists family-A/H — only the `class` value of the matching line changes, every other byte stays; keeps a
pre-image in 🗑️generated and prints per-class before/after counts and the transition census.
Usage: python3 fh5-apply.py [--dry]"""
from __future__ import annotations

import collections
import json
import shutil
import sys
from pathlib import Path

HERE = Path(__file__).parent
SETS = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-sets/class")
CLASSES = ("input-invalid", "precondition-failed", "conflict", "permission-denied", "unavailable", "cancelled", "internal")


def decisions() -> dict[tuple[str, str], str]:
    rows = [line.split("\t") for line in (HERE / "fh5-decisions.tsv").read_text().splitlines() if line and not line.startswith("#")]
    return {(owner, code): to for owner, code, _, to, _ in rows}


def main() -> None:
    dry = "--dry" in sys.argv
    wanted = decisions()
    applied: set[tuple[str, str]] = set()
    for family in ("A", "H"):
        path = SETS / f"family-{family}.json"
        lines = path.read_text().split("\n")
        before, after, moves = collections.Counter(), collections.Counter(), collections.Counter()
        for index, line in enumerate(lines):
            body = line.rstrip(",")
            if not body.startswith("{"):
                continue
            entry = json.loads(body)
            key = (entry["owner"], entry["code"])
            before[entry["class"]] += 1
            new = wanted.get(key, entry["class"])
            if new != entry["class"]:
                old_field, new_field = f'"class": "{entry["class"]}"', f'"class": "{new}"'
                assert line.count(old_field) == 1, key
                lines[index] = line.replace(old_field, new_field)
                moves[f'{entry["class"]}→{new}'] += 1
                applied.add(key)
            elif key in wanted:
                applied.add(key)
            after[new] += 1
        print(f"family {family}: changed {sum(moves.values())}")
        print("  " + " | ".join(f"{cls} {before[cls]}→{after[cls]}" for cls in CLASSES))
        print("  " + ", ".join(f"{move} {count}" for move, count in moves.most_common()))
        if not dry:
            shutil.copy(path, HERE / "🗑️generated" / f"family-{family}.pre.json")
            path.write_text("\n".join(lines))
    missing = set(wanted) - applied
    assert not missing, missing


if __name__ == "__main__":
    main()
