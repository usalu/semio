# Parity inventory (working notes)

- **semio/rs**: `KitStoreHandle` + GraphQL `submitKitCommand` (receipt) + `eventStream` (`SemioKitCommand` phases). `ChangeKitCommand` via actor `GraphWork::ChangeKitCommands`. `KitStoreNode` exposes `type_ids`, `design_ids`, `types_metadata`, `designs_metadata`.
- **semio/js**: `kitGraphqlSubmitCommandShell` for writes; `FallbackKitStoreClient` / `KitWorkerApi` use shell (worker must mirror all mutators). `kitGraphqlMapReadCommand` was partial; `LiveKitRoot` missing `readTypesMetadata` / `readDesignsMetadata` (bug).
- **semio/react**: `useTypesIds` / `useDesignsMetadata` refetch on every event; will use `SemioKitViewStore` for equality-gated updates.
- **sketchpad**: imports hooks from `@semio/react` only; boundary unchanged after React fix.

Temporary logs: use `[DEBUG] ` prefix if needed; remove after validation.
