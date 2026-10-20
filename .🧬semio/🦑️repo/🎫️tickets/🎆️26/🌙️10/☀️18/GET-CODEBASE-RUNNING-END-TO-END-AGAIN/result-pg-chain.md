# result-pg-chain: plugin playground pipeline cached under nx and Bazel (wave 1020d, resumed, stage only)

READY: staged `C/pg-chain-tools/apply.py` (dry run by default, `--write` applies; 14 edited targets, 2 new files; bases recorded 18:35). Nothing written outside `C/`, nothing applied, no build run.

`C` = this ticket. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. `I` = `L/🏗️build/🕸️graph/📥️inference`. The 1020c agent died before staging anything (`edits.py` was empty); this set is written from scratch on the live tree.

## 1. What changes

| Step | Before | After |
| --- | --- | --- |
| `<crate>:component` (68 play components, 44 hub, any plugin/extension crate) | uncached `driver-owned`, outputs = raw store paths | cached `deterministic/pure-function-of-inputs`, serial (`parallelism: false`), one output: the store receipt `{projectRoot}/.🧾️store/component.json` (verified against the products store on every hit; the publication already records into it through `withRepositoryArtifactDestinationV1`). Inputs: `nativeSources`, `^nativeSources`, dependent outputs (describe emitter, support receipt), committed descriptor pair (`<crate>/../..`), pinned Binaryen manifest file, Rust toolchain files/env/commands from `policy.json`, `jco` + `preview2-shim` external dependencies, platform, Cargo.lock closure (project-assembly), driver import closure (automatic) |
| `framework-plugin-web:support` | uncached `managed-output` | cached, output `{projectRoot}/.🧾️store/support.json`, inputs `sharedGlobals`, `browserBundleSources`, `preview2-shim` (script closure automatic) |
| `prepare-<lane>-react\|wgpu-<profile>` (x2 profiles, plus profile-less aliases) | uncached `driver-owned`; ran the build driver | cached `deterministic/check-without-outputs`; depends on session, support, fonts, engines and the `component` targets of the boot closure; the script no longer builds |
| `prepare-<lane>-native-<profile>` | uncached `driver-owned` | cached `deterministic/pure-function-of-inputs`, receipt output; depends on session + `component` targets of the full closure; script no longer builds |
| `activate-<lane>-react-<profile>` | cached, inputs only dependent outputs | + owner-tree inputs of the boot closure (`writeStagedSourceFreshness` hashes them) |
| `activate-<lane>-wgpu-<profile>` | uncached `side-effecting/publishes-live-state` | cached like react (`ActivationScript` has no wgpu-specific effect; same runtime-dir output). Verify after GO, see 3 |
| `build-<lane>-react`, `build-<lane>-react-release` (312) | uncached `driver-owned`; built components inside | cached `deterministic/pure-function-of-inputs`, receipt output, serial. New named input `viteBundleSources` (os-dev project): framework tree and `✏️s/🧑‍💻dev` minus tests, generated dirs and receipts, root `package.json`/`bun.lock`/`bunfig.toml`/`tsconfig.json`; plus lane owner trees, the dev contribution file, its `ownerRoot` and `browserSessionFactories` engine trees, S_HUB_URL/S_DATA_DIR, distribution inputs probe; the script no longer builds components |
| Bazel `bazelCargoClassV1` | only `native cargo ... --manifest` | also `component --manifest` of an orchestration task: `exclusive` action and Rust toolchain digest in the key |
| `build-all-playground-cdn-sites` | uncached | stays: `side-effecting/publishes-remote` |

Scripts that built components inside their lane task (os-dev `prepare`, distribution `build`, wgpu native `publish`) lost the `buildComponentClosureV1` call. Direct tools that build one component on demand (`components iterate`, store packaging, catalog browser-host staging, the dev-server activation plugin, describe) are unchanged: interactive/on-demand, not graph lanes.

Final uncached count of the pipeline lanes: **1** (the CDN publish). Remaining uncached around it by nature: `deps-wasm-opt` probe, `run-/smoke-<lane>-native-*` (live window), `serve-/dev-/mcp-*` (continuous), `build-<lane>-site` aggregates (noop ordering).

## 2. Files (staged copies are edits in `C/pg-chain-tools/edits.py`; `apply.py status|apply [--write] [--rebase]`)

Library: `I/🧩️component-targets/🟨️.mjs`, `I/🧱️project-assembly/🟨️.mjs` (no driver-owned wrapper for `component`; lock inputs for `componentStoreV1`), `I/🎮️playground-targets/🟨️.mjs`, `L/🏗️build/🔧️tools/🏗️bazel/🦀️cargo/{🟦️.ts,🧬️schema/🔣️.json,🧪️tests/🔣️.json}`.
Product: os-dev `♻️activation/🧰️preparation/🟦️.ts`, `🚚️distribution/📜️script.ts`, `📦️packages/🟦️typescript/📋️project.json` (`viteBundleSources`); wgpu `⌨️native-entrypoint/📦️modules/📜️script.ts`; `🔌️plugin/📦️packages/🟦️typescript/📋️project.json` (`support`).
Tests: `I/🧪️tests/💾️cache/🔣️.json` (8 template rows), `L/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`, `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🏗️preparation/🟦️.ts`; new `I/🧪️tests/🧩️component-cache/{🟦️.ts,🔣️.json}` (language-agnostic fixture; expectations read from `policy.json` and the store-receipt schema; Ajv validates the decision and receipt target).

## 3. Verification (overlay copy `/private/tmp/pgc-ov`, one runner at a time)

- `🧩️component-cache`: 1 pass, 48 expects; mutation (`parallelism` flipped) fails it.
- `💾️cache` (corpus + synthesized playground targets incl. build rows): 10 pass, 822 expects.
- Bazel cargo suite with 5 new class rows: 7 pass, 1 skip (as before).
- S preparation test over all 156 lanes x 2 profiles on the real catalog: pass (overlay run; I disabled `testSelectedRuntimeDependencies` there: its fixture lists engines (`flow-browser-package`) the live graph does not give puzzle3d, a drift independent of this set; `timeout 30000` raised, generation takes 45 s under load 22).
- `bun build --no-bundle` of the three edited scripts and the Bazel cargo module: ok.
- Not run: the live graph, `show projects`, any cargo/jco build, `cache-contracts` (heavy, already stale on `dist/runtime` output paths), Bazel.

## 4. After GO (order, one heavy process at a time)

1. `python3 C/pg-chain-tools/apply.py apply` (dry), then `--write` (refuses on drift; `--rebase` after reviewing; coordinator note: round-2 groups landing now, anchors were still matching at 18:35).
2. Library suites: `bun test ./I/🧪️tests/🧩️component-cache/🟦️.ts ./I/🧪️tests/💾️cache/🟦️.ts ./L/🏗️build/🔧️tools/🏗️bazel/🦀️cargo/🧪️tests/🟦️.ts`.
3. One component twice: `bun repo run <hub-note-component>:component` (cold), again (0 spawns), check `.🧾️store/component.json` holds the module and model directory entries; same under `--tool=bazel`.
4. Receipt content must be stable: module directories must not receive later writes (hot-swap file); if a receipt keeps failing verification, find the writer.
5. One lane: `prepare-<lane>-react-prototype` + `activate-...` warm = 0 spawns; edit one component source: only its `component`, the lane verdicts containing it and the lane activate re-run.
6. `activate <lane> wgpu <profile>` once and diff its output tree against react's shape (claim: no live side effect); revert `activate-*-wgpu` to uncached if one appears.
7. Vite bundle soundness proof: run one `build-<lane>-react` under the node:fs tracer of play-bzl (`play-bzl-tools/fs-trace.ts`, `compare-reads.ts`) and diff reads against `viteBundleSources` + lane trees; add misses to the named input. Hashing the framework tree once per distinct lane key is the price.

## 5. Open items and risks

- Cold cost: 68 components are serial (`parallelism: false`, Cargo is exclusive anyway); estimate unchanged (~3 h) but every later run is a hit.
- A receipt holds store content only; a fresh checkout with a warm remote cache re-executes components (store-aware restore is the adapter owner's item, see result-play-bzl section 4).
- `policy.json` still carries the exact `driver-owned` rows for `prepare-prototype`/`activate-prototype`/`prepare-release`/`activate-lanes-prototype`; play-bzl and demo-bzl override them per manifest. Remove the rows once both are applied (library edit, not staged here).
- Existing tests that asserted the old shape are rewritten (`prepare` has no outputs, depends on `:component`); the old `vectors.closurePreparation.prerequisites` are now asserted absent from the lane.
- `bazel-nocache` message answered by this file: all prepare shapes, wgpu activate and the 312 Vite bundles are covered; only the CDN publish stays uncached.
