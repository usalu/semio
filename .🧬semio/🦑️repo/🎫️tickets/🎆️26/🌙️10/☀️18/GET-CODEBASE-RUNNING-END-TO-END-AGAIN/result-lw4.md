# Result lw4: WP4 hub to framework (layering)

Executor `lw4`. Source area `🌎️hub/**`. No `package.json`/`bun.lock` edit, nothing under the frozen storage/revision paths, no cargo, no git-modifying command, no compat barrel or re-export.

## 1. Totals (scope: edges whose source is under `🌎️hub/`)

Same dependency-cruiser policy as `result-lay0.md`; scoped run (`lw4-tools/cruise.sh`: the 257 hub source files as roots, `--do-not-follow '^(🧰️framework|✏️s|♻️mit-bestand)/'`, about 2 min). The `before` run reproduced the audit exactly: 713 violations, 141 files.

| rule | before | after |
|---|---:|---:|
| no-cross-package-relative | 324 | 323 |
| no-cross-technology | 389 | 240 |
| **total** | **713** | **563** |
| source files with a violation | 141 | 133 |

Edge view (a relative import to another technology that is also a deep package import fires both rules): 471 unique import edges before, 323 after; 164 import specifiers in 66 files rewritten.

## 2. Change

Every hub import that reaches a framework file already published by an `exports` entry of the owning workspace package is now the bare package specifier (`@semio-tech/process/routing`, `/budget`, `/execution`, `/compiler-capacity`, `/native-build`, `/test-budget`, `/cargo-laws`, `/leases`, `@semio-tech/framework-os`, `@semio-tech/framework-os-dev/activation`, `@semio-tech/framework-compiler-ts/syntax/module-tree`). The type checker proved that the entry re-exports the identical declarations of every imported symbol (alias-resolved, `lay6` method); bare imports are `aliased-workspace` for the cruiser, which clears `no-cross-technology` and `no-cross-package-relative` for the edge.

Changed files: `lw4-tools/changed-files.txt` (66 files under `🌎️hub/{🎛️commands,🏗️bootstrap,📇️directory,🔐️auth,🚀️local-bootstrap,🚀️local-relay,🧩️compositions,🧪️testing,🧪️tests}`).

## 3. Remaining 563: blocked on missing `exports` entries (manifest freeze)

All 323 remaining import edges point at a framework file that no package publishes. Spec in `lw4-tools/exports-needed.json` (lay6 format, 90 entries):

| package | new entries | importer edges |
|---|---:|---:|
| `@semio-tech/repo-lib` (exports today: only `./package.json`) | 62 | 268 |
| `@semio-tech/framework-plugin` (0 exports) | 19 | 43 |
| `@semio-tech/plugin-registry` | 7 | 10 |
| `@semio-tech/framework-renderer-wgpu` | 1 | 1 |
| `@semio-tech/process` (`🧪️testing/📥️invocation`, namespace use) | 1 | 1 |

Most used repo-lib subpaths: `./process/cargo/admission` (50 edges), `./process/cargo/exact` (27), `./process` (24), `./process/cargo/run` (11), `./caching/artifacts/native-orchestration/invocation` (11), `./process/cargo/dependencies` (10). Seven entries are namespace/side-effect/dynamic uses (symbols `*`), so the glue file must export the whole surface. The later batch must add the manifest key plus the explicit re-export glue file (lay6 convention), then re-run `lw4-tools/fix.ts` in `apply` mode.

Not done on purpose: hub has no workspace dependency list for these packages (bare imports resolve through the existing `🌎️hub/node_modules/@semio-tech/*` symlinks; the manifest batch should list them). No direction inversion was needed: hub never reaches into a lower layer through a wrong direction, the whole remainder is public-entry hygiene.

## 4. Verification

All from `/Users/ueli/Documents/semio`, foreground.
- `lw4-tools/cruise.sh <out>` before and after, `bun lw4-tools/agg.ts <out>` for the totals above.
- Resolution: `Bun.resolveSync(replacement, dirname(importer))` for all 164 replacements: 0 unresolved.
- tsc: `bun lw4-tools/tsc-hub.ts <hub file list> <out>` (TypeScript API, root `tsconfig.json` options, program of 3,612 files incl. 277 hub files, all hub diagnostics counted): 0 diagnostics after (the rewritten imports typecheck with the same symbols).
- `bun test` (SEMIO_TEST_ARTIFACT_DIR in the ticket area): `🧩️compositions/🗄️stdio/🧩️component/🏗️compiler/♻️capture/🧪️tests` 3 pass; `🧪️tests/🧱️foundation-source` 15 pass, 1 fail (`read-only current Dev actors use the actual owner and honor cancellation without acquisition`): the failure is `withBuildTestInvocationV1` throwing "Original parent Script process invocation required" because the test is not launched through the repo script envelope, before reaching the rewritten import. `🧪️tests/🏷️build-freshness` and `catalog-contract-and-flow-decode` register 0 tests under bare `bun test` (script entries).
- The remaining hub tests need live servers, cargo builds or browsers (build ownership belongs to the coordinator) and were not run.

## 5. Open
- 90 export entries (section 3) for the manifest batch.
- `foundation-source` actor test must be run through the repo test runner to confirm it is green.

Ticket tools kept (inputs): `lw4-tools/{cruise.sh,agg.ts,fix.ts,pkgs.ts,tsc-hub.ts,exports-needed.json,changed-files.txt}`; generated output deleted.
