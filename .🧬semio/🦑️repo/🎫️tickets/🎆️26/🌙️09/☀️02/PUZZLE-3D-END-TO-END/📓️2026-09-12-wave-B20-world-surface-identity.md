# Wave B20 — world surface identity, the focus camera, and the outliner Hide hop

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Continues `📓️2026-09-12-wave-B18-probe-hardening.md` §6
("Still red — handed over"), `📓️2026-09-11-wave-B9-guest-lanes.md` §1, `📓️2026-09-11-wave-B6-lane-diagnosis.md`
and `📓️2026-09-10-fill-build-host-tick.md` §8.29.

Files this wave owns:

| file | change |
|---|---|
| `🧰️framework/…/🧱️elements/🗣️Interpreter/🟦️.tsx` | `surfaceHostIdentityV1` — a surface host's identity is the WINDOW it was mounted into |
| `🧰️framework/…/🧱️elements/🌐️World3dHost/🟦️.tsx` | `worldSurfaceSelectionDomV1` + `data-selection-json` / `data-window-instance-id` |
| `🧰️framework/…/🧱️elements/🌐️World3dHost/🧫️fixtures/🪪️world-surface-identity.json` | new, language-neutral |
| `🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts` | the law reading that fixture |
| `✏️s/…/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `outliner_hide_reaches_the_world_instance_lane_and_flips_the_row_control` |
| `🔍️browser-probe.ts` | the three lanes only — every step name kept |

---

## 0 Verdict summary

| defect | verdict | root cause | where the fix landed |
|---|---|---|---|
| 1 both world surfaces are `data-surface-id="1"` with no selection | **FIXED, host-live on `:6013`** | `🗣️Interpreter/🟦️.tsx:411` took `String(record.id)` — a per-document DFS integer both panes mint — for the surface identity; and no host attribute ever carried the painted selection | host (Interpreter + World3dHost), vitest law + neutral fixture |
| 2 `focusSelection` accepted, camera bit-identical | **NOT A CAMERA DEFECT** | `context-menu-zoom-moves-camera` is green with a live selection, and moves exactly the pane the menu opened over (§2.2); `focus-selection` reds because it asks an already-whole-document-framed camera to frame the whole document (§6.2) | probe instrumentation only — both panes are now sampled, so a wrong-window write can never again read as "dropped" |
| 3 outliner Hide leaves the row unchanged | **guest half PROVEN GREEN, host hop handed to B19** | the guest emits `change-object-hidden`, the host records it in history, and the document the guest renders from never receives it — the mutation lane B19 is bisecting, red for duplicate/delete/addTargetVolume/gumball/relocate in the same battery (§6) | new guest law pins the two hops the browser measured; no host fix (B19's lane) |

---

## 1 Defect 1 — a surface host's identity is the window it is mounted into

### 1.1 Hop table

| hop | where | what it does | state |
|---|---|---|---|
| 0 | `✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:670` `scene_surface(SURFACE_VIEWPORT, World3d, &scene)` | authors the world surface node with `.try_id("puzzle.3d.play.viewport")` — the program's own surface id, which becomes the node's `key` | **correct** |
| 1 | `🖱️ui/🧬️contract/…/🗺️surface.rs` `SurfaceProps` | carries `kind` / `docSchema` / `doc` / `bindings` — `surfaceId` and `controllerId` were DROPPED from the contract in the `ui-w4-core` mirror regeneration | by design |
| 2 | `🗣️Interpreter/🟦️.tsx` `surfacePropsToComponentSceneNode` | substituted `String(record.id)` for BOTH dropped fields | **BROKEN — fixed, §1.2** |
| 3 | `🌐️World3dHost/🟦️.tsx:6063` | `data-surface-id={node.surfaceId}` | correct, fed a broken value |
| 4 | `🌐️World3dHost/🟦️.tsx:4634/4638` | `selection = mergeWorldSelectionWithLeftover(parseSelection(scene.selectionJson), instances)` — the pane's PAINTED selection | computed, **never published — fixed, §1.3** |

### 1.2 Root cause, hop 2

`🗣️Interpreter/🟦️.tsx:411` (pre-fix):

```ts
surfaceId: String(record.id),
controllerId: String(record.id),
```

`UiNodeRecord.id` is a DFS-order integer re-minted on every full-body reconciliation
(`uiNodeDomId`'s own docstring already says so, and is why DOM ids are keyed on `record.key` instead).
Each window instance renders its OWN `UiDocument`, so the world surface node is node `1` in the
Perspective document AND node `1` in the Top document — which is exactly the `data-surface-id="1"`
B18 §6.3 measured on both panes.

The document's own `surface` is the identity that already exists and already names the window. The
renderer mints one `UiDocumentStore` per window instance under `window:${instance.id}`
(`🏛️ShellHost/🟦️.tsx:9380` `builtNodeStoreFor(\`window:${instance.id}\`, …)`), so
`store.getState().surface` is `window:puzzle3d-main-perspective` / `window:puzzle3d-main-top` — the
renderer-side twin of the `1:puzzle3d-main-perspective` `pluginSurfaceRef` the plugin host's
`surface_contexts` table is keyed by (`📓️2026-09-10-fill-build-host-tick.md` §8.29 item 1). Same window,
same one-to-one identity, named on this side of the bridge; panel documents are `panel:<id>` by the same
rule, which is why the outliner's rows carry DOM ids like `panel:puzzle3d-play-document/seed-left-001`.

**Fix.** New exported `surfaceHostIdentityV1(surface, key, recordId)`, the one place the three
identities are decided, and `SurfaceView`/`PagedSurfaceView` now pass `store.getState().surface` in:

- `surfaceId` ← the owning document's surface (`window:puzzle3d-main-perspective` / `window:puzzle3d-main-top`)
- `controllerId` ← still `record.id`; the per-app registries keyed on it (the catalogue drop preview,
  `registerWorldCatalogueDropHost`) are deliberately pane-independent and must NOT gain the window
- `paneId` ← `record.key`, the program's authored surface id (`puzzle.3d.play.viewport`) — the one
  thing the contract really did drop, and stable across refreshes where `record.id` is not

### 1.3 The selection the pane paints was never published

`data-status-json` and `data-interaction-json` are the wrong places to look for a selection and always
were, which is why B18 read `selectedIds: []` on a pane that was rendering a selection:

| attribute | what it actually carries |
|---|---|
| `data-interaction-json` | `WorldInteractionRecord` — `activeUtility`, `brushCandidateIndex`, `hoveredVortexFullId`, `voxelDims`, `suggestionMenu`, `fillBuild`, `revealCutoffs`, `meshResidency`. **No `selectedIds`, no `hoverTarget`.** |
| `data-status-json` | the off-main-thread COMPUTE status (`{computing,label}`), read at `🌐️World3dHost/🟦️.tsx:4674`. puzzle3d never sets `scene.status_json` at all, so `"1="` was a correct reading of an unrelated lane. |
| `data-instances-json` | the guest's geometry cache. `world_instances_geometry_json`'s own docstring: "Selection/hover paint is driven by `selectionJson` on the host — never baked here so instance geometry stays stable across picks." |

So the merged `WorldSelectionRecord` — the thing the pane paints from — had no DOM projection.
New `worldSurfaceSelectionDomV1(selection, interaction)` publishes it as `data-selection-json`:
`selectedIds`, `activeObjectId`, `targetVolumeIds`, `referenceSelectedId`, `hoverTarget` (domain
inferred from the id shape), `hoveredVortexFullId`, `hoveredKindId`, `gumballActive`, `gumballTarget`,
`transformMode`, `activeUtility`. `data-window-instance-id` rides along so a reader can join a surface
to a window without parsing the id.

Per the ticket's model, and asserted by the law: **selection is per document** (both panes publish the
same `selectedIds`) and **hover is per window** (each pane publishes its own `hoverTarget`); the armed
utility is per window instance (wave B9).

### 1.4 Live result on `:6013` (`--only=context-menu-rows,outliner-rows`)

Before (B18 §6.3):

```
interaction=[{"surface":"1","selectedIds":[],…},{"surface":"1","selectedIds":[],…}]
surfaceStatus=["1=","1="]
```

After, same step, same document:

```
interaction=[{"surface":"window:puzzle3d-main-top","window":"puzzle3d-main-top","selectedIds":["seed-left-001"],"hovered":null,"selectedInstances":[]},
             {"surface":"window:puzzle3d-main-perspective","window":"puzzle3d-main-perspective","selectedIds":["seed-left-001"],"hovered":null,"selectedInstances":[]}]
… landedVia=background dispatchObserved=true selectedIds=["seed-left-001","seed-left-001"]
```

Two distinct surface identities, each naming its window, and both publishing the document selection the
leftover overlay carries. `selectedInstances` stays `[]` by design — the guest's instance lane is
geometry, not selection (§1.3).

### 1.5 Law

`🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts`, `"every pane of one document names its OWN window
surface, and publishes the selection it paints"`, reading the new language-neutral
`🌐️World3dHost/🧫️fixtures/🪪️world-surface-identity.json` (two panes + a keyless panel pane + the
selection publication, vortex-domain hover and object-domain hover).

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts \
  --testNamePattern='names its OWN window surface'
 Test Files  1 passed | 23 skipped (24)
      Tests  1 passed | 886 skipped (887)
```

Proven against the code it covers — `surfaceId` put back to `String(recordId)`, the law re-run, the file
restored in the same command:

```
AssertionError: pane puzzle3d-main-perspective: expected { surfaceId: '1', …(2) } to deeply equal { …(3) }
-   "surfaceId": "1:puzzle3d-main-perspective",
+   "surfaceId": "1",
 Test Files  1 failed | 23 skipped (24)
```

The failure text IS the browser symptom.

---

## 2 Defect 2 — `context-menu-zoom-moves-camera`

### 2.1 The lane, traced

| hop | where | state |
|---|---|---|
| 0 | `✏️editor/🦀️.rs:2831` context-menu `zoom` row → `focusSelection` (NOT the unregistered `zoomToSelection`) | correct (B18) |
| 1 | `🌐️World3dHost/🟦️.tsx` `handleWorldMenuDispatch` | intercepts `zoomToSelection` host-locally; `focusSelection` falls through to `dispatch(action, args)` with **no `windowId`** |
| 2 | `🏛️ShellHost/🟦️.tsx:5978` | `dispatchWindowId = actionWindowId ?? activeWindowIdRef.current` → `hostArmedViewContext` stamps `view_state.windowId` |
| 3 | `✏️editor/🦀️.rs:7754` `Puzzle3dFocusSelectionWork` | four stages; `Publish` writes `next.camera.position/target` onto `config_from_snapshot(window_config)` and emits `window_ownership::addressed_config(view, next)` — which FAULTS `puzzle3d-window-required` when `view.window_id` is absent, and returns `Emit::default()` (accepted, silent) when `matched == 0` |
| 4 | `Puzzle3dWindowConfig.camera` → `window_ownership::runtime` → `camera_json(&envelope.runtime)` → `scene.cameraJson` → `data-camera-json` | the lane's single writer, as B15 recorded |

Two ways this reads as "accepted and dropped" with no fault: `matched == 0` (`snapshot.typed().objects`
empty — B9 §2's class), or hop 2 addressing the OTHER pane, since `focusSelection` carries no window of
its own. **Only the second is distinguishable from outside, and only by sampling both panes** — which
B18 did not do.

### 2.2 What the probe now measures, and what it read

`context-menu-zoom-moves-camera` now samples `windowHostState()` (both panes, surface + camera) before
and after, reports `movedPanes`, and dumps the `focus|camera|window-required|windowConfig` console tail.

```
zoom panes before=[{"id":"puzzle3d-main-top","surface":"window:puzzle3d-main-top","camera":"{\"position\":[3.5,0,9.455],\"target\":[3.5,0,0.005],…"},
                   {"id":"puzzle3d-main-perspective","surface":"window:puzzle3d-main-perspective","camera":"{\"position\":[11.5131,-3.767,5.9804],\"target\":[0,0,0],…"}]
zoom panes after =[{"id":"puzzle3d-main-top",…"camera":"{\"position\":[3.5,0,9.455],…"},
                   {"id":"puzzle3d-main-perspective",…"camera":"{\"position\":[13.6973,-0.3725,5.8087],\"target\":[0,0,0],…"}] moved=["puzzle3d-main-perspective"]
zoom console tail=["[DEBUG] performInvocation {\"actionId\":\"focusSelection\"}","[DEBUG] command ingress lane {\"actionId\":\"focusSelection\",\"seq\":53,\"lane\":\"Interactive\"}"]
verdict context-menu-zoom-moves-camera PASS
```

The camera moves, exactly one pane moves, and it is the pane the menu was opened over — so hops 2-4 are
all correct on this build. **This wave changed nothing in the camera lane**, so the flip relative to
B18 §6.1 is not attributable to it; the plugin wasm was rebuilt at 18:46 local and other waves (B19,
B21, B22) were live in the same window. What the wave DOES leave behind is the instrumentation that
makes the two failure modes separable: a wrong-window write now prints
`movedPanes=["puzzle3d-main-top"]` instead of reading identically to a dropped one, and a
`puzzle3d-window-required` fault now lands in the step's own console tail.

**No fix and no new law were warranted for a lane that measures green.** If it regresses, the first
thing to read is `movedPanes`; if it is `[]` with no fault, the suspect is `matched == 0` — i.e.
`Puzzle3dFocusSelectionWork` reading `snapshot.typed()` where B9's `puzzle3d_fixture_from_projection`
made every other read go through the projection.

---

## 3 Defect 3 — outliner Hide

### 3.1 Hop table, with live evidence for every row

| hop | where | evidence | state |
|---|---|---|---|
| 0 | `📌️panels/🗿️artifact/🦀️.rs:135` `hide_lock_actions` → `flag_args(entity, id, "hidden", !hidden)` | row renders `Hide`+`Lock`; icon `eye`/`eye-off` | **correct** (B10, and re-pinned §3.3) |
| 1 | the DOM control | `clicked={"tag":"button","slot":"action","text":"Hide","row":"panel:puzzle3d-play-document/seed-left-001"}` | **correct** — the right control, on the right row |
| 2 | host dispatch | `[DEBUG] performInvocation {"actionId":"setSelectionFlag"}` + `[DEBUG] command ingress lane {"actionId":"setSelectionFlag","seq":26,"lane":"Interactive"}` | **correct** |
| 3 | guest arm | `🎮️commands/🔖️set-selection-flag/🦀️.rs` → `apply_puzzle3d_selection_flag(fixture,"object",[id],"hidden",true)`; `[DEBUG] puzzle3d.utility.publish action=setSelectionFlag window=Some("puzzle3d-main")` | **correct** |
| 4 | operation emitted + recorded | `[DEBUG] history patch applied {"currentCursor":3,"patchCursor":4,"upserts":1,"labels":["change-object-hidden id=seed-left-001 new-hidden=true"],"canUndo":true}` | **correct — the exact right operation, on the exact right object** |
| 5 | **the mutation reaches the document the guest renders from** | after hop 4 the guest re-renders repeatedly (`[DEBUG] puzzle3d.brushPreview.lane …` ×8) and still publishes the object at scale `[1,1,1]`: `worldHiddenAfter=[{"surface":"window:puzzle3d-main-top","hidden":[]},{"surface":"window:puzzle3d-main-perspective","hidden":[]}]`, row text byte-identical | **BROKEN — not this wave's lane, §3.2** |

`performInvocation settled {"frames":2,"frameKinds":["Invocation","Ephemeral"],"historyUpserts":0,"effects":0}`
— the invocation itself carries no history upsert; the operation reaches history on the typed-operation
lane afterwards, and it is that lane's commit into the artifact store that hop 5 is waiting on.

### 3.2 Why hop 5 is B19's, not this wave's

Three measurements, all on `:6013`, all this evening:

1. **The guest applies it.** `outliner_row_hide_and_show_round_trip_through_their_own_declared_args`
   (pre-existing) is green, and it runs on the SAME document the browser boots: `testkit::app()` builds
   `EditorApp::<Puzzle3dPlayApp>::default()`, whose `initial_snapshot` is the concrete-forest default
   fixture, and `first_object_id` is `seed-left-001`. The flag round-trips through the PROJECTION both
   ways in-crate.
2. **The guest publishes it, on both surfaces the user looks at.** New law, §3.3 — the world instance
   lane goes to scale `[0,0,0]` and the row control re-renders `eye` → `eye-off`. Also green in-crate.
3. **The browser's mutation lane is not wholly dead, but this path is.** `--only=catalogue-panel` on the
   same build:

```
verdict catalogue-add-object-kind FAIL before=1 after=1
verdict catalogue-add-selects-new-object FAIL added=0 selected=[…] ids=["seed-left-001"]
catalogue drag-drop {"ran":true,…} before={"count":1,"ids":["seed-left-001"]} after={"count":2,"ids":["seed-left-001","puzzle3d.object.c05b8dbd798d5454"]}
verdict catalogue-drag-drop PASS
```

A catalogue DROP commits a document mutation; a catalogue row CLICK and an outliner row CLICK do not.
That is the same family B19 is bisecting (catalogue add / duplicate / delete / undo-unwind), and the
drag-drop counter-example narrows it: the store commit path itself works, so the break is in how the
row-click path's typed operation is scheduled — explicitly off-limits for this wave. **Handed to B19
with the console tail above; no host change was made.**

### 3.3 New guest law

`✏️editor/🧪️tests/🔬️unit/🦀️.rs` → `outliner_hide_reaches_the_world_instance_lane_and_flips_the_row_control`.
The pre-existing law asserts the PROJECTION; the browser measured the two SURFACES, which is a different
claim and was unpinned. This one hides the default document's first object and asserts both:

- `world3d.instancesJson` for that id goes `[1,1,1]` → `[0,0,0]` (`world_instances_geometry_json` keeps
  the id in the array so no other object's index shifts)
- the outliner's visibility control re-renders `eye` → `eye-off`, found by walking to the smallest node
  that carries an `icon` AND declares that object's `hidden` flag args beneath it — keyed on the args,
  not on a row id, so it survives reshaping of the tree above the control

```
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly \
  --lib outliner_ -- --test-threads=1
test editor::puzzle3d::component::tests::outliner_hide_reaches_the_world_instance_lane_and_flips_the_row_control ... ok
test editor::puzzle3d::component::tests::outliner_row_hide_and_show_round_trip_through_their_own_declared_args ... ok
test editor::puzzle3d::panels::document::tests::an_outliner_flag_row_undoes_itself_on_the_second_click ... ok
test editor::puzzle3d::panels::document::tests::outliner_hide_and_lock_rows_dispatch_the_inverse_of_the_current_flag ... ok
…
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 702 filtered out
```

---

## 4 Probe changes (three lanes only, every step name kept)

| helper / step | change |
|---|---|
| `worldInteraction()` | reads `data-selection-json` FIRST (the painted selection), falling back to `data-interaction-json`; reports the new `window` field per surface |
| `context-menu-zoom-moves-camera` | samples BOTH panes before/after, reports `movedPanes`, logs a `focus\|camera\|window-required\|windowConfig` console tail |
| `outliner-hide-applies` | logs the control it actually clicked, the per-surface list of zero-scaled (hidden) instance ids before/after, and a 28-line dispatch console tail; **polls** up to 8 s for the row to change instead of one fixed 3 s sample |

No verdict name, step name or NDJSON key changed.

---

## 5 Verification — every command foreground, tails quoted

| command | result |
|---|---|
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: … generated 88 warnings` / `Finished \`dev\` profile … in 13.28s` — **0 errors** (the warnings are the proof expansion ran) |
| `RUST_MIN_STACK=134217728 cargo test … --lib outliner_ -- --test-threads=1` | `test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 702 filtered out` |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts` (whole config) | `Test Files 3 failed \| 21 passed (24)` / `Tests 9 failed \| 878 passed (887)` |
| `bun x tsc --noEmit -p …/⚛️react/tsconfig.json`, scoped | `Interpreter/🟦️.tsx`: only the pre-existing `import.meta.dir`. `World3dHost/🟦️.tsx`: the same four pre-existing errors (1303 `leftoverSelectIdsMustNameHoverPickV1`, 3302/4160 `pickEnabled`, 4497 overload) — none in the edited regions |

The vitest nine are **exactly** B18's nine — 1 engine-contract `buildNoteShellCommandAction`, 6
`🧪️tests/🧩️package-integration` wgpu-worker, 2 `🔌️PluginRuntime` — and the totals moved
`886 → 887` / `877 → 878` because this wave ADDED one passing test. **No new failure.**

### Probe verdicts on `:6013` (host-live; the wasm is the coordinator's, not rebuilt here)

Run `--only=context-menu-rows,outliner-rows` (directly comparable to B18 §5.1):

| verdict | B18 | B20 |
|---|---|---|
| `boot` | PASS | PASS |
| `context-menu-selection-precondition` | PASS | PASS — now with two distinct surface identities (§1.4) |
| `context-menu-opens` | PASS | PASS `rows=5` + 2 folded |
| `context-menu-object-vocabulary` | PASS | PASS |
| `context-menu-zoom-row-action-is-registered` | PASS | PASS (`zoom` → `focusSelection`) |
| `context-menu-zoom-moves-camera` | **FAIL** bit-identical | **PASS** `movedPanes=["puzzle3d-main-perspective"]` (§2.2) |
| `outliner-panel-opens` | PASS | PASS |
| `outliner-hide-control-present` | PASS | PASS |
| `outliner-hide-applies` | FAIL | **FAIL** — hop 5, handed to B19 (§3.2) |
| `outliner-show-restores` | FAIL | **FAIL** (unreachable behind the above) |
| `guest-alive-mutate` | PASS | PASS |
| `battery-hard-faults` / `battery-faults` | PASS | PASS |

`[58.9s] battery PASS=11 FAIL=2 FAULTS=0 first-hard-fault-at=none guest-death-faults=0`

### Host-live vs the coordinator's `#48`

Everything this wave changed is **host TypeScript**, so all of it is live on the vite-host at `:6013`
now and needed no rebuild. The one Rust change is a TEST (`🔬️unit/🦀️.rs`) and therefore rides no wasm
at all. Nothing in this wave waits on `#48`.

Timeline of the measurements, so each is attributable to a build:

| measurement | local time | build under it |
|---|---|---|
| `--only=context-menu-rows,outliner-rows` (§1.4, §2.2, §3.1) | 19:48 | pre-`#48`; directly comparable to B18's 19:25 run, same plugin wasm (built 18:46) |
| `--only=outliner-rows`, broadened console tail (§3.1 hops 4-5) | 19:50 | same |
| `--only=catalogue-panel` (§3.2 item 3) | 20:05 | same |
| the eight-step sweep (§6) | 20:13 → 21:00 | booted pre-`#48`; the `#48` deploy chain ran 20:31 → 20:35:12 underneath it (`:6013` went to `000` for new requests; the loaded page survived, `done booted=true faults=0 hard=0`) |

`:6013` answered `200` again at 20:51. Anything measured on `#48` itself is still to be taken — see §6.

---

## 6 The five lanes B18 expected defect 1 to unblock

```
bun 🔍️browser-probe.ts --only=selection-surfaces,volume-brush,relocate,brush-stroke,gumball-drag,context-menu-rows,outliner-rows,selection-keybindings --port=6013
```

Launched 20:13 local, ran 2810 s, log `🗑️generated/probe-2026-09-11T18-04-15.md` / `.ndjson`. The
coordinator's `#48` deploy chain started at 20:31 and finished 20:35:12
(`🗑️generated/deploy-2026-09-12-48.txt`, `Activated puzzle3d react release: 1 completed components
(changed)`), taking `:6013` to `000` for new requests in between; the probe's page was already loaded
and survived, and `done booted=true faults=0 hard=0` — but the WASM under it is the pre-`#48` one this
page booted, and the app carried ~45 minutes of accumulated mutation attempts by the end.

```
[2810.3s] done booted=true faults=0 hard=0 collateral=0 first-hard-fault-at=none guest-death-faults=0 verdicts=56
```

| verdict | B18 | this sweep | reading |
|---|---|---|---|
| `volume-brush-arm` | FAIL | **PASS** | **flipped** — the armed utility now reaches the pane that armed it |
| `volume-brush-target-volume-attribute` | — | PASS | |
| `volume-brush-voxel-dims` | — | PASS | |
| `relocate-arm` | — | PASS | |
| `gumball-handle-enter` | — | PASS | |
| `context-menu-selection-precondition` | FAIL→PASS (B18) | PASS | |
| `outliner-panel-opens`, `outliner-hide-control-present` | PASS | PASS | |
| `inspection-object-fields` | FAIL | FAIL `[expect-42] populated=false empty=true id=null` | no selection was live at the assert; not a surface-identity miss |
| `inspection-locked-flag-row` | FAIL | FAIL `[expect-42] lockChrome=false` | follows the above |
| `brush-preview-place` | FAIL | FAIL `[expect-41] instances=1 preview=null` | still `[expect-41]`-tagged |
| `gumball-scene-delta` | FAIL | FAIL `sceneDelta=false poseLen=266` | **mutation lane, §3.2** |
| `relocate-pose-delta` | FAIL | FAIL `beforeLen=296 afterLen=296 instances=1` | **mutation lane** |
| `volume-brush-add-target-volume` | — | FAIL `before=0 after=0` | **mutation lane** |
| `duplicate-selection` / `duplicate-reselects-clone` | — | FAIL `before=1 after=1` | **mutation lane — B19's own symptom** |
| `delete-selection` | — | FAIL `before=1 after=1` | **mutation lane — B19's own symptom** |
| `outliner-hide-applies` / `outliner-show-restores` | FAIL | FAIL | **mutation lane, §3** |
| `context-menu-opens` / `-object-vocabulary` / `-zoom-row-action-is-registered` | PASS | FAIL `rows=0` | see §6.1 |
| `focus-selection` | — | FAIL, camera bit-identical | see §6.2 |
| `guest-alive-mutate`, `battery-hard-faults`, `battery-faults` | PASS | PASS | |

So **one of the five flipped** (`volume-brush-arm`), and the other four did not: three of them
(`gumball-scene-delta`, `relocate-pose-delta`, and by extension anything that must SEE a mutation) sit
behind the same hop 5 as defect 3, and `inspection-*` failed for want of a live selection at the assert.
That matches §1.3's reading — those lanes were never blocked by the id collision itself; they read
per-element attributes that were always published per pane.

The whole mutating family is red together in this one battery — `duplicateSelection`, `deleteSelection`,
`setSelectionFlag`, `addTargetVolume`, `translateSelection` (gumball) and `worldRelocate` — which is the
single cleanest statement of B19's bisect target this ticket has: **no document or pose mutation
commits**, while the read/arm/chrome half of the app is entirely green.

### 6.1 `context-menu-opens rows=0` here, `rows=5` at 19:48

Not a regression from this wave: in the long battery the row this step selects came back as a VORTEX,
`selectedIds:["seed-left-001:v10"] hovered:"seed-left-001:v10"` — the outliner had been walked by the
earlier steps and the first `[role=treeitem]` under the object was one of its vortices, so the menu
opened on a granularity with no object vocabulary. The isolated `--only=context-menu-rows,outliner-rows`
run 45 minutes earlier selected `seed-left-001` itself and read `rows=5` + 2 folded (§1.4).

### 6.2 `focus-selection` FAIL is a probe artefact, not a camera defect

The `F` keybinding fires at the END of the battery, when `frame-perspective` and the earlier steps have
already framed the camera on the whole document (`position:[70.5577,-63.5577,46.614] target:[0,0,0]`)
and nothing is selected. B11's rule — an empty selection frames the WHOLE document — then asks for the
pose the camera is already in, so a correct `focusSelection` produces a bit-identical camera and the
verdict reds. `context-menu-zoom-moves-camera`, which runs with a live single-object selection and an
un-framed camera, PASSed on the same build (§2.2). **The step needs a precondition (a live selection, or
a deliberately displaced camera) before its red means anything**; left for whoever owns §22, since this
wave's edit budget was the three lanes named in its own brief.

### 6.3 Verdict→step map, for the re-run on `#48`

`--only=` takes STEP names, not verdict names:

| verdict | step |
|---|---|
| `inspection-object-fields`, `inspection-locked-flag-row` | `selection-surfaces` |
| `brush-preview-place` | `brush-stroke` |
| `volume-brush-arm` | `volume-brush` |
| `gumball-scene-delta` | `gumball-drag` |
| `relocate-pose-delta` | `relocate` |
| `focus-selection` | `selection-keybindings` |
| `context-menu-*` | `context-menu-rows` |
| `outliner-*` | `outliner-rows` |

---

## 7 Handover

1. **The whole mutating family → B19.** For `setSelectionFlag` specifically, hops 0-4 are proven
   correct live (§3.1) including the exact operation in the host's history
   (`change-object-hidden id=seed-left-001 new-hidden=true`), and the guest half is proven green
   in-crate on the same document (§3.3). Hop 5 — the typed operation committing into the document the
   guest renders from — is B19's. §6's battery shows the same hop killing `duplicateSelection`,
   `deleteSelection`, `addTargetVolume`, gumball `translateSelection` and `worldRelocate` in one run,
   with the read/arm/chrome half entirely green. The one counter-example worth bisecting against:
   `catalogue-drag-drop` DID commit (`before=1 → after=2`, §3.2) on the same build, so the store commit
   path is alive and the break is in how a row-click-authored action's typed operation is scheduled.
2. **§6 re-run on `#48`** — one command, quoted above. `:6013` answered `200` again at 20:51.
3. **`context-menu-zoom-moves-camera`** reads green; if it goes red, read `movedPanes` in the verdict
   note FIRST (§2.2) — the two failure modes are no longer indistinguishable.
4. **`focus-selection` (§22) needs a precondition** (§6.2): it currently asks an already-whole-document-
   framed camera to frame the whole document, so a CORRECT `focusSelection` reds it. Not this wave's
   lane to edit.
