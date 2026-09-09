# Writer Declaration Integration

Moved the three standard/subset declaration assertions into the existing plugin surface tests. These now instantiate the actual closed WriterApps type, which supplies the required editor and viewer variants, instead of leaving a generic application parameter uninferable in the artifact crate. Removed the emptied old mounts and test files. Registered the three declaration laws and both existing editor/viewer integration laws.

Three Rust files and runner syntax parsed. Runtime and full compiler validation pending.

- updated: ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🦀️.rs
- removed: ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🧪️tests/🔬️unit/🦀️.rs
- updated: ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs
- removed: ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔬️unit/🦀️.rs
- updated: ✏️s/🔌️plugins/✒️writer/🧪️tests/🔬️surface/🦀️.rs
- updated: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/📜️script.ts
