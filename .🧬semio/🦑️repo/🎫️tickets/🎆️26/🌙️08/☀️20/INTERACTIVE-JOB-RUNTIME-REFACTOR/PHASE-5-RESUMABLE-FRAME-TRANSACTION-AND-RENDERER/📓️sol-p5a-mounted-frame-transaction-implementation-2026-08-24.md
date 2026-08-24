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
| `📜️script.ts` | Added the permanent P5a live-source verifier and 33 faithful structural mutations, including the mounted Build/Finish callee graph; registered it in `verify interactivity`. Existing peer edits in this file were preserved. |
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

The permanent verifier rejects 33 mutations covering:

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
33. deferred work driven to completion in one opportunity.

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
| Complete icon/glyph pixel clone | `icon_offset` and `glyph_offset` copy at most `FRAME_ATLAS_PAGE_BYTES == 16 KiB` per grant | Upload item credit is checked before transfer; partial page buffers are retained and page-truncated on close. |
| Complete `render_chrome` | `ShellChromeFrameCursor` has explicit setup, main, left, right, navbar, tutorial, footer, overlay, drag, gesture, error, and persistence phases | Dynamic previous-frame collections release one exact entry per setup grant; each render child has exactly one call site and one opportunity. |
| `take_packets` | `EngineCanvasBuildContext` and mounted `FrameEnginePackets` each own 256 fixed optional slots; one packet or the exact rejected packet crosses `take_packet_step`/`try_push` | Rejected and accepted packets close through `EngineCanvasPacket::close_step`; terminal requires every producer and collector slot empty. |
| World `append_to` | `World3dBuildContext` owns fixed upload, eviction, mesh-request, and raster-request slots; `append_step` moves one owner/request | Admission checks prepared-input credit before move; rejected upload/eviction remains explicit and closes incrementally. |
| Immediate deferred-frame drive | Finish only retains a `FrameDeferredCursor`; new frame input is routed before the old deferred cursor receives one shared-pool submission opportunity | No Finish call executes deferred product/plugin work inline and no pending owner is overwritten. |

The strengthened verifier reads the production bodies of renderer glue, Shell, EngineCanvas, and
World3D. It rejects restored opaque calls, dynamic resource fields, bulk transfers/clears, complete
atlas clones, duplicate chrome-child calls, immediate deferred driving, and deferred run-to-
completion mutations. The exact named counterexample residuals are all zero.

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
| Faithful P5a verifier mutations | **33 / 33 rejected** |

## Validation

| Gate | Result |
|---|---|
| Original `rustfmt --edition 2021 --config skip_children=true` on the five mounted Rust files | **PASS** (parse and format) |
| Remediation rustfmt edition-2021 parse emission on glue, Shell, EngineCanvas, and World3D | **PASS** |
| Isolated Bun invocation of `interactivityMountedFrameTransactionSelfTests(process.cwd())` | **PASS** — `P5a isolated verifier clean` |
| Scoped `git diff --check` on the remediation implementation/verifier files | **PASS** |
| Exact constructor/forbidden/stage/publication residual census | **PASS**, counts recorded above |
| Cargo/Nx/Wasm/browser/native timing matrix | **DEFERRED by coordinator instruction**, not claimed |

## Deferred Integrated Gates

After overlapping source packets are quiescent, run the renderer/UI compile and test matrix through
the repository's Nx/Bun entrypoints, then the native and browser-worker mounted storm fixtures at
1/2/4/default workers. Those gates must confirm every worker opportunity remains below 8 ms and the
already-present callback p99 laws remain at or below 2 ms. This report deliberately makes no runtime
pass claim before those executable gates run.
