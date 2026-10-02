# Landed Declaration Helper and Physical Witnesses

Read-only current source review. No compiler/tests/jobs executed. Root reports source-services direct GREEN102/232/8.11s; this audit does not independently replay it.

## Concrete current contradiction

`library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts:18` now binds source to normalizationSourceDeclarations(sourcePath), while final assertion389 still compares source with readFileSync(sourcePath,"utf8"). Aggregate AST text cannot equal umbrella physical bytes. Keep distinct physicalSource/owner capture and compilerSource variables. If semantic proof now depends on moved declarations, capture each required owner explicitly as `{path,bytes/hash}` and recheck every owner after oracle execution; a synthetic joined-source hash is not a physical input receipt.

## Helper scope

`normalization/🧪️support/🏗️source-services/🟦️.ts:6–15` reads umbrella plus six actual owners plus serialization, removes import/export declarations and serialization private type aliases, and joins declaration getText spans. It preserves actual implementation text for isolated AST/compiler tests, but intentionally loses module import binding/physical offsets and cannot prove runtime module closure. Source-services module boundary tests supply that distinct proof. Symbol lookup must reject duplicate names rather than silently selecting first if future declarations collide. This helper is test-only and introduces no production runtime dependency.

Ordinary extracted-function harnesses can use aggregate AST text: preflight-reference-basis, typescript-path-collection, readme-move-source-authority, bare-reference-sibling-precedence, reference-coordinate-progress, rust-writable-path-authority, transaction-recovery-authority. Their physical source stability assertions, if present, still require explicit real owner captures. Strict module typing tests must check actual files rather than only a synthetic joined program; helper stripping imports is useful for injection, not proof that live imports typecheck.

## Physical witnesses to preserve

Workspace-contract7002–7027 and7046–7060 capture the actual `normalization/🧬️mutation/📸️captured-source/🟦️.ts`, fixtures/schema and test input bytes; recorded sourceHash/helperHash/sourceStable/inputsStable refer to those physical paths. They must not be mechanically routed through aggregate helper. Source-index-capture74 uses an actual sourceIndexPath content hash as dynamic import receipt; preserve that exact physical file and snapshot/index corpus hashes at84 onward. Mutation captured-source fixture/descriptor/taxonomy content receipts remain actual bytes. AST aggregate text does not substitute for them.

For moved provider coverage extend physical witness input list to required owner files, not all unrelated test helper inputs. Preserve existing single-file capture and add explicit needed owner capture, rather than changing original hash meaning.

## Actual canonical imports and exports

Pure admission test now imports projectTaxonomySourceAdmission directly from `normalization/🚪️source-admission/🟦️.ts:5`. Captured-source now imports inventoryTaxonomySources/TaxonomySourceInventory from `🚪️source-admission/📁️io/🟦️.ts:5`, removing its demonstrated broad runtime edge. Remaining mutation imports of package BreachRecord are type-only (workflow/evidence/structural-reachability); they do not recreate the runtime edge but remain candidates for a separate domain-owned breach contract.

Pure owner exports TaxonomyNodeKind, all source observation/admission interfaces, origin constants and projection/fence/pathspec helpers; IO exports TaxonomySourceInventoryOptions and TaxonomySourceInventory alongside physical operations. Loader owns TaxonomyLoadOptions and schema contracts. Keep actual explicit import types rather than facade callbacks or backward loadNormalizationTaxonomy alias. No new foreign runtime library appears in inspected owners.
