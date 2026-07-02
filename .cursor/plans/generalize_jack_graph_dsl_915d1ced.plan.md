---
name: Generalize Jack Graph DSL
overview: Generalize the existing Trinity "Jack" query DSL down into the shared `mathematical/graph` foundation so every graph framework (flow, sequence, dag, puzzle/2d, puzzle/3d, puzzle/5d, wires, s) can run the same Cypher-like query language against its own graph; add a writer fixture proving it works on a non-Trinity domain; and give every graph playground a new "Jack" window kind with a rewrite-style bidirectional hover/selection bridge.
todos:
  - id: phase1-property-model
    content: Elevate kind+properties onto GraphEngine<P,D> Node/Handle/Edge in mathematical/graph/lib.rs, preserving user_data through sync_descriptor
    status: completed
  - id: phase2-dsl-crate
    content: Extract shared mathematical/graph/dsl crate with QueryableGraph trait + generalized Jack AST/parser/executor/LSP-helpers
    status: completed
  - id: phase2-graphengine-impl
    content: Implement QueryableGraph once for GraphEngine<P,D>, covering flow/sequence/dag/puzzle2d/wires
    status: completed
  - id: phase2-trinity-thin
    content: Shrink trinity/jack/core/lib.rs to a thin specialization re-exporting the shared DSL + trinity_ram QueryableGraph impl + TrinityGraphOp mutations
    status: completed
  - id: phase2-lsp-domain
    content: Generalize trinity/jack/lsp to accept a graphDomain selector and dispatch to the right adapter
    status: completed
  - id: phase3-puzzle3d-adapter
    content: Implement QueryableGraph adapter for puzzle/3d's manifest-driven entity model
    status: completed
  - id: phase3-s-adapter
    content: Implement QueryableGraph adapter for s/core's SMediaGraph
    status: completed
  - id: phase4-writer-fixture
    content: Add writer/fixture/dag.jack.writer.json demonstrating the DSL against a non-Trinity domain
    status: completed
  - id: phase5-shared-hub
    content: Extract shared Jack hover/selection bridge helper for reuse across playground controllers
    status: completed
  - id: phase5-flow-jack
    content: Add Jack window kind + hover bridge to flow/play
    status: completed
  - id: phase5-sequence-jack
    content: Add Jack window kind + hover bridge to sequence/play
    status: completed
  - id: phase5-dag-jack
    content: Add Jack window kind + hover bridge to mathematical/graph/port/directed/dag/play
    status: completed
  - id: phase5-puzzle2d-jack
    content: Add Jack window kind + hover bridge to puzzle/2d/play (inherited by wires)
    status: completed
  - id: phase5-puzzle3d-jack
    content: Add Jack window kind + hover bridge to puzzle/3d/play
    status: completed
  - id: phase5-puzzle5d-jack
    content: Add Jack window kind + hover bridge to puzzle/5d/play
    status: completed
  - id: phase5-s-jack
    content: Add Jack window kind + hover bridge to s/play
    status: completed
  - id: verify-regressions
    content: Run existing trinity/jack, trinity/rewrite, and all touched playground test suites to confirm no regressions
    status: completed
isProject: false
---

# Generalize Jack Graph DSL Across All Graph Frameworks

## Current state (confirmed by exploration)

- **Jack** (`trinity/jack/core/lib.rs`) is a full Cypher-inspired DSL (lexer, AST, recursive-descent parser, `MATCH`/`WHERE`/`RETURN`/`CREATE`/`DELETE`/`SET`/`MERGE`, LSP hover/completion/lint/format) but is hard-wired to `trinity_ram::Graph` (`Node`/`Edge`/`Port` with typed `PropertyBag`, `kind: String`).
- **`mathematical/graph`** (`GraphEngine<P,D>` in `mathematical/graph/lib.rs`) is the shared foundation for flow, sequence, dag, puzzle/2d, wires — but it is a pure geometry/interaction engine: `Node`/`Handle`/`Edge` have **no `kind` and no properties**. Only the JSON descriptors (`NodeDescJson.node_kind`, `.user_data: Option<serde_json::Value>`) carry semantics, and `sync_descriptor()` **drops `user_data`** when populating board runtime types.
- `GraphManifest` (`mathematical/graph/manifest`) already defines the shared compile-time schema (`nodeKinds`/`edgeKinds`/`portKinds`/`wireKinds` + `PropertyDef`s) used by flow-dag, puzzle2d, wires, s-resources, nakagin (trinity) — this is the correct shared vocabulary source for `MATCH (a:Kind)`.
- `trinity/rewrite/play/index.ts` already implements the target UX pattern: a variable-centric hub (`activeHoverVar`/`activeSelectVar`/`hoverEpoch`) that bridges canvas hover ↔ Jack text occurrences ↔ other panes, using `jackSymbolAtOffset`/`jackVariableOccurrences` from `writer/core/index.ts` (these are already domain-agnostic text-level helpers — no change needed there).
- `writer/fixture/jack.writer.json` + `writer/manifest/languages.manifest.json` already register "jack" as a language; `trinity/jack/lsp` hard-loads Trinity fixtures only.

## Decisions (confirmed with user)

1. **One shared DSL**: generalize Jack itself rather than inventing a new language or duplicating one per framework. `trinity/jack` becomes a thin Trinity-specific specialization of a shared core.
2. **Scope**: all graph frameworks — flow, sequence, dag (`mathematical/graph/port/directed/dag`), puzzle/2d, puzzle/3d, puzzle/5d, wires, and s (media graph) — each gets the DSL and a new Jack-style window kind.

## Phase 1 — Elevate the property model into the shared graph engine

The cleanest single-source-of-truth fix: give `GraphEngine<P,D>` itself a runtime `kind: Option<String>` + `properties: PropertyBag` on `Node`/`Handle`/`Edge`, reusing `PropertyValue`/`PropertyBag` types from `mathematical_graph_manifest` (currently only conceptually defined for `trinity_ram`).

- `mathematical/graph/lib.rs`: add `kind`/`properties` fields to `Node`, `Handle`, `Edge<E>`; extend `Selection`/scene-json sync so `sync_descriptor()` preserves `node_kind`/`edge_kind`/`handle_kind`/`wire_kind` and promotes `user_data` (JSON) into typed `PropertyBag` (validated against the domain's `GraphManifest` where one is registered).
- `mathematical/graph/port/directed/lib.rs` and `.../normal/lib.rs`: remove the now-redundant `NodeData.node_kind`/`EdgeData.edge_kind` string duplication in favor of the engine-level fields (or keep as thin accessors) — single source of truth per the "no inconsistencies" rule.
- Doing this once at the `GraphEngine<P,D>` level automatically gives **flow, sequence, dag, puzzle/2d, wires** a queryable, manifest-typed graph — no per-framework property-model work needed for these five.
- `puzzle/3d` and `s` don't sit on `GraphEngine`; they get bespoke adapters in Phase 2.

## Phase 2 — Extract the shared DSL core + `QueryableGraph` trait

- New crate: `mathematical/graph/dsl` (core `lib.rs`) — move Jack's `Ast`, `Lexer`, `Parser`, `LanguageService` (hover/lint/format/completion), and executor regions out of `trinity/jack/core/lib.rs`, generalizing the executor to run against a new trait:
  ```rust
  pub trait QueryableGraph {
      fn node_ids(&self) -> Vec<String>;
      fn node_kind(&self, id: &str) -> Option<&str>;
      fn node_property(&self, id: &str, key: &str) -> Option<&PropertyValue>;
      fn edges(&self) -> Vec<(String, String, String)>; // (edgeId, sourceNodeId, targetNodeId)
      fn edge_kind(&self, id: &str) -> Option<&str>;
      fn manifest(&self) -> &GraphManifest;
      // + mutation hooks (create/delete/set) returning a generic GraphOp for CREATE/DELETE/SET/MERGE
  }
  ```
- `impl<P: GraphPortModel, D: Directedness> QueryableGraph for GraphEngine<P,D>` once in `mathematical/graph/dsl` (or `mathematical/graph/port/directed`) — this single impl covers flow, sequence, dag, puzzle/2d, wires.
- `trinity/jack/core/lib.rs` shrinks to: re-export the shared `run`/`run_json`/`complete`/`lint`/`hover`/`format` API from `mathematical_graph_dsl`, plus `impl QueryableGraph for trinity_ram::Graph` and Trinity-specific `TrinityGraphOp` emission for CREATE/SET/DELETE/MERGE. Existing public API signatures stay stable so `trinity/jack/lsp`, `trinity/rewrite`, `trinity/react`'s `runJackOnFixture` keep working unchanged.
- `trinity/jack/lsp/lib.rs`: generalize fixture loading to accept a `graphDomain` selector (`trinity` | `dag` | `puzzle2d` | `s-media-graph` | ...) that picks the right `QueryableGraph` adapter, so the same LSP binary can power query editors across domains (needed for Phase 4's per-playground Jack windows).
- Writer/TS side needs **no functional change** — `tokenizeJackSource`, `JackAstParser`, `jackSymbolAtOffset`, `jackVariableOccurrences` in `writer/core/index.ts` already operate purely on query text and are already domain-agnostic.

## Phase 3 — Bespoke adapters for non-`GraphEngine` domains

- `puzzle/3d`: implement `QueryableGraph` over its object/vortex/attraction/cable manifest-driven entity model (react/play layer, or a small new Rust/TS adapter next to `puzzle/3d/rs`).
- `s` (media graph): implement `QueryableGraph` over `s/core/index.ts`'s `SMediaGraph` (app instances as nodes, typed ports/edges) — likely a TS-side adapter since `s/core` is TypeScript, calling into `mathematical_graph_dsl`'s WASM for parsing but doing node/edge resolution in TS, or exposing a small Rust shim if `s/rs` exists.
- Each domain exposes one exported entry point, e.g. `run_dag_jack_query(fixtureJson, queryText)`, `run_puzzle2d_jack_query(...)`, `run_wires_jack_query(...)`, `run_puzzle3d_jack_query(...)`, `run_s_jack_query(...)` — mirroring the existing `runJackOnFixture` used by `trinity/react`.

## Phase 4 — Writer fixture proving generalization

- Add `writer/fixture/dag.jack.writer.json` (or similarly named) — a `writer.document` fixture with `languageId: "jack"` whose query text targets a **non-Trinity** domain, e.g. `MATCH (n:computation) RETURN n.id, n.label` against a DAG fixture, to prove the same language/editor now works outside Trinity.
- Wire it into `writer/play/fixture-slugs.ts` and the existing `import.meta.glob("../fixture/*.writer.json")` loader — no new loader code needed, just the new fixture file plus (if the LSP needs to know which domain/fixture to bind against) a `graphDomain`/`fixtureRef` field alongside `languageId` in the fixture JSON, consumed by `writer/play/index.ts`'s controller when spinning up the LSP worker.

## Phase 5 — New "Jack" window kind + bidirectional hover/selection per playground

Replicate the `trinity/rewrite` pattern (`activeHoverVar`, `hoverEpoch`, `subscribeSnapshot`, `jackSymbolAtOffset` bridge — see `trinity/rewrite/play/index.ts` lines ~259-267, 428-488, 645-667) in each playground's controller, adding a domain-specific `varForNodeId`/`nodeIdsForVar` mapper (built on that domain's Jack query entry point from Phase 2/3) plus a new `WindowKindRuntime` hosting an editable `WriterCanvas` (unlike rewrite's read-only derived query).

To avoid duplicating the ~8-method hub boilerplate 7 times (flow, sequence, dag, puzzle/2d [wires inherits], puzzle/3d, puzzle/5d, s), extract a shared helper/mixin (e.g. `createJackHoverBridge()`) into `framework/product/platform/core/index.ts` or a new shared module, used by every controller.

Per playground:
- **flow** (`flow/play/index.ts`): add `flow-jack` window kind; map flow widget/node ids ↔ Jack vars.
- **sequence** (`sequence/play/index.ts`): add `sequence-jack` window kind (replacing/joining the existing read-only "Compiled Script" plaintext window); map step ids ↔ vars.
- **dag** (`mathematical/graph/port/directed/dag/play/index.ts`): add `dag-jack` window kind.
- **puzzle/2d** (`puzzle/2d/play/index.ts`, inherited by `reasoning/mindmap/wires/play/index.ts`): add `2d-jack` window kind; lift hover from the React shell context (`Puzzle2dPlayShellValue`) into the controller hub so Jack can bridge to it, mirroring rewrite's LHS/RHS treatment.
- **puzzle/3d** (`puzzle/3d/play/index.ts`): add `puzzle-3d-jack` window kind; bridge existing `hoverFocus`/selection to Jack vars.
- **puzzle/5d** (`puzzle/5d/play/index.ts`): add `puzzle-5d-jack` window kind; bridge both `selected2d`/`selected3d` to the same var hub.
- **s** (`s/play/index.ts`): add `s-jack` window kind; bridge media-graph node/app-instance ids to vars.

Each requires: window kind registration + `buildWriterWindowBody`, a new surface host in `framework/product/playground/renderer/react/index.tsx` (`registerUiWriterSurfaceHost`) following `TrinityRewriteJackSurfaceHost`'s prop wiring (`externalHoverOccurrences`, `onHoverChange`, epoch signals), and controller command handlers (`setGraphHover`, `setJackHover`, `setGraphSelect`, `setJackSelect`).

## Execution notes

- Work happens under a single new ticket (e.g. `GENERALIZE-JACK-GRAPH-DSL`), goal `🎯trinity` or a new cross-cutting goal — confirm goal via `repo://goals` at ticket-open time.
- Given the size, phases 1-3 (DSL generalization) should land and be verified (existing `trinity/jack` tests + `trinity/rewrite` tests still green) before phase 5 (8 playground UIs) begins, since phase 5 depends on phase 2/3's per-domain query entry points.
- All existing Jack/Trinity call sites (`trinity/react`'s `runJackOnFixture`, `trinity/rewrite`, `trinity/jack/play`, `trinity/jack/lsp`) must keep working unchanged after the extraction — this is the regression bar for phases 1-2.
