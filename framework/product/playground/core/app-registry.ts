// #region 🧲Header
/** @emoji 🗂️ `@semio-tech/framework-playground-core` — registry of all playground app definitions keyed by dev host entry kind. */
// #endregion 🧲Header

import type { PlaygroundAppDefinition } from "./index.ts";
import { drawPlayAppDefinition } from "@semio-tech/draw-core";
import { writerPlayAppDefinition } from "@semio-tech/writer-core/playground";
import { rasterPlayAppDefinition } from "@semio-tech/raster-core";
import { formsPlayAppDefinition } from "@semio-tech/forms-core";
import { flowPlayAppDefinition } from "@semio-tech/flow-core";
import { dagPlayAppDefinition } from "@semio-tech/dag-host-core";
import { imperativePlayAppDefinition } from "@semio-tech/imperative-core";
import { sequencePlayAppDefinition } from "@semio-tech/sequence-core";
import { layoutPlayAppDefinition } from "@semio-tech/layout-core";
import { lowpolyPlayAppDefinition } from "@semio-tech/lowpoly-core";
import { procedural2dPlayAppDefinition } from "@semio-tech/procedural-2d-core";
import { procedural3dPlayAppDefinition } from "@semio-tech/procedural-3d-core";
import { shootingPlayAppDefinition } from "@semio-tech/shooting-core";
import { sPlayAppDefinition } from "@semio-tech/s-core";
import { vcsPlayAppDefinition } from "@semio-tech/vcs-core";
import { gis2dPlayAppDefinition } from "@semio-tech/gis-2d-core";
import { wiresPlayAppDefinition } from "@semio-tech/reasoning-mindmap-wires-core";
import { trinityJackPlayAppDefinition } from "@semio-tech/trinity-jack-host-core";
import { trinityRewritePlayAppDefinition } from "@semio-tech/trinity-rewrite-core";
import { presentationPlayAppDefinition } from "@semio-tech/framework-presentation-core";
import { puzzle2dPlayAppDefinition } from "@semio-tech/puzzle-2d-core";
import { puzzle3dPlayAppDefinition } from "@semio-tech/puzzle-3d-core";
import { puzzle5dPlayAppDefinition } from "@semio-tech/puzzle-5d-core";

//#region 🔖Registry
function registerPlaygroundApp(registry: Map<string, PlaygroundAppDefinition>, app: PlaygroundAppDefinition): void {
	const kind = app.devHost?.playEntryKind;
	if (!kind) throw new Error(`Playground app "${app.id}" is missing devHost.playEntryKind`);
	registry.set(kind, app);
}

/** @emoji 🗂️ All playground apps keyed by {@link AppDevHostConfig.playEntryKind}. */
export const PLAYGROUND_APP_REGISTRY = new Map<string, PlaygroundAppDefinition>();

const ALL_PLAYGROUND_APPS: readonly PlaygroundAppDefinition[] = [
	puzzle2dPlayAppDefinition,
	puzzle3dPlayAppDefinition,
	puzzle5dPlayAppDefinition,
	flowPlayAppDefinition,
	dagPlayAppDefinition,
	imperativePlayAppDefinition,
	sequencePlayAppDefinition,
	layoutPlayAppDefinition,
	lowpolyPlayAppDefinition,
	procedural2dPlayAppDefinition,
	procedural3dPlayAppDefinition,
	shootingPlayAppDefinition,
	formsPlayAppDefinition,
	rasterPlayAppDefinition,
	drawPlayAppDefinition,
	writerPlayAppDefinition,
	sPlayAppDefinition,
	vcsPlayAppDefinition,
	gis2dPlayAppDefinition,
	wiresPlayAppDefinition,
	trinityJackPlayAppDefinition,
	trinityRewritePlayAppDefinition,
	presentationPlayAppDefinition,
];

for (const app of ALL_PLAYGROUND_APPS) {
	registerPlaygroundApp(PLAYGROUND_APP_REGISTRY, app);
}

/** @emoji 🔎 Resolves a playground app by dev-host entry kind. */
export function playgroundAppByEntryKind(playEntryKind: string): PlaygroundAppDefinition | undefined {
	return PLAYGROUND_APP_REGISTRY.get(playEntryKind);
}
//#endregion 🔖Registry
