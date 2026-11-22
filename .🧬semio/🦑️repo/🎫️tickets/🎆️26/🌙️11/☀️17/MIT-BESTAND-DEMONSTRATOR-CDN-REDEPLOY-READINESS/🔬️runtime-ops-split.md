# runtime-ops source boundary split

Gate: `semio-plugin-runtime-ops` was 268,268 of 262,144 bytes (`maxSourceBytes`).

Cause: recent growth in `🧾️typed-operations`, `📄️document`, `🖥️surface-render`, `🎞️raw-media` (mtimes 11-21).

Split: the cohesive raw media ingress module (`🎞️raw-media/`, ~35 KB: natural source formats, original media control, folder archive load, picker effects) became its own crate `semio-plugin-runtime-ops-raw-media`, the same way `-wire`, `-document`, `-surface` were split.

- New `🎞️raw-media/📦️packages/🦀️rust/Cargo.toml` (lib path `../../🦀️.rs`, sources unmoved, no `#[path]`).
- `runtime/🧰️ops/🦀️.rs`: `#[path] pub mod ops_raw_media` replaced by `pub use semio_plugin_runtime_ops_raw_media as ops_raw_media;`, so `poll-kernel-turn` and `🧪️tests/📤️picker` need no change.
- `runtime/🧰️ops/📦️packages/🦀️rust/Cargo.toml`: new path dependency.
- Cargo.lock: added the package and the ops dependency line in `🌎️hub/Cargo.lock` and all 122 other lockfiles listing `-ops-surface` (so `--locked` stays valid).
- The module had no `crate::` coupling (only `super::` internals), so the split is orthogonal to concurrent edits in ingress, typed-operations and document.
