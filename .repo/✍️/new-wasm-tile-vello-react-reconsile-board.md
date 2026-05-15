## Recommended architecture

Use a **Rust-owned retained scene graph** with a **thin imperative TypeScript facade** and a **React custom renderer** on top.

```txt
React JSX declarative layer
  elements/client/lib/board/react/index.tsx
        ↓ reconciles props into imperative objects

Three-like imperative TS API
  elements/client/lib/board/js/index.ts
        ↓ batches commands across WASM boundary

Rust/WASM engine
  elements/client/lib/board/rs/lib.rs
        ↓ builds visible/tiled Vello scenes

Vello + wgpu/WebGPU canvas renderer
```

Vello is a good fit because it is a Rust 2D renderer focused on GPU compute, uses `wgpu`, and is designed for large interactive 2D scenes. ([GitHub][1]) The React layer should follow the `react-three-fiber` model: JSX declaratively creates and mutates native imperative objects, rather than React owning the render loop. R3F describes itself as a React renderer for Three.js and maps JSX elements to native Three.js objects. ([GitHub][2])

---

# 1. Rust/WASM core: `elements/client/lib/board/rs/lib.rs`

## Core principle

Rust owns:

| Concern            | Owner                                     |
| ------------------ | ----------------------------------------- |
| Scene graph        | Rust                                      |
| Node/edge geometry | Rust                                      |
| Spatial index      | Rust                                      |
| Tile invalidation  | Rust                                      |
| Hit testing        | Rust                                      |
| Selection state    | Rust                                      |
| Drag interaction   | Rust                                      |
| Camera transform   | Rust                                      |
| Render scheduling  | JS/React asks, Rust decides what is dirty |
| GPU rendering      | Rust/Vello                                |

JS and React should not compute geometry per frame.

---

## Rust module layout

Even if implemented in one `lib.rs` initially, structure it internally like this:

```rust
mod ids;
mod math;
mod scene;
mod node;
mod handle;
mod edge;
mod bezier;
mod spatial;
mod tile;
mod hit_test;
mod interaction;
mod render;
mod wasm;
```

Suggested public WASM surface:

```rust
#[wasm_bindgen]
pub struct BoardEngine {
    scene: Scene,
    camera: Camera,
    tiles: TileCache,
    spatial: SpatialIndex,
    interaction: InteractionState,
    renderer: VelloRenderer,
    events: EventQueue,
}
```

Export coarse-grained methods only:

```rust
#[wasm_bindgen]
impl BoardEngine {
    #[wasm_bindgen(constructor)]
    pub async fn new(canvas: web_sys::HtmlCanvasElement) -> Result<BoardEngine, JsValue>;

    pub fn resize(&mut self, width: u32, height: u32, dpr: f32);
    pub fn set_camera(&mut self, x: f32, y: f32, zoom: f32);

    pub fn begin_batch(&mut self);
    pub fn end_batch(&mut self);

    pub fn create_node(&mut self, id: u64, x: f32, y: f32, r: f32);
    pub fn update_node(&mut self, id: u64, x: f32, y: f32, r: f32);
    pub fn remove_node(&mut self, id: u64);

    pub fn create_handle(&mut self, id: u64, node_id: u64, angle: f32);
    pub fn create_edge(&mut self, id: u64, from_handle: u64, to_handle: u64);

    pub fn pointer_down(&mut self, x: f32, y: f32, buttons: u32, modifiers: u32);
    pub fn pointer_move(&mut self, x: f32, y: f32, buttons: u32, modifiers: u32);
    pub fn pointer_up(&mut self, x: f32, y: f32, buttons: u32, modifiers: u32);

    pub fn render(&mut self, timestamp_ms: f64);
    pub fn drain_events_ptr(&mut self) -> *const u8;
    pub fn drain_events_len(&self) -> usize;
}
```

Use `wasm-bindgen` for Rust/JS interop; it exports Rust functions to JavaScript and generates only the glue needed for used imports/exports. ([GitHub][3])

---

## Scene model

Use stable integer IDs, not JS object identity.

```rust
type NodeId = u64;
type HandleId = u64;
type EdgeId = u64;

struct Node {
    id: NodeId,
    center: Vec2,
    radius: f32,
    handles: SmallVec<[HandleId; 8]>,
    flags: NodeFlags,
    style: StyleId,
    bounds: Aabb,
}

struct Handle {
    id: HandleId,
    node: NodeId,
    angle: f32,       // radians around node circle
    radius: f32,      // visual handle radius
    flags: HandleFlags,
}

struct Edge {
    id: EdgeId,
    from: HandleId,
    to: HandleId,
    curve: CubicBezier,
    flags: EdgeFlags,
    style: StyleId,
    bounds: Aabb,
}
```

For large canvases, prefer **SoA / packed storage** later:

```rust
struct NodeArena {
    ids: Vec<NodeId>,
    xs: Vec<f32>,
    ys: Vec<f32>,
    radii: Vec<f32>,
    flags: Vec<u32>,
}
```

Start with ergonomic structs; migrate hot paths to packed arrays once profiling identifies bottlenecks.

---

## Tangent edge geometry

Handles live on node circles:

```txt
handle position = node.center + node.radius * [cos(angle), sin(angle)]
tangent        = [-sin(angle), cos(angle)]
```

For an edge between two handles:

```rust
fn compute_edge(from: HandleGeom, to: HandleGeom) -> CubicBezier {
    let p0 = from.position;
    let p3 = to.position;

    let d = (p3 - p0).length();
    let k = (d * 0.35).clamp(24.0, 240.0);

    let t0 = from.tangent_oriented_towards(p3);
    let t1 = to.tangent_oriented_towards(p0);

    CubicBezier {
        p0,
        p1: p0 + t0 * k,
        p2: p3 + t1 * k,
        p3,
    }
}
```

When a node moves, recompute only:

1. That node’s bounds.
2. Its handles.
3. Incident edge curves.
4. Affected spatial index entries.
5. Affected tiles.

---

## Tiling and dirty regions

Use **world-space tiles**, not screen-space tiles.

```rust
const TILE_SIZE_WORLD: f32 = 512.0;

struct Tile {
    coord: IVec2,
    dirty: bool,
    visible_last_frame: bool,
    object_ids: Vec<ObjectId>,
    cached_scene: Option<VelloTileScene>,
}
```

Pipeline:

```txt
Mutation
  → compute old bounds + new bounds
  → mark intersecting tiles dirty
  → update spatial index
  → render only visible dirty tiles
  → composite into canvas
```

Do not rebuild the entire Vello scene for every pointer move. Build per-tile display lists or per-visible-region scenes.

Recommended tile strategy:

| Layer         | Contents                             | Update frequency |
| ------------- | ------------------------------------ | ---------------- |
| Static layer  | Grid, background, locked nodes       | Rare             |
| Edge layer    | Bezier curves                        | Medium           |
| Node layer    | Circles, handles                     | Frequent         |
| Overlay layer | selection boxes, hover, drag handles | Very frequent    |

The overlay layer can be rebuilt every frame. The other layers should be tile-dirty.

---

## Spatial index

Use a broad-phase structure:

```rust
struct SpatialIndex {
    nodes: RTree<NodeId>,
    handles: RTree<HandleId>,
    edges: RTree<EdgeId>,
}
```

Hit testing order should be deterministic:

```txt
1. Active drag target
2. Handles
3. Nodes
4. Edges
5. Selection rectangle / background
```

For edges:

1. Query candidate edge bounds from R-tree.
2. Flatten cubic Bezier into line segments at current zoom.
3. Compute minimum distance from pointer to segments.
4. Select if distance <= hit tolerance in screen pixels converted to world units.

```rust
let tolerance_world = 6.0 / camera.zoom;
```

---

## Rendering with Vello

Use Vello for the actual vector drawing:

```rust
fn render_tile(&mut self, tile: TileCoord) {
    let mut scene = vello::Scene::new();

    for edge in visible_edges_in_tile(tile) {
        scene.stroke(... cubic_bezier_path ...);
    }

    for node in visible_nodes_in_tile(tile) {
        scene.fill(... circle ...);
        scene.stroke(... circle outline ...);
    }

    for handle in visible_handles_in_tile(tile) {
        scene.fill(... small circle ...);
    }

    self.renderer.render_scene(scene, tile_target);
}
```

Keep Vello scene generation inside Rust. JS should never assemble paths.

Vello is PostScript/SVG/canvas-like in API scope: shapes, gradients, images, text, and vector paths. ([GitHub][1])

---

## Interaction state in Rust

Pointer interactions should not round-trip through React.

```rust
enum InteractionMode {
    Idle,
    Panning { start: Vec2, camera_start: Camera },
    DraggingNode { ids: Vec<NodeId>, start_world: Vec2 },
    DraggingHandle { handle: HandleId },
    DraggingSelectionRect { start: Vec2, current: Vec2 },
}
```

Rust emits semantic events:

```rust
enum BoardEvent {
    SelectionChanged { ids: Vec<ObjectId> },
    NodeMoved { id: NodeId, x: f32, y: f32 },
    EdgeCreated { id: EdgeId },
    HoverChanged { id: Option<ObjectId> },
}
```

JS drains events after `render()` and dispatches callbacks.

---

# 2. Imperative TypeScript API: `elements/client/lib/board/js/index.ts`

## Shape it like Three.js

Three.js centers around imperative objects, scene graph objects, and explicit rendering; `Object3D` is the base class for most scene objects and stores transforms, visibility, parent/child state, and render-related metadata. ([threejs.org][4]) Three’s renderer API renders a scene with a camera. ([threejs.org][5])

Mirror that pattern, but for 2D boards:

```ts
export class BoardRenderer {
 constructor(options: { canvas: HTMLCanvasElement });

 scene: BoardScene;
 camera: BoardCamera;

 setSize(width: number, height: number, dpr?: number): void;
 render(): void;
 invalidate(): void;
 dispose(): void;
}

export class BoardScene {
 add(object: BoardObject): this;
 remove(object: BoardObject): this;
 getObjectById(id: string): BoardObject | undefined;
}

export class BoardObject {
 id: string;
 visible: boolean;
 selected: boolean;
 draggable: boolean;
 userData: Record<string, unknown>;
 parent: BoardScene | null;

 dispose(): void;
}

export class Node extends BoardObject {
 x: number;
 y: number;
 radius: number;
 handles: Handle[];

 setPosition(x: number, y: number): this;
 setRadius(radius: number): this;
}

export class Handle extends BoardObject {
 node: Node;
 angle: number;
}

export class Edge extends BoardObject {
 from: Handle;
 to: Handle;
}
```

Usage:

```ts
import { BoardRenderer, Node, Handle, Edge } from "elements/client/lib/board/js";

const renderer = new BoardRenderer({ canvas });
const a = new Node({ id: "a", x: 0, y: 0, radius: 48 });
const b = new Node({ id: "b", x: 300, y: 120, radius: 48 });

const ah = new Handle({ id: "a.out", node: a, angle: 0 });
const bh = new Handle({ id: "b.in", node: b, angle: Math.PI });

const e = new Edge({ id: "e1", from: ah, to: bh });

renderer.scene.add(a).add(b).add(e);
renderer.render();
```

---

## JS should batch WASM commands

Avoid this:

```ts
node.x = x
wasm.update_node(...)
node.y = y
wasm.update_node(...)
```

Prefer:

```ts
renderer.batch(() => {
 node.setPosition(x, y);
 node.setRadius(r);
});
```

Internally:

```ts
class CommandBuffer {
 private commands: Float64Array | number[];

 updateNode(id: number, x: number, y: number, r: number) {
  // encode compact op
 }

 flush(engine: WasmBoardEngine) {
  engine.apply_commands(this.ptr, this.len);
 }
}
```

For small MVP size, direct method calls are fine. For high-frequency drag/animation, use a command buffer.

---

## JS owns browser integration

JS layer owns:

| Concern                      | Reason         |
| ---------------------------- | -------------- |
| Canvas DOM element           | Browser object |
| ResizeObserver               | DOM API        |
| Pointer event listeners      | Browser API    |
| RAF scheduling               | Browser API    |
| Worker/OffscreenCanvas setup | Browser API    |
| React bridge                 | JS ecosystem   |

Use `OffscreenCanvas` as an optional backend for worker rendering. MDN documents that OffscreenCanvas can decouple canvas rendering from the DOM and run rendering work in a worker context. ([MDN Web Docs][6])

Recommended modes:

```ts
type RenderMode = "main-thread" | "worker-offscreen" | "headless-test";
```

Start with `"main-thread"`. Add `"worker-offscreen"` after correctness.

---

## Event model

```ts
renderer.on("select", (event) => {});
renderer.on("nodeMove", (event) => {});
renderer.on("hover", (event) => {});
renderer.on("edgeCreate", (event) => {});
```

But JS should not emit every raw pointer movement to React. For dragging:

```txt
pointermove
  → JS calls wasm.pointer_move(...)
  → Rust updates selected nodes
  → Rust marks tiles dirty
  → renderer.invalidate()
  → React receives optional semantic event after frame
```

---

# 3. React layer: `elements/client/lib/board/react/index.tsx`

## Do not make this a wrapper component library

Make it a **React renderer/reconciler**, like `react-three-fiber`.

Target API:

```tsx
import { BoardCanvas, Node, Handle, Edge } from "elements/client/lib/board/react";

export function App() {
 return (
  <BoardCanvas camera={{ x: 0, y: 0, zoom: 1 }}>
   <Node id="a" x={0} y={0} radius={48} draggable selected>
    <Handle id="a.out" angle={0} />
   </Node>

   <Node id="b" x={300} y={120} radius={48} draggable>
    <Handle id="b.in" angle={Math.PI} />
   </Node>

   <Edge id="e1" from="a.out" to="b.in" />
  </BoardCanvas>
 );
}
```

React reconciles this:

```tsx
<Node x={100} />
```

into this imperative call:

```ts
node.setPosition(100, node.y);
renderer.invalidate();
```

React should not render SVG, DOM nodes, or canvas paths.

---

## React package internals

```ts
import Reconciler from "react-reconciler";

const hostConfig = {
 createInstance(type, props, root) {
  switch (type) {
   case "node":
    return new Board.Node(props);
   case "handle":
    return new Board.Handle(props);
   case "edge":
    return new Board.Edge(props);
  }
 },

 appendChild(parent, child) {
  parent.add(child);
 },

 removeChild(parent, child) {
  parent.remove(child);
  child.dispose();
 },

 commitUpdate(instance, updatePayload, type, oldProps, newProps) {
  applyProps(instance, oldProps, newProps);
  instance.renderer.invalidate();
 },
};
```

Expose ergonomic components:

```tsx
export function BoardCanvas(props: BoardCanvasProps) {
 return <CanvasRoot {...props} />;
}

export const Node = "node" as any;
export const Handle = "handle" as any;
export const Edge = "edge" as any;
```

Or use typed component wrappers if you do not want intrinsic JSX types.

---

## React hooks

```ts
export function useBoard(): BoardRenderer;
export function useCamera(): [CameraState, SetCamera];
export function useSelection(): SelectionState;
export function useBoardEvent<T extends BoardEventName>(name: T, handler: BoardEventHandler<T>): void;
export function useFrame(callback: (state: FrameState, dt: number) => void): void;
export function invalidate(): void;
```

Important: `useSelection()` should subscribe to an external store, not React state inside every node.

Use:

```ts
useSyncExternalStore(selectionStore.subscribe, selectionStore.getSnapshot);
```

---

# Key design decision: declarative setup, imperative hot path

Use this split:

| Path                   | Mechanism                                     |
| ---------------------- | --------------------------------------------- |
| Initial board creation | React JSX                                     |
| Prop changes           | React reconciler → JS object mutation         |
| Dragging               | JS pointer event → Rust interaction state     |
| Hit testing            | Rust spatial index                            |
| Selection              | Rust state → JS event → external store        |
| Rendering              | Rust/Vello                                    |
| Animation              | JS RAF + Rust render                          |
| Persistence            | JS serializes scene or asks Rust for snapshot |

This avoids the common failure mode: React rerendering thousands of nodes during drag.

---

# Suggested package structure

```txt
elements/client/lib/board/
  rs/
    Cargo.toml
    src/
      lib.rs
      math.rs
      scene.rs
      render.rs
      tile.rs
      spatial.rs
      hit_test.rs
      interaction.rs
      wasm.rs

  js/
    index.ts
    renderer.ts
    scene.ts
    object.ts
    node.ts
    handle.ts
    edge.ts
    camera.ts
    events.ts
    command-buffer.ts
    wasm-loader.ts

  react/
    index.tsx
    Canvas.tsx
    reconciler.ts
    host-config.ts
    apply-props.ts
    hooks.ts
    store.ts
    jsx-types.ts
```

---

# Rendering pipeline

```txt
1. JS receives pointer/camera/resize event
2. JS forwards compact event to Rust
3. Rust mutates scene/interactions
4. Rust marks dirty world bounds
5. Dirty bounds map to dirty tiles
6. RAF calls renderer.render()
7. Rust computes visible tile range from camera
8. Rust rebuilds only dirty visible tile scenes
9. Vello renders vector layers to canvas
10. Rust emits semantic events
11. JS dispatches events
12. React external stores update only subscribers
```

---

# Data flow

```txt
React props
  ↓
TS object properties
  ↓
Batched WASM commands
  ↓
Rust scene mutation
  ↓
Dirty tiles + spatial index update
  ↓
Vello scene build
  ↓
GPU render
```

---

# Selection model

Represent selection in Rust as object IDs:

```rust
struct Selection {
    nodes: FxHashSet<NodeId>,
    edges: FxHashSet<EdgeId>,
    handles: FxHashSet<HandleId>,
}
```

Expose snapshots to JS:

```ts
renderer.selection.ids;
renderer.selection.has(id);
renderer.selection.subscribe(listener);
```

React usage:

```tsx
const selected = useSelected("node-a");
```

Do not pass `selected` into thousands of nodes as React state unless selection size is small. Prefer style resolution in Rust:

```rust
fn resolve_style(object, selection, hover) -> StyleId
```

---

# Styling

Use style IDs instead of large per-object style objects crossing WASM:

```ts
renderer.styles.define("node.default", {
 fill: "#18181b",
 stroke: "#71717a",
 strokeWidth: 1,
});

new Node({
 id: "a",
 x: 0,
 y: 0,
 radius: 48,
 style: "node.default",
});
```

Rust stores:

```rust
type StyleId = u32;
```

JS maps string style names to numeric style IDs.

---

# Camera and infinite canvas

Use double precision in JS-facing camera state, but Rust rendering can use `f32` relative to a local origin.

```rust
struct Camera {
    x: f64,
    y: f64,
    zoom: f64,
}
```

For render:

```rust
let local_origin = visible_world_rect.center();
let local_x = (world_x - local_origin.x) as f32;
let local_y = (world_y - local_origin.y) as f32;
```

This prevents precision problems when users pan very far from origin.

---

# LOD strategy

At different zoom levels:

| Zoom     | Render                                                |
| -------- | ----------------------------------------------------- |
| Very low | nodes as simple circles, hide handles, simplify edges |
| Medium   | nodes + edges, hide labels                            |
| Normal   | full nodes, handles, edges                            |
| High     | handles, labels, hover affordances, precise strokes   |

This should be Rust-side because the renderer already has camera and tile visibility.

---

# Worker strategy

Phase 1:

```txt
Main thread:
  React
  JS API
  WASM
  Vello render
```

Phase 2:

```txt
Main thread:
  React
  event capture
  semantic events

Worker:
  WASM
  scene
  hit testing
  Vello render
  OffscreenCanvas
```

OffscreenCanvas is transferable and can be rendered from a worker, which is appropriate once the core API is stable. ([MDN Web Docs][6])

---

# MVP order

## Phase 1: correctness

Implement:

```txt
Rust:
  nodes
  handles
  edges
  camera
  hit testing
  full-scene Vello render

JS:
  BoardRenderer
  BoardScene
  Node
  Handle
  Edge

React:
  <BoardCanvas>
  <Node>
  <Handle>
  <Edge>
```

No tiling yet. Full redraw is acceptable for MVP.

## Phase 2: performance

Add:

```txt
dirty bounds
tile index
visible tile culling
spatial index
batched WASM commands
style registry
LOD
```

## Phase 3: production

Add:

```txt
worker/offscreen mode
incremental edge recomputation
text labels
snap lines
selection rectangles
copy/paste serialization
undo/redo command log
benchmark suite
```

---

# Main recommendation

Build it as **three layers with strict ownership**:

```txt
Rust = engine
TypeScript = imperative object API
React = declarative reconciler
```

The important constraint: **React never owns the hot path**. Pointer movement, hit testing, selection mutation, geometry recomputation, tile invalidation, and rendering should stay in Rust/WASM. React should describe the scene and subscribe to semantic state changes only.

---

# Current workspace status (CPU reference vs WASM target)

| Layer | Path | Role today | WASM target |
| ----- | ---- | ---------- | ----------- |
| React descriptors | `elements/client/lib/board/react/index.tsx` | Declarative `<BoardCanvas>` / `<Node>` / `<Handle>` / `<Edge>` markers; `syncBoardScene` preserves stable imperative instances | Same public JSX; optional future `react-reconciler` host |
| Imperative API | `elements/client/lib/board/js/index.ts` | **Main-thread Canvas2D** renderer, hit testing, wheel zoom, optional **`worldRasterTiling: "world-clip"`** (world-space tiles drawn with per-tile clip + bounds cull — scheduler parity hook for Vello tiles) | Thin `wasm-bindgen` batch + event drain |
| Rust engine | `elements/client/lib/board/rs/lib.rs` | Single-crate retained model, pointer + selection, `#[cfg(test)]` parity | `BoardEngine` export surface from §1 |

Vello and wgpu are **not** wired in this repository slice yet; the CPU path exists so Storybook and unit tests can validate semantics before GPU stacks are enabled in CI.

---

# Canvas debug attributes (host harness)

The `<canvas>` inside `BoardCanvas` exposes stable `data-*` hooks refreshed each `render()`:

| Attribute | Meaning |
| --------- | ------- |
| `data-board-raster` | `none` \| `world-clip` |
| `data-board-lod` | `subgrid` \| `grid-only` \| `full` \| `fine` from `resolveBoardLodLabel(zoom)` |
| `data-board-zoom` | Numeric zoom after clamp |
| `data-board-camera` | `x,y` camera world center |
| `data-board-selection` | Comma-separated sorted ids |

These exist for **Playwright** and debugging; gate or remove when a formal inspector ships.

---

# End-to-end verification matrix

From repo root (static Storybook + preview static server — see `test.script.ts` `storybook` slice):

```bash
bun run test:storybook
```

| Automation | Story id | Covers |
| ---------- | -------- | ------ |
| `.storybook/board.spec.ts` | `elements-board--default` | Raster `none`, node selection, zoom-in → `fine` LOD, clear selection |
| `.storybook/board.spec.ts` | `elements-board--default` | Zoom-out → `grid-only` LOD; wheel anchor keeps world point stable (`data-board-camera`) |
| `.storybook/board.spec.ts` | `elements-board--world-tile-clip` | Raster `world-clip`, node + handle hit order |

Focused Vitest (board modules only):

```bash
bunx vitest run --config .repo/🎫/26/05/15/IMPLEMENT-WASM-TILED-VELLO-BOARD/vitest.board.config.ts
```

---

# Reconciliation notes (React without custom reconciler yet)

The React surface today is a **declarative sync layer** (`buildBoardSceneDescriptor` + `syncBoardScene`) rather than a forked `react-reconciler` host. JSX markers return `null`; prop diffs apply to imperative `BoardRenderer.scene` instances.

Promoting to a full reconciler keeps the public JSX stable while swapping internals to `react-reconciler` once interaction parity is frozen; imperative objects remain the source of truth either way.

---

# Open items toward the WASM document

1. Export `BoardEngine` over `wasm-bindgen` matching the coarse WASM API sketch in §1.
2. Move Canvas2D draw passes into Vello scene builders per world tile; reuse `world-clip` tile indices as the scheduler input.
3. Promote LOD gates from `resolveBoardLodLabel` into Rust style resolution keyed by zoom and object kinds.
4. Consolidate hit-testing between JS preview and Rust engine (both exist today for migration).

---

[1]: https://github.com/linebender/vello "GitHub - linebender/vello: A GPU compute-centric 2D renderer. · GitHub"
[2]: https://github.com/pmndrs/react-three-fiber "GitHub - pmndrs/react-three-fiber:  A React renderer for Three.js · GitHub"
[3]: https://github.com/rustwasm/wasm-bindgen "GitHub - wasm-bindgen/wasm-bindgen: Facilitating high-level interactions between Wasm modules and JavaScript · GitHub"
[4]: https://threejs.org/docs/pages/Object3D.html "Object3D - Three.js Docs"
[5]: https://threejs.org/docs/pages/WebGLRenderer.html "WebGLRenderer - Three.js Docs"
[6]: https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas "OffscreenCanvas - Web APIs | MDN"
