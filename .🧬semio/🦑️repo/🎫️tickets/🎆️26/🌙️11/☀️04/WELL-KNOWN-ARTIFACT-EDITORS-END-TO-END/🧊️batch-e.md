# 🧊️ Batch E (obj, stl, ply, gltf, las, step), T11

Status: implemented, NOT yet compiled or tested (cargo queue starved; see "Verification"). Update this file when the queued runs finish.

## Findings (Phase 1)
- Editors live in the per-subset `✏️editor` dirs (`ArtifactEditor` + `MeshWindowKit` main window + generic snapshot details window), published by the hub `🧩️extensions/🔺️mesh` (obj, stl, ply, gltf, las) and `🛠️cad` (step). Typed mutations exist for every kind and are reachable through the generic `setSnapshotValue`/insert/remove actions via each `🧭️edit-rules`.
- Gaps found: no `natural_file_codec` on any of them; all 3D windows drew a placeholder "box" per entity, not the document geometry; no group transform gesture.

## Changes
- Natural file codec (`natural_file_codec` + `encode_natural_file` + `decode_natural_file` + `import_media`) in all editors of obj, stl, ply, gltf, las, step (base + cc1..cc6). Real engine codecs reused: obj `engine::{encode,decode}_obj`, stl ascii write / auto (ascii+binary) read, ply `encode_ply`/`decode_ply` (keeps ascii/binary form), las `encode_las`/`decode_las`, gltf `serialize_gltf_document` + `parse_gltf_document`/`decode_glb` (decode sniffs `glTF` magic: `.gltf` and `.glb` both open; save writes `.gltf` JSON with inlined buffers because the framework allows one codec/extension per editor), step Part 21 `parse_part21`/`write_part21`.
  - format kinds: `s.stdio.obj@3.0`, `s.stdio.stl@ascii`, `s.stdio.ply@1.0`, `s.stdio.gltf@2.0`, `s.stdio.las@1.0`, `s.stdio.step@ap214`.
- Shared transform gesture `SetValue` at `/transform` with `{translate, rotate (deg, Euler XYZ), scale, pivot}`: `✏️editing/📐️transform` (cross-checked against three.js r182 via `🧪️tests/🧫️fixtures/📐️transform/🔣️.json`, generator `🧊️batch-e-scripts/transform-oracle.mjs`). Handlers via `snapshot_edit_special`: obj (vertices moved, normals rotated, `w` kept), stl (facets), ply (vertex x/y/z and nx/ny/nz keep float/double type), las (points + bounds, classification/intensity/scale kept), gltf (scene-root nodes: TRS or matrix kept; shear refused), step base (CARTESIAN_POINT moved, DIRECTION turned, units untouched).
- Real geometry in the main windows (editor and viewer) of obj, stl, ply, las through `semio_s_artifact_stdio_contract::mesh_scene` (new, `🪟️window-kit/🧊️mesh-scene`) fed by per-artifact `engine::scene_geometry`.
- Shared natural-file fixture rows (12 added) and count bump in `F/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle`.
- Tests: per-editor natural round trip + transform + third-party cross-check (stl_io, ply-rs, las crate, gltf container oracle, Part 21 oracle) via added dev-dependencies on the existing `*-test-oracle` crates; obj is checked against its own spec fixtures (no third-party obj oracle exists).

## Open
- gltf and step 3D windows still render the placeholder; gltf needs accessor/node-transform scene building, step needs brep/point tessellation.
- gltf save emits `.gltf` only (`.glb` import only) until the framework supports several natural codecs per editor.
- step cc1..cc6 have codecs but not the transform gesture (they route through their own `special`).
- `MeshWindowKit::editable_window_kind` `set-vertex` direct manipulation not wired (details pane + `/transform` are the edit path).
- Native receipt/catalog regeneration is not needed (native factories already declared).

## Transcribed evidence (T20, before the `🗑️generated` deletion)

Full text in `✅️validation.md` E5 and E6.
- `e-obj-editor-test.log`: `cargo test -p semio-s-artifact-stdio-obj editor:: …` from the repo root; exit status 1; `error: package ID specification `semio-s-artifact-stdio-obj` did not match any packages`. Failure (root selection). No pass/fail counts.
- `e-semio-s-artifact-stdio-obj-editor::obj.log`: `cargo test -p semio-s-artifact-stdio-obj --lib editor::obj …` from the repo root; exit status 1; same `did not match` error. Failure (root selection).
- `e-transform-test.log`: `cargo test -p semio-s-artifact-stdio-contract-editing transform …`; exit status 1; `did not match any packages`. Failure (root selection). The transform tests have not run.
- `e-semio-s-artifact-stdio-contract-editing-transform.log`: capacity-wait lines only; no result.
- Contract (offline) for gltf, las, obj, ply, stl, step: passing in E1 (`✅️validation.md`).

## Update (later in the session)
- stl natural codec is now truthful: writes ASCII (`binary: false`, shared fixture row updated); reading accepts ASCII and binary STL (`decode_stl_auto`). The stl editor window doc no longer claims `set-vertex`.
- stl and ply demo assets `🧊️.stl` / `🧊️.ply` replaced with a deterministic tetrahedron (`🧊️batch-e-scripts/make-tetrahedron-demo-assets.py`); independent stdlib check `check-tetrahedron-demo-assets.py` passes (closed manifold, outward normals, volume 1/6). No snapshot embeds these files (packs embed the dsl demo); the stl `codec_retention_law` test now exercises the real file instead of falling back.
- Row-content edits: ply vertex coordinates and stl vertex coordinates already resolve through the Details pointers (`/elements/<e>/rows/<r>/values/<p>`, `/triangles/<i>/vertices/<v>/<axis>`); tests added for both.
- Keyboard: `🧰️framework/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🎬️canvas/🟦️.tsx` now makes the world canvas focusable, binds three's key controls only to the focused canvas (arrows pan, Ctrl+arrows orbit) and adds `+`/`-`/PageUp/PageDown zoom (perspective dolly, orthographic zoom). Not browser-verified (needs T06).
- GL release: verified in `node_modules/@react-three/fiber` 9.8.2 (`dist/events-0dab3c53.cjs.dev.js` `releaseRenderer`/`disposeRenderer`): unmount disposes the renderer and calls `forceContextLoss()`, for owned and caller-provided renderers; no extra release code needed. A repeated open/close browser test remains for T06.
- Cargo: each artifact is its own workspace and lockfile; run `repo cargo` from the artifact directory (`🧊️batch-e-scripts/run-batch-e-tests.sh` does this). Adding the existing `*-test-oracle` crates as dev-dependencies would change the per-artifact Cargo.lock (`--locked --offline`), so the in-test cross-checks are independent first-party byte/text readers written from the format specs; real third-party oracles (stl_io, ply-rs, las, ruststep) need a lock sync by T05.
## Still open
gltf GLB save, step cc1..cc6 product/shape rules and transform, ply grammar scaffold, step/gltf real 3D rendering, selection by keyboard, third-party oracles in editor unit tests.

## Update 3 (dependency freeze, decisions applied)
- No lockfile was touched (the `repo dependencies lock` authorization was revoked and never used). The in-test cross-checks are now language-agnostic fixtures written by independent stdlib code: `🧊️batch-e-scripts/reference-parsers.py` parses `sample.obj/.stl/.ply/.step/.gltf`, writes `sample.las` and `sample.glb` with `struct`, and emits `expected.json` next to them; each editor test compares the Rust decode to that file (obj, stl, ply, las, gltf and glb, step). Regenerate with `python3 -I reference-parsers.py <artifacts-root>`. `numpy` is the only third-party package in `.venv` (no trimesh, laspy, numpy-stl, pygltflib).
- Oracle path dev-dependencies for T05 (not applied; per-artifact workspaces, `*-test-oracle` crates are excluded from their workspace, so each lock needs them plus their registry crates, none of which is present in the current lock or in a local registry cache). Crate to add as `[dev-dependencies]` path dependency of the editor test crate, brings in:
  - `🗿️artifacts/🔺️stl`: `semio-s-artifact-stdio-stl` + `semio-s-artifact-stdio-stl-test-oracle` (`../../🔮️oracles/📦️packages/🦀️rust`) -> `stl_io 0.8`, `semio-repo-test-host`
  - `🗿️artifacts/🧱️ply`: `semio-s-artifact-stdio-ply` + `…-ply-test-oracle` -> `ply-rs 0.1`
  - `🗿️artifacts/☁️las`: `semio-s-artifact-stdio-las` + `…-las-test-oracle` -> `las 0.11`
  - `🗿️artifacts/🧊️gltf`: `semio-s-artifact-stdio-gltf-editor` + `…-gltf-test-oracle` (eight `../` up) -> `json 0.12`
  - `🗿️artifacts/📐️step`: `semio-s-artifact-stdio-step-base-editor` + `…-step-test-oracle` (eight `../` up) -> `ruststep 0.4` (locked) + `semio-s-plugin-stdio-part21-test-oracle`
  - obj has no third-party oracle crate; the fixture check is the Python reference parser.
- Real geometry now also for gltf (`io::scene_geometry`: accessor triangles placed by node world matrices, test `the_scene_places_accessor_triangles_by_the_node_transform`) and for step (`StepSnapshot::scene_points`: cartesian points as markers; edges are not drawn) in base and cc1..cc6 windows (editor and viewer).
- step cc1..cc6 Details rules (virtual pointers handled in each cc `special`, helpers in `🪜️ladder`): `/product`, `/product/{productName,formationId,definitionId}` -> `set-product-identity` (remove `/product` clears the chain); `/shapeRepresentations/<id>[/…]` set -> `set-shape-representation` (cc2..cc6; cc1 refuses), remove -> `remove-shape-representation` (cc1), `demote-shape-representation` (cc2..cc5), `set-shape-representation` with `None` (cc6); `/transform` -> `restore-entities` rows. Ladder tests added; cc crates and editors are in the queued run script.
- GLB / binary STL / binary PLY encode paths are plain functions with round-trip tests, ready for T01: `io::export::encode_glb` (+ test `the_glb_encode_path_round_trips_through_the_natural_decoder`), `engine::encode_stl_binary` (+ test), `engine::encode_ply_with_format` (+ test; the ply snapshot already keeps `format` and `encode_natural_file` writes the opened form).
- ply grammar scaffold: `🔤️.ebnf` and `🅰️.g4` of the ply text mutations replaced with the real one-line op grammar (the `📖️.grammar.semio` was already real). 
- Fixture count: untouched; T06's `BATCH_E_NATURAL_FILE_APPS` set covers the 12 rows.
- Still open: Details-pane keyboard selection of objects/vertices; gltf/step edge rendering; running everything (queue starved).
