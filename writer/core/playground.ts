// #region 🧲Header
/** @emoji 🛝 Writer playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import {
	Platform,
	Playground,
	createProductPlaygroundPlatform,
	isPlaygroundFixtureLocked,
	isPlaygroundNoFixtureId,
	playgroundResolvedFixtureId,
	type PlaygroundAppDefinition,
} from "@semio-tech/framework-playground-core";
import { WRITER_PLAY_FIXTURE_DEFAULT_ID, resolveWriterPlayFixtureSlug } from "./fixture-slugs.ts";
import {
	WRITER_PLAY_APP_ID,
	WriterPlayController,
	buildWriterPlayAppRuntime,
	registerWriterPlayDeclarativeBodies,
	type WriterPlayFixtureAccess,
} from "./index.ts";

const writerFixtureModules = import.meta.glob("../fixture/*.writer.json", { eager: true }) as Record<string, { default: unknown }>;

function writerFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.writer\.json$/, "");
}

const WRITER_PLAY_FILE_FIXTURE_JSON_BY_ID: Record<string, string> = Object.fromEntries(
	Object.entries(writerFixtureModules).map(([path, mod]) => {
		const id = writerFixtureIdFromGlobPath(path);
		const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
		return [id, json];
	}),
);

/** @emoji 📁 Writer play fixture access for controller and playground bootstrap. */
export function createWriterPlayFixtureAccess(): WriterPlayFixtureAccess {
	return {
		jsonById: (fixtureId) => WRITER_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId],
		options: Object.keys(WRITER_PLAY_FILE_FIXTURE_JSON_BY_ID)
			.sort()
			.map((id) => ({ id: id === "jack" ? WRITER_PLAY_FIXTURE_DEFAULT_ID : id, label: id === "jack" ? "Jack" : id })),
	};
}

/** @emoji 🛝 Writer playground app. */
export class PlaygroundWriter extends Playground {
	readonly id = WRITER_PLAY_APP_ID;

	createRuntime(): Platform {
		const fixtureAccess = createWriterPlayFixtureAccess();
		const locked = isPlaygroundFixtureLocked();
		const noFixture = isPlaygroundNoFixtureId();
		const fixtureId = playgroundResolvedFixtureId(WRITER_PLAY_FIXTURE_DEFAULT_ID, resolveWriterPlayFixtureSlug);
		const json = fixtureAccess.jsonById(fixtureId) ?? fixtureAccess.jsonById("jack")!;
		if (locked || noFixture) {
			void json;
		}
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new WriterPlayController(runtime.commandBus, () => runtime.notify(), json, fixtureAccess);
		const resolved = playgroundResolvedFixtureId(WRITER_PLAY_FIXTURE_DEFAULT_ID, resolveWriterPlayFixtureSlug);
		if (!locked && !noFixture) {
			ctrl.run("setActiveFixture", { fixtureId: resolved });
		}
		runtime.addApp(buildWriterPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerWriterPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Writer playground app definition. */
export const writerPlayAppDefinition: PlaygroundAppDefinition = {
	id: WRITER_PLAY_APP_ID,
	label: "Writer",
	controllerId: "writer-play",
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundWriter(),
	bootRenderer: async (pg) => {
		const { bootWriterPlay } = await import("@semio-tech/framework-playground-renderer-react/writer");
		bootWriterPlay(pg);
	},
	devHost: {
		playEntryKind: "writer",
		resolveDedupe: ["react", "react-dom", "three", "@semio-tech/writer-react"],
		optimizeDeps: { include: ["react", "react-dom"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition

export { WRITER_PLAY_FIXTURE_DEFAULT_ID, resolveWriterPlayFixtureSlug };
