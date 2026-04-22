---
technology: semio
bundle:
 name: rs
 emoji: 🦀
 description: The rs bundle for semio.
 kind: library
---

# 🧾 Specification

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

### Layout

The library is a single crate root file, [`lib.rs`](lib.rs) (no `src/` tree). It uses inline
`pub mod … { … }` for each domain (including a large [`change_command`](lib.rs) section). Every parent owns its children through
`Arc<RwLock<T>>`; every child keeps a `Weak<RwLock<T>>` back-reference to its
parent. Derived data (content-addressable hashes, flatten caches) lives in
`OnceLock` fields invalidated by the owning entity's setters.

- Primitives: `id`, `hash`, `error`, `report`, `geom` modules in `lib.rs`.
- Value objects: `attribute`, `prop`, `benchmark`, `stat`, `tag`, `concept`, `author`.
- Kit-scoped entities: `file`, `folder`, `quality`.
- Type-scoped children: `port`, `connector`, `representation`.
- Design-scoped children: `piece`, `connection`, `side`, `layer`, `group`.
- Aggregates: `typ` (Type), `design`, `kit`.
- Change flow: `change_command` produces sparse diffs; [`KitStore::apply_kit_diff`](lib.rs) and [`DesignStore::apply_diff`](lib.rs) are the central write paths for kit and design trees (**remove → update → add** per child collection, then rewire + invalidate where applicable). [`ChangeKitCommand::apply`](lib.rs) returns `(KitDiff, inverse)` from `KitDiff::between` on the live kit before/after each step; [`ChangeKitCommand::apply_many`](lib.rs) merges those via [`KitDiff::merge`](lib.rs) and stacks inverses. [`FileStore::apply_diff`](lib.rs) / [`FolderStore::apply_diff`](lib.rs) apply sparse file/folder patches. [`ChangeKitCommand::compact`](lib.rs) coalesces consecutive kit metadata writes and cancels adjacent add/remove pairs for types and designs. VCS: [`KitChange`](lib.rs) stores `Vec<ChangeKitCommand>` forward + inverse; diffs are ephemeral per-apply products; snapshot reconciliation uses [`ChangeKitCommand::ReplaceKitFromFullDto`](lib.rs) when no command history exists. See also `read_command`, `kit_store_command`, `kit_session`, and related `kit_*` modules.
- I/O: `io::json` and native-only SQLite/ZIP under `io` in `lib.rs`.
- WASM surface: `pub mod wasm` in `lib.rs` (identical JS names, delegates to OO API).

## 📛 Entities

### Ownership graph

```
Kit  ─┬─> Type  ─┬─> Port
      │          ├─> Connector ──> Weak<Port>
      │          └─> Representation ──> Weak<File>
      ├─> Design ─┬─> Piece ──> Weak<Type>
      │           ├─> Connection ──> Side { Weak<Piece>, Weak<Port> }
      │           ├─> Layer
      │           └─> Group ──> [Weak<Piece>]
      ├─> File
      ├─> Folder
      └─> Quality (also referenced from Port/Type/Design)
```

Every solid arrow is an `Arc<RwLock<T>>`; every back-reference (drawn from a
child to its parent) is a `Weak<RwLock<T>>`. IDs appear only on `*Dto`
structs and as keys in the kit-level resolver during `Kit::from_dto`.
