# 📓️ S5 goal audit 2 (read-only, code on disk, 2026-10-05 09:40-10:00)

Auditor: S5-GOAL-AUDIT-2. Method: every `path:line` below was read or grepped on disk during this audit (`/usr/bin/grep -rn`, `python3`
censuses over the file tree); report files were used only to locate code and to read the live-run ledger. Nothing was compiled, run or
edited (only this file was written). Verdict words: MET / MET ON REACT ONLY (mechanism on disk + live proof on React, none on wgpu) /
PARTIAL / NOT MET. UNVERIFIED = not confirmed by reading. The tree moves under the audit (peers save continuously): S5-PUZZLE landed
design §22.13 (connect-handles `tolerance`, `mutation.precondition-drifted`) at 09:52 while I was reading; that is called out where it matters.

Abbreviations: `FW` = `🧰️framework`, `OS` = `FW/🛍️products/💻️os/🔨️modules`, `TT` = `FW/🔨️modules/⏪️time-travel/🦀️.rs` (pure reducer, TS twin
`TT/../🟦️.ts`, 610 lines), `PTT` = `OS/🔌️plugin/⏪️time-travel/🦀️.rs` (4503 lines), `PLUG` = `OS/🔌️plugin/🦀️.rs`, `STORE` = `OS/🏪️store/🦀️.rs`,
`MAN` = `FW/🔨️modules/🛂️manifest/🦀️.rs`, `TM` = `FW/🔨️modules/🛠️tool-machine/🦀️.rs`, `R` = `OS/📺️renderer/🧑‍🎨engine/🧱️elements`,
`P2D` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any`. Line numbers are of the files as read today (they moved since the
10-04 audit).

---------------------------------------------------------------------------------------------------------------------------------------

## 1. Headline

- 16 clauses: **MET 1**, **MET ON REACT ONLY 10**, **PARTIAL 5**, **NOT MET 0**. No clause is proven live on wgpu (no wgpu probe output exists; the first
  wgpu serve came up 07:26 on :6112 and nothing was run against it), none in German in session 5, none on a plugin other than puzzle 2d.
- Closed since the 10-04 audit (verified in code): framework-owned `Frozen` mapping + `GestureLedger` (G2.2), `Multiline` control kind in the manifest
  reader/guest/React (G5.3, wgpu half missing), `widget: "dictionary"` gone (G5.2), declaration gate rules (G5.4), one inverse failure = one Fatal
  mutation (G6.2), `nextProblem` on the wire + band control (G7.2), Withdraw on every row via `withdrawable` (G7.3), both `.expect(` on the user path
  converted, per-viewer alternative head (G9.1, `REC_VIEWER`), band copy single-sourced in the corpus `labels` table (wgpu embeds it), wgpu mirror value path
  through the number law, wgpu row tone + `set_size/pos_in_set`, `precondition-drifted` producer (landed 09:52, unproven).
- Still open and most damaging: (1) the newest live probe on disk (Run 6, build B1, 09:48-09:52) FAILS the folder reload (edit ids, document rows and the
  overwrite row are lost; uncaught `actor-document-control.receipt-count`) — a regression against Run 5/B0 (67/0); (2) wgpu and `de` have zero live proof;
  (3) the cross-plugin acceptance law passed for 2 of 96 crates this session; (4) tools-as-transactions is adopted by 26 of 99 artifacts and 4 interactive
  tool families bypass it; (5) §22.11 cost work (O(n) digests per keystroke, linear mutation lookup, repair replays from the first draft) is not landed.

---------------------------------------------------------------------------------------------------------------------------------------

## 2. Clause table (16 clauses)

Live-proof sources: `T/📓️w3-e2e-report.md` § S5.2/S5.7 (React `en`, build B0: batches A-J 376 PASS / 10 FAIL, then hot-reloaded host fixes), and the raw files
`T/🗑️generated/s5-e2e/run6-react-en-{A,B}.txt` (build B1, channel 22) which the report does not yet contain. B0 step ids are the probe's (A = steps 1-7+9 ...).

| # | Clause | Mechanism (verified) | Law / test (verified to exist, not run) | Live | Verdict |
|---|---|---|---|---|---|
| 1 | non-destructive editing of artifacts | original op never rewritten; `HistoryTransition::Supersede{inputs[{target, replacement}]}` (`FW/🔨️modules/📡️replication/🔗️causal/🔀️transition/🦀️.rs:125` family); fold resolves `EffectiveSupersession`; law `admit_replacement` `STORE:23776` (fatal no-op when the replacement breaks the schema) | `OS/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs` `interior_supersede_equals_a_fresh_replay_of_the_edited_log:145`, `withdrawing_an_operation_folds_it_as_a_no_op:256`, `an_input_breaking_the_supersede_law_folds_as_a_fatal_no_op_at_every_site:628`; `🧪️supersede-law` `every_input_row_is_admitted_and_folded_as_the_table_says:171`; oracle `fast-json-patch`, TS fold twin | B0 A5/A6 undo/redo, E13 | **MET** |
| 2 | a mutation in the history must be editable | `history_mutation_view_of` `PTT:1705`: `editable = !viewer && op.unit.is_none() && !may_emit_foreign_steps() && input_schema().is_some_and(time_travel_schema_shows_inputs)`, `withdrawable` separate; open `PTT:746`; session `TT:542 apply`; verb `historyEditBegin` (`MAN:2656`) | `OS/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs` `every_mutation_of_a_long_transaction_is_reachable_and_edit_follows_the_begin_law:551`, `a_mutation_row_names_why_edit_or_withdraw_is_refused:2448` | React B0: A3, C8/10, D12, E13, G16 (374th mutation of one transaction opens), J19 — puzzle 2d only | MET ON REACT ONLY |
| 3 | UI jumps into a time-travel mode when history edit starts | pure reducer `TT:125 TimeTravelStage` (Inactive/Editing/Replaying/Reviewing/Choosing/Finalizing), `TT:254 TimeTravelEvent`, `TT:419 TimeTravelEffect`; ledger `PTT:329/431/436 status()` -> wire `HistoryPatch.timeTravel`; freeze: `freeze_tool_machines` `OS/🔌️plugin/🛠️tool-machine/🦀️.rs` + `time_travel_busy PTT:2824`; React band `R/🛠️ShellHelpers/⏪️time-travel/🟦️.tsx` (`role=status aria-live=polite` :433, indicator `role=note` :377); wgpu `R/🐚️Shell/🎯️targets/🧊️wgpu/⏪️time-travel/🦀️.rs` (`time_travel_band_controls :143`, status node ~:975, indicator note :1142) | law fixture `TT/../🧫️fixtures/🧫️lifecycle-law/🔣️.json` (Rust `🧪️tests/🔬️unit` + TS `🧪️tests/🧪️conformance`, xstate/ajv/fast-check oracles); plugin `an_open_session_freezes_document_verbs_and_refuses_stale_or_invalid_drafts:375`, `opening_a_history_edit_and_a_remote_edit_deliver_host_events_to_every_window:1208`, `a_stage_change_ships_its_generation_in_the_same_turn_and_a_verb_stamped_with_it_applies:1128`; band corpus `R/🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band` (55 labels, en+de x normal+beginner, 0 gaps) | React: A3 (band, indicator x3, focus), F14/15; wgpu not probed | MET ON REACT ONLY |
| 4 | current mutation shown, downstream not yet applied | preview = state before target + accepted upstream drafts + draft folded once: `PTT` `TimeTravelStoreState::preview` -> `STORE:19355 state_before` -> `STORE:18162 derive_history_snapshot`; render seam `render_snapshot_or PTT:287` (4 call sites `PLUG:35774/35811/35856/35894`); downstream row `pending` dimmed (`history_mutation_entry PTT:1052`) | `STORE` `state_before_answers_the_projection_before_an_operation_under_drafts` (`🧪️supersede-replay:417`), `prefix_ring_evictions_retire_through_the_snapshot_factory:469` | React: A3, C8 ("Not applied while editing"); two-peer case F4 fixed on the attach wave (H 27/0) | MET ON REACT ONLY (cost PARTIAL, see gaps) |
| 5 | finalize or discard the input change (accept / discard) | events `Accept`/`Discard` (`TT:254`), verbs `historyEditAccept/Discard` (`MAN:2666/2668`), band chords, guest rows `PTT:4064+` | corpus + reducer fixture; `a_remote_edit_while_editing_keeps_the_draft_and_accept_replays_it_too:2198` | React: A4 chord, D11 button+chord, F touch, G16 | MET ON REACT ONLY (Accept is always enabled, G4.1) |
| 6 | every input has UI-element info (slider, stepper, min, max, snapping points) | `ActionArgDef MAN:679`, `ArgSchema::{Number,Vector,...} MAN:168`, `ArgPresentation MAN:339`, `SnapSource MAN:357`, `ActionArgControl MAN:466`, `x-semio-ui` strict meta-schema (widget enum `MAN/../🧬️schema/🔣️.json:1179`), reader `mutation_input_defs`; guest row builder `time_travel_input_row PTT:4187` (see section 3a) | reader conformance fixture `FW/🔨️modules/🛂️manifest/🧫️fixtures/🧫️mutation-inputs` (Rust + TS + Python), `🧫️number-controls` corpus; plugin `snap_sources_resolve_from_the_grid_factor_and_the_previewed_document:847`, `colour_inputs_render_the_contract_recipe_and_vector_axes_keep_their_snaps:924`, `the_band_and_the_editor_rows_hold_at_most_one_single_control:988` | React B0: A3/4 steppers+snap, C8 hard-min refusal, C10 dial ticks/degrees/log slider, D12 references, J19 lists; wgpu not probed | **PARTIAL** (vocabulary + both renderers OK; 546 bare stdio inputs; snaps are bare numbers; wgpu has no multiline law) |
| 7 | Accept re-applies all downstream mutations (replay, progress, cancel) | `settle TT:811` -> `StartReplay`; stepped Report replay `step_time_travel_replay PTT:3096` (4 ms wall slices `PTT:30`, progress throttle), `CancelReplay`, `Rerun`; store `begin_report_replay STORE:19387`, `replay_window :20706`, convergence `EditReplay::converge :24429` | `STORE` `cancelling_a_report_replay_at_every_step_leaves_the_store_untouched:344`, `convergence_early_exit_equals_the_full_replay:380`; plugin `a_deferred_history_step_ends_its_turn_at_the_wall_deadline_not_an_operation_count:2094`; puzzle `history-edit-runtime` `replay_progress_rides_the_ui_frames_over_a_long_downstream:360`, `switching_to_a_long_alternative_replays_over_turns_and_cancel_leaves_zero_trace:471` | React B0 G16: 766-mutation replay shows progress, Cancel -> "Replay again", Edit refused during replay | MET ON REACT ONLY |
| 8 | succeed / warn / error; new warnings visible in the history | `OutcomeCode` (11 codes, `FW/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1032..1064`), per-mutation `MutationReplayOutcome`, `introduced` flag, row tone + "New since this edit"; inverse failure = one Fatal `mutation.inverse-refused` (`STORE:24324 replay_operation`, :24364) | `supersede-law` `a_refused_inverse_is_one_fatal_mutation_and_the_session_stays_repairable:302`; `downstream_warning_error_and_fatal_outcomes_are_reported_per_mutation:217` | React B0 E13 (Warning "Partially applied", survives reload on B0), C8/D12 Error; B1 reload RED (clause 16) | MET ON REACT ONLY |
| 9 | fatal -> problematic mutations must be edited first, repeat until error free | `TT` `RequestFinalize` needs a non-blocking report; Finalize disabled naming why (`finalizeDisabledBy`, React `.../⏪️time-travel/🟦️.tsx:~75`); `next_problem PTT:431` -> `status.next_problem{mutationId, store}` -> band control `nextProblem` (React `timeTravelControlActionV1`, wgpu `TimeTravelVerb::NextProblem`); Withdraw on any row (`withdrawable PTT:1746`, reducer `BeginWithdrawn`) so non-editable blockers are resolvable | plugin `a_blocking_mutation_without_editable_inputs_is_withdrawn_and_the_review_becomes_ready:2609`, `a_mutation_row_offers_withdraw_and_restore_takes_an_accepted_draft_back:2397`; puzzle corpus scenario `fatal-and-error-resolved-then-overwritten-undone-and-redone` | React B0: C8 (blocked -> Next problem -> Withdraw -> ready), D12 (blocked -> Use selection -> ready); row-Withdraw step 20 (batch K) WRITTEN, NEVER RUN | MET ON REACT ONLY |
| 10 | final result visible; finalize or keep changing other mutations | stage `Reviewing` shows the replayed head; `Begin` from review stacks drafts (`return_stage`); `Restore` event `TT:261` (§22.16) | reducer fixture (22 `restore` rows); `a_remote_edit_while_editing...:2198` | React B0 D11 (two accepted drafts -> one overwrite row "2 mutations") | MET ON REACT ONLY |
| 11 | on finalize: prompt new alternative vs overwrite | `OpenFinalizePrompt` effect `PTT:2732` -> `history_edit_finalize_dialog MAN:2843`; `commit_time_travel` -> `commit_finished_replay STORE:21545` (atomic) ; per-replica viewed alternative (`REC_VIEWER` `OS/📡️spr/📜️history/🦀️.rs:1174`) | store `a_new_alternative_preserves_the_trunk_it_branched_from:833`, `a_viewer_head_round_trips_through_the_spr_pack:901`; `🧪️viewer-head` | React B0: A5 (Overwrite), A7 (New alternative + switch both ways), B/E reload restores rows+alternatives; **B1 reload FAILS** | MET ON REACT ONLY (B1 persistence red) |
| 12 | puzzle 2d: drag -> drag mutation; selection AND offset editable afterwards | leaf `✋️drag-selection` (section 3c); select tool machine `P2D/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs:76` yields `drag_selection(targets, dx, dy)` inside the transaction (:189-190, :214-234) | `P2D/✏️editor/🧪️tests/🧪️select-tool-transactions/🦀️.rs` `one_board_drag_is_one_edit_one_row_and_one_transaction:87`, `a_drop_and_its_connection_are_one_transaction:138`, `a_cancelled_drag_leaves_zero_trace:160`; `🧪️history-edit-runtime` corpus | React B0: A2-4,7 (offset dx -> 120 + Accept), D12 (targets via "Use selection") | MET ON REACT ONLY |
| 13 | tools are state machines yielding mutations within a transaction | `ToolYield TM:19`, `ToolTransaction TM:167`, `ToolMachine TM:242`, `ToolMachineRunner TM:287`, `GestureTool TM:485`, `drive_gesture TM:514`, `GestureLedger TM:740`, `ScrubMachine TM:926`, `node_drag_emit TM:1307`, `TypingMachine TM:1547`; runtime `OS/🔌️plugin/🛠️tool-machine/🦀️.rs`; identity `TransactionRef` (`FW/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1545`) | `TM/🧪️tests/🔬️unit/🦀️.rs` `press_drag_release_is_one_transaction_and_escape_leaves_zero_trace:475`; plugin `a_committed_tool_transaction_is_one_row_with_its_reference_and_mutation_rows:458`, `a_streamed_tool_transaction_is_one_row_and_its_abort_leaves_none:616` | React B0: A2, C10, J19 (puzzle 2d only) | **PARTIAL** (26/99 artifacts; 4 offender families, section 3d) |
| 14 | tools are interactive, not alterable by history; their mutations are | tool has no history row (one row per transaction, mutation rows editable); abort reasons incl. `Frozen` mapped by the framework (`GestureHostEvent::TimeTravelFrozen TM:556,585`, `freeze_tool_machines`) | same as 13; `frozen` law `an_open_session_freezes_document_verbs...:375` | React B0: A3, C8, J19 (undo skips the camera) | MET ON REACT ONLY |
| 15 | clean artifact-agnostic mechanisms that work for any artifact | core (reducer, ledger, store, wire, band corpus) names no plugin in code; one framework hook set (`entity_label`, `tool_intent_kinds`, `typing_fold`) | cross-plugin law `OS/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs` (`history_edit_acceptance_law!` in 115 files / 93 artifact crates / 31 plugins) | **this session 2 of 96 crates PASS (puzzle 2d, 3d); 94 NOT RUN** (`T/🗑️generated/s5-agnostic/acceptance-results.tsv`); no second artifact in a browser | **PARTIAL** (section 3e) |
| 16 | everything end to end | probe `OS/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts` (~5.1 k lines, 19+ steps, en/de, phone/tablet, React/wgpu arms), launch rows `⚖️gate⏪️time-travel⚛️react` (:6012) / `...🧊️wgpu` (:6112) | probe self-asserts; i18n/a11y laws (section 3f) | React/en B0 376/10 (7 product findings F1-F7, F1/F2/F4/F5/F8 closed on the hot-reloaded host/`attach` wave); **B1 Run 6: A 75/1, B 63/7 + 2 uncaught (reload red)**; React `de` never; wgpu never | **PARTIAL** |

---------------------------------------------------------------------------------------------------------------------------------------

## 3. Special questions

### 3a. "Snapping points on a slider" — exact vocabulary and renderer support

Carried explicitly (not only `step`). Fields, verified on disk:
- Schema side (`x-semio-ui`, strict key list `INPUT_UI_KEYS MAN:1393`; number keys `MAN:1394`, vector keys `MAN:1397`): `snaps` (array of finite numbers, each must lie inside the
  hard bounds, `MAN:2012-2019`), `snapSource` (`{step:true}` | `{config:"<key>"}` | `{snapshot:"<json pointer>"}`, `MAN:2020-2027`), plus `step`, `precision`,
  `softMin`/`softMax` (slider travel narrower than the hard bounds), `scale: "linear"|"log"`, `unit`, `displayUnit`, `displayFactor`.
- Rust types: `ArgSchema::Number { snaps: Vec<f64>, snap_source: Option<SnapSource>, soft_min, soft_max, step, precision, scale, ... } MAN:168/195-230`; `ArgSchema::Vector { snaps, snap_source, ... }`;
  `SnapSource::{Step, Config{key}, Snapshot{pointer}} MAN:357`; `ActionArgControl::{Slider, Dial, Stepper, Vector}` all carry `snaps` + `snap_source` (`MAN:523-611`, `control() MAN:872-905`);
  `ActionArgNumberFacets.snaps MAN:1097` (the single mapping both renderers and both shells' staged dialogs share; out-of-range / limit-crossing / unsorted snaps are dropped, `MAN:1171`).
- Guest function: `time_travel_input_row PTT:4187` -> `time_travel_slider PTT:4136` (`builder.try_snap(f64)` per snap :4153; slider AND dial go through it :4341), stepper :4350/:4370,
  vector :4437/:4451, number input :4383. `Config`/`Snapshot` sources are resolved to numbers before the row is built: `resolve_time_travel_snaps PTT:2315` ->
  `time_travel_resolve_snaps :1338` -> `time_travel_apply_snap_spacing :1462` (multiples inside the travel become detents when at most `UI_FIXED_LIST_ITEMS = 32` fit, else the
  spacing becomes the step; `FW/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:21`).
- Do NOT exist: labelled marks (a snap is a bare number: `try_snap(f64)`), per-input snap radius or strength (one global `SLIDER_SNAP_RADIUS` in `slider_pointer_value`,
  `FW/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:812`), more than 32 detents, a `marks`/`detents` key name.
- React: `🎚️Slider/🟦️.tsx` `snapValues` (:86, :229): pointer snapping (`sliderPointerValue` :389), page keys stop on the next snap (`sliderKeyValue` :450), ticks drawn on track (:540) and dial (:496);
  `🪜️Stepper` `snapValues` (:55) -> page keys only (no ticks); Interpreter wires `component.snaps` (`R/🗣️Interpreter/🟦️.tsx:1699` slider, `:1738` stepper, typed-value candidates :1485/:1555).
- wgpu: track ticks `slider_tick_rects` (`FW/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:3139`), dial ticks `dial_tick_lines` (:3125), pointer `slider_node_pointer_value`
  (`⚡️events/🦀️.rs:708, 3277`), keys `slider_key_value_from :734`, typed values keep exact detents (`constrain_slider_value :629`), stepper/number input keep `snaps` for page keys
  (`🔀️reconcile/🦀️.rs:939/1011/1034`); both the canvas path and (since wave 1) the AT mirror path go through `typed_number` + refusal (`⚡️events/🦀️.rs:2659-2685`).
- Shared law corpus `FW/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧫️number-controls` (React component tests, contract unit, wgpu events). Live: React B0 C10 (dial ticks at 0/90/180 deg, log slider ticks);
  wgpu never run.
Verdict (a): MET for numeric snap points on both renderers; NOT present: labelled marks and per-input radius. A stepper shows no ticks on either renderer (keys only).

### 3b. Puzzle 2d: explicit UI metadata per mutation input

Census (python over the 36 leaf payload schemas `P2D/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json`, read 09:52): **104 non-discriminator inputs, 0 lack `x-semio-ui`**.
Controls declared: 29 `stepper` (19 with `snapSource` `{config: gridFactor}`), 7 `dial` (all with `snaps`, `softMin/Max`), 2 `slider` (log, `snaps`, `softMin/Max`), 9 `toggle`, 16 `text`, 33 `reference`,
2 `select`, 1 `segmented`, 5 label-only record inputs (below). Min/max: hard bounds from JSON Schema (`minimum`/`exclusiveMinimum`/`minItems`); units on dials (`rad`->`deg` with `displayFactor`,
`deg`), `gap/shift/rise` (`m`).
Not "bare", but inferred-default or snap-less:
- Label-only record inputs (no `widget`; the guest flattens an object into its field rows, `time_travel_input_rows PTT:1278`; the fields inside are 16/16, 9/9, 8/8, 4/4 declared):
  `🌱create-node/🧬️schema/🔣️.json:24 node`, `➕add-node-handle/…:47 handle`, `🌍create-target-region/…:24 targetRegion`, `🔌replace-node-handle/…:70 newHandle`,
  `📚replace-kind-catalogs/…:23 newCatalogs` (arrays of catalog kinds -> list rows; nested catalog-kind fields UNVERIFIED).
- Numeric inputs that declare neither `snaps` nor `snapSource` (10): `index` of create-node / add-node-handle / create-target-region (integer stepper, `minimum 0`),
  `newGap/newShift/newRise` (replace-edge-geometry), `gap/shift/rise` and the new `tolerance` (connect-handles `:302`, landed 09:52): each has `step`+`precision`(+`unit`) but no detents.
- `drag-selection.dx/dy`, `move-node.newX/newY`, pivots: stepper with `snapSource gridFactor` and NO min/max (unbounded stepper; the grid spacing becomes the step on an unbounded range).

### 3c. The drag example

Leaf `P2D/🧬️schema/🧬️mutations/✋️drag-selection/🧬️schema/🔣️.json` (3 inputs):
- `targets`: `array<string>`, `minItems 1`, `x-semio-ui {widget: reference, role: target, ref: {kind: [node, targetRegion], domain: "vortex", granularity: "node"}}` -> `ActionArgControl::Reference`
  -> guest Reference row (`PTT:4260`): chips, "Use selection" (`historyEditUseSelection`, `PTT:4272`, `MAN:2661`), remove, `min_items`/`many` honoured.
- `dx`, `dy`: `number`, `x-semio-ui {widget: stepper, role: value, step: 1, precision: 2, snapSource: {config: gridFactor}}` -> `ActionArgControl::Stepper` -> `number_stepper` (`PTT:4350`); React `NumberStepperView`
  (`Interpreter:1738`), wgpu `NumberStepper`.
Editable after the drop: the leaf is a `#[mutation_leaf(payload = ...)]` variant with an input schema, so `editable` is true (`PTT:1705`). The same machine yields `rotate-selection` (dial `angle`, pivot steppers) and `scale-selection`
(log slider `factor`). Proven live on React B0 (A2-4,7; D12). The recorded proximity `connect-handles` now states its `tolerance` and warns `mutation.precondition-drifted` on a drifted base
(`.../🪢️connect-handles/🔺️diff/🦀️.rs:40-49`, tool side `select/🦀️.rs:470-471`; unit test mention `🧪️tests/🔬️unit/🦀️.rs:109-131`) — landed 09:52, not run, no live step.

### 3d. Tools as state machines within a transaction (whole repo)

Method: 99 artifact directories under `✏️s/🔌️plugins/*/🗿️artifacts/*`. (i) `python3`/grep over non-test `.rs` for machine/transaction vocabulary (`ToolMachineRunner`, `GestureTool`, `drive_gesture`, `gesture_emit`,
`admit_gesture_slot`, `node_drag_emit`, `node_graph_edit_rows`, `commit_/stream_/abort_transaction`, `ToolYield`, `ToolTransaction`, `typing_fold`); (ii) `UtilityDefinition::new(` per artifact = the declared interactive tools
(19 artifacts); (iii) per-offender reading. Not a read of every tool body: pure command-driven apps (norm, energy, stdio importers, block 2d/5d, forms ...) declare no tool utility and were not judged.
- **26 of 99 artifacts carry tool-transaction machinery**: 13 own a `ToolMachineRunner` (puzzle 2d/3d/5d, note, raster, drawing, cad, layout, lowpoly, fem 2d, fem 3d, shooting, generation3d), 2 use the framework slots
  `GestureLedger`/`admit_gesture_slot`+`gesture_emit` (wfc/bitmap brush, process3d world), 9 more drive shared node-graph helpers (`node_drag_emit`/`node_graph_edit_rows`: flow, dag, sequence, wires, equation, rewriting,
  wfc 2d, wfc 3d, generation2d), remodel stamps `stream_/commit_transaction` without a runner (streamed ingest, §15), writer uses `TypingFold`.
- `drive_gesture` (the shared driver) is called by 4 crates only: raster, lowpoly, fem 2d, generation3d (fem 3d implements `GestureTool`). The other machine tools keep a private wrapper with own start/resume/abort:
  layout (`🔄️transform/🦀️.rs:287-308`), note (`🖊️ink-apply-events/🦀️.rs:338-362`), drawing (`🖱️canvas-pointer-down/🦀️.rs:704-710`), puzzle 2d (`select/🦀️.rs:337-359`, `P2D/✏️editor/🦀️.rs:1887`),
  puzzle 3d (`🔄️transform/🦀️.rs:338`), puzzle 5d (`:286`), cad (`🧭️transform/🦀️.rs:252`), shooting (`🧭️gumball/🦀️.rs:118`), fem 3d (`gumball:264-287`).
- Per-plugin `HostEvent::TimeTravelFrozen -> ToolAbortReason::Frozen` arms still exist although the framework now maps it once (`TM:556/585`, `freeze_tool_machines`): raster `✏️editor/🦀️.rs:1257`,
  generation3d `:2464`, layout `:1256`, puzzle 2d `:5035` (redundant leftovers).
- **Offenders: declared interactive tools / node-graph edits that mutate outside a machine/transaction** (no `ToolTransaction`, no `TransactionRef`):
  1. puzzle 2d non-drag board tools: `P2D/✏️editor/🎮️commands/🎲️apply-board-events/🦀️.rs:108-152` — `regionCreate`, `regionResize`, `brushPlace`, `edgeCreate`, `edgeDelete`, `nodeDelete` are folded into the scratch
     fixture and diffed into a single history edit; only `gesture` rows (drag/rotate/scale) go through the select machine (`:192-230`). Result: one editable row each, but not a machine, not a transaction.
     (puzzle 5d consumes the same board events: UNVERIFIED).
  2. wfc grid2d, 3 tool utilities (Select/Pin/Mask): `🀄️wfc/🗿️artifacts/🔲️grid2d/.../✏️editor/🦀️.rs:483-487` -> `Emit::mutations` `:578`.
  3. wfc grid3d, 3 tool utilities (Select/Pin/Mask): `🧱️grid3d/.../✏️editor/🦀️.rs:340` `Emit::mutations`.
  4. block 3d, Surface brush: `🧱️block/🗿️artifacts/🧊️3d/.../✏️editor/🎮️commands/📍️place-vortex/🦀️.rs:32` (`artifact_mutations`), `🖌️hover-surface`.
  5. architect program node-graph edits: `🏛️architect/🗿️artifacts/🏛️program/.../✏️editor/🎮️commands/🕸️graph/🦀️.rs:53` `Emit::mutations(emitted)` of host-supplied `operations_json` (the other 9 node-graph
     artifacts use the shared row/drag helpers).
  Sampling note: items 2-4 come from the "utilities declared but no machine vocabulary" filter (grid2d/grid3d/block 3d are the only three of the 19); a continuous drag that emits per-tick plain mutations in an
  artifact with no `UtilityDefinition` would not be caught by this filter (UNVERIFIED for the 20 artifacts without utilities that have pointer handlers: none found by file-name/function-name grep).

### 3e. Artifact-agnosticism in the framework

Core history-editing path is clean: no plugin/artifact name in executable code of `TT`/`TT.ts`, `TM`/`TM.ts`, `PTT`, `OS/🔌️plugin/🛠️tool-machine`, `FW/🔨️modules/📡️replication/🔗️causal`, `R/🛠️ShellHelpers/⏪️time-travel`, wgpu
`R/🐚️Shell/🎯️targets/🧊️wgpu/⏪️time-travel`, `ShellHost` (21 hits, all comments); `STORE` only in doc comments (`:1576, 2459, 6201, 11616, 12385, 13570, 23388`). Framework code that does name an artifact/domain (file:line):
1. `PLUG:29433-29453` and `:29688-29692` — literal selection domain id `"vortex"` (puzzle/block's domain) in the runtime's interaction-leftover selection handling (not in the history path, but in the gesture/selection path every plugin shares).
2. `R/🖥️Board2dHost/🟦️.tsx` (52 hits): `Puzzle2dFixtureDropPayload :47`, `parsePuzzle2dCatalogueDragPayload :162`, `vortex` granularity classifier `:203`, `PUZZLE2D_TRANSIENT_EVENT_NAMES :324`,
   `PUZZLE2D_FLUSH_NOW_EVENT_NAMES :325`, `collectPuzzle2dLiveMirrorMutations :385`, `puzzle2dFixtureDropPreviewJson :455`, `puzzle2dScreenToWorld :497`, peer-gesture functions `:590-645`, drop handlers `:1619-1704`.
3. `R/🗣️Interpreter/🟦️.tsx:1210, 1795` `classifierKind === "puzzle2d"`; `:821` context-menu key `vortex`.
4. wgpu `R/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:3606-3615` (`puzzle3d_catalogue_drag_payload*`), `:5086-5105, 5394, 5478-5614` (`puzzle_board_*`), generic-named `board_event_transient/flush_now :5127/5133`; callers
   `R/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15216, 17597, 17662`.
5. Off-path naming: `MAN:7691-7693` `export_stdio_kinds`/`import_stdio_kinds`, `MAN:7744` `ArtifactKind::Trinity` (kind taxonomy members).
§22.9 (board gestures as a framework contract) is therefore NOT landed; block 2d has no `gesture` producer.

### 3f. Progress/cancel, localization, accessibility

- Progress + cancellation of the replay (expensive op): reducer events `ReplayProgressed/ReplayCancelled/Rerun` (`TT:254`), stepped job (`PTT:3096`, wall slice `PTT:30`, progress ticks throttled by
  `TIME_TRAVEL_PROGRESS_REFRESH_*`), verb `historyEditCancelReplay` (`MAN:2681`), store cancel leaves zero trace (`supersede-replay:344`). React band: `<progress data-semio-time-travel-progress>` + Cancel control; wgpu band: progress
  node `role=progressbar` with `value_now/value_max` (`…⏪️time-travel/🦀️.rs:~995`) + `shell.time-travel.cancel-replay` control. Reprojection/load jobs report and cancel through the same status (React `[data-semio-history-reprojection]`, :480-488). Live React B0: G16.
- Localization en + de: guest `HistoryPanelText` `PTT:3581` (51 variants, two exhaustive `match`es without wildcard, `PTT:3636/3691` -> 102 literal arms); reducer labels `TT` (EN/DE); band copy = corpus `labels` (55 keys x normal/beginner x en/de, verified by script: 0 gaps),
  React pinned to it by test, wgpu embeds it (`band_corpus()` `...wgpu/⏪️time-travel/🦀️.rs:~270`, `include_str!`); framework fault notices `FRAMEWORK_FAULT_NOTICE_LABELS` (82 rows, en/de). Residual: wgpu `band_label` returns `""` for a key the corpus lacks (no failure; pinned only if a
  totality test exists: UNVERIFIED). No default language (every string has both).
- Accessibility: React band `Surface role="status" aria-live="polite"` with `<progress aria-label>`, controls never `disabled` but `aria-disabled` + `aria-describedby` reason + `aria-keyshortcuts` (`…/⏪️time-travel/🟦️.tsx:377-447`), indicator `role="note"`;
  wgpu: ONE polite status node focusable programmatically, `busy` while replaying/finalizing, `progressbar` child, controls via `note_chrome_control_name/description/disabled`, indicator `role=note`
  (`…wgpu/⏪️time-travel/🦀️.rs:~960-1000, 1142`); editor rows are the shared UI-contract tree (slider/spinbutton roles from the contract on both renderers). Live ARIA oracle clean on React B0 (A3/5, F); wgpu mirror
  oracle never run.

### 3g. Residue on the time-travel paths (non-test source)

Scope scanned: `TT`(+ts), `TM`(+ts), `PTT`, `OS/🔌️plugin/🛠️tool-machine`, `PLUG`, `STORE`, `FW/🔨️modules/🎠️kernel`, `MAN`(+ts), `FW/🔨️modules/📡️replication/🔗️causal`, React `ShellHelpers/⏪️time-travel`, `ShellHost`, `Board2dHost`,
`EngineCanvas`, `Interpreter`, `Shell` wgpu `⏪️time-travel`, Slider, and the whole puzzle 2d tree.
- `[DEBUG] `: **0** in non-test source. (Test files carry `[DEBUG]` prints — plugin `🧪️tests/🧩️composition/📨️emission/🦀️.rs:169,215`, `🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:3208`, store sqlite test `:118`, puzzle 3d sqlite test (2) — not on the time-travel path; repo-wide `verify debug-tags` = 708 lines/277 files per the 10-04 census, not re-run.)
- `TODO`/`FIXME`/`todo!`/`unimplemented!`: **0**.
- Stub refusal: `PLUG:45260` `AppCommand::CommandText` answers "CommandText not yet wired" (agents cannot drive history verbs through text lines).
- User-path invariant panics: `PTT:3513` `.expect("a finished authoring replay was held")` (finalize path); `PTT:1127` `.expect("just pushed")` (local invariant). The two earlier `.expect(` (schema, editor) are gone.
- Legacy: `STORE:23056 edit_is_local` still documents and counts "Unauthored (legacy) edits" as local; `"local"` author fallbacks `STORE:20278, 22154`; dead `HistoryTransition::Checkout` (tag 4) still in
  `FW/🔨️modules/📡️replication/🔗️causal/🔀️transition/🦀️.rs:125, 232-263` (§22.14 deletion pending).

---------------------------------------------------------------------------------------------------------------------------------------

## 4. Previous gaps re-verified (G ids of `📓️audit-s5-goal.md`)

| Gap | Status today (evidence) |
|---|---|
| G1.1 composite ops never editable | by design; withdrawing a foreign-step unit handled (`op.unit`, §22.15 `PTT:1746`); fresh editability census not run |
| G1.2 linear `locate_mutation`/`mutation_ops` | **OPEN**: `STORE:19284` loop over all applied edits, `STORE:19197` O(applied ops); §22.11 not landed |
| G2.1 triple band | **OPEN structurally** (`PTT:3922 time_travel_band_section`, React `timeTravelBandControlsV1`, wgpu `time_travel_band_controls:143`); copy single-sourced (corpus `labels`) |
| G2.2 per-plugin `Frozen` | **CLOSED in framework** (`TM:585`, `freeze_tool_machines`); 4 redundant plugin arms remain |
| G3.1 O(n) digests per keystroke | **OPEN**: `STORE:20764-20765` `prefix_state_recorded` builds both digest chains every call |
| G3.2 convergence at edit boundaries | **OPEN**: `STORE:24429 converge` (called from the close-edit path ~:24314) |
| G3.3 non-render readers read committed doc | **OPEN/unproven**: 4 render seams only (`PLUG:35774/35811/35856/35894`) |
| G4.1 Accept always enabled | **OPEN**: `bandControl("accept")` has no `disabledBy` (React `…⏪️time-travel/🟦️.tsx:~83`), guest passes `true` (`PTT` editor sections); `status.changed` exists (`PTT:505`) but is unused |
| G4.2 slider issues `historyEditInput` per change | **OPEN/UNVERIFIED** (slider/dial bind `Trigger::Change`, `PTT:4346, 4417, 4432`; no scrub ledger in the editor) |
| G5.1 bare stdio inputs | **OPEN (reduced)**: now 599 bare inputs of 4751 (546 stdio), 283 stdio leaves with inputs but no declaration, 107 numeric-widget inputs with no `step` (all stdio); census over 2554 editable leaf schemas by `os.walk` (the earlier 2959 came from a looser `find`; not reconciled) |
| G5.2 `widget: dictionary` | **CLOSED** (no plugin schema left; only a negative fixture) |
| G5.3 Multiline control | **CLOSED for guest + React** (`MAN control()` Multiline arm; `PTT:4490 InputKind::LongText`; React `Interpreter:1186, 1572`, `uiTextInputKeyOf` :1533); **wgpu OPEN**: `text_input_key` (Enter inserts a line, Ctrl/Cmd+Enter commits) has no consumer outside the contract; wgpu only maps the kind name (`🔀️reconcile/🦀️.rs:427`) — multiline editing on wgpu UNVERIFIED/likely absent |
| G5.4 gate counts inference as declared | **CLOSED in the gate** (`📋️orchestration/🟦️.ts:90-95` codes `numericUndeclared`, `labelAbsent`, `widgetUndeclared`, `multilineUncontrolled`, `inputless`; census rows declared/inferred); stdio/other leaves still red |
| G5.5 runtime reachability of Edit | **OPEN** (acceptance law per fixture app only) |
| G6.1 Error/Fatal outcome => empty diff not gated | **OPEN/UNVERIFIED** (no gate found) |
| G6.2 `InverseRefused` aborts the session | **CLOSED** (`STORE:24324-24364`; law `a_refused_inverse_is_one_fatal_mutation...:302`; `Merge` replay still refuses `:24341`) |
| G7.1 repair replays from first accepted target | **OPEN**: `settle TT:811-823` `from = accepted.first()` |
| G7.2 no "Next problem" | **CLOSED** (`PTT:431`, status `next_problem` with store; React + wgpu control; live B0 C8/D12) |
| G7.3 non-editable blocker cannot be withdrawn | **CLOSED in code** (`withdrawable`, `BeginWithdrawn`, law `a_blocking_mutation_without_editable_inputs...:2609`); live step 20 never run |
| G7.4 faulted-not-fatal inverse | **CLOSED** with G6.2 |
| G9.1 shared alternative head | **CLOSED** (per-viewer head, `REC_VIEWER`, `🧪️viewer-head`); dead `Checkout` transition not yet deleted |
| G9.2 no finalizing progress | **OPEN/UNVERIFIED** (band offers no controls while `Finalizing`, stage label only) |
| G9.3 duplicate alternative name | **OPEN**: `STORE:21683` `ValidationFailed("alternative … already exists")`; no localized name-conflict hint found |
| G10.1 literal proximity connect | **CLOSED in source 09:52** (`tolerance` + `PreconditionDrifted` producer `connect-handles/🔺️diff:40-49`); no run |
| G10.2 handles/edges never transform targets | by design, unchanged |
| G10.3 hand-written open/abort dance | **OPEN** (`P2D/✏️editor/🦀️.rs:1887`) |
| G11.1 gesture drivers duplicated | **OPEN** (4 adopters of `drive_gesture`, 9 private wrappers, section 3d) |
| G11.2 per-artifact `Frozen` | **CLOSED** in framework (leftovers remain) |
| G11.3 remodel streamed ingest outside the machine | **OPEN** (documented exception §15) |
| G12.1 puzzle-named board filters | **OPEN** (`Board2dHost:324-325`, wgpu `puzzle_board_*`; §22.9 not landed) |
| G12.2 cross-plugin acceptance | **OPEN**: 2 of 96 PASS this session, 94 NOT RUN |
| G12.3 one gesture slot | **PARTIAL** (`GestureLedger` landed; only wfc bitmap + process3d adopted) |
| G13.1 live run of current tree | **PARTIAL**: React/en run on B0 and B1 (A 75/1, B 63/7); wgpu/de none |
| G13.2 folder reload | **REGRESSED**: green on B0 (Run 5 B 67/0, E 35/0), RED on B1 (Run 6 B, section 5) |
| G13.3 second-artifact journey | **OPEN** (zero browser evidence outside puzzle 2d) |
| G13.4 probe-staleness gate | **OPEN** (no such gate found) |

---------------------------------------------------------------------------------------------------------------------------------------

## 5. Live-proof ledger (what is real, with dates)

- React :6012, puzzle 2d, build B0 (guest 00:58, session-5 host hot-loaded), locale `en` only: batches A-J = 376 PASS / 10 FAIL, 0 uncaught, 0 hard faults (`T/📓️w3-e2e-report.md` S5.2). Product faults F1 (Tree remount), F2 (band touch size),
  F3 (pending row action), F5 (load status), F8 (tablet tab bar over band) closed by s1/r2/r3 on the hot-reloaded host; F4 (two peers diverge) closed on the `attach` wave (batch H 27/0, 05:41); F6/F7 open at report time.
- React :6012, build B1 (channel 22, wave B), raw files only: `run6-react-en-A.txt` 75 PASS / 1 FAIL (uncaught `actor-document-control.receipt-count`), `run6-react-en-B.txt` (09:48-09:52) **63 PASS / 7 FAIL, 2 uncaught**:
  steps 1-4 green; step 5 FAILS `folder-attach-writes-the-document-archive` (no files), `folder-reconnect-offered`, `positions-persist-after-reload`, `edit-ids-survive-the-reload` (3 -> 1), `document-rows-survive-the-reload`
  (rows after reload: only "Set Active Example"), `overwrite-row-survives-the-reload`. This is the newest evidence on disk and it is not yet in the report.
- Not run live at all: React `de` (session 5), the whole wgpu arm (serve 6112 up since 07:26; no `run*-wgpu*` output), probe step 20 (row Withdraw/Restore, batch K), N17 history step, long-option rows, presence (needs a hub),
  `mutation.inverse-refused` (no user-visible step), `mutation.precondition-drifted` (landed after), any plugin other than puzzle 2d.
- Non-live substitutes on disk: store/plugin/reducer laws and corpora (names in section 2), React component tests `R/🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component/🟦️.tsx`, wgpu shell tests `R/🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs`
  (all written; this audit ran none).

---------------------------------------------------------------------------------------------------------------------------------------

## 6. Ranked gap list (most goal-relevant first)

1. **[End to end / persistence — host + LOAD + STORE]** Folder reload loses the history edits on build B1 (Run 6 B: edit ids 3 -> 1, overwrite row and document rows gone, folder archive not written, uncaught
   `actor-document-control.receipt-count` in the port retire at attach). Without it "overwrite the existing artifact" does not survive a reload. Closure: fix the receipt-count/port-retire control turn
   (S5-LOAD), re-run batches A/B/E on B1 until step 5/reload is green again, then add the reload verdict to the pre-activation gate.
2. **[wgpu — wgpu shell + E2E]** Zero live proof on wgpu for every clause, plus an unwired text law: `text_input_key` (multiline Enter/Ctrl+Enter/Escape) has no wgpu consumer, so `Multiline` inputs are not editable as
   multi-line on wgpu (UNVERIFIED visually). Closure: wire `text_input_key` in `⚡️events/🦀️.rs`, run `--renderer wgpu --explore` then A-K on :6112, fix what it finds.
3. **[Localization proof — E2E]** No German live run in session 5 (React or wgpu); the en/de corpus is complete on disk but unexercised since Run 2. Closure: React `de` A-K on B1, then wgpu `de`.
4. **[Cross-plugin proof — tests/AGNOSTIC]** `history_edit_acceptance_law!` (115 files / 93 crates / 31 plugins) passed for 2 crates this session (puzzle 2d/3d, pre-wave-B), 94 NOT RUN; the "any artifact" claim has no current
   evidence, and no browser journey exists for a second artifact. Closure: batch the 70+ runnable crates on the quiet tree (one crate per call), then one flow/cad/layout live journey.
5. **[Tools — tool-machine + plugins]** Tools-as-transactions is incomplete: puzzle 2d brush/connect/delete/region tools (`apply-board-events:108-152`), wfc grid2d/grid3d Pin/Mask (`✏️editor/🦀️.rs:578 / :340`), block 3d surface brush
   (`📍️place-vortex:32`), architect program graph edits (`🕸️graph/🦀️.rs:53`) mutate outside a machine/transaction; 9 of 13 machine tools keep private wrappers instead of `drive_gesture`; 4 redundant per-plugin `Frozen` arms.
   Closure: express each as a `GestureTool`/`drive_gesture` machine (or the shared node-graph row helper), delete the wrappers and the four arms.
6. **[Framework agnosticism — React/wgpu hosts + runtime]** `Board2dHost` (52 puzzle-named hits), `EngineCanvas` (`puzzle_board_*`, `puzzle3d_catalogue_drag_payload*`), Interpreter `classifierKind === "puzzle2d"` and the runtime's literal
   `"vortex"` domain (`PLUG:29433-29692`). Closure: §22.9 — schema-generated transient/flush table per event kind, app-descriptor-declared drag payloads and selection domain; a block 2d `gesture` producer as proof.
7. **[Replay cost — store + reducer]** §22.11 not landed: two O(n) digest chains per draft keystroke (`STORE:20764-20765`), linear `locate_mutation`/`mutation_ops` (`:19284/:19197`), repair iterations replay from the first accepted
   draft (`TT:811-823`), convergence only at edit boundaries (`:24429`), no 100k-op budget law. Closure: cache the live digest chain, id index for mutations, resume from the previous report's longest unaffected prefix, op-count budget law.
8. **[Input metadata — stdio + plugins + gate]** 599 bare inputs (546 stdio), 283 stdio leaves with inputs but no declaration, 107 numeric inputs without `step` (stdio), 52 editable leaves that show no input (44 stdio, 4 cad, 2 energy, 1 procedural,
   1 forms) vs only 9 leaves declared `editable:false` (gate `inputless` red; runtime already degrades them to Withdraw-only); snap points are bare numbers (no labelled marks, no per-input radius, <= 32); a stepper shows no ticks.
   Closure: declare the stdio inputs and the 52 `editable:false` markers, decide whether labelled marks are wanted.
9. **[Band structure — React shell + wgpu + guest]** The band exists three times (guest section `PTT:3922`, React `timeTravelBandControlsV1`, wgpu `time_travel_band_controls:143`); copy is shared, control logic is not. Accept is always enabled
   (G4.1) although `status.changed` exists. Closure: derive control states from one shared table/corpus row set (both hosts already read the corpus), disable Accept on `unchanged`.
10. **[Unproven new code — PUZZLE/E2E]** §22.13 (`tolerance`/`precondition-drifted`, landed 09:52), row Withdraw/Restore live step 20, `nextProblem` on a composed child store: written and type-checked at best, no run. Closure: run the puzzle unit laws and probe batch K.
11. **[Reads while editing — plugin runtime]** Non-render readers (pick/hit-test/selection resolution/entity labels) read the committed doc while the preview is shown (4 render seams). Closure: one live probe (hover/pick during Editing), then decide.
12. **[Legacy residue — store, replication, plugin runtime]** `edit_is_local` unauthored-as-local + `"local"` author literals (`STORE:23056, 20278, 22154`), dead `HistoryTransition::Checkout`, `CommandText not yet wired` (`PLUG:45260`),
    `.expect` at `PTT:3513`. Closure: §22.6/§22.14 leftovers.
13. **[Finalize UX — plugin runtime + host]** Duplicate alternative name has no hint (`STORE:21683`), no finalizing progress beyond the stage label, descriptor `order`/`required` read but not used for row order (UNVERIFIED sort). Low.
