# Wave 5 Report — FEM (`semio-s-plugin-fem`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Owns `✏️s/🔌️plugins/🏗️fem/**` plus this ticket folder.

| Artifact | Key | Prefix | Schema id | Old → new snapshot |
| --- | --- | --- | --- | --- |
| `🗿️artifacts/◻2d/` | `fem2d` | `Fem2d` | `s.fem.fem2d` | `Fem2dDocument` → `Fem2dSnapshot` |
| `🗿️artifacts/🧊️3d/` | `fem3d` | `Fem3d` | `s.fem.fem3d` | `Fem3dDocument` → `Fem3dSnapshot` |

## 1. Field inventory (final)

### Fem2d

| Field | State | Notes |
| --- | --- | --- |
| `nodes` | persistent | snapshot |
| `elements` | persistent | snapshot |
| `regions` | persistent | snapshot (2d only) |
| `materials` | persistent | snapshot |
| `sections` | persistent | snapshot |
| `supports` | persistent | snapshot |
| `loadCases` | persistent | snapshot |
| `combinations` | persistent | snapshot |
| `analysis` | persistent | snapshot (`FemAnalysisSettings`) |
| `resultSourceId` | shared-ui | optional case/combo id for results |
| `resultMode` | shared-ui | contour / reactions / modal / buckling |
| `resultModeIndex` | shared-ui | mode index |
| `camera` | local-ui | `FemCamera` |
| `locale` | local-ui | 2d only |
| `solverResultsJson` | preview | engine-derived; not persisted |
| `meshPreviewJson` | preview | engine-derived; not persisted |
| effect | — | none retained as fields (`results:out` is a port side-effect) |

### Fem3d

Same classification as Fem2d except: `solids` replaces `regions`; no `locale`.

Snapshot facet = exactly the persistent fields. Artifact facet = snapshot ∪ shared-ui ∪ local-ui ∪ preview.

## 2. Solver / preview classification rationale

`solverResultsJson` and `meshPreviewJson` are **preview**, not persistent:

- Recomputed from mesh + loads + analysis settings via `fem2d_solve*` / `fem3d_solve*` and mesh-preview helpers.
- Bundled examples and `results:out` export solve on demand; caching them in the snapshot would duplicate authority with the mesh and drift on reload.
- They still appear on `Fem*Artifact` so the engine/view-model can surface them without inventing a parallel type.

No dedicated effect fields: result export is an IO port (`results:out`), not an artifact field.

## 3. Diff-delta shape

`Fem2dDiff` / `Fem3dDiff` are sparse field deltas (lowpoly pattern):

- `artifact: Option<Box<Fem*Artifact>>` — whole replacement wins
- per-collection deltas (`added` / `removed` / `patched` / `reordered`) for nodes, elements, regions|solids, materials, sections, supports, loadCases, combinations
- `analysis: Option<FemAnalysisSettings>`
- shared/local/preview optional entries mirror the artifact
- helpers: `diff_set_*` / `diff_remove_*` / `diff_set_analysis` / `diff_set_snapshot`
- `MutationDiff<Fem*Snapshot>` applies persistent entries; `apply_to_artifact` applies all classes

`SetDocument` → `SetSnapshot { snapshot }`.

## 4. Glue convention

Leaf-prefixed `#[path = "../../…"]` with grouping `#[path = "."]` (same as lowpoly). Nested:

- `schema` at artifact root
- `diff { runtime + schema }`
- `snapshot { schema + pack }`

TS `📦️index.ts` re-exports schema / snapshot / diff / pack mirrors. `Cargo.toml` depends on `semio-framework-schema`; glue `extern crate semio_framework_schema as schema`.

Engines own real `Fem*Artifact` + cached `Fem*Snapshot` (`type Artifact = Fem*Artifact`, never `= Fem*Snapshot`). Schema descriptors registered via `OnceLock` registry.

## 5. Structural moves

- `🎒️pack/` relocated under `📸️snapshot/🎒️pack/`
- Mutation folders `📄set-document` → `📄set-snapshot`
- Fifteen leaves × 2 artifacts generated and kept in parity
- Protocol ids `2d.*` / `3d.*` → `fem2d.*` / `fem3d.*`; SPR records use `tag=N` (parser requires `=`)
- Bundled demo DSL restored from handcrafted-grammar dump (12 nodes / 9 elements / region + dead/live/uls for 2d; 16/16/solid for 3d)

## 6. Gates (verbatim tails)

### `cargo check -p semio-s-plugin-fem`

```
781 +     type Snapshot = Fem3dSnapshot;
    |

warning: `semio-s-plugin-fem` (lib) generated 65 warnings (run `cargo fix --lib -p semio-s-plugin-fem` to apply 47 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.40s
```

### `cargo test -p semio-s-plugin-fem --lib`

```
test formulation::tests::gauss_tri_panics_on_unsupported_order - should panic ... ok
test formulation::tests::gauss_1d_panics_on_unsupported_order - should panic ... ok
test sparse::tests::pcg_reports_not_converged_when_max_iter_is_zero ... ok
test sparse::tests::rcm_reduces_bandwidth_on_scattered_path_graph ... ok
test sparse::tests::pcg_matches_ldlt_and_dense_lu ... ok
test sparse::tests::subspace_iteration_matches_dense_jacobi_on_small_nondiagonal_case ... ok
test sparse::tests::subspace_iteration_matches_diagonal_analytic_case ... ok
test artifacts::fem3d::snapshot::pack::tests::fem3d_pack_agrees_with_dsl_for_bundled_default_example ... ok
test artifacts::fem3d::snapshot::pack::tests::fem3d_pack_agrees_with_dsl_for_fixture_documents ... ok
test apps::fem3d::modes::edit::windows::results::tests::results_window_renders_buckling_mode_shape_3d ... ok
test apps::fem3d::modes::edit::windows::results::tests::results_scene_captions_name_mode_and_factor_3d ... ok
test artifacts::fem3d::engine::component::tests::example_fixture_parses ... ok

test result: ok. 332 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
```

### `bun ./📜️script.ts policy 2>&1 | rg -i 'fem'` (+ probe)

```
=== bun ./📜️script.ts policy | rg -i fem ===
(no stdout matches — breaches live in cache / probe API)
=== policyArtifactSchemaBreaches filter fem ===
fem artifact-schema breaches: 0
fem total policy breaches: 129
byKind {
  "app-plugin/app-coupling": 1,
  "taxonomy/dead-example-leaf": 5,
```

**Verdict:** cargo check green; 332/332 lib tests pass; **0** `artifact-schema` breaches for fem. Remaining fem-tagged policy hits (emoji VS16, dead-example-leaf, mutation-migration, etc.) are pre-existing taxonomy/migration noise outside this wave's facet contract (lowpoly pilot left similar non-ASB residue).

## 7. Shared-framework blockers

None that blocked FEM facet delivery. Notes for fixup (do not fix in this wave):

1. Protocol dialect requires `record Name tag=N` (`=` mandatory). Several sibling plugins still use `tag N` and will fail the same parse once exercised.
2. Identifiers with digit-leading path segments (`fem.2d-…`) break protocol Ident lexing — FEM protocols now use `fem.fem2d` / digit-free record names.
3. Repo MCP `ticket_*` tools were unavailable in this Cursor session; report landed in the existing ticket folder without MCP close.

## 8. Files touched (high level)

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻2d,🧊️3d}/🧬️schema/**` (15 leaves × 2)
- `…/📸️snapshot/{🧬️schema,🎒️pack}/**`
- `…/🔺️diff/{🦀️component.rs,🧬️schema/**}`
- `…/⚙️engine/**`, `🗣️dsl/**`, `📡️spr/**`, `🧬️mutations/**` (incl. `📄set-snapshot`)
- `…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (both artifacts)
- `🎛️apps/{◻2d,🧊️3d}/**` (DocumentApp snapshot API + tests)
- `📦️packages/{🦀️rust/{Cargo.toml,📦️glue.rs},🟦️typescript/📦️index.ts}`
- Ticket: `🧪wave5-fem-*.{md,txt,ts,semio}` probes/logs + this report
