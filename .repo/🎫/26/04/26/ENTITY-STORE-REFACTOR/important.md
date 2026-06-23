# Summary

- **compose/js**: `kitStoreFromKitStoreClient`; `KitStore.designRowIds`, `kindRowIds`, `kindMetadataRows`, `designMetadataRows`; `PieceStore` flat/parent reads; `DesignStore` clustered/catalog/quality/pieces reads; `TypeStore.readBestRepresentation`; `WasmKitStoreClient` delegates to entity stores; embedded tests for row ids + store surface.
- **compose/react**: Catalog + shallow + design-scoped hooks route through `kitStoreFromKitStoreClient` + entity stores / `KitStore` reads with `useComposeReadSnap`; re-export `kitStoreFromKitStoreClient`.
- **Validation**: `compose/js` vitest (19), `compose/react` vitest (13), `npm run build` in `compose/react`.
