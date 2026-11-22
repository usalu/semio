# Window layout unification: design and status

## Verified starting point

Four shapes exist, not three:

1. Contract `WindowLayoutNode` / `WindowLayout` (`🖱️ui/🧬️contract/🖼️presentation/📐️layout/🦀️.rs`): tagged `window` / `stack` / `split{axis}`. No renderer consumer.
2. Declaration `WindowLayout` (`🖱️ui/📜️declaration/📐️layout/🧭️layout-contract/🪟️window-layout-tree/🦀️.rs`): `kind` strings `row`/`column`/`stack`, untagged unions with hand-written `ToValue`. Used by about 150 Rust files (wgpu dock, shell persistence, TUI, plugin builders, about 60 plugin crates through `create_default_layout`).
3. wgpu in-memory `DockNode` (`Row/Column/Stack`, `f32` sizes) converted from/to (2) in `🛰️Dock/🎯️targets/🧊️wgpu/📐️layout-conversion-and-drop-zones`.
4. React runtime shape (`🖱️ui/🔨️modules/🗺️window-layout/🟦️.ts`): `row`/`column`, `id`, `activeId`, percent sizes; converted from/to a TS copy of (2) (`windowKindId`, `instanceId`, in onboarding/manifest/config/platform) by `ShellHelpers/🛣️route-and-chrome` (`modeLayoutNodeToFramework`, `collectFrameworkLayoutWindowSeeds`, an `extraWindowInstances` id-to-kind map).

## Decision: the contract model is THE schema-owned layout

Reasons: one recursive internally tagged enum (no untagged unions, no hand-written `ToValue`), already in the schema-owned contract crate with a TS projection, `Axis` already shared with `LayoutSpec`, and it carries both `windowKindId` and `instanceId`, which removes React's id-to-kind side map.

Refinements made to it (the contract had no consumers, so no migration cost):

- Stack field `activeWindowKindId` becomes `activeWindowId` (it always held a window id: `instanceId ?? windowKindId`). React's `activeId` and the declaration's alias disappear.
- Wire names are camelCase everywhere; the checked-in TS projection and `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` had snake_case fields for this type and are corrected.
- `size` is a fraction of the parent split (0, 1]. React converts to percent only at the `Resizable` boundary; wgpu `f32` is a view-only type.
- `maximize` and active-stack remain view state outside the layout (a path into it), as in `DockState.maximized_stack`; the corpus path queries define path validity.
- Even layout: one stack for one window, otherwise a horizontal split of single-window stacks sized `1/n`, active on their window (the declaration/wgpu variant; React's was the same without sizes/active).

Schema: `📐️layout/🧬️schema/🔣️.json` (draft-07, `window-layout.v1`) holds `WindowLayout`, `DropZone` and `Operation`. Stricter than the Rust types: stack children are windows, split children are stacks/splits, splits have at least 2 children.

## Shared corpus and reference operations

- Corpus: `📐️layout/🧪️tests/🧫️fixtures/🔣️.json` with 24 operations (setActive, closeWindow incl. collapse, reorderTab, setCorner, applySizes, reconcile, dropWindow to tab/split/root incl. external new window and invalid target, dropStack to tab/root), 6 queries (stackPathOf, collectWindowIds, readAt) and 3 builders (even). Expected results were derived by hand from the React semantics, not generated.
- Operations port React's `🎨️Canvas/{🗺️layout,📐️corners,🎯️drop}` exactly (including: a stack ends path walking, drop paths address the layout after removal, collapse hoists a lone child keeping its own size else the split's size, invalid drop targets are no-ops).
- Rust twin: `📐️layout/🪟️window-layout-ops/🦀️.rs` (`apply_window_layout_op`, serde `WindowLayoutOp`), test `📐️layout/🧪️tests/🪟️window-layout-corpus/🦀️.rs`.
- TS twin: `📐️layout/🟦️.ts` (exported as `@semio-tech/ui-contract/presentation/layout`, plus `/schema.json` and `/testing/fixtures.json`), test `📐️layout/🧪️tests/🪟️window-layout-corpus/🟦️.ts` (also validates every corpus layout and operation with Ajv against the schema).

## Consumption plan (how React and wgpu move over)

React: `Canvas/{🗺️layout,📐️corners,🎯️drop}` collapse into the contract TS functions; Canvas elements read ids with `windowIdOf`, corners with `windowCornerOf`; `🔨️modules/🗺️window-layout` and the onboarding/manifest/config/platform TS copies are deleted; `route-and-chrome` conversions and `extraWindowInstances` map disappear; Canvas path strings `"0.1"` become index arrays; sizes lose the `*100`.

wgpu/Rust: delete `declaration::layout::window_layout_tree` (`WindowLayoutRoot/Child/AxisNode/StackNode/WindowNode`, `create_*`, `even_window_layout`, `NamedLayout` moves next to the contract), re-point `semio_ui_declaration::layout` to the contract types, convert `dock_from_window_layout` / `dock_node_to_layout_root` and `DockState` edits to the contract tree (or call `window_layout_ops`), migrate the plugin builders (`create_default_layout` callers), TUI (`⌨️tui/📏️layout`, 98 hits) and persisted config layouts in one pass.

## Status

### Landed
Contract rename (`activeWindowId`), schema, TS types and operations, Rust operations, shared corpus, both corpus tests, TS projection and generated-TS field-name fixes, package exports, README, ui-react vitest include.

### Verified
- TS operations against the corpus: a throwaway `bun` script (no repo) ran all 24 operations, 6 queries and 3 builders: 0 mismatches. This also confirms the hand-derived expected values.
- NOT verified: the Rust corpus test and the vitest run (with the Ajv schema validation). `repo cargo ... test -p semio-ui-contract` is refused (the crate is not a workspace member; it is tested through `repo run @semio-tech/ui-contract-rs:test -- window_layout`), and that run was stopped after about 40 minutes in the project-graph phase on the loaded host (no compile reached). The Rust ops file and test were never compiled. Run next: `repo run @semio-tech/ui-contract-rs:test -- window_layout` and `repo run @semio-tech/ui-react:test -- <corpus test path>`.
- No `check` was run on other crates: nothing else was edited besides the contract crate (new module, renamed field with no other Rust users per grep) and string/TS fixes.

### Not done (honest)
React and wgpu do not yet consume the contract model; no redundant model was deleted, no persisted layout migrated. Migrating without adapters is a single atomic change across about 150 Rust files, about 45 TS files and ~60 plugin builders (see the consumption plan); it could not be completed or verified safely on this host. Until then the contract ops are covered by the corpus only. The generated-TS fix was hand-applied; re-run `repo run @semio-tech/ui-contract-rs:generate` to confirm no drift.
