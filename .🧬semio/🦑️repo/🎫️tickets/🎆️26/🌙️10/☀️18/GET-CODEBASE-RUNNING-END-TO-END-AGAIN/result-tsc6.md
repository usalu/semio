# Result tsc6: TypeScript typecheck after the apply wave (2026-10-20 11:04 to 12:45)

Executor `tsc6`. No `package.json` / `bun.lock` / `🔣️taxonomy.json` / root `📜️script.ts` edit, no cargo, no sub-agent, no modifying git command. Scripts: `C/tsc6-tools/` (`runs6.sh` one config, `mkoverlay.py` overlay generator, `chain6*.sh`, `wire.sh`). Logs, buildinfo, overlays and the scratch artifact dir under `C/🗑️generated/tsc6-*` were deleted at the end.

## Method
tsc5's method (one config at a time, `node node_modules/typescript/bin/tsc -p <cfg> --noEmit --incremental --pretty false --extendedDiagnostics`, 12 GB heap, three chains in parallel), 30 configs (third-party Lean `proofwidgets` skipped as before). `Files` 368 to 25,000 proves the checker ran.

New trap found and handled: **unsynced workspace packages**. `package.json` (root and `✏️s`) gained 15 workspace entries at 11:04 (apply-wave2 manifest batch), but `node_modules` is still generation `70825028305173c7f252` (05:43). Packages `@semio-tech/{framework-action-argument-resolution,framework-effect,framework-filesystem,framework-hash,framework-interaction,framework-manifest,framework-onboarding,framework-trace,framework-transient,plugin-registry-publication,puzzle-5d-ts,repo-product,repo-server-library,ui,ui-contract}` are not linked, so every rewritten `@semio-tech/framework-manifest/...` style import fails. As-is (no overlay): framework 580 errors (380 TS2307, 149 TS7006 cascade), os 2,021. A real signal needs `bun repo dependencies lock --all` + `sync --all` (queued). To get the signal now, every run uses a generated overlay tsconfig (`extends` the real config, `paths` = the config's own paths + one entry per export of each unlinked workspace package, absolute targets; scratch only, nothing in the repo). Runtime bun tests that import these packages fail until the sync (verified: reading suite 44 fail "Cannot find module '@semio-tech/framework-filesystem/...'").

## Per config: tsc5 final -> tsc6 first overlay run -> tsc6 final (F pass, all 30 re-run after the fixes)
| config | tsc5 | tsc6 first | tsc6 final | remaining |
| --- | ---: | ---: | ---: | --- |
| hub, hub admin | 2, 2 | 0, 0 | 0, 0 | package-projection fixed by a peer |
| course package / react | 0 / 0 | 0 / 0 | 0 / 0 | |
| os (play) | 3 | 13 | 1 | stale `flow_core.js` (no `semioSetRuntimeAssets`, needs cargo + wasm-bindgen) |
| framework package | 12 | 15 | 0 | |
| repo | 48 | 18 | 0 at the F pass; 1 in the last re-run | peer in flight: `📚️library/🔍️discovery/🐹️go-layout/🟦️.ts(3)` import depth (edited 12:37 by someone else; later text looks right) |
| repo library | 50 | 10 | 0 | |
| repo test | 2 | 0 | 0 | |
| ui react | 0 | 2 | 0 | |
| engine, window-kits, locale, ui rust, machine, assets, 3d, intro, compute, vscode, coordinator, devcad, 4 cad extensions, pets, pets react | 0 | 0 | 0 | |
| plugins `✏️s/tsconfig.json` | 25 | 27 | 11 | 10 dist-only packages (`exports` -> `dist/`, need `:build`), 1 stale `trinity_jack_lsp.js` (no `JackLspSession`, needs wasm build) |
| root `tsconfig.json` | 455 | 73 | 21 | 11 plugins (as above) + 3 `@semio-tech/puzzle-wasm` stories/editor (root alias, see tsc5) + `lines-and-columns` types (root config does not map it) + flow_core + trinity + `stdio-docx`/`forms-js` dist-only |

The print d3 dependency errors from tsc5 (334 in the root run) are gone: the d3 `@types` arrived with the dependency sync; `suncalc` and `d3-regression` are untyped and are now declared in `🌿️ambient`.

## Fixed, by root cause
Class A: importers pointing at modules that moved or never exported the member (apply-wave2 / lift codemods).
1. `ScriptInvocation` is declared in `🧭️routing/📥️invocation/🟦️.ts`, the process module imports it but does not export it. Four tests now import the type from its owner: `🖥️server/🎛️coordinator/🏗️builder/🎟️admission/🧪️tests/🟦️.ts`, `🏃️process/📦️artifacts/🐹️go-build/🧪️tests/🟦️.ts`, `📁️filesystem/🛡️durability/🧪️tests/🟦️.ts`, `🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🟦️.ts`.
2. `🛂️manifest/🧬️schema/🟦️.ts`: `export { manifestSchema } from "./🔣️.json"` named a JSON key that does not exist; now `export { default as manifestSchema } ... with { type: "json" }`.
3. Named imports from JSON fixtures (exports codemod produced `import { xFixture as y } from "...json"`): `os/🧪️tests/🪟️visible-surfaces`, `✏️s/.../🖨️raster/.../🎚️config/🧪️tests/🔬️unit`, `♻️mit-bestand/🧺️demonstrator/🧪️tests/🏛️shell-brand` (this one only the root config sees) now use default JSON imports with `with { type: "json" }`.
4. `os/📦️packages/🟦️typescript/.../🎫️local-session/🗄️broker/🧬️schema/🟦️.ts` public entry did not re-export `LOCAL_SESSION_REQUEST_SCHEMA` (used by `🧑‍💻dev/🚀️local-hub/🎫️session/🗄️broker`).
5. `print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/📜️script.ts` now exports `verifyVizChartInferences` (the function lives in the lifted test child; `print-pipeline-verification` imports it from `@semio-tech/print-viz-inference/script`, it never resolved).
6. `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/{🧪️tests/🔬️rust-package-commands,📦️packages/🦀️rust/📜️script.ts}`: lift-fix dropped the argument again; `runScriptProgram(packageRoot)`.
7. `course/🎯️targets/⚛️react/.../🛂️proctor/🟦️.ts`: `HttpRequest`, `HttpResponse`, `HttpTransport` are types, moved into the `export type` list (isolatedModules).
8. 6 files `import type X from "x.json" with { type: "json" }` (TS2857, attributes on a type-only import): attribute removed (library: store, cargo receipt/descriptor x2, graph x2, rust assembly).
Class B: API changed, tests stale.
9. dependency reading suite: custody stub now passes `observed` (`observed:()=>custody.observed()`).
10. Bazel closure fetch: `readExactSourceInventoryV1` namespace parameter cast to its own parameter type (`Parameters<typeof readExactSourceInventoryV1>[1]`).
11. Bazel sources / workspace tests and source-demand test: fixture literals cast to the union / literal types (`reference.plan!`, dangling rows, `BazelUncachedReportV1`, `row.env as unknown as Record`, `"compile"|"freshness"`, `"runtime"|"compile"`, `sourceInputs as Record<string,string>`).
12. source-demand test: the owner-discovery and `relativeScriptInputs` port stubs now answer `prepareSourceAnalysesV1().references` (`{runtime:[],urls:[]}`) and provide `workspacePackageModuleV1` / `workspaceUrlTargetV1`; fixture `🧫️fixtures/🕸️graph/🔣️.json` bindings 0 and 3 name the package specifiers `@semio-tech/framework-manifest/view-state-and-public-invocation` and `/schema` (the importers use them now).
13. `⏲️expiry/🧪️tests/⏱️timers/🟦️.ts`: `row.cancelBeforeCheck===undefined?null:...`.
14. exact Cargo proc-macro test (tsc5 "owner needed", 11 errors): rewritten on `withBuildTestInvocationV1` + `withIssuedLawStorageV1` + `writeOwnerPackageV1`, with positive `buildMilliseconds`/`listMilliseconds`/`lawMilliseconds`, the `invocation` identity check replaced by `invocation.policy` identity (the runner derives a child invocation). Type-checks.
15. `repo/.../💻️client/🔌️mcp/📥️installation/🧪️tests/🟦️.ts`: `oracle as {published,cleaned,closed}`.
Class C: configuration / declarations.
16. `ui/react` tsconfig: `types` gains `vitest/importMeta` (two tests type against `ImportMeta["vitest"]`; the previous green came from an incidental transitive import).
17. New ambient declarations `🧰️framework/📦️packages/🟦️typescript/🌿️ambient/☀️suncalc/🟦️.d.ts` and `.../📈️d3-regression/🟦️.d.ts` (untyped packages, `@types` do not exist; print oracles).
18. `build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.d.mts` (new): types `component` and re-exports `discoverBuildOwnerAdmissionFactsV1` (root config has `allowJs:false` and flagged TS7016).

## Files
Paths relative to the repo root unless prefixed. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/🎟️admission/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/{📦️artifacts/🐹️go-build/🧪️tests/🟦️.ts,🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🟦️.ts,🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/⏱️timers/🟦️.ts}`, `🧰️framework/🔨️modules/📁️filesystem/🛡️durability/🧪️tests/🟦️.ts`, `🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/{📦️packages/🟦️typescript/🔨️modules/📇️directory/🎫️local-session/🗄️broker/🧬️schema/🟦️.ts,🧪️tests/🪟️visible-surfaces/🟦️.ts}`
- `L/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/🟦️.ts`, `L/🏗️build/🔧️tools/🏗️bazel/{🔗️closure/🟦️.ts,🧪️tests/📥️sources/🟦️.ts,🧪️tests/📥️workspace/🟦️.ts}`, `L/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts`, `L/🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json`, `L/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.d.mts`
- type-only JSON import attribute removed: `L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts`, `L/⚡️caching/🦀️cargo/📦️units/{📤️receipt,📥️descriptor/🎯️selection/📥️configuration,📥️descriptor/📖️read}/🟦️.ts`, `L/🏗️build/🕸️graph/🟦️.ts`, `L/🏗️build/🕸️graph/🦀️rust/🧩️assembly/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📥️installation/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript/🔨️modules/🛂️proctor/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json`
- `🧰️framework/📦️packages/🟦️typescript/🌿️ambient/{☀️suncalc,📈️d3-regression}/🟦️.d.ts` (new)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/{🧪️tests/🔬️rust-package-commands/🟦️.ts,📦️packages/🦀️rust/📜️script.ts}`, `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧪️tests/🔬️unit/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🏛️shell-brand/🟦️.ts`
- ticket tooling: `C/tsc6-tools/*`, this report.

## Verification
- 30 configs ran twice with the overlay (first run, then the F pass after the fixes). Final: every config 0 errors except os 1, plugins 11, root 21, repo 0 (one run showed 1 peer error, see table). Intermediate repo re-runs: 18 -> 0, repolib 10 -> 0, uireact 2 -> 0, framework 15 -> 0.
- `bun test` with a hand-made process envelope (`C/tsc6-tools/wire.sh` exported as `SEMIO_SCRIPT_PROCESS_INVOCATION`, `SEMIO_TEST_ARTIFACT_DIR` in the ticket): bazel sources + workspace tests 13 pass, 2 skip, 0 fail; source-demand test 19 of 20 pass and the previously failing "declared generated compile outputs" test passes (`-t`); the remaining one ("semantic runtime values retain their canonical leaf binding") needs the sync (the package specifier resolves to `undefined` until `@semio-tech/framework-manifest` is linked) and was not re-run green.
- Exact Cargo proc-macro test: runs up to the planner; fails there because `cargo metadata` needs the repository launcher ("Original compiler ingress requires the repository launcher and authentic enforcing store"). Not run green; verify under `bun repo run`.
- Reading suite and everything importing `@semio-tech/framework-filesystem` etc. cannot run before the sync (44 fails on module resolution).
- Not run: any Vite boot, any cargo.

## Open
1. **Lock + sync** (`bun repo dependencies lock --all`, `sync --all`) to link the 15 new workspace packages; then re-run `C/tsc6-tools/chain6*.sh` without the overlay (or drop `mkoverlay.py` from `runs6.sh`) to confirm the same zero counts.
2. Peer in flight: `📚️library/🔍️discovery/🐹️go-layout/🟦️.ts` (edited 12:37 by someone else), `💻️client/🔌️mcp` (untracked `📥️installation/` being created); re-run the repo config once they settle.
3. Stale generated outputs (need cargo + wasm-bindgen): os `🌊️flow/🎁️browser-package/.../🕸️bindings/flow_core.js`, plugins `🔱️trinity/.../trinity_jack_lsp.js`.
4. Dist-only packages (10 plugin sqlite/consumer tests, `stdio-docx`): `:build` first or point the tests at source.
5. Root-config-only gaps: `@semio-tech/puzzle-wasm` alias (3 files), `lines-and-columns` types; unchanged from tsc5 (an alias in the root tsconfig would be honoured by bun at runtime).
6. Source-demand "canonical leaf binding" test and the exact Cargo proc-macro test: confirm after sync / under the launcher.
