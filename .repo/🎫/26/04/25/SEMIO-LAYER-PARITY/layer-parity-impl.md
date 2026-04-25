# Semio layer parity — implementation notes (2026-04-25)

## Done

- **@semio/js**: `kitEventShouldRefetchViewCatalogKey` for `SemioKitViewStore` selective refetch; `kitEventAffectsDesignRead` + `SemioKitDesignReadStore` (per-design pieces/connections/metadata); `kitEventAffectsShallowList` + `SemioKitShallowListReadStore` (designs/types/authors); exported `KitStoreReadSnap`.
- **@semio/react**: `useSemioReadSnap`, refactored `useKitPieces`, `useKitConnections`, `usePiecesMetadataMap`, `useKitDesignsShallow`, `useKitTypesShallow`, `useKitAuthorsShallow` to `useSyncExternalStore` + new stores (no broad `subscribe`+refetch loops).
- **Rust**: No code change this pass; existing actor + `KitEvent` wire already aligned; `cargo test --lib` passed.

## Validation run locally

- `cd semio/rs && cargo test --lib`
- `cd semio/js && vitest run` with `SEMIO_JS_RUN_EMBEDDED_TESTS=1`
- `cd semio/react && vitest` with `SEMIO_REACT_RUN_EMBEDDED_TESTS=1`
- `tsc --noEmit` in `semio/js` and `semio/react`
