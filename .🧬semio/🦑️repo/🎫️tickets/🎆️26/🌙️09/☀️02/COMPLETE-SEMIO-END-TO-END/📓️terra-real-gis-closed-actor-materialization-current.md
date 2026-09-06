# Real GIS Closed-Actor Materialization: Current Readiness

## Verdict

Home's genuine guest-drop receipt is a meaningful JCO/JSPI host-lifecycle prerequisite, but it is **not** evidence for the GIS plugin. Its terminal `actor-import-5UDfwr` receipt proves the fixture guest's stream drop/cancellation behaviour only; it explicitly withholds production GIS activation. [Sol's receipt](./sol-browser-actor-canonical-async-import.md#Genuine-guest-stream-drop)

There is still no real, accepted GIS materialization in the current ticket. A read-only unignored-file scan found one peer-produced Stdio Wasip2 artifact under `🗑️generated/home-space-component-sol-target`, no `semio_s_plugin_gis.wasm`, and only fixture/staging `closed-actor.mjs` files. Those staging files cannot stand in for a catalog generation. No Cargo, JCO, browser, or native target was run for this audit.

The exact next execution boundary is already registered:

```
⚖️gate🧬️trusted-stdio-gis-bundle-native🌎️hub
```

It invokes `nx run os-hub:trusted-stdio-gis-bundle-check --skip-nx-cache -- --native`, sets the ticket-owned `SEMIO_TEST_ARTIFACT_DIR` to `🗑️generated/trusted-stdio-gis-native`, and holds `CARGO_BUILD_JOBS=1`. [launch.json](/Users/ueli/Documents/semio/.vscode/launch.json:6313) It is the correct first real GIS launch target; the process/rotation target is subsequent evidence, not a substitute.

## Actual producer closure

The native gate first builds its own ticket-owned `os-hub` target, then invokes the materializer; it does not consume ambient target artifacts. [Hub script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:6231)

The materializer creates a random private build root and staging root, then always produces Stdio followed by GIS. Each request uses a separate target (`stdio-target`, `gis-target`), and GIS's fresh-component lease is consumed by `buildClosedBrowserActorArtifactV1` before it may be retired. [Hub script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5336) [Hub script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5365)

The fresh producer itself always runs:

```
cargo rustc -p semio-s-plugin-gis --lib --crate-type cdylib \
  --target wasm32-wasip2 --profile wasm-release
```

with a private `CARGO_TARGET_DIR`, incremental compilation/wrappers disabled. It snapshots that exact component before JCO extraction/descriptor generation and stages only verified component and descriptor bytes. [fresh producer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:417)

Therefore a peer's current Stdio or future GIS `target/...wasm` is **not an admissible materializer input**: the current API accepts no input path or prior receipt and unconditionally creates/fills the private fresh target. A peer artifact may be inspected diagnostically, but it cannot be promoted by hash comparison alone. The only presently supported independent verification is a new materializer run that snapshots and validates its own output. This prevents an unowned peer target from acquiring descriptor, core, policy, or generation authority.

## Expected WIT versus the fixed actor policy

The declared framework component world has exactly two direct imports and four required exports:

| Kind | Static WIT contract |
| --- | --- |
| Direct imports | `semio:framework/pure@1.0.0`, `semio:framework/host-async@1.0.0` |
| Required exports | `reactor`, `jobs`, `checkpoint`, `describe` |

[Actor world](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:1319)

That source world does **not** establish the generated GIS component's full Preview2 import closure. GIS does enable the framework component guest and depends on Stdio's full artifact catalog, so only the native producer's JCO manifest is authoritative for the actual interface set. [GIS Cargo manifest](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:66)

The closed-actor builder admits only the sorted subset of this fixed vocabulary:

- the two Semio interfaces above;
- `wasi:cli/{environment,exit,stdin,stdout,stderr,terminal-input,terminal-output,terminal-stdin,terminal-stdout,terminal-stderr}@0.2.0`;
- `wasi:clocks/monotonic-clock@0.2.0`;
- `wasi:io/{error,poll,streams}@0.2.0`.

[Policy list](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:22) [WASI implementation vocabulary](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:8)

For a component that actually needs WASI, the policy further pins JSPI lowering of eight blocking poll/stream operations; it does not permit a synchronous Promise-shaped substitute. [JSPI list](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:23) The build generates with that fixed interface map and rejects any JCO-reported import outside it before the bundle is closed. [Generator admission](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:298)

Thus the next native run has only two sound outcomes:

1. it persists the actual, sorted GIS `importInterfaces` subset and a closed actor bound to the fresh component/descriptor; or
2. it rejects an unsupported generated import or invalid JSPI suspension before publication.

It must not widen the policy based on the Home fixture or a Wasip2 assumption. In particular, filesystem, socket, random, or wall-clock interfaces remain denied unless a separately justified policy slice is designed.

## Required observed output

`component.wit`, JCO extraction files, and raw cores are producer work files and are retired in the fresh producer's `finally`; their mere appearance under a peer target is not a result. [fresh producer cleanup](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:417)

For an accepted native run, inspect the returned native receipt and the immutable generation it names. The generation verifier requires this exact closure:

```
trusted-catalog.json
packages/gis/component.wasm
packages/gis/descriptor.semio
packages/gis/browser/closed-actor.mjs
packages/stdio/component.wasm
packages/stdio/descriptor.semio
```

and rejects symlinks, extra/missing entries, non-regular inputs, incorrect lengths, or incorrect hashes. [generation verifier](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5208)

The catalog GIS row must carry a `closed-browser-actor` record whose path is the fixed `packages/gis/browser/closed-actor.mjs`, with the closed-byte SHA/length, component and descriptor source hashes, policy hash, and JCO-observed import list. [catalog assembly](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5371) The same run must then load the generation in a candidate Hub and issue an authenticated GIS Map plan before printing `trusted-stdio-gis-bundle-native-receipt`. [native gate](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:6243)

This proves fresh component → descriptor → policy-bound closed bytes → trusted generation → selected plan. It does **not** prove the actor can be fetched, reserved, invoked, render, or persist a GIS edit: the private Backbone reservation/worker containment and later scoped host-effect bridge remain mandatory before any actor body route is enabled.

## Sequencing recommendation

1. Run the registered native gate above when the shared Cargo capacity is available; retain its receipt/error in the ticket-owned artifact directory.
2. On success, use the persisted catalog record—not temporary JCO/Wasm files—as the input to the pending private Backbone reservation seam.
3. Only after reservation, worker containment, and protected body handoff are qualified, perform a real actor activation. Then bind host effects to the authenticated scope and the fixed-three Store/WAL approval path.
4. Run the registered process/rotation gate after native materialization, not before it.

