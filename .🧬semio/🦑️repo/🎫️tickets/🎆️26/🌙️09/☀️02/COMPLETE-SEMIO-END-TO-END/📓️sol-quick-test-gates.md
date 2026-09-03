# Quick Test Gate Repair

## Scope

Repaired the three quick-test blockers assigned under the existing umbrella ticket:

1. Vite could not transform the executable in-source `📜️script.ts` test because its client import was inserted before the Bun shebang.
2. `os-hub-admin:test-quick` forwarded `quick` as a Vitest filename filter and passed with no selected files.
3. `os-hub-ts:test-quick` selected one opt-in E2E test and skipped it, so no test executed.

The repo MCP was not exposed in this agent's tool inventory. Work continued inside the explicitly assigned existing ticket; the umbrella ticket was not closed.

## Findings

- Nx's inferred `test-quick` target correctly invokes `📜️script.ts test quick`.
- `resolveTestLevel` is the level-token boundary: it sets `SEMIO_TEST_LEVEL` and returns only remaining Vitest arguments.
- The admin test router bypassed that boundary and forwarded `quick`.
- Shared `runVitest` added `--passWithNoTests` unconditionally, overriding the owning config's non-vacuous policy.
- The hub TypeScript package had no unconditional quick contract.
- Once the dev script transformed, the suite exposed a filesystem-heavy classifier in a unit test and three stale test-only coordinates. The host classifier is now injectable, so the unit test uses its in-memory branch while production keeps the real classifier.

## Changes

- Added a Vite pre-transform that removes only a leading executable shebang from `📜️script.ts`, plus LF, CRLF, and embedded-text regression cases.
- Extracted `vitestRunArguments` and removed the global `--passWithNoTests` override.
- Routed admin test arguments through `resolveTestLevel`.
- Added executable admin regressions using the TypeScript parser for command routing and the shared argv builder for no-test policy.
- Added two unconditional hub quick tests for released-port rebinding, real HTTP readiness, and platform binary-path resolution; the opt-in hub E2E remains skipped without `HUB_E2E=1`.
- Updated dev test-only expectations to the current demonstrator entry, declared factory module contract, and generated host-shim filename.

## Verification

- PASS — `bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache`
  - 4 files, 75 tests passed.
- PASS — `bun nx run os-hub-admin:test-quick --skip-nx-cache`
  - 2 files, 10 tests passed.
- PASS — `bun nx run os-hub-ts:test-quick --skip-nx-cache`
  - 1 file, 2 passed and 1 opt-in E2E skipped.
- EXPECTED FAILURE — `bun ./📜️script.ts test quick __semio_missing_quick_test__.test.ts` from the admin package.
  - Vitest exited 1 with “No test files found,” proving an empty selection no longer passes.
- PASS — isolated dev shebang regression before the complete dev rerun.
  - 1 file, 3 tests passed.
- The repository-library monolithic test entry was also attempted, but it currently fails before selection on an unrelated live taxonomy rename: missing `../../🧹️normalization/🧪️tests/🟦️source-admission.ts`. The shared argv regression therefore lives in the independently green admin quick suite.

No temporary generated output was retained.
