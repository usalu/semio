# Browser Per-Activation Preview2 Contract

## Decision

The closed actor bundle needs a first-party Preview2Activation, one fresh instance per activate(identity, port, control) call. It must construct the exact JCO import record from the verified JCO import manifest. It must not import @bytecodealliance/preview2-shim in the bundle: that package is a useful test oracle, but its io.js has module-global resource ids and cli.js has module-global stdin/stdout/stderr instances.

This is not yet a browser GIS execution claim. The current closed actor builder deliberately rejects every WASI interface, so it cannot instantiate a real Rust Preview2 guest today.

## Current evidence

- [browser-bundle script](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:13) maps only semio:framework/pure@1.0.0 and semio:framework/host-async@1.0.0, then rejects every other requested interface at lines 14–18. Its generated lifecycle creates only createBrowserHostActivation and closes only that host at lines 21–42.
- [Browser host](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:13) is already per activation. It has no global pending requests or guest streams, and admits 128 pending requests, 128 streams, 1,048,576 copied buffered bytes, and a 65,536-byte stream chunk (lines 16–18, 50–55, 80–82, 99–104). It supplies all 24 host-async functions at lines 132–145, but no raw WASI stdin/stdout, resource, or timer capability.
- The retained JCO actor-import proof accesses literal @0.2.0 input keys: custom semio:framework/host-async@1.0.0, CLI environment/exit/stdin/stdout/stderr/terminal-input/terminal-output/terminal-stderr/terminal-stdin/terminal-stdout, wasi:clocks/monotonic-clock, and wasi:io/error/poll/streams; see [JCO imports](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-tQ9aw0/jco.stdout.json). Its generated TypeScript/debug labels say @0.2.9; do not replace the literal source keys with those labels.
- Its emitted declarations prove the resource constructors and minimum methods: [streams](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-tQ9aw0/jco/interfaces/wasi-io-streams.d.ts), [poll](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-tQ9aw0/jco/interfaces/wasi-io-poll.d.ts), and [clock](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-tQ9aw0/jco/interfaces/wasi-clocks-monotonic-clock.d.ts).
- JCO captures host-owned resources and invokes instance Symbol.dispose or Symbol.for(dispose) upon guest-owned handle disposal; see [generated disposal](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-tQ9aw0/jco/actor-import.js:9414). Instance disposal is mandatory. A static cabiDispose needs an activation-local representation map only if a future generated artifact actually reaches that fallback.
- The current genuine JCO/Rust test uses the third-party shim solely as its runtime oracle in [actor-import script](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/📜️script.ts:95). It validates only two direct host-async calls and streams; line 88 expects the closed actor bundle to reject WASI.

## Exact first-party seam

Add 🌐️host/🧩️preview2/🟦️.ts beside the browser host. It must be domain-neutral: it owns no plan, document, space, or plugin selection authority.

~~~ts
export type BrowserWasiChannel = "stdout" | "stderr";
export type BrowserWasiResult<T> =
  | Readonly<{ tag: "ok"; val: T }>
  | Readonly<{ tag: "err"; val:
      Readonly<{ tag: "last-operation-failed"; val: BrowserWasiIoError }>
      | Readonly<{ tag: "closed" }> }>;

export interface BrowserWasiPort {
  writeWasi(channel: BrowserWasiChannel, bytes: Uint8Array): BrowserWasiResult<void>;
  flushWasi(channel: BrowserWasiChannel): BrowserWasiResult<void>;
  monotonicNowMs(): bigint;
  scheduleWasiPoll(deadlineMs: bigint, signal: AbortSignal): Promise<void>;
  wasiExit(status: Readonly<{ tag: "ok" | "err" }>): void;
}

export interface BrowserPreview2Activation {
  imports(requiredInterfaces: readonly string[]): Readonly<Record<string, object>>;
  close(): Promise<void>;
}

export function createBrowserPreview2Activation(
  identity: BrowserHostIdentity,
  port: BrowserWasiPort,
  signal?: AbortSignal,
): BrowserPreview2Activation;
~~~

closedBrowserActorBundle must create host and wasi in the same activation scope. It calls wasi.imports(__semioRequiredInterfaces), adds the two exact Semio imports from host, and passes only that record to instantiateFreshComponent. The helper rejects a missing, duplicate, undeclared, or unsupported literal key. It must not accept a caller-provided import object, URL, or version.

The actor close path must settle wasi.close(), host.close(), and started calls under the existing aggregate-error discipline. The WASI port remains distinct from BrowserHostPort: stdout/stderr are arbitrary bytes, not strings for console or log; exit is lifecycle control, not successful return. writeWasi and flushWasi are synchronous, copy guest bytes before return, and must never retain a view into linear memory.

### Local classes and ownership

createBrowserPreview2Activation declares fresh private classes every time:

- WasiIoError: bounded toDebugString().
- WasiInputStream: read, blockingRead, skip, blockingSkip, subscribe, instance disposal. Each getStdin() returns a new closed resource: read returns the WIT closed result and its pollable is ready. Do not expose DOM input and do not return an empty-open stream that can spin.
- WasiOutputStream: checkWrite, write, blockingWriteAndFlush, flush, blockingFlush, writeZeroes, blockingWriteZeroesAndFlush, splice, blockingSplice, subscribe, instance disposal. Its fallible operations return WIT result tags. Every getStdout()/getStderr() creates a new wrapper over an activation-local channel—returning one singleton is incorrect because JCO disposes an owned returned handle.
- WasiPollable: ready, block, disposal. It owns one ready closed-stream event, output event, or timer registration. Parent/activation close retires all its children.
- WasiTerminalInput and WasiTerminalOutput: constructor classes only; all three terminal getters return undefined.

getEnvironment() returns an empty list; it does not reveal process, browser, account, plan, document, or space data. exit(ok or err) calls wasiExit once, aborts the activation, and throws an internal tagged exit observed by actor lifecycle. It must not no-op.

Use an activation-local ticket table. Dispose a resource before reclaiming its ticket. No global id, class, resource map, timer, output buffer, or singleton stdio object is allowed. Unsupported Preview2 filesystem, sockets, random, wall-clock, and HTTP keys are rejected before instantiation; tile/provider access remains the scoped host-async.httpFetch route.

| Boundary | Bound |
| --- | ---: |
| total live Preview2 resources, including derived pollables | 256 |
| pollables supplied to one poll | 128 |
| one copied input/stdout/stderr chunk | 65,536 bytes |
| aggregate copied stdout+stderr bytes per activation | 1,048,576 bytes |
| environment entries / terminal input bytes | 0 / 0 |

The requested 256-resource ceiling is coherent with the root plan; the poll and byte ceilings retain the current host's 128/65,536/1,048,576 limits. Refuse before allocation or forwarding at bound + 1. checkWrite returns no more than one-chunk capacity and the remaining aggregate capacity. Closed/retired/exhausted resources return closed or last-operation-failed(WasiIoError) and never fall through to another activation.

## Required JSPI policy

The retained generated actor-import output's _trampoline18 calls Pollable.block() synchronously; see [generated code](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/browser-actor-import-exact/actor-import-tQ9aw0/jco/actor-import.js:7177). A returned promise is not awaited. Therefore nonzero clock waits cannot be implemented honestly in that current output. No Atomics.wait, busy waiting, main-thread block, or unawaited timer promise is admissible.

The installed JCO exposes the required policy in [JCO CLI source](/Users/ueli/Documents/semio/node_modules/@bytecodealliance/jco/dist/jco.js:74). Its build-time transform must use asyncMode: jspi with exactly:

~~~ts
[
  "wasi:io/poll#poll",
  "wasi:io/poll#[method]pollable.block",
  "wasi:io/streams#[method]input-stream.blocking-read",
  "wasi:io/streams#[method]input-stream.blocking-skip",
  "wasi:io/streams#[method]output-stream.blocking-flush",
  "wasi:io/streams#[method]output-stream.blocking-write-and-flush",
  "wasi:io/streams#[method]output-stream.blocking-write-zeroes-and-flush",
  "wasi:io/streams#[method]output-stream.blocking-splice",
]
~~~

Those selectors are JCO's installed ASYNC_WASI_IMPORTS list in [common.js](/Users/ueli/Documents/semio/node_modules/@bytecodealliance/jco/dist/common.js:9). Bind JCO version, async mode, and the exact selector list into the generated browser artifact/descriptor digest; a verified component cannot switch lowering policy after plan verification. With JSPI, a clock pollable calls scheduleWasiPoll with its abort signal. Segment long u64 waits rather than relying on browser timer clamping. Close aborts and awaits all timer registrations. Until a JSPI artifact is generated and accepted, nonzero subscribeDuration must fail in a controlled way rather than pretend to wait.

## GIS boundary

[GIS Cargo](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:63) enables component-guest. The common production actor WIT imports pure at [line 910](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:910) and 24 host-async methods at [line 1179](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:1179). Thus a GIS browser component additionally needs the literal custom semio:framework/pure@1.0.0 import. Current host source covers all 24 host-async names, though the retained JCO probe exercises only blobRead and linkResolve.

No actual GIS JCO output/import manifest is retained in the worktree. Its complete WASI list is therefore not source-proven. The first GIS bundle must persist JCO's literal import list and reject it unless it is an admitted subset. The expected baseline is the actor-import CLI/environment/exit/io/poll/monotonic set; an extra filesystem, socket, random, wall-clock, or HTTP key is a fail-closed admission result, not a reason to add a dummy shim.

## Independent corpus

Extend the existing @semio-tech/dev:closed-browser-component-factory-check registration in [dev script](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:5113). Add a schema-backed browser-preview2-activation-v1 fixture with a genuine Rust wasm32-wasip2 guest that writes stdout, starts a monotonic wait, polls/blocks, calls one direct host-async method, and disposes resources.

1. Raw JCO with installed Preview2 shim is the third-party protocol oracle; result bytes must equal the first-party closed bundle. The shim stays test-only.
2. Two activations write distinct data, drop handles, and continue independently. Passing actor A's resource into B fails due fresh per-activation constructors.
3. A nonzero clock wait suspends only under emitted JSPI lowering; B remains runnable. Closing A while blocked has no late completion and does not affect B.
4. Environment is empty; terminals are undefined; stdin is closed; exit closes one actor; an @0.2.9 substitute or undeclared wasi:filesystem key is rejected before instantiation.
5. Resource 257, poll list 129, a 65,537-byte write, and aggregate output byte 1,048,577 are denied before port forwarding. Repeated disposal/close leaves no live resource or timer.

The test must execute the genuine component and assert the generated JSPI lowering; TypeScript-class-only tests do not qualify the ABI.

## no_std

A no-std direct-async GIS guest is not a small parallel path. [Plugin Cargo](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml:68) deliberately enables wit-bindgen std and async, and component-persistent actor state directly uses std cell/sync in [plugin source](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🦀️.rs:70). GIS also carries the normal serde/JSON SDK graph. The existing actor-import component proves real wasm32-wasip2 async behavior, not a no-std ABI. Do not add a second no-std host ABI or fake allocator in this slice.

## Order

1. Add the private, per-activation Preview2 resource module and synchronous output/exit port.
2. Merge its exact verified imports in the closed actor bundle; close it with the existing host.
3. Generate JSPI source with the listed selectors and bind the configuration into the bundle digest; land the real timer/resource fixture.
4. Generate and verify the real GIS JCO manifest before admitting GIS browser execution.

