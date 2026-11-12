# remodel (11/12)
## Step 1: remodeling-model
Root cause: BTreeMap no longer RetainedClone. snapshot `assets` / `durable_artifacts` are now `RetainedOrderedMap<String, _>` (aliases `RemodelingAssetStore`, `RemodelingDurableArtifactStore`; Ordered* aliases removed); sqlite snapshot reader builds them with `cold_insert`.
Verified: W-remodel-3 (hub wasm, -p semio-s-artifact-remodel-remodeling-model) rc 0.
Downstream (schema/diff/mutations/io/mesh-export) still use BTreeMap APIs: next.
## Step 2: remodeling-images
Roots: Arc<Vec<u8>>/Arc<Counters> in CompressedChunkRope -> ArcLease (manual Default/Clone, counters AtomicU64 + derive RetireOwned); OpaqueOwner<T> RetireOwned ported to retirement_seed with a pub-API cursor `OpaqueOwnerRetirement` (one charged drop).
Verified: W-remodel-6 (hub wasm, -p semio-s-artifact-remodel-remodeling-images) rc 0.
## Step 3: remodeling-model-diff (+ schema)
- schema: `assets` is `RemodelingAssetStore`.
- diff: cold apply/inverse on `RetainedOrderedMap` stores (`cold_insert`/`cold_remove`); rows-buffer/rows-edit ported to effect shape (`advance/close_step` take `&mut RetainedAllocationEffect`, return `OriginalRetirementPoll`, Pending passes through); `RemodelingApply` rewritten reply-shaped: 9 `FieldSlot<Spec>` (RetainedFieldPreparation + staged value + RetainedCloneClose) + `ParamsSlot` (RetainedRowPatchApply), composite like EnergyDocumentApply (preflight_reply/commit(effect)/close_reply, `retained_routed!`).
- Verified: W-remodel-9 (hub wasm, -p semio-s-artifact-remodel-remodeling-model-diff) rc 0.
## Step 4: component reds revealed (W-remodel-13 -> next)
- io: DiffBinary/DiffText for RemodelingDiff moved into the diff crate (orphan rule; `semio_os_kernel::__diagnostic` for TextError); `insert` -> `cold_insert`.
- mut-results / mut-media-2 / editor: `contains_key` -> `get().is_some()`, `keys()` -> `iter()`, `insert` -> `cold_insert`.
- mut-params-2: RetainedParameterTextOwner / DiagnosticOwner effect-shaped (advance(effect), close_step(body,effect)->Poll, settle); (cfg(test) module there still grant-shaped: debt).
- mutations: `RemodelingMutation` OpBinary now encodes/decodes the pack wire value (was tagged protocol) so that `dsl::original_pack_value_operation_receiving!(RemodelingMutation)` (OpBinaryReceiving) matches; the tagged `📡️.protocol.semio` bytes are no longer the op encoding (io binary tests are debt).
- cmd-workflow: 15 mutation builder imports `mutations::x` -> `mutations::x::x` (module vs fn).
- Remaining after W-remodel-13: remodeling-session (39), viewer (OpBinaryReceiving, fixed above, unverified), cmd-workflow (fixed above, unverified).
## Step 5: semio-hub-remodel-component GREEN
- W-remodel-21: `"$C/1105b-tools/chk.sh" W-remodel-N "🌎️hub/Cargo.toml" wasm -p semio-hub-remodel-component` rc 0 (hub wasm).
- session: run-job/revalidate-job close_step effect-shaped (`close_original_effect_step`/`close_original_retained_effect_step`), RunRemnant scene as `ArcLease`, Outbox close/retire via effect (`settled` helper).
- modes: three window-config mutations: OpBinary now pack wire value + `semio_os_kernel::original_pack_value_operation_receiving!`.
- cmd-ingest/cmd-workflow/artifact-editor: mutation builder imports `mutations::x::x`.
- editor: `extern crate semio_os_kernel as dsl;` + deps semio-framework-diagnostic/-dsl-record/-dsl-record-derive (app_commands! expansion); remodeling top crate gains remodeling-inferences (inference descriptor path); inferences `RemodelingPoseDelta: RetireOwned`; semio-hub-remodel gains semio-framework-value. LOCK NEEDED entries logged (cargo auto-refreshed the hub lock).
## Debts / not verified
- cfg(test) targets across remodel (grant-shaped helpers: mut-params-2 numeric_text_tests, rows-buffer/edit tests, diff retained tests, io binary op tests now on pack wire encoding, images tests with `Arc<[u8]>` push) are not compiled or ported.
- rows-buffer / rows-edit in model-diff are dead code (no caller); ported to effect shape only.
- mesh-export (out of closure) uses old `RemodelingOrdered*Store` aliases removed from the model: not compiled here; needs `RemodelingDurableArtifactStore` (RetainedOrderedMap) port when it is wired.
- Native (non-wasm) check not run.
Rerun: "$C/1105b-tools/chk.sh" W-remodel-22 "🌎️hub/Cargo.toml" wasm -p semio-hub-remodel-component
STATUS: FIXED-verified W-remodel-21
