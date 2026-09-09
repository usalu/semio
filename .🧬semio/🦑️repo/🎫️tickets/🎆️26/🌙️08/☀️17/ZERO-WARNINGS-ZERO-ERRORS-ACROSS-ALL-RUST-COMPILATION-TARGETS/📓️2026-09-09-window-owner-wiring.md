# Window Transient Owner Wiring

wasm1015 reached the plugin layer after the kernel bounds repairs. Initialize each registered window's concrete owner bundle and maintenance cursor, clone its Arc factories before borrowing a mutable partition, and propagate tracked snapshot capture failures through the registry's Result. Consolidate the viewer assertion's generic bound in its existing where clause. The removed transient factory reexports and stale zero-transient store layout assumption remain to be resolved against the current owner API.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs
