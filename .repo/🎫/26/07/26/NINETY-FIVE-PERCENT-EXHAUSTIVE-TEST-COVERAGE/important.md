# Important: vitest v8/istanbul coverage is non-functional in this Claude Code sandbox

**Status:** the JS/TS coverage wiring (`runVitest` coverage flags, `coverage.include` in ~29 vitest configs,
`@vitest/coverage-v8`) is fully implemented and matches the design, but could not be runtime-verified in this
session — `@vitest/coverage-v8` (and, tested as a control, `@vitest/coverage-istanbul`) produce an empty
`coverage-final.json` (`{}`) for every run in this execution environment, regardless of:

- vitest version (tried 3.2.4, 4.0.17, 4.1.7 — all matched to their coverage package)
- Node version (tried 22.23.1 and 24.15.0 via homebrew)
- sandbox mode (tried with and without `dangerouslyDisableSandbox`)
- repo involvement (reproduced in a bare scratch project fully outside the monorepo, zero ancestor configs)

Root-caused as far as is possible from here: a raw `node:inspector/promises` `Session.post("Profiler.
takePreciseCoverage")` call **does** return real per-file coverage data when called directly in the same
process (verified — captured the calling script's own function). But the same V8 Profiler data never reaches
vitest's coverage report, for both the v8 provider (inspector-based) and the istanbul provider (instrumentation-
based, no inspector at all) — meaning whatever is broken is shared plumbing inside vitest's coverage pipeline in
this environment, not a provider-specific inspector issue and not a version regression.

**Action needed:** re-run the smoke test (`SEMIO_COVERAGE=1 bun ./script.ts test quick` in e.g. `cad/core` or
`mathematical/graph/dsl/core`) in the actual devcontainer/CI environment, where this sandbox restriction likely
does not apply. Check `.repo/coverage/js/**/lcov.info` for non-empty `SF:`/`DA:` records. If it's still empty
there, this needs upstream investigation (vitest/coverage-v8 issue tracker) before Phase A of the workforce can
trust any JS coverage numbers — until then, treat repo-wide coverage percentages as Rust/Go/Python/.NET-only.

**What IS verified working end-to-end in this session:**
- Rust: `cargo-llvm-cov` on `mathematical_number` — real LCOV with populated `DA:` hit counts (72.60% on a
  first run), confirmed via `.repo/coverage/rust/*.lcov`.
- The aggregation pipeline itself (`parseLcov`/`mergeLcov`/`summarizeCoverage` in `repo/lib/js/index.ts`) —
  verified against the real Rust LCOV output, produces correct per-file and repo-wide percentages.
- `bun install` dependency resolution for `@vitest/coverage-v8` (root `package.json`).
- The `test-exhaustive` nx-target gap closure (0 offenders remain, verified via a full repo scan).
- `runVitest`'s pre-existing latent bug (fixed as a side effect): `bun x vitest` resolves bunx's own globally
  cached vitest version rather than the workspace's locally installed one — silently drifted to 3.2.7 while
  the workspace was pinned to `^4.0.17`/resolved 4.1.7. Fixed by invoking `node_modules/vitest/vitest.mjs`
  directly. Coverage runs additionally need to run under plain `node`, not bun — Bun's `node:inspector` shim
  does not implement the V8 Profiler coverage APIs (`Session.post` on `Profiler.startPreciseCoverage` throws
  "Coverage APIs are not supported").
