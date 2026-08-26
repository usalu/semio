# S-Transaction Dispositions Engine

## Scope

Transaction plan/journal schema v2 implementation in the normalization engine, its permanent TypeScript boundary, and the language-neutral disposition golden. The live repository was not physically normalized; Compose and temp/Compose were not accessed.

## Frozen public boundary

- `TaxonomyPlan.schemaVersion` is exactly `2`; all seven operation arrays and `expectedAffectedPreStateDigest` are required.
- `parseTaxonomyPlan(value: unknown)` rejects schema v1, missing/unknown keys, malformed nested records, mismatched hashes/IDs, orphan dispositions, and incomplete embedded-root membership.
- Embedded-root IDs use the coordinator's acyclic correction: immutable group authority excludes child-ID arrays, child IDs bind the group ID, and the plan digest plus exhaustive sorted arrays bind membership.
- Journal schema v2 has required intent/result arrays, typed backups, strict parsing, and phase records.
- Inventory freezes numeric mode, size, and raw no-follow symlink target evidence.

## Permanent evidence

The golden `📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-dispositions/🔣️.json` serializes Windows-reserved/trailing-name cases without creating checkout-hostile nodes and includes POSIX/drive/UNC ownership cases.

Focused command:

```text
bun test '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts' --test-name-pattern='transaction dispositions v2'
3 pass, 0 fail, 16 assertions
```

Regression command:

```text
bun test '.../🧪️index.test.ts' --test-name-pattern='stale preimages block apply without changing source bytes|cancellation rolls back and a successful retry converges to an empty second plan'
2 pass, 0 fail, 19 assertions
```

Bundle command:

```text
bun build '.../🧹️normalization/🟦️.ts' --target=bun --outfile '.../normalization-v2-check.js'
Bundled 15 modules; success
```

## Active closure

The public parser/type boundary, typed backup encoding, no-follow digest, virtual sentinel golden, symlink disposition planning, and initial journal phases are present. Exact affected path-state v2 construction, full evidence/embedded-root detection, exhaustive preflight authority checks, cross-flavor symlink ownership, and complete resume-state reconciliation remain under active implementation; this report is not yet the completion claim.
