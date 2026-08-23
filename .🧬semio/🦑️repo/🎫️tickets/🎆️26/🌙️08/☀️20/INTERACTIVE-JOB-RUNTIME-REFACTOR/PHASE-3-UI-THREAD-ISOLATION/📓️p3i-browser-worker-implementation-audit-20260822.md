# Phase 3i Browser Worker Implementation Audit

Date: 2026-08-22  
Verdict: **Source and focused TypeScript gates advanced; Phase 3 remains open.**

## Implemented browser boundary

- The browser renderer now transfers an `OffscreenCanvas` to one dedicated module Worker. The UI
  isolate owns only bounded admission/coalescing, message transfer, RAF scheduling, callback timing,
  and directives. There is no inline redraw fallback.
- Boot receives deterministic renderer JavaScript/Wasm URLs. Product catalog/plugin discovery and
  module loading execute in the Worker. Async operations are passed as thunks so their synchronous
  prefix is timed, and a post-resolution macrotask lets the heartbeat observe a long final promise
  continuation.
- Lossless text/paste ingress uses conservative UTF-8 reservation, surrogate-safe chunks, and a
  4-KiB wire batch ceiling. Pointer movement uses sixteen fixed numeric identity slots. Future
  generations and protocol corruption fault closed.
- Worker frame realization and offscreen presentation use the Worker-owned present capability; the
  browser Worker does not mint the native UI presentation token.
- Fault/quarantine now shuts the interactive scheduler and enters the same bounded close loop as a
  requested close. Runtime and interactive scheduler close steps alternate across macrotasks. The UI
  transport no longer force-terminates after a fixed timeout; it terminates only after Worker
  `closed` acknowledgement.

## Shared interactive-job substrate

- A domain-neutral `InteractiveJobPort` is exported through the React host port seam. It has fixed
  slots, revisioned stable readiness snapshots, bounded observers, per-kind page limits, aggregate
  process item/byte credits, exact generation/operation authority, and closeable consumers.
- Consumers remain owned through terminal, fault, replacement, and port close until both
  `closeStep()` and `terminalIsEmpty()` witness completion. Normal completion drains only its slot and
  leaves other jobs and the shared port live.
- The Worker registry is a fixed static descriptor table. There is no dynamic map or Diagram logic
  in the transport. Each scheduled turn performs one governed job action/phase transition. Factory,
  ingress, cancellation, step, output, close, and post exceptions are converted to protocol
  quarantine and retained bounded close.
- The first registered job kind is `diagram-directed-layout-v1`, implemented in the P10-owned pure
  module and executed on the existing frame Worker rather than a second Worker.

## Paged text authority progress

- Browser text streams use slot epochs, aggregate byte credits, segmented pages, Unicode-safe
  boundaries, balanced cached-byte roots, atomic root publication, and bounded projection.
- Focus now accepts already-owned identifier/value strings. Full editor-document focus no longer
  clones the document into the generic input buffer; editor focus publishes only its owned identity.
- Older undo roots are retained in fixed root-retirement slots before disposal. Publication yields
  before mutation if retirement credit is unavailable; undo leaves ownership in place until credit
  exists.
- `InputState` exposes close-step and terminal-empty witnesses and the browser runtime uses them.
- `InputState` now preallocates and caps hit targets, pending lossless events, pending keys, and drag
  points. Overflow records a deterministic authority fault instead of allowing unbounded growth.
- `TextEditAuthority` fail-safe destruction now also detaches owned ingress, cancelled ingress,
  retired storage, and every page Arc, so an abandoned authority has a shallow destructor. Normal
  cancellation retains owned ingress until its operation retirement is admitted and releases pages
  cursor-by-cursor.
- `OsHostRetirement` now owns the realm-close cursor. Frame handles, host input events, deadline maps,
  snapshots, capabilities, scheduler/kernel/runtime, and presenter are transferred across separate
  outer cursor turns, followed by an explicit terminal-empty witness. Its fail-safe destructor
  detaches remaining owners rather than recursively destroying them. Inner runtime/presenter/GPU
  retirement is not yet accepted and remains listed below.

## Persistent frame pipeline progress

- `FrameBuildJob` no longer scans/clones its entire deadline map in one step. It retains the owned
  iterator, admits at most 256 bounded identifiers, visits at most sixteen entries per governed step,
  and publishes typed directives without a JSON encode/decode round trip.
- Neither native WorkerPool scheduling nor the wasm Worker frame path calls `run_to_completion`.
  Native returns the owned `ActiveFrameBuild` through its capacity-one completion channel and
  resubmits one phase; wasm retains the same active authority in `FrameBuildHandle` and advances one
  phase per tick.
- Prepared render measurement is retained as `AppFramePreparation` and advances at most 64 measured
  items under a one-millisecond step deadline. A presentation becomes visible only after terminal
  completion; pending, cancelled, stale, and faulted generations leave the previous valid surface
  untouched.
- Runtime mailbox replay is a separate persistent phase. It dequeues at most one ready completion
  per Worker turn, including stale keyed completions; the former loop could skip all 128 entries in
  one callback. Product frame construction begins only after a later turn observes that phase idle.
- Runtime completions are now a closed typed enum rather than arbitrary `FnOnce` closures. Lossless
  drained input is retained in `RuntimeDispatchCursor`: pointer, scroll, then the fixed four discrete
  slots advance one normalized event per capacity-reserved interaction completion. Saturation retains
  the cursor and interaction authority for retry, restoration completions cannot be stale-key
  coalesced, and abandoned cursor payloads expose a one-event-per-close-step retirement witness.
- Stale/cancelled deadline, apply, build, and prepared phases use one shared retirement transition.
  Prepared uploads, draw lists, engine-canvas packets, and the underlying Vello encoding retire one
  owned leaf per turn before terminal-empty, rather than being discarded with the channel result.
- World3D deadline admission rejects a 257th identity and identifiers over 256 bytes before the
  bounded frame-input clone. Rejection becomes a deterministic `frame-credits` Worker fault.
- Normal `EventQueue` consumption now transfers at most four lossless events per frame reduction.
  Each owned event is capped at 4 KiB and the fixed 256-event queue has aggregate byte credits; the
  former `drain(..).collect()` whole-queue allocation was removed.
- Each active frame phase now checks the process watchdog violation authority immediately after its
  governed step. A matching overrun records a mailbox `frame_fault`, cancels the generation, and
  routes any completed-but-unpublished preparation through bounded retirement before presentation;
  the last valid surface remains published.
- `ActiveFramePhase::Build` now owns `AppFrameTransaction` rather than calling `AppRuntime::frame`
  directly. Live World3D deadline candidates advance one admitted surface per Worker turn. The
  global scene-camera deadline authority is transferred O(1) into `SceneCameraDispatchCursor`, which
  advances or restores one bounded identifier per turn and rejects the 257th/over-256-byte identity
  before ownership. Cancellation restores future deadlines one per close turn.
- Chrome output is retained in `AppFrameAfterChrome`. `InputState` now owns a fixed 256-slot FIFO and
  transfers exactly one action directly into the transaction per turn; the former whole-`Vec`
  replacement/`IntoIter` transfer was removed. Cancellation retires one queued event or deferred
  action per close turn, followed by the prepared draw/upload and engine-packet cursors. Final
  publication remains generation-owned and occurs only after the separate finish phase.
- Wheel propagation no longer traverses every World3D/node-graph/map/board surface in the finish
  callback. Each surface map maintains a deterministic admission-order page with fixed
  256-item/256-byte credits; the transaction indexes that page in O(1), resolves one map entry, and
  drains each bounded generated-action page one action per later turn. The previous
  `HashMap::{values_mut,iter}().nth(index)` traversal was nondeterministic and cumulative O(N²).
  Pending raster traversal uses the same stable page. Aggregate deferred-action credit remains 256
  across deadline, input, and wheel sources.
- The stable surface authority no longer exposes `Deref`/`DerefMut` to its backing `HashMap`.
  Replacement, removal, clear, lookup, iteration, and value mutation are explicit audited methods;
  a value-only `World3dStateAccess` trait lets legacy asset polling update states without permission
  to structurally mutate keys behind the order page. Source fixtures cover replacement, removal,
  clear, stable order, identifier saturation, and the 256/257 boundary. Full-map clear remains a
  separate chrome-retirement red below.
- Pending scene raster uploads no longer scan every scene and append every pixel buffer in one frame
  callback. `PendingRasterUploadCursor` visits one capped scene state or transfers one admitted
  upload per turn; uploads are capped at sixteen per scene, one MiB of pixels, and 256-byte keys. The
  unchanged-source check no longer clones the full scene state before admission.
- Frame-deferred actions no longer enter one `dispatch_actions(Vec)` future. `FrameDeferredCursor`
  retains the aggregate action authority and advances one pump/action/tutorial/assets operation per
  reserved interaction completion; mailbox saturation retains the cursor for retry instead of
  dropping the action vector. Each ordinary action therefore crosses the async product boundary
  alone, and cancellation has a one-action close step.

## Action authority packet — 2026-08-22

- Added a schema-first `BoundedActionQueue::reserve` / `BoundedActionReservation` producer seam.
  It checks the fixed item slot and declared aggregate byte credit before allocating either of the
  action's fixed buffers. Publication cannot race capacity because the reservation exclusively
  borrows the queue until `publish`.
- `BoundedAction` is non-`Clone` and has no recursive or per-node heap ownership. Controller,
  action, key, and value text are copied into one 16-KiB inline byte slab; the at-most-256 nodes are
  index-linked `Copy` records with a maximum depth of 32. Builder failures poison the builder and
  release only those two flat fixed buffers. Queue close releases one flat owner per grant.
- Per-item string, node, depth, and byte credits and aggregate queue item/byte credits are explicit.
  Focused source fixtures cover max-plus-one reservation, hostile depth, full-queue refusal before
  builder creation, FIFO retry after credit returns, one-owner close, and permanent rejection of
  `mem::forget`, background-drop spawning, and `BoundedAction: Clone`.
- `InputState::reserve_action` exposes the owned producer seam and the frame transaction consumes
  one `BoundedAction` per turn through a consuming `into_descriptor` handoff. The former
  `take_event_step` call is removed from the frame path.
- This is a source-safe intermediate boundary, not action acceptance. Existing Interpreter/Scenes
  producers still construct recursive `ActionDescriptor`/JSON values first and enter the temporary
  `InputState::queue_event` bridge. That bridge reserves before copying into the flat queue and
  records deterministic credit faults, but it cannot retain the already-constructed source owner on
  saturation and performs a bounded-tree copy only after the legacy object exists. Therefore no
  production producer is yet claimed schema-first sealed, and lossless saturation remains red.
- `rustfmt --edition 2021` completed on the four touched Rust files and `git diff --check` was
  clean. Cargo, Nx, and Wasm/browser gates were deliberately not run for this packet at coordinator
  direction; the shared target directory had been concurrently removed, so the next build is cold.

### Action producer migration continuation

- The production `.queue_event(` scan is now zero and the temporary `InputState` bridge was
  removed. `InputState` exposes only single/batch reservation, direct publication, and one-owner
  consumption. Action faults are observed by the frame transaction before another queued owner.
- Batch admission is atomic: up to sixteen actions reserve aggregate item/byte authority before a
  semantic mutation, stage only flat non-`Clone` owners, and publish in FIFO order. An incomplete or
  over-credit batch publishes nothing. Saturation fixtures retain the caller's semantic string,
  release one credit, retry, and prove the retried action remains last in FIFO order.
- Text-editor action routes no longer construct JSON/Dsl descriptors in production. They reserve
  exact two- or four-action batches before EditorHost mutation, compute checked credits from
  controller/surface/projected-document/selection bounds, and write `textSelect`/`textEdit`
  directly. Failed menu, key, completion, rename, and context-click admissions retain their exact
  source for retry. The old Vec-returning text helpers are test-only and an exhaustive source fixture
  enforces that boundary.
- Pending keys now use a fixed 64-slot/4-KiB aggregate ring. Rendering transfers one key per turn;
  retry restores the popped owner at the FIFO front. The two whole-`Vec` `drain_keys` paths were
  removed and the source fixture rejects their return.
- Generic scene events now enter a fixed 256-slot `SceneInteractionIntent` FIFO carrying generation,
  stable window/node/surface identity, geometry, and scalar event data. One retained intent advances
  per frame turn. Capacity refusal occurs before the identifier clone; a failed action reservation
  restores the exact intent to the FIFO front. Cursorized close/terminal-empty witnesses release one
  bounded intent per grant.
- Canvas2d and TextEditor consume that intent through direct pre-mutation builders. Canvas pointer
  batches stage their complete flat action set before updating pointer/drag/stroke state; text batches
  do the same before updating EditorHost. Canvas pan/wheel mutations emit only the separately retained
  camera-deadline authority.
- The live generic scene route no longer calls a `Vec<ActionDescriptor>` producer. Legacy
  `handle_scene_wheel`, `handle_scene_pointer_move`, `handle_scene_pointer_button`, and their
  recursive Ink action helpers are test-only; a permanent source fixture verifies the attributes,
  rejects the former max-batch transcode, and rejects whole-scene clones. Passive table/VFS/paint/
  timeline/diff/feed mutations use a no-action scalar route.
- TextEditor and Canvas2d now execute against a borrowed retained node. The former
  `Some(scene.clone())` copied arbitrary document/scene payloads after an intent was popped; it is
  removed. Canvas state reads use a scalar `(Viewport, pan, paint-active)` projection instead of a
  `SceneSurfaceState` clone.
- Every admitted intent captures the retained tree revision, generational `NodeId`, surface kind,
  and bounded surface identifier. The engine increments the revision on a changed tree. Every
  resume revalidates the full witness before mutation/publication; stale work enters the same
  retained close cursor. A fixture replaces the tree between enqueue and drive and asserts zero
  action publication plus terminal-empty retirement.
- InkCanvas owns a retained generation-tagged `InkInteractionJob`. Document/selection admission is
  capped at 16 KiB, 256 values, depth 32, 256 identifiers, and 4 KiB aggregate identifier text.
  Wheel, hover, direct selection, pan, marquee, primitive/stroke creation, move, resize, stroke,
  stroke eraser, point eraser, and pointer-up commit all have explicit retained routes. Block scans
  advance one admitted block per turn. Eraser fragments advance one remove/add event per later turn.
  Dynamic event JSON writes into fixed 4-KiB storage with rollback on failed append; publication
  reserves exact queue/node/string/byte credits before the semantic state closure runs.
- `BoundedActionReservation::publish_with` and its batch equivalent first finish/validate the flat
  owner, then run the infallible semantic commit, then publish FIFO. A poisoned or incomplete owner
  never invokes the semantic closure. Canvas pointer state and all direct Ink mutations use this
  boundary.
- Nested parsed Ink values no longer final-drop with a top-level block. `InkValueRetirementCursor`
  destructively opens one array/object node at a time into a fixed 128-slot stack (the admitted
  maximum depth requires at most 96 live slots), retires one leaf/key per close grant, and witnesses
  empty before the intent owner is released. Selected/result identifiers, pending eraser fragments,
  stroke preview, drag maps/lists, blocks, and asset values all join the same close sequence.
- Source-only evidence for this continuation: `rustfmt --edition 2021` completed on the five action/
  engine/product files; path-scoped `git diff --check` completed cleanly; production scans found no
  `.queue_event(`, `drain_keys(`, live generic max-batch transcode, whole-scene clone, or
  `mem::forget`. Cargo, Nx, Wasm, and browser runtime gates were not run by coordinator direction,
  so this remains compile/runtime-unverified.
- **Next action residual:** bespoke NodeGraph/TiledMap/Board EngineCanvas APIs still expose live
  `Vec<ActionDescriptor>` producers. They are excluded from the generic intent path by the explicit
  bespoke-dispatch gate, but remain reachable from their separately retained surface cursors and
  have not yet been converted to schema-first flat builders. This packet therefore closes the
  generic scene/Ink ownership bypass only; it does not claim the entire product action graph green.

### Bespoke action and shallow Ink continuation

- NodeGraph, TiledMap, and Board pointer/wheel call sites no longer consume their legacy
  `Vec<ActionDescriptor>` return APIs in production. Direct `_into` routes reserve the flat action
  authority first and the old APIs are test-only. This is not yet a lossless acceptance boundary:
  the engine hosts currently mutate before their result snapshot/encoding finishes. A post-mutation
  cap/serialization failure can therefore abandon a reservation after preview state changed; Board
  can additionally take buffered events before the flat owner is complete. These routes remain RED
  until each host exposes an immutable bounded operation/result plan followed by an infallible
  `publish_with` commit, or an exact retained rollback owner. Worst-case slot reservation alone is
  not counted as correctness evidence.
- Ink interaction documents no longer retain the recursively owned parsed `InkDocumentJson`.
  Admission keeps one capped raw document allocation and records nested block byte ranges in a fixed
  256-entry Copy span page. The scanner records parent then nested `children` in stable depth-first
  order; one block is materialized per interaction turn. Point-eraser fragments use one fixed byte
  slab plus Copy spans, and stroke updates use one fixed raw slab, so an interrupted job no longer
  owns a `VecDeque<Value>`/`Option<Value>` fragment graph. The source scanner and nested-order fixture
  are parse/format evidence only; exact legacy differential behavior and release-build interrupted
  drop remain unrun until the serialized Rust gate.
- The retained Ink owner is now shallow on ordinary release, but the admission turn still parses a
  bounded `InkDocumentJson` before transferring it into the raw slab, and each scan turn materializes
  one bounded block `Value` for the existing geometry helpers. Those temporary CPU/destructor costs
  are capped by the 16-KiB/256-node admission contract but are not yet runtime-timed; they remain a
  hard-ceiling RED until the serialized build/browser timing gate or direct flat-node geometry proves
  the bound. No claim is made that shallow retained ownership alone proves the eight-millisecond step.
- `rustfmt --edition 2021` and path-scoped `git diff --check` were rerun after the bespoke and Ink
  edits. Cargo, Nx, Wasm, and browser gates remain deliberately unrun at coordinator direction.

### Immutable wheel-plan checkpoint

- `MapHost` now exposes an immutable wheel/down/move/up interaction plan. A plan carries its source
  revision plus exact camera and pan-gesture witnesses, derives the next camera/gesture without
  mutating the host, and exposes the planned camera to the flat action builder. The renderer builds
  all three map actions before calling `publish_with_checked`; that API revalidates and commits the
  plan before publishing, and publishes nothing when commit returns false. Direct-vs-plan, stale
  revision, and full action-queue/no-camera-mutation fixtures were added.
- `GraphHost`, `FlowHost`, and `BoardHost` now expose the same immutable/revisioned plan shape for
  wheel camera transactions. NodeGraph and Board wheel renderer routes build their flat camera/
  interaction owners from the plan and use checked publication. Board event ownership is peeked
  before wheel publication instead of being taken before flat encoding. This is progress, not a
  complete bespoke acceptance boundary: Graph/Flow pointer down/move/up still mutate Dag/Flow
  gesture state before their selected/hover/camera snapshot is encoded; Board pointer routes still
  mutate graph/brush/link state and drain recursive event rows before flat encoding. Board's wheel
  commit also clears the recursive pending-event vector in one turn. Those routes remain RED until
  host-owned pointer plans and shallow/cursor-retired event pages replace these paths.
  Focused source fixtures saturate the action queue before Map, Graph, and Board wheel admission and
  assert the respective camera remains byte-for-byte unchanged; these fixtures are authored but not
  executed pending the serialized Rust gate.
- `BoundedActionBatchReservation` gained checked complete and checked partial publication. Both
  finish the already-flat owner first, invoke a boolean revision commit, and publish no queued owner
  on rejection. Source fixtures cover rejected complete and partial batches.
- The shallow Ink block scanner now descends `children` only for a block whose decoded `kind` is
  `group`, matching the legacy traversal even when JSON field order varies. A non-group block with a
  `children` property is included in the stable depth-first differential fixture and its child is
  intentionally absent.
- Ink CPU remains explicitly RED. `checked_ink_document` still deserializes the full capped 16-KiB
  document to `InkDocumentJson`, validates its recursive values, clones the raw source, and scans it
  again during interaction-job construction. Production `ink_current_camera`, selection/geometry
  helpers, and `render_ink_canvas` still synchronously deserialize a whole document; render then
  clones overrides, recursively flattens blocks, and traverses every block in one frame turn. The
  legacy action helpers around the earlier pointer sites are test-only, but the render/current-camera
  sites are live. No measured sub-eight-millisecond evidence exists, so neither construction nor
  render is considered bounded.
- Source evidence for this checkpoint is `rustfmt --edition 2021` plus path-scoped
  `git diff --check`. Cargo, Nx, Wasm, and browser runtime gates were not run by coordinator
  direction; these APIs and fixtures remain compile-unverified.

Exact live residual routes after this checkpoint:

- `node_graph_pointer_down_into`, `node_graph_pointer_move_into`, and
  `node_graph_pointer_up_into` still call the mutating Flow/Dag pointer APIs before the flat
  selection/hover/viewport result exists. The wheel route alone has crossed the immutable plan
  boundary. Graph selection admission now reads the O(1) selected count, validates at most the fixed
  node cap through borrowed identifiers, and only then clones the admitted identifiers; it no longer
  serializes then reparses the whole selection to discover overflow.
- Map wheel and pan pointer routes use the immutable MapHost plan. Map selection admission likewise
  checks the two O(1) set lengths and borrowed string bytes before cloning. The map marquee/feature
  selection branches in `Scenes` still own separate scene-state vectors and feature traversal and
  are not included in this host-camera transaction claim.
- `puzzle_board_pointer_move_into`, `puzzle_board_pointer_up_into`, and
  `puzzle_board_pointer_leave_into` remain mutate/drain-before-encode. Board wheel camera mutation is
  planned, but `board_drain_into_buffer`, `coalesce_board2d_events`, and successful pending-event
  clearing still deserialize/traverse/drop recursive aggregate rows in one turn. Board pointer and
  event ownership therefore remain RED.
- Remaining live recursive action producers outside these bespoke routes include World3D
  `handle_world3d_pointer_move`, `handle_world3d_paint_actions`,
  `handle_world3d_pointer_button`, gumball/pick/marquee helpers; Shell keybinding/directory actions;
  and the native/wasm `request_media_frames` aggregate `Vec<ActionDescriptor>` paths. EngineCanvas
  drag/drop action helpers also remain typed `ActionDescriptor` producers. The many legacy graph/
  map/board/text/Ink helpers found by a signature scan are guarded `#[cfg(test)]`; the listed
  World3D/Shell/media/drop routes are the next production cutover, not verifier allow-list entries.

## Executed gates

- `bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker` — **passed**, 2 files / 32
  tests.
- `bun nx run @semio-tech/framework-renderer-wgpu:check-browser-worker` — **passed**; UI boot bundle
  39.60 KB, Worker bundle 0.63 MB.
- `rustfmt --edition 2024` was run on the modified Rust source files and completed successfully.
- `bun nx show project @semio-tech/framework-renderer-wgpu` confirmed the permanent `wasm` Nx target.

## Unrun and blocking gates

- Cargo, Trunk, wasm-bindgen compilation, native renderer tests, and the real browser
  Worker/OffscreenCanvas harness were not run. Filesystem free space recovered to approximately
  111 GiB, but the shared `target` directory was concurrently removed, making the next build cold.
  The exact controlled build
  command is `bun nx run @semio-tech/framework-renderer-wgpu:wasm`; its output directory is
  `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu`. A cold or invalidated incremental wgpu Wasm
  build can plausibly add approximately 0.5–3 GiB, so it is unsafe in the current disk state.
- `BrowserRendererWorker::close_step` now transfers the host into persistent `OsHostRetirement` and
  requires its terminal-empty witness. The outer host fields are cursorized, but `RuntimeMailbox` and
  `AppPresenter` remain single-owner retirement steps; their internal AppRuntime/Shell/GPU graphs are
  not yet cursorized. Therefore the hard sub-8-ms realm-wide close proof remains red.
- The frame now has an owned transaction and persistent deadline/input cursors, but
  `frame_before_input` still contains text stepping, shell/theme state, tutorial, and the synchronous
  `render_chrome`; `frame_after_input` still contains glyph-atlas publication and asset-poll
  discovery. Deferred actions are cursorized, but the individual tutorial/assets operations and
  other event-handler call sites can still perform scale-dependent async work. The watchdog
  deterministically quarantines an overrun and retires `AppFrameAfterChrome` before publication, but
  cannot preempt those remaining synchronous calls. They still need internal item/byte/deadline
  cursors before the frame hard ceiling is source-proven.
- The generic Interpreter/Scenes path now originates actions through direct flat builders, but the
  bespoke NodeGraph/TiledMap/Board EngineCanvas APIs still originate recursive
  `Vec<ActionDescriptor>` batches. Those producers must be converted to retained schema-first jobs
  before the complete product action graph can be called lossless under saturation.
- `InputState::clear_frame()` still clears the capped hit-target collection in one call, and one hit
  target can own an `ActionDescriptor` tree. `ShellState::render_chrome_build` similarly clears and
  rebuilds graph/map/board/widget/find/tooltip/element structures synchronously. Fixed count alone is
  not a hard byte/deadline proof; old-frame ownership needs a retained retirement cursor.
- Icon/glyph atlas publication still clones a complete pixel buffer in one phase. Asset discovery
  uses four `collect_pending_*` helpers merely to test non-empty state, allocating/cloning aggregate
  pending vectors. `poll_pending_assets` then collects/fetches/buffers/decodes/applies all assets in
  one reserved future, while native `fetch_url_bytes` can execute synchronous filesystem reads.
  Event-maintained pending roots and per-asset retained I/O/decode/apply jobs are still required.
- Shell chrome still performs synchronous preference/layout/introduction storage, presence
  heartbeat, and atlas/resource work. Those platform/product operations have not yet been separated
  into resumable jobs.
- The actual presenter path is also still red: `build_and_publish_snapshot` reaches
  `EngineCanvasPresenter::realize`, whose per-packet work can create a Vello renderer, allocate or
  replace textures, render to texture, clone views, and register textures. Metrics handling performs
  synchronous presenter/GPU resize. The UI presenter must be reduced to one measured bounded
  prepared submission/directive handoff; current source does not establish the two-millisecond UI
  callback gate.
- Generic `ui_render::Dispatcher` no longer performs three full-map registration prunes on every
  event, hit testing no longer allocates overlay/normal child vectors, and pointer/registration/
  overlay/focus/scroll state has hard caps. It still owns its legacy `String` edit state and ignores
  segmented edit events. The paged authority is integrated into the actual wgpu product path, but it
  is not yet the sole authority for every generic/native dispatch consumer.
- Worker frame admission now caps the JSON wire message at 4 KiB (down from 16 KiB), including
  worst-case escape-aware UI chunking. JSON stringify, Rust decode, atomic preflight, event apply, and
  tick still share one Worker callback and have no resumable parse/apply cursor, so only runtime timing
  can establish whether this cap is sufficient; a hard source proof remains red.
- The owned convenience path now rejects payloads above one 16-KiB page. Larger replacement/paste
  must use segmented begin/push/commit, whose pages own independent bounded allocations. Ordinary
  generic/native dispatch is not wired to that segmented requirement yet and therefore remains part
  of the `DispatchState` residual above.
- Rust compiler evidence and actual runtime timing are mandatory before Phase 3 acceptance. No claim
  is made that the Rust/Wasm browser path compiles or runs.
- Rust bootstrap phase labels are persistent, but atlas construction/upload, plugin parse/filter,
  `ShellState` construction, `shell.boot().await`, and final runtime/host construction remain coarse
  owned phases rather than internally fuel/deadline-cursorized jobs. The thunk/heartbeat detects an
  overrun; it does not prevent the owned CPU phase from overrunning before quarantine.

### Revisioned Graph/Flow pointer and shallow Board-plan source checkpoint

- The Graph/Flow pointer renderer route no longer calls `pointer_down_screen`,
  `pointer_move_screen`, or `pointer_up_screen` before action ownership exists. `DagHost` now derives
  Down/Move/Up/Leave plans from a revisioned fixed projection: a 256-node selection bitset, optional
  numeric hover slot, camera scalars, and fixed gesture/move pages. Projection construction rejects
  257 nodes, identifiers over 256 bytes, and aggregate identifier ownership over 16 KiB. It does not
  clone the host, document, history, selection sets, or fixture.
- `GraphHost` and `FlowHost` retain the projection beside their interaction revision. EngineCanvas
  borrows the planned selected/hover identifiers, validates their aggregate bytes, constructs the
  three exact flat actions, and calls the checked revision commit only from
  `publish_with_checked`. A stale commit publishes nothing. Flow applies the bounded move page to its
  layout and closes gesture history only after revision revalidation. Source fixtures were authored
  for direct-vs-plan click behavior, stale replacement, 256/257 and identifier overflow, and full
  action-queue/no-selection-mutation. They are not compiler- or runtime-verified.
- The first Board Move/Up/Leave plan boundary is present as an intermediate source checkpoint.
  `BoardPointerPlan` owns at most 256 numeric deltas, 16 KiB of identifiers, and one 16-KiB encoded
  event page in three flat boxed arrays; it is definitionally shallow on ordinary drop. Output JSON
  is pre-admitted with worst-case JSON escaping before construction, sealed into the fixed page,
  then its exact bytes are reserved in the flat action queue. Pan and node-drag plans revalidate the Board interaction revision before camera,
  node, gesture, or pointer-inside mutation. A retirement cursor clears one logical delta per close
  grant before releasing the empty flat shell. Authored fixtures cover direct-vs-plan pan, stale
  rejection, 257-delta rejection, one-delta close progress, and saturated pointer-up followed by an
  exact retry.
- Board is **not accepted** at this checkpoint. Selection, link, external-link, and brush interaction
  variants currently fail closed, and the legacy Board down/wheel/event path still owns recursive
  `Vec<Value>` rows. The shallow plan covers pan/drag/idle Move/Up/Leave only and does not yet preserve
  proximity-link and hover-event side effects exactly. A schema-first fixed typed event union with
  string spans, differential FIFO/coalescing fixtures, and one-event-per-close retirement must
  replace `events`, `board_pending_events`, `board_drain_into_buffer`, and
  `coalesce_board2d_events` for all reachable interaction variants before Board can be green.
- Graph is also still an intermediate boundary for special DAG controls: minimap, port/handle, and
  embedded widget hits fail closed before mutation. Rectangle selection currently uses bounded node
  centers and has not yet been proven differential against the legacy bounds/crossing/lasso rules.
  Those modes require typed plan variants rather than a fallback to the mutating host.
- Source-only evidence at this save is `rustfmt --edition 2021` over the five touched Rust files and
  path-scoped `git diff --check`, both completed successfully on 2026-08-23. Cargo, Nx, Wasm, and
  browser gates were not run by coordinator direction. The new APIs and authored fixtures remain
  compile-unverified, and Phase 3 remains RED for the previously listed chrome/assets/presenter/deep
  close and generic dispatcher/text boundaries.

### Fixed Board event-owner continuation

- `BoardHost.events` is no longer a recursive `Vec<serde_json::Value>`. It is a fixed
  `BoardEventQueue` with 256 FIFO slots, 256-KiB aggregate credits, an explicit `BoardEventKind`
  union, one independently allocated 16-KiB flat payload slab per occupied slot, and a fixed
  256-byte coalescing-key span. `BoardHost` is no longer cloneable. All event call sites now select
  an enum variant, including indirect/proximity link completion; arbitrary string event kinds are
  not accepted by the owner.
- EngineCanvas pending Board ownership is also a `BoardEventQueue`. Transfer checks downstream item
  and byte credits before popping exactly one host event. The typed coalescer reads the fixed FIFO
  without deserializing or cloning payload trees, preserves latest-camera and first-key-order/latest-
  node-move rules, suppresses moves before a drag-end, drops the exact transient kinds, and retains
  the existing flush-now taxonomy. Published pending ownership is swapped into a retirement owner;
  one flat event is released per subsequent retirement grant. The old recursive row coalescer and
  `drain_events_json` are now test-only differential surfaces.
- Authored fixtures fill all 256 typed slots, reject +1 without consuming the returned owner,
  verify exact FIFO, and require one event per close step plus terminal-empty. A differential fixture
  compares typed and legacy coalescing across camera replacement, repeated node IDs, transient
  preselect, flush-now selection, and drag-end suppression. These fixtures are source-authored and
  unrun.
- Board remains **RED**: the event storage/coalescer is shallow, but legacy selection/link/brush
  producers still construct temporary recursive `json!` payloads after semantic mutation and only
  then ask the queue to admit them. The queue retains the first overflow record and marks subsequent
  schema/credit failure, but that is diagnostic containment, not lossless pre-mutation reservation.
  Selection/link/brush pointer plan variants are still fail-closed. Acceptance still requires a
  fixed payload builder per typed variant, exact event batch reservation before mutation, and
  immutable plan/checked commit for every reachable pointer mode. World3D, Shell/media, and
  EngineCanvas drag/drop producer cutovers have not started.
- `rustfmt --edition 2021` and path-scoped `git diff --check` completed successfully after this
  continuation. Cargo, Nx, Wasm, and runtime gates remain unrun by coordinator direction.

### Flat selection reservation checkpoint

- The typed Board queue now has an owned reservation token containing the expected FIFO length and
  aggregate byte count. `publish_reserved` performs no allocation, payload traversal, or schema
  conversion and rejects a stale token. Selection, gestured selection, area preselection, area
  commit, controlled selection replacement, and background-clear build their exact JSON payload in
  a fixed 16-KiB writer and reserve the complete event before changing selection/preselection/chrome
  state. String escaping is performed directly into the slab; these paths no longer create a
  recursive `Value` payload.
- An authored fixture checks quotes, backslashes, and controls in the flat selection payload, fills
  all 256 queue slots, attempts a replacement, and verifies selection remains unchanged while the
  exact rejected event owner is retained. It is not executed pending the serialized Rust gate.
- This is still partial: link gesture sync requires an atomic two-event reservation token, brush
  preview/candidate/place needs direct fixed writers for its bounded handle/candidate pages, and
  preselect-cancel/link commit/delete/hover/camera legacy emitters still use the temporary `json!`
  bridge. Board cannot be marked source-green until those remaining producers are cut over and all
  reachable Move/Up/Leave variants use immutable plans.

### Board flat-producer and retained deletion checkpoint

- The production Board event bridge is removed. A path-scoped exhaustive scan now finds zero
  `push_event(` definitions/calls and zero combinations of the former NodeMove, NodeDragEnd,
  EdgeCreate/Delete, NodeDelete, or PreselectCancel variants with `json!`. The unused recursive brush
  preview/place helpers and placement-id mutator were removed as well. This is a source invariant,
  not compiler evidence.
- Link completion constructs its EdgeCreate plus optional Indirect/ProximityConnect records in the
  fixed writer and obtains one atomic two-event reservation before inserting the edge. Link gesture
  compatible/ring publication and brush preview/candidate publication use the same fixed four-slot
  reservation; emit keys advance only after exact item/byte admission. Brush placement constructs
  the fixed record before advancing the placement serial or consuming the preview. Camera, hover,
  selection, preselect cancellation, descriptor-created edges, node moves, and node-drag completion
  likewise use typed flat writers.
- Direct multi-node move now preflights the exact aggregate item/byte credits for every bounded
  NodeMove record before changing the first node. Saturation restores the retained DragNodes
  interaction without partial geometry mutation; after admission, each flat record is constructed
  before its matching node update. Descriptor edge creation preflights all new edge records before
  descriptor map mutation and returns the deterministic `EventCredits` fault on rejection.
- Selection deletion is derived into a revisioned shallow `BoardDeletePlan`: a 16-KiB identifier
  slab and 256 Copy kind/span entries preserve legacy selected-edge, incident-edge, handle/wire, and
  node order without recursive JSON/Vec event ownership. The final Select record is built before
  mutation, aggregate event credits are checked once, and commit revalidates the interaction
  revision. On saturation the exact plan plus flat Select owner remains in
  `pending_delete_operation`; host event draining retries it in FIFO order as credits return. A stale
  retained plan faults closed without changing maps. Authored fixtures cover full-queue retention
  and retry ordering, stale retirement, oversized identifiers, and unchanged state on rejection.
- Board is still **RED**. The retained deletion operation has only a single pending slot and its
  broader realm-close hookup is not yet proven. Deletion now claims exact queue credits once, then
  advances one typed mutation entry or one flat publication record per host-drain/close grant; no
  partially admitted output can steal its credits. `close_event_authority_step` drives a saturated
  retained operation, drains one flat overflow/batch/queue owner per grant, and exposes an exact
  terminal-empty witness. An interrupted-close fixture was authored. Individual BTree map removal
  can still release a scale-dependent Node/Edge/Handle/Wire payload, and the drain grant is not yet
  the shared `StepContext` deadline/fuel grant, so the deletion boundary remains RED. Direct
  node-move publication also still processes up to the admitted 256 entries in one call. Graph/map/selection hit traversal and
  `sync_selection_flags_to_objects` remain scale-dependent. Selection/link/brush Move/Up/Leave are
  not all represented in `BoardPointerPlan`; several production callers outside the renderer still
  reach the legacy mutating pointer methods. The typed event storage eliminates recursive payload
  creation, but does not by itself close those transaction and timing gaps.
- Source-only gates completed on 2026-08-23: `rustfmt --edition 2021` on the Board normal-port file,
  path-scoped `git diff --check`, the zero legacy-producer scans above, and a production-boundary
  `mem::forget` scan (zero hits). Cargo, Nx, Wasm, browser execution, and the authored Rust fixtures
  remain unrun under the coordinator's serialized-build instruction.
