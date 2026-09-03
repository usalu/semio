# P5 Integrated Release-Gate and Launch Topology Audit

**Scope.** Read-only source audit on 2026-09-03. This is the launch/release proof topology after the secure-session, authoritative-open-plan, catalog/CAS, P2-C, native, and GIS/MCP seams land. No builds or tests were run for this audit. “Existing” below means source-present, not runtime-confirmed.

## Decision and first deterministic blocker

**Do not add an integrated green target yet.** The registered hub launch is deterministically unable to start: `.vscode/launch.json:4342-4361` supplies development mode and a loopback bind, but `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2121-2124` hard-codes both `IdentityAssertionVerifier` and `LocalBootstrapTransport` to `None` before applying the fail-closed check. Development explicitly requires a protected local bootstrap adapter (`📦️bin.rs:515-520`). Thus every compound containing `🛠️dev🗄️os-hub` is blocked before admin, React, MCP, or native code can be evaluated.

This is correct fail-closed behavior, but it makes the prior static-token launch topology stale. It must not be bypassed by reviving an arbitrary-email session mint endpoint or an environment bearer token.

## Current inventory

| Surface | Registered entry point | What it actually starts/evidences | Release-gate finding |
|---|---|---|---|
| Hub | `os-hub:dev`; `.vscode/launch.json:4342-4361` | Hub script builds the admin SPA before `cargo run` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:5-11,41-48`). Router has directory, admin, blob, document and WS routes (`📦️bin.rs:1982-2010`). | Fails at auth startup as above. No unprivileged `/readyz`; old harness treats protected `/admin/api/overview` as readiness. |
| Admin SPA | `os-hub-admin:dev`, Vite at hub URL (`.vscode/launch.json:4364-4380`) | Separate dev server only; hub dev already builds a static SPA. | Fixed 8790/8787, no ordered readiness. Existing SPA/admin API must first consume secure sessions and its event commands; it is not a gate driver. |
| React users | `@semio-tech/framework-os-dev:dev`; two-user registrations (`.vscode/launch.json:2502-2547`) | Starts per-user Vite shells with fixed ports and workspace-resident data dirs. It passes `S_USER=userN@semio.dev`, `S_HUB_URL`, and local paths. | `S_USER` is an identity hint, not an authenticated session. Fixed ports 6072/6073 and `.🧬semio/🔗space/s-user*` collide with humans/other gates and retain cross-run state. A hub open plan must replace client-chosen plugin/app/surface authority. |
| WGPU wasm/native | wasm user registrations at `:2550-2595`; native registrations at `:2597+` invoke the Rust script directly | Native launch has no hub/session/actor/data configuration. WGPU script inherits environment for Trunk (`…/wgpu/📜️script.ts:31-39`) and may install `trunk` with `cargo install` (`:61-75`). | Native starts are manual visual launches, not an observable authenticated scenario. Automatic tool installation/network access is not zero-touch release evidence. The current ProgramBridge/P2-C attach gap remains a prerequisite. |
| MCP | root launch calls `bun ./📜️script.ts dev mcp {stdio|http} os` (`.vscode/launch.json:4383-4402`) | Root router runs `cargo run -p semio-framework-os-mcp`; HTTP merely defaults to 6300 and passes arbitrary extra args (`📜️script.ts:656-665`). The MCP project exposes test targets only (`…/mcp/📋️project.json:7-48`). | It neither starts nor discovers a secure hub session by default; a React+MCP compound omits the hub (`.vscode/launch.json:7572-7580`). Local CLI principal/scopes cannot be treated as hub authorization. |
| Plugin registry/catalog | `@semio-tech/plugin-registry:{generate,check-generated,check}` (`…/plugin/📇️registry/📋️project.json:4-45`) | The generator is authoritative for generated playgrounds and `.vscode/launch.json` (`📜️script.ts:2-11`). OS dev has catalog build machinery and a non-cached old collaboration target (`…/os/…/dev/📋️project.json:173-184`). | Generator equality is not a deployment catalog, trusted descriptor, package-byte binding, or CAS availability check. It must feed a sealed catalog receipt before runtime launch. |
| GIS/map | root GIS launches call generic `bun ./📜️script.ts dev gis …`; TS project test runs only `console.log("[DEBUG] gis ts ok")` (`✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/📜️script.ts:4-7`). Rust GIS has bounded cargo test/describe targets. | Current TypeScript GIS “test” is zero assertion; local map playgrounds do not prove MCP inference, hub authorization, typed apply/undo, or collaborator observation. |
| Existing real-hub tests | `os-hub-ts:test` | The real-server test is entirely skipped unless `HUB_E2E=1` (`🌎️hub/📦️packages/🟦️typescript/📜️script.ts:1-21`; `🧪️index.test.ts:459-470`). It uses `e2e-admin`, raw actor identities, and mints email sessions after restart (`🧪️index.test.ts:579-620`). | Default test success is not hub E2E evidence; opt-in scenario captures the obsolete auth/actor model and cannot certify the session redesign. |
| Existing browser collaboration runner | `@semio-tech/framework-os-dev:collab-e2e` | Own hub, two React servers, two browser contexts, 8 fixed steps and best-effort teardown (`…/dev/📜️script.ts:3213-3251`). | Static `e2e-admin` and email users (`:2681-2693`), obsolete protected-admin readiness (`:2722-2740`), old ticket log directory (`:2698-2702`), port-pool TOCTOU scan, and bearer/body/WS frame logging (`:3305-3321`) make this diagnostic-only. It also excludes revocation, lag rebootstrap, MCP, map inference/apply/undo and native. |
| PostgreSQL/Neo4j | No registered launch compound or compose topology found | Hub can select FS/SQLite/Postgres/Neo4j storage (`📦️bin.rs:2040-2069`) and independently select SQLite/Postgres/Neo4j directory (`:2073-2113`). | Default is FS document store + SQLite directory, not an all-SQLite pair. Postgres/Neo require connection variables and the existing hub TS harness documents that `--all-features` is red because DB driver deps are unwired (`🌎️hub/📦️packages/🟦️typescript/📜️script.ts:13-21`). They are unavailable as release-gate backends. |

Two further topology hazards apply across the table:

1. `@semio-tech/framework-os-dev` injects macOS-only `DEVELOPER_DIR`/`SDKROOT` into every target (`…/dev/📋️project.json:12-24` and repeated target blocks). That is not a portable launch contract for Linux, Windows, or a devcontainer.
2. Nx correctly disables cache for `dev` and the old `collab-e2e`, but normal test defaults can be cached (`nx.json:38-59`); a process-owning release target must explicitly use `cache: false` and cannot reuse a cached test as runtime proof. `nx.json:85-95` also registers the same custom library plugin twice.

## Why existing compounds cannot be renamed as a release gate

The only compounds are React+MCP without a hub, React+hub, and two React users+hub (`.vscode/launch.json:7572-7599`). They start concurrently rather than await an authenticated ready state, have no per-run ownership/cleanup receipt, and share fixed ports and persistent directories. They do not include the admin SPA, native process, catalog authority, artifact CAS, or GIS. `stopAll` only stops debugger configurations; it is not evidence that browser children, MCP subprocesses, or a cargo process tree were terminated.

The old runners prove useful pieces, not the requested journey. In particular, the current browser scenario’s eight steps stop at simple edit/checkpoint/admin-connections/restart (`…/dev/📜️script.ts:2685-2694`); it cannot establish secure revocation, a forced P2-C rebootstrap, an authoritative open plan, or an approved GIS mutation.

## Required release contracts before orchestration

The gate must consume, rather than manufacture, these schema-first artifacts:

1. **`HubReadinessV1`** — a loopback-only, non-secret `GET /readyz` response with run nonce, protocol/schema version, selected storage/directory backend, secure-session/bootstrap mode, trusted catalog bundle digest, catalog profile, CAS capability/max object and chunk bounds, and open-plan/inference route availability. It must return non-ready for missing/partial authority. It cannot expose bearer values, user emails, private locators, raw package bytes, or admin subjects.
2. **`ReleaseBootstrapV1`** — a parent-to-hub protected local IPC/bootstrap contract that creates three isolated test sessions (administrator, user A, user B) from fixture credential material. Only the runner owns its one-time local endpoint/file; no HTTP route, generic email, `S_USER`, or static admin token mints a session. The gate injects session capabilities into each client’s private process-local/profile directory and verifies their expiry/revocation.
3. **`OpenPlanV1`** — authenticated actor/session/space/document request and immutable, server-derived descriptor/package/schema/app/surface capability response. Client plugin/app/schema/actor selections are inputs for telemetry or requested intent only. Include descriptor and package-byte digests, read/write/inference/apply capabilities, plan revision/frontier, expiry, and revalidation rules.
4. **`ReleaseCatalogReceiptV1`** — the independently validated trusted catalog digest, descriptor hashes, package bytes/byte hashes, codec availability and P2-D chunk-manifest CAS limits. The runtime fixture may be bounded to `s`, writer and GIS only after a separate full-catalog check has certified every declared row; it must never point at incidental `target/` residue.
5. **`ReleaseTraceV1`** — redacted, correlation-ID-only lifecycle events: readiness, plan issuance/revalidation, connection, progress monotonicity, cancellation acknowledgement, checkpoint, rebootstrap, session revoke/kick, inference lifecycle, approved apply, undo, and shutdown. A test failure may retain an explicitly requested evidence directory; default output is a temp directory removed after clean completion.

These contracts make test pass/fail attributable without leaking capabilities or document payloads. They also keep public `(pack,spr)` restore separate from private locator and session data.

## Smallest bounded topology after prerequisites

Put the orchestration in the **existing** `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`; add one non-cached project target there, for example `release-gate` → `bun ./📜️script.ts release gate`. Do not create a new runner file and do not make root `test` imply it. Reuse its process/readiness/budget helpers but replace the old token/port/log mechanics.

### A. `release prerequisite` (no services)

Run serially and fail closed:

1. registry generate/check-generated/check;
2. catalog-completion and descriptor/hash/codec receipt verification for all rows;
3. trusted-catalog authority load plus P2-D chunk-CAS publication, retention and corruption fixtures; and
4. OpenPlanV1, session, MCP/inference and native bridge contract fixtures.

This stage records **blocked**, not success, for unavailable packages, untracked WASM, the 64 MiB pair/496 KiB legacy blob ceiling, missing codec, or unavailable artifact authority. It must not build selected plugins opportunistically and then call that a complete catalog.

### B. `release sqlite` (mandatory zero-touch integration)

Use an independently named `mkdtemp` root and run nonce. Allocate loopback ports through one gate-owned allocator and retry only a bounded spawn race; never use 8787, 6072, 6073, 6300, the 7400-7498 pool, or workspace data directories. Start **one already-built hub binary directly** with explicit loopback development mode, per-run `OS_HUB_DATA`, FS/SQLite defaults recorded in readiness, sealed catalog/CAS configuration, and the protected bootstrap endpoint. Wait only for `HubReadinessV1` within a fixed boot budget.

Then, in dependency order:

1. bootstrap admin/A/B; prove an unauthenticated request, expired capability, cross-space session, share capability, and client-supplied actor are rejected;
2. start the static admin app (or test it through the hub static asset) after readiness; create space/member/invite/document through event commands, not raw CRUD; verify EN/DE labels and keyboard/semantic actions;
3. start two isolated React profiles/browser contexts with injected sessions, request the server open plan, and verify document-wide roster (surface is non-authoritative peer telemetry), concurrent edit, checkpoint, short network interruption/reconnect, forced lag/rebootstrap, and restart against the same run directory;
4. use a separate MCP process with a user-A session to enumerate workspace/catalog metadata and invoke GIS inference. Capture bounded progress, issue cancellation for one job, then run a second job to an approved typed mutation; have user B observe the result, undo it, and observe audit/event lineage;
5. revoke A’s durable session and separately kick one active connection. Assert both routes/WS/MCP revalidate immediately, while a kick alone does not revoke a credential; verify share revocation and cross-space isolation; and
6. assert all trace invariants, destroy processes in reverse dependency order, await exit, remove temporary private state, and fail if any child survives its bounded grace period.

Every stage has a total deadline plus per-readiness/action deadline; progress sequence numbers must be monotonic and cancellation terminal. The driver must redact `Authorization`, query credentials, IPC material, capability strings, document payloads and private locators—unlike the current runner’s raw logging.

### C. `release native` (required desktop evidence, separately executable)

Run after the SQLite scenario from a supported desktop host. Start the native WGPU executable with the same explicit plan/session/profile contract, not `S_USER`; drive a supported control/automation receipt to open, restore, checkpoint, reconnect/rebootstrap and observe the GIS mutation. There is no such native automation/control surface today, so absence is **blocked**, never skipped/pass. A devcontainer/headless host should report `native-environment-unavailable` as a non-green required-platform result, rather than pretend a browser WGPU pass proves native OS parity.

### D. `release backend-parity` (optional external live lanes)

Keep SQLite/FS document + SQLite directory as the mandatory zero-touch lane. PostgreSQL and Neo4j are explicitly opt-in probes, started only from user-supplied connection settings (and, once supplied, distinct databases/namespaces/run nonce). Their readiness must confirm both the DB and directory backend, clean schema ownership, and teardown. Missing Docker, connection variables, feature wiring, or an external service yields `not-run/external-prerequisite`, never a green parity result. It does not weaken the required SQLite release result, and it cannot be advertised as full multi-backend release coverage until the existing all-feature/driver gap is fixed.

## Ordered implementation packet

| Order | Bounded packet | Exact primary files | Exit evidence |
|---:|---|---|---|
| 0 | Secure-session executable bootstrap plus non-secret readiness; replace obsolete `OS_HUB_ADMIN_TOKEN`, arbitrary email mint and client actor ownership in launch/test harnesses. | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, `🌎️hub/🔐️auth/**`, `.vscode/launch.json`, `🌎️hub/📦️packages/🟦️typescript/🟦️.ts`, `🧪️index.test.ts` | Fresh process reaches readiness only through protected bootstrap; public/non-loopback and spoof cases fail. |
| 1 | Complete verified catalog loader/byte association, native codecs and P2-D chunk-manifest CAS. Close the 64 MiB pair versus 496 KiB store ceiling before runtime claims large artifact support. | hub artifact-authority/DB/store seams; plugin registry/check scripts | ReleaseCatalogReceiptV1 accepts all declared rows and rejects a tampered descriptor, byte, chunk, reference/race. |
| 2 | Land authoritative OpenPlanV1 and P2-C server/client revalidation/rebootstrap wiring. | hub router/state/WS, directory document projection; React ShellHost; native ProgramBridge/store | Plan follows session+scope, not client identity; stale/revoked/cross-space plan cannot restore/read/write. |
| 3 | Bring admin to secure session/event command semantics and repair admin-to-directory actor mapping, locale/a11y and live projection controls. | `🌎️hub/🔨️modules/🛡️admin/**`, admin routes | Bilingual admin performs creation/revoke/kick via events; fails closed without session. |
| 4 | Implement authenticated MCP metadata/pair access and hub `ArtifactInferenceRouter`; bind job input/revision, progress/cancel, typed approval, audit and undo. | MCP Rust/TS gateway, hub inference routes, GIS service/plugin | A cannot query B; stale/cancelled output cannot apply; B sees approved apply+undo only. |
| 5 | Complete native lifecycle/ProgramBridge attachment and an inspectable native automation receipt. | WGPU native script/ProgramBridge/Shell launch seams | Native process proves the same plan/reconnect/rebootstrap, not merely a window opens. |
| 6 | Add `release-gate` target and new VS Code compounds **only after 0–5**. Move/remodel the old runner rather than wrapping it. | existing OS-dev `📜️script.ts` + `📋️project.json`, `.vscode/launch.json`, existing neutral fixtures | One owned SQLite run emits redacted receipt, cleans up, and has no false skipped stages. |
| 7 | Add explicit optional PostgreSQL/Neo4j launch profiles after their drivers/features and run isolation are real. | hub Cargo/backend config, `.vscode/launch.json` | Each external lane is separately reported and cannot mask SQLite failure. |

## Launch registration design

Register these only when their packets are implemented, preserving the generated launch catalog’s ownership:

* `⚖️gate🌎️hub🧩️release🪶️sqlite` — `bun nx run @semio-tech/framework-os-dev:release-gate -- sqlite`; one terminal-owned orchestrator, not a compound of independent daemons.
* `⚖️gate🌎️hub🧩️release🧊️wgpu-native` — invokes the native sub-gate and fails non-green when native prerequisites are absent.
* `⚖️probe🌎️hub🧩️release🐘️postgres` and `…🌐️neo4j` — opt-in only, visibly labelled external prerequisites; never `stopAll` companions of the SQLite release gate.
* A human dev compound may include hub/admin/React/MCP after it receives an explicit local-bootstrap profile, but it must not call itself a gate and must use distinct, documented local data paths.

Avoid `npx --yes` MCP Inspector (`.vscode/launch.json:4405-4413`) in the gate: it downloads/exposes an external tool and is neither Bun/Nx nor deterministic. Use the repo’s existing MCP client/test dependencies or an independently implemented wire client instead.

## Neutral and independent oracle plan

* Keep language-neutral JSON fixtures for `HubReadinessV1`, `ReleaseBootstrapV1`, `OpenPlanV1`, `ReleaseCatalogReceiptV1`, `ReleaseTraceV1`, plus hostile variants. Validate schemas with an independent JSON Schema implementation (the hub already uses Ajv and Node crypto for neutral authority vectors at `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:51-88`) and reproduce hashes/signatures/manifest roots with the Rust implementation.
* Use two independently persisted browser contexts and one independent MCP wire client; assert only server-observed receipt/event IDs, plan revisions and allowed state transitions—not DOM text or client logs alone. Use a separate WebSocket client to prove actor spoof/revoke/kick/lag behavior.
* Make a genuine native process receipt a distinct oracle. A screenshot, a successful command spawn, a skipped GPU error, or a test that merely logs `[DEBUG] gis ts ok` is insufficient.
* Test crash/restart, partial CAS publish, retention race, invalid descriptor/codec, share/session confusion, bad progress ordering, cancellation-after-completion, stale result apply, cleanup timeout, and backend configuration mismatch. Each must be a deterministic fail or named blocked state.

## Focused commands after the corresponding packets land

These are evidence slices, not commands run by this audit and not substitutes for the final orchestrator:

```sh
bun nx run @semio-tech/plugin-registry:check-generated
bun nx run @semio-tech/plugin-registry:check
bun nx run os-hub-admin:test
bun nx run @semio-tech/framework-renderer-react:test
bun nx run @semio-tech/framework-renderer-wgpu:test-native
bun nx run @semio-tech/framework-os-mcp:test
bun nx run @semio-tech/gis-plugin:test
bun nx run @semio-tech/gis-js:test
HUB_E2E=1 bun nx run os-hub-ts:test
bun nx run @semio-tech/framework-os-dev:collab-e2e
```

The last two are currently historical diagnostics, not expected green release commands: both assume the old auth/session shape, and the first requires a build. The intended final command is instead:

```sh
bun nx run @semio-tech/framework-os-dev:release-gate -- sqlite
```

only after the ordered packets above exist. PostgreSQL/Neo4j need separate user-provided environment/profile commands and must never be silently run by the zero-touch SQLite target.

## Exit criteria

P5 is complete only when a fresh, loopback-only, SQLite zero-touch run proves two isolated authenticated users plus an authenticated administrator and MCP worker through catalog receipt → authoritative open plan → edit/checkpoint → short reconnect/forced rebootstrap → restart → GIS progress/cancel → approved typed mutation → collaborator observation → undo/audit → durable revoke and separate kick; produces redacted bounded evidence; and kills every owned child. Native evidence must be real and non-skipped on each supported desktop platform. External backend coverage remains separately labelled until its feature/driver/runtime prerequisites are executable.

