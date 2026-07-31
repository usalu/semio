# Compose React Hook And Scope Refactor — Implementation

## Rust (`compose/rs/lib.rs`)

- Routed `set_field_rpc`, `add_child_rpc`, and `remove_child_rpc` through `control_plane_batch_apply_with_undo` (removed duplicate `rpc_undo_lifting_apply_and_emit`).
- Fixed `control_plane_batch_apply_with_undo_matches_bare_apply_many_dto` to clone from one `KitFullDto` so kit ids match.

## JS (`compose/js/index.ts`)

- `theKit()` now uses `read(theKitReadScope, [{ readKitFullCommand: null }])` (single read path with snapshot fallback).

## React (`compose/react/index.tsx`)

- Added `ComposeKitScopedView` + `ComposeKitScopedViewContext` + `useComposeKitScopedView()`; `KitScope` provides scoped kit id, read scope, and write scope; `useKitDataScope` reads from it first; `useResolvedKitIdentifier` prefers explicit id, then scoped `kitId`, then tab shell, then runtime.
- `useTypesFull` / `useDesignsFull` / `useFilesFull` / `useTagsFull` use `KitStore.read` + `getComposeKitLiveReadStore` (RS materialized `readKitFullCommand`) instead of host store snapshot DTOs.
- Renamed `classicWritable` → `schemaScanWritable` in `useSchemaFieldState`.

## Sketchpad

- No import changes (already uses `@semio-tech/compose-react` only); `npm run build` verified.

## Tests run

- `cargo fmt` + `cargo test --lib` (compose/rs)
- `npm run test` (compose/js)
- `npm run build` + `npm run test` (compose/react)
- `npm run build` (compose/sketchpad)

## Files touched

- `compose/rs/lib.rs`
- `compose/js/index.ts`
- `compose/react/index.tsx`
- `.repo/🎫/26/04/27/COMPOSE-REACT-HOOK-AND-SCOPE-REFACTOR/ticket.json` (this ticket)
