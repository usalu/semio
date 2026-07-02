/** @emoji 🧩 Loads all technology program extensions into the s registry. */

import {
	COMPOSE_SKETCHPAD_PROGRAM_ID,
	mergeSProgramDefinition,
	registerAppVcsHandler,
	createDrawAppVcsHandler,
	createWriterAppVcsHandler,
	createRasterAppVcsHandler,
	createFormsAppVcsHandler,
	createFlowDocumentAppVcsHandler,
	createFlowDagAppVcsHandler,
	createProcedural2dAppVcsHandler,
	createProcedural3dAppVcsHandler,
	createShootingAppVcsHandler,
	createTrinityGraphAppVcsHandler,
	createGisMapAppVcsHandler,
	createPresentationDeckAppVcsHandler,
	createPuzzle2dAppVcsHandler,
	createPuzzle3dAppVcsHandler,
	createCatalogueKindsAppVcsHandler,
	createSequenceAppVcsHandler,
	createLayoutAppVcsHandler,
	createImperativeAppVcsHandler,
	createLowpolyAppVcsHandler,
	createVcsDemoAppVcsHandler,
} from "@semio-tech/s-core";
import { createPresentationAppVcsHandler } from "@semio-tech/framework-presentation-core";
import { puzzle5dDefaultManifestCatalogBundle } from "@semio-tech/puzzle-5d-react";

export type SProgramExtensionLoader = () => Promise<void>;

const extensionLoaders: readonly SProgramExtensionLoader[] = [
	async () => {
		const { buildDrawProgramDefinition } = await import("@semio-tech/draw-core");
		mergeSProgramDefinition("draw", buildDrawProgramDefinition());
		registerAppVcsHandler(createDrawAppVcsHandler());
	},
	async () => {
		const { buildWriterProgramDefinition } = await import("@semio-tech/writer-core");
		mergeSProgramDefinition("writer", buildWriterProgramDefinition());
		registerAppVcsHandler(createWriterAppVcsHandler());
	},
	async () => {
		const { buildRasterProgramDefinition } = await import("@semio-tech/raster-core");
		mergeSProgramDefinition("raster", buildRasterProgramDefinition());
		registerAppVcsHandler(createRasterAppVcsHandler());
	},
	async () => {
		const { buildFlowProgramDefinition } = await import("@semio-tech/flow-core");
		mergeSProgramDefinition("flow", buildFlowProgramDefinition());
		registerAppVcsHandler(createFlowDocumentAppVcsHandler());
	},
	async () => {
		const { buildFormsProgramDefinition } = await import("@semio-tech/forms-core");
		mergeSProgramDefinition("forms", buildFormsProgramDefinition());
		registerAppVcsHandler(createFormsAppVcsHandler());
	},
	async () => {
		const { buildPuzzle2dProgramDefinition } = await import("@semio-tech/puzzle-2d-play");
		mergeSProgramDefinition("puzzle.2d", buildPuzzle2dProgramDefinition());
		registerAppVcsHandler(createPuzzle2dAppVcsHandler());
	},
	async () => {
		const { buildPuzzle3dProgramDefinition } = await import("@semio-tech/puzzle-3d-play");
		mergeSProgramDefinition("puzzle.3d", buildPuzzle3dProgramDefinition());
		registerAppVcsHandler(createPuzzle3dAppVcsHandler());
	},
	async () => {
		const { buildPuzzle5dProgramDefinition, createSPlayPuzzle5dAppVcsHandler } = await import("./puzzle5d-extension.ts");
		mergeSProgramDefinition("puzzle.5d", buildPuzzle5dProgramDefinition());
		registerAppVcsHandler(createSPlayPuzzle5dAppVcsHandler());
	},
	async () => {
		const { buildTrinityProgramDefinition } = await import("@semio-tech/trinity-jack-play");
		mergeSProgramDefinition("trinity", buildTrinityProgramDefinition());
		registerAppVcsHandler(createTrinityGraphAppVcsHandler());
	},
	async () => {
		const { buildShootingProgramDefinition } = await import("@semio-tech/shooting-play");
		const { createSPlayShootingAppVcsHandler } = await import("./shooting-extension.ts");
		mergeSProgramDefinition("shooting", buildShootingProgramDefinition());
		registerAppVcsHandler(createSPlayShootingAppVcsHandler());
	},
	async () => {
		const { buildGisMapProgramDefinition } = await import("@semio-tech/gis-2d-play");
		mergeSProgramDefinition("gis.map", buildGisMapProgramDefinition());
		registerAppVcsHandler(createGisMapAppVcsHandler());
	},
	async () => {
		const { buildCadProgramDefinition } = await import("@semio-tech/cad-js-renderer/play");
		mergeSProgramDefinition("cad", buildCadProgramDefinition());
	},
	async () => {
		const { buildDagProgramDefinition } = await import("@semio-tech/dag-play");
		mergeSProgramDefinition("dag", buildDagProgramDefinition());
		registerAppVcsHandler(createFlowDagAppVcsHandler());
	},
	async () => {
		const { buildProcedural2dProgramDefinition } = await import("@semio-tech/procedural-2d-play");
		mergeSProgramDefinition("procedural.2d", buildProcedural2dProgramDefinition());
		registerAppVcsHandler(createProcedural2dAppVcsHandler());
	},
	async () => {
		const { buildProcedural3dProgramDefinition } = await import("@semio-tech/procedural-3d-play");
		mergeSProgramDefinition("procedural.3d", buildProcedural3dProgramDefinition());
		registerAppVcsHandler(createProcedural3dAppVcsHandler());
	},
	async () => {
		const { buildReasoningWiresProgramDefinition } = await import("@semio-tech/reasoning-mindmap-wires-play");
		mergeSProgramDefinition("reasoning.wires", buildReasoningWiresProgramDefinition());
	},
	async () => {
		const { buildReasoningMindmapProgramDefinition } = await import("@semio-tech/reasoning-mindmap-play");
		mergeSProgramDefinition("reasoning.mindmap", buildReasoningMindmapProgramDefinition());
		registerAppVcsHandler(createPuzzle2dAppVcsHandler());
	},
	async () => {
		const { buildPresentationProgramDefinition } = await import("@semio-tech/framework-presentation-core");
		mergeSProgramDefinition("presentation", buildPresentationProgramDefinition());
		registerAppVcsHandler(createPresentationDeckAppVcsHandler());
		registerAppVcsHandler(createPresentationAppVcsHandler());
	},
	async () => {
		const { buildSketchpadProgramDefinition } = await import("@semio-tech/compose-sketchpad");
		mergeSProgramDefinition(COMPOSE_SKETCHPAD_PROGRAM_ID, buildSketchpadProgramDefinition());
	},
	async () => {
		const { buildLowpolyProgramDefinition } = await import("@semio-tech/lowpoly-play");
		mergeSProgramDefinition("lowpoly", buildLowpolyProgramDefinition());
		registerAppVcsHandler(createLowpolyAppVcsHandler());
	},
	async () => {
		const { buildSequenceProgramDefinition } = await import("@semio-tech/sequence-play");
		mergeSProgramDefinition("sequence", buildSequenceProgramDefinition());
		registerAppVcsHandler(createSequenceAppVcsHandler());
	},
	async () => {
		const { buildLayoutProgramDefinition } = await import("@semio-tech/layout-play");
		mergeSProgramDefinition("layout", buildLayoutProgramDefinition());
		registerAppVcsHandler(createLayoutAppVcsHandler());
	},
	async () => {
		const { buildImperativeProgramDefinition } = await import("@semio-tech/imperative-play");
		mergeSProgramDefinition("imperative", buildImperativeProgramDefinition());
		registerAppVcsHandler(createImperativeAppVcsHandler());
	},
	async () => {
		const { buildVcsProgramDefinition } = await import("@semio-tech/vcs-play");
		mergeSProgramDefinition("vcs", buildVcsProgramDefinition());
		registerAppVcsHandler(createVcsDemoAppVcsHandler());
	},
	async () => {
		registerAppVcsHandler(createCatalogueKindsAppVcsHandler(() => puzzle5dDefaultManifestCatalogBundle() ?? {}));
	},
];

let extensionsLoaded = false;

/** @emoji 📦 Dynamically imports and registers every technology s program extension. */
export async function loadAllSProgramExtensions(): Promise<void> {
	if (extensionsLoaded) return;
	await Promise.all(extensionLoaders.map((loader) => loader()));
	extensionsLoaded = true;
}
