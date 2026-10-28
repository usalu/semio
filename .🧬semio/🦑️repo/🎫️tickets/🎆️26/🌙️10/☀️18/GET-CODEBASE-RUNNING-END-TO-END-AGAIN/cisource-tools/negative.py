#!/usr/bin/env python3
"""🧨 Negative controls of batch `cisrc`: seeded violations on a MIRROR (never live); each must fail exactly the named tests of the dashboard CI law and the files are restored byte for byte.
usage: python3 -I negative.py MIRROR_ROOT"""
import os, re, subprocess, sys
root = os.path.realpath(sys.argv[1])
if "/sm/" not in root: sys.exit("refusing: not a mirror")
DASH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/"
TEST = DASH + "🧪️tests/🏗️ci/🟦️.ts"
FIXTURE = DASH + "🧪️tests/🏗️ci/🔣️.json"
MANIFEST = DASH + "📦️packages/🦀️rust/📋️project.json"
CLOSED = "dashboard CI host and gate vectors are closed and independently ordered"
SOURCE = "dashboard CI Source gate is closed and reaches the owned early test branch"
READS = "dashboard CI Source gate declares every workflow and manifest file its test reads"
OWN = '{projectRoot}/📋️project.json'
WORKFLOW = "{workspaceRoot}/.github/workflows/dashboard.yml"
CASES = [
    ("fixture entry removed only (the drift this batch fixes)", [("sub", FIXTURE, OWN, None), ], {CLOSED, SOURCE}),
    ("manifest input removed only", [("sub", MANIFEST, OWN, None)], {SOURCE, READS}),
    ("manifest input removed everywhere", [("sub", FIXTURE, OWN, None), ("sub", MANIFEST, OWN, None)], {CLOSED, READS}),
    ("new unlisted workflow read", [("append", TEST, '\nvoid document(".github/dependabot.yml");\n')], {READS}),
    ("dangling input spelled the same in fixture and manifest", [("rename", FIXTURE, WORKFLOW, WORKFLOW.replace("dashboard", "dashbord")), ("rename", MANIFEST, WORKFLOW, WORKFLOW.replace("dashboard", "dashbord"))], {READS}),
]
def run():
    out = subprocess.run(["bun", "test", "./" + TEST], cwd=root, capture_output=True, text=True)
    return {m.group(1) for m in re.finditer(r"^\(fail\) (.+?)(?: \[[\d.]+ms\])?$", out.stdout + out.stderr, re.M)}
def apply(edit):
    kind, path = edit[0], os.path.join(root, edit[1]); text = open(path, encoding="utf8").read()
    if kind == "append": return text + edit[2]
    if kind == "rename":
        if text.count(edit[2]) != 1: sys.exit(f"rename anchor found {text.count(edit[2])} times in {edit[1][-40:]}")
        return text.replace(edit[2], edit[3])
    index = text.index('"' + edit[2] + '"')
    start = text.rfind("\n", 0, index); end = text.index("\n", index)
    line = text[start:end]
    before = text[:start]; after = text[end:]
    if not line.rstrip().endswith(","): before = before.rstrip()[:-1] if before.rstrip().endswith(",") else before
    return before + after
backup = {}
bad = 0
assert run() == set(), "controls need a green start"
try:
    for label, edits, expected in CASES:
        print("case", label, flush=True)
        for edit in edits:
            path = os.path.join(root, edit[1]); backup.setdefault(path, open(path, "rb").read())
        for edit in edits:
            changed = apply(edit)
            open(os.path.join(root, edit[1]), "w", encoding="utf8").write(changed)
        failed = run(); ok = failed == expected; bad += not ok
        print(("OK  " if ok else "FAIL"), label, "-> failed:", sorted(f[:60] for f in failed), "" if ok else "expected: " + str(sorted(f[:60] for f in expected)))
        for path, data in backup.items(): open(path, "wb").write(data)
finally:
    for path, data in backup.items(): open(path, "wb").write(data)
assert run() == set(), "restore must leave the laws green"
sys.exit(1 if bad else 0)
