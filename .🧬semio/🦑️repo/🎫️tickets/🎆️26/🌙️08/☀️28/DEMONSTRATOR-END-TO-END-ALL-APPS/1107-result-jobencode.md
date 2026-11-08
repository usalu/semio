STATUS: GREEN lib on wasm32-wasip2 (W-jobencode-1 rc 0) and native (W-jobencode-2 rc 0); logs in 🗑️generated/1105b/. Crate tests UNVERIFIED and still stale.

## Done
- job-native-encoding ported to the recorded-source value API: `NativeEncodeControl::new_recorded_source` over allocation, span and checkpoint loans, plus work and construction loans (nested `with_send_loan` / `with_loan`).
- Owner holds five generic `EncodingCustody<A>` (allocation, span, checkpoint, work, construction) behind one `EncodingAuthority` trait; admit/close ladders are one generic implementation (was three copy-pasted ones). Close order is the reverse of admit order.
- One `EncodingTurnContext` view replaces three context traits; per-call `EncodingHop` (planned backing, observed, write permits) replaces the owner Cells and the removed `pending` / `actual` / `observed` fields. Allocation policy (exact single backing, post observation, cancel on post) lives in `allocate()` over `NativeAllocationBudget`.
- `effect_allocated` closures are `|physical, effect|`: `effect.performed(1)` (false is WorkLimit), `*effect.actual()` for the marker.
- Removed dead `admit_source_issuer`.
- Value change (additive, claimed): `value/🛫️encode/🦀️.rs:3` now re-exports `OriginalRecordedAuthority/Producer/Reply/ReplyComplete`, `OriginalSpanRequest`, `OriginalCheckpointRequest` through `native_encoding`.

## Notes
- The witness for `new_recorded_source` comes from a raw pointer to the context's source turn, because the context is behind a `Mutex<&mut StepContext>` that the loan validation locks (a held borrow would deadlock). Documented here only; no comments in code.
- `OriginalSourceEncoding::{allocate_bytes,append_bytes}` is still never constructed (pre-existing), ported for compile only. Its append does not admit a span, so `settle()` provenance (copied <= permits) needs callers to go through `advance_source`.

## Not done
- `🧪️tests/🦀️.rs` still uses removed API (`birth_bytes`, `NativeEncodeAllocationIssuer`, `control.append_bytes` on a recorded source, `allocate_effect` flow). Needs a rewrite to `advance_source`/`effect_allocated`; assigned nowhere yet.

## Follow-up slices (coordinator)
- job-native-decode (job/⏱️context/🛬️decode): GREEN wasm W-jobencode-3 rc 0, native W-jobencode-5 rc 0. Allocation = `NativeAllocationAuthority<DecodeAllocationProducer>` (ledger moved into the producer, policy in `ActualWorkContext::allocate`) + `NativeForwardedDecodeFrame::begin` over its send loan; frame work u128. Recipient close = local `close_recipient_step` (`cx.with_original_effect` + `receive_original_retained_work`), because `close_original_effect_step` gives its closure no cx. Additive value change: decode root re-exports NativeAllocationAuthority/Port/Reply/ReplyComplete/Refusal.
- graph-manifest (receive): GREEN wasm W-jobencode-4, native W-jobencode-5. PropertyBagReceiver on the new NativeReceiver (select_advance Local{1} for root qualification, advance_local, take via resident `taken`, close delegates).
- tool-run: GREEN wasm W-jobencode-4, native W-jobencode-5. Helper paths `crate::component::*`; `ToolRunTickSource::{admit,operation,trace}` take the effect. Tests in tool-run/🧪️tests/🔬️unit still call the grant API (UNVERIFIED).

## Audit finding 1 (raw-pointer witness) fixed
`NativeEncodeControl::new_recorded_source` now takes the Copy `OriginalSourceTurnIdentity` (value encode root + 3 value test callers); job-native-encoding passes `original_source_turn_identity()` read before the mutex. No witness pointer remains. Green: wasm W-jobencode-6, native W-jobencode-7 (encoding + decode). Remaining raw `*mut StepContext` in `EncodingEffectReceipt` (settle on drop) is pre-existing and not part of the finding.
## Next red
semio-framework-tool-run-presentation: claimed, but W-jobencode-8 (wasm) is rc 0 with no edit of mine (peers fixed the duplicate impls). Not claimed by anyone: tool-run-tick-wire (tickwire's lane, 4 errors in main-148).

## EncodingEffectReceipt raw pointer removed
Safe shape: the nested loan call runs in `catch_unwind`; afterwards `Mutex::into_inner` returns the `&mut StepContext`, `settle_encoding(cx, output, &hop, start)` settles, and a caught panic stores the retained failure in the owner then `resume_unwind`s. No `*mut`/`*const` left (remaining `unsafe` is only the byte allocator). Green: wasm W-jobencode-9, native W-jobencode-10.

## job-worker (main-149, unclaimed) GREEN
wasm W-jobencode-11, native W-jobencode-12. `RetainedWorkerWake::close_step(effect)`; `close_session_retained_wake(inner, cx)` opens the effect via `cx.with_original_effect` (fuel stays debited by the existing progress-based `receive_original_worker_close_effects`, performed work is not debited a second time); `SessionHandle::admit_empty` uses a local actual marker (no effect in that path); erasure close annotates its outcome error type. Tests of wake/session unverified.
Other unclaimed in main-149 not taken: os-history-record-decode (62), os-inference (36), os-infinite-board-schema (19), os-store-command (48), os-store-cursor (3), os-store-retirement (8), s-fem-sparse (10).

## actor-retained-context, ui-contract-builder (callers-2)
- actor-retained-context: GREEN wasm W-jobencode-13 (rc 0). `JobPayloadAuthority::admit` local actual marker, `record_close_work(u128)`, cancel retirement demand wrapped in Ok, retained-slot close = `close_original_effect_step(cx, demand, |effect| owner.step(effect))` guarded by the quoted work vs fuel. Tests UNVERIFIED.
- ui-contract-builder: GREEN wasm W-jobencode-13; no edit of mine (peer `release_empty_page_leased` had landed).
- Native W-jobencode-14 rc 1 stops upstream in semio-framework-value-native-decoding (`🛂️allocation/🦀️.rs:25`, `.or(outcome.quote_refusal)` Option<NativeAllocationRefusal> vs Option<ValueError>); unclaimed by me, a peer's in-flight edit. Native verdict for my two crates is pending that.

## 12:1x job-recorded-clock / value-native-decoding
Both green with no edit of mine: wasm W-jobencode-15 (job-recorded-clock), native W-jobencode-16 (job-recorded-clock + value-native-decoding). runtime2's 12:00 edit of ⏱️recorded/🏛️source cleared the 10 errors; testdebt-f's allocation:25 fix cleared the other.

## os-dsl (32) / os-history (73) from main-151: NOT DONE, no source edit landed
Findings
- os-dsl `🫴️receiving` fails because its trait `OpBinaryReceivingCursor` (framework `📡️replication/🚪️io/💾️binary/🦀️.rs:22-40`) is grant-shaped while the children are not: `RetainedRecordReceiving::step(source,native,effect)`, `OwnedSlot::close_step(effect)`, ControlledRetirement. Its supertrait `RetireOwned` also needs `RetirementCursor`/`retirement_seed` instead of `retirement()`. The mirrored receiver (`🧭️mirrored`) forwards to the same trait.
- Other implementors of the trait (all in os, owned by store lanes): store `🧾️document/📜️history/💧️hydration`, `🎮️one-item/…/🧵️operation-wire/🧳️receiving/🔢️binary`, `📬️command/🛬️receiving/🪜️cursor`, mcp `🔬️probe-document/📥️receiving`, plugin test `🎚️test-app-config-selection-unit`.
- Design agreed with runtime2 (Option A): `step(source,native,body,grant)` stays and is `native.with_effect(grant,max_work,|e| self.step_in(source,native,body,e))` (None = no-progress step); `step_in(.., effect)` is the one body, and callers already inside a receive_effect/with_effect closure must call `step_in`; only `close_step(&mut self, effect)->OriginalRetirementPoll`, and the seed is a ControlledRetirement over it. I did not land the trait change because it breaks the five implementors at once and they belong to other lanes; it should land in one compile-atomic wave with them.
- os-history: all 73 errors are in `📡️spr/📜️history/🛬️decode` (`🗂️file/🦀️.rs` 41 KB in 106 lines, `📚️frames`, `🗜️deflate`, root). Pattern: `step(source,control,g)`/`close_step(source,control,g)` ladder over history-record-decode (store2 already moved those to `&mut RetainedAllocationEffect`, u128 `last_turn_work`, `NativeAllocationIdentity` ids) plus `NativeDecodeAllocationIssuer` import and `prepare_work`/`commit_work` on `NativeRetirementDecodeControl`. Needs `g` -> `effect`, `child` grants -> `effect.nested(child)`, polls instead of outcomes, and the control identity tuple `(u64,Option<u64>)` -> `(NativeAllocationIdentity,Option<u64>)`. Estimated a dedicated multi-hour slice; start only after store2's history-record-decode is green.

## OpBinaryReceivingCursor wave: Option A is infeasible, claim released, nothing edited
`NativeDecodeControl::with_effect(grant,max,|effect| ..)` (decode/🏭️construction/📦️effect:179) hands the closure only the effect, not the control. The effect-shaped children need the control as well: `RetainedRecordReceiving::step(source,native,effect)` uses `native.charge/checkpoint/owned_bytes/maximum_bytes`, and `native` is mutably borrowed by `with_effect` itself. So a `step(.., grant)` that wraps `with_effect` + `step_in(.., native, effect)` cannot borrow.
Working shapes (decision for runtime2, options):
- B: `receive_effect(demand,max_work,&mut frame,|control,effect| ..)` with a resident `NativeReceivingEffect` frame, as the tool-run tick cursor does (R6). Then `step_in(source,control,body,effect)` is the trait method and every caller (store command cursor, one-item operation wire, hydration, mcp probe-document, plugin test, replication facade tests) owns the frame and the Pending/recovery mapping. Large blast radius over 5 implementors plus callers.
- A': keep trait `step(source,native,body,grant)` and give each implementor an owned `NativeReceivingEffect` frame field; `step` runs `native.receive_effect(demand,max,&mut frame,..)` on it (take the frame out of self around the closure to avoid a self borrow). Callers unchanged; implementors gain the frame, terminal_is_empty requires it Empty, and `close_step(effect)` plus ControlledRetirement seed as agreed. Smaller blast radius, more work inside os-dsl (the generator ladder keeps its own `phase`).
Recommendation: A'. Not started.

## OpBinaryReceivingCursor wave (Option A with port-e's `with_effect(|control,effect|)`)
- Trait (framework replication `🚪️io/💾️binary`): required `step_in(source,native,body,effect)`, provided `step(.., grant)` = `native.with_effect(grant,STEP_WORK,|control,effect| step_in(..))` (None = idle outcome; one shared body, so a provided method, not a shim), `close_step(effect)->OriginalRetirementPoll`.
- os-dsl `🫴️receiving` (+mirrored; macro now declares `DomainFailure`): ported; `receive_to_completion` closes through a bounded `AuthoringWallet` (test/standalone driver). GREEN wasm W-jobencode-21 (rc 0, `-p semio-os-dsl` in `🌎️hub/Cargo.toml`). Cursor types `OperationRetirement`/`ProbeReceivingRetirement` made pub(doc hidden) because they are the RetireOwned seed type.
- mcp probe-document cursor and store command cursor (+ its test double): converted, UNVERIFIED (their crates sit behind red upstream).
- OPEN, not edited: store `🧵️operation-wire/🧳️receiving/🔢️binary/🦀️.rs` (`cursor.close_step(g)` at line 108). It sits in the grant-shaped ladder `ArtifactOriginalOperationReceiving::close_step(control:&mut dyn OriginalRetirementControl, grant)` whose other children (`retirement.step_granted`, `factory.step(g)`, `begin_granted`) are grant-shaped too; there is no effect to hand the cursor. It needs the store lane's whole ladder in effect form, not a signature fit.
- hydration (generic bound only), plugin test-app-config-selection-unit (consumer, stale `.unwrap()` API) and os-dsl/replication tests: not edited.
