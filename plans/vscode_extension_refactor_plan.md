# VSCode Extension Refactor Plan (Streaming-only + Lazy Trees)

**Non-negotiables**
- **Streaming-only command execution**: no “fetch everything then parse”, no request/response style wrappers, no legacy APIs, no compatibility shims.
- **Lazy trees**: hierarchical views (e.g. *Codebase*) **must not** fetch entire trees in one go. Children are fetched **only when a node is unfolded**.
- **Adapters**: this plan focuses on the **VSCode extension** adapter. It assumes the repo tool/engine already exposes commands as **streams of NDJSON events**.

---

## 0) Current problems in `extension.ts` (what to delete)

### 0.1 Output is still effectively request/response
The extension executes the repo command via `execFile` and parses the **entire stdout as a string**:

- `execFileAsync(repoPath, repoArgs, ...)`
- `parseRepoEvents(output: string) -> RepoEvent[]`
- `extractRepoResult(events) -> result`

This is *not* streaming: stdout is buffered until process exit.

### 0.2 Codebase tree is fetched in one go
`loadCodebase()` calls `fetchRepoViaGraphQL()` which runs `RepoDocument` and builds a full `tree` map from the returned repo object. That implies **bundles + folders + files + tickets + contributors** (or similarly large payload) are loaded at once.

Result: slow initial load, heavy memory usage, no incremental UI, and impossible to scale.

### 0.3 URQL GraphQL usage encourages “single response” thinking
URQL `client.query(RepoDocument,{})` expects a single result object. Even if the backend is streaming, the extension currently collapses it into a single result.

---

## 1) Target architecture

### 1.1 A streaming RepoClient (persistent process over stdio)
Replace “spawn per query + buffer output” with a **single long-lived child process**:

- Extension spawns `repo` once (or per-workspace).
- Communication is **NDJSON** over stdin/stdout.
- Each request is an envelope with `id`, and the tool streams back `event`s tagged with the same `id`.

**Protocol (VSCode v2)**
- **Client → server** (one JSON object per line):
```json
{"id":"req-123","command":"codebase.children","input":{"nodeId":"bundle:core"}}
```
- **Cancel**:
```json
{"id":"req-123","cancel":true}
```
- **Server → client** (one JSON object per line):
```json
{"id":"req-123","event":{"kind":"start","command":"codebase.children"}}
{"id":"req-123","event":{"kind":"item","data":{"id":"file:src/a.ts","label":"a.ts","kind":"file","hasChildren":false}}}
{"id":"req-123","event":{"kind":"done","done":{"exit_code":0,"status":"ok"}}}
```

**Rules**
- No final “response” wrapper object.
- Completion is signaled **only** by `event.kind == "done"`.
- Errors are `event.kind == "error"` (fatal or non-fatal), then `done`.

### 1.2 Tree views are backed by a lazy, per-node loader
Every tree node has:
- a stable `id` (string)
- `kind` (`bundle|folder|file|ticket|...`)
- `label`, optional `description`, optional `icon`
- `hasChildren` boolean
- optional `meta` used for commands (e.g. file path)

TreeDataProvider `getChildren(element?)` performs:
- root call when `element == null`
- children call when `element != null` and hasChildren

**Important**: `getChildren()` must never fetch a whole tree. It fetches **only the next level**.

---

## 2) Backend contract required for lazy trees (no legacy)

The extension refactor assumes the repo tool exposes these streaming commands (names can change, but the concept must exist). No compatibility layer.

### 2.1 Codebase commands (streaming)
1) `codebase.roots`
- Streams top-level nodes (bundles/projects).

2) `codebase.children`
- Input: `{ nodeId: string }`
- Streams direct children of that node only.

3) `codebase.node`
- Input: `{ nodeId: string }`
- Streams metadata for one node (optional, used for tooltips/details).

4) `codebase.search`
- Input: `{ query: string, scope?: string }`
- Streams matching nodes as items (used for search UI).

**Event payload for `item`**
```ts
type CodebaseNodeDTO = {
  id: string;
  kind: "bundle" | "folder" | "file" | "symbol" | "ticket" | string;
  label: string;
  description?: string;
  tooltip?: string;
  icon?: string;          // vscode icon name, or semantic kind
  hasChildren: boolean;
  meta?: Record<string, any>;
}
```

This is deliberately small and shallow. No `children` arrays.

---

## 3) VSCode extension refactor steps

### Phase 1 — Delete URQL + GraphQL fetch pathway
**Remove entirely:**
- `@urql/core` usage (`Client`, `cacheExchange`, `fetchExchange`)
- `getUrqlClient()`
- `fetchRepoViaGraphQL()`
- any usage of `RepoDocument` for codebase hydration
- `parseRepoEvents(output: string)` and `extractRepoResult(events)`

**Replace with:**
- `RepoClient` streaming process

Rationale: URQL/GraphQL “single response” mindset is incompatible with “streaming-only, no request/response”.

> If you still need GraphQL for some UI actions, it must become a **streaming command** (e.g. `graphql.query`) behind RepoClient, never URQL.

### Phase 2 — Implement `RepoClient` (stdio streaming bridge)

#### 3.2.1 Create module: `src/repoClient.ts`
Responsibilities:
- Spawn the repo tool process (e.g. `semio repo` or workspace-specific command).
- Maintain a map: `requestId -> RequestState` with:
  - resolve/reject
  - cancel function (AbortController)
  - event listeners
- Read stdout line-by-line, `JSON.parse` each line, route to the matching request.
- Expose **stream subscription**:

```ts
type RepoEvent = { kind: string; data?: any; error?: any; done?: any; message?: string; progress?: any };

type StreamHandle = {
  id: string;
  onEvent(cb: (ev: RepoEvent) => void): () => void;
  cancel(): void;
  finished: Promise<void>; // resolves when done received
};

class RepoClient {
  stream(command: string, input: any): StreamHandle;
  dispose(): Promise<void>;
}
```

**Notes**
- The handle is “stream-first”; `finished` exists only to let VSCode await completion when required.
- `finished` does **not** return a result object.

#### 3.2.2 Backpressure & stability
- Buffer writes to stdin if needed.
- Cap maximum in-flight requests (e.g. 32); reject with a friendly error.
- On process crash, fail all active requests, then auto-restart on next call (or prompt user).

#### 3.2.3 Cancellation
- `cancel()` sends `{"id":"...","cancel":true}`.
- Also calls local AbortController to stop local listeners.

### Phase 3 — Refactor Codebase tree to lazy loading

#### 3.3.1 Delete `loadCodebase()` and full-repo cache
Remove:
- `cachedCodebase`
- `codebaseLoadPromise`
- `loadCodebase()`, `refreshCodebase()`, and anything that requires `repo.files`, `repo.folders` arrays.

Replace with:
- A light `CodebaseTreeCache` that caches **children lists per node**.

#### 3.3.2 Create `CodebaseNode` model for TreeDataProvider
```ts
class CodebaseNode extends vscode.TreeItem {
  constructor(
    public readonly nodeId: string,
    public readonly kind: string,
    label: string,
    public readonly hasChildren: boolean,
    public readonly meta?: Record<string, any>,
  ) {
    super(label, hasChildren ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None);
  }
}
```

#### 3.3.3 Provider: `CodebaseProvider implements TreeDataProvider<CodebaseNode>`
Key behavior:
- `getChildren(undefined)`:
  - if root cached, return it
  - else start streaming `codebase.roots`
- `getChildren(node)`:
  - if `childrenCache.has(node.nodeId)` return cached children
  - else stream `codebase.children {nodeId}` and cache as items arrive

**Important**: VSCode expects `getChildren()` to return a Promise of the final array. But we can still be streaming-first:
- while streaming items arrive, store them incrementally
- call `this._onDidChangeTreeData.fire(node)` to prompt VSCode to ask `getChildren(node)` again
- return partial results quickly (first render), then update as more items arrive

This gives “streaming UI” without fetching everything.

#### 3.3.4 Implement “loading” placeholders
When a node is expanded and children are not yet ready:
- return `[new LoadingNode("Loading…")]`
- once first child arrives, replace placeholder and fire change event

This keeps the UI responsive even for large directories.

#### 3.3.5 Avoid duplicate fetches
Maintain `inFlightChildren: Map<string, StreamHandle>`:
- if already in flight, do not start another stream
- use same cached incremental results

#### 3.3.6 Cache invalidation strategy
- Invalidate children cache when:
  - workspace changes
  - repo refresh command executed
  - file watcher triggers (optional)
- Provide `refresh()` that clears caches and fires `onDidChangeTreeData`

### Phase 4 — Lazy tickets / contributors / policies trees too
Your extension has multiple providers (`TicketsProvider`, `PoliciesProvider`, `ContributorsProvider`, etc.) that currently cache “full lists”. Apply the same pattern:

- Root list fetch is allowed (bounded), but avoid “fetch all nested data”.
- For nested items (like ticket commits, violation kinds):
  - fetch on expansion only (children command)
  - cache per parent id

Example:
- `ticket.list` streams tickets
- `ticket.commits` streams commits for ticket id
- `policy.list` streams policies
- `policy.violationKinds` streams kinds for policy id

### Phase 5 — Remove any command helpers that aggregate output
If you currently have helpers like:
- `runRepoCommandJson<T>(...)` that returns a parsed object
- `runRepoCommand(...)` that collects all events

Replace with streaming helpers only:
- `streamCommand(command,input,handlers)`.

If you need a “single object” for a specific UI action, you can implement a *local* accumulator at the callsite (not a shared API), but the transport and command remain streaming. For example:
- “Open details view” may collect `item` events into a list, but never by running a “bulk tree fetch”.

---

## 4) Tree UX requirements: unfolding-only fetch

### 4.1 Codebase tree must be shallow per call
**Strict requirement**: Each `codebase.children` call returns only direct children:
- For folder nodes, only immediate children entries.
- No recursive expansion, no “flattened list of all descendants”.

### 4.2 Pagination for huge folders (optional but recommended)
If a directory can contain 50k entries:
- `codebase.children` should accept `{ nodeId, cursor?, limit? }`
- stream `item`s, then `done` includes `nextCursor` in `done.meta` (or a `result` event containing `cursor`)

In VSCode:
- show a `“Load more…”` virtual node when `nextCursor` exists
- expanding “Load more…” triggers another `codebase.children` with cursor

Still streaming-only; the extension never aggregates the entire folder.

### 4.3 Search should not prefetch
Search uses `codebase.search` command:
- streams matched nodes
- selecting a match can reveal its parent path by lazy fetching each segment (optional)

---

## 5) Telemetry & logging (developer ergonomics)
- Add a `RepoClient` debug channel output:
  - log every `start/error/done` and timing
  - keep it behind a user setting: `semio.debug.streaming`

- Display status bar progress:
  - show “Loading children…” while a stream is active for an expanded node
  - clear on `done`

---

## 6) Concrete file/module plan

### 6.1 New modules
```
src/
  repoClient.ts          // persistent streaming process + routing
  protocol.ts            // event types + guards + parseLine()
  trees/
    codebaseProvider.ts  // lazy codebase tree provider
    loadingNode.ts
    cache.ts             // generic per-node cache + in-flight registry
```

### 6.2 `extension.ts` changes
- Keep activation/registration glue in `extension.ts`.
- Move providers into separate files.
- Replace all command execution paths with RepoClient streaming calls.

---

## 7) Testing plan

### 7.1 Unit tests (Node)
- `protocol.parseLine()` handles:
  - valid NDJSON
  - partial lines
  - malformed JSON (emits local error event)
- `RepoClient` routes events to correct request id.
- cancellation sends cancel message and completes stream.

### 7.2 Provider tests (mock RepoClient)
- Expanding a node triggers exactly one `codebase.children` stream.
- Items appear incrementally:
  - first `item` causes `fire(node)`
  - subsequent `getChildren(node)` returns more items
- Cache prevents duplicate streams.

### 7.3 Manual QA checklist
- Expanding root loads bundles without freezing.
- Expanding large folder shows “Loading…” then streams entries.
- Collapsing a node does not trigger re-fetch unless refresh.
- Refresh clears caches and refetches on next expand.
- Cancel in-flight request when user triggers refresh.

---

## 8) Deletions checklist (no legacy, no compat)

**Remove completely**
- URQL client + exchanges + `local://graphql` fetch bridge
- `parseRepoEvents(output: string)` and any “collect whole output then parse”
- `loadCodebase()` full-tree fetch and `cachedCodebase`
- Any `--format json` / non-streaming formats used by the extension
- Any protocol version flags (document and implement only the new protocol)

**Replace with**
- streaming RepoClient
- lazy tree providers with per-node caching and incremental updates

---

## 9) End state definition

After this refactor:
- The VSCode extension communicates with the repo tool via a **single streaming protocol** (NDJSON over stdio).
- The extension **never** fetches the entire codebase tree. It requests:
  - roots on initial render
  - children only on expansion
  - optional paging via “Load more…”
- No backwards compatibility code remains: no old URQL path, no buffered execFile parsing, no legacy tree hydration.

