# r11-w02-diagnostics execution report (WP-02 surface diagnostics + u-fix leftovers)

Status: **WRITTEN BUT UNVERIFIED for every Rust change**; the python oracle logic and the gesture-cases oracle ran.
Cause: from 01:06 to 05:20 `cargo check -p semio-s-artifact-bim-model --lib` never reached the BIM crate. The dependency graph fails in the owner's half-finished
retirement refactor (commit 677). Progress while I waited: 455 errors at 03:16 (all in `🏪️store`), the store and os-kernel libs compile at 05:20, and the last
errors are **two framework artifact crates nobody owns**, which still use the 676 retirement API:
`🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🦀️.rs` (about 10 errors: `store::SnapshotRetirementStep`, `owned_retirement`, `close_step` arity) and
`…/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs` (about 17 errors, same causes). `r11-store` never wrote `T/r11-exec-store.md`. Log of the last attempt: `T/🗑️generated/r11-w02-diagnostics/check12.txt`.
**Until those two files are ported no BIM agent can compile or test anything.**

S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, G = `S/🧬️schema/💡️inferences`, E = `S/✏️editor`, T = this folder.

## Before → after

### WP-02
| Item | Before | After |
|---|---|---|
| Index in the model graph | `DiagnosticIndex` present, not wired | new node kind `DiagnosticIndex` (`G/🕸️model-graph/🦀️.rs`: `NodeKind`, `ModelNode::DiagnosticIndex`, `Data::Index`, `kinds::DIAGNOSTIC_INDEX`, `requires = [Diagnostics]`); `🧭️plan`: parents are exactly all `Diagnostics` nodes; `🧮️compute`: dependency `Null` (it reads no snapshot row, honest like `Totals`), value `DiagnosticIndex::of(ordered findings)`; `🪞️projection` apply/retract; `🎯️dirty::touched` answers `false`; `ModelInference.diagnostic_index` (`#[derived]`) and its `InferenceFieldSpec` (`…diagnostic-index`, reads = diagnostics reads) |
| Index tests | one unit test of the pure index | `G/⚠️diagnostics/🗂️index/🧪️tests/🔬️unit/🦀️.rs` `mod graph`: projection equals `DiagnosticIndex::of(diagnostics)` for clean/defects/empty; parents are exactly the findings nodes and the dependency is the same for any snapshot; a selection plans only findings + ancestors and projects the index only when wanted; cache transparency (cold = gated = warm = uncached, `diagnostic-index` computed once); fixing a defect updates the index = a fresh inference, recomputed at most once |
| Panel `📌️panels/🚨️diagnostics` | missing | `E/📌️panels/🚨️diagnostics/🦀️.rs` (+ 11 unit tests): severity group (errors, warnings, notes) → storey by level (unknown after known, whole model last) → kind (domain of the code) → finding row (message in the locale of the view, `Severity · element names`, icon, click = `selectFindings`). Severity filter = the open state of the three severity sections (host-owned, notes closed by default, kinds with more than 12 findings closed): a panel has no config lane, so no new state was invented. Accessible name + description (`render_body`/`accessible_surface`), en+de. `groups`, `groups_json`, `groups_json_of` are pure and shared with the feature adapter |
| Click selects in plan + outliner | – | command `selectFindings` (`E/🎮️commands/🎯️select-findings`, 3 unit tests): `select_effect` into the framework `elements` domain, which plan, 3D and outliner share; ids no collection holds are skipped, none left → fault `bim.diagnostic.target-missing` |
| Registered in mount + command table | – | `M/🦀️.rs` (`panels::diagnostics`, `commands::select_findings`), editor `🦀️.rs`: command row, bridge arm, action args, `panel_tab_def`, body routing, accessible surface, fault notice |
| Chrome status count | – | `🎛️chrome`: `problems(labels, counts)` ("Errors: 2 · Notes: 5" / "No problems") as a second status line `<window>.problems` of every addressed window, fed by `problems_of(doc)` (the diagnostic index of the instance's registry session); chrome tests updated (+2) |
| Labels en+de | – | 56 new `app_labels!` rows: panel, description, severities, model-wide, empty, 12 category labels, status, 2×3 command labels, 1+3 faults, 3 classification args, 18 cursor command labels |
| JSON export | – | `S/🚪️io/📤️export/🧾️json` (`ModelIntoJson`, dialect `s.stdio.json@rfc8259/*`, `IoFidelity::Lossy`, registered in `io()`), values typed `JsonValue`, text written by the stdio json artifact (`codec` = the one place naming it). Document: counts, findings (severity, code, category, storey, elements, missing, exact numbers, messages en+de), index by element and storey. `adjudicated_json` reads it back into the oracle table. Cargo: `semio-s-artifact-stdio-json` workspace + package dependency. 5 unit tests + `BIM_BLESS` test for the committed file |
| CSV | export + unit tests (r11-baseline) | `export_diagnostics`, `findings_json`; 2 more tests (record per finding of the defect house, committed file current) |
| `.feature` | clean/defects only | `🗺️infer-bim-1-plan-and-diagnostics`: `diagnostics-export-json` (json module of python reads the committed export; its measures must equal the **shapely** table), `diagnostics-export-csv` (csv module of python, one record per finding, every shapely finding present), `diagnostics-panel-groups` (plain dictionaries regroup the exported findings by severity, storey level, kind). Oracle `🐍️.py` handlers + Rust adapter subjects registered |
| Facets | – | `T/r11-w02-diagnostics-facets.py` (idempotent; ran): `SeverityCounts`, `ElementFindings`, `DiagnosticIndex`, `ModelInference.diagnostic_index` in `G/🔣️.json`, `🟦️.ts`, `🔗️.graphql`, `🛰️.proto` and `⚠️diagnostics/🟦️.ts` |

### u-fix
| r9 item | Result |
|---|---|
| 8 classification commands | `setClassification` (`🎮️commands/🗂️set-classification`) and `removeClassification` (`🗄️remove-classification`): typed ids/system/code/title, selection fallback, skip unchanged, refuse invalid/none; 3+2 unit tests, mounted-app test (set, remove, undo, undo); bridge, args, faults |
| 11 arrow-key cursor | `🎮️commands/🧭️cursor-keys`: 9 commands (`mod+arrow` 0.1 m, `mod+shift+arrow` 1 m, `mod+enter` clicks), `ToolSession::{nudge, place, cursor}`, `gestures::run_cursor` (start = last pointer, else gesture anchor, else view centre). `alt+arrow` is taken by `storeyUp/Down`, hence `mod`. Descriptions of plan and section surfaces name the keys (en+de). Tests: oracle replay of 3 `cursor` cases (numpy sums), wall drawn with the cursor alone, rubber band, mounted-app wall drawn from arrows with undo |
| 12 accessible names | already present in `render_body`/`accessible_surface` (canvas, world, schedule, panels); added the diagnostics panel; completeness test extended |
| 18 schedule window vs panel | ruling applied: **stays a window**; nothing moved (r2 design §6 should read "window") |
| 19 hard-coded `"wall"` | world: scene fallback granularity = kind of the first visible solid, instances without a kind carry no override (no `map_or("wall")`); schedule: `row_kind` = kind of the elements the item rows stand for, none for a material take-off (no pick); test added |
| 20 `📌️.empty.md` | **kept, justified**: all 16 are taxonomy-required facet markers (`🔣️taxonomy.json`: `surfaceRequiredChildDirs` = modes, commands, config, presence, transient; `modeRequiredChildDirs` = windows, commands, config, presence, transient; `windowRequiredChildDirs` = actions, utilities, options, config, presence, transient). Note the inconsistency: only the world window carries its 3, plan/section/schedule carry none and `options` is missing everywhere, so the gate evidently does not enforce window facets; removing the 3 world ones is safe but not asked by the taxonomy |
| 21 runtime verification | NOT DONE (no build) |

## Tests and oracles
| Command | Result |
|---|---|
| `cargo check/test …bim-model --lib`, wasm32-wasip2 check | NOT RUN: framework blocker above |
| `.venv/Scripts/python.exe S/🧪️tests/🛠️gestures-bim-1/🐍️.py write` then `check` on `S/🧫️fixtures/🛠️gestures/🔣️.json` | wrote 3 `cursor` cases (+52 lines, nothing else changed); `ok: 42 cases agree with numpy 2.5.0 and shapely` |
| `🗺️infer-bim-1-plan-and-diagnostics/🐍️.py` | compiles; the three new python functions (`export_json_table`, `export_csv_table`, `panel_group_table`) ran on hand-made documents and gave the expected tables; the harness cannot run the new scenarios until the committed exports exist |
| `bun T/r3-f1-check-names.ts` | my new names are clean; 2 foreign problems (`🚪️io/🪶️sqlite/📸️snapshot/🗄️schema` vs `🗄️.sql`; `🧊️element-solids/🔲️ceilings-meshes` vs `ceilings-holes-slope`) |
| path lengths | longest path under `🏙️bim` is 195 characters |
| mutation generators (`r3-f1-gen-mutation-facets/oracle/feature`) | not re-run: no mutation leaf was added or changed by me |

## To run when the framework compiles (in this order)
1. `"$T/🚦️gate.sh" r11-w02-diagnostics -- cargo test --manifest-path S/../Cargo.toml -p semio-s-artifact-bim-model --lib` (fix compile slips first; I could not check any of my Rust).
2. `"$T/🚦️gate.sh" … -- env BIM_BLESS=1 cargo test … --lib the_committed_json_file_is_the_current_export_of_the_defect_house the_committed_csv_file_is_the_current_export_of_the_defect_house` writes `S/🧫️fixtures/💡️inferences/⚠️diagnostics/💥️defects/📤️export/⚠️diagnostics.{json,csv}` (the directory does not exist yet; the two tests create it).
3. `cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test && SEMIO_TEST_LEVEL=quick bun ./📜️script.ts oracle quick --case 🗺️infer-bim-1-plan-and-diagnostics` (needs step 2) and `--case 🛠️gestures-bim-1`.
4. `cargo check … --target wasm32-wasip2 --lib`.

## Open items and risks
- Likely first compile slips: `Component::TreeItem` icon patch in the panel (same pattern as `kit::tree_item_with_icon`), `ui_node_list([empty_row(labels)])`, `BTreeMap<&str, …>` keys in `groups`, the `semio_s_artifact_stdio_json::schema::snapshot` path and `TextError: Display` in the json codec, the new Cargo dependency (lock update, wasm build of the json artifact), `Feed::Cursor` arm in `settle`.
- Peer interplay: `w03-session` deleted the editor's own inference table (my `fields!` row is gone with it; the graph projection carries the index). `every_command()` in `E/🧪️tests/🔬️unit/🦀️.rs` still lacks peers' commands (`ArmCeiling`, annotation arms, …): my 12 are added, theirs belong to their finishers.
- Chrome tests: `E/🎛️chrome/🧪️tests` still expect `bim.measure.plan.storey` while the chrome offers `bim.measure.plan.view` (w12 views); not mine.
- The editor `[DEBUG]` timer in `🧮️compute` belongs to z-incremental; none of my code carries `[DEBUG]` lines.
- Files I created in `T`: `r11-w02-diagnostics-*.py` (one-shot patch scripts, kept as inputs; the facet one is the reusable one).

## Update 05:45 (after `r11-exec-store.md` appeared)
`semio-framework-plugin` (with dag and playbook) now compiles. The next blockers, again not mine: `🌊️flow` (`🧩️extensions/🕸️wasm/🦀️.rs`, `🖥️host/🦀️.rs`, `🌿️vcs/🦀️.rs`: `neural_engine::ValueRetirementStep`, `os_store::SnapshotRetirementStep`, `semio_framework_value::SnapshotRetirementStep`) and a syntax error in
`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/…/🎬️media-export/🦀️.rs:196` (`expected where/{/( after struct name`, a peer edit in progress). Log: `T/🗑️generated/r11-w02-diagnostics/check13.txt`. My code is still not reached by the compiler.
