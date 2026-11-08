STATUS: PARTIAL (11-07 08:15 update below). `semio-framework-tool-run-entities` lib GREEN native (port-d-7 rc 0) and wasm32-wasip2 (port-d-9 rc 0). The rest of the tool-run port is EDITED-UNVERIFIED: the tool-run lib cannot be type-checked until upstream reds clear (port-d-6.txt).

# port-d: tool-run crates (took over ledger's claim)

## Proven
- `tool-run-entities` (24 errors -> 0): `State<K>` and `Editor<K>` derive `RetireOwned` (State is `pub` + `doc(hidden)` because the derived cursor type leaks it); `ToolRunEntityEdit` / `...VersionEdit` / `...ProvenanceEdit` use `type RetirementCursor=<State as RetireOwned>::RetirementCursor` + `retirement_seed` (sets `transferred`, takes the state).
- New root helpers: `record_work(effect, u128)`, `publish_sealed(effect, value)`, `admit_sealed(value, effect)`, `duplicate_sealed(&sealed, effect)`, `reserve_vec`. Quoted work is recorded before the sealed primitive runs (a refused quote hands the value back); only surplus beyond the quote is best effort.
- `ToolRunEntityVersion::admit/capture/take_output`, `ToolRunEntityProvenance::admit/admit_empty/capture/take_output` take the effect (grant = `effect.grant()`, marker = `effect.actual()`).

## Edited, unverified (no compiler verdict)
- tool-run crate root: `reserve_vec`, `settle_close`, `record_work`, `admit_sealed` (pub(crate)); `progress_clone` (advance(effect), allocation via reserve_vec, State pub doc(hidden), seed); `step_ring_insert` (advance(grant, effect), growth via reserve_vec + VecDeque from Vec, close via settle_close); `payload_stream::update` (advance/close_step(effect), nested effects `effect.nested(child(g))`, `Close` cursor on the new `RetirementCursor::close_step(effect)`).
- presentation: root `admit/capture/capture_view(effect)`; `seed.advance(effect)`; `update` (capture, provenance capture/take_output, tool_id/payload duplicate_sealed, admit with effect, cursor type `Box<State>`); `slot` (capture, capture_view, begin_tick take effect).
- plugin: tick native `capture` and `begin_tick` go through `cx.with_original_effect` (tick restored when the quote is not admitted; one debit `1.max(performed)`), `advance_presentation` keeps ledger's single debit `work.max(performed)`; driver `capture_original_view_for` likewise.

## Not done
- tool-run lib compile: blocked upstream (port-d-6.txt): `semio-framework-async` wake/thread (aggregator), `replication-retained-field/key` (bearbeiten), `pack-json` (aggregator/valuefix), `io-control` (78 errors, UNOWNED; koordinator released it, needs a dedicated slice).
- tick-wire (`tick/receive`: entities column receiving, native cursor close_step) and plugin `OriginalToolRunNativeState` (`RetireOwned::retirement`, `RetirementCursor::close_step(grant)`) still on the old API; needs the job-side `close_original_retained_effect_step`.
- Tests of entities, presentation, payload stream, tool-run unit still use the grant API (fixture effect needed).
- `[DEBUG] ledger-lease-drop` at `plugin/♻️cancellation/🦀️.rs:285` left in place: no aggregator log shows its backtrace yet.

## Logs
`🗑️generated/1105b/port-d-{1..9}.txt`; port-d-7 native rc 0, port-d-9 wasm rc 0.

## Update (coordinator: continue)
- `async` wake/thread: already FIXED by aggregator (aggregator-146); not mine.
- tool-run lib check `port-d-10.txt` (rc 1): only claimed crates red: pack-json (aggregator/valuefix), io-control (port-f), replication-retained-field (port-c). Nothing of mine is red.
- Plugin `OriginalToolRunNativeState`: `RetireOwned::retirement_seed`, `RetirementCursor::close_step(effect)`, new `close_with(cx, demand)` (job-side `cx.with_original_effect`, one debit `1.max(performed)`), `close_effect_step`; callers in `driver-and-ledger` and `retirement/entry` now use `close_with`. EDITED-UNVERIFIED.
- tick-wire (`tick/receive`) BLOCKED on a design decision, sent to runtime: `receive_construction_step` gives a grant only, but the entity column's `reserve_capacity_one_funded` / `adopt_reserved` need a `RetainedAllocationEffect`. Option A: move tick cursor + `ToolRunEntityColumnReceiving` onto the NativeReceiver protocol. Option B: add `receive_construction_effect_step(demand, |control, effect|)` to the construction control. I lean B. Not ported until answered.

## tick-wire decision (runtime: Option A) and why it is NOT started
runtime ruled A: move `ToolRunTickNativeCursor` + `ToolRunEntityColumnReceiving` onto the NativeReceiver protocol (select_advance / advance_nested / advance_local(effect, reply) / take(output, effect, reply) / close_step(effect)).
Sizing (not a single-lane blind edit): `tick/receive` is ~110 KB of dense source (root 25 KB, progress 27 KB, native 16 KB, trace 15 KB, entities 9 KB, step 9 KB, identity 6 KB). The cursor is a pack record parser state machine (`RetainedRecordBodyCursor` grant-based `admit_record_storage` / `retire_record_storage`, `grant()`, `admit_byte`), driven from the plugin by `cx.with_native_receiving(|native| cursor.advance(source, native))`, with the wire source a `&dyn OriginalTickWireSource`, not a `RetainedCloneRef<I>`. The protocol's `NativeReceiver<I,T>` assumes a retained original `I`, so the source/slot shape has to be designed before code (a NativeReceiver over the wire pages, or a producer-side cursor like capsplit's NativeLocalProducer). The parser APIs it sits on (`pack` record) and `pack-json` are still red and claimed, so nothing can be compiled.
Proposed slicing for a dedicated lane, each step checkable with `-p semio-framework-tool-run-tick-wire --lib` once pack is green:
1. entities column (`ToolRunEntityColumnReceiving`): phases Field/Tag/Length/Bytes = Local{work:1} pure; reserve = Local with `reserve_capacity_one_funded(entities, effect)`; Commit = Local with `adopt_reserved(&mut original, effect)`; close = Local pure.
2. identity, step, progress receivers (pure Local phases).
3. trace cursor (nested child).
4. root cursor: parser storage admit/retire as Local phases with the effect, token dispatch as Local, trace/entities as Nested.
5. plugin tick native `receive_with_domain` phases 0..2 swap `with_native_receiving` for the slot-owned effect.
Needs from runtime/owner of the slot: how a producer-side cursor (no retained original) is hosted (answer pending in the slot design).

## Hosting shape decided (runtime): (2) producer-side cursor
The tick cursor keeps ONE resident `NativeLocalAuthority<F>` (`OriginalEffectAuthority<NativeLocalProducer<F>>`). Pure phases run through `native_receive_local(native, work, || RetainedWorkOutcome..)` (protocol `local.rs`). Effect-needing phases (`reserve_capacity_one_funded`, `adopt_reserved`, parser record steps) run through `poll_local(native, demand, grant, work, maximum, authority, |_, effect, authority, action| authority.drive(&mut NativeLocalOperation::Poll(..), effect, action))`. Reference: capsplit shared-field `decode/construction/field/` (value_action / retire_action tags, authority retired on close). Close is driven from the parent's `close_step(effect)`; the authority is retired last. One phase = one action, quote = ceiling, reported work == recorded work. The 5-step slicing above stands. Waits on pack-json decode (aggregator). Needs a dedicated lane: NOT started by port-d.

## job-failure (coordinator exclusive) GREEN
`semio-framework-job-failure` lib: native `port-d-16` rc 0, wasm32-wasip2 `port-d-17` rc 0.
- frame: `OriginalReturnFrame::close_step(effect)->OriginalRetirementPoll` (patch continuation-close-port.md item 1); callers `🎟️origin` and `📦️capsule` use `close_original_effect_step`.
- domain: `OriginalDomainReservation::close_step(effect)->OriginalRetirementPoll`; reservation cursor maps Pending/Complete; `DomainReservationRetirement` is `pub` + `doc(hidden)` (derived cursor type leak); the other seed/cursor impls were already migrated.
- snapshot projection: `refusal()` projects `NativeAllocationRefusal` (retained handles project Null); allocation failure array rebuilt per element type; uses new additive `NativeSnapshotFailure::original_fields()` / `NativeSnapshotFailureFields` in `io/⏱️failure-owner`.
- Not done: job-failure tests (grant API).

## io-schema, replication-edit, replication-history, replication-conflict GREEN (lib)
- io-schema: `IoOutcome<T>` / `IoError` derive `RetireOwned`; the manual `♻️retirement` module is removed. replication-edit was ported by port-c and verified here together with io-schema: native `port-d-20` rc 0, wasm32-wasip2 `port-d-21` rc 0.
- replication-conflict: protocol owner closes on the effect (`admitted()` maps a poll through `admit_retained_clone_close`), `ConflictId`/`ConflictKind` seeds.
- replication-history: transition enums and `HistoryInputDrafts` derive `RetireOwned`; fold primitives that need an effect run through a standalone `AuthoringWallet` effect (`authored()` + `settled()` in the fold root) funded by the fold's own grant, `HistoryFoldJob`'s `ErasedSnapshotRetirement::close_step(effect)` delegates to `close_step_granted(grant)`. Debt: the fold job still reports its own work (`last_work`), so the surrounding effect records 0 performed for a fold close; `record_constructed_allocation` is gone (the construction step already submits progress) so the history native-binding no longer records an extra allocation. Proof: native `port-d-25` rc 0, wasm32-wasip2 `port-d-26` rc 0. Tests of these crates are not ported.

## runtime2 ruling on the fold (open, NOT done)
runtime2 wants `authored()` deleted: fold primitives on the turn's effect (`effect.nested(child_grant)`), `close_step(effect)` as the real body (no `close_step_granted`/`last_work`), advance entered with an effect from its plugin caller, and a test that the history native-binding's retained progress total equals the fold's step progress sum. Blocker: the primitives run inside futures polled with only a Context (control.retire, SharedActorLease, FoldBackingBirth, FoldPagedByte), so the control needs a step-scoped effect slot (design confirmation asked from runtime2). Needs a dedicated slice; current green state is port-d-25 native / port-d-26 wasm.
- runtime2 confirmed `LentEffectSlot` (lend in `close_step(effect)` and the native entry, `slot.with_effect` in the poll primitives). Still queued, not started: it needs three layers together (fold crate; `step_original`/`close_original` onto `native.receive_effect` + resident `NativeReceivingEffect` frame; plugin typed-pack-load/native caller lines 79-81 and ~25 companion test sites using `step(grant)`/`last_turn_work`). Asked runtime2/coordinator for the plugin-side owner or one dedicated slice.

## Fold on LentEffectSlot (runtime2 ruling) - DONE in the fold crate, plugin unverified
- Fold crate (`replication-history`, `replication-conflict`): native `port-d-30` rc 0, wasm32-wasip2 `port-d-31` rc 0. `HistoryFoldControl` holds a `LentEffectSlot`; `charged()` (run a primitive on the lent effect, charge any reported work beyond the receipt) and `owe()` replace `authored()` in `control.retire`, `close_one`, `SharedActorLease`, `HistoryFoldAdmission`, `FoldBackingBirth`, `FoldPagedByte`, `cleanup_one`. `close_step(effect)` = `close_lent` is the real body (work = `effect.performed_work()`); `step_effect(effect)`/`lent()` lend for exactly one call (`lent` flag keeps the Arc quote exact). `close_step_granted` and the internal `last_work` accounting are gone (the field remains only as the test-visible tally set from the effect at entry exit).
- Native entry: `step_original` lends via `NativeDecodeControl::with_effect(step_grant, FOLD_TURN_WORK, ..)` inside the construction step (construction work reported 0, the effect carries it); `close_original(native, effect) -> OriginalRetirementPoll`. I used `with_effect` instead of `receive_effect` + a resident `NativeReceivingEffect` frame because the job already runs inside the construction step and needs no second frame.
- Plugin `typed-pack-load/native`: close stage 8 and the drop path now run `close_original_effect_step(cx, d, |effect| job.close_step(effect))` (the native-loan identity check of `close_original` is not applied there). EDITED-UNVERIFIED (rustfmt parse only).
- Grant-wallet entries kept for cold callers and tests only: `authored()`, `step(grant)`, `close_cold(grant)` (`finish_cold` is a cold caller outside any job turn).
- Tests: `fold_work` target compiles after porting the `Validation` fixture and `close_cold` call sites (13 sites). Still red (pre-existing drift, not from this slice): drafts `cursor.step(grant)` (ControlledRetirement::step now effect-based) and the manual-protocol `admit_owned_retirement(value, grant)` check.
- NOT done: the single-post test (history native-binding retained progress == fold step progress sum); it needs a native decode harness. Open question for runtime2: because step_original reports construction work 0 and the effect carries the work, confirm the decoder work authority is not also debited by `reconcile` in the plugin (`native_step`).

## runtime2 rulings applied (charge + lineage)
- `step_original` now returns `work` = the lent effect's performed work (`last_work`, 0 when the quote is not admitted); no constructor-side constant.
- `validate_original(native)` is `pub` and runs again before every effect close: typed-pack-load/native `close_fold(cx, job, d)` = `cx.with_native_retirement(|native| job.validate_original(native))??` then `close_original_effect_step(cx, d, |effect| job.close_step(effect))` (stage 8 and the drop path). No other `native_binding` users exist outside the fold crate.
- Proof: fold crate `port-d-34` native rc 0; plugin rustfmt parse only.
- NOT written: the two laws (different lineage refused with 0 charge and the job left resident; native_step work == sum of effect performed); both need a native decode harness.
