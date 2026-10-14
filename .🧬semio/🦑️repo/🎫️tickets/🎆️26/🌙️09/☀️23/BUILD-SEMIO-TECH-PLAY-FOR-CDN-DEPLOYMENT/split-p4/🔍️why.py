#!/usr/bin/env python3
"""🔍️ Prints the sample references behind crate edges from an `✂️owner-cut.py --dump` file.

usage: 🔍️why.py <dump.json> <from>[>to] [--tests]
"""
import collections, json, sys
dump = json.load(open(sys.argv[1]))
want = sys.argv[2]
tests = "--tests" in sys.argv
for key, rows in sorted(dump.items()):
    a, b = ("<consumer>", key[len("<consumer>>"):]) if key.startswith("<consumer>") else key.split(">", 1)
    if ">" in want:
        if key != want:
            continue
    elif a != want:
        continue
    counts = collections.Counter((r[3], r[0].split("✳️any/")[-1][-70:], r[2]) for r in rows if tests or r[3] == "-")
    if not counts:
        continue
    print(f"== {key}")
    for (t, f, item), n in counts.most_common(12):
        print(f"   {t} {n}x {f} :: {item}")
