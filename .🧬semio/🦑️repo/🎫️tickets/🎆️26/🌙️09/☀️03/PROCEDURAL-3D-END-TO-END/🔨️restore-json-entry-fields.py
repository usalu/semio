#!/usr/bin/env python3
"""🩹 Restores path-bearing fields of workspace `package.json` / `📋️project.json` files (`exports`, `main`,
`module`, `types`, `bin`, `files`; `sourceRoot`, `targets.*.options.cwd`) from HEAD when the current value
was rewritten by the 2026-09-03 codemod — detected as a key emptied of its dot (`"."` → `""`) or a path that
no longer resolves relative to the file. Only those fields move; every other edit stays.
Usage: 🔨️restore-json-entry-fields.py [--apply]
"""
import json, os, subprocess, sys
REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip()
apply = "--apply" in sys.argv
status = subprocess.run(["git", "status", "--porcelain=v1", "-z"], capture_output=True, text=True, cwd=REPO, check=True).stdout
files = [e[3:] for e in status.split("\0") if len(e) > 3 and "M" in e[:2] and (e[3:].endswith("package.json") or e[3:].endswith("📋️project.json")) and not e[3:].startswith(".🧬semio/")]
def paths_in(value):
    if isinstance(value, str): return [value]
    if isinstance(value, dict): return [p for v in value.values() for p in paths_in(v)]
    if isinstance(value, list): return [p for v in value for p in paths_in(v)]
    return []
def broken(value, base):
    if isinstance(value, dict) and "" in value: return True
    return any(p.startswith(".") and not os.path.exists(os.path.join(base, p)) for p in paths_in(value) if isinstance(p, str))
changed_files = 0
for rel in files:
    head = subprocess.run(["git", "show", f"HEAD:{rel}"], capture_output=True, text=True, cwd=REPO)
    if head.returncode != 0: continue
    try: head_doc, doc = json.loads(head.stdout), json.load(open(os.path.join(REPO, rel), encoding="utf-8"))
    except Exception: continue
    base = os.path.dirname(os.path.join(REPO, rel)); touched = []
    for key in ("exports", "main", "module", "types", "bin", "files", "sourceRoot"):
        if key in doc and key in head_doc and doc[key] != head_doc[key] and broken(doc[key], base):
            doc[key] = head_doc[key]; touched.append(key)
    for name, target in (doc.get("targets") or {}).items():
        cwd = (target.get("options") or {}).get("cwd"); hcwd = ((head_doc.get("targets") or {}).get(name) or {}).get("options", {}).get("cwd")
        if cwd and hcwd and cwd != hcwd and not os.path.isdir(os.path.join(REPO, cwd)) and os.path.isdir(os.path.join(REPO, hcwd)):
            target["options"]["cwd"] = hcwd; touched.append(f"targets.{name}.cwd")
    if touched:
        changed_files += 1; print(f"  {rel}: {', '.join(touched)}")
        if apply:
            raw = open(os.path.join(REPO, rel), encoding="utf-8").read()
            indent = 2 if raw.lstrip().startswith("{\n  ") else 4
            open(os.path.join(REPO, rel), "w", encoding="utf-8").write(json.dumps(doc, ensure_ascii=False, indent=indent) + ("\n" if raw.endswith("\n") else ""))
print(f"{'applied' if apply else 'dry-run'}: files={changed_files}")
