// #region 🧲Header
/** @emoji 🛝 Procedural 2D playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, eagerPlayFixtureGlob, isPlaygroundNoFixtureId, PLAYGROUND_NO_FIXTURE_ID, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import { PROCEDURAL_DEFAULT_FIXTURE, proceduralFixtureToJson, type FlowFixtureV1 } from "@semio-tech/procedural-2d-react";
import {
	PROCEDURAL_2D_PLAY_APP_ID,
	PROCEDURAL_2D_PLAY_CONTROLLER_ID,
	Procedural2dPlayController,
	buildProcedural2dPlayAppRuntime,
	registerProcedural2dPlayDeclarativeBodies,
} from "./index.ts";

const PROCEDURAL_2D_PLAY_DEFAULT_FIXTURE_JSON = proceduralFixtureToJson(PROCEDURAL_DEFAULT_FIXTURE);

import { PROCEDURAL_2D_PLAY_FIXTURE_DEFAULT_ID, resolveProcedural2dPlayFixtureSlug } from "./fixture-slugs.js";

export { PROCEDURAL_2D_PLAY_FIXTURE_DEFAULT_ID, resolveProcedural2dPlayFixtureSlug };

const proceduralFixtureModules = eagerPlayFixtureGlob("../fixture/*.procedural2d.json");

function proceduralFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.procedural2d\.json$/, "");
}

function proceduralFixtureLabelFromId(id: string): string {
	return id
		.split("-")
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

const PROCEDURAL_2D_PLAY_FILE_FIXTURE_JSON_BY_ID: Record<string, string> = Object.fromEntries(
	Object.entries(proceduralFixtureModules).map(([path, mod]) => {
		const id = proceduralFixtureIdFromGlobPath(path);
		const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
		return [id, json];
	}),
);

export const PROCEDURAL_2D_PLAY_EMPTY_FIXTURE: FlowFixtureV1 = {
	schema: "flow.fixture/v1",
	camera: { x: 0, y: 0, zoom: 1 },
	widgets: [],
	synapses: [],
};

export const PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON = proceduralFixtureToJson(PROCEDURAL_2D_PLAY_EMPTY_FIXTURE);

export const PROCEDURAL_2D_PLAY_FIXTURE_OPTIONS: ReadonlyArray<{ readonly id: string; readonly label: string }> = [
	{ id: PROCEDURAL_2D_PLAY_FIXTURE_DEFAULT_ID, label: "Draw rect + fill" },
	...Object.keys(PROCEDURAL_2D_PLAY_FILE_FIXTURE_JSON_BY_ID)
		.sort()
		.map((id) => ({ id, label: proceduralFixtureLabelFromId(id) })),
];


function proceduralFixtureJsonForId(fixtureId: string): string {
	if (isPlaygroundNoFixtureId(fixtureId)) {
		return proceduralFixtureToJson(PROCEDURAL_2D_PLAY_EMPTY_FIXTURE);
	}
	if (fixtureId === PROCEDURAL_2D_PLAY_FIXTURE_DEFAULT_ID) {
		return PROCEDURAL_2D_PLAY_DEFAULT_FIXTURE_JSON;
	}
	const fileJson = PROCEDURAL_2D_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId];
	if (fileJson) return fileJson;
	return PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON;
}

/** @emoji 🧪 Resolves procedural play fixture JSON by catalog id. */
export function procedural2dPlayFixtureJson(fixtureId: string = PROCEDURAL_2D_PLAY_FIXTURE_DEFAULT_ID): string {
	return proceduralFixtureJsonForId(fixtureId);
}



/** @emoji 🛝 Procedural playground app. */
export class PlaygroundProcedural2d extends Playground {
	readonly id = PROCEDURAL_2D_PLAY_APP_ID;
	readonly keybindings = [
		{ key: "ctrl+a,meta+a", controllerId: PROCEDURAL_2D_PLAY_CONTROLLER_ID, command: "selectAll" },
		{ key: "Delete", controllerId: PROCEDURAL_2D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
		{ key: "Backspace", controllerId: PROCEDURAL_2D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
	];

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new Procedural2dPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildProcedural2dPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerProcedural2dPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Procedural 2D playground app definition. */
export const procedural2dPlayAppDefinition: PlaygroundAppDefinition = {
	id: PROCEDURAL_2D_PLAY_APP_ID,
	label: "Procedural 2D",
	controllerId: PROCEDURAL_2D_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundProcedural2d(),
	bootRenderer: async (pg) => {
		const { bootProcedural2dPlay } = await import("@semio-tech/framework-playground-renderer-react/procedural-2d");
		bootProcedural2dPlay(pg);
	},
	devHost: {
		playEntryKind: "procedural-2d",
		resolveDedupe: ["react", "react-dom", "scheduler", "@semio-tech/flow-react", "@semio-tech/procedural-2d-react"],
		watchIgnored: ["../../../flow/core/lib.rs",
		"../../../flow/core/target/**",
		"../../../flow/module/**/lib.rs",
		"../../../flow/module/**/target/**",],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "@semio-tech/infinite-cavas-react-renderer"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
