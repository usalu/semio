# Wave 1.A Summary — `.sxt` Extension Package Format

## Status
Done. New OS module implements pack/unpack/verify/content_hash for runtime-installable extension packages.

Repo MCP was unavailable (no `repo` namespace / `repo://goals` fetch failed). Continued on existing open ticket `26/08/07/RUNTIME-INSTALLABLE-EXTENSIONS` (goal `r2602/runningsketchpad`). Ticket left **open** for remaining Wave 1.* / later waves.

## Deliverable
- **Module:** `🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️component.rs`
- **Wire-in:** path-included as `os_extension` in kernel `📦️glue.rs`, re-exported as `extension` (not globbed — avoids collisions with pack `content_hash` and semio `verify`)
- **Dep:** `zip = { version = "2.4", default-features = false, features = ["deflate"] }` on `semio-framework-os-kernel`

## Package layout (`.sxt`)
1. Outer: `.semio` binary envelope `os.extension.pack v1` (`wrap_binary` / `unwrap_binary`)
2. Inner: deterministic deflate zip (epoch `DateTime`, sorted asset names)
   - `🛂️manifest.semio` — JSON `ExtensionPackageManifest` (camelCase)
   - `component.wasm` — raw wasip2 component bytes
   - optional `assets/**`

## API (`semio_framework_os_kernel::extension` / `os_extension`)
- `pack(manifest, component_wasm, assets) -> Result<Vec<u8>, ExtensionPackageError>`
- `unpack(bytes) -> Result<ExtensionPackage, ExtensionPackageError>`
- `verify(bytes) -> Result<ExtensionPackageManifest, ExtensionPackageError>`
- `content_hash(bytes) -> String` via `semio_framework_hash::hash_bytes`

## Verification
- `cargo check -p semio-framework-os-kernel --lib` — **ok** (lib only; unrelated bin/`cfg(test)` breakage elsewhere)
- Ticket probe `probe-sxt/`:
  - runtime: `[DEBUG] wave1a probe ok` (pack→verify→unpack→repack byte-identical; blake3 hash stable)
  - unit tests `os_extension::*`: **4/4 passed**
    - `pack_unpack_verify_round_trip`
    - `content_hash_is_stable_blake3`
    - `verify_rejects_wrong_envelope`
    - `pack_rejects_empty_component`
- Logs: `wave1a-probe-clt.log`, `wave1a-unit-tests-extension.log`, `wave1a-cargo-check-lib.log`

## Files touched
| Path | Action |
|------|--------|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️component.rs` | created |
| `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` | updated (`os_extension` + `extension` alias) |
| `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` | updated (`zip` dep) |
| ticket `probe-sxt/`, logs, this summary | created (temp) |

## Notes
- `ExtensionPackageManifest.capabilities` are topic **strings** and `contributions` is `serde_json::Value` (v1 package shape); WIT guest `ExtensionManifest` remains the typed runtime manifest in `🔌️plugin`.
- Full-crate `cargo test --lib` still fails on pre-existing spr/vcs demo test mismatches unrelated to this wave.
