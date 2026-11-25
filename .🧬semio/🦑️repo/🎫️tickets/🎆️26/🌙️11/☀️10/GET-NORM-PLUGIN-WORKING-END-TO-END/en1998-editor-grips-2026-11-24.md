# en1998 editor compile, grips (en1998, en1997), test-support gaps (2026-11-24)

Norm root: `✏️s/🔌️plugins/📕️norm/`. Cargo test lines: `repo cargo --store=shared --configuration=prototype -- test -p <pkg> --offline`.

## 1. en1998 editor did not compile

### Cause
Same as en1999, one level deeper. The dispatch crate `🧬️mutations/📦️packages/🦀️rust` forwards `test-support` to every `mut-*` crate it depends on (13 of 13, nothing missing). But `mut-members` -> `mut-members-joints` -> `mut-members-joints-more` -> `mut-members-joints-rest`, and `mut-bridges-towers` -> `mut-bridges-pier` -> `mut-bridges-pier-more`, ended in two leaf crates that had no `test-support` feature at all, so `InsertJoint`, `RemoveJoint`, `InsertTieBeam`, `RemoveTieBeam`, `InsertSeismicSlope`, `RemoveSeismicSlope`, `ChangeTowerEiNm2`, `ChangeBridgeDeckEiNm2`, ... had no serde derives (`cfg_attr(any(test, feature = "test-support"), derive(serde::..))`).

### Fix
- New `[features] test-support = ["dep:serde", diff/test-support, model/test-support]` in `mut-members-joints-rest` and `mut-bridges-pier-more` (same line as every sibling leaf crate).
- Forwarded from their parents: `mut-members-joints-more` -> `mut-members-joints-rest/test-support`, `mut-bridges-pier` -> `mut-bridges-pier-more/test-support`.

## 5. Test-support forwarding gaps in other families
Script: for every crate with a `test-support` feature, each artifact dependency that has the feature (or an optional serde) must be forwarded; run over all families.
- en1998: the two above.
- **en1991** dispatch (`🧬️mutations/📦️packages/🦀️rust/Cargo.toml`): `mut-dust-explosions`, `mut-fatigue-models`, `mut-local-fires`, `mut-rail-derailments`, `mut-rail-load-groups`, `mut-snow-guards`, `mut-support-collisions` were not forwarded. Added.
- **iso16757**: `mut-definitions-more` had no feature, `mut-definitions` did not forward. Feature added and forwarded.
- **en1993**: `mut-buckling-lengths`, `mut-fastened-joints`, `mut-fatigue-stresses`, `mut-forming-gains`, `mut-material-ductility` are not forwarded by `en1993-mutations`. **Not edited** (other agent is splitting the en1993 crates). Needs the same one-line addition.
- All other families (en1990, en1992, en1994, en1995, en1996, en1997, en1999, din*, vdi3805): clean.
- coordinator follow-up, en1992: `En1992MembersOpeningsRows` (`🧬️schema/🔺️diff/🦀️.rs`) was the only `list_delta!` without the two `cfg_attr(... test-support ...)` serde attributes; added (a script found no other `list_delta!` without them).
- coordinator follow-up, din18599: `crate::document::child_identity_oracle` does not exist; the module is `semio_s_plugin_norm_compliance::child_identity_oracle` (`⚖️compliance/🦀️.rs:50`, behind the `compliance-testing` feature). Both copies of the path fixed (`🧬️schema/🦀️.rs:61`, `🧬️schema/📸️snapshot/🦀️.rs:65`; the snapshot file is not generated, it is hand-kept) and the features wired: model `test-support` now enables `semio-s-plugin-norm-compliance/compliance-testing`; the schema crate got `semio-s-plugin-norm-compliance` as optional dependency enabled by its `test-support`.

## 2. en1998 editor tests

### Grips (en1998 own `handle()` in `🎬️scene/🕹️handles/🦀️.rs`)
The grip law (`every_grip_of_every_entity_kind_...`) failed at `bldg-office:s2:h`: map relative to the value, read at the absolute rest parameter (value + rest = 8.0). `handle()` now returns `with_rest_in_limits(rebased(handle))` (shared kit). The `qk` `load_arrow` grips were already absolute. Scene unit tests that read "at zero" now read at the rest parameter (`AtRest::writes_at_rest`, uses `rest_of`).

### NDP read leaks (incremental laws)
`the_ordered_fold_of_units_equals_the_full_evaluation_exactly`, `the_retained_session_...`, `every_unit_declares_all_it_reads` failed because a check cites every registry read since the last drain: reads of a previous row moved into the next check (diagnosed with a temporary test: first differing check `en1998.1.bldg-office.sys-x.overturning`, ndp lists differ). Fix as in en1999: every row starts with an empty log (`isolate()` in `🧬️schema/💡️inferences/⚖️part-evaluators/🦀️.rs`, called for every row of every collection loop and for the buildings and cases loops of `🌍️full-seismic-check/🦀️.rs`).

### Other en1998 content fixes
- Field specs: 42 bound paths had `Inferred` specs (`m!(en, de)`); typed now in `✏️editor/🏷️field-meta/🦀️.rs` (units, new `ratio` and `int` macro arms; macro arm order fixed).
- Diagram `<collection>-checks:<id>` with a single check had a one-point polyline (`at-least-two`): a scatter series for one check (`🎬️scene/📈️diagrams/🦀️.rs`); collection tokens now kebab-case (`retaining-walls-checks:` / `tie-beams-checks:`, `<kind>:<row>` law).
- Remedy law (18 findings -> 4): `regularity.claim` offers a remedy only when it alone clears the check; `baseShear` offers the permanent-load remedy only when it lowers the demand (target = current minus the excess mass); `torsion` offers the plan-regularity remedy when only the regularity flag fails (the eccentricity ratio never entered the amplification); `planRegularity` offers the centre-of-mass remedy only with a positive radius.
- Tower base spring support: one stiffness component instead of six (`six-components` scene rule), `🏭️vessels-and-towers/🦀️.rs`.
- en1998 scene unit tests: 3 compile errors (`Diagram2d` path, `!m.material ==`, `AnalysisDirection` from `&str`).

### Test lines
- editor lib (run before the diagram, remedy and tower fixes of the last edits): `test result: FAILED. 30 passed; 2 failed` (before: `25 passed; 7 failed`; the grip law, the 3 incremental laws and the touch test pass).
- editor integration: `test result: FAILED. 47 passed; 5 failed` (before: `45 passed; 7 failed`).
- Not re-run after the last en1998 edits (compiler queue): the 2 integration failures expected to be fixed (`retainingWalls-checks` diagram id, remedy law down to the `t1Rayleigh` rows).
- scene (`semio-s-artifact-norm-en1998-scene`): did not compile; now `test result: FAILED. 50 passed; 26 failed` (first run after compiling: `41 passed; 35 failed`). The grip-at-rest tests pass. `semio-s-artifact-norm-en1998-scene-structures`: `test result: ok. 0 passed` (no tests).

### Still failing, not fixed
En1998 content (scene crate, 26): entities that the tests look up by name are not projected (`column region`, `isolation support`, `pier base support`, `ratio diagram`, `as-1`, `t1-gross` marker), `layer-exists` (an entity uses a layer that `LAYERS` in `🎬️scene/🧰️kit/🦀️.rs` does not list; not located), drag budget `scene-drag p95 2490 ms > 240 ms` (debug build), `golden_scene_matches_committed_bytes` (`🧪️tests/🧫️fixtures/🏛️scene/🔣️.json` was empty; the harness wrote it, review it), `every_numeric_field_of_placements_..._has_a_grip` (snapshot schema file not found), a few value mismatches of the assessed-member/overturning/spectrum-surface projections.
Editor (en1998 content): remedy law `t1Rayleigh` (4 rows): the check only exists while the method is rayleigh and the input is bad, so every remedy makes it vanish; needs an always-emitted row (changes check counts and the oracle) - decision.
Editor `live_acceptance` x2 (`every_registered_entity_kind_deletes...`, `..._supports_duplicate_copy_paste_and_bulk_edit...`): the default document has no `bridges` row; `insert_edit` inserts `value: None` into an empty collection (`📇️registry/🧬️contract/🖥️app-surface/🕹️direct-manipulation/➕insert-tool/🦀️.rs:48`) and `apply_to_tree` stores a Null row (`🧰️entity-editing/🦀️.rs:53`), which fails the test at `🧪️tests/🔬️live-acceptance/🦀️.rs:262` and `:312`. Framework-owned (or a default document with a row of every kind).
Framework-owned (history, undo, close), not touched:
- `undo_redo_round_trips_through_the_wrapper`: `WorkLimit ... original fixture settlement stopped after 4098 turns` (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:50`).
- `history_edits_end_to_end`: `original report requires its installed semantic and retirement issuers` (`semio-plugin-testing-acceptance/🎛️acceptance-policy-pump-and-drive/🦀️.rs:117`).
- `conflict_history_edits_end_to_end`: `the archive load did not settle` (`semio-plugin-testing-acceptance/⚔️cascade-withdraw-edit-and-conflict-resolution/🦀️.rs:257`).

## 4. en1997 grips
`Grip::build` (`🌍️en1997/.../🎬️scene/🕹️handles/🦀️.rs`) now builds the relative handle with the unwidened limits and returns `with_rest_in_limits(rebased(handle))`; the module doc is updated. The value at the rest parameter is the current value. Radial, FaceOffset, Translate and CountStepper kinds have no rest parameter and are unchanged (the circle radius test `moved.map.base + k * 1.0` still holds).
**Not run**: the en1997 editor lib and scene tests were queued behind the shared compiler and never reached.

## Coordinator follow-up (en1992, din18599)
Fixes above. `semio-s-artifact-norm-en1992-editor` was started (`--no-fail-fast`, includes `history_edit_inputs_resolve`) and was still waiting on the compiler image when this note was written; din18599 editor not started. en1992 compiles now (the diff serde fix works). Its editor lib run printed no `test result` line (probably the release-only `latency_bench` hanging in a debug build, as in en1999); failures seen in the lib: `incremental::tests::the_retained_session_...`, `incremental::tests::every_unit_declares_all_it_reads` (the NDP read leak pattern of section 2, not fixed for en1992), `incremental::ownership::every_check_of_every_demo_document_...`, `live_acceptance::a_click_on_a_check_row_routes_...`. `history_edit_inputs_resolve` is an integration test and was not reached. **din18599: not run.**

## Changed files
- Mutation features: `🫨️en1998/.../🧬️mutations/{🔩️members-joints-rest,🗼️bridges-pier-more,🔩️members-joints-more,🗼️bridges-pier}/📦️packages/🦀️rust/Cargo.toml`; `🏋️en1991/.../🧬️mutations/📦️packages/🦀️rust/Cargo.toml`; `📇️iso16757/.../🧬️mutations/{📐️definitions,📐️definitions-more}/📦️packages/🦀️rust/Cargo.toml`.
- en1992 diff `🧬️schema/🔺️diff/🦀️.rs`; din18599 `🧬️schema/🦀️.rs`, `🧬️schema/📸️snapshot/🦀️.rs`, both `Cargo.toml` of model and schema.
- en1998 engine: `💡️inferences/⚖️part-evaluators/🦀️.rs`, `🌍️full-seismic-check/🦀️.rs`, `.../🏢️building-regularity-and-base-shear-checks/🦀️.rs`, `.../🧱️system-checks/🧭️direction-and-base-shear/🦀️.rs`, `.../🧱️system-checks/🏬️storey-forces-and-drift/🦀️.rs`.
- en1998 scene: `🕹️handles`, `📈️diagrams`, `🏭️vessels-and-towers`, tests `🧪️tests/🔬️unit/🦀️.rs`, `🧪️tests/🔬️live/🦀️.rs`. Editor: `🏷️field-meta/🦀️.rs`.
- en1997 scene `🕹️handles/🦀️.rs`.
- Temporary tests (regen and diagnostic) were removed; no `[DEBUG]` left. Logs in `🗑️generated` of this task.

---

# Round 2 (coordinator decisions): t1Rayleigh, scene failures, golden, en1992

## t1Rayleigh emitted on every evaluation
`en1998.1.<b>.t1Rayleigh` is now one row per building: not applicable when the method is not rayleigh, pass (u 0) when the storey stiffnesses exist, fail with the method remedy when they do not (`🌍️full-seismic-check/🏢️building-regularity-and-base-shear-checks/🦀️.rs`). The independent Python oracle (`🔮️oracles/🚦️check-outcome/🐍️.py`) emits the same three outcomes. No expected check count is written down anywhere (grep of rust, feature, json and python files found none), so no count changed in a test.

## en1998 scene (`semio-s-artifact-norm-en1998-scene`): 26 failures -> 2 at the last full run (74 passed; 2 failed)
Causes and fixes (projection gaps first, no lookup renamed except where the test itself was wrong):
- **Grips read at rest**: the en1998 `handle()` snapped the absolute parameter on a grid in parameter units, so the value at rest was rounded away (stiffness 7.5e9 for 1e10, bar 12.5 for 20, width 0.1 for 0.3). The snap is gone; the step is a `Quantise::Round` of the written value, only when the current value is on the step grid (an untouched grip never rewrites a field).
- **Tower base spring**: one stiffness component instead of six (`six-components`).
- **Fixture gap, showcase**: `full_showcase` had no isolation system, pier base, critical region, nor the liquefiable foundation the tests and the scene code are written against. Rows added to `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`: `iso-1`, `pb-1`, `cr-1` (values taken from the committed mutation fixtures), `fd-3` with layer `l7` (n1_60 6, fines 5 %). `bldg-pending` renamed `Annex B` (the German scene text read as English through the English name); the same rename in the `seismic-showcase` DSL asset.
- **Assessment rows without a section were not drawn** (`en1998.3.as-1.supportedBuilding routes to as-1, which the scene does not draw`; the test finds `as-1` itself): every assessment now has a row entity `as-1` (a small marker carrying the governing binding), the member glyph `as-1:member` stays where a section exists.
- **Hosted rows and order**: assessed members and critical-region glyphs are projected by their own document entity (`hosted_rows`), critical regions are document entities of the scene (`rows_of`), and `project` assembles the whole scene with `project_parts` like en1991 (several rows draw the entities of their children, so sorting by document id, as `project_whole` does, orders them differently from the live projector). This removes the incremental-versus-full mismatches (3 live-law tests).
- **Incremental dependencies** (`entity_dependencies`, 24 misses, drag 2.3 s): every building depended on every other building, and dependencies are transitive. A building now reads no other building; where it stands and its seismic joint to the next building are stamps of its pose (`layout_stamps`), as are the hosted rows. The global part no longer derives the grip index. Drag p50 on 61 buildings: 1.3 s -> 0.17 to 0.27 s (`en1998/scene-drag`, budget 240 ms in a debug build). Under the load of the shared compiler the p95 still exceeds 240 ms (last numbers: p50 272 ms, p95 384 ms); on the quieter run before the last change p50 was 169 ms, p95 257 ms. Not green on a loaded machine; assembly and `finish` over about 40 000 entities in a debug build are the rest of the cost and belong to the framework.
- **Diagram laws**: the bearing `design` marker is `reference:design` when its check does not exist; the pier-base `fixed` marker is `reference:fixed` (test lookup updated, the law demands typed references); the chord-rotation `demand` marker of the moment-curvature diagram carried a check but a reference id; the pile profile series `inertial` collided with the marker of the same id (`inertial-profile`); the cracked and gross T1 markers are always both drawn.
- **Tests that were wrong**: drift-bar test included the `drift-map` heat map; spectrum surface test took the first point as the origin; overturning test took the governing check over all entities instead of the building; the snapshot-schema path went one directory short.
- **Goldens** (`🧪️tests/🧫️fixtures/*.txt`, `🏛️scene/🔣️.json`): deleted and regenerated by the harness (it writes an absent or empty golden and fails the first run). The pre-change files are in `🗑️generated/en1998-golden-before`.

## Golden verification (independent derivation)
`golden-verify-en1998.py` derives from the showcase JSON alone: every document row has an entity, every entity id has a document row as base id, per storey the slab, the drift bar and the force arrow of every system, one drift bar per storey and system, the assessment rows, the region member glyph and hoops, the isolation ghosts, the pier-base footing; no duplicate ids. **Not run against the final golden yet** (the final scene run had not finished writing `full_showcase.txt` when this was written); run it with `python3 golden-verify-en1998.py` after the scene run.

## en1992
- `evaluate_scoped` now starts every row (and the globals) with an empty NDP read log (`isolate()`), the pattern of en1998 and en1999. Not verified by a run: the en1992 editor binary was killed by the coordinator after 2 h 49 min.
- Reruns are chained in `run-en1998-en1992-chain.sh`: build (`--no-run`, no timeout), then each run with a 29 minute hard `timeout`, `--test-threads=4` and `--skip latency_bench`. Logs: `🗑️generated/chain-*.log`.

## Round 2 results so far

### Scene (`semio-s-artifact-norm-en1998-scene`, last run, test binary run directly)
`74 passed; 2 failed`. Failing: `scene_matches_the_numpy_oracle_for_spectra_forces_and_modes` (no Python interpreter: `SEMIO_PYTHON` unset, `repo dependencies ensure` would download packages and was not run) and `a_drag_preview_stays_within_the_budget_on_a_large_document` (p50 544 ms, p95 1402 ms at a load average of 11; 169 / 257 ms on a quiet minute; was 2300 ms). The scene golden was regenerated and then passes (the run after the write passes the byte comparison: it was not in the failure list).
Golden verification: `python3 golden-verify-en1998.py` -> `876 entities, 42 document rows, 0 problems` (ids and counts derived from the showcase JSON; also checks one drift bar per storey and system, regions, isolation ghosts, pier-base footing, no duplicate ids).

### en1998 editor, lib (binary run directly, `--skip latency_bench`)
`26 passed; 5 failed`. All 5 are the empty-collection framework gap, now reading `kind 'bridge': the first row does not resolve: insertItem 'bridges' needs a value`: the default document has no `bridges` row and `insert_edit` (`📇️registry/🧬️contract/🖥️app-surface/🕹️direct-manipulation/➕insert-tool/🦀️.rs:48`) answers an empty collection with `value: None`, which the resolver refuses. Framework-owned; not local (a default document with a row of every palette kind is the other way out and changes every default-document test).
Integration binary: `tests_10::renders_the_family_headline` aborted with a stack overflow when the binary was run directly (default 2 MiB test stacks; under `repo cargo test` the same suite ran 47 passed). The chain now sets `RUST_MIN_STACK=33554432`; the rerun is in `run-en1998-editor-*.log` / `run-en1992-editor-*.log` (`run-*.exits` hold the exit codes) and was still building when this note was written.

### en1992 editor, lib (first direct run, 29 minute timeout, exit 124)
Passed: the ordered-fold law (the NDP read isolation works), 8 live acceptance tests, the case-edit paths. Failed: `incremental::ownership::every_check_of_every_demo_document_has_an_owner_or_is_documented_global_and_a_failure_colours_its_entity` and `live_acceptance::a_click_on_a_check_row_routes_to_the_selection_of_its_owner_entity` (messages lost: the binary was reclaimed from the cache before a rerun, the chain now copies the binaries to `🗑️generated/bin` first). Not finished within 29 minutes: `every_unit_declares_all_it_reads`, `an_edit_re_runs_exactly_the_units_whose_declared_reads_changed`, `the_retained_session_equals_the_full_evaluation_after_random_edits`, `every_bound_diagram_marker_commits_...`. The first three perturb every leaf of every demo document and rerun a unit per leaf, so their cost is documents x leaves x unit cost in a debug build; the CPU of the killed run is consistent with a slow quadratic law, not with a settlement loop (no test stopped making progress: the passing ones finished one by one for 25 minutes). Whether `every_bound_diagram_marker_commits_...` is a real hang is not decided; run it alone from `🗑️generated/bin` with a 10 minute timeout.
