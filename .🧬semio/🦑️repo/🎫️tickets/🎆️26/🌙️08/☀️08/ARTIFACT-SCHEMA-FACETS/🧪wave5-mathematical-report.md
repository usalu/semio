# Wave 5 Report — Mathematical (`semio-s-plugin-mathematical`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/➗️mathematical/**` plus this ticket folder.
Key `mathematical`, prefix `Mathematical`, schema id `s.mathematical.mathematical`. Former snapshot type `MathProjection` → `MathematicalSnapshot`.

## 1. Field inventory (final)

| Field | State | Source |
| --- | --- | --- |
| `graph` | persistent | document (`MathematicalGraph`: directed, nodes, edges, algorithm, algorithmSeed) |
| `geometry` | persistent | document (`MathematicalGeometry` point cloud) |
| `cameraX` / `cameraY` / `cameraZoom` | local-ui | `MathematicalConfig.camera` (flattened in artifact schema) |
| `locale` | local-ui | `MathematicalConfig` |

Snapshot facet = `graph` and `geometry` exactly.

**Computed results (not in snapshot):** `algorithm_overlay` / `workflow_json` label suffixes are **preview** (derived each render from persisted graph). `result:out` export JSON (`algorithm` + `overlay` map) is **effect** (fire-and-forget port output). Convex-hull / centroid canvas layers from `geometry_layers_json` are **preview** (derived from points at render).

## 2. Diff-delta shape

`MathematicalDiff` sparse field delta:

- `artifact: Option<Box<MathematicalArtifact>>` — whole replacement wins
- persistent: `graph: Option<MathematicalGraph>`, `geometry: Option<MathematicalGeometry>` (whole-slice replacement; mutations still `SetGraph` / `SetGeometry` / `SetSnapshot`)
- local-ui: `cameraX`, `cameraY`, `cameraZoom`, `locale` as optional scalars

`MutationDiff<MathematicalSnapshot>` applies persistent entries; `apply_to_artifact` applies all classes. `absorb` merges field-wise with artifact replacement dominance.

## 3. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]`:

- `artifacts::mathematical::schema`
- `artifacts::mathematical::snapshot::{schema, pack}`
- `artifacts::mathematical::diff::{component, schema}` (`pub use super::schema::*`)

TypeScript `📦️index.ts` mirrors snapshot pack path and three schema facet exports.

## 4. Other structural changes

- Fifteen handcrafted schema leaves under `🧬️schema`, `📸️snapshot/🧬️schema`, `🔺️diff/🧬️schema`
- Pack relocated: `🎒️pack/` → `📸️snapshot/🎒️pack/`
- `MathematicalEngine` owns `MathematicalArtifact` + `MathematicalSnapshot`; `ArtifactEngine` API
- `DocumentApp` uses `Snapshot` / `.snapshot` / `initial_snapshot`; config envelope `mathematical.config`
- `mathematical_artifact_schema_descriptor()` registered from `engine::register()`
- Real `🗣️example.dsl.semio` round-trip fixture (`mathematical.mathematical` envelope)
- `SetSnapshot` mutation + `📄set-snapshot` taxonomy folder
- Tests: `store::os_store::test_support`, `InvocationResult.mutations`, removed duplicate `#[dsl(keyword)]` on `SetAlgorithm` (wire baseline)

## 5. Gate tails (verbatim)

### cargo check -p semio-s-plugin-mathematical

```
warning: `semio-s-plugin-mathematical` (lib) generated 6 warnings (run `cargo fix --lib -p semio-s-plugin-mathematical` to apply 6 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 31.20s
```

### cargo test -p semio-s-plugin-mathematical --lib

```
test result: ok. 52 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'math'

```
(empty — no lines matched)
```

Direct `policyArtifactSchemaBreaches(root)` filter for `mathematical` → **0** breaches.

## 6. Shared-surface blockers

None for this crate; workspace intermittently fails on unrelated plugins during parallel edits.

## 7. Not validated

- Full `bun nx run workspace:verify-gate`
- TypeScript vitest package run
- Interactive playground / UI smoke beyond lib tests
