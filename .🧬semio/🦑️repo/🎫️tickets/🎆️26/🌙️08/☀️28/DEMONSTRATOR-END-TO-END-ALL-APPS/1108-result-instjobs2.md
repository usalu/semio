# Lane instjobs2 result

## State
- Ported to ArcLease (instance INTERFACE): render work.rs `OriginalRenderWork<ArcLease<S>>` (snapshot_loan still returns `&Arc<S>`), work-tests, components `pending_render_source`, plugin-app-impl render-surface (retain + original callers).
- render close paths effect-shaped (`close_original_effect_step`), except OriginalBuiltTreeRetirement (stays grant-shaped).
- typed-operations close_configuration_failure: `original_lifecycle_effect_step` + `plugin_settle_poll`; components: removed `owner.receipt` arm; declarations: effect-shaped close_step, units `[None;2]`, explicit `Ok::<_,ValueError>`.
- W-instjobs2-1 (27 errs) -> edits -> W-instjobs2-3 native rc0, W-instjobs2-4 wasm rc0. No tests run; render work-tests ported to ArcLease but unrun.

## Rerun
`chk.sh W-instjobs2-N Cargo.toml native -p semio-plugin-instance-jobs`; wasm: `chk.sh W-instjobs2-N 🌎️hub/Cargo.toml wasm -p semio-plugin-instance-jobs`

STATUS: FIXED-verified W-instjobs2-3 (native), W-instjobs2-4 (wasm hub)
