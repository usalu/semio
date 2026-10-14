# Portable Workspace Identity

## Implemented Contract

A workspace has regular, colocated `📋️project.json` and `package.json` files, and its package explicitly declares `workspaces` as an array or object. A nested ordinary package/project does not become the repository root. Runtime discovery and persistence guards no longer depend on `nx.json`. Malformed paired package JSON refuses discovery instead of silently climbing to a different owner.

The shared Rust filesystem admission function lives in `🧰️framework/🔨️modules/🏃️process/🧭️routing/🏠️workspace/🦀️.rs`. Its caller owns JSON decoding, keeping the dependency-free repository host independent of framework serialization and allowing OS runtime consumers to use the existing owned JSON implementation. Python, Go, and C# use their system JSON APIs.

## Updated Surfaces

- OS MCP repository discovery and repository-root workspace binding guard.
- OS run bootstrap discovery.
- OS DSL mutation-source authority root pairing, preserving no-follow checks and strict missing-pair refusal.
- OS store folder event-log root-write guard and its existing regression fixture.
- Rust, Python, C#, and Go repository test-host discovery. Missing workspace roots now refuse instead of returning the invocation working directory.
- Stdio deletion-proof bootstrap copies package identity instead of an Nx marker.
- DSL leaf and aggregate authority fixtures use package workspace declarations and neutral nested workspace anchors.
- Framework process script and project manifest expose `test-workspace`; the integrating agent owns corresponding editor launch entries.

## Validation

Executed on macOS, 2026-10-14:

1. Existing process routing Bun suite: **41 passed**, 206 assertions.
2. New workspace identity suite: **2 passed**, 117 assertions. Twelve schema-validated language-neutral vectors agree across TypeScript, the existing `find-up` third-party oracle, and the actual Rust, Python, Go, and C# test hosts. Rust and Go hosts were compiled from their authored sources; C# linked the authored host. Runtime output confirmed all twelve vectors with a `[DEBUG]` line. Vectors include array/object declarations, nested ordinary projects, nearest nested workspaces, missing marker pairs, absent/null/scalar workspace declarations, Nx-only identity refusal, malformed JSON, and malformed nested JSON that must not resolve an outer workspace.
3. Strict TypeScript check of the new test source: **passed**.
4. Focused DSL native Cargo test through the owned public repository target: the Nx backend failed before Cargo because its dependency graph referenced absent ignored `.lake/packages/proofwidgets/widget/package.json`. An initial fresh graph discovery was cancelled by its owning process after a reusable graph became available. The Bazel retry completed the actual owned schema-index generator and native Cargo test command successfully: all three selected mutation-source-authority tests passed, including leaf and domain fixture enforcement. A subsequent complete library run through `SEMIO_BUILD_TOOL=bazel SEMIO_BUILD_REUSE_GRAPH=true bun repo run @semio-tech/dsl-derive-rs:test -- --lib -- --nocapture` passed **all 22 native tests**, including aggregate-source authority, mandatory mutation, schema-document index, and existing third-party schema/hash oracles. Bazel completed three actions in both invocations.

The native fixture workspace paths are padded above absent-root fixtures to exercise the 32-ancestor limit without accidentally discovering the real repository. Each asserted start path is at most 256 characters. Generated multi-host compiler outputs, binaries, and fixture files beneath this ticket's `🗑️generated/wi-*` and DSL fixtures beneath `🗑️generated/dsl` were removed after successful verification. No source inputs or Markdown reports were removed. Full OS application builds and its persistence-guard unit suites were not executed in this subtask; the guards use the shared classifier exercised by the real Rust host and retain their existing updated unit fixtures.


# Play and Pets Public Build Boundary

The Play fresh-build fixture was updated before implementation. Its first executed run failed on the old backend-specific arguments, environment, E2E identity, and Vite cache selection, establishing the expected failing assertions.

## Changes

- Fresh-build arguments now use `--skip-cache`, `--skip-remote-cache`, and `--output=stream`; concurrency still follows the existing CPU/memory policy. Cache bypass is expressed by arguments without duplicate environment switches.
- Fresh-build environment declares `SEMIO_BUILD_MODE=release`, `SEMIO_NATIVE_BUILD_GENERATION=fresh`, and `SEMIO_BUILD_TERMINAL=false`.
- Play Vite cache selection uses `SEMIO_BUILD_CACHE_DIRECTORY`. Its assertions account for the existing compiler-state implementation resolving storage paths to absolute paths on every platform.
- Play E2E service generation requires `SEMIO_BUILD_INVOCATION_PID`, with schema-validated positive, safe-integer identities and refusal of missing or malformed values. The Nx backend owner was separately notified to issue the same shared identity as Bazel.
- Play and Pets development manifests use `SEMIO_BUILD_TERMINAL` and `SEMIO_BUILD_DYNAMIC_OUTPUT` and remove the private native-command-runner switch.
- Only the matching fresh-build launch entry was edited in the seed and generated launch JSON: its command and environment now exactly match the fixture. This also replaces its invalid `ship` compiler intent with `release`. Other configurations and canonical acceptance entries were preserved.

## Verification

The original fresh-build test registration was executed through the small Bun adapter retained at `🧪️play/📜️script.ts` in this ticket. **9 tests passed, 86 assertions**, plus a successful strict TypeScript check of the updated helper, E2E helper, and original test source, using the existing `yargs-parser`, `jsonc-parser`, TypeScript parser, and Ajv third-party oracles. This includes both editor launch entries, the Play/Pets manifests, resource-bound parallelism, Vite path partitioning, and nine E2E identity vectors. Runtime `[DEBUG]` output confirmed admitted shared E2E identities and refusal of missing or malformed identities.

A separate runtime call passed the actual `playFreshBuildArgs` output to the public neutral frontend's `validateBuildTaskArgumentsV1`; it accepted the complete argument vector and emitted a `[DEBUG]` confirmation. No full Play release, browser E2E session, or Pets development server was started for these contract changes.

Updated authored files: Play fresh-build helper, Play E2E runtime helper, adjacent fresh-build tests/fixture, Play project manifest, Pets React project manifest, and the matching `.vscode/launch.json` / `.vscode/🧩️launch.seed.jsonc` entry. The ticket-owned Bun adapter is preserved as an input; its empty generated output directory was removed.


# Neutral Input and Observation Ownership

Schema/fixture references were updated first. Complete file moves (no compatibility modules):

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📥️inputs/🟨️.mjs` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📥️inputs/🧮️digest/🔣️schema.json` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🔣️schema.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📥️inputs/🧮️digest/📜️script.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📥️inputs/🧮️digest/🟨️.mjs` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📥️inputs/🔒️lock/🔣️schema.json` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/🔣️schema.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📥️inputs/🔒️lock/📜️script.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📥️inputs/🔒️lock/🟨️.mjs` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/📥️inputs/🔒️lock/📥️read/🟨️.mjs` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/👁️observation/🟨️.mjs` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/👁️observation/🧪️tests/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/👁️observation/🧪️tests/📦️projects/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/📦️projects/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/👁️observation/🧪️tests/🧫️fixtures/🔣️.json` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/👁️observation/🧪️tests/🧫️fixtures/📦️projects/🔣️.json` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧫️fixtures/📦️projects/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/👁️observation/🧬️schema/🔣️.json` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔌️nx-plugin/👁️observation/🧬️schema/📦️projects/🔣️.json` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧬️schema/📦️projects/🔣️.json`

Cache inventories move from `nx-input-closures` / `nx-lock-closures` to `build-input-closures` / `build-lock-closures`; the existing neutral digest environment names remain unchanged. Active source, schema, fixture, command, editor, and projection references are included; historical ticket evidence is preserved.


## Ownership Validation

- Schema and fixture references changed before source movement. The portable graph test then failed with the expected missing neutral input module. After the filesystem rename and source reference updates, the portable graph suite passed **10 tests / 53 assertions**, including its JSON Schema, independent bundler, installed Nx glob, and independent task graph oracles.
- The moved observation law suite passed **21 tests / 62 assertions** against its existing language-neutral filesystem scenarios, strict Ajv record schema, and fast-glob listing oracle. Runtime `[DEBUG]` output confirmed hits for unchanged/identical bytes and misses for changed facts.
- The focused ticket input harness `🧪️inputs/📜️script.ts` passed **3 tests / 25 assertions**. It uses the existing language-neutral input/lock/digest vectors and validates the neutral compaction manifest, safe-path schemas, minimatch membership, graphlib lock reachability, installed Nx named-input expansion, and independent Bun SHA-256.
- The actual Node project/dependency inference suite was executed through `SEMIO_BUILD_TOOL=bazel SEMIO_BUILD_REUSE_GRAPH=true bun repo run repo:test-project-admission --exclude-dependencies --skip-cache --output=stream`: **2 tests / 21 assertions passed**. Its source fixtures now author `repo:script`, shared inference records use `SEMIO_BUILD_INFERENCE_DIRECTORY`, and Nx SDK oracle environment variables remain separate. Runtime counts confirmed cold owner computations, warm reuse, selective reverse-import recomputation, and dependency byte reuse. Bazel completed two actions successfully.
- The broad existing `testNxBuildModes` function completed its early common input schema/compaction/lock/oracle assertions but stopped at a separate command dependency oracle mismatch at line 152. That entry still points at the old Nx bootstrap while the common owner is being extracted by the core agent. This broad function is **not reported passing**. The core owner received the exact entry and mismatch.

All 15 moved source files have absolute paths of at most **148 UTF-16 code units**. A repository-wide active-text scan found no old input/observation ownership paths, old closure-cache namespaces, or old observation schema identifiers. Historical ticket evidence intentionally retains old→new mappings. The Nx patch itself contained no old cache namespace literal; its common runtime authority reexport now points at the neutral input module, and the core owner manages patch-lock refresh.

`📥️workspace-files.json` is the exact authored file inventory for all workspace-identity, Play/Pets, and neutral ownership work in this ticket, including each moved old and new path. It contains 84 unique paths. Temporary generated outputs for observation and focused input checks were removed; input harnesses and Markdown evidence are retained.

A later graph-suite rerun picked up three concurrently added core cases. With the required ticket artifact environment set, **12 passed and 1 failed**: the new source-watch test did not produce its selected-file receipt. The exact failure and the possible absolute-path ignore-filter cause were sent to the core owner; no source-watch implementation was changed in this subtask. The original ten graph cases and both new pinned-authority cases passed.
