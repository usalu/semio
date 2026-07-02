/** @emoji 🧩 S-play puzzle 5d VCS handler with catalogue input and multi-port projection. */

import { createTypedAppVcsHandler } from "@semio-tech/s-core";
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import {
	parseModel,
	project2d,
	project3d,
	type KindCatalogBundle,
	type Model,
} from "@semio-tech/puzzle-5d-react";

function meshUrlFromModel(model: Model): string | null {
	const fixture3d = project3d(model);
	for (const object of fixture3d.objects) {
		if (object.meshUrl) return object.meshUrl;
	}
	return null;
}

import { puzzle5dPlayAppDefinition } from "@semio-tech/puzzle-5d-core";

export function buildPuzzle5dProgramDefinition(): PlatformDefinition {
	const app = puzzle5dPlayAppDefinition;
	return {
		id: "puzzle.5d",
		name: "Puzzle 5D",
		apiVersion: "1",
		apps: [{ id: "puzzle5d", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}

export function createSPlayPuzzle5dAppVcsHandler() {
	return createTypedAppVcsHandler<Model, { readonly op: "setRevision"; readonly revision: number }>(
		"puzzle.5d",
		"puzzle.5d",
		() => ({
			schema: "puzzle.5d",
			domain: "architecture",
			camera2d: { x: 0, y: 0, zoom: 1 },
			camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
			parts: [],
			fasteners: [],
		}),
		(doc, op) => doc,
		undefined,
		{
			applyInputBindings: (model, inputBindings) => {
				const catalogue = inputBindings.catalogue as KindCatalogBundle | undefined;
				if (!catalogue) return model;
				return { ...model, kindCatalogs: catalogue };
			},
			projectOutput: (model, portId) => {
				if (portId === "graph2d") return project2d(model);
				if (portId === "mesh3d") {
					const url = meshUrlFromModel(model);
					return { url: url ?? "/mesh/base.glb" };
				}
				return model;
			},
		},
	);
}

export function parsePuzzle5dModelFromSource(source: { readonly inline?: string; readonly vcsJson?: string }): Model {
	if (source.inline) return parseModel(JSON.parse(source.inline));
	return parseModel({
		schema: "puzzle.5d",
		domain: "architecture",
		camera2d: { x: 0, y: 0, zoom: 1 },
		camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
		parts: [],
		fasteners: [],
	});
}
