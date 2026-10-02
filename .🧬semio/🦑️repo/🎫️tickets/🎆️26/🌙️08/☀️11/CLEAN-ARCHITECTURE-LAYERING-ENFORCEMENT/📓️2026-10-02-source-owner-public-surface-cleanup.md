# Source Owner Public Surface Cleanup

Read-only source audit; no jobs executed.

## Taxonomy helper exports

Canonical taxonomy owner exports record430, stringArray435, requiredLiteral441, requiredString447 and requireExactKeys452 because normalization still consumes them. Their implementations enforce general JSON shape but diagnostics say Taxonomy v7 even for unrelated generator preview, dependency state and mutation catalog inputs. Actual normalization call sites include plan variants487/520/529/611, generator preview771–812, dependency state3346–3358 and mutation catalog3777 onward. These are real callers, not dead extraction helpers.

Do not rename them into public taxonomy contracts merely to conceal the edge. A coherent next partition is one owned neutral schema/JSON shape validation leaf with caller-owned diagnostic domain/name, then taxonomy and normalization import it directly; alternatively keep each named domain parser's private checks where its semantics diverge. Current requiredString nonempty semantics and requireExactKeys canonical array sorting must remain exact. Existing general framework schema validator interface can own validation services if its contract fits; no runtime third-party validator is needed. Do not export a foreign AJV type or accept facade parser callbacks. This partition is separate from finishing the current extraction and needs its own closed fixture before implementation.

Meaningful taxonomy public surface: TaxonomyLoadOptions, LoadedTaxonomy, GeneratorInputTaxonomy, loadTaxonomy and schema domain contracts used by actual consumers. JsonRecord is neutral structural type. Broader schema declaration exports are first-party; inspected public surfaces reference discovery-owned types, native RegExp and system node Stats only. No external library class/interface leaks from the six inspected owners. Node Stats is a framework system interface used by the physical service, rather than foreign validator API.

## Actual redundant normalization imports

Textual identifier census below adapters found zero body occurrences for: createHash, parseFixedDirectoryContractSetScope, parseNamedFixedDirectoryContractSetScope, parseSemanticOwnedDocumentCorrections, validateTaxonomy, GeneratorProjectionActivation, RegistryCatalogInputDiscovery, SemanticPackageGeneration and DiscoveryTaxonomy. Remove those actual unused bindings when finishing source cleanup; their residual runtime imports obscure dependency ownership. Keep parseSemanticOwnedCurrentSourceRevisions (3), Stats (3), SemanticFacetPrimaryFileProjectionContract (1), SemanticPathProjectionReferenceConsumerForm (1), SEGMENTER (1), isEmojiGrapheme (1), TAXONOMY_RELATIVE_PATH (1) because actual body uses remain. Census is lexical evidence, not a compiler unused-symbol diagnostic.

No immediately adjacent empty region pair found. Whitespace-only empty region cleanup should preserve structural owner sections only when substantive declarations remain; comments referring to old normalization line locations are documentation drift rather than runtime authority.

## Canonical JSON facade export

Normalization still imports and reexports canonicalJson at38–39. Repository source search found no actual runtime import of canonicalJson from normalization; index/workflow use direct serialization owner. Package facade source also had no canonicalJson occurrence. Remove the unused reexport while retaining normalization's own direct serializer import. A no-backward-alias greenfield extraction should not preserve that unused facade solely because it was once public. Borrowed AST compiler harnesses already select actual serialization declaration owner and need no runtime facade export.

## Authority distinctions

Mutation's BreachRecord imports remain type-only package facade edges; no runtime cycle is restored by them. Source admission pure test imports its actual projector; captured-source imports actual IO owner. Keep those canonical direct edges. Future helper extraction must not reverse-import normalization, or the reduced runtime closure would be lost.
