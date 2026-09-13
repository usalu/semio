# 🕹️ wgpu WORLD3D INTERACTION — hover, selection and camera, from the pointer to the guest's door

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane **wgpu-world3d-interaction**, 2026-09-13.
Target: `http://127.0.0.1:6118/?plugin=generation3d` (editor) and `…&role=viewer`, the coordinator's wgpu serve.

Repo MCP was down all session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`); no
ticket was opened, closed or reopened, `📓️status.md` and `🎫️ticket.json` were not touched. No
git-state-modifying command was run. No dev server was started or stopped. Nothing under `🗑️generated`
that this lane did not create was touched.

---

## 1. TL;DR

`📓️wgpu-input-hit-runtime-2026-09-13.md` §11 left this at "the intent is admitted and the authority
never faults, but nothing downstream moves". **Seven defects** sat between that point and the guest,
each measured on 6118 and each fixed at its owning layer. After them, every gesture the brief names
now produces the exact wire message the shared fixture
`🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json` demands, in both roles, with **zero** dispatch
failures, **zero** authority faults and **zero** panics:

| gesture | what 6118 now publishes |
|---|---|
| hover the mesh | `interactionHover {domainId:"graph", channel:"pointer", targets:"[{\"granularity\":\"handle\",\"id\":\"extrude@solid\"}]"}` |
| hover off | `interactionHover … targets:"[]"` |
| click | `interactionSelect {…, targets:"[{\"granularity\":\"object\",\"id\":\"extrude@solid\"}]", merge:"replace", method:"pick"}` |
| shift-click | the same, `merge:"additive"` |
| empty click | `interactionSelect … targets:"[]", merge:"replace"` |
| marquee release | `interactionSelect … targets:"[{\"granularity\":\"object\",\"id\":\"extrude@solid\"}]", method:"rectangle"` |
| wheel | `setCamera {windowId:"procedural-preview", camera:{position,target,fov}}`, accepted |
| alt + right-drag | 7 × `setCamera`, the local rig orbiting `[4,-4,3] → [-2.114,-1.994,4.219]` |
| shift + right-drag | 7 × `setCamera`, the local rig panning its target to `[-0.639,0.186,-0.232]` |

**What is NOT claimed:** the guest still does not APPLY the hover/selection. One hop remains, it is
not this layer, and it is named exactly in §7: the wgpu bridge **drops the `spawn-job` effect** that
every framework reserved tool job (`interactionSelect`/`interactionHover`) is delivered as. `setCamera`
is not affected — it crosses as a typed operation and is accepted.

---

## 2. The hop, and where it now stops

```
pointer ─► dispatch_normalized_event ─► app.modifiers  ❶ never left default()          ← FIXED §3.1
   └─► handle_pointer_* ─► enqueue_world3d_event ─► WorldInteractionAuthority::step
          ├─ PointerMove  ─► WorldRayPickCursor[Hover]    ─► finish_plan ─► publish
          ├─ PointerButton─► GumballPick ─► marquee ─► WorldRayPickCursor[Instance]/MarqueePick
          │                                    ❷ then_some underflow → PANIC            ← FIXED §3.2
          ├─ Wheel/Drag   ─► plan_world3d_wheel / plan_world3d_drag ─► publish
          │
          └─► publish_world3d_plan_step
                 ├─ targets as a JSON ARRAY       ❸ guest reads it with `as_str`        ← FIXED §3.3
                 ├─ granularity = scene's          ❹ a pick reports "object"             ← FIXED §3.4
                 ├─ id = RENDER id                 ❺ not in the guest's topology         ← FIXED §3.5
                 ├─ merge: ctrl → invertive        ❻ no `subtractive` existed            ← FIXED §3.6
                 └─ setCamera keyed `surfaceId`    ❼ window kind does not own it         ← FIXED §3.7
                          │
   ═══════════════ every hop above is now browser-proven correct ═══════════════
                          │
                 guest: Effect::SpawnJob{framework.reserved.tool}
                          └─► wireEffectToFriendly: "unmapped effect spawn-job dropped"  ← §7, NOT this layer
```

---

## 3. The seven defects

### 3.1 Pointer modifiers never reached the shell — `🪟️winit-app/🦀️.rs:320`

`DispatchEvent`'s pointer variants carry no modifier state (`PointerDown/Up/Move {pointer, x, y, button}`);
only `KeyDown`/`KeyUp` do. `dispatch_normalized_event` read `app.modifiers` for the pointer arms, and
`app.modifiers` was written **only** by `handle_pointer_*` from the value those same arms had just
handed it — a closed loop that starts at `PointerModifiers::default()` and never leaves it.

Measured: every world3d intent carried `mods=----` even while Playwright held Shift or Alt, so a
shift-click published `merge:"replace"` and alt/shift + right-drag never reached `plan_world3d_drag`'s
orbit/pan arms at all — the pre-fix run's `h4`/`h8`/`h9` reported `newActions: []` outright.

**Fix** — the key arms mirror the full post-event state into `app.modifiers`, which is exactly winit's
`WindowEvent::ModifiersChanged` for the normalized dispatch path. The events already carried it
verbatim: `KeyDown { key: "Alt", modifiers: EventModifiers { … alt: true … } }`.

### 3.2 A marquee release killed the whole renderer — `🌍️world/🦀️.rs:3411`

```rust
let target_index = (self.stage >= 3 && self.stage < end).then_some(usize::from((self.stage - 3) / 4));
```

`then_some`'s argument is evaluated **eagerly**, so `self.stage - 3` ran on stages 0, 1 and 2 too and
underflowed the `u16`. The arithmetic panic aborted the wasm instance; the Worker installed no panic
hook, so the only visible symptom was the NEXT `#[wasm_bindgen] &mut self` call:

```
70595 pageerror Error: recursive use of an object detected which would lead to unsafe aliasing in rust
```

— the poisoned borrow the aborted call left behind. From that instant every pointer, wheel and key was
silently dead, which is why the first two runs of this lane measured "wheel and orbit do nothing".

**Fix, two parts.** `then`, never `then_some` (and the staging ladder is now one target per bounded
step rather than four fields). And **a panic hook**: `install_worker_panic_trace`
(`🌐️browser-worker/🦀️.rs`) routes a Rust panic to `console.error` once, at boot — the same class of
defect as the 27 dead `eprintln!` traces of `📓️wgpu-input-hit-runtime-2026-09-13.md` §3, fixed the
same way. It named this one on the first run afterwards, verbatim:

```
92748 error [DEBUG] wgpu-worker panicked: panicked at …/🌍️world/🦀️.rs:3344:101:
```

### 3.3 `targets` was a JSON array; the framework reads it as text

The framework's own decoder is `parse_interaction_targets`
(`💻️os/🔨️modules/🔌️plugin/🦀️.rs:592`):

```rust
let raw = args.and_then(|value| value.get("targets")).and_then(DslValue::as_str).ok_or_else(…)?;
serde_json::from_str(raw)…
```

and the manifest declares the arg `ActionArgDef::text("targets")` (`🛂️manifest/🦀️.rs:1172`). A
structured array makes `as_str` answer `None`, so the guest faults the reserved job — and because a
reserved tool job's fault never reaches the dispatch promise, the shell saw a **clean** dispatch and
the selection simply never moved. React encodes the same arg with `JSON.stringify(targets)`
(`world3dSelectionActionArgs`/`world3dHoverActionArgs`).

Measured before (the pre-fix run) and after (`🗑️generated/wgpu-world3d/final-editor/`), same click, same point:

```
before  args={"domainId":"graph","targets":[{"granularity":"handle","id":"extrude@solid#0"}],"merge":"replace","method":"pick"}
after   args={"domainId":"graph","targets":"[{\"granularity\":\"object\",\"id\":\"extrude@solid\"}]","merge":"replace","method":"pick"}
```

**Fix** — `INTERACTION_TARGETS_*` + `push_interaction_targets` build the value through the bounded
builder's `string_joined`, so the JSON text costs no allocation and stays inside the action's own byte
credits. The marquee's staged publisher assembles the same text one target per step into its own fixed
4 KiB buffer.

### 3.4 A pick reported the scene's hover granularity

`🖱️pointer-gestures.json` separates them deliberately: hover reports the scene's `domainGranularityId`
(what the pointer is *over*), a pick reports `"object"` (it selects the whole object). The wgpu
authority stamped the scene granularity on both. `WORLD_PICK_GRANULARITY_ID` now owns the pick half,
for the ray pick and the marquee release alike.

### 3.5 The published id was the RENDER id, not the topology target

A surface renders one channel as several instances (`extrude@solid#0`, `extrude@solid#1`) and the
guest's topology knows only the channel (`extrude@solid`). The wgpu ingest
(`World3dSceneInstanceEntry`) **dropped** the scene's `interactionId` field outright, so the authority
could only publish the render id — which the guest drops as unknown. React resolves the same way
(`interactionTargetsForInstances`).

**Fix** — `World3dSceneInstanceEntry.interaction_id` is ingested, `World3dState.instance_interaction_ids`
holds the render→topology map (only the entries that differ, so a miss means "its own target", exactly
React's `?? id`), and `instance_interaction_id` is the one resolver used by the ray pick and the
marquee. `apply_runtime_draw_flags` matches a published id against **either** the instance or its
target, because both reach this state: the guest's `selectionJson.ids` carries render ids (pinned by
`🧫️fixtures/🌉️scene-bridge/🔣️.json`) while this surface's own optimistic `local_hover_id` is the
topology id it just dispatched.

### 3.6 The merge vocabulary had no `subtractive`

Six call sites open-coded `shift → additive, ctrl → INVERTIVE, else replace`. The one vocabulary of
`🕹️interaction/🧬️schema/🔣️.json` is `shift+ctrl → invertive`, `shift → additive`, `ctrl` (and its
platform twin `meta`) `→ subtractive`, nothing `→ replace` — React's `resolveWorldMergeMode` verbatim.
`world_merge_mode`/`world_merge_code`/`world_merge_wire_label` are now the single owner; all six sites
read them.

### 3.7 `setCamera` was addressed to the wrong window — the §10.3 defect class again

`setCamera` is a WINDOW-OWNED action: `generation3d` declares it on its two preview window kinds only
(`✏️editor/🦀️.rs:2324`, `:2340`). The wgpu plan sent `{surfaceId, camera}`, and `dispatch_action`
resolves `ActionAddress::window_instance_id` from the `windowId` **argument**, falling back to the
focused window. Quoted from 6118 (the pre-fix run, after this lane made the failure visible at all — §4):

```
71489 [DEBUG] frame deferred action failed: handle_action promise failed:
              window kind procedural-main does not own action setCamera
```

**Fix** — the Camera action carries `windowId`, which for a World3d surface is its own id
(`ShellState::world3d_states` is keyed by the window instance id), exactly React's
`worldCameraSetCameraDispatchArgs(windowId, camera)`. Proven in both roles:
`windowId:"procedural-preview"` in editor, `windowId:"procedural-view-preview"` in viewer, **0**
dispatch failures in either.

### 3.8 (bonus) An empty marquee page was two bytes short

With `targets` as text, a page with NO targets still writes `[]`. `page_credit` charged the array
delimiters per target — zero for an empty page — and the shortfall surfaced on the LAST string of the
action, faulting the whole publish: `leave step=Fault active=MarqueePublish[page=0 stage=5 targets=2b]`.
The reservation now charges them once, unconditionally.

---

## 4. Diagnostics this lane added (and why they were needed)

| trace | what it separates |
|---|---|
| `World3dState::interaction_census()` + `[DEBUG] world3d interaction surface=… enter/step/leave` | "nothing moved" into five distinguishable stops: no intent arrived, the intent sat behind a generation, the ray found no triangle, the plan was built but not published, the action published but refused. Carries the front intent (phase/button/down/point/**modifiers**), the active transition (`Pick[Hover hit=…]`, `MarqueePublish[page stage targets]`), the gestures, the pick inputs (`draws`, `objects`, `bounds`, `pick`) and the surface's own `camera=`/`hover=`/`selected=`. A Pending step is logged only when the census CHANGED — that is the transition ladder — plus a stride line every 4096 steps for a spin whose census is constant. |
| `[DEBUG] frame input action controller=… action=… args=…` | what the frame actually took OUT of the bounded input queue, with the args JSON. This is the line that showed the array-shaped `targets`. |
| `[DEBUG] frame deferred action failed: …` | the guest's refusal, which used to be swallowed into `shell.error` with no trace at all. §3.7's quote exists only because of it. |
| `install_worker_panic_trace` | a Rust panic in the frame Worker, which had **no** hook and therefore no message. |

---

## 5. Laws

`🌍️world/🧪️tests/🖱️pointer-gestures/🦀️.rs`, mounted at `🌍️world/🦀️.rs:12247`, is the **third reader**
of `🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json` — the language-neutral oracle React's own two
suites already answer (`🧑‍🎨engine/🧪️tests/🖱️world3d-interaction/🟦️.tsx` in jsdom, and the React-free
Node oracle in `🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`) — and the first that drives the
wgpu implementation: `WorldRayPickCursor` → `finish_plan` → `publish_world3d_plan_step`, plus
`WorldMarqueePublishJob` and `plan_world3d_wheel`. Each law also re-derives the PRE-FIX shape from the
same fixture and asserts it differs, so the fixture is held to discriminating rather than agreeing.

```
RUST_MIN_STACK=33554432 CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-infinite --lib \
  -- pointer_gesture_tests:: --test-threads=1 --nocapture
```

```
[DEBUG] pointer-gestures instance-pick-replaces: targets=[{"granularity":"object","id":"extrude@solid"}] merge=replace
[DEBUG] pointer-gestures instance-pick-additive: targets=[{"granularity":"object","id":"extrude@solid"}] merge=additive
[DEBUG] pointer-gestures instance-pick-subtractive: targets=[{"granularity":"object","id":"extrude@solid"}] merge=subtractive
[DEBUG] pointer-gestures instance-pick-subtractive-on-command: targets=[{"granularity":"object","id":"extrude@solid"}] merge=subtractive
[DEBUG] pointer-gestures instance-pick-invertive: targets=[{"granularity":"object","id":"extrude@solid"}] merge=invertive
[DEBUG] pointer-gestures second-instance-of-one-topology-id-pick: targets=[{"granularity":"object","id":"extrude@solid"}] merge=replace
[DEBUG] pointer-gestures instance-hover: targets=[{"granularity":"handle","id":"extrude@solid"}] hover=Some("extrude@solid")
[DEBUG] pointer-gestures background-click: targets=[]
[DEBUG] pointer-gestures marquee-release-replaces: targets=[{"granularity":"object","id":"extrude@solid"}] instances=2
[DEBUG] pointer-gestures empty-marquee-credit: base=76 credit=78 array=2
[DEBUG] pointer-gestures orbit-completes-into-one-setcamera: windowId=procedural-preview camera=Object([("position", …), ("target", …), ("fov", Number(Float(45.0)))])

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 343 filtered out
```

Compiled with **109 warnings emitted** for the crate under test — quoted because zero errors means
nothing if the expansion never ran. The marquee law drives BOTH rendered instances of one topology id
and asserts ONE target, so the dedup is pinned at the publisher, not only at a call site.

Two existing laws in `🌍️world/🧪️tests/🔬️unit/🦀️.rs` stated the OLD rule and were rewritten to the new
one (greenfield, no compatibility shim):
`world_marquee_pages_build_one_target_per_grant_and_publish_atomically_fifo` (was `…_one_target_field_per_grant…`;
now decodes `targets` the way the framework does and pins `"object"`), and
`scene_bridge_honours_selection_hover_camera_and_sun` now passes again against the either-id match of §3.5.

**Suite state, stated honestly.** `cargo test -p semio-framework-os-infinite --lib` finishes
**328 passed / 15 failed** with this lane's test module SKIPPED — i.e. the same 15 fail without it —
and 9 of those 15 are still red after every fix above. They are **not** this lane's: 4 are
`board::ports::directed_dag`/`directed_normal` (the concurrent `wgpu-retained-controls-wires` lane owns
that file, and its own diff is in the tree), and the world ones fail on code no hunk of this lane
touches — `world_object_registry_enforces_capacity_revision_and_aba` faults inside
`WorldInteractionObjectRegistry::resolve`, `prepared_world_resources_are_send_and_deduplicate_uploads`
on `input.uploads`, `sync_terrain_state_queues_fetch…` on banded terrain draws. 199 files are modified
in the tree by peers, including `🖱️ui/🧬️contract/…` which all three depend on. `git diff` on
`🌍️world/🦀️.rs` shows only this lane's hunks, so the attribution is by hunk, not by guess.

---

## 6. 6118, run by run

Probe: `<ticket>/🐍️wgpu-world3d-interaction-probe.mjs`, written this lane. Every point is DERIVED from
the shell's own `[DEBUG] wgpu-shell dock plan` body — the preview's rect, its centre and its empty
corner — never guessed. Nine gestures per run, each with a before/after of the camera (local rig AND
the guest's `cameraJson`), the selection lane, the authority census, the published actions, the guest's
refusals and the intent ladder.

The intermediate run directories were trimmed after their findings landed here; the two FINAL runs are
kept under `🗑️generated/wgpu-world3d/`. Every quotation below is reproduced verbatim in this report.

| run | what it established |
|---|---|
| 1st, 2nd | the authority DOES resolve `extrude@solid#0` and publishes `interactionHover`/`interactionSelect`; the marquee release kills the runtime (`recursive use of an object`) and every later gesture is dead |
| 3rd | gestures reordered so the crash cannot mask them: `setCamera` reaches the guest and is REFUSED, verbatim (§3.7); `mods=----` on every intent (§3.1) |
| 4th | after §3.1/§3.3–§3.7: every wire arg correct, 0 dispatch failures; panic hook names the marquee panic's file:line |
| 7th, 8th | census carries the publish job's `page/stage/targets` — the empty-page byte shortfall of §3.8 read directly off `active=MarqueePublish[page=0 stage=5 targets=2b]` |
| 10th | a CROSSING marquee (right → left) selects `extrude@solid`; the earlier WINDOW band (left → right) selected nothing because it did not fully enclose the mesh — strict enclosure, not a hole |
| **`final-editor`** | all nine gestures, editor role, table in §1 |
| **`final-viewer`** | all nine gestures, viewer role, `windowId:"procedural-view-preview"` |

Counts, `final-editor` (one boot): `os_host pointer hit` **93**, `world3d interaction` **300**,
`frame input action` **24**, `frame deferred action failed` **0**, `render begin` **222**,
`pointer failed` **0**, `world3d retained interaction authority faulted` **0**, `panicked` **0**,
`wgpu-worker panicked` **0**.
`final-viewer`: the same, `frame deferred action failed` **0**, `panicked` **0**.

Console excerpt, the click (`final-editor/console.txt:937`):

```
52294 log [DEBUG] frame input action controller=s.procedural.generation3d@1/*#editor action=interactionSelect
      args={"domainId":"graph","targets":"[{\"granularity\":\"object\",\"id\":\"extrude@solid\"}]","merge":"replace","method":"pick"}
```

and the orbit's own rig, off the authority's census:

```
world3d interaction surface=procedural-preview leave g=… camera=[-2.114,-1.994,4.219]->[0.000,0.000,0.000]/45.0deg …
```

---

## 7. The one hop that remains — and it is not this layer

`interactionHover` and `interactionSelect` are FRAMEWORK RESERVED TOOL JOBS. The guest answers the
action immediately with nothing but an effect (`🔌️plugin/🦀️.rs:24235`):

```rust
Effect::SpawnJob { job, kind: FRAMEWORK_RESERVED_JOB_KIND, input: encode_framework_reserved_job_input(…), placement: Isolated }
```

and the actual selection work happens in that job, which the HOST must drive (`start-job` → `step-job`
until `Done` → `Event::JobCompleted{job, result}` back into the next turn — the shape
`🖥️host/🧵️shard/🦀️.rs:1733` already implements natively, and `settle_framework_reserved_admission`
documents for guest laws).

The browser path drops it. `🎭️actor/🖼️wire-turn/🟦️.ts:556`:

```
43509 warning wireEffectToFriendly: unmapped effect "spawn-job" dropped — unverified wasm-boundary conversion
```

**18 of them in `final-editor`, exactly two per `interactionHover`/`interactionSelect`.** The friendly
`Effect` union already declares `spawnJob` (`🎠️kernel/🟦️.ts:1357`); `wireEffectToFriendly` has no case
for it, and no production caller of `ShardClient.startJob`/`stepJob` exists
(`📮️shard-client/🟦️.ts:2070`/`:2076` are reached only from their own tests). So **no** browser host
runs a framework reserved job today — this is not specific to World3d and not specific to the wgpu
target; it gates every reserved interaction verb.

This lane did **not** implement it: it is the actor/job-transport layer, it needs the effect mapping,
the job pump and the `job-completed` turn event together, and doing half of it would be a hack. It
pre-dates this lane — the same warning appears 8× in the earlier
`🗑️generated/wgpu-input/deliverables-editor-2/console.txt`.

`setCamera` is **not** affected: it crosses as a typed operation
(`[DEBUG] wgpu-bridge typed-operation command instance=1 pages=3 terminal=true` right after each
dispatch) and the guest accepts it.

---

## 8. Scoreboard against the brief

| # | deliverable | verdict |
|---|---|---|
| 1 | hover → hover lane names `extrude@solid` | ✅ **shell-side proven** — the surface's own `hover=Some("extrude@solid")` and the exact fixture-shaped `interactionHover`. ❌ the guest's lane does not move: §7. |
| 2 | click → selection `["extrude@solid"]` + gumball | ✅ **shell-side proven** — `interactionSelect` with the deduplicated topology target at `object` granularity, single / shift-add / empty-clear / marquee all correct. ❌ the guest's `selectedIds` does not move: §7, so the gumball (which arms off the guest's selection) is **not claimed**. |
| 3 | wheel/drag → camera changes and `setCamera` dispatches | ✅ **fully proven** — the local rig moves on wheel, orbit and pan, `setCamera` carries `windowId` and the guest accepts it with 0 refusals, in editor and viewer. The guest's `cameraJson` echo is a separate question this lane did not chase. |

---

## 9. Files

**Changed**

| file | what |
|---|---|
| `…/♾️infinite/🌍️world/🦀️.rs` | `interaction_census` + `world_interaction_active_census`; `instance_interaction_ids` ingest + `instance_interaction_id`; `WORLD_PICK_GRANULARITY_ID`; `world_merge_mode`/`world_merge_code`/`world_merge_wire_label` as the ONE merge rule (6 open-coded sites folded in); `INTERACTION_TARGETS_*` + `push_interaction_targets` (JSON-text `targets`); `setCamera` keyed `windowId`; `WorldMarqueePublishJob` staging rewritten (one target per grant, own bounded `targets` buffer, dedup, `then` not `then_some`, empty-array credits); `apply_runtime_draw_flags` matches instance OR topology id |
| `…/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` | `world3d_interaction_trace` (enter / census-change / stride / leave); `[DEBUG] frame input action …` with its args; the deferred-action failure is logged, not only stored on `shell.error` |
| `…/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` | `dispatch_normalized_event` mirrors `EventModifiers` into `app.modifiers` on both key edges |
| `…/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs` | `install_worker_panic_trace` — the frame Worker's panics reach `console.error` |
| `…/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` | four helpers `pub(super)` for the sibling law; the marquee `targets` law rewritten to the text encoding and the pick granularity |

**Added**

| file | what |
|---|---|
| `…/♾️infinite/🌍️world/🧪️tests/🖱️pointer-gestures/🦀️.rs` | the wgpu reader of the shared gesture fixture — 7 laws, each with its pre-fix counter-derivation |
| `<ticket>/🐍️wgpu-world3d-interaction-probe.mjs` | the nine-gesture probe: dock-plan-derived points, local-rig AND guest-camera witnesses, the intent ladder, published actions, guest refusals |

---

## 10. Peer notes

* `🌍️world/🦀️.rs` carried no peer hunks while this lane worked (`git diff` is all mine); `🔬️unit/🦀️.rs`
  was touched only for the four `pub(super)` and the two rewritten laws.
* The tree was red on arrival and stayed red in areas this lane does not own — see §5's attribution.
  Nothing of a peer's was reverted.
* `pgrep -fl framework-renderer-wgpu:wasm` was checked before every renderer build; 10 builds this
  session, each verified by `Successfully ran target`, the `dist/wasm-dev` mtime, and (for the first
  ones) that the new trace string is present in the shipped `.wasm` before probing.
