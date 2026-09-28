# Display Taxonomy Order

## Authority

React `worldProjectionTemplatesToTreeItems` pre-reverses every taxonomy level because the Display Windows panel is anchored at bottom-left and its production `Tree direction="up"` reverses siblings at every level. `buildDisplayWindowsTree` publishes the pre-reversed templates first and the plain window-kind row last. The resulting collapsed visual order is Puzzle 3D, Parallel, Perspective. When expanded, an up-flow branch places its children above the branch row: Orthographic, Axonometric's variants then Axonometric, Oblique's variants then Oblique, then Parallel; Perspective's variants then Perspective.

## Neutral Contract and React Oracle

The shared `window-lifecycle-template-drag` schema and fixture now include `displayResolvedOrder`: the `up` direction, the real `puzzle3d-main` kind id, and all sixteen expected top-to-bottom row ids. The production React oracle resolves `createFrameworkDisplayPanelTabs`, mounts the resulting config through the actual `Tree direction="up"`, expands the four real taxonomy branches through their accessible disclosure buttons, and compares the mounted `treeitem` order with that neutral list. It uses the existing Ajv schema validation and Testing Library/jsdom host.

## WGPU Gap

Current WGPU authors projection templates in declaration order and puts the plain kind first. Its retained BottomStart Tree reverses those siblings again, so the live panel displays Perspective, Parallel, Puzzle 3D and reverses nested children. The bounded repair is to reverse `world_projection_template_rows` immediately before each recursive return and compose the Display section as pre-reversed template rows followed by the plain kind. `WORLD_PROJECTION_TEMPLATES`, selection ids, drag payloads, and the separate in-window Projection pane remain unchanged.

WGPU23 physical acceptance confirms the current collapsed visual order is Perspective, Parallel, plain Puzzle 3D, then the Puzzle 3D section header at the bottom; accepted accessibility exposes the row order as plain kind, Parallel, Perspective. A physical chevron press expands Parallel and paints its three children. The same branch's accessibility `Expand` secondary action produces no change, while the Orthographic leaf incorrectly advertises collapsed/Expand despite having no children. Those disclosure bridge defects are recorded for a separate accessibility packet; this ordering packet must not infer physical order from accessibility enumeration.

## Tree Disclosure Accessibility

The shared Tree disclosure fixture is now schema-first at `🧰️framework/🔨️modules/🖱️ui/🧬️schema/♿️tree-disclosure/🔣️.json`. It distinguishes localized branch rows from localized leaves: branches expose an expanded state and a named disclosure button; leaves remain `treeitem`s but expose neither. The actual React `TreeItem` oracle validates the schema, drives Enter and Space on every branch disclosure, and checks that both English and German Orthographic leaves have no `aria-expanded` and no disclosure button.

The native AccessKit fixture names the retained counterpart: an expanded branch supports `Collapse`, a collapsed branch supports `Expand`, and a leaf supports neither. The neutral projection now emits `expanded` only when a Tree item has a semantic child; `defaultOpen` alone cannot make Orthographic a disclosure. The native bridge declares the state-matching platform action, verifies that the accepted platform node supports the requested action, and maps only that accepted `Expand`/`Collapse` request to the existing neutral activation. Contradictory requests and leaf requests are inert, which prevents a delayed secondary action from blindly inverting newer disclosure state. Production source is coherent; Rust verification remains pending.

Focused React command: `NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long bun nx run @semio-tech/ui-react:test --excludeTaskDependencies -- '../../../../🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx' --silent=false --reporter=verbose`. Result: **43/43 passed**, one file, Vitest 8.49 s, Nx 9.9 s. Output is at `🗑️generated/sol-tree-disclosure/react-focused.log`.

## Verification

The first focused React run correctly failed 1/11 after mounting only the collapsed default branches: it observed the kind, Parallel, and Perspective rows while the neutral contract names the fully expanded taxonomy. The next interaction run exposed a second bad fixture assumption: expanded up-flow branches place children above their parent row. The neutral list now records that actual production order, and the oracle opens the Puzzle 3D section plus Parallel, Axonometric, Oblique, and Perspective through the real disclosure controls before reading visual order.

Focused command: `NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long bun nx run @semio-tech/framework-renderer-react:test --excludeTaskDependencies -- '../../../../🧱️elements/📌️ChromePanels/🧪️tests/🧩️component/🟦️.tsx' --silent=false --reporter=verbose`. Result: **11/11 passed**, one file, Vitest 15.04 s, Nx 16.7 s. Output is at `🗑️generated/sol-display-order/react-focused-green.log`. The native receipt is pending and no production Display mutation has landed yet.

The native law now consumes the same `displayResolvedOrder` fixture and sorts the accepted label-hit rectangles by physical `y`; its collapsed subset must be kind, Parallel, Perspective. The older publication-order assertion is corrected to the inverse authored order required by a retained Up-flow Tree: Perspective, Parallel, kind. This law is staged without a new Cargo invocation while the existing Projection run owns the native queue.

## Root Takeover — 2026-09-28 00:24 CEST

After the execution agent reached its usage limit, root applied the bounded production repair: recursive Display template levels are pre-reversed, and each window-kind section appends the plain kind after its templates. The source now uses the same up-flow authored order as React. The existing actual React11/11 and observed WGPU23 reversed order are the regression evidence; the staged native physical-hit law and refreshed artifact acceptance remain pending.
