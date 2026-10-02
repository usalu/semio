"""Replaces the `//#region … <name>` block (through its `//#endregion … <name>` line) of a UTF-8 file with one
placeholder line, keeping the file's line endings: cut_region.py <file> <name> <placeholder>."""
import io
import sys

path, name, placeholder = sys.argv[1:4]
with io.open(path, encoding="utf-8", newline="") as source:
    lines = source.read().split("\n")
first = next(index for index, line in enumerate(lines) if line.startswith("//#region") and line.rstrip().endswith(name))
last = next(index for index, line in enumerate(lines) if line.startswith("//#endregion") and line.rstrip().endswith(name))
ending = "\r" if lines[first].endswith("\r") else ""
lines[first : last + 1] = [placeholder + ending]
with io.open(path, "w", encoding="utf-8", newline="") as target:
    target.write("\n".join(lines))
print(f"cut lines {first + 1}-{last + 1}")
