# Mutation Taxonomy Files 52 Admission Projection

## Completed Boundary

`inventoryMutationTaxonomy` no longer calls `mutationTaxonomyFiles`; the recursive helper is removed. For each selected mutation root it now projects `before.files` from the already-captured `MutationTaxonomySourceIndex` by root prefix. It neither rescans a facet nor loads another taxonomy/schema authority.

The separate structural scanner fallback inventory documented in [`mutation-taxonomy-source-admission-51-fallback-audit.md`](mutation-taxonomy-source-admission-51-fallback-audit.md) remains unchanged and out of this packet.

## Actual Red And Green

The ticket controller extracted the actual pre-cutover `mutationTaxonomyFiles` TypeScript declaration, transpiled that exact body, and executed it with a closed in-memory filesystem stub. Its neutral tree contained one admitted leaf, one ignored/unadmitted leaf, and `nested/CoMpOsE/escape.rs`. The actual old function returned all three. The controller also extracted the actual `inventoryMutationTaxonomy` body and verified its former call seam. This retained `fallback-red` completed `10` assertions.

After removal, `fallback-green` completed `31` assertions. It invokes the actual exported source index with injected complete admissions and ticket-local regular files; no repository inventory or global scan was run. The green proves:

- two comma-separated root selections retain both roots and an external catalog consumer from one full canonical admission;
- absent rows, symlink leaves, a regular file literally named `🧬️mutations`, and the ignored/mixed-case Compose red inputs cannot re-enter indexed files or create phantom facets;
- `Compose`, `cOmPoSe/child`, and `owners/COMPOSE/child` reject before capture;
- a newly admitted file, removed admitted file, and changed admitted mode each change endpoint `sourceTreeDigest` through the bound membership identity;
- the source index reads admitted bytes through `semanticOwnedInputFileSnapshot` and the actual inventory body contains no fallback call.

Commands and retained machine-readable results:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-taxonomy-source-admission-51/📜️script.ts' fallback-red
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-taxonomy-source-admission-51/📜️script.ts' fallback-green
```

- [`fallback-red.json`](../🧪️mutation-taxonomy-source-admission-51/🧫️runs/fallback-red.json)
- [`fallback-green.json`](../🧪️mutation-taxonomy-source-admission-51/🧫️runs/fallback-green.json)

## Final Captures

| Input | SHA-256 |
| --- | --- |
| root `📜️script.ts` | `418662067e8737ec806dfb334f6131559adf86419e7cf266b3788f43399f43f6` |
| canonical normalizer | `342e780b71b6bd0fc9e6cc66b151e58fa9e78ecf0e149846ab297fe62659b0fe` |
| controller | `e7655402541ef5e50501331e7022ae78254702040da511402c4f62dcc843e54a` |
| neutral schema | `0e4a04d9a53fc5a5f275620c3ff3b7dbf3c13efb5a56e87cadd48cd3f1a45de1` |
| neutral vectors | `9e8a42bdf0dcc8a36ede779c49ffe2b1fdff5c6272520fd634412de62a096e24` |

No Cargo, native test, Plugin/normalizer edit, or global inventory was performed.
