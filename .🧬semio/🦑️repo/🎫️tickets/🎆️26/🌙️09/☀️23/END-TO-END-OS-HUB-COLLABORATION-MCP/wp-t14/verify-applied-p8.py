#!/usr/bin/env python3
"""🔎️ T14: applied-state check of a P8 patch set (`wp-p8/patches/*.py`) on a tree — every `replace` hunk counts as applied when
its new text is present (and its old text is gone unless contained in the new one), every `create` when the file already
equals its content; prints `nothing to do (applied)` or the hunks still pending. usage: verify-applied-p8.py <set.py> <root>"""
import os
import runpy
import sys
from pathlib import Path

script, root = sys.argv[1], sys.argv[2]
os.environ["P8_ROOT"] = root
sys.path.insert(0, str(Path(script).parent))
import p8_patch

pending, applied = [], 0


def replace(part, path, old, new, count=1):
    global applied
    text = path.read_text(encoding="utf-8")
    if new in text and (old in new or old not in text):
        applied += 1
    else:
        pending.append(f"{part}: {path.relative_to(p8_patch.ROOT)}: old {text.count(old)}x, new {text.count(new)}x: {old.strip()[:70]!r}")


def create(part, path, content):
    global applied
    if path.exists() and path.read_text(encoding="utf-8") == content:
        applied += 1
    else:
        pending.append(f"{part}: create {path.relative_to(p8_patch.ROOT)}")


def finish(doc):
    print(f"{applied} applied, {len(pending)} pending")
    for row in pending:
        print("  pending", row)
    print("nothing to do (applied)" if not pending else "NOT fully applied")
    sys.exit(0 if not pending else 1)


p8_patch.replace, p8_patch.create, p8_patch.finish = replace, create, finish
sys.argv = [script, "--dry-run"]
runpy.run_path(script, run_name="__main__")
