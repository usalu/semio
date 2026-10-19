import type { Plan } from "../split.ts";
const base = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker";
export const plan: Plan = {
  source: `${base}/🟦️.ts`,
  baseDir: base,
  modules: [
    { dir: "🧬|bindings-types", ranges: [["BrowserRendererWorkerHandle", "RendererBindings"]] },
    { dir: "🧵|state", ranges: [["BOOT_HEARTBEAT_MS", "lastStepOutcome"], ["DedicatedWorkerScope", "pageImageDecode"]], names: ["bootPhaseStack", "pendingHostIo", "hostIoSeq", "originalFrameSource"] },
    { dir: "⏱|steps-and-reporting", ranges: [["ownedStep", "driveChunks"], ["progress", "progress"], ["declarePhase", "declareBootSubphase"], ["post", "post"]] },
    { dir: "🖼|page-image-decode", ranges: [["requestPageImageDecode", "cancelPageImageDecode"]] },
    { dir: "📦|assets-and-host-io", ranges: [["AssetRequest", "AssetRequest"], ["scheduleAssetPump", "pumpAsset"], ["requestHostIo", "settleHostIo"]] },
    { dir: "🧯|close-and-fault", ranges: [["closeRuntime", "beginClose"], ["fault", "requestFault"]] },
    { dir: "🎞|frame-and-introspection", ranges: [["runFrameTurn", "answerIntrospection"], ["answerMediaCommand", "answerMediaCommand"]] },
    { dir: "🔌|boot-and-plugins", ranges: [["PluginHandleMount", "boot"]] },
    { dir: "📨|receive", ranges: [["receive", "receive"]] },
    { dir: ".", ranges: [[355, 371], [1028, 1028], [1069, 1111]] },
  ],
  stateModule: "🧵|state",
};

export const patches = [
  { dir: "🔌|boot-and-plugins", from: "prepareWgpuPluginModules(workerCatalog, explicitPluginModules,", to: "prepareWgpuPluginModules(workerCatalog!, explicitPluginModules!," },
  { dir: "🔌|boot-and-plugins", from: "maxDescriptors: explicitPluginModules.length,", to: "maxDescriptors: explicitPluginModules!.length," },
  { dir: "🔌|boot-and-plugins", from: "const pluginCredits = explicitPluginModules.length;", to: "const pluginCredits = explicitPluginModules!.length;" },
  { dir: "🔌|boot-and-plugins", from: '(selection !== "all" || explicitPluginModules.length !== 0)', to: '(selection !== "all" || explicitPluginModules!.length !== 0)' },
  { dir: "🔌|boot-and-plugins", from: "new OriginalBrowserFrameSource(runtime,", to: "new OriginalBrowserFrameSource(runtime!," },
  { dir: "🔌|boot-and-plugins", from: "await originalFrameSource.admit();", to: "await originalFrameSource!.admit();" },
  { dir: "🎞|frame-and-introspection", from: "if (quarantined) requestFault(quarantined.code, quarantined.detail);", to: "if (quarantined) requestFault((quarantined as { readonly code: string; readonly detail: string }).code, (quarantined as { readonly code: string; readonly detail: string }).detail);" },
];
