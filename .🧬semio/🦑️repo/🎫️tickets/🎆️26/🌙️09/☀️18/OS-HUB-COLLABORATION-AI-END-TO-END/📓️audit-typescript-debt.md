# TypeScript type-error census — 2026-09-18

Read-only audit. Captures and parser live in this ticket's `🗑️generated/` (ts-census-root.txt,
ts-census-hub.txt, ts-census-renderer.txt, ts-census-parsed.md) and `🐍️ts-census.py`.

## 1. Tsconfig / typecheck inventory

`find . -name 'tsconfig*.json'` outside node_modules/target/.venv/storybook-static returned 44
files. Most are scratch/ticket-owned (`.🧬semio/🦑️repo/🎫️tickets/**`, `dist/**`, `temp/compose/**`,
`♻️mit-bestand/**`) or per-plugin package configs. The load-bearing ones:

- **Root** `tsconfig.json` — `include: ["**/*.ts","**/*.tsx", storybook .ts glob]`,
  `exclude: ["**/node_modules/**","js/temp","temp","reports","log"]`, `strict: true`, no `types`
  restriction (auto-includes all `@types/*`). This is a whole-repo glob with **no exclude for
  build output** (`storybook-static`, generated `dist/`, `🤖️generated/`), which matters below.
- **Hub** `🌎️hub/📦️packages/🟦️typescript/tsconfig.json` — `include: ["**/*.ts"]` relative to its own
  dir, `types: ["node"]` (excludes Bun globals), `paths` aliases `@semio-tech/framework-os` into
  `💻️os/📦️packages/🟦️typescript`. Program still pulls in the whole reachable import graph
  (`🎭️actor`, `🖱️ui`, `📚️library`, parts of `💻️os`).
- **Renderer** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json`,
  invoked by its own `nx typecheck` target (`📜️script.ts:56` → `tsc --noEmit -p tsconfig.json`).
- **os/dev has no dedicated tsconfig or typecheck target.** `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
  exists but has no `tsconfig`/`typecheck` logic; the only `tsconfig*.json` under that tree live in
  `dist/**` (build output). Dev-module `.ts` sources are only ever covered by the root glob.

nx targets named `typecheck`/`tsc`/`check-types` (grep of `📋️project.json`): exactly three —
`@semio-tech/ui-react` (framework/ui react), `@semio-tech/plugin-window-kits` (os/plugin/window-kits),
`@semio-tech/framework-renderer-react` (os/renderer react). No hub or dev nx typecheck target exists;
those must be invoked directly via `tsc -p`, as done here.

## 2. Captures

| capture | command | raw diagnostic lines | wall time |
|---|---|---|---|
| root | `bun tsc --noEmit -p tsconfig.json --pretty false` | 3552 `error TS…` lines | ~ran to completion, background |
| hub | same, `-p 🌎️hub/📦️packages/🟦️typescript/tsconfig.json` | 151 diagnostics (160 raw lines incl. wrapped detail) | 14.8s |
| renderer | same, `-p …/📺️renderer/…/📦️packages/🟦️typescript/tsconfig.json` | 865 diagnostics (1170 raw lines) | 1m15s |

**Root capture is not a trustworthy full-repo semantic census.** It completed (bun printed the
normal "exited with code 2" — tsc's standard "diagnostics found" exit status, not a crash trace),
but every one of its 3552 lines is a **parse/syntax** error code (TS1127/1434/1128/1443/1160/1109/1005/1137/1351)
and it only ever touched files under `storybook-static/`, `♻️mit-bestand/`, `✏️s/🔌️plugins/`, and a
slice of `🧰️framework/🛍️products/*`. It never surfaced a single diagnostic for `📺️renderer`, `📚️library`,
`🎭️actor`, or `🖱️ui` files — even though the hub/renderer captures prove those files have real (TS2xxx)
errors and are unambiguously inside the root glob. Treat the root capture only as evidence for the
parse-error cluster below (§4, cluster 1); use the hub/renderer per-package captures as the reliable
source for real semantic-error counts and areas. This is exactly the "OOM/anomalous — fall back to
per-package tsconfigs" case the task anticipated.

Deduped by `(file, line, code, message)` across all three captures: **2608 unique diagnostics**, of
which **1775 (68%) are the generated-file parse-error class** (cluster 1) and **833 (32%) are real
semantic errors** (TS7006 216, TS2345 147, TS2339 114, TS2304 74, TS2322 51, TS2868 32, TS2347 26,
TS2367 22, TS2353 17, TS2556 13, TS7053 12, TS2741 10, TS2749 10, TS2352 10, TS2307 10, TS2769 9, …).

### By area (hub + renderer captures, the trustworthy ones)

| area | hub | renderer |
|---|---:|---:|
| 💻️os/other (mostly `💻️os/🧪️tests/*`) | 58 | 199 |
| 🖱️ui (framework/ui) | 12 | 195 |
| 💻️os/…/📺️renderer | 0 | 141 |
| 🦑️repo/🔨️modules/📚️library | 13 | 116 |
| renderer's own `📜️script.ts` build tool | 0 | 98 |
| 🎭️actor (framework) | 46 | 47 |
| framework/modules-other | 22 | 31 |
| ✏️s (plugins) | 0 | 30 |

### Top files (root capture, dominated by cluster 1 — see caveat above)

Top 40 by count are all generated `*_component.d.ts` (wasm-component bindings under
`🧑‍💻dev/🧩️extension-modules/*` and `🔌️plugin-modules/*`, 132 distinct files, ~24 errors each),
`storybook-static/**/*.d.ts` (56 distinct files, 1344 lines), `jcoprobe.d.ts` bundles (2 files, 56
lines), and the fem-2d/3d CSV IO serializer (`✏️s/🔌️plugins/🏗️fem/…/🚪️io/…/🟦️.ts`, 34+34 lines,
committed 2026-09-07, not a live edit). **None of the top-40-by-count files were modified in the last
90 minutes** — the mtime/git-log check found no live-edit collisions among them.

For the actual **live-edit collision**, see cluster 5 below — those files carry only 1-2 errors each
so they never appear in a top-40-by-count list, but they are the ones that matter for scheduling.

## 3. Root-cause clusters

### Cluster 1 — Corrupted generated `.d.ts` bindings (emoji spliced into identifiers)
**~1775 deduped / ~3552 raw diagnostics (dominant cluster).** All TS1127 ("Invalid character") /
TS1434 ("Unexpected keyword or identifier") plus a handful of TS1128/1443/1160/1109/1005. Root
cause: an automated tool (matches the known 2026-09-03 "emoji/literal corruption" incident, see
memory `project-codex-rename-plan-codemod-incident`) spliced emoji+VS16 sequences into the middle of
identifiers inside auto-generated wasm-component bindings. Example (verified byte-for-byte):

```
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️extension-modules/🪟️sourcing-module-windows/semio_s_plugin_sourcing_windows_component.d.ts:2
export type * as WasiCliEnvironmen🔬️t029 from './interfaces/wasi-cli-environment.js';
```
`WasiCliEnvironmen🔬️t029` should read `WasiCliEnvironment029` — a 🔬️ (microscope+FE0F) got inserted
mid-word. Same pattern in `storybook-static/plugin-modules/*/semio_s_plugin_*_component.d.ts` (56
files), `💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-bundles/{out-jspi-explicit,out-callback}/jcoprobe.d.ts`
(2 files), and hand-authored files that embed similarly-generated/templated blocks:
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/…/🚪️io/…/🟦️.ts` (34+34 lines each, inside a JSDoc
comment block — same stray-character signature), `🧰️framework/🛍️products/📓️print/🔨️modules/{📊️visualization-gallery,🖨️tectonic-template-compilation}/🟦️.ts`,
and `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts`.
**Single fix:** this is a codegen/data-corruption problem, not a type-error backlog — regenerate the
~190 affected files from their wit-bindgen/jco source (or storybook rebuild), not hand-edit. Separately,
add `storybook-static`, `dist`, and `🤖️generated` to the **root tsconfig's `exclude`** so build output
never re-enters the census.

### Cluster 2 — Missing `"bun"` in tsconfig `types`
**32 "Cannot find name 'Bun'" (TS2868) + 4 "Cannot find module 'bun:…'" (TS2307), 36 deduped.**
Both `🌎️hub/📦️packages/🟦️typescript/tsconfig.json` and the renderer tsconfig set
`"types": ["node"]`, which suppresses Bun's ambient globals/module declarations. Examples:
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts:138`,
`📚️library/🟦️.ts:3013`, `📜️script.ts:499` (renderer's own build script). **Single fix:** add `"bun"`
to the `types` array in both tsconfigs (and confirm `@types/bun`/`bun-types` is installed) — a
one-line config change each, clears ~36 errors.

### Cluster 3 — `Taxonomy.testsDirName` renamed
**2 deduped `TS2551`/adjacent `TS2339` on `testFixturesDirName`, all 4 lines in one file.**
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:1184-1185` still reads
`taxonomy.testsDirName`; the type now offers `targetsDirName`/`testFixturesDirName`. **Single fix:**
update the two call sites in that one file.

### Cluster 4 — `DirectoryClient` used as a type + a broken relative import, same file
**1 `TS2749` + 3 `TS2307`, all in one file.** `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts`:
line 1835 uses `DirectoryClient` (a value) as a type (should be `typeof DirectoryClient`); lines
1742/1768(×2) import `'./🔨️modules/📇️directory/🧬️schema/🟦️.ts'`, which doesn't resolve — the module
was likely moved/renamed. **Single fix:** correct the relative import path and the type reference in
this one shared test file (it's pulled in by both the hub and renderer programs — see §5, it's not
owned by a single slice).

### Cluster 5 — `ShellState`/`FaultScope` drift in the wgpu/renderer bridge (LIVE PEER EDIT)
**1 `TS2353` on `ShellState.uiAppearance` + 2 `TS2353` on `FaultScope.req`.** Files:
`🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts:1459`, `🧱️elements/🔌️PluginRuntime/🟦️.tsx:3212`,
`🧱️elements/🔗️AgentBridge/🟦️.tsx:197`. All four candidate files (add `🧱️elements/🏛️ShellHost/🟦️.tsx`,
18 unrelated errors) were **modified 58-78 minutes ago** (`stat -f %m`) with an uncommitted diff on
top of a 2026-09-17 12:02 commit — i.e. live working-tree edits, not stale debt. **Do not slice this
into parallel repair.** It matches `.../🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/📓️status.md`, which is
actively mid-flight (git status shows `MM` on that ticket's own files right now) porting
wire-payload/typology/picking logic between the React and wgpu targets and explicitly logs touching
`♾️infinite/🌍️world`, `PluginRuntime`, and the wgpu plugin-bridge. This ticket's own status.md (line 28)
already lists "renderer TS 865 pre-existing errors" as known M2 debt, and line 29 (C1) references a
"PluginRuntime retry storm root fix" in the same file — two different agents mid-edit in the same
module tree.

### Cluster 6 — Long-tail real type debt (independent, safe to slice)
**~833 - (36+2+4) ≈ 790 deduped diagnostics** spread across otherwise-untouched files, mostly
`TS7006` (implicit-any, 216), `TS2345`/`TS2339`/`TS2304`/`TS2322` (mismatched args/missing
props/undeclared names/assignability). Heaviest single files: `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`
(152), `📚️library/🕸️dependencies/📇️inventory/🟦️.ts` (59), `💻️os/🧪️tests/🔬️interactivity-p1q-r4/🟦️.ts`
(59), `💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts` (45), `📚️library/🧹️normalization/🟦️.ts`
(27, includes a `caseId`/`TaxonomyRemovalAuthority` shape mismatch — likely the same taxonomy-schema
drift as cluster 3), `♾️infinite/🌍️world/🎨️r3f/🧪️tests/🧪️chunkkey/🟦️.tsx` (21). No single fix; genuine
per-file/per-module debt.

## 4. Recommended repair slices

1. **Slice "codegen regen"** (not a manual-fix slice) — regenerate the ~190 corrupted generated
   `.d.ts` files (cluster 1) via their normal build step, and add `storybook-static`/`dist`/`🤖️generated`
   to root tsconfig excludes. Owner: whoever owns the wasm-component/jco pipeline + repo tsconfig.
   Eliminates ~3550 raw / ~1775 deduped lines with zero hand-editing. **Do first** — it's what makes
   every other slice's error count legible (right now it's ~68% noise).

2. **Slice "🦑️repo/📚️library + hub config"** — clusters 2 (Bun types, both tsconfigs), 3
   (`testsDirName`), part of cluster 6 (`📇️inventory` 59, `🧹️normalization` 27, `🔍️discovery`).
   Roughly hub's 151 + the library-area subset of renderer's 116 ≈ **160-200 errors**, mostly in one
   directory (`🦑️repo/🔨️modules/📚️library`) plus two tsconfig.json one-liners. No live-peer overlap
   found.

3. **Slice "framework/ui + framework/actor tests"** — `🖱️ui` test fixtures (owned-locale-detector,
   theme-resolve, playgroundflowwasmdevstubplugin ≈ 195 in renderer capture, 12 in hub) + `🎭️actor`
   test fixtures (mailbox/createboundedmailbox, turnscheduler-lane-priority, etc ≈ 46-47 in both
   captures). Est **~250-300 errors**, self-contained under `🧰️framework/🔨️modules/{🖱️ui,🎭️actor}/🧪️tests`.
   No live-peer overlap found.

4. **Slice "os/tests + os/renderer, EXCLUDING the four live wgpu files"** — `💻️os/🧪️tests/*`
   (`backbone-envelope-io` 48/58, `space-artifact-creation-owner` 45, `interactivity-p1q-r4` 59) +
   the rest of `📺️renderer` (141) minus `🧊️wgpu/🐚️plugin-bridge`, `🔌️PluginRuntime`, `🔗️AgentBridge`,
   `🏛️ShellHost` (cluster 5, ~23 errors — **defer these explicitly**, do not touch). Also fix cluster
   4 (`DirectoryClient`/broken import) here since `backbone-envelope-io` is the file. Est **~250-300
   errors after excluding the live 23**.

5. **Slice "build scripts + s-plugins"** — the renderer typecheck's own `📜️script.ts` (98 errors,
   likely implicit-any in the build tool itself) + the 30 `✏️s`-area renderer diagnostics (verify
   these aren't part of cluster 1 before counting — the root capture's `✏️s` numbers were 100%
   parse-class, but the renderer capture's 30 need a fresh check since renderer's tsconfig may
   resolve different `✏️s` files via import graph). Est **~100-130 errors**, lowest confidence slice —
   scope it with a quick grep before assigning.

**Flag for the coordinator:** cluster 5 (`ShellState`/`FaultScope`, ~23 errors across 4 files) is
owned by the live `WGPU-RENDERER-REACT-PARITY` (☀️17) session — exclude it from every slice above and
re-poll after that ticket's next commit. No other cluster showed a live-edit (<90 min mtime) collision.
