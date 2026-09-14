# ⏱️ wgpu EDIT-MODE CONVERGENCE — where the 3–5× against React went, and what closed it

Lane `wgpu-edit-convergence-perf`, ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, 2026-09-14.
Target: the coordinator's wgpu serve `http://127.0.0.1:6118/?plugin=generation3d`.

Repo MCP was down this session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`);
no ticket was opened, closed or reopened, `📓️status.md` and `🎫️ticket.json` were not touched, and no
git-state-modifying command was run.

---

## 1. TL;DR

| question | answer |
|---|---|
| **where did the 3–5× go?** | Not into CPU. A CPU profile of the live `semio-frame-worker` isolate during an edit-mode convergence was **87.6 % idle**. The wall time was a **sleep**: the retained-UI intake drive handed the isolate back with `requestAnimationFrame` once every `RETAINED_UI_INTAKE_SLICE_STEPS` (4 096) phases, and generation3d's flow-window document costs ~525 000 phases to publish — **128 animation frames = 2.13 s of pure idle per published surface**, seven of them inside one boot (§2). |
| **the second cost** | The same drive `await`ed a freshly-allocated promise on EVERY decoder phase, so one 525 k-phase publication cost 525 k promise allocations and ~1.05 M microtask ticks for ~0.20 ms of actual decoding (§3.2). |
| **what changed** | Both at the owning layer, `🐚️plugin-bridge/🟦️.ts`: a slice boundary is now a TASK, not a frame; and the cursor returns *what the caller must await*, which is nothing while the drive is inside its 8 ms hold budget (§4). |
| **proved on 6118** | `bun 🐍️wgpu-battery.mjs --only=examples`, twice, before and after, 16/16 green both times, 0 page errors. Edit mode **1.49–2.95× faster per example**; the worst example went 32.37 s → 13.49 s, the best 27.97 s → 9.47 s. The viewer lane also got 1.16–1.81× faster (§5). |
| **against React** | Mean edit-lane ratio to React **4.8× → 2.06×**. Four of the eight examples are now **within 2×** (1.35 / 1.60 / 1.66 / 1.75) and four are not (2.21 / 2.39 / 2.58 / 3.37). The brief's "within 2× for EVERY example" is **not** met, and §7 names the one remaining structural term with its measurement and its file:line. |
| **one fix made and REVERTED** | Input renumbering the frame generation underneath a live build cancels that build (28 supersessions in one convergence). Suppressing it took supersessions to 0 — and produced `offscreen prepared frame admission: prepared render revision is stale: live=35, packet=27`, a surface quarantine. It needs the presenter's freshness gate changed with it, so it is characterised in §6 and **not** shipped. |
| **the coordinator's status-lane addition** | Measured on the current build: during one live re-evaluation the wgpu host asked the guest for `procedural-preview` **78 times** and forwarded **54** World3d publications, every one of them `phase:"idle" facesTotal:0`. The host drops and coalesces **nothing**. §8. |

---

## 2. The profile, hop by hop — `hexagonal-mushroom-column`, edit lane, before

`🐍️wgpu-edit-perf-probe.mjs` (new, this lane): one page load, the full `[DEBUG]` console with
millisecond stamps, a per-trace GAP ledger, and a CPU profile of the frame Worker taken over that
Worker's own debugger socket (Playwright attaches only to page targets — the technique
`🐍️wgpu-spin-stack-probe.mjs` established). Evidence: `🗑️generated/wgpu-perf/edit-base-hex/`.

**The whole convergence happens inside `boot_shell`.** Nothing paints until it ends:

```
  1 923 ms  wgpu-shell render begin surface=procedural-main        ← the chain starts
 28 151 ms  wgpu-shell ui chain settled after 1 round(s)
 28 151 ms  boot-phase shell-boot:flush-deferred   21 627 ms
 28 152 ms  wgpu-worker boot_shell leave           27 261 ms
 28 959 ms  world3d surface=procedural-preview …                   ← the first mesh can land
 31 370 ms  converged
```

Accounting of the 32.5 s the page was alive: 1.9 s load + wasm/plugin parse, **21.1 s inside
`render begin`…`render leave`**, 2.9 s inside `invokeExtension dispatch`…`answered`, 4.4 s post-boot
frame loop.

**16.0 s of that 21.1 s is seven blocks, and every one of them is ~2.14 s:**

| block | surface | ms | intake phases that render consumed |
|---|---|---|---|
| 4 804 | `procedural-main` | 2 881 | 676 132 |
| 9 943 | `procedural-main` | 2 376 | 574 312 |
| 12 771 | `framework.panel.catalogue` | 2 144 | 525 153 |
| 17 922 | `framework.panel.catalogue` | 2 148 | 525 606 |
| 20 489 | `procedural-main` | 2 134 | 526 113 |
| 22 905 | `framework.panel.history` | 2 136 | 528 159 |
| 25 455 | `framework.panel.inspection` | 2 142 | **527 592** — for a **4-node** panel |

A four-node inspection panel costing the same 2.14 s as a 38-node flow window is the tell: the cost
is not the document. It is `525 153 / 4 096 = 128.2` slice boundaries.

### 2.1 What a slice boundary cost, measured inside the live Worker

`🐍️wgpu-raf-cost-probe.mjs` opens the frame Worker's own CDP target on the running 6118
page and times both primitives in the isolate that actually pays for them:

```
{"hasRaf":true,"hasOffscreen":true,"rafMsPerFrame":15.46,"portMsPerTask":0.0115}
```

`128.2 × 15.46 ms = 1 982 ms`, against the 2 134–2 148 ms measured. That is the whole block.
A `MessageChannel` task is **1 345× cheaper** than the animation frame that was being awaited.

And nothing in the frame Worker is animation-frame driven: the shell paints inside `tick()`
(`🌐️browser-worker/🦀️.rs:230`), which arrives as a `postMessage` **task** from the UI isolate
(`🚚️browser-frame-transport/🟦️.ts:433` `requestFrame` → rAF → `flush` → `worker.postMessage`). So the
awaited frame let nothing run that a task does not — it only slept.

---

## 3. Root causes, at file:line

### 3.1 A fixed-cadence animation frame in the intake drive

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`,
`WgpuUiIntakeCursor.next` (was line 347) —

```ts
if (this.#steps % RETAINED_UI_INTAKE_SLICE_STEPS === 0) await nextWgpuFrame();
else await yieldWgpuUi();
```

`nextWgpuFrame` (was line 326) preferred `requestAnimationFrame` "where the target has one (the
worker's `OffscreenCanvas` context does)". It does have one, and it costs 15.46 ms.

The irony is that the same file had already made this exact fix one level down: `yieldWgpuUi`'s own
docstring says *"A fixed every-Nth-step cadence cannot know what a step cost"* — and then the slice
boundary right beside it was a fixed every-Nth-step cadence.

`🐚️plugin-bridge/🟦️.ts` is the **only** user of `RETAINED_UI_INTAKE_SLICE_STEPS` as a yield cadence;
the constant's other three references are the declaration, the Rust twin
(`🖱️ui/🧬️contract/🛡️limits/🦀️.rs:79`) and the language-agnostic fixture. React never drives
`OwnedUiPatchIntake` at all, which is why the React target pays none of this.

### 3.2 One promise and two microtask ticks per decoder phase

`OwnedUiPatchIntake.advance` performs **exactly one** decoder phase per call, whatever grant it is
given (`🧱️elements/📃️UiDocumentStore/📥️intake/🟦️.ts:41` → `🧵️retained/📦️wire/🟦️.ts:262`'s
`#advance`, a single `switch` arm per call). `WGPU_UI_GRANT` is `{maxItems: 1, maxBytes: 4_096}` and
raising it would change nothing. So a 525 k-phase publication is 525 k `await`s, and `yieldWgpuUi`
was an `async` function that allocated a promise on every one of them even when its body returned
immediately. Measured after §4's first fix: 265 ms per publication, of which the decoding itself is
~0.20 ms of real work (the 4 096-phase slices finish well inside one 8 ms hold).

---

## 4. What changed

Both changes are in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`.

1. **`nextWgpuFrame` → `handBackWgpuIsolate`.** A slice boundary hands the ISOLATE back with the same
   unthrottled `MessageChannel` task every other yield uses, and restarts the hold budget. The slice
   keeps its declared meaning — a resumption point for a document larger than one slice — and stops
   being a sleep. `requestAnimationFrame` is now absent from the whole frame-worker bundle.
2. **`WgpuUiIntakeCursor.next` returns what the caller must await, or nothing.** `yieldWgpuUiIfDue()`
   reports the same deadline decision synchronously; `yieldWgpuUi()` is kept as the awaiting wrapper
   for the call sites that want it. Five call sites (`intake`, `publication-close`, `intake-close`,
   `owner-close`, `lifecycle-close`) and `#advanceMaintenance` now pay for a yield exactly when the
   8 ms budget really is spent.

Nothing else was touched: the grant, the ceiling, the slice constant, the intake state machine and
the 32-consecutive-zero-progress stall rule are all unchanged.

---

## 5. Proof on 6118 — every example, before and after

Both runs: `cd <ticket> && bun 🐍️wgpu-battery.mjs --only=examples`, the probe's own predicate
(a scene pass, a per-surface census carrying meshes AND a drawable, at least one published mesh whose
`positions`/`edgePositions` really starts with a number, no `[role=alert]`, stable across two samples,
75 s floor). **16/16 green in both runs, 0 page errors in both.**
Before `🗑️generated/wgpu-perf/examples-before.json` + `battery-before.txt` (3 259 s);
after `🗑️generated/wgpu-perf/examples-after.json` + `battery-after.txt` (3 217 s, `examples ok=true
16/16 pageerrors=0`).

React's numbers are `📓️react-end-to-end-verification-2026-09-13.md` §2, unchanged.

| example | lane | before s | **after s** | speed-up | React s | after ÷ React |
|---|---|---|---|---|---|---|
| Hexagonal Mushroom Column | edit | 32.37 | **13.49** | 2.40× | 4 | 3.37 |
| Rectangle Extrude Volume | edit | 36.03 | **15.47** | 2.33× | 7 | 2.21 |
| Rectangle Wire Preview | edit | 11.55 | **7.74** | 1.49× | 3 | 2.58 |
| Box Shell Preview | edit | 20.86 | **11.96** | 1.74× | 5 | 2.39 |
| Box Fillet Preview | edit | 22.88 | **9.59** | 2.39× | 6 | **1.60** |
| Sphere Cut With Torus | edit | 29.88 | **11.65** | 2.56× | 7 | **1.66** |
| Sphere Box Fuse | edit | 27.97 | **9.47** | 2.95× | 7 | **1.35** |
| Face Sweep Extrude | edit | 37.88 | **13.97** | 2.71× | 8 | **1.75** |
| Hexagonal Mushroom Column | viewer | 8.64 | **6.52** | 1.33× | 3 | 2.17 |
| Rectangle Extrude Volume | viewer | 7.72 | **5.44** | 1.42× | 3 | 1.81 |
| Rectangle Wire Preview | viewer | 6.52 | **5.45** | 1.20× | 3 | 1.82 |
| Box Shell Preview | viewer | 6.52 | **5.51** | 1.18× | 3 | 1.84 |
| Box Fillet Preview | viewer | 7.58 | **6.55** | 1.16× | 3 | 2.18 |
| Sphere Cut With Torus | viewer | 8.75 | **5.46** | 1.60× | 4 | 1.36 |
| Sphere Box Fuse | viewer | 7.75 | **4.27** | 1.81× | 4 | **1.07** |
| Face Sweep Extrude | viewer | 7.69 | **5.38** | 1.43× | 4 | 1.34 |

Mean edit-lane ratio to React: **4.8× → 2.06×**. Mean viewer-lane ratio: 2.5× → 1.70×.

### 5.1 The same convergence, instrumented before and after

`🗑️generated/wgpu-perf/edit-base-hex/`, `…/edit-fixA-hex/`, `…/edit-fixAB-hex/` — the hexagonal
column, edit lane, one probe:

| reading | before | after §4.1 | after §4.1 + §4.2 |
|---|---|---|---|
| converged | 31.37 s | 14.51 s | 11.01 s |
| `boot_shell` | 27 261 ms | 8 226 ms | **7 636 ms** |
| `shell-boot:flush-deferred` | 21 627 ms | 5 831 ms | **5 445 ms** |
| Σ `render begin`…`render leave` | 21 102 ms | 4 640 ms | **4 213 ms** |
| Σ retained-intake accept | — | 3 260 ms | **2 722 ms** |
| worst single surface render | 2 881 ms | 550 ms | **350 ms** |
| frame-Worker CPU idle during the chain | **87.6 %** | 57.9 % | 64.7 % |

Run-to-run spread on the same build is real (11.01 / 12.58 / 13.49 s for the same lane across three
runs, with eight other agents building Rust on the same machine), which is why §5's table is the
claim and this one is the mechanism.

---

## 6. Made, measured, and REVERTED — input must not renumber a live build

`🪟️winit-app/🦀️.rs:133` already states the law: *"the frame generation names the INPUT STATE a build
is answering, so it advances when input changes … and when a redraw finds no build to invalidate —
**never underneath a live one**"*. `redraw_core` honours it (`else if !self.frame_build.has_live_session()`).
`enqueue_host_event`/`enqueue_host_metrics` did not: every pointer move renumbered the generation,
`FrameBuildHandle::poll_runtime_and_resubmit` read that as `frame build superseded`, and **cancelled
the in-flight build from phase 0**.

Measured (`🗑️generated/wgpu-perf/edit-fixA-hex/` vs `…/edit-fixA-hex-nonudge/`): the same build, same
example, converged in **14.51 s while the probe nudged the pointer 5×/s and in 10.69 s with the
pointer still** — a 26 % tax on a chain the input had nothing to do with. 28 `frame build superseded`
in the moving run, 0 in the still one.

Gating the renumber on `has_live_session()` (a renderer wasm build, `🗑️generated/wgpu-perf/edit-fixABC-hex/`)
took supersessions **28 → 0** and the convergence to 10.79 s — and the next run quarantined the
surface:

```
os_host present_step faulted: offscreen prepared frame admission:
  prepared render revision is stale: live=35, packet=27
wgpu renderer fault: worker-present-failed
```

A build that is allowed to run to completion now completes against a render revision the presenter
refuses. The supersede and the presenter's prepared-revision freshness gate are **one decision in two
places**, and moving only the first one trades a 26 % delay for an intermittent dead surface. The
change was fully reverted (`🪟️winit-app/🦀️.rs` and its callback-latency suite are back at `HEAD`
apart from a peer's own wheel hunk), the renderer wasm was rebuilt without it, and §5's table is on
that rebuilt wasm. It belongs to the frame-build / presentation lane as a paired change — the same
spin `📓️wgpu-frame-loop-after-selection-2026-09-13.md` §7.1 already assigned there.

---

## 7. The one structural term left, with its number

**`refresh_ui` re-renders every surface on every settle round, because the wgpu shell ignores
`UiDirtyScope`.**

`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4171` `refresh_ui` walks every live window and then every
panel leaf unconditionally. In the before run that was 137 `renderSurface` calls of which **116
returned `patched=0`** — the guest had nothing to say about that surface. The ones that do patch are
the expensive ones: `procedural-main` is re-minted **eight times at ~525 000 intake phases each**,
~215 ms apiece after §4, ≈1.7 s of the remaining 5.4 s `flush-deferred`.

React does not do this. `🧱️elements/🏛️ShellHost/🟦️.tsx` imports `resolveUiDirtyScope` (line 105),
`mergeUiDirtyScopeV1` (line 490) and threads a `UiDirtyScope` through its refresh lane (lines 4668,
4889, 4895, 4920, 4927, 4970) — the guest already reports which surfaces it dirtied on every
`InvocationResult.ui_scope`, and the wgpu shell throws that field away. Honouring it is the brief's
"a settle that changes one window's document must reconcile exactly that surface", it is the largest
remaining term, and it is a lane of its own: a wrong scope is a surface that silently stops updating,
and the wgpu Rust law suite could not be run this session (§9).

---

## 8. The coordinator's addition — the window-transient `status` lane

**The wgpu host neither drops nor coalesces it. It polls the producer about twice a second and
forwards every answer; the producer answers `idle` every time.**

Measured on the CURRENT build with `SEMIO_PROBE_PICK=1 bun 🐍️wgpu-status-pill-recon.mjs`
(`🗑️generated/wgpu-perf/status-picked-now/`): boot with no example, wait until the World3d surface is
live and painting (3.2 s), then drive the shell's own picker to `shell.example.sphere-cut-with-torus`
(located from the shell's own `os_host pointer hit` trace) and watch the whole evaluation. In the
40 s window covering it:

| what the host did | count |
|---|---|
| `wgpu-shell render begin surface=procedural-preview` — the host ASKING the guest | **78** |
| `wgpu-bridge renderSurface surface=procedural-preview` — the guest ANSWERING | **78** |
| `world3d surface=procedural-preview` — the host PUBLISHING to the renderer | **54** |
| `flowEvalTick settled` / `invokeExtension answered` / `typed-operation command` | 7 / 5 / 8 |
| distinct `phase` values in those 54 publications | `{"idle": 54}` |
| distinct `facesTotal` values in those 54 publications | `{"0": 54}` |

`firstNonIdle = null`, `pills = 0`, `computingPills = 0` over 140 s. The same shape holds in the
earlier capture `🗑️generated/wgpu-a11y-runtime/status-picked-2/`: 61 host renders of the preview
during that 23 s evaluation, 2 distinct statuses, both `idle`.

So there is no coalescing to remove: 78 asks produced 78 identical `idle` answers. This confirms
`📓️wgpu-a11y-status-i18n-runtime-2026-09-14.md` §3.3 independently and on a newer build — the owning
layer is the generation3d preview-eval projection and the flow host's progress ledgers
(`✏️s/…/🧊️generation3d/…/🧵️preview-eval/🦀️.rs` `preview_scene_status_json`,
`🧰️framework/…/🌊️flow/🖥️host/🦀️.rs`), where `tick_scheduled` is false at every hop boundary of a
`flowEvalTick` chain and both ledgers are empty.

**The brief's premise does not hold as stated.** "On React the same guest publishes phase/ratio/
facesDone per step (React battery `status-parity` 10/10)" — React's own artifact
`🗑️generated/react-verify/status-parity/results.json` records **10 statuses and every one of them is
`phase: "idle"`** (plus 5 nulls). That probe's 10/10 asserts *"every preview host carries a non-null
status object"* (`🐍️react-battery.mjs:239`), not that progress crossed per step; and neither
`status-parity/console.txt` nor `journey/console.txt` contains a single `phase` or `facesDone`
reading. I found no evidence, on either renderer, that this guest ever publishes a non-idle phase —
so I did not change the wgpu host to "restore" something it is not losing.

`bun 🐍️wgpu-battery.mjs --only=status-a11y-i18n` was therefore **not** re-run as a proof of a fix:
there is no host fix here to prove, and its `status:pill-while-computing` step will stay red until the
producer publishes a non-idle phase. What §5's run does show is that the 78 asks → 54 identical
publications are pure waste, which is the same §7 item.

---

## 9. Laws

| law | where | count |
|---|---|---|
| `slice-yield-is-a-task-not-a-frame` | `🖱️ui/🧬️contract/🧵️retained/🧫️fixtures/📥️intake/🔣️.json` `laws[]` | the language-agnostic declaration |
| the Rust twin asserts that law name | `🖱️ui/🧬️contract/🧪️tests/🔬️limits-unit/🦀️.rs` `retained_ui_intake_budget_matches_the_shared_fixture` | **ran, passed** (`cargo test -p semio-framework-ui-contract --lib …` → `1 passed`) |
| *"crosses a slice boundary with a TASK and never with an animation frame"* — installs a `requestAnimationFrame` spy, drives 3 slices, demands **0** frames | `🧑‍🎨engine/🧪️tests/📥️wgpu-intake-budget/🟦️.ts` | new |
| *"owes the isolate NOTHING for a step inside its hold budget, and a task only at the slice boundary"* | same suite | new |
| the four laws that suite already had (300 KB ceiling, fixture parity, slice resumability, terminal ceiling) | same suite | updated to the new `next` contract |
| the whole suite | same suite | **7/7 green** |

**The failing-first check was run, not assumed.** With the animation frame restored in
`handBackWgpuIsolate` the new law fails exactly as intended —
`AssertionError: … expected 3 to be +0` (three slices, three frames) — and passes with the fix.

---

## 10. Files

Changed:

* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`
  — `handBackWgpuIsolate` replaces `nextWgpuFrame`; `yieldWgpuUiIfDue`; `WgpuUiIntakeCursor.next`
  returns `Promise<void> | undefined`; six call sites await only what is returned.
* `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧫️fixtures/📥️intake/🔣️.json` — the new law name.
* `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️limits-unit/🦀️.rs` — the Rust twin assertion.
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📥️wgpu-intake-budget/🟦️.ts` — two
  new laws, four updated.

Created, in the ticket root:

* `🐍️wgpu-edit-perf-probe.mjs` — the hop-by-hop convergence profiler (console + GAP ledger + frame-Worker
  CPU profile over its own debugger socket).
* `🐍️wgpu-raf-cost-probe.mjs` — times an animation frame against an unthrottled task INSIDE the live
  `semio-frame-worker` isolate, over that Worker's own debugger socket (§2.1).

Evidence, under `🗑️generated/wgpu-perf/`: `edit-base-hex/`, `edit-fixA-hex/`, `edit-fixA-hex-nonudge/`,
`edit-fixAB-hex/`, `edit-fixABC-hex/`, `edit-final-hex/`, `edit-final2-hex/`, `status-picked-now/`,
`examples-before.json`, `examples-after.json`, `battery-before.txt`,
`battery-after.txt`, `renderer-wasm*.txt`.

Report: this file.

---

## 11. Fix-forward on peers' work, and what the environment did to the measurements

* **The `pk:` invocation migration.** Rebuilding the frame worker shipped a peer's uncommitted
  TypeScript half of the `handleAction`/`handleCommand`/`renderDocument` pack migration
  (`🐚️plugin-bridge/🟦️.ts` `packValueFromBase64(invocationPack)`) against a renderer wasm that
  predated its Rust half, and 6118 stopped booting with
  `shell-boot: handle_action failed: packValueFromBase64: expected pk: prefix`. Fixed forward by
  building the renderer wasm (`CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false bunx nx run
  @semio-tech/framework-renderer-wgpu:wasm`), which completes that peer's migration on the serve.
  Nothing of theirs was reverted.
* **A puzzle-3d compile break** blocked the first wasm build
  (`match arms have incompatible types … expected trait InteractiveJob + Send, found trait
  ToolRunRetargetableJob<Puzzle3dConfig>`, `✏️s/…/🧊️3d/…/✏️editor/🦀️.rs:7683`). Its owning lane was
  editing the file at that second and had fixed it 90 s later; the retry built clean. Not touched.
* **`nx run @semio-tech/framework-renderer-wgpu:test-quick` could not be run**: it builds Rust first
  and a peer's tree had `error[E0433]: cannot find module or crate semio_framework_trace` in
  `semio-framework` (lib). The TypeScript laws were therefore run directly through the engine vitest
  config, and the Rust fixture law through `cargo test -p semio-framework-ui-contract`. The wgpu
  renderer's own Rust law suite was **not** run this session.
* **The before-battery spans two renderer wasm builds.** A peer rebuilt the renderer at 09:15, eight
  minutes into the 54-minute before run. The before numbers therefore hold across that build, not
  under it — which makes the speed-ups conservative, not inflated, since the after run is on a strictly
  later renderer.
* Eight to ten peer `cargo` processes were running throughout both batteries.

---

## 12. What is NOT claimed

1. **"Within 2× of React for every example" is not met.** Four of eight edit-lane examples are
   (1.35 / 1.60 / 1.66 / 1.75); four are not (2.21 / 2.39 / 2.58 / 3.37). §7 names the remaining term.
2. **The `UiDirtyScope`-scoped refresh was not implemented.** It is measured (116 of 137 renders were
   `patched=0`; the flow window is re-minted 8× at ~525 k phases ≈ 1.7 s) and located, not fixed.
3. **Input no longer superseding a live frame build was reverted**, for the surface quarantine in §6.
   The 26 % it is worth is measured, not delivered.
4. **No guest, plugin or React-target code was changed**, and no status/progress behaviour was changed:
   §8 is a measurement that contradicts the brief's premise, not a fix.
5. **The ~525 000 intake phases per flow-window publication were not reduced.** They are the honest
   phase count of the wire payload the guest emits (`OwnedUiPatchIntake` advances one decoder phase per
   call at any grant); only their *cost* was reduced, from 2.13 s of sleep to ~215 ms of work. Whether
   the guest should be emitting a full-document patch instead of a delta is a guest question this lane
   did not touch.
6. **Only `--only=examples` was re-run.** The other battery rows in
   `🗑️generated/wgpu-verify/scoreboard.json` are carried from earlier lanes' runs, including a red
   `generate-add` row this lane never executed. No claim is made about them.
7. **No native/winit proof.** Everything here is the browser worker path on 6118.
