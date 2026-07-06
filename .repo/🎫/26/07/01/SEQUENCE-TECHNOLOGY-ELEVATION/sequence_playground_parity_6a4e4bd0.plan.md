---
name: Sequence Playground Parity
overview: Bring `sequence` to full styling and feature parity with the established `dag`/`flow` playgrounds (document/catalogue/inspection panels, reorganize + LOD toolbar, framework commands for every action), and fix the shared styling bug in `imperative/play` as well.
todos: []
isProject: false
---


# Sequence Playground Parity + Styling Fix

## Root cause of "styling doesn't work"

Every established play app (`flow/play`, `mathematical/graph/port/directed/dag/play`, `puzzle/2d/play`) has a `globals.css` that imports the shared Tailwind chain:

```css
@import "../../ui/react/globals.css";
@source "../../framework/product/playground/core";
@source "../../framework/product/playground/renderer/react";
@source "../../framework/product/platform/renderer/react";
@source "../../ui/react";
@source "../react";
@source "..";
@source ".";
```

[`sequence/play/globals.css`](sequence/play/globals.css) and [`imperative/play/globals.css`](imperative/play/globals.css) only contain a bare layout reset (`html, body, #root { height: 100%; margin: 0; }`) — no `@import`, no `@source`. Tailwind v4 never sees the utility classes used throughout `SequenceCanvas`/`ImperativeEditor` (`grid`, `rounded`, `border`, `text-xs`, `var(--background)`, ...), and design tokens/fonts from `@semio-tech/ui-styling` never load, so `bootstrapElementsSurfaceChromeDocument`'s `var(--base)`/`var(--foreground)` are undefined. `index.html` for both also lacks the semio favicon links and the `semio · <tech>` title convention used elsewhere.

## Scope decisions (confirmed with dev)

- **Sequence**: full parity with `dag`/`flow` — document panel, catalogue panel, inspection panel, reorganize + orientation + LOD toolbar, and a framework command for every mutating action (no more silent in-component-only state).
- **Imperative**: styling/shell fix only (same root cause), no new side panels/toolbar for imperative.
- **Not in scope**: VCS/undo-redo (`DocumentVcsStore`) for sequence — dag's undo/redo isn't a general playground requirement, it is out of scope for this pass; flagged here for a future ticket if wanted.

---

## 1. Styling & shell parity (sequence + imperative)

- [`sequence/play/globals.css`](sequence/play/globals.css) / [`imperative/play/globals.css`](imperative/play/globals.css): rewrite to the same chain as [`flow/play/globals.css`](flow/play/globals.css) (same repo depth, `../../`), keeping the `html,body,#root` reset appended after the `@source` lines.
- [`sequence/play/index.html`](sequence/play/index.html) / [`imperative/play/index.html`](imperative/play/index.html): add `SEMIO_FAVICON_HEAD_HTML`-equivalent `<link>` tags and rename titles to `semio · sequence` / `semio · imperative` (matching `semio · flow`, `semio · mathematical · graph · port · directed · dag`).
- [`sequence/play/package.json`](sequence/play/package.json) / [`imperative/play/package.json`](imperative/play/package.json): add `@tailwindcss/vite` and `@vitejs/plugin-react` to `devDependencies` to match `flow/play`/`dag/play`.

## 2. Sequence WASM completeness (`sequence/core/lib.rs`)

`SequenceHost` (native) already wraps `DagHost` as a public field (`self.dag`) exactly like `DagSession` does, so the same delegation pattern applies directly. Add to the `SequenceSession` wasm-bindgen impl:

```rust
#[wasm_bindgen(js_name = disconnectSteps)]
pub fn disconnect_steps(&self, from_id: &str, to_id: &str) -> bool {
    self.state.borrow_mut().host.disconnect_steps(from_id, to_id)
}

#[wasm_bindgen(js_name = lodScaleJson)]
pub fn lod_scale_json(&self) -> String {
    dag::dag_lod_scale_json()
}

#[wasm_bindgen(js_name = setAutomaticLod)]
pub fn set_automatic_lod(&self, enabled: bool) {
    self.state.borrow_mut().host.dag.set_automatic_lod(enabled);
}

#[wasm_bindgen(js_name = setForcedDrawLodLabel)]
pub fn set_forced_draw_lod_label(&self, label: &str) {
    self.state.borrow_mut().host.dag.set_forced_draw_lod_label(label);
}

#[wasm_bindgen(js_name = drawLodLabel)]
pub fn draw_lod_label(&self) -> String {
    self.state.borrow().host.dag.draw_lod_label().to_string()
}
```

This mirrors `mathematical/graph/port/directed/dag/lib.rs`'s `DagSession` LOD bindings exactly (`self.state.borrow_mut().host.set_automatic_lod(...)`, etc.), just delegating one field deeper into `.dag`. Add inline `#[cfg(test)]` cases exercising `disconnect_steps` round-trip and `lod_scale_json` non-empty output.

## 3. Catalogue schema enrichment (shared by imperative + sequence)

Today `catalogue_json()` in [`imperative/module/core/lib.rs`](imperative/module/core/lib.rs) only emits `kind/name/abbreviation/icon/summary` — no parameter metadata, forcing both `StepParamForm` (imperative/react) and any new sequence inspector to hardcode per-kind fields. `OperatorInfo.inputs: Vec<ChannelSpec>` already carries `name` + `code` (`"S"` string, `"N"` number, `"*"` wildcard) per parameter. Extend the JSON:

```rust
"inputs": info.inputs.iter().map(|c| serde_json::json!({ "name": c.name, "code": c.code })).collect::<Vec<_>>()
```

- Update [`imperative/core/index.ts`](imperative/core/index.ts) `ImperativeCatalogueItem` type: add `readonly inputs: readonly { readonly name: string; readonly code: string }[]`.
- Refactor `StepParamForm` in [`imperative/react/index.tsx`](imperative/react/index.tsx) to render fields from `catalogue` lookup by `step.kind` (`code === "N"` → number input, else text input) instead of the current hardcoded 4-kind switch. Removes a stub flagged during investigation.

## 4. `SequenceCanvas` rewrite (`sequence/react/index.tsx`)

Bring props in line with `DagCanvasProps`, delegating catalogue/inspector UI out to framework side panels and toolbar/LOD to props (mirrors `mathematical/graph/port/directed/dag/react/index.tsx` lines ~375-577 exactly, including the `syncLodMode`/`reportDrawLod`/reorganize-effect pattern):

```typescript
export interface SequenceCanvasProps {
  readonly fixtureJson?: string;
  readonly className?: string;
  readonly reorganize?: SequenceReorganizeRequest; // { epoch, optionsJson }
  readonly runRequest?: SequenceRunRequest; // { epoch }
  readonly automaticLod?: boolean;
  readonly lod?: DagDrawLodKind;
  readonly selectedStepIds?: readonly string[];
  readonly onFixtureChange?: (fixtureJson: string) => void;
  readonly onSelectionChange?: (ids: readonly string[]) => void;
  readonly onLodChange?: (lod: DagDrawLodKind) => void;
  readonly onRunResult?: (result: RunResult) => void;
}
```

- Remove the inline catalogue "+ button" header and the inline `StepParamForm` inspector aside (they move to framework side panels, section 6).
- Keep "Compiled Text" and "Effect Log" asides inline (no dag equivalent to mirror; these stay as real-time canvas readouts).
- Add `session.setSelection(ids)` on `selectedStepIds` prop change, and read `session.selectedNodeIds()` after pointer-up to call `onSelectionChange` — this closes the canvas↔panel selection bidirectionality gap that the DAG technology itself doesn't have yet (its WASM has no `setSelection`/`selectedNodeIds` bridge), which is a straightforward add here since `SequenceSession` already exposes both.
- Wire `runRequest.epoch` similarly to `reorganize.epoch`: on change, call `session.run()`, parse `RunResult`, replay via `performImperativeEffects` (reuse from `@semio-tech/imperative-core`), update compiled text/effect log locally, and call `onRunResult`.
- Re-export `DagDrawLodKind`, `dagLodCanvasProps`, `dagPlayLodTiers` from `@semio-tech/dag-react` for reuse (no need to duplicate LOD tier plumbing).

## 5. `SequencePlayController` rewrite (`sequence/play/index.ts`)

Mirror `DagPlayController` field-for-field where applicable, replacing DAG's `DocumentVcsStore` with the existing plain `fixtureJson` string (no VCS, per scope decision):

- Fields: `fixtureJson`, `orientation`, `layerSpacing`, `siblingGap`, `reorganizeEpoch`/`reorganizeOptionsJson`, `runEpoch`, `lodMode`/`lodModeByInstance`, `effectiveLod`, `selectedStepIds`, `interactionRevision`, `snapshotListeners`, `engagementInput`.
- Commands handled by `run()`: `setFixtureJson`, `addStep` (parse fixture, push a new `StepWidgetV1` at an incremented `x` offset, `emit()`), `disconnectSteps` (`{ from, to }` — patches fixture edges array), `setSelection`, `reorganize`, `setOrientation`, `setSpacing`, `setLodMode`, `setEffectiveLod`, `run` (bumps `runEpoch`), `engagementInput`/`engagementSubmit` (aliases: "reorganize"/"layout", "lr"/"left", "tb"/"top", plus "run").
- `getReorganize()`, `getRunRequest()`, `lodModeForScope()`, `getSelectedStepIds()`, `getInteractionRevision()`, `subscribeSnapshot()` accessors, matching `DagPlayController`'s shape exactly.
- `rebuildShellMode()` sets `mainMode.tools` (toolbar) and `mainMode.windowKinds` (with `windowMeasures()` for LOD select and `windowEngagement()` for the reorganize/orientation/spacing panel), calling `enforcePlaygroundWindowEngagementInput` like DAG does.

Toolbar (`buildSequencePlayToolbarTools`), mirroring `buildDagPlayToolbarTools`:

```typescript
toolCollection("execution", "play", [
  { kind: "button", id: "sequence.run", label: "Run", iconId: "play", controllerId, command: "run" },
]),
toolCollection("layout", "layout-grid", [
  { kind: "button", id: "sequence.reorganize", label: "Reorganize", iconId: "refresh-cw", controllerId, command: "reorganize" },
  layoutToggle("sequence.orientation.lr", "Left to right", "leftRight"),
  layoutToggle("sequence.orientation.tb", "Top to bottom", "topBottom"),
])
```

## 6. Document / Catalogue / Inspection tree builders (`sequence/play/index.ts`)

Mirror `buildDagPlayDocumentTree` / `buildDagPlayCatalogueTree` / `buildDagPlayInspectorTree` exactly:

- `buildSequencePlayDocumentTree(fixtureJson, selectedStepIds)`: "Steps" section (label = kind, `command: setSelection`) + "Edges" section (label = `${from} → ${to}`, `command: disconnectSteps` on click — since there's no dedicated delete affordance otherwise, clicking an edge in the document disconnects it; document this clearly in the tab's item description e.g. `"click to disconnect"`).
- `buildSequencePlayCatalogueTree()`: one section per catalogue section (currently just "Actions"), each item `command: addStep({ kind })` — unlike DAG's display-only catalogue, this is sequence's actual step-creation entry point now that the inline "+ button" header is removed.
- `buildSequencePlayInspectorTree(fixtureJson, selectedStepIds)`: for the selected step, look up its catalogue entry's `inputs` (from section 3) and emit one field per input (`code === "N"` → number field, else text field) via `dagPlayInspectorTextField`/`NumberField`-equivalent helpers, plus a base group (id, kind — read-only) and a "Remove step" action row. Falls back to "Select a step in the document." when nothing is selected, matching DAG's empty-state text exactly.

## 7. Framework renderer wiring (`framework/product/playground/renderer/react/index.tsx`, `🔖SequencePlayHost` region)

Mirror `🔖DagPlayHost` exactly:

- Add `SequencePlayDocumentPanelDefinition`, `SequencePlayCataloguePanelDefinition`, `SequencePlayInspectionPanelDefinition` (`PureSidePanelTabDefinition` subclasses using `CallbackTreePanelDefinition` + `uiTreeNodeToTreePanelConfig`, same as the three `DagPlay*PanelDefinition` classes).
- `useSequencePlayInteractionRevision` hook (copy of `useDagPlayInteractionRevision`) so panels refresh on canvas interaction.
- `SequencePlayPaneSurfaceHost` gains `reorganize`, `runRequest`, LOD props (`dagLodCanvasProps` reused), `selectedStepIds`, `onSelectionChange`, `onLodChange` wired to the new controller commands — same shape as `DagPlayPaneSurfaceHost`.
- `SequencePlayInner` passes `augmentPanelTabs={{ workbench: [document, catalogue], details: [inspection] }}` to `PlaygroundView`, exactly like `DagPlayInner`.

## 8. Tests

- `sequence/core/lib.rs`: inline tests for `disconnect_steps` WASM binding and `lod_scale_json`/LOD label round-trip (extend existing `#[cfg(test)]` module).
- `imperative/module/core/lib.rs`: extend the existing catalogue test to assert `inputs` entries are present with correct `name`/`code`.
- `sequence/react/index.tsx`, `sequence/play/index.ts`: extend existing `import.meta.vitest` blocks for the new props/commands (selection round-trip, addStep/disconnectSteps commands, reorganize/run epoch bumps).
- `imperative/react/index.tsx`: extend vitest for the catalogue-driven `StepParamForm`.
- Run `cargo test` for `sequence_core` + `imperative_module_core`, `bun test` for touched TS packages, and `nx dev`/`build` smoke checks for `sequence-play` and `imperative-play` to visually confirm styling now matches `dag-play`/`flow-play`.

## 9. Ticket

Reopen `.repo/🎫/26/07/01/IMPERATIVE-AND-SEQUENCE-TECHNOLOGIES` (already covers this work), do the implementation, then close with an updated summary listing every file touched in this pass.
</plan>
<todos>[{"id":"ticket-reopen","content":"Reopen IMPERATIVE-AND-SEQUENCE-TECHNOLOGIES ticket for this follow-up pass"},{"id":"styling-fix","content":"Fix globals.css/index.html/package.json Tailwind wiring for sequence/play and imperative/play"},{"id":"sequence-wasm","content":"Add disconnectSteps/lodScaleJson/setAutomaticLod/setForcedDrawLodLabel/drawLodLabel to SequenceSession WASM"},{"id":"catalogue-schema","content":"Extend imperative catalogue_json with per-item inputs (name/code); update TS types and refactor StepParamForm to be catalogue-driven"},{"id":"sequence-canvas","content":"Rewrite SequenceCanvas props (reorganize, runRequest, LOD, selection) and remove inline catalogue/inspector UI"},{"id":"sequence-controller","content":"Rewrite SequencePlayController with full command surface, toolbar, LOD/engagement window measures, mirroring DagPlayController"},{"id":"sequence-panels","content":"Add buildSequencePlayDocumentTree/CatalogueTree/InspectorTree in sequence/play/index.ts"},{"id":"renderer-wiring","content":"Wire SequencePlay document/catalogue/inspection panel definitions into 🔖SequencePlayHost region"},{"id":"tests","content":"Extend inline Rust/TS tests for all new WASM methods, commands, and catalogue fields; run cargo test, bun test, nx dev/build smoke checks"},{"id":"ticket-close","content":"Close ticket with summary and full list of files touched"}]</todos>
