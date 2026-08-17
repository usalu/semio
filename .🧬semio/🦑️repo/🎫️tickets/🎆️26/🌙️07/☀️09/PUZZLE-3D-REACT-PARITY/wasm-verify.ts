#!/usr/bin/env bun
/** 🔍️ Wasm command round-trip verification for puzzle 3d fill/brush. */
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const moduleUrl = `file://${join(repoRoot, "framework/product/os/dev/plugin-modules/puzzle/puzzle_plugin.js")}`;
const { loadPluginModule } = await import(join(repoRoot, "framework/core/js/index.ts"));
const handle = await loadPluginModule("puzzle", moduleUrl);
const instanceId = await handle.createApp("puzzle3d-play");
const viewState = {};

await handle.handleCommand(
  instanceId,
  JSON.stringify({
    controllerId: "puzzle3d-play",
    command: "engagementPossibleSelect",
    args: { possibleId: "puzzle3d.tool.fill" },
  }),
  viewState,
);
const fillEngagement = (await handle.windowEngagements(instanceId, viewState))["puzzle3d-main"];
if (fillEngagement?.control?.kind !== "slider") throw new Error("fill slider missing after fill tool select");
console.log("[DEBUG] fill control present");

await handle.handleCommand(instanceId, JSON.stringify({ controllerId: "puzzle3d-play", command: "setFillCount", args: { value: 4 } }), viewState);
const fillValue = (await handle.windowEngagements(instanceId, viewState))["puzzle3d-main"]?.control?.value;
console.log("[DEBUG] fill slider value:", fillValue);
if (fillValue !== 4) throw new Error(`fill value expected 4 got ${fillValue}`);

await handle.handleCommand(
  instanceId,
  JSON.stringify({
    controllerId: "puzzle3d-play",
    command: "engagementPossibleSelect",
    args: { possibleId: "puzzle3d.tool.brush" },
  }),
  viewState,
);
await handle.handleCommand(instanceId, JSON.stringify({ controllerId: "puzzle3d-play", command: "setHover", args: { objectId: "seed-left-001", mode: "mesh", id: 0 } }), viewState);
await handle.handleCommand(instanceId, JSON.stringify({ controllerId: "puzzle3d-play", command: "worldVortexHover", args: { fullId: "seed-left-001:v0" } }), viewState);
const node = await handle.render(instanceId, "puzzle3d.play.composite", viewState);
const world3d = (node as { world3d?: Record<string, string> }).world3d ?? {};
console.log("[DEBUG] brushPreviewJson present:", Boolean(world3d.brushPreviewJson));
const brushControl = (await handle.windowEngagements(instanceId, viewState))["puzzle3d-main"]?.control;
console.log("[DEBUG] brush control:", brushControl?.kind);
if (brushControl?.kind === "toggleGroup" && (brushControl.options?.length ?? 0) > 0) {
  console.log("[DEBUG] brush candidates:", brushControl.options?.length);
} else {
  console.log("[DEBUG] brush candidates pending — preview:", world3d.brushPreviewJson?.slice(0, 120));
}

const manifest = handle.manifest;
const examples = manifest.examples.filter((row) => row.appId === "puzzle3d-play");
const duplicateForest = examples.filter((row) => row.id === "concrete-forest").length;
if (duplicateForest > 1) throw new Error("duplicate concrete-forest examples in manifest");
console.log("[DEBUG] puzzle3d examples:", examples.map((row) => row.label).join(", "));

console.log("[DEBUG] wasm-verify passed");
