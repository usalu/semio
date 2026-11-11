# Lane instdocs result (semio-plugin-instance-docs port)

## State
- hub-6 had 34 errors; W-instdocs-1 (native) 27 after the earlier fixes; W-instdocs-2 1 error (u128 work narrowing); native W-instdocs-3 rc0; wasm hub W-instdocs-5 rc0 (W-instdocs-4 was blocked upstream by closeshared's SharedCloseRoute Eq, fixed by them).
- document-media: `envelope_decode_replacement` still exists in `🗃️document/📨️envelope-decode-and-ingress`; it was only missing from the root `pub use` (re-exported).
- media-download: `ManuallyDrop` import.
- checkpoint: `polled()` helper turns an `OriginalRetirementPoll` into progress (Pending keeps state); error-place/`close_factory_ticket*` closes run in `cx.with_original_effect(grant, ..)` with the new `(slot, body, effect)` shape; error owner close via `close_original_effect_step(cx, demand, |effect| plugin_typed_owner_original(.., effect))` inside the retained loan; derived-checkpoint step uses `with_native_receiving(&mut scope, |native,_| ..)` with a `NativeSource` failure when the scope retains a call/source refusal.
- lease/render: `ToolCancellationLeaseRetirement::close_step(canonical, effect)` and refusal owner/place closes are effect-shaped (render via `close_original_effect_step`).
- maintenance-ladder: archive close runs `ActiveDocumentArchiveLoad::close_step(effect, &StepTurn)` inside `with_original_effect`; Pending maps to default progress with its work; `observed`/`work` u128 narrowed to u64 for `receive_observed_work`.
- ingress: `maintenance_returned_reads_demands_root(body)` lost its factory argument.

## Debts
- (resolved) No Arc/ArcLease sites existed in these files except `NativeCheckpointExportSource.snapshot` (bare `Arc<A::Snapshot>`, not yet an ArcLease; compiles as is, unverified against RetireOwned bounds).
- `with_original_effect` performed work is not added to fuel in the checkpoint/lease/render sites (same as the existing preview/snapshot branches).

## Rerun
`chk.sh W-instdocs-N Cargo.toml native -p semio-plugin-instance-docs`; wasm: `chk.sh W-instdocs-N 🌎️hub/Cargo.toml wasm -p semio-plugin-instance-docs`

## Round 2 (coordinator)
- `NativeCheckpointExportSource.snapshot` is `ArcLease<A::Snapshot>` (store retires `lease.0`; field cleared when empty).
- Performed work from `with_original_effect` is charged to fuel in checkpoint (error place, preview, factory ticket, snapshot), lease and render closes.
- store/🦀️.rs:300 re-export of nonexistent NativeSnapshotEncodeEffectQuote removed (kernel E0432).

STATUS: FIXED-verified W-instdocs-8 (native) + W-instdocs-9 (wasm32-wasip2, hub manifest)
