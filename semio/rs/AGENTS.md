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

One concept per file under [`src/`](src/). Every parent owns its children
through `Arc<RwLock<T>>`; every child keeps a `Weak<RwLock<T>>` back-reference
to its parent. Derived data (content-addressable hashes, flatten caches) lives
in `OnceLock` fields invalidated by the owning entity's setters.

- Primitives: [`id`](src/id.rs), [`hash`](src/hash.rs), [`error`](src/error.rs), [`report`](src/report.rs), [`geom`](src/geom.rs).
- Value objects: [`attribute`](src/attribute.rs), [`prop`](src/prop.rs), [`benchmark`](src/benchmark.rs), [`stat`](src/stat.rs), [`tag`](src/tag.rs), [`concept`](src/concept.rs), [`author`](src/author.rs).
- Kit-scoped entities: [`file`](src/file.rs), [`folder`](src/folder.rs), [`quality`](src/quality.rs).
- Type-scoped children: [`port`](src/port.rs), [`connector`](src/connector.rs), [`representation`](src/representation.rs).
- Design-scoped children: [`piece`](src/piece.rs), [`connection`](src/connection.rs), [`side`](src/side.rs), [`layer`](src/layer.rs), [`group`](src/group.rs).
- Aggregates: [`typ`](src/typ.rs) (Type), [`design`](src/design.rs), [`kit`](src/kit.rs).
- Change flow: [`diff`](src/diff.rs) (`DesignDiff`/`DesignChange`), [`session`](src/session.rs) (`KitGraphSession` owns `Arc<RwLock<Kit>>`).
- I/O backends: [`io::json`](src/io/json.rs); SQLite/ZIP stubs under [`io`](src/io/mod.rs).
- WASM surface: [`wasm`](src/wasm.rs) (identical JS names, delegates to OO API).

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
