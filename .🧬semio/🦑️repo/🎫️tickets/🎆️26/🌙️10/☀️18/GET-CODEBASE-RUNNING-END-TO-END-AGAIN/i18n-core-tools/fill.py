#!/usr/bin/env python3
"""usage: fill.py SPEC DE   fills the `=> ???` lines of a scaffold with the German texts of DE: blocks `== label` in the order of the spec's
`== file ¦ scope` blocks, one line per entry, optional ` ¦ names` overrides the placeholder names of the entry. Refuses on any count mismatch."""
import sys

spec_path, de_path = sys.argv[1], sys.argv[2]
spec = open(spec_path, encoding="utf8").read().split("\n")
blocks = []
for line in open(de_path, encoding="utf8").read().split("\n"):
    if not line.strip() or line.startswith("#"):
        continue
    if line.startswith("== "):
        blocks.append([])
        continue
    blocks[-1].append(line)
starts = [i for i, line in enumerate(spec) if line.startswith("== ")]
if len(starts) != len(blocks):
    raise SystemExit("block count mismatch: spec %d, german %d" % (len(starts), len(blocks)))
out, block, index, total = [], -1, 0, 0
for i, line in enumerate(spec):
    if line.startswith("== "):
        block += 1
        index = 0
        end = starts[block + 1] if block + 1 < len(starts) else len(spec)
        expected = sum(1 for k in range(i + 1, end) if spec[k] == "=> ???")
        if expected != len(blocks[block]):
            raise SystemExit("count mismatch in block %d (%s): spec %d, german %d" % (block + 1, line[-50:], expected, len(blocks[block])))
    if line == "=> ???":
        de = blocks[block][index]
        index += 1
        total += 1
        if " ¦ " in de:
            de, names = de.rsplit(" ¦ ", 1)
            head = out[-1].split(" ¦ ")
            head[1] = names.strip()
            out[-1] = " ¦ ".join(head)
        out.append("=> " + de)
    else:
        out.append(line)
open(spec_path, "w", encoding="utf8").write("\n".join(out))
print("filled", total)
