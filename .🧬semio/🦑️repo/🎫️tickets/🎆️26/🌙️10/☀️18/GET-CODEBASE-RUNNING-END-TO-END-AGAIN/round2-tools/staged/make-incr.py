#!/usr/bin/env python3
"""Derives the merged copy of bzl-incr from the CURRENT original: the same tool directory with the edit of the generated help-snapshot fixture removed (its row anchors, e.g. `(used by: graph, show)`,
changed when mem-gov added `capacity` to `--json`); the fixture is regenerated from the edited catalogue by the group's post step (regen-help.ts)."""
import os, shutil
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(os.path.dirname(HERE))
SOURCE, TARGET = C + "/bzl-incr-tools", HERE + "/bzl-incr"
shutil.rmtree(TARGET, ignore_errors=True)
shutil.copytree(SOURCE, TARGET, ignore=shutil.ignore_patterns("__pycache__"))
path = TARGET + "/apply.py"
text = open(path, encoding="utf8").read()
old = "edits=list(wire.EDITS)"
assert text.count(old) == 1
open(path, "w", encoding="utf8").write(text.replace(old, 'edits=[row for row in wire.EDITS if "\U0001F4D6️help" not in row[0]]'))
print("merged bzl-incr: help fixture edit removed")
