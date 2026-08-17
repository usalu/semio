# UI Label Value Contract Retention Audit

## Classification

`UiLabel/🟦️component.tsx` is a cycle-free branded display-text value contract. It is semantically distinct from the `Label` inference/presentation umbrella and must remain independently owned.

## API and Responsibility

- `UiLabel`: branded `string` representing display-ready text.
- `uiDataLabel(value: string)`: the explicit mint point for runtime filenames, counts, user content, demo data, and manifest/wire values.
- `uiLabelBrand`: private compile-time brand.

The implementation has no React, i18n, driver, or rendering dependency. Its runtime function is the intentional boundary cast into repository-owned display text.

## Consumer Evidence

Active direct UI consumers include PanelTabBar, Input, Label, VirtualFileSystem, Tree, Textarea, ContextMenu, Toggle, Canvas, Navbar, Command, ActionGroup, Table, and Icons. Independent product terminals include OS Shell, Interpreter, ShellHelpers, World3dHost, and ChromePanels.

`uiDataLabel` itself is actively called from seven independent UI/product components. Unused OS package bridge imports do not count.

VirtualFileSystem calls `uiDataLabel` during module-level fixture initialization. Direct leaf import is required; routing through the React barrel would recreate an initialization-order cycle.

## Disposition

- Retain `UiLabel` and `uiDataLabel` at the standalone leaf.
- Do not merge with `Label`: `Label` resolves translation/driver/control policy and renders UI, whereas this leaf owns the value type it produces.
- No zero- or one-consumer facet exists.
- No external-library type leaks from the public API.
- No source lease is warranted.

## Baseline SHA-256

- UiLabel: `d15a4facbb4400758f6d1b81adcaacba483acb57833a22623bc495787ff63c04`
- Label: `754e706044acfa16efccd7f4c9330c3c3064550cbe60087654105a87544fa301`
- React barrel: `a9a764971875336ed637b8be0ec1dae23150dfce09985ddf7cd5d69cafc774f6`
- VirtualFileSystem: `3c1ce5cfc96b49967d1f9a1050fea59f0d91385e7198bc7b9b1857aabd9c7540`
