// #region 🧾NakaginSceneBake
/** @emoji 🧾 Fixture bake entrypoint: merges external layout JSON → `elements.scene.fixture/v1` with three.js-only `origin` / `orientation` (see `authoringToThreeFixture.ts`). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { authoringPlaneToThreeFixture } from "./authoringToThreeFixture.ts";
import {
	mergeAuthoringPlanesFromFlatLayoutPlanesV1Doc,
	mergeAuthoringPlanesFromFlatPlanesV1Doc,
	type SemioAuthoringPlane,
} from "./semioDesignPlane.ts";

function iconKindToMeshUrl(icon: string): string {
	return `/meshes/${icon}.glb`;
}

function bodyRadius(node: Record<string, unknown>): number {
	if (typeof node.radius === "number") return node.radius;
	const w = typeof node.width === "number" ? node.width : 40;
	const h = typeof node.height === "number" ? node.height : 40;
	return Math.max(w, h) * 0.5;
}

export interface BakeNakaginCapsuleTowerSceneFixtureOptions {
	readonly repoRoot: string;
	readonly boardRelativePath?: string;
	readonly sceneOutRelativePath?: string;
	/** @emoji 🧾 Defaults to `elements/client/lib/scene/fixtures/nakagin-flat-layout.planes.v1.json` (Flat design piece `name` → plane). */
	readonly flatLayoutPlanesRelativePath?: string;
	/** @emoji 🧾 Optional `elements.scene.flat-planes/v1` keyed by **board piece id** (overrides layout plane when present). */
	readonly flatPlanesV1RelativePath?: string;
}

export type { SemioAuthoringPlane };

/** @emoji 🧾 Rebuilds nakagin scene fixture: three.js origin/orientation from Flat layout **plane only** (match board `label` to piece `name`); never uses board x/y for placement. */
export function bakeNakaginCapsuleTowerSceneFixture(opts: BakeNakaginCapsuleTowerSceneFixtureOptions): void {
	const repoRoot = opts.repoRoot;
	const boardPath = join(repoRoot, opts.boardRelativePath ?? ".storybook/fixtures/nakagin-capsule-tower.board.json");
	const outPath = join(repoRoot, opts.sceneOutRelativePath ?? "elements/client/lib/scene/fixtures/nakagin-capsule-tower.scene.json");
	const layoutPath = join(
		repoRoot,
		opts.flatLayoutPlanesRelativePath ?? "elements/client/lib/scene/fixtures/nakagin-flat-layout.planes.v1.json",
	);

	const planeByLabel = new Map<string, SemioAuthoringPlane>();
	const layoutDoc = JSON.parse(readFileSync(layoutPath, "utf8")) as unknown;
	mergeAuthoringPlanesFromFlatLayoutPlanesV1Doc(layoutDoc, planeByLabel);

	const planeBySourceId = new Map<string, SemioAuthoringPlane>();
	if (opts.flatPlanesV1RelativePath) {
		const raw = JSON.parse(readFileSync(join(repoRoot, opts.flatPlanesV1RelativePath), "utf8")) as unknown;
		mergeAuthoringPlanesFromFlatPlanesV1Doc(raw, planeBySourceId);
	}

	const board = JSON.parse(readFileSync(boardPath, "utf8")) as Record<string, unknown>;
	const nodes = board.nodes as Record<string, unknown>[];
	const edges = board.edges as Record<string, unknown>[];
	const meta = board.meta;

	const objects = nodes.map((node) => {
		const id = String(node.id);
		const label = typeof node.label === "string" ? node.label : id;
		const authoringPlane = planeBySourceId.get(id) ?? planeByLabel.get(label);
		if (!authoringPlane) {
			throw new Error(`[bake nakagin scene] missing flat-layout plane for board node ${id} label "${label}"`);
		}
		const { origin, orientation } = authoringPlaneToThreeFixture(authoringPlane);
		const iconKind = String(node.iconKind ?? "placeholder");
		const br = bodyRadius(node);
		const handles = (node.handles as Record<string, unknown>[] | undefined) ?? [];
		const vortices = handles.map((h) => {
			const angle = Number(h.angle ?? 0);
			const lx = Math.cos(angle) * br;
			const lz = Math.sin(angle) * br;
			return {
				id: String(h.id),
				vortexKind: typeof h.handleKind === "string" ? h.handleKind : undefined,
				position: [lx, 0.4, lz] as [number, number, number],
				...(typeof h.radius === "number" ? { radius: h.radius * 0.12 } : {}),
			};
		});
		return {
			id,
			label,
			objectKind: typeof node.nodeKind === "string" ? node.nodeKind : undefined,
			meshUrl: iconKindToMeshUrl(iconKind),
			origin,
			orientation,
			vortices,
		};
	});

	const ties = edges.map((e) => ({
		id: String(e.id),
		source: String(e.source),
		target: String(e.target),
	}));

	const scene = {
		schema: "elements.scene.fixture/v1",
		camera: {
			position: [420, 320, 420],
			target: [0, 40, 0],
			zoom: 1,
		},
		...(meta && typeof meta === "object" ? { meta } : {}),
		ties,
		objects,
	};

	writeFileSync(outPath, JSON.stringify(scene, null, 2));
	console.log(
		`[bake nakagin scene] wrote ${outPath} (${objects.length} objects, ${ties.length} ties, ${planeByLabel.size} layout planes, ${planeBySourceId.size} id overrides)`,
	);
}
// #endregion 🧾NakaginSceneBake
