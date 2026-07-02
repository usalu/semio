// #region 🧲Header
/** @emoji 🛝 Draw playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import {
	Platform,
	Playground,
	createProductPlaygroundPlatform,
	eagerPlayFixtureGlob,
	playgroundResolvedFixtureId,
	type PlaygroundAppDefinition,
} from "@semio-tech/framework-playground-core";
import { drawDocumentFromJson, flattenDrawLayers } from "./internal.ts";
import {
	DRAW_PLAY_APP_ID,
	DRAW_PLAY_CONTROLLER_ID,
	DrawPlayController,
	buildDrawPlayAppRuntime,
	registerDrawPlayDeclarativeBodies,
	type DrawPlayFixtureHostConfig,
} from "./index.ts";
import { DRAW_PLAY_FIXTURE_DEFAULT_ID } from "./fixture-slugs.ts";

let drawPlayFixtureHostCache: DrawPlayFixtureHostConfig | undefined;

function drawFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.draw\.json$/, "");
}

function drawFixtureLabelFromId(id: string): string {
	return id
		.split("-")
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

/** @emoji 📂 Builds draw playground fixture host config. */
export function createDrawPlayFixtureHost(): DrawPlayFixtureHostConfig {
	if (drawPlayFixtureHostCache) return drawPlayFixtureHostCache;
	const drawFixtureModules = eagerPlayFixtureGlob("../fixture/*.draw.json");
	const fileJsonById = Object.fromEntries(
		Object.entries(drawFixtureModules).map(([path, mod]) => {
			const id = drawFixtureIdFromGlobPath(path);
			const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
			return [id, json];
		}),
	);
	drawPlayFixtureHostCache = {
		defaultId: DRAW_PLAY_FIXTURE_DEFAULT_ID,
		options: Object.keys(fileJsonById)
			.sort()
			.map((id) => ({ id, label: drawFixtureLabelFromId(id) })),
		fileJsonById,
	};
	return drawPlayFixtureHostCache;
}

/** @emoji 🛝 Draw playground app. */
export class PlaygroundDraw extends Playground {
	readonly id = DRAW_PLAY_APP_ID;
	readonly keybindings = [{ key: "ctrl+a,meta+a", controllerId: DRAW_PLAY_CONTROLLER_ID, command: "selectAll" }];

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const fixtureHost = createDrawPlayFixtureHost();
		const ctrl = new DrawPlayController(runtime.commandBus, () => runtime.notify(), fixtureHost);
		const resolved = playgroundResolvedFixtureId(DRAW_PLAY_FIXTURE_DEFAULT_ID);
		if (fixtureHost.fileJsonById[resolved]) ctrl.run("setActiveFixture", { fixtureId: resolved });
		runtime.addApp(buildDrawPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerDrawPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Draw playground app definition. */
export const drawPlayAppDefinition: PlaygroundAppDefinition = {
	id: DRAW_PLAY_APP_ID,
	label: "Draw",
	controllerId: DRAW_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundDraw(),
	bootRenderer: async (pg) => {
		const { bootDrawPlay } = await import("@semio-tech/framework-playground-renderer-react/draw");
		bootDrawPlay(pg);
	},
	devHost: {
		playEntryKind: "draw",
		resolveDedupe: ["react", "react-dom", "@semio-tech/draw-react"],
		optimizeDeps: { include: ["react", "react-dom", "@semio-tech/draw-react"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("DRAW_PLAY_FIXTURE_OPTIONS", () => {
		it("includes semio fixture", () => {
			const host = createDrawPlayFixtureHost();
			expect(host.options.some((row) => row.id === "semio")).toBe(true);
		});

		it("renders the semio emblem from only its three authored paths", () => {
			const host = createDrawPlayFixtureHost();
			const doc = drawDocumentFromJson(host.fileJsonById.semio!);
			expect(doc.layers.map((layer) => layer.id)).toEqual(["emblem-group"]);
			expect(flattenDrawLayers(doc.layers).filter((layer) => layer.kind === "path").map((layer) => layer.id)).toEqual([
				"emblem-orange",
				"emblem-red",
				"emblem-teal",
			]);
		});
	});
}
// #endregion 🧪Tests
