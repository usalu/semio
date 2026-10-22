---
name: Generalize Trinity Off DAG
overview: Trinity is documented as a general directed property port graph technology, but its canvas host copy-pasted DAG-only assumptions. Fix the concrete DAG leaks in trinity, and generalize the shared layout math so it is no longer gated to a hardcoded per-technology allowlist.
todos:
 - id: remove-enforce-acyclic
   content: Remove enforce_acyclic=true from TrinityHost::rebuild_engine in trinity/rewrite/engine/lib.rs
   status: completed
 - id: generalize-schema-allowlist
   content: Replace fixture_schema_ok's hardcoded 2-item match in mathematical/graph/normal/undirected/lib.rs with a compile-time schema array including trinity.graph/v1
   status: completed
 - id: trinity-force-reorganize
   content: Implement real force-directed reorganize() in trinity/rewrite/engine/lib.rs via an adapter into mathematical_graph_port_directed::force_graph
   status: completed
 - id: generalize-recompute-derived
   content: Generalize recompute_derived in trinity/ram/lib.rs to multi-source traversal covering all weakly-connected components, add disconnected-component test
   status: completed
 - id: validate-and-ticket
   content: Run cargo tests across affected crates, verify TS playgrounds still build, and do the work inside a repo ticket (reopen or open new) closed with a summary
   status: completed
isProject: false
---

# Generalize Trinity To Directed Port Graphs (Not DAG)

## Root cause

Trinity ([trinity/AGENTS.md](trinity/AGENTS.md)) is explicitly "directed property port graphs where edges connect on ports" — no acyclicity requirement. It correctly reuses the general `mathematical_graph_port_directed::BoardEngine` (same engine `mathematical/graph/port/directed/normal` uses for puzzle-2d), but `TrinityHost` was bootstrapped by copying patterns from the DAG board host, carrying over two DAG-only assumptions into `trinity/rewrite/engine/lib.rs`:

```550:553:trinity/rewrite/engine/lib.rs
    fn rebuild_engine(&mut self) {
        self.graph.recompute_derived();
        self.engine = TrinityBoardEngine::new();
        self.engine.enforce_acyclic = true;
```

```435:440:trinity/rewrite/engine/lib.rs
    pub fn reorganize(&mut self) {
        for (i, node) in self.graph.nodes.values_mut().enumerate() {
            node.x = i as f64 * 140.0 - 200.0;
            node.y = (i % 3) as f64 * 100.0;
        }
```

By contrast `mathematical/graph/port/directed/normal/lib.rs` (`BoardHost`, puzzle-2d's general board) never sets `enforce_acyclic` — it stays at `GraphEngine`'s default `false` ([mathematical/graph/lib.rs](mathematical/graph/lib.rs) line 872). So today, Jack's `CREATE`/`MERGE` clauses can build a cycle in `trinity_ram::Graph` with zero restriction, but dragging a wire to form that same cycle on the canvas is silently rejected — an inconsistency that only exists because trinity's canvas thinks it's a DAG.

Separately, `recompute_derived` in [trinity/ram/lib.rs](trinity/ram/lib.rs) (lines 258-298) derives a `flatPosition` by walking from a single root; nodes unreachable from that root (a second weakly-connected component) never get a `flatPosition` at all. General directed port graphs are not required to be single-rooted trees, so this needs multi-root generalization.

Also uncovered: the shared force-directed layout math is gated by a hardcoded two-technology schema allowlist instead of a proper compile-time-declared, extensible list:

```104:106:mathematical/graph/normal/undirected/lib.rs
    fn fixture_schema_ok(schema: Option<&str>) -> bool {
        matches!(schema, Some("puzzle.2d.fixture") | Some("reasoning.mindmap.fixture"))
    }
```

This blocks trinity (and any future technology) from reusing the generic layout without editing product-specific string literals in an ad hoc match arm.

## Changes

### 1. Remove the acyclic constraint from `TrinityHost`

[trinity/rewrite/engine/lib.rs](trinity/rewrite/engine/lib.rs) `rebuild_engine`: delete `self.engine.enforce_acyclic = true;`. `GraphEngine::default()` already leaves it `false`, matching `normal::BoardHost`. This unblocks cycles/feedback wiring on the trinity canvas, consistent with Jack's own query semantics and `trinity/AGENTS.md`.

### 2. Generalize the force-graph layout schema gate

[mathematical/graph/normal/undirected/lib.rs](mathematical/graph/normal/undirected/lib.rs): replace the inline two-item `matches!` with a single canonical compile-time array of accepted board-fixture schemas, and add `trinity.graph/v1`:

```rust
const FORCE_GRAPH_COMPATIBLE_SCHEMAS: &[&str] = &[
    "puzzle.2d.fixture",
    "reasoning.mindmap.fixture",
    "trinity.graph",
];

fn fixture_schema_ok(schema: Option<&str>) -> bool {
    matches!(schema, Some(s) if FORCE_GRAPH_COMPATIBLE_SCHEMAS.contains(&s))
}
```

Update the `Err(...)` message to be built from the array instead of a hardcoded string. This turns a closed, per-technology special case into a single, extensible, compile-time-declared list — any future technology adds one line instead of editing match arms.

### 3. Wire trinity's `reorganize()` into the shared force-directed layout

[trinity/rewrite/engine/lib.rs](trinity/rewrite/engine/lib.rs): replace the dummy grid stub with a real adapter that reuses `mathematical_graph_port_directed::force_graph` (already a dependency of this crate):

- Build a `serde_json::Value` shaped as the generic ported board fixture: `{"schema": trinity_ram::GraphFixture::SCHEMA, "nodes": [{"id","x","y","width","height","shape":"rectangle","handles":[{"id": port_key(node.id, port.id)}, ...]}], "edges": [{"source","target"}]}` — trinity edges already store `source`/`target` as `"nodeId:portId"` strings, so they map 1:1 onto the expected handle-id fields.
- Call `mathematical_graph_port_directed::force_graph::apply_force_graph_layout_to_fixture_v1_value(&mut value, &ForceGraphLayoutOptions::default())`.
- Copy the resulting `x`/`y` back onto `self.graph.nodes`, then `self.rebuild_engine()`.

A spring/force layout works regardless of cycles or multi-parent convergence (unlike DAG's Buchheim `hierarchical_tree`, which requires a rooted tree), so this is the mathematically correct generalization for an arbitrary directed port graph rather than either the current no-operation stub or a borrowed tree layout.

### 4. Generalize `recompute_derived` to multiple roots/components

[trinity/ram/lib.rs](trinity/ram/lib.rs) `recompute_derived`: after traversing from the primary root (`root_node_id` or first node, unchanged), repeat the same edge-offset walk seeded at `(0.0, 0.0)` from any remaining un-derived node, until every node has a `flatPosition` — i.e. multi-source traversal over all weakly-connected components instead of stopping after the first. Traversal order stays deterministic via the existing `BTreeMap` iteration. Add a test with two disconnected node/edge groups asserting both get a derived `flatPosition`.

### 5. Validate

- `cargo test` for `trinity_ram`, `trinity_rewrite_engine`, `mathematical_graph_normal_undirected`, `mathematical_graph_port_directed`, plus existing puzzle-2d/reasoning-mindmap suites to confirm the widened schema list is purely additive (no behavior change for their fixtures).
- Confirm `trinity/jack/play` and `trinity/rewrite/play` still build against the unchanged public `reorganize(&mut self)` signature.

### Ticket workflow

Work happens inside a `.repo/🎫️` ticket per repo rules — check `repo://goals` and existing tickets (e.g. `IMPLEMENT-TRINITY-TECHNOLOGY`) for one to reopen before opening a new one, then close it with a summary listing every file touched.
