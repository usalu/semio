# Mutation Taxonomy Files 52 Prewrite

## Exact Write Boundary

Only root [`📜️script.ts`](../../../../../../📜️script.ts) and the existing ticket-owned `🧪️mutation-taxonomy-source-admission-51` controller, neutral fixtures, captured run JSON, and reports will change.

The change removes the `mutationTaxonomyFiles` recursive fallback from `inventoryMutationTaxonomy`. The existing `MutationTaxonomySourceIndex.files` roster will become the only inventory file projection, restricted by the selected root prefix. No new scanner, schema load, filesystem abstraction, normalizer change, or structural-analyzer change is authorized.

## Test-First Proof

Before the replacement, the controller will parse and extract the actual current `mutationTaxonomyFiles` body, execute that exact body with an in-memory filesystem stub, and verify it returns an admitted leaf plus an ignored/unadmitted leaf and a nested `CoMpOsE` child. It will separately extract the actual `inventoryMutationTaxonomy` body and verify the old seam invokes `mutationTaxonomyFiles`.

The green path will invoke the actual exported source index with injected canonical admissions and ticket-local regular files. It will prove that the admitted indexed file set excludes the unadmitted and mixed-case Compose paths; that new, removed, and mode-changed membership identities produce a different endpoint `sourceTreeDigest`; and that no content fallback traversal remains in the inventory body.

Controller workspace discovery will walk upward from its own path to the `.🧬semio` ancestor under no-follow checks, rather than relying on a fixed `../../` depth. Each red/green execution will retain its result as a ticket JSON record.
