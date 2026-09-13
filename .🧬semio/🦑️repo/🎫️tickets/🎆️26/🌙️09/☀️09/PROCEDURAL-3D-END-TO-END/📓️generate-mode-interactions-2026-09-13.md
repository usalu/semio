# Generate-Mode Interactions — Items #3/#5/#7/#8/#10 (2026-09-13)

Lane: `generate-mode-interactions`. Scope: `📓️audit-user-journey-gaps-2026-09-13.md` gaps #3 (wire
editing), #5 (generate-mode gumball), #7 (generation row verbs), #8 (form value edit), #10
(`reorganize` / `moveMediaNode`), plus the wgpu handoff from
`📓️wgpu-retained-controls-wires-2026-09-13.md` §7.4(a).

## TL;DR

Five of the five items landed with laws; four of five are browser-proven on 6018 against a wasm
restaged from this lane's own source (17:13). Three of the five gaps turned out to be **real defects,
not missing proofs**:

1. **`selectGeneration` was a no-op for everything a user can see.** All three generate-mode window
   bodies resolved "the current generation" off the ARTIFACT snapshot, which only
   `CreateGeneration`/`DeleteGeneration` replay ever writes — while the evaluation path already
   resolved it off the CONFIG. Clicking a generation row moved the evaluation and moved nothing on
   screen. The config is now the single authority.
2. **A `commit: "blur"` input could not be typed into at all.** `InputView` rendered a CONTROLLED
   input with no `onChange` (React's read-only spelling), so every keystroke was reverted and the
   blur committed the unchanged value. Measured live: typing `Balcony Study` dispatched
   `renameGeneration` and settled it with the history label `new-name="Generation 2"` — the old name.
   This was fleet-wide, not generation3d-specific.
3. **`nodeGraphEdit` silently dropped `move` and `disconnect`.** Both fell through the catch-all arm,
   so a fallback node drag and every wire cut spent a whole retained command and changed nothing.

Two honest negatives: the flow canvas's **port-to-port wire drag does not rewire at runtime** (gap #3's
renderer half — measured, not merely unproven), and the **wgpu `addGeneration` divergence is not the
row's arguments** (lawed natively; the defect is downstream of the command).

**Laws: 28 Rust, 28 green** (22 + 2 + 4 across three filtered runs,
`🗑️generated/generate-interactions/laws-run-{4,5}.txt`), plus **1 TypeScript twin** over a shared
language-agnostic fixture.

---

## 1. Generations window — `selectGeneration` / `renameGeneration` / `removeGeneration` (gap #7)

### Root cause

`✏️editor/🦀️.rs:249-251` handed `&document.generation` to all three generate-mode renders, and
`generation_tree` / `form::render` / `generate_preview::render` each resolved the current generation
through `playbook::selected_generation(state)` — i.e. off `GenerationPlayState.selected_generation_id`
on the ARTIFACT. That field is written only by `Generation3dMutation::CreateGeneration` /
`DeleteGeneration` replay (`🧬️schema/🧬️mutations/💾️binary/🦀️.rs:636,643-654`). `selectGeneration`
emits **no artifact mutation at all** — only `Generation3dConfigMutation::SetSelectedGeneration`
(`🎮️commands/🧬️generation/🦀️.rs:37`). Meanwhile `flow_eval_tick::evaluate`
(`🎮️commands/⏱️flow-eval-tick/🦀️.rs:49`) and `generation_command_result`
(`🎮️commands/🧬️generation/🦀️.rs:21`) both already overrode the state's selection with the config's.
So the evaluation followed the config and the three window bodies followed the artifact: two
authorities, and the one a user clicks had no visible effect.

Rename had a second defect: the row action dispatched a HARDCODED `"{name} copy"`
(`🫀️core/🖼️semantic-ui/🦀️.rs`, old line 84) from a `RowActionPlacement::Menu` affordance — right-click
only. A user could reach the verb and never choose a name.

### Fix (owning layer)

| File | Change |
|---|---|
| `🧬️schema/🦀️.rs:313` | new `generation_by_id(generation, selected_id)` — the ONE lookup; `generation_fixture_for` now takes `selected_id` explicitly |
| `✏️editor/🦀️.rs:245` | `let selected_generation_id = config.selected_generation_id.as_deref();` threaded into all three generate renders (`:249-251`) |
| `🫀️core/🖼️semantic-ui/🦀️.rs:64-73` | new `generation_rename_field` — a text `input` with `commit: "blur"` bound `Trigger::Commit` → `renameGeneration{id}`, rendered on the SELECTED row only |
| `🫀️core/🖼️semantic-ui/🦀️.rs:118` | delete row action → `RowActionPlacement::Row` (painted on the row, not folded into its context menu); the fixed-argument rename row action is gone |
| `✏️editor/🦀️.rs:1897` | `renameGeneration` now reads `str_arg(&["name", "value"])` — a scalar `Trigger::Commit` payload is NAMED `value` by `uiIntentPayload` (`🛠️ShellHelpers/🟦️.tsx:2022-2039`), never `name` |
| `🌀️generation2d/…/✏️editor/🦀️.rs` + its generations window | same `selected_id` threading, so the twins stay identical |

### Framework defect found and fixed

`🗣️Interpreter/🟦️.tsx` `InputView`: `commit === "blur"` rendered `<Input value={component.value}>`
with `onChange={undefined}`. That is a read-only controlled input — React reverts every keystroke, and
the blur handler then commits the value the field already had. New `useCommitDraft`
(`🗣️Interpreter/🟦️.tsx:1091-1105`) keeps a local draft, follows the published value when the GUEST
changes it, and commits on blur **and on Enter** (`commitOnEnter`, added for the keyboard-only path).
Applies to `Input` and `Textarea` alike.

### Laws (9, all green)

`🎭️modes/🧬️generate/🪟️windows/🗂️generations/🧪️tests/🔬️unit/🦀️.rs`
- `every_generation_row_verb_is_reachable_from_the_rendered_tree` — all four verbs emitted; no
  `"placement":"menu"` survives; exactly the selected row carries the rename editor
- `select_rename_and_remove_converge_on_the_roster_the_user_asked_for` — select #2, rename it, remove
  #1; asserts the roster AND reads the selection back the way a user sees it (off the rendered tree)
- `an_inline_rename_commit_carries_its_typed_text_as_value`
- `generation_row_affordances_are_localized_in_german` — en+de, no English fallback
- `add_generation_lands_a_row_under_every_renderer_argument_shape` (see §6)
- plus the pre-existing `generate_mode_renders_surfaces`

### Browser proof — 6018, `🗑️generated/generate-interactions/probe-4`

`🐍️generate-mode-probe.mjs` extended from "click Add Generation" to the full roster journey:

```
add-1=true add-2=true add=true select=true rename-typed=true rename=true
remove-click=true remove=true form=true gumball-rail=true gumball-lane=true
```

- **add** — two rows, each carrying its own `Remove` button in the row itself
- **select** — clicking row 2 moved the inline rename editor from `generation-1.rename` to
  `generation-2.rename`, i.e. the config selection now drives what the windows render
- **rename** — typed `Balcony Study`, Enter; the roster row reads `Balcony Study`
- **remove** — the row's own Remove button left a one-row roster (`Balcony Study`)

## 2. Form window — `updateGenerationValues` (gap #8)

Source was already wired; the gap was proof, plus the selection authority above (with the artifact
selection the form could show a different generation than the one being evaluated).

Laws (`🎭️modes/🧬️generate/🪟️windows/📝️form/🧪️tests/🔬️unit/🦀️.rs`, 3 green):
`a_selected_generation_turns_the_form_into_bound_controls` (the hint retires, every control dispatches
`updateGenerationValues`, each carries its generation id) and
`editing_a_form_value_repatches_the_generate_preview_fixture` (the edited number reaches the PATCHED
fixture `flow_eval_tick` evaluates — the mechanism by which the mesh changes).

Browser (`probe-4`, step `form`): dragging `generate.form.height.slider` moved the generate preview's
mesh payload from `meshes=3, 3641 chars` to `meshes=2, 593 chars`. Note for the next run: the probe
asserts the payload CHANGED and snapshots it at `phase:"invalid"` (re-eval in flight) rather than
waiting for the next `idle` — worth tightening.

## 3. Generate preview gumball (gap #5)

The generate preview is the same World3d surface as the edit preview, over the same `graph` domain and
the same `preview_selection_json` (whose `transformMode`/`gumballActive` pair is exactly what
`World3dHost` gates on, `🌐️World3dHost/🟦️.tsx:3043`) — it was simply never given the transform utility
rail, so `ViewModel.active_utility_id` never became a transform mode there.

- `✏️editor/🦀️.rs:2302` — `window_kind_utilities(generate_preview, ["move","rotate","scale"])`
- `✏️editor/🦀️.rs:2331` — the gumball trio added to its `window_kind_action_refs`

Laws (`…/👁️preview/🧪️tests/🔬️unit/🦀️.rs`, 2 green): `both_previews_offer_the_same_gumball_surface`
(parity of actions AND rail) and
`the_generate_preview_selection_payload_arms_the_gumball_under_a_transform_utility`.

Browser (`probe-4`): the generate preview's Utilities menu lists `["Move","Rotate","Scale"]`, and
clicking a generated instance publishes
`{"selectedIds":["extrusion-axis@vector#0"],"gumballActive":true}` on `data-guest-selection-json`.

## 4. Flow wire editing (gap #3) — guest fixed, renderer gesture still dead

### Guest half — fixed and lawed

`🎮️commands/✏️node-graph-edit/🦀️.rs` handled `setFixture`, `deleteSelection`, `connect` and dropped
everything else. `disconnect` (a wire cut) and `move` (the `Diagram` fallback's `onNodeDragStop`
payload) fell through `_ => {}`: silent no-ops that still spent a retained command. Both are handled
now (`:56-68`). Law `disconnect_then_connect_round_trips_one_wire` cuts a live synapse and redraws it
between the same two ports; `move_relocates_the_widget_a_node_drag_names` pins the coordinates.

### Renderer half — measured NOT working

`🐍️flow-window-probe.mjs` gained a `wire` mode. To aim at a canvas that paints itself, this lane added
`window.__semioFlowGraphProbe[surfaceId]` (`🕸️NodeGraph/🟦️.tsx:2186-2210`, modelled on ShellHost's
`__semioOsCatalogProbe`) which reuses `dagIntroductionResolver` verbatim — one entity→screen
implementation, not a probe-only second one.

Result (`🗑️generated/generate-interactions/wire-{1,2,3}`): the handles resolve
(`height@number` → `(541, 492)`, rect 18×13; `extrusion-axis@z` → `(643, 492)`), but **12 grab points
across both ports' rects, zoomed in and out, produced no fixture change** — the synapse list is
identical before and after, and the console shows only `interactionHover`, never a `nodeGraphEdit`.

The hit path exists in source: `DagHost::pointer_down_screen`
(`🕸️dag/🦀️.rs:5197-5202`) routes `world_hits_handle` → `engine.pointer_down_screen`, and
`rim_handle_anchor_hit` (`:4750`) tightens the port tolerance to `handle.radius + 1.5` **world** units
under the row-pick LODs. That gate is the first place to bisect. **Not claimed as proven.**

## 5. `reorganize` and `moveMediaNode` (gap #10)

**`reorganize` — given a keyboard trigger.** It had exactly one reachable trigger, the flow canvas's
right-click menu, so a keyboard-only user could not reach it. `✏️editor/🦀️.rs:2382`
`.keybinding("mod+alt+l", "reorganize")` (arg-free, so `ShellHost`'s keybinding loop fires it straight
through `onAction`). Laws: `reorganize_is_reachable_by_menu_and_by_keyboard_in_both_languages`
(en+de label, exact chord) and `reorganize_is_the_context_menus_top_level_layout_verb`.
Browser (`🗑️generated/generate-interactions/reorganize-1`): `⌘⌥L` on the flow window moved **all 7
widgets** to a fresh layout (e.g. `extrude` `(34.8, -154.2)` → `(60, -40)`).

**`moveMediaNode` — deleted.** No renderer dispatches it anywhere: the flow canvas commits node moves
through `setFixture` and the `Diagram` fallback through `nodeGraphEdit`'s `move`. It was a second
spelling of a verb with no caller — a dead command by the audit's own definition. Removed from the
command enum, the retained tool-id table, the publication contracts, the bounded proof list, the
action bridge, the action declaration, the interactive-job list and the module mount (9 sites +
`🗿️artifacts/🧊️generation3d/🦀️.rs:721`), its command directory deleted, and the two tests that used it
rewritten onto `nodeGraphEdit`'s `move`. Roster counts updated (24→23 retained, 30→29 proofs). It
survives untouched in `space`/`dag`/`flow`, where it does have callers.

## 6. wgpu handoff — `addGeneration` settles but creates nothing (§7.4(a))

**It is not the row's arguments.** Both renderers build the identical descriptor for that row:
React's `uiIntentPayload` drops an empty payload entirely, and wgpu's `record_action`
(`🧊️wgpu/🔀️reconcile/🦀️.rs:278-284`) carries `binding.args` through `ui_value_to_dsl`, which for the
authored `factory.action("addGeneration", None)` is `None` on both sides.
`command_from_action("addGeneration", …)` (`✏️editor/🦀️.rs:1662`) ignores args entirely, and
`generation_operations` returns the Add unconditionally.

Lawed at the owning layer so it cannot regress:
`add_generation_lands_a_row_under_every_renderer_argument_shape` dispatches through the public
`PluginApp::handle_action` bridge under three envelopes — `None`, `{}`, and a host envelope
`{surfaceId, windowId}` — settles each, and asserts one generation lands per dispatch. **Green.**

So the divergence is downstream of the command. The remaining candidates, in order:
`addGeneration`'s publication contract declares three lanes
(`Artifact | Config | Transient`, `✏️editor/🦀️.rs:733`) — a host that settles the job `terminal=true`
without acknowledging the **Artifact** lane gets exactly the reported symptom (settled, roster empty).
Next step: on 6118 read the settle receipt's per-lane acknowledgement, not just `terminal`.
**Not probed on 6118 by this lane** — that needs a wgpu restage this lane could not take without
violating the one-restage rule.

## 7. Peer breakages fixed forward (noted, not silently absorbed)

1. `✏️editor/🎮️commands/📤️export-document/` and `📥️import-document/` mounted `mod tests;` before those
   files existed, which broke the whole crate's `--tests` target for every concurrent lane. Created as
   placeholders with a header saying so; **the io-surface lane should overwrite them**.
2. `cancelPreviewEval` was added to both preview windows' `window_kind_action_refs` and given a
   `mod+.` chord while still declared ONLY as a `CommandDefinition`, so `build_definition`
   rejected the app (`app-definition.invalid: … references undeclared action cancelPreviewEval`) and
   **every** app-fixture test in the crate died at construction for ~90 minutes. Fixed by declaring it
   as the view action it is (`✏️editor/🦀️.rs:2231`), with a comment naming the owning lane.

## 8. Method / not claimed

- Laws run: `🗑️generated/generate-interactions/laws-run-4.txt` (22 passed, filter `generate`) and
  `laws-run-5.txt` (2 + 4 passed, filters `node_graph_edit`, `reorganize`). A full unfiltered `--lib`
  run does **not** complete: it hangs in the pre-existing
  `flow_eval_tick::tests::an_uncontributed_graph_arms_no_tick_while_a_served_one_keeps_its_chain`
  (`laws-run-3.txt`, 20 tests in then stalled) — untouched by this lane, worth its own look.
- TS twin: `✏️editor/🧪️tests/🔬️generate-interactions/🟦️.ts`, run with `bun`, green
  (`windows=5 rowVerbs=4 previews=2 graphOps=5 commands=7 removed=1`). It re-implements
  `uiIntentPayload`'s merge and `World3dHost`'s gumball gate from the rule rather than importing them,
  over the shared fixture `🧫️fixtures/🎛️generate-mode-interactions.json`.
- Browser runs are against the wasm restaged at 17:13 from this lane's own source
  (`🗑️generated/generate-interactions/restage-1.txt`, 10m50s).
- **Not claimed**: port-to-port wire drag on the canvas (§4, measured not working); anything on 6118
  (§6); the form step's post-settle mesh (§2).

## 9. Files

Changed: `🫀️core/🖼️semantic-ui/🦀️.rs`; `🧊️generation3d/🦀️.rs`; `🧊️generation3d/…/🧬️schema/🦀️.rs`;
`…/✏️editor/🦀️.rs`; `…/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs`; `…/✏️editor/🎭️modes/🧬️generate/🪟️windows/{🗂️generations,📝️form,👁️preview}/🦀️.rs`;
`…/✏️editor/🎮️commands/{🧬️generation,⏱️flow-eval-tick}/🦀️.rs`; `🌀️generation2d/…/✏️editor/🦀️.rs` and its
generations window; `🧰️framework/…/🗣️Interpreter/🟦️.tsx`; `🧰️framework/…/🕸️NodeGraph/🟦️.tsx`;
`T/🐍️generate-mode-probe.mjs`; `T/🐍️flow-window-probe.mjs`.
Added: `🧫️fixtures/🎛️generate-mode-interactions.json`; `…/✏️editor/🧪️tests/🔬️generate-interactions/{🦀️.rs,🟦️.ts}`;
`…/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs`; laws appended to the three generate-window
test files and to `🎮️commands/🗺️reorganize/🧪️tests/🔬️unit/🦀️.rs`.
Removed: `…/✏️editor/🎮️commands/🚚️move-media-node/`.
