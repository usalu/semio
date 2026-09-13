# 🐚️ wgpu SHELL CHROME — example picker, mode/role groups, preview cancel, `Fit graph` (2026-09-13)

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "wgpu shell chrome parity".
Closes `📓️audit-wgpu-journey-readiness-2026-09-13.md` §5 lanes **2**, **3** and **6** at the source
level, plus §5 lane 5's chrome half (`Fit graph`), with fixture-driven Rust laws and a TypeScript
twin. **Runtime on 6118 is NOT claimed** — see §6, which names the exact hop that blocks it and why
it is not this lane's.

Repo MCP was down all session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`),
so no ticket was opened, closed or reopened. The procedural guest was **not** restaged (its staged
module is still `2026-09-12 06:56`), the react serve on 6018 was not touched, 6118 is the peer's
server and was **not** restarted (its own report records that new renderer wasm needs only a page
reload), and no git-state-modifying command was run.

Abbreviations below: `wgpu-shell.rs` =
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`.
Line numbers are as of this writing; the file is under concurrent edit by three other lanes.

---

## 1. TL;DR

| deliverable | state |
|---|---|
| **Example picker** | **Done.** `playground.navbar.fixture` is painted, opens a dropdown, and the dropdown emits one `shell.example.<id>` hit target per example of the OPEN DIALECT (`manifest::examples_for_app`). The dead handler the audit found is now reachable. |
| **Mode group** | **Done.** `playground.navbar.modes.<id>` per declared mode, pressed state, localized from each `ModeDefinition`'s own label; nothing painted for a one-mode surface. |
| **Role group** | **Done.** `playground.navbar.roles.{editor,viewer}`, rendered only when the plugin declares BOTH surfaces of the open dialect, backed by a **transactional** `create → seal → retire → publish → refresh` session switch — the wgpu counterpart of React's `switchToPluginApp`. |
| **Chords** | **Done.** `mod+alt+e` / `mod+alt+v` (role) and `mod+alt+←` / `mod+alt+→` (mode) are reserved shell chords and outrank every app-declared keybinding. |
| **Preview cancel** | **Done.** A cancel control appears over a World3d surface exactly while its own `statusJson` says `cancellable` AND names a non-empty `cancelAction`, and dispatches **that id**, never a compiled-in verb. Needed a new `status_json` lane from the scene into the shell. |
| **`Fit graph`** | **Done.** A control on every node-graph pane plus the `F` shortcut React declares; both frame the graph through `DagHost::fit_camera_to_content` and persist the result through `nodeGraphViewport` the way a pan/zoom gesture is persisted. |
| **`?example=` boot query** | **Not done, deliberately.** `🚀️browser-boot/🟦️.ts` is owned by the concurrent `wgpu-server-input-present` lane and is additionally read back by an AST/`node:vm` oracle (`🧪️tests/🧊️wgpu-browser-boot-cache-inputs`), so a second lane editing it this session would have raced both. Left for whoever owns that file next. |
| **Tests** | 13 new Rust laws, **13 passed / 0 failed**; 3 TypeScript twin tests, **3 passed**. Commands and verbatim output in §5. |
| **Runtime on 6118** | **Blocked, not claimed.** Two independent blockers, §6. |

---

## 2. Example picker — the dead handler made reachable

### 2.1 The three halves the audit found missing

`📓️audit-wgpu-journey-readiness-2026-09-13.md` §1 established that `handle_control_command` already
had both `"playground.navbar.fixture"` and `id if id.starts_with("shell.example.")` (the latter
dispatching the correct `setActiveExample`), while **nothing ever painted either control**, and that
the example RESOLUTION that did run predated the dialect refactor. All three are now closed:

| half | where | what it does now |
|---|---|---|
| resolution | `wgpu-shell.rs:3571` `session_example_rows` + `sync_session_chrome` | resolves through `semio_framework::manifest::examples_for_app(&plugin.manifest.examples, &session.app)` — the ONE shared predicate, twin of React's `examplesForApp`, pinned by `🛂️manifest/🧫️fixtures/📚️example-picker.json`. The old body read `plugin.manifest.examples` whole, so a plugin publishing several dialects offered another artifact's fixtures. |
| trigger | `wgpu-shell.rs:11841` `render_navbar_step` phase 4 | paints a `playground.navbar.fixture` item, gated on `rows.is_empty()` — React's `exampleOptions.length > 0` gate verbatim. Its label is the SELECTED row's own localized label (a select shows its value), falling back to the localized noun; an open dropdown is its pressed state. |
| rows | `wgpu-shell.rs:12300` `render_overlay_step` phase 2 | one hit-testable row per resolved example, `control_id = format!("shell.example.{id}")`, `HitKind::DropdownItem`, selected row highlighted — the row-emission half that existed before the namespace refactor (`…/DEGENERALIZE-…/before/wgpu-lib.rs:21912`) and was dropped, rebuilt against the new dialect-keyed list. |

`"playground.navbar.fixture"`'s handler now **toggles** rather than only opening, so a second click
closes the dropdown (`wgpu-shell.rs:6626`). No guest change was needed: `setActiveExample` already
exists, is `ActionKind::View` (so a viewer session may fire it), and the dispatch call site was
already correct.

### 2.2 Localization

Row labels are each `ExampleDefinition`'s own `LocalizedLabel`, resolved through
`label.resolve(self.active_terminology(), self.active_locale())` — no shell dictionary, no default
language. The trigger's fallback noun and the overlay title are new `shell_chrome_string` rows
(`example.picker` → `Example`/`Beispiel`, `example.overlay.title` → `Examples`/`Beispiele`), the same
curated en/de table the rest of this shell's own chrome text lives in.

---

## 3. Mode + role groups, the chords, and the transactional switch

### 3.1 The navbar centre cluster

`render_navbar_step` gained three phases between the app title and the existing right-hand cluster
(fullscreen + panel toggles), each driven by one shared incremental painter,
`render_navbar_cluster_step` (`wgpu-shell.rs:12024`), which emits at most one scalar/glyph/hit per
grant — the retained-chrome discipline the rest of this navbar already follows. Phase 3 now also
advances `cursor.x` past the measured title, which it previously did not (the title's width was never
consumed, so anything painted after it would have overlapped).

| phase | control ids | render gate |
|---|---|---|
| 4 | `playground.navbar.fixture` | the open dialect authored ≥ 1 example |
| 5 | `playground.navbar.modes.<id>` | `app.modes.len() >= 2` — a one-mode surface (generation3d's viewer) paints no switcher, exactly as React renders none |
| 6 | `playground.navbar.roles.{editor,viewer}` | the loaded plugin declares BOTH surfaces for the OPEN document's dialect (`surface_role_apps`) |

Role labels come from each target `AppDefinition`'s own `LocalizedLabel`; role icons are fixed per
role (`pencil` / `eye`) and deliberately NOT each app's `iconId`, because both surfaces of one
artifact carry the same artifact icon and the two buttons would be indistinguishable. Focus order is
editor → viewer, matching `SURFACE_ROLE_ORDER`.

### 3.2 The chords are SHELL verbs

`is_reserved_shell_chord` (`wgpu-shell.rs:8968`) now reserves the alt axis alongside the existing
palette/find/panel/nav/fullscreen chords, and `handle_keyboard_async` matches them ahead of every
app-declared keybinding. The reason is the same one the React lane recorded: both verbs are pure
shell-state transitions that never reach the guest, so an app keybinding structurally cannot express
them.

| chord | resolver | effect |
|---|---|---|
| `mod+alt+e` / `mod+alt+v` | `shell_role_chord` (`:9111`) | `switch_to_session_role(AppRole::{Editor,Viewer})` |
| `mod+alt+→` / `mod+alt+←` | `shell_mode_step_chord` (`:9126`) | `apply_mode_step(±1)` → `step_mode_id` → `apply_navbar_mode` |

`apply_navbar_mode` (`:6155`) is now the ONE body behind a mode change, whether a navbar button or a
chord asked for it, so keyboard and pointer cannot diverge. The alt axis is disjoint from the
existing accelerators — `mod+f` is still find, `mod+alt+f` is not, and a bare `f` is the `Fit graph`
shortcut (asserted, §5).

### 3.3 `run_session_app_switch` — create before retire, one create per switch

`switch_to_session_role` (`:6051`) resolves the sibling through `role_switch_target` (a role already
mounted, or a dialect with no sibling, answers `None` and nothing at all happens) and hands it to
`run_session_app_switch` (`:6072`), which runs, in this order:

1. **create** — `program.create_app(&app.id)`, so the successor exists before anything is torn down;
2. **seal** — `retire_documents_outside(&[], true/false)` drives every retained window and panel
   document of the outgoing app to terminal. This is not cosmetic: `UiResidentPermit` is a FIXED
   process-wide aggregate, and a leaked root refuses every later surface with `Capacity`
   (`📓️wgpu-dock-layout-world3d-2026-09-12.md` §6.2);
3. **retire** — `program.destroy_app(previous.instance_id)`. The two pre-existing wgpu switches
   (`switch_to_managed_app`, `switch_to_app`) never destroyed anything, i.e. they leaked one
   `create_app` per switch, exactly the defect React's lane fixed on its side;
4. **publish** — `push_contributions()`;
5. **seed + refresh** — landing window, `layout_override = None`, `sync_dock()`, `refresh_ui()`.

Each step emits a `[DEBUG] shell session switch {"step": …}` line, so the ladder is readable from a
browser console the moment 6118 boots again.

React's fixture also declares a `quiesce` step before `seal`. It has **no wgpu counterpart** — this
target exposes no pending-guest-work tracker to quiesce — and the law therefore does not assert it
rather than faking it (§5.1, `the_session_switch_creates_before_it_retires_and_leaks_no_instance`).

---

## 4. Preview cancel and `Fit graph`

### 4.1 The cancel contract needed a lane into the shell

`World3dScene.statusJson` already carried `"cancellable":true,"cancelAction":"cancelPreviewEval"`,
and the wgpu renderer **dropped it entirely**: `World3dState` keeps geometry, never status, and
`EngineSurfaceKindDetail::World3d` carried no payload at all. Three surgical, additive changes open
that lane:

| file | change |
|---|---|
| `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` | `EngineSurfaceKindDetail::World3d` → `World3d { status_json: Option<String> }` (the same shape `TiledMap`/`Board2d` already use to project a per-kind tail into the shell) |
| `🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` | the World3d attach passes `scene.world_3d…status_json.clone()` into that registration |
| `wgpu-shell.rs` | new `world3d_status: HashMap<String, String>` field, filled by the same per-frame `sync_engine_surface_states` walk that mirrors bounds and pruned against the live surface set |

`world3d_cancel_affordance` (`:9168`) is the Rust twin of `world3dComputeStatusV1`
(`🔨️modules/🖱️ui/🎬️scene/🟦️.ts`): total by construction (malformed JSON, a non-object payload, a
hostile `cancellable` type and an absent status all degrade to "no affordance"), and `cancellable` is
honoured only alongside a non-empty `cancelAction` — a button with nothing to dispatch is worse than
no button. The hit handler dispatches **whatever id the surface published**; a law asserts the string
`"cancelPreviewEval"` appears nowhere in the shell source, so the funnel stays domain-neutral.

### 4.2 `Fit graph`

New `node_graph_fit_camera(surface_id) -> Option<[f64; 3]>` in EngineCanvas calls the existing
`DagHost::fit_camera_to_content` (the very twin React's `fitGraphToView` already names) and answers
the camera it installed. `fit_node_graph_camera` (`:6112`) then dispatches
`nodeGraphViewport {surfaceId, viewport:{x,y,zoom}}` — the identical action the wheel/pan gestures
already persist through — so the next open honours the fit.

Reachable two ways: the painted control, and **`F`** (React's own `aria-keyshortcuts="F"`), matched
AFTER the content-focus routing block so a plain `f` typed into a focused note or field is never
hijacked, and consumed only when a graph surface actually resolves
(`keyboard_fit_surface_id`: the surface under the pointer, else the only live one — ambiguity answers
`None` rather than re-framing a pane the user was not pointing at).

### 4.3 Where both controls are painted, and the honest limit on "keyboard reachable"

Both are emitted in the overlay pass (`render_overlay_step` phase 7) anchored inside their surface's
own bounds, so they paint above the surface and — because `InputState::hit_at` scans in reverse
registration order — win the hit over it. A pane too small to carry a control offers none.

**Keyboard:** `Fit graph` has a real shortcut (`F`). The cancel does **not**, and this is stated
rather than papered over: this shell has no chrome focus traversal at all — not for my two controls
and not for any existing navbar button — so building one is a lane of its own. What the cancel verb
does have today is the command palette (`mod+p`) and the right-click context menu, both of which
already list the focused window's resolved actions, and `cancelPreviewEval` is an unowned action that
`build_definition` copies onto every window kind
(`📓️preview-eval-cancellation-2026-09-12.md` §1.2). React's own cancel is a plain tab-reachable
`<button>`; the wgpu equivalent of that reachability is owed by a future chrome-focus lane.

---

## 5. Tests — all run in the foreground, results verbatim

### 5.1 Rust: `🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs` (new, 13 laws)

Fixture-driven, and every fixture is **shared with another implementation**:

| fixture | owner | what it answers here |
|---|---|---|
| `🛂️manifest/🧫️fixtures/📚️example-picker.json` | the viewer-examples lane; already answered by `manifest::examples_for_app`'s own Rust law **and** by the TypeScript `examplesForApp` twin | the picker's rows and the trigger's render gate, 5 dialect cases |
| `🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json` | the React surface-switch lane | 4 `group` rows, 4 `roleTargets` rows, 6 `modeSteps` rows, 4 `keybindings` rows, and the `switch` step order |
| `🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json` | **new**, answered by this Rust law **and** by a TypeScript twin (§5.2) | 9 cancel-contract rows, 5 surface-control rows |

The new fixture carries `controlHeightPx` (the shared `UI_SPACING_COMPACT_PX × CONTROL_HEIGHT_UI_SPACING`
token, 22.4) so the TypeScript twin applies the very same size gate without re-deriving it, and the
Rust law asserts the fixture's declared value still equals `Theme::control_height`.

```
RUST_MIN_STACK=33554432 CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false \
  cargo test -p semio-framework-os-renderer-wgpu --lib -- shell_chrome_parity_tests:: --test-threads=1 --nocapture
```
→ **13 passed; 0 failed** (515 filtered out), with:

```
[DEBUG] wgpu example picker: 5 dialect cases answered by the shared 📚️example-picker fixture
[DEBUG] wgpu example picker trigger: localized label, selected row and pressed state all hold
[DEBUG] wgpu roles group: 4 shared fixture rows answered identically to React
[DEBUG] wgpu role switch targets: 4 shared fixture rows answered identically to React
[DEBUG] wgpu mode group: two-mode editor paints both ids, one-mode viewer paints none
[DEBUG] wgpu mode step: 6 shared fixture rows answered identically to React
[DEBUG] wgpu shell chords: 4 shared fixture rows route to the same verb React's SHELL_KEYBINDINGS declare
[DEBUG] wgpu shell chords: the alt axis is disjoint from the palette/find/panel accelerators
[DEBUG] wgpu world3d cancel contract: 9 fixture rows, hostile payloads included, degrade to no affordance
[DEBUG] wgpu surface controls: 5 fixture rows offer the Fit graph and cancel hit targets exactly when the surface declares them
[DEBUG] wgpu world3d cancel: the dispatched verb is the published one and no domain verb is compiled in
[DEBUG] wgpu session switch order: create → seal → retire → publish → refresh, one create and one retire; shared fixture declares ["quiesce", "seal", "create", "retire", "publish", "seed", "refresh"]
[DEBUG] wgpu session switch: asking for the mounted role is a no-operation
```

The requested coverage, by law:

| requirement | law |
|---|---|
| navbar hit targets exist iff the app declares them | `the_example_picker_offers_every_example_of_the_open_dialect_and_nothing_else` (trigger present iff rows non-empty), `the_roles_group_exists_exactly_when_the_plugin_declares_both_surfaces_of_the_open_dialect`, `the_mode_group_renders_one_pressed_button_per_declared_mode_and_none_for_a_single_mode_surface` |
| picking a row dispatches `setActiveExample` | `the_example_picker_offers_…` asserts every row's `control_id` is exactly `shell.example.<id>` in manifest order — the id whose handler (`wgpu-shell.rs:6644`) dispatches `setActiveExample`; `…trigger_shows_the_picked_row…` pins the selected/pressed state and the localized (de) labels |
| chords route | `every_shared_keybinding_row_routes_to_its_shell_verb_and_outranks_app_keybindings` builds each fixture row's real `KeyAction`+`PointerModifiers`, asserts `is_reserved_shell_chord`, asserts the role/mode resolver answers what the row declares, and feeds a role row on through `role_switch_target` to the app id the fixture names; `the_alt_axis_never_swallows_a_neighbouring_chord` is the negative half |
| cancel/fit controls dispatch | `a_live_surface_offers_exactly_the_overlay_controls_the_shared_fixture_declares` (ids + anchors), `the_cancel_contract_is_read_the_way_the_shared_fixture_declares`, `the_cancel_control_dispatches_only_the_action_the_surface_itself_published` |
| the switch is transactional | `the_session_switch_creates_before_it_retires_and_leaks_no_instance` (a source-order law over `run_session_app_switch`'s own body: `create < seal < retire < publish < refresh`, exactly one `create_app` and exactly one `destroy_app` — `created − retired === 0`), `asking_for_the_role_already_mounted_creates_and_retires_nothing` (driven through a real `ShellState`) |

### 5.2 TypeScript twin

Three `it`s appended to the already-registered `🧪️tests/🔬️engine-contract/🟦️.ts`
(`describe("🛑️ world3d cancel contract")`) — deliberately no new nx target and therefore no new
`launch.json` row, since the suite and its gate already exist. **ajv** (third party) validates the
new fixture against a schema declared in the suite; the shipped `world3dComputeStatusV1` parser then
answers the same 9 cancel rows and the same 5 surface-control rows the Rust law answers, with two
independent implementations and one fixture.

```
NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:test-long -- -t "world3d cancel contract"
```
→ **Test Files 1 passed | 34 skipped; Tests 3 passed | 1072 skipped.**

### 5.3 Whole-crate and typecheck

```
CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false cargo check -p semio-framework-os-renderer-wgpu --lib
```
→ **0 errors** (warnings only).

```
RUST_MIN_STACK=33554432 cargo test -p semio-framework-os-renderer-wgpu --lib -- shell:: --test-threads=1
```
→ **183 passed; 6 failed.** All six reproduce when run **alone**, and none touches a function this
lane changed:

| failing test | attribution |
|---|---|
| `shell_document_retirement_tests::*` (2) | `ArenaFull` on the process-wide `UiDocument` arena; the retirement registry, untouched here |
| `panel_anchor_model_tests::panel_layout_round_trips_through_prefs_store`, `…_is_idempotent_when_nothing_changed` | the panel-tab lane's in-flight prefs-store work (same lane that moved this test's fixture, §7) |
| `chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments` | `WindowSilhouette` geometry, the dock lane's |
| `command_registry_tests::directory_home_bootstrap_retries_cancels_and_rebootstraps_without_cursor_loss` | the directory client, untouched here |

```
NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:typecheck
```
→ 810 errors repo-wide, **none** on any line this lane added (the highest engine-contract error is at
line 8792; the new block starts past 11 380).

---

## 6. Runtime on 6118 — attempted, not claimed

The renderer wasm was rebuilt (`bun nx run @semio-tech/framework-renderer-wgpu:wasm`, 5 m 52 s,
`dist/wasm-dev` re-published 04:37) and the page reloaded — the peer's server needs no restart, and
6118 is its process (`bun` pid 55855), so nothing of it was touched.

**The shell never finishes booting on the freshly built host wasm**, in edit mode and in generate
mode alike. Full evidence in `🗑️generated/wgpu-chrome/runtime-6118.txt`; the shape:

```
[DEBUG] wgpu-shell render begin surface=procedural-main body=procedural.play.main
[DEBUG] wgpu-bridge renderSurface surface=procedural-main turn=0 … intakeSteps=0
… turn=1 … turn=2 … turn=3 …
<< nothing further for > 6 minutes; no `render leave`, no fault banner, no beacon >>
DOM: [role=status] "shell-boot 86%", [role=alert] null, canvas 1×1
```

Why this is **not** attributable to this lane, stated with its evidence:

1. The stop is inside `renderSurfaceSerialized`'s 5th `await submitTurn(...)`
   (`🐚️plugin-bridge/🟦️.ts:927-940`). Its budget is `RETAINED_DOCUMENT_OPPORTUNITIES = 256`, so it is
   not budget exhaustion, and neither `wgpu-ui.render-budget-exhausted` nor
   `wgpu-ui.surface-not-published` is thrown.
2. `render leave` never logs, so **no shell chrome step has run even once** — the navbar and overlay
   code this lane added is not reached before the hang.
3. It reproduces on a different first surface (`generation3d-generations`, generate mode) with
   `effects=0 carried=0`, so the carried `dispatchAction` effect visible in edit mode is not the cause.
4. The same guest wasm (staged `2026-09-12 06:56`, unchanged) booted in 27.6 s at 02:49
   (`🗑️generated/wgpu-input/baseline/console.txt`, `ui chain settled after 1 round(s)`) and in 1.9 s at
   04:17 (`…/hit-after-fix/console.txt`) — in the baseline, `turn=4` followed `turn=3` after 2.7 s.
   The host wasm built at 04:37 carries every lane's Rust that landed in the intervening twenty
   minutes, not only this one's.

**Independently, clicking could not have been proven even with a healthy boot.** The concurrent
`wgpu-server-input-present` lane's own report (`📓️wgpu-server-input-present-2026-09-13.md` §1, landed
04:30) states the remaining gap in its own words: the drained input never reaches
`Ui::dispatch_event` — "the `RuntimeApply::DispatchEvents` the host enqueues is never applied". Every
control this lane adds is a hit target resolved by `Ui::dispatch_event`. So per the brief's
instruction, this lane **stops at native proof** and says so.

What a future runtime pass should measure, once both hops are closed:

1. boot `?plugin=generation3d`, dump the navbar subtree, assert the `playground.navbar.fixture`,
   `playground.navbar.modes.*` and `playground.navbar.roles.*` hit targets are present;
2. click the fixture trigger, assert 8 `shell.example.<id>` rows, click one, assert a
   `renderBeginDelta` on the Flow window;
3. press `⌘️⌥️V` / `⌘️⌥️E` and assert the `[DEBUG] shell session switch` ladder logs
   `create → seal → retire → publish → refresh` with one create and one retire per switch;
4. press `⌘️⌥️→` and assert the dock plan flips to the 3-window generate layout;
5. with a preview computing, assert the cancel control appears and that clicking it logs
   `[DEBUG] shell world3d cancel {"action":"cancelPreviewEval"}`;
6. press `F` over the Flow pane and assert `[DEBUG] shell node-graph fit` plus a `nodeGraphViewport`
   dispatch.

---

## 7. Two peer breakages repaired in passing (not reverts, not behaviour changes)

Both had the whole crate's **test target** wedged for every lane, and both were left alone for
20+ minutes first:

1. `wgpu-shell.rs` `handle_pointer_move` — the catalogue-drop-preview lane called
   `self.sync_world3d_catalogue_drop_preview(…)` (which takes `&mut self`) while holding
   `&mut self.tree_drag`; `E0499`, the crate did not compile at all. Repaired by snapshotting the
   drag payload first and re-taking the borrow afterwards; behaviour identical, comment records why.
2. `🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs` — the panel-tab lane moved the `📌️panel-state` fixture
   from `🧑‍🎨engine/🧫️fixtures/` into `🐚️Shell/🧫️fixtures/` and `PanelTabKind` stopped being reachable
   through the shell module's `use` list; the test's `include_str!` and one import were re-pointed.
3. `🎯️targets/🧊️wgpu/🧪️tests/🎮️wgpu-browser-input-wire/🦀️.rs` — `🎮️input-wire/🦀️.rs` registers this
   module with `#[cfg(test)] #[path = …]` and the file did not exist (`couldn't read …`), wedging
   `cargo test --lib` for the crate. Created as an **empty, doc-comment-only placeholder** — no
   assertion was invented on that lane's behalf — so the test target builds; the
   `wgpu-server-input-present` lane owns the real laws and simply replaces the file.

---

## 8. Files

| file | change |
|---|---|
| `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | **the lane.** `//#region 🔀️ChromeParity` (`:9017-9180`): `ShellNavbarControl`, `ShellExampleRow`, `shell_example_rows`, `shell_example_control`, `shell_mode_controls`, `surface_role_apps`, `role_switch_target`, `shell_role_controls`, `step_mode_id`, `shell_role_chord`, `shell_mode_step_chord`, `World3dCancelAffordance`, `world3d_cancel_affordance`, `surface_overlay_controls_for`. `is_reserved_shell_chord` extended. `session_example_rows` + dialect-keyed `sync_session_chrome`. `switch_to_session_role`, `run_session_app_switch`, `apply_mode_step`, `apply_navbar_mode`, `fit_node_graph_camera`, `keyboard_fit_surface_id`. `render_navbar_step` phases 4/5/6 + `render_navbar_cluster_step`; `render_overlay_step` phases 2 and 7 + `surface_overlay_controls`. New `world3d_status` state and its fill in `sync_engine_surface_states`. New hit arms for the roles group, `shell.nodeGraph.fit::*` and `shell.world3d.cancel::*`; `playground.navbar.fixture` toggles. `F` chord. Eight new `shell_chrome_string` en/de rows. Test module registration. Plus the peer repair in §7.1. |
| `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` | `EngineSurfaceKindDetail::World3d { status_json }`; new `node_graph_fit_camera` |
| `🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` | the World3d attach passes the scene's `status_json` into its registration |
| `🧱️elements/🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json` | **new** — 9 cancel-contract rows, 5 surface-control rows, the shared `controlHeightPx` token |
| `🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs` | **new** — 13 laws |
| `🧪️tests/🔬️engine-contract/🟦️.ts` | **new** `describe("🛑️ world3d cancel contract")` — the ajv-validated TypeScript twin |
| `🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs` | peer repair (§7.2) |
| `🎯️targets/🧊️wgpu/🧪️tests/🎮️wgpu-browser-input-wire/🦀️.rs` | **new** placeholder (§7.3) |
| `<ticket>/🗑️generated/wgpu-chrome/` | `shell-suite.txt`, `ts-twin.txt`, `wasm-build.txt`, `runtime-6118.txt` |

No `launch.json` / `🧩️launch.seed.jsonc` row was added: the Rust laws run under the existing
`@semio-tech/framework-renderer-wgpu:test-native` gate (`⚖️gate📺️renderer🧊️wgpu🦀️native`) and the
TypeScript twin under the existing `@semio-tech/framework-renderer-react:test-long`.
