#!/usr/bin/env python3
"""🎭️ P9 stage: copies each repo file into the gitignored stage twice — `base/` (the tree as it was when staged, the
anchor source) and `stage/` (edited by P9) — unless it is staged already. `p9-hunks.py` diffs base → stage, so a peer's
later tree edit never turns into a hunk that reverts it; the dry run on the live tree proves every anchor still holds.
Usage: p9-stage.py <repo-relative path>... | --status"""
import shutil
import os
import sys
from pathlib import Path

TREE = Path("/Users/ueli/Documents/semio")
STAGE = TREE / os.environ.get("P9_STAGE", ".🧬semio/🌐hub/s14-p9-stage")

if sys.argv[1:] == ["--status"]:
    for base in sorted((STAGE / "base").rglob("*")):
        if base.is_file():
            rel = base.relative_to(STAGE / "base")
            tree, stage = TREE / rel, STAGE / "stage" / rel
            drift = "tree-drift" if not tree.exists() or tree.read_bytes() != base.read_bytes() else "tree=base"
            edit = "edited" if stage.read_bytes() != base.read_bytes() else "unedited"
            print(f"{edit:9} {drift:10} {rel}")
    sys.exit(0)
for rel in sys.argv[1:]:
    source = TREE / rel
    for side in ("base", "stage"):
        target = STAGE / side / rel
        if target.exists():
            print(f"kept   {side}/{rel}")
            continue
        target.parent.mkdir(parents=True, exist_ok=True)
        if source.exists():
            shutil.copy2(source, target)
            print(f"staged {side}/{rel}")
        elif side == "stage":
            target.write_text("", encoding="utf-8")
            print(f"new    {side}/{rel}")
