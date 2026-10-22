# Source0 GraphQL Freshness Audit

2026-10-21. Static reads only; no Source/Native execution, production edits, cache/store inspection, or manifest mutation.

## Recommended Inputs

Append these five explicit strings to workspace:test-workspace-build inputs after lock session 37397 finishes and before Sync:

```json
[
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🔤️types/📥️source/**/*",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🔤️types/📋️plan/**/*",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🔤️types/📽️projection/**/*",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🔤️types/📖️parse/**/*",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🔤️types/⌛️queries/**/*"
]
```

The exact five kernel module paths are the same strings with `/**/*` replaced by `/🟦️.ts`. They are authoritative in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🔤️types/🧪️tests/🔒️producer/🧫️fixtures/🔣️.json` (`expectedModules`). The fixture excludes `🔤️types/🤖️generated/🔗️graphql-types/🟦️.ts`; none of the five globs includes it.

## Matcher And Backend Findings

Brace alternatives ARE explicitly implemented by Source `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🟨️.mjs:31-32`. Source owner discovery uses that matcher (`🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs:149-152`), so a five-way comma brace is authoritatively supported there. Bazel inputMatcher (`🔧️tools/🏗️bazel/📜️script.ts:337-340`) delegates ordinary syntax, including braces, to Bun.Glob; unfamiliar patterns retain a broad prefix. Five literal globs avoid depending on cross-backend brace parity and retain narrower traversal roots. Nx receives the manifest input declarations through inference; the existing target already uses the same workspaceRoot and **/* syntax. No runtime backend check was performed during the freeze.

## Freshness Gap And Boundaries

The current target has only the build-tree glob and root manifest as authored inputs. Its test entry statically imports the producer test (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🟦️.ts:1`); derived command closure can cover that test, its JSON fixture/schema, and imported oracle implementation. The producer test reads fixture.contract through resolve(root, fixture.contract), then dynamically traverses the resulting entries using relativeSourceInputs and TypeScript preProcessFile. Those dynamically named kernel bytes are not literal static imports. The proposed source glob additionally covers the dynamically read `📥️source/🔣️.json` contract, which declares source and plan entries, generated=[], externalDependencies=[]. All five kernel reads need freshness coverage even if some happen to join another static closure today.

Keep sourceInputObservation=freshness and existing runtimeSourceEntries unchanged: owner discovery adds matched bytes to facts while compileByTarget stays empty for freshness scans (`owner-discovery:205-214`). Promoting these data inputs to runtime entries or compile inputs would unnecessarily restore compiler/generated-output demand.

Future fixture expansion to modules outside these five directories requires a matching declared input update. The existing derived closure should cover statically imported producer fixture/schema changes; the five globs do not claim coverage of arbitrary future dynamic paths. This is a bounded static audit, not proof of complete runtime filesystem-read coverage.

## Bounded Source0 Hook Ownership Extension

The automatic-hooks leaf statically imports its lifecycle helper; that helper statically imports the lifecycle implementation. Those TS modules belong to derived command closure. The ownership function dynamically reads these four DATA files; append their exact freshness inputs alongside the five GraphQL globs:

```json
[
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🧫️fixtures/🔁️lifecycle/🔣️.json",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🐳️containers/🔁️lifecycle/🧬️schema/🔣️.json",
  "{workspaceRoot}/.devcontainer/devcontainer.json",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1"
]
```

These are exact current paths, including the ordinary `.devcontainer/devcontainer.json` filename. The fixture and policy reads use join(import.meta.dir, ...); nativeBootstrap.path comes from fixture data. They are not JSON import declarations or literal new URL resources.

The function also asserts all fixture.dashboardOnly.removedPaths absent. File reappearance coverage needs these five additional input patterns:

```json
[
  "{workspaceRoot}/.vscode/**/*",
  "{workspaceRoot}/.cursor/mcp.json",
  "{workspaceRoot}/.cursor/hooks.json",
  "{workspaceRoot}/.devcontainer/editor-overlay.json",
  "{workspaceRoot}/Monorepo.sln"
]
```

The .vscode recursive glob covers its six explicitly named JSON/JSONC children plus any other regular child file. Source scans existing regular files only, skip symlinks, and do not hash directory existence (owner-discovery:151-152). Therefore these declarations cover ordinary file reappearance, but cannot prove the fixture assertion for an empty `.vscode` directory or symlink `.vscode`; a glob alone cannot close that exact absence condition. Do not declare the directory itself as a file input: current literal scanning admits an existing path, while downstream source readers expect files. This remaining limitation is reported for owner review, not a request to add a runtime feature during freeze. No need to cover forbiddenState as host paths: this test compares those strings only against persisted-directory constants and config mounts.

Generation document-size leaf imports its fixture, fixture schema, generation schema and generation implementation statically, all within the existing build/**/* glob. No analogous missing DATA input found there.

The current retirement/support TS test statically imports `../🧬️schema/🔣️.json` and `./🧫️fixtures/🔣️.json`; it performs no dynamic filesystem reads and does not read Rust source. Its Rust sibling uses compile-time include_str!("🧫️fixtures/🔣️.json"), but the Source0 TS leaf never executes that Rust sibling. No additional Source0 freshness input is justified for Rust bytes by the current leaf. This is a bounded inspection of the named leaves, not a whole Source0 audit.

## Final Source0 Cache Decision Review

Recommend classify only `workspace:test-workspace-build` as an uncached live runtime gate at the queued post-lock quiet edit:

```json
{
  "cache": false,
  "metadata": {
    "semio": {
      "cache": { "version": 1, "class": "live", "reason": "exercises-host-resources" }
    }
  }
}
```

Merge the metadata into the existing semio object; retain sourceInputObservation=freshness, runtimeSourceEntries, outputs=[], command and runtime roster. `🏃️execution/🧬️schema/💾️cache.json` defines Live explicitly as observing real locks, storage, process tree or clock; exercises-host-resources is an allowed reason. This precisely fits actual worker, cancellation/deadline and real checkout absence assertions. The build skill says live targets declare cache:false. It removes replay risk without introducing a negative-facts framework. Keep the dynamic DATA and GraphQL inputs for admission/freshness; with caching disabled, absence inputs are documentation/file-admission coverage, not a claim that regular-file hashing proves directory/symlink absence.

Target-local dependency checks: the Source roster test and fixture have no cache:true assertion or cache field; no existing roster schema change is required to flip this declaration. Root manifest target project schema already permits cache:false and cache metadata through the existing Cache contract. No changes to generic cache schemas or global test policy are warranted.

Recommended regression adjustment: add one narrowly scoped test to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🎯️source-roster/🟦️.ts` reading the existing root manifest and asserting test-workspace-build.cache===false and metadata.semio.cache deep-equals the Live object above; validate that object against the existing cache schema via Ajv and compare the manifest values with SQLite json_extract (existing dependencies). Keep this assertion specific to Source0; do not assert all Source roster targets are uncached. Existing role/runtime roster assertions continue unchanged. If expressing it as language-neutral fixture data, add a separate closed optional/top-level runtime-gate expectation in that roster fixture/schema rather than altering all 26 case records; that is more schema work than needed for this declaration-only correction.

Precision policy caveat: `📥️inputs/🎯️precision/🔣️.json`, row workspace-test-build-library, selects both test-workspace-build and test-runtime-build-producer and still describes only build-library reads. Do not broaden that shared row for the hook/GraphQL reads, because that would unnecessarily broaden the separate runtime producer. Explicit new target inputs are retained by the row (its replaces list is only named defaults/native/production inputs). A description-only correction acknowledging explicit extra inputs is optional; no cache assertion there requires mutation.

Bounded alternative-owner inspection found no existing separate uncached owner covering all these assertions. Root setup-devcontainer-start/attach are uncached side-effecting setup commands, not assertion gates. test-build-tool-bazel is an uncached live gate for its own Bazel phases, not an owner of the newly imported automatic-hooks leaf or every Source0 worker/cancellation test. The repo-lib manifest has no separate devcontainer/lifecycle/worker/deadline owner (its taxonomy-cli-cancellation target is cached and unrelated). Thus retaining Source0 cache:true on the theory another uncached gate always runs all these checks is unsupported by the inspected declarations. No commands executed; no production mutations.
