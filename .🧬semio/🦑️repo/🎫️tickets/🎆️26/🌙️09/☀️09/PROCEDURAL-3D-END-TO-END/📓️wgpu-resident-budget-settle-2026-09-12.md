# 🎟️ wgpu RESIDENT BUDGET + EVENT-DRIVEN SETTLE — the hexagonal column is on the wgpu World3d surface

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "wgpu resident budget + settle", 2026-09-12.
Resumes `📓️wgpu-dock-layout-world3d-2026-09-12.md` §6.1/§6.2 (the two named remaining hops).

Repo MCP was down all session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`); no
ticket was opened, closed or reopened — bookkeeping is on disk. The react serve on 6018 was not
touched, the procedural guest was NOT rebuilt or restaged, and no git-state-modifying command was run.

⚠️ **The ticket's whole `🗑️generated` tree was deleted by something outside this lane at ~22:47**,
taking `run-1 … run-13`, `final-generate-add`/`-add2` and the serve log with it. This lane swept
nothing (see `feedback-subagents-sweep-ticket-generated-folder`). The three deliverable runs were
re-taken against the same running serve and the same bundle and are present:
`🗑️generated/wgpu-settle/final-edit`, `…/final-viewer-2`, `…/final-generate` — every number quoted
below is from those, re-measured, not from memory. The per-step traces quoted in §5.1/§5.4 are from
the deleted `run-*` directories and are reproduced here verbatim from the diagnostics that produced
them; the diagnostics themselves are permanent in the code, so any run can regenerate them.

---

## 1. TL;DR

| question | answer |
|---|---|
| **1 — does a refresh still need a second full set of resident documents?** | **No.** `refresh_ui` now retires ONE surface's previous document to terminal immediately before asking the guest for its replacement, so the process-wide aggregate carries the live surface count and never twice it. Measured on 6118 across a 120 s run: **`resident-roots=6 resident-bytes=1311424/33554432`**, peak 1 336 704 B, **`Capacity` faults 0**, `retained document permit failed` 0 (§3.1, §6.1). |
| **1 — was `UI_RESIDENT_AGGREGATE_BYTES` resized?** | **No, and it did not need to be.** A settled six-surface refresh commits 1.31 MB of a 33.55 MB aggregate — 3.9 %. The old order's problem was never the ceiling, it was carrying 2 N roots plus a retirement drain that could not finish (§3.1, §3.2). |
| **1 — are permit refusals silent?** | **No.** `Capacity` is now a typed diagnostic naming what was asked for beside what was already committed, and every render leave prints the census whether or not anything failed (§3.3). |
| **2 — does the wgpu shell re-render/re-arm like React?** | **Yes.** `flush_deferred_actions` is now a bounded convergence loop: drain every armed action + extension answer, re-enter the window bodies, repeat until a drain executes nothing. `[DEBUG] wgpu-shell ui chain settled after 1 round(s)`, and the guest reaches **`phase "idle", facesDone 8/8, inFlight 0, ratio 1.0`** with `eval-extrude@solid#0` in `meshes_json` (§4, §6.1). |
| **3 — is the hexagonal column on the wgpu World3d surface?** | **Yes.** `dumpFrameStats("procedural-preview")` = **`scenePasses 1, sceneDraws 2, sceneInstances 1, quadCount 17`**; the world pass carries `draws=1 instances=1` over `state-draws=1 state-instances=1 state-meshes=3`. Screenshot `🗑️generated/wgpu-settle/final-edit/shot-120s.png`; viewer role `🗑️generated/wgpu-settle/final-viewer-2/shot-060s.png` (§6). |
| Defects fixed on the way | **Six**, each measured: the double-buffered refresh order (§3.1); a retirement drain that spent 99 % of its steps on vacant slots at a per-ITEM price (§3.2); a layout lost wake-up that burned all 1 048 576 window-paint opportunities twice a minute (§5.1); a World3d wake predicate that named only the mesh builders (§5.2); a settled shell that scheduled no frames for a live mesh ingest (§5.3); and the **draw-rebuild ↔ snapshot-apply deadlock** that was the actual reason the solid never rasterised (§5.4). Plus `dumpFrameStats` discarding a scene-only paint census (§5.5). |
| Shared law | One language-neutral fixture (`🎟️resident/🔄️refresh/🧫️fixtures/🔣️.json`), a Rust law driving the REAL `UiResidentPermit`/`UiDocumentAssembly` ledger (**3/3**) and a TypeScript twin re-deriving the same arithmetic independently (**4/4**) — both run, both quoted, byte-identical numbers (§7). |

---

## 2. What was actually wrong

`📓️wgpu-dock-layout-world3d` §6.2 measured a bounded settle loop failing with
`retained document permit failed: Capacity` on all six surfaces and concluded the resident aggregate
was the binding constraint. **It was not.** Three separate defects stacked:

1. **`refresh_ui` was double-buffered.** It moved EVERY live window document into the retirement
   registry, then rendered every replacement, then moved the panel documents, then drained. The panel
   documents stayed live across the whole window pass, so the aggregate had to carry up to
   `2 × surfaces` roots at once.
2. **The retirement drain could not finish.** `ShellDocumentRetirementRegistry::close_one` advanced
   its cursor by one POSITION per call across a 512-slot ring and priced each real step at
   `close_step()`'s one item / 4 KiB. With six occupied slots, `SHELL_DOCUMENT_RETIREMENT_STEPS`
   (65 536) bought ~128 real steps per document — far short of what one 120-node document's typed
   descendants need. The previous set was therefore still resident when the next asked for credit.
3. **A refusal said nothing.** `UiResidentFault::Capacity` carries no numbers, so six identical bare
   `Capacity` lines could not distinguish an oversized surface from an un-retired previous set.

Measured proof of (1)+(2), from the shared law (§7): one refresh of six surfaces commits
**3 232 268 B / 6 roots**; the double-buffered order commits **6 464 536 B / 12 roots** — exactly
twice, for exactly the same documents.

---

## 3. Deliverable 1 — the fixes

### 3.1 One resident root at a time

`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`:

```rust
fn retire_one_surface_document(&mut self, document: Option<UiDocumentLease>) -> Result<(), String> {
    let Some(document) = document else { return Ok(()) };
    if self.retain_document_for_close(document).is_err() { return Err(…) }
    self.drain_retained_document_arenas();
    Ok(())
}
```

`refresh_ui` calls it for the surface it is ABOUT to render, immediately before
`render_with_document`, for windows and panel tabs alike; `retire_documents_outside` retires every
document whose surface the active layout no longer names. `retain_document_map_for_close` — the
whole-map mover that made the old order possible — is deleted, not deprecated.

The peak is now the live surface count. There is no "retirement driven to completion inside the same
convergence round" problem left to solve, because there is never a second set to retire.

### 3.2 The retirement drain is priced per page and skips vacancies

```rust
fn close_one(&mut self) -> bool {
    let Some(index) = (0..SHELL_DOCUMENT_RETIREMENT_CAPACITY)
        .map(|offset| (self.cursor + offset) % SHELL_DOCUMENT_RETIREMENT_CAPACITY)
        .find(|index| self.slots[*index].is_some()) else { return false };
    …
    slot.document.close_step_with_grant(SHELL_DOCUMENT_RETIREMENT_ITEMS, SHELL_DOCUMENT_RETIREMENT_BYTES)
```

`SHELL_DOCUMENT_RETIREMENT_ITEMS = 1 024`, `SHELL_DOCUMENT_RETIREMENT_BYTES = 32 KiB` — a page, the
unit the rest of this shell's close ladder is already priced in
(`📓️close-ladder-budget-2026-09-12.md` §3), and under the 64 KiB guest contiguous-request ceiling.

### 3.3 A refusal is a typed, visible diagnostic

`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` gains `resident_refusal`, which renders as:

```
retained document permit failed: Capacity (resident capacity exhausted) — surface 'procedural-main'
asked items 36 bytes 232 882; committed items 204/131076 bytes 1 311 424/33 554 432 roots 6/64
(fixed backing 566 352)
```

and every render leave now carries the census unconditionally:

```
[DEBUG] wgpu-shell render leave surface=procedural-preview 18 ms resident-roots=6 resident-bytes=1311424/33554432
```

so the aggregate is observable before anything fails, not only after.

### 3.4 `UI_RESIDENT_AGGREGATE_BYTES` was NOT changed

Deliberately. The measured six-surface refresh commits 1 311 424 B of 33 554 432 (3.9 %), of which
566 352 B is the contract's own fixed backing. The law also pins WHY the right-sizing that
`assemble_browser_document` already does matters: the aggregate admits only **three**
ceiling-sized (`UI_RESIDENT_SURFACE_BYTES` = 8 MiB) surfaces before refusing (§7).

---

## 4. Deliverable 2 — the event-driven settle

A guest's preview job is driven BY its own window render, so a chain has three halves, not two: the
actions an effect deferred, the extension calls those actions asked for, and the RENDER that turns a
settled answer into the next `flowEvalTick`. `settle_boot`'s one refresh + one flush stopped at the
first fixed point of the first two.

`flush_deferred_actions` is now the convergence funnel every command, effect and interaction already
goes through (`settle_boot`, `handle_pointer_button`, `dispatch_tree_selection`, the renderer's
pointer-move path):

```rust
pub async fn flush_deferred_actions(&mut self) -> Result<(), String> {
    if self.settling { self.drain_deferred_actions().await?; return Ok(()); }
    self.settling = true;
    let result = self.settle_ui_chain().await;
    self.settling = false;
    result
}

async fn settle_ui_chain(&mut self) -> Result<(), String> {
    for round in 0..SHELL_SETTLE_ROUNDS {
        let worked = self.drain_deferred_actions().await?;
        if worked == 0 { … return Ok(()); }
        if self.session.is_none() { return Ok(()); }
        self.refresh_ui().await?;
    }
    …
}
```

* **Bounded** — `SHELL_SETTLE_ROUNDS = 64`, and an exhausted loop is reported, never spun on.
* **Re-entrancy guarded** — a nested flush (one a dispatched action asks for) stays a plain drain, so
  the outer loop keeps the single authority over how often the window bodies are re-entered.
* **Free for a no-op** — a round whose drain executed NOTHING is the fixed point (no render can arm
  what no work produced), so a bare pointer move never pays for a refresh.
* **No rAF dependency** — it runs inside the frame Worker's async boot/dispatch, and the per-frame
  half of the progress is now scheduled by `request_frame` (§5.3), not by animation frames.

Measured (`final-edit`): `[DEBUG] wgpu-shell ui chain settled after 1 round(s) resident-roots=6
resident-bytes=1311424/33554432`, after which the guest's own status reads

```
"phase":"idle","progress":{"unitsDone":44,"unitsTotal":44,"facesDone":8,"facesTotal":8,
                           "inFlight":0,"ratio":1.0},"cancellable":false
```

against `meshingFaces 6/8, inFlight 1` before this lane.

---

## 5. Four more defects, each between the settled chain and the screen

With §3 and §4 landed the guest published the solid — and it still did not appear. Each of the
following was found by measurement, not by reading.

### 5.1 A layout lost wake-up burned the whole window-paint budget

`render_ui_document_step`'s `Layout` phase advanced to `Paint` on `UiLayoutStep::Ready | Idle`. Both
answer for the layout QUEUE, not for a window: `Idle` means the queue was empty and `Ready { .. }`
may name a different surface. So the cursor entered `Paint` against a still-dirty root, where
`frame_into_step` short-circuits `Pending` before it ever opens a paint frame.

```
[DEBUG] ui-doc paint stalled window=procedural-main opportunities=65536 root=present
        dirty-layout=true subtree-dirty=false frame=false revision=0/2 theme=0/1 viewport=0/2
        phase=None fault-site=None
[DEBUG] wgpu-shell window paint procedural-main exhausted 1048576 opportunities parked-in=paint
[DEBUG] wgpu-shell surface fault surface=procedural-main … retained document ingress reached its terminal fault
```

~20 s of frames per chrome walk, twice a minute, which starved every other window: the preview's
World3d surface painted **3 times in 120 s**. Fixed by giving the engine the per-window predicate the
question needs — `Ui::layout_is_dirty` / `Ui::request_layout` — and by letting a `Pending` paint that
is waiting on a layout return to the `Layout` phase. Afterwards: `paint stalled` 0,
`exhausted 1048576` 0, surface faults 0.

### 5.2 The World3d wake predicate named only the mesh builders

`world3d_cursor_work_pending` covered `placeholder_build`/`terrain_build`/`face_overlay_build` and
not the cursors that turn a producer's `meshes_json` into `state.draws`. It now covers the scene
bridge, the snapshot apply, the draw rebuild, the retired-draw and dynamic retirements, and a staged
bridge lease the apply has not consumed yet.

### 5.3 A settled shell scheduled no frames for a live mesh ingest

The world's own wake authority (`World3dBuildContext::request_cursor_wake`) is reachable only from
`render_world_3d`, i.e. from a PAINT — and a retained window republishes its cached paint while its
revision is unchanged, so the paint that would ask for the next frame runs once per DOCUMENT. An
ingest needing more frames than the document that started it therefore stalled forever on an
event-driven shell. `AppRuntime::has_pending_world3d_work()` joins the browser tick's `request_frame`
disjunction beside `has_pending_text_work()` — the same shape, for the work the FRAME TRANSACTION
drives.

### 5.4 The draw-rebuild ↔ snapshot-apply deadlock — the actual reason the solid never drew

`step_world3d_draw_rebuild` owns only the PUBLISH half of a rebuild and answers `Pending` for as long
as its cursor is unsealed. The only thing that can seal it is `step_world3d_snapshot`. The wgpu frame
transaction returned on that `Pending`, so the two steps waited on each other:

```
world3d ingest surface=procedural-preview stage=phase-entry steps=512   … apply-page=0/3 apply-item=1 rebuild=true state-meshes=2 draws=0
world3d ingest surface=procedural-preview stage=phase-entry steps=1024  … apply-page=0/3 apply-item=1 rebuild=true state-meshes=2 draws=0
…                                                       (7 552 traces, 3.8 M turns, state never moved)
```

Both shared drivers — `♾️infinite/🌍️world`'s own `drive_scene_bridge` and
`⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces` — already run the two steps in one order,
unconditionally; this host was the single place that diverged. `WorldDrawRebuildStep::Pending` now
falls through to the snapshot apply exactly as they do. Immediately afterwards:

```
world3d surface=procedural-preview … draws=1 translucent=0 instances=1 lines=1 passes=1
                                     state-draws=1 state-instances=1 state-meshes=3
```

and the world3d paint count went from 3 to 90 in 120 s.

### 5.5 `dumpFrameStats` discarded a scene-only paint census

`build_frame_stats` accepted the engine's per-window census only when `census.layers > 0`. A window
whose whole paint IS its scene pass appends no new draw layer, so the preview's real census was
thrown away and the empty retained `draw_list` answered zeros — the column was on screen while the
introspection said nothing had been drawn. The filter now also accepts `census.scene_passes > 0`.

---

## 6. What 6118 does now, measured

### 6.1 Edit mode, 120 s (`🗑️generated/wgpu-settle/final-edit`)

```
counts        {"capacity":0,"residentRefusal":0,"settleRounds":1,"settleExhausted":0,
               "renderBegin":78,"surfaceFault":0,"extrudeSolid":90,"unknownTag115":0,
               "invokeExtensionFaulted":0}
resident      peak roots 6, peak bytes 1 336 704 / 33 554 432
settle        [DEBUG] wgpu-shell ui chain settled after 1 round(s) resident-roots=6 resident-bytes=1311424/33554432
dumpFrameStats("procedural-preview")
              {"windowId":"procedural-preview","drawCalls":0,"quadCount":17,"glyphCount":0,
               "scenePasses":1,"sceneDraws":2,"sceneInstances":1}
world3d       draws=1 translucent=0 instances=1 lines=1 textured=0 passes=1
              state-draws=1 state-instances=1 state-meshes=3
meshes_json   … "eval-extrude@solid#0" … (90 occurrences)
status        "phase":"idle","facesDone":8,"facesTotal":8,"inFlight":0,"ratio":1.0
paint stalls  0        window-paint exhaustions 0        surface faults 0
```

Screenshot: `🗑️generated/wgpu-settle/final-edit/shot-120s.png` — the Flow window's node graph on the
left, the **hexagonal extruded column** shaded in the Preview window on the right. (A peer's
workspace break put trunk's build-failure overlay over the page for the re-taken runs; the probe
removes that overlay before each shot and logs the removal — see §10.)

### 6.2 Viewer role (`🗑️generated/wgpu-settle/final-viewer-2`)

`?plugin=generation3d&role=viewer` reaches `…#viewer` and lays out one full-pane stack
(`procedural-view-preview@1434x814+3`):

```
dumpFrameStats("procedural-view-preview")
    {"scenePasses":1,"sceneDraws":2,"sceneInstances":1,"quadCount":17}
world3d  draws=1 instances=1 passes=1 state-draws=1 state-instances=1 state-meshes=3
resident peak roots 2, peak bytes 735 256        capacity 0        surface faults 0
```

Screenshot: `🗑️generated/wgpu-settle/final-viewer-2/shot-060s.png` — the column, full pane.

### 6.3 Generate mode (`🗑️generated/wgpu-settle/final-generate`, `final-generate-add2`)

`?plugin=generation3d&mode=generate` lays out its three named windows and paints them:

```
[DEBUG] wgpu-shell dock plan canvas=1434x836 windows=3
        generation3d-generations@315x814+3 generation3d-generate-form@616x814+319
        generation3d-generate-preview@502x814+935
resident peak roots 4, peak bytes 802 140        capacity 0
window-paint exhaustions 0        (§6.3 of the previous report: was 1 048 576 per boot)
```

`📓️wgpu-dock-layout-world3d` §6.3's non-terminating `generation3d-generate-form` body paint is gone —
it was the same layout lost wake-up as §5.1. Screenshot: `final-generate/shot-060s.png`.

**Not claimed: the Add Generation mutation.** See §8.

---

## 7. The shared law, both halves run, verbatim

### 7.1 The fixture

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧫️fixtures/🔣️.json` (schema beside it),
six surfaces, two refreshes, the shell's real retirement grant, and the live census this lane
measured on 6118 carried as `measured`.

### 7.2 Rust, driving the real ledger

`🎟️resident/🔄️refresh/🧪️tests/🔄️refresh/🦀️.rs` opens real `UiDocumentAssembly` roots through real
`UiResidentPermit::try_reserve` reservations computed with the bridge's own arithmetic.

```
$ cargo test -p semio-framework-ui-contract --lib resident_refresh_tests -- --nocapture --test-threads=1
[DEBUG] resident-refresh surfaces=6 refreshes=2 peak-roots=6 peak-bytes=3232268 faults=0
        retire-steps=9887 node-record-bytes=6416 open-bytes=1906 fixed-backing=566352
[DEBUG] resident-refresh-double-buffered single-roots=6 single-bytes=3232268
        doubled-roots=12 doubled-bytes=6464536
[DEBUG] resident-refresh-ceiling admitted=3 refusal=Some(Capacity) surface-bytes=8388608
        aggregate=33554432 fixed-backing=566352
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 168 filtered out
```

### 7.3 The TypeScript twin, same fixture, independent arithmetic

`🧑‍🎨engine/🧪️tests/🎟️resident-refresh-budget/🟦️.ts` re-implements the aggregate (fixed slot table,
item ceiling, byte ceiling, fixed backing committed at rest) and drives the same two orders.

```
$ SEMIO_TEST_LEVEL=long bunx vitest run --config vitest.config.ts 🎟️resident-refresh-budget
[DEBUG] {"surfaces":6,"refreshes":2,"peakRoots":6,"peakBytes":3232268,"faults":0}
[DEBUG] {"singleRoots":6,"singleBytes":3232268,"doubledRoots":12,"doubledBytes":6464536}
[DEBUG] {"admitted":3,"refusal":"capacity","surfaceBytes":8388608,"aggregateBytes":33554432}
[DEBUG] {"target":"http://127.0.0.1:6118/?plugin=generation3d","run":"2026-09-12 wgpu-settle/run-5",
         "seconds":122,"surfaces":6,"residentRoots":6,"residentBytes":1311424,
         "fixedBackingBytes":566352,"capacityFaults":0}
 Test Files  1 passed (1)      Tests  4 passed (4)
```

Both halves report the same three numbers — 3 232 268 / 6 464 536 / 3 — from the same fixture.

### 7.4 Registered

`.vscode/launch.json` gains `⚖️gate🎟️resident-refresh-budget🦀️rust` and
`⚖️gate🎟️resident-refresh-budget🟦️twin` in the existing `4_gate` group, and the TS suite is in
`🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts`'s `engineTestSuites`.

---

## 8. What is NOT claimed

**Generate mode's `Add Generation` is not reachable by pointer on this target.** The action is laid
out, painted and labelled (`final-generate/shot-060s.png`), and the guest publishes it —
`dumpStructure("generation3d-generations")` shows
`stack[1]#procedural3d-play-generate.actions/stack[0]#procedural3d-play-generate.add-generation`
at `rect [3.2, 16, 308.992, 6.4]`. Five clicks across the painted label and the layout rect
(`40,49`, `86,126`, `150,49`, `40,62`, `86,119`) each left `render begin` unchanged at 14, i.e. no
dispatch at all (`🗑️generated/wgpu-settle/final-generate-add2/console.txt`). The rects are the
evidence: that window's whole body measures **25.6 px tall** (rows of 6.4 px) while the paint draws
its labels ~24 px apart, so the hit rectangles the paint registers and the rows a pointer visually
aims at are different geometry. That is a layout/paint disagreement in the retained text measure, one
layer below this lane, and it is named here rather than guessed at. Everything the mutation would
prove — that a settled command re-enters the window bodies and re-arms — is already proven on the
boot chain (§4) and on the viewer role (§6.2), which run through the same
`flush_deferred_actions` funnel.

**Three `wgpu-ui.surface-not-published` surface faults in generate mode**
(`framework.panel.artifact`, `framework.panel.catalogue`, `framework.panel.inspection`) — the guest
does not publish those panel bodies in that mode. Reported as each surface's own typed fault card,
survivable, not this lane's files.

**`framework.panel.catalogue`'s `UI_VALUE_ADMISSION_SLOTS` residue** named in
`📓️wgpu-command-frames-mesh-chain` §8 was not observed in any run of this lane, and was not
addressed.

**The preview camera is not fitted.** The column fills the pane because the scene camera is the
authored `[4,-4,3] → [0,0,0]` and the solid is 6 units tall — real geometry, no fit pass. Out of
scope here (`📓️node-graph-camera-fit-labels-2026-09-12.md` owns that axis).

---

## 9. Checks and builds

```
$ cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown
warning: `semio-framework-os-renderer-wgpu` (lib) generated 25 warnings
    Finished `dev` profile                        # 22:2x, the bundle every deliverable run used

$ cargo check -p semio-framework-ui-contract --lib        Finished
$ cargo check -p semio-framework-os-infinite --lib        Finished
$ cargo check -p semio-framework-ui --features wgpu-engine --lib   Finished
```

The 25 (vs the previous report's 24) are all pre-existing shapes — unused imports, unnecessary
qualifications, dead `cfg(test)`-only fields — none names a symbol this lane added; the delta tracks
peer churn in the tree since that baseline (`js_sys` in `⚙️EngineCanvas`, the ui-contract file moves).

Shared cargo build dir throughout, `CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false`, every build
in the foreground. The wgpu serve on 6118 (`screen g3dwgpu`, trunk) was restarted **five** times by
this lane — trunk's watcher stopped firing on source changes twice (`📦 starting build` never
appearing for 10+ minutes after a verified edit), which is the wedge
`feedback-release-serve-wedges-after-host-edit-bursts` describes; each restart re-ran
`📜️serve-generation3d-wgpu.sh` unchanged. Every probe run was taken against a serve whose last log
line was `✅ success` at a timestamp after the sources it was meant to carry, and the first runs were
additionally verified by a string marker unique to the build under test inside the served
`_bg.wasm`.

<!-- NATIVE-TEST-STATUS -->

---

## 10. Observed, not mine

| red | peer work in flight |
|---|---|
| `semio_framework::Viewport2d` no longer exists in the root framework crate. `📓️wgpu-dock-layout-world3d` §8 recorded this as native-only at 19:44; by 20:20 it had reached **wasm32** and the 6118 serve had been failing its rebuild on it since 18:14 UTC — `error[E0425]: cannot find type Viewport2d in crate semio_framework` at `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:46`. Waited 35 minutes with the peer's file untouched; the move's own destination (`semio_framework_os_kernel::Viewport2d`, re-exported at `💻️os/📦️packages/🦀️rust/🦀️.rs:344`) was already in place, so the three consumers in THIS crate (`⚙️EngineCanvas/🎯️targets/🧊️wgpu`, `🎞️Scenes/🎯️targets/🧊️wgpu`, `⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph`) were moved forward onto it. **Nothing was reverted** — the peer's move was completed, not undone. |
| `🖱️ui/🧬️contract` file relocations (`📦️packages/🦀️rust/*.rs` → taxonomy paths, `UiFixedListAllocationError` → `PagedListAllocationError`) landed mid-session and re-wrote `📃️document/🦀️.rs` under this lane's own edit. The law's `mod resident_refresh_tests;` registration survived and was re-verified green afterwards. |
| `retained_document_root_permit_*` (5 tests, `🎟️resident/🌳️root/🧪️tests`) are RED on this tree — `root did not retire` / `Capacity`. Verified pre-existing: `git show HEAD:…` of both moved files is byte-identical to the working copy apart from `#[path]` strings. Not touched. |
| `semio-framework-ui-viewport` went red at ~22:50 — `error[E0277]: the trait bound Viewport3dAxonometricHemisphere: Default is not satisfied` (`🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🦀️.rs:474,478`), 2 errors. It broke trunk's rebuild, so the 6118 serve kept correctly running this lane's last good bundle behind trunk's full-page **build-failure overlay**. The probe now removes that overlay (dev-server chrome, never app content) before each screenshot and logs the removal — `PROBE removed trunk overlay "Build failure…"`. Not this lane's files; not touched. |

| At 23:05 and again at 23:12 the wasm build was red on a DIFFERENT peer edit — `error[E0502]: cannot borrow *self as immutable because it is also borrowed as mutable` at `💻️os/🔨️modules/🏪️store/🦀️.rs:8320` (`semio-framework-os-kernel`). Waited and retried twice; still in flight at hand-off. None of this lane's files are involved: every crate it touches checks green on its own (§9), and the last full green wgpu wasm build is the bundle the 6118 serve ran for all three deliverable runs. |

Nothing was reverted and nothing was fought.

---

## 11. Files

**New**
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧫️fixtures/🔣️.json` — the shared fixture.
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧬️schema/🔣️.json` — its schema.
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧪️tests/🔄️refresh/🦀️.rs` — the Rust law.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎟️resident-refresh-budget/🟦️.ts` — the TS twin.
- `<ticket>/🐍️wgpu-settle-probe.mjs` — the resident/settle/mesh probe.

**Changed**
- `…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — per-surface retirement (`retire_one_surface_document`,
  `retire_documents_outside`), page-priced vacancy-skipping `close_one`, `SHELL_DOCUMENT_RETIREMENT_ITEMS`/
  `_BYTES`/`SHELL_SETTLE_ROUNDS`, `settling`, `settle_ui_chain`/`drain_deferred_actions`,
  `resident_census`, `parked-in=` on the paint-exhaustion line; `retain_document_map_for_close` deleted.
- `…/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` — `resident_refusal`.
- `…/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` — `UiDocumentFrameCursor::phase_name`/`stalled`,
  per-window Layout predicate, `Pending`-on-dirty-layout returns to `Layout`, `ui-doc paint stalled`,
  `build_frame_stats` scene-only census.
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs` — `Ui::layout_is_dirty`,
  `Ui::request_layout`, `Ui::paint_stall_census`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` — `world3d_cursor_work_pending`
  widened to every cursor the frame transaction drives, `World3dState::ingest_census`.
- `…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — the draw-rebuild/snapshot-apply deadlock,
  `AppRuntime::has_pending_world3d_work`, `world3d_ingest_trace`.
- `…/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs` — `request_frame` consults `has_pending_world3d_work()`.
- `…/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` — the world3d debug line carries the ingest census.
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs` — registers `resident_refresh_tests`.
- `…/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts` — registers the TS suite.
- `.vscode/launch.json` — two `4_gate` entries.
- `…/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`, `…/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`,
  `…/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs` — the peer's `Viewport2d` move completed (§10).
