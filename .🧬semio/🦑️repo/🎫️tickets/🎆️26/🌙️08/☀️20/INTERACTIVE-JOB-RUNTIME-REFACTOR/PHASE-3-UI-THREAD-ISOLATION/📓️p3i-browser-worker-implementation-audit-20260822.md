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

### Board retained pointer transaction checkpoint

- Every non-Idle `BoardPointerPlan` now requires `BoardHost.begin_pointer_commit`; the public direct
  commit seam accepts only Idle, and the former direct semantic implementation is retained solely as
  a `cfg(test)` differential. EngineCanvas therefore cannot route Pan, FinishPan, DragMove,
  FinishDrag, SelectionPreview/Commit, LinkMove/Finish/Retain, Hover, Brush, or LeaveIdle through the
  synchronous direct body.
- The retained owner advances one funded semantic item per `StepContext` grant. Multi-node drag,
  selection set replacement, old selection/preview/signature retirement, selection point/overlay
  construction, link dedupe-key/state publication, and brush candidate reconstruction are explicit
  phases. Link dedupe keys are copied into the pre-admitted fixed plan during derivation and are not
  recomputed from host maps during commit. Completion rechecks the exact interaction revision before
  publishing the already-sealed fixed event page; stale/faulted work publishes nothing, preserving
  the last valid rendered generation.
- Cancellation before semantic mutation retires one fixed-plan delta, selection owner, point, or
  string per grant. Once a semantic transaction has begun it finishes or faults under the same
  retained authority rather than exposing a partial event publication. Authored source fixtures
  cover zero fuel, one-delta progress, pre-mutation cancellation, selection/link/brush multi-turn
  progress, stale generation, and publication terminal-empty.
- This checkpoint does not make Board or Phase 3 green. Several plan-derivation helpers still perform
  admitted but multi-item hit/compatibility traversals, and the realm-wide BoardHost/EngineSurface
  close witness is not complete. The `render_chrome`, assets, icon/scene caches, GPU presenter, and
  broader dispatcher boundaries remain RED.

### Retained Board delete derivation and property audit checkpoint

- Production `delete_selection` no longer calls the whole-plan builder. It installs a revisioned
  `BoardDeletePlanningOperation` and advances the legacy deterministic selected-edge, node, handle,
  wire, incident-edge, and node order one map entry per `StepContext` grant. The old builder and its
  whole-tree property counter are `cfg(test)` differentials only.
- Property admission is now a reversible retained transaction. The planner temporarily owns exactly
  one entity property root, replaces one array/object child with a scalar sentinel, visits one
  value/key per grant, and restores the exact root before recording descriptor/property item and byte
  credits. Its fixed 256-frame stack and 16-KiB byte ceiling fault closed; overflow, cancellation, or
  stale generation unwinds one retained frame per grant and restores the entity before terminal
  fault/cancel. A hostile key is length-admitted before allocation, so the +1 path does not clone the
  rejected key.
- The final Select payload is appended one retained selection ID per turn into the fixed payload
  owner and transferred into `BoardOwnedEvent` by moving its slab, avoiding a terminal 16-KiB copy.
  Exact event queue credits are still claimed before the first map mutation. Authored fixtures cover
  hostile 257-value properties, +1 property-key bytes, one-node-per-turn progress, mid-audit cancel
  with exact root restoration, stale generation, full-queue retry, and terminal-empty.
- Source evidence only: `rustfmt --edition 2021` and path-scoped `git diff --check` are clean for the
  Board file. Cargo, Nx, Wasm, browser execution, and all authored Rust fixtures remain unrun by the
  coordinator's serialized-build direction. The ordinary BoardHost/EngineSurface destructor can
  still bypass nested scene/cache/app retirement, so Phase 3 remains **RED**.

### Board and EngineSurface retirement checkpoint

- The product `ENGINE_SURFACES` authority is no longer a string-keyed `HashMap`. It is a fixed
  256-slot registry with a 256-byte identity cap, per-slot generation tokens, reserve-before-owner
  construction, collision/saturation faulting, frozen closing slots, and stale-token rejection.
  Authored fixtures cover 256/+1 saturation, oversized identity rejection, slot ABA, close-time
  registration freeze, and exact nonopaque terminal publication.
- Board close is now an ordered retained transaction. It first cancels/drains the event and pointer
  authorities, detaches the cached world scene and icon scenes, then advances entity/property,
  selection, interaction, catalog, preview, string, and weight owners one `StepContext` grant at a
  time. `BoardHostRetirement` holds the host behind `ManuallyDrop` until
  `nonopaque_terminal_is_empty`; its unexpected release path cannot run the deep host destructor.
  The actual EngineSurface Board field is likewise `ManuallyDrop`-protected, and its close owner
  releases action claims, event pages, the Board retirement, Board sync fields, and scalar owners
  before shallow surface release. A close-during-populated-Board fixture and zero-fuel interruption
  fixture are authored.
- `IconPaintCache` is a fixed 256-slot generation/epoch registry instead of a string `HashMap`.
  Source and key bytes are capped before parse/build, insertion reserves before constructing a
  vector scene, invalidation advances an epoch without dropping old owners, and close visits one
  slot per grant. Vector scenes transfer to the shared opaque-scene owner; failed reservations are
  observable. Epoch/rebuild, oversized-source, and terminal-empty close fixtures are authored.
- Opaque `vello::Scene` destruction is not presented as cursorizable. Canvas owns a fixed
  1,024-slot generation-witnessed `OpaqueSceneRetirement` quarantine whose slots contain
  `ManuallyDrop<Scene>`. Engine packets, stale/current Board world caches, and icon vectors reserve a
  slot before detaching a scene. Saturation fails closed before transfer; retained count/fault are
  observable, and local capacity/+1 plus late-token fixtures are authored. This removes opaque scene
  destruction from the Board/surface callback, but the packet remains **RED** until the renderer or
  vello-removal lane proves an owned bounded release and measured <8 ms drop timing.
- The EngineSurface close witness deliberately faults and remains retained if a surface still owns a
  NodeGraph, Flow, Map, Editor, or their nonempty sync caches; those owners do not yet have honest
  deep retirement cursors. Presenter/GPU surfaces, chrome/assets, and the full realm-close hookup
  remain RED. The new Board-only terminal witness is therefore source progress, not Phase 3
  acceptance.
- Source-only evidence on 2026-08-23: `rustfmt --edition 2021` completed on the Canvas, directed
  Board, normal Board, and EngineCanvas files; path-scoped `git diff --check` is clean; scans find no
  `mem::forget`, old icon/engine string `HashMap`, or EngineCanvas packet call to
  `Scene::retirement_step`. Cargo, Nx, Wasm, browser execution, and all authored fixtures remain
  unrun by coordinator direction.

### World3D fixed-plan foundation

- `World3dState` now owns an interaction revision that advances on a changed scene projection. A
  dependency-neutral `WorldInteractionIntent` and `WorldInteractionPlan` establish the shared flat
  representation for move/button/drag/wheel/close work: a 16-KiB byte slab, 256 Copy action slots,
  generation plus revision witnesses, fixed numeric fields, one-action cursor, fault state, and
  one-slot close retirement. It contains no recursive `ActionDescriptor`, `Value`, or dynamic action
  vector.
- The wheel variant is the first end-to-end flat transaction. It derives the next camera without
  mutating the live orbit, stores the complete camera packet in the fixed plan, revalidates exact
  generation/revision, reserves and builds the bounded action before mutation, and changes the orbit
  only inside successful publication. Authored fixtures cover 16-KiB/+1 bytes, 256/+1 actions, stale
  no-mutation/no-output, one-fuel publication, FIFO output, and terminal-empty close.
- World3D remains **RED**. The renderer still calls the legacy recursive producers for pointer move,
  paint, pointer button/pick/marquee/gumball, drag, and wheel; the new wheel transaction is not wired
  into the debounced camera-claim authority. Hit/pick/marquee/paint traversals are not yet retained
  cursors, the remaining flat variants are not encoded, and glue/Shell consumer cutover has not
  occurred. `rustfmt` and path-scoped `git diff --check` are clean; Cargo/tests remain unrun.

### World3D retained ingress and ray-pick checkpoint

- World intent ingress now has a fixed 64-slot FIFO owner. Saturation and close return/retain the
  exact scalar intent, generation-gated retirement cannot consume a newer or older front entry, and
  close advances one slot per call with a terminal-empty witness. No descriptor, JSON value, or
  recursive payload is constructed at ingress. Authored fixtures cover 64/+1, exact FIFO retry,
  wrong-generation non-consumption, late work after close, and one-slot close progress.
- Camera drag (pan/orbit) and paint-stroke begin/end now share the wheel flat-plan publication seam.
  They compute bounded numeric/string claims without mutating the live world, revalidate revision
  and generation, reserve and finish the schema-first action, then mutate orbit or stroke state only
  inside successful publication. Zero fuel and stale generation leave both state and output
  unchanged.
- Mesh hit work has a retained `WorldRayPickCursor`. It advances exactly one triangle or one
  draw/instance boundary per `StepContext` grant, keeps only Copy indices/barycentrics/normal/point,
  faults malformed vertex indices, and converts a terminal hit into the fixed slab. Paint and
  surface-place publication are direct bounded-builder writes; interruption/staleness and close
  never own a recursive descriptor. Authored fixtures cover one-triangle progress, zero fuel,
  terminal paint publication, mid-pick staleness, and two-step retained-hit close.
- This remains source-only **RED** progress. The cursor still looks up the current mesh by the legacy
  string `HashMap` once per boundary; stable admitted mesh/draw registries are not yet installed.
  Component/vortex/reference/gumball and marquee/lasso traversals remain monolithic, the instance
  selection result schema is not yet encoded, and glue/Shell still call every legacy producer and
  batch descriptors in `Vec`. `WorldInteractionPlan.number_len` and the unimplemented flat kinds are
  deliberately not claimed live. `rustfmt --edition 2021` and path-scoped `git diff --check` passed;
  Cargo, Nx, Wasm, browser execution, and authored Rust fixtures remain unrun by coordinator order.

### World3D fixed-consumer cutover checkpoint

- Renderer glue and Shell no longer call the recursive `handle_world3d_*` descriptor producers or
  reconstruct a `world_actions: Vec`. Pointer button, move/drag, wheel, and the combined Shell seam
  admit Copy-only fixed intents; a maximum four-intent Shell batch preflights all slots and
  generations before publishing any member. The 64-slot queue has one explicit retained saturation
  owner, so the first full-queue event remains exact FIFO work instead of being consumed/dropped;
  a second overrun fails deterministically before state mutation.
- `AppFrameTransaction` now owns a stable surface cursor phase that advances one World3D authority
  step with the shared `StepContext`. Action-queue saturation is reported as `OutputBlocked` and
  yields back to the existing one-action input drain before retry, avoiding the producer/drain
  deadlock. Wheel no longer mutates orbit or installs a deferred recursive camera descriptor in its
  traversal phase. Legacy producer entry points are `cfg(test)` private differentials, and a
  compile-time source fixture rejects their names in glue/Shell and any public legacy export.
- Flat publication now covers camera wheel/pan/orbit, paint begin/end and paint-at, surface place,
  one-hit selection, and pointer hover/clear. Selection/hover target IDs use a new bounded-builder
  `string_joined` operation, which writes `surface/id` directly into the pre-admitted flat slab and
  never constructs an intermediate `String`; exact/+1 fixtures are authored at the shared action
  authority. Hover and selection revalidate revision/generation and publish before local hover
  mutation.
- This checkpoint remains **RED** and compile/runtime unverified. The live retained authority
  deliberately faults unsupported left-drag marquee, component/vortex/reference, gumball, brush,
  context-menu, and multi-selection variants rather than falling back to UI/Worker-inline recursive
  producers. The cursor still traverses legacy dynamic draw/mesh containers, and stable
  generation-keyed mesh/component/reference registries plus cursorized topology/marquee/gumball
  owners remain required. The old debounced camera deadline map and expired-descriptor seam also
  remain pending removal. `rustfmt`, include-path existence checks, path-scoped `git diff --check`,
  and source scans are clean; Cargo, Nx, Wasm, browser execution, and Rust fixtures remain unrun.

### World3D mesh/topology registry checkpoint

- Interactive mesh identity is now admitted to a deterministic fixed 256-slot registry with
  256-byte inline IDs, per-slot generations, replacement ABA invalidation, exact vertex/triangle
  counts, a 64-MiB topology byte ceiling, malformed triplet/UV rejection, and a sticky observable
  fault. Re-admitting an unchanged version returns the same token; a changed version advances the
  epoch before any new interactive job can resolve it. Authored fixtures cover 256/+1, oversized ID,
  malformed topology, unchanged identity, replacement ABA, and stale-token rejection.
- The ray cursor resolves its mesh token in a separate funded turn, validates generation, fixed ID,
  version, and topology counts before each triangle, records the exact token in its best hit, and
  revalidates it again before flat result construction. A missing render payload after an admitted
  token faults instead of silently skipping the draw. Mesh admission happens before publication to
  the interactive registry; if it fails, the existing renderer payload remains owned by the render
  map while the interactive authority faults closed.
- Token lookup is now retained: one deterministic collision slot is probed per funded turn, followed
  by one draw/instance boundary or triangle. The renderer payload map and draw vectors are still
  dynamic legacy owners and their removal/close is not cursorized, so this is not the final registry
  gate.
- A second deterministic fixed 1,024-slot registry now owns shallow Copy projections for instance,
  vortex, and reference interaction identities. Inline IDs are capped at 256 bytes; tokens contain
  slot generation plus scene revision; same-revision replacement and cross-revision slot reuse both
  invalidate ABA tokens. A retained rebuild cursor admits one instance, vortex, reference, or mesh
  collision probe per `StepContext` grant and refuses interaction until the complete revision witness
  is published. Authored fixtures cover capacity/+1, ID +1, same/cross-revision ABA, zero fuel,
  interruption, and three-kind terminal projection.
- Vortex and reference ray work now has a separate retained cursor that visits one registry slot per
  turn, retains only a Copy token/distance, revalidates at flat-plan construction, and retires that
  token in one close grant. Live vortex hover/select and mesh-hover reference fallback publish through
  schema-first bounded actions before local hover mutation. Component topology, marquee/lasso,
  gumball, brush, context-menu, fixed render-payload removal, and the legacy camera deadline seam
  remain **RED**. The reference projection still performs one capped-key lookup in the legacy aspect
  map during its funded build turn; that map and the upstream monolithic JSON/state synchronization
  require their own fixed/paged owners. No Cargo/Nx/Wasm/browser/Rust fixture execution was performed.

### World3D component topology cursor checkpoint

- Interactive mesh tokens now include exact edge count in addition to vertex/triangle counts. A
  retained component cursor first visits one fixed instance-registry slot, then exactly one admitted
  vertex, edge, or face per `StepContext` grant. Vertex projection, edge screen/ray comparison, and
  face ray/triangle work retain only Copy object tokens, numeric component IDs, and two scalar ranking
  values. Every resume revalidates scene revision, operation generation, instance ABA token, mesh ABA
  token, and the complete object-registry witness before touching topology.
- Component hover and click selection are wired through flat `setHover`/`worldPick` plans. Exact
  builder reservation and payload construction happen before hover mutation; action saturation keeps
  the plan and front intent live for retry. Close releases one retained best/current token per grant.
  Authored fixtures cover one-topology-item advancement, exact publication, object replacement ABA,
  stale no-publication, zero-budget behavior through the shared cursor contract, and interrupted close.
- This checkpoint is still source-only and **RED**. Component marquee/lasso and multi-pick are not yet
  represented by a fixed result-page authority; the vertex utility's legacy semantic preference for
  vortex selection remains explicit. Gumball, brush, context-menu, dynamic render-map/draw retirement,
  legacy aspect/state synchronization, and camera deadlines remain pending. Cargo, Nx, Wasm, browser,
  and authored fixture execution remain unrun by coordinator direction.

### World3D detached action draft and context-menu checkpoint

- The fixed action authority now exposes a detached claimed draft: queue item/byte credits and claim
  epoch are reserved first, then a shallow flat builder can add exactly one schema node per worker
  turn without borrowing `InputState`; only a complete draft becomes a `PreparedClaimedAction` that
  can consume the original claim. A stale/cancelled claim rejects later publication. This is the
  required generic substrate for cursorized marquee/multi-target payload construction; it does not
  itself make those consumers green. Authored fixtures cover incremental array construction,
  complete-only publication, claim cancellation, stale-epoch rejection, and shallow owner release.
- Right-click ownership moved into the World interaction authority as Copy press/drag state. A
  retained context cursor captures a capped inline target ID, scans one fixed object-registry slot per
  turn, revalidates the exact vortex/object/reference ABA token, and publishes a bounded
  `worldContextMenuAt` action. A right drag retires its up-event without context publication; close
  retires the press flag and one target token per funded grant. Authored fixtures cover exact target
  publication, revision/token validation, right-drag suppression, and interrupted token close.
- Phase 3 remains **RED**: the detached draft is not yet wired to marquee result-page production;
  gumball and brush still use legacy state paths; ordinary World/render state synchronization and
  retirement remain dynamic; the camera deadline map remains live. No Cargo/Nx/Wasm/browser/Rust
  fixture execution was performed.

### World3D fixed marquee ingress checkpoint

- Left-button selection gestures now enter a fixed 256-point Copy-only owner instead of mutating the
  legacy dynamic `marquee_points` vector. Down captures scene revision and start generation; each move
  admits exactly one point and retires exactly one intent; capacity +1 faults while retaining the
  current lossless intent. A click marks the gesture for one-point-per-grant retirement before the
  unchanged up-intent can retry through the component/instance cursor, preserving FIFO order and
  preventing a stale point owner from crossing generations.
- Close and scene-revision invalidation retire one point per `StepContext` grant and require the fixed
  array terminal witness before release. Authored fixtures cover exact 256/+1 admission, click/drag
  threshold, generation ordering, click retry without consuming the up-intent, interrupted retirement,
  and terminal-empty fixed storage.
- Drag marquee/multi-pick remains deliberately **RED**: the fixed gesture detects a non-click and
  fails closed with exact gesture/up-intent ownership until the nested entity/polygon/result-page
  cursor and detached action draft are connected. No legacy recursive selection fallback is invoked.

### World3D retained gumball checkpoint

- The fixed instance projection now carries selected state and translation scalars. A retained
  gumball cursor scans one registry slot per turn, admits at most 64 exact selected ABA tokens,
  derives the centroid without traversing legacy draw vectors, tests one of the twelve finite handles
  per turn, and then revalidates one selected token per turn before publishing a gesture owner. The
  selected-capacity +1 path faults with all admitted tokens retained for one-token close.
- A gumball gesture is Copy/token-only. Pointer moves retain the exact front intent while validating
  one selected token per turn, then perform one finite axis/plane/ring projection and retire that
  intent. Concurrent move replacement faults; scene revision or per-instance ABA invalidates without
  mutation. Gesture close first retires a pending update and then one selected token per grant.
  Authored fixtures cover 64/+1 selected tokens, one-token validation, replacement ABA, interrupted
  pending update, and terminal-empty close.
- This is still **RED**: renderer preview application has not consumed the fixed gesture, and pointer
  release deliberately faults closed with the gesture/up-intent retained until the multi-target
  detached action draft emits atomic translate/rotate/scale selection. Brush remains unconverted.

### World3D atomic result pages, gumball release, and brush snapshot checkpoint

- The action FIFO now supports an exact fixed batch of at most sixteen detached claims. Admission
  preflights aggregate item and byte credits plus sixteen fixed claim slots before publishing any
  ownership. Each claimed action is built independently, and `publish_prepared_claimed_batch`
  validates every epoch/action/byte witness before moving the entire FIFO-ordered page batch. Close
  can detach one prepared page and release one claim per grant. Authored fixtures cover three-page
  FIFO publication, batch capacity +1, no partial publication, and one-claim-per-turn cancellation.
- Non-click marquee owns sixteen fixed result pages of sixty-four Copy ABA tokens each. Object
  registry scanning advances one slot per grant; lasso winding advances one polygon edge per grant.
  Page publication reserves every exact page credit first, then writes one target schema field per
  grant into detached drafts. No page reaches the lossless action FIFO until all pages are complete.
  Result targets and gesture points retire one owner per close/publication grant. Authored fixtures
  cover 1,024/+1 page capacity, two-page FIFO shape, output saturation with exact result retention,
  close, and empty-page ownership.
- Marquee selection now consumes the stable admitted instance order and retains one mesh token at a
  time. Window selection projects one vertex per grant and requires every visible vertex to be
  enclosed. Rectangle crossing tests one triangle per grant; lasso crossing retains a triangle and
  advances one polygon edge per grant while accumulating vertex winding and edge intersection.
  Object and mesh ABA plus scene revision are revalidated before result admission. Authored
  differential fixtures compare the retained cursor with the legacy full-mesh selector for enclosed,
  crossing, disjoint, and degenerate geometry, plus a 65-instance multi-page draw-order case. The
  component-topology marquee variants now share that cursor: vertex, edge, and face topology advances
  exactly one component per grant; lasso edge/face candidates retain their polygon-edge cursor; active
  object filtering, mesh/object ABA, revision, and a fixed 64-component result cap are enforced.
  Component output merges one existing/result identifier per grant into a fixed array, reserves one
  exact `setSelection` claim, writes one schema node per grant, and publishes once. Authored fixtures
  compare vertex/edge/face output with the legacy selector, cover additive atomic publication, and
  cover capacity +1 retirement. Runtime execution remains **RED**.
- Gumball release now reserves one detached action before construction, emits one flat node or
  selected target per grant, revalidates each selected object token, publishes once complete, and
  retires selected tokens one per grant. Saturation retains the whole gesture for retry; stale/ABA
  construction transfers the draft and claim through cursorized close. Authored fixtures cover
  exact translate publication, output saturation, replacement ABA, interrupted draft/claim close,
  and terminal selected-byte accounting. Renderer culling consumes the fixed retained preview model
  and applies its Copy transform to one instance per traversal turn; the former whole-state preview
  mutation is no longer on that production render route. Preview runtime execution remains pending.
- Brush release now snapshots target/kind strings into one fixed byte slab in 256-byte turns, repeats
  the same chunk walk as a stability validation pass, and only then reserves and incrementally builds
  `addBrushObject`. Numeric origin/orientation/index and a bounded scalar/three-item scale union are
  Copy owners; recursive or oversized scale input fails closed before action ownership. Authored
  fixtures cover multi-chunk publication, same-length source replacement, interrupted draft/claim
  close, and terminal no-publication. Runtime fixture execution remains pending.
- The separate reference-aspect hash map was removed; aspect now derives from the already-owned image
  dimensions. The World3D camera deadline `HashMap`, expired-surface descriptor phase, and associated
  frame-build cloning were removed from the active renderer transaction; camera work must now enter
  through the retained World3D intent authority. This does not green reference/render ownership: the
  pixel map is still dynamic and image decode/insertion is monolithic. Full scene JSON cloning/parsing,
  dynamic draw/render maps, cursorized old-owner retirement, asset I/O/decode,
  presenter/GPU realization, and realm-wide close remain **RED**. `rustfmt` and scoped
  `git diff --check` were run successfully; Cargo, Nx, Wasm, browser, and all authored Rust fixtures
  remain unrun by coordinator instruction.

### World3D typed-snapshot ownership blocker

- The current cross-crate `World3dScene` contract still owns camera, mesh, instance, selection,
  topology, reference, environment, terrain, and interaction data exclusively as monolithic JSON
  `String` fields. It exposes neither a revision witness nor an owned typed/page cursor that can be
  retained after `render_world_3d` returns. A product-local cursor cannot safely retain the borrowed
  `&World3dScene`, while cloning those strings into the cursor repeats the exact whole-scene work this
  packet forbids.
- Consequently, `sync_world3d_state` still performs whole-string equality/cloning and synchronous
  `serde_json` parsing, clears/rebuilds dynamic mesh/draw maps, and can decode image/mesh payloads in
  one frame step. This remains a hard **RED**, not a report-only caveat. The clean next seam is an
  owned revisioned `World3dSnapshot` contract in the scene-schema crate with fixed typed pages and
  explicit input/item/byte credits, produced before renderer ownership transfer; the World3D state
  then consumes one field/item per `StepContext` grant and keeps the last complete generation. The
  P8-owned plugin component cannot be edited by this packet, so its old JSON-only producer must be
  coordinated rather than bypassed with a product-local compatibility parse.

### World3D fixed typed snapshot lease checkpoint

- The ui-scene contract now owns a fixed snapshot store: eight generation-keyed slots, at most 256
  pages per snapshot, 64 Copy typed items and 16 KiB of admitted semantic strings per page. A
  descriptor pre-admits exact aggregate page/item/byte credits. Page admission returns the exact
  rejected owner; sealing requires equality with every declared credit. `World3dSnapshotLease`
  carries slot epoch, revision, generation, and aggregate witnesses. Borrowed page access validates
  all ABA witnesses. Lease close and interrupted writer abort detach exactly one page per call and
  release the slot only after a terminal-empty witness. Authored ui-scene fixtures cover ABA page
  access, page/item +1, interrupted writer abort, and one-page close.
- `World3dScene` carries the Copy lease. The product consumer no longer reads any JSON field in its
  production `sync_world3d_state`: absence is unavailable/fail-closed, replacement while an apply is
  active is rejected, and the previous complete state remains published. `AppFrameTransaction`
  advances the retained snapshot cursor before World3D input work. It reads at most one item per
  grant; Camera accepts only ten numeric typed fields and stages the orbit until the complete lease
  commits. A malformed/string-backed Camera page faults without parsing or fallback. Authored
  typed-vs-current camera construction compares the typed result with the old parser only inside a
  test-only legacy function.
- This boundary remains **RED** beyond Camera. Mesh/topology/instance/selection and extension item
  schemas are admitted by the page authority but not yet applied into fixed staging registries.
  Existing draw, mesh, texture-pixel, URL, terrain, and decoded-resource owners are still dynamic and
  lack complete one-owner retirement. The coordinated P8 producer currently copies already-built JSON
  strings into pages and builds/hashes all pages synchronously; it marks writer abort but does not pump
  it. That producer is explicitly rejected as typed/bounded evidence until it constructs direct typed
  pages through a retained producer job and drains abort one page per scheduler turn.

### World3D dynamic-owner retirement checkpoint

- A retained `World3dDynamicRetirement` can now detach the live mesh map, draw list, reference pixel
  map, and paint texture map from `World3dState` in O(1). It advances one nested owner per
  `StepContext` grant: one of the nine mesh buffers, one draw instance, one pixel allocation/key, or
  one exhausted collection shell. The terminal witness requires every detached iterator/current
  owner and every corresponding live-state collection to be empty. An authored fixture interrupts at
  zero fuel, retires a mesh with multiple buffers, 32 draw instances, and two pixel allocations, and
  verifies terminal empty.
- This is retirement progress, not fixed live ownership acceptance. Insertions still target dynamic
  `HashMap`/`Vec` containers; identifiers and pixel allocations are not yet admitted by generation-
  keyed fixed registries; ordinary `World3dState` destruction is not yet structurally forced through
  this cursor; and one pixel/mesh buffer deallocation lacks a proven byte-time ceiling. These remain
  **RED** along with the monolithic asset collector/fetch/decode/apply path.

### World3D fixed live dynamic-owner checkpoint

- The active World3D product state no longer stores meshes, draws, reference pixels, or paint pixels
  in a `HashMap`/`Vec` field. Mesh and pixel authorities use fixed 256-slot registries with a checked
  256-byte identifier ceiling, per-slot epochs, exact rejected-owner return, token-validated removal,
  replacement ABA invalidation, and a closing admission state. Draws use a fixed 256-slot ordered
  registry with the same checked identifier and closing contract. Direct production pixel insertion
  and whole draw-list assignment were removed; a permanent source fixture rejects those field and
  mutation shapes.
- Replaced, removed, saturated, and closing owners transfer into a fixed 1,024-owner process
  quarantine. Quarantine saturation is observable and returns the exact owner. A state-local blocked
  owner retains the first exact owner that could not enter quarantine, faults the interaction/frame
  authority, and is retried before any registry close work. Replacement rollback restores the prior
  generation and retains the unpublishable replacement. Authored fixtures cover registry and ID
  capacity +1, replacement ABA, exact quarantine saturation handback, interrupted close, and late
  insertion rejection.
- `World3dDynamicRetirement` now freezes all four registries and transfers at most one opaque owner to
  quarantine per `StepContext` grant. `RuntimeMailbox::close_world3d_dynamic_step` walks the admitted
  surface order and pumps exactly one World3D retirement grant before `OsHostRetirement` may release
  the runtime owner. Its terminal witness requires the retirement cursor, blocked owner, and all four
  fixed registries to be empty. Opaque mesh/draw/pixel allocation release remains quarantined and is
  deliberately not described as timed or completed.
- This checkpoint is still source-only and **RED**. The temporary grouped
  `HashMap<String, Vec<Instance3d>>` rebuild was renamed test-only and removed from the production
  render route; production now keeps the last complete draw registry rather than rebuilding from
  the rejected JSON/`parsed_instances` path. Mesh versions have moved to the same fixed generation-keyed registry and
  retire one bounded identifier/value owner per close grant. Production `Drop` for both registry
  types and `World3dState` now requires the dynamic terminal-empty witness, but ordinary
  `AppRuntime` destruction is not yet definitionally shallow for every non-dynamic field; the static opaque
  quarantine has no renderer/GPU-owned bounded release lane; and the surface map itself still owns a
  dynamic `HashMap`. Asset collection/fetch/decode/apply, presenter realization, GPU submission and
  realm-wide terminal close remain **RED**. `rustfmt` on the three changed Rust sources, whole-tree
  `git diff --check`, and the production bypass `rg` scan completed successfully. No Cargo, Nx, Wasm,
  browser, or authored Rust fixture was run under the coordinator's serialized-build restriction.

### Fixed renderer surface authority checkpoint

- `AdmittedSurfaceMap<T>` no longer exposes or stores a `HashMap`. It owns 256 fixed value slots,
  fixed epochs, and a fixed insertion-order slot page. Lookup, value-only iteration, stable O(1)
  `id_at`, replacement, and removal are explicit methods; there is no `Deref`/`DerefMut` structural
  bypass. Replacement and slot reuse increment the slot epoch, and token lookup fails closed on an
  ABA witness. Existing invariant fixtures now cover order, 256/+1, replacement/removal/clear, token
  ABA, and a permanent no-HashMap/no-structural-deref scan.
- This remains **RED** for complete surface ownership. Exact rejected/replaced owner return and
  one-owner close were added in the subsequent checkpoint below, but the realm-wide consumer of those
  owners is not yet wired. The World3D runtime close pump drains its fixed dynamic registries before
  the containing runtime releases, while every other Shell/AppRuntime owner still requires the
  realm-wide close protocol. These fixtures remain authored but unrun.

- Surface insertion now returns a generation token or the exact rejected `{id,value,fault}` owner.
  One rejected owner and one replaced owner have dedicated retained slots; producers stop before
  constructing another value while either is pending. `begin_close`/`close_step` detaches exactly one
  rejected, replaced, or live surface owner and `terminal_is_empty` witnesses every slot. NodeGraph,
  TiledMap, and Board producers use exact insertion and retain first rejection; stable identities
  update only Copy bounds, while owned-string replacement faults closed pending a field-specific
  publication job. This is not yet wired to a realm-wide owner disposer, so production map `Drop`
  is not claimed terminal-shallow.

### Retained fixed-credit draw rebuild checkpoint

- The active World3D state now owns a revisioned/generation-keyed `WorldDrawRebuildCursor` rather
  than admitting a production JSON/`parsed_instances` rebuild. Its descriptor reserves the exact
  draw, instance, and aggregate byte credits before an instance owner is constructed. It admits at
  most 256 draw drafts, 4,096 instances in aggregate, and 16 MiB aggregate bytes; mesh/instance IDs
  are capped at 256 bytes. The public draft writers borrow identifiers, validate the exact remaining
  credits first, and construct owned strings/instances only after admission, so saturation leaves the
  producer's source owner untouched for FIFO retry rather than accepting and returning a rejected
  recursive owner.
  Draft instance storage is a fixed-length `MaybeUninit` page with an initialized-prefix witness, so
  admission never reallocates or recopies preceding instances.
- Worker stepping materializes one source instance into one staged draw per `StepContext` grant,
  moves one completed draw into the fixed draw registry on a separate grant, revalidates revision and
  generation throughout, and atomically swaps the staged registry only after every exact descriptor
  credit seals. The previous complete registry moves to a retained displacement slot and transfers
  one draw per later grant into the observable opaque quarantine. Stale/cancel close returns one
  output instance, draft instance, draft shell, or staged draw per grant; production `Drop` asserts
  terminal-empty for drafts and the rebuild cursor. The frame transaction pumps this cursor before
  snapshot/input work, and realm close pumps it before live registry retirement.
- Authored fixtures cover mixed two-group order, multi-instance FIFO, no partial publication,
  byte +1 and ID +1 exact handback, stale revision, zero-fuel interruption, resumed close, draw
  instance cap +1, registry ABA, and terminal witnesses. The old grouped rebuild remains only under
  `cfg(test)` as a differential fixture path. These fixtures are not executed yet.
- Remaining **RED**: the upstream typed snapshot producer does not yet fill draw drafts; the
  `SceneDraw3d` staged output still uses an exactly pre-sized `Vec` during one-draw materialization
  and draw-field allocation timing is unmeasured; normal terrain/overlay/render packet `Vec`s remain
  dynamic; surface rejected/replaced owners are not yet pumped by the realm disposer; and opaque
  quarantine release, asset I/O/decode, presenter/GPU realization, and complete AppRuntime close are
  unresolved.

### Draw writer and surface-iteration tightening

- The fixed surface authority now exposes only audited `keys`, shared `IntoIterator`, `iter`, and
  value-only iteration. There is still no structural `Deref`. The former synchronous `clear` call
  now marks the authority closing and records an observable fault so live owners remain retained for
  the realm close pump; it never walks or drops the fixed slots in the caller turn. This deliberately
  makes the still-monolithic chrome rebuild fail closed until its old-frame retirement cursor is
  connected.
- Draw draft writers now take borrowed mesh/instance identifiers and scalar instance fields. They
  check identifier, item, and aggregate byte credits before `to_owned` or `Instance3d`
  construction. Descriptor admission now enforces 4,096 instances globally rather than 4,096 per
  each of 256 drafts. `rustfmt` and whole-tree `git diff --check` completed successfully after this
  change. Cargo, Nx, Wasm, browser, and authored Rust fixtures remain unrun by explicit coordinator
  instruction.
- Asset work is still **RED** at this save. `collect_pending_*` allocates/clones whole vectors,
  `poll_pending_assets` buffers every fetched result, image/GLB/map decode and apply are monolithic,
  and browser `array_buffer`/native file reads have no fixed aggregate byte-credit job. The next
  authority must replace those operations; merely measuring the outer async suspension is not an
  accepted bound.

- As an initial asset-owner reduction, the async poller no longer retains three additional
  `fetched_*` vectors containing every completed GLB, map tile, and UI image response. Each fetched
  response is now handed to its current exact consumer before the next request starts. This removes
  the previous multi-response byte amplification, but it is not the fixed I/O lane: request
  discovery still clones whole pending collections, each response is still a monolithic `Vec<u8>`,
  and decode/apply remain indivisible. This change therefore does not change the **RED** verdict.

### Fixed asset request and streamed-page ownership foundation

- `WorldAssetIoAuthority` is now a production-owned field of each `World3dState`. It has 64 fixed
  generation-keyed request slots and a single exact 16 MiB aggregate response-byte authority.
  Request reservation validates the 2 KiB URL ceiling, item credits, per-request byte credits, and
  aggregate byte credits before copying the borrowed URL. A token carries slot epoch, generation,
  and source revision; failed reservation does not advance the state's asset generation.
- Fetch results enter as independently owned pages of at most 16 KiB, with at most 1,024 pages under
  the same pre-reserved aggregate byte claim. The request owner supports exact transfer to the I/O
  suspension, return, seal, generation/revision-validated decode checkout, and terminal finish.
  Cancellation/close retains an in-flight claim until its external owner returns, then retires one
  page index or the bounded URL owner per close grant. Production `Drop` for both request owners and
  the authority asserts the exact terminal-empty witness. World3D realm retirement now closes this
  authority before draw/mesh/pixel owners.
- Authored, unrun fixtures cover request item +1, response byte +1, response page +1 exact return,
  a two-page partial stream, one-page-per-close interruption, stale generation checkout, decode
  resume across two pages, terminal finish, and full authority close. `rustfmt` and `git diff
  --check` are green; Cargo/Nx/Wasm/browser remain unrun by instruction.
- This foundation is not yet an accepted asset lane. Existing producers still write unbounded
  `pending_*` collections, request discovery still uses whole `collect_pending_* -> Vec`, browser
  fetch still materializes `Response.array_buffer`, native file fetch still performs an ungoverned
  whole read, and GLB/image/map decoders still require a contiguous buffer and publish in one call.
  No old collector or decoder has been relabeled as bounded; Phase 3/5 remains **RED** until those
  producers and consumers are cut over to the fixed authority and an incremental decoder/output
  cursor exists.

### Streamed GLB transport cutover checkpoint

- The compiled renderer path is `♾️infinite/🌍️world/🦀️component.rs`, mounted as
  `infinite_world::world`. Its live URL-backed mesh producer now reserves directly in the fixed
  `WorldAssetIoAuthority`; Shell/glue no longer collect or clone a `PendingGlbFetch` vector and the
  compiled collector/fetch-loop symbols were removed. The stale compiled fixture now checks the
  same exact take, close, return, and terminal-retirement ownership route rather than reconstructing
  a temporary surface `HashMap`.
- The dedicated browser Worker polls one exact request owner, requires a finite bounded
  `Content-Length`, reserves aggregate response credits before reading the body, and uses a BYOB
  `ReadableStream` reader with a fresh 16 KiB page owner. One read/page is transferred per
  macrotask; zero, oversized, overflowing, and short pages fail closed. No `Response.arrayBuffer`
  remains on this Worker path. Abort and Worker close return the exact fetch owner to its source
  authority or retain it in the Worker's bounded close state. Rust rejects a page-length violation
  before cloning borrowed Wasm bytes and requires received bytes to equal reserved bytes before a
  response can seal.
- The adjacent `♾️infinite/🦀️component.rs` is not an unreachable duplicate: the infinite
  crate mounts it as private `component` and publicly reexports it, while the World3D renderer uses
  the separate `world` module. It was therefore neither edited nor deleted. A direct path census
  found one stale terrain doc reference, but the crate module wiring itself is authoritative.
- Source-only gates at this checkpoint: `rustfmt --check` passed on the compiled World3D,
  browser-worker, glue, Shell, and Scenes sources; scoped `git diff --check` passed. The Browser
  transport fixture was updated for the BYOB page contract but was not run. Cargo, Nx, Wasm, and
  browser runtime gates remain unrun by instruction.
- This is still **RED**. Map-tile and UI-image producers retain dynamic global Vec/HashMap queues;
  `poll_pending_assets` still drains those queues and invokes monolithic fetch/decode/apply. Native
  `ReadBytes` still aggregates the whole file even though its I/O job reads 32 KiB chunks. Completed
  GLB owners are not yet consumed by a retained GLB header/JSON/accessor/mesh publication cursor;
  reference-image/terrain producers are not fully cut over; and realm/presenter/GPU close is not
  terminal-complete.

### Completed-response, typed-map, and native streaming checkpoint

- The fixed asset authority now exposes a one-slot completed-response cursor rather than scanning
  all 64 claims in one frame turn. A retained renderer probe owns one exact sealed response, reads
  one 16 KiB page per governed frame grant, copies only its fixed 64-byte signature prefix, checks
  the sealed byte witness, and rewinds valid input for the format decoder. Malformed input enters
  cursorized page retirement and exact source-authority handback. Valid input intentionally remains
  backpressured at the decoder boundary; no legacy whole-buffer decoder is called from the probe.
- The compiled tiled-map request producer no longer parses a JSON array or constructs a temporary
  tile `Vec`. `VisibleTileCursor` is a fixed shallow typed cursor, while `MapTileRequestCursor`
  retains the map revision and bounded URL-template witnesses and advances one visible tile per
  frame grant. Fixed authority saturation keeps the current tile unadvanced for exact FIFO retry.
  Raster/vector upload is still monolithic and therefore remains **RED**.
- Native file reads use one `NativeIoRequest::ReadPage` of at most 16 KiB per I/O continuation.
  Native HTTP uses `SocketHttpTransport` for plain HTTP and an owned ureq/TLS adapter for HTTPS.
  Both implement the same `AsyncHttpTransport`/`HttpBody` contract and return at most one 16 KiB
  body page per pull. The ureq adapter no longer calls `read_to_end` on successful or error-status
  responses; it admits URL/header/body limits before starting, retains one response reader, checks
  cancellation before start and every pull, and releases the reader at EOF or body drop. Unknown or
  chunked body length is admitted incrementally against the fixed 16 MiB response claim rather than
  falling back to whole-body buffering. The external ureq/TLS implementation remains the explicit
  Phase 9 dependency-removal residual; secure asset URL functionality is preserved for Phase 3.
- Authored, unrun fixtures cover completed-slot advancement, split GLB signature validation,
  malformed close, typed-map FIFO equivalence, native partial/eof/+1 pages, plain HTTP partial
  Content-Length and chunked bodies, and owned HTTPS reader partial/error/terminal/backpressure and
  cancel-before-pull behavior. `rustfmt --check` and scoped `git diff --check` passed. Cargo, Nx,
  Wasm, browser, and network gates remain unrun by explicit instruction.
- Phase 3/5 remains **RED**: GLB accessor/mesh construction, PNG/JPEG/SVG rasterization, MVT token
  decode, terrain decode, typed atomic apply, cache retirement, presenter/GPU realization, and the
  realm-wide terminal close witness are not yet governed retained jobs. The 64-byte probe is only
  an admission stage and is not counted as a decoder.

### Retained GLB schema, BIN, and paged-mesh cutover checkpoint

- The completed-response probe now owns a fixed 1,024-entry response-page offset index and advances
  from container scanning into a retained GLB JSON tokenizer/schema cursor. One call consumes at
  most 256 input bytes or one token. Fixed schema pages admit at most 512 accessors, buffer views,
  primitives, meshes, nodes, scene roots, and child edges. Accessor spans, component/shape pairing,
  stride, normalized flags, triangle mode, default-scene references, node cycles/depth, and exact
  instantiated output byte credits are checked before output page allocation.
- The BIN materializer keeps the sealed response owner in place and reads components through the
  fixed page index; it never flattens or rescans the response. It allocates one 16 KiB result page
  per grant, then advances one position, explicit/generated normal, UV, triangle, or normal
  normalization item per grant. The source-only implementation covers u8/u16/u32 indices;
  triangles, strips, and fans; FLOAT positions; FLOAT or normalized signed byte/short normals;
  FLOAT or normalized unsigned byte/short UVs; node TRS/matrix composition; inverse-transpose
  normal transforms; generated normals; cap/overflow faults; and one-page-per-grant result close.
  A retained differential fixture compares transformed positions, normals, and strip indices with
  the existing glTF-based `mesh_from_glb` oracle. These fixtures are authored but unrun.
- Publication is deliberately still **RED**. The repository's public `Mesh3d` owns nine contiguous
  `Vec` fields. Flattening fixed GLB pages into that type would reintroduce an ungoverned allocation,
  copy, and final destructor. The required boundary is now an atomic schema-first fixed-page
  `Mesh3dLease` authority from producer birth through hit-testing, render preparation, GPU upload,
  and retirement. A source census is saved in `mesh3d-census-20260823.txt`: 267 direct construction
  or field-consumer references across ui-scene and OS. Both the private/reexported
  `♾️infinite/🦀️component.rs` and the compiled World3D
  `♾️infinite/🌍️world/🦀️component.rs` are mounted and contain live consumers. The P8-owned plugin
  `component.rs` has no `Mesh3d` constructor or direct field consumer; its
  `world3d_scene_extended` symbol remains a typed scene-snapshot producer and is not being edited.
- Scoped `rustfmt --config skip_children` and `git diff --check` passed before the most recent BIN
  fixture additions and are rerun at each save. No Cargo, Nx, Wasm, browser, or network gate has
  run. PNG/JPEG/MVT/SVG semantics, paged mesh publication/borrow/ABA/close, cache retirement,
  presenter/GPU realization, and realm-wide close all remain explicit **RED** gates.

### Direct paged-mesh publication checkpoint

- The GLB materializer no longer terminates at a private result-page owner. Its typed positions,
  normals, UVs, and indices are born in the shared fixed `Mesh3d` page authority and the sealed
  `Mesh3dLease` now transfers directly from the retained renderer probe into the exact World
  surface. No intermediate contiguous mesh or `Vec` conversion is used on this route.
- Canonical World mesh publication is now a three-authority transaction. Mesh storage, version
  storage, and interaction topology each create a fixed-slot/epoch plan before the first mutation.
  Closing, ID, item, byte, stale-epoch, and topology admission faults return the exact input lease.
  A successful replacement transfers the prior generation into the existing page-stepped close
  owner. The decoder also revalidates the source revision against the current World interaction
  revision before publication.
- Renderer backpressure retains ownership: capacity/closing restores the same lease into the GLB
  probe for a later turn; a stale revision restores it and then enters the probe's page-stepped
  mesh/response close while recording a deterministic frame fault. Only a committed publication
  detaches the lease before response retirement.
- The exact remaining census is refreshed in `mesh3d-census-20260823.txt`. It is deliberately not
  zero: ui-scene retains five old-type/test references, canonical World retains 21, and the second
  mounted/reexported World component retains 21. Canonical placeholder, terrain, legacy scene,
  paint-bake, and test producers plus all corresponding code in the second mounted component must
  still move to schema-first retained jobs before the Vec-backed type can be deleted.
- Scoped parse/format evidence at this save: `rustfmt --edition 2021 --config skip_children=true`
  passed for canonical World; `rustfmt --edition 2024 --config skip_children=true` passed for
  renderer glue; scoped `git diff --check` passed. These are source checks only. Cargo, Nx, Wasm,
  browser, network, and root lint remain unrun by instruction.
- Phase 3/5 remains **RED**. The internal one-slot blocked-mesh fallback is not a complete producer
  saturation authority, the second mounted component and old constructors are not migrated, GPU
  upload still traverses a whole lease synchronously, and PNG/JPEG/MVT/SVG output, cache disposal,
  presenter/GPU close, and realm terminal-empty remain incomplete. The World3D typed snapshot
  producer is not claimed and no JSON fallback was added.
- The canonical World consumer no longer accepts inline mesh buffers from legacy scene JSON and no
  longer exports or calls its monolithic `mesh_from_glb -> MeshData -> Mesh3d` apply route. URL
  meshes now stay backpressured until the retained renderer decoder publishes a paged lease. The
  old GLB integration test that exercised the deleted contiguous path was removed; the retained
  decoder's page/geometry differential fixture remains the intended replacement, still unrun.

### Retained GPU mesh and presentation staging checkpoint

- `MeshGpuTable` no longer maps a full vertex/index buffer and loops over an entire lease. A
  retained cursor owns the exact key, version, lease, schema, destination buffers, and independent
  vertex/index positions. Each presentation turn reads and writes one typed vertex or one index;
  another generation is rejected while the cursor owns the fixed upload authority. The completed
  buffers become visible in the mesh table only after both cursors reach terminal.
- Prepared presentation is now a retained phase machine: gate admission, one EngineCanvas packet,
  one eviction, one upload step, final render, fullscreen, and cursor directive are distinct turns.
  Mesh uploads remain in the Uploads phase across as many turns as their admitted items require.
  The old all-upload loop, direct `submit_prepared` call, full-lease vertex/index loops, and
  all-EngineCanvas-packets loop are absent. While a presentation is pending, OsHost does not take a
  second frame, native scheduling requests another resource-ready turn, and the browser Worker
  reports `request_frame` until completion.
- A permanent source fixture now rejects reintroduction of direct submit, all-upload, and whole
  schema vertex/index loops. It is authored but unrun. Scoped rustfmt passed for draw/gpu with
  edition 2021 and renderer glue/winit/browser/EngineCanvas with edition 2024; scoped diff-check
  passed. No Cargo, Nx, Wasm, browser, network, or root-lint gate ran.
- This boundary remains **RED**: EngineCanvas `realize_one` and final wgpu render/submit are opaque
  one-turn operations; atlas/raster uploads and dynamic cache eviction are monolithic; upload and
  pending-presentation failure/realm-close retirement are not terminally cursorized; the mounted
  root `infinite/component.rs`, schema-first placeholder/terrain builders, and legacy Mesh3d tests
  remain unmigrated.

### Active-upload close and mounted-root placeholder checkpoint

- The retained mesh upload cursor now has an exact abort path. Key bytes retire one scalar per
  turn, each optional destination buffer is explicitly destroyed and detached in its own turn, and
  only the empty scalar lease/schema/version shell is released. `OsHostRetirement` pumps this
  witness before beginning World mesh-authority retirement, preventing a live upload from reading a
  lease after its paged owner begins closing. A permanent source fixture asserts this order.
- This is deliberately narrower than presenter acceptance. Pending prepared packets, the completed
  GPU mesh table, atlas/raster resources, EngineCanvas realization, and final render/queue submit do
  not yet have exact deep-close/timing witnesses. The platform-bound render/submit step remains
  **RED** and is not described as bounded.
- The mounted `♾️infinite/🦀️component.rs` was a second complete World implementation but had no
  production consumer of its bespoke API; every renderer path imports `infinite_world::world`.
  The mounted root now reexports that canonical authority, reducing its compiled Mesh3d/direct-field
  census from 21/80 conservative hits to zero without a Vec adapter or fallback.
- Canonical placeholder production no longer constructs `WorldMeshBuffers -> Mesh3d`. A retained
  analytic writer covers box, plane, cylinder, cone, and subdivision-1 icosphere geometry. It owns
  an epoch/generation/revision write token, allocates one 16 KiB authority page or writes one typed
  position/normal/index per call, seals before publication, and aborts one page per close call.
  An authored differential fixture compares its complete geometry with the former generator and
  covers interrupted terminal close; it has not run because Cargo remains prohibited.
- Source gates run at this save: scoped rustfmt for draw/gpu (edition 2021), renderer glue/host
  (edition 2024), and both mounted/canonical World files (edition 2021); scoped `git diff --check`;
  exact `rg` census. All passed. Cargo, Nx, Wasm, browser, network, and root lint were not run.
- Phase 3/5 remains **RED**. Terrain-band and face-overlay producers still construct contiguous
  Mesh3d values; legacy tests remain Vec-backed; placeholder progress currently depends on a later
  render invalidation instead of an explicit scheduler-ready signal; and semantic PNG/JPEG/MVT/SVG
  decode, cache retirement, full presenter/GPU close, realm terminal-empty, and browser runtime
  evidence remain outstanding. The World3d snapshot producer is still unclaimed.

### Retained terrain-band output checkpoint

- Production terrain-band output no longer scans a tile into temporary positions/normals/indices
  vectors or constructs `Mesh3d`. A retained tile cursor first counts one triangle per turn for the
  current band, then allocates one paged-authority page per turn and writes one position, normal, or
  index item per turn. Each nonempty band seals and publishes independently under ten reserved mesh
  generations and a terrain-revision witness; the tile becomes built only after every band and the
  flat source owner reach terminal.
- Style replacement, invisible tiles, malformed indices/floats, paged-authority rejection,
  publication saturation, cancellation, and realm close converge on the same write-token/lease
  abort cursor. It retires one authority page, one flat Copy-vector allocation, or one surface-id
  scalar per call. An authored differential fixture compares low/high bands against the former test
  oracle and covers malformed admission plus interrupted terminal close. It is unrun.
- The input side is not relabeled bounded: `terrain_tile_mesh_json` and serde still materialize the
  whole decoded `TerrainTileMeshPayload` before this cursor owns it. The required upstream typed
  terrain page producer remains **RED**, as does explicit scheduler-ready signalling for cursor
  progress without another render invalidation.
- Current compiled census: mounted root 0 Mesh3d symbols; canonical World 17 total symbols, with
  one remaining production constructor in the face-overlay path and the rest in cfg(test)/oracle
  code. Stale prose references to `infinite_world::render_world_3d`/root `World3dState` were updated
  to the canonical `infinite_world::world` spelling.
- Scoped rustfmt (edition/style 2021) and scoped `git diff --check` passed for canonical/mounted
  World plus the two comment-only renderer files. No Cargo, Nx, Wasm, browser, network, or root lint
  ran. Phase 3/5 remains **RED** for face-overlay production, terrain input materialization, legacy
  tests/type validation, explicit progress scheduling, semantic image/map decoders, cache close,
  final presenter/GPU/realm close, and browser evidence.

### Retained face-overlay output checkpoint

- The last non-test production `Mesh3d::from_buffers` route is removed. Face overlays now use a
  retained `WorldFaceOverlayMeshCursor` with fixed three-category state, an exact mesh write
  token/lease owner, interaction-revision and draw-generation witnesses, and first-seen bucket
  ordering matching the prior traversal. One turn advances one draw/instance/triangle boundary,
  one preview/selected identifier comparison, one authority page, one position/normal, one index,
  one seal/publication transition, or one stale-key removal. Removal advances only after the
  registry accepts it; a busy close owner leaves the exact bucket/key stage at the FIFO front.
- The writer preserves the former preview > hovered > selected color precedence, hovered full
  offset versus half offset, and double-sided `[0,1,2,0,2,1]` winding. It never constructs a
  bucket `Vec` or whole overlay mesh. Sealed leases enter the observed-slot publication authority;
  a rejected lease and bounded key return to the same cursor's publish stage for exact FIFO retry.
  Stale revisions, realm close, write faults,
  and interruption converge on the page-stepped token/lease abort cursor. Dynamic realm close now
  drains this cursor before the World mesh registry.
- Authored, unrun fixtures compare positions, colors, offset, winding, stale-generation
  retirement, empty-bucket removal order, identifier-cap rejection, and terminal-empty close. A
  permanent source fixture rejects `Mesh3d::from_buffers`, `FaceOverlayBucket`, selection
  `HashSet` construction, and contiguous float buckets in the production face route.
- Source gates run at this save: `rustfmt --edition 2021 --config style_edition=2021 --check` and
  scoped `git diff --check` for canonical World passed. The exact face-route negative scan passed.
  Cargo, Nx, Wasm, browser, network, and root lint were not run by instruction.
- The non-test production constructor census is zero, but the public obsolete Vec-backed
  `Mesh3d` type and cfg(test) oracles/fixtures remain. They are not called migrated or compile
  validated. The next serialized packet must replace those fixtures with paged leases, delete the
  old type/import, and run the Rust type gate. Phase 3/5 remains **RED** for that test/type gate,
  explicit wake scheduling for retained builders, presenter-witnessed normal replacement close
  (the face cursor retains its retry lease rather than closing a generation that an active upload
  may still read), atomic visibility for a multi-bucket overlay-family replacement, typed terrain
  input, PNG/JPEG/MVT/SVG semantics, cache close, final presenter/GPU/realm close, and real
  Wasm/browser evidence.

### Scoped paged-mesh audit formatting repair

- Repaired the sole blocker in
  `📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️os_host.rs`: edition-2021 rustfmt now
  orders `default_intent_exchange` before `AppKernelSeam`. The working-tree diff against the
  staged source contains exactly that one import-order line and no semantic change.
- Reran the Terra audit cohort exactly at source scope. Edition-2021 rustfmt passed for the mounted
  root, canonical World, ui-scene math, draw, GPU, and `os_host`; edition-2024 rustfmt passed for
  renderer glue. Mounted-root forbidden census remained 0, face-route forbidden census remained
  0, and the exact legacy `Mesh3d::from_buffers` census remained 12. The authority constants remain
  256 slots, 16 KiB pages, 1,024 pages/16 MiB per owner, and 4,096 process pages. The source order
  still drains `close_active_upload_step` and its terminal witness before
  `close_world3d_dynamic_step`.
- Whole working, staged, and HEAD `git diff --check`, plus the exact scoped diff check, all passed.
  Cargo, Nx, Wasm, browser, network, and root lint were not run by instruction. This repairs only
  the audit's isolated formatting rejection; all independently listed Phase 3 residuals remain
  **RED**.

### Atomic mesh-family visibility and explicit cursor wake checkpoint

- The obsolete public Vec-backed mesh type and its conversions are removed. Production and tests
  consume only the generation/revision/epoch-witnessed paged lease. Differential inputs are named
  `LegacyMeshOracleData`, are confined to `cfg(test)`, and must publish through the real paged
  writer before any shared consumer observes them. The exact framework-wide old-type/constructor
  spelling census is zero; `mesh3d-census-20260823.txt` records the superseding command and result.
- Face overlay publication is now family-atomic. Each fixed category is sealed under a staging key
  containing the candidate generation. The visible generation plus fixed color table swaps only
  when the retained cursor reports the whole family complete. Partial and stale superseding builds
  preserve the last complete family. Realm retirement clears colors and generation witnesses one
  owner per grant before the mesh registry terminal witness.
- Terrain bands remain invisible until the retained ten-band builder publishes its complete tile
  marker. This closes partial-family visibility, but normal style replacement still retires the old
  family too early and therefore remains RED for last-valid/presenter-witnessed replacement.
- Retained placeholder, terrain, and face cursors now request progress through a fixed
  generation/armed `WorldCursorWake`, not polling. Duplicate requests coalesce until one exact
  take. The bit is retained through frame build, render preparation, and presentation completion;
  native scheduling records RESOURCE_READY and the browser Worker consumes the host bit exactly
  once into `request_frame`. A resumed cursor creates a fresh ABA-distinguishable generation.
- Authored source fixtures cover partial face invisibility, whole-family atomic publication, stale
  supersede preserving last-valid, complete-marker terrain visibility, duplicate wake coalescing,
  exact wake handoff, and rearming. They were not executed because Cargo remains prohibited.
- Executed source-only gates all passed: framework-wide legacy spelling negative scan; obsolete
  face-removal and tuple-completion negative scans; required fixture/wake/terminal positive scans;
  `rustfmt --edition 2024 --check` for ui-scene math, canonical World, winit app, frame job, OsHost,
  and browser worker; renderer glue parse through rustfmt stdout; scoped `git diff --check`.
  Cargo, Nx, Wasm, browser, network, and root lint were not run.
- Phase 3/5 remains **RED**. Normal face replacement intentionally blocks while one retired
  generation lacks a presenter acknowledgment and bounded registry-lease retirement. Terrain
  last-valid replacement is also absent. Typed terrain input, dynamic collection/frame-owner
  retirement, semantic PNG/JPEG/MVT/SVG jobs, full pending-packet/GPU-table/atlas/raster/cache
  close, opaque render/submit timing, realm terminal-empty, and real Wasm/browser evidence remain.

### Retained generation-qualified cursor-wake remediation

- This checkpoint repairs the wake-specific rejection recorded in
  `📓️sol-independent-p3-atomic-family-wake-legacy-zero-audit-2026-08-23.md` and supersedes
  the preceding checkpoint's per-frame `WorldCursorWake`/host-bit claim. `RuntimeMailboxInner` now
  owns the one durable `WorldCursorWakeAuthority`. Every `World3dBuildContext` receives only a
  shallow clone of that authority, so consecutive frames observe the same retained generation
  state; there is no production `World3dBuildContext::default()` path.
- `WorldCursorWakeToken` remains a non-Copy, generation-qualified owner through
  `AppFrameAfterChrome`, `AppFrameBuild`, `AppFramePreparation`, `AppFramePresentation`,
  `AppPresentStep::Complete`, and the `OsHost` platform directive. Presentation acknowledges only
  the exact pending generation. Duplicate and stale tokens cannot acknowledge a rearmed
  generation. An already-pending platform directive is replaced only by a strictly newer token,
  which prevents an older completed frame from erasing a newer wake.
- Native completion retains the accepted token, invalidates `RESOURCE_READY`, requests the window
  redraw, and takes that token exactly once at the platform edge. The browser Worker also takes the
  token exactly once; only then is it projected to the wire `request_frame: bool`. Other browser
  deadline/text/presentation reasons remain independent boolean sources and never recreate or
  acknowledge the World token.
- Close is explicit at every retained seam. `OsHostRetirement` clears one pending platform token
  before presentation retirement; `AppPresenter` clears one token in a pending prepared frame;
  frame preparation clears its token on a separate close turn; the runtime authority then retires
  pending generation, current generation, and acknowledged generation on separate grants before
  its terminal-empty witness succeeds.
- The wake fixture now uses the same authority across consecutive build contexts, coalesces a
  128-request storm onto one generation, proves exact one-shot acknowledgement, rearm to a newer
  generation, stale/duplicate ABA rejection, and scalar close. A permanent live-path source
  fixture rejects per-frame authority recreation, typed-token-to-boolean erasure, missing ACK,
  lost native handoff, missing newest-generation coalescing, missing close take, and a browser
  projection that observes without consuming. The face-family fixture now constructs preview,
  hovered, and selected categories as three nonempty buckets, requires all three staging leases
  before visibility, preserves the previous complete generation across stale interruption, and
  proves the stale partial generation remains retained by its generation-qualified registry.
- Source-only gates executed at this save: edition-2024 `rustfmt --check --config
  skip_children=true` passed for ui-scene math, canonical World, renderer glue, OsHost, winit app,
  browser Worker, and frame job; the exact framework-wide legacy `Mesh3d` spelling scan returned
  zero matches; `LegacyMeshOracleData` remained 17 test-only references; production build-context
  recreation was absent; internal renderer wake fields remained typed and `request_frame: bool`
  appeared only in the final browser wire output; required wake/face paths were present; scoped and
  whole working, staged, and `HEAD` `git diff --check` all passed. The Rust fixtures were authored
  and inspected but not executed. Cargo, Nx, Wasm, browser, network, and root lint were not run by
  instruction.
- This is a wake-remediation source boundary, not Phase 3 acceptance. The independently recorded
  normal face/terrain replacement retirement, typed terrain input, dynamic/frame-owner close,
  semantic image/map decode, pending packet/GPU table/atlas/raster/cache retirement, opaque
  render/submit timing, full realm terminal-empty, and real native/Wasm/browser timing gates remain
  **RED**.

### Exact typed wake-handoff evidence repair

- This source-only evidence repair addresses the isolated rejection in
  `📓️sol-independent-p3-atomic-family-wake-legacy-zero-reaudit-2026-08-23.md`; it does not
  change the coherent live wake authority or platform behavior. The permissive `>= 4` predicate is
  removed. The permanent source fixture now requires exactly five internal typed-token handoffs,
  each in its named region: `AppFrameBuild` at renderer glue line 6922,
  `AppFrameAfterChrome` at 6930, `AppFramePresentation` at 7508,
  `AppFramePreparation` at 7531, and `AppPresentStep::Complete` at 7619. It also requires the exact
  `AppPresenter.pending -> AppPresentCursor.frame -> AppFramePresentation` container path.
- Host ownership is independently enumerated: exactly one typed field in `OsHost` and one in
  `OsHostRetirement`, with exactly two host-token fields repository-locally. The predicate rejects
  internal `cursor_wake: bool`, `Option<bool>`, `request_frame: bool`, and boolean host-token
  fields. The final browser wire DTO remains outside this glue/host internal census.
- The former single-occurrence token-erasure mutation is replaced by five region-qualified
  mutations. Each removes exactly one named typed handoff and must fail the predicate. Separate
  mutations erase all five, inject an extra internal boolean channel, erase the presenter frame
  owner, erase the AppPresenter pending owner, erase either host field, recreate the per-frame
  authority, skip exact acknowledgement, lose native transfer, disable newest-generation
  coalescing, skip close take, or observe the browser token without consuming it. The exact global
  count also rejects an unenumerated sixth typed field.
- Source gates run at this save all passed: edition-2024 rustfmt/parser for ui-scene math, canonical
  World, renderer glue, OsHost, winit app, browser Worker, and frame job; root interactivity
  verifier self-test in clean DENY mode; exact five-field glue census; exact two-field host census;
  no internal boolean wake fields; no production build-context recreation; framework-wide legacy
  `Mesh3d` census zero; and 17 test-only `LegacyMeshOracleData` references. Scoped and whole
  working, staged, and `HEAD` whitespace checks all passed. Rust fixtures,
  Cargo, Nx, Wasm, browser, network, root lint, and runtime timing were not run.
- Phase 3/5 remains **RED** only for the separately recorded production/runtime residuals. This
  packet makes no claim about them and starts no next residual.

### Presenter-acknowledged prepared-packet and fixed mesh-GPU retirement checkpoint

- Prepared packets are now single owned values rather than `Arc` snapshots. The worker receiver,
  preparation job, presentation cursor, gate candidate, last-valid owner, replacement owner, and
  close cursor all transfer the same packet. The packet exposes a retained retirement cursor that
  releases at most one admitted pixel page, action/draw owner, key scalar, eviction, or metadata
  item per grant. Atlas/raster allocation release remains explicitly outside this bounded claim.
- The presenter uses a non-Clone `PreparedPresenterWitness { sequence, scene_revision,
  preview_generation }`. A candidate is staged under that exact witness before platform render;
  render borrows only the matching staged packet; and only a later exact one-shot acknowledgement
  swaps it into `last_valid`. Missing, stale, duplicate, superseding, submission-fault, and close
  paths keep the old packet authoritative and return the exact staged candidate to the retained
  abort/retirement path. Frame production itself is invoked through `admit_next_frame`, so no
  second frame is constructed before the capacity-one presenter accepts ownership.
- Normal replacement is source-ordered as admission -> one engine/upload step -> stage -> opaque
  platform render/present -> exact ACK -> old-packet/GPU retirement. A failed admission, engine
  realization, upload, or submit enters `AppPresentPhase::Aborted`; its active upload buffers and
  packet/frame owners are then retired through the same bounded cursor. The last-valid packet is
  not replaced by any aborted candidate.
- `MeshGpuTable` no longer uses a `HashMap` or whole-table `retain`. It is a fixed 256-slot registry
  with 256-byte keys, exact key/version/lease upload identity, capacity-plus-one owner handback,
  one-slot scanning, one-buffer retirement, ABA-safe version lookup, and terminal close. After ACK,
  eviction first scans one prepared upload per grant into a fixed 256-version keep set, then retires
  one table slot/buffer per grant. This preserves every mesh version referenced by the newly
  acknowledged packet instead of deleting a same-key replacement.
- `OsHostRetirement` now requires prepared cursor/gate/active-upload/fixed-table terminal witnesses
  before `RuntimeMailbox::close_world3d_dynamic_step`. Authored Rust fixtures cover exact one-shot
  ACK, old-visible-before-ACK, missing/stale/duplicate ACK, exact candidate handback, capacity +1,
  ABA replacement, acknowledged-version preservation, page-stepped close, and source mutations
  for stage/ACK/abort/Arc/HashMap/unbounded-cap/keep-version/admission/close bypasses. Those Rust
  fixtures were not compiled or executed because Cargo remained prohibited.
- Source-only gates run at this save: edition-2024 scoped rustfmt/parser passed; a 28-rule Bun
  structural predicate passed 28/28; the interactivity verifier self-test and plain run both
  reported clean DENY mode; scoped working/staged/HEAD whitespace checks passed. No Cargo, Nx,
  Wasm, browser, network, or root lint ran.
- This is a coherent source checkpoint, not Phase 3/5 acceptance. Opaque
  `wgpu::Buffer::destroy`/destructor timing, `EngineCanvasPresenter::realize_one`,
  `GpuContext::render_prepared`/queue submit/surface present, atlas/raster/cache ownership, the
  remaining generic `GpuContext`/presenter/realm graph close, typed terrain input, semantic
  PNG/JPEG/MVT/SVG work, and native/Wasm/browser timing evidence remain **RED**. Rust type/runtime
  correctness is also unverified until the serialized build gate runs.

### Independent presenter-freshness and Rust-2021 repair

- This bounded repair addresses only the two source blockers in
  `📓️sol-independent-p3-presenter-gpu-retirement-audit-2026-08-23.md`. All live let-chains in the
  six audited renderer files were rewritten as Rust-2021 nested conditionals, and the complete
  seven-file touched cohort (including the browser Worker constructor) now parses and formats with
  manifest-selected edition/style 2021. No crate edition was changed.
- `RuntimeMailboxInner` now owns a retained `RuntimePresentationAuthority`, independent of any
  prepared candidate. Accepted and returned runtime completions advance its scene revision; the
  native frame admission edge records the authoritative input generation before starting a frame.
  `AppFrameTransaction` captures the authority witness before candidate construction and writes
  that scene revision/input generation into `PreparedRenderInput`. A stale input generation faults
  before candidate construction.
- `AppPresenter` receives a clone of the same retained authority at both native and dedicated
  browser-Worker construction sites. `BeginGpu` reads current authority and passes it to
  `begin_prepared`/`begin_prepared_offscreen`; it no longer derives expected revision/generation
  from the packet. Immediately before exact presenter acknowledgement it reads authority again.
  A mismatch aborts the pending candidate into the retained retirement owner, leaves the previous
  last-valid packet authoritative, and never publishes the stale candidate.
- The runtime-free source fixture changes the authority while retaining an unchanged candidate,
  then changes only the candidate while retaining an unchanged authority, and checks stale/current
  input generations separately. The permanent presenter contract now requires exactly one
  pre-candidate witness capture, exactly two independent presenter authority reads, both native and
  offscreen BeginGpu calls, the ACK freshness comparison, exactly two scene-revision advances, and
  the native input observation. It explicitly denies packet-derived expected assignments. Four new
  mutations independently replace BeginGpu authority with packet values, replace the frame witness
  with candidate/job values, remove scene-revision advancement, and remove input-generation
  observation. Together with the existing thirteen ownership/ACK/table/close mutations, the
  permanent Rust fixture now contains seventeen mutations. The Rust fixture was inspected but not
  executed because builds were prohibited.
- Executed source gates at this save:
  - `rustfmt --edition 2021 --check --config style_edition=2021,skip_children=true` passed for
    `prepared.rs`, `draw.rs`, `gpu.rs`, product `os_host.rs`, product `glue.rs`, `winit_app.rs`, and
    `browser_worker.rs`; edition-2021 `--emit stdout` parser checks passed for each file;
  - the exact six-file Rust-2024 let-chain scan and live packet-derived expected-value scan returned
    zero;
  - the independently reconstructed presenter/GPU predicate passed **28/28**;
  - the focused independent-authority mutation probe denied **4/4** mutations;
  - scoped and whole working, staged, and `HEAD` `git diff --check` passed.
- Both interactivity verifier commands were rerun and are **RED outside this packet** on the
  concurrently edited P1t DB history route:
  `🛢️db/⚙️engine/🦀️component.rs:0: retained history route contains while`. The finding is not in
  any audited P3 renderer file and was not modified here. Cargo, Nx, Wasm, browser, network, root
  lint, and runtime timing were not run.
- This source repair closes the independent audit's edition and tautological-freshness findings
  only. Phase 3/5 remains **RED** for the previously enumerated opaque buffer destruction,
  realization/render/submit/present timing, atlas/raster/cache ownership, full realm teardown,
  typed terrain/semantic codec work, Rust type validation, and real native/Wasm/browser evidence.

### Fixed staged raster GPU table and presenter-retirement checkpoint

- This bounded source packet replaces only the raster GPU cache/upload-consumer authority. The
  former dynamic `HashMap<String, RasterTexture>` is gone from the production table. Its
  replacement is a deterministic fixed registry with 256 slots, an eight-probe maximum, and
  256-byte fixed UTF-8 keys. Each entry carries scene revision, preview generation, and an exact
  per-frame operation identity. A vacant-only insertion API returns the complete rejected owner;
  no release-mode replacement/drop depends on a debug assertion.
- Raster upload admission checks the exact `width * height * 4` byte claim and requires one row to
  fit in the 16 KiB page budget. Each worker grant writes at most one group of whole rows totaling
  at most 16 KiB. An interrupted upload retains its texture, key, dimensions, revision,
  generation, operation, and row cursor. Capacity, probe, stale, duplicate, close, and occupied
  failures leave the live entry unchanged and return or retain the exact candidate owner.
- New textures publish into a staged registry. Rendering can borrow a staged entry only while the
  matching `(scene_revision, preview_generation)` presenter witness is active; otherwise lookup
  returns the previous live entry. Exact presenter ACK moves at most one staged slot per grant into
  the live registry, then retires the displaced bind group, texture, key, and scalars one owner
  action per grant. Abort/device/surface/realm-close paths retire the staged candidate through the
  same cursor and preserve the old live texture. Terminal close requires live, staged, upload,
  retirement, candidate, and presenting owners all to be empty.
- Prepared raster uploads now pass the prepared packet revision/generation plus a checked upload
  index as operation identity. EngineCanvas texture realization uses a disjoint checked operation
  range. If staging rejects that texture, the exact `wgpu::Texture` is returned and restored to
  the owning surface together with its newly reconstructed view; it is not discarded. Renderer
  presentation begins the raster witness before platform render, commits it only after the exact
  prepared presenter ACK, aborts it with the rejected packet witness, and drives raster terminal
  close before the World-owner terminal witness.
- Permanent Rust source fixtures require the fixed capacity/probe/page constants, generation and
  operation checks, vacant-only insertion, staged begin-before-render ordering, commit/abort
  retirement, exact EngineCanvas handback, and close-before-terminal ordering. Ten mutations erase
  or weaken those independent rules. They were authored and inspected but not compiled or run
  because builds remain prohibited. Independently executed Bun source predicates passed **11/11**
  and denied **10/10** mutations, including capacity, page limit, generation, operation, vacant
  ownership, begin, commit, abort, close, and returned-texture erasures.
- Executed source gates passed: edition-2021 `rustfmt --check` for framework `draw.rs` and `gpu.rs`;
  edition-2021 skip-children rustfmt/parser for product renderer `glue.rs`; edition-2021 parser for
  EngineCanvas; production negative census for the dynamic raster map and replacement insertion;
  and scoped working, staged, and `HEAD` `git diff --check`. The interactivity verifier self-test
  had already passed **245/245** at the preceding save. Its plain repository run remained the
  expected broader Phase 1/8 RED and is not claimed as a packet gate. Cargo, Nx, Wasm, browser,
  network, root lint, and runtime timing were not run.
- Phase 3/5 remains **RED**. `PreparedRenderUpload::{GlyphAtlas, IconAtlas, Raster}` still owns
  contiguous pixel `Vec`s, so producer-side paging and old-packet pixel retirement are not closed
  by this packet. `IconAtlas` still owns a pixel `Vec` plus a string `HashMap`; `FontAtlas` retains
  dynamic glyph maps and pixel buffers; atlas writes remain whole-buffer operations. EngineCanvas
  still owns a dynamic surface map and performs Vello realization/rendering. `GpuContext` still
  has scene/frame/depth/pipeline/atlas resource owners without a realm-wide exact terminal close.
  Platform queue submit, swapchain present, opaque GPU destruction timing, generic draw-list
  construction/traversal, semantic codecs/input, and real native/Wasm/browser timing evidence are
  also explicitly outside this source checkpoint and remain RED.

### Independent raster-audit remediation: reservation, operation authority, and exact view ownership

- This bounded repair addresses only the blockers in
  `📓️sol-independent-p3-raster-gpu-checkpoint-audit-2026-08-23.md`. The previous checkpoint's
  claim that EngineCanvas had an exact view handback was incorrect: it cloned the view and did GPU
  work before table admission. `RasterTextureTable` now exposes a non-cloneable
  `RasterTextureAdmission` obtained before `Renderer::new`, target creation/replacement, and Vello
  render. Admission owns an exact staged slot/nonce, a fixed 256-byte key, dimensions, the full
  witness, one item credit, and checked `width * height * 4` bytes. Per-owner bytes are limited to
  16 MiB and simultaneous live/staged/reserved/retiring bytes to 256 MiB; simultaneous owners are
  also limited to the fixed 256-item authority. Saturation or `+1` fails before GPU owner creation.
- Staging now consumes the original `wgpu::Texture` and `wgpu::TextureView` by value. EngineCanvas
  swaps both exact field owners for a fresh replacement pair without cloning either published
  owner. Every explicit stage rejection returns the same admission, texture, and view; EngineCanvas
  cancels the exact reservation and restores those returned owners. Prepared pixel uploads use the
  same reservation seam before texture/view/bind-group creation. An invalid rejected token is
  retained in a fixed reservation-retirement owner rather than deep-dropped.
- `RuntimeMailboxInner` now owns a separate monotonic `RuntimeRasterOperationAuthority`.
  `AppPresenter` mints a raster operation from the independently captured live presentation
  revision/input generation, not from the prepared candidate. The full
  `{ scene_revision, preview_generation, operation }` witness is retained through EngineCanvas,
  prepared raster pages, staging, submit, and presenter ACK. EngineCanvas and the table compare
  candidate and independent expected witnesses before realization/staging; submit and ACK compare
  the still-live authority again. Stale/duplicate/ABA operations fail closed. Exact success, abort,
  pending-close, device/surface close, and realm close release that authority once.
- Raster entry retirement now retains and advances bind group, exact view, texture, inline key,
  scene revision, preview generation, operation, width, height, and byte credit one owner/scalar per
  grant. Candidate and presenting witnesses use three individually optional scalar fields and are
  cleared one scalar per grant only after the fixed registry scan. An outstanding reservation has
  its own fixed per-field close cursor. The terminal witness includes live, staged, upload,
  reservation, reservation retirement, entry retirement, candidate, and presenting authorities.
- Focused source fixtures cover the exact 256-byte key and `+1`, exact 16 MiB pixel claim and `+1`,
  operation-only stale/duplicate/ABA comparisons, independent operation occupancy/release/device
  close, and one-scalar witness retirement. The permanent source contract now includes
  EngineCanvas itself and requires pre-realization reservation, by-value view transfer/return,
  independent RuntimeMailbox authority, independent expected-value minting, submit/ACK freshness,
  fixed key/item/table byte credits, and close ordering. Its exact matrix contains **13** mutations
  and the reconstructed probe denied **13/13**; the requirement probe passed **11/11**.
- Executed source-only gates:
  - edition-2021 `rustfmt --check --config skip_children=true` passed for framework `draw.rs`,
    `gpu.rs`, framework WGPU `glue.rs`, product renderer `glue.rs`, `browser_worker.rs`, and the
    complete EngineCanvas file; rustfmt parsing therefore passed for all six scoped sources;
  - the reconstructed raster predicate passed **11/11**, its permanent-contract predicate was
    true, and all **13/13** exact mutations were denied;
  - `bun ./📜️script.ts verify interactivity --self-test --format json` and the plain verifier both
    exited zero in clean DENY mode with the one recorded allowlisted blocking-bridge finding;
  - the dynamic raster-map, borrowed/cloned-view, old pair-witness, old `u32` operation, and
    production `mem::forget` negative scans were clean; scoped and whole `git diff --check` passed.
- Cargo, Nx, Wasm, browser, network, root lint, and runtime timing were not run. Rust type checking
  and actual GPU identity/runtime behavior remain unverified until the serialized build/browser
  gate. Phase 3/5 remains **RED** for the same out-of-packet atlas/icon/glyph, prepared-pixel
  producer, dynamic EngineCanvas surface, Vello/render/submit/present timing, full GpuContext/realm
  close, codec/input, opaque GPU destruction, and real native/Wasm/browser evidence residuals.

### Raster checkpoint independent-rejection repair: permanent generation, retained cancel/close, and allocation claim

- This source-only repair addresses exactly P3-R1 through P3-R3 from
  `📓️sol-independent-p3-raster-gpu-checkpoint-remediation-audit-2026-08-23.md`. It changes
  framework WGPU `draw.rs`/`gpu.rs` and product WGPU `📦️glue.rs`; it does not alter browser,
  Wasm, renderer-world, atlas, codec, or runtime behavior outside the raster authority.
- `RuntimeRasterOperationAuthority` now pairs its monotonic atomic with a permanent atomic
  exhaustion bit. Ordinary values advance through compare-exchange; exactly one caller may mint
  `u64::MAX`, after which every begin fails permanently even after the exact MAX witness is
  released. The fixture exercises MAX-1, MAX, occupied MAX, release, exhausted-next, and reopen,
  and asserts both the counter and exhausted state cannot return to operation 1. The permanent
  predicate rejects restoration of unchecked `fetch_add`.
- Matching EngineCanvas cancellation no longer clears the table reservation. It atomically moves
  both the exact table reservation and the consumed admission token into
  `RasterTextureReservationCloseCursor`. Each fixed key retires as one root and each witness,
  dimension, byte, slot, and nonce field retires as one scalar. The runtime-free authority fixture
  observes exactly two roots and sixteen scalars, never more than one released unit per grant, and
  a terminal-empty shell. Retry admission advances this retained cursor one grant at a time and
  remains fail-closed until it is empty.
- Interrupted prepared upload now moves the whole upload owner in O(1) into
  `RasterTextureUploadCloseCursor`; it does not clear a populated reservation, clear the upload,
  or synthesize a three-scalar presenting witness in that opportunity. The cursor separately
  retains and drains the allocation claim, bind group, exact view, texture, key, dimensions,
  bytes, row, admission, and witness scalars. `RasterTextureCleanupStep` reports exact
  `Pending { released_roots, released_scalars }`, `Blocked`, or `Complete`; ownership-only
  transfers truthfully report zero releases. Empty-shell assertions run in release builds before
  cursor owners are removed. Fixtures drive both before-first-page and mid-page interruption to
  terminal empty and assert no step releases more than one root or scalar.
- `RasterTextureStageClaim` captures the full reserved key/dimensions/bytes/slot/nonce, candidate
  witness, and staged-slot generation. `claim_stage_before_gpu_allocation` validates missing or
  stale reservation, nonce ABA, changed candidate, and occupied/invalid staged slot immediately
  before both prepared texture allocation and EngineCanvas bind-group allocation. Publication
  revalidates that exact retained claim before vacant-only staged insertion. Runtime-free fixtures
  discriminate valid, missing-reservation, nonce-ABA, candidate-change, and occupied-slot cases;
  the ordering mutation moves both claim calls away from the preallocation seam and is denied.
- Preserved source invariants: 256 fixed slots/eight probes, 256-byte keys, 16 MiB per item,
  256 MiB aggregate, 16 KiB upload opportunities, pre-realization reservation, by-value exact
  `Texture` plus `TextureView` stage handback, independent presenter freshness, vacant-only staged
  publication, last-valid live ownership, and raster-before-World close ordering.
- Executed non-build evidence on the final source:
  - canonical edition-2021 `rustfmt` write/check and `rustfmt --emit stdout` parsing passed for
    framework `draw.rs`, framework `gpu.rs`, and product WGPU `📦️glue.rs`;
  - the reconstructed complete raster predicate was true and denied **17/17** mutations; a second
    rejection-focused probe denied **4/4** operation-wrap, wholesale-cancel, upload-drop, and
    allocation-order mutations;
  - `bun ./📜️script.ts verify interactivity --self-test --format json` and the plain
    verifier both exited zero in clean DENY mode, retaining only the recorded allowlisted
    blocking-bridge finding;
  - production-only negative scans found zero unchecked raster operation `fetch_add`, wholesale
    raster reservation clears, interrupted-close presenting-witness fabrication, dynamic raster
    map, cloned EngineCanvas published view, `u32` raster operation, or `mem::forget`;
  - scoped and whole working/staged/`HEAD` `git diff --check` all passed.
- This packet is **audit-ready, not accepted**. Cargo, Nx, Wasm, browser, runtime/GPU timing,
  network, and root lint were not run by instruction, so Rust type correctness and device identity
  remain unproved. Phase 3/5 remains **RED** for atlas/icon/glyph and producer paging, dynamic
  EngineCanvas surfaces, Vello/render/submit/present timing, opaque GPU destruction, full
  GpuContext/realm teardown, semantic codecs/input, and native/Wasm/browser evidence.

### Raster allocation-boundary and publication-owner re-audit repair

- This source-only repair addresses exactly P3-R3 and P3-R4 from
  `📓️sol-independent-p3-raster-gpu-checkpoint-remediation-reaudit-2026-08-23.md`. The owned source
  scope is framework WGPU `draw.rs`/`gpu.rs`/`📦️glue.rs`, product
  `EngineCanvas/🧊️component.rs`, product WGPU `📦️glue.rs`, and this report. No browser-worker,
  World, atlas, codec, Wasm, or platform source changed in this packet.
- `RasterTextureUploadCursor` is installed in the fixed raster table before any prepared GPU
  allocation and owns texture, view, bind group, admission, upload row, and exact allocation claim
  separately. A fresh call through the complete
  reservation/key/witness/dimensions/bytes/staged-index/nonce/candidate/staged-vacancy validator
  now immediately precedes prepared texture creation, view creation, and bind-group creation.
  No other GPU or renderer allocation occurs between each matching validation and allocation.
- The external EngineCanvas path performs the same complete validation immediately before each
  target texture, target view, `Renderer::new`, resize replacement texture/view, final replacement
  texture/view, and table bind-group allocation. Partial target/view owners created before a later
  failed validation move into the table's reserved upload-close slot rather than unwinding on the
  presenter stack.
- Both prepared and EngineCanvas publication faults now move the exact admission, retained claim,
  texture, view, and newly allocated bind group into `RasterTextureUploadCloseCursor`. Early
  external bind-group validation faults return the exact admission, texture, and view through the
  non-cloneable `RasterTextureStageFault::Returned` owner so EngineCanvas can cancel the exact
  reservation and restore both surface roots. Once a bind group exists, publication faults instead
  return the observable `Retained` state and all three GPU owners remain in the table close cursor.
  `GpuContext::close_raster_upload_step` remains the public resumable cleanup seam. It first retires
  the table reservation, then advances the exact fault/upload cursor; owner transfers report zero
  releases and each later grant releases at most one bind group, view, texture, key, or scalar. The
  prepared
  `map_err(|(fault, _, _)| fault)` adapter and the external bind-group owner-erasure are absent.
- The runtime-free tuple fixture now independently changes key, all three witness components,
  width, height, byte credit, staged index, nonce, candidate, and staged occupancy. Every mismatch
  rejects before allocation. The permanent structural predicate also proves the table-owned
  preallocation cursor, every named allocation edge, both external fault-close branches, all five
  GPU roots/authorities in publication close, and the absence of the owner-erasing adapter. Its
  matrix increased from **17** to **38** mutations, including independent erasure of every named
  allocation validation, each complete-tuple component, the fault cursor, and each returned GPU
  root/handback. A faithful Bun reconstruction passed the baseline and denied **38/38**.
- Executed non-build evidence on the final source:
  - canonical edition-2021 `rustfmt` write/check and `rustfmt --emit stdout` parsing passed for all
    five owned Rust source files;
  - `bun ./📜️script.ts verify interactivity --self-test --format json` and the plain verifier both
    exited zero in DENY mode with only the recorded allowlisted test-only blocking bridge;
  - the focused scan found zero prepared/external
    `map_err(|(fault, _, _)| fault)` adapters, cloned EngineCanvas published views, dynamic raster
    maps, raster `u32` operations, or production `mem::forget`; the source contains three retained
    fault-cursor construction sites and all named allocation validation edges;
  - scoped working, staged, and `HEAD` diff checks passed for the five source files plus this
    report. Whole-tree checks remain red only on concurrent, out-of-scope hygiene:
    `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md:459` has trailing whitespace in working/`HEAD`, and the
    staged/`HEAD` prior raster audit has a blank line at EOF on line 102. Neither file was edited by
    this packet.
- This repair is **audit-ready, not accepted**. Cargo, Nx, Wasm, browser, runtime/GPU execution,
  network, and root lint were not run. Rust type checking and actual device behavior therefore
  remain unproved. Phase 3/5 remains **RED** for prepared-pixel producer ownership, atlas/icon/glyph
  resources, dynamic EngineCanvas surface retirement, Vello/render/submit/present timing, opaque
  GPU destruction, complete GpuContext/realm teardown, semantic codecs/input, and the native/Wasm/
  browser matrix.

### Raster reservation-mutation verifier repair

- The independent final re-audit found one verifier-only false negative: mutation 8 removed
  `gpu.reserve_engine_texture`, but the permanent raster contract checked only the later
  validation/allocation seams. Live raster allocation and ownership source was not changed by this
  repair.
- The contract now requires exactly one exact
  `let admission = gpu.reserve_engine_texture(&key, width, height, candidate, expected)?;`
  occurrence in EngineCanvas and requires its source index to precede the earliest target-texture,
  view, or `Renderer::new` allocation. The existing immediate full-tuple validation rules still
  guard every individual allocation edge.
- The original 38-entry mutation matrix is unchanged. Its reservation-erasure mutation now fails
  the exact-count rule, and a separate adversary removes the reservation line and reinserts it
  immediately after the first target-texture allocation. That source retains one syntactically
  exact token but fails the ordering rule, preventing a presence-only witness.
- A faithful Bun reconstruction using Rust's all-occurrence replacement and absent-index sentinel
  semantics reports baseline true and **38/38** original mutations denied. The independent
  reservation-order probe reports the live count/index witness true, erasure denied, and
  move-after-first-allocation denied.
- Edition-2021 scoped rustfmt/check and parser output passed for framework draw/GPU/glue, product
  WGPU glue, and EngineCanvas. The authored source check finds the exact count/order rules,
  unchanged `mutations.len() == 38`, mutation 8, and the separate ordering adversary.
  Interactivity self-test and plain DENY both exit zero with only the recorded allowlisted
  test-only blocking bridge. Focused owner-erasure/clone/map/legacy scans remain clean. Scoped
  working/staged/`HEAD` whitespace checks pass; whole checks remain concurrently RED only for the
  unrelated prompt trailing whitespace and the staged prior raster audit's blank line at EOF.
- Cargo, Nx, Wasm, browser, runtime, network, and root lint were not run. This is audit-ready
  source, not Phase 3 acceptance; all previously reported runtime and out-of-packet residuals
  remain RED.
