"""Builds plan.json: relative-import replacements (rel-plan.ts output) + authored E2E readiness edits + frozen manifest edits, each with base and result sha256."""
import hashlib, json, sys, os
sys.path.insert(0, os.path.dirname(__file__))
import importlib.util
spec = importlib.util.spec_from_file_location("e2e_edits", os.path.join(os.path.dirname(__file__), "e2e-edits.py"))
edits = importlib.util.module_from_spec(spec); spec.loader.exec_module(edits)

root, rel_plan_path, out_path = sys.argv[1], sys.argv[2], sys.argv[3]
sha = lambda text: hashlib.sha256(text.encode("utf-8")).hexdigest()

def apply(text, rows, path):
    for old, new, count in rows:
        found = text.count(old)
        if found != count:
            raise SystemExit(f"anchor count {found} != {count} in {path}: {old[:80]!r}")
        text = text.replace(old, new)
    return text

merged = {}
for item in json.load(open(rel_plan_path))["files"]:
    merged.setdefault(item["path"], {"frozen": False, "rows": []})["rows"] += [(r["old"], r["replacement"], r["count"]) for r in item["replacements"]]
for path, rows in edits.EDITS.items():
    merged.setdefault(path, {"frozen": False, "rows": []})["rows"] += rows
for path, rows in edits.FROZEN.items():
    merged.setdefault(path, {"frozen": True, "rows": []})["rows"] += rows

files = []
for path, item in sorted(merged.items()):
    text = open(os.path.join(root, path), encoding="utf-8", newline="").read()
    after = apply(text, item["rows"], path)
    files.append({"path": path, "frozen": item["frozen"], "sha256": sha(text), "afterSha256": sha(after), "replacements": [{"old": o, "new": n, "count": c} for o, n, c in item["rows"]]})
new_files = [{"path": p, "sha256": sha(c), "content": c} for p, c in sorted(edits.NEW_FILES.items())]
json.dump({"version": 1, "files": files, "newFiles": new_files}, open(out_path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(f"files={len(files)} (frozen={sum(f['frozen'] for f in files)}) newFiles={len(new_files)} replacements={sum(len(f['replacements']) for f in files)}")
