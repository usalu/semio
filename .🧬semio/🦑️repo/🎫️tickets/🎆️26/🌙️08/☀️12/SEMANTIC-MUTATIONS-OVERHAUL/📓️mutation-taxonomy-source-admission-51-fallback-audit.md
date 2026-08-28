# Mutation Taxonomy Source Admission 51 Fallback Audit

## Scope

Read-only audit of [`📜️script.ts`](../../../../../../📜️script.ts) after the root source-admission selector repair. No inventory or filesystem scan was run.

## Remaining Admission Bypasses

1. `mutationTaxonomyFiles` at root lines 20656–20672 recursively re-enumerates every entry beneath each selected mutation facet. It reads filesystem membership after `mutationTaxonomySourceIndex` has built the admitted roster, accepts any regular file under that root regardless of canonical admission origin, and has only exact-case `compose` exclusion. `inventoryMutationTaxonomy` calls it at line 20917 for each selected root. This is the primary remaining fallback traversal.

2. `policyMutationStructuralBreaches` receives selected roots from `inventoryMutationTaxonomy`, so its outer root list is not rediscovered in that path. It nonetheless calls `policyListMutationDirs`, `existsSync`, `policyReadFileSafe`, and Rust reachability readers beneath each root. Those checks are structural facts, not content membership selection, but they can still observe files absent from the admitted source roster. They need an explicit admission-backed file/dir view before they can claim one-source snapshot closure.

3. Default-argument structural scanners independently reacquire a canonical admission through `policyFindAllMutationsDirs(repoRoot)`: `policyMutationDirectOwnerBreaches` (27796), `policyMutationImplPresenceBreaches` (27821), `policyMutationEmojiUniquenessBreaches` (28019), `policyMutationDispatchCoverageBreaches` (28181), and `policyMutationOutcomeBreaches` (28786). They then use direct directory/content calls. The global `policyMutationArtifactEngineBreaches` invokes structural and emoji defaults, while the outcome policy invokes its own default. These are repeated-admission/fallback flows, separate from the now single-admission mutation inventory path.

4. `policyRepositoryOwnedRoots` and broad `policyWalkRelFiles` remain for other policy families. They are not used by the repaired mutation source index, but they cannot be cited as mutation-inventory completeness authority because they retain independent topology and dot/build skip rules.

## Recommended Follow-up Boundary

Do not alter these paths in this consumer packet. A separate collector-projection packet should replace `mutationTaxonomyFiles` with the admitted file view and explicitly thread that same view through structural scanners. It should include a provenance-preserving directory projection for `policyListMutationDirs`; deriving only from filename strings would lose required physical directory facts.
