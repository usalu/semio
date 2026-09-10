# 🎞️ wgpu frame-worker staleness — root cause, fix, build proof (2026-09-10)

Lane: wgpu wasm playground (`SEMIO_RENDERER=wgpu`, `procedural 3d`).
Symptom: `🎞️frame-worker.js is stale; run the generate-frame-worker target` → `wgpu trunk serve failed`,
even immediately after `bun nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker` succeeded
twice with an identical sha.

## 1. Reproduction and diff

`checkFrameWorker` re-renders the worker in memory and compares byte-for-byte with the tracked
`🟦️typescript/🎞️frame-worker.js`. Rendering the same `bundleRoot` from the two cwds the two callers
actually use reproduces the failure exactly.

| render context | bytes | sha256 (12) | vs tracked file |
| --- | --- | --- | --- |
| cwd = repo root (what the dev lane uses) | 1 131 201 | `6b65104753c0` | 162 differing lines |
| cwd = `…/🧊️wgpu/📦️packages/🦀️rust` (what the Nx target uses) | 1 130 503 | `4129b5bd8170` | 16 differing lines |
| tracked `🎞️frame-worker.js` (as found) | 1 130 503 | `325ccaf07e01` | — |

Two independent differences, on two different axes:

**(a) cwd-derived module banner comments — 162 lines.** Every bundled module carries a banner spelling
its own path, and Bun spells it relative to `process.cwd()`:

```
< /* ../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts */      (rendered from the package dir)
> /* 🧰️framework/🔨️modules/🛂️manifest/🟦️.ts */                     (rendered from the repo root)
< // ../../../../../../../../../../node_modules/react/cjs/react.development.js
> // node_modules/react/cjs/react.development.js
```

**(b) per-build wasm identity — 16 lines / 10 distinct SHA-256 strings.**

```
< …executionMode: "isolated", hashes: { wasmSha256: "844bdecb8b39…", coreWasmSha256: "411cdb951c5c…", descriptorSha256: "9a5210b31598…" } },
> …executionMode: "isolated", hashes: { wasmSha256: "f3bb398088c7…", coreWasmSha256: "052e636fa90b…", descriptorSha256: "4222de511cf4…" } },
```

Raw evidence: `🗑️generated/fw-committed.txt`, `fw-render-root.txt`, `fw-render-pkg.txt`,
`fw-diff-root.txt`, `fw-diff-pkg.txt`. Probe used: `🔬️fw-render-probe.ts` (calls the production
`renderFrameWorker` with the production `bundleRoot`, writes the bytes, prints cwd/bytes/sha).

## 2. Root cause

### (a) The render was a function of `process.cwd()`

`renderBrowserEntry` — `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⚙️browser-build/🟦️.ts:29-38`
(pre-fix) — called `Bun.build({ entrypoints: [entryPath], target: "browser", … })` with whatever cwd the
caller happened to have. Bun labels each bundled module with a banner comment relative to `process.cwd()`;
the `root` build option does NOT govern that label (verified empirically on Bun 1.3.14: passing
`root: <repoRoot>` still produced `../🦑️repo/…` banners when run from a nested cwd).

The two callers never share a cwd:

- generate: `📦️packages/🦀️rust/📋️project.json:139-145` — Nx runs `bun ./📜️script.ts generate-frame-worker`
  with `"cwd": "🧰️framework/…/🧊️wgpu/📦️packages/🦀️rust"`;
- check: `📦️packages/🦀️rust/📜️script.ts:206-210` (`checkFrameWorker`, called from `TrunkBuildScript.run`
  at `:217` and the serve path at `:1496`) runs in the dev lane's process, whose cwd is the repo root.

So `generate` wrote package-relative banners and `check` demanded root-relative ones. That is why running
the generate target twice was deterministic (same cwd both times) yet the dev lane's very next check
reported stale — the two observers were never rendering the same thing.

### (b) The tracked artifact baked machine-local, per-build data

The worker imports `PLUGIN_CATALOG` (`🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:4`) →
`🔨️modules/🔌️plugin/📇️registry/🟦️.ts:10` → `🤖️generated/🧩️plugins.ts`. That generated module carried a
`hashes: { wasmSha256, coreWasmSha256, descriptorSha256 }` field on every row, emitted at
`🔨️modules/🔌️plugin/📇️registry/📜️script.ts:690` (pre-fix). `toCatalogTarget` drops the field, but a
bundler cannot tree-shake string literals out of the source array, so all 59+ rows of build identity were
inlined verbatim into the tracked `🎞️frame-worker.js`.

`🤖️generated/` is gitignored (`.gitignore:91`) and those hashes change on every plugin core rebuild —
which the dev lane performs (`plugin catalog build summary: 11/11 crate(s) produced .wasm`) immediately
before it checks the worker. A tracked artifact defined as a function of untracked, per-build inputs can
never be fresh: not after a plugin rebuild, and not at all on a fresh clone.

## 3. Fix

### `…/🧊️wgpu/⚙️browser-build/🟦️.ts` — render at one fixed anchor, for every caller

`renderBrowserEntry` now resolves the entry to an absolute path, anchors `process.cwd()` at the workspace
root for the duration of the bundle, and restores the caller's cwd in a `finally`. Renders are serialized
through a `bundleLane` promise chain so a concurrent caller never observes another render's anchored cwd.
Generate and check already shared one render function (`renderFrameWorker`) and one path resolution
(`join(bundleRoot, …)`); they now also share one cwd, which was the missing half.

### `🔨️modules/🔌️plugin/📇️registry/📜️script.ts` — build identity leaves the browser-facing module

`emitTypeScript` no longer renders `hashes` into `🤖️generated/🧩️plugins.ts`, and `PluginBuildTarget` /
`PluginDescriptorHashes` are gone from the emitted type surface. Per-build artifact identity stays where
it is actually verified — the descriptor pair and `🤖️generated/🔌️plugins.json` (42 hash fields, unchanged),
audited by `validateCatalogDescriptorValue`/`verifyDescriptorPairBytesV1` from the in-memory
`PluginRegistryEntry`, which still carries `entry.hashes` and is unaffected. No consumer read `hashes` off
the emitted TS module (verified repo-wide; the emitter at `:690` was the only writer and there was no
reader). The tracked worker bundle is now a pure function of tracked sources: `grep -c '[0-9a-f]{64}'` over
`🎞️frame-worker.js` is 0, and the artifact shrank 1 130 503 → 1 120 358 bytes.

### `…/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts` — worker-global side effect off module scope

A module-scope `self.addEventListener("message", …)` (added 2026-09-09, `599a5d8450`) made merely
*importing* the bridge throw `ReferenceError: self is not defined` in any non-worker host, which took the
whole `@semio-tech/framework-renderer-wgpu` vitest suite down at import (0 of 53 tests ran) and would have
blocked the determinism test below. The route is now armed lazily by `routeShardPorts()` on the first
`MainThreadShardWorker` construction — the only moment a `shard-port` handoff can arrive — instead of at
module scope.

### Test

`🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts` gains a case that renders the frame worker three times
from three unrelated cwds and env sets (repo root / the package dir with
`SEMIO_RENDERER=wgpu S_OS_PORT=6118 CARGO_TARGET_DIR=…` / the repo's parent directory with
`SEMIO_RENDERER=react S_OS_PORT=6018 NODE_ENV=production`), asserts all three SHA-256 digests are equal,
asserts the caller's cwd is restored after each render, and asserts the bytes contain no 64-hex run at all
so the artifact cannot silently re-acquire per-build identity.

```
bun --bun x vitest run --config vitest.config.ts   (cwd = 🧊️wgpu/📦️packages/🦀️rust)
Test Files  3 passed (3)
     Tests  53 passed (53)
```

## 4. Build proof

```
bun nx run @semio-tech/plugin-registry:generate                          → ok (59 plugin crates, 60 playgrounds)
bun nx run @semio-tech/framework-renderer-wgpu:generate-browser-boot     → ok
bun nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker     → ok
bun ./📜️script.ts check-frame-worker         (cwd = package dir) → "🎞️frame-worker.js is fresh"
bun …/📜️script.ts check-frame-worker         (cwd = repo root, dev-lane env) → "🎞️frame-worker.js is fresh"
```

Non-serving bundle half of the dev lane (`TrunkBuildScript`, registered as `build`/`wasm`; it runs
`ensureTrunk` → `ensureWasmTarget` → `checkBrowserBoot` → `checkFrameWorker` → `trunk build --config
Trunk.toml` → `syncStableRendererArtifacts`):

```
CARGO_TARGET_DIR=$S/target-wgpu-boot RUSTC_WRAPPER="" SEMIO_RENDERER=wgpu CARGO_PROFILE_WASM_DEV_DEBUG=false \
  bun "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts" build
```

First attempt got past `checkFrameWorker` and Trunk reported `✅ success` / `Finished dev profile in
3m 30s`, then the script itself threw `missing trunk wgpu renderer js artifact` from
`syncStableRendererArtifacts` (`📦️packages/🦀️rust/📜️script.ts:146-153`). A separate pre-existing bug,
newly reachable now that the worker check passes: that function scanned `outDir` for a `<crate>-` prefix,
i.e. for Trunk's CONTENT-HASHED artifact names, but `Trunk.toml` pins `filehash = false`, so Trunk emits
`semio-framework-os-renderer-wgpu.js` / `…_bg.wasm` — no dash, never matched. Rewritten to address both
files by their exact pinned names and fail with the missing path. The copies are load-bearing: the React
shell's `🎬️renderer-boot/🟦️.ts:15` requests
`/renderer-modules/wgpu/semio_framework_renderer_wgpu.js`.

Second attempt — **the non-serving build completes**:

```
Finished `dev` profile [unoptimized] target(s) in 3.00s
INFO applying new distribution
INFO ✅ success
trunk built wgpu renderer -> /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu
```

Dist `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu`:

| file | bytes |
| --- | --- |
| `index.html` | 1 956 |
| `semio-framework-os-renderer-wgpu.js` | 174 386 |
| `semio-framework-os-renderer-wgpu_bg.wasm` | 75 969 615 |
| `semio_framework_renderer_wgpu.js` (stable copy) | 174 386 |
| `semio-framework-renderer-wgpu_bg.wasm` (stable copy) | 75 969 615 |
| `🎞️frame-worker.js` | 1 120 358 |
| `🚀️boot.js` | 43 694 |
| `🔌️plugin-modules/` | 61 entries, incl. `🌀️procedural/semio_s_plugin_procedural_component.core.wasm` (80 681 563) |
| `🖼️assets/` | 19 entries |

`trunk serve` was NOT started from this lane.

## 5. Serve command for the coordinator

The bundle half is now unblocked; the serve half is unchanged and still belongs to the main session:

```
cd /Users/ueli/Documents/semio
CARGO_TARGET_DIR=$S/target-wgpu-boot CARGO_PROFILE_WASM_DEV_DEBUG=false RUSTC_WRAPPER="" \
NX_DAEMON=false SEMIO_RENDERER=wgpu S_OS_PORT=6118 \
  bun ./📜️script.ts dev procedural 3d
```

Trunk's own dist for this target is `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/🧊️wgpu` (pinned by
`assertRendererCacheHome`, which asserts `Trunk.toml`'s `dist` resolves there).

## 6. Unrelated failures observed (peer churn, not this fix)

`@semio-tech/plugin-registry` vitest: 26 passed, 3 failed — none touching the emitter change.

1. `🧪️tests/🚀️launch/🟦️.ts` — WASI codegen profile policy route mismatch (`.vscode/launch.json` is
   regenerated by the registry generate target and by other live lanes).
2. `🧪️tests/✅️catalog-complete/🟦️.ts` — `ENOENT … 🧫️fixtures/🧬️catalog-complete/🧬️schema/🔣️.json`
   (the ASSETS-FIXTURES-SEPARATION ticket is moving fixtures in this tree).
3. `🧪️tests/✅️catalog-complete/🟦️.ts` — "known 19 missing source pairs" is now 17; the coordinator's
   plugin build produced descriptors for `flow-extension-bim` and `flow-extension-draw`, so that
   expectation needs re-baselining by whoever owns it.

## 7. Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⚙️browser-build/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js` (regenerated)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🚀️boot.js` (regenerated)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🔬️fw-render-probe.ts` (ticket probe)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📓️wgpu-frame-worker-2026-09-10.md` (this report)
