# W4 — `remodel` composes stdio mesh + image

**ucas-status: partial** (both duplication shapes identified in the design map landed as real,
verified composition — `results.mesh.mesh` → `s.stdio.semio/v1/mesh` child, `assets` →
`s.stdio.semio/v1/image` children — with 502/504 crate tests passing; marked `partial` rather than
`complete` because the JPEG image bridge, the sibling-language schema-facet mirrors, and a
pre-existing dangling example-fixture test mount are honestly deferred, see `## Deferred` below)

## Pre-flight

Per `📌️important.md`: remodel was skipped in batch 1 because a live uncommitted edit
(`🎛️apps/📸️remodel/⚙️engine/🎥️video/🦀️component.rs`) matched ticket #2553's in-flight `⚙️engine`
dissolution pattern. Re-checked before starting this dispatch:

```
git diff --stat -- ✏️s/🔌️plugins/📸️remodel   → (empty)
git status --porcelain -- ✏️s/🔌️plugins/📸️remodel → (empty)
git log --oneline -5 -- .../🎥️video/🦀️component.rs → 515271bf60 (🚩️503), 31209e7afe (🚩️498)
```

Clean — cleared to proceed, matching `📌️important.md`'s own note that remodel was "re-dispatched
after #2553's edit cleared."

## What the codebase actually looks like (verified against code, not the one-line design summary)

`✏️s/🔌️plugins/📸️remodel` has exactly **one** artifact root (`📸️remodel`, `s.remodel`), one subset
(`✳️any`), declaring kind `3d.remodel`. `RemodelSnapshot` (`🗿️artifacts/📸️remodel/🏅️standards/🔖️1/
🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`) has two real content-duplication shapes:

1. **`results.mesh.mesh: MeshData`** — the reconstructed/placeholder mesh's flat geometry buffers
   (positions/normals/colors/indices/uvs/face_ids/vertex_ids/edge_*/paint_texture_base64), the
   framework's canonical mesh-editing interchange type, embedded directly. `RemodelMesh` (the
   wrapping struct) also carries `source: MeshSource`, `texture_asset_id: Option<String>`,
   `watertight: Option<WatertightReportSnapshot>` — genuinely NOT part of the mesh's own geometry
   (provenance / an asset reference / a derived QC summary), so these stay sibling fields, not folded
   into the composed child.
2. **`assets: BTreeMap<String, ImageAsset>`** — embedded `{mime, data: base64 String, width, height}`
   pixel bytes: video frames (`MediaStream.frames`, `image/jpeg`), baked mesh textures and
   DSM/DTM/ortho rasters (`image/png`). Verified this is the SAME shape `🖨️raster`'s own migration in
   this ticket already solved (`RasterImageAsset{mime,data}` → composed `s.stdio.semio.image` child) —
   reused that precedent directly rather than reinventing it.

**On the design map's literal "R:image" (link, not child) — investigated, and diverged from with a
documented reason.** `store::ArtifactLink` is real: "an INDEPENDENT-lifecycle reference to another
artifact... renders as a chip, never nests inline" (its own doc comment). `remodel`'s `assets` are not
that — they are embedded content OWNED by this exact document, addressed only by ids this same
document's own fields (`MediaStream.frames`, `RemodelMesh.texture_asset_id`, `GeoProducts.*_asset_id`)
ever reference. `🖨️raster`'s own migration hit the identical design-map wording ("raster→C:image
layers R:drawing") for the identical embedded-bytes shape and converted it to a composed CHILD, with
an in-place doc comment explaining why. Followed the same reasoning here, documented in
`🗿️artifacts/📸️remodel/🦀️component.rs`'s new `🧩️Composition` region doc comment.

**Pre-existing baseline break found and fixed first** (unrelated to composition, but blocking any
check): `🚪️io/🦀️component.rs` imported `semio::standards::v1::engine::geometry::{...}` — a stale path
left over from ticket #2553's engine-dissolution rename (the module is now `v1::subsets::any::schema::
geometry`). Fixed the import. Also fixed 3 `E0716` "temporary value dropped while borrowed" errors in
`🎛️apps/📸️remodel/🦀️component.rs`'s own tests (`ArtifactView::new(&doc, &HistoryView::empty())` — a
temporary borrowed across the statement boundary; W1's `HistoryView::empty()` signature evidently
changed to return an owned value rather than a `&'static` reference at some point). Both are trivial,
safe, unambiguous fixes squarely inside this plugin's own boundary — fixed outright per
`📌️important.md`'s own guidance rather than left as an open provenance question.

## What changed

### `results.mesh.mesh` → composed `s.stdio.semio/v1/mesh` child

`RemodelMesh.mesh: MeshData` → `RemodelMesh.mesh: store::ArtifactChild<SemioMeshSnapshot>` (bare,
never `Option` — the mesh is always present, matching `writer`'s "always-present slot" convention).

- **Real bidirectional converter, both directions** (`🚪️io/🦀️component.rs`, `🔖️Exporters` region):
  `mesh_data_to_semio_mesh` already existed (real, used by the PLY/LAS export hand-off, made
  `pub(crate)`); added its inverse `semio_mesh_to_mesh_data`. Round-trips positions/normals/uvs/
  colors/indices exactly; `face_ids`/`vertex_ids`/`edge_*`/`paint_texture_base64` are honestly NOT
  representable in `SemioMeshSnapshot`'s one-primitive gltf shape — documented in place, never
  fabricated. Covered by a new test, `semio_mesh_to_mesh_data_recovers_the_representable_buffers`.
- **Content-addressed handle minting** (`🗿️artifacts/📸️remodel/🦀️component.rs`, `🧩️Composition` →
  `🔖️MeshHandle`): `mesh_content_child_handle` hashes the REAL canonical conversion's pack bytes
  (`mesh_data_to_semio_mesh(mesh)` → `ArtifactPack::encode_pack`), never raw/incrementing.
- **Working-scene cache** (`REMODEL_MESH_SCRATCH`, `thread_local!`): caches the REAL, full-fidelity
  `MeshData` directly — deliberately not the lossy `SemioMeshSnapshot` projection — so the plugin's
  interactive mesh editing/undo path never loses `face_ids`/`vertex_ids`/`edge_*`/paint-texture data
  for the live document. `mint_and_stash_mesh`/`remodel_mesh_workspace` are the mint/read funnels every
  call site now goes through.
- **`RemodelMesh` simplified from hand-rolled to derived DSL codec.** The framework capability flagged
  in this ticket's own recipe (`impl<S> DslField for ArtifactChild<S>`, `🏪️store/🦀️component.rs:523`)
  is real — checked directly, confirmed present. `RemodelMesh` now just derives
  `#[derive(dsl::DslRecord)]` like any other record; the entire former `🔖️MeshBridge` region
  (`MeshDataTwin`, a private `PackedU32`, two `From` impls, and ~80 lines of hand-written
  `__dsl_spec`/`__dsl_to_record`/`__dsl_from_record`/`DslField` — all built solely to route `MeshData`
  around the orphan-rule gap) is GONE. Only `impl dsl::DslField for Box<RemodelMesh>` remains (needed
  because `ReplaceMeshResult.mesh: Box<RemodelMesh>`, and `Box<T>` needs its own impl regardless of how
  `T`'s own impl is produced) — now a pure one-line delegation to the derive-generated impl.
- Every call site that used to construct `RemodelMesh{ mesh: <MeshData>, .. }` now calls
  `mint_and_stash_mesh(<MeshData>)` for that field: `default_remodel_scene()`, `RemodelMesh::default()`
  (hand-written now, since `ArtifactChild<S>` has no `Default`), the reconstruction pipeline's real
  mesh-result emission (`🎛️apps/📸️remodel/🎮️commands/🚀️reconstruction`), the reset command's
  placeholder/empty results, and every test fixture across `🗿️artifacts/📸️remodel/🦀️component.rs`,
  `📸️snapshot/📝️text/🦀️component.rs`, `🧬️mutations/🦀️component.rs`, `💡️inferences/🦀️component.rs`,
  `💡️inferences/📦bounds/🦀️component.rs`.
- Every call site that used to read `results.mesh.mesh` as `MeshData` directly now reads through
  `remodel_mesh_workspace` (real content, `None` on a cold cache — documented staleness gap, matches
  every prior exemplar): `export_media("mesh:out", ...)`, the Model window's world-scene JSON
  (`🎭️modes/🧊️model/🪟️windows/🧊️model`), the Results panel's vertex/triangle-count labels
  (`📌️panels/🧵️results`), the `bounds` inference (`💡️inferences/📦bounds`), and the reset command's own
  vertex-count assertions.
- `ReplaceMeshResult`'s mutation payload shape is UNCHANGED (`Box<RemodelMesh>`) — the composition only
  changed `RemodelMesh.mesh`'s own field type, exactly the recipe's "unchanged public payload shapes"
  rule (the field IS the content being composed, so its type changing is the composition, not a
  violation of it). `🔺️diff`/`↩️inverse` for `replace-mesh-result` needed ZERO code changes: both
  already whole-value-replace the entire `ReconstructionResults`/`RemodelMesh`, so the new handle field
  flows through generically.

### `assets` → composed `s.stdio.semio/v1/image` children

`RemodelSnapshot.assets: BTreeMap<String, ImageAsset>` → `BTreeMap<String, store::ArtifactChild<
SemioImageSnapshot>>` (type alias `RemodelAssetChild`). `RemodelArtifact.assets` (the sibling
`🧬️schema/🦀️component.rs` facet) and `RemodelDiff.assets: Option<BTreeMap<...>>` mirrored identically.

- **Real bidirectional converter** (`🚪️io/🦀️component.rs`, new `🔖️SemioBridge` region): reuses
  `🖨️raster`'s exact `io_dispatch`-through-stdio's-real-PNG-codec pattern
  (`semio_image_from_png_bytes`/`png_bytes_from_semio_image`, dispatching `s.stdio.semio/v1/image` ↔
  `s.stdio.png` through the process-global composer registry, never a hand-rolled PNG reader/writer),
  one layer up for `remodel`'s own `ImageAsset{mime, data: base64 String, width, height}` shape
  (`semio_image_snapshot_from_image_asset`/`image_asset_from_semio_image_snapshot` — base64
  (de)coding is the only real difference from raster's `Vec<u8>`-shaped asset). Only `image/png`
  round-trips through this bridge today — matches raster's own scope exactly.
- **Divergence from raster's cache design, documented and deliberate**: raster's working-scene cache
  stores the DECODED `SemioImageSnapshot` and leaves the slot unpopulated on a decode failure (honest,
  since raster's assets are ALWAYS png — a failure there is anomalous input). `remodel`'s `assets`
  legitimately carry TWO real mimes in normal operation (`image/png` for textures/rasters, `image/jpeg`
  for `MediaStream.frames` — real call sites in `🎛️apps/📸️remodel/🎮️commands/📥️ingest`). Caching the
  REAL `ImageAsset` bytes directly (regardless of whether the png bridge could decode them) — rather
  than raster's decoded-content cache — keeps every `create-asset`/`delete-asset` inverse exact for
  BOTH mimes, not just png. Documented in place on `REMODEL_ASSET_SCRATCH`.
- **Content-addressed handle minting**: canonical (`image_content_child_handle`, hashes the composed
  child's own pack bytes) when the png bridge succeeds; raw-bytes (`image_asset_child_handle`, hashes
  `(mime,data)`) otherwise — same two-tier rationale as raster's own handle minting.
- `mint_and_stash_asset`/`remodel_asset` are the mint/read funnels. Every real call site that used to
  do `scene.assets.get(id)`/`.insert(id, ImageAsset{..})` now goes through these:
  `🚀️reconstruction`, `🎯️calibration`, `📥️ingest`, `🎭️modes/📷️capture/🪟️windows/🖼️frames`, the io
  layer's `remodel_png_export`, plus every test fixture.
- **`create-asset`/`delete-asset` mutation triads rewired, payload shape UNCHANGED**
  (`CreateAsset.asset: ImageAsset`, `DeleteAsset.key: String` — both still real event-log bytes, per
  the recipe's "mutation payloads carry content, only the document field became a handle" rule).
  `create-asset/🔺️diff` now calls `mint_and_stash_asset` before inserting into the document's map.
  Both inverses now reconstruct the OLD `ImageAsset` via `remodel_asset` (working-scene cache) instead
  of cloning it straight off the old `BTreeMap<String, ImageAsset>` — documented staleness gap
  (cold cache ⇒ honestly `Vec::new()`, matches `💠️lowpoly`'s own `CreateMesh`/`DeleteMesh` inverse
  precedent) rather than fabricated content.

### Fixture regeneration

`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` embedded the OLD `mesh { source=placeholder }` shape
with no `mesh=` handle line at all (the old codec tolerated an absent geometry block; the new
`mesh: RemodelMeshChild` field is bare/required). Regenerated via a temporary `#[cfg(test)]` in
`📸️snapshot/📝️text/🦀️component.rs` dumping real `print_dsl(&default_remodel_scene())` output
(`cargo test ... debug_fixture_regen... -- --nocapture`), spliced in as the new fixture file, then
verified it actually parses under the new codec with a second temporary test
(`<RemodelSnapshot as store::ArtifactDsl>::parse_dsl(text)`, real success), then removed both temporary
tests (`grep -rn debug_fixture_regen` / `debug_verify_example_dsl_semio` both return nothing now).

**Note on this fixture's test coverage**: found, while doing this, that
`📚️examples/🎬️demo/🧪️tests/🦀️test.rs` (`primary_asset_is_nonempty`, `inference_determinism_law`,
`inference_default_law` — tests that WOULD have caught this fixture going stale) is never mounted by
`📦️glue.rs` at all — a pre-existing dangling-mount gap, confirmed by grepping `📦️glue.rs` for
`🧪️tests` (zero hits) and cross-checking the file was never part of the 504-test run before or after
my edit. `📦️glue.rs` is W5-owned, out of this dispatch's scope — filed under `sharedFileRequests`.

### `3d.mesh` duplicate-kind check (per lowpoly's precedent, grepped first)

`remodel` declares only its own `3d.remodel` kind (`artifact_kind()`, `🗿️artifacts/📸️remodel/
🦀️component.rs`). `"3d.mesh"` appears twice, both as a REFERENCE to lowpoly's already-declared media
kind (the `mesh:out` media port's `kind_id: Some("3d.mesh".into())` and its own export payload's
`schema` tag) — never a second `ArtifactKindSpec` declaration. No duplicate to remove.

## Verification

`CARGO_TARGET_DIR=<ticket>/🎯️target` for every invocation.

- **Baseline** (`cargo check -p semio-s-plugin-remodel --all-targets`, before any edit): **RED** — 1
  `E0433` (stale `engine::geometry` import path) + 3 `E0716` (borrow-lifetime, `HistoryView::empty()`)
  — all four traced to genuine pre-existing breaks strictly inside `✏️s/🔌️plugins/📸️remodel/**`, fixed
  first (see `## Pre-flight`).
- After that fix, before any composition edit: **0 errors**, 779 warnings (recorded as the real
  starting baseline for the composition work itself).
- **After the full composition migration**, `cargo check -p semio-s-plugin-remodel --all-targets`
  (final re-run, immediately before writing this report): **0 errors**, 781 warnings (net +2 over the
  779 baseline: one is the SAME pre-existing `serde_to_json_value`/`json_value_to_serde` "never used"
  warning pair in an unrelated JSON-import file, now counted once more due to a lib/lib-test
  warning-grouping shift; one is a genuine new `semio_mesh_to_mesh_data is never used` — real in the
  non-test `lib` build specifically, because its only caller today is its own round-trip test; kept per
  the recipe's "real bidirectional converters" rule rather than deleted).
- `cargo nextest run -p semio-s-plugin-remodel --no-fail-fast`: **504 tests run, 502 passed, 2 failed.**
  Reproduced identical (same 2 failing test names) across **three** consecutive full runs (not flaky).

### The 2 failures — independently traced, neither caused by this migration

Both in code this migration never touched, both traced by real commit + `--date=iso`:

1. `apps::remodel::engine::images::tests::jpeg_decode_never_panics_on_truncated_input` —
   `⚙️engine/🖼️images/🦀️component.rs:975`, a self-contained JPEG codec test
   (`encode_jpeg`/`decode_jpeg`, this app's OWN jpeg engine — unrelated to the stdio jpg bridge and
   unrelated to `ImageAsset`/`RemodelAssetChild`). `git log -1 --date=iso -- .../⚙️engine/🖼️images/
   🦀️component.rs` → `9149914f9b…`, **2026-08-13 01:28:00**.
2. `apps::remodel::engine::reconstruction::tests::long::video_in_yields_watertight_mesh_out` —
   `⚙️engine/🏭️reconstruction/🦀️component.rs:1460`, `expected a non-empty mesh, got 0 triangles`. This
   test calls `engine.take_mesh()` directly on the RAW `ReconstructionEngine`'s internal output and
   asserts on it — entirely upstream of `RemodelMesh`/`ArtifactChild`/anything this migration touched.
   `git log -1 --date=iso -- .../⚙️engine/🏭️reconstruction/🦀️component.rs` → `dda7ceead1…`,
   **2026-08-13 18:52:17** — and `git show --stat` on that exact commit shows it also touched
   `⚙️engine/🥽️mesh`, `⚙️engine/🏃️motion`, `⚙️engine/📸️sfm`, `⚙️engine/🗺️geo`, `⚙️engine/🌟️feature`,
   `⚙️engine/📷️camera`, plus a `📓️wave-m3d-remodel-family-report.md` — this is DKM's
   `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` M3d math-extraction wave
   (flagged repo-wide in `📌️important.md` as active transient churn tonight across the whole
   photogrammetry engine stack), landing in the same auto-commit bundle as my own session's work, not
   my own edit.

Neither failure's stack trace, module path, or the commits' own diffs reference `RemodelMesh`,
`ArtifactChild`, `assets`, `mint_and_stash_mesh`/`_asset`, or `remodel_mesh_workspace`/`remodel_asset`.

## Deferred (honest accounting)

1. **JPEG image bridge.** `assets` compose for real (real handles, real working-scene cache, real
   inverses) for BOTH mimes today, but the CANONICAL semantic-dedup path
   (`semio_image_snapshot_from_image_asset`) only decodes `image/png` — matches `🖨️raster`'s own
   already-accepted scope limit for the identical shape. `stdio`'s `jpg` artifact already has real
   `decode_jpg`/`encode_jpg` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/…/🚪️io/🦀️component.rs:571,756`)
   — wiring an `s.stdio.semio/v1/image ↔ s.stdio.jpg` bridge the same way as the png one is a concrete,
   scoped follow-up, not attempted here under this dispatch's time budget.
2. **Sibling-language schema-facet mirrors** (TS/GraphQL/proto/JSON) for `RemodelSnapshot.assets`,
   `RemodelArtifact.assets`, `RemodelDiff.assets`, and `RemodelMesh.mesh` were NOT updated — still show
   the old inline shapes. Non-compiled documentation leaves, correctness-neutral to `cargo check`/tests
   (matches `puzzle`'s own identical deprioritization for the same reason under this ticket's time
   pressure).
3. **The dangling `📚️examples/🎬️demo/🧪️tests/🦀️test.rs` mount** (see `## What changed` → Fixture
   regeneration) — a pre-existing gap, `📦️glue.rs`-owned, filed below.

## sharedFileRequests

1. **File**: `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📦️glue.rs`.
   **Region**: the `examples` module mount (two places: `#[path="."] pub mod examples { pub mod demo
   { ... } }` inside the `standards::v1::subsets::any` tree, and the crate-root `pub mod examples {
   pub mod art_remodel_demo; pub mod app_remodel_demo_session; }`).
   **Reason**: `🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/
   🦀️test.rs` is a real, non-stub test file (`primary_asset_is_nonempty`, `inference_determinism_law`,
   `inference_default_law` — the last two would have caught this migration's fixture-format break
   automatically) that has never been mounted by any `#[path]` in `glue.rs`, so it has never compiled
   or run as part of `semio-s-plugin-remodel`'s test suite. Needs a `#[path = ".../🧪️tests/
   🦀️test.rs"] mod tests;` added under the `demo` module. No patch file written (a one-line addition,
   described precisely here); `glue.rs` is W5-owned so I did not make the edit myself.
2. No other shared-file changes needed — every other edit is inside `✏️s/🔌️plugins/📸️remodel/**`
   proper (not `glue.rs`/`index.ts`), this plugin's own exclusive fan-out boundary.

## Concurrent-churn observations

- Baseline stdio check hit transient `semio-s-plugin-stdio` compile errors on the very first attempt
  (`cannot find function mesh_to_dwg_drawing`), gone on the second attempt seconds later — classic DKM
  math-extraction churn per `📌️important.md`; retried in the foreground, cleared on its own, never
  touched.
- `git status`/`git diff --stat -- ✏️s/🔌️plugins/📸️remodel` were both empty at dispatch start (no live
  uncommitted edit from another session in this plugin's subtree).
- One auto-commit (`dda7ceead1`, 2026-08-13 18:52:17, `--date=iso`) landed mid-session, bundling
  unrelated concurrent work into `⚙️engine/**` files (see the 2 failures' trace above) alongside
  whatever of my own edits had accumulated by that point — did not collide with or require touching
  anything from that commit.
- No auto-commit advanced past `dda7ceead1` for the remainder of this session; all edits in this
  report's file list remain as normal uncommitted `git status` entries as of writing.

## Honest accounting

**Complete and verified**: both duplication shapes the design map names for `remodel`
(`results.mesh.mesh` → composed mesh child, `assets` → composed image children) are real —
content-addressed handles, real bidirectional converters (one pre-existing and reused, three newly
written), working-scene caches with documented staleness gaps matching this ticket's established
pattern, mutation triads rewired with unchanged public payload shapes, a stale fixture found and
regenerated for real (not hand-transcribed), a genuine pre-existing baseline break fixed first. 502/504
crate tests passing, reproduced stable across 3 runs; the 2 failures independently traced to code this
migration never touched, landing from a concurrent, already-flagged-in-`📌️important.md` math-extraction
wave.

**Deferred, with precise reasons**: JPEG image-bridge decoding (real cache/inverse-fidelity already
works for jpeg today via the full-`ImageAsset` cache design — only the CANONICAL content-hash path is
png-only, matching raster's own accepted scope), non-`📸️snapshot` cross-language schema mirrors, and a
pre-existing dangling example-test mount (filed as `sharedFileRequests`, `glue.rs`-owned).

Files touched (21, all inside `✏️s/🔌️plugins/📸️remodel/**`):
`🗿️artifacts/📸️remodel/🦀️component.rs`,
`🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`,
`🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`,
`🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`,
`🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`,
`🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs`,
`🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`,
`🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`,
`🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🔺️diff/🦀️component.rs`,
`🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/↩️inverse/🦀️component.rs`,
`🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/↩️inverse/🦀️component.rs`,
`🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`,
`🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️component.rs`,
`🎛️apps/📸️remodel/🦀️component.rs`,
`🎛️apps/📸️remodel/🎮️commands/🚀️reconstruction/🦀️component.rs`,
`🎛️apps/📸️remodel/🎮️commands/🧹️reset/🦀️component.rs`,
`🎛️apps/📸️remodel/🎮️commands/🎯️calibration/🦀️component.rs`,
`🎛️apps/📸️remodel/🎮️commands/📥️ingest/🦀️component.rs`,
`🎛️apps/📸️remodel/🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️component.rs`,
`🎛️apps/📸️remodel/🎭️modes/🧊️model/🪟️windows/🧊️model/🦀️component.rs`,
`🎛️apps/📸️remodel/📌️panels/🧵️results/🦀️component.rs`.

ucas-status: partial
