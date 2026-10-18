import type { Plan } from "../split.ts";
const base = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🚪️artifact-io";
export const plan: Plan = {
  source: `${base}/🟦️.ts`,
  baseDir: base,
  modules: [
    { dir: "📋️|owner-table", ranges: [["POLICY_STDIO_OWNER_TABLE_REL", "policyRustFileHasRealTraitImpl"]] },
    { dir: "🗂️|stdio-catalog", ranges: [["policyStdioFacetRsTsBreaches", "POLICY_DERIVED_FACETS"], ["policyDerivedArtifactFacetBreaches", "policyArtifactComposerBreaches"]] },
    { dir: "🧾️|standards", ranges: [["policyStandardsCoverageBreaches", "policyStandardsCoverageBreaches"], ["PolicyStandardManifestGroup", "policyStandardSubsetVocabularyBreaches"]] },
    { dir: "🧬️|schema-representation", ranges: [["policySchemaRepresentationLeavesFor", "policySchemaRepresentationBreaches"]] },
    { dir: "🔌️|io-matrix", ranges: [["policyHasComponentUnder", "policyCodecFidelityBreaches"]] },
    { dir: "🔁️|dialects-and-round-trip", ranges: [["POLICY_DIALECT_LITERAL_RE", "policyMutationVocabularyBreaches"]] },
    { dir: ".", ranges: [["policyArtifactIoBreaches", "policyStdioArtifactsBreaches"]] },
  ],
};
