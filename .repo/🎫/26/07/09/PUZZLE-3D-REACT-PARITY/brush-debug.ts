#!/usr/bin/env bun
/** 🔍 Debug brush candidates in wasm runtime. */
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
		args: { possibleId: "puzzle3d.tool.brush" },
	}),
	viewState,
);
const pickOps = await handle.handleCommand(
	instanceId,
	JSON.stringify({ controllerId: "puzzle3d-play", command: "worldPick", args: { granularity: "mesh", id: 0, merge: "replace" } }),
	viewState,
);
console.log("[DEBUG] worldPick ops", pickOps.length);
const nodeBefore = await handle.render(instanceId, "puzzle3d.play.composite", viewState);
const fullId = JSON.parse((nodeBefore as { world3d?: { vorticesJson?: string } }).world3d?.vorticesJson ?? "[]")[0]?.fullId;
await handle.handleCommand(
	instanceId,
	JSON.stringify({ controllerId: "puzzle3d-play", command: "worldVortexHover", args: { fullId } }),
	viewState,
);
const node = await handle.render(instanceId, "puzzle3d.play.composite", viewState);
const world3d = (node as { world3d?: Record<string, string> }).world3d ?? {};
console.log("[DEBUG] brushPreviewJson", world3d.brushPreviewJson);
console.log("[DEBUG] interactionJson", world3d.interactionJson);
const engagement = (await handle.windowEngagements(instanceId, viewState))["puzzle3d-main"];
console.log("[DEBUG] control", engagement?.control?.kind);
