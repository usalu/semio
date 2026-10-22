#!/usr/bin/env python3
"""Round-6 assertion: the tree the chain produced contains no `repo repo` (the command-form codemod must never prefix an existing `repo`). Scans the files modified since the marker (R6_MARKER: written when the overlay was cloned / when apply-all started) below the root. usage: assert-no-repo-repo.py <root>"""
import os, re, sys
root = os.path.abspath(sys.argv[1])
marker = os.environ.get("R6_MARKER", "/private/tmp/i6/ov-born")
since = os.stat(marker).st_mtime if os.path.exists(marker) else 0
pattern = re.compile(r"(?<![\w-])repo repo(?![\w-])")
bad = []
skip = {".git", "node_modules", ".cache", ".nx", "target", "\U0001F5D1️generated"}
for base, dirs, files in os.walk(root):
    dirs[:] = [d for d in dirs if d not in skip and not d.startswith("r6-") and d != "round6-tools"]
    rel = os.path.relpath(base, root)
    if "\U0001F3AB️tickets" in rel:
        dirs[:] = []
        continue
    for f in files:
        p = os.path.join(base, f)
        try:
            if os.lstat(p).st_mtime < since or os.path.islink(p) or os.path.getsize(p) > 3_000_000:
                continue
            text = open(p, encoding="utf8").read()
        except Exception:
            continue
        if pattern.search(text):
            bad.append(os.path.relpath(p, root))
print("files modified since the marker scanned; `repo repo` in %d file(s)" % len(bad))
for b in bad[:40]:
    print("  REPO-REPO", b)
sys.exit(1 if bad else 0)
