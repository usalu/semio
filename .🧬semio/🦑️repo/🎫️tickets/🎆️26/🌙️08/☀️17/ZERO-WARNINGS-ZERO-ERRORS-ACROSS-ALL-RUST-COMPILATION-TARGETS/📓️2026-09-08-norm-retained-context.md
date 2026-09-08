# Norm Retained Reducer Context

Native604 reported E0308 when the shared Norm reducer was passed to `BoundedArtifactCommandWork::new`. The framework's `ArtifactCommandReducer` requires eight arguments, including optional `ArtifactOwnedToolJobContext` before the operation context. Norm's reducer only accepted seven.

Added the unused optional context parameter, specialized to `EditorApp<A>`, at the required position. The existing reducer still validates its tool route, constructs the operation-aware document view, dispatches the typed Norm command and reverses the publication mutation order. No reducer body or factory behavior changed. Source search found no direct callers to update.

Changed file: `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs`.

The compiler error provides the failing pre-change evidence. Parsing, type-checking and retained-command runtime verification are pending.

## Pass 610

Both the Norm reducer and Store Sync qualification changes passed rustfmt's Rust 2021 parser; neither file changed during validation. Ticket TypeScript also parses. The exact runtime list now includes three existing DIN 4108 retained-publication/config-only/catalog laws, bringing the total to 329 laws in 57 groups. The owning manifest has no component-app-assembly feature, so none is requested. Compiler and runtime confirmation remain pending.
