//#region 🧲Header
/** @emoji 📋 Playground app manifest scan — Node fs only; safe to dynamic-import without child_process. */
//#endregion 🧲Header

import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

//#region 🔖PlaygroundAppManifest
export type PlaygroundHostKind = string;

export type PlaygroundAssetKind =
	| "puzzle3d-meshes"
	| "gis-tiles"
	| "sketchpad-mdx"
	| "playwright-dev-stub"
	| "vitest-dev-stub"
	| "react-all-extensions";

export type PlaygroundAppManifest = {
	readonly kind: string;
	readonly aliases?: readonly string[];
	readonly packageRoot: string;
	readonly corePackage: string;
	readonly definitionExport: string;
	readonly rendererPackage: string;
	readonly rendererExport: string;
	readonly programExport?: string;
	readonly programId?: string;
	readonly programIds?: readonly string[];
	readonly hostKind?: PlaygroundHostKind;
	readonly port?: { readonly dev: number; readonly test?: number; readonly env?: string };
	readonly site?: { readonly embedKind: string; readonly host: string };
	readonly assets?: readonly PlaygroundAssetKind[];
	readonly lockedExampleFixtures?: Readonly<Record<string, readonly string[]>>;
	readonly optimizeDepsExclude?: readonly string[];
	readonly osProgramContributions?: boolean;
	readonly programContributionKinds?: readonly string[];
};

export type PlaygroundAppManifestEntry = PlaygroundAppManifest & { readonly corePackageJsonPath: string };

/** @emoji 🗺 programId -> playground app kind (from manifest scan). */
export function buildProgramIdToPlaygroundKind(manifests: readonly PlaygroundAppManifestEntry[]): Readonly<Record<string, string>> {
	const map: Record<string, string> = {};
	for (const manifest of manifests) {
		if (manifest.programId) map[manifest.programId] = manifest.kind;
		for (const programId of manifest.programIds ?? []) map[programId] = manifest.kind;
	}
	return map;
}

/** @emoji 📦 Union of Vite asset plugins required by manifests (optionally filtered to one app kind). */
export function collectPlaygroundManifestAssets(
	manifests: readonly PlaygroundAppManifestEntry[],
	activeKind?: string,
): ReadonlySet<PlaygroundAssetKind> {
	const assets = new Set<PlaygroundAssetKind>();
	for (const manifest of manifests) {
		if (activeKind && manifest.kind !== activeKind && !(manifest.aliases ?? []).includes(activeKind)) continue;
		for (const asset of manifest.assets ?? []) assets.add(asset);
	}
	return assets;
}

/** @emoji 🔒 Merged locked-example fixture paths from all manifests. */
export function collectLockedExampleFixturesFromManifests(
	manifests: readonly PlaygroundAppManifestEntry[],
): Readonly<Record<string, readonly string[]>> {
	const merged: Record<string, string[]> = {};
	for (const manifest of manifests) {
		if (!manifest.lockedExampleFixtures) continue;
		for (const [exampleId, paths] of Object.entries(manifest.lockedExampleFixtures)) {
			const bucket = merged[exampleId] ?? [];
			for (const path of paths) {
				if (!bucket.includes(path)) bucket.push(path);
			}
			merged[exampleId] = bucket;
		}
	}
	return merged;
}

/** @emoji ⚡ Merged optimizeDeps.exclude entries from all manifests (optionally filtered to one app kind). */
export function collectPlaygroundOptimizeDepsExclude(
	manifests: readonly PlaygroundAppManifestEntry[],
	activeKind?: string,
): readonly string[] {
	const exclude = new Set<string>();
	for (const manifest of manifests) {
		if (activeKind && manifest.kind !== activeKind && !(manifest.aliases ?? []).includes(activeKind)) continue;
		for (const entry of manifest.optimizeDepsExclude ?? []) exclude.add(entry);
	}
	return [...exclude];
}

const PLAYGROUND_MANIFEST_SKIP_DIRS = new Set(["node_modules", ".git", ".nx", "dist", "target", "storybook-static", ".repo-cache"]);

/** @emoji 📋 Scans workspace package.json files for semio.app / semio.playgroundApp manifests. */
export function scanPlaygroundAppManifests(repoRoot: string): readonly PlaygroundAppManifestEntry[] {
	const entries: PlaygroundAppManifestEntry[] = [];
	const rootPkg = resolve(repoRoot, "package.json");
	const walk = (dir: string): void => {
		const pkgPath = join(dir, "package.json");
		if (existsSync(pkgPath) && pkgPath !== rootPkg) {
			try {
				const pkg = JSON.parse(readFileSync(pkgPath, "utf8")) as {
					semio?: { app?: PlaygroundAppManifest; playgroundApp?: PlaygroundAppManifest };
				};
				const manifest = pkg.semio?.app ?? pkg.semio?.playgroundApp;
				if (manifest?.kind && manifest.packageRoot && manifest.corePackage && manifest.definitionExport) {
					entries.push({ ...manifest, corePackageJsonPath: pkgPath });
				}
			} catch {
				/* ignore malformed package.json */
			}
		}
		for (const entry of readdirSync(dir)) {
			if (PLAYGROUND_MANIFEST_SKIP_DIRS.has(entry)) continue;
			const full = join(dir, entry);
			if (statSync(full).isDirectory()) walk(full);
		}
	};
	walk(repoRoot);
	return entries.sort((a, b) => a.kind.localeCompare(b.kind));
}

/** @emoji 🗺 kind -> manifest entry */
export function playgroundAppManifestByKind(
	manifests: readonly PlaygroundAppManifestEntry[],
): ReadonlyMap<string, PlaygroundAppManifestEntry> {
	const map = new Map<string, PlaygroundAppManifestEntry>();
	for (const entry of manifests) {
		map.set(entry.kind, entry);
		for (const alias of entry.aliases ?? []) map.set(alias, entry);
	}
	return map;
}

/** @emoji 🧭 CLI segment -> app kind (derived from manifest aliases). */
export function resolvePlaygroundDevAppFromManifests(
	segments: string[],
	manifests: readonly PlaygroundAppManifestEntry[],
): { readonly app: string; readonly rest: string[] } | null {
	const byAlias = playgroundAppManifestByKind(manifests);
	if (segments.length === 0) return null;
	for (let len = Math.min(3, segments.length); len >= 1; len -= 1) {
		const key = segments.slice(0, len).join(" ");
		const entry = byAlias.get(key);
		if (entry) return { app: entry.kind, rest: segments.slice(len) };
	}
	return null;
}

/** @emoji 📂 Core entry path for a manifest's {@link PlaygroundAppManifest.corePackage}. */
export function playgroundManifestCoreEntry(manifest: PlaygroundAppManifestEntry): string {
	const packageDir = dirname(manifest.corePackageJsonPath);
	const nestedCore = resolve(packageDir, "js/index.ts");
	if (existsSync(nestedCore)) return nestedCore;
	return resolve(packageDir, "index.ts");
}

/** @emoji 📂 Optional internal entry path for a manifest core package. */
export function playgroundManifestInternalEntry(manifest: PlaygroundAppManifestEntry): string | undefined {
	const packageDir = dirname(manifest.corePackageJsonPath);
	for (const candidate of [resolve(packageDir, "js/internal.ts"), resolve(packageDir, "internal.ts")]) {
		if (existsSync(candidate)) return candidate;
	}
	return undefined;
}

/** @emoji 🔗 Vite resolve aliases for a manifest core package and optional `/internal` export. */
export function playgroundManifestCoreAliases(
	manifest: PlaygroundAppManifestEntry,
): ReadonlyArray<{ readonly find: string; readonly replacement: string }> {
	const coreEntry = playgroundManifestCoreEntry(manifest);
	const internalEntry = playgroundManifestInternalEntry(manifest);
	const aliases: Array<{ find: string; replacement: string }> = [];
	if (internalEntry) {
		aliases.push({ find: `${manifest.corePackage}/internal`, replacement: internalEntry });
	}
	aliases.push({ find: manifest.corePackage, replacement: coreEntry });
	return aliases;
}

/** @emoji 🎯 App manifests included in a scoped play virtual module. */
export function filterPlaygroundAppManifestsForActiveKind(
	manifests: readonly PlaygroundAppManifestEntry[],
	activeKind?: string,
): readonly PlaygroundAppManifestEntry[] {
	if (!activeKind) return manifests;
	const host = manifests.find((manifest) => manifest.kind === activeKind || (manifest.aliases ?? []).includes(activeKind));
	const kinds = new Set<string>([activeKind, ...(host?.programContributionKinds ?? [])]);
	return manifests.filter(
		(manifest) => kinds.has(manifest.kind) || (manifest.aliases ?? []).some((alias) => kinds.has(alias)),
	);
}

/** @emoji 🧩 Program contribution manifests included in a scoped play virtual module. */
export function filterPlaygroundProgramManifestsForActiveKind(
	manifests: readonly PlaygroundAppManifestEntry[],
	activeKind?: string,
): readonly PlaygroundAppManifestEntry[] {
	const withProgram = manifests.filter((manifest) => manifest.programExport);
	if (!activeKind) return withProgram;
	const host = manifests.find((manifest) => manifest.kind === activeKind || (manifest.aliases ?? []).includes(activeKind));
	if (host?.osProgramContributions) return withProgram;
	const contributionKinds = host?.programContributionKinds;
	if (contributionKinds?.length) {
		const kinds = new Set(contributionKinds);
		return withProgram.filter(
			(manifest) => kinds.has(manifest.kind) || (manifest.aliases ?? []).some((alias) => kinds.has(alias)),
		);
	}
	return withProgram.filter((manifest) => manifest.kind === activeKind || (manifest.aliases ?? []).includes(activeKind));
}
//#endregion 🔖PlaygroundAppManifest
