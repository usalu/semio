# r7 execution report: z-depth (authored depth parameters, framework straight skeleton, beam top_offset)

`S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `G` = `🧰️framework/🔨️modules/📐️geometry`.

## Status
* Framework (verified): `cargo test --manifest-path G/📦️packages/🦀️rust/Cargo.toml` = lib 168 passed (22 skeleton, 12 roof), 9 + 5 oracle/first-party tests, 0 failed; `--target wasm32-wasip2 --lib` check OK;
  python oracle `G/🧪️tests/🦴️skeleton-oracles/🐍️.py` 104 passed (`.venv/Scripts/python.exe -m pytest -p no:cacheprovider`).
* BIM crate: last full run BEFORE the workspace split (commit 675), `cargo test … --lib`: 3656 passed, 19 failed. The 19: 6 editor close-stall (`zz_debug_close*`, schedule, world window, two plan windows), 6 gesture `app_tests`, 3 window-config sum-law tests,
  1 diagnostics table count (60 vs 54, z-graph's in-flight message rows), and two tests my change touched, fixed afterwards: `entities::the_table_reads_every_entity_of_the_demo_model` (storey `cut_height` row now reads the default 1.2) and
  the renamed opening-frames sill test. NONE of my 11 new+changed leaves/cases failed (all mutation law tests green after bless).
  After commit 675 the BIM dependency graph does not compile at HEAD for reasons outside this scope (`semio_framework::io` missing, `protocol::mutation_fixture_ops`, window-config `ConfigRecord`/`MutationDiff` traits, stdio-ifc borrow error); the
  last check (`check11`) shows only those errors, none in files I own. The post-split state of my last edits (editor entities row types, renamed test) is therefore WRITTEN BUT UNVERIFIED until z-baseline lands.

## Built (schema-first; `r3-f1-gen-model.ts` is the single source, regenerated all Rust/JSON Schema/TS/GraphQL/proto/patches)
* Snapshot: `Storey.cut_height: Option<f64>`; `Opening.sill` -> `sill_override: Option<f64>` (REPLACES the type sill; F27); `Stair` + `stringer: StairStringer{kind None|Closed|Open|Mono,width,depth}`, `nosing`, `tread_thickness`, `riser Open|Closed`, `landing_depth`;
  `Railing` + `profile`, `post_profile`, `baluster: Option<Baluster{profile,spacing}>`, `infill None|Glass{thickness}|Panel{thickness}`. Field descriptions in Rust docs and JSON Schema `description`. Standard values in `S/🧬️schema/📸️snapshot/✅️validity` (`STANDARD_*`, `DEFAULT_CUT_HEIGHT`), plus `stair_construction_problem`, `railing_construction_problem`, `cut_height_problem`.
* Leaves: `set-storey-cut-height` (new, tag 12, 6 cases; `Assigned<Option<f64>>`, null = default); `set-opening` (`sill_override` assigned null), `create-opening`, `create-stair`/`set-stair` (5 new fields, refusals for stringer size, nosing >= min tread, tread thickness, landing depth), `create-railing`/`set-railing` (sections, baluster with assigned null, infill), `create-storey` (cut_height refusal). x-semio-ui en/de for every new property; 28 new fixture cases (all blessed from code and reviewed); generators `r3-f1-gen-mutation-facets.ts`, `r3-f1-gen-oracle.ts` (88 kinds, 540 scenarios), `r3-f1-gen-feature.ts` re-run; `r3-f1-check-names.ts` ok.
* Migration by hand-regeneration (no leftovers): `r7-z-depth-migrate.py` (244 JSON docs: every fixture, sill semantics converted so resolved sills are unchanged), `r4-x-examples-gen.ts` (house/office regenerated, DSL blessed; house shows closed/open stringers, nosing, balusters, panel infill, attic cut height), `r7-z-depth-new-cases.ts`, Rust literals (tests, editor, IFC import: `sill_override` only when it differs from the type), python oracles read `sill_override`.
* Editor: entity rows (storey cut height, sill override, stair construction, railing sections/baluster/infill), parsers, create defaults, en/de labels.
* Beam `top_offset` decided by reading solids/IFC/bodies: signed, + above / - below the storey top, 0 flush. Docs of the entity, create/set-beam leaves, schema description (en/de), design doc updated; validation stays "finite".
* Design: `r2-design.md` snapshot rows, catalogue (`set-storey-cut-height`), ruling 8.
* Framework: `G/🦴️skeleton` (weighted straight skeleton for simple polygons with holes, per-edge speeds, vertical edges, horizon + wavefront, cancellation, 22 tests incl. 400 random stars, 400 weighted stars, 300 histograms, 200-edge run) and `G/🏠️roof` (hip, gable via per-edge vertical pitch, mansard, per-edge pitched; faces, ridge/hip/valley/verge/break lines, closed layer shells with volume = thickness x plan area; 12 tests). Fixture `G/🧫️fixtures/🦴️skeleton/🔣️.json` (54 cases) + schema.
* Oracle: `py_straight_skeleton` 0.1.0 added as TEST dependency (`pyproject.toml` test group, `uv.lock`; never runtime) + shapely mitre buffer / half-plane intersection; documented in `r7-api-skeleton.md`.
* Hand-offs: `r7-api-skeleton.md` (API), `r7-depth-inference-handoff.md` (exact solids/plan/quantities/roof/diagnostics work for the new parameters, 7 sections).

## Edits inside the inference tree (mechanical only)
`opening-frames/resolve_size` (`sill_override.unwrap_or(type_sill)`), struct literals in inference unit tests, one test rename. Inference semantics for the new parameters are NOT implemented (handoff).

## Open
1. Re-verify the BIM crate once z-baseline fixes HEAD: `cargo test --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml -p semio-s-artifact-bim-model --lib`, wasm check, `BIM_BLESS=1 … bless_the_` if DSL texts drift.
2. Skeleton limit: vertical edge next to a reflex corner is not a physical roof (handoff 4.5 requires a hip fallback + warning).
3. Roof/stair/railing/cut-height inference behaviour: z-graph or later agent per handoff.
