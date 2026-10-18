"""🧽 OHH sandbox macro blanker: removes one top-level `name! { ... }` invocation (found by its start line) from a SANDBOX file so that a failing macro_rules arm no longer stops the oracle. Refuses paths outside ~/.cache/ohh.
usage: blank_macro.py <sandbox-file> <line-inside-or-start>"""
import os, sys
p = os.path.realpath(sys.argv[1])
assert p.startswith("/Users/ueli/.cache/ohh/"), p
ln = int(sys.argv[2])
lines = open(p, encoding="utf-8").read().split("\n")
i = ln - 1
import re
while i > 0 and not re.match(r"^[A-Za-z_][A-Za-z0-9_]*!\s*[\{\(\[]", lines[i]):
    i -= 1
start = i
depth, j = 0, i
seen = False
while j < len(lines):
    for ch in lines[j]:
        if ch in "{([":
            depth += 1
            seen = True
        elif ch in "})]":
            depth -= 1
    if seen and depth <= 0:
        break
    j += 1
assert seen
lines[start:j + 1] = [""] * (j + 1 - start)
open(p, "w", encoding="utf-8").write("\n".join(lines))
print("blanked", start + 1, "-", j + 1)
