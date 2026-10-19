import type { Plan } from "../split.ts";
const base = "🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🤝️collaboration";
export const plan: Plan = {
  source: `${base}/🟦️.ts`,
  baseDir: base,
  modules: [
    { dir: "📋|contract", ranges: [["HubCollaborationJourneyV1", "SeverableRelayV1"]] },
    { dir: "🧲|relay-and-pixels", ranges: [["startSeverableRelay", "distinctColours"]] },
    { dir: "🖥|shells", ranges: [["MirrorNode", "ReactShell"]] },
    { dir: "🧭|native-law-and-follow", ranges: [["Wasm32Kind", "followNativeCursors"]] },
    { dir: "🎬|journeys", ranges: [["JourneyRun", "JOURNEY_BODIES"]] },
    { dir: ".", ranges: [["freeLoopbackPort", "runHubCollaborationCli"]] },
  ],
};
