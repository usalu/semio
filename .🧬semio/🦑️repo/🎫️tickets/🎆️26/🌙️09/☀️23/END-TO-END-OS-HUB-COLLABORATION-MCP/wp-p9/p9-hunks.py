#!/usr/bin/env python3
"""🧩️ Turns P9's staged edits into an anchored patch set for `patches/p9_patch.py`: every changed region between the
staged base and the staged edit becomes one `replace()` hunk whose old text is widened with context until it is unique in
the base file (change regions ≤ `P9_MERGE` lines apart, default 3, share one hunk, so a peer's edit between
two of P9's changes keeps both anchors); a file with no base becomes a `create()`. The dry run on the live tree then proves every anchor.
Usage: p9-hunks.py <patch-name> <docstring-file> <repo-relative path>..."""
import difflib
import os
import sys
from pathlib import Path

TREE = Path("/Users/ueli/Documents/semio")
STAGE = TREE / ".🧬semio/🌐hub/s14-p9-stage"
name, doc_file = sys.argv[1], Path(sys.argv[2])
files = sys.argv[3:]
MERGE = int(os.environ.get("P9_MERGE", "3"))
out = [f'#!/usr/bin/env python3\n"""{doc_file.read_text().rstrip()}\nUsage: {name}.py --dry-run | --write"""\nfrom p9_patch import ROOT, create, finish, replace\n']
total = 0
for rel in files:
    base_path, stage_path = STAGE / "base" / rel, STAGE / "stage" / rel
    after = stage_path.read_text(encoding="utf-8")
    if not base_path.exists():
        out.append(f"create({name!r}, ROOT / {rel!r}, {after!r})\n")
        total += 1
        continue
    before = base_path.read_text(encoding="utf-8")
    if before == after:
        continue
    a, b = before.splitlines(keepends=True), after.splitlines(keepends=True)
    regions = []
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(a=a, b=b, autojunk=False).get_opcodes():
        if tag == "equal":
            continue
        if regions and i1 - regions[-1][1] <= MERGE:
            regions[-1] = (regions[-1][0], i2, regions[-1][2], j2)
        else:
            regions.append((i1, i2, j1, j2))
    hunks = []
    for i1, i2, j1, j2 in regions:
        context = 1
        while True:
            lo, hi = max(0, i1 - context), min(len(a), i2 + context)
            old = "".join(a[lo:hi])
            if old and before.count(old) == 1:
                break
            if lo == 0 and hi == len(a):
                break
            context += 1
        hunks.append((lo, hi, old, "".join(a[lo:i1] + b[j1:j2] + a[i2:hi])))
    for index, (lo, hi, old, new) in enumerate(hunks):
        if index and lo < hunks[index - 1][1]:
            raise SystemExit(f"{rel}: overlapping hunks at lines {lo}-{hi}; widen the merge distance")
        out.append(f"replace({name!r}, ROOT / {rel!r}, {old!r}, {new!r})\n")
        total += 1
out.append("\nfinish(__doc__)\n")
target = Path(__file__).parent / "patches" / f"{name}.py"
target.write_text("".join(out), encoding="utf-8")
print(f"{target.name}: {total} hunk(s) over {len(files)} file(s)")
