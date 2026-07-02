// #region 🧲Header
/** @emoji 🛝 Note playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import {
	Platform,
	Playground,
	createProductPlaygroundPlatform,
	eagerPlayFixtureGlob,
	playgroundResolvedFixtureId,
	type PlaygroundAppDefinition,
} from "@semio-tech/framework-playground-core";
import { noteDocumentFromJson } from "./internal.ts";
import {
	NOTE_PLAY_APP_ID,
	NOTE_PLAY_BODY_KEY_COMPOSITE,
	NOTE_PLAY_BODY_KEY_NAVIGATOR,
	NOTE_PLAY_CONTROLLER_ID,
	NOTE_PLAY_SURFACE_ID_COMPOSITE,
	NOTE_PLAY_SURFACE_ID_NAVIGATOR,
	NOTE_PLAY_WINDOW_KIND_COMPOSITE,
	NOTE_PLAY_WINDOW_KIND_NAVIGATOR,
} from "./play-ids.ts";
import {
	NotePlayController,
	buildNotePlayAppRuntime,
	registerNotePlayDeclarativeBodies,
	type NotePlayFixtureHostConfig,
} from "./index.ts";
import { NOTE_PLAY_FIXTURE_DEFAULT_ID } from "./fixture-slugs.ts";

let notePlayFixtureHostCache: NotePlayFixtureHostConfig | undefined;

function noteFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.note\.json$/, "");
}

function noteFixtureLabelFromId(id: string): string {
	return id
		.split("-")
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

/** @emoji 📂 Builds note playground fixture host config. */
export function createNotePlayFixtureHost(): NotePlayFixtureHostConfig {
	if (notePlayFixtureHostCache) return notePlayFixtureHostCache;
	const noteFixtureModules = eagerPlayFixtureGlob("../fixture/*.note.json");
	const fileJsonById = Object.fromEntries(
		Object.entries(noteFixtureModules).map(([path, mod]) => {
			const id = noteFixtureIdFromGlobPath(path);
			const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
			return [id, json];
		}),
	);
	notePlayFixtureHostCache = {
		defaultId: NOTE_PLAY_FIXTURE_DEFAULT_ID,
		options: Object.keys(fileJsonById)
			.sort()
			.map((id) => ({ id, label: noteFixtureLabelFromId(id) })),
		fileJsonById,
	};
	return notePlayFixtureHostCache;
}

/** @emoji 🛝 Note playground app. */
export class PlaygroundNote extends Playground {
	readonly id = NOTE_PLAY_APP_ID;
	readonly keybindings = [{ key: "ctrl+a,meta+a", controllerId: NOTE_PLAY_CONTROLLER_ID, command: "selectAll" }];

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const fixtureHost = createNotePlayFixtureHost();
		const ctrl = new NotePlayController(runtime.commandBus, () => runtime.notify(), fixtureHost);
		const resolved = playgroundResolvedFixtureId(NOTE_PLAY_FIXTURE_DEFAULT_ID);
		if (fixtureHost.fileJsonById[resolved]) ctrl.run("setActiveFixture", { fixtureId: resolved });
		runtime.addApp(buildNotePlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerNotePlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Note playground app definition. */
export const notePlayAppDefinition: PlaygroundAppDefinition = {
	id: NOTE_PLAY_APP_ID,
	label: "Note",
	controllerId: NOTE_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundNote(),
	bootRenderer: async (pg) => {
		const { bootNotePlay } = await import("@semio-tech/framework-playground-renderer-react/note");
		bootNotePlay(pg);
	},
	devHost: {
		playEntryKind: "note",
		resolveDedupe: ["react", "react-dom", "@semio-tech/note-react"],
		optimizeDeps: { include: ["react", "react-dom", "@semio-tech/note-react"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("NOTE_PLAY_FIXTURE_OPTIONS", () => {
		it("includes semio fixture", () => {
			const host = createNotePlayFixtureHost();
			expect(host.options.some((row) => row.id === "semio")).toBe(true);
		});

		it("loads semio fixture blocks", () => {
			const host = createNotePlayFixtureHost();
			const doc = noteDocumentFromJson(host.fileJsonById.semio!);
			expect(doc.blocks.length).toBeGreaterThan(0);
		});
	});
}
// #endregion 🧪Tests
