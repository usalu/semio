# 🧮️ wgpu EXAMPLE CHAIN — all eight examples, three boot lanes, measured (2026-09-13)

Lane **wgpu-example-chain** · ticket `26/09/09/PROCEDURAL-3D-END-TO-END` · server `http://127.0.0.1:6118/?plugin=generation3d`

Abbreviations: `wgpu-shell.rs` =
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`;
`bridge.ts` = `…/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`;
`flow-host.rs` = `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`;
`T` = this ticket folder.

---

## 1. TL;DR

| | |
|---|---|
| **Where it started** | Nothing booted at all. On 6118 the shell stopped forever at `shell-boot 86%` after the FIRST `render begin`, in every mode and for every example — the state `📓️wgpu-chrome-parity-2026-09-13.md` §6 had recorded and left. React on 6018 was wedged too. |
| **Root cause of the boot hang** | A spinning guest turn, not a host one: `FlowHost::retire_cold`'s `while !close_page(…) {}` never terminates because `FlowHostRetirement::close_page`'s `state.domain` branch only ever CLOSED, and `FlowRetirement`'s `close_step` answers `Blocked` — never an error, never progress — while `next_allocation_bytes()` still names a page. Named from a CPU profile of the blocked shard worker, §3. |
| **How it was proven** | Two new probes. `T/🐍️wgpu-shard-liveness-probe.mjs` separates "compute-bound" from "message lost" (a blocked worker stops answering `evaluate`; one core pinned at 97 % for 300 s). `T/🐍️wgpu-spin-stack-probe.mjs` then talks raw CDP to that worker's own target and returns a CPU profile **and** a `Debugger.pause` stack — 12 demangled frames from `Generation3dPlayApp::render_with_request_context` down to `PagedList::next_capacity_allocation_bytes`. |
| **Who fixed it** | NOT this lane. The fix was a peer's uncommitted reserve-then-close ladder in `flow-host.rs` (`📓️fix-forward-set-contributions-hang-2026-09-13.md`). This lane proved that was the cause and **restaged the procedural guest** so the fix reached the browser. Boot then converged in **3.5 s**. |
| **`?example=` boot axis** | **New, this lane.** `?example=<id>` now opens a named example directly on BOTH ports, so no example needs a pointer gesture to be reached. Contract in the shared boot-query module, threaded UI-isolate → transport → frame worker → `semioWgpuSetBootExample` → `sync_session_chrome` → `apply_boot_example`'s `setActiveExample`. §4. |
| **Edit mode** | **8/8 converge to a rendered mesh**, 10.9–37.0 s. §2. |
| **Viewer role** | **6/8 converge**, 5.4–8.7 s. The two failures are boolean examples that stall with one kernel unit permanently `inFlight`; deterministic to the digit across four independent runs. §6.1. |
| **Generate mode** | **0/8 before `addGeneration`, and that is the honest reading** — the generate preview has no generation to show until one is added. The `Add Generation` row is published, hit-testable and the pointer reaches it, but the click **does not dispatch**. §6.2. |
| **Laws** | 2 new Rust laws (`--test-threads=1`, **20/20 passed** in the chrome-parity suite), 3 new TypeScript twin tests (**3 passed**), 3 new rows in the shared boot-descriptor oracle (**12 cases, passed**), 1 new Rust law on the `UiFixedMap` refusal vocabulary. §5. |
| **NOT claimed** | §8. |

---

## 2. Per-example table

Probe: `T/🐍️wgpu-example-matrix-probe.mjs`, run `T/🗑️generated/wgpu-examples/matrix-2/`, one page load per cell,
budget 150 s, 1440×900, headless Chromium with `--enable-unsafe-webgpu`.

**Convergence is the pair `📓️wgpu-resident-budget-settle-2026-09-12.md` §6.1 proved the hexagonal column
with, and nothing weaker** — all four, stable across two samples:
1. the frame ledger reports `scenePasses > 0`;
2. the renderer's own per-surface World3d census carries `state-meshes > 0` AND a drawable (`instances > 0`
   for a solid body, `lines > 0` for a wire one);
3. at least one published mesh carries REAL geometry — a `positions` or `edgePositions` array that actually
   starts with a number;
4. no `[role=alert]`.

Clause 3 is not pedantry. generation3d's preview always ships a `…@wire#0` companion whose `positions` are
`[]`, so "the mesh list is not `[]`" passes on an empty scene; and `rectangle-wire-preview` is a wire example
whose only geometry is `edgePositions` with `instances=0`, so demanding `positions`/`sceneInstances` fails a
body that is on screen. Both mistakes were made and corrected while building this probe.

### 2.1 Edit mode (`?mode=edit`, editor role) — 8/8

| example | verdict | time-to-mesh | guest progress at convergence | evidence |
|---|---|---|---|---|
| `hexagonal-mushroom-column` | **pass** | 30.75 s | `facesDone 6/8, ratio 0.545` | `matrix-2/hexagonal-mushroom-column/edit/` |
| `rectangle-extrude-volume` | **pass** | 37.01 s | `facesDone 6/6, ratio 1` | `…/rectangle-extrude-volume/edit/` |
| `rectangle-wire-preview` | **pass** | 10.86 s | `facesDone 0/0, ratio 1` (wire) | `…/rectangle-wire-preview/edit/` |
| `box-shell-preview` | **pass** | 24.69 s | `facesDone 12/12, ratio 1` | `…/box-shell-preview/edit/` |
| `box-fillet-preview` | **pass** | 17.97 s | `facesDone 26/26, ratio 1` | `…/box-fillet-preview/edit/` |
| `sphere-cut-with-torus` | **pass** | 28.73 s | `facesDone 3/3, ratio 1` | `…/sphere-cut-with-torus/edit/` |
| `sphere-box-fuse` | **pass** | 27.81 s | `facesDone 7/7, ratio 1` | `…/sphere-box-fuse/edit/` |
| `face-sweep-extrude` | **pass** | 35.96 s | `facesDone 6/6, ratio 1` | `…/face-sweep-extrude/edit/` |

A partial `ratio` at convergence is not a defect: the first complete face is already a drawable body, and the
probe stops at the first stable sample that satisfies all four clauses rather than waiting for `ratio 1`.

### 2.2 Viewer role (`?role=viewer`) — 6/8

| example | verdict | time-to-mesh | guest progress when the probe gave up |
|---|---|---|---|
| `hexagonal-mushroom-column` | **pass** | 7.78 s | `8/8, ratio 1` |
| `rectangle-extrude-volume` | **pass** | 8.66 s | `6/6, ratio 1` |
| `rectangle-wire-preview` | **pass** | 6.68 s | `0/0, ratio 1` (wire) |
| `box-shell-preview` | **pass** | 6.51 s | `12/12, ratio 1` |
| `box-fillet-preview` | **pass** | 6.39 s | `26/26, ratio 1` |
| `sphere-cut-with-torus` | **fail** | — | `samplingEdges`, `unitsDone 0/0`, **`inFlight 1`**, `ratio 0.0` |
| `sphere-box-fuse` | **fail** | — | `meshingFaces`, `unitsDone 24/41`, `facesDone 7/7`, **`inFlight 1`**, `ratio 0.5853658536585366` |
| `face-sweep-extrude` | **pass** | 5.39 s | `6/6, ratio 1` |

### 2.3 Generate mode (`?mode=generate`) — 0/8 before a generation exists

All eight cells boot the three-pane layout (`generation3d-generations`, `generation3d-generate-form`,
`generation3d-generate-preview` — dock plan `315×814+3,54 / 616×814+319,54 / 502×814+935,54`) and in all eight
the guest's own evaluation reaches `ratio 1.0` (`8/8`, `6/6`, `0/0` wire, `12/12`, `26/26`, `3/3`, `7/7`,
`6/6`). What never carries a drawable is `generation3d-generate-preview` itself: `instances=0 draws=0`,
`state-meshes=2` (the seeded wire companions), no geometry evidence, in every cell and identically. That is
the expected reading of an empty Generations list, not a renderer fault — a generation must be added first,
which §6.2 covers. Note the contrast with §6.1: here the evaluation finishes and the surface is empty; there
the surface is empty because the evaluation never finishes.

### 2.4 Against React

React's journey (`📓️summary-2026-09-12.md`) converges the hexagonal column in **12–13 s** and the brief's
band for the eight is **3–78 s**. wgpu viewer (5.4–8.7 s) is FASTER than React's headline figure; wgpu edit
(10.9–37.0 s) sits inside React's band, at its slower end. Edit costs more than viewer on this target because
it renders five window surfaces plus three panels per refresh against the viewer's one
(`matrix-2/sphere-box-fuse/edit/console.txt` line 326 ff: `procedural-main`, `procedural-preview`,
`framework.panel.artifact`, `framework.panel.catalogue`, `framework.panel.inspection` all re-render after
every extension answer). No example exceeds React's band.

---

## 3. Root cause #1 — the boot hang, and how it was named

### 3.1 The symptom, stated exactly

`T/🗑️generated/wgpu-examples/trace-2/console.txt`: boot reaches
`[DEBUG] wgpu-shell render begin surface=procedural-main body=procedural.play.main` at 3 613 ms and then
prints **nothing for 300 s**. `[role=status]` freezes at `shell-boot 86%`, `[role=alert]` stays null, no
fault, no `Capacity`, no budget message, and the shard pool's own 180 s heartbeat never fires.

Temporary instrumentation in `bridge.ts` (`renderSurfaceSerialized`, since removed — the file is byte-identical
to its committed state) placed the stop precisely: `renderSurface enter` printed, the following
`surface-visible returned` never did. The hang is inside the FIRST
`submitTurn(actorId, [{kind: "surface-visible", …}])` (`bridge.ts:931`), before any intake or projection.

### 3.2 Compute-bound, not a lost message — `T/🐍️wgpu-shard-liveness-probe.mjs`

The shard workers are page-level dedicated workers (`MainThreadShardWorker` posts `shard-spawn` up to the UI
isolate, which constructs the real `Worker`), so Playwright's `page.workers()` sees all five. A
`worker.evaluate()` that does not answer within the ping window proves that worker's event loop is BLOCKED.

`T/🗑️generated/wgpu-examples/shard-liveness/samples.json`, 14 samples over 130 s: the frame worker and three
of four shard workers answer in 0–17 ms every time; **shard worker #0 never answers**, and the busiest browser
process sits at **93–100 % CPU** for the whole window. A deadlocked wait parks at 0 %; this was a spin.

### 3.3 The spinning frame — `T/🐍️wgpu-spin-stack-probe.mjs`

Playwright's `CDPSession` does not attach to worker targets, so the probe launches Chromium with
`--remote-debugging-port`, reads `/json/list`, and opens the blocked worker's own `webSocketDebuggerUrl`
directly. It then takes both readings, because either can come back empty on a wasm stack: a 4 s `Profiler`
sample reported as a self-time ranking, and one `Debugger.pause` with its `callFrames`.

Self-time ranking (`T/🗑️generated/wgpu-examples/spin-wgpu/findings.json`), all inside
`semio_s_plugin_procedural_component.core.wasm`:

| % | frame (demangled) |
|---|---|
| 42.0 | `PagedList<FlowOwner, FLOW_RETIREMENT_FRONTIER_OWNERS>::next_page_allocation_bytes` |
| 23.5 | `<FlowRetirement as ErasedSnapshotRetirement>::close_step` |
| 6.3 | `PagedList<…>::next_capacity_allocation_bytes` |
| 6.0 | `PagedList<…>::page_items` |
| 5.1 | `FlowHostRetirement::close_page` |
| 3.2 | `FlowHost::with_fixture::<…, generation3d…modes::edit::windows::flow::render>` |
| 3.0 | `semio_framework_artifact_flow::flow::retained::owner_continuation_slots` |

`Debugger.pause` call chain, innermost first — the same ladder, confirming the profile is not aliasing:

```
PagedList<FlowOwner, …>::next_capacity_allocation_bytes
FlowRetirement::next_allocation_bytes
<FlowRetirement as ErasedSnapshotRetirement>::close_step
FlowHostRetirement::close_page
FlowHost::retire_cold
FlowHost::with_fixture::<(Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>, Vec<NodeGraphOperatorRecord>), …>
…generation3d::editor::generation3d::modes::edit::windows::flow::render
…generation3d::editor::generation3d::component::generation3d_render_body
<Generation3dPlayApp as ArtifactEditor>::render_with_request_context
```

### 3.4 The defect, with file:line

- `flow-host.rs:2500-2502` — `FlowHost::retire_cold` is `let mut retirement = FlowHostRetirement::new(self);
  while !retirement.close_page(1, 4096).expect("cold flow host retirement") {}`: an UNBOUNDED close-only loop.
- `flow-host.rs:2357-2395` — `close_page`'s `state.domain` branch called only `close_step`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs:316-325` —
  `<FlowRetirement as ErasedSnapshotRetirement>::close_step` returns `Blocked` (not an error, not progress)
  whenever `next_allocation_bytes()` still names a page.
- `…/🧵️retained/🦀️.rs:226-236` — `FlowRetirement::retire_cold`, the correct twin, pays the reservation first:
  `loop { while let Some(bytes) = next_allocation_bytes() { reserve_allocation(bytes) } … close_step(…) }`.

So a driver that only closes spins forever on any fixture whose widgets claim continuation slots — which is
every generation3d document, because the edit-mode flow window's `render` builds a host per render through
`FlowHost::with_fixture`.

### 3.5 Fix and proof

The reserve-then-close ladder in `close_page` is a **peer's** change
(`📓️fix-forward-set-contributions-hang-2026-09-13.md`), already in the working tree but NOT in the staged
guest (`2026-09-12 06:56`). This lane restaged it —
`bun nx run @semio-tech/framework-os-dev:activate-generation3d-wgpu-dev`, 31 m 35 s,
`T/🗑️generated/wgpu-examples/restage-1.txt` — which republished
`🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/…core.wasm` (96 MB → 69 MB).

Immediately afterwards (`T/🗑️generated/wgpu-examples/after-restage/`):
`boot:edit-editor: pass in 3.48s`, `renderBegin=84 renderLeave=84`, `Capacity=0`, and the guest published
`eval-extrude@solid#0` with `facesDone 8, facesTotal 8, ratio 1.0`.

---

## 4. `?example=` — the boot axis this lane added

The audit (`📓️audit-wgpu-journey-readiness-2026-09-13.md` §1 step 5) proposed it and the chrome-parity lane
explicitly left it (`📓️wgpu-chrome-parity-2026-09-13.md` §1, "Not done, deliberately"). It is done now, and on
**both** renderers, so one url opens the same document on 6018 and 6118.

| hop | file | change |
|---|---|---|
| contract | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔗️boot-query/🟦️.ts` | `BOOT_QUERY_EXAMPLE_PARAM`, `resolveBootQueryExampleId(search, fallback)` — the third axis beside `?plugin=`/`?role=`, in the module that already declares them |
| React entry | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts:46-51` | `defaults.exampleId` is now the query over the `VITE_SEMIO_DEFAULT_EXAMPLE` seed — a DEFAULT, never a lock, so the picker stays live |
| wgpu UI isolate | `…/🧊️wgpu/🚀️browser-boot/🟦️.ts:45-56, 258` | `bootDescriptor().appExample`, bounded by the same `BOOT_FIELD_CAPACITY` every other field uses |
| transport | `…/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts:167-170` | `BrowserFrameWorkerBoot.appExample` |
| frame worker | `…/🧊️wgpu/🎞️frame-worker/🟦️.ts:46, 482` | `loaded.semioWgpuSetBootExample?.(message.appExample ?? "")` |
| renderer | `…/🧊️wgpu/🧊️renderer/🦀️.rs` region `📚️ExampleBoot` | `BOOT_APP_EXAMPLE` thread-local, `#[wasm_bindgen(js_name = semioWgpuSetBootExample)]`, `boot_app_example()` — the same idiom as `?role=`/`?mode=` |
| shell resolution | `wgpu-shell.rs:3652` (`sync_session_chrome`) | resolves through the new shared `resolve_boot_example_id` with `crate::boot_app_example()` as the declared default, replacing an inline first-or-keep rule |
| shell announcement | `wgpu-shell.rs:3613` (`apply_boot_example`), called at `:3594` | dispatches `setActiveExample` so the GUEST opens that document, between `push_contributions` and the first `refresh_ui` — after the flow-extension registry is armed, before the first render |

`resolve_boot_example_id` (`wgpu-shell.rs:9248`) is a true twin of React's `resolveBootExampleId`
(`🧱️elements/🐚️Shell/🟦️.tsx:277`), not a second rule: live selection ≻ declared default ≻ the dialect's first
example, and an id the open dialect does not author falls back rather than faulting — the rule `?role=` and
`?mode=` already follow.

Runtime proof, every cell of §2: `[DEBUG] wgpu-shell boot example {"controller":"s.procedural.generation3d@1/*#editor","exampleId":"box-fillet-preview"}`
and the viewer counterpart with `…#viewer`. On React, `http://127.0.0.1:6018/?plugin=generation3d&example=box-fillet-preview`
opened that document with a live `procedural-preview` World3d host carrying 15 `data-*` carriers.

---

## 5. Laws

| law | where | result |
|---|---|---|
| `the_boot_example_is_resolved_the_way_the_shared_fixture_declares` | `🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs` | 8 shared fixture rows (precedence, both fallbacks, the empty dialect) |
| `the_boot_example_query_reaches_the_picker_through_the_shared_resolver` | same | a source law: `sync_session_chrome` resolves through the shared predicate and is handed `boot_app_example()`; `apply_boot_example` dispatches `setActiveExample`; and `push_contributions < apply_boot_example < refresh_ui` inside `settle_boot` |
| `📚️ boot example contract` (3 `it`s) | `🧪️tests/🔬️engine-contract/🟦️.ts` | ajv-validates the fixture, replays all 8 rows through the shipped `resolveBootExampleId`, and all 4 `?example=` rows through `resolveBootQueryExampleId` incl. the overflow refusal |
| `a_carrier_map_refusal_names_capacity_and_page_grant_apart` | `🖱️ui/🧬️contract/🧪️tests/🛍️catalogue-carrier-map/🦀️.rs` | the two refusals are distinguishable, and the decoder surfaces the same vocabulary — `capacity="UiFixedMap is full at its 32-entry capacity"`, `grant="UiFixedMap admission ran out of page grant at entry 2 of at most 32"` |
| boot-descriptor oracle | `🧫️fixtures/🧊️wgpu-browser-boot-cache-inputs/🔣️.json` | 9 → **12** cases; `appExample` added to every expectation, 3 new `?example=` cases (named, beside `role`+`mode`, and empty-is-absent) |

The new fixture `🐚️Shell/🧫️fixtures/📚️boot-example/🔣️.json` is answered by **two independent
implementations** — `resolve_boot_example_id` (Rust) and the shipped `resolveBootExampleId` (TypeScript,
validated by third-party **ajv**) — over one language-neutral corpus.

Commands and verbatim results:

```
RUST_MIN_STACK=33554432 CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-renderer-wgpu --lib \
  -- shell_chrome_parity_tests:: --test-threads=1 --nocapture
→ test result: ok. 20 passed; 0 failed; 518 filtered out
  [DEBUG] wgpu boot example: 8 shared fixture rows answered identically to React's resolveBootExampleId
  [DEBUG] wgpu boot example: the `?example=` query reaches the picker through sync_session_chrome and is
          announced between push_contributions and refresh_ui

NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:test-long -- -t "boot example contract"
→ Test Files 1 passed | 34 skipped; Tests 3 passed | 1078 skipped

bun -e 'testWgpuBootInputs(cwd, out)'   (the 🧊️wgpu-browser-boot-cache-inputs oracle, node:vm over the real
                                         bootDescriptor source)
→ [DEBUG] wgpu boot descriptor oracle: all fixture cases answered

RUST_MIN_STACK=33554432 CARGO_INCREMENTAL=0 cargo test -p semio-framework-ui-contract \
  --test catalogue_carrier_map -- --test-threads=1 --nocapture
→ test result: ok. 12 passed; 0 failed
  [DEBUG] catalogue-carrier-map refusal vocabulary: capacity="UiFixedMap is full at its 32-entry capacity"
          grant="UiFixedMap admission ran out of page grant at entry 2 of at most 32""

CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false CARGO_INCREMENTAL=0 cargo check \
  -p semio-framework-os-renderer-wgpu --lib
→ Finished in 2m 02s, 0 errors, 29 warnings (warnings are the proof the expansion actually ran)
```

---

## 6. What is still red, with its owner

### 6.1 Two boolean examples stall in the VIEWER with one kernel unit permanently in flight

`sphere-cut-with-torus` and `sphere-box-fuse` are the only two examples whose graph contains a boolean
(`brep.bool.cut`, `brep.bool.fuse`), and they are the only two that fail in the viewer. **Deterministic**, to
the digit, across four independent sessions and two different builds — 120 s and 240 s budgets, matrix-1
(pre-catalogue-fix) and matrix-2 (post):

```
sphere-box-fuse      phase meshingFaces  unitsDone 24/41  facesDone 7/7  inFlight 1  ratio 0.5853658536585366
sphere-cut-with-torus phase samplingEdges unitsDone 0/0   facesDone 0/0  inFlight 1  ratio 0.0
```

What is NOT the cause, each ruled out from the same consoles: no `[role=alert]`, no
`wgpu-shell surface fault`, no `Capacity`, no `render-budget-exhausted`, no
`intake-budget-exhausted`, no `invokeExtension faulted`, no cancel. `status.cancellable` stays `true`
forever — the guest still believes work is outstanding.

The discriminator is one trace. The **extension ladder is identical** in both roles — 5 invocations,
`req=1..3 evaluate`, `req=4..5 tessellate`, the last answered `effects=0`, all of them
`flow-extension-brep`:

```
edit   : … invokeExtension answered req=5 … tessellate effects=0   then  flowEvalTick settled effects=0   (6 ticks)
viewer : … invokeExtension answered req=5 … tessellate effects=0   then  ——nothing——                      (5 ticks)
```

The editor receives one final `flowEvalTick` that settles with `effects=0` — the terminal tick that reaps the
in-flight unit and publishes the finished mesh set (`meshesLen` 1089 on a converged surface vs **2** on a
stalled one). The viewer never issues it, so the unit stays `inFlight: 1` forever. The per-actor
typed-operation drain is NOT the discriminator: a PASSING viewer cell (`face-sweep-extrude`) reports the
identical `typed-operation drain polls=4 stopped=idle instance=1 pages=3 terminal=true`.

**Owner:** the viewer surface's eval chain — `📓️viewer-eval-chain-2026-09-12.md`'s subject, and the
`may_rearm` continuation gate the boot report in `wgpu-shell.rs`'s `settle_boot` comment names. Evidence to
start from: `T/🗑️generated/wgpu-examples/matrix-2/sphere-box-fuse/{edit,viewer}/console.txt`, diffed around
`invokeExtension answered req=5`.

### 6.2 `addGeneration`: published, hit-tested, pointer delivered — and not dispatched

Driven by `T/🐍️wgpu-add-generation-probe.mjs`, which derives the row's page rect from the two published
sources rather than guessing a pixel: the retained node's `mounted_layout` rect from `dumpStructure(windowId)`
offset by that window's rect in the shell's own `[DEBUG] wgpu-shell dock plan`.

The row IS published — this contradicts the older `wgpu-ui.surface-not-published` reading for generate mode:

```
windowId generation3d-generations
path     tree[0]/stack[1]#procedural3d-play-generate.actions/stack[0]#procedural3d-play-generate.add-generation
rect     [0, 72, 315.392, 24]   body 315×814+3,54   →   page (160.696, 138)
```

That page point is the one the Rust law already asserts
(`🖱️ui/🧪️tests/🎯️retained-hit-targets/🦀️.rs:324-325`: `(160.696, 138) -> addGeneration`), derived here
independently from the live browser.

The full delivery chain then reads green up to the last hop
(`T/🗑️generated/wgpu-examples/generate-2/hexagonal-mushroom-column/console.txt:2690-2717`; reproduced on the
current build in `T/🗑️generated/wgpu-examples/generate-3/` for `hexagonal-mushroom-column` AND
`box-shell-preview`, both `targets=1 delivered=1 dispatched=0`):

```
os_host handle_event PointerDown { … x: 160.696, y: 138.0, button: Primary } gen=569
os_host dispatch_normalized_event PointerDown { … x: 160.696, y: 138.0, button: Primary }
os_host pointer hit x=160.696 y=138 targets=36 hit=Some((TreeItem, Some("tree.label.procedural3d-play-generate.add-generation")))
<< no addGeneration dispatch, ever >>
```

The browser delivered the event (probe witness `delivered=1`), the host admitted it, `Ui::dispatch_event` ran,
and the hit registry resolved the CORRECT control id. The missing hop is `HitKind::TreeItem` + a control id →
the row's own `ActionDescriptor`.

**Owner: the `wgpu-input-hit-runtime` lane** — this lane was instructed not to touch input/hit/dispatch code
and did not. Everything upstream of that hop is now proven from a real browser, so that lane can start at the
dispatch site rather than at the beginning.

Because the click does not dispatch, **generate-mode convergence after `addGeneration` is NOT proven** — see
§8.

### 6.3 A note for whoever raises `UiFixedMap`

The catalogue lane fixed the ordering half (`📓️wgpu-catalogue-fixed-map-2026-09-13.md`), and that fix is what
turned `box-fillet-preview` and `sphere-cut-with-torus` from red to green in edit mode between matrix-1 and
matrix-2 (both had failed with
`renderDocument result parse failed: UiFixedMap … at line 1 column 20911`, thrown at
`🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:843`). Their producer,
`semio_framework_plugin::app::section_text_chunks`, packs the catalogue into `1 + UI_FIXED_LIST_ITEMS` slices —
**exactly** at the 32-entry capacity, with zero headroom. A catalogue one slice larger fails again, and the
failure will now be legible: this lane made the refusal name its cause
(`🖱️ui/🧬️contract/🎬️action/🦀️.rs:631` `UiFixedMap::refusal`, used by both `Deserialize` and `FromValue`), so
"full at its 32-entry capacity" and "ran out of page grant at entry N" are no longer the same sentence.

---

## 7. Files

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔗️boot-query/🟦️.ts` | `BOOT_QUERY_EXAMPLE_PARAM`, `resolveBootQueryExampleId` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts` | `defaults.exampleId` reads the query over the env seed |
| `…/🧊️wgpu/🚀️browser-boot/🟦️.ts` | `appExample` in `bootDescriptor` and in the boot message |
| `…/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` | `BrowserFrameWorkerBoot.appExample` |
| `…/🧊️wgpu/🎞️frame-worker/🟦️.ts` | `semioWgpuSetBootExample` binding + call |
| `…/🧊️wgpu/🧊️renderer/🦀️.rs` | region `📚️ExampleBoot`: thread-local, wasm hook, `boot_app_example()` |
| `wgpu-shell.rs` | `resolve_boot_example_id`, `apply_boot_example`, `sync_session_chrome` re-pointed, call site in `settle_boot` |
| `🐚️Shell/🧫️fixtures/📚️boot-example/🔣️.json` | **new** — 8 resolution rows + 4 `?example=` rows |
| `🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs` | 2 new laws |
| `🧪️tests/🔬️engine-contract/🟦️.ts` | **new** `describe("📚️ boot example contract")`, 3 `it`s |
| `🧫️fixtures/🧊️wgpu-browser-boot-cache-inputs/🔣️.json` | `appExample` on every case, 3 new cases |
| `🧪️tests/{📨️browser-frame-transport,⏱️wgpu-ui-turn-budget,🎮️wgpu-browser-input-wire}/🟦️.ts` | boot literals declare `appMode`/`appExample` (fix-forward: they already omitted the required `appMode` a peer added) |
| `🖱️ui/🧬️contract/🎬️action/🦀️.rs` | `UiFixedMap::refusal()` + both decoders use it |
| `🖱️ui/🧬️contract/🧪️tests/🛍️catalogue-carrier-map/🦀️.rs` | 1 new law |
| `T/🐍️wgpu-shard-liveness-probe.mjs` | **new** — blocked-vs-idle worker discriminator + CPU census |
| `T/🐍️wgpu-spin-stack-probe.mjs` | **new** — raw-CDP `Profiler` + `Debugger.pause` on a blocked worker |
| `T/🐍️wgpu-example-matrix-probe.mjs` | **new** — 8 examples × 3 boot lanes, four-clause convergence |
| `T/🐍️wgpu-add-generation-probe.mjs` | **new** — generate mode, dock-plan-derived row rect, pointer witness |
| `T/🗑️generated/wgpu-examples/` | every run, console and screenshot cited above |

`bridge.ts` carries **no** change: its temporary instrumentation was removed and the file is byte-identical to
its committed state. No git-state-modifying command was run; the ticket was not opened, closed or reopened;
`📓️status.md` and `🎫️ticket.json` were not touched; no server was started or stopped.

---

## 8. What is NOT claimed

1. **The boot-hang fix is not this lane's.** The reserve-then-close ladder in `flow-host.rs` is a peer's
   (`📓️fix-forward-set-contributions-hang-2026-09-13.md`). This lane named the cause from a profile and a
   stack, and restaged the guest so it reached the browser.
2. **Generate mode after `addGeneration` is unproven.** The click does not dispatch (§6.2), and there is no
   boot-query or introspection door to fire the verb another way — `semioWgpuIntrospection` is read-only by
   construction. A generate-mode preview was therefore never observed carrying a generation's mesh. §2.3's
   `0/8` is "no generation exists yet", not "the generate preview is broken".
3. **The two viewer stalls are diagnosed, not fixed.** §6.1 names the missing terminal `flowEvalTick` and the
   evidence; it does not change the viewer eval chain, which is another lane's.
4. **No claim about hover, selection, orbit or camera fit.** Not exercised by these probes.
5. **Time-to-mesh is a single measurement per cell** on a machine shared with several concurrent Rust builds,
   not a distribution. The wgpu tick is input-driven, so the probe nudges the pointer 1 px continuously; a
   measurement taken with a different drive cadence will differ.
6. **`?example=` is proven on the wgpu shell for all 8 ids and on React for one.** The React half is covered
   by the shared fixture and by one live boot (`box-fillet-preview` on 6018), not by a React sweep.
7. **The `UiFixedMap` refusal message was improved, not its capacity.** No cap was raised and no projection
   was paged.
