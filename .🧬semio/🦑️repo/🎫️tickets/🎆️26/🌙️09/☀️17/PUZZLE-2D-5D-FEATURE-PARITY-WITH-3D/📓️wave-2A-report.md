# 📓️ Wave 2A — the four red lanes of the ◻️2d browser battery (product fixes)

Slice 2A. Scope: the PRODUCT defects behind `8-transform/engagement-move`,
`20-history/undo-changes-document`, `16-inspection/inspector-fresh-on-open` and the 198 red 2d unit
tests. Probe-side fixes belong to 2G; the probe was not touched.

Paths are relative to `/Users/ueli/Documents/semio`.

---

## 1. Engagement text normalizer — the typed line now reaches every guest VERBATIM

**Root cause (confirmed exactly as E7 §1 described it):** the one Action line every app renders through
(`Search`, `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`) ran `normalizeEngagementActionText`
over the typed draft, the published value and every routed keystroke, so `move 50 25` reached
`engagementSubmit` as `Move5025` and every `verb <args>` grammar in the repo was a silent no-op.

**A second half of the same defect, not in E7:** even with the normalizer gone a user could not TYPE a
space — the field's own `keydown` swallowed Space unconditionally and submitted the line (CAD's
step-value grammar). Delivering the text verbatim is meaningless if the separator can never be entered.

### What landed (all in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`)

| line | change |
|---|---|
| 10221 | `normalizeEngagementActionText` docstring: it normalizes action NAMES for display/matching, never the typed line |
| 10524 | `routeWindowSearchSpace` reads the raw value |
| 10537 | `routeWindowSearchKeydown` appends the keystroke verbatim |
| 10499-10506 | NEW `searchSpaceConfirmsLine(draft, sessionActive)` — Space confirms an EMPTY line (repeat-last / step submit) or any line during a live engagement session; otherwise it is a separator that belongs in the text |
| 10509-10526 | `applySearchSpaceAction` submits `draft.trim()` |
| 10754-10755 | `publishedLine`/`draft` are the program's value verbatim |
| 10775-10790 | `applyDraft` dispatches the typed value verbatim to `onChange` |
| 10899-10912 | the field's Space branch only `preventDefault`s when it really consumes the key; Enter submits `draft.trim()` |

`normalizeEngagementActionText`/`engagementActionTokenEquals` stay — they are still the right thing for
possible LABELS and token comparison, and they are exported for apps that own a name-token grammar.

### Consumers audited (every engagement parser in the repo)

- **cad** (`✏️s/🔌️plugins/📐️cad/…/⚙️engine/📺️renderer/🟦️.tsx:4074`) — owns its own line: its
  `onInputChange` calls `replNormalizeActionText(value, engagementActionMode)` and PascalCases there,
  and `onInputSubmit` ignores the submitted value for its own `cmdLine`. Unaffected by this change, and
  its Space grammar is preserved (idle → a matching possible is activated; session → `sessionActive`
  keeps Space confirming). No shim was added anywhere for it.
- **framework guest helpers** `strip_engagement_prefix` / `engagement_token_matches`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:39360+`) were ALREADY case- and
  separator-insensitive on the verb token; only their module docstring claimed the shell PascalCases —
  corrected.
- **puzzle 2d/3d/5d, writer, layout, draw, note, animate, lowpoly** all lower-case + `split_whitespace`
  or use those helpers → all correct on verbatim text. Two were *broken by* the normalizer and are
  fixed by removing it: `🪐️space` `workflow-engagement-submit` needs `<plugin> <app>` (two
  whitespace-separated operands) and animate's `"copy prompt"` arm could never match `CopyPrompt`.
- **writer test** `📤️engagement-submit/🧪️tests/🔬️unit/🦀️.rs` renamed to
  `engagement_submit_parses_separatorless_drafts` with an honest comment (it still proves both spellings
  parse).
- No wgpu or tui twin of the normalizer exists (`🎯️targets/🧊️wgpu`, `🎯️targets/⌨️tui` searched).

### Laws

- React (`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`):
  `Search onChange carries the typed line verbatim, spaces and arguments included`,
  `Search keeps an argument line verbatim through change, Enter, and a session Space`,
  `Search Space with a draft submits the step during a session and types a separator when idle`,
  `searchSpaceConfirmsLine confirms an empty line or a live session and types a separator otherwise`.
  Four pre-existing laws that asserted the PascalCase squash were rewritten to the new contract (they
  were the specification of the defect).
- Guest (NEW `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/🎮️commands/📨️engagement-submit/🧪️tests/🔬️unit/🦀️.rs`,
  mounted from the command): `engagement_line_carries_its_arguments_verbatim` drives
  `move 50 25` / `rotate 45` / `scale 1.5` / `fill 12` through the real dispatch, and
  `engagement_verbs_are_case_insensitive_and_unknown_lines_are_no_ops`.

---

## 2. Fill = one history edit

**E7 §2's mechanism is WRONG and the report should not be trusted on this point.** `commit-slot`
(`…/🎮️commands/✅️commit-slot/🦀️.rs`) is the BRUSH per-click commit; the fill run never drives it. A
2d fill run is a framework tool run: the run job only publishes provisional ops through the tick writer
and the tool-run ledger publishes **one** `Edit` at finalize
(`🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs:47,60` — `TOOL_RUN_GROUP_ID_PREFIX`, "the single `Edit` this
run publishes at finalize"), which the pre-existing law
`fill_run_start_complete_finalize_is_one_undo_entry` already pinned at 8 placements.

### What landed

- NEW law `a_hundred_placement_fill_is_one_history_entry_that_undoes_and_redoes`
  (`…/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`): 100 requested placements on concrete-forest →
  exactly one history entry → one undo restores the pre-fill document → one redo re-applies it. This is
  the battery's own scenario at the battery's own count.
- **Real coalescing gap found and closed:** 2d's dispatch emitted `coalesce_key: None` for EVERY verb
  (`…/✏️editor/🦀️.rs`, the old comment claimed "no action coalesces anymore"), so a gumball/keyboard
  transform gesture spent one of the 64 edit-ledger slots PER DRAG TICK where 3d spends one per
  gesture. NEW `puzzle2d_gesture_coalesce_key` (`…/✏️editor/🦀️.rs:2470-2481`) mirrors 3d's
  `🦀️.rs:3557-3562` for `translateSelection`/`rotateSelection`/`scaleSelection`; law
  `transform_gesture_ticks_coalesce_into_one_undo_step` (3 ticks → one undo).
- **Board drags and brush strokes were already one edit per gesture** — the host buffers a whole drag
  (`coalesceBoard2dEvents` + `dispatchBufferedEvents` on pointerup,
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx:111-141,469-486,826-836`);
  `nodeMove` is not a flush-now event, so the `nodeMove` stream + `nodeDragEnd` arrive as ONE
  `applyBoardEvents` dispatch. A brush placement is a flush-now event because one click IS one
  placement gesture. No change needed; verified, not assumed.

### The 64-slot ceiling — what the framework expects

`ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:186`) is a
HARD ceiling with **no runtime compaction anywhere in the framework**: a slot is only freed by
`remove_key` (close/retirement paths), and `commitCheckpoint`/`reconcile_alternative`
(`…/🏪️store/🦀️.rs:10795-10813`) push change/checkpoint rows without releasing a single edit slot. Past
the ceiling `reserve_edit_history_slot` (`…/🏪️store/🦀️.rs:15871-15883`) refuses with
`edit history ledger is saturated`. **3d honours this the same way 2d now does — by spending one slot
per gesture — and neither app has any other lever.** A session of 65+ *distinct* user edits is a
framework limit, not a 2d defect; NEW law
`sequential_small_edits_honour_the_fixed_edit_ledger_ceiling` pins where the wall stands (all 64 slots
admitted, a refusal past it names the saturated ledger, the store stays usable). See §5 for the verdict
and the hand-off.

---

## 3. Blank panes — the recurrence is `parse_fixture_json` clearing before it validates

**E7 §4's proposed cause does not exist.** There is no second production `sync_descriptor` call site:
the announcing `sync_descriptor` is reachable only through `syncDescriptorJson`
(`…/◻️2d/…/🌉️wasm/🦀️.rs:190-197`), and NOTHING in the shipped JS calls it — grep for
`syncDescriptorJson` across `🧰️framework`/`✏️s` returns only the session type declaration and a vitest
stub. The `announce_new_edges=false` fix of 09-17 02:55 therefore already covers every live path.

**The actual mechanism** (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs`):
`parse_fixture_json` set the port mode, moved the camera and **`clear_scene()`d first**, then built the
descriptor with twenty `return false` exits and finally called `sync_descriptor_with`, which can itself
refuse (entity/byte ceilings) or fail halfway (`InvalidHandleColor`). So *every* refusal — a malformed
row, a descriptor past its cap, an unparseable handle colour — emptied all three panes and left them
empty until the next successful parse. "The engine refused the fixture" and "the board went blank" were
the same event, which is exactly the transient `Nodes 0 / Edges 0` the inspector honestly reported.

### What landed

- `descriptor_admission(desc)` (NEW): the two fixed descriptor ceilings **plus** every handle colour,
  checked before anything is mutated. `sync_descriptor_with` now preflights through it, so an authoring
  sync is all-or-nothing too instead of half-committing nodes/handles before an invalid colour.
- `parse_fixture_json` split into a pure `fixture_scene_descriptor(f, has_ports) -> Option<…>` and a
  commit that only runs once the descriptor is built AND admitted. A refused fixture leaves the live
  board exactly as it was.

### Laws (`…/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`)

- `a_whole_editing_session_reparses_the_board_after_every_gesture` — parse → drag (real pointer plan +
  retained commit + publication close) → re-parse the board a 100-placement fill grew → delete → parse
  again, twice: every parse admitted, zero refusals, in ONE host.
- `a_refused_fixture_parse_leaves_the_painted_board_untouched` — three refusal shapes, each leaving the
  previous board intact and committing none of its own rows.

---

## 4. The 2d unit suite

_(filled in below from the single run)_

---

## 5. Commands run

_(filled in below)_

---

## 6. Not verified / hand-offs

_(filled in below)_
