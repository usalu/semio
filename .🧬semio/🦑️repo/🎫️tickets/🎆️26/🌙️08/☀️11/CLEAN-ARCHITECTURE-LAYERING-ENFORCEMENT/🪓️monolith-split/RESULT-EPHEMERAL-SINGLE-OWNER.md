# One owner for the ephemeral lane (slice EPHEMERAL-SINGLE-OWNER, prefix `eso`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. Raw output is in `🗑️generated/eso-*`. `M` = `🧰️framework/🔨️modules`, `P` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## 0. Verdict

Done. One owner remains: `M/🫧️transient/🟦️.ts` (the kernel store, moved one level up out of `🎠️kernel`, content unchanged). The UI twin, its test and its fixture are deleted. All 16 UI importers, the 4 product importers, the framework barrel, the kernel entry, the nx project, the build-owners row, the wgpu pins, the taxonomy rows, one source-text reader and the launch entries were re-pointed in one pass; no shim, alias or compatibility export is left.

- `ui-no-framework-packages` (UI-scope cruise): 244 to **260** raw, i.e. exactly the 16 new UI to owner edges and nothing else changed (26 new edges, 0 gone). With the one-line policy change of section 2.3 admitted the rule returns to **244**. `no-circular`: 4 to 4 (identical edges).
- Side effect: `no-cross-package-relative-@semio-tech/framework` and `-rs` +5 each (120 to 125), from the 5 UI files that sit outside the framework package owner (theme, vite builder, i18n, appearance, react barrel). A workspace package for the owner removes them (section 7).
- Tests: ui-react vitest failure set identical (same 3 files, same reasons), 1157 of 1157 tests pass (was 1162; the 5 deleted tests are the UI twin test). Kernel vitest 14 files, 91 of 91, identical. Owner `bun test` 5 to 6 pass (new seeded differential). tsc ui-react 50 to 50 errors, 0 new.

## 1. Found state

- Two implementations: the kernel store `M/🎠️kernel/🫧️transient` (`TransientStore` with box, map, set, weakMap and `reset`; the free functions `ephemeralBox/Map/Set/WeakMap` and `defaultTransientStore`) and the UI twin `M/🖱️ui/🎨️styling/🫧️ephemeral-state/🟦️.ts` (box, map, set only, a subset). The twin, its test `M/🖱️ui/🧪️tests/🫧️ephemeral-lane` and its fixture are untracked files from RESULT-UI-BARREL-IMPORTS D1.
- The kernel store is an nx project `@semio-tech/framework-transient` (`📋️project.json`, `📜️script.ts`, no `package.json`), pinned in `📋️build-owners.json`, in the wgpu `📦️packages/🟦️typescript/📋️project.json` (`frameWorkerSources`) and in the taxonomy `generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile.sourceModulePaths`.
- Real importers of the kernel store (measured with `git grep` plus untracked files, `🗑️generated/eso-ts-users.txt`):
  - the kernel package entry `M/🎠️kernel/📦️packages/🟦️typescript/🟦️.ts` (one re-export line) and the subpath file `M/🎠️kernel/📦️packages/🟦️typescript/🫧️transient/🟦️.ts`;
  - four products by relative path: `P/🔍️discovery`, `P/🎮️playground` (through the subpath file), `🖥️server/🎛️coordinator/🪝️webhook/🐙️github`, `🖥️server/📚️library/🗄️persistence`;
  - about 30 plugin, presentation and vscode-client files through the framework barrel `@semio-tech/framework` (unchanged by this slice, it keeps exporting the same names).
- Finding: **no kernel child imports the store.** The kernel only re-exports it from its package entry, which is how the framework barrel (`export *` of the kernel entry) exposes it. So "re-point the kernel children" reduces to dropping that re-export and exporting from the owner in the barrel.

## 2. Design (recorded before the first edit)

### 2.1 What UI may import

Rule `ui-no-framework-packages` (`📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`, rule at line 422):

```
from: ^🧰️framework/🔨️modules/🖱️ui/
to:   ^🧰️framework/  or  ^@semio-tech/framework-
pathNot: ^🧰️framework/🔨️modules/🖱️ui/  and  ^@semio-tech/ui-
```

`pathNot` is matched against the resolved path, so a workspace package named `ui-*` that lives outside `🖱️ui` is not exempt. The taxonomy direction rules add nothing for UI (`framework-neutral` may not import `framework-products`; `io` may not import ui or renderer). Consequence: **no location outside `🖱️ui` exists that UI may import without a policy change**, and every location inside `🖱️ui` would force the kernel, the server products and the plugin side (through the barrel) to depend on UI (a kernel to UI edge, and server code importing a UI folder).

### 2.2 Owner

`M/🫧️transient/` (`🧰️framework/🔨️modules/🫧️transient/🟦️.ts`), the kernel store moved one level up, content unchanged:

- A leaf with no imports (the owner test already bundles it with esbuild and asserts one input file and no imports).
- It sits in `🧰️framework/🔨️modules`, i.e. role `framework-neutral`; nothing in it knows kernel or UI. The kernel never used it, so it does not belong under `🎠️kernel`.
- `🫧️transient` is already the global directory kind `transient` (emoji `🫧️`, slug `^transient$`), so the taxonomy needs no new row; the emoji is unused among the siblings of `🔨️modules`.
- It stays the nx project `@semio-tech/framework-transient` (same name, new root), so launch entries and cache keys keep their names.
- UI imports the leaf by relative path. A workspace package would remove the cross-package-relative findings, but it needs a root `package.json` workspace entry, a hand-edited `bun.lock`, a build-owners package row and a `bun install` on every machine; I did not do that under a running peer fleet (follow-up in section 7).
- The framework barrel exports the owner directly (`export { TransientStore, defaultTransientStore, ephemeralBox, ephemeralMap, ephemeralSet, ephemeralWeakMap, type EphemeralBox } from "../../🔨️modules/🫧️transient/🟦️.ts"`); the kernel entry and its subpath file lose their re-exports (no alias left).

### 2.3 Policy change the parent must admit (one line, not applied by me)

File `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`, rule `ui-no-framework-packages` (line 428):

```diff
-        pathNot: ["^🧰️framework/🔨️modules/🖱️ui/", "^@semio-tech/ui-"],
+        pathNot: ["^🧰️framework/🔨️modules/🖱️ui/", "^@semio-tech/ui-", "^🧰️framework/🔨️modules/🫧️transient/"],
```

Until it is admitted the 16 UI importers show up as 16 `ui-no-framework-packages` edges (244 to 260 raw). Section 5 reports the raw number and the number with the line applied.

### 2.4 Edits planned

1. Move `M/🎠️kernel/🫧️transient/*` to `M/🫧️transient/*` (`mv`, no git); fix `📜️script.ts` relative imports (one `../` less), project `cwd` fields.
2. Re-point: barrel, kernel entry (drop line), delete the kernel subpath file, `P/🔍️discovery`, `P/🎮️playground`, github webhook, server persistence.
3. UI: 16 files import the owner; drop the `ephemeralBox/Map/Set` export from the `@semio-tech/ui-styling` entry; delete the twin folder, the `🧪️tests/🫧️ephemeral-lane` test, its fixture and its entry in the ui-react vitest config; drop `🫧️ephemeral-state` from taxonomy `members-of-styling`.
4. Pins: `📋️build-owners.json` (root and manifest), wgpu `frameWorkerSources`, taxonomy `sourceModulePaths` (byte-sorted position), the coordinator input-closure test that asserts the old path, `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (entry name only).
5. Test: the owner test keeps its `lru-cache` reference model and fixture; the seeded random-script differential of the deleted UI test is ported into it (it is the only part of that test the owner test did not already cover).

## 3. What changed

Moved (`mv`, sha256 identical before the edits below): `M/🎠️kernel/🫧️transient/{📋️project.json,📜️script.ts,🟦️.ts,🧪️tests/🟦️.ts,🧪️tests/🧫️fixtures/🔣️.json}` to `M/🫧️transient/`. Edits inside the moved files: `📜️script.ts` four relative imports (`../../🏃️process` to `../🏃️process`), `📋️project.json` two `cwd` values, `🧪️tests/🟦️.ts` +36 lines (the seeded differential).

Deleted: `M/🎠️kernel/📦️packages/🟦️typescript/🫧️transient/🟦️.ts` (kernel subpath re-export), `M/🖱️ui/🎨️styling/🫧️ephemeral-state/🟦️.ts` (twin), `M/🖱️ui/🧪️tests/🫧️ephemeral-lane/🟦️.ts` (twin test), `M/🖱️ui/🧪️tests/🧫️fixtures/🫧️ephemeral-lane/🔣️.json`.

Re-pointed:

| Area | Files |
| --- | --- |
| framework barrel | `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` exports the owner directly (+1 line); `M/🎠️kernel/📦️packages/🟦️typescript/🟦️.ts` drops its re-export (-1 line) |
| products (relative) | `P/🔍️discovery`, `P/🎮️playground` (was through the kernel subpath file), `🖥️server/🎛️coordinator/🪝️webhook/🐙️github`, `🖥️server/📚️library/🗄️persistence` |
| UI, 16 files | `🎯️targets/⚛️react/{🟦️.tsx,🌐️i18n,🌓️appearance}`, `🎨️styling/{🌓️theme,🏗️builder/🌐️vite}`, `🔨️modules/{🕹️control-keybinding-context,🗣️ui-terminology,🛟️chrome-safe-area,🧲️panel-dock}`, `🧱️elements/{🌈️Surface,🌳️Tree,🎨️Canvas,🎬️Scene,🐚️ShellScope,🔣️Icons,🚗️UiDriver}`; each now imports the owner by relative path, the other `@semio-tech/ui-styling` names stay (tool: `eso-tools/repoint_ui.py`, guarded: exactly one matching import per file) |
| UI package entry | `🎨️styling/📦️packages/🟦️typescript/🟦️.ts` no longer exports `ephemeralBox/Map/Set/EphemeralBox` |
| UI test config | `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` lost the `🫧️ephemeral-lane` entry |
| plugin and presentation consumers (about 30 files) | unchanged: they import the same names from `@semio-tech/framework` |

Pins and readers:

- `📋️build-owners.json`: `@semio-tech/framework-transient` root and manifest.
- wgpu `📦️packages/🟦️typescript/📋️project.json` `frameWorkerSources`: old kernel path removed, new path inserted in byte order (after `🪪️identity/…`). The `🎠️kernel/*/🟦️.ts` glob there no longer matches the store, hence the explicit new line.
- Taxonomy `generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile.sourceModulePaths`: same swap, list still byte-sorted (209 entries, checked in Python). The lone `inputPatterns` list never contained the store (the ownership test only requires browser-transport targets there), so it is unchanged.
- Taxonomy `members-of-styling`: `🫧️ephemeral-state` removed. The new folder needs no row: `🫧️transient` is the global directory kind `transient`.
- Source-text reader: `P/🖥️server/🎛️coordinator/🏗️builder/📥️inputs/🧪️tests/🟦️.ts` asserted `path.includes("/🎠️kernel/🫧️transient/")` on the captured source closure; it now asserts `"/🔨️modules/🫧️transient/"`. (Not run: the test needs the launcher.)
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: entry name `…🎠️kernel🫧️transient🧬️contract` to `…🫧️transient🧬️contract` (it now matches its sibling `…🫧️transient📦️retained`); command, `order` and group untouched.
- Stale-path search after the edits (`git grep` outside tickets plus untracked files under `🧰️framework` and `.vscode`): 0 hits for the old kernel path, the kernel subpath file, `ephemeral-state` and `ephemeral-lane`.

Test change: the owner test keeps its `lru-cache` `ReferenceStore` oracle and its 10-case fixture unchanged. The seeded random-script differential of the deleted UI test (4 seeds, 400 steps, box/map/set identity and size against the `lru-cache` reference) was ported into it, because the owner test did not cover random sequences. Function-init and isolation cases were already covered by the fixture cases `function-init`, `retained-function-init` and `isolation`.

Behaviour change to be aware of: UI keys now live in the process-wide `defaultTransientStore` instead of a private lane. Safe because all 38 UI keys start with `framework.modules.ui.` and no non-UI file uses that prefix (`git grep`), and `defaultTransientStore.reset()` has no production caller (RESULT-UI-BARREL-IMPORTS D1).

## 4. Verification (baseline taken before the first edit)

| Check | Command | Before | After |
| --- | --- | --- | --- |
| Cruiser, UI scope | `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework/🔨️modules/🖱️ui --do-not-follow '^(?!🧰️framework/🔨️modules/🖱️ui/)' --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err` | exit 186, 698 violations; `ui-no-framework-packages` **244**, `no-circular` **4** | exit 212, 724 violations; `ui-no-framework-packages` **260**, `no-circular` **4** |
| Edge diff | normalized (ASCII skeleton, because the cruiser's piped output has mangled multibyte characters, as RESULT-UI-BARREL-IMPORTS notes) | | 26 new, 0 gone; all 26 end at `🧰️framework/🔨️modules/🫧️transient/🟦️.ts`: 16 `ui-no-framework-packages`, 5 `…/framework`, 5 `…/framework-rs`; the 4 `no-circular` edges are the same |
| Policy line applied | the 16 leaf edges removed from the rule's count | | 260 - 16 = **244** |
| ui-react vitest | `bun uib-tools/🧪️run-vitest/📜️script.ts 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript ../../🧪️tests/🎚️config/🟦️.ts <log>` | 46 files, 3 failed (`overlay-flow`, `ShellScope` component, `ShellSearch` component); 1162 of 1162 tests | 45 files, the same 3 failed with the same reasons; 1157 of 1157 tests (-1 file, -5 tests: the deleted twin test) |
| Kernel vitest | `zsh ikt-tools/vitest.sh kernel` | 14 files, 91 of 91 | 14 files, 91 of 91 |
| Owner test | `bun test ./🧪️tests/🟦️.ts` in the owner folder | 5 pass (old path) | 6 pass, 37 expects (new seeded differential) |
| tsc ui-react | `node_modules/.bin/tsc -p 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json --noEmit`; `bun uib-tools/🆚️tsc-diff/📜️script.ts` | 50 errors | 50 errors, new=0, fixed=0; `--listFilesOnly` confirms the owner and the touched elements are in the closure (the vite builder is not) |
| tsc scope | `tsc -p 🗑️generated/eso-tsconfig-scope.json` (28 files: barrel, kernel entry, owner, test and script, the 4 products, theme, vite builder, styling entry, 14 UI importers, a presentation and a vscode importer) | no baseline of this exact set was taken | 2 errors, both `TS2307 '@/lib/event'` and `'@/lib/persistence'` in the github webhook file (lines 18-19, alias imports; my edit is line 13); no error names the owner, `ephemeral*` or a missing module of mine |
| Specifier resolution | `Bun.resolveSync` over every relative specifier naming `🫧️transient` in 30 files | | 22 checked, 0 unresolved |
| wgpu browser closure oracle | `testBrowserInputClosure` (via `uib-tools/drafts/run-closure.ts`) | UI-barrel result: 41 missing, 1 unused (peer reasons) | 41 missing, 1 unused (`🎭️actor/📦️packages/🟦️typescript/🟦️.ts`); neither list mentions the owner, so the owner is observed and listed |
| Taxonomy validation | `validateTaxonomy(JSON.parse(taxonomy.json))` | | 0 problems |
| Launcher | `bun repo run @semio-tech/framework-transient:test-contract --skip-cache` | | It resolved the moved project (selection phases ran), then stopped at `NX Original daemon child exited before acquisition`: infrastructure, not a test result. Not rerun; the direct `bun test` is the evidence |

Logs: `🗑️generated/eso-*` (cruiser before/after and rule counts, tsc, vitest, bun test, closure, scope list).

## 5. Foreign findings

- `P/🖥️server/🎛️coordinator/🪝️webhook/🐙️github` imports `@/lib/event` and `@/lib/persistence` (unresolvable aliases); pre-existing.
- The ESLint override in `🧹️lint/📐️source-policy/🟨️.mjs` bans `class *Store` outside the OS product; `TransientStore` is such a class and was already so in the kernel. The class and its exports were left untouched (renaming would touch the public barrel). The same file's name-based `no-cross-package-relative` heuristic will also warn on the new relative specifiers that name the sibling `🫧️transient` (same WARN level as the existing ones). ESLint was not run.
- Stale prose naming the old place is limited to ticket history files, which I did not touch.

## 6. Hand-offs

1. **Parent, policy**: admit the one-line change of section 2.3 in `🏗️construction/🟨️.cjs`; the construction self-checks and tests of that file may want an entry showing that `🫧️transient` is the only non-UI path UI may import. Until then the cruiser shows 260.
2. Nothing else in the fleet needs to move: the kernel never imported the store, so no kernel child was edited; the kernel entry only lost the re-export.

## 7. Follow-ups (not done, with reasons)

- Promote the owner to a workspace package (`package.json` with `exports["."]`, root `workspaces` entry, `bun.lock` rows, build-owners `packages` row, `bun install`). UI would then import `@semio-tech/framework-transient`, removing the 10 `no-cross-package-relative` edges and the deep relative specifiers. I did not do it under the running peer fleet because it needs a lockfile and install round and I could not prove it without them.
- Decide whether `TransientStore` should lose its `Store` suffix for the ESLint rule above.

## 8. Tools

`eso-tools/repoint_ui.py` (guarded import rewriter for the 16 UI files). Everything else was inline commands. No `[DEBUG]` logging was added to any source file; the generated scratch scripts (`eso-validate-taxonomy.ts`, `eso-resolve.ts`) and logs live only in `🗑️generated/`.
