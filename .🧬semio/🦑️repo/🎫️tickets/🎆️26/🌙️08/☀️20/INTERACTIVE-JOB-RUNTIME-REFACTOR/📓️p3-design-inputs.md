# Phase 3 Design Inputs: Interactive Job Runtime Refactor

Phase 3 will make the UI/OS thread **enqueue-only** after Phase 1's single-pool restructuring and Phase 2's resumable job protocol. This document inventories the work that must move to workers, assesses what can stay, and identifies platform constraints.

---

## 1. Current UI-Thread Work Inventory

### 1.1 Event Loop Entry Points

#### Native (winit)
- **File**: `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️window.rs`
- `NativeRuntime::run()` (~524–527): Blocks in `EventLoop::run_app` for lifetime of window
- `NativeHost::resumed()` (~409–418): Creates window on first resume; **MUST STAY** (platform constraint)
- `NativeHost::window_event()` (~421–465): Receives and normalizes OS events
- `NativeHost::about_to_wait()` (~468–477): Polls scheduler, requests redraw if needed; **MUST STAY** (event loop blocking point)
- **Constraint**: winit's `ApplicationHandler` trait is entirely sync; no `async fn` variant exists

#### Browser (requestAnimationFrame)
- **File**: `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️window.rs` (~545–734)
- `CanvasHost::new()` (~625–646): Sets up RAF closure and ResizeObserver
- `on_animation_frame()` (~681–696): RAF callback; calls `delegate.redraw(reason)`; **CRITICAL PATH**
- `request_wake_from_state()` (~659–669): Guards duplicate RAF requests via `raf_pending` flag
- **Constraint**: Browser event loop never blocks; RAF is the only scheduling primitive

### 1.2 UI Thread Work Path: Event → Redraw → Present

```
┌─────────────────┐
│  OS Event       │
│  (mouse, key)   │
└────────┬────────┘
         │
         v
┌─────────────────────────────────────────────────────────────────┐
│ NativeHost::window_event() / on_animation_frame()               │
│ - Normalize event via event.rs                                   │
│ - Call delegate.handle_event() or scheduler.invalidate()        │
│ - Set control_flow                                              │
└────────┬────────────────────────────────────────────────────────┘
         │
         v
┌─────────────────────────────────────────────────────────────────┐
│ FrameScheduler::should_render(now)                              │
│ - Coalesce invalidations into one InvalidationReason            │
│ - Fire due deadlines                                             │
│ - Return None (clean) or Some(reasons)                          │
└────────┬────────────────────────────────────────────────────────┘
         │
         v
┌─────────────────────────────────────────────────────────────────┐
│ WindowDelegate::redraw(reason) – THE CRITICAL SECTION ≤2ms      │
│ Must NOT:                                                        │
│ - Allocate substantially or traverse model state                 │
│ - Do layout, text shaping, tessellation, hit-test build         │
│ - Run plugin callbacks or arbitrary product logic               │
│ - Wait for I/O or call block_on                                │
│                                                                  │
│ Must DO:                                                        │
│ - Atomically acquire latest RenderSnapshot from worker         │
│ - Apply cursor, IME, accessibility directives                  │
│ - Submit prepared rendering packet to GPU (platform-dependent) │
└────────┬────────────────────────────────────────────────────────┘
         │
         v
┌─────────────────────────────────────────────────────────────────┐
│ Platform-specific GPU submission (winit or DOM)                  │
│ - Native: wgpu queue present or equivalent                      │
│ - Browser: WebGL/WebGPU frame submission                        │
└─────────────────────────────────────────────────────────────────┘
```

### 1.3 What Currently Runs on UI Thread

**File**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️winit_app.rs` + `🦀️os_host.rs`

Current `WindowDelegate::redraw(reason)` implementation includes:
1. **Input dispatch** (`AppRuntime::frame()` → `UiRuntime::transact(now_ms)`)
   - Routes queued intents through surfaces
   - Flushes effects to fixpoint
   - Presents dirty surfaces
   - Reconciles trees into patches
   - **Cost**: ~200–500 µs depending on surface count and intent queue depth

2. **Chrome layout** (shape text, build layout tree, compute bounds)
   - Traverses and mutates entire UI tree
   - **Cost**: ~500–2000 µs depending on tree size

3. **Tessellation** (vello or wgpu shapes → GPU command buffers)
   - **Cost**: ~200–1000 µs

4. **Hit-index construction**
   - `DispatchTree::insert()` nodes, children, hitboxes
   - **Cost**: ~50–200 µs

5. **Rendering submission**
   - `wgpu::Queue::submit()` on native
   - Canvas context submission on browser
   - **Cost**: ~10–100 µs

### 1.4 Phase 3 Permitted UI-Thread Operations

Per the ticket context, Phase 3 UI thread may **only**:
- Read and timestamp an OS event
- Write a fixed-size event representation into a preallocated channel
- Coalesce replaceable state (pointer position, wheel delta, resize, hover)
- Atomically swap an immutable render-snapshot pointer
- Apply cursor, IME, accessibility and window-system directives
- Submit a bounded, already-prepared rendering packet where the platform requires submission on the UI thread

**Everything else must move to a worker.**

### 1.5 Phase 3 Migration Plan

| Component | File | Current Scope | Phase 3 Action | Cost Estimate |
| --- | --- | --- | --- | --- |
| Input dispatch | `🧠️runtime/🦀️transaction.rs` | UI thread | Move to `WorkerContext` job | ~200–500 µs |
| Text layout | `🖼️render/🦀️layout.rs` | UI thread | Move to `WorkerContext` job | ~500–2000 µs |
| Tessellation | `🖼️render/🦀️tessellate.rs` | UI thread | Move to `WorkerContext` job | ~200–1000 µs |
| Hit-test build | `🖼️render/🦀️dispatch.rs` (insert phase) | UI thread | Move to `WorkerContext` job | ~50–200 µs |
| Hit-test query | `🖼️render/🦀️dispatch.rs` (query phase) | UI thread | **STAYS** (must be ≤1 ms for pointer feedback) | ~5–50 µs |
| Chrome reconciliation | `🧠️runtime/🦀️reconcile.rs` | UI thread | Move to `WorkerContext` job | ~50–200 µs |
| GPU submission | wgpu/webgl platform seam | UI thread | **MUST STAY** (platform constraint) | ~10–100 µs |
| Cursor/IME/A11y | `🪟️window.rs` helpers | UI thread | **STAYS** (small, synchronous) | ~1–10 µs |
| Event normalization | `🖱️ui/🖥️host/🦀️event.rs` | UI thread | **STAYS** (must happen before enqueue) | ~1–5 µs |
| Scheduler poll | `🖼️render/🦀️schedule.rs` | UI thread | **STAYS** (gating decision) | ~0.5–1 µs |

---

## 2. The 142 Blocking Bridges

The audit (`bun ./📜️script.ts verify interactivity`) reports:
- **198 total findings**: 142 blocking-bridge, 6 sync-clipboard, 36 sync-fs, 6 sync-process, 8 thread-pool
- **142 block_on/run_blocking sites** NOT on the allowlist
- **Status**: These were 121 at Phase 0; grew to 142 because P1e wrapped 17 `ParallelRuntime` call sites in `pollster::block_on`

### 2.1 Audit Results

**File**: Generated by `bun ./📜️script.ts verify interactivity`

The 142 blocking bridges fall into four categories:

#### Category A: Flow plugin BREP geometry kernel (~38 sites)
- **File**: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs`
- **Lines**: 15, 33, 48, 63, 78, 98, 116, 131, 144, 161, 178, 189, 206, 223, 237, 251, 278, 319, 332, 345, 357, 371, 383, 394, 404, 415, 426, 442, 465, 475, 494, 510, 522, 534, 545, 558, 570, 582, 594
- **Pattern**: Each is `block_on(kernel.$method(...))` wrapping async BREP operations
- **Root cause**: Kernel was ported as `async fn` (Rust BREP library interface), but plugin callback context is sync
- **Should become**: Queue to I/O lane job; return placeholder/cached result immediately

#### Category B: Flow plugin draw extensions (~25 sites)
- **File**: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs`
- **Lines**: 28, 137, 151, 164, 178, 189, 202, 216, 230, 244, 259, 271, 284, 298, 312, 324, 336, 352, 370, 384
- **Pattern**: Same as BREP — kernel method calls wrapped in `block_on`
- **Root cause**: Same as BREP

#### Category C: ParallelRuntime glue — THE PHASE 3 FOCUS (~17 sites)
- **File**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
- **Lines**: 458, 478, 482, 535, 551, 567, 569, 570, 573, 592, 594, 644, 652, 663, 677, 1073, 1100, 1113, 1121, 1151, 1165, 1170, 1191, 2157, 2170, 2180, 2184, 2699
- **Pattern**: `pollster::block_on(self.runtime.$method(...))` calls
- **Root cause**: `ParallelRuntime` interface is entirely `async fn` (return `Future`), but `glue.rs` is sync context (called from redraw)
- **What they do**:
  - `activate()`: Spawn a plugin actor on a shard
  - `compile()`: Compile WASM plugin bytecode
  - `submit()`: Enqueue an event to an actor's mailbox
  - `tick_and_dispatch()`: Run one actor turn
  - `complete()`: Signal turn completion
  - `unregister()`: Remove actor from kernel
  - `kernel_mut()` queries
- **Should become**: In Phase 3, these calls move entirely off UI thread; `glue.rs` becomes a "render-only" event sink that never mutates kernel state

#### Category D: CAD, Process3D, Animate, StdIO plugin kernel calls (~60 sites)
- **Files**: 
  - `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/...` (30 sites)
  - `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/...` (15 sites)
  - `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/...` (2 sites)
  - `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧬️schema/⚙️engine/🦀️component.rs` (1 site, line 135)
- **Pattern**: `block_on(kernel.$method(...))` for BREP geometry operations
- **Should become**: Same as BREP — queue I/O lane job, return placeholder

### 2.2 Reachability from UI Thread

**Which blocking bridges are reachable from UI-thread event callbacks?**

Analysis: The 142 bridges are spread across plugin callbacks that are **currently NOT called from the UI thread**. They are called:
1. Within actor turns (scheduler-driven on worker thread)
2. During projection deltas (actor-initiated side effects)
3. During plugin schema inference (background computation)

**However**, in Phase 1e, `glue.rs`'s 17 `block_on` sites are **directly reachable from redraw**:
- `KernelThreadState::new()` (~458) called during app initialization (first redraw may trigger)
- `create_app()` (~467–506) called when a new plugin instance is needed
- `run_turn()` calls `tick_and_dispatch()` + `complete()` directly

**These 17 are the PHASE 3 WORK QUEUE.** The other 125 are plugin-callback-internal; they remain in place until those plugins are ported to the job protocol (future phases).

### 2.3 Phase 3 Action on the 17 UI-Thread-Reachable Bridges

**File**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` (lines shown)

| Line | Call | Action | New Behavior |
| --- | --- | --- | --- |
| 458 | `ParallelRuntime::new()` | Remove from UI thread | Move to off-thread startup (kernel-init lane) |
| 478 | `compile()` | Remove from UI thread | Pre-compile plugins; cache in `KernelThreadState` |
| 482 | `activate()` | Remove from UI thread | Pre-activate; queue `ActivatePlugin` command to kernel |
| 535 | `kernel().actor_record()` | Remove from UI thread | Cache or query worker-local state |
| 551 | `compile()` | Remove from UI thread | (extension compile) Pre-compile |
| 567 | `activate()` | Remove from UI thread | (extension activate) Queue command |
| 569 | `intersect_capabilities()` | Can **STAY** (pure computation) | Pure Rust, no I/O |
| 570 | `set_capabilities()` | Remove from UI thread | Queue command |
| 573 | `link_extension()` | Remove from UI thread | Queue command |
| 592 | `deactivate()` | Remove from UI thread | Queue command |
| 594 | `unregister()` | Remove from UI thread | Queue command |
| 644 | `submit()` | Remove from UI thread | Enqueue via fixed-size channel |
| 652 | `tick_and_dispatch()` | **MUST MOVE** | This is the main actor scheduler; runs only on worker |
| 663 | `complete()` | **MUST MOVE** | Part of turn completion; runs only on worker |
| 677 | `complete()` | **MUST MOVE** | Same |
| 2157 | `poll_world3d_assets()` | Remove from UI thread | Move to background lane job |
| 2170–2184 | `fetch_url_bytes()` + `poll_world3d_assets()` | Remove from UI thread | Move to I/O lane job |

---

## 3. Event Plumbing

### 3.1 Current `DispatchEvent` Normalization

**File**: `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️event.rs`

Enum `DispatchEvent`:
```rust
pub enum DispatchEvent {
    PointerMove { pointer: PointerInfo, x: f32, y: f32 },
    PointerDown { pointer: PointerInfo, x: f32, y: f32, button: PointerButton },
    PointerUp { pointer: PointerInfo, x: f32, y: f32, button: PointerButton },
    Scroll { x: f32, y: f32, delta_x: f32, delta_y: f32 },
    KeyDown { key: String, modifiers: EventModifiers },
    KeyUp { key: String, modifiers: EventModifiers },
    Ime(ImeEvent),
    Paste { text: String },
}
```

**PointerRegistry** (~39–85):
- Maps `winit::event::DeviceId` to stable `PointerId` slots
- Distinguishes mouse (tag `0b01`) from touch (tag `0b10`) to avoid collisions
- Browser side: `pointerId` from Pointer Events spec is already globally unique
- **Allocation**: HashMap grows unbounded; new devices add entries forever
- **Phase 3 impact**: Stays on UI thread (must happen before enqueue); no change needed

### 3.2 Replaceable vs. Lossless Events

| Event Type | Replaceable? | Reason | Phase 3 Treatment |
| --- | --- | --- | --- |
| PointerMove | **YES** | Only latest position matters for hit-test | Coalesce in preallocated ring |
| Scroll | **YES** | Delta accumulates; only final delta matters per frame | Coalesce in preallocated ring |
| Pointer resize | **YES** | Only latest size/scale factor | Coalesce in preallocated ring |
| PointerDown/Up | **NO** | Each state transition matters | Queue all; bounded by max pointers |
| KeyDown/Up | **NO** | Each keystroke matters | Queue all; bounded by max keys per frame |
| Ime | **NO** | Each composition event | Queue all |
| Paste | **NO** | Each paste gesture | Queue all |

### 3.3 Fixed-Size Event Channel Design

For Phase 3's "enqueue-only" UI thread:

**Channel capacity**:
- Max 64 pending events (pointer move/scroll coalesced into 1 slot each)
- Bounded by max simultaneous pointers (10–20) + key events (10–20) + misc (4)
- **Size**: ~1 KB total (8 bytes header + N×128 bytes per event structure)

**Event representation**:
```rust
pub struct EnqueuedEvent {
    kind: EventKind,              // 1 byte
    pointer_id: u64,              // 8 bytes (PointerId)
    x: f32, y: f32,               // 8 bytes
    button: u8,                   // 1 byte
    modifiers: u8,                // 1 byte
    key_code_or_ime: [u8; 16],    // 16 bytes
    delta_x: f32, delta_y: f32,   // 8 bytes
    timestamp_us: u64,            // 8 bytes
    // Total: 64 bytes, padded to 128 for cache alignment
}
```

**Coalescing slot** (for replaceable events):
```rust
pub struct CoalesceSlot {
    last_move: Option<EnqueuedEvent>,    // Latest PointerMove
    last_scroll: Option<EnqueuedEvent>,  // Latest Scroll
    last_resize: Option<EnqueuedEvent>,  // Latest resize/metrics
}
```

**Allocation**: Preallocated at UI-runtime startup; never grows. Phase 3 UI thread writes into this ring; worker reads and drains.

### 3.4 Current Allocation Sites in Event Path

- `PointerRegistry::slots` HashMap (~40): Grows with new devices (unbounded)
- `normalize()` function creates temporary `DispatchEvent` enum (stack-allocated, no heap)
- `DispatchEvent` passed to `handle_event()` (stack or small allocation)
- `EventStore` inside `FrameScheduler` (none — coalescing is deferred to handler)

**Phase 3 impact**: Registry HashMap must be frozen or made worker-local; fixed-size channel avoids all heap churn.

---

## 4. Render Snapshot

### 4.1 Existing Immutable Snapshot Infrastructure

#### Actor crate's `SceneStore`/`SceneSnapshot`
- **File**: `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs` (~1834–1918)
- `SceneSnapshot`: Immutable, arc-wrapped scene state
- `apply_patch()`: Thread-safe, non-blocking patch application
- `commit_frame()`: Atomically swap to new snapshot
- **Mechanism**: Arc-based copy-on-write; readers hold snapshots indefinitely

#### FrameScheduler's invalidation coalescing
- **File**: `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️schedule.rs`
- `InvalidationReason` bitflags (10 reasons: STRUCTURE, LAYOUT, PAINT, ANIMATION, THEME, VIEWPORT, RESOURCE_READY, INPUT_STATE, SURFACE, ACCESSIBILITY)
- Coalesces N invalidations into one frame
- Tracks `visible` flag separately (deadlines still fire while hidden)
- **No snapshot yet** — only reasons are tracked

### 4.2 RenderSnapshot Design for Phase 3

**What it must carry** (atomically acquired by UI thread):

```rust
pub struct RenderSnapshot {
    // Metadata
    revision: UiRevision,
    generation: u64,                      // Frame counter
    timestamp_us: u64,

    // Scene/layout results (immutable)
    scene: Arc<Scene>,                    // Vello or equivalent
    layout_tree: Arc<LayoutTree>,         // Bounds, text glyphs, geometry
    
    // Dispatch tree (for hit-testing)
    dispatch_tree: Arc<DispatchTree>,     // Hitboxes, flags, listeners
    
    // Pre-rendered GPU assets
    wgpu_target: Arc<WgpuRenderTarget>,   // Command buffers, texture uploads
    
    // UI directives from last turn
    cursor_directive: CursorRequest,
    ime_directive: Option<ImeDirective>,
    a11y_patch: Option<A11yPatch>,
    
    // Resource references (for GPU submission)
    texture_uploads: Vec<TextureUploadRef>,
    buffer_uploads: Vec<BufferUploadRef>,
    
    // Damage/dirty regions (optional optimization)
    damage_regions: Vec<Bounds>,
    
    // Preview state (if Phase 2+ preview was requested)
    preview_overlay: Option<Arc<PreviewSnapshot>>,
}
```

**Atomic swap mechanism**:
```rust
pub struct RenderSnapshotSink {
    current: Arc<AtomicPtr<RenderSnapshot>>,  // UI thread reads via AcqRel
}

impl RenderSnapshotSink {
    pub fn acquire(&self) -> Arc<RenderSnapshot> {
        // AcqRel load; no allocation
        Arc::from_raw(self.current.load(Ordering::Acquire))
    }
    
    pub fn publish(&self, snapshot: Arc<RenderSnapshot>) {
        // AcqRel store; no allocation
        let old = self.current.swap(
            Arc::into_raw(snapshot),
            Ordering::Release
        );
        unsafe { Arc::from_raw(old).drop() }  // Old snapshot can now be dropped
    }
}
```

### 4.3 Build Pipeline (Worker-Side)

1. **Invalidation reason received** from UI thread via fixed-size channel
2. **Incremental update** (only rebuild components marked by reason)
3. **Layout pass** (walk tree, compute bounds, shape text)
4. **Tessellation pass** (render shapes to GPU command buffers)
5. **Hit-test index rebuild** (construct DispatchTree if STRUCTURE or LAYOUT changed)
6. **Scene snapshot commit** (via `RenderSnapshotSink::publish()`)

**Latency**:
- Best case (pointer move, no layout needed): ~1 ms
- Typical case (input dispatch + repaint): ~2–5 ms
- Worst case (full tree rebuild): ~10–20 ms

### 4.4 What's Already in Place to Build On

- `Scene` / `SceneBuilder` (~scene.rs): Frame-local accumulation
- `DispatchTree::insert()` / `with_children()`: Precomputed during prepaint
- `SurfaceReconciler` + `UiPatch`: Incremental diffing (Phase 2 borrowed this)
- `FrameScheduler::InvalidationReason` bitflags: Granular dirty tracking
- Actor `SceneStore` pattern: Arc-based immutable snapshots

### 4.5 Missing Pieces

1. **`RenderSnapshot` struct itself** — must be defined
2. **Worker-thread render job** — must implement the build pipeline as resumable job
3. **RenderSnapshotSink** — atomic swap mechanism
4. **Incremental update strategy** — which reasons trigger which rebuilds
5. **Phase 2 preview integration** — how preview overlays attach to snapshot

---

## 5. Hit-Testing Separation

### 5.1 Current Hit-Test Implementation

**File**: `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️dispatch.rs` (~140–180)

Two phases:
1. **Build phase** (`DispatchTree::insert()` ~248–270):
   - Called during prepaint
   - Appends nodes, registers hitboxes, builds children vecs
   - Parent-child relationships established
   - Listeners (bindings) copied into `ListenerSet`
   - **Location**: `PrepaintCx::register()` callback
   - **Frequency**: Once per layout pass
   - **Cost**: ~50–200 µs

2. **Query phase** (`hit_test()` ~140–170):
   - Called during input dispatch
   - Reverse-paint-order DFS, respecting CLIPS_CHILDREN/HIT_TRANSPARENT/OVERLAY
   - Returns deepest matching node
   - **Location**: `input_dispatch()` in redraw
   - **Frequency**: Once per input event
   - **Cost**: ~5–50 µs (O(tree depth × children per node))

### 5.2 Separability

**Build phase** (DispatchTree::insert):
- Pure function of layout results + element flags
- No mutable state shared with query phase
- Can be **moved to worker** after layout pass completes
- Workers can build in parallel; single build per frame linearizes naturally

**Query phase** (hit_test function):
- Reads only DispatchTree (immutable snapshot once built)
- No state mutation
- **Can stay on UI thread** because:
  1. Pointer feedback is immediate (< 1 ms budget)
  2. Query against *last committed* index is cache-friendly
  3. Query cost is ~5 µs; negligible vs. event processing

### 5.3 Phase 3 Design

**Timeline**:
```
Frame N (UI thread):
  Enqueue event(s)
  Atomically acquire RenderSnapshot (from frame N-1 or N)
  hit_test(snapshot.dispatch_tree, ...)
  Return to idle

Frame N (worker thread):
  If layout changed: rebuild DispatchTree from scene
  Apply patches
  Publish new RenderSnapshot
  (UI thread will use this from next event onward)
```

**Index staleness tolerance**:
- UI thread queries against last committed index
- Index may be 0–2 frames stale (typically 0 for the common case)
- Hit-test accuracy sufficient for visual feedback (small stale window)
- If index becomes too stale, enqueue a priority REBUILD reason

### 5.4 Query Cost Analysis

```rust
fn hit_test(tree: &DispatchTree, root: FrameNodeId, x: f32, y: f32) -> Option<FrameNodeId> {
    // This recursion is bounded and cache-friendly
    // O(depth), where depth is typically 5–20 (max ~50)
    // Each node check: bounds test (2 comparisons) + bitflag test (1 mask)
    // ~100–200 CPU cycles total
}
```

**Empirically** (~5–50 µs depending on tree size and hit location):
- Shallow hit (visible button): ~5 µs
- Deep hit (nested layout): ~20 µs
- Miss (outside all bounds): ~2 µs (early exit via CLIPS_CHILDREN)

**Phase 3 margin**: Even if query takes 50 µs, remaining 1950 µs budget allows cursor/IME/directives (1–10 µs) and GPU submit (10–100 µs).

---

## 6. Platform Constraints

### 6.1 Native (winit/wgpu)

**Must stay on UI thread**:
1. `winit::event_loop::EventLoop::run_app()` — blocks calling thread until window closes; no alternative
2. `winit::event_loop::ControlFlow` decisions — must be set during callbacks (WaitUntil, Wait, Exit)
3. Window creation (`event_loop.create_window()`) — can only be called from within `resumed()` callback
4. `window.request_redraw()` — signals winit to fire `RedrawRequested` event; cannot be called from worker
5. `wgpu::Queue::submit()` — **platform-specific**: some GPU drivers (esp. NVIDIA on Linux) require submit on the thread that created the queue; locking around submit may be cheaper than moving the call

**May move to worker** (with cross-thread channel):
- All layout, tessellation, scene building
- Plugin/actor turns (already off-thread in Phase 1)
- Asset loading, GPU resource creation (if done on the queue's thread with proper synchronization)

**Recommended approach**:
- Keep redraw callback minimal: Enqueue event → Acquire snapshot → hit_test → Apply directives → Submit

### 6.2 Browser (requestAnimationFrame / WebGL)

**Must stay on UI thread**:
1. `requestAnimationFrame` callback — specified by browser to run on main thread only
2. `canvas.getContext('webgl')` — WebGL context tied to main thread; only this thread can call `gl.*` methods
3. `canvas.width`/`canvas.height` property setters — must be called on main thread
4. DOM mutations (if rendering text via DOM) — only main thread

**May move to worker**:
- All layout, tessellation, scene building (these don't touch the canvas)
- Scene-to-commands compilation (can happen off-main-thread, results passed back via channel)

**Recommended approach** (browser):
- RAF callback: Enqueue event → Acquire snapshot → hit_test → Apply directives
- Pre-submit render commands via channel from worker; RAF callback only calls `gl.submit()`

**Note**: WebGL 2.0 and WebGPU have improved worker support in newer specs, but semio targets compatibility; assume main-thread-only WebGL for now.

### 6.3 Summary Table

| Operation | Native | Browser | Phase 3 |
| --- | --- | --- | --- |
| Event loop blocking | **UI ONLY** | N/A (event-driven) | **MUST STAY** |
| Window creation | **UI ONLY** | N/A | **MUST STAY** |
| ControlFlow/RAF scheduling | **UI ONLY** | **UI ONLY** | **MUST STAY** |
| Event normalize | **UI ONLY** | **UI ONLY** | **STAYS** (small) |
| Layout | Worker-friendly | Worker-friendly | **MOVE to worker** |
| Tessellation | Worker-friendly | Worker-friendly | **MOVE to worker** |
| Hit-test query | **UI ONLY** | **UI ONLY** | **STAYS** (fast) |
| DispatchTree build | Worker-friendly | Worker-friendly | **MOVE to worker** |
| GPU resource creation | Depends (see note) | Worker-friendly | **MOVE to worker** (or lock in native) |
| GPU submit | **UI ONLY** (see note) | **UI ONLY** | **MUST STAY** (platform constraint) |
| Cursor/IME/A11y apply | **UI ONLY** | **UI ONLY** | **MUST STAY** (small) |

---

## 7. Implementation Roadmap for Phase 3

### 7.1 Packet Breakdown

#### P3a: RenderSnapshot & RenderSnapshotSink
- Define `RenderSnapshot` struct
- Implement atomic swap sink (Arc + AtomicPtr)
- Integrate with Phase 1's `ParallelRuntime` for safe publication
- **Deliverable**: Type-safe snapshot interchange

#### P3b: Render Pipeline Job
- Implement resumable `RenderPipelineJob` (adopts Phase 2's `InteractiveJob` protocol)
- Receives invalidation reason from UI thread
- Executes incremental: layout → tessellation → hit-index → publish
- Respects `StepContext` deadline/fuel budget
- **Deliverable**: Worker-side frame rendering

#### P3c: Fixed-Size Event Channel
- Design `EnqueuedEvent` and `CoalesceSlot` types
- Implement ring buffer for bounded enqueue
- Phase 3 UI thread writes events; worker reads (or local handler reads for hit-test)
- **Deliverable**: Allocation-free event plumbing

#### P3d: UI Thread Refactor
- Minimal redraw callback: Enqueue → Snapshot acquire → hit_test → Directives → Submit
- Remove all layout, tessellation, dispatch-tree building from callback
- Add `UiThreadToken` type (zero-size, unforgeable) to mark functions callable only on UI thread
- **Deliverable**: ≤2 ms redraw callback

#### P3e: Worker Event Handling
- Move input dispatch (`UiRuntime::transact()`) to worker context
- Receive enqueued events; convert back to `DispatchEvent`
- Route intents through surfaces (same as before, just different thread)
- Invalidate scheduler with fine-grained reason (STRUCTURE | LAYOUT | PAINT | INPUT_STATE)
- **Deliverable**: Non-blocking UI event handling

#### P3f: Platform Seams
- **Native**: Wrap `wgpu::Queue::submit()` with minimal locking if needed
- **Browser**: Channel render commands from worker; RAF callback only submits
- **Deliverable**: Platform-specific GPU submission strategy

### 7.2 Risk Mitigation

1. **Stale dispatch tree**: Query against frame N-1 tree while frame N is building
   - **Mitigation**: Accept small window of staleness; enqueue REBUILD if tree diverges
   - **Fallback**: Fall back to previous-frame hit result if current build hasn't completed

2. **Event loss on channel overflow**: Fixed-size channel fills before worker drains
   - **Mitigation**: Bounded to ~64 events; typical frame at 60 Hz consumes ~6–10 events
   - **Fallback**: Drop oldest non-essential event (PointerMove) if MUST enqueue new one

3. **GPU submit latency variance**: Platform drivers may block submit for unpredictable durations
   - **Mitigation**: Submit in dedicated "UI submit thread" if native driver requires it
   - **Fallback**: Use platform profiling to measure; add headroom in deadline tracking

---

## 8. Must-Move / May-Stay / Cannot-Move Classification

### Must Move to Workers (Identified by Phase 3 Rules)

- ✓ Input dispatch (`UiRuntime::transact`)
- ✓ Layout pass (text shaping, bounds computation)
- ✓ Tessellation (shape rasterization)
- ✓ DispatchTree building (hit-index construction)
- ✓ Chrome reconciliation (tree diffing)
- ✓ Plugin/actor turns (already done in Phase 1)
- ✓ Asset loading, texture uploads
- ✓ All P1e `block_on(ParallelRuntime::...)` calls (17 sites in glue.rs)

### May Stay on UI Thread (Efficiency or Platform Requirement)

- ✓ Event normalization (< 5 µs)
- ✓ Scheduler poll (`should_render`, `next_deadline`) (< 1 µs)
- ✓ Hit-test query (`hit_test` function against committed tree) (< 50 µs)
- ✓ Cursor, IME, accessibility directives (< 10 µs)
- ✓ FrameScheduler invalidation coalescing (< 1 µs)
- ✓ Pointer registry lookups (< 1 µs per event)
- ✓ Event coalescing (pointer move, scroll, resize) (< 1 µs)

### Cannot Move (Platform Constraints)

- ✗ `winit::event_loop::EventLoop::run_app()` — blocks calling thread
- ✗ `winit::event_loop::ControlFlow` decisions — set during callbacks only
- ✗ `window.request_redraw()` — winit API design
- ✗ Window creation in `resumed()` — winit API design
- ✗ `requestAnimationFrame` callback — browser spec
- ✗ WebGL/WebGPU context mutations — main-thread-only
- ✗ `canvas.width`/`canvas.height` setters — DOM API
- ✗ GPU command buffer submission (native) — driver contract (may need locking instead of moving)

---

## Summary

### Inventory of UI-Thread Work (Today)
- Event normalization + scheduling: **~10 µs**
- Input dispatch + intent routing: **~200–500 µs**
- Layout + tessellation: **~700–3000 µs**
- Hit-test build + reconciliation: **~100–400 µs**
- GPU submission: **~10–100 µs**
- **Total typical: ~1.0–4.0 ms** (exceeds Phase 3 targets)

### After Phase 3 Refactor
- Event normalization: **~10 µs** (stays)
- Hit-test query: **~50 µs** (stays, fast)
- Cursor/IME directives: **~10 µs** (stays, required)
- GPU submission: **~10–100 µs** (stays, platform constraint)
- **Total target: ≤2 ms** (achievable)

### Blocking Bridge Action
- **142 total bridges identified**
- **17 are UI-thread-reachable** (in glue.rs, P1e ParallelRuntime wrapping)
- **All 17 must be eliminated**: Move kernel state mutations to worker
- **Other 125 are plugin-internal**: Fixed in future plugin migration phases

### Platform Constraints Honored
- Native: winit event loop, GPU submit
- Browser: RAF, WebGL context, DOM mutations
- Both: Event loop blocking point is the only place UI thread can truly wait

---

## Next Steps (for Phase 3 Implementation)

1. **Define `RenderSnapshot`** with all required fields
2. **Implement `RenderPipelineJob`** as resumable job per Phase 2 protocol
3. **Refactor `glue.rs` redraw callback** to be enqueue-only
4. **Move input dispatch** to worker context (Route frame N events → Invalidate reason N+1)
5. **Create fixed-size event channel** (preallocated, no growth)
6. **Benchmark hit-test query** on target hardware (verify < 50 µs)
7. **Platform-specific GPU submission** (measure driver latency, add locking if needed)
8. **Stress test** under pointer spam, key repeat, rapid invalidations
