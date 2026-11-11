# Lane femrecv: fem 2d/3d receive stops at the source-admission projection

## Done
- Shared `semio-s-fem-receiving::reply`: `admission_demand` / `admission_advance` (identity projection: `owned_projection_demand` + `project_owned` while `is_projecting()`, else `next_demand` + `advance`). 2d and 3d `snapshot/native.rs` `demands` phase 0 and `advance` phase 0 use them (no duplicated code). The "original projection must be driven or returned through its resident frame" refusal is gone; the 3d receive now gets 261 items (was 75).
- Child ceiling derived from the exact next-action quote (coordinator ruling; the hand-picked `1<<16` constant was removed again, `ACTION_WORK` no longer feeds child receivers). New API `advance_work_demand(input, body)`:
  - `NativeReceiver` trait (protocol crate): provided method, Local -> work, Nested -> refusal (override required).
  - Overrides: `NativeReceiveSlot` (capture/bind/reserve/seed/take/child), `NativeSequenceReceiver`, `NativePersistentListReceiver` (RowAdvance -> row child), `NativeSharedReceiver` (Receive -> child, Seal -> `R::work_demand`), derive `NativeReceiving` record (action 14 -> field slot / default slot), `NativeDefaultSlot::advance_work_demand(body)`.
  - fem 2d/3d: `maximum = rx.advance_work_demand(source, body)?` for both `demands` and `advance` of the node/material receivers.
- No `[DEBUG]` left in my files (grep clean; one foreign hit in `construction/effect/tests/retained-outcome.rs`).

## Verified
- wasm32-wasip2 fem 3d/2d model + receiving: W-femrecv-w1 rc0.
- native 3d model compiles (W-femrecv-t12 ran).

## Open (BLOCKED)
`original_job_receives_and_retires_fem_rows` / `..._typed_record_into_caller_slot` still fail with "snapshot cursor original grant cannot admit its frontier" after 261 items (W-femrecv-t12/t16/t17, 3d). Diagnosis with temporary prints: the stall is NOT the work ceiling (same stall with ceiling 4, exact quotes 1/3). It is the record schema matcher (`record/🔎️schema/🎛️drive.rs` `advance_nested`): selection `KeyAcknowledgement`, grant admits the 838 B demand, `key_advance_acknowledgement(control)` returns `Ready{work:0}`, so `idle(0)` = zero progress and zero work, which the snapshot cursor loop refuses. Owner: the dslrecv/record-matcher lane. Fix direction: the acknowledgement step must report progress or a paid unit (quote said 1, actual 0), or the cursor loop must treat an acknowledgement as progress.
- fem `native-tests` / 3d `roundtrip-tests.rs`: not compiled by me (snapshot-tests crates need `semio-plugin`; not attempted after the stall).
- 2d native test W-femrecv-t18 (`--test` not run): refused before the compiler, "Rust crate Source extent or boundary exceeded" reason `files`, remainingFiles 0 at `2d/…/🔺️diff/🏋️loads/🦀️.rs` for `semio-s-artifact-fem-2d-model`. Not caused by my edits (none add files to 2d); the 2d crate is over its source file count and needs a split or a move of tests to `[[test]]` crates (no limit raised).

## Rerun
`"$C/1105b-tools/runtime/tst.sh" W-femrecv-<n> "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml" native -p semio-s-artifact-fem-3d-model --test original_fem_receiving` (same with ◻️2d)

STATUS: BLOCKED projection port and exact-quote ceilings done (wasm W-femrecv-w1 rc0, 3d receive 75 -> 261 items); both receive tests stop in the record matcher KeyAcknowledgement returning Ready{work:0} (zero progress, zero work)
