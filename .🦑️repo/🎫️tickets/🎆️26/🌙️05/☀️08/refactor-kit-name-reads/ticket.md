# Refactor Kit Name Reads And Split Rename Hook

## Status: closed

### Summary

- **`compose/js`**: Store primitives (`StoreField`, `StoreCommand`, `RequestCorrelator`, `OperationRouter`), `KitStore.kitName` / `renameKit` / cacheless `readKitName()`, correlation via `kitRenamed` + `operationFailed`. **GraphQL**: Kit-level `types` / `designs` / `authors` selections use Relay `edges { node { … } }`; `kitGraphqlJsonToReadonlyArray` unwraps connections; design shallow parse unwraps nested `pieces` / `connections` relay; `vcsState()` uses relay for checkpoints/alternatives and normalizes to arrays for `canUndo` / tests; `KIT_SESSION_QUERY_ENTRY` matches `Query.wip` (no invalid root `session { }`); SDL contract test updated for `type Session` + `type Subscription`.
- **`compose/react`**: `useKitName(): string`, `useRenameKit(): [rename, WriteStatus]`, exports aligned with js; test stub `as unknown as KitStoreClient`; `KitAlternativeSelectionProvider` receives `children` prop.
- **`compose/sketchpad`**: Kit name row uses `useKitName` + `useRenameKit` + inline controls (not triad).

### Files touched (this ticket / final pass)

- `compose/js/index.ts`
- `compose/react/index.tsx` (stub + provider fix from handoff)
- `compose/sketchpad/index.tsx` (from earlier in ticket)
- `.repo/🎫️/26/05/08/refactor-kit-name-reads/ticket.md`

### Validation run

- `npm run depcruise:layers` (repo root): pass
- `compose/js`: `npx tsc --noEmit`, `npm test` (30 tests): pass
- `compose/react`: `npx tsc --noEmit`, `npm test` (16 tests): pass
- `compose/sketchpad` `tsc`: still reports missing Node/rs-wasm types when compiling against `../js` (pre-existing config gap); not introduced here.
