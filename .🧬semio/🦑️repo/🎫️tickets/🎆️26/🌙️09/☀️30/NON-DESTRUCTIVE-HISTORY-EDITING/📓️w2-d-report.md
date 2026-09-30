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
