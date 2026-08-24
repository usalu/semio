# P5d Mounted Prepared-render Worker Repair — 2026-08-24

## Status

**Source-audit-ready; independent acceptance and executable gates remain open.** The mounted OS
renderer now transfers a generation-qualified prepared input into a retained `PreparedRenderJob`,
drives it through the shared `BatchJobSession` with one fuel unit and a one-millisecond step budget,
publishes through a fixed capacity-one mailbox, and presents through a retained GPU cursor. Input,
job, receiver, packet, GPU, and process-credit interruption paths publish the exact owner into fixed
abandonment authority and the mounted P5a frame boundary advances each authority incrementally.

No Cargo, Nx, Wasm, browser, native runtime, or timing gate was run. Those gates were explicitly
excluded while shared source packets and peer build processes overlap.

## Governing Route and Prerequisites

The repaired production route is:

`OsHost::redraw_core` -> `AppFrameTransaction` -> `AppFrameBuild::into_preparation` ->
`PreparedRenderJob::try_new` -> `BatchJobSession<PreparedRenderJob>` ->
`PreparedRenderReceiver` -> `AppPresenter::present_step` -> `PreparedGpuPresentCursor` ->
matching presenter acknowledgement.

The implementation retains the independently accepted P5a frame transaction, P5b exact-owner live
reconciliation, P5c mounted layout/text, P2a1 worker session, and paged-raster protocol. The dormant
`FrameEngine::build_frame` and `Scene::finish` complete builders remain available only to tests.

## Changed-file Inventory

| File | Exact P5d work |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs` | Added fixed paged metadata and command owners, cumulative generation-qualified process permits, fallible input/job/mailbox/packet admission, retained preparation cursors, capacity-one publication, incremental retirement, fixed abandonment registries, Drop handback, and hostile ownership laws. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️draw.rs` | Added cumulative retained draw claims, physical nested/backing retirement, and one-scalar GPU encoders for UI, raster, vector triangle, world instance, world line, blur mip, glass region, and final scene blit. Complete batch render helpers are test-only. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️gpu.rs` | Replaced whole prepared rendering with a generation-qualified retained present cursor covering target, clear, one command, surface/view, one blur mip, composite, one glass command, present, and close; mounted exact cursor abandonment recovery and a two-millisecond post-call guard. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` | Cut the mounted build/preparation/presenter route over to fallible prepared input binding, `PreparedRenderJob::try_new`, the shared worker session at fuel `1`/budget `1 ms`, atomic gate staging, retained GPU stepping/close, and the five P5d abandonment drain phases. |
| `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️frame.rs` | Restricted the unmounted synchronous `build_frame` alternative to `cfg(test)`. |
| `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️scene.rs` | Restricted the unmounted synchronous `finish` alternative to `cfg(test)`. |
| `📜️script.ts` | Added the isolated P5d live-source verifier and 39 faithful mutations; updated the P5a preservation predicate to require every mounted P5d abandonment phase and exact incremental close call. |

All files already contained accepted P5a/P5b/P5c or peer work. The changes above preserve those
regions and do not modify unrelated P1/P6, stdio, oracle, or Puzzle Fill ownership.

## Fixed Admission and One-opportunity Laws

| Authority | Fixed/cumulative limit | One admitted opportunity |
|---|---:|---|
| Prepared metadata | 256 owners, 32 owners per independently allocated page | Transfer or retire one owner; release one empty page backing separately. |
| Prepared commands | 4,096 pages, 64 commands per page, 16 fixed directories | Admit, measure, hash, publish, or retire one scalar/page/backing owner. MAX + 1 returns the same command. |
| Process permits | 64 generation-qualified slots, 16,383 pages, bounded backing-unit ledger | Reserve before transfer/allocation, grow through checked CAS, release backing/pages/item/slot in separate close steps. |
| Prepared mailbox | 64 fixed generation-qualified slots, one packet per slot | Publish or take one exact packet without a blocking or dynamic queue. |
| Worker turn | `fuel_per_step: 1`, `step_budget_ms: 1` | One raster page/producer backing/scalar census or preparation unit, with cancel/yield/generation checks before and after bounded work. |
| GPU present cursor | 64 fixed generation-qualified abandonment slots | One target/clear/command/platform/view/blur-mip/composite/glass/present opportunity, then a two-millisecond watchdog check. |
| Draw retirement | Existing populated draw graph plus cumulative item/byte claim | Release one key byte, primitive, nested owner, string backing, vector backing, or outer backing per close call. |

The preparation cursor covers draw, overlay, upload, eviction, damage, clip, directive, validate,
snap, order, tessellate, batch, hash, packet sealing, and publication sections without a production
loop. `DrawMeasureCursor` retains layer/pass/item/instance/key-byte/line/textured/glass positions.
Tessellation commands carry the exact draw cursor and overlay owner into the immutable packet.

The GPU consumer emits one actual UI/raster instance, one vector triangle, one mesh instance, or one
line segment from a command cursor. Blur and glass have distinct retained mip/region cursors. The
existing renderer has no production textured-scene pipeline; textured owners remain measured,
hashed, ordered, and cursor-retained without inventing a compatibility renderer.

## Exact Ownership and Close Table

| Owner | Refusal/stale/cancel/fault/panic path | Incremental close and terminal witness |
|---|---|---|
| `PreparedRenderInput` | `try_new` and `try_bind_draw` return `PreparedRenderInputRejected` with the exact draw/overlay/permit owners. | Upload, producer, eviction, draw, overlay, metadata backing, permit dimensions, and abandonment slot close independently. |
| `PreparedRenderJob` | Mailbox or abandonment refusal returns `PreparedRenderJobRejected`; stale generation, cancel, credit fault, publication refusal, and panic retain input/commands/rejected packet. | `InteractiveJob::begin_close`, `close_step(1, JOB_PAYLOAD_PAGE_BYTES)`, and `terminal_is_empty` gate session retirement. Drop publishes the populated job to its fixed slot. |
| `PreparedRenderReceiver` | Reservation and publication are generation-qualified CAS operations; occupied capacity returns the packet unchanged. | Packet, references, state, and slot generation are retired without a blocking queue; interrupted ownership rejoins the mounted receiver drain. |
| `PreparedRenderPacket` | Gate validation rejects stale revision/generation, credits, pending presentation, or closing before GPU mutation. | Nested uploads/evictions/draw/overlay/commands/backings, permit dimensions, and abandonment slot close one unit at a time. Drop publishes the exact packet owner. |
| `PreparedGpuPresentCursor` | Generation/capacity refuse before surface ownership; stale and platform faults abort the candidate while the last-valid logical packet remains gated. | Surface, view, cursor scalars, and fixed slot close through `begin_close`/`close_step`; Drop hands an interrupted cursor to the mounted GPU drain. |
| Presenter/gate | A candidate stages only after revision/generation validation; ACK follows completed GPU close. Abort/device or surface fault cannot replace last-valid state. | `AppPresentPhase::CloseGpu` finishes the cursor before acknowledgement and retained packet retirement. |
| Mounted abandonment pump | P5a runs GPU -> input -> job -> receiver -> packet abandonment phases before new frame text work. | Every `close_abandoned_step` advances at most one exact retained owner/backing/permit action and parks the frame while more close work remains. |

## Hostile Laws and Verifier

Local Rust laws cover:

- exact input Drop process-permit handback and incremental close;
- worker panic handback of the exact job and mailbox owners;
- packet Drop retirement of nested backing and permit dimensions separately;
- command-page MAX + 1 rejection without consuming the command;
- exact tessellation scalar and overlay cursors;
- stale-generation refusal before publication and cancellation without packet replacement;
- interrupted GPU cursor handback; and
- GPU generation exhaustion/capacity refusal before ownership.

The permanent `verify interactivity p5d` verifier reads the six production files, strips test-only
items, and rejects 39 direct mutations. The mutations cover blocking/wrapping process ledgers,
missing permit or Drop recovery, dynamic metadata/commands, identity-losing refusal, cursorless
tessellation, bulk worker fuel/loops, stale publication, dynamic capacity staging, missing hostile
laws, bulk GPU command/glass/blur traversal, missing watchdog, whole GPU rendering, dynamic scalar
batches, unmounted abandonment drains, wide caller budget, dormant whole builders, and missing GPU
interruption law.

## Exact Residual Census

| Census | Result |
|---|---:|
| P5d verifier production files | **6** |
| P5d live-source residual failures | **0** |
| Faithful P5d mutations rejected | **39 / 39** |
| Production `GpuContext::render_prepared` / `finish_prepared` | **0** |
| Production complete `render_scene_content` / `composite_to_swapchain` / `run_blur_chain` | **0**; definitions are `cfg(test)` only |
| Production `FrameEngine::build_frame` / `Scene::finish` | **0**; definitions are `cfg(test)` only |
| `WorkerPool::new` / `thread::spawn` / run-to-completion loop inside the prepared job boundary | **0** |
| Dynamic metadata or command working-set owner in the P5d fixed regions | **0** |
| Mounted P5d abandonment authorities | **5 / 5** |

## Validation

| Gate | Result |
|---|---|
| `bun ./📜️script.ts verify interactivity p5d` | **PASS** — live source and all 39 hostile mutations clean |
| Isolated `interactivityMountedFrameTransactionSelfTests(process.cwd())` | **PASS** after updating the stale preservation predicate for the five live P5d abandonment phases |
| Isolated `interactivityLiveReconcileSelfTests(process.cwd())` | **PASS** |
| Isolated `interactivityMountedLayoutTextSelfTests(process.cwd())` | **PASS** |
| `rustfmt --edition 2021 --config skip_children=true --check` on all six P5d Rust files | **PASS** |
| Scoped `git diff --check` on all six Rust files plus `📜️script.ts` | **PASS** |
| Direct P5d residual invocation | **PASS** — `{ files: 6, residuals: 0, failures: [] }` |

The aggregate `bun ./📜️script.ts verify interactivity p5a` command remains **RED outside P5d** at
the concurrently edited Puzzle Fill baseline:

`Puzzle FillBuilder still materializes a whole preview/result envelope inside one worker grant`

That aggregate failure occurs before the P5 preservation checks. It is recorded here as unrelated
evidence and was not modified or masked. The isolated P5a, P5b, P5c, and P5d gates above are green.

## Deferred Gates

After source overlap is quiescent, run the repository-owned Cargo/Nx/Bun compile and test entries,
native and Wasm-shaped renderer suites, and the 1/2/4/default-worker storm/timing matrix. Those gates
must establish actual worker turns below 8 ms, UI/platform callbacks at or below 2 ms p99, packet
determinism, panic/device/surface interruption behavior, and last-valid atomicity. This source report
does not claim executable or runtime acceptance before those deferred gates run.
