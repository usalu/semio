# os-flow MeshData call-site conversion (2026-09-02)

## Goal
Take `semio-framework-os-flow` from 4 cargo-check errors to 1 (the known, unrelated
`🌿️vcs/🦀️.rs:2771` `E0502` borrow-checker error from concurrent peer work), following
through on `MeshData` (`🧰️framework/🔨️modules/🔺️mesh-engine/🦀️.rs`) having gained its own
first-party `impl pack::value::ToValue` / `impl pack::value::FromValue`, with
`Serialize`/`Deserialize` now `#[cfg_attr(test, …)]`-gated only, and `serde` moved to
mesh-engine's `[dev-dependencies]`.

## Call sites converted
Three downstream call sites in `semio-framework-os-flow` still used serde on `MeshData` and
failed with `E0277`. All three are reached through the `crate::os_pack` re-export chain
(`semio_framework_os_kernel::os_pack` → `pub use pack::json;`, where the compiled lib crate
`semio-framework-pack`'s `[lib] name` is `pack` — the same single crate instance mesh-engine
implements `ToValue`/`FromValue` against), since the `os-flow` crate itself has no direct
Cargo dependency on the `pack` crate.

1. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs:522`
   (`tessellate_geometry_json_for_wasm`) — was
   `serde_json::to_string(&mesh).unwrap_or_else(|_| …)`. `pack::json::to_json_string` is
   infallible (walks `ToValue::to_value` directly, no `Result`), so the fallback branch was
   dropped, not just swapped:
   `Ok(mesh) => crate::os_pack::json::to_json_string(&mesh),`

2. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs:4869`
   (`FlowProgramPhase::Domain` tessellation step) — was
   `serde_json::to_string(&mesh).map_err(domain_error)?`. Same infallible swap:
   `Ok(mesh) => crate::os_pack::json::to_json_string(&mesh),`
   (both match arms of the surrounding `match` now produce a bare `String`, matching the
   existing `Err(error) => json!({ "error": error }).to_string()` arm.)

3. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs:5622`
   (`dwg_encode_mesh`) — was
   `serde_json::from_str::<semio_framework::MeshData>(mesh_json)`. Swapped to:
   `crate::os_pack::json::from_json_str::<semio_framework::MeshData>(mesh_json)`
   still bound by `let Ok(mesh) = … else { … };`, since `from_json_str` also returns a
   `Result` (`Result<T, ValueError>`).

Nothing else in either file was touched: the surrounding `json!({...})` error-object literals
still use `serde_json::json!`/`Value` (imported via `use serde_json::{json, Value};`), which is
unrelated to `MeshData` and out of this slice's scope. `domain_error` is still used at ~30
other call sites in `component.rs`, so it was not touched or removed.

## Not touched (by design)
- `MeshData`'s `ToValue`/`FromValue` impl and its `#[cfg_attr(test, …)]` serde gating in
  mesh-engine — untouched, per the ticket's explicit instruction not to undo that work.
- `serde` in mesh-engine's `[dev-dependencies]` — untouched.
- `🌿️vcs/🦀️.rs:2771` `E0502` — left as the expected single remaining error; unrelated
  borrow-checker issue from concurrent peer work on `vcs`.

## Verification
Isolated `CARGO_TARGET_DIR` (peers hold the shared build lock), `RUSTC_WRAPPER=""`:

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/isolated-target2
export RUSTC_WRAPPER=""
cargo check -p semio-framework-os-flow --message-format short
cargo check -p semio-framework-mesh-engine --message-format short
```

Error counts via `grep -cE ': error(\[|:)'` (anchored `^error` undercounts, per prior
incident in this ticket):

| crate | before | after |
|---|---|---|
| `semio-framework-os-flow` | 4 | 1 (`vcs/🦀️.rs:2771` E0502 only, as expected) |
| `semio-framework-mesh-engine` | 0 | 0 (confirmed not regressed — `Finished` with warnings only) |

Both runs were foreground, no Monitor, no sub-agents.
