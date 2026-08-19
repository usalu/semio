# Pack Module Relocation — Verification

## Finding

The `🎒️pack` container submodules (`async_`, `format`, `http`, `io`) were relocated from
`🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/` to `🧰️framework/🔨️modules/🎒️pack/` as part of
commit `3966c824fa` (🎆️26🌙️06☀️04🚩️538, 2026-08-19 00:00).

`📦️glue.rs` in `semio-framework-os-kernel` was updated in that commit:

- Removed stale `#[path]` mounts for `async_`, `format`, `http`, `io`
- Replaced with `pub use pack::{async_, codec, format, http, source, io}` from `semio-framework-pack`
- Removed `extern crate self as pack` (now uses the standalone `pack` lib crate)
- Added `every_path_mount_in_this_glue_resolves_to_an_existing_file` guard test

Os-side mounts intentionally retained for schema-driven code only:

- `🦀️component.rs` (facade re-exporting `pack::*` + os value helpers)
- `🔢️value/🦀️component.rs`
- `🧪️testkit/🦀️component.rs`
- `⌨️cli/🦀️component.rs` (native-only)

## Path mount audit

Manual scan of all `#[path = "..."]` lines in os-kernel `📦️glue.rs`: **0 missing targets**.

No remaining `#[path]` references to deleted os-side `⏳️async`, `🌐️http`, `📐️format`, or `🔌️io` anywhere under `🧰️framework/`.

## Build verification

| Command | Result |
|---------|--------|
| `cargo build -p semio-framework-os-kernel` | ✅ Finished (warnings only) |
| `cargo build -p semio-framework-os-kernel --target wasm32-wasip2 --lib` | ✅ Finished in ~17m |
| `cargo build -p semio-s-plugin-energy --target wasm32-wasip2` | ❌ Blocked by unrelated `📇️directory` API drift (`mint_session` missing `OperationContext` arg) — **no pack path errors** |

## Conclusion

Pack relocation glue fix is **complete and green**. Plugin wasm builds remain blocked by a separate in-flight `directory` client signature change, not by pack mounts.
