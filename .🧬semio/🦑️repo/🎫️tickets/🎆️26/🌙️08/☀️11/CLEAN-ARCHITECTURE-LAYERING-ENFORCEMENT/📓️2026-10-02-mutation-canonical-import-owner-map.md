# Mutation Canonical Import Owner Map

Read-only source inspection; no jobs. Concurrent changes are present. Paths below relative to Repo library `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Exact package-facade runtime imports

| Caller | Imported runtime names | Existing actual owner |
|---|---|---|
| normalization/mutation/📸️captured-source/🟦️.ts:4 | loadTaxonomy | 🔍️discovery/🟦️.ts:1334 |
| normalization/mutation/🧾️evidence/🟦️.ts:2 | canonicalPrimaryFilenameForKind, loadTaxonomy | discovery :1385/:1334 |
| normalization/mutation/🪪️identity/🟦️.ts:1 | canonicalPrimaryFilenameForKind | discovery :1385 |
| normalization/mutation/📐️structural-reachability/🟦️.ts:2 | canonicalPrimaryFilenameForKind, loadTaxonomy | discovery :1385/:1334 |
| normalization/mutation/📇️index/🟦️.ts:7 | getRepoMetaDir, loadTaxonomy | library 🟦️.ts:81; discovery :1334 |

Replace discovery runtime names with direct canonical discovery imports. getRepoMetaDir presently has its definition in main library, not a separate existing leaf found by declaration census; preserving exact owner semantics requires extracting the actual function into a proper metadata/workspace owner, then explicitly reexporting it. Do not substitute hardcoded repo metadata folder assumptions. BreachRecord appears in evidence/reachability mixed imports and workflow type-only import :5; its actual definition is library 🟦️.ts:105. Use import type from current actual owner or establish domain-owned finding contract with explicit reexports; type-only imports do not create runtime closure.

Authoring mutation-tree :4–13 already imports discovery functions directly, with loadCatalogTaxonomy aliased to loadTaxonomy. That is intentionally different from discovery loadTaxonomy caching/default authority—do not rename it to the other implementation while adjusting imports. Remaining authoring dependencies identity/direct-owner-index/structural-reachability cause transitive package-facade loading through rows above, despite its own direct imports.

## Other inward broad implementation dependencies

captured-source :6 imports inventoryTaxonomySources and TaxonomySourceInventory from normalization/🟦️.ts. Index :4 and workflow :4 import canonicalJson from that same implementation. These are actual defining owners, not reexports: canonicalJson is normalization/🟦️.ts:1751. Removing their runtime closure requires extracting their real definitions/dependencies into domain-owned source-inventory and canonical serialization modules; changing import text alone cannot fix it. Preserve all source admission/byte digest/cancellation contracts. This report does not invent an existing narrower owner where none was found.

## Storybook asset contract

Initial source search found `.storybook/📖️stories/🧭️coordination/🟦️.ts:20` importing stale PlaygroundAssetSpec from generated playground output. A later read during this audit shows concurrent fix already imports and exports `AssetDeliveryDeclarationV1` from `🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🧭️dispatch/🟦️.ts` and StoryScope.assets uses it. That is the canonical current delivery contract; it owns declaration parsing and delivery provider dispatch. OS registry playground discovery's AssetSpecRow is producer input metadata (:18), not the generated consumer delivery contract. Do not patch generated playground TypeScript or retain PlaygroundAssetSpec compatibility alias.

No unresolved runtime behavior or passing claim is inferred from import source review.
