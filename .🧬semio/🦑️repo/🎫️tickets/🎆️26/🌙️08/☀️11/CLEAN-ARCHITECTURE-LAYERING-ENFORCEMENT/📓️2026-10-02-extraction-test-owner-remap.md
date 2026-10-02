# Extraction Test Owner Remap

Read-only current source audit. No tests, compiler, Nx or generation executed. Paths below are relative to library unless otherwise specified.

## Registration gap

The two existing tests `🧹️normalization/🧪️tests/🚪️source-admission/🟦️.ts` and `🚪️source-admission-io/🟦️.ts` exist with owned fixture/schema collections. Current package script, project, package manifest and workspace-contract source searches found no explicit inclusion of either file. The default registered library test route executes only `🧪️tests/🔬️workspace-contract/🟦️.ts` (package script715); workspace imports do not include these admission tests. Typecheck inclusion is not runtime execution. Add an explicit canonical script command for both existing owner files, its Nx target/package script and ordered launch registration, or import both once into an existing runtime collection. Do not claim original registered admission coverage merely because the files exist.

## Canonical declaration registry

Use actual source owner selection per declaration, retaining AST extraction and both compilers.

| Planned normalization owner | Declarations |
| --- | --- |
| 🔣️taxonomy/🟦️.ts | parseTaxonomy, loadTaxonomy, LoadedTaxonomy, taxonomy schema types, parser helper/constants, pure parse-facts cache |
| 🛣️path/🟦️.ts | sourceRelative, normalizeRelative, inScope and canonical lexical path helpers |
| 📁️input/🟦️.ts | assertNoFollowAncestors, assertLexicalInputOutsideOpaque, lstatOrNull and owned physical input helpers |
| 🏃️operation/🟦️.ts | TaxonomyProgress, report, TaxonomyCancellationError and cancellation helpers |
| 🚪️source-admission/🟦️.ts | projector, origin constants/contracts, safe/opaque/shape/physical consistency/fence helpers, scoped Git pathspec contract/functions |
| 🚪️source-admission/📁️io/🟦️.ts | preparation, no-follow census, Git records/index receipt cache, walks/observation, collect, inventoryTaxonomySources, TaxonomySourceInventory |
| original normalization/🟦️.ts | inventoryTaxonomyWithSourceParentPruning, explicitTicketRows, planning/transaction/reference/generator orchestration |

Affected borrowed-source harnesses found by repository source-only search: taxonomy-pattern-compiler-reuse (parser/load/path/input); typescript-path-collection (sourceRelative/normalizeRelative); readme-move-source-authority (path/input plus unchanged plan parsing); preflight-reference-basis (path/input); readme-current-source-activation (parser/load/path/input); rust-finite-target-consumption (path/input/operation); rust-writable-path-authority (operation); bare-reference-sibling-precedence (path); reference-coordinate-progress (path/input). Transaction-v2/recovery and path-emoji-statutes also contain relevant symbols; distinguish actual declaration requests from names inside parseTaxonomyPlan and metadata strings before changing them. Workspace-contract3020 borrows assertGeneratorPreviewTarget/invokeGeneratorPreview: keep these on orchestration.

source-admission-io14–23 must select declarations across these owners; lines78/90 still borrow inventoryTaxonomyWithSourceParentPruning/explicitTicketRows from orchestration. Pure source-admission test line5 imports projector directly from its new owner. No broad alias facade is necessary.

## Matcher law contradiction must be explicit

`🧪️tests/♻️taxonomy-pattern-compiler-reuse/🟦️.ts:143–243` structurally locates LoadedTaxonomy, parseTaxonomy, loadTaxonomy and matcher calls in the old normalization AST. Update ownership census across actual files; retain every original query owner rather than deleting moved owners. Lines218–222 demand distinct matcher/schema/input and [2 reads,2 parses,2 validations]. Current concurrent content cache contradicts that law.

Fresh input identity/read and changed/invalid/lossy/syntax refusal are authority contracts. Exact parse/validation invocation counts are implementation assertions. Distinct mutable schema protects against cross-session mutation; distinct matcher originally defines invocation-local pattern cache ownership. A clean split can cache immutable content-derived parse facts while constructing fresh session schema view and matcher per freshly admitted input. That requires an explicit schema-first law revision proving mutation isolation, fresh physical capture and changed-input failure, plus accurately updated parse-count receipt; silently removing identity/count assertions would waive the actual failure. Keep fresh sessions even when bytes match; no cached physical input/path authority.

Retarget strict typing law264–275 to parser, IO and normalization files rather than checking only old normalizerPath. Registration law below it owns exact route/launch budget; preserve its closed fixture or handcraft an intentional owner-source list extension.

## Typecheck and Nx input closure

`📦️packages/🟦️typescript/tsconfig.json` includes ../../**/*.ts, so all six new owner files are automatically checked; generated/distribution exclusions remain. No new tsconfig needed. Package project namedInputs currently include literal normalization/🟦️.ts at113/128 and targeted task inputs561/601; literal coverage does not observe moved owner bytes. Add the canonical six owner paths or a narrow normalization owner subtree input to each affected target/named input. Follow imported runtime closure where Nx already does so, but do not assume literal source hashes cover new leaves.

Permanent executable changes belong to existing package 📜️script.ts, project commands call it, package scripts call Nx, and .vscode/launch.json must register the new focused route in canonical order. Existing typecheck/public source inputs should follow actual loadTaxonomy owner; remove loadNormalizationTaxonomy usages rather than retaining a backward alias. Discovery's separate loadTaxonomy vocabulary remains semantically distinct and must not be mechanically replaced.
