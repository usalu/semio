# Draw viewer: undeclared canvas actions

## Root cause
The shared Canvas2d surface (native `scene-input` and `Canvas2dHost`) dispatches `canvasPointerDown/Move/Up`, `canvasDoubleClick`, `canvasDragOver/Leave`, `canvasDrop` and `setCamera` at every Canvas2d window, read-only or not. The shell gate (`undeclaredActionDiagnostic`) is app-wide: an action must be declared on some window kind of the app. The Draw viewer's `drawing-view-canvas` declared only `setCamera`, so every hover dropped `canvasPointerMove`. Wheel zoom is session-local and syncs through `setCamera`, which was already declared. There is no wheel or escape dispatch.

## Fix (framework contract: declare the host vocabulary, as the wfc grid2d viewer does)
- Canvas window kind declares the seven host input verbs as inert, non-palette `ActionKind::View` actions, plus `setCamera`. Exported `HOST_INPUT_ACTION_IDS` and `HOST_ACTION_IDS`.
- Viewer command gains seven unit variants. They are routed through the retained tool factory (`TOOL_IDS`, proofs list) with the `HostOnly` publication lane, and `camera_emit` returns an empty `Emit` for them. `setCamera` keeps the `WindowConfig` lane. The viewer still emits no document mutation.
- Laws in the viewer unit tests: `drawing_viewer_canvas_declares_every_action_its_surface_dispatches`, `drawing_viewer_host_input_commands_publish_nothing`, and the updated contract test.
- No descriptor or TS mirror of viewer window-kind actions exists (the publication-authority fixture and the draw-js twin are editor only), so the coordinator's regeneration needs no manual edit.

## Files
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️canvas/🦀️.rs`
- `.../👁️viewer/🦀️.rs`
- `.../👁️viewer/🧪️tests/🔬️unit/🦀️.rs`

## Native evidence
Not obtained after the coordinator's retry request. Seven reruns of the viewer package tests (via `--manifest-path` of the drawing workspace) all failed for environmental reasons before any viewer test ran:
- compiler image full (ENOSPC), then reclaimed;
- capacity wait cancelled by an external SIGTERM;
- host memory warden killed the build three times (cargo, then `semio_draw_member_decode` twice);
- `Closed repository cache partition declaration required` from repo tooling (a peer's in-flight change).

The last run compiled framework and store crates up to the warden kill, so the Draw crates are the remaining units. No compile error attributable to this change was seen. Re-run when host memory is calm:
`repo capacity run --memory=1800 --cpu=1 --jobs=1 -- repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --manifest-path <drawing>/Cargo.toml -p semio-s-artifact-draw-drawing-viewer`
