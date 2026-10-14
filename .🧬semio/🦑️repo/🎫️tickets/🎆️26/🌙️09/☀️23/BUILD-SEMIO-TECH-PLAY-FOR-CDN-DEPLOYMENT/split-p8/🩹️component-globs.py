#!/usr/bin/env python3
"""🩹️ Restores `pub use component::*;` in every inline module of the given crate roots that mounts a private `component`
file without re-exporting it (repairs roots emitted before the `✂️crate-cut.py` glob fix).

usage: 🩹️component-globs.py <root.rs>...
"""
import re, sys
for path in sys.argv[1:]:
    lines = open(path, encoding="utf-8").read().split("\n")
    out, stack = [], []
    for line in lines:
        if re.match(r"^\s*(pub\s+)?mod\s+\w+\s*\{\s*$", line):
            stack.append({"component": False, "glob": False, "indent": len(line) - len(line.lstrip())})
        elif re.match(r"^\s*mod component;\s*$", line) and stack:
            stack[-1]["component"] = True
        elif re.match(r"^\s*pub use component::\*;\s*$", line) and stack:
            stack[-1]["glob"] = True
        elif re.match(r"^\s*\}\s*$", line) and stack:
            frame = stack.pop()
            if frame["component"] and not frame["glob"]:
                out.append(" " * (frame["indent"] + 4) + "pub use component::*;")
        out.append(line)
    text = "\n".join(out)
    if text != "\n".join(lines):
        open(path, "w", encoding="utf-8").write(text)
        print("fixed", path)
