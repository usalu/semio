# Writer React Parity — Verify Log

## Root cause / scope

At the `premigration` tag, `writer/react/index.tsx` (1542 lines) + `writer/core/js` implemented a full jack code editor (AST tree, selectable spans, symbol occurrences/rename, placeholders, newline gating, completions, context menu, window measures/engagement, toolbar) on top of a GPU editor session. The post-migration `writer/plugin/rs` only had a crude keyword-scan AST, naive occurrence highlighting, no spans/placeholders/gating/measures/engagement/toolbar, and the generic `text-editor-host.tsx` lacked keyboard navigation, completions UX, rename, and context menu, plus had a latent `rect`-undefined crash on pointer-up.

## Changes

- `framework/core/rs/lib.rs`: `TextEditorScene` += `hoverJson`, `newlineGatesJson`, `renameJson`.
- `framework/editor/rs/lib.rs`: `sync_from_scene_json` (both the native `EditorHost` path and the wasm-bindgen `EditorSession` path) now consume `hoverJson` → `set_hover_range`.
- `framework/renderer/react/os-shell.tsx`: mirrored the 3 scene fields on the TS `TextEditorScene` type; extended `EditorWasmSession` with the editor-rs exports the host needs (move*, tabInsertText, setSelectionRange, selectSpanAt/AtScreen, pickTargetsAtScreenJson, caretWorldJson, worldToScreenJson, setSelectionOccurrencesJson, setExtraCaretsJson, setCaretVisible).
- `writer/plugin/rs/lib.rs`: full rewrite of the jack intelligence — recursive-descent AST parser (query/match/where/return/create/delete/set/merge/pattern/edge/expr), selectable spans (atomic/varLabel/propertyAccess), placeholders, newline gating, jack symbol resolution (variable/property/nodeKind/edgeKind) + rename; scene now emits `selectableSpansJson`/`placeholdersJson`/`newlineGatesJson`/`hoverJson`/`renameJson` and rebuilt `occurrencesJson`/`extraCaretsJson`; new commands `textHover`, `setAstHover`, `engagementInput`, `engagementSubmit`; multi-occurrence `commitRename`; `tools()`/`window_measures()`/`window_engagements()` implemented; `dag.jack` example added; `default_line_height` fixed to 22 (premigration parity).
- `framework/renderer/react/components/text-editor-host.tsx`: rewrote `WasmEditorSurface` — fixed the undefined-`rect` crash on pointer-up; added arrow/Home/End/Tab keyboard navigation, Enter newline-gating, Cmd+Space/Alt+right-click completions with host-local open/cycle state, F2/context-menu rename with live multi-span preview (scene-sync frozen while renaming), a generic right-click context menu (`buildTextEditorContextMenuItems`, via `ContextMenuController`), double-click span select, hover dispatch deduped by token range, and wheel-driven camera persistence (`setCamera` dispatched from live session state). Exported `multiSpanReplace`, `lineRangeAt`, `buildTextEditorContextMenuItems` for testing.
- `.claude/launch.json`: added `writer-react-dev` (port 6062, `SEMIO_RENDERER=react`) for browser preview, mirroring the existing per-tech entries.

## Test results

### `cargo test -p writer-plugin` (native — NOT blocked by `component_export_anchor`, contrary to the 2D/3D-ticket note; that assumption did not hold for this crate)
24/24 passed: full AST shape, smallest-containing-node selection mapping, selectable spans (varLabel/propertyAccess offsets), symbol occurrences (bound variable vs nodeKind), placeholders (expr/Label), newline gates (allow/disallow), multi-occurrence `commitRename`, `engagementSubmit "font 16"`, `window_measures`/`window_engagements`/`tools` contents, scene field presence, `setAstHover` → tree highlight + scene `hoverJson`, manifest `dag.jack` example.

### `cargo test -p framework_editor -p semio-framework-core`
18 + 7 passed, including new `sync_from_scene_json_sets_and_clears_hover_range`.

### `cargo check -p writer-plugin --target wasm32-wasip2`
Clean (no errors/warnings beyond 2 pre-existing crate-wide dead-code warnings elsewhere).

### `bun ./script.ts test --run index.test.ts` (framework/renderer/react, vitest)
55/55 passed, including new cases: scene renders with hover/newline/rename fields; `buildTextEditorContextMenuItems` suggest-first and pick-rows cases (ported from premigration); `multiSpanReplace`; `lineRangeAt`.

### `bunx tsc --noEmit -p tsconfig.json`
`text-editor-host.tsx`: 0 real errors (only the pre-existing, repo-wide `TS5097` `.tsx`-import-extension noise, 254 instances across the whole codebase, unrelated to this change). Also fixed one pre-existing bug while rewriting the file: the wasm-session fallback stub was missing `setCanvasThemeJson`, silently violating `EditorWasmSession` even before this ticket.

### `bun framework/product/os/dev/script.ts build writer` (wasm32-wasip2 release + jco transpile)
Plugin component built cleanly: `writer_plugin.wasm`, jco-transpiled `writer_plugin_component.js`, and the bridge `writer_plugin.js` all produced under `framework/product/os/dev/plugin-modules/writer/`. (The script's subsequent static `vite build` step for a `writer/index.html` entry fails — that target expects a standalone static bundle entry point that was never part of the plugin-dev flow; irrelevant to the dev-server/browser verification path used here.)

### `wasm-verify.ts` (`.repo/🎫/26/07/10/WRITER-REACT-PARITY/wasm-verify.ts`)
Ran against the real compiled `writer_plugin.js` bridge — all checks passed:
- Manifest exposes examples `empty`, `jack`, `dag.jack`.
- Initial jack scene carries all new fields (`hoverJson`, `newlineGatesJson`, `placeholdersJson`, `selectableSpansJson`, `completionsJson`).
- Placing the caret on the bound variable `a` yields 3 occurrences, `extraCaretsJson`, and `renameJson` with all 3 spans.
- `commitRename` renames all 3 occurrences in one op (verified buffer text).
- `engagementSubmit {value:"font 16"}` updates the live `window_measures` slider to 16.
- `window_engagements` input placeholder contains "Format"; `tools()` contains `writer-format` + `writer-lint`.
- `setAstHover` on the tree's root id sets `hoverJson` to the root span and the document tree's `highlightedIds`.

### Browser verification (`writer-react-dev`, port 6062) — NOT PERFORMED, documented honestly
Could not open a live browser preview in this session: the folder's dev-server pool (max 5 concurrent) was entirely occupied by other concurrent sessions at verification time, and stopping someone else's in-progress server would destroy their work. Automated evidence above (24 Rust unit tests, 55 vitest cases, and an end-to-end `wasm-verify.ts` round-trip against the actual compiled `writer_plugin.js`/`writer_plugin.wasm`) is strong but does not substitute for a real GPU-rendered click-through. A follow-up session with a free dev-server slot should run `writer-react-dev` (port 6062) / `bun run dev:writer` with `SEMIO_RENDERER=react` and manually confirm:
- Enter inserts a newline only at jack-meaningful positions (e.g. after `)`), not mid-token or before `.`.
- Cmd+Space opens the completions popup at the caret; Arrow keys cycle, Tab/Enter accepts, replacing the identifier prefix.
- Placing the caret on a bound variable (e.g. `a` in the jack example) shows highlighted occurrences with extra carets at each.
- F2 opens the rename input at the caret; typing live-updates every occurrence; Enter commits, Escape reverts.
- Right-click opens the context menu (Suggest/pick rows/Select token/Select line/Select all/Rename/Cut/Copy/Paste/Format/Lint); Alt+right-click opens completions directly.
- Hovering an AST tree row highlights the corresponding editor span and vice versa.
- The font-size/line-height/tab-size sliders and line-numbers toggle in the window measures panel update the editor live.
- Typing "font 16" into the engagement input and submitting changes the font size.
- Toolbar Format/Lint buttons work.
- Scrolling the editor (wheel) persists camera position across the next keystroke (no snap-back).
- The `dag.jack` example loads from the catalogue.
Hard-refresh after any plugin rebuild — Vite does not reliably hot-swap `.wasm`.

## Pitfalls encountered

- Two unrelated concurrent sessions were editing shared files at the same time (`framework/core/rs/lib.rs` `UiComponentSceneNode` icon_render/note_canvas fields, `framework/plugin/rs/lib.rs` `RasterScene` shape, `@semio-tech/raster-rs` workspace symlink, `os-shell.tsx` `PluginWasmHandle.windowMeasures`) — this repo runs on a shared "wip" branch with an apparent auto-commit process (HEAD advanced from commit 188 to 196+ during this session). Confirmed transient breakage self-resolved as those sessions landed their work; none of it required intervention from this ticket.
- `WindowEngagementPossible`/`WindowEngagementStatus` are not re-exported at `semio_framework_plugin`'s crate root (only inside `layout::`) — imported via the explicit `semio_framework_plugin::layout::{...}` path.
- `WindowEngagementInput`/`UiTreeNode` gained new required fields (`on_abort`, `drop_command`) from concurrent work partway through this ticket — both `None`.
- `jack_symbol_at_offset`/`jack_newline_allowed_at` test-writing bugs (off-by-one for single-char identifiers; escaped-JSON substring assertions) were caught and fixed — not logic bugs in the plugin itself.

## Files touched

- `writer/plugin/rs/lib.rs`
- `framework/core/rs/lib.rs`
- `framework/editor/rs/lib.rs`
- `framework/renderer/react/os-shell.tsx`
- `framework/renderer/react/components/text-editor-host.tsx`
- `framework/renderer/react/index.test.ts`
- `.claude/launch.json`
- `.repo/🎫/26/07/10/WRITER-REACT-PARITY/wasm-verify.ts` (new, ticket-scoped)
- `.repo/🎫/26/07/10/WRITER-REACT-PARITY/verify-log.md` (this file)
