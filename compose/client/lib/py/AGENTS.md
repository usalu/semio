---
technology: compose
bundle:
 name: py
 emoji: 🐍
 description: The py bundle for compose.
 kind: library
---

# 🧾 Specification

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

- **Rust kit store (sidecar)**: [`compose/store`](../../store) builds `compose-store` (NDJSON JSON-RPC 2.0 on stdio). The Python client is [`store.py`](store.py) (`StoreClient`, `load_kit_via_io` for one-shot I/O + snapshot). `COMPOSE_STORE_BIN` overrides the binary path; the client looks for `../../target/release/compose-store[.exe]` from this bundle.
- **Import/export**: `import_file_kit`, `import_folder_kit`, `import_kit` (zip), and matching `export_*` delegate to the sidecar’s `io.*` and `kit.create` / `kit.snapshot`. Command-based edits go through `kit.executeChangeKitCommands` (see `edit_*_kit` and `StoreClient` in [`store.py`](store.py)). In-memory `commit_kit_graph_change` still uses validated dict diffs for interactive graph mutation and backbone notifications, distinct from the sidecar command JSON shape.
- **Tests**: `store_test.py` covers the sidecar client; some workflow tests remain skipped where they still targeted removed dict-`edit_*_kit` entry points (see `skip` reasons in `main.py`).

## 📛 Entities
