# Result tsc4: TypeScript typecheck after lay13 / glue / fixups / split-fw / split-s (2026-10-19, 17:40 to 18:20)

No `package.json` / `bun.lock` edit, no cargo, no sub-agent, no modifying git command. Tools: `C/tsc-tools/runs4.sh`, `chain4a|b|c.sh`, `C/tsc4-tools/intersect.ts` (intersects diagnostics with `lay13-tools/ncpr-changed.txt`). Logs and buildinfo under `C/🗑️generated/tsc4-*` were deleted at the end.

## Method
Same as tsc3: one config at a time, `node node_modules/typescript/bin/tsc -p <cfg> --noEmit --pretty false --extendedDiagnostics`, plus `--incremental --tsBuildInfoFile C/🗑️generated/tsc4-info/<tag>.tsbuildinfo` (5.9.3, 12 GB heap, two chains in parallel at load 20 to 30). 31 configs (the third-party Lean `proofwidgets` tsconfig skipped as in tsc3). `Files` 365 to 23,554 proves the checker ran. After the fixes below the four configs that contained fixed files (framework, os, repo, plugins) and ui react were re-run; the diff first run to second run confirms each fix (errors gone, nothing new from my edits).

## Per config: tsc3 final -> tsc4 first run -> tsc4 after fixes
| config | tsc3 | now | after fixes | note |
| --- | ---: | ---: | ---: | --- |
| hub typescript | 0 | 2 | 2 | 2 x `creatorPending` (below) |
| hub admin | 17 | 19 | 19 | +2 `creatorPending` |
| course package | 3 | 5 | 5 | +2 `creatorPending` |
| course react | 9 | 9 | 9 | deps only |
| os (play) | 32 (to 56) | 36 | 43 | 7 more from peers' ui resident reader refactor (in flight, below); my 2 fixes removed 2 |
| framework package | 61 | 87 | 86 | 6 wire-turn fixed; +5 `leases` 4-argument tests (peer, below) |
| repo | 42 | 45 | 47 | `leases` tests +2, bazel `operations` test +1 (peers in flight), my tool-job fix -1 |
| repo library | 60 | 39 | not re-run | no fixed file inside |
| repo test | 1 | 2 | not re-run | `creatorPending` replaces `smart-buffer` |
| ui react | 45 | 23 | 23 | deps + `creatorPending`; the canvas-pick split landed |
| renderer engine react | 44 | 22 | not re-run | deps |
| window-kits | 0 | 2 | not re-run | `creatorPending` |
| locale | 0 | 2 | not re-run | `creatorPending` |
| ui rust | 0 | 2 | not re-run | `creatorPending` |
| machine, assets, 3d, intro, compute, vscode, coordinator, devcad, 4 cad extensions | 0 | 0 | 0 | green |
| pets | 1 | 1 | 1 | `d3-ease` types |
| pets react | 7 | 7 | 7 | vite plugin typing, `surface-survey` |
| plugins `✏️s/tsconfig.json` | 80 | 83 | 85 | my fixes -3, peers' resident reader +5 |
| root `tsconfig.json` | 1885 | 1908 | not re-run | not meaningful (see tsc3) |

## Findings by cause
Errors attributable to the lay13 rewrites, the glue files or the splits: **3 distinct root causes, all fixed** (below). Everything else new is either a missing dependency (unchanged set from tsc3) or a concurrent peer's in-flight edit.

### Fixed (new, caused by lay13 / split-fw / earlier moves)
1. lay13 rewrite changed `await import("../../🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts?raw")` into the package entry `@semio-tech/framework-renderer-wgpu/host-io`. The test reads `default` (the file source as a string via Vite `?raw`), a namespace has none: TS2339 and a runtime break. Reverted that single import to the `?raw` form: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔁️refresh-coalescing/🟦️.ts`. A diff scan of all changed `.ts/.tsx/.mts/.mjs` against HEAD found no other `?raw|?url|?worker|?inline` import turned into a bare specifier (the only other `?raw` removal is in the deleted learner-journey test).
2. lay13 moved `toolJobFactoryModuleFile` from `🏭️factory` to `📋️inventory` but its helper `toolJobArtifactCrateRoot` (only used by that function) stayed in factory: TS2304 in os, repo and plugins. Moved the helper (private, with its docstring) next to its only user: `…/📚️library/⚖️laws/🎯️tool-job-coverage/{🏭️factory,📋️inventory}/🟦️.ts`.
3. split-fw split `wire-turn/🟦️.ts` (typed-operation symbols now live in `🔖️turn-result/🟦️.ts`), two tests still imported them from the root: 6 TS2305/TS2459. Re-pointed both imports to the owner: `…/🎭️actor/🖼️wire-turn/⚠️diagnostic/🧪️tests/🟦️.ts` and `…/📬️typed-operation/📥️receiving/🧪️tests/🟦️.ts`.
4. (pre-existing, found on the way) both fem generator scripts imported `runRepositoryCargoTests` from the non-existent `…/📚️library/🟦️.ts`; now the existing `…/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts` entry that every hub script uses: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/📜️script.ts`.

### Glue (result-glue) and split-s: no new error. Missing named exports in glue: none found. `bun`-bundled-only rewrites of lay13: the intersection of all diagnostics with the 407 rewritten files shows only item 1 and item 2 (the other intersect hits were pre-existing peer errors in the same files: dashboard-control `known` typing, the `flow-core` / `jsdom` dependencies).

### Not fixed (not mine, files edited by others)
- `creatorPending` (2 errors in every config that reaches it, 12 configs): `…/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts` lines 224 and 227 read `scope.creatorPending`, but the native-storage owner's in-flight rewrite (file modified 16:30, 55 insertions / 166 deletions against HEAD) removed the field from `FixedPreparation` and the creator logic moved; `📥️allocation/🧪️tests` still expects it. Same rewrite owns the other native-storage test errors (`nativeFixedVhdFooterV1`, `NativeFixedPublicationSystemV1`, `nativeFixedWindows*V1` not exported from `🧱️fixed`, directory/host/journal test arities, `Windows` schema member). Needs the owner; removing the two reads would change their semantics.
- ui resident reader (`UiResidentReaderRetirementSeam`, `UiResidentPayloadReaderSeam` vs `#state`, 4 + 5 errors in os/framework/plugins): `…/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts` edited 18:03 by a peer, appeared between my two runs.
- `leases` tests `Expected 4 arguments, but got 3` (5 errors): `…/🏃️process/🔒️leases/🟦️.ts` edited 17:52 (lay13 open item 2, injected checker with four test call sites); bazel `operations` test imports `BazelTaskV1` from the bazel `📜️script.ts` that declares it locally (script edited 17:58).
- Carried over from tsc3, unchanged: dependencies (`@dnd-kit/*`, `react-resizable-panels`, `katex`, `class-variance-authority`, `@tailwindcss/postcss`, `strip-ansi`, `smart-buffer`, `dotenv`, `@webgpu/types`, `brepjs`, `d3-geo`, `@types/{jsdom,pngjs,d3-*,aria-query,opentype.js,graphlib}`, `check-error`, `color-convert`); stale wasm outputs (`flow_core.js` `semioSetRuntimeAssets`, `trinity_jack_lsp.js` `JackLspSession`, `@semio-tech/flow-core`); 9 built-output self-import sqlite tests; fem 2d/3d `schema/diff` (`parseBinary64` takes one argument, `GUARDReject`, `GUARDMember`) and `mutations` (`Change*` placeholders); `🎯️exact/🧪️tests` (undefined `ScriptInvocation`, `runExactCargoLaws`, … already missing at HEAD); repo build tests (`bazel/actions`, `snapshot`, `probe`, `store-receipt`, `invocation`, `cross-platform-bootstrap` `nativePins`/`windowsLongPaths`, `build/script.ts(218)`); puzzle 5d catalog test fixture typing; pets react vite plugin typing.

## Verification
- Full 31-config run, then re-runs of framework, os, repo, plugins (all incremental) and ui react; diffs first/second run quoted above. Not run: any bun/vitest test of the edited files (type-check only); the `?raw` revert was not exercised under Vite.
