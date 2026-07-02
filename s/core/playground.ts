// #region 🧲Header
/** @emoji 🛝 S playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import {
	CommandBus,
	Platform,
	Playground,
	createProductPlaygroundPlatform,
	eagerPlayFixtureGlob,
	playgroundResolvedFixtureId,
	type PlaygroundAppDefinition,
} from "@semio-tech/framework-playground-core";
import {
	parseSStudioDocument,
	registerSFixtureJsonResolver,
	type SStudioDocumentV1,
} from "./internal.ts";
import {
	S_PLAY_APP_ID,
	S_PLAY_CONTROLLER_ID,
	SPlayController,
	buildSPlayAppRuntime,
	createStudioStore,
	registerSPlayDeclarativeBodies,
} from "./index.ts";
import { S_PLAY_FIXTURE_DEFAULT_ID, resolveSPlayFixtureSlug } from "./fixture-slugs.ts";

export { S_PLAY_FIXTURE_DEFAULT_ID, resolveSPlayFixtureSlug };

let sPlayFixtureJsonByIdCache: Readonly<Record<string, string>> | undefined;
let sFixtureResolverRegistered = false;

function fixtureSlugFromPath(path: string): string {
	return path.split("/").pop() ?? path;
}

function sFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.s\.json$/, "");
}

function ensureSPlayFixtureCatalog(): Readonly<Record<string, string>> {
	if (sPlayFixtureJsonByIdCache) return sPlayFixtureJsonByIdCache;
	const sFixtureModules = eagerPlayFixtureGlob("../fixture/*.s.json");
	const technologyFixtureModules = eagerPlayFixtureGlob(["../../draw/fixture/*.json", "../../writer/fixture/*.json"]);
	const slugJsonByPath = Object.fromEntries(
		Object.entries(technologyFixtureModules).map(([path, module]) => [fixtureSlugFromPath(path), JSON.stringify(module.default)]),
	);
	if (!sFixtureResolverRegistered) {
		registerSFixtureJsonResolver((slug) => slugJsonByPath[slug] ?? null);
		sFixtureResolverRegistered = true;
	}
	sPlayFixtureJsonByIdCache = Object.fromEntries(
		Object.entries(sFixtureModules).map(([path, module]) => [sFixtureIdFromGlobPath(path), JSON.stringify(module.default)]),
	);
	return sPlayFixtureJsonByIdCache;
}

export function getSPlayFixtureJsonById(): Readonly<Record<string, string>> {
	return ensureSPlayFixtureCatalog();
}

export const S_PLAY_FIXTURE_OPTIONS = (): ReadonlyArray<{ readonly id: string; readonly label: string }> =>
	Object.keys(ensureSPlayFixtureCatalog())
		.sort()
		.map((id) => ({ id, label: id }));

/** @emoji 📂 Loads an s studio document from a playground fixture id. */
export function loadSPlayStudioDocument(fixtureId: string): SStudioDocumentV1 {
	const json = ensureSPlayFixtureCatalog()[fixtureId];
	if (!json) throw new Error(`unknown s fixture: ${fixtureId}`);
	return parseSStudioDocument(JSON.parse(json));
}

/** @emoji 🎮 Creates an s play controller wired to playground fixtures. */
export function createSPlayController(
	commandBus: CommandBus,
	notify: () => void,
	fixtureId: string = S_PLAY_FIXTURE_DEFAULT_ID,
): SPlayController {
	const resolved = playgroundResolvedFixtureId(fixtureId);
	const store = createStudioStore(loadSPlayStudioDocument(resolved));
	return new SPlayController(commandBus, notify, store, resolved, loadSPlayStudioDocument);
}

/** @emoji 🧪 Test helper for s play controller with fixture. */
export function createSPlayTestController(fixtureId: string): SPlayController {
	const bus = new CommandBus();
	return createSPlayController(bus, () => {}, fixtureId);
}

/** @emoji 🛝 S playground app. */
export class PlaygroundS extends Playground {
	readonly id = S_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id, "S");
		const resolved = playgroundResolvedFixtureId(S_PLAY_FIXTURE_DEFAULT_ID);
		const ctrl = createSPlayController(runtime.commandBus, () => runtime.notify(), resolved);
		runtime.addApp(buildSPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerSPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 S playground app definition. */
export const sPlayAppDefinition: PlaygroundAppDefinition = {
	id: S_PLAY_APP_ID,
	label: "S",
	controllerId: S_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundS(),
	bootRenderer: async (pg) => {
		const { bootSPlay } = await import("@semio-tech/framework-playground-renderer-react/s");
		bootSPlay(pg);
	},
	devHost: {
		playEntryKind: "s",
		resolveDedupe: ["react", "react-dom", "@semio-tech/s-react"],
		optimizeDeps: { include: ["react", "react-dom"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("S_PLAY_FIXTURE_OPTIONS", () => {
		it("includes demo fixture", () => {
			expect(S_PLAY_FIXTURE_OPTIONS().some((row) => row.id === "demo")).toBe(true);
		});
	});
}
// #endregion 🧪Tests
