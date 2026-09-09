# Child Projection Fault Conversion

The shared document closure view is now connected to parent child restoration, eliminating the earlier unused declarations. Diagnostic WASI 1118 reached that new call and found its typed ChildRestoreProjectionError cannot convert to the plugin Fault through ?. Map the typed projection failure into the existing SDK fault boundary, retaining its diagnostic variant text. The exact parent dialect and child membership checks remain in place. Fresh strict compilation is still required.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs
