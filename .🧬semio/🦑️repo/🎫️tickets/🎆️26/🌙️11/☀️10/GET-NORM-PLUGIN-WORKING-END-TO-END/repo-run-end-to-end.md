# repo run @semio-tech/norm-js:test --output=static end to end

Result: 11991 pass, 0 fail, 92343 expects, 50 files (nx target test:prototype). Wall 15m17s (graph load dominates).

## Root cause of nx:authorizers failure
The authorizer compiler bundles Sources into one staged `.semio-authorizer-compiler.mjs`. Eager `createRequire(import.meta.url)("../../../../...managers/.cjs")` calls in the build-graph inference modules (and inputs) were not followed by the bundler; the hoist rewrite only covers an allowlist (`BundledLazyLoadOwners`) of runtime modules. In the staged bundle `import.meta.url` is the staging dir, so the relative path broke. The cjs file exists; the path was right for the source tree.

## Fix
Replaced the relative createRequire with a static `import managersV1 from ".../managers/.cjs"` (resolved by the bundler by source identity) in 7 modules under `library/build`: `inputs`, `graph/inference` (plus its sourceFilesV1 use), `revision`, `component-targets`, `script-inputs`, `dependency-edges`, `project-nodes`. Removed the unused createRequire imports.
Test added: `bootstrap/tools/authorizers/tests/ts` ("sealed authorizer compiler holds no module-relative require ...") compiles the sealed compiler and asserts no relative managers require remains. Needs the repo runner (invocation env); not run standalone.

## pinned-publication
Under bare bun 6 of 12 tests fail with "Original captured runtime entry provider required": requires the repo runner environment, not a real bug.

## Notes
Remaining lazy createRequire("../../admission") calls in dependency-edges/playground-targets are runtime-lazy and unreached in the bundle; left alone.

## Follow-up: artifact packages and "norm loads in s"
- Step 1 `repo run-many --targets=check,test` over the 15 artifact packages (log `🗑️generated/repo-run-1.log`): the graph service cancelled the plan at 2999 of 2999 MB ("ask for a smaller selection") after 12m43s while graph slice verification ran 3824 of 6500 rows, with other agents holding the host near its memory floor.
- Single project `repo run @semio-tech/norm-iso16757:check --output=static` passed (11m10s, `repo-run-2.log`).
- Step 2 `repo run @semio-tech/plugin-registry:generate` (`repo-run-3.log`): even a 1-target plan was cancelled at 3502 of 3502 MB after 42m56s (7 agents contending). No registry, s catalog, semantic-vocabulary or allowlist change was made.
- The s catalog generator is the publication script `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/📜️script.ts` (commands generate, check, preview, verify), the project owning `@semio-tech/s-dev-catalog-ts`.
- Blocker (repo tooling): the graph service plan memory ceiling is hit regardless of selection size when the graph slice must be re-verified under host contention. Needs the graph owner (memory per slice or a persistent warm graph), or an idle host. Steps not yet run: wire-twins, the two s-norm-composition-tests verifies, semantic-vocabulary allowlist resync, catalog-check.
