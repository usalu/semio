# React ShellHost Retained-Home Bootstrap ACK P0

## Decision

`DirectoryEventPageBootstrapV1` is a valid worker-side fetch → exact-ACK → live-stream state machine, but React has no consumer for it.  The first real page would either never be opened (the shell still posts legacy `directory-open`) or, if opened manually, remain in `awaiting-ack`.  More importantly, current `PluginWasmHandle.handleAction` returns no Config receipt that proves the Home replacement reached the Config store.  Treating a successful invocation as an ACK is unsound.

The smallest honest slice is a **separate, hidden Home instance** plus one exact post-publication `DirectoryProjectionReceiptV1`.  Do not reuse `session`, `switchToManagedApp`, `foldDirectoryEvents`, `ReadConfig` opaque bytes, or a visible Home render.  It leaves the worker fetch/protocol implementation and the Home worker/client work owned by other lanes unchanged.

## Current source evidence

| Boundary | Current fact | Consequence |
| --- | --- | --- |
| Page owner | [`backbone-worker.ts:1404`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1404) accepts only `after == acknowledgedThrough`; [`:1410`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1410) compares epoch, receipt, session binding, authorization generation, and through frontier before advancing. | The React side must return exactly those fields, after—not before—Home Config publication. |
| Worker ownership | [`backbone-worker.ts:1484`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1484) posts one pending canonical page; [`:1559`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1559) alone transitions it on ACK. Reject retries from the last acknowledged frontier. | A permanent local validation fault must close the epoch, not endlessly send `directory-bootstrap-reject`. A transient retained-action failure may reject once and let the worker retry. |
| Shell ingress | [`ShellHost/🟦️.tsx:1438`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1438) handles `directory-message`, status, command-result, scope revocation, and artifact bootstrap messages; there is no `directory-event-page` or `directory-bootstrap-failed` branch. | **Decisive RED:** no page reaches `applyDirectoryEventPage`, and no ACK can be emitted. |
| Shell boot | [`ShellHost/🟦️.tsx:1677`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1677) posts `directory-open` once after identity resolution, rather than `directory-bootstrap-open`. | The old live socket bypasses the retained projection/ACK frontier. Replace this one start path; do not open both streams. |
| Visible session | [`ShellHost/🟦️.tsx:1733`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1733) creates the sole primary session; [`:2964`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2964) replaces it on app switch; [`:4260`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4260) still sends live events only to that visible session via legacy `foldDirectoryEvents`. | A `Home` projection must not be tied to the visible app. It needs its own retained instance and must use `applyDirectoryEventPage` only. |
| Real Home route | [`home editor:60`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:60) registers `applyDirectoryEventPage`; [`:69`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:69) permits Config only; [`:128`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:128) constructs `ArtifactRetainedCommandJob`, and [`:440`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:440) accepts only `pageJson`. | Invoke this real factory through `PluginWasmHandle.handleAction`, with a private Home `ActiveSession`-shaped address. No JS reducer and no raw config mutation. |
| Config truth | [`HomeConfig::apply_directory_event_page`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:126) validates the sealed page, enforces same-authority `after == cursor`, resets only at `after == 0` on changed authority, and writes cursor/binding/generation/receipt at [`:148`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:148). | The receipt must reflect this installed Config, including its frontier, not only the worker page header. |
| Missing receipt | `AppChannelClient` has wire `ReadConfig` support, but no public method in [`os/🟦️.ts:2465`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:2465); [`PluginWasmHandle`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:100) exposes neither config read nor Home receipt. The underlying Rust `ReadConfig` response is opaque pack/spr/ops at [`plugin/🦀️.rs:32108`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32108), while `InvocationResult` for Config-only paths returns `DslValue::Null` at [`plugin/🦀️.rs:20840`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20840). | **Decisive RED:** React cannot inspect or verify the installed Home Config. Do not add a second TS pack decoder or ACK on `handleAction` success. |

The existing Home source/native owner law is useful but insufficient: [`space script:80`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts:80) proves the reducer/config replacement only; it does not create a Shell, a worker ACK, or a retained app lifecycle.

## Smallest implementation split

### Slice A — sealed Config receipt (framework/Home owner)

Add one language-neutral `DirectoryProjectionReceiptV1` schema/fixture under the existing Home directory-page command ownership. Its exact five fields are:

```json
{
  "schema": "semio.space.home.directory-projection-receipt.v1",
  "sessionBindingSha256": "<64 lower hex>",
  "authorizationGeneration": 1,
  "throughSeqInclusive": 0,
  "receiptSha256": "<64 lower hex>"
}
```

`bootstrapEpoch`, `afterSeqExclusive`, and `hasMore` are worker-owner fields and must not be copied into persisted Home state or the Home receipt.

Thread this scalar receipt through the existing retained completion only **after** the `Config` preparation is terminal. For an idempotent duplicate, return the current equal receipt; for a new page, construct it from the committed candidate. The receipt cannot be exposed at reducer time, via `Effect`, or by attempting to decode an application pack in React. A narrow completed-invocation result/frame is preferable to widening `PluginWasmHandle` to arbitrary app-config deserialization; it must be tied to the exact action invocation and unavailable before Config publication.

Acceptance invariants:

- canonical page → one Config replacement → returned receipt equals `HomeConfig` binding/generation/receipt and decoded directory cursor;
- duplicate same receipt/frontier returns the same receipt with no extra config edit;
- wrong binding, authorization generation, receipt, or cursor never returns a receipt;
- cancellation before Config terminal returns no receipt and leaves no active operation;
- receipt is not a client-supplied echo.

### Slice B — retained hidden Home owner in React ShellHost

In `ShellHost/🟦️.tsx`, add a private `directoryHomeRef` whose owner has exactly:

```ts
{
  plugin: PluginWasmHandle;
  app: AppDefinition;                 // resolved live landing app, exact identity
  instanceId: number;
  viewState: ViewModel;               // private Home-only address context
  bootstrapEpoch: number;
  abort: AbortController;
  pending: null | { canonicalJson: string; sessionBindingSha256: string;
                    authorizationGeneration: number; throughSeqInclusive: number;
                    receiptSha256: string };
}
```

Create it only after all of the following are current: resolved identity, `setPluginRuntimeActor`, live host plugin handle, and `resolveRequiredHostApps(...).landing`. Use `createApp(landing.id)` once, keep it out of `shellState.pluginRuntime.session`, and retain it while `switchToManagedApp` replaces visible surfaces. Its private `viewState` has the landing app's declared default mode/window identity and current locale/terminology; it has no user-visible window or UI refresh.

Replace the post-identity `directory-open` at `ShellHost/🟦️.tsx:1680` with `directory-bootstrap-open { baseUrl, after: 0, bootstrapEpoch }` only after the hidden owner exists. Do not run old `directory-open` concurrently. Handle `directory-event-page` in the existing `worker.onmessage` before the artifact early return:

1. Require exact owner epoch and no existing `pending`; retain the entire header and canonical JSON in the owner.
2. Invoke the real `PluginWasmHandle.handleAction(owner.instanceId, encodeWindowActionInvocation(privateHomeSession, { controllerId: landing.controllerId, action: "applyDirectoryEventPage", args: { pageJson: canonicalJson }}), owner.viewState)`.
3. Await the post-Config `DirectoryProjectionReceiptV1`. Require exact equality of its binding, auth generation, through cursor, and receipt hash with the pending worker header. This validates the durable Config frontier. It is not enough that the invocation resolved.
4. Clear `pending`, then post `directory-bootstrap-ack` using the owner epoch and those four receipt fields. Only this branch may ACK.

On a recoverable action/channel failure: retain the page until posting exact `directory-bootstrap-reject {bootstrapEpoch, receiptSha256}`; clear the local pending only after the message is sent. On invalid receipt/frontier, stale owner, wrong epoch, or authorization failure: post `directory-bootstrap-close`, abort and destroy this Home instance, show a non-retryable localized failure, and require a fresh authenticated identity/bootstrap epoch. This avoids the worker's current `reject` loop for deterministic faults.

On unmount, sign-out, identity/base-url change, plugin hot-swap, or explicit cancel: first post `directory-bootstrap-close` for the current epoch, abort local receipt handling, then `destroyApp(instanceId)`. `destroyApp` is the existing real terminal lifetime path (`PluginRuntime/🟦️.tsx:1399`), so it is the appropriate cancellation boundary when `PluginWasmHandle` has no per-action abort API. Do not reuse a destroyed owner. Visible app changes deliberately do neither.

### Slice C — accessible shell status

Do not overload `BootstrapStatusNotice`: its `BootstrapUiStatus` is artifact-document-only (`host-bootstrap/🟦️.tsx:39`). Add a small directory-bootstrap status record beside the new owner, with a single EN/DE copy table selected by `uiLocale`:

- pending: `role="status"`, polite live region, current frontier and an accessible cancel button;
- retrying transport: polite status, no claim that Config progressed;
- invalid/fault/unauthorized: `role="alert"`, assertive, no raw page JSON, binding hash, session material, or hub error body;
- cancellation: clears the status after the close/destroy promise settles.

The visible landing/Studio/Space content remains usable. Locale changes update status text in place; bootstrap control never takes focus or assumes English.

## Focused acceptance packet

### Schema/source gates

1. Extend the existing language-neutral Home page fixture and owner oracle in [`space/📜️script.ts:80`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts:80) with receipt vectors: first page, successor, same-page duplicate, rebootstrap, four field substitutions, and no-receipt-before-terminal. Maintain the existing `@semio-tech/space-plugin:home-directory-event-page-owner-check` plus its native sibling; do not call either a browser acceptance.
2. Add a narrowly scoped React fixture/controller test in the renderer React target, not the giant current `index.test.ts` comment-only ShellHost boundary. It uses a fake Worker and a fake **real-shaped** `PluginWasmHandle` to prove the ordering rather than faking Config mutation:
   - hidden Home is created once while visible Home → Studio → Space switches create/destroy only visible instances;
   - exactly one retained `handleAction` with `applyDirectoryEventPage` and the canonical JSON occurs before exactly one ACK;
   - mismatched returned binding/generation/cursor/receipt emits close, never ACK;
   - duplicate is ACKed once after the idempotent receipt;
   - second worker page is not invoked until first ACK;
   - cancel/unmount while action is unresolved posts close and destroys the hidden instance; a late completion produces neither ACK nor React state update;
   - retryable rejection preserves the exact receipt/epoch; deterministic failure does not spin;
   - EN and DE status controls have the stated roles and accessible name.

The existing renderer target is `@semio-tech/framework-renderer-react:test` via [`react/📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts). Add a dedicated Nx/launch entry rather than relying on a broad suite's incidental mocks.

### Process law

Extend the existing browser-capable `@semio-tech/framework-os-dev:collab-e2e` launcher, not the hub route test, with one one-user prerequisite phase:

1. authenticated shell loads a real Home/Space package and the hub event-page endpoint;
2. observe `directory-bootstrap-open`, one canonical page, real Home Config receipt, then ACK before `streamAcknowledged` opens;
3. switch visible app Home → Studio/Space before a second page; assert the same hidden Home instance processes the page and the visible app does not receive `foldDirectoryEvents`;
4. force a receipt/frontier mismatch and assert no socket opens from that unacknowledged cursor, then cancel/rebootstrap successfully;
5. run the same step once in EN and DE and assert no secret/header is rendered.

This is process evidence only after the real hub, worker, wasm app, and React shell run. No such process or mounted-React acceptance was observed in current source.

## Nonclaims and ownership boundary

- The worker bootstrap machine, the hub endpoint, scoped-socket revocation, native/WGPU surface, Flow, and Stdio are outside this slice.
- Existing worker unit tests prove state-machine comparisons, and the Home native law proves reducer behavior. Neither proves React invocation, post-publication receipt validation, or a live ACK.
- This packet does not claim browser directory administration, event-page persistence across a browser restart, or a generic Config decoder.
- The receipt must never contain a raw session ID, page event payload, invite secret, or browser-supplied `bootstrapEpoch` beyond the worker-local ACK envelope.
