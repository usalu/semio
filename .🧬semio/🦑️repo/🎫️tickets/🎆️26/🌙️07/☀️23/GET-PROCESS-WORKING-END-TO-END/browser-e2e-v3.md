# Browser E2E v3 (Process 3D, native route, rebuilt hub)

## Result: NOT green. Different blocker than v2: stage=24 / zero-progress fault is gone; now setContributions publication blocks.

## Steps
1. Repo route: not retried; `repo-tooling-recheck.md` (2026-11-22) shows `repo run` never leaves the project-graph phase (timeout 2700 s). Went native.
2. Disk 22 GB before; hub rebuilt with `hub-v4.sh` (wasm32-wasip2, wasm-prototype, -j 3, CARGO_BUILD_BUILD_DIR=`🗑️generated/native/build`) at 11:51-12:19; no compile errors (warnings only). Disk bottomed at ~12 GB, 14 GB at the end. Wasm 405,631,328 B.
3. `publish-modules.ts <generated-dir> closure-hub.json` (note: first arg is the generated dir) and `activate-native.ts process-closure.json`; vite on 6022 (pid 25230) kept running (HTTP 200).
4. `verify-process-firefox-bidi.ts` with SETTLE_MS=150000: stdout `🗑️generated/e2e-stdout-v4.txt`, report `🗑️generated/e2e-report.json`, screenshot `🗑️generated/e2e-process-v4.png`.

## Outcome
- canvases = 0; chrome renders (Artifact/Catalogue/Workshop..., "semio · process · 3d"); status bar "Document load: Loading document: 125 of 125 ... Cancel replay"; banner "has not answered an interaction for 92 s".
- No `stage=24` / `FEM cleanup semantic failure` / `native-return-refused` any more (v2 blocker resolved by the newer maintenance-ladder source).
- Console error: `setContributions command failed ... interactive-job.publication-stalled: typed operation 'setContributions' advanced 4096 publication units without moving its ladder` (retainedProgress all 0).
- Guest `[DEBUG]` lines preceding it: `admit_loan heap=(0,0) receipt_delta=128`, `one-item publication blocked` (from `🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs`, `ArtifactStoreOneItemAdvance::Blocked` branch), `stalled 'setContributions' pending_artifact=true pending_child=false owned_group=false publication=true plan=false ack=false` (`👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs:203`). Then `recv exit tag=5` / `tag=8` repeating, `[os-shell] command stalled: process interaction has held process#1's lane`.
- Interpretation: the one-item store publication of `setContributions` returns `Blocked` forever (no advance of the ladder), matching the area other agents are fixing (replication seal cursor "page allocation attempt is occupied", captured-original / receiving refund). Not changed by me; files under `🏪️store/🎮️one-item/📬️publication` and typed-operation-unit are being edited concurrently (mtime < 30 min).

## Next
Re-run after the other agents land: rebuild hub (~28 min, ~10 GB), `publish-modules.ts $G closure-hub.json`, `activate-native.ts process-closure.json`, then the verify script. The `[DEBUG] one-item publication blocked` line should disappear first.

## Files changed
No source edits. Ticket: this file, `🗑️generated/hub-v4.sh`, `e2e-stdout-v4.txt`, `e2e-process-v4.png`, `e2e-report.json`. Published (cache): modules `🏭️process` + activation receipt.
