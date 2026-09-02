# Duplicate Mutation Owner Key

## Intent

`duplicate-mutation-owner` enforces the breach rule that exactly one smallest semantic subset owns a mutation. Its `ownerOf` map is local to `mutationInventoryBreaches`; its only read is the duplicate check immediately after registration, so no consumer relies on an artifact-scoped key.

`owningSubsetOf` resolves `ManifestMutation.subset` before falling back to `MutationManifest.subset`. The ownership coordinate already derives from that resolved subset, so the key must reuse the coordinate rather than rebuild an artifact/standard string independently.

## Registry Scan

Scanned all 144 `**/🪆️subsets/*/🧪️oracle/🔣️.json` files. The cross-subset collisions under `s.stdio.semio@v1` are:

- `move-vertex`: `brep`, `mesh`
- `no-mutation`: `cad`, `document`
- `set-snapshot`: `cad`, `document`
- `remove-block`: `cad`, `document`

The scan also found independently scoped duplicate names in `s.stdio.pdf@1.7`; the canonical subset coordinate separates those too.

## Contract Result

`bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts contract` completed with 1,875 pre-existing high-priority breaches across other rules. Its emitted breach set contains zero `duplicate-mutation-owner` records globally and zero under `s.stdio.semio@v1`; no duplicate-owner breach was added.
