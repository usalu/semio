# Coordinator Fixture Resolution

The protocol schema no longer permits local fixture URIs/scopes. The resolver derives fixture and asset roots from the semantic owner, rejects case-local and traversal paths, requires a regular file, and rejects symbolic path redirection. Case discovery no longer exposes a local fixture directory; case contracts no longer permit nested fixture directories; Nx cache inputs no longer include a case-local fixture glob.

Regression development: the new language-neutral vectors are owned under `🧫️fixtures/🧭️fixture-resolution` and validated by Ajv against the test protocol schema. The actual pre-change resolver run had 6 passes and 5 failures (one was an over-broad schema wrapper in the test harness, corrected); the corrected schema/resolver run passed all 11 cases and 34 assertions through public Bun/Nx. The new case has not yet been verified through the complete repository dispatcher.

Three existing test-platform JSON vectors moved out of test folders byte-for-byte; their current imports and discovered-case construction were updated. Broader consumer/source audits and runtime integration remain in progress.

## Byte-preserved Moves

```json
[
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧭️contribution-directory-ownership/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧭️contribution-directory-ownership/🔣️.json",
    "sha256": "8ae46aeda364ae469341cf56e796e0bed8f874577762f482e2401289ff6fa5d6"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/📐️test-layout/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/📐️test-layout/🔣️.json",
    "sha256": "8f3f5aa729a996a0e777797fedf90d5393e1e1406ea518abf3959f09ba2e8056"
  },
  {
    "old": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️schema-invariants/🔣️.json",
    "new": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧬️schema-invariants/🔣️.json",
    "sha256": "a33fc55a682251a553353f6a4ec7d7a124b18b07a2a50ed5851e583ab25621fa"
  }
]
```

## Authored Paths

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🎫️ticket.json",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📌️important/📝️.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️coordinator-fixture-resolution-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️fixture-layout-current-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️fixture-separation-plan-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🔎️fixture-layout/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧫️nx-fixture/nx.json",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧫️nx-fixture/package.json",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧫️nx-fixture/project.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/README.md",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📜️script.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/📐️test-layout/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/📐️test-layout/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️schema-invariants/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧭️contribution-directory-ownership/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧭️fixture-resolution/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/📐️test-layout/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧬️schema-invariants/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧭️contribution-directory-ownership/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧭️fixture-resolution/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json"
]
```

The existing package test router now includes the new canonical resolution case. The existing real-document asset check was repaired to the current bachelor-thesis filename and precise example owner, removing its silent missing-file return. The projected-vector check now rejects test storage masquerading as an asset. The complete package runtime failed to finish within its supported long-level budget.

## Physical Layout Enforcement

Nine neutral regression vectors were added before enforcement. The original scanner failed five: JSON/binary data in cases, nested fixture folders, delivery-scoped fixtures, and source examples misclassified as executable tests. After implementation the full canonical-layout suite passed all 32 tests with 84 assertions, including its Rust compiler, TypeScript checker, minimatch, and Ajv oracles. The scanner now enumerates all authored file paths and reads source only when needed; fixture examples are treated as data, requiring a separate audit that they are not linked implementations or production dependencies. The real full-path scan is running.

The package router quick run reached its existing 30-second process budget during registry discovery and terminated. A confirmed terminal result justified running its supported long level. The long run terminated at its 300-second budget after registry/profile shape assertions failed (semantic-pdf-v1 comparison, missing oracle comparisonProfiles and no-oracle substitutes), and global discovery checks timed out. These assertions were not changed by this fixture work; no package-wide passing claim is made.

## Production Import and Compile-Time Include Boundary

Nine added language-neutral cases exposed five missing dependency findings before the implementation (4 passed, 5 failed). The source scanner now resolves actual TypeScript imports/reexports/dynamic imports/unshadowed requires and Rust literal includes/path modules against the discovered file set. Canonical test implementations and cfg(test)-required Rust items are test reads. A production import of fixture source is reported before that target can be treated as an opaque example. Filesystem reads, manifest/glob edges and full target-specific feature closure remain additional work; this is not yet a complete production dependency proof.

## Dependency and Mutation Pair Regression Results

The complete direct import/include boundary and physical-layout suite passed 42 tests with 97 assertions after the initial failing vectors. A subsequent filesystem-read extension was developed from eight neutral vectors: four initially failed because production `readFileSync`, aliased `readFile`, namespace reads, and `Bun.file` were not inspected. All eight now pass. Reads are resolved through an unshadowed module-relative URL, with local read/URL shadowing and unused URL values treated separately. This is a bounded static analysis, not yet a complete transitive dependency proof.

The mutation registry now validates a canonical implementation and a separate five-leaf data bundle under their semantic owner. The old combined source/projected bundle fallback is removed. The new neutral pair contract failed before the change and now passes six tests with 11 assertions, including an Ajv closed-file-set oracle. Missing evidence, missing implementation, executable code inside the data bundle, and extra fixture members are rejected.

## Additional Authored Paths

```json
[
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️mutation-fixtures/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/🧬️mutation-fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/📐️test-layout/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧫️fixtures/📐️test-layout/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📜️script.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🔎️fixture-layout/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧪️hub-root/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧪️caching-consumers/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧪️netz/📜️script.ts"
]
```

## Manifest Boundary Enforcement

Seven additional neutral cases exercise static fixture routes, asset routes, fixture Cargo workspace members, development fixture packages, inert metadata, Node fixture entry points, and asset entry points. Four cases failed before implementation; all seven passed after. Manifest inspection uses the framework TOML parser and resolves actual authored targets. Fixture directories containing opaque manifests remain examples unless an outside manifest selects them as executable nodes.

The full current layout regression suite passed 59 tests with 133 assertions, including independent @iarna/toml parsing, native Cargo metadata discovery, Node filesystem execution through esbuild, and the earlier Rust/TypeScript/Nx oracles. A fresh manifest-only census examined 102843 eligible paths and 457 package manifests, finding no remaining selected fixture package or static asset dependencies. This census complements the broader layout snapshot and does not replace the final transitive source audit.

## Current Coordinator Input and Report Inventory

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️cli-analyzer-fixtures-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️coordinator-fixture-resolution-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️fixture-dependency-audit-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️fixture-layout-current-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️fixture-manifest-dependencies-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️fixture-separation-plan-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️framework-fixture-separation-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️global-fixture-asset-audit-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️hub-fixture-reference-audit-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️hub-fixture-relocation-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️hub-root-test-boundaries-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️netz-fixture-relocation-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️plugins-fixture-separation-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️repo-caching-fixture-moves-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️repo-data-classification-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️repo-fixture-backlog-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️repo-root-boundary-followup-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️stray-test-overlay-classification-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🔎️fixture-layout/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🔎️fixture-manifests/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧪️caching-consumers/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧪️hub-fixtures/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧪️hub-root/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧪️netz/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧪️vscode/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧫️nx-fixture/nx.json",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧫️nx-fixture/package.json",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧫️nx-fixture/project.json"
]
```

## Fixture Directory Spelling and Follow-up Validation

The taxonomy now declares prohibited legacy fixture-directory spellings through testFixtureLegacyDirectoryNames, validated by discovery and the test taxonomy loader. The scanner rejects old directories while preserving canonical case names and opaque fixture internals. Five language-neutral vectors went from 3 failures / 2 passes to 5 passes. A minimatch oracle independently checks the spelling vectors. The complete layout case passed 65 tests / 145 assertions.

The latest whole-repository scan inspected 114,137 authored paths and recorded 87 remaining findings; scoped residuals were handed to their implementation agents. This is a progress snapshot, not a clean final audit.

New supporting reports: 📓️nakagin-fixture-separation-2026-09-09.md; 📓️computed-fixture-read-audit-2026-09-09.md; 📓️historical-output-and-computed-fixture-consumers-2026-09-09.md.

Final library read follow-up: 95 canonical TypeScript case files yielded 362 statically resolved literal, composed and JSON-property file reads with zero missing paths after the runtime asset moves. The audit remains bounded to supported expressions; indirect production dependencies are covered by the separate final audit. Production catalog loading and README identity evidence are retained in 📓️runtime-taxonomy-assets-2026-09-09.md.

## Final Registry and Read Follow-up

The real registry initially exposed ordinary canonical unit tests being mistaken for undeclared mutation fixture vectors. Catalog declarations still require an implementation and their exact five-leaf fixture bundle, and physical undeclared fixture bundles still fail. An ordinary canonical test without a fixture bundle does not claim to be a catalog vector. Two additional neutral cases and the Ajv closed-file-set oracle distinguish those cases. The focused suite passed 8 tests and 15 assertions.

The actual registry loader and pair validator subsequently passed all 76 inspected catalogs and 1,890 scenarios across 203 contributions, with zero findings. The durable result is in `📓️mutation-fixture-registry-current-2026-09-09.md`.

After the two additional normalization catalog asset moves, the computed-read audit inspected 97 canonical TypeScript case files and resolved 365 supported reads with zero missing paths. See `📓️normalization-runtime-catalogs-2026-09-09.md` for the 8 passing consumer tests and observed production planner reads; that report also distinguishes the unrelated later compiler-manifest failure.

The final combined fixture-resolution, physical-layout, and mutation-pair regression run passed all 84 tests and 194 assertions through Bun/Nx. This includes native Rust and TypeScript checks, filesystem execution, Nx input/discovery behavior, and the independent Ajv, minimatch and TOML oracles.
