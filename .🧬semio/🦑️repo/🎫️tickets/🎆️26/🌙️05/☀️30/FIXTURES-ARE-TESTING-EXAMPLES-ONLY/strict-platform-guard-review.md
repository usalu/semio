# Current Test Platform Guard Review

Read-only Source3 review; no source edits/Cargo/tests run by this agent. Hashes in 🗑️generated/strict-platform-review-inputs.json. Root's prior executed tests are separate evidence; this review does not adopt them as proof of new candidates.

Confirmed current changes: sharedFixtureDir at316 and shared URI base at909 use owner/tests/fixtures; protocol TestLayoutFindingCode and TS finding constants agree on fixture-outside-tests. Bounded repo scan found no remaining fixture-in-test-case spelling outside tickets/generated exclusions. inspectTestLayoutSource now returns early for actual fixture collections, preserving inert parser specimens instead of executing their declarations.

## Concrete Semantic Bugs

1. Missing true-test ancestry carve-out. inspectTestLayoutDirectory:1661 and fixtureLayoutFindings:1676 use segments.indexOf(taxonomy.testsDirName) without module-member exclusion. Path `🧰️framework/🔨️modules/🧪️tests/🧫️fixtures/input.json` has fixtures index4 and tests index3; current condition tests<0 || tests>fixtures is false. Therefore a module named tests grants false collection ancestry. This counterexample has no true tests collection. Need same ancestor classification for both fixtures and tests, including vocabulary moduleMemberDirName.
2. testFixtureCollectionIndex:1650 uses literal 🔨️modules; taxonomy's moduleMemberDirName rename invariant already exists in schema-invariants. The new minimatch oracle at test-layout:29 likewise hardcodes modules and counts any tests path segment, sharing the semantic blind spot. Add renamed-module test plus module named tests nested fixture refusal; use taxonomy vocabulary in both production and independent expectation construction.
3. inspectTestLayoutSource:2073 sets inFixture = segments.includes(testFixturesDirName), so a genuine production module named fixtures suppresses assessment.finding even when testFixtureCollectionIndex correctly says no fixture collection. fixtureSourceDependencies:1827 also bypasses every raw fixtures-name segment. Example `framework/modules/fixtures/tests/case/extra/implementation.ts` loses depth finding because inFixture=true. Use actual collection classifier consistently, preserving direct module members.

## Case Validator Semantics

caseContractBreaches:1534-1536 rejects every nested directory under a runnable test case, including fixtures. This remains coherent with chosen canonical owner/tests/fixtures shared data; fixture folder under tests/<case>/fixtures is accepted by ancestry-only placement but rejected by case contents. Decide explicitly whether strict policy merely demands tests ancestry or also requires sibling shared collection. For the current canonical moves, keep strict direct implementation leaves and owner shared collection; change stale remediation wording (1536 says owner's fixtures instead of owner's tests/fixtures).

Discovery: discoverTestCases:295 recognizes basename tests without module carve-out; module named tests could be traversed as a test collection if child contains feature. Existing discovery has this older weakness independently of strict placement. Its fixtures child becomes a case only if authored feature file exists there; ordinary shared fixtures do not automatically register a case.

## Non-Adopting Counterexample Inputs

- named-tests-module: framework/🔨️modules/🧪️tests/🧫️fixtures/input.json → must refuse fixture-outside-tests.
- renamed-module-vocabulary: taxonomy moduleMemberDirName=🧱️units; framework/🧱️units/🧫️fixtures/implementation.ts → ordinary module source, not fixture collection.
- named-fixtures-module-test-depth: framework/🔨️modules/🧫️fixtures/🧪️tests/🧪️case/extra/🟦️.ts → must retain test-implementation-depth.
- canonical-shared: owner/🧪️tests/🧫️fixtures/specimen.ts with schema-shaped literals/test function text → inert fixture input, no inline executable test findings.

No counterexample was executed here, so findings are reasoned branch witnesses anchored to current source hashes, not passing/failing test claims.
