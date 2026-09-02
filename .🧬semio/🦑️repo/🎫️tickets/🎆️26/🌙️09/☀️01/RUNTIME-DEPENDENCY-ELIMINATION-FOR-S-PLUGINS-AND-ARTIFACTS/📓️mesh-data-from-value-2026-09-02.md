# MeshData `FromValue` — 2026-09-02

## Scope

File owned exclusively for this slice: `🧰️framework/🔨️modules/🔺️mesh-engine/🦀️.rs` (plus its
package `Cargo.toml`). No files under `✏️s/🔌️plugins/` were edited — this note lists the call sites
the new impl unblocks so that follow-up is mechanical.

## What changed

1. **`impl pack::value::FromValue for MeshData`** — hand-written (not `#[derive(FromValue)]`), added
   directly after the pre-existing `impl pack::value::ToValue for MeshData`. Decision rationale:
   `semio-framework-value-derive`'s expansion hardcodes `::semio_framework_os_kernel::…` paths
   (confirmed by reading `🌱️value/✨️derive/🦀️.rs`), so any crate using
   `#[derive(ToValue, FromValue)]` must depend on `semio-framework-os-kernel` directly — a huge,
   platform-heavy crate (tokio, wasm-bindgen, web-sys, zip…) that sits ABOVE mesh-engine in the
   dependency graph (mesh-engine's own doc comment: "consumed only from artifact facet code ... or
   engine-to-engine callers", i.e. it's a leaf). Taking that dependency here would invert the
   layering, so the derive "cannot express it" per the brief's own escape hatch — hand-writing
   against `pack::value` (mesh-engine's existing dependency, re-exported from `protocol::value`) was
   the correct call.
2. Reads the exact camelCase object shape `ToValue`/`From<MeshData> for pack::json::Value` already
   emit: `positions`/`normals`/`colors`/`indices` default to empty `Vec` when the key is absent
   (mirrors `#[serde(default)]`); `uvs`/`faceIds`/`vertexIds`/`edgePositions`/`edgeIds`/`edgeUvs`/
   `edgeIsSeam`/`paintTextureBase64` default to empty/`None` when absent (mirrors
   `skip_serializing_if`). Delegates every leaf to the already-existing blanket `Vec<T>: FromValue`
   / `f32`/`u32`/`u8`/`String: FromValue` impls in `🌱️value/🔁️codec/🦀️.rs` — no new leaf codec
   needed. Indices/faceIds/vertexIds/edgeIds/edgeIsSeam decode through `u32`/`u8`'s `FromValue`
   (accepts `UInt`/`Int`/`Float` `Number` arms but the encoder only ever emits `UInt` for them);
   positions/normals/colors/uvs/edgePositions/edgeUvs decode through `f32`'s. The two families never
   cross, so a mesh index cannot silently decode as a float (the `3600.0` regression the ticket
   warned about).
3. `MeshData`'s own `Serialize`/`Deserialize` derive is now `#[cfg_attr(test, derive(Serialize,
   Deserialize))]` (was unconditional), with every `#[serde(...)]` field attribute moved to
   `#[cfg_attr(test, serde(...))]` alongside it — `serde` is no longer a production dependency of
   this crate now that `MeshData` has its own first-party codec. Moved `serde` from `[dependencies]`
   to `[dev-dependencies]` in
   `🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/Cargo.toml` (`serde_json` was already
   dev-only). `serde`/`serde_json` survive purely as `#[cfg(test)]` differential oracles.
4. New test module `mesh_data_from_value_round_trip` (6 tests): default/dense/fully-populated
   round-trip (`FromValue::from_value(ToValue::to_value(&mesh)) == mesh`), an explicit
   `Number::UInt`-vs-`Number::Float` assertion per field family (the integer-fidelity regression
   guard), a decode-error-path test (`ValueError` reports `"indices.0.expected a number, found
   Bool(true)"`), and a serde_json differential oracle test (our `FromValue` decode vs. serde_json's
   `Deserialize` decode of the identical JSON, both compared to the original mesh).

## Verification

Isolated target dir, `RUSTC_WRAPPER=""`, deps warm:

```
cargo check -p semio-framework-mesh-engine --message-format short   → 0 errors (grep -cE ': error(\[|:)' = 0)
cargo test  -p semio-framework-mesh-engine                          → 35 passed; 0 failed (was 29 before; +6 new)
cargo check -p semio-framework  (downstream facade that path-deps mesh-engine, and separately deps
                                  os-kernel + value-derive directly) → 0 errors, clean
```

No plugin crate was built/checked (out of scope; the file list below is for the follow-up ticket to
verify against its own crates).

## Call sites this unblocks (read-only survey, not edited)

### Genuine decode-direction blockers (the ones the ticket brief named)

All three already carry a docstring naming this exact framework gap — they can drop straight to
`MeshData::from_value(...)` (or `pack::value::from_dsl_value::<MeshData>(...)`) once
`semio_framework_plugin`'s re-export surface exposes `FromValue`/`from_dsl_value` (it already
re-exports `MeshData` itself via the `pub use semio_framework::*;` glob at
`🔌️plugin/🦀️component.rs:10761`, and `ToValue` reaches the same crate the same way, so `FromValue`
should already be reachable through the identical glob — no plugin-side re-export change needed,
only the call-site rewrite):

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1088-1106`
  `mesh_data_for_preview_handle` — line 1095:
  `serde_json::from_str::<semio_framework_plugin::MeshData>(json)`. Comment at 1093-1094 already
  says: *"`MeshData` derives `ToValue` but not `FromValue` ... decoding back into it stays a
  one-directional `serde_json` boundary."*
- same file, `pending_preview_tessellate_handles` — line 1125:
  `serde_json::from_str::<semio_framework_plugin::MeshData>(json).ok()?`.
- same file, `export_mesh_from_document` — line 1312-1330, decode at line 1325:
  `.filter_map(|data| serde_json::from_value::<semio_framework_plugin::MeshData>(serde_json::Value::from(dsl::json::to_dsl_value(&data))).ok())`.
  Comment at 1317-1318: *"`MeshData` has no `FromValue` ... decoding the per-mesh `data` field back
  into it stays on `serde_json`, bridged from the `pack::json` tree."* This one is the cleanest
  mechanical win — the whole `serde_json::Value::from(dsl::json::to_dsl_value(&data))` round-trip
  collapses to `MeshData::from_value(dsl::json::to_dsl_value(&data))` directly, no `serde_json`
  detour at all.
- Same file also has ~8 more `serde_json::from_value::<semio_framework::MeshData>(...)` /
  `semio_framework_plugin::MeshData` decode sites inside `#[cfg(test)] mod tests` (lines 1888, 1907,
  1961, 2006 and similar) — lower priority than the production call sites above but equally
  mechanical to convert, and would let that test module's `serde_json` usage shrink too.

### NOT decode blockers — flagging a stale comment, not a new problem

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs:84-89`
  (`mesh_data_to_dsl`) and the structurally identical
  `.../👁️viewer/🎭️modes/👁️view/🪟️windows/📐️shape/🦀️.rs:55-59` are **encode**-direction
  (`serde_json::to_value(data)`, i.e. the `ToValue` direction), not decode. Both carry a comment
  claiming `MeshData` derives "not `ToValue`/`FromValue`" — that half is already stale:
  `impl pack::value::ToValue for MeshData` was already present in this file BEFORE this session's
  change (verified by reading the file's history in this session, not something this ticket slice
  added). These two sites were never blocked by the missing `FromValue`; if they're still on
  `serde_json` it's for a different reason (likely: `pack::value::ToValue` wasn't a convenient/known
  import path from inside the plugin crate), which this ticket slice does not change. Worth a look in
  a follow-up but it is a different problem than the one this ticket describes.

## Files touched (this slice only)

- `🧰️framework/🔨️modules/🔺️mesh-engine/🦀️.rs` — added `FromValue` impl, gated `Serialize`/
  `Deserialize` to `#[cfg(test)]`, added `mesh_data_from_value_round_trip` test module.
- `🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/Cargo.toml` — moved `serde` to
  `[dev-dependencies]`.
