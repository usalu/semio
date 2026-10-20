#!/usr/bin/env python3
"""🧷️ Joins the ide-free manifest rows to the round-5 lock-sync batch: writes staged/lock-sync-plan-merged.json (same schema as lock-sync-tools/staged/plan.json, group `manifests`).
Use: python3 lock-sync-tools/apply.py --plan <this file> (dry run, then run.py's preflight/apply with --plan), so one lock/sync cycle carries every manifest edit. Read only on the live tree."""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
base = json.load(open(os.path.join(C, "lock-sync-tools", "staged", "plan.json"), encoding="utf-8"))
mine = json.load(open(os.path.join(HERE, "staged", "plan.json"), encoding="utf-8"))
files = {f["path"]: f for f in base["files"]}
added = 0
for e in mine["edits"]:
    if e["group"] != "manifests": continue
    entry = files.setdefault(e["path"], {"path": e["path"], "edits": []})
    if entry not in base["files"]: base["files"].append(entry)
    entry["edits"].append({"id": e["id"], "group": "manifests", "old": e["old"], "new": e["new"], "counts": e["counts"], "source": "ide-free manifests"})
    added += 1
out = os.path.join(HERE, "staged", "lock-sync-plan-merged.json")
json.dump(base, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(f"merged {added} edits into {len(files)} files -> {out}")
