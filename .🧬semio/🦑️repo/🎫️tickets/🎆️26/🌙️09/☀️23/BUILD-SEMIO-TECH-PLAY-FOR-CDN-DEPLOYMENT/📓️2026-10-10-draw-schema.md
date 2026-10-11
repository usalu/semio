# 2026-10-10 draw-schema: schema, derives, editor/viewer, window configs

Executor `draw-schema`, scope `✏️s/🔌️plugins/🖍️draw/**` except host/job code (draw-host). Single crate: `semio-s-artifact-draw-drawing` (`--manifest-path ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.toml`, no `component-app-assembly` feature exists in this crate). Logs: `.🧬semio/🦑️repo/⚡️cache/play-fleet/draw-schema/native{1,2}.log`, script `chk.sh` there.

## Results (native `cargo check --lib`)
- Round 1: `could not compile semio-s-artifact-draw-drawing (lib) due to 124 previous errors`.
- Round 2 (after my edits): `due to 59 previous errors; 283 warnings emitted`. Every remaining error is name-resolution or job/host code in draw-host scope; rustc stops before type-check, so my window-config / one-item-preparation / presence edits are NOT yet compiler-verified. wasm32-wasip2 not run (blocked by the same errors).

## Done (my scope)
- Root `🦀️.rs`: `DrawingShapeBody/Path/Text/Image/Group/Boolean/TraceBody` no longer derive `CanonicalJsonTree` (flatten refused); a local `flattened_layer_body_tree!` macro hand-writes the trees (base fields first, then present own fields, ToValue key order). `op` re-exports `inverse_drawing_mutation`.
- Window configs (editor + viewer canvas): state, mutation enum and `Set` payload derive `RetainedClone`; `WindowConfigApplyMutation::exchange` + `admissible` (viewport.validate); owner has `type Edit = WindowConfigApplyEdit<..>`, `MAXIMUM_PREPARATION_DEPTH = 64`, `build_retained_edit` (corrections #35/#36). Viewer `DrawingViewCommand` derives `RetireOwned`.
- Presence: `impl store::ArtifactPresenceSnapshot for DrawingPresence {}` (#34); `DrawingPresenceMutation` derives `RetireOwned` (old hand impl deleted).
- Editor: instance operation owner ported (`retirement_demands`, `maintenance_step/close_step(grant) -> PluginLifecycleStep`); presence retirement factory paths fixed (`crate::editor::drawing::presence::..`); mounted hooks now take grants and delegate to `geometry_session::{maintenance_demands, maintenance, close_demands, close}` (contract sent to main for draw-host).
- Schema: `DrawingIdentityAssignment` derives `DslRecord`; `DrawingSceneText.font_family` added; scene cursor/owners `retired_texts: Vec<String>` added (constructor and RetireOwned already referenced it); `ImportImageAsset` semantic verb `import` -> `add` (not in `APPROVED_VERBS`); scene placement serde derives gated by `cfg(test)` (no runtime serde dependency); identity clone/creation private imports fixed; `TracePath` derives `RetireOwned`; pointer-down `ActorId(.into())`; properties panel `args.take()` instead of cloning `UiValue`.

## Remaining (draw-host scope, first errors)
`PluginCloseStep` imports (geometry, clipboard job, gesture job close ~line 1040 of editor), `ControlledRetirement` import (simplify), clipboard `ClipboardFragment/ClipboardError/NativeDecodeContinuation/InteractionHoverState/owned`, export `svg` path and private `export_stem`, import-image admission (`retained_work_*`, `DrawingImageAsset`, `PagedUtf8AppendCursor` RetireOwned/work_demands), job `step`/`borrow_outcome` shapes (gesture, clipboard, export owned, export-document), geometry `BoundedJob`, `PagedListError`/`PagedListAllocationError` conversions (clipboard decode, io write), clipboard `&str` vs `&PagedUtf8`.

## Next
After draw-host clears resolution errors: re-run native, fix type-check errors in window configs / one-item preparation / presence, then wasm32-wasip2.
