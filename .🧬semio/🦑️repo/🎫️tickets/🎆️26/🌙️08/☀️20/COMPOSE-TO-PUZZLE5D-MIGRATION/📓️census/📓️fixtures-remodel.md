# 📸️ remodel — handcrafted mutation fixtures (34/34)

Tree: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Wiring: `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📦️glue.rs` (34 `#[cfg(test)] mod tests_*` entries, one
inserted directly after each leaf's `pub mod inverse;`).

`fixtures lint --by-tree`: the remodel tree no longer appears in the uncovered list (0/34) and raises
no error findings. `cargo` was NOT run — a peer's de-async sweep has the workspace broken — so **no
test is claimed to pass**; validation was structural (see §5).

## 1. The base snapshot

One `⬅️before` scene is shared by all 34 cases (each case carries its own copy). It was designed so
that every leaf has a real target AND so that every push-to-end inverse restores list order exactly.

| Field | Content | Why |
| --- | --- | --- |
| `schema` / `id` | `remodel.scene` / `remodel-fixture` | |
| `streams` | `stream-a` (video, `cam-a`, 2 frames, mp4 `source`), `stream-b` (image-sequence, no camera, 1 frame, no source) | `stream-b` is LAST and is the only stream a GCP observation references, so `delete-stream` exercises the cascade while still restoring stream order |
| `assets` | one key `asset-a` → composed `s.stdio.semio/v1/image` child `remodel-asset-45070beb0101de64` | referenced from 3 frames + `results.mesh.texture_asset_id` → `delete-asset` reports 4 stale references |
| `calibration.cameras` | `cam-a` (brownConrady, posed), `cam-b` (pinhole, locked, UNposed) | `cam-b` is free for `create-rig-extrinsic` and is LAST so `delete-camera-calibration` round-trips |
| `calibration.rig` | one identity pose for `cam-a` | target for `update`/`delete-rig-extrinsic` |
| `params` | all 8 facets populated, every `f32` a dyadic value | see §4 on float canonicality |
| `gcps` | `gcp-ridge` (0 observations), `gcp-corner` (1 observation on `stream-b`) | ridge takes `add-gcp-observation`; corner is LAST so `delete-gcp` cascades AND round-trips |
| `job` | `bundle-adjusting`, progress 0.5, one pose preview, packed preview buffer | |
| `results` | `sparse` (colored), `dense` (all 4 buffers), `mesh` (reconstructed + texture + watertight report), `trajectory` (2 poses), `tracks` (1 moving), `geo` (DSM only), `qc` (1 warning, no watertight) | every replace verb has a non-trivial base to differ from |

## 2. Cases (one per leaf, all `applied`)

| Leaf | Case | What the diff does |
| --- | --- | --- |
| `🌱create-stream` | `adds-stream-c-bound-to-cam-b` | appends to `streams` |
| `🪓delete-stream` | `removes-stream-b-and-cascades-its-gcp-observation` | writes `streams` AND `gcps` (only two-collection leaf) |
| `⏱️change-stream-sync` | `shifts-stream-a-sync-offset-to-minus-seven-and-a-half` | final-state `sync_offset_ms` |
| `➕add-stream-frame` | `appends-a-third-frame-to-stream-a` | pushes frame + re-stamps `kind` |
| `➖remove-stream-frame` | `removes-the-last-frame-of-stream-a` | removes by BASE position |
| `🔁replace-stream-source` | `clears-the-video-source-of-stream-a` | clears `source` |
| `🧷create-asset` | `stores-a-new-jpeg-frame-asset` | mints a composed child handle into `assets` |
| `🗞️delete-asset` | `removes-asset-a-and-reports-its-stale-references` | removes key, reports 4 stale refs |
| `🔭create-camera-calibration` | `adds-the-cam-c-fisheye-calibration` | appends to `calibration.cameras` |
| `🛠️update-camera-calibration` | `refines-the-cam-a-focal-length-and-rms` | in-place record replace |
| `🚫delete-camera-calibration` | `removes-the-cam-b-calibration` | retains out of `cameras`, no cascade |
| `⛓️create-rig-extrinsic` | `adds-a-rig-extrinsic-for-cam-b` | appends to `calibration.rig` |
| `✂️delete-rig-extrinsic` | `drops-the-cam-a-rig-extrinsic` | retains out of `rig` |
| `🔩update-rig-extrinsic` | `retunes-the-cam-a-rig-translation` | in-place pose replace |
| `🧿create-gcp` | `adds-gcp-tower-with-one-observation` | appends record + its observations |
| `🚮delete-gcp` | `removes-gcp-corner-and-cascades-its-observation` | retains out of `gcps`, cascade note |
| `🔎add-gcp-observation` | `adds-the-first-observation-to-gcp-ridge` | pushes observation |
| `🚷remove-gcp-observation` | `removes-the-only-observation-of-gcp-corner` | removes by BASE position |
| `🥣update-ingest-params` | `tightens-the-ingest-sharpness-gate` | `params.ingest` |
| `🌠update-feature-params` | `switches-the-detector-to-akaze` | `params.feature` |
| `🪢update-match-params` | `switches-the-matcher-to-a-kd-tree` | `params.matching` |
| `🧮update-sfm-params` | `switches-the-robust-loss-to-cauchy` | `params.sfm` |
| `🌁update-dense-params` | `raises-the-dense-resolution-and-confidence-gate` | `params.dense` |
| `🕸️update-mesh-params` | `doubles-the-texture-size-and-drops-the-watertight-guarantee` | `params.mesh` |
| `🏎️update-motion-params` | `enables-motion-tracking` | `params.motion` |
| `🌐update-geo-params` | `enables-georeferencing-with-an-origin` | `params.geo` |
| `🏗️replace-job` | `advances-the-job-to-texturing` | whole `job` record |
| `⭐replace-sparse` | `swaps-in-an-uncolored-four-point-sparse-cloud` | `results.sparse` |
| `☁️replace-dense` | `swaps-in-a-two-point-classified-dense-cloud` | `results.dense` |
| `🧱replace-mesh-result` | `swaps-in-an-imported-untextured-mesh` | `results.mesh` |
| `🛣️replace-trajectory` | `clears-the-camera-trajectory` | `results.trajectory` |
| `🚂replace-tracks` | `replaces-the-moving-track-with-two-static-tracks` | `results.tracks` |
| `🗾replace-geo-products` | `adds-the-dtm-and-ortho-rasters` | `results.geo` |
| `🧾replace-qc` | `records-a-qc-report-carrying-a-watertight-summary` | `results.qc` |

Each `🦀️component.rs` carries four tests: a forward test naming that leaf's own field writes and
non-writes, an inverse test asserting the exact inverse mutation VARIANT and payload, the canonical
JSON fixed-point test, and an outcome test asserting which guards stayed silent and which diff
fields the leaf writes. No shared harness, no macro, no loop.

## 3. Rejection / no-op codes found in the diff oracles

Read off `🔺️diff/🦀️component.rs`, in evaluation order. Note that `RemodelDiff::apply` NEVER returns
`Err`: a rejection is an empty diff plus an `Error`/`Fatal` `MutationMessage`, so the outcome tests
inspect `MutationOutcome::messages()`, not the `Result`.

| Leaf | Codes, in the order the diff checks them |
| --- | --- |
| `create-stream` | fatal `mutation.duplicate-id`; fatal `mutation.invariant` (unknown `camera_id`) |
| `delete-stream` | error `mutation.target-missing`; info `mutation.cascade` (GCP observations) |
| `change-stream-sync` | error `mutation.target-missing`; warn `mutation.no-op`; fatal `mutation.invariant` (non-finite offset) |
| `add-stream-frame` | error `mutation.target-missing`; warn `mutation.no-op` (exactly-equal frame present) |
| `remove-stream-frame` | error `mutation.target-missing` (unknown stream); error `mutation.target-missing` (index ≥ len) |
| `replace-stream-source` | error `mutation.target-missing` — **only guard; no no-op at all** |
| `create-asset` | **none** — deliberate upsert, no duplicate-id, no no-op |
| `delete-asset` | error `mutation.target-missing`; info `mutation.cascade` (stale frame/texture/geo refs) |
| `create-camera-calibration` | fatal `mutation.duplicate-id` — no finite-intrinsics check |
| `update-camera-calibration` | error `mutation.target-missing`; warn `mutation.no-op`; fatal `mutation.invariant` (non-finite intrinsics/distortion/RMS) |
| `delete-camera-calibration` | error `mutation.target-missing` — no cascade note |
| `create-rig-extrinsic` | fatal `mutation.duplicate-id`; fatal `mutation.invariant` (unknown camera) |
| `delete-rig-extrinsic` | error `mutation.target-missing` |
| `update-rig-extrinsic` | error `mutation.target-missing`; fatal `mutation.invariant` (non-finite rotation/translation); warn `mutation.no-op` |
| `create-gcp` | fatal `mutation.duplicate-id` |
| `delete-gcp` | error `mutation.target-missing`; info `mutation.cascade` (owned observations) |
| `add-gcp-observation` | error `mutation.target-missing`; warn `mutation.no-op` |
| `remove-gcp-observation` | error `mutation.target-missing` (unknown GCP); error `mutation.target-missing` (index ≥ len) |
| `update-ingest-params` | fatal `mutation.invariant` FIRST (finite ≥0 sharpness, non-zero max-frames/stride); warn `mutation.no-op` |
| `update-feature-params` | fatal `mutation.invariant` FIRST (positive target count, finite ≥0 edge threshold); warn `mutation.no-op` |
| `update-match-params` | fatal `mutation.invariant` FIRST (`ratio_test` in (0, 1]); warn `mutation.no-op` |
| `update-geo-params` | fatal `mutation.invariant` FIRST (positive finite gsd/dsm-cell/dtm-radius, non-zero `ortho_max_px`, lat ∈ [-90,90], lon ∈ [-180,180]); warn `mutation.no-op` |
| `update-sfm-params` | warn `mutation.no-op` FIRST; fatal `mutation.invariant` (finite thresholds) |
| `update-dense-params` | warn `mutation.no-op` FIRST; fatal `mutation.invariant` (finite confidence threshold) |
| `update-mesh-params` | warn `mutation.no-op` FIRST; fatal `mutation.invariant` (finite TSDF voxel size/truncation) |
| `update-motion-params` | warn `mutation.no-op` FIRST; fatal `mutation.invariant` (finite min track quality) |
| `replace-job` | warn `mutation.no-op` only |
| `replace-sparse` | warn `mutation.no-op` only |
| `replace-dense` | warn `mutation.no-op` only |
| `replace-mesh-result` | warn `mutation.no-op` only |
| `replace-tracks` | warn `mutation.no-op` only |
| `replace-trajectory` | error `mutation.target-missing` (clearing an already-absent trajectory); warn `mutation.no-op` |
| `replace-geo-products` | error `mutation.target-missing` (clearing already-absent products); warn `mutation.no-op` |
| `replace-qc` | error `mutation.target-missing` (clearing an already-absent report); warn `mutation.no-op` |

**Guard-order asymmetry worth flagging**: `ingest`/`feature`/`match`/`geo` validate BEFORE the no-op
check, while `sfm`/`dense`/`mesh`/`motion` check the no-op FIRST. A no-op-but-invalid payload
therefore warns on four leaves and is fatal on the other four. Not changed here — recorded only.

Every case in this wave is `applied`; no `rejected` fixtures were authored, so no case carries
`🔺️diff/🚫️component.absent`. The table above is the inventory a rejection wave would draw from.

## 4. Decisions forced by the oracles

1. **`create-asset` mints a content-addressed handle.** `mint_and_stash_asset` hashes with
   `std::collections::hash_map::DefaultHasher`, so the `➡️after` child id cannot be guessed. The
   payload uses `image/jpeg`, which `semio_image_snapshot_from_image_asset` rejects, so the diff
   falls back to `image_asset_child_handle` (a hash of the RAW `(mime, data)` strings). Both
   fixture handles were computed by a throwaway single-file `rustc` program (NOT `cargo`) against
   this toolchain's own hasher: `image/jpeg|ZnJhbWUtYQ== → remodel-asset-45070beb0101de64` and
   `image/jpeg|ZnJhbWUtYg== → remodel-asset-75b20f8d69a86e9a`.
2. **`delete-asset`'s inverse is empty**, not a restore. It reads the deleted bytes back through the
   `thread_local!` working-scene cache, which is cold in a fresh test thread, so the leaf honestly
   returns `Vec::new()`. The test asserts that, and asserts the forward delete is still real.
3. **`delete-stream`'s inverse is lossy.** It emits one `create-stream` and nothing else, so the
   cascaded GCP observation stays gone. The test asserts `snapshot.streams == base.streams` AND
   `snapshot != base`, naming the one-way cascade.
4. **Every list-removing case targets the LAST element.** All inverses of a removal push to the end
   of the `Vec`, so removing a middle element would not round-trip. Middle-element order loss is a
   real property of these leaves and is a candidate for its own fixture wave.
5. **Every `f32` in the fixtures is a dyadic rational** (0.25, 0.5, 0.625, 0.75, 0.875, 0.0625,
   0.03125, 0.125, 1.5, 2.5, 12.5 …). `serde_json::to_value(f32)` stores `f as f64`, so a value like
   `0.3f32` re-encodes as `0.30000001192092896` and the `committed_json_is_canonical` assertion
   would fail against a `0.3` literal. Dyadic values widen exactly. Every float-typed field is also
   written with an explicit decimal point so it decodes as `Number::Float`, not `PosInt`.
6. **Nothing is hand-forged for the derived encodings.** No `.dsl.semio` / `.pack.semio` /
   `.op.semio` / `.spr.semio` / `.patch.semio` and no `🔺️diff/` directories were created; `fixtures
   lint` reports them as expected warnings until `fixtures generate` runs.

## 5. Verification performed

| Check | Result |
| --- | --- |
| `fixtures lint --by-tree` | remodel absent from the uncovered list (0/34) and from every error line |
| case file set | 34 cases × {`🦠️mutation`, `🎯️outcome`, `🦀️component.rs`, `⬅️before`, `➡️after`} — 0 missing |
| `include_str!` targets | 136/136 resolve |
| glue `#[path]` | 388/388 resolve (34 newly added) |
| `rustfmt --edition 2021 --emit stdout` | 34/34 test files parse; `📦️glue.rs` parses |
| JSON sanity | 136 files parse; 5910 float literals, all exactly `f32`-representable; no float-typed field carries an integer literal |

## 6. Could not determine

- Whether the tests actually PASS. `cargo` is unusable (peer's in-flight de-async sweep); nothing
  here claims a green run.
- The real minted child id for an `image/png` asset — it hashes the composed `SemioImageSnapshot`'s
  canonical pack bytes, which needs a built workspace. Sidestepped by using `image/jpeg`.
- `results.mesh.mesh` in `⬅️before` reuses `remodel-mesh-901ccade3f60f8f1` verbatim from the committed
  `📚️examples/🎬️demo` DSL asset (a genuinely minted handle). The `replace-mesh-result` PAYLOAD handle
  `remodel-mesh-2f6c81b0d4a37e59` is synthetic — that leaf stores the payload handle verbatim and
  never re-mints, so no oracle constrains its value.

---

# 🔺️ Follow-up wave — the serialized diff

`🔺️diff/🔣️component.json` is now a required core file. All 34 cases carry one, and each test file
gained the three mandated assertions (`produces_committed_diff`, `committed_diff_is_canonical`,
`committed_diff_applies_to_after`), bringing every case to seven tests.

## 7. What remodel's diff type actually looks like

`RemodelDiff` (`🧬️schema/🔺️diff/🦀️component.rs`) is **not** compose's or puzzle5d's shape. It has no
`added`/`removed`/`patched`/`reordered` collection algebra at all — it is **seventeen top-level
`Option` fields, each of which is a WHOLE-COLLECTION replacement**:

```
artifact, schema, id, streams, assets, calibration, params, gcps, job, results,      ← artifact lane
selection, activeUtilityId, reportTable, frameCursor,                                 ← presence lane
camera, layers, locale                                                                ← config lane
```

`#[serde(rename_all = "camelCase", default)]` on the container and **no `skip_serializing_if` on any
field**, so serde emits all seventeen keys — sixteen `null`s plus the one the leaf wrote. Every
committed diff JSON here is that exact shape.

Consequences that shape the fixtures:

- **The delta is block-level, never field-level.** Every diff builder in this tree does
  `let mut x = base.<collection>.clone(); …edit…; RemodelDiff { <collection>: Some(x), ..Default::default() }`.
  So the written field is byte-for-byte the collection's AFTER state, including every sibling record
  it never touched. `change-stream-sync` ships BOTH streams; `update-ingest-params` ships all eight
  params facets; `replace-sparse` ships the whole results block. The diff therefore pins *which
  collection* a leaf may touch, not which field inside it.
- `streams` and `gcps` are wrapped: `{"values": [...]}` (`RemodelMediaStreamList` / `RemodelGcpList`,
  wrappers that exist only to keep optional list diffs scalar across formats). `assets` is a bare
  map, `calibration`/`params`/`job`/`results` are bare records.
- `assets` is a whole-map replace, so `delete-asset`'s delta is the EMPTIED map and `create-asset`'s
  delta repeats the pre-existing key alongside the new one.
- **`🪓delete-stream` is the only two-field delta in the tree** (`streams` + `gcps`), and `gcps` is set
  only when the cascade count is non-zero — the fixture deliberately triggers it.
- `RemodelDiff::apply` never returns `Err`. A rejection is `MutationOutcome::error/fatal`, which
  yields `RemodelDiff::default()` — an all-`null` diff that applies as a no-op. Had any case been
  classified `rejected`, its file would have been `🚫️component.absent`; **all 34 are `applied` with a
  non-empty diff**, so all 34 carry the JSON and none carries the absent marker.

Field written per leaf (verified against each `🔺️diff/🦀️component.rs`):

| Written field | Leaves |
| --- | --- |
| `streams` | create-stream, change-stream-sync, add-stream-frame, remove-stream-frame, replace-stream-source |
| `streams` + `gcps` | delete-stream (cascade) |
| `assets` | create-asset, delete-asset |
| `calibration` | create/update/delete-camera-calibration, create/delete/update-rig-extrinsic |
| `gcps` | create-gcp, delete-gcp, add-gcp-observation, remove-gcp-observation |
| `params` | the eight `update-*-params` leaves |
| `job` | replace-job |
| `results` | replace-sparse, replace-dense, replace-mesh-result, replace-trajectory, replace-tracks, replace-geo-products, replace-qc |

## 8. Assertions added to every test file

1. `produces_committed_diff` — `serde_json::to_value(<RemodelMutation as protocol::Mutation<RemodelSnapshot>>::diff(…).diff())`
   equals the committed JSON, followed by a handcrafted per-leaf block naming what that delta must
   contain (e.g. change-stream-sync pins `values[0].sync_offset_ms == -7.5` AND that the sibling
   stream is repeated verbatim; delete-asset pins the emptied map AND that `streams`/`results` stay
   `null` so the four stale references are reported and never rewritten).
2. `committed_diff_is_canonical` — decode into `RemodelDiff`, re-encode, compare.
3. `committed_diff_applies_to_after` — `<RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&committed, &before())`
   equals `after`.

## 9. Re-verification after this wave

| Check | Result |
| --- | --- |
| `fixtures lint --by-tree` | remodel raises **0 errors** and appears nowhere in the output (covered 191 → 205 as the diff files landed) |
| lint rules re-run scoped to remodel | 34 variants = 34 leaves = 34 cases; 0 errors; 0 rejected cases so 0 `🚫️component.absent` expected and 0 present |
| case file set | 34 × {mutation, **diff**, outcome, rs, before, after} — 0 missing |
| `include_str!` targets | 170/170 resolve (was 136; +34 `DIFF`) |
| glue `#[path]` | 388/388 resolve, 34 test mods |
| `rustfmt --edition 2021 --emit stdout` | 34/34 test files parse; `📦️glue.rs` parses |
| JSON sanity | 170 files parse; 6403 float literals all exactly `f32`-representable; no float-typed field carries an integer literal; every diff has exactly the 17 expected keys |

Still not run: `cargo`. No test is claimed to pass.
