"""Replaces the lines <first>..<last> (1-based, inclusive) of a UTF-8 file with one placeholder line, keeping the file's
line endings, after checking that the first line contains <expected>: cut_lines.py <file> <first> <last> <expected> <placeholder>."""
import io
import sys

path, first, last, expected, placeholder = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4], sys.argv[5]
with io.open(path, encoding="utf-8", newline="") as source:
    lines = source.read().split("\n")
if expected not in lines[first - 1]:
    raise SystemExit(f"line {first} does not contain {expected!r}: {lines[first - 1][:120]!r}")
ending = "\r" if lines[first - 1].endswith("\r") else ""
print(f"cutting {first}-{last}; last line cut: {lines[last - 1][:60]!r}; next: {lines[last][:60]!r}")
lines[first - 1 : last] = [placeholder + ending]
with io.open(path, "w", encoding="utf-8", newline="") as target:
    target.write("\n".join(lines))
