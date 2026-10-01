# 📓️ Resume Audit — Tool-Machine Waves (W3-T, W3-T2)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Read-only state reconstruction for the session-2 coordinator, 2026-10-01 ~12:05.
Scope: W3-T-PUZZLE, W3-T-DRAW (+ note), W3-T-SPATIAL, W3-T-FLOWCAD, W3-T-LAYOUT, W3-T2-CONTROLS, W3-T2-TEXT, W3-T2-STROKES,
W3-T2-PROCEDURAL, and the never-started W3-T2-GRAPHS, W3-T2-CLOSURE, W3-T-GEN3D. No file other than this one was written; no cargo/nx/bun,
no git write, no server. Requirement under audit: *every tool is a state machine that yields mutations inside one `ToolTransaction`; tools are
never history-editable, their yielded mutations are* (design §5, §7, §10, §12, §13, §15).

Aliases: `T` = ticket folder; `G` = `T/🗑️generated`; `PL` = `✏️s/🔌️plugins`; `FW` = `🧰️framework/🔨️modules`; `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`;
`RE` = `OS/📺️renderer/🧑‍🎨engine/🧱️elements`; `HUB` = `🌎️hub/🧩️compositions`; `S1` = `🏅️standards/🔖️1/🪆️subsets/✳️any`.
Evidence tags: **(log)** a file in `G` I read; **(grep)** a `git grep`/`grep` result at the current worktree; **(mtime)** `stat`; **(read)** read the code; **(inferred)**.

## 0. Verdict in twelve lines

1. **Nothing is lost and nothing is half-written syntactically.** Every fleet edit is already in HEAD (auto-commit `4e36b2b5012`, 11:16); the only dirty plugin files are
   peers' (`🗄️stdio` 170, `🀄️wfc/🧊️3d` snapshot-sqlite 14, `📸️remodel` container providers 6) (grep: `git status --porcelain -- ✏️s/🔌️plugins` = 188 lines).
   The 92 Rust files and 85 JSON files edited in the dead window (07:21 → 08:16, **(mtime)**) are all brace-balanced / parse (own script, sanity-tested on a known-bad file);
   every symbol I spot-checked at its definition exists (lowpoly `stamp_brush_on`/`pixel_runs_from_diff`, wfc `drag_slots`/`set_slot_positions` wiring, procedural `generation2d_node_drag_leaves`/`slider_gesture_ui_scope`, writer `TypingFold`/`TextSpliceComposition`, flow `flow_drag_tool_commit`).
2. **The real risk is "never compiled/never run", not "half done".** Last green compile per WP: puzzle 07:35 (wasm lib), note 07:54, fem 08:10, layout 08:04, writer 08:09, wfc-bitmap lib 07:54; **never** compiled since conversion: shooting, flow, lowpoly,
   procedural gen2d/gen3d, wfc 2d/3d, process3d; draw last compiled 22:54; no WP has run its full crate test suite green end to end.
3. **Finished (source) and verified in part:** puzzle 3d/5d (leaf tests 377 + 463), cad (459/2 pre-existing), layout (588/2), fem 2d (1284/2), note (compiles), controls core (scrub law + F-1), tool-machine crate (Rust 28, TS 33).
4. **Largest unfinished conversions:** lowpoly (0%: brackets + scratch + paint session untouched, only the `apply-paint-stroke` leaf was written 07:36), raster, remodel, flow F6, dag, sequence, mathematical, hub-space (`Emit::amend` still alive in 6 places), trinity rewriting, stdio md/html (`SetSnapshot` per delivery), W3-T2-CLOSURE.
5. **Writer typing laws are red** (6 of 23, all "one run = one row" count assertions, 3 vs 2 at 08:09) — first thing to diagnose in TEXT.
6. **Layout has one red law with a likely cause**: `one_shot_turns_and_scalings…` fails with `batched item candidate failed its exact fixed fold contract`; the documented cause is an under-declared one-item footprint for leaves whose inverse has several rows (`rotate-frames`/`scale-frames`) — a hypothesis to confirm, and a hazard for every relative leaf with a multi-row inverse (§2.3.1).
7. **The §12 (flow composed-child) runtime slice is written but uncompiled** (`OS/🔌️plugin/⏪️time-travel/🦀️.rs` 07:52, 2994 lines; 4 flow laws written 07:52, never run).
8. **Stale audit rows:** `🪐️space` moved to `HUB/🪐️space/⚙️engine/🪐️space` at 04:27 (peer); the audit's space rows are valid but under that path. Writer/gis/forms/playbook/vcs/jack/procedural `Emit::amend` are gone (grep: 0 non-test hits).
9. **One leftover temp log:** `eprintln!("[DEBUG] shooting retained reduce …")` at `PL/🎥️shooting/🗿️artifacts/🎥️shooting/S1/✏️editor/🦀️.rs:493` (committed). The `[DEBUG]` println in `PL/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🦀️.rs:584` predates this ticket (blame 09-30 00:11).
10. **Environment now:** load average 83, swap 9.2/10 GB, 54 GiB free, 4 rustc — builds are 8–16 min each and die under fleet load. Peers are editing `OS/🏪️store`, `OS/🌿️vcs` (paged-ledger session, 11:13/11:44), stdio, wfc/3d, remodel, energy, wgpu ui `🎬️action` right now.
11. **Proposed regrouping:** 12 executor WPs with exclusive trees (§6): RUNTIME, PUZZLE, INK, SPATIAL, FLOW, LAYOUT-CAD, CONTROLS, TEXT, PROCEDURAL, STROKES, GRAPHS, GATES.
12. Decisions needed from the coordinator are in §8 (forms leaf shape, raster stroke leaf, typing-run co-edit, CAD streamed gumball, config-lane amend policy).

## 1. Method

- Read: `📋️design.md`, `🧭️plan.md`, `📓️status.md`, `📓️audit-remaining-tools.md`, all reports named in the brief, `📓️api-scrub-machine.md`, `📓️w2-d-report.md`, `G/w3-*`.
- Timeline anchors: session-1 resume 07:21; fleet death ~08:05–08:16; session-2 start 11:30. `find -newer` against reference files at 07:21/08:16/08:30.
- Census: one `git grep -n -I -E '<patterns>'` over `✏️s/🔌️plugins/**/*.rs|ts|tsx` (450 hits, saved in my scratchpad), plus a second pass `\bamend[a-z_]*\(|\bAmend|COALESCE_KEY|coalesce_key` (62 hits) and a hub/framework pass. `git grep` only sees tracked files; the few untracked files (wfc 3d sqlite, remodel composition) are peers'.
- Per-WP: report → `G/<wp>` logs (command, result, mtime) → mtime of every owned file after the last log → definition check of anything newly referenced.

## 2. Global facts that change the plan

### 2.1 Dead window and peers
- Last log writes: puzzle 07:54, draw/note 07:54, spatial 08:10, flowcad 07:52, layout 08:04, controls 07:54, text 08:09, strokes 07:54, procedural 07:52.
- After 08:16 only peers touched the trees: repo-wide `package.json` sweep 08:48, `Cargo.toml` sweep 09:11 (dozens of manifests), note sqlite 09:52–10:47, remodel container providers 11:27–11:40 (new untracked `📸️remodel/🗿️artifacts/📸️remodeling/🧩️composition/`), energy kernels 11:36, wfc 3d snapshot sqlite 11:17–11:36, wgpu ui `🎬️action` (`DslValue::Bytes`) 11:32–11:34, store 11:13, vcs 11:44, `RE/✏️TextEditor|🏛️ShellHost|…` import sweep 11:12, `RE/🕸️NodeGraph/🟦️.tsx` 10:49.
- Consequence: before any edit a successor must re-read the file (fleet rule 6) and expect `ArtifactSqliteSnapshot` / `ArtifactCodec::of` churn in stdio, note, wfc/3d, remodel.

### 2.2 Blockers (non-fleet)
| # | Blocker | Evidence | Hits |
|---|---|---|---|
| B1 | `ArtifactSqliteSnapshot` rollout (peer): stdio crates did not compile at 07:54 (`las`: no `ArtifactCodec::of`; `dwg`, `pdf`, `gltf`); stdio has 179 dirty files now | `G/w3-t2-controls/gis-test-3.txt`, `G/w3-t-draw/test-lib-3.txt`, `git status` | puzzle 3d (direct stdio deps), draw, gis, flow (stdio semio), wfc/3d, note |
| B2 | Store/vcs under the PAGED-ARTIFACT-HISTORY-LEDGER session (`OS/🏪️store/🦀️.rs` 11:13, `OS/🌿️vcs/🦀️.rs` 11:44, new `🌿️vcs/🧪️tests/🔬️paged-ledger`). Store HEAD already has `ArtifactCommand::{AppendTransaction,CommitTransaction,AbortTransaction}` and `open_transaction` (§15 store half); the plugin-runtime half (Emit shape, `tool_transaction_shape_fault` refinement) is absent (grep: 0 hits of `AppendTransaction` in `OS/🔌️plugin`) | grep | remodel (§15), CLOSURE (`AmendLast` still 31× in store, 11× in plugin) |
| B3 | Taxonomy/nx plugin load repeatedly invalid (peers): `verify taxonomy report`, `schema *`, activation | `G/w3-t2-text/lint.txt` (07:41: `plugin-registry`/`wgpu-frame-worker` inputPatterns), puzzle report §4 | every gate step |
| B4 | Build budget: load 83, swap 9.2/10 GB; a cold plugin check took 7–17 min even at 07:30 (`check-13` 7m04, `fem3d-check` 16m30) | `uptime`, logs | all |

### 2.3 Cross-cutting hazards found during the audit
1. **One-item fold footprint.** `OS/🏪️store/🦀️.rs:15638-15680`: `work_items` counts staged ROWS = forwards + inverse rows; a leaf whose inverse yields k rows needs `for_one_item(k, bytes)`, never a hard-coded count. Layout's red law is this (see §4.5). Audit every relative leaf with a multi-row inverse: puzzle selection leaves (N targets), draw (one absolute row per layer), fem `move-selection`, shooting, lowpoly, wfc `drag-slots`, procedural transforms, flow `drag-nodes`. Explicit declarations exist in ~40 artifact trees (grep `for_one_invertible_item|for_one_item|ArtifactStoreOneItemFootprint`; every converted artifact except **shooting**); layout declares only for its window-transient lane (`PL/📏️layout/…/📐️blueprint/🫧️transient/🦀️.rs:123-127`, `work_items: 1`), so its artifact lane runs on the framework default **(inferred)**. The refusal is raised in `ArtifactStore::fold_batch_item` (`:18928`) on several exact conditions (one forward row, ids, sequence number, stamped identity, declared footprint), so footprint under-declaration is the documented cause, not the only one.
2. **Hot-file contention.** `OS/🔌️plugin/🦀️.rs` (2.9 MB) carries `Emit`, dispatch, scrub/typing seams, `TextWindowKit`, composed-child publication, `AmendLast`; four WPs need it. §6 gives it to one owner.
3. **Shared hosts.** `RE/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` carries the wgpu gumball, board coalescer (W2-D) and node-graph edit writer (flow, 03:40): exactly one owner.
4. **Legacy debris:** `PL/🖍️draw/🏭️bridge/Cargo.lock` still lists `semio-s-plugin-draw-fsm(-macros)` (lines 715, 937, 943); `PL/🏭️process/…/🧬️mutations/🦀️.rs:24` doc comment names the removed `change-cursor` leaf; fixtures `OS/🔌️plugin/📇️registry/🧫️fixtures/📖️generated-projection.json` and `📚️library/🧫️fixtures/🖍️draw-source-scenario` mention `draw-fsm` (test inputs; verify they are intentionally historical).
5. **Stale count assertions** from the new relative leaves: fem 2d `…/🌐️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:305` (`kinds.len() == 29`, actual 30 — red in `G/w3-t-spatial/fem2d-test.log`), fem 3d same file at `:330` (same literal 29; likely red).
6. **Lint finding owned by draw:** `s.draw.drawing.1.transform.mutation.drag-path-points` — `/targets` is `x-semio-ui.role: target` over an object array (`DrawingPathPointTarget`) → `widgetIncompatible` (`G/w3-t-spatial/lint-mutation-inputs.log`, 03:35; file `PL/🖍️draw/…/🔀️transform/🧬️schema/🧬️mutations/📍️drag-path-points/🧬️schema/🔣️.json`, mtime 09-30 21:49, unchanged).

## 3. Status matrix

| WP | Agent (session 1) | Last assignment | Last source edit | Last green compile | Tests | Interrupted-edit risk | Remaining |
|---|---|---|---|---|---|---|---|
| W3-T-PUZZLE | a2db38a4d83b6e330 | verify + 4 fixes (rotate pivot parity, attraction re-solve, paged relocate scan, derive warning) | 5d editor `🦀️.rs` 07:40 | wasm32 lib check 07:35 | leaf 377/463 green; editor/tool laws never run | MED (5d editor edited after last compile) | S–M |
| W3-T-DRAW (draw+note) | ad1bf38b258043a7b | tool machines for draw canvas + note ink, hosts | note `ink-apply-events` 07:40; draw 03:02 | note 07:54; draw 22:54 | draw lib aborted at 136/472; note never run | LOW (note), MED (draw: 3 Rust files edited after last compile) | S |
| W3-T-SPATIAL | aac13b60d073dd9a1 | shooting, fem 2d/3d, lowpoly, hosts, bracket rule | lowpoly leaf 07:36; fem2d tests 07:42 | fem 2d+3d lib 08:10 | fem2d 1284/2; shooting/lowpoly never | MED (lowpoly leaf uncompiled) | L (lowpoly) |
| W3-T-FLOWCAD | a8a5db0e746a8ede6 | CAD done; flow drag; §12 composed-child | `⏪️time-travel/🦀️.rs` 07:52, flow tests 07:52 | plugin lib 07:47 (before time-travel edit) | cad 459/2; semio flow leaves 49/49; tool-machine 28+33; flow plugin never | HIGH (07:52 runtime edit uncompiled) | M |
| W3-T-LAYOUT | acde7cd8aaa4b17f0 | layout gumball → leaves + tool | `patch-frame` 07:46 | test build 08:04 | 588/2 | LOW | S |
| W3-T2-CONTROLS | a26b93e980098bdf4 | scrub machine, hosts, F-1/F-4, 5 plugins | 07:23 (source-complete) | plugin lib laws 02:31 (27/27 filter) | ui corpus 91/1 at 03:02, rest unverified | LOW | S–M (verify) |
| W3-T2-TEXT | (slot 22:42) | typing runs + explicit draft | plugin `🛠️tool-machine/🦀️.rs` 07:52 | writer build 08:09 | writer 17/6 | MED | M |
| W3-T2-STROKES | ace6d82c4c565ec71 | wfc bitmap, raster, remodel, process | wfc 2d/3d 07:31–07:48 | wfc-bitmap lib 07:54 (test target 1 error) | none green | HIGH (wfc 2d/3d never compiled) | M–L |
| W3-T2-PROCEDURAL | (slot 03:19) | gen2d/gen3d sliders, widgets, moves, gumball | 07:48 | **never** | none | HIGH (≈45 files, never compiled) | M |
| W3-T2-GRAPHS | — | never started | — | — | — | — | M |
| W3-T2-CLOSURE | — | never started | — | — | — | — | S–M |
| W3-T-GEN3D | — | merged into PROCEDURAL (21:36/22:05 decisions); gen3d gumball code exists | 07:35–07:38 | never | none | HIGH | (in PROCEDURAL) |

## 4. Per-WP detail

### 4.1 W3-T-PUZZLE (puzzle 3d/5d)
- **Last assignment.** Launched 21:29 (opus). Source-complete 03:19 (status). Resumed 07:23: verification, plus (a) wgpu multi-object rotate preview vs committed pivot, (b) re-solve attractions/fasteners on move, (c) page the relocate scan with progress + cancel, (d) `machine::statechart!` `unexpected cfg serde` warning, and the report's §7 command list.
- **Done (evidence).**
  - Leaf suites: `cargo test -p semio-s-artifact-puzzle-3d --lib` selection+mutations **377 passed** (`G/w3-t-puzzle/test-3d-leaves.txt` 09-30 21:57); 5d **463 passed** (`test-5d-leaves.txt` 22:37). Lints `mutation-inputs`/`mutation-payloads` 0 findings both (`lint-*` 02:48–02:49; 3d 38/38 leaves witnessed, 5d 39/39).
  - **wasm32-wasip2 lib check of 5d+3d `Finished` at 07:35, 0 errors, 5 warnings** (`check-wasm-5d.txt`; `Checking …puzzle-3d` line 437, `…puzzle-5d` line 539). This is new relative to the report (report §4/§5 said blocked).
  - (d): the derive opt-in landed (`FW/🔄️machine/✨️derive/⚙️expansion/🦀️.rs` 07:51; `semio-framework-machine-derive` 11/11, `G/w3-t-puzzle/test-machine-derive.txt` 07:54, test `statechart_event_codec_is_an_explicit_opt_in`). The two charts still warned at 07:35 (`…🔄️transform/🦀️.rs:251` 3d, `:203` 5d); confirm the warning is gone on the next check, then remove the `#![allow(unexpected_cfgs)]` in shooting (`PL/🎥️shooting/…/✏️editor/🎮️commands/🧭️gumball/🦀️.rs:10`).
- **Not done (grep/read).** (a) no leaf or host change since the report; (b) no re-solve code; (c) `Puzzle5dTransformWork` is still `Read → Commit` with `extent Some(2)` (`P5/✏️editor/🦀️.rs` ~4990-5110), 3d `Puzzle3dTransformWork` untouched.
- **Unverified.** All editor/tool/store laws: 3d `🧪️transform-tool`, `🔬️unit` (`gumball_gestures_declare_no_bracket_verbs`, nakagin relocate), store `a_drag_edited_in_history_replays_its_downstream`; 5d `one_gumball_translate_is_one_edit_one_row_and_one_transaction`, `two_gumball_gestures_are_two_transactions`, `a_board_drag_is_one_board_leaf_in_one_transaction`, `an_inspector_nudge_is_one_relative_transaction`, `a_motionless_gumball_release_leaves_zero_trace`, `selection_transform_hostile_static_law_rejects_one_grant_reducers_and_bypassed_tools`, `a_board_drag_edited_in_history_replays_its_downstream`; retained-jobs oracle (`P3|P5/🧫️fixtures/🗄️retained-jobs/🔣️.json`); TS suite. `G/w3-t-puzzle/test-3d-editor.txt` (07:51) died on os-kernel `E0560 open_transaction` (W1-G follow-up 5 mid-edit; store HEAD now has the field).
- **Interrupted-edit suspicion: MED.** `P5/✏️editor/🦀️.rs` edited 07:40, five minutes after the last compile finished; content unknown (brace-balanced). The wasm check does not compile test code at all.
- **Remaining checklist.**
  1. Gate: `cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --target wasm32-wasip2 --lib --message-format=short` (covers 3d). Then `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib` and the 5d twin (report §7 steps 2–3, focused filters listed there). Private `CARGO_TARGET_DIR=…/target-nde-puzzle`.
  2. Fix (a): decide leaf semantics for rotate. Recommended: add `pivotX/pivotY/pivotZ` to `rotate-selection` (3d) and `rotate-selection3d` (5d) like puzzle 2d/draw/layout (pivot = gumball pivot) so wgpu preview == commit; touches both leaf schemas (`x-semio-ui`), Rust, TS twin, binary protocol, text grammars, quintets, Python oracle (`🧊️mutate-puzzle-3d-1/🐍️.py`, `🖐️mutate-puzzle-5d-1/🐍️.py`), `from_gumball`/`from_pose_delta`.
  3. Fix (b): attractions/fasteners attached to moved items are not re-derived on selection moves (3d `Puzzle3dSelectionRecord.attractions`, 5d `fastenings`). Implement as extra yielded `connect-vortices`/`connect-grips` + removals, deterministic ids (`attraction-<a>-<b>`, `fastener-<s>-<t>`), same transaction.
  4. Fix (c): page the proximity scan: add `Scan` stage(s) with `Progress{stage,…}` + cancel to both transform works; `puzzle5d_relocate_record` is O(parts×grips) in one step.
  5. Footprint audit for the multi-row inverse (§2.3.1).
  6. `bun nx run @semio-tech/puzzle-js:test`; `schema mutation-inputs|mutation-payloads --under …/🧊️3d|🖐️5d` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, repo-relative `--under`); `verify taxonomy report --scope …`.
  7. Optional: stale hostile anchors in retained fixtures (3d `Puzzle3dScalarConfigWork`, `Puzzle3dEngagement*Work`; 5d `apply_world3d_sun_action`, `SetBrushCandidateIndex`, `SetEngagementInput`).
  8. Update `📓️w3-t-puzzle-report.md` §4–§7 with the 07:35 evidence and the new results.
- **Blockers/deps.** B1 (puzzle 3d links stdio crates directly), B3; (a) only needs puzzle files; no host change if the leaf carries the pivot.
- **Successor brief.** The conversion is complete and type-checks for wasm; your job is to prove it. Compile the editor *test* targets first (never done since the editors changed), run the named laws, fix what breaks in `P3`/`P5` only, then do the three deferred behaviours: pivot-carrying rotate leaves (preferred over touching wgpu), attraction/fastener re-solve, and a paged relocate scan with progress/cancel. Do not touch World3dHost (SPATIAL) or the board engine. Re-run both lints after any schema change.

### 4.2 W3-T-DRAW (draw + note + ink hosts)
- **Last assignment.** Launched 21:29 (opus); 02:41 decision: canvas-tool tests/fixtures moved out of the projected command dir; package-move coverage re-target handed to W3-TAX (05:50).
- **Done (read/grep).** Draw: vendored `fsm` deleted (`git ls-files | grep -i fsm` = 0), `canvas_tool` statechart + `DrawingTool`, four relative leaves (`drag-layers`, `rotate-layers`, `scale-layers`, `drag-path-points`) in subset `🔀️transform`, nudges one-shot tool, opacity absolute leaves for the scrub machine; policy anchors in root `📜️script.ts`. Note: `ink_tool` statechart, `NoteInkTool`, window-transient `inkTool`, hosts React `🖋️InkCanvasHost` (03:03) and wgpu `🎞️Scenes` (stream/commit/abort), nudges one `drag-blocks`. Remaining `coalesce` in note: pass-through plumbing in `NO/✏️editor/🧵️retained/🦀️.rs:173-181,215,358` only.
- **Verification evidence.**
  - Note: `cargo check` lib **Finished** 07:40 (7m04) and 07:54 (3m33) (`G/w3-t-draw/check-note-2.txt`, `check-note-3.txt`).
  - Draw: last successful compile of the **test** target 22:54 (10m34). That run aborted at **136 of 472 ok**: `selected_group_and_layer_drag_preserves_selection_and_one_history_edit` asserted the row label `"Drag 2 layers by (30, 20)"` but received the op line `drag-layers dx=30 dy=20 targets=[ group other ]` (`unit/🦀️.rs:1472`), then the abort is the `FixedOperationRegistry` Drop witness (`🧵️job/🦀️.rs:362`). The earlier 22:37 run also failed `archive_load_tests::demo_example_load_settles_through_the_host_document_archive_door`, `unit_tests::renders_canvas_scene_with_segments`, `retained_route_dispositions_are_exact_and_exhaustive`, `set_active_example_resolves_the_registered_catalogue` (triage: caused by this WP or peers?). The `ArtifactEditor::mutation_label` hook was added 03:02 (probably the fix) but **draw was never recompiled after 22:54** (test-lib-3 02:45 died on stdio peers).
  - vitest (React) ink 20/21 at 03:11; the one failure is a source-scan test (`scene-pointer-cancellation` expecting `DispatchEvent::PointerCancel { pointer } => app.handle_pointer_cancel(pointer.id)` in a winit host file) — probably pre-existing, verify.
  - The report `📓️w3-t-draw-note-report.md` (07:33) has no verification/open-items section.
- **Interrupted-edit suspicion.** Note LOW (compiled after its last edit). Draw MED: edits after the last compile (22:54) were never compiled: `🌫️set-selected-opacity/🦀️.rs` 02:32, `✏️editor/🧪️tests/🔬️canvas-tool/🦀️.rs` 02:47, `✏️editor/🦀️.rs` 03:02 (and kinds-catalog TS 02:46).
- **Remaining checklist.**
  1. `cargo test -p semio-s-artifact-draw-drawing --lib` (expect 472; triage the 4 failures listed; if the label assertion persists check `mutation_label` wiring on the retained path and the Drop-witness helper from the "Store Drop Witness" lesson).
  2. `cargo test -p semio-s-artifact-note-note --lib` (ink laws in `NO/✏️editor/🎮️commands/🖊️ink-apply-events/🧪️tests/🔬️unit/🦀️.rs`, window tests).
  3. Fix the draw lint finding (§2.3.6): drop `role: target` from the object-array `targets` or model the leaf input as id references (`layerId`+`index` pairs) — then re-run `schema mutation-inputs|payloads --under ✏️s/🔌️plugins/🖍️draw` and `…/🗒️note`.
  4. wgpu Rust edits never compiled (`RE/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` `write_ink_events_action`, `ink_gesture_abort_into`, `ink_gesture_open`; `RE/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` `cancel_scene_pointer`; `RE/🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs`): `cargo check -p semio-framework-os-renderer-wgpu --tests` + `…wgpu-ui-command-wiring` law.
  5. TS: draw package `bun ./📜️script.ts test` (canvas test + 4 leaf Ajv tests, list in `PL/🖍️draw/📦️packages/🟦️typescript/📜️script.ts`), `RE/🧪️tests/{🎬️surface-behavior,🖋️ink-canvas-clipboard,🛑️scene-pointer-cancellation}`.
  6. Hygiene: remove `draw-fsm` from `PL/🖍️draw/🏭️bridge/Cargo.lock`; verify the two fixtures that name `draw-fsm`; append §Verification + §Open items to the report; regeneration list: draw/note descriptors, schema catalog (4 draw leaves in a new subset), taxonomy for `🔀️transform`.
- **Blockers/deps.** B1 (draw's test build died at stdio-pdf/dwg), B3.
- **Successor brief.** Treat draw as "written, never fully run". Compile and run the draw lib suite first and fix whatever is red in `PL/🖍️draw` only; run note's suite; then the wgpu ink Rust check and the draw lint finding. Do not re-design anything: the census/design in the report is sound and note compiles. Own `🖋️InkCanvasHost`, wgpu `🎞️Scenes` and the wgpu Interpreter target file; leave `🗣️Interpreter/🟦️.tsx` and wgpu ui events to CONTROLS.

### 4.3 W3-T-SPATIAL (shooting, fem 2d/3d, lowpoly, hosts, bracket rule)
- **Last assignment.** Launched 21:29 (opus). 22:03: shooting + World3dHost protocol done; queue fem → overlay → lowpoly → bracket rule. 03:30: fem 2d/3d `move-selection` + gumball tool machines + bracket deletion "written (uncompiled)"; lowpoly next. Layout and generation3d were carved out (22:33 / 22:05).
- **Done.**
  - Host protocol (a dispatch without `phase` = one-shot; `stream`/`commit`/`abort{reason}`; `gumballIdentityDelta`): `RE/🌐️World3dHost/🟦️.tsx` (09-30 22:00) and `RE/📐️Canvas2dHost/🟦️GumballOverlay.tsx` (09-30 22:40); React gumball vitest **12/12** (`G/w3-t-spatial/engine-contract-gumball.log` 03:20). No `transformBegin|transformEnd` left in either host or in puzzle/fem (grep).
  - Shooting: gumball tool (`ToolMachineRunner`, relative `drag-/rotate-/scale-assets`, key `assets:0`, source 21:51/22:14). **No compile or test log exists for shooting** (grep of `G`).
  - Fem 2d/3d: `move-selection` leaf (verb `move`; 2d 22:32, 3d 02:32), gumball machines, artifact-level local-only `FemGumballTransient` keyed by window (documented deviation). `cargo check` of fem-2d + fem-3d libs **Finished** 08:10 (`fem3d-check.log`, 16m30). `cargo test -p semio-s-artifact-fem-2d --lib` **1284 passed / 2 failed** (`fem2d-test.log` 07:55): `analyses::tests::p6h_owned_assembly_lookup_partition_scan_transfer_interrupt_replay_and_timing` (8 ms timing law under load — flake) and `every_mutation_registers_a_semantic_descriptor` (stale `29`, §2.3.5). fem 3d tests never run.
- **Not done (grep/read).**
  - **Lowpoly: conversion not started.** Editor still has `transformBegin`/`transformEnd` (20 lines in `PL/💠️lowpoly/…/✏️editor/🦀️.rs`: 336, 361, 547, 572, 612, 634, 674, 676, 1260, 1281, 1942, 1963, 2249, 2286, 2402, 2429, 2451-2452, 2477-2478; `🖌️session/🦀️.rs` 377/383/508 + `LowpolyScratch` 239; `🎮️commands/🧲️transform`; `🌐️model/🦀️.rs:46-47`; mounted tests `🧪️tests/🔌️mounted/🦀️.rs` ×10; feature `🧪️tests/🎮️command-lowpoly-1/🥒️.feature`), `paintStrokeBegin/End` in `🎮️commands/🖌️paint` (17), `session` (15), `🦀️.rs` (33). No `ToolMachineRunner` in lowpoly (grep).
  - What *was* written at 07:36: leaf **`apply-paint-stroke`** (`…/🧬️schema/🧬️mutations/🖌️apply-paint-stroke/`: payload, diff, inverse, schema, 6 test cases; fixtures ×6; aggregate variant `ApplyPaintStroke` line 65, KINDS line 91, module `🦀️.rs:795`, grammar/graphql/proto/ts/binary, oracle + `🧪️mutate-lowpoly-1` py/feature/rs). Uncompiled; its helpers exist. There is no relative mesh-transform leaf yet (existing `move-object`/`rotate-object`/`scale-object` are object-level absolute; the old gumball wrote a whole-mesh `CreateMesh` via `LowpolyScratch`).
  - **Bracket rule not deleted:** `FW/🛂️manifest/🦀️.rs:2049-2064` (`GUMBALL_GESTURE_BRACKET_ACTION_IDS`, `framework_fixed_audience`), fixture `FW/🛂️manifest/🧫️fixtures/🖐️gumball-verb-audience.json`, test `…/🧪️tests/🖐️gumball-verb-audience/🦀️.rs`, comment at `OS/🔌️plugin/🦀️.rs:21346`. Delete after lowpoly (the last consumer); `GUMBALL_CHROME_ACTION_IDS` stays.
- **Interrupted-edit suspicion: MED.** The 07:36 lowpoly leaf batch (≈45 files) was never compiled; `G/w3-t-spatial/check_lowpoly_py.py` (07:35) suggests a fixture self-check was being written. Brace-balance OK.
- **Remaining checklist.**
  1. Lowpoly: (i) relative transform leaf, e.g. `transform-mesh {objectId, targets (vertex/face/object ids), motion: drag|rotate|scale, pivot…}` (schema-first, `x-semio-ui`, quintets, wire witness, labels en/de, inverse as exact absolute rows → fold-footprint declaration); (ii) gumball tool machine over the shared host protocol; (iii) paint stroke machine yielding `apply-paint-stroke` (points streamed in transient, one transaction per stroke; UV canvas + World3d paths); (iv) delete `LowpolyScratch`, `transformBegin/End`, `paintStrokeBegin/End` verbs, contracts, extents, manifest rows, descriptions, audience entries, mounted tests; (v) `cargo check`/test `semio-s-artifact-lowpoly-lowpoly`; (vi) lints.
  2. Bracket rule deletion (list above) + `cargo test -p semio-framework-manifest` gumball-verb-audience replaced by a negative law.
  3. Fem: fix stale counts (2d line 305, 3d line 330), run `cargo test -p semio-s-artifact-fem-3d --lib`, re-run timing flake at low load; lint `leafUncatalogued` for fem3d `move-selection` clears with central `schema generate`.
  4. Shooting: remove the `[DEBUG]` eprintln (§0.9) and `#![allow(unexpected_cfgs)]`; `cargo test -p semio-s-artifact-shooting-shooting --lib` (gumball laws at `…🧭️gumball/🧪️tests/🔬️unit/🦀️.rs`, editor unit tests).
  5. Footprint audit (§2.3.1) for fem/shooting/lowpoly leaves; write the verification section of `📓️w3-t-spatial-report.md` (it is still "IN PROGRESS", 4.4 KB, census + protocol only).
- **Blockers/deps.** B1 (shooting/fem links to stdio? verify), B3; lowpoly deletions need the manifest crate (SPATIAL owns it for this wave).
- **Successor brief.** Shooting, fem and the two host overlays are done in source: compile/run them, remove the debug log, fix the stale `29` counts. Then do lowpoly end to end — it is the only gesture family in the repo still on brackets + scratch; start from the already-written `apply-paint-stroke` leaf, add the relative transform leaf, build both tool machines on the stable host protocol, delete everything bracket-shaped, and only then delete the manifest bracket rule. Do not touch World3dHost beyond bug fixes: layout, procedural gen3d and puzzle consume its protocol.

### 4.4 W3-T-FLOWCAD (flow + cad, composed-child transactions §12)
- **Last assignment.** Launched 21:29 (opus). CAD finished first; flow `drag-nodes` child leaf landed (02:23); then §12 (22:10 decision): transactions spanning parent+children, history lists child-member mutations with store identity, time travel on a child member store.
- **Done (log/grep/read).**
  - CAD: relative `drag-selection`/`rotate-selection`/`scale-selection` (pane-scoped), transform tool (`CA/✏️editor/🎭️modes/✏️edit/🛠️tools/🧭️transform/🦀️.rs`), engagement commits through it; `cargo test -p semio-s-artifact-cad-cad --lib` **459 passed / 2 failed pre-existing** (`G/w3-t-flowcad/cad-test-1.txt` 22:14); lints: 28/28 witnessed, 3 `leafUncatalogued`.
  - Flow: `drag-nodes` leaf (`PL/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🧬️mutations/✋️drag-nodes/`), `semio-s-artifact-stdio-semio --lib` flow filter **49/49** (`semio-test-2.txt` 22:39); `NodeDrag` region + `node_drag_commit` in `FW/🛠️tool-machine` (Rust 28 passed `tool-machine-test-1.txt` 03:32; TS 33 pass `ts-tool-machine.txt` 03:31; fixture `🧫️node-drag-law`); flow `✋️drag` tool (`PL/🌊️flow/…/🛠️tools/✋️drag/🦀️.rs`, 03:38) ; React host journals `{gestureId, nodeIds, dx, dy}` (`RE/🕸️NodeGraph/🟦️.tsx:1375`), wgpu writes `DagGraphEdit::Move{gesture_id,node_ids,dx,dy}` (`RE/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4114-4160`, 03:40).
  - §12 framework: kernel `HistoryMutationEntry.store` (`FW/🎠️kernel/🦀️.rs:1896`; TS twin; kernel TS 77/77 `ts-kernel.txt`), reserved verb `historyEditBegin{mutationId, store?}` TS 3/3, `Emit::commit_child_transaction` (`OS/🔌️plugin/🦀️.rs:12836`), `PendingChildGroupPublication.transaction`, `tool_transaction_shape_fault` allows parent+owned children (`:24051`), time-travel `member: Option<TimeTravelMemberSubject>`, `TimeTravelStoreState`, composed-member history backfill (`⏪️time-travel/🦀️.rs:1350`), finalize-on-member re-derive (`:1769`). Plugin lib `cargo check` **Finished 07:47** (`plugin-check-2.txt`, 1m56) — i.e. after the 07:44 plugin edit but **before** the 07:52 `⏪️time-travel` edit; `plugin-check-3.txt` (07:52) died on the store.
- **Unverified/red.** `semio-s-artifact-flow-flow` was **never type-checked** after conversion (flow-check-1..5 each stopped before reaching the flow crate: stdio-semio sqlite, os-kernel macro token, framework-plugin `M: Send` at `OS/🔌️plugin/🦀️.rs:7674` — fixed by 07:47 —, framework-3d). Four §12 laws written 07:52 and never run: `a_node_drag_is_one_child_transaction_row_naming_its_member_store`, `editing_a_node_drag_offset_replays_the_member_and_the_parent_scene_follows`, `withdrawing_a_node_drag_in_history_puts_the_node_back`, `a_blocking_member_replay_refuses_to_finalize_and_exits_with_zero_trace` (`PL/🌊️flow/…/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs:234-328`). TS `node-graph-wire-edit` 21/22 (the failing test is a source scan expecting `setPointerCapture(event.pointerId)`; likely pre-existing). wgpu `plan_graph_edits` Rust tests never run.
- **Not done.** F6 `patchFlowWidgets` (`PL/🌊️flow/…/🎮️commands/🩹️patch-flow-widgets/🦀️.rs:50` static key `patch-<field>-<ids>`; `✏️editor/🦀️.rs:1269`), static key `generation-values` (`✏️editor/🦀️.rs:1062`) and `duplicateWidget:<id>` (`:1021`): the scrub glue captures `artifact_mutations` only (`OS/🔌️plugin/🛠️tool-machine/🦀️.rs`: 0 mentions of `child`), so F6 needs a child-leaf scrub seam. CAD: engagement scratch is still a window-config snapshot amended under `engagement` (`CA/✏️editor/🎮️commands/🤝️engagement/🦀️.rs:38`, F-9); streamed CAD gumball not built (CAD has `NoTransient`). The report `📓️w3-t-flow-cad-report.md` covers CAD only.
- **Interrupted-edit suspicion: HIGH.** `OS/🔌️plugin/⏪️time-travel/🦀️.rs` (07:52; 1190 diff lines vs the `time-travel-before-s3.rs.txt` snapshot taken 07:39) and the 07:52 flow tests were written after the last compile. Brace-balanced; `plugin-check-2` proves only the pre-07:52 plugin compiles.
- **Remaining checklist.**
  1. Compile `semio-framework-plugin` (lib + tests), then `semio-s-artifact-flow-flow` (+ `OS/🌊️flow/🖥️host`); run the four §12 laws + flow lib suite; fix flow-side failures in `PL/🌊️flow`; runtime-side failures go to the RUNTIME owner (§6).
  2. F6 + the two static keys through the scrub machine with a child-leaf seam (RUNTIME + FLOW); delete the keys.
  3. CAD: rerun lib tests; decide F-9 (engagement scratch → window transient) and the streamed gumball (§8); 3 uncatalogued leaves wait for `schema generate`.
  4. TS: `bun`/vitest for `node-graph-wire-edit`, `ts-history-patch`; wgpu EngineCanvas graph-edit Rust tests.
  5. Acceptance (§12): history lists child-member mutations grouped by `TransactionRef`; `historyEditBegin{mutationId, store?}` → child store; after accept/finalize the parent re-derives from the replayed child; e2e on 6012 if the coordinator boots it.
  6. Write the flow half of the report.
- **Blockers/deps.** B1 (flow depends on stdio semio, mid-rollout), B2 (store `AppendTransaction` mid-edit), B3. Runtime ownership conflict (§2.3.2).
- **Successor brief.** Get the §12 slice compiling first — everything else in flow is written and waiting on it. Run the four laws red→green (they define what "composed-child time travel" means), then convert F6. Keep CAD as is unless you can finish F-9 cheaply. Do not edit `⏪️time-travel`/`🔌️plugin/🦀️.rs` concurrently with another WP: under §6 the RUNTIME owner holds those files, so send precise requests or take the slice as a joint phase-1 task.

### 4.5 W3-T-LAYOUT
- **Last assignment.** Launched 22:33 (opus): gumball → relative `drag-frames`/`rotate-frames`/`scale-frames`, Blueprint transform tool in the window transient, host aborts, overlay protocol from SPATIAL.
- **Done.** Report `📓️w3-t-layout-report.md` §1–§4: lints 0 (146/146 inputs of 48 leaves, 62/62 payloads), numpy/jsonschema cross-checks, `verify layout-document-contract|layout-window-ownership|layout-frame-selection` exit 0. Cargo: test build OK 08:04; **`cargo test -p semio-s-artifact-layout-layout --lib` 588 passed / 2 failed** (`G/w3-t-layout/test-2.txt`; 582/8 at 07:45).
- **Red now.**
  1. `editor::layout::component::transform_tool_transaction_tests::one_shot_turns_and_scalings_are_one_transaction_each` (`…/✏️editor/🧪️tests/🧪️transform-tool-transactions/🦀️.rs:59`): `plugin.internal: validation failed: batched item candidate failed its exact fixed fold contract`. Likely cause (read `OS/🏪️store/🦀️.rs:15638-15680`, `:18928`): `rotate-frames`/`scale-frames` invert to several absolute rows per frame (`move-frame`+`resize-frame`+`rotate-frame`) while a translate inverts to one, and one-shot translate is green; the default one-item declaration (forward + one inverse row = 2) would then be exceeded. Declare via `ArtifactStoreOneItemFootprint::for_one_item(inverse_rows, bytes)` or reshape the inverse. **(hypothesis, not proven — `fold_batch_item` has other exact conditions; log the failing condition first)**.
  2. `editor::layout::panels::document::tests::frame_rows_declare_their_granularity_while_the_tree_binds_the_one_interaction_select` ("a page row keeps its own action", `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:212`): matches the pre-existing triage item of 17:00 ("page row action") — not this WP.
- **Not verified.** `cargo check` section 5 of the report was never filled; the wasm32-wasip2 plugin check was not run.
- **Interrupted-edit suspicion: LOW** (07:45–07:46 edits were compiled by the 08:04 build).
- **Remaining checklist.** Fix red #1 and add a law for N frames; confirm #2 pre-existing on HEAD of a peer-free build; `cargo check -p semio-s-plugin-layout --target wasm32-wasip2`; update report §5; regeneration: layout descriptor (`describe`), `schema generate` (3 leaf schemas, diff `PagePatch`, transient/config), launch row for `verify layout-frame-selection`.
- **Blockers/deps.** None specific (B3 for `verify`).
- **Successor brief.** One law to fix, one to classify, one plugin check to run. Small; merge with CAD (§6).

### 4.6 W3-T2-CONTROLS (scrub machine, hosts, ToolRun F-1, F-4)
- **Last assignment.** Launched 22:05 (opus). Source-complete 07:23 (status); verification blocked by peers.
- **Done (log/read).** F-1: `⏯️tool-run` publishes `Some(TransactionRef)` + localized tool label (`OS/🔌️plugin/⏯️tool-run/🦀️.rs`); `ScrubMachine`/`Scrub`/`ScrubLedger` in `FW/🛠️tool-machine` (Rust + TS twin + schema + fixture `🧫️scrub-law`, last edit 03:30); runtime `OS/🔌️plugin/🛠️tool-machine/🦀️.rs` (`ToolMachineRuntime`), seams in `🔌️plugin/🦀️.rs` (`admit_tool_dispatch`, `settle_tool_operation`, render seams); hosts: lane `FW/🖱️ui/🎬️scene/🟦️.ts` (`abort`), React Interpreter, wgpu slider/stepper (`FW/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`), shared corpus `FW/🖱️ui/🧫️fixtures/🎛️retained-control-commit`; F-4 lint `document_input_commit_lint`; plugins: gis terrain `set_exaggeration` absolute leaf, energy (name fields blur), forms (5 verbs `Emit::mutations`, static keys gone), norm (`bind_on` blur), playbook (`change-title`). Census confirms 0 `Emit::amend` in all five (grep).
  - Verification: `cargo test -p semio-framework-plugin --lib` filter incl. `time_travel_tests` + `tool_machine::tests` **27/27** (`G/w3-t2-controls/plugin-test-2.txt` 02:31); earlier 22:55 run 32/5 → fixed. `semio-framework-ui --lib` **91 passed / 1 failed** (`ui-test-2.txt` 03:02): `wgpu::events::control_commit_tests::every_retained_control_commits_its_own_guest_action`, case `input-number-commits-a-number` still expects the old bare `{value}` shape (corpus vs new `{value, gesture, commit}`); not re-run after (ui-test-3 died on os-kernel). Host TS typecheck (`tsc-hosts.txt` 03:19): only 3 pre-existing peer errors.
- **Unverified.** Plugin end-to-end laws (tick publishes nothing, release = one edit with `TransactionRef`, abort = zero trace) for energy/forms/gis (`gis-test-1..3` all died on peers), full plugin lib (`plugin-test-3` died on `os_store`), norm/playbook tests, React lane vitest, wgpu events corpus.
- **Interrupted-edit suspicion: LOW** (no edits after 07:23; log writes to 07:54 only).
- **Open (report §5).** Playbook descriptor regen; **forms leaf decision** (`replace-block` whole block vs field-parametric leaf; raised, not decided); `setSlider` lane for NodeGraph moved to PROCEDURAL (done there: gesture/commit top-level, `slider_gesture_ui_scope`); derived previews via `provisional()` (landed: `OS/🔌️plugin/🦀️.rs:16598`, used by generation3d `✏️editor/🦀️.rs:714`).
- **Remaining checklist.** (1) Re-run plugin lib (scrub/typing/tool_machine laws) once os-kernel compiles — RUNTIME owner; (2) `cargo test -p semio-framework-ui --lib` and fix the corpus case (update the fixture row to the press shape, keep Rust + TS twin equal); (3) gis terrain, energy (`no_inspector_field_edits_the_document_per_keystroke`), forms (230 baseline), norm, playbook crate tests; (4) vitest/bun for lane + Interpreter; (5) decide forms leaf (§8); (6) hand `document_input_commit_lint` wiring for the remaining apps to CLOSURE; (7) report §3 table (it still says "filled below").
- **Blockers/deps.** B1 (gis crate chain hit stdio-las), RUNTIME for runtime laws.
- **Successor brief.** This WP is code-complete; its debt is proof. Run the framework-ui corpus, the five plugin suites and the lane/Interpreter TS tests, fix the one known corpus case, take the forms leaf decision, and write the verification table. Own `FW/🖱️ui`, `RE/🗣️Interpreter/🟦️.tsx` and the five plugin trees; leave the runtime seam files to RUNTIME.

### 4.7 W3-T2-TEXT (typing runs, §13.2)
- **Last assignment.** Launched into the 22:42 slot (no agent id recorded). Scope (audit WP-TEXT): typing-run machine + explicit-draft policy in `TextWindowKit` (F-5); writer, vcs, trinity (jack query, rewriting JSON/parameter/node graph), stdio md/html (+ json/binary/deflate kit users).
- **Done (grep/read/log).** Framework: `TypingMachine`/`TypingLedger`/`TypingFold`/`TYPING_*` in `FW/🛠️tool-machine/🦀️.rs:971-1235` + TS twin + fixture `🧫️typing-law` (02:52); plugin runtime `commit_typing_before` (`OS/🔌️plugin/🦀️.rs:30056`), `ArtifactApp::typing_fold` (`:13728`, `EditorApp` forwards `:36804/:37981`), `TypingRun` fixture helpers (`artifact_app_laws::typing_run`). Hosts: React `TextEditor` typing run (`createTextEditorTypingRunV1`, in HEAD before 07:21), `👕️canvas-presence` `TextPeerCaretsOverlayV1` (03:36–03:38; shows peers' pending run beside the caret). Plugins: writer `typing_fold` (splice composition, `✒️writer/…/✏️editor/🦀️.rs:1063`), jack `text_edit` (`set-query` whole query, key removed), vcs; no `*_TYPING_COALESCE_KEY` remains (grep). `bun test` text-splice **68 pass** (07:37).
  - **Red.** `cargo test -p semio-s-artifact-writer-writer --lib` (filtered, 23 tests): **17 passed / 6 failed** (`G/w3-t2-text/writer-tests.txt`, 08:09; build 14m48): `a_caret_jump_splits_the_run`, `a_frozen_document_refuses_typing_with_zero_trace`, `a_run_edited_in_history_replays_deterministically`, `one_typing_run_is_one_edit_and_one_row_with_its_transaction`, `two_runs_separated_by_idle_are_two_transactions`, `two_hundred_typed_characters_are_one_edit` (`…/✏️editor/🧪️tests/🧪️typing-runs/🦀️.rs`, written 07:28). Pattern: one **extra** edit row (3 vs 2; `(3,2)` vs `(2,2)`); the replay law ends with `"hello"` kept and `"howdy"`/`!` lost. Passing: `a_typing_burst_is_one_run_one_edit_and_one_undo_step`, the ledger-length law, `two_authors_typing_at_once…`, `a_peer_sees_the_run_only_once_it_commits`. **(inferred)** either the `end_run` helper's `textSplice{commit:<reason>}` delivery is itself published as an edit, or `commit_typing_before` fires twice around the history-edit verbs; the runtime laws that pass suggest a test-helper/verb-routing issue rather than the machine.
  - TS: `bun-carets` 0/5 (`document is not defined` — wrong runner; the test needs vitest/jsdom), `vitest-renderer` 14/14 but `TextEditor/🧪️tests/🪞️echo-pack` fails to load (generated plugin registry TS, peer), `tsc.txt` only peer errors. `lint`/`taxonomy` died on invalid taxonomy (B3).
- **Not done.** Trinity rewriting (`rewriting_snapshot_mutations(state,&next)` scratch diff in `set-lhs-json`, `set-rhs-json`, `set-parameter`, `patch-nodes`, `add-rule-clause-command`, and `node-graph-edit`; none converted); stdio md/html still `ReplaceText → SetSnapshot{snapshot}` per delivery (`PL/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/✏️editor/🦀️.rs:134-136`, html `:134-135`; F-5 fix lives in `TextWindowKit`, `OS/🔌️plugin/🦀️.rs:35534-35714`); json/binary/deflate not examined; vcs/jack adoption tests exist (`jack/…/🧪️tests/🔬️unit/🦀️.rs:328`, `vcs/…:639`) but were not run.
- **Interrupted-edit suspicion: MED.** `OS/🔌️plugin/🛠️tool-machine/🦀️.rs` and `⏪️time-travel` 07:52 were compiled by the 07:54–08:09 writer build, so the tree compiles; the suspicion is a debugging session cut mid-diagnosis (red tests, no fix recorded).
- **Remaining checklist.** (1) Diagnose the extra row (start with `end_run`/`deliver` helper and `commit_typing_before` for `textSplice` with only `commit`); fix in the runtime (RUNTIME owner) or test helpers; (2) run vcs/jack/writer suites; (3) convert trinity rewriting handlers (blur/explicit apply per §13.2; node graph via the flow record → `move-nodes`-style leaf modelled on procedural gen2d `move-nodes`); (4) F-5 in `TextWindowKit` + md/html/json/binary/deflate (explicit draft or typing run; coordinate with the stdio sqlite peer, B1); (5) TS: carets/echo-pack under vitest; (6) lints + taxonomy; report (none exists).
- **Blockers/deps.** B1 (stdio editors), B3; runtime ownership for (1)/(4).
- **Successor brief.** Typing machinery and writer/jack/vcs adoption are in; six writer laws are red with a uniform off-by-one that points at test plumbing or double commit. Fix that first, then finish the breadth: trinity rewriting and the stdio text editors (kit-level F-5). Write a root report — none exists.

### 4.8 W3-T2-STROKES (wfc, process, raster, remodel, §15)
- **Last assignment.** Launched 02:35 (opus, ace6d82c4c565ec71); after the flow record landed it did wfc 2d/3d node moves; §15 decision (transaction-scoped amend) made 05:50 for remodel.
- **Done (read/mtime).**
  - wfc bitmap: leaf `✍️paint-input-stroke` (03:06–03:09, 2 fixture quintets, mutate case py/feature), brush utility/tool (`✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️input/🪛️utilities/🖌️brush/🦀️.rs` 03:14), stroke scratch removed from window config (law `the_brush_verb_and_utility_are_declared_and_answered`), editor 04:48. `cargo check` lib **OK 07:54**; its **test target fails**: `E0609 no field 'id' on type &UtilityRef` at `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:115` (`UtilityRef(String)`, `FW/🛂️manifest/🦀️.rs:3187`; compare via the string accessor). One-line fix.
  - process3d: cursor moved to window config (leaf `change-cursor` deleted 03:28–03:31, `Emit::amend_config(…, PROCESS3D_CURSOR_COALESCE_KEY)` in `🎮️commands/⏱️cursor/🦀️.rs:32`), world gesture tool (`🎮️commands/🌍️world/🦀️.rs`, `ToolMachineRunner`). **Never compiled** after conversion (baseline log only).
  - wfc 2d/3d (written 07:31–07:48, **never compiled**): leaves `✋️drag-slots` + `🎯️set-slot-positions` (diff/inverse/schema/tests, aggregate variants `DragSlots`/`SetSlotPositions`, modules wired in both crate roots, text/proto/graphql/binary/oracle/py/feature updates 07:34–07:40), `🛠️tools/✋️drag/🦀️.rs` using `node_drag_commit` (2d 07:41; 3d 07:45), editors/tests 07:43–07:48. `move-slot` stays as the absolute inverse.
- **Not done.** Raster (no change: `RE/🖌️Paint2dHost/✍️editing/🟦️.tsx:305-315` → `RA/🎮️commands/🎨️edit-pixels/🦀️.rs:56-217`; decision pending: parametric `paint-stroke{layerId,target,points,brush}` with a deterministic rasterizer vs one-edit + `TransactionRef`; prototype the replay cost first, audit §9.4). Remodel (3 `Emit::amend`: `RM/🎮️commands/📼️import-video-frame-payload/🦀️.rs:221`, `🖼️import-frame-payload:128`, `✅️import-video-done:31`; `VideoImportScratch` ×2) — needs the §15 plugin-runtime half (B2); a peer is editing remodel video-engine files and added `🧩️composition/` (11:27–11:40).
- **Interrupted-edit suspicion: HIGH** for wfc 2d/3d (≈70 files, never compiled), process3d (compile unknown), LOW for wfc bitmap.
- **Remaining checklist.** (1) fix the bitmap test, `cargo test -p semio-s-artifact-wfc-bitmap --lib` (incl. `🧪️brush-tool`); (2) `cargo check`/test wfc-2d, wfc-3d, process3d (`--tests`), run `mutate-wfc2d-1`/`mutate-wfc3d-1`/`mutate-bitmap-1` cases and `🧩️mount-contract`; (3) lints (`paint-input-stroke`, `drag-slots`, `set-slot-positions` leafUncatalogued → `schema generate`); (4) raster decision + conversion; (5) remodel after the runtime §15 half; (6) report (only the census exists, "IN PROGRESS").
- **Blockers/deps.** B1 (wfc/3d snapshot sqlite edits are landing in the same crate now: 11:17–11:36), B2/§15 for remodel, flow record (landed).
- **Successor brief.** First make the three wfc crates and process3d compile; they hold ~70 never-compiled files. Then raster and remodel. Expect the sqlite peer to be editing `🧬️schema/📸️snapshot` in wfc/3d concurrently; keep your edits in leaf/editor dirs.

### 4.9 W3-T2-PROCEDURAL (generation2d/3d, incl. generation3d gumball)
- **Last assignment.** Slot at 03:19 (no id, no report). Audit WP-PROCEDURAL + the 21:36 decision that generation3d gumball is its scope.
- **Written (mtime/read).** gen3d leaves `🎚️change-slider-value`, `✋️drag-transforms`, `🔃️rotate-transforms`, `📏️scale-transforms`, `🚚️move-nodes` (03:43–03:44; 8 files each: diff/inverse/schema/ts, **no per-leaf test dir**), wire witnesses (07:45), `🧪️tests/🧪️gesture-leaves` (07:48), aggregate `🦀️.rs` 07:48, protocol 07:23; gen2d leaves `🎚️change-slider-value`, `🚚️move-nodes` (07:41, 6 files each, wire witnesses 07:45). Editors: gen3d `🧭️transforms` gumball tool (460 lines, `ToolMachineRunner`), `translate|rotate|scale-selection`, `node-graph-edit` (07:32), `set-widget-input`, `patch-flow-widgets`, `update-generation-values`, `🧬️generation` (07:26–07:27), editor `🦀️.rs` 07:38 (provisional overlay at `:714`); gen2d `node-graph-edit` (07:43: `move`→`move-nodes` via `node_drag_commit`, `setSlider`→absolute leaf, structural rows via `host_operations`), `🧬️generation` (07:44). No `Emit::amend`/static keys remain in procedural (grep). One-shot scratch authors remain: gen3d `commit_host_snapshot` in add/remove-widget, delete-selection, set-widget-input, knife/edit-mesh-selection, reorganize, node-graph structural rows; gen2d `host_operations` in connect-media-ports, reorganize, remove-widget (class O*).
- **Verification: none.** `G/w3-t2-procedural/check-g3-1.txt` (07:52) died on os-kernel (`DOCUMENT_BACKBONE_BATCH_*` missing).
- **Interrupted-edit suspicion: HIGH** (≈45 files in 07:23–07:48, never compiled; no leaf test dirs for the 5+2 new leaves; schema `mutation-*` lints never run).
- **Remaining checklist.** (1) `cargo check` + test `semio-s-artifact-procedural-generation3d` and `…generation2d` (private target dir); (2) leaf tests/quintets or wire witnesses validated by `schema mutation-payloads`; lints `mutation-inputs`; (3) the footprint audit for transform leaves (§2.3.1); (4) one-shot O* leaves optional (`insert-mesh-operation{operation,targets,params}` makes mesh-op inputs editable); (5) gumball host protocol consumer parity with SPATIAL (stable); (6) write the report.
- **Blockers/deps.** B1/B3; CONTROLS scrub (landed), flow record (landed), SPATIAL host protocol (landed).
- **Successor brief.** Everything is source-written; start by compiling both crates and fixing the (probably many) first-contact errors, then add the missing leaf tests and run the lints. Do not redesign: gen2d `move-nodes` + `generation2d_node_drag_leaves` is the template GRAPHS will copy.

### 4.10 W3-T2-GRAPHS (never started)
- **Scope (audit §7 row 4, updated).** dag, sequence, mathematical, **hub space** (`HUB/🪐️space/⚙️engine/🪐️space/…`, moved 04:27). Templates now exist: gen2d `move-nodes` + `node_drag_commit`, flow `drag-nodes` + `flow_drag_tool_commit`, wfc `drag-slots`.
- **Remaining checklist (grep-verified sites).**
  - dag: `PL/🕸️dag/…/🎮️commands/🚚️move-media-node/🦀️.rs:20` `Emit::amend(move_node, "move-{id}")`; `🩹️patch-dag-nodes/🦀️.rs:46` `Emit::amend(…, "patch-{field}-{ids}")` (slider value/min/max live drag → scrub + `set-slider{nodeId, value|min|max}` leaf instead of `replace_node_kind`+`resize_node`); `✏️node-graph-edit/🦀️.rs:54-63` snapshot diff (`dag_snapshot_mutations`) → `move-nodes{ids,dx,dy}`.
  - sequence: `SQ/…/🎮️commands/🕸️node-graph/🦀️.rs:21-56` (`sequence_child_emit_from_host_mutation`, whole composed-child publish) + `✏️editor/🦀️.rs:931` — needs the composed-child transaction (FLOW §12) first.
  - mathematical: `MA/…/🎮️commands/🕸️node-graph-edit/🦀️.rs:62` rebuilds `equation_graph` and emits `ReplaceGraph{whole graph}` per op; `🧮️set-algorithm` `Emit::commit(ReplaceGraph)` → leaves `move-nodes`, `add-node`, `connect`, `delete-nodes`.
  - space (hub): `🚚️move-media-node/🦀️.rs:18` `Emit::amend(MoveNode, "moveMediaNode:{id}")`; `✏️node-graph-edit` (`setHostSnapshot` at `:24`); parameter/number/text inputs (`🩹️patch-parameter`, `🔧️patch-media-nodes`, `🩺️patch-app-instances`) → `commit("blur")` + scrub (F-4).
  - architect/reasoning: intent one-shots only, no work.
- **Blockers/deps.** FLOW §12 for sequence; CONTROLS scrub; hub-composition descriptors regenerate in GATES.
- **Successor brief.** Copy the gen2d/flow pattern per guest: relative leaf (schema-first, `x-semio-ui`, witnesses), `node_drag_commit` in the node-graph handler, delete `Emit::amend`. Do sequence last.

### 4.11 W3-T2-CLOSURE (never started)
- **Scope.** After all conversions: delete `Emit::amend` (`OS/🔌️plugin/🦀️.rs:12814`), the artifact-lane `Emit.coalesce_key`/`description`-for-gestures use (31 mentions of `coalesce_key` in the file), `ArtifactCommand::AmendLast`/`AmendLastInLane` (plugin 11, store 31 — the store is another session's file, B2), `UtilityPreviewContract` (3), the config-lane policy question (§8); wire `document_input_commit_lint` into every app's tests; extend the policy predicates (`FW/⏯️tool-run/🧪️tests/🔬️interactivity-tool-run-policy/🟦️.ts` mentions `Emit::amend` ×6; root `📜️script.ts` `ToolRunPolicy`) to fail on `Emit::amend`/artifact `coalesce_key: Some`/described gesture commits.
- **Sites that must be gone first (grep at HEAD).** `Emit::amend(`: remodel ×3, dag ×2, hub space ×1. Static artifact keys: flow `patch-flow-widgets:50`, `✏️editor/🦀️.rs:1021/1062/1269`; puzzle 3d `PUZZLE3D_SET_ACTIVE_EXAMPLE_COALESCE_KEY` (`✏️editor/🦀️.rs:5310`, used `:5576` — an artifact edit with a static key on example load); note retained pass-through. Config-lane `amend_config` (shooting camera, process3d cursor, cad engagement, fem playback) and viewer-camera keys (gis, energy) are view/config state.
- **Dependencies.** Everything else + §15 plugin half for remodel + store owner.

### 4.12 W3-T-GEN3D
- Never spawned; merged into PROCEDURAL by the 22:05 decision. The gen3d gumball code exists (§4.9); host protocol is SPATIAL's and stable.

## 5. Repo-wide census of REMAINING non-machine gesture commit paths

Commands (all from `/Users/ueli/Documents/semio`, `git grep -n -I`, tracked files, worktree state at 12:05):
`-E 'Emit::amend\(|amend_config|AmendLast|coalesce_key|transformBegin|transformEnd|ToolMachineRunner|ToolYield|ScrubMachine|ScrubLedger|TypingMachine|TypingLedger|statechart!|mod fsm|coalesce' -- '✏️s/🔌️plugins/**/*.rs|ts|tsx'` (450 hits); `-E '\bamend[a-z_]*\(|\bAmend[A-Za-z]*\b|COALESCE_KEY|coalesce_key'` (62 hits, tests/fixtures excluded); `-E 'transformBegin|transformEnd|GUMBALL_GESTURE_BRACKET|…'` repo-wide (195 hits, mostly old ticket folders); `-E 'Emit::amend\(|AmendLast'` over `🧰️framework` and `🌎️hub`; `git ls-files | grep -i fsm` (0); scratch/`*_snapshot_mutations`/`host_operations`/`commit_host_snapshot` pass.

Key counts:

| Pattern | Non-test hits | Where |
|---|---|---|
| `Emit::amend(` | **6** | remodel 3, dag 2, hub space 1 (+ definition `OS/🔌️plugin/🦀️.rs:12814`) |
| artifact-lane static `coalesce_key` | **5 sites** | flow 4 (`patch-flow-widgets:50`, editor 1021/1062/1269), puzzle 3d `set-active-example` (1) |
| config/view-lane `amend_config` / `coalesce_key` | ~14 | shooting camera 2, process3d cursor 2, cad engagement 1, fem playback 5, gis/energy viewer 3, forms try-value (transient) 1 |
| `AmendLast` | plugin 11, store 31, vcs 0; plugin trees 2 (comments) | closure |
| `transformBegin\|transformEnd` | lowpoly 28 non-test lines (4 files) + 11 test/feature lines; manifest 3; plugin comment 1 | SPATIAL |
| vendored `fsm` modules | **0** | draw's deleted |
| `ToolMachineRunner\|ToolYield` users | note, draw, puzzle 2d/3d/5d, fem 2d/3d, wfc bitmap, layout, procedural gen3d, cad, process3d, shooting (13 artifacts) | done |
| `node_drag_commit\|NodeDragRecord` users | flow, wfc 2d, wfc 3d, procedural gen2d, gen3d (5 artifacts) | done (uncompiled except flow's leaf crate) |
| Scrub/typing in plugins | gis terrain, energy, forms, draw, animate, process3d, procedural (scrub); writer/jack/vcs (typing) | done |

| Plugin / artifact | Remaining non-machine gesture commit paths | Class | Proposed WP |
|---|---|---|---|
| puzzle 2d | none (W2-D) | — | PUZZLE (follow-ups only) |
| puzzle 3d / 5d | machines done, unverified; pivot/re-solve/paging gaps; 3d `set-active-example` static key | G | PUZZLE (+CLOSURE key) |
| block 2d/3d/5d | none (one click, config) | — | — |
| draw | machines done, unverified; lint finding | G | INK |
| note | machines done, compile OK | G | INK |
| shooting | machine done, never compiled; camera = config | G | SPATIAL |
| fem 2d/3d | machines done; stale counts; playback config keys (fine) | G | SPATIAL |
| lowpoly | **gumball + brackets + `LowpolyScratch` + paint stroke session** | G | SPATIAL |
| layout | machine done; one red law | G | LAYOUT-CAD |
| cad | machine done; engagement scratch in window config amend (F-9); streamed gumball | G | LAYOUT-CAD |
| flow | node drag done (uncompiled); **F6 `patchFlowWidgets`**, `generation-values`, `duplicateWidget` keys | G/C | FLOW |
| procedural 2d/3d | converted, never compiled; O* one-shots via `commit_host_snapshot`/`host_operations` | G/C/O* | PROCEDURAL |
| writer / vcs / jack | typing machine done; writer 6 laws red | T | TEXT |
| trinity rewriting | **JSON/parameter/node graph via `rewriting_snapshot_mutations` scratch diff** | T/G | TEXT |
| stdio md, html (+ json/binary/deflate) | **`SetSnapshot` per delivery (F-5)** | T | TEXT (kit in RUNTIME) |
| wfc bitmap | machine done; test target compile error | G | STROKES |
| wfc 2d/3d | `drag-slots` machines written, never compiled | G | STROKES |
| process3d | cursor→config + world tool done, never compiled | V/O | STROKES |
| raster | **brush/eraser/mask stroke published as PNG blobs** | G | STROKES |
| remodel | **per-frame `Emit::amend` import (3), `VideoImportScratch`** | I | STROKES (needs RUNTIME §15) |
| dag | **`Emit::amend` ×2, snapshot-diff node-graph edit** | G/C | GRAPHS |
| sequence | **child-lane whole-snapshot node-graph publish** | G | GRAPHS (needs FLOW §12) |
| mathematical | **`ReplaceGraph` whole-graph per op** | G | GRAPHS |
| space (hub composition) | **`Emit::amend` move, `setHostSnapshot`, per-keystroke parameter inputs** | G/C | GRAPHS |
| energy / forms / norm / playbook / gis terrain | scrub/blur done; forms `replace-block` leaf shape (§8) | C | CONTROLS |
| gis map | `move-feature` one-shot scratch diff (no host drives it) | O* | — (note only) |
| animate | `patch-tile-crops` one-shot (driver unknown) | O | — |
| architect, reasoning, sourcing, imperative, demonstrator, home | intent one-shots only | O | — |
| hosts | Paint2dHost (raster) | G | STROKES |
| hosts | World3dHost, Canvas2dGumballOverlay (done) | G | SPATIAL |
| framework | `Emit::amend`, `AmendLast`, static keys, `UtilityPreviewContract`, policy gates, F-4 wiring, F-5 kit | closure | RUNTIME |

## 6. Proposed work packages (12, exclusive ownership)

Rules for all: private `CARGO_TARGET_DIR=…/⚡️cache/cargo/target-nde-<wp>`, gated foreground builds, re-read before edit, no git writes, no ticket close, `[DEBUG]` hygiene. Because of load 83 / swap 9 GB, launch ≤ 6 concurrent builders; the order below is a recommended dependency order.

| # | WP | Exclusive ownership (files/trees) | Depends on | Size |
|---|---|---|---|---|
| 1 | **RUNTIME** (phase 1 verify + fix; phase 2 CLOSURE) | `OS/🔌️plugin/**` (incl. `🦀️.rs`, `⏪️time-travel`, `🛠️tool-machine`, `⏯️tool-run`, tests, fixtures), `FW/🛠️tool-machine/**`, `FW/⏯️tool-run/**`, `FW/🎠️kernel/**`, root `📜️script.ts` policy region. **Not** `OS/🏪️store`, `OS/🌿️vcs`, `OS/📡️spr` (W1-G / paged-ledger session). | B2 for §15 and `AmendLast`; others finish before phase 2 | M + S |
| 2 | **PUZZLE** | `PL/🧩️puzzle/**`, `OS/♾️infinite/🎲️board/**`, `RE/🖥️Board2dHost/**` | RUNTIME compile | S–M |
| 3 | **INK** | `PL/🖍️draw/**`, `PL/🗒️note/**`, `RE/🖋️InkCanvasHost/**`, `RE/🎞️Scenes/**`, `RE/🗣️Interpreter/🎯️targets/🧊️wgpu/**` | RUNTIME, B1 | S |
| 4 | **SPATIAL** | `PL/🎥️shooting/**`, `PL/🏗️fem/**`, `PL/💠️lowpoly/**`, `RE/🌐️World3dHost/**`, `RE/📐️Canvas2dHost/**`, `FW/🛂️manifest/🦀️.rs` (gumball region) + `🧫️fixtures/🖐️gumball-verb-audience.json` + `🧪️tests/🖐️gumball-verb-audience/**` | RUNTIME | L (lowpoly) |
| 5 | **FLOW** | `PL/🌊️flow/**` (incl. `🧩️extensions/**`), `OS/🌊️flow/**`, `RE/🕸️NodeGraph/**`, `RE/⚙️EngineCanvas/**` (wgpu graph + gumball + board coalescer file), the `🌊️flow` subset dirs inside `PL/🗄️stdio/🗿️artifacts/🧿️semio` | RUNTIME (§12 compile), B1, B2 | M |
| 6 | **LAYOUT-CAD** | `PL/📏️layout/**`, `PL/📐️cad/**` (incl. extensions) | RUNTIME (compile) | S |
| 7 | **CONTROLS** | `PL/🔋️energy/**`, `PL/📋️forms/**`, `PL/📖️playbook/**`, `PL/🌍️gis/**`, `PL/📕️norm/📇️registry/🧬️contract/🖥️app-surface/**`, `FW/🖱️ui/**`, `RE/🗣️Interpreter/🟦️.tsx` (+ its tests) | RUNTIME laws green | S–M |
| 8 | **TEXT** | `PL/✒️writer/**`, `PL/🌿️vcs/**`, `PL/🔱️trinity/**`, editors of stdio `📝️md`, `🌐️html`, `🧾️json`, `💾️binary`, `🗜️deflate`, `RE/✏️TextEditor/**`, `RE/👕️canvas-presence/**` | RUNTIME (typing diag + `TextWindowKit`), B1 | M |
| 9 | **PROCEDURAL** | `PL/🌀️procedural/**` | RUNTIME compile, SPATIAL host stable | M |
| 10 | **STROKES** | `PL/🀄️wfc/**`, `PL/🏭️process/**`, `PL/🖨️raster/**`, `PL/📸️remodel/**`, `RE/🖌️Paint2dHost/**` | RUNTIME §15 (remodel), B1 | M–L |
| 11 | **GRAPHS** | `PL/🕸️dag/**`, `PL/🎬️sequence/**`, `PL/➗️mathematical/**`, `HUB/🪐️space/**`, `PL/🪐️space/**` | FLOW §12 (sequence), CONTROLS | M |
| 12 | **GATES** | generated/central artifacts only: plugin descriptors (`🔣️.json`, `🛂️.descriptor.semio`), `OS/🔌️plugin/📇️registry/🤖️generated/**`, `📚️library/🔣️schema-catalog.json`, `📚️library/🔣️taxonomy.json`, `HUB/*/🔣️.json`, `.vscode/launch.json` + seed, probe `T/🔍️time-travel-probe.ts` | all | S |

Notes: RUNTIME is the only owner of `OS/🔌️plugin/🦀️.rs`; other WPs send precise requests (symbol, region, acceptance test) and keep going. FLOW is the only owner of `RE/⚙️EngineCanvas/…/🦀️.rs`; if SPATIAL needs a wgpu gumball fix it goes through FLOW. Space, wfc, procedural and trinity node graphs stay in their own plugin trees (STROKES, PROCEDURAL, TEXT, GRAPHS) and copy the flow/gen2d pattern; none edits `RE/🕸️NodeGraph`.

Recommended order: **Wave A** RUNTIME-phase-1, PUZZLE, INK, LAYOUT-CAD (compile/verify, small); **Wave B** FLOW, SPATIAL, TEXT, CONTROLS; **Wave C** PROCEDURAL, STROKES, GRAPHS; **Wave D** RUNTIME-phase-2 (closure), GATES (+ e2e re-run of the 6012/6112 probes).

## 7. Regeneration and gate list (for GATES, nothing was regenerated)

- Descriptors (`describe`): puzzle (still lists `transformBegin/End`, lacks 7 leaf kinds), layout, playbook (`updatePlaybook` text), draw, note, shooting, fem 2d/3d, lowpoly, flow, cad, procedural, writer, wfc ×3, process, energy/forms/norm/gis if their verbs changed; aec descriptor freshness (framework version 20 vs 19, status 19:00).
- `schema generate` (central) for: puzzle 3d/5d leaves, layout (3 leaves + `PagePatch` + transient/config), cad 3 (`leafUncatalogued`), fem `move-selection` ×2 (`leafUncatalogued`), wfc bitmap `paint-input-stroke` (`leafUncatalogued`), wfc 2d/3d `drag-slots`/`set-slot-positions`, draw 4 (subset `🔀️transform`), procedural 5+2, lowpoly `apply-paint-stroke`, tool-machine schemas, `DragNodes`.
- Lints to re-run with 0 target: `schema mutation-inputs --under <plugin>`, `schema mutation-payloads --under <plugin>` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, repo-relative `--under`).
- Launch rows: `verify layout-frame-selection`; any new nx targets.
- `verify taxonomy report --scope` for new dirs (draw `🔀️transform`, `🧪️canvas-tool`, layout tests, wfc, procedural gesture-leaves), `verify dependencies literal-external`, `verify layering`, docstring/`[DEBUG]` sweep (shooting `[DEBUG]`), rust warnings on touched crates (`machine::statechart!` serde warning).
- Wasm: `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2`; stale built wasm under `PL/🧩️puzzle/📦️packages/🦀️rust/{dist,pkg}`.

## 8. Decisions needed

1. **Forms leaf shape** (CONTROLS raised, undecided): keep absolute `replace-block` or add `change-block-field{blockId, field, value}` (recommended: one generic field leaf with a typed value union, hard bounds in the schema).
2. **Raster stroke:** parametric `paint-stroke` with a deterministic rasterizer (editable brush, PNG re-encode on replay) vs one edit stamped with a `TransactionRef` (audit §9.4: prototype first).
3. **Typing-run co-edit** (audit §6.1/F-8): the committed design commits on idle ≤ 1 s with peers seeing the pending run only through presence (`TextPeerCaretsOverlayV1`); confirm product acceptance.
4. **CAD streamed gumball and F-9:** build a CAD window transient for a streamed gumball and move `CadEngagementScratch` out of window config, or accept one-shot-on-release + config amend for the engagement REPL.
5. **Config-lane `amend_config` policy at closure:** keep (camera/playback/cursor/engagement = view state) or also delete; the audit assumed it stays.
6. **Lowpoly relative transform leaf naming** (`transform-mesh` is not an approved verb; fem used `move-selection`, verb `move`).
7. **Fold footprint:** one framework helper deriving the declaration from the leaf's inverse length would remove the whole hazard class (§2.3.1).

## 9. Files read (evidence index)

`T/📋️design.md`, `🧭️plan.md`, `📓️status.md`, `📓️audit-remaining-tools.md`, `📓️w3-t-{puzzle,draw-note,spatial,flow-cad,layout}-report.md`, `📓️w3-t2-{controls,strokes}-report.md`, `📓️api-scrub-machine.md`, `📓️w2-d-report.md`; `G/w3-t-puzzle`, `G/w3-t-draw`, `G/w3-t-spatial`, `G/w3-t-flowcad`, `G/w3-t-layout`, `G/w3-t2-controls`, `G/w3-t2-text`, `G/w3-t2-procedural`, `G/w3t2-strokes`; code: puzzle 5d editor/transform work, draw/note census sites, lowpoly editor + new leaf, fem unit tests, shooting gumball, flow drag tool + node-graph tests, plugin runtime (`Emit`, shape rule, typing/scrub seams), `⏪️time-travel`, `🛠️tool-machine`, kernel `HistoryMutationEntry`, store footprint doc, manifest bracket rule, layout/writer/wfc/procedural/process3d files named above. Scratch scripts (brace balance, file lists) live only in the session scratchpad.
