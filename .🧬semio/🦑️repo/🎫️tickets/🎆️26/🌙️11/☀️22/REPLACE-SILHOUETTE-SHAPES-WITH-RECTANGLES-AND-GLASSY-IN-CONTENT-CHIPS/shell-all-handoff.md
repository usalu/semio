# Shell wgpu test crates: handoff

(Final per-crate table is appended at the end of the task. This file starts with the reusable patterns so the second agent, who owns shell-tests-docs-1/-2/-3 and shell-tests-windows-1/-2, can use them now.)

## Patterns that worked (reuse these)

Logs: `/private/tmp/claude-501/-Users-ueli-Documents-semio/0fec7092-b872-4c05-9d02-7a8aa493f8d0/scratchpad/shell-all/` (`g6-*.txt`, `g7-*.txt`, `g6-summary.txt`). Runner: `run.sh` / `groups.sh` there (shared cargo lock, `-j 3`, `--test-threads=2`, memory check, retry on source-boundary refusal). One test directly: `RUST_MIN_STACK=268435456 RUST_BACKTRACE=1 ./<test binary from the log> <filter> --test-threads=1`.

1. "UI value arena requires its actual registered Source" (`ValueError UnsupportedOwner`): the test builds or publishes a document without an arena scope. Add `let _arena = semio_ui_contract_values::testkit::UiArenaFixture::new();` as the first statement and the dependency `semio-ui-contract-values = { workspace = true, features = ["testkit"] }` in the crate's Cargo.toml (already added to every shell-tests-* crate).
2. "original work loan callback replaced its initiating context or refilled its work" (`OriginalTurnFailure`, `🎟️original-turn/🦀️.rs:57`): the paint or step callback spent more than the one fuel unit of `FixtureTurnV1::turn`. Use `caller.turn_funded(1 << 20, |cx| ...)` for render, paint and chrome steps (applied across `🧱️elements/🐚️Shell/🧪️tests` for callers of `render_ui_document_step`, `PanelsViewCtx`, `ChromeViewCtx`, `OverlaysViewCtx`). Wide documents also need `FixtureTurnV1::wide(grant, 1)`.
3. `🎟️original-turn/🦀️.rs:66` (document close never finishes) with a mounted World3d: use `retained_world_sequence_probe_closing(name, semio_renderer_wgpu_host_winit::os_host::finish_world_fixture_component_close)` (the crate needs `semio-renderer-wgpu-host-winit` with feature `testkit`), or `PointerFixtureTurns` with `serving_world_closes`, `retire_shell_worlds`, retire the documents, then `finish_with(host)`. Board2d scenes painted through the plain `paint_component_pointer_documents` wrapper still hang on close; not solved yet.
4. Source-text laws read `semio_renderer_wgpu_test_support::shell_source()`. It silently dropped every file mounted from outside a crate's own directory (`../` mounts), i.e. all staged Shell code. Fixed with `read_module_tree_source_within(root, boundary)` in `🧯️test-support/🌳️module-tree` and `shell_source()` using the engine root as boundary. The remaining stale laws are text drift: the product moved code behind `self.settle.`, `self.as_actions().`, `self.as_window_actions().` and `semio_renderer_wgpu_host_base::`, and React/TS sources moved (`🔨️modules/🖱️ui/🔨️modules/🆔️element-identity`, `ShellHost/📋️panel-hosting/🔶️brand`, `🎞️frame-worker/{🔌️boot-and-plugins,📨️receive}`). Update the needle to the current spelling; verify with a python walk over the `#[path]` mounts (`scratchpad/shell-all/tree.py`). Never delete or weaken the law.
5. Platform: `host_platform_uses_meta()` is a thread-local defaulting to macOS. Tests built on ctrl chords call `semio_renderer_wgpu_host_base::set_host_platform_uses_meta(false)` first. `shell_edit_verb_for` now answers ctrl or meta without alt (React's law), not the platform `mod`.
6. Borrow fixes after the ShellState context refactor: `SessionCtx::of(&mut shell)` needs `let mut shell`; inside helpers that take `shell: &mut ShellState` use `Ctx::of(shell)`.
7. Region laws that assumed one `//#region` now see one region per mounting file: use all regions (`regions_of`) or the vocabulary tree lines.
8. Unknown locale (for example `fr`) shows both tongues (`EN | DE`); there is no default language (`keybinding_unowned_text_v1`).

# Final state (not all green)

Logs: `scratchpad/shell-all/` (`f-a/b/c`, `f2-a`, `f3-a/b`, `f4-c5`, `g6-*`, `g7-*`). All runs ran under the shared cargo lock, `-j 3`, `--test-threads=2`. Other agents' in-flight compile errors blocked runs repeatedly; every one cleared by itself, except the ones fixed below.

## Counts per crate (latest run of that crate)

| Crate | Result | Run |
|---|---|---|
| shell-tests-chrome-1 | 57 passed / 4 failed | f3-a |
| shell-tests-chrome-2 | 10 / 10 | f-b |
| shell-tests-chrome-3 | 40 passed / 1 failed | f3-a |
| shell-tests-chrome-4 | 102 / 102 | f-c |
| shell-tests-chrome-5 | not countable: the test binary aborts with a stack overflow after at least 10 failures (listed below) | f4-c5 |
| shell-tests-chrome-6 | 95 / 95 | f3-a |
| shell-tests-input-1 | 73 / 73 | f-b |
| shell-tests-input-2 | 58 passed / 14 failed | f3-b |
| shell-tests-input-3 | 78 passed / 1 failed | f3-b |
| shell-input | 5 / 5 | f-c |
| shell-view-chrome (with `--features semio-renderer-wgpu-shell-view-chrome/testkit`) | 6 / 6 | f-c |
| shell-tests-docs-3 | 50 passed / 9 failed | g7-c (second agent) |
| shell-tests-windows-1 | 67 passed / 4 failed | g7-d (second agent) |
| shell-tests-windows-2 | 54 passed / 6 failed | g7-d (second agent) |
| shell-tests-docs-1, docs-2 | not measured after the last fixes: g7-b stopped on a docs-2 compile error | second agent |
| the 27 non-test `semio-renderer-wgpu-shell-*` crates | no tests of their own besides the above (0 passed in the run output) | |

## Fixes (root cause, file)

- Arena scope missing for ~40 tests: `UiArenaFixture::new()` in the test, dev-dependency in every shell-tests crate (`Cargo.toml` of each).
- `FixtureTurnV1` fuel: `turn_funded(1 << 20, ..)` in paint helpers (test fixture, stale test).
- `shell_source()` dropped all `../` mounts: `read_module_tree_source_within` in `🧯️test-support/🌳️module-tree/🦀️.rs` and `📜️repo-text/🦀️.rs` (test infrastructure; this makes ~15 source-text laws read the real Shell code).
- Product: `UiPreferencesLogRefusal` and `ShellPrefLocks` made `pub` (`🎛️prefs/🔑️storage-prefs-and-i18n`); `shell-actions` import of `sync_attach_document_id`; `shell_edit_verb_for` answers ctrl or meta without alt (law in its own docs); `keybinding_unowned_text_v1` shows both tongues for an unknown locale.
- Stale tests updated to the current product spelling (needles, fixture paths, moved React/TS files): chrome-i18n laws, boot example, switch body, file-open-import, sync-owner-lifetime, tour-overlay (control ids, panel journal, platform door), appearance-tour, ui-prefs-themes-i18n, browser-prefs-persistence, chrome-maintenance-pressure, host-door-remainder, settle-pump (+ fixture `🫀️settle-pump/🔣️.json`), navbar-footer-parity fixture paths, confirm-dialog scrim (reduced motion so the enter animation is settled, scrim found by its viewport rectangle).
- Borrow/`mut` repairs in the window tests (second agent's crates now).

## Still failing and why

- chrome-1 (4): `accepted_event_feed_rows_publish_tabbable_buttons_read_only_text_and_e...` (accessibility activate returns none), `apply_drop_committed_queues_the_merged_action_into_input` (an action is queued where none is expected), `accepted_node_graph_note_caret_arms_resets_and_retires_with_its_exact_...` (fixture: attached surface never reaches terminal witness), `retained_input_refusal_settlement_obeys_the_shared_draft_disposition_l...` (commit does not submit). Not root-caused; retained-input product work, likely another agent's area.
- chrome-3 (1): `no_capitalised_chrome_phrase_escapes_the_bilingual_tables`. Now that the source tree is read completely, the law lists real untranslated or non-table literals (local-folders, file-open fault messages, form page, export labels, ...). Product decision needed: route them through the tables, or declare the fault diagnostics an exception in the law. The law was passing vacuously before.
- chrome-5: stack overflow in the test binary (one `<unknown>` thread; 256 MB `RUST_MIN_STACK` is not enough) plus 10 failures seen: `browser_prefs_persistence_tests::{the_browser_preference_lane_is_the_page_door, the_key_census_covers_reacts_own_key_list}` (source-text drift not yet updated), 7 `display_conflicts_marketplace_tests` (arena scope added, still failing; reasons not read after the fix), `navbar_footer_parity_tests::an_unfolded_projection_pane_publishes_its_rows_within_one_frame_budget` (paint budget of 4096 exhausted even with a wide caller).
- chrome-6: green.
- input-2 (14): 9 `presented_input_authority_tests`: `paint_retained_button_candidate` finishes its caller while the test still holds the document, so the fixture close never terminates ("original fixture root closure never reached terminal", demand copy_bytes 112); the helper must keep the caller alive across the test and close after the document is retired (pattern 3 above). `retained_tree_reorder_marker_...` (no marker), `find_publishes_filters_and_activates_...` (same close problem), `picking_an_arg_carrying_command_opens_its_form` and `workspace_clipboard_keeps_copy_through_the_next_paste_shortcut` ("stale UI value owner" at document retirement), `workspace_clipboard_keeps_the_latest_effect_and_preserves_pa...` (clipboard ingress, owned by the other agent as instructed).
- input-3 (1): `the_status_stacks_sit_and_size_like_react_notice_stacks`: `route.frame.w <= 1440` fails; the route-admission width fraction comes from the chrome parity constants and was not examined.
- Peer roster announcement (`wgpu-time-travel/👥️notices-peer-edits-and-finalize`, docs-1): product now prefixes "Bearbeitet" to every peer, including one with no activity; looks like a product bug, not decided.
- Not touched by me: `semio-os-flow-host-read` source boundary, `semio-framework-value-native-*`, `semio-plugin-composition-publication` (each blocked runs for a while, then cleared).

Verified by running: every count above and the green crates. Reasoned only: the cause statements for the unresolved items.
