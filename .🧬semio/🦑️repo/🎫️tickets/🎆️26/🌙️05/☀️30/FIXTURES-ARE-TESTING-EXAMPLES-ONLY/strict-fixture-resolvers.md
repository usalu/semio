# Test Platform Resolver and Contract Audit

Read-only snapshot; source hashes in 🗑️generated/strict-resolver-inputs.json. No data moves, source edits or execution.

## Exact Coordinate Computations

All paths below relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules`:

- `🧪️test/🟦️.ts:316` discoverTestCases computes sharedFixtureDir = join(ownerAbs, taxonomy.testFixturesDirName). Insert taxonomy.testsDirName before fixtures.
- `🧪️test/🟦️.ts:909` resolveTestInputs shared:// computes owner/fixtures while asset:// computes owner/assets. Change only shared branch to owner/tests/fixtures; schema:// remains actual contract lookup.
- `🧪️test/🟦️.ts:1336` mutationVectorRegistryBreaches fixtureRoot = sourceOwner/fixtures/mutations. Insert tests; casePath is already sourceRoot/mutation/tests/scenario and should remain unchanged.
- `🧪️test/🟦️.ts:1531,1536` missing-fixture remediation and unknown-case-child detail repeat owner fixtures location; update text.
- `📚️library/🔍️discovery/🟦️.ts:2271` projected mutation fixture path constructs sourceOwner/fixtures/mutations/canonical/scenario; insert tests.
- `📚️library/🔍️discovery/🟦️.ts:4839` exactFixture grammar must add `{kindId: "tests", literal: "🧪️tests"}` between subset and fixtures, in sync with taxonomy mutationCatalogProjection.fixtureSegments.
- `📚️library/🧹️normalization/🟦️.ts:4408` mutationFixtureMirrorOwnerPath splits /fixtures/mutations and would retain /tests in the synthetic schema-side owner after relocation. Match /tests/fixtures/mutations as the mirror prefix and strip both before replacing with /schema/mutations.
- `📚️library/🧹️normalization/🟦️.ts:4463` provenMutationLeafOwners derives schema-side sibling root from dirname(dirname(vector)), now tests. Strip tests boundary before selecting owner/schema/mutations. Scenario child recognition 4472 still uses fixtures immediate ancestry and should remain valid.
- `📚️library/🧹️normalization/🟦️.ts:4851,4852,4913,4918` canonical mutation fixture source/destination/expected roots and scenario recognition all construct or match /fixtures/mutations; insert tests everywhere consistently.
- `📚️library/🧹️normalization/🟦️.ts:7246` mutation scope resolver selects schema or fixtures and slices the owner prefix. A relocated fixture prefix now includes tests and needs explicit owner correction. Do not modify schema branch.

`mutationFixtureRoot` literal symbol found in library/🧪️tests/🧬️mutation-authority/🧫️fixture/🟦️.ts:22 creates an isolated temporary workspace using SEMIO_TEST_ARTIFACT_DIR and mkdtemp; it is unrelated to canonical persisted fixture collection placement.

Concrete scaffold test update: 🧪️test/🧪️tests/🧭️fixture-resolution/🟦️.ts:15 sets discovered.sharedFixtureDir to owner/fixtures. Test-platform corpus cases at 1474ff also construct old shared paths; update positive fixtures and expected resolved coordinates, preserving traversal and symlink negatives.

## Neutral Root Contract Readers

Current `🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json` has 78 `$defs`. No corpus-only Cases/Law/Corpus definition names appeared in the current root definitions.

Actual readers validating fixture examples against domain contracts:

- neutral test adapter tests `🔌️adapter/🧪️tests/🟦️.ts:13` compiles #/$defs/Scenario; :19 compiles #/$defs/TestEvidence and validates fixture.evidence.cases[*].data plus owned validator agreement. These are test execution configuration API, preserve them. The same test asserts forbidden fixture definitions absent.
- neutral mutation inventory tests `🎮️mutation/🏭️inventory/🧪️tests/🟦️.ts:12` compiles #/$defs/RuntimeMutationInventory and validates fixture.cases[*].expected. This is actual wire output, preserve it.
- repo test-platform `contributionSchemaProblems:726` dynamically selects definitions from CONTRIBUTION_RECORD_DEFINITIONS for parsed contribution records. These are actual registry/contribution configurations, preserve them.
- SchemaTestExample/SchemaFixtureTarget/SchemaStageExpectation are protocol configuration consumed by discoverSchemaFixtures (:5586), runSchemaFixture (:5495), schemaFixtureMetadataProblems (:5570); discovery reads declared `schemaFixtures` from contribution files and resolves subject contracts through schema export identity. No AJV/#/$defs/SchemaTestExample selector against a separate example corpus was found in the bounded neutral/repo test-platform TypeScript scan. Their presence alone is not proof of a forbidden separate fixture corpus schema.

Keep TestInput, TestLayoutFindingCode, TestCasePlan, TestEvidence, Scenario and their public config/report support types. Plain fixture examples may validate against these actual contracts, and corpus shape should remain incidental data.
