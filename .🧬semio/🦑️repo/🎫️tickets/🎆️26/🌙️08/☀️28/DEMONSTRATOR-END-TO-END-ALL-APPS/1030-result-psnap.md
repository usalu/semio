STATUS (12:40): snapshot-read wasm --lib GREEN incl. new `SnapshotReadSourceStaging` (psnap-w6); all callers ported, check of the caller crates waits for upstream (psnap-w6 stops in replication-mutation-messages 15 / replication-history 23 errors, foreign). Earlier: snapshot-read GREEN (psnap-w3). Callers ported (below) but UNVERIFIED: batched check psnap-w5 stops upstream in os-store-link-schema, replication-edit, ui-contract-values (foreign). Lib tests blocked by semio-framework-async (2 errors, psnap-t5); my test edits uncompiled.

# 1030 psnap result (semio-framework-value-snapshot-read)

Mirrored the peer API exactly: `close_step -> RetainedWorkOutcome`, `RetirementStep::{Progress(p,u64),Failure(e,u64),Complete(u64)}`, `admit_owned_retirement`/`admit_shared_retirement`/`SnapshotRetirementFactory::retire` -> `RetainedWorkOutcome<(..),(ValueError,orig)>`, `RetainedCloneSource::admit_borrowed` -> staged `admission_borrowed` (`RetainedCloneSourceAdmission`), `FactoryAuthority::step -> RetainedWorkOutcome`.

## Changes
- `SnapshotRead::admit_retained_clone_source(grant)` replaced by `SnapshotRead::source_admission() -> Result<RetainedCloneSourceAdmission<T,Self>,(ValueError,Self)>` (same shape as the peer's `ReadLease::source_admission`).
- `try_admit_one_returned` now takes a closure returning `RetainedWorkOutcome<(R,Progress),(ValueError,Arc<T>)>` and returns `RetainedWorkOutcome<(Option<R>,Progress),SnapshotReadLeaseRefusal>` (work = admission work, else 1 when anything moved).
- `artifact_retirement_box_close_step`, `artifact_retirement_admit_owned`, `artifact_retirement_admit_shared_lease`, `advance_returned_snapshot_read`, `advance_returned_snapshot_read_root` return `RetainedWorkOutcome`.
- New helpers `bounded_outcome` (one bounded action = 1 work unit iff any currency moved, refusals/starved = 0) and `outcome_retirement_step`.
- Cursors (`SnapshotReadRetirement`, `ErasedSnapshotReadRetirement`, `CapturedSnapshotRetirement`, `SnapshotReadProjection` cursor, return cursor) report exact work; nested child work is forwarded (`last_turn_work`, box close, factory step). Registry alias `ErasedSnapshotRetirement::close_step` wraps `step` via `bounded_outcome`. `snapshot_registry_alias_close_step`, `close_custody_step`, `close_owned_backing_step` stay `Result` (bounded single actions).
- Tests ported: `♻️snapshot-read-retirement`, return/projection tests, retirement unit test (staged source admission with per-currency refusals items/capacity=0/depth, heap-exact per turn, `take_output`), `🧪️testing/👥️presence-retirement`.

Files: `🧰️framework/🔨️modules/🌱️value/📸️snapshot/👁️read/{🦀️.rs,🔗️source/🦀️.rs,🧪️testing/👥️presence-retirement/🦀️.rs}`; under `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/`: `🔗️read/{♻️retirement,🧾️return,🪞️projection,🔒️registry/♻️retirement}/🦀️.rs`, their `🧪️tests`, `🧪️tests/♻️snapshot-read-retirement/🦀️.rs`. Scripts: `C/1030-tools/psnap/{wasm.sh,test.sh}`.

## Callers outside this crate that must be ported (other lanes)
`admit_retained_clone_source` -> `source_admission` + staged `advance`/`take_output`: store `🧬️snapshot-clone/🦀️.rs:369`, `📬️paged-one-item/🦀️.rs:460`, inference `🎟️source/🦀️.rs:60`, draw clipboard `🧵️job/🦀️.rs:131`, store tests `🧬️retained-clone/🦀️.rs:368,391`, `🌆️bin-02/🔬️unit/🦀️.rs:757`.
`try_admit_one_returned` (closure + result now `RetainedWorkOutcome`): store `👥️presence/♻️retirement/🦀️.rs:17`, `⚙️engine/🧱️store-components/.../🏗️store-owners-retained-reads-and-retirements/🦀️.rs:293`, io serialization owned tests, bin-01 tests.
`artifact_retirement_box_close_step`/`advance_returned_snapshot_read(_root)` callers now get `RetainedWorkOutcome`.

## Callers ported (12:20)
- `store/🧬️snapshot-clone`: staged `source_admission` field driven in the Source phase; close ladder returns inputs / publishes output; demands + terminal updated.
- `store/📬️paged-one-item` (policy lane file, untouched >2 h): same staging; `artifact_retirement_admit_owned(..).result`.
- `inference/🎟️source`: `Staging<P>` (Owned via `admission_borrowed`, StoreRead via `source_admission`) driven through `admit_original_construction`/`close_original_retained_step`; the rest of that file (`ControlledRetirement::step` / `Sealed::close_step` fed to `close_original_retained_step`, `project_owned`) is still the job-API port owed by the job lane.
- draw `📋️clipboard/🧵️job`: `SealStaging` RetireOwned wrapper (cursor returns the read or hands the source to its own retirement) because `Workspace` derives RetireOwned.
- `store/👥️presence/♻️retirement`: `advance_returned_local` delegates to `advance_returned_snapshot_read_root(..).result`; box-close callers use `.result` (also `🦀️.rs`, `🚫️rejection`). Presence is otherwise still Result-based (runtime lane).
- engine store-owners: `take_returned_snapshot_read_retirement` wraps the admission in `RetainedWorkOutcome`, keeps its Result API.
- tests: store `🧬️retained-clone` (`seal_read` helper), `🌆️bin-02`, io-serialization owned, bin-01 x2.

## Still calling the removed API (outside the coordinator list, not ported)
`admit_retained_clone_source` in: `🔌️plugin/📜️declaration/🧩️app/🛬️lifted/🎬️media-export/📖️read/🧪️tests.rs:3`, `🔌️plugin/📜️declaration/🪆️child/🎯️editor/📖️read/🔗️custody/🦀️.rs:25`, and layout editor `📤️export-tool-job/🧵️retained/🦀️.rs:31`, `🎥️media-export-job-and-factories/🦀️.rs:81`, `🎮️commands/🧵️patch-page/🖼️drawing/📖️source/🦀️.rs:52`, `⚙️engine/📤️export/🧪️tests/{🗺️planning,🔬️unit/✒️pdf-drawing-laws}/🦀️.rs`.
Also stale TS source-grep tests mention the old names: store `🧩️composition/📨️emission/📦️owned/🧪️tests/🟦️.ts`, repo laws `🎯️tool-job-coverage/🗄️artifact-store/🖼️caller-retained-media/🟦️.ts`.

## 12:40 remaining callers ported
- New reusable `SnapshotReadSourceStaging<T>` (`snapshot-read/🪜️staging/🦀️.rs`, re-exported as `store::os_store::snapshot_read::SnapshotReadSourceStaging`): RetireOwned holder of the staged admission; `begin(read)`, `next_demand()`, `step(&mut Option<source>, grant)`, `close_step(...)`. The draw clipboard now uses it instead of its private wrapper.
- Layout editor: `📤️export-tool-job/🧵️retained` (ExportOwners.sealing), `🎥️media-export-job-and-factories` (MediaSources.sealing), `🎮️commands/🧵️patch-page/🖼️drawing/📖️source` (Owners.sealing, phase 1); child editor `📖️read/🔗️custody` (ChildReadSourceCustody.sealing, duplicate-guard keyed on empty staging).
- Tests: layout export `🗺️planning`, `✒️pdf-drawing-laws`, plugin `🎬️media-export/📖️read/🧪️tests.rs` use the staged admission inline.
- TS source-grep: `caller-retained-media/🟦️.ts` now expects `base.source_admission()`; `emission/📦️owned/🧪️tests/🟦️.ts` still holds (engine ingress keeps `try_admit_one_returned::<P, _>(grant`, no `for _ in`/`Box::new`).
- inference/🎟️source: controlled owners report work via `controlled_turn` (`last_turn_work`), `Sealed::close_step` returns `RetainedWorkOutcome`, `Sealed::project` returns the `project_owned` outcome, close closures wrapped with `bounded_outcome`. Not yet confirmed by the job lane (pjob result file absent).
- No remaining `admit_retained_clone_source` callers.
Open: batched check of snapshot-clone, one-item-batch (paged-one-item), inference, presence, engine-core, layout/draw/plugin crates once upstream is green; then snapshot-read lib tests (blocked by semio-framework-async).
