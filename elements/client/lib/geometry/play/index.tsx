// #region 🧲Header
// 💻 elements/client/lib/geometry/play/index.tsx — Geometry play harness: Topologic all-kinds selector, single-window UI shell, and transform gumball editing for every entity kind.
// #endregion 🧲Header

import { App, LevelProvider, createDefaultLayout, getLevelBgClass, useApp, type AppConfig, type UIToolbarItem } from "@elements/ui";
import { BoxSelect, Move3d, Rotate3d, Scaling } from "lucide-react";
import { act, createContext, useContext, useEffect, useMemo, useState, type ReactElement } from "react";
import { createRoot } from "react-dom/client";

import topologyJson from "../fixtures/topology.json";
import { TopologicViewport, type TopologicTransformMode } from "../react/index.tsx";
import {
	TOPOLOGIC_KINDS,
	TopologicWasmSession,
	collectContainerPickPoints,
	ensureTopologicWasmLoaded,
	loadTopologicFixtureV1,
	topologicEntityLabel,
	updateTopologicFixtureTransform,
	type TopologicEntity,
	type TopologicFixtureV1,
	type TopologicKind,
	type TopologicStyle,
	type TopologicTopologyEntity,
	type TopologicTransform,
	type Vec3,
} from "../wasm/index.ts";
import "./globals.css";

//#region 🔖Ids
const GEOMETRY_PLAY_APP_ID = "elements-geometry-play";
const GEOMETRY_PLAY_WINDOW_ID = "geometry-topologic-window";
const GEOMETRY_PLAY_WINDOW_LABEL = "Topologic Playground";
const GEOMETRY_PLAY_DEFAULT_LAYOUT = createDefaultLayout([GEOMETRY_PLAY_WINDOW_ID], "row", [100], [GEOMETRY_PLAY_WINDOW_LABEL]);
const GEOMETRY_PLAY_TRANSFORM_MODES = ["translate", "rotate", "scale"] as const satisfies readonly TopologicTransformMode[];
const GEOMETRY_PLAY_MODES = ["edit", "analyze"] as const;
const GEOMETRY_PLAY_TRANSFORM_ICONS: Record<TopologicTransformMode, ReactElement> = {
	translate: <Move3d className="size-4" aria-hidden />,
	rotate: <Rotate3d className="size-4" aria-hidden />,
	scale: <Scaling className="size-4" aria-hidden />,
};
//#endregion 🔖Ids

//#region 🔖AnalyzeKinds
type GeometryPlayMode = (typeof GEOMETRY_PLAY_MODES)[number];
type AnalyzeSurfaceExposure = "external" | "internal";
type AnalyzeSurfaceStance = "horizontal" | "vertical";
type AnalyzePartOverlap = "none" | "difference" | "intersection";
type AnalyzeKind =
	| `surface.${AnalyzeSurfaceExposure}.${AnalyzeSurfaceStance}`
	| `part.${AnalyzePartOverlap}`
	| "solid";

const ANALYZE_KINDS = [
	"surface.external.horizontal",
	"surface.external.vertical",
	"surface.internal.horizontal",
	"surface.internal.vertical",
	"part.none",
	"part.difference",
	"part.intersection",
	"solid",
] as const satisfies readonly AnalyzeKind[];

const ANALYZE_SURFACE_KINDS = ANALYZE_KINDS.filter((kind) => kind.startsWith("surface.")) as readonly AnalyzeKind[];
const ANALYZE_PART_KINDS = ANALYZE_KINDS.filter((kind) => kind.startsWith("part.")) as readonly AnalyzeKind[];
const ANALYZE_KIND_SET = new Set<string>(ANALYZE_KINDS);

const ANALYZE_STYLE_BY_KIND: Record<AnalyzeKind, TopologicStyle> = {
	"surface.external.horizontal": { color: "#38bdf8", edgeColor: "#7dd3fc", opacity: 0.4 },
	"surface.external.vertical": { color: "#60a5fa", edgeColor: "#93c5fd", opacity: 0.28 },
	"surface.internal.horizontal": { color: "#f59e0b", edgeColor: "#fbbf24", opacity: 0.42 },
	"surface.internal.vertical": { color: "#f97316", edgeColor: "#fdba74", opacity: 0.34 },
	"part.none": { color: "#22c55e", edgeColor: "#4ade80", opacity: 0.22 },
	"part.difference": { color: "#eab308", edgeColor: "#facc15", opacity: 0.24 },
	"part.intersection": { color: "#ef4444", edgeColor: "#f87171", opacity: 0.34 },
	solid: { color: "#a855f7", edgeColor: "#d8b4fe", opacity: 0.14 },
};
//#endregion 🔖AnalyzeKinds

//#region 🔖AnalyzeDerivation
type Axis = "x" | "y" | "z";

interface Bounds {
	readonly min: Vec3;
	readonly max: Vec3;
}

interface CellBoundsInfo {
	readonly cellId: string;
	readonly label: string;
	readonly bounds: Bounds;
	readonly overlaps: boolean;
}

interface VoxelCell {
	readonly xIndex: number;
	readonly yIndex: number;
	readonly zIndex: number;
	readonly bounds: Bounds;
	readonly ownerIds: readonly string[];
	readonly ownerKey: string;
	readonly overlap: AnalyzePartOverlap;
}

interface GridPartition {
	readonly xs: readonly number[];
	readonly ys: readonly number[];
	readonly zs: readonly number[];
	readonly voxels: readonly VoxelCell[];
	readonly voxelByKey: ReadonlyMap<string, VoxelCell>;
}

interface VoxelComponent {
	readonly componentId: string;
	readonly ownerIds: readonly string[];
	readonly overlap: AnalyzePartOverlap;
	readonly voxelKeys: ReadonlySet<string>;
	readonly voxels: readonly VoxelCell[];
}

interface MergedGridRect<T extends string> {
	readonly tag: T;
	readonly uStart: number;
	readonly uEnd: number;
	readonly vStart: number;
	readonly vEnd: number;
}

function createBounds(points: readonly Vec3[]): Bounds | null {
	if (points.length === 0) return null;
	let minX = Infinity;
	let minY = Infinity;
	let minZ = Infinity;
	let maxX = -Infinity;
	let maxY = -Infinity;
	let maxZ = -Infinity;
	for (const [x, y, z] of points) {
		minX = Math.min(minX, x);
		minY = Math.min(minY, y);
		minZ = Math.min(minZ, z);
		maxX = Math.max(maxX, x);
		maxY = Math.max(maxY, y);
		maxZ = Math.max(maxZ, z);
	}
	return { min: [minX, minY, minZ], max: [maxX, maxY, maxZ] };
}

function boundsCenter(bounds: Bounds): Vec3 {
	return [
		(bounds.min[0] + bounds.max[0]) / 2,
		(bounds.min[1] + bounds.max[1]) / 2,
		(bounds.min[2] + bounds.max[2]) / 2,
	];
}

function boundsContainPoint(bounds: Bounds, point: Vec3, epsilon = 1e-9): boolean {
	return (
		point[0] >= bounds.min[0] - epsilon &&
		point[0] <= bounds.max[0] + epsilon &&
		point[1] >= bounds.min[1] - epsilon &&
		point[1] <= bounds.max[1] + epsilon &&
		point[2] >= bounds.min[2] - epsilon &&
		point[2] <= bounds.max[2] + epsilon
	);
}

function boundsOverlap(left: Bounds, right: Bounds, epsilon = 1e-9): boolean {
	return (
		Math.min(left.max[0], right.max[0]) - Math.max(left.min[0], right.min[0]) > epsilon &&
		Math.min(left.max[1], right.max[1]) - Math.max(left.min[1], right.min[1]) > epsilon &&
		Math.min(left.max[2], right.max[2]) - Math.max(left.min[2], right.min[2]) > epsilon
	);
}

function uniqueSorted(values: readonly number[]): number[] {
	return [...new Set(values)].sort((left, right) => left - right);
}

function intervalCenter(start: number, end: number): number {
	return (start + end) / 2;
}

function voxelKey(xIndex: number, yIndex: number, zIndex: number): string {
	return `${xIndex}:${yIndex}:${zIndex}`;
}

function collectCellBounds(session: TopologicWasmSession): readonly CellBoundsInfo[] {
	const cells = session.listByKind("cell");
	const boundsByCell = cells
		.map((cell) => {
			const points = collectContainerPickPoints(session, cell.id);
			const bounds = createBounds(points);
			if (!bounds) return null;
			return { cellId: cell.id, label: topologicEntityLabel(cell), bounds };
		})
		.filter((entry): entry is { cellId: string; label: string; bounds: Bounds } => Boolean(entry));
	const overlappingCellIds = new Set<string>();
	for (let leftIndex = 0; leftIndex < boundsByCell.length; leftIndex += 1) {
		for (let rightIndex = leftIndex + 1; rightIndex < boundsByCell.length; rightIndex += 1) {
			if (!boundsOverlap(boundsByCell[leftIndex]!.bounds, boundsByCell[rightIndex]!.bounds)) continue;
			overlappingCellIds.add(boundsByCell[leftIndex]!.cellId);
			overlappingCellIds.add(boundsByCell[rightIndex]!.cellId);
		}
	}
	return boundsByCell.map((entry) => ({ ...entry, overlaps: overlappingCellIds.has(entry.cellId) }));
}

function createGridPartition(cells: readonly CellBoundsInfo[]): GridPartition {
	const xs = uniqueSorted(cells.flatMap((cell) => [cell.bounds.min[0], cell.bounds.max[0]]));
	const ys = uniqueSorted(cells.flatMap((cell) => [cell.bounds.min[1], cell.bounds.max[1]]));
	const zs = uniqueSorted(cells.flatMap((cell) => [cell.bounds.min[2], cell.bounds.max[2]]));
	const voxels: VoxelCell[] = [];
	for (let xIndex = 0; xIndex < xs.length - 1; xIndex += 1) {
		for (let yIndex = 0; yIndex < ys.length - 1; yIndex += 1) {
			for (let zIndex = 0; zIndex < zs.length - 1; zIndex += 1) {
				const bounds: Bounds = {
					min: [xs[xIndex]!, ys[yIndex]!, zs[zIndex]!],
					max: [xs[xIndex + 1]!, ys[yIndex + 1]!, zs[zIndex + 1]!],
				};
				const center: Vec3 = [
					intervalCenter(bounds.min[0], bounds.max[0]),
					intervalCenter(bounds.min[1], bounds.max[1]),
					intervalCenter(bounds.min[2], bounds.max[2]),
				];
				const owners = cells.filter((cell) => boundsContainPoint(cell.bounds, center)).map((cell) => cell.cellId);
				if (owners.length === 0) continue;
				const ownerKey = owners.join("|");
				const overlap = owners.length > 1 ? "intersection" : cells.find((cell) => cell.cellId === owners[0])?.overlaps ? "difference" : "none";
				voxels.push({ xIndex, yIndex, zIndex, bounds, ownerIds: owners, ownerKey, overlap });
			}
		}
	}
	return {
		xs,
		ys,
		zs,
		voxels,
		voxelByKey: new Map(voxels.map((voxel) => [voxelKey(voxel.xIndex, voxel.yIndex, voxel.zIndex), voxel])),
	};
}

function collectVoxelComponents(partition: GridPartition): readonly VoxelComponent[] {
	const components: VoxelComponent[] = [];
	const visited = new Set<string>();
	for (const seed of partition.voxels) {
		const seedKey = voxelKey(seed.xIndex, seed.yIndex, seed.zIndex);
		if (visited.has(seedKey)) continue;
		const stack = [seed];
		const componentVoxels: VoxelCell[] = [];
		const componentKeys = new Set<string>();
		visited.add(seedKey);
		while (stack.length > 0) {
			const current = stack.pop()!;
			const currentKey = voxelKey(current.xIndex, current.yIndex, current.zIndex);
			componentKeys.add(currentKey);
			componentVoxels.push(current);
			const neighbors = [
				[current.xIndex - 1, current.yIndex, current.zIndex],
				[current.xIndex + 1, current.yIndex, current.zIndex],
				[current.xIndex, current.yIndex - 1, current.zIndex],
				[current.xIndex, current.yIndex + 1, current.zIndex],
				[current.xIndex, current.yIndex, current.zIndex - 1],
				[current.xIndex, current.yIndex, current.zIndex + 1],
			] as const;
			for (const [xIndex, yIndex, zIndex] of neighbors) {
				const next = partition.voxelByKey.get(voxelKey(xIndex, yIndex, zIndex));
				if (!next || next.ownerKey !== seed.ownerKey || next.overlap !== seed.overlap) continue;
				const nextKey = voxelKey(next.xIndex, next.yIndex, next.zIndex);
				if (visited.has(nextKey)) continue;
				visited.add(nextKey);
				stack.push(next);
			}
		}
		components.push({
			componentId: `analyze.part.${components.length + 1}`,
			ownerIds: seed.ownerIds,
			overlap: seed.overlap,
			voxelKeys: componentKeys,
			voxels: componentVoxels,
		});
	}
	return components;
}

function mergeTaggedGrid<T extends string>(cells: readonly (readonly (T | null)[])[]): readonly MergedGridRect<T>[] {
	const width = cells.length;
	const height = cells[0]?.length ?? 0;
	const visited = new Set<string>();
	const rects: MergedGridRect<T>[] = [];
	for (let uIndex = 0; uIndex < width; uIndex += 1) {
		for (let vIndex = 0; vIndex < height; vIndex += 1) {
			const tag = cells[uIndex]?.[vIndex] ?? null;
			if (!tag) continue;
			const key = `${uIndex}:${vIndex}`;
			if (visited.has(key)) continue;
			let uEnd = uIndex + 1;
			while (uEnd < width && cells[uEnd]?.[vIndex] === tag && !visited.has(`${uEnd}:${vIndex}`)) uEnd += 1;
			let vEnd = vIndex + 1;
			while (vEnd < height) {
				let matches = true;
				for (let scan = uIndex; scan < uEnd; scan += 1) {
					if (cells[scan]?.[vEnd] !== tag || visited.has(`${scan}:${vEnd}`)) {
						matches = false;
						break;
					}
				}
				if (!matches) break;
				vEnd += 1;
			}
			for (let markU = uIndex; markU < uEnd; markU += 1) {
				for (let markV = vIndex; markV < vEnd; markV += 1) {
					visited.add(`${markU}:${markV}`);
				}
			}
			rects.push({ tag, uStart: uIndex, uEnd, vStart: vIndex, vEnd });
		}
	}
	return rects;
}

function analyzeMetadata(kind: AnalyzeKind, selectable: boolean, parentId?: string): Record<string, unknown> {
	return {
		analyzeKind: kind,
		analyzeGroup: kind.startsWith("surface.") ? "surface" : kind.startsWith("part.") ? "part" : "solid",
		analyzeSelectable: selectable,
		...(parentId ? { analyzeParentId: parentId } : {}),
	};
}

function createRectangleFace(
	id: string,
	label: string,
	axis: Axis,
	plane: number,
	u0: number,
	u1: number,
	v0: number,
	v1: number,
	style: TopologicStyle,
	metadata: Record<string, unknown>,
): TopologicEntity {
	const vertices: readonly Vec3[] =
		axis === "x"
			? [[plane, u0, v0], [plane, u1, v0], [plane, u1, v1], [plane, u0, v1]]
			: axis === "y"
				? [[u0, plane, v0], [u1, plane, v0], [u1, plane, v1], [u0, plane, v1]]
				: [[u0, v0, plane], [u1, v0, plane], [u1, v1, plane], [u0, v1, plane]];
	return {
		id,
		kind: "face",
		label,
		wires: [],
		surface: { vertices, triangles: [0, 1, 2, 0, 2, 3] },
		style,
		metadata,
	};
}

function createBoxFaces(prefix: string, label: string, bounds: Bounds, kind: AnalyzeKind, selectable: boolean, parentId?: string): readonly TopologicEntity[] {
	const style = ANALYZE_STYLE_BY_KIND[kind];
	const metadata = analyzeMetadata(kind, selectable, parentId);
	return [
		createRectangleFace(`${prefix}.bottom`, `${label} Bottom`, "y", bounds.min[1], bounds.min[0], bounds.max[0], bounds.min[2], bounds.max[2], style, metadata),
		createRectangleFace(`${prefix}.top`, `${label} Top`, "y", bounds.max[1], bounds.min[0], bounds.max[0], bounds.min[2], bounds.max[2], style, metadata),
		createRectangleFace(`${prefix}.left`, `${label} Left`, "x", bounds.min[0], bounds.min[1], bounds.max[1], bounds.min[2], bounds.max[2], style, metadata),
		createRectangleFace(`${prefix}.right`, `${label} Right`, "x", bounds.max[0], bounds.min[1], bounds.max[1], bounds.min[2], bounds.max[2], style, metadata),
		createRectangleFace(`${prefix}.front`, `${label} Front`, "z", bounds.min[2], bounds.min[0], bounds.max[0], bounds.min[1], bounds.max[1], style, metadata),
		createRectangleFace(`${prefix}.back`, `${label} Back`, "z", bounds.max[2], bounds.min[0], bounds.max[0], bounds.min[1], bounds.max[1], style, metadata),
	];
}

function createComponentFaces(component: VoxelComponent, partition: GridPartition, label: string, kind: AnalyzeKind): readonly TopologicEntity[] {
	const faces: TopologicEntity[] = [];
	const style = ANALYZE_STYLE_BY_KIND[kind];
	const metadata = analyzeMetadata(kind, false, component.componentId);
	const makeGrid = (width: number, height: number, include: (uIndex: number, vIndex: number) => boolean): (boolean | null)[][] =>
		Array.from({ length: width }, (_, uIndex) => Array.from({ length: height }, (_, vIndex) => (include(uIndex, vIndex) ? "face" : null)));

	for (let planeIndex = 0; planeIndex < partition.xs.length; planeIndex += 1) {
		const merged = mergeTaggedGrid(
			makeGrid(partition.ys.length - 1, partition.zs.length - 1, (yIndex, zIndex) => {
				const left = planeIndex > 0 ? component.voxelKeys.has(voxelKey(planeIndex - 1, yIndex, zIndex)) : false;
				const right = planeIndex < partition.xs.length - 1 ? component.voxelKeys.has(voxelKey(planeIndex, yIndex, zIndex)) : false;
				return left !== right;
			}),
		);
		for (const rect of merged) {
			faces.push(
				createRectangleFace(
					`${component.componentId}.face.${faces.length + 1}`,
					`${label} Face ${faces.length + 1}`,
					"x",
					partition.xs[planeIndex]!,
					partition.ys[rect.uStart]!,
					partition.ys[rect.uEnd]!,
					partition.zs[rect.vStart]!,
					partition.zs[rect.vEnd]!,
					style,
					metadata,
				),
			);
		}
	}
	for (let planeIndex = 0; planeIndex < partition.ys.length; planeIndex += 1) {
		const merged = mergeTaggedGrid(
			makeGrid(partition.xs.length - 1, partition.zs.length - 1, (xIndex, zIndex) => {
				const bottom = planeIndex > 0 ? component.voxelKeys.has(voxelKey(xIndex, planeIndex - 1, zIndex)) : false;
				const top = planeIndex < partition.ys.length - 1 ? component.voxelKeys.has(voxelKey(xIndex, planeIndex, zIndex)) : false;
				return bottom !== top;
			}),
		);
		for (const rect of merged) {
			faces.push(
				createRectangleFace(
					`${component.componentId}.face.${faces.length + 1}`,
					`${label} Face ${faces.length + 1}`,
					"y",
					partition.ys[planeIndex]!,
					partition.xs[rect.uStart]!,
					partition.xs[rect.uEnd]!,
					partition.zs[rect.vStart]!,
					partition.zs[rect.vEnd]!,
					style,
					metadata,
				),
			);
		}
	}
	for (let planeIndex = 0; planeIndex < partition.zs.length; planeIndex += 1) {
		const merged = mergeTaggedGrid(
			makeGrid(partition.xs.length - 1, partition.ys.length - 1, (xIndex, yIndex) => {
				const front = planeIndex > 0 ? component.voxelKeys.has(voxelKey(xIndex, yIndex, planeIndex - 1)) : false;
				const back = planeIndex < partition.zs.length - 1 ? component.voxelKeys.has(voxelKey(xIndex, yIndex, planeIndex)) : false;
				return front !== back;
			}),
		);
		for (const rect of merged) {
			faces.push(
				createRectangleFace(
					`${component.componentId}.face.${faces.length + 1}`,
					`${label} Face ${faces.length + 1}`,
					"z",
					partition.zs[planeIndex]!,
					partition.xs[rect.uStart]!,
					partition.xs[rect.uEnd]!,
					partition.ys[rect.vStart]!,
					partition.ys[rect.vEnd]!,
					style,
					metadata,
				),
			);
		}
	}
	return faces;
}

function createAnalyzeFixture(fixture: TopologicFixtureV1): TopologicFixtureV1 {
	const session = new TopologicWasmSession(fixture);
	const cells = collectCellBounds(session);
	const partition = createGridPartition(cells);
	const components = collectVoxelComponents(partition);
	const topologies: TopologicEntity[] = [];
	const rootMembers: string[] = [];
	const partOverlapCounts: Record<AnalyzePartOverlap, number> = { none: 0, difference: 0, intersection: 0 };

	for (const [surfaceIndex, axis] of (["x", "y", "z"] as const).entries()) {
		const values = axis === "x" ? partition.xs : axis === "y" ? partition.ys : partition.zs;
		const uValues = axis === "x" ? partition.ys : partition.xs;
		const vValues = axis === "z" ? partition.ys : partition.zs;
		for (let planeIndex = 0; planeIndex < values.length; planeIndex += 1) {
			const grid =
				axis === "x"
					? Array.from({ length: partition.ys.length - 1 }, (_, yIndex) =>
						Array.from({ length: partition.zs.length - 1 }, (_, zIndex) => {
							const left = planeIndex > 0 ? partition.voxelByKey.get(voxelKey(planeIndex - 1, yIndex, zIndex)) : undefined;
							const right = planeIndex < partition.xs.length - 1 ? partition.voxelByKey.get(voxelKey(planeIndex, yIndex, zIndex)) : undefined;
							if ((left?.ownerKey ?? "") === (right?.ownerKey ?? "")) return null;
							return `${!left || !right ? "surface.external" : "surface.internal"}.${axis === "y" ? "horizontal" : "vertical"}` as AnalyzeKind;
						}),
					)
					: axis === "y"
						? Array.from({ length: partition.xs.length - 1 }, (_, xIndex) =>
							Array.from({ length: partition.zs.length - 1 }, (_, zIndex) => {
								const bottom = planeIndex > 0 ? partition.voxelByKey.get(voxelKey(xIndex, planeIndex - 1, zIndex)) : undefined;
								const top = planeIndex < partition.ys.length - 1 ? partition.voxelByKey.get(voxelKey(xIndex, planeIndex, zIndex)) : undefined;
								if ((bottom?.ownerKey ?? "") === (top?.ownerKey ?? "")) return null;
								return `${!bottom || !top ? "surface.external" : "surface.internal"}.horizontal` as AnalyzeKind;
							}),
						)
						: Array.from({ length: partition.xs.length - 1 }, (_, xIndex) =>
							Array.from({ length: partition.ys.length - 1 }, (_, yIndex) => {
								const front = planeIndex > 0 ? partition.voxelByKey.get(voxelKey(xIndex, yIndex, planeIndex - 1)) : undefined;
								const back = planeIndex < partition.zs.length - 1 ? partition.voxelByKey.get(voxelKey(xIndex, yIndex, planeIndex)) : undefined;
								if ((front?.ownerKey ?? "") === (back?.ownerKey ?? "")) return null;
								return `${!front || !back ? "surface.external" : "surface.internal"}.vertical` as AnalyzeKind;
							}),
						);
			const merged = mergeTaggedGrid(grid);
			for (const rect of merged) {
				const kind = rect.tag;
				const id = `analyze.surface.${topologies.length + surfaceIndex + 1}`;
				const label = `Surface ${analyzeKindLabel(kind)} ${rootMembers.filter((member) => member.startsWith("analyze.surface.")).length + 1}`;
				topologies.push(
					createRectangleFace(
						id,
						label,
						axis,
						values[planeIndex]!,
						uValues[rect.uStart]!,
						uValues[rect.uEnd]!,
						vValues[rect.vStart]!,
						vValues[rect.vEnd]!,
						ANALYZE_STYLE_BY_KIND[kind],
						analyzeMetadata(kind, true),
					),
				);
				rootMembers.push(id);
			}
		}
	}

	for (const cell of cells) {
		const solidId = `analyze.solid.${cell.cellId}`;
		const label = `Solid ${cell.label}`;
		const faces = createBoxFaces(`${solidId}.face`, label, cell.bounds, "solid", false, solidId);
		const solid: TopologicTopologyEntity = {
			id: solidId,
			kind: "topology",
			label,
			members: faces.map((face) => face.id),
			style: ANALYZE_STYLE_BY_KIND.solid,
			metadata: analyzeMetadata("solid", true),
		};
		topologies.push(solid, ...faces);
		rootMembers.push(solidId);
	}

	for (const component of components) {
		partOverlapCounts[component.overlap] += 1;
		const kind = `part.${component.overlap}` as AnalyzeKind;
		const label = `Part ${analyzeKindLabel(kind)} ${partOverlapCounts[component.overlap]}`;
		const faces = createComponentFaces(component, partition, label, kind);
		const part: TopologicTopologyEntity = {
			id: component.componentId,
			kind: "topology",
			label,
			members: faces.map((face) => face.id),
			style: ANALYZE_STYLE_BY_KIND[kind],
			metadata: analyzeMetadata(kind, true),
		};
		topologies.push(part, ...faces);
		rootMembers.push(part.id);
	}

	return {
		schema: fixture.schema,
		label: `${fixture.label ?? "Geometry"} Analyze`,
		roots: ["analyze-root"],
		topologies: [
			{
				id: "analyze-root",
				kind: "topology",
				label: "Analyze Root",
				members: rootMembers,
				metadata: { analyzeMode: "analyze" },
			},
			...topologies,
		],
	};
}
//#endregion 🔖AnalyzeDerivation

//#region 🔖Context
interface GeometryPlayValue {
	readonly fixture: TopologicFixtureV1;
	readonly session: TopologicWasmSession;
	readonly analyzeFixture: TopologicFixtureV1;
	readonly analyzeSession: TopologicWasmSession;
	readonly selectableKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly visibleKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly analyzeSelectableKinds: Readonly<Record<AnalyzeKind, boolean>>;
	readonly analyzeVisibleKinds: Readonly<Record<AnalyzeKind, boolean>>;
	readonly selectedId: string | null;
	readonly transformMode: TopologicTransformMode;
	readonly toggleSelectableKind: (kind: TopologicKind) => void;
	readonly toggleVisibleKind: (kind: TopologicKind) => void;
	readonly toggleAnalyzeSelectableKind: (kind: AnalyzeKind) => void;
	readonly toggleAnalyzeVisibleKind: (kind: AnalyzeKind) => void;
	readonly setSelectedId: (id: string | null) => void;
	readonly setTransformMode: (mode: TopologicTransformMode) => void;
	readonly onTransformCommit: (id: string, transform: TopologicTransform) => void;
}

const GeometryPlayContext = createContext<GeometryPlayValue | null>(null);

function useGeometryPlay(): GeometryPlayValue {
	const value = useContext(GeometryPlayContext);
	if (!value) throw new Error("GeometryPlayContext missing");
	return value;
}
//#endregion 🔖Context

//#region 🔖Controls
function geometryKindLabel(kind: TopologicKind): string {
	if (kind === "cellComplex") return "CellComplex";
	return kind.charAt(0).toUpperCase() + kind.slice(1);
}

function analyzeKindLabel(kind: AnalyzeKind): string {
	if (kind === "solid") return "Solid";
	if (kind.startsWith("surface.")) {
		const [, exposure, stance] = kind.split(".") as ["surface", AnalyzeSurfaceExposure, AnalyzeSurfaceStance];
		return `${exposure.charAt(0).toUpperCase() + exposure.slice(1)} ${stance.charAt(0).toUpperCase() + stance.slice(1)}`;
	}
	const [, overlap] = kind.split(".") as ["part", AnalyzePartOverlap];
	return overlap.charAt(0).toUpperCase() + overlap.slice(1);
}

function createAllKindsEnabled<TKind extends string>(order: readonly TKind[]): Record<TKind, boolean> {
	return Object.fromEntries(order.map((kind) => [kind, true])) as Record<TKind, boolean>;
}

function listEnabledKinds<TKind extends string>(order: readonly TKind[], kinds: Readonly<Record<TKind, boolean>>): TKind[] {
	return order.filter((kind) => kinds[kind]);
}

function formatEnabledKindsLabel(enabledKinds: readonly string[], totalCount: number): string {
	return enabledKinds.length === totalCount ? "all" : enabledKinds.join(",") || "none";
}

function geometryKindToolbarToggles<TKind extends string>(
	prefix: "selection" | "filter",
	order: readonly TKind[],
	labelForKind: (kind: TKind) => string,
	kinds: Record<TKind, boolean>,
	toggle: (kind: TKind) => void,
): UIToolbarItem[] {
	return order.map((kind, itemOrder) => ({
		id: `geometry.${prefix}.kind.${kind}`,
		kind: "toggle" as const,
		text: labelForKind(kind),
		onPressedChange: () => toggle(kind),
		order: itemOrder,
		pressed: kinds[kind],
	}));
}

function geometryAnalyzeToolbarToggles(
	prefix: "selection" | "filter",
	kinds: Record<AnalyzeKind, boolean>,
	toggle: (kind: AnalyzeKind) => void,
): UIToolbarItem[] {
	const items: UIToolbarItem[] = [
		...geometryKindToolbarToggles(prefix, ANALYZE_SURFACE_KINDS, analyzeKindLabel, kinds, toggle),
		{ id: `geometry.${prefix}.group.surface.separator`, kind: "separator", order: ANALYZE_SURFACE_KINDS.length },
		...geometryKindToolbarToggles(prefix, ANALYZE_PART_KINDS, analyzeKindLabel, kinds, toggle).map((item) => ({
			...item,
			order: (item.order ?? 0) + ANALYZE_SURFACE_KINDS.length + 1,
		})),
		{ id: `geometry.${prefix}.group.part.separator`, kind: "separator", order: ANALYZE_SURFACE_KINDS.length + ANALYZE_PART_KINDS.length + 1 },
		...geometryKindToolbarToggles(prefix, ["solid"], analyzeKindLabel, kinds, toggle).map((item) => ({
			...item,
			order: (item.order ?? 0) + ANALYZE_SURFACE_KINDS.length + ANALYZE_PART_KINDS.length + 2,
		})),
	];
	return items;
}

function entityAnalyzeKind(entity: TopologicEntity): AnalyzeKind | null {
	const kind = entity.metadata?.analyzeKind;
	return typeof kind === "string" && ANALYZE_KIND_SET.has(kind) ? (kind as AnalyzeKind) : null;
}

function isAnalyzeEntitySelectable(entity: TopologicEntity, selectableKinds: Readonly<Record<AnalyzeKind, boolean>>): boolean {
	const kind = entityAnalyzeKind(entity);
	if (!kind) return false;
	return entity.metadata?.analyzeSelectable === true && selectableKinds[kind];
}

function isAnalyzeEntityVisible(entity: TopologicEntity, visibleKinds: Readonly<Record<AnalyzeKind, boolean>>): boolean {
	const kind = entityAnalyzeKind(entity);
	return kind ? visibleKinds[kind] : false;
}

function listSelectableEntities(session: TopologicWasmSession, selectableKinds: Readonly<Record<TopologicKind, boolean>>) {
	return session.fixture.topologies.filter((entity) => selectableKinds[entity.kind]);
}

function listAnalyzeSelectableEntities(session: TopologicWasmSession, selectableKinds: Readonly<Record<AnalyzeKind, boolean>>) {
	return session.fixture.topologies.filter((entity) => isAnalyzeEntitySelectable(entity, selectableKinds));
}

function isSelectableEntity(
	session: TopologicWasmSession,
	selectableKinds: Readonly<Record<TopologicKind, boolean>>,
	id: string | null,
): boolean {
	if (!id) return false;
	const entity = session.getEntity(id);
	return Boolean(entity && selectableKinds[entity.kind]);
}

function isAnalyzeSelectableEntity(session: TopologicWasmSession, selectableKinds: Readonly<Record<AnalyzeKind, boolean>>, id: string | null): boolean {
	if (!id) return false;
	const entity = session.getEntity(id);
	return Boolean(entity && isAnalyzeEntitySelectable(entity, selectableKinds));
}

function geometryPlayModeFromApp(activeModeId: string | null): GeometryPlayMode {
	return activeModeId === "analyze" ? "analyze" : "edit";
}

function GeometryPlayWindow(): ReactElement {
	const play = useGeometryPlay();
	const { activeModeId } = useApp();
	const mode = geometryPlayModeFromApp(activeModeId);
	const activeSession = mode === "analyze" ? play.analyzeSession : play.session;
	const activeFixture = mode === "analyze" ? play.analyzeFixture : play.fixture;
	const activeSelectableEntities = mode === "analyze" ? listAnalyzeSelectableEntities(activeSession, play.analyzeSelectableKinds) : listSelectableEntities(activeSession, play.selectableKinds);
	const activeSelectedEntity = play.selectedId ? activeSession.getEntity(play.selectedId) : null;
	useEffect(() => {
		const selectedStillValid =
			mode === "analyze"
				? isAnalyzeSelectableEntity(activeSession, play.analyzeSelectableKinds, play.selectedId)
				: isSelectableEntity(activeSession, play.selectableKinds, play.selectedId);
		if (play.selectedId && !selectedStillValid) play.setSelectedId(null);
	}, [activeSession, mode, play, play.analyzeSelectableKinds, play.selectableKinds]);
	return (
		<div className="flex h-full w-full flex-col">
			<div className="flex shrink-0 gap-2 border-b border-border bg-muted/40 p-2">
				<span className="text-muted-foreground px-1 text-xs font-semibold uppercase tracking-wide" data-e2e-geometry-mode>
					{mode}
				</span>
				<span className="text-muted-foreground px-1 text-xs font-semibold uppercase tracking-wide" data-e2e-geometry-transform-mode>
					{mode === "edit" ? play.transformMode : "locked"}
				</span>
				<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-selection-kinds>
					{mode === "analyze"
						? formatEnabledKindsLabel(listEnabledKinds(ANALYZE_KINDS, play.analyzeSelectableKinds), ANALYZE_KINDS.length)
						: formatEnabledKindsLabel(listEnabledKinds(TOPOLOGIC_KINDS, play.selectableKinds), TOPOLOGIC_KINDS.length)}
				</span>
				<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-visible-kinds>
					{mode === "analyze"
						? formatEnabledKindsLabel(listEnabledKinds(ANALYZE_KINDS, play.analyzeVisibleKinds), ANALYZE_KINDS.length)
						: formatEnabledKindsLabel(listEnabledKinds(TOPOLOGIC_KINDS, play.visibleKinds), TOPOLOGIC_KINDS.length)}
				</span>
				<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-selection>{activeSelectedEntity ? topologicEntityLabel(activeSelectedEntity) : "—"}</span>
				<span className="text-muted-foreground px-1 text-xs">{activeSelectableEntities.length}</span>
			</div>
			<div className="relative min-h-0 flex-1">
				<TopologicViewport
					fixture={activeFixture}
					selectedId={play.selectedId}
					selectableKinds={mode === "edit" ? play.selectableKinds : undefined}
					visibleKinds={mode === "edit" ? play.visibleKinds : undefined}
					isEntitySelectable={mode === "analyze" ? (entity) => isAnalyzeEntitySelectable(entity, play.analyzeSelectableKinds) : undefined}
					isEntityVisible={mode === "analyze" ? (entity) => isAnalyzeEntityVisible(entity, play.analyzeVisibleKinds) : undefined}
					onSelect={play.setSelectedId}
					onTransformCommit={mode === "edit" ? play.onTransformCommit : undefined}
					transformMode={play.transformMode}
				/>
			</div>
		</div>
	);
}
//#endregion 🔖Controls

//#region 🔖Controller
function GeometryPlayController(): ReactElement {
	const [fixture, setFixture] = useState<TopologicFixtureV1 | null>(null);
	const [loadError, setLoadError] = useState<Error | null>(null);
	const session = useMemo(() => (fixture ? new TopologicWasmSession(fixture) : null), [fixture]);
	const analyzeFixture = useMemo(() => (fixture ? createAnalyzeFixture(fixture) : null), [fixture]);
	const analyzeSession = useMemo(() => (analyzeFixture ? new TopologicWasmSession(analyzeFixture) : null), [analyzeFixture]);
	const [selectableKinds, setSelectableKinds] = useState<Record<TopologicKind, boolean>>(() => createAllKindsEnabled(TOPOLOGIC_KINDS));
	const [visibleKinds, setVisibleKinds] = useState<Record<TopologicKind, boolean>>(() => createAllKindsEnabled(TOPOLOGIC_KINDS));
	const [analyzeSelectableKinds, setAnalyzeSelectableKinds] = useState<Record<AnalyzeKind, boolean>>(() => createAllKindsEnabled(ANALYZE_KINDS));
	const [analyzeVisibleKinds, setAnalyzeVisibleKinds] = useState<Record<AnalyzeKind, boolean>>(() => createAllKindsEnabled(ANALYZE_KINDS));
	const [selectedId, setSelectedId] = useState<string | null>(null);
	const [transformMode, setTransformMode] = useState<TopologicTransformMode>("translate");

	useEffect(() => {
		let cancelled = false;
		void ensureTopologicWasmLoaded()
			.then(async () => {
				const parsedFixture = await loadTopologicFixtureV1(topologyJson as unknown);
				if (!parsedFixture) throw new Error("geometry topology fixture failed to parse");
				if (!cancelled) setFixture(parsedFixture);
			})
			.catch((error) => {
				if (!cancelled) setLoadError(error instanceof Error ? error : new Error(String(error)));
			});
		return () => {
			cancelled = true;
		};
	}, []);

	useEffect(() => {
		if (!session) return;
		if (!isSelectableEntity(session, selectableKinds, selectedId)) {
			setSelectedId(null);
		}
	}, [selectedId, selectableKinds, session]);

	const value = useMemo<GeometryPlayValue | null>(
		() =>
			fixture && session && analyzeFixture && analyzeSession
				? {
					fixture,
					session,
					analyzeFixture,
					analyzeSession,
					selectableKinds,
					visibleKinds,
					analyzeSelectableKinds,
					analyzeVisibleKinds,
					selectedId,
					transformMode,
					toggleSelectableKind: (kind) => setSelectableKinds((current) => ({ ...current, [kind]: !current[kind] })),
					toggleVisibleKind: (kind) => setVisibleKinds((current) => ({ ...current, [kind]: !current[kind] })),
					toggleAnalyzeSelectableKind: (kind) => setAnalyzeSelectableKinds((current) => ({ ...current, [kind]: !current[kind] })),
					toggleAnalyzeVisibleKind: (kind) => setAnalyzeVisibleKinds((current) => ({ ...current, [kind]: !current[kind] })),
					setSelectedId: (id) => {
						if (!id || isSelectableEntity(session, selectableKinds, id) || isAnalyzeSelectableEntity(analyzeSession, analyzeSelectableKinds, id)) {
							setSelectedId(id);
						}
					},
					setTransformMode,
					onTransformCommit: (id, transform) =>
						setFixture((current) => (current ? updateTopologicFixtureTransform(current, id, transform) : current)),
				}
				: null,
		[analyzeFixture, analyzeSelectableKinds, analyzeSession, analyzeVisibleKinds, fixture, selectableKinds, selectedId, session, transformMode, visibleKinds],
	);

	const selectionKindOrderBase = TOPOLOGIC_KINDS.length;
	const apps = useMemo<AppConfig[]>(
		() => [
			{
				id: GEOMETRY_PLAY_APP_ID,
				label: "Geometry play",
				options: { selectableKinds, visibleKinds, analyzeSelectableKinds, analyzeVisibleKinds, transformMode },
				windowKinds: [{ id: GEOMETRY_PLAY_WINDOW_ID, label: GEOMETRY_PLAY_WINDOW_LABEL, component: GeometryPlayWindow }],
				defaultLayout: GEOMETRY_PLAY_DEFAULT_LAYOUT,
				defaultModeId: "edit",
				modes: [
					{
						id: "edit",
						label: "Edit",
						tools: {
							selection: [
								...geometryKindToolbarToggles("selection", TOPOLOGIC_KINDS, geometryKindLabel, selectableKinds, (kind) =>
									setSelectableKinds((current) => ({ ...current, [kind]: !current[kind] })),
								),
								{ id: "geometry.selection.separator.clear", kind: "separator" as const, order: selectionKindOrderBase },
								{
									id: "geometry.selection.clear",
									icon: <BoxSelect className="size-4" aria-hidden />,
									label: "Clear",
									onClick: () => setSelectedId(null),
									order: selectionKindOrderBase + 1,
								},
							],
							filter: geometryKindToolbarToggles("filter", TOPOLOGIC_KINDS, geometryKindLabel, visibleKinds, (kind) =>
								setVisibleKinds((current) => ({ ...current, [kind]: !current[kind] })),
							),
							actions: GEOMETRY_PLAY_TRANSFORM_MODES.map((mode, order) => ({
								id: `geometry.transform.${mode}`,
								kind: "toggle" as const,
								icon: GEOMETRY_PLAY_TRANSFORM_ICONS[mode],
								label: mode.charAt(0).toUpperCase() + mode.slice(1),
								onPressedChange: (pressed: boolean) => {
									if (pressed) setTransformMode(mode);
								},
								order,
								pressed: transformMode === mode,
							})),
						},
					},
					{
						id: "analyze",
						label: "Analyze",
						tools: {
							selection: [
								...geometryAnalyzeToolbarToggles("selection", analyzeSelectableKinds, (kind) =>
									setAnalyzeSelectableKinds((current) => ({ ...current, [kind]: !current[kind] })),
								),
								{ id: "geometry.analyze.selection.separator.clear", kind: "separator" as const, order: ANALYZE_KINDS.length + 2 },
								{
									id: "geometry.analyze.selection.clear",
									icon: <BoxSelect className="size-4" aria-hidden />,
									label: "Clear",
									onClick: () => setSelectedId(null),
									order: ANALYZE_KINDS.length + 3,
								},
							],
							filter: geometryAnalyzeToolbarToggles("filter", analyzeVisibleKinds, (kind) =>
								setAnalyzeVisibleKinds((current) => ({ ...current, [kind]: !current[kind] })),
							),
						},
					},
				],
			},
		],
		[analyzeSelectableKinds, analyzeVisibleKinds, selectableKinds, transformMode, visibleKinds],
	);

	if (loadError) throw loadError;
	if (!value) {
		return <div className={`flex h-screen items-center justify-center text-sm text-muted-foreground ${getLevelBgClass("window")}`}>Loading geometry wasm…</div>;
	}

	return (
		<GeometryPlayContext.Provider value={value}>
			<App apps={apps} defaultAppId={GEOMETRY_PLAY_APP_ID} className={getLevelBgClass(0)} />
		</GeometryPlayContext.Provider>
	);
}

function GeometryPlayApp(): ReactElement {
	return (
		<LevelProvider>
			<GeometryPlayController />
		</LevelProvider>
	);
}

const rootElement = document.getElementById("root");
if (rootElement) {
	createRoot(rootElement).render(<GeometryPlayApp />);
}
//#endregion 🔖Controller

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("geometry play fixture", () => {
		it("enables selection and visibility for every kind by default", () => {
			expect(createAllKindsEnabled(TOPOLOGIC_KINDS)).toEqual({
				topology: true,
				vertex: true,
				edge: true,
				wire: true,
				face: true,
				shell: true,
				cell: true,
				cellComplex: true,
				cluster: true,
			});
			expect(createAllKindsEnabled(ANALYZE_KINDS)).toEqual({
				"surface.external.horizontal": true,
				"surface.external.vertical": true,
				"surface.internal.horizontal": true,
				"surface.internal.vertical": true,
				"part.none": true,
				"part.difference": true,
				"part.intersection": true,
				solid: true,
			});
		});

		it("renders through wasm fixture load without changing hook order", async () => {
			const container = document.createElement("div");
			document.body.appendChild(container);
			const root = createRoot(container);
			const errors: string[] = [];
			const originalError = console.error;
			const originalActEnvironment = (globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT;
			(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
			console.error = (...args: unknown[]) => {
				errors.push(args.map((value) => String(value)).join(" "));
			};
			try {
				await act(async () => {
					root.render(<GeometryPlayController />);
					await Promise.resolve();
					await Promise.resolve();
				});
				expect(errors.some((entry) => entry.includes("change in the order of Hooks called by GeometryPlayController"))).toBe(false);
				expect(errors.some((entry) => entry.includes("Rendered more hooks than during the previous render"))).toBe(false);
			} finally {
				console.error = originalError;
				(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = originalActEnvironment;
				await act(async () => {
					root.unmount();
				});
				container.remove();
			}
		});

		it("ships at least one selectable entity for every topologic kind", async () => {
			const fixture = (await loadTopologicFixtureV1(topologyJson as unknown)) as TopologicFixtureV1;
			const session = new TopologicWasmSession(fixture);
			for (const kind of TOPOLOGIC_KINDS) {
				expect(session.listByKind(kind).length).toBeGreaterThan(0);
			}
		});

		it("registers translate, rotate, and scale as toolbar transform tools", () => {
			expect([...GEOMETRY_PLAY_TRANSFORM_MODES]).toEqual(["translate", "rotate", "scale"]);
			expect(Object.keys(GEOMETRY_PLAY_TRANSFORM_ICONS)).toEqual(["translate", "rotate", "scale"]);
		});

		it("labels enabled kind sets for the status strip", () => {
			const all = createAllKindsEnabled(TOPOLOGIC_KINDS);
			expect(formatEnabledKindsLabel(listEnabledKinds(TOPOLOGIC_KINDS, all), TOPOLOGIC_KINDS.length)).toBe("all");
			expect(formatEnabledKindsLabel(["vertex", "edge"], TOPOLOGIC_KINDS.length)).toBe("vertex,edge");
			expect(formatEnabledKindsLabel([], TOPOLOGIC_KINDS.length)).toBe("none");
		});

		it("derives analyze solids, parts, and semantic surfaces from the shipped fixture", async () => {
			const fixture = (await loadTopologicFixtureV1(topologyJson as unknown)) as TopologicFixtureV1;
			const analyzeFixture = createAnalyzeFixture(fixture);
			const analyzeSession = new TopologicWasmSession(analyzeFixture);
			const selectable = analyzeFixture.topologies.filter((entity) => entity.metadata?.analyzeSelectable === true);
			expect(selectable.filter((entity) => entity.metadata?.analyzeGroup === "solid")).toHaveLength(3);
			expect(selectable.filter((entity) => entity.metadata?.analyzeKind === "part.difference")).toHaveLength(3);
			expect(selectable.filter((entity) => entity.metadata?.analyzeKind === "part.intersection")).toHaveLength(2);
			expect(selectable.filter((entity) => String(entity.metadata?.analyzeKind).startsWith("surface."))).not.toHaveLength(0);
			expect(isAnalyzeSelectableEntity(analyzeSession, createAllKindsEnabled(ANALYZE_KINDS), "analyze.part.1")).toBe(true);
			expect(isAnalyzeSelectableEntity(analyzeSession, createAllKindsEnabled(ANALYZE_KINDS), "analyze.part.1.face.1")).toBe(false);
		});
	});
}
//#endregion 🧪Tests