"""🏷️ Retypes `@mode-property` scenario blocks whose ONLY assertion is a cross-producer agreement.

A block asserting nothing but "the oracle and the subject agree …" is a DIFFERENTIAL scenario; the
`@mode-property` tag on it names a law it does not state. Blocks that also assert an inverse or a
round-trip law keep their tag — the differential is measured for them regardless, because
`evaluateParity` compares every scenario both roles produced, not only the `@mode-differential` ones.
"""
import re, sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
CROSS = re.compile(r"(the oracle and the subject agree|the reference implementation and this repository agree)", re.I)
IN_ROLE = re.compile(r"(asserted in role|independently read)", re.I)
APPLY = "--apply" in sys.argv

paths = sorted(set(sys.argv[1:]) - {"--apply"})
changed = []
for rel in paths:
    p = ROOT / rel
    lines = p.read_text(encoding="utf-8").split("\n")
    # block starts at the first tag line of a contiguous tag run followed by Scenario
    edits = []
    i = 0
    while i < len(lines):
        if not lines[i].strip().startswith("@"):
            i += 1
            continue
        start = i
        while i < len(lines) and lines[i].strip().startswith("@"):
            i += 1
        if i >= len(lines) or not re.match(r"\s*Scenario", lines[i]):
            continue
        tags = lines[start:i]
        mode_idx = next((k for k in range(start, i) if lines[k].strip() == "@mode-property"), None)
        # body of the block: until the next tag run or EOF
        j = i
        while j < len(lines) and not (lines[j].strip().startswith("@") and j + 1 < len(lines)):
            j += 1
        body = lines[i:j]
        if mode_idx is None:
            continue
        asserts = [l.strip() for l in body if re.match(r"\s*(Then|And)\s", l)]
        # only lines after the first Then count as assertions
        first_then = next((k for k, l in enumerate(body) if re.match(r"\s*Then\s", l)), None)
        if first_then is None:
            continue
        asserts = [l.strip() for l in body[first_then:] if re.match(r"\s*(Then|And)\s", l)]
        if not asserts:
            continue
        if all(CROSS.search(a) and not IN_ROLE.search(a) for a in asserts):
            edits.append((mode_idx, len(re.findall(r"^\s*\|", "\n".join(body), re.M))))
    if not edits:
        continue
    for idx, _ in edits:
        lines[idx] = lines[idx].replace("@mode-property", "@mode-differential")
    changed.append((rel, len(edits)))
    if APPLY:
        p.write_text("\n".join(lines), encoding="utf-8")

for rel, n in changed:
    print("%2d block(s)  %s" % (n, rel))
print("files:", len(changed), "blocks:", sum(n for _, n in changed), "applied:", APPLY)
