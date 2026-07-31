#!/usr/bin/env python3
"""Round-3 path translator: longest-prefix match over the crate/package move map.
Given a repo-root-relative OLD path (a crate dir itself, or a file/dir inside one),
returns the equivalent NEW path. Paths outside the moved set pass through unchanged."""
import json
import os

_HERE = os.path.dirname(os.path.abspath(__file__))
with open(os.path.join(_HERE, "moves-v3.json"), encoding="utf-8") as f:
    _MOVES = json.load(f)

# sort by descending old-path length so the longest (most specific) prefix wins
_MOVES_SORTED = sorted(_MOVES, key=lambda m: -len(m["old"]))


def translate(old_relpath: str) -> str:
    for m in _MOVES_SORTED:
        old = m["old"]
        if old_relpath == old:
            return m["new"]
        if old_relpath.startswith(old + "/"):
            return m["new"] + old_relpath[len(old) :]
    return old_relpath


if __name__ == "__main__":
    import sys

    for line in sys.stdin:
        line = line.rstrip("\n")
        if not line:
            continue
        print(translate(line))
