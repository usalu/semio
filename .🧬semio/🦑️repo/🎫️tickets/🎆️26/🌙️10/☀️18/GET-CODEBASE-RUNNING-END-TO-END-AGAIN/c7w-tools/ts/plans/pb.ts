import type { Plan } from "../split.ts";
const base = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge";
export const plan: Plan = {
  source: `${base}/🟦️.ts`,
  baseDir: base,
  modules: [
    { dir: "🧵|shard-and-actor-calls", ranges: [["MainThreadShardWorker", "serializeWgpuActorCall"]] },
    { dir: "🍽|ui-intake-and-route", ranges: [["RETAINED_DOCUMENT_OPPORTUNITIES", "reconcileRetainedWindowPatch"]] },
    { dir: "🎛|contributions-and-effects", ranges: [["declareWgpuBootSubphase", "effectTags"]], names: ["pendingTurnEffects"] },
    { dir: "🧾|typed-operations-and-handle", ranges: [["WgpuPluginExtensionCompletion", "WgpuTypedOperationDrive"], ["WgpuOperationPublication", "wgpuEphemeralSnapshot"]] },
    { dir: "🔌|plugin-module-loader", ranges: [["loadPluginModule", "loadPluginModule"]] },
    { dir: ".", ranges: [["WgpuJsBridge", "invocationResponseJson"]] },
  ],
};
