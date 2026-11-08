STATUS: GREEN for the lib check of semio-os-history-record-decode and semio-os-store-command: wasm32-wasip2 W-store2-8 rc 0, native W-store2-9 rc 0 (W-store2-6/7 for history alone). Test targets and downstream callers UNVERIFIED (not compiled).

Logs: C/🗑️generated/1105b/W-store2-1..7.txt/.rc. Tools: C/1105b-tools/store2/{effectify.py,wrap.py}, C/1105b-tools/store2-errs.py.

## history-record-decode (62 -> 0 errors)
- Record decoders (metadata, transition, edit, identity-records, conflict, origin + origin json): `step`/`close_step` take `&mut RetainedAllocationEffect`; children get `effect.nested(child)`; wrapper `step`/`close_step` over `*_body` records the claimed work once via `crate::settle_since` (delta of the turn's effect, no second charge). Types without work counters (operation, identity, message, records, payload) stay pure and are paid by their parent's claim.
- `last_turn_work` is u128; ledger identities are `NativeAllocationIdentity`; owned retirement via `admit_owned_retirement(&mut slot, effect)` and `close_step(effect)` poll (Pending => Blocked, failure mapped by `OriginalRetirementFailure::into_value_error`); SharedUtf8::receive_original takes the nested effect; paged records reserve through `reserve_one_funded(effect)` and `append_original(original, effect)`.
- No wallets. Untouched: companion tests crate (`semio-os-history-record-decode-tests`) and the `file` decoder callers still call the old grant forms (not compiled in this proof).

## store-command (48 -> 0 errors)
- `ArtifactCommand` / `SupersedeInput` now `#[derive(RetireOwned)]` (schema); hand seed impls removed from the retirement module.
- Cursor (`receiving/cursor`): every advance action is one `act` = `native.with_allocated_effect(demand, 1, ..)` (allocating actions bind the marker via `effect.actual()`), no `prepare_work`/`commit_work` around it (ruling runtime2: fund exactly once); `receive_retained_work` and `physical`/`own` removed; storage/SharedUtf8/vec backing use effect forms (`allocate_empty_vec`, `reserve_empty`, `put_from`, `take_box_into`, `release`). Nested operation cursors (`OpBinaryReceivingCursor`, still grant-shaped) keep their own funding; the grant-shaped `close_step(grant)` bridge pays its reported work with `RetirementPollExit::pay`.
- Close: `close_step(effect) -> OriginalRetirementPoll` ladder with `RetirementPollExit::settle`; `close_on_original` runs it through `with_retirement(..).receive_effect` with a resident `NativeReceivingEffect` frame; envelope wrapper forwards.
- Unwraps in flag/append/finish replaced by typed refusals. Public signatures kept (admit, step, take_into, receive_step, close_on_original); `close_step` now takes an effect (callers: plugin job reserved checkpoint, owner `callers`).


## Round 2 (coordinator: store-backbone, store-canonical-json, store-child, store-space-schema, action-bus; statik-224)
- store-space-schema: `SpaceHistoryMutation` derives `RetireOwned` (hand impl removed). store-canonical-json: reader `close_step(effect)` ladder with a local `retire_snapshot` (SnapshotRetirementFactory through FactoryProducerReply). store-backbone: retirement `close_step(effect)` via `RetirementPollExit::settle`; original outbox `reserve_step/transfer_step/close_delivered_step/recycle_step/close_step` take the effect (paged-list reserve/release outcomes handled typed).
- Proof: wasm W-store2-13 rc 0, native W-store2-14 rc 0 (manifest 🧰️framework/Cargo.toml).
- store-child: `ArtifactChild` retirement via its (child_id, target) tuple cursor; `ArtifactChildNativeReceiver` delegates the full NativeReceiver protocol; the retained-clone cursor is effect-based (advance/take with reply/begin_close_paid/close_step ladder, `settle_outcome` pays one unit for a progressing turn without recorded work).
- action-bus: tuple/unit/Vec retirement seeds, registered source cursor on effect, `admit_original_tool_construction{,_scoped}`/`admit_original_tool_job*` and `OriginalWireAdmission::{admit,advance}` take `&mut OriginalConstructionAdmission` (resident across turns) and return `OriginalConstructionStatus`; wire close uses `close_original_effect_step`.
- Proof: wasm W-store2-19 rc 0 with manifest ✏️s/Cargo.toml (store-child is in the framework workspace exclude list, so `-p` fails there: "did not match any packages"). Native W-store2-20: see log.
- Downstream breakage (not compiled): plugin retained-command producer/factory and reserved-jobs call sites of admit_original_tool_*; tests of canonical reader, outbox, action-bus admission.
