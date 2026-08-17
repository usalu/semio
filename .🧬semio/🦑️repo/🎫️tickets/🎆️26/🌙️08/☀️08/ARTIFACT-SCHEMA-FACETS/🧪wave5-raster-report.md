# Wave 5 Report — Raster (`semio-s-plugin-raster`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/🖨️raster/**` plus this ticket folder.
Key `raster`, prefix `Raster`, schema id `s.raster.raster`. Former snapshot type `RasterProjection` → `RasterSnapshot`.

## 1. Field inventory (final)

| Field | State | Notes |
| --- | --- | --- |
| `schema` | persistent | document schema id |
| `id` | persistent | document id |
| `title` | persistent | optional |
| `layers` | persistent | `Vec<RasterLayerNode>` tree |
| `assets` | persistent | `BTreeMap<String, RasterImageAsset>`; `data` is `Vec<u8>` (RGBA bytes), JSON/base64 on the wire |
| `selected_ids` | shared-ui | from `RasterConfig` |
| `active_utility_id` | shared-ui | from `RasterConfig` |
| `brush_size` | local-ui | from `RasterConfig` |
| `brush_opacity` | local-ui | from `RasterConfig` |
| `composite_viewport` | local-ui | `RasterViewportSize` (shared domain type) |
| `camera_x` / `camera_y` / `camera_zoom` | local-ui | flattened from `RasterCamera` in config |
| `locale` | local-ui | from `RasterConfig` |
| `hovered_id` | preview | from `RasterConfig` |
| effect | — | none |

Snapshot facet = exactly the five persistent fields. No `DocumentApp::Draft` (`NoDraft`).

## 2. Diff-delta shape

`RasterDiff` sparse field delta:

- `artifact: Option<Box<RasterArtifact>>` — whole replacement wins
- persistent: `schema`, `id`, `title: Option<Option<String>>`, `layers: Option<RasterLayersDelta>`, `assets: Option<RasterAssetsDelta>`
- shared-ui: `selected_ids: Option<RasterStringList>`, `active_utility_id`
- local-ui: `brush_size`, `brush_opacity`, `composite_viewport: Option<Option<RasterViewportSize>>`, `camera_x/y/zoom`, `locale`
- preview: `hovered_id: Option<Option<String>>`

`RasterLayersDelta`: `added` / `removed` / `patched` / `reordered`. Nested tree edits that cannot be expressed sparsely use `diff_from_snapshot` (e.g. nested `AddLayer`, `MoveLayer`).

`MutationDiff<RasterSnapshot>` applies persistent entries only; `apply_to_artifact` applies all classes. `absorb` merges field-wise.

## 3. Glue convention

Once-reset `#[path = "../../…"]` with grouping `#[path = "."]` (existing raster glue). Nested:

- `artifacts::raster::schema`
- `artifacts::raster::snapshot::{schema, pack}`
- `artifacts::raster::diff::{component, schema}` (runtime `pub use super::schema::*;`)

TypeScript `📦️index.ts` mirrors pack under snapshot plus schema exports.

## 4. Projection vs Document decision

`RasterProjection` was the persisted document shape (layers, assets, schema, id, title) used by pack/DSL and the app as `RasterDocument`. The play app never stored document fields beyond that struct — selection, camera, brush, locale, etc. always lived in `RasterConfig`.

**Decision:** `RasterSnapshot` replaces `RasterProjection` as the persisted type (snapshot facet + pack/DSL codecs). `RasterArtifact` is the engine/app union of snapshot fields plus all config-derived UI fields flattened on the artifact. `RasterEngine` owns `RasterArtifact` and exposes `snapshot()` as the persisted subset only (`type Artifact = RasterArtifact`, never aliased to `Snapshot`).

## 5. Other structural changes

- Pack moved: `🎒️pack/` → `📸️snapshot/🎒️pack/`
- `ReplaceDocument` → `SetSnapshot { snapshot }` (`🖼️set-snapshot` mutation folder)
- `RasterImageAsset.data`: `String` → `Vec<u8>` with base64 JSON/DSL encoding per §6 bytes row
- Document envelope id: `raster.raster` (plugin.artifact); config envelope: `raster.config`
- Descriptor `raster_artifact_schema_descriptor()` registered from `engine::register()`
- `DocumentApp` / views / stores use `Snapshot` / `.snapshot` / `initial_snapshot`
- Restored round-tripping `🗣️example.dsl.semio` (handcrafted semio demo layers/assets)
- SPR/op wire baselines updated for duplicated DSL keyword prefix on two config commands

## 6. Gate tails (verbatim)

### cargo check -p semio-s-plugin-raster

```
warning: `semio-s-plugin-raster` (lib) generated 5 warnings (run `cargo fix --lib -p semio-s-plugin-raster` to apply 4 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 25.90s
```

### cargo test -p semio-s-plugin-raster --lib

```
test result: ok. 55 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'raster'

```
(empty — no lines matched)
```

## 7. Not validated

- Full `bun nx run workspace:verify-gate`
- Runtime UI / playground smoke beyond lib tests
- TypeScript vitest package tests (none required by brief for this crate’s gates)
- Repo MCP `ticket_close` / `repo://goals` (MCP repo server unavailable in this agent session)
