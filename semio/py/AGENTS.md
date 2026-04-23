---
technology: semio
bundle:
 name: py
 emoji: 🐍
 description: The py bundle for semio.
 kind: library
---

# 🧾 Specification

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

- **Rust kit store (sidecar)**: [`semio/store`](../../store) builds `semio-store` (NDJSON JSON-RPC 2.0 on stdio). The Python client is [`store.py`](store.py) (`StoreClient`, `load_kit_via_io` for one-shot I/O + snapshot). `SEMIO_STORE_BIN` overrides the binary path; the client looks for `../../target/release/semio-store[.exe]` from this bundle.
- **Import/export**: `import_file_kit`, `import_folder_kit`, `import_kit` (zip), and matching `export_*` delegate to the sidecar’s `io.*` and `kit.create` / `kit.snapshot` instead of ad-hoc Python SQLite. The in-memory Pydantic `Kit` type still uses dict-shaped graph diffs for `commit_kit_graph_change` / validation until a follow-up migrates that path to `ChangeKitCommand` JSON.
- **Tests**: workflow tests that relied on `edit_*_kit(..., diff: dict)` are skipped until ported.

## 📛 Entities
