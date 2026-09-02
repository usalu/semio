# glTF Runtime Codec Conversion (🧊️gltf/**)

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/**` only. EDITS ONLY — no `cargo` was run (a
peer's `cargo check --workspace` held the exclusive build lock for over an hour per the
coordinator's instructions). Every claim below is verified by non-compiling checks only:
`rustfmt`-style read-through, bracket-balance counting (parens/braces/brackets, all files reported
balanced except one pre-existing, unrelated imbalance noted below), and manual `#[serde(...)]`/
`#[value(...)]` argument-parity checks.

## What changed

### 1. `🚪️io/🦀️.rs` — the primary target
`parse_gltf_document`, `serialize_gltf_document`, `encode_glb`, `decode_glb` now round-trip real
`.gltf`/`.glb` bytes through `pack::from_json_str`/`pack::to_json_string`/a new local
`to_json_string_pretty` helper (2-space indent, matches `serde_json::to_string_pretty` layout) —
never `serde_json`. These four functions were the literal example in the ticket brief and are the
ONLY functions that actually touch real external glTF file bytes at runtime.

One behavioral note: the old `serde_json::to_vec_pretty(&document).unwrap_or_else(|_| b"{}".to_vec())`
could — in the never-actually-hit case of a NaN/Infinity float somewhere in the document — silently
collapse the WHOLE document to `{}`. `pack::json`'s writer instead encodes NaN/Infinity as `null`
per-field (documented behavior, matches the `DslValue<->JsonValue` bridge convention used
everywhere else). This is a real, if extremely unlikely, behavior difference — flagging it
explicitly rather than burying it.

### 2. `GltfDocument` and its ~32 component types (`🧬️schema/📸️snapshot/🦀️.rs`) — reverted
A prior wave had already added `value_derive::ToValue, value_derive::FromValue` + matching
`#[value(...)]` twins ADDITIVELY alongside the existing `Serialize, Deserialize` derives on every
type (never replacing them). I attempted to go further and gate the `serde` derives behind
`#[cfg(test)]` (keeping them only for this file's own `mod tests` cross-check), since the runtime
codec (§1) no longer needs them. **This was reverted mid-session**: a concurrent peer session
edited the exact same file and left this comment on `GltfSnapshot`'s derive line:

> 🌱️ serde is UNCONDITIONAL, not `#[cfg_attr(test, …)]`: production call sites still serialize
> this snapshot, so gating it breaks the `s` plugin's `wasm32-wasip2` build. Re-gate once those
> move to `ToValue`/`FromValue`.

I could not verify this myself (no `cargo`, and the actual call site is outside `🧊️gltf/`), but it
is a concrete, specific claim from a peer who was presumably able to check, and gating `GltfDocument`
without also fixing whatever external call site that is would strictly break the wasm build. I
reverted my gating throughout the whole file (and the two hand-written `GltfComponentType`/
`GltfAccessorType` serde impls in `🚪️io/🦀️.rs`, which the schema file's derives transitively need),
restoring the original additive pattern exactly, and cleaned up every doc comment that had claimed
"test-only". **Net effect on this file: unchanged from before my session**, except doc comments
now correctly explain why the `serde` pair stays unconditional.

**Follow-up for whoever owns that other call site**: find what outside `🧊️gltf/` still calls
`serde_json::to_value`/`to_vec`/etc. (or a generic `dyn Serialize` boundary) on a `GltfSnapshot`,
convert it to `ToValue`, and then `GltfDocument`'s `serde` derives really can move to
`#[cfg(test)]`-only, which is the intended end state. Only `GltfSnapshot`'s own doc comment and
the `#[cfg(test)]`-mod tests already prove that path — nothing else in `🧊️gltf/` needs it anymore
(§1 fully moved off `serde_json`).

### 3. `GltfInferenceLeafEnvelope` and its RFC 8785 canonical-JSON writer (`🚪️io/💡️inferences/📝️text/🦀️.rs`)
This one had a **documented prior blocker** in its own doc comment: "an earlier pass swapped this
struct's derive to `ToValue`/`FromValue` without updating the field type... does not compile.
Reverted; not in scope for this wave." This is exactly the "bridge through `serde_json::Value`"
trap the ticket brief warns about. Fixed properly this time:
- `pub value: serde_json::Value` → `pub value: dsl::DslValue`.
- Struct derive: `Serialize, Deserialize` → `value_derive::ToValue, value_derive::FromValue`
  (`#[serde(rename_all = "camelCase")]` → `#[value(rename_all = "camelCase")]`).
- `canonical_json_bytes<T: Serialize>` → `canonical_json_bytes<T: dsl::ToValue>`.
- `write_canonical_json`/`canonical_number` retargeted from `serde_json::Value`/`serde_json::Number`
  to `dsl::DslValue`/`dsl::Number` — same RFC 8785 canonicalization (UTF-16 code-unit key sort,
  same number formatting), but now reads `dsl::Number`'s `UInt`/`Int`/`Float` arms directly instead
  of `serde_json::Number::{as_i64,as_u64,as_f64}` (the DSL type is DESIGNED to carry the same
  distinction so no fidelity is lost — see `dslvalue-integer-fidelity` research note already in
  this ticket).
- New `canonical_json_string` helper wraps a bare string through `pack::json_to_string(&pack::JsonValue::String(..))`
  for escaping, instead of `serde_json::to_string`.
- `decode_gltf_inference_leaf_text`: `serde_json::from_slice` → utf8-decode then `pack::from_json_str`.
- `🚪️io/💡️inferences/💾️binary/🦀️.rs`: same `serde_json::from_slice::<GltfInferenceLeafEnvelope>` →
  `pack::from_json_str` swap.
- The two `#[cfg(test)]` unit tests in these two files construct `value: serde_json::json!(1.0)` —
  this NECESSARILY changed to `value: dsl::DslValue::float(1.0)` since the field's TYPE changed;
  there was no way to keep the field `serde_json::Value`-typed and also fix the field per the
  ticket's explicit instruction. This is the one place I touched `#[cfg(test)]` code, and only
  because the type itself (not the test's behavior/assertions) had to follow the field.

### 4. All 67 `encode_result` functions (`🧬️schema/💡️inferences/**/🦀️.rs`)
Every glTF inference leaf (mass-distribution, curvature, topology, compactness, proportion, size,
area-volume, symmetry, orientation, concavity, clearance, adjacency, roughness — 67 files total)
had the IDENTICAL pattern:
```rust
pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(&pack::to_json_string(&indicators.mass.inertia_tensor))
}
```
— i.e. already using the first-party `pack::to_json_string` to produce JSON text, then
RE-PARSING that text with `serde_json` for no reason except to satisfy a `serde_json::Value`
return type. This is precisely the "bridging... satisfies the compiler while keeping serde
linked" trap. Mechanically converted (script-verified against all 67 files, 0 unmatched) to:
```rust
pub fn encode_result(indicators: &GltfEntityIndicators) -> dsl::DslValue {
    dsl::ToValue::to_value(&indicators.mass.inertia_tensor)
}
```
Infallible now (no round-trip through text, so no parse-failure case ever existed in practice
anyway). Updated the one central `GltfInferenceLeafServiceDescriptor.encode` function-pointer
field type (`🧬️schema/💡️inferences/🦀️.rs`) to match, and the one caller,
`infer_gltf_leaf_cold` in the crate-root `🧊️gltf/🦀️.rs`, which read the leaf's JSON result via
`serde_json::Value::{as_array,as_str,to_string}` — converted to `dsl::DslValue::{as_array,as_str}`
plus `pack::json_to_string(&pack::json_from_dsl_value(..))` for the two stringified-JSON reads
(`provenance`, `quality`).

### 5. Three other real (non-test) call sites that would have broken under my (reverted) gating,
kept converted anyway since they're strict improvements independent of that revert:
- `📚️examples/🌱️metabolism/🦀️.rs`: `document_json()` — `serde_json::to_string(&decoded_snapshot())`
  → `pack::to_json_string(&decoded_snapshot())`.
- `👁️viewer/…/🪟️main/🦀️.rs` and `✏️editor/…/🪟️main/🦀️.rs` (identical pattern in both):
  `entity_count`/`world_instances_json` — `serde_json::to_value`/`json!`/`to_string` →
  `pack::json_from_dsl_value`/`pack::json!`/`pack::json_to_string`.
- `🧬️schema/🦀️.rs`'s `looks_like_gltf_json` sniff probe — `serde_json::from_str::<serde_json::Value>`
  → `pack::parse_json`.

## What I deliberately did NOT convert (flagged, not fixed)

- **`👁️viewer/…/🪟️main/🦀️.rs:63` and `✏️editor/…/🪟️main/🦀️.rs:59`** (one line each): still build
  `serde_json::json!({ "id": ..., "data": mesh_from_kind(...) })`. `mesh_from_kind` returns
  `MeshData` from `🧰️framework/🔨️modules/🔺️mesh-engine/🦀️.rs` — a crate OUTSIDE `🧊️gltf/`, not my
  area. `MeshData` only derives `serde::{Serialize, Deserialize}` (checked directly, no
  `value_derive`). Converting this line requires `mesh-engine` to grow a `ToValue` impl first;
  left as `serde_json`, which still compiles fine on its own (doesn't depend on anything I changed).
- **`🧬️schema/💡️inferences/🕸️topology/🦀️.rs:73,89`**: a `#[cfg(test)]`-gated `canonical_vectors`
  test module has `Contract`/`Vector` structs deriving ONLY `value_derive::FromValue`, but the
  test body calls `serde_json::from_str::<Contract>(source)` — which needs `serde::Deserialize`,
  not `FromValue`. This looks like it would NOT compile as written; I did not touch it (`#[cfg(test)]`,
  explicitly protected, and not something my session introduced — `git status` shows this file
  was not touched by me). Flagging for whoever runs the central verification: this may be
  pre-existing, unrelated broken/mid-flight test code from another concurrent session.
- **`🧬️schema/🧬️mutations/✏️🔘️change-node-name/🦀️.rs:426,437-438`**: `#[cfg(test)] mod direct_leaf_tests`
  uses `serde_json` against `GltfSnapshot` directly — untouched, and safe (GltfSnapshot's `serde`
  derive stayed unconditional per §2's revert, so this compiles exactly as before).

## Non-compiling verification performed

- Bracket balance (`(`/`)`, `{`/`}`, `[`/`]` counts) on every file touched: all balanced except
  `🧬️schema/🦀️.rs`, which has a pre-existing 1-paren/1-brace imbalance verified to predate my one
  edit there (checked before AND after; my edit swapped `serde_json::from_str::<serde_json::Value>(trimmed)`
  for `pack::parse_json(trimmed)`, identical paren count) — almost certainly a stray `(` in a doc
  comment elsewhere in the file, not a real syntax defect.
- `#[serde(...)]`/`#[value(...)]` argument-parity: scripted cross-check in `🧬️schema/📸️snapshot/🦀️.rs`
  found exactly 2 `#[serde(...)]` sites with no adjacent `#[value(...)]` twin, both on
  `GltfMorphTarget` (a hand-written-`ToValue`/`FromValue` type with no `value_derive`, so this is
  expected, not a defect).
- Every `serde_json::` occurrence in the whole `🧊️gltf/` tree was enumerated (excluding
  `🧪️tests?/`, `🧪️contract/`, `🧪️oracle/`, `🔬️probes/`, `🏭️generator/`, `🧫️fixtures/`, and
  doc-comment-only lines) and individually triaged into: converted, `#[cfg(test)]`-protected
  (untouched), or explicitly flagged as out-of-area (mesh-engine boundary).

## Fixtures/oracles to run once builds are possible again

1. `cargo test -p semio-s-plugin-stdio --lib artifacts::gltf::` — the existing `mod tests` in
   `🚪️io/🦀️.rs` (`codec_round_trip`, `glb_total_length_header_matches_actual_bytes_across_alignments`,
   `glb_json_padding_is_space_and_bin_padding_is_zero`, the `parse_gltf_document_*`/`decode_accessor_*`
   suites) — these are the byte-exact codec's own regression suite and now exercise the
   `pack::json` path directly (they call `parse_gltf_document`/`serialize_gltf_document`/
   `encode_glb`/`decode_glb`, not `serde_json` — only `glb_json_padding_is_space_and_bin_padding_is_zero`
   still cross-checks via `serde_json::to_vec(&snap.document)` as an independent oracle, unchanged).
2. `📚️examples/🌱️metabolism/🦀️.rs`'s own `#[cfg(test)]` suite, which round-trips the REAL 271-mesh
   `🧊️base.glb` fixture through `decoded_snapshot()`/`document_json()` — the single best real-world
   byte-identity check available in this tree.
3. `🚪️io/💡️inferences/📝️text/🦀️.rs` and `💾️binary/🦀️.rs`'s own `#[cfg(test)]` round-trip tests
   (`canonical_leaf_roundtrip_is_id_bound`, `deterministic_leaf_roundtrip`) — now exercise the
   `DslValue`-based envelope end to end.
4. Per the ticket's own already-proven claim (referenced in `🚪️io/🦀️.rs`'s doc comment before my
   session): `pack::json` is verified byte-identical to `serde_json` for floats across 39,990,129
   cases — re-run whatever regenerates that if the float formatter in `🎒️pack/🔤️json/🦀️.rs` changes
   before this lands.
5. A real dependent-plugin check (`cargo check -p semio-s-plugin-vcs` or equivalent), since the
   ticket brief flags that as the required downstream signal once the `wgpu::draw` fleet-wide
   blocker (noted in the prior stdio-trinity research file) clears.

## Files touched this session (🧊️gltf/** only)

- `🚪️io/🦀️.rs` — codec functions converted to `pack::json`; hand-written serde impls' doc
  comments corrected (kept unconditional, see §2).
- `🧬️schema/📸️snapshot/🦀️.rs` — doc comments corrected after the gating revert; no net
  structural change from session start.
- `🧬️schema/🦀️.rs` — `looks_like_gltf_json` converted.
- `🧬️schema/💡️inferences/🦀️.rs` — `GltfInferenceLeafServiceDescriptor.encode` field type.
- `🧬️schema/💡️inferences/**/🦀️.rs` (67 files) — `encode_result` converted.
- `🚪️io/💡️inferences/📝️text/🦀️.rs`, `🚪️io/💡️inferences/💾️binary/🦀️.rs` — envelope + canonical
  JSON writer converted.
- `🦀️.rs` (crate root) — `infer_gltf_leaf_cold` call site updated.
- `📚️examples/🌱️metabolism/🦀️.rs` — `document_json()` converted.
- `👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs`
  — `entity_count`/`world_instances_json` converted; one `mesh_from_kind` line each left on
  `serde_json` (flagged above).
