// #region 🧲Header
/** @emoji 🗂️ `@semio-tech/framework-playground-core` — lazy registry of playground app definitions keyed by dev host entry kind. */
// #endregion 🧲Header

import type { PlaygroundAppDefinition } from "./index.ts";

//#region 🔖Registry
const PLAY_ENTRY_KIND =
	typeof import.meta.env !== "undefined" && typeof import.meta.env.PUZZLE_PLAY_ENTRY === "string"
		? import.meta.env.PUZZLE_PLAY_ENTRY
		: "";

const ALL_PLAY_ENTRY_KINDS = [
	"2d",
	"3d",
	"5d",
	"flow",
	"dag",
	"imperative",
	"sequence",
	"layout",
	"lowpoly",
	"procedural-2d",
	"procedural-3d",
	"shooting",
	"forms",
	"raster",
	"draw",
	"note",
	"writer",
	"vcs",
	"gis-2d",
	"wires",
	"trinity-jack",
	"trinity-rewrite",
	"presentation",
	"cad",
] as const;

/** @emoji 🗂️ Loaded playground apps keyed by {@link AppDevHostConfig.playEntryKind}. */
export const PLAYGROUND_APP_REGISTRY = new Map<string, PlaygroundAppDefinition>();

function registerPlaygroundApp(app: PlaygroundAppDefinition): void {
	const kind = app.devHost?.playEntryKind;
	if (!kind) throw new Error(`Playground app "${app.id}" is missing devHost.playEntryKind`);
	PLAYGROUND_APP_REGISTRY.set(kind, app);
}

/** @emoji 📦 Imports one playground app; unreachable branches are dropped per {@link PLAY_ENTRY_KIND}. */
async function importPlaygroundAppDefinition(kind: string): Promise<PlaygroundAppDefinition> {
	if (kind === "2d") return (await import("@semio-tech/puzzle-2d-core")).puzzle2dPlayAppDefinition;
	if (kind === "3d") return (await import("@semio-tech/puzzle-3d-core")).puzzle3dPlayAppDefinition;
	if (kind === "5d") return (await import("@semio-tech/puzzle-5d-core")).puzzle5dPlayAppDefinition;
	if (kind === "flow") return (await import("@semio-tech/flow-core")).flowPlayAppDefinition;
	if (kind === "dag") return (await import("@semio-tech/dag-host-core")).dagPlayAppDefinition;
	if (kind === "imperative") return (await import("@semio-tech/imperative-core")).imperativePlayAppDefinition;
	if (kind === "sequence") return (await import("@semio-tech/sequence-core")).sequencePlayAppDefinition;
	if (kind === "layout") return (await import("@semio-tech/layout-core")).layoutPlayAppDefinition;
	if (kind === "lowpoly") return (await import("@semio-tech/lowpoly-core")).lowpolyPlayAppDefinition;
	if (kind === "procedural-2d") return (await import("@semio-tech/procedural-2d-core")).procedural2dPlayAppDefinition;
	if (kind === "procedural-3d") return (await import("@semio-tech/procedural-3d-core")).procedural3dPlayAppDefinition;
	if (kind === "shooting") return (await import("@semio-tech/shooting-core")).shootingPlayAppDefinition;
	if (kind === "forms") return (await import("@semio-tech/forms-core")).formsPlayAppDefinition;
	if (kind === "raster") return (await import("@semio-tech/raster-core")).rasterPlayAppDefinition;
	if (kind === "draw") return (await import("@semio-tech/draw-core")).drawPlayAppDefinition;
	if (kind === "note") return (await import("@semio-tech/note-core")).notePlayAppDefinition;
	if (kind === "writer") return (await import("@semio-tech/writer-core")).writerPlayAppDefinition;
	if (kind === "vcs") return (await import("@semio-tech/vcs-core")).vcsPlayAppDefinition;
	if (kind === "gis-2d") return (await import("@semio-tech/gis-2d-core")).gis2dPlayAppDefinition;
	if (kind === "wires") return (await import("@semio-tech/reasoning-mindmap-wires-core")).wiresPlayAppDefinition;
	if (kind === "trinity-jack") return (await import("@semio-tech/trinity-jack-host-core")).trinityJackPlayAppDefinition;
	if (kind === "trinity-rewrite") return (await import("@semio-tech/trinity-rewrite-core")).trinityRewritePlayAppDefinition;
	if (kind === "presentation") return (await import("@semio-tech/framework-presentation-core")).presentationPlayAppDefinition;
	if (kind === "cad") return (await import("@semio-tech/cad-js-renderer-core")).cadPlayAppDefinition;
	throw new Error(`unknown playground app: ${kind}`);
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
