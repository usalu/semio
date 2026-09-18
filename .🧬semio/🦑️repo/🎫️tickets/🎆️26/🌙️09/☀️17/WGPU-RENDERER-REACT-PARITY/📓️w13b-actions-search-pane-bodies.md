# 🎬️ W13b — the per-window Actions pane body and Search pane body in the wgpu shell

Packet W13b of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. Input: `📓️w12d` §2's remaining gap —
*"wgpu paints **no Actions/Search pane body at all**… `render_engagement_input` no longer exists…
React's window-scoped search palette has no wgpu twin to census yet"*. All paths absolute under
`/Users/ueli/Documents/semio`. The shell file below is
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`.

---

## 0. Two premises in the packet brief are wrong, and the evidence says so

**(a) "the probe's click on the first row journals `addObjectKind`".** It does not — and neither does
React's. `🗑️generated/w12c-parity-run-19/steps.json` rows `pane-chip-engagement-toggle` /
`pane-chip-search-toggle` show React resolving the chip by suffix, journalling `addObjectKind`, and
moving **no control and no surface** (`controlCount: [335, 335]`, `controlsAdded: []`). A press that
opened the Actions pane would add ~96 ids. It is the press W12d §1 already diagnosed: the journey
addresses `puzzle3dMainTop`, whose top-left/top-middle chips lie under the open **Catalogue panel**,
so the click lands on the catalogue's `Hexagonal Cut Concrete Forest Right` row, which dispatches the
guest's `addObjectKind`. `addObjectKind` is a catalogue row, not an Actions row.

The live React capture confirms it from the other side: unfolding the pane by its own id adds 96 ids
and **`addObjectKind` is not among them** (`🗑️generated/w13b-react-dom.json`) — puzzle3d declares it
outside the window kind's panel-eligible set. The honest expectation for W13c's next run is therefore
`pane-chip-{engagement,search}-toggle` journalling `addObjectKind` on BOTH renderers *because the
catalogue panel is open*, with the pane bodies measurable only from a run taken with that panel
closed (or by addressing the right pane, `puzzle3dMainPerspective`).

**(b) "ranked rows via W1c's `rank_fuzzy_items`".** React's window search does not call
`rankFuzzyItems` at all — that is the ⌘️K palette's ranker. The window line ranks with
`searchPossibleRankScore`/`filterSearchPossibles` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10269`/`:10297`), a
different, higher-is-better scorer over `normalizeEngagementActionText`. Borrowing W1c's ranker would
have made the two shells order the same suggestion list differently, so this packet ports React's own.
The choice is stated in the port's docstring so nobody re-derives it.

---

## 1. React's structure, as read and as measured

Captured live against `http://127.0.0.1:6313/?plugin=puzzle3d` (read-only) with
`🐍️w13b-actions-search-dom-dump.mjs` → `🗑️generated/w13b-react-dom.json`, before that serve went down
(see §6). Sources: `🪟️Window/🟦️.tsx:379-412`, `🛠️ShellHelpers/🟦️.tsx:4060-4260`,
`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10758-10990`.

### 1.1 The two panes are ONE fold

`Window` mounts a top-left `Pane` (`engagement`) and a top-middle `Pane` (`search`), both driven by
`setEngagementBarFolded` over the single `actionsFolded` state (`:373`/`:388`), which is exactly the
fold `WindowPaneChip::{Actions,Search}` already share on this renderer (W12d §2).

### 1.2 The Actions pane body

```
[data-slot="window-engagement-body"]
  └ [data-slot="engagement"]            ← <Engagement/>: status rows, quick-action options
  └ [data-slot="window-action-pane"]    ← <WindowActionPane/>
      └ [data-slot="tree"] role=tree
          └ #action.category.<categoryId>   role=button   (collapsible header, defaultOpen)
          └ #action.<actionId>              role=treeitem (one per panel-eligible action)
```

* Rows are `resolveWindowActions(app, kind).filter(inPalette)` in declaration order, bucketed by
  `actionCategoryId` = `category ?? (kind === "history" ? "history" : "actions")`, categories in
  first-declaration order.
* A zero-arg row fires `onExecute({controllerId, action})` directly; an arg-carrying row wears a
  trailing `…` and only toggles `SET_ACTION_PANE_EXPANDED`.
* The expanded action adds a sibling section `action.category.<categoryId>.form`, labelled with the
  action's own label, carrying one `action.<actionId>.arg.<argId>` row per argument and the two
  section actions `childElementId("framework.window", windowId, "action", actionId, "execute"|"reset")`
  — Execute disabled while `unresolvedActionArgs` is non-empty.
* An armed utility with `allowsActionsWhileActive === false` disables every row EXCEPT
  `FRAMEWORK_RESERVED_ACTION_IDS` (`🛠️ShellHelpers/🟦️.tsx:308`, 17 ids).
* Measured census for puzzle3d's top pane: **96 ids added**, of which 7 are `action.category.*`, 82
  are `action.*`, 7 are React-generated `semio-collapsible-*` DOM scaffolding (the class w9c §3
  established no canvas renderer publishes), plus the search line.

### 1.3 The Search pane body

```
[data-slot="window-search-body"]   rect [118.25, 63.94, 300, 44.78]
  └ [data-slot="search"]
      └ [data-slot="search-row"]
          └ [data-slot="search-input"] → <input id="puzzle3d-engagement">
          └ (Action id="ui.windowSearch.suggestions")   ← only when possibles exist
      └ (Popover → Command rows, only while expanded)
```

* The line's id is the **engagement input's own** (`puzzle3d-engagement`), falling back to
  `ui.windowSearch.action` for an unnamed or internal-chrome id (`:10868`).
* Suggestions are filtered by `filterSearchPossibles` and shown only while the chevron is expanded.
* `Search` returns `null` without an input, so a window with no engagement input has no search body.
* Both panes are `PANE_DEFAULT_SIZE = 300` logical px wide (`:9912`); measured 300 for both.

---

## 2. What landed in the wgpu shell

### 2.1 The assembler grew a `Tree` arm (the enabling change)

`panel_ui_records`' `PanelProjection` refused `UiNode::Tree`, which is precisely what React's Actions
pane IS. A `Section` container registers **no** hit (`retained_hit_registration`, `📥️input/🦀️.rs:718`),
so a category header authored as one would have no twin for React's `action.category.<id>` control —
only a `Tree`'s section/item pair gives both rows a control on this renderer.

| symbol | line | what |
| --- | --- | --- |
| `PanelProjection::node`'s `Tree` arm | shell file `4208` | `UiNode::Tree` → `Component::Tree`, children are its sections |
| `PanelProjection::tree_section` | `4232` | → `Component::TreeSection` (label, `default_open`), items as children |
| `PanelProjection::tree_item` | `4247` | → `Component::TreeItem` (label, icon, description), the row's click as a `Trigger::Activate` binding, nested rows as children |

No `TreeWindow` is written: a shell-owned body materialises every row it has.

### 2.2 The new region `//#region 🎬️WindowActionsAndSearchPanes` (shell file `14214`–`14700`)

| symbol | line | what |
| --- | --- | --- |
| `window_actions_surface_id` / `window_search_surface_id` | `14219` / `14229` | `"{windowId}/framework.section.engagements"` and `…engagements.search` — the Measures overlay's own keying rule |
| `WINDOW_PANE_BODY_WIDTH_PX` | `14235` | React's `PANE_DEFAULT_SIZE` (300) |
| `WINDOW_SEARCH_ACTION_CONTROL_ID` / `…SUGGESTIONS…` | `14239` / `14243` | React's two fixed search ids |
| `FRAMEWORK_RESERVED_ACTION_IDS` | `14248` | the 17 ids React's own set carries, verbatim |
| `action_category_id` / `action_category_label` | `14270` / `14277` | React's `actionCategoryId` / `actionCategoryLabel` |
| `action_requires_staged_form` | `14283` | React's own — an action whose every argument is hidden fires bare |
| `search_possible_rank_score` | `14291` | React's window-line scorer (NOT `rank_fuzzy_items`; the docstring says why) |
| `normalize_engagement_action_text` | `14317` | React's normalizer, decimal-inside-a-number preserved |
| `filter_search_possibles` | `14348` | React's `filterSearchPossibles` |
| `staged_action_arg_row` | `14361` | React's `renderStagedArgControl` with React's `action.<id>.arg.<argId>` id |
| `ShellState::window_kind_of` | `14396` | the KIND a live instance renders — `pane-top`/`pane-perspective` are one kind |
| `ShellState::build_window_actions_ui` | `14413` | the Actions body: engagement status/options, the category `Tree`, the expanded action's staged form |
| `ShellState::build_window_search_ui` | `14526` | the Search body: the typed line, the chevron, the ranked rows |
| `ShellState::refresh_window_action_panes` | `14588` | republishes both per live pane through `publish_surface_records` |
| `ShellState::window_actions_rect` / `window_search_rect` | `14626` / `14633` | React's `top-left` / `top-middle` anchors, 300 px, under the pane's own chip row |
| `ShellState::paint_window_pane_body_step` | `14648` | one paint opportunity of either body, gated on the shared `actionsFolded` |

Wiring:

* `ShellState` gained `window_actions_documents`, `window_search_documents`, `search_possibles_open`
  (`2994`-`3000`).
* `refresh_ui` publishes both bodies **unscoped**, beside the shell-owned panel leaves (`5744`): the
  expanded action, its staged buffer and the chevron are shell state the user moves with no guest
  round trip, so a dirty-scope gate would freeze a pressed row on its previous frame. The ingress is
  revision-keyed, so an unchanged body still pays nothing.
* Two new window-walk phases, `12` (Actions) and `13` (Search), after the projection pane (`18614`),
  so the bodies outrank the chips they grow under.
* Six new `"framework"` dispatch arms (`7891`): `setActionExpanded`, `stageActionArg`,
  `resetActionArgs`, `executeStagedAction`, `setEngagementInput`, `toggleSearchPossibles`. Only
  `executeStagedAction` reaches the app, through the existing `execute_staged_action` funnel.
* Four `ui.windowSearch.*` EN/DE rows added to `shell_chrome_string`, transcribed from React's own
  bundles (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:2971`/`:3811`).

### 2.3 The dead wgpu-private `shell.action.*` grammar is gone

`shell.action.{expand,reset,argtoggle,argselect,exec}::<window>::<action>` were the deleted rail's own
id vocabulary, with no minting site left anywhere — five `handle_shell_hit` arms, a pre-flush guard,
`commit_staged_input`, `staged_input_seed`, `arg_default`, `fmt_num` and an empty `// #region
ActionsRail` block. All removed; the pane now routes through React's ids and the framework verbs
above. `stage_arg` / `staged_map_for` / `reset_staged_args` / `execute_staged_action` /
`resolved_execute_args` / `effective_arg_value` are kept and are the new bodies' readers.

### 2.4 The id shape, stated plainly

`panel_ui_records` qualifies every shell-owned key with its surface, so the published control id is
`"{windowId}/framework.section.engagements/action.setActiveExample"`, whose **tail is React's id
exactly**. That qualification is not cosmetic: `retained_hit_windows` is a process-wide map keyed by
control id, and React's own pane ids are duplicated across window instances (the same invalid-HTML
defect `📓️w12d` §3 records for the utility rail), so two open panes would otherwise fight over one
owner entry. The parity probe resolves by suffix (`resolved: "suffix"` is what run-19 records for
every pane chip), so this meets React on the probe's own terms.

A `Tree` row additionally wears this renderer's fixed row prefix (`section.chevron.<key>` /
`tree.label.<key>`, minted by `retained_hit_registration`, the same convention the catalogue panel
already publishes and `📓️w9c` blessed), which suffix-resolution also carries.

---

## 3. Tests

New fixture `🐚️Shell/🧫️fixtures/🎬️window-actions-search-panes/🔣️.json` (the id templates, the reserved
action set, the search labels and a five-case ranking oracle) and new suite
`🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs`, wired as
`mod window_actions_search_pane_tests`, **8 laws**:

| law | pins |
| --- | --- |
| `the_actions_pane_publishes_reacts_own_category_and_row_census` | one `action.category.<id>` header per category in first-declaration order, its own `action.<id>` rows after it, and a non-palette action in neither renderer |
| `the_first_row_of_the_actions_pane_dispatches_the_apps_own_verb` | the first row carries the APP's controller and verb, bare args (P4) — the first-row dispatch law |
| `an_arg_carrying_row_opens_reacts_staged_form_and_refuses_execute_until_it_resolves` | the `…` suffix, `setActionExpanded` instead of a fire, the form's five React ids, the expanded action keeping exactly its one list row (§4.1), Execute's `missing.length > 0` gate |
| `an_armed_gating_utility_disables_every_app_row_but_not_the_frameworks_own` | the 17-id reserved set equals React's, and only those rows stay pressable |
| `the_search_pane_publishes_reacts_input_and_suggestion_ids` | no input → no body; the engagement's own id on the line; `ui.windowSearch.suggestions`; a collapsed chevron publishes no row; the `ui.windowSearch.action` fallback |
| `the_search_possibles_rank_the_way_reacts_own_scorer_does` | the fixture's five ranking cases and the decimal-preserving normalizer |
| `the_pane_bodies_move_no_shell_surface` | `chrome_surface_census` unchanged by publication (React's `+s` column is empty for these steps), one document per live pane, retirement on close |
| `both_pane_bodies_read_the_one_actions_fold` | either chip opens and closes both, and never the sibling pane's |

---

## 4. Verification — RUN, green

The peer's `semio-s-artifact-stdio-pdf` landed and the coordinator unblocked the gates; everything
below was then run in the foreground, `CARGO_INCREMENTAL=0 … -j 4 --test-threads=1`.

| command | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | **0 errors** | `🗑️generated/w2i-w13b-native-check.txt` (coordinator's run) |
| `cargo test … --lib -- --test-threads=1 window_actions_search` | **8 passed / 0 failed** | `🗑️generated/w13b-tests.txt` |
| `cargo check … --lib --target wasm32-unknown-unknown -j 4` | **0 errors**, exit 0 | `🗑️generated/w13b-wasm-check.txt` |
| `cargo test … --lib -- window_pane_chrome panel_anchor_model window_measures command_registry` | **96 passed / 0 failed** | `🗑️generated/w13b-tests-neighbours.txt` |
| `cargo test … --lib -- shell_input shell_shortcuts_palette shell_chrome_parity navbar_footer_parity chrome_overlays` | **117 passed / 0 failed** | `🗑️generated/w13b-tests-shell.txt` |
| `cargo test … --lib -- shell::` | **414 passed / 0 failed** | `🗑️generated/w13b-tests-shell-all.txt` |
| `cargo test … --lib` (whole crate) | 928 passed, **2 failed — neither this packet's** | `🗑️generated/w13b-tests-full.txt` |

The neighbour runs matter for two reasons beyond regression: `window_pane_chrome` is W12d's 15 chip
laws (this packet changed nothing under them), and `panel_anchor_model` carries
`the_panel_assembler_refuses_an_unprojectable_node` — still green with `Tree` moved out of the refused
set, so the assembler still fails loudly on a variant no builder authors.

The two whole-crate failures are `agent_bridge::tests::every_{gateway_to_shell,modelled_shell_to_gateway}_fixture_round_trips_through_this_codec`,
both `AgentMessage did not decode: UnknownTag(9)` — a peer's live AgentBridge lane (`🔗️AgentBridge/🟦️.tsx`
and `💬️AgentChatPanel/*` are dirty in the working tree, and their fixture has grown a message tag the
Rust codec does not decode yet). This packet touches neither file.

### 4.1 The one law that failed, and its root cause

`an_arg_carrying_row_opens_reacts_staged_form_and_refuses_execute_until_it_resolves` panicked on
*"the expanded action keeps its list row"* — an assertion that asserted the **opposite** of the law its
own message states. The implementation was right and the test was wrong:
`buildActionCategoryTree` maps EVERY `categoryActions` entry and pushes the form section *beside* the
list (`🛠️ShellHelpers/🟦️.tsx:4097`-`:4145`), because the list row IS the accordion trigger that folds
the form again. Only `buildCommandCategoryTree` — the bottom-middle Command dock, a different surface —
filters its expanded command out, and that is the rule this law was first written with by mistake
(`build_command_category_ui` implements it correctly for the dock, and the wrong comment travelled).

The law now pins React's real rule — exactly one list row for the expanded action, beside its form —
and carries the citation so it cannot drift back.

## 5. Files

| file | change |
| --- | --- |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | assembler `Tree` arm + two helpers · new region `🎬️WindowActionsAndSearchPanes` · 3 state fields · `refresh_ui` publication · walk phases 12/13 · 6 `"framework"` dispatch arms · 4 i18n rows · the dead `shell.action.*` grammar removed |
| `🐚️Shell/🧫️fixtures/🎬️window-actions-search-panes/🔣️.json` | new shared fixture |
| `🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs` | new suite, 8 laws |
| `🐍️w13b-actions-search-dom-dump.mjs`, `🐍️w13b-actions-form-dump.mjs` | the two read-only React DOM probes |

---

## 6. Gaps this packet deliberately leaves open

1. **The React serve on 6313 died mid-packet.** It answered `200` for the first capture
   (`🗑️generated/w13b-react-dom.json`, the whole §1 evidence) and `000` / `ERR_CONNECTION_REFUSED`
   for the second, so the staged FORM's ids are read off `buildActionCategoryTree`'s source rather
   than from a live capture. Whoever recycles 6313 should re-run `🐍️w13b-actions-form-dump.mjs`.
2. **The Actions/Search chips still mount unconditionally.** React mounts the Actions `Pane` only for
   `engagement || actionPane` and the Search `Pane` only for a `search` spec, and the shared fixture
   `🪟️window-pane-chrome/🔣️.json` says `"mounted": "always"` for both. Now that the bodies exist the
   chip could follow its body, but that is the CHIP lane's fixture (W12d's), not this one's — flagged,
   not changed.
3. **The engagement's `control`/`controls` are not painted.** React's `<Engagement/>` also renders a
   slider/stepper/ring/toggle-group/select row; this body renders the status lines and the quick-action
   options. Those five controls are their own vocabulary and belong with the engagement lane.
4. **The search line has one binding, not two.** React's input carries both `onChange` (per keystroke,
   which feeds the guest's autocomplete) and `onSubmit` (Enter). The retained engine produces only
   `Trigger::Change` or `Trigger::Commit` per input node — there is no `Trigger::Submit`/`Abort`/
   `RepeatLast` producer anywhere in `🖱️ui` — so the line binds the engagement's `on_submit` on commit
   and falls back to `on_change`. `on_abort`/`on_repeat_last` have no twin at all. Framework gap.
5. **Category headers of the staged FORM section.** The form is a `Section` sibling of the `Tree`, not
   a `Tree` section, so its header registers no hit where React's `action.category.<id>.form` header
   is a collapsible button. Every control INSIDE it carries React's id.
6. **Nothing was seen in a browser.** No `activate-*`, no wasm build, no trunk — W13c owns the
   puzzle3d rebuild. Live confirmation is W13c's next parity run; expect the Actions/Search census to
   appear only on a run whose Catalogue panel is closed (§0).
