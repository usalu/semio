#!/usr/bin/env python3
"""Merged copy of bzl-tests-rest for the live tree after conc-bazel: the adapter's `groups:tree.groups` became `groups:new Map([...views.values()]...)`, so anchor 12 of build-staged.py ends at `groups:`.
The copy lives in C/round4-tools/staged/bzl-tests-rest; running its build-staged.py re-stages from the live files."""
import os, shutil
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(os.path.dirname(HERE))
SOURCE, TARGET = C + "/bzl-tests-rest-tools", HERE + "/bzl-tests-rest"
shutil.rmtree(TARGET, ignore_errors=True)
shutil.copytree(SOURCE, TARGET, ignore=shutil.ignore_patterns("__pycache__", "backup"))
path = TARGET + "/build-staged.py"
text = open(path, encoding="utf8").read()
a = "...(modules.has(action.id)?{modules:modules.get(action.id)}:{})})),groups:tree.groups'"
b = "...(modules.has(action.id)?{modules:modules.get(action.id)}:{}),...(action.test&&witnessed.has(action.id)?{test:{...action.test,witness:true}}:{})})),groups:tree.groups'"
assert text.count(a) == 1 and text.count(b) == 1
text = text.replace(a, "...(modules.has(action.id)?{modules:modules.get(action.id)}:{})})),groups:'").replace(b, "...(modules.has(action.id)?{modules:modules.get(action.id)}:{}),...(action.test&&witnessed.has(action.id)?{test:{...action.test,witness:true}}:{})})),groups:'")
open(path, "w", encoding="utf8").write(text)
print("merged bzl-tests-rest: adapter anchor 12 ends at `groups:`")
