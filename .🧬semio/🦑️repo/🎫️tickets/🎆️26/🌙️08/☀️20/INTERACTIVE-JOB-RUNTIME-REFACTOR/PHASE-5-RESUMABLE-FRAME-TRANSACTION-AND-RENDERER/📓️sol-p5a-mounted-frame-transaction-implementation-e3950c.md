# P5a Mounted Frame-transaction Implementation — 2026-08-24

## Status

**Source-audit-ready.** The live OS renderer now constructs exactly one production
`FrameTransaction` inside its retained `FrameBuildHandle` worker session. The former headless
transaction family is test-only, so production cannot select an unmounted alternate authority.
Native and browser-worker-shaped execution both submit one retained opportunity to the shared
process `WorkerPool` on `Lane::Interactive`; neither route drives the transaction on its caller.

No Cargo, Nx, Wasm, browser, or broad runtime gate was run because the coordinator explicitly
reserved those gates until overlapping source packets finish.

## Changed Files

| File | Exact P5a change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` | Consolidated the mounted authority under the exact `FrameTransaction` name; added the seven monotonic stages, fixed FIFO action ownership, effect-storm credit, retained cursors, operation/generation/base-witness/deadline/cancel guards, checked raster admission, terminal-empty witness, generation-carrying build/preparation/presentation owners, and exact MAX + 1 action identity law. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️frame_job.rs` | Mounted `FrameTransaction::new` through the retained worker session, made native and browser-worker routes use `try_submit_step` on the shared pool, retained exact rejected/outcome/terminal owners, and blocked phase retirement until the transaction or prepared child is terminal-empty. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️winit_app.rs` | Replaced wrapping frame generations with permanent checked exhaustion, kept UI callbacks enqueue/poll/present-only, revalidated the completed generation, and moved the one atomic snapshot publication behind accepted presentation completion. Added mounted pointer/resize p99 and generation-exhaustion laws. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️render_snapshot.rs` | Made publication revisions checked and permanently exhausted, preserving the last-valid snapshot; added the exact exhaustion law. |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📦️glue.rs` | Restricted the dormant headless transaction module and export to `cfg(test)` while preserving its legacy oracle tests. |
| `📜️script.ts` | Added the permanent P5a live-source verifier and 44 faithful structural mutations, including the mounted Build/Finish callee graph, every live Shell/document/node child authority, and fixed atlas page allocation/upload/close; registered it in `verify interactivity`. Existing peer edits in this file were preserved. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` | Replaced complete chrome construction with a retained child cursor; frame cleanup releases one find/tooltip/element/widget owner per opportunity and each main/panel/navbar/footer/overlay child has one distinct grant. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs` | Replaced dynamic packet staging and whole `take_packets` with 256 fixed slots, identity-preserving rejection, one-packet extraction, and terminal-empty witness. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs` | Replaced dynamic World3D frame uploads, evictions, and request sets with fixed slots and a fallible one-owner `append_step`, including retained rejected-owner close. |

The renderer glue and root script already contained staged peer changes. This packet preserved the
accepted P5b/P5c core contracts and changed only their mounted external Shell/EngineCanvas/World3D
producer boundaries. It did not alter P1/P6 regions, plugin stdio/oracle work, or renderer Packet B
implementation files.

## Mounted Seven-stage Authority

| Stage | Retained mounted unit | Transition/freshness law |
|---|---|---|
| `DrainProjectionDeltas` | One scene-camera dispatch cursor opportunity or one fixed admitted action | Begins with the retained operation and generation; no stage loop or fallthrough. |
| `RouteIntents` | One bounded input action, scene interaction step, or fixed deferred owner | FIFO action admission occurs before transfer; MAX + 1 returns the same owner. |
| `FlushEffects` | One board/world retained authority opportunity | `EFFECT_STORM_BUDGET == 64`; checked opportunities fault without wrapping. |
| `PresentSurface` | One retained wheel/surface traversal opportunity | Surface and controller identifiers are byte-credited before ownership moves. |
| `ReconcileTree` | One retained raster-producer checkout/bind opportunity | P5b/P5c child owners stay generation-qualified; checked item credit precedes packet insertion. |
| `BuildRenderPackets` | One accepted P5c prepared-render child opportunity | Parent completion is blocked until the prepared child closes to terminal-empty. |
| `PublishSnapshot` | One generation/revision revalidation and one `RenderSnapshotSink::publish` swap | A stale, cancelled, faulted, or exhausted candidate cannot replace the last-valid snapshot. |

Every mounted transaction opportunity sets the exact stage and validates operation, generation,
input generation, base scene revision, cancellation, and deadline before domain work. The retained
frame generation is carried through build, preparation, presentation, and host publication so the
final host comparison cannot accidentally stamp a newer generation onto an older frame.

## Ownership and Close Laws

| Owner | Refusal/terminal path | One-grant close |
|---|---|---|
| `FrameActionOwners` | `try_push` returns `Err(ActionDescriptor)` without transfer | `pop_front` releases one action. |
| Worker session admission | `WorkerJobSession::try_new` yields an exact rejected session | `close_step(1, JOB_PAYLOAD_PAGE_BYTES)` advances one retained item/page. |
| Submitted worker opportunity | Pool saturation/contended takes the rejected owner and calls `resume`; shutdown/poison begins close | Outcome is taken by ticket; only `Yield` resumes. |
| Transaction terminal | Cancel, stale generation/revision, deadline, effect storm, raster fault, and phase fault converge on retained phase close | `FrameTransaction::close_step` and `terminal_is_empty` gate parent retirement. |
| P5c prepared child | Rejected/session/job owners remain explicit | Parent uses `close_step` and refuses completion until child `terminal_is_empty`. |
| Completed presentation | Carries its exact build generation through terminal checkout | Stale completion is not published; the presentation retirement cursor closes retained packet owners incrementally. |
| Snapshot revision | `next_revision() -> Option<u64>` permanently refuses `u64::MAX` exhaustion | Existing `Arc<RenderSnapshot>` remains the last-valid accepted snapshot. |

The mounted transaction region has no `VecDeque`, dynamic action iterator, stage `loop`/`while`,
`wrapping_add`, `saturating_add`, bulk `clear`, `unwrap`, or `expect`. Engine packet and World3D
producer owners now cross fixed/fallible P5a boundaries one at a time; accepted P5c prepared-render
owners still close through the child retirement cursor.

## Laws and Hostile Mutations

The permanent verifier rejects 44 mutations covering:

1. zero production mounted constructor;
2. second worker runtime;
3. caller-driven browser step;
4. dynamic action storage;
5. post-allocation/saturating raster credit;
6. wrapping frame generation;
7. stage fallthrough loop;
8. missing base-revision/input freshness;
9. missing transaction terminal witness;
10. missing terminal-empty-gated close;
11. missing rejected-owner take;
12. missing resume;
13. missing terminal take;
14. stale partial publication;
15. wrapping snapshot revision;
16. removed effect-storm budget;
17. removed MAX + 1 owner identity proof;
18. removed mounted input-storm p99 law;
19. removed last-valid exhaustion law;
20. restored production reachability of the dormant headless transaction;
21. restoration of the opaque pre-input callee;
22. bulk draw clear in place of retained retirement;
23. complete icon-pixel clone in place of page copying;
24. immediate deferred-work drive after frame completion;
25. restoration of complete `render_chrome`;
26. looped introduction persistence;
27. two chrome children in one grant;
28. dynamic EngineCanvas packet staging;
29. whole EngineCanvas packet take;
30. dynamic World3D upload staging;
31. whole World3D append;
32. whole atlas copy in place of the retained page cursor;
33. deferred work driven to completion in one opportunity;
34. dynamic atlas page slots;
35. atlas backing allocated before process-credit transfer;
36. bulk atlas page close;
37. two atlas pages uploaded in one GPU opportunity;
38. restoration of the complete document frame;
39. a dynamic retained-paint traversal stack;
40. recursive child painting from the one-node paint authority;
41. whole scene collection from the one-node scene authority;
42. restoration of the complete context-menu renderer;
43. restoration of the complete tour renderer;
44. cloned dynamic cleanup keys.

Local Rust laws additionally cover fixed action MAX + 1 pointer identity, one-action close,
generation exhaustion, pointer/input storms, resize storms, atomic last-valid preservation, stale
generation refusal, and retained session/preparation cancellation. Existing P5b/P5c replay,
atomicity, deep/wide, child-live, and per-worker-count laws remain unchanged.

## Coordinator Pre-acceptance Remediation

The coordinator counterexample was valid: the outer seven-stage transaction previously hid complete
candidate construction and finish inside two calls. The mounted call graph now exposes both
boundaries as transaction-owned cursors. Every opportunity is freshness/deadline/cancel checked by
`FrameTransaction::step` before it reaches one child, owner, packet, upload, or 16 KiB atlas page.

| Former opaque work | Retained remediation | Refusal and close law |
|---|---|---|
| `draw.clear` / `overlay.clear` | O(1) owner checkout followed by repeated `DrawList::retire_step`; candidate draw and overlay transfer in distinct Finish phases | Previous and candidate lists remain cursor-owned until each nested draw owner is retired or transferred. |
| Complete icon/glyph pixel clone | `PreparedAtlasPages` admits at most 2,048 independently allocated 16 KiB pages under a 64 MiB process ledger; one `push_page` copies one page per grant | Process byte/page credit is reserved before slot or page allocation; `close_step` releases one page backing and then the exact ledger credit. |
| Complete `render_chrome` | `ShellChromeFrameCursor` has explicit setup, main, left, right, navbar, tutorial, footer, overlay, drag, gesture, error, and persistence phases | Dynamic previous-frame collections release one exact entry per setup grant; each render child has exactly one call site and one opportunity. |
| `take_packets` | `EngineCanvasBuildContext` and mounted `FrameEnginePackets` each own 256 fixed optional slots; one packet or the exact rejected packet crosses `take_packet_step`/`try_push` | Rejected and accepted packets close through `EngineCanvasPacket::close_step`; terminal requires every producer and collector slot empty. |
| World `append_to` | `World3dBuildContext` owns fixed upload, eviction, mesh-request, and raster-request slots; `append_step` moves one owner/request | Admission checks prepared-input credit before move; rejected upload/eviction remains explicit and closes incrementally. |
| Immediate deferred-frame drive | Finish only retains a `FrameDeferredCursor`; new frame input is routed before the old deferred cursor receives one shared-pool submission opportunity | No Finish call executes deferred product/plugin work inline and no pending owner is overwritten. |

The strengthened verifier reads the production bodies of renderer glue, Shell, EngineCanvas, and
World3D. It rejects restored opaque calls, dynamic resource fields, bulk transfers/clears, complete
atlas clones, duplicate chrome-child calls, immediate deferred driving, and deferred run-to-
completion mutations. The exact named counterexample residuals are all zero.

## Coordinator Second Pre-acceptance Remediation

The second coordinator counterexample was also valid: the first callee-graph repair still admitted
complete Shell subtrees behind named child calls and reserved full icon/glyph `Vec` capacity before
copying bounded slices. The live mounted path now retains progress through the Shell child, UI
document, retained paint walk, scene-node, atlas page, and GPU upload authorities. A worker grant can
advance one phase scalar, one independently bounded widget/node, one owner, or one 16 KiB page.

### Exact Second-remediation Inventory

| File | Exact second-remediation change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` | Replaced icon/glyph candidate backings with retained `PreparedAtlasPages`; Build and Finish copy one page, transfer one paged upload, and close one page per checked opportunity. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` | Added the retained `ShellChromeChildCursor`; parked main, panel, navbar, tutorial, footer, overlay, context-menu, tooltip, dialog, and tour phases on node/widget subphases; replaced cloned cleanup keys with exact entry extraction. Complete renderer oracles remain `cfg(test)` only. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🧊️component.rs` | Added `UiDocumentFrameCursor` and one-step ingress, viewport, mounted-layout, retained-paint, and terminal phases; the mounted caller no longer reaches complete `Ui::frame`, complete composite, or pointer dispatch. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs` | Added fixed atlas page storage, page/item/process-byte admission, one-page population, exact page metadata, incremental close, terminal witness, MAX + 1 rejection law, and one-page/ledger-close law. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️gpu.rs` | Added a generation/upload/page-qualified atlas cursor and one-page glyph/icon upload opportunity. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️draw.rs` | Added origin/row-count-qualified glyph and icon page upload calls. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️engine.rs` | Added a fixed 64-depth retained DFS cursor and atomic candidate phases; `frame_into_step` contributes one node/scalar/scene/publication unit to its caller-owned unpublished frame. Added tree-order and depth MAX + 1 laws. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️paint.rs` | Split shallow node synchronization/paint from the legacy recursive test oracle so the mounted paint visit cannot recurse into a child. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️scene_slots.rs` | Added a one-node scene-slot authority, leaving whole scene traversal outside the mounted call graph. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` | Exported the retained UI frame step without reintroducing an adapter or second authority. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs` | Extended exact rejected/retiring upload ownership to paged glyph/icon atlas variants. |
| `📜️script.ts` | Deepened the P5a verifier to 15 production sources and 44 mutations, including every live Shell/document/paint/scene/atlas child boundary and backing-credit/close order. |

### Retained Child and Page Laws

| Authority | One-grant unit | Close/terminal law |
|---|---|---|
| Shell chrome | One phase scalar or independently bounded child/widget; the outer phase remains parked until the child cursor completes | Dynamic previous-frame map/list owners are extracted one at a time; no cloned cleanup key is created. |
| UI document | One ingress page/validation scalar, viewport scalar, mounted-layout opportunity, retained node, or publication scalar | Fault remains explicit; no complete `render_ui_document`, `Ui::frame`, composite, or recursive paint call is reachable. |
| Retained UI paint | One fixed-stack visit, one shallow node sync/paint, one scene-node lookup, or one publish swap | Depth credit is 64; MAX + 1 faults without dynamic spill; previous draw retires one scalar before replacement. |
| Atlas candidate | One independently allocated 16 KiB page copied from the retained source | Page and process byte caps are checked before allocation; close drops one page, then releases its exact ledger reservation, then witnesses terminal empty. |
| GPU atlas upload | One generation-qualified glyph/icon page | Cursor advances with checked addition and clears only after the final retained page. |

The direct Rust laws exercise page-cap MAX + 1 refusal, page identity/geometry, one-page close followed
by exact credit release, retained DFS tree order, and depth-cap MAX + 1 refusal without a dynamic
spill. They were parsed and formatted but not executed because Cargo was explicitly deferred.

## Exact Residual Census

| Census | Result |
|---|---:|
| Production `FrameTransaction::new` outside the headless test oracle | **1** (`frame_job.rs`) |
| Production `UiRuntime::new` | **0** |
| Caller-driven `try_step_on_caller` in mounted native/browser job | **0** |
| `WorkerPool::new` or `thread::spawn` in mounted frame job | **0** |
| Dynamic/panicking/recursive forbidden forms in mounted `FrameTransaction` region | **0** |
| `frame_generation.wrapping_add` in mounted host/job | **0** |
| `snapshot_sink.publish` mounted sites | **1**, generation-matching completion branch only |
| Production headless `mod transaction`/export | **0** (`cfg(test)` oracle only) |
| Exact stage variants and labels | **7 / 7** |
| Production complete Shell child calls in the mounted child region | **0** |
| Production complete document/frame/composite calls in the mounted document region | **0** |
| Production recursive paint/whole scene collection in the mounted node regions | **0** |
| Full-capacity icon/glyph atlas candidate `Vec` allocation | **0** |
| `keys().next().cloned()` in mounted renderer/Shell production | **0** |
| Faithful P5a verifier mutations | **44 / 44 rejected** |

## Validation

| Gate | Result |
|---|---|
| Original `rustfmt --edition 2021 --config skip_children=true` on the five mounted Rust files | **PASS** (parse and format) |
| Final `rustfmt --edition 2021` parse/format on all 11 second-remediation Rust files | **PASS** |
| Isolated Bun invocation of `interactivityMountedFrameTransactionSelfTests(process.cwd())` after final rustfmt | **PASS** — `P5a isolated verifier passed` |
| Scoped unstaged and staged `git diff --check` on the remediation implementation/verifier files | **PASS** |
| Exact constructor/forbidden/stage/publication residual census | **PASS**, counts recorded above |
| Cargo/Nx/Wasm/browser/native timing matrix | **DEFERRED by coordinator instruction**, not claimed |

## Deferred Integrated Gates

After overlapping source packets are quiescent, run the renderer/UI compile and test matrix through
the repository's Nx/Bun entrypoints, then the native and browser-worker mounted storm fixtures at
1/2/4/default workers. Those gates must confirm every worker opportunity remains below 8 ms and the
already-present callback p99 laws remain at or below 2 ms. This report deliberately makes no runtime
pass claim before those executable gates run.

## Coordinator Third Pre-acceptance Remediation

The third coordinator counterexample was valid. This remediation replaces the remaining blocking
find sink, frame-local platform work, complete text/scene leaves, and byte-only atlas ledger with
retained authorities in the mounted call graph. The result is source-audit-ready for an independent
acceptance pass; it is not self-accepted and makes no broad compile, runtime, or timing claim.

### Exact Third-remediation Inventory

| File | Exact third-remediation change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` | Replaced the dynamic blocking find sink with a generation-qualified fixed 256-item/1 MiB collector, non-nesting worker-local binding, exact owner refusal, one-item transfer/close, and terminal witness. Frame chrome now only coalesces bounded preference, introduction, layout, presence, and persistence requests; one maintenance field/page advances outside the frame opportunity. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` | Mounted Shell maintenance on the shared pool's typed I/O lane, retained/coalesced unfinished maintenance, and mounted one abandoned-atlas close opportunity in `FrameBuildPhase::Deferred` before new candidate work. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️paint.rs` | Added `RetainedNodePaintCursor` byte/glyph/line/pen/chrome state. Text advances one UTF-8 scalar and at most one pre-admitted glyph per grant, checks the 4 MiB cap before layout/product work, advances its byte only after output admission, and closes the bound node explicitly. Non-text nodes receive one fixed 256-output child admission. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️engine.rs` | Retains the exact paint node and scene leaf plus their child cursors across `frame_into_step`; no mounted opportunity calls the legacy complete paint/tree or complete scene collection oracle. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️scene_slots.rs` | Added exact scene-node binding plus checked phase/item/page/single-byte state, stale-owner refusal, incremental close, and terminal witness. The byte API can advance only one scalar per call. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🧊️component.rs` | `FrameworkSceneHost` resumes `render_component_scene_step` or `render_ui_image_step`. Image identifiers and sources advance one byte per opportunity, data decoding fails closed, URL assets use the admitted request/cache boundary, and one raster output item crosses the draw boundary. The complete image renderer is test-only. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Scenes/🧊️component.rs` | Added the retained component-scene scalar/item consumer and made the complete component-scene renderer a test-only oracle. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs` | Replaced the atlas mutex/byte counter with one packed nonblocking `AtomicU64` permit covering item, page, payload, and backing units before allocation. Fixed 2,048 page slots retain the exact permit. Close releases one page, one backing owner, or one permit dimension per grant. `Drop` moves unfinished owners into one of 64 pre-reserved abandonment slots for the same incremental close authority. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️draw.rs` | Preserved one-page atlas upload as the only mounted GPU transfer boundary. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` | Re-exported the retained scene cursor/step boundary directly, without an adapter or alternate authority. |
| `📜️script.ts` | Expanded the P5a verifier to 16 live sources and 69 faithful mutations, including direct find sink/binding/push/take, every synchronous maintenance restoration, paint byte/output/whole-wrap, complete scene/image restoration, scene scalar/stale-owner, all four atlas permit dimensions, allocation order, Drop recovery, mounted abandonment drain, and hostile-law removal. |

### Exact Owner, Refusal, and Close Laws

| Authority | Admission/refusal | Resume and close |
|---|---|---|
| Shell find generation | Fixed slots and aggregate payload bytes are checked before move; stale generation and MAX + 1 return the same `ShellFindItem` pointer identity | One `pop_front` or `close_step` releases one owned item; empty slots plus zero payload bytes form the terminal witness; the worker-local binding cannot nest |
| Shell maintenance | Frame work only sets/coalesces request state; identifiers and stored fields have 4 KiB caps, aggregate configuration has a 64 KiB cap | The shared-pool I/O child advances one preference field, introduction owner, layout page, presence page, or persistence field, then re-enqueues only if work remains |
| Retained text node | A node over 4 MiB faults before layout/glyph/output work and the retained tree owner pointer is unchanged | Each accepted call consumes one UTF-8 scalar and emits at most one glyph; cancellation closes the exact bound node before terminal empty |
| Scene/image leaf | `bind` accepts one `NodeId`; a different live node is rejected without changing the active owner | Checked phase/item/page/byte cursors remain parked until `finish`; one close grant removes the bound owner and the next witnesses terminal empty |
| Atlas process permit | One CAS attempt jointly checks item, page, payload, and backing limits; contention refuses without waiting and allocation occurs only after permit transfer | Normal close and Drop recovery share one page/backing/permit-dimension close authority; the mounted deferred phase drains one abandonment unit before candidate work |

### Hostile Laws and Verifier Evidence

The added direct Rust laws cover:

1. fixed find MAX + 1 pointer identity, stale-generation identity, one-item close, and terminal empty;
2. one-glyph-per-grant text progress, 4 MiB + 1 tree-owner identity, and cancellation close;
3. stale scene-node refusal, one-byte progress, and exact bound-owner close;
4. atlas page-cap refusal before permit transfer, exact one-page close, process item MAX + 1,
   allocation refusal with unchanged packed ledger, concurrent nonblocking/poison-free permit
   attempts, ordinary abandonment, and interruption after a partial close.

The isolated verifier rejects **69 / 69** mutations. The 25 third-remediation additions restore each
counterexample directly in its live source or remove its hostile law; the earlier 44 mounted-stage,
Shell child, document, packet, world, atlas-upload, and publication mutations remain rejected.

### Third-remediation Residual Census

| Census | Result |
|---|---:|
| Dynamic `Arc<Mutex<Vec<ShellFindItem>>>`, whole find take, or unowned find push in the admitted find authority | **0** |
| Synchronous complete preference/introduction/layout/presence calls in `render_chrome_step` | **0** |
| Mounted text `wrap_text`, all-line loop, recursive child call, or byte advance before output admission | **0** |
| Mounted Framework host calls to complete `render_component_scene` / `render_ui_image` | **0** |
| Complete scene/image functions retained as production alternatives | **0**; one of each remains under `cfg(test)` as a legacy oracle |
| Atlas `Mutex<usize>`, byte-only ledger, dynamic page slots, allocation before permit, bulk page clear, or missing Drop recovery in the atlas authority | **0** |
| Mounted `PreparedAtlasPages::close_abandoned_step` sites | **1**, `FrameBuildPhase::Deferred` |
| P5a verifier production sources | **16** |
| Faithful P5a mutations | **69 / 69 rejected** |

### Third-remediation Validation

| Gate | Result |
|---|---|
| `rustfmt --edition 2024` on the 10 touched Rust files | **PASS** |
| Isolated Bun invocation of `interactivityMountedFrameTransactionSelfTests(process.cwd())` after rustfmt | **PASS** — `P5a isolated verifier passed` |
| Scoped unstaged and staged `git diff --check` across the 11 source/verifier files | **PASS** |
| Exact live-boundary residual census above | **PASS** |
| Cargo/Nx/Wasm/browser/native timing matrix | **DEFERRED by coordinator instruction**; not run and not claimed |

The broad renderer/UI compile, hostile Rust test execution, worker-count matrix, browser-worker
replay, and measured under-8 ms opportunity gates remain for the coordinator after overlapping
source work is quiescent.

## Independent RED B1/B2/B3 Remediation

The independent RED was valid. This pass closes its three concrete blockers in the mounted caller
graph and strengthens the source verifier at the actual callees. This is a source-audit-ready
handoff for a fresh independent review, not self-acceptance.

### Exact Changed-file Inventory

| File | Independent-RED remediation |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️paint.rs` | Kept `paint_node_step` exhaustive over every `UiNode` variant and completed the retained collection state: Select lookup now inspects one item per grant, Select MAX + 1 refuses before output, KeyValue advances one label/value scalar at a time, and Tree retains section/item/action/control/depth/ascent state. `RetainedGlyphCursor` is the single 4 MiB-capped string authority. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` | Re-exported the retained glyph authority and its public maximum without a compatibility adapter. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` | Routed live dialog and tour title/body/button strings through `chrome_text_complete_step`, parking the Shell scalar until the retained glyph cursor completes. Replaced production native whole-config JSON/Mutex storage with one fixed 4 KiB field page and bounded per-field paths; panel layout uses a capped six-field encoding. The former full-config store remains test-only. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` | Replaced the discarded generic native maintenance task with a generation-qualified `FrameMaintenanceAuthority`, atomic single-claim `FrameMaintenanceOwnerCell`, exact refusal owner, typed `Lane::Io` admission, mounted cancellation token, deadline/freshness guards, and incremental close before completion handback. Cancel/stale/deadline share one production terminal classifier. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️runtime_mailbox_core.rs` | Added exact cancellation of a refused interaction completion reservation and a law proving only the reserved slot is released. |
| `📜️script.ts` | Traces the live per-variant paint, dialog/tour glyph, maintenance owner/spawn/terminal, and native preference field bodies. Added faithful B1/B2/B3 mutations, repaired the live P5b fixed-boxed arena predicates and mutations, and made P5c mutate the accepted-layout read inside `paint_node_step` rather than an obsolete complete painter. No accepted P5b/P5c implementation source was changed. |

### Exact Owner and Close Laws

| Authority | Refusal/cancel/stale behavior | One-grant close behavior |
|---|---|---|
| Retained node painter | A different node/origin faults without moving the cursor. Select/KeyValue/Tree collection caps reject before output. A string over 4 MiB faults before glyph admission. | One scalar, fixed emitted item, collection item, Tree action/control, depth transition, or terminal presence unit advances. |
| Dialog/tour text | The Shell child scalar remains parked while `RetainedGlyphCursor` is pending; MAX + 1 sets the retained fault path with zero output. | One call admits at most one scalar/glyph; the Shell scalar advances only after the text cursor reports complete. |
| Native maintenance task | Generation reservation precedes submission. `try_submit` refusal recovers the exact owner cell and releases the exact generation; the completion reservation stays explicit until refusal recovery. Cancellation, stale presentation, and deadline are checked before field work. | Every terminal reason calls `begin_close`; `FrameDeferredCursor::close_step` removes one action or one Shell/pump/tutorial owner, and only terminal empty is handed back. |
| Native preference field | Key/value length is capped before path or write work; a read uses `[u8; 4096]`; full-config JSON and its blocking Mutex are not production-reachable. | One maintenance grant reads or writes one bounded field and retains the remaining Shell maintenance phases. |

The new source laws are
`retained_multi_megabyte_input_advances_one_scalar_per_grant`,
`retained_select_max_plus_one_refuses_before_output_without_moving_tree_owner`,
`dialog_and_tour_text_advance_one_scalar_and_one_glyph_per_grant`,
`dialog_and_tour_text_max_plus_one_fails_without_output`,
`frame_deferred_cancel_token_closes_one_exact_owner_per_grant`,
`frame_maintenance_authority_refuses_aba_and_releases_the_exact_generation`,
`frame_maintenance_refusal_cell_recovers_exact_identity_without_blocking`,
`frame_maintenance_cancel_and_stale_each_close_one_populated_owner_per_grant`, and
`rejected_interaction_submission_releases_only_its_reserved_slot`.

### Independent-remediation Residual Census

| Live boundary | Residual |
|---|---:|
| Direct `paint_node_step` boundary calls to `paint_node_self` | **0** |
| Direct retained paint `wrap_text` or whole-value scalar loops | **0** |
| Accepted-layout reads in the mounted `paint_node_step` boundary | **1**, exact live snapshot read |
| Dialog/tour calls to whole `chrome_text` or direct `draw_text` | **0** |
| Native maintenance `KernelPoolFuture::spawn`, blocking `.lock()`, or discarded `try_submit` | **0** |
| Production native preference `Mutex`, `serde_json`, or dynamic 4 KiB page | **0** |
| Faithful P5a mutations | **81 / 81 rejected** |

### Independent-remediation Static Validation

| Gate | Result |
|---|---|
| `rustfmt --edition 2024 --config skip_children=true` on the five touched Rust files | **PASS** |
| Root `📜️script.ts` Bun import/parser | **PASS** |
| `interactivityMountedFrameTransactionSelfTests(process.cwd())` after formatting | **PASS** — P5a **81 / 81** mutations rejected |
| `interactivityLiveReconcileSelfTests(process.cwd())` | **PASS** — accepted P5b source baseline and hostile mutations preserved |
| `interactivityMountedLayoutTextSelfTests(process.cwd())` | **PASS** — faithful `paint_node_step` accepted-layout mutation rejected |
| Scoped `git diff --check` on the six implementation/verifier files and this report | **PASS** |
| Cargo/Nx/Wasm/browser/native runtime gates | **DEFERRED by coordinator instruction**; not run and not claimed |

The broad compile/test matrix, executable Rust hostile laws, worker-count replay, browser-worker
coverage, and measured under-8 ms opportunity checks remain deferred until overlapping source work
is quiescent.

## Fresh Terra Post-RED Remediation

The fresh independent RED was valid. The mounted interactive synchronization, remaining Shell
chrome labels, and accepted native-maintenance execution now retain their exact work and recovery
authority across worker grants. This is a source/static handoff for another independent Terra audit;
it is not self-acceptance and makes no compile, runtime, browser, or measured-latency claim.

### Exact Changed-file Inventory

| File | Fresh post-RED remediation |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️paint.rs` | Replaced mounted whole Select synchronization and recursive Tree synchronization with `RetainedInteractiveSyncCursor`. Fixed source, output-record, depth, and key credits are checked before mutation. Select advances one source-item inspection or one layout write; Tree retains fixed direct-sibling frames and advances one section, item, ascent, child inspection, layout write, or record close. Fault and abandonment close one retained record/depth owner per grant. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️engine.rs` | Mounted synchronization parks the exact node and cursor. Stale revisions and faults enter the same incremental close phase and cannot publish or advance the frame until the synchronization cursor is terminal-empty. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` | Routed mounted Navbar, TutorialBar, footer utility, sync-status, and check-in labels through a retained chrome-group subcursor. Width admission is fixed-cost and capped before work; each grant advances one background, icon, UTF-8 scalar/glyph, border, hit, or terminal transition. Dynamic status/check-in strings remain in `UiText` across grants. The two legacy whole-string helper call seams are now `cfg(test)` only. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` | Added the generation-qualified nonblocking `FrameMaintenanceExecutionRegistry`, accepted-job envelope, and execution guard. Queue loss or interrupted execution restores the exact owner to its pre-reserved cell before publishing one mounted recovery wake; refusal reclaims the queued generation without ordinary-drop loss. Cancel, stale witness, and deadline still enter retained `begin_close`. |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs` | Applied the requested scoped P5b rustfmt preservation only; reconciliation semantics and verifier predicates remain unchanged. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/{🦀️component.rs,🦀️widgets.rs,🦀️mounted_layout.rs,🦀️tree.rs,🦀️events.rs}` | Applied the requested scoped P5c rustfmt preservation only; accepted-layout, text, event, and tree semantics remain unchanged. |
| `📜️script.ts` | Traces the retained synchronization, chrome-group, and native execution registry bodies directly. The suite retains exactly 81 mutations and replaces the duplicate broad Shell glyph mutation with a faithful mounted-navbar whole-label restoration. Navbar, tutorial, footer, overlay, and error production slices all reject direct whole-string painting. |

### Exact Owner and Close Laws

| Authority | Admission and progress | Refusal, stale, fault, and close |
|---|---|---|
| Interactive Select synchronization | The fixed 256-source/512-output census completes before writes. One grant inspects one source item, scans one retained sibling, or writes one child layout. | MAX + 1 faults without moving the tree owner. Engine freshness is checked before each step; stale/fault parks the node while one cursor record/depth owner closes per grant. |
| Interactive Tree synchronization | Each fixed frame retains its stable worker-owned direct-items pointer, item count, next item, output record, and depth. One section/item/ascent/child/layout action advances per grant without path replay or recursion. | Depth, item, output, and key caps fail closed. `close_step` releases one output record or one depth frame and only terminal empty releases the mounted node. |
| Retained Shell chrome group | Label bytes are capped before background/icon work. A group cursor retains the phase and glyph byte; dynamic sync/check-in `UiText` ownership remains parked until completion. | MAX + 1 preserves input identity and stages no group owner. Glyph fault resets the exact group/glyph cursor; mounted callers record a fault and advance only after that retained child is closed. |
| Accepted maintenance execution | Registry publication precedes `try_submit`. A generation can transition once through queued, running, abandoned/recovering, and complete states without a lock, loop, or dynamic recovery set. | Submission refusal reclaims the exact queued owner. Queue Drop and execution-guard Drop restore the exact interaction/cursor into the owner cell before the abandonment wake. Mounted recovery takes only the matching generation and resumes incremental cancel/stale/deadline close. |

### Added Hostile Laws and Faithful Mutations

The fresh pass adds direct Rust laws for retained Select one-row progress and MAX + 1 refusal,
Tree one-record/depth abandonment close, dynamic chrome-group one-output/glyph progress and MAX + 1
identity preservation, accepted maintenance queue Drop handback, and interrupted execution guard
handback. Existing cancel/stale maintenance laws continue to prove that populated action, Shell,
pump, and tutorial owners enter `begin_close` and close one owner per grant.

The P5a verifier rejects **81 / 81** mutations. Five auditor counterexample families are mutated at
their live callees: Select materialization, Tree child materialization, mounted Navbar whole-label
painting, populated deferred bulk close, and maintenance authority release/Drop handback. The
remaining mutations preserve the mounted stage, paint-variant, scene/image, atlas, find, packet,
world-resource, publication, and hostile-law coverage.

### Fresh-remediation Residual Census

| Live boundary | Residual |
|---|---:|
| Mounted engine calls to complete `sync_interactive_state_node` | **0** |
| Retained synchronization `Vec`, `collect`, source clone, `tree.children` replay, recursion, `for`, `while`, or `loop` | **0** |
| Mounted Navbar/Tutorial/Footer/Overlay/Error calls to direct `chrome_text` or `draw_text` | **0** |
| Production helper seams capable of reaching whole `chrome_text` | **0**; the two former production-defined seam locations are now excluded by the `cfg(test)` panel-tab item and the `cfg(test)` label block in the legacy chrome-group oracle; other direct calls live only inside independently `cfg(test)` legacy render oracles |
| Maintenance execution registry `Mutex`, blocking lock, dynamic owner set, whole take, or recovery loop | **0** |
| Accepted execution path without generation-qualified external handback | **0** |
| Faithful P5a mutations | **81 / 81 rejected** |

### Fresh-remediation Static Validation

| Gate | Result |
|---|---|
| `rustfmt --edition 2024 --check --config skip_children=true` on the four fresh P5a implementation files | **PASS** |
| `rustfmt --edition 2021 --check --config skip_children=true` on exact P5b `reconcile.rs` and five P5c files | **PASS** after scoped formatting |
| Root `📜️script.ts` Bun parser plus `interactivityMountedFrameTransactionSelfTests(process.cwd())` | **PASS** — P5a **81 / 81** mutations rejected |
| `interactivityLiveReconcileSelfTests(process.cwd())` | **PASS** — accepted P5b reconciliation baseline and hostile mutations preserved |
| `interactivityMountedLayoutTextSelfTests(process.cwd())` | **PASS** — accepted P5c layout/text baseline and live accepted-layout mutation preserved |
| Scoped `git diff --check` across implementation, preservation, verifier, and report files | **PASS** |
| Cargo/Nx/Wasm/browser/native hostile-law and timing gates | **DEFERRED by coordinator instruction**; not run and not claimed |

The renderer/UI compile matrix, executable Rust hostile laws, browser-worker replay, native
worker-count matrix, and measured under-8 ms opportunity gates remain for the coordinator after
overlapping source work is quiescent.

## Narrow Legacy Chrome and Formatter Remediation

The final narrow Terra RED was valid. `measure_chrome_group_item` and `render_chrome_group` had no
mounted caller, but their definitions still compiled in production and retained a whole-string
`FontAtlas::measure_text` reachability. Both definitions, `chrome_group_border`, and every remaining
legacy caller that can reach them are now explicitly `#[cfg(test)]`. The mounted Navbar, Tutorial,
Footer, Overlay, and Error paths remain on `RetainedChromeGroupCursor` and `RetainedGlyphCursor`.

### Exact Source and Verifier Changes

| File | Change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` | Made `chrome_group_border`, `measure_chrome_group_item`, `render_chrome_group`, `render_presence_bar`, `render_footer_utility_nodes`, `render_studio_canvas_bars`, the three legacy window rails, `render_staged_form`, and `render_staged_arg` test-only. Normalized the already-test-only native preference conditional into edition-2021 syntax without changing behavior. |
| `📜️script.ts` | Binds both legacy definitions to exact `#[cfg(test)]` attributes, excludes them from the production-source view, forbids their restoration in every mounted child slice, and adds three faithful legacy-chrome mutations: remove either test boundary or restore the legacy renderer inside mounted Navbar work. Updated the P5b token-publication predicate for rustfmt's multiline `.take_ready()` form and made the two-caller raw-completion mutation alter both live callers. |
| P5b/P5c declared Rust inventories | Applied formatting only to the complete edition-2021 source inventories requested by the independent audit. No compatibility or runtime adapter was added. |

### Final Residual Census

| Boundary | Residual |
|---|---:|
| Production-compiled `fn measure_chrome_group_item` | **0** |
| Production-compiled `fn render_chrome_group` | **0** |
| Mounted child calls to either legacy helper | **0** |
| Production-mounted whole-string `FontAtlas::measure_text` through the legacy helper chain | **0** |
| Exact legacy helper attributes | **2 / 2 `#[cfg(test)]`** |
| P5a hostile mutations | **81 / 81 core plus 3 / 3 legacy-chrome rejected** |

### Final Source/Static Gates

| Gate | Result |
|---|---|
| `interactivityMountedFrameTransactionSelfTests(process.cwd())` | **PASS** — 81 core plus 3 legacy-chrome mutations |
| `interactivityLiveReconcileSelfTests(process.cwd())` | **PASS** after faithful live-token formatting predicate repair |
| `interactivityMountedLayoutTextSelfTests(process.cwd())` | **PASS** |
| Edition-2021 `rustfmt --check --config skip_children=true` on the P5b nine-file union | **PASS** — accepted eight-file set plus the auditor-declared plugin shard |
| Edition-2021 `rustfmt --check --config skip_children=true` on the declared P5c eight-file set | **PASS** |
| Scoped `git diff --check` over root verifier and all P5b/P5c source inventories | **PASS** |
| Cargo/Nx/Wasm/browser/build/runtime gates | **DEFERRED by coordinator instruction**; not run and not claimed |

The shared Shell, renderer glue, UI-WGPU engine, and paint sources are intentionally left in the
auditor-required edition-2021 format. Reapplying edition-2024 formatting to that shared subset would
invalidate the explicit P5b/P5c preservation gate. This handoff is source-audit-ready for the
requested narrow fresh Terra audit; it is not self-acceptance.
