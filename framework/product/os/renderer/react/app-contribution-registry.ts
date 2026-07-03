// #region 🧲Header
/** @emoji 🗂️ OS app renderer contribution registry — derived from playground app manifests. */
// #endregion 🧲Header

import type { AppInstanceHostComponent, AppRendererContribution } from "@semio-tech/framework-platform-core";
import { applyAppRendererContribution } from "@semio-tech/framework-playground-renderer-react";
import { loadPlaygroundRendererContribution } from "@semio-tech/framework-playground-core/app-registry";

//#region 🔖ProgramKindMap
let programIdToPlaygroundKind: Readonly<Record<string, string>> = {};

/** @emoji 🧭 Maps an OS program id to a playground manifest kind. */
export function playgroundKindForProgramId(programId: string): string {
	return programIdToPlaygroundKind[programId] ?? programId;
}
//#endregion 🔖ProgramKindMap

//#region 🔖Registry
type PlaygroundRendererImports = Readonly<Record<string, () => Promise<AppRendererContribution>>>;

const contributionByKind = new Map<string, AppRendererContribution>();
const contributionLoads = new Map<string, Promise<AppRendererContribution | undefined>>();

function isVitestRuntime(): boolean {
	if (import.meta.env?.VITEST) return true;
	return typeof process !== "undefined" && Boolean(process.env?.VITEST);
}

async function loadPlaygroundRendererImports(): Promise<PlaygroundRendererImports> {
	try {
		const mod = (await import("virtual:semio-playground-apps")) as {
			playgroundRendererImports: PlaygroundRendererImports;
			programIdToPlaygroundKind?: Readonly<Record<string, string>>;
		};
		if (mod.programIdToPlaygroundKind) {
			programIdToPlaygroundKind = mod.programIdToPlaygroundKind;
		}
		return mod.playgroundRendererImports;
	} catch (error) {
		if (!isVitestRuntime()) throw error;
		const { dirname, resolve } = await import("node:path");
		const { fileURLToPath } = await import("node:url");
		const { scanPlaygroundAppManifests } = await import("../../../../../repo/lib/js/playground-manifest.ts");
		const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../..");
		const manifests = scanPlaygroundAppManifests(repoRoot);
		const imports: Record<string, () => Promise<AppRendererContribution>> = {};
		for (const manifest of manifests) {
			if (!manifest.rendererPackage || !manifest.rendererExport) continue;
			const kind = manifest.kind;
			const rendererPackage = manifest.rendererPackage;
			const rendererExport = manifest.rendererExport;
			imports[kind] = async () => {
				const mod = (await import(rendererPackage)) as Record<string, AppRendererContribution>;
				const contribution = mod[rendererExport];
				if (!contribution) throw new Error(`missing ${rendererExport} on ${rendererPackage}`);
				return contribution;
			};
		}
		return imports;
	}
}

/** @emoji 📦 Loads and caches one app renderer contribution by manifest kind. */
export async function ensureOsAppContributionByKind(kind: string): Promise<AppRendererContribution | undefined> {
	const cached = contributionByKind.get(kind);
	if (cached) return cached;
	const pending = contributionLoads.get(kind);
	if (pending) return pending;
	const loadPromise = (async () => {
		try {
			const contribution = await loadPlaygroundRendererContribution(kind);
			applyAppRendererContribution(contribution);
			contributionByKind.set(kind, contribution);
			return contribution;
		} catch {
			const imports = await loadPlaygroundRendererImports();
			const loader = imports[kind];
			if (!loader) return undefined;
			const contribution = await loader();
			applyAppRendererContribution(contribution);
			contributionByKind.set(kind, contribution);
			return contribution;
		}
	})();
	contributionLoads.set(kind, loadPromise);
	try {
		return await loadPromise;
	} finally {
		contributionLoads.delete(kind);
	}
}

/** @emoji 📦 Loads and caches one app renderer contribution by OS program id. */
export async function ensureOsAppContribution(programId: string): Promise<AppRendererContribution | undefined> {
	return ensureOsAppContributionByKind(playgroundKindForProgramId(programId));
}

/** @emoji 🖥️ Returns a cached instance host for an OS program id. */
export function resolveInstanceHost(programId: string): AppInstanceHostComponent | undefined {
	return contributionByKind.get(playgroundKindForProgramId(programId))?.instanceHost;
}

/** @emoji 🚀 Preloads every manifest app contribution for S studio boot. */
export async function applyAllOsSurfaceContributions(): Promise<void> {
	const imports = await loadPlaygroundRendererImports();
	const hostKind =
		typeof import.meta.env !== "undefined" && typeof import.meta.env.PLAYGROUND_APP_KIND === "string"
			? import.meta.env.PLAYGROUND_APP_KIND
			: "";
	const kinds = Object.keys(imports).filter((kind) => !hostKind || kind !== hostKind);
	await Promise.all(kinds.map((kind) => ensureOsAppContributionByKind(kind)));
}
//#endregion 🔖Registry
