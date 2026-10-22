# result-demo-libs (executor demo-libs, wave 1022a) - PRE-VALIDATED DIRECT, REPO RUNS OPEN (v3 generation pid 29009 still cold at 06:07)

Tools (`C/demo-libs-tools/`): `run.sh <name> run <target>` (nice, `SEMIO_BUILD_GRAPH_POLICY=current`, logs in `C/🗑️generated/demo-libs/`), `retry.sh` (loops on repo-CLI faults; killed 05:12 after the coordinator killed my run), `imports.py`, `resolve.mjs`, `tscheck.mjs`, `bericht-bundle.mjs`, `watch.sh`.

`♻️mit-bestand` holds 3 repo projects. Two kinds of evidence below: VIA REPO (the real `bun repo run`) and PRE-VALIDATION (direct vitest / bun test / bun scripts on the same sources, because the repo graph was down 02:35-05:12; direct runs are NOT proof of the repo target).

## Status table
| project:target | via repo | pre-validation (direct) | notes |
|---|---|---|---|
| demonstrator:test (vitest quick+long, includes canonical-architecture shell-brand) | NOT RUN (graph) | GREEN 6 files / 61 tests (long) | 2 root-cause fixes below |
| demonstrator:canonical-architecture | NOT RUN | GREEN (inside the long run) | |
| demonstrator:test-runtime-catalog | NOT RUN | GREEN `bun test ./…/🧩️runtime/🧪️tests/🟦️.ts` 1 pass | |
| demonstrator `🧩️engine-producers` bun test | n/a | GREEN 1 pass | |
| bericht:test (quick) | NOT RUN (needs `build`, slice-demo) | GREEN (scratch harness running the real TestScript.run quick) | 3 stale-test fixes below |
| bericht:test (long/exhaustive PDF.js part) | NOT RUN | NOT RUN (needs published PDFs) | code rewritten to published print artifacts, WRITTEN BUT UNVERIFIED |
| bericht:akteursnetz-validate | NOT RUN | GREEN (798 nodes, 444 edges, 14 programs) | |
| bericht:check-generated | NOT RUN | GREEN (fragments current) | |
| praesentation-projektetage:test (quick) | NOT RUN | GREEN 1 file / 28 tests | |
| praesentation-projektetage:canonical-architecture (long) | NOT RUN | GREEN 2 files / 35 tests | |
| static: relative imports / bare specifiers / TS export check of all 3 projects | n/a | GREEN (100 files; 122 pairs; only bun-types noise) | |

## Repo-CLI blockers seen (not mine, for the record)
5 `bun repo run …demonstrator:test` runs 02:35-05:10 died in the repo CLI before reaching the target: `Captured require declaring module required` (cargo script; fixed by loader-own 03:22), `Uncaptured implementation import refused` routing barrel, `Authenticated captured build command entry required`, then attempt 3 held graph-facts exclusive ~1 h building an obsolete-format generation (killed by the coordinator 05:10). Re-run after the v3 generation (pid 29009) is sealed.

## Changed files (root causes)
1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json`, `…/🏗️build/🧬️schema/🔣️.json`, `…/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json`: `ReadyDeclaration.anyOf` items now carry `properties:{port|portEnv:true}` next to `required`; Ajv `strict:true` (strictRequired) refused the schema, so the demonstrator E2E-source-scope test and every strict compile of the owner contract failed. All three copies stay identical (`🟢️readiness/🧪️tests` 18 pass).
2. `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorcompileclosure/🎭️e2e/🧫️cases.json`: `dependencies.build` gained the two site artifact targets (`semio-framework-os-flow-browser-package:wasm`, `semio-framework-canvas:stage-runtime-assets`) that project.json and `pipeline.site.artifactTargets` declare.
3. `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📜️script.ts`: build outputs asserted at `{workspaceRoot}/.cache/b/p/m/print/{options.env.SEMIO_BUILD_MODE}/documents/report/<id>` (was `{projectRoot}/dist/documents/<id>`); PDF.js check reads the published print artifact via `withPrintExecutionV1`/`withPublishedPrintDocumentV1`/`readPrintMaterialFileV1` (as `CompleteScript`) instead of `dist/documents`; import of `createIssuedResourceControlV1`.
4. `♻️mit-bestand/📋️bericht/🧪️tests/🧫️fixtures/🧭️commands.json`: `maxSourceFiles` 13 -> 24 (measured 22: the routing barrel closure grew), documents command externals `@semio-tech/print`,`@semio-tech/process` (pdfjs-dist/sharp moved into the print library).

## Open
- Re-run every target above via `bun repo run` once the v3 generation is sealed.
- Direct non-framework dependencies (plugins in `✏️s`) are covered by apps-a / apps-b; `✏️s/🧑‍💻dev` projects (s-dev-catalog-ts, s-dev-composition) not yet examined by me.
