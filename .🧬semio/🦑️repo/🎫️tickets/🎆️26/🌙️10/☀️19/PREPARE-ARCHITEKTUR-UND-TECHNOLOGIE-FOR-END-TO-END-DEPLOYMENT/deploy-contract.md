# Runtime Image Deployment Contract

The deployment test suite still required a second Docker compiler stage and a separate Cargo configuration bypass. Those expectations contradicted the current authenticated native producer and finite runtime tar packaging design.

The course deployment tests now check one pinned runtime stage, exactly four authored runtime COPY roles, and the JSON-form proctor content check. They reject compiler commands, source-tree copies and the obsolete Dockerfile.cargo.toml file. The Docker context fixture includes the executable, Dockerfile, empty data directory, selected course catalog and quizzes, and Compose/Caddy files. It explicitly excludes Cargo manifests, lockfiles, toolchain configuration, Rust sources and Cargo configuration. Existing picomatch checks evaluate those language-neutral inclusion vectors against the actual Docker ignore rules; the actual bounded content selector must select exactly the fixture's content roster.

The old TOML import, Cargo configuration type and builder-bypass assertions were removed. Remaining test literals were normalized from architecture-quiz to architecture-course and QUIZ_PAGES_DIRECTORY to COURSE_PAGES_DIRECTORY, preserving their existing drift/release assertions. The README deployment directory table now describes the runtime-only recipe and bounded tar encoder, consistent with the existing manual runbook.

## Initial Validation

```powershell
node node_modules/vitest/vitest.mjs run --config '🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎚️config/🟦️.ts' '🧪️tests/🧪️deploy/🟦️.ts' --reporter=verbose
```

Exit 1, zero tests executed: suite collection fails because `@semio-tech/repo-lib/cargo` cannot resolve from `🎓️teaching/🛂️proctor/🏗️bootstrap/🕸️manifest-closure/🟦️.ts:19`. This is an import blocker, not a semantic failing baseline for the revised runtime assertions. Output is retained temporarily at `🗑️generated/deploy-contract-red.log`.

Root agent owns the production deployment module, Dockerfile, Docker ignore rules, removal of Dockerfile.cargo.toml, and the Vitest alias repair. Semantic red/green verification will follow those shared repairs.

## Completed Deployment Verification

After public workspace alias resolution and the shared production repairs, the focused deployment suite executed23 tests. Its first runnable semantic result was22 passed and1 failed: the workflow test still expected old bare Bun consumer commands and an extra site-local test command, while the current workflow has a privileged finite preflight/dependency/publication sequence and already depends on the full check job.

The neutral readiness fixture now records exact `siteCommands` and `proctorSteps`. Assertions compare the YAML-decoded actual workflow to those lists, including preflight, owned dependency sync, image build, pinned registry login and image publish. Existing check-job tests/typecheck/browser-rehearsal gates, dependency relationships and least-privilege assertions remain enforced.

The subsequent selected deploy run exited0: one test file,23 passed tests. It completed in13.56s, with1.25s executing assertions. All finite context, real content selection, runtime-only Dockerfile, drift, release workflow, CDN publication, CORS-related configuration, bundle and cancellation assertions executed. Diagnostic output is retained temporarily in `🗑️generated/deploy-contract-green.log`; the actual22/23 result is in `deploy-contract-integrated.log`.

The README no longer claims the removed `--jobs` option reserves marker capacity; it describes the genuine whole-host reservation. No compiler or Docker execution was bypassed or represented as successful by these unit results.

## Final Source Reader Verification

Deployment drift mutation tests now use an in-memory source reader passed to deploymentDrift(repoRoot, reader). No version-controlled sources are copied into ticket fixtures. The latest focused combined diagnostic passed all 44 tests in launch, resolution and deployment (course-fence-final.log), including the 23 deployment tests and current privileged workflow gate assertions.
