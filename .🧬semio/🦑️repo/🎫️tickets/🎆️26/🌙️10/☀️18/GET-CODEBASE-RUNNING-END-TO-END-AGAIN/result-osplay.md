# result-osplay: os dev vitest, Play tests, catalog audit runtime

Executor `osplay`, 2026-10-18, foreground, no cargo, no `package.json`/`bun.lock` edit, nothing under the frozen storage/revision paths. Runner: copy of `vit-tools/runvt.py` (`osplay-tools/runvt.py` + `run.sh`: full logs instead of the last 80 kB, `OSPLAY_ARGS` for `-t` filters, `OSPLAY_RUNTIME`). Policy runtime stays `node` (`repositoryVitestPolicyV1`). The Playwright figures use a scratch `PLAYWRIGHT_BROWSERS_PATH` that symlinks chromium 1234 as 1248 (harness only, nothing committed).

## Result

| suite | before | after | what is left |
|---|---|---|---|
| os dev (`framework-os-dev`) | 29 failed / 271 tests (5 files) | 5 failed / 271 | contribution (frozen storage), katex (deps), vite config graph x3 (real growth) |
| Play (`semio-tech-play`) | 9 failed tests, 5 files | 56 tests green, 1 file blocked | `playpages` needs `http-cache-semantics` (deps) |
| catalog (`s-dev-composition`) | 3 failed, audit 78 s vs 30 s | 14 of 16 green; integration test 32 s total (audit + independent oracle) | 2 session tests need the launcher's native caller capabilities; 13 descriptor pairs need the `component` build |
| regression guard | | actor 104/104, kernel 91/91, styling 82/82, plugin-registry 90/90, 8 bun law tests (gitlink, exclusion, owner index) green | |

## 1. os dev: runtime decision and fixes

Decision per package: the suites stay on Node (the policy runtime; running them on Bun failed differently, `import(blob:)` inside vite-node breaks the build-input loader, 23 failures). Code under test is made runtime-neutral where cheap, and the cases that need a Bun-hosted function run it in a Bun child (`underBun` in the staging test kit), the way the suite already spawns Node children.

Code fixes (evidence in parentheses):
- `⚖️parity/🔬️probe`: `Bun.hash` -> `node:crypto` sha256 (`Bun is not defined`).
- `🔌️vite-plugins/🔖️source-freshness`: an atomic rename delivered **three** HMR updates (the sibling sweep plus two rename events each emitted `change`; measured with event traces, fixture law says exactly one). The named path now emits `change` only when the freshness registry says the file moved (`movedFile`). Three hot-update cases green.
- `🧭️routing/🟦️.ts` (`Script` constructor parameter property) and `🚪️io/🪶️sqlite-snapshot`: strip-only Node could not load the registry's lazy `require` chain (`SyntaxError ... parameter property`), which failed both catalog session cases and the host-activation case.
- Discovery modules imported `ephemeralMap`/`ephemeralBox`/`blake3Hex` from the `@semio-tech/framework` barrel (a layering sweep in HEAD 694); the barrel drags `import.meta.vitest` top-level-await modules in, so `require()` of discovery failed (`ERR_REQUIRE_ASYNC_MODULE`). Re-pointed to the declared subpaths `@semio-tech/framework/transient` and `/hash` (4 files).
- Strip-only fallout reached through that chain, fixed by the repo convention: parameter properties -> explicit fields (`osplay-tools/fix-pp.mjs`, TypeScript API, keeps `super()` first) and `with { type: "json" }` on json imports. 31 files, list at the end.

Stale expectations fixed with evidence:
- Bridge source: `fuel: BigInt(turn.fuel), deadlineUs: turn.deadlineUs, maxEffects/maxPatchBytes/maxFrames: budget.*` (the generated bridge no longer has `deadlineMs`/`maxFrames: 8`).
- Six node:vm bridge cases rewritten against the retained original-turn contract: the generated `poll` holds events and command pages as producer input (`original-turn.canonical-input-held`), requires an original admission, clock and return, and one host-shim module per actor/generation. New test-kit helpers `generatedBridge` (real bridge + real host shim + a guest answering `channelVersion`/`admitActorContext`/`closeActorContext`/`poll` with a retained receipt, loaded by the platform loader because vite-node strips the activation query) and `retainedGuestSource`. Worker-dispatcher cases use the existing `✂️worker-vm-source` helpers with a fake api that carries the retained original source.
- `rewritePreview2ShimImportSource`: callers pass a prefix ending in `/`; fixture `🪞️vendor.json` prefix had none.
- WASI profile fixture: package `semio-hub-gis` -> `semio-hub-gis-component` (artifact is `semio_hub_gis_component.wasm`, catalog `wasmOut`).
- `assertExtensionOutputsFresh` cases need `directoryName` on the rebuilding targets.
- jco async-result fixture: newer wasm-tools requires `(callback (core func ...))`.
- Distribution layout law: symbolic `@engine/<id>/<facet>` coordinates are not files.
- Vendor transport: extension store takes `options.streams` now; `pluginOutRoot` export is gone (uses `builtPluginModulesRootIn`, skips with a stated reason until a prototype activation exists).
- Watch policy fixture: the "unwatched generated file" row had lost its `🤖️generated` segment in the 09-02 rename (history: `…/📇️registry/🤖️generated/🎮️playgrounds`); now the S catalog's generated playgrounds file.
- Config laws: anchored regex alias form; `resolve.extensions` guard no longer matches the default export's empty `extensions: []`.
- Time-travel boot: the playground catalog is read from `SEMIO_PLUGIN_CATALOG`; the suite config now supplies the catalog owner's publication output root (or the launcher's value).

Still open in os dev:
- `🧩️contribution`: its Node child imports `observedFs` -> native-build storage, which imports `🧬️schema/🔣️.json` without `with { type: "json" }` in about ten **frozen storage** modules (`💾️storage/🌐️platform`, `🍎️image`, `📤️publication/*`, ...). Needs the storage owner.
- `deployed vendor transport > admits only hand-authored ...`: `node_modules/katex` is in no store generation (deps).
- Vite config graph, 3 cases (`🧹️config`): the entry's static closure is 219 modules (law bound 40), 2 denied discovery modules statically (`🚚️distribution` -> discovery `🕸️runtime` -> `📇️registry-catalog-inputs`/`🪧️owner-markers`) and 40 more through the lazy `import("driver-stages")` of the activation plugin (esbuild follows literal dynamic imports). Largest subtrees: `⚡️caching/🌐️vite/🕸️wasm` 124, activation plugin 50, distribution components 26. A real design regression, not a stale expectation; bound deliberately not raised. Tool: `osplay-tools/config-chain.mjs` prints the import chains.
- `local-hub` race cases flake only when the whole suite runs in parallel under load (green alone 3/3).

## 2. Play

Descriptors are committed at `🌎️hub/🧩️compositions/<plugin>/🧩️component/🔣️.json` (written by the plugin's `component` target; the generated registry rows already point there). `pluginDirectories` now means a directory that owns a hub component (excludes `✏️s/🔌️plugins/🧪️tests`), descriptors are read from that path, stdio's component bound is read from the composition's runtime dependencies (the `component-app-assembly` features are gone), `catalog-release` is asserted removed, the dev-mount plugin root is `builtPluginModulesRootIn`, fresh-build fixtures carry the current graph (`@semio-tech/assets:build` prerequisite, five wasm lane jobs, `components:complete`), the launch env includes the build-environment protocol (`BUN_RUNTIME_TRANSPILER_CACHE_PATH`, `NODE_DISABLE_COMPILE_CACHE`) read from its schema, and the root release driver caller moved to `📚️library/🎛️commands/🚀️release`.

## 3. Catalog audit

Profile (bun CPU profile, `osplay-tools/profile-*.ts`): the audit was one full filesystem walk of the repo. 115,181 tracked directories and about 235k entries, each read with readdir plus lstat; time was `readdirSync` 42 %, `lstatSync` 16 %, `resolve`/`relative`/`join` 22 %, `pathIsExcluded` (two pattern rewrites per exclusion per call) 8 %. The machine is syscall-latency bound (CPU 24 % at load 30 to 40): 16 overlapping readers walk the same tree 5.3x faster than one.

Fixes (algorithmic):
1. `registryCatalogOwnerIndex` (new, `📇️registry-catalog-inputs`): one `git ls-files --cached` listing yields the tracked directories and the owner ancestors. `scanRepo` skips a child directory only when the index knows it and no owner lies beneath; a directory the index has never seen (new, untracked) is always walked, and the default view is the only caller (projected views and the input-witness walk stay full). readdir calls 115k -> 9.4k. Result identical to the full walk: 3,296 packages, equal key by key. Fallback to the full walk when git is unavailable. Residual blind spot: an untracked owner created directly inside an existing tracked directory that has no owner yet; the independent oracle catches it.
2. `pathIsExcluded` / `registryCatalogInputView`: opaque prefixes precomputed per exclusion table, membership by path segment; the kind cache is consulted before admission.
3. `rejectDuplicateJsonObjectNames`: sticky regexp instead of `source.slice(index).match` per scalar, charCode whitespace fast path.
4. The independent oracle `sourceDeploymentOwnersV1` (test) enumerates with `sourceCensus.concurrency` (schema + fixture field, 16) overlapping reads: 26.6 s -> 16 s under load.
Measured (load 30 to 43): scan 65 to 100 s -> 9.6 to 15.5 s; audit 78 to 105 s -> 24 to 28 s; vitest integration case 32 s with both enumerations (limit per enumeration 30 s). Language-agnostic test for the owner index added to `🧾️registry-catalog-gitlink-boundary` (fixture rows + `picomatch` oracle, both transpilers, git-unavailable case).

13 descriptor-pair issues: not fixable without the component build. `animate block dag fem mathematical reasoning remodel sequence sourcing space vcs wfc writer`: commit f856dff0813 rewrote their `🔣️.json` on 10-15 (new `framework.io.text` carrier `ioEntries`, re-sorted; `space` gained `topicContributions`) while `🛂️.descriptor.semio` is still from 10-13. The JSON is `packValueToExactJson(pack)`; a JSON to pack inverse is deliberately absent ("JSON never learns to carry an integer carrier"), so the pair must be re-emitted by `bun repo run semio-hub-<id>-component:component` (cargo + wasm). `osplay-tools/diff-pair.ts <dir>...` prints the first differences per pair. The integration case does not assert on `descriptor-invalid`, so it is green.

Other catalog items: the two session cases need `readNativeOwnerCapabilities` data from the launcher (harness envelope carries `{}`); `host-activation` is green now (strip-only fix above).

## Changed files (all outside the freeze)

- Play: `🏢️semio-tech/🎡️play/🧪️tests/{🧪️playpanecoverage,🧪️playdevmounts,🧪️playactivation,🧪️playfreshbuild}/🟦️.ts`, `…/🧪️playfreshbuild/🧫️fixtures/🔣️.json`.
- Discovery perf: `📚️library/🔍️discovery/{📇️registry-catalog-inputs,🔭️repository-scan,🧲️registry-catalog-projection,🔐️taxonomy-authority}/🟦️.ts`, barrel->subpath in `{✅️owned-file-approvals,🧭️workspace-taxonomy-locator,🪧️owner-markers,🕸️runtime/🔎️verification}/🟦️.ts`; `🔌️plugin/📇️registry/✅️catalog-verification/🛡️guards/🟦️.ts`; `✏️s/🧑‍💻dev/🧩️catalog/{🧪️tests/🧩️scope/🟦️.ts,🧪️tests/🧫️fixtures/🔣️.json,🧬️schema/🔣️.json}`; `📚️library/🧪️tests/{🧾️registry-catalog-gitlink-boundary/🟦️.ts,🧫️fixtures/🧾️registry-catalog-gitlink-boundary/🔣️.json}`.
- os dev code: `🧑‍💻dev/⚖️parity/🔬️probe`, `🧑‍💻dev/🔌️vite-plugins/🔖️source-freshness`.
- os dev tests/fixtures: `🧑‍💻dev/🧪️tests/{🎚️config,🧹️config}/🟦️.ts`, `🧪️ticket-owned-browser-host-staging/{🟦️,🧰️test-kit,🛂️routes-and-bridge,🧬️jco-rewrites,🚚️vendor-transport,🗃️handles-and-output}/🟦️.ts`, fixtures `🪞️vendor.json`, `👁️watch-policy.json`, `🦀️wasm-profile-policy/🧬️v1/🔣️.json`, `🔌️plugin/🧪️tests/🧫️fixtures/⏳️async-results.json`.
- Strip-only neutral edits (parameter property -> field, json import attribute), 31 files under `🧰️framework/`: `📦️packages/🟦️typescript/🛂️manifest/🧬️schema`; `🔨️modules/` `🌱️value/{⚠️refusal, 📦️packages/.../🧬️schema, 🧬️schema/🌳️intrinsic}`, `🎠️kernel/📇️descriptor/{📥️text,📦️package}`, `🎭️actor/🎟️retained-turn/{🟦️, 🏛️bootstrap}`, `🎭️actor/📮️shard-client/{🟦️ and 9 children}`, `🏃️process/🧭️routing`, `🖱️ui/🪟️viewport/{2d,3d schema, 2 tests}`, `🚪️io/{🔤️base64, 🧬️schema/⚠️refusal, 🧬️schema/🏛️ownership tests, 🧬️schema package, 🪶️sqlite-snapshot}`, `🛂️manifest/📥️mutation-input-schema`, `🧬️schema/.../🗿️artifact-reference`, `🛍️products/💻️os/.../🏪️store/🪆️child/🧬️schema`. Some json-attribute rows may overlap `vit-tools/json-attribute-files.txt`.

## Verification

- `OSPLAY_ARGS=... osplay-tools/run.sh <list> <name>` (full logs) for os dev, Play, catalog; before/after numbers above come from `Test Files`/`Tests` lines of those runs (os dev final: 5 failed, 266 passed).
- `bun test` of `🧾️registry-catalog-gitlink-boundary` + `🚧️cargo-discovery-exclusions`: 8 pass (the view/exclusion laws evaluate the edited functions from source).
- `scanRepo` equivalence: `osplay-tools/profile-scan.ts full` printed `packages pruned 3296 full 3296 equal true`; `count-scan.ts`: 9,457 readdir calls.
- Regression runs: actor 104/104, kernel 91/91, ui-styling 82/82, plugin-registry 90/90.
- No `[DEBUG]` left in repo files (temporary instrumentation in the watcher was restored; scripts under `osplay-tools/` keep their `[DEBUG]` prints).

## Open

Freeze/deps: katex, http-cache-semantics, brepjs family, storage json attributes. Build: 13 descriptor pairs. Launcher env: 2 catalog session cases. Design: vite config graph (3 cases), bridge rewrite covers the retained protocol only for the cases listed (the old event-through-poll features are gone by design). Note for the fleet: peers' layering sweeps keep re-introducing barrel imports and parameter properties into the Node-loaded registry chain; a law that `require()`s `🔎️discovery` under strip-only Node would catch it at edit time.
