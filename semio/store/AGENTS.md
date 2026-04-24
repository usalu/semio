---
technology: semio
bundle:
 name: store
 emoji: 🏪
 description: The store bundle for semio (stdio JSON-RPC sidecar to KitStore).
 kind: application
---

# 🧾 Specification

## 🕸️ Systems

- **Kit sidecar**: one long-running `semio-store` process per kit. The process holds a native [`kit_store::KitStore`](https://github.com/usalu/semio/blob/main/semio/rs/lib.rs) (WIP + coordinator + optional backbone + conflict registry), analogous in role to a wasm `KitStoreHandle` over a single graph.

## 🛠️ Mechanisms

- **Transport**: NDJSON — each line is a single JSON value. **Requests** are JSON-RPC 2.0 objects with `jsonrpc: "2.0"`, `method`, optional `id`, and optional `params`. **Server → client** events are JSON-RPC 2.0 **notifications** with `method: "event"` and `params` = serialized [`semio` `KitEvent`](https://github.com/usalu/semio/blob/main/semio/rs/lib.rs) (no `id`).
- **Binary**: `semio-store` (crate `semio-store`), `[[bin]]` in [`Cargo.toml`](Cargo.toml). Implementation: [`bin.rs`](bin.rs), [`jsonrpc.rs`](jsonrpc.rs).
- **Lifecycle**: the first `kit.create` or `io.importFromFolder` / `io.importFromFile` / `io.importFromZip` / `io.importFromRemote` installs the sole control-plane `KitStore` (wrapping the live `KitGraphRef`). A second install returns a JSON-RPC error. **Shutdown**: `server.shutdown` or close stdin.
- **Method catalog (high level)**:
  - **Static (no store)**: `semio.generateId`, `semio.round`, `semio.normalizeName`, `kit.fromJson`, `kit.toJson`, `kit.validate`, `kit.equals`, `design.flatten` (takes a `kit` DTO + `designId`).
  - **Install**: `kit.create` `{ dto }`, `io.importFromFile` `{ path }`, `io.importFromFolder` `{ path }`, `io.importFromZip` `{ path }`, `io.importFromRemote` `{ hubUrl, sessionId }`.
  - **Export**: `io.exportToFile`, `io.exportToFolder`, `io.exportToZip` `{ path }` (store must exist).
- **Store**: `kit.snapshot`, `kit.theKitDto`, `kit.execute`, `kit.executeChangeKitCommands` `{ cmds }` (applies with kit undo snapshots), `kit.executeReadKitCommands` `{ cmds }`, `kit.materializeAt`, `kit.vcsState`, `kit.getField`, planners `kit.changeKitCommandsForFieldPatch` `{ kind, id, field, value }` (includes `Family` and family-owned `Port` scoping only), `kit.changeKitCommandsForAddChild` / `kit.changeKitCommandsForRemoveChild` (`Kit`→`Family`, `Design`→`Piece`; return command arrays; apply with `kit.executeChangeKitCommands`). Matches wasm `KitStoreHandle`: `changeKitCommandsForFieldPatch`, `changeKitCommandsForAddChild`, `changeKitCommandsForRemoveChild`, `executeChangeKitCommands` (legacy `setField` / `addChild` / `removeChild` wasm entrypoints removed).
  - **Backbone / sync (native control plane)**: `backbone.attach` `{ config }` (`dev` / `local` / `remote` variants, camelCase), `backbone.detach`, `backbone.status`, `backbone.setActiveCheckpoint` `{ id }` (local materialization), `conflicts.list`, `conflicts.resolve` `{ id, strategy }`, `coordinator.syncNow`. Results use the same tagged shapes as [`KitStoreCommandResult`](https://github.com/usalu/semio/blob/main/semio/rs/lib.rs) (e.g. `attachBackbone`, `backboneStatus`, `listConflicts`).
  - **Design**: `design.clusterPieces`, `design.dragPieces`, `design.movePieces`, `design.fixPieces`, `design.flattenDesign`, `design.expandDesign`, `design.deleteConnection`, `design.changePieceType`, `design.pasteDesignSelection`, `design.createHangingPieces`, `design.createConnectedPiece`, `design.createFixedPiece` (see wasm `KitStoreHandle` for parameter names, camelCase in JSON).
  - **VCS (snapshot undo, graph edits)**: `vcs.undo`, `vcs.redo`, `vcs.canUndo`, `vcs.canRedo`. Session/draft/transaction flow is `kit.execute` with [`KitStoreCommand`](../rs/lib.rs) only (no `beginTx` / `commitTx` on the store).
  - **Query**: `query.pieces` / `query.piecesMetadata` / `query.connections` `{ designId }`, `query.designs` / `query.types` / `query.authors` / `query.kitMetadata` (no extra params where not needed).
  - **Events (no-ops)**: `events.subscribe`, `events.unsubscribe` — the server always forwards `event` once a kit exists.
- **Error codes**: `-32700` parse, `-32600` invalid request, `-32601` method not found, `-32602` invalid params, `-32000` application (`SemioError` / lock / I/O as string).

## 📛 Entities

- Wire DTOs and command enums are the same [`serde` types as `semio`](../rs/lib.rs) (`KitFullDto`, `KitStoreCommand`, `ChangeKitCommand`, `ReadKitCommand` in [`read` / `read_module.rs`](../rs/read_module.rs), …). **`kit.executeReadKitCommands`** takes `{ cmds: ReadKitCommand[] }` (externally tagged, camelCase keys on each command variant) and returns `ReadKitCommandOutput[]`. No second schema.
