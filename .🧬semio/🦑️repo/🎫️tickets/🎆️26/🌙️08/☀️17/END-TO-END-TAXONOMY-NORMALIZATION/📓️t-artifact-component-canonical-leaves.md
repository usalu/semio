# Artifact Component Canonical-Leaf Integration

## Scope

Baseline remains `9f449b10659b95148c8bcb3f91ce583bf7446973`. This packet implements canonical runtime discovery and authoring consumers for the component cohort recorded in `📓️h-artifact-leaf-authority.md` and its existing mutation directory-compaction authority from `📓️h-component-path-budget-authority.md`. No production artifact file was moved. Actual `compose/**` and `temp/compose/**` remain wholly opaque; no real Git mutation or worktree was used.

## Schema and Shared Renderer

`physicalLeafRendering` now declares forward-only kind-only filenames, longest registered source extension, schema-ordered primary authoring extension, and canonical-only runtime lookup. The shared renderer retains `.tsx`, `.d.ts`, `.grammar.semio`, and other complete format chains; it does not itself authorize any owner or move. Discovery, root authoring commands, plugin-registry surface authoring, mutation descriptor lookup, language mirrors, and active artifact policy filename selectors use file-kind authority rather than the removed physical `component` stem.

Optional mutation payload schemas remain distinct from the owner descriptor: the approved schema-owned `mutationPayloadSchemaLocation` is `🧬️schema/🔣️.json`, while the owner descriptor is `🔣️.json`. Both the authoring descriptor pointer and mutation structural discovery use that location; native Rust type fragments retain their type identity behind canonical `🦀️.rs#<Type>` pointers.

Root authoring imports are now side-effect-free: alias maintenance runs only for the actual main command. Root absent-leaf authoring now uses exclusive creation and preserves existing bytes/modes. The artifact/standard/subset operations are exported for direct isolated verification.

## Canonical Runtime Regression

The first canonical semantic-census test exposed a real basename-role bug: `🟦️.ts` was classified as package glue solely because a configurable entry uses the same filename, causing two real production consumers to become zero. The corrected implementation identifies package glue through discovered package ancestry, not the basename. Canonical TS and TSX owner leaves remain production consumers, and language mirrors resolve by file kind.

## Executed Evidence

The final ticket integration command passed **8 tests, 0 failures, 232 assertions in 64.43 seconds**:

```sh
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️artifact-component-leaf-integration.test.ts'
```

The language-neutral fixture covers 17 physical format cases and one actual energy-model mutation scenario. Ajv independently validates format identities and the emitted mutation payload schema; TypeScript's parser independently checks 19 active authoring/discovery consumers for forbidden component-filename selectors. The all-format source-to-canonical transaction preserves every fixture byte and mode and yields an empty replan. Folded destination occupancy and unknown semantic owners both reject application.

The energy scenario exercises real six-file source bytes and modes, the existing semantic mutation directory projection, structured Rust references, a deliberate after-edits rollback, retry, commit, canonical runtime discovery, and empty replan. Its filename-only destination exceeds 240 bytes; the composed semantic-directory projection keeps every destination at or below **204 bytes**. The isolated plan contains **10 moves, 6 reference edits, 1 registry regeneration, 0 unresolved items**. Nx ran both `@semio-tech/plugin-registry:generate` and `@semio-tech/plugin-registry:check-generated` successfully for this pilot and for the all-format roundtrip. Registry implementation inputs are present in the isolated fixture, not bypassed.

The retained successful evidence is `🧪️component-pilot-iY6OFq/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/ARTIFACT-COMPONENT-PILOT/🔣️component-pilot-evidence.json` inside this ticket. Its frozen isolated plan digest is `8bc8a5c66705a9a32d09b3d4365115b3698e344cd07aa193424a918b8d243ead`; source-tree digest is `fd746f141a99e46930020d63e72cc8b5b305afc1fbc96ca0cf23e674ff1cf9f4`. These identify the isolated proof, not a production apply authorization. All isolated fixtures and transaction journals remain inside this ticket.

Initial renderer TDD failed for missing exports, and canonical runtime TDD caught the basename-role bug described above. A final AST assertion additionally caught outdated diagnostic text; the diagnostic now renders the schema-owned Rust filename. Scoped `git diff --check` passed for the changed implementation and aggregate fixture files.

## Existing Aggregate Fixture Checks

The bounded existing semantic-collection/direct-mutation selection initially ran 25 tests: 22 passed, one hit its existing five-second timeout, one loaded an in-flight version-1 inventory test schema, and one exposed a concurrent generic mutation-inventory comparator missing its second argument. The schema-version and comparator issues were corrected; their exact rerun passed the schema case and exposed the next generic incoming-reference gap: TypeScript edges and the aggregate mount resolve, while external Rust command/catalog/registry/sibling/cross-owner edges remain absent. That cross-family closure is assigned to the following bounded implementation lane, not hidden with a fixture exception. The timing gate is unchanged. The entire aggregate suite is not claimed by this packet.

## Release Boundary

The implementation does not add runtime aliases for old component filenames. Exact physical source literals and normalization source-preimage contracts remain under the wider repository reference-closure/apply work; they are not fallback lookup alternatives. No production component move, whole-repository acceptance, or production apply readiness is claimed here. The representative lifecycle is executable and its source-to-canonical runtime roundtrip is complete.

## Owned Changes

- Taxonomy: `physicalLeafRendering`, optional payload schema location, mutation optional facet declaration.
- Discovery: shared physical leaf renderers, canonical component/manifest lookup, language mirror and structural package-role classification.
- Root router: import-side-effect guard; artifact/standard/subset/mutation authoring; active artifact filename consumers; payload descriptor and schema discovery.
- Plugin registry: shared primary-leaf wrapper and canonical mutation descriptor selector only; dependency/catalog generation is owned by the parallel registry lane.
- Permanent language-neutral `🧪️artifact-component-leaf-authority/🔣️.json`.
- Ticket-local `🧪️artifact-component-leaf-integration.test.ts` and retained disposable fixtures.
- Existing aggregate tests: canonical semantic-census and direct-mutation fixture regions, plus two exact optional-facet assertions; source-normalizer goldens are unchanged.
