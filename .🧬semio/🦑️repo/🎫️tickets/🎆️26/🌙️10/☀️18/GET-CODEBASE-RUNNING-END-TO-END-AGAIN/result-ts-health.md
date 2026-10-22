# result ts-health (wave 1022a, executor ts-health)

## Method / infra findings
- `bun repo run <p>:typecheck --exclude-dependencies` passes the capacity gate, then dies in graph inference: "Captured require declaring module required" (importer `⚡️caching/🦀️cargo/📜️script.ts`, via `🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:302`). Same infra class course-rest saw; owner = graph/loader executors. Running a package `📜️script.ts typecheck` directly is refused ("Original parent Script process invocation required"). Every repo command also queues for minutes on the dependency `build` resource.
- Fallback (as tsc5/tsc6): `node node_modules/typescript/bin/tsc -p <cfg> --noEmit --incremental --pretty false` via `C/ts-health-tools/run1.sh <cfg> <tag>` (logs `C/🗑️generated/ts-health-run/logs/<tag>.txt|.done`), the same config the target uses (framework: package tsconfig; os: `os/tsconfig.json`; s-dev-composition: `✏️s/tsconfig.json`; others package-local). Workspace packages are linked now, no overlay needed. Single-file checks: `C/ts-health-tools/subset.py <cfg> <tag> <files...>`.
- Closure (project.json dependsOn + package.json deps; `C/ts-health-tools/closure.py`): 78 projects, 10 with a `typecheck` target: framework, framework-os, repo-lib, pets, pets-react, ui-react, framework-renderer-react, course-react, s-dev-composition, architecture-course (skipped: course-rest owns it). Other TS projects of the closure with a tsconfig but no typecheck target: assets, framework-3d-js, machine, repo-coordinator, repo-test, ui-rs, workspace (root).
- bun-types 1.4.3 has strict matchers (`toBe(expected: T)` with a `NoInfer` overload): JSON-fixture literal-union vs string mismatches were TS2769 in ~100 tests; fixed per site with `expect<unknown>(...)` / `expect<string>(...)` (script `C/ts-health-tools/widen_expect.py`, no `any`).
- ui-react: two vitest installs (+1fb5.. root vs +8484.. package) made `import("vitest")` and `vitest/importMeta` incompatible; fixed with a `paths` entry in the package tsconfig.

## Table (final direct-tsc run 04:02-04:04, incremental)
| project | tsc config | status |
| --- | --- | --- |
| @semio-tech/pets | package | 0 errors |
| @semio-tech/pets-react | package | 0 errors |
| @semio-tech/course-react | package | 0 errors |
| @semio-tech/framework-renderer-react | package | 0 errors |
| @semio-tech/ui-react | package | 0 errors (tsconfig `paths` for vitest) |
| @semio-tech/framework | package | 0 errors (45 at start) |
| @semio-tech/repo-lib | package | 0 errors (345 at start) |
| @semio-tech/framework-os | os/tsconfig.json | 1 error, environmental: stale generated `🌊️flow/🎁️browser-package/🕸️bindings/flow_core.js` (Oct 12) lacks `semioSetRuntimeAssets`; needs the flow wasm build (cargo + wasm-bindgen) |
| @semio-tech/s-dev-composition | ✏️s/tsconfig.json | 11 errors, all environmental: 10 TS2307 for `@semio-tech/<artifact>` imports that resolve to unbuilt `dist/` (writer-writer, gis-gisterrain, gis-gismap-js, lowpoly-lowpoly x2, reasoning-wires, forms-js, space-home, space-space, stdio-docx; the target's dependsOn builds them) + TS2614 stale wasm pkg `trinity_jack_lsp.js` (no `JackLspSession`) |
| @teaching/architecture-course | - | skipped (course-rest) |
| extra, no typecheck target: assets, framework-3d-js, machine, repo-coordinator, repo-test, ui-rs | package | 0 errors each |
| extra: workspace (root tsconfig) | - | not run |

Real `bun repo run @semio-tech/framework:typecheck --exclude-dependencies` (03:34, pid 72351, log `C/🗑️generated/ts-health-run/framework-repo2.log`) got past the earlier loader fault but queued 30+ min on the exclusive `repository-graph-facts shard:7` lease behind a peer `bun --eval` (pid 72404, 9 GB RSS, 30 min); I killed my waiting process. Not verified through the repo CLI.

## Fixed (live edits, by root cause)
Library = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, M = `🧰️framework/🔨️modules`.
1. Declarations missing for runtime exports: `⚡️caching/📥️inference/🟨️.d.mts` (`commandUrlReferences`), `🏗️build/👁️observation/🟨️.d.mts` (`inferenceObservationBlockPathV1`), `🏗️build/🕸️graph/🔁retry/🟨️.d.mts` (new file, the module had none), `🏗️build/📥️inputs/🧮️digest/🟨️.d.mts` (`sourceClosureTaskEnvironments` returned `unknown`), `…/🧾️closures/📡️session/🟨️.d.mts` (startup witness texts typed), `🏗️build/🕸️graph/📥️inference/🟨️.d.mts` (`componentBuildTargets` + `ComponentBuildTargetV1`).
2. Production API drift: `compileNxAuthorizerSnapshotV1` 5th argument in `⚡️caching/🦀️cargo/📦️units/🏗️compiler/🔏️code/🟦️.ts`; `createRequire` import in `…/📦️sdk/📥️source/🧊️pristine/🟦️.ts`; reset ancestry `queued lease` owner + async `withFile` in `…/♻️reset/🟦️.ts`; duplicate `requireIssuedResourceControlDescendantV1` (two peers) in `M/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` (kept the schema-governed one); `ImportMeta` fields for captured sources in `🏗️build/📥️source/🔏️closure/🟦️.ts`; `pending.push([...child])` in `🏗️build/📥️caller/📤️child/🟦️.ts`; `createBuildBootstrapCapabilitiesV1` return type inferred (was `object`); `bazelOtherModeOutputV1` weak-type; `NativeRepoRecoveryPlanV1` reset target literal; toolchain classifier `row.command` narrowing; native extract table typing.
3. tool-setup (langs-pin): explicit schema-derived `ToolPinV1/ToolLocateV1/ToolSetupKindV1/ToolRowV1` instead of JSON-widened unions (37 errors gone).
4. Stale tests: missing `verify` on store fakes, `prepare` on management ports, `ScriptInvocation` import owner, duplicate `node:path` imports (12 files), fixture JSON unions (`Record<string,string|undefined>` helpers), callback shadowing the JSON fixture in `♻️scoped` (real bug), `ControlDescendant`, `nativeFs` readonly assignment, etc.
5. `{…}.d.mts` signature additions only describe existing runtime exports.

## Open / hand-over
- Run the real `bun repo run <p>:typecheck` for all 10 targets once the graph fault above is fixed (esp. s-dev-composition, whose dependsOn builds the dist-only packages and the puzzle wasm, and framework-os, whose flow bindings need the wasm build).
- Debug line `[DEBUG] graph-retry` in `🏗️build/🕸️graph/🟦️.ts` (peer, not mine) still writes to stderr under `SEMIO_BUILD_TRACE_AUTHORITY`.
