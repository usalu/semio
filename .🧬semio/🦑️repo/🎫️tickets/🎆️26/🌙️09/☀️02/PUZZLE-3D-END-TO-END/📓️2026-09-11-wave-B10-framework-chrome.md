# Wave B10 — Framework / Host Chrome Layer

Implementation pass, 2026-09-11. Eight defects from `📓️2026-09-09-user-feature-checklist.md` §1–§4, §14,
§17, §19, §25, as handed over by the coordinator and evidenced by
`📓️2026-09-11-wave-B1-battery-extension.md` §5 and `📓️2026-09-11-audit-A2-unproven-sections.md`.

Every command tail quoted below is real output from this pass.

**Probe status**: `🔍️browser-probe.ts` could NOT be run in this pass. `:6013` never answered a request
during the whole wave (`curl … --max-time 60` → `6013:000 time=60.009049`; the bound `vite` pid 26789 sat
at ~94 % CPU), and from 16:20 a peer's `bun 🔍️browser-probe.ts --battery --reload-between-groups
--port=6013` held the one-tab lease. So every probe verdict below is quoted as **before** only, from B1's
run; the **after** column names the lane that must be re-run. Host-side fixes (1, 2-host, 4, 5, 8-host)
are vite-live and show on a reload; the guest-side half of 2 (measure ids/default-open live in
`🔌️plugin/🦀️.rs`, which the wasm component links) rides the next component build.

---

## 1. §4 — Tree row checkboxes could never toggle — FIXED

**Root cause** `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:609-617` (before the fix):

```tsx
<label data-slot="tree-action-checkbox-wrapper" … onClick={(event) => { event.preventDefault(); event.stopPropagation(); }}>
  <input data-slot="tree-action-checkbox" type="checkbox" … />
```

`preventDefault()` on a bubbled checkbox click runs the HTML *legacy-canceled activation behavior*: the
checkedness the pre-click steps already applied is reverted and no native `change` fires. It hits a
pointer click on the input, a pointer click on the label, and the keyboard Space key alike. Framework-wide
— it is every `WindowMeasure::Toggle` in every app's measures rail
(`🛠️ShellHelpers/🟦️.tsx:3025-3036`, `windowMeasureToggleControl`), i.e. `puzzle3d-play-grid-visible`,
`-grid-snap`, `-lod-auto`, `-lod-depth-variable`, `-select-objects/-vortices/-attractions`.

**Fix** the wrapper now only stops propagation (so the enclosing tree row still does not select), never
cancels the default; `onChange` additionally short-circuits on `disabled`. Docstring records the spec
reference.

**Law** `🌳️Tree/🧪️tests/🧩️component/🟦️.tsx`, new `☑️CheckboxActivation` region — five laws: one
activation for a click on the input, one for a click on the wrapping label (this one was red before the
fix), one for the keyboard Space key with focus retained, a disabled checkbox stays inert and never
reaches the enclosing row (also red before the fix), and the control keeps an explicit accessible name.

Red first, then green:

```
 ❯ |@semio-tech/ui-react| ../../../../🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx (8 tests | 2 failed) 992ms
     × reports exactly one activation for a pointer click on the wrapping label 3ms
     × keeps a disabled checkbox inert and never reaches the enclosing row 63ms
 Test Files  1 failed (1)
      Tests  2 failed | 6 passed (8)
```

```
 Test Files  1 passed (1)
      Tests  8 passed (8)
   Duration  4.09s
```

**Probe** before: `window-option-puzzle3d-play-grid-visible FAIL before={…"checked":true…} after={…"checked":true…}`
(same for `-grid-snap`, `-lod-auto`). After: re-run
`--only=window-option-puzzle3d-play-grid-visible,window-option-puzzle3d-play-grid-snap,window-option-puzzle3d-play-lod-auto`.
Host-live. **Caveat, stated plainly**: this fix restores the *gesture*. The probe reads `checked` back
from a controlled input bound to `runtime.grid_visible`, so it only flips once the guest's WindowConfig
publication round-trips — see §3 below for why that is a separate, still-open question.

## 2. §3/§4 — projection measures and the sun group render nowhere — FIXED (two real causes)

Neither group was missing from the published `measures` list. `main/🦀️.rs:67-80` appends
`options::projection::measure(...)` and `options::sun::measure(...)` unconditionally, both delegating to
the framework builders, and `partition_window_measures`
(`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧩️component.rs:1157`) cannot drop them —
it only re-routes groups carrying `active_utility_id`, and `measure_group_with_open`
(`🔌️plugin/🦀️.rs:34314`) always sets that to `None`. Two separate real defects instead:

**2a — the projection family had no `-measure-` segment.** Every id was `<prefix>-projection…`
(`🔌️plugin/🦀️.rs:34578-34652`), while the sibling family is `<prefix>-measure-sun…`. Nothing in the repo
depended on the old spelling (grep for `projection-orthographic-view` / `projection-perspective-kind` /
`projection-axonometric-variant` returned only the definition site), so all 23 occurrences are renamed to
`{id_prefix}-measure-projection…`. That is the spelling checklist §3 and the probe both address
(`[id^="puzzle3d-measure-projection"]`).

**2b — both group roots were `default_open: Some(false)`.** `WindowMeasureTreeGroup`
(`🌳️Tree/🟦️.tsx:4136-4196`) renders **no children at all** for a collapsed group — not hidden by CSS,
simply unmounted — so a closed-by-default root put `puzzle3d-measure-sun-enabled` and every projection
leaf outside the DOM. Every sibling group is open (`☑️options/🔭️lod/🦀️.rs:16`, `🌐️grid/🦀️.rs:11`,
`🎯️select/🦀️.rs:11` are all `Some(true)`); Sun and Projection were the only two that were not. Both are
now `Some(true)`.

**Law** `🔌️plugin/🧪️tests/🔬️world3d-host-unit/🦀️.rs`,
`projection_and_sun_measure_families_are_addressable_and_open_by_default` — walks the built measure tree,
descending only into groups that are not `Some(false)` (exactly what the renderer mounts), and asserts the
sun enable toggle, the three sun sliders, the parallel group, the orthographic-view select and the
perspective-kind select are all reachable, and that every projection id shares the `-measure-projection`
prefix.

```
running 1 test
test component::world3d_host::tests::projection_and_sun_measure_families_are_addressable_and_open_by_default ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 655 filtered out; finished in 0.00s
```

**Probe** before: `projection-measures-present FAIL ids=[]`,
`window-option-puzzle3d-measure-sun-enabled FAIL control absent from the measures rail even after re-unfolding it`.
After: re-run `--only=projection-options,window-options`. **Rides the next wasm component build** — the
ids and the default-open flags are compiled into the guest-linked `semio-framework-plugin`.

**Residual, flagged not fixed**: `world3d_sun_measures` / `world3d_projection_measures` hard-code English
labels ("Sun", "Enabled", "Azimuth", "Projection", "Parallel", "Orthographic", …). That breaks the
no-default-language rule. Fixing it means giving both framework builders a locale/label parameter and
updating four plugin call sites (lowpoly, cad, puzzle5d, puzzle3d) — a cross-plugin refactor outside this
wave's blast radius, and worth its own ticket item.

## 3. §4 — vortex show / direction selects no-op — NO HOST DEFECT FOUND; one real a11y defect fixed

Traced end to end; every hop is correctly wired, with no key mismatch and no dropped `windowId`:

- guest declares: `☑️options/🌀️vortex/🦀️.rs:11-21,24-34`, `WindowMeasure::Select { id:
  "puzzle3d-play-vortex-show"/"-direction", value: runtime.vortex_show/vortex_direction, on_change:
  puzzle3d_action("setVortexShow"/"setVortexDirection", None) }`
- host renders and dispatches: `🛠️ShellHelpers/🟦️.tsx:3009-3023` — `onValueChange` spreads
  `args: { …, value }`; the framework `Select`'s `selectValue` (`🔽️Select/🟦️.tsx:296-305`) fires it
  whenever the picked value differs from the controlled one. No `preventDefault` wrapper here — the
  Tree-checkbox defect of §1 does not apply to selects.
- `windowId` is stamped by `taggedOnAction` (`🛠️ShellHelpers/🟦️.tsx:3208-3210`), read back at
  `🏛️ShellHost/🟦️.tsx:5932-5933`, and resolved in the guest by `puzzle3d_addressed_window_id`
  (`✏️editor/🦀️.rs:911-918`).
- guest handlers read `args.get("value")` and write `ctx.scene.runtime.*`
  (`🎮️commands/🌀️set-vortex-show/🦀️.rs:6-11`, `🧭️set-vortex-direction/🦀️.rs:6-11`).

The guest half already has laws, and they are green in current source:

```
test editor::puzzle3d::component::tests::vortex_direction_option_is_local_to_the_window_instance ... ok
test editor::puzzle3d::component::tests::vortex_direction_window_option_defaults_to_outwards_and_switches_to_inwards ... ok
test editor::puzzle3d::component::tests::vortex_show_window_option_defaults_to_selected_and_switches_to_always ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 685 filtered out; finished in 0.33s
```

So the surviving suspect is the shared **WindowConfig publication/refresh round-trip**, not the select.
That is the same lane as §1's toggles — and it is the one place B1 already caught a hard fault
(`setGridSpacing` → `SemioFaultError: fixed typed-operation and segmented-output authorities did not
pre-admit the exact operation slot`, B1 §4.3). Note that the camera "works" in the browser only because
`data-camera-json` mirrors `World3dHost`'s own local `cameraState`, so it never proved the lane round-trips
at all. One silent-drop worth someone's attention: `dispatch_step` builds the window-config mutation as
`view_state.and_then(|view| window_ownership::addressed_config(view, window_after).ok()).into_iter().collect()`
(`✏️editor/🦀️.rs:3329`) — a `None` window id there discards the whole mutation with no fault.

**Fixed here anyway** — a real accessibility defect on the same control: the vortex measures declare
`label: None`, so `WindowMeasureTreeLeaf` renders no visible label and the `Select`'s
`aria-labelledby` fell through to `context.labelId === undefined`. Both combobox triggers reached
assistive technology unnamed. `windowMeasureSelectControl` now passes
`aria-label={uiDataLabel(measure.label ?? measure.id)}` (`🛠️ShellHelpers/🟦️.tsx:3008-3012`).

**Probe** before: `window-option-puzzle3d-play-vortex-show FAIL before={…"text":"Selected"} after={…"text":"Selected"}`,
same for `-vortex-direction`. After: re-run `--only=window-options` once the WindowConfig lane question is
settled — this wave did not close it.

## 4. §19 — no `puzzle3d-play-settings` panel tab — FIXED

**Root cause** `🏛️ShellHost/🟦️.tsx`, `defaultDock`: `top-left` and `top-right` were built from
`session.app.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === …)` (`:7793-7795`, `:7845-7847`),
but `bottom-left` was framework-only (`createFrameworkDisplayPanelTabs` + the sync leaf) and `bottom-right`
was the literal `[frameworkSettingsTab, frameworkMarketplaceTab]`. So **every app-declared `Display`- or
`Settings`-group panel tab was dropped before a dock node existed.**

The guest side was never wrong: `📌️panels/⚙️settings/🦀️.rs:15,20-22` declares
`PanelTabDefinition { kind: App("puzzle3d.panel.settings"), group: PanelGroup::Settings, body_key:
Some("puzzle.3d.play.settings") }`, registered by `.panel_tab_def(settings_panel::definition())`
(`✏️editor/🦀️.rs:8195`) exactly like catalogue and inspection, and its body is rendered
(`✏️editor/🦀️.rs:8033`). `flattenPanelTabLeaves` (`🏛️ShellHost/🟦️.tsx:4303`) flattens *all* declared
tabs into `buildUiRefreshRequest`, so the shell asked the guest to render the section and cached the
result in `panelUiByKey` — then never mounted it. `PanelGroup::anchor()`
(`🛂️manifest/🦀️.rs:3142-3149`) and its TS mirror `panelAnchorForGroup`
(`🛠️ShellHelpers/🟦️.tsx:1277-1283`) both already said `settings → bottom-right`.

**Fix** one `appTabsForBottomAnchor` callback plus `displayBottomLeftTabs` / `settingsBottomRightTabs`
memos beside `detailsRightTabs`, both spread into their anchors, both added to `defaultDock`'s dependency
array. The app's own settings lead the bottom-right anchor, with the shell-wide Settings branch and the
Marketplace behind them — the same "document first, shell chrome last" ordering the bottom-left anchor
already uses for its sync leaf. That also disambiguates the probe's `openPanel(/settings/i)`, which
otherwise stops at the framework tab. Note `createFrameworkSettingsPanelTab`
(`📌️ChromePanels/🟦️.tsx:1032-1121`) is a fixed General/Theme/Keybindings list — `framework.settings`
opening successfully never had anything to do with the app's own section.

**Law** `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`,
`anchors every declarable panel group on one of the four corners the dock fills from the session's own
panel tabs` (`panelAnchorForGroup` is now re-exported from the renderer-react barrel for it):

```
 Test Files  1 passed (1)
      Tests  1 passed | 524 skipped (525)
```

**Probe** before: `settings-steppers-present FAIL ids=[]`, `settings-grid-spacing-bumps FAIL plusButtons=0`,
`settings-value-reaches-window-rail FAIL settings=null windowRail=null`. After: re-run
`--only=settings-panel`. Host-live. The stepper's clickable id keeps the `.control` suffix from
`stepper_field` (A2 §19), which the probe's `plus` locator already handles by walking the field's parent.

## 5. §14 — the engagement pane never renders — FIXED (two causes)

**5a — `Pane` never rendered its own `id`.** `PaneProps.id` (react-package `:9357-9359`) was read only to
derive fallback `chromeToggleId`/`foldControlId`, both of which `Window` overrides explicitly; the pane's
root `<div>` (`:9502-9530`) never spread it. So
`document.getElementById("framework.window.puzzle3dMainPerspective.engagement")` returned `null`
unconditionally, for every pane, in every fold state. Fixed: the root div now carries `id={id}`, and the
prop's docstring says it is both the container id and the stem of the derived chrome ids.

**5b — the engagement bar is one affordance with two independent folds.** `🪟️Window/🟦️.tsx` rendered a
top-left Engagement pane (`:356-374`, folded by `actionsFolded`) and a top-middle Search pane
(`:375-399`, folded by a private `searchFolded` that starts `true`, `:178`). The typed command line —
`<Input id="puzzle3d-engagement">` with the verb placeholder — lives only in the Search pane's body, and
`Pane` renders `body={!effectiveFolded ? children : undefined}`, so unfolding "Actions" left the bar's only
input unmounted. The guest publishes the input correctly and always
(`🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:605-621`), and `windowEngagementToSearchSpec`
(`🛠️ShellHelpers/🟦️.tsx:1689-1712`) returns a real spec for it.

**Fix** `searchFolded` is gone. Both panes derive from one `actionsFolded`, and either toggle drives
`setEngagementBarFolded`, which also focuses the command input when it expands (the focus the search
pane's own toggle used to do alone). The "type anywhere to reveal the command line" keydown and the
measures-expanded collapse effect both drive the same single state.

**Law** `🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` — the two existing Window laws are
updated to the new contract (they were the deliberate old two-fold laws, so they are rewritten, not
worked around): clicking `…engagement.toggle` now reveals the status readout **and** the placeholder input
together and both body slots, clicking `…search.toggle` folds both back, and the second law additionally
asserts both pane containers resolve by their own DOM id (5a).

```
 Test Files  4 failed | 18 passed (22)
      Tests  4 failed | 702 passed (706)
```

The 4 remaining ui-react failures are **not** from this wave — `package entry and self-alias resolve to the
canonical React source`, `UIDialog … nested owned kind picker` (`expected last vi.fn() call to have been
called with [ { name: 'Map C' } ]`), `UIIntroduction … glass boxes` (a stylesheet-text assertion), and
`UiDriver … catalogue transfer on the move handle only` (an attribute-order assertion on
`tree-item-row` that flipped between two runs in this session, i.e. live peer churn). The two Window laws
that this wave did own were red before the rewrite and are green after.

**Probe** before: `engagement-input-present FAIL toggle=1 input=0 placeholder=null`,
`pane={"rootPresent":false,…,"fields":["?|file|"]}`. After: re-run `--only=engagement-bar`. Host-live.

## 6. §17 — outliner Hide is a no-op — NO SOURCE DEFECT; pinned with a law

The guest is already correct and this pass proves it end to end rather than by reading.
`📌️panels/🗿️artifact/🦀️.rs:126-149`: `flag_args(entity, id, flag, value)` is called with `!hidden` /
`!locked` from `hide_lock_actions`, and `set_selection_flag`
(`🎮️commands/🔖️set-selection-flag/🦀️.rs:9-25`) takes the explicit `(entity, ids)` arm for a row action,
so it patches the row's own object rather than the live selection. `World3dHost` has no `hidden` field on
`WorldInstanceRecord` by design — a hidden object keeps its array index and renders at zero scale
(`🧊️main/🦀️.rs:125-135`), which is what keeps `worldPick`'s index-addressed picking stable.

The `flag_args` `value` parameter was added in `ebbace9b32`; the probe run that produced
`outliner-hide-applies FAIL` was driven against the `:6013` serve that `📓️2026-09-11-audit-A5-peer-map.md:127`
records as "stale/background instance, up 1d1h", i.e. a wasm bundle older than the fix.

**Law** (new) `✏️editor/🧪️tests/🔬️unit/🦀️.rs`,
`outliner_row_hide_and_show_round_trip_through_their_own_declared_args` — renders the outliner body, walks
the built UI for the hidden-flag action the row itself declares, asserts it asks for the inverse of the
state it renders, dispatches **those** args, and repeats for the un-hide. Driving the panel's own args is
the point: the original defect was a wrong `value` baked into them, which a hand-written literal would
never catch.

```
test editor::puzzle3d::component::tests::outliner_row_hide_and_show_round_trip_through_their_own_declared_args ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 688 filtered out; finished in 0.27s
```

**Probe** before: `outliner-hide-applies FAIL beforeHead=[…"Hexagonal Cut Concrete Forest Left Hide Lock"…] afterHead=[…same…]`.
After: re-run `--only=outliner-rows` against a **rebuilt** serve; a re-run against the stale one proves
nothing.

## 7. §25 — switching to German empties the outliner — NO TRANSLATION DEFECT; root cause is actor loss

`🗣️terminology/🦀️.rs:8-109` authors all four cells for all ~90 labels (`objects: native_en "Objects",
native_de "Objekte", reuse_en "Building components", reuse_de "Baukomponenten"`), and the `app_labels!`
expansion (`🔌️plugin/🦀️.rs:6177-6214`) is an exhaustive 4-arm match over `(Terminology, Locale)` — it
cannot compile with a missing cell and has no empty-string path. `puzzle3d_labels` fails **closed**
(`✏️editor/🦀️.rs:8049`, `ui.localization.unsupported`), never silently to English, which is what the
no-default-language rule requires. The DE law is green in current source:

```
test editor::puzzle3d::component::tests::document_and_kinds_trees_use_german_reuse_section_labels ... ok
```

It asserts `Baukomponenten` / `Verbindungen` / `Referenzen` / `Zielvolumina` in the document tree,
`Kabel` / `Verbindungen` in the catalogue, and that the English strings do **not** leak.

In B1's browser run the panel's own fixed section headers went missing too — not just the item rows —
i.e. the whole body build returned nothing, and the same step logged
`pageerror: Error: actor-activation.revoked` and `pageerror: Error: plugin-handle.closed`. That is this
codebase's signature for a torn-down guest actor, and it is why the subsequent
`locale-switch-back-en FAIL switched=false` could not find `#framework.settings.language` either. So §25
reads as "the panel query landed on a dying actor", not "German resolves empty". Two follow-ups belong to
whoever owns actor stability, not to this wave: the `registerBrushMesh` retry storm visible in the same
console tail, and the fact that a panel body query answered by a revoked actor renders *empty* instead of
retrying or surfacing a fault.

**Probe** before: `locale-flips-document-labels FAIL en=OBJECTS | … de=`, `locale-de-document-section-label FAIL de=`,
`locale-no-english-leak FAIL de=`. After: an **isolated** `--only=locale-switch` run against a freshly
rebuilt serve, so the verdict is not contaminated by the preceding steps' fault storm.

## 8. §2 — camera must not add an artifact history entry — FIXED

**Root cause A (docstring)** `🌐️World3dHost/🟦️.tsx:4756-4758` justified the debounced dispatch with
"so the shell-side command-history panel has something to show" — flatly contrary to the per-window
session-only view-state contract (`🎚️config/🦀️.rs:60-63`: *"Session-only per-window viewport camera —
never a document field … must never create a VCS edit"*). Rewritten: the dispatch exists so the guest owns
one authoritative pose per window instance (every camera-dependent projection is re-derived from it, and a
reattaching pane adopts it instead of snapping back), and it states that `setCamera` is `ActionKind::View`
on the WindowConfig lane only.

**Root cause B (the entry itself)** `🔌️plugin/🦀️.rs`, `dispatch_emit`: when `artifact_mutations` is empty
it unconditionally called `record_command(verb, kind, …)`, so every debounced `setCamera` — and every
`setGridVisible`/`setVortexShow`/… — appended its own `framework.history.entry.<seq>` row. (The
`record_command` docstring promises consecutive identical `View`/`Shell` dispatches fold into one `count`
row; `push_log_entry` always pushes `count: 1`, so that folding is *also* not implemented — which is why
B1 saw 3 rows for 3 gestures rather than one ×3 row. Reported, not fixed here.)

**Fix** a `published_window_config` flag captured before the window-config dispatch loop, and the log call
guarded:

```rust
if !(published_window_config && config_edit_id.is_none() && matches!(kind, ActionKind::View)) {
    self.record_command(verb, kind, description.clone(), None, config_edit_id, None).await;
}
```

A `View`-kind dispatch whose ONLY emission is the per-window config lane is one window's session view
state — its own store, its own lane, nothing revertible — so it is not a command-history row. The
predicate is deliberately narrow: a `View` action that reaches the document or the shared config store
still logs, which is exactly the shape of the existing `select` laws
(`an_op_less_view_action_is_logged_with_edit_id_none_and_count_one` asserts `config_edit_id.is_some()`,
"select is a config-op emission"), so those three deliberate laws keep their contract.

**Law — NOT ADDED, and here is why.** The six laws that pin this behaviour
(`an_op_less_view_action_…`, `consecutive_identical_view_dispatches_…`,
`view_dispatches_remain_distinct_…`, `view_action_emits_no_operations`,
`view_action_emitting_ops_is_rejected`, `view_action_with_inverse_is_revertible_…`) are **already red in
current source for an unrelated reason** — `TestApp::<false>` returns no tool proofs
(`🧪️tests/⏳️completion/🦀️.rs:5-8`, `if !RETAINED { return Vec::new(); }`), so every `dispatch_typed`
faults before it reaches `dispatch_emit`:

```
thread '…::an_op_less_view_action_is_logged_with_edit_id_none_and_count_one' panicked at …:4234:94:
select: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), …, message: "typed command 'select' has no exact controller/owner/factory/tool/schema proof", … }
test result: FAILED. 0 passed; 6 failed; 0 ignored; 0 measured; 650 filtered out; finished in 0.05s
```

That is an admission failure raised well upstream of the single guard this wave added, so the change
cannot be its cause — but it does mean I cannot claim a green law for it, and I will not. Writing a new
law needs a `WindowConfigOwner` fixture on the contract test's `TestApp` (State + Mutation + descriptors +
store owners + preparation factory + disposer, ~100 lines of boilerplate) *and* the missing-factory
regression fixed first. Handing that on rather than asserting an unverified law.

**Probe** before: `camera-emits-no-artifact-history FAIL historyEntries before=2 after=5`;
`window-options-emit-no-history` is scored the same way. After: re-run
`--only=camera-gestures,window-options`. Rides the next wasm component build (the guard lives in the
guest-linked runtime).

---

## Verification runs

- **ui-react vitest** (Tree + Window + Pane): `4 failed | 702 passed (706)`; all four attributed above to
  peer churn / pre-existing, none in a file this wave touched.
- **renderer-react lane**, `SEMIO_TEST_LEVEL=long bun x vitest run --config
  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts`:

  ```
   Test Files  3 failed | 20 passed (23)
        Tests  9 failed | 861 passed (870)
  ```

  None new from this wave: 6 in `🧪️tests/🧩️package-integration/🟦️.ts` (generated wgpu worker bytes / Bun
  pin), 2 in `🧱️elements/🔌️PluginRuntime/🟦️.tsx`, and 1 in engine-contract —
  `buildNoteShellCommandAction`, whose diff is a peer's newly added `inverseArgs`/`inverseCommandId`
  (`+ "inverseArgs": { "windowId": "w1" }, + "inverseCommandId": "shell.windowClose"`). This wave's own
  engine-contract law passes.
- **`bun nx run @semio-tech/framework-renderer-react:typecheck`**: fails only on peer churn. Filtered to
  the files this wave touched, every reported error predates it — `World3dHost/🟦️.tsx(3219|4077|4414)`,
  `ShellHost/🟦️.tsx(1898|7759|7760)`, `⚛️react/🟦️.tsx(10870|10926)` `ImportMeta.dir` — all at lines below
  or unrelated to this wave's insertions; the bulk (`🧪️tests/🧪️docklayoutstore/🟦️.ts`, ~70 errors) is a
  peer's in-flight refactor. No new error in `🌳️Tree`, `🪟️Window`, `🛠️ShellHelpers`, or the new memos.
- **guest**: `RUST_MIN_STACK=134217728 cargo check -p semio-s-artifact-puzzle-3d --features
  component-app-assembly` →

  ```
  warning: `semio-s-artifact-puzzle-3d` (lib) generated 88 warnings
      Finished `dev` profile [unoptimized] target(s) in 0.91s
  ```

  0 errors. Targeted `cargo test` tails are quoted per defect above.
- **probe**: not runnable this pass (see the note at the top).

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` — `TreeCheckbox` no longer cancels its own
  activation; `disabled` guard; spec-referencing docstring.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx` — new `☑️CheckboxActivation`
  region (5 laws).
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` — `Pane` renders its own
  `id`; `PaneProps.id` docstring.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx` — one fold state for the whole engagement bar;
  focus the command line on expand.
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` — two Window laws
  rewritten to the single-fold contract, plus the pane-container-id assertions.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` —
  `appTabsForBottomAnchor` + `displayBottomLeftTabs`/`settingsBottomRightTabs`, both spread into
  `defaultDock`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` —
  accessible name on measures-rail selects.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` —
  re-export `panelAnchorForGroup`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — panel
  anchor law.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` —
  `dispatchWorldCameraDebounced` docstring corrected to the session-only view-state contract.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — window-config-only `View` dispatches log no
  command row; `world3d_projection_measures` id family renamed; Sun and Projection groups open by default.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️world3d-host-unit/🦀️.rs` — measure-family
  addressability law.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` —
  outliner hide/show round-trip law.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️2026-09-11-wave-B10-framework-chrome.md` —
  this report.

## Handover

1. **The WindowConfig publish/refresh round-trip is the one unresolved blocker** behind §4's toggles and
   selects (defects 1 and 3). The gestures and the guest handlers are both proven; what is unproven is
   that a changed `Puzzle3dWindowConfig` gets back into the rail. Whoever picks it up should start from
   B1's `setGridSpacing` hard fault and from `✏️editor/🦀️.rs:3329`'s silent `.ok().into_iter()` drop, and
   should not treat `data-camera-json` moving as evidence the lane works.
2. **`record_command`'s promised `View`/`Shell` folding is not implemented** — `push_log_entry` always
   appends `count: 1`. The docstring at `🔌️plugin/🦀️.rs:20743-20750` and the `count` field's own doc
   describe behaviour that does not exist.
3. **`TestApp::<false>` has no tool proofs**, so six deliberate history laws in
   `🔬️plugin-runtime-plugin-builder-contract` fault at admission with `interactive-job.missing-factory`.
   Nothing in the history lane can be re-pinned until that is fixed.
4. **`world3d_sun_measures` / `world3d_projection_measures` hard-code English labels** — a real
   no-default-language violation, needing a locale parameter and four plugin call-site updates.
5. **Re-run the probe against a rebuilt serve.** `:6013` was unresponsive for this entire wave and the
   run B1's verdicts come from was against a day-old bundle; §17 and §25 in particular cannot be judged
   against it.
