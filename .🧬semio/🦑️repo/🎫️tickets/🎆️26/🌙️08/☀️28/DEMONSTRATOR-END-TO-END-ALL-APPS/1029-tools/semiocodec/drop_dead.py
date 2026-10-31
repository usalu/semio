#!/usr/bin/env python3
"""Drops the dead hand-pack constants a ported adapter no longer reads, with the `#:` comment block above each. usage: drop_dead.py <file> NAME..."""
import ast
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
names = set(sys.argv[2:])
lines = path.read_text(encoding="utf-8").split("\n")
tree = ast.parse("\n".join(lines))
spans = []
for node in tree.body:
    if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id in names for t in node.targets):
        start = node.lineno - 1
        while start > 0 and lines[start - 1].startswith("#:"):
            start -= 1
        spans.append((start, node.end_lineno))
for start, end in sorted(spans, reverse=True):
    print("drop", "\n     ".join(lines[start:end]))
    del lines[start:end]
text = "\n".join(lines)
while "\n\n\n\n" in text:
    text = text.replace("\n\n\n\n", "\n\n\n")
path.write_text(text, encoding="utf-8")
