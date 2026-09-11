# Wave B6 — first-pick Inspection, Brush preview, clipboard, import: hop-by-hop lane diagnosis

Ticket 26/09/02/PUZZLE-3D-END-TO-END. Serve `http://127.0.0.1:6013/?plugin=puzzle3d`, guest wasm **build
#44-pre** (HEAD 46c3cb9, 14:30), host vite-live from the working tree. All browser evidence in this report
was produced by `🔍️b6-lane-probe.ts` (this wave's own probe — `🔍️browser-probe.ts` is mid-refactor by B1:
every step is `add(…)`-registered into a `plan` array that nothing iterates yet, so `--selection --brush
--clipboard` boots and exits after the boot verdict).

Outputs: `🗑️generated/b6-lane-probe-2026-09-11T12-5*.md`, `…T13-3*.md`, `🗑️generated/b6-vitest.txt`,
`🗑️generated/b6-vitest-suite.txt`, `🗑️generated/b6-cargo.txt`.

## 0 Verdict summary

| Lane | Verdict | Where | Live now? |
|---|---|---|---|
| 2 Brush preview `utility=select` | **Host defect found and FIXED** | `ShellHost.applyLeftoverInteractionView` wiped the armed utility | ✅ vite-live, verified in the browser |
| 4 Import distinct no-op | **Real no-op (not latency)** — guest fold | `importFixture` settles with `effects:0 historyUpserts:0` | ❌ guest tap added, needs next wasm |
| 3 Clipboard | **NOT broken** — the probe's verdict is a ~12 s world-lane latency | paste really creates `object-1` | n/a |
| 1 First-pick Inspection | **Guest defect, boundary proven** — host does everything right | persisted `vortex` selection is empty at panel render | ❌ witness added, needs next wasm |
| Fault `…has no exact pending spawn slot` | **Root-caused and FIXED** | direct-mapped `pending_reserved` residue collision | ✅ guest Rust, needs next wasm |
| Fault `1:framework.panel.history: AliasCapacity` | **Root-caused, NOT mine to fix** | UI-runtime alias credits exhausted by the growing history panel | — |
| Fault `interactionSelect … framework route` | same family as the spawn-slot fault | — | fixed with it |

---

## 1 First-pick Inspection empty

### Hop table

| # | Hop | Evidence | Verdict |
|---|---|---|---|
| 1 | canvas pick → `interactionSelect` admitted + reserved job | `performInvocation {actionId:"interactionSelect"}`, `spawn-job routed kind=framework.reserved.tool job=46 inline=1`, `job done … status=done steps=2` | OK |
| 2 | guest leftover carries the pick | `[DEBUG] leftover InteractionView {"selectedIds":["seed-left-001"],"gumball":true}` | OK — guest computed the selection |
| 3 | host observes leftover, bumps the Inspection epoch | `[DEBUG] leftover Inspection tab {"anchor":"top-right","path":["framework.panel.inspection"]}` then `[DEBUG] leftover Inspection refresh {"epoch":1}` | **OK — the host lane fires on the FIRST pick** |
| 4 | host issues a **full** refresh asking for the inspection panel | `b6 refresh panels {"scope":"full","gen":4,"panelCount":5,"asked":["framework.panel.inspection#5bd00904:1"]}` | OK |
| 5 | guest re-renders the panel surface | `got:["framework.panel.inspection#5bd00904:1=same"]` — retained surface hash unchanged | **FOLD** |
| 6 | is it a race? | 25 further full refreshes over **73 s** (canvas-orbit `setCamera` forced), hash stayed `5bd00904:1` the whole time | **not a race** |
| 7 | is it host refresh scheduling? | the panel DID repaint the same turn (`inspectorIds` went from `[framework.panel.inspection]` to the 4 summary rows) | **not the host** |

So the coordinator's two candidate hops are both cleared: the leftover is **not** publishing `selectedIds: []`,
and `leftoverInspectionRefreshScope`/the epoch effect **does** fire the first time. The fold is one hop later.

### Root cause (boundary proven, exact line still open)

`plugin_refresh_ui` is NOT what serves a browser refresh — `PluginRuntime.refreshUi`
(`🔌️PluginRuntime/🟦️.tsx:2403`) submits `surface-visible` events and then projects the **retained** surface
(`ownedUiRefreshResponse`, line 1810; `uiRefreshSectionUnchanged` answers `{key,hash}` with no value when the
retained hash still matches). `Event::SurfaceVisible` marks the surface dirty
(`⚛️reactor/🔄️turn/🦀️.rs:535`) and the dirty pass calls `plugin_render_surface`
(`🔌️plugin/🦀️.rs:31660`) → `instance.app.render(body_key, None, view_state)` → `VcsArtifactApp::render`
(line ~26027) → `Puzzle3dPlayApp::render_with_request_context` → `inspection::render(&envelope, interaction, …)`.

`Puzzle3dInteractionSnapshot::from_interaction` reads `interaction_state()` =
`interaction_store.snapshot()` + `interaction_hover`. `inspection::selected_section`'s final `.or_else`
matches **any** selected id against the fixture, and the fixture demonstrably contains `seed-left-001`
(`clipboard census before: {"count":1,"ids":["seed-left-001"]}`; the same panel renders its fields at the
LOCKED step). Therefore, at every one of those 26 renders, `interaction.selected` was **empty** — the
persisted `vortex` selection is not visible to the guest's own render after `interactionSelect` commits.

This is corroborated by the framework's own comment on
`leftover_interaction_view_from` (`🔌️plugin/🦀️.rs:21783`): *"Built from the just-computed InteractionView,
not the post-revalidate store snapshot, so a pick still lands on leftover when topology later prunes."* The
leftover is a workaround for exactly this loss; the Inspection panel has no such workaround.

Two candidates remain for the last line, and they are indistinguishable from the host wire:

- **(a) pruning** — `revalidate_and_persist_interaction_state` → `protocol::validate_state`
  (`📡️replication/📡️wire/🦀️.rs:2616`) drops every id `topo.contains(id)` rejects, and the `vortex` domain is
  `HierarchyProvider::Topology` (`✏️editor/🦀️.rs:8090`) → `Puzzle3dPlayApp::interaction_topology`
  (line 7755) reading `doc.snapshot.typed()`. An empty/stale topology prunes the pick silently.
- **(b) store round-trip** — `interaction_store.dispatch(ApplyInLane{lane: Interaction})` lands but
  `interaction_store.snapshot()` errs, and **every reader silently collapsed that to an empty state** via
  `.unwrap_or_default()` (5 call sites).

### What landed

- `🔌️plugin/🦀️.rs` — new `VcsArtifactApp::interaction_selection_snapshot()`: the store read failure is now
  NAMED (`[DEBUG] interaction-store snapshot unavailable, selection read as empty: …`) instead of collapsing
  into "nothing is selected". Replaces all five `interaction_store.snapshot().unwrap_or_default()` readers
  (`interaction_state`, both `dispatch_interaction_action` entries, the reserved-tool input, and the
  retained-command interaction input). This is a real silent-failure removal, not only instrumentation.
- `🔌️plugin/🦀️.rs` — `revalidate_and_persist_interaction_state` now emits ONE witness when the persist does
  not read back what was dispatched:
  `[DEBUG] interaction selection lost dispatched=… validated=… readback=… domains=vortex:N`.
  `dispatched` vs `validated` separates hypothesis (a) (pruning — ids present in `dispatched`, gone in
  `validated`, `domains=vortex:0`) from (b) (store — both carry the id, `readback` is empty).
  **Guest-side: this line only appears after the coordinator's next wasm build. It is temporary and must be
  removed by whoever closes lane 1.**

**Probe verdicts expected to flip:** none yet — `inspection-object-fields` / `inspection-locked-flag-row`
stay FAIL until the guest fix lands. The next wasm's console names the hop in one line.

---

## 2 Brush preview null — FIXED (host, live now)

### Hop table

| # | Hop | Evidence | Verdict |
|---|---|---|---|
| 1 | `#brush` pressed | `arm-brush post-click {"pressed":"true"}` | OK |
| 2 | host arms the window utility | `[DEBUG] setActiveUtility hop window=puzzle3d-main-perspective next=brush` | OK |
| 3 | host publishes the leftover overlay with the utility | `ShellHost:5717` `publishLeftoverWorldSelectionV1({…, activeUtility: next ?? "select"})` | OK |
| 4 | world lane reads it | `arm-brush poll 0 utility=brush` (battery, 247.3 s) | OK |
| 5 | any later leftover republish | `brush preview before storm … "utility":"select"` (battery, 258.8 s — after one camera frame) | **FOLD** |

### Root cause

`ShellHost.applyLeftoverInteractionView` (`🏛️ShellHost/🟦️.tsx:1887`) called
`publishLeftoverWorldSelectionV1({ ids, hoveredId, hoveredDomain, gumballActive, gumballAnchorId })` — with
**no `activeUtility` field**. `publishLeftoverWorldSelectionV1` REPLACES the module-level overlay wholesale
(`🌐️World3dHost/🟦️.tsx:1238`), and `mergeWorldInteractionWithLeftoverV1` only overlays `activeUtility` when
the leftover carries one — so the world lane fell back to the guest's `scene.interactionJson`, whose own
`activeUtility` is empty (guest witness: `[DEBUG] puzzle3d.brushPreview.lane utility= preview=0`).

The armed utility is host session state owned by `SET_ACTIVE_UTILITY_ACTION_ID`; it is not part of the
guest's `InteractionView`. So **every** `interactionSelect` / `interactionHover` / `setCamera` leftover — five
call sites of `applyLeftoverInteractionView` — disarmed Brush on the first pointer move after arming it.
`brushMode = activeUtility === "brush"` and `worldInstancePickBlocked(activeUtility)` are both keyed on it,
which is why the lane reported `hover=null`, `rawLen=0` and `brush-place hop: 0` for the whole storm.

### Fix

- `🌐️World3dHost/🟦️.tsx` — exported `LeftoverWorldSelectionOverlayV1` and added the pure
  `leftoverOverlayCarryingUtilityV1(next, prior)`: an overlay with no `activeUtility` of its own carries the
  prior one forward; an explicit `activeUtility` (including the disarm `"select"`) still wins.
- `🏛️ShellHost/🟦️.tsx` — `applyLeftoverInteractionView` publishes through that helper.

### Law

`🧪️tests/🔬️engine-contract/🟦️.ts` → `"carries the armed utility across an InteractionView leftover republish"`.

```
SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/…/🎯️targets/⚛️react/vitest.config.ts \
  --testNamePattern='carries the armed utility|leftover activeUtility'
 Test Files  1 passed | 22 skipped (23)
      Tests  2 passed | 865 skipped (867)
```

### Browser verification (host fix is vite-live — measured after the edit)

```
[10.0s] brush armed {"utility":"brush","hover":null,"previewLen":0}
[55.8s] pick landed at {"x":625,"y":293} selectedIds=["seed-left-001:v4"]
[55.8s] brush after pick     {"utility":"brush","hover":"seed-left-001:v4","previewLen":0}
[58.8s] brush after camera   {"utility":"brush","hover":"seed-left-001:v4","previewLen":0}
[59.8s] brush late hover 0.52 {"utility":"brush","hover":"seed-left-001:v4","previewLen":0}
[60.8s] brush hop-census place=2 utilityHops=["[DEBUG] setActiveUtility hop window=puzzle3d-main-perspective next=brush"]
```

Before this fix the same sequence read `utility=select hover=null` and `brush-place hop: 0`. The utility now
survives a pick leftover, a `setCamera` leftover and the hover storm; vortex hover reaches the lane and the
**brush place hop fires (2, was 0)**.

### Remaining guest half of this lane

`previewLen` is still 0: the guest never publishes a brush preview because the guest's own
`active_utility` is empty. Its witness names the exact miss:
`[DEBUG] puzzle3d.utility.publish action=registerBrushMesh window=Some("puzzle3d-main") utility= map_hit=false`
— `puzzle3d_scene_active_utility` (`✏️editor/🦀️.rs:905`) looks
`view_state.active_utility_by_window_id[wid]` up with `wid` resolved in `render_body`
(`✏️editor/🦀️.rs:7988`) as `window_id_from_key.or_else(|| config.window_ids.first())`, which on the
reducer/non-window-keyed paths yields the window **kind** `puzzle3d-main` while the host's map is keyed by
the window **instance** `puzzle3d-main-perspective`. That is a guest-side id-scope defect needing the next
wasm; `brush-preview-place` will not flip until it lands.

**Probe verdict expected to flip:** `brush-preview-place` — the `utility`/`hover`/`brush-place hop` half of it
is fixed now; the `preview=null` half waits on the guest id-scope fix above.

---

## 3 Clipboard — the feature works; the verdict measures latency

### (a) How copy/paste is reached

`copy`/`cut`/`paste` are **framework-owned** actions, declared in `clipboard_action_definitions()`
(`🔨️modules/🛂️manifest/🦀️.rs:1051`) with keybindings `mod+c` / `mod+x` / `mod+v`, localized labels and icons.
They are NOT puzzle3d menu rows — puzzle3d's own workspace menu authors `Duplicate Selection (mod+d)`,
`Delete Selection (backspace)`, `Selection ›`, `More ›`. So the battery's `copyBtn=0 copyById=0` is
**expected, not a defect**: there is no top-level Copy control by design, and the probe's `Meta+c` /`Meta+v`
fallback is the real user route (plus the command palette and the text context menu, which does render a
`text-copy` row with the `mod+c` shortcut, `🖱️ContextMenu/🟦️.tsx:785`).

### (b) Does paste add an instance? YES.

| # | Hop | Evidence |
|---|---|---|
| 1 | `mod+c` | `performInvocation {actionId:"copy"}` → `settled … historyCursor:5 historyUpserts:1`, `history patch applied … labels:["Copy"]` |
| 2 | `mod+v` | `performInvocation {actionId:"paste"}` → `settled … historyCursor:7 historyUpserts:2`, `labels:["Paste","create-object object { id=object-1 …"]` |
| 3 | world lane | `clipboard poll 0…6` still `{"count":1,"ids":["seed-left-001"]}`; **poll 7 (+12 s)** `{"count":2,"ids":["seed-left-001","object-1"]}` |

The battery's `clipboard FAIL delta=0` sampled at +14 s from the pick but only ~2 s after the paste settled;
the battery itself then recorded `instanceCount=2` nine seconds later at 138.8 s. Same in this wave's probe:
the object appears at **+12 s**.

Root cause of the lag is NOT the clipboard route: the world window body only republishes when the guest
re-renders that surface, and after the paste nothing dirties it until an unrelated action (the probe's next
camera frame) arrives. `dispatchDirectBrowserActorCommand` (`🏛️ShellHost/🟦️.tsx:4836`) — the actor path
these commands take — never calls `refreshUi` at all; the world lane rides retained UI patches only.

**Not fixed here** (it is the same "who dirties the window surface after a document mutation" question as
`gumball-scene-delta FAIL sceneDelta=false`, and it belongs with whoever owns the retained-surface dirty
policy). The actionable statement for the battery: the `clipboard` verdict must poll for the census change
(≥ 15 s) instead of sampling once, or the world surface must be dirtied by the paste's own `UiDirtyScope`.

---

## 4 Import distinct — a real no-op

| # | Hop | Evidence | Verdict |
|---|---|---|---|
| 1 | menu row → host arm | `[DEBUG] import-picker hop host-arm openImportFixture` | OK |
| 2 | file chooser → bytes | `[DEBUG] import-picker opened=1 name=…-distinct.json bytes=9177` | OK |
| 3 | host re-dispatch | `ShellHost:5990` `onAction({action:"importFixture", args:{payload, name}})` | OK |
| 4 | ingress with the right args | `[DEBUG] importFixture ingress {"payloadType":"string","payloadLen":9177,"argKeys":["payload","name","windowId"]}` | OK — payload, name AND windowId all present |
| 5 | guest command settles | `performInvocation settled {actionId:"importFixture", frames:2, historyUpserts:0, effects:0}` | **FOLD — zero mutations** |
| 6 | document | `import poll 0…19` over **30 s**: `{"count":1,"ids":["seed-left-001"]}` unchanged | no-op confirmed (unlike clipboard, this is NOT latency) |

So A1's audit is right that the fold is present and the host arm exists — and both host hops are correct: the
distinct payload reaches `importFixture` with the right args and window. The guest's own command produced no
operations. The native laws (`open_import_fixture_requests_file_open_then_import_applies_payload` and the
distinct-import law at `🔬️unit/🦀️.rs:4532`) pass on the same payload shape, so the divergence is in the
browser's retained/typed-operation route for this verb (`"importFixture" => Puzzle3dWindowCommandWork::new`,
`✏️editor/🦀️.rs:7655`; contract `lanes: &[Artifact]`, line 6939) — most likely the 9 177-byte `payload`
never reaching `import_fixture`'s `args` on the leftover-commit half, which fails closed to
`ctx.notice(import_invalid) + abort` (and notices do not surface in the browser — see the battery's
`locked-refusal-notice FAIL notices=[]`).

### What landed

`✏️editor/🎮️commands/📥️import-fixture/🦀️.rs` — two guest `[DEBUG]` taps that decide it in one line:
`[DEBUG] puzzle3d.import.ingress args=<bool> payload_len=<n>` at entry and
`[DEBUG] puzzle3d.import.parsed objects=<n> before=<n>` right before `ctx.scene.fixture = fixture`.
Together with the pre-existing `[DEBUG] puzzle3d.import.apply ops=… after_objects=…`
(`✏️editor/🦀️.rs:3280`) the next wasm distinguishes: no ingress line = the command never reached the
reducer; `args=false`/`payload_len=0` = the args were dropped on the retained-commit half; parsed line with
`objects=2` and `apply ops=0` = the fixture-delta bridge is the fold.
**Guest-side: these lines only appear after the coordinator's next wasm build; they are temporary and must be
removed by whoever closes lane 4.**

**Probe verdict expected to flip:** `import-distinct` — not yet.

---

## 5 The three new faults

### 5.1 `framework route 'interactionSelect' / 'interactionHover' has no exact pending spawn slot` — FIXED

`dispatch_framework_reserved_action` (`🔌️plugin/🦀️.rs:22313`) mints an operation id from the process-wide
counter and then tests `pending_reserved.can_insert(job)`. `ArtifactFixedRegistry` is **direct-mapped** on
`job % ARTIFACT_LIVE_OUTPUT_SLOTS` (= 64, line 15837/15893). `retire_pending_reserved_latest_wins`
(line 22270) may only retire pendings whose action IS an interaction verb — so a residue class held by any
other framework-reserved verb (`noteShellCommand`, which the shell records for **every** user command; an
in-flight `undo`; `revertToCommand`) faulted the whole route while up to 63 classes stood empty.

This is the exact sibling of B0's typed-operation slot fault, one table over: mint-then-test instead of
test-then-mint.

**Fix:** the admitting caller owns the id, so it now re-mints into a vacant residue class (bounded by the 64
classes, `retire_pending_reserved_latest_wins` still running first so an interaction storm keeps its
latest-wins retirement) and only refuses on true saturation, with a message that says so.

**Law** (`🔬️unit/🦀️.rs`):
`interaction_admits_in_every_residue_class_while_a_reserved_verb_stays_pending` — one unsettled
`noteShellCommand` holds a class, then 64 rounds of `select_id_unsettled` + `settle_reserved` cover every
residue class.

```
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib residue -- --test-threads=1
test editor::puzzle3d::component::tests::interaction_admits_in_every_residue_class_while_a_reserved_verb_stays_pending ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 675 filtered out; finished in 0.08s
```

Verified as real coverage: with the re-mint loop temporarily deleted the same law FAILS
(`test result: FAILED. 0 passed; 1 failed`), and passes again once restored.

Guest Rust → **rides the next wasm**.

### 5.2 `1:framework.panel.history: AliasCapacity` — root-caused, NOT fixed here

`SurfaceReconcileFault::AliasCapacity` (`🖱️ui/🧠️runtime/…/♻️reconcile.rs:616`, raised at line 801 and
1706) means `credited_clone()` returned `None` — the shared-alias credit budget for the surface's
`UiText`/`UiValue` aliases is exhausted while reconciling `framework.panel.history`. The history panel grows
without bound during a battery (31 entries by the import step), and it is the largest text-leaf tree in the
shell. It surfaces attached to whatever action happened to carry that turn's render (`interactionHover`
here), which is why it reads as an interaction fault.

Owner: the retained-UI alias-credit accounting / the history panel's own paging — not one of B6's four lanes
and not a one-line change (it is a framework-wide fixed-capacity constant). Recommended: bound the rendered
history entries the way the inspector already pages `IDS_ROWS`, or price the alias credits per surface.
Related to the standing note "one UI admission fault kills every later refresh".

### 5.3 `interactionSelect … framework route` — same family as 5.1, fixed with it.

---

## 6 Verification (all foreground)

| Command | Result |
|---|---|
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `Finished dev profile … in 33.06s`, **0 errors**, 88 warnings (proof the tree really expanded) |
| `cargo test … --lib residue -- --test-threads=1` | `1 passed; 0 failed` (and `1 failed` with the fix reverted) |
| `cargo test … --lib interaction -- --test-threads=1` | `9 passed; 0 failed; 667 filtered out` |
| `cargo test … --lib import -- --test-threads=1` | `7 passed; 0 failed; 669 filtered out` |
| `cargo test … --lib inspector -- --test-threads=1` | `10 passed; 0 failed; 666 filtered out` |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts --testNamePattern='carries the armed utility\|leftover activeUtility'` | `Test Files 1 passed \| 22 skipped`, `Tests 2 passed \| 865 skipped` |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts '🔌️PluginRuntime' '🔬️engine-contract'` | `Tests 3 failed \| 617 passed (620)` — the 3 are the pre-existing peer-owned failures B2 recorded verbatim (`buildNoteShellCommandAction`, `binds two instances of one body to distinct surfaces`, `readAppDocumentPack()` extra `ops` field); none is in this wave's diff |

Note: the root router no longer accepts `bun ./📜️script.ts test long --run …`
(`Unknown workspace test selection`) — vitest lanes must go through
`bun x vitest run --config 🧰️framework/…/🎯️targets/⚛️react/vitest.config.ts`, as B2 §6 recorded.

## 7 Files touched

| File | Change |
|---|---|
| `🧰️framework/…/🧱️elements/🌐️World3dHost/🟦️.tsx` | exported `LeftoverWorldSelectionOverlayV1`; new `leftoverOverlayCarryingUtilityV1` |
| `🧰️framework/…/🧱️elements/🏛️ShellHost/🟦️.tsx` | `applyLeftoverInteractionView` carries the armed utility forward |
| `🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts` | law: armed utility survives a leftover republish |
| `🧰️framework/…/🔨️modules/🔌️plugin/🦀️.rs` | reserved-spawn residue-class re-mint; `interaction_selection_snapshot` replaces 5 silent `unwrap_or_default()` readers; persist read-back witness |
| `✏️s/…/🧊️3d/…/🎮️commands/📥️import-fixture/🦀️.rs` | two `[DEBUG]` ingress/parse taps |
| `✏️s/…/🧊️3d/…/🧪️tests/🔬️unit/🦀️.rs` | law: interaction admits in every residue class |
| `T/🔍️b6-lane-probe.ts` | this wave's standalone lane probe |

The temporary **host** tap (`[DEBUG] b6 refresh panels`) that produced §1's evidence has been removed; the
suites were re-run after its removal with the same result. The **guest** `[DEBUG]` lines listed in §1 and §4
are deliberately still in the tree — they are unreadable until the coordinator's next wasm build and must be
swept once lanes 1 and 4 close.

⚠️ While proving the §5.1 law fails without its fix I restored `🔌️plugin/🦀️.rs` from a ~90 s old copy. B0's
hunks (`test_typed_operation_slot_preadmission`, `slot_is_vacant`) are present and the crate compiles, but B0
should re-check anything they wrote to that file between 15:38 and 15:40.
