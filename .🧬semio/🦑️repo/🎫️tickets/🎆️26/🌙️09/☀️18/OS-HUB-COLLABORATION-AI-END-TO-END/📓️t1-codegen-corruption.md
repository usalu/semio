# T1 — generated-code corruption, root tsconfig scope, permanent census

Slice T1 of ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`. Input: `📓️audit-typescript-debt.md`
cluster 1 ("68 % of 2608 TS diagnostics are emoji-spliced identifiers in generated `.d.ts`"). All
captures in `🗑️generated/t1-*.txt`; all tools in this folder.

## 1. What the corruption actually is

The predecessor's census (`🐍️t1-corruption-census.ts`, capture `🗑️generated/t1-corruption-before.txt`)
reported **823 occurrences "in code position" across 296 files**. That number is wrong in both
directions and the first thing this slice did was pin down the real set.

Classifying every splice in the repo by the *line shape* it sits on (2033 splices, 407 files):

| line shape | splices | verdict |
|---|---:|---|
| `export type * as …` / `export * as …` in a jco world binding | **792** | real corruption |
| inside a string / template literal | 1020 | the repo's own emoji-separator idiom, not corruption |
| inside a `//` or `/* */` comment | 221 | same |

Every one of the predecessor census's 31 "code position" hits outside the jco bindings is a false
positive: `↔` (U+2194, `Extended_Pictographic`) inside JSDoc prose (`guest↔host`, `persp↔ortho`,
`snake_case↔camelCase`), emoji-labelled paths inside comments, and the census script quoting its own
example. Its per-line "am I inside a string?" heuristic cannot see multi-line comments.

The real signature, byte-verified:

```
🧰️framework/…/🧑‍💻dev/🧩️extension-modules/🪟️sourcing-module-windows/semio_s_plugin_sourcing_windows_component.d.ts:3
export type * as WasiCliEnvironmen🔬️t029 from './interfaces/wasi-cli-environment.js'; // import wasi:cli/environment@0.2.9
```

A 🔬️ (U+1F52C U+FE0F) was inserted before every `t0` substring — so only aliases ending `…t029` are
hit, while `WasiCliStderr029` / `WasiIoPoll029` in the same file are intact. Matches the 2026-09-03
rename-plan codemod incident (memory `project-codex-rename-plan-codemod-incident`).

**Blast radius is type-declarations only.** In every affected directory the runtime `.js`, the
`.core.wasm`, `🌉️bridge.js`, `🟨️.js` and `📥️install.json` are clean (checked per file); the
`interfaces/*.d.ts` are clean too. Only the top-level world binding `*_component.d.ts` was corrupted,
so nothing booted differently — it only broke `tsc`.

## 2. Why this was regenerated, not re-transpiled

A raw `jco transpile` of the current `dist/component-dev/*.wasm` is **not** a safe regeneration. Run
against `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_sourcing_windows.wasm`,
jco emits a *different world* than the staged output carries — versioned interfaces
(`semio:framework/types@1.0.0` → `SemioFrameworkTypes100`, plus `reactor`/`jobs`/`checkpoint`/`describe`
exports) where the staged binding has the unversioned pre-1.0 world (`plugin`/`contributor`). The
staged `.js`/`.core.wasm` came from the older wasm, so transpiling only the `.d.ts` would desynchronise
the directory, and a full restage would need all ~130 plugin wasm components rebuilt plus the
`rewriteJcoAsyncResultLifting` / `rewriteJcoComponentAssetUrls` passes in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`.

So the alias was regenerated **from the source of truth jco prints on the same line** — the
fully-qualified WIT interface id in the trailing `// import <id>` comment — using jco's own naming rule
(`PascalCase(namespace)+PascalCase(package)+PascalCase(interface)+version digits`, dots dropped; bare
camelCase interface name for exports). `🔨️t1-regenerate-jco-aliases.py` validates that rule against
every *intact* generator-written alias in the repo before it writes a byte, and aborts on any
disagreement:

```
intact generator-written aliases that validate the rule: 9072
rule disagreements on intact aliases: 0
files with corrupted aliases: 132  aliases regenerated: 792
```

All 132 files are gitignored build output (`git check-ignore` confirmed one by one — zero tracked
files). Capture: `🗑️generated/t1-regenerate-jco-aliases.txt`. After the run the tool re-checks clean
across a widened scope (any `.d.ts` whose first line is jco's `// world ` header, which also covers the
`jcoprobe.d.ts` bundles): **9946 aliases validated, 0 corrupted**.

## 3. Root tsconfig scoped to source

`tsconfig.json` previously globbed `**/*.ts,**/*.tsx` with only `node_modules`/`temp`/`reports`/`log`
excluded, so every build artefact in the repo was in the root program. Added excludes: `**/dist/**`,
`**/📤️dist/**`, `**/out/**`, `**/coverage/**`, `**/target/**`, `**/🎯️target/**`, `**/.nx/**`,
`storybook-static`, `**/🗑️generated/**`, `**/🤖️generated/**`, `**/⚡️cache/**`, `**/🕸️bindings/**`,
`**/🔌️plugin-modules/**`, `**/🧩️extension-modules/**`, `**/📺️renderer-modules/**`,
`**/🌐️browser-bundles/**`, `.🧬semio/🦑️repo/🎫️tickets/**`.

Two compiler options were also wrong for this repo. `@types/bun@1.4.2` was added to root
`devDependencies` — the root tsconfig sets no `types` array, so the package is auto-included and `Bun`
globals + `bun:*` modules now resolve there (root `TS2868` is 0). And `allowImportingTsExtensions:
true` was added: the repo imports `.ts` paths everywhere (the hub tsconfig already set it, the root one
did not), which alone accounted for **5935 `TS5097`** plus a large cascade of `TS2303`/`TS2459` in the
first full semantic run.

**`bun add` was blocked and had to be unblocked first.** The root `workspaces` array was missing
`🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript` and its `🎯️targets/⚛️react` child, while a
listed workspace (`♻️mit-bestand/🎤️präsentation/📅️33.projektetage/…`) depends on both, so *every*
`bun install` in the repo failed with `Workspace dependency "@semio-tech/presentation" not found`. Both
entries were added to `package.json`; the install then succeeded (85 packages linked) and `bun.lock`
was updated. This is a real repo-wide unblock, not a T1 side effect.

## 4. Four tracked source files that were masking the entire root semantic typecheck

With build output excluded, the root program still reported only 20 diagnostics — every one of them a
**parse** error. `tsc` does not run the semantic pass while syntactic diagnostics exist, so the root
typecheck had been blind for as long as these files were broken. All four were corrupted by commit
`bb961413d4` (2026-09-14) and have been committed-broken since; none is owned by T2/T3/T4.

| file | damage | repair |
|---|---|---|
| `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🎚️config/🟦️.ts:21` | a bare file path injected as a line inside the `alias:` array | line removed |
| `🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts:17` | orphan `}` left by a replaced `visualizationTemplates` body; the same commit also dropped the `getWorkspaceRoot` import the file still calls | brace removed, import restored |
| `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts:60,146,165` | one orphan `}` plus **two function headers deleted, leaving headless bodies** | brace removed; `compileLightAndDark` and `compilePrintDocument` signatures restored verbatim from `78b716e661` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts:16` | orphan `}` after a gutted `BuildScript` class | brace removed |

Only the syntax was repaired. The two gutted modules are **still semantically broken** and are reported
as findings in §7 rather than reconstructed — rebuilding them means guessing at the 09-14 migration's
intent, which is not this slice's call.

## 5. Before / after

### Generated-code corruption

| | before | after |
|---|---:|---:|
| files with spliced jco aliases | 132 | **0** |
| spliced aliases | 792 | **0** |
| jco binding files scanned | — | 400 |
| jco aliases validated against the WIT id | — | 9946 |
| TS/JS files scanned | — | 23158 |

Re-run on the settled tree after every other change in this slice
(`🗑️generated/t1-census-final.txt`): `scanned=23158 jco-bindings=400 jco-aliases-validated=9946`,
`passed.`, exit 0.

### TS diagnostics per project

| project | tsconfig | before | after | parse-class after |
|---|---|---:|---:|---:|
| root | `tsconfig.json` | 3496 | ROOT_AFTER | ROOT_PARSE |
| hub | `🌎️hub/📦️packages/🟦️typescript/tsconfig.json` | 151 | 800 | 0 |
| framework | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/…/tsconfig.json` | (not captured by the audit) | 539 | 0 |
| os renderer | `…/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/…/tsconfig.json` | 865 | 684 | 0 |

Root "before" is the predecessor's `🗑️generated/t1-root-before.txt` (3496 diagnostics, **100 % parse
class**: TS1127 ×1776, TS1434 ×1584, TS1128 ×49, TS1443 ×44, TS1160 ×22, TS1109 ×14, …). Staged
progression, all captured: 3496 → 20 (`t1-tsc-root-after.txt`, tsconfig scoping only) → 2
(`t1-tsc-root-after2.txt`, after three of the four source repairs) → ROOT_AFTER
(`t1-tsc-root-after3.txt`, after the two restored function headers).

**The hub 151 → 800 rise is not a regression and is not T1 work.** 432 of the 800 come from a single
file, `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, which had **zero** diagnostics in the audit capture and 382
`TS2775` now (`assert.equal(...)` needs an explicitly-typed call target). Eighteen modules the hub
program had never type-checked are now reached, and hub's `TS2868` ("Cannot find name 'Bun'") went
3 → 0. The cause is the same workspace/install unblock from §3: modules that previously failed to
resolve — and therefore silently typed as `any` — now resolve and get checked. Real debt made visible,
owned by T2. No hub or library source file was edited by this slice (`find -newermt '-45 minutes'`
showed no live peer edits during the measurement, so the delta is attributable to node_modules, not to
a concurrent edit).

## 6. The permanent gate

`bun ./📜️script.ts verify generated-corruption` — new `//#region 🔖️GeneratedCorruptionCensus` in the
root `📜️script.ts` (~150 lines) plus `VerifyScript.runGeneratedCorruption`. Two checks, **neither a
heuristic**:

1. **jco alias derivation.** Every `.d.ts` whose first line is `// world ` has each world alias
   re-derived from its WIT id and compared. Exact.
2. **Spliced emoji in code position.** A file is a candidate only if the splice regex matches; the
   candidate is then handed to the repo's own TypeScript 5.9.3 parser (`ts.createSourceFile`, resolved
   via `createRequire(join(root, "package.json"))` so a stray global TypeScript 7 cannot be picked up),
   and a splice is reported **only where a parse diagnostic lands at that exact offset**. This is what
   separates corruption from the repo's legitimate emoji idiom: `🖱️ui⚛️react` in a string, `guest↔host`
   in a comment, `/🎆️26🌙️06☀️04📊️metric/` in a regex and `<div>🖱️ui⚛️react</div>` in JSX all parse
   cleanly and are silent.

The first hand-rolled attempt at check 2 (blank comments/strings/regexes myself, then scan) produced
27 false positives and was thrown away — its scanner desynchronised on a template literal around
`ShellHost/🟦️.tsx:1069` and mis-read every backtick after it. That is why the parser does the work.

Wiring, following the existing idiom for `verify-package-purity`:
- `📋️project.json` → target `verify-generated-corruption` (`nx:run-commands`, `forwardAllArgs`).
- `.vscode/🧩️launch.seed.jsonc` → row `📦️check🤖️generated🔬️corruption`, group `4_build`, order 206.164,
  appended after H1's os-hub rows (the seed is the source of truth; launch.json is generated and must
  never be hand-edited). `.vscode/launch.json` regenerated with
  `bun 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate`
  (60 plugin crates, 65 playgrounds, 54 framework packages; 288 configurations, valid JSON), and the
  freshness gate then passes: `… check-generated` → *"plugin registry generated catalog and launch
  bytes are fresh."*, exit 0 (`🗑️generated/t1-registry-generate.txt`,
  `🗑️generated/t1-launch-freshness-check.txt`). It had to be run through the bundle script rather than
  `bun nx run @semio-tech/plugin-registry:generate`, which died on `NX The daemon timed out while
  processing REQUEST_PROJECT_GRAPH`.

**Negative control (`🗑️generated/t1-census-negative-control.txt`).** Two deliberate corruptions were
injected — one jco alias (`WasiCliExit029` → `WasiCliExi🔬️t029`) and one fresh `.ts` with
`export const bad🔬️Name = 1;` — and the gate fired on both and exited non-zero:

```
jco-alias …/semio_s_plugin_sourcing_windows_component.d.ts:4 alias "WasiCliExi🔬️t029" ≠ "WasiCliExit029" derived from wasi:cli/exit@0.2.9
code-position …/🐍️t1-negative-control.ts:1 spliced 🔬️ in identifier "bad🔬️Name" (TypeScript cannot parse it)
error: [verify generated-corruption] 2 spliced-emoji identifier(s) in 2 file(s)
```

Both were reverted immediately (the `.d.ts` restored from a byte copy, the probe file deleted) and the
gate re-run clean.

**The verb is verified; the nx *invocation* is not, right now.** `bun ./📜️script.ts verify
generated-corruption` was run four times here (baseline, clean, negative control, clean again) and
behaves correctly. `bun nx run workspace:verify-generated-corruption` currently fails before reaching
the executor with `NX The daemon timed out while processing REQUEST_PROJECT_GRAPH`
(`🗑️generated/t1-nx-target-run.txt`). That is an environment condition, not a target defect: the
machine was at load average ≈107 from the concurrent worker fleet, and the peer-standard
`bun nx run @semio-tech/plugin-registry:generate` died at the identical point in the same window. The
target definition is byte-identical in shape to `verify-package-purity`. Re-run it once the fleet
quiets to close this out.

## 7. Honest gaps and hand-offs

- **`📓️print/🖨️tectonic-template-compilation/🟦️.ts` is still gutted.** It now parses, but
  `compilePrintDocument`'s body lost the `spawn(tectonic, …)` invocation and the `searchPaths`
  computation that `78b716e661` had, and gained `if (!shape.requirePdf) return;` referencing an
  undeclared `shape`. `compileLightAndDark` has no caller. The print product cannot compile a document
  in this state. Owner: whoever owns the 09-14 print migration — not reconstructed here.
- **`🦑️repo/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts` is gutted.** `BuildScript` has an
  empty body, `DevScript` and the `router` declaration are gone (the last line still calls
  `runBundleScriptMain(router, …)`), and the value-import line was dropped. Same commit, same call.
- **`framework` and `os renderer` tsconfigs still set `types: ["react","react-dom","vite/client"]`**, so
  `Bun` does not resolve there — `TS2868` ×16 in the framework capture and ×25 in the os renderer
  capture. Adding `"bun"` to each `types` array clears them (`@types/bun` is now installed), but those
  two files belong to T3 and T4; not touched.
- **Hub's 800, framework's 539 and the os renderer's 684 were not reduced by this slice** — out of
  scope by instruction; they are T2/T3/T4 work, and §5 explains why hub's number moved. The os
  renderer's 865 → 684 is T4's concurrent work, not T1's; it is recorded, not claimed.
- The census walks 23 137 files and parses only the ~400 emoji candidates, so it is I/O-bound, not
  parse-bound. It ran in well under two minutes on every invocation here.
- `🐍️t1-corruption-census.ts` (the predecessor's prototype) is left in the folder as the record of the
  first measurement; the permanent implementation is the `📜️script.ts` verb, not that file.

## 8. Files changed

Tracked source and configuration:
- `tsconfig.json` — source-only exclude list, `allowImportingTsExtensions`.
- `package.json` — two missing `🎤️presentation` workspaces, `@types/bun` devDependency.
- `bun.lock` — regenerated by the resulting `bun install`.
- `📜️script.ts` — `//#region 🔖️GeneratedCorruptionCensus` + `verify generated-corruption` dispatch and
  `runGeneratedCorruption`.
- `📋️project.json` — `verify-generated-corruption` target.
- `.vscode/🧩️launch.seed.jsonc` — `📦️check🤖️generated🔬️corruption` row; `.vscode/launch.json` and the
  plugin-registry `🤖️generated` catalog regenerated by `📇️registry/📜️script.ts generate` (the freshness
  gate had flagged launch.json stale, including a peer's four os-hub seed rows that had never been
  regenerated).
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🎚️config/🟦️.ts` — injected path line removed.
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts` — orphan brace, restored import.
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts` — orphan brace, two
  restored function headers.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts` —
  orphan brace.

Generated (gitignored) output: 132 `*_component.d.ts` world bindings under
`💻️os/🔨️modules/🧑‍💻dev/{🧩️extension-modules,🔌️plugin-modules}/`, `storybook-static/plugin-modules/`,
`💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/{dev,release}/` and
`.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/` — 792 aliases regenerated. No `.js`, `.core.wasm`,
`🌉️bridge.js` or `📥️install.json` was touched (mtimes verified).

Ticket folder: `🔨️t1-regenerate-jco-aliases.py`, `🐍️t1-parse-probe.ts`, this report, and captures
`🗑️generated/t1-{corruption-before,regenerate-jco-aliases,census-after,census-negative-control,tsc-root-after,tsc-root-after2,tsc-root-after3,tsc-root-after4,tsc-hub-after,tsc-framework-after,tsc-os-renderer-after,census-final,nx-target-run,registry-generate,launch-freshness-check}.txt`.
