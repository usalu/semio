const { R, f, ph, write, dir, doc } = require("./lib_ps.cjs");
const file = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔣️taxonomy/🟦️.ts";
write({
  file, function: "parseTaxonomy", keep: [], resolve: true,
  shared: { dir: dir("🧰", "taxonomy-parse-helpers"), doc: doc("🧰", "The shared taxonomy v7 parse helpers: the expiry reader, the opaque path exclusion table and the projection types the parse phases use.") },
  typeOverrides: { raw: "unknown", validatedContractPattern: "(value: unknown, name: string, exactBasename: boolean) => string", parseProjectionSegment: "(value: unknown, name: string, destination: boolean) => SemanticProjectionSourceSegment | SemanticProjectionDestinationSegment", pathMatcher: "ReturnType<typeof createTaxonomyPathMatcher>" },
  files: [
    f("🧱", "kinds-and-fixed-contracts", "The taxonomy v7 prelude, file and directory kinds, fixed filename and directory contracts, rejections, configurable entries and resolution rules.", [
      ph("parseTaxonomyPrelude", "🔰", "Reads the root rows and validates the unicode, selector, collision and enforcement policies.", 45, 96),
      ph("parseFileAndDirectoryKinds", "🗂", "Parses the file kinds and the semantic directory kinds.", 98, 119),
      ph("parseFixedContracts", "📌", "Parses the fixed filename and fixed directory contracts.", 121, 179),
      ph("parseRejectionsAndResolution", "🚫", "Parses the fixed filename rejections, configurable entries, file kind resolution rules and scoped file kinds.", 184, 229),
    ]),
    f("🌳", "members-renderers-and-descendants", "The directory member kinds, projection renderers, descendant contracts and catalog projection contracts.", [
      ph("parseMemberKindsAndRenderers", "👥", "Parses the directory and projected member kinds and the projection profile renderers.", 253, 292),
      ph("parseDescendantsAndCatalogs", "🌲", "Parses the semantic descendant contracts and the path projection catalog contracts.", 294, 417),
    ]),
    f("🪞", "projections-and-consumers", "The path projection contracts, owned file projections, reference consumers and the mutation catalog projection.", [
      ph("parsePathProjections", "🧭", "Parses the semantic path projection contracts and the owned file projection contracts.", 418, 565),
      ph("parseConsumersAndMutationCatalog", "📚", "Parses the projection reference consumer contracts and the mutation catalog projection.", 566, 607),
    ]),
    f("🏭", "generators-packages-and-exclusions", "The generator contracts, ecosystems, package rules, source dispositions, path exclusions and the assembled schema.", [
      ph("parseGeneratorsAndEcosystems", "⚙", "Parses the generator contracts and the ecosystems.", 609, 667),
      ph("parsePackageRulesAndDispositions", "📦", "Parses the package glue grammar, boundary rules and profiles and the source dispositions.", 669, 726),
      ph("parseExclusions", "🚧", "Parses the path exclusions and the opaque subtree set.", 728, 749),
      ph("assembleTaxonomySchema", "🧬", "Assembles the taxonomy v7 schema object from the parsed rows.", 755, 755),
    ]),
  ],
}, process.argv[2]);
