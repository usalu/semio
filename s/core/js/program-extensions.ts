/** @emoji 🧩 Loads all technology program extensions into the s registry. */

import {
	COMPOSE_SKETCHPAD_PROGRAM_ID,
	mergeSProgramDefinition,
	registerAppVcsHandler,
	createFlowDocumentAppVcsHandler,
	createFlowDagAppVcsHandler,
	createProcedural2dAppVcsHandler,
	createProcedural3dAppVcsHandler,
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
} from "./internal.ts";
import { createDrawAppVcsHandler } from "@semio-tech/draw-core";
import { createWriterAppVcsHandler } from "@semio-tech/writer-core";
import { createRasterAppVcsHandler } from "@semio-tech/raster-core";
import { createFormsAppVcsHandler } from "@semio-tech/forms-core";
import { createSPlayShootingAppVcsHandler } from "./shooting-extension.ts";
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
		const { buildNoteProgramDefinition, createNoteAppVcsHandler } = await import("@semio-tech/note-core");
		mergeSProgramDefinition("note", buildNoteProgramDefinition());
		registerAppVcsHandler(createNoteAppVcsHandler());
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
		const { buildPuzzle2dProgramDefinition } = await import("@semio-tech/puzzle-2d-core");
		mergeSProgramDefinition("puzzle.2d", buildPuzzle2dProgramDefinition());
		registerAppVcsHandler(createPuzzle2dAppVcsHandler());
	},
	async () => {
		const { buildPuzzle3dProgramDefinition } = await import("@semio-tech/puzzle-3d-core");
		mergeSProgramDefinition("puzzle.3d", buildPuzzle3dProgramDefinition());
		registerAppVcsHandler(createPuzzle3dAppVcsHandler());
	},
	async () => {
		const { buildPuzzle5dProgramDefinition, createSPlayPuzzle5dAppVcsHandler } = await import("./puzzle5d-extension.ts");
		mergeSProgramDefinition("puzzle.5d", buildPuzzle5dProgramDefinition());
		registerAppVcsHandler(createSPlayPuzzle5dAppVcsHandler());
	},
	async () => {
		const { buildTrinityProgramDefinition } = await import("@semio-tech/trinity-jack-host-core");
		mergeSProgramDefinition("trinity", buildTrinityProgramDefinition());
		registerAppVcsHandler(createTrinityGraphAppVcsHandler());
	},
	async () => {
		const { buildTrinityRewriteProgramDefinition } = await import("@semio-tech/trinity-rewrite-core");
		mergeSProgramDefinition("trinity.rewrite", buildTrinityRewriteProgramDefinition());
		registerAppVcsHandler(createTrinityGraphAppVcsHandler());
	},
	async () => {
		const { buildShootingProgramDefinition } = await import("@semio-tech/shooting-core");
		mergeSProgramDefinition("shooting", buildShootingProgramDefinition());
		registerAppVcsHandler(createSPlayShootingAppVcsHandler());
	},
	async () => {
		const { buildProcedural2dProgramDefinition } = await import("@semio-tech/procedural-2d-core");
		mergeSProgramDefinition("procedural.2d", buildProcedural2dProgramDefinition());
		registerAppVcsHandler(createProcedural2dAppVcsHandler());
	},
	async () => {
		const { buildProcedural3dProgramDefinition } = await import("@semio-tech/procedural-3d-core");
		mergeSProgramDefinition("procedural.3d", buildProcedural3dProgramDefinition());
		registerAppVcsHandler(createProcedural3dAppVcsHandler());
	},
	async () => {
		const { buildGisMapProgramDefinition } = await import("@semio-tech/gis-2d-core");
		mergeSProgramDefinition("gis.map", buildGisMapProgramDefinition());
		registerAppVcsHandler(createGisMapAppVcsHandler());
	},
	async () => {
		const { buildPresentationDeckProgramDefinition } = await import("@semio-tech/framework-presentation-core");
		mergeSProgramDefinition("presentation.deck", buildPresentationDeckProgramDefinition());
		registerAppVcsHandler(createPresentationDeckAppVcsHandler());
	},
	async () => {
		const { buildPresentationProgramDefinition } = await import("@semio-tech/framework-presentation-core");
		mergeSProgramDefinition("presentation", buildPresentationProgramDefinition());
		registerAppVcsHandler(createPresentationAppVcsHandler());
	},
	async () => {
		const { buildSequenceProgramDefinition } = await import("@semio-tech/sequence-core");
		mergeSProgramDefinition("sequence", buildSequenceProgramDefinition());
		registerAppVcsHandler(createSequenceAppVcsHandler());
	},
	async () => {
		const { buildLayoutProgramDefinition } = await import("@semio-tech/layout-core");
		mergeSProgramDefinition("layout", buildLayoutProgramDefinition());
		registerAppVcsHandler(createLayoutAppVcsHandler());
	},
	async () => {
		const { buildImperativeProgramDefinition } = await import("@semio-tech/imperative-core");
		mergeSProgramDefinition("imperative", buildImperativeProgramDefinition());
		registerAppVcsHandler(createImperativeAppVcsHandler());
	},
	async () => {
		const { buildLowpolyProgramDefinition } = await import("@semio-tech/lowpoly-core");
		mergeSProgramDefinition("lowpoly", buildLowpolyProgramDefinition());
		registerAppVcsHandler(createLowpolyAppVcsHandler());
	},
	async () => {
		const { buildVcsProgramDefinition } = await import("@semio-tech/vcs-core");
		mergeSProgramDefinition("vcs", buildVcsProgramDefinition());
		registerAppVcsHandler(createVcsDemoAppVcsHandler());
	},
	async () => {
		const { buildDagProgramDefinition } = await import("@semio-tech/dag-host-core");
		mergeSProgramDefinition("dag", buildDagProgramDefinition());
		registerAppVcsHandler(createFlowDagAppVcsHandler());
	},
	async () => {
		const { buildCadProgramDefinition } = await import("@semio-tech/cad-js-renderer-core");
		mergeSProgramDefinition("cad", buildCadProgramDefinition());
	},
	async () => {
		const { buildSketchpadProgramDefinition } = await import("@semio-tech/compose-sketchpad");
		mergeSProgramDefinition(COMPOSE_SKETCHPAD_PROGRAM_ID, buildSketchpadProgramDefinition());
	},
	async () => {
		const { buildReasoningWiresProgramDefinition } = await import("@semio-tech/reasoning-mindmap-wires-core");
		mergeSProgramDefinition("reasoning.wires", buildReasoningWiresProgramDefinition());
		registerAppVcsHandler(createPuzzle2dAppVcsHandler());
	},
	async () => {
		mergeSProgramDefinition("reasoning.mindmap", {
			id: "reasoning.mindmap",
			name: "Reasoning Mindmap",
			apiVersion: "1",
			apps: [{ id: "mindmap", label: "Mindmap", controllerId: "reasoning-mindmap", modes: [{ id: "explore", label: "Explore" }], defaultModeId: "explore" }],
			createPlatformApi: () => ({}),
		});
		registerAppVcsHandler(createPuzzle2dAppVcsHandler());
	},
	async () => {
		registerAppVcsHandler(
			createCatalogueKindsAppVcsHandler(() => puzzle5dDefaultManifestCatalogBundle() ?? {}),
		);
	},
];

/** @emoji 🧩 Loads every registered technology extension into the s registry. */
export async function loadAllSProgramExtensions(): Promise<void> {
	for (const loader of extensionLoaders) {
		await loader();
	}
}
