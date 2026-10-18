import type { Plan } from "../split.ts";
const base = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape";
export const plan: Plan = {
  source: `${base}/🟦️.ts`,
  baseDir: base,
  modules: [
    { dir: ".", ranges: [["policyTaxonomyDirsBreaches", "policyModeCompletenessBreaches"]] },
    { dir: "📚️|examples", ranges: [["policyExampleSlugOk", "policyComponentFileBreaches"]] },
    { dir: "🧱️|lib-and-barrel-shape", ranges: [["POLICY_FN_OR_IMPL_OPEN_RE", "policyTaxonomyBarrelShapeBreaches"]] },
    { dir: "🏷️|naming", ranges: [["POLICY_PROTOCOL_SEGMENT_RE", "policySprNamingBreaches"]] },
  ],
};
