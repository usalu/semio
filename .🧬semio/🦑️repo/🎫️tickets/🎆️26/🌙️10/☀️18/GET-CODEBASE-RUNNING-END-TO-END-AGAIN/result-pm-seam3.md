# pm-seam round 3 (READY3): re-cut against the combined round-6 tree

Cause of the red suite: other sets changed the neutral parts of the install and generation schema bases (sync-incr-3 text and version edits, new defs) and added two consumers that still named the javascript manager (`🏗️build/📜️script.ts`: resident command input paths; `🏗️build/🧹️cache-gc/🚀️launch/🟦️.ts`: environment allowlist). The suite pinned whole schema documents and a per-file literal count, so every neutral edit by a peer turned it red.

## What round 3 changes (staged under `pm-seam-tools/staged3/`, groups `seam3`, `ratchet3`, apply with `--round 3` after round 2 and every other set)
- Regions instead of documents: `managerRegionsV1(base, view)` lists the positions of a base that depend on the manager with their materialized values. Vectors (`materialized` digests), the Python oracle and the pre-seam originals (`🧫️fixtures/📥️install.original.json`, `🧾️generation.original.json`, now region lists of 26 and 9 rows) pin only those positions. Neutral edits around them no longer break anything. One amendment is recorded in `mkregions.py` (sync-incr-3 inserted a phrase inside the install description).
- `runtime.hostEnvironment` (descriptor schema, interface, `hostEnvironment()` on the manager, accessor, six descriptors; bun: `BUN_INSTALL`). The cache collector asks the selected manager; the resident command asks it for the lock and scope configuration names. Ratchet is zero again (0 files, 0 literals), and the neutrality law also scans the new tokens.
- Vectors re-minted by the oracle (only descriptor and materialized digests moved).

## Evidence
- Combined tree `ckpt-hot-cache` (all sets through hot-cache) plus round 3: managers suite 88/88 (before: 84/88).
- `apply.py apply|check|undo --round 3` proven on a copy: 27 operations, result byte-equal to the verified tree, idempotent, undo restores the base.
- Tools: `plan3.py`, `plan3_sources.py`, `plan3_regions.py`, `plan3_tests.py`, `mkregions.py`, `fillmirror.py`.
- Type check of the touched files equals the base diagnostics; cache-gc launch tests 6/6 in both trees.
- Not verified: sets after hot-cache (agent-session, perf-slo, ide-free, langs-conc, graph-mem) were in no usable checkpoint. Run the managers suite once in the final overlay.
