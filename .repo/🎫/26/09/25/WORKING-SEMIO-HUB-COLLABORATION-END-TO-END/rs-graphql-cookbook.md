# 🦀 rs GraphQL Cookbook — semio control plane

Researched 2026-09-25 (read-only agent, no source edits). **The working tree is being edited concurrently by other agents in this ticket** (F1/F2/H/M). Every finding below is either (a) a static read of `semio/client/lib/rs/lib.rs` with an exact line ref, re-checked at the *end* of this research pass, or (b) a live `cargo build` + `curl` transcript, timestamped, against the exact commit/diff state at that moment. Where the source changed *during* this research (it did, twice, on the exact topics this cookbook covers — see §5), both the old (build-verified) and new (static-only, not yet build-verified) states are given, clearly labelled. **At the moment this file was written, `cargo build -p semio-store` is RED** (unrelated WIP by another agent adding `DesignDiff.connections`, see §7). Re-run §7's commands after pulling the latest tree before trusting any live number here.

Base ref at last read: git commit `277881d` (2026-06-04) + uncommitted local edits (`git status` shows `M semio/client/lib/rs/lib.rs`).

---

## 1. Native Rust: open a store, execute/subscribe GraphQL

Crate `semio` (`semio/client/lib/rs/Cargo.toml`, path `semio/client/lib/rs/lib.rs`). Single runtime type: `worker::ParentStore` (lib.rs:13763). It hosts two child actors (`wip`, `auth`) over `async_channel::unbounded<Command>`, a shared `EventBus` (blake3-free broadcast bus, lib.rs:13615), and the async-graphql schema is built *around* an `Arc<ParentStore>` injected as GraphQL context data — there is no separate "database"; the `ParentStore` **is** the store.

### 1.1 Construct a runtime (3 ways, all `pub`)

```rust
// lib.rs:13778 — empty in-memory kit ("dev://empty" only)
let rt: Arc<ParentStore> = ParentStore::spawn().await;

// lib.rs:13811 — WASM/host bootstrap: hydrate WIP graph directly from a kit-projection JSON value
// (authoritative line stays empty). This is what KitStoreHandle.create + installProjection use together.
let rt = ParentStore::spawn_wip_overlay_from_initial_kit_projection_json(json_value).await?;

// lib.rs:13843 — what `semio-store`'s POST /install uses: accepts EITHER
//   { "schema": "semio.kit-store-bundle/v1", ... }   (DevBackboneBundleDoc, full change history)
//   OR a bare initialKit projection object (same shape installProjection takes)
let rt = ParentStore::spawn_from_install_json_value(json_value).await?;

// lib.rs:13856 — hydrate an ALREADY-RUNNING runtime's WIP graph (this is what the
// GraphQL `installProjection` mutation resolver calls, lib.rs:17143)
rt.install_projection_json(json_str).await?;
```

`ParentStore` fields you'll want (lib.rs:13763): `bus: Arc<EventBus>`, `wip_graph: Arc<Graph>`, `auth_graph: Arc<Graph>`, `sessions: RwLock<Vec<Arc<Session>>>`.

### 1.2 Execute a GraphQL document (sync-looking, actually async)

Native HTTP host pattern — **exactly** what `semio/client/bin/store/bin.rs:166-172` does, and the pattern any embedder (hub) should copy verbatim:

```rust
use semio::gql;

// body: &str = r#"{"query":"...","variables":{...},"operationName":null}"# (see §1.4 for the JSON shape)
let mut req = gql::graphql_request_from_json_str(body)?;      // lib.rs:18377 — parses {query,variables?,operationName?}
req = req.data(rt.clone()).data(rt.bus.clone());               // both MUST be injected — resolvers pull Arc<ParentStore> AND Arc<EventBus> from ctx
let schema = gql::build_schema_for(rt.clone());                // lib.rs:18366 — cheap, schema is just a thin wrapper, safe to rebuild per request
let resp = schema.execute(req).await;                          // async_graphql::Response
let json = serde_json::to_value(async_graphql::Response::from(resp))?;
```

`build_schema_for` is stateless/cheap (no caching needed) — `semio-store` rebuilds it on every POST /graphql (bin.rs:171). The WASM `KitStoreHandle` caches it once in a `Mutex<Option<AppSchema>>` (lib.rs:18472) purely to avoid rebuilding on every call from JS, not for correctness.

### 1.3 Subscriptions — native in-process only, NOT over semio-store's HTTP

`Subscription` root type (lib.rs:18012) has exactly two fields: `session` (mirrors `Query.session`, live-updates on bus events) and `operation` (emits `OperationInterface` — the live per-mutation event feed the hub protocol's websocket `operation {...}` frame should be sourced from). Both use `ctx.data::<Arc<EventBus>>()` + `bus.subscribe_paths(watched)` (path-filtered by GraphQL selection-set via `collect_subscription_field_paths`, lib.rs:14047) or `bus.subscribe()` (unfiltered).

To consume a subscription **natively** (this is what the hub must do — embed `semio` as a lib, not shell out to `semio-store`):
```rust
let mut req = gql::graphql_request_from_json_str(subscribe_body)?;
req = req.data(rt.clone()).data(rt.bus.clone());
let schema = gql::build_schema_for(rt.clone());
let mut stream = schema.execute_stream(req);          // futures_util::Stream<Item = async_graphql::Response>
while let Some(resp) = stream.next().await { /* forward as one 'operation' websocket frame */ }
```
This is exactly the WASM path (`KitStoreHandle.subscribe`, lib.rs:18508-18537) minus the JS callback marshaling.

**`semio-store`'s HTTP surface (`bin.rs`) does not expose subscriptions at all** — routes are only `GET /healthz`, `GET /graphiql`, `POST /graphql` (single execute, no SSE/WS), `POST /install`, `POST /server/shutdown` (bin.rs:204-213). If the hub wants a subscription feed, it must embed the `semio` crate directly and call `schema.execute_stream()` itself (as above) — going through `semio-store` over HTTP is a dead end for live events. `HttpStringTransport.subscribe()` in `@semio/js` (index.ts:494) is literally a no-op today (`return;` — no-op, matches this gap).

### 1.4 Wire JSON shape (identical native ↔ WASM ↔ JS)

```json
{ "query": "mutation { ... }", "variables": { "...": "..." }, "operationName": null }
```
Parsed by `gql::graphql_request_from_json_str` (lib.rs:18377, native/HTTP) and the WASM-local twin `graphql_execute_request_from_str` (lib.rs:18428). Both: `query` required string, `variables` optional (skipped if null), `operationName` optional non-empty string. `@semio/js` builds this exact shape via `graphqlWirePostBodyJson` (index.ts:314-320) — see §4.

---

## 2. Kit projection: install, export, cheap hash

### 2.1 Install (`installProjection`) — accepted JSON

GraphQL: `StoreCommand.installProjection(json: String!): Response!` (schema.golden.graphql:9769; resolver lib.rs:17143-17150 → `ParentStore::install_projection_json`, lib.rs:13856).

Accepts **either**:
1. A bare `initialKit` projection object — top-level `{ id, name, version, createdAt, updatedAt, description, icon, image, remote, homepage, license, preview, tags, concepts, qualities, files, folders, authors, families, hash, typologies }` where `typologies` is `{ hash, items: [{ id, name, types: {hash, items:[...]}, designs: {hash, items:[{id,name,hash,createdAt,updatedAt,typology,families,pieces?}]} }] }` — this is literally `semio/fixtures/stores/metabolism/wip/initialKit/kit.semio.json` (verified live, §7).
2. A full bundle doc: `{ "schema": "semio.kit-store-bundle/v1", ... }` (constant `KIT_STORE_BUNDLE_SCHEMA`, `DevBackboneBundleDoc`, lib.rs:12836) — carries `wip.initialKit` + full unsaved/saved `changes[].edits[].forwards/backwards` operation history, used for full replay/round-trip tests. `semio-store`'s `POST /install {"create":{"dto": <either shape>}}` accepts both (bin.rs:59-99, dispatches on presence of `"schema"` field).

Both native (`ParentStore::spawn_from_install_json_value` / `install_projection_json`, lib.rs:13843/13856) and GraphQL do the *same* dispatch: presence of `json.schema == KIT_STORE_BUNDLE_SCHEMA` picks the bundle path (`DevBackboneBundleDoc::hydrate_into_graph`), else the bare-projection path (`kit_backbone::hydrate_kit_from_initial_projection_value`, `pub(crate)` — see caveat below).

Exact JS call (`@semio/js`, index.ts:2103-2110 / used by `Store.installProjection`, index.ts:2596-2598):
```graphql
mutation($storeId: ID!, $json: String!) {
  session { store(id: $storeId) { installProjection(json: $json) { ok errors { kind message requestId } result { ... on IdResult { value } } } } }
}
```
`{ "storeId": "e0", "json": "<the whole kit JSON, stringified>" }`. `installProjection` refuses a second install on the same `ParentStore` at the HTTP-sidecar level (`POST /install` → 409 if already installed, bin.rs:107-111); the GraphQL mutation itself has no such guard (it just re-hydrates `wip_graph` in place, lib.rs:13856-13875 — clobbers current WIP content, no merge).

### 2.2 Export the current projection back — fixed while this research was underway

Two states again (§0 caveat applies): the export path (`kit::Kit::projection_value(&self) -> serde_json::Value`, lib.rs:6135, delegating to `pub(crate) kit_backbone::initial_kit_projection_value`, lib.rs:12034) was **not** reachable from GraphQL when first checked — no `exportProjection`/`kitJson`/`toProjection` field existed anywhere. **As of the latest read it now is:** `Kit.projection: String!` (lib.rs:6777-6780, `pub async fn projection(&self) -> String { self.projection_value().await.to_string() }`) — a real GraphQL field, same canonical JSON `installProjection` accepts, so `installProjection(json: kit.projection)` round-trips. Query it the same way as `hash` (§2.3):
```graphql
query($storeId: ID!) { session { stores { edges { node { wip { theKit { kit { id hash projection } } } } } } } }
```
(cursor-filter client-side on `edges[].cursor == storeId`.) This is also `pub` for native embedders that don't want to go through GraphQL at all (`rt.wip_graph.materialized_head_kit_from_ref().await` then `.projection_value().await` — no GraphQL round-trip needed in-process). Not yet build-verified end-to-end (§7.3) — the fixture-observed gap that `connections` was always serialized as `{ "hash": KIT_BUNDLE_HASH_STUB, "items": [] }` (lib.rs:12189, always empty) is exactly what the in-flight `DesignDiff.connections` work (§3.2 stop-press, §7.3) is closing; re-check after a green build.

### 2.3 Cheap kit hash — **just fixed while this research was underway; verify after rebuild**

Two states observed in the same research session (source is being live-edited, §0):

- **State A (build-verified live, §7.1/§7.2):** `Kit::compute_hash()` = `hash::h(&[kid.as_str(), name.as_str()])` (id + name only, lib.rs read at the time of the first successful build). Confirmed by curl: adding a piece to a design changed **nothing** in `kit.hash`, `kit.hasDesigns.hash`, or the design's own `hash` (see exact before/after values in §7.2).
- **State B (current static source, NOT yet re-verified by a green build — see §0/§7.3):** `Kit::compute_hash()` is now `crate::kit_backbone::canonical_json_hash(&self.projection_value().await)` (lib.rs:6130-6132) — i.e. blake3 over the canonical JSON of the *entire* projection. This is a real, cheap (one JSON walk + one blake3 hash), deterministic, full-content hash — **if it survives the current red build**.

Either way, `Kit.hash` is reachable with a tiny query:
```graphql
query($storeId: ID!) { session { stores { edges { node @include(if: true) { wip { theKit { kit { id hash } } } } } } } }
```
(cursor-filter client-side on `edges[].cursor == storeId`, or just take `edges[0]` when there is one store — `sessionStoreNodeFromData` in `@semio/js`, index.ts:711-715, does exactly this).

**Important nuance even under State B:** only the *root* `Kit.hash` got the content-hash treatment. Every other entity's own `hash` GraphQL field observed in this pass is still the shallow `h(&[id, name])` (or, for `Family`/`Typology`, `h(id,name,description,icon,folder_id)`) form — see §5 for the full list and exact line numbers. So `kit.hasDesigns.edges[].node.hash` and `kit.design(id).hash` will **not** move when a piece is added/moved/connected even after the Kit-root fix, because `DesignConnection`'s merkle rollup (`entity_relay!` macro, lib.rs:79-95) folds in each design's own (still-shallow) `compute_hash()`, not the new projection-based one. Only the single top-level `Kit.hash` scalar is trustworthy for "did anything in this kit change" today (assuming State B compiles) — don't rely on any nested `.hash` field for that purpose.

---

## 3. Mutation catalog (kit-changing commands sketchpad/JS actually uses)

**Everything nests under one root:** `Mutation.session: SessionCommand` (lib.rs:18002-18010, only field) → `SessionCommand.store(id: ID!): StoreCommand` (lib.rs:17106) → `StoreCommand.theKit: VersionCommand` (lib.rs:17131) → `VersionCommand.startNewChange` opens a transaction (returns a change/tx id) → `VersionCommand.unsavedChange(id): UnsavedChangeCommand` re-enters the *same* transaction → `.kit: OperationInput` is where every domain mutation actually lives → `.save` commits. There is no other mutation entry point; `Query.node`/`Query.entity`/`Query.session` are read-only.

### 3.1 Transaction lifecycle (every mutation is wrapped in this)

```graphql
# 1. open a change (transaction) — GQL_RESPONSE_SELECTION = "ok errors { kind message requestId } result { ... on IdResult { value } }"
mutation($storeId: ID!) { session { store(id: $storeId) { theKit { startNewChange { ok errors { kind message requestId } result { ... on IdResult { value } } } } } } }
# variables: { "storeId": "e0" }
# -> result.value is the CHANGE id (tx id), NOT any domain entity id.

# 2. apply N domain operations against that change (see §3.2), e.g.:
mutation($storeId: ID!, $changeId: ID!) {
  session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit {
    op1: rename(newName: "New Name") { ok errors { kind message requestId } result { ... on IdResult { value } } }
  } } } } }
}
# variables: { "storeId": "e0", "changeId": "<tx id from step 1>" }

# 3. commit
mutation($storeId: ID!) { session { store(id: $storeId) { theKit { save { ok errors { kind message requestId } result { ... on IdResult { value } } } } } } }
```
This is exactly `Session.ensureChangeId` / `Session.mutateScoped` / `Session.saveChange` in `@semio/js` (index.ts:2049-2110) and `scopedKitMutationBody` (index.ts:660-664) — the JS layer never deviates from this shape; every auto-installed operation method (`installKitOperationMethods`, index.ts:1014-1029) produces `this.mutateScoped(cid, this.kitInnerPath(spec.buildInner(...)))` which is *this exact* 3-step wrapper (steps 1 and 3 are separate calls the caller must sequence itself via `ensureChangeId()`/`saveChange()`; only step 2 is auto-built per-operation).

**No explicit undo/redo mutation exists.** `to_backwards()` computes backward operations and they're recorded per-`Edit` (`redo_edits`, `the_kit_redo_edits` fields, lib.rs:7567/7683 — bookkeeping only), but there is no `undo`/`redo` GraphQL field anywhere in `schema.golden.graphql` or the code-first schema. `VersionCommand.createCheckpoint` also exists in the schema but is `not_implemented()` (lib.rs:17206-17209, `SessionCommand.end` likewise not_implemented at lib.rs:17008).

### 3.2 `OperationInput` (`kit { ... }`) — full mutation roster, wiring status, id-determinism

`Operation` enum (lib.rs:9229-9255) has exactly **24 variants**. GraphQL wiring status per operation (checked against the *current* source, since this is exactly the area under concurrent edit):

| GraphQL mutation | Operation variant | Wired? | Client `id` param? | Notes / line |
|---|---|---|---|---|
| `rename` (kit) | `RenameKit` | ✅ | n/a (singleton) | lib.rs:17263 |
| `changeDescription` (kit) | `ChangeDescription` | ✅ | n/a | lib.rs:17268 |
| `createTag` | `CreateTag` | ✅ | **`id: Option<Id>`** (new, lib.rs:17281) | server falls back to `Id::new()` if omitted |
| `tag(id).rename/changeDescription/changeIcon/addAttribute/removeAttribute(s)` | `RenameTag`/… | rename+changeDescription ✅, rest `not_implemented` | — | lib.rs:17563-17609 area |
| `deleteTag`/`deleteTags` | `DeleteTag(s)` | ✅ | — | |
| `createConcept` | `CreateConcept` | ✅ | **`id: Option<Id>`** | lib.rs:17280 (region) |
| `concept(id).*` | — | only `changeDescription` ✅, rest `not_implemented` | — | |
| `deleteConcept`/`deleteConcepts` | `DeleteConcept` | ✅ | — | |
| `createQuality` | `CreateQuality` | ✅ | **`id: Option<Id>`** | |
| `quality(id).*` | — | **all `not_implemented`** | — | |
| `deleteQuality`/`deleteQualities` | `DeleteQuality` | ✅ | — | |
| `createType` | `CreateType` | ✅ | **`id: Option<Id>`** | |
| `type(id).*` (rename, changeDescription, changeIcon, addAttribute, removeAttribute(s), **createPort**) | — | **all `not_implemented`** | — | lib.rs:17617-17656 area |
| `deleteType`/`deleteTypes` | `DeleteType` | ✅ | — | |
| `createDesign` | `CreateDesign` | ✅ | **`id: Option<Id>`** | |
| `design(id).rename/changeDescription/changeIcon/flatten/addAttribute/removeAttribute(s)` | — | **all `not_implemented`** | — | lib.rs:17758-17792 |
| `design(id).addFixedPiece` | `CreateFixedPiece` | ✅ | **`id: Option<Id>`** (new, lib.rs:17857) | position/name/description required-ish inputs |
| `design(id).addChildPieceWithParentConnection` | — | **`not_implemented`** | — | this is the only mutation shaped like "create a piece + a connection at once" and it's stubbed |
| `design(id).addHangingChildPieceWithParentConnection` | — | **`not_implemented`** | — | ditto |
| `design(id).piece(id).rename/changeDescription/move/fix/changeBlueprint/addAttribute/removeAttribute(s)` | — | only **`drag`** ✅ (→ `DragPieceInDesign`), rest `not_implemented` | — | lib.rs:17903-17958 |
| `design(id).pieces(ids).move/fix/changeBlueprint` | — | only **`drag`** ✅ (→ `DragPiecesInDesign`), rest `not_implemented` | — | lib.rs:17968-17998 |
| `design(id).deletePiece`/`deletePieces`/`deletePiecesAndConnections` | `DeletePieceInDesign` exists on the `Operation` enum but **the resolvers are `not_implemented`** | ❌ | — | lib.rs:17853-17866 — domain logic exists (`to_diff`/`apply_kit_operation` handle it), GraphQL wiring doesn't call it |
| `createFolder` | `CreateFolder` | ✅ | **`id: Option<Id>`** | |
| `moveToFolder` | `MoveToFolder` | ✅ | — (moves existing node, no new id) | |
| `deleteDesign`/`deleteDesigns` | `DeleteDesign` | ✅ | — | |
| *(no field at all)* | — | there is **no `Connection`-creating mutation anywhere** in the schema; `type Connection` (schema.golden.graphql:7006) is query-only | — | confirmed no `connect`/`createConnection` string anywhere in schema.golden.graphql or lib.rs mutation surface |

**Practical read for the hub's MCP tool list (`connect_pieces`, `remove_connections`, `update_piece`) — STOP-PRESS, re-checked after the table above was written, source moves fast:** `add_piece` → `addFixedPiece` (works, optional client id). As of the *very latest* read, `DesignOperationInput` in source (lib.rs:18436-18480) now **also** has `connectPieces(id: Option<Id>, parentPieceId, parentConnector, childPieceId, childConnector, joint: ConnectionJointInput)`, `deleteConnections(ids)`, `deletePiece(id)`, `deletePieces(ids)`, `deletePiecesAndConnections(pieceIds, connectionIds)` (all delegate to a shared `dispatch_design_deletion`/`dispatch_design_items` pair, and deleting a piece correctly cascades to its connections per the docstring), and `addChildPieceWithParentConnection` is implemented (creates a piece + a connection atomically via `PieceAdded`/`ConnectionAdded`). `ConnectionJointInput { gap, shift, rise, rotation, turn, tilt, u, v: Option<f64> }` (all default 0). **This closes essentially the entire gap this cookbook's table above documents** — but it landed *while this cookbook was being written* (see §7.3: the tree does not currently compile because of it — `DesignDiff` is missing the new `connections` field in three call sites). Treat the table above as "true as of a green build a few minutes ago"; treat this paragraph as "true in source right now, not yet build-verified." Re-run §7's build before trusting either for a real integration. `update_piece` (rename/description/blueprint-change) still has no matching table update confirmed — spot-check `PieceOperationInput` again once the tree is green.

### 3.3 Exact copy-pasteable example: create a design + add a fixed piece with a client-chosen id

```graphql
mutation($storeId: ID!, $changeId: ID!, $designId: ID!, $pieceId: ID!, $typeId: ID!) {
  session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit {
    cD: createDesign(id: $designId, name: "My Design") { ok errors { kind message requestId } result { ... on IdResult { value } } }
    afp: design(id: $designId) {
      addFixedPiece(id: $pieceId, blueprintId: $typeId, position: { center: { u: 1.5, v: 2.5 }, plane: { origin: {x:0,y:0,z:0}, xAxis:{x:1,y:0,z:0}, yAxis:{x:0,y:1,z:0} } }, name: "my-piece") {
        ok errors { kind message requestId } result { ... on IdResult { value } }
      }
    }
  } } } } }
}
```
variables: `{ "storeId": "e0", "changeId": "<tx>", "designId": "<client-generated uuid>", "pieceId": "<client-generated uuid>", "typeId": "<existing type id>" }`.

**Caveat verified live (§7.2) and still true in the current source (`dispatch_wip_wait`, lib.rs:13928-13956 unchanged):** even when you pass an explicit `id`, the mutation's own `result.value` is **not** that id — it's an unrelated internal `request_id` minted fresh per call (`CommandResponse::ok_request(request_id)`, which is just `ok_id(request_id)`, lib.rs:10832-10840). If you rely on the response to learn "what id did the server pick" (relevant when you *don't* pass one), you will get the wrong value. The only reliable way to find a server-picked id is to re-query the collection afterward and diff, or (correctly) *always* pass your own id.

---

## 4. The single mutation choke point in `@semio/js` (for hub interception)

**Update: this is a moving target and it moved for the better while this research was running.** Two states, both confirmed by direct reads of `semio/client/lib/js/index.ts` (5772+ lines) minutes apart:

**State A (what a plain grep of `executeGraphql(` call sites shows, and what §3/§6 of this cookbook were written against):** every read *or* write funnels through two *private* one-liner `Session` methods, `readEnvelope`/`mutateEnvelope` (previously index.ts:1937-1947), both calling `executeGraphql(this.handle, body, this.timeoutMs)` — the literal bottom (index.ts:730-737): `handle.execute(graphqlWirePostBodyJson(body))`, `this.handle = { execute: (j) => inner.execute(j) }` set once in the constructor, `inner: WorkerStringTransport | InlineTransport | HttpStringTransport` (index.ts:502, a closed union of un-exported concrete classes). Code outside `Session` reached `mutateEnvelope` via a privacy-bypassing cast, `executeSessionWriteGraphql`/`executeSessionReadGraphql` (index.ts:2515-2527: `(session as unknown as {mutateEnvelope(b):...}).mutateEnvelope(body)`).

**State B (current, live in the file right now — F2 already built this, region `🎛️KitOperations`, index.ts:1826-1968):** a proper, purpose-built, *already-exported* choke point superseding State A:
```ts
export type KitOperation = Readonly<{ operationId: string; query: string; variables: GraphqlVariables; origin: "local" | "remote" }>;
export type KitOperationExecutor = (operation: KitOperation) => Promise<KitOperationEnvelope>;
export type KitOperationMiddleware = (operation: KitOperation, next: KitOperationExecutor) => Promise<KitOperationEnvelope>;

class Session {
  useOperationMiddleware(mw: KitOperationMiddleware): Unsubscribe   // index.ts:1932 — composes in registration order
  onOperation(listener: (op: KitOperation, envelope) => void): Unsubscribe  // index.ts:1941 — fires after every executed op, local or remote
  async executeOperation(operation: KitOperation): Promise<KitOperationEnvelope> {  // index.ts:1949 — THE choke point now
    assertGraphqlWireKind(operation.query, "mutation");
    const local = (op) => executeGraphql(this.handle, { query: op.query, variables: op.variables }, this.timeoutMs);
    const run = this.operationMiddlewares.reduceRight((next, mw) => (op) => mw(op, next), local);
    return run(operation);   // then fan out to onOperation listeners
  }
  private async mutateEnvelope(body) {  // now just a thin shim over executeOperation, kept for old call sites
    return this.executeOperation({ operationId: newKitOperationId(), query: body.query, variables: body.variables ?? {}, origin: "local" });
  }
}
```
`readEnvelope` is separate and unreplicated by design (index.ts:1920-1925, now a `pub` method, docstring: *"never replicated"*) — reads never go through `executeOperation`. Every kit-changing mutation (every auto-installed operation method, `Store.installProjection`, etc.) still routes through `mutateEnvelope` → `executeOperation`, so **`executeOperation` is now the single, intentionally-designed, already-public choke point** — no cast hack needed, no closed-union transport problem to route around.

**This is not hypothetical — F2 already wired hub forwarding through it.** `HubReplica` (index.ts:5833+) does exactly `this.store.session.useOperationMiddleware((operation, next) => this.forward(operation, next))` (index.ts:5861) to intercept every local kit operation, forward it to the hub, rebind `storeId`/`changeId`, replay remote operations arriving over the hub websocket, confirm own echoes by `operationId`, queue while offline, and `resync()` from the hub's authoritative projection on hash mismatch (`localHash()` at index.ts:5918-5921 reads `store.readKitInner("hash")` — i.e. `Kit.hash`, see §2.3/§5 for what that number currently means). This class already implements the entire "hub client" half of important.md's protocol; if you are building H (the hub) or M (MCP), read `HubReplica` (index.ts:5833-~6100) directly rather than re-deriving the wire contract from important.md's prose — it's the executable spec.

**Confirmed unchanged in both states: mutations are sent as full `{query, variables}` documents** (`operationName` dropped for operations specifically — `KitOperation` has no `operationName` field), matching the hub protocol's `POST /sessions/{id}/operations {operationId, clientId, baseVersion, query, variables}` shape field-for-field once you add `operationId`/`baseVersion`/`clientId` (which `HubReplica.forward` does — read it for the exact envelope). No client-side DTO marshaling, no partial/patch format.

---

## 5. Hash determinism — what's real, what's a stub, what could diverge

No `Utc::now()`, `SystemTime::now()`, or any wall-clock call exists anywhere in `lib.rs` (grepped, zero hits) — **timestamps are never a divergence risk** for replay, because rs never stamps anything with "now" on its own. Every `Timestamp` field (`createdAt`/`updatedAt` on `Kit`/`Design`/`File`/etc.) is only ever set from JSON hydration (`installProjection`, lib.rs:12752-12756: `json.get("createdAt")` → `Timestamp(c.to_string())`) — deterministic as long as the JSON itself is deterministic across replicas.

**The actual divergence risk is entity ids, not timestamps.** `id::Id::new()` = `Uuid::now_v7()` (lib.rs:755-756) — time-based + random, **not deterministic**. Every `create*` mutation used to mint this server-side with no way to override it; **as of the very latest source read** (concurrent edit, see §0/§2.3/§3.2), `createTag`/`createConcept`/`createQuality`/`createType`/`createDesign`/`createFolder`/`addFixedPiece` all now take an optional `id: Option<Id>` and only fall back to `Id::new()` when omitted (e.g. lib.rs:17857-17869 for `addFixedPiece`). **`@semio/js` was missing this when first grepped, but is fixed as of the latest read** (same live-edit pattern as everywhere else in this cookbook — re-verify before trusting): `declare createTag: (name, description?, icon?, order?, id?: string | null) => ...` (index.ts:2332) and every sibling create-method now takes a trailing optional `id`, and — the part that actually matters — the `buildInner` roster entries **always** embed one into the GraphQL string, generating a fresh one when the caller didn't supply it: `createTag(id: ${gqlString(String(id ?? newUuidV7()))}, ...)` (index.ts:2481-2483), same pattern for `addFixedPiece` (index.ts:3474-3476: `addFixedPiece(id: ${gqlString(String(id ?? newUuidV7()))}, ...)`). So **as of right now, every create-shaped mutation document leaving `@semio/js` already carries an explicit, client-minted `uuid v7` id, never relying on rs's server-side fallback.** This is the one part of the whole determinism story that is fully closed, source-confirmed on both the rs (optional `id` param, §3.2) and JS (always-pass-an-id, here) sides — the only thing not yet re-verified live is whether it survives the currently-red build (§7.3) and whether hub replay actually reuses the *same* client-supplied variables (it must, since `HubReplica`/hub protocol broadcast `query`+`variables` verbatim, §4 — if `id` is inside `variables` rather than inlined in `query` text it will replay correctly either way, since the whole document is replayed as one unit).

**Hash tree reality (checked exhaustively for every root-ish entity's own `compute_hash`/`hash()` resolver):**

| Entity | Hash formula (as read) | Line | Content-aware? |
|---|---|---|---|
| `Kit` | *(new, unverified by build)* `canonical_json_hash(projection_value())` — full JSON | 6130-6132 | ✅ (if it compiles) |
| `Kit` | *(old, build-verified §7.2)* `h([id, name])` | — | ❌ |
| `Design` | `h([id, name])` | 4682-4685 | ❌ — **still true in latest read** |
| `Piece` | `h([id, name.unwrap_or("")])` | 3873-3876 | ❌ — position/blueprint/connection NOT hashed |
| `Typology` | `h([id, name, description, icon, folder_id])` | 1609-1625 | partial (own scalars only, not owned types/designs) |
| `Family` | `h([id, name, description, icon, folder_id])` | 1532-1544 | partial |
| `File` | `h([id, name, url, mime, size, hash(blob), description, icon, folder_id, created, updated])` | 2528-2546 | ✅ full scalar content (this one DOES include timestamps — the only entity where a timestamp participates in a hash) |
| `Graph` (wip/authoritative) | `h([id])` | 8191-8193/8199-8203 | ❌ — pure identity, changes on every new workspace only |
| `Session` | `h([id])` | 8262-8264 | ❌ |
| `StoreConnection` (`Query.store`) | `h(["store"])` — **literal constant string** | 16911 | ❌ — always identical regardless of anything |
| `IdResult` / `CommandResponse` | `h([...ids])` | 10794-10795/10863-10864 | n/a (response envelope, not domain data) |

Relay collections (`entity_relay!` macro, lib.rs:79-95) *do* correctly roll up children via `merkle_collection(sorted child.compute_hash())` — e.g. `hasDesigns.hash` is a real merkle fold — **but it folds in each child's own (often shallow) hash**, so the fold is only as good as its weakest child. `DesignConnection.hash` will not move when a piece inside one of its designs changes, because `Design::compute_hash()` doesn't look at pieces.

**On-disk projection fixtures corroborate this is a known, acknowledged stub, not an oversight**: every collection `hash` field in `semio/fixtures/stores/metabolism/wip/initialKit/kit.semio.json` (files, folders, families, tags, concepts, qualities, typologies, ports, connectors, representations, pieces, connections) is the **same literal constant**, `af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262` = `KIT_BUNDLE_HASH_STUB` (lib.rs:11132), documented in-source as *"Blake3 hex (empty-input digest) used on the wire until per-entity merkle is filled"* (lib.rs:11123). Same stub is reused for every `OperationStep.hash` in the bundle/backbone operation log (lib.rs:13069, 13083, 13365) — so a persisted operation-log hash is **not yet meaningful either**, if the hub planned to use per-operation hashes from that log for anything.

**Bottom line for the hub's "on hash mismatch, resync" design:** it works (today, pre-fix) only for whole-kit identity/name changes; it does not — and until the in-flight `Kit.hash` fix (§2.3 State B) lands and is verified, still mostly does not — detect the most likely real divergence (different piece/design/type ids per replica from independent `Id::new()` calls, or drifted piece positions). Verify §7.3 after the tree goes green again before relying on `Kit.hash` for anything.

---

## 6. Response envelope (all mutations)

```graphql
interface Response { id: ID! hash: String! owner: Entity owns: EntityConnection ok: Boolean! errors: SemioError result: Result }
interface Result   { id: ID! hash: String! owner: Entity owns: EntityConnection }
type IdResult implements Result { id: ID! hash: String! owner: Entity owns: EntityConnection value: ID! }
```
(`ResponseInterface::Command(CommandResponse)`, `ResultInterface::Id(IdResult)`, lib.rs:10782-10898.) `errors` is `SemioError { kind: String, message: String, requestId: Option<ID> }` (lib.rs:868-907, `kind` is a free-form string like `"Invalid"`/`"NotFound"`, mapped 1:1 to JS `SetErrorKind` — index.ts:593-607). `GQL_RESPONSE_SELECTION` (index.ts:39-40) is the canonical minimal selection: `"ok errors { kind message requestId } result { ... on IdResult { value } }"`.

---

## 7. Live verification transcripts

### 7.1 Build + boot (first, successful build — this is the state §5's "State A" numbers come from)

```
$ cargo build -p semio-store --bin semio-store        # ~3-4 min cold, 4 shared CPUs
   Compiling semio v0.1.0 (...)
   Compiling semio-store v0.1.0 (...)
    Finished `dev` profile
$ SEMIO_STORE_PORT=4010 ./target/debug/semio-store &
$ curl -sS http://127.0.0.1:4010/healthz
semio-store
$ curl -sS -X POST http://127.0.0.1:4010/graphql -d '{"query":"{ __typename }"}'
{"data":{"__typename":"Query"}}
```

### 7.2 Install `metabolism` fixture, add a piece, prove the (old) shallow-hash + id-mismatch behavior

```
$ curl -sS -X POST http://127.0.0.1:4010/install -d '{"create":{"dto": <kit.semio.json, 249604 bytes>}}'
ok   (HTTP 201)

# before any mutation:
query { session { stores { edges { node { wip { theKit { kit { id name hash hasDesigns { hash edges { node { id name hash } } } } } } } } } } }
-> kit.id = "f042c2a4-3ba5-44b0-b22c-0ae8f568aacc", kit.name = "Metabolism"
-> kit.hash        = "bb33d8896346a6f63671fc0b97d82df89dbbdfdbc0c30e0a1162840a0e1032af"
-> hasDesigns.hash = "336af5203b350f15df03bb146aaa8deb4f7925943ac44ebeb49a035dbba70831"
-> design "Capsule Dream" (id 37ba7ec4-9023-4be7-9ab6-e0ebc80007f8).hash = "7d1fed8d257727c5c933ac8875df80720206a078c8257ef1afd9edc277fbd825"

# startNewChange -> tx = "01a0d9bb-80e2-7360-8b76-f12a795acebf"
# unsavedChange(tx) { kit { design(id: "37ba7ec4-...") { addFixedPiece(blueprintId: "71749140-9db9-43f6-bd81-d89011667b80" /* type "Capsule" */, position: {...}, name: "replay-test-piece") { ok result { ...on IdResult { value } } } } } }
-> ok: true, result.value = "01a0d9bb-9b0b-70c3-8070-3bd5e11cd343"     # <-- this is NOT the piece's id (see below)
# theKit { save { ... } }  -> ok: true

# after save, re-query:
-> the actually-created piece's real id = "01a0d9bb-9b0b-70c3-8070-3be4941765c7"   # <-- DIFFERENT from result.value above
-> design "Capsule Dream".hash            = "7d1fed8d257727c5c933ac8875df80720206a078c8257ef1afd9edc277fbd825"   # UNCHANGED despite gaining a piece
-> kit.hasDesigns.hash                    = "336af5203b350f15df03bb146aaa8deb4f7925943ac44ebeb49a035dbba70831"   # UNCHANGED
-> kit.hash                               = "b27a0f5d80b81106716a27e73ec54bae83ada423a60ca2ae48b8b3b75cc349d0"   # CHANGED (bb33d8...032af -> b27a0f...49d0) — see note
```
Note on the last line: at read-time the *source* already had the `Kit.hash` upgrade (canonical_json_hash) landed, but the *running binary* was built from the version compiled a few minutes earlier — i.e. this specific `kit.hash` delta was very likely produced by the *new* (content-aware) formula already, while `Design.hash` (unchanged) confirms that fix did not touch the per-design hash. This is exactly the split described in §2.3/§5: root fixed, children not. Treat this row as corroborating, not as the sole proof — the static source reads in §2.3/§5 are the authoritative claim.

This transcript alone proves, independent of any static reading: (1) a real piece was created with a real, distinct, server-picked id; (2) the mutation's own `result.value` is a different id than the entity that was actually created (the `request_id`-not-`entity_id` bug, §3.3); (3) `Design.hash` and `hasDesigns.hash` are insensitive to piece creation.

### 7.3 Second rebuild attempt — tree currently RED (informational, not a claim about my own work)

```
$ cargo build -p semio-store --bin semio-store
error[E0063]: missing field `connections` in initializer of `DesignDiff`  (lib.rs:9721, 9763, 9802)
error[E0308]: mismatched types — Piece::new_fixed_with_external_id expects PositionInput, got Option<PositionInput> (lib.rs:5874)
error[E0308]: mismatched types — position_input_to_json expects &PositionInput, got &Option<PositionInput> (lib.rs:11372)
error: could not compile `semio` (lib)
```
This is unrelated, in-flight work by another agent (adding a `connections` field to `DesignDiff` — i.e. someone is actively building the Connection-mutation support this cookbook's §3.2 flags as missing). Re-run `cargo build -p semio-store --bin semio-store` after pulling the latest tree to get fresh numbers for §2.3/§5 before depending on them; do not assume State B (§2.3) is load-bearing until that build is green.

---

## Notes for implementers (F2 / H) — most of this list turned out to already be in flight, see inline status

1. ~~Hook the choke point~~ **Already done.** `Session.executeOperation` (index.ts:1949, region `🎛️KitOperations`) is the designed-for-this choke point (middleware chain + listener fan-out), and `HubReplica` (index.ts:5833+) already uses `useOperationMiddleware` to forward every local op to the hub and replay remote ones. Read `HubReplica` directly for the exact wire envelope instead of re-deriving it.
2. ~~Client-generated ids~~ **Done on both sides, source-confirmed.** rs: `createTag`/`createConcept`/`createQuality`/`createType`/`createDesign`/`createFolder`/`addFixedPiece`/`addChildPieceWithParentConnection`/`connectPieces` all accept an optional `id`, falling back to server `Id::new()` only when omitted. JS: every corresponding `@semio/js` builder now inlines `id: ${gqlString(String(id ?? newUuidV7()))}` into the mutation string (index.ts:2481-2483, 3474-3476 — i.e. it *always* sends an id, generating one client-side when the caller didn't). Not yet build-verified (§7.3). Regardless of this fix, never trust the mutation response to recover a server-picked id on an *omitted* one — it returns an unrelated `request_id`, not the entity id (confirmed live, §7.2; `dispatch_wip_wait`/`CommandResponse::ok_request` unchanged at last read) — moot in practice now since JS never omits it, but rs-native callers (hub, MCP) still could and should know this.
3. Do not rely on any nested `.hash` field (`design.hash`, `hasDesigns.hash`, `type.hash`, …) as a divergence signal — only the root `Kit.hash` is (in-flight, unverified by a green build) content-aware; everything else is `id`(+name) only, confirmed by both source and live query. `Session.localHash()`/`HubReplica` already read exactly this field (`store.readKitInner("hash")`) — good, that's the only field worth reading — just know its blast radius is currently "whole kit" not "this specific entity."
4. ~~No connection mutations~~ **Also already implemented in source** (`connectPieces`, `deleteConnections`, `deletePiece(s)`, `deletePiecesAndConnections`, `addChildPieceWithParentConnection` — lib.rs:18400-18480, see §3.2 stop-press). Not yet build-verified (§7.3) — re-run the build before wiring MCP tools to it.
5. ~~No export query~~ **Also already added**: `Kit.projection: String!` (lib.rs:6777-6780, §2.2). Not yet build-verified (§7.3).
6. **Narrower than it looked:** `PieceOperationInput.rename`/`changeDescription`/`move` are now also implemented (via a shared `dispatch_piece_patch`, lib.rs:18538-18570 — `move` is an absolute reposition, doc'd as "a linked piece becomes fixed"). Only `fix` and `changeBlueprint` remain `not_implemented` (lib.rs:18571-18578). So `update_piece` (MCP) can cover rename/description/position; blueprint-swap and "unfix" cannot yet.
7. Given how fast this file is moving: whoever reads this next should treat every claim above as "true as of a specific git-dirty snapshot, re-verify with a fresh `grep`/build before depending on it in production code," not as a frozen spec.
