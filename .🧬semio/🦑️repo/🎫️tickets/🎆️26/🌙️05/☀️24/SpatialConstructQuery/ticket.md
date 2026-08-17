# Spatial Construct Query

MCP `ticket_open` unavailable in this session; manual ticket for traceability.

## Scope

- `@spatial/js-query` with Chevrotain, construct language, planner, executor
- Core: `EntityMetadataStore`, `ConstructRunner`, `InteractionRuntime.query`, `KernelAdapter` adjacency, `Expr` + `evalExpr` field support
- `BrepjsKernel`: `adjacentCells`, `sharedFacesBetween`

## Status

Completed (tests green for `@spatial/js-query`, `@spatial/js-core`, `@spatial/js-kernel-brepjs`).

## Fixes in final pass

- `cstToExpr`: unwrap top-level `expr` CST node so `RETURN f.id`, `WHERE`, and `WITH` expressions are not compiled as empty constants.
- `ExprEnv.derived` + `readTopologyEntityProperty` cases for `surface` / `part` (including `id` and view fields via `DerivedViewService`); query executor passes `ctx.derived` into `rowVarsToEnv`.

## Files touched

- `spatial/js/query/index.ts`
- `spatial/js/core/index.ts`
- `spatial/js/kernel-brepjs/index.ts` (if present from earlier work)
- `spatial/js/package.json` (workspace)
- `.repo/🎫️/26/05/24/SpatialConstructQuery/debug-parse.ts` (temporary parse probe)
- `.repo/🎫️/26/05/24/SpatialConstructQuery/ticket.md`
