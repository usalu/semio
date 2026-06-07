---
name: Composable Window Option Trees
overview: Make window options a generic, recursively-composable tree in the framework `WindowMeasure` model, render it with a new ultra-compact collapsible rail, and reorganize every play app (3D/2D/5D/CAD) into grouped trees (e.g. Brush -> Tolerance + Distribution -> Objects/Vortices).
todos:
  - id: ticket
    content: Read repo://goals and open ticket 'Composable Window Option Trees' under the best goal
    status: cancelled
  - id: core-model
    content: Add WindowMeasureGroup to framework/core WindowMeasure union (recursive tree)
    status: completed
  - id: ui-render
    content: Add UI group node + compact UIWindowMeasureGroup collapsible renderer + rail styles; recurse in UIWindowMeasures
    status: completed
  - id: mapping
    content: Make windowMeasuresToGolden recurse into group children
    status: completed
  - id: producers
    content: Reorganize 3d/2d/5d/cad windowMeasures builders into grouped trees (Brush->Tolerance+Distribution->Objects/Vortices)
    status: completed
  - id: tests
    content: Extend inline vitest suites for tree producers + mapping recursion; runtime-verify compact rail
    status: completed
isProject: false
---

## Goal

Today "window options" are the right-edge **measures rail**: a flat `WindowMeasure[]` (select/slider/toggle) per window, mapped by `windowMeasuresToGolden` and rendered as flat floating tiles by `UIWindowMeasures`. Make the model a **recursive tree** at the framework level, render it as compactly as possible, and reorganize all producers.

Target shape (Puzzle 3D), matching the request:

```
- LOD            (group)
- Select         (group)
- Brush          (group)
  - Tolerance    (slider)
  - Distribution (group)
    - Objects    (group -> object-kind weight sliders)
    - Vortices   (group -> vortex-kind weight sliders)
```

## 1. Ticket setup

- Read `repo://goals` (MCP `project-0-semio-repo`), then open a new ticket via `ticket_open` titled "Composable Window Option Trees" associated with the most fitting goal (likely a framework/ui goal). The closed `🎫/26/06/02/HIDE-CATALOG-VORTICES-IN-TREE-BY-DEFAULT` is a different surface (kinds side-panel tree), so a new ticket is appropriate. Keep any scratch files inside the ticket folder.

## 2. Core model: make `WindowMeasure` a tree

In [framework/core/index.ts](framework/core/index.ts) `//#region 🔖WindowMeasure` (~67-99), add a group node and fold it into the union:

```ts
export interface WindowMeasureGroup {
  readonly kind: "group";
  readonly id: string;
  readonly label: string;
  readonly defaultOpen?: boolean;
  readonly children: readonly WindowMeasure[];
}
export type WindowMeasure = WindowMeasureSelect | WindowMeasureSlider | WindowMeasureToggle | WindowMeasureGroup;
```

This makes the framework model itself the "composable tree"; leaves stay unchanged so every existing control keeps working.

## 3. UI model + new compact rendering

In [framework/product/platform/renderer/react/index.tsx](framework/product/platform/renderer/react/index.tsx):

- Extend `UIWindowMeasure` union (~300) with `{ kind: "group"; id; label; defaultOpen?; children: UIWindowMeasure[] }` (alongside existing `section`/`separator`, which groups now supersede).
- Add a `UIWindowMeasureGroup` component rendering a minimal disclosure: a single-line header (rotating `ChevronRightIcon` + tiny uppercase label reusing `windowMeasureSectionClass`) toggling local `useState(defaultOpen ?? true)`, with children indented by a thin per-depth pad and rendered recursively. Built on the already-wrapped `@radix-ui/react-collapsible` (`CollapsiblePrimitive` in `ui/react`) so we stay behind the existing interface.
- Update `UIWindowMeasures` (~572) to dispatch `kind: "group"` to `UIWindowMeasureGroup` (recursion); all leaf cases unchanged.
- Compactness: deep groups (`Distribution`, `Objects`, `Vortices`) default collapsed; nested leaf tiles drop their border (rely on indent). Add compact rail style helpers in [ui/react/index.tsx](ui/react/index.tsx) near `windowMeasureTileClass` (~2236-2263): a `windowMeasureGroupHeaderClass`, `windowMeasureGroupChildrenClass` (small `pl`, thin indent guide), and a borderless nested tile variant. Keep `windowMeasuresRailWidthClass` width.

## 4. Recursive mapping

Update `windowMeasuresToGolden` (~2530) to handle `kind: "group"` by recursing into `children` and emitting the UI group node; leaf mapping (select/slider/toggle) unchanged.

## 5. Reorganize all producers into trees

Rewrite each `windowMeasures` builder to emit grouped trees (regions preserved):

- [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts): wrap `lodMeasures()` -> "LOD" group, `selectionMeasures()` -> "Select" group, and `brushMeasures()` (~953) -> "Brush" group with the tolerance slider as `Tolerance` and a `Distribution` group whose children are `Objects` (object `kindWeightMeasures`) and `Vortices` (vortex `kindWeightMeasures`) subgroups.
- [puzzle/2d/play/index.ts](puzzle/2d/play/index.ts) `windowMeasuresForPane()` (~744): "LOD" group + "Brush" group -> Flush slider + "Distribution" -> Nodes/Handles weight subgroups.
- [puzzle/5d/play/index.ts](puzzle/5d/play/index.ts) `lod2dMeasure()`/`lod3dMeasures()` (~223-262): wrap into "LOD" groups per window.
- [cad/js/renderer/play/index.tsx](cad/js/renderer/play/index.tsx) `computeMeasureForPane()`/`transformMeasureForPane()` (~569-584): wrap into "Compute"/"Transform" groups.

`WindowKindRuntime.measures` and app-runtime `measures?` fields in [framework/product/platform/core/index.ts](framework/product/platform/core/index.ts) need no signature change (still `readonly WindowMeasure[]`, now tree-capable).

## 6. Tests

Extend the existing inline `import.meta.vitest` suites (no new files):

- [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts) (`import.meta.vitest` ~2125): assert `windowMeasures()` returns a `Brush` group containing `Tolerance` and a `Distribution` group with `Objects`/`Vortices` subgroups, and that `Distribution` defaults collapsed.
- Add/extend inline vitest in the platform renderer for `windowMeasuresToGolden` recursion (group -> nested UI group) if a vitest block exists there; otherwise assert mapping via the 3d suite.
- Validate at runtime with the dev playground per repo rules (confirm the compact collapsible rail renders and toggles via `[DEBUG]` logging before removing logs).

## 7. launch.json / scripts

No new executables; verification runs through existing `nx`/`launch.json` test + dev targets. No `project.json`/`package.json` script changes expected.
