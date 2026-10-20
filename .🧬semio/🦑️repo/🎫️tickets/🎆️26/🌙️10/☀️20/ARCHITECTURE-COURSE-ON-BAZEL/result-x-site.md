# result-x-site: course site build, publish, deploy-check, typecheck

`T` = this ticket folder. `S` = `🎓️teaching/🏛️architecture/📚️course`. `PKG` = `S/📦️packages/🟦️typescript`. Tools: `T/x-site-tools/`. Status (16:25): STATIC WORK, TESTS AND STAGING DONE; NO REAL RUN POSSIBLE (bun repo does not boot after the 16:03 kernel panic). Measurements pending the "bun repo boots" line.

## 0. Run status

- 15:23 `bun repo --tool=bazel run @teaching/architecture-course:build` (run 1): 10 m 43 s, 9 m 30 s of it queued on the shared `native-fixed-image:.cache/b/d` lease, then failed with `Exact original parent child allocation required` (`readNativeStorageMountedChildV1`). Same failure as `--dry-run` of the peers and `tools setup bazel`. Log: `T/🗑️generated/x-site/bazel-build-1.log`.
- 15:34 run 2 started, killed at 15:42 on the coordinator's order (no retry loop until coordination.md says FIXED).
- 16:00 run 3: died in 2 s at `ensureBuildEnvironmentV1` ("Bounded regular bootstrap input required", nlink check). After the 16:03 panic the dependency store links are gone too (`bun test` of the deploy suite: `Cannot find package 'ajv'`). Cold/warm measurements are therefore empty; section 7 lists what to run.

## 1. Finding: a warm run is NOT a hit today (measured by code, not by clock)

`buildViteArtifact` (`⚡️caching/🌐️vite/🟦️.ts`) publishes into `.cache/b/p/m/vite/<mode>/<sha256(owner)[0:16]>/out`. The slot is keyed by (mode, owner) only, never by the input digest. `withCompilerState` keys only the Vite dependency-optimizer state (`vite`, `bun`, config text, owner, mode). So every `build` runs `vite build` again; only the optimizer cache is warm. There is no "artifact key covers all inputs, Vite skipped" path. `sourceRevision` is used only by `docker-image-build` and `docker-image-publish`; the site build and `publish` never read it, so it cannot reach the artifact bytes (a test pins this).

## 2. Design (neutral, tool-independent)

A product that lives in a managed, content-addressed store is cached through the store receipt protocol that already exists (`🏗️build/🧾️store-receipt`, `result-store-out.md`): the target declares `{projectRoot}/.🧾️store/build.json` as its only output and `cache: true`. A hit restores the receipt; the backend verifies every receipt entry (size and digest) against the store and treats anything but `valid` as a miss (re-executes that task alone). The slot being overwritten by another `PROCTOR_URL` build is therefore safe: the digest differs, so the hit degrades to one rebuild. The same declaration works for Nx and Bazel; no adapter edit.

- `build`: `cache: true`, `outputs: [receipt]`, no `managed-output` decision (`cache: true` makes the policy class `deterministic`), inputs `default`, `{env: PROCTOR_URL}`, the Vite-bundled external packages, the energy quiz JSONs and the assets package (see 3). `SEMIO_BUILD_MODE` is always hashed by the policy.
- `publish`: stays `cache: false`. The workspace name policy (`⚡️caching/🔣️policy.json`, `uncached: publish`) forbids replay for the name `publish` workspace-wide and that file is not ours. Instead `publish` depends on `build` (a receipt hit) and only verifies and copies the artifact into `dist/release/pages/course`; it no longer runs Vite. It declares `{env: PROCTOR_URL}`; with no variable the origin is derived deterministically from `🚀️deploy/🔣️.json` (`COURSE_PROCTOR_ORIGIN`, the same value the Vite config falls back to).
- `deploy-check`: `dependsOn` `typecheck` and `publish` (so `build`) as graph tasks; the script keeps drift, `checkCourse`, image, stack and the live end-to-end gate. Cached steps, individual progress.
- One site builder: `buildCourseSite(repoRoot, bundleRoot, invocation)` with the fixed owner `architecture-course:site` and the mode of the original caller; `build` and `publish` share the slot, so release `publish` reads exactly the release artifact `build` produced.

## 3. Key completeness (TypeScript-API oracle)

`T/x-site-tools/site-closure.ts` walks the value imports of the Vite entry (`S/🟦️.ts`) and config with the TypeScript module resolver (`ts.preProcessFile` replaced by an AST walk that drops type-only imports) and matches the workspace files against the build inputs with `picomatch`. Result on the live manifest: 608 files reached; the files neither in `default` nor in the router import closure (the inference adds the script closure) are the 5 energy quiz JSONs and the `🖼️assets` package files. External packages bundled or loaded by the build: `@dnd-kit/{core,sortable,utilities}`, `@tailwindcss/vite`, `@vitejs/plugin-react`, `i18next`, `react`, `react-dom`, `react-i18next`, `tailwindcss`, `vite` (the live `externalDependencies` lists none of the runtime ones; Bazel hashes all bundled manifests when any `externalDependencies` entry exists, Nx hashes the named ones).
`PROCTOR_URL` is the only environment variable the site code reads (`S/🟦️.ts` `import.meta.env.VITE_PROCTOR_URL`, Vite config `process.env.PROCTOR_URL`). `dist/wasm/<mode>` snapshots are read by `canonicalWasmBuildVitePlugin` only for packages declaring `semioWasmOwner`; none in the closure, so inert.

## 4. SEMIO_VITE_CACHE_DIRECTORY

Already neutral: it is a `derivedVariable` of the Vite environment protocol (`📥️environment/🧬️schema`), minted by the framework from the bounded state directory and written last into the child environment (`buildViteArtifact`, `viteCommandChildEnvironmentV1`), overriding any ambient value. The course reads it only through `viteCacheDirectory` in the config factory. Tests pin: the factory refuses without it and no course source sets it.

## 5. P2-4 tsBuildInfoFile

`typecheckProject` always passes `--incremental true --tsBuildInfoFile <compiler state>/state.tsbuildinfo` (state root in the products partition, `withCompilerState`), so `tsconfig.json` `tsBuildInfoFile: node_modules/.cache/...` was dead config naming a leased dependency store. Removed (live edit). Test: TypeScript's own `parseJsonConfigFileContent` yields no `tsBuildInfoFile`.

## 6. P2-7 typecheck timeout

`120_000` replaced by `courseTypecheckBudgetMs(invocation)` = `scriptInvocationBudget(invocation, buildBudgetMs())` (default `SEMIO_BUILD_BUDGET_MS`=0 means the caller's remaining deadline). Cold time: not measured yet (section 7).

## 7. Measurements

| run | tool | cold/warm | seconds | note |
|---|---|---|---|---|
| build 1 | bazel | cold | n/a | 643 s, 570 s of it queued on `.cache/b/d`, then "Exact original parent child allocation required" |
| build 3 | bazel | cold | n/a | 2 s, boot failure (nlink) |
| overlay deploy suite (bun test, static) | | | 17.6 s | 33 of 35 pass in a scratch overlay; the 2 failures are overlay artifacts (see 8) |
| live deploy suite (bun test, static) | | | 5 s | 28 of 28 pass before the reboot |
| `site-closure.ts` (TypeScript resolver walk of the site entry and the Vite configuration) | | | 2.7 s | |
| `check-types.ts` on 42 course files | | | 31 s | 0 diagnostics with the whole staged set applied |

To measure once `bun repo` boots (foreground, timeout 600000, one at a time):

1. Before apply, baseline: `bun repo --tool=bazel run @teaching/architecture-course:build` twice (expect two full Vite runs, warm only for the optimizer), then the same with `bun repo run` (Nx).
2. `bun repo --tool=bazel run @teaching/architecture-course:typecheck` cold (also answers P2-7: the old constant was 120 s), then warm.
3. Apply (section 8), then `build` twice per tool: the second run must print no `Bundling` line and report a cache hit; then run `build` with `PROCTOR_URL=https://staging.example`, then without: expect one rebuild per switch (digest mismatch), never a stale artifact.
4. Compare the `outputs` digest of `.🧾️store/build.json` between a clean and a dirty tree: they must be identical (no source revision in the bytes).
5. `publish --configuration=release` (after x-release lands) and `deploy-check`: the progress must show `typecheck`, `build` and `publish` as separate tasks and no second compiler run.

## 8. Files, tests, apply order

Live (done, course code):
- `S/📦️packages/🟦️typescript/tsconfig.json`: dropped the dead `tsBuildInfoFile` (P2-4).
- `S/🏗️builder/⏱️budget/{🟦️.ts,🧬️schema/🔣️.json,🧫️fixtures/🔣️.json}`: `courseBudgetRequestMs`, `courseBudgetMs` = `scriptInvocationBudget(invocation, buildBudgetMs())` (P2-7).
- `S/📦️packages/🟦️typescript/📜️script.ts`: both typecheck calls use `courseBudgetMs(this.invocation)` instead of `120_000`.
- `S/🚀️deploy/🌐️site/🟦️.ts`: `siteFiles` lists in name order (the artifact checks and the CDN staging no longer depend on the file system's listing order; this also fixed the one test that failed on APFS under `bun test`).
- `S/🧪️tests/🧪️deploy/🏗️site-build/🟦️.ts` (+ registered in `S/🧪️tests/🧪️deploy/🟦️.ts`): budget fixture vs Ajv and the process budget schema; tsconfig parsed by the TypeScript API has no `tsBuildInfoFile` and the runner always passes its own; the Vite config refuses a missing or ambient cache directory and no course file sets `SEMIO_VITE_CACHE_DIRECTORY`; the site code never reads `sourceRevision`. Red first (2 failures), then green.

Staged (T/x-site-tools/staged/, coupled to the frozen manifest, applied together): `apply.py` (idempotent, anchored, atomic, `--check`, `--emit-manifest`), `check.sh`, `site-graph.test.ts`, `site-graph.fixture.json`, `site-graph.schema.json`.
- manifest `PKG/📋️project.json`: `build` (cache true, receipt output, no managed-output decision, inputs with `PROCTOR_URL`, bundled packages, energy quiz JSONs, assets package), `publish` (depends on `build`, inputs `default` + `PROCTOR_URL`), `deploy-check` (depends on `publish`, plus the existing `typecheck`).
- `S/🚀️deploy/🌐️site/🟦️.ts`: `COURSE_SITE_OWNER`, `courseSiteBuildPlan`, `buildCourseSite`, `publishCourseSite` without a build callback.
- `S/📦️packages/🟦️typescript/📜️script.ts`: `build`, `publish`, `deploy-check` routes.
- `S/🚀️deploy/⚖️readiness/🟦️.ts`: 6 live steps, `DEPLOY_CHECK_GRAPH_TASKS`, fail-closed manifest check.
- tests: staging test drops the plan test and the `cache:false` pin; sealed-publication steps; new `S/🧪️tests/🧪️deploy/🕸️site-graph/` (+ schema) and `S/🧫️fixtures/🕸️site-graph/🔣️.json`; publication fixture loses `build`.
- `S/README.md`: readiness gate rewritten (soft: the build, publish, deploy-check rows and the cached-targets sentence in English and German).

Tests of the staged set (scratch overlay = private copy of the course tree, symlinks elsewhere; `T/x-site-tools/overlay-test.sh`): 33 of 35 pass. The 2 failures are overlay artifacts: the image test refuses a symlinked ancestor, and (on the overlay run before the sort fix) the CDN-artifact message order. Type check of 42 course files with everything applied: 0 diagnostics. Oracles: Ajv (fixtures), the TypeScript module resolver plus `picomatch` (key completeness: the value-import closure of the site entry and the Vite configuration is covered by the build inputs or the router's own closure, and every bundled package is declared), `typescript` parsing `tsconfig.json`.

Apply order (after the peer freeze lifts; our manifest anchors are the `build`, `publish`, `deploy-check` blocks only):
1. The peers' manifest series first (uncached-owners, x-inputs default-input extension, x-release configurations), because they rewrite the same file and the `build` block (the managed-output decision).
2. `python3 T/x-site-tools/staged/apply.py --check` then `python3 T/x-site-tools/staged/apply.py` (exit 2 and nothing written if a hard anchor drifted).
3. `T/x-site-tools/staged/check.sh live` (deploy tests, type check), then the measurements of section 7.

Open points / risks:
- `publish` cannot be cached: the workspace name policy `uncached: publish` (`⚡️caching/🔣️policy.json`, not ours) fixes it workspace-wide. `build` carries the cost, `publish` only verifies and copies.
- The `build` receipt follows `result-store-out.md`; nothing was run end to end. The hit path (receipt restore, digest check, Vite skipped) is design plus the protocol's own fixture tests, not yet measured.
- `deploy-check` loses the early typecheck-before-build ordering: graph tasks run in parallel before the script. A failing type error and a failing site build now both report before the gate starts.
- Peers edit the README concurrently; the README edits are soft and report drift instead of failing.
- No generic backend edit was made ("Backend edits": none).
