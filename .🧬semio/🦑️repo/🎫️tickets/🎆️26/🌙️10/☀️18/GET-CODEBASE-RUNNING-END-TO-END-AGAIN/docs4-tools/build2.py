"""📓️ docs4 round 2: builds the staged2 batches from the guide as it is now. `DOCS4_STAGED=staged2 python3 -I build2.py` (graph-free, writes only below staged2/)."""
import json
import os
import sys

os.environ["DOCS4_STAGED"] = "staged2"
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib
import build_guide2  # noqa: F401  (importing registers the batches)

for batch in lib.BATCHES:
    batch.write()
rows = [{"name": b.name, "doc": b.doc.rel, "baseSha256": b.doc.sha, "code": b.code, "order": b.order, "edits": len(b.edits), "summary": b.summary} for b in sorted(lib.BATCHES, key=lambda x: x.order)]
with open(os.path.join(lib.STAGED, "index.json"), "w", encoding="utf8") as handle:
    json.dump({"batches": rows}, handle, ensure_ascii=False, indent=1)
lines = ["# Staged doc batches, round 2 (docs4)\n", "Each batch: `python3 -I staged2/<name>/stage.py check|apply|revert [--root DIR]` (idempotent, anchored, exit 2 on drift, writes nothing then). Laws: `DOCS4_STAGED=staged2 python3 -I verify.py <mirror root>`.\n",
         "| # | batch | edits | adds |", "|---|---|---|---|"]
for position, row in enumerate(rows, 1):
    lines.append("| %d | `%s` | %d | %s |" % (position, row["name"], row["edits"], row["summary"].replace("|", "/")))
with open(os.path.join(lib.STAGED, "INDEX.md"), "w", encoding="utf8") as handle:
    handle.write("\n".join(lines) + "\n")
print("built %d batches" % len(rows))
for row in rows:
    print("%-16s %2d edits  %s" % (row["name"], row["edits"], row["baseSha256"][:12]))
