# Dashboard Build Tool Boundary

## Changes

- The neutral build runner in `📚️library/🏗️build/📜️script.ts` now detects per-mode installed dashboard launches and hands over directly with the original cancellation/deadline and selected backend.
- Dashboard registry effects use `buildFlags`/`BuildFlags`/`build_flags` and neutral task options. Target, group, playground, declared tool and projected editor commands invoke `bun repo`.
- Dashboard inventory consumes `.🧬semio/🦑️repo/⚡️cache/build/graph.json` with `version: 1`, direct `projects` records and dependency arrays. The snapshot version is now 7. Backend graph errors are represented as neutral strings.
- Graph reuse compares the neutral graph publication and manifest content hashes. Nx file map reads were removed. The runner receives neutral terminal, graph reuse, dynamic output and invocation environment names.
- Launch generator, launch schema, shared fixtures, authority schema and independent TypeScript oracles use the neutral interfaces.
- Editor seed and generated output include neutral commands, options and serialized capability argument arrays. Public backend environment entries were removed. `normalizeDashboardLaunchBuildV1` is called by projection and by the package launch write owner; recursive normalization covers fresh backend command text, argv, nested capability JSON and backend environment fields, while preserving Nx/Bazel selector choices. RFC6902 and Ajv independently validate the normalization vector.
- Projection refresh preserves a peer configuration appended after an existing owner block. Added an explicit regression assertion. Fixed preexisting launch/authority fixture paths and one generated configuration with missing debug type/request.

## Validation

Executed with Bun 1.3.14:

- Launch and control-plane suites: 15 tests, 767 assertions passed. Independent libraries: Ajv, fast-json-patch, Node readline/Buffer, strip-ansi and Nx task graph scheduling. Runtime `[DEBUG]` output confirmed neutral commands for 496 current ticket tools and 992 English/German launch entries.
- Registry declaration and neutral publication schema subset: 2 tests, 20 assertions passed. Ajv accepted neutral shared vectors and rejected the obsolete backend graph/effect shape.
- Installed native launch argument selection and neutral runner bridge: 2 tests, 25 assertions passed, with Nx independently confirming task scheduling. A controlled immutable installation received daemon status, release mode and Bazel backend before graph discovery; `[DEBUG]` output confirmed the handoff.
- Root authority subset: 4 tests, 22 assertions passed, confirming the actual root package script and declared script command.
- Both editor documents parsed as JSON after neutralization. `git diff --check` reported no whitespace errors.

## Native Validation Limitation

`bun repo run @semio-tech/repo-dashboard-rs:test --skip-cache --exclude-dependencies -- unit` was launched. It provisioned a pinned Nx SDK snapshot and immutable authorizer inputs, then entered project graph preparation. The invocation failed before compiling Rust: emoji project plugin reported native host/producer mismatch, a frame-worker generated JavaScript output without a producer, and a missing original full module graph producer; test-cases plugin also reported a missing dispatch-macros Rust dependency. No Rust compile/test success was observed. An earlier direct Nx task attempt failed at Nx daemon graph calculation and was terminated. Rustfmt independently parsed both modified Rust modules and both associated unit test modules successfully.

## Notes

Other concurrent workers continue appending backend-specific launch rows. The active dashboard launch projection and package write owner now normalize these fields at publication. The current root script contains no general launch-seed generator to call; searches of active source found only the dashboard owner. Both documents were normalized using that same exported production helper. Generated installed-runner fixture paths assert the 256-character limit and the fixture removes its generated tree afterward.
