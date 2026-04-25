# Semio layer parity — implementation notes (2026-04-25)

## Done

- **@semio/js**: `kitEventShouldRefetchViewCatalogKey` for `SemioKitViewStore` selective refetch; `kitEventAffectsDesignRead` + `SemioKitDesignReadStore` (per-design pieces/connections/metadata); `kitEventAffectsShallowList` + `SemioKitShallowListReadStore` (designs/types/authors); `kitEventAffectsCanUndoRedo`, `kitEventAffectsPieceLiveRead`, `kitEventAffectsTypeScopedRead`, `kitEventAffectsDesignQualitySumRead`, `kitEventAffectsKitColoredConnectorsRead`, `kitEventAffectsReplaceableCatalogRead` + `getSemioKitLiveReadStore` / `SemioKitLiveReadStore` (keyed async reads + per-key `shouldRefresh`). Embedded test covers `getSemioKitLiveReadStore` in the WASM client flow.
- **@semio/react**: `useSemioReadSnap` / `EMPTY_KIT_READ_SNAP`; `useKitPieces`, `useKitConnections`, `usePiecesMetadataMap`, `useKitDesignsShallow`, `useKitTypesShallow`, `useKitAuthorsShallow`, `useCanUndo`/`useCanRedo`, `usePieceFlatPlane`/`usePieceFlatCenter`/`usePieceParentConnection`, `useIncludedDesigns`, `useDesignClusterableGroups`, `useDesignQualitySum`, `useTypeBestRepresentation`, `useKitColoredConnectors`, `useReplacableTypes`, `useReplacableDesigns`, `useExplodeableDesignNodes` use `getSemioKitLiveReadStore` + `useSyncExternalStore` (no `useEffect` + broad `kitClient.subscribe(() => load())` for those).
- **Rust**: No code change this pass; existing actor + `KitEvent` wire already aligned; `cargo test --lib` passed.

## Validation run locally

- `cd semio/rs && cargo test --lib`
- `cd semio/js && vitest run` with `SEMIO_JS_RUN_EMBEDDED_TESTS=1`
- `cd semio/react && vitest` with `SEMIO_REACT_RUN_EMBEDDED_TESTS=1`
- `tsc --noEmit` in `semio/js` and `semio/react`
