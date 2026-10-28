"""📓️ docs3-g: builds every staged doc batch from the guide as it is now. `python3 -I build.py` (graph-free, reads the guide, writes only below staged/)."""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib
import build_a, build_b, build_c  # noqa: F401  (importing registers the batches)

for batch in lib.EDITS:
    batch.write()
index = [{"name": b.name, "code": b.code, "lane": b.lane, "order": b.order, "edits": len(b.edits), "summary": b.summary} for b in sorted(lib.EDITS, key=lambda x: x.order)]
with open(os.path.join(lib.STAGED, "index.json"), "w", encoding="utf8") as handle:
    json.dump({"guide": lib.GUIDE, "baseSha256": lib.BASE, "batches": index}, handle, ensure_ascii=False, indent=1)
print("built %d batches against sha256 %s" % (len(index), lib.BASE[:12]))
for row in index:
    print("%-14s %2d edits  <- %s" % (row["name"], row["edits"], row["code"]))

LANDING = {
    "bench2": "any time (measurement only)", "mirror": "any time (tool exists)", "guard-go": "any time (guard-g edits are live)",
    "quietfc": "G1, right after the quietfc code batch (needs its text: exit 3 before)", "journal-fix": "G2, with verify2 `journal-fix`", "vouch": "G2, with verify2 `vouch` (after journal-fix)",
    "probes": "G2, with toolsetup `probes` (or `probes-after-conv`)", "purge": "G3, with purge-g `purge` (P0, land early)", "dashlock": "G3, with dashlock", "plan-once": "G3, with graphlat `plan-once`",
    "oracle2": "G4, with oracle2", "conv": "G4, with conv (after oracle2)", "xplat3": "G4, with xplat3 (creates the Windows section)", "platforms": "G4, with xplat4 `platforms`",
    "freespace": "G4, with xplat4 `freespace` (after xplat3)", "fleet": "G4, with obs-g `fleet` (same text as obs-g's apply_guide.py: either may land first)", "setup": "G4, with setup",
    "nxfix-core": "G6 INSTEAD of the guide file of nxfix `core` (core replaces the whole guide and refuses once any other guide edit landed)", "nxlock": "G6, with nxlock (after nxfix core)",
    "census2": "G6, with census2 (xor testmem2)", "fdleak-guard": "G6, with fdleak `guard` (after census2)", "jobsrv": "G6, with jobsrv",
}
lines = ["# Staged doc batches for the build guide (docs3-g)\n", "Guide: `%s`, base sha256 `%s`.\n" % (lib.GUIDE, lib.BASE),
         "Each batch: `python3 -I staged/<name>/stage.py check|apply|revert [--root DIR]` (idempotent, anchored, refuses on drift with exit 2, on a missing code prerequisite with exit 3, writes nothing then). Land the doc batch WITH its code batch and revert both together.\n",
         "| # | doc batch | land | code batch | edits | adds |", "|---|---|---|---|---|---|"]
for position, row in enumerate(sorted(lib.EDITS, key=lambda x: x.order), 1):
    lines.append("| %d | `%s` | %s | `%s` | %d | %s |" % (position, row.name, LANDING[row.name], row.code, len(row.edits), row.summary.replace("|", "/")))
with open(os.path.join(lib.STAGED, "INDEX.md"), "w", encoding="utf8") as handle:
    handle.write("\n".join(lines) + "\n")
