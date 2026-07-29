---
name: Unify Graph DSL Syntax
overview: Unify sigils across Jack and a new compact "wire literal" notation (`:` kind, `.` property, `@` port, `->` directed edge, `-` undirected edge); make flow, sequence, puzzle/2d (wires), and s media graph all compile their fixtures into an enriched, semantically-complete DAG with a read-only writer window; and give neural a real adapter that consumes that compiled DAG as flow's actual execution input, replacing flow's bespoke tree builder.
todos:
 - id: jack-ports-undirected
   content: Extend Jack grammar (jack_impl.rs) with @port pattern syntax and undirected - edges, including executor matching and completions/hover
   status: completed
 - id: wire-literal-notation
   content: Add new wire.rs module to mathematical/graph/dsl with wire-literal printer/parser (WireNode/WireEdge), plus TS tokenizer in writer/core for a new "wire" languageId
   status: completed
 - id: dag-fixture-schema
   content: Add operator_kind + properties (PropertyBag) fields to DagNodeSpec and DagFixtureEdge in mathematical/graph/port/directed/dag/lib.rs, and a dag_fixture_to_wire_literal helper
   status: completed
 - id: flow-compile-dag-window
   content: Populate operator_kind/properties in flow's build_dag_fixture_v1, add compiled_wire_literal(), and a flow-compiled-dag writer window in flow/play
   status: completed
 - id: neural-dag-adapter
   content: Create neural/dag crate with tree_from_dag() adapter; switch flow/core evaluate_internal() to use it instead of tree_from_fixture(), with output-parity regression tests
   status: completed
 - id: sequence-compiled-dag-window
   content: Add operator_kind/properties to sequence's build_dag_fixture() and a sequence-compiled-dag writer window
   status: completed
 - id: puzzle2d-compiled-dag-window
   content: Add a 2d-compiled-dag writer window to puzzle/2d/play (inherited by wires)
   status: completed
 - id: s-compiled-dag-window
   content: Add an s-compiled-dag writer window to s/play showing the media graph as wire-literal text
   status: completed
 - id: verify-regressions-v2
   content: Run full regression suite (Rust + vitest) and manual flow evaluate() parity check before/after neural adapter swap
   status: completed
isProject: false
---

# Unify Graph DSL Syntax and Compile Flows into a Neural-Importable DAG

## Confirmed scope (from clarifying questions)

- Both: extend Jack's `MATCH` grammar with `@port` and undirected `-` edges, AND add a separate compact "wire literal" notation for compiled DAG text.
- Broad scope: flow, sequence, puzzle/2d (wires inherits), and s media graph all compile to this unified DAG notation and get a read-only writer window.
- Full rearchitecture: neural gets a real adapter that consumes the compiled DAG as its actual execution input, replacing today's per-technology tree-building.

## Key assumption to flag

Only `flow` has a natural neural-execution mapping today. `sequence` compiles to an imperative `Path`/text via [sequence/core/lib.rs](sequence/core/lib.rs) `build_path()`/`compile_text()`, with nested control-flow bodies (`if`/`while`/`repeat`) that do not reduce to a flat dataflow graph. `puzzle/2d`/`wires` and `s` media graph are not computational engines at all. Therefore:

- All five technologies get: compile fixture into an enriched `dag.fixture` (semantically complete, not just visual) then a "Compiled DAG" read-only writer window in the new wire-literal notation (display/interop).
- Only `flow`'s actual execution swaps to run through the new DAG-to-neural adapter. `sequence`/`puzzle`/`s` keep their existing execution models unchanged; their DAG output is display/interop only for now.

If this narrower execution scope is wrong, correct it before implementation starts (it changes Phase 5 significantly).

## Current state (confirmed by exploration)

- **Jack grammar** ([mathematical/graph/dsl/jack_impl.rs](mathematical/graph/dsl/jack_impl.rs)): `(var:Kind)-[var:Kind]->(var:Kind)` only. No `@` token, no undirected edges (`PatternEdge.directed` always `true`).
- **Port ID formats already in use are inconsistent**: trinity/DAG/S use `nodeId:portId` (colon), puzzle/2d mixes `.`/`:`. Introducing `@` as the port sigil in the new syntax avoids colliding with Jack's existing `:Kind` usage, but the _runtime_ `nodeId:portId` convention is untouched by this plan (only the new DSL text notation uses `@`).
- **Neural** ([neural/engine/lib.rs](neural/engine/lib.rs)): `Tree { neurons, synapses }`, `Neuron { id, kind, params, tree }`, `Synapse { id, from, to, from_port, to_port }`. `Neuron.kind` is dispatched verbatim as an operator id via `Registry::dispatch`. `neural_engine`'s `Cargo.toml` has **no** dependency on any graph/DAG/manifest crate today.
- **Flow execution** ([flow/core/lib.rs](flow/core/lib.rs)): `tree_from_fixture()` hand-builds a neural `Tree` from `FlowFixture.widgets`/`.synapses`, with widget-kind-specific match arms (`Widget::Neuron` -> `neuronKind` verbatim, `Widget::InputSlider` -> `"core.number"`, etc.). Separately, `build_dag_fixture_v1()` projects the same fixture into `dag.fixture` for canvas display only — it does **not** carry the operator kind or params needed to reconstruct execution (`DagNodeKind::Computation` has ports but no operator id field).
- **`DagNodeSpec`** ([mathematical/graph/port/directed/dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs) line 665): `{ id, name, abbreviation, icon, x, y, width, height, kind: DagNodeKind }` — `DagNodeKind` is a purely visual enum (`Computation`, `Slider`, `Note`, `Cluster`, ...). No `properties`/operator-kind field exists yet. This is the concrete gap that must close for neural to import the DAG.
- **Sequence** ([sequence/core/lib.rs](sequence/core/lib.rs)): `build_dag_fixture()` is canvas-only (positions + generic `prev`/`next` ports); execution semantics live entirely in `build_path()`/`compile_text()` and are not representable as a flat DAG once control bodies nest.
- The prior `GENERALIZE-JACK-GRAPH-DSL` ticket already added `kind: Option<String>` + `properties: PropertyBag` to `GraphEngine<P,D>`'s runtime `Node`/`Handle`/`Edge` ([mathematical/graph/lib.rs](mathematical/graph/lib.rs)) and to `mathematical/graph/dsl` (`BoardQueryableGraph`, Jack executor). This plan builds directly on that foundation and reuses `mathematical_graph_manifest::PropertyBag`.

## Phase 1 — Extend Jack grammar with ports and undirected edges

File: [mathematical/graph/dsl/jack_impl.rs](mathematical/graph/dsl/jack_impl.rs)

- Lexer: add `Token::At` for `@`.
- `PatternNode` gains `port: Option<String>`; parser accepts `(var:Kind@port)` in addition to `(var:Kind)`.
- `PatternEdge`: parse a bare `-node` (no trailing `->`) as undirected (`directed: false`). Executor's `match_pattern`/`match_patterns` currently always treats edges as directed — implement undirected matching by trying both `(source,target)` orientations when `!directed`.
- `QueryableGraph`/`QueryableEdge` ([mathematical/graph/dsl/queryable.rs](mathematical/graph/dsl/queryable.rs)): add optional `source_port`/`target_port` so `@port` can filter matches; extend `BoardQueryableGraph::from_fixture_json` to parse existing `nodeId:portId`/`nodeId.portId` handle formats into this field.
- Extend completions/hover to suggest port kinds after `@` (from `GraphManifest.port_kinds`).
- Trinity's `TrinityQueryableGraph` ([trinity/jack/core/queryable.rs](trinity/jack/core/queryable.rs)) gains the same port fields (trinity already has real `Port`s on `Node`).
- Add Rust tests: `parse_match_with_port`, `parse_undirected_edge`, `run_undirected_query`, `run_port_filtered_query`.

## Phase 2 — New "wire literal" compiled-DAG text notation

New module: `mathematical/graph/dsl/wire.rs` (included from [mathematical/graph/dsl/lib.rs](mathematical/graph/dsl/lib.rs) the same way `jack_impl.rs` is).

- Grammar: newline-separated statements. A node declaration is `id:Kind` (optionally with inline properties, e.g. `id:Kind{value: 3}` using `.` for later property access elsewhere in the language family). A connection is `id1:Kind1@port1->id2:Kind2@port2` (directed) or `id1:Kind1@port1-id2:Kind2@port2` (undirected), matching the example `p:Puzzle3d@3d->s:Shooting@3d`.
- `pub fn wire_literal_from_dag(nodes: &[WireNode], edges: &[WireEdge]) -> String` — pretty-printer, operating on small neutral structs (`WireNode { id, kind, port: Option<String>, properties }`, `WireEdge { from, from_port, to, to_port, directed }`) so this module has no dependency on any specific `DagFixture` type.
- `pub fn dag_from_wire_literal(text: &str) -> Result<(Vec<WireNode>, Vec<WireEdge>), String>` — parser (round-trip target for future authoring, not required for Phase 4/5 which only need the printer direction).
- Register a `"wire"` `languageId` in writer, mirroring how `"jack"` was registered: a minimal TS tokenizer (`tokenizeWireSource`) in [writer/core/index.ts](writer/core/index.ts) for the writer canvas to syntax-highlight it (reuse Jack's token classes where possible).
- Rust tests: `wire_literal_roundtrip_simple`, `wire_literal_undirected`, `wire_literal_with_properties`.

## Phase 3 — Carry operator kind and properties on the shared DAG fixture

File: [mathematical/graph/port/directed/dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs)

- Add `operator_kind: Option<String>` and `properties: PropertyBag` (reuse `mathematical_graph_manifest::PropertyBag`) to `DagNodeSpec`, alongside the existing visual `kind: DagNodeKind`. `operator_kind` is the semantic/execution kind (e.g. `"math.add"`, `"core.number"`); `DagNodeKind` stays purely visual (ports/layout for canvas rendering).
- Add `properties: PropertyBag` to `DagFixtureEdge` for edge-level metadata.
- Update all `DagNodeSpec` builders (`::computation`, `::cluster`, etc.) to accept/default these new fields; thread through `DagHost` construction/round-trip (`sync_descriptor` equivalent) so nothing is silently dropped.
- Add a `dag_fixture_to_wire_literal(fixture: &DagFixture) -> String` helper in the dag crate (or in `mathematical_graph_dsl`, taking `DagFixture` by value/reference) that maps `DagNodeSpec`/`DagFixtureEdge` into Phase 2's `WireNode`/`WireEdge` and calls `wire_literal_from_dag`.

## Phase 4 — Flow compiles to the enriched DAG and gets a Compiled DAG writer window

File: [flow/core/lib.rs](flow/core/lib.rs)

- `build_dag_fixture_v1()`: populate the new `operator_kind` (the widget's `neuronKind`, or the translated input-kind such as `"core.number"` for `InputSlider`) and `properties` (widget params) on each `DagNodeSpec`, and `properties` on each `DagFixtureEdge` if any synapse-level metadata exists.
- Add `pub fn compiled_wire_literal(&self) -> String` calling Phase 3's helper.
- [flow/play/index.ts](flow/play/index.ts): add a new `flow-compiled-dag` window kind (alongside the existing `flow-jack` window added in the prior ticket) with a read-only `WriterCanvas` surface (`languageId: "wire"`), following the same `buildWriterWindowBody`/`WindowKindRuntime` pattern used for `flow-jack`.

## Phase 5 — Neural DAG adapter and flow execution swap

New crate: `neural/dag` (depends on `neural_engine` + `mathematical_graph_manifest`; explicitly **not** on `mathematical_graph_port_directed_dag`, to keep `neural_engine` decoupled from any specific graph-engine implementation per the "external deps behind an interface" rule — it consumes Phase 2's neutral `WireNode`/`WireEdge` shape).

- `pub fn tree_from_dag(nodes: &[WireNode], edges: &[WireEdge]) -> Tree` — a generalized version of flow's `tree_from_fixture`, purely data-driven by `operator_kind`/`properties`/edges (no widget-kind match arms).
- [flow/core/lib.rs](flow/core/lib.rs) `evaluate_internal()`: switch from `tree_from_fixture(&self.fixture, ...)` to building the enriched dag fixture (Phase 4) and calling `neural_dag::tree_from_dag(...)`. Remove the widget-specific match arms once behavior parity is verified, since input-widget translation and `Cluster` nested-tree recursion are now encoded via `operator_kind` + a nested wire-literal/DAG on `Cluster` nodes.
- Regression gate: flow_core's existing `evaluate()`-oriented tests must produce **identical outputs** before and after the swap (run against fixtures using `math.add`, `core.number`, and at least one `Cluster`).

## Phase 6 — Apply "compile to DAG + writer window" to sequence, puzzle/2d (wires), and s (display/interop only)

- **sequence**: [sequence/core/lib.rs](sequence/core/lib.rs) `build_dag_fixture()` gains `operator_kind` (`step.kind`) + `properties` (step params) for its flat top-level DAG. Add a `sequence-compiled-dag` writer window in [sequence/play/index.ts](sequence/play/index.ts) alongside the existing `sequence-script` and `sequence-jack` windows. Explicit constraint documented in code comments: nested control bodies are not represented (flat DAG only, matching today's canvas projection) — the imperative `Compiled Script` window remains the source of truth for control flow.
- **puzzle/2d** ([puzzle/2d/play/index.ts](puzzle/2d/play/index.ts), inherited by [reasoning/mindmap/wires/play/index.ts](reasoning/mindmap/wires/play/index.ts)): add a `2d-compiled-dag` writer window showing the wire-literal text of the current puzzle fixture.
- **s** ([s/play/index.ts](s/play/index.ts)): add an `s-compiled-dag` writer window showing the wire-literal text of the media graph (nodes = app instances with `operator_kind = pluginId`, edges = media graph edges).
- All wired via the same `buildWriterWindowBody`/`WindowKindRuntime`/`createJackPlayWindowEngagement`-style helpers established for the Jack windows in the prior ticket — introduce an equivalent `createCompiledDagPlayWindowEngagement` if a command-line engagement is required (these windows are read-only, so likely no engagement input is needed, similar to `sequence-script`).

## Phase 7 — Regression and verification

- `cargo test -p mathematical_graph_dsl -p mathematical_graph_port_directed_dag -p neural_engine -p neural_dag -p flow_core -p sequence_core -p trinity_jack`
- `bunx vitest run` in `flow/play`, `sequence/play`, `puzzle/2d/play`, `s/play`, `mathematical/graph/port/directed/dag/play`
- Manual/scripted parity check: evaluate a flow fixture containing `math.add`, `core.number` (slider), and a `Cluster` widget before and after the Phase 5 swap, diff the resulting output dictionaries.

## Todos summary

Phases 1-3 (syntax + fixture schema) should land and be verified before Phase 4/5 (flow compile + neural swap) begins, since those depend on the enriched `DagNodeSpec`. Phase 6 (sequence/puzzle/s display windows) can proceed in parallel with Phase 5 once Phase 3 lands, since it only needs the enriched fixture schema, not the neural adapter.
