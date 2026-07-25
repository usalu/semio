---
name: kit-name-rename-flow
overview: "Implement the first canonical example of the new state-management architecture: an end-to-end, non-blocking kit-name rename flow where sketchpad's details panel input drives `useKitName()` -> `KitStore.rename()` -> GraphQL `renameKit` mutation -> rs validation/apply -> `kitRenamed` / `operationFailed` subscription events -> status fan-in to the input (idle / pending / success / error)."
todos:
 - id: rs_validate_emit
   content: Add 256-char name validation, emit Event::RenamedKit with requestId, and move CommandSucceeded after apply in compose/rs/lib.rs
   status: completed
 - id: rs_subscription
   content: Add Subscription.kitRenamed in compose/rs/lib.rs and ensure RenamedKit + ComposeError carry requestId
   status: completed
 - id: graphql_sdl
   content: Regenerate compose/graphql/schema.graphql from gql::sdl()
   status: completed
 - id: js_kitstore_rename
   content: Add KitStore.rename(name) + internal RxJS BehaviorSubjects for kitName / renameStatus + subscribe/get snapshot APIs in compose/js/index.ts
   status: completed
 - id: js_subscription_wire
   content: Wire startSubscriptionLoop to dispatch kitRenamed/operationFailed into the new subjects, with requestId correlation
   status: completed
 - id: react_use_kit_name
   content: Rewrite useKitName() in compose/react/index.tsx to use useSyncExternalStore against the KitStore subjects (return [name, rename, status])
   status: completed
 - id: sketchpad_input
   content: Update KitSectionForm in compose/sketchpad/index.tsx to consume the triad and show inline Spinner (pending) + error text (error)
   status: completed
 - id: verify_runtime
   content: Run sketchpad, edit kit name in details panel, verify success path (live name updates) and error path (>256 chars shows error inline, never blocks)
   status: completed
isProject: false
---

## Goal

Wire a single, clean `kit name` rename slice end-to-end through the new layered stack:

`Input` (sketchpad) -> `useKitName()` (`compose/react`) -> `KitStore.rename()` (`compose/js`, internal RxJS) -> `renameKit` mutation + `kitRenamed`/`operationFailed` subscription (`compose/graphql`) -> validation + emission (`compose/rs`).

The hook returns `[kitName, renameKit, status]` where `kitName` is live (auto-updated via `useSyncExternalStore`), `renameKit(name)` returns a `Promise<{ ok, requestId }>`, and `status` is the lifecycle of the latest request (`idle | pending | success | error`). `compose/rs` validates `name.len() <= 256` to demonstrate failure path.

## Sequence

```mermaid
sequenceDiagram
  participant UI as Sketchpad Input
  participant Hook as useKitName
  participant Store as KitStore.rename
  participant GQL as GraphQL transport
  participant RS as compose/rs worker
  participant Bus as EventBus

  UI->>Hook: renameKit("New Name")
  Hook->>Store: rename("New Name")
  Store->>Store: status$.next(pending, requestId=r)
  Store->>GQL: mutation renameKit(... name)
  GQL->>RS: dispatch_wip(Command::RenameKit{r,...})
  RS-->>Store: requestId r
  alt name length > 256
    RS->>Bus: Event::OperationFailed{request_id:r, kind:"invalid"}
    Bus-->>Store: operationFailed
    Store->>Store: status$.next(error, msg)
  else valid
    RS->>RS: apply (write the_kit.name)
    RS->>Bus: Event::RenamedKit{request_id:r, name}
    Bus-->>Store: kitRenamed
    Store->>Store: name$.next(name); status$.next(success)
  end
  Hook-->>UI: live name + status
```

## Files

- [compose/rs/lib.rs](compose/rs/lib.rs) - validation, `Event::RenamedKit` emission, `Subscription.kitRenamed` field, fix `commandSucceeded` ordering (currently emitted before `apply`).
- [compose/graphql/schema.graphql](compose/graphql/schema.graphql) - regenerated SDL (target only; produced by `gql::sdl()`).
- [compose/js/index.ts](compose/js/index.ts) - new `KitStore.rename(name)` and internal RxJS `BehaviorSubject`s for live kit name + rename-request status; subscription loop forwards `kitRenamed` / `operationFailed`.
- [compose/react/index.tsx](compose/react/index.tsx) - rewrite `useKitName()` body to subscribe via `useSyncExternalStore` directly to `KitStore` (skip `useSchemaFieldState` for this slice).
- [compose/sketchpad/index.tsx](compose/sketchpad/index.tsx) - in `KitSectionForm`, consume the triad and render inline `Spinner` + error text alongside the name `<Input>`. Non-blocking lazy commit stays.

## Concrete changes

### 1. `compose/rs/lib.rs`

In `gql` Mutation `renameKit` (around line 6276) keep signature `renameKit(draftId, transactionId, name): ID!`. JS still passes ephemeral ids (existing wip write anchors), so no rs API churn.

In `ChildRuntime::run` (around line 5997) move the `CommandSucceeded` emission to AFTER `apply` returns Ok (currently it fires before `apply`, defeating the contract):

```rust
if let Err(e) = self.apply(cmd).await {
    self.bus.emit_event(Event::OperationFailed(e.with_request(request_id))).await;
} else {
    self.bus.emit_event(Event::CommandSucceeded(CommandReceipt { request_id, kind: kind.to_string() })).await;
}
```

In `apply` for `Command::RenameKit { request_id, name, .. }` (around line 6057) add validation and emit `Event::RenamedKit`:

```rust
Command::RenameKit { request_id, name, .. } => {
    if name.chars().count() > 256 {
        return Err(ComposeError::invalid(format!("Kit name too long: {} > 256", name.chars().count())));
    }
    *self.graph.the_kit.name.write().await = name.clone();
    self.graph.the_kit.bump_touch_epoch().await;
    let operation = operation::RenamedKit::new(/* input, kit, diff */ ...).await;
    self.bus.emit_event(Event::RenamedKit(Arc::new(operation))).await;
    Ok(())
}
```

Ensure `operation::RenamedKit` (already defined ~line 5019) and its GraphQL `SimpleObject` expose a `requestId` field correlated to the originating command (thread `request_id` into the constructor / payload).

In `Subscription` (around line 6512) add a dedicated stream:

```rust
#[graphql(name = "kitRenamed")]
pub async fn kit_renamed(&self, ctx: &Context<'_>) -> async_graphql::Result<KitRenamedStream> {
    // filter Event::RenamedKit -> RenamedKit payload
}
```

Also keep `operationFailed` as the failure channel; ensure `ComposeError` payload includes `request_id` so JS can correlate.

### 2. `compose/graphql/schema.graphql`

Regenerate via `COMPOSE_GRAPHQL_SCHEMA_OUT=... cargo test` (the rs export path) so the SDL exposes `kitRenamed` and the `RenamedKit { requestId, name, ... }` shape. Do not hand-edit drift.

### 3. `compose/js/index.ts`

In the `KitStore` class:

- Add internal RxJS subjects (RxJS is already imported):
  - `private readonly kitName$ = new BehaviorSubject<string>("")` -- seeded from initial `KitFullDto` in `open()`
  - `private readonly renameStatus$ = new BehaviorSubject<RenameStatus>({ kind: "idle" })` where `RenameStatus = { kind: "idle" } | { kind: "pending"; requestId: string } | { kind: "success"; requestId: string; name: string } | { kind: "error"; requestId: string; message: string }`
- Add public reactive surface:
  - `subscribeKitName(h: (n: string) => void): Unsubscribe`
  - `getKitNameSnapshot(): string`
  - `subscribeRenameStatus(h: (s: RenameStatus) => void): Unsubscribe`
  - `getRenameStatusSnapshot(): RenameStatus`
- Add public method:

```ts
async rename(name: string): Promise<{ ok: boolean; requestId: string; error?: SetError }> {
  const draftId = this.kitWriteDraftId ?? (await this.ensureWriteScope()).draftId;
  const transactionId = ...;
  const body = JSON.stringify({ query: "mutation($d:ID!,$t:ID!,$n:String!){ renameKit(draftId:$d,transactionId:$t,name:$n) }", variables: { d: draftId, t: transactionId, n: name } });
  const requestId = (await kitGraphqlRunTyped<{ renameKit: string }>(this.transport, body)).renameKit;
  this.renameStatus$.next({ kind: "pending", requestId });
  return await this.awaitRenameOutcome(requestId);
}
```

- Extend `startSubscriptionLoop` so that when an event arrives on `KIT_EVENT_STREAM_SUBSCRIPTION` (or a new dedicated `kitRenamed` subscription) we:
  - On `RenamedKit { requestId, name }`: `kitName$.next(name); renameStatus$.next({ kind: "success", requestId, name });`
  - On `OperationFailed { requestId, message }`: if it matches the in-flight rename's requestId -> `renameStatus$.next({ kind: "error", requestId, message })`
- `awaitRenameOutcome(requestId)` resolves on the first matching success/error.

Region-place all of this under a new `//#region 🪪Rename` block inside the `KitStore` class.

### 4. `compose/react/index.tsx`

Replace `useKitName` body (around line 8763). New implementation pulls the active `KitStore` directly (via the existing runtime / scope context that already surfaces `KitStoreClient`; expose the underlying `KitStore` if not already accessible) and uses `React.useSyncExternalStore` twice:

```tsx
export function useKitName(): readonly [string, (n: string) => Promise<SetResult>, WriteStatus] {
 const store = useKitStore(); // existing helper that returns the active KitStore
 const name = React.useSyncExternalStore(store.subscribeKitName, store.getKitNameSnapshot, store.getKitNameSnapshot);
 const rename = React.useSyncExternalStore(store.subscribeRenameStatus, store.getRenameStatusSnapshot, store.getRenameStatusSnapshot);
 const setter = React.useCallback(
  async (next: string) => {
   const r = await store.rename(next);
   return r.ok ? { ok: true } : { ok: false, error: r.error! };
  },
  [store],
 );
 const status: WriteStatus = rename.kind === "pending" ? { kind: "pending", pending: 1 } : rename.kind === "error" ? { kind: "error", pending: 0, lastError: { kind: "Invalid", message: rename.message } } : { kind: "idle", pending: 0 };
 return [name, setter, status] as const;
}
```

(Drop the `useSchemaFieldState("Kit", "name", ...)` indirection for this slice; it stays for other fields untouched.)

### 5. `compose/sketchpad/index.tsx` - `KitSectionForm` (around line 15639)

Replace:

```tsx
const [, setName] = useKitName();
...
<Input lazy ... value={kit.name} onLazyChange={(value) => void setName(value)} showLabel />
```

with status-aware composition:

```tsx
const [kitName, renameKit, status] = useKitName();
...
<TreeRow>
  <div className="flex items-center gap-single w-full">
    <Input lazy id="..." value={kitName} onLazyChange={(v) => void renameKit(v)} showLabel />
    {status.kind === "pending" && <Spinner size="small" className="text-muted-foreground" />}
  </div>
  {status.kind === "error" && (
    <div className="text-destructive text-xs pl-tiny">{status.lastError?.message}</div>
  )}
</TreeRow>
```

`Spinner` is already imported from `@compose/ui`. No changes needed in `elements/ui` -- composition is local and non-invasive.

## Out of scope (intentionally)

- Other entity name fields (Design/Type/Family) keep their current path; this ticket is the canonical example only.
- The legacy `useSchemaFieldState` machinery stays intact for other fields.
- Backbone / VCS draft semantics for rename are unchanged (still anchored to the wip kit auto-write scope).
