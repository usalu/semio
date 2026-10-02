# Normalization Service Extraction Closure

Read-only source inspection; no jobs. Owner base `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Serialization is a small complete cut

Normalization `🧹️normalization/🟦️.ts:1727–1753` canonicalJson owns three functions: canonicalArrayKey, recursive canonicalValue, exported canonicalJson. Only supporting type is JsonRecord (:446); Buffer byte comparison is system runtime. Preserve identifier priority order operationId/sourcePath/path/id/destinationPath/code/relativeRoot/structuredLocation, NUL joined keys, sorting only arrays whose every row has a key (including empty array vacuous case), undefined-object-key omission and array order otherwise. This is repository record canonicalization, not generic RFC canonical JSON. Do not replace with ordinary sorted-key third-party stringify without reproducing array semantics.

Move this exact closure into a canonical serialization owner under normalization, import directly from index/workflow and reexport from umbrella. The leaf must not import umbrella. Existing source admission membershipDigest :3047, plan/digest/transaction laws and captured inventory byte-digest tests are consumers; preserve bytes. Add closed nested-object/keyed-array/mixed-array/undefined rows with an independent JSON.parse/stringify + separately expressed byte-sort oracle, rather than rely only on implementation-mirroring tests.

## Source admission is a larger coherent cut

inventoryTaxonomySources :3052–3055 is not self-contained: it calls sourceAdmissionPrepareOptions, private normalization loadTaxonomy and collectTaxonomySourceAdmission. Required pure projection region :2467–2594 owns sourceAdmissionSafePath/Opaque/Record/InputShape/PhysicalConsistent/RepositoryFences/ContainingRepository/AssertRepositoryPath and projectTaxonomySourceAdmission. Required IO region :2761–3057 owns SourceAdmissionUnsafeAncestorError, SourceAdmissionPreparedOptions, lexical/root-chain/lstat preparation, cancellation, GitRecords/Exclusions/IndexObservation/GitRows/UntrackedRows, no-follow Walk/StructuralDirectories, Observation and collectTaxonomySourceAdmission.

Cross-region authorities: TaxonomyInventoryOptions :277; TaxonomyProgress/report :2444; source admission observation/index/origin/admission/inventory types :311–350; SOURCE_ADMISSION_ORIGINS; taxonomyScopedGitPathspec and its type; path/source normalization/exclusion helpers; sha256; canonicalJson; private LoadedTaxonomy and private loadTaxonomy :1707 (validated parseTaxonomy :929 plus semantic normalization). Filesystem, path, crypto and Git process APIs must retain safe Git env/no-follow root ancestor/repository fence/cancellation behavior.

Critical distinction: mutation currently imports discovery loadTaxonomy for semantic names, but inventory admission uses normalization's options-aware validated loadTaxonomy. They are not interchangeable. A clean split is (1) source-admission owned types+pure projection, (2) source-admission IO+fresh capture, (3) options-aware taxonomy authority parser/projection leaf shared with umbrella. Move genuine dependent definitions, not a port that calls back into umbrella from mutation; that would retain the expensive reverse closure. If parser extraction is staged, explicitly parameterize fresh validated taxonomy authority at the IO composition owner, and let a narrow canonical public capture entrypoint load it; mutation must consume that entrypoint, never inject an unchecked snapshot.

Preserve `inventoryTaxonomySources` API through umbrella explicit reexport, while captured-source imports narrow IO entrypoint/type. Index/workflow serialization direct imports then remove both reverse umbrella edges. Type-only TaxonomySourceInventory does not load umbrella if split alone, but runtime function still would; meaningful15s closure reduction needs both.

## Existing owned proof/harness

Pure fixture/test `🧹️normalization/🧫️fixtures/🚪️source-admission/🔣️.json` and `🧪️tests/🚪️source-admission/🟦️.ts`; IO fixture/schema `🧫️fixtures/🚪️source-admission/🧪️io/🔣️.json`, `🧬️schema/🚪️source-admission/🧪️io/🔣️.json`, test `🧪️tests/🚪️source-admission-io/🟦️.ts`. IO test :15–18 extracts declarations from umbrella source using TypeScript AST; extraction must update that authoritative declaration source to the new owned modules and retain all adapter inputs. Simply reexporting would make extraction fail. Existing physical source-index/root ancestor and inventory fast-glob parity remain independent evidence. No new compiler cohort required for this TS owner cut.

## Breach contract

BreachPriority and BreachRecord actual definitions are library 🟦️.ts:103–119, with priority high/medium/low and required id/summary/kind/scope plus optional source location/excerpt/autofixable/reason/solution. A domain-owned lint finding schema/type leaf is appropriate (library lint/reporting contract), imported type-only by mutation and explicitly reexported by facade/core. No existing exact closed BreachRecord schema owner was established by this inspection; do not claim one exists. Type extraction reduces dependency direction clarity but runtime reduction comes from mixed imports being split, not from already type-only imports.
