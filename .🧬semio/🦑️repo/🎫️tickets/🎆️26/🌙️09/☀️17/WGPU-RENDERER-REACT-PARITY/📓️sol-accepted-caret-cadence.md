# Accepted Caret Cadence

## Scope

This packet replaces the disconnected global 500 ms renderer sweep with caret state owned by an exact accepted retained surface. The shared clock identifies the accepted `UiSurfaceToken`, document node id, caret source, and visible phase. It arms only for an editable focus, starts solid, toggles every 500 ms, resets after accepted editing input, and cancels on blur, accepted hide, close, commit, or exact identity retirement.

The neutral contract lives in `🧰️framework/🔨️modules/🖱️ui/🧬️contract/⌨️caret-cadence`. It covers focus, the 499/500 ms boundary, edit reset, blur, accepted hide, idle ticks, and solid refocus. The actual React oracle mounts the real TextEditor hidden textarea cadence from `TextEditor/🟦️.tsx` and evaluates the same fixture.

## Production closure

- `EventRouter` owns retained and scene caret clocks and contributes their exact next deadline to the existing per-window monotonic clock.
- `Ui` exposes token-qualified scene arm/clear results and `UiWindowClockStep.caret`; no candidate seal mutates accepted cadence.
- Production retained paint emits bounded caret primitives for Input, IconSelect, NumberStepper, and Slider readout editing.
- TextEditor installs exact accepted surface/document focus, applies every due clock step, resets on text/key input, and clears on blur or retirement.
- NodeGraph note editing now owns an exact accepted component focus. Pointer entry arms cadence only after a real Flow note edit exists; keys route only to that component; edit resets the solid phase; commit or focus transfer cancels it. The old process-global scan is removed.
- Every `stamp_window_clock` caller applies the returned scene-caret phase before continuing, so a due phase cannot be consumed and discarded ahead of the retained clock phase.

## Verification

Fresh actual React oracle:

```text
SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=@semio-tech/framework-renderer-wgpu --excludeTaskDependencies -- bunx vitest run --config <react-config> <caret-cadence-test>
Test Files 1 passed (1)
Tests 2 passed (2)
Duration 19.19s
```

Native laws supplied to the parent coordinated run:

- `accepted_input_caret_starts_solid_blinks_resets_and_cancels_from_the_shared_fixture`
- `scene_caret_requires_exact_accepted_surface_and_document_identity`
- `retained_input_paints_a_caret_only_while_the_accepted_cadence_is_visible`
- `accepted_node_graph_note_caret_arms_resets_and_retires_with_its_exact_scene`

The Rust files were parsed by scoped Nx `rustfmt`. Native and WASM results remain owned by the coordinated parent build and must be appended from its receipts before a green claim.

Parent build receipt after source closure: WASM13's first renderer Cargo pass completed successfully in 9m36s. The publication stage was still pending when recorded, so this is production Rust compile evidence rather than an end-to-end artifact result.
