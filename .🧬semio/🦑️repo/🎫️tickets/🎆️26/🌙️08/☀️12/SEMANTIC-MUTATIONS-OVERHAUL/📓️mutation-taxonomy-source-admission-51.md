# Mutation Taxonomy Source Admission 51

## Narrow Declaration Footprint

Only root [`📜️script.ts`](../../../../../../📜️script.ts) and ticket-owned neutral evidence changed. The root declaration footprint is intentionally limited to:

- import `inventoryTaxonomySources` and `TaxonomySourceInventory` from the canonical normalization authority;
- add the explicit optional `ticketDir` field to `MutationTaxonomyInventoryOptions`, forwarded only when the mutation CLI has explicitly received a ticket directory;
- add private `mutationTaxonomySourceAdmission(repoRoot, options)`, which acquires exactly one canonical inventory admission and rejects any non-complete result before content selection;
- export the pure, injection-capable `mutationTaxonomySourceFiles(admission)` and `policyFindAllMutationsDirs(repoRoot, admission)` selectors;
- make `mutationTaxonomySourceIndex` acquire one admission, pass it to both selectors, recheck each selected regular file without following links before the existing content read, and bind `membershipDigest` plus `taxonomyContentHash` into `sourceTreeDigest`.

`MUTATION_TAXONOMY_SOURCE_SKIP` is removed: it had no remaining caller after the two old filesystem walkers were retired. No structural mutation analyzer, parser, normalizer, scaffold, Plugin input, or global inventory command was changed or run.

## Neutral Contract

The schema and vectors live in [`🧪️mutation-taxonomy-source-admission-51`](../🧪️mutation-taxonomy-source-admission-51):

- a root-level authored `📜️script.ts` is selected;
- authored paths under `🧬️authored`, `build`, and `target` are selected from admission instead of inferred root/segment skips;
- an admitted ignored-generator file is retained with its own canonical provenance;
- an absent historical row and a symlink leaf are not selected for content reads and the absent row cannot create a mutation facet;
- a rejected/conflicted admission throws before either selector produces a result.

The ticket controller parses the actual root TypeScript AST, checks that the two injected selectors contain no filesystem traversal/read identifiers, then imports and invokes the actual exported root functions with the fixture admission and an intentionally nonexistent `"/forbidden/filesystem"` repository argument. It therefore exercises the mounted selectors rather than a copied membership algorithm.

## Retained Source Runs

The initial scoped Bun/Nx source red was real code-shape evidence: the old root functions still referenced `policyRepositoryOwnedRoots`, the obsolete skip constant existed, and no admission seam existed. It completed with `13` assertions. The first controller execution exposed and corrected a strict Ajv fixture-schema defect (`pattern` without a string `type`); that setup failure is retained in terminal history and is not presented as an admission red.

After the narrow cutover, the same controller completed `15` assertions:

```text
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-taxonomy-source-admission-51/📜️script.ts' source-red
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-taxonomy-source-admission-51/📜️script.ts' source-green
```

The green run used only injected fixture admissions; it did not enumerate the worktree or run a mutation inventory while the normalizer IO collector remained under its separate release.

| Input | SHA-256 at Green |
| --- | --- |
| root `📜️script.ts` | `95af58d1920c20db8416f01013ca852c23d9239df1c59f75788de7743596afdb` |
| canonical normalizer `🧹️normalization/🟦️.ts` | `7e5275065ec8c5b545e3a03f2d6efd243ddff8ac30636827d6f840ca65a6acac` |
| controller | `dff416592ec5a84ddadf8964411398c975e6e9c929fc0fa12685cf2102c5800a` |
| neutral schema | `e459e308525ccec76a8627d420b7d1ad13d55469381b77df500f4d0c45b25389` |
| neutral vectors | `889e9a2d67b804a99c90eb41d171424f4b6bf46bc9336681d523f52321daab4a` |

## Status

This is a source/neutral selector proof, not a full mutation-taxonomy inventory, compiler, or native-runtime claim. A final registration/replay must occur only after the canonical inventory collector is released and the root reviewer chooses the coherent normalizer hash.

## Scope And Snapshot Repair

The initial mounted version forwarded `options.scope` to canonical admission. That was incorrect: mutation scopes are a comma-separated local root selector and must not truncate the one canonical membership admission, especially where an external catalog/registry consumer is evidence for a selected mutation leaf.

The repair keeps one full admission, validates each requested comma-separated root locally, then selects matching mutation roots while retaining all admitted evidence files. `mutationTaxonomySourceIndex` is now exported solely for this real injected-admission regression and takes an optional already-captured admission; normal callers still acquire exactly one canonical admission. It captures each selected regular file through `semanticOwnedInputFileSnapshot`, which verifies a no-follow ancestry and descriptor-stable read in one existing repository-owned primitive.

The same repair also makes every case-folded `compose` path segment invalid before a scope/input path can reach filesystem logic. Facet discovery is byte-sorted and only accepts an observed directory named `🧬️mutations` or that directory as a proper ancestor of an observed regular file. A regular file named `🧬️mutations`, absent history, and symlink leaves cannot create a facet.

The behavioral injected-admission green completed `25` assertions through scoped Bun/Nx. It exercised two selected roots plus an external consumer from one actual ticket-local workspace, case-folded Compose scope rejection, absent/symlink exclusion, directory-vs-file facet formation, and snapshot-backed content capture. It did not enumerate the repository.

The scope defect was identified from the already-mounted source (`options.scope` was passed directly into `inventoryTaxonomySources`) after the first packet green; there is no retained pre-fix executable behavioral red for this narrower correction. The `13`-assertion initial red remains evidence for the prior hardcoded-root/skip implementation only. This distinction is intentional and no reconstructed/copy implementation is presented as an actual red.

| Input | SHA-256 after Scope Repair Green |
| --- | --- |
| root `📜️script.ts` | `30316df307deb556d426c6a4e0a6cfb59e5bbf6bc41e46dc16e64c8a2a1040f0` |
| canonical normalizer `🧹️normalization/🟦️.ts` | `342e780b71b6bd0fc9e6cc66b151e58fa9e78ecf0e149846ab297fe62659b0fe` |
| controller | `862f4b6d0bd01d9bfc3ba5f4c2b96085f2adf029593521ed404dc0fc8d868c72` |
| neutral schema | `a21147349535287261d8e44d1387ae42c7fa7b6acf17bed7047c9ed77b0299cd` |
| neutral vectors | `244e309123ac72da5d82230df4a35c9a7da5bc3b43a5a82123fab8df47a46766` |
