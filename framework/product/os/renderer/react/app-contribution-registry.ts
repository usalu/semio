// #region 🧲Header
/** @emoji 🗂️ OS app renderer contribution registry — derived from playground app manifests. */
// #endregion 🧲Header

import type { AppInstanceHostComponent, AppRendererContribution } from "@semio-tech/framework-platform-core";
import { applyAppRendererContribution } from "@semio-tech/framework-playground-renderer-react";

//#region 🔖ProgramKindMap
let programIdToPlaygroundKind: Readonly<Record<string, string>> = {};

/** @emoji 🧭 Maps an OS program id to a playground manifest kind. */
export function playgroundKindForProgramId(programId: string): string {
	return programIdToPlaygroundKind[programId] ?? programId;
}
//#endregion 🔖ProgramKindMap

//#region 🔖Registry
type PlaygroundAppImports = Readonly<Record<string, () => Promise<{ readonly loadRenderer?: () => Promise<AppRendererContribution> }>>>;

const contributionByKind = new Map<string, AppRendererContribution>();
const contributionLoads = new Map<string, Promise<AppRendererContribution | undefined>>();

async function loadPlaygroundAppImports(): Promise<PlaygroundAppImports> {
	const mod = (await import("virtual:semio-playground-apps")) as {
		playgroundAppImports: PlaygroundAppImports;
		programIdToPlaygroundKind?: Readonly<Record<string, string>>;
	};
	if (mod.programIdToPlaygroundKind) {
		programIdToPlaygroundKind = mod.programIdToPlaygroundKind;
	}
	return mod.playgroundAppImports;
}

/** @emoji 📦 Loads and caches one app renderer contribution by manifest kind. */
export async function ensureOsAppContributionByKind(kind: string): Promise<AppRendererContribution | undefined> {
	const cached = contributionByKind.get(kind);
	if (cached) return cached;
	const pending = contributionLoads.get(kind);
	if (pending) return pending;
	const loadPromise = (async () => {
		const imports = await loadPlaygroundAppImports();
		const loader = imports[kind];
		if (!loader) return undefined;
		const appDefinition = await loader();
		if (!appDefinition.loadRenderer) return undefined;
		const contribution = await appDefinition.loadRenderer();
		if (contribution.preload) await contribution.preload();
		applyAppRendererContribution(contribution);
		contributionByKind.set(kind, contribution);
		return contribution;
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
	const imports = await loadPlaygroundAppImports();
	await Promise.all(Object.keys(imports).map((kind) => ensureOsAppContributionByKind(kind)));
}
//#endregion 🔖Registry
