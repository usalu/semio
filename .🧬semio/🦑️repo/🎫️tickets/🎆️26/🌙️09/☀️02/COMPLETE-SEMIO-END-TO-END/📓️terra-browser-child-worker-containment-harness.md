# Browser Child-Worker Containment Harness Frontier

## Verdict

The smallest honest runtime qualification is a new, lightweight `os-hub` gate beside the existing real Chromium document-open gate.  It should use that gate's Vite + Playwright setup, but **must not** reuse the whole Hub authority server, socket exchange, Cargo, WGPU, or the Shard pool.  The existing browser-bundle checks exercise the closed actor under Node; the existing OS tests and WGPU frame-worker tests use Vitest/fakes.  Neither proves browser transfer detachment or forced termination of a non-cooperative child.

The current production boundary is correctly fail-closed at `renderer-unavailable`: a verified private `DocumentExecutionTargetLease` is retained by the Backbone worker, but no browser actor body is fetched or activated.  A child executor may be introduced only after the forthcoming actor body is byte-verified and bound to that private lease; this packet does not recommend fetching or activating an actor yet.

## Existing real-browser harness to reuse

`os-hub:browser-document-open-check` is the exact reusable harness owner, not a new test framework:

| Evidence | Exact source | What it proves now | Reuse decision |
| --- | --- | --- | --- |
| Vite starts on a free loopback port with the repo dev config | [`🌎️hub/📦️packages/🦀️rust/📜️script.ts`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1942) | A browser can load repo TypeScript through Vite | Reuse exactly, but without the relay/Hub server setup above it. |
| Playwright launches Chromium and creates a real page | [`📜️script.ts`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1949) | This is browser Chromium, not a simulated Worker | Reuse.  Retain console/page-error/request-failure collection. |
| Page creates a module Worker and transfers a `MessagePort` | [`📜️script.ts`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1968) | Actual dedicated Worker construction and transferable-port delivery | The closest direct precedent. |
| The worker is served as Vite `@fs` TypeScript | [`📜️script.ts`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1990) | Vite transforms the real worker module in the browser | Use the same only in the harness.  Production should use the static `new URL(..., import.meta.url)` pattern below. |
| Registered non-cached Nx target | [`🌎️hub/📦️packages/🦀️rust/📋️project.json`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📋️project.json:398) | A repeatable gate exists | Add a sibling target, rather than overload document-open's identity/HTTP assertion. |
| Existing launch entry | [`.vscode/launch.json`](/Users/ueli/Documents/semio/.vscode/launch.json:6194) | Exact launch ordering and target naming convention | Add the sibling launch entry in the same `4_gate` section. |

The current runnable command is:

```sh
bun ./📜️script.ts nx run os-hub:browser-document-open-check --skip-nx-cache
```

The proposed containment-only target should be `os-hub:browser-actor-child-worker-containment-check`, implemented as `bun ./📜️script.ts browser-actor-child-worker-containment-check`, then registered with `cache: false` beside `browser-document-open-check` and launched with the analogous root command.  It should start only Vite and Chromium, execute the fixture, then close page/browser/Vite in `finally`; no Hub child process or Cargo test belongs in this first gate.

## What is not a browser containment proof

| Existing path | Why it is insufficient |
| --- | --- |
| [`framework-os` Vitest configuration](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🧪️tests/🟦️.ts:17) | It declares `environment: "node"`; `test-quick` and `test-long` only vary test selection.  In particular, the `execution-target-lease-browser-check` script calls a Node Vitest selector at [`Hub script:3910`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3910).  Its current “browser Worker” wording must not qualify a real child Worker. |
| [`BrowserWorkerTestScript`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:560) | Runs Vitest frame-transport tests.  Their `FakeWorker` at [`browser-frame-transport.ts:59`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/📨️browser-frame-transport.ts:59) cannot demonstrate transfer detachment, browser termination, or browser event-loop isolation. |
| [`actor/lifetime` Node worker](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts:440) | Uses `node:worker_threads` with an `eval` worker and a hand-written `self` shim.  It is useful only as a protocol unit test. |
| [`browser-bundle` actor-factory tests](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:890) | Imports the generated ESM through a Node `data:` URL at [`:1031`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:1031).  It qualifies closure and per-activation behavior, not the browser child boundary. |
| [`os-dev` bench browser harness](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:4982) | It does run headless Chromium and real browser Workers, but it intentionally runs a stub worker rather than a generated guest and carries fleet benchmark setup.  It is evidence that Playwright is installed, not the smallest containment gate. |

## Production seam and ABI constraints

The parent must be the existing Backbone Worker, which already owns one document's abort controller, runtime key, and private verified lease ([`backbone-worker.ts:222`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:222)).  `installDocumentExecutionTargetLease` verifies only manifest/component/descriptor before constructing that private owner ([`backbone-worker.ts:729`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:729)); after a valid socket grant it deliberately emits `renderer-unavailable` ([`backbone-worker.ts:868`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:868)).  Thus the containment manager belongs between that retained verified actor-body handoff (not yet implemented) and the present unavailable status.

The closed bundle ABI is:

```ts
export async function activate(identity, port, control = {})
// -> { invoke(path, args), close(), progress(), resolveEffect(), rejectEffect() }
```

It is emitted by [`closedBrowserActorBundle`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:398), with an active-invocation cap of 32 but no byte aggregate/deadline ([`:452`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:452)).  Its `BrowserActorPort` contains functions (`dispatch`, `cancelEffect`, `log`, `nowMs`, `traceSpan`, optional `wasi`) ([`host/🟦️.ts:4`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:4)); it cannot be structured-cloned into a child.

Therefore add a small first-party static child module under the browser-bundle owner (for example `🌐️actor-child-worker/🟦️.ts`), but instantiate it in the Backbone Worker with the established static URL form used by ShellHost ([`ShellHost/🟦️.tsx:1712`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1712)):

```ts
new Worker(new URL("./…/actor-child-worker/🟦️.ts", import.meta.url), { type: "module" })
```

This is a first-party fixed worker URL; the plan, descriptor, or UI must never select it.  The child receives a transferred `MessagePort` and an exact private verified closed-bundle `ArrayBuffer`, not a module URL.  It may import a blob URL created from those bytes only after the parent has admitted the actor record and transferred ownership; the closed-bundle graph validator already denies imports.  It must revoke that blob URL after import and wipe/release its byte owner on every terminal path.

The child builds a local `BrowserActorPort` that relays **bounded typed messages** over the transferred port.  The parent retains the real host/WASI authority and resolves/rejects effects back to the child by activation generation and invocation/effect ID.  Do not transfer an `AbortSignal`, pass callbacks, or let the child call Backbone's Hub/DB routines directly.  A parent `AbortController` close sends a close control message first, then forces `worker.terminate()` at the containment deadline; a synchronous `while (true)` guest cannot observe cooperative cancellation.

`ShardClient` has useful generic lifetime precedents—`WorkerLike` includes `terminate` ([`shard-client:345`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:345)), installs handlers before use ([`:1247`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1247)), and retains a pending-reply registry before posting ([`:1413`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1413)).  It is **not** a safe drop-in: `activate` accepts a caller-provided `moduleUrl` ([`:1434`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1434), and it multiplexes untrusted pooled actors.  The verified closed actor needs one exclusive child and a private verified-byte move.

## Minimal fixture and executable law matrix

Place a schema-first fixture next to browser-bundle (the protocol owner), with the Hub script consuming it in the Chromium gate.  It needs only small self-contained closed ESM rows; those are containment probes, never a claim that GIS executed.

| Law | Fixture input | Actual Chromium assertion |
| --- | --- | --- |
| Static-child provenance | fixed source-module URL, no URL in start message | Vite loads the first-party child; an injected caller `workerUrl` field is rejected/ignored. |
| Verified-byte ownership | small closed ESM bytes, exact digest and length | Parent admits exact bytes, transfers its `ArrayBuffer`, observes sender `byteLength === 0`; child reports matching digest/length before `activate`. |
| Transfer round trip | an accepted bounded `ArrayBuffer` argument and reply | Parent input detaches immediately; child reply buffer is received intact and child later reports its post-transfer result buffer detached. |
| Pre-transfer aggregate admission | per-call and aggregate `max+1` buffers | Rejection happens before `postMessage`; rejected caller buffer remains attached, no child receipt is emitted, and no byte reservation changes. |
| Tight-loop deadline | fixture operation deliberately `while (true) {}` after ready | The page's own timer fires, parent rejects that one pending call as terminated, calls `terminate`, and a newly created child completes an echo.  No “cooperative cancellation” or guest cleanup is claimed. |
| Generation fence | terminate generation 1, then start generation 2; retain stale port/message fixture | A stale completion/effect from generation 1 cannot settle or mutate generation 2. |
| Close/error retirement | activation failure and a normal active-call close | Every pending request settles exactly once, both ports are closed, blob URL is revoked, and the next activation is capacity-clean. |
| Host isolation | forged effect resolution and wrong runtime key/scope | Parent drops it before its real `BrowserActorPort`; no peer/Hub credential/lease bytes appear in any child-facing message. |

The first four and tight-loop rows are the required initial acceptance.  Keep all deadlines bounded by the fixture (for example a 100–250 ms page-side timer plus a finite Playwright wait); do not simulate a hang with a fake clock.  Verify detachment with native `ArrayBuffer.byteLength`, not a wrapper's bookkeeping.  Use ordinary `ArrayBuffer`, reject `SharedArrayBuffer` from this protocol, and avoid Node `Buffer` backing-store ambiguities.

## Immediate code/test inventory

1. Add the static child source and a parent-side `DocumentClosedActorChild` owner at the Backbone Worker execution-target seam; the owner is private to the document state and owns worker, port, pending map, byte reservations, generation, termination timer, and the transferred closed-bundle bytes until transfer succeeds.
2. Add only a typed fixture/schema and a containment function to `🌎️hub/📦️packages/🦀️rust/📜️script.ts`; register a sibling Nx target in [`🌎️hub/📦️packages/🦀️rust/📋️project.json`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📋️project.json:398), then a matching `4_gate` launch entry next to [the existing browser-document-open entry](/Users/ueli/Documents/semio/.vscode/launch.json:6194).
3. Initially invoke the static child directly from the Chromium fixture.  A later integration law must start it through Backbone only after the catalog/plan/lease delivers and verifies the mandatory closed actor bytes.  That later law can prove authenticated actor-body handoff; the synthetic containment law cannot.
4. Preserve the current `renderer-unavailable` behavior until all of the preceding pieces exist.  No asset body route, renderer selection, or user-controlled module URL should be broadened merely to make this harness run.

No runtime or build command was run for this audit.
