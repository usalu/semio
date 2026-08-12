# W8 — End-to-End Acceptance Scenarios (a)/(b)/(c)

Fresh-eyes confirm/build pass for the master plan's e2e acceptance scenarios (a), (b), (c).
Scenario (d) was independently verified in W7 (`w7fix-report.md`/`w7fix-verify-report.md`), out of
this task's scope. Scenarios (e)/(f) are the orchestrator's final-gate items, also out of scope.

## Scenario (a): cad → semio/brep → .step → reimport → semio/brep → semio/mesh → .gltf

**Status: extended and passing.** A real test already existed
(`export_solids_as_step_round_trips_through_real_semio_brep_bridge`,
`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`) covering cad → real
box solid → `semio/brep` → `.step` text → reimport → `semio/brep`, asserting solid/face/vertex
COUNT geometry-equivalence. It stopped there — the `semio/mesh`/`.gltf` hops were not chained.

Extended the SAME test (region `🔖️SemioBrepBridge`, no new test file) to chain the remaining two
hops and assert a real GEOMETRIC (not just count) invariant survives all the way to the final
`.gltf` bytes:
1. Computes the reimported `SemioBrepSnapshot`'s own vertex bounding box.
2. Tessellates the same live kernel solid the reimported brep was just proven topologically
   equivalent to, into a real `SemioMeshSnapshot` (via cad's existing
   `semio_mesh_snapshot_from_solids`), and asserts its mesh-position bounding box matches the
   brep's vertex bounding box (`< 1e-6`).
3. Serializes that mesh through the REAL `SemioMeshToGltf` codec
   (`✏️s/🔌️plugins/🗄️stdio/…/✳️mesh/🚪️io/📤️export/…/gltf/…`), then decodes the exported gltf
   buffer's own raw little-endian POSITION bytes (via the accessor/bufferView it just built) back
   into a bounding box, and asserts THAT still matches the brep's bounding box (`< 1e-4`, wider
   epsilon for the f32 round trip).

Documented inline (matching the file's existing pattern) why the mesh hop tessellates the live
kernel solid rather than re-importing the bridge's own STEP text through `kernel.import_step`: a
pre-existing, out-of-plugin-scope framework AP203-reader gap (documented in the same file, one test
above) rejects `SemioBrepToStep`'s own spec-valid `$` `ref_direction` output — every assertion is
still anchored to the REIMPORTED brep's own vertex data, not the original box dimensions.

Files touched: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
(1 new `#[cfg(test)]` import + extended docstring + extended test body, `🔖️SemioBridgeImports` /
`🔖️SemioBrepBridge` regions).

```
cargo check -p semio-s-plugin-cad --lib --tests   → 0 errors (7 pre-existing warnings, untouched)
cargo test -p semio-s-plugin-cad --lib "artifacts::cad::standards::v1::engine::"
  → 32 passed; 0 failed; 0 ignored
  test ...::export_solids_as_step_round_trips_through_real_semio_brep_bridge ... ok
```
Raw logs: `w8-scenarios-cad-check.txt`, `w8-scenarios-cad-test.txt`.

## Scenario (b): draw → semio/drawing → svg AND dwg

**Status: SVG direction already real and passing; DWG direction is an honest, correctly-documented
capability gap — not a missing test.**

SVG direction: `draw_document_to_svg_bridges_shape_text_image_and_gradient_nodes_through_semio_drawing`
(`✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`, region `🧪️Tests`)
builds a `DrawSnapshot` with shape/gradient/text/image layers, exports through the real
`SemioDrawingSnapshot → io_dispatch → svg` bridge, then RE-PARSES the resulting SVG text through
stdio's own `parse_svg_xml`/`svg_element_from_xml_node` (not substring matching) and asserts typed
structure: filled rect path, dropped (not fabricated) gradient fill, escaped text content, and a
base64 `data:image/png` image href. This is real and already passing — no change needed.

DWG direction: independently confirmed by directly listing stdio's
`🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/{📤️export,📥️import}/…/🗿️artifacts/` tree — only
`svg`, `pdf`, `dxf` subdirectories exist, no `dwg`. `s.stdio.semio/v1/drawing` bridges to
svg/dxf/pdf only, per the master plan's own lattice; DWG only bridges from the separate
`s.stdio.semio/v1/cad` hub (standard `ac1024`), an architecturally distinct dialect from draw's own
`s.stdio.dwg@ac1018`. Draw's DWG import leaf is an honest degenerate stub (documented inline as a
`stdio_gap`, matching its svg/pdf/png sibling leaves' shape) — this is a real, correctly-documented
capability gap, confirmed genuine by this session's own independent grep, not something to force a
test for.

No files touched (verification only).

```
cargo check -p semio-s-plugin-draw --lib --tests   → 0 errors (4 pre-existing warnings, untouched)
cargo test -p semio-s-plugin-draw --lib "artifacts::draw::standards::v1::engine::"
  → 34 passed; 0 failed; 0 ignored
  test ...::draw_document_to_svg_bridges_shape_text_image_and_gradient_nodes_through_semio_drawing ... ok
```
Raw logs: `w8-scenarios-draw-check.txt`, `w8-scenarios-draw-test.txt`.

## Scenario (c): animate → semio/video → real mp4

**Status: extended and passing.** A real test already existed
(`writer_buffers_frame_and_finalizes_a_real_decodable_mp4`,
`✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🎥️video/🦀️component.rs`,
region `writer::tests`) writing 2 real frames through the writer, finalizing a real mp4 partial, and
asserting `decode_mp4` succeeds with the right track/sample counts and byte-exact frame data — this
already IS the box-walk proof (ISO-BMFF decode is a nested-box-tree walk; a malformed tree is a
hard `Err`, never a silent partial result).

Extended the same test with explicit track/duration invariant assertions the task's exit checklist
calls for by name, so they're asserted, not just implied:
- `snapshot.ftyp.major_brand` non-empty (explicit ftyp-box-survived-the-walk assertion).
- `track.timescale > 0` (real positive timescale).
- Total track duration in ticks (`sum(sample.duration)`) == 2 (2 frames × 1 tick/frame), and
  ticks/timescale > 0 (real positive wall-clock duration derived from the container's own fields).
- Byte-exact frame payload for both samples (already present, kept).

Files touched:
`✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🎥️video/🦀️component.rs`
(extended docstring + test body, `writer::tests` region, no new test file).

```
cargo check -p semio-s-plugin-animate --lib --tests   → 0 errors (5 pre-existing warnings, untouched)
cargo test -p semio-s-plugin-animate --lib "artifacts::present::standards::v1::engine::video::"
  → 12 passed; 0 failed; 0 ignored
  test ...::video::writer::tests::writer_buffers_frame_and_finalizes_a_real_decodable_mp4 ... ok
```
Raw logs: `w8-scenarios-animate-check.txt`, `w8-scenarios-animate-test.txt`.

## Concurrent-churn note

`git status` at the start of this session showed large, unrelated, live in-progress edits across
`draw`, `gis`, `procedural`, `writer`, and `cad`'s own tree (mutation-directory restructuring,
consistent with the pattern W5a/W5b closers already flagged as "another session's in-progress
refactor"). None of the 3 files this session edited overlapped with that churn's file set at any
point checked (`git status` re-verified after every edit — both edited files retained exactly this
session's changes, no clobbering observed). All `cargo check`/`cargo test` runs above are fresh,
this-session, scoped to the 3 owning crates (cad/draw/animate) — not the full workspace — to avoid
attributing unrelated concurrent foreign breakage to this task.

## Summary

| # | Scenario | Outcome |
|---|---|---|
| a | cad → brep → step → reimport → brep → mesh → gltf | Extended existing test, chained all hops, bounding-box geometry-equivalence proven end to end through real `.gltf` bytes. 32/32 cad engine tests pass. |
| b | draw → drawing → svg AND dwg | SVG: already real, re-parses, passing as-is. DWG: confirmed honest capability gap (no `semio/drawing↔dwg` bridge in stdio), correctly documented in-code, not a missing test. 34/34 draw engine tests pass. |
| c | animate → video → mp4 | Extended existing test with explicit box-walk/track/duration invariant assertions. 12/12 animate video engine tests pass. |
