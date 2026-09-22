# Virtual Tree Window Extent Audit

## Scope

Read-only source audit on 2026-09-21. No build, test, browser action, or production edit was run.

## Current contract

[`TreeWindow`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:465>) contains only `total` and `offset`. It appears on both `TreeSectionProps` and `TreeItemProps`, so a window can be nested. Its generated TypeScript schema and typed visitor have the same two fields: [schema](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs:865>) and [visitor](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧾️typed/🦀️.rs:27>). It defines a logical slice, but no row geometry, row kind, or uniformity guarantee.

That absence is significant: a `TreePresentation::Compact` tree has deliberately variable materialized row heights. The native retained metric starts a compact text row at `SIZE_TINY × 1.5`, then raises it for an Input/Select or another control in [`TreeRowMetrics::for_item`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:163>). React now has the matching `auto` height/min-height compact scope in [`Tree`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:4014>). Compact Window Options is finite and publishes `window: None` for its section in [the WGPU producer](</Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4381>), so it does not presently rely on virtual geometry.

The implementation chooses a fixed *unmaterialized* spacer pitch, rather than proving every materialized row has that height:

- React materialized rows are read from their actual top positions; only spacer bands use the fallback pitch ([`treeWindowRowIndexAt`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:845>)). Its spacers always multiply the standard `treeRowHeightPx` ([`renderTreeWindowSpacer`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:1061>)), and the observer passes that same one global pitch to every container ([`useTreeWindowObserver`](</Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:1708>)).
- The native generic Tree target has the same fixed-standard policy: [`tree_window_pitch`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs:84>) multiplies `TREE_ROW_HEIGHT` for both leading and trailing spacers. It has no scroll/window-report observer, which its own source calls out at [lines 92–97](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs:92>).

The early comment that `treeRowHeightPx` is the one exact height for every row is stale once the compact CSS variables make rows variable ([`Tree`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:212>)). The later window algorithm is the accurate description: materialized rows may vary, while a spacer is a fixed fallback band.

## Recommended field and unit

Add a required, closed `TreeWindowRowExtent` field to `TreeWindow`, for example:

```rust
pub enum TreeWindowRowExtent {
    Standard,
    CompactText,
    CompactSmallControl,
    CompactControl,
}

pub struct TreeWindow {
    pub total: u32,
    pub offset: u32,
    pub row_extent: TreeWindowRowExtent,
}
```

The field means **the closed, unmaterialized row extent used for the two spacer bands**. It does not assert that an open materialized group has no descendants; React should keep measuring those rows exactly. A producer may put a window only around a list whose closed logical entries share its declared extent. This makes a mixed compact list explicit: split it into separate windowed containers, or keep the finite list unwindowed as Window Options already does.

This should be a new enum, not a raw `f32` or an existing style token:

- A raw `f32` conflicts with the contract's token rule and would remove `Eq` from the current `Copy + Eq` `TreeWindow`/typed machinery.
- [`SpaceToken`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📐️layout/🦀️.rs:13>) is the nearest existing layout unit but cannot name the established 24 px standard row or the 14.4 px compact text row: its available compact-root multiples are `0, 1, 2, 4, 6, 8, 12` UI-spacing steps. [`SizeToken`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎨️style/🦀️.rs:35>) is an appearance size, not a row-geometry contract.
- The enum lets each renderer resolve its active-theme metric: Standard to `theme.tree_row_height` / `--size-workbench`; CompactText to its compact text minimum; CompactSmallControl to `theme.control_height_small` / `--size-small`; CompactControl to `theme.control_height` / `--size-medium`. The existing fixture pins the standard pair at 24 px from the shared styling metric ([fixture](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️tree-row-rects/🔣️.json:12>)).

`TreeWindowRowExtent` can derive `Copy`, `Eq`, `Serialize`, `Deserialize`, `ToValue`, and `FromValue`, so `TreeWindow` retains its current value semantics. Make `rowExtent` required in the generated schema instead of silently defaulting it: this is a greenfield wire contract and every window producer must state its fallback geometry.

## Required propagation

1. Add the enum and field in [the contract](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:465>), its typed visitor ([line 27](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧾️typed/🦀️.rs:27>)), and generated schema ([line 865](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs:865>)). The manifest reexports the generated type at [line 277](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:277>).
2. The only common production contract stamp is [`TreeWindows::window`](</Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6336>). Its current panel helpers produce Standard windows, so they should set `Standard` explicitly. The host request remains `{offset, rows}`; extent belongs to the published document, not `ViewModel.tree_windows`.
3. Carry the field through the native legacy mirror: [`UiTreeWindow`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:2390>), [`tree_window`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:589>), and the panel projection's section/item copies ([lines 5118 and 5153](</Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:5118>)). The settings-theme editor is another live legacy constructor: `theme_window_rows`, `theme_leaf_section`, metrics, and appearances at [lines 29068–29203](</Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:29068>). Those are Standard extent today.
4. Add the field to React `TreeDataWindow` ([lines 755–767](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:755>)), mirror it as a DOM token attribute with the existing window identity attributes, and resolve the spacer height per container. `TreeWindowContainerMeasure` needs that resolved pitch ([lines 804–813](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:804>)); then `treeWindowRowIndexAt` and the viewport request functions use the container's pitch instead of one global argument. This preserves bounded work: the body already bounds materialized nodes and its window-container ledger.
5. Resolve the same enum in the native `tree_window_pitch` function before it produces either spacer band. Its current `TREE_ROW_HEIGHT` multiplication is the exact place a compact virtual-window mismatch would occur ([line 99](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs:99>)).

Every current `TreeWindow { ... }` literal is compiler-visible. Besides the two production paths above, the required fixture/test updates are in the contract component unit, runtime reconcile unit, plugin panel-kit and plugin-builder-contract units, and Shell WGPU test constructors. The complete literal census is short and was taken from source at this audit time; no dynamic fallback or compatibility default is needed.

## Fail-first laws

Use a new neutral `tree-window-row-extent` fixture and schema rather than extending the finite Window Options fixture:

1. A Standard window with `total: 10`, `offset: 4`, one materialized row has leading/trailing spacer geometry of `4 × standard` and `5 × standard` in both targets.
2. A CompactSmallControl window with the same slice and a Select materialized row uses its declared small-control spacer extent, while the materialized row remains read from actual geometry.
3. A CompactControl window with a Checkbox uses the control extent. A CompactText window may not carry that control row: the producer/document admission fails before either renderer receives an ambiguous virtual list.
4. A materialized open nested child retains the current exact-DOM-row handling. Its row need not be equal to the spacer extent, and a nearby off-screen logical entry still receives the declared closed extent. This protects the existing oscillation avoidance in [`treeWindowRowIndexAt`](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:845>).

The existing React window component laws are at [`Tree` component tests](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx:388>) and the interpreted host window laws are at [`Interpreter` tree-window tests](</Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🪟️tree-windows/🟦️.tsx:123>). The native shared metric fixture is [tree-row-rects](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️tree-row-rects/🔣️.json:12>). Those are the narrowest places to add the language-neutral and target-specific laws.

## Decision

Do not make Compact globally uniform, and do not let either renderer infer a spacer pitch from currently materialized controls. A required, finite `TreeWindowRowExtent` makes the existing fixed-band virtualisation model explicit, keeps finite Compact trees variable, retains `Eq`/bounded value semantics, and gives both targets the same source of geometry.
