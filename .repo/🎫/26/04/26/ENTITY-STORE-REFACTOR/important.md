# Summary

- **semio/js**: `kitStoreFromKitStoreClient`; `KitStore.designRowIds`, `kindRowIds`, `kindMetadataRows`, `designMetadataRows`; `PieceStore` flat/parent reads; `DesignStore` clustered/catalog/quality/pieces reads; `TypeStore.readBestRepresentation`; `WasmKitStoreClient` delegates to entity stores; embedded tests for row ids + store surface.
- **semio/react**: Catalog + shallow + design-scoped hooks route through `kitStoreFromKitStoreClient` + entity stores / `KitStore` reads with `useSemioReadSnap`; re-export `kitStoreFromKitStoreClient`.
- **Validation**: `semio/js` vitest (19), `semio/react` vitest (13), `npm run build` in `semio/react`.
