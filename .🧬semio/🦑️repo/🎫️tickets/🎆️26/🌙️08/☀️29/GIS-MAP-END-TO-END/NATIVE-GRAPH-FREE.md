# Graph-Free Native GIS2D

## Launch

```sh
SEMIO_RUNTIME_DIAGNOSTICS=1 repo ./📜️script.ts dev playground gis2d native prototype
```

The root playground request admits the exact native renderer and profile under a fresh Original. It prepares or reuses the registered catalog, including the Rust projection embedded by the GIS host, then the profile-specific playground session. The session supplies the model closure and the host declaration. A missing model invokes the shared model-only driver. Browser builds also publish these retained models, so the browser output can supply them.

The native runtime publisher takes a shared artifact group over the genuine component models. It uses the model owner reader to verify the bytes, descriptors and receipts, including both actor-preparation sidecars if present. The native runtime artifact contains the existing runtime manifest and the component model, JSON descriptor, packed descriptor and receipt for each plugin. The native ProgramBridge opens the Wasm and JSON descriptor named in the manifest using the recorded digest. The sidecars remain in the model publication for consumers that use them.

GIS2D declares the GIS services Rust package, its native binary and target. The graph-free path resolves its source-owned project, Cargo and script through the native host admission and accepts a self-contained, profiled script command. The same Original supervises the bounded child process and issued StoreReceipt. The Cargo publisher stages the executable and source record under the canonical compiler product key, ending in the exact profile. The existing native resolver consumes the declared receipt and pins the executable. Variants without a custom host use the existing managed native WGPU renderer producer and its retained executable reader. The existing native runner owns the window process, asset server, module and session reads, startup policy and cancellation.

Native launch gets the renderer assets directly from the repository root. The Canvas runtime asset roster supplies the six tracked fonts and the generated icon pack. If the pack is missing, the owned Rust icon catalog generator publishes it before compilation. All seven inputs and the catalog Rust projection were present on this host. The renderer WASM package and guest-slim font bundle are browser prerequisites.

## Verification

- The language-neutral root cases cover native prototype and release and reject extra flags. The focused test first failed and then passed with the schema change. The dev birth suite passed all 13 tests, including its Ajv schema oracle and the host recipe vectors checked against an independently parsed TOML declaration.
- A lightweight owned child process observed the exact Original capabilities and prototype mode through the native caller phase.
- The authored host recipe test covers both profile commands and rejects a foreign path, unexpected flags, shell syntax and a dependency-bearing target. The native host laws passed, including wrong project, target and receipt profile, duplicate receipt entries, forged links and foreign outputs.
- The native runtime manifest contract test passed, including duplicate module rejection, wrong profile and escaped or foreign paths. The runtime publisher, entrypoint and GIS owner scripts imported successfully.
- The root help printed `<react|wgpu|native>`. An unknown variant reached the owned session lookup and was refused. A conflicting `SEMIO_BUILD_MODE` was rejected at birth.
- The canonical GIS Cargo publication paths resolved to `.cache/b/r/m/published/b76f1f02f139925e10a7/prototype` and its `release` sibling on this host. The previous output directory basename `native-prototype` failed the existing receipt contract, which requires `prototype`.
- Native compilation and window interaction were deferred while the browser GIS WASI link held the compiler slot. The native window, tiles and gestures still require a live run.

## Source Changes

- Repo root command registry and dev birth schema, parser, playground fixture and birth tests.
- Repo root playground orchestration and the native-host recipe schema and source resolver.
- WGPU native runtime publication and its existing publish script wrapper.
- The GIS-owned native Cargo producer publication destination.

## Native Cargo Source Drift and Retry

The first native host run published the GIS runtime and compiled the declared host dependencies. It encountered Rust E0599 while the renderer consumer referenced a method whose producer was concurrently updated. Two later child attempts reported an admitted Source digest change and the child exited with status 75. The live producer Source now declares the method. A fresh native invocation is already running.

The diagnostic `[cargo-plan-drift]` is per build attempt. The Cargo planner compares captured plan identity before producing it. Its `graph-comparison-limit` category means the diagnostic walk stopped before it could isolate the changed dependency. The Cargo driver raises a typed `ConcurrentEditErrorV1` with reason `input`, which its convergent build loop handles before the final Script process failure handler. That handler uses status 75 for several restartable classes. The parent receives an owned command failure and has no typed Source evidence from that status.

A future transport should preserve only terminal, validated Source churn for the selected native host. The child Script birth boundary can emit a versioned bounded stderr record after its admitted invocation and custody have settled, immediately before the existing restart handler may exit. The producer must derive the record from the terminal error using `concurrentEditOfV1`. A request identifier issued by the parent should bind the record to the actual child. The direct owner process should read and remove that identifier from its environment before invoking Cargo so compiler subprocesses cannot echo it. The parent should observe only the exact record prefix and retain at most one matching record, then accept it only with an unsignalled failure using the expected terminal status and with the canonical selected manifest or an admitted workspace path. It should settle the failed StoreReceipt before restoring the error for a fresh root Original. Child build rank must be interpreted explicitly at this boundary because the child has already exhausted its own build loop.

The preparation custody diagnostic is the existing model for bounded serialization and recovery. Its current schema accepts only `bytes` and `vanished` at the pre-build rank, so this separate native host case needs a reason and rank contract that covers terminal Cargo `input`. The focused test should run real lightweight owned children with one valid terminal record and negative cases for bare 75, per-attempt diagnostic text, malformed fields, duplicate records, wrong request identifier, foreign path, cancellation, success, and a primary-plus-cleanup failure. Expected recovery can be independently checked against a language-neutral JSON corpus. A restored terminal error should exercise the root Original retry fixture and prove the previous Original and pending StoreReceipt settled before the next attempt. A persistent unchanged plan should stop within the existing bounded retry policy.

This diagnosis and design were recorded without changing native runtime or compiler code during the warm retry.

### Implemented Native Retry Boundary

The second warm host attempt reached a different transient E0599 in the runtime archive operations, followed by the same terminal source-plan drift. The next fresh native invocation was already running while this patch was verified.

The owned GIS script now accepts the request-scoped diagnostic identifier from its root launch and removes it from the environment before running Cargo. The Script process receiver calls its opt-in terminal observer after joining accepted work and before the normal exit classification. The observer emits only a real build-layer `ConcurrentEditErrorV1` with kind `input` and a canonical path inside the workspace. The root collects one bounded diagnostic from its child stderr and restores a typed build edit only after the original failure and pending StoreReceipt have settled. No other host receives recovery from status 75 alone.

A real-process fixture passed all 13 vectors and 30 assertions. Recovery passed in prototype and release, including a terminal edit after the child exhausted its own build loop. The fixture observed accepted work draining before the diagnostic. It refused an ordinary status 75, a per-attempt diagnostic, a mismatched identifier, malformed or duplicate records, a foreign source path, success, an ordinary nonzero exit, mixed cleanup failure, a cancelled command, and a valid-shaped record sent to stdout. The selected root restart case also passed and confirms a fresh Original is born after the previous scope settles.

The GIS owner script imported successfully. A bare dev birth suite invocation failed the standalone GIS and Space test with `Vite config requires its original bounded compiler cache`. With `SEMIO_VITE_CACHE_DIRECTORY` pointing to the declared products namespace at `.cache/b/p/m/gis-dev-birth-fixture`, all 13 dev birth tests passed with 276 assertions. The fixture only constructed the Vite configuration. No Rust compiler or renderer was started by these checks.
