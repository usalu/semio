const { R, f, ph, write, dir, doc } = require("../lib_ps.cjs");
const spec = {
  file: R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🧮️pack/🟦️.ts",
  function: "planArtifactCratesOnce", keep: [], typeOverrides: { options: "PlanOptions" }, resolve: true, typesFromProducers: true,
  shared: { dir: dir("📚", "pack-types"), doc: doc("📚", "The shared types of the artifact crate pack planner: the crate under construction and the moved root item unit.") },
  files: [
    f("📥", "source-and-slices", "The crate source folding, the carve slices and the root item index of the artifact crate pack planner.", [
      ph("readCrateSource", "📖", "Reads the manifest and the crate root source, folds the mount trees and applies the test, lift and inline transforms.", 43, 62),
      ph("buildCarveSlices", "🍰", "Converts mutation leaves, collects the declaration tree and builds the carvable slices with their gates and wrappers.", 68, 87),
      ph("indexRootItems", "🗂", "Indexes the crate root items: extern rows, gated uses, use targets, regions and the measurement root.", 91, 122),
    ]),
    f("🧲", "slice-ownership", "The mirror measurement closures, the slice infos and the chain ownership lookups of the artifact crate pack planner.", [
      ph("defineMirrorMeasurement", "📐", "Defines the unit, cut overlay, wrapper spec and measurement closures that mirror one set of slices as a crate.", 123, 155),
      ph("collectSliceInfos", "🧮", "Collects the coherence types, trait names, type definitions and one info per carvable slice.", 161, 167),
      ph("defineChainOwnership", "🧭", "Indexes the owners of declaration paths and defines the member visibility, source text and shim lookups of path chains.", 175, 236),
    ]),
    f("🔗", "edges-and-closures", "The slice edge resolution and the root item closures of the artifact crate pack planner.", [
      ph("resolveSliceEdges", "🕸", "Resolves the library and test edges, root references and coherence of every slice info.", 237, 312),
      ph("closeRootItemReferences", "🔁", "Computes the transitive closure of the root items every slice references and picks the closest owners.", 318, 351),
    ]),
    f("🧵", "weave-merge-and-blockers", "The root item weaving, the component merging and the blockers of the artifact crate pack planner.", [
      ph("planRootItemWeaving", "🧶", "Plans which root module items are woven into the slices and which tests are dropped.", 366, 386),
      ph("mergeCoupledComponents", "🧩", "Merges slices that are coupled by coherence, import cycles and file chains into components.", 394, 455),
      ph("collectComponentBlockers", "🚧", "Groups the merged slices into components and collects the blockers that keep a component from being carved.", 456, 460),
    ]),
    f("🗜", "packing", "The component layering and the packing of components into crates of the artifact crate pack planner.", [
      ph("prepareComponentPacking", "🪜", "Layers the components by dependency depth and defines the group, acyclicity and mirror validity helpers.", 502, 536),
      ph("packComponents", "🧳", "Packs the components into crate groups by the measured budget.", 540, 540),
    ]),
    f("🚚", "moved-items", "The root items that move into crates and their hosting in the artifact crate pack planner.", [
      ph("findMovedRootItems", "🔎", "Finds the root items that move into the crates the groups reference.", 573, 583),
      ph("pieceMovedRootItems", "🧱", "Cuts the moved items into pieces and computes their dependencies.", 602, 624),
      ph("buildMovedUnits", "📋", "Builds the moved units from the strongly connected pieces.", 625, 662),
      ph("hostMovedUnits", "🏠", "Assigns every moved unit a hosting crate.", 676, 718),
    ]),
    f("🏗", "crate-drafts", "The crate drafts, their names and their mirrored files of the artifact crate pack planner.", [
      ph("draftCrates", "🏷", "Drafts one crate per group and labels the drafts.", 719, 745),
      ph("nameAndPlaceCrates", "📛", "Names every draft crate, picks its directory and plans its dependencies.", 752, 755),
      ph("renderCrateMirrors", "🪞", "Renders the mirrored files of every planned crate and resolves their moved item appends.", 827, 830),
    ]),
    f("🖇", "root-edits", "The root edits and the final plan of the artifact crate pack planner.", [
      ph("collectRootEditsAndPlan", "✂", "Collects the root edits, file cuts and woven exports and assembles the crate plan.", 891, 936),
    ]),
  ],
};
write(spec, process.argv[2]);
