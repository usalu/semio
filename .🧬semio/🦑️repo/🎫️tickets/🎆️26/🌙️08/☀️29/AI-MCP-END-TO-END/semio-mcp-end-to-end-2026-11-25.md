# 🌉️ Semio MCP End to End (2026-11-25)

Goal: the `semio` server in `.mcp.json` (`repo dev mcp stdio os …`) connects in an MCP client and serves discovery and tool calls.

Probe: `probe-semio-mcp.sh <out> [seconds]` (this folder) launches the exact `.mcp.json` command, sends `initialize`, `notifications/initialized`, `tools/list` and timestamps every stdout frame.

## Findings

### 1. Handshake blocked by the root dependency gate (fixed)

- Symptom: Claude Code reported `MCP server semio connection timed out after 30000ms`.
- Measured: the warm gateway answered `initialize` only at t+92 s.
- Cause: `📜️script.ts` registers every root command behind `dependencyGateV1`. `dev` is `requires: "dependencies"`. The gate skips only for build-CLI-issued invocations; an MCP client launches through the Go launcher, so every connect ran a blocking `repo dependencies ensure` (source admission, generation verify, rustup check) before the `dev` module — and so the warm gateway — was loaded.
- Fix (schema-first):
  - `🗂️registry/🧬️schema/🔣️.json`: `Requires` gains `staged`; commands may declare `routes` (`segments`, `requires`, `reason`); label `requiresStaged`.
  - `🗂️registry/🔣️.json`: `dev` declares route `mcp stdio os` as `staged`.
  - `🗂️registry/🟦️.ts`: `rootCommandRequirementV1` (longest route prefix), route admission, help, gate receives the route segments.
  - `🏃️process/🧭️routing/🟦️.ts`: lazy loaders receive the segments after the command name.
  - `🥾️provision/🟦️.ts`: `ensureRootDependenciesV1`; a `staged` route with installed dependencies loads at once.
  - `🛠️dev/🟦️.ts` `runMcpOs`: the warm stage ensures dependencies as its first step.
- Measured after fix: `initialize` answered at t+1 s; provisioning progress continues on stderr.

### 2. Native build fails source admission (in progress)

- After staging starts, `buildMcpArtifactV1` fails: `process.source.unresolved-generated-module` for `icon_name` in `🖼️assets/🔣️icons/🧬️schema/🦀️.rs` (owner `@semio-tech/assets:build`); queued `tools/list` is refused with `semio-os-mcp is unavailable: staging exited with status 1`.
- Report: `mcp-native-build-staging.md`.

### 3. Discovery while warming (fixed)

- Claude Code honors `list_changed`; a discovery request queued for minutes exceeds the MCP SDK's 60 s request timeout.
- The gateway now answers `tools/list`, `resources/list`, `resources/templates/list`, `prompts/list` with the empty lists of schema `WarmDiscovery` and announces `list_changed` once the native server is ready. Report: `warm-gateway-discovery.md` (gateway tests 25 pass).
- The client journey (`driveOsMcpClientSessionV1`) waits for `notifications/tools/list_changed` from a warming server (`awaitMcpNativeCatalogV1`, deadline in schema `ClientReadiness`).

### 4. Resident graph service blocks every `repo run` (fleet, not fixed here)

- `repo run …:build` died after 11 min: slice worker exited by signal (ref 9a7af9ea); `repo run …:generate --graph-policy=current` timed out after 590 s waiting for a free slice worker.
- Fallbacks: catalog produced in-process (`catalog-direct-generate.ts`, report `mcp-catalog-generation.md`); native build prerequisites (`mcp-native-build-staging.md`).
- Revalidation design so a usable published server never waits on the build CLI: `warm-gateway-revalidate.md`.

- Revalidation: when a published binary and catalog exist, the gateway serves them at once from a private executable copy, keeps component readers held, refreshes in the background and hot-swaps the native child. Swapped servers get gateway-unique server-request ids; client session state (log level, subscriptions, roots) is replayed; the candidate handshake has a timeout. An independent review found 7 swap and lease defects, all fixed: gateway 43 pass, staging 20 pass, publication-revalidate 7 pass.
- Router: `ScriptRouter.registerGated` runs the route gate on every dispatch, loads the command module once and retries a failed load. `ensureRootDependenciesV1` runs once per process and ports, and does not memoize a failure (registry 68 pass, routing 50 pass).

### 4b. Native binary prerequisites (fixed), build infrastructure (blocked)

- The staging schema now declares `Producer.prerequisites` (generator owners and the files they produce). Their missing files are produced before Cargo admission. `framework-schema:generate` and `ui-styling-tokens:generate` declare their outputs. Staging tests pass 31/31.
- Still blocked on machine and fleet infrastructure: the resident graph slice hangs, the `repo cargo` ingress stalled, and a published root vanished (`OriginalAdmissionClosed`). The fallback `repo tools exec cargo -- build` waits for the capacity governor while other agents' builds hold the memory.

### 4c. Verification

- Binary built natively (fallback, because the ingress and graph were blocked): `.cache/mcp-fallback-bin/semio-os-mcp`. The warden killed `semio_os_mcp_plugin_runtime` (2711 MB) and `semio_os_mcp` (3819 MB) until their carving budgets were declared from those measurements (2980 MB and 4200 MB, within the 10% law; unit-ceiling tests 11 pass). Carving both crates is the real follow-up.
- `semio-mcp-journey.py` (exact `.mcp.json` command, `SEMIO_OS_MCP_BIN` override) passes every step:
  - initialize at 0.4 s, warm discovery, native catalog announced at 10 s
  - 28 tools, 5 prompts, 5 resources
  - context_resolve, capabilities_search, artifact_create, artifact_snapshot, artifact_validate
  - clean EOF exit
- `artifact_create` keeps `initial` as the first journal edit (genesis pack `null`). This is by design and is now documented and tested (`artifact-create-initial-content.md`).
- The repo's own `runOsMcpClientJourney` passed 15 of 16 steps. The typed `s.note.note` steps need the note component model, which was blocked by low disk (`mcp-client-journey-results.md`).
- Cold path without the override:
  - Fixed: the dangling published root that poisoned the process owner (`published-root-dangling-record.md`).
  - Fixed: the cold path now serves as soon as a published executable and the catalog exist, even if later stages hang.
  - Remaining: ingress publication defects (`cold-ingress-publication.md`).
- My own 48 GB fallback target dir was removed after copying the binary, which freed disk for the other builds.

### 5. Zero-touch closure laws (fixed)

- About 870 findings: workspace package specifiers in roughly 30 modules, an undeclared static seam, and an undeclared playground seam. All were rewritten or declared (`zero-touch-closure-fixes.md`, `zero-touch-specifier-rewrite.ts`). `zero-touch-findings.ts` now prints no findings.
- The real-checkout test still exceeds its 120 s budget under machine load (128 s at load avg ~10).

### 6. Cold checkout state

- No `semio-os-mcp` binary, no `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`, no `.🧬semio/🔗space/os-mcp` on this machine at start.
