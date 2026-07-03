// #region 🧲Header
/** @emoji 🗂️ `@semio-tech/framework-playground-core` — lazy registry of playground app definitions keyed by dev host entry kind. */
// #endregion 🧲Header

import type { PlaygroundAppDefinition } from "./index.ts";

//#region 🔖Registry
function isVitestRuntime(): boolean {
	if (import.meta.env?.VITEST) return true;
	return typeof process !== "undefined" && Boolean(process.env?.VITEST);
}
const PLAY_ENTRY_KIND =
	typeof import.meta.env !== "undefined" && typeof import.meta.env.PLAYGROUND_APP_KIND === "string"
		? import.meta.env.PLAYGROUND_APP_KIND
		: "";

/** @emoji 🗂️ Loaded playground apps keyed by {@link AppDevHostConfig.playEntryKind}. */
export const PLAYGROUND_APP_REGISTRY = new Map<string, PlaygroundAppDefinition>();

function registerPlaygroundApp(app: PlaygroundAppDefinition): void {
	const kind = app.devHost?.playEntryKind;
	if (!kind) throw new Error(`Playground app "${app.id}" is missing devHost.playEntryKind`);
	PLAYGROUND_APP_REGISTRY.set(kind, app);
}

type PlaygroundAppImports = Readonly<Record<string, () => Promise<PlaygroundAppDefinition>>>;

async function repoRoot(): Promise<string> {
	const { dirname, resolve } = await import("node:path");
	const { fileURLToPath } = await import("node:url");
	return resolve(dirname(fileURLToPath(import.meta.url)), "../../../../..");
}

async function resolveAllPlayEntryKinds(): Promise<readonly string[]> {
	if (PLAY_ENTRY_KIND) return [PLAY_ENTRY_KIND];
	try {
		const { playgroundAppImports } = (await import("virtual:semio-playground-apps")) as {
			playgroundAppImports: PlaygroundAppImports;
		};
		return Object.keys(playgroundAppImports);
	} catch (error) {
		if (!isVitestRuntime()) throw error;
		const { scanPlaygroundAppManifests } = await import("../../../../../repo/lib/js/playground-manifest.ts");
		return scanPlaygroundAppManifests(await repoRoot()).map((entry) => entry.kind);
	}
}

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
		if (!isVitestRuntime()) throw error;
		const { scanPlaygroundAppManifests } = await import("../../../../../repo/lib/js/playground-manifest.ts");
		const manifest = scanPlaygroundAppManifests(await repoRoot()).find((entry) => entry.kind === kind);
		if (!manifest) throw new Error(`unknown playground app: ${kind}`);
		const mod = (await import(manifest.corePackage)) as Record<string, PlaygroundAppDefinition>;
		const app = mod[manifest.definitionExport];
		if (!app) throw new Error(`missing ${manifest.definitionExport} on ${manifest.corePackage}`);
		return app;
	}
}

/** @emoji 📦 Resolves a renderer export — supports async factory functions. */
export async function resolvePlaygroundRendererExport(exported: unknown): Promise<import("@semio-tech/framework-platform-core").AppRendererContribution> {
	if (typeof exported === "function") {
		const result = (exported as () => unknown | Promise<unknown>)();
		if (result && typeof (result as Promise<unknown>).then === "function") {
			return (await result) as import("@semio-tech/framework-platform-core").AppRendererContribution;
		}
		return result as import("@semio-tech/framework-platform-core").AppRendererContribution;
	}
	return exported as import("@semio-tech/framework-platform-core").AppRendererContribution;
}

/** @emoji 📦 Loads a renderer contribution by manifest kind (virtual module or vitest manifest scan). */
export async function loadPlaygroundRendererContribution(playEntryKind: string): Promise<import("@semio-tech/framework-platform-core").AppRendererContribution> {
	try {
		const { playgroundRendererImports } = (await import("virtual:semio-playground-apps")) as {
			playgroundRendererImports: Readonly<Record<string, () => Promise<import("@semio-tech/framework-platform-core").AppRendererContribution>>>;
		};
		const loader = playgroundRendererImports[playEntryKind];
		if (!loader) throw new Error(`unknown playground renderer: ${playEntryKind}`);
		return loader();
	} catch (error) {
		if (!isVitestRuntime()) throw error;
		const { scanPlaygroundAppManifests } = await import("../../../../../repo/lib/js/playground-manifest.ts");
		const manifest = scanPlaygroundAppManifests(await repoRoot()).find((entry) => entry.kind === playEntryKind);
		if (!manifest) throw new Error(`unknown playground renderer: ${playEntryKind}`);
		const mod = (await import(manifest.rendererPackage)) as Record<string, unknown>;
		const exported = mod[manifest.rendererExport];
		if (!exported) throw new Error(`missing ${manifest.rendererExport} on ${manifest.rendererPackage}`);
		return resolvePlaygroundRendererExport(exported);
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
	const kinds = await resolveAllPlayEntryKinds();
	const apps: PlaygroundAppDefinition[] = [];
	for (const kind of kinds) {
		const app = await loadPlaygroundApp(kind);
		if (app) apps.push(app);
	}
	return apps;
}

type PlaygroundProgramImports = Readonly<Record<string, () => Promise<import("@semio-tech/framework-platform-core").OsProgramContribution>>>;

/** @emoji 🧩 Loads every manifest-declared OS program contribution (vite virtual module or vitest manifest scan). */
export async function loadAllOsProgramContributions(): Promise<readonly import("@semio-tech/framework-platform-core").OsProgramContribution[]> {
	try {
		const { playgroundProgramImports } = (await import("virtual:semio-playground-apps")) as {
			playgroundProgramImports: PlaygroundProgramImports;
		};
		const loaders = Object.values(playgroundProgramImports);
		if (loaders.length > 0) {
			return Promise.all(loaders.map((loader) => loader()));
		}
	} catch (error) {
		if (!isVitestRuntime()) throw error;
	}
	if (!isVitestRuntime()) return [];
	const { scanPlaygroundAppManifests } = await import("../../../../../repo/lib/js/playground-manifest.ts");
	const manifests = scanPlaygroundAppManifests(await repoRoot()).filter((entry) => entry.programExport);
	const loaded = await Promise.all(
		manifests.map(async (manifest) => {
			try {
				const mod = (await import(manifest.corePackage)) as Record<string, import("@semio-tech/framework-platform-core").OsProgramContribution>;
				const contribution = mod[manifest.programExport!];
				if (!contribution) throw new Error(`missing ${manifest.programExport} on ${manifest.corePackage}`);
				return contribution;
			} catch (error) {
				if (!isVitestRuntime()) throw error;
				return null;
			}
		}),
	);
	return loaded.filter((contribution): contribution is import("@semio-tech/framework-platform-core").OsProgramContribution => contribution !== null);
}
//#endregion 🔖Registry
