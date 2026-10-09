# Artifact I/O Infrastructure Report

## Architecture

Semantic schema trees contain domain declarations and behavior. Native wire ownership is representation-first: `🚪️io/{📝️text,💾️binary}/{📸️snapshot,🔺️diff,🧬️mutations,💡️inferences}/[semantic-member]`. Foreign codecs retain directional import/export trees. No migration allowance or semantic codec alias is admitted.

## Implemented

- Taxonomy native semantic collections and grammar/protocol mappings use representation-first I/O paths. Old schema wire mappings and migration-shape commentary were removed.
- Discovery rejects text/binary below every schema facet and accepts native representation/facet/member paths. Inference and foreign-artifact member lookup now supplies exact semantic parent context.
- Artifact authoring and publication preflight use representation-first ownership, with authored empty mutation/inference facets below each representation.
- Root schema policy now checks codec completeness under I/O. Root artifact policy also checks schema codecs, wire specifications, codec interfaces, codec mounts, public aliases and incorrectly ordered I/O paths.
- Production architecture verification covers plugin and framework artifact owners, excludes tests/fixtures/examples/oracles, and never follows symlinks.
- New JSON Schema and language-neutral path corpus are validated against independent Ajv and picomatch oracles. Synthetic native and invalid-owner fixtures exercise actual filesystem/source policy behavior.
- Registered `@semio-tech/repo-lib:test-artifact-io-ownership` and `lint-artifact-io-ownership` through package `📜️script.ts`, Nx, package scripts and both launch configurations.

## Verification So Far

`bun ./📜️script.ts test artifact-io-ownership` from the repo-library TypeScript package: **3 passed, 0 failed, 66 assertions**. Runtime logs confirm six independent filesystem/source violations are rejected while valid native codecs and literal/comment mentions are accepted.

The initial command from repository root crashed inside Bun before loading assertions (SIGTRAP, around 2 GB RSS). Running from the package directory avoids that crash and completes in under one second. Initial path corpus runs then caught missing semantic parent context for inference and foreign artifact members; the corrected discovery passed the corpus.

The live architecture gate currently fails during concurrent extraction and relocation. A red run after narrowing the gate to architectural concerns reported 383 violations: 203 aliases, 69 misplaced facet paths, 62 inline codec files, 39 wire specifications, and 10 codec mounts. Subsequent verification excludes test-owned adapters and includes framework artifacts. Final results will be recorded after coordinated moves finish.


## Final Infrastructure Validation

The final scoped router run (`bun ./📜️script.ts test artifact-io-ownership`) passed **4 tests, 0 failures, 86 assertions**. The tests independently compile the scanner, authoring and builder with the third-party TypeScript compiler, parse both launch configurations with jsonc-parser, validate the neutral corpus using Ajv and compare its accepted/rejected paths with picomatch. Filesystem cases now reject eight wire-ownership violations, including inline owner assembly mounting an I/O file inside a semantic `schema` namespace. Legitimate semantic reexports through a subset named `text` are accepted. Cancellation and progress are checked.

The registered Nx route (`bun nx run @semio-tech/repo-lib:test-artifact-io-ownership --skip-nx-cache`) also passed **4 tests, 0 failures, 86 assertions**, with a 3.2 second target duration. A subsequent Nx rerun also completed successfully with a 2.5 second target duration; no global resets or process cleanup were used.

SQLite is an optional representation (`representationDirs`); required native wire representations (`nativeRepresentationDirs`) remain text and binary. Both `ArtifactSqliteSnapshot` implementations and SQL files are rejected in semantic schema trees. TypeScript barrels cannot reexport native wire APIs. Schema-scope inventory no longer treats representation subtrees as semantic facet chains. The architecture scanner accepts authored aggregate dispatch implementations under I/O; its purity rules apply to semantic schema only.

The last intermediate live run, while extraction was still in progress, reported **71 schema codec files, 44 schema codec aliases, 38 incorrectly placed facets and one schema-to-I/O mount**. The durable details are in `infrastructure-current-violations.md`; these counts are intermediate evidence, not a final architecture pass. The main coordinator will rerun the live lint after all concurrent moves.

## Changed Files Owned By This Agent

- `📜️script.ts`: taxonomy schema/I/O validation, schema/representation policy and root artifact-I/O policy aggregation.
- `.vscode/launch.json`: test and live lint gate registration.
- `.vscode/🧩️launch.seed.jsonc`: matching ordered seed registration.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: representation-first collections/specs, SQLite vocabulary, required native representations and normative ownership comments.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`: representation ownership paths, exact member context, native representation validation and pure semantic schema inventory.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️builder/🟦️.ts`: representation-first scaffold publication authority.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🗿️artifact-tree/🟦️.ts`: native scaffold paths.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🟦️.ts`: new production scanner with progress/cancellation and source-aware ownership checks.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🚪️artifact-io-ownership/🔣️.json`: new language-neutral path corpus schema.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🚪️artifact-io-ownership/🔣️.json`: new accepted/rejected path corpus.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪶️artifact-empty-facet-authoring/🔣️.json`: handcrafted native I/O scaffold layout.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚪️artifact-io-ownership/🟦️.ts`: new oracle, source, filesystem and registration regressions.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts`: current representation-first taxonomy expectations.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`: test and live lint routes.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`: Nx targets.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`: Nx package routes.
- Ticket `infrastructure-report.md` and `infrastructure-current-violations.md`: durable architecture and verification evidence.

Generated raw output stays below this ticket's `🗑️generated` until the coordinator finishes final verification and removes the generated output folder. No AGENTS files, worktrees or modifying Git commands were used.


Final publication smoke test added and executed: `newScaffoldSubsetTree` wrote every declared representation-first file into a ticket-owned synthetic owner; bytes were read back, the complete output matched the handcrafted language-neutral authoring fixture, and optional SQLite was absent. The final router run passed **5 tests, 0 failures, 89 assertions**. The Nx route had already passed the four prior tests; the additional publication test was confirmed through its package router. Temporary debug logs were removed after runtime confirmation.
