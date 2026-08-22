# Phase 9/10 Next Dependency Packets

<!-- #region Snapshot -->

## Snapshot

Read-only audit on 2026-08-22. The live dependency ratchet reports **170** current third-party identities from the 238-identity baseline (**68 removed**); this is already below the 173-identity checkpoint. `bun ./📜️script.ts verify dependencies parity js` reports `manifests=83 external-rows=291 evidenced=141 unowned=150 undeclared-imports=0` and is clean. The `@types/d3-force`, `@types/dagre`, and `@types/pg` packets are already removed in this live snapshot, so they are deliberately not scheduled again.

All packets below exclude `./compose`, avoid Cargo, and are file-disjoint from one another except for the shared final dependency gates.

<!-- #endregion Snapshot -->

<!-- #region RankedPackets -->

## Ranked Packets

### 1. Remove the unused React reconciler façade — 2 identities, low risk

- Identities: `react-reconciler`, `@types/react-reconciler`.
- Manifest: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/package.json`.
- Exact source: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🟦️component.tsx` imports and re-exports `Reconciler` and its constants at lines 8–27. Workspace consumers import only the owned `GraphWasmCanvas`, `GraphWasmSession`, and `CanvasInputModifiers`; no consumer uses the reconciler re-exports.
- Replacement: remove the unused imports and re-exports, then remove both manifest rows. The package remains a normal React canvas host; it does not construct a reconciler.
- Existing owned boundary/tests: `GraphWasmCanvas` and `CanvasWasmBridge` are owned contracts in the same component, with the package’s in-source Vitest suite.
- Gates: `bun install --ignore-scripts`; `bun nx run @semio-tech/infinite-canvas-react-renderer:test-quick --skip-nx-cache`; renderer focused quick test; `bun ./📜️script.ts verify dependencies`; `bun ./📜️script.ts verify dependencies parity js`.

### 2. Remove the PostCSS config type-only package — 1 identity, low risk

- Identity: `postcss-load-config`.
- Manifest: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`.
- Exact source: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️postcss.config.ts` imports only `Config` as a compile-time annotation; it never loads or executes the package.
- Replacement: define a concise local structural `OwnedPostcssConfig` for the object actually exported (`plugins: Record<string, Record<string, never>>` or equivalent) and retain the direct `@tailwindcss/postcss` program key.
- Existing owned boundary/tests: the PostCSS configuration itself is the owned build boundary; Tailwind remains the declared compiler implementation.
- Gates: `bun install --ignore-scripts`; `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`; `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`; `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`; dependency freeze/parity gates.

### 3. Replace the React PDF convenience layer with an owned PDF canvas port — 1 identity, medium risk

- Identity: `react-pdf` (retain `pdfjs-dist`, which has independent UI and print consumers).
- Manifest: `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/package.json`.
- Exact source: `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️component.tsx` imports `Document`, `Page`, and `pdfjs` from `react-pdf`; its own `pdfEmbodiment*` helpers already own page choice, navigation, sizing, and loading state. The package test setup has an explicit React-PDF mock.
- Replacement: introduce an owned `PdfCanvasPort` in the presentation renderer. Load the document and page through `pdfjs-dist`, render into a managed canvas, expose only owned document/page/load contracts, and migrate the existing embodiment helpers and test mock to that port. Cancellation must dispose document/render tasks on source/page changes.
- Existing owned boundary/tests: `PdfEmbodiment` helpers and page-navigation tests are co-located; `🟦️vitest.setup.ts` already isolates the external presentation implementation.
- Gates: `bun install --ignore-scripts`; `bun nx run @semio-tech/animate-js:test-quick --skip-nx-cache`; the presentation renderer’s PDF-focused Vitest selection; plugin build; dependency freeze/parity.

### 4. Replace the hotkey hook with the existing owned keybinding context — 1 identity, medium risk

- Identity: `react-hotkeys-hook`.
- Manifest: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`.
- Exact source: direct use is limited to `📦️index.tsx` (the palette/panel chords and a re-export) and `🔨️modules/⌨️control-keybinding-context/🟦️component.tsx` (the `useControlKeybinding` adapter).
- Replacement: make `control-keybinding-context` own a small React keyboard-listener hook: parse the already-declared comma-separated chords, normalize `mod`, honour `enabled` and `enableOnFormTags`, prevent default only for a matched command, and bind/unbind via an effect. Export the owned hook rather than the third-party hook.
- Existing owned boundary/tests: `ControlKeybindingOptions`, `ControlKeybindingDependencies`, `SHELL_KEYBINDINGS`, and `useControlKeybinding` already define the product contract; `formatKeybindingShortcut` centralizes platform labels.
- Gates: `bun install --ignore-scripts`; `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`; UI quick and lint; focused keyboard-chord tests including form-field, Meta/Control, disabled, and cleanup cases; dependency freeze/parity.

### 5. Replace class-variance-authority with an owned style-variant compiler — 1 identity, medium risk

- Identity: `class-variance-authority`.
- Manifest: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`.
- Exact source: `📦️index.tsx` owns the public re-export and `cva` calls; leaf uses are `🧱️elements/⚡️ActionGroup/🟦️component.tsx`, `🧱️elements/🎛️ButtonGroup/🟦️component.tsx`, `🧱️elements/🎛️ToggleGroup/🟦️component.tsx`, and type-only use in `🧱️elements/🔘️Button/🟦️component.tsx`.
- Replacement: add a schema-first owned `styleVariants` compiler beside `🏷️class-name-composition`, with owned `StyleVariantProps`. It need only encode the present base/variant/default/compound selection semantics, so its behavior can be exhaustively table-tested before migration. Remove the third-party public re-export rather than leaking it through the UI API.
- Existing owned boundary/tests: `cn` is the owned class-composition gateway; the four affected controls are focused leaves, making variant matrices deterministic.
- Gates: UI typecheck, quick suite, lint, `check-ui-primitives`, and component matrix tests for every existing variant/default/compound combination; then dependency freeze/parity.

### 6. Replace the class composition implementation — 2 identities, high risk

- Identities: `clsx`, `tailwind-merge`.
- Manifest: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`.
- Exact source: only `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🟦️component.ts` imports them. All product call sites already use the owned `cn` gateway.
- Replacement: preserve `ClassNameInput`, implement owned recursive token flattening, then add a deliberately scoped Tailwind conflict table covering the workspace’s generated class families (including the existing `ui-surface`, `ui-glass`, and `ui-veil` group). Do not claim broad Tailwind compatibility: derive the finite conflict table from actual UI source and lock it with fixtures.
- Existing owned boundary/tests: the single `cn` gateway confines the change to one module. This is the correct locus for a test corpus of nested inputs, falsey suppression, custom groups, and conflict winners.
- Gates: UI typecheck, quick, lint, `check-ui-primitives`, plus a source-derived conflict fixture suite and demonstrator build; dependency freeze/parity.

### 7. Replace the presentation Markdown compiler — 5 identities, high risk

- Identities: `unified`, `remark-parse`, `remark-gfm`, `remark-rehype`, `rehype-stringify`.
- Manifest: `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/package.json`.
- Exact source: the same presentation renderer’s `defaultMarkdownHtmlCompiler` is the sole importer and already exposes the owned `MarkdownHtmlCompiler`, `setMarkdownHtmlCompiler`, and `compileMarkdownToHtml` seam.
- Replacement: define an owned schema-first CommonMark/GFM subset AST and an HTML serializer behind `MarkdownHtmlCompiler`; use the repository’s existing CommonMark import/export artifact schema as the content contract. Start with the exact rendered forms covered by slides (paragraphs, headings, emphasis, code, links, lists, tables) and add HTML escaping and URL-scheme policy before activating it.
- Existing owned boundary/tests: the compiler substitution seam and a GFM table test already exist in the renderer; the Rust CommonMark import/export artifacts provide a native specification anchor without coupling the web renderer to a third-party AST.
- Gates: `bun nx run @semio-tech/animate-js:test-quick --skip-nx-cache`, focused markdown fixture suite (escaping, tables, links, lists, malformed input), plugin build, and dependency freeze/parity. This is a discrete high-effort Phase 10 packet, not a quick deletion.

<!-- #endregion RankedPackets -->

<!-- #region DeliberateExclusions -->

## Deliberate Exclusions

- `@types/mocha`, `@types/vscode`, `@types/three`, React types, and Node types remain direct platform/type boundaries; the snapshot establishes actual use, not stale manifest rows.
- `pg-boss`, `react-resizable-panels`, DnD, XYFlow, Reveal, PDF.js, Playwright, Nx, and the renderer/graphics libraries remain active implementation boundaries. Replacing them is a separate product/runtime redesign, not an evidence-backed deletion packet.
- No Rust packet is proposed here: the Cargo slot is serialized for the active P4 work, and the remaining Rust dependencies sampled are active runtime dependencies rather than isolated stale declarations.

<!-- #endregion DeliberateExclusions -->
