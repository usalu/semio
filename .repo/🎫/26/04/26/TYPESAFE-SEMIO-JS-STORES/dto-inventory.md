# DTO inventory (typesafe semio/js)

Grouped loose surfaces addressed in this ticket:

- **Wire tree**: `SemioJson` → `SemioKitWireTreeDto` / `SemioKitWireStructDto` (no `Json` / `Record` alias names on boundary).
- **GraphQL**: `kitGraphqlRun` / `kitGraphqlData` / `gqlDataKitQueryRoot` → `KitGraphqlExecuteBodyDto`, `KitGraphqlEnvelopeWireDto`, `KitGraphqlDataRootDto`, `KitGraphWireQueryBranchDto`.
- **Reads**: `Read*CommandOutput`, `materializedLiveJsonForReadScope`, `getPieces*`, `getKitMetadata`, `readDesignFlattenMap`, `vcsState`, piece/type/design live reads.
- **Writes / batch**: session/live/backbone shells → explicit `SemioShellVariablesPlainDto`, batch row DTOs, `changeKitWithInverse` result DTO.
- **Patches**: `piecePatchToWireCommands`, `connectionPatchToWireCommands`, `buildSchemaEntityChangeCommands` field values.
- **Client**: `KitStoreClient`, `WasmKitStoreClient`, `FallbackKitClient`, `SemioKitBridge`, `KitStoreReadSnap`, `LiveKitRoot`, stores, event filters.
- **Zod**: `z.any()` on diff `added` → entity `*Schema` arrays.
