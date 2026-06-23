---
name: compose store rust sidecar
overview: The `compose/store` Rust binary (`compose-store`) provides NDJSON JSON-RPC 2.0 on stdio over `KitStore`. Python, .NET, and tools route kit I/O and command execution through this sidecar. This plan is **complete**; the sections below are retained as an architecture record.
todos:
 - id: new_store_bundle
   content: Add compose/store Cargo binary bundle (Cargo.toml, bin.rs, package.json, AGENTS.md, README.md) with NDJSON JSON-RPC 2.0 server wrapping KitStoreHandle. Register in root Cargo.toml workspace members.
   status: completed
 - id: store_method_catalog
   content: Implement KitStoreHandle-aligned RPC (lifecycle, kit.execute / executeChangeKitCommands / executeReadKitCommands, materializeAt, vcsState, field/child, design.*, vcs.*, query.*, static compose/kit utilities) and event notifications from KitStore::subscribe
   status: completed
 - id: store_integration_tests
   content: Add compose/store/tests/rpc.rs integration tests (spawn binary, create/snapshot, executeChangeKitCommands, undo/redo, events)
   status: completed
 - id: py_delete_native
   content: Remove native kit graph/diff I/O from compose/py in favor of store client + sidecar; delegate import/export and command edits
   status: completed
 - id: py_store_client
   content: compose/py/store.py StoreClient; edit_*/import_/export_ delegate to sidecar
   status: completed
 - id: py_tests
   content: store_test.py; rewrite or skip tests for removed entry points
   status: completed
 - id: net_store_migration
   content: "Remove in-process SQLite kit persistence; add Compose/Store (StoreClient, StoreKitIO, KitInPlaceDiff, ComposeDiff wire helpers). FileKit, FolderKit, zip paths use compose-store. Grasshopper Load/Save/Update Kit use StoreKitIO."
   status: completed
 - id: net_tests
   content: compose/net/Compose.Tests/StoreClientTests.cs and integration scenarios
   status: completed
 - id: docs
   content: compose/store/AGENTS.md, compose/net/AGENTS.md, compose/py/AGENTS.md, compose/gh/AGENTS.md (Grasshopper + sidecar)
   status: completed
isProject: false
---

## Status (complete)

- **compose/store**: `compose-store` binary, method catalog in [`AGENTS.md`](../../compose/store/AGENTS.md), `COMPOSE_STORE_BIN` / `StorePaths` resolution.
- **compose/py**: [`store.py`](../../compose/py/store.py) client; I/O and command paths through the sidecar. See [`AGENTS.md` Mechanisms](../../compose/py/AGENTS.md).
- **compose/net**: [`StoreKitIO.cs`](../../compose/net/Compose/Store/StoreKitIO.cs), `StoreClient`, `KitInPlaceDiff`; `Kit.ApplyDiff` remains for in-memory DTO merge; on-disk I/O is sidecar-based.
- **compose/gh**: Grasshopper persistence components use `StoreKitIO.LoadKitFromFolder` / `SaveKitToFolder`; see [`gh/AGENTS.md`](../../compose/gh/AGENTS.md). Project copies `target/release/compose-store.exe` when built.
- **Tests**: `compose/store/tests/rpc.rs`, Python `store_test.py`, C# `StoreClientTests.cs`.

## Out of scope (unchanged)

- Multi-kit in one `compose-store` process.
- LSP `Content-Length` framing (NDJSON only).
- wasm `compose/js` continues to use `KitStoreHandle` in-process.
