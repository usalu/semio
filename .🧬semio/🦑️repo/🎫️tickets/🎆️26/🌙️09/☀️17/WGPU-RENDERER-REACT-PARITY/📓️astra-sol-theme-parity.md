# Astra Sol Theme Parity

## Scope

This packet implements findings 5 and 6 only: one canonical theme document path and React-equivalent independently expandable, scrollable WGPU theme groups. It does not add migration or retain the removed five-paint compatibility format.

## Canonical theme document

- Removed the executable `CustomChromeTheme` / `ChromeColorOverrides` model, draft helpers, five-paint resolution branch, page state, page actions and their old fixtures.
- `ThemeDocument` is now the single read, label, save, import, export and runtime-resolution shape.
- The parser requires React's canonical fields, including `canvasFonts`, scalar radii/opacities, both appearances and the six `board`, `map`, `canvas`, `chrome`, `outcome`, and `diagram` palette groups. It rejects unresolved token references and preserves optional icon overrides.
- Both preference projection directions pass through the canonical document. Invalid legacy shapes are filtered rather than migrated.
- The neutral JSON fixture at `🧱️elements/🐚️Shell/🧪️tests/🧱️fixtures/🎨️canonical-theme-document/🔣️.json` is accepted by Rust's document parser, `serde_json`, and React's `parseUiTheme` / `serializeUiTheme` path.

## Theme tree and viewport

- The selector and eight editor sections publish as one `UiNode::Tree` with the existing control IDs and actions.
- Section and nested group expansion use `ShellState::tree_open_states` through `canonical_tree_open`. Generic section/tree chevrons update this same map, and tutorial capture/apply consumes it.
- The old exclusive open section and 16-row page controls are gone. Colors and Spacing can remain open together, as can nested Metrics and Appearance groups.
- The stable scroll region is `tree:framework.settings.theme.select`. A logical row cursor and `UiTreeWindow` materialize the current viewport while preserving the full scrollbar pitch under the 128-node retained transport limit.
- Save, delete, reset, import, export and live canonical draft retokenization remain wired. Reset is disabled exactly when no draft exists and the active theme is `semio`.

## Fresh React reference observation

The local React reference was inspected at a temporary browser viewport of 1280×720, then the viewport override was reset. The theme panel's measured scrolling ancestor had `clientHeight = 643`, `scrollHeight = 1442`, and `overflow-y: auto`.

Colors and Spacing were simultaneously expanded. The actual visible rows reported by intersection with the 643 px viewport were `touch`, `compact`, `white`, `warning`, `tertiary`, `success`, `secondary`, `primary`, `light-light-gray`, `light-gray-gray`, `light-gray`, `light-8-9`, `light-7-9`, `light-6-7`, `light-5-9`, `light-5-7`, `light-4-7`, `light`, `l-l-l-g`, and `l-g-g-g`. Fonts, Strokes, Radii, Opacities, Metrics and Appearances headers were present, and there were no page controls.

## React theme input follow-up

The React text, numeric, and appearance-alpha theme editors now use the shared `Input` component's controlled lazy contract: canonical `value`, local edits while focused, and one `onLazyChange` commit on blur or Enter. This removes the prior `defaultValue`/forced-empty-`value` collision that rendered blank fields and triggered React's controlled/uncontrolled warning. Numeric scalar/list parsing and alpha clamping are unchanged.

The mounted ChromePanels component law renders all three consumers, confirms their canonical initial values, changes and blurs each field, verifies the resulting commits, and asserts that React emits no console error.

## Verification actually run

- `rustfmt --edition 2021 --emit stdout` parser checks passed for the Shell WGPU source, WGPU widgets source, theme-editor tests and preferences/theme tests.
- `git diff --check` passed.
- A Bun JSON fixture check passed and confirmed the six light appearance groups.
- The canonical neutral fixture is mounted in the styling Vitest source target and exercises production `parseUiTheme` / `serializeUiTheme` parsing, serialization, label lookup, all six appearance groups, `#336699`, and `navbarHeightUiSpacing = 8`.
- The fresh CUA React inspection above verified the 643 px scroll viewport and the named live groups and rows.
- Root ran `bun nx run @semio-tech/ui-styling-tokens:test-quick` through Nx after mounting the canonical fixture source. All 39 tests passed in 7.7 seconds; the retained log is `🗑️generated/astra-runtime/theme-styling-canonical.log`.
- `SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache -- '../../../../🧱️elements/📌️ChromePanels/🧪️tests/🧩️component/🟦️.tsx' -t 'ChromePanels theme inputs'` passed the focused component file and all 3 tests.
- Native and WASM renderer builds/tests were not launched from this lane because the root task owns those build slots.

## Source files

- `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs`
- `🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs`
- `🧱️elements/🐚️Shell/🧪️tests/🧱️fixtures/🎨️canonical-theme-document/🔣️.json`
- `🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs`
- `🎨️styling/🧪️tests/🧪️theme-resolve/🟦️.ts`
- `🧱️elements/📌️ChromePanels/🟦️.tsx`
- `🧱️elements/📌️ChromePanels/🧪️tests/🧩️component/🟦️.tsx`
- `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`
