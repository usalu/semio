// #region 🧲Header
/** @emoji 🛝 Procedural 3D playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, eagerPlayFixtureGlob, isPlaygroundNoFixtureId, PLAYGROUND_NO_FIXTURE_ID, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import { PROCEDURAL_DEFAULT_FIXTURE, proceduralFixtureToJson, type FlowFixtureV1 } from "@semio-tech/procedural-3d-react";
import {
	PROCEDURAL_3D_PLAY_APP_ID,
	PROCEDURAL_3D_PLAY_CONTROLLER_ID,
	ProceduralPlayController,
	buildProceduralPlayAppRuntime,
	registerProceduralPlayDeclarativeBodies,
} from "./index.ts";

const PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON = proceduralFixtureToJson(PROCEDURAL_DEFAULT_FIXTURE);

import {
    PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID,
    PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID,
    resolveProceduralPlayFixtureSlug,
} from "./fixture-slugs.js";

export { PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID, PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID, resolveProceduralPlayFixtureSlug };

const proceduralFixtureModules = eagerPlayFixtureGlob("../fixture/*.procedural.json");

function proceduralFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.procedural\.json$/, "");
}

function proceduralFixtureLabelFromId(id: string): string {
	return id
		.split("-")
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

const PROCEDURAL_PLAY_FILE_FIXTURE_JSON_BY_ID: Record<string, string> = Object.fromEntries(
	Object.entries(proceduralFixtureModules).map(([path, mod]) => {
		const id = proceduralFixtureIdFromGlobPath(path);
		const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
		return [id, json];
	}),
);

export const PROCEDURAL_PLAY_EMPTY_FIXTURE: FlowFixtureV1 = {
	schema: "flow.fixture/v1",
	camera: { x: 0, y: 0, zoom: 1 },
	widgets: [],
	synapses: [],
};

export const PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON = proceduralFixtureToJson(PROCEDURAL_PLAY_EMPTY_FIXTURE);

export const PROCEDURAL_PLAY_FIXTURE_OPTIONS: ReadonlyArray<{ readonly id: string; readonly label: string }> = [
	{ id: PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID, label: "Box fillet move" },
	...Object.keys(PROCEDURAL_PLAY_FILE_FIXTURE_JSON_BY_ID)
		.sort()
		.map((id) => ({ id, label: proceduralFixtureLabelFromId(id) })),
];

function proceduralFixtureJsonForId(fixtureId: string): string {
	if (isPlaygroundNoFixtureId(fixtureId)) {
		return proceduralFixtureToJson(PROCEDURAL_PLAY_EMPTY_FIXTURE);
	}
	if (fixtureId === PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID) {
		return PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON;
	}
	const fileJson = PROCEDURAL_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId];
	if (fileJson) return fileJson;
	return PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON;
}

/** @emoji 🧪 Resolves procedural play fixture JSON by catalog id. */
export function proceduralPlayFixtureJson(fixtureId: string = PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID): string {
	return proceduralFixtureJsonForId(fixtureId);
}



/** @emoji 🛝 Procedural playground app. */
export class PlaygroundProcedural extends Playground {
	readonly id = PROCEDURAL_3D_PLAY_APP_ID;
	readonly keybindings = [
		{ key: "ctrl+a,meta+a", controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "selectAll" },
		{ key: "Delete", controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
		{ key: "Backspace", controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
	];

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new ProceduralPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildProceduralPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerProceduralPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Procedural 3D playground app definition. */
export const procedural3dPlayAppDefinition: PlaygroundAppDefinition = {
	id: PROCEDURAL_3D_PLAY_APP_ID,
	label: "Procedural 3D",
	controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundProcedural(),
	bootRenderer: async (pg) => {
		const { bootProceduralPlay } = await import("@semio-tech/framework-playground-renderer-react/procedural-3d");
		bootProceduralPlay(pg);
	},
	devHost: {
		playEntryKind: "procedural-3d",
		resolveDedupe: ["react", "react-dom", "three", "scheduler", "@semio-tech/flow-react", "@semio-tech/procedural-3d-react"],
		watchIgnored: ["../../../flow/core/lib.rs",
		"../../../flow/core/target/**",
		"../../../flow/module/**/lib.rs",
		"../../../flow/module/**/target/**",],
		optimizeDeps: { include: [
			"react",
			"react-dom",
			"three",
			"@react-three/fiber",
			"@react-three/drei",
			"@semio-tech/infinite-world-r3f",
			"@semio-tech/flow-react",
			"@semio-tech/procedural-3d-react",
		] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
