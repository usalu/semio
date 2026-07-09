---
name: Puzzle2d Live Peer Sync
overview: Restore commit-93 canvas speed while keeping commit-94 live cross-pane node-move/delete sync, by replacing the per-frame full-scene WASM re-push on peer panes with a lightweight incremental per-node position update in the Rust/WASM BoardSession.
todos:
 - id: ticket
   content: Read repo://goals and open an MCP ticket for the puzzle2d live peer-sync perf fix
   status: completed
 - id: rust-setter
   content: Add set_node_positions host method + setNodePositionsJson wasm_bindgen wrapper in puzzle/2d/rs/lib.rs and rebuild wasm bindings
   status: completed
 - id: js-incremental
   content: Add pendingIncrementalNodeMoves; rewrite applyNodePositionSilent to drop full re-push; flush incremental moves + skip full descriptor sync in pushSceneToWasmDriver
   status: completed
 - id: tests
   content: Extend existing puzzle/2d/react/index.tsx peer-sync test and Rust lib.rs tests to cover the incremental path
   status: completed
 - id: verify
   content: Build wasm, run cargo test + vitest, and manually verify smooth multi-pane drag on the Metabolism fixture
   status: completed
 - id: close-ticket
   content: Close the MCP ticket with summary and touched files
   status: completed
isProject: false
---

# Puzzle2d Live Peer Sync

## Root cause

The triptych play shell drives all panes from one shared declarative `fixture` + `selectionIds` + `sceneAuthoringEpoch` (`framework/product/playground/renderer/react/index.tsx`). Selection synced cheaply because it is a low-frequency shared-state update. Node drags are authored inside WASM and only mutate the local pane, so before commit 94 they never reached other panes.

Commit 94 added the `//#region 🔖MultiViewAuthoring` global `Set<Puzzle2dRenderer>` broadcast in [puzzle/2d/react/index.tsx](puzzle/2d/react/index.tsx). On every `nodeMove` drain event (~60fps during a drag), each peer's `applyNodePositionSilent` does:

```2636:2637:puzzle/2d/react/index.tsx
    this.lastPushedDescriptorJson = null;
    this.invalidate();
```

Nulling `lastPushedDescriptorJson` forces every peer to re-serialize its entire scene via `descriptorJsonForWasmHost()` (O(all nodes/handles/edges/wires)) and re-push it through `session.syncDescriptorJson(...)` every frame. With N panes and the large Metabolism fixture, per-frame cost goes from ~1 full push (commit 93) to N full serializes + N WASM re-syncs -> the jank. The only WASM sync API today is full-descriptor (`syncDescriptorJson`, lib.rs:6768).

## Approach (live, made cheap)

Keep live cross-pane motion, but give peers an O(moved-nodes) path instead of O(scene):

### 1. Rust/WASM: incremental per-node position setter — [puzzle/2d/rs/lib.rs](puzzle/2d/rs/lib.rs)

- Add a host method near `sync_descriptor` (lib.rs:4747) inside its `impl`:
  - `pub fn set_node_positions(&mut self, moves: &[(String, f64, f64)])` that does `if let Some(n) = self.nodes.get_mut(id) { n.x = x; n.y = y; }` for finite coords. Handle/edge/wire geometry is derived from node positions at draw time, so no extra recompute is needed (verify nothing caches node world geometry; if a redraw/dirty flag exists, set it).
- Add a `#[wasm_bindgen(js_name = setNodePositionsJson)]` wrapper next to `sync_descriptor_json` (lib.rs:6768) that parses `[{"id","x","y"}]` and calls `set_node_positions`. Use the existing `#region` structuring.
- Rebuild bindings so the `.d.ts` exposes `setNodePositionsJson` to TS: `bun ./script.ts wasm` in `puzzle/2d/rs`.

### 2. JS renderer: route peer moves through the incremental setter — [puzzle/2d/react/index.tsx](puzzle/2d/react/index.tsx)

- Add `private pendingIncrementalNodeMoves = new Map<string, { x: number; y: number }>();`.
- Rewrite `applyNodePositionSilent` (lib:2617): still update the JS scene node (`node.setPosition`) and `lastNodeAuthoringPositionById`, but record the move in `pendingIncrementalNodeMoves` and `invalidate()` — DROP the `this.lastPushedDescriptorJson = null;` line (this is what forces the full re-push).
- In `pushSceneToWasmDriver` (lib:3660), before the `descriptorJsonForWasmHost()` block (lib:3687-3701): if `pendingIncrementalNodeMoves.size > 0`, flush them via `this.session.setNodePositionsJson(json)`, clear the map, and when `lastPushedDescriptorJson !== null` (no structural/style change pending) skip the full `descriptorJsonForWasmHost()` serialize+sync for this frame. A later non-incremental frame self-heals the cache with one full push (idempotent). The actively-dragged source pane keeps its normal full-push path; only receiving peers use the cheap path.
- Leave the structural delete path (`applyStructuralRemoveSilent`, lib:2640 + `bumpWasmHostSceneMergeResyncEpoch`) as-is — deletes are one-shot/low-frequency, so a single resync per peer is acceptable.

### 3. Verify the broadcast loop stays cheap

`puzzle2dBroadcastNodeMove` (lib:~6961) keeps iterating the global Set, but each peer now does O(moved) work instead of O(scene). The `nearlyEqual` guards in `applyNodePositionSilent` and the `nodeMove` drain case (lib:3764) already prevent feedback loops.

## Tests (extend existing files only)

- Extend the existing `broadcasts nodeMove and structural deletes to peer renderers` test in [puzzle/2d/react/index.tsx](puzzle/2d/react/index.tsx) (~lib:4674): assert the peer position updates AND that a peer drag does not trigger a full `syncDescriptorJson` (spy/count), i.e. it routes through `setNodePositionsJson`.
- Extend the Rust `#[cfg(test)]` tests in [puzzle/2d/rs/lib.rs](puzzle/2d/rs/lib.rs) to cover `set_node_positions` updating an existing node and ignoring unknown ids / non-finite coords.

## Verification

- `bun ./script.ts wasm` in `puzzle/2d/rs` (rebuild bindings), then `cargo test` for the Rust unit tests.
- `bun ./script.ts test` in `puzzle/2d/react` (vitest) for the JS suite.
- Manual: `bun run dev:puzzle:2d`, drag a node in one triptych pane with the Metabolism fixture and confirm all panes move live and smoothly (no jank), plus deletes still propagate.

## Ticket workflow

Open a repo MCP ticket (`ticket_open`) associated with the most appropriate goal from `repo://goals` before editing; put any temp logs/scripts under the ticket folder; close it with `ticket_close` and the touched-file summary when done.
