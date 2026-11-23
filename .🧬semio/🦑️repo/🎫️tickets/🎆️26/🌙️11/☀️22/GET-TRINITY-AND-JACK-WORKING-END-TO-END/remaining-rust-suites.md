# Remaining Rust suites on Windows (Trinity, Jack, Rewriting)

Status: in progress. All runs are native cargo through `cargo-rs.sh` (ticket folder), logs on `G:\t\logs-rs`.

## Environment finding (applies to every package)

`CARGO_BUILD_BUILD_DIR` on a separate directory (the layout asked for) makes cargo's new build-dir layout put one `-L dependency=` / `PATH` entry per dependency on every rustc command line. `semio-os-spr` fails with `The filename or extension is too long (os error 206)`. Fixed in `cargo-rs.sh`: build dir equals target dir (`G:\t\rs`) with `CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false` and `CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false`.

## 1. `@semio-tech/trinity-jack-rs` (Jack workspace, all 14 members incl. shell, lsp, lsp component)

`cargo test --workspace --no-fail-fast`: every member builds and links on Windows. Result after the fixes below: all green (main crate 106 passed, executor 15, jack-jack 2, language-service 1, lsp laws 12, shell 4, results/transient/window-config 1 each, graph-manifest-tests 2, remaining crates 0 tests).

Root causes and fixes:

| Failure | Root cause | Fix |
|---|---|---|
| `semio-s-artifact-trinity-jack-jack` lib test: E0283 `io_declaration` | The declaration test is generic over `JackComposition` and was compiled inside the mirror crate, which has no composition | Test module moved out of `🏅️standards/…/✳️any/🦀️.rs` into the main crate root (`🦀️.rs`), where `test_support::FixtureComposition` exists; the call names it |
| `sqlite_snapshot_jack_declared_json_camera_words_are_closed_and_exact` | Jack JSON decoder (`🚪️io/🔣️json/🦀️.rs`) still required the removed inline `manifest` field and admitted it as a known field | Required field is `manifestId`; `manifest` removed from the allowed fields |
| `sqlite_snapshot_jack_independent_sqlite_understands_all_authored_entities_and_words` | Independent bun oracle inserted `NULL` for the now `NOT NULL` `manifest_id` | Oracle inserts `'nakagin'` |
| `sqlite_snapshot_jack_populated_public_independent_edit_retirement` | Oracle expected 14 tables; Jack now owns 3 plus `semio_snapshot` | Table census is 4 |

Files: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs`, `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs`, `…/✳️any/🚪️io/🔣️json/🦀️.rs`, `…/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`, `…/📸️snapshot/🧪️tests/🚪️io/🦀️.rs`.

The `infinite-canvas` asset path and `core <-> ui <-> s-3d` cycle claims did not reproduce: shell and lsp are members of the Jack workspace and build and test without either (no reference to `infinite-canvas` anywhere under the Jack tree).

## 2. `@semio-tech/trinity-jack-lsp` wasm32

`cargo build --target wasm32-unknown-unknown -p semio-s-artifact-trinity-jack-lsp-component` (Jack workspace, own target dir `G:\t\rsw`) finished in 15m49s. Native lsp laws (`jack_lsp_laws`, 12 tests) pass in section 1. Not run: the wasm-bindgen/wasm-pack packaging step of `script.ts wasm`.

## 3. `@semio-tech/trinity-jack-shell`

Member of the Jack workspace: 4 tests pass (`semio_s_artifact_trinity_jack_shell` bin). No `infinite-canvas` or `core <-> ui <-> s-3d` problem exists in its build graph (the claim does not reproduce).

## 4. `@semio-tech/trinity-rewriting-rs` (Rewriting workspace): fixes made, test run NOT finished

The Rewriting Rust test crates had drifted far behind the framework, beyond the manifest change. Fixes applied:

| Area | Problem | Fix |
|---|---|---|
| SQLite backing `💰️backing/🗂️indexes/🦀️.rs` | `TABLES` still listed 30 tables (17 own + 13 Jack) while `🗄️.sql` is 17 own + 3 Jack and the test asserts 20 | `TABLES:[(&str,usize);20]`, counts array 20 (dropped the 10 removed `jack_*` tables) |
| 23 mutation tests (`narrows`, `rewrites`, `places`, `nudges`, `retitles`, `swaps`, `clears`, `drops`, `moves`) | diff parents carry `content: Option<ArtifactChild>`, `materialize_jack_snapshot` takes the child | `if let Some(content)=&mut parent.content{materialize_jack_snapshot(content,child);}` (the `tests-narrows-lhs` lib test now compiles) |
| `♻️retirement/🧪️tests/🔬️document-retirement/🦀️.rs` | `admit_owned_retirement`/`close_step` now take a `RetainedAllocationEffect` through `OriginalRetirementPump` and return `RetainedWorkPoll`; `Arc<T>` is no longer `RetireOwned` | `admit` and `drain` rewritten on `OriginalRetirementPump` (same as Jack `retire_owned_to_terminal`); the live-alias law uses `ArcLease` (a live alias blocks, the last alias retires at least like a direct owner). Retirement byte budgets 66/120/68 are read from the shared fixture, unchanged in Rust |
| `⚙️operations/🧪️tests/🔬️unit/🦀️.rs` | generic `edit_*<Op>` builders ambiguous (E0283), `dispatch_rewrite_rule_mutations` takes an `EntityIdentityAuthority` | turbofish `::<RewriteRuleMutation>`, `test_identity!` macro |
| `🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs` | generic `T` compared against `&str` | `.as_str()`/`.as_u64()` comparisons, `binary_tag: u64` |

State of the run: `cargo test --workspace --no-fail-fast` (log `G:\t\logs-rs\rewriting-test.log`) compiles every crate; the last step, linking the `semio_s_artifact_trinity_rewriting_v1` test exe, had been running for more than 3 hours (link.exe at 2-5 GB working set and about 40 s CPU: the machine disks are saturated, a 256 MB write takes 26 s on `J:` and `E:`, a 311 MB read from `G:` 7.7 s, with 7 link.exe running at once). No Rewriting test result exists yet; pass/fail counts pending. Whether the shared fixtures (manifestId-only, 17+3 tables, budgets 66/120/68) pass in Rust is therefore unverified.

### Rewriting result (run finished): 153 passed, 48 failed

| Binary | Passed | Failed |
|---|---|---|
| rewriting, editor, layout, model, mut_any, schema | 0+1+1+4+5+0 | 0 |
| inferences | 24 | 1 |
| operations | 29 | 6 |
| tests_narrows_lhs | 49 | 16 |
| v1 (mirror, 1370 s) | 41 | 25 |

Total 48 failures. The 25 in `v1`, classified by message:

1. Child-edit publication / history path (14, the area the child-edit agent owns): `history_edits_end_to_end` (acceptance pump `PluginLifecycleFailure`), `history_edit_inputs_resolve` (3 leaf payload schemas do not resolve), `conflict_history_edits_end_to_end` (conflict law on edit-rhs), `documents_reload_identically` (history edit of edit-lhs), `child_history_edits_end_to_end` (registered fixture never reaches its exact terminal state), `a_history_edit_of_a_binary64_offset_validates_and_replays` (`OriginalTurnFailure` on `historyEditInput`), `a_released_working_graph_drag_is_one_tool_transaction...`, `one_canvas_drag_is_one_history_row...`, `a_deleted_working_graph_selection_is_relative_leaves`, `patch_nodes_from_the_rail...`, `the_add_node_verb_is_one_relative_leaf...` (all `PluginLifecycleFailure`/`app.message` faults), `full_child_frame_laws` x2 (`ChildMemberRegistrationError` on "publish complete member"), `window_config ... two_concrete_windows` (`ArtifactApp::genesis_child_pack` panics in staging ingress), `export_media_graph_out_reflects_rule_applied_fixture` ("artifact child is not materialized").
2. Rewriting 🗄️.sql/SQLite: `sqlite_snapshot_rewriting_composed_child_retains_actual_rich_graph_fields`: "SQLite snapshot has no explicit retirement recipient" (framework retirement-recipient requirement, same lane as 1).
3. Framework UI value arena (4, `panels::document::tests::*`): "UI value arena requires its actual registered Source" (`collection-arena` line 138), framework-side drift, not Rewriting data.
4. Fixtures not updated to the schema change (3): `authored_rule_documents_reload_with_their_derivable_children` ("declared example"), `inference_determinism_law` and (inferences binary) `nakagin_label_core_example_dsl_round_trips`: example DSL text lacks `camera` ("missing required field 'camera'", column 104). The Rewriting example DSL strings embed a Jack parent without the required `camera`; this is a leftover of the Jack snapshot shape change and is fixable in the shared example fixture.
5. Test expectation drift, probably real assertions (3): `context_menu_grouped_disclosure...` (top-level menu exceeds row budget), `patch_nodes_refuses_what_it_cannot_apply`, `set_active_example_loads_the_registered_demo_document` (expected exactly one LoadDocument effect), `the_shared_node_graph_rows_map_to_intent_leaves` (refused row "whole-fixture" not refused by name).

`tests_narrows_lhs` (16) and the mutation tests in `v1`: `Rewriting map delta entries required` (committed diff JSON no longer encodes/decodes: the fixtures' `parameterBindings`/`ruleLayout` deltas need explicit map entries, or the diff codec must accept sparse deltas). This is the dominant leftover of the fixture change (16 + 14 in `v1` incl. `*_committed_diff_*`, `produces_committed_diff`). Next step: compare the fixture diff shape with the TS `mutate-rewrite-1` snapshot and decide whether the Rust codec or the JSON is stale.

`operations` (6): `document_text_round_trip_rewrite_rule_store` and `command_envelope_round_trip...`: "original work requires its original caller loan" (test must use the framework's original-caller fixture instead of `test_identity!`); the 2 structural-correspondence tests fail with `NotFound` (path of a mutation `🟦️.ts` surface/descriptor: the test resolves the mutation root relative to a directory that does not exist on this layout, e.g. `direct mutation surface`); the retirement test fails my new alias assertion ("a live alias must block": `ArcLease` of a live alias retires 0 bytes only if the step released nothing, here the drain released bytes; adjust the law to compare to the direct owner instead of 0).

### devgraph-build.log

No error: the `pub use semio_s_artifact_stdio_semio_inferences::io::sqlite::*` line is an `unused import` warning (`stdio/semio` base `🚪️io/🦀️.rs:243`); the log simply ends there because the build was still compiling (about 600 crates, then link). The cargo root was stopped on the coordinator's request; nothing from the dev graph or hub was run.

### Next steps

1. Fix the sparse map-delta fixtures/codec (largest group, 30 tests), the `camera` omission in example DSL (3), and the operations/structural-correspondence test paths (4); all are local to the Rewriting tree.
2. Re-run Rewriting after the child-edit agent lands (group 1, 15 tests).
3. Rebuild `semio-s-dev-trinity-jack` (`cargo-rsd.sh test --no-run -j 6`), run the `reorganize` tests with a stack sampler, then `semio-hub-trinity`.
4. Link time is the bottleneck on this machine (rewriting_v1: 3.5 h); avoid parallel links.

## 5. `@semio-tech/trinity-plugin` (`semio-hub-trinity`) and the 3 `reorganize` layout tests

Not run, same cause. `semio-s-dev-trinity-jack` (own workspace, `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/📦️packages/🦀️rust`) is built by `cargo-rsd.sh test --no-run` (target `G:\t\rsd`, log `G:\t\logs-rs\devgraph-build.log`); about 600 crates compiled, link pending. The hub crate depends on it, so build it next with the same target dir.

Layout hang: not reproduced yet. Findings so far: only the lifecycle tests (`reorganize_start_complete_finalize_is_one_undo_entry`, `aborting_a_reorganize_run_leaves_the_document_byte_identical`) go through `pump_until`, whose 120 s wall-clock deadline is checked only between loop turns, so a run of more than 10 min means one turn (`maintenance_step`, `advance_typed_operation_publication` or a `resolve_ready`) never returns. The pure layout (`layout_run_drive` over the Nakagin graph with the same job) passes in seconds in the sibling tests, and the job's emit loop is bounded by node count, so the layout job (`🕸️graph/⏯️layout-run/🏃️job`) is not the loop. Suspect: the framework tool-run turn driver (`⏯️tool-run/🧵️instance/…generation-watch-and-turn-driver`, `drive_tool_run_turn`), an area another agent is changing. Next step: take a stack sample of the running test once the exe links, then fix.

## verify-* targets

`verify-*` runs `repo ./📜️script.ts verify <name>` (bun oracles plus exact cargo laws, `🧪️tests/🔬️rust-package-commands/🟦️.ts`); the cargo laws are a subset of the full Jack suite above (green). The bun oracles were not run.

## Files changed

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs`, `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs`, `…/✳️any/🚪️io/🔣️json/🦀️.rs`, `…/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`, `…/📸️snapshot/🧪️tests/🚪️io/🦀️.rs`
- Rewriting: SQLite `💰️backing/🗂️indexes/🦀️.rs`, retirement test, operations unit test, structural-correspondence test, and the nine mutation test files named above
- ticket helpers: `cargo-rs.sh`, `cargo-rsd.sh`, `cargo-rsw.sh`
