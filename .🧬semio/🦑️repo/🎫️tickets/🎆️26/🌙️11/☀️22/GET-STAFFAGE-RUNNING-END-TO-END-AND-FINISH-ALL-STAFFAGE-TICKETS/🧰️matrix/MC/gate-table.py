"""🧮️ The hard findings of the fixer's gate per asset from a log of `measure.sh` (`<tag>-gate.txt`): `python3 gate-table.py <log> [<id>]` prints `<id> <count> <code×count, …>` per asset, or the count and codes of one asset."""
import re, sys, collections
rows, codes = {}, collections.Counter()
for line in open(sys.argv[1]):
    m = re.match(r"(\S+) 1:\S+ HARD (\S+) ×(\d+)", line)
    if m:
        codes[m.group(2)] += int(m.group(3))
    m = re.match(r"\[DEBUG\] gate (\S+) exit", line)
    if m:
        rows[m.group(1).split("/")[-1].replace(".json", "")] = (sum(codes.values()), ", ".join(f"{c}×{n}" for c, n in sorted(codes.items())))
        codes = collections.Counter()
if len(sys.argv) > 2:
    print("%d: %s" % rows.get(sys.argv[2], (-1, "not measured")))
else:
    for a, (n, c) in rows.items():
        print(f"{a}\t{n}\t{c}")
    print("total", sum(n for n, _ in rows.values()))
