# FacetReport — `📸️remodel` / `📸️remodel`

- **facet**: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`
- **status**: `done` for code; **`blocked-churn` for the test gate** — `cargo check -p semio-s-plugin-remodel` passed clean (observed), but `cargo test` cannot build: its dependency `semio-framework-plugin` has 15 foreign errors from another session. See `gates`.
- **count**: 34 mutations replacing 20 banned `Set*` variants. Zero `Set*` variants survive in the dispatch enum.

## mutationsCreated

| # | triad dir (emoji + slug) | verb | entity | Rust variant | superseded old variant |
|---|---|---|---|---|---|
| 1 | `🌱create-stream` | create | stream | `CreateStream` | `SetStreams` |
| 2 | `🪓delete-stream` | delete | stream | `DeleteStream` | `SetStreams` |
| 3 | `⏱️change-stream-sync` | change | stream | `ChangeStreamSync` | `SetStreams` |
| 4 | `➕add-stream-frame` | add | stream | `AddStreamFrame` | `SetStreams` |
| 5 | `➖remove-stream-frame` | remove | stream | `RemoveStreamFrame` | `SetStreams` (new, inverse-only counterpart) |
| 6 | `🔁replace-stream-source` | replace | stream | `ReplaceStreamSource` | `SetStreams` |
| 7 | `🧷create-asset` | create | asset | `CreateAsset` | `SetAsset` |
| 8 | `🗞️delete-asset` | delete | asset | `DeleteAsset` | `SetAsset{value:None}` |
| 9 | `🔭create-camera-calibration` | create | camera-calibration | `CreateCameraCalibration` | `SetCalibration` |
| 10 | `🛠️update-camera-calibration` | update | camera-calibration | `UpdateCameraCalibration` | `SetCalibration` |
| 11 | `🚫delete-camera-calibration` | delete | camera-calibration | `DeleteCameraCalibration` | `SetCalibration` |
| 12 | `⛓️create-rig-extrinsic` | create | rig-extrinsic | `CreateRigExtrinsic` | `SetCalibration` |
| 13 | `✂️delete-rig-extrinsic` | delete | rig-extrinsic | `DeleteRigExtrinsic` | `SetCalibration` |
| 14 | `🔩update-rig-extrinsic` | update | rig-extrinsic | `UpdateRigExtrinsic` | `SetCalibration` |
| 15 | `🧿create-gcp` | create | gcp | `CreateGcp` | `SetGcps` |
| 16 | `🚮delete-gcp` | delete | gcp | `DeleteGcp` | `SetGcps` |
| 17 | `🔎add-gcp-observation` | add | gcp | `AddGcpObservation` | `SetGcps` |
| 18 | `🚷remove-gcp-observation` | remove | gcp | `RemoveGcpObservation` | `SetGcps` (new, inverse-only counterpart) |
| 19 | `🥣update-ingest-params` | update | ingest-params | `UpdateIngestParams` | `SetIngestParams` |
| 20 | `🌠update-feature-params` | update | feature-params | `UpdateFeatureParams` | `SetFeatureParams` |
| 21 | `🪢update-match-params` | update | matching-params | `UpdateMatchParams` | `SetMatchParams` |
| 22 | `🧮update-sfm-params` | update | sfm-params | `UpdateSfmParams` | `SetSfmParams` |
| 23 | `🌁update-dense-params` | update | dense-params | `UpdateDenseParams` | `SetDenseParams` |
| 24 | `🕸️update-mesh-params` | update | mesh-params | `UpdateMeshParams` | `SetMeshParams` |
| 25 | `🏎️update-motion-params` | update | motion-params | `UpdateMotionParams` | `SetMotionParams` |
| 26 | `🌐update-geo-params` | update | geo-params | `UpdateGeoParams` | `SetGeoParams` |
| 27 | `🏗️replace-job` | replace | job | `ReplaceJob` | `SetJob` |
| 28 | `⭐replace-sparse` | replace | sparse | `ReplaceSparse` | `SetSparse` |
| 29 | `☁️replace-dense` | replace | dense | `ReplaceDense` | `SetDense` |
| 30 | `🧱replace-mesh-result` | replace | mesh-result | `ReplaceMeshResult` | `SetMeshResult` |
| 31 | `🛣️replace-trajectory` | replace | trajectory | `ReplaceTrajectory` | `SetTrajectory` |
| 32 | `🚂replace-tracks` | replace | tracks | `ReplaceTracks` | `SetTracks` |
| 33 | `🗾replace-geo-products` | replace | geo-products | `ReplaceGeoProducts` | `SetGeoProducts` |
| 34 | `🧾replace-qc` | replace | qc | `ReplaceQc` | `SetQc` |

All 34 emoji are unique within this facet and distinct from the (now deleted) old set.

## genericVariantsRemoved

All 20, in full, with nothing left behind: `SetStreams`, `SetAsset`, `SetCalibration`, `SetGcps`,
`SetIngestParams`, `SetFeatureParams`, `SetMatchParams`, `SetSfmParams`, `SetDenseParams`,
`SetMeshParams`, `SetMotionParams`, `SetGeoParams`, `SetJob`, `SetSparse`, `SetDense`,
`SetMeshResult`, `SetTrajectory`, `SetTracks`, `SetGeoProducts`, `SetQc`.

Also removed: the hand-written `impl Mutation<RemodelSnapshot> for RemodelMutation` (its 20-arm `diff`
match and its `inverse` match), the `apply_remodel_mutation_in_place` free function, and all 20
apply-and-capture `pub fn apply(next: &mut RemodelSnapshot, …)` mutation leaves.
`apply_remodel_mutation`/`inverse_remodel_mutation` survive as thin free-function wrappers over the
derive-generated trait methods (matching `🎬️sequence`'s own pattern), because
`🎛️apps/📸️remodel/🦀️component.rs`'s tests call `apply_remodel_mutation` by name.

`SetSnapshot`, `NoMutation`, `CollectionMutation<`/`::` — grepped repo-wide across this plugin
(`*.rs` + `*.ts`): **zero occurrences before or after.** This facet never had them.

## Vocabulary decisions — every `Set*`, what it became, and why

### The three collection questions the coordinator asked about, answered directly

- **`SetStreams { streams: Vec<MediaStream> }`** — this was a **whole-`Vec` setter**, and
  `MediaStream` **is id-keyed** (`MediaStream.id: String`, and every app call site already searched by
  `stream.id`). Decomposed into **6 element-scoped mutations** (#1–#6). No whole-collection setter
  survives.
- **`SetGcps { gcps: Vec<GroundControlPoint> }`** — same shape, same finding: whole-`Vec` setter over
  an **id-keyed** collection (`GroundControlPoint.id`). Decomposed into **4 mutations** (#15–#18).
  No whole-collection setter survives.
- **`SetTracks { tracks: Vec<MotionTrackSummary> }`** — this one **is** a `Vec`, and
  `MotionTrackSummary` **does** carry an `id`, so it superficially looks id-keyed. I deliberately
  kept it as a single `replace-tracks`, and this is the one judgement call most worth challenging.
  Justification: there is **no per-track user gesture anywhere in the plugin** — I grepped every call
  site. `results.tracks` is written in exactly two places: the reconstruction engine's whole-run
  output (`run_whole_pipeline`, which re-derives all tracks from scratch on every run) and
  `clearTracks` (`replace_tracks(Vec::new())`). Nothing creates, deletes, renames, or edits an
  individual track; the `id` is an engine-minted label for the report table, not a user-addressable
  handle. That makes it structurally identical to `sparse`/`dense`/`trajectory` — a large
  engine-derived structured sub-payload swapped wholesale — so it takes the same `replace` verb they
  do. **If a future editor gains per-track selection/deletion, this must be decomposed into
  create/delete-track and this justification retired.** It is a whole-`Vec` payload, but not a
  whole-*collection setter* in the banned sense: it is one field of `results`, always re-derived, never
  incrementally edited.

### `change` — single scalar field on an addressed target (1)

- `SetStreams` → **`change-stream-sync{id, new_sync_offset_ms}`**. The `setStreamSync` command edits
  exactly one `f64` on one addressed stream. Single scalar ⇒ `change`, not `update`.

### `add`/`remove` — set-like `Vec` members on an owner (4)

- `SetStreams` → **`add-stream-frame{id, frame, kind}`** / **`remove-stream-frame{id, frame_index}`**.
  `MediaStream.frames: Vec<FrameRef>` is appended one frame at a time by three import handlers.
- `SetGcps` → **`add-gcp-observation{id, observation}`** / **`remove-gcp-observation{id, observation_index}`**.
  `GroundControlPoint.observations: Vec<GcpObservation>` is appended one at a time by
  `placeGcpObservation`.

### `create`/`delete` — id-keyed entity lifecycle (10)

`create-stream`/`delete-stream`, `create-asset`/`delete-asset`,
`create-camera-calibration`/`delete-camera-calibration`,
`create-rig-extrinsic`/`delete-rig-extrinsic`, `create-gcp`/`delete-gcp`.
Each `delete`'s inverse captures the full removed payload from BASE. No cascades exist in this
snapshot (nothing references a stream/gcp/camera by foreign key inside the document), so no
re-`connect` step is needed.

### `replace` — large structured sub-payload, whole-value swap (9)

`replace-stream-source`, `replace-job`, `replace-sparse`, `replace-dense`, `replace-mesh-result`,
`replace-trajectory`, `replace-tracks`, `replace-geo-products`, `replace-qc`.
All are engine-written or `clear*`-written blobs (`VideoSource`, `ReconstructionJob`, point clouds,
`RemodelMesh`, `CameraTrajectory`, `GeoProducts`, `QcReportSnapshot`). None is ever field-edited by a
user; each has exactly one writer that computes the whole value. `replace-mesh-result` keeps its
`Box<RemodelMesh>` (`clippy::large_enum_variant`).

### `update` — inseparable multi-field facets, ALL 10 justified individually

`update` is the verb most likely to be wrong, so each is justified against the actual call site:

1. **`update-ingest-params`** — `setIngestParams` command takes `{frame_sample_stride, max_frames,
   downscale_long_edge_px, min_sharpness}` as one flat palette arg-form and constructs the whole
   `IngestParams`. There is no code path that writes one field. ✔ inseparable.
2. **`update-feature-params`** — `setFeatureParams` submits `{detector, target_count, octaves,
   edge_threshold}` together; `detector` is a string→enum decode done in the same handler. ✔
3. **`update-match-params`** — `setMatchParams` submits all 6 fields together. ✔
4. **`update-sfm-params`** — `setSfmParams` submits all 6 fields together (incl. the `robust_loss`
   enum decode). ✔
5. **`update-dense-params`** — `setDenseParams` submits all 5 together. ✔
6. **`update-mesh-params`** — `setMeshParams` submits all 9 together; the three watertight knobs
   (`guarantee_watertight`, `hole_fill_max_boundary_verts`, `self_intersection_check`) are validated
   as a group by the mesher. ✔
7. **`update-motion-params`** — `setMotionParams` submits all 5 together. ✔
8. **`update-geo-params`** — `setGeoParams` submits all 8 together (incl. 3 interleaved `Option`
   origin fields that only make sense as a set). ✔
9. **`update-camera-calibration`** — `EditCalibration` submits `{camera_id, label, model, fx, fy, cx,
   cy, skew, k1..k3, p1, p2, locked}` as ONE properties form and rebuilds the whole
   `CameraCalibration`. Intrinsics + distortion are a jointly-validated optical model — `fx` without
   `cx` is not a meaningful edit. The pre-existing code already did
   `match cameras.iter_mut().find(…) { Some(e) => *e = entry, None => push(entry) }`, i.e. a
   full-record replace. That branch is now the explicit `update` vs `create` choice at the call site. ✔
10. **`update-rig-extrinsic`** — one rigid pose `{rotation_wxyz, translation_m}`. A quaternion and its
    translation are one SE(3) element; changing one component alone yields a different pose, not a
    partial edit. ✔ (No app call site writes `calibration.rig` today — schema-complete but
    unexercised, as briefed.)

**Nothing was given `update` merely to avoid enumerating fields.** Every `update` above corresponds to
exactly one arg-form / one engine write that always carries the complete block.

### Deviations from the briefed spec (2, both additive)

- **`remove-stream-frame` (#5)** and **`remove-gcp-observation` (#18)** were added — the brief left
  the `add-*` inverse design to my discretion. Chosen because it is the only shape that makes
  `assert_mutation_inverse_law` hold honestly: `add-stream-frame`'s inverse looks up BASE's
  `frames.len()` and emits `remove-stream-frame{id, frame_index: <that len>}`, which addresses exactly
  the element the append will create. Both removal mutations document in their own payload doc-comment
  that they round-trip exactly for the last index only — which is the only index `add-*` ever produces,
  since frames/observations are append-only and never reordered. The alternative (reconstructing the
  whole pre-add `Vec` inside the inverse) would have smuggled a whole-collection setter back in, which
  is exactly the forbidden shape. Count therefore went 32 → 34.
- **`delete-asset` (#8)** was added as `create-asset`'s inverse-only counterpart. `assets` is a
  `BTreeMap`; `create-asset` is an upsert. When BASE already has the key, the inverse is
  `create-asset{key, old}` (recreate the old value); when BASE does not, the inverse must express
  "this key did not exist", which needs a real removal mutation. No app command calls `delete-asset`
  today; a `MutationKind` is legitimate without its own command route.

### Sparseness note (`RemodelDiff` is coarse by construction — not my choice)

`RemodelDiff.streams: Option<RemodelMediaStreamList>` and `.gcps: Option<RemodelGcpList>` are
whole-list-when-present, and `.calibration`/`.params`/`.results` are whole-sub-struct-when-present.
`RemodelDiff`'s shape is out of this facet's scope and `🔺️diff/🦀️component.rs` was left untouched.
Every `diff()` therefore clones only the ONE relevant `base` sub-value, applies only the ONE targeted
change, and populates only the ONE `RemodelDiff` top-level field (`..Default::default()` everywhere
else). No `diff()` ever constructs a mutated whole `RemodelSnapshot`, so this is direct sparse
construction from `(payload, base)`, not apply-and-capture. I verified `assets` REPLACES rather than
merges on apply (`🔺️diff/📝️text/🦀️component.rs`'s `MutationDiff::apply` does
`next.assets = assets.clone()`), so `create-asset`/`delete-asset` clone `base.assets` and mutate the
clone — a single-entry map would have silently dropped every other asset.
Every mutation targeting a possibly-absent entity returns `RemodelDiff::default()` when the target is
missing (the `📋️forms/➕add-step` idempotent-early-return idiom), and every `inverse` returns
`Vec::new()` for a missing target.

## filesTouched

### created (139 files: 34 triads × (3 `🦀️component.rs` + 3 `🟦️component.ts`), minus none)

All under
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`,
one `🦠️mutation/🦀️component.rs` + `🦠️mutation/🟦️component.ts` + `🔺️diff/🦀️component.rs` +
`🔺️diff/🟦️component.ts` + `↩️inverse/🦀️component.rs` + `↩️inverse/🟦️component.ts` per dir:

`🌱create-stream/`, `🪓delete-stream/`, `⏱️change-stream-sync/`, `➕add-stream-frame/`,
`➖remove-stream-frame/`, `🔁replace-stream-source/`, `🧷create-asset/`, `🗞️delete-asset/`,
`🔭create-camera-calibration/`, `🛠️update-camera-calibration/`, `🚫delete-camera-calibration/`,
`⛓️create-rig-extrinsic/`, `✂️delete-rig-extrinsic/`, `🔩update-rig-extrinsic/`, `🧿create-gcp/`,
`🚮delete-gcp/`, `🔎add-gcp-observation/`, `🚷remove-gcp-observation/`, `🥣update-ingest-params/`,
`🌠update-feature-params/`, `🪢update-match-params/`, `🧮update-sfm-params/`, `🌁update-dense-params/`,
`🕸️update-mesh-params/`, `🏎️update-motion-params/`, `🌐update-geo-params/`, `🏗️replace-job/`,
`⭐replace-sparse/`, `☁️replace-dense/`, `🧱replace-mesh-result/`, `🛣️replace-trajectory/`,
`🚂replace-tracks/`, `🗾replace-geo-products/`, `🧾replace-qc/`

(= 102 `.rs` + 102 `.ts`; verified programmatically that every one of the 34 dirs has exactly 3 of each.)

### updated

- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — dispatch enum rewritten (`dsl::DslEnum` + `dsl::Mutations`, `#[mutations(snapshot = RemodelSnapshot, diff = RemodelDiff, schema = "remodel.scene")]`, 34 single-field-tuple variants), re-export block, `apply_/inverse_remodel_mutation` wrappers, `#[cfg(test)]` region rewritten
- `.../🧬️mutations/📝️text/🦀️component.rs` — dropped the `apply_remodel_mutation_in_place` re-export
- `.../🧬️mutations/💾️binary/🦀️component.rs` — two test fixtures moved to `update_feature_params(…)`
- `.../🧬️mutations/🟦️component.ts` — was `export {};`, now a real 34-member `RemodelMutationTag` union
- `.../🧬️mutations/🔗️component.graphql` — was a snapshot-shaped placeholder, now a `union RemodelMutation` + one `type` per kind
- `.../🧬️mutations/🔣️component.json` — now a `oneOf` over 34 `$defs`, one per kind
- `.../🧬️mutations/🛰️component.proto` — now `oneof kind` over 34 tagged messages
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs` — one doc comment naming `SetMeshResult` reworded to `ReplaceMeshResult` (policy greps comments)
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📦️glue.rs` — `pub mod mutations { … }` block: 20 old mounts removed, 34 new ones added (same `#[path]` depth)
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🦀️component.rs` — `import_media("photos:in")` now emits `create-asset` + (`add-stream-frame` | `create-stream`); test assertion message updated
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/⚙️params/🦀️component.rs` — 8 handlers → the 8 `update_*_params` builders
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/🎯️calibration/🦀️component.rs` — `EditCalibration` now picks `update-` vs `create-camera-calibration` by id presence; `CalibrateCameras` emits one `create-camera-calibration` per newly-derived camera; `AddGcp`→`create-gcp`, `RemoveGcp`→`delete-gcp`, `PlaceGcpObservation`→`add-gcp-observation`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/📥️ingest/🦀️component.rs` — all 7 handlers rewritten onto `create-asset`/`create-stream`/`add-stream-frame`/`replace-stream-source`/`delete-stream`/`change-stream-sync`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/🧹️reset/🦀️component.rs` — 7 clear/reset handlers → `replace-*`
- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/🚀️reconstruction/🦀️component.rs` — engine publish path → `replace-job`/`replace-sparse`/`replace-trajectory`/`replace-mesh-result`/`replace-qc`/`replace-geo-products`/`create-asset`

### removed (20 old triad dirs, 3 `.rs` + 3 `.ts` each = 120 files)

`✅️set-qc/`, `✨️set-sparse/`, `🌍️set-geo-products/`, `🌟️set-feature-params/`, `🌧️set-dense/`,
`🌫️set-dense-params/`, `🎛set-mesh-params/`, `🎞️set-streams/`, `🏃️set-motion-params/`, `🏭️set-job/`,
`📈️set-tracks/`, `📍️set-gcps/`, `📐️set-calibration/`, `📥️set-ingest-params/`, `📦️set-mesh-result/`,
`🔗️set-match-params/`, `🖼️set-asset/`, `🗺️set-geo-params/`, `🛤️set-trajectory/`, `🧭️set-sfm-params/`
(all under the `🧬️mutations/` dir above).

## sharedFileRequests

**None.** Every edit is inside `✏️s/🔌️plugins/📸️remodel/**`, including its own `📦️glue.rs`.
Root `📜️script.ts` was NOT touched (allowlist changes are listed below for the coordinator to apply).

## allowlistKeysToRemove

`POLICY_SEMANTIC_VOCABULARY_ALLOWLIST` — I inspected the list and **no `📸️remodel` path is in it**
(this facet's `Set*` variants were flagged at `medium`/advisory, never allowlisted), so there is
**nothing to remove**. Recorded for completeness: these two paths are now free of the medium-priority
bare-`Set*` token and were the only remodel `mutation-migration/semantic-vocabulary` findings my work
could clear:

- `✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel/🎮️commands/⚙️params/🦀️component.rs` — the 8
  `Set*Params` hits are the **app command payload struct names** (`set_ingest_params::SetIngestParams`
  etc.), not mutations. They are wire-format rows in `app_commands!` with pinned binary ordinals and
  pinned hex fixtures (`optional_field_rows_keep_their_pre_migration_bytes`), and their manifest action
  ids (`"setIngestParams"`) are the host-facing API. Renaming them is an app-command-surface change,
  not a mutation-vocabulary one — out of this facet's scope. Same for
  `🎮️commands/👁️view/🦀️component.rs` (`SetSelection`/`SetCamera`/…, all `ActionKind::View`, no document
  mutation at all) and the two residual hits in `🎮️commands/📥️ingest/🦀️component.rs`
  (`SetStreamSync`, and a test that constructs `SetIngestParams`). **Flagging for the coordinator**: if
  app command structs are in scope for a later wave, these 3 files are the remaining `Set*` surface in
  this plugin.

## gates

**NOT RUN to completion — deferred to the coordinator per explicit instruction** (~79 contending
cargo processes across sessions; per-lane gating was pure waiting). Honest record of what I *did*
observe:

1. `cargo check -p semio-s-plugin-remodel` — **RAN, OBSERVED, PASSED.**
   `Finished \`dev\` profile [unoptimized] target(s) in 11m 24s`,
   `warning: \`semio-s-plugin-remodel\` (lib) generated 13 warnings`, **0 errors**. I grepped the full
   output for every one of my 34 new triad dirs and the dispatch file: **zero warnings and zero errors
   attributable to any file I created or edited.** All 13 warnings are pre-existing
   (`unnecessary qualification` in `⚙️engine`, `unused import` in `🚪️io`/`🔺️diff/📝️text`,
   `field \`artifact\` is never read`).
2. `cargo test -p semio-s-plugin-remodel --lib` — **RAN ONCE, FAILED TO COMPILE, FIXED, RE-RUN
   ABANDONED.** First run produced exactly 2 errors, both mine, both the same cause — verbatim:
   ```
   error[E0599]: no variant, associated function, or constant named `kinds` found for enum `RemodelMutation` in the current scope
      --> …/🧬️mutations/🦀️component.rs:318:38
   318 |         for kind in RemodelMutation::kinds() {
   error[E0599]: no variant, associated function, or constant named `kinds` found for enum `RemodelMutation` in the current scope
      --> …/🧬️mutations/🦀️component.rs:321:37
   321 |         assert_eq!(RemodelMutation::kinds().len(), 34);
   error: could not compile `semio-s-plugin-remodel` (lib test) due to 2 previous errors; 16 warnings emitted
   ```
   Cause: the derive-generated `SemanticMutation` trait was not in scope. **Fixed** by adding
   `use protocol::SemanticMutation as _;` at the dispatch file's top (and removing the now-redundant
   duplicate `use protocol::Mutation as _;` from the test module).
   **Re-run outcome: `blocked-churn` (foreign).** The re-run landed after the report was first
   written. It never reached `semio-s-plugin-remodel` — it died compiling a *dependency*:
   ```
   error[E0277]: `(dyn SpaceMember + 'static)` cannot be sent between threads safely
   error: could not compile `semio-framework-plugin` (lib) due to 15 previous errors; 40 warnings emitted
   ```
   All 15 errors resolve to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/…` — **zero are in any
   `✏️s/🔌️plugins/📸️remodel` file.** This is another session's in-flight framework refactor, matching
   the brief's `blocked-churn` clause; I did not attempt to fix it and did not retry.
   **Net: I have still NOT observed a green remodel test run and do not claim one.** What I can state
   from observation: the pre-fix build compiled all 34 triads, the derive expansion, and every
   rewritten call site with the 2 errors above as its *only* remodel defects, and both are fixed.
   **Please re-run `cargo test -p semio-s-plugin-remodel --lib` once `semio-framework-plugin` is
   green again.**
3. `bun ./📜️script.ts policy` — **RAN, OBSERVED.** Zero `mutation-migration/semantic-vocabulary`
   **high**-priority breaches anywhere in `📸️remodel` (the only 2 repo-wide are foreign, both in
   `🗄️stdio/🧿️semio`). Remodel's remaining findings: 3 × `medium` bare-`Set*` (all in app **command**
   files — see `allowlistKeysToRemove` above), 1 × `medium` `dispatch-coverage` reading "2 triad dir(s)
   with no variant" (this is the rule miscounting `📝️text/` and `💾️binary/` as triad dirs — they are the
   facet's codec leaves, not mutations; the same false positive exists in every migrated facet), and
   2 × `high` `triad-completeness`/`artifact-engine` claiming the artifact "is missing required
   `🧬️mutations/` / `⚙️engine/` facet" — both pre-existing and both false (the dirs exist; the rule
   looks under `🗿️artifacts/📸️remodel/` directly rather than under `🏅️standards/🔖️1/…`). **No new
   high-priority breach kind was introduced by this work.**

**Foreign break check (per your note):** remodel is **NOT affected** by the
`📌️panels/📄️document/` → `📄️artifact/` rename. Its glue already mounts
`#[path = "../../🎛️apps/📸️remodel/📌️panels/📄️artifact/🦀️component.rs"] pub mod document;` (line 811) —
the disk path is the new one. That is consistent with the test build above having gotten far enough to
report only my 2 errors.

## lawTests

Written into the dispatch file's existing `#[cfg(test)]` region (no new test file), region
`🔖️MutationLaws`. **These are authored but their green run is UNVERIFIED** — see `gates` #2.

- `assert_mutation_inverse_law` (from `protocol::testkit`) — **all 34 kinds covered**, across 9 tests:
  `create_delete_stream_inverse_law`, `change_stream_sync_inverse_law`,
  `add_remove_stream_frame_inverse_law`, `replace_stream_source_inverse_law`,
  `create_delete_asset_inverse_law` (covers both the overwrite and the fresh-key branch of
  `create-asset`), `camera_calibration_inverse_law`, `rig_extrinsic_inverse_law`, `gcp_inverse_law`,
  `update_params_inverse_law` (8 kinds), `replace_job_and_results_inverse_law` (8 kinds).
- `assert_mutation_diff_absorb_law` — 1 test (`move_step_style_diff_absorb_law`) on
  `change-stream-sync`, the facet's only true single-scalar `change`.
- `store::os_store::test_support::assert_op_line_round_trip` — `every_mutation_variant_roundtrips_through_op_text`,
  **39 calls covering all 34 variants** (5 extra calls exercise the `None` branch of the optional
  payloads: `replace-stream-source`, `replace-sparse`, `replace-dense`, `replace-trajectory`,
  `replace-geo-products`, `replace-qc`).
- `dispatch_registers_semantic_descriptors` — calls the derive-generated
  `register_remodel_mutation_descriptors()`, asserts `protocol::is_approved_verb(kind.verb)` for every
  kind, and asserts `RemodelMutation::kinds().len() == 34`.
- Convergence tests kept and rewritten: `concurrent_create_asset_ops_converge_regardless_of_order`
  (disjoint asset keys) and `concurrent_edits_across_different_op_families_converge`
  (`update-feature-params` vs `create-gcp`).
- `populated_scene_fixture` kept verbatim as the shared fixture (it exercises every optional/collection
  field, so the law helpers walk real data, not `default_remodel_scene()`'s empty surface).
- Compile-time laws come free from `#[derive(dsl::Mutations)]`: per-variant
  `assert!(str_eq(SEMANTICS.kind, <kebab of variant>))` and `assert!(is_approved_verb(SEMANTICS.verb))`.
  The successful `cargo check` in `gates` #1 means **all 34 of those const assertions already passed** —
  every `kind` matches its triad-dir stem and its variant kebab, and every verb is in `APPROVED_VERBS`.

## incomplete / requeue precisely

1. **`cargo test -p semio-s-plugin-remodel --lib` has never been observed green — now blocked by
   foreign churn.** The `SemanticMutation` import fix landed, but the re-run died in the dependency
   `semio-framework-plugin` (15 errors, all under `🧰️framework/…/🔌️plugin/`, headline
   `error[E0277]: (dyn SpaceMember + 'static) cannot be sent between threads safely`). Nothing in
   remodel is implicated. Requeue this single command once that crate builds.
2. **`assert_op_text_binary_equivalence` not added for the mutation enum.** The facet has both codecs
   and `💾️binary/🦀️component.rs` already calls it for one mutation; a full per-variant binary/text
   equivalence sweep would be the natural completion. Not attempted — I did not want to add 34 more
   unverified assertions on top of an unverified test run.
3. **`DiffAlgebra for RemodelDiff` not implemented.** Step 7 of the brief mentions implementing it if
   missing; it is missing, but it lives in `🔺️diff/`, which the brief scoped OUT of this facet
   (`leave 🔺️diff/🦀️component.rs itself untouched`). Flagging so you can route it deliberately —
   `assert_diff_algebra_between_law`/`assert_diff_algebra_inverse_law` cannot be used here until it exists.
4. **`💾️binary/📡️component.protocol.semio` not given per-mutation `record <VariantPascal> tag N` rows.**
   I rewrote the `graphql`/`json`/`proto` descriptions honestly, but the binary-set `.protocol.semio`
   (and its `.abnf`/`.ksy`/`.spicy` siblings) still carry the old generic shape. The binary wire form is
   derive-generated via `dsl::variants_binary`, so nothing is *broken*, but the description file is now
   stale relative to the enum.
5. **`📖️component.grammar.semio` left as-is.** Its current content
   (`add-vertex`/`set-face`/`transform-mesh`/`merge-solid`) describes a mesh-op grammar that matches
   neither the old nor the new enum — it was already stale before this work, and rewriting it is a
   grammar-authoring task with its own `handcrafted-grammar/*` policy rules (19,601 repo-wide
   high-priority findings in that family already). Deliberately not touched; requeue as grammar work.
6. **App command structs still named `Set*`** in `🎮️commands/⚙️params`, `🎮️commands/👁️view`,
   `🎮️commands/📥️ingest` — see `allowlistKeysToRemove`. These are the host-facing action surface with
   pinned wire ordinals, not mutations. Out of scope here; requeue as a command-surface wave if wanted.
