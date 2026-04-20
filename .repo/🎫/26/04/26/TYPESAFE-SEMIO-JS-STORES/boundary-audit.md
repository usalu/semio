# Boundary audit (Typesafe Semio)

## Rust (`semio/rs/lib.rs`)
- `KitEvent::SemioKitCommand` / lifecycle: `command_kind`, `phase` as strings → replace with explicit enums; `result`/`error` as `serde_json::Value` → typed DTOs.
- RPC helpers: `get_field_rpc`, `set_field_rpc`, `add_child_rpc`, `change_kit_commands_for_field_patch` use `serde_json::Value` → per-field typed inputs or small serde-tagged command payloads at the boundary; internal JSON only behind parse/format.
- Undo: `before`/`after` snapshots as JSON → keep internal but expose typed `UndoStepSnapshot` where consumed.
- GraphQL `kit_graphql`: `variables: Option<serde_json::Value>`; batch rows as JSON scalars → explicit GraphQL types for read outputs and event payloads.
- `get_*_json` on kit graph: return structured DTOs in Rust and GraphQL, JSON string only in WASM if needed with typed round-trip.

## `semio/graphql/schema.graphql`
- Replace JSON/Scalar tunnels for `KitEvent`, `batch` results, read scope rows with explicit object shapes matching Rust DTOs.

## `semio/js/index.ts`
- Public `Record<string, unknown>`: `BackboneConfig`, `BackboneStatusDto`, `ConflictResolution`, `ReadKitCommandOutput` → specific interfaces / discriminated `ReadKitCommand`→output map.
- `KitEvent` union with `Record<string, unknown>` → `KitInvalidationEvent` + `KitCommandLifecycleEvent` + narrow invalidation tag set.
- `piecePatchToWireCommands` / field patches: use entity-specific patch DTOs and wire command types only.
- `KitStoreClient` methods returning `unknown` / `Record<string, unknown>` → named result types; keep raw GraphQL only inside `private` helpers.
- `materializedLiveJsonForReadScope` / `vcsState` → return typed DTOs.

## `semio/react/index.tsx`
- Remove `// @ts-nocheck` after store types land.
- `KitHostGraphOp`: replace `unknown` id/patch/body with DTOs from `@semio/js` / kit JSON shapes; use `string` for entity ids.
- `executeSemioKitCommand` / `createKitCommandEngineExplicitOrigin`: remove or mark `@internal` with minimal typed entry points; consumers use `applyKitHostGraphOp` + specific hooks.
- `HookTriad<readonly unknown[]>`, `any` in schema hooks: use DTO array types (TypeDto, DesignDto, etc. from semio).
