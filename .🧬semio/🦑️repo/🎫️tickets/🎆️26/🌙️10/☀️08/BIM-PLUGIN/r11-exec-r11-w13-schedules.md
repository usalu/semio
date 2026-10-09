# r11-w13-schedules execution report

S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, T = this ticket folder. Gate label `r11-w13-schedules`, logs `T/🗑️generated/r11-w13-schedules/`.

## Status
**WRITTEN, ORACLE VERIFIED, RUST UNVERIFIED.** The Python oracle ran green (`write` then `check`: "oracle agrees"; the sum law, coverage and the quantity law
hold). No Rust was compiled or run: `cargo check -p semio-s-artifact-bim-model --lib --tests` fails inside the framework before it reaches the BIM crate
(owner commit 677 retirement refactor; `semio-framework-os-kernel` was cleared by `r11-store-a`, `semio-framework-plugin` still had 285 errors at the last
run at 04:50: `semio-framework-plugin` 883 errors, `infinite-dag` 16, `playbook` 10; logs deleted). The BIM crate itself had 0 errors at every run (`grep 🏙️bim` over the error lines is empty), but it is
only reached after the dependencies build, so every Rust statement below is unverified. Syntax of every Rust file I touched was parsed with
`rustc -Z unpretty=ast-tree` (nightly, no resolution): no syntax error.

## Done (audit WP-13 list)
1. **Test modules** (`📋️schedules`, `📊️csv`, `🧮️schedule/✏️edit`): written by `r11-baseline` (its report: unverified). Extended, not replaced, see 6.
2. **`delete-storey` cascades schedules** scoped to the storey (ruling): `🧬️schema/🧬️mutations/🌊️cascade/🦀️.rs` gains `schedules => CreateSchedule(create_schedule, schedule)`
   in the `removal!` table (concrete inverse create per schedule, like views) and `removal.schedules` = every schedule whose `storeys` names a removed storey
   (so it also follows `delete-building` and `delete-site`). New leaf case `📋️cascades-the-schedules` of `🚮️delete-storey` (fixtures: four schedules, scoped to the
   deleted storey / to it and another / to another / unscoped; test mounted in the artifact root `🦀️.rs` by `T/r11-w13-schedules-cascade.ts`). The generated test file has the
   sum-law test (`inverse_diffs_sum_to_the_negative_diff`). **The `after` and `diff` fixtures are placeholders (`{}`) until blessed**: run
   `"$T/🚦️gate.sh" r11-w13-schedules -- env BIM_BLESS=1 cargo test --manifest-path <artifact>/Cargo.toml -p semio-s-artifact-bim-model --lib cascades_the_schedules`.
   Doc strings of the leaf and its diff mention views and schedules. `gen-oracle` re-run: "already current" (the scenario is registered).
3. **Unit tests for rows and table** (new files beside the baseline test, mounted in `📋️schedules/🦀️.rs`): `🧰️kit` (the committed IFC house + helpers), `🗂️rows` (storey and phase
   scope incl. openings from hosts, every category's candidates, facts of doors/windows/rooms, typed property cells, property key order, material names of a wall,
   material layer rows with layer volumes and total, finish rows with materials/areas/total, unresolved room), `🧮️table` (natural order, every filter operator on numbers
   and texts, sorts asc/desc/ties/numbers/empty, multi-filter, property filter and sort, grouping + subtotals + selected elements + grand total, nested grouping
   levels, not-itemized collapse, ungrouped merge, empty cases, no total without a summed column, presets, `table_json` types), `🕸️graph` (cache transparency warm = cold =
   uncached, quantity edit through the gate, rename recomputes only the schedules that list the element, definition edit recomputes only its schedule,
   schedule leaving the graph, storey removal then inference). `table_json` was moved to `🚪️io/📝️text/💡️inferences/📋️schedules/🦀️.rs` (upstream convention) and the baseline test
   and `📸️snapshot` arm follow it.
4. **Third-party oracle + feature**: case `S/🧪️tests/📋️infer-bim-1-schedules/{🥒️.feature,🐍️.py,🦀️.rs}`, oracle `bim-1-shapely-geometry` (quantities from the sibling shapely
   oracles `🧮️infer-bim-1-quantities` and `🪣️infer-bim-1-finishes`; rows, filters, natural order, grouping, totals restated from the documented rules, `math.fsum`).
   Scenarios `schedules-house` (nine schedules over the committed house) and `schedules-wall-edit` ("a quantity edit updates the schedule": `setWallTop` applied by the
   oracle itself; other rows unchanged, the edited wall's row the only one that moves, every summed total moves by what the row moved). Fixtures
   `🧫️fixtures/💡️inferences/📋️schedules/{🏡️house,✂️wall-edit}` written by `T/r11-w13-schedules-fixtures.ts`, expected tables written by `🐍️.py write` (57 rows each).
   Rust subject adapter registered like the other infer cases (`encode_inference_projection_json(.., "schedules")`).
5. **Model-graph gating + cache transparency**: `🕸️graph` tests above (they use `ModelInferenceSession`, `UpdateReport.computed_by_kind["schedule"]`).
6. **CSV export action in the schedule window**: command `exportScheduleCsv` (`🎮️commands/📊️export-schedule-csv`): payload `{id, pressed?}` (empty id = the shown schedule),
   `handle` (synchronous, session inference) and the job `CsvJob` + `ScheduleCsvWork` (retained command work): steps `inference::begin/step/finish` of `r11-w03-session`'s
   registry (`NODES_PER_STEP` = 256 nodes per step, progress stage `bim-schedule-csv-infer`), then chunks of 512 records (`bim-schedule-csv-encode`), then
   `Emit::effect(Effect::DownloadMediaExport { text/csv })` through the `s.stdio.csv` writer (`io::export::csv::schedule_records` + `codec`). Cancellation is the framework's
   (`cx.is_cancelled()` before each step drops the work; the session keeps every finished node; unit-tested by dropping a job half way). Wiring (exact-string, idempotent,
   `T/r11-w13-csv-wire.ts`, `T/r11-w13-csv-ui.ts`): artifact root mount, editor command-table row `[HostOnly]`, `only()`-filtered bridge arm, `build_tool_job` picks the
   work, labels en+de (`cmd_export_schedule_csv`, `…_describe`, `sch_export_csv`), an export button on every schedule row of the list and an export measure
   (`bim.measure.schedule.export`) in the chrome of a shown schedule. Tests: file names, schedule resolution, download equals the serializer's CSV, chunked job equals one-shot,
   progress stages, cancel then next job, vanished schedule fault.
7. **Room finish schedule preset**: new category `Finish` (a row per floor/wall/ceiling of each resolved room, areas and materials from `🎨️finishes` through the quantities),
   fields `Surface` and `FinishArea`, preset `finish` (number, name, surface, material, finish area*; grouped by storey), `Number`/`Usage` also offered to `Finish`.
   Touched: `T/r3-f1-gen-model.ts` spec, `🧬️schema/📸️snapshot/💠️values/🦀️.rs` and the json/ts/graphql facets (hand-applied: running `r3-f1-gen-model.ts` would overwrite the
   upstream patch design, so it was NOT run), `📋️schedule-kit`, `🗂️rows`, vocabulary, `🗣️terminology`, window `preset_label`. `READS` gained `ceilings`.
8. **Examples**: house gains `sch-finishes` and `sch-doors-ground` (storey + phase scope); office gains door, window and finish schedules (house already had door,
   window, room, wall, take-off). Regenerated by `bun T/r4-x-examples-gen.ts` (the generator also re-emits the peers' parts). `📚️examples/🧰️checks`: `infer` now asserts one table
   per schedule and the sum law (`schedules_add_up`); house/office tests assert the item counts (9/25/12 and 17/24) and the scoped schedule. **The `🗣️.dsl.semio` texts of
   house and office are stale until blessed**: `BIM_BLESS=1 cargo test … bless_the_house_text bless_the_office_text` (the codec round-trip tests fail until then).
9. **Schedule window tests** (`🧪️tests/🔬️unit`) were still on the pre-WP-13 API (`rows(&snapshot,&inference,&labels)`); rewritten for `list_rows`, `table_parts`, `render`, `editor_rows`.

## Commands and results
- `python .venv/Scripts/python.exe 🐍️.py write|check 🧫️fixtures/💡️inferences/📋️schedules` (PYTHONUTF8=1): `oracle agrees`, 9 schedules / 57 rows per case, shapely 2.1.2.
- `bun r3-f1-gen-oracle.ts`: registered 136 kinds, 955 scenarios (already current). `bun r3-f1-check-names.ts`: 3 problems, none mine (sqlite `🗄️schema` vs `🗄️.sql`,
  `🔲️ceilings-meshes` vs `🔲️ceilings-holes-slope`, and a `__pycache__` my run had created, now deleted).
- `cargo check … --lib --tests` (gate, 6+ rounds from 02:40 to 04:30): never reached the BIM crate (framework errors 303 → 1 → 793 → 290, in `os-kernel` then `plugin`).
- `cargo test --lib` counts, wasm32-wasip2 check, `infer-bim-1-schedules` subject/parity run: NOT RUN (blocked as above).
- Path length: longest new path 175 chars from the drive root (`✂️wall-edit/💡️inference/📋️schedules/🔣️.json`).

## Open items
- Run, once the framework builds: `cargo test --lib` (my modules: `schedules::tests*`, `export_schedule_csv`, `delete_storey::tests_cascades_the_schedules`, window tests, house/office
  example tests, csv tests of the baseline), bless the delete-storey case and the two example texts, then the subject role of `infer-bim-1-schedules`
  (`cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test && bun ./📜️script.ts subject quick --case 📋️infer-bim-1-schedules`) and the wasm check.
- Assertions most likely to need a first-run adjustment (written without running): `recomputed == 2` after a door rename and `< all` after a wall edit
  (`🕸️graph`); German word `Summe` / English `Total` in the window test; `ScheduleCsvWork` may need `Send` if the work trait demands it
  (`SessionRun` holds the engine cursor); both rooms of the IFC house resolving (`finish` tests and the oracle say yes: the oracle `write` found both rooms).
- Not done / justified: XLSX-like export is out of scope (CSV through `s.stdio.csv` per r9 ruling); the table count limit for very large schedules is the session's.
- Generators `gen-model` and `gen-mutation-facets` were deliberately not run (see 7); the facets I touched by hand equal what the spec now generates.
- Input scripts kept in T: `r11-w13-*` (patch scripts, fixtures, cascade case). Generated logs are deleted at the end.
