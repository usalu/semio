# Wave 5 Report — GIS (`semio-s-plugin-gis`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/🌍️gis/**` plus this ticket folder.

Two artifacts:

| Artifact path | Key | Prefix | Schema id | Former snapshot → new |
| --- | --- | --- | --- | --- |
| `🗿️artifacts/🏔️gisterrain/` | `gisterrain` | `GisTerrain` | `s.gis.gisterrain` | `Gis3dTerrainDocument` → `GisTerrainSnapshot` |
| `🗿️artifacts/🗺️gismap/` | `gismap` | `GisMap` | `s.gis.gismap` | `GisMapDocument` → `GisMapSnapshot` |

Apps: `◻2d` ↔ `gismap`, `🧊️3d` ↔ `gisterrain` (bound by `type Snapshot = …`).

## 1. Field inventory (final)

### GisTerrain (`s.gis.gisterrain`)

| Field | State | Notes |
| --- | --- | --- |
| `exaggeration` | persistent | f64; seeded `1.5` from reuse fixture |
| `importedFeaturesJson` | persistent | last-imported `2d.map` descriptor JSON |
| `selectedIds` | shared-ui | from `Gis3dConfig` |
| `cameraJson` | local-ui | free/live world camera JSON |
| `locale` | local-ui | BCP-47 |

Snapshot facet = the two persistent fields. Fixture scenery (`origin` / `position` lines) stays outside the snapshot and is read by the engine’s hand-rolled `terrain_fixture_text` parser from the same `.dsl.semio` file.

### GisMap (`s.gis.gismap`)

| Field | State | Notes |
| --- | --- | --- |
| `positions` | persistent | `Vec<MapFeature>` (`id` + opaque `DslValue` data) |
| `routes` | persistent | `Vec<MapFeature>` |
| `regions` | persistent | `Vec<MapFeature>` |
| `selectedIds` | shared-ui | from `Gis2dConfig` |
| `featureSelectionJson` | shared-ui | `{positions,routes}` selection JSON |
| `layerVisibility` | shared-ui | `BTreeMap<String, bool>` |
| `layerStrokeScale` | shared-ui | `BTreeMap<String, f64>` |
| `cameraJson` | local-ui | `{x,y,zoom}` |
| `renderMode` | local-ui | image / vector / combined |
| `vectorStyle` | local-ui | colored / figureGround / invertedFigure |
| `lodMode` | local-ui | LOD tier id |
| `hoverJson` | local-ui | hover payload or `"null"` |
| `selectionMethod` | local-ui | rectangle / lasso |
| `selectionMode` | local-ui | default / additive / … |
| `locale` | local-ui | BCP-47 |

Snapshot facet = the three persistent feature collections.

## 2. Diff-delta shape

### `GisTerrainDiff`

Sparse field delta:

- `artifact: Option<Box<GisTerrainArtifact>>` — whole replacement wins
- persistent: `exaggeration`, `importedFeaturesJson`
- shared-ui: `selectedIds: Option<GisTerrainStringList>`
- local-ui: `cameraJson`, `locale`

### `GisMapDiff`

Sparse field delta:

- `artifact: Option<Box<GisMapArtifact>>` — whole replacement wins
- persistent: `positions` / `routes` / `regions` as `Option<GisMapFeaturesDelta>` (`added` / `removed` / `patched` / `reordered`)
- shared-ui: `selectedIds: Option<GisMapStringList>`, `featureSelectionJson`, `layerVisibility: Option<GisMapBoolMapDelta>`, `layerStrokeScale: Option<GisMapNumberMapDelta>`
- local-ui: `cameraJson`, `renderMode`, `vectorStyle`, `lodMode`, `hoverJson`, `selectionMethod`, `selectionMode`, `locale`

`MutationDiff<XSnapshot>` applies persistent entries only; `apply_to_artifact` applies all classes. `absorb` merges field-wise.

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]` (same as draw / lowpoly). Nested per artifact:

- `artifacts::<key>::schema`
- `artifacts::<key>::snapshot::{schema, pack}`
- `artifacts::<key>::diff::{component, schema}` (runtime `pub use super::schema::*;`)

TypeScript `📦️packages/🟦️typescript/📦️index.ts` mirrors pack under snapshot plus schema exports.

## 4. Other structural changes

- Fifteen handcrafted leaves × 2 artifacts (`🧬️schema` / `📸️snapshot/🧬️schema` / `🔺️diff/🧬️schema` × rs/ts/graphql/json/proto)
- Pack relocated: `🎒️pack/` → `📸️snapshot/🎒️pack/` for both artifacts
- `Gis3dTerrainDocument` / `GisMapDocument` removed; `GisTerrainSnapshot` / `GisMapSnapshot` live in snapshot schema and are re-exported from artifact roots
- `📄set-document` → `📄set-snapshot` (`SetSnapshot { snapshot }`)
- Engines own real `XArtifact` + cached `XSnapshot` (`type Artifact = XArtifact`, never aliased to Snapshot)
- Descriptor `gisterrain_artifact_schema_descriptor()` / `gismap_artifact_schema_descriptor()` registered from `engine::register_artifact_schema()`
- Apps / views / stores use `Snapshot` / `.snapshot` / `initial_snapshot`
- Handcrafted reuse fixtures restored under each artifact’s `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (terrain includes scenery lines; map seeds the Liège pin + two named routes)
- Config envelope ids set to `gis.gis2dcfg` / `gis.gis3dcfg` (plugin.artifact form required by `SemioEnvelope::from_envelope_id`)
- Tests call `store::os_store::test_support::*` (kernel glob-export workaround)

## 5. Gate tails (verbatim)

### cargo check -p semio-s-plugin-gis

```
warning: `semio-s-plugin-gis` (lib) generated 7 warnings (run `cargo fix --lib -p semio-s-plugin-gis` to apply 7 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 3.98s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### cargo test -p semio-s-plugin-gis --lib

```
test result: ok. 144 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'gis'

```
(empty — no lines matched)
```

No `gis` lines matched — no artifact-schema policy breaches for this plugin in the filtered gate.

## 6. Shared-surface notes

- Concurrent framework sweep was fixing `semio-framework-os-flow` / host `.projection()` call sites. Final gates for `semio-s-plugin-gis` compiled and tested green without requiring edits outside `✏️s/🔌️plugins/🌍️gis/**`.
- Kernel crate-root glob-exports both `os_dsl::test_support` and `os_store::test_support`; bare `store::test_support::*` resolves to the DSL helper set. GIS tests use `store::os_store::test_support::*` (plugin-side workaround, same as draw).

## 7. Not validated

- Full `bun nx run workspace:verify-gate` (out of scope; other artifacts may still be incomplete).
- Runtime UI / playground smoke beyond lib tests.
- TypeScript vitest package tests (not required by the wave-5 GIS brief gates).
