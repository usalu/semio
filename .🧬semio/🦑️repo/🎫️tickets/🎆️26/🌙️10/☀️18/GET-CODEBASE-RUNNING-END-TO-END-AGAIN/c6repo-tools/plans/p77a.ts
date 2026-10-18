import type { Plan } from "../split.ts";
const base = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔁️live-replay";
export const plan: Plan = {
  source: `${base}/🟦️.ts`,
  baseDir: base,
  modules: [
    { dir: ".", ranges: [["toolJobActorProgressOverlayExact", "toolJobLiveFixedReplayExact"]] },
    { dir: "🪟️|fem2d-mounted-session", ranges: [["toolJobFem2dMountedSessionExact", "toolJobFem2dMountedSessionExact"]] },
    { dir: "🧊️|retained-ownership", ranges: [["toolJobLayoutColdRelayRetainedExact", "toolJobUniversalRetainedOwnershipExact"]] },
    { dir: "🧮️|fem-numerical-microcursor", ranges: [["toolJobFemNumericalMicrocursorExact", "toolJobFemNumericalMicrocursorExact"]] },
  ],
};
