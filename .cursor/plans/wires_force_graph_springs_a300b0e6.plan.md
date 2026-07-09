---
name: wires force graph springs
overview: Fix the WIRES force-directed graph so edges (relationships) actually become spring constraints. The Rust layout engine currently resolves edges only via handle ids, but WIRES (graphPortMode "normal") uses node-id edges with no handles, so all edges are dropped and the graph has no cohesion.
todos:
 - id: force-edge-fallback
   content: Add node-id fallback to force_graph edge resolution in mathematical/graph/port/directed/lib.rs (~636-641), resolving via handle_to_node then id_to_index.
   status: completed
 - id: tree-edge-fallback
   content: Add the same node-id fallback to hierarchical_tree edge resolution (~1121-1135), validating against id_to_node.
   status: completed
 - id: shared-helper
   content: Optionally extract a shared resolve_endpoint_node_id helper in a region to avoid duplication.
   status: completed
 - id: tests
   content: Extend puzzle/2d/rs/lib.rs test module with normal-mode (no-handle, node-id edge) force-graph and hierarchical-tree regression tests.
   status: completed
 - id: validate
   content: Run the puzzle 2d Rust tests and confirm WIRES springs now apply and the graph settles instead of flying out.
   status: completed
isProject: false
---

# Fix WIRES Force Graph: Resolve Node-Id Edges

## Problem

WIRES uses `@puzzle/2d` in `graphPortMode: "normal"`: nodes have empty `handles` and edges reference node ids directly (see [metabolism.wires.json](reasoning/mindmap/wires/fixture/metabolism.wires.json) edges and [wiresFixtureBoard](reasoning/mindmap/wires/react/index.ts)).

The layout engine in [mathematical/graph/port/directed/lib.rs](mathematical/graph/port/directed/lib.rs) resolves edge endpoints exclusively through the handle->node map, so for WIRES every edge is skipped, `edge_pairs` is empty, and no spring forces are applied. Only repulsion + gravity run, so the graph has no structure and drifts/expands off-screen instead of laying out.

## Fix

Make edge-endpoint resolution support both port models at the root: try `handle_to_node` (ported graphs), else treat the id as a node id directly (normal graphs), then validate against the node-id index. This matches the documented "normal directed graph host: no handles, edges reference node ids."

### 1. Force-graph edge resolution

In `force_graph::apply_force_graph_layout_to_fixture_v1_value` ([mathematical/graph/port/directed/lib.rs](mathematical/graph/port/directed/lib.rs) ~636-641), replace the handle-only lookup with a fallback:

```rust
let a = handle_to_node.get(src_h).map(String::as_str).unwrap_or(src_h);
let b = handle_to_node.get(tgt_h).map(String::as_str).unwrap_or(tgt_h);
if a == b { continue; }
let Some(&ia) = id_to_index.get(a) else { continue; };
let Some(&ib) = id_to_index.get(b) else { continue; };
```

`id_to_index` already maps node ids, so handle-based (ported) and node-id (normal) edges both resolve; invalid ids still get dropped.

### 2. Hierarchical-tree edge resolution

Apply the same fallback in `hierarchical_tree` ([mathematical/graph/port/directed/lib.rs](mathematical/graph/port/directed/lib.rs) ~1121-1135), resolving via `handle_to_node` then raw id, and skipping ids not present in `id_to_node`. This fixes WIRES tree mode (same root bug).

### 3. Optional shared helper

Introduce a small private helper (e.g. `resolve_endpoint_node_id`) within a region to avoid duplicating the fallback logic across both layout functions, per the regions/single-source convention.

## Tests

Extend the existing test module in [puzzle/2d/rs/lib.rs](puzzle/2d/rs/lib.rs) (do not create new files):

- Force-graph: a normal-mode fixture (nodes with no `handles`, edges using node ids as `source`/`target`) where two connected, far-apart nodes move toward each other / settle near `idealEdgeLength` (asserts springs now apply). Mirror the style of the existing `redraw_force_graph_*` tests but without handles.
- Hierarchical-tree: a normal-mode parent/child node-id fixture producing the expected parent-above-child layering.

## Validation

- Run the Rust tests for the puzzle 2d crate (the crate exposing `boardRedrawLayoutFixtureJson`) and confirm new + existing force-graph/tree tests pass.
- Confirm WIRES play runtime behavior: connected identities now attract and the graph settles centered instead of flying out (verify via the WIRES play surface / live force loop).

## Notes

- Gravity center (`camera.x/y`) is already the world-space viewport center (`worldToScreen` in [puzzle/2d/react/index.tsx](puzzle/2d/react/index.tsx)); no change there.
- This is a root-level engine fix (no adapter/synthetic-handle workaround), consistent with the repo's clean-mechanism rule.
