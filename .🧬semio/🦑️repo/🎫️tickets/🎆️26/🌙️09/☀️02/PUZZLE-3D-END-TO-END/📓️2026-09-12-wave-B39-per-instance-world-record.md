# Wave B39 — the world record per window INSTANCE

Ticket `26/09/02/PUZZLE-3D-END-TO-END` · 2026-09-12 · host vite-live on `:6013`.
Everything below ran in the FOREGROUND with its tail quoted.

---

## 1 The keying chain, `file:line`

The chain B37 handed over is per window INSTANCE at **every** hop — the shared record it suspected
does not exist. Traced end to end:

| hop | `file:line` | key |
|---|---|---|
| guest body render | `✏️s/…/✏️editor/🦀️.rs:8218` `render_body(body_key, …)`, `:8227` `body_key.split_once(':')`, `:8229` `puzzle3d_addressed_window_id(view_state, window_id_from_key, …)` | body key + the view's OWN `window_id` |
| guest surface table | `🧰️framework/…/🔌️plugin/⚛️reactor/🪟️surfaces/🦀️.rs:42` `SurfaceBinding{surface, body_key, view_state}`, `:81` `role.project(&view_state)` | one slot per SURFACE, each with its own projected `ViewModel` |
| guest mount | `🔌️plugin/🦀️.rs:32555` `plugin_show_surface` → `view.for_window_instance(window)`, `:32574` `plugin_render_surface(surface)` | surface |
| guest retained patches | `⚛️reactor/🩹️patches/🦀️.rs:341` slot lookup `slot.surface.as_ref() == surface` | surface |
| host intake | `🔌️PluginRuntime/🟦️.tsx:1823` `wirePatchSurfaceId(patch)`, `:1849` `surfaces.set(surfaceId, surface)` | `instance:windowKey` |
| host mount ask | `🔌️PluginRuntime/🟦️.tsx:1681` `windowHostContextBindings(…)`, `:1570` `pluginSurfaceRef(instanceId, target.key)` | window INSTANCE key |
| host projection | `🔌️PluginRuntime/🟦️.tsx:1906` `surfaces.get(retainedSurfaceId(instanceId, target.key))` | window INSTANCE key |
| host request/cache | `🛠️ShellHelpers/🟦️.tsx:4782` `sessionWindowInstances`, `:4827` `cache.get('window:${instance.id}')` | window INSTANCE id |
| host store | `🏛️ShellHost/🟦️.tsx:4667` `cache.get('window:${instance.id}')`, `:9869`/`:9915` `builtNodeStoreFor('window:${id}', …)`, `:1848` `new UiDocumentStore(key)` | one `UiDocumentStore` per instance |
| host record identity | `🗣️Interpreter/🟦️.tsx:417` `surfaceHostIdentityV1(surface, record.key, record.id)` | `surfaceId = store surface`, `paneId = record.key` |

`recordKey: puzzle.3d.play.viewport` in B37 §5 is `record.key` — the *authored pane id*
(`🪟️windows/🧊️main/🦀️.rs:34 SURFACE_VIEWPORT`), which is B20's `paneId` and is identical in both
panes **by design**. The probe's own output proves the two panes are two stores:

```
panes=[{"window":"puzzle3d-main-top","surface":"window:puzzle3d-main-top","activeUtility":"select"},
       {"window":"puzzle3d-main-perspective","surface":"window:puzzle3d-main-perspective","activeUtility":"select"}]
```
(`🗑️generated/probe-2026-09-12T07-03-20.md`, verdict `engagement-brush-verb`)

## 2 Root cause — the host LEFTOVER overlay is a module singleton

The record was never shared. What IS shared — one module-level slot for every pane of every
document — is the host's **leftover InteractionView overlay**, and it is the last thing laid over the
guest's per-pane record before it reaches the DOM:

```
🌐️World3dHost/🟦️.tsx:1246   let leftoverWorldSelectionOverlay: LeftoverWorldSelectionOverlayV1 | null = null;   ← ONE slot
🌐️World3dHost/🟦️.tsx:4794   const interaction = useMemo(() => mergeWorldInteractionWithLeftoverV1(
                              parseInteraction(scene?.interactionJson), leftoverWorldSelectionOverlayV1()), …);
🌐️World3dHost/🟦️.tsx:1347   mergeWorldInteractionWithLeftoverV1 → { ...base, activeUtility }   ← overrides the pane's own
🌐️World3dHost/🟦️.tsx:6335   data-interaction-json={JSON.stringify(interaction)}
```

`mergeWorldInteractionWithLeftoverV1` overrides exactly the two fields the probe reads
(`activeUtility`, `hoveredVortexFullId`) and `brushPreviewJson` is read off the same slot
(`:4801`) — so both panes published **whatever the last publication anywhere in the app said**.
Every publication site wrote it globally: `🏛️ShellHost/🟦️.tsx:6095` (`SET_ACTIVE_UTILITY`, which
HAS the `windowId` and threw it away), `:6152` (`SET_ACTIVE_TOOL`), `:1999`
(`applyLeftoverInteractionView`, on the response of every guest action). That is why
`engagement-brush-verb` reads `select` in BOTH panes when it fails and `brush` in BOTH when it
passes: the `registerBrushMesh`/`setCamera` leftover from `puzzle3d-main-top` (`utility=` /
`map_hit=false`, visible in every probe log) lands on the same slot after the arm and wipes it.

The doc comment at `:1372` already stated the intended contract — "`activeUtility` rides along
because it is per window instance too (wave B9)" — while the slot it rode from was global.

Also fixed on the way: `brushPreviewJson` was read and written on
`LeftoverWorldSelectionOverlayV1` (`:1275`, `:4801`) but never declared on it.

## 3 Fix — the overlay is a per-instance registry with a declared scope

`🌐️World3dHost/🟦️.tsx`:

- `LEFTOVER_WORLD_WINDOW_FIELDS` names the four fields ONE pane owns (`hoveredId`,
  `hoveredDomain`, `activeUtility`, `brushPreviewJson`); everything else in the overlay is the
  document's (`ids`/`gumball*` are a document selection per B20, `activeToolId` is a mode-level tool).
- `LeftoverWorldOverlayScopeV1` = `window` | `document` | `allWindows`, and
  `publishLeftoverWorldSelectionV1(overlay, scope)` now demands one. A `window` publication writes
  only that pane's entry and hands the document slot the publication's SHARED fields while keeping
  its own window fields; `document` touches no pane; `allWindows` (a mode-level tool, which has
  just cleared every window's utility) replaces them all.
- `leftoverWorldWindowOverlayV1(windowId)` is what a pane reads — the document's shared fields under
  its own window fields, falling back to the document-wide ones for a pane that published none.
- `leftoverWorldSelectionOverlayV1()` keeps its name and is now explicitly the DOCUMENT-scoped read
  (Outliner/Inspection tree ids, the armed mode tool).
- `leftoverWorldArmedWindowOverlayV1()` / `leftoverWorldArmedWindowIdV1()` answer "which pane is
  armed" for the host's brush-preview refresh lane, which used to read the global slot.
- every pane read is now pane-scoped: `interaction` (`:4853`), `selection` (`:4849`, via
  `mergeWorldSelectionWithLeftover(base, windowId, instances)`), `brushPreviewJson` (`:4858`),
  and both hover dispatchers' armed-brush guards (`:5546`, `:5560`). `leftoverArmedToolId` (`:5500`)
  stays document-scoped — a tool is not a window's.

`🏛️ShellHost/🟦️.tsx`:

- `applyLeftoverInteractionView(output, actionId?, windowId?)` — the pane the action addressed
  publishes under `{kind:"window"}`, a windowless action under `{kind:"document"}` so it can no
  longer disarm a pane. Threaded from the world-pane dispatch route (`:6427`, `dispatchWindowId`,
  which `🌐️World3dHost`'s own `dispatch`/`dispatchSettled` always stamp with
  `windowId: windowInstanceId`) and from the `SET_ACTIVE_UTILITY` follow-up (`:6141`).
- `SET_ACTIVE_UTILITY` publishes `{kind:"window", windowId}` and carries the PANE's prior overlay
  forward (`:6100`); `SET_ACTIVE_TOOL` publishes `{kind:"allWindows"}` (`:6161`).
- the four brush-preview refresh readers (`:1950`, `:4608`, `:9425`, `:9440`, `:9454`) read
  `leftoverWorldArmedWindowOverlayV1()`.

## 4 The second defect the first fix exposed — one arm authority

With the overlay per pane, the isolated battery still read `select` in BOTH panes
(`🗑️generated/probe-2026-09-12T07-56-42.md`) — no longer a bleed, a MISSING publication:

- `🏛️ShellHost/🟦️.tsx:6135` (the utility-bar action) published the pane's overlay itself.
- `🏛️ShellHost/🟦️.tsx:5235` (the guest's own `setActiveUtility` EFFECT — what an engagement `brush`
  verb raises, the effect whose refresh B37 fixed) updated `activeUtilityByWindowIdRef` + the store
  and published NOTHING. The pane's overlay kept the `select` an earlier `interactionHover` /
  `setCamera` leftover left there, and `mergeWorldInteractionWithLeftoverV1` laid that stale `select`
  over the guest body that already said `brush`.

Fix: the leftover publication moved INTO `setActiveUtilityForWindow` (`:4026`) — the ONE place one
window's arm changes, which both routes already call — and the action route's own duplicate
publication is gone. One authority, one publication, always `{kind:"window", windowId}`.

## 5 Laws

| law | file | output |
|---|---|---|
| guest: two mounted instances publish two DISTINCT world bodies with their own utilities, through the `<body>:<instance>` surface route | `✏️editor/🧪️tests/🔬️unit/🦀️.rs` `two_mounted_instances_publish_two_distinct_world_bodies_with_their_own_utilities` | `test result: ok. 2 passed; 0 failed; … 730 filtered out` (with B9's `each_window_instance_publishes_its_own_armed_utility_into_its_world_lane`) |
| host: the leftover overlay is per window INSTANCE — two panes bind two records, arming one leaves its sibling untouched, a windowless leftover cannot disarm a pane, a mode tool speaks for all | `🧪️tests/🔬️engine-contract/🟦️.ts` | `Tests 2 passed \| 577 skipped (579)`; **proven to FAIL without the fix**: `AssertionError: the armed pane's own record: expected 'select' to be 'brush'` |
| host: one arm authority publishes the armed pane's own overlay; every publication names its scope | `🧪️tests/🔬️engine-contract/🟦️.ts` | same run; proven live (§6: the same battery FAIL→PASS across this one change) |

Gates (foreground, tails quoted):

| command | tail |
|---|---|
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 two_mounted_instances each_window_instance` | `test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 730 filtered out; finished in 0.67s` |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: … generated 88 warnings` / `Finished \`dev\` profile … in 13.11s` — 0 errors |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Checking semio-s-plugin-puzzle v0.1.0` / `Finished \`dev\` profile … in 15.21s` |
| renderer-react `engine-contract` (new laws) | `Test Files 1 passed (1)` / `Tests 2 passed \| 577 skipped (579)` |
| renderer-react `engine-contract` unfiltered | `Tests 7 failed \| 572 passed (579)` — the SAME 7 peer reds B37 reported (6 × `extension invocation completion ownership`, 1 × `buildNoteShellCommandAction`), down from B37's 10 as that peer lands work. **No new failure**, none in this wave's regions. |
| renderer-react `world3d-interaction` | `Tests 14 passed (14)` |
| `bun x tsc --noEmit -p tsconfig.json` (renderer-react) | filtered to the two edited files: `World3dHost(1366,59)`, `(3411,12)`, `(4336,12)`, `(4673,42)`, `ShellHost(2007,52)`, `(8286,35)`, `(8287,116)`, `(8998,59)` — B37's identical pre-existing set at drifted lines (they named `1987,79`/`1999,52`/`8212,35`/`8213,116`/`8924,59`). **No error in any line this wave wrote**, and one of B37's two `applyLeftoverInteractionView` errors is gone. |

## 6 Live — the two panes diverge for the first time

wasm **#56** (`🗑️generated/deploy-2026-09-12-56.txt`, `[deploy] done … 09:51:58`). The guest half rides
#56 **unchanged** — this wave wrote no guest product code (its only `🦀️.rs` edit is the new law), so
the whole fix is host vite-live.

> The first attempt (`probe-2026-09-12T07-51-34`) is void: the coordinator's #56 materialize landed
> AT 09:51 while it ran, and the swap revoked the actor
> (`actor-activation.revoked`, `no channel for instance 1`, `canvases=0`, `panes=[]`).

1. **Per-instance record, proven** — `probe-2026-09-12T07-54-36.md`, the first run on the settled
   #56 and still BEFORE §4's fix. Volume-Brush armed in the perspective pane through the utility bar:

```
verdict engagement-brush-verb FAIL activeUtility=volumeBrush panes=[
  {"window":"puzzle3d-main-top","surface":"window:puzzle3d-main-top","activeUtility":"select"},
  {"window":"puzzle3d-main-perspective","surface":"window:puzzle3d-main-perspective","activeUtility":"volumeBrush"}]
```

   The two panes read DIFFERENT values for the first time in this ticket. Every earlier run in every
   earlier wave reported one value in both (`select`/`select` when red, `brush`/`brush` when green).
   `surface` is each pane's own `📃️UiDocumentStore` key, so this line is also the per-pane record read
   task 1 asked for; `data-window-instance-id` is the `window` field.

2. **The residual, and its fix** — the same probe, isolated (`brush-stroke,engagement-bar`):

| run | change in between | `engagement-brush-verb` |
|---|---|---|
| `probe-2026-09-12T07-56-42` | overlay per instance only | `FAIL activeUtility=select panes=[…top…select, …perspective…select]` |
| `probe-2026-09-12T07-59-41` | + §4 one arm authority | `PASS` — `battery PASS=11 FAIL=0 FAULTS=0` |

3. **Verdicts, `--only=brush-stroke,engagement-bar,volume-brush --port=6013`**
   (`probe-2026-09-12T08-00-31`, repeated stable as `…T08-06-04`):

```
battery PASS=15 FAIL=0 FAULTS=0 first-hard-fault-at=none guest-death-faults=0
boot PASS · brush-preview-place PASS · volume-brush-arm PASS ·
volume-brush-target-volume-attribute PASS · volume-brush-add-target-volume PASS ·
volume-brush-voxel-dims PASS · engagement-input-present PASS ·
engagement-placeholder-has-no-dead-verbs PASS · engagement-brush-verb PASS ·
engagement-clear-is-a-noop PASS · engagement-fill-verb PASS · engagement-abort PASS ·
guest-alive-mutate PASS · battery-hard-faults PASS · battery-faults PASS
```

`engagement-brush-verb` (B37's handover) and `engagement-abort` (B31's red, still open at B35 §3.4
and B37 §5) are both green, with zero faults.

## 7 Files

Product:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
  — `LEFTOVER_WORLD_WINDOW_FIELDS`, `LeftoverWorldOverlayScopeV1`,
  `leftoverWorldOverlayByWindow`/`leftoverWorldDocumentOverlay`,
  `leftoverWorldOverlayForWindowFieldsV1`, `publishLeftoverWorldSelectionV1(overlay, scope)`,
  `leftoverWorldWindowOverlayV1`, `leftoverWorldArmedWindowOverlayV1`,
  `leftoverWorldArmedWindowIdV1`, `brushPreviewJson` declared on the overlay, and every pane read
  (`interaction`, `selection`, `brushPreviewJson`, both hover dispatchers) pane-scoped.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
  — `setActiveUtilityForWindow` is the one arm authority and publishes the pane's overlay;
  `applyLeftoverInteractionView(output, actionId?, windowId?)` threaded from the world-pane dispatch
  route and the utility follow-up; `SET_ACTIVE_TOOL` publishes `allWindows`; the brush-preview
  refresh readers read `leftoverWorldArmedWindowOverlayV1()`.

Laws:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` (2 new)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (1 new)

Ticket: `🗑️generated/probe-2026-09-12T07-5{1,4}-*`, `…T07-56-42.*`, `…T07-59-41.*`, `…T08-00-31.*`,
`…T08-06-04.*`.

## 8 For the fleet

1. **The world record was ALWAYS per instance.** Guest surface table, host intake, host projection,
   host refresh cache, host store — every hop keys by the window instance (§1). `record.key`
   (`puzzle.3d.play.viewport`) is B20's `paneId` and is identical in both panes by design; it is not
   evidence of a shared record. The probe's `surface":"window:<instance>"` field is.
2. **Two panes reading one value is a shared-state signature, but look at what is laid over the
   record last, not at the record.** Here it was one module-level `let` in `🌐️World3dHost` — host
   state with no pane in its key at all.
3. **Host state the guest reads back needs ONE authority.** Two routes wrote the arm
   (`SET_ACTIVE_UTILITY` action, `setActiveUtility` effect) and only one of them published the
   overlay the pane paints from. B37's lesson was that such an effect must earn its refresh; this
   wave's is that it must go through the same authority, or it earns a refresh that publishes a value
   the host then masks.
4. Two open reds from earlier waves (`engagement-brush-verb`, `engagement-abort`) had ONE cause
   between them, and it was neither a refresh nor a guest defect.

## 9 Pre-existing reds seen but NOT touched

- `semio-s-artifact-puzzle-3d --lib`: `the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind`
  and `two_instances_converge_disjoint_object_edits_via_backbone` both FAIL **in isolation**
  (`0 passed; 1 failed; … 732 filtered out`), so they are not order effects of the new law and not
  this wave's. The first asserts a roster fallback (`left: Some("puzzle3d-main-top")` /
  `right: Some("puzzle3d-main")`).
- renderer-react `engine-contract`: the 7 peer reds named in §5.
