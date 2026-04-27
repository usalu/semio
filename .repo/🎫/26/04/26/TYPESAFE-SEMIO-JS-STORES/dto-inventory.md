# DTO inventory (typesafe semio/js)

Grouped loose surfaces addressed in this ticket:

- **Wire tree**: `SemioJson` → `SemioKitWireTreeDto` / `SemioKitWireStructDto` (no `Json` / `Record` alias names on boundary).
- **GraphQL**: `kitGraphqlRun` / `kitGraphqlData` / `gqlDataKitQueryRoot` → `KitGraphqlExecuteBodyDto`, `KitGraphqlEnvelopeWireDto`, `KitGraphqlDataRootDto`, `KitGraphWireQueryBranchDto`.
- **Reads**: `Read*CommandOutput`, `materializedLiveJsonForReadScope`, `getPieces*`, `getKitMetadata`, `readDesignFlattenMap`, `vcsState`, piece/type/design live reads.
- **Writes / batch**: session/live/backbone shells → explicit `SemioShellVariablesPlainDto`, batch row DTOs, `changeKitWithInverse` result DTO.
- **Patches**: `piecePatchToWireCommands`, `connectionPatchToWireCommands`, `buildSchemaEntityChangeCommands` field values.
- **Client**: `KitStoreClient`, `WasmKitStoreClient`, `FallbackKitClient`, `SemioKitBridge`, `KitStoreReadSnap`, `LiveKitRoot`, stores, event filters.
- **Zod**: `z.any()` on diff `added` → entity `*Schema` arrays.

## 2026-04-27 follow-up (this pass)

- **Batch semantics**: `KitStoreBatchResult.changeKind` is GraphQL enum `KitChangeSemanticKind`; `KitChangeKind::Other` uses `changeKindOther`. JS maps via `kitChangeSemanticKindToWire` to `KitChangeKindWire` (including `{ other }`).
- **Authors shallow**: `authorsShallow` is `[AuthorShallowRow!]!` (removed `AuthorShallowList` scalar); JS parsers still accept GraphQL arrays.
- **React graph ops**: `KitHostGraphOp` uses `TypeId` / `DesignId` / `PieceId` / `ConnectionId` and Plain/Diff DTOs; `kitHostUndo` / `kitHostRedo` replace sketchpad undo/redo `executeSemioKitCommand` calls.
- **Still scalar (next passes)**: `ChangeKitCommand`, `KitEvent`, `KitFullSnapshot`, `DesignShallowList`, `TypeShallowList`, `ConnectionFullList`, `PieceFullList` — large unions; prefer incremental object typing like authors.
