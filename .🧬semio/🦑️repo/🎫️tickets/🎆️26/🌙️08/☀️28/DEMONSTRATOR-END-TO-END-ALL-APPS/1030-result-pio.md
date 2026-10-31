STATUS: DONE. wasm --lib GREEN: io-control, io-sqlite-model, canvas-scene, artifact-reference, job, os-store-link-schema. Native lib tests GREEN (pio-t6 rc=0): io-control 31 passed, io-sqlite-model 0 tests.

# 1030 pio result

| Crate | Fix | wasm32 --lib |
|---|---|---|
| `semio-framework-io-control` | `🚪️io/⏱️control/🛫️snapshot/🦀️.rs`: `NativeSnapshotReceiving::close_step` now returns `RetainedWorkOutcome` (closure computes the `Result`, `work` summed from `close_factory_ticket(..).work` and `ControlledRetirement::last_turn_work()` of the stepped child); original-drive loop uses `close_factory_ticket(..).result`; imported `RetainedWorkOutcome`. | green |
| `semio-framework-io-sqlite-model` | `🚪️io/🪶️sqlite-snapshot/🏛️model/🦀️.rs` `close_original_recipient`: `match recipient.close_step(grant).result{..}`. | green |

Check: `C/1030-tools/pio/wasm.sh 2 --lib -p semio-framework-io-control -p semio-framework-io-sqlite-model` (rc=0). First attempt (pio-w1) was blocked by the peer's in-flight retained-clone edits (original-slot, retained-map, snapshot-projection); re-run after they quieted was green.

Tests: `C/1030-tools/pio/test.sh 1 --lib -p ...` fails before reaching my crates: `semio-framework-dsl-record-model` (lib) 17 errors (`NativeReceivingOriginal` lost `shell_demand`/`begin_shell`/`admit_original`/`begin_receiver`, `close_step` signature, `RetainedWorkOutcome` used as Result/iterator, `RetainedCloneSource::admit_owned`). Foreign; needs a dsl-record-model port lane. Rerun test.sh once it compiles.

## Addendum: canvas-scene + test rerun
- `semio-framework-canvas-scene`: no edit by me; the stroke retirement (`🖼️canvas/♻️retirement/🖊️stroke/🦀️.rs`) already carried `close_step -> RetainedWorkOutcome` when I looked. wasm --lib check pio-w3 rc=0, canvas-scene compiled (GREEN).
- io lib tests rerun (pio-t2, rc=1): dsl-record-model no longer blocks; native test build now stops in `semio-framework-artifact-reference` (lib, 4 errors, foreign). io-control/io-sqlite-model tests still unrun; rerun `C/1030-tools/pio/test.sh <n> --lib -p semio-framework-io-control -p semio-framework-io-sqlite-model` once it compiles.

## Addendum 2
- `semio-framework-artifact-reference`: `#[value(..., equality="::semio_framework_value_persistent")]` on `ArtifactDialect` and `ArtifactRef` (derive RetainedEquality default crate `semio_framework_value` has no `retained_equality`). pio-w4 green.
- `semio-framework-job`: `📮️return/📦️bytes/🦀️.rs` `publish`: removed the undefined `metadata(..)` call; now `registry_control_ready(cx)` guard before the move and `registry_control(cx,false)?` after publication (mirrors the peer's page-reservation port). wasm green (pio-w6 no job errors).
- `semio-framework-os-store-link-schema` E0282: root cause was in the NativeReceive struct derive for zero-field structs (LinkPin's unit variant generates an empty payload struct; repro: `struct E{}` with derive failed): `let flatten=match field{.. _=>return Err}` had no type. Fixed in `🌱️value/✨️derive/🛬️receive/🦀️.rs` with `let flatten:bool=`. pui had edited the same file shortly before (derive owner). pio-w9 green.
- io-control tests ported: `.result` on `close_step`/`close_retirement_recipient`/`admit_owned_retirement`/`close_factory_ticket` in `🧪️tests/{🦀️.rs,nested-io.rs}`, `🛫️snapshot/🧪️tests/🦀️.rs`; `RetirementStep::ChildProgress(c,p,_)`/`Complete(_)`; zero-grant recipient close now tolerates the peer's WorkLimit refusal (progress from `retained_progress()`). Product fix: `IoRunControl::select` treated an EMPTY pending slot as non-empty (`pending.is_some()` on `Option<&mut Option<..>>`); now `pending.as_deref().is_some_and(Option::is_some)` (fixed 3 nested-io tests).

## Addendum 3: semio-framework-os-vcs (wasm --lib GREEN, pio-w12 rc=0; was 33 errors in pio-w10)
- `🚪️io/💾️binary/🪪️checkpoint-identity/🎟️original/{facts,commit-facts,repin-facts,repin-alternative,pending}.rs`: inherent `close_step` keeps its `Result` API; `.step/.begin_granted/.step_granted` outcomes read via `.result` (work not surfaced here).
- `🗃️history-ledger/♻️retirement/🦀️.rs`: `LedgerRetirement::step` takes `&mut u64` work, new `tally` helper folds `RetainedWorkOutcome.work` (close_factory_ticket, step_granted, authority.step, begin_granted, admit_original); cursor `close_step` returns `RetirementStep::{Complete,Progress,Failure}(.., work)`.
- `🔗️read/🦀️.rs`: `.result` on `read.advance`, `retain_original`, `return_original`.
- `♻️retirement/🦀️.rs` (peer was mid-port): last two sites `Failure(.., 0)` and `Complete(1)`.
- Lib tests of os-vcs not run (not asked); the inherent `close_step` tests in checkpoint-identity may still need `.result` ports.
