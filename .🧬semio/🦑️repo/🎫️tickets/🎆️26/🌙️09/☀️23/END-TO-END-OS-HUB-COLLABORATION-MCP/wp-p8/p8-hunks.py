#!/usr/bin/env python3
"""🧩️ Turns edits validated in P8's scratch clone into an anchored patch set for `patches/p8_patch.py`: every changed
region becomes one `replace()` hunk whose old text is widened with context until it is unique in the tree file.
Usage: p8-hunks.py <clone-root> <patch-name> <docstring-file> <repo-relative-path | @list-file>..."""
import difflib
import sys
from pathlib import Path

TREE = Path("/Users/ueli/Documents/semio")
clone, name, doc_file = Path(sys.argv[1]), sys.argv[2], Path(sys.argv[3])
files = [line for arg in sys.argv[4:] for line in (Path(arg[1:]).read_text().splitlines() if arg.startswith("@") else [arg]) if line]
out = [f'#!/usr/bin/env python3\n"""{doc_file.read_text().rstrip()}\nUsage: {name}.py --dry-run | --write"""\nfrom p8_patch import ROOT, create, finish, replace\n']
total = 0
for rel in files:
    tree_path, clone_path = TREE / rel, clone / rel
    after = clone_path.read_text(encoding="utf-8")
    if not tree_path.exists():
        out.append(f"create({name!r}, ROOT / {rel!r}, {after!r})\n")
        total += 1
        continue
    before = tree_path.read_text(encoding="utf-8")
    if before == after:
        continue
    a, b = before.splitlines(keepends=True), after.splitlines(keepends=True)
    regions = []
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(a=a, b=b, autojunk=False).get_opcodes():
        if tag == "equal":
            continue
        if regions and i1 - regions[-1][1] <= 6:
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
        new = "".join(a[lo:i1] + b[j1:j2] + a[i2:hi])
        hunks.append((lo, hi, old, new))
    merged = []
    for lo, hi, old, new in hunks:
        if merged and lo < merged[-1][1]:
            raise SystemExit(f"{rel}: overlapping hunks at lines {lo}-{hi}; widen the merge distance")
        merged.append((lo, hi, old, new))
    for lo, hi, old, new in merged:
        out.append(f"replace({name!r}, ROOT / {rel!r}, {old!r}, {new!r})\n")
        total += 1
out.append("\nfinish(__doc__)\n")
target = TREE / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/END-TO-END-OS-HUB-COLLABORATION-MCP/wp-p8/patches" / f"{name}.py"
target.write_text("".join(out), encoding="utf-8")
print(f"{target.name}: {total} hunk(s) over {len(files)} file(s)")
