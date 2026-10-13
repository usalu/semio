# TypeScript and Web Build Audit

Read-only source audit on 2026-10-13. No compiler, application runtime, or expensive build was run. Findings are source-level evidence, not a runtime success claim. Root and applicable product/repo AGENTS instructions were read.

## Recommended Execution Ownership

Parent owns central build-mode schema/policy and process helpers. Web execution can own application site script/config changes and TypeScript configuration cache partitioning. Preserve Bun commands and Nx orchestration; add no standalone script files.

## Confirmed Fixes

1. **Concurrent TypeScript incremental state shares one root file.** Root `tsconfig.json` enables incremental compilation and sets `node_modules/.cache/tsconfig.tsbuildinfo`. The following extending configs do not override that setting and therefore inherit the same resolved root file: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/tsconfig.json`, `🧰️framework/🔨️modules/🖱️ui/🌐️locale/tsconfig.json`, `🧰️framework/🔨️modules/🖱️ui/🎓️introduction/tsconfig.json`, and `🧰️framework/🔨️modules/◻️2d/🧮️compute/tsconfig.json`. Give each configuration an explicit owned cache location. For simultaneous runs of the same config, use an invocation-specific mutable file with a leased/shared immutable warm-state transfer rather than concurrent writes. A package-wide scan should resolve inherited options before claiming complete coverage; one config contains JSON comments, so bare JSON.parse is insufficient.

2. **Generic JavaScript cache fingerprints include broad root manifests.** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json` javascript.files includes `package.json`, `bun.lock`, `bunfig.toml`, and `tsconfig.json`; `projectInputs` in the sibling `🟨️.mjs` adds them for all generic JS tasks. This causes unrelated dependency and workspace registration changes to invalidate otherwise local tasks. Keep genuinely global compiler/config inputs, but use admitted externalDependencies and local package/compiler closure where possible. Do not remove lock hashing until exact package/tool version admission is established.

3. **Generic inputs expand to entire delivery owner.** `projectInputs` in `🟨️.mjs`, around lines 833-896, expands package roots to their owner and hashes all matching TS/JS/JSON/CSS/HTML source under that owner. Generic cached targets retain default/^default at lines 987-988. Artifact TypeScript build targets already have a narrower artifactSources/artifactCommandSources route; extend the explicit source-input contract to ordinary bundling/typecheck owners rather than introducing another discovery mechanism. Test that sibling production edits do not invalidate an unrelated consumer and admitted imports do invalidate it.

4. **Play fresh build forcibly disables warm cache reuse.** `🏢️semio-tech/🎡️play/🔨️modules/📦️site/🆕️fresh-build/🟦️.ts` uses `--skip-nx-cache`, `--skip-remote-cache`, `NX_SKIP_NX_CACHE=true`, `NX_SKIP_REMOTE_CACHE=true`, and `NX_DAEMON=false`. Preserve an explicit fresh verification option, but normal prototype/release builds should use warm admission. The helper's memory-aware concurrency formula is useful and should consume the central policy instead of remaining another independent scheduler.

5. **Play Vite cache identity is independent of explicit mode.** `playViteCacheDirectory` selects only NX_CACHE_DIRECTORY or a common fallback; `🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts` uses it for serve/build. Partition mutable optimizer state by mode, platform/toolchain/config identity and invocation or protect it with an owner lease. Share validated reusable content separately. This is a contention risk identified from source; no collision was reproduced.

6. **Play site only expresses ship build behavior.** `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts` bundles with SEMIO_BUILD_MODE=ship and writes dist/site, then pages assembles dist/pages. Its `📋️project.json` build is an uncached release-driver orchestrator with a metadata DAG. Add explicit prototype/release entry points backed by parent mode policy, mode-specific deliverable ownership, and corresponding launch entries. Apply equivalent site-driver handling to demonstrator and other Vite apps after enumerating current commands.

## Existing Structure to Preserve

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts` builds into a unique mkdtemp directory, handles AbortSignal cancellation, and publishes only after Vite succeeds. Keep this interface and publication behavior.
- Play release jobs already allow flow/surface/puzzle WASM and map tiles concurrently; bundle waits only on these prerequisites, while final pages waits on bundle, support, and component publication. Avoid introducing a global dependency barrier.
- Nx plugin already records command-source closures and native graph admission. It has generator-output hashing to avoid assuming dependsOn alone fingerprints ignored outputs. Reuse these mechanisms.
- Framework typecheck explicitly scopes modules and packages, with an owned framework build-info file. Domain typechecks should similarly own their cache and input closure.

## Modularity

Root `📜️script.ts` currently has 26,023 lines and the Nx plugin entry `🟨️.mjs` has 1,711. Move build scheduling/mode policy out of root script into domain-neutral process modules, leave root command routing thin, and retain repo dependency/source inference in repo modules. Root BuildScript currently maps concrete product slices and directly implements Storybook work; product-specific implementation belongs to its owner script with root routing only.

## Required Validation

Create a language-agnostic fixture describing modes, output ownership, cache identity, and independent/dependent edits. Compare owned input-closure results with an independent TypeScript resolution/config oracle. Check different modes and simultaneous same-owner invocations cannot write the same mutable build-info/Vite staging files. Run actual Nx commands with console evidence before claiming runtime success. No validation success is claimed by this audit.
