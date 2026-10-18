import type { Plan } from "../split.ts";
const base = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧬️schema-overhaul-s2";
export const plan: Plan = {
  source: `${base}/🟦️.ts`,
  baseDir: base,
  modules: [
    { dir: "📇️|stdio-entries", ranges: [["PolicyStdioStandardEntry", "policyListStdioSchemaOwningEntries"]], names: ["policyStdioStandardKey"] },
    { dir: "🚫️|machinery-bans", ranges: [["POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST", "policyStdioVcsMachineryBanBreaches"], ["POLICY_DIFF_ALGEBRA_ALLOWLIST", "policyDiffAlgebraBreaches"]] },
    { dir: "🧱️|grammar-honesty", ranges: [["POLICY_GRAMMAR_HONESTY_LEAF_MARKERS", "policyGrammarHonestyBreaches"]] },
    { dir: "🪞️|facet-mirror-drift", ranges: [["POLICY_FACET_MIRROR_DRIFT_FACETS", "policyFacetMirrorDriftBreaches"]] },
    { dir: ".", ranges: [["POLICY_SNIFF_REALITY_ALLOWLIST", "policySniffRealityBreaches"], ["policySchemaOverhaulS2Breaches", "policySchemaOverhaulS2Breaches"]] },
  ],
};
