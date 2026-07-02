# Declarative Spatial Actions

## Summary
Refactor `spatial` actions so action definitions are JSON data only. The TypeScript runtime will only interpret action documents; no action asset may contain executable code or register a `run` function. Built-in actions will call named kernel capabilities, assign action-local variables, patch context, and return data/diffs through declarative steps.

## Key Changes
- Open/reopen a repo ticket under the Running Sketchpad spatial work before implementation, then close it with touched files listed.
- Add `spatial.action/v1` schema in `spatial/schema/json/action.json`.
- Add JSON action assets under existing `spatial/asset/extension/builtin/action/**`.
- Replace `ActionFn`/code-backed built-ins in `spatial/js/core/index.ts` with:
  - `ActionSpec` parser/catalog loader.
  - `DeclarativeActionRuntime`.
  - Data-only execution steps: `let`, `setContext`, `deleteContext`, `kernel.call`, `return`, `guard`.
- Keep interactions unchanged at the call site: existing `commit.operation.action` and transition `op: "action"` still reference action IDs.
- Extend `SpatialKernel`/`SpatialPreviewKernel` contracts so every currently code-backed geometry behavior is available as a named kernel capability, including box helpers, transforms, curve creation, measurement, anchor placement, selection operations, and command finalization.
- Move geometry/model-diff construction logic out of action definitions and into kernel capability implementations in `@spatial/js-kernel-brepjs` or existing kernel-facing helpers.
- Preserve existing user edits in `spatial/js/core/index.ts`; do not restore removed tests or unrelated code.

## Action Document Shape
Each action JSON document will use this contract:

```json
{
  "schema": "spatial.action",
  "id": "transform.move",
  "version": "1.0.0",
  "label": "Move",
  "parameters": {
    "targets": { "kind": "unknown" },
    "from": { "kind": "vec3" },
    "to": { "kind": "vec3" }
  },
  "variables": [
    { "name": "constrainedTo", "value": { "kind": "kernel.call", "function": "constrainMovePoint", "args": {} } }
  ],
  "steps": [
    { "op": "kernel.call", "function": "transformMoveDiff", "args": {}, "assignTo": "diff" },
    { "op": "return", "diff": { "kind": "var", "name": "diff" } }
  ]
}
```

Expressions remain declarative JSON. Add a `kernel.call` expression form only for named kernel functions; no arbitrary JS, lambdas, imports, script strings, or expression bodies are allowed.

## Test Plan
- Extend existing `spatial/js/core/index.ts` tests only.
- Add schema/parser tests that reject action documents containing legacy executable fields such as `run`, `code`, `function`, or unknown step ops.
- Add catalog tests proving every built-in action ID resolves from JSON assets and `ActionRegistry.withBuiltins()` contains no code-backed built-ins.
- Keep existing interaction e2e coverage: `primitive.box`, transforms, curves, measures, selection, anchor creation, and command interactions must still commit successfully.
- Run with `bun nx run @spatial/js-core:test` and `bun nx run @spatial/js-kernel-brepjs:test`.

## Assumptions
- New JSON files under `spatial/asset/extension/builtin/action/**` are allowed because you selected JSON Assets as the storage model.
- Runtime interpreter code is allowed; the “no code” rule applies to action definitions, not to the generic action interpreter or kernel implementations.
- “Use functionality from the kernel” means all geometry/model mutations happen via named `SpatialKernel`/`SpatialPreviewKernel` capabilities, not inside action definitions.
