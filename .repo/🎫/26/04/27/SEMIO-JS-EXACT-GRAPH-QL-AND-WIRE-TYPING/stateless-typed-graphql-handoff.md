# Handoff: Semio JS Stateless Typed GraphQL (2026-04-27)

## Done

- `semio/js/index.ts`: `kitGraphqlRunTyped`, `materializedLiveJsonForReadScope` uses it; `KitStoreClient` includes `kitReadScope`; `KitStoreReadSnap.data` is `unknown`; `isKitCommandLifecycleEvent(unknown)`; fixed corrupted `buildSchemaEntityChangeCommands` (`switch` + `case "Kit"`); `writeKitStoreClient*`, `kitStoreClientUpdatePiece/Connection` accept `unknown` where appropriate; embedded tests for `KIT_SCOPED_FULL_DTO_QUERY`, no `KitStore.snapshot()`.
- `semio/react/index.tsx`: `fetchFullKit` in embedded clients; `SemioKitDesignReadStore.getSnapshot` returns empty metadata object before first poll; duplicate `KitEvent` import removed.
- **Verify:** `npm run build` + `npm run test` in `semio/js` and `semio/react`; `npm run depcruise:layers` at repo root — all passed.

## Process

- `repo` MCP `ticket_close` was not available in this environment; close the ticket manually when possible.
