# 🎬 Media-flow planner fixtures

Scripted `OsMediaGraph` + dirty-instance-set vectors that keep `plan_media_flow` (Rust,
`framework/product/os/core/rs/lib.rs`) and `planMediaFlow` (TS twin, `framework/product/os/core/js/index.ts`)
in lockstep. Both sides replay every `*.json` file here and assert an **exact-order** match against
`expectedDeliveries` — both implementations are deterministic (topological DFS driven purely by
`graph.nodes`/`graph.edges` insertion order, no hashmap-iteration-order dependence), so exact order is a
meaningful assertion, not a flaky one.

- **Rust** — `framework/product/os/core/rs/lib.rs` `media_graph::tests::media_flow_fixtures_match_expected_deliveries`.
- **TypeScript** — `framework/product/os/core/js/index.ts`'s `import.meta.vitest` block, `"planMediaFlow matches shared fixtures"`.

## Format (`MediaFlowFixture`)

```jsonc
{
  "name": "single-edge",
  "graph": { "schema": "s.media-graph", "nodes": [ /* OsMediaGraphNode[] */ ], "edges": [ /* OsMediaGraphEdge[] */ ] },
  "dirtyInstanceIds": ["app-1"],
  "expectedDeliveries": [ /* MediaFlowDelivery[], in the exact order plan_media_flow/planMediaFlow must produce */ ]
}
```

All keys are camelCase, matching `OsMediaGraph`/`OsMediaGraphNode`/`OsMediaGraphEdge`/`OsMediaPort`/
`MediaFlowDelivery`'s `#[serde(rename_all = "camelCase")]` on the Rust side.
