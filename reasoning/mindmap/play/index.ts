// #region 🧲Header
/** @emoji 🧠 `@semio-tech/reasoning-mindmap-play` — mindmap play exports for WIRES and other normal-graph hosts. */
// #endregion 🧲Header

export { type MindmapFixtureV1, type MindmapCameraState } from "@semio-tech/reasoning-mindmap-react";
export { Puzzle2dCanvas, type Puzzle2dGraphPortMode } from "@semio-tech/puzzle-2d-react";

//#region 🔖SExtension
import { baselineSingleAppPlatformDefinition, type PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for reasoning mindmap. */
export function buildReasoningMindmapProgramDefinition(): PlatformDefinition {
	return baselineSingleAppPlatformDefinition("reasoning.mindmap", "Reasoning Mindmap", "mindmap", "Mindmap", "reasoning-mindmap-play");
}
//#endregion 🔖SExtension
