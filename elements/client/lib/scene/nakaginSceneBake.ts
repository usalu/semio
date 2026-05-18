// #region 🧾NakaginSceneBake
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { planeBasisToThreeJs } from "./coordsPlane.ts";
import {
	mergeAuthoringPlanesFromDesignDoc,
	mergeAuthoringPlanesFromFlatPlanesV1Doc,
	paperAuthoringPlaneAtBoard,
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

const nakaginDesignFixtureBasenames = [
	"nakagin-capsule-tower.shallow.design.semio.json",
	"nakagin-capsule-tower.with-diff.design.semio.json",
	"nakagin-capsule-tower.copy.design.semio.json",
	"nakagin-capsule-tower.paste.design.semio.json",
	"nakagin-capsule-tower.paste.with-coordinate.design.diff.semio.json",
	"nakagin-capsule-tower.paste.design.diff.semio.json",
	"nakagin-capsule-tower.deleted.design.diff.semio.json",
] as const;

export interface BakeNakaginCapsuleTowerSceneFixtureOptions {
	readonly repoRoot: string;
	readonly boardRelativePath?: string;
	readonly sceneOutRelativePath?: string;
	readonly extraDesignRelativePaths?: readonly string[];
	readonly flatPlanesV1RelativePath?: string;
}

export type { SemioAuthoringPlane };

/** @emoji 🧾 Rebuilds `nakagin-capsule-tower.scene.json` from board + semio design planes (id-matched); falls back to paper plane at board (x,y). */
export function bakeNakaginCapsuleTowerSceneFixture(opts: BakeNakaginCapsuleTowerSceneFixtureOptions): void {
	const repoRoot = opts.repoRoot;
	const boardPath = join(repoRoot, opts.boardRelativePath ?? ".storybook/fixtures/nakagin-capsule-tower.board.json");
	const outPath = join(repoRoot, opts.sceneOutRelativePath ?? "elements/client/lib/scene/fixtures/nakagin-capsule-tower.scene.json");
	const semioFixtures = join(repoRoot, "semio/assets/fixtures");

	const planeById = new Map<string, SemioAuthoringPlane>();
	for (const base of nakaginDesignFixtureBasenames) {
		const p = join(semioFixtures, base);
		try {
			const doc = JSON.parse(readFileSync(p, "utf8")) as Record<string, unknown>;
			mergeAuthoringPlanesFromDesignDoc(doc, planeById);
		} catch {
			/* optional fixtures */
		}
	}
	for (const rel of opts.extraDesignRelativePaths ?? []) {
		const doc = JSON.parse(readFileSync(join(repoRoot, rel), "utf8")) as Record<string, unknown>;
		mergeAuthoringPlanesFromDesignDoc(doc, planeById);
	}
	if (opts.flatPlanesV1RelativePath) {
		try {
			const raw = JSON.parse(readFileSync(join(repoRoot, opts.flatPlanesV1RelativePath), "utf8")) as unknown;
			mergeAuthoringPlanesFromFlatPlanesV1Doc(raw, planeById);
		} catch {
			/* optional */
		}
	}

	const board = JSON.parse(readFileSync(boardPath, "utf8")) as Record<string, unknown>;
	const nodes = board.nodes as Record<string, unknown>[];
	const edges = board.edges as Record<string, unknown>[];
	const meta = board.meta;

	const objects = nodes.map((node) => {
		const id = String(node.id);
		const x = Number(node.x);
		const y = Number(node.y);
		const authoringPlane = planeById.get(id) ?? paperAuthoringPlaneAtBoard(x, y);
		const { origin, orientation } = planeBasisToThreeJs(authoringPlane);
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
			label: typeof node.label === "string" ? node.label : id,
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
	console.log(`[bake nakagin scene] wrote ${outPath} (${objects.length} objects, ${ties.length} ties, ${planeById.size} design planes merged)`);
}
// #endregion 🧾NakaginSceneBake
