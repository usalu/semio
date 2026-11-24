# Browser E2E v5 (Process 3D, native route)

## Result: GREEN for the goal. One canvas renders (1 canvas from ~100 s, Firefox BiDi, native route, hub v17), UI shows Workpiece/Projection/Window Options/Actions panels, no `publication-stalled`, no `one-item publication blocked`.

Evidence: `🗑️generated/e2e-report.json` (probe `canvases: 1`), `🗑️generated/e2e-stdout-v17b.txt`, screenshot `🗑️generated/e2e-process-v17b.png`.
Timeline (SETTLE 150 s): 40 s and 70 s 0 canvases (document load 125 of 125, archive load), 100 s and 130 s 1 canvas.
Remaining console errors: `setContributions command failed process app-channel.disposed` (twice, page teardown/reload after the canvas appeared) and the already-known `setActiveExample refused: instance-retired`. Not investigated further.

## Root-cause chain (this task)
1. Seal digest stalled with a 0 work quote (v4 report): `ArtifactCanonicalEditSealCursor::next_work_demand` now floors its source take/close/retirement quotes at 1 (law: `paged_seal_digest` asserts a non-zero quote on every unsettled turn).
2. After that, other agents' concurrent changes to the typed-operation publication unit (receiving-scope funding, closing-publication retirement, per-site wait witnesses) moved the failure through three messages ("refused the granted page", "grant cannot fund one receiving step", "pending publication is closing"). My probes at `NativeBatchDigest::take_edit`, every `blocked_here()` site after line 301 and the unit's funding check never fired in v13..v15 (the publication was already past them), and the scalar-close probe never fired in v16/v17: so none of those sites was the refuser, and no further product edit of mine was needed.
3. v17 (tree of 2026-11-24 00:39): `setContributions` completes, UI patches accepted, document archive load proceeds, canvas renders.

## Not done / caveats
- No new law beyond the seal quote one; the later stalls were fixed in other agents' files, so a law for them belongs to those changes.
- `history_edits_end_to_end` left to its owner as instructed.
- v14 and v16 builds first failed on other agents' in-flight edits and a transient `rustc` working-directory error; `hub-retry*.sh` (retry loop) reran them.

## Cleanup
All my `[DEBUG]` edits are removed (receiving work, bundle, digest, seal, typed-operation-unit, scalar close). Remaining `[DEBUG]` lines in the console (`command turn`, `drain`, `ui patches`, `completion refresh`, `loadDocumentArchive`, `scope body`) belong to other agents.
Ticket files: this file, `🗑️generated/hub-v5.sh`..`hub-v17.sh`, `hub-retry*.sh`, logs and screenshots (generated; delete at ticket close).
