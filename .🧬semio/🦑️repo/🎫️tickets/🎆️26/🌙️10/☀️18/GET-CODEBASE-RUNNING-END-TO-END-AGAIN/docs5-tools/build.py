"""📓️ docs5: builds every staged doc batch from the documents as they are now. `python3 -I build.py` (graph-free, reads the documents, writes only below staged/)."""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib
import build_guide, build_readme  # noqa: F401  (importing registers the batches)

import liveness
base = json.load(open(os.path.join(lib.TOOLS, "claims_base.json"), encoding="utf8"))
alive = liveness.live(base["candidates"])
for batch in lib.BATCHES:
    text = " ".join((e.get("block") or e.get("new") or "") for e in batch.edits)
    batch.pending = [n for n in base["candidates"] if n not in alive and n in text]
    batch.write()
claims = {"live": base["live"] + [n for n in base["candidates"] if n in alive], "pending": {n: w for n, w in base["candidates"].items() if n not in alive}, "files": base["files"], "paths": base["paths"], "rootfiles": base["rootfiles"]}
with open(os.path.join(lib.TOOLS, "claims.json"), "w", encoding="utf8") as handle:
    json.dump(claims, handle, ensure_ascii=False, indent=1)
print("candidates live now: %s; pending: %s" % (sorted(n for n in base["candidates"] if n in alive) or "none", sorted(claims["pending"]) or "none"))
rows = [{"name": b.name, "doc": b.doc.rel, "baseSha256": b.doc.sha, "code": b.code, "order": b.order, "edits": len(b.edits), "summary": b.summary} for b in sorted(lib.BATCHES, key=lambda x: x.order)]
with open(os.path.join(lib.STAGED, "index.json"), "w", encoding="utf8") as handle:
    json.dump({"batches": rows}, handle, ensure_ascii=False, indent=1)
lines = ["# Staged doc batches (docs5)\n", "Each batch: `python3 -I staged/<name>/stage.py check|apply|revert [--root DIR]` (idempotent, anchored, exit 2 on drift, writes nothing then). Laws: `sh staged/<any>/verify.sh <mirror root>`.\n",
         "| # | batch | document | edits | adds |", "|---|---|---|---|---|"]
for position, row in enumerate(rows, 1):
    lines.append("| %d | `%s` | `%s` | %d | %s |" % (position, row["name"], row["doc"].split("/")[-1], row["edits"], row["summary"].replace("|", "/")))
with open(os.path.join(lib.STAGED, "INDEX.md"), "w", encoding="utf8") as handle:
    handle.write("\n".join(lines) + "\n")
print("built %d batches" % len(rows))
for row in rows:
    print("%-16s %2d edits  %s" % (row["name"], row["edits"], row["baseSha256"][:12]))
