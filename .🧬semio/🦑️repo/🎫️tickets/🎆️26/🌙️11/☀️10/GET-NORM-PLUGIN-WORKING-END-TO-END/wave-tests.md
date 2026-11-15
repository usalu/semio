# wave-tests report (din4108, din16798, din18599, iso16757, vdi3805, en1997-1999 test targets)

## Key findings
1. Test targets must be built in the FAMILY workspace (`🗿️artifacts/<family>/Cargo.toml`, `T/wave-tests-ws.sh <log> check|test <family-dir>`), not in the hub component workspace: path crates that are not workspace members get no dev-dependencies there (that was the source of ~900 bogus `serde`/`serde_json`/`async_macros`/`pack` errors, and of the en1998 `oracle_driver`/`serde` failure).
2. Framework test drift (sqlite snapshot codecs now take an owner and an output slot and return `RetainedWorkOutcome`): new helpers in `🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners` (`decode_native`, `encode_native`, `export_sqlite`, `import_sqlite`, `OriginalSnapshotRefusal`; new dependency `semio-framework-artifact-reference`). All `sqlite/snapshot/tests` of din4108/16798/18599, iso16757, vdi3805, en1997/98/99 were codemodded onto them (`T/wave-tests-codemod.py`, `-codemod2.py` for `decode_sqlite_snapshot_record_native`).
3. Moved io mounts: mutation text/binary unit tests use `crate::text::*` / `crate::binary::*` (modules mounted in the mutations crate now).
4. Editor tests: `ActionMeta.view_state` takes `.into()` (`SharedAllocation`); `norm_live_acceptance!` macro (app-surface `🧪️tests/🔬️live-acceptance`, non-artifact) used paths of the pre-split crates: `$crate::viewport3d::{lane_overlays,toolbar}` -> `$crate::{lane_overlays,toolbar}`, `$crate::entity_editing::bulk_writes` -> `$crate::bulk_writes`, `touch_menu(.., None)`.
5. din16798 viewer test: `example`/`noncompliant_multi_fail` constructors do not exist -> `default()`/`noncompliant_office()`; din18599 viewer test -> `noncompliant_detached_house()`.
6. Source-boundary gate (32 files / 256 KiB / 4000 lines in test cfg; a manifest can only lower it): every editor crate was 35-39 files. Split by domain: the window/panel/command unit tests moved out of the lib into an integration target `[[test]] integration` (`<editor>/🧪️tests/🧩️integration/🦀️.rs` mounts the unchanged leaf test files via `#[path]` against the public editor surface; generator `T/wave-tests-editor-split.py`). Editor crates now 24-27 files (din4108 25, din16798 25, din18599 25, iso16757 27, vdi3805 24; en1997-99 below 32). Predictor: `T/wave-tests-closure.ts` (`ROOT=<dir> repo tools exec --light bun -- run`). All din/iso/vdi/en1997-99 crates are within the gate.
7. din18599 `-wm2` rename finished for the Rust module identifiers (coordinator request); proto field `internal_gains_w_m2` in `mutations/🛰️.proto` is a different message (kept).

## Per family status
| family | check --tests --workspace | tests |
|---|---|---|
| din16798 | PASS (exit 0, `d16798-3.log`, before the editor split) | not run: `test` runs d16798-t1/t2 refused by the source gate (editor), d16798-t3 (editor excluded) killed with exit 124 after >3000 s of capacity starvation |
| din4108, din18599, iso16757, vdi3805, en1997, en1998, en1999 | not compiled (source-only prep done: sqlite codemod, mutation import paths, view_state, editor split) | not run |

Next compile per family (after "TEST COMPILES RESUME"): `T/wave-tests-ws.sh <log> test <family-dir>` (compiles and runs; the editor integration target is new and unverified: items it needs from the lib must be `pub`).

## Status update (19:10)
- Limited go-ahead used: `T/wave-tests-ws.sh` now passes `--parallelism=3` (parallelism=1 raises the capacity quote to 4909 MB, 3 gives 3413 MB).
- din4108 `check --tests --workspace` queued as `d4108-3` (`🗑️generated/d4108-3.log`), waiting 90+ min at `door.capacity` (needs 3413 MB, spare 0.7-1.6 GB above the floor) behind the component run `GKOQUuyXSwf669pXUDSaaQ` (5 jobs, 5.3 GB). It starts by itself when memory frees; nothing else of mine is queued (an orphaned parallelism=1 request was killed).
- Remaining order once it runs: read `d4108-3.log` -> fix -> din18599, iso16757, vdi3805, en1997-99 `check --tests` -> `test` per family (din16798 first, `T/wave-tests-ws.sh <log> test <family>`).
