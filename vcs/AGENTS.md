---
technology: vcs
emoji: 🗄️
---

# VCS

Generic document version control: typed operations, replay materialization, checkpoints, alternatives, and backbone persistence.

## Entities

- **Operation** — stored semantic mutation; defines `diff(pre)` and `backwards(pre)`
- **Edit** — forwards/backwards operation lists, sequence number, timestamps
- **Change** — groups edits saved into one checkpoint
- **Checkpoint** — parent chain, authors, message, cumulative change ids
- **Alternative** — named track of checkpoints (branch)
- **Author** — contributor identity for checkpoint attribution

## Bundles

| Bundle | Role |
|--------|------|
| `vcs/rs` | Rust/WASM engine (`vcs` crate) |
| `vcs/core` | TypeScript mirror + `DocumentVcsStore` |
| `vcs/react` | `HistoryTable` UI |
| `vcs/play` | History playground demo |

## Mechanisms

- `DocumentVcsStore::dispatch` — apply, undo, redo, commit checkpoint, create/switch alternative
- `materialize_document_projection` — replay applied edit ids
- `Backbone` trait — `dev://`, `local://`/`sqlite://`, `remote://` URI resolution
