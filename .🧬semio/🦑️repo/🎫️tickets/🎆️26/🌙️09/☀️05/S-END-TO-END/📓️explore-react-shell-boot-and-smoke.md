# Explore: `s` React OS shell — boot, catalog, opening, and smoke-test tooling

Generated: 2026-09-05 (Sonnet read-only explorer). Paths relative to repo root.

## Top 5 concrete blockers

1. **The one Playwright spec that iterates every plugin (`.storybook/os-plugins.spec.ts`) is broken at module resolution.** It and `.storybook/framework/os/index.tsx:10` (`OsBootHost`) import `🤖️generated/🟦️plugins.ts`, which does not exist. The generator (`🔌️plugin/📇️registry/📜️script.ts:1859`) only writes `🧩️plugins.ts` and prunes anything else (`:1876-1877`). Verified live: `bun -e 'import("./.storybook/framework/os/index.tsx")'` → `Cannot find module '…/🟦️plugins.ts'`. The whole `framework/os` Storybook scope cannot build.
2. **No automated test opens every app inside the unified `s` host shell.** The host-mode e2e (`verify e2e` / `runStudioE2eVerify`, `🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2569-2655`) spawns exactly one app (`draw`). `.storybook/s-end-to-end.spec.ts` only proves boot-to-ready plus palette/context menu.
3. **`parity sweep` boots every playground variant in its own single-plugin filtered dev server, not as a spawned sub-app inside `s`.** The `s` row only proves the landing page.
4. **`os-plugins.spec.ts`, even if fixed, asserts only a boot-outcome beacon** (ready/error/artifact-missing), not rendering.
5. **A primary/host plugin failing is fatal for `s`** (`ShellHost/🟦️.tsx:2633-2643`, `noPluginsLoaded`), while other plugins failing is swallowed into per-plugin `"failed"/"crashed"` status and console-logged (`:2672-2673`). A smoke runner must treat the two classes differently.

## 1. Boot sequence of `dev s` (react)

Entry: `.claude/launch.json` `s-react` → `bun ./📜️script.ts dev s` → `DevScript.run` (`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:1827-1976`).

- `publishShardWorker()` (`:1834`) always runs first (writes `🔌️plugin-modules/🧵️shard/🟨️shard-worker.js`, even under `SKIP_PLUGIN_BUILD=1`).
- `filterPlugin = variantSegment ?? SEMIO_PLUGIN ?? PLAYGROUND_APP_KIND ?? DEFAULT_HOST_VARIANT` (`:1853`); `DEFAULT_HOST_VARIANT = "s"` (`🤖️generated/🎮️playgrounds.ts:86`).
- Cross-process plugin-build lease (`:1868-1894`) so two `dev s` processes don't both build; follower takes over if the holder never reports ready or leaves the cache empty (`pluginBuildOutputsPresent`, `:1785-1792`).
- React + no skip (`streamPluginBuilds`, `:1864`): before Vite only `ensurePluginRegistry(filterPlugin)` (fast, no cargo) and `buildEngineWasm` run (`:1898-1900`); the ~60-crate build (`buildPluginsStreaming`) runs **after** Vite listens (`:1967-1973`).
- Vite env: `SEMIO_PLUGIN`, `SEMIO_RENDERER=react`, `VITE_SEMIO_PLUGIN` (`:1953-1966`). `⚙️vite.config.ts:23` reads the same filter at config-eval time and imports the regenerated catalog statically.
- Lease holder then runs `buildPluginsStreaming(filterPlugin)` and `watchPluginRebuilds(targets)` (`:1968-1972`).

`🔌️plugin-modules` and `/plugin-modules/watch` SSE:
- `semioBackboneVitePlugin` (`:295-375`) — file/folder document IO and its own `/watch` SSE (unrelated to plugins).
- `semioPluginHotSwapVitePlugin` (`:416-458`) — `PLUGIN_SOURCE_WATCH_PATH = ${MODULE_PLUGIN_ROUTE}/watch` (`🔌️plugin/📇️registry/📦️deployment/🟦️.ts:21` → `/🔌️plugin-modules/watch`). On connect sends one `snapshot` (`scanBuiltPluginModules()`, every dir with a completed `.core*.wasm`, `:388-403`), then `built` whenever `♻️hot-swap.json` changes (debounced 200 ms, `:424-438`). Keepalive every 15 s (`:267-278`).
- Kernel consumer: `createDevPluginSource(registry, watchUrl)` (`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:2751-2778`).

Install into the shell (`ShellHost/🟦️.tsx`, under `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/`):
- Primary/host boot effect (`:2635-2644`) installs only `primaryPluginId` (`hostConfig.pluginId`, `:1816`).
- Streaming install effect (`:2660-2698`) subscribes to `pluginSource` and queues `installPlugin`/`reloadPlugin` through a bounded pool (`pluginInstallConcurrency()`).
- `installPlugin` (`:1888-1932`): `pluginSource.moduleUrl` → `loadPluginModuleResilient` → upsert `loadedPlugins`; primary establishes the session (`establishPrimarySession`).

Failure semantics: primary fails → `SET_ERROR ui.common.noPluginsLoaded` (fatal, `:2633-2644`, banner `data-semio-os-shell-error`, `:6983`). Non-primary fails → `console.error("[os-shell] plugin install/reload failed")` (`:2672-2673`), status `"failed"`/`"crashed"` in `pluginStatusById`/`pluginSupervisorById` (`:1900-1902`); shell keeps running.

## 2. Home/Studio host-app resolution — the 09-03 alias defect is FIXED

`🏛️ShellHost/🧬️contracts/🪪️host-bootstrap/🟦️.tsx:22-46` `resolveRequiredHostApps(apps, aliases)` resolves aliases against the live manifest (direct id match, else `role === "editor"` and `dialect.artifactKind` last segment equals the alias), throws typed errors on absent/ambiguous/same app; memoized per manifest (`HOST_APP_RESOLUTION_CACHE`). Wired at `ShellHost/🟦️.tsx:1202-1222`; on failure dispatches `SET_SESSION null` + `SET_ERROR`. All consumers read resolved `hostApp?.id`/`landingApp?.id` (`:1213-1214`, `3460-3527`, `4410`, `5523`, `6269`, `6525-6604`, `6623-6657`, `6989`). Unit tests in `🎯️targets/⚛️react/⚡️quick.test.ts:33-43`. `s` host config is `{ pluginId: "s", landingAppId: "home", hostAppId: "studio" }` (`🤖️generated/🧩️plugins.ts:42,71`). **Do not re-dispatch this.**

## 3. How an app is opened

Routes: `parseShellRoute` (`ShellHelpers/🟦️.tsx:983-986`) → `landing | space{spaceId, instanceId?} | notFound`. Host branch (`ShellHost/🟦️.tsx:3448-3527`): `/spaces/{id}/studio` forces resolved `hostAppId`; bare `/spaces/{id}` opens landing; `/spaces/{id}/instances/{id}` deep links (`:3396`); notFound → `ShellRouteNotFoundPage` (`:6960-6961`).

`os.open-artifact` relay: `resolveArtifactOpeningRelay(actionId, args, router, preferences)` (`🧰️framework/🛍️products/💻️os/🟦️.ts:66-118`); explicit `pluginId+appId` must match `router.entriesFor(dialect, role)` or is rejected `opening.app-mismatch`; else `resolveOpeningApp`. Wired at `ShellHost/🟦️.tsx:4725`, called from `:3302`. Fixture-conformance test `🎯️targets/⚛️react/🚪️opening.test.ts:11-23`.

`AppRouter`: TS type from `@semio-tech/framework`; Rust authority `🔌️plugin/🖥️host/🦀️.rs:8621-8983` (`AppRouter::build`/`surfaces_for`).

User-facing opening: command palette "Spawn <App>" items (`ShellHost/🟦️.tsx:6715-6723`, `id: "spawn.${pluginId}"`, action `spawnApp`); catalogue panel tab `s-play-catalogue` (`ShellHelpers/🟦️.tsx:1035`, `buildSpacePanelState`). `spawnApp` handled at `:3896-3911`; windows tracked in `panel.spawnedApps`.

DOM ids / beacons:
- `#s-presence-peers` (`ShellHost/🟦️.tsx:7159`; wgpu mirror `🖱️ui/🧱️elements/👥️PresenceBar/🎯️targets/🧊️wgpu/🦀️.rs:113-213`).
- `data-semio-os-shell-error` (`:6983`), `data-semio-portal-layer` (`:1147`), `data-shell-id` (`:1144`).
- Readiness beacon: `document.documentElement.dataset.semioOsReady === pluginId` / `semioOsError` / `semioOsNotFound` (`.storybook/s-end-to-end.spec.ts:19`, region `🔖️ReadinessBeacon` in ShellHost), mirrored as `data-shell-ready` (`spec:71`).
- Window ids: `childElementId("framework.window", kind.id | instance.id)` (`:6826,6867`).
- `[data-slot='app-name']` navbar; palette `[role='dialog'] [data-slot='command-input']`, items `[data-slot="command-item"]` (`📜️script.ts:2549-2558`).

## 4. Existing automated evidence tooling

| Command / file | What it does | Coverage |
|---|---|---|
| `bun ./📜️script.ts test [quick|long|exhaustive]` (`TestScript`, `📜️script.ts:2476-2481`, `🧪️tests/🟦️.ts:29-31`) | In-source unit tests of dev tooling helpers only. No browser boot. | none |
| `🎯️targets/⚛️react/🔬️index.test.ts` (7004 lines), `⚡️quick.test.ts`, `🚪️opening.test.ts`, `📇️directory-home-bootstrap.test.tsx` | Renderer host unit tests with fake `PluginWasmHandle`s. | fixed unit scenarios |
| `bun ./📜️script.ts verify` (`VerifyScript`, `:3344-3366`) | cargo lib tests per plugin, react vitest, `runStudioE2eVerify`, capability lint. | one browser app |
| `verify e2e` → `runStudioE2eVerify` (`:2569-2655`) | Real Playwright + `dev s`: Home lists seeded studio, `Meta+n` new studio, 3 fixed windows, spawns **`draw` only** (`:2536-2567`), undo, palettes, Home nav; zero page errors. | 1 app |
| `verify collab` → `runCollabE2eVerify` (`:2658-3350`) | Two-user hub+shell Playwright collaboration proof. | collaboration axis |
| `parity smoke|triage|probe|verify|sweep` (`:4494-4624`, registered `:5485-5498`) | `sweep` boots all 61 `playgroundCatalog` rows standalone (react + wgpu trunk), triages boot, DOM/pixel/behavior diff. | 61 variants, isolated, not inside `s` |
| `.storybook/os-plugins.spec.ts` (+ `plugins.stories.tsx`, `OsBootHost`) | Intended per-plugin boot-outcome beacon over `PLUGIN_BUILD_TARGETS`. **Broken import `🟦️plugins.ts`.** | all 60, non-functional |
| `.storybook/s-end-to-end.spec.ts` | Boots `s` story to `data-shell-ready`, structural landmarks, palette open/close, context menu, zero errors. | `s` only |
| `.storybook/framework-hosts-wasm.spec.ts` / `-no-wasm.spec.ts` | Renderer host elements (NodeGraph, TextEditor, Canvas2dHost, World3dHost…). | host elements |

Net gap: nothing working proves that every app `s` hosts opens and renders from within one `s` session.

## 5. `dev <plugin>` playground vs host-mode `s`

`resolvePluginHostConfig(catalog, "s")` matches `catalog.hosts` (`🤖️generated/🧩️plugins.ts:42`) → `hostMode = true` (`kernel/🟦️.ts:2910`, `ShellHost/🟦️.tsx:1181`). `expandPluginRegistry` (`kernel/🟦️.ts:301-324`): host mode returns the entire catalog (all 60 crates); single-plugin mode computes plugin + `consumes`/`contributes` + transitive `dependencies` closure (`:304-322`). `resolvePlaygroundBoot` orders by dependency (`:2918-2932`). Vite serves the whole `plugin-modules/` dir for `s` (`📜️script.ts:1840-1842`). Only the primary establishes a session (`ShellHost/🟦️.tsx:1910`, `pluginShouldEstablishSession`); every other plugin waits for `spawnApp` → `createApp`.
