# Wave B15 — the pane's opening camera, the Settings panel's window scope, and the §2/§19 probe steps

Implementation pass, 2026-09-11. Picks up `📓️2026-09-11-wave-B12-windowconfig-roundtrip.md` §4.1 ("no
pane's camera is guest-owned until the user moves it"), §4.2 (the stale §19 selectors) and §5.1 ("a
Settings edit lands on `puzzle3d-main`, which is not a pane anyone is looking at").

Every command tail quoted below is real output from this pass.

---

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD. No `git commit`/`stash`/`checkout`, no worktree,
  no `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`, everything foreground. The ticket was not opened/closed here.
- The repo MCP server did not connect (`repo (-32602): invalid initialize params`), so the ticket
  folder is managed on disk.
- `[DEBUG] ` logs: **none added**; none removed (the traces quoted are the shell's/guest's existing
  ones).
- Peers were live in the same files throughout (B13 export/history/locale/engagement, B14
  `🔌️PluginRuntime`, a peer in `🔌️plugin/🦀️.rs`). No peer hunk was reverted.

---

## 1 Every pane opens on its own framed camera — GUEST, rides #47

### Root cause

`Puzzle3dCamera::default()` is all zeros, and `World3dHost` frames each pane **locally** — `WorldAutoFit`
computes the fit from the live scene `Box3` and `🌐️World3dHost/🟦️.tsx:4752-4760`'s own docstring says it
deliberately never dispatches that framing back. So the pose the guest published on each pane's world
lane was `position == target == [0,0,0]` until the first `setCamera` landed: no view direction at boot,
and `window-distinct-camera` comparing two identical non-poses rather than two framings.

### Fix

`🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` gains four functions in the `SceneJson` region:

- `camera_unset(&Puzzle3dCamera)` — `position == target`, the one reading that can never be a real pose.
- `framing_bounds(&Puzzle3dFixture)` — centre + largest span over object, reference and target-volume
  origins (a fixture with no objects still has something to frame); empty ⇒ `([0,0,0], 1.0)`.
- `framing_projection(window_id)` — the Top instance opens orthographic/plan, every other instance keeps
  the three-point default, mirroring the `TEMPLATE_TOP`/`TEMPLATE_PERSPECTIVE` display templates the
  mode's own layout declares.
- `framed_camera(window_id, fixture)` — `distance = max(span * 1.35, 1.4)` (the guest twin of the host's
  `span * 2.5` seed narrowed by `world3dFrameCameraFromInstances`'s `padding / 2.5`), posed through the
  framework's own `world3d_projection_pose`, i.e. the same primitive `setProjection` already snaps with.
- `frame_unset_camera(&mut Puzzle3dScene, window_id)` — idempotent; frames only a pane no gesture and no
  stored `WindowConfig` has posed.

Applied at the two places the fixture and the instance id meet on a READ path:
`Puzzle3dPlayApp::scene_for` (window measures / engagements / tool measures / context menu) and
`render_body`'s envelope (the world body, i.e. `cameraJson` → `data-camera-json`).

### What it is deliberately NOT

The first draft also framed inside `Puzzle3dActionPrologue::scene_step`/`refreshed`, so that the first
action on an unframed pane would PUBLISH the pose onto that pane's `WindowConfig` lane. That is wrong and
the suite said so immediately — 52 laws red with

```
retained operation faulted: typed-operation emitted a store lane absent from its exact factory publication contract
```

because the per-verb `ArtifactToolPublicationContract` declares `WindowConfig` for `setCamera` and the
rail/Settings verbs only; `setSelectionFlag` and ~50 others may not touch that lane at all. The opening
pose is therefore **derived, not published**: it is what the pane renders with until a real `setCamera`
writes one, and `setCamera` remains the single writer of the lane. That also keeps the coordinator's
"`setCamera` adds no artifact history entry" untouched — nothing new is emitted anywhere.

### Laws (3 new, `✏️editor/🧪️tests/🔬️unit/🦀️.rs`)

- `every_pane_opens_on_its_own_framed_camera_before_any_gesture` — fresh session: each pane publishes a
  three-axis non-zero position, `position != target`, the two panes' poses DIFFER, Top is
  `orthographic`/`top` and Perspective is `threePoint`.
- `switching_the_example_reframes_every_unposed_pane_on_the_new_document` — `setActiveExample nakagin`
  moves BOTH panes' published poses and keeps them distinct.
- `an_opening_camera_is_stable_and_never_overrules_a_pose_the_user_set` — an unrelated window option does
  not move it; a real `setCamera` owns the pane from then on and the framing never takes it back.

Red first, actually run: with `frame_unset_camera`'s body replaced by `let _ = (window_id, &scene.fixture);`

```
running 3 tests
test editor::puzzle3d::component::tests::every_pane_opens_on_its_own_framed_camera_before_any_gesture ... FAILED
test editor::puzzle3d::component::tests::switching_the_example_reframes_every_unposed_pane_on_the_new_document ... FAILED
test editor::puzzle3d::component::tests::the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind ... ok
…
test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 699 filtered out; finished in 0.69s
```

then green with the body restored:

```
running 3 tests
test editor::puzzle3d::component::tests::an_opening_camera_is_stable_and_never_overrules_a_pose_the_user_set ... ok
test editor::puzzle3d::component::tests::every_pane_opens_on_its_own_framed_camera_before_any_gesture ... ok
test editor::puzzle3d::component::tests::switching_the_example_reframes_every_unposed_pane_on_the_new_document ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 696 filtered out; finished in 1.12s
```

The Top pane's opening pose, read off the law's own failure output before the projection assertion was
corrected to the wire's `mode`/`orientation` spelling:

```
{"position":[3.5,0.0,9.455000000000002],"projection":{"mode":{"kind":"orthographic"},"orientation":{"type":"cardinal","view":"top"}},"target":[3.5,0.0,0.005],"up":[0.0,1.0,0.0],"zoom":1.0}
```

---

## 2 The Settings panel addresses the last-focused pane — FRAMEWORK host-live, GUEST hop rides #47

### Root cause (B12 §5.1, confirmed)

An app-level panel body is rendered through `ViewModel::for_panel()` (guest,
`🔌️plugin/🦀️.rs:32203`) and `panelViewContext` (host, `🛂️manifest/🟦️.ts`), both of which clear
`window_id` **by design**. `puzzle3d_addressed_window_id` therefore fell straight through to the live
roster's first entry — the base window KIND `puzzle3d-main`, which is not one of the two open panes —
and the guest bakes that id into every stepper's args, so ShellHost's own
`actionWindowId ?? activeWindowIdRef.current` fallback never got to pick the focused pane either.

### Fix — one new host-owned field, schema-first

`focusedWindowId` / `focused_window_id`: *the window instance the user is LOOKING at*, distinct from
`windowId` (*the window a call is rendered FOR*) and deliberately surviving the panel projection.

| surface | file | change |
|---|---|---|
| neutral schema | `🛂️manifest/🪟️view-context/🧬️schema/🔣️.json` | new `focusedWindowId` `Identifier` property |
| TS type + admission | `🛂️manifest/🟦️.ts` | `PluginViewState.focusedWindowId`; added to `parseResolvedPluginViewState`'s `short`/`allowed`; `panelViewContext` docstring states the survival rule |
| Rust mirror | `🛂️manifest/🦀️.rs` | `ViewModel::focused_window_id`; `VIEW_CONTEXT_IDENTIFIER_FIELDS` 5 → 6 |
| host stamp | `🏛️ShellHost/🟦️.tsx` | stamped from `activeWindowIdRef.current` onto the `refreshUi` view state, the action dispatch base view state and the context-menu base view state |
| host refresh | `🏛️ShellHost/🟦️.tsx` | new effect: a focus change re-fetches the PANEL bodies only (`{kind:"partial", panelBodies}`) — hash-conditional, so a panel that ignores the focused pane costs one compare |
| wgpu shell | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | the five full `ViewModel` literals + `live_view_state` carry `self.active_window_id` |
| guest | `✏️editor/🦀️.rs` | `puzzle3d_addressed_window_id` resolves `keyed → window_id → focused_window_id → fallback → roster` |

`panelViewContext` / `for_panel` needed no code change to preserve it — both spread the source view and
clear an explicit field list; the docstrings now say the omission is deliberate.

### Laws

Framework, both language twins over the SAME shared fixture
(`🛂️manifest/🧫️fixtures/🔬️window-view-context/🔣️.json`, which gained `focusedWindowId: "right"`):
`panel.focusedWindowId === view.focusedWindowId` and a windowed projection carries it alongside its own
render target. The neutral-schema fixture gained a `focusedWindowId: ""` invalid case so Ajv and the
hand-written admission agree it is `Identifier`-shaped.

```
window-view-context cases=5 hostArmed=3 isolation=valid preferences=preserved
resolved-host-context cases=9 schema=valid explicit-preferences=required
```
```
running 4 tests
test manifest::view_context_capacity_tests::a_capacity_filled_context_fits_the_bound ... ok
test manifest::view_context_capacity_tests::capacities_match_the_neutral_schema ... ok
test manifest::view_context_capacity_tests::the_admission_bound_covers_every_schema_valid_context ... ok
test manifest::window_view_context_tests::window_view_context_uses_the_addressed_instance ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 223 filtered out; finished in 0.01s
```

Guest, 2 new laws plus a new testkit helper `render_panel_body(app, body_key, focused_window_id)` — the
first thing in this testkit that renders a body through `ViewModel::for_panel()`, i.e. the projection a
panel actually receives and the one `render_body`/`render_window_refresh` can never reproduce:

- `the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind` — with `-top` focused the
  section title names `-top` and all four steppers tag `windowId: "puzzle3d-main-top"`; same for
  `-perspective`; with nothing focused it still falls back to the roster.
- `a_settings_bump_retunes_only_the_focused_panes_rail` — the coordinator's law: bump grid spacing on the
  focused pane, that pane's rail carries the new value, the sibling keeps its own.

Red first, actually run: with the `focused_window_id` hop deleted from `puzzle3d_addressed_window_id`,
the first law fails and prints exactly B12 §5.1's live reading back out of the guest —

```
the Settings section names the pane it is addressed at …:
… "args":{"windowId":"puzzle3d-main"} … "label":"Settings — puzzle3d-main","role":"section" …
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 701 filtered out; finished in 0.08s
```

then green with the hop restored.

```
running 6 tests
test editor::puzzle3d::component::tests::a_settings_bump_retunes_only_the_focused_panes_rail ... ok
test editor::puzzle3d::component::tests::an_opening_camera_is_stable_and_never_overrules_a_pose_the_user_set ... ok
test editor::puzzle3d::component::tests::every_pane_opens_on_its_own_framed_camera_before_any_gesture ... ok
test editor::puzzle3d::component::tests::settings_panel_steppers_carry_their_value_and_the_trigger_they_dispatch_on ... ok
test editor::puzzle3d::component::tests::switching_the_example_reframes_every_unposed_pane_on_the_new_document ... ok
test editor::puzzle3d::component::tests::the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 696 filtered out; finished in 0.69s
```

---

## 3 Probe fixes (`🔍️browser-probe.ts`)

Every existing step name is unchanged; only locators and waits moved.

### 3.1 New helpers

- `cameraSettled(id, previous, budgetMs = 5000)` — polls `data-camera-json` every 250 ms until the pose
  leaves `previous`, then returns the last reading either way (a pane that genuinely never moves still
  fails). Replaces the fixed `waitForTimeout(1200)+500` / `1600` of `camera-orbit`/`camera-zoom`, which
  sampled before the host's trailing `CAMERA_SYNC_DEBOUNCE_MS` had even started the round trip — B12 §4.1's
  measured false negative. `camera-pan` uses the same waiter.
- `cameraIsPosed(raw)` — `position != target`, i.e. a real pose rather than the all-zero default.
- `domIdForAuthoredId(authored)` — resolves an authored ui-node id to the DOM id it actually carries.
  `uiNodeDomId` namespaces every node as `${surface}/${key}` (26/09/09/PROCEDURAL-3D-END-TO-END,
  `f39d4b0db3`), so matching the authored SUFFIX keeps the probe independent of the surface prefix
  instead of hard-coding `panel:…/`.

### 3.2 Step changes

- `window-distinct-camera` — was `Boolean(top.camera && persp.camera && top.camera !== persp.camera)`,
  which two all-zero poses could never satisfy but which also never said WHY. Now requires both to be
  real guest-published poses (`cameraIsPosed`) and to differ, and the message says so.
- §19 `settings-steppers-present` — was `[id^="puzzle3d-play-settings"]` (dead since the surface prefix
  landed). Now counts `[id*="puzzle3d-play-settings."]` nodes ending in `.control`, and logs the full
  `allSettingsIds` inventory on a miss.
- §19 `settings-grid-spacing-bumps` — resolves `puzzle3d-play-settings.grid-spacing.control` through
  `domIdForAuthoredId` (falling back to the field row), and drives `stepper-plus` with a real
  `mouse.down`/`mouse.up` pair: `Stepper`'s +/− is a press-and-hold and never fires on a synthesized
  `click` (B12 §2.5). Fails honestly with a named reason when no stepper is addressable at any prefix.
- §19 `settings-value-reaches-window-rail` — re-unfolds the measures rail before reading it, since opening
  the Settings panel can collapse it.
- `context-menu-selection-precondition` — `selectViaOutliner` clicked
  `[id^="panel:puzzle3d-play-document/"]`'s FIRST match, which is a group header, not an entity row. It now
  takes the first `[role="treeitem"] / [data-slot="tree-item"]` that is a real ENTITY row — rejecting both
  the tree ROOT (`puzzle3d-play-document`, itself a `treeitem`) and the three group headers
  (`…/puzzle3d-play-document.objects|.references|.target-volumes`) — logs the whole row inventory, the exact
  id it clicked, and each world surface's `data-status-json`, so a red verdict says WHICH row was clicked
  and whether the app-side selection domain saw anything. See §4.4 for the live reading.

---

## 4 Probe verdicts

`bun 🔍️browser-probe.ts --only=window-content,camera-gestures,settings-panel --reload-between-groups --port=6013`
(the coordinator's list is VERDICT names; `--only` takes STEP names, and these three steps are exactly the
seven verdicts asked for). Run with `pgrep -f "browser-probe|lane-probe"` clear, after polling it out.
Full log: `🗑️generated/probe-2026-09-11T16-14-26.md` / `.ndjson`; parse-only dry run first:
`--only=boot --port=6013` → `done booted=true faults=0 hard=0 collateral=0 verdicts=6`.

```
[52.5s] battery PASS=15 FAIL=3 FAULTS=0
```

| verdict | B12 (#45) | this run (#46) | why |
|---|---|---|---|
| `camera-orbit` | **FAIL** | **PASS** | the polling waiter — `camera settle puzzle3d-main-perspective moved=true waitedMs=5000` |
| `camera-zoom` | **FAIL** | **PASS** | same — `moved=true waitedMs=1797` |
| `camera-pan` | PASS | PASS | `moved=true waitedMs=4093` |
| `camera-per-window` | PASS | PASS | the Top pane is untouched by a Perspective gesture |
| `camera-json-attribute` | PASS | PASS | — |
| `settings-steppers-present` | **FAIL** `ids=["puzzle3d-play-settings"]` | **PASS** | the four `panel:puzzle3d-play-settings/…​.control` ids now resolve through the authored suffix |
| `settings-grid-spacing-bumps` | **FAIL** `plusButtons=0` | **PASS** | `stepper-plus` driven with a real `mouse.down`/`mouse.up` |
| `window-distinct-camera` | PASS (vacuous, two zero poses) | **FAIL, honestly** | guest, rides the next wasm — see below |
| `settings-value-reaches-window-rail` | FAIL | **FAIL, honestly** | guest, rides the next wasm — see below |
| `camera-emits-no-artifact-history` | FAIL | FAIL | B10's `dispatch_emit` guard, not this wave |

Three of the seven asked-for verdicts flipped red→green on the live host; three more stayed green; the
remaining one is the honest red below.

### 4.1 `window-distinct-camera` — exactly the defect §1 fixes, still on the old wasm

```
top={"position":[0,0,0],"target":[0,0,0],"up":null,"zoom":0,"fov":50,"projection":"perspective"}
perspective={"position":[0,0,0],"target":[0,0,0],"up":null,"zoom":0,"fov":50,"projection":"perspective"}
```

Both panes publish `Puzzle3dCamera::default()` verbatim. That is the reading §1's framing is written
against; it cannot move until the guest is rebuilt. The verdict's own message now says so instead of
passing on two identical non-poses.

### 4.2 `settings-value-reaches-window-rail` — the served wasm predates B12's guest half too

```
settings={"tag":"input","slot":"input","value":"0.5"} windowRail={"slider","value":"10"}
```

The stepper rendered EMPTY and one bump produced `0.5`, i.e. `defaultValue (0) + step` — the exact
`uniform: false` MIXED symptom B12 §2.6 diagnosed and fixed in the guest. So the wasm behind `:6013`
carries neither B12's `uniform: true` nor this wave's focused-pane hop, and `0.5` never had a chance to
reach a rail that reads `10`. Both halves ride the next build; the probe now measures the right controls
and fails for a stated reason instead of `settings=null`.

### 4.3 What can and cannot flip on `:6013`

That serve is wasm **#46** with a vite-live host, so

- **host-live now** (the run above already includes them): every probe change, the `focusedWindowId`
  stamp and the focus-change panel refresh (`faults=0 hard=0` — the new effect introduces none);
- **rides the next wasm (#47)**: the guest half — the framed opening camera (§1) and
  `puzzle3d_addressed_window_id`'s focused-pane hop (§2). On #46 the Settings panel still bakes
  `puzzle3d-main` into its stepper args no matter what the host sends, and both panes still publish the
  all-zero pose.

### 4.4 `context-menu-selection-precondition` — selector fixed, verdict still honestly red

Measured live in three separate `--only=context-menu-rows --port=6013` runs
(`🗑️generated/probe-2026-09-11T16-27-19|16-27-58|16-28-59.md`).

Run 1, with only the `treeitem` filter in place, showed the probe was still clicking the wrong thing —
because the outliner's ROOT is a `treeitem` too:

```
treeRows=[{"id":"puzzle3d-play-document","text":"OBJECTS Hexagonal Cut Concrete Forest Le"},
          {"id":"panel:puzzle3d-play-document/seed-left-001", …},
          {"id":"panel:puzzle3d-play-document/ref-masterarbeit", …},
          {"id":"panel:puzzle3d-play-document/ref-rathaus-ahlen", …}]
target=puzzle3d-play-document
```

So the row picker now rejects the root and the three group headers
(`…/puzzle3d-play-document.objects|.references|.target-volumes`) and takes the first real ENTITY row.
Runs 2 and 3 click exactly that:

```
selectViaOutliner clicked=panel:puzzle3d-play-document/seed-left-001 entitySelected=[]
  allAriaSelected=["mode-dock-tab-0-puzzle3d-main-top=Top","mode-dock-tab-1-puzzle3d-main-perspective=Perspective"]
  surfaceStatus=["1=","1="]
verdict context-menu-selection-precondition FAIL outlinerRows=4 clicked=panel:puzzle3d-play-document/seed-left-001 selected=0 surfaceStatus=["1=","1="]
```

**It does not land**, and the evidence is now specific rather than `outlinerRows=1 selected=0`: a real
object row is addressable and clicked by id, nothing in the DOM ends up selected (the only
`aria-selected` nodes are the two dock tabs), and BOTH world surfaces publish an EMPTY
`data-status-json`, so the framework selection domain never saw the pick either. `context-menu-opens`
reads `rows=0` behind it, exactly as before. This is a product defect on the wasm `:6013` serves, not a
probe-selector artifact any more — whether B6/B9's first-pick chain fixes it is a question for the build
that carries them; it is not fixed here and is not claimed to be.

---

## 5 Verification

| gate | result |
|---|---|
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `Finished dev profile … in 16.60s`, **0 errors**, 88 warnings (B12's exact baseline count) |
| whole guest lib, `RUST_MIN_STACK=134217728 … --lib -- --test-threads=1` | `test result: FAILED. 699 passed; 3 failed` |
| `cargo test -p semio-framework --lib window_view_context view_context` | `4 passed; 0 failed` |
| `bun 📜️script.ts verify window-view-context` / `verify resolved-host-context` | both green (tails in §2) |
| renderer-react vitest (`SEMIO_TEST_LEVEL=long`) | `Test Files 3 failed | 21 passed (24)` / `Tests 9 failed | 875 passed (884)` |
| `bun x tsc --noEmit` (renderer-react project) | no new error in any file this wave touched |

The three red guest laws are B12's exact three and none touches a file this wave changed:
`open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin` (wall-clock budget,
the long-standing flaky class), `two_instances_converge_disjoint_object_edits_via_backbone`, and
`every_advertised_engagement_verb_is_implemented` (the selection/engagement lane B13 is live in).

The nine red vitest tests are **exactly B10/B12's nine** — six `🧪️tests/🧩️package-integration/🟦️.ts`
(generated wgpu worker bytes / the Bun pin), two `🔌️PluginRuntime`, one engine-contract
`buildNoteShellCommandAction`. B12's three extra `submitPluginTurn` failures are gone (B14). **Nothing
new from this wave.**

`tsc` hits in touched files, all pre-existing and unrelated to the insertions: `🛂️manifest/🟦️.ts(1112)`
`ImportMeta.dir` (the file's already-existing `registerTests` call), `🏛️ShellHost/🟦️.tsx(1905|7786|7787)`
(the interaction-state and note-shell-command shapes), and the two `row.remove`/`AssertPredicate` hits in
the two view-context test files, which predate this wave's fixture additions.

---

## 6 Files touched

- `🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧬️schema/🔣️.json` — `focusedWindowId`.
- `🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧫️fixtures/🪟️resolved-host-context/🔣️.json` — valid value + `invalid-focused-window` case.
- `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` — `PluginViewState.focusedWindowId`, admission, `panelViewContext` docstring.
- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` — `ViewModel::focused_window_id`, `VIEW_CONTEXT_IDENTIFIER_FIELDS`.
- `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🔬️window-view-context/🔣️.json` — `focusedWindowId`.
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️window-view-context/🦀️.rs` + `/🟦️.ts` — the survival law, both twins.
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️view-context-capacity/🦀️.rs` — the capacity-filled literal.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — three `focusedWindowId` stamps + the focus-change panel refresh effect.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — the five `ViewModel` literals + `live_view_state`.
- `✏️s/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` — `camera_unset`, `framing_bounds`, `framing_projection`, `framed_camera`, `frame_unset_camera`.
- `✏️s/…/✏️editor/🦀️.rs` — `scene_for` + `render_body` frame an unset pane; `puzzle3d_addressed_window_id`'s focused hop + docstring.
- `✏️s/…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` — `render_panel_body`.
- `✏️s/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — 5 new laws.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts` — §2/§15/§19 locators and waits.
