# Compose layer parity — implementation notes (2026-04-25)

## Done

- **@semio-tech/compose-js**: `kitEventShouldRefetchViewCatalogKey` for `ComposeKitViewStore` selective refetch; `kitEventAffectsDesignRead` + `ComposeKitDesignReadStore` (per-design pieces/connections/metadata); `kitEventAffectsShallowList` + `ComposeKitShallowListReadStore` (designs/types/authors); `kitEventAffectsCanUndoRedo`, `kitEventAffectsPieceLiveRead`, `kitEventAffectsTypeScopedRead`, `kitEventAffectsDesignQualitySumRead`, `kitEventAffectsKitColoredConnectorsRead`, `kitEventAffectsReplaceableCatalogRead` + `getComposeKitLiveReadStore` / `ComposeKitLiveReadStore` (keyed async reads + per-key `shouldRefresh`). Embedded test covers `getComposeKitLiveReadStore` in the WASM client flow.
- **@semio-tech/compose-react**: `useComposeReadSnap` / `EMPTY_KIT_READ_SNAP`; `useKitPieces`, `useKitConnections`, `usePiecesMetadataMap`, `useKitDesignsShallow`, `useKitTypesShallow`, `useKitAuthorsShallow`, `useCanUndo`/`useCanRedo`, `usePieceFlatPlane`/`usePieceFlatCenter`/`usePieceParentConnection`, `useIncludedDesigns`, `useDesignClusterableGroups`, `useDesignQualitySum`, `useTypeBestRepresentation`, `useKitColoredConnectors`, `useReplacableTypes`, `useReplacableDesigns`, `useExplodeableDesignNodes` use `getComposeKitLiveReadStore` + `useSyncExternalStore` (no `useEffect` + broad `kitClient.subscribe(() => load())` for those).
- **Rust**: No code change this pass; existing actor + `KitEvent` wire already aligned; `cargo test --lib` passed.

## Validation run locally

- `cd compose/rs && cargo test --lib`
- `cd compose/js && vitest run` with `COMPOSE_JS_RUN_EMBEDDED_TESTS=1`
- `cd compose/react && vitest` with `COMPOSE_REACT_RUN_EMBEDDED_TESTS=1`
- `tsc --noEmit` in `compose/js` and `compose/react`
