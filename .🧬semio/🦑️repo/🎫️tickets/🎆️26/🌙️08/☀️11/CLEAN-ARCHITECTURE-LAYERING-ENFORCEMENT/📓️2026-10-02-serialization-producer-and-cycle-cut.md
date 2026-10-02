# Serialization Producer and Cycle Cut

Read-only source inspection; no jobs. Current full17 remains RED; concurrency is not a complete receipt.

Owner base `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Exact smallest production closure

Normalization 🟦️.ts:1727–1753 owns canonicalArrayKey, canonicalValue and canonicalJson; JsonRecord is local alias Record<string,unknown>. No other normalization import/helper is needed. Move these definitions to a specific canonicalization/serialization leaf, direct-import from mutation index/workflow, and explicitly reexport canonicalJson from umbrella. Leaf uses system Buffer only. Object keys use current JS sort, keyed record arrays Buffer byte sort. Retain exact key identity priority and undefined omission; scalar/ordinary arrays are preserved. Existing implementation returns JSON.stringify result despite declared string for top-level undefined—do not silently widen/change this behavior as incidental extraction; own an explicit schema-first contract if changing accepted input domain.

## Existing proof and missing direct wire owner

Existing canonicalJson checks are mostly plan/source digest/transaction comparisons in workspace-contract and physical-reference tests, not an independent closed serialization corpus. Source-admission IO harness injects JSON.stringify for canonicalJson (:71), so it cannot prove repository keyed-array ordering. A dedicated closed facet is required before calling exact serialization behavior independently proved.

Minimum fixture rows: object nested key order; keyed row array permutations; Unicode byte-order identity; mixed keyed/unkeyed array preserves order; multiple identifier fields priority; empty array; undefined object property via explicit authored input constructor (JSON cannot encode undefined); duplicate identity stable order; scalar null/boolean/number/string. Third-party oracle can use existing TypeScript compiler execution of a separately authored reference expression plus ordinary JSON serialization, but avoid claiming JSON.stringify alone proves keyed-array semantics. An installed stable-stringify library, if already present, validates object sorting only; repository row-array preprocessing remains separately specified. Do not add runtime dependency.

Producer stays existing package 📜️script.ts, registers canonical serialization owner test source with original default15s command budget; project/package/launch seed dispatch that command. No new script filename, ad hoc shell runner or native cohort. Full workspace-contract collection imports owner laws so extraction retains collection coverage. All generated diagnostics remain ticket artifacts.

## Source-admission next exact partition and cycles

Pure region2467–2594 contains projector and closed shape/no-follow observation consistency functions; IO2761–3057 contains current fresh Git/filesystem capture. Move source-admission types/index/origin/observation/admission/inventory and minimal capture options/progress contract with them. Required cross-region functions are taxonomyScopedGitPathspec, lexical relative/source path helpers, inScope/isExcluded, sha256/canonicalJson, report, options-aware LoadedTaxonomy/parser loader. Those must be direct owned leaves, not imported back from umbrella. Normalization's loadTaxonomy1707/parseTaxonomy929 differs from discovery loadTaxonomy; preserve options-aware taxonomy authority.

After serialization cut, captured-source still runtime-imports inventoryTaxonomySources from normalization umbrella, and index imports TaxonomySourceInventory type from umbrella (erased) plus captured-source; workflow no longer runtime-imports umbrella canonicalJson but captured-source/evidence still do. Thus normalization umbrella -> mutation workflow/evidence/captured-source -> normalization umbrella cycle remains until admission capture moves. Splitting canonicalJson alone is valuable but not a full closure deletion.

Pure source-admission test currently imports projector from umbrella. IO test has sourcePath URL ../../🟦️.ts and declaration(name) TypeScript AST lookup; retarget pure imports and IO declaration owner, retain every dependency adapter name. If functions split across two owners, lookup by explicit name->owned-source mapping or direct pure imports, not concatenating arbitrary full source text. Source-admission JSON schema under normalization remains canonical even if provider implementation moves. Existing AJV schema vectors, independent Git/source census and no-follow root/fence laws must retain actual route.
