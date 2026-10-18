import type { Plan } from "../split.ts";
const base = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔣️taxonomy";
export const plan: Plan = {
  source: `${base}/🟦️.ts`,
  baseDir: base,
  modules: [
    { dir: "📐️|kind-specs", ranges: [["TaxonomyLoadOptions", "SemanticDirectoryKindSpec"], ["FixedContractScope", "SemanticProjectedMemberKindSpec"]] },
    { dir: "🗃️|owned-file-projection", ranges: [["SemanticLifecycleOwnedFileProjectionContract", "SemanticOwnedFileProjectionContract"]] },
    { dir: "🧭️|path-projection", ranges: [["SemanticProjectionCaptureField", "SemanticPathProjectionProfileRenderer"], ["SemanticMutationPathProjectionCatalogContract", "MutationCatalogProjectionContractIds"]] },
    { dir: "🌳️|descendants", ranges: [["SemanticDescendantKindNode", "SemanticDescendantContract"]] },
    { dir: "🏭️|generators-packages", ranges: [["GeneratorOwnership", "PackageGlueGrammar"]] },
    { dir: "📚️|loaded", ranges: [["TaxonomyV7", "LoadedTaxonomy"], ["GeneratorInputTaxonomy", "GeneratorInputTaxonomy"]] },
    { dir: ".", ranges: [["TAXONOMY_RELATIVE_PATH", "parseTaxonomy"], ["PARSED_TAXONOMIES", "loadTaxonomy"]] },
  ],
};
