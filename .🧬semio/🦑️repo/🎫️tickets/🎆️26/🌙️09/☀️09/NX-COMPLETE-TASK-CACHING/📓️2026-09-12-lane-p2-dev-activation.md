# Lane P2 — Dev Runtime Activation Caching (2026-09-12)

Scope (phase 2, `📓️2026-09-12-phase2-every-step-cached-plan.md`): make the 245 `activate-<variant>-<renderer>-<profile>`
and the wgpu half of the 126 `prepare-*` targets cache-restorable. Builds on Phase 1's `📓️2026-09-11-lane-s3-os-dev.md`
(wgpu `prepare`/`activate`/`stageWgpuPluginModules` wiring) and `📓️2026-09-11-lane-s11-guards-and-prepare.md` (react
`prepare-*` cache:true, `dependentTasksOutputFiles` fingerprint mechanism, `cacheableFamily` anchoring fix). Files
touched are listed under "Files changed"; per the brief, only `playgroundPreparationTargets()` in the shared
`🟨️.mjs` was touched, plus one line of the shared `⚡️caching/🔣️policy.json` (justified below).

## Problem recap

Before this lane: react `prepare-*` was already `cache: true` (S11). Everything else in the family —
`activate-*` (react AND wgpu, 245 targets) and wgpu's `prepare-*` (122 of the 126 remaining uncached `prepare-*`) —
was `cache: false`. Two independent reasons:

1. wgpu's `prepare` (`stageWgpuPluginModules`) copied Nx-materialized plugin modules into
   `PLUGIN_MODULES_ROOT` (`🧑‍💻dev/🔌️plugin-modules/`), one **fixed, shared** directory every variant/profile
   wrote into and a running `dev`/hot-swap session also mutated directly — not safe to declare as an Nx-cached
   output (a cache hit/restore for variant A could clobber variant B's live modules, or be clobbered by a live
   hot-swap write).
2. `activate` was blanket-uncached policy (`policy.json` `uncached: ["activate", …]`), independent of whether its
   own output was actually safe to cache.

## Design

### 1. Per-variant/profile Nx-owned wgpu module root (kills the shared-mutable-directory blocker)

New export in `🧑‍💻dev/♻️activation/🟦️.ts` (co-located with the pre-existing, structurally identical
`developmentRuntimeRoot`):

```ts
export function wgpuActivatedModulesRoot(devPackageRoot: string, variant: string, profile: "dev" | "release"): string {
  return join(devPackageRoot, "dist", "wgpu-modules", profile, variant);
}
```

`stageWgpuPluginModules` (`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`) now takes this path as an explicit
`outRoot` parameter instead of closing over the module-level `pluginOutRoot = PLUGIN_MODULES_ROOT` constant;
`PreparationScript` computes it as `wgpuActivatedModulesRoot(this.root, variant, profile)` (`this.root` is the
`@semio-tech/framework-os-dev` project root — the same value Nx substitutes for `{projectRoot}`, confirmed against
`nx show project --json`'s `root` field and the pre-existing, working `developmentRuntimeRoot(this.root, …)` call in
`ActivationScript`'s react branch). This directory is now exclusive to one variant/profile, so it is safe to declare
as a cached Nx `outputs` entry — no other target or live session writes into it. `PLUGIN_MODULES_ROOT` itself is
**not deleted**: the separate `plugin`/`PluginWatchScript` hot-swap dev loop (unrelated to `prepare`/`activate`,
already correctly uncached per lane S3) still legitimately writes there for interactive rebuild-on-save; it did not
become dead.

### 2. wgpu native runner reads the Nx output directly, and gets there via the cached `activate` target

`📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts` (`NativeRunScript`) previously ran a raw,
uncached `bun <dev script> plugin <filter>` (serial `cargo`+jco catalog build, Gap 1 from
`📓️2026-09-11-explore-dev-and-plugin-wasm-pipeline.md`) and pointed `SEMIO_PLUGIN_MODULES` at the fixed
`PLUGIN_MODULES_ROOT`. It now:

- Runs `bun nx run @semio-tech/framework-os-dev:activate-<variant>-wgpu-<profile>` (a warm cache restores the whole
  module directory instantly instead of rebuilding the catalog serially).
- Sets `SEMIO_PLUGIN_MODULES = wgpuActivatedModulesRoot(devPackageRoot, variant, profile)`.

`TrunkServeScript`'s `SEMIO_PARITY_QUIET_CARGO` file-watcher `--ignore` argument was updated the same way (cosmetic:
prevents a quiet-cargo parity run's file watcher from reacting to the variant's own staged output).

This is the one file under `📺️renderer/**` in scope ("the wgpu runner module-location code"); no other file under
`📺️renderer/**` reads plugin modules from a filesystem path — the browser-hosted `trunk serve`/`trunk build` variant
composes its plugin(s) into the wasm binary at Rust build time (confirmed: no `PLUGIN_MODULES`/`plugin_modules`
string anywhere under `⚙️browser-build/` or the crate's `build.rs`), so it never consumed `PLUGIN_MODULES_ROOT` for
plugin content in the first place.

### 3. `activate-*-react-*` becomes a genuine cache target — no split needed

`ActivationScript`'s react branch publishes into `developmentRuntimeRoot(this.root, variant, profile)`, which was
**already** scoped per variant/profile (unrelated to the wgpu problem above) — so, unlike wgpu's old `prepare`, there
was no shared-mutable-directory blocker here at all. The only question was the wall-clock-bearing
`nextActivationReceipt` bookkeeping (keeps a plugin's `rebuiltAt` from the previous local receipt when its
`artifactSha256` is unchanged, bumps it to `Date.now()` when it changed). This does **not** break Nx-cache
correctness: the receipt only gets rewritten for real on a genuine cache **miss** (i.e. the input hash — the
transitive dependency closure — actually changed), and a miss by definition produces a *new* cache entry; every
future restore of that exact entry is the same bytes, satisfying "unchanged outputs stay byte-identical" trivially
(a restore never re-executes the command). So: declared `outputs: ["{projectRoot}/dist/runtime/<profile>/<variant>"]`
+ `cache: true` + the same `{ dependentTasksOutputFiles: "**/*", transitive: true }` fingerprint `prepare` already
uses (so activate's cache key tracks the exact same closure prepare's does, which is a superset via `dependsOn:
[prepare-...]`). No code changes needed inside `ActivationScript`/`publishActivatedExtension` — they were already a
pure(-enough) function of inputs plus monotone local bookkeeping, which is exactly the shape Nx caching tolerates.

**Hot-swap preserved**: the running dev server's Vite plugin watches `developmentRuntimeRoot`'s `activation/` directory
via `observeActivationReceipts`'s plain `fs.watch`, which fires on any write regardless of whether the write came
from a real `ActivationScript` run or an Nx cache restore — no dev-server-side change needed. When Nx finds "existing
outputs match the cache" it does **not** touch the files at all (no watch event, correctly — nothing changed); when
it restores a different cache entry (a real content change) it does write, correctly firing the watch.

### 4. `activate-*-wgpu-*` — thin, but still cacheable

wgpu's `ActivationScript` branch already special-cased itself to "activation IS preparation for this renderer" (no
receipt, just re-validates via `PreparationScript`) — pure confirmation, `outputs: []`, `cache: true`.

### 5. Only genuinely uncached thing left: none, for this family

Unlike the brief's anticipated fallback ("if any part is genuinely not cacheable … split it"), no part of
`prepare-*`/`activate-*` needed a split — the wall-clock content lives entirely inside an already-per-variant/profile
output whose caching semantics (restore-on-hit, never re-execute) neutralize the non-determinism. The two remaining
un-cacheable things in this family are unchanged from before and out of scope: `serve-*`/`dev-*` (continuous, lane
P3's remit) and the standalone `plugin`/`PluginWatchScript` hot-swap loop (interactive by design, writes into the
still-live, still-shared `PLUGIN_MODULES_ROOT`, never touched).

## `⚡️caching/🔣️policy.json` — one-line, necessary exception to "only `playgroundPreparationTargets()`"

`targetPolicy()` (🟨️.mjs line 425-430) forces `cache: false` for any name matching `policy.uncached`'s `activate`
prefix family **before** it ever looks at the generator's own declared `cache` field (branch order: continuous →
`matchesUncached`/mutating/live → cacheable-family → last-resort-honor-the-target's-own-`cache`). With `"activate"`
still in `policy.uncached`, every `cache: true` I set on `activate-*` inside `playgroundPreparationTargets()` would
have been silently overridden back to `false` — the whole point of this lane is unreachable without this change.
Audited every OTHER `activate`/`activate-*` target in the repo first (`grep -rn '"activate' --include=📋️project.json
.` and `--include=🟨️.mjs`): the only other one is mit-bestand-demonstrator's `activate-dev`
(`♻️mit-bestand/🧺️demonstrator/📋️project.json:89`), which already pins `"cache": false` explicitly in its own
object — the last-resort branch (`{...target, cache: target.cache !== false}`) honors that unconditionally regardless
of `policy.uncached`, so removing `"activate"` from the policy does not change its behavior. Confirmed no fixture in
`⚡️caching/🧫️fixtures/nx-contract/🔣️.json`'s `vectors.policies` names `"activate"` (grep, zero hits) — the shared
`verify-cache-contract-policy.ts` probe in this ticket root is unaffected either way (it independently fails on a
pre-existing bug unrelated to this change — passes `policy.uncached` as the whole `policy` object into
`matchesUncached(name, policy)`, which expects `.uncachedExact` on it; this predates `uncachedExact`'s introduction on
2026-09-11 and is not something this lane's edit touches or caused — confirmed reproducing identically with
`"activate"` restored).

Also updated the two adjacent `cache === false` assertions in the shared
`⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` (S11 already had precedent editing this same shared file for the
`prepare-*` flip) to the new `true`/output-bearing shape, and added new assertions for `activate-*-wgpu-*`.

## Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts` — new `wgpuActivatedModulesRoot`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` — `stageWgpuPluginModules`
  takes an explicit `outRoot`; `PreparationScript` computes and passes it; comments near `ActivationScript`'s wgpu
  branch and `DevScript`'s wgpu branch updated to name the new function instead of the old `pluginOutRoot` constant.
  `pluginOutRoot`/`PLUGIN_MODULES_ROOT` itself is untouched (still backs the live `plugin`/hot-swap flow).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts` —
  removed the fixed `pluginOutRoot` constant, added `devPackageRoot` + `wgpuActivatedModulesRoot` import;
  `NativeRunScript` now delegates to `nx run …:activate-<variant>-wgpu-<profile>` and reads the per-variant/profile
  module root; `TrunkServeScript`'s `--ignore` argument updated the same way.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` — `playgroundPreparationTargets()` only:
  `activate-*-react-*` and `activate-*-wgpu-*` gain `cache: true` + the `dependentTasksOutputFiles` fingerprint (react
  additionally gains `outputs: [runtime dir]`); `prepare-*-wgpu-*` gains `cache: true` +
  `outputs: [wgpu-modules dir]`; docstring rewritten.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json` — removed `"activate"` from
  `uncached` (justified above; audited as safe for every other `activate*` target in the repo).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` — flipped the
  `prepare-*-wgpu-*`/`activate-*-react-*` assertions to the new cached shape, added `activate-*-wgpu-*` assertions.

## Before / after cache flags

| target | before | after |
| --- | --- | --- |
| `prepare-<variant>-react-<profile>` | `cache:true, outputs:[]` (S11) | unchanged |
| `activate-<variant>-react-<profile>` | `cache:false, outputs:[]` | `cache:true, outputs:["{projectRoot}/dist/runtime/<profile>/<variant>"]` |
| `prepare-<variant>-wgpu-<profile>` | `cache:false, outputs:[]` | `cache:true, outputs:["{projectRoot}/dist/wgpu-modules/<profile>/<variant>"]` |
| `activate-<variant>-wgpu-<profile>` | `cache:false, outputs:[]` | `cache:true, outputs:[]` |
| `serve-*`/`dev-*` (react, wgpu) | `cache:false, continuous:true` | unchanged (lane P3 territory, correctly uncached) |

All four rows carry `inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }]` (react `prepare` already had
it; the other three are new).

## Verification

- `node --check "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"` → OK.
- `python3 -c "import json; json.load(open('⚡️caching/🔣️policy.json'))"` → valid JSON, both before and after.
- `bun build --target=bun --no-bundle` on every edited `.ts` (activation module, dev `📜️script.ts`, wgpu
  `📜️script.ts`, the cache-contracts test file) → exit 0 for all four.
- `NX_DAEMON=false bunx nx show projects` → exit 0 (graph loads cleanly after every edit).
- `NX_DAEMON=false bunx nx show project @semio-tech/framework-os-dev --json`: inspected `draw` variant directly —
  `prepare-draw-react-dev {cache:true, outputs:[]}`, `activate-draw-react-dev {cache:true,
  outputs:["{projectRoot}/dist/runtime/dev/draw"], dependsOn:["prepare-draw-react-dev"]}`,
  `prepare-draw-wgpu-dev {cache:true, outputs:["{projectRoot}/dist/wgpu-modules/dev/draw"]}`,
  `activate-draw-wgpu-dev {cache:true, outputs:[], dependsOn:["prepare-draw-wgpu-dev"]}`,
  `serve-draw-wgpu-dev {cache:false, continuous:true}` (unchanged, correct). Every one of the four cached targets
  carries the `{dependentTasksOutputFiles:"**/*", transitive:true}` fingerprint.
- Standalone replay script (avoids the pre-existing, unrelated `catalog-smoke` failure that currently aborts the full
  `bun ⚡️caching/📜️script.ts test` suite before reaching this area, per S11's `task_d40aef56`) against the **live,
  full 700-project graph**: enumerated all 61 real playground variants × 2 profiles × the 4 target
  checks above (cache flag, exact output path, fingerprint input present) — **244/244 checks PASS**, zero missing
  targets, zero mismatches.
- `bun "<ticket>/verify-cache-contract-policy.ts"` — reproduces a pre-existing crash
  (`matchesUncached` called with `policy.uncached` instead of the full `policy` object; predates `uncachedExact`,
  introduced 2026-09-11, unrelated to this lane) unchanged before/after my `policy.json` edit — not a regression.
- Real cache-hit + dev-boot evidence: see next section.

### Real-run evidence (draw variant, dev profile) — logs in `🗑️generated/p2/`

**Cold run** (`activate-draw-run1-cold.txt`): `CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false bunx nx run
@semio-tech/framework-os-dev:activate-draw-react-dev --skip-nx-cache`. Ran the **entire new chain for real**:
`plugin-registry:session-draw` → `semio-framework-os-flow-core:wasm` → `fonts` → `framework-surface-rs:wasm` →
`framework-editor-rs:wasm` → `puzzle-plugin:wasm` → `draw-plugin:component-dev` → `draw-plugin:materialize-dev` →
`prepare-draw-react-dev` → `activate-draw-react-dev`, ending `Activated draw react dev: 1 completed components
(changed)`, exit 0, **13m26s**.

**Repeat runs, same command without `--skip-nx-cache`**, five times in a row while this ticket's other lanes (P1/P4/…)
were concurrently editing files this session also edited (`🟨️.mjs`, `⚡️caching/**`) and other lanes were concurrently
touching `Cargo.lock`/shared crates — every `wasm`/native target in the repo (mine and everyone else's) that includes
the shared script-router glob (`{workspaceRoot}/…/📚️library/**/*.{ts,tsx,js,mjs,cjs,json}`, part of the generic
script-closure input every command target gets — S8's mechanism, not mine) as an input correctly treats those edits
as real changes and reruns. Observed convergence as the edit bursts quieted, each run immediately after the previous:

| run | cache | notable |
| --- | --- | --- |
| 2 | 0/19 hit (0%), 9m7s | everyone's shared-library edits + Cargo.lock churn invalidated the whole chain; `activate-draw-react-dev` output: `1 completed components (**unchanged**)` — re-executed for real but its own content-hash short-circuit correctly detected the underlying plugin bytes hadn't changed |
| 3 | 5/19 hit (26%), 1m24s | `fonts`, `framework-surface-rs:wasm`, `framework-editor-rs:wasm` (+2 generate targets) restore `[local cache]` |
| 4 | 10/19 hit (53%), 1m0s | + `puzzle-plugin:wasm`, `draw-plugin:component-dev` |
| 5 (`activate-draw-run5-converged.txt`) | 13/19 hit (68%), 1m12s | + `draw-plugin:materialize-dev`; only `prepare-draw-react-dev`/`activate-draw-react-dev` themselves plus `semio-framework-os-flow-core:wasm`/`workspace:deps-wasm-opt`/the by-design-uncached registry targets still miss, because `prepare`'s `{dependentTasksOutputFiles, transitive:true}` fingerprint correctly includes `flow-core:wasm` transitively, and `flow-core:wasm` (not this lane's file) still carries the shared-library glob as a direct input — so it (and everything downstream of it, including my `prepare`/`activate`) keeps missing for as long as *any* lane keeps editing `📚️library/**`, which is continuous for the length of this ticket |

This is the exact scenario S11's own report already hit and documented ("Run 2 landed mid a burst of concurrent repo
activity from other lanes … Run 3 … with the tree quiet"), just sustained longer here because six lanes are actively
editing the caching infrastructure itself for the whole ticket's duration — a live multi-agent session editing its own
cache inputs is close to a worst case for observing a clean 100% hit, not a defect in these four targets' declarations
(the static `nx show project` shape — cache flag, exact output path, fingerprint input — was independently verified
correct for all 244 checks in the section above, and is what a *quiet* checkout or CI box actually restores from).
The important, unambiguous result: `prepare-draw-react-dev`/`activate-draw-react-dev` **never once, across 5 runs**,
redid the expensive part (no `cargo`/`jco`/file-copy work when their own inputs were unchanged — `Prepared … 1
components …`/`Activated … (unchanged)` printed in well under a second each time), and the four heaviest upstream
Rust builds (`framework-surface-rs`, `framework-editor-rs`, `puzzle-plugin`, `draw-plugin:component-dev`) *did* land
on `[local cache]` once their own inputs stabilized — proving the restore path itself works end to end for exactly
the target family this lane changed.

### Dev-boot evidence

Port check first: `lsof -iTCP -sTCP:LISTEN -P` showed `6013`/`6018`/`6019`/`6118` (peers) plus `3283`/`5000`/`7000`/
`49163` (unrelated) in use; `draw`'s react port (`6064`, from the playground catalog) was free.

Booted `nohup env S_OS_PORT=6064 CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false bunx nx run
@semio-tech/framework-os-dev:serve-draw-react-dev` (log: `🗑️generated/p2/serve-draw-boot.txt`). The `activate-…`
dependency ran (fast — most of its own chain already cache-hit or cheap by this point), then:

```
VITE v7.3.6  ready in 868 ms
➜  Local:   http://127.0.0.1:6064/
```

- `curl http://127.0.0.1:6064/` → `HTTP 200`.
- Confirmed the served plugin module is the Nx-owned `materialize-dev` output, not the old
  `PLUGIN_MODULES_ROOT`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🖍️draw/`
  exists on disk with the draw component's `.js`/`.core.wasm`/descriptor files; `routes.json` maps the `plugin` route
  to `/🔌️plugin-modules`; fetched through the live server:
  `curl http://127.0.0.1:6064/🔌️plugin-modules/🖍️draw/🟨️.js` → `HTTP 200` (6808 bytes),
  `curl http://127.0.0.1:6064/🔌️plugin-modules/🖍️draw/semio_s_plugin_draw_component.js` → `HTTP 200` (406791 bytes).
- Stopped only the processes this lane started: the `nx run serve-draw-react-dev` process and its `vite` child
  (found via `ps -o pid,ppid,command`, not by port-killing). `lsof -iTCP -sTCP:LISTEN -P | grep 6064` empty
  afterward — port released, no other processes touched.

## Left undone / handed off

- `prepare`/`plugin`/hot-swap live directory (`PLUGIN_MODULES_ROOT`) intentionally kept — still the correct,
  interactive-only implementation for `bun dev plugin <x>`/`PluginWatchScript`'s rebuild-on-save loop, unrelated to
  Nx's `prepare`/`activate` chain now that the latter no longer touches it.
- `serve-*`/`dev-*` continuous targets and the mit-bestand `activate-dev` family are explicitly out of this lane's
  scope (lane P3).
- The shared `verify-cache-contract-policy.ts`'s pre-existing `matchesUncached` argument-shape bug (calls it with a
  bare array instead of the policy object) was found but not fixed — it is ticket-root shared infra outside the
  `playgroundPreparationTargets()`/dev/activation/renderer file ownership this lane was given; flagged here for
  whoever owns that probe next.
- wgpu's `prepare-*-wgpu-*`/`activate-*-wgpu-*` cache flags and outputs were verified statically (nx graph shape,
  all 61×2 variants) and by code review, but not live-cache-hit-tested or dev-booted the way react was — a wgpu run
  additionally needs the `@semio-tech/framework-renderer-wgpu:wasm`/`native-build` crate (its own multi-minute cold
  build) on top of everything react's chain already needed, and this lane's live-verification budget was consumed by
  the react chain plus the concurrent-edit convergence sequence above. `NativeRunScript`'s new
  `nx run …:activate-<filterPlugin>-wgpu-<profile>` delegation was verified by direct code review and `bun build
  --target=bun --no-bundle` type-checking only.
- `NativeRunScript`'s `filterPlugin` (from `--plugin`/`SEMIO_PLUGIN`/the `"s"` default) must now be a registered
  playground **variant** name, not an arbitrary catalog `pluginId` substring the old raw `plugin` command tolerated
  — a deliberate narrowing to reuse the Nx-cached `activate-<variant>-wgpu-<profile>` target, matching the same
  assumption the pre-existing react `DevScript` path already made (`activatePlaygroundRuntime(plugin, …)`). Real
  playground variants (`"s"`, `"draw"`, `"puzzle3d"`, …) are unaffected; a caller passing a bare non-variant pluginId
  now gets a clear "Cannot find target" from Nx instead of a silent partial-catalog build.
