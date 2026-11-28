# 🏁️ Sourcing End to End — Final Summary 2026-11-28

## Verdict

The sourcing plugin works end to end on the current tree. The final plain-cargo build of the hub and the beams, slabs and windows extensions (00:46–01:00) was staged on the native dev host 127.0.0.1:6081. Every browser journey passes in English and German:

| Journey | Pass | Fail |
|---|---|---|
| Core (`sourcing-journey.ts`) | 34 | 0 |
| Editing (`journey-editing.ts`) | 40 | 0 |
| History (`journey-history.ts`) | 112 | 0 |
| Surfaces (`journey-surfaces.ts`) | 68 | 0 |

All four committed descriptors (`🛂️.descriptor.semio` / `🔣️.json`) match the built wasm.

## Product and Framework Fixes (with Reports)

| Area | Fix | Report |
|---|---|---|
| Boot | Retirement receipt of a shared publication window counts the direct units' progress (peer's shared-window design kept) | `boot-retirement-invariant-2026-11-27.md` |
| Revert | History replay `retire_controlled` funds the failure spare and runs the child birth before the general retirement effect | `history-revert-edit-2026-11-27.md` |
| Discard | Discard receipt grant is intersected with the remaining turn authority | `history-discard-2026-11-27.md` |
| Preview cleanup race | Issuer hand-off only after the preview base is released | `history-preview-cleanup-2026-11-27.md` |
| Open File | Generic buffered natural-file decoder (`BufferedNaturalDecodeCursor`); the sourcing JSON codec uses it | `open-file-livelock-2026-11-27.md` |
| Module toggles (lost update) | Per-module `ToggleFilterModule` event instead of a whole-list rewrite | `surfaces-regression-2026-11-27.md` |
| Rapid inputs | Config batches publish against the live generation; host settles waiters of orphan faults | `input-lane-stall-2026-11-27.md` |
| Grid marquee | Subtractive marquee that empties the selection now commits | `editing-selection-2026-11-27.md` |
| Curation store | `decode_framed` lends SQL control only where needed, so progress and cancel flow; mutations keep the ordinal binary wire | `curation-remaining-2026-11-27.md` |
| Cancellation | A cancelled job delivers a typed `Cancelled`; `StepContext::cancelled_turn()`; 5 jobs audited and fixed | `surface-fixture-final-2026-11-27.md`, `job-cancel-audit-2026-11-27.md`, `job-cancel-fixes-2026-11-27.md` |
| Repo build tool | Torn or NUL-filled history lines are dropped instead of failing every cargo build | `repo-json-parse-root-cause-2026-11-27.md` |
| Build cache | Removed NUL-corrupt view, sealed snapshot and spill blobs left by the disk-full force-unmount | `🗺️coordination-plan-2026-11-27.md` |
| Plain-cargo fallback | `NATIVE=1` path in `native-wasm-build.sh` with `native-profile.toml` | `native-wasm-fallback-2026-11-27.md` |
| Journeys | Shared `attachFolder`; corrected expectations (Restore after Accept, Alternatives hidden ≤ 1 line, ControlOrMeta, Select before marquee, de catalog name, Back via Escape) | per-area reports |

## Tests

- **Curation workspace:** 226/226.
- **`semio-plugin-surface-fixture`:** 34/34.
- **`semio-plugin-tests-15`:** 59/59.
- **Job crates:** `original-core-budget` 6/6, `original-return-and-retirement-contracts` 24/24.
- **New native behavioural tests** for revert replay, the window receipt (plus a birth-minimum boundary), discard receipt, back-to-back toggles and preview cleanup, all passing (`runtime-fix-tests-2026-11-27.md`).
- **Python oracles:** io, stock and mutate curation.

## Open Items

- **`semio-plugin-tests-14-2`:** compiles, 10/33 pass. 12 stall in typed-operation settlement; an agent is assigned (`plugin-tests-14-2-2026-11-28.md`).
- **Tests blocked by peers' in-flight API changes:** the snapshot-read test target (124 errors, lease test unrun) and the `LayoutExportJob` testkit.
- **Unreferenced file:** `GuestRelayLifecycleProbeJob` sits in a file no module mounts.
- **Repo full build:** a full `repo cargo` build after the JSON fix is not yet exercised; the plain-cargo fallback was used.

## Development Speed

Long-term proposals are in `🚀️development-speed-proposals-2026-11-27.md`.
