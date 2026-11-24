# Procedural component staging (2026-11-23)

## Root cause of the wedge

The build `repo run semio-hub-procedural-component:component` (door 96971 -> 25339 -> nx 79402 -> 80080 -> 80295 -> `80298 bun exec repo`) was not stuck itself. `repo leases status` showed 80298 as `WAITING exclusive tools:descriptor-emitter:prototype:darwin ... waiting for 58486`. Process 58486 (started by `native-launch.ts` of the WGPU lane, log `🗑️generated/wgpu-native-components.txt`) held that exclusive lease for 6778 s while being idle (0 % CPU, no children, no cargo child).

58486 had died in its own cargo diagnostics drain when the disk was full:

```
ENOSPC: no space left on device, write
  at writeSync  ... 🏗️native-build/🟦️.ts:428  (BoundedDiagnosticsV1.line)
  at readline "line" event (child.stderr)
```

`BoundedDiagnosticsV1.line` wrote the complete-log copy with an unguarded `writeSync` inside a readline `line` handler. The throw escaped into the event emitter, the stderr readline interface never reached `close`, `diagnosticsDrained` never resolved and the build promise never settled. The progress heartbeat (`still running after N s`) kept printing, the lease stayed exclusive, and every other component build queued behind it for ~2 h.

Remedy: killed only 58486 and its parent 58479 (both launched from this ticket's `native-launch.ts`). The lease released at once and 80298 acquired it and started rebuilding the descriptor emitter (`Descriptor emitter is stale (203 changed source file(s))`).

## Fix (repo tool source, with test)

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`: `BoundedDiagnosticsV1.line` treats the log copy as best effort. A write failure sets `logFailed`, stops logging and keeps forwarding and counting. `finish` then no longer points to the incomplete log file.
- Test `a failing diagnostics log write never aborts the diagnostics stream a compiler build drains` in `🏗️native-build/🧪️tests/🟦️.ts` (a closed fd stands in for ENOSPC; run with `SEMIO_TEST_ARTIFACT_DIR=<dir> bun test ./🧪️tests/🟦️.ts -t "diagnostics"`: 2 pass).
- Still open (separate hardening): the `child.stderr` readline has no `error` listener and `diagnosticsDrained` should also resolve on `error`; a lease holder that has no live child and no progress for N minutes should be reported by `leases status` as `idle holder`.

## Catalog regeneration (React lane's hand edits)

Graph-free check: `bun catalog-preview-native.ts` renders the real generator (`renderCatalogFiles` with `specificCatalogPublicationV1`) and byte-compares. Result: the `generation3d` and `generation2d` rows, including `devContribution`, are byte-equal to the generator output in `🚀️playgrounds.json`, `🎠️playgrounds.json` and `🎮️playgrounds/🟦️.ts`. The remaining drift is unrelated to this ticket (other lanes' source changes: examples now emitted without their emoji prefix; `devContribution` rows for `stdio-pdf`/`stdio-xlsx`; `🔌️plugins.json` `dependsOn` of one plugin). It was not written to avoid overwriting peers.

## Staging result (2026-11-23 12:07)

The live repo build staged `🌀️procedural` (path: `repo run semio-hub-procedural-component:component -c prototype --exclude-dependencies`, log `🗑️generated/native-component-procedural-4.txt`). The native cargo fallback was abandoned (disk full at 1021 crates, own scratch `tp2` deleted); the private-root lane (`📓️private-root-build-2026-11-23.md`) did not stage it either. The run took about 6 h wall clock: lease wedge (above), repeated descriptor-emitter rebuilds from peer edits, one `CargoLockRefused` (`semio-os-spr` path, corrected by its owner).

## Start commands

```
# activation receipts (graph-free twin of activate-generation3d-{wgpu,react}-prototype)
bun TF/wgpu-native-activate.ts                       # wgpu
S_G3D_RENDERER=react bun TF/wgpu-native-activate.ts  # react
# serves (graph-free)
screen -dmS g3dwgpu TF/serve-generation3d-wgpu-native.sh   # 6118, real catalog from 🧊️wgpu/🟦️.ts
screen -dmS g3dreact TF/serve-generation3d-native.sh       # 6018
# smoke + screenshots
bun TF/wgpu-native-smoke.mjs                      # SEMIO_BATTERY_URL=... for 6018; SMOKE_MS
```

## Verification (13:10)

- 6118: boot passes the plugin registry; it now stops with `plugin.descriptor-unavailable: …/extension-modules/flow-extension-bim/🔣️.json (HTTP 404)`: the flow component and the nine flow extensions are not staged. Build started: `repo run-many -t component -p "semio-hub-flow-component,semio-s-plugin-flow-*-component" -c prototype --exclude-dependencies --parallel=1` (log `🗑️generated/native-component-flow-1.txt`, still in graph read after 34 min). Evidence: `🗑️generated/wgpu-6118-{shot.png,console.txt}`.
- 6018: shell renders (toolbar, "semio · procedural · 3d Editor"); the canvas shows `original-turn.native-return-refused: semantic` from the guest: `member.publication: original page allocation attempt is occupied` (`🌱️value/📋️list/🦀️.rs:590`, `🏪️store/🎮️one-item/📬️publication`), a retained-allocation bug another lane is debugging with `[DEBUG]` prints (50+ console errors). Evidence: `🗑️generated/react-6018-{shot.png,console.txt}`.

## Lazy extension activation: bundle rebuild (17:10)

- `bun TF/wgpu-native-generate.ts` initially failed: `WGPU browser import is not schema-owned: ../../⏳️on-demand-activation/🟦️.ts` (new kernel module not in the WGPU browser source list) and then `@semio-tech/framework-kernel/plugin-registry` (no workspace-import binding). Fixed at the schema: added `kernel/⏳️on-demand-activation/🟦️.ts`, `kernel/📦️packages/🟦️typescript/🗂️plugin-registry/🟦️.ts` and `kernel/🗂️plugin-registry/🟦️.ts` to the WGPU browser source lists (wgpu `📦️packages/🟦️typescript/📋️project.json` inputs and `🔣️taxonomy.json`, same rows as the neighbouring kernel entries; helper `add-browser-source.py`) and the `@semio-tech/framework-kernel/plugin-registry` binding to the taxonomy workspace imports. Boot, renderer-boot and frame-worker bundles then regenerate.
- 6118 restarted and smoked: the boot plan no longer requests `flow-extension-bim`; the first missing descriptor is now the flow hub: `plugin-modules/🌊️flow/🔣️.json` (HTTP 404). Evidence: `🗑️generated/wgpu-6118-{shot-2.png,console-2.txt}`.
- Remaining: stage the `🌊️flow` hub component (other lane), then re-smoke.
