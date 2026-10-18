import type { Plan } from "../split.ts";
const base = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store";
export const plan: Plan = {
  source: `${base}/🟦️.ts`,
  baseDir: base,
  modules: [
    { dir: ".", ranges: [["toolJobImmutableOperationRootsExact", "toolJobArtifactEnvelopeRejectionTransferExact"]] },
    { dir: "📄️|caller-retained-documents", ranges: [["toolJobPresentationEnvelopeCallerRetainedExact", "toolJobGisMapEnvelopeCallerRetainedExact"]] },
    { dir: "🖼️|caller-retained-media", ranges: [["toolJobRasterEnvelopeCallerRetainedExact", "toolJobDrawingEnvelopeCallerRetainedExact"]] },
    { dir: "🏛️|structural-owners", ranges: [["toolJobHistoryLedgerAdmissionExact", "toolJobPagedIngressExact"]] },
  ],
};
