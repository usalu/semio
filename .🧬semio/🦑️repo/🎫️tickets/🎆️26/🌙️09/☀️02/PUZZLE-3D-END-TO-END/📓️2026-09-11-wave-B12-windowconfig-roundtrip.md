# Wave B12 — the per-window `WindowConfig` round trip, end to end

Implementation pass, 2026-09-11. Closes the "unresolved blocker" of
`📓️2026-09-11-wave-B10-framework-chrome.md` (§1/§3: the window-option toggles and selects were fixed as
*gestures* and still read `before == after`), plus `📓️2026-09-11-audit-A2-unproven-sections.md` §3/§4/§19
and the checklist's §2 camera claim.

Every command tail and every browser reading quoted below is real output from this pass.

**Headline**: the six failing `window-option-*` verdicts are **PASS**, host-live, on wasm #45.

```
[42.4s] verdict window-option-puzzle3d-play-grid-visible PASS
[45.6s] verdict window-option-puzzle3d-play-grid-snap PASS
[48.4s] verdict window-option-puzzle3d-play-grid-spacing PASS
[51.6s] verdict window-option-puzzle3d-play-lod-auto PASS
[54.9s] verdict window-option-puzzle3d-play-lod-value PASS
[58.1s] verdict window-option-puzzle3d-play-vortex-show PASS
[61.5s] verdict window-option-puzzle3d-play-vortex-direction PASS
[64.9s] verdict window-option-puzzle3d-measure-sun-enabled PASS
[64.9s] verdict window-options-lane-responsive PASS
[66.3s] verdict window-options-emit-no-history PASS
```
(`🗑️generated/b12-window-options-6013.txt`, `--only=activate-perspective,camera-gestures,window-options,settings-panel --reload-between-groups --port=6013`.)

---

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD. No `git commit`/`stash`/`checkout`, no worktree, no
  `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`, everything foreground. The ticket was not opened/closed by this wave.
- The repo MCP server did not connect (`repo (-32602): invalid initialize params`), so the ticket folder
  is managed on disk.
- Live target: the already-running `:6013` dev serve, wasm component **#45** with a vite-live host. The
  probe was run only with `pgrep -f "browser-probe|lane-probe"` clear. A separate read-only browser tab
  was used for diagnosis (own browser profile, own plugin instance, never the probe's tab lease); it was
  closed and its `SEMIO_RUNTIME_DIAGNOSTICS` localStorage key cleared afterwards.
- Peers edited the same files throughout. Two of their hunks are relevant and were **kept, not reverted**:
  `window_ownership::addressed_config_for`/`addressed_transient_for` (the silent-drop removal B10 asked
  for) and `settings_panel::stepper_window_args`. The first was left mid-edit and did not compile
  (`E0382: borrow of moved value: wid` at `✏️editor/🦀️.rs:3400`); this wave completed it with
  `wid.clone()` on the `Effect::SetActiveUtility` line above, which is the fix the compiler itself named.
- `[DEBUG] ` logs: none added. The traces read below are the shell's existing ones, armed through
  `runtimeDiagnosticsEnabled()`'s `localStorage` key rather than by editing source.

---

## 1 The traced chain, hop by hop

One toggle, `puzzle3d-play-grid-visible`, on window instance `puzzle3d-main-top`. Read live off the
running shell, not from source.

| # | Hop | file:line | Live evidence |
|---|---|---|---|
| 1 | rail checkbox | `🛠️ShellHelpers/🟦️.tsx` `windowMeasureToggleControl` → `TreeCheckbox` | `checked` flips on click (B10's `Tree` fix is live) |
| 2 | action + args, tagged with the INSTANCE | `🛠️ShellHelpers/🟦️.tsx:3208-3210` `windowMeasuresChrome`'s `taggedOnAction` | — |
| 3 | host resolves the target window and projects the view | `🏛️ShellHost/🟦️.tsx:5937-5949` (`dispatchWindowId`, `hostArmedViewContext`) | — |
| 4 | wire invocation | `🏛️ShellHost/🟦️.tsx:837-859` `windowActionInvocation` | `[DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"setGridVisible"}` |
| 5 | actor ingress, re-projected at the address | `🔌️plugin/🦀️.rs:33138` `admit_addressed_action_view` | `[DEBUG] command ingress lane {"actionId":"setGridVisible","seq":10,"lane":"Interactive"}` |
| 6 | guest resolves the instance | `✏️editor/🦀️.rs:932` `puzzle3d_addressed_window_id` | `[DEBUG] puzzle3d.utility.publish action=setGridVisible window=Some("puzzle3d-main-top")` |
| 7 | arm mutates the runtime | `🎮️commands/👁️set-visible/🦀️.rs:6-8` | — |
| 8 | `WindowConfig` mutation, addressed at the resolved instance | `✏️editor/🦀️.rs:3400` → `🪟️window/🦀️.rs:286` `addressed_config_for` | — |
| 9 | publication on that instance's own lane | `🔌️plugin/🦀️.rs:23734` `window_config_store.begin` | `[DEBUG] command ingress settled status=command-complete` |
| 10 | terminal completion carries the scope | `🏛️ShellHost/🟦️.tsx:5244` `subscribeOperationCompletions` | `[DEBUG] completion apply {"operation":128,"scope":{"kind":"full"}…}` |
| 11 | host refresh | `🏛️ShellHost/🟦️.tsx:5224` `applyHostEffects` → `refreshUi` | `[DEBUG] applyHostEffects refresh {"scope":{"kind":"full"}}` |
| 12 | refresh re-mounts the `measures` section surface | `🔌️PluginRuntime/🟦️.tsx:1608-1615` `uiRefreshSurfaceEvents` → `Event::SurfaceVisible` | — |
| 13 | guest re-renders the section from the published config | `🔌️plugin/🦀️.rs:31937` `plugin_render_section(Measures)` → `🔌️plugin/🦀️.rs:26444-26455` per-instance `window_config_store.capture` | — |
| 14 | rail re-renders | `🏛️ShellHost/🟦️.tsx:9277` `windowMeasuresChrome(windowMeasuresByWindowId[instance.id] …)` | checkbox reads `false` |

**Not one hop is broken.** Measured end to end on an idle app:

```
{ "before": true, "flippedAfterMs": 695, "now": false }
```

and confirmed on the sibling control (`vortex-show` trigger text `Selected` → `Always` after the round
trip, a fully controlled Radix `Select` whose text can only move when `measure.value` moves).

### 1.1 So why did the probe read `before == after`?

Because **nothing in the chain shows the user what they asked for while it is in flight**, and the round
trip is 0.7 s idle / several seconds on a loaded app. Reproduced live, replaying the probe's own
escalation (`🔍️browser-probe.ts` `nudgeMeasure`: click → 900 ms → Space → 900 ms → synthetic wrapper
click, then read immediately):

```
{ "before": true, "log": [["click",0],["t900",true],["space-equiv",1],["t1800",true],["wrapper",2]], "final(+4s)": false }
```

At 900 ms and at 1800 ms the control still renders the pre-click value, so the probe escalates and
dispatches two MORE toggles, then samples `after` at ~0 ms — and reads the value none of the three
dispatches has published yet.

That is not a probe artifact. It is the same thing a user sees: the checkbox ignores the click, so they
click again, and the second click is computed from a value the program has already left behind.

### 1.2 Why the two sliders "passed" all along

`grid-spacing` and `lod-value` are `WindowMeasureSlider`s, and `🎚️Slider/🟦️.tsx:120-130`
(`resolveSliderDraftClear`) has always held a **local draft** until the controlled prop catches up. Their
PASS was the draft, not the lane: the battery's own `settings-value-reaches-window-rail` line reads
`windowRail value "12.5"` = 10.0 + 5 × 0.5, i.e. exactly the five `ArrowRight`s the window-options step
had pressed, while the rail's guest-rendered LABEL stayed `Spacing 10.0` in every rail dump of that run
(`🗑️generated/battery-2026-09-11-45b-6013.txt`, `rg -c "Spacing 10.0"` → 3, `rg -c "Spacing 12"` → 0).

---

## 2 Fixes

### 2.1 The rail's toggle and select now hold the value the user asked for — HOST, live

New `🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx` (split out of `🛠️ShellHelpers/🟦️.tsx`, which cannot be
imported by a law without cycling through `🏛️ShellHost`/`🐚️Shell`):
`useWindowMeasureDraft` + `WindowMeasureToggle` + `WindowMeasureSelect`, the same discipline
`WindowMeasureSlider` already had. The draft retires the moment the program publishes **anything other
than** the value the draft was taken against — retiring on inequality rather than on a match is what
stops a draft resurrecting when the published value later returns to what it was.

**Law** `🧪️tests/🎚️window-measure-controls/🟦️.tsx` (new suite, registered in the renderer-react
`vitest.config.ts` `engineTestSuites`) — three laws: the toggle shows the asked-for state immediately and
survives a re-render at the same published value; a program that answers with a different value overrules
the draft; the select does both. Red first (the whole draft replaced by `return [published, () => {}]`),
then green:

```
      Tests  3 failed | 875 skipped (878)
```
```
 Test Files  1 passed | 23 skipped (24)
      Tests  3 passed | 875 skipped (878)
```

### 2.2 A window option no longer repaints the whole shell — GUEST, rides #46

`✏️editor/🦀️.rs`: new `Puzzle3dScopeClass::WindowOption` + `puzzle3d_window_option_scope()`. The 19
rail/Settings verbs (`setGridVisible`, `-GridSnapEnabled`, `-GridSpacing`, `-LodAutomatic`,
`-LodDepthVariable`, `-LodManual`, `-VortexShow`, `-VortexDirection`, `toggleSun`, `setSun{Azimuth,
Elevation,Intensity}`, `-SelectableKind`, `-TransformGumballFlag`, `-ProximityRadius`, `-ChunkSize`,
`-VoxelDims`, `-Projection`, `-ProjectionParam`) fell through the scope table to
`Puzzle3dScopeClass::Chrome`, i.e. `UiDirtyScope::Full` — one grid toggle re-rendered every window body
(180 Nakagin instances), every panel body and the whole label overlay before its own checkbox could move.
They now paint exactly the three surfaces they can move: the world body, the measures rail, and the
Settings panel (which mirrors four of the same fields and must never disagree with the rail).
`setProjection`/`setProjectionParam` moved off `Viewport` for the same reason — that class carries
`measures: false`, so the projection selects could never re-read their own published value.

### 2.3 `data-camera-json` reports the guest's pose, not the local rig — HOST, live

`🌐️World3dHost/🟦️.tsx:6037` was `world3dCameraDomJson(cameraState)` — `viewportCamera ?? sceneCamera`,
i.e. this component's own gesture state, which moves for every drag whether or not `setCamera` ever
reached the guest. It is now `world3dCameraDomJson(sceneCamera)` — the pose the program published on THIS
window instance's `WindowConfig` lane — with the live rig pose kept as its own
`data-viewport-camera-json`. See §4 for what that honestly measures.

### 2.4 A section and a field row carry their own DOM id — HOST, live

`🗣️Interpreter/🟦️.tsx` `ContainerView` dropped `id` for exactly two roles, `section`/`group` and `field`
— the two an app uses to author a form. Puzzle 3d's whole Settings panel is authored that way
(`ui::section(...).try_id("puzzle3d-play-settings")` over four
`ui::field(...).try_id("puzzle3d-play-settings.grid-spacing")` rows), so not one of its rows or its
section was addressable while the innermost control was. `Section` (react package `:8540`) put `id` on
both the `<section>` and its `<h2>` — the heading now derives `${id}.title` and the section
`aria-labelledby`s it. `Field` (`📝️Field/🟦️.tsx:25`) now renders `id` on its root and associates its
label with `${id}.control`, which is the id an authored `try_child(control)` gives that child.

### 2.5 A stepper's +/− reaches a program that declares only `Change` — HOST, live

`Stepper`'s own contract is *"reports a relative delta via `onDelta` when provided, otherwise falls back
to computing an absolute `onChange`"* (`🪜️Stepper/🟦️.tsx:37-38,86-93`). `NumberStepperView` supplied
`onDelta` **unconditionally**, so every +/− click went down a `delta` trigger and
`emitIntent` (`📃️UiDocumentStore/🟦️.tsx:527-531`) dropped it for any node that does not bind `delta` —
which is every one of puzzle 3d's four Settings steppers. Browser-confirmed: before the fix a real
mousedown/mouseup on `stepper-plus` produced no `performInvocation` at all; after it,

```
[DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"setGridSpacing"}
[DEBUG] puzzle3d.utility.publish action=setGridSpacing window=Some("puzzle3d-main")
```

`onDelta` is now passed only when the node declares a `delta` binding.

**Law** `🗣️Interpreter/🧪️tests/🪪️container-node-ids/🟦️.tsx` (new module, registered from the
Interpreter's existing in-source `import.meta.vitest` block): a stepper declaring only `change` sends
`change` with the absolute `10.5`; one declaring `delta` still gets `delta` with `0.5`; plus the two
container-id laws of §2.4. Red first, then green:

```
     × sends a +/− bump down the absolute `change` trigger a program that declares only `change` can actually receive
      Tests  1 failed | 1 passed | 880 skipped (882)
```
```
 Test Files  1 passed | 23 skipped (24)
      Tests  2 passed | 880 skipped (882)
```

`🖌️render.ts`'s `fireEvent` facade gained `mouseDown`/`mouseUp` — `Stepper`'s +/− is a press-and-hold
driven from those two events, never `click`, so a law about one bump has to play what a browser plays.

### 2.6 The Settings steppers render their value — GUEST, rides #46

`📌️panels/⚙️settings/🦀️.rs:28` declared `NumberStepperProps { …, uniform: false }`. `uniform: false` is
MIXED-selection: `NumberStepperView` passes `value={undefined}` and `mixed`, so the box renders a
placeholder instead of the setting AND a +/− bump is computed from the widget's `defaultValue` (0) rather
than from the value. Browser-confirmed on #45: all four boxes read `""`, and the first bump dispatched
`setGridSpacing 0.5` instead of `10.5`. These are one window's own scalars, never an aggregate:
`uniform: true`.

**Law** `✏️editor/🧪️tests/🔬️unit/🦀️.rs`
`settings_panel_steppers_carry_their_value_and_the_trigger_they_dispatch_on` — all four rows and their
controls exist by id, every control is `uniform: true` with a real `value`, declares exactly one binding,
that binding is `Change` (never `Delta`), names its own verb, and carries the `windowId` the panel was
rendered for.

---

## 3 Laws and their outputs

### Guest (`RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib … -- --test-threads=1`)

New: `window_option_toggle_round_trips_on_its_own_instance_and_leaves_the_sibling_alone` — the law this
wave was asked for: `setGridVisible {"pressed": false}` on `puzzle3d-main-perspective` advances **that**
instance's window-config generation by exactly one, comes back through that instance's own measures rail
as `false`, reaches the world body it renders (`showLodGrid: false`), and leaves `puzzle3d-main-top`'s
generation, rail and body untouched.

New: `window_option_verbs_paint_only_their_window_their_rail_and_the_settings_panel` — all 19 verbs
resolve to `WindowOption`, and that scope names the world body, `measures`, and the Settings panel body,
and never the outliner / catalogue / inspector / history / labels / utilities / tools, and is never
`Full`.

New: `settings_panel_steppers_carry_their_value_and_the_trigger_they_dispatch_on` (§2.6).

```
running 4 tests
test editor::puzzle3d::component::tests::vortex_direction_window_option_defaults_to_outwards_and_switches_to_inwards ... ok
test editor::puzzle3d::component::tests::vortex_show_window_option_defaults_to_selected_and_switches_to_always ... ok
test editor::puzzle3d::component::tests::window_option_toggle_round_trips_on_its_own_instance_and_leaves_the_sibling_alone ... ok
test editor::puzzle3d::component::tests::window_option_verbs_paint_only_their_window_their_rail_and_the_settings_panel ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 687 filtered out; finished in 0.31s
```
```
test editor::puzzle3d::component::tests::settings_panel_steppers_carry_their_value_and_the_trigger_they_dispatch_on ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 695 filtered out; finished in 0.09s
```

Neighbours, unchanged: the seven `…scope…` laws (`command_scope_classes_name_the_panels_they_change`
included) and all 19 `window_`-matching laws are green.

```
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 684 filtered out; finished in 0.97s
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 672 filtered out; finished in 2.88s
```

Whole guest lib:

```
test result: FAILED. 693 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 56.18s
```

The three are **not** this wave's and none touches a file it changed:
`open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin` (a wall-clock budget,
`worst turn 4.70ms over 2ms` on a loaded box — the long-standing flaky class),
`two_instances_converge_disjoint_object_edits_via_backbone` (`module.vcs: remote snapshot merge is
fail-closed until the app-owned streaming envelope decoder … terminal-authorized`), and
`every_advertised_engagement_verb_is_implemented` (`typing clear must empty the framework-owned
selection`, i.e. the selection/engagement lane a peer is live in).

`cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly`:

```
warning: `semio-s-artifact-puzzle-3d` (lib) generated 88 warnings (run `cargo fix …`)
    Finished `dev` profile [unoptimized] target(s) in 2.47s
```

0 errors.

### Renderer-react (`SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts`)

```
 Test Files  3 failed | 21 passed (24)
      Tests  12 failed | 870 passed (882)
```

Both new suites pass. **None of the 12 is new from this wave.** Nine are exactly B10's list (6 in
`🧪️tests/🧩️package-integration/🟦️.ts` — generated wgpu worker bytes / the Bun pin; 2 in
`🔌️PluginRuntime`; 1 engine-contract `buildNoteShellCommandAction`, a peer's added
`inverseArgs`/`inverseCommandId`). The other three are new **since B10 and from a peer, not from here**:
three `submitPluginTurn` laws in `🔌️PluginRuntime/🟦️.tsx`, whose source and test file were both written
at 17:35–17:38, i.e. two minutes before this run (`stat` quoted in the session); this wave never touched
`🔌️PluginRuntime`.

### `bun x tsc --noEmit` (renderer-react project)

Filtered to the files this wave touched, the only two hits are pre-existing and unrelated to its
insertions: `🗣️Interpreter/🟦️.tsx(1460) ImportMeta.dir` (on the file's ALREADY-EXISTING `registerTests1`
call) and `🗣️Interpreter/🧪️tests/🧪️unknown-component-placeholder/🟦️.tsx(140)`. No error in
`🛠️ShellHelpers`, `🎚️measure-controls`, `🎚️window-measure-controls`, `🪪️container-node-ids`, `📝️Field`
or `🖌️render.ts`. `🌐️World3dHost`'s three errors are B10's three, shifted by exactly the +32 lines this
wave inserted above them (`3219|4077|4414` → `3251|4109|4446`).

---

## 4 Probe verdicts, before and after

`🗑️generated/b12-window-options-6013.txt` (this wave) against
`🗑️generated/battery-2026-09-11-45b-6013.txt` (the coordinator's run of the SAME wasm #45 with B10's
host fixes already live).

| verdict | before | after | live on #45? |
|---|---|---|---|
| `window-option-puzzle3d-play-grid-visible` | FAIL `before==after` | **PASS** | host-live |
| `window-option-puzzle3d-play-grid-snap` | FAIL | **PASS** | host-live |
| `window-option-puzzle3d-play-lod-auto` | FAIL | **PASS** | host-live |
| `window-option-puzzle3d-play-vortex-show` | FAIL | **PASS** | host-live |
| `window-option-puzzle3d-play-vortex-direction` | FAIL | **PASS** | host-live |
| `window-option-puzzle3d-measure-sun-enabled` | FAIL | **PASS** | host-live |
| `window-option-puzzle3d-play-grid-spacing` / `-lod-value` | PASS (on the slider's local draft) | **PASS** (now on the lane) | host-live |
| `window-options-lane-responsive` / `-emit-no-history` | PASS | PASS | — |
| `camera-json-attribute` | PASS | PASS | host-live |
| `camera-orbit` | PASS (local state) | **FAIL, honestly** — see §4.1 | host-live |
| `camera-pan` | PASS (local state) | PASS (guest pose) | host-live |
| `camera-zoom` | PASS (local state) | **FAIL, honestly** — see §4.1 | host-live |
| `camera-per-window` | PASS | PASS | — |
| `camera-emits-no-artifact-history` | PASS on #45b / FAIL earlier | FAIL `newEntries=[…7,8,9,13]` | B10's guard **rides #46** |
| `settings-panel-opens` | PASS | PASS | B10's dock fix, host-live |
| `settings-steppers-present` | FAIL `ids=["puzzle3d-play-settings"]` | FAIL, same — see §4.2 | — |
| `settings-grid-spacing-bumps` | FAIL `plusButtons=0` | FAIL, same — see §4.2 | — |
| `settings-value-reaches-window-rail` | FAIL `settings=null` | FAIL, same — see §4.2/§5.1 | — |

Nothing in this wave is waiting on a wasm build except §2.2 (the `WindowOption` scope, a latency and
blast-radius improvement, not a correctness gate) and §2.6 (`uniform: true`). The window-option round
trip is proven on #45 as it stands.

### 4.1 What the camera verdicts now measure

They are no longer vacuous. `camera-orbit` reads

```
before={"position":[0,0,0],"target":[0,0,0],"up":null,"zoom":0,"fov":50,"projection":"perspective"}
after= {"position":[0,0,0],…}
```

— the guest's per-window camera is `Puzzle3dCamera::default()`, i.e. **all zeros**, and stays that way
until the first `setCamera` actually lands. `camera-pan`, one step later, reads a real published pose
(`position:[9.6468,-1.9008,4.6119]`), so the lane does round-trip; it is simply slower than the probe's
fixed `waitForTimeout(1200)+500` (the gesture itself is trailing-debounced by
`CAMERA_SYNC_DEBOUNCE_MS` before the round trip even starts). `camera-zoom` fails for the same reason at
1600 ms.

Two real findings fall out, both flagged and **not** fixed here:

1. **No pane's camera is guest-owned until the user moves it.** `World3dHost` auto-fits each pane locally
   (`world3dFrameCameraFromInstances`) and deliberately never dispatches a programmatic camera change
   (`🌐️World3dHost/🟦️.tsx:4752-4760`'s own docstring). So checklist §2's "the guest owns one
   authoritative pose per window instance" is true only after the first gesture. This also means the
   still-passing `window-distinct-camera` verdict in the full battery is now measuring two identical zero
   poses rather than two distinct framings — it should be re-read as unproven until a pane's fit is
   published.
2. `camera-emits-no-artifact-history` is still red on #45 (`before=3 after=7`) purely because B10's
   `dispatch_emit` guard has not been built yet.

### 4.2 Why the three §19 verdicts cannot pass as the probe is written

The Settings panel is now fully correct in the browser and fully addressable — it is the probe's
**selectors** that are stale. Read live off `:6013` after this wave:

```
["puzzle3d-play-settings",                                                       (tree row)
 "panel:puzzle3d-play-settings/puzzle3d-play-settings",                          (section)
 "panel:puzzle3d-play-settings/puzzle3d-play-settings.title",
 "panel:puzzle3d-play-settings/puzzle3d-play-settings.overlap-budget",           (field row)
 "panel:puzzle3d-play-settings/puzzle3d-play-settings.overlap-budget.control",   (stepper)
 …proximity-radius…, …chunk-size…, …grid-spacing…]
```

All four steppers are present, their `field-label`s associate with their controls, and a real
mousedown/mouseup on `stepper-plus` now dispatches `setGridSpacing`. The probe asks for
`[id^="puzzle3d-play-settings"]` and `[id="puzzle3d-play-settings.grid-spacing"]`, which predate
`uiNodeDomId`'s surface prefix — `${surface}/${key}`, introduced by 26/09/09/PROCEDURAL-3D-END-TO-END in
`f39d4b0db3` (2026-09-10) and documented as deliberate namespacing so two windows of one app cannot
collide. The probe is not editable by this wave; its §19 locators need the `panel:<bodyKey>/` prefix (or
a `[id$="…"]` suffix match), and then `settings-steppers-present` and `settings-grid-spacing-bumps` read
the panel that is actually there.

`settings-value-reaches-window-rail`'s `windowRail value "12.5"` is now a REAL published value (10.0 + 5
× 0.5 from the preceding window-options step), not a draft.

---

## 5 Flagged, not fixed

### 5.1 A Settings edit lands on `puzzle3d-main`, which is not a pane anyone is looking at

Live: `[DEBUG] puzzle3d.utility.publish action=setGridSpacing window=Some("puzzle3d-main")`. A peer's
`stepper_window_args` now stamps the panel's own `window_id` into every stepper's args — but the panel is
rendered under `panelViewContext` (`🛂️manifest/🟦️.ts:1219-1221`), which strips `windowId` by design, so
`puzzle3d_addressed_window_id` falls through to the roster's first entry, and
`sessionWindowInstances` puts the base window KIND (`puzzle3d-main`) ahead of the two open instances
(`-top`, `-perspective`). Because the guest now bakes a `windowId` into the args, ShellHost's own
`actionWindowId ?? activeWindowIdRef.current` fallback (`🏛️ShellHost/🟦️.tsx:5938`) never gets to pick
the focused pane either. This is A1/A2 §19's "verify in a split-window test" ambiguity, now with a live
reading. The honest fix is a framework decision — either the host publishes the active window instance
into the panel view context, or a per-window setting stops living in an app-level panel — and
`📌️panels/⚙️settings/🦀️.rs` has a peer live in it, so it was left alone.

### 5.2 `record_command` folding, and `Puzzle3dScopeClass::Chrome` as the table's default

B10 already reported that `push_log_entry` always pushes `count: 1` so the promised consecutive-View
folding is not implemented. Related: the scope table's `_ => Chrome` default is what let 19 rail verbs
repaint the whole shell for years without anyone noticing; the `command_scope_classes_name_the_panels_they_change`
law only constrains `ActionKind::Mutation` verbs, so a `View` verb can still fall through silently. Worth
a law that every DECLARED window-kind action id appears in the table explicitly.

### 5.3 `ContainerView`'s remaining wrapper roles

Only `section`/`group` and `field` were fixed. Any other wrapper role added later will drop its id the
same way unless the id is threaded at the top of `ContainerView` rather than per branch.

---

## 6 Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🎚️measure-controls/🟦️.tsx` — NEW: `useWindowMeasureDraft`, `WindowMeasureToggle`, `WindowMeasureSelect`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` — the two control functions now delegate to that module.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎚️window-measure-controls/🟦️.tsx` — NEW suite (3 laws).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts` — registers that suite.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` — section/field DOM ids; `onDelta` only when declared; registers the new test module.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🪪️container-node-ids/🟦️.tsx` — NEW (4 laws).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` — `data-camera-json` = the guest pose; new `data-viewport-camera-json`; docstring.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` — `Section` heading id / `aria-labelledby`.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📝️Field/🟦️.tsx` — root id, label association, prop docstring.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🖌️render.ts` — `fireEvent.mouseDown`/`mouseUp`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — `WindowOption` scope class + table; `wid.clone()` completing a peer's hunk.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/⚙️settings/🦀️.rs` — `uniform: true`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — 3 new laws.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🗑️generated/b12-window-options-6013.txt` — the probe run quoted above.
