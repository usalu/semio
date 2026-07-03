// #region 🧲Header
/** @emoji 🗂️ `@semio-tech/framework-playground-core` — lazy registry of playground app definitions keyed by dev host entry kind. */
// #endregion 🧲Header

import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { scanPlaygroundAppManifests } from "../../../../repo/lib/js/index.ts";
import type { PlaygroundAppDefinition } from "./index.ts";

//#region 🔖Registry
const PLAY_ENTRY_KIND =
	typeof import.meta.env !== "undefined" && typeof import.meta.env.PUZZLE_PLAY_ENTRY === "string"
		? import.meta.env.PUZZLE_PLAY_ENTRY
		: "";

const REPO_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../..");

const ALL_PLAY_ENTRY_KINDS = scanPlaygroundAppManifests(REPO_ROOT).map((entry) => entry.kind);

/** @emoji 🗂️ Loaded playground apps keyed by {@link AppDevHostConfig.playEntryKind}. */
export const PLAYGROUND_APP_REGISTRY = new Map<string, PlaygroundAppDefinition>();

function registerPlaygroundApp(app: PlaygroundAppDefinition): void {
	const kind = app.devHost?.playEntryKind;
	if (!kind) throw new Error(`Playground app "${app.id}" is missing devHost.playEntryKind`);
	PLAYGROUND_APP_REGISTRY.set(kind, app);
}

type PlaygroundAppImports = Readonly<Record<string, () => Promise<PlaygroundAppDefinition>>>;

/** @emoji 📦 Imports one playground app; unreachable branches are dropped per {@link PLAY_ENTRY_KIND}. */
async function importPlaygroundAppDefinition(kind: string): Promise<PlaygroundAppDefinition> {
	try {
		const { playgroundAppImports } = (await import("virtual:semio-playground-apps")) as {
			playgroundAppImports: PlaygroundAppImports;
		};
		const loader = playgroundAppImports[kind];
		if (!loader) throw new Error(`unknown playground app: ${kind}`);
		return loader();
	} catch (error) {
		if (!(import.meta.env.VITEST || process.env.VITEST)) throw error;
		const { scanPlaygroundAppManifests } = await import("../../../../repo/lib/js/index.ts");
		const manifest = scanPlaygroundAppManifests(REPO_ROOT).find((entry) => entry.kind === kind);
		if (!manifest) throw new Error(`unknown playground app: ${kind}`);
		const mod = (await import(manifest.corePackage)) as Record<string, PlaygroundAppDefinition>;
		const app = mod[manifest.definitionExport];
		if (!app) throw new Error(`missing ${manifest.definitionExport} on ${manifest.corePackage}`);
		return app;
	}
}

/** @emoji 🔎 Lazily loads a playground app by dev-host entry kind. */
export async function loadPlaygroundApp(playEntryKind: string): Promise<PlaygroundAppDefinition | undefined> {
	if (PLAY_ENTRY_KIND && playEntryKind !== PLAY_ENTRY_KIND) return undefined;
	const cached = PLAYGROUND_APP_REGISTRY.get(playEntryKind);
	if (cached) return cached;
	const app = await importPlaygroundAppDefinition(PLAY_ENTRY_KIND || playEntryKind);
	registerPlaygroundApp(app);
	return app;
}

/** @emoji 🔎 Resolves a previously loaded playground app by dev-host entry kind. */
export function playgroundAppByEntryKind(playEntryKind: string): PlaygroundAppDefinition | undefined {
	return PLAYGROUND_APP_REGISTRY.get(playEntryKind);
}

/** @emoji 🗂️ Eagerly loads every registered playground app (tests, audits). */
export async function loadAllPlaygroundApps(): Promise<readonly PlaygroundAppDefinition[]> {
	const kinds = PLAY_ENTRY_KIND ? [PLAY_ENTRY_KIND] : ALL_PLAY_ENTRY_KINDS;
	const apps: PlaygroundAppDefinition[] = [];
	for (const kind of kinds) {
		const app = await loadPlaygroundApp(kind);
		if (app) apps.push(app);
	}
	return apps;
}
//#endregion 🔖Registry
