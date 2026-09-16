# 🗺️ Plan — fem2d interactive feature completeness (2026-09-16)

Coordinator: session ⚪34c772d7 (Fable 5.1). Execution: Opus agents per slice, Sonnet agents for audits. Inputs: `📓️explore-2026-09-16-{editor-architecture,ui-primitives-and-precedents,schema-mutations-solver,runtime-and-verification}.md`.

## Architecture decisions

- **Selection is framework-owned.** fem2d declares one `InteractionDefinition` domain `FEM2D_INTERACTION_DOMAIN = "fem2d"` with granularities `node`, `element`, `region`, `support`, `load`, `material`, `section`, `loadCase`, `combination`; a target id is the RAW entity id (loads: the load id; the owning case is resolved by lookup). Both Canvas2d windows bind the domain (`window_kind_interactions`). Viewport picks: the host emits `canvasPointerDown/Move/Up` (`📐️Canvas2dHost/🟦️.tsx:550-611`, args `x,y,button,shift,ctrl,meta,alt,width,height`); the app hit-tests in screen space (`screen_2d`, camera from the addressed window config) and requests `Effect::ReplayShellCommand { interactionSelect, {domainId, targets, merge, method:"pick"} }` (draw precedent `🖱️canvas-pointer-down/🦀️.rs:84-99`); pointer move requests `interactionHover` (channel `pointer`). Rendering reads `InteractionView` via `render_with_request_context` and paints selected/hovered entities.
- **Panels** (`📌️panels/`, repo convention, cad/puzzle precedents): `🗿️artifact` (outliner, paged `PanelTreeBuilder` bound to the domain, row pick = `INTERACTION_SELECT_ACTION_ID`, row actions ≤ 2: focus + delete), `🔍️inspection` (selected entity fields with real controls: `input(Number)`, `select`, `toggle`, `slider` bound with `Trigger::Change` → `patch*` commands; document summary when nothing is selected), `📊️results` (results + playback control panel: source select, mode select, mode index stepper, deformation scale slider, phase slider, play/pause, speed, loop, step buttons → `setResultDisplay` / `setResultAnimation` / `setAnalysisSettings`, all tagged with the results `windowId`).
- **Editable inspector → mutations.** Existing `replace-element/material/section/support/region` + `update-analysis-settings`; NEW (slice C): `replace-node`, `replace-load`, `change-load-case-name`, `replace-combination`. Patch commands take `{ id, field, value }` with `value` as text (the host merges the control's value under `value`; the bridge stringifies numbers/bools) and emit the whole-record replace with one field changed.
- **Animation** lives in the results window config (`Fem2dResultsWindowConfig.animation: Fem2dResultsAnimation { phase: f64 0..1, playing: bool, speed: f64 cycles/s, loop_mode: Loop|PingPong|Once, waveform: Ramp|Sine }`), never in the document. Playback clock: `resultAnimationTick` handled window-scoped advances `phase` and re-emits `Effect::DispatchAction { action: "resultAnimationTick", args: {windowId}, delay_ms: 33 }` while `playing` (host `🏛️ShellHost/🟦️.tsx:5555` honours `delayMs` and keeps the target view state). Render draws `fem2d_deformed_shape_layers(doc, disp, scale × waveform(phase))`. Results are cached per (app instance, base revision) in a thread-local so ticks do not re-solve.

## Command contract (rows appended to `app_commands!` in this order — binary ordinal, never reorder)

| Manifest id | wire | payload | scope |
|---|---|---|---|
| `canvasPointerDown` | `canvas-pointer-down` | `{x,y,width,height,button:u32,shift,ctrl,meta,alt}` | window (camera) — pick |
| `canvasPointerMove` | `canvas-pointer-move` | `{x,y,width,height}` | window — hover |
| `canvasPointerUp` | `canvas-pointer-up` | `{x,y,width,height,shift,ctrl,meta,alt}` | window — no-op / drag end |
| `patchNode` | `patch-node` | `{id, field, value}` fields `x,y` | doc → `ReplaceNode` |
| `patchElement` | `patch-element` | `{id, field, value}` fields `kind(bar/beam),start,end,materialId,sectionId` | doc → `ReplaceElement` |
| `patchMaterial` | `patch-material` | `{id, field, value}` fields `name,e,nu,rho` | doc → `ReplaceMaterial` |
| `patchSection` | `patch-section` | `{id, field, value}` fields `name,area,iy` | doc → `ReplaceSection` |
| `patchSupport` | `patch-support` | `{id, field, value}` fields `nodeId,tx,ty,rz (bool toggles)` | doc → `ReplaceSupport` |
| `patchRegion` | `patch-region` | `{id, field, value}` fields `name,thickness,meshSize,materialId` | doc → `ReplaceRegion` |
| `patchLoad` | `patch-load` | `{id, field, value}` fields `nodeId,dof,value,elementId,wx,wy,regionId,pressure` | doc → `ReplaceLoad` |
| `patchLoadCase` | `patch-load-case` | `{id, field, value}` fields `name,selfWeight` | doc → `ChangeLoadCaseName` / `ChangeLoadCaseSelfWeight` |
| `patchCombination` | `patch-combination` | `{id, field, value}` fields `name`, `term:<caseId>` (factor; 0 removes; unknown caseId adds) | doc → `ReplaceCombination` |
| `setResultAnimation` | `result-animation` | `{phase?,playing?,speed?,loopMode?,waveform?}` | window (results) |
| `resultAnimationTick` | `result-animation-tick` | `{}` | window (results) — self re-armed |
| `focusEntity` | `focus-entity` | `{id}` | window (model) → `setCamera` on the entity |

Shared-file ownership: the coordinator owns `✏️editor/🦀️.rs` (enum rows, bridge arms, manifest, render dispatch), the crate root module mounts, and the `every_command()` law rows. Slice agents own their leaf files only.

## Slices

- **A — interaction + picking + highlight (Opus)**: `✏️editor/🕹️interaction/🦀️.rs` (domain definition, hit-testing, target/effect helpers, `Fem2dSelection` snapshot from `InteractionView`), commands `🖱️canvas-pointer-down/move/up`, `🎯️focus-entity`, model+results window highlight painting, delete keybinding via `removeSelection` reading the live selection, tests.
- **B — artifact tree panel (Opus)**: `📌️panels/🗿️artifact/🦀️.rs` + tests, `🗣️terminology/🦀️.rs` label set (shared with D and E — B creates it first with every label the plan needs).
- **C — mutations (Opus, running)**: the four new mutation kinds on every surface.
- **D — inspector + patch commands (Opus)**: `📌️panels/🔍️inspection/🦀️.rs` + tests, the nine `🩹️patch-*` commands + tests.
- **E — results panel + animation (Opus)**: results window config schema (`animation`), `⏯️set-result-animation` + `⏱️result-animation-tick` commands, results cache, animated render, `📌️panels/📊️results/🦀️.rs` + tests.
- **F — coordinator**: skeleton, integration build (native + wasm32-wasip2), `component-dev`, activation, serve on 6086, browser probes, QA fleet (Sonnet audits), status/close.

## Verification gates

1. `cargo check -p semio-s-artifact-fem-2d --features component-app-assembly --tests` green (ticket `📜️check-fem2d.sh`).
2. `cargo nextest run -p semio-s-artifact-fem-2d --features component-app-assembly --profile fundamental` green; `bun nx run @semio-tech/fem-2d-rs:test` and `fem-plugin:test` green.
3. `bun nx run @semio-tech/fem-plugin:component-dev` green (wasm32-wasip2).
4. React lane: `activate-fem2d-react-dev` + serve on 6086; console-clean boot; artifact tree lists the demo; tree click selects and highlights; inspector edit dispatches and the viewport updates; play animates the deformation; proven with console dumps in `🗑️generated/`.
