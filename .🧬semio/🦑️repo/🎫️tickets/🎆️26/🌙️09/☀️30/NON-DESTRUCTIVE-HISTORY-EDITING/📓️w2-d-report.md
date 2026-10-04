# 📓️ W2-D Report: Puzzle 2D Select Tool, Gesture Records and Coalescers

Executor W2-D. Scope: puzzle 2d editor (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/✳️any/✏️editor/**`), the board engine (`♾️infinite/🎲️board/**`), React `🖥️Board2dHost`, and the wgpu board coalescer. It also covers the follow-up edits in puzzle 5d, the stories, and the puzzle publication fixture and audit that the event vocabulary change forced.

## 1. What Landed

### Board engine: one gesture, one record
- A pointer gesture publishes **one** `gesture` row at release, with no per-frame edit rows. It has three shapes:
  - `{gestureId, kind: "drag", targets, dx, dy, proximity:[{source,target}]}`
  - `{…, kind: "rotate", pivotX, pivotY, angle}`
  - `{…, kind: "scale", pivotX, pivotY, factor}`
- The `select` row of the same gesture carries the same `gestureId`. Selection staging is deferred to release. A click without motion publishes only its selection.
- Cancelling a gesture publishes nothing dispatchable and restores the selection that existed before the press.
- `nodeDragEnd`, `nodeRotate` and `regionMove` are deleted. `nodeMove` and `transformPreview` stay as transient preview rows that are never dispatched.
- A region body drag is a `drag` record. A region grip resize stays `select` + `regionResize`.
- A group drag records every unlocked member plus one offset. Locked members are excluded by the engine.
- Types: `GestureStage` and `BoardGestureMotion`, with the builders `staged_select` and `gesture`.
- Laws: `🎬️Gestures` region in `➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`.

### Both coalescers: identical, sharing one corpus
- TS `coalesceBoard2dEvents` (`🖥️Board2dHost/🟦️.tsx`) and Rust `coalesce_owned_board_events` (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`) apply the same rules:
  - keep the latest camera, first in the batch;
  - drop transient rows;
  - hold back a `select` tagged with a `gestureId` until its record is in the same batch.
- Shared corpus: `🖥️Board2dHost/🧫️fixtures/🧫️board-event-coalescing/🔣️.json` (18 cases).
- Schema: `🖥️Board2dHost/🧬️schema/🔣️board-event-coalescing/🔣️.json`, with `oneOf` Drag/Rotate/Scale records.
- The corpus is validated with strict Ajv and replayed by both implementations.

### Select tool: a ToolMachine yielding one ToolTransaction
- `🖱️select/🦀️.rs` defines the `select_tool` statechart.
  - `idle` handles `Records` (guarded; yields and commits) and `Stream` (opens the stream).
  - `streaming` handles `Stream`, `Finish` (yields and commits) and `Cancel`.
  - Its effects are `ToolYield<Puzzle2dMutation>`.
- Every selection transform yields the parametric leaf (`drag-selection` / `rotate-selection` / `scale-selection`) plus the recorded `connect-handles`, as ONE `ToolTransaction` committed with `Emit::commit_transaction`.
  - Proximity: the board-recorded pairs come first, then the guest radius search over the moved state for a drop.
  - Edge ids are deterministic: `edge-<source>-<target>`, with a `-2`… suffix on collision.
  - A commit emit never carries a `coalesce_key`.
- The same runner serves every entry point: board drag/rotate/scale records (`applyBoardEvents`), `translateSelection` (keyboard nudge), `rotateSelection`, `scaleSelection`, the HUD `move/rotate/scale` (`engagementSubmit`), and the inspector x/y `delta` (`patchInspectorNodes`). Scratch-fixture diffing for these gestures is deleted.
- Locks: the tool refuses only when **every** target is locked, with one localized notice. A mixed selection commits, and its leaf reports `mutation.partial`.
- Targets: `puzzle2d_unique_targets` dedupes in first-seen order, both at record construction and in `mutation()`. Target-less, non-finite and factor ≤ 0 records leave zero trace and raise no lock notice. This follows the W1-F leaf invariants.

### Multi-dispatch gestures: runner persisted in the window transient
- `Puzzle2dWindowTransient.select_tool: Option<Puzzle2dSelectToolState>` holds:
  - the statechart configuration by stable ids;
  - the verb, the authoring seed, and the base revision it opened on (hex);
  - `connect`;
  - the open `TransactionRef`;
  - the entries, in value form.
- It is mirrored in the Rust config runtime, the window JSON schema (`Puzzle2dSelectToolState`, strict-Ajv clean) and the TS twin.
- `Puzzle2dSelectTool::start/resume/send/abort/persist` wraps W1-C's `ToolMachineRunner::resume/into_parts/abort`. It runs on the host clock (`default_now_ms`).
- The transform verbs take `phase`:
  - absent: one-shot, one transaction per dispatch;
  - `stream`: ticks accumulate in ONE open transaction;
  - `commit`: publishes the net leaf under the ref minted at the first tick;
  - `abort` with a `reason`.
- Host aborts, each leaving zero trace and clearing the transient:
  - `blur`, `captureLost`, `frozen`: sent as `phase:"abort"`.
  - `captureLost` also fires when a one-shot or a different verb interrupts an open gesture.
  - `baseMoved`: the admission's `canonical_base_revision` differs from the one the gesture opened on.
  - `retired`: the window's active utility is no longer `select`. It fires at the first verb whose publication contract admits the window-transient lane. Until then the preview is not painted, because the preview is only painted under `select`.
- Preview: `render_body` paints `puzzle2d_select_tool_preview(document, state)` for the owning window only. It is never history.
- Publication contracts: `translateSelection`, `rotateSelection`, `scaleSelection` and `patchInspectorNodes` gain the `WindowTransient` lane, in both the Rust factory and `🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json`.

### History labels
- `ArtifactEditor::mutation_label` uses the leaf's `SemanticMutation::label`. A drag row reads "Drag 2 items by (80, 40)" in English and "2 Elemente um (80; 40) ziehen" in German; the German label uses W1-F's decimal-comma formatting.

## 2. Verification (all run by W2-D)

| command | result |
|---|---|
| `cargo test -p semio-framework-os-infinite --lib -- directed_normal` | 52 passed |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- board2d` | 3 passed (includes the shared corpus replay) |
| `bun ./📜️script.ts test long board-event-coalescing` (React pkg) | 19 passed |
| `bun ./📜️script.ts test long engine-contract -t "puzzle 2d\|board 2d\|live mirror\|hover out of the board"` | 39 passed |
| `cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib` | 1052 passed, 13 failed (see §3) |
| `cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib -- a_board_gesture_drag board_node_delete apply_board_events language_neutral_fixtures` | 4 passed |
| `cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle` | ok (after the last edits) |
| `cargo check -p semio-s-artifact-trinity-rewriting --features component-app-assembly --tests`, `-p semio-s-artifact-trinity-jack --tests` | ok |
| `bun ./📜️script.ts publication-authority-audit` (puzzle pkg) | all 3 owners validated |
| `bun ./📜️script.ts verify taxonomy report --scope …` | clean for `🧪️select-tool`, `🧪️select-tool-transactions`, `🖱️select`; the new Board2dHost dirs are clean (the three existing Board2dHost errors predate this work) |

Laws added or updated in puzzle 2d:
- `🧪️tests/🧪️select-tool-transactions` covers:
  - one drag = one edit, one row and one transaction;
  - drop + connection = one transaction;
  - cancel leaves zero trace;
  - two drags = two transactions;
  - a streamed ring rotation = one transaction;
  - three nudges = three transactions;
  - rotate and scale verbs;
  - HUD move and inspector delta;
  - mixed locks;
  - the en/de row label;
  - the four dispatch-threading laws: stream then commit, every host abort, a one-shot interrupt, a repeated-id selection;
  - the emit carrying its transaction.
- `🧪️tests/🧪️select-tool` covers:
  - gesture decoding;
  - yields and edge ids;
  - the deterministic ref;
  - zero-trace requests;
  - the pivot;
  - the chart shape;
  - persist → resume → commit;
  - every host abort reason;
  - tampered-state refusals;
  - the wire round trip;
  - repeated and inadmissible targets.
- Existing laws were updated: locks, proximity-connect, board-host, linking, and the declared-action roster (it now skips the framework's time-travel, cancel and export/import verbs).

## 3. Open Items and Blockers (not W2-D code)
1. **Framework drops typed-op window-transient publications.** After a retained verb that emits `EphemeralEmit.window_transient`, the window transient generation stays 0 and the snapshot is unchanged. The existing law `exact_overview_window_transient_isolates_abort_and_resets_on_reload` (engagementInput) fails the same way.
   - Five app-level streamed laws fail at the "gesture is open" / preview assertion for this reason: `a_gesture_streamed_over_several_dispatches_is_one_transaction`, `a_host_abort_mid_gesture_leaves_zero_trace`, `a_one_shot_transform_interrupts_an_open_gesture`, `leaving_the_select_utility_retires_an_open_gesture`, `a_document_moved_under_an_open_gesture_aborts_it`.
   - The same logic is proven at the dispatch boundary by the threading laws, which pass.
   - Likely source: the concurrent plugin-runtime refactor (W2-A time travel, removed `A::ephemeral` hook).
2. **Framework widens ui_scope to Full.** `camera_event_declares_window_only_ui_scope`, `empty_board_events_declare_none_ui_scope` and `select_action_declares_partial_ui_scope` fail even though the app emits the narrow scope. This was confirmed with a temporary `[DEBUG]` log, which has since been removed.
3. **Brush and fill job laws** fail in code W2-D did not touch: `board_fill_*` ×3 and `fill_run_job_places_only_inside_visible_target_regions`.
4. **No framework tool-abort hook.** Blur, time-travel freeze and utility change are not delivered to apps. Hosts must dispatch `phase:"abort"` with a `reason`, and utility retirement is applied lazily as described in §1.
   - Recommended: a framework `abort_tools(window, reason)` call made on blur, freeze and utility switch.
   - The framework also refuses artifact-lane verbs while frozen, so `frozen` only reaches the app through a transient-lane dispatch.
5. The `phase` and `reason` arguments are host protocol only and are deliberately not declared as palette `ActionArgDef`s.
6. The wgpu plan path does not drive the rotate ring. React/wasm does, and the wgpu coalescer handles its records.

## 4. Files (W2-D)
- Engine: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs`, `…/➕️normal/🦀️.rs`, `…/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`
- wgpu: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`, `…/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs`, `🧊️wgpu-standalone/🦀️.rs` (legacy coalescer removed)
- React: `…/🧱️elements/🖥️Board2dHost/🟦️.tsx`, `…/🖥️Board2dHost/🧫️fixtures/🧫️board-event-coalescing/🔣️.json`, `…/🖥️Board2dHost/🧬️schema/🔣️board-event-coalescing/🔣️.json`, `…/🖥️Board2dHost/🧪️tests/🧪️board-event-coalescing/🟦️.ts`, `…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`, `…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
- Puzzle 2d (ED = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`), plus `◻️2d/📦️packages/🦀️rust/Cargo.toml`:
  - `ED/🦀️.rs`
  - `ED/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs`
  - `ED/🎮️commands/{🎲️apply-board-events,🚀️translate-selection,🔄️rotate-selection,📏️scale-selection,📨️engagement-submit,🩹️patch-inspector}/🦀️.rs`
  - `ED/🪟️window/🦀️.rs`, `ED/🪟️window/🧬️schema/{🔣️.json,🟦️.ts}`, `ED/🎚️config/🦀️.rs`
  - `ED/🧪️tests/{🧪️select-tool,🧪️select-tool-transactions,🔬️unit,🔬️locks}/🦀️.rs`
  - `ED/🎮️commands/🔗️proximity-connect/🧪️tests/🔬️unit/🦀️.rs`, `ED/⚙️engine/{🎲️board-host,🔗️linking}/🧪️tests/🔬️unit/🦀️.rs`
- Puzzle 5d (P5 = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any`):
  - `P5/✏️editor/🦀️.rs` (gesture drag stage, each id once)
  - `P5/✏️editor/🎮️commands/🎲️apply-board-events/🦀️.rs`
  - `P5/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (new law)
  - `P5/🧫️fixtures/🗄️retained-jobs/🔣️.json`
- Stories: `✏️s/🔌️plugins/🧩️puzzle/📖️stories/{🎭️2d-board,🎭️2d-fixtures}/🧪️.story.tsx`
- Puzzle package: `✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json`, `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts` (stale dispatch-pipeline audit strings realigned)
- Command outputs: `🗑️generated/w2d-*.txt` (tool output, left for the coordinator's sweep).

## 5. Follow-up (Coordinator Requests After the First Report)

### 5.1 Brush/fill failures: bisect verdict and root fixes
- **Not a regression of `48d881aa7ab`.** That auto-commit touches the board host only in W2-D's gesture regions:
  - `GestureStage`, `BoardEventKind`, the gesture builders;
  - the pointer down/move/up/cancel handlers;
  - the close steps.

  No line of the fill job (`BoardFillJob`, its stages, `restore`/`adopt_checkpoint`, `publish_prefix`, the step epilogue) changed after 2026-08-25 (`git log -L`). The test files changed last on 09-08/09-23.
- The same four laws were already recorded red on 2026-09-22, in `☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP/📓️audit-native-summary.md` (run12) and `📓️block-puzzle.md` ("board-engine fill family ×3 + `fill_run_job_places_only_inside_visible_target_regions`").
- Root causes found by tracing every fill stage and outcome. The first two are engine bugs, the last two are test/fixture contract bugs:
  1. **The step epilogue swallowed every unit's preview.** After a completed unit, `BoardFillJob::step` consumed fuel and returned `Yield` whenever `should_yield()`. Under `fuel_per_step: 1` that holds after every unit, so a mounted fill never published a `PreviewReady`. Fix: a completed unit always publishes its preview. `Yield` stays only at entry, before a unit.
  2. **A resumed checkpoint overwrote an unclaimed placement.** A checkpoint carries the placement it published as the owner's hand-off. `BoardFillJob::restore` resumed one whose placement nobody had claimed. The next `AcceptCandidate` then assigned over it, the `Drop` assertion panicked on the pool worker, and the session reported a code-less fault at `AcceptCandidate`. Fixes:
     - `restore` and `adopt_checkpoint` refuse a checkpoint that still holds its placement.
     - `accept_candidate` refuses with `placement-unclaimed` instead of dropping.
     - The production path (`Puzzle2dFillRunJob::accept`) already claims before it adopts.
  3. **The checkpoint law never claimed the placement.** `take_first_fill_checkpoint` now claims the published placement and returns its witness. The law compares the claimed first placement plus the resumed run with the uninterrupted run. New law: `a_checkpoint_with_an_unclaimed_placement_never_resumes`.
  4. **Two test inputs did not match the engine's actual behaviour.**
     - The field-stage law asked a `count: 1` fill for a checkpoint, but the last placement rides the commit, never a checkpoint; it now requests 2.
     - The `targetRegion` vector's `halfSpan: 120` no longer admitted any placement (W1 block-puzzle had diagnosed this). It is re-derived from the measured unconstrained reach (148, ≈188, 276, ≈316, 404) to `200`, and the derivation is recorded in the fixture description.
- Results:
  - Passing now: `board_host_brush_fill_checkpoint_restore_matches_uninterrupted_replay`, `board_fill_candidate_acceptance_exposes_every_retained_field_stage`, the new unclaimed-placement law, and `fill_run_job_places_only_inside_visible_target_regions`.
  - Still red, timing only: `board_fill_job_large_host_has_no_step_at_or_above_eight_ms` and `fill_run_job_drive_step_stays_below_the_interactive_ceiling_for_nakagin`, measured at load averages 30–60 (worst 10.8 ms and 5.6 ms). The overrunning step was `PrepareSources`, an O(1) unit, so the overrun is preemption, not work. The fill-run driver gives the search unlimited fuel, so fix 1 does not change its per-step cost. Both laws need a rerun at low load.

### 5.2 Gesture drag offsets without f32 noise
- Engine: `board_pointer_offset` (public, `➕️normal/🦀️.rs`) records drag `dx`/`dy` as the shortest decimal of their f32 value (`80.00003051757813` → `80.00003`, `0.30000000000000004` → `0.3`). Rotation angles keep f64, because a snapped angle is an exact multiple and narrowing it would move every orbit off its pose.
- React host: `board2dFloat32Decimal` canonicalizes `clientX`/`clientY`, which Chromium reports as f32, before the engine sees them. So an 80 px drag between f32-born coordinates records exactly `80`.
- Shared corpus: `🖥️Board2dHost/🧫️fixtures/🧫️float32-decimal/🔣️.json` with schema `🖥️Board2dHost/🧬️schema/🔣️float32-decimal/🔣️.json`.
  - TS test `🧪️tests/🧪️float32-decimal`, registered in the React suite: 10/10.
  - Rust replay `the_engine_offset_form_replays_the_shared_f32_decimal_corpus` in the wgpu board2d tests. WRITTEN BUT UNVERIFIED: the last wgpu run was blocked by peer compile errors in `semio-framework-plugin`.
  - Engine law `a_drag_offset_is_recorded_without_f64_pointer_noise`: engine suite 53/53.

### 5.3 Example loader no-op
- `Puzzle2dActiveExampleWork`'s `Catalogs` stage now pushes `replace-kind-catalogs` only when the document's catalogs differ from the example's, matching the leaf's own no-op rule; the manifest stage was already guarded. Kit import was already guarded (`catalog_changed`).
- New law `an_example_reload_publishes_no_no_op_mutation`: a reload re-states neither catalogs nor the manifest id, and no reloaded op diffs as `mutation.no-op`. Passing.

### 5.4 Coordination notes
- W2-A's typed host events now reach the select tool as `translateSelection{phase:"abort", reason}` through `ArtifactEditor::host_event`, which builds the variant directly; that edit is kept. The production path no longer calls the test-only `Puzzle2dCommand::from_action`.

## Session 2 — 2026-10-01

Successor S2-W2D (coordinator `⚪552b484a…`). ED = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`,
R = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`, P = `🧰️framework/🛍️products/💻️os/🔨️modules`.
Status: SOURCE COMPLETE; verification in progress (see S2.6, updated at every milestone).

### S2.1 Repair (rule 21)
- No half-finished W2-D edit: every W2-D file was last touched ≤ 03:40; the 06:22–06:28 edits in `ED/🦀️.rs`, the board-host
  tests and the board `➕️normal/🦀️.rs` are the peer graph-manifest rework (committed in `4e36b2b5012`).
- The predecessor's 08:01 lib run (`🗑️generated/w2d-test-all.txt`): 1071 listed, 1056 ok, 9 FAILED, then SIGABRT in
  `fill_run_start_complete_finalize_is_one_undo_entry` (a `BoardFillJob` Drop assertion panicking during unwinding).
  - **Roster law** `every_declared_action_resolves_to_a_command` (`ED/🧪️tests/🔬️unit/🦀️.rs`): `hostEvent`
    (`semio_framework::HOST_EVENT_ACTION_ID`) joins the framework-owned verbs.
  - **H3 root cause (Nakagin manifest).** The peer's graph-catalog ownership change (`📓️2026-10-01-goal-framework-execution.md`,
    CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT) gave every owner a one-manifest profile; `nakagin` went to trinity jack, so the puzzle
    crate's `crate::graph_manifest::manifest_by_id("nakagin")` was `None`. The Nakagin example (`manifest-id=nakagin`) lost its
    engine kind catalogs, which explains the fill and brush reds. The producer excludes foreign owners, so the fix is ownership.
    Following the draw demo precedent (`🖍️drawing/…/📚️examples/🎬️demo/🖼️assets/🛂️manifest.json`), the puzzle 2d Nakagin example now
    owns its manifest at `◻️2d/…/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🛂️manifest.json`. It is a byte copy of jack's
    `nakagin` source, the vocabulary the central registry compiled before. It is registered in `◻️2d/🛂️manifest/📇️outputs.json`
    and regenerated with the owner task: `bun ./📜️script.ts graph-generate` in `◻️2d/📦️packages/🦀️rust` printed "wrote 2
    manifests", and the generated `🏢️nakagin/🦀️.rs` is identical to jack's except its header.
    - Note for the coordinator: two owners now hold the same vocabulary, jack and the puzzle 2d example. Jack's source file
      still carries a mangled name, `🛂️manifest.jsonnakagin.manifest.json`, which is not mine to fix.
    - `bun ./📜️script.ts graph-wire-check` for the puzzle 2d package fails in its own runner: `cargo test --manifest-path
      Cargo.toml -p semio-s-artifact-puzzle-2d` reports "did not match any packages". That is the peer's script; the crate now
      lives in the `✏️s/Cargo.toml` workspace.
  - **Drop asserts survivable.** The five fill owners in `P/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` (placement,
    checkpoint, capture, ingress, job) now assert `… || std::thread::panicking()`. That is the file's own `BoardFillSnapshot`
    convention: production keeps the law, and a failing test reports instead of aborting the whole binary.
  - A peer's `DslValue::Bytes` (11:09) left a non-exhaustive match in
    `P/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs`. I added the exact retirement arm `DslValue::Bytes(value) =>
    self.push(DagOwner::Bytes(value))` so the build could proceed (the coordinator reports this arm as fixed).

### S2.2 (b) wgpu replay of the shortest-decimal offsets
- The 07:49 run (`🗑️generated/w2d-test-wgpu.txt`) already shows `the_engine_offset_form_replays_the_shared_f32_decimal_corpus`
  green (board2d filter 6/6).
- The wgpu host now also maps pointers the way React does. New `board_local_pointer(inner, x, y)` in
  `R/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` takes the shortest decimal of the f32 position and of the surface origin, then
  subtracts them (React: `board2dFloat32Decimal(clientX) - rect.left`). The four puzzle board pointer paths use it: down, move,
  up and wheel. `map_local_pointer` stays for the map surfaces.
- New law `the_wgpu_board_pointer_replays_the_shared_f32_decimal_corpus` in `R/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs`
  covers both the origin and a fractional origin.

### S2.3 (c) + G3/G4/G12 acceptance, schema-first
- Language-agnostic corpus `ED/🧫️fixtures/🧫️select-tool-history/🔣️.json` with its schema
  `ED/🧬️schema/🔣️select-tool-history/🔣️.json`. One board (left, mid, locked pin), which is also the e2e content (import it with
  `importFixture`). Four scenarios, each with steps, expectations, `log`, `drafts` and `head`:
  1. `drag-offset-and-targets` is the dev's example:
     - a board drag of a two-node selection gives ONE row from `drag-selection` with en/de label and `TransactionRef`;
     - a nudge follows;
     - time travel exposes `/targets` as a reference (vortex, node, many) and `/dx` and `/dy` as steppers that snap to the
       window `gridFactor` (rendered step 5);
     - the preview is the state before the drag plus the draft, downstream not applied;
     - the board highlights the referenced nodes;
     - editing `/dx`, then a board pick and "Use selection", replaces the targets;
     - accept gives a ready review, and the overwrite equals the fresh fold.
  2. `rotate-and-scale-replay`: rotate and scale rows labelled from their leaves; `/angle` is a dial; `/factor` is a log slider
     with snaps 0.25 to 4; two sessions edit the angle and then the factor; the downstream transforms replay about the
     recorded pivots.
  3. `fatal-loop-by-editing-targets` (G3): withdrawing the upstream `create-node` makes the downstream drag an Error
     `mutation.target-missing`, so the review is blocked. Next problem, then select, then "Use selection" on `/targets`, then
     accept gives ready, and finalize overwrite reaches the fresh fold.
  4. `warning-from-an-upstream-edit` (G4): withdrawing the upstream unlock makes the downstream drag a Warning `mutation.partial`
     on `pin`. The review is ready, not blocked, and the warning is still on the row after the overwrite. E2E content: this
     scenario on the corpus board.
- Rust laws `ED/🧪️tests/🧪️select-tool-history/🦀️.rs`, mounted in `ED/🦀️.rs`:
  - `every_corpus_scenario_edits_its_leaves_in_history_and_overwrites_to_a_fresh_fold` drives the real verbs:
    - the engine drag and pick, interaction select, the translate, rotate and scale verbs, ingested leaves;
    - all of `historyEdit*`: Begin, Input, UseSelection, Withdraw, Accept, Finalize, Commit overwrite;
    - every expectation is checked: rows, leaf, label, tool, stage, review, blocking, worst, outcomes, next problem, inputs
      (control kind, reference domain, snap source, scale, snaps, rendered step), preview, absent, highlighted, draft;
    - the head is compared with the corpus and with a fresh app folding the edited log.
  - `every_corpus_edit_previews_and_replays_through_the_store` is G12, the siblings' store law. For every scenario it checks
    `state_before` equals the log prefix, runs `begin_report_replay` to finish, requires the report to be non-blocking and to
    match the corpus report, requires the replayed state to equal the fresh fold, then commits an overwrite and checks that
    the head equals the fold.
- The select-tool transaction laws reuse the corpus board and export their board helpers (`painted_host_of`, `press`/`move_to`/`release`,
  `flush`, `dispatched_rows`, `english`/`german`).
- Python oracle `ED/🧪️tests/🧪️select-tool-history/🐍️.py`. It validates the corpus schema and every leaf against its puzzle 2d
  payload schema with `jsonschema`, then re-folds the edited log with shapely `affinity`. It already caught one corpus fault:
  `createNode.node.anchor` is required. `python3 …/🐍️.py` → "4 scenarios, 12 head nodes agree".
- The "keyboard nudge / HUD move / inspector delta / translateSelection reuse the same leaves", "cancel = zero trace" and "two drags
  = two transactions" laws are already in `🧪️select-tool-transactions` (session 1); they are re-run below.

### S2.4 G3 board side: `InteractionView::draft_references` seam (coordinator-approved, generic)
- Runtime (S2-W2A's files, region-scoped edits):
  - `TimeTravelLedger::draft_references()` in `P/🔌️plugin/⏪️time-travel/🦀️.rs` returns the ids every `Reference` input of the
    open draft holds, keyed by the domain the input declares, in draft order with each id once. It is empty while no editor is
    open.
  - `InteractionView.draft_references` and `InteractionView::draft_references(domain)` in `P/🔌️plugin/🦀️.rs`. The field is
    filled at the four render seams (render, window engagements, window measures, context menu). Dispatch, presence stamping
    and the viewer laws use `empty_draft_references()`.
  - Toy-app law `the_open_draft_references_its_reference_inputs_per_domain` in `P/🔌️plugin/🧪️tests/🧪️time-travel/🦀️.rs`.
    The builder-contract test literal gained the field.
- UI scene (S2-W1E's files, additive): `Board2dScene.highlighted_ids_json` (JSON id array, default `[]`, skipped on the wire while
  `[]` so existing goldens are unchanged) in `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` (struct, pack wire, `base`,
  ToValue/FromValue) and TS twin `🎬️scene/🟦️.ts` (`highlightedIdsJson?`). **S2-W1E: please note.** Puzzle 5d's two struct literals
  (S2-PUZZLE) gained `highlighted_ids_json: "[]"`.
- Puzzle 2d: `Puzzle2dInteractionSnapshot.referenced` reads `draft_references("vortex")`, and the board scene carries
  `referenced_json()`. The wasm `BoardSession.setHighlightedIdsJson` (`ED/🌉️wasm/🦀️.rs`) calls the engine's
  `set_highlighted_ids`.
- Hosts:
  - React `R/🖥️Board2dHost/🟦️.tsx` applies `setHighlightedIdsJson` in an effect and adds `data-board-highlighted-ids-json`
    for probes. The type is `R/🪪️WasmSessionLoader/🟦️.tsx` (`setHighlightedIdsJson?`). Engine-contract test "projects the
    ids a time-travel draft references onto the puzzle 2d board host".
  - wgpu: the EngineCanvas board sync (S2-FLOWCAD's file, region-scoped) adds the `BoardSyncCache.highlighted_ids_json`
    field, its close and terminal rules, and `host.set_highlighted_ids`.

- **Chip labels (G3, coordinator 17:40).** The runtime hook `ArtifactEditor::entity_label` already existed (S2-W2A), so
  puzzle 2d implements it in `ED/🦀️.rs` through the new `puzzle2d_entity_label(fixture, kinds, id)`:
  - a node is named by its display label (authored label or text, else the kind catalogue name);
  - a target region by its label;
  - an edge by its endpoint nodes;
  - a handle by its node and handle kind;
  - only the declared kinds are looked up, the label is data in every locale, and `None` (show the id) applies when only
    the id would name the entity.
  - Laws: `reference_chips_name_board_entities_like_the_outliner` (unit), and corpus `chips` expectations: the corpus board
    nodes carry texts Left, Middle and Pin, and the Use-selection chips must read them; a node absent from the preview
    shows its id.
- **wgpu `dumpBoard2d.highlighted` (coordinator 17:40).** `DumpBoard2dSurface.highlighted` holds the ids of
  `Board2dScene.highlighted_ids_json`, the same content as React's `data-board-highlighted-ids-json`. It lives in
  `R/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`, and the law is updated in `R/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs`.
  **S2-W2C: please note this probe field.**

### S2.5 (d) `[DEBUG]`
- `/usr/bin/grep -rn '\[DEBUG\]'` over ED, `♾️infinite/🎲️board`, `🖥️Board2dHost` and `⚙️EngineCanvas`: 0 hits.

### S2.6 Verification (gated, one cargo at a time, private `target-nde-s2-w2d`)
- `python3 ED/🧪️tests/🧪️select-tool-history/🐍️.py`: PASS (4 scenarios, 12 head nodes).
- `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib --no-run`
  (12:52–13:30) was blocked first by the dag `DslValue::Bytes` arm, then by a peer break in `semio-s-artifact-stdio-gltf`: 210
  errors of "MutationLeaf source authority failed: source owner is not an exact registered domain operation" (a taxonomy or
  derive rule in flux). No error appeared in any file of mine. After the cut, nine orphan cargos sat deadlocked in
  `prebuild_lock_exclusive` for 3 h with 0 rustc; I killed that set per pid (mine included). RERUN PENDING.
- React `bun ./📜️script.ts test long board-event-coalescing float32-decimal` (React pkg) at 13:07 failed at import:
  `🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts` `inventory.filter` was undefined while the registry was being
  regenerated. Environment, not the tests.
- Milestone 17:20, after the reset. Each run below was a single gated foreground command with the private target dir.
  - `python3 ED/🧪️tests/🧪️select-tool-history/🐍️.py`: PASS (4 scenarios, 12 head nodes).
  - Oracle negative controls (`🐍️.py <corpus>`, copies in `🗑️generated/s2-w2d/neg-*.json`): a wrong head, a non-number
    leaf input and an unknown step key each FAIL, as they must (3 of 3).
  - `cargo test -p semio-framework-os-infinite --lib -- directed_normal`: **53 passed, 0 failed**.
  - `cargo test -p semio-framework-os-renderer-wgpu --lib -- board2d`: **7 passed, 0 failed**. This includes the new
    `the_wgpu_board_pointer_replays_the_shared_f32_decimal_corpus`, the shared coalescer corpus and the engine f32 corpus. The
    run compiles the EngineCanvas highlight sync and the scene field.
  - `cargo test -p semio-framework-plugin --lib -- time_travel`: **31 passed, 0 failed**. This includes the new
    `the_open_draft_references_its_reference_inputs_per_domain` and compiles the builder-contract literal.
  - React `bun ./📜️script.ts test long board-event-coalescing float32-decimal`: **29 passed** (2 files).
  - React `bun ./📜️script.ts test long engine-contract -t "puzzle 2d|board 2d|live mirror|hover out of the board"`: **40
    passed**. This includes the new highlight projection test; the session-1 baseline was 39.
  - React `bun ./📜️script.ts typecheck`: 4 errors, all in peers' files, none in mine. Three are TS2741 `line` missing in
    `🏪️store/👷️worker/🟦️.ts` and the backbone-parity test; one is `idleInstalledServiceStatusV1` in `🐚️Shell/🟦️.tsx`.
  - Puzzle 2d lib tests: BLOCKED by peers. `semio-s-artifact-stdio-gltf`, which puzzle 2d depends on transitively, has 16
    errors in `🧬️schema/📸️snapshot/📦️pack/🦀️.rs` and is being edited, AM at 16:45. `semio-framework-ui` (`🧊️wgpu/🧩️component`
    cannot find `wgpu::layout` / `wgpu::stepper`) also broke at 17:15. In the same build, `semio-framework-plugin`,
    `semio-framework-os-infinite`, the dag crate and the scene crate compiled with no error or warning in my edits.
  - `bun ./📜️script.ts verify taxonomy report --scope …` for the new dirs is blocked: the taxonomy is invalid again
    (`generatorContracts["graph-catalog"].inputPatterns` unsorted, `repo-entity-kinds` invocation; peers).

- Milestone 22:35, after the second reset. Puzzle 2d lib build attempts:
  - `build-p2d-5` stopped on a peer's in-flight `HistoryPatch.remote_replay` initializer, which was fixed 5 minutes later.
  - `build-p2d-6` ran when load was about 150. gltf now compiles, but the build fails with **disk full**: "No space left on
    device (os error 28)" while writing the rlibs of stdio-pdf, stdio-gltf and puzzle-3d, with 415 MiB free. Reported to
    the coordinator.
  - Waiting on disk space to rerun the build, then run the lib suite, the 5d laws and the wasm32 check.
- Milestone 23:23, after the coordinator's disk prune.
  - `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib --no-run`:
    **Finished** in 7m27s. Every new source compiled at first build: the runtime seam, the scene field, the entity label, the
    corpus laws and the wasm binding. The only new warning was one unnecessary qualification in my transactions edit, now
    fixed.
  - `… --lib -- select_tool`: **23 passed, 1 failed**.
    - Passing includes `every_corpus_edit_previews_and_replays_through_the_store` (G12, all four scenarios),
      `reference_chips_name_board_entities_like_the_outliner`, and every session-1 select-tool and select-tool-transaction law.
    - The red `every_corpus_scenario_…` stopped at step 1 ("translateSelection is one edit", 0 edits). It was a harness gap:
      the drag step painted the selection into the engine only, so the app held no selection for the following nudge.
      Fixed: the drag step selects in the app first (`interactionSelect`), then paints it into the engine as hosts do.
      RERUN PENDING (cargo hold, rule 26).
- Milestone 03:10, after the hold lifted.
  - The full lib suite ran from the 23:23 binary (`RUST_MIN_STACK=67108864`, which is what `.cargo/config.toml` sets): **1079
    tests, 1009 passed, 70 failed**, with no abort, so the Drop fix holds.
  - **All but two reds share one peer cause.** The examples' DSL no longer parses ("expected LBrace, found Ident 'id'" at
    concrete-forest 28:160 and nakagin 25:125, which are the `handles` record lists). The peer's in-flight dsl grammar and
    record refactor (`🗣️dsl/📖️grammar`, `🧬️schema/🛫️encoding|🛬️decoding`, uncommitted) has not migrated the assets. Every law
    that loads an example then fails with a poisoned `LazyLock`, or with `job-session.terminal-fault` on the example load. The
    nine predecessor reds therefore cannot be re-judged until that lands.
  - The other two:
    - the corpus harness gap, already fixed in source;
    - `board_fill_job_large_host_has_no_step_at_or_above_eight_ms`, the known 8 ms timing law under load.
  - os-kernel is red from the peer `RecordSpecProducer` refactor (coordinator 03:06), so any rebuild is peer-blocked.
- G4 `introduced` (design §16.5, the field now exists in the kernel) is added to the corpus outcomes:
  - `true` for the downstream error (C) and the warning (D) while reviewing;
  - `false` on D's warning after the overwrite, where no session holds a report.

### S2.7 Coordinator actions
- None of my changes needs a channel bump.
- The puzzle plugin needs re-describe and re-activation for the e2e: new wasm binding `setHighlightedIdsJson`, the
  Nakagin manifest registration, and the wgpu pointer mapping.
- Central `schema generate` is needed for the new scope `editor/select-tool-history`.

## Session 3 — 2026-10-02

Successor S3-W2D (coordinator `⚪b7db773a…`). Aliases as in Session 2 (ED, R, P). Status (10-03 11:20): G3 laws on both hosts
and the wgpu direct-lane fix (S3.6) VERIFIED on the core crates; the puzzle 2d lib / 5d / wasm32 runs and the 8 ms fill laws (S3.4)
WAIT for "TREE GREEN (✏️s)" — the kernel was red again at 11:19 (`🏪️store/🦀️.rs:17111,17263,22056`, store peer in flight).

### S3.1 Repair (rule 28)
- Since the last session-2 milestone (03:10) only two owned files changed, both by peers and compile-consistent: `ED/🦀️.rs`
  (`store::EngineHandles` → `semio_framework_2d::compute::EngineHandles`) and `ED/👥️presence/🦀️.rs` (dsl `spec_fn.ordinary`).
  No half-finished W2-D edit; the session-2 harness fix and the G4 `introduced` corpus edits are on disk (02:40–02:48).

### S3.2 G3 laws on both hosts (source)
- Engine law `draft_referenced_ids_paint_highlighted_without_publishing` (new region `🔗️DraftReferences` in
  `P/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`): a referenced id styles
  `Highlighted` (locked too, never dimmed), the selection keeps `Selected`, no event is published, the overlay pass carries it,
  a fixture re-parse (each draft edit repaints the preview) keeps it, `[]` clears it.
- wgpu law `a_draft_reference_highlight_reaches_the_wgpu_board_engine` (`R/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs`):
  `sync_board_engine` projects `Board2dScene.highlighted_ids_json` onto the engine exactly as React's `setHighlightedIdsJson`
  effect does; unchanged scene = no re-apply; new preview fixture keeps it; `[]` clears.
- Puzzle 2d corpus law: the `useSelection` step now clicks the rendered button — it finds the `historyEditUseSelection` binding
  inside `framework.history.editor.input/targets` of the Rust-produced history body (the body both hosts render), asserts it is
  enabled and bound to the step's path, and dispatches the binding's own args (`ED/🧪️tests/🧪️select-tool-history/🦀️.rs`,
  helper `find_binding`).

### S3.3 Verification (gated `pgrep -x rustc` < 14, one cargo at a time, private `target-nde-s3-w2d`, outputs `🗑️generated/s3-w2d/`)
- Milestone 11:26.
  - `python3 ED/🧪️tests/🧪️select-tool-history/🐍️.py`: PASS (4 scenarios, 12 head nodes agree).
  - `cargo test -p semio-framework-os-infinite --lib -- directed_normal`: **54 passed, 0 failed** (53 + the new highlight law).
  - `cargo test -p semio-framework-os-renderer-wgpu --lib -- board2d`: **8 passed, 0 failed** — shared coalescer corpus
    (`the_wgpu_coalescer_replays_the_shared_corpus`), S2.2 shortest-decimal proofs over the shared f32 corpus
    (`the_engine_offset_form_replays_the_shared_f32_decimal_corpus`, `the_wgpu_board_pointer_replays_the_shared_f32_decimal_corpus`),
    the new `a_draft_reference_highlight_reaches_the_wgpu_board_engine`, and `board2d_dump_reads_the_published_board_like_reacts_vitals`
    (`dumpBoard2d.highlighted`).
  - React (`⚛️react/📦️packages/🟦️typescript`) `bun ./📜️script.ts test long board-event-coalescing float32-decimal`: **29 passed** (2 files).
  - React `bun ./📜️script.ts test long engine-contract -t "puzzle 2d|board 2d|live mirror|hover out of the board"`: **40 passed**
    (incl. "projects the ids a time-travel draft references onto the puzzle 2d board host").
- Milestone 12:35: puzzle 2d lib build (`cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-2d --features
  component-app-assembly --lib --no-run`) is PEER-BLOCKED by an in-flight schema crate split (`🧬️schema/📶️state`,
  `🧬️schema/🧩️composition`, `🧬️schema/📇️registry`): first `semio-framework-os-kernel` (29 errors, unresolved
  `semio_framework_schema_state`/`_composition` in `📡️spr/🦀️.rs:56`, `🏪️store/🦀️.rs:3353`, edited 12:24–12:32), then
  `semio-framework-schema-registry` (28 errors, `🧬️schema/🦀️.rs:349` `ArtifactSchemaRegistry` defined twice). None in W2-D files.
  Retrying when the peer's files settle.
- Milestone 12:46: the registry duplicate was fixed by the peer (12:40). The rebuild then stops on
  `semio-framework-artifact-infinite-dag`: `♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🦀️.rs:48` E0433
  `semio_framework_schema_state` is not a dependency. The peer's uncommitted derive (`🧬️schema/✨️derive/⚙️expansion/🦀️.rs:223,262`)
  now emits `::semio_framework_schema_state::StateClass`. Reported to the coordinator (SendMessage `main`).
- Resume 18:40 (usage cut ~13:05, machine reboot ~17:00). The `find_binding` helper and its `useSelection` call site are both on
  disk and consistent (a reviewer re-keyed the lookup to the input's `….row` node at 12:49, which still holds the reference
  control and its Use-selection button — kept). Peer schema split: dag and puzzle 2d manifests now depend on
  `semio-framework-schema-state`; retrying the puzzle 2d build.
- 18:43–19:20 puzzle 2d build retries, each stopped by a different in-flight peer edit (none in W2-D files):
  `semio-s-artifact-stdio-contract` (`🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🦀️.rs:76` private `LocalizedLabel`, 18:54),
  `semio-s-artifact-stdio-json` (`…/🧾️json/…/🩹️patch-snapshot/🦀️.rs:39` no field `edits`, 19:01), os-kernel `🗣️dsl/🦀️.rs:931`
  ambiguous `canonicalize` (19:08, fixed 19:16), then os-kernel 295 errors from the dsl glob-export cleanup (`📡️spr/🧵️channel/🦀️.rs`
  `crate::Fault*`, `os_dsl::Severity` in `🏪️store`/`📡️spr`). Waiting for the dsl peer to finish.
- 19:30 coordinator: the framework is red from a Codex peer's DSL crate extraction; no polling, one cheap gated check per ~20 min,
  wait for "TREE GREEN". Source/TS work meanwhile:
  - React G3 law on the session seam: new exported `applyBoard2dHighlightedIds(session, scene)` in `R/🖥️Board2dHost/🟦️.tsx`
    (the highlight effect now calls it; re-exported in the `🔖️Board2dHost` region of `⚛️react/🟦️.tsx`), and engine-contract test
    "forwards the ids a time-travel draft references to the puzzle 2d board session as the wgpu board sync does" (ids forwarded,
    absent → `[]`, no session / a session without the binding / a throwing session are no-ops).
  - React `bun ./📜️script.ts test long engine-contract -t "puzzle 2d|board 2d|live mirror|hover out of the board"`: **41 passed**.
  - React `bun ./📜️script.ts typecheck`: 4 errors, the same 4 peer errors as session 2 (`🏪️store/👷️worker/🟦️.ts:3776,3842` and
    `🔄️sync/🧪️tests/🔬️backbone-parity/🟦️.ts:77` TS2741 `line`; `🐚️Shell/🟦️.tsx:1112` `idleInstalledServiceStatusV1`); none in W2-D files.
  - `bun ./📜️script.ts verify taxonomy report --scope …` (the three session-2 `select-tool-history` dirs) is blocked: the root
    `📜️script.ts:21073` fails to load (`Cannot use "continue" here`, peer).

### S3.4 Owed verification (runs when the coordinator signals TREE GREEN)
1. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib` (full lib,
   `RUST_MIN_STACK=67108864`), then the filters `select_tool` (machine + transactions + the corpus/G3 laws incl. the new
   Use-selection click) and the two 8 ms fill laws (`board_fill_job_large_host_has_no_step_at_or_above_eight_ms`,
   `fill_run_job_drive_step_stays_below_the_interactive_ceiling_for_nakagin`) re-run alone at the lowest load seen.
2. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib --
   a_board_gesture_drag board_node_delete apply_board_events language_neutral_fixtures`.
3. `cargo check --manifest-path ✏️s/Cargo.toml --target wasm32-wasip2 -p semio-s-plugin-puzzle`.

### S3.5 Coordinator actions (exact)
1. Puzzle plugin re-describe + re-activation (React 6012): new wasm binding `BoardSession.setHighlightedIdsJson`
   (`ED/🌉️wasm/🦀️.rs`) and the Nakagin manifest registration (`◻️2d/🛂️manifest/📇️outputs.json`, example
   `🏗️nakagin-capsule-tower/🖼️assets/🛂️manifest.json`). No channel bump.
2. wgpu shell re-activation (6112) for the renderer changes: `board_local_pointer` mapping and the board highlight sync
   (`R/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`), `dumpBoard2d.highlighted` (`R/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`).
3. Central `schema generate` for the new scope `editor/select-tool-history` (`ED/🧬️schema/🔣️select-tool-history/🔣️.json`).
4. Session-3 changes are tests plus one exported React helper: no further describe, activation or generation.
- Resume 10-03 05:47 (TREE GREEN core, coordinator 05:47). No interrupted edit: every session-3 source change is on disk
  (engine + wgpu + React highlight laws, `applyBoard2dHighlightedIds`, `find_binding` + the Use-selection click). Running S3.4.
- 05:57 build stopped on `semio-framework-plugin` `🔌️plugin/⏪️time-travel/🦀️.rs:2311` (`TimeTravelLabel::MemberEdited`, peer caller
  landed before its callee; the callee was on disk 1 min later). 06:02 build stopped on stdio crates (`zip` 52, `svg` 6, `gltf` 1
  errors: `?` cannot convert to `ValueError` in the `📸️snapshot/🪶️sqlite` modules; stdio peer editing 06:00–06:02). Not W2-D files.
- 06:26 one build was SIGKILLed (exit 137, external; 12 peer cargos running). 06:39 rebuild: blocked by stdio crates puzzle 2d
  depends on (`stdio-gltf` 13 errors incl. `🧊️gltf/…/🧬️mutations/📸️snapshot/🩹️patch/🦀️.rs:10` "MutationLeaf source authority
  failed"; `stdio-zip` `…/🪶️sqlite/🚦️native/🦀️.rs:41,45`; `stdio-svg` `🎨️svg/…/🔰️basic/🧬️schema/🦀️.rs:198,200`; stdio peer active
  06:00–06:37). Reported to the coordinator; retrying about every 20 min.
- 07:00 cheap check (`cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-stdio-gltf -p …-zip -p …-svg`): core is red again,
  `semio-framework-os-kernel` `🧬️semio/🦀️.rs:4` unresolved `crate::os_dsl::ValueRefusalKind` (dsl extraction peer). Next check ~07:20.

### S3.6 Open item analysed: session-1 §3.6 (wgpu rotate ring) — root cause, not yet fixed
- wgpu routes board moves and releases only through the retained plan (`puzzle_board_pointer_move_into` / `_up_into` →
  `BoardHost::plan_pointer`, `R/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5588,5624`). `plan_pointer`
  (`P/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:12214`) models `Interaction` only, while `pointer_move_screen`/`_up_screen`
  (`:12733`, `:12893`) also drive three lanes kept outside `Interaction`: `transform_drag` (gumball move / rotate ring → the
  `rotate` gesture record), `region_drag` (target-region body drag → `drag` record, grip resize → `regionResize`) and the area-brush
  `region_paint`. On wgpu a move during those lanes plans a hover and the release plans `Idle`, so the ring/region gesture never
  updates or commits; React calls the direct methods and works.
- Fix shape (W2-D trees): an engine predicate for "direct lane in flight" + `plan_pointer` refusing it, and a wgpu fallback to
  `pointer_move_screen`/`pointer_up_screen` that drains ALL host events (the current `board_drain_into_buffer`, `:5230`, moves one
  event per call) and flushes them through the shared coalescer; laws: a wgpu ring rotate and a region body drag each publish one
  `select` + `gesture` batch identical to the engine's own release rows. Deferred behind the owed S3.4 verification.
- 10:45 resume (usage cut ~07:15): no interrupted edit. Source work while ✏️s is red (stdio sqlite migration, coordinator):
  **S3.6 fixed in source — WRITTEN BUT UNVERIFIED (tree red)**, callee before caller:
  - Engine `P/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs`: new `BoardHost::pointer_lane_is_direct()` (area brush, region
    drag, transform drag); `plan_pointer` refuses `Move`/`Up` during a direct lane with `Unsupported` (a leave still plans);
    `transform_gumball_geometry` is public (read-only ring geometry for hosts and laws).
  - wgpu `R/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`: `puzzle_board_pointer_move_into`/`_up_into` route a direct lane through
    `puzzle_board_direct_pointer_into` (the engine's `pointer_move_screen`/`pointer_up_screen`, then every queued event into the
    buffer — bounded by `2 × BOARD_EVENT_ITEM_CAPACITY + 2` turns — coalesced by the shared corpus rules into at most ONE
    `applyBoardEvents`, the buffer retired either way via the new `board_retire_pending_events`).
  - Laws: engine `direct_pointer_lanes_refuse_the_retained_plan` (`🕹️TransformGumball` region of the board-host tests); wgpu
    `the_wgpu_rotate_ring_publishes_one_rotate_record` (`R/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs`: press on the ring
    → three frames dispatch nothing but turn the selection → the release publishes ONE `applyBoardEvents` = `[gesture rotate]`,
    the lane closed, nothing left for the frame pump).
  - Owed: `cargo test -p semio-framework-os-infinite --lib -- directed_normal` and `cargo test -p semio-framework-os-renderer-wgpu
    --lib -- board2d` (added to S3.4). Coordinator action 2 (wgpu re-activation) now also carries this routing.
- Milestone 11:09, S3.6 VERIFIED (core crates green; ✏️s still red):
  - `cargo check -p semio-framework-os-infinite --tests`: Finished (no warning in the new code; two pre-existing dead-code
    warnings at `➕️normal/🦀️.rs:3529,10864`).
  - `cargo test -p semio-framework-os-infinite --lib -- directed_normal`: **55 passed, 0 failed** (incl.
    `direct_pointer_lanes_refuse_the_retained_plan`, `draft_referenced_ids_paint_highlighted_without_publishing`, every rotate-ring law).
  - `cargo test -p semio-framework-os-renderer-wgpu --lib -- board2d`: **9 passed, 0 failed** (incl.
    `the_wgpu_rotate_ring_publishes_one_rotate_record`, the highlight sync, both shared corpora).
  - `cargo test -p semio-framework-os-renderer-wgpu --lib -- board puzzle`: 57 passed, **2 failed, neither W2-D**:
    `shell::navbar_footer_parity_tests::the_puzzle3d_app_carries_the_introduction_the_tour_arms_on` (reads the moved descriptor
    `✏️s/🔌️plugins/🧩️puzzle/🔣️.json`, now under `🌎️hub/🧩️compositions/…`; `🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs:626`,
    owner S3-W2C/shell) and `interpreter::ui_command_wiring_tests::focused_text_editor_clipboard_composition_and_accessibility_share_the_accepted_host`
    (`["textEdit","textSelect"]` vs `["textSelect"]`, `🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:1416`, text-editor owner).

## Session 4 — 2026-10-04

S4-PUZZLE inherits S3-W2D together with S3-PUZZLE; the session-4 work for both (puzzle 2d editor, board engine, wgpu board, React Board2dHost,
select-tool laws) is reported in `📓️w3-t-puzzle-report.md` § "Session 4 — 2026-10-04".
