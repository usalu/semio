// #region 🧲Header
/** @emoji 🛝 Puzzle 5D playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, type PlaygroundAppDefinition, type PlaygroundKeybinding } from "@semio-tech/framework-playground-core";
import { parseModel, PUZZLE_5D_SCHEMA, type Model as Puzzle5dModel } from "../react/index.tsx";
import {
	PUZZLE_5D_PLAY_APP_ID,
	PUZZLE_5D_PLAY_CONTROLLER_ID,
	buildPuzzle5dPlayRuntime,
} from "./index.ts";

export const PUZZLE_5D_PLAY_FIXTURE_NAKAGIN_ID = "nakagin";
export const PUZZLE_5D_PLAY_FIXTURE_CONCRETE_FOREST_ID = "concrete-forest";

export const PUZZLE_5D_PLAY_FIXTURE_OPTIONS = [
	{ id: PUZZLE_5D_PLAY_FIXTURE_CONCRETE_FOREST_ID, label: "Concrete Forest" },
	{ id: PUZZLE_5D_PLAY_FIXTURE_NAKAGIN_ID, label: "Nakagin capsule tower" },
] as const;

/** @emoji 🔒 Resolves a playground fixture slug (e.g. `concrete`) to a puzzle 5d fixture id. */
export function resolvePuzzle5dPlayFixtureSlug(slug: string): string | undefined {
	const aliases: Record<string, string> = { concrete: PUZZLE_5D_PLAY_FIXTURE_CONCRETE_FOREST_ID };
	const normalized = aliases[slug] ?? slug;
	return PUZZLE_5D_PLAY_FIXTURE_OPTIONS.some((row) => row.id === normalized) ? normalized : undefined;
}

const PUZZLE_5D_PLAY_FIXTURE_URL_BY_ID: Readonly<Record<string, string>> = {
	[PUZZLE_5D_PLAY_FIXTURE_CONCRETE_FOREST_ID]: "/puzzle-5d-fixture/concrete-forest.5d.json",
	[PUZZLE_5D_PLAY_FIXTURE_NAKAGIN_ID]: "/puzzle-5d-fixture/nakagin-capsule-tower.5d.json",
};

async function readPuzzle5dPlayModelFromDisk(fixtureId: string): Promise<Puzzle5dModel | null> {
	const url = PUZZLE_5D_PLAY_FIXTURE_URL_BY_ID[fixtureId];
	if (!url) return null;
	const fileName = url.split("/").pop();
	if (!fileName) return null;
	const { readFile } = await import("node:fs/promises");
	const { join, dirname } = await import("node:path");
	const { fileURLToPath } = await import("node:url");
	const filePath = join(dirname(fileURLToPath(import.meta.url)), "../fixture", fileName);
	return parseModel(JSON.parse(await readFile(filePath, "utf8")) as unknown);
}

/** @emoji 📥 Loads a play sample by catalog id (browser fetch or disk in non-browser hosts). */
export async function fetchPuzzle5dPlayModel(fixtureId: string): Promise<Puzzle5dModel | null> {
	const url = PUZZLE_5D_PLAY_FIXTURE_URL_BY_ID[fixtureId];
	if (!url) return null;
	if (typeof window === "undefined") {
		return readPuzzle5dPlayModelFromDisk(fixtureId);
	}
	const response = await fetch(url);
	if (!response.ok) return null;
	return parseModel((await response.json()) as unknown);
}

/** @emoji 📭 Empty puzzle 5d model for the no-fixture playground catalog entry. */
export function puzzle5dPlayEmptyModel(): Puzzle5dModel {
	return {
		schema: PUZZLE_5D_SCHEMA,
		domain: "architecture",
		camera2d: { x: 0, y: 0, zoom: 1 },
		camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
		parts: [],
		fasteners: [],
		label: "",
	};
}

/** @emoji 🛝 Puzzle 5d play harness as a single {@link Playground} instance. */
export class Playground5d extends Playground {
	readonly id = PUZZLE_5D_PLAY_APP_ID;
	readonly keybindings: readonly PlaygroundKeybinding[] = [
		{ key: "Delete", controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
		{ key: "Backspace", controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
	];

	createRuntime(): Platform {
		return buildPuzzle5dPlayRuntime();
	}

	registerBodies(): void {
		/* window bodies registered with surface hosts in {@link registerPuzzle5dPlaySurfaceHosts} */
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Puzzle 5D playground app definition. */
export const puzzle5dPlayAppDefinition: PlaygroundAppDefinition = {
	id: PUZZLE_5D_PLAY_APP_ID,
	label: "Puzzle 5D",
	controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new Playground5d(),
	bootRenderer: async (pg) => {
		const { boot5dPlay } = await import("@semio-tech/framework-playground-renderer-react/puzzle/5d");
		boot5dPlay(pg);
	},
	devHost: {
		playEntryKind: "5d",
		resolveDedupe: ["react", "react-dom", "three", "@semio-tech/puzzle-2d-react", "@semio-tech/puzzle-3d-react", "@semio-tech/puzzle-5d-react"],
		optimizeDeps: {
			include: [
				"react",
				"react-dom",
				"react/jsx-runtime",
				"react/jsx-dev-runtime",
				"three",
				"@react-three/fiber",
				"@react-three/drei",
				"lucide-react",
				"@semio-tech/infinite-world-r3f",
				"@semio-tech/puzzle-2d-react",
				"@semio-tech/puzzle-3d-react",
				"@semio-tech/puzzle-5d-react",
			],
		},
	},
};
//#endregion 🔖PlaygroundAppDefinition
