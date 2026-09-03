# OS-Style Frontend Audit

Date: 2026-09-03  
Scope: read-only audit of the `s` React shell, its browser-development host, plugin/artifact registry, surfaces, collaboration-facing UI and test gates. No production files were changed.

## Decision Summary

The repository contains a substantial React OS shell and a generated plugin/playground catalog, but it is not presently an end-to-end usable collaborative frontend. The registry integrity gate is also red: after approximately four minutes it reported thousands of taxonomy violations. The current tree has active taxonomy/deduplication work and broad in-flight renames, so the report separates those normalization failures from independently evidenced runtime design holes. The highest dependency-critical browser gaps are explicit in the committed collaboration harness:

1. Creating/opening a hub artifact does not bind a real `documentId`, so the browser creates an unbound ephemeral editor rather than a shared hub document.
2. React does not render the `#s-presence-peers` roster required by the two-user scenario, despite lower-layer presence wiring.
3. The OS-dev `test-quick` gate currently fails while attempting to load its `📜️script.ts` as a browser module; the quick gate therefore cannot certify the shell.

Do these three packets before attempting a catalog-wide browser proof. The hub/admin and AI-map lanes can progress independently, but the frontend cannot demonstrate collaborative editing until the document-opening contract is corrected.

## What Exists Today

### Launch and boot path

The intended browser path is:

`VS Code launch entry or bun nx` → root `📜️script.ts` `DevScript` → `@semio-tech/framework-os-dev:dev` → Vite → `🧑️‍💻️dev/🟦️.ts` → `bootFrameworkOs()` → `FrameworkOsShell`/`ShellHost` → generated registry → WASM plugin actors.

Evidence:

- Root `📜️script.ts:463-502` resolves `dev s` and catalog variants; `served` specifically selects `SEMIO_RENDERER=react` and `SKIP_PLUGIN_BUILD=1`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🟦️.ts:32-65` chooses React by default and dynamically imports the renderer before calling `bootFrameworkOs`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🟦️.tsx:1104-1119` bootstraps root, appearance, locks/defaults and renders `FrameworkOsShell`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts` exposes the browser root (`/🟦️.ts`), emits `VITE_S_HUB_URL`, `VITE_S_USER`, and `VITE_S_DATA_DIR`, hosts plugin and extension folders, and includes static production copying for `/plugin-modules`.

Useful executable targets:

| Intent | Exact target |
| --- | --- |
| Studio React development | `bun nx run @semio-tech/framework-os-dev:dev` with `S_OS_PORT=6070`, `SEMIO_PLUGIN=s`, `SEMIO_RENDERER=react` |
| Catalog-routed local development | `bun ./📜️script.ts dev s` (full build), or `bun ./📜️script.ts dev s served` (React against existing plugin modules) |
| Two React clients plus hub | VS Code compound `🧭️compound🖥️s👥️users🗄️os-hub` in `.vscode/launch.json:7590-7597` |
| Browser collaboration scenario | `bun nx run @semio-tech/framework-os-dev:collab-e2e` |
| React renderer unit suite | `bun nx run @semio-tech/framework-renderer-react:test` |
| Registry integrity | `bun nx run @semio-tech/plugin-registry:check` |

The user-facing VS Code launch records are generated from `.vscode/🧩️launch.seed.jsonc` by the plugin registry. Current `launch.json` contains a single React studio entry at lines 2460-2499 and two hub-backed React users at lines 2502-2547. The two-user entries pass separate `S_DATA_DIR`s and the same hub URL. There are 558 launch configurations, 68 matching generated React entries, and 79 generated placeholders in the seed; launch changes must flow through the registry generator rather than manual edits.

### Shell, rendering, and local-first state

The React renderer is not a placeholder. Its public target is `@semio-tech/framework-renderer-react`, whose entrypoint re-exports `bootFrameworkOs` from `🧱️elements/Shell/🟦️.tsx`. `ShellHost` expands the plugin registry, loads the primary plugin first, concurrently streams remaining plugin/extension entries, creates the chosen app session, and has an explicit hot-swap/reload path. `PluginRuntime/🟦️.tsx` owns a pooled-shard actor design with watchdog/recovery hooks rather than one worker per plugin.

The shell provides a substantial OS chrome surface:

- docked windows/panels, search/command UI, document synchronization, history, conflicts, task-manager, tutorial, map/canvas/graph/text hosts;
- `viewer` and `editor` roles (`resolveBootAppRole`, session role chip and read-only support);
- explicit error boundaries with localized retry alerts;
- local browser storage plus scoped/memory storage for embedded and ephemeral shells;
- local preference persistence for layout, locale, terminology, keybindings, driver, appearance and custom themes;
- theme authoring/import/export via `.theme.dsl`.

This is evidence of implemented infrastructure, not browser acceptance proof for every plugin. The renderer’s own test configuration does not automatically include all co-located component tests: its `includeSource` currently lists `UiDocumentStore`, `Interpreter`, and `PluginRuntime` but excludes files such as `AgentPresence`, `AgentApprovals`, `TaskManager`, `AgentBridge`, `TiledMapHost` and `ShellHelpers` even though their test files exist.

### Plugin registry and artifact/surface model

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts` is the one product-level adapter that constructs `PLUGIN_CATALOG` from generated `🟦️plugins.ts` and `🟦️playgrounds.ts`. The generated rows contain plugin id, crate path, wasm output, role, capabilities, contributions/consumes, dependencies, activation events, extension points and optional hashes. The host configuration currently identifies plugin `s`, landing app `home`, and host app `studio`.

The registry is designed around per-artifact-subset surfaces, not the old generic app directory. `🧰️framework/🛍️products/💻️os/AGENTS.md` defines a surface address as `<kind>@<standard>/<subset>#<role>`. The registry validator walks `🗿️artifacts/<kind>/🏅️standards/<standard>/🪆️subsets/<subset>/👁️viewer|✏️editor` and separately audits schema, codec IO, engine and examples. It deliberately treats taxonomy gaps as warn-only while plugin areas remain `legacy`/`mixed` (`📇️registry/📜️script.ts:2012-2081`), so a green registry gate is not equivalent to complete artifacts.

The completed `bun nx run @semio-tech/plugin-registry:check` run was red after roughly four minutes and emitted thousands of violations: required schema/IO/engine/example facets absent, unowned Rust `#[path]` leaves, missing plugin command/module roots, and missing surface configuration/presence schema. This is a release-blocking acceptance gate, but not every individual error should become new product work: active taxonomy/dedup tickets and mass tree moves can transiently orphan paths. The execution owner should first classify each diagnostic as (a) a real missing semantic facet, (b) stale registry/index generation, or (c) expected in-flight rename/dedup work, then fix the root taxonomy/configuration before requiring catalog-wide browser mounting. The static directory counts below are inventory evidence only, not proof that the registry check is healthy.

Filesystem snapshot:

- 40 static descriptor files under `✏️s/🔌️plugins`;
- 94 direct artifact-root Rust entries measured by `.../🗿️artifacts/<kind>/🦀️.rs`;
- 144 viewer directories observed during the audit (the matching editor count query did not complete within the 10-second read-only bound, so do not infer parity solely from this count);
- the prior still-open surface migration ticket records 143 owned subsets / 286 surfaces, but this needs a current catalog-check confirmation before treating it as the exact live count.

Codecs exist structurally under each artifact’s `🚪️io` facet, including the `energy` model’s import deserializers for XLSX, TXT, ZIP and JSON. Editors/viewers are predominantly Rust/TypeScript semantic surface declarations; they are rendered by the generic React interpreter/host registry rather than by a separate React component per plugin. This is the intended extensibility seam, but it means every surface needs runtime catalog and browser coverage rather than relying on directory presence.

## Collaboration Frontend: Current Reality

The React shell contains hub identity bootstrap, offline handling, a worker-owned directory socket, and presence interaction projections. `ShellHost` documents the desired behavior: reuse cached identity if the hub is unavailable, show offline rather than block the UI, and open a single directory socket after identity resolution. It also produces peer interactions for domain selection/hover overlays.

However, the committed browser E2E is unusually candid about two blocking holes:

| Dependency order | Observed break | Evidence | Required result |
| --- | --- | --- | --- |
| 1 | Hub-created artifact is not opened as its actual document | `🧑️‍💻️dev/📜️script.ts:2998-3013` says `os.open-artifact` is replayed without `documentId`; `ShellHost` consequently never calls `openDocument` for the hub-bound artifact. | One canonical `ArtifactRef`/document id flows command → shell effect → browser backbone attachment → session load. |
| 2 | A second author cannot see edits because both editors are unbound ephemeral instances | `📜️script.ts:3026-3047` asserts the same failed prerequisite. | Two separate shells attach the same persisted shared document and receive/reconcile its event stream. |
| 3 | React has no visible two-peer roster | `📜️script.ts:3059-3073` asserts `#s-presence-peers` does not exist; its comment says PresenceBar was only wired for wgpu. | React renders a localized, keyboard/screen-reader-accessible roster with exactly the hub presence projection; no duplicate local state. |

The eight-step `runCollabE2eVerify` does have meaningful intended coverage: public space propagation, author membership, shared writer edits, presence, check-in/history, admin connections, and persistence through hub restart. It exits nonzero if any step fails. It is not a passing proof today: the code itself marks the three above as expected upstream gaps. Do not bypass it or weaken its assertions.

## Accessibility, Internationalization, Customization

Implemented foundation:

- The shell supports `en`/`de` detection and persisted or locked locale/terminology (`Shell/🟦️.tsx:233-264`, `1083-1092`; `ShellHost` propagates updates through labels).
- It persists layout, appearance, theme, custom themes, custom UI drivers and keybindings; embedded shells receive scoped storage; ephemeral brands receive in-memory storage.
- `ShellFaultBoundary` uses `role="alert"`; shell status uses `role="status"`/`aria-live`; AgentPresence is covered by a direct accessible-name component test.
- The shared UI toolkit contains semantic controls and keyboard/ARIA tests.

Gaps:

1. The OS policy says there is no default language, but the concrete shell falls back to `en` in several invalid/missing preference paths (`Shell/🟦️.tsx:241-242`, `1090`). Decide whether `en` is a UI fallback versus a violation of product policy, then make the contract explicit and test it.
2. The React renderer target does not include all existing co-located accessibility component suites in its configured `includeSource`; required a11y assertions can silently remain out of the target gate.
3. There is no evidence of catalog-wide browser a11y/i18n verification across every generated viewer/editor, or of a test that a locked preference cannot be changed through the UI.
4. The absent React presence bar is both a collaboration gap and an accessibility gap: peers have no discoverable live-status presentation.

## Test and Gate Audit

### Confirmed broken/vacuous gates

- `bun nx run @semio-tech/framework-os-dev:test-quick` currently fails after 2 passing files / 9 tests: Vitest/Vite prepends `/@vite/client` before `📜️script.ts`’s shebang and Rollup reports a line-one parse error. This was independently observed by the umbrella coordinator.
- Root cause chain: `@semio-tech/framework-os-dev:test-quick` invokes `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` `TestScript` (`2479-2483`); it invokes `runVitest(..., "🧪️tests/🟦️.ts")`; that config has an empty `include` and includes `📜️script.ts` through `includeSource` (`🧪️tests/🟦️.ts:1-23`). The source is also a Bun executable with `#!/usr/bin/env bun`. The resulting browser transform cannot preserve its required first-byte hashbang. Fix the test architecture; do not remove the script shebang merely to satisfy a browser transform.
- `os-hub-admin:test-quick` exits zero with “No test files found”; `os-hub-ts:test-quick` exits zero with one skipped test (umbrella coordinator observation). Treat both as vacuous gates until each executes a non-skipped behavior assertion and reports its count.

### Existing useful coverage

- The React package root suite tests descriptor admission, extension completion ownership, app-session isolation, sync UI, document-measure dispatch and related contracts in `.../react/🧪️index.test.ts`.
- Co-located component tests exist for AgentBridge, AgentPresence, TaskManager, AgentApprovals, ShellHelpers and TiledMapHost.
- Registry test target validates generated launch/catalog contracts; `check` additionally byte-compares generated catalog/launch output and runs descriptor/taxonomy diagnostics.
- The browser E2E code is located in `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts:2486-3356`, with single-user and two-user paths.

- `bun nx run @semio-tech/plugin-registry:check` completed red after approximately four minutes, with thousands of taxonomy diagnostics (missing required schema/IO/engine/example facets; unowned Rust leaves; missing plugin command/module roots; and missing surface config/presence schemas). Treat it as a hard release blocker. Its diagnostic population overlaps active taxonomy/dedup normalization and large renames, however, so classify and eliminate the structural root causes rather than converting every generated orphan into a one-off product implementation packet.

## Test-First Implementation Packets

### P0 — Repair OS-dev test isolation

Owner area: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/`.

1. Write a minimal regression test that loads each test-only implementation module under the actual Vitest config and asserts that the first transformed source is parseable (or, better, tests the extracted pure helper module directly).
2. Move testable pure runner logic out of the executable `📜️script.ts` into an ordinary library module; keep command registration only in `📜️script.ts`. If extraction is not sufficient, configure an SSR/node transform specifically for the runner source. Preserve the hashbang executable.
3. Run `bun nx run @semio-tech/framework-os-dev:test-quick` and require a nonzero collected-test count. Then run the non-quick target.
4. Add an explicit test that `includeSource` covers every in-source test file intended for this target; no silent configuration drift.

### P1 — Canonical hub artifact opening and shared document binding

Owner seam: space plugin effect emission, shared OS command schema, browser `ShellHost` host-effect application, backbone/document transport.

1. Add language-neutral schema vectors for `os.open-artifact` that require space id, artifact id, selected surface id, document id/backbone URI and role.
2. Add a Rust/TypeScript codec parity test plus a browser-shell unit test showing the received effect calls `openDocument` with the exact id and cannot fall back to an ephemeral buffer.
3. Test two browser-side session adapters against the same in-memory/shared transport: first user writes, second receives the persisted revision; verify retry after a transient disconnect.
4. Make collab E2E steps 3 and 4 green without loosening their current assertions.

### P2 — React presence roster

Owner seam: worker directory-presence projection → `ShellHost` selector → new/ported `PresenceBar` React component.

1. Start with snapshots/vectors for 0, 1, 2, reconnecting and removed peers; include color, role, activity/surface and ephemeral interaction fields.
2. Add component tests asserting `#s-presence-peers`, exact peer rows, localized accessible names, `role=status`/live updates, keyboard reachability and roster removal on disconnect.
3. Render from the already authoritative projection only; do not introduce a CRDT or duplicate websocket ownership.
4. Make collaboration E2E step 5 green and ensure user restart rehydrates the roster.

### P3 — Normalize registry taxonomy before browser-matrix proof

Owner seam: artifact taxonomy/dedup tickets + registry generator/validator.

1. Capture the red registry diagnostics as deterministic categorized output, grouped by plugin and surface address.
2. For every group, assign exactly one disposition: required semantic facet to implement, stale generated index/configuration to regenerate, or in-flight rename/dedup ownership to resolve in its owning ticket. Do not mask any class globally.
3. Restore `bun nx run @semio-tech/plugin-registry:check` to a clean exit with zero unresolved diagnostics, and retain a regression fixture for each prior diagnostic class.
4. Only then use generated `PLUGIN_CATALOG` as the authoritative input to browser acceptance; before this point matrix failures cannot distinguish absent frontend behavior from broken catalog topology.

### P4 — Turn catalog claims into browser proof

Owner seam: registry + React renderer + plugin build pipeline.

1. Generate a deterministic matrix from `PLUGIN_CATALOG`: each declared artifact subset must expose viewer/editor entries; each entry must resolve a module and instantiate a manifest session.
2. Use a capability-free smoke artifact per surface class to assert viewer rejects mutation commands and editor accepts the canonical mutation event; compare language-neutral vector output.
3. Run the matrix in bounded shards with progress/cancellation and produce a machine-readable report under the ticket’s generated directory during execution.
4. Gate launch/catalog freshness, module availability, required label translations and a basic accessibility landmark assertion per surface.

### P5 — Make acceptance gates non-vacuous

Add at least one active behavior assertion to `os-hub-admin:test-quick` and `os-hub-ts:test-quick`, fail on zero executed tests, and add a workflow target that sequences registry check → OS-dev quick test → React renderer test → collab E2E. Do not report aggregate “green” until the exact target executions and collected-test counts are recorded.

## Existing Work to Coordinate With

Do not reimplement these architectural layers; integrate with their committed seam and pick up their remaining blockers.

- `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` — **open**. It introduced per-subset viewer/editor surfaces, opening defaults, role-aware sessions and catalog/launch rules. Its record states browser E2E was blocked by plugin WASM compilation and that some authored surfaces remain unmounted. P3 normalization followed by this audit’s P4 catalog-smoke packet is the appropriate continuation.
- `26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` — **closed**. It delivered dependency-ordered loading, transaction/contribution routing and cross-language wire work; reuse its catalog and effect/transaction mechanisms rather than a new app registry.
- `26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS` — existing two-user harness owner. Its in-source assertions identify the exact P1 and P2 gaps, so its scenario is the primary acceptance test.
- `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` — **open**. Owns shard/actor lifecycle, quotas and task manager; the frontend should consume its host abstractions rather than create workers or connection loops per plugin.
- `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY` and `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-5-RESUMABLE-FRAME-TRANSACTION-AND-RENDERER` — **open**. They own UI-patch contract and resumable rendering/scheduling. Keep P1/P2 at the ShellHost/projection seam; do not fork their renderer architecture.
- `26/09/02/COMPLETE-SEMIO-END-TO-END` — current umbrella ticket. Its backend/hub/map execution lanes must share the canonical space/artifact/document identity and same test evidence.

## Handoff Priorities

1. Assign P0 immediately so the OS-dev quick gate becomes trustworthy.
2. Begin P3 taxonomy classification immediately in its existing ownership tickets; it can run alongside P0/P1, but browser-wide evidence must wait for it.
3. Assign P1 next; without it there is no valid shared-editor proof, AI-generated artifact opening will also be unsafe, and P2 can only show presence without collaboration.
4. Assign P2 in parallel with P1 only after agreeing the projection payload contract.
5. When P0–P3 are green, run the full two-user E2E and then perform P4 surface-matrix acceptance.
