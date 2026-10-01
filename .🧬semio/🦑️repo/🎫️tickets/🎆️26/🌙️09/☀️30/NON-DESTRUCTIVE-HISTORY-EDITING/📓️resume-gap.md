# 📓️ Resume Gap Audit — Non-Destructive History Editing

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, requirements-gap audit written by the session-2 coordinator's read-only auditor
(2026-10-01, about 11:40). Nothing was built, run, committed or edited except this report. Every status below comes from
reading the design, plan, status log, reports and the CODE ON DISK; no test was run by me. The tree is being edited concurrently
by peers (line numbers shift between reads), and the whole fleet died mid-edit around 08:05–08:16, so "compiles today" is
UNKNOWN for every claim that rests on an earlier report.

Aliases: `T` = this ticket folder; `FW` = `🧰️framework/🔨️modules`; `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`;
`PLG` = `OS/🔌️plugin`; `STORE` = `OS/🏪️store/🦀️.rs`; `RE` = `OS/📺️renderer/🧑‍🎨engine/🧱️elements`;
`PZ2D` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any`.

Status legend: **LIVE** = implemented and verified live in a browser (e2e evidence); **UNIT** = implemented and unit/law-tested
(Rust, TS, fixture corpus); **UNVER** = implemented in source, never compiled/run since it landed (or only reported "written");
**PARTIAL**; **MISSING**.

## 0. Verdict in ten lines

1. The core mechanism is built and, for puzzle 2d in the React shell, works end to end live: drag, one history row, Edit, time-travel band,
   preview (draft applied, downstream not applied), real controls (stepper), Accept, replay, review, Finalize dialog, overwrite and new
   alternative, undo/redo of a finalize, fatal path (blocked, Next problem, Withdraw, ready, Exit with zero trace), en and de (Run 2: 85 PASS / 3 FAIL).
2. ALL live evidence is from one old build (activation #4, 2026-09-30 about 19:20). Activations #5 and #6 failed or were aborted. Everything landed
   afterwards (trunk as an alternative, section order, host events, folder binding, reload labels, `Edit.verb`, W3-T conversions) is unit-level only.
3. The wgpu shell has NEVER run live. The probe mode (`--renderer=wgpu`, port 6112) is written and strict-tsc clean; no `probe-wgpu-*` artifact exists.
4. "Use selection" for the targets input (goal item: selection stays editable) has no unit test and no live run; the fatal path was resolved by
   Withdraw only, never by editing targets. Chips show raw node ids.
5. Input metadata is rendered live only as a stepper. Slider-with-snaps is unit-tested in both shells but not live; the history editor's builder DROPS
   `snaps`, `unit`, `displayUnit/displayFactor` for a Dial (puzzle 2d rotate angle) and ignores `scale: log` for sliders (puzzle 2d scale factor).
6. Warnings: rendering is proven live (Error rows while blocked; a Warning row after reload), but no e2e makes an edit produce a new warning, finalizes
   it, and checks the row after finalize and after reload.
7. Reload persistence of edited histories and alternatives FAILED live in Run 2 (R2-2); fixes landed in source afterwards and were not re-probed.
8. Artifact-agnostic runtime is real (framework runtime + toy app + gis, writer, flow-child runtime sessions + ~10 per-plugin replay laws), but history-row
   labels fall back to raw op text for every app that does not override `ArtifactApp::mutation_label` (only 8 app crates do).
9. The 64-slot ledger (`ARTIFACT_HISTORY_LEDGER_CAPACITY`) caps a document at about 60 edits; the synchronous `dry_run` / `reproject` paths are explicitly
   budgeted for it (no progress, no cancel). The sub-ticket `PAGED-ARTIFACT-HISTORY-LEDGER` is owned by another session.
10. Design reading to confirm with the dev: the design makes Error AND Fatal block finalizing; the goal text says only fatal errors must be resolved first.

## 1. Requirement table

| ID | Requirement (atomic, testable) | Status | Evidence | Gap and work to close |
|---|---|---|---|---|
| R01 | Editing never rewrites an op: a replicated `Supersede` history transition (tag 6) replaces the effective input or withdraws it; content-addressed, schema-first, 3 encoders | UNIT | `FW/📡️replication` `HistoryTransition::Supersede`; `🧪️tests/🧪️history-transition/🐍️.py` (independent Python encoder) + TS Ajv; `🧪️supersede-fold`; replication 313/0, TS 20 (status 02:46) | Re-run after the mid-edit fleet stop (`Edit.verb` landed in `🔗️causal` and `🎮️mutation` codecs). Acceptance: replication lib + TS conformance green on the final tree. |
| R02 | Every fold site folds effective forwards; an overwrite head equals a fresh fold of the edited log; cancel at any step leaves the store untouched | UNIT | `STORE` `state_before` / `begin_report_replay` / `commit_finished_replay`; laws `interior_supersede_equals_a_fresh_replay_of_the_edited_log`, `cancelling_a_report_replay_at_every_step_leaves_the_store_untouched`, `convergence_early_exit_equals_the_full_replay`, `an_input_breaking_the_supersede_law_folds_as_a_fatal_no_op_at_every_site` (`OS/🏪️store/🧪️tests/🧪️supersede-replay`); TS oracle Ajv + fast-json-patch | None functional. Final-tree re-run. |
| R03 | Every mutation in history is editable (has an input schema, no foreign steps, not a viewer) | PARTIAL | `editable` computed in `PLG/⏪️time-travel/🦀️.rs` `history_mutation_views_of` = `!viewer && !may_emit_foreign_steps && input_schema().is_some()`; per-WP lint zero reports; payload-law sweep 103 binaries / 199 tests / 0 failures (status 17:05) | Triage item 17:05: hand-written/generic aggregates have no emitted payload law. No repo-wide gate that every applied leaf of every plugin is editable (or declares why not). Work: gate enumerating every `Mutation` aggregate asserting `input_schema().is_some()` or a declared non-editable reason; run the final strict `schema mutation-inputs` + `mutation-payloads` repo-wide (never run centrally after the WPs closed). |
| R04 | Mutation rows are labelled from the leaf in every locale (en/de) for every plugin, also after reload | PARTIAL | `ArtifactApp::mutation_label` defaults to `None` (`PLG/🦀️.rs` ~13721) so rows fall back to the locale-invariant op text (`time_travel_mutation_label`); only 8 app crates override (puzzle 2d, writer, shooting, fem 2d + 3d, layout, draw, note); reload label via new `Edit.verb` field (`🎮️mutation/🦀️.rs` ~1647, `plugin/🦀️.rs` ~27205) is UNVER | Work: derive the default from `SemanticMutation::label` for every app whose aggregate derives `Mutations` (delete the per-app overrides); gate test across all plugins asserting no row falls back to `print_op`; re-probe the reload label (Run 2 saw `create-node node {…}` after reload). |
| R05 | A tool gesture is ONE history row (grouped by `TransactionRef`), expandable to its mutation rows | LIVE | Run 2 step 2 (en+de): exactly one new row "Drag 2 items by (80, 40)" / "2 Elemente um (80; 40) ziehen", expands to its `drag-selection`; plugin law `a_committed_tool_transaction_is_one_row_with_its_reference_and_mutation_rows` | Rows cap at 8 mutation children in the panel (`HISTORY_PANEL_MUTATION_ROWS`), flagged first; 32 on the wire. Add a "show N more" row if a transaction can exceed 8 mutations. |
| R06 | `TransactionRef` is persisted and replicated (all codecs, `.spr`, `.ops`) | UNIT | `operations_carry_semantic_kind_label_and_transaction_everywhere`, `supersede_commands_round_trip_every_codec`; `FW/📡️replication/🧪️tests/🧪️transaction-ref` | `.spr`/`.ops` persistence is unit-level; the SQLite snapshot rollout (non-fleet peer) may change the on-disk path. Re-verify on the final tree. |
| R07 | Editing any history mutation enters time-travel mode in BOTH shells: persistent polite status band, per-window indicator, remappable chords, document frozen | PARTIAL | React LIVE (Run 2 step 3: `role=status`, `aria-live=polite`, stage `editing`, indicator on all 3 windows, chords Alt+Enter / Alt+Backspace / Alt+Shift+Backspace, `timeTravel.frozen`). wgpu UNIT only (`RE/🐚️Shell/🧪️tests/🧪️wgpu-time-travel`, 21 tests counted on disk, 22 reported; `shell.time-travel.status` in the ARIA mirror) | wgpu live: see G2. React does not auto-reveal the History panel at the edge into a session (wgpu does, tested `a_session_reveals_the_history_tab…`); matters for agent/MCP-begun sessions. |
| R08 | The session reducer: stages, events, generation fence, BaseMoved, illegal/stale/blocked/empty refusals | UNIT | `FW/⏪️time-travel` Rust 12/12, bun 18/18 with ajv + xstate + fast-check (34 of 34 legal rows visited), lifecycle-law fixture | Open audit nits N-1, N-3, N-4, N-6, N-8 (`📓️w1-b-report.md` §5): non-blocking. |
| R09 | While Editing, the UI shows the document as of the mutation with the draft applied; downstream mutations are NOT applied and are marked | LIVE (React) | Run 2 step 3: `preview-is-state-before-target-plus-draft`, `downstream-not-applied-while-editing`, row reads "Not applied while editing" / "Beim Bearbeiten nicht angewendet" | wgpu not live. Only the board window was position-checked; check inspector/other windows too. |
| R10 | Draft editing is generic: `historyEditInput{path,value}` -> pointer set -> JSON-schema validation -> rebuild -> canonical bytes; invalid values refused with a reason on the editor | UNIT (+ LIVE for a stepper) | `draft_time_travel_input` (`PLG/⏪️time-travel/🦀️.rs` ~1865); test `an_open_session_freezes_document_verbs_and_refuses_stale_or_invalid_drafts`; live: dx fill 120 + Enter, ArrowUp 121 / ArrowDown 120 (step 1 from `gridFactor`) | A hard-bound violation (min/max) is never exercised live in either shell. Add a probe verdict: type a value outside a schema bound -> refusal row names the reason, draft unchanged. |
| R11 | Accept replays all downstream mutations in Report mode from the earliest accepted draft; every mutation reports success/warning/error/fatal | LIVE (React) | Run 2 step 4: Accept -> `reviewing` / `ready` in about 100 ms, head moved to +120, downstream drag re-applied; store `downstream_warning_error_and_fatal_outcomes_are_reported_per_mutation` | Only Error (target-missing) was produced live. Warning-class outcome never produced by an edit (see R20). |
| R12 | Discard and Exit leave zero trace (no new row, no position drift) | LIVE (React) | Run 2 step 8: Exit -> 0 drift, 0 new rows; plugin scenarios `exit-leaves-zero-trace`, `discarding-the-only-draft-exits` | wgpu not live. |
| R13 | A mutation can be withdrawn as a resolution | LIVE (React) | Run 2 step 8: real `framework.history.editor.withdraw.row`; store law `withdrawing_an_operation_folds_it_as_a_no_op` | wgpu not live. |
| R14 | Fatal/Error mutations block finalizing; a resolution loop ("Next problem", disabled Finalize with a reason) repeats until the tail is clean | LIVE (React), via Withdraw only | Run 2 step 8: review `blocked`, Finalize disabled ("Blocked: …"), failing row "Error: Target missing", `framework.history.timeTravel.nextProblem.row` opens the failing drag, Withdraw + Accept -> `ready` ("Accepted changes: 2") | (1) Resolution by EDITING the failing mutation's inputs (targets via Use selection, dx/dy) was never exercised; only Withdraw. (2) Design reading: Error blocks too; goal text says fatal. Confirm (G14). (3) The loop cap in the probe is 4 rounds; a deeper cascade is untested. |
| R15 | The user can keep making changes to OTHER mutations after a clean review (Begin from Reviewing; multi-mutation drafts; re-edit an accepted draft; re-replay from the earliest accepted) | PARTIAL | Reducer law `Begin` legal from `Inactive`/`Reviewing` (`FW/⏪️time-travel/🦀️.rs` `apply`, `begin` starts from the accepted draft); plugin scenarios `withdrawing-the-failing-mutation-unblocks-finalizing` (8 steps); LIVE only from a BLOCKED review (step 8: edit another row, Accepted changes 2). The Edit row action is not stage-gated (`history_panel_mutation_row`) | No test or probe step does `ready` -> Edit another mutation -> Accept -> `ready` (2 drafts) -> Finalize once and checks the final head equals a fresh fold with both edits. Add as plugin scenario + probe step 4b. |
| R16 | Finalize prompts: overwrite the existing artifact or create a new alternative (name field, localized default, destructive tone for overwrite) | LIVE (React) | Run 2 steps 5 and 7 en+de: dialog `finalizeHistoryEdit`, Overwrite `data-destructive=true`, name default "Edited history" / "Bearbeiteter Verlauf"; wgpu dialog by keyboard alone is UNIT (`the_finalize_prompt_is_operated_by_keyboard_alone`) | wgpu live. Run 2 console showed `commitCheckpoint refused: timeTravel.frozen` at the New-alternative submit (R2-6); fix landed (`dispatchCheckpoint` is a no-op in time travel), not re-probed. |
| R17 | Overwrite = one unscoped `Supersede`, effective on every alternative; Undo/Redo of a finalize | LIVE (React) | Run 2 steps 5 and 6: "History edited — overwrite: 1 mutation" row, drag row relabelled (120, 40), Cmd+Z -> +80, Cmd+Shift+Z -> +120; plugin laws `undo_and_redo_of_a_finalize_author_restoring_supersedes_on_both_replicas` | — |
| R18 | New alternative = pending-commit + `Branch` + scoped `Supersede`, atomic; the original is preserved | LIVE (React) | Run 2 step 7 (two alternatives created, positions switch +120,+70 / +120,+100); store `a_new_alternative_preserves_the_trunk_it_branched_from` | Each alternative consumes alternative + checkpoint + change ledger slots (64 each); see R39. |
| R19 | Alternatives are listed and switchable, including the trunk ("Main line") | PARTIAL | LIVE for named alternatives; trunk as first-class alternative landed after Run 2 (W1-G follow-up 2: replication 309/0, kernel 878/1) = UNIT; Run 2 FAIL `trunk-listed-after-new-alternative` | Re-probe after re-activation: `switching-to-the-trunk-shows-original-positions`. Per-viewer head is a separate ticket (R38). |
| R20 | New warnings are visible in history, persisted per mutation, also after finalize and reload | PARTIAL | Row severity/code text in words (`history_mutation_description`); live: Error text while blocked; a "Warning ·" (`mutation.no-op`) on the Set Active Example row after folder re-attach (finding 8); outcomes are recomputed on load (`reprojection_replay`) | No e2e where an edit yields a NEW Warning (`mutation.partial` / `clamped`), review `ready` with the warning, finalize, row shows it after finalize and after reload (en+de). The wire does not distinguish "newly arisen" from pre-existing warnings. |
| R21 | Replay shows progress and can be cancelled; a cancelled replay keeps the drafts and offers "Replay again" | UNIT | `step_time_travel_replay` (<= 4 ms slices, progress throttled 100 ms / 5 %), `historyEditCancelReplay`, `historyEditRerun`; tests `cancel_keeps_drafts_and_a_remote_edit_replays_again`, `replay_progress_rides_the_unsolicited_ui_frame_with_the_session_status`; wgpu `unsolicited_progress_patches_move_the_band_between_dispatches` | Never seen live: a 2-3 mutation replay completes inside one turn (no `replaying` frame). The synchronous paths have NO progress/cancel: `dry_run` (`STORE` ~20003, finalize authoring), `reprojection_replay` (`STORE` ~19278, remote supersession / undo / checkout), preview `state_before`. AGENTS.md demands progress + cancel for expensive operations; acceptable only while the ledger is 64 (see G9). |
| R22 | A remote change during a session moves the base (BaseMoved) and re-previews/re-replays; remote supersessions converge | UNIT | `watch_time_travel_base`; `cancel_keeps_drafts_and_a_remote_edit_replays_again`; store `replicas_converge_on_a_supersession_under_shuffled_arrival`; hub concurrent-supersede law over real sockets (status 02:46) | Never live (no second peer or hub in any e2e). W1-G follow-up 4 (hub no longer forces a rebuild after a refusal) and the rebootstrap test are source-only/unverified after the fleet stop. |
| R23 | Peers' history-edit presence: roster badge and row notes | UNIT | React 17/17 and wgpu 15/15 on the shared fixture `🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-peers`; probe asserts the one-tab negative only | Live positive case needs a hub-backed space serve. |
| R24 | Hub admission, grading and check-in handle `Supersede` (no ownership rule; Error/Fatal blocking replay refused) | UNIT | `OS/🛢️db/🗿️artifact` `history_transition_refusal`, grading by replacement target; db 718 (2 pre-existing), sync 87/87; os-hub 173/1; semio-hub 247/8 | `semio-hub` crate never confirmed green (peer breaks); `check_in_refuses_a_supersession_whose_replay_blocks_under_normal` exists at store level. Re-run hub crates on the final tree. |
| R25 | Edited histories (supersedes) and alternatives survive reload | PARTIAL | UNIT: `a_history_edit_is_its_own_row_locally_remotely_and_after_reload` (text + pack), `a_document_archive_round_trip_lists_every_history_row_of_its_source`, folder-archive-restore 4/4, `retire_displaced_document_rows` (`PLG/⏪️time-travel/🦀️.rs` ~2302), folder binding event-sourced (W2-B f4). Run 2 LIVE FAIL: `document-rows-survive-the-reload`, `overwrite-row-survives-the-reload` (R2-2, R2-4) | Re-probe step 5 on the new build. Native wgpu folder re-attach not done (W2-C item 5, status unknown). Commands without a description relied on `Edit.verb` (UNVER). |
| R26 | Every mutation input carries UI-element metadata (`x-semio-ui`: widget, min/max, step, snaps, unit, label en/de, role/ref); one reader Rust + TS | UNIT | Leaf schemas annotated per scope (lint 0 per scope: stdio-a/b, norm, energy, architect, mid, design, tail, puzzle 2d 36 leaves); `mutation_input_defs` Rust + TS twin + corpus; census 4297/4600 at 11:55 then per-WP zero | No single repo-wide strict run after all WPs closed (W3-G). |
| R27 | Stepper with step/min/max/precision rendered from the metadata in both shells | LIVE (React) | Run 2 step 3/4: `framework.history.editor.input.dx|dy` stepper, `step="1"` resolved from `snapSource {config: gridFactor}` (`time_travel_apply_snap_spacing`) | wgpu: UNIT only (`the_actions_form_renders_every_editor_kind`). |
| R28 | Slider with snapping points (detents + ticks) rendered from the metadata in both shells | UNIT | UI contract corpus `slider-with-snaps` (React `Slider` component tests; wgpu `slider_tick_rects`, "one tick painted per detent"); panel `Slider` arm passes `try_snap` (`PLG/⏪️time-travel/🦀️.rs` ~2855) | Not live in either shell. Puzzle 2d `scale-selection.factor` (slider, snaps 0.25 0.5 1 2 4, log scale) is not exercised; `scale: log` and `displayFactor` are not consumed by the editor builder or the UI contract slider. |
| R29 | Dial/angle input shows snaps and the display unit (degrees) | MISSING (in the editor) | `ActionArgControl::Dial { min, max, step, .. }` arm (`~2868`) drops `snaps`, `unit`, `display_unit`, `display_factor`; `displayFactor` is only consumed by dialog/palette staged args (`🛠️ShellHelpers/🟦️.tsx` `stagedNumberDisplayText`). Puzzle 2d `rotate-selection.angle` declares snaps (-pi .. pi) and degrees | Work: pass snaps and show degrees (add `displayFactor`/`displayUnit`/`scale` to the contract slider in Rust + TS + React + wgpu, or convert in the builder); acceptance in G6. |
| R30 | Vector, colour, toggle, select/segmented, nullable (Clear), text inputs derive their control | UNIT | `colour_inputs_render_the_contract_recipe_and_vector_axes_keep_their_snaps`, `a_nullable_input_offers_a_clear_control_that_drafts_null`, `union_inputs_resolve_against_the_active_variant`; wgpu `the_history_editor_controls_project_their_corpus_accessibility` | Not live. |
| R31 | Selection inputs (targets) are editable: chips, remove, "Use selection" in the input's selection domain | PARTIAL | Rendering LIVE (Run 2 step 3: reference list with 2 chips + Use selection). `draft_time_travel_selection` (`~1944`) reads the interaction selection of `Reference.domain` at its granularity; puzzle 2d declares domain `vortex` (`PUZZLE2D_INTERACTION_DOMAIN`) and granularity `node` | NO unit test of `historyEditUseSelection` (no hit in any test), NO live run, no chip-remove test. Chips show raw UUIDs, not node labels, and cap at 8 (`TIME_TRAVEL_PANEL_CHIPS`). Work in G3. |
| R32 | Tools are state machines yielding mutations inside ONE `ToolTransaction`; tools themselves are never history-editable; host events (blur, capture lost, frozen, base moved) abort with zero trace | UNIT | `FW/🛠️tool-machine` (Rust 13/13, bun 18/18, laws `transaction-law`, `scrub-law`, `typing-law`, `node-drag-law`); plugin runtime glue `PLG/🛠️tool-machine`; freeze + `HostEvent::TimeTravelFrozen`; puzzle 2d `a_forwarded_host_event_mid_gesture_leaves_zero_trace` | Run 2 console: `hostEvent` dropped as undeclared (R2-5); fix landed (framework-declared `hostEvent`), not re-probed. |
| R33 | Puzzle 2d: drag/rotate/scale are parametric leaves; one gesture = one edit = one row; cancel = zero trace; keyboard nudge, HUD move, inspector delta reuse the leaves | LIVE (drag) / UNIT (rest) | Run 2 step 2 (group drag, mod+d clone drag); `PZ2D/✏️editor/🧪️tests/🧪️select-tool-transactions` (20 tests: one drag one row, cancel, two drags two transactions, streamed ring rotation, threaded streams, host aborts) | Rotate and scale gestures and their edit flow never run live. No crate-level `state_before` / `begin_report_replay` law in puzzle 2d (wfc, layout, puzzle 3d/5d, process3d have one; puzzle 2d relies on the live run). wgpu board coalescer replay "pending" (W2-D). |
| R34 | Puzzle 2d: selection AND offset stay editable after the drag (targets, dx, dy) | PARTIAL | dx/dy LIVE; targets see R31 | Same as R31; plus edit-targets then replay must be exercised. |
| R35 | The runtime is artifact-agnostic: any plugin gets the full flow without plugin code | PARTIAL | Framework runtime (`PLG/⏪️time-travel`) + toy app tests; real-plugin runtime sessions: gis exaggeration (begin/input/accept/finalize), writer typing runs, flow composed-child (design §12, `historyEditBegin{store}`); store-level replay laws in wfc 2d/3d/bitmap, layout, puzzle 3d/5d, process3d; cad transform replay | Only puzzle 2d React ever ran live. No cross-plugin acceptance harness that runs begin/input/accept/finalize on a representative leaf of EVERY plugin. R03/R04 hold the generic holes. |
| R36 | All other plugins' gestures are tool machines (no per-tick amend, no static coalesce key) | PARTIAL | 13 crates reference `ToolMachine` (puzzle 2d/3d/5d, fem 2d/3d, shooting, layout, cad, draw, note, generation3d, wfc bitmap, process3d) + framework Scrub/Typing glue (gis, energy, forms, norm, playbook, writer); source-complete but verification pending for W3-T-PUZZLE, W3-T-SPATIAL, W3-T-LAYOUT, W3-T-FLOWCAD | 8 files still call `Emit::amend` (remodel x3, process cursor, cad engagement, dag x2, shooting camera); lowpoly, block, space, trinity, mathematical, sequence, animate, procedural 2d, forms stroke paths not converted; W3-T2-CLOSURE (delete `Emit::amend` / static coalesce keys) not started. |
| R37 | i18n en/de with no default language; accessibility (roles, aria-live, keyboard); remappable chords | PARTIAL | `TimeTravelLabel` 23 labels en+de; `HistoryPanelText` per-locale; `ui.timeTravel.*` bundles (translation-totality 2/2, check-chrome-i18n clean at 22:38); band `role=status` polite, indicator `role=note`, stepper arrows (live); wgpu `shell.time-travel.status`, notices as polite status, dialog keyboard-only (UNIT); `ui.timeTravel.accept|discard|exit` rows, Escape never discards | Live: en chords were not driven (probe default `--chords=de`); focus management on Edit/Accept/Finalize never checked; wgpu mirror never live; mobile and tablet layouts (desktop first, then mobile, then tablet) never checked. |
| R38 | Collaboration semantics are per viewer: a new alternative must not move other users' heads | MISSING (separate ticket) | `Branch`/`Checkout` fold on every replica (design §9.8); ticket `26/10/01/PER-VIEWER-ALTERNATIVE-HEAD` open, other session | Finalize-as-new-alternative switches every collaborator today. Coordinate: the finalize flow's acceptance test must be re-run when that ticket lands. |
| R39 | Long editing sessions are possible (no 64-edit ceiling) and time-travel stays responsive on them | MISSING (separate ticket) | `ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` (`OS/🌿️vcs/🦀️.rs:196`); 65th edit fails "edit history ledger is saturated" (`STORE` `reserve_edit_history_slot` ~17549); four ledgers (edits, changes, checkpoints, alternatives) each 64; `dry_run` and `reprojection_replay` are documented as synchronous "by budget" of 64 edits; sub-ticket `PAGED-ARTIFACT-HISTORY-LEDGER` (other session) | Effect: about 60 gestures per document; each new alternative also spends alternative + checkpoint + change slots. See G9. |
| R40 | Gates and hygiene: descriptors regenerated, launch rows, schema generate, taxonomy, dependencies, layering, docstrings, warnings, hub and energy crate rechecks | MISSING (pending W3-G) | W2-A open item: every plugin descriptor stale (12 verbs, finalize dialog, `noteShellCommand.label`); `launch.json` carries the new `-rs` packages (10 hits) | Run the central chores after the tree is compile-clean (G11). |

## 2. The thirteen focus areas, answered directly

**(a) Keep changing other mutations after a clean review.** The reducer and the runtime allow it (`Begin` from `Reviewing`; the accepted draft is the
starting value; `Accept` re-replays from the earliest accepted; the Edit row action is not stage-gated). Live proof exists only from a blocked review
(Run 2 step 8: second `Edit`, "Accepted changes: 2", `ready`). Missing: the clean-`ready` variant and a final-head assertion with both drafts finalized together.

**(b) Warnings in history after finalize and reload.** Durable per-mutation outcomes come from the store message ledger (`mutation_outcomes`), rendered in words
by `history_mutation_description`; reload recomputes them by replay. Live proof of rendering exists (Error rows; a post-reload Warning). Missing: a produced-by-edit
warning carried through finalize and reload; no "new vs existing" marker.

**(c) Fatal resolution loop.** Live and green via Withdraw (blocked review, Finalize disabled with reason, Next problem opens the failing mutation, ready). The
real intent, fixing the failing mutation's inputs (for puzzle 2d: re-point targets with Use selection), is untested at every level. Also confirm the Error-blocks
reading (G14).

**(d) Metadata rendered in both shells.** React live: stepper (step/precision, keyboard arrows). Slider+snaps: unit/corpus-tested in React and wgpu, not live.
Dial: the editor builder discards snaps/unit/displayFactor; `scale: log` is not consumed anywhere in the editor path. wgpu: nothing live.

**(e) Selection editable via "use selection".** Control renders (live); the verb has no test, chips show ids, no board highlight of the referenced nodes.

**(f) Alternatives.** Create-and-switch between named alternatives is live (React). Trunk listing/switch landed afterwards (unit). Not live: wgpu, trunk,
per-viewer heads.

**(g) Artifact-agnosticism.** The mechanism is framework runtime code (`PLG/⏪️time-travel`, `PLG/🛠️tool-machine`, store, replication), tested with a toy app and
with gis, writer and flow-child sessions through the real verbs. Per-plugin replay laws exist for wfc, layout, puzzle 3d/5d, process3d, cad. Live: puzzle 2d only.
Generic holes: raw-op-text labels (R04), no editability gate (R03), 8 `Emit::amend` holdouts (R36).

**(h) wgpu live e2e.** Never run. Prerequisites landed: `dumpBoard2d` introspection, notices in the ARIA mirror, rejection codes, heartbeat publishing `historyEdit`.
Activation `activate-puzzle2d-wgpu-dev` / `serve-puzzle2d-wgpu-dev` (6112) was held under load and never started. Known risk: W2-C reports the wgpu shell test module
cannot run in one process (dozens of order-dependent laws; "pass one per process", pre-existing).

**(i) Persistence/reload.** Unit-level strong; live FAIL in Run 2 (rows lost, `local.backbone-scope-mismatch` on folder-bound edits). Fixes landed in W2-B follow-up 3/4
and W2-A; none re-probed. Native wgpu folder re-attach open.

**(j) Collaboration.** `BaseMoved`, remote-supersession convergence, hub admission/grading, presence: unit/law-tested; never live. Hub crates not confirmed green. Per-viewer
head is a separate ticket.

**(k) 64-slot ledger.** A hard cap of about 60 gestures per document; no compaction; four ledgers; synchronous replay budgeted for it. Time-travel itself consumes no edit slot
(overwrite = one transition), but "new alternative" consumes alternative + checkpoint + change slots. Everything long-session-related (progress/cancel on `dry_run` and
`reproject`, prefix-ring stride, 8192 DAG capacity, 64-item envelope decode) depends on that sub-ticket.

**(l) Progress and cancellation.** Implemented for the session replay (band progress, Cancel, Replay again, Exit cancels, wgpu progress frames); unit-tested; never seen
live. Not implemented for finalize authoring (`dry_run`), remote reprojection, or preview folding.

**(m) i18n, accessibility, keybindings.** en/de complete in the unit/corpus layers; React band/indicator/stepper live in en and de (de chords live). Not verified: focus
management, en chords, wgpu mirror live, mobile/tablet. React lacks the wgpu-style auto-reveal of the History panel.

## 3. Prioritized gaps that block "everything end to end"

Ordering: P0 blocks any honest "done"; P1 is required by the goal text; P2 completes artifact-agnostic quality; P3 hygiene.

### P0

**G1 — Re-establish live evidence on the current tree (owner: evidence/coordinator).**
The fleet died mid-edit; activations #5 and #6 failed (puzzle-2d `Puzzle2dCommand::from_action`, then taxonomy/nx-plugin loads from peers). Work: complete interrupted edits
compile-atomically, then one activation of React 6012 and wgpu 6112, then the probe.
Acceptance: `bun T/🔍️time-travel-probe.ts --port=6012 --locales=en,de` gives 0 uncaught, 0 hard faults, and the three Run 2 FAILs turn green
(`document-rows-survive-the-reload`, `overwrite-row-survives-the-reload`, `trunk-listed-after-new-alternative` + both trunk-switch verdicts); the console digest loses
`hostEvent … undeclared`, `commitCheckpoint refused`, `local.backbone-scope-mismatch`.

**G2 — wgpu shell live e2e (owner: wgpu + evidence).**
Work: boot 6112 (`--explore` first to calibrate mirror keys), run `--renderer=wgpu --locales=en,de`, fix what breaks (history body tree rows, editor controls, dialog, band
progressbar, folder re-attach native/browser). Resolve the order-dependent wgpu shell test module so the suite is a usable gate.
Acceptance: the same verdict names as React pass for steps 1-9 in both locales on wgpu (renderer notes recorded per `📓️w3-e2e-report.md` W.2), 0 uncaught errors.

**G3 — Fatal loop by EDITING inputs, and selection editing (owner: runtime + puzzle2d + evidence).**
Work: unit tests for `draft_time_travel_selection` (domain absent, granularity mismatch, empty selection, `many`/`max_items`, id types), chip removal, and a runtime law
"edit targets -> replay -> report"; node labels (not raw ids) on chips and a highlight of the referenced nodes in the preview; chip overflow row beyond 8.
Acceptance (live, both shells, en+de): delete an upstream node -> the downstream drag reports `mutation.target-missing` -> review `blocked` -> Next problem opens the drag ->
select two remaining nodes on the board -> Use selection -> chips show the nodes' labels -> Accept -> `ready` -> Finalize overwrite -> head equals expectation. Withdraw is no longer the only resolution path.

**G4 — Warning flow end to end (owner: evidence + store/runtime).**
Work: a puzzle 2d scenario where an upstream edit makes a downstream mutation emit a Warning (for example a locked or missing member -> `mutation.partial`); optionally a
"new since this edit" marker on rows (kernel `HistoryMutationEntry` gains `introduced`).
Acceptance: review `ready` with the warning row visible (word + localized code), Finalize overwrite, the row still shows the warning, folder reload and re-attach, the row still shows it (en+de).

**G5 — Reload persistence re-proof (owner: store/React shell/wgpu).**
Acceptance: Run 2 step 5 passes: after reload + re-attach every pre-reload row (drags, overwrite, undone/redone, both alternatives, clone/drag) is back with its LOCALIZED label (no
`create-node node {…}` op text), the alternatives list holds trunk + both alternatives, the current alternative is restored. Native wgpu re-attach parity decided and tested.

### P1

**G6 — Input-metadata rendering completeness (owner: UI contract + runtime panel + React + wgpu).**
Work: Dial arm passes `snaps`, `unit`; add `displayUnit`/`displayFactor`/`scale` to the contract slider/dial (Rust + TS + generated contract + React `Slider` + wgpu slider) or
convert in the panel; stepper snaps/detents where present; hard min/max refusal visible.
Acceptance: unit corpus cases for dial-with-snaps, log-scale slider, display factor (Rust + TS twin, third-party oracle for the number mapping); live in both shells: after a
rotate gesture, Edit -> angle dial shows ticks at 0 and +/-90/180 degrees, reads degrees, ArrowKeys step 1 degree, snap lands on 90; after a scale gesture, Edit -> factor slider shows
ticks at 0.25/0.5/1/2/4 on a log axis; typing -5 for a factor shows a refusal naming the bound and keeps the draft.

**G7 — Generic localized labels for every plugin (owner: runtime + all plugins).**
Work: default `ArtifactApp::mutation_label` from `SemanticMutation::label` (remove the 8 overrides and the `None` default); add a repo gate that fails when any applied leaf
of any plugin resolves to the `print_op` fallback or lacks a de label.
Acceptance: gate green repo-wide; a history-row law per plugin family shows en + de labels; re-probe of the reload label (G5).

**G8 — Editable-everything gate (owner: runtime + schema).**
Work: enumerate every `Mutation` aggregate (including hand-written/generic ones that have no emitted payload law) and assert `input_schema().is_some()` and the generic payload
round-trip, or a declared, tested non-editable reason (`may_emit_foreign_steps`); run the final strict `schema mutation-inputs` and `schema mutation-payloads` repo-wide.
Acceptance: both lints 0 repo-wide, gate green, list of non-editable leaves with reasons committed in the ticket.

**G9 — Long histories (owner: store; coordinate with sub-ticket `PAGED-ARTIFACT-HISTORY-LEDGER`).**
Work (this ticket's share): when the ledger lifts, make `dry_run` and `reprojection_replay` resumable (job with progress and cancel), size the prefix ring, and add the
live replay-progress scenario that was impossible with 2-3 mutations; until then document the ceiling in the UI (a refusal notice for the 65th edit that names the cause).
Acceptance: a fixture document with 200 gestures; Edit mutation #3; Accept shows `replaying` with progress and a working Cancel (Replay again afterwards); Finalize overwrite
and as new alternative succeed; remote ingest of a supersession on that document shows progress and does not freeze the app.

**G10 — Collaboration live (owner: evidence + hub + the per-viewer-head ticket).**
Work: hub-backed two-peer serve; rerun the hub crates (`semio-hub`, `os-hub`) on the final tree; sequence the finalize flow with `PER-VIEWER-ALTERNATIVE-HEAD`.
Acceptance: peer B edits upstream while A reviews -> A's band restarts the replay (BaseMoved) and finalizes on the moved base; both heads converge after A finalizes overwrite;
a new alternative from A does not switch B's head (after the per-viewer ticket); ⏪ badge and row notes appear for B.

### P2

**G11 — Close the tool-machine conversions and the central gates (owner: tool conversions + coordinator).**
Work: finish verification of W3-T-PUZZLE (3d/5d), W3-T-SPATIAL, W3-T-LAYOUT, W3-T-FLOWCAD; convert the 8 `Emit::amend` holdouts and the unconverted plugins (lowpoly, block,
space, trinity, mathematical, sequence, animate, procedural 2d, forms strokes); delete `Emit::amend` and static coalesce keys (W3-T2-CLOSURE); run W3-G (descriptors, launch rows,
`schema generate`, taxonomy, dependencies, layering, docstrings, warnings, hub and energy crate checks).
Acceptance: repo gate "no `Emit::amend`, no static `coalesce_key`"; per-plugin law "one gesture = one edit = one row with `TransactionRef`; cancel = zero trace; edited leaf replays
deterministically"; `verify dependencies literal-external` and `schema check` green.

**G12 — Cross-plugin acceptance harness (owner: runtime/testkit).**
Work: one generic runtime law that, for a representative editable leaf of every plugin, runs begin -> input -> accept -> finalize overwrite and finalize alternative and compares
the head with a fresh fold; puzzle 2d gets its own `state_before`/`begin_report_replay` law like its siblings.
Acceptance: law runs in each plugin crate's test target; failures name the plugin and leaf.

**G13 — Shell parity, accessibility and devices (owner: React shell + wgpu + a11y).**
Work: React auto-reveal of the History panel on the edge into a session (parity with wgpu); focus moves to the first editor control on Begin and to the dialog/band on stage
changes; probe drives en chords (`--chords=en,de`); mobile and tablet viewport pass (band `max-w-[90vw]`, editor in compact panel); wgpu mirror verified live.
Acceptance: probe verdicts `history-panel-reveals-on-session-start` and `focus-moves-to-the-editor` pass on both renderers; a mobile-viewport probe pass (375 px) shows band + editor +
dialog reachable by touch and keyboard; no axe-level violations on the band, editor and dialog.

### P3 and decisions

**G14 — Confirm the blocking rule with the dev.** Design decision 2: Error and Fatal block finalizing (matches hub check-in strictness `MergePolicy::Normal`). The goal text names only
"fatal errors" as blocking. If Errors may be finalized, the overwritten history would be rejected by a hub check-in; recommend keeping the design and saying so explicitly in the UI
copy. Acceptance: decision recorded in `📋️design.md`; if reversed, `ReplayReport::blocks_finalize`, the reducer law fixture, both shells' band copy and the hub rule change together.

**G15 — Small items.** Chips >8 and rows >8 "show more" affordances; `historyEditUseSelection` MCP description; N-1/N-3/N-4/N-6/N-8 of `📓️w1-b-report.md` and N-10..N-13b of
`📓️w1-c-report.md`; hub trusted-catalog 500 on hub-less boots (fixed in W2-B f3.6, unverified); dev reload `useShellScope` identity law (fixed, unverified live).

## 4. Evidence inventory (what "live" means here)

- Live runs: `T/🗑️generated/e2e/probe-2026-09-30T16-49-01` (en) and `…16-51-11` (de): PASS 85 / FAIL 3 / uncaught 0 / hard 0 each (Run 2, real controls only); regression
  `…19-43-21` en steps 1-3: 26/26. All against the activation #4 build. Run 1 (bypass) 117 PASS / 27 FAIL is obsolete.
- Probe: `T/🔍️time-travel-probe.ts` (92 `verdict(` call sites, steps 1-9, `--renderer=react|wgpu`, `--folder-at`, `--chords`, `--explore`). Not covered by any step: slider/dial controls,
  use-selection, warning outcomes, replay progress/cancel, multi-draft from a ready review, hard-bound refusal, en chords, mobile, second peer.
- Unit/law evidence by layer: replication (Supersede codec, fold, report types), time-travel and tool-machine modules (Rust + TS with ajv/xstate/fast-check), store
  `🧪️supersede-replay` (24 tests), plugin `🧪️time-travel` (24 tests incl. replicas, undo/redo of finalize, alternatives, host events), React `⏪️time-travel` component suite (17) +
  command-rejection + folder-archive-restore, wgpu `🧪️wgpu-time-travel` (21-22) + presence (15) + vitest (86), UI contract corpus (slider-with-snaps, stepper, dialog-choices, tree-row-recipes),
  puzzle 2d `🧪️select-tool-transactions`.
- Caveats on all of it: reports of 02:46–03:21 are the last confirmed green points; after 07:23 twenty agents resumed and died before reporting (W3-T-PUZZLE verification, W2-D
  re-verification, W1-G follow-up 5 `open_transaction_edit` (§15, present in `STORE` ~19810, unverified), W2-C native re-attach, W2-A reload label). Peer-owned breakages seen in the log: SQLite
  snapshot codec rollout, taxonomy ordering, nx plugin load, `os-kernel` derive errors; re-check each before trusting a "green" claim.
