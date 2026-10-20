#!/usr/bin/env python3
"""Derives the merged copy of bzl-default for the flip group from the CURRENT original: the same apply.py and plan.json minus the edits of the generated help-snapshot fixture
(its Nx-default sentences occur 38 times once mem-gov, bzl-rbe, parity and bzl-incr added operations, the plan expects 24); the fixture is regenerated from the edited catalogue by the group's post step."""
import json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(os.path.dirname(HERE))
SOURCE = C + "/bzl-default-tools"
TARGET = HERE + "/bzl-default-flip"
FIXTURE = "📖️help/🔣️.json"
plan = json.load(open(SOURCE + "/staged/plan.json", encoding="utf8"))
kept = [item for item in plan["files"] if not item["path"].endswith(FIXTURE)]
dropped = len(plan["files"]) - len(kept)
assert dropped == 1, "expected exactly one help fixture entry, found %d" % dropped
os.makedirs(TARGET + "/staged", exist_ok=True)
shutil.copyfile(SOURCE + "/apply.py", TARGET + "/apply.py")
json.dump({**plan, "files": kept}, open(TARGET + "/staged/plan.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("merged flip plan: %d files (help fixture entry removed)" % len(kept))
