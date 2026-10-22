#!/usr/bin/env python3
"""🧷️ Applies anchored edits to live files: each anchor must occur exactly once, the file is re-read right before the write and replaced atomically. usage: anchored.py <edits.py> [--write]; edits.py defines EDITS = [(path, anchor, replacement), ...] (path relative to the repository root)."""
import os, sys, runpy, tempfile
ROOT = "/Users/ueli/Documents/semio"
edits = runpy.run_path(sys.argv[1])["EDITS"]
write = "--write" in sys.argv
texts = {}
for path, anchor, replacement in edits:
    absolute = os.path.join(ROOT, path)
    text = texts.get(path) or open(absolute, encoding="utf-8").read()
    count = text.count(anchor)
    if count == 0 and text.count(replacement) >= 1:
        print("SKIP (already applied):", path[-40:], anchor[:60].replace("\n", "\\n"))
        texts[path] = text
        continue
    if count != 1:
        raise SystemExit(f"anchor occurs {count} times in {path}: {anchor[:100]!r}")
    texts[path] = text.replace(anchor, replacement)
    print("OK", path[-50:], anchor[:60].replace("\n", "\\n"))
if write:
    for path, text in texts.items():
        absolute = os.path.join(ROOT, path)
        fd, tmp = tempfile.mkstemp(dir=os.path.dirname(absolute), prefix=".gp2-")
        with os.fdopen(fd, "w", encoding="utf-8") as handle:
            handle.write(text)
        os.chmod(tmp, os.stat(absolute).st_mode & 0o7777)
        os.replace(tmp, absolute)
    print("WRITTEN", len(texts), "files")
else:
    print("dry run")
