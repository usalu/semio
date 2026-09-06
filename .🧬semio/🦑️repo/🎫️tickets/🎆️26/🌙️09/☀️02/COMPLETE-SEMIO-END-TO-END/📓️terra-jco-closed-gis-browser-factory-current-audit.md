# JCO Closed GIS Browser Factory: Current Audit

Status: read-only source audit on 2026-09-05. No Cargo build or GIS JCO transpile was started.

## Decision

The new `instantiation: "async"` direction is the right primitive.  Unlike the current
module-materializer output, JCO's custom-instantiation surface is an `instantiate` function;
the new factory copies that function body into `__semioInstantiate` and calls it anew per
`instantiateFreshComponent`.  Thus its JCO runtime locals are activation-local.  It is not safe
to bundle the current default JCO module or current Preview2 modules unchanged and call that an
actor factory: both carry module-level state.

The current foundation proves only an import-free counter.  It is not yet a GIS actor activation
or a proof that the real host/WASI import graph, cancellation, disposal, or private plan binding
works.

**Correction — the former async-wrapper P0 was false.** The retained JCO output at
`🗑️generated/browser-component-factory/browser-component-factory-ZqFzIR/jco.stdout.json`
starts `export function instantiate(...)`, even though it was requested with
`instantiation: "async"`. Its body implements asynchronous work with `runNext`/generator and
Promise machinery; it is not an ECMAScript `async function`. Requiring `AsyncKeyword` would reject
the installed generator. The current builder correctly preserves the input declaration's actual
modifier (absent or present), reparses the reconstructed source, and rejects `import.meta`.
Root reports the retained Node two-instance law as GREEN `90548`; this audit did not rerun it.

## Exact current seams

| Concern | Current authority | Finding |
| --- | --- | --- |
| Closed JCO body and embedded cores | [`browser-bundle/📜️script.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts) lines 13-129 | Parses one JCO `instantiate(getCoreModule, imports, instantiateCore)` function, removes the only expected fetch fallback, embeds named cores, and invokes a fresh function per call. This is the reusable build seam. |
| Current web execution path | [`📦️typescript/🟦️.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts) lines 565-590 and [`store.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️store.ts) lines 239-263 | It dynamically imports a query-parameterized component and shim. That does give separate ESM copies today, but it has relative vendor/core fetches and is not the requested closed factory. |
| Current host shim | [`📦️typescript/🟦️.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts) lines 902-1057 | Its URL-derived actor identity, `effectSeq`, and `pendingEffects` are module state. The new factory must replace this with a passed, actor-local host closure. |
| Existing selected-target owner | [`backbone-worker.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts) lines 532-578 and 724-769 | `DocumentExecutionTargetLease` already privately retains verified component and descriptor bytes, zeros them on `drop`, and is minted only after plan/manifest/body/digest/descriptor checks. It is the correct owner to hand exact component bytes to the closed-bundle materializer; never expose those bytes or a caller module URL. |

`browser-bundle/📜️script.ts` is currently a source/test foundation only: no production materializer
calls it. The one integration replacement is `webMaterialize` at `store.ts` lines 249-263: replace
the emitted component module + `🟨️.js` + copied Preview2 directory + `🌉️bridge.js` closure with one
Bun-built, descriptor-bound factory ESM. Do not leave a runtime fallback to the old bridge.

## Actual GIS actor import contract

The component package is `semio:framework@1.0.0` ([`📜️.wit`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit) line 1). `world actor` imports exactly `pure` and `host-async` and exports `reactor`, `jobs`, `checkpoint`, and `describe` (lines 1319-1325). The factory must supply the JCO-emitted, literal component-interface identifiers for:

- `semio:framework/pure@1.0.0`: synchronous `log`, `now-ms`, `trace-span` (lines 911-915).
- `semio:framework/host-async@1.0.0`: 24 async operations—`storage-{read,write,delete}`, `blob-{load,write,read}`, `http-fetch`, `document-{read,write}`, `link-resolve`, `registry-query`, `io-{compose,run}`, `cache-{derive,read}`, `invoke-extension`, `open-{window,dialog}`, `dispatch-action`, `spawn-plugin-instance`, `request-{file-open,media-frames,capability}`, `spawn-job`—plus synchronous `emit` and `emit-patch` (lines 1203-1244).
- Whatever literal `wasi:*@0.2.*` interfaces the generated component requests. The existing async probe demonstrates the relevant family: CLI environment/exit/stdin/stdout/stderr/terminal resources, monotonic clock, and IO error/poll/streams. Do not hard-code their spellings from the probe: collect only literal `imports[...]` keys from the actual JCO body and compare that closed set to a schema-owned required-import manifest emitted with the bundle.

The present JS shim does map the first two interfaces to camel-cased functions (lines 1024-1055). That mapping is a useful input to the actor-local closure, not authority to retain its `import.meta.url` parsing or global pending map.

## Corrected JCO/WASI boundary and the first canonical async-stream fixture

### What is verified now

The GIS crate is a real component guest: [`Cargo.toml:80`](../../../../../../✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml#L80)
enables `component-guest`, and its `plugin_exports!` expansion reaches the SDK's
`wit_bindgen::generate!({ world: "actor" })` at
[`🔌️plugin/🦀️.rs:26-34`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs#L26).
The declared actor world is therefore authoritative. The GIS crate does **not** enable the SDK's
`component-guest-async` feature, and its own sources have no direct `host-async` call. Its shipped
component/JCO output is not retained in this worktree, so its literal emitted import-key set cannot
honestly be asserted without the prohibited GIS transpile.

The retained real JCO probe does prove the relevant lowering conventions. Its generated import
record at [`jcoprobe.js:10838-10895`](../../../../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/jcoprobe.js#L10838)
contains literal package keys and resource operations, not a generic WASI object:

- `wasi:cli/{environment,exit,stdin,stdout,stderr,terminal-input,terminal-output,terminal-stdin,terminal-stdout,terminal-stderr}@0.2.0`;
- `wasi:clocks/monotonic-clock@0.2.0` / `subscribe-duration`;
- `wasi:io/error@0.2.0`, `wasi:io/poll@0.2.0` / `poll`, `[method]pollable.block`, and
  `[resource-drop]pollable`; and
- `wasi:io/streams@0.2.0` / stream resource drops plus output `check-write`, `write`,
  `blocking-flush`, and `subscribe`.

Those resources are nominal JCO resources: the retained declarations expose `Pollable`,
`InputStream`, `OutputStream`, `TerminalInput`, and `TerminalOutput` as non-public-constructor
classes. A closed actor bundle cannot replace that set with plain records. It needs a fresh
Preview2-compatible resource adapter per activation, then passes only the literal generated keys
for that component.

For a WIT `result`, the retained JCO lowerer explicitly accepts `{ tag: "ok" | "err", val }` and
otherwise normalizes an untagged value to `{ tag: "ok", val }`
([`jcoprobe.js:3894-3915`](../../../../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/jcoprobe.js#L3894)).
Thus an ordinary host failure must travel as an `err` **value**, never as a rejected transport
Promise. Rejection remains for closure, abort, malformed reply, or worker transport failure. The
probe's `fetch-body` also proves the guest-side stream shape is
`Promise<AsyncIterable<number>>` ([`probe-host.d.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️harness/📞️out-callback/interfaces/semio-jcoprobe-probe-host.d.ts)); each byte is yielded independently.

### Minimal adapter contract

`createBrowserHostActivation` at
[`🌐️host/🟦️.ts:12-128`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts#L12)
is useful actor-local pending/stream retirement scaffolding, but it is not yet a JCO import object,
does not create Preview2 resources, and is not mounted under a verified execution-target lease.
It currently projects `blobRead` directly to a stream and `rejectEffect` directly to a rejected
Promise. That is valid only as a transport foundation; it is not a canonical
`result<stream<u8>, pack>` implementation.

The smallest first-party contract is:

```ts
type CanonicalResult<T> = { tag: "ok"; val: T } | { tag: "err"; val: Uint8Array };
type VerifiedActorAuthority = /* worker-private plan + target + scope + generation binding */ unknown;

function createActorImports(
  authority: VerifiedActorAuthority,
  port: /* exact-scope worker relay */ unknown,
  signal: AbortSignal,
): { imports: Record<string, unknown>; close(): Promise<void> };
```

`authority` is constructed only by the worker that owns the private verified target lease; it
derives the actor/generation used in frames and is never included in plugin inputs or responses.
`imports["semio:framework/host-async@1.0.0"].blobRead(hash)` must return
`Promise<CanonicalResult<AsyncIterable<number>>>`: require a tagged relay reply, map only an `ok`
`ReadableStream<Uint8Array>`/owned byte buffer into the activation-local generator, and preserve an
`err` `Uint8Array` unchanged. Apply the same tagged validation to every canonical `result<_, pack>`
method. `rejectEffect` is reserved for transport/termination. The adapter then merges those two
canonical interface records with the fresh Preview2 records under the exact literal keys extracted
from that JCO source; no key is hand-authored from a guessed GIS build.

**Mounting P0:** the current foundation's `close()` awaits every
`BrowserHostPort.cancelEffect()` promise ([`🌐️host/🟦️.ts:25`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts#L25)). A relay cancellation acknowledgement that never returns can therefore retain a
closed actor indefinitely if a worker awaits `close()` before deleting it. Make cancellation a
synchronous best-effort relay enqueue, or expose a separately bounded retained close-step; local
pending rejection, stream cancellation, resource retirement, and lease drop must not wait for a
remote cancellation acknowledgement.

### Fixture to add before GIS claim

Add a small neutral wasip2 component fixture next to `browser-bundle` which imports the **actual**
`semio:framework/host-async@1.0.0` interface and exports `probe(hash): async result<u32, list<u8>>`.
Its Rust guest calls canonical `blob-read`, consumes the returned `stream<u8>` byte by byte, and
returns the count; on the imported `err(pack)`, it returns that exact error unchanged. JCO-transpile
that fixture in custom-instantiation mode and make the generated literal import manifest an output
of the same test, not a checked-in estimate.

The first neutral vectors are: (1) tagged `ok` with `[0, 1, 128, 255]` and an exact count of four;
(2) tagged `err` with a non-empty pack, returned by the guest rather than a rejected Promise;
(3) raw/untyped reply rejected by the adapter before JCO sees it; (4) A/B interleaved streams with
different bytes and no shared resource identity; and (5) close while the stream is live, followed
by a late reply that cannot dispatch or resurrect the actor. This proves one canonical async
result-and-stream import, its generated WASI resource keys, and per-actor disposal. It does not
yet prove the absent retained GIS JCO import list or a GIS editor journey.

## Required actor-local closure

The smallest sound output is one closed ESM with immutable encoded core chunks and a single exported
activation factory. The outer ESM may retain only immutable bytes, names, source fingerprint, and
the generated function definition. Each `activate(verifiedHost, {signal, progress})` must allocate
freshly:

1. a `HostActivation` containing the already-attested `{actorId, activationGeneration, scope,
   verifiedTargetFields}`, a private `AbortController`, sequence counter, and pending-effect table;
2. the exact `pure` and `host-async` import records, whose functions close over that
   `HostActivation` and reject before dispatch when closed/aborted;
3. a fresh Preview2 adapter and all of its resources; and
4. the JCO `instantiate` call using only embedded core modules and those records.

Return an owned `{ api, resolveEffect, rejectEffect, close }`, not a raw JCO instance. `close` must
linearize first, abort the local signal, reject and clear every pending effect, cancel/close owned
streams and timers, remove actor-local resource tables, and make all late resolve/reject/messages a
no-op. The worker must retain this owner and invoke `close()` before deleting the actor entry.

This replaces query-string identity in the current `createActorApi` bridge (lines 565-575), but does
not widen authority: the worker creates `verifiedHost` only after it owns the live private
`DocumentExecutionTargetLease`; the public Shell continues to receive status only. A component is
selected by the authenticated plan/descriptor fields, not by an actor-supplied URL or directory
scan. On scope/revision expiry or document close, drop the activation then the lease; the existing
lease's byte wiping and object-URL revocation remain the terminal owner actions.

## Preview2 is a mandatory factory dependency, not a static vendor import

The installed browser Preview2 shim is observably stateful:

- `io.js` owns a module-global stream `id` ([line 1](../../../../../../node_modules/@bytecodealliance/preview2-shim/dist/browser/io.js#L1));
- `environment.js` owns global environment/args/cwd ([lines 2-4](../../../../../../node_modules/@bytecodealliance/preview2-shim/dist/browser/environment.js#L2-L4));
- `cli.js` constructs module-global stdin/stdout/stderr and terminal instances ([lines 31-88](../../../../../../node_modules/@bytecodealliance/preview2-shim/dist/browser/cli.js#L31-L88)).

Therefore `Bun.build` may inline its source, but must compile an equivalent first-party
`createPreview2ActorAdapter(host, signal)` into the ESM and create it inside `activate`. It must not
inline the current module singleton. The adapter needs only the literal WASI records collected from
the JCO factory body; every stream/pollable/terminal and scheduled clock wait belongs to one
activation and is cancelled/retired by `close`.

## Current P0/P1 corrections before treating the foundation as GIS-ready

1. **P0 — preserve the actual JCO declaration shape.** Accept the retained plain `function
   instantiate` form and preserve an explicit `async` modifier only when a separately admitted
   generator supplies it. Parse/import both forms in the source law; do not impose `AsyncKeyword`.
2. **P0 — prove and validate the real import surface.** The registered counter fixture has no
   imported interface and calls `instantiateFreshComponent({})` ([`📜️script.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts) lines 141-181). It cannot catch a missing `pure`, `host-async`, or WASI record. Add an instantiation-mode component fixture which calls a `host-async` result operation and consumes a stream; assert the generated literal import-key manifest, exact local host records, independent A/B actor state, and the expected `result<_, pack>` shape. Do not use an import-free counter as proof of GIS activation.
3. **P0 — add disposal to the public factory result.** Current factory returns only `instance`
   (lines 123-127); current shard-worker `dispose` only deletes its map entry (the generated worker
   source, [`📦️typescript/🟦️.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts), lines 352-358). A pending host effect can keep guest and host state live forever. Make
   `close()` required and test a pending async import, disposal, late completion, and a second actor
   that remains unaffected.
4. **P0 — factory-local Preview2.** The default vendor has the global state above. A Bun bundle that
   merely absorbs its import graph still shares those globals. Generate the factory adapter in the
   per-call closure; include one stream/terminal/resource identity hostile law.
5. **P0 — bind the closed ESM as an execution-target artifact.** The current protected lease owns
   only raw component and descriptor bytes ([`backbone-worker.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts) lines 532-578), and descriptor admission requires its
   `wasmSha256` to equal that raw component digest (lines 705-721). A Bun-built ESM has different
   bytes. Add exact `browserBundle` length/SHA-256/BLAKE3 plus generator/source fingerprint to the
   signed descriptor, selected plan, strict target manifest, private lease and protected target
   route; verify it before import. Do not use a public sidecar, an unverified build output, or a
   bundle URL supplied by the caller.
6. **P1 — retain strict source admission.** `closedBrowserComponentFactory` now rejects static
   imports, dynamic import, `fetch`, `URL`, workers, residual `import.meta`, and unrecognized
   top-level source. Keep its exact fallback-shape requirement and derive/validate literal
   `imports` property paths; do not turn the observed plain JCO function into an async-only fence.
7. **P1 — cancellation is cooperative, not immediate.** Decode buffers are wiped in `finally`
   (lines 98-115), which is good. But `finally { await Promise.allSettled(pending) }` (line 127)
   means a cancelled multi-core activation waits for all non-abortable `WebAssembly.compile` work.
   Specify that honestly, stop issuing further work once aborted, zero each decoded buffer, and add a
   two-core cancellation vector. Separately verify an abort while a host async import/stream is
   outstanding—today no host cancellation request exists in the WIT (line 1176).

## First executable acceptance set

1. **Build/source law:** from an actual actor-component fixture in JCO `instantiation: "async"`
   mode, AST-admit one `instantiate` body, verify exactly the generated core names and literal
   import-key manifest, and reject static/dynamic import, `import.meta`, URL/fetch, extra/missing or
   duplicate core, and an unlisted interface key.
2. **Native/JSPI law:** activate A and B with different actor-local `pure`, one `host-async`
   `document-read` result, and one byte stream. Interleave calls; prove A never observes B's
   response/stream/sequence/resource identity and outputs match an independently instantiated wasm
   oracle.
3. **Cancellation/disposal law:** abort pre-decode, during two-core compile, during a pending host
   future, and after a stream starts. In every case, no later host dispatch occurs, decoded staging
   is zeroed, pending effects reject exactly once, late replies are ignored, and B survives.
4. **Plan binding law:** mint a real verified target lease, materialize/activate only from its private
   component bytes, then close/revoke it. Assert no module URL, component bytes, lease receipt, or
   host capability crosses a worker/Shell message; a substituted component, descriptor, scope, or
   closed lease creates neither an actor nor a request.

The existing counter law is a useful pure core-isolation check, but it should remain labelled as
such until these four laws exist.
