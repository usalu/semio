# Result TEST-SWEEP-LIBRARY (prefix `tsl2`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. Logs `🗑️generated/tsl2-*`, tools `tsl2-tools/`.

## 0. Verdict

IN PROGRESS (baseline collected, fixes being applied; see sections 2 to 4). Scope: all `bun:test` files under `🧰️framework/🛍️products/🦑️repo` and `🧰️framework/🔨️modules/🏃️process` (619 files); 115 sit in peer-owned areas (`🏃️process/🏗️build`, `🏃️process/📦️artifacts/🏗️native-build/💾️storage`, `🏃️process/🔒️leases`, `📚️library/🏗️build`) and are recorded only; 504 are in scope.

## 1. Method

- `tsl2-tools/tsl2_list.py` lists test files (`git grep -F bun:test` plus untracked) into `🗑️generated/tsl2-candidates.txt`; in-scope and excluded split in `tsl2-inscope.txt` / `tsl2-excluded.txt`.
- `tsl2-tools/tsl2_run.py <label> <list> <jobs> <limit-secs>`: one `bun test --timeout 120000 --reporter=junit` per file (cwd = nearest ancestor with a `package.json`, else the repo root; nice 10; retried once on segfault), per-file junit and log in `🗑️generated/tsl2-<label>/`, summary `🗑️generated/tsl2-<label>.json`.

## 2. Baseline (before any edit)

Harness findings that change how the suites must be run (all recorded in `tsl2-tools/tsl2_run.py`):

- A bare `bun test <file>` fails 347 tests with `Original parent Script process invocation required` (the library's process-envelope receiver). Without an envelope almost every `registerBuildTestLawV1` suite is red; this is environment, not code. The runs below set `SEMIO_SCRIPT_PROCESS_INVOCATION` to the root envelope wire printed by `rg7-tools/envelope.ts` (same workaround as REPO-GATES-7), `SEMIO_BUILD_MODE=prototype`, run from the repo root (the owner routes in `📜️script.ts` use `cwd = repoRoot`), with a short `SEMIO_TEST_ARTIFACT_DIR` under `.🧬semio/🦑️repo/⚡️cache/🧪️tests/` (a ticket-folder artifact dir exceeds the 256-character path cap and fails 21 tests).
- `bun test` needs `./`-prefixed or absolute test paths; the emoji `🟦️.ts` files never match the `.test` filter.
- 14 files were SIGKILLed (rc -9) in the first pass while other suites ran (bursts of 0.1 s deaths); rerun alone they pass, so they are a concurrency/peer artefact and are re-run in a second pass.

| Pass | Files | Green | Red | Tests passed | Tests failed |
|---|---:|---:|---:|---:|---:|
| bare `bun test` (no envelope, partial 245 files) | 245 | n/a | 139 | n/a | n/a |
| env1 (envelope, ticket-dir artifact dir) | 504 | 194 | 310 | 2,865 | 972 |
| env2 + env2b (envelope, short artifact dir, rc -9 files re-run) | 504 | 223 | 281 | 3,387 | 986 |

Baseline used for the before/after comparison: **env2 + env2b = 281 red files, 986 failing tests** (14 files still killed by signal after the re-run).

## 3. Fixes applied so far (owner side, intended behaviour kept)

| Cause | Fix | Files |
|---|---|---|
| fixture placement law moved every `<owner>/🧫️fixtures` to `<owner>/🧪️tests/🧫️fixtures`, read sites still pointed at the old root | read sites re-pointed (`tsl2-tools/tsl2_fixpath.py`, verified by run) | 41 test files (canonical-execution, artifact-io-ownership, cross-platform-bootstrap, os-source-topology, taxonomy-cli-cancellation, app-verification-source-ownership, artifact-support-leaf-authority, workspace-negation-closure, frozen-markdown-coordinates, historical-package-owner-identity, reference-coordinate-progress, test-dispatch, artifact-packages oracle, cargo-provenance, inference, deployment, discovery runtime, direction binding x2, cargo control, nested-owners, mutation-*, framework/root topology, publication source, empty-facet authority, source-services, taxonomy-input, ...) |
| `package.json` scripts now call `bun --cwd "<root>" repo run @semio-tech/repo-lib:<target>` (no more `nx run`) and project targets use `repo:script` (no more `nx:run-commands`) | expectations updated (`tsl2-tools/tsl2_nxrun.py`) | 16 tests and 4 fixture JSONs |
| the 10k-line normalizer calls the private `requireTaxonomyValue` helper; tests that compile extracted production declarations in isolation did not carry it (ReferenceError) | helper added to the extracted-name sets / support sources | json-reference-owner-lookup, nx-workspace-root-file-reference, run-vitest-config-argument-tokens, typescript-path-collection, taxonomy-scaling-laws, gherkin-description-inline-code, preflight-reference-basis, rust-finite-target-consumption (76 tests), planner-blockers, path-emoji-statutes |
| `normalizeRelative`/`sourceRelative` etc. are now `export function` in `🛣️path`; extracted text carried `export` | `export ` stripped in the extraction | bare-reference-sibling-precedence, path-emoji-statutes |
| `🎟️capacity/🟦️.ts` barrel deleted (commit 693) and the publisher moved to `📥️source/🧊️pristine` | tests import `📥️physical` + `🧮️formula` directly; publisher-source reader points at `🧊️pristine` | 3 capacity tests |
| cargo env: `canonicalArchitectureEnvironment` follows Cargo precedence (target and build dir are separate, repo `.cargo/config.toml` honoured) | fixture gained `expectedBuild`; tests compute expectations from an independent TOML oracle; root script class read from `🎛️commands/✅️verify` | canonical-execution (8/8 incl. real `cargo metadata`) |
| stale hard-coded old ticket as the "active ticket" artifact root | artifact root = caller-owned `SEMIO_TEST_ARTIFACT_DIR` (inside the repo) or the cache default | package-boundary-classification, rust-writable-path-authority |
| corrupted string literal (`"../🧫️fixtures/…/🔣️.json"` where a binary name belongs) in a rustc oracle | restored to a plain binary name | rust-writable-path-authority |
| mangled assertion (`contract.domains[...]` for top-level fixture keys) | back to `fixture[...]` | path-emoji-statutes |
| mutation `📸️set-snapshot` of the TSV artifact deleted by design (commit 251fa724c4a) | fixture row and expectation removed | path-emoji-statutes |
| graph crate carved into six; manifest discovery crate has no `build.rs` | law now accepts `build = false` or no `build` key and no `build.rs` at the new manifest crate | path-emoji-statutes |
| released-checkpoint pin (`rust-finite-target-consumption` content changed by the carve fleet and by the helper above) | pin re-derived (sha256, 86 tests, 436 assertions) | rust-writable-path-authority fixture and inline literal |
