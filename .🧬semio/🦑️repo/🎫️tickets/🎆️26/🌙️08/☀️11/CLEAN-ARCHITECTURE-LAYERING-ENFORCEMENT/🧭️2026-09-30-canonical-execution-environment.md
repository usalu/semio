# Canonical Execution Environment Regression

The root canonical architecture orchestrator now obtains its Cargo environment from the generic repository Cargo owner. The helper accepts explicit first-party environment values, copies them without ambient reads or mutation, resolves the caller's `CARGO_TARGET_DIR` against the workspace, otherwise selects the canonical repository cache, and sets both `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` to that same absolute root. Existing explicit-empty/nullish target semantics remain intact. A stale build-directory override cannot keep canonical compilation on shared compiler-unit locks.

This follows the actual sampled lock diagnosis in [the native cache assessment](./🔒️2026-09-30-brep-native-cache-lock-assessment.md). Only the environment construction and its owning import were extracted from the root method. Existing `--skip-nx-cache`, parallel flags, command admission and orchestration budgets remain as verified by the root owner.

## Schema-First TDD Proof

The closed draft-07 JSON schema and four neutral vectors were written before the production helper. The vectors cover empty input/default cache, relative override plus stale build root and preserved fake secret, absolute override plus ordinary environment values, and explicit empty target plus ignored secondary target. Tests also assert immutable input, no extra inherited keys, and exclusion of an ambient fake secret when explicit input is empty. Neither implementation nor test logs expose real environment values.

RED command:

```
SEMIO_TEST_ARTIFACT_DIR=<this-ticket>/🗑️generated/canonical-execution-environment bun nx run @semio-tech/repo-lib:test-canonical-execution --skip-nx-cache
```

Actual result before helper implementation: exit1, absent production export `canonicalArchitectureEnvironment`, zero tests passed, one module failure. No implementation facade or weakened assertion was introduced.

GREEN used the exact same owned Nx target and artifact location after the production helper/root consumer were implemented: exit0, **7 tests passed, 0 failed, 33 assertions**, Bun test416ms; Nx owned task787ms, cache explicitly skipped.

The independent existing `@iarna/toml` parser reads a real closed Cargo config whose target/build directories differ. The first-party effective-directory consumer agrees with those independent parsed config values. After environment construction, it resolves both directories to the selected isolated root. Real Cargo `metadata --no-deps --offline --format-version 1` independently reports that exact target directory for every neutral vector. Cargo compiled no native source and used no new runtime library.

The RepoLib `canonical-architecture` contribution now runs this portable law first. The focused test target invokes only the existing owning `📜️script.ts`. Parent owns the final launcher/catalog update and full aggregate. No guest,355-law native, broad typecheck or full aggregate was duplicated here. This execution ran on native macOS; portable paths and isolated Cargo fixture construction use standard platform path functions, but Windows/Linux executions are not claimed.

## Exact File Manifest

Created:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏛️canonical-execution/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🏛️canonical-execution/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏛️canonical-execution/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🧭️2026-09-30-canonical-execution-environment.md`

Updated:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts`: first-party environment contract/helper; existing Cargo directory/provenance behavior preserved.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`: focused portable test route and canonical contribution invocation.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`: uncached focused target calling the existing script.
- `📜️script.ts`: one owner import and replacement of the inline canonical environment expression.

Removed source/input files: none.

Owned temporary RED/GREEN logs and Cargo oracle fixture output were generated exclusively under this ticket's generated directory; removed after results were recorded. No peer output, shared cache, Git state or AGENTS file was modified.

## Settled Source Identities

SHA-256:

- Cargo owner: `5363ef0e10cf5dcfcdde7d85eca66786a1e6d2eb49b03f3a839685e9e8e8500b`
- Portable test: `546ddfb2856bda645e42a4af4978928c9a0892e98c0c3b64365c65b2de0321d1`
- Neutral vectors: `89a09ffe2c9313bb55c3180557a8e6d41d550c3b54c61b1d0206c12dac015ec2`
- Closed schema: `c77e182b9c0cd6700f0e3c6165a62c9e693a294a28b9de3749a1a1b282147419`
