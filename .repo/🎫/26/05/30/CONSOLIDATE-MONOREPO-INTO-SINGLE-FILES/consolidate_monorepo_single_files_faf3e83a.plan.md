---
name: Consolidate Monorepo Single Files
overview: "Continue the existing \"Consolidate Monorepo Into Single Files\" effort: merge every remaining multi-file Rust crate and tightly-coupled TS/TSX module cluster into a single file per logical unit, using the repo's established `pub mod name { //#region ... }` (Rust) and `//#region 🔖name` (TS) inlining pattern, while leaving genuinely platform-mandated splits untouched."
todos:
  - id: reopen-ticket
    content: Reopen CONSOLIDATE-MONOREPO-INTO-SINGLE-FILES ticket
    status: completed
  - id: rust-ui-wgpu
    content: Consolidate ui/wgpu/rs (12 files) into lib.rs, preserving uncommitted draw.rs/shaders.rs edits
    status: completed
  - id: rust-renderer-wgpu
    content: Consolidate framework/renderer/wgpu/rs (7 files) into lib.rs, preserving uncommitted edits
    status: completed
  - id: rust-framework-core
    content: Consolidate framework/core/rs (7 files) into lib.rs
    status: completed
  - id: rust-os-core
    content: Consolidate framework/product/os/core/rs (7 files) into lib.rs
    status: completed
  - id: rust-framework-plugin
    content: Consolidate framework/plugin/rs (6 files) into lib.rs
    status: completed
  - id: rust-layout
    content: Consolidate layout/rs (6 files) into lib.rs
    status: completed
  - id: rust-graph-dsl
    content: Consolidate mathematical/graph/dsl/rs (4 files, incl. include! jack_impl.rs) into lib.rs
    status: completed
  - id: rust-trinity-jack-core
    content: Consolidate trinity/jack/core/rs (queryable.rs) into lib.rs
    status: completed
  - id: rust-writer
    content: Consolidate writer/rs and writer/plugin/rs into their lib.rs files
    status: completed
  - id: ts-graph-canvas-triad
    content: Merge flow-graph-canvas-host.tsx + graph-canvas-overlays.tsx into node-graph-host.tsx
    status: completed
  - id: ts-os-shell-satellites
    content: Merge os-chrome-panels/ui-search-find/tool-tree/types/plugin-runtime/wasm-session-loader into os-shell.tsx
    status: completed
  - id: ts-storybook-kit-store
    content: Merge .storybook kit-store 9-file cluster into one file
    status: completed
  - id: ts-small-pairs
    content: Merge remaining small file pairs (plugin-registry, generate.neo4j.gen, storybook decorators, jack lsp protocol, sketchpad docs-mdx)
    status: completed
  - id: verify
    content: Run cargo check/test per crate, TS build/typecheck, and visual dev-server smoke test
    status: completed
  - id: close-ticket
    content: Update ticket summary and close it
    status: completed
isProject: false
---

## Ticket

An existing ticket already covers exactly this task: [`.repo/🎫/26/05/30/CONSOLIDATE-MONOREPO-INTO-SINGLE-FILES/ticket.json`](.repo/🎫/26/05/30/CONSOLIDATE-MONOREPO-INTO-SINGLE-FILES/ticket.json) (status `closed`, goal `aioptimizedrepo/singlefilerepo`). Per repo rules, the first execution step is `ticket_reopen` on this ticket rather than opening a new one. All scratch scripts go into that ticket folder (following the precedent of `inline-rust-mod.ts` / `merge-scattered-files.ts` already there).

## Mechanical approach (already established, reuse it)

- Rust: for a crate with `lib.rs` + `pub mod foo;` / `mod foo;` pointing at sibling `foo.rs`, replace the declaration with `pub mod foo {\n// #region foo\n<contents>\n// #endregion foo\n}` inlined directly in `lib.rs`, then delete `foo.rs`. This preserves `crate::foo::X` paths so no call-site rewrites are needed. (Exact pattern already used in [`inline-rust-mod.ts`](.repo/🎫/26/05/30/CONSOLIDATE-MONOREPO-INTO-SINGLE-FILES/inline-rust-mod.ts).)
- Rust `include!("file.rs")` cases: splice the file's literal contents in place of the `include!` line (no `mod` wrapper needed, since `include!` is already flat) and delete the file.
- TS/TSX: paste each satellite file's body (minus its own header/local-import lines that become redundant) into the target file wrapped in `//#region 🔖name` / `//#endregion 🔖name`, merge/dedupe imports, delete the satellite file, fix any external importers. (Pattern from [`merge-repo-lib-js-satellites.ts`](.repo/🎫/26/05/30/CONSOLIDATE-MONOREPO-INTO-SINGLE-FILES/merge-repo-lib-js-satellites.ts).)
- After each crate/module merge: `cargo check -p <crate>` (+ `cargo test` where tests exist) or the module's `nx build`/`tsc`/vitest, before moving to the next one, since several of these files have in-flight uncommitted changes from the current Flow/wgpu rich-rendering-parity work that must be preserved verbatim.

## Rust crates to consolidate (10)

1. [`ui/wgpu/rs/`](ui/wgpu/rs/lib.rs) — 12→1: `draw.rs`(2734) `widgets.rs`(1568) `input.rs`(506) `shaders.rs`(405) `text.rs`(340) `gpu.rs`(295) `theme.rs`(259) `cursor.rs`(250) `chrome.rs`(110) `layout.rs`(94) `geometry.rs`(28) all inline into `lib.rs`. Note: `draw.rs`/`shaders.rs` carry uncommitted edits from the active rendering-parity ticket — merge their current (modified) contents, don't revert.
2. [`framework/renderer/wgpu/rs/`](framework/renderer/wgpu/rs/lib.rs) — 7→1: `shell.rs`(4810) `dock.rs`(1825) `scenes.rs`(1819) `engine_canvas.rs`(1004) `interpreter.rs`(401) `plugin_bridge.rs`(228) into `lib.rs` (also carries uncommitted edits — preserve them).
3. [`framework/core/rs/`](framework/core/rs/lib.rs) — 7→1: `ui.rs` `mesh.rs` `layout.rs` `tools.rs` `platform.rs` `command_bus.rs` into `lib.rs`.
4. [`framework/product/os/core/rs/`](framework/product/os/core/rs/lib.rs) — 7→1: `host.rs` `media_graph.rs` `instance.rs` `registry.rs` `media_export_simple.rs` `media_export_raster.rs` into `lib.rs`.
5. [`framework/plugin/rs/`](framework/plugin/rs/lib.rs) — 6→1: `generate_mode.rs` `scaffold.rs` `app.rs` `plugin_runtime.rs` `world3d_host.rs` into `lib.rs`.
6. [`layout/rs/`](layout/rs/lib.rs) — 6→1: `engine.rs` `document.rs` `export.rs` `wasm_session.rs` `display.rs` into `lib.rs`.
7. [`mathematical/graph/dsl/rs/`](mathematical/graph/dsl/rs/lib.rs) — splice `jack_impl.rs`(1467, currently via `include!`) literally in place, and inline `wire.rs`(394) + `queryable.rs`(293) as `mod` blocks.
8. [`trinity/jack/core/rs/`](trinity/jack/core/rs/lib.rs) — inline `queryable.rs`(113).
9. [`writer/rs/`](writer/rs/lib.rs) — inline `document_vcs.rs`(143).
10. [`writer/plugin/rs/`](writer/plugin/rs/lib.rs) — inline `grammar.rs`(97).

## TS/TSX modules to consolidate

11. [`framework/renderer/react/components/node-graph-host.tsx`](framework/renderer/react/components/node-graph-host.tsx) — merge `flow-graph-canvas-host.tsx`(634, has uncommitted edits — preserve) and `graph-canvas-overlays.tsx`(494) into it as regions; both are already only statically imported by `node-graph-host.tsx` (not separately lazy-loaded), so this doesn't change the `React.lazy()` code-split boundary in [`ui-interpreter.tsx`](framework/renderer/react/ui-interpreter.tsx).
12. [`framework/renderer/react/os-shell.tsx`](framework/renderer/react/os-shell.tsx) — merge shell-only satellites `os-chrome-panels.tsx`(344) `ui-search-find.tsx`(244) `tool-tree.tsx`(124) `types.ts`(646) `plugin-runtime.ts`(54) `wasm-session-loader.ts`(148) into it. Verify `types.ts` re-exports stay reachable (type-only imports are erased at build time so lazy-host imports of it are unaffected either way) and update the few cross-importers (`ui-interpreter.tsx`, `node-graph-host.tsx`) to import from `os-shell.tsx`.
13. `.storybook/compose/algorithm/kit-store/*` (9 files, ~2392 lines, only consumed by one story) — merge into a single file.
14. Smaller pairs: `framework/product/os/dev/js/plugin-registry.ts`→`index.ts`; root `generate.neo4j.gen.ts`→`script.ts`; `.storybook/withLevel.tsx`+`withTheme.tsx`→`preview.ts`; `trinity/jack/lsp/js/protocol.ts`→`worker.ts`; `compose/client/lib/sketchpad/js/docs-mdx.ts`→`index.ts`.

## Explicit exclusions (kept split — not "possible" without breaking behavior)

- Rust `build.rs` companions (`infinite/cavas/rs`, `puzzle/2d/rs`, `mathematical/graph/manifest/rs`, `gis/2d/rs`) — Cargo requires build scripts as a separate file.
- Codegen outputs: `ui/styling/rs/generated.rs`, `mathematical/graph/manifest/rs/../generated/*` — same precedent as the prior ticket ("left ... generated as codegen output").
- TS `*.worker.ts` entry points (`compose/client/lib/js/kit-store.worker.ts`, `kernel/3d/brep/js/tessellate.worker.ts`, `repo/server/lib/worker.ts`) — separate execution context/thread, can't be merged into the main module.
- Electron/VS Code multi-entry bundles (`compose/client/ui/desktop`, `coda/client/ui/desktop`, `compose/client/ui/vscode`) — distinct build targets (main/preload/renderer, extension/webview).
- Next.js App Router pages under `repo/server/coordinator/js/app/**` — framework-mandated file-per-route convention.
- The other lazy-loaded scene hosts in `framework/renderer/react/components/` (`world-3d-host.tsx`, `canvas-2d-host.tsx`, `text-editor-host.tsx`, `raster-host.tsx`, `table-host.tsx`, and post-merge `node-graph-host.tsx`) are each already a single file and are independently `React.lazy()`-loaded — not merged together, since that would collapse separate code-split chunks into one.
- `ui/styling/js/` (`tokens.generated.ts` is codegen; `vite-elements-assets.ts`/`index.ts` are shared infra consumed by many unrelated modules across compose/cad/storybook/mit-bestand) — left as-is.
- `mit-bestand` slide content files — data loaded via glob, not an implementation split.

## Verification

- Rust: `cargo check` and `cargo test` per touched crate (`ui_wgpu`, `semio-framework-renderer-wgpu`, `framework_core`, `framework_product_os_core`, `framework_plugin`, `layout`, `mathematical_graph_dsl`, `trinity_jack_core`, `writer`, `writer_plugin`), plus a full workspace `cargo check` at the end.
- TS: `bun run nx build framework-renderer-react` (or equivalent typecheck target) and existing test files for touched modules; visually confirm the dev server (`bun run dev:procedural:3d` / `?plugin=flow`) still renders after the `ui/wgpu`, `framework/renderer/wgpu`, and `node-graph-host` merges, since these are mid-flight in the currently uncommitted rich-rendering-parity work.
- Close out by updating and re-closing the `CONSOLIDATE-MONOREPO-INTO-SINGLE-FILES` ticket with a summary of every file merged/deleted.
