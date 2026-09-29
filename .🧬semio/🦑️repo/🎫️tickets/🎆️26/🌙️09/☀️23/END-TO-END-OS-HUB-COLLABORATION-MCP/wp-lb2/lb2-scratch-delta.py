#!/usr/bin/env python3
"""🩹️ LB2 scratch delta: re-applies ONE set's revised edits to a scratch that already carries an older revision of that set,
file by file, without reverting the shared files later sets also edit. For each named path: the pre-set text is the set's own
byte-exact backup when it has one (the older revision touched the file), else the scratch file itself (a newly touched file,
backed up now); the set's CURRENT edit function is applied to it and the result written. Moved files take a literal transform.

usage: python3 lb2-scratch-delta.py <set-script> <scratch-root> <path>[::<edit-function>]… [--sub <path> <old> <new>]…
(`::<edit-function>` names the set's edit function directly for a set without `plan()`.)
"""
import importlib.util, os, sys

script, root = sys.argv[1], sys.argv[2]
rest = sys.argv[3:]
paths, subs = [], []
while rest:
    if rest[0] == "--sub":
        subs.append(tuple(rest[1:4]))
        rest = rest[4:]
    else:
        paths.append(rest.pop(0))
sys.argv = [script, "--dry-run", "--root", root]
spec = importlib.util.spec_from_file_location("set", script)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
edits = module.plan() if hasattr(module, "plan") else {}
for spec_path in paths:
    path, _, function = spec_path.partition("::")
    edit = getattr(module, function) if function else edits[path]
    backup = os.path.join(module.BACKUP, path)
    target = os.path.join(root, path)
    if os.path.isfile(backup):
        before = open(backup, encoding="utf-8").read()
    else:
        before = open(target, encoding="utf-8").read()
        os.makedirs(os.path.dirname(backup), exist_ok=True)
        open(backup, "w", encoding="utf-8").write(before)
    after = edit(before)
    open(target, "w", encoding="utf-8").write(after)
    print("applied" if after != before else "UNCHANGED", path)
for path, old, new in subs:
    target = os.path.join(root, path)
    text = open(target, encoding="utf-8").read()
    print(f"sub x{text.count(old)}", path)
    open(target, "w", encoding="utf-8").write(text.replace(old, new))
for problem in module.problems:
    print("PROBLEM", problem)
sys.exit(1 if module.problems else 0)
