#!/usr/bin/env python3
"""Fix nx project.json cwd/sourceRoot/$schema for the emoji tree.
Invariant discovered by audit: every project.json has at most one distinct
cwd value across all its targets, and that value always equals the file's
own containing directory (nx commands run `bun ./script.ts ...` from there).
So the fix is purely mechanical: recompute cwd/sourceRoot as the file's own
repo-root-relative dir, and recompute $schema's ../ depth to match.
"""
import json
import glob
import os

SKIP = ("node_modules", "/target/", "compose/", ".git/")

fixed = 0
unchanged = 0
errors = []

for f in glob.glob("**/project.json", recursive=True):
    if any(s in f for s in SKIP):
        continue
    own_dir = os.path.dirname(f)
    depth = 0 if own_dir in ("", ".") else own_dir.count("/") + 1
    schema_target = "/".join([".."] * depth + ["node_modules/nx/schemas/project-schema.json"])
    try:
        raw = open(f, encoding="utf-8").read()
        d = json.loads(raw)
    except Exception as e:
        errors.append((f, str(e)))
        continue
    changed = False
    if d.get("$schema") != schema_target:
        d["$schema"] = schema_target
        changed = True
    if "sourceRoot" in d and d["sourceRoot"] != own_dir:
        d["sourceRoot"] = own_dir
        changed = True
    for t in d.get("targets", {}).values():
        opts = t.get("options")
        if isinstance(opts, dict) and "cwd" in opts and opts["cwd"] != own_dir:
            opts["cwd"] = own_dir
            changed = True
    if changed:
        new_raw = json.dumps(d, ensure_ascii=False, indent=2) + "\n"
        open(f, "w", encoding="utf-8").write(new_raw)
        fixed += 1
    else:
        unchanged += 1

print(f"fixed: {fixed}")
print(f"unchanged: {unchanged}")
print(f"errors: {len(errors)}")
for e in errors:
    print(e)
