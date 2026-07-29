# 🎬 Media-flow planner fixtures

Scripted `OsWorkflow` + dirty-instance-set vectors that keep `plan_workflow` (Rust,
`framework/product/os/core/rs/lib.rs`) and `planWorkflow` (TS twin, `framework/product/os/core/js/index.ts`)
in lockstep. Both sides replay every `*.json` file here and assert an **exact-order** match against
`expectedDeliveries` — both implementations are deterministic (topological DFS driven purely by
`graph.nodes`/`graph.edges` insertion order, no hashmap-iteration-order dependence), so exact order is a
meaningful assertion, not a flaky one.

- **Rust** — `framework/product/os/core/rs/lib.rs` `workflow::tests::workflow_fixtures_match_expected_deliveries`.
- **TypeScript** — `framework/product/os/core/js/index.ts`'s `import.meta.vitest` block, `"planWorkflow matches shared fixtures"`.

## Format (`WorkflowFixture`)

```jsonc
{
  "name": "single-edge",
  "graph": { "schema": "s.workflow", "nodes": [ /* OsWorkflowNode[] */ ], "edges": [ /* OsWorkflowEdge[] */ ] },
  "dirtyInstanceIds": ["app-1"],
  "expectedDeliveries": [ /* WorkflowDelivery[], in the exact order plan_workflow/planWorkflow must produce */ ]
}
```

All keys are camelCase, matching `OsWorkflow`/`OsWorkflowNode`/`OsWorkflowEdge`/`OsMediaPort`/
`WorkflowDelivery`'s `#[serde(rename_all = "camelCase")]` on the Rust side.
