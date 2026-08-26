# S-CAD-DRAW-PROJECTION-ENGINE

## Outcome

The normalization engine now consumes the strict schema/discovery projection authority for the CAD example-model catalog and Draw editor-command bundle without duplicating mapping tables. Read-only live planning produces exactly 209 CAD file moves plus 11 Draw file moves, with zero projection-specific unresolved findings and zero projection mapping collisions. No live CAD or Draw path was applied.

The implementation also closes the reference, transaction, and convergence boundaries:

- source discovery is bound to the exact registered artifact owner and `🗿️artifacts/<artifact>` context;
- destination paths and rationale rules come only from `semanticPathProjectionAuthority`;
- the 99-occurrence audit surface is covered as 75 CAD structured edit records representing 76 occurrences plus the adjacent `Path::join`, and 23 Draw edit records;
- selector-bearing and selector-less structural forms require artifact ownership or one exact `semanticPathProjectionReferenceConsumerContracts` identity;
- broadened regexes cannot admit counterfeit consumers because identity, pattern, adapter, and form are conjunctive;
- the two Draw Cargo `lib.path` edits reconcile one-to-one with authority requirements, including path, adapter, semantic location, values, and preimage hash;
- post-apply verification checks exact projected descendants and scans schema-owned owner/consumer surfaces for stale CAD, Draw, and mutation hierarchy tokens even when a converged second plan has no projection moves;
- fixture transactions prove injected-failure rollback, cancellation without workspace mutation, successful commit, committed-journal resume, empty second plan, and stale-token rejection after convergence;
- strict engine loading runs the discovery validator first and then parses tagged fixed scopes, projection consumers/configurable descendants, fixed-name rejections, ecosystem package identities, package profiles, and source dispositions without fallbacks.

## TDD evidence

Initial red boundaries included: no projection planner hook; malformed Unicode regex; unregistered source scenario/member contexts; wrong artifact marker glyph; counterfeit source acceptance; unowned selector/prose rewrites; discarded non-path violations; incomplete CAD/Draw reference adapters; move-dependent stale scanning; discarded authority `lib.path` requirements; old fixed-scope parsing; an invalid non-package Cargo fixture; and generator-preview fixture replacement of unrelated strict generator ownership.

Final focused command:

```text
bun test '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts' --test-name-pattern='(artifact-editor-command-projection preserves|plans the exact CAD and Draw authority mappings|rejects unowned artifact prose|rolls back and atomically applies CAD and Draw projections|blocks a reintroduced mutation source token)'
```

Result: 5 pass, 0 fail, 139 assertions. The independent final reviewer ran the expanded boundary including CAD authority/fail-closed cases, Draw golden, mutation projection, exact plan/reference coverage, transaction/stale negatives, strict schema boundary, and generator preview: 10 pass, 0 fail, 312 assertions.

Generator-preview regression command:

```text
bun test '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts' --test-name-pattern='plans, applies, verifies, and converges an exact Nx-owned preview'
```

Result: 1 pass, 0 fail, 10 assertions; both generated-output and check targets completed through Nx. The fixture now retains all unrelated schema-owned generator contracts and adds only its lexically ordered fixture contract.

Strict engine schema-boundary negative: 1 pass, 0 fail, 2 assertions. It rejects a consumer identity mismatch and an extra configurable-descendant alias through the strict discovery validator invoked by the normalization trust boundary.

## Live read-only census

The census invoked `inventoryTaxonomy` and `planTaxonomy` separately for the two permanent golden source roots at baseline `9f449b10659b95148c8bcb3f91ce583bf7446973`, with no opaque digest because the registered `compose/` prefix is absent. It did not call apply.

```json
{
  "rows": [
    {
      "contractId": "artifact-example-model-catalog-v1",
      "expected": 209,
      "moves": 209,
      "projectionUnresolved": 0,
      "mappingCollisions": 0
    },
    {
      "contractId": "artifact-editor-command-bundle-v1",
      "expected": 11,
      "moves": 11,
      "projectionUnresolved": 0,
      "mappingCollisions": 0
    }
  ],
  "totalMoves": 220,
  "totalProjectionUnresolved": 0
}
```

Permanent golden parity remains the mapping authority. The frozen Draw values consumed directly from it are 11 files, 9 destination directories, 20 destination nodes, max path bytes 210, two authority reference edits, and digest `1f28fcc6e28e54001a9df6ce98b1c30b565cd42b824ed2491bb9b5e407b7436b`.

## Build and deterministic checks

```text
bun build normalization/🟦️.ts --target=bun --outfile <ticket>/🧪️cad-draw-projection-engine-build.js
Bundled 15 modules; 0.65 MB.

git diff --check -- <normalization-module> <normalization-test>
exit 0

rg -n '\[DEBUG\]' <normalization-module> <normalization-test>
no matches
```

SHA-256:

- normalization engine: `b3bc75814884ae6d815955a4a8a2c6e8a4de650bc18483b62ef535628b97b8fa`
- retained Bun bundle: `3ea54d79e81bd92529f2f990b1d7a112281ea47d6f95fc98e80ef8a7346ed3e0`

## Owned paths

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️cad-draw-projection-engine-build.js`
- this report

The permanent CAD/Draw golden fixture was consumed read-only. Taxonomy, discovery, CAD/Draw physical trees, Compose/temp-Compose, manifests, and Git state were not modified by this lane.
