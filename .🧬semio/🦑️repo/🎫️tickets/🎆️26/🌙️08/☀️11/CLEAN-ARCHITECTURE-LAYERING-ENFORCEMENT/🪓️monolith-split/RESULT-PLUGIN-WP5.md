# ⏪️ Result PLUGIN-WP5: features package (tool-run, time-travel, time-travel-panel) staged, not flipped

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Plan: `PLAN-PLUGIN-CRATES.md` (12.4, 12.5, Appendices H to K), on top of `RESULT-PLUGIN-WP0.md` and the staged `RESULT-PLUGIN-WP3.md` / `RESULT-PLUGIN-WP4.md` (`RESULT-PLUGIN-WP1.md` and `RESULT-PLUGIN-WP2.md` do not exist yet; the tree shows WP1 flipped, WP2 not). Tools: `pwp5-tools/`. Logs: `🗑️generated/pwp5-*`. `PLUG` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`.

## 0. Verdict

WP5 is **staged, not flipped**, like WP3 and WP4. `tool-run`, `time-travel` and `time-travel-panel` depend on `decl-views`, `decl-emission`, `decl-app`, `child` and `command`, which are still skeletons (WP2 and WP3 have not flipped), so a flip would turn every peer's differential red. What is done is the whole S0 stage: the 3,456-line time-travel file is cut into the three crate shapes, every item move of 5.2 that starts in a WP5 file is applied, the crate graph has **no back edge left that touches a WP5 crate**, and the one-unit equivalence evidence is complete (section 5). The tree is still one compilation unit and resolution-equivalent: the patched full-resolution differential is identical (onlyNew 0, onlyOld 0) in all five configurations.

**No compiler saw any of this.** The launcher failed at both ends with `error: Live Original resource control issuance required` (`pwp5-cargo-1.log`, `pwp5-cargo-2.log`; a new error, in `🏃️process/🧭️routing/📥️invocation`, not a WP5 file). Evidence is static (section 5).

## 1. Decisions and deviations from the plan

| id | what | why |
|---|---|---|
| W5-1 | The lift into `command` is **832 lines / 50 items**, not 1,032 / 56 | WP3 already lifted `ui_history_panel` (218 lines) up into the panel. The items that stay panel-only (`time_travel_band_section` 98, `time_travel_reprojection_section` 47, the editor sections) were pulled down in the plan only because `ui_history_panel` named them. The cut moves the minimal closure the lower crates need: the Driver region, plus `HistoryPanelText` (+ impl, `history_severity_text`, `history_code_text`, `history_severity_tone`, `history_mutation_description`), `MutationRowRefusals`, `history_mutation_entry` (+ `history_outcome_introduced`, `TimeTravelPanel`, `TimeTravelEditorPanel`, `TimeTravelInputRow/List/ListItem`) and `HISTORY_ROW_MUTATION_ROWS`. `history_mutation_entry` must sit in `command` because the instance command routes (`instance-commands`, no edge to panel or time-travel in Appendix H) call it; the other four are what the `history_panel_*_row` helpers that stay in `command` name. |
| W5-2 | `time_travel_arg_generation` stays in **`time-travel`**, not `tool-run` (plan: `command -> tool-run`, 4 lines) | It names `HISTORY_EDIT_ARG_GENERATION`, a history-edit constant; moving it into `tool-run` made `tool-run` know history-edit arguments and need `semio-framework`. In `time-travel` it reads the generic `tool_run_arg_u64` downward, exactly as before, with no new edge. The first cut did put it in `tool-run`; the differential showed one extra external-crate error for the added `use`, I reverted it (snapshot restore + re-cut) and re-ran the differential on the final state. |
| W5-3 | `is_tool_run_action_id` lift goes to the `job` lift stub, not `runtime-ops` | As Appendix J (runtime-ops depends on job). Both `job` and `runtime-ops` named it. |
| W5-4 | `tests-08` and `tests-09` host **no test source** | Measured: every test the WP5 crates own fits its own test closure (time-travel 5 files / 1,665 lines, tool-run 16 / 2,252, panel 3 / 1,404). The only time-travel test over budget, `🧪️tests/🧪️time-travel` (3,394 lines), is mounted by `🧪️testing`, names `VcsArtifactApp` and lives in WP7's tree (plan: `tests-20`, `tests-21`). Both docstrings say so; WP0 may retire the skeletons. |
| W5-5 | Fixed a pre-existing region mismatch in `🛠️tool-machine/🦀️.rs` | `//#region 🔖️Tag` was closed by `//#endregion 🖐️Gesture`, `//#region 🔖️Runtime` by `//#endregion 🔖️Driver` (WP3 reported five unbalanced regions there). The two end markers were renamed to the region they close; no marker removed. |

## 2. Census (gate counter `readRustSourceClosureV1`, `pwp5-tools/census3.ts`, files / bytes / active lines)

Limits 3,500 lines / 28 files / 230,000 bytes (gate 4,000 / 32 / 262,144). All lib rows are identical on native, `wasm32-wasip2` and `wasm32-unknown-unknown` (the feature row adds +5 lines for `tool-run`, +12 for `job`). Before = pre-edit (`pwp5-census-before.log`), after = final (`pwp5-census-final.log`).

| crate | lib before | lib after | test closure before | test closure after |
|---|---|---|---|---|
| `time-travel` | 3 / 240,332 / **3,632** (over the target) | 3 / 117,240 / 1,585 | 5 / 268,903 / 3,718 | 5 / 145,825 / 1,671 |
| `time-travel-panel` | 1 / 17,785 / 224 | 3 / 92,914 / 1,404 | 1 / 17,785 / 224 | 3 / 92,914 / 1,404 |
| `tool-run` | 6 / 130,621 / 2,099 | 6 / 130,443 / 2,096 | 16 / 164,738 / 2,255 | 16 / 164,560 / 2,252 |
| `command` (WP3, receives the lift) | 3 / 57,095 / 741 | 4 / 106,416 / 1,643 | 5 / 71,388 / 766 | 6 / 120,709 / 1,668 |
| `job` (WP3, receives 4 lines) | 6 / 160,340 / 2,318 | 7 / 160,860 / 2,328 | 7 / 162,034 / 2,441 | 8 / 162,554 / 2,451 |

Every row is inside the target on all three axes. Longest new path 148 bytes.

## 3. What changed

All paths relative to `PLUG`.

* **`⏪️time-travel/🦀️.rs`**: 3,456 to 1,418 lines. Keeps Limits, Outcome, Ledger, StoreState, Supersessions, plus `time_travel_selection_value` (panel to time-travel, it names `TimeTravelActionRefusal`) and `time_travel_arg_generation`. Three mounts `#[path] mod x; pub use x::*;` put the moved text back, so no module path of the monolith changed.
* **New piece files of crate `time-travel-panel`** (class A root `🪟️panel/🦀️.rs`, stub untouched): `🪟️panel/🔢️pointer/🦀️.rs` (471 lines: RFC 6901 pointer, input rows, snaps, options, coercion) and `🪟️panel/🖼️sections/🦀️.rs` (707 lines: row refusals, band, reprojection and editor sections, control rows, the two limits `TIME_TRAVEL_ENTITY_NAME_VISITS` and `TIME_TRAVEL_SHORT_ID_CHARS` moved down from time-travel). The WP3 file `🪟️panel/🛬️lifted/🎮️command/🦀️.rs` (`ui_history_panel`) is the third file of the crate.
* **`🎮️command/🛬️lifted/⏪️time-travel/🦀️.rs`** (stub filled, 901 lines): the 50 items of W5-1, mounted from the time-travel root. Region markers repeated and balanced in every receiving file (Driver with its three sub-regions stay whole).
* **`⏯️tool-run/🦀️.rs`**: `is_tool_run_action_id` cut to **`👷️job/🛬️lifted/⏯️tool-run/🦀️.rs`** (stub filled), mounted from the tool-run root with `pub use` so `job`'s own re-export keeps resolving.
* Widened (cut tool, name-keyed, only names used across the new file boundary): `history_code_text`, `time_travel_admits_withdrawal` (command lift), `time_travel_color_rgba`, `time_travel_schema_documents` (pointer piece), all `pub(crate)`; `pub(super)` in moved text became `pub(crate)`.
* **Manifests** (own crates only): `time-travel` +`semio-framework`, +`semio-framework-plugin-ownership` (named by `🎮️decision/🗑️discard`); `time-travel-panel` +`semio-framework-time-travel` (the sections piece names it). All 87 path dependencies of the five WP5 manifests resolve, TOML parses, boundary triple at the ceiling. `PKG/Cargo.toml`, registries, launch files: not touched (frozen); the six routes per crate exist in both launch files and both files parse.
* **Companions**: `tests-08` / `tests-09` docstrings (W5-4).
* **Law test** (new, `⏪️time-travel/🧪️tests/🧩️history-partition/🟦️.ts` + fixture `🧪️tests/🧫️fixtures/🧩️history-partition.json`): 25 named items each have exactly one home in a file of their crate, the monolith mounts each moved file once and re-exports it, no home file names an item of a higher-ranked crate (command 20 < job < panel 40 < tool-run 41 < time-travel 42), plus seeded negatives (injected upward reference, misplaced item, duplicated home, reversed rank). `4 pass 0 fail`. Red-first: run against the pre-cut files in a scratch tree it fails `0 pass 4 fail`.
* Tools: `pwp5-tools/` (`carve_tt.py`, `carve_toolrun.py`, `lib5.py`, `oracle5.py`, `scans3.py`, `mounts5.py`, `lossless5.py`, `readers5.py`, `needs5.py`, `surface5.py`, `restore5.py` and the WP3 copies).

## 4. Crate graph

Item-level back edges on the whole plugin tree: 55 before, **29 after**; of the 7 pairs that touched WP5 (`command -> panel` 6, `panel -> time-travel` 3, `command -> tool-run` 1, `command -> time-travel` 1, `runtime-ops -> tool-run` 1, `job -> tool-run` 1) none is left. The one remaining line that mentions a WP5 crate, `command -> reactor-turn: HistoryPanelText -> Inputs`, is a name collision (the enum variant `Inputs` against WP4's withdrawn `Inputs` struct, WP4's PD4 class), not an edge. Used first-party edges of the three crates are a subset of the declared ones (`manifest_edges3.py`): panel `decl-views`, `command`; tool-run `decl-views`, `decl-emission`, `decl-app`; time-travel `decl-views`, `decl-app`, `child`, `command`, `time-travel-panel`, `tool-run`; no upward edge. Declared-but-unevidenced edges (`interaction`, `window-*`, `mounted-owner`) stay (WP4 PD14: the token graph is blind to them).

## 5. Verification (commands run, real results)

| gate | result |
|---|---|
| patched full-resolution differential, `oracle5.py g3` (A = live tree, B = same tree with the 7 touched files restored from the pre-edit snapshot and the 2 created files removed; sandbox patches of `📓️RESOLUTION-ORACLE.md` plus `extern crate` neutralisation), one run on the final state | `lib` 8,359 vs 8,359; `--test` 3,421 vs 3,421; `wasm32-wasip2` 8,529 vs 8,529; lib + 4 testing features 9,189 vs 9,189; `--test` + features 3,422 vs 3,422. **onlyNew 0, onlyOld 0 in all five.** (The first cut, W5-2, showed onlyNew 1: the added `semio_framework` use. Gone with the revert.) |
| seeded sensitivity (`s1` to `s3`; the SPEED directive arrived during `s3`, which was stopped) | `s1`, lib, five seeds at once (`pub use lifted_command::*` removed, `pub use panel_sections::*` removed, `pub use lifted_job::*` removed, two visibility drops): **onlyNew 142** (E0422/E0425/E0432 for the moved names, `tool_run::is_tool_run_action_id`, the facade `app::time_travel::{history_mutation_entry, ...}`). `s2`, lib, the two visibility seeds alone: **onlyNew 2** (`time_travel_admits_withdrawal`, `time_travel_schema_documents`, E0425). The broken-`#[path]` seed (`s3`) was not run to completion. |
| definition scan (`scans3.py`, private names of the 20 lift/piece members used by the source module or its sibling children) | 0 hits. Seeded: `time_travel_admits_withdrawal` and `time_travel_schema_documents` made private again are each reported |
| orphan/inherent scan (every impl of the plugin tree) | 2 findings, both `impl RetireOwned for AsyncTask` in `🧵️task` (WP1/WP2), none in WP5 files. Seeded: the `HistoryPanelText` type mapped away from its impl reports `inherent-foreign` |
| mount equivalence (`mounts5.py`, roots `job`, time-travel, tool-run, tool-machine) | 34 files reachable before, 38 after; only-after = the 2 created pieces + the 2 lift stubs that are now mounted; nothing lost; 32 mounts before, 36 after, 0 lost, 0 retargeted, 4 new (`panel_pointer`, `panel_sections`, `lifted_command`, `lifted_job`) |
| losslessness (`lossless5.py`) | 7 touched + 2 created `.rs` files: 5,119 non-blank lines before, 5,151 after; lost 9 / new 41 are exactly the widened lines, the old stub docstrings, the two renamed end markers and the new mounts, region repeats, `use super::*` lines and docstrings |
| path literals, regions, rustfmt | 45 path literals in 31 files unresolved/long: none; regions unbalanced: none (tool-machine fixed); rustfmt (nightly 2026-07-20, `skip_children`) parses all 9 changed or new `.rs` files |
| source-text readers (`readers5.py`) | literals of the original text that left the module tree a TS reader sees: 1 doc mention (`member_backfill` in the comment of `🧪️tests/🧪️composed-child-history/🟦️.ts`, WP7's file) and 4 false positives of the framework module `🔨️modules/⏪️time-travel`. No assertion reads moved text. `ts_gate` on the 19 TS files that name these sources: no new failing test caused by WP5 (8 new failures in two files are `ENOENT` on `🧵️instance/**` files a peer moved) |
| launcher | start and end `Live Original resource control issuance required`; no cargo check or test could run |

## 6. Flip checklist (mechanical; compile level order: after WP2 `decl-*` and WP3 `child`, `command`)

1. **`time-travel-panel`** (class A): fill `🪟️panel/🦀️.rs` with `#[path] mod` + `pub use` of `🔢️pointer`, `🖼️sections` and `🛬️lifted/🎮️command` (the last is the panel's own file after the flip); delete the `panel_pointer` / `panel_sections` mounts in the time-travel root. Replace the three `use super::*` by explicit imports: `needs5.py` lists what each takes from the root (`semio_framework` 8 and 22 names, `semio_framework_os_kernel` 3, `semio_framework_time_travel` 4); the rest comes from the job-facet glob and the compiler names it.
2. **`command`** (WP3): one `mod` line for the lift; its 3 `use super::*` imports are `semio_framework_os_kernel` 3, `semio_framework` 4, `semio_framework_time_travel` 4 plus the facet glob; the manifest already has all three crates. Remove the `lifted_command` mount from the time-travel root.
3. **`job`** (WP3): one `mod` line for the lift; it needs `TOOL_RUN_ACTION_IDS` from `semio_framework_tool_run`, and **the `job` manifest has no `semio-framework-tool-run` dependency**: request to WP3/WP0. Remove the `lifted_job` mount from the tool-run root.
4. **`time-travel`, `tool-run`** (class B): flip `[lib] path` to `../../🦀️.rs`, delete the package stubs, replace `use super::*` and the `use crate::tool_run_contract::...` lines of the tool-run root by crate imports, and remove the three mounts in `👷️job/🦀️.rs` (`tool_run`, `tool_machine`, `time_travel`; Appendix K) in the same change; the facade lists `pub use app::time_travel::{...}` must re-export the moved names explicitly from `command` (`history_mutation_entry`, `HistoryMutationPage(s)`, `TimeTravelPanel`, `TimeTravelEditorPanel`, `HISTORY_ROW_MUTATION_ROWS`).
5. **Visibility** (`surface5.py`, names used across a new boundary and not yet `pub`; make them `pub`, `#[doc(hidden)]` where the facade must not re-export): panel to time-travel 14 (`time_travel_coerce`, `_default_value`, `_input_at`, `_input_rows`, `_json`, `_pointer_get/_insert/_remove/_set`, `_resolve_options`, `_resolve_snaps`, `_schema_documents`, `_switch_variant`, `_variant_selector`); command lift to panel and time-travel 14 (`HistoryPanelText`, `HistoryStoreStamp`, the six `Member*` visitors and tails, `history_code_text`, `history_mutation_view_of`, `history_severity_text/_tone`, `time_travel_admits_withdrawal`, `time_travel_arg_text`, `time_travel_mutation_label`); command lift to the command root and the instance family 9 (`HistoryPanelText`, `HistoryStoreStamp`, `MemberBackfill`, `history_mutation_description`, `history_mutation_views`, `history_severity_tone`, `history_store_change`, `history_store_stamp`, `member_backfill`); tool-run to time-travel 1 (`tool_run_arg_u64`).
6. **Statics**: the only static of the package, thread-local `TIME_TRAVEL_SCHEMA_SHOWS_INPUTS`, has one owner (the command lift) and one user (`time_travel_schema_shows_inputs` in the same file).
7. `instance_turns` mounts in the time-travel and tool-run roots are WP7's lines (Appendix K).

## 7. Requests

* **WP3/WP0**: `semio-framework-tool-run` for `job` (item 3); the `command` lift is 832 lines, not 1,032 (Appendix J row).
* **WP0**: Appendix J rows for the corrected sizes; `tests-08` and `tests-09` can be retired (W5-4).
* **WP7**: `time_travel_arg_generation` is called by the time-travel turns through the time-travel root (`super::*`); after the flip import it from `time-travel`. Doc mention of `member_backfill` in `🧪️tests/🧪️composed-child-history/🟦️.ts` points at the old file (now `🎮️command/🛬️lifted/⏪️time-travel/🦀️.rs`).

## 8. Residual risks

* No compiler: the differential covers name resolution, not typeck, trait solving or borrowck. Highest-risk spots at the first compile: the glob chains `pub use piece::*` next to `use super::*` (E0659), the private `thread_local!` in the command lift, the `TimeTravelPanel` view model now in `command` while `TimeTravelLedger` builds it.
* Name-keyed widening is conservative; the first compile may find a method or field the scan treats as used. Fields and methods are under-reported by the token scans.
* Peers edit the same trees (WP6/WP7 moved `🧵️instance/**` during the work; the TS gate shows the fallout, not mine).
* Not run: wasm32-wasip2 `--test` differential (WP3 skipped it too), the broken-`#[path]` seed, TS suites outside the 19 reader files.
