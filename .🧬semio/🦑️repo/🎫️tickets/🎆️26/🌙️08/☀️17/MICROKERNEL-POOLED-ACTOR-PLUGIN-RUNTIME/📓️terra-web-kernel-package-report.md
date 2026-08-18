# terra-web-kernel-package report

## delivered

New real TS package, modeled on `🎭️actor/📦️packages/🟦️typescript/` and on the `../../🟦️component.ts`-wrapping
shape used by `framework-os`/`framework-os-mcp`/`framework-os-shell` (kernel, like those three, keeps its real
source at the module root, not inside `📦️packages/`):

- `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/package.json` — `@semio-tech/framework-kernel`, `exports: {".": "./🟦️glue.ts"}`, `dependencies: {"@semio-tech/framework-actor": "workspace:*"}` (documentation only — see honest gaps).
- `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/📋️project.json` — `test`/`test-quick`/`test-long`/`test-exhaustive` targets, `nx:run-commands` → `bun ./📜️script.ts test [level]`, mirrors actor's exactly.
- `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/📜️script.ts` — identical router shape to actor's (`BundleScript`/`ScriptRouter`/`resolveTestLevel`/`runBundleScriptMain`/`runVitest` from the shared repo lib).
- `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/🟦️glue.ts` — `export * from "../../🟦️component.ts"` (same glue pattern as `framework-os`/`framework-os-shell`).
- `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/🧪️vitest.config.ts` — `root = resolve(configDir, "../..")` (kernel module root), `environment: "jsdom"` (matches the verified ad-hoc `terra-t1-kernel-vitest.config.ts`), `includeSource`/`coverage.include: ["*.ts"]` (glob, not an explicit filename array — see next paragraph), `include: []`.

**Why `include` is empty and not the same glob as `includeSource`**: my first cut set `include`/`includeSource`/`coverage.include` all to `["*.ts"]` (mirroring how the actor/os/mcp/shell sibling configs list the *same* file(s) in `include` and `includeSource` identically) and it reported **58 passed**, not 29 — every in-source test ran twice, once via the normal `include` test-file path and once via the `includeSource` in-source path. I re-ran the actor sibling's own `bun ./📜️script.ts test` standalone (untouched, not part of my owned paths) and it reports **"6 test files" / 58 tests passed** for only 3 real source files — i.e. the sibling I was told to model this on has the exact same doubling bug live today. This is the ticket's own recurring defect class ("a check that exists but never runs, or doesn't mean what it says") showing up a second way: not an orphaned suite, but a suite quietly running every test twice while reporting green. I did not touch the actor package (outside owned paths) — flagging it below and in honest-gaps instead. The proven-correct shape, already used by the ad-hoc `terra-t1-kernel-vitest.config.ts` (`include: []`, only `includeSource` set), is what I kept for the new kernel config, with `include: []` explicit and documented inline so a future editor doesn't "fix" it back to matching arrays.

## orphaned-suite inventory

Scope: every `🟦️component.ts`/`🟦️component.tsx` file repo-wide containing `import.meta.vitest` with ≥1 real
`it(` (42 candidates found; files with 0 actual tests behind the guard — `📇️directory/🟦️component.ts`,
`AgentBridge/🟦️component.tsx`, `🧬️typology`/`📔️registry` under the cad artifact editor engine,
`🧑️‍💻️dev/🟦️component.ts` — are omitted as moot), plus the non-inline `🧪️component.test.tsx`/`.ts` element
suites this ticket's own prior packets already flagged as a second orphan shape. "Tests found" = number of
top-level `it(` occurrences (approximate but consistent with the counting used elsewhere in this ticket).

| file | tests | config | verdict |
|---|---|---|---|
| `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` | 29 | **new** `🎠️kernel/📦️packages/🟦️typescript/🧪️vitest.config.ts` (this packet) | **in-gate** (was orphaned before this packet) |
| `🧰️framework/🛍️products/💻️os/🟦️component.ts` | 63 | `os/📦️packages/🟦️typescript/🧪️vitest.config.ts` (`include`=`includeSource`=`["../../🟦️component.ts","../../🟦️backbone-worker.ts"]`) | in-gate, but **same include/includeSource doubling bug as actor** — likely double-counted, not fixed (outside owned paths) |
| `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts` | (bundled w/ above) | same os config | same caveat |
| `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts` + `📬️mailbox.ts` + `🧵️turn-scheduler.ts` | — | actor's own `🧪️vitest.config.ts` | in-gate, **verified doubling** ("6 test files"/58 tests for 3 files) — the sibling this packet was told to model on; not fixed (outside owned paths) |
| `🌉️mcp/🟦️component.ts` | 2 | `mcp/📦️packages/🟦️typescript/🧪️vitest.config.ts` (`include`=`includeSource`=`["../../🟦️component.ts"]`) | in-gate, same doubling-bug shape, not verified numerically |
| `🖥️shell/🟦️component.ts` | 3 | `shell/📦️packages/🟦️typescript/🧪️vitest.config.ts`, same shape | in-gate, same doubling-bug shape |
| `📐️cad/🧩️extensions/{🏢️aec-building,📐️spatial-shape,🏛️aec-building-structure,🔥️aec-building-energy}/🟦️component.ts` (1,2,2,1) | — | each extension's own `📦️packages/🟦️typescript/🧪️vitest.config.ts`, `include`=`includeSource`=`["🟦️component.ts"]` (root redefined via `resolve(configDir,"../..")`) | in-gate, same doubling-bug shape ×4 |
| `🎞️animate/…/✏️editor/📺️renderer/⚛️react/🟦️component.tsx` | 136 | `animate/📦️packages/🟦️typescript/🧪️vitest.config.ts`, same identical-array shape | in-gate, same doubling-bug shape |
| `🌎️hub/…/📚️I18n/🟦️component.tsx` | 3 | `hub/…/admin/📦️packages/🟦️typescript/🧪️vitest.config.ts` — `include=["🧪️admin.test.tsx"]`, `includeSource=["../../🧱️elements/📚️I18n/🟦️component.tsx"]` (**no overlap**) | **in-gate, correctly single-counted** — the one sibling config that gets this right |
| `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/{🗺️spatial,🧱️brepjs,📐️geometry}/🟦️component.ts` (5,30,44) | — | referenced by `📐️cad/📦️packages/🟦️typescript/🧪️vitest.config.ts`'s `DOMAIN_FILES` via `"../../🔨️modules/🌐️spatial-kernel/…"` | **nominally in-gate, actually broken** — see below |
| `✏️s/🔌️plugins/📐️cad/…/🧬️schema/💡️inferences/🟦️component.ts` | 21 | same `DOMAIN_FILES` array | same breakage |
| `✏️s/🔌️plugins/📐️cad/…/✏️editor/⚙️engine/{📺️renderer,🎰️stately,🏃️runtime,🎬️actions,📄️artifact}/🟦️component.ts(x)` (69,4,2,1,77) | — | **not actually reached** — `DOMAIN_FILES` points at `🎛️apps/📐️cad/⚙️engine/…`, a directory tree that **no longer exists**; the real files live at `🗿️artifacts/📐️cad/…/✏️editor/⚙️engine/…` (comment in the config itself even says the artifact `⚙️engine` "has dissolved… into the app `⚙️engine`", but the paths were never updated to match) | **orphaned** — stale paths, silently match 0 files, masked because 4 *other* `DOMAIN_FILES` entries still match something |
| `🗄️stdio/…/📰xml/…/🧬️schema/🟦️component.ts` | 4 | none — `stdio/📦️packages/🟦️typescript/` exists but has no `🧪️vitest.config.ts` at all | **orphaned** |
| `🛂️manifest/🟦️component.ts` | 6 | none — module has no `📦️packages/` at all | **orphaned** |
| window-kits `{🌳️tree,📊️table,🎬️media,🧊️mesh,🖼️image,📄️document}/🟦️component.ts` (1,3,1,1,1,1 = 8) | — | none — no `📦️packages/` anywhere under `🪟️window-kits/` or its `🔌️plugin` parent | **orphaned** |
| `♾️infinite/🖼️canvas/🎨️react-renderer/🟦️component.tsx` | 1 | `…/react-renderer/📦️packages/🟦️typescript/🧪️vitest.config.ts` — `include=includeSource=["index.tsx"]`, but that dir only contains `🟦️glue.tsx`; `passWithNoTests: true` | **orphaned, silently green** — 0 tests actually collected, masked by `passWithNoTests: true` |
| `♾️infinite/🌍️world/🎨️r3f/🟦️component.tsx` | 100 | `…/r3f/📦️packages/🟦️typescript/🧪️vitest.config.ts` — `include=includeSource=["📦️index.tsx"]`, dir only has `🟦️glue.tsx`; `passWithNoTests: true` | **orphaned, silently green — 100 tests, the single largest gap found** |
| `TaskManager/🧪️component.test.tsx` (12), `AgentApprovals/🧪️component.test.tsx` (9), `AgentPresence/🧪️component.test.tsx` (11), `AgentBridge/🧪️component.test.ts` (12) | 44 total | `@semio-tech/framework-renderer-react`'s own `🧪️vitest.config.ts` has no `test.include`/`includeSource` at all (only `coverage.include:["index.tsx"]`); `root` is nested under `📦️packages/🟦️typescript/🎯️targets/⚛️react/`, a sibling of — not an ancestor of — `🧱️elements/`, so even vitest's own default include glob can't reach them | **orphaned** — each file's own header comment already says so and points at manual `vitest run` invocations in earlier packets' reports (`📓️terra-T1-report.md`, `📓️terra-P10-report.md`) |

Two distinct defect shapes, both matching this ticket's "check exists but doesn't run/doesn't mean what it
says" pattern:
1. **Never wired at all** — `🛂️manifest`, `🗄️stdio` schema, all 6 `🪟️window-kits`, the 4 renderer `🧱️elements` test files.
2. **Wired but broken/stale, masked by config**:
   - `♾️infinite` canvas/r3f (`index.tsx`/`📦️index.tsx` don't exist — 101 tests, `passWithNoTests:true` hides it).
   - `📐️cad` plugin `DOMAIN_FILES` half pointing at a directory (`🎛️apps/📐️cad/⚙️engine/…`) that no longer exists (153 tests silently unreached).
   - the 4 valid `📐️cad` `DOMAIN_FILES` entries (spatial-kernel ×3 + inferences) additionally fail to even *load*: `bun node_modules/vitest/vitest.mjs run --config "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️vitest.config.ts" --passWithNoTests` → **exit 1**, `Error: Cannot find package '@semio-tech/kernel-3d-js'`. Ticket `S-MODULES-CRATE-CONSOLIDATION-AND-NAMING-FIX` (2026-08-06) renamed this package to `@semio-tech/s-3d-js` and repointed ~32 plugin `package.json` deps, but the actual `import … from "@semio-tech/kernel-3d-js"` source lines in `spatial-kernel`'s 3 `component.ts` files plus 6 `cad` artifact-editor `component.ts` files were never updated to the new specifier, and `cad-js`'s vitest config has no alias for either name. Net effect: **the entire `@semio-tech/cad-js` vitest project currently exits 1 with zero passing tests**, independent of the stale-path issue above.
   - actor/os/mcp/shell/cad-extensions/animate configs' identical `include`=`includeSource` arrays, doubling every in-source test's count (verified on actor: 58 reported for what should be far fewer real tests).

All of the above are **outside my owned paths** (`✏️s/🔌️plugins/📐️cad/**`, `✏️s/🔨️modules/🌐️spatial-kernel/**`, `🧰️framework/🔨️modules/🛂️manifest/**`, `🗄️stdio/**`, `🪟️window-kits/**`, `♾️infinite/**`, renderer `🧱️elements/**`, and the actor/os/mcp/shell/animate/cad-extension vitest configs) — none were touched, per "fix only what falls inside your owned paths."

## commands + exit codes

All run from `/Users/ueli/Documents/semio` unless noted.

```
$ cd 🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript && bun ./📜️script.ts test
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel
 Test Files  1 passed (1)
      Tests  29 passed (29)
   Duration  546ms
EXIT_CODE:0
```

```
$ bun nx run @semio-tech/framework-kernel:test
> bun ./📜️script.ts test
 Test Files  1 passed (1)
      Tests  29 passed (29)
 NX   Successfully ran target test for project @semio-tech/framework-kernel
NX_EXIT_CODE:0
```
(nx discovered the new project via the existing `🟨️nx-emoji-project-plugin.mjs`, which globs every `📋️project.json` repo-wide by its own `name` field — no root `📋️project.json`/`package.json` edit was needed or made. Confirmed root `package.json`'s `"workspaces"` array also does not list `🎭️actor` itself, so kernel not being listed either is consistent with the closest sibling, not a regression.)

```
$ bun x vitest run --config ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-t1-kernel-vitest.config.ts"
 Test Files  1 passed (1)
      Tests  29 passed (29)
ADHOC_EXIT_CODE:0
```
Same 29 — confirms the new package wires up the identical suite, not a subset.

```
$ sed -n '561,800p' 🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts | shasum -a 256
ddb2ce7f1f8fb21ca2ebf6cb7934261e34e50fcce605455823c69ea19e8136a7
```
Matches the frozen hash given in the brief exactly (before and after all work in this packet).

```
$ bun x tsc --noEmit
(19 × "error TS...", all in ✏️s/🔌️plugins/🔱️trinity/…/🧠️lsp/🟦️component.ts (14), 
 ✏️s/🔌️plugins/🗄️stdio/…/🏗️ifc/…/🟦️component.ts (2), 
 ✏️s/🔌️plugins/🗄️stdio/…/📐️step/…/🟦️component.ts (2), 
 🧰️framework/…/💻️client/🧩️vscode/…/🟦️extension.ts (1))
TSC_EXIT_CODE:2
```
19 pre-existing errors, none in any file this packet touched — matches the brief's own description (trinity, stdio schemas, vscode extension) and the "exit 2" variant the brief said another packet had observed.

Extra verification run for the include/includeSource-doubling finding above:
```
$ cd 🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript && bun ./📜️script.ts test
 Test Files  6 passed (6)
      Tests  58 passed (58)
EXIT_CODE:0
```
(6 test files for 3 real source files — confirms the doubling live on the sibling this packet modeled on.)

```
$ bun node_modules/vitest/vitest.mjs run --config "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️vitest.config.ts" --passWithNoTests
 Test Files  8 failed (8)
      Tests  no tests
Error: Cannot find package '@semio-tech/kernel-3d-js' imported from …
EXIT_CODE:1
```

## frozen region evidence

`//#region 🔖️IoRouter` … `//#endregion 🔖️IoRouter` is at lines 561–800 in the current file (line numbers
unchanged from the brief). `sha256` of that exact range: `ddb2ce7f1f8fb21ca2ebf6cb7934261e34e50fcce605455823c69ea19e8136a7`
— checked once before any work in this packet and again after finishing; identical both times. I never opened
an editor on `🎠️kernel/🟦️component.ts` in write mode, only `Read`/`grep`.

## lease-requests

None. Root `package.json` (`"workspaces"`) and root `📋️project.json` were both read-checked and neither needed
editing: nx project discovery goes through the emoji `project.json` crawler plugin (name-keyed, repo-wide),
independent of bun's `workspaces` array, and the sibling `@semio-tech/framework-actor` itself isn't listed in
`workspaces` either, so kernel's new package.json `dependencies` entry is inert metadata today, same as every
package in this "module-root component.ts + package wrapper" family that isn't itself a workspace member.

## honest gaps

- The `include`/`includeSource` doubling bug is real and verified on the actor sibling (58 tests for 3 files),
  and by inspection affects the *same-shape* configs for `framework-os` (+`backbone-worker.ts`), `framework-os-mcp`,
  `framework-os-shell`, all 4 `cad` extension packages, and `animate`'s artifact-react package — I did not
  run every one of those to get an exact multiplier confirmed per-file (only actor, empirically) and none were
  fixed; all are outside my owned paths. If any of this ticket's exit-checklist numbers for those packages are
  taken from their current `test.include` totals, they may be inflated 2×.
- I did not attempt to fix the `@semio-tech/kernel-3d-js` → `@semio-tech/s-3d-js` rename gap or the `🎛️apps/📐️cad/⚙️engine`
  stale-path gap in `📐️cad/📦️packages/🟦️typescript/🧪️vitest.config.ts` — both are squarely outside my owned
  paths and touch a plugin currently in a broken (exit 1) state; flagging here rather than silently leaving
  it as just a table row, since "the whole cad-js vitest project is red right now" is a bigger deal than a
  routine orphan.
- I did not audit standalone `*.test.ts(x)` files outside the renderer `🧱️elements/` set already named in the
  brief (TaskManager) plus the two more I found by directory listing (AgentApprovals, AgentPresence) and the
  fourth I found only because its own doc comment cross-referenced it (AgentBridge). There may be more
  standalone `.test.ts(x)` files elsewhere in the repo with the same root-scope-mismatch problem; I did not
  grep for every `🧪️component.test.ts(x)` repo-wide, only followed the specific lead the brief gave me plus
  its own internal cross-references. This was a deliberate scope cut to stay within "do not widen scope to
  chase them."
- `@semio-tech/framework-actor` is listed as a `dependencies` entry in the new `package.json` for documentation
  accuracy (kernel's `🟦️component.ts` does import `ShardClient`/`TurnScheduler` from it, by relative path, not
  by that specifier) but is not currently resolvable as an installed workspace dependency, matching the
  pre-existing state of every sibling in this family.
