# 📓️ Wave 2A — the four red lanes of the ◻️2d browser battery (product fixes)

Slice 2A. Scope: the PRODUCT defects behind `8-transform/engagement-move`,
`20-history/undo-changes-document`, `16-inspection/inspector-fresh-on-open` and the red 2d unit-test
classes. Probe-side fixes belong to 2G; the probe was not touched. Paths are relative to
`/Users/ueli/Documents/semio`.

**Headline:** two of E7's four root causes are WRONG (§2, §3 below) — both were re-derived from source
and from a green law. The blank-pane family turned out to be two real defects nobody had named, and
both are fixed and proven (board_host **35/35 green**). Every 2d-crate law is currently blocked by a
registry fault that is not this slice's (§5).

---

## 1. Engagement text normalizer — the typed line now reaches every guest VERBATIM

**Root cause (E7 §1 confirmed):** the one Action line every app renders through (`Search`,
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`) ran `normalizeEngagementActionText` over the
typed draft, the published value and every routed keystroke, so `move 50 25` reached
`engagementSubmit` as `Move5025` and every `verb <args>` grammar in the repo was a silent no-op.

**A second half of the same defect, not in E7:** even with the normalizer gone a user could not TYPE a
space — the field's `keydown` swallowed Space unconditionally and submitted the line (CAD's step-value
grammar). Delivering text verbatim is meaningless if the separator can never be entered.

### Landed — `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`

| line | change |
|---|---|
| 10221 | `normalizeEngagementActionText` docstring: it normalizes action NAMES for display/matching, never the typed line |
| 10524 | `routeWindowSearchSpace` reads the raw value |
| 10537 | `routeWindowSearchKeydown` appends the keystroke verbatim |
| 10499-10506 | NEW `searchSpaceConfirmsLine(draft, sessionActive)` — Space confirms an EMPTY line (repeat-last / step submit) or any line during a live engagement session; otherwise it is a separator that belongs in the text |
| 10509-10526 | `applySearchSpaceAction` submits `draft.trim()` |
| 10764-10765 | `publishedLine`/`draft` are the program's value verbatim |
| 10785-10800 | `applyDraft` dispatches the typed value verbatim to `onChange` |
| 10899-10917 | the field's Space branch only `preventDefault`s when it really consumes the key; Enter submits `draft.trim()` |

`normalizeEngagementActionText`/`engagementActionTokenEquals` stay: they are still right for possible
LABELS and token comparison, and exported for apps that own a name-token grammar.

### Consumers audited (every engagement parser in the repo)

- **cad** (`✏️s/🔌️plugins/📐️cad/…/⚙️engine/📺️renderer/🟦️.tsx:4074`) owns its own line: `onInputChange`
  calls `replNormalizeActionText(value, engagementActionMode)` and PascalCases THERE; `onInputSubmit`
  ignores the submitted value and uses its own `cmdLine`. Unaffected, and its Space grammar survives
  (idle → the matching possible is activated by `shouldActivateSearchPossibleOnConfirm`; session →
  `sessionActive` keeps Space confirming). **No shim was added for it anywhere.**
- **framework guest helpers** `strip_engagement_prefix` / `engagement_token_matches`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:39360+`) were ALREADY case- and
  separator-insensitive on the verb token; only their module docstring claimed the shell PascalCases —
  corrected in place.
- **puzzle 2d/3d/5d, writer, layout, draw, note, animate, lowpoly** all lower-case + `split_whitespace`
  or use those helpers → all correct on verbatim text. Two were *broken by* the normalizer and are
  fixed by removing it: `🪐️space` `workflow-engagement-submit` needs `<plugin> <app>` (two
  whitespace-separated operands) and animate's `"copy prompt"` arm could never match `CopyPrompt`.
- **writer test** `…/📤️engagement-submit/🧪️tests/🔬️unit/🦀️.rs` renamed to
  `engagement_submit_parses_separatorless_drafts` with an honest comment (it still proves both
  spellings parse).
- No wgpu or tui twin of the normalizer exists (`🎯️targets/🧊️wgpu`, `🎯️targets/⌨️tui` searched).

### Laws

- React (`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`):
  `Search onChange carries the typed line verbatim, spaces and arguments included`,
  `Search keeps an argument line verbatim through change, Enter, and a session Space`,
  `Search Space with a draft submits the step during a session and types a separator when idle`,
  `searchSpaceConfirmsLine confirms an empty line or a live session and types a separator otherwise`.
  Four pre-existing laws that asserted the PascalCase squash were rewritten to the new contract — they
  were the specification of the defect.
- Guest (NEW `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/🎮️commands/📨️engagement-submit/🧪️tests/🔬️unit/🦀️.rs`,
  mounted from the command): `engagement_line_carries_its_arguments_verbatim` drives
  `move 50 25` / `rotate 45` / `scale 1.5` / `fill 12` through the real dispatch, plus
  `engagement_verbs_are_case_insensitive_and_unknown_lines_are_no_ops`.

---

## 2. Fill = one history edit — E7 §2's mechanism is WRONG

`commit-slot` (`…/🎮️commands/✅️commit-slot/🦀️.rs`) is the BRUSH per-click commit; the fill run never
drives it. A 2d fill run is a framework tool run: the run job publishes only provisional ops through
the tick writer and the tool-run ledger publishes **one** `Edit` at finalize
(`🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs:47,60` — `TOOL_RUN_GROUP_ID_PREFIX`, "the single `Edit` this
run publishes at finalize"), which the pre-existing law `fill_run_start_complete_finalize_is_one_undo_entry`
already pinned at 8 placements. **Fill was never the thing that spent 100 ledger slots.**

### Landed

- NEW law `a_hundred_placement_fill_is_one_history_entry_that_undoes_and_redoes`
  (`…/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs:614`): 100 requested placements on concrete-forest →
  exactly one history entry → one undo restores the pre-fill document → one redo re-applies it.
- **The real coalescing gap, found and closed:** 2d's dispatch emitted `coalesce_key: None` for EVERY
  verb (the old comment claimed "no action coalesces anymore"), so a gumball/keyboard transform gesture
  spent one of the 64 edit-ledger slots PER DRAG TICK where 3d spends one per gesture. NEW
  `puzzle2d_gesture_coalesce_key` (`…/✏️editor/🦀️.rs:2644-2655`, used at `:2784`) mirrors 3d's
  `🦀️.rs:3557-3562` for `translateSelection`/`rotateSelection`/`scaleSelection`. Law
  `transform_gesture_ticks_coalesce_into_one_undo_step` (3 ticks accumulate, ONE undo restores).
  Verified this does not change dispatch semantics: neither 2d nor 3d implements
  `ArtifactOwnedToolJobFactory::latest_wins_target`, so a coalesce key only selects
  `ArtifactCommand::AmendLast`; no invocation is dropped.
- **Board drags and brush strokes were already one edit per gesture** — verified, not assumed: the host
  buffers a whole drag and flushes on pointerup
  (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx:111-141,469-486,826-836`),
  and `nodeMove` is deliberately not a flush-now event, so the `nodeMove` stream + `nodeDragEnd` arrive
  as ONE `applyBoardEvents` dispatch. `brushPlace` IS flush-now because one click is one placement
  gesture. No change needed.

### The 64-slot ceiling — what the framework expects

`ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:186`) is a
HARD ceiling with **no runtime compaction anywhere in the framework**: a slot is freed only by
`remove_key` (close/retirement paths), and `commitCheckpoint`/`reconcile_alternative`
(`…/🏪️store/🦀️.rs:10795-10813`) push change/checkpoint rows without releasing a single edit slot. Past
the ceiling `reserve_edit_history_slot` (`…/🏪️store/🦀️.rs:15871-15883`) refuses with
`edit history ledger is saturated`. **3d honours this exactly the way 2d now does — one slot per
gesture — and neither app has any other lever.** A session of 65+ *distinct* user edits is a framework
limit, not a 2d defect. NEW law `sequential_small_edits_honour_the_fixed_edit_ledger_ceiling`
(`…/🧪️tests/🔬️unit/🦀️.rs:367`) pins where the wall stands: all 64 slots admitted, a refusal past it
must name the saturated ledger, and the store stays usable (undo still works). **Owed to integration:**
this law has never run (§5) — if the framework is to survive 70 sequential edits it needs an eviction
or compaction step in `🌿️vcs`, which is a framework change outside this slice.

---

## 3. Blank panes — E7 §4's cause does not exist; two REAL causes found and fixed

**E7 §4 is wrong.** There is no second production `sync_descriptor` call site: the announcing
`sync_descriptor` is reachable only through `syncDescriptorJson` (`…/◻️2d/…/🌉️wasm/🦀️.rs:190-197`), and
NOTHING in the shipped JS calls it — `grep -r syncDescriptorJson` over `🧰️framework`/`✏️s` returns only
the session-type declaration (`🪪️WasmSessionLoader/🟦️.tsx:284`) and a vitest stub. The
`announce_new_edges=false` fix of 09-17 02:55 already covers every live path.

All work below is in
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed{,/➕️normal}/🦀️.rs`.

### 3a. `parse_fixture_json` cleared the board BEFORE it validated

It set the port mode, moved the camera and **`clear_scene()`d first**, then built the descriptor through
twenty `return false` exits, then called `sync_descriptor_with`, which can refuse (entity/byte ceilings)
or fail HALFWAY (`InvalidHandleColor`, after nodes and handles were already inserted). So *every*
refusal emptied all three panes and left them empty until the next successful parse: "the engine refused
the fixture" and "the board went blank" were the same event — exactly the transient `Nodes 0 / Edges 0`
the inspector honestly reported in `16-inspection/inspector-fresh-on-open`.

- NEW `descriptor_admission(desc)` (`➕️normal/🦀️.rs:9613`): the two fixed descriptor ceilings **plus**
  every handle colour, checked before anything mutates. `sync_descriptor_with` preflights through it,
  so the authoring sync is all-or-nothing too instead of half-committing.
- `parse_fixture_json` (`:9879`) split into a pure `fixture_scene_descriptor(f, has_ports) -> Option<…>`
  (`:9912`) and a commit that runs only once the descriptor is built AND admitted.

### 3b. The board engine REQUIRED a camera the document no longer carries

`FixtureJson.camera` was a required field. Since wave T made `setCamera` a View-kind verb, 2d documents
are session-camera documents and carry **no `camera` key at all** (its own law
`set_camera_is_session_only_and_never_undoable` asserts "no `camera` key"). So `serde_json::from_str::<FixtureJson>`
failed before reading a single node and `parse_fixture_json` returned `false` for *every* such document —
`data-board-fixture-parsed=false`, three blank panes, with nothing in the log to bisect.

- `camera` is now `Option<CameraJson>` (`➡️directed/🦀️.rs:236-240`, with its `ToValue`/`FromValue` twins
  at `:244`/`:262` updated), and the parse adopts a named camera or keeps the one the session is looking
  through (`➕️normal/🦀️.rs`).

### Laws — `…/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`

- `a_whole_editing_session_reparses_the_board_after_every_gesture` — parse → drag (real pointer plan,
  retained commit, publication close) → re-parse the board a 100-placement fill grew → delete (driven to
  its terminal step, asserting no fault) → parse again, twice: every parse admitted, zero refusals, ONE
  host.
- `a_refused_fixture_parse_leaves_the_painted_board_untouched` — three refusal shapes, each leaving the
  previous board intact and committing none of its own rows.
- `a_fixture_without_a_camera_parses_and_keeps_the_session_camera`.

### The two pre-existing red laws — both root-caused and fixed

Both were brittle ladder-length assertions, not defects in the port:

- `drag_commit_obeys_zero_budget_one_delta_turn_cancel_and_publication_witness` pinned the commit at
  exactly three live turns; the drag commit gained a phase (the `Interaction::DragNodes.proximity_pair`
  the proximity slice added), so the third turn returned `Pending`. Now: zero budget is a no-op, the
  FIRST live turn applies the delta, and the tail is driven to `Complete` under a bounded ceiling with
  every intermediate asserted `Pending`.
- `delete_plan_retains_fifo_until_exact_credits_and_rejects_stale_or_oversized` asserted
  `close_turns > 4` on a cancelled delete's close ladder, which now finishes in 2 cooperative turns
  (and the counter did not even count the loop's own first call). Now: the first turn is still proven
  not to close, the close is cooperative and bounded at 32, counted correctly.

---

## 4. The 2d red-test classes

- **`artifact store reached Drop without its exact terminal-empty witness` (harness apps never closed)
  — FIXED.** Every 2d test that builds an app now closes it: 19 in `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
  (including `ingest_operations_is_idempotent`, whose non-unwinding Drop panic aborted the whole test
  process and is why the suite has to be run with `--skip`) and
  `renders_puzzle2d_board_scene` in `…/🪟️windows/👁️overview/🧪️tests/🔬️unit/🦀️.rs`. A repo-wide scan of
  the 2d tree now reports zero test functions that construct an app without `close_app`.
- **The 2d lib-test target did not COMPILE at all** (6 errors, `🗑️generated/2A/test-2d-lib-1.txt`), so
  the whole 724-test suite was unrunnable for every slice. Five were sibling call sites left behind by
  sibling production changes; they are mechanical downstream updates (no semantics changed, nothing
  reverted) and were applied to unblock the crate — each is named here so its owner can check it:
  - `📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:241-242` — `UiMapCursor` yields owned `(UiText, UiValue)`:
    `key == "value"` → `key.as_str() == "value"`, `Some(*value)` → `Some(value)`.
  - `⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs:436,447` — `Puzzle2dFillRevalidateJob::new` gained a fifth
    `placement_slack: f64` argument; the two revalidate laws pass `0.0`.
  - `🪟️window/🧪️tests/🔬️unit/🦀️.rs:202` — `Puzzle2dWindowConfig` gained nine persisted pane options
    (`grid_visible`, `proximity_radius`, `area_brush_{width,height}`, `transform_{move,rotate}`,
    `selectable_{nodes,handles,edges}`); the round-trip fixture now sets all of them, each different
    from the default as its own comment demands.
  The sixth was mine (a `close_app(&mut app)` inserted into a test whose bindings are `sender`/`receiver`).
- **Mutation-fixture canonical-JSON drift and `edit history insertion requires its exact mutation
  retirement factory`: NOT re-observed.** Both classes come from the 09-17 01:30 run; today's blocker
  (below) fires before any store is built, so neither class could be reached. Owed to integration.

---

## 5. What is red now, and who owns it

1. **Crate-wide registry blocker — NOT this slice.** Every 2d law that builds a registry-backed app
   dies in `app_with_registry()` with
   `tool factory proof rejected tool 'setActiveTool': … generated_migrated=false, typed_join=false`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21876`). `setActiveTool`/`setActiveUtility`
   are declared by `Puzzle2dHostConfigurationProofs`
   (`…/✏️editor/🦀️.rs:4748-4759`, added by a sibling) but are absent from the app's generated migrated
   declarations (`generated_ids`, 54 entries — it carries every sibling verb including `createEdge`,
   `proximityConnect` and the target-region family, but not the two host-configuration verbs). Evidence
   that it is not mine: the PRE-EXISTING laws `add_node_action_emits_upsert_op_and_appends_node` and
   `fill_run_start_complete_finalize_is_one_undo_entry` fail with the identical fault
   (`🗑️generated/2A/test-2d-control.txt`), while `tool_registry_declares_fill_tool`, which needs no app,
   passes. **Whoever added that proofs block owes the generated-declaration side (descriptor regen or
   the `.action_interactive_job` row).** Until it lands, all five of this slice's 2d laws are
   unverifiable.
2. **`shipped_examples_parse_in_the_board_engine` is red on a DATA regression, not on the engine.**
   `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/📄️document.json`
   — the 2d example's own document — currently holds **3d** data:
   `{"schema":"puzzle.3d.fixture","domain","meta","objects","attractions","targetVolumes","references"}`,
   with no `nodes`/`edges` at all. The board port refuses it at the schema gate, which is exactly the
   reported `first refused node = None; first refused edge = None`. This is a real blank-pane cause
   wherever that raw document reaches the board, and converting a 3d example into a 2d one is an
   examples job, not a board-port job — left untouched deliberately, routed to the examples owner.

---

## 6. Commands run (all `CARGO_INCREMENTAL=0`, foreground, logs in `TICKET/🗑️generated/2A/`)

| command | verdict |
|---|---|
| `cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly` | **GREEN** — `Finished dev profile … in 21m 04s`, 0 errors; 68 workspace warnings, 7 of them in `semio-s-artifact-puzzle-2d`, 5 in `semio-framework-os-infinite` (all pre-existing dead-code/qualification suggestions) → `check-native-1.txt` |
| `cargo test -p semio-framework-os-infinite --lib -- board_host` (×4) | run 1 **3 failed** (2 pre-existing + my new session law under-budgeted its delete drive) → run 3 **34/34 GREEN** → run 4 after the camera fix **35/35 GREEN**, 0 errors → `test-board-host-{1,3,4}.txt` |
| `cargo test -p semio-s-artifact-puzzle-2d … --lib -- --skip ingest_operations_is_idempotent` | **did not compile**, 6 errors (§4) → `test-2d-lib-1.txt` |
| `cargo test -p semio-s-artifact-puzzle-2d … --lib -- <2A laws>` | **compiles now** (867–870 tests enumerated), all 5 laws blocked by the §5.1 registry fault → `test-2d-2A-laws{,-2}.txt` |
| `cargo test -p semio-s-artifact-puzzle-2d … --lib -- <pre-existing controls>` | proves the blocker is crate-wide, not mine → `test-2d-control.txt` |
| `cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly --target wasm32-wasip2` | **GREEN** — `Finished dev profile … in 30.39s`, 0 errors, 76 warning lines (all pre-existing) → `check-wasm-1.txt`. This is the run that covers the wasm-gated `🌉️wasm` bridge, whose `syncDescriptorJson`/`parseFixtureJson` entry points sit on the two functions this slice rewrote. |

Not run by design (coordinator's 19:55 addendum): the full 724-test 2d suite, and the React laws —
the `🖱️ui` module has no standalone TS package, so its vitest suite only runs through the forbidden
workspace `nx test` target. Both edited `.tsx` files were parse-checked with esbuild (`parse OK`).

## 7. Registries touched

None. This slice added no verb, so no `puzzle2d_command_variants!`, `PUZZLE2D_RETAINED_TOOL_IDS`,
`bounded_first_step_tool_proofs!`, `build_tool_job`, `PUBLICATION_CONTRACTS`, `command_from_action`,
terminology or fixture row changed. `Emit.coalesce_key` is an existing field of an existing emit.

## 8. Hand-offs

- **Integration:** land §5.1 (the `setActiveTool` generated declaration) and re-run this slice's five 2d
  laws — they are written and compile, they have simply never been allowed to execute.
- **Examples owner / 2F:** §5.2, the 2d concrete-forest document is 3d data.
- **2E (board engine / Board2dHost):** the drag commit's ladder length is no longer pinned; if the
  rotate handle adds a phase, `drag_commit_obeys_zero_budget…` will not go red for it.
- **2G (probe):** `8-transform/engagement-move` should now pass unchanged — `input.fill("move 50 25")`
  drives the real controlled `onChange` and the line reaches the guest verbatim. The blank-pane
  detector (`__semioBoard2dRefusedFixture`) will now only fire for documents that are genuinely
  unpaintable, because a refusal no longer empties the board.
- **Framework/store owner:** the 64-edit ledger has no compaction (§2). If a session of 65+ distinct
  edits must survive, that lever has to be built in `🌿️vcs`; no app can work around it.
