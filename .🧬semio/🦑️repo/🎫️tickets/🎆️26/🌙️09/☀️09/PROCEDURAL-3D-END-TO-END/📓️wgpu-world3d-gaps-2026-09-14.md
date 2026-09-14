# 🕳️ wgpu World3d gaps — the three reds §5 left open, root-caused and closed (lane `wgpu-world3d-gaps`)

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, 2026-09-14. Target: the coordinator's wgpu serve,
`http://127.0.0.1:6118/?plugin=generation3d` (`&role=viewer`, `&mode=generate`).

Repo MCP was down all session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`); no
ticket was opened, closed or reopened, `📓️status.md` and `🎫️ticket.json` were not touched. No
git-state-modifying command was run. No dev server was started or stopped. Nothing under `🗑️generated`
that this lane did not create was touched.

---

## 1. TL;DR

`📓️wgpu-end-to-end-verification-2026-09-14.md` §5 left three reds on the World3d preview. All three are
closed at their owning layer, each with a browser-measured root cause, a failing-first law pair
(Rust + TypeScript over one shared fixture) and a green battery lane. A **fourth** defect of the same
family surfaced while reproducing §5.3 and is closed too.

| item | root cause, in one line | proof on 6118 |
|---|---|---|
| §5.1 wheel dead in EDIT mode | `dispatch_normalized_event` DROPPED the wheel's own point; the frame applied the coalesced delta at `last_pointer_x/y`, which a slow session has already moved | `world3d-editor` **10/10**, `h7_wheel_zoom` publishes `setCamera`, camera `[4,-4,3] → [3.200,-3.200,2.400]` |
| §5.2 viewer mints `translateSelection` | the gumball was an unconditional affordance of every World3d surface; nothing read `window_kind_action_refs` | `world3d-viewer` **10/10**, `frame deferred action failed: 0` |
| §5.3 `world3d scene mesh-wire bridge faulted` | "this build has no pages" was encoded as `World3dSnapshotFault::Capacity`, and every bridge fault quarantines the surface | `frame-loop` **4/4**, 20 gestures + 3 Form slider edits, **0 quarantines** |
| §5.3′ (new) transform verbs addressed to the focused window | `translateSelection` carried `surfaceId` but no `windowId`, so the shell's address fell through to focus | `handle_action promise failed: … generation3d-generate-form does not own action translateSelection` gone |

**Battery:** `bun 🐍️wgpu-battery.mjs --only=world3d-editor,world3d-viewer,frame-loop` →
`world3d-editor ok=true 125s steps=10/10`, `world3d-viewer ok=true 122s steps=10/10`,
`frame-loop ok=true 128s steps=4/4`, **0 page errors** across all three
(`🗑️generated/wgpu-verify/scoreboard.json`, this lane's rows).

**Laws:** 7 Rust + 13 TypeScript, all green; **6 of them proven RED against the pre-fix shape** by
temporarily reverting each production hunk and re-running (§4.4). Two existing gumball laws updated for
the added `windowId` argument.

---

## 2. §5.1 — the wheel was applied where the pointer had wandered to, not where it was scrolled

### 2.1 What the browser said

A `[DEBUG] wheel gate` trace was added at the frame's own wheel phase, printing the point the frame
applies the wheel at, the hit the scene-surface gate resolves there, and whether any world3d surface's
bounds contain it. Four notches scrolled over the EDIT preview centre arrived as **one** line:

```
[DEBUG] wheel gate x=5 y=5 delta=-480 hit=None control=None propagates=true
        worlds=[("procedural-preview", false)]
```

Four notches **coalesced into one delta**, applied at `(5, 5)` — the top-left corner, 1 200 px from the
preview — where no world3d surface's bounds contain the point, so `WheelWorld3d` enqueued nothing and
every one of the authority's intents carried `wheel=0`, exactly as §5.1 reported. The VIEWER lane
produced **four separate** gate lines for the same gesture.

### 2.2 The hop that loses it — `🪟️winit-app/🦀️.rs:317` (pre-fix)

```rust
DispatchEvent::Scroll { delta_y, .. } => {
    app.wheel_delta += delta_y;
}
```

`DispatchEvent::Scroll { x, y, delta_x, delta_y }` carries its own position, and
`🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json`'s `wheel-over-the-preview` row already pins that the
position survives BOTH hops from the DOM (`offsetX/Y × devicePixelRatio` → wire → dispatch). The wgpu
dispatch arm was the one place that threw it away, leaving `AppFrameTransactionPhase::WheelStart` to
read `app.last_pointer_x/y`.

### 2.3 Why that is EDIT-mode-only, which is what made it look like chrome

The wgpu tick is input-driven. A wheel invalidates and asks for a frame, but the frame that actually
drains the accumulator is whichever one runs next — and the battery's own `h7_wheel_zoom` nudges the
pointer to the corner between notches (`🐍️wgpu-world3d-interaction-probe.mjs:218-222`). In the VIEWER a
frame lands between the wheel and the nudge, so `last_pointer` is still the centre and it worked. The
EDIT session is converging a flow window, a node graph AND a preview, so the drain lands AFTER the
nudge and the wheel is applied at the corner. The difference is timing, not chrome: there is no gumball,
no transform rail and no hit-priority involved, and the gate answered `propagates=true` in both roles.
**Nothing in the report's three suspected causes was the cause.**

### 2.4 The fix — the wheel carries its own point

`🧊️renderer/🦀️.rs:10801` — `AppWheel { delta, x, y }` replaces the bare `wheel_delta: f32`:
`accumulate(x, y, delta_y)` coalesces the delta and keeps the NEWEST event's point (what a browser's own
wheel stream means); `take()` answers `Option<(delta, x, y)>` and leaves none. The Scroll arm becomes
`app.wheel.accumulate(x, y, delta_y)` (`🪟️winit-app/🦀️.rs:319`) and `WheelStart` reads
`app.wheel.take()` (`🧊️renderer/🦀️.rs:11778`) instead of `last_pointer_*`. Pointer bookkeeping is not
touched — a wheel is not a pointer move.

### 2.5 Proof

```
Scroll { x: 1208.0, y: 461.0, delta_x: 0.0, delta_y: 200.0 }   ×5   (the preview centre)
frame input action … action=setCamera                          ×5
guest camera [4.0,-4.0,3.0] → [2.048,-2.048,1.536]
```

and the battery's own step: `✓ h7_wheel_zoom {"actions":["interactionHover","setCamera"],
"camera":"[3.200,-3.200,2.400]->[0.000,0.000,0.000]/45.0deg"}`.

The temporary `wheel gate` trace was removed once it had answered.

⚠️ **A probe artefact this lane found and fixed in itself.** This lane's first probe moved the pointer
to the corner BEFORE scrolling, so Playwright's `mouse.wheel` fired at `(5,5)` and the wheel's own point
was genuinely the corner — which would have made the fix look unproven. The probe now mirrors the
battery exactly: move to the centre, scroll, nudge, repeat (`🐍️wgpu-world3d-gaps-probe.mjs`, §5.1
block). Every number above is from the corrected shape.

---

## 3. §5.2 — a surface may only mint a verb its window kind declares

### 3.1 What the browser said

A shift-drag on the gumball in the VIEWER, reproduced verbatim before the fix:

```
frame input action … action=translateSelection
frame deferred action failed: handle_action promise failed:
    window kind procedural-view-preview does not own action translateSelection
```

`👁️viewer/🦀️.rs:1629` gives `preview::WINDOW_KIND_ID` exactly `setShowMode`, `setLodMode`, `setCamera`,
`exportDocument` and the sun group. `✏️editor/🦀️.rs:2460` gives `procedural-preview` those PLUS
`translateSelection`/`rotateSelection`/`scaleSelection`. The manifests were already right; nothing read
them.

### 3.2 The fix — one declaration, published by the shell, read by the world

The gumball is an affordance of a WINDOW KIND, not of World3d:

* `World3dState.declared_action_ids` (`🌍️world/🦀️.rs`, `World3dState`) — the action ids the window kind
  hosting this surface declares. Empty means "nothing declared yet", which offers nothing.
* `set_world3d_declared_actions` (`🌍️world/🦀️.rs:1697`), `world3d_declares_action`,
  `world3d_offers_transform_gumball` (`:1712`).
* `gumball_handle_action_id` (`:4654`) — the single handle→verb mapping, now shared by the pick and by
  `WorldGumballCommitJob::action_id` (it was duplicated).
* Three gates, all reading that one declaration: the pick ADMISSION (`:5606`), the PER-HANDLE candidate
  (`:4775` — a window that declares only `rotateSelection` offers only rotate handles), and the PAINT
  (`:10444` — the viewer does not draw a gizmo it cannot use).
* The publisher: `ShellState::sync_world3d_declared_actions`
  (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4405`), called from `sync_engine_surface_states` after every
  completed chrome walk. A World3d surface is keyed BY its window instance id, so its owning kind is
  what the dock says that instance is, and the kind's `actions` are `window_kind_action_refs` after
  `build_definition` resolved them (`🔌️plugin/🦀️.rs:5329`). No renderer-side guess, no role branch.

### 3.3 Proof

The same shift-drag in the viewer now publishes `interactionHover` + `interactionSelect` and **nothing
else**; `frame deferred action failed` is **0** in both the lane probe and the battery's own
`no authority fault, no panic, no dropped effect` step (`world3d-viewer 10/10`). The editor keeps its
gumball — `procedural-preview` declares all three verbs, and `world3d-editor` is 10/10 with its own
selection gestures unchanged.

---

## 4. §5.3 — "nothing to draw" was reported as "out of credits", and that quarantines the surface

### 4.1 Reproduction

§5.3 does not reproduce on gestures alone: a clean 20-gesture `frame-loop` run against the §B renderer
gave `quarantines: 0`. It needs a **mesh update**. This lane's probe boots `&mode=generate`, presses
`procedural3d-play-generate.add-generation`, then interleaves 20 world3d gestures with 3 Form slider
DRAGS resolved from the Form's own published node rects (`generate.form.height`, `.radius`, `.sides`).
The fault fired on the **third** edit, every time.

### 4.2 The census that named it

A `[DEBUG] world3d bridge fault surface=… <ingest_census()>` line was added at the quarantine point
(`🧊️renderer/🦀️.rs:11674`) — kept, because the previous message named neither the surface nor the
fault:

```
world3d bridge fault surface=generation3d-generate-preview
    state-draws=1 state-instances=1 state-meshes=3 bridge=false bridge-lease=true
    apply=false … snapshot-lease=true fault=Some(Capacity)
frame fault recorded: world3d scene mesh-wire bridge faulted
```

`fault=Some(Capacity)` with three published meshes is not credit exhaustion. The mesh wire at that
instant, against every settled sample of the same run:

| moment | `meshes_json` | `instances_json` | state |
|---|---|---|---|
| settled (×5 distinct samples) | 3 644 b / 4 646 b / 4 786 b | 705–707 b | `draws=1 meshes=3` |
| **the faulting turn** | **169 b** | **267 b** | `draws=1 meshes=3` |

169 bytes cannot hold a tessellated solid. It is the wire the guest publishes WHILE it recomputes after
`updateGenerationValues`: mesh records that are named but carry no triangles yet.

### 4.3 Root cause

`step_world3d_scene_bridge`'s `Parse` phase drops them —
`retain(vertex_count() > 0 && indices.len() >= 3)` — and the instance retain then drops every instance
that referenced them. The `Pages` phase went straight to `publish_world3d_scene_bridge_snapshot`, whose

```rust
if pages.is_empty() { return Err(World3dSnapshotFault::Capacity); }
```

is the only answer it has, and the wgpu host quarantines the surface on ANY bridge fault. A snapshot is
at least one page by construction — `world3d_snapshot_begin` refuses `page_count == 0`
(`🖱️ui/🎬️scene/🌍️world3d-snapshot/🦀️.rs:291`) — so "this build has nothing to publish" and "credits
exhausted" were the same value. **The first build of any surface survived it only by accident**: its
camera digest is always new, so it always carried a camera page. Every later empty build was fatal.

### 4.4 The fix

`world3d_scene_bridge_has_pages` (`🌍️world/🦀️.rs:9958`) asks first whether the staged build has
anything a snapshot can carry — one published mesh with at least one instance, or a changed camera. A
build with neither records its digest (so the same payload never re-stages), records the camera digest
if it moved, and answers `Complete` WITHOUT publishing (`:9864`). The surface keeps its last good
geometry until the guest publishes real meshes again, which is what a preview should do during a
recompute. `World3dSnapshotFault::Capacity` now means only what it says.

### 4.5 Proof

20 gestures + 3 Form slider edits (all three `seated`), on the fixed build:
`frameFaults: [] bridgeFaults: [] dispatchFailures: [] quarantines: 0 pageErrors: 0`. The battery's own
`frame-loop` lane: `✓ the frame wire never quarantines {"quarantines":0}`,
`✓ the frame wire is still answering {"batches":7131,"frames":7131,"answered":true,"faults":0}`,
`✓ the loop keeps publishing actions {"actions":27,"admits":603,"sweeps":43}` — **4/4**.

---

## 5. §5.3′ — the fourth defect, found while reproducing §5.3

The pre-fix generate run also produced, from the same gumball drag:

```
handle_action promise failed: window kind generation3d-generate-form does not own action translateSelection
```

Here the verb WAS owned — `generation3d-generate-preview` declares all three — but the action carried
only `surfaceId`, so `ActionAddress::window_instance_id` fell through to the focused window, which was
the Form. This is the same defect class `📓️wgpu-world3d-interaction-2026-09-13.md` §3.7 closed for
`setCamera` and `📓️wgpu-input-hit-runtime-2026-09-13.md` §10.3 closed for the retained `addGeneration`
row; the transform verbs were simply never converted.

**Fix:** `WorldGumballCommitJob::step` stages a `windowId` node carrying the surface id
(`🌍️world/🦀️.rs:5260`), alongside the `surfaceId` the guest reads for its own purposes; the staging
ladder shifts by one and `string_bytes` accounts for the extra key and value. The `#[cfg(test)]`
`gumball_commit_action` twin mirrors it. `world_gumball_commit_builds_one_flat_node_per_grant_then_retires_tokens`
now asserts both arguments, and `world_gumball_commit_saturation_aba_and_interrupted_close_retain_claim_authority`
counts five staged nodes before the first selected-id resolve instead of four.

---

## 6. The laws

| law | reads | drives | count |
|---|---|---|---|
| `🧪️tests/🖱️wheel-application-point/🦀️.rs` | `🧑‍🎨engine/🧫️fixtures/🖱️wheel-application-point/🔣️.json` | the production `AppWheel` | 1 test, 6 cases |
| `🧪️tests/🖱️wheel-application-point/🟦️.ts` | the same fixture | an independent TS implementation of the rule | 7 assertions |
| `🌍️world/🧪️tests/📇️surface-verbs/🦀️.rs` | `🌐️World3dHost/🧫️fixtures/📇️surface-verbs.json` | `world3d_offers_transform_gumball`, `gumball_handle_action_id`, and the real authority admission (`active=GumballPick`) | 3 tests, 4 cases |
| `🧪️tests/📇️world3d-surface-verbs/🟦️.ts` | the same fixture | an independent TS implementation | 6 assertions |
| `🌍️world/🧪️tests/🌉️bridge-empty-build/🦀️.rs` | `🌍️world/🧫️fixtures/🌉️bridge-empty-build/🔣️.json` applied to the committed `🌉️scene-bridge` payload | `sync_world3d_state` → `step_world3d_scene_bridge` → `step_world3d_snapshot` → `step_world3d_draw_rebuild` | 3 tests, 4 cases |

Both new fixtures carry the 6118 reading that produced them in their own `provenance`/`why` fields, and
every Rust law re-derives the PRE-FIX shape from the same fixture and asserts it DIFFERS — the idiom
`🖱️pointer-gestures` already uses.

**Failing-first, measured.** Each production hunk was temporarily reverted and the laws re-run:

```
gumball offered unconditionally + bridge publishes unconditionally
  → 5 of 6 world laws FAILED, with
    "meshes-without-triangles: a mesh update never faults the bridge — … fault=Some(Capacity)"
    "the-procedural-viewer-preview-owns-none-of-them: … active=GumballPick …"
AppWheel::accumulate drops the point
  → the wheel law FAILED: left Some([0.0, 0.0, -120.0]) right Some([1207.5, 461.0, -120.0])
```

**Green:** `cargo test -p semio-framework-os-infinite --lib -- surface_verb bridge_empty_build` → 6
passed. `cargo test -p semio-framework-os-renderer-wgpu --lib -- wheel_application_point` → 1 passed.
`bunx vitest run --config 🧪️tests/🎚️config/🟦️.ts …` → 2 files, **13 tests passed**.

### 6.1 Suite state, and what is NOT this lane's

`cargo test -p semio-framework-os-infinite --lib` → **354 passed, 12 failed**. Those same 12 fail at
HEAD: with this lane's two changed files temporarily replaced by `git show HEAD:…` the suite answers
**347 passed, 13 failed** (the extra one is a parallel-flaky marquee test). The 12 are peer-owned —
`board::ports::directed_dag/*`, `world::tests::world_object_registry_*`, `world::tests::prepared_world_resources_*`,
`world::tests::live_renderer_retains_generation_wake_*` (a source scan over files the
`wgpu-edit-convergence-perf` lane is rewriting) and siblings; this lane's whole diff is listed in §7 and
touches none of them.

The wgpu renderer crate's own `async_boundary_tests` has 4 failures that are likewise pre-existing:
`native_binary_owns_exactly_one_entrypoint_driver` counts `drive_entrypoint(` in
`⌨️native-entrypoint/🦀️.rs`, which is **byte-identical to HEAD** and already carries 3 where the law
expects 2.

---

## 7. Files

**Changed (production):**

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — `AppWheel` (§2.4), `WheelStart`, the bridge-fault census line (§4.2), the wheel law mount
- `…/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` — the `Scroll` arm keeps the wheel's point
- `…/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs` — `AppInteractionState` initialiser
- `…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `sync_world3d_declared_actions` (§3.2)
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` — the declaration API and its three gates, the shared handle→verb mapping, the `windowId` stage, `world3d_scene_bridge_has_pages`, two law mounts

**Changed (tests/fixtures):**

- `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` — three bridge helpers made `pub(super)`; two gumball laws updated for `windowId`
- `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` — `AppInteractionState` initialiser
- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts` — the two new TS suites

**New:**

- `♾️infinite/🌍️world/🧪️tests/📇️surface-verbs/🦀️.rs`, `♾️infinite/🌍️world/🧪️tests/🌉️bridge-empty-build/🦀️.rs`
- `♾️infinite/🌍️world/🧫️fixtures/🌉️bridge-empty-build/🔣️.json`
- `📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/📇️surface-verbs.json`
- `📺️renderer/🧑‍🎨engine/🧫️fixtures/🖱️wheel-application-point/🔣️.json`
- `📺️renderer/🧑‍🎨engine/🧪️tests/🖱️wheel-application-point/🦀️.rs` + `🟦️.ts`
- `📺️renderer/🧑‍🎨engine/🧪️tests/📇️world3d-surface-verbs/🟦️.ts`

**Peer break fixed forward (minimal, noted):**

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🪣️fill/🦀️.rs` —
  `Puzzle3dCatalogVortexTemplate` gained `mandatory`/`radius`; the one struct literal there did not
  follow and broke every crate downstream of `semio-s-artifact-puzzle-5d`, including the renderer.

**Ticket (this lane):**

- `📓️wgpu-world3d-gaps-2026-09-14.md` (this report)
- `🐍️wgpu-world3d-gaps-probe.mjs` (new) — the three isolated readings, plus the `&mode=generate`
  gestures × Form-slider-edits reproduction
- `🗑️generated/wgpu-world3d-gaps/` — `editor-baseline`, `viewer-baseline`, `generate-baseline-2`
  (the three defects reproduced), `editor-fixed-2`, `viewer-final`, `generate-final` (closed), each with
  `console.txt` + `results.json`; `frame-loop-baseline`
- `🗑️generated/wgpu-verify/{world3d-editor,world3d-viewer,frame-loop}/` — this lane's battery rows

---

## 8. What is NOT claimed

- **A scene that legitimately becomes empty is not cleared.** §4.4 keeps the last good geometry when a
  build has nothing to publish, because the snapshot store cannot express a zero-page snapshot at all.
  A producer that deliberately empties its scene therefore leaves the previous solid painted. That is a
  separate item (it needs the draw-retirement ladder, not the bridge) and no run in this lane exercised
  it.
- **No claim about the other battery reds.** `status-a11y-i18n`, `port-fit`, `chrome` and the retained
  hit-registry drain (§D) were not touched; the merged scoreboard's remaining reds are other lanes'.
- **The gumball's own geometry was not re-verified.** This lane proves which verbs a surface may emit and
  that the viewer emits none of them; it does not re-measure handle picking accuracy, and no run here
  completed a transform commit end-to-end to the guest.
- **`world3d-editor`'s `h4_shift_add` still reports one action where the viewer reports two.** Both lanes
  are 10/10 against the shared fixture; the difference is the probe's hover bookkeeping, not a claim
  about merge semantics.
- **One guest build, several renderer builds.** The renderer wasm was rebuilt three times during this
  lane (09:15 instrumented, 10:34, 11:06 final); every number in §2.5, §3.3, §4.5 and the battery run is
  from the **11:06** build. A peer restaged `generation3d` during the session; the guest was not pinned.
- **The 12 world-crate and 4 renderer-crate test failures in §6.1 were not fixed**, only shown to be
  pre-existing at HEAD.
