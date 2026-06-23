---
name: refactor kit name reads
overview: "Introduce a clean, extendable kit-store mechanism in `compose/js` (`StoreField<T>`, `StoreCommand<TArgs>`, `RequestCorrelator`, `OperationRouter`, `mutateWithRequestId`) so any kit-store-backed value/command is a one-block declaration. Apply it first to the kit name: split the React surface into `useKitName(): string` + `useRenameKit(): [(name) => Promise<SetResult>, WriteStatus]` (both reduce to one-liners over generic `useStoreField` / `useStoreCommand` hooks), remove the kit-name triad mechanism entirely from sketchpad, keep Promise-based reads cacheless via GraphQL into rs, and feed sync mirrors only from the rs output event stream."
todos:
  - id: ticket_open
    content: Open MCP ticket 'Refactor Kit Name Reads And Split Rename Hook' under appropriate goal.
    status: cancelled
  - id: move_writestatus
    content: Move WriteStatus + SetError + SetResult + helpers (SCHEMA_HOOK_IDLE_STATUS, SCHEMA_HOOK_READONLY_STATUS, USE_KIT_NAME_PENDING_STATUS, writeStatusEquivalent) from compose/react/index.tsx into compose/js/index.ts and re-export.
    status: pending
  - id: generic_primitives
    content: In compose/js/index.ts add StoreField<T>, StoreCommand<TArgs>, RequestCorrelator, OperationRouter, mutateWithRequestId helper, and a single startEventStreamLoop pumping operationSucceeded + operationFailed into router/correlator.
    status: pending
  - id: kitstore_refactor
    content: In compose/js/index.ts drop renameStatus$/KitRenameStatus/KitRenameResult/renameResolvers/renamePendingEvents/dispatchKitRenameSubscription/startRenameSubscriptionLoop/seedLiveKitNameFromDto; declare kitName=StoreField<string>, renameKit=StoreCommand<string>; wireKitName() via OperationRouter; add cacheless readKitName(); seedFieldsFromDto() at open(); dispose disposes fields/commands/correlator.
    status: completed
  - id: split_hooks
    content: In compose/react/index.tsx add generic useStoreField/useStoreCommand and useKitStore() helper; export useKitName=useStoreField(ks.kitName) and useRenameKit=useStoreCommand(ks.renameKit); drop runtime.store / schemaTriad fallbacks and switch to throwing useKitRuntime().
    status: pending
  - id: test_mocks
    content: Update createTestKitClient stubs (lines ~17285 and ~17707) and rewrite the two affected tests against the split API.
    status: completed
  - id: sketchpad_kitform
    content: In compose/sketchpad/index.tsx KitSectionForm import useRenameKit, drop the SketchpadTriadInputRow usage for the kit name, and inline the input row consuming kitName + renameKit + renameKitStatus directly (no triad).
    status: pending
  - id: validate
    content: Run depcruise:layers, type-check, compose/react vitest, sketchpad rename smoke (spinner, error, success, long-name, timeout).
    status: completed
  - id: ticket_close
    content: Close the MCP ticket with summary of files touched.
    status: completed
isProject: false
---

## Goals

- Split the React surface in two: `useKitName(): string` (read hook) and `useRenameKit(): readonly [(name: string) => Promise<SetResult>, WriteStatus]` (write hook). No triads.
- All Promise-based reads of the name go through GraphQL into rs each time, with internal request-id tracking only used to correlate responses on the rs output event stream — no Promise read ever consults a local store.
- `compose/js` `KitStore` exposes two independent `useSyncExternalStore`-compatible pairs — one for the kit name, one for the rename status — both backed by rxjs `BehaviorSubject`s internally and fed exclusively by the rs `kitRenamed` / `operationFailed` event stream and the rename setter.
- Each hook MUST use exactly one `useSyncExternalStore` call and contain zero mechanism.
- All current functionality (rename request, timeouts, validation errors, UI spinner / error display, sketchpad `KitSectionForm`) stays intact.

## Non-goals

- No changes to `compose/rs` (per the user's "extend_existing_query" answer).
- No changes to the GraphQL schema.
- Other entity stores (`DesignStore`, etc.) and other `useKit*` hooks are untouched.

## Affected files

- [compose/js/index.ts](compose/js/index.ts) — `KitStore` rename / kit-name surface (~lines 1547–1916), plus moving `WriteStatus` helpers in from react.
- [compose/react/index.tsx](compose/react/index.tsx) — split `useKitName` (~lines 8938–9032) into `useKitName` + new `useRenameKit`; update test mocks at ~17285–17289 and ~17707–17710; rewrite the two affected tests.
- [compose/sketchpad/index.tsx](compose/sketchpad/index.tsx) — `KitSectionForm` (~line 15775) imports both `useKitName` + `useRenameKit` (drop the `useKitName` triad usage), inlines the kit name input row directly without `SketchpadTriadInputRow`; add `useRenameKit` + `useWriteIndicator` (already imported) to the import block.

## Design

## Smell-free, extendable mechanism (kit name is just one example)

The previous draft per-fielded everything (`kitName$`, `subscribeKitName`, `getKitNameSnapshot`, `renameKitStatus$`, `subscribeRenameKitStatus`, `getRenameKitStatusSnapshot`, `renameKit`, `dispatchRenameMutation`, `publishRenameKitError`, `lastRenameKitError`, `renameResolvers`, `renamePendingEvents`, `dispatchKitRenameSubscription`, `startRenameSubscriptionLoop`, ...). Adding a 2nd field (description, icon, image, homepage, license, release, ...) would duplicate every one of those.

Replace with three small generic primitives in `compose/js/index.ts`. Each new field/command then needs ≤ 10 lines.

### Generic primitives

```typescript
//#region 🧱 generic store primitives

/** @emoji 📥 Sync mirror of one rs-owned value, fed by the rs event stream. */
export class StoreField<T> {
  private readonly value$: BehaviorSubject<T>;
  constructor(initial: T) { this.value$ = new BehaviorSubject(initial); }
  subscribe = (h: () => void): Unsubscribe => {
    const s = this.value$.subscribe({ next: () => h() });
    return () => s.unsubscribe();
  };
  getSnapshot = (): T => this.value$.getValue();
  /** @internal Update from event-stream listeners. */
  set(next: T): void { this.value$.next(next); }
  dispose(): void { try { this.value$.complete(); } catch { /* ignore */ } }
}

/** @emoji 📝 Async kit-store command with bound runner + render-ready {@link WriteStatus}. */
export class StoreCommand<TArgs> {
  readonly status: StoreField<WriteStatus> = new StoreField<WriteStatus>(SCHEMA_HOOK_IDLE_STATUS);
  private lastError: SetError | null = null;
  constructor(private readonly exec: (args: TArgs) => Promise<SetResult>) {}
  readonly run = async (args: TArgs): Promise<SetResult> => {
    this.status.set(USE_KIT_NAME_PENDING_STATUS);
    const r = await this.exec(args);
    if (r.ok) {
      this.lastError = null;
      this.status.set(SCHEMA_HOOK_IDLE_STATUS);
    } else {
      const cached = this.lastError;
      const lastError = cached && cached.kind === r.error.kind && cached.message === r.error.message ? cached : r.error;
      this.lastError = lastError;
      const cur = this.status.getSnapshot();
      const next: WriteStatus = { kind: "error", pending: 0, lastError };
      if (!writeStatusEquivalent(cur, next)) this.status.set(next);
    }
    return r;
  };
  dispose(): void { this.status.dispose(); }
}

/** @emoji 🚦 Generic request-id ↔ Promise-resolver correlator (no per-command duplication). */
export class RequestCorrelator {
  private readonly resolvers = new Map<string, (r: SetResult) => void>();
  private readonly pending = new Map<string, SetResult>();
  constructor(private readonly timeoutMs: number) {}
  await(requestId: string): Promise<SetResult> {
    const buffered = this.pending.get(requestId);
    if (buffered) { this.pending.delete(requestId); return Promise.resolve(buffered); }
    return new Promise<SetResult>((resolve) => {
      const t = setTimeout(() => {
        if (!this.resolvers.has(requestId)) return;
        this.resolvers.delete(requestId);
        resolve({ ok: false, error: { kind: "Timeout", message: `request ${requestId}: timed out` } });
      }, this.timeoutMs);
      this.resolvers.set(requestId, (r) => { clearTimeout(t); resolve(r); });
    });
  }
  resolve(requestId: string, r: SetResult): void {
    const fn = this.resolvers.get(requestId);
    if (fn) { fn(r); this.resolvers.delete(requestId); return; }
    this.pending.set(requestId, r);
    setTimeout(() => { if (this.pending.get(requestId) === r) this.pending.delete(requestId); }, 10_000);
  }
  disposeAll(reason = "KitStore disposed"): void {
    for (const [rid, fn] of this.resolvers) fn({ ok: false, error: { kind: "Disposed", message: `${reason} (${rid})` } });
    this.resolvers.clear();
    this.pending.clear();
  }
}

/** @emoji 🚌 Routes typed rs operation events to listeners; one demux for the whole store. */
export interface OperationEvent<P = JsonObject> { readonly kind: string; readonly requestId: string | null; readonly payload: P }
type OperationListener<P> = (ev: OperationEvent<P>) => void;
export class OperationRouter {
  private readonly listeners = new Map<string, Set<OperationListener<JsonObject>>>();
  on<P extends JsonObject>(kind: string, l: OperationListener<P>): Unsubscribe {
    let set = this.listeners.get(kind);
    if (!set) { set = new Set(); this.listeners.set(kind, set); }
    set.add(l as OperationListener<JsonObject>);
    return () => { set!.delete(l as OperationListener<JsonObject>); };
  }
  emit<P extends JsonObject>(ev: OperationEvent<P>): void {
    const set = this.listeners.get(ev.kind);
    if (!set) return;
    for (const l of set) l(ev as unknown as OperationEvent<JsonObject>);
  }
}

//#endregion
```

### `WriteStatus` types (moved from `compose/react`)

`SetError`, `SetResult`, `WriteStatus`, `SCHEMA_HOOK_IDLE_STATUS`, `SCHEMA_HOOK_READONLY_STATUS`, `USE_KIT_NAME_PENDING_STATUS`, `writeStatusEquivalent` — defined exactly once in `compose/js/index.ts`, re-exported from `@semio-tech/compose-js`, consumed from `@semio-tech/compose-react`.

### `KitStore` glue (one-time)

```typescript
export class KitStore {
  // ... existing fields ...
  private readonly correlator = new RequestCorrelator(this.timeoutMs);
  private readonly router = new OperationRouter();

  /** @emoji 🛠️ One-line helper for any kit-store mutation that returns a `requestId` and resolves on `operationSucceeded` / `operationFailed`. */
  private async mutateWithRequestId(
    inTx: boolean,
    query: string,
    variablesFor: (tx: { draftId: string; transactionId: string }) => GraphQlVariables,
    extractRequestId: (data: JsonObject) => string,
  ): Promise<SetResult> {
    let tx: { draftId: string; transactionId: string };
    try { tx = await this.ensureOpenKitWriteTransaction(); }
    catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; }
    let requestId: string;
    try {
      const data = kitGraphqlData(await this.gqlRun({ query, variables: variablesFor(tx) })) as JsonObject;
      requestId = String(extractRequestId(data) ?? "");
      if (requestId === "") throw new Error("mutation: empty requestId");
    } catch (e) { return { ok: false, error: { kind: "Internal", message: String(e) } }; }
    const r = await this.correlator.await(requestId);
    if (inTx) {
      if (r.ok) await this.finalizeKitWriteTransaction().catch(() => undefined);
      else      await this.abortKitWriteTransaction().catch(() => undefined);
    }
    return r;
  }

  /** @emoji 🚌 Single rs subscription loop pumping every operation event into {@link router} + {@link correlator}. */
  private startEventStreamLoop(): void { /* one subscription on operationSucceeded + one on operationFailed; routes by event kind, resolves correlator by requestId */ }

  private mapOperationFailedToSetError(kind: string, message: string): SetError {
    const k = kind.trim();
    if (k === "Invalid")  return { kind: "InvalidValue", message };
    if (k === "NotFound") return { kind: "NotFound",     message };
    return { kind: "Internal", message };
  }
}
```

### Per-field declarations (kit name is just an example)

Every kit-store-backed field/command is now a one-block declaration. No new mechanism per field.

```typescript
// Kit name
readonly kitName = new StoreField<string>("");
readonly renameKit = new StoreCommand<string>(async (name) =>
  this.mutateWithRequestId(
    /* inTx */ true,
    `mutation RenameKit($draftId: Id!, $transactionId: Id!, $name: String!) {
       renameKit(draftId: $draftId, transactionId: $transactionId, name: $name)
     }`,
    (tx) => ({ ...tx, name }),
    (data) => String((data as { renameKit?: string }).renameKit ?? ""),
  ),
);
private wireKitName(): void {
  this.router.on<{ kit?: { name?: string } }>("kitRenamed", (ev) =>
    this.kitName.set(String(ev.payload.kit?.name ?? "")),
  );
}
async readKitName(): Promise<string> {
  const data = kitGraphqlData(await this.gqlRun({ query: "query { wip { theKit { name } } }" })) as { wip?: { theKit?: { name?: string } } };
  return String(data.wip?.theKit?.name ?? "");
}
```

Adding e.g. **kit description** is then:

```typescript
readonly kitDescription = new StoreField<string>("");
readonly changeKitDescription = new StoreCommand<string>(async (description) =>
  this.mutateWithRequestId(
    true,
    `mutation ChangeDescription($draftId: Id!, $transactionId: Id!, $entityId: Id!, $description: String!) {
       changeDescription(draftId: $draftId, transactionId: $transactionId, entityId: $entityId, description: $description)
     }`,
    (tx) => ({ ...tx, entityId: this.kitId, description }),
    (data) => String((data as { changeDescription?: string }).changeDescription ?? ""),
  ),
);
private wireKitDescription(): void {
  this.router.on<{ description?: string }>("changedDescription", (ev) =>
    this.kitDescription.set(String(ev.payload.description ?? "")),
  );
}
async readKitDescription(): Promise<string> { /* analogous one-line GraphQL read */ }
```

No status BehaviorSubject, no resolver Map, no per-field subscription loop, no `lastError` ref. Everything is in `StoreField` / `StoreCommand` / `RequestCorrelator` / `OperationRouter`.

### Public surface (per field)

For any `StoreField` / `StoreCommand` declared on `KitStore`:

- Read mirror: `ks.kitName.subscribe(cb)` + `ks.kitName.getSnapshot()`.
- Promise read (cacheless): `ks.readKitName()`.
- Write command: `ks.renameKit.run(name)` + `ks.renameKit.status.subscribe(cb)` + `ks.renameKit.status.getSnapshot()`.

### React hooks (also generic)

In `compose/react/index.tsx`:

```typescript
export function useStoreField<T>(field: StoreField<T>): T {
  return React.useSyncExternalStore(field.subscribe, field.getSnapshot);
}
export function useStoreCommand<TArgs>(cmd: StoreCommand<TArgs>): readonly [(args: TArgs) => Promise<SetResult>, WriteStatus] {
  const status = useStoreField(cmd.status);
  return [cmd.run, status] as const;
}

// Per-field hooks become one-liners:
export const useKitName = (): string => useStoreField(useKitStore().kitName);
export const useRenameKit = () => useStoreCommand(useKitStore().renameKit);

// Future fields are just as cheap:
export const useKitDescription = (): string => useStoreField(useKitStore().kitDescription);
export const useChangeKitDescription = () => useStoreCommand(useKitStore().changeKitDescription);
```

`useKitStore()` is a tiny adapter on top of the existing `useKitRuntime()` + `kitStoreFromKitStoreClient` so callers don't repeat that boilerplate.

### Removed members on `KitStore` and its types

- `renameStatus$ : BehaviorSubject<KitRenameStatus>`, `subscribeRenameStatus`, `getRenameStatusSnapshot`.
- `KitRenameStatus`, `KIT_RENAME_STATUS_IDLE`, `KitRenameResult`.
- The old `rename(name): Promise<KitRenameResult>` (replaced by `renameKit.run(name): Promise<SetResult>`).
- `seedLiveKitNameFromDto` private method (replaced by a generic `seedFieldsFromDto(dto)` that pumps initial values into all `StoreField`s registered with a `seedFromDto` extractor — kit name being one of them).
- `dispatchKitRenameSubscription`, `startRenameSubscriptionLoop` (replaced by the single `startEventStreamLoop` + `OperationRouter`).
- `renameResolvers`, `renamePendingEvents` (now inside `RequestCorrelator`).

### `KitStore` wiring + dispose

```typescript
static async open(initialKit: KitFullDto, opts?: KitStoreOpenOptions): Promise<KitStore> {
  // ... transport / handle wiring unchanged ...
  ks.wireFields();              // calls wireKitName(), wireKitDescription(), ...
  ks.seedFieldsFromDto(dto);    // sets kitName / kitDescription / ... initial values from the opening DTO
  await withTimeout(ks.warmGraphqlRead(), timeoutMs, "graphql");
  void ks.startSubscriptionLoop();   // existing fanout loop
  void ks.startEventStreamLoop();    // new: feeds OperationRouter + RequestCorrelator
  return ks;
}

async dispose(): Promise<void> {
  if (this.disposed) return;
  this.disposed = true;
  this.correlator.disposeAll();
  this.fanout.complete();
  this.kitName.dispose();
  this.renameKit.dispose();
  // ...future fields dispose themselves the same way...
  this.transport.dispose();
}
```

### `KitStore` (compose/js) — narrative summary

The full design and target code are in the **"Smell-free, extendable mechanism"** section above. In short:

- All per-field bespoke wiring is gone. Each kit-store-backed value is a `StoreField<T>`; each kit-store-backed mutation is a `StoreCommand<TArgs>`. Both implement the rxjs-backed subscribe / getSnapshot contract.
- A single `RequestCorrelator` owns request-id ↔ Promise tracking for every command on the store. A single `OperationRouter` demuxes the rs operation event stream into typed listeners.
- A single `mutateWithRequestId` helper does the open-tx → mutation → correlator-await → finalize/abort dance for every command on the store.
- A single `startEventStreamLoop` subscribes once to `operationSucceeded` + `operationFailed` and pumps every event into the router + correlator.
- All `WriteStatus` types move from `compose/react` into `compose/js` so `StoreCommand.status` returns a render-ready value.
- The kit name surface is just a one-block declaration: `kitName: StoreField<string>`, `renameKit: StoreCommand<string>`, `wireKitName()`, and a tiny `readKitName()` for cacheless Promise reads. Adding kit description, icon, image, homepage, license, release, etc. follows the exact same shape — no new mechanism per field.

### `useKitName` + `useRenameKit` (compose/react) — both extremely lean, both one-liners

```typescript
export function useStoreField<T>(field: StoreField<T>): T {
  return React.useSyncExternalStore(field.subscribe, field.getSnapshot);
}
export function useStoreCommand<TArgs>(cmd: StoreCommand<TArgs>): readonly [(args: TArgs) => Promise<SetResult>, WriteStatus] {
  const status = useStoreField(cmd.status);
  return [cmd.run, status] as const;
}

function useKitStore(): KitStore {
  const { kitClient } = useKitRuntime();
  return React.useMemo(() => kitStoreFromKitStoreClient(kitClient), [kitClient]);
}

export const useKitName = (): string => useStoreField(useKitStore().kitName);
export const useRenameKit = () => useStoreCommand(useKitStore().renameKit);
```

- One `useSyncExternalStore` call per `useStoreField`/`useStoreCommand`. Zero mechanism in any per-field hook.
- `useKitRuntime()` (the throwing variant) replaces `useKitRuntimeSafe()`. Outside `KitScope`, calling either hook is an error.
- `useKitName` returns the bare string. `useRenameKit` returns `[run, status]`.
- `idValue` parameter is dropped on both (KitStore is already kit-scoped).
- Adding hooks for new fields (description, icon, ...) is one line each: `export const useKitDescription = () => useStoreField(useKitStore().kitDescription);`.

### `KitSectionForm` (compose/sketchpad) — no triad anywhere on the kit-name path

The kit name field MUST NOT go through `SketchpadTriadInputRow`. The triad mechanism is removed entirely from the kit-name code path. Other Kit fields (release, description, icon, image, homepage, license) keep using `SketchpadTriadInputRow` because their hooks still return triads — only the kit name is split.

Render the kit name input inline in `KitSectionForm`, consuming the two split hooks directly:

```typescript
const kitName = useKitName();
const [renameKit, renameKitStatus] = useRenameKit();
const { spinning, error, disabled } = useWriteIndicator(renameKitStatus);
// ...
<TreeRow>
  <div className="flex min-w-0 w-full flex-col gap-tiny">
    <div className="flex min-w-0 w-full items-center gap-single">
      <div className="min-w-0 flex-1">
        <Input
          lazy
          id="compose.sketchpad.app.kit.panel.details.section.kit.name"
          value={kitName}
          readOnly={disabled}
          onLazyChange={disabled ? undefined : (v) => void renameKit(v)}
          showLabel
        />
      </div>
      {spinning ? <Spinner size="small" className="text-muted-foreground shrink-0" /> : null}
    </div>
    {error?.message ? <p className="pl-tiny text-xs text-destructive">{error.message}</p> : null}
  </div>
</TreeRow>
```

This duplicates the row chrome from `SketchpadTriadInputRow` once for the kit name only; no shared triad type, no triad composition, no `HookTriad<...>` type used on the kit-name path. The other fields in `KitSectionForm` are unchanged.

### Tests / mocks

The test stubs at [compose/react/index.tsx](compose/react/index.tsx) lines ~17285–17289 and ~17707–17710 are simplified the same way as the production code: just construct a real `StoreField<string>` and `StoreCommand<string>` inside `createTestKitClient`, no bespoke `subscribeKitName` / `getKitNameSnapshot` / `subscribeRenameStatus` / `getRenameStatusSnapshot` shims.

- Replace the four kit-name stubs with:

  ```typescript
  kitName: new StoreField<string>(String((kitJsonFromStore(store) as KitFullDto).name ?? "")),
  renameKit: new StoreCommand<string>(async (next) => {
    const v = String(next ?? "").trim();
    if (v === "") return { ok: false, error: { kind: "InvalidValue", message: "kit name required" } };
    return { ok: false, error: { kind: "Internal", message: "embedded test client" } };
  }),
  ```

- The existing test `useKitName rejects empty required name via kit client` is rewritten against the split API: `useKitName()` returns the current name; `useRenameKit()[0]("")` resolves to `{ ok: false, error.kind === "InvalidValue" }`; `useRenameKit()[1].kind === "error"` after the call.
- The other test (`kit metadata hooks write through the kit client`) updates `setName = useKitName()[1]` to `[setName] = useRenameKit()`; other metadata hooks (still triads) are unchanged.

## Validation

- `npm run depcruise:layers` at repo root (no new cross-bundle imports).
- Type-check: `npm run build` / `tsc --noEmit` in `compose/js` and `compose/react`.
- Unit tests in `compose/react` (`useKitName rejects empty required name`, `kitName` test from `compose/js` `index.ts` line ~7522).
- Manual check of sketchpad rename flow: verify spinner appears while rename is in flight, error displays for too-long names, name updates everywhere after success.

## Ticketing (per workspace AGENTS.md rule)

Open a new ticket via repo MCP `ticket_open` for "Refactor Kit Name Reads To Cacheless Promises" before implementation; close it with `ticket_close` and the summary of touched files when done.
