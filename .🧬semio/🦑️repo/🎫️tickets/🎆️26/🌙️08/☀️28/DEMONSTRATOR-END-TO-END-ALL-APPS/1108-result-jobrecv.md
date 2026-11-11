# Lane jobrecv result (semio-plugin-job-receiving, 77 errors in W-declviews-hub-1)

## State
- Ported all four modules (steps, pack, copies, transaction) to the 733 API in one pass; first wasm hub compile W-jobrecv-1 queued (cargo capacity queue).
- Design: internal metadata moves are direct funded moves (`moved`/`paid`/`forgotten`, no construction admission); only the born Box<owner> uses `admit_original_construction` with the caller's `OriginalConstructionAdmission` + `OriginalConstructionFailure` slot (typing-edit recipe). Parser, admission, source, receiver, cursor run as original effects (`with_original_effect` / `close_original_effect_step`), retirement via `RetainedCloneClose::begin_granted/step_granted`.
- Receiver advance goes through `with_native_receiving_quoted` (quote = receiver.demands(body, fuel), 1 item); its Source outcome and any `NativeReceivingFailure` stay retained in the owner and are retired on close.
- Intrinsic parser is created in phase 0 via an effect (`RetainedIntrinsicBody::new` now takes an effect); bytes move into the owner at admit.

## Interface changes (callers outside this crate)
- `OriginalPackReceiving::{admit,admit_value}`, `OriginalIntrinsicReceiving::admit`, `OriginalValueCopies::admit`, `OriginalTransactionPrepareSource::admit`: new params `admission:&mut OriginalConstructionAdmission, failure:&mut Option<OriginalConstructionFailure>` before `output`; `admit_value` also takes `limits`. Return `Ok(true)` only when born, settled and reset; call again until true.
- New `admission_demands()` on each owner (frame quote).
- `original_steps` now exports `fixed, receipt, funds, funds_retirement, paid, moved, forgotten, settle, begin, retired, forward, replied`; old `built/built_with/retired(grant)` are gone (callers: plugin `job` crate `original_steps` re-export and `child-vcs`).
- Owners close the receiver through the cursor protocol (`begin_granted`), so `begin_close()` on the receiver no longer exists.

## Debts
- Tests (`🧪️tests/🦀️.rs` in pack/copies/transaction) still use the old shapes.
- Receiver forward quote uses the whole remaining fuel as work claim (receiver publishes no advance work quote).
- `Pending` from the receiver keeps state but no recovery path is wired.

## Coordinator round 2 (pending recovery, exact quote, callers)
- (2) DONE (unverified): `NativeReceivingOriginal::advance_work_demand(body, ceiling)` added in value/🛬️decode/🧳️owned/🧳️original/🦀️.rs (reserve/seed/take/local selection exact; a Nested child action is quoted by the caller ceiling because the child owns its selection). Pack phase 6 quotes `(demands, advance_work_demand)` instead of the whole fuel.
- (1) DONE (unverified): a Pending receiver action freezes its ceiling (`pending_ceiling`); the next turn re-enters `advance` with the same (body, ceiling) and the receiver's frozen action demand/work, which is the receiver's own `recover_*` route. Debt: a cancel while a forward action is Pending cannot resume through the retirement control (it exposes no NativeDecodeControl), so close fails loudly until the action settles.
- (3) caller ports, unverified (not compiled yet):
  - steps: `born`, `acted`, `produced`, `moved`, `forgotten`, `begin`, `settle`, `forward`, `replied`, `retired`, `construction`.
  - child-vcs `ChildVcsCommandUnit`, `ChildCheckpointUnit`, `ChildCheckpointCascade`, reserved `OriginalCheckpointRoute`: admit takes admission+failure, internal frames `frame`/`failure`, effect-shaped closes, reply-shaped cursor advance/take, `take_authority_into`.
  - job outcomes (`CleanupOutcomes::close_step(effect)`), job lib re-export.
  - runtime custody manifest: parser created in phase 0 via `original_pack_receiving::start`, advanced via `parse`, closed via `close_parser`; held `bytes` retired.
- also ported (unverified): job `📨️emission/🎟️capture` (bind via `born`, `produced`/`acted`, capture `admit`/`handoff_into` direct), job `🏛️reserved/🦀️.rs` + `🛬️lifted/🔄️full-operation` + `🧾️bytes` closes (effect-shaped wrappers), job lib `plugin_typed_owner_original` sites (effect), runtime custody `manifest/🦀️.rs` (frame/failure fields, `take_media_in` via `born`, ControlledRetirement.step(body,effect), forgotten) and `metadata.rs` (born admit, deferred parser, quoted receiver, effect closes).
- NOT yet ported (need compile feedback; stale on their own APIs, not only this crate): runtime custody `window-view.rs`; `⚛️reactor/🏃️runtime/🧰️ops/🦀️.rs` and `🪟️surfaces/native.rs` (private `built_with` copies on the old admit_original_construction); `🧾️typed-operations/{intent,response}.rs` (import this crate's built/built_with; also depend on `ui_contract::OriginalUiArguments::admit`); job `🦀️.rs:736 lease close_step(canonical,grant)` and reserved `774`.

## Handover (coordinator split, 07:30)
- Verified: job-receiving lib wasm W-jobrecv-3, native W-jobrecv-6; child-vcs (unit, checkpoint, cascade) wasm W-jobrecv-5 (0 errors in that crate).
- Released to lane jobcrate: all semio-plugin-job files listed in the claims log (07:30 entry). Their remaining errors are in W-jobrecv-5.txt (capture 127/128, reserved 173/458/781, full-operation 105, lib 736 plus registration RetireOwned drift).
- Mine, unverified (blocked behind semio-plugin-job): runtime custody `manifest/🦀️.rs`, `metadata.rs` (edited); `window-view.rs`, `ops/🦀️.rs`, `surfaces/native.rs`, `typed-operations/{intent,response}.rs` (not started). Recipe: `born` for Box births with caller admission+failure, `produced`/`acted`/`moved` for funded moves, `forgotten` for shells, `begin`+`retired` for retirement, `forward`/`replied` for effect cursors.
- Hub map is refused by a peer crate (norm-en1991-inferences unbalanced delimiter); use per-package `chk.sh W-jobrecv-N 🌎️hub/Cargo.toml wasm -p <crate>`.
- Tests in pack/copies/transaction `🧪️tests` still use old shapes.

STATUS: FIXED-verified W-jobrecv-3 (wasm), W-jobrecv-6 (native) job-receiving; child-vcs W-jobrecv-5; runtime custody callers open behind semio-plugin-job
