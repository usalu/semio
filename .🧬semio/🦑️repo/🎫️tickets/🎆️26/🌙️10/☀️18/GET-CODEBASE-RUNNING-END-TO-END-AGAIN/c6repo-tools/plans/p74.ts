import type { Plan } from "../split.ts";
const base = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan";
export const plan: Plan = {
  source: `${base}/🟦️.ts`,
  baseDir: base,
  modules: [
    { dir: "💰️|budget", ranges: [["CrateBudget", "parseCrateBudget"]] },
    { dir: "🧾️|model", ranges: [["CrateMeasure", "PackingStats"]], names: ["PlanOptions", "SliceInfo"] },
    { dir: "📏️|measure", ranges: [["HOST_CFG", "WASM_CFG"], ["crateTreeHostCfg", "measureFits"]], names: ["readMeasureSources"] },
    { dir: "🏷️|naming", ranges: [["mutationFamilyOf", "isMutationCrateLabel"]], names: ["facetSlug", "crateIdent", "facetEmoji", "crateIdentPlaceholder"] },
    { dir: "🧭️|module-chains", ranges: [["inlineModuleBase", "groupHeads"], ["aliasChains", "implHeader"]] },
    { dir: "📦️|cargo-lib", names: ["resolveOwningCrate", "readCargoLib", "externCrateRows", "topLevelNames"], ranges: [] },
    { dir: "🪢️|weave", names: ["WeaveNode", "weaveRootEdits"], ranges: [] },
    { dir: "🧰️|tree-helpers", names: ["lcaDirectory", "allItems", "removeRange"], ranges: [] },
    { dir: "🧮️|pack", names: ["planArtifactCrates", "planArtifactCratesOnce"], ranges: [] },
  ],
};
