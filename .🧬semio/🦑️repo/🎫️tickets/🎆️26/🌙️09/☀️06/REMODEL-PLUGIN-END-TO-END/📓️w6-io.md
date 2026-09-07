io() landed — `pub fn io() -> IoDeclaration` lives at `…/✳️any/🚪️io/🦀️.rs:456` (committed 2d2b39eb7f); W11 can wire `subset.io = crate::artifacts::remodeling::standards::v1::subsets::any::io::io()` now.

# W6 — 📸️remodel io: moved onto the typed `IoDeclaration` channel, all 7 broken casts replaced

Scope owned: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/**/🦀️.rs`
plus the ONE `.composers(…)` line in the artifact root and the two dwg `#[path]` mounts in
`📦️packages/🦀️rust/🦀️.rs`. All paths relative to `/Users/ueli/Documents/semio`.

## 1. Channel diff

| | before | after |
|---|---|---|
| registration | `ArtifactDeclaration::builder(…).composers(io_registry::entries())` — 1 native + 8 export `ComposerEntry` rows | `io()` returns `IoDeclaration { native: NativeCodecs{…}, entries: &[IoEntry; 16] }`; `.composers(native_composer_entries())` keeps ONLY the derive-generated native composer |
| per-hop type | free `serialize_bytes`/`deserialize_bytes` fns behind `compose_export_*` thunks | `impl Serializer<RemodelingSnapshot>` / `impl Deserializer<RemodelingSnapshot>` per leaf, erased by `serializer_entry`/`deserializer_entry` |
| fidelity | not modelled (every row `IoConfidence::Medium`) | declared per hop (`IoFidelity::Exact` for json/txt, `Lossy` for the six geometry/raster hops) |
| sniffing | none | every import leaf overrides `Deserializer::sniff` on the format's real magic (`ply`, `LASF`, `glTF`, PNG's 8-byte signature, `solid`, `v `/`f `, `{`) |
| composition facet | `RemodelingComposerComposition::reads()` listed 10 dialects, 9 of them delegating to a broken import leaf | native dialect only |

**The typed entries are written and preflight-shaped but NOT yet reachable at runtime.** The only
call site of `io_mechanism::io_register` is `commit_artifact_declarations`, which is fed by
`Plugin::builder(…).declare_artifact(app::declarations::ArtifactDeclaration{…})`. remodel is still on
the OLD `.artifact(declaration()?)` builder, which has no `.io(IoDeclaration)` method. Completing the
wiring means adding a subset root + standard root + `artifact()` and swapping four lines in the
plugin root — all of which need `editor_surface::<RemodelingPlayApp, RemodelApps>` /
`viewer_surface::<RemodelingViewer, RemodelApps>`, i.e. W4's in-flight editor/viewer rewrite. **Handover
below (§6).** Until then `io()` is the single source of truth for this subset's io surface and is
exercised by the Rust tests in §4.

## 2. Per-format table

| dialect | dir | fidelity | mapping | payload | fixture |
|---|---|---|---|---|---|
| `s.stdio.json@rfc8259/*` | export | Exact | `dsl::ToValue` → `pack::json::from_dsl_value` → stdio `write_json_pretty` | Text | round-trip test |
| | import | Exact | stdio `parse_json_text` → `JsonSnapshot::to_serde_value` → `dsl::FromValue` | Text | round-trip test |
| `s.stdio.txt@utf-8/*` | export | Exact | `store::ArtifactDsl::print_dsl` — the txt rendition of a scene IS its `.remodeling` DSL text | Text | round-trip test |
| | import | Exact | `store::ArtifactDsl::parse_dsl` | Text | round-trip test |
| `s.stdio.ply@1.0/*` | export | Lossy | `results.mesh` if a bounded durable mesh resolves, else `results.dense`/`results.sparse` → `SemioMeshToPly` → `ply::engine::encode_ply` | Binary | `🧫️fixtures/🧱️four-points.ply` |
| | import | Lossy | `decode_ply` → `SemioMeshFromPly` → `results.sparse` (+ `results.mesh` when the file carried faces) | Binary | same |
| `s.stdio.las@1.0/*` | export | Lossy | `results.dense` else `results.sparse` → `SemioMeshToLas` → `las::engine::encode_las` | Binary | encode→decode test |
| | import | Lossy | `decode_las` → `SemioMeshFromLas` → `results.sparse` | Binary | encode→decode test |
| `s.stdio.obj@3.0/*` | export | Lossy | `results.mesh` → `SemioMeshToObj` → `obj::engine::encode_obj` | Text | `🧫️fixtures/🗿️unit-cube.obj` |
| | import | Lossy | `decode_obj` → `SemioMeshFromObj` → durable mesh in `results.mesh` (`MeshSource::Imported`) | Text | same |
| `s.stdio.stl@ascii/*` | export | Lossy | `results.mesh` → `SemioMeshToStl` → `stl::engine::encode_stl_ascii` | Text | `🧫️fixtures/🔺️unit-tetra.stl` |
| | import | Lossy | `decode_stl_ascii` → `SemioMeshFromStl` → durable mesh | Text | same |
| `s.stdio.gltf@2.0/*` | export | Lossy | `results.mesh` → `SemioMeshToGltf` → `gltf::engine::encode_glb` (self-contained GLB) | Binary | encode→decode test |
| | import | Lossy | `decode_glb` → `SemioMeshFromGltf` → durable mesh | Binary | encode→decode test |
| `s.stdio.png@1.2/*` | export | Lossy | DSM → ortho → DTM → mesh texture, read back through the real `s.stdio.semio/v1/image` ↔ png `io_dispatch` bridge | Binary | `🧫️fixtures/📷️two-by-two.png` |
| | import | Lossy | `decode_png` → `ImageAsset` + a one-frame `MediaKind::ImageSequence` stream over a real asset handle | Binary | same |
| `s.stdio.dwg@ac1018/*` | — | — | **leaf + both mounts DELETED** — a DWG is a 2D/3D CAD drawing database (layers, entities, blocks, xrefs); a remodeling scene has no drawing, and a DWG carries no media streams, calibration, GCPs or point cloud. There is no mapping to make honest at any fidelity, so the hop is unregistered rather than registered-and-refusing. | | |

### Import-side semantics worth naming

- **`results.dense` is never produced by io.** A dense cloud is distinguished from a sparse one in
  this schema by `DenseCloud.confidence` (and `classification`). Neither `SemioMeshSnapshot` nor any
  of the five foreign mesh dialects as stdio models them carries a per-point confidence channel, so
  "dense if confidence present" is unreachable through this path. Documented on
  `scene_from_semio_cloud` rather than faked.
- **Imported meshes are REAL durable content.** `seed_remodeling_mesh` frames the `MeshData` into the
  exact chunk shape `apply_mesh_chunk` replays (one leading field tag 0..4, ≤4092 payload bytes,
  non-decreasing tag order), inserts it into `durable_artifacts` under a content-addressed id, and
  sets a `replayable_remodeling_mesh_handle` — the same shape `🏗️run-reconstruction`'s
  `TerminalPhase::Mesh` commits. A mesh beyond the bounded 512-vertex / 512-triangle envelope is
  rejected up front with its measured size, because storing it would produce a handle that
  `resolve_bounded_remodeling_mesh` can never resolve again.
- **Every "nothing to export" path is a typed `Err` naming the missing field**, never an empty
  snapshot or a blank canvas.

## 3. Deleted code

- `io_registry` module (128 lines): `rebuild_native_snapshot`, 8 `EXPORT_*_DIALECT` consts, 8
  `compose_export_*` thunks, the `OnceLock<Vec<ComposerEntry>>` table. Replaced by
  `native_composer_entries()` (the one surviving row).
- `derived_composition`'s 9 foreign-dialect branches and 9 `DEP_*` consts.
- 18 leaf bodies: 14 cross-type `ArtifactPack` casts (`serialize`/`serialize_bytes` +
  `deserialize`/`deserialize_bytes` for las/ply/png/dwg/stl/gltf/obj), 2 `"not yet implemented"` txt
  stubs, 2 real json bodies (preserved verbatim inside the new typed impls).
- 18 no-op `pub async fn register() {}` leaf functions.
- `🖊️dwg/🔖️ac1018/✳️any/{🦀️.rs,🟦️.ts}` × 2 directions, plus their two `#[path]` mount blocks in
  `📦️packages/🦀️rust/🦀️.rs`.
- `#[cfg(test)]` removed from `semio_mesh_to_mesh_data`: it is production code now (every mesh import
  hop runs it).
- `import_stdio_kinds`/`export_stdio_kinds`/`mesh_to_*_bytes`/`remodeling_png_export`/the semio-image
  bridge de-`async`ed (they never suspend; every one was already being called without `.await`).

## 4. Tests

18 tests in `🚪️io/🦀️.rs`'s `io_tests` module (inline, so `include_bytes!`/`include_str!` resolve
against the leaf directory). Four committed fixtures under `🚪️io/🧫️fixtures/`, all authored by
`🐍️w6-io-fixtures.py` from Python's stdlib only (`struct`, `zlib`) or hand-written text — **never by
this repo's own encoders**, so reading one back is a real cross-implementation check:

| fixture | bytes | content |
|---|---|---|
| `🧱️four-points.ply` | 279 | ascii PLY 1.0, 4 vertices, `x y z red green blue` (uchar), no faces |
| `🗿️unit-cube.obj` | 218 | 8 shared vertices, 12 triangles |
| `🔺️unit-tetra.stl` | 520 | ascii solid, 4 facets with explicit normals |
| `📷️two-by-two.png` | 75 | real 2×2 RGBA8 PNG (IHDR/IDAT/IEND, zlib level 9, CRC32) |

Generators kept in the ticket folder: `🐍️w6-io-leaves.py` (the 16 leaves), `🐍️w6-io-fixtures.py`.
Re-running either re-emits the pre-`rustfmt` layout — run `rustfmt` over the emitted paths afterwards.

### Oracle rows for W2b (`✳️any/🔮️oracle/🔣️.json`)

`las` 0.11, `ply-rs` 0.1, `tobj` 4 and `stl_io` 0.8 are already `[dev-dependencies]` of
`semio-s-plugin-stdio-test-oracle` (`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml`,
feature `oracles`), and remodel's plugin-root `🧪️oracle/🔣️.json` already names that package as its
`oracleHostPackages[rust]`. What is missing is the per-subset manifest rows. Suggested entries, one
`oracle` per (format, reader), each `vectors[]` pointing at the fixture asset by name:

| oracle id | implementation | reference crate | vector asset | assertion |
|---|---|---|---|---|
| `ply-rs-remodeling-ply-1-0-import` | rust | `ply-rs` 0.1 | `🚪️io/🧫️fixtures/🧱️four-points.ply` | reader's 4 vertices + rgb == `results.sparse.{points,colors}` |
| `tobj-remodeling-obj-3-0-import` | rust | `tobj` 4 | `🚪️io/🧫️fixtures/🗿️unit-cube.obj` | reader's 8 positions / 36 indices == replayed `results.mesh` |
| `stl-io-remodeling-stl-ascii-import` | rust | `stl_io` 0.8 | `🚪️io/🧫️fixtures/🔺️unit-tetra.stl` | reader's 4 facets == replayed 36 position floats |
| `las-remodeling-las-1-0-export` | rust | `las` 0.11 | (produced by `RemodelingIntoLas`) | reader's point count + xyz within LAS scale quantization |

`las` and `gltf` have no committed hand-authored fixture: a LAS 1.2 header and a GLB container are
both byte-layout-sensitive enough that a hand-rolled Python writer would be testing my transcription
of the spec rather than the codec. They are covered instead by an encode→decode pass through stdio's
own real engines, and the `las` oracle row above closes the independence gap from the export side.

## 5. Compile / test status

_(filled in below once the shared cargo lock cleared — see §7.)_

## 6. Handover to W4 (plugin root + artifact root)

Four items, none of which W6 may touch:

1. **Wire the typed channel.** Add `pub fn subset() -> SubsetDeclaration<crate::RemodelApps>` (subset
   root), `pub fn standard() -> StandardDeclaration<…>`, and `pub fn artifact() -> app::declarations::ArtifactDeclaration<…>`;
   then in `✏️s/🔌️plugins/📸️remodel/🦀️.rs` replace `.artifact(…) .editor::<…> .editor_mutation_roster::<…> .viewer::<…> .viewer_mutation_roster::<…>`
   with `.declare_artifact(crate::artifacts::remodeling::artifact())`. The subset's `io` field is
   literally `io: crate::artifacts::remodeling::standards::v1::subsets::any::io::io()`.
2. **Then delete `pilot_languages()`** from `🗿️artifacts/📸️remodeling/🦀️.rs` and the `.languages(…)`
   + `.document_codec::<…>()` builder calls: `io()`'s own `languages()` + `NativeCodecs.codec` are the
   same five specs and the same codec, duplicated only for the duration of the transition.
3. **`artifact_kind()` (lines 36-37) still lists `"stdio.dwg"`** in both `export_stdio_kinds` and
   `import_stdio_kinds`, and drops `"stdio.txt"` which is now a real Exact hop both ways. The truthful
   lists are `crate::…::io::{export,import}_stdio_kinds()` (this file exports both).
4. **`definition()` still declares `composer.format-5` = `s.stdio.dwg@ac1018/*`** (line 62) and eight
   `composer.format-*` rows generally. With `.composers()` trimmed to the native row those claims are
   unused; the capability set that matches the new channel is one row per registered `IoEntry` target
   dialect, dwg absent, `s.stdio.txt@utf-8/*` present. The owner-root `🔣️.json` /
   `🛂️.descriptor.semio` regeneration (`describe`) in the ticket's DoD #3 must run after that edit.

## 7. Verification log

## 8. W6b continuation (2026-09-06 21:00→, after W6 was killed by the Opus session limit)

**Predecessor state found on disk = fully committed** (`2d2b39eb7f`, 12:08; `git diff 5e03e56997 -- 🚪️io`
is exactly that commit, 42 files, +1453/-663). Everything §1-§4 above describes had LANDED, including
`pub fn io()` (`🚪️io/🦀️.rs:456`), the 16 typed `IoEntry` rows, the 4 fixtures, the 19 tests and the
dwg deletion (`grep dwg 📦️packages/🦀️rust/🦀️.rs` → 0 hits). Nothing was redone.

Two things it never got to, both consequences of W9's serde elimination landing AFTER it wrote the file
(`📓️w9-schema-engine-compile.md` §8 lists them as W6's):

| site | before | after |
|---|---|---|
| `🚪️io/🦀️.rs:38` | `use serde_json::Value;` | removed — no `serde_json` anywhere in `🚪️io/**` now |
| `🚪️io/🦀️.rs:117` `remodeling_mesh_from_document(&Value)` | `serde_json::from_value::<RemodelingSnapshot>` then `scene_mesh_data` | **deleted** — zero callers in the crate, and `scene_mesh_data(&scene)` is the same function without the impossible decode |
| `🚪️io/🦀️.rs:138` `remodeling_png_export(&Value)` | decoded the doc, then `remodeling_png_asset` | signature is now `(&RemodelingSnapshot)`; `RemodelingSnapshot` left the serde type graph with `store::ArtifactChild`, so a `serde_json::Value` is no longer a carrier this crate can decode |
| `io_tests::png_export_round_trips_a_stored_texture_asset` | `remodeling_png_export(&serde_json::to_value(&scene)…)` | `remodeling_png_export(&scene)` |
| `📥️import/…/🔣️json/…/🦀️.rs:16` | `JsonSnapshot::from_value(v).to_serde_value().into()` | `pack::json::to_dsl_value(&JsonSnapshot::from_value(v).to_pack_value())` — stdio's own first-party analog (`🧾️json/…/📸️snapshot/🦀️.rs:619`), no `serde_json::Value` hop |

The export json leaf already used `pack::json::from_dsl_value(&from.to_value())` + `From<pack::JsonValue>
for JsonValue`, so both json directions are now `pack::json` end to end and stay `IoFidelity::Exact`.

### Static verification done in place of the (blocked) compiler

- every `use semio_s_plugin_stdio::…` path in `🚪️io/**` (20 distinct) resolved against
  `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`'s `#[path]` mount names — the mesh-subset io leaves
  mount as `gltf`/`stl`/`obj`/`ply`/`las` (module names, not the `🎬️`/`🧊️` dir emoji) and all ten
  `SemioMesh{To,From}*` structs exist; `decode_ply`/`encode_ply`/`decode_las`/`encode_las`/`decode_obj`/
  `encode_obj`/`decode_stl_ascii`/`encode_stl_ascii`/`decode_glb`/`encode_glb`/`decode_png`/`encode_png`
  all exist with the signatures the leaves call.
- `Serializer`/`Deserializer` (`🧰️framework/🔨️modules/🚪️io/🦀️.rs:2373,2387`) are still future-returning,
  so the leaves' `async fn serialize/deserialize/sniff` and `serializer_entry`/`deserializer_entry`
  (`:2669,:2710`, both `S: store::ArtifactPack`) match; `IoDeclaration`/`NativeCodecs`/`LanguagePair`
  are at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27872-27904`, field-for-field as `io()`
  constructs them.
- no crate-internal reference to the deleted `io_registry`, `remodeling_mesh_from_document` or the dwg
  leaves survives (only prose in this file's module doc and one stale sentence at
  `🗿️artifacts/📸️remodeling/🦀️.rs:105`, W11's file).

### Still owned by other lanes

- `✏️s/🔌️plugins/📸️remodel/🔣️.json` (owner-root descriptor) still names the dwg hop — it is regenerated
  by `describe` (ticket DoD #3) after W11 edits `artifact_kind()`/`definition()`; §6 items 1-4 above are
  unchanged and still W11's.
