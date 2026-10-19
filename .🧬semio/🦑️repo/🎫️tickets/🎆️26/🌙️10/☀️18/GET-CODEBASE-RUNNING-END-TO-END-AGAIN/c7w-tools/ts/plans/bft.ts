import type { Plan } from "../split.ts";
const base = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚚️browser-frame-transport";
export const plan: Plan = {
  source: `${base}/🟦️.ts`,
  baseDir: base,
  modules: [
    { dir: "📋|contract", ranges: [["FRAME_WORKER_LOSSLESS_ITEM_CAPACITY", "BrowserFrameLosslessEvent"], ["BrowserFrameWorkerBoot", "QueuedLossless"]] },
    { dir: "🖱|dom-events", ranges: [["BrowserFramePointerDomFields", "browserFrameEventIsReplaceable"]] },
    { dir: ".", ranges: [["BrowserFrameTransport", "isLowSurrogate"]] },
  ],
};
