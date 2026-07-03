---
name: Command Line Input Elements Refactor
overview: Establish one enforced contract for the window command line's engagement controls ("no command active -> nothing shown below the input"), add missing toggle-button-group and select control kinds, and fix every playground that currently leaks persistent sliders/pickers outside that contract.
todos:
  - id: types
    content: Add toggleGroup/select EngagementControl kinds (neutral + ui types, digest, renderer bridging, EngagementControlView rendering)
    status: completed
  - id: invariant
    content: Extend enforcePlaygroundWindowEngagementInput to require sessionActive when control/controls is set
    status: completed
  - id: flow-sequence-dag-procedural
    content: Move layer-spacing/sibling-gap sliders from engagement.controls to windowMeasures() in flow, sequence, dag, procedural-2d, procedural-3d
    status: completed
  - id: writer
    content: Remove writer's duplicate leaked engagement controls; add tabSize to windowMeasures()
    status: completed
  - id: puzzle
    content: "Refactor puzzle-2d/3d/5d: move idle tool picker to options; use session-gated toggleGroup/select for brush candidates"
    status: completed
  - id: tests
    content: Extend existing tests in ui/react, framework/product/playground/core, and each touched playground; run nx test targets
    status: completed
isProject: false
---

# Command Line Input Elements Refactor

## Current architecture (confirmed)

The "window command line" is the shared `Engagement` overlay in [ui/react/index.tsx](ui/react/index.tsx) (component around line 14870), fed by a neutral `WindowEngagement` snapshot built per-controller in each playground's `*/core/js/index.ts`, bridged to the UI-facing `EngagementSpec` by [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) (`windowEngagementToGolden`, line 1183).

Today `EngagementControlView` (`ui/react/index.tsx:14803`) renders exactly three kinds: `slider`, `stepper`, `ring`. `ring` is used for two unrelated things: a genuine continuous/cyclic dial (CAD angle/length, `cad/core/js/index.ts:5302`) and a plain 3-way tool picker (puzzle 2d/3d/5d). There is no `select` kind at all on the command line (large enums currently only exist in the separate, always-visible "measures" rail).

Visibility today is inconsistent:
- CAD (`cad/renderer/js/index.tsx:4675,5642`) and puzzle 2d/3d/5d correctly tie `sessionActive`/`control` to an actual in-progress tool session.
- flow, sequence, dag, procedural-2d/3d unconditionally push 2 layout sliders into `controls` with `sessionActive: false` forever (`flow/core/js/index.ts:879-915`, and identical copies in `sequence`, `mathematical/graph/port/directed/dag`, `procedural/2d`, `procedural/3d`).
- writer duplicates font-size/line-height into both `controls` (always shown, leak) and `windowMeasures()` (correctly gated rail) (`writer/core/js/index.ts:507-599`).

## Decisions (confirmed with user)

1. Persistent layout/editor settings (layer spacing, sibling gap, writer font size/line height) move out of the command line entirely, into the existing `windowMeasures()` rail (same place `proximityMeasure`/LOD already live) — the command line will show **zero** controls when idle for these playgrounds.
2. Split the overloaded `ring` kind: keep `ring` only for continuous/cyclic dial values (CAD angle/length). Add a new `toggleGroup` kind (segmented `ButtonGroup`) for small, unordered enums.
3. Add a `select` kind (dropdown, mirrors `WindowMeasureSelect`) for large enums, wired to a real consumer where one naturally exists (puzzle brush-candidate picker) rather than left unused.
4. Enforce the contract structurally: `engagement.control`/`engagement.controls` may only be populated when `engagement.sessionActive === true`. This is checked by the existing `enforcePlaygroundWindowEngagementInput` (called by every playground already), extended rather than duplicated.

## Type & rendering changes

**[framework/product/playground/core/js/index.ts](framework/product/playground/core/js/index.ts)** (`WindowEngagement` region, ~line 215-402):
- Add `WindowEngagementToggleGroupControl { kind: "toggleGroup"; id?; label?; value?: string; options: readonly { id; label; disabled? }[]; disabled?; onSelect?: CommandDescriptor }`.
- Add `WindowEngagementSelectControl { kind: "select"; id?; label?; value?: string; placeholder?; items: readonly { id; value; label }[]; disabled?; onChange?: CommandDescriptor }`.
- Extend `WindowEngagementControl` union with both.
- Extend `windowEngagementControlDigest` to hash the two new kinds.
- Extend `enforcePlaygroundWindowEngagementInput` (keep the name; broaden its docstring/contract) to also throw when `control`/`controls` is non-empty while `sessionActive` is not `true`. This single change protects every existing call site (~20 playgrounds already call it via `enforceWindowKindsEngagementInput`/directly).

**[ui/react/index.tsx](ui/react/index.tsx)**:
- Mirror `EngagementToggleGroupControl`/`EngagementSelectControl` next to `EngagementRingControl` (~line 14337-14386), extend `EngagementControl` union.
- Extend `EngagementControlView` (~line 14803): `toggleGroup` renders `<ButtonGroup><ButtonGroupItem .../></ButtonGroup>` with selected item using the existing `interactiveActiveFillClass` highlight (same pattern already used for `options` at line 15141); `select` renders `<Select><SelectTrigger><SelectValue/></SelectTrigger><SelectContent>{items}</SelectContent></Select>` (same primitives already used for `WindowMeasureSelect`).
- Extend the existing engagement test block (~line 22700+) to cover the two new kinds and the `sessionActive` gating invariant — no new test files.

**[framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx)**:
- Extend `windowEngagementControlToGolden` (line 1115) and `engagementSpecControlMirror` (line 1150) with `toggleGroup`/`select` branches (dispatch `onSelect`/`onChange` through `CommandBus`, same shape as the existing `ring` branch).

## Fixing the leaks (move settings out of the command line)

For each of **flow**, **sequence**, **mathematical/graph/port/directed/dag**, **procedural/2d**, **procedural/3d** (all near-identical copies of the same pattern):
- Remove the `controls: [layerSpacing slider, siblingGap slider]` array from `windowEngagement()`.
- Add the same two sliders to the file's existing `windowMeasures()` (e.g. `flow/core/js/index.ts:864`, `sequence/core/js/index.ts:606`, dag's `windowMeasures()` at line 498, procedural's `flowWindowMeasures()`), following the exact shape already used by `proximityMeasure`/`lodMeasure`.
- `sessionActive` on these `windowEngagement()` results stays `false`; the enforced invariant now holds trivially since `controls` is gone.

For **writer** (`writer/core/js/index.ts:507-599`):
- Delete the duplicate `controls: [fontSize, lineHeight, tabSize]` array from `windowEngagement()` — `windowMeasures()` (line 507) already exposes font size / line height as measures; add `tabSize` there too (currently only in the leaked copy) as a `stepper`-shaped... note `WindowMeasure` has no stepper kind, so tabSize becomes a small `slider` measure (min 1, max 8, step 1), matching its existing engagement bounds.

## Fixing puzzle 2d/3d/5d (tool picker vs. session controls)

Currently the idle 3-way tool picker (select/brush/fill) and the dynamic brush-candidate picker both reuse `kind: "ring"` in `control`, and the tool picker is shown even when `sessionActive` is `false` (violates the new invariant).

For each of **puzzle/2d**, **puzzle/3d**, **puzzle/5d** core (`puzzle/2d/core/js/index.ts:1375-1399`, `puzzle/3d/react/index.tsx:8608-8650`, `puzzle/5d/core/js/index.ts:1280-1304`):
- Move the static tool picker (select/brush/fill) out of `control` into the existing `options` array (already a `ButtonGroup` with `pressed` state, already rendered unconditionally — the correct mechanism for a persistent mode selector, not a command parameter).
- Keep `control` populated only for the two genuinely session-gated cases (`sessionActive` already `true` in both):
  - Fill tool: `kind: "slider"` (unchanged).
  - Brush tool with placement candidates (`brushEngagementPossibles`/`brushPossibles`): use the new `kind: "toggleGroup"` when the candidate count is small (e.g. <= 6), otherwise `kind: "select"` — this is the real "large enum" consumer, since candidate count is dynamic and can grow.
- This satisfies "commands decide individually what UI element to use for what input" per the given guidance, and removes the last `sessionActive: false` + non-empty `control` combination in the codebase.

## Enforcement & verification

- Run the existing `nx` test targets for `ui/react`, `framework/product/playground/core`, `flow`, `sequence`, `mathematical/graph/port/directed/dag`, `procedural/2d`, `procedural/3d`, `writer`, `puzzle/2d`, `puzzle/3d`, `puzzle/5d`, and `cad` (regression check only, no behavior change expected there) via `bun`/`nx`, per repo conventions — extend existing test files, never add new ones.
- Manually confirm via the standalone playground dev runner (`framework/product/playground/dev`) for flow, sequence, writer, puzzle-2d/3d that: idle state shows no controls under the command bar; the moved settings now appear in the measures rail; brush/fill sessions still show the right control.

## Todos summary

- Add `toggleGroup`/`select` control kinds end-to-end (neutral type, UI type, renderer, bridging, digest, tests).
- Extend `enforcePlaygroundWindowEngagementInput` with the `sessionActive`-gates-`control(s)` invariant.
- Move flow/sequence/dag/procedural-2d/procedural-3d layout sliders from `engagement.controls` to `windowMeasures()`.
- Remove writer's duplicate engagement controls; add `tabSize` to its measures.
- Refactor puzzle-2d/3d/5d: tool picker to `options`, brush-candidate picker to session-gated `toggleGroup`/`select`.
- Run/extend tests for all touched packages; manual smoke test via playground dev runner.
