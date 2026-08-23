# Sol Read-Only Phase 5 Status and Gap Audit

Date: 2026-08-23  
Auditor: Sol High  
Verdict: **RED — Phase 5 is not source-complete or mounted end to end.**

## Scope and method

This audit is read-only with respect to production source, tests, and the permanent verifier. The only write is this report in the existing Phase 5 ticket. I read the Phase 5 plan and all reports in this folder, then traced the current UI-runtime, UI-WGPU, renderer-host, plugin-reactor, and platform-host sources. I ran only `rg`, scoped `rustfmt --check`, and the root interactivity verifier. I did not run Cargo, Nx, Wasm, a browser, a runtime, or network access.

The historical reports are implementation records, not current acceptance evidence. In particular, their Cargo and latency results were not rerun and are not carried forward as current results.

## Plan-to-source matrix

| Packet | Existing GREEN evidence | Current RED evidence | Status |
| --- | --- | --- | --- |
| P5a — seven-stage `FrameTransaction` | `FrameTransactionStage` contains the specified seven ordered stages. `step` receives `StepContext`, tracks an input epoch, checks cancellation/fuel, retains shadow reconcilers until publication, preserves a 256-delta limit, and has semantic tests for storms, stale input, cancellation, ordering, and credits. Production `UiRuntime::transact` is absent; the run-to-completion helper is `#[cfg(test)]`. | There are **zero production `FrameTransaction::new` callers** and zero production `UiRuntime::new` callers, so this transaction is not mounted. `submit_intent`, `request_wake`, surface registration, transaction queues, outputs, and commit queues are dynamic and lack pre-admission item/byte ownership. Defaults allow 262,144 items/nodes and 64 MiB only after owners have already been inserted. `step` contains a stage-fallthrough `loop`. `prepare_surfaces` drains/collects/sorts the complete dirty set in one opportunity. Effect notification/flush and application `Present::present` remain indivisible. `publish` drains every commit, scans/retains all wakes, and flushes presence in one call. supersede/fault/cancel/reset use `clear`, `take`, or `None` to bulk-drop candidate trees, cursors, patches, and shadow reconcilers. There is no public terminal take/resume/one-owner close authority. | **RED** |
| P5b — presentation/reconciliation cursor | `SurfaceReconcileCursor` exists and the dormant `FrameTransaction` calls it. Candidate reconciler state remains shadowed until completion. Tests cover parity, abandonment, cancellation, duplicate keys, and large-tree stepping. | The cursor is crate-private and contains phase-fallthrough/nested loops, dynamic `Vec`/`HashMap` growth, node/string/record clones, whole immediate-child collections, and whole patch-estimate copies without fixed item/byte admission or a terminal close cursor. More importantly, the live plugin reactor reaches `PatchTracker::diff`, which still calls production `SurfaceReconciler::reconcile` synchronously and run-to-completion. `PatchTracker::revision` calls `snapshot`, which clones the complete retained-node set merely to read the revision. `SurfaceReconciler::mark_rejected` clears retained maps wholesale. | **RED** |
| P5c — layout/text shaping job | `LayoutJob` and `Ui::step_layouts` exist. The job has retained traversal/text/result indices, checks `StepContext`, advances a selected window, and has large-tree, cancellation, and sub-8-ms historical tests. A weighted three-lane scheduler exists. | The mounted `Interpreter::render_ui_node` calls `Ui::{set_theme,apply_tree,set_viewport,frame}` but never `step_layouts`; repository-wide production caller count for `step_layouts` is **zero**. A dirty live tree therefore returns its previous draw list while its queued job is never driven by this route. `LayoutJob` has no fixed item/byte admission; it grows multiple `Vec`/`HashMap` owners. `step`, traversal, and text shaping contain loops/fallthrough. `collect_node` reserves the full preorder result length, `publish` mutates every result in one call, and cancellation in `Ui::step_layouts` sets `layout_job = None`, ordinarily dropping all retained work. Text measurement remains one opaque full-string call per node. | **RED** |
| P5d — tessellation/batching/upload preparation | The current OS renderer mounts `AppFrameBuild::into_preparation` -> `PreparedRenderJob` -> capacity-one retained packet -> `AppPresenter`. Revision/generation validation, last-valid preservation, cancel/stale rejection, raster 16 KiB pages, raster generation credits, presentation ACK/abort, and incremental packet retirement are present. The accepted raster table work is materially stronger than the original P5 report. | `PreparedRenderInput::new` accepts an already-materialized `DrawList` plus dynamic damage/clip/directive/upload/eviction vectors before aggregate admission. Generic `PreparedRenderJob::step` processes up to 64 items in a `while` per renderer grant. Default draw credits are 262,144 items/64 MiB and are measurement-after-ownership, not process admission. Live icon and glyph atlas paths clone complete pixel vectors before preparation, and `GpuContext::apply_prepared_upload_step` uploads each complete atlas in one call. `GpuContext::render_prepared` performs complete scene rendering, encoding, queue submissions, composition, and presentation as one indivisible UI-side operation. Separately, the public `ui_render::FrameEngine::build_frame` and `Scene::finish` still express compose -> layout -> prepaint -> paint -> validate/snap/order/batch/hash as one synchronous transaction, although repository census finds no non-test mounted `FrameEngine::build_frame` caller. | **RED**, with accepted raster sub-seam GREEN |
| P5e — multi-window/surface lanes and stress | `Ui` has deterministic weighted lanes and tests for resize coalescing/background fairness. The mounted winit host uses a fixed/coalescing event queue, event-driven redraw invalidation, one frame generation, and process-pool frame work. Metrics enqueue the newest logical size and do not run layout in the callback. | The lane scheduler has no production `step_layouts` caller, so the tested fairness contract is not mounted. Window maps/queues and window identifiers are dynamically allocated and uncapped. `set_theme` clones and scans all window IDs; lane changes retain-scan all three queues. `Ui::frame` clears/rebuilds a complete draw list and iterates every scene slot. Native metrics also call `presenter.resize` synchronously; `GpuContext::resize` reconfigures surface/depth resources as an opaque platform step. There is no mounted stress proof in this source-only audit for simultaneous effect, resize, multi-window invalidation, surface loss/recreation, and close. | **RED** |
| Phase 5 gate — no stage >=8 ms and responsive input | `StepContext`, UI event/present watchdogs, event-driven redraw, and historical focused tests exist. Root interactivity DENY currently passes. | The permanent verifier has no faithful P5a/P5b/P5c/P5e structural or mutation corpus. Its only `PreparedRender` coverage is the later paged-raster producer predicate. The live synchronous reconcile, whole-atlas copies/uploads, full GPU submission, and immediate resize paths are not discriminated by the current gate. No current runtime latency test was permitted or run. | **RED** |

## Exact caller and reachability census

### P5a UI runtime transaction

- Definition: `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️transaction.rs` defines `UiRuntime` and `FrameTransaction`.
- Production constructors: **0** `FrameTransaction::new` calls and **0** `UiRuntime::new` calls outside that file's `#[cfg(test)]` region.
- Test-only drivers: the private `UiRuntime::transact` helper and transaction tests instantiate and loop the retained API.
- Reachability verdict: the seven-stage transaction is currently an unmounted library primitive, not the OS/product frame path.

### P5b reconciler

- Retained cursor caller: `FrameTransaction::reconcile_tree` is its only production-shaped caller, but P5a itself is unmounted.
- Live run-to-completion caller: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️component.rs::PatchTracker::diff` calls `SurfaceReconciler::reconcile`.
- Live root: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs` calls `PATCHES.diff` from the production reactor poll after `plugin_render`. This is plugin/product reachable and not test-only.
- The same live tracker reads revision through a whole `SurfaceReconciler::snapshot` clone.

### P5c layout and text

- Definition: `Ui::step_layouts` in UI-WGPU `🦀️engine.rs`; retained work in `🦀️flex.rs::LayoutJob`.
- Production callers of `Ui::step_layouts`: **0**. The three calls found in `🦀️engine.rs` are under `#[cfg(test)]`.
- Live retained-engine root: `Interpreter::render_ui_node` calls `Ui::apply_tree`, `Ui::set_viewport`, and `Ui::frame` once per render.
- Live product callers of `render_ui_node`: two Shell sites, for active-tab content and general window content.
- Reachability verdict: retained trees are mounted, but their queued layout authority is not.

### P5d prepared renderer and separate render family

- Live prepared constructor: `AppFrameBuild::into_preparation` creates `PreparedRenderJob`; `AppFramePreparation::drive_step` drives it through the frame worker; `AppPresenter::present_step` consumes the result.
- Live build root: `OsHost::redraw_core` -> `build_and_publish_snapshot` -> `FrameBuildHandle::poll_runtime_and_resubmit` -> `AppFrameTransaction` -> `AppFrameBuild` -> `AppFramePreparation` -> `AppPresenter`.
- One active frame preparation and one active presentation are coalesced by the frame/presenter authorities; stale generation checks exist.
- `PreparedRenderInput::new` also appears in tests and adjacent product fixtures; the mounted renderer construction above is the relevant production route.
- `ui_render::FrameEngine::build_frame` has no non-test repository caller. Its synchronous `Scene::finish` route remains public dormant debt rather than evidence that the mounted OS renderer is resumable.

### P5e platform and surface roots

- Native resize: winit `WindowEvent::{Resized,ScaleFactorChanged}` -> `OsHost::handle_metrics` -> coalesced metrics event + immediate `AppPresenter::resize`/`GpuContext::resize` + keyed runtime resize message.
- Native redraw: `WindowEvent::RedrawRequested` -> `OsHost::redraw` -> nonblocking frame mailbox/presenter stepping.
- Native wake: worker completion invalidates `RESOURCE_READY`; `about_to_wait` requests a redraw only for a pending invalidation/deadline.
- UI-WGPU surface lanes: definitions and tests only; zero mounted layout-step caller.

## Ownership, cap, and terminal summary

| Area | Item/byte admission | Generation/freshness | Per-grant unit | Terminal ownership |
| --- | --- | --- | --- | --- |
| `FrameTransaction` | Post-hoc usage only; dynamic ingress/queues; 262,144 items/nodes and 64 MiB defaults | Input epoch and revision guard | Intended fuel unit, but stage loop and bulk transitions remain | Missing public retained close/take/resume; bulk clears/drops |
| `SurfaceReconcileCursor` | None; dynamic vectors/maps/clones | Base revision assertion only | Intended node phase, but nested/fallthrough loops and whole local child work remain | Dropped wholesale on supersede/cancel; no public terminal cursor |
| `LayoutJob` | None; dynamic vectors/maps | Window revision invalidates by replacing job | Mostly one node/glyph, but loops, whole publish, and opaque string measurement remain | `layout_job = None` drops retained job; no terminal authority |
| Prepared render generic packet | Measurement caps after input materialization; 262,144/64 MiB draw and 256/32 MiB upload defaults | Scene revision + preview generation + raster witness | Up to 64 measured items per worker grant; whole atlas upload per present step | Packet/gate/raster close paths exist; generic input construction and atlas ownership remain incomplete |
| Prepared raster sub-seam | Fixed 256 generations, 4,096 items, 32 MiB aggregate, <=16 KiB pages | Generation-tagged credit and upload witness | One producer page and accepted upload cursor step | Public rejection/close and incremental retirement present |
| Multi-window lanes | No fixed window/key/queue byte admission | Window revision and latest resize state | One selected layout job call when explicitly driven | No mounted host close proof for queued layouts/windows |

## Test and verifier audit

Existing semantic Rust tests are meaningful for their local primitives: frame input/effect storms, deterministic surfaces, reconcile parity/abandonment, large layout stepping, weighted-lane fairness, prepared packet stale/cancel/credit behavior, raster cap/+1/ABA, and packet/gate retirement. They do not establish mounted reachability, pre-admission ownership, no-bulk-drop terminal behavior, or the absence of the live reactor's synchronous reconciliation.

The root `📜️script.ts` has permanent checks for the later prepared raster page producer, including 16 KiB pages, caps, direct paged upload consumption, stale generation, and close. It has no current predicate or faithful mutations for:

- mounting `FrameTransaction`;
- rejecting dynamic P5a ingress or bulk phase/publish/close paths;
- denying production `SurfaceReconciler::reconcile`;
- requiring the live renderer to call `Ui::step_layouts`;
- layout fixed admission and retained cancellation/close;
- whole icon/glyph atlas clones and uploads;
- generic preparation's 64-item loop;
- full `render_prepared` submission or immediate resize stress.

## Permitted gate results

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` on UI-runtime `transaction.rs` and `reconcile.rs` | PASS |
| `rustfmt --edition 2021 --check` on UI-WGPU `flex.rs`, `engine.rs`, `prepared.rs`, and `gpu.rs` | PASS |
| `rustfmt --edition 2021 --check` on renderer `Interpreter/component.rs` and `winit_app.rs` | PASS |
| `rustfmt --edition 2021 --check` on renderer `📦️glue.rs` | PARSE PASS / FORMAT RED: current shared test code differs from rustfmt around the job-progress identity closure; no rewrite was made |
| `bun 📜️script.ts verify interactivity --self-test` | PASS, DENY clean; one reported allowlisted blocking bridge; self-test completed |
| `bun 📜️script.ts verify interactivity` | PASS, DENY clean; same one reported allowlisted blocking bridge |

No runtime behavior or latency is claimed from these source-only gates.

## Smallest next file-disjoint source packet

### P5b-live-reactor-reconcile-cutover

This is the smallest cohesive packet because it removes an actually mounted plugin/product whole-tree operation without colliding with current raster, renderer presentation, P4 fill, collision, or preview work.

Owned files should be limited to:

- UI-runtime `🦀️reconcile.rs` and its existing exports;
- plugin reactor `🩹️patches/🦀️component.rs`;
- the narrow reactor poll cursor state in `⚛️reactor/🦀️component.rs` only if scheduling cannot stay inside `PatchTracker`;
- the existing root `📜️script.ts` interactivity region and this Phase 5 report.

Required boundary:

1. Remove production reachability of `SurfaceReconciler::reconcile`; keep any direct oracle strictly `#[cfg(test)]`.
2. Expose an owned generation-tagged reconcile authority with fixed operation/node/key/string/patch item and byte caps admitted before moving the tree.
3. Make one reactor grant advance at most one node, child identifier, semantic field, patch operation, or close owner. Remove nested/fallthrough loops and whole child/patch copies from the live cursor.
4. Replace `PatchTracker::diff` with begin/step/take-result/take-rejected/resume/one-owner-close semantics. Keep stable FIFO surface ordering and revision/ACK/rejection rules.
5. Add an O(1) revision accessor; no whole `snapshot` clone to read a scalar.
6. Preserve exact input tree/reconciler/patch ownership on saturation, stale generation, duplicate keys, panic/fault, ACK rejection, cancel, and close; terminal-empty only after all owners and admission credit are returned.
7. Add cap/+1 items and nested bytes, large/wide/deep trees, stale/ABA, quiet wake, cancel at every cursor phase, exact handback/pointer identity, interrupted close, and last-valid revision fixtures.
8. Add faithful verifier mutations denying any production `.reconcile(tree)`, whole `snapshot()` revision read, phase loop/drain, dynamic post-admission growth, bulk clear/drop, and missing terminal retrieval.

This packet does not make P5a, P5c, P5d, or P5e GREEN. After independent acceptance, the next dependency is mounting the existing layout authority in the live renderer worker cursor; that should be a separate renderer/Interpreter packet because it has different owners and collision risk.

## Final recommendation

Do not accept or close Phase 5. Treat the historical P5 reports as useful primitive-level implementation evidence only. Execute and independently audit `P5b-live-reactor-reconcile-cutover` first, then separately mount and harden P5c/P5e, and finally close generic draw/tessellation/atlas/GPU submission ownership for P5d. The current root interactivity gate being clean is necessary but not discriminating enough to override the live source findings above.
