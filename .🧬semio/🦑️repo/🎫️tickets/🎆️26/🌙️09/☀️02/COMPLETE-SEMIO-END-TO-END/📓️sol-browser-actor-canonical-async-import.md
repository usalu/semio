# Browser Actor Canonical Async Import Acceptance

## Outcome

The standalone actor-import fixture is green for the exact versioned `semio:framework/host-async@1.0.0` component-model boundary. This is a real Rust WasIp2 component transpiled by JCO and executed by Node with JSPI; it is not an adapter-only JavaScript test.

The fixture proves `result<pack, pack>` and `result<stream<u8>, pack>` across the guest boundary, two independently instantiated actors, success and error lowering, guest-owned stream consumption, and a real nonzero timer suspension. The std-based guest imports fourteen exact WASI interfaces, now supplied by the first-party closed actor bundle. Both the independent third-party Preview2 oracle and the first-party closed actor path are green.

## Schema And Runtime

- Fixture root: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import`.
- Schema identity pins `semio:framework/actor-import-probe@1.0.0`, `semio:framework/host-async@1.0.0`, all fifteen exact imports, JCO `asyncMode: "jspi"`, the exact eight `asyncImports`, the intentionally unsupported filesystem interface, two actors, and bounded build/runtime/output limits.
- The standalone Rust crate depends only on pinned `wit-bindgen 0.57.1`; it has no framework, plugin, Stdio, GIS, or host dependency graph.
- WIT imports `blob-read: async func(string) -> result<stream<u8>, pack>` and `link-resolve: async func(pack) -> result<pack, pack>` and exports matching round-trip/collect probes plus `timer-roundtrip(u32) -> u32`. The timer calls Rust `std::thread::sleep`, so its clock/poll behavior crosses the real WasIp2 component boundary.
- JCO runs in a bounded Node child. This avoids Bun's unsupported `tcp_wrap` worker path, which had made a functionally successful early run exit nonzero.
- JCO's default mapping removes interface versions. The fixture supplies `map: Object.fromEntries(expectedImports.map(name => [name, name]))` and rejects any emitted import set that differs from the schema.
- JCO transpiles with `asyncMode: "jspi"` and schema-owned async entries for poll, pollable block, input blocking read/skip, and output blocking flush/write/write-zeroes/splice. The emitted evidence includes the accepted policy as `jco-policy.json`.
- JCO represents an async WIT result import as a resolved payload or a rejected error. The test's canonical tagged envelopes are therefore unwrapped on `ok` and thrown on `err`; the Rust guest receives the WIT result and exports the same semantics back through JCO.
- The emitted-source gate verifies each used blocking/poll trampoline's exact `fnName`, `.manuallyAsync = true`, and `new WebAssembly.Suspending` branch. Flipping the actual pollable-block flag is rejected before embedding.
- The first-party closed actor path uses only `activate`, supplies the exact `wasi:{nowNs,write}` capability, and runs two isolated actors. A timer remains unsettled after a microtask, takes at least its requested nonzero duration, and permits a sibling host-effect invocation to complete before it.
- Closing or aborting during a 32 ms timer advances `open/1 -> closing/1 -> closed/0` and rejects the suspended call through the owned WASI retirement path. Missing WASI capability and the intentionally unsupported filesystem import both fail closed.

## Executed Evidence

Registered command:

```text
bun nx run @semio-tech/browser-actor-import-fixture:runtime-check --skip-nx-cache
```

Terminal receipt:

```text
browser-actor-import: AJV=1 JCO=1 Wasm=1 JSPI=8 emitted=3 actors=2 pack-ok=2 pack-err=2 stream-ok=2 stream-err=2 closed-laws=14 interface=semio:framework/host-async@1.0.0 factory=raw+closed unsupported-wasi=wasi:filesystem/types@0.2.0 evidence=.../browser-actor-import-exact/actor-import-wSPwAN
NX Successfully ran target runtime-check for project @semio-tech/browser-actor-import-fixture
```

The durable evidence directory retains Cargo stdout/stderr, the component, every JCO core, emitted source and interface declarations, exact import JSON, raw and closed runtime observations, and runtime stderr. The component SHA-256 is `db0a0535fc19db8532be317e5548b358f70699a884af2dede1a536864c0e2f80`; the embedded closed module SHA-256 is `181f9cf421d027e8d2ca4f0ea0cc257a3f6ec880d5568ace1db88cdab9989848`.

## Host-Retirement Qualification

Receipt `actor-import-9Uq185` first exposed a strict-Node orphan when closing with a pending canonical host effect. The host now rejects the WIT result with an owned JSON Fault payload instead of a plain JavaScript Error, matching JCO's declared result lowering and removing that orphan without an unhandled-rejection listener.

The separate registered `pending-host-close-check` is green at `actor-import-Xekpga` with `host-close-laws=3`. It strictly decodes the rejected `Uint8Array` as `{origin:"os",code:"capability-revoked",severity:"error",message:"browser host: closed",scope:{},retryable:false}` and proves: queued close cancellation; invalid cancellation admission leaves the invocation typed while `close()` rejects its retirement AggregateError and reaches `closed/0`; abort returns the same typed fault and reaches `closed/0`.

The semantic receipt ran through the permanent `📜️script.ts` command. During this pass, Nx's project graph intermittently reported the project missing for `nx run` even while `nx show project @semio-tech/browser-actor-import-fixture` returned both registered targets. The target remains registered and must receive an Nx-level freshness rerun after the shared registry/project-graph generation stabilizes.
