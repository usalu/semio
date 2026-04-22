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
`pub mod … { … }` for each domain. Every parent owns its children through
`Arc<RwLock<T>>`; every child keeps a `Weak<RwLock<T>>` back-reference to its
parent. Derived data (content-addressable hashes, flatten caches) lives in
`OnceLock` fields invalidated by the owning entity's setters.

- Primitives: `id`, `hash`, `error`, `report`, `geom` modules in `lib.rs`.
- Value objects: `attribute`, `prop`, `benchmark`, `stat`, `tag`, `concept`, `author`.
- Kit-scoped entities: `file`, `folder`, `quality`.
- Type-scoped children: `port`, `connector`, `representation`.
- Design-scoped children: `piece`, `connection`, `side`, `layer`, `group`.
- Aggregates: `typ` (Type), `design`, `kit`.
- Change flow: `diff` (`DesignDiff`/`DesignChange`); VCS: `read_command`, `change_command`, `kit_store_command`, `kit_session`, and related `kit_*` modules.
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
