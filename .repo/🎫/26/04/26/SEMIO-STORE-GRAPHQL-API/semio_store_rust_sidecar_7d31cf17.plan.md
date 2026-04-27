---
name: semio store rust sidecar
overview: The `semio/store` Rust binary (`semio-store`) provides NDJSON JSON-RPC 2.0 on stdio over `KitStore`. Python, .NET, and tools route kit I/O and command execution through this sidecar. This plan is **complete**; the sections below are retained as an architecture record.
todos:
 - id: new_store_bundle
   content: Add semio/store Cargo binary bundle (Cargo.toml, bin.rs, package.json, AGENTS.md, README.md) with NDJSON JSON-RPC 2.0 server wrapping KitStoreHandle. Register in root Cargo.toml workspace members.
   status: completed
 - id: store_method_catalog
   content: Implement KitStoreHandle-aligned RPC (lifecycle, kit.execute / executeChangeKitCommands / executeReadKitCommands, materializeAt, vcsState, field/child, design.*, vcs.*, query.*, static semio/kit utilities) and event notifications from KitStore::subscribe
   status: completed
 - id: store_integration_tests
   content: Add semio/store/tests/rpc.rs integration tests (spawn binary, create/snapshot, executeChangeKitCommands, undo/redo, events)
   status: completed
 - id: py_delete_native
   content: Remove native kit graph/diff I/O from semio/py in favor of store client + sidecar; delegate import/export and command edits
   status: completed
 - id: py_store_client
   content: semio/py/store.py StoreClient; edit_*/import_/export_ delegate to sidecar
   status: completed
 - id: py_tests
   content: store_test.py; rewrite or skip tests for removed entry points
   status: completed
 - id: net_store_migration
   content: "Remove in-process SQLite kit persistence; add Semio/Store (StoreClient, StoreKitIO, KitInPlaceDiff, SemioDiff wire helpers). FileKit, FolderKit, zip paths use semio-store. Grasshopper Load/Save/Update Kit use StoreKitIO."
   status: completed
 - id: net_tests
   content: semio/net/Semio.Tests/StoreClientTests.cs and integration scenarios
   status: completed
 - id: docs
   content: semio/store/AGENTS.md, semio/net/AGENTS.md, semio/py/AGENTS.md, semio/gh/AGENTS.md (Grasshopper + sidecar)
   status: completed
isProject: false
---

## Status (complete)

- **semio/store**: `semio-store` binary, method catalog in [`AGENTS.md`](../../semio/store/AGENTS.md), `SEMIO_STORE_BIN` / `StorePaths` resolution.
- **semio/py**: [`store.py`](../../semio/py/store.py) client; I/O and command paths through the sidecar. See [`AGENTS.md` Mechanisms](../../semio/py/AGENTS.md).
- **semio/net**: [`StoreKitIO.cs`](../../semio/net/Semio/Store/StoreKitIO.cs), `StoreClient`, `KitInPlaceDiff`; `Kit.ApplyDiff` remains for in-memory DTO merge; on-disk I/O is sidecar-based.
- **semio/gh**: Grasshopper persistence components use `StoreKitIO.LoadKitFromFolder` / `SaveKitToFolder`; see [`gh/AGENTS.md`](../../semio/gh/AGENTS.md). Project copies `target/release/semio-store.exe` when built.
- **Tests**: `semio/store/tests/rpc.rs`, Python `store_test.py`, C# `StoreClientTests.cs`.

## Out of scope (unchanged)

- Multi-kit in one `semio-store` process.
- LSP `Content-Length` framing (NDJSON only).
- wasm `semio/js` continues to use `KitStoreHandle` in-process.
