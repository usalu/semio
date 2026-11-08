STATUS: GREEN for the 4 assigned crates (native lib port-b-202 rc 0, wasm32-wasip2 port-b-203 rc 0); dependents UNVERIFIED (blocked upstream, port-b-205).

Logs: C/🗑️generated/1105b/port-b-201..205.txt/.rc

Ported
- semio-os-semio: RetainedTextEnvelope::step/advance take `effect`; controlled owner stepped via effect.nested(grant) and RetainedWorkPoll (Pending continues, Complete records progress/failure, OriginalRetirementFailure mapped to ValueError without dropping a cause); RetireOwned via retirement_fields_type!/seed; RetainedTextEnvelopeSources now pub(doc hidden); unwrap/expect/unreachable removed from the stepper.
- os-vcs-entity-identity control: NativeEncodeAllocationPort -> dyn NativeSendAllocationLoan; forwarded hop failures folded by hop_refusal (receipt, context, operation, result error); resume failure restores the receipt; pause failure surfaces its error.
- plugin-window-config-mutation: Box<dyn ErasedWindowMutationValue> RetireOwned via lazily funded ErasedWindowMutationRetirement (admit_payload through admit_retirement_cursor, ChildProgress); refresh_transfer takes effect and returns RetainedWorkOutcome<WindowAuthorityRefreshStep>, restores the displaced snapshot on denial or refusal.
- plugin-window-config-registry: insert (cold) uses reserve_one_cold; insert_original and close_backing_step take effect (funded reserve/release, one performed(1) per pure action).

Downstream debt (callers still grant-based, not mine)
- registry: owner-registry, store-owners, retained/typed-pack-load call insert_original/close_backing_step with a grant.
- mutation: instance stores/typed-ops/staging call refresh_transfer with a grant.
- semio: unit tests in os/semio/tests call RetainedTextEnvelope::step without effect.
- Dependents of the 4 crates cannot be checked until pack-json, io-control, replication-retained-field compile.

## Callers (coordinator request 08:2x) - EDITED, UNVERIFIED (upstream pack-json, io-control, replication-retained-field still red, so semio-plugin-window-config cannot compile)
Re-proof of the 3 lib crates after adding `settle_step`: port-b-206 native rc 0.

Now effect-based:
- registry crate: new `settle_step(poll)` folds a ControlledRetirement poll into a step.
- owner-registry: `close_window_actor`, `close_step`, `close_direct_ingress` take effect; `close_step_context` uses `close_original_effect_step` (Fault kept aside, never dropped); `commit_retained_load_context` uses `with_original_effect` + `receive_original_retained_work` instead of the stale `built`.
- store-owners: trait `close_step`, `close_direct_ingress`, `refresh`, `commit_retained_load` take effect; `close_step_context` as above; partition `insert_original` runs under `with_original_effect` and folds its work once.
- typed-pack-load: `install`, `commit_typed_window_config_pack_load` take effect (insert_original and SharedUtf8::admit_clone via `effect.nested`).
- window-transient: owner and registry `close_step`, `refresh`, registration `insert_original` (same pattern).
- lifecycle-stores `close_owned_stores_step(effect)`; close-ladder OwnedStores rung folds itself via `close_original_effect_step` (added to the quoted-retirement list so the harness does not charge it again); close-ladder and maintenance DirectIngress fold via `with_original_effect`.
- typed-operations: `advance_window_config_refresh(effect)`, caller folds; unit file `granted_effect` helper for the two transient refresh sites.

Not done (needs owner decision or other lanes)
- `ensure_original_partition` still calls the old `admit_original_construction`/`built`/`SharedUtf8::admit(grant)`; construction scope exposes no effect, so construction-based insertion needs a runtime ruling.
- Other ControlledRetirement::step users inside store-owners (`original_close_step`, preview steps), tests (`tests/pack-identity`, os/semio unit `RetainedTextEnvelope::step`), unproven until upstream clears.
- Risk: sites that use `with_original_effect` inside an existing lifecycle harness may be charged `work` twice (safe direction); verify when it compiles.

## ui-viewport / artifact-reference / paged-text (coordinator, exclusive) - GREEN
- Root cause of ui-viewport (3) and artifact-reference (1): `dsl_owned_scalar_receiving!` (dsl/🧬️schema/🧲️bounded-scalar) implemented the old two-argument `DslReceivingField::receive_value`. `copy` arm now mirrors the scalar! impls (`meter.admit`, convert, `owner.receive_copy`); `move` arm is now `(move T, take)` and receives through `owner.receive` + `from_value_controlled` (funded copies). Callers updated (artifact-reference binding, link codec: dead payload/convert helpers removed).
- ui-viewport 4th error: `Viewport2d` NativeDefaultValue gained `default_work_demand`/`default_allocates` and the 3-argument `admit_default`.
- paged-text: no errors left in the lib check (lawfix fixed it); nothing to unblock.
- Proof: port-b-211 native rc 0 (viewport, artifact-reference, paged-text), port-b-212 wasm32-wasip2 rc 0, port-b-213 native rc 0 (semio-os-store-link-schema, the link codec).

## semio-os-vcs (68), semio-os-history-schema (9), semio-os-store-message-ledger (12) - GREEN
Proof: port-b-222 native rc 0, port-b-223 wasm32-wasip2 rc 0 (all three).
- vcs: page-stack and group-visibility cursors on retirement_seed/effect (record_retirement_work per action, nested effect for the controlled child); history-ledger LedgerRetirement on effect (all children via nested effects and RetirementPollExit); history-ledger read API (advance_retained_read, return_retained_read, retain_read) effect-based; checkpoint-identity original owners (pending/branch/commit/repin) close_step and branch advance take effect; u128 work.
- history-schema: OriginalHistorySource::admit now runs through `receive_effect` (resident frame), close_original takes effect; HistoryLog::fold uses an AuthoringWallet (cold entry, no turn) and typed ProtocolError instead of expect.
- message-ledger: both ErasedSnapshotRetirement::close_step on effect via RetirementPollExit::settle; replay diagnostics close_step takes effect (ledger retirement nested effect); store replay caller moved to close_original_effect_step.
- Shared additive: value control-contract `OriginalRetirementFailure::into_value_error`.
Downstream still grant-based (not in this slice): plugin job reserved checkpoint (checkpoint, child-vcs checkpoint), store replay checkpoint/repin/publication, store composition space-member, store testing doubles; unit tests in the history-ledger retirement module are stale.
