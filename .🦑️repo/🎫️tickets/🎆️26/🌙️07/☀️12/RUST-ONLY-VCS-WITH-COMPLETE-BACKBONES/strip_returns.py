#!/usr/bin/env python3
"""🧹️ Temp helper: convert an old handle_action match's per-arm op-return expressions
(`vec![set_document_op(&envelope)]` / bare `Vec::new()`) into unit `{}` so the match becomes a
pure mutation statement. Operates on an inclusive 1-based line range only, to avoid touching the
sibling puzzle modules in the same file."""
import sys

path, start, end = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
with open(path, encoding="utf-8") as handle:
    lines = handle.readlines()

for index in range(start - 1, end):
    line = lines[index]
    stripped = line.strip()
    if stripped == "vec![set_document_op(&envelope)]":
        lines[index] = line.replace("vec![set_document_op(&envelope)]", "{}")
    elif stripped == "Vec::new()":
        lines[index] = line.replace("Vec::new()", "{}")
    elif stripped == "_ => Vec::new(),":
        lines[index] = line.replace("_ => Vec::new(),", "_ => {}")
    elif "vec![set_document_op(&envelope)]" in line:
        lines[index] = line.replace("vec![set_document_op(&envelope)]", "{}")

with open(path, "w", encoding="utf-8") as handle:
    handle.writelines(lines)
print("done")
