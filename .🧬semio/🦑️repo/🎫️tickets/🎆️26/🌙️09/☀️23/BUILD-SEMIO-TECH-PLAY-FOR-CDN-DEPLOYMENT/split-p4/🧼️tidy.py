#!/usr/bin/env python3
"""🧼️ Removes `//#region X` … `//#endregion X` pairs that enclose nothing but blank lines and collapses runs of blank lines
to one, in the given Rust files.

usage: 🧼️tidy.py <file.rs>...
"""
import re, sys
for path in sys.argv[1:]:
    text = open(path, encoding="utf-8").read()
    new = text
    while True:
        nxt = re.sub(r"(?m)^[ \t]*//#region ([^\n]*)\n(?:[ \t]*\n)*[ \t]*//#endregion \1[ \t]*\n", "", new)
        if nxt == new:
            break
        new = nxt
    new = re.sub(r"\n{3,}", "\n\n", new)
    if new != text:
        open(path, "w", encoding="utf-8").write(new)
        print("tidied", path.split("✳️any/")[-1].split("🗿️artifacts/")[-1])
