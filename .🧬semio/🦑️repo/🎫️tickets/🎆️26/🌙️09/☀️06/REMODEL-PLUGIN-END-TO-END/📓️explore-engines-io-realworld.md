# Explore: remodel engine stack, IO registration, real-world fixtures

Scope: `✏️s/🔌️plugins/📸️remodel`. Read-only audit, no build/test run (host swap/load
constraint) — findings on runtime behavior are code-inspection-derived and marked
**[unverified by execution]** where relevant. All paths relative to
`/Users/ueli/Documents/semio`.

## 1. Engine topic inventory (`…/✳️any/✏️editor/⚙️engine/`)

| file | LOC | pub fn | status |
|---|---:|---:|---|
| `⚙️engine/🦀️.rs` (top-level params/wiring) | 465 | 14 | real |
| `🌟️feature/🦀️.rs` | 1,407 | 16 | real — Shi-Tomasi, KLT, forward/backward prune, full AKAZE nonlinear-diffusion scale space (`build_akaze_scale_space` etc.) |
| `🌫️dense/🦀️.rs` | 2,508 | 37 | real — PatchMatch MVS, plane-sweep, left/right check, TSDF volume + integration, point-cloud filters, PMF ground classification, region-growing planes, M3C2 |
| `🎥️video/🦀️.rs` | 4,128 | 12 | real — largest by LOC; bounded/incremental mp4 frame decode plumbing |
| `🏃️motion/🦀️.rs` | 1,441 | 24 | real, with one **documented, deliberate substitution** — see §1.1 |
| `🏭️reconstruction/🦀️.rs` | 2,408 | 23 | real |
| `📷️camera/🦀️.rs` | 1,493 | 14 | real |
| `📸️sfm/🦀️.rs` | 3,849 | 55 | real — fundamental/homography/essential (incl. 5-point), P3P (Grunert), EPnP, PnP-RANSAC, LM pose/point refinement, rotation/translation averaging, loop detection, pose-graph optimize, local/global bundle adjustment as a resumable state machine (`register_next`, `advance_bundle`, …) |
| `🖼️images/🦀️.rs` | 1,475 | 38 | real — PNG decode/encode delegates to stdio's real codec; bounded chunked decoder |
| `🗺️geo/🦀️.rs` | 1,217 | 36 | real |
| `🥽️mesh/🦀️.rs` | 5,333 | 44 | real (largest file in the crate) |
| **engine total** | **~25,724** | **313** | |
| `🧬️schema/➕️algebra-internals/🦀️.rs` | 2,092 | — | real (LLL lattice reduction on `number::VecG`/`Rational`) |
| `🧬️schema/🎯️optimize-internals/🦀️.rs` | 1,241 | — | real (Levenberg-Marquardt, numeric Jacobian) |
| `🧬️schema/🔷️lie-internals/🦀️.rs` | 806 | — | real (SE3/SO3) |
| `🧬️schema/📶️signal-internals/🦀️.rs` | 586 | — | real |
| `🧬️schema/🗺️spatial-internals/🦀️.rs` | 755 | — | real (KdTree) |
| **internals total** | **~5,480** | | |

**Bottom line: this is one of the most substantively implemented plugins in the repo.**
Every named pipeline stage — import → features → matching → SfM (2-view init, PnP
registration, bundle adjustment, loop closure, pose-graph optimize) → dense (PatchMatch
MVS, TSDF fusion) → mesh → geo/motion — has real, non-trivial Rust algorithms, not
default-returning stubs. `todo!()`/`unimplemented!()` macros: **zero occurrences** in the
whole crate (grep confirmed).

### 1.1 The one confirmed stub, self-documented

`⚙️engine/🏃️motion/🦀️.rs:163-168` (doc comment on `MultiObjectTracker`):
> "`crate::graph_matching` (this crate's originally-planned max-weight-matching
> dependency) is an unimplemented one-line stub with no public API — confirmed by direct
> inspection and reflected in this crate's `Cargo.toml`, which no longer depends on it —
> so `MultiObjectTracker::update` solves the gated assignment with a real, working,
> greedy ascending-cost matcher instead."

I.e. the crate proactively removed a dead dependency and replaced it with real (if
simpler) code — the opposite of a live gap. `MultiObjectTracker::update` (line ~207) is
implemented and real.

### 1.2 Other `todo!/TODO/stub/placeholder/not yet` grep hits — all benign

- `⚙️engine/🦀️.rs:128`, `🎮️commands/🏗️run-reconstruction/🦀️.rs:781`: "Ground control
  points are set but checkpoint RMSE is not yet computed" — a documented, narrow
  reporting gap (QC checkpoint-RMSE metric), not a pipeline stage.
- `🎮️commands/🔭️calibrate-cameras/🦀️.rs:14`: "Auto-derives **placeholder** pinhole
  intrinsics" — this is the intentional initial-guess heuristic (`fx=fy=max(w,h)`)
  every photogrammetry pipeline needs before refinement; not a stub of calibration
  itself.
- `🎮️commands/♻️reset-placeholder-mesh/🦀️.rs`: a real, intentional named UI command
  ("Reset Placeholder Mesh") that restores the seeded placeholder box — not accidental
  dead code.
- `🚪️io/…/🔤️txt/…`: **honest typed stub**, see §3.

## 2. `run-reconstruction` / `run-stage` / `retry-stage`

- `▶️run-stage/🦀️.rs` (21 lines) and (by symmetry) `🔁️retry-stage/🦀️.rs` are thin
  wrappers calling `run_reconstruction::begin_stage_reconstruction`.
- `🏗️run-reconstruction/🦀️.rs` (1,524 lines) is a real **bounded, resumable, chunked
  continuation** — `RECONSTRUCTION_STEP_BUDGET`, `MAX_RECONSTRUCTION_TICKS`,
  chunked mesh/asset streaming (`MeshPreparation::next_chunk`, `MESH_CHUNK_BYTES`),
  matches CLAUDE.md's progress/cancellation rule.

### 2.1 Empty-frames short-circuit — confirmed, high confidence

`🏗️run-reconstruction/🦀️.rs:648-650` (`begin_requested_reconstruction`):
```rust
let scene = doc.snapshot;
if scene.streams.iter().all(|stream| stream.frames.is_empty()) {
    return Ok(Emit::default());
}
```
And `default_remodeling_scene()` (`🗿️artifacts/📸️remodeling/🦀️.rs:1643-1656`) seeds
`streams: Vec::new()`. Both shipped examples confirm this:
- `📚️examples/🎬️demo-session` (editor-level): its `🖼️assets/🎮️.cmd.semio` primary
  text is just `semio remodeling.remodeling.cmd v1\naction=demo` — no import commands,
  and its own test only asserts `text.len() > 8`.
- `📚️examples/🎬️demo` (subset-level): its `🖼️assets/🗣️.dsl.semio` is the serialized
  default scene — `streams […] {}` (empty table), `cameras […] {}` (empty),
  `job { stage=idle }`.

**Conclusion: running `run-reconstruction`/`run-stage`/`retry-stage` against any shipped
boot document or example is a silent no-op** — `Emit::default()`, no mutation, no error,
no job-status change. It would only produce a sparse cloud/mesh after a prior
`import-video`/`import-frames`/`import-frame-payload` command supplies real frame bytes,
which no committed fixture currently does (§4). `[unverified by execution — inferred
from the guard clause and both examples' literal content, not from running it]`

### 2.2 Mutations emitted once frames exist

Traced through the file: `replace_job` (per-tick progress), then on completion of the
terminal phase machine (`TerminalPhase::{Sparse,Quality,Mesh,Geo,Dsm,Dtm,Commit}`),
`commit_reconstruction(CommitReconstruction { job, sparse, trajectory, mesh, geo, qc,
assets })` (line 891) plus `create_asset` mutations for chunked raster/mesh content
streamed in along the way (`RasterAssetPreparation`, `MeshPreparation`). No
`replace-sparse`/`replace-dense` mutation is emitted directly by this file — those exist
as their own mutation modules (`🧬️mutations/⭐replace-sparse`,
`🧬️mutations/☁️replace-dense`) but `run-reconstruction`'s own terminal path folds
sparse/dense results into the single `commit-reconstruction` mutation instead.

## 3. IO — registered but 7 of 9 format bridges are broken

Per the BLOCK ticket's healthy-reference pattern
(`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️w3-io.md`), the
"real" channel is `IoDeclaration`/`SubsetDeclaration.io` (typed `Serializer`/
`Deserializer` impls, `serializer_entry`/`deserializer_entry`, registered via
`commit_artifact_declarations`).

**remodel is not on that channel.** Grep for `IoDeclaration|io: io::io()|SubsetDeclaration`
in the crate: zero hits. Instead, `🗿️artifacts/📸️remodeling/🦀️.rs:97` calls
`.composers(…::io_registry::entries())` — `ArtifactDeclarationBuilder::composers()`
(`🧰️framework/🔨️modules/🚪️io/🦀️.rs:3184`), a **different, older mechanism**
(`ComposerEntry`, defined next to but distinct from the typed `io_mechanism` at line
2364 of the same file) keyed by `IoKey`/`IO_REGISTRY`. Unlike block's *pre-fix* state
(dead code, zero callers), remodel's `.composers()` call is live/reachable — it is not
the "unregistered" failure mode the BLOCK ticket found. But it is not the io_mechanism
channel either, and its entries are functionally broken:

`🚪️io/🦀️.rs:409-536` (`io_registry::entries()`) registers 8 `ComposerEntry` export rows
— las, ply, png, json, dwg, stl, gltf, obj — each calling
`compose_export_<fmt>` → `…::io::export::serializers::artifacts::<fmt>::…::serialize_bytes`.

**7 of those 8 leaf serializers (las/ply/png/dwg/stl/gltf/obj) share one broken pattern**
(e.g. `📤️export/…/🖊️dwg/…/🦀️.rs`):
```rust
pub async fn serialize(snapshot: &RemodelingSnapshot) -> Result<DwgSnapshot, store::TextError> {
    let bytes = <RemodelingSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <DwgSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(...)
}
```
`ArtifactPack` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9332`) is a
**per-type, `dsl_derive`-generated, field-id-keyed** binary codec; its own doc states the
law `P::decode_pack(&p.encode_pack())` recovers an equal `p` — i.e. round-trip is only
guaranteed *same-type*. Encoding a `RemodelingSnapshot` (scene: streams/calibration/
params/gcps/job/results) and decoding those bytes as a `DwgSnapshot`/`PngSnapshot`/
`LasSnapshot`/`StlSnapshot`/`GltfSnapshot`/`ObjSnapshot`/`PlySnapshot` (all structurally
unrelated schemas) is, by this design, **deterministically a `PackError`** every time
it's actually invoked. The import side mirrors it (e.g. `📥️import/…/📷️png/…/🦀️.rs`):
`decode_pack` first, falling back to `parse_dsl(String::from_utf8_lossy(bytes))` — a real
PNG/LAS/OBJ file's binary bytes will not parse as remodeling DSL text either. **This is
the identical 7-file copy-paste shape**, almost certainly generator/codemod output, not
7 independently-reasoned implementations. `[high confidence from the trait's own
round-trip law + identical shape across 7 files; not confirmed by running cargo test]`

- Only **`json`** is genuinely real: `to_value()` → `pack::json::from_dsl_value` →
  stdio's `write_json_pretty` (`📤️export/…/🔣️json/…/🦀️.rs`).
- **`txt`** is an honest typed stub: `Err("txt export/import not yet implemented")`,
  with a doc comment explaining the leaf was a stray copy-paste of stdio's own
  json↔txt bridge, deliberately left as a stub pending real work
  (`📤️export/…/🔤️txt/…/🦀️.rs`, `📥️import/…/🔤️txt/…/🦀️.rs`).
- **No test exercises the broken path.** `🚪️io/🦀️.rs`'s own `exporters_tests` module
  (lines 195-279) tests `mesh_to_ply_bytes`, `mesh_to_las_bytes`, and
  `remodeling_png_export` — three **separate, genuinely real** bridge functions (mesh
  buffer → stdio's `SemioMeshToPly`/`SemioMeshToLas`/PNG codec) defined earlier in the
  same file, which the registered `ComposerEntry.compose_export_*` functions do **not**
  call. The broken whole-snapshot cross-pack-cast path is untested.

Media: images/video go through stdio's own `png`/`mp4` codecs (`semio-s-plugin-stdio`
dependency in Cargo.toml); no direct dependency on `semio-framework-pixels` was found in
remodel's own Cargo.toml (only `base64_codec` and `semio-framework-pixels` are both
listed — pixels is used, at `📦️packages/🦀️rust/Cargo.toml:51`).

## 4. Real-world fixtures for remodel testing

Repo-wide media search (`find … -size +5k`, excluding node_modules/target) found
**no committed multi-view photo set or camera-pose-labeled dataset anywhere in the
repo.** Closest candidates:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/…/🧫️fixtures/🎬️.mp4` — a real 2.7 MB
  ISOBMFF video fixture, but it is stdio's own codec-conformance fixture (single file,
  no known camera path); usable as a real-decode smoke test for remodel's
  `import-video` frame extraction, not as an SfM ground-truth source.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/…/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg`
  — one real 483 KB JPEG, but it is a single architectural floor-plan drawing photo, not
  multi-view photography.
- `./temp/bauen-mit-bestand.mp4` (16 MB, real ISO-MP4) exists on disk but is
  **gitignored** (`.gitignore:6:temp`) and untracked — not a usable committed fixture.
- `♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/{◀️…,▶️…}hexagonal-cut-concrete-forest-{left,right}.glb`
  and `🧱️block`/stdio's CSG mesh fixtures (`.stl`/`.obj`/`.ply`/`.gltf` under
  `🗿️artifacts/🧿️semio/…/🔺️mesh/🧫️fixtures/`) are real meshes with known geometry — in
  principle a mesh-with-known-camera-poses synthetic dataset (render N views of one of
  these + inject known poses) would make an excellent ground-truth SfM fixture, but
  **no such pipeline exists today**; remodel's SfM tests never touch these files.
- `🎥️shooting` and `💠️lowpoly` plugins exist by name (rendering / low-poly mesh
  authoring) but whether `shooting` can rasterize a scene to RGBA frames usable as
  remodel input was **not verified** — out of budget for this pass, flagged for
  follow-up.

**remodel already has its own synthetic ground-truth generator**, and it is used, not
just present: `📸️sfm/🦀️.rs:185` `synthetic_scene(seed, camera_count, point_count,
planar)`, `:216` `project_observations(scene, pixel_noise_std, outlier_fraction, seed)`,
`:244` `render_textured_scene(scene)`. Grep for
`synthetic|render_textured|project_observations` across the crate: 30 hits, concentrated
in `📸️sfm/🦀️.rs`, `🏭️reconstruction/🦀️.rs`, `🗺️geo/🦀️.rs` and their test modules —
i.e. the crate's SfM/reconstruction/geo tests already validate against procedurally
generated cameras+points+pixel-noise+outliers, not against any real external asset.

## 5. Third-party oracle crates available in `Cargo.lock`

Already used by remodel as a dev-only oracle (per its own Cargo.toml comment, "Test
oracle only (never `[dependencies]`)"):
- `png = "0.17.16"` — remodel's own dev-dependency, used in `🖼️images/🦀️.rs`'s test
  module (`png::Decoder`, line ~956) to cross-check the crate's own PNG codec output.

Present in the workspace `Cargo.lock` (available to pin as a new dev-dependency, not
currently used by remodel):
- `nalgebra 0.33.3` — no direct `[dependencies]`/`[dev-dependencies]` consumer found by
  grep across `🧰️framework`/`✏️s`; present only transitively. Candidate oracle for
  camera math (SE3/rotation/essential-matrix cross-checks) if pinned directly.
- `image 0.25.10` — a real **runtime** dependency (`[dependencies]`) in 4 framework
  crates (`📏️intrinsic-size`, `🗺️surface`, `♾️infinite`, the wgpu renderer target) and
  correctly `optional = true` in stdio's own `🧪️oracle` package — this is repo-wide
  precedent, not a remodel-specific violation. remodel itself does not depend on it.
- `png 0.18.1` (a second, newer version also in the lockfile for a different consumer)
- `gltf 1.4.1` — candidate oracle for validating remodel's own real gltf bytes are
  spec-conformant (once the export leaf is fixed, see §3).
- `approx 0.5.1`, `rand 0.8.6` / `0.9.5` / `0.10.2` — general float-tolerance and
  independent-RNG oracle helpers.
- No `ply-rs`/`tobj`/`argmin`/`levenberg-marquardt`/`ndarray`/`kiddo`/`kdtree` crate is
  present anywhere in `Cargo.lock` — an independent PLY/OBJ-parsing or LM/KdTree oracle
  would require adding a new dependency, not just pinning an already-locked one.

## Prioritized gaps

1. **[HIGH, high-confidence, unverified by execution]** 7 of 9 remodel IO format
   bridges (las/ply/png/dwg/stl/gltf/obj, both directions) are structurally guaranteed
   to fail at runtime (`ArtifactPack` cross-type decode) despite compiling and looking
   implemented — untested, identical-shape bug across 7 files. `txt` is at least an
   honest stub; only `json` works.
2. **[HIGH, code-confirmed]** Every shipped example/boot document has zero frames, so
   `run-reconstruction`/`run-stage` silently no-op on all of them — there is no
   committed fixture anywhere that exercises the pipeline end to end.
3. **[MEDIUM]** remodel's IO registration uses the older `.composers()`/`ComposerEntry`
   mechanism, not the `IoDeclaration`/`SubsetDeclaration.io` channel the BLOCK ticket
   established as the repo's healthy reference — inconsistent with `🗒️note`/`🖍️draw`/etc.
4. **[LOW]** GCP checkpoint RMSE reporting is a documented, narrow gap (not a pipeline
   stage).
5. **[Not a gap]** The engine math itself (features/SfM/dense/mesh/motion/geo) is real,
   large, and substantially complete — the crate's own synthetic-scene generator already
   gives it ground-truth SfM test coverage that most other engine topics in the repo
   lack.
