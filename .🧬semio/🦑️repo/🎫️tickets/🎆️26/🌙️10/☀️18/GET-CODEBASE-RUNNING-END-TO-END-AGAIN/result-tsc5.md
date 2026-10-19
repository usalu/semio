# Result tsc5: TypeScript typecheck after the dependency sync went green (2026-10-19 23:00 to 2026-10-20 00:40)

Executor `tsc5`. No `package.json` / `bun.lock` / `🔣️taxonomy.json` / root `📜️script.ts` edit, nothing under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/` edited (HARD FREEZE respected), no cargo, no sub-agent, no modifying git command. Scripts: `C/tsc5-tools/runs5.sh` (one config), `chain5a..f.sh`. Logs, buildinfo and the scratch `tsc5-mit` tsconfig under `C/🗑️generated/tsc5-*` were deleted at the end.

## Method
tsc4's method: one config at a time, `node node_modules/typescript/bin/tsc -p <cfg> --noEmit --incremental --tsBuildInfoFile <generated> --pretty false --extendedDiagnostics`, 12 GB heap, two chains in parallel (load 50 to 100). 30 configs ran (the 31st, the third-party Lean `proofwidgets` tsconfig, is skipped as in tsc3/tsc4). `Files` 365 to 24,478 proves the checker ran.

Two masking traps found and avoided: (1) a program with a syntactic or global diagnostic (`TS6053` missing root file, `TS1005` syntax error) reports only those and skips every semantic check, so a 1 or 8 error count can hide dozens (os first run: 1 error and a file renamed under the glob; plugins second run: my own typo). Every final number below comes from a run with no such error. (2) The node_modules symlinks differ per project (`node_modules` -> `9e8ff88…`, `✏️s/node_modules` -> `a8e26b3…`, `♻️mit-bestand/node_modules` -> `08a6a4d…`); a root-config run resolves everything from the root generation only.

## Per config: tsc4 -> now (tsc5 first run) -> after fix (tsc5 last run)
| config | tsc4 | now | after fix | remaining (class) |
| --- | ---: | ---: | ---: | --- |
| hub typescript | 2 | 3 | 2 | frozen library `package-projection` |
| hub admin | 19 | 2 | 2 | frozen library `package-projection` |
| course package | 5 | 0 | 0 | |
| course react | 9 | 0 | 0 | |
| os (play) | 43 | 1 (TS6053, semantic skipped) | 3 | 1 stale wasm output, 2 frozen library |
| framework package | 86 | 21 | 12 | 11 exact-cargo proc-macro test (open), 1 peer in flight (`⏲️expiry` timers test) |
| repo | 47 | 51 | 48 | all 48 frozen library |
| repo library | 39 | 50 | 50 (not re-run, no library edit) | all frozen library |
| repo test | 2 | 2 | 2 | frozen library `package-projection` |
| ui react | 23 | 0 | 0 | |
| renderer engine react | 22 | 0 | 0 | |
| window-kits | 2 | 0 | 0 | |
| locale | 2 | 0 | 0 | |
| ui rust | 2 | 0 | 0 | |
| machine, assets, 3d, intro, compute, vscode, coordinator, devcad, 4 cad extensions | 0 | 0 | 0 | green |
| pets | 1 | 0 | 0 | |
| pets react | 7 | 4 | 0 | |
| plugins `✏️s/tsconfig.json` | 85 | 43 | 25 | 11 peer gis `document-open` split in flight, 10 dist-only packages, 1 `d3-geo` types, 1 stale wasm output, 2 frozen library |
| root `tsconfig.json` | 1908 | 662 | 455 | 334 print + 9 presentation + 10 mit-bestand (`d3-force` / `reveal.js` types), 48 frozen library, 11 peer gis, 11 exact-cargo, rest stale outputs / dist-only (below) |

All packages that were "missing" in tsc3/tsc4 now resolve for the 30 project configs except the ones listed under "Open: dependencies". The `creatorPending` (12 configs), ui resident reader, `smart-buffer`, `@dnd-kit/*`, `katex`, `class-variance-authority`, `d3-ease`, `@types/{jsdom,pngjs,aria-query,opentype.js,graphlib}`, `brepjs`, `@webgpu/types`, `strip-ansi` and the pets vite plugin groups from tsc4 are gone (dependency sync or the owners' rewrites landed).

## Fixed (outside the frozen paths), by root cause
Class A: modules whose body moved into `🟦️.ts` while an importer still pointed at the retired `📜️script.ts`, or members moved to another module.
1. `🧰️framework/🔨️modules/🎒️pack/🌱️value/📦️packages/🟦️typescript/📜️script.ts`: `../../📜️script.ts` -> `../../🟦️.ts` (hub, framework).
2. `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/⌨️native-entrypoint/📜️script.ts` and `…/📜️script.ts` (`runNativeBinary`): re-export / import `nativeRunnerEnvironment`, `runNativeBinary` from `⌨️native-entrypoint/🟦️.ts` (repo, os).
3. `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📜️script.ts`: `actorNetworkData`, `validateActorNetwork*` come from `actor-network/🟦️.ts`, only `ActorNetworkScript` from its `📜️script.ts`.
4. `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/♻️activation/🟦️.ts`: `ActivationComponentSpec` now lives in `activation-vite-plugin/🧬️options/🟦️.ts`.
5. `🧰️framework/🔨️modules/🏃️process/💾️state/🧪️tests/🟦️.ts`: removed the call to `testNativeMetadataSourceV1`, whose module (image metadata test) was deleted with the VHD/image backend.
6. Mis-extracted script bodies: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/` (`runScriptProgram(packageRoot)` takes the root its script passes) and `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/` (`runCanonicalArchitectureScript({contract, check})` receives the two steps instead of referencing classes of the other file).
Class B: API changed, callers stale.
7. `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/📦️packages/🦀️rust/📜️script.ts`: `commands` / `testCommands` are `ScriptRouter` instances (were object literals).
8. `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/{🔒️resource-leases,🎟️writer}/🟦️.ts`: `retainResourceQueueMetadataHeadroomV1` takes `requireResourceLeaseCustodyV1` as 4th argument (5 call sites).
9. `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/…/🧬️schema/🔺️diff/🟦️.ts`: `parseBinary64(value)` takes one argument (9 call sites); 3d `GUARDReject` / `GUARDMember` -> `femFem3dDiffGuardReject` / `femFem3dDiffGuardMember`.
10. `✏️s/…/◻️2d/…/🧬️mutations/🟦️.ts`: the nine `Change*` mutation interfaces the union referenced were never declared; added from the Rust shapes (`ChangeNode|Material|Section|Support|Region|Element` `{id, patch}`, `ChangeLoad {caseId, id, patch}`, `ChangeCombinationName`, `ChangeCombinationTerm`).
11. `🧰️framework/🛍️products/🖥️server/🧪️tests/🎚️config/🟦️.ts` and `♻️mit-bestand/…/33.projektetage/🧪️tests/🎚️config/🟦️.ts`: vitest 4 has no `test.mode` (default is already `test`), removed.
12. `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts`: `hostBuildPlugins` returns `(OwnedBuildPlugin & Plugin)[]` (was `Plugin[]`, not assignable back into an owned config; pets react 4 -> 0). `♻️mit-bestand/…/🎚️config/🟦️.ts` wraps `semioAssetsVitePlugin` in `hostBuildPlugins` for the vitest config.
13. `✏️s/🔌️plugins/🖍️draw/…/🔤️text/🧪️tests/🟦️.ts`: dropped `{hinting:false}` (opentype 1.3.4 `getPath` default is no hinting, `RenderOptions` has no such member). `✏️s/…/🌀️procedural/…/🧊️generation3d/…/🚀️original-field/🧪️tests/🟦️.ts`: `graphlib.alg.preorder(graph,[member])` (types want `string[]`, runtime accepts both).
14. `🧰️framework/🛍️products/💻️os/🧪️tests/🔁️execution-target-retry/🟦️.ts`: cockatiel 3.2.1 `IBackoff.next(context)` takes one argument.
15. `🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🟦️.d.ts`: `declare module "check-error"` (`getMessage`); no `@types` exists.
16. `🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/🌐️browser-host/🟨️.d.ts` (new): types of the `🟨️.js` host shim the jcoprobe-callback test imports.
Class C: typing of fixtures and shadowing.
17. `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧪️tests/🔬️rust-package-commands/🟦️.ts`: the `consume` callback parameter shadowed the function's `context`, so `context.repoRoot` read the callback object; renamed it `issued`.
18. `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧊️execution/🚀️launch/🟦️.ts` (`expect<unknown>`), `…/🎯️exact/🧪️tests/📦️executable/🟦️.ts` (`row.kinds as string[]`), `✏️s/…/🖐️5d/…/🗂️catalog/🧪️tests/🟦️.ts` (`row.weights as Readonly<Record<string, number>>`), `🧰️framework/🧪️tests/🧱️ownership/🟦️.ts` (`role as string`).
19. `🧪️tests/🧪️diff-only-law-gate/🟦️.ts`: imports `policyHostApplyFileBreaches` (used but never imported); needs the library barrel patch below.
20. Native-build custody test `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/🟦️.ts` ("compiler custody runs after leases…"): rewritten onto the current API (`withBuildStorageV1` + `lawBuildOptionsV1`, `commandPort` with compiler/toolchain identity, custody refusal / cancellation injected through the planning port because `beforeCompile` no longer exists). Type-checks; not run (needs the fixed image storage).
Class D: the mit-bestand product (one of the four target products), visible only to the root config because it has no own tsconfig; 160 -> 10 errors in the root run (9 `d3-force` types in actor-network, 1 `reveal.js`).
21. Deck test kit `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🧪️projektetage-deck/{🟦️.ts,🧰️deck-test-kit/🟦️.ts}`: the kit's `Presentation`, `Thought` and `dependencies` were `any`, which made every `.find((chapter) => …)` an implicit any (145 errors); typed from `@semio-tech/presentation` and the spec module (`ProjektetageDeckDependencies`).
22. `…/33.projektetage/📦️packages/🟦️typescript/📦️index.ts`: `addZukunftBauTitleMorph(slide: Slide): Slide` (the `| undefined` leaked into `slides`), `📜️script.ts`: `runViteBunxDev` now requires `runtimeChannels: []` (as the course does).
23. Demonstrator shell, the real runtime break: `bootFrameworkOsDev` requires `variant` and `admission`, `createFrameworkOsDevConfig` requires `browserSessionFactories`. `♻️mit-bestand/🧺️demonstrator/🐚️shell/🟦️.ts` now mirrors the puzzle variant entry (`VITE_SEMIO_PLUGIN` variant, one admission shared with the catalog, dispose on pagehide); `🐚️shell/🧬️schema/🔣️.json` carries `browserSessionFactories` (puzzle-2d, as the puzzle variant's contribution does), `🏗️builder/🌐️vite/🐚️shell/🟦️.ts` passes it; `🧪️tests/🧪️demonstratorpanebranding/🟦️.ts` normalises the readonly manifest app into `AppDefinition` shape. Not run (no Vite boot).

## Errors inside the frozen library (to fix after the lift)
Paths relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`. 48 in the repo config (50 in repo library; hub, hub admin, repo test, os, plugins each show only the two `package-projection` ones).
- `🔍️discovery/🎁️package-projection/🟦️.ts:69,85` TS2339 `taxonomy.collisionPolicy.maxPathBytes`: the policy now has `maxPathUtf16Units` and `maxComponentUtf8Bytes` (tax3 ruling); compare `leaf.path.length` with `maxPathUtf16Units` and the UTF-8 component bytes with `maxComponentUtf8Bytes`.
- `📦️packages/🟦️typescript/⚖️laws/🟦️.ts:23`: add `policyHostApplyBreaches, policyHostApplyFileBreaches, policyRowEmitterBreaches` to the export list from `…/🧪️diff-only-law/🟦️.ts` (all three exist there; `🧪️tests/🧪️diff-only-law-gate/🟦️.ts` imports them: 3 + 3 root errors).
- `⚡️caching/📦️artifacts/🟦️typescript/🧪️tests/🧭️commands/📜️script.ts:10` TS2740: an object literal is passed where a `ScriptRouter` is required.
- `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts:274,275`: fixture `string` where the union `"other" | "cache-link-refused"` is required (cast the fixture field).
- `⚡️caching/🚀️bootstrap/🛠️tools/🔁️probe/🟦️.ts:20,21,24,24,26,26,26,26`: 8 implicit-any / unknown callback parameters (`specification`, `inputs`, `target`, `output`, `task`, `fixture`, `environment`).
- `⚡️caching/🚀️bootstrap/🛠️tools/🔁️probe/🧪️tests/🟦️.ts:17,22,30`: `version: number` where the literal `1` is required (`1 as const`).
- `⚡️caching/🚀️bootstrap/🛠️tools/🧪️tests/🟦️.ts:11,58`: imports `../📜️script.ts`, which is now `../🟦️.ts`.
- `⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts:15`: `@semio-tech/nx-bootstrap-tools/script` no longer resolves (the `patch` import at line 31 does).
- `🏗️build/📜️script.ts:218` (2x) TS2322 `Promise<number>` is not assignable to `Promise<void>` (the `runBazelTasksV1` callbacks must return `void`, or the type must say number).
- `🏗️build/🔧️tools/🏗️bazel/⚙️operations/🧪️tests/🟦️.ts:15`: `BazelTaskV1` is declared locally in the bazel `📜️script.ts`, not exported.
- `🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️actions/🟦️.ts:33,33,48,48,60,87`: fixture strings vs `BazelUncachedReasonV1`, `"noop"|"task"|"launch"`, and an `unknown` path.
- `🏗️build/🔧️tools/🏗️bazel/🧪️tests/📦️snapshot/🟦️.ts:25,42,56`: `version` does not exist in `BazelSnapshotContextV1`.
- `🏗️build/🕸️graph/📥️inference/🧪️tests/💾️cache/🟦️.ts:36,47,122`: `number` vs `1`, and tuples that need two elements.
- `🏗️build/🧪️tests/🕸️graph/🟦️.ts:980` (new since tsc4/peer): `Promise<{…}>` with `undefined` where `{dependsOn?: unknown[]}` is required.
- `🏗️build/🧭️invocation/🧪️tests/🟦️.ts:24`: `null` where `string | undefined` is required.
- `🏗️build/🧾️store-receipt/🧪️tests/🟦️.ts:50,96,127`: fixture strings vs `"directory"|"file"` and `"valid"|"malformed"|"invalid"|"unwritten"`.
- `🏗️build/🛠️cmake/🧪️tests/🟦️.ts:34`: `dotenv` not installed (dependency).
- `🧪️tests/🎛️dashboard-control/🟦️.ts:200,334,345`, `🧪️tests/🥾️cross-platform-bootstrap/🟦️.ts:256,333,339` (`nativePins`, `windowsLongPaths` no longer in the policy type), `🧪️tests/🦑️repo-source-ownership/🟦️.ts:137`, `🧪️tests/🧮️path-budget/🟦️.ts:26`: fixture strings vs the new literal unions / missing members.
Most "No overload matches" entries are `expect(x).toEqual/toBe(fixtureString)` against a typed value: cast the fixture field to the union.

## Open: not fixed
Peer in flight (files touched in the last hour by others; re-run after they settle):
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🗺️document-open/**` (11 errors in plugins, 17 in root): the verbatim split of the cold-pair transfer test (23:48) left `DocumentExecutionTargetLease` unimported in two children and `state`, `fields`, `documentId` used before declaration in the parent (`🟦️.ts:76-84`); nullable `probe` fields need narrowing in `↩️undo-owner-phases` and `🚚️transfer-and-render-phases`.
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/⏲️expiry/🧪️tests/⏱️timers/🟦️.ts:18` `row.cancelBeforeCheck` possibly undefined (file edited 00:15).
Owner needed:
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🟦️.ts:38-69,146` (11 errors in framework and root): the "proc-macro exact laws discover controlled compiler runtime" test uses the pre-storage API (no `ScriptInvocation` / `ExactCargoLawPort` / `runExactCargoLaws` / `configuredExactCargoLawPolicyV1` import, no `originalInvocation`, and `ExactCargoLawOptions` now requires `control`, `storage`, `leaseDirectory`, `sourceBoundary`, `manifestPaths`, `CARGO_HOME`, `CARGO_BUILD_BUILD_DIR`, plus a real Cargo project because planning and source admission run before `port.probe`; native library discovery moved to `port.nativeLibraryDirectory`). It needs a rewrite on `withIssuedLawStorageV1` from `📦️artifacts/🏗️native-build/🧪️tests/🦀️cargo/🟦️.ts` like `🍎️image/🟦️.ts`; I could not verify one without running the storage image.
Stale generated outputs (regeneration needs cargo + wasm-bindgen, so listed, not regenerated):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️bindings/flow_core.js` (os, root): no `semioSetRuntimeAssets` (consumer `🎁️browser-package/📦️packages/🦀️rust/🟦️.ts:1`).
- `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust/pkg/trinity_jack_lsp.{js,d.ts}` (August stub, 519 byte wasm): no `JackLspSession`.
- `@semio-tech/puzzle-wasm` unresolved in the root config only (`✏️s/tsconfig.json` maps it to `🌎️hub/…/🦀️rust/pkg/semio_puzzle.d.ts`, which exists); I tried moving the alias to the root tsconfig and reverted it because bun honours `paths` at runtime and would redirect every non-`✏️s` importer to a `.d.ts`.
Packages that resolve only after their build (`exports` point at `dist/🟦️.js`, 10 plugin tests): `@semio-tech/{writer-writer, gis-gisterrain, gis-gismap-js, lowpoly-lowpoly (2), reasoning-wires, forms-js, space-home, space-space, stdio-docx}` (`:build` first, or point the tests at the source module).
Open: dependencies (declared in a `package.json` but absent from the node_modules a config resolves from; deps owner):
- print (`🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/package.json` declares them; 334 root errors, mostly cascading implicit-any): `d3-shape` (27), `d3-interpolate`, `d3-time`, `d3-chord`, `d3-contour`, `d3-hexbin`, `d3-delaunay`, `d3-format`, `d3-time-format`, `d3-sankey`, `d3-random`, `d3-regression`, `d3-quadtree`, `dagre`, `suncalc`, `fft.js`, `@asamuzakjp/css-color`; types for `d3-path`, `d3-polygon`, `d3-geo` (also 1 plugins error), `d3-force` (mit-bestand actor-network, 8 errors).
- presentation react: `unified`, `remark-gfm`, `remark-parse`, `remark-rehype`, `rehype-stringify`; `@types/reveal.js` (declared, not installed; 5 errors).
- `lines-and-columns` types (framework config maps it by path, the root config does not), `dotenv`, `@semio-tech/nx-bootstrap-tools`.
- `♻️mit-bestand/🧺️demonstrator/package.json` and `🏢️semio-tech/🎡️play/package.json` still carry `semio.browserSessionFactories`; the demonstrator now reads the schema JSON like the puzzle variant, so the package.json copy is a duplicate (remove after the lift if no tool reads it).

## Verification
- 30-config run, then targeted re-runs: os x5 (1, 5, 4, 4, 3), framework x3 (21, 11, 12), plugins x4 (43, 8 syntax-masked, 31, 25), repo x2 (51, 48), hub x2 (3, 2), pets react x2 (4, 0), ui react x2 (0, 0), root x2 (662, 455); the mit-bestand slice with a scratch tsconfig (160 -> 10). The diff between runs confirmed each fix (errors gone, none new from my edits); one regression of mine (a stray `)` in the 5d catalog test, TS1005) was caught by the plugins re-run and fixed.
- Not run: any bun/vitest test of the edited files (type-check only); the demonstrator shell entry, the rewritten custody test and the `hostBuildPlugins` return type under a real Vite boot were not exercised.
