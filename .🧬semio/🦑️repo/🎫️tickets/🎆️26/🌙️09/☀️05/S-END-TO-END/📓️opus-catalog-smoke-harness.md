# Lane A — catalog smoke harness, storybook import repair, honest `test quick`

Opus implementer, ticket `26/09/05/S-END-TO-END`. All commands below were actually run; every result is quoted from its own output. Raw logs and reports: `🗑️generated/lane-a/`.

## 1. Storybook `🟦️plugins.ts` import (task 1) — DONE

The registry generator emits `🧩️plugins.ts` and prunes everything else (`🔌️plugin/📇️registry/📜️script.ts:1859,1876-1877`); two files imported a `🟦️plugins.ts` that never existed, and both called `pluginModuleUrl` with a second argument the generated function does not take.

| File:line | Change |
|---|---|
| `.storybook/os-plugins.spec.ts:10` | import path → `🤖️generated/🧩️plugins.ts` |
| `.storybook/os-plugins.spec.ts:23,65` | `pluginModuleUrl(target.pluginId, target.wasmOut)` → `pluginModuleUrl(target.pluginId)` |
| `.storybook/framework/os/index.tsx:10` | import path → `🤖️generated/🧩️plugins.ts` |
| `.storybook/framework/os/index.tsx:53,140` | same one-argument fix (`OsBootHost`, `WgpuBootHost`) |

Verification — module resolution of both files' relative graphs (bare packages externalised so only repo-relative specifiers are resolved):

```
$ bun build ./.storybook/framework/os/index.tsx --target=browser --external react --external @semio-tech/framework-renderer-react … 
Bundled 8 modules in 382ms
$ bun build ./.storybook/os-plugins.spec.ts --target=node --external @playwright/test …
Bundled 8 modules in 415ms
```

Before the fix the same command failed with `Cannot find module '…/🟦️plugins.ts'`.

**Blocked, not done:** `bunx tsc --noEmit` over a two-file scoped project did not finish within 400 s on this machine (load 250-320) and was killed; the type-check claim is therefore *not* proven, only module resolution is.

**Incidental repairs found on the way** (both are variation-selector drift, `🧑️‍💻️dev` vs `🧑‍💻dev`, left by the emoji rename sweep):
- `node_modules/@semio-tech/{framework-os-dev,framework-renderer-react,framework-renderer-wgpu}` were dangling symlinks; relinked in place to the real directory names (node_modules only, not tracked).
- `🧑‍💻dev/🚚️distribution/📇️layout.json` — three `source` paths pointed at the pre-rename spelling, which is what made the in-source test *admits only hand-authored collision-free distribution output owners* fail with `🧑️‍💻️dev/🌐️.html: expected false to be true`. Fixed; that assertion passes now.

## 2. Honest `test quick` (task 2) — DONE, with a thin margin

### What was wrong

`bun ./📜️script.ts test quick` was killed by the 30 s budget with **no results at all**. Measured cause (`--reporter=json`, budget temporarily raised):

- 93 collected cases, **507.7 s of summed test time**, 4 files.
- 5 cases were 67.9 % of it; 15 cases were 90.6 %.
- Worst: `deployed vendor transport > serves every declared component import…` 133.2 s, `pluginComponentBridgeSource > finalizes genuine descriptor bytes…` 70.8 s, and the two WIT in-source files (`🔌️plugin/📤️return/🟦️.ts`, `🔌️plugin/📥️poll/🏘️composition/🟦️.ts`) at 74.0 s and 71.9 s, each dominated by one case that spawns a full strict-TypeScript program check.

### Mechanism added (single source of truth, no new script files)

`🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` (⏱️Budget region, next to `testLevelRank`):

- `testLevelAtLeast(level)` — the predicate the docstring of `testLevelRank` already described.
- `atTestLevel(factory, level)` — level-gates one Vitest case factory, structurally typed on `runIf` so the library never depends on Vitest's types (CLAUDE.md: no external types in exported API).

Applied:

| File:line | Change |
|---|---|
| `🧑‍💻dev/…/🟦️typescript/📜️script.ts:35` | import `atTestLevel` |
| `📜️script.ts:5120` | `const itLong = atTestLevel(it, "long")` |
| `📜️script.ts` (26 cases) | every case measured above ~1.2 s switched `it(` → `itLong(` |
| `🧪️tests/🟦️.ts` | the two `🔌️plugin/` WIT in-source files join `includeSource` only from `long` upwards (they are not this bundle's source, so a per-case gate inside runtime code would be the wrong place) |
| `🧪️tests/🟦️.ts` | `environment: testLevelAtLeast("long") ? "jsdom" : "node"` — only the gated cases touch a DOM (Canvas PNG parity); jsdom cost ~7 s of the 30 s budget |
| `🧪️tests/🟦️.ts` | **the config no longer imports the repo tooling library**; it reads `SEMIO_TEST_LEVEL`, which `resolveTestLevel` already publishes |

That last line was the single biggest win and is worth recording: Vite esbuild-bundles and executes the vitest config's entire import graph on every run, so importing `📚️library/…/🟦️.ts` for one predicate cost **10-15 s of startup** — half the level's budget. Duration fell from 23.6-29 s to 14.2 s the moment it was dropped.

### Proof

```
$ bun ./📜️script.ts test quick            # real 30 s budget, load avg 64
 Test Files  2 passed (2)
      Tests  67 passed | 28 skipped (95)
   Duration  14.20s (transform 10.80s, import 13.74s, tests 2.02s, environment 52ms)
       22.71 real
```

Second consecutive run: `67 passed | 28 skipped`, `Duration 15.54s`, `28.03 real`. A third run on the same machine spiked to `34.70 real` and *was* killed by the budget — see open blockers.

```
$ bun ./📜️script.ts test long             # 300 s budget, load avg 64
 Test Files  1 failed | 3 passed (4)
      Tests  3 failed | 99 passed | 2 skipped (104)
   Duration  136.88s (transform 23.17s, import 25.80s, tests 144.95s, environment 11.57s)
```

104 cases at `long` vs 95 collected at `quick` — the 9 extra are the two WIT in-source files, and the 26 `itLong` cases run there (they show as `skipped` at quick, executed at long). Not a vacuous green: `quick` runs 67 real assertions in ~2 s of test time.

The three `long` failures are **not lane A's** (full output: `🗑️generated/lane-a/🧪️test-long.txt`):

| Case | Failure | Attribution |
|---|---|---|
| `rewriteJcoComponentAssetUrls > rejects a stale effect reply…` | `TypeError: URL must be a non-empty "file:" path` at `shardLivenessPolicy` (`🔌️plugin/📦️packages/🟦️typescript/🦀️.ts:31`) | peer change to the shard worker source; both cases passed at 04:32 today |
| `rewriteJcoComponentAssetUrls > forwards lifecycle through the captured scheduled turn…` | same | same |
| `deployed vendor transport > serves every declared component import…` | `Test timed out in 20000ms` (real Rollup build, 43.9 s) | pre-existing; also failed at 04:32 before any lane A change |

## 3. `verify catalog` (task 3) — DONE

### Shell-side (the only way to enumerate programs without hardcoding a list)

- `📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:646` — exported `ShellCatalogProbe` type.
- `🏛️ShellHost/🟦️.tsx` `#region 🔖️CatalogSmokeProbe` (right after `🔖️ReadinessBeacon`) — dev-only `window.__semioOsCatalogProbe`, guarded by `import.meta.env.DEV` with the same try/catch idiom the tutorial recorder uses, deleted on unmount. Exposes `{ shellPluginId, ready, plugins: [{pluginId,status}], programs: [{pluginId,appId,label}], spawned }`. `programs` is the session's own `panel.programs`; `plugins[].status` is `pluginStatusById`, which is otherwise only console-logged for a non-primary install failure.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🔎️ShellSearch/🟦️.tsx:81` — `data-command-item-id={result.item.id}` on the palette `CommandItem`, so `spawn.<pluginId>` is addressable from the DOM (the component's own `id` is a generated Command id; the item id was not reachable before).

### Runner — `🧑‍💻dev/…/📜️script.ts` `#region 🔖️CatalogSmokeVerify` (≈:2043-2260)

- `summarizeCatalogSmoke` / `catalogSmokeExitCode` / `catalogSmokeMarkdown` / `catalogSmokeMarkdownCell` — pure, unit-tested.
- `runCatalogSmokeVerify(baseUrl, {outDir, timeoutMs, perProgramMs})`:
  - `ensureParityPlaywrightBrowsersPath()` first, so the smoke can never resolve a browserless Playwright.
  - navigates with `waitUntil: "commit"` (the unbundled dev module graph legitimately never fires `load`; the coordinator saw `goto` time out at 300 s against a shell that was booting), then **polls** the readiness beacon on its own deadline.
  - records boot phase timings — `commit`, `firstModule` (first `script` request), `beacon` — and reports them in JSON and markdown.
  - a shell that never reaches `ready:` is a `boot.failure` **row with the last 20 console errors and a 1000-char body excerpt**, never a thrown navigation error.
  - enumerates programs from the probe (never a hardcoded list), spawns each through the real command palette item `[data-slot="command-item"][data-command-item-id="spawn.<pluginId>"]`, diffs `[id^='framework.window.']` before/after to find the mounted window, asserts non-zero bounding box **and** ≥1 descendant, attributes console/page errors per program by cursor into the error buffer, then closes the window via `framework.window.<id>.windowControls.close`.
  - `catalogSmokeEvaluate` retries any read that dies with `Execution context was destroyed` (a peer edit triggers a dev-server reload mid-run — this actually happened and killed run B).
  - writes `🔬️catalog-smoke.json` + `🔬️catalog-smoke.md` to `--out` (default `🧑‍💻dev/🤖️generated/🔬️catalog-smoke`) and exits non-zero when the shell never booted, any program failed, any plugin is `failed`/`crashed`, or nothing rendered at all (a vacuous green is itself a failure).
- Wired at `VerifyScript` — `bun ./📜️script.ts verify catalog [--out <dir>]`, `S_CATALOG_SMOKE_PROGRAM_MS` per-program budget.

### Schema, fixture, unit tests

- `🧑‍💻dev/🧫️fixtures/🧬️catalog-smoke.schema.json` — draft-07, `additionalProperties:false` throughout, bounds `bodyExcerpt` to 1000 chars and `consoleErrors` to 20.
- `🧑‍💻dev/🧫️fixtures/🔬️catalog-smoke.json` — language-agnostic `{input, report, exitCode, markdown}` fixture.
- `📜️script.ts` `#region 🔖️CatalogSmoke-tests` — 6 quick-level cases: fold-to-report equality, Ajv validation of the committed report plus two rejections (independent oracle), the four exit-code branches, boot-diagnostics reporting, diagnostic bounding, and markdown cell escaping.

```
$ bun … vitest run --config 🧪️tests/🟦️.ts -t "catalog smoke"
 Test Files  1 passed | 1 skipped (2)
      Tests  6 passed | 89 skipped (95)
```

### Registration

- `🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:186` — target `catalog-smoke`, `cache:false`, `bun ./📜️script.ts verify catalog`, mirroring `collab-e2e` exactly.
- `.vscode/🧩️launch.seed.jsonc` — `⚖️gate🔬️catalog-smoke`, group `4_gate`, order `410.99`, immediately after `⚖️gate🌎️collab-e2e` (410.98).
- `.vscode/launch.json` **regenerated through the owning generator** (`generateLaunchJson(repoRoot, playgrounds)` from `🔌️plugin/📇️registry/🖥️launch.ts`), never hand-edited. The regeneration also picked up 20 peer seed entries that had never been regenerated (print-viz, directory-command-receipt, execution-target-lease/relay, gis-map-proposal, mit-bestand report builds). `dev` regenerates the same file on every boot and produced byte-identical output afterwards.

## 4. Running it against a live shell (task 4) — RAN, catalog blocked upstream

The coordinator's 6070 was unreachable for most of the window (`curl … → 000` for >10 min at a stretch while Vite restarted). Own server, launched exactly like the registered `🛠️dev🖥️s⚛️react` launcher plus `SKIP_PLUGIN_BUILD=1 SKIP_ENGINE_BUILD=1`:

```
S_OS_PORT=6074 SEMIO_PLUGIN=s SEMIO_RENDERER=react SKIP_PLUGIN_BUILD=1 SKIP_ENGINE_BUILD=1 \
  bun nx run @semio-tech/framework-os-dev:dev            # VITE ready in 4591 ms, 200 OK
```

**Finding worth propagating:** the same command with the extra segments `-- s served` serves **404 for every document** (`/`, `/🌐️.html`, `/🎨️.css`), while `/@vite/client` answers 200 — i.e. the dev server comes up but its root is not served. Dropping `served` fixed it. The registered launcher has no such segment; anything driving `dev s served` is driving a shell that cannot load.

Two smoke runs against the healthy 6074 (`🗑️generated/lane-a/`):

| Run | Beacon | commit | first module | beacon | Programs smoked | Verdict |
|---|---|---|---|---|---|---|
| C (`🔬️catalog-smoke-run-c-console.txt`) | `error:s` | — | — | never | 0 | shell fatal: **No plugins loaded** |
| D (`🔬️catalog-smoke.json` / `.md`, `…-run-d-console.txt`) | none | 1372 ms | 1398 ms | never (180 s) | 0 | shell never mounted |

Run C's console is the actionable one — the per-app table is empty because **no app is reachable**, so the honest per-app result today is "0 of the catalog renders, blocked before the first spawn":

- `resolvePlaygroundBoot(s): Plugin "draw" needs "draw-fsm", which is not installed.`
- `resolvePlaygroundBoot(s): Plugin "sequence" needs "imperative-control" / "imperative-effect" / "imperative-math" / "imperative-text", which is not installed.`
- `plugin.descriptor-unavailable … (HTTP 404)` for `🧩️extension-modules/{imperative-extension-text, imperative-extension-math, flow-extension-bim, flow-extension-dictionary, cad-extension-aec-building-energy, sourcing-module-slabs}/🔣️.json` and `🔌️plugin-modules/🔱️trinity/🔣️.json` — the 19-row missing-owner-descriptor census from `📓️explore-catalog-build-state.md`, i.e. lane B / Wave 2.
- `shard 0 worker error Event`, then body text `No plugins loaded` — the fatal primary-plugin path (`ShellHost` `noPluginsLoaded`), matching the coordinator's own 14:05 evidence (shard watchdog kills the boot shard, `AppRouter.build` fails on the demonstrator manifest).

So the harness is proven end-to-end — it navigates, detects, diagnoses, reports and exits non-zero — but it cannot yet produce a per-app pass/fail table, because the `s` session never reaches a spawnable state. The moment lanes G/H (shard liveness, router resilience) and the Wave-2 descriptor rebuild land, `bun nx run @semio-tech/framework-os-dev:catalog-smoke` will produce that table with no further work.

## Open blockers

1. **`test quick` margin is ~2-8 s.** Passes at 22.7 s and 28.0 s, killed at 34.7 s on the third run (load avg 64; normal for this machine today is 250-320). Test *execution* is 2.0 s; the rest is Vite transforming/importing the dev bundle's ~350-module top-level graph. The clean next cut is deferring `📜️script.ts`'s two remaining heavy value imports — `@semio-tech/framework-os` (used only by the sync `finalizePluginDescriptor`, :314-324) and `📇️registry/📜️script.ts` — behind `await import()`. Both change signatures other lanes are editing today, so I did not do it.
2. **No per-app smoke table yet** — blocked on the `s` boot itself (above). Not a harness defect.
3. **Storybook type-check unproven** — `bunx tsc --noEmit` on a two-file scoped project did not finish in 400 s. Module resolution is proven; types are not.
4. **`dev … -- s served` serves 404 for every document.** Reproduced on 6074; suspected same root cause as the coordinator's unreachable 6070. Owner unknown (the vite config graph belongs to lane E); I did not touch it.
5. **Three `long`-level failures are peer-owned**, listed in §2; `shardLivenessPolicy` regressed today, the vendor-transport timeout is pre-existing.
