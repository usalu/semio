# F6 — 🧊️gltf 2.0 — OpText/OpBinary + DiffCodec Report

**Artifact**: `🧊️gltf`, standard `2.0`, path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/`.
**Scope worked**: `protocol::DiffCodec for GltfDiff` (🔺️diff/component.rs) and `protocol::OpText`/
`protocol::OpBinary for GltfMutation` (🧬️mutations/component.rs). Snapshot/diff/mutation SHAPE
untouched — `GltfDiff`'s `DiffAlgebra` impl and `GltfMutation`'s `Mutation` impl (both F4 work) are
exactly as found.

## Classification — verified for real, not trusted from the recon table

The recon's row #23 guessed **HAND-ROLL (3b) — LARGE**, flagging "0 enums" as unconfirmed (glTF's
own `GltfJson` for `extras`/`extensions` needed checking as a possible diff-reachable enum). Per
`f6-recon-report.md` §9 Step 1, both sides were tested for real before writing any grammar:

- **Diff side**: temporarily added `#[derive(dsl::DslDiff)]` to `GltfDiff`, ran
  `cargo check -p semio-s-plugin-stdio --lib`. **77 `E0277` errors** (full output:
  `f6-gltf-diff-derive-check1.txt`), then reverted. Two independent, simultaneous blockers, both
  worse than what the recon table anticipated:
  1. **New blocker beyond 3a/3b**: every one of the 14 top-level arrays is typed through the
     GENERIC `GltfCollectionDiff<T, D>` wrapper (e.g. `GltfCollectionDiff<GltfScene,
     GltfSceneDiff>`, `GltfCollectionDiff<GltfCamera, GltfCamera>`) — `DslField` has **no blanket
     impl for any user-defined generic struct** in the `dsl` crate (only `Vec<T>`/
     `BTreeMap<String,T>`/`[T;N]` do), so the derive fails on every collection field regardless of
     enum/tri-state content. This alone would force hand-rolling even if the tree had zero enums
     and zero tri-state fields.
  2. **3a, resolving the recon's open question**: `Option<GltfJson>` is not `DslField` —
     `GltfJson` (`Null`/`Bool`/`Number`/`String`/`Array`/`Object`) IS a real data-carrying enum,
     confirmed reachable via 20+ `Option<Option<GltfJson>>` extras/extensions fields.
     `GltfCameraProjection` (`Perspective`/`Orthographic`, inside `GltfCamera`) is a second
     data-carrying enum in the tree, reachable via the `cameras` field. So **"0 enums" does NOT
     hold** — the recon's flagged uncertainty resolves to: there are 2.
  3. 3b (tri-state) also fires independently on every `Option<Option<T>>` field (`GltfSourceForm`
     itself additionally lacks `DslField` since it was never `DslScalar`-derived, a third,
     independent, trivially-fixable-in-isolation reason that doesn't change the verdict).
- **Mutation side**: temporarily added `#[derive(dsl::DslOps)]` to `GltfMutation`, ran the same
  check. **33 `E0277` errors** (full output: `f6-gltf-mutation-derive-check1.txt`), then reverted.
  `SetSnapshot{snapshot: GltfSnapshot}` recursively requires `DslField` on `GltfAsset`/`GltfScene`/
  `GltfNode`/`GltfMesh`/`GltfAccessor`/`GltfMaterial`/`GltfBuffer`/`GltfAnimation`/`GltfSnapshot`
  itself (none `DslRecord`-derived), and even if every one of those got `#[derive(dsl::DslRecord)]`
  added, the walk would still hit `GltfJson`/`GltfCameraProjection` and fail per 3a. Both sides:
  **HAND-ROLL**, confirmed, not assumed.

Both derive attempts were reverted immediately after capturing the compiler output — `GltfDiff`
and `GltfMutation`'s derive lists are unchanged from what F4 left them (no `dsl::DslDiff`/
`dsl::DslOps` anywhere in the final diff).

## What was built

### `🔺️diff/component.rs` — `HandcraftedDiffCodec` region (~1120 lines)

Grammar follows the same conventions as `GifDiff`/`SvgDiff`'s hand-rolled codecs (`f6-recon-report.md`
§5): bracket-depth-aware `split_top_level`, hex (not base64) for strings/bytes, `[0]`/`[1,x]` for
`Option<T>`, positional `[f1,f2,...]` tuples for structs, single-letter tag prefix for data-carrying
enums, `[removed];[modified];[added]` for collection triples, space-separated `name=value` for the
top-level line, `encode_diff`/`encode_op` = the text bytes verbatim (same simplification `GifDiff`/
`SvgDiff`/`WriterDiff` all use — satisfies every `DiffCodec` law without inventing a second wire
format). Given the size (by far the largest hand-roll in the F6 program per the recon's own sizing),
the value codecs are grouped by field GROUP, per the recon's own suggested structure, as regions:

- `Primitives` — `hex_encode`/`hex_decode`/`enc_str`/`dec_str`/`split_top_level`/`strip_brackets`/
  `encode_option`/`decode_option` (identical to gif89a/svg's copies) plus two NEW primitives this
  artifact needed that neither prior hand-roll did: `encode_option_option`/`decode_option_option`,
  a real `Option<Option<T>>` two-layer helper — gif89a/svg's own tri-state fields always lived at
  the TOP level of their Diff struct (where "token present or not" peels the outer layer for free);
  gltf's tri-state fields are overwhelmingly NESTED inside per-entity diff structs
  (`GltfNodeDiff::mesh`, `GltfAccessorDiff::sparse`, …) that are themselves embedded as one
  positional field inside a LARGER bracketed tuple, where there's no "absent token" to lean on —
  both Option layers must be explicit.
- `ScalarCodecs` — `f64`/`u64`/`bool`/`Vec<f64>`/`[f64;N]` (const-generic `dec_f64_array::<N>`)/
  `Vec<usize>`/`Vec<String>`/`Vec<(String,usize)>` (the `GltfPrimitive.attributes` shape).
- `GltfJsonCodec` — `GltfJson`'s 6 variants: `Z`=Null (bare), `B[0|1]`=Bool, `F[<f64>]`=Number,
  `S[<hex>]`=String, `A[v,v,...]`=Array, `O[k:v,...]`=Object (member order preserved, matching
  `GltfJson::Object`'s own `Vec<(String,GltfJson)>` shape, never a map).
- `UnitEnumCodecs` — `GltfComponentType` (reuses its own `code()`/`from_code()`, the spec numeric
  code), `GltfAccessorType` (reuses `as_str()`/`from_str()`, the spec string), `GltfAlphaMode`/
  `GltfInterpolation`/`GltfAnimationPath`/`GltfSourceForm` (word tags).
- `AssetSceneNodeGroupCodecs`, `MeshAccessorMaterialGroupCodecs`, `BufferGroupCodecs`,
  `TextureImageSamplerSkinGroupCodecs`, `AnimationGroupCodecs`, `CameraGroupCodecs` — one
  `enc_<T>`/`dec_<T>` pair per struct type reachable from the diff tree (asset, scene, node,
  primitive, mesh, sparse indices/values/accessor, accessor, texture info ×3, pbr, material,
  buffer, buffer view, texture, image, sampler, skin, animation channel/target/sampler, animation,
  perspective, orthographic, camera), plus every corresponding `*Diff` type's own codec
  (`enc_asset_diff`, `enc_scene_diff`, `enc_node_diff`, `enc_mesh_diff`, `enc_accessor_diff`,
  `enc_material_diff`, `enc_buffer_diff` — the 7 STRONG-entity diff types) — `GltfCameraProjection`
  gets tag `P`=Perspective/`O`=Orthographic (the artifact's second, previously-unflagged, enum).
- `GenericCollectionCodec` — ONE real generic `enc_collection<T,D>`/`dec_collection<T,D>` pair
  (mirrors `GltfCollectionDiff<T,D>` itself being one generic algebra, not 14 hand-duplicated ones,
  per that struct's own doc comment) — every one of the 14 top-level arrays' `print`/`parse` calls
  through this same pair of functions with different `enc_item`/`enc_diff` closures; WEAK
  collections (bufferViews/buffers-bytes/textures/images/samplers/skins/animations/cameras) pass
  the same item encoder for both `enc_item` and `enc_diff` since `D = T` there.
- `Document` — `enc_document`/`dec_document` (whole `GltfDocument`, all 19 fields) and
  `enc_gltf_snapshot`/`dec_gltf_snapshot` (schema + document + raw buffer bytes + source form) —
  built entirely from the above, positioned in the diff file since that's where every constituent
  codec already lives; consumed by `GltfMutation::SetSnapshot`'s hand-rolled `OpText`/`OpBinary`.
- `TopLevel` — `print_gltf_diff`/`parse_gltf_diff` (21 space-separated `name=value` tokens, one per
  `GltfDiff` field) and the `impl protocol::DiffCodec for GltfDiff` (4 methods: `print_diff`/
  `parse_diff` delegate to the above; `encode_diff`/`decode_diff` = text bytes verbatim).

### `🧬️mutations/component.rs` — `OpCodecs` region (~110 lines, replacing the `serde_json` stubs)

`print_gltf_mutation`/`parse_gltf_mutation` (`keyword arg=value ...`, one match arm per variant,
same shape the derive's own handcrafted wrapper uses per `f6-recon-report.md` §2, even though
nothing here derives `DslVariants`) plus `impl protocol::OpText`/`impl protocol::OpBinary for
GltfMutation`. Every variant reuses the diff module's `pub(crate)` codecs — the mutation file adds
ZERO new value codecs of its own (a second confirmation, alongside gif89a's precedent, that
`SetSnapshot`-carrying mutation enums can reuse their sibling Diff file's grammar wholesale when
every payload type the Mutation touches is already covered there).

## Tests (both mandatory, both real, both green)

- `diff_codec_text_binary_roundtrip_law` (🔺️diff/component.rs, new `handcrafted_diff_codec_tests`
  module) — exercises a **representative SUBSET of the 42 tri-state fields, documented in the
  test's own doc comment**:
  1. `sweep_a()`/`sweep_b()` — the file's PRE-EXISTING `field_sweep_covers_every_mutable_field`
     fixture, factored out (`pub(super) fn`) and reused rather than re-derived: every top-level
     `GltfDiff` field populated at least once (all 14 collections, `asset`/`scene`/
     `extensions_used`/`extensions_required`/`extensions`/`extras`/`source_form`), every
     `GltfAssetDiff` field going `Some -> None`, a `Perspective` camera.
  2. `tristate_snapshot_a`/`tristate_snapshot_b` — NEW fixture targeting the tri-state fields
     `sweep_a`/`sweep_b` don't touch: `GltfNodeDiff::mesh`/`camera`/`skin`/`matrix` going
     `Some(Some) -> Some(None)` AND `translation`/`rotation`/`scale` going `Some(None) ->
     Some(Some)` on the SAME collection-modified entry (both tri-state directions at once),
     `GltfAccessorDiff::sparse` going `None -> Some`, `GltfMaterialDiff::
     pbr_metallic_roughness`/`normal_texture`/`occlusion_texture`/`emissive_texture` all going
     `None -> Some`, `GltfBufferDiff::uri` going `Some -> None`, an `Orthographic` camera (the
     OTHER `GltfCameraProjection` variant), and `GltfJson::Null`/`Number`/`Array` (sweep only hits
     `Bool`/`String`/`Object`).
  - Together: every `GltfJson` variant, both `GltfCameraProjection` variants, and at least one
    tri-state field per STRONG-entity diff type are exercised. `GltfDiff::default()` (empty diff)
    is a third case. All 5 cases assert `!printed.contains('\n')`, `parse(print(x)) == x`,
    `decode(encode(x)) == x`.
- `op_text_binary_roundtrip_law` (🧬️mutations/component.rs, existing `mod tests`) — every one of
  the 25 `GltfMutation` variants, incl. `SetSnapshot` against a NEW `full_snapshot()` fixture
  (bufferViews/textures/images/samplers/skins populated, plus an `Orthographic` camera — the WEAK
  collections `base_snapshot()` never populates) and representative Insert/Remove/Set triples for
  every STRONG entity. Same 3 assertions per case.

## Verification (real, this session)

| Check | Result |
|---|---|
| `cargo check -p semio-s-plugin-stdio --lib` with `dsl::DslDiff` temporarily on `GltfDiff` | 77 `E0277` errors (`f6-gltf-diff-derive-check1.txt`) — reverted |
| `cargo check -p semio-s-plugin-stdio --lib` with `dsl::DslOps` temporarily on `GltfMutation` | 33 `E0277` errors (`f6-gltf-mutation-derive-check1.txt`) — reverted |
| `cargo check -p semio-s-plugin-stdio --lib` (final, both hand-rolled codecs in place) | 0 gltf-related errors/warnings (`f6-gltf-mutation-check2.txt`). Transiently saw 4 unrelated errors from `pptx`/`pdf1.7` mutations files mid-edit by concurrent sibling F6 sessions (`f6-gltf-diff-check1.txt`/`f6-gltf-mutation-check1.txt`, missing `use protocol::OpText;`) — confirmed NOT caused by this session (those files are `M`/`MM` in `git status` under other artifacts, untouched by me) and resolved on their own by the next check, per the "concurrent cargo workspace churn" pattern. |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::gltf"` | **37/37 passed** (35 pre-existing incl. the 🌱️metabolism example + 2 new law tests), 0 failed (`f6-gltf-test2.txt`) |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate) | **1061 passed, 0 failed** (`f6-gltf-full-crate-test.txt`) — count only went up from the S1-F6b baseline (1033+), consistent with sibling F6 agents' concurrent work plus this session's +2 |

## Deviations from the recon's §5 grammar template

- Added `encode_option_option`/`decode_option_option` (two-layer `Option<Option<T>>` helper) —
  not present in gif89a/svg's primitive set because neither of THEIR tri-state fields was ever
  nested inside a larger bracketed tuple; gltf's overwhelmingly are (42 occurrences, virtually all
  inside per-entity `*Diff` structs, not at `GltfDiff`'s own top level). Documented inline as a
  genuine, non-duplicative addition to the shared primitive vocabulary (candidate for the
  recon's flagged future "shared hand-roll helpers module" once it exists).
- Added `enc_collection`/`dec_collection` as ONE real generic pair over `GltfCollectionDiff<T,D>`
  rather than one hand-duplicated triple-codec per collection (gif89a/svg had no generic collection
  wrapper to reuse — `GltfCollectionDiff<T,D>` is itself F4's own generic algebra, this just extends
  that same generality to the grammar layer, consistent with the struct's own doc comment).
- `GltfCameraProjection` is a SECOND data-carrying enum this artifact's snapshot module has, beyond
  `GltfJson` — not flagged by the recon's file-level `pub enum` grep (defined in `📸️snapshot/
  component.rs`, not `🔺️diff/component.rs`, so the recon's per-file sweep couldn't see it). Both
  are now confirmed and covered by the grammar (`P`/`O` tags) and by the law tests.
- No changes to `POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts`) — not touched, per instructions.
- No changes to `📦️glue.rs`, the SDK traits, `schema`/`dsl`/`protocol` modules, or `🏪️store` — verified via `git status`/`git diff --stat` scoped to those paths, both empty for this session.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — added imports (`GltfAlphaMode`/`GltfAnimation*`/`GltfCameraProjection`/`GltfImage`/`GltfInterpolation`/`GltfNormalTextureInfo`/`GltfOcclusionTextureInfo`/`GltfOrthographic`/`GltfPbrMetallicRoughness`/`GltfPerspective`/`GltfSparseIndices`/`GltfSparseValues`/`GltfTextureInfo`), the `HandcraftedDiffCodec` region (`impl protocol::DiffCodec for GltfDiff` + ~50 `enc_*`/`dec_*` value codecs), factored `sweep_a()`/`sweep_b()` out of `field_sweep_covers_every_mutable_field` into `pub(super) fn`s, added `handcrafted_diff_codec_tests` module with `diff_codec_text_binary_roundtrip_law`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — added imports (grammar functions from the diff module, `protocol::OpText`), replaced the `serde_json`-based `OpText`/`OpBinary` stub with the hand-rolled `print_gltf_mutation`/`parse_gltf_mutation` + trait impls, added `full_snapshot()` fixture + `op_text_binary_roundtrip_law` test.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-gltf-diff-derive-check1.txt`, `f6-gltf-mutation-derive-check1.txt`, `f6-gltf-diff-check1.txt`, `f6-gltf-mutation-check1.txt`, `f6-gltf-mutation-check2.txt`, `f6-gltf-difftest-check1.txt`, `f6-gltf-test1.txt`, `f6-gltf-test2.txt`, `f6-gltf-full-crate-test.txt`.

## Summary JSON

```json
{
  "artifact": "gltf",
  "standard": "2.0",
  "diff_path": "hand-roll",
  "mutation_path": "hand-roll",
  "tests_passed": 1061,
  "tests_failed": 0,
  "deviations": [
    "Added encode_option_option/decode_option_option (2-layer Option<Option<T>> helper) — not needed by gif89a/svg since their tri-state fields live at the Diff struct's own top level; gltf's 42 tri-states are overwhelmingly nested inside per-entity *Diff structs instead.",
    "Added one real generic enc_collection<T,D>/dec_collection<T,D> pair over GltfCollectionDiff<T,D> instead of per-collection duplication, matching that struct's own generic-algebra design.",
    "Confirmed GltfCameraProjection as a second data-carrying enum (beyond GltfJson) not visible to the recon's file-level grep since it's declared in snapshot/component.rs, not diff/component.rs — both are now covered."
  ],
  "report_path": ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6-gltf-report.md"
}
```
