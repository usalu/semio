# Mutation Direct-Child Classification Research

## Exact Existing Authority

The existing captured taxonomy already distinguishes the required categories; no new root, skip list, or taxonomy key is warranted.

- [`artifactFacetPathIsDeclared`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:2416>) accepts `🧬️schema/🧬️mutations/<name>` only when `<name>` passes [`mutationDirectoryNameIsValid`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:2356>). It explicitly rejects `📝️text` and `💾️binary` directly below the mutation collection.
- [`artifactFacetChildLevel`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:2362>) permits the configured `mutationOptionalFacetDirs` only one level *below* an accepted mutation owner. Thus `🔺️diff`, `↩️inverse`, `🧩️plan`, `📝️text`, `💾️binary`, and `🧬️schema` are leaf facets, not direct collection owners.
- The taxonomy fixes `mutationOptionalFacetDirs` in canonical order and validates their registry membership in the discovery vocabulary audit at [`component.ts`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts:3498>). `semanticDirectoryKindId` can identify these fixed infrastructure names, but it cannot substitute for `mutationDirectoryNameIsValid`: arbitrary operation emoji/verb-noun names are intentionally not one global fixed directory kind.
- `semanticCollections["🧬️mutations"]` declares the collection as `mutation`; `schemaChildDirs` admits it under `🧬️schema`; `representationDirs` remain native codec categories elsewhere. The mutation ownership comment and `mutationOptionalFacetDirs` are the relevant direct-owner authority.

## Smallest Shared Contract

Add one pure captured-view classifier, for example `mutationStructuralDirectChild(view, mutationsRel, name)`, shared by direct-owner enumeration and structural scanning. It must derive only from the parsed captured taxonomy and the current admission observations:

| Captured direct-child state | Taxonomy test | Classification | Required accounting |
|---|---|---|---|
| regular directory | `mutationDirectoryNameIsValid(name, taxonomy)` | `direct-owner` | include exactly once as a leaf candidate |
| regular directory | `mutationOptionalFacetDirs.includes(name)` | `root-infrastructure` | emit a direct-root placement violation; never fabricate a leaf |
| regular directory | otherwise | `malformed-child` | emit a malformed direct-child violation, including dot-prefixed names |
| symlink, non-regular file, or observation without captured source bytes | n/a | `nonregular-or-unadmitted` | emit an admitted-source/reachability violation; never satisfy a leaf or codec |
| no observation | n/a | `absent` | do not enumerate it as a child; a mount or declared variant must instead fail the existing reachability/bijection proof |

The classifier's candidate universe must be the union of one-segment entries in `view.directories` (which includes a folder supported only by a nested admitted regular file) and one-segment direct symlink/nonregular observations from `view.admission.observations`. This preserves a symlink/nonregular direct child as evidence instead of silently dropping it, while retaining ordinary file-supported mutation folders. The classifier must separately retain direct root files as file evidence: a canonical aggregate `🦀️.rs`, `🟦️.ts`, or root schema file is never a malformed folder. A direct regular file whose name itself passes `mutationDirectoryNameIsValid` is an explicit missing-directory candidate; it is unresolved/violating evidence, never a fabricated leaf. An absent observation remains absent without an unconditional structural error; a declared aggregate variant or mount still fails the existing reachability/bijection proof.

Existing `policyStructuralMutationDirs` should become the `direct-owner` projection of that exhaustive classifier. It must no longer hardcode `📚️examples`, `📝️text`, `💾️binary`, or skip `name.startsWith(".")`. The same classification result should feed direct-owner, folder/variant, descriptor, and codec checks, so every accepted folder and every aggregate declaration receives an explicit disposition.

## Generic Codec Conflict

The present structural scanner treats `🧬️schema/🧬️mutations/📝️text/🦀️.rs` and `…/💾️binary/🦀️.rs` as aggregate codec sources. The existing artifact-facet declaration rejects those root-level paths: text/binary are permitted only under a concrete owner (`…/<valid-mutation>/📝️text` and `…/<valid-mutation>/💾️binary`). The new classification must preserve each root-level codec as violation evidence rather than skip it, while still allowing the scanner to report the current placement conflict. It must not relocate or delete any codec in this follow-up.

## Neutral Contract Cases

The schema-validated cases are retained in [schema](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-direct-child-classification-54/🧫️fixtures/🔣️schema.json) and [vectors](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-direct-child-classification-54/🧫️fixtures/🔣️vectors.json): valid direct owner; owner supported solely by nested regular source; aggregate root file evidence; a valid-name root file missing its directory; malformed hidden/unsafe children; root text/binary infrastructure; examples at the wrong level; symlink/nonregular valid-name candidates; and an absent name. The fixture distinguishes declared path grammar from captured regular child: a symlink may have a syntactically valid name but cannot be admitted as a leaf.

The ticket-only [revised reference receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-direct-child-classification-54/🧫️runs/reference-75ebefd0-1ee1-47cc-9594-da04a55ee21c/result.json) ran through scoped Bun/Nx. It validates the neutral schema with Ajv 2020 and invokes the current discovery `artifactFacetPathIsDeclared` and `mutationDirectoryNameIsValid` for all 12 cases: 64 assertions with first/final-equal source hashes, including the descriptor schema consumed by the extracted scanner.

It also executes the extracted current `policyStructuralMutationDirs` and full `policyMutationStructuralBreachesView` against a closed captured view. The old helper returns only `CoMpOsE` and `➕️add-node`; it silently omits `.hidden`, root `📝️text`, and root `💾️binary`. The old actual scanner produces no breach for those three omitted scopes. This is retained OLD scanner RED evidence, not a successful production classification result.

## Proposed Source Footprint

1. Root `📜️script.ts`: captured-view-only classification helper, `policyStructuralMutationDirs` projection, and the direct structural scanner's breach emission/candidate accounting.
2. Ticket controller and neutral fixture only: extracted actual taxonomy/classifier tests with closed admission facts, including no ambient filesystem calls.

Do not change `🔣️taxonomy.json`, `artifactFacetChildLevel`, `artifactFacetPathIsDeclared`, discovery APIs, or generic codec source placement in this packet. The fixed rules and existing mutation fields suffice; a new taxonomy key would duplicate authoritative vocabulary.

## Mounted Captured-View Classifier

The authorized root splice is now mounted in [`📜️script.ts`](/Users/ueli/Documents/semio/📜️script.ts). It imports the existing `mutationDirectoryNameIsValid` authority once, and `policyStructuralMutationChildren` builds one candidate universe from all one-level captured directory facts plus all one-level direct admission observations. The direct-owner projection is now `policyStructuralMutationDirs`; no hardcoded facet/name skip list remains in that projection.

The classifier retains root aggregate/schema files as evidence, treats a mutation-named regular root file as a high-severity missing-directory candidate, and lets a direct nonregular observation override an inferred directory. `policyMutationStructuralBreachesView` consumes the same classifications and emits `mutation/direct-owner` high breaches for malformed/unsafe/root-infrastructure/missing-directory/nonregular children. Absent observations remain non-errors until an existing aggregate variant or mount contradicts them.

The prior [old-omission RED receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-direct-child-classification-54/🧫️runs/reference-75ebefd0-1ee1-47cc-9594-da04a55ee21c/result.json) is preserved. The new [actual GREEN receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-direct-child-classification-54/🧫️runs/green-c5f6bc79-9043-4131-9775-5de9af595a4b/result.json) ran the extracted real classifier and full real structural scanner through scoped Bun/Nx against schema-validated captured views: 87 assertions across all 12 contract cases, with first/final-equal hashes. It proves the scanner now accounts invalid/unresolved children while retaining direct owners, canonical root files, and absent observations without fabricated leaf owners.

The root source capture was SHA-256 `c1c1f78360d2db91fc611b413ed6deabf4e144b324754c42e45a95b9ff7afb5c`. This is source/reference proof only; it does not establish a whole-repository scan or native runtime behavior.

## Contradictory-Admission Repair

The initial classifier still allowed an inferred directory fact to outrank a direct captured file or absent observation at the same child path. The neutral contract now adds three cases: inferred-directory plus direct file, inferred-directory plus absent, and an unsafe-ancestor admission already represented at this boundary as `unobserved`. It preserves the distinct ordinary absent non-error case.

The [pre-repair RED receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-direct-child-classification-54/🧫️runs/red-657170ad-f03a-4348-b720-a2c9510c607b/result.json) has 73 assertions and captures the actual defect at root SHA-256 `c1c1f783…`: both contradictory cases became `direct-owner` and generated no direct-owner breach. This is a correctness RED distinct from the retained historical omission observation.

The narrow classifier repair gives a direct file/directory conflict the unresolved `missing-directory-candidate` disposition, and a directory/absent conflict the `nonregular-or-unadmitted` disposition. Existing nonregular and normalized unsafe-ancestor (`unobserved`) observations remain nonregular/unadmitted. This follows the admission contract: `TaxonomySourceObservation` intentionally does not retain the candidate-only `unsafeAncestor` flag; unsafe ancestry is normalized to `unobserved` before this captured view receives it.

The [post-repair GREEN receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️mutation-direct-child-classification-54/🧫️runs/green-e1c48911-5139-469e-8326-90821a6ba46c/result.json) has 102 assertions over 15 schema-validated cases and exercises the extracted real classifier and scanner. The stable root capture is now SHA-256 `e41abcb93ee624c43b443d42b0848a100bddaf39f052519064c673327c1134d7`; first/final hashes match. The preserved structural fixture remains 67 assertions with the explicit root-binary placement violation, and the targeted classifier TypeScript range 27817–27855 has no diagnostic in the retained `type-diagnostics-44fb8e1e-fe95-44fd-931b-359d141953ad` receipt.
