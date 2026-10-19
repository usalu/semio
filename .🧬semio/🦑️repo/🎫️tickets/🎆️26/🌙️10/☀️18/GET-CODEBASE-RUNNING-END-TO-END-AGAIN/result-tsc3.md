# Result tsc3: TypeScript typecheck at HEAD (2026-10-19)

No `package.json` / `bun.lock` edit, no cargo, no sub-agent, no modifying git command. `bun test` was run once on one explicit file (`vcs/read/tests`, 0 tests, only to load it); never on a directory.

## Method
- 32 `tsconfig*.json` outside node_modules/dist/.cache/target/`.🧬semio`: 31 ours plus `👴️leutwiler/…/.lake/…/proofwidgets/widget/tsconfig.json` (third-party Lean widget, skipped). Runners in `C/tsc-tools/` (`run3.sh` 590 s alarm; `runs.sh` + `chain*.sh` detached, polled from the same turn because os took 14 min at load 70; `NO_BG_NICE` because a niced tsc got 4 percent CPU).
- Command: `node node_modules/typescript/bin/tsc -p <cfg> --noEmit --pretty false --extendedDiagnostics` (5.9.3). `Files` 365 to 23,531 and `Check time` prove the checker ran. Fast scratch configs (`extends` + `files` of the touched sources) were used for iteration. Logs deleted at the end.
- Churn: ~20 peers edited the same trees during the session; some runs failed with `ENFILE` (system file table, caused by a fleet `bun test ./`), re-run later.

## Per config (errors first run -> last run)
| config | first | last | what remains |
| --- | --- | --- | --- |
| hub typescript | 10 | 0 | green |
| hub admin | 48 (tsc, earlier) / 17 | 17 | deps: `@dnd-kit/*`, `react-resizable-panels` (+4 derived TS7006) |
| course package | 13 | 3 | deps only (`@dnd-kit/*`); pet-walk TS2783 fixed after the last run |
| course react (framework products) | 40 | 9 | deps (`@dnd-kit`, `d3-scale`, `aria-query`, `opentype.js`, `jsdom` types), 1 derived |
| os (play) | 277 | 32 to 56 | last stable run 32: `@dnd-kit`, `react-resizable-panels`, `katex`, `jsdom`, `pngjs`, `@webgpu/types`, `@semio-tech/flow-core` (deps) + stale wasm `flow_core.js` (`semioSetRuntimeAssets`); the 56 includes a peer's in-flight ui split (`canvas-pick`, `adapter-exports`, 24 TS2305) |
| framework package | 286 | 61 | ~25 deps/derived; peers in flight: `tool-run` fixtures move, `wire-turn` exports, `🎯️exact/🧪️tests` rewrite, native-storage `loop` tests (xplat-linux), png/image decode types |
| repo | 34 | 42 | build-system new files from other executors (bazel `actions`/`snapshot` tests, `tools/probe`, `build/script.ts(218)` watch change, `store-receipt`/`invocation` tests, `cross-platform-bootstrap` nativePins/windowsLongPaths, `control-plane` strip-ansi) |
| ui react | 21 | 45 | peer's in-flight `canvas-pick` / `adapter-exports` split |
| renderer engine react | 26 | 44 | same split + `wire-turn` |
| window-kits, machine, assets, 3d, locale, intro, compute, ui rust, vscode, coordinator, devcad, 4 cad extensions | 0 each | 0 | green |
| pets | 1 | 1 | `d3-ease` types (deps) |
| pets react | 7 | 7 | vite plugin typing (`OwnedBuildPlugin`), `surface-survey` |
| repo test | 1 | 1 | `smart-buffer` (deps) |
| repo library | 60 | 60 | same as repo |
| plugins `✏️s/tsconfig.json` | 63 | 80 | deps (`brepjs`, `d3-hierarchy`, `d3-geo`, built `@semio-tech/*` outputs, stale `trinity_jack_lsp.js` wasm); fem 2d/3d `schema/diff` + `mutations` are being rewritten by a peer (`GUARDReject`, `ChangeMaterial` placeholders) |
| root `tsconfig.json` | 1885 | 1885 | not meaningful: workspace packages (`@semio-tech/course`, `-react`) are unresolved from the root, 800+ implicit-any follow-ups, plus the print/presentation products that have no package config; each product is covered by its own config above |

## Fixes (root causes)
- Cargo admission migration left half-done across 90 scripts (new provider API `runAdmittedExactCargoLawsV1` needs `control`, loses `artifactDir`/`cargoTargetDir`/`preparation`; `runCargoTestsV1` needs `(request, policy, root, invocation, phaseControl)`): scripted codemod (`scratchpad/migrate-exact.js`, `fix-tests-arity.js`, brace-aware) over framework, plugins, hub, os; imports repaired in 12 files (energy, fem 2d/3d/node/material, puzzle 2d, value derive, snapshot state, fixture-ownership, hub native-catalog); removed phantom `testFeatures`/`snapshotSqliteTestFeatures` options of fem-3d; math rank/dense/replication scripts moved off the repo-lib barrel that no longer exists.
- `scriptInvocationBudget` instead of `number | null` budgets (7 scripts); `testCargo` callers of `framework` script.
- Syntax corruption: `const ;{` (storage reading test), `export export` (vscode test kit); dev script `await import(...).register("ticket-probe")` repaired and `ticket-probe` registered.
- Missing exports/types: `InteractionTarget` (framework barrel), `walkCargoTomlControlledV1` and 5 more typed in the inference `d.mts`, `chunkBytes` in `SourceOwnerLockProtocol` schema, `cargoWorkspaceForManifestControlledV1`, `PayloadUpdate` import, `ReactNode` generic in `Shell`, `ShellHelpers`/`shard-client` moved paths, `tools/sdk` import path.
- Course: tsconfig `paths` for `@semio-tech/course-react/{stylesheet,modules/run/.tsx,modules/results/.tsx}`; `PeerCursor` fixtures gain `place`/`home` in 5 tests.
- Test typing: Ajv type-guard narrowing to `unknown` (`assert(Boolean(validate(x)))`) in 8 files, `Database.run(sql, [..])` array bindings (4 files), fixture literal casts (preflight, graph, runtime-bootstrap, dashboard-control, cross-platform), 5 more.
- Rewritten: `cargo/tests/original-control` against the issued-control API (`withBuildTestInvocationV1`, `createIssuedResourceControlV1`, `CargoPreparationControlV1`); identity-config-facet fold test uses real diffs; `vcs/read` snapshot-source corpus authored (schema + fixture, it never existed; verified with `bun -e` calling `verifyCapturedSnapshotSourceOracle`).
- Removed: `tool-job-coverage/🖼️raster-envelope` (406-line synthetic fixture file, superseded by the neutral corpus `🖨️raster-source.json`: 55 obligations, 206 cases, all driven against the real production sources).
- Config: `lib`/`types` fixes and ambient `d.ts` include from the earlier tsc runs stay.

## Open (not mine / not fixable without the owner)
- Dependencies (manifests frozen): `@dnd-kit/{core,sortable,utilities}`, `react-resizable-panels`, `katex`, `class-variance-authority`, `@tailwindcss/postcss`, `strip-ansi`, `smart-buffer`, `dotenv`, `@webgpu/types`, `brepjs`, `d3-geo`, `@types/{jsdom,pngjs,d3-scale,d3-array,d3-ease,d3-hierarchy,aria-query,opentype.js,graphlib}`; root-level workspace links for the root config.
- Stale generated outputs: wasm-bindgen `flow_core.js` (`semioSetRuntimeAssets`), `trinity_jack_lsp.js` (`JackLspSession`), built `@semio-tech/*` self-imports of 9 sqlite tests.
- In-flight peer work (files modified minutes before the last run): ui `canvas-pick`/`adapter-exports`, `wire-turn`, `tool-run`, `🎯️exact/🧪️tests`, native-storage `loop`, fem schema `diff`/`mutations`, store worker split, bazel/probe tools tests, `build/script.ts` watch. Re-run the os/ui/repo configs once they land; none of these needs a design decision from me.
- `cargo/tests/original-control` was ported and type-checked only; it needs the issued-invocation harness (`bun repo run …test`), not run.
- Not run: any runtime test of the edited files other than the vcs corpus oracle; the edited Rust-launching scripts were only type-checked.
