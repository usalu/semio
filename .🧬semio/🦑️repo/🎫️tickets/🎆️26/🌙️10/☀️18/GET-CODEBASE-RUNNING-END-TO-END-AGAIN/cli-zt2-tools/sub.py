"""Exact anchored replacement inside an overlay file; usage: sub.py <file> <old> <new> [count=1]. Refuses unless the anchor occurs exactly count times."""
import sys
path, old, new = sys.argv[1:4]
count = int(sys.argv[4]) if len(sys.argv) > 4 else 1
text = open(path, encoding="utf-8", newline="").read()
if text.count(old) != count: sys.exit(f"{path}: anchor count {text.count(old)} != {count}: {old[:80]!r}")
open(path, "w", encoding="utf-8", newline="").write(text.replace(old, new))
