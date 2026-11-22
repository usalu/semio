"""Anchored in-place edit of a live file: python3 anchored_edit.py <file> <old-file> <new-file> [count]; loads the live file, requires the old text exactly `count` times (default 1), replaces and writes at once."""
import hashlib
import sys

path, old_path, new_path = sys.argv[1:4]
count = int(sys.argv[4]) if len(sys.argv) > 4 else 1
old, new = open(old_path, encoding="utf8").read(), open(new_path, encoding="utf8").read()
text = open(path, encoding="utf8").read()
found = text.count(old)
if found != count:
    raise SystemExit(f"anchor found {found} times, expected {count}")
before = hashlib.sha256(text.encode()).hexdigest()[:8]
text = text.replace(old, new)
open(path, "w", encoding="utf8").write(text)
print("ok", path.split("/")[-2], before, "->", hashlib.sha256(text.encode()).hexdigest()[:8])
