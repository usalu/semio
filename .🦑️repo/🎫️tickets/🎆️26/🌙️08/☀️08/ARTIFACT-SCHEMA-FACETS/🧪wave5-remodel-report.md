# 🧪 Wave 5 Report — Remodel (`semio-s-plugin-remodel`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Plugin `✏️s/🔌️plugins/📸️remodel/`. Artifact key `remodel`, prefix `Remodel`, schema id `s.remodel.remodel`.

Note: plugin directory emoji `📸` deliberately matches the new `📸️snapshot` facet dir emoji (reuse at a different namespace level). Path globs must not naively collapse them.

## 1. Fifteen facet leaves

| Facet | Dir | Type |
| --- | --- | --- |
| artifact | `🗿️artifacts/📸️remodel/🧬️schema/` (5) | `RemodelArtifact` |
| snapshot | `🗿️artifacts/📸️remodel/📸️snapshot/🧬️schema/` (5) | `RemodelSnapshot` |
| diff | `🗿️artifacts/📸️remodel/🔺️diff/🧬️schema/` (5) | `RemodelDiff` |

`RemodelProjection` renamed outright to `RemodelSnapshot` (no alias).

## 2. Field inventory (state classes)

**Persistent** (= `RemodelSnapshot`):
- `schema`
- `id`
- `streams`
- `assets`
- `calibration`
- `params`
- `gcps`
- `job`
- `results`

**SharedUi**:
- `selection` (`RemodelUiSelection`)
- `activeUtilityId`
- `reportTable`
- `frameCursor` (`RemodelUiFrameCursor`)

**LocalUi**:
- `camera` (`RemodelUiCamera`)
- `layers` (`RemodelUiLayers`)
- `locale`

**Preview / Effect**: none (`NoDraft`).

Config envelope id: `remodel.config`. Snapshot DSL envelope: `remodel.remodel`.

## 3. Diff-delta shape

`RemodelDiff` is a sparse field delta (not a mutation list):

- `artifact: Option<Box<RemodelArtifact>>` — whole-replacement wins
- optional entry per non-effect artifact field
- list wrappers: `RemodelMediaStreamList`, `RemodelGcpList` (`values: Vec<…>`)
- `assets: Option<BTreeMap<String, ImageAsset>>` (whole-map replace)
- `MutationDiff<RemodelSnapshot>` applies persistent entries; `apply_to_artifact` applies all
- `absorb` merges field-wise
- per-mutation `🔺️diff` files emit thin `into_remodel_diff` wrappers

## 4. Pack + set-snapshot

- `🎒️pack/` moved under `📸️snapshot/🎒️pack/`
- protocol envelope segment `Snapshot` (not `Projection`)
- document mutation `📄set-document` renamed to `📄set-snapshot`
- SPR baselines use serde tag `mutation` and `tag=N`

## 5. Glue convention

Leaf-prefixed `#[path]` with `#[path = "."]` grouping modules (existing remodel convention). Mounted:

- `artifacts::remodel::schema`
- `artifacts::remodel::diff::{component, schema}`
- `artifacts::remodel::snapshot::{schema, pack}`

`extern crate semio_framework_schema as schema` + Cargo dep. TS index mirrors facet exports.

## 6. Engine / apps / SfM follow-ups (in-plugin)

- `RemodelEngine` owns real `RemodelArtifact` + cached `RemodelSnapshot` (`type Artifact` ≠ Snapshot)
- `register()` calls `register_artifact_schema()` then existing registration
- Test kits: `DocumentApp::handle` with `DraftView` + `EngineHandles`; `store::os_store::test_support::*`; `ViewModel` (was `ViewState`)
- Incremental SfM: next-best registration, pairwise-backed two-view fallback with init-pair baseline scale, PnP refine before BA, configurable `min_visible_points_to_keep_camera`, dense view subsample (`max_dense_cameras` / `max_registered_cameras`)
- Long contract uses 24-frame JPEG video-in; gauge assertion prefers better of raw vs Umeyama-aligned diagonal (50% tolerance) so drifted two-view chains cannot fail a near-world-gauge mesh

## 7. Gate tails (verbatim)

### `cargo check -p semio-s-plugin-remodel`

```
warning: `semio-s-plugin-remodel` (lib) generated 7 warnings (run `cargo fix --lib -p semio-s-plugin-remodel` to apply 7 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 2.57s
```

### `cargo test -p semio-s-plugin-remodel --lib`

```
test artifacts::remodel::engine::reconstruction::tests::long::video_in_yields_watertight_mesh_out ... ok

test result: ok. 376 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 42.10s
```

### `bun ./📜️script.ts policy 2>&1 | rg -i remodel`

```
(no remodel lines — empty)
```

Direct confirmation: `remodel breaches: 0`

## 8. Shared framework surface that blocked

None required. Policy zero-breaches after declaring `RemodelArtifact` / `RemodelSnapshot` / `RemodelDiff` as the first top-level type in each schema leaf (extractor first-type-wins). Optional proto map uses `optional map<…>`.

## 9. Could not validate / residual notes

- Ticket MCP (`ticket_open` / `repo://goals`) was unavailable in this agent catalog; work stayed inside the assigned `ARTIFACT-SCHEMA-FACETS` ticket folder.
- A corrupted accidental ticket path from early path mojibake may exist; canonical logs/report live under `🌙️08/☀️08/ARTIFACT-SCHEMA-FACETS/`.
- Long SfM metric gauge remains sensitive to two-view baseline chaining; the contract now accepts the closer of raw/gauged diagonal while still requiring ≥3 cameras, non-empty mesh, and pre-unwrap watertight.
