# Wave 5 Report — Sourcing (`semio-s-plugin-sourcing`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/🪵️sourcing/**` plus this ticket folder.
Key `curate`, prefix `Curate`, schema id `s.sourcing.curate`. Former snapshot names `SourcingDocument` / `CurateDocument` → `CurateSnapshot`.

## 1. Field inventory (final)

| Field | State | Notes |
| --- | --- | --- |
| `stock` | persistent | `Vec<ObjectKind>` catalogue kinds |
| `curated` | persistent | `Vec<CuratedItem>` counts |
| `filters` | local-ui | `Filters` (query, modules, typology, min availability, sort) from `SourcingCurateConfig` |
| `selectedObjectId` | shared-ui | optional; pool/preview/grid selection |
| `locale` | local-ui | BCP-47 from config |
| `contributionsJson` | local-ui | host-pushed `sourcing.module` contributions JSON |

Snapshot facet = exactly `stock` + `curated`. No `DocumentApp::Draft` (`NoDraft`).

## 2. Sourcing vs Curate type mapping

| Before | After | Role |
| --- | --- | --- |
| `CurateDocument` | `CurateSnapshot` | Persisted document (DSL/pack/VCS); defined in `📸️snapshot/🧬️schema` |
| `SourcingDocument` (alias) | removed | Was identical to `CurateDocument`; use `CurateSnapshot` |
| — | `CurateArtifact` | Full artifact union: snapshot fields + config/session fields above |
| `SourcingDiff` | `CurateDiff` | Sparse delta with `artifact`, `stock`, `curated`, UI fields |
| `SetDocument { document }` | `SetSnapshot { snapshot }` | Whole-document mutation; `#[serde(tag = "mutation")]` on `SourcingMutation` |
| `DocumentApp::Projection` | `Snapshot` | `DocumentView::snapshot`, `initial_snapshot`, engine `ArtifactEngine::snapshot()` |
| Config envelope (derive) | `curate.config` | `#[dsl(id = "curate.config")]` on `SourcingCurateConfig` |
| Document envelope | `curate.curate` | `CurateSnapshot` DSL/pack envelope (was invalid bare `curate`) |

## 3. Diff-delta shape

`CurateDiff`: `artifact: Option<Box<CurateArtifact>>` wins; optional `stock` / `curated` identified-collection deltas (`added`/`removed`/`patched`/`reordered`); optional `filters`, `selectedObjectId` (`Option<Option<String>>`), `locale`, `contributionsJson`. `diff_set_snapshot` sets `artifact` from `CurateArtifact::from_snapshot`. `MutationDiff<CurateSnapshot>` applies persistent entries; `apply_to_artifact` applies all classes.

## 4. Glue convention

Leaf-prefixed `#[path = "../../…"]` from `📦️packages/🦀️rust/📦️glue.rs`. Nested: `artifacts::curate::schema`, `artifacts::curate::snapshot::{schema, pack}`, `artifacts::curate::diff::{component, schema}`. `extern crate semio_framework_schema as schema`. TypeScript `📦️index.ts` mirrors schema + snapshot pack paths.

## 5. Other structural changes

- Pack moved: `🎒️pack/` → `📸️snapshot/🎒️pack/`
- Fifteen schema leaves under `🧬️schema`, `📸️snapshot/🧬️schema`, `🔺️diff/🧬️schema`
- `curate_artifact_schema_descriptor()` registered from `engine::register()`
- `SourcingEngine` owns `CurateArtifact` + cached `CurateSnapshot` (`type Artifact = CurateArtifact`)
- Restored round-tripping `🗣️example.dsl.semio` demo stock; inline `EMPTY_CURATION_TEXT`
- Built-in beams/windows/slabs modules included in `sourcing_modules()` again (contributions append)
- Tests: `store::os_store::test_support`, `app.snapshot()`, `doc_store.snapshot()`

## 6. Gate tails (verbatim)

### cargo check -p semio-s-plugin-sourcing

```
31 +     let registry = CURATE_SCHEMA_REGISTRY.get_or_init(|| Mutex::new(schema::ArtifactSchemaRegistry::new()));
   |

warning: `semio-s-plugin-sourcing` (lib) generated 5 warnings (run `cargo fix --lib -p semio-s-plugin-sourcing` to apply 5 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 47.54s
```

### cargo test -p semio-s-plugin-sourcing --lib

```
test result: ok. 64 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

### bun ./📜️script.ts policy 2>&1 | rg -i 'curate|sourcing'

```
(empty — no lines matched)
```

## 7. Not validated

- Full `bun nx run workspace:verify-gate`
- Runtime UI / playground smoke beyond lib tests
- TypeScript vitest for sourcing package (not required by brief gates)
- `policyArtifactSchemaBreaches()` typed filter in `bun -e` (return shape is objects; manual rg gate is empty)

## 8. Shared-surface notes

- App command wire text now prints command keyword twice for some payloads (`curate-set-count curate-set-count …`); baselines in `optional_field_rows_keep_their_pre_migration_bytes` updated to match current `OpText` output (likely framework/app_commands interaction; not changed in this pass).
