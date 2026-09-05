# Directory Home Browser Process Acceptance P0

## Decision

**RED for the complete eight-law browser process journey, with two narrower
boundaries green.** The retained Home controller now has an actual headless
Chromium execution gate, and the source/schema/model oracle is green. The real
Hub process plus physical Space component boundary is not green: native Hub
materialization is still being established, while the exact one-crate Space
WASI materialization first stopped on this host before repository code because
the invalid guest graph reached `ring` without a WASI-capable C compiler. The
underlying boundary is now corrected: Space selects feature-free
`semio-framework-os` on `wasm32-wasip2` and retains `os-host-full` on non-WASI
targets. The corrected one-crate producer compiled through that former graph
blocker before an external `SIGTERM` interrupted the outer Nx owner; it has
resumed on the same warm dedicated target.

This packet does not promote a static development module URL to verified
activation. The Hub-selected, authenticated execution-target closure and its
private browser handoff remain separate work.

## Implemented Packet

- The fixture uses canonical Home application identity
  `s.space.home@1/*#editor`, the real plugin id `s`, and the retained action
  `applyDirectoryEventPage`.
- The language-agnostic fixture contains eight positive traces and eight
  hostile cases. AJV independently validates it; the local model proves that
  only an exact page ACK advances the frontier and that hostile epoch,
  binding, generation, through, receipt, duplicate, pre-ACK socket, and late
  completion cases advance no authority.
- The source oracle checks the real backbone worker, retained Home controller,
  and ShellHost wiring. Four mutations remove independent ACK/cancellation
  fences and must fail the oracle.
- The language-neutral fixture also declares Space guest dependency policy.
  The source gate asks Cargo for both WASI and native feature graphs: WASI must
  exclude `ring`, `cc`, `tokio`, and `os-host-full`, while native must retain
  `os-host-full`.
- The browser runtime bundles the actual retained Home controller with Bun,
  serves it from loopback, launches actual Playwright Chromium, proves the
  exact terminal receipt/ACK, closes the owner, and proves a delayed terminal
  cannot publish after close.
- The process phase is registered to build the real `os-hub` binary, run the
  existing two-restart SQLite directory event-page journey, attest the physical
  Space component, and only then exercise actual `PluginRuntime` plus Home in
  Chromium.
- Three Nx targets and three ordered launch entries expose source, Chromium
  runtime, and process phases. The process launch uses a ticket-owned artifact
  directory, one Cargo job, an explicit 24-hour budget, and the shared native
  target agreed with the Hub native acceptance work.

## Evidence Matrix

| Boundary | Result | Evidence |
|---|---|---|
| Fixture, AJV, model, source and Cargo graph oracles | GREEN | Latest registered runtime reports `directory-home-browser-process-oracle: ajv=1 model=16 source=5 hostile-source=6 guest-graph=3 native-feature=1 passed`; process source phase reports `traces=8 hostile=8`. |
| Actual Chromium retained Home controller | GREEN | Latest registered rerun: `Chromium actual same-visible-instance Hub identity, pre-open/pre-ACK refresh, ACK and late-cancel laws passed`; runtime phase reports `traces=8 hostile=8`. |
| Exact visible-row native laws | SOURCE GREEN; NATIVE STOPPED BEFORE LAWS | The registered source parity gate is green for the typed async inference conversion and the native group selects its semantic law plus three Space row laws. Retry session `33039`, receipt `exact-cargo-laws-1uMOf1/00`, stopped during the build on a concurrent durable-group `CursorRevisionAccumulator` import boundary. The cache was released to that owner; this is not a Home/host semantic-law verdict. |
| Exact Space-only materialization | ACTIVE AFTER EXTERNAL INTERRUPTION | The corrected one-crate WASI graph passed OS kernel, WIT, UI, actor, framework and plugin compilation and was compiling the declared Stdio guest dependency when the outer Nx/Bun owner received `SIGTERM` (exit `143`). It emitted no compiler error or `[budget]` line, and no child survived. The same registered command resumed on the same warm dedicated target as session `45110`; no physical attestation is claimed before its terminal. |
| Real Hub binary + two-process page | PENDING TERMINAL RECEIPT | The old registered process attempt timed out before emitting a binary. Root-owned exact Hub laws currently reserve the shared Hub cache; no duplicate process build is queued. |
| Physical Space attestation | ACTIVE PRODUCER, NO VERDICT | Registered Space-only materialization now uses session `45110`, `SEMIO_PLUGIN_ONLY=s`, one Cargo job and ticket target `home-space-component-sol-target`. The existing dev closure is mixed across three modification dates and remains rejected until the resumed producer exits and its descriptor/hash closure is attested. |
| Actual mounted Framework OS Shell | RED AFTER REAL MOUNT | Shell chrome mounted and the real shard worker started. Latest browser console reports `404 application/json` for sourcing beams/slabs/windows, then `shard0` lost/restoring `s#1` and terminated at `checkHeartbeats`; DOM became empty. See `📓️sol-actual-shell-bootstrap-producer-frontier.md`. |
| Static dev Space WASM in Chromium | NOT RUN / NOT VERIFIED ACTIVATION | It is downstream of physical attestation. Even if it runs, the harness labels `static-dev-space-wasm=1 verified-activation=0`; it uses a caller-selected development bridge URL and cannot establish Hub-selected closure authority. |
| Full eight-law browser process journey | RED | No actual ShellHost/backbone worker process currently owns the complete fetch/ACK/socket/gap/disconnect/rebootstrap/restart journey. |

## Exact Commands

Source gate, exited 0:

```text
NX_ISOLATE_PLUGINS=false bun ./📜️script.ts nx run os-hub:directory-home-browser-process-source-check --skip-nx-cache
```

Runtime gate, exited 0:

```text
NX_ISOLATE_PLUGINS=false bun nx run os-hub:directory-home-browser-process-runtime-check --skip-nx-cache
```

Process gate, run with the registered target and shared ticket-native cache:

```text
NX_ISOLATE_PLUGINS=false SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated/directory-home-browser-process-exact CARGO_TARGET_DIR=<ticket>/🗑️generated/space-public-boundary-sol-target CARGO_BUILD_JOBS=1 SEMIO_BUILD_BUDGET_MS=86400000 RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER= bun ./📜️script.ts nx run os-hub:directory-home-browser-process-check --skip-nx-cache
```

Exact one-crate Space materialization (the original pre-graph-repair invocation
exited at the external C toolchain; the corrected resumed invocation uses the
same registered shape and warm ticket target):

```text
NX_ISOLATE_PLUGINS=false SEMIO_PLUGIN_ONLY=s CARGO_TARGET_DIR=<ticket>/🗑️generated/home-space-component-sol-target CARGO_BUILD_JOBS=1 SEMIO_BUILD_BUDGET_MS=86400000 SEMIO_MATERIALIZE_CONCURRENCY=1 bun nx run @semio-tech/framework-os-dev:plugin --skip-nx-cache -- s
```

After the Home, execution-target, WAL state, and WAL recovery launch-seed
additions settled, the plugin-registry generator exited 0 with `59 plugin
crates, 60 playgrounds, 45 framework packages` and regenerated
`.vscode/launch.json`. The direct Nx freshness target then exited 0 with
`plugin registry generated catalog and launch bytes are fresh.` The generated
launch retains the Home source/controller/process entries, the shared native
target and 24-hour process budget, the WGPU retained-Home 24-hour budget, WAL
orders `.067` through `.070`, durable Map orders `.071`/`.072`, and the
registered plugin-host inference conversion source gate at order `407.66`.

## Ownership and Registration

- Fixture:
  `🌎️hub/📇️directory/🧫️fixtures/🌐️directory-home-browser-process-v1/🔣️.json`
- Harness and command router:
  `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- Nx targets:
  `🌎️hub/📦️packages/🦀️rust/📋️project.json`
- Launch seed and generated launch:
  `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`

## Honest Limits

- The green Chromium law executes the actual retained Home controller but uses
  a real-shaped plugin handle; it is not a mounted full ShellHost plus backbone
  worker plus live Hub journey.
- The downstream static module exercise, if unblocked, uses the actual
  `PluginRuntime` and actual Space component bytes but not a Hub-selected
  `DocumentOpenPlan`, private verified activation closure, or authenticated
  execution-target byte handoff.
- This packet does not establish GIS Map activation, WGPU behavior, Stdio
  document transport, or caller-independent module selection.
- The real two-process page phase is necessary producer/SQLite/restart evidence,
  but by itself does not prove a global directory WebSocket, the
  fetch-to-socket gap, disconnect retry, session rebootstrap, or duplicate-free
  retained projection across all eight laws.
- No current evidence supports calling the complete browser process acceptance
  green. Promotion requires a fresh, hash-consistent physical Space closure and
  an actual browser owner wired to the Hub-selected verified activation handoff.
- The Space target boundary now keeps `os-host-full` out of the guest graph.
  Its resumed registered materialization still needs a terminal receipt before
  the physical closure may be treated as usable; exit `143` from the preceding
  corrected run was an outer orchestration `SIGTERM`, not a compiler verdict.
