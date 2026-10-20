#!/usr/bin/env python3
# usage: export.py   turns the recorded overlay edits into staged/patches/lib-defects.json (anchored old/new/count rows per file)
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
rows = [json.loads(line) for line in open("/private/tmp/claude-501/ld.edits.jsonl", encoding="utf-8")]
by = {}
for row in rows:
    by.setdefault(row["rel"], []).append({"old": row["old"], "new": row["new"], "count": row["count"]})
out = [{"file": rel, "edits": edits} for rel, edits in sorted(by.items())]
with open(os.path.join(HERE, "staged", "patches", "lib-defects.json"), "w", encoding="utf-8") as handle:
    json.dump(out, handle, ensure_ascii=False, indent=1)
print(len(out), "files", len(rows), "edits")
