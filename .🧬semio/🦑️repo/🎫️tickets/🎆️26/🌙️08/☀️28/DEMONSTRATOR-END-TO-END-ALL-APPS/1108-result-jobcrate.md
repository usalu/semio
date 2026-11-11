# Lane jobcrate result (semio-plugin-job)

## State
- W-jobcrate-1 baseline: 46 errors. Ported (uncompiled until W-jobcrate-2): registration (catalog + alias + registration alias leases via `ControlledRetirement<ArcLease<DeclaredToolCatalog>>`, one shared `DeclaredToolAliasRetirement` cursor with `native_retirement_reply!`, `retirement_seed`), reserved (`QualifiedToolProof` seed, outcomes `settle` split, lease `close_original_effect_step`, `NativeEncodeControl` pause error), full-operation, capture decoder identity type, worker work->fuel, lib lease close + removed unused work quotes.
- Coordinator request: `ConfigurationDecodeRetirement::step_granted` removed; `step(effect)` is the only protocol, the reserved configuration job calls `owner.step(effect)` directly (child via `ControlledRetirement::step(body,effect)` + `RetirementPollExit::resume`).

## Interface changes for callers outside this crate
- `OriginalDeclaredToolCatalog::close_step(effect) -> OriginalRetirementPoll`, `DeclaredToolCatalogAlias::close_step(effect)`, `DeclaredToolRegistrationAlias::close_step(effect)` (were grant -> Result); new `close_work(body)`; `DeclaredToolRegistrationAlias::into_catalog_alias`. Callers still on grants: maintenance-ladder close-ladder:181/182/266/267, command typed proof/typed/admission, instance jobs components declarations capture.

## Verification
- W-jobcrate-2 wasm32-wasip2 via hub manifest: rc0 (semio-plugin-job lib). W-jobcrate-3 native: rc0.
- Hub W-jobcrate-hub-1 (rc1): single red, `semio-framework-manifest-view-model` `🔗️shared-source/🦀️.rs:5` (`Arc<ViewModel>: RetireOwned` not satisfied; `impl OriginalSourceProjection<ViewModel> for Arc<ViewModel>` needs `SharedOriginalStorage`/an owner bound). Owner: custody (claim 08:05). Everything downstream (plugins, hub components) not reached yet; rerun `hub.sh W-jobcrate-hub-2` after custody fixes it.

## Open items / debts
- Callers of the registration closes still on grants: maintenance-ladder close-ladder (181/182/266/267), command typed proof/typed/admission, instance jobs declarations capture; tests in this crate (`🧪️tests`) not compiled (`--tests` not run).

## Round 2 (coordinator: callers of the registration closes)
- close-ladder (maintenance-ladder 181/182): `declared_poll` helper; Pending stays an unfunded retry, settled failure becomes the plugin fault. 266/267 demands unchanged. Instance declarations capture: catalog close via `close_original_effect_step(cx,d,|effect|catalog.close_step(effect))`.
- command typed `proof`, `admission`, `typed`: local grant helpers removed; births via `born` (resident `OriginalConstructionAdmission` + failure slot on `OriginalTypedCommandSource`, new `frame/failure/slot` fields, re-entry gates), internal moves via `acted/produced`, closes via `retired/begin/settle`, decoder identity `(NativeAllocationIdentity,u64)`. `semio_plugin_job::original_steps` is now a public re-export (no manifest change). These three files compiled clean in W-jobcrate-6 (no errors in them).
- W-jobcrate-6 (wasm, command + instance-ladders + instance-jobs): command lib still has 58 errors, all in files that were never ported and are NOT in the registration/proof scope: `command/📥️admission/🦀️.rs` (latest-wins admission: `SharedOriginalStorage`/ordered `SharedOwner` `admit`, `next_release_byte_demand`, `release_step` gone; `plugin_typed_owner_original`/`.result` on Poll), `command/🗝️latest/🦀️.rs:33`, `command/🎟️typed/📨️wire/🦀️.rs`, `command/🎟️typed/🏃️runtime/🦀️.rs` (old `original`/`built` helper, 6-arg construction). instance-ladders and instance-jobs not reached (blocked by upstream document-media / publication-mounted).
- Hub W-jobcrate-hub-2 rc1: only `semio-framework-value-factory` (4x `ReplyStash` not found in crate root, arc-custody factory + factory/root), owner `factoryticket` (claim 08:57, ReplyStash module move). view-model is no longer red. Everything past value-factory is unreached.

## Remaining reds and owners
- value-factory ReplyStash: factoryticket.
- document-media (20), publication-mounted (6), retained-command (8), scrub-host (4), typing-edit (4): pubmedia / toolmachine / custody per claims log (seen in W-jobcrate-4).
- command crate rest (58): UNOWNED after my claim is released; needs a lane for latest-wins admission over the new SharedOriginalStorage/ordered SharedOwner retirement (`retire()`, `ArcLeaseRetirement`) and wire/runtime helpers.

STATUS: BLOCKED command rest (latest-wins admission/wire/runtime, 58 errs, needs SharedOriginalStorage redesign) and hub upstream value-factory ReplyStash (factoryticket); semio-plugin-job FIXED-verified W-jobcrate-3
