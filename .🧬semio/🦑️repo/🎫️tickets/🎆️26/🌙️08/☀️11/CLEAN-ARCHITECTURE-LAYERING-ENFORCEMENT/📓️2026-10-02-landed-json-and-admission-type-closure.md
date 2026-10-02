# Landed JSON and Source Admission Closure

Read-only source inspection; no tests, compiler, Nx or generation executed. Root reported the pre-matrix JSON provider GREEN 3 laws/104 assertions; the new matrix and registered route remain unverified here.

## Serialization

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧾️serialization/🔣️json/🟦️.ts` is an import-free leaf containing the original three-function closure. Null-prototype object accumulation preserves an own `__proto__` member, and top-level undefined/function/symbol now throw instead of violating the declared string result. Integer object property enumeration still follows native JSON.stringify; object key sorting remains UTF-16, identity-bearing array sorting UTF-8 bytes. These are intentionally distinct contracts.

`library/🧪️tests/🧾️canonical-json/🟦️.ts:14–23` evaluates the actual owned source with runtime import, Bun transpilation and TypeScript transpilation. Each matrix retains all 17 JSON and five runtime rows. This proves compilation/output parity; the compilers do not independently implement canonicalization. The independently handwritten prepared oracle values and jsonc-parser serialization supply that separate witness. The parseTree/findNodeAtLocation checks at lines 33–39 avoid jsonc-parser's plain-object proto assignment limitation. Negative schema variants retain the full row cardinalities, so their rejection is no longer masked by a shortened cases table. No concrete new source defect found in this bounded contract.

## Exact admission partition

All following line locations refer to `library/🧹️normalization/🟦️.ts` at inspection.

1. Shared owned contracts: TaxonomyNodeKind (50), TaxonomyProgress (271), source contracts (294–352), TaxonomyScopedGitPathspec (2577), CollectedTaxonomySourceAdmission (3293). Move definitions once; normalization may import/reexport the owned declarations. Admission options need only repoRoot, scope, ticketDir, cancelFile, structuralDirectoryNames, progress and taxonomyPath (279–290). workers, baselineCommit and excludedTreeDigests belong to broader normalization operations and need no dependency in the leaf.
2. Pure admission owner: sourceAdmissionByteCompare (2443), SafePath/Opaque/Record/InputShape/PhysicalConsistent (2445–2482), RepositoryFences/ContainingRepository/AssertRepositoryPath (2483–2500), SOURCE_ADMISSION_ORIGINS, projectTaxonomySourceAdmission and taxonomyScopedGitPathspec (2604). Preserve exact ordered origins, stage/mode/objectId observations, unsafeAncestor refusal, gitlink boundaries, generator inclusion and diagnostic output. Do not replace the projection with an IO callback.
3. IO admission owner: the complete 2736–3030 region, including PreparedOptions, UnsafeAncestorError, index observation cache/receipt, GitRecords/Exclusions/Rows/UntrackedRows, DirectoryChain/Lstat, Walk/StructuralDirectories, Observation, collect and inventory. Also move its private lstatOrNull (8689), cancellation error (2422) and a narrow progress emitter compatible with TaxonomyProgress. The existing report signature also references Plan/Apply options; importing those broad interfaces into admission is unnecessary.
4. Options-aware taxonomy owner: LoadedTaxonomy (858), parseTaxonomy (931), its exact schema declarations and helper closure, TAXONOMY_RELATIVE_PATH, opaque contract table, PARSED_TAXONOMIES/capacity and loadTaxonomy (1709). This must be a single canonical provider used by both admission and normalization, not an injected facade callback and not a second parser. It depends on discovery validation/path matcher/projection parsers and semanticOwnedInputFileSnapshot; retain those direct owners. Parsing cache stores only pure content-derived data; every load still obtains fresh no-follow bytes/contentHash before reuse.

Admission consumes only taxonomy.path, input.contentHash, exclusions[*].path, schema.generatorContracts[*].outputRoots[*].path/inclusion, schema.fixedDirectoryContracts["nested-git-metadata"].pathPattern and pathMatcher.matches. Expose a typed admission projection of the canonical loaded result if desirable; never accept an arbitrary unvalidated lookalike as public source authority. collect currently asserts exact prepared taxonomy path plus input presence (2971), and public inventory prepares then loads itself (3026–3030).

## Freshness and physical facts that must move intact

PrepareOptions performs raw lexical validation before resolution, root ancestor no-follow checks, tracked enumeration/gitlink fences, then taxonomy regular-file admission (2789–2814). Lstat rechecks ancestor dev/ino/mode after observing the leaf (2769–2786). Walk and structural enumeration recheck directory identity and timestamps (2890–2952). The indexed-row cache is conditional on fresh physical Git authority snapshots and matching before/after receipt (2840–2875); unsupported Git environments, includes, split/sparse indexes, alternates or linked metadata disable reuse. This state belongs to the new IO owner, not an umbrella-global callback.

CanonicalJson extraction removes the serialization reverse import, but captured-source still imports inventoryTaxonomySources from normalization. Moving inventory plus its own parser closure is required to break that cycle; simply exporting inventory from a new leaf which calls the umbrella restores the same cycle.

## Exact test retarget requirements

`normalization/🧪️tests/🚪️source-admission/🟦️.ts` should import the pure projector's canonical owner while retaining its existing fixture/schema.

`normalization/🧪️tests/🚪️source-admission-io/🟦️.ts:14–23` reads umbrella source and finds top-level declarations by AST name. Retarget declaration lookup across the pure/IO/taxonomy owner files; declaration text must come from the actual implementation. Its injections at lines 29–78 include GitRows, UntrackedRows, DirectoryChain, Lstat, Observation, Walk, collect, inventory and PrepareOptions. The GitRows harness presently injects the older four-dependency shape while the current implementation also references sourceAdmissionIndexObservation, sourceAdmissionIndexObservations and canonicalJson; these exact newly landed dependencies need explicit owned declarations/injections, rather than allowing free names to resolve by accident. The collect harness uses JSON.stringify as its canonicalJson injection (71); retain that existing fixture behavior but do not count it as the owned canonical serializer proof.

The same file also extracts explicitTicketRows and inventoryTaxonomyWithSourceParentPruning (78/90), which remain normalization orchestration. Keep those lookups on normalization; a blanket sourcePath replacement would silently lose their declarations. This mixed-owner declaration registry is the smallest coherent harness change.
