# Wave B31 — a delete that keeps its victim selected, an Escape that cannot leave Fill, and a gumball drag that never moved along its axis

Implementation pass, 2026-09-12 04:4x–0x:xx CEST. Every reading below is real output from this pass. No git
write, ticket not opened/closed/reopened, `🗑️generated` written to and never deleted, no `[DEBUG] b31` tap
left anywhere (`rg "DEBUG\] b31|b31 "` over `🧰️framework`, `✏️s` and the probe → no match), every command
foreground.

**Headline** — the three product defects are root-caused to a named `file:line` and fixed, and two of them
turn out to be ONE framework defect:

| defect | root cause | state |
| --- | --- | --- |
| 1 `delete-selection` 30 s red | `🔌️plugin/🦀️.rs:22208` — the leftover selection overlay refills an EMPTY store selection, so an app-authored clear is undone on the next read | **fixed** (framework rule + guest arm + publication lane), law green, red proven |
| 1b HEAD's `every_advertised_engagement_verb_is_implemented` (`clear` arm) | the SAME line | **fixed by the same change**, green |
| 2 `engagement_abort` cannot leave Fill | `🎮️commands/🛑️engagement-abort/🦀️.rs:9-11` returned untouched while the fill tool was armed | **fixed**, semantics decided and stated, law green, red proven |
| 3 gumball drag dispatches nothing | the probe dragged `+72,+72`, which is ~perpendicular to the projected axis; and the host FABRICATED a 0.5 translate whenever the pose did not move | **fixed** (probe geometry + the fabrication deleted), law green, red proven |
| 4 probe repairs | `openHistory()` toggled the panel shut AND pressed Undo; `locked-refusal` never checked that the lock took | **fixed**; `selectionState()` was already repaired by a peer between 04:47 and 05:07 and is left alone |

---

## 1 Defect 1 — `deleteSelection` deletes the object and keeps it selected

### 1.1 What the browser actually showed

B29's red was `delete-selection FAIL before=2 after=2 waitedMs=30299`. The same run's console
(`🗑️generated/probe-2026-09-12T01-26-57.md:1093`) says the delete LANDED:

```
[DEBUG] history patch applied {"replace":false,"currentCursor":24,"patchCursor":25,"upserts":1,
                               "labels":["delete-object id=object-1"],"canUndo":true}
[DEBUG] completion apply {"operation":2176,"scope":{"kind":"partial", …,"windowBodies":["puzzle3d.play.composite"]}}
[DEBUG] performInvocation settled {"actionId":"deleteSelection", …,"effects":0}
```

`effects:0` rules out `refuse_without_selection` (a refusal pushes exactly one `Effect::Notify`), and the
history row names the object that went. So neither the keybinding, nor the action route, nor the guest arm,
nor the mutation, nor the completion scope was the defect — all five hops were already correct.

### 1.2 The law that found the real one

New puzzle3d testkit law, `✏️editor/🧪️tests/🔬️unit/🦀️.rs`
*`delete_selection_shrinks_the_world_census_and_drops_the_deleted_id`* — select → delete → assert the
RENDERED world census is −1, exactly one command-log row rode the settle, the deleted id is gone from the
framework-owned selection, and a second delete records nothing. Its first three assertions passed on HEAD
source; the fourth failed:

```
the deleted id must be gone from the selection, else a second Delete is a silent no-op:
  ["puzzle3d.object.ba7e6b1dd38ec927"]
```

So the guest census DOES shrink; the SELECTION is what does not move. Two `Delete` presses therefore delete
one object and then run an empty edit against a phantom id — and the census predicate could not fire in that
browser run because the probe's own `select()` had accepted a vortex suggestion (`create-object
id=puzzle3d.suggestion.f98…`, same log, cursor 23→24) between `beforeDelete` and the delete, so the count
went 2 → 3 → 2.

### 1.3 Root cause, measured in-process

`ctx.clear_selection()` (`✏️editor/🦀️.rs:2656`) emits a `MergeMode::Subtractive` `InteractionWrite` naming
exactly what is selected. Three `[DEBUG] b31` taps (since removed) through one delete:

```
[DEBUG] b31 snapshot  store=Some(["puzzle3d.object.ba7e…"]) overlay=Some(["puzzle3d.object.ba7e…"])
[DEBUG] b31 write     domain=vortex merge=Subtractive mode=Multiple targets=[…ba7e…] current=[…ba7e…] next=[]
[DEBUG] b31 revalidate minted=true before=Some([…ba7e…]) persisted=Some([])
[DEBUG] b31 snapshot  store=Some([])                     overlay=Some(["puzzle3d.object.ba7e…"])
```

The machine is right (`next=[]`), the store is right and immediate (`store=Some([])`) — and the very next
read puts the id back, off the overlay.

**`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22208-22220`**, `interaction_selection_snapshot`:

```rust
if let Some(overlay) = &self.interaction_leftover_selection {
    for (domain, selection) in &overlay.selection {
        let empty = state.selection.get(domain).map(|current| current.ids.is_empty()).unwrap_or(true);
        if empty && !selection.ids.is_empty() { state.selection.insert(domain.clone(), selection.clone()); …
```

That overlay exists for a real reason — a pick whose ids the store has not answered with yet, or that
`validate_state` pruned, still has to reach the render (wave B6/B23). But it is written ONLY by the
framework-reserved pick route (`dispatch_interaction_action`, `:22499`), so nothing ever retired it, and it
cannot tell "the store has not caught up with the pick" from "the app deliberately emptied the selection".
Every `ctx.clear_selection()` in every plugin was therefore unobservable. That is also, exactly,
`every_advertised_engagement_verb_is_implemented`'s HEAD red (`typing clear must empty the framework-owned
selection / left: 1 / right: 0`, B29 §2.4) — one defect, two symptoms.

### 1.4 Fix

- **`🔌️plugin/🦀️.rs`** — new pure rule beside `interaction_selection_loss_v1`:

  ```rust
  pub(crate) fn leftover_after_app_selection_write_v1(
      overlay: Option<&protocol::InteractionState>, leftover_ids: &[String],
      next: &protocol::InteractionState, domains: &[String],
  ) -> (Option<protocol::InteractionState>, Vec<String>)
  ```

  A write names the newest truth for its domain, so that domain's overlay entry BECOMES the freshly
  computed selection (empty included) and the flat `interaction_leftover_ids` keep only what is still
  selected somewhere. Domains the write never named are untouched, so an unrelated pick in flight keeps its
  cover. `apply_interaction_writes` applies it after the machine runs and before
  `revalidate_and_persist_interaction_state`.
- **`✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs`** — the arm calls `ctx.clear_selection()` after removing
  its victims. A delete that leaves them selected also leaves the gumball and the inspector bound to an
  object the world no longer carries.
- **`✏️editor/🦀️.rs:7120`** — `deleteSelection`'s publication contract gains
  `ArtifactToolPublicationLane::Interaction`. Without it the typed operation FAULTS outright
  (`🔌️plugin/🦀️.rs:24067`: "typed-operation emitted a store lane absent from its exact factory publication
  contract"), which is why the arm had no clear to begin with.
- **`🧫️fixtures/🔏️publication-authority/🔣️.json`** — the contract fixture is the audit's authority
  (`exactContracts`), so it is hand-corrected. It was **already stale at HEAD in five further ways** and all
  of them are repaired in the same pass, because the audit compares the whole map or nothing:
  `exportFixture`/`openImportFixture` (missing, `host-only`), `importFixture` (missing, `artifact`),
  `setPanelPage` (missing, `window-config`), `setActiveExample` (`artifact` → `artifact`+`config`),
  `engagementSubmit` (+`interaction`).
- **`🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts`** — the audit could not even REACH the contract
  comparison: its neutral window-ownership fixtures were missing `Puzzle3dWindowConfig.selectionMethod` and
  `Puzzle3dWindowTransient.activation`, both already required by the window schema, so it failed on Ajv
  first. Both added.

### 1.5 Laws + output

```
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 delete engagement
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 694 filtered out; finished in 0.54s
```

```
RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- --test-threads=1 an_app_selection_write interaction_selection_loss
test component::app::an_app_selection_write_retires_the_leftover_overlay_of_the_domains_it_names ... ok
test component::app::interaction_selection_loss_names_every_way_a_pick_can_disappear ... ok
test result: ok. 2 passed; 0 failed
```

Red proofs (both halves, each restored immediately after):

```
# the framework rule reduced to a no-op over its domains
assertion `left == right` failed: the written domain's overlay entry becomes the freshly computed
selection, empty included
  left: Some(["object-1"])  right: Some([])

# HEAD's arm, before ctx.clear_selection()
the deleted id must be gone from the selection, else a second Delete is a silent no-op:
  ["puzzle3d.object.ba7e6b1dd38ec927"]
```

Publication audit, red at HEAD for two independent reasons and green after:

```
bun ./📜️script.ts publication-authority-audit Puzzle3dPlayApp   (HEAD)
error: Puzzle3dWindowConfig neutral fixture failed Ajv validation: … missingProperty "selectionMethod"
error: Puzzle3dWindowTransient … missingProperty "activation"
error: Puzzle3dPlayApp publication authority diverged from the fixture

bun ./📜️script.ts publication-authority-audit                  (after)
validated Puzzle publication authority; owners=Puzzle2dPlayApp,Puzzle3dPlayApp,Puzzle5dPlayApp;
admitted=…; windowOwnershipCases=7; schema=Ajv; oracle=independent
```

---

## 2 Defect 2 — Escape could never leave the Fill tool

### 2.1 Root cause

`✏️editor/🎮️commands/🛑️engagement-abort/🦀️.rs:9-11` (at HEAD):

```rust
if puzzle3d_fill_tool_active(ctx.config) || ctx.scene.active_utility == fill_tool::TOOL_ID {
    return;
}
```

The rule it served is real and is stated in the epilogue (`✏️editor/🦀️.rs:3432-3436`): "leaving fill is
exclusively a host `setActiveTool \"\"` — an empty tool effect HERE bounce-disarms a just-armed fill". That
is correct for the epilogue's GENERIC utility-switch effect and wrong for this arm, because this arm IS the
gesture that means "leave what is armed". `config.active_tool_id` is host-owned, so it still read `fill`
when Escape arrived, and the arm returned untouched forever — B29 §2.5's
`engagement-abort rearmed=true activeUtility=brush waitedMs=15511`.

### 2.2 The semantics decided

Checklist §12/§14 (`📓️2026-09-09-user-feature-checklist.md`) put `Abort` as "in-flight engagement cancels",
and §12's Fill row carries a live background plan with its own cancel identity. So Escape while Fill is
armed now:

1. drops the typed engagement line and the brush candidate cursor (unchanged),
2. **cancels the in-flight fill plan** — `precompute.fill_job_identity()` then `cancel_fill_job_for(...)`,
   and `Effect::CancelJob { job }` only when the cancel actually took (a stale identity is a no-op, the same
   guard `cancelFillBuild` uses),
3. **disarms the tool** with one explicit `Effect::SetActiveTool { tool_id: "" }`, and resets the window
   utility to `PUZZLE3D_DEFAULT_UTILITY`.

It returns to NO tool and NO utility rather than to a remembered previous tool, because that is exactly
where the shell's own Escape leaves them (`🏛️ShellHost/🟦️.tsx` dispatches `SET_ACTIVE_TOOL ""`), and this
app keeps no per-window tool history — inventing one would be a new persisted field on a window schema two
peers are editing. The epilogue's generic rule is untouched: only this arm emits a tool effect.

Because `active_tool_id` is host-owned, the disarm is **idempotent**: a second Escape before the host has
answered repeats it rather than giving up. The law states that too, instead of pretending the guest can see
its own effect land.

### 2.3 Law + output

`✏️editor/🧪️tests/🔬️unit/🦀️.rs` — new
*`escaping_the_armed_fill_tool_cancels_the_plan_and_disarms_the_tool`*:

```
test editor::puzzle3d::component::tests::escaping_the_armed_fill_tool_cancels_the_plan_and_disarms_the_tool ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 722 filtered out
```

Red with HEAD's early return restored:

```
Escape with fill armed must disarm the tool: []
```

**One existing law states the OPPOSITE and is rewritten, not worked around.**
`fill_flow_does_not_emit_empty_set_active_tool` (wave W-AB) listed `engagementAbort` beside `fillBuildTick`
as a forbidden bounce — which is precisely the decision this wave overturns, and the only new red the wave
produced. It is now
*`fill_flow_only_disarms_the_tool_when_the_user_aborts`*: an empty `setActiveTool` IS a disarm, so
background traffic (`fillBuildTick`) and parameter traffic (`setFillCount`, added) must never emit one while
fill is armed, and `engagementAbort` must. Same file, same region, stronger statement.

```
test editor::puzzle3d::component::tests::fill_flow_only_disarms_the_tool_when_the_user_aborts ... ok
```

---

## 3 Defect 3 — the gumball drag

### 3.1 Two causes, one in each half

**Probe.** `dragGumballMoveX` pressed the axis and then dragged a fixed `+72,+72`. An axis handle only
answers the component of the pointer's travel that lies ALONG the axis as the camera projects it
(`🎬️Scene/🟦️.tsx` `gumballProjectRayOntoAxis`, then `param - state.startAxisParam`). B28's own stamp
measured origin `(474,418)` and the moveX tip `(449,447)`: the projected axis is `(-0.65, +0.76)`, so a
`(1,1)` drag keeps **11 %** of its length and the world delta lands at noise level. That is the
`gumball pose delta skipped {dx: 0, dy: 0}` reading.

**Host.** `🌐️World3dHost/🟦️.tsx:5723-5735` did not stop at the skip — it then **synthesized** a translate:

```ts
const step = 0.5;
const synthesized = { ...selectionArgs(), dx: axis === "moveX" ? step : 0, … };
return Promise.resolve(dispatch("translateSelection", synthesized));
```

A document edit the user never made, minted precisely when the gesture failed to say anything, and a
`gumball-scene-delta` that could go green on the fabrication instead of the drag. Deleted.

### 3.2 Fix

- **probe** — the drag runs along the projected axis: unit vector origin→tip × `AXIS_DRAG_PX = 96`, from
  each press fraction of B28's origin→tip scan (`1, 0.8, 0.6, 0.4`), every read still strictly after
  `mouse.up()`. The log now carries `gumball axis projection len=… stride=…` so a future zero delta can be
  read against the geometry that produced it.
- **host** — the zero-delta branch records the numbers and returns. Nothing is dispatched.

### 3.3 Law + output

`🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — new *"a gumball drag whose pose never moved commits nothing
— no fabricated axis step"*: the host's own `dispatchGumballPoseDelta` body is read off disk and its
zero-delta branch must contain neither a `translateSelection` dispatch nor a `synthesized` step, plus the
pure rule's `null` for an unmoved `moveY` pose.

```
bun x vitest run --config …/📺️renderer/…/⚛️react/vitest.config.ts -t "a gumball drag whose pose never moved"
 Test Files  1 passed | 28 skipped (29)
      Tests  1 passed | 947 skipped (948)
```

Red with the fabrication restored:

```
AssertionError: expected '"[DEBUG] gumball pose delta skipped",…' not to match /dispatch\(\s*["']translateSelection["…/
      Tests  1 failed | 947 skipped (948)
```

> Recorded for whoever takes the next gumball hop: a 02:37:53 run on this serve
> (`🗑️generated/probe-2026-09-12T02-37-53.ndjson`, not this wave's) shows
> `gumball-handle-enter PASS` and a REAL `[DEBUG] gumball pose delta {action: translateSelection…}` — no
> skip, no synthesis — while `gumball-scene-delta` still read
> `FAIL sceneDelta=false poseLen=266 waitedMs=30246`. So on that wasm a genuinely dispatched
> `translateSelection` did not move the world census either. With the fabrication gone and the drag now
> axis-aligned, that hop is the next one to measure, and it is downstream of everything this wave changed.

---

## 4 Item 4 — probe repairs

| repair | state |
| --- | --- |
| `selectionState()` counted mode-dock tabs as the selection | **already repaired by a peer** between 04:47 and 05:07 (tab strips excluded, `data-interaction-json` folded in) — left exactly as found |
| `openHistory()` toggled the panel shut on a second call | **fixed** — it presses the tab only when the tab is not active and no `framework.history.*` body is mounted |
| `openHistory()` pressed `#framework.history.undo` | **fixed — deleted.** A helper whose job is to LOOK at the history was undoing the mutation under test; that is B30's `import-distinct-records-history before=3 after=0 sections=[]` |
| `openHistory()` re-clicked already-expanded section headers, collapsing them | **fixed** — disclosures are pressed only while `aria-expanded="false"` |
| `locked-refusal` never checked that the lock took | **fixed** — the Inspection flag row (`📌️panels/🔍️inspection/🦀️.rs` `flag_row`, whose rendered value text IS the flag) is pressed and then polled to `true` for up to 15 s; the verdict note now carries `locked=… flag="…"`, so a refusal that never fired can no longer read the same as an object that was never locked |

Every step name and every verdict name is unchanged.

```
bun x tsc --noEmit … 🔍️browser-probe.ts
 the two pre-existing errors only (`import.meta.dir`, `mouse.click({ modifiers })`); no new ones
```

---

## 5 Verification

| command | result |
| --- | --- |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 delete engagement` | `29 passed; 0 failed` |
| same, whole `--lib` suite | `711 passed; 12 failed` — **zero new reds**, three fixed (§5.1) |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- --test-threads=1 interaction leftover selection` | `51 passed; 10 failed` — all ten are the peer's `interactive-job.missing-factory`/`output-envelope` family B25 §4 measured; this wave's law passes |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `Finished dev profile … in 31.28s`, **0 errors**, 88 warnings (pre-existing; their presence proves expansion completed) |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Finished dev profile … in 49.17s`, **0 errors** |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config …/📺️renderer/…/⚛️react/vitest.config.ts` | `Test Files 4 failed \| 25 passed (29)`, `Tests 10 failed \| 938 passed (948)` — nine are B4/B26/B27/B29's, the tenth is a peer's (§5.2); this wave's new law is among the 938 |
| `bun ./📜️script.ts publication-authority-audit` | green for all three owners; red at HEAD (§1.5) |
| `bun x tsc --noEmit … 🔍️browser-probe.ts` | the two pre-existing errors only |
| probe lanes | §6 |

### 5.1 The puzzle3d suite, name-diffed against a real baseline

A baseline was produced by temporarily disabling exactly this wave's three behaviour changes (the framework
overlay retirement, `delete_selection`'s `clear_selection`, the abort's new branch) and re-running the whole
suite; both failure lists were reduced to sorted names and `comm`-diffed
(`🗑️generated/b31-puzzle3d-lib-{baseline,final}-names.txt`).

```
baseline: test result: FAILED. 708 passed; 15 failed
final:    test result: FAILED. 711 passed; 12 failed

only in the wave run (regressions): NONE
only in the baseline (fixed by the wave):
  delete_selection_shrinks_the_world_census_and_drops_the_deleted_id      (this wave's law)
  escaping_the_armed_fill_tool_cancels_the_plan_and_disarms_the_tool      (this wave's law)
  every_advertised_engagement_verb_is_implemented                         (HEAD's red, §1.3)
```

The one regression the wave DID produce mid-pass — `fill_flow_does_not_emit_empty_set_active_tool` — was
the law encoding the overturned decision; it is rewritten in §2.3 and green, which is why it appears in
neither column.

Two of the twelve remaining reds were checked individually against the same disabled-retirement baseline and
fail identically without this wave's change, so they are not its:

```
world_pick_null_clears_without_reselecting_first_object            FAILED with the retirement disabled
world_vortices_reveal_in_selected_mode_only_for_the_selected_object FAILED with the retirement disabled
```

`gumball_active_only_for_transform_utilities_with_object_selection` is B25 §4's recorded red (the editor
peer, live in those files). `selected_object_inspector_renders_that_object_field_group` passes in isolation
and is order-dependent.

### 5.2 The renderer lane's tenth failure is a peer's

```
FAIL …/🔬️engine-contract/🟦️.ts > noteShellCommand > buildNoteShellCommandAction …
- Expected  + Received
+     "inverseArgs": { "windowId": "w1" },
+     "inverseCommandId": "shell.windowClose",
```

`buildNoteShellCommandAction` grew an inverse pair and its law was not updated. Nothing this wave touched is
on that path.

### 5.3 One thing found while reading the serve log

`🗑️generated/serve-6013-2026-09-12-b.txt` ends (05:11) on an esbuild transform failure in
`🌐️World3dHost/🟦️.tsx:5346` — `Expected ";" but found ")"`, a peer's in-flight edit. It was already
repaired by 05:19 and this wave's own hunk in that file survived intact (checked line by line). Recorded
because a probe run inside that window would have measured a host that could not transform.

---

## 6 Probe lanes

All against the live `:6013` React serve (vite-live host, **wasm #51** — this wave's four guest/SDK `.rs`
files ride the NEXT wasm, so every verdict below measures #51's guest plus the vite-live host).

```
bun 🔍️browser-probe.ts --only=boot --port=6013
[6.9s] verdict boot PASS · battery-hard-faults PASS · battery-faults PASS
[6.9s] done booted=true faults=0 hard=0 collateral=0 verdicts=6
```

```
--only=selection-keybindings,gumball-drag,locked-refusal,export-import --port=6013
                                            (probe-2026-09-12T03-27-18, 310.1s)
PASS boot
FAIL locked-flag-row            lockChrome=false
FAIL locked-refusal-notice      locked=false flag="locked false" notices=[] waitedMs=30441
PASS gumball-handle-enter       handle={"kind":"moveX","sx":448,"sy":538,"ndcZ":0.971} entered=true taps=2
FAIL gumball-scene-delta        sceneDelta=false poseLen=282 waitedMs=30242
FAIL duplicate-selection        before=1 after=1 waitedMs=30393
FAIL duplicate-reselects-clone  selected=[]
PASS focus-selection            camera moved
PASS delete-selection           before=3 after=2 waitedMs=19932
PASS export-only                download=concrete-forest.json
PASS export-names-the-example   download=concrete-forest.json expected=concrete-forest.json
PASS import-same-file-idempotent before=2 after=2
FAIL import-distinct            before=2 after=2  (guest: import.apply ops=0 after_objects=2)
PASS import-distinct-records-history before=9 after=10
PASS guest-alive-mutate · PASS guest-alive-replace · PASS battery-hard-faults · PASS battery-faults
```

```
--only=selection-keybindings --port=6013     (probe-2026-09-12T03-42-37, 98.0s, after the select() repair)
PASS boot                       6.6s
     selection precondition attempt=0 ids=["seed-left-001","seed-left-001"] waitedMs=16
PASS duplicate-selection        selected=["seed-left-001","seed-left-001"] before=1 after=2 waitedMs=17537
FAIL duplicate-reselects-clone  selected=[]
PASS focus-selection            camera moved
     selection precondition attempt=0 ids=["object-1","object-1"] waitedMs=25
PASS delete-selection           selected=["object-1","object-1"] before=3 after=2 waitedMs=10811
PASS guest-alive-mutate · PASS battery-hard-faults (hard=0 collateral=0) · PASS battery-faults (raw=0)
```

### 6.1 What the lanes proved about this wave

- **`delete-selection` is GREEN, twice, on #51** — `before=3 after=2 waitedMs=19932` and
  `before=3 after=2 waitedMs=10811`, with the selection it acted on printed in the note. That is the direct
  confirmation of §1.1/§1.2: the census hop was never broken. B29's 30 s red was the lane's own
  precondition — its `select()` accepted a vortex suggestion between `beforeDelete` and the press, so the
  census went 2 → 3 → 2 and `count < before` could never fire — plus the phantom re-delete this wave fixes
  in the guest.
- **`duplicate-selection` flipped FAIL → PASS purely on the precondition repair**: `before=1 after=1` with
  `selected=[]` in the first run, `before=1 after=2` with `selected=["seed-left-001"]` in the second. The
  guest was refusing an empty selection, exactly as it should.
- **`import-distinct-records-history` flipped FAIL → PASS** (`before=9 after=10`): `openHistory()` no longer
  toggles the panel shut and no longer presses Undo. The panel state is logged
  (`history panel opened active=false bodyBefore=0` on the first call, `active=true bodyBefore=15` and
  `bodyBefore=16` on the later ones — it correctly declined to press the tab again).
- **The gumball drag now grabs and produces a real delta.** The axis-aligned stride is what did it:

  ```
  gumball handle {"kind":"moveX","sx":448,"sy":538} origin={"kind":"origin","sx":474,"sy":514}
  gumball axis projection len=35 stride=-71,65
  gumball grabbed tried=["1@448,538=false","0.8@453,533=false","0.6@458,528=true"] stride=-71,65
  gumball hops entered=[…moveZ…, …moveX…]
            delta=["[DEBUG] gumball pose delta {action: translateSelection, ids: Array(1), mode: mesh}",
                   "[DEBUG] gumball pose delta {action: translateSelection, ids: Array(1), mode: mesh}"]
  ```

  No `skipped`, no `synthesized` — two REAL `translateSelection` dispatches off a 96 px axis drag, where
  B28/B29's `+72,+72` produced `dx: 0, dy: 0`. `gumball-scene-delta` is still FAIL
  (`sceneDelta=false poseLen=282 waitedMs=30242`): a genuinely dispatched `translateSelection` with a real
  delta did not move `data-instances-json` in 30 s. That is now a clean, isolated hop — see §7.
- **`locked-refusal` names its own missing precondition** instead of blaming the notice:
  `locked-flag-row FAIL lockChrome=false` and `locked-refusal-notice FAIL locked=false flag="locked false"
  notices=[] waitedMs=30441`. The inspection panel rendered no `object.locked` row at all
  (`lock flag row="locked false" locked=false waitedMs=15427`), so nothing was ever locked and there was no
  refusal to render. B28 §5's host `Effect::Notify` → `showTransientNotice` chain stays correct and stays
  unexercised.

### 6.2 Which verdicts wait for the next wasm

This wave's guest + SDK changes (`delete_selection`'s clear, the `Interaction` lane, the leftover-overlay
retirement, the abort's disarm/cancel) are **not in #51**. These wait for it:

| verdict | what the next wasm changes |
| --- | --- |
| `delete-selection` | already green; the wasm removes the phantom re-delete (Delete twice can no longer land an empty edit on a deleted id) and the stale gumball/inspector binding after a delete |
| `engagement-abort` | the whole defect — unmeasured on #51 by design, since #51's guest still returns untouched while fill is armed |
| `duplicate-reselects-clone` | possibly: the overlay retirement changes what a post-duplicate read sees, but the store itself answering empty (§7 item 1) is upstream of it |

### 6.3 One lane run could not boot, and why

Between the two runs above, two consecutive `--only=selection-keybindings` attempts failed at boot with
`windows=0 canvases=0 faults=1`:

```
pageerror: SyntaxError: The requested module '…/🏛️ShellHost/🔀️surface-switch/🟦️.ts'
  does not provide an export named 'createSessionWorkLedgerV1'
```

`:6013` itself answered `http=200 t=0.006`, so the serve was healthy — a peer was live in
`🔀️surface-switch/🟦️.ts` and `🏛️ShellHost/🟦️.tsx` (mtimes 05:39 and 05:33, seconds before each attempt).
The serve was NOT restarted. The two files' mtimes were polled until stable for 60 s and the lane was then
re-run once, which is the clean 03:42:37 run above. Also recorded in §5.3: the same file pair had a
transform error at 05:11, and this wave's own hunk in `🌐️World3dHost/🟦️.tsx` survived both peer passes
intact.

---

## 7 Honest gaps

1. **`duplicate-reselects-clone` is the sharpest remaining red in this family**: `selected=[]` after a
   duplicate whose census DID grow. The guest law `duplicate_selection_reselects_the_created_clones` passes
   in-process, so `replace_selection(clone_ids)` is emitted correctly; what the pane paints is empty. Since
   the store answering non-empty is what makes the leftover overlay stand down, an empty store here is
   upstream of this wave's overlay rule — the next hop is whether the `Interaction` lane publication for
   `duplicateSelection` lands, or `validate_state` prunes a clone id that is not yet in topology at write
   time.
2. **`gumball-scene-delta` is now a single clean hop**: the handle grabs, the pose moves, a real
   `translateSelection` is dispatched with a non-zero delta, and the world census does not move for 30 s. No
   fabrication is left to hide it. Everything upstream of the dispatch is proven; everything downstream is
   unmeasured by this wave.
3. **`engagement-abort` is unmeasured live** — it rides the next wasm by construction (§6.2). Its law is
   green in-process with a proven red.
4. **`locked-flag-row` / `locked-refusal-notice` stay red on a missing precondition** (the inspection panel
   renders no `object.locked` row for the picked object). Named, not fixed: that is the inspection/leftover
   population lane, not this wave's.
5. **`import-distinct` stays red for the reason B29/B30 named** — the guest's own taps in this run say
   `import.parsed objects=2 before=2` → `import.apply ops=0`, then a distinct payload `objects=3 before=2` →
   `ops=1 after_objects=3`. The identity re-import correctly emits nothing; the verdict's own expectation of
   which payload is "distinct" is what is off, and it belongs to the import lane.
6. **The `select()` precondition repair was measured once** (two calls, both `attempt=0`). Its retry branch
   (`attempt=1`/`2`) and its give-up branch have not fired on a live run, so they are unproven paths.
7. **The `framework-plugin` suite was not name-diffed against its own baseline.** Its ten
   interaction-filtered reds all fail at their first `dispatch_typed` with
   `interactive-job.missing-factory`/`output-envelope`, which is B25 §4's recorded peer family and is
   upstream of any selection-machine behaviour, but the full-suite diff this wave did for puzzle3d (§5.1)
   was not repeated there.
8. **Only one of the two ways a zero-delta drag can be caught is a law.** The engine-contract law reads the
   host's own zero-delta branch off disk — a text contract, which catches a re-introduced fabrication but
   cannot catch a NEW dispatch added elsewhere in the same callback. jsdom performs no layout and the
   gumball lives inside the stubbed r3f canvas, so the real gesture is only ever measured in the browser.

---

## 8 Files

Product (guest, **rides the next wasm**):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛑️engagement-abort/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (plugin SDK — compiled into the guest)

Product (host, vite-live now):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`

Laws:

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (the rule's own law)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`

Fixtures / audit:

- `✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts`

Probe (this wave owns it):

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts`
