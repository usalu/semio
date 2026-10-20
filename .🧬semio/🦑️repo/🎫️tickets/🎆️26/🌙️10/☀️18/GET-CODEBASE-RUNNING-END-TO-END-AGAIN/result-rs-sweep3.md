# rs-sweep3 result

Executor `rs-sweep3`. Scope: Rust crates under `✏️s/**`, `🎓️teaching/**`, `🏢️semio-tech/**`, `♻️mit-bestand/**`. Checker: `C/rs-sweep3-tools/` (copy of `rs-sweep-tools`; rustfmt syntax gate fixed to `--config skip_children=true`, repoint widen and import-insertion fixes, macro-collected items treated as `pub`). Static analysis only: **no cargo, no `bun repo dependencies`, no `bun repo show projects`, no bun test, no product activation was run by me.** The cargo window runner had produced no `results.tsv` when I finished (it was still waiting on `bun repo dependencies lock --all`), so there were no compile errors to read.

## Result

| run | scope | findings |
| --- | --- | ---: |
| `all1` (start, whole universe `scope-all.txt`, before dependency sync fixes) | 2,286 touched packages | 15,532 |
| `all8` (after first repoint/fixdeps rounds) | same | 1,079 |
| `it1` | same | 650 |
| `it4` | same | 248 |
| `it6` (with rustfmt syntax gate) | same | 62; **syntax gate: 25,975 files, 0 syntax errors** |
| `it7` (final) | same | **37** |

Syntax of the 27 Rust files touched after `it6` was re-checked with rustfmt (0 errors). All 372 changed `Cargo.toml` files parse as TOML (`Bun.TOML.parse`; 5 reported ENOENT are renames made by peers).

Per-crate before/after for the 380 crates of the first scope (`C/🗑️generated/rs-sweep3-run/table.md`):

| crate | before | after |
| --- | ---: | ---: |
| `artifact-norm-din18599-checks` | 25 | 0 |
| `fem-analyses` | 24 | 1 |
| `artifact-puzzle-5d` | 18 | 0 |
| `artifact-energy-model-editor` | 12 | 0 |
| `artifact-stdio-docx` | 11 | 0 |
| `artifact-puzzle-2d-v1` | 10 | 0 |
| `fem-analyses-model` | 9 | 0 |
| `artifact-fem-2d-editor` | 8 | 1 |
| `artifact-fem-3d-editor` | 8 | 1 |
| `artifact-fem-3d-editor-commands` | 8 | 0 |
| `artifact-flow-flow-editor-leaves` | 8 | 0 |
| `artifact-procedural-generation3d` | 7 | 0 |
| `artifact-stdio-semio-value-mutations` | 7 | 0 |
| `artifact-puzzle-2d-text-inverse` | 6 | 0 |
| `artifact-puzzle-3d-mutations` | 5 | 0 |
| `artifact-puzzle-5d-interaction` | 5 | 0 |
| `fem-analyses-assembly` | 5 | 21 |
| `artifact-fem-3d-editor-session` | 4 | 0 |
| `artifact-cad-cad` | 3 | 0 |
| `artifact-fem-2d-editor-interaction` | 3 | 0 |
| `artifact-puzzle-5d-retained` | 3 | 0 |
| `artifact-puzzle-5d-store-preparation` | 3 | 0 |
| `artifact-stdio-dwg-snapshot` | 3 | 0 |
| `artifact-energy-model-mutation-fixtures` | 2 | 0 |
| `artifact-fem-2d-editor-session` | 2 | 0 |
| `artifact-fem-3d-editor-results-window` | 2 | 0 |
| `artifact-stdio-docx-base-mutations` | 2 | 0 |
| `artifact-energy-model-editor-results` | 1 | 0 |
| `artifact-fem-2d-mutations` | 1 | 0 |
| `artifact-fem-3d` | 1 | 0 |
| `artifact-fem-3d-mutations` | 1 | 0 |
| `artifact-flow-flow-editor` | 1 | 0 |
| `artifact-norm-iso16757-editor` | 1 | 0 |
| `artifact-puzzle-2d-tests` | 1 | 0 |
| `artifact-puzzle-2d-unit-tests` | 1 | 1 |
| `artifact-raster-raster-editor` | 1 | 0 |
| `artifact-sequence-sequence-tests` | 1 | 0 |
| `artifact-wfc-2d` | 1 | 0 |
| `artifact-writer-writer` | 1 | 0 |
| `fem-sparse-subspace` | 1 | 0 |
| `plugin-energy-engine-sim-unit-tests` | 1 | 0 |
| `plugin-energy-epw` | 1 | 0 |
TOTAL 218 25

## What changed (root causes, not symptoms)

- Upward dependency inversion fixed by moving the definition down: iso16757 `Iso16757Snapshot::reference_fixture` into the model crate; vdi3805 Blatt/valve datasets and `sync_*_into_records` into the model crate; cad `derive-testing`; note snapshot into schema; `integer_json_scalar!` into engine-json; `Puzzle3dPlaySnapshot` serde impls into the defining crate; `BinaryApps`/`TxtApps` into the carve crate that needs them.
- Private items given a real cross-crate user made `pub` (never without a user): html diff codec helpers, fem assembly types, ldlt checkpoint, raster `image_content_child_handle`/`RASTER_OWNED_FIELD_BYTES`/`asset_image`/`released`, norm `checks::timber`/`checks::masonry`, cad `commit_session`, semio cad/document/model `wire_tag`.
- `cfg(test)` items needed by other crates' tests moved behind the repo `testkit` feature (`cfg(any(test, feature = "testkit"))`, feature declared, consumer dev-dependency with the feature): din16798, ifc 2x3/4 demo cases, pptx/xlsx/docx demo cases and `opc_layer::demo_cases`, bim `storey_bands`/`infer_selected`/`stair_geometry`/`table_problems`/`assert_window_config_laws`, block-3d `exact_borrowed_wire`, svg-diff feature declaration, video demo mutations (body moved into the lib; the unit test now delegates).
- Stale APIs migrated: 14 callers of `assert_mutation_inverse_sum_law_cold` and 5 of `assert_mutation_inverse_law_cold`/`assert_mutation_diff_absorb_law_cold` moved to the non-cold async laws; sqlite io tests use the `original_owners::io_run(route, payload, limits, progress)` signature; `bestest::run` -> `sim::testing::run_preborn_factory`; flow-snapshot request close/demand through `MemberOpenRequest`/`ErasedSnapshotRetirement::next_demand`.
- Restored lost code: `norm_contract::payload_op_binary` (module body had been dropped by a carve, left as an orphan doc comment; restored verbatim from `7c8af563c74^`), `ifc-4 io_registry::register`.
- Wrong or missing mounts/aliases: `field_meta` mounted from the test file instead of the real file in en1990/en1993/en1995/din4108 editors; wrong `extern crate ... as schema/interaction/job` aliases (results-config, cad-kit, draw tests); `fixture_corpus_tests` `framework_schema` alias un-gated; 28+ exact duplicate imports removed.
- Dependencies: about 620 dependency/feature/alias lines added by `fixdeps` (every addition cycle-checked; `cyclesRefused: 0`) plus hand-added features and dev-dependencies listed above. `Cargo.lock` files need a refresh by the dependency sync.

Backups of every file at first touch: `C/🗑️generated/rs-sweep3-x/backup/ledger.json`. Reports of the fixdeps rounds: `C/🗑️generated/rs-sweep3-x/fixdeps-report-*.json`.

## Verification (exact commands)

- `bun C/rs-sweep3-tools/main.ts --scope C/🗑️generated/rs-sweep3-run/scope-all.txt --syntax --tag it6` -> 62 findings, 25,975 files, 0 syntax errors.
- `bun C/rs-sweep3-tools/main.ts --scope C/🗑️generated/rs-sweep3-run/scope-all.txt --tag it7` -> 37 findings (23 unresolved-qualifier, 5 unresolved-import, 4 unresolved-path, 4 duplicate-name, 1 undeclared-dependency; the last was fixed by `fixdeps --apply` afterwards and not re-run).
- `bun C/rs-sweep3-tools/fixdeps.ts findings-it6.jsonl --apply`, `... findings-it7.jsonl --apply`.
- `rustfmt +nightly-2026-07-20 --edition 2021 --check --config skip_children=true` over the 27 files newer than `it6`: 0 syntax errors.
- Not run: cargo (any), bun test, `bun repo ...`.

## Open items (37 remaining findings, none a regression I know of)

Checker false positives (macro-generated items, nothing to fix): bim gestures `CursorLeft::DELTA`... (8) and `BimCommand::SelectFindings`/`SetClassification` (2, macro-generated command structs); layout-model `FromValue`/`ToValue` (4, derive macro and trait share a name in different namespaces); sourcing `SourcingModules::from` (2, macro-generated enum).

Real, needing a decision or framework work (not done):
- `fem-analyses-assembly` (10) and `fem-analyses` (1): the tests use `CsrTripletBuild`, `CsrTripletBuildStage`, `SparseTriplet`, `AssemblyOrder`, which exist nowhere in the tree.
- `assert_registered_presence_round_trip` / `assert_presence_round_trip` (fem2d, fem3d, puzzle2d editor tests, 3): the framework law helper no longer exists.
- layout export unit tests (2): `semio_framework::ToolJobDispatch` no longer exists.
- pptx `outline::tests::fixture` (2): a `cfg(test)` fixture used from the model-laws target; needs its own fixture in that test crate.
- gltf-model-snapshot `io::text::snapshot` self-referential re-export (2).
- `bim_window_config!` expansion inside the bim kit `window_config!` macro still writes `crate::bim_window_config!` (not flagged by the checker because it is inside a macro); verify at the first cargo run.

Other notes: the original `rs-sweep-tools` rustfmt syntax gate used `disable_all_formatting=true` and silently missed syntax errors; both tool copies now use `skip_children=true` only (re-run found and fixed 3 real syntax errors early on). `Cargo.lock` refresh, and the first real cargo run over the window crates, are the next verification steps.
