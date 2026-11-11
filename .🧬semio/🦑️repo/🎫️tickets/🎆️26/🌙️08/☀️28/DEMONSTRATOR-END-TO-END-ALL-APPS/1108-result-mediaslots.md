# 1108 mediaslots result

## State

### semio-os-store-sync (2 errors)
- `actor_identity_batch_admits(count, effect)` quotes against the calling turn's effect (grant + remaining work, one unit per lease).
- `wire_envelopes_with_original_actor(envelopes, actor, effect)`, `envelopes_from_history_edit(.., effect)`, `spr_events(.., effect)` take `&mut RetainedAllocationEffect`; every identity lease is `SharedUtf8::admit_clone(effect)`, charged to the caller's turn (no grant, no wallet). Progress receipts are still returned for the caller to consume.
- Callers outside the lib (native/wasm `ArtifactActor` crates under `🖥️native`, `🌐️wasm`, `actor_identity_grant` config) still hold a grant and call the 3-arg `SharedUtf8::admit`; they were already red before this change and are not part of this lane. They need a turn that owns the effect (job turn per socket flush / bootstrap); not designed here.

### semio-os-services-native-io (7 errors)
- payload pump `close_step(effect)` (builder `close_step(effect)`), `NativeIoValue::close_step(effect)`, `NativeIoJob::close_granted(effect)`; the two job entry points run through `close_original_effect_step` (`progress_poll` settles the grant-based children `paths/entries/read buffer` with their own work).
- E0283: `RetainedWorkOutcome::<RetainedCloneStep,ValueError>` annotations at five `close_original_retained_step` closures (lib + path-return).
- Test debt: `📦️payload/tests.rs` and `🧪️tests/🔬️native-io-unit` still call the grant-shaped `close_step(g)`.

### semio-renderer-wgpu-media-slots (29 errors)
- props: `ledger: Option<NativeAllocationIdentity>`; `advance` runs `parser.step_source` inside `cx.with_original_effect(grant, ..)` (None = not admitted -> Ok(false)), fuel charged with the performed work; close via `close_original_effect_step(.., |effect| parser.step(bytes, effect))`.
- slot: same close; `NativeEncodeControl::resume/pause` errors `(error, continuation)` keep the continuation in `self.encoding`.
- collect: `act(cx, demand, run)` helper = one quoted effect (grant exactly the demand, fuel = performed, retained progress consumed, failures debited). Measurement chain redone: `RetainedCloneSource::admission_owned(slots, ())` driven by `advance`/`take_output`; projection through `RetainedOwnedProjectionAdmission<dyn ArtifactCanonicalJsonTree>` with two `ReplyStash<ValueError>` (admit, take); authority handback through `take_authority_into` with `ReplyStash<OriginalRetirementFailure>` into `RosterOwners.back` then moved to `slots`; canonical tree `advance(effect)`/`close_step(effect)`. Close order: tree, projection, projecting admission, source, source admission (`return_inputs` into `owners.slots`), params, assembly, failure.
- Failure staging: `json_failure`/`compound` go through `OriginalDomainReservation` (`admit_staged`: prepare, with_body, admit), reservations in `terminal_is_empty`.
- native decode `resume/pause` errors keep the continuation.

### Actor crates (store-sync-native 4 errors, store-sync-wasm 8 errors; coordinator add-on)
- Design: the actors are not job turns, so identity leases run in a host-funded pump turn: `actor_identity_turn(units, grant, |effect| ..)` (store-sync, `OriginalRetirementPump::turn(fuel = lease count)` + `with_effect`; a quote the pump does not admit is a typed WorkLimit, nothing runs). Cargo.toml: `semio-framework-job = { workspace = true }` added to store-sync (LOCK NEEDED logged).
- Async split: `spr_events` replaced by `spr_history(spr).await` (decode) + sync `history_events(log, doc, schema, effect)` / `history_events_in_turn(log, doc, schema, grant)`; `envelopes_from_history_edit` and `op_ids_of` are sync now (nothing awaited).
- native + wasm actors: `wire_envelopes_with_original_actor` inside `actor_identity_turn(envelopes.len(), grant, ..)` (Err keeps the existing queue_outbox fallback, Pending keeps state), bootstrap/folder reads via `spr_history` + `history_events_in_turn`, 3-arg `SharedUtf8::admit(.., &mut None).result`.
- wasm only: dropped native-only `now_ms` import (the browser never stamps a clock), added `acknowledged_edited_head` import and `OpBinary` (for `BackboneMessage::encode_op`).
- Debt: store-sync tests (`hostile-binding-heartbeat` fixture still calls `envelopes_from_history_edit(.., grant)`; fixtures/`SharedUtf8::admit` 2-arg at hub-link 404/413) are stale; the `actor_identity_progress` accumulator now only sums returned receipts.

## Verification
- media-slots: native W-mediaslots-4 rc0; wasm32-unknown-unknown W-mediaslots-7 rc0 (at the time with hosting features, W-mediaslots-12 without).
- store-sync + native-io: W-mediaslots-8 rc0 (native).
- store-sync-native: compiled clean (warnings only) in W-mediaslots-10; the shell-session native check there is red only in plugin crates owned by peers (host-configuration 13, job-receiving 14, decl-app 38, host-wake 9, host-shard 13).
- store-sync-wasm + media-slots: W-mediaslots-12 (wasm32-unknown-unknown, no extra features: hosting deps are no longer optional) rc0.

## Rerun
- shell native: `"$C/1105b-tools/chk.sh" W-mediaslots-N "<shell-session Cargo.toml>" native -p semio-renderer-wgpu-shell-session`
- wasm: same manifest, `native --target wasm32-unknown-unknown -p semio-os-store-sync-wasm`

STATUS: FIXED-verified W-mediaslots-12 (wasm32-unknown-unknown: media-slots, store-sync-wasm), W-mediaslots-4 (native media-slots), W-mediaslots-8 (store-sync, native-io), W-mediaslots-10 (store-sync-native); shell-session native W-mediaslots-10 red only in peer plugin crates (host-configuration, job-receiving, decl-app, host-wake, host-shard)
