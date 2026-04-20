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

- Crate root: [`src/lib.rs`](src/lib.rs) (domain + tests + WASM).
- [`KitGraphSession`](src/lib.rs) owns `Arc<RwLock<Kit>>` (`kit_handle()` for shared access).
- Kit lookups: `design`, `design_mut`, `semio_type`, `semio_type_mut`, `file`, `folder`, `author`, `tag`, `concept`, `quality`, `port` (replacing `*_by_guid`).
- Future: split I/O into [`src/io/`](src/io/README.md); extra tests under [`src/tests/`](src/tests/extra_smoke.rs).

## 📛 Entities

### Kit

```rs

pub struct Connector {
        pub name: String,
}

pub struct Type {
        pub name: String,
}

pub struct Piece {
        pub name: String,
}

pub struct Connection {

}

pub struct Design {

}

```
