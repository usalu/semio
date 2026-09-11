# Wave B26 — Residual non-selection lanes off battery #48

Implementation pass, 2026-09-11 evening CEST (the "#48" lane pass in
`🗑️generated/lanes-2026-09-12-48.txt`). Six reds that are NOT the selection-scoped family B25 owns.

Every command tail and every browser reading quoted below is real output from this pass, taken on the
live `:6013` React serve while no other probe was running.

**Headline** — four of the six are now root-caused to a named `file:line` and FIXED (two product, two
probe); one is a product defect this wave LOCATED with a new DOM predicate and did not repair; one is
handed on with the hop it dies in named. Two of the previously-red lanes turn out to have been scoring
**vacuously**: `undo-unwind` passed on a document that had never switched, and
`import-same-file-idempotent` passed with no import having happened at all.

Guest changes ride **wasm #50** (`export_fixture`, `set_active_example`, the publication contract). Host
changes (`🌳️Tree`, `🏛️ShellHost`, `🛠️ShellHelpers`) and every probe change are **vite-live now** — the
`data-activatable` attribute added below was already being read back off `:6013` in this pass, which is
its own proof that the host half is live.

---

## 1. §24 `export-only download=none` — PROBE ROUTE (fixed) + the B13 residual (fixed, rides #50)

### 1a The route — the probe was driving a menu that cannot exist

`activateWorkspaceMenuOrdinal` (`🔍️browser-probe.ts:955-1005`, now deleted) right-clicked the canvas and
looked for `[data-menu-action="exportFixture"]` / `[id="shell-menu.action.exportFixture"]`. That menu is
`ShellHost`'s FALLBACK context menu, and `ShellHost` suppresses it by design:

```ts
// 🏛️ShellHost/🟦️.tsx — the shell context-menu listener
if (event.defaultPrevented) return;
```

`World3dHost.onContextMenu` calls `preventDefault()` synchronously — that IS how an inner surface claims
a right-click — so the only menu that ever carries `shell-menu.action.exportFixture` is suppressed over
a viewport, and the guest's own viewport menu carries no file verbs. Every run since #45b recorded
`menu nodes exportFixture: []`. B13 §2 already proved the chain itself works
(`{"download":"puzzle-3d.json","size":7542}` live).

**Fix (probe)** `🔍️browser-probe.ts` — `activateWorkspaceMenuOrdinal` is replaced by
`activateWindowFileAction("exportFixture" | "openImportFixture")`, which drives the real user path: unfold
the window's Actions pane (`framework.window.puzzle3dMainPerspective.engagement.toggle`, the `Pane`
`toggleId` in `🪟️Window/🟦️.tsx:384`) and press the action's own row (`action.exportFixture` /
`action.openImportFixture`, built by `windowActionPaneSections` in `🛠️ShellHelpers/🟦️.tsx:3580`). Both
verbs are `ActionKind::Shell` with no declared args, so their row EXECUTES on one press — no staged form.
The helper presses the toggle ONLY while the row is absent, which is what the old fallback got wrong (it
clicked the last button whose text was exactly `actions` and FOLDED the pane it needed:
`action-pane export=0 textBtn=0`). A new `actionPaneState()` reading is logged with every attempt.

The dead "staged execute / `[role=menuitem]` retry" fallbacks are deleted: they never measured anything
and turned a red route into an inconclusive one.

### 1b B13's residual — the filename now names the example (guest, rides #50)

`export_fixture` (`✏️editor/🎮️commands/📤️export-fixture/🦀️.rs:8-16`) hardcoded
`filename: "puzzle-3d.json"`, so Concrete Forest and Nakagin exported as two identically-named files.
B13 named the clean fix and handed it over because `🎚️config` was B12's.

**Fix (product, guest)** — one new field, schema-first across all five facets:

- `✏️editor/🎚️config/🧬️schema/{🦀️.rs,🟦️.ts,🔗️.graphql,🔣️.json,🛰️.proto}` — `activeExampleId: String`
  (`#[state(config)]`, `required`, proto field 5, a `text()` guard in the TS parser).
- `✏️editor/🎚️config/🦀️.rs` — `Puzzle3dConfig.active_example_id` + `Puzzle3dRuntime.active_example_id`
  and both `Default` impls.
- `✏️editor/🪟️window/🦀️.rs` — `shared()` carries it out of the runtime, `runtime()` projects it back in.
- `✏️editor/🎮️commands/🛍️set-active-example/🦀️.rs` — the direct arm resolves the picker alias
  (`concrete`/`nakagin`) to the canonical id and stamps it on the fresh runtime.
- `✏️editor/🦀️.rs` — `Puzzle3dSetActiveExampleWork::canonical_example_id` + its `Publish` arm now emits
  ONE `Puzzle3dConfigMutation::Snapshot` carrying `active_example_id` over `..config.clone()`, so the
  user's fill count / overlap budget / kind weights ride through untouched.
- `✏️editor/🦀️.rs` — `setActiveExample`'s `ArtifactToolPublicationContract` gains
  `ArtifactToolPublicationLane::Config`. Without it the very first run faulted
  *"typed-operation emitted a store lane absent from its exact factory publication contract"* — the
  contract table is the thing that makes a new lane a deliberate act.
- `✏️editor/🎮️commands/📤️export-fixture/🦀️.rs` — new pure `puzzle3d_export_filename(active_example_id)`:
  `"<example-id>.json"`, or `"puzzle-3d.json"` for a document that came from no example.

**Law** `✏️editor/🧪️tests/🔬️unit/🦀️.rs` →
`export_fixture_names_the_download_after_the_active_example` (blank → `puzzle-3d.json`, Concrete Forest →
`concrete-forest.json`, the `nakagin` ALIAS → `nakagin-capsule-tower.json`, cleared → back to generic).
The pre-existing `set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin` assertion
*"example loading leaves shared app preferences untouched"* is rewritten to the sharper truth it now has:
exactly one config row, moving exactly one field.

```
test editor::puzzle3d::component::tests::export_fixture_downloads_round_trippable_json ... ok
test editor::puzzle3d::component::tests::export_fixture_names_the_download_after_the_active_example ... ok
test editor::puzzle3d::component::tests::leftover_export_fixture_downloads_puzzle_3d_json ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 714 filtered out; finished in 1.19s
```

```
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 693 filtered out; finished in 4.37s   (filter: example)
```

**Probe** gains `export-names-the-example`, which asserts the download's `suggestedFilename()` equals the
active example's id (the ids ARE the labels lowercased and dash-joined). It is red until #50 — say so.

---

## 2. §24 `import-distinct before=1 after=1` — the import NEVER RAN, and `import-same-file-idempotent` was vacuous

`🗑️generated/probe-2026-09-11T20-51-23.md`:

```
[94.2s] import chooser=none  fileOpenFx=[… only "unmapped effect \"send-message\"" …]
[110.0s] import hop-census: {… "import-picker hop":0, "importFixture ingress":0, "request-file-open mapped":0}
[110.0s] verdict import-same-file-idempotent PASS        ← nothing was imported; the count could not move
[143.9s] distinct chooser=none
[143.9s] distinct import path missed chooser and execute
[151.8s] verdict import-distinct FAIL [expect-41] before=1 after=1
```

Same root cause as §1a: the import route was the suppressed canvas menu (`activateWorkspaceMenuOrdinal("5")`),
so `openImportFixture` was never dispatched and `requestFileOpen`'s `<input type=file>`
(`🛠️ShellHelpers/🟦️.tsx`, `input.click()`) never opened a chooser. Not a guest fold defect: B9 §3's law
`exported_fixture_bytes_reimport_as_a_distinct_document_and_then_as_an_identity` is green, and the
guest's `[DEBUG] puzzle3d.import.parsed objects=N before=M` taps recorded a real 1→2 round trip natively.

**Fix (probe)** both imports now go through `activateWindowFileAction("openImportFixture")` — whose host
half is fully shell-intercepted (`🏛️ShellHost/🟦️.tsx:6032`, `[DEBUG] import-picker hop host-arm
openImportFixture` → `requestFileOpen` → re-dispatch as `importFixture`). Three further repairs to the
step so a green means something:

- the distinct fixture's clone now re-keys its VORTICES onto its own id (`<cloneId>:vN`); a clone that
  keeps `seed-left-001:v0…` is not a second object the app can attract against;
- the census is POLLED (ingress → guest fold → world republication are three round trips) instead of
  sampled once at a fixed 2 s;
- new verdict `import-distinct-records-history` (entry count must grow), and `import-distinct`'s note now
  quotes the guest's own `[DEBUG] puzzle3d.import.*` taps, so "the guest saw N=2 and folded it to nothing"
  and "the guest never saw anything" can never again read the same.

Not re-measured green in this pass — the export/import lane needs the Actions-pane route on a free
`:6013`; it is the first lane the coordinator should re-run.

---

## 3. §20 `undo-redo FAIL example=Concrete Forest` — the lane never loaded Nakagin (probe), plus one real host defect

### 3a The lane was measuring a document it never switched

`🗑️generated/probe-2026-09-11T20-49-21.md`:

```
[36.8s] example after switch: Concrete Forest          ← the switch never happened
[40.8s] unwind start: example=Concrete Forest entries=[…"Set Active Example↶"…]   ← the BOOT row
[49.4s] verdict undo-unwind PASS                       ← vacuous: /concrete forest/ && !/nakagin/
[118.0s] verdict undo-redo FAIL example=Concrete Forest ← demands a Nakagin that was never loaded
```

Two probe defects compounding, both at `🔍️browser-probe.ts`:

1. `const family = process.argv.includes("--reserved-family") || battery` (`:239-242`) reads argv ALONE.
   Under `--only=example-switch,history-open,undo-unwind,undo-redo` the undo steps are registered (`--only`
   overrides every gate) while `wantUndo` stays FALSE — so `example-switch` picked `options.nth(1)`
   instead of Nakagin and emitted no `example-switch` verdict at all. **A lane and its flag must select the
   same behaviour or the lane's greens mean nothing.** `family`/`wantExample` now count `--only`
   membership.
2. `example-switch` reached for `page.locator("select").first()` and, failing that,
   `[role="combobox"]` `.first()`. The `history-open` step runs FIRST (read group) and opens the history
   panel, which carries its own command-filter control — so `.first()` was not the example picker.
   The step, `readExample()` and `historyState()` now all address
   `#playground.navbar.fixture` (`NavbarExampleSelect`, `🏛️ShellHost/🟦️.tsx:8331`) by id, with the bare
   combobox only as a fallback.

### 3b A real host defect underneath it: redo could not relabel the picker

Even with the switch repaired, redo was one-way. From the same run's console:

```
[DEBUG] history patch applied {… "labels":["Undo","Undo","Undo","Switch Panel Tab","Toggle Panel","Set Active Example"], "canUndo":false}
[DEBUG] navbar example from history {"navbarExample":"concrete-forest","remembered":""}
[DEBUG] history route fallback handleAction {"action":"redo"}
[DEBUG] history patch applied {… "labels":["Redo","Undo","Undo","Undo",…,"Set Active Example"], "canUndo":true}
        ← and NO "navbar example from history" line: the label never followed the redo
```

`navbarExampleIdFromHistoryUpserts` (`🛠️ShellHelpers/🟦️.tsx:352-361`) answers a POPPED row with
`bootExampleId` (no memory needed — which is why undo relabelled fine) and a LIVE row with
`rememberedExampleId || undefined`. `remembered` was `""`, so redo returned `undefined` and the picker
stood still. The only writer of that memory was `NavbarExampleSelect`'s own `onValueChange`
(`🏛️ShellHost/🟦️.tsx:8336`) — a row can be redone that this shell dispatched from the palette, a context
menu, a replayed shell command or the boot load, and none of those ever wrote it down.

**Fix (product, host, vite-live)** new pure `rememberedExampleIdFromDispatchV1(action, remembered)` in
`🛠️ShellHelpers/🟦️.tsx`, applied in the ONE `onAction` funnel (`🏛️ShellHost/🟦️.tsx:6041`), so every route
that loads an example teaches the memory. An empty `exampleId` means "the app's default document" and
leaves the memory standing rather than erasing it (the popped-row branch already answers that case).

**Law** `🧪️tests/🔬️engine-contract/🟦️.ts` →
*"remembers the example id of every setActiveExample dispatch, so a redone row can relabel the picker"*
(undo → boot id; redo → exactly what was remembered; an unrelated verb never overwrites it).

```
 Test Files  1 passed | 23 skipped (24)
      Tests  3 passed | 895 skipped (898)
```

---

## 4. §15/§22 `context-menu-zoom-moves-camera` / `focus-selection` — MEASUREMENT, not dispatch (probe)

Both steps measured a camera that was ALREADY framing the thing they asked it to frame:

```
[1208.3s] verdict context-menu-zoom-moves-camera FAIL before={"position":[11.5131,-3.767,5.9804],…} after={…identical…} movedPanes=[]
[27.6s]   verdict focus-selection FAIL before={"position":[13.6973,-0.3725,5.8087],…} after={…identical…}
```

`focusSelection` frames the selection (B11 made an empty selection frame the whole document); framing a
pane that is already framed republishes a BIT-IDENTICAL pose, which reads exactly like a dropped camera
write. Neither step had a precondition that made "moved" a question worth asking.

**Fix (probe)** new shared `orbitPerspectiveAway()` helper (Alt + right-drag, the orbit binding
`camera-gestures` documents, then `cameraSettled`). `context-menu-rows` orbits BEFORE opening the menu (an
Alt+right-drag would dismiss an open one); `selection-keybindings` orbits between selecting and pressing
`f`, records what is actually selected into the verdict note, and waits on `cameraSettled` instead of a
fixed 3 s — the camera lane is trailing-debounced host-side, so the old fixed wait could sample a pose
that had not moved yet (wave B12 §4.1). If either still reads identical after this, it is a real dropped
write and the note now says what was selected when it happened.

---

## 5. §18 `catalogue-add-object-kind before=1 after=1` — LOCATED, not fixed: the row is COVERED

This one is neither the guest nor the row wiring. Three independent green measurements and one red:

**Guest — green.** New law `✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` →
`every_object_kind_row_binds_activate_to_add_object_kind_with_its_own_kind_id`: every object-kind row
declares `Trigger::Activate → addObjectKind` with `{objectKind: <its own id>}`, on the ROW (which is
expandable — its rim-vortex templates are its children), not on a leaf underneath it.

```
test editor::puzzle3d::panels::catalogue::tests::every_object_kind_row_binds_activate_to_add_object_kind_with_its_own_kind_id ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 714 filtered out; finished in 0.10s
```

**Host — green.** New law `🧪️tests/🔬️engine-contract/🟦️.ts` →
*"fires an expandable catalogue row's own activate binding, args and all, instead of only folding it"*:
the real `uiNodeToTreePanelConfig` → `UiDocumentStore` → `InterpretedUiNode` → `Tree` path dispatches
`{controllerId:"puzzle3d-play", action:"addObjectKind", args:{objectKind:…}}` on a click of the row shell
AND on a click of the row's label.

**Live DOM — green.** New `data-activatable` attribute (below) reads back off `:6013`:

```
catalogue rows=[{"id":"panel:puzzle3d-play-kinds/Hexagonal Cut Concrete Forest Left","activatable":"true","rowKind":"group",…}]
```

**Live click — red, and the reason is hit-testing.** New hit-test in the step, before the press:

```
catalogue add hit-test={"label":"span|tree-label","top":"div|?|?","covered":true,"activatableRow":null,
 "topChain":["div#-[-]{p-single flex gap-single items-center min-w-0 h-full}",
             "nav#ui.navbar[navbar]{relative h-large z-navbar bg-transparent}",
             "div#-[-]{flex-shrink-0}","div#-[layout]{…h-screen…}", … ,"div#root[-]{}","body#-[-]{}"]}
verdict catalogue-add-object-kind FAIL before=1 after=1 ids=["seed-left-001"]
```

`document.elementFromPoint` at the centre of the first kind row's label returns **a child of
`nav#ui.navbar`**, not the row. The catalogue panel's rows are laid out UNDER the navbar, so a real
pointer press can never reach them — and the console for the whole click window carries no
`performInvocation {"actionId":"addObjectKind"}` at all, only the `registerBrushMesh` background traffic.
`catalogue-drag-drop` passes in the same step because it synthesizes `DragEvent`s directly on the source
element and bypasses hit-testing entirely.

**Left to the owner of the panel/navbar layout, deliberately** — it is chrome geometry, not this lane's
plumbing, and it will move under whoever is holding the dock. Everything needed to fix it and to prove the
fix is now in place. Note it is NOT a probe artifact: `click({force:true})` skips the "receives pointer
events" check, which is precisely why this read as "the row is not wired" for three waves.

**Product change landed alongside it (host, vite-live)** `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx`:

- new `TreeItemProps.activatable`, passed by `TreeDataItemView` as `Boolean(item.onClick)` — `onClick`
  cannot answer this question because the tree wires a selection handler onto EVERY row;
- `data-activatable` on all three row shells, so "the guest never authored the binding", "the binding was
  lost on the way in" and "the click was swallowed" stop being one indistinguishable red;
- the `property` layout's label handler no longer swallows an activatable expandable row's click into a
  fold (`:2252` used to `setOpen(); return;` unconditionally for `isExpandable`, and that branch's row
  shell carries no `onClick` at all, so such a row's action was unreachable by ANY click). Folding stays
  on the chevron button beside it, exactly as in the default layout. The live catalogue row turns out to
  render in the DEFAULT layout, so this is a latent twin of the same defect rather than this red's cause —
  fixed because it is the same bug one branch over.

---

## 6. §17 `outliner-hide-applies` — NOT selection-scoped, and the dispatch LANDS; the panel is never re-taken

Re-measured this pass (`--only=catalogue-panel,outliner-rows --port=6013`):

```
outliner hide console tail=[
  "[DEBUG] performInvocation {\"invocationKind\":\"action\",\"instanceId\":1,\"actionId\":\"setSelectionFlag\"}",
  "[DEBUG] command ingress lane {\"instanceId\":1,\"actionId\":\"setSelectionFlag\",\"seq\":17,\"lane\":\"Interactive\"}",
  "[DEBUG] puzzle3d.utility.publish action=setSelectionFlag window=Some(\"puzzle3d-main-top\") …",
  "[DEBUG] performInvocation settled {…\"actionId\":\"setSelectionFlag\",\"frames\":2,\"historyUpserts\":0,\"effects\":0}" ]
verdict outliner-hide-applies FAIL … worldHidden=[{"surface":"window:puzzle3d-main-top","hidden":["seed-left-001"]},
                                                 {"surface":"window:puzzle3d-main-perspective","hidden":["seed-left-001"]}]
                                    clicked={"tag":"button","slot":"action","label":null,"text":"Hide","row":"panel:puzzle3d-play-document/seed-left-001"}
```

So, precisely: the row action **is** dispatched (it carries the row's OWN id — `flag_args(entity,id,"hidden",!hidden)`,
`📌️panels/🗿️artifact/🦀️.rs:131`, so this is not the selection-scoped family and not B25's), the guest
**does** apply it (both world surfaces publish a zero scale for `seed-left-001`), and the artifact panel
body is **never re-taken** — `rowDump` is byte-identical across 8 polled seconds, and the settle reports
`historyUpserts:0, effects:0`.

The world lane moving proves nothing about `refreshUi`: the world scene rides the guest's own publication
lanes, not the panel refresh. The guest half is green on both halves of the claim
(`outliner_hide_reaches_the_world_instance_lane_and_flips_the_row_control` asserts scale `[0,0,0]` AND the
row control flipping `eye`→`eye-off`, and `hide_lock_actions` flips the LABEL `Hide`↔`Show` too, which is
what the probe reads):

```
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 710 filtered out; finished in 0.27s   (filter: outliner)
```

`puzzle3d_command_scope_class("setSelectionFlag")` is `Document`, whose partial scope names
`puzzle.3d.play.document` (`puzzle3d_document_panel_bodies`, `✏️editor/🦀️.rs:2259`), so the scope is right.
**The open hop is the browser-actor dispatch's own refresh**: B9 §4's `browserActorDispatchUiScopeV1`
gives `{kind:"full"}` only for `mutationCount > 0`, and a typed operation is merely ADMITTED on the reply
that carries that count — its mutations land on a later `OperationCompleted` frame. A reply reporting
`mutationCount: 0` yields `{kind:"none"}` and nothing ever asks for the panel. That is the next thing to
instrument (`🏛️ShellHost/🟦️.tsx` `dispatchDirectBrowserActorCommand` /
`subscribeOperationCompletions`), and it is adjacent enough to B25's `SurfaceContexts` work that this wave
deliberately did not edit the plugin `🦀️.rs` under it.

---

## Files

Product (host, vite-live now):

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`

Product (guest, rides wasm #50):

- `✏️s/…/🧊️3d/…/✏️editor/🎚️config/🦀️.rs` and `🎚️config/🧬️schema/{🦀️.rs,🟦️.ts,🔗️.graphql,🔣️.json,🛰️.proto}`
- `✏️s/…/🧊️3d/…/✏️editor/🪟️window/🦀️.rs`
- `✏️s/…/🧊️3d/…/✏️editor/🎮️commands/📤️export-fixture/🦀️.rs`
- `✏️s/…/🧊️3d/…/✏️editor/🎮️commands/🛍️set-active-example/🦀️.rs`
- `✏️s/…/🧊️3d/…/✏️editor/🦀️.rs`

Laws:

- `✏️s/…/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/…/🧊️3d/…/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`

Probe (owned by this wave; every existing step and verdict name preserved, two verdicts added —
`export-names-the-example`, `import-distinct-records-history`):

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts`

## Verification

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
    Finished `dev` profile [unoptimized] target(s) in 0.79s          (0 errors)
cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2
    Finished `dev` profile [unoptimized] target(s) in 0.91s          (0 errors)
bun 🔍️browser-probe.ts --only=boot --port=6013
    [7.0s] battery PASS=3 FAIL=0 FAULTS=0 first-hard-fault-at=none guest-death-faults=0
```

Renderer-react vitest lane, full: `Test Files 3 failed | 21 passed (24)`, `Tests 9 failed | 889 passed`.
**None of the nine are new from this wave** and none touch the three files it changed — they are live
peers' in-flight work: `buildNoteShellCommandAction` now emits `inverseCommandId`/`inverseArgs` its own
test has not caught up with, `readAppDocumentPack()` now returns an `ops` field, and the six
`🧪️tests/🧩️package-integration` wgpu-worker rows. This wave's three laws are green in the same run.
