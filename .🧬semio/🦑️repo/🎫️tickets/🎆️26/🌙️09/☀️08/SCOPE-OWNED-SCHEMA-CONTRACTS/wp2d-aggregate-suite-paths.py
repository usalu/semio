#!/usr/bin/env python3
"""🛂️ Repoints the aggregate library suite's per-case authority reads at `🛂️schema/` (row 115).

Only the listed lines are touched: every other `🧬️schema/🔣️.json` in the file is either the library's own
schema module or the taxonomy mutation-payload location, both of which keep their name.
"""
import sys

PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts"
CASE_AUTHORITY_LINES = [319, 409, 560, 2850, 2894, 2956, 4343, 4382, 4491, 4502, 4514, 4557, 6989, 7081]


def main():
    lines = open(PATH, encoding="utf-8").read().split("\n")
    changed = 0
    for number in CASE_AUTHORITY_LINES:
        line = lines[number - 1]
        if "🧬️schema/🔣️.json" not in line:
            print(f"skip L{number}: no case authority read", file=sys.stderr)
            continue
        lines[number - 1] = line.replace("🧬️schema/🔣️.json", "🛂️schema/🔣️.json")
        changed += 1
    open(PATH, "w", encoding="utf-8").write("\n".join(lines))
    print(f"# {changed}/{len(CASE_AUTHORITY_LINES)} lines repointed")


main()
