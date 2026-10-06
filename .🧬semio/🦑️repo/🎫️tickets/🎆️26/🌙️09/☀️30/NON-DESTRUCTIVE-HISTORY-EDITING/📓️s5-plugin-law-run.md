# 📓️ Plugin law run on the ONE shared test binary

Binary: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-framework-plugin/b80951f2dbb7024a/out/semio_framework_plugin-b80951f2dbb7024a` (`cargo test -p semio-framework-plugin --lib --features artifact-app-testing --no-run`, built 2026-10-05 16:38).
Invocation per row: `cd 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust && RUST_MIN_STACK=268435456 <binary> --test-threads=4 <filters>`; whole outputs under `🗑️generated/s5-runtime/law-run/`.

| owner | filters | passed | failed | failures (first message) |
| --- | --- | ---: | ---: | --- |
| RUNTIME | `transient_root` | 4 | 0 |  |
| RUNTIME | `an_inverse_refusal_is_one_mutations_fatal_that_its_row_names_and_resolves hostile_history_edit_input_is_answered_neve…` | 3 | 0 |  |
| RUNTIME | `an_opened_instance_acts_as_its_admitted_actor_across_reload_and_on_the_revert_route` | 1 | 0 |  |
| RUNTIME | `every_route_authors_as_its_acting_actor` | 0 | 1 | `every_route_authors_as_its_acting_actor` — assertion `left == right` failed: a route without an actor of its own authors as the instance's left: [Some("local")] right: [Some("ada")] |
| RUNTIME | `time_travel supersede history_label_reload history_alternatives ui_history_panel rendering_the_history_body composed_…` | 79 | 1 | `every_route_authors_as_its_acting_actor` — assertion `left == right` failed: a route without an actor of its own authors as the instance's left: [Some("local")] right: [Some("ada")] |
| LOAD | `folder_reload_route a_merge_archive_command document_backbone` | 10 | 1 | `document_backbone_binding_reducer_preserves_generation_and_live_owner` — canonical instance zero command decodes: "plugin.document-backbone.binding-noncanonical" |
| TOOLS | `gesture_laws` | 4 | 0 |  |
| GATES | `a_blocking_ledger_replay` | 1 | 0 |  |
| CHANNEL | `typed_wire_values_leave_in_key_byte_order_whatever_their_types_declare` | 1 | 0 |  |
| NESTED | `a_child_survives an_agent_transaction_carries_owned_child composite_gesture_produces_one_undo_group a_composed_docume…` | 39 | 8 | `member_run_abort_leaves_both_stores_the_command_log_and_the_history_untouched` — abort settles never settled; state Some(Aborted)<br>`member_run_finalize_is_one_member_edit_carrying_the_run_transaction_and_one_undo_removes_it` — member finalize publishes never settled; state Some(Finalized)<br>`member_run_holds_at_most_the_member_ceiling_and_reports_the_cap` — the capped member run completes never settled; state Some(Faulted)<br>`a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` — load parent document pack: Fault { origin: Plugin, code: FaultCode("plugin.internal.document-archive-replacement.closure-rejected"), severity: Error, message: "document archive replacement failed its closure leg: recursive ownership closure validation rejected the candidate (Incomplete)", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], para<br>`member_run_pause_then_step_drives_exactly_one_unit_per_step` — abort settles never settled; state Some(Aborted)<br>`created_children_survive_absorb_into_the_child_store_map` — artifact store reached Drop without its exact terminal-empty shallow-shell witness<br>`retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires` — assertion `left == right` failed left: 9 right: 0<br>`member_run_ticks_compose_the_member_on_read_while_both_stores_stay_untouched` — abort settles never settled; state Some(Aborted) |
| AGNOSTIC | `history_edit_acceptance` | 0 | 0 | no test matches |
| ALL | `time_travel` | 66 | 1 | `every_route_authors_as_its_acting_actor` — assertion `left == right` failed: a route without an actor of its own authors as the instance's left: [Some("local")] right: [Some("ada")] |
| EXTRA | `tool_run` | 33 | 9 | `member_run_abort_leaves_both_stores_the_command_log_and_the_history_untouched` — abort settles never settled; state Some(Aborted)<br>`member_run_finalize_is_one_member_edit_carrying_the_run_transaction_and_one_undo_removes_it` — member finalize publishes never settled; state Some(Finalized)<br>`member_run_holds_at_most_the_member_ceiling_and_reports_the_cap` — the capped member run completes never settled; state Some(Faulted)<br>`member_run_pause_then_step_drives_exactly_one_unit_per_step` — abort settles never settled; state Some(Aborted)<br>`tool_run_panel_of_a_running_run_is_the_shell_fixture` — assertion `left == right` failed: the running panel is the shell fixture left: Object {"accessibility": Object {"label": String("Tool runs")}, "activity": String("idle"), "bindings": Array [], "children": Array [Object {"accessibility": Object {"label": String("Toy fill")}, "activity": String("idle"), "bindings": Array [], "children": Array [Object {"accessibility": Object {"live": String("polite")}, "activity": Stri<br>`tool_run_reconfigure_resume_retargets_a_retargetable_job_in_place` — a resident job of a complete run is no work<br>`tool_run_settings_changed_fires_only_for_the_declared_settings_reads` — assertion `left == right` failed: toyFill: only a declared read reconfigures left: 0 right: 1<br>`tool_run_window_settings_reads_follow_the_starting_window_only` — assertion `left == right` failed: a changed value on the starting window reconfigures left: 0 right: 1<br>`member_run_ticks_compose_the_member_on_read_while_both_stores_stay_untouched` — abort settles never settled; state Some(Aborted) |

Totals over the owner rows (the `ALL` regression row overlaps `w2a`; the `EXTRA` row is the whole tool-run family, of which NESTED's `member_run_` laws are a part): **142 passed / 11 failed**.
## Notes (S5-RUNTIME, 2026-10-05 16:40)

- Two runs on the same binary path: 16:24 (tree with wave H) and 16:38 (after my fix-forward H2, rebuilt). The table is the
  second run. Between them only RUNTIME's actor rows changed; every other row is identical (NESTED 39 / 8 both times).
- RUNTIME `every_route_authors_as_its_acting_actor` is the EXPECTED red: a plain `Apply` (text ingest, plain emit) still
  authors as `local`; it turns green with S5-STORE's §22.34. It is the only red in the `w2a` and `time_travel` rows.
- RUNTIME fix-forward H2 (train line 16:31:02): wave H gave every constructed document store the actor `local`; the store
  retires a replaced actor string as a displaced owner, so every directly constructed app gained one displaced owner on its
  first verb. Now the constructor binds nobody, genesis alone is authored `local`, the open binds the admitted actor.
  The actor law is split: the runtime half (`an_opened_instance_acts_as_its_admitted_actor…`) is green.
- LOAD `document_backbone_binding_reducer_preserves_generation_and_live_owner`: its canonical command bytes are refused as
  `plugin.document-backbone.binding-noncanonical` — the law's golden bytes predate f9's key-byte member order (LOAD).
- NESTED 8 reds, unchanged by H2 (so not caused by H's constructor default; the tests construct their apps directly or
  through `plugin_create_app` and dispatch with their own `ActionMeta`, which no other H hunk touches — not A/B-tested
  against a tree without H):
  five `member_run_*` never settle (`tool_runs.has_pending_work()` stays true in state Aborted / Finalized / Faulted);
  `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` — archive replacement
  `closure-rejected`, "recursive ownership closure validation rejected the candidate (Incomplete)";
  `created_children_survive_absorb_into_the_child_store_map` — "artifact store reached Drop without its exact
  terminal-empty shallow-shell witness"; `retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires`
  — the group undo leaves the parent count at 9 (expected 0).
- EXTRA (not asked for; run to place the `member_run_` reds): the whole `tool_run` family is 33 / 9 — the five member laws
  plus four non-member ones with the same "work never ends / nothing reconfigures" shape
  (`tool_run_panel_of_a_running_run_is_the_shell_fixture`, `tool_run_reconfigure_resume_retargets_a_retargetable_job_in_place`,
  `tool_run_settings_changed_fires_only_for_the_declared_settings_reads`, `tool_run_window_settings_reads_follow_the_starting_window_only`);
  the 2 ms overlay timing law failed in the first run only (load average ~100). The member reds are therefore the tool-run
  family's, not specific to N1's four tool-run sites.
- AGNOSTIC: no test of the plugin crate matches `history_edit_acceptance` (the harness is a macro other crates expand).
