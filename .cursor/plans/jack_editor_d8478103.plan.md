---
name: Jack Editor
overview: Replace the adhoc single-line Jack engagement input with a handcrafted code editor (syntax highlighting + autocomplete driven by the Rust jack core via WASM) exposed as a new generic framework `editor` component, laid out in a proper multi-window split (graph canvas + Jack editor + results table).
todos:
  - id: rust-lang
    content: Refactor jack lexer to emit token spans; add tokenize() + TokenClass and complete()/Completion in trinity/jack/core/lib.rs with tests
    status: completed
  - id: wasm
    content: Add tokenize_jack_json/complete_jack_json on TrinityHost + wasm bindings in trinity/rewrite/engine/lib.rs; rebuild WASM pkg; add host tests
    status: completed
  - id: react-bridge
    content: Add tokenizeJackOnFixture/completeJackOnFixture + token/completion types and vitest in trinity/react/index.tsx
    status: completed
  - id: editor-component
    content: Add generic 'editor' ComponentKind, UiEditorHostSurfaceNode, buildEditorWindowBody, and renderer surface-binding/registration across framework platform+playground core and renderers
    status: completed
  - id: code-editor
    content: Implement handcrafted language-agnostic CodeEditor React primitive (textarea+highlight overlay, gutter, autocomplete popup, Cmd+Enter submit) with theme color mapping and a vitest
    status: completed
  - id: jack-windows
    content: Rewire trinity/jack/play into graph+editor+results windows with nested split layout; drop engagement input; wire editor/results surface hosts in the playground renderer; update vitests
    status: completed
  - id: validate
    content: Run cargo tests, vitest, and the trinity jack dev server; confirm highlighting/completion/run/results at runtime via [DEBUG] logs
    status: completed
isProject: false
---

# Properly Implement a Jack Text Editor

## Context / current state
The Jack query is currently an "adhoc" single-line `engagement.input` (id `trinity-jack-query`) crammed into the graph canvas window's engagement bar in [trinity/jack/play/index.ts](trinity/jack/play/index.ts) (lines 105-125). The graph canvas is the only window; the computed `jackResultJson` is never shown. The Jack grammar lives in Rust at [trinity/jack/core/lib.rs](trinity/jack/core/lib.rs) (lexer discards token spans, no completion API). WASM is exposed via `TrinitySession` in [trinity/rewrite/engine/lib.rs](trinity/rewrite/engine/lib.rs); the React bridge is [trinity/react/index.tsx](trinity/react/index.tsx). The framework has a fixed `ComponentKind` vocabulary in [framework/product/platform/core/index.ts](framework/product/platform/core/index.ts) (line 286) rendered via surface-binding hosts in the renderers.

Decisions confirmed: handcrafted editor (zero external deps), language services from the Rust jack core via WASM (single source of truth), new reusable framework `editor` component + split layout (graph canvas + Jack editor + results table).

## 1. Jack language services in Rust (single source of truth)
File: [trinity/jack/core/lib.rs](trinity/jack/core/lib.rs)
- Refactor the `🔖Lexer` region so `lex` records byte spans. Introduce a `TokenSpan { class: TokenClass, start: usize, end: usize }` and a `TokenClass` enum (`Keyword`, `Ident`, `Number`, `String`, `Operator`, `Punctuation`, `Error`), both `Serialize`. Keep the parser working by adapting it to consume `(Token, span)` (strip spans where unused). Use clean refactor (no parallel duplicate lexer).
- Add `pub fn tokenize(input: &str) -> Vec<TokenSpan>` that is total/non-failing (unterminated strings, unexpected chars become `String`/`Error` tokens) so it can highlight while typing.
- Add a `🔖Language` region with `pub struct Completion { label, kind, detail, insert }` (Serialize) and `pub fn complete(graph: &Graph, source: &str, cursor: usize) -> Vec<Completion>`. Context logic from tokens before the cursor:
  - Statement start / after a clause: clause keywords (`MATCH`, `WHERE`, `RETURN`, `CREATE`, `DELETE`, `SET`, `MERGE`), plus `AND`/`OR` inside WHERE.
  - After `:` in a node/edge pattern: node/edge kinds from the graph (distinct `node.kind`, `edge.kind`) and manifest kinds.
  - After `.`: property names = `id`, `name`, `kind` + union of property keys across `graph.nodes`.
  - Otherwise: variables already bound earlier in the query.
- Extend the existing `🔖Tests` region with tokenize-span and completion tests (keywords, kinds, properties, variables). No new test files.

## 2. Expose language services over WASM
File: [trinity/rewrite/engine/lib.rs](trinity/rewrite/engine/lib.rs)
- Re-export `tokenize`/`complete`/`TokenSpan`/`Completion` from `trinity_jack` (near line 14).
- On `TrinityHost` (impl near line 182): add `tokenize_jack_json(&self, source) -> String` and `complete_jack_json(&self, source, cursor) -> String` (serde to JSON), mirroring `run_jack_json` (line 256).
- In `wasm_session` (near line 562): add `#[wasm_bindgen(js_name = tokenizeJackJson)]` and `#[wasm_bindgen(js_name = completeJackJson)]` bindings.
- Rebuild the WASM pkg with the existing build (`bun ./script.ts wasm` in `trinity/rewrite/engine`, see [trinity/rewrite/engine/script.ts](trinity/rewrite/engine/script.ts)) so `pkg/trinity_rewrite.js` picks up the new methods. Add native host tests for the two json methods in the engine `tests` region.

## 3. React language bridge
File: [trinity/react/index.tsx](trinity/react/index.tsx)
- In the `🔖Fixture` region, add `TrinityJackTokenV1 { class, start, end }` and `TrinityJackCompletionV1 { label, kind, detail, insert }` types, plus `tokenizeJackOnFixture(fixtureJson, source)` and `completeJackOnFixture(fixtureJson, source, cursor)` that mirror `runJackOnFixture` (line 84): new `TrinitySession`, `loadFixtureJson`, call the new WASM methods, `JSON.parse`. Export them. Add vitest cases in the `🧪Tests` region.

## 4. New generic framework `editor` component
- [framework/product/platform/core/index.ts](framework/product/platform/core/index.ts): add `"editor"` to `ComponentKind` (line 286) and to `CANVAS_COMPONENT_KINDS` (line 288, host-bound full surface). Add `interface UiEditorHostSurfaceNode { type: "editor"; componentKind: "editor"; surfaceId; controllerId; paneId?; bindingId? }`, include it in the `UiNode`/host unions (lines ~173, ~431), and add `buildEditorWindowBody(surfaceId, controllerId, paneId?, bindingId?)` next to `buildTableWindowBody` (line 512).
- [framework/product/playground/core/index.ts](framework/product/playground/core/index.ts): import `UiEditorHostSurfaceNode`, add to the `UiNode` union (lines 122-135), re-export `buildEditorWindowBody`.
- Renderer: add `registerUiEditorSurfaceHost` (alias of `registerSurfaceBinding`) in [framework/product/platform/renderer/react/index.tsx](framework/product/platform/renderer/react/index.tsx) next to `registerUiTableSurfaceHost` (line 3416), a `renderEditor` -> `renderBoundComponent(node, "panel", ...)` and a `case "editor"` in `renderNode` (line 3557). In [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) add `"editor"` to `PLAYGROUND_CANVAS_HOST_TYPES` (line 372) and the editor branch in `renderPlaygroundHostSurface` (line 434), plus an exported `registerUiEditorSurfaceHost`.

### Handcrafted `CodeEditor` React primitive (language-agnostic)
Add a `🔖CodeEditor` region in [framework/product/platform/renderer/react/index.tsx](framework/product/platform/renderer/react/index.tsx). Props: `value`, `onChange`, `onSubmit`, `tokenize(text) => Token[]`, `complete(text, cursor) => Completion[]`. Implementation (classic overlay technique, no contenteditable):
- A transparent `<textarea>` layered over a highlight `<pre>` that renders tokenized spans with theme color classes; scroll positions kept in sync; a line-number gutter.
- Autocomplete popup: on input/caret change call `complete`; render a floating list positioned at the caret (mirror-div coordinate measurement); Up/Down to move, Enter/Tab to accept (insert `insert`), Esc to dismiss.
- Keybindings: `Cmd/Ctrl+Enter` -> `onSubmit`; `Tab` indent (suppressed when the popup is open).
- Token color classes map `TokenClass` -> existing `@semio-tech/ui-styling` semantic colors (keyword/string/number/operator/ident). Add a minimal vitest covering tokenized rendering output.

## 5. Rewire the Trinity Jack playground into proper windows
File: [trinity/jack/play/index.ts](trinity/jack/play/index.ts)
- Add surface ids/body keys/window kinds: graph (existing `trinity` surface), editor (`trinity.jack.editor/v1`), results (`trinity.jack.results/v1`, reuses the existing `table` component).
- `rebuildShellMode`: drop the engagement command-line input entirely (the editor replaces it). Engagement is optional, so window kinds without engagement are valid per `enforceWindowKindsEngagementInput`. Keep `setJackQuery` / `runJackQuery` commands on the controller; `runJackQuery` populates `jackResultJson`.
- Register three declarative bodies via `registerWindowBody`: `buildTrinityWindowBody` (graph), `buildEditorWindowBody` (editor), `buildTableWindowBody` (results).
- Layout: hand-build a nested `WindowLayout` (row -> [stack(graph) size ~0.6, column -> [stack(editor) ~0.55, stack(results) ~0.45] size ~0.4]) instead of `createStackLayout`, following the nested tree shape used by `orbitViewArrangementToLayout` in [framework/product/playground/core/index.ts](framework/product/playground/core/index.ts).
- Update the in-file vitest: assert the three window kinds/bodies and that `runJackQuery` fills results.

File: [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) (`🔖TrinityPlayHost`)
- Add `TrinityJackEditorSurfaceHost` rendering `<CodeEditor value={ctrl.getJackQuery()} onChange={(v)=>ctrl.run("setJackQuery",{value:v})} onSubmit={()=>ctrl.run("runJackQuery")} tokenize={(t)=>tokenizeJackOnFixture(ctrl.getFixtureJson(), t)} complete={(t,c)=>completeJackOnFixture(ctrl.getFixtureJson(), t, c)} />`.
- Add `TrinityJackResultsSurfaceHost` that builds the table from `ctrl.getJackResultJson()` (columns/rows), following the existing table host/`renderBoundComponent` pattern.
- In `registerTrinityJackPlaySurfaceHosts` (line 6967): also register the editor surface (`registerUiEditorSurfaceHost`) and results surface (`registerUiTableSurfaceHost`), and the new declarative bodies.

## 6. Validation
- `cargo test -p trinity_jack` and `-p trinity_rewrite` (tokenize/complete + host json methods).
- Rebuild WASM, then `nx`/vitest for `trinity-react`, `trinity-jack-play`, and the framework editor component.
- Run `dev:trinity:jack` (port 6054) and confirm at runtime via `[DEBUG]` logs: highlighting tokens, completion list on `:`/`.`/keywords, `Cmd+Enter` runs the query, results table populates.

## Notes / conventions
- Work inside the repo MCP ticket flow: read `repo://goals`, then open a new ticket (goal `🎯trinity`) since the prior trinity ticket is closed and this is a distinct task; keep any temp files inside the ticket folder.
- All new code goes into existing files using `#region`/`pub mod` structuring; no new script/test/example files; docstrings start with an emoji; external libs only behind interfaces (none added here).