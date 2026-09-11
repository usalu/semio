# Wave B1 — Battery Extension to Every Checklist Section

Implementation pass, 2026-09-11. Extends `🔍️browser-probe.ts` so every section of
`📓️2026-09-09-user-feature-checklist.md` is driven against the live release serve
(`http://127.0.0.1:6013/?plugin=puzzle3d`, vite dev server from source, wasm #43), following
`📓️2026-09-11-audit-A4-battery-coverage.md`.

All verdict lines quoted below are real output from runs made during this pass. Screenshots, prose
timelines and the new NDJSON streams are in `🗑️generated/probe-2026-09-11T12-*`.

---

## 1. Host change — `data-camera-json` / `data-target-volumes-json`

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`

Two additions, both on the existing root div next to `data-instances-json`:

- `data-target-volumes-json={scene.targetVolumesJson ?? undefined}` — pure pass-through of a string the
  component already holds (`scene.targetVolumesJson`, parsed at `:4516`). Zero cost.
- `data-camera-json={world3dCameraDomJson(cameraState)}` — new exported pure helper placed next to the
  other `WorldViewportCamera` helpers. It rounds `position`/`target`/`up`/`zoom`/`fov` to 4 decimals so an
  unchanged pose re-renders to a byte-identical string (no attribute churn), and emits `projection` as the
  family string. It reads the same `cameraState` the rig already consumes, so it refreshes exactly when the
  camera state changes (`adoptViewportCamera` → `setViewportCamera`, `:4670`).

The helper is re-exported from the renderer-react barrel
(`📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`) and covered by a new engine-contract test.

**Test** — `🧪️tests/🔬️engine-contract/🟦️.ts`, `"mirrors the world-3d camera pose into a stable rounded dom
json attribute"` (asserts rounding stability, that each of position/target/zoom changes the string, and
that a missing `up` serializes as `null`):

```
> nx run @semio-tech/framework-renderer-react:test-long ../../../../🧪️tests/🔬️engine-contract/🟦️.ts --run --testNamePattern=stable rounded dom json
 Test Files  1 passed (1)
      Tests  1 passed | 519 skipped (520)
   Duration  5.10s
 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react
```

**Typecheck** — `bun nx run @semio-tech/framework-renderer-react:typecheck` fails, but only on a peer's
concurrent churn in an unrelated file; zero errors in World3dHost, the barrel, or the engine-contract suite:

```
../../../../../../../../../🧪️tests/🧪️docklayoutstore/🟦️.ts(1013,25): error TS2315: Type 'Invariant' is not generic.
… (11 errors, all in 🧪️tests/🧪️docklayoutstore/🟦️.ts)
```

**Runtime confirmation** — both attributes are live on both windows:

```
windows: [{"id":"puzzle3d-main-top",…,"camera":"{\"position\":[5.4054,2.3406,8.2583],\"target\":[5.4054,2.3406,1.5015],\"up\":[0,1,0],\"zoom\":6.9694,\"fov\":50,\"projection\":\"orthographic\"}","cameraAttr":true,"volumesAttr":true,"instances":1,…},
          {"id":"puzzle3d-main-perspective",…,"camera":"{\"position\":[9.6468,-1.9008,4.6119],\"target\":[5.4054,2.3406,1.5015],\"up\":[0,0,1],\"zoom\":1,\"fov\":50,\"projection\":\"perspective\"}","cameraAttr":true,"volumesAttr":true,"instances":1,…}]
```

---

## 2. Probe infrastructure changes

### 2.1 Gating fixed, `--only=` added

The audit's §0 finding is fixed: `--battery` no longer skips the smoke steps. Every step is now registered
through one `add(name, section, group, gate, run)` call, and `--battery` is OR'd into every gate, including
`--fill` (§12) and `--marquee`, which were previously battery-exempt.

`--only=a,b` overrides every other gate and runs exactly the named steps in plan order. This is what makes
targeted re-runs cheap — every run below used it.

### 2.2 Blast-radius grouping + `--reload-between-groups`

Steps carry a group: `read` (nothing mutates the document) → `mutate` (reversible document edits) →
`replace` (the document itself is swapped). The runner executes groups in that order, registration order
within a group. `--reload-between-groups` reboots the page between groups through the same `waitForBoot`
the cold start uses, and scores the reboot as its own verdict. Verified:

```
[7.1s] plan: [{"group":"read","steps":["window-content"]},{"group":"mutate","steps":["volume-brush"]},{"group":"replace","steps":[]}]
[12.4s] reloading page before group mutate
[16.0s] reboot:mutate booted: windows=[{"id":"puzzle3d-main-top",…},{"id":"puzzle3d-main-perspective",…}] canvases=2
[16.2s] verdict reboot-mutate PASS
```

### 2.3 NDJSON verdict stream

`🗑️generated/probe-<stamp>.ndjson`, one record per verdict plus a final summary record. Join key is
`[section, step, verdict]`, exactly the diff key the audit asked for:

```json
{"t":13,"ts":"2026-09-11T12:58:55.968Z","section":"§0","step":"boot","verdict":"boot","ok":true,"status":"PASS","note":"windows ready","faults":0,"hardFaults":0,"collateralFaults":0,"distinctFaults":0}
{"t":13,"ts":"2026-09-11T12:58:55.969Z","summary":"battery PASS=3 FAIL=0 FAULTS=0","pass":3,"fail":0,"faults":0,"hardFaults":0,"collateralFaults":0,"distinctFaults":0,"steps":[]}
```

The literal `battery PASS=n FAIL=n FAULTS=n` line is also logged to stdout and into the prose `.md`.

### 2.4 Fault classification

`FAULT_RE` is unchanged. Every match is now routed through `noteFault`, which splits it into
`hardFaults` (`HARD_FAULT_RE`: worker fault, `unreachable`, `[semio-plugin panic]`, `panicked at`,
`SemioFaultError`, `terminal-fault`, `admission failed`, shard lost/terminated, `native-owner-required`)
versus `collateralFaults` (everything else FAULT_RE matches), and deduplicates by the first 60 characters
into `faultKeys` so "one fault ×200" is distinguishable from "200 distinct faults". Every verdict record
carries `faults`/`hardFaults`/`collateralFaults`/`distinctFaults`. Two summary verdicts are scored:
`battery-hard-faults` and `battery-faults`. The `isCollateralNotice` filter (agent disconnected / remote:
detached / reconnect), previously inlined in `locked-refusal` only, is now a shared helper.

The prose report's `## faults` section is split into `### hard` / `### collateral`.

### 2.5 Other correctness fixes to existing machinery

- `readHistoryEntryIds`/`newHistoryEntries` replace raw entry counts. The raw count is unreliable (the
  21:28 coordination entry's history-panel paging artefact — this pass reproduced `before=2 after=0`
  with no action in between), so "did this action add a history row?" is now a set difference on
  `framework.history.entry.*` ids. It also never clicks `#framework.history.undo`, unlike `openHistory()`,
  which would mutate the document from a read-only step.
- `unfoldMeasures` focuses the perspective pane first and retries until the toggle id flips
  `.unfold`→`.fold`. Without this the rail silently stayed folded and §3/§4 reported "no controls" for
  the wrong reason (first two window-options runs in this pass did exactly that).

---

## 3. New steps and what each verdict asserts

| step | group | § | verdicts |
|---|---|---|---|
| `window-content` | read | §1 | `window-both-present` (both window ids), `window-one-canvas-each` (per-window canvas count, not the global locator), `window-same-document-both-views` (equal `data-instances-json` length > 0), `window-distinct-camera` (`data-camera-json` differs between top and perspective). Writes one cropped screenshot per window. |
| `camera-gestures` | read | §2 | `camera-json-attribute`, `camera-orbit` (Alt+right-drag), `camera-pan` (Shift+right-drag), `camera-zoom` (wheel) — each asserts `data-camera-json` changed; `camera-lane-responsive` (page still answers `snapshot()`), `camera-emits-no-artifact-history` (no new history entry ids), `camera-per-window` (top window's camera untouched). Gesture mapping is derived from `resolveWorldOrbitMouseButtonsIdle` = `{LEFT:null, MIDDLE:PAN, RIGHT:null}` (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3255,3273-3276`) — LEFT is marquee, not orbit. |
| `projection-options` | read | §3 | `projection-measures-present` (any `puzzle3d-measure-projection-*` / `framework.worldOrbit.projection` in the DOM), `projection-control-flips`, `projection-repaints-camera`. |
| `window-options` | read | §4 | one `window-option-<id>` verdict per control (`grid-visible`, `grid-snap`, `grid-spacing`, `lod-auto`, `lod-value`, `vortex-show`, `vortex-direction`, `measure-sun-enabled`), each read→drive→read with a re-unfold retry before reporting absence; plus `window-options-lane-responsive` (the WindowConfig hang detector) and `window-options-emit-no-history`. |
| `settings-panel` | read | §19 | `settings-panel-opens`, `settings-steppers-present` (`puzzle3d-play-settings.*`), `settings-grid-spacing-bumps`, `settings-value-reaches-window-rail`. |
| `add-object-dialog` | read | §23 | `add-object-trigger-present`, `add-object-dialog-opens`, `add-object-kind-options-are-dynamic` (>1 kind option). |
| `locale-switch` | read | §25 | `locale-control-present`, `locale-flips-document-labels`, `locale-de-document-section-label` (checklist §25's `Baukomponenten`), `locale-no-english-leak`, `locale-switch-back-en`. |
| `volume-brush` | mutate | §10 | `volume-brush-arm` (`activeUtility === "volumeBrush"`), `volume-brush-target-volume-attribute`, `volume-brush-add-target-volume` (`data-target-volumes-json` count +1 after Alt+click), `volume-brush-voxel-dims` (`puzzle3d-voxel-w`). |
| `relocate` | mutate | §11 | `relocate-arm`, `relocate-pose-delta` (`data-instances-json` string delta after a raw drag), `relocate-no-hard-fault` (the checklist's predicted extent fault above 62 objects). |
| `engagement-bar` | mutate | §14 | `engagement-input-present`, `engagement-placeholder-has-no-dead-verbs`, `engagement-brush-verb`, `engagement-clear-is-a-noop`, `engagement-fill-verb`, `engagement-abort`. |
| `context-menu-rows` | mutate | §15 | `context-menu-selection-precondition` (an entity row is actually selected), `context-menu-opens`, `context-menu-object-vocabulary` (`duplicate`/`select-same-kind`/`zoom`/`delete`/`hide-show`/`lock-unlock`), `context-menu-zoom-row-action-is-registered` (`data-menu-action !== "zoomToSelection"`), `context-menu-zoom-moves-camera`. |
| `outliner-rows` | mutate | §17 | `outliner-panel-opens`, `outliner-hide-control-present`, `outliner-hide-applies`, `outliner-show-restores` (the `flag_args` hardcoded-`true` prediction; explicitly reported unreachable when Hide itself no-ops). |
| `catalogue-panel` | mutate | §18 | `catalogue-panel-opens`, `catalogue-kind-rows-present`, `catalogue-add-object-kind`, `catalogue-add-selects-new-object`, `catalogue-drag-drop` (synthesized `DragEvent`s carrying `application/x-semio-catalogue-item`). |
| `selection-keybindings` | mutate | §22 | `duplicate-selection` (Cmd/Ctrl+D), `duplicate-reselects-clone`, `focus-selection` (`f` → camera delta), `delete-selection` (Delete then Backspace) — ordered duplicate→focus→delete so Delete cannot starve the later assertions. |
| `export-import` (existing, refined) | replace | §24 | new `export-only` (download event scored on its own) and `import-same-file-idempotent` (instance count unchanged on a re-import of the just-exported file) alongside the existing `import-distinct`. |
| `example-switch` (existing, refined) | replace | §5 | new `example-switch-instances` (the document actually carries instances after the switch, not just a relabelled combobox). |
| `fill-history` (existing, refined) | mutate | §12 | new `fill-history-entry` verdict. |
| `context-menu` (existing smoke) | mutate | §15 | now dumps the raw menu and dismisses it; no longer skipped under `--battery`. |

---

## 4. Run output

### 4.1 §1 windows — `--only=window-content`

```
verdict window-both-present PASS
verdict window-one-canvas-each PASS
verdict window-same-document-both-views PASS
verdict window-distinct-camera PASS
battery PASS=7 FAIL=0 FAULTS=0
```

### 4.2 §2 camera — `--only=camera-gestures`

```
camera gestures: resolveWorldOrbitMouseButtonsIdle is {LEFT:null, MIDDLE:PAN, RIGHT:null} — orbit is Alt+right-drag, pan is Shift+right-drag, zoom is the wheel
verdict camera-json-attribute PASS
verdict camera-orbit PASS
verdict camera-pan PASS
verdict camera-zoom PASS
verdict camera-lane-responsive PASS
verdict camera-emits-no-history FAIL historyEntries before=2 after=5
verdict camera-per-window PASS
```

### 4.3 §3/§4 measures rail — `--only=window-options,projection-options`

```
verdict projection-measures-present FAIL ids=[]
verdict window-option-puzzle3d-play-grid-visible FAIL before={…"checked":true…} after={…"checked":true…}
verdict window-option-puzzle3d-play-grid-snap FAIL before={…"checked":false…} after={…"checked":false…}
verdict window-option-puzzle3d-play-grid-spacing PASS
verdict window-option-puzzle3d-play-lod-auto FAIL before={…"checked":true…} after={…"checked":true…}
verdict window-option-puzzle3d-play-lod-value PASS
verdict window-option-puzzle3d-play-vortex-show FAIL before={…"text":"Selected"} after={…"text":"Selected"}
verdict window-option-puzzle3d-play-vortex-direction FAIL before={…"text":"Outwards"} after={…"text":"Outwards"}
verdict window-option-puzzle3d-measure-sun-enabled FAIL control absent from the measures rail even after re-unfolding it
verdict window-options-lane-responsive PASS
```

with the rail inventory actually exposed:

```
measures rail unfoldControl=1 {"measures":["puzzle3d-play-vortex-show|select-trigger|","puzzle3d-play-vortex-direction|select-trigger|","puzzle3d-play-lod-auto|tree-action-checkbox|on","puzzle3d-play-lod-depth-variable|tree-action-checkbox|on","puzzle3d-play-lod-value|slider|","puzzle3d-play-grid-visible|tree-action-checkbox|on","puzzle3d-play-grid-snap|tree-action-checkbox|on","puzzle3d-play-grid-spacing|slider|","puzzle3d-play-select-objects|tree-action-checkbox|on","puzzle3d-play-select-vortices|tree-action-checkbox|on","puzzle3d-play-select-attractions|tree-action-checkbox|on"],…}
nudge puzzle3d-play-vortex-show select options=["Always","Selected"] current=Selected picking=0
nudge puzzle3d-play-vortex-direction select options=["Outwards","Inwards"] current=Outwards picking=1
nudge puzzle3d-play-grid-visible checkbox fell through to a synthetic wrapper click {"wrapper":true,"disabled":false}
```

An earlier run of the same step raised a hard fault from the slider:

```
verdict battery-hard-faults FAIL hard=3 collateral=4 distinct=5 first=[DEBUG] action failed setGridSpacing {value: 25, windowId: puzzle3d-main-perspective} SemioFaultError: fixed typed-operation and segmented-output authorities did not pre-admit the exact operation slot
```

### 4.4 §19/§23/§25 — `--only=settings-panel,add-object-dialog,locale-switch`

```
openPanel /settings/i hit={"id":"framework.settings","text":"Settings"} tabs=[{"id":"framework.panel.artifact","text":"Artifact"},{"id":"framework.panel.catalogue","text":"Catalogue"},{"id":"framework.panel.inspection","text":"Inspection"},{"id":"framework.category.display","text":"Display"},{"id":"s-sync-status","text":"Remote: detached"},{"id":"framework.settings","text":"Settings"},{"id":"framework.marketplace","text":"Marketplace"},{"id":"framework.panel.history","text":"History"},{"id":"framework.category.tool","text":"Tool"},{"id":"framework.category.command","text":"Command"}]
verdict settings-panel-opens PASS
settings steppers=[]
verdict settings-steppers-present FAIL ids=[]
verdict settings-grid-spacing-bumps FAIL plusButtons=0 before=null after=null
verdict settings-value-reaches-window-rail FAIL settings=null windowRail=null
add-object trigger=1 menu=["shell-menu.action.setActiveExample=1 Set Active Example","shell-menu.action.addObjectKind=2 Add Object…","shell-menu.action.duplicateSelection=3 Duplicate Selection mod+d","shell-menu.action.exportFixture=4 Export","shell-menu.action.openImportFixture=5 Import","menu.group.history=6 History ›","menu.group.hand=7 Hand ›","menu.group.selection=8 Selection ›","menu.group.more=9 More ›","shell-menu.action.deleteSelection=Delete Selection backspace"]
verdict add-object-trigger-present PASS
verdict add-object-dialog-opens FAIL dialogs=[]
verdict add-object-kind-options-are-dynamic FAIL no kind select rendered inside the dialog
locale en labels={"rootText":"OBJECTS | Hexagonal Cut Concrete Forest Left | Hide | Lock | REFERENCES | TARGET VOLUMES | ATTRACTIONS","rails":[]}
verdict locale-control-present PASS
locale de labels={"rootText":"","rails":[]}
verdict locale-flips-document-labels FAIL en=OBJECTS | … de=
verdict locale-de-document-section-label FAIL de= — checklist §25 expects "Baukomponenten"
verdict locale-no-english-leak FAIL de=
verdict locale-switch-back-en FAIL switched=false restored=true
```

The DE switch itself demonstrably lands — the shell chrome flips in the same dump
(`{"id":"framework.settings.default-apps","text":"Standard-Apps"}`, `{"id":"s-sync-status","text":"Remote: getrennt"}`).

### 4.5 §15/§17/§18 — `--only=outliner-rows,catalogue-panel,context-menu-rows`

```
selectViaOutliner rows=1 entitySelected=[] allAriaSelected=["mode-dock-tab-0-puzzle3d-main-top=Top","mode-dock-tab-1-puzzle3d-main-perspective=Perspective"]
context-menu rows=[{"id":"shell-menu.action.setActiveExample","action":"setActiveExample",…},{"id":"shell-menu.action.addObjectKind","action":"shell.openActionPane",…},{"id":"shell-menu.action.duplicateSelection","action":"duplicateSelection",…},{"id":"shell-menu.action.exportFixture","action":"exportFixture",…},{"id":"shell-menu.action.openImportFixture","action":"openImportFixture",…},{"id":"menu.group.history",…},{"id":"menu.group.hand",…},{"id":"menu.group.selection",…},{"id":"menu.group.more",…},{"id":"shell-menu.action.deleteSelection","action":"deleteSelection",…}]
verdict context-menu-opens PASS
verdict context-menu-object-vocabulary FAIL missing=["duplicate","select-same-kind","zoom","delete","hide-show","lock-unlock"] present=["shell-menu.action.…"]
verdict context-menu-zoom-row-action-is-registered FAIL zoom row action=row absent
verdict outliner-panel-opens PASS
outliner row controls=[{"tag":"button","slot":"action","text":"Hide"},{"tag":"button","slot":"action","text":"Lock"},{"tag":"button","slot":"action","text":"Hide"},{"tag":"button","slot":"action","text":"Unlock"},{"tag":"button","slot":"action","text":"Show"},{"tag":"button","slot":"action","text":"Unlock"}]
verdict outliner-hide-control-present PASS
verdict outliner-hide-applies FAIL beforeHead=[…"Hexagonal Cut Concrete Forest Left Hide Lock"…] afterHead=[…"Hexagonal Cut Concrete Forest Left Hide Lock"…]
verdict catalogue-panel-opens PASS
catalogue rows=[{"id":"panel:puzzle3d-play-kinds/Hexagonal Cut Concrete Forest Left","draggable":false,…},{"id":"panel:puzzle3d-play-kinds/puzzle3d-kind-entry:b-l","draggable":false,…},… 9 rows]
verdict catalogue-kind-rows-present PASS
verdict catalogue-add-object-kind FAIL before=1 after=1
verdict catalogue-add-selects-new-object FAIL added=0 …
verdict catalogue-drag-drop FAIL ran=true payload=panel:puzzle3d-play-kinds/Hexagonal Cut Concrete Forest Left before=1 after=1
```

### 4.6 §10/§11/§14/§22 — `--only=volume-brush,relocate,engagement-bar,selection-keybindings`

```
arm-utility volumeBrush found=1 active=volumeBrush
verdict volume-brush-arm PASS
verdict volume-brush-target-volume-attribute PASS
volume-brush volumes before=0 after=0 instances={"count":1,"ids":["seed-left-001"]}
verdict volume-brush-add-target-volume FAIL before=0 after=0
verdict volume-brush-voxel-dims PASS
arm-utility worldRelocate found=1 active=worldRelocate
verdict relocate-arm PASS
relocate poseBeforeLen=266 poseAfterLen=266 instances={"count":1,"ids":["seed-left-001"]}
verdict relocate-pose-delta FAIL beforeLen=266 afterLen=266 instances=1
verdict relocate-no-hard-fault PASS
engagement pane={"rootPresent":false,"rootText":null,"fields":["?|file|"]}
verdict engagement-input-present FAIL toggle=1 input=0 placeholder=null
verdict duplicate-selection FAIL before=1 after=1
verdict duplicate-reselects-clone FAIL
verdict focus-selection FAIL before={"position":[70.5577,-63.5577,46.614],"target":[0,0,0],…} after={"position":[70.5577,-63.5577,46.614],"target":[0,0,0],…}
verdict delete-selection FAIL before=2 after=2      (an earlier ordering of this step scored: verdict delete-selection PASS  before=1 after=0)
```

---

## 5. Product defects vs probe limitations

### Likely product defects, with DOM evidence

1. **§4 tree-row checkboxes cannot be toggled at all** — `grid-visible`, `grid-snap`, `lod-auto` (and by
   construction `lod-depth-variable`, `select-objects/vortices/attractions`). Three independent gestures
   were tried per control: a real Playwright click on the `input`, focus + `Space`, and a synthetic click on
   the wrapper; `checked` never flips. Source explanation: `🌳️Tree/🟦️.tsx:609-617` wraps the checkbox in
   `<label data-slot="tree-action-checkbox-wrapper" onClick={(event) => { event.preventDefault(); … }}>`.
   For a checkbox, `preventDefault()` on the (bubbled) click reverts the pre-click activation and suppresses
   `change` — so `onCheckedChange` can never fire from a user gesture. This is framework-wide, not
   puzzle3d-specific, and it is the concrete runtime reason §4's grid/LOD/select toggles "do nothing".
2. **§4 sun group is not rendered** — `puzzle3d-measure-sun-enabled` is absent from the rail dump even
   after re-unfolding, and no `puzzle3d-measure-sun-*` id exists anywhere in the document.
3. **§3 projection measures are not rendered** — `projection-measures-present FAIL ids=[]`; neither
   `puzzle3d-measure-projection-*` nor `framework.worldOrbit.projection` is in the DOM. The measures rail
   carries only grid / LOD / vortex / select.
4. **§4 `setGridSpacing` can hard-fault** — `SemioFaultError: fixed typed-operation and segmented-output
   authorities did not pre-admit the exact operation slot`, raised by the slider in one run and not in the
   next. Intermittent, and the rail collapsed afterwards (collateral). The slider's *DOM* value does move
   (`window-option-puzzle3d-play-grid-spacing PASS`), so the fault is on the publish side.
5. **§4 vortex-show / vortex-direction selects no-op** — the option lists are correct (`["Always","Selected"]`,
   `["Outwards","Inwards"]`), the probe picks the option that differs from the current value, and the
   trigger text is unchanged afterwards.
6. **§19 has no puzzle3d settings panel** — the complete panel-tab inventory is
   `framework.panel.artifact`, `framework.panel.catalogue`, `framework.panel.inspection`,
   `framework.settings`, `framework.marketplace`, `framework.panel.history` plus the display/tool/command
   categories. There is no `puzzle3d-play-settings` tab, and `[id^="puzzle3d-play-settings"]` matches
   nothing, so none of §19's four steppers is reachable.
7. **§23 Add Object opens nothing** — the workspace menu row exists and is
   `shell-menu.action.addObjectKind` with `data-menu-action="shell.openActionPane"` (note: **not**
   `openAddObjectDialog`, which the checklist assumes). Clicking it produces no `[role="dialog"]`,
   `[role="alertdialog"]`, `[data-slot="dialog"]` or `[id*="addObjectKind"]` surface, so the hardcoded
   single-`"Object"` kind option could not even be reached to be confirmed.
8. **§15 the puzzle3d per-selection context menu never appears** — right-clicking the viewport returns the
   shell fallback menu (`shell-menu.action.*` + `menu.group.*`) on every attempt, so none of
   `duplicate` / `select-same-kind` / `zoom` / `delete` / `hide-show` / `lock-unlock` is reachable and the
   `zoomToSelection` defect cannot be exercised. **Caveat**: the precondition verdict shows the selection
   was empty at the time (`entitySelected=[]`), so this is a *joint* failure of §6 viewport/outliner select
   and §15 — the menu falling back is the correct behaviour for an empty selection. Whoever picks this up
   should first make a selection land, then re-run `--only=context-menu-rows`.
9. **§17 outliner Hide is a no-op** — the row's `Hide` / `Lock` / `Show` / `Unlock` buttons exist
   (`data-slot="action"`), clicking `Hide` leaves the row text and order byte-identical. The checklist's
   predicted `flag_args` defect ("Show cannot un-hide") is therefore *worse* than described in the
   browser: Hide itself does not apply, so `outliner-show-restores` is reported unreachable rather than
   falsely PASSing.
10. **§18 catalogue rows are not draggable and adding does nothing** — nine kind rows render
    (`Hexagonal Cut Concrete Forest Left`, `b-l`, `b-l-m`, `b-s`, `b-s-m`, `c-b`, `c-t`, `cable.link`,
    `puzzle3d.attraction.link`), every one with `draggable: false` (contradicting §18's "rows are
    draggable"); clicking one leaves `data-instances-json` at `{"count":1,"ids":["seed-left-001"]}`.
11. **§14 the engagement pane does not render** — the toggle
    `framework.window.puzzle3dMainPerspective.engagement.toggle` exists and is clicked, but the container
    `framework.window.puzzle3dMainPerspective.engagement` is absent and the only `input` in the whole
    document is a `type=file`. No sub-verb could be exercised.
12. **§10 Alt+click adds no target volume** — the utility arms correctly
    (`activeUtility=volumeBrush`), `data-target-volumes-json` stays at length 0, and the voxel-dim slider
    does move. So the utility and its options are live; `addTargetVolume` is not.
13. **§11 relocate drag produces no pose delta** — the utility arms (`activeUtility=worldRelocate`), the
    raw `mouse.down`/`move`/`up` drag leaves `data-instances-json` byte-identical (266 chars both sides),
    and no hard fault is raised. On Concrete Forest (1 object) the checklist expects this to *work*.
14. **§22 duplicate and focus do nothing** — `Cmd+D` then `Ctrl+D` leave the instance count at 1; `f`
    leaves `data-camera-json` byte-identical. `Delete` did work once (`before=1 after=0`) and did not in a
    later ordering (`before=2 after=2`), i.e. it is order/selection-dependent.
15. **§25 switching to German empties the outliner** — the shell chrome translates correctly
    (`Standard-Apps`, `Remote: getrennt`), but `puzzle3d-play-document`'s own text goes from
    `"OBJECTS | … | REFERENCES | TARGET VOLUMES | ATTRACTIONS"` to the empty string, even after the probe
    polls for it 8×1.2 s. So §25's ambiguity resolves as "worse than a missing translation": the document
    tree stops rendering under a non-English locale.

### Confirmed working (first runtime confirmation in this ticket)

- **§2 camera is fully alive and the WindowConfig hang is gone for camera**: orbit, pan and zoom each move
  `data-camera-json`, the page stays responsive, and the sibling (top) window's camera is untouched —
  `set_camera_is_per_window_…` confirmed in the browser for the first time.
- **§1**: two windows, one canvas each, same instance count, genuinely different cameras (top is
  `"projection":"orthographic"` with `up:[0,1,0]`, perspective is `"projection":"perspective"` with
  `up:[0,0,1]`).
- **§4 sliders**: `grid-spacing` and `lod-value` both move.
- **§10 utility arming and voxel options**, **§11 utility arming**.

### Probe limitations (FAILs that are not, or not yet, product evidence)

- `camera-emits-no-artifact-history FAIL` (3 gestures → 3 rows). World3dHost's own docstring says the
  debounced `setCamera` exists *so that* "the shell-side command-history panel has something to show", which
  contradicts checklist §2's "emits no artifact mutations". The verdict note now carries both readings; the
  ticket needs to decide which is the contract before this counts as a defect.
- `window-options-emit-no-history` is scored on new entry ids, but the history panel's own paging still
  makes the underlying list jumpy (`before=2 after=0` with no action between). Treat a single FAIL here as
  weak evidence.
- `context-menu-*` is blocked behind an empty selection (see defect 8) — the menu vocabulary is not yet
  proven absent for a *populated* selection.
- `locale-switch-back-en FAIL switched=false` — after switching to DE the settings panel navigates into its
  sub-tabs (`framework.settings.general`/`…theme`/`…keybindings` appear in the tab dump) and
  `#framework.settings.language` is no longer reachable from where the probe looks. Probe-side; the DE
  switch itself is proven.
- `catalogue-add-selects-new-object` and `duplicate-reselects-clone` can only be judged once the
  corresponding add/duplicate works; both currently FAIL as a consequence of defects 10 and 14.
- `selection-keybindings` is order-sensitive: it now runs duplicate → focus → delete so Delete cannot
  starve the later assertions, but the instance count still drifted between sub-steps in one run
  (`duplicate before=1`, `delete before=2`), which suggests an async add landed late.

---

## 6. Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` —
  `world3dCameraDomJson` + `WORLD_CAMERA_DOM_PRECISION`; `data-camera-json` and
  `data-target-volumes-json` on the host root div.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` —
  re-export of `world3dCameraDomJson`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` —
  new test.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts` — all probe work.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️2026-09-11-wave-B1-battery-extension.md` —
  this report.
