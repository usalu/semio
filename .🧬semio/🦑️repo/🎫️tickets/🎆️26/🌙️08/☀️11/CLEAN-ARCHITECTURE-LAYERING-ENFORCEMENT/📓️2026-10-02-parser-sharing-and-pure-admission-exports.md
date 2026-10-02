# Parser Sharing and Pure Admission Exports

Read-only inspection of current-canonical-boundary-audit.md and current source. No tests/builds/compiler/generation performed.

## Actual retained full-suite result

`🗑️generated/goal-root/rust-source-direction-current-provider.log` ends with **17 pass, 0 fail, 979 expect() calls, 35.19s**. The retained finite expansion and scope laws report 23.33s and 19.89s respectively; concurrent durations overlap. This is a read retained receipt, not an independently executed replay.

## Content-derived matcher sharing

Normalization `🟦️.ts:1706–1720` caches Omit<LoadedTaxonomy,"path"|"input"> by freshly captured taxonomy contentHash, capacity eight. A cache hit reattaches the current physical path/input. The reused pathMatcher is `discovery/🟦️.ts:2718–2740`: frozen public method object with a private pattern→RegExp cache; expressions have no global/sticky flags, so test has no mutable lastIndex dependence. Normalization parse constructs this matcher at 937. Sharing it does not grant physical authority: it caches only pure normalized pattern compilation. No caller can reach its private expressions Map through the exported interface.

Cached schema/exclusions/fileKinds/directoryKinds remain mutable nested objects despite readonly TypeScript declarations. Current code inspection found no mutation proving a live defect, but public exposure of the full cached object would allow one caller to affect a later same-hash load. Keep parser/cache result internal to its owned module or expose immutable typed contracts; do not turn the content cache into a mutable public snapshot API. A fresh matcher per caller is unnecessary for current semantics and would discard the measured pure compilation benefit.

## Exact single-owner export partition

The symbol closure in current-canonical-boundary-audit.md is authoritative for moving whole declarations: parser 59 top-level declarations and load 70 (including types). Move that exact parser closure once, not a second abbreviated admission parser. Export parseTaxonomy, the options-aware loadTaxonomy, LoadedTaxonomy and the taxonomy schema types currently needed by normalization. Export a narrow `{repoRoot:string; taxonomyPath?:string}` load option instead of importing TaxonomyInventoryOptions/OpaqueTreeDigest merely for a Pick. GeneratorInputTaxonomy and loadNormalizationTaxonomy (1693–1700) can move to the same canonical owner, with original public facade exports delegating by direct reexport if still required. No callback to normalization should remain.

Pure admission closure is 23 declarations in the boundary audit. Public consumer exports: projectTaxonomySourceAdmission; TaxonomyNodeKind; TaxonomySourceOrigin/ObservedKind/IndexEntry/GeneratorOutput/CandidateObservation/AdmissionInput/Observation/AdmissionDiagnostic/Admission. IO needs direct exports of SOURCE_ADMISSION_ORIGINS, sourceAdmissionByteCompare, sourceAdmissionSafePath, sourceAdmissionOpaque, sourceAdmissionRepositoryFences, sourceAdmissionContainingRepository and inScope. sourceAdmissionAssertRepositoryPath (2495) and taxonomyScopedGitPathspec/TaxonomyScopedGitPathspec (2577/2604) are additional IO dependencies outside the projector seed: give them one owned implementation and direct imports, rather than duplicating them in the IO extraction. normalizeRelative/sourceRelative are shared path helpers; move once or retain a separate owned path leaf used by parser and pure admission.

LoadedTaxonomy requires discoverySchema, pathMatcher, input, schema, exclusions, fileKinds and directoryKinds. Parser depends directly on discovery validation and schema projection parsers, canonical JSON and node:path; loader additionally requires fresh semanticOwnedInputFileSnapshot and lexical/no-follow helpers. Preserve raw path validation/no-follow behavior: current assertLexicalInputOutsideOpaque (2186) normalizes with resolve before checking ancestry, whereas admission prepares with stricter raw checks and root ancestors. Extracting the loader must preserve that distinction honestly rather than claiming standalone load has admission's full physical guarantee.

## Actual remaining broad dependency

Current `normalization/🧬️mutation/📸️captured-source/🟦️.ts:5` imports inventoryTaxonomySources at runtime from normalization. Index line4 imports TaxonomySourceInventory type-only; its canonicalJson and metadata/discovery runtime imports are already narrow. No current normalization runtime import of the mutation subtree was found in the inspected normalization module; therefore the exact earlier normalization→mutation→captured-source→normalization cycle should not be reported as a presently established direct cycle. The proven current issue is mutation captured-source loading the entire normalization provider and its broad closure.

Parser and pure projector extraction alone reduce that closure but do not remove captured-source's runtime dependency: inventoryTaxonomySources still lives in normalization at3026. Complete the IO partition from the adjacent landed-json-and-admission-type-closure report, then retarget captured-source to the canonical IO owner and Index's type import to the owned contracts. This avoids an adapter whose public inventory function immediately calls the normalization umbrella.

## Tests and declaration injection

Keep the source-admission pure corpus/schema and source-admission-io corpus/schema owners. Retarget actual AST declarations by symbol owner; the IO harness also extracts normalization orchestration functions, so a single blanket source-path swap is incorrect. New owner import/export shape must supply the exact static types above, not inferred provider callbacks. No passing claim for the upcoming extraction is made here.
