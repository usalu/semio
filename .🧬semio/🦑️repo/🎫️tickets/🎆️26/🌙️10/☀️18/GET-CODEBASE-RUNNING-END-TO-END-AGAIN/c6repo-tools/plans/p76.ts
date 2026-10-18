import type { ClassPlan } from "../classsplit.ts";
export const cp: ClassPlan = {
  source: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🟦️.ts",
  className: "VerifyScript",
  baseClass: "Script",
  groups: [
    { dir: "🧾️|taxonomy-lanes", className: "VerifyTaxonomyLanes", doc: "🧾️ Taxonomy, composed-child, mutation-outcome, history, vocabulary, purity, corruption, breach and Rust-warning verification lanes.", members: ["lanes", "runTaxonomy", "runComposedChildRefs", "runMutationOutcomeLaw", "runHistoryClosure", "runSemanticVocabulary", "runPackagePurity", "runGeneratedCorruption", "runPolicyBreach", "runRustWarnings"] },
    { dir: "🔬️|interactivity-lanes", className: "VerifyInteractivityLanes", doc: "🔬️ Interactivity audit verification lanes.", members: ["runInteractivityAudit", "runInteractivityApps", "runInteractivityP1qB1B6", "runInteractivityP1w", "runInteractivityP1x", "runInteractivityP1y", "runInteractivityP1z", "runInteractivityP5d", "runInteractivityP5e", "runInteractivityP3mn"] },
    { dir: "🎯️|tool-job-lane", className: "VerifyToolJobLane", doc: "🎯️ Tool-job coverage verification lane.", members: ["runToolJobCoverage"] },
    { dir: "🏁️|gate-lanes", className: "VerifyGateLanes", doc: "🏁️ Dependency-freeze, layering, canonical-architecture and whole-gate verification lanes.", members: ["runDependencyFreeze", "runLayering", "runCanonicalArchitecture", "runGate"] },
  ],
};
