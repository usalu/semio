---
name: Trinity Rewrite LHS Graph Sync
overview: Fix the actually-broken trinity/rewrite LHS/RHS pattern-graph rendering, then wire a real bidirectional variable-hover/selection bridge across LHS, RHS, the generated Jack query text, and (as a stretch) the Before/After previews.
todos:
 - id: fix-lhs-rhs-render
   content: Fix TrinityRewriteLhsSurfaceHost/RhsSurfaceHost to use declarativeSceneDescriptor + onDragEnd instead of nonexistent fixtureJson/onFixtureChange props
   status: completed
 - id: label-formatting
   content: "Reformat rewrite pattern node text to 'var : Kind' spacing and update parser/tests"
   status: completed
 - id: var-node-mapping
   content: Export rewriteVarForNodeId/rewriteNodeIdsForVar helpers in trinity-rewrite-react
   status: completed
 - id: writer-external-occurrences
   content: Add externalHoverOccurrences/externalSelectionOccurrences (+signals) props to WriterCanvas wired to existing session occurrence setters
   status: completed
 - id: controller-bridge-state
   content: Add hover/select var state, commands, and getters to TrinityRewritePlayController for LHS/RHS/Jack
   status: completed
 - id: wire-hosts
   content: Wire LHS/RHS Puzzle2dCanvas hosts and Jack WriterCanvas host to the new bridge props in the playground renderer
   status: completed
 - id: before-after-highlight
   content: Add highlighted_ids channel to shared BoardHost + TrinityBridge + TrinityCanvasProps, and compute bound node ids via MATCH...RETURN against Before/After fixtures
   status: completed
 - id: validate
   content: Extend existing vitest/cargo tests, rebuild WASM, and re-screenshot the dev server to confirm the fix and sync behavior
   status: completed
 - id: ticket
   content: Do the work inside a reopened/new repo ticket and close it with a full summary
   status: completed
isProject: false
---

# Trinity Rewrite: Fix LHS/RHS Pattern Graphs + Shared Hover/Selection

## Root-cause findings (verified against the running dev server at `:6056`)

Screenshot of `trinity/rewrite/play` today:

![LHS/RHS render nothing, Jack shows text](attachment)

**Bug 1 — LHS/RHS panels render nothing.** [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) `TrinityRewriteLhsSurfaceHost`/`TrinityRewriteRhsSurfaceHost` (~7248-7286) pass `fixtureJson`/`onFixtureChange` to `Puzzle2dCanvas`:

```7257:7265:framework/product/playground/renderer/react/index.tsx
  return (
    <Puzzle2dCanvas
      fixtureJson={ctrl?.getLhsFixtureJson() ?? REWRITE_DEFAULT_LHS_FIXTURE_JSON}
      kindCatalogs={kindCatalogs}
      fixtureDragDrop
      onFixtureChange={onFixtureChange}
      className="h-full min-h-0"
    />
  );
```

But `Puzzle2dCanvasProps` ([puzzle/2d/react/index.tsx:11956](puzzle/2d/react/index.tsx)) has **no such props** — it takes `declarativeSceneDescriptor` (built via `buildPuzzle2dSceneDescriptorFromFixture(fixture)`) plus `onDragEnd`/`onSelect`/`onHover`/etc., exactly as every _other_ `Puzzle2dCanvas` caller in this same file does (e.g. ~4259-4280). These two hosts never got migrated to the real API, so the fixture is silently ignored and the panes draw an empty grid forever — this alone makes the LHS pattern (and RHS) invisible today, independent of any hover/selection work.

**Bug 2 — no writer <-> graph sync exists anywhere in the repo.** Confirmed via full-repo search: `writer/play` only syncs its editor with its own AST tree panel; no technology links a `writer` text window to a graph canvas. All the building blocks exist but are unwired:

- Graph side: `Puzzle2dCanvas` already has controlled `hoveredId`/`selection` + `onHover`/`onSelect` ([puzzle/2d/react/index.tsx:12036-12055](puzzle/2d/react/index.tsx)).
- Writer side: `WriterCanvasProps` already has `externalSelection(Signal)`/`externalHoverRange(Signal)` + `onSelectionChange`/`onHoverChange` ([writer/react/index.tsx:50-68](writer/react/index.tsx)), and writer-core already parses Jack variables with byte spans via `jackSymbolAtOffset`/`jackVariableOccurrences` ([writer/core/index.ts:1167-1198](writer/core/index.ts)) — built for exactly this, currently only used for in-editor same-text highlighting.
- Missing link: `TrinityRewriteJackSurfaceHost` (~7288-7295) renders `WriterCanvas` with **no** interaction props at all — it's decorative only.

## Scope (per user decision: broader pass, bidirectional)

1. Fix the LHS/RHS rendering bug (prerequisite for everything else).
2. Small formatting polish so the pattern variable name reads clearly on nodes.
3. Bidirectional hover/selection bridge: LHS <-> Jack, RHS <-> Jack (same variable-name key).
4. Extend the bridge to Before/After bound-node highlighting (stretch — needs a small new capability, see below).

## 1. Fix LHS/RHS Puzzle2dCanvas wiring

In [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx), rewrite `TrinityRewriteLhsSurfaceHost`/`TrinityRewriteRhsSurfaceHost` to match the real pattern used elsewhere in the same file (~4194-4280):

- Parse `ctrl.getLhsFixtureJson()`/`getRhsFixtureJson()` via the existing `parseRewriteGraphFixtureJson` (from `@semio-tech/trinity-rewrite-react`).
- `declarativeSceneDescriptor={buildPuzzle2dSceneDescriptorFromFixture(fixture)}` (import from `@semio-tech/puzzle-2d-react`).
- `onDragEnd` commits moved node `x`/`y` back through `ctrl.run("setLhsFixtureJson"/"setRhsFixtureJson", {json})` (fixture already has these commands; see [trinity/rewrite/play/index.ts:449-467](trinity/rewrite/play/index.ts)).
- Keep `kindCatalogs`/`fixtureDragDrop`.
- Full node/edge CRUD editing on these canvases (dragging new pattern nodes from a palette) stays out of scope for this pass — flagged as a known follow-up, not required by the request.

## 2. Variable-name label clarity

In [trinity/rewrite/react/index.tsx](trinity/rewrite/react/index.tsx), `rewriteGraphNode` callers currently squash `"a:Piece"` with no spacing. Reformat to `"${var} : ${kind}"` consistently with the already-spaced `"a.name = 'b'"` where-clause convention; update `parseMatchLabel` to tolerate the extra spaces (trim already handles it) and adjust the two round-trip tests in the same file's `#region Tests`.

## 3. Bidirectional LHS/RHS <-> Jack variable hover/selection bridge

### 3a. Var <-> node-id mapping helpers (`trinity/rewrite/react/index.tsx`)

Export two small functions reusing the existing (currently unexported) `parseMatchLabel`/`parseSetLabel`/`parseParameterLabel`:

- `rewriteVarForNodeId(fixture: Puzzle2dFixtureV1, nodeId: string): string | null`
- `rewriteNodeIdsForVar(fixture: Puzzle2dFixtureV1, varName: string): readonly string[]`

Works for both LHS (`rewrite.match`/`rewrite.where` nodes) and RHS (`rewrite.set`/`rewrite.create`/`rewrite.delete`/`rewrite.merge` nodes reference vars; `rewrite.parameter` nodes are parameters, not graph vars — excluded).

### 3b. Expose multi-occurrence hover/selection externally (`writer/react/index.tsx`)

`WriterCanvas` already drives `session.setHoverOccurrencesJson`/`setSelectionOccurrencesJson` internally ([writer/react/index.tsx:291-328](writer/react/index.tsx)) for in-editor same-variable highlighting. Add matching **external** props, following the exact pattern of the existing `externalSelection`/`externalHoverRange` effects (~622-641):

- `externalHoverOccurrences?: readonly {start,end}[]` + `externalHoverOccurrencesSignal?: number`
- `externalSelectionOccurrences?: readonly {start,end}[]` + `externalSelectionOccurrencesSignal?: number`

### 3c. Controller state (`trinity/rewrite/play/index.ts`)

Add to `TrinityRewritePlayController`:

- `activeHoverVar: string | null`, `hoverEpoch: number`
- `activeSelectVar: string | null`, `selectEpoch: number`
- Commands: `setLhsGraphHover`/`setRhsGraphHover` (`{id}` -> resolve var via `rewriteVarForNodeId`), `setJackHover` (`{offset}` -> `jackSymbolAtOffset(jackQueryText, offset)`, only `kind === "variable"`), and `*GraphSelect`/`setJackSelect` mirrors for selection.
- Getters: `getLhsHoveredNodeIds()`/`getRhsHoveredNodeIds()` (via `rewriteNodeIdsForVar`), `getJackHoverOccurrencesJson()`/`getJackHoverSignal()` (via `jackVariableOccurrences`), and selection equivalents.

### 3d. Wire the hosts (`framework/product/playground/renderer/react/index.tsx`)

- LHS/RHS hosts (from step 1): add `hoveredId`/`onHover` and `selection`/`onSelect` wired to the new controller commands/getters.
- `TrinityRewriteJackSurfaceHost`: add `onHoverChange`, `onSelectionChange`, `externalHoverOccurrences(+Signal)`, `externalSelectionOccurrences(+Signal)` wired the same way.

## 4. Extend to Before/After bound-node highlight (stretch)

This needs one new, genuinely-shared capability since it does not exist yet: `TrinityCanvasProps` has no externally-controlled highlight channel today (only `onSelectionChange` fires _out_). The underlying shared engine already has an unused-for-this-purpose `BoardElementStyleKind::Highlighted` render style with its own theme colors ([mathematical/graph/port/directed/normal/lib.rs:939,1161,1171,1230](mathematical/graph/port/directed/normal/lib.rs)), currently only driven internally by area-select/link-compat previews.

- Add a `highlighted_ids: BTreeSet<String>` field + `set_highlighted_ids_json`/`highlighted_ids_json` wasm-bindgen pair to the shared `BoardHost` (same file), feeding `hovered_style_kind`-style resolution so `Highlighted` wins when an id isn't already `Selected`/`Hovered`.
- Thread it through `TrinityBridge` in [trinity/rewrite/engine/lib.rs](trinity/rewrite/engine/lib.rs) (mirrors existing `selected_node_ids_json`, ~756) and add `highlightedNodeIds`/`onHighlightedNodeIdsChange`-style prop to `TrinityCanvasProps` in [trinity/react/index.tsx:358](trinity/react/index.tsx).
- Add `rewriteLhsMatchQuery(lhsJson): string` in trinity-rewrite-react building `MATCH (pattern) [WHERE clause] RETURN <activeVar>`; call the existing `runJackOnFixture(fixtureJson, query)` ([trinity/react/index.tsx:134-138](trinity/react/index.tsx)) against Before/After fixtures — a bare-var `RETURN` already yields a `graphFixture` subgraph of exactly the bound node(s) ([trinity/jack/core/lib.rs:1409-1421](trinity/jack/core/lib.rs), `return_items_want_graph`/`collect_graph_entities`), so no new Rust query logic is needed there.
- Wire `TrinityRewriteBeforeSurfaceHost`/`AfterSurfaceHost` to pass the computed ids through the new prop.
- Known limitation to note in the ticket: only works for variables that exist in the LHS-matched pre-image (RHS-only `CREATE` variables have no Before-side node and simply highlight nothing).

## 5. Validate

- Extend existing vitest suites in place (no new test files, per repo rule): `trinity/rewrite/react` (var<->node mapping, label formatting round-trip), `trinity/rewrite/play` (hover/select commands + getters), `writer/react` (new external occurrence props), `puzzle/2d/react` if `Puzzle2dCanvasProps` needs any adjustment.
- `cargo test` for `mathematical_graph_port_directed_normal`, `trinity_rewrite_engine`, `trinity_react` wasm glue as touched.
- Rebuild trinity/puzzle2d WASM; re-screenshot `:6056` to confirm LHS/RHS now render nodes with clear `var : Kind` labels and that hovering a Jack query variable highlights the matching LHS/RHS node (and vice versa).

## 6. Ticket workflow

Read `repo://goals`, reopen `GENERALIZE-TRINITY-DIRECTED-PORT-GRAPH` (same technology area) or open a new ticket if that one's summary doesn't fit, do the work inside it, close with a summary listing every touched file.
