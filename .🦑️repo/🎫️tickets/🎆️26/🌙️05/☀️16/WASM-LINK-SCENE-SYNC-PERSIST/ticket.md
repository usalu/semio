# Wasm Link Scene Sync Persist

**Status:** Done

**Problem:** WASM link gesture created an edge for one frame, then it disappeared because `syncBoardScene` purged handles before upserting from the merged descriptor (handle removal cascaded edge removal) and `BoardScene.remove(Edge)` cleared `wasmHostAuthoredEdgeIds`, so the next merge no longer re-injected the edge.

**Approach**

1. `syncBoardScene`: upsert nodes, handles, edges first; purge edges, handles, nodes after.
2. `BoardRenderer`: `wasmHostAuthoredLinkByEdgeId` map set on `edgeCreate`; `clearWasmHostAuthorshipForEdge` replaces `forgetWasmHostAuthoredEdge`; merge can rebuild descriptor when the scene edge is gone but endpoints still exist.
3. `BoardScene.remove(Edge)`: do not clear WASM authorship; clear explicitly on handle/node cascade, wasm `edgeDelete`, JSX adoption (merge), descriptor edge purge, dispose.

**Repo MCP:** unavailable in this session (`ticket_open` / `search` not registered); goals read from `repo://goals`; goal association: Running Sketchpad (`r2602`).

**Files:** `elements/client/lib/board/index.ts`, `elements/client/lib/board/index.tsx`, this ticket.

**Verification:** `bun ./📜️script.ts test` in `elements/client/lib/board` (Rust 17, Vitest 51, Playwright 1 passed / 2 skipped).

**Summary:** `syncBoardScene` now upserts nodes/handles/edges before purging orphans so transient handle removal no longer drops WASM links before handles are reconciled; WASM link endpoints are stored for merge recovery; WASM authorship is cleared only where the graph or host intentionally deletes the link (`clearWasmHostAuthorshipForEdge`), not on every `scene.remove(Edge)`.
