# Generation3d Generate-Mode Preview Eval Wiring — 2026-09-11

Generate-mode preview now uses the same `flowEvalTick` → `ExtensionInvocation` → `flowEvalResolve` → tessellate → window-transient path as edit-mode preview. The chain is addressed at window id `generation3d-generate-preview`.

## What changed

- Preview roster includes both `procedural-preview` and `generation3d-generate-preview`. `pending_effects`, `setContributions`, and `setActiveExample` re-arm every attached preview.
- `FlowEvalTick` / `flowEvalResolve` / `flowTessellateResolve` carry `windowKindId` so `retained_window_transient_target` captures the correct kind.
- Tick work admits either preview kind and publishes into that window's transient. Generate ticks evaluate the patched generation fixture via `generation_fixture_for` when a generation is selected; otherwise evaluate is a no-op until `addGeneration`.
- `Generation3dPreviewCommandWork` no longer runs a dead synchronous `FlowEvalSession::tick`. Generation commands emit the mutation and re-arm `flowEvalTick`. Unused patched fixtures are `retire_cold()`'d.
- Generate preview render reads window-transient eval text and passes `Some(session)` into `preview_payload`.
- Generate preview registers `Generation3dGeneratePreviewWindowTransientOwner` (same state schema as edit, different `WINDOW_KIND_ID`).
- Language-agnostic fixture `tick-addressing.json` plus third-party twin `contract.ts` (`JSON.parse`) require generate preview on the same arming/dispatch chain as edit preview.
- Package tests enable Cargo feature `component-app-assembly` so editor + example-geometry trees compile.

Ingress (64-page / 262144 B) and the 512 MiB wasm budget were not changed.

## Compile follow-ups (2026-09-11)

- Editor unit constructors for `FlowEvalResolve` / `FlowTessellateResolve` now set `window_kind_id`.
- `generation3d_render_body` test calls match the 7-argument signature (`preview_eval_text`, no extra `None`).
- Tick-addressing `brep_extension::settle(&mut *app, …)` derefs `Generation3dAppFixture` to `PluginApp`.
- `set-contributions` unit test imports `semio_framework_artifact_flow_flow::neural::ColdRetire`.

## Files changed

- generation3d editor `editor.rs` — roster, retained target, tick work, render dispatch, tessellate args
- `commands/flow-eval-tick`, `flow-eval-resolve`, `flow-tessellate-resolve`
- `commands/set-contributions`, `commands/set-active-example`
- generate-mode preview window + new window-transient owner
- `fixtures/tick-addressing.json` and `tests/tick-addressing` (Rust + `contract.ts` twin)
- testkit `generate_shell_views` and `FlowEvalTick` constructions that now pass `windowKindId`
- `packages/rust/script.ts` — `testFeatures: ["component-app-assembly"]`
- editor unit / set-contributions unit / tick-addressing tests (signature + trait-scope fixes)

## How to verify

`runCargoTestBudgeted` already floors `RUST_MIN_STACK` at 128 MiB. A bare `cargo test` on a default 2 MiB worker thread overflows in debug; use the nx/bun wrapper or export `RUST_MIN_STACK=33554432`.

Native rust (no wasm restage):

```
nx test @semio-tech/procedural-generation3d-rs -- generate_preview_eval_emits_extension_or_rearms_flow_eval_tick
nx test @semio-tech/procedural-generation3d-rs -- every_armed_tick_names_a_preview_window_that_is_actually_attached
nx test @semio-tech/procedural-generation3d-rs -- only_a_preview_addressed_tick_passes_the_retained_preflight
```

Equivalent cargo (lib + feature):

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- editor::generation3d::component::tick_addressing::
```

Third-party twin: `bun` the `contract.ts` under `editor/tests/tick-addressing`.

Playground generate preview needs a **wasm restage** of the procedural / generation3d guest. Native tests do not.

## Verified 2026-09-11 (native)

Ran `tick_addressing` lib tests with `RUST_MIN_STACK=33554432` and `component-app-assembly`:

- `every_armed_tick_names_a_preview_window_that_is_actually_attached` — `generate-preview-attached` arms `generation3d-generate-preview`.
- `only_a_preview_addressed_tick_passes_the_retained_preflight` — `generate-preview-addressed-from-generations` admitted (`refusal=None`).
- `generate_preview_eval_emits_extension_or_rearms_flow_eval_tick` — `[DEBUG] extension runner received extension=flow-extension-brep capability=tessellate ok=true` then `rearmed=[] answered=1`.
- `set_active_example_drives_the_self_dispatched_tick_chain_to_a_rendered_mesh` — edit path still finishes with `meshes=1`.
- `contract.ts` — `generatePreview=generation3d-generate-preview armed=generation3d-generate-preview admitted=true`.

## Expected tick law

1. Generate roster attached → `pending_effects` arms `flowEvalTick` at `generation3d-generate-preview`.
2. `setActiveExample` / `setContributions` / generation commands re-arm the same id.
3. The first admitted tick emits `ExtensionInvocation` or re-arms `flowEvalTick`. It must not complete as a host-local sync tick with no continuation.
